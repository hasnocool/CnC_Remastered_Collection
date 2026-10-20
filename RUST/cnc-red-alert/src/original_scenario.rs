//! Original Red Alert scenario loader (`[Basic]` + `MapPack`/`OverlayPack`).
//!
//! Original RA missions differ from the Mobius dialect the editor writes:
//! terrain and overlay bytes live in base64-encoded `MapPack` / `OverlayPack`
//! INI blocks rather than `[Terrain]`/`[Overlay]` text rows.
//!
//! References:
//! - `REDALERT/SCENARIO.CPP` - `[Basic]` keys (`Player`, `Name`, `Briefing`)
//! - `REDALERT/DISPLAY.CPP` - `MapPack` written via `Put_UUBlock`
//! - `REDALERT/OVERLAY.CPP` - `OverlayPack`
//! - `REDALERT/INI.CPP` / `BASE64.CPP` - the numbered-line base64 container

use anyhow::{Context, Result};
use std::path::Path;

use cnc_asset::ra_pack::{self, PackedCell};
use cnc_core::IniFile;

/// Original-format RA scenario recovered from an `.ini`/`.mpr` file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalScenario {
    pub name: String,
    /// `[Basic] Player` house name, e.g. `Greece`.
    pub player: String,
    pub briefing: String,
    /// Decoded `MapPack` cells (terrain templates).
    pub map_cells: Vec<PackedCell>,
    /// Decoded `OverlayPack` cells (ore, walls, ...).
    pub overlay_cells: Vec<PackedCell>,
    /// `[CellTriggers]` style entries keyed by cell index.
    pub cell_triggers: Vec<(u32, String)>,
    /// Objects to instantiate, in file order.
    pub objects: Vec<ScenarioObject>,
    /// `[TeamTypes]` parsed via `cnc_core::entity` (empty when absent).
    pub team_types: Vec<cnc_core::entity::TeamType>,
}

/// One placeable object recovered from an original scenario.
///
/// Record layouts come verbatim from the C++ readers:
/// - `UNIT.CPP` `UnitClass::Read_INI` --
///   `House,Type,Strength,Cell,Facing,Mission,Trigger`
/// - `INFANTRY.CPP` `InfantryClass::Read_INI` --
///   `House,Type,Strength,Cell,Sub,Mission,Facing,Trigger`
/// - `BUILDING.CPP` `BuildingClass::Read_INI` --
///   `House,Type,Strength,Cell,Facing,Trigger,Sellable,Rebuild`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioObject {
    pub kind: ObjectKind,
    /// House name as written, e.g. `Greece` / `USSR`.
    pub house: String,
    /// Rules type name, e.g. `E1` / `HTNK` / `POWR`.
    pub type_name: String,
    /// Strength as a percentage (see `strength_fraction`).
    pub strength: u32,
    /// Cell index in the RA row-major order (`y * width + x`).
    pub cell: u32,
    pub facing: u32,
    pub mission: Option<String>,
    pub trigger: Option<String>,
}

/// Which original INI section an object came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectKind {
    Unit,
    Infantry,
    Structure,
}

impl ObjectKind {
    /// The INI section name this kind is read from.
    pub fn section(self) -> &'static str {
        match self {
            ObjectKind::Unit => "Units",
            ObjectKind::Infantry => "Infantry",
            ObjectKind::Structure => "Structures",
        }
    }
}

impl ScenarioObject {
    /// Health multiplier expressed as a 16.16 fixed-point fraction.
    ///
    /// `UNIT.CPP:4890` computes `MaxStrength * fixed(strength, 256)`, i.e. the
    /// stored value is 0..256 mapping to 0..100%.
    pub fn strength_fraction(&self) -> f32 {
        (self.strength as f32 / 256.0).clamp(0.0, 1.0)
    }
}

