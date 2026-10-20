//! Golden original-format fixtures (synthetic, license-clean).
//!
//! These tests pin the checked-in bytes under `RUST/fixtures/` and decode them
//! with the production codecs (INI parser, format80/LCW, `ra_pack`). They do
//! not depend on retail game data; the optional real-data path is gated behind
//! `CNC_ORIGINAL_DATA_DIR` and skips with a warning when it is unset or
//! unusable.

use std::path::{Path, PathBuf};

use cnc_asset::ra_pack::{self, PackedCell};
use cnc_asset::{base64, format80};
use cnc_core::{IniFile, IniSection, IniValue};
use cnc_tiberian_dawn::scenario_loader::{
    bin_path_for, load_bin, load_scenario, looks_like_original_scenario, parse_scenario, BinCell,
    BIN_CELL_BYTES,
};

const SIDE: usize = 16;
const CELLS: usize = SIDE * SIDE;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures")
}

/// Render an INI value the way the loaders do (arrays re-join with commas).
fn ini_text(value: &IniValue) -> String {
    match value {
        IniValue::String(text) => text.clone(),
        IniValue::Integer(number) => number.to_string(),
        IniValue::Float(number) => number.to_string(),
        IniValue::Boolean(flag) => u8::from(*flag).to_string(),
        IniValue::Array(items) => items.iter().map(ini_text).collect::<Vec<_>>().join(","),
    }
}

/// Decode a numbered-line base64 INI block to its raw (still LCW) bytes.
fn base64_block(section: &IniSection) -> Vec<u8> {
    let entries: Vec<(String, String)> = section
        .values
        .iter()
        .map(|(key, value)| (key.clone(), ini_text(value)))
        .collect();
    let stream = ra_pack::gather_lines(&entries);
    base64::decode(&stream).expect("numbered lines are valid base64")
}

// ---------------------------------------------------------------------------
// Tiberian Dawn fixture
// ---------------------------------------------------------------------------

#[test]
fn fixture_td_ini_parses_every_original_section() {
    let path = fixtures_dir().join("td/FIXTURE.INI");
    let text = std::fs::read_to_string(&path).expect("TD fixture INI");
    assert!(
        looks_like_original_scenario(&text),
        "[Basic] + [Wunits] must look like an original scenario"
    );

    let ini = IniFile::parse(&text).expect("fixture INI parses");
    let scenario = parse_scenario(&ini).expect("fixture scenario parses");

    assert_eq!(scenario.name, "Synthetic TD Golden Fixture");
    assert_eq!(scenario.player, 0);
    assert_eq!(scenario.credits, 5000);
    assert_eq!(scenario.tech_level, 7);
    assert_eq!(scenario.theater, "Temperate");
    assert_eq!(scenario.briefing.len(), 2, "Briefing + Briefing1");
    assert_eq!(scenario.objectives.len(), 2);

    assert_eq!(scenario.units.len(), 3);
    let e1 = scenario
        .units
        .iter()
        .find(|unit| unit.unit_type == "E1")
        .expect("GDI E1");
    assert_eq!(
        (e1.house, e1.x, e1.y, e1.facing, e1.health),
        (0, 5, 6, 32, 255)
    );
    let mtnk = scenario
        .units
        .iter()
        .find(|unit| unit.unit_type == "MTNK")
        .expect("GDI MTNK");
    assert_eq!((mtnk.house, mtnk.health), (0, 200));
    let buggy = scenario
        .units
        .iter()
        .find(|unit| unit.unit_type == "BGGY")
        .expect("Nod BGGY");
    assert_eq!(buggy.house, 1);

    assert_eq!(scenario.structures.len(), 2);
    let yard = scenario
        .structures
        .iter()
        .find(|structure| structure.structure_type == "FACT")
        .expect("GDI FACT");
    assert_eq!((yard.house, yard.x, yard.y), (0, 4, 4));
    assert!(scenario
        .structures
        .iter()
        .any(|structure| structure.structure_type == "OBLI" && structure.house == 1));

    assert_eq!(scenario.triggers.len(), 2, "[Trigs] sections");
    assert_eq!(scenario.waypoint_ids, vec![0, 1, 2]);
    assert_eq!(scenario.houses.len(), 2);
    assert_eq!(scenario.houses[0].name, "GDI");
    assert_eq!(scenario.houses[0].credits, 500_000, "Credits are x100");
    assert_eq!(scenario.houses[1].name, "Nod");
    assert_eq!(scenario.houses[1].credits, 250_000);
    assert_eq!(scenario.houses[0].edge, "West");
    assert_eq!(scenario.houses[1].edge, "East");

    // [TRIGGER] carries the original comma-record table.
    assert_eq!(scenario.trigger_records.len(), 2);
    let entry = scenario
        .trigger_records
        .iter()
        .find(|record| record.name == "T_ENTRY")
        .expect("T_ENTRY record");
    assert_eq!(entry.event, Some(1), "Player Enters");
    assert_eq!(entry.action, Some(1), "Win");
    assert_eq!(entry.data, 17);
    assert_eq!(entry.house, "GDI");
    assert_eq!(entry.width, 16, "[Map] Width");
    let clock = scenario
        .trigger_records
        .iter()
        .find(|record| record.name == "T_CLOCK")
        .expect("T_CLOCK record");
    assert_eq!(clock.event, Some(11), "Time");
    assert_eq!(clock.persistent, 1);
}

