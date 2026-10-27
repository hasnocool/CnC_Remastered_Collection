//! Original-format TD scenarios must build a playable game.
//!
//! These tests drive the *production* path end to end:
//! [`cnc_tiberian_dawn::scenario_loader::load_scenario`] reads the sibling
//! `.ini` + `.bin`, and [`scenario_to_game`] turns the parsed
//! scenario into a live simulation (terrain, overlay, units, structures and
//! executable triggers). Everything runs against the synthetic golden
//! fixtures under `RUST/fixtures/td`; the optional real-data check reads
//! `CNC_ORIGINAL_DATA_DIR` and skips cleanly when it is unset or unusable.

use std::path::{Path, PathBuf};

use cnc_core::{IniFile, Vec2i};
use cnc_tiberian_dawn::scenario_loader::{
    load_scenario, parse_scenario, scenario_to_game, ParsedScenario, ScenarioMap,
};
use cnc_tiberian_dawn::{StructureType, UnitType};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures")
}

fn fixture_scenario() -> (ParsedScenario, ScenarioMap) {
    let path = fixtures_dir().join("td/FIXTURE.INI");
    let (scenario, map) = load_scenario(&path).expect("fixture scenario loads");
    let map = map.expect("fixture ships its sibling .bin");
    (scenario, map)
}

fn health_fraction(health: f32, max_health: f32, fraction: u8) -> bool {
    let expected = max_health * (f32::from(fraction) / 256.0);
    (health - expected).abs() <= max_health * 0.001
}

