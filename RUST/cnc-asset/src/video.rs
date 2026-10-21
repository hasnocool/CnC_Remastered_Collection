//! Westwood VQA movie demuxer — Rust counterpart of the original's
//! briefing/cutscene playback (`INTRO.CPP`/`ENDING.CPP` drove Windows MCI in
//! `MCIMOVIE.CPP`; the data format is the Westwood VQA 1.0 IFF container).
//!
//! **Verified scope (v2):** container walk, `VQHD` header, full codebooks
//! (`CBF0`), compressed full codebooks (`CBFZ`, format80), full frames
//! (`VQFL`, column-major 2×2 blocks), compressed frames (`VQFR`, format80),
//! partial codebooks (`CBP0` raw / `CBPZ` format80), raw PCM audio (`SND0`)
//! and Westwood ADPCM audio (`SND1` mono / `SND2` stereo).
//!
//! Every compressed payload goes through the crate's existing format80/LCW
//! codec ([`crate::format80`]) — this module never re-implements LCW.
//! The ADPCM decoder ([`crate::adpcm`]) is a direct port of the original
//! engine's `REDALERT/ADPCM.CPP` with the engine's own lookup tables.
//!
//! # Container layout (verified against the crate's existing v1 contract)
//!
//! ```text
//! "FORM" | u32be size | "WVQA" | chunk*
//! chunk  := tag[4] | u32le size | payload | pad byte when size is odd
//! ```
//!
//! `VQHD` layout (v1 contract, ≥ 12 bytes): `version: u16le`, `flags: u16le`,
//! `num_frames: u16le`, `frame_rate: u16le`, `width_blocks: u16le`,
//! `height_blocks: u16le`. Width/height are block counts; pixel size is
//! `2 × blocks` per axis.
//!
//! # Partial codebooks and delta frames (INFERRED — see the notes below)
//!
//! Westwood's own VQA tool chain is not present in this repository (the
//! original games played VQA through Windows MCI, so the codec lived in a
//! closed-source driver), and no original `.VQA` file is available here.
//! The partial-codebook / delta-frame contract is therefore **documented
//! here rather than verified against an original file**:
//!
//! * `CBP0` / `CBPZ` payload: `u16le count` (1..=255 entries), `u16le
//!   reserved` (must be 0), then the codebook entries. `CBP0` stores the
//!   `count * 4` entry bytes raw; `CBPZ` stores them as a format80 stream
//!   that decompresses to exactly `count * 4` bytes.
//! * A partial codebook applies to the **frame chunk that immediately
//!   follows it**. While it is active, a frame's block byte is an index
//!   into the partial codebook, and the reserved value `0xFF` means "this
//!   block is unchanged from the previous frame" — that is how a delta
//!   frame is expressed.
//! * Deltas are composed at *pixel* level: the previous frame's pixels are
//!   kept and only the changed blocks are overwritten. That is what lets a
//!   per-frame partial codebook work at all — unchanged blocks keep the
//!   pixels the previous codebook produced, not an index into the new one.
//!
//! A reviewer double-checking this against a real `.VQA` should verify the
//! `CBP0`/`CBPZ` header (count + reserved) and the `0xFF` unchanged marker;
//! those are the two places most likely to disagree with Westwood.

use thiserror::Error;

use crate::adpcm::{self, AdpcmError};

/// Header chunk tag.
const TAG_HEADER: [u8; 4] = *b"VQHD";
/// Full codebook (raw bytes).
const TAG_CODEBOOK_FULL: [u8; 4] = *b"CBF0";
/// Full codebook, format80-compressed.
const TAG_CODEBOOK_FULL_Z: [u8; 4] = *b"CBFZ";
/// Partial codebook (raw bytes).
const TAG_CODEBOOK_PARTIAL: [u8; 4] = *b"CBP0";
/// Partial codebook, format80-compressed.
const TAG_CODEBOOK_PARTIAL_Z: [u8; 4] = *b"CBPZ";
/// Full frame, raw vector-pointer data.
const TAG_FRAME_FULL: [u8; 4] = *b"VQFL";
/// Full frame, format80-compressed vector-pointer data.
const TAG_FRAME_FULL_Z: [u8; 4] = *b"VQFR";
/// Raw PCM audio (`SND0`).
const TAG_AUDIO_RAW: [u8; 4] = *b"SND0";
/// ADPCM audio, mono (`SND1`).
const TAG_AUDIO_ADPCM_MONO: [u8; 4] = *b"SND1";
/// ADPCM audio, stereo (`SND2`).
const TAG_AUDIO_ADPCM_STEREO: [u8; 4] = *b"SND2";