#[test]
fn fixture_td_bin_lcw_matches_the_raw_image() {
    let raw = std::fs::read(fixtures_dir().join("td/FIXTURE_RAW.bin")).expect("raw TD image");
    assert_eq!(raw.len(), CELLS * BIN_CELL_BYTES, "16x16x2 bytes");

    let lcw = std::fs::read(fixtures_dir().join("td/FIXTURE.bin")).expect("LCW TD image");
    assert_ne!(lcw, raw, "the pair stores a compressed image");
    let inflated = format80::decompress(&lcw, raw.len()).expect("fixture LCW decodes");
    assert_eq!(inflated, raw, "LCW payload inflates to the raw image");

    let from_raw = load_bin(fixtures_dir().join("td/FIXTURE_RAW.bin"), Some(SIDE as u32))
        .expect("raw image loads");
    let from_lcw = load_bin(fixtures_dir().join("td/FIXTURE.bin"), None)
        .expect("LCW image loads without a pinned side");
    assert_eq!(from_raw, from_lcw);
    assert_eq!((from_lcw.width, from_lcw.height), (16, 16));

    // Hand-checked template histogram: diagonal / outer ring / interior.
    let mut diagonal = 0;
    let mut ring = 0;
    let mut interior = 0;
    for cell in &from_lcw.cells {
        match cell.template {
            12 => diagonal += 1,
            5 => ring += 1,
            7 => interior += 1,
            other => panic!("unexpected template {other}"),
        }
    }
    assert_eq!((diagonal, ring, interior), (16, 58, 182));

    // Anchor cells prove both template and icon bytes survive the codec.
    assert_eq!(
        from_lcw.get(0, 0),
        Some(BinCell {
            template: 12,
            icon: 0
        })
    );
    assert_eq!(
        from_lcw.get(1, 0),
        Some(BinCell {
            template: 5,
            icon: 3
        })
    );
    assert_eq!(
        from_lcw.get(3, 2),
        Some(BinCell {
            template: 7,
            icon: 3
        })
    );
    assert_eq!(
        from_lcw.get(15, 15),
        Some(BinCell {
            template: 12,
            icon: 4
        })
    );
}

#[test]
fn fixture_td_scenario_loads_with_its_sibling_bin() {
    let ini_path = fixtures_dir().join("td/FIXTURE.INI");
    assert_eq!(
        bin_path_for(&ini_path),
        fixtures_dir().join("td/FIXTURE.bin")
    );

    let (scenario, map) = load_scenario(&ini_path).expect("paired scenario loads");
    assert_eq!(scenario.units.len(), 3);
    let map = map.expect("sibling .bin is found");
    assert_eq!((map.width, map.height, map.cells.len()), (16, 16, CELLS));
}

// ---------------------------------------------------------------------------
// Red Alert fixture
// ---------------------------------------------------------------------------