#[test]
fn original_td_scenario_builds_game_with_terrain_overlay_units() {
    let (scenario, image) = fixture_scenario();
    let (game, report) = scenario_to_game(&scenario, Some(&image));

    // --- terrain + overlay copied out of the .bin image -------------------
    assert_eq!((game.map.width, game.map.height), (16, 16));
    let mut terrain_cells = 0usize;
    let mut overlay_cells = 0usize;
    for y in 0..game.map.height {
        for x in 0..game.map.width {
            let cell = game
                .map
                .grid
                .get(Vec2i::new(x as i32, y as i32))
                .expect("cell inside the grid");
            let source = image.get(x, y).expect("cell inside the bin image");
            // Every bin cell lands verbatim: template -> terrain_type,
            // icon -> template_icon (MAP.CPP Read_Binary).
            assert_eq!(cell.terrain_type, source.template);
            assert_eq!(cell.template_type, source.template);
            assert_eq!(cell.template_icon, source.icon);
            if cell.terrain_type != 0 {
                terrain_cells += 1;
            }
            if cell.template_icon != 0 {
                overlay_cells += 1;
            }
        }
    }
    assert_eq!(terrain_cells, 256, "every fixture cell carries a template");
    assert!(overlay_cells > 0, "fixture .bin carries cell icons");
    // Anchor cells prove the codec + conversion agree byte for byte.
    let anchor = game.map.grid.get(Vec2i::new(1, 0)).expect("anchor cell");
    assert_eq!((anchor.terrain_type, anchor.template_icon), (5, 3));

    // --- units: house + health from the original 0..=255 fraction --------
    assert_eq!(game.units.len(), 3);
    assert_eq!(report.units_placed, 3);
    let e1 = game
        .units
        .iter()
        .find(|unit| unit.unit_type == UnitType::InfantryE1)
        .expect("GDI E1");
    assert_eq!(e1.owner, 0);
    assert!(health_fraction(
        e1.health.to_f32(),
        e1.max_health.to_f32(),
        255
    ));
    let mtnk = game
        .units
        .iter()
        .find(|unit| unit.unit_type == UnitType::MediumTank)
        .expect("GDI MTNK");
    assert_eq!(mtnk.owner, 0);
    assert!(health_fraction(
        mtnk.health.to_f32(),
        mtnk.max_health.to_f32(),
        200
    ));
    let buggy = game
        .units
        .iter()
        .find(|unit| unit.unit_type == UnitType::Buggy)
        .expect("Nod BGGY");
    assert_eq!(buggy.owner, 1);
    assert!(health_fraction(
        buggy.health.to_f32(),
        buggy.max_health.to_f32(),
        180
    ));

    // --- 2 structures ----------------------------------------------------
    assert_eq!(game.buildings.len(), 2);
    assert_eq!(report.structures_placed, 2);
    let yard = game
        .buildings
        .iter()
        .find(|building| building.structure == StructureType::ConstructionYard)
        .expect("GDI FACT");
    assert_eq!(yard.owner, 0);
    assert!(health_fraction(
        yard.health.to_f32(),
        yard.max_health.to_f32(),
        255
    ));
    let obelisk = game
        .buildings
        .iter()
        .find(|building| building.structure == StructureType::Obelisk)
        .expect("Nod OBLI");
    assert_eq!(obelisk.owner, 1);

    // --- 2 executable triggers installed on the mission ------------------
    assert_eq!(report.triggers_installed, 2);
    // "Player Enters" (EVENT_PLAYER_ENTERED) has no engine condition yet,
    // so it is reported rather than guessed as "always" (which would make
    // its `Win` action fire on the first tick).
    assert_eq!(
        report.unmapped_triggers,
        vec!["event: Player Enters".to_string()]
    );
    assert_eq!(game.mission.triggers.len(), 2);
    let names: Vec<&str> = game
        .mission
        .triggers
        .iter()
        .map(|trigger| trigger.name.as_str())
        .collect();
    assert_eq!(names, vec!["T_CLOCK", "T_ENTRY"]);
    // The T_CLOCK record is persistent, so the trigger is repeatable.
    let clock = game
        .mission
        .triggers
        .iter()
        .find(|trigger| trigger.name == "T_CLOCK")
        .expect("T_CLOCK");
    assert!(!clock.once, "IsPersistant > 0 means repeatable");
    // --- 3 waypoints -----------------------------------------------------
    assert_eq!(scenario.waypoint_ids, vec![0, 1, 2]);
    assert_eq!(scenario.waypoints.len(), 3);
    assert_eq!(
        scenario
            .waypoints
            .iter()
            .map(|waypoint| (waypoint.id, waypoint.cell))
            .collect::<Vec<_>>(),
        vec![(0, 55), (1, 170), (2, 255)]
    );
    assert_eq!(report.waypoints.len(), 3);
    assert_eq!(game.mission.objectives.len(), 2);
    assert!(!game.mission.briefing.is_empty());
    assert!(
        report.has_gaps(),
        "the Player Enters vocabulary gap must stay visible: {report:?}"
    );
    assert!(report.warnings.is_empty(), "{report:?}");
    assert!(game.player_owner == 0);
}

