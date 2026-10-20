//! Westwood format80 (LCW) decompression — Rust port of the original
//! `LCWUNCMP.CPP` / `LCW.CPP` opcode table, verified against the C#
//! reference in `CnCTDRAMapEditor/Utility/WWCompression.cs:249-477`
//! (`LcwDecompress`).
//!
//! Opcodes (n = control byte read from the stream, operands little-endian):
//!
//! | n                     | operation                                            |
//! |-----------------------|------------------------------------------------------|
//! | `0xxxxxxx`            | short copy from dest: count = `(n >> 4) + 3`, back-offset = `next + ((n & 0x0F) << 8)` |
//! | `10000000`            | end of stream                                        |
//! | `10xxxxxx` (≠ 0x80)   | literal run: copy `n & 0x3F` bytes from the source   |
//! | `11xxxxxx` (≠ FE, FF) | copy `(n & 0x3F) + 3` bytes from dest offset (LE u16) |
//! | `11111110` (FE)       | RLE fill: `w1` (LE u16) bytes of `b1`                |
//! | `11111111` (FF)       | long copy: `w1` (LE u16) bytes from dest offset `w2` (LE u16) |
//!
//! Copies are byte-by-byte forward, so overlapping references repeat the
//! pattern (same semantics as the original pointer walk). Offsets index the
//! output buffer from its start, unless the stream opens with a single nul
//! byte (`0x00`): that prefix selects the "relative LCW" variant used by
//! hi-colour VQA32 data, in which the 0xC0..0xFD and 0xFF offsets are a
//! distance *back* from the current write position. The prefix is not
//! decodable as an opcode (a short copy at an empty output cannot resolve its
//! back-reference), so auto-detecting it cannot shadow a valid absolute
//! stream — the same rule the C# reference applies.
//!
//! Intentional hardening over both the C# reference and the original C code
//! ("does not check for corrupt source data"):
//!
//! * every read is bounds-checked and every reference is range-checked; the
//!   reference silently truncates copies at the output cap or reads zeroed
//!   buffer bytes past the written prefix;
//! * output may never exceed `expected_len`, so hostile streams cannot
//!   balloon memory;
//! * the terminating `0x80` is required and must land on exactly
//!   `expected_len` — the reference treats a full output buffer as success
//!   and merely skips a following `0x80`.
//!
//! These stricter rules only turn streams the reference would decode to a
//! *short* (or silently corrupt) buffer into errors; every well-formed
//! MapPack/VQA stream decodes identically.

use thiserror::Error;

/// Decompression failures.
#[derive(Debug, Error)]
pub enum Format80Error {
    #[error("unexpected end of input inside opcode at offset {0}")]
    UnexpectedEnd(usize),
    #[error("back-reference beyond the start of the output at offset {0}")]
    BadBackReference(usize),
    #[error("absolute reference {offset} beyond written output ({len}) at stream offset {at}")]
    BadAbsoluteReference {
        offset: usize,
        len: usize,
        at: usize,
    },
    #[error(
        "relative reference {offset} does not reach written output ({len}) at stream offset {at}"
    )]
    BadRelativeReference {
        offset: usize,
        len: usize,
        at: usize,
    },
    #[error("decompressed {actual} bytes; expected {expected}")]
    LengthMismatch { actual: usize, expected: usize },
}