fn ra_fixture_ini() -> IniFile {
    let text =
        std::fs::read_to_string(fixtures_dir().join("ra/FIXTURE.MPR")).expect("RA fixture MPR");
    IniFile::parse(&text).expect("RA fixture INI parses")
}

#[test]
fn fixture_ra_mpr_carries_every_required_section() {
    let ini = ra_fixture_ini();
    for name in [
        "Basic",
        "MapPack",
        "OverlayPack",
        "Houses",
        "Waypoints",
        "Trigs",
    ] {
        assert!(ini.section(name).is_some(), "missing [{name}]");
    }

    let basic = ini.section("Basic").expect("[Basic]");
    assert_eq!(
        basic.get_string("Name"),
        Some("Synthetic RA Golden Fixture")
    );
    assert_eq!(basic.get_string("Player"), Some("Greece"));

    let houses = ini.section("Houses").expect("[Houses]");
    assert_eq!(houses.values.len(), 2);
    assert_eq!(houses.get_string("0"), Some("Greece"));
    assert_eq!(houses.get_string("1"), Some("USSR"));
    assert!(ini.section("Greece").is_some());
    assert!(ini.section("USSR").is_some());

    let waypoints = ini.section("Waypoints").expect("[Waypoints]");
    assert_eq!(waypoints.get_i64("0").ok(), Some(17));
    assert_eq!(waypoints.get_i64("1").ok(), Some(240));
    assert_eq!(waypoints.get_i64("2").ok(), Some(100));

    let trigs = ini.section("Trigs").expect("[Trigs]");
    assert_eq!(trigs.values.len(), 2);
    assert!(ini.section("TRIGA").is_some());
    assert!(ini.section("TRIGB").is_some());

    assert_eq!(ini.section("Units").expect("[Units]").values.len(), 2);
    assert_eq!(ini.section("Infantry").expect("[Infantry]").values.len(), 1);
    assert_eq!(
        ini.section("Structures")
            .expect("[Structures]")
            .values
            .len(),
        2
    );
}

#[test]
fn fixture_ra_mappack_lcw_decodes_to_expected_cells() {
    let ini = ra_fixture_ini();
    let lcw = base64_block(ini.section("MapPack").expect("[MapPack]"));
    let plain = format80::decompress(&lcw, CELLS * ra_pack::PACK_ENTRY_BYTES)
        .expect("MapPack LCW payload decodes");
    assert_eq!(plain.len(), CELLS * 3);
    assert_ne!(plain, lcw, "MapPack is LCW-encoded, not raw");

    let cells = ra_pack::decode_pack_bytes(&plain).expect("3-byte records");
    assert_eq!(cells.len(), CELLS);
    for (index, cell) in cells.iter().enumerate() {
        assert_eq!(cell.index as usize, index, "row-major indices");
    }

    let diagonal = cells.iter().filter(|cell| cell.id == 11).count();
    let corners = cells.iter().filter(|cell| cell.id == 22).count();
    let empty = cells.iter().filter(|cell| cell.id == 0).count();
    assert_eq!((diagonal, corners, empty), (16, 2, 238));

    assert_eq!(cells[0], PackedCell { index: 0, id: 11 });
    assert_eq!(cells[15], PackedCell { index: 15, id: 22 });
    assert_eq!(cells[240], PackedCell { index: 240, id: 22 });
    assert_eq!(cells[255], PackedCell { index: 255, id: 11 });
}

#[test]
fn fixture_ra_overlaypack_lcw_decodes_to_expected_cells() {
    let ini = ra_fixture_ini();
    let lcw = base64_block(ini.section("OverlayPack").expect("[OverlayPack]"));
    let plain = format80::decompress(&lcw, 2 * ra_pack::PACK_ENTRY_BYTES)
        .expect("OverlayPack LCW payload decodes");
    assert_ne!(plain, lcw);

    let cells = ra_pack::decode_pack_bytes(&plain).expect("3-byte records");
    assert_eq!(
        cells,
        vec![
            PackedCell { index: 0, id: 3 },
            PackedCell { index: 255, id: 3 },
        ]
    );
}

