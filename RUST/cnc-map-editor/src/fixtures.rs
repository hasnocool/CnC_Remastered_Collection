//! Byte-exact original-format scenario fixtures.
//!
//! Every other import test in this workspace builds synthetically. These
//! fixtures instead encode the original on-disk layout exactly as the retail
//! readers parse it, so the importers are exercised against bytes rather than
//! against a string built by the test itself:
//!
//! - RA `Get_UUBlock` (`REDALERT/INI.CPP`): numbered lines whose values are
//!   RFC-4648 base64, decoded in order and concatenated. Each decoded cell is
//!   `(u16 LE cell index, u8 id)` (`cnc_asset::ra_pack`).
//! - TD `.bin` (`TIBERIANDAWN/MAP.CPP` `Read_Binary`): one 2-byte record per
//!   cell, `TemplateType` then `TIcon`, row-major.
//!
//! The fixtures are small but structurally real: a 16x16 RA map with mixed
//! overlay cells and a full object/house/trigger set, and a 16x16 TD map
//! whose icon bytes are non-zero so a reader that drops them is caught.

use cnc_asset::ra_pack::{encode_pack, PackedCell};

/// Cells of the RA fixture map: a diagonal band of template 11, id 22 in the
/// corners, and an overlay of id 3 on cell 0.
pub fn ra_fixture_cells() -> Vec<PackedCell> {
    let mut cells = Vec::new();
    for index in 0u16..256 {
        let x = index % 16;
        let y = index / 16;
        let id = if x == y {
            11
        } else if (x == 0 || x == 15) && (y == 0 || y == 15) {
            22
        } else {
            0
        };
        cells.push(PackedCell { index, id });
    }
    cells
}

/// Overlay cells: one road tile at cell 0 and one at cell 255.
pub fn ra_fixture_overlay() -> Vec<PackedCell> {
    vec![
        PackedCell { index: 0, id: 3 },
        PackedCell { index: 255, id: 3 },
    ]
}

/// A complete original-format RA scenario.
///
/// Contains `[Basic]`, base64 `MapPack`/`OverlayPack`, object sections,
/// `[Houses]`, `[CellTriggers]` and `[Triggers]` so every RA reader path
/// sees realistic input.
pub fn ra_scenario_text() -> String {
    let mut text = String::new();
    text.push_str("[Basic]\n");
    text.push_str("Name=Fixture One\n");
    text.push_str("Player=Greece\n");
    text.push_str("Briefing=Hold the ridge.\n");
    for (section, cells) in [
        ("MapPack", ra_fixture_cells()),
        ("OverlayPack", ra_fixture_overlay()),
    ] {
        text.push_str(&format!("[{section}]\n"));
        for (key, value) in encode_pack(&cells) {
            text.push_str(&format!("{key}={value}\n"));
        }
    }
    text.push_str("[Units]\n");
    text.push_str("1=Greece,HTNK,256,17,0,Guard,CELLA\n");
    text.push_str("2=USSR,MTNK,128,240,64,Guard,\n");
    text.push_str("[Infantry]\n");
    text.push_str("1=USSR,E1,256,200,0,Guard,0,\n");
    text.push_str("[Structures]\n");
    text.push_str("1=Greece,POWR,256,16,0,,1,1\n");
    text.push_str("2=USSR,BARR,255,255,0,,\n");
    text.push_str("[Houses]\n0=Greece\n1=USSR\n");
    text.push_str("[Greece]\nCredits=40\nTechLevel=7\nMaxInfantry=60\nMaxVessel=3\n");
    text.push_str("[USSR]\nCredits=20\n");
    text.push_str("[CellTriggers]\n17=CELLA\n");
    text.push_str("[Triggers]\n1=CELLA|elapsed:999|win\n");
    text
}