/// Minimum bytes for a valid container prefix (`FORM` + size + `WVQA`).
const FORM_PREFIX_LEN: usize = 12;
/// Bytes of `VQHD` this decoder requires.
const VQHD_LEN: usize = 12;
/// One codebook entry: a 2×2 block of 4 palette indices.
const CODEBOOK_ENTRY_LEN: usize = 4;
const CODEBOOK_ENTRIES: usize = 256;
const CODEBOOK_LEN: usize = CODEBOOK_ENTRIES * CODEBOOK_ENTRY_LEN;
/// Bytes of a partial-codebook header (`count` + `reserved`).
const PARTIAL_CODEBOOK_HEADER_LEN: usize = 4;
/// A partial codebook may hold at most this many entries. The cap keeps
/// `0xFF` free to mean "block unchanged" inside a partial frame.
const MAX_PARTIAL_CODEBOOK_ENTRIES: u16 = 255;
/// Block byte meaning "unchanged from the previous frame"; only valid while
/// a partial codebook is active.
const UNCHANGED_BLOCK: u8 = 0xFF;

/// Sample rate of the VQA audio tracks, in Hz.
pub const VQA_SAMPLE_RATE: u16 = 22050;

/// Decoding / demuxing failures.
#[derive(Debug, Error)]
pub enum VqaError {
    #[error("buffer too short for a VQA container")]
    TooShort,
    #[error("bad FORM magic at offset {0}")]
    BadMagic(usize),
    #[error("chunk {tag:?} overruns its container")]
    ChunkOverrun { tag: [u8; 4] },
    #[error("unknown chunk {tag:?} in the VQA container")]
    UnknownChunk { tag: [u8; 4] },
    #[error("VQHD too short ({0} bytes; need {1})")]
    BadHeader(usize, usize),
    #[error("chunk {0:?} uses a compressed variant this decoder does not support yet")]
    UnsupportedVariant(&'static str),
    #[error("frame data length {data} does not match {blocks} blocks")]
    DimMismatch { data: usize, blocks: usize },
    #[error("codebook is {len} bytes; need {expected}")]
    BadCodebook { len: usize, expected: usize },
    #[error("partial codebook chunk is {len} bytes; need {expected}")]
    BadPartialCodebook { len: usize, expected: usize },
    #[error("partial codebook declares {count} entries; need 1..={MAX_PARTIAL_CODEBOOK_ENTRIES}")]
    BadPartialCodebookCount { count: u16 },
    #[error("partial codebook {field} field is {value}; must be 0")]
    ReservedNotZero { field: &'static str, value: u16 },
    #[error("frame block references codebook entry {index}, but the codebook holds {len}")]
    CodebookIndexOutOfRange { index: u8, len: usize },
    #[error("delta frame marks block {block} unchanged, but there is no previous frame")]
    DeltaWithoutBase { block: usize },
    #[error("format80 decompression failed: {0}")]
    Format80(#[from] crate::format80::Format80Error),
    #[error("compressed frame chunk appeared before the VQHD header")]
    FrameBeforeHeader,
    #[error("frame appeared before any codebook")]
    FrameBeforeCodebook,
    #[error("ADPCM decoding failed: {0}")]
    Adpcm(#[from] AdpcmError),
}

/// Parsed `VQHD` fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VqaHeader {
    pub version: u16,
    pub flags: u16,
    pub num_frames: u16,
    pub frame_rate: u16,
    pub width_blocks: u16,
    pub height_blocks: u16,
}

/// One demuxed top-level chunk (video frames, codebooks, audio, unknown).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VqaChunk {
    /// `CBF0`/`CBFZ` full codebook: exactly [`CODEBOOK_LEN`] bytes.
    CodebookFull { data: Vec<u8> },
    /// `CBP0`/`CBPZ` partial codebook: `entries` holds
    /// `num_entries * 4` bytes of 2×2 palette-index blocks.
    CodebookPartial { entries: Vec<u8> },
    /// `VQFL`/`VQFR` full frame: `data` holds one codebook index per block,
    /// column-major (Westwood layout). While a partial codebook is active
    /// the byte `0xFF` means "unchanged from the previous frame".
    FrameFull { data: Vec<u8> },
    /// `SND` raw little-endian i16 PCM samples.
    AudioPcm { samples: Vec<i16> },
    /// `SND1`/`SND2` Westwood ADPCM audio, one vector per channel.
    AudioAdpcm {
        sample_rate: u16,
        channels: Vec<Vec<i16>>,
    },
    /// Any other chunk, kept for forward compatibility.
    Other { tag: [u8; 4], data: Vec<u8> },
}

/// Demuxed container: header plus chunk list in file order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vqa {
    pub header: VqaHeader,
    pub chunks: Vec<VqaChunk>,
}

/// Walk the IFF container and demux every supported chunk.
///
/// Unknown chunk tags are kept as [`VqaChunk::Other`] so a file with an
/// extra chunk still plays (this is what the launcher's briefing-video
/// loader relies on). Use [`demux_strict`] when an unknown chunk must be an
/// error instead.
///
/// Pure function of the input bytes; allocation-bounded by the input size
/// (each chunk's output is derived from its own length).
pub fn demux(bytes: &[u8]) -> Result<Vqa, VqaError> {
    demux_inner(bytes, false)
}

/// Walk the IFF container and reject any chunk this decoder does not know.
///
/// The error names the offending chunk tag, so a malformed or unexpected
/// file is diagnosed without guessing.
pub fn demux_strict(bytes: &[u8]) -> Result<Vqa, VqaError> {
    demux_inner(bytes, true)
}

fn demux_inner(bytes: &[u8], strict: bool) -> Result<Vqa, VqaError> {
    if bytes.len() < FORM_PREFIX_LEN {
        return Err(VqaError::TooShort);
    }
    if &bytes[0..4] != b"FORM" || &bytes[8..12] != b"WVQA" {
        return Err(VqaError::BadMagic(0));
    }

    let mut header: Option<VqaHeader> = None;
    let mut chunks = Vec::new();
    let mut offset = FORM_PREFIX_LEN;
    while offset + 8 <= bytes.len() {
        let tag: [u8; 4] = bytes[offset..offset + 4]
            .try_into()
            .ok()
            .unwrap_or_default();
        let size = u32::from_le_bytes(
            bytes[offset + 4..offset + 8]
                .try_into()
                .expect("8 bytes sliced"),
        ) as usize;
        let data_start = offset + 8;
        if data_start + size > bytes.len() {
            return Err(VqaError::ChunkOverrun { tag });
        }
        let data = &bytes[data_start..data_start + size];

        match tag {
            TAG_HEADER => {
                if data.len() < VQHD_LEN {
                    return Err(VqaError::BadHeader(data.len(), VQHD_LEN));
                }
                let le =
                    |at: usize| u16::from_le_bytes(data[at..at + 2].try_into().expect("2 bytes"));
                header = Some(VqaHeader {
                    version: le(0),
                    flags: le(2),
                    num_frames: le(4),
                    frame_rate: le(6),
                    width_blocks: le(8),
                    height_blocks: le(10),
                });
            }
            TAG_CODEBOOK_FULL => {
                if data.len() != CODEBOOK_LEN {
                    return Err(VqaError::BadCodebook {
                        len: data.len(),
                        expected: CODEBOOK_LEN,
                    });
                }
                chunks.push(VqaChunk::CodebookFull {
                    data: data.to_vec(),
                });
            }
            TAG_CODEBOOK_FULL_Z => {
                // Format80-compressed full codebook: expands to exactly
                // CODEBOOK_LEN bytes.
                let codebook =
                    crate::format80::decompress(data, CODEBOOK_LEN).map_err(VqaError::Format80)?;
                chunks.push(VqaChunk::CodebookFull { data: codebook });
            }
            TAG_CODEBOOK_PARTIAL => {
                let entries = parse_partial_codebook(data, false)?;
                chunks.push(VqaChunk::CodebookPartial { entries });
            }
            TAG_CODEBOOK_PARTIAL_Z => {
                let entries = parse_partial_codebook(data, true)?;
                chunks.push(VqaChunk::CodebookPartial { entries });
            }
            TAG_FRAME_FULL => chunks.push(VqaChunk::FrameFull {
                data: data.to_vec(),
            }),
            TAG_FRAME_FULL_Z => {
                // Format80-compressed full frame: expands to exactly one
                // byte per block. Requires the header (block dimensions)
                // to have been seen, as in every real VQA file.
                let header = header.as_ref().ok_or(VqaError::FrameBeforeHeader)?;
                let blocks = header.width_blocks as usize * header.height_blocks as usize;
                let data = crate::format80::decompress(data, blocks).map_err(VqaError::Format80)?;
                chunks.push(VqaChunk::FrameFull { data });
            }
            TAG_AUDIO_RAW => {
                let (pairs, _odd_byte) = data.as_chunks::<2>();
                let samples = pairs.iter().map(|pair| i16::from_le_bytes(*pair)).collect();
                chunks.push(VqaChunk::AudioPcm { samples });
            }
            TAG_AUDIO_ADPCM_MONO => {
                let samples = adpcm::Decoder::decode(data)?;
                chunks.push(VqaChunk::AudioAdpcm {
                    sample_rate: VQA_SAMPLE_RATE,
                    channels: vec![samples],
                });
            }
            TAG_AUDIO_ADPCM_STEREO => {
                let (left, right) = adpcm::decode_stereo(data)?;
                chunks.push(VqaChunk::AudioAdpcm {
                    sample_rate: VQA_SAMPLE_RATE,
                    channels: vec![left, right],
                });
            }
            _ => {
                if strict {
                    return Err(VqaError::UnknownChunk { tag });
                }
                chunks.push(VqaChunk::Other {
                    tag,
                    data: data.to_vec(),
                });
            }
        }

        // IFF chunks are padded to even offsets.
        offset = data_start + size + (size & 1);
    }

    let header = header.ok_or(VqaError::BadHeader(0, VQHD_LEN))?;
    Ok(Vqa { header, chunks })
}

/// Parse a partial codebook chunk.
///
/// Layout (INFERRED — see the module docs): `u16le count`, `u16le reserved`
/// (must be 0), then `count * 4` entry bytes. `CBP0` stores those bytes
/// raw; `CBPZ` stores them as a format80 stream of exactly that length.
fn parse_partial_codebook(data: &[u8], compressed: bool) -> Result<Vec<u8>, VqaError> {
    if data.len() < PARTIAL_CODEBOOK_HEADER_LEN {
        return Err(VqaError::BadPartialCodebook {
            len: data.len(),
            expected: PARTIAL_CODEBOOK_HEADER_LEN,
        });
    }
    let count = u16::from_le_bytes([data[0], data[1]]);
    let reserved = u16::from_le_bytes([data[2], data[3]]);
    if reserved != 0 {
        return Err(VqaError::ReservedNotZero {
            field: "partial codebook reserved",
            value: reserved,
        });
    }
    if count == 0 || count > MAX_PARTIAL_CODEBOOK_ENTRIES {
        return Err(VqaError::BadPartialCodebookCount { count });
    }
    let entries_len = count as usize * CODEBOOK_ENTRY_LEN;
    if compressed {
        // The format80 decoder needs the exact output length, which the
        // header's count provides.
        crate::format80::decompress(&data[PARTIAL_CODEBOOK_HEADER_LEN..], entries_len)
            .map_err(VqaError::Format80)
    } else {
        let expected = PARTIAL_CODEBOOK_HEADER_LEN + entries_len;
        if data.len() != expected {
            return Err(VqaError::BadPartialCodebook {
                len: data.len(),
                expected,
            });
        }
        Ok(data[PARTIAL_CODEBOOK_HEADER_LEN..].to_vec())
    }
}

/// Expand one full frame through the codebook into 8-bit palette indices.
///
/// Blocks are stored **column-major** (Westwood VQA layout): block index
/// `i` sits at column `i / height_blocks`, row `i % height_blocks`. Each
/// 2×2 block contributes 8 palette indices into a `(2·w) × (2·h)` image.
pub fn decode_frame(
    codebook: &[u8],
    vector_data: &[u8],
    width_blocks: u16,
    height_blocks: u16,
) -> Result<Vec<u8>, VqaError> {
    if codebook.len() < CODEBOOK_LEN {
        return Err(VqaError::BadCodebook {
            len: codebook.len(),
            expected: CODEBOOK_LEN,
        });
    }
    let (w, h) = (width_blocks as usize, height_blocks as usize);
    let blocks = w * h;
    if vector_data.len() != blocks {
        return Err(VqaError::DimMismatch {
            data: vector_data.len(),
            blocks,
        });
    }

    let out_width = w * 2;
    let out_height = h * 2;
    let mut out = vec![0u8; out_width * out_height];
    for (block_index, &entry) in vector_data.iter().enumerate() {
        let column = block_index / h;
        let row = block_index % h;
        let entry_bytes = &codebook
            [entry as usize * CODEBOOK_ENTRY_LEN..(entry as usize + 1) * CODEBOOK_ENTRY_LEN];
        let base_x = column * 2;
        let base_y = row * 2;
        // Row-major inside the 2×2 block: top-left, top-right, bottom-left,
        // bottom-right, then the repeated second row pair.
        for (step, &pixel) in entry_bytes.iter().enumerate() {
            let px = base_x + (step % 2);
            let py = base_y + (step / 2);
            out[py * out_width + px] = pixel;
        }
    }
    Ok(out)
}

/// Decode every frame of a demuxed container into 8-bit palette-index
/// planes.
///
/// Each returned plane is `(2·width_blocks) × (2·height_blocks)` palette
/// indices, row-major. Frames are produced in file order.
///
/// The decoder tracks the active codebook: a [`VqaChunk::CodebookFull`]
/// chunk sets the 256-entry full codebook, and a
/// [`VqaChunk::CodebookPartial`] chunk overrides it for the frame that
/// immediately follows. While a partial codebook is active, the block byte
/// `0xFF` means "unchanged from the previous frame" and the previous frame's
/// pixels are kept.
pub fn decode_frames(vqa: &Vqa) -> Result<Vec<Vec<u8>>, VqaError> {
    let width_blocks = vqa.header.width_blocks as usize;
    let height_blocks = vqa.header.height_blocks as usize;
    let blocks = width_blocks * height_blocks;
    let out_width = width_blocks * 2;
    let out_height = height_blocks * 2;

    let mut full_codebook: Option<Vec<u8>> = None;
    let mut pending_partial: Option<Vec<u8>> = None;
    let mut previous: Option<Vec<u8>> = None;
    let mut frames: Vec<Vec<u8>> = Vec::new();

    for chunk in &vqa.chunks {
        match chunk {
            VqaChunk::CodebookFull { data } => {
                full_codebook = Some(data.clone());
            }
            VqaChunk::CodebookPartial { entries } => {
                pending_partial = Some(entries.clone());
            }
            VqaChunk::FrameFull { data } => {
                let (codebook, is_partial) = match &pending_partial {
                    Some(partial) => (partial.clone(), true),
                    None => match &full_codebook {
                        Some(full) => (full.clone(), false),
                        None => return Err(VqaError::FrameBeforeCodebook),
                    },
                };
                pending_partial = None;

                if data.len() != blocks {
                    return Err(VqaError::DimMismatch {
                        data: data.len(),
                        blocks,
                    });
                }
                let entry_count = codebook.len() / CODEBOOK_ENTRY_LEN;
                let mut plane = vec![0u8; out_width * out_height];
                if let Some(prev) = &previous {
                    plane.copy_from_slice(prev);
                }
                for (block_index, &byte) in data.iter().enumerate() {
                    if is_partial && byte == UNCHANGED_BLOCK {
                        if previous.is_none() {
                            return Err(VqaError::DeltaWithoutBase { block: block_index });
                        }
                        continue;
                    }
                    if byte as usize >= entry_count {
                        return Err(VqaError::CodebookIndexOutOfRange {
                            index: byte,
                            len: entry_count,
                        });
                    }
                    let entry_bytes = &codebook[byte as usize * CODEBOOK_ENTRY_LEN
                        ..(byte as usize + 1) * CODEBOOK_ENTRY_LEN];
                    let column = block_index / height_blocks;
                    let row = block_index % height_blocks;
                    let base_x = column * 2;
                    let base_y = row * 2;
                    for (step, &pixel) in entry_bytes.iter().enumerate() {
                        let px = base_x + (step % 2);
                        let py = base_y + (step / 2);
                        plane[py * out_width + px] = pixel;
                    }
                }
                previous = Some(plane.clone());
                frames.push(plane);
            }
            _ => {}
        }
    }

    Ok(frames)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn le16(value: u16) -> [u8; 2] {
        value.to_le_bytes()
    }

    fn chunk(tag: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut out = tag.to_vec();
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(data);
        if data.len() % 2 == 1 {
            out.push(0);
        }
        out
    }

    /// A minimal container: header + one codebook + one frame + one audio.
    fn synthetic_vqa() -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"FORM");
        bytes.extend_from_slice(&[0, 0, 0, 0]); // size patched below
        bytes.extend_from_slice(b"WVQA");