#[test]
fn fixture_ra_payload_lines_are_numbered_base64() {
    let ini = ra_fixture_ini();
    for name in ["MapPack", "OverlayPack"] {
        let section = ini.section(name).expect("packed section");
        let mut keys: Vec<u64> = section
            .values
            .keys()
            .map(|key| key.parse::<u64>().expect("numeric key"))
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            (1..=keys.len() as u64).collect::<Vec<_>>(),
            "[{name}] keys are 1..N"
        );
        for value in section.values.values() {
            let text = ini_text(value);
            assert!(
                text.chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=')),
                "[{name}] value is base64: {text}"
            );
            assert!(text.len() <= 70, "[{name}] line respects Put_UUBlock width");
        }
    }
}

// ---------------------------------------------------------------------------
// Checksums + optional real-data path
// ---------------------------------------------------------------------------

#[test]
fn fixture_checksums_pin_every_committed_fixture_file() {
    let manifest =
        std::fs::read_to_string(fixtures_dir().join("CHECKSUMS.sha256")).expect("CHECKSUMS.sha256");
    let root = fixtures_dir();
    let mut listed = Vec::new();
    for line in manifest.lines() {
        let (digest, path) = line.split_once("  ").expect("sha256sum format");
        assert_eq!(digest.len(), 64, "SHA-256 hex: {line}");
        assert!(
            digest.chars().all(|c| c.is_ascii_hexdigit()),
            "SHA-256 hex: {line}"
        );
        let path = root.join(path);
        assert!(path.is_file(), "pinned file exists: {}", path.display());
        listed.push(
            path.strip_prefix(&root)
                .expect("inside fixtures")
                .to_owned(),
        );
    }

    for expected in [
        "README.md",
        "tools/gen_fixtures.py",
        "td/FIXTURE.INI",
        "td/FIXTURE.bin",
        "td/FIXTURE_RAW.bin",
        "ra/FIXTURE.MPR",
    ] {
        assert!(
            listed.iter().any(|path| path == Path::new(expected)),
            "CHECKSUMS.sha256 must pin {expected}"
        );
    }
}

fn collect_scenarios(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    if depth > 3 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if file_type.is_dir() {
            collect_scenarios(&path, depth + 1, out);
        } else if file_type.is_file()
            && path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| {
                    ext.eq_ignore_ascii_case("ini") || ext.eq_ignore_ascii_case("mpr")
                })
        {
            out.push(path);
        }
    }
}

#[test]
fn fixture_real_data_env_is_optional_and_never_fails() {
    // CI runs without retail data. Unset -> skip with a warning.
    let Some(root) = std::env::var_os("CNC_ORIGINAL_DATA_DIR") else {
        eprintln!("warning: CNC_ORIGINAL_DATA_DIR is unset; skipping real-data fixture check");
        return;
    };
    let root = PathBuf::from(root);
    if !root.is_dir() {
        eprintln!(
            "warning: CNC_ORIGINAL_DATA_DIR={} is not a directory; skipping real-data fixture check",
            root.display()
        );
        return;
    }

    let mut candidates = Vec::new();
    collect_scenarios(&root, 0, &mut candidates);
    candidates.sort();
    if candidates.is_empty() {
        eprintln!(
            "warning: {} has no .ini/.mpr scenario files; skipping real-data fixture check",
            root.display()
        );
        return;
    }

    let mut parsed = 0usize;
    let mut skipped = 0usize;
    for path in candidates.iter().take(64) {
        match load_scenario(path) {
            Ok((scenario, _map)) => {
                parsed += 1;
                eprintln!(
                    "real-data scenario: {} -> {} unit(s), {} structure(s)",
                    path.display(),
                    scenario.units.len(),
                    scenario.structures.len()
                );
            }
            Err(error) => {
                skipped += 1;
                eprintln!(
                    "warning: {} did not load ({error:#}); skipping",
                    path.display()
                );
            }
        }
    }
    eprintln!("real-data fixture check: {parsed} parsed, {skipped} skipped");
    if parsed == 0 {
        eprintln!(
            "warning: no candidate file parsed; treating the directory as unusable and skipping"
        );
    }
}