/// A complete original-format TD binary map image: 16x16 cells of
/// `(TemplateType, TIcon)`. The icon byte equals the low nibble of the cell
/// index, so a reader that ignores `TIcon` cannot match these bytes.
pub fn td_bin_bytes() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(16 * 16 * 2);
    for index in 0u16..256 {
        // Template 7 everywhere except the diagonal, which is template 12.
        let template = if (index % 16) == (index / 16) { 12 } else { 7 };
        bytes.push(template);
        bytes.push((index % 16) as u8);
    }
    bytes
}

/// The TD `.bin` as it appears on disk when the payload is LCW-compressed.
///
/// `MAP.CPP` `Read_Binary` reads the file through `LCW_Uncomp`, so a real
/// `.bin` may be either raw or compressed. This fixture compresses
/// [td_bin_bytes] with the format80 encoder so the compressed decode path is
/// exercised by the same importer the raw path is.
pub fn td_bin_bytes_lcw() -> Vec<u8> {
    let raw = td_bin_bytes();
    cnc_asset::format80::compress(&raw, raw.len()).expect("format80 compress")
}
/// The matching TD `.ini`, including `[Houses]`, objects and a trigger.
pub fn td_scenario_text() -> String {
    let mut text = String::new();
    text.push_str("[Basic]\n");
    text.push_str("Name=Fixture TD\n");
    text.push_str("Player=GDI\n");
    text.push_str("Theater=Temperate\n");
    text.push_str("[Houses]\n0=GDI\n1=Nod\n");
    text.push_str("[GDI]\nCredits=50\nMaxUnit=120\nMaxBuilding=90\nEdge=West\n");
    text.push_str("[Nod]\nCredits=25\nEdge=East\n");
    text.push_str("[Wunits]\n1=GDI,E1,256,17,2,Guard,\n2=Nod,MTNK,256,240,0,Guard,\n");
    text.push_str("[Structures]\n1=GDI,NUK,256,16,0,\n");
    text.push_str("[Trigs]\n1=TRIGA\n");
    text.push_str("[TRIGA]\nName=First\nCondition1=Player Enters\nAction1=Win\n");
    text.push_str("[Waypoints]\n0=17\n1=240\n");
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ra_fixture_uses_the_numbered_base64_container() {
        let text = ra_scenario_text();
        assert!(text.contains("[MapPack]"));
        // Values must be base64, not raw binary.
        let pack = text
            .split("[MapPack]")
            .nth(1)
            .and_then(|rest| rest.split("[OverlayPack]").next())
            .expect("MapPack section");
        let first = pack
            .lines()
            .find_map(|line| line.split_once('=').map(|(_, v)| v.to_string()))
            .expect("a packed line");
        assert!(
            first
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='),
            "base64 alphabet only: {first}"
        );
    }

    #[test]
    fn ra_fixture_round_trips_through_the_pack_decoder() {
        let cells = ra_fixture_cells();
        let encoded = encode_pack(&cells);
        let decoded = cnc_asset::ra_pack::decode_pack(&encoded).expect("decode");
        assert_eq!(decoded.len(), 256);
        assert_eq!(decoded[0].id, 11, "diagonal cell is template 11");
    }

    #[test]
    fn td_bin_is_two_bytes_per_cell() {
        let bytes = td_bin_bytes();
        assert_eq!(bytes.len(), 16 * 16 * 2);
        assert_eq!(bytes[0], 12, "cell 0 is on the diagonal");
        assert_eq!(bytes[1], 0, "its icon index is 0");
    }

    #[test]
    fn td_bin_icon_bytes_are_not_all_zero() {
        // Guards the fixture itself: if every icon were 0, a reader that
        // dropped TIcon would still pass.
        let bytes = td_bin_bytes();
        let icons: Vec<u8> = bytes.chunks_exact(2).map(|r| r[1]).collect();
        assert!(
            icons.iter().any(|i| *i != 0),
            "the fixture must exercise the icon byte"
        );
    }

    #[test]
    fn td_scenario_text_carries_every_imported_section() {
        let text = td_scenario_text();
        for section in ["[Basic]", "[Houses]", "[Wunits]", "[Structures]", "[Trigs]"] {
            assert!(text.contains(section), "missing {section}");
        }
    }
}