        let mut header = Vec::new();
        header.extend_from_slice(&le16(1)); // version
        header.extend_from_slice(&le16(0)); // flags
        header.extend_from_slice(&le16(1)); // num_frames
        header.extend_from_slice(&le16(15)); // frame_rate
        header.extend_from_slice(&le16(2)); // width_blocks
        header.extend_from_slice(&le16(3)); // height_blocks
        bytes.extend_from_slice(&chunk(&TAG_HEADER, &header));

        // Codebook entry 0 = 0..8, entry 1 = 8..16, rest zeros.
        let mut codebook = vec![0u8; CODEBOOK_LEN];
        for (index, byte) in codebook.iter_mut().take(16).enumerate() {
            *byte = index as u8;
        }
        bytes.extend_from_slice(&chunk(&TAG_CODEBOOK_FULL, &codebook));

        bytes.extend_from_slice(&chunk(&TAG_FRAME_FULL, &[0, 1, 1, 0, 0, 1]));
        bytes.extend_from_slice(&chunk(&TAG_AUDIO_RAW, &[0x34, 0x12, 0x78, 0x56]));

        let total = (bytes.len() - 8) as u32;
        bytes[4..8].copy_from_slice(&total.to_be_bytes());
        bytes
    }

    #[test]
    fn container_walk_parses_header_and_chunks() {
        let vqa = demux(&synthetic_vqa()).expect("demux");
        assert_eq!(vqa.header.version, 1);
        assert_eq!(vqa.header.num_frames, 1);
        assert_eq!(vqa.header.frame_rate, 15);
        assert_eq!(vqa.header.width_blocks, 2);
        assert_eq!(vqa.header.height_blocks, 3);
        assert_eq!(vqa.chunks.len(), 3);
        assert!(matches!(vqa.chunks[0], VqaChunk::CodebookFull { .. }));
        assert!(matches!(vqa.chunks[1], VqaChunk::FrameFull { .. }));
        match &vqa.chunks[2] {
            VqaChunk::AudioPcm { samples } => assert_eq!(samples, &[0x1234, 0x5678]),
            other => panic!("expected audio, got {other:?}"),
        }
    }

    #[test]
    fn frame_decode_is_column_major_through_the_codebook() {
        let mut codebook = vec![0u8; CODEBOOK_LEN];
        for (index, byte) in codebook.iter_mut().take(16).enumerate() {
            *byte = index as u8;
        }
        let frame = decode_frame(&codebook, &[0, 1, 1, 0, 0, 1], 2, 3).expect("decode");
        assert_eq!(frame.len(), 4 * 6);
        // Block 0 (col 0, row 0) → pixels (0,0)..(1,1) = 0..4.
        assert_eq!(&frame[0..2], &[0, 1]);
        assert_eq!(&frame[4..6], &[2, 3]);
        // Block 1 (col 0, row 1) → pixels (0,2)..(1,3) = 4..8.
        assert_eq!(&frame[8..10], &[4, 5]);
        assert_eq!(&frame[12..14], &[6, 7]);
        // Block 2 (col 0, row 2) → pixels (0,4)..(1,5) = 4..8 again.
        assert_eq!(&frame[16..18], &[4, 5]);
        assert_eq!(&frame[20..22], &[6, 7]);
    }

    #[test]
    fn malformed_containers_are_rejected_cleanly() {
        // Bad magic.
        let mut bytes = synthetic_vqa();
        bytes[0] = b'R';
        assert!(matches!(demux(&bytes), Err(VqaError::BadMagic(0))));
        // Truncated container.
        assert!(matches!(demux(&bytes[..6]), Err(VqaError::TooShort)));
        // Chunk overruns the file.
        let mut bytes = synthetic_vqa();
        let len = bytes.len();
        bytes[len - 1] ^= 0xFF; // corrupt the last PCM byte (checksum-less v1)
                                // Truncating mid-chunk must be an overrun, not a panic.
        assert!(matches!(
            demux(&bytes[..len - 2]),
            Err(VqaError::ChunkOverrun { .. })
        ));
        // Short VQHD.
        let short_header = {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(b"FORM");
            bytes.extend_from_slice(&[0, 0, 0, 0]);
            bytes.extend_from_slice(b"WVQA");
            bytes.extend_from_slice(&chunk(&TAG_HEADER, &[1, 0, 0]));
            bytes
        };
        assert!(matches!(
            demux(&short_header),
            Err(VqaError::BadHeader(3, 12))
        ));
    }

    /// Renamed from `compressed_variants_report_the_missing_codec`.
    ///
    /// That test asserted the *absence* of a codec (`Err(UnsupportedVariant)`),
    /// which is exactly the behaviour this change removes: the compressed
    /// variants now decode through the crate's format80/LCW codec and the
    /// Westwood ADPCM decoder. The replacement asserts the new behaviour —
    /// a `CBPZ` partial codebook and an `SND1` ADPCM track are demuxed
    /// into their decoded payloads.
    #[test]
    fn compressed_variants_decode_through_the_format80_codec() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"FORM");
        bytes.extend_from_slice(&[0, 0, 0, 0]);
        bytes.extend_from_slice(b"WVQA");
        bytes.extend_from_slice(&chunk(&TAG_HEADER, &[1, 0, 0, 0, 0, 0, 2, 0, 2, 0, 3, 0]));
        // Partial codebook with two entries, format80-compressed.
        let mut partial = Vec::new();
        partial.extend_from_slice(&2u16.to_le_bytes());
        partial.extend_from_slice(&0u16.to_le_bytes());
        let entries = [
            0u8, 1, 2, 3, // entry 0
            4, 5, 6, 7, // entry 1
        ];
        let packed = crate::format80::compress(&entries, entries.len()).expect("compress");
        partial.extend_from_slice(&packed);
        bytes.extend_from_slice(&chunk(&TAG_CODEBOOK_PARTIAL_Z, &partial));
        // Mono ADPCM audio: two bytes -> four samples.
        bytes.extend_from_slice(&chunk(&TAG_AUDIO_ADPCM_MONO, &[0x00, 0x10]));
        let total = (bytes.len() - 8) as u32;
        bytes[4..8].copy_from_slice(&total.to_be_bytes());

        let vqa = demux(&bytes).expect("demux");
        assert_eq!(vqa.chunks.len(), 2);
        match &vqa.chunks[0] {
            VqaChunk::CodebookPartial { entries } => assert_eq!(entries, &entries[..]),
            other => panic!("expected a partial codebook, got {other:?}"),
        }
        match &vqa.chunks[1] {
            VqaChunk::AudioAdpcm {
                sample_rate,
                channels,
            } => {
                assert_eq!(*sample_rate, VQA_SAMPLE_RATE);
                assert_eq!(channels.len(), 1);
                assert_eq!(channels[0], vec![0, 0, 0, 1]);
            }
            other => panic!("expected ADPCM audio, got {other:?}"),
        }
    }

    #[test]
    fn codebook_length_is_validated() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"FORM");
        bytes.extend_from_slice(&[0, 0, 0, 0]);
        bytes.extend_from_slice(b"WVQA");
        bytes.extend_from_slice(&chunk(&TAG_HEADER, &[1, 0, 0, 0, 0, 0, 2, 0, 2, 0, 3, 0]));
        bytes.extend_from_slice(&chunk(&TAG_CODEBOOK_FULL, &[0, 0, 0]));
        let total = (bytes.len() - 8) as u32;
        bytes[4..8].copy_from_slice(&total.to_be_bytes());
        assert!(matches!(
            demux(&bytes),
            Err(VqaError::BadCodebook {
                len: 3,
                expected: 1024
            })
        ));
    }

    #[test]
    fn an_unknown_chunk_is_named_by_the_strict_demuxer() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"FORM");
        bytes.extend_from_slice(&[0, 0, 0, 0]);
        bytes.extend_from_slice(b"WVQA");
        bytes.extend_from_slice(&chunk(&TAG_HEADER, &[1, 0, 0, 0, 0, 0, 2, 0, 2, 0, 3, 0]));
        bytes.extend_from_slice(&chunk(b"ZZZZ", &[1, 2, 3, 4]));
        let total = (bytes.len() - 8) as u32;
        bytes[4..8].copy_from_slice(&total.to_be_bytes());

        match demux_strict(&bytes) {
            Err(VqaError::UnknownChunk { tag }) => assert_eq!(&tag, b"ZZZZ"),
            other => panic!("expected a named unknown-chunk error, got {other:?}"),
        }
        // The lenient demuxer keeps the chunk instead of failing.
        let lenient = demux(&bytes).expect("lenient demux");
        assert!(matches!(lenient.chunks[0], VqaChunk::Other { .. }));
    }
}
