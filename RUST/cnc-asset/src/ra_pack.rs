//! Red Alert `MapPack` / `OverlayPack` INI blocks.
//!
//! The original engine writes bulk map data into an INI section as numbered
//! base64 lines (`INIClass::Put_UUBlock`, `REDALERT/INI.CPP`) decoded through
//! `Base64Pipe` (`REDALERT/BASE64.CPP`). Each line is a key `1`, `2`, ... whose
//! value holds at most 70 encoded characters.
//!
//! Byte layout follows `MapClass::Read_Binary`: the decoded stream is a
//! sequence of 3-byte groups (u16 little-endian cell index, then one id byte).

use anyhow::{bail, Result};

use crate::base64;

/// Bytes per packed cell entry: `u16` index plus one id byte.
pub const PACK_ENTRY_BYTES: usize = 3;

/// One decoded `MapPack` / `OverlayPack` entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedCell {
    /// Map cell index the entry applies to.
    pub index: u16,
    /// Terrain template (MapPack) or overlay id (OverlayPack).
    pub id: u8,
}

/// Gather an INI block's numbered lines into one base64 stream.
///
/// Lines are consumed in numeric key order (`1`, `2`, ...) because that is the
/// order `Put_UUBlock` wrote them.
pub fn gather_lines(entries: &[(String, String)]) -> String {
    let mut numbered: Vec<(u64, String)> = entries
        .iter()
        .filter_map(|(key, value)| key.trim().parse::<u64>().ok().map(|n| (n, value.clone())))
        .collect();
    numbered.sort_by_key(|(n, _)| *n);
    numbered.into_iter().map(|(_, value)| value).collect()
}

/// Decode a `MapPack` / `OverlayPack` block into its cell entries.
#[allow(clippy::chunks_exact_to_as_chunks)]
pub fn decode_pack(entries: &[(String, String)]) -> Result<Vec<PackedCell>> {
    let stream = gather_lines(entries);
    if stream.is_empty() {
        return Ok(Vec::new());
    }
    let bytes = base64::decode(&stream)
        .map_err(|err| anyhow::anyhow!("decoding packed map block: {err}"))?;

    // Raw cell stream: 3 bytes per cell, indices in range.
    if bytes.len() % PACK_ENTRY_BYTES == 0 {
        let n = bytes.len() / PACK_ENTRY_BYTES;
        let raw = decode_pack_bytes(&bytes);
        if let Ok(cells) = raw {
            if cells.iter().all(|cell| (cell.index as usize) < n) {
                return Ok(cells);
            }
        }
    }

    // LCW-compressed (format80) stream: try square map sizes first.
    for side in 8u32..=512 {
        let want = (side as usize) * (side as usize) * PACK_ENTRY_BYTES;
        if let Ok(inflated) = crate::format80::decompress(&bytes, want) {
            if inflated.len() == want && inflated.len() % PACK_ENTRY_BYTES == 0 {
                return Ok(inflated
                    .chunks_exact(PACK_ENTRY_BYTES)
                    .map(|entry| PackedCell {
                        index: u16::from_le_bytes([entry[0], entry[1]]),
                        id: entry[2],
                    })
                    .collect());
            }
        }
    }
    // Then small (possibly non-square) payloads such as overlays.
    for want in (PACK_ENTRY_BYTES..=4096).step_by(PACK_ENTRY_BYTES) {
        if let Ok(inflated) = crate::format80::decompress(&bytes, want) {
            if inflated.len() == want && inflated.len() % PACK_ENTRY_BYTES == 0 {
                return Ok(inflated
                    .chunks_exact(PACK_ENTRY_BYTES)
                    .map(|entry| PackedCell {
                        index: u16::from_le_bytes([entry[0], entry[1]]),
                        id: entry[2],
                    })
                    .collect());
            }
        }
    }

    // Fallback: truncate to aligned raw cells.
    decode_pack_bytes(&bytes)
}

/// Decode an already-unwrapped packed block.
#[allow(clippy::chunks_exact_to_as_chunks)]
pub fn decode_pack_bytes(bytes: &[u8]) -> Result<Vec<PackedCell>> {
    if bytes.len() % PACK_ENTRY_BYTES != 0 {
        bail!(
            "packed block length is not a multiple of {} ({} bytes)",
            PACK_ENTRY_BYTES,
            bytes.len()
        );
    }
    Ok(bytes
        .chunks_exact(PACK_ENTRY_BYTES)
        .map(|entry| PackedCell {
            index: u16::from_le_bytes([entry[0], entry[1]]),
            id: entry[2],
        })
        .collect())
}

/// Encode packed entries back into numbered base64 lines (inverse of
/// [`decode_pack`]); used by tests and round-trip tooling.
pub fn encode_pack(cells: &[PackedCell]) -> Vec<(String, String)> {
    let mut bytes = Vec::with_capacity(cells.len() * PACK_ENTRY_BYTES);
    for cell in cells {
        bytes.extend_from_slice(&cell.index.to_le_bytes());
        bytes.push(cell.id);
    }
    base64::uu_lines(&bytes)
        .into_iter()
        .enumerate()
        .map(|(index, line)| ((index + 1).to_string(), line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_numbered_lines() {
        let cells = vec![
            PackedCell { index: 0, id: 1 },
            PackedCell {
                index: 257,
                id: 200,
            },
            PackedCell {
                index: u16::MAX,
                id: 0,
            },
        ];
        let lines = encode_pack(&cells);
        assert!(!lines.is_empty());
        assert_eq!(
            lines[0].0, "1",
            "keys are 1-based, as Put_UUBlock writes them"
        );

        let decoded = decode_pack(&lines).unwrap();
        assert_eq!(decoded, cells);
    }

    #[test]
    fn line_order_is_numeric_not_lexicographic() {
        let entries = vec![
            ("10".to_string(), "Z".to_string()),
            ("2".to_string(), "A".to_string()),
        ];
        assert_eq!(gather_lines(&entries), "AZ");
    }

    #[test]
    fn decodes_little_endian_indices() {
        let bytes = [0x00u8, 0x01, 0x07];
        let cells = decode_pack_bytes(&bytes).unwrap();
        assert_eq!(cells, vec![PackedCell { index: 256, id: 7 }]);
    }

    #[test]
    fn rejects_misaligned_blocks() {
        let err = decode_pack_bytes(&[0u8; 5]).unwrap_err();
        assert!(err.to_string().contains("multiple of 3"));
    }

    #[test]
    fn empty_block_yields_no_cells() {
        assert!(decode_pack(&[]).unwrap().is_empty());
    }
}