#[test]
fn malformed_sections_report_warnings_instead_of_panicking() {
    // Missing [Basic] is the one fatal shape: the loader returns an error and
    // never panics.
    let empty = IniFile::parse("[Wunits]\nUnit1=UNIT_MISSING\n").expect("INI parses");
    let error = parse_scenario(&empty).expect_err("no [Basic] is fatal");
    assert!(
        format!("{error:#}").contains("no [Basic] section"),
        "{error:#}"
    );

    // Every other malformed section is recoverable: the loader reports it in
    // `warnings` and keeps whatever did parse.
    let text = [
        "[Basic]",
        "Name=Malformed Fixture",
        "Player=0",
        "[Wunits]",
        "Unit1=UNIT_MISSING",
        "Unit2=UNIT_NO_TYPE",
        "[UNIT_NO_TYPE]",
        "House=0",
        "X=1",
        "Y=1",
        "[Structures]",
        "Structure1=STRUCT_MISSING",
        "[Trigs]",
        "Trigger1=TRIG_MISSING",
        "[TRIGGER]",
        "T_BAD=Not An Event,Not An Action,12,,,0",
        "[Waypoints]",
        "0=notacell",
        "1=-1",
        "Weird=5",
    ]
    .join("\n");
    let ini = IniFile::parse(&text).expect("INI parses");
    let scenario = parse_scenario(&ini).expect("malformed optional sections still parse");

    let warnings = scenario.warnings.join("\n");
    assert!(
        warnings.contains("UNIT_MISSING") && warnings.contains("no matching"),
        "missing unit section reported: {warnings}"
    );
    assert!(warnings.contains("UNIT_NO_TYPE"), "{warnings}");
    assert!(warnings.contains("STRUCT_MISSING"), "{warnings}");
    assert!(warnings.contains("TRIG_MISSING"), "{warnings}");
    assert!(
        warnings.contains("unknown event 'Not An Event'"),
        "{warnings}"
    );
    assert!(
        warnings.contains("unknown action 'Not An Action'"),
        "{warnings}"
    );
    assert!(warnings.contains("notacell"), "{warnings}");
    assert!(warnings.contains("Weird"), "{warnings}");

    // Only the placeholder-less unit survived; nothing panicked while
    // converting the rest.
    assert_eq!(scenario.units.len(), 1);
    assert!(scenario.units[0].unit_type.is_empty());
    assert!(scenario.structures.is_empty());
    assert!(scenario.triggers.is_empty());
    let (game, report) = scenario_to_game(&scenario, None);
    assert_eq!(game.units.len(), 0);
    assert!(report.has_gaps());
    assert!(
        !report.warnings.is_empty(),
        "warnings carry into the report"
    );
}

/// `CNC_ORIGINAL_DATA_DIR` when the operator pointed at a retail install.
fn real_data_root() -> Option<PathBuf> {
    let raw = std::env::var_os("CNC_ORIGINAL_DATA_DIR")?;
    let root = PathBuf::from(raw);
    if !root.is_dir() {
        eprintln!(
            "warning: CNC_ORIGINAL_DATA_DIR={} is not a directory; skipping real-data integration check",
            root.display()
        );
        return None;
    }
    Some(root)
}

fn collect_scenarios(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    if depth > 3 || out.len() >= 512 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_scenarios(&path, depth + 1, out);
        } else if path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
        {
            out.push(path);
        }
    }
}

/// Env-gated integration check: a real retail campaign directory must load
/// through the production path. When the variable is unset or unusable the
/// test skips with a warning instead of failing.
#[test]
fn real_td_campaign_builds_games_when_provided() {
    let Some(root) = real_data_root() else {
        eprintln!(
            "warning: CNC_ORIGINAL_DATA_DIR unset; skipping real TD campaign integration check"
        );
        return;
    };
    let mut candidates = Vec::new();
    collect_scenarios(&root, 0, &mut candidates);
    candidates.sort();
    if candidates.is_empty() {
        eprintln!(
            "warning: {} has no .ini scenarios; skipping real TD campaign integration check",
            root.display()
        );
        return;
    }

    let mut built = 0usize;
    let mut skipped = 0usize;
    for path in candidates.iter().take(32) {
        let Ok(text) = std::fs::read_to_string(path) else {
            skipped += 1;
            continue;
        };
        if !cnc_tiberian_dawn::scenario_loader::looks_like_original_scenario(&text) {
            continue;
        }
        match load_scenario(path) {
            Ok((scenario, map)) => {
                let (game, report) = scenario_to_game(&scenario, map.as_ref());
                assert_eq!(game.units.len(), report.units_placed);
                assert_eq!(game.buildings.len(), report.structures_placed);
                assert_eq!(game.mission.triggers.len(), report.triggers_installed);
                eprintln!(
                    "real TD scenario: {} -> {} unit(s), {} structure(s), {} trigger(s), {} warning(s)",
                    path.display(),
                    game.units.len(),
                    game.buildings.len(),
                    game.mission.triggers.len(),
                    report.warnings.len()
                );
                built += 1;
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
    eprintln!("real TD campaign integration check: {built} built, {skipped} skipped");
    if built == 0 {
        eprintln!(
            "warning: no original-format scenario built a game; treating the directory as unusable"
        );
    }
}
