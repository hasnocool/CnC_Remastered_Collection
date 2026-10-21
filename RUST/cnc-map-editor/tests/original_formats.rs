//! Opening and saving retail-format maps through the plugin load/save path.
//!
//! The golden fixtures live in `RUST/fixtures` (todo 11); this suite proves the
//! editor opens each dialect, writes it back through the `cnc-asset` codecs,
//! and re-opens to the same sections.

use std::path::{Path, PathBuf};

use cnc_core::IniFile;
use cnc_map_editor::map::ImportKind;
use cnc_map_editor::{load_scenario, save_scenario, MapModel};

fn fixture(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures")
        .join(relative)
}

fn temp_dir(tag: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("cnc-editor-original-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

/// Canonical value text so two parses compare equal regardless of formatting.
fn canonical(value: &cnc_core::IniValue) -> String {
    use cnc_core::IniValue;
    match value {
        IniValue::String(text) => text.clone(),
        IniValue::Integer(number) => number.to_string(),
        IniValue::Float(number) => format!("{number:?}"),
        IniValue::Boolean(flag) => flag.to_string(),
        IniValue::Array(values) => values.iter().map(canonical).collect::<Vec<_>>().join(","),
    }
}

/// Compare two scenario files section-by-section, decoding the RA packs so a
/// re-encoded but semantically identical grid still compares equal.
fn sections_equal(left: &str, right: &str) -> bool {
    let left = IniFile::parse(left).expect("parse original");
    let right = IniFile::parse(right).expect("parse saved");
    if left.sections.len() != right.sections.len() {
        return false;
    }
    for (name, left_section) in &left.sections {
        let Some(right_section) = right.section(name) else {
            return false;
        };
        let packed =
            name.eq_ignore_ascii_case("MapPack") || name.eq_ignore_ascii_case("OverlayPack");
        if packed {
            let left_entries: Vec<(String, String)> = left_section
                .values
                .iter()
                .map(|(k, v)| (k.clone(), canonical(v)))
                .collect();
            let right_entries: Vec<(String, String)> = right_section
                .values
                .iter()
                .map(|(k, v)| (k.clone(), canonical(v)))
                .collect();
            let decode = |entries: &[(String, String)]| {
                cnc_asset::ra_pack::decode_pack(entries)
                    .map(|cells| {
                        cells
                            .iter()
                            .map(|cell| format!("{}:{}", cell.index, cell.id))
                            .collect::<Vec<_>>()
                            .join(",")
                    })
                    .unwrap_or_default()
            };
            if decode(&left_entries) != decode(&right_entries) {
                return false;
            }
            continue;
        }
        if left_section.values.len() != right_section.values.len() {
            return false;
        }
        for (key, left_value) in &left_section.values {
            let Some(right_value) = right_section.get(key) else {
                return false;
            };
            if canonical(left_value) != canonical(right_value) {
                return false;
            }
        }
    }
    true
}

fn assert_same_model(left: &MapModel, right: &MapModel) {
    assert_eq!(left.width, right.width, "width");
    assert_eq!(left.height, right.height, "height");
    assert_eq!(left.name, right.name, "name");
    assert_eq!(left.player, right.player, "player");
    assert_eq!(left.briefing, right.briefing, "briefing");
    assert_eq!(left.theater, right.theater, "theater");
    assert_eq!(left.terrain, right.terrain, "terrain grid");
    assert_eq!(left.overlay, right.overlay, "overlay grid");
    assert_eq!(left.icon, right.icon, "icon grid");
    assert_eq!(left.houses, right.houses, "houses");
    assert_eq!(left.waypoints, right.waypoints, "waypoints");
    assert_eq!(left.objects, right.objects, "objects");
    assert_eq!(left.triggers, right.triggers, "triggers");
}

#[test]
fn editor_roundtrips_original_td_fixture() {
    let dir = temp_dir("td");
    let source = fixture("td/FIXTURE.INI");
    let source_text = std::fs::read_to_string(&source).expect("read fixture");

    let (model, kind) = load_scenario(&source).expect("open TD fixture");
    assert_eq!(kind, ImportKind::OriginalTd);
    assert_eq!((model.width, model.height), (16, 16));
    assert_eq!(model.objects.len(), 5, "3 units and 2 structures");
    assert_eq!(model.houses.len(), 2);
    assert_eq!(model.waypoints.len(), 3);
    assert_eq!(model.triggers.len(), 2, "named trigger sections");
    assert!(model.icon.iter().any(|icon| *icon != 0), "icon grid");
    assert!(
        model.terrain.iter().any(|template| *template != 0),
        "template grid"
    );

    let target = dir.join("ROUNDTRIP.INI");
    let written = save_scenario(&model, &target).expect("save TD scenario");
    assert_eq!(written.len(), 2, "the .ini and its .bin are both written");
    assert!(target.exists());
    assert!(target.with_extension("bin").exists(), "sibling cell image");

    let (reopened, kind) = load_scenario(&target).expect("re-open TD scenario");
    assert_eq!(kind, ImportKind::OriginalTd);
    assert_same_model(&model, &reopened);

    let saved_text = std::fs::read_to_string(&target).expect("read saved");
    assert!(
        sections_equal(&source_text, &saved_text),
        "TD sections survive the round-trip:\n{saved_text}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn editor_roundtrips_original_ra_fixture() {
    let dir = temp_dir("ra");
    let source = fixture("ra/FIXTURE.MPR");
    let source_text = std::fs::read_to_string(&source).expect("read fixture");

    let (model, kind) = load_scenario(&source).expect("open RA fixture");
    assert_eq!(kind, ImportKind::OriginalRa);
    assert_eq!((model.width, model.height), (16, 16));
    assert_eq!(model.objects.len(), 5, "2 units, 1 infantry, 2 structures");
    assert_eq!(model.houses.len(), 2);
    assert_eq!(model.waypoints.len(), 3);
    assert_eq!(model.terrain[0], 11, "diagonal template");
    assert_eq!(model.overlay[0], 3, "road overlay at cell 0");
    assert_eq!(model.overlay[255], 3, "road overlay at cell 255");

    let target = dir.join("ROUNDTRIP.MPR");
    let written = save_scenario(&model, &target).expect("save RA scenario");
    assert_eq!(written.len(), 1);
    assert!(target.exists());

    let (reopened, kind) = load_scenario(&target).expect("re-open RA scenario");
    assert_eq!(kind, ImportKind::OriginalRa);
    assert_same_model(&model, &reopened);

    let saved_text = std::fs::read_to_string(&target).expect("read saved");
    assert!(
        sections_equal(&source_text, &saved_text),
        "RA sections survive the round-trip:\n{saved_text}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn editor_preserves_basic_and_house_sections() {
    let dir = temp_dir("sections");

    for (name, source) in [
        ("td", fixture("td/FIXTURE.INI")),
        ("ra", fixture("ra/FIXTURE.MPR")),
    ] {
        let (model, _kind) = load_scenario(&source).expect("open fixture");
        let target = dir.join(format!(
            "{name}.{}",
            if name == "ra" { "MPR" } else { "INI" }
        ));
        save_scenario(&model, &target).expect("save fixture");

        let text = std::fs::read_to_string(&target).expect("read saved");
        let ini = IniFile::parse(&text).expect("parse saved");
        let basic = ini.section("Basic").expect("[Basic] survives");
        assert_eq!(
            basic.get_string("Name"),
            Some(model.name.as_str()),
            "[Basic] Name {name}"
        );
        assert_eq!(
            basic.get_string("Briefing").map(str::trim),
            Some(model.briefing.trim()),
            "[Basic] Briefing {name}"
        );

        let houses = ini.section("Houses").expect("[Houses] survives");
        assert_eq!(houses.values.len(), 2, "[Houses] list {name}");
        let greece = ini
            .section(&houses.values.values().next().map(canonical).unwrap())
            .expect("first house section");
        assert!(
            greece.get("Credits").is_some(),
            "per-house [{}] survives {name}",
            greece.name
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn editor_preserves_sections_the_model_does_not_understand() {
    let dir = temp_dir("unknown");
    let source = fixture("ra/FIXTURE.MPR");
    let mut text = std::fs::read_to_string(&source).expect("read fixture");
    text.push_str("[CustomRules]\nFoo=Bar\nBaz=7\n");

    let (model, kind) = MapModel::import_text_with(&text, None).expect("import RA");
    assert_eq!(kind, ImportKind::OriginalRa);
    let target = dir.join("UNKNOWN.MPR");
    save_scenario(&model, &target).expect("save");

    let saved = std::fs::read_to_string(&target).expect("read saved");
    let ini = IniFile::parse(&saved).expect("parse saved");
    let custom = ini.section("CustomRules").expect("unknown section kept");
    assert_eq!(custom.get_string("Foo"), Some("Bar"));
    assert_eq!(custom.get_i64("Baz").unwrap(), 7);

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn malformed_and_unsupported_inputs_are_errors_not_panics() {
    let dir = temp_dir("errors");
    let unsupported = dir.join("map.txt");
    std::fs::write(&unsupported, "[Basic]\nName=X\n").unwrap();
    let error = load_scenario(&unsupported).expect_err("unsupported extension");
    let message = error.to_string();
    assert!(message.contains("unsupported"), "{message}");
    assert!(message.contains(".ini"), "{message} (.ini listed)");
    assert!(message.contains(".mpr"), "{message} (.mpr listed)");

    let missing_pack = MapModel::from_original_ra("[Basic]\nName=X\n").expect_err("no MapPack");
    assert!(
        missing_pack.to_string().contains("MapPack"),
        "{missing_pack}"
    );

    let bad_pack = MapModel::from_original_ra("[Basic]\nName=X\n[MapPack]\n1=!!!!not-base64!!!!\n");
    assert!(bad_pack.is_err(), "truncated pack must not panic");

    let bad_bin = MapModel::from_original_td("[Basic]\nName=X\n", &[1, 2, 3]);
    assert!(bad_bin.is_err(), "undecodable cell image must not panic");

    let bad_ini = MapModel::from_original_ra("[Basic\nbroken");
    assert!(bad_ini.is_err());

    let _ = std::fs::remove_dir_all(&dir);
}