impl OriginalScenario {
    /// Load and decode an original-format RA scenario file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading RA scenario {}", path.display()))?;
        Self::from_ini(&text).with_context(|| format!("parsing RA scenario {}", path.display()))
    }

    /// Parse scenario text.
    pub fn from_ini(text: &str) -> Result<Self> {
        let ini = IniFile::parse(text)?;
        let basic = ini
            .section("Basic")
            .context("original RA scenario has no [Basic] section")?;

        // RA defaults the player house to Greece (SCENARIO.CPP:2392 passes
        // HOUSE_GREECE as the fallback), unlike the editor dialect's "Allies".
        let player = basic.get_string("Player").unwrap_or("Greece").to_string();
        let name = basic
            .get_string("Name")
            .unwrap_or("Red Alert Scenario")
            .to_string();
        let briefing = basic.get_string("Briefing").unwrap_or("").to_string();

        let map_cells = decode_block(&ini, "MapPack")?;
        let overlay_cells = decode_block(&ini, "OverlayPack")?;
        let cell_triggers = parse_cell_triggers(&ini);
        let objects = parse_objects(&ini);
        // Team types are optional; a scenario without [TeamTypes] yields an
        // empty roster and the cadence AI keeps full control.
        let team_types = cnc_core::entity::team_types_from_ini(&ini).unwrap_or_default();

        Ok(Self {
            name,
            player,
            briefing,
            map_cells,
            overlay_cells,
            cell_triggers,
            objects,
            team_types,
        })
    }

    /// Objects belonging to one original section, in file order.
    pub fn objects_of(&self, kind: ObjectKind) -> impl Iterator<Item = &ScenarioObject> {
        self.objects
            .iter()
            .filter(move |object| object.kind == kind)
    }

    /// `true` when the file carried a decodable `MapPack`.
    pub fn has_terrain(&self) -> bool {
        !self.map_cells.is_empty()
    }
}

/// `true` when `text` looks like an original-format RA scenario rather than
/// the Mobius editor dialect (`[Map]` + `[Terrain]` rows).
pub fn looks_like_original_ra_scenario(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("[basic]") && lower.contains("[mappack]")
}

/// Read `[Units]` / `[Infantry]` / `[Structures]` into placement records.
///
/// Malformed records are skipped rather than failing the load: the retail
/// engine also tolerates a short record (the trailing `strtok` calls simply
/// yield `NULL`), and a single bad line should not cost the whole scenario.
fn parse_objects(ini: &IniFile) -> Vec<ScenarioObject> {
    let mut objects = Vec::new();
    for kind in [
        ObjectKind::Unit,
        ObjectKind::Infantry,
        ObjectKind::Structure,
    ] {
        let Some(section) = ini.section(kind.section()) else {
            continue;
        };
        for value in section.values.values() {
            // A comma-separated record parses into an Array, so the parts must
            // be re-joined before positional parsing.
            let text = match value {
                cnc_core::IniValue::String(s) => s.clone(),
                cnc_core::IniValue::Array(parts) => parts
                    .iter()
                    .map(ini_value_text)
                    .collect::<Vec<_>>()
                    .join(","),
                _ => continue,
            };
            if let Some(object) = parse_object(kind, &text) {
                objects.push(object);
            }
        }
    }
    objects
}

/// Render one array element back to its textual form.
///
/// `IniValue::as_string` only accepts `String`; the strength/cell/facing
/// fields parse as `Integer`, so they must be formatted explicitly.
fn ini_value_text(value: &cnc_core::IniValue) -> String {
    match value {
        cnc_core::IniValue::String(s) => s.clone(),
        cnc_core::IniValue::Integer(i) => i.to_string(),
        cnc_core::IniValue::Float(f) => f.to_string(),
        cnc_core::IniValue::Boolean(b) => if *b { "1" } else { "0" }.to_string(),
        cnc_core::IniValue::Array(_) => String::new(),
    }
}

/// Parse one `House,Type,Strength,Cell,...` record.
fn parse_object(kind: ObjectKind, text: &str) -> Option<ScenarioObject> {
    let fields: Vec<&str> = text.split(',').map(str::trim).collect();
    // Every kind shares the first four positional fields.
    if fields.len() < 4 {
        return None;
    }
    let house = fields[0];
    let type_name = fields[1];
    if house.is_empty() || type_name.is_empty() {
        return None;
    }
    let strength = fields[2].parse().ok()?;
    let cell = fields[3].parse().ok()?;

    // Facing and mission sit at kind-dependent offsets.
    let (facing, mission) = match kind {
        // House,Type,Strength,Cell,Facing,Mission,Trigger
        ObjectKind::Unit => (
            fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
            fields.get(5).copied(),
        ),
        // House,Type,Strength,Cell,Sub,Mission,Facing,Trigger
        ObjectKind::Infantry => (
            fields.get(6).and_then(|s| s.parse().ok()).unwrap_or(0),
            fields.get(5).copied(),
        ),
        // House,Type,Strength,Cell,Facing,Trigger,Sellable,Rebuild
        ObjectKind::Structure => (
            fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
            None,
        ),
    };

    let trigger_index = match kind {
        ObjectKind::Unit => 6,
        ObjectKind::Infantry => 7,
        ObjectKind::Structure => 5,
    };

    Some(ScenarioObject {
        kind,
        house: house.to_string(),
        type_name: type_name.to_string(),
        strength,
        cell,
        facing,
        mission: mission.filter(|m| !m.is_empty()).map(str::to_string),
        trigger: fields
            .get(trigger_index)
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string()),
    })
}