/// Decompress a format80 stream into exactly `expected_len` bytes.
///
/// Pure function of `input`; allocation is bounded by `expected_len`
/// (output can never exceed it, so hostile streams cannot balloon memory).
pub fn decompress(input: &[u8], expected_len: usize) -> Result<Vec<u8>, Format80Error> {
    let mut out: Vec<u8> = Vec::with_capacity(expected_len);
    let mut i = 0usize;

    // A leading nul selects relative LCW (VQA32 hi-colour). Such a byte can
    // never be a valid short-copy opcode at an empty output, so this mirrors
    // the C# reference without shadowing an absolute stream.
    let relative = input.first() == Some(&0x00);
    if relative {
        i = 1;
    }

    // Byte-by-byte forward copy preserving overlap semantics; `from` may
    // reference bytes this very opcode is producing.
    fn copy_forward(out: &mut Vec<u8>, from: usize, count: usize) {
        for step in 0..count {
            let byte = out[from + step];
            out.push(byte);
        }
    }

    while i < input.len() {
        let op = input[i];
        i += 1;

        if op & 0x80 == 0 {
            // Short copy from dest (relative, 12-bit offset).
            let Some(&lo) = input.get(i) else {
                return Err(Format80Error::UnexpectedEnd(i));
            };
            i += 1;
            let count = (op >> 4) as usize + 3;
            let back = lo as usize + (((op & 0x0F) as usize) << 8);
            if back == 0 || back > out.len() {
                return Err(Format80Error::BadBackReference(i));
            }
            let from = out.len() - back;
            if from + count > expected_len || out.len() + count > expected_len {
                return Err(Format80Error::LengthMismatch {
                    actual: out.len() + count,
                    expected: expected_len,
                });
            }
            copy_forward(&mut out, from, count);
            continue;
        }

        if op == 0x80 {
            if out.len() != expected_len {
                return Err(Format80Error::LengthMismatch {
                    actual: out.len(),
                    expected: expected_len,
                });
            }
            return Ok(out);
        }

        if op & 0x40 == 0 {
            // Literal run from the source.
            let count = (op & 0x3F) as usize;
            if i + count > input.len() {
                return Err(Format80Error::UnexpectedEnd(i));
            }
            if out.len() + count > expected_len {
                return Err(Format80Error::LengthMismatch {
                    actual: out.len() + count,
                    expected: expected_len,
                });
            }
            out.extend_from_slice(&input[i..i + count]);
            i += count;
            continue;
        }

        if op == 0xFE {
            // Long RLE run.
            let Some(&lo) = input.get(i) else {
                return Err(Format80Error::UnexpectedEnd(i));
            };
            let Some(&hi) = input.get(i + 1) else {
                return Err(Format80Error::UnexpectedEnd(i + 1));
            };
            let Some(&fill) = input.get(i + 2) else {
                return Err(Format80Error::UnexpectedEnd(i + 2));
            };
            i += 3;
            let count = u16::from_le_bytes([lo, hi]) as usize;
            if out.len() + count > expected_len {
                return Err(Format80Error::LengthMismatch {
                    actual: out.len() + count,
                    expected: expected_len,
                });
            }
            out.extend(std::iter::repeat(fill).take(count));
            continue;
        }

        // Remaining opcodes carry a little-endian u16 immediately after the
        // control byte (two for the 0xFF long copy).
        let operands = if op == 0xFF { 4 } else { 2 };
        if i + operands > input.len() {
            return Err(Format80Error::UnexpectedEnd(i));
        }
        let count = if op == 0xFF {
            u16::from_le_bytes([input[i], input[i + 1]]) as usize
        } else {
            (op & 0x3F) as usize + 3
        };
        let offset = if op == 0xFF {
            u16::from_le_bytes([input[i + 2], input[i + 3]]) as usize
        } else {
            u16::from_le_bytes([input[i], input[i + 1]]) as usize
        };
        i += operands;

        if op == 0xFE {
            unreachable!("0xFE handled above");
        }

        // Medium/long copies use absolute offsets into the output, or a
        // distance back from the write head when the relative prefix was seen.
        let from = if relative {
            if count > 0 && (offset == 0 || offset > out.len()) {
                return Err(Format80Error::BadRelativeReference {
                    offset,
                    len: out.len(),
                    at: i,
                });
            }
            out.len().saturating_sub(offset)
        } else {
            if offset >= out.len() && count > 0 {
                return Err(Format80Error::BadAbsoluteReference {
                    offset,
                    len: out.len(),
                    at: i,
                });
            }
            offset
        };
        if out.len() + count > expected_len {
            return Err(Format80Error::LengthMismatch {
                actual: out.len() + count,
                expected: expected_len,
            });
        }
        copy_forward(&mut out, from, count);
    }

    Err(Format80Error::UnexpectedEnd(input.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literals_copy_from_the_source() {
        // 0x86 = 0x80 | 6 → six literal bytes, then the end marker.
        let out = decompress(&[0x86, b'a', b'b', b'c', b'd', b'e', b'f', 0x80], 6).expect("ok");
        assert_eq!(out, b"abcdef");
    }

    #[test]
    fn short_copy_repeats_from_a_relative_back_reference() {
        // "ab", then short copy: count = (0>>4)+3 = 3, offset = 2 →
        // forward-overlapping copy yields one extra "aba" cycle.
        let out = decompress(&[0x82, b'a', b'b', 0x00, 0x02, 0x80], 5).expect("ok");
        assert_eq!(out, b"ababa");
    }

    #[test]
    fn short_copy_without_overlap_repeats_the_tail() {
        // "abcdef", then short copy: count = 3, offset = 3 → "def".
        let out = decompress(
            &[0x86, b'a', b'b', b'c', b'd', b'e', b'f', 0x00, 0x03, 0x80],
            9,
        )
        .expect("ok");
        assert_eq!(out, b"abcdefdef");
    }

    #[test]
    fn absolute_medium_copy_can_reference_the_start() {
        // "xyzw", then medium copy from dest: count = (1 & 0x3F) + 3 = 4,
        // absolute offset 0 → append "xyzw".
        let out =
            decompress(&[0x84, b'x', b'y', b'z', b'w', 0xC1, 0x00, 0x00, 0x80], 8).expect("ok");
        assert_eq!(out, b"xyzwxyzw");
    }

    #[test]
    fn long_run_fills_one_byte() {
        let out = decompress(&[0xFE, 0x05, 0x00, 0xAA, 0x80], 5).expect("ok");
        assert_eq!(out, vec![0xAA; 5]);
    }

    #[test]
    fn long_copy_uses_a_16bit_absolute_offset() {
        // "abcdef" then long copy: count = 3, absolute offset 0 → "abc".
        let out = decompress(
            &[
                0x86, b'a', b'b', b'c', b'd', b'e', b'f', 0xFF, 0x03, 0x00, 0x00, 0x00, 0x80,
            ],
            9,
        )
        .expect("ok");
        assert_eq!(out, b"abcdefabc");
    }

    #[test]
    fn length_mismatch_is_detected_like_openra_validates() {
        // Terminates after 2 bytes but 4 were expected.
        assert!(matches!(
            decompress(&[0x82, b'a', b'b', 0x80], 4),
            Err(Format80Error::LengthMismatch {
                actual: 2,
                expected: 4
            })
        ));
        // Missing terminator.
        assert!(matches!(
            decompress(&[0x82, b'a', b'b'], 2),
            Err(Format80Error::UnexpectedEnd(3))
        ));
        // Back-reference before the start of the output.
        assert!(matches!(
            decompress(&[0x01, 0x05, 0x80], 4),
            Err(Format80Error::BadBackReference(2))
        ));
    }
}

// ---------------------------------------------------------------------------
// Compression
// ---------------------------------------------------------------------------

/// Compress `input` into a format80 (LCW) stream.
///
/// Emits only the three opcodes this module decodes, all of which the
/// original compressor (LCWCOMP.CPP) also defines: literal runs, RLE fills,
/// and absolute copies. Back-references are matched at a previous absolute
/// offset, so the loader reads them exactly as it reads retail data.
///
/// The contract is round-trip: `decompress(&compress(x, n), n) == x`.
pub fn compress(input: &[u8], expected_len: usize) -> Result<Vec<u8>, Format80Error> {
    if input.len() != expected_len {
        return Err(Format80Error::LengthMismatch {
            actual: input.len(),
            expected: expected_len,
        });
    }
    let mut out: Vec<u8> = Vec::new();
    let mut i = 0usize;

    fn run_at(input: &[u8], start: usize) -> usize {
        let mut n = 1usize;
        while start + n < input.len() && input[start + n] == input[start] {
            n += 1;
        }
        n
    }

    while i < input.len() {
        // 1. RLE: three or more identical bytes.
        let run = run_at(input, i);
        if run >= 3 {
            let count = run.min(u16::MAX as usize);
            out.push(0xFE);
            out.extend_from_slice(&(count as u16).to_le_bytes());
            out.push(input[i]);
            i += count;
            continue;
        }

        // 2. Absolute copy of the longest match starting before `i`.
        let mut best_len = 0usize;
        let mut best_off = 0usize;
        let window = i.saturating_sub(4095);
        for candidate in window..i {
            // Absolute LCW offsets are 16-bit; never emit a truncated one.
            if candidate > u16::MAX as usize {
                continue;
            }
            let mut n = 0usize;
            while i + n < input.len() && candidate + n < i && input[candidate + n] == input[i + n] {
                n += 1;
            }
            if n > best_len {
                best_len = n;
                best_off = candidate;
            }
        }

        if best_len >= 3 {
            let count = best_len.min(u16::MAX as usize);
            // The medium-copy family is 0xC0..=0xFD (count 3..=64); 0xFE and
            // 0xFF are the fill and long-copy opcodes, so a 65/66-byte match
            // must not be encoded as a medium copy.
            if count <= 64 {
                out.push(0xC0 | ((count - 3) as u8));
                out.extend_from_slice(&(best_off as u16).to_le_bytes());
            } else {
                out.push(0xFF);
                out.extend_from_slice(&(count as u16).to_le_bytes());
                out.extend_from_slice(&(best_off as u16).to_le_bytes());
            }
            i += count;
            continue;
        }

        // 3. Literal run, stopping before a compressing run.
        let mut n = 0usize;
        while n < 63 && i + n < input.len() {
            if n > 0 && run_at(input, i + n) >= 3 {
                break;
            }
            n += 1;
        }
        let n = n.max(1);
        out.push(0x80 | n as u8);
        out.extend_from_slice(&input[i..i + n]);
        i += n;
    }

    out.push(0x80);
    Ok(out)
}

#[cfg(test)]
mod compress_tests {
    use super::*;

    #[test]
    fn compress_round_trips_pseudo_random_data() {
        let mut data = Vec::new();
        let mut state: u32 = 0x1234_5678;
        for _ in 0..4096 {
            state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            data.push((state >> 16) as u8);
        }
        let packed = compress(&data, data.len()).expect("compress");
        let back = decompress(&packed, data.len()).expect("decompress");
        assert_eq!(back, data, "round trip must be lossless");
    }

    #[test]
    fn compress_round_trips_runs_and_repeats() {
        let mut data = vec![7u8; 500];
        let block = [1u8, 2, 3, 4, 5, 6, 7, 8];
        data.extend_from_slice(&block);
        for _ in 0..40 {
            data.extend_from_slice(&block);
        }
        let packed = compress(&data, data.len()).expect("compress");
        assert!(packed.len() < data.len(), "repetitive data should shrink");
        let back = decompress(&packed, data.len()).expect("decompress");
        assert_eq!(back, data);
    }

    #[test]
    fn compress_round_trips_a_td_bin_shaped_payload() {
        let mut data = Vec::new();
        for index in 0u16..256 {
            data.push(if index % 16 == index / 16 { 12 } else { 7 });
            data.push((index % 16) as u8);
        }
        let packed = compress(&data, data.len()).expect("compress");
        let back = decompress(&packed, data.len()).expect("decompress");
        assert_eq!(back, data);
    }

    #[test]
    fn compress_rejects_a_length_mismatch() {
        assert!(compress(&[1u8; 10], 20).is_err());
    }
}
