//! Westwood VQA movie demuxer — Rust counterpart of the original's
//! briefing/cutscene playback (`INTRO.CPP`/`ENDING.CPP` drove Windows MCI in
//! `MCIMOVIE.CPP`; the data format is the Westwood VQA 1.0 IFF container).
//!
//! **Verified scope (v1, synthetic-tested):** container walk, `VQHD` header,
//! full codebooks (`CBF0`), full frames (`VQFL`, column-major 2×2 blocks),
//! raw PCM audio (`SND`). Compressed variants (`CBP0`/`CBPZ`/`CBFZ` partial
//! or format80/LZW codebooks, `VQFR` delta frames, `SND1`/`SND2` ADPCM) are
//! detected and reported as [`VqaError::UnsupportedVariant`] — they need a
//! Westwood format80 decoder, which the asset pipeline does not ship yet
//! (it consumes OpenRA-preprocessed atlases; see `convert.rs`).
//!
//! `VQHD` layout (v1 contract, ≥ 12 bytes): `version: u16le`, `flags: u16le`,
//! `num_frames: u16le`, `frame_rate: u16le`, `width_blocks: u16le`,
//! `height_blocks: u16le`. Width/height are block counts; pixel size is
//! `2 × blocks` per axis.

use thiserror::Error;

/// Header chunk tag.
const TAG_HEADER: [u8; 4] = *b"VQHD";
/// Full codebook (raw bytes).
const TAG_CODEBOOK_FULL: [u8; 4] = *b"CBF0";
/// Full codebook, format80-compressed.
const TAG_CODEBOOK_FULL_Z: [u8; 4] = *b"CBFZ";
/// Full frame, raw vector-pointer data.
const TAG_FRAME_FULL: [u8; 4] = *b"VQFL";
/// Full frame, format80-compressed vector-pointer data.
const TAG_FRAME_FULL_Z: [u8; 4] = *b"VQFR";
/// Raw PCM audio (`SND0`).
const TAG_AUDIO_RAW: [u8; 4] = *b"SND0";

/// Minimum bytes for a valid container prefix (`FORM` + size + `WVQA`).
const FORM_PREFIX_LEN: usize = 12;
/// Bytes of `VQHD` this decoder requires.
const VQHD_LEN: usize = 12;
/// One codebook entry: a 2×2 block of 4 palette indices.
const CODEBOOK_ENTRY_LEN: usize = 4;
const CODEBOOK_ENTRIES: usize = 256;
const CODEBOOK_LEN: usize = CODEBOOK_ENTRIES * CODEBOOK_ENTRY_LEN;

/// Decoding / demuxing failures.
#[derive(Debug, Error)]
pub enum VqaError {
    #[error("buffer too short for a VQA container")]
    TooShort,
    #[error("bad FORM magic at offset {0}")]
    BadMagic(usize),
    #[error("chunk {tag:?} overruns its container")]
    ChunkOverrun { tag: [u8; 4] },
    #[error("VQHD too short ({0} bytes; need {1})")]
    BadHeader(usize, usize),
    #[error("chunk {0:?} uses a compressed variant this decoder does not support yet")]
    UnsupportedVariant(&'static str),
    #[error("frame data length {data} does not match {blocks} blocks")]
    DimMismatch { data: usize, blocks: usize },
    #[error("codebook is {len} bytes; need {expected}")]
    BadCodebook { len: usize, expected: usize },
    #[error("format80 decompression failed: {0}")]
    Format80(#[from] crate::format80::Format80Error),
    #[error("compressed frame chunk appeared before the VQHD header")]
    FrameBeforeHeader,
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
    /// `VQFL` full frame: `data` holds one codebook index per block,
    /// column-major (Westwood layout).
    FrameFull { data: Vec<u8> },
    /// `CBF0` full codebook: exactly [`CODEBOOK_LEN`] bytes.
    CodebookFull { data: Vec<u8> },
    /// `SND` raw little-endian i16 PCM samples.
    AudioPcm { samples: Vec<i16> },
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
/// Pure function of the input bytes; allocation-bounded by the input size
/// (each chunk's output is derived from its own length).
pub fn demux(bytes: &[u8]) -> Result<Vqa, VqaError> {
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
                let samples = data
                    .chunks_exact(2)
                    .map(|pair| i16::from_le_bytes([pair[0], pair[1]]))
                    .collect();
                chunks.push(VqaChunk::AudioPcm { samples });
            }
            _ => {
                // Remaining Westwood variants stay unsupported (partial
                // codebooks and Westwood ADPCM audio).
                match &tag {
                    b"CBP0" => return Err(VqaError::UnsupportedVariant("CBP0")),
                    b"CBPZ" => return Err(VqaError::UnsupportedVariant("CBPZ")),
                    b"SND1" | b"SND2" => return Err(VqaError::UnsupportedVariant("SND1/SND2")),
                    _ => chunks.push(VqaChunk::Other {
                        tag,
                        data: data.to_vec(),
                    }),
                }
            }
        }

        // IFF chunks are padded to even offsets.
        offset = data_start + size + (size & 1);
    }

    let header = header.ok_or(VqaError::BadHeader(0, VQHD_LEN))?;
    Ok(Vqa { header, chunks })
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
            let px = base_x + (step % 2) as usize;
            let py = base_y + (step / 2) as usize;
            out[py * out_width + px] = pixel;
        }
    }
    Ok(out)
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

    #[test]
    fn compressed_variants_report_the_missing_codec() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"FORM");
        bytes.extend_from_slice(&[0, 0, 0, 0]);
        bytes.extend_from_slice(b"WVQA");
        bytes.extend_from_slice(&chunk(&TAG_HEADER, &[1, 0, 0, 0, 0, 0, 2, 0, 2, 0, 3, 0]));
        bytes.extend_from_slice(&chunk(b"CBPZ", &[1, 2, 3, 4]));
        let total = (bytes.len() - 8) as u32;
        bytes[4..8].copy_from_slice(&total.to_be_bytes());
        assert!(matches!(
            demux(&bytes),
            Err(VqaError::UnsupportedVariant("CBPZ"))
        ));
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
}