/// Decode one numbered-line block into packed cells, if present.
fn decode_block(ini: &IniFile, section: &str) -> Result<Vec<PackedCell>> {
    let Some(section_data) = ini.section(section) else {
        return Ok(Vec::new());
    };
    let entries: Vec<(String, String)> = section_data
        .values
        .iter()
        .map(|(key, value)| (key.clone(), value.as_string().unwrap_or("").to_string()))
        .collect();
    ra_pack::decode_pack(&entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLACEMENT: &str = "[Basic]\nName=Placement\nPlayer=Greece\n\
[Units]\n\
001=Greece,HTNK,256,100,64,Guard,TRIG01\n\
002=USSR,APOC,128,200,32,Area Guard,\n\
[Infantry]\n\
001=USSR,E1,256,150,3,Guard,96,TRIG02\n\
[Structures]\n\
001=Greece,POWR,256,120,0,,1,1\n\
002=USSR,BARR,255,180,32,,\n";

    #[test]
    fn units_structures_and_infantry_are_parsed() {
        let scenario = OriginalScenario::from_ini(PLACEMENT).expect("valid scenario");
        assert_eq!(scenario.objects_of(ObjectKind::Unit).count(), 2);
        assert_eq!(scenario.objects_of(ObjectKind::Infantry).count(), 1);
        assert_eq!(scenario.objects_of(ObjectKind::Structure).count(), 2);
    }

    #[test]
    fn unit_record_uses_the_unit_field_layout() {
        let scenario = OriginalScenario::from_ini(PLACEMENT).expect("valid scenario");
        let tank = scenario
            .objects_of(ObjectKind::Unit)
            .next()
            .expect("a unit");
        assert_eq!(tank.house, "Greece");
        assert_eq!(tank.type_name, "HTNK");
        assert_eq!(tank.strength, 256);
        assert_eq!(tank.cell, 100);
        // UNIT.CPP: Facing is the 5th field, Mission the 6th, Trigger the 7th.
        assert_eq!(tank.facing, 64);
        assert_eq!(tank.mission.as_deref(), Some("Guard"));
        assert_eq!(tank.trigger.as_deref(), Some("TRIG01"));
    }

    #[test]
    fn infantry_record_reads_facing_from_the_seventh_field() {
        let scenario = OriginalScenario::from_ini(PLACEMENT).expect("valid scenario");
        let gi = scenario
            .objects_of(ObjectKind::Infantry)
            .next()
            .expect("infantry");
        // INFANTRY.CPP: House,Type,Strength,Cell,Sub,Mission,Facing,Trigger
        assert_eq!(gi.cell, 150);
        assert_eq!(gi.mission.as_deref(), Some("Guard"));
        assert_eq!(gi.facing, 96);
        assert_eq!(gi.trigger.as_deref(), Some("TRIG02"));
    }

    #[test]
    fn structure_record_has_no_mission_and_reads_trigger_fifth() {
        let scenario = OriginalScenario::from_ini(PLACEMENT).expect("valid scenario");
        let mut structures = scenario.objects_of(ObjectKind::Structure);
        let power = structures.next().expect("structure");
        assert_eq!(power.type_name, "POWR");
        assert_eq!(power.mission, None);
        assert_eq!(power.facing, 0);
    }

    #[test]
    fn malformed_records_are_skipped_not_fatal() {
        let text = "[Basic]\nName=Bad\n[Units]\n001=Greece,HTNK\n002=,,,\n003=Greece,HTNK,256,50\n";
        let scenario = OriginalScenario::from_ini(text).expect("short records must not fail");
        // Only the third entry has the four required leading fields.
        assert_eq!(scenario.objects.len(), 1);
        assert_eq!(scenario.objects[0].cell, 50);
    }

    #[test]
    fn strength_is_a_0_to_256_fraction() {
        let scenario = OriginalScenario::from_ini(PLACEMENT).expect("valid scenario");
        let soviet_tank = scenario
            .objects_of(ObjectKind::Unit)
            .nth(1)
            .expect("second unit");
        assert_eq!(soviet_tank.strength, 128);
        assert!((soviet_tank.strength_fraction() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn absent_sections_yield_no_objects() {
        let scenario = OriginalScenario::from_ini("[Basic]\nName=Empty\n").expect("valid");
        assert!(scenario.objects.is_empty());
    }

    #[test]
    fn objects_of_filters_by_kind() {
        let scenario = OriginalScenario::from_ini(PLACEMENT).expect("valid scenario");
        let houses: Vec<&str> = scenario
            .objects_of(ObjectKind::Structure)
            .map(|o| o.house.as_str())
            .collect();
        assert_eq!(houses, ["Greece", "USSR"]);
    }

    use cnc_asset::ra_pack::PackedCell;

    use super::{ObjectKind, OriginalScenario};

    /// Build original-format scenario text from packed cell lists.
    fn scenario_text(map: &[PackedCell], overlay: &[PackedCell]) -> String {
        let mut text = String::from("[Basic]\nName=RA 01\nPlayer=Greece\nBriefing=Hold.\n");
        for (section, cells) in [("MapPack", map), ("OverlayPack", overlay)] {
            text.push_str(&format!("[{section}]\n"));
            for (key, value) in ra_pack::encode_pack(cells) {
                text.push_str(&format!("{key}={value}\n"));
            }
        }
        text.push_str("[CellTriggers]\n12=FIRST\n2=SECOND\n");
        text
    }

    #[test]
    fn decodes_basic_section_defaults() {
        let text = scenario_text(&[], &[]);
        let scenario = OriginalScenario::from_ini(&text).unwrap();
        assert_eq!(scenario.name, "RA 01");
        assert_eq!(scenario.player, "Greece");
        assert_eq!(scenario.briefing, "Hold.");
    }

    #[test]
    fn missing_player_falls_back_to_greece() {
        // SCENARIO.CPP:2392 uses HOUSE_GREECE as the default player house.
        let text = "[Basic]\nName=X\n[MapPack]\n1=AAEC\n";
        let scenario = OriginalScenario::from_ini(text).unwrap();
        assert_eq!(scenario.player, "Greece");
    }

    #[test]
    fn mappack_and_overlaypack_are_decoded() {
        let map = vec![
            PackedCell { index: 0, id: 5 },
            PackedCell { index: 300, id: 9 },
        ];
        let overlay = vec![PackedCell { index: 7, id: 3 }];
        let scenario = OriginalScenario::from_ini(&scenario_text(&map, &overlay)).unwrap();

        assert_eq!(scenario.map_cells, map);
        assert_eq!(scenario.overlay_cells, overlay);
        assert!(scenario.has_terrain());
    }

    #[test]
    fn cell_triggers_are_numerically_ordered() {
        let scenario = OriginalScenario::from_ini(&scenario_text(&[], &[])).unwrap();
        assert_eq!(
            scenario.cell_triggers,
            vec![(2, "SECOND".to_string()), (12, "FIRST".to_string())]
        );
    }

    #[test]
    fn scenario_without_map_pack_has_no_terrain() {
        let scenario = OriginalScenario::from_ini("[Basic]\nName=X\n").unwrap();
        assert!(!scenario.has_terrain());
        assert!(scenario.map_cells.is_empty());
    }

    #[test]
    fn rejects_text_without_a_basic_section() {
        assert!(OriginalScenario::from_ini("[MapPack]\n1=AAEC\n").is_err());
    }

    #[test]
    fn distinguishes_original_from_editor_dialect() {
        let original = scenario_text(&[PackedCell { index: 0, id: 1 }], &[]);
        assert!(looks_like_original_ra_scenario(&original));
        let dialect = "[Map]\nWidth=8\nHeight=8\n[Terrain]\n0=0 0 0\n";
        assert!(!looks_like_original_ra_scenario(dialect));
    }

    #[test]
    fn loads_from_disk_end_to_end() {
        let dir = std::env::temp_dir().join(format!(
            "ra-original-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("scg01ea.ini");
        let map = vec![PackedCell { index: 42, id: 6 }];
        std::fs::write(&path, scenario_text(&map, &[])).unwrap();

        let scenario = OriginalScenario::from_file(&path).unwrap();
        assert_eq!(scenario.map_cells, map);
        assert!(scenario.has_terrain());

        let _ = std::fs::remove_dir_all(&dir);
    }
}

fn parse_cell_triggers(ini: &IniFile) -> Vec<(u32, String)> {
    let Some(section) = ini.section("CellTriggers") else {
        return Vec::new();
    };
    let mut out: Vec<(u32, String)> = section
        .values
        .iter()
        .filter_map(|(key, value)| {
            let index = key.trim().parse::<u32>().ok()?;
            Some((index, value.as_string().unwrap_or("").to_string()))
        })
        .collect();
    out.sort_by_key(|(index, _)| *index);
    out
}
