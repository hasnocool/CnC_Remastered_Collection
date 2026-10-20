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

use anyhow::{bail, Context, Result};
use std::path::Path;

use cnc_asset::base64;
use cnc_asset::format80::{self, Format80Error};
use cnc_asset::ra_pack::{self, PackedCell};
use cnc_core::{IniFile, IniSection, IniValue};

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
    /// `[Houses]`: house index -> name, in index order.
    pub houses: Vec<OriginalHouse>,
    /// `[Waypoints]`: waypoint index -> row-major cell index.
    pub waypoints: Vec<Waypoint>,
    /// `[Trigs]` and the per-trigger sections they name.
    pub triggers: Vec<OriginalTrigger>,
    /// `[Base] Player` house name, when the scenario authored a base list.
    pub base_player: Option<String>,
    /// `[Base]` build-order nodes (`000=Type,Cell`).
    pub base_nodes: Vec<BaseNode>,
    /// `[SMUDGE]` craters and scorch marks.
    pub smudges: Vec<SmudgeEntry>,
    /// `[TERRAIN]` trees and other terrain objects.
    pub terrain_objects: Vec<TerrainObject>,
}

/// One `[Houses]` entry plus the matching per-house section (`[Greece]`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalHouse {
    /// House index as written in `[Houses]` (`0`, `1`, ...).
    pub index: u32,
    /// House name, e.g. `Greece` / `USSR`.
    pub name: String,
    /// Starting credits. `HOUSE.CPP` scales the INI value by 100.
    pub credits: i32,
    pub tech_level: Option<u32>,
    pub max_infantry: Option<u32>,
    pub max_unit: Option<u32>,
    pub max_building: Option<u32>,
    pub max_vessel: Option<u32>,
}

/// One `[Waypoints]` entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Waypoint {
    /// Waypoint number (`0`..=N).
    pub index: u32,
    /// Target cell as a row-major index (`y * width + x`).
    pub cell: u32,
}

/// One `[Trigs]` entry, resolved through its per-trigger section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalTrigger {
    /// Index/id as written in `[Trigs]` (or the discovery order).
    pub id: u32,
    /// Reference name `[CellTriggers]` / objects use (`TRIGA`).
    pub name: String,
    /// Display name from the trigger section (`Name=Alpha`).
    pub display_name: String,
    /// Raw event token (`Time`, `All Destr.`, or a numeric record).
    pub event: String,
    /// Raw action token (`Win`, `Lose`, ...).
    pub action: String,
    /// `Data=` payload.
    pub data: i64,
}

/// One `[Base]` build-order node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BaseNode {
    /// Rules building type name, e.g. `POWR`.
    pub type_name: String,
    /// Row-major cell index.
    pub cell: u32,
}

/// One `[SMUDGE]` entry (`SMUDGE.CPP`: `Type,Cell,Data` under a cell key).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmudgeEntry {
    pub cell: u32,
    pub type_name: String,
    pub data: Option<u32>,
}

/// One `[TERRAIN]` entry (`TERRAIN.CPP`: cell key, terrain type value).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerrainObject {
    pub cell: u32,
    pub type_name: String,
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
    /// `AIRCRAFT`: `House,Type,Strength,Cell,Facing,Mission`.
    Aircraft,
    /// `SHIPS`: same record shape as `UNITS`.
    Ship,
}

impl ObjectKind {
    /// The INI section name this kind is read from.
    pub fn section(self) -> &'static str {
        match self {
            ObjectKind::Unit => "Units",
            ObjectKind::Infantry => "Infantry",
            ObjectKind::Structure => "Structures",
            ObjectKind::Aircraft => "Aircraft",
            ObjectKind::Ship => "Ships",
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
        let houses = parse_houses(&ini);
        let waypoints = parse_waypoints(&ini);
        let triggers = parse_triggers(&ini);
        let (base_player, base_nodes) = parse_base(&ini);
        let smudges = parse_smudges(&ini);
        let terrain_objects = parse_terrain_objects(&ini);

        Ok(Self {
            name,
            player,
            briefing,
            map_cells,
            overlay_cells,
            cell_triggers,
            objects,
            team_types,
            houses,
            waypoints,
            triggers,
            base_player,
            base_nodes,
            smudges,
            terrain_objects,
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
        ObjectKind::Aircraft,
        ObjectKind::Ship,
    ] {
        let Some(section) = section_ci(ini, kind.section()) else {
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
        // House,Type,Strength,Cell,Facing,Mission (AIRCRAFT.CPP:561):
        // aircraft carry no trigger field.
        ObjectKind::Aircraft => (
            fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
            fields.get(5).copied(),
        ),
        // House,Type,Strength,Cell,Facing,Mission,Trigger (VESSEL.CPP:2073)
        ObjectKind::Ship => (
            fields.get(4).and_then(|s| s.parse().ok()).unwrap_or(0),
            fields.get(5).copied(),
        ),
    };

    let trigger_index = match kind {
        ObjectKind::Unit | ObjectKind::Ship => 6,
        ObjectKind::Infantry => 7,
        ObjectKind::Structure => 5,
        ObjectKind::Aircraft => usize::MAX,
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

/// Case-insensitive section lookup.
///
/// The retail engine writes the object sections in upper case (`UNITS`,
/// `TERRAIN`, `SMUDGE`) while editors and hand-authored files use title
/// case, so matching is folded rather than exact.
fn section_ci<'a>(ini: &'a IniFile, name: &str) -> Option<&'a IniSection> {
    ini.sections
        .iter()
        .find(|(section, _)| section.eq_ignore_ascii_case(name))
        .map(|(_, section)| section)
}

/// Render one INI value as scenario text (arrays are re-joined with commas).
fn value_text(value: &IniValue) -> Option<String> {
    match value {
        IniValue::String(text) => Some(text.clone()),
        IniValue::Integer(number) => Some(number.to_string()),
        IniValue::Float(number) => Some(number.to_string()),
        IniValue::Boolean(flag) => Some(if *flag { "1" } else { "0" }.to_string()),
        IniValue::Array(values) => Some(
            values
                .iter()
                .map(ini_value_text)
                .collect::<Vec<_>>()
                .join(","),
        ),
    }
}

/// Decode one numbered-line `MapPack`/`OverlayPack` block into packed cells.
///
/// `INIClass::Put_UUBlock` writes numbered base64 lines; both retail packs are
/// LCW-compressed before base64 (`MapClass::Write_Binary`,
/// `OverlayClass::Write_INI`), and the decompressed stream is a run of 3-byte
/// records ([`ra_pack`]). Every failure names the section it came from.
fn decode_block(ini: &IniFile, section: &str) -> Result<Vec<PackedCell>> {
    let Some(section_data) = section_ci(ini, section) else {
        return Ok(Vec::new());
    };
    let entries: Vec<(String, String)> = section_data
        .values
        .iter()
        .map(|(key, value)| (key.clone(), value_text(value).unwrap_or_default()))
        .collect();
    let stream = ra_pack::gather_lines(&entries);
    if stream.trim().is_empty() {
        return Ok(Vec::new());
    }
    let bytes = base64::decode(&stream).with_context(|| format!("[{section}] base64 payload"))?;
    let plain = decompress_lcw(&bytes).with_context(|| format!("[{section}] LCW payload"))?;
    ra_pack::decode_pack_bytes(&plain).with_context(|| format!("[{section}] packed cells"))
}

/// Decompress an LCW stream whose output length the file does not state.
///
/// `format80::decompress` needs the exact output length. The stream's `0x80`
/// terminator reports the real length through `LengthMismatch`, so the block
/// is probed with a generous cap and then decoded once more at the reported
/// size. Only the shared codec runs; there is no local LCW implementation.
/// An opcode that would overrun the cap (a genuinely long stream) doubles the
/// cap instead of being mistaken for the terminator.
fn decompress_lcw(input: &[u8]) -> Result<Vec<u8>> {
    /// Upper bound for one decompressed pack (retail maps are far smaller).
    const MAX_OUTPUT: usize = 64 * 1024 * 1024;
    let mut cap = input.len().saturating_mul(64).clamp(1024, MAX_OUTPUT);
    loop {
        match format80::decompress(input, cap) {
            Ok(plain) => return Ok(plain),
            // The terminator landed short of the cap: `actual` is the real
            // decompressed length.
            Err(Format80Error::LengthMismatch { actual, expected }) if actual <= expected => {
                return format80::decompress(input, actual)
                    .map_err(|err| anyhow::anyhow!("LCW decode failed: {err}"));
            }
            Err(Format80Error::LengthMismatch { .. }) => {
                if cap >= MAX_OUTPUT {
                    bail!("LCW stream expands past {MAX_OUTPUT} bytes");
                }
                cap = cap.saturating_mul(2).min(MAX_OUTPUT);
            }
            Err(err) => return Err(anyhow::anyhow!("LCW decode failed: {err}")),
        }
    }
}

/// Parse `[Houses]` plus the per-house sections it names (`[Greece]`).
///
/// `HOUSE.CPP` keys its state by the house *name*; the index list is what
/// ties `[Houses]` to the object `House` field. `Credits` is scaled by 100
/// exactly as `HouseClass::Read_INI` does (`InitialCredits = Credits * 100`).
fn parse_houses(ini: &IniFile) -> Vec<OriginalHouse> {
    let Some(section) = section_ci(ini, "Houses") else {
        return Vec::new();
    };
    let mut entries: Vec<(u32, String)> = section
        .values
        .iter()
        .filter_map(|(key, value)| {
            let index = key.trim().parse::<u32>().ok()?;
            let name = value_text(value)?.trim().to_string();
            (!name.is_empty()).then_some((index, name))
        })
        .collect();
    entries.sort_by_key(|(index, _)| *index);
    entries
        .into_iter()
        .map(|(index, name)| {
            let details = section_ci(ini, &name);
            let int = |key: &str| {
                details
                    .and_then(|section| section.get_ci(key))
                    .and_then(|value| value.as_i64().ok())
            };
            OriginalHouse {
                index,
                name,
                credits: int("Credits")
                    .and_then(|credits| i32::try_from(credits).ok())
                    .unwrap_or(0)
                    .saturating_mul(100),
                tech_level: int("TechLevel").and_then(|value| u32::try_from(value).ok()),
                max_infantry: int("MaxInfantry").and_then(|value| u32::try_from(value).ok()),
                max_unit: int("MaxUnit").and_then(|value| u32::try_from(value).ok()),
                max_building: int("MaxBuilding").and_then(|value| u32::try_from(value).ok()),
                max_vessel: int("MaxVessel").and_then(|value| u32::try_from(value).ok()),
            }
        })
        .collect()
}

/// Parse `[Waypoints]` (waypoint number -> row-major cell index).
fn parse_waypoints(ini: &IniFile) -> Vec<Waypoint> {
    let Some(section) = section_ci(ini, "Waypoints") else {
        return Vec::new();
    };
    let mut waypoints: Vec<Waypoint> = section
        .values
        .iter()
        .filter_map(|(key, value)| {
            let index = key.trim().parse::<u32>().ok()?;
            let cell = value_text(value)?.trim().parse::<u32>().ok()?;
            Some(Waypoint { index, cell })
        })
        .collect();
    waypoints.sort_by_key(|waypoint| waypoint.index);
    waypoints
}

/// Parse `[Trigs]` and the trigger sections its entries name.
///
/// Two shapes are accepted: the modern numbered list (`1=TRIGA` plus a
/// `[TRIGA]` section with `Name`/`Event`/`Data`/`Action`) and the classic
/// single-line `TRIGA=Event,Action,Data,House,Team,Persistant` record
/// (`TRIGTYPE.CPP` `Fill_In`). Unknown events/actions are preserved verbatim.
fn parse_triggers(ini: &IniFile) -> Vec<OriginalTrigger> {
    let Some(section) = section_ci(ini, "Trigs") else {
        return Vec::new();
    };
    let mut triggers: Vec<OriginalTrigger> = Vec::new();
    for (key, value) in &section.values {
        let record = value_text(value).unwrap_or_default();
        let record = record.trim();
        if record.is_empty() {
            continue;
        }
        let numeric_key = key.trim().parse::<u32>().ok();
        let id = numeric_key.unwrap_or(triggers.len() as u32 + 1);
        if record.contains(',') {
            // `<event>,<action>,<data>,<house>,<team>,<persistant>`.
            let fields: Vec<&str> = record.split(',').map(str::trim).collect();
            let name = key.trim().to_string();
            triggers.push(OriginalTrigger {
                id,
                display_name: name.clone(),
                name,
                event: fields.first().copied().unwrap_or_default().to_string(),
                action: fields.get(1).copied().unwrap_or_default().to_string(),
                data: fields
                    .get(2)
                    .and_then(|data| data.parse::<i64>().ok())
                    .unwrap_or(0),
            });
            continue;
        }
        let name = record.to_string();
        let details = section_ci(ini, &name);
        let field = |key: &str| {
            details
                .and_then(|section| section.get_ci(key))
                .and_then(value_text)
        };
        triggers.push(OriginalTrigger {
            id,
            display_name: field("Name").unwrap_or_else(|| name.clone()),
            name,
            event: field("Event").unwrap_or_default(),
            action: field("Action").unwrap_or_default(),
            data: field("Data")
                .and_then(|data| data.trim().parse::<i64>().ok())
                .unwrap_or(0),
        });
    }
    triggers.sort_by_key(|trigger| trigger.id);
    triggers
}

/// Parse `[Base]` (`BASE.CPP`): `Player=<house>`, `Count=<n>`, `000=Type,Cell`.
fn parse_base(ini: &IniFile) -> (Option<String>, Vec<BaseNode>) {
    let Some(section) = section_ci(ini, "Base") else {
        return (None, Vec::new());
    };
    let player = section
        .get_ci("Player")
        .and_then(value_text)
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty());
    let declared = section
        .get_ci("Count")
        .and_then(|value| value.as_i64().ok())
        .and_then(|count| usize::try_from(count).ok())
        .unwrap_or(0);
    let mut entries: Vec<(u32, String)> = section
        .values
        .iter()
        .filter_map(|(key, value)| {
            let index = key.trim().parse::<u32>().ok()?;
            let record = value_text(value)?;
            (!record.trim().is_empty()).then_some((index, record))
        })
        .collect();
    entries.sort_by_key(|(index, _)| *index);
    if declared > 0 && declared < entries.len() {
        entries.truncate(declared);
    }
    let nodes = entries
        .into_iter()
        .filter_map(|(_, record)| {
            let mut fields = record.split(',').map(str::trim);
            let type_name = fields.next()?.to_string();
            let cell = fields.next()?.parse::<u32>().ok()?;
            (!type_name.is_empty()).then_some(BaseNode { type_name, cell })
        })
        .collect();
    (player, nodes)
}

/// Parse `[SMUDGE]` (cell key, value `Type[,Cell[,Data]]`).
fn parse_smudges(ini: &IniFile) -> Vec<SmudgeEntry> {
    let Some(section) = section_ci(ini, "SMUDGE") else {
        return Vec::new();
    };
    let mut smudges: Vec<SmudgeEntry> = section
        .values
        .iter()
        .filter_map(|(key, value)| {
            let record = value_text(value)?;
            let mut fields = record.split(',').map(str::trim);
            let type_name = fields.next()?.to_string();
            if type_name.is_empty() {
                return None;
            }
            let cell = fields
                .next()
                .and_then(|cell| cell.parse::<u32>().ok())
                .or_else(|| key.trim().parse::<u32>().ok())?;
            let data = fields.next().and_then(|data| data.parse::<u32>().ok());
            Some(SmudgeEntry {
                cell,
                type_name,
                data,
            })
        })
        .collect();
    smudges.sort_by_key(|smudge| smudge.cell);
    smudges
}

/// Parse `[TERRAIN]` (cell key -> terrain object type, e.g. `TREE01`).
fn parse_terrain_objects(ini: &IniFile) -> Vec<TerrainObject> {
    let Some(section) = section_ci(ini, "TERRAIN") else {
        return Vec::new();
    };
    let mut objects: Vec<TerrainObject> = section
        .values
        .iter()
        .filter_map(|(key, value)| {
            let cell = key.trim().parse::<u32>().ok()?;
            let type_name = value_text(value)?.trim().to_string();
            (!type_name.is_empty()).then_some(TerrainObject { cell, type_name })
        })
        .collect();
    objects.sort_by_key(|object| object.cell);
    objects
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
    /// Encode packed cells the way `MapClass::Write_Binary` does: 3-byte
    /// records, LCW-compressed, then base64 into numbered `1=`, `2=`, ... lines.
    fn encode_block(cells: &[PackedCell]) -> Vec<(String, String)> {
        let mut plain = Vec::with_capacity(cells.len() * ra_pack::PACK_ENTRY_BYTES);
        for cell in cells {
            plain.extend_from_slice(&cell.index.to_le_bytes());
            plain.push(cell.id);
        }
        let packed = format80::compress(&plain, plain.len()).expect("LCW compress");
        base64::uu_lines(&packed)
            .into_iter()
            .enumerate()
            .map(|(index, line)| ((index + 1).to_string(), line))
            .collect()
    }

    fn scenario_text(map: &[PackedCell], overlay: &[PackedCell]) -> String {
        let mut text = String::from("[Basic]\nName=RA 01\nPlayer=Greece\nBriefing=Hold.\n");
        for (section, cells) in [("MapPack", map), ("OverlayPack", overlay)] {
            text.push_str(&format!("[{section}]\n"));
            for (key, value) in encode_block(cells) {
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
        let text = "[Basic]\nName=X\n";
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

    const SECTIONS: &str = "[Basic]\nName=Sections\nPlayer=Greece\n\
        [Houses]\n0=Greece\n1=USSR\n\
        [Greece]\nCredits=40\nTechLevel=7\nMaxInfantry=60\nMaxVessel=3\n\
        [USSR]\nCredits=20\n\
        [Waypoints]\n0=17\n1=240\n2=100\n\
        [Trigs]\n1=TRIGA\n2=TRIGB\n\
        [TRIGA]\nName=Alpha\nEvent=Time\nData=300\nAction=Win\n\
        [TRIGB]\nName=Beta\nEvent=All Destr.\nData=0\nAction=Lose\n\
        [Base]\nPlayer=Greece\nCount=2\n000=POWR,16\n001=BARR,17\n\
        [SMUDGE]\n16=CRATER1,16,3\n\
        [TERRAIN]\n20=TREE01\n21=TREE02\n\
        [AIRCRAFT]\n1=USSR,MIG,256,120,64,Hunt\n\
        [SHIPS]\n1=Greece,DD,256,121,32,Guard,SHIPTRIG\n";

    #[test]
    fn houses_waypoints_triggers_base_smudge_and_terrain_parse() {
        let scenario = OriginalScenario::from_ini(SECTIONS).expect("valid scenario");

        assert_eq!(scenario.houses.len(), 2);
        assert_eq!(scenario.houses[0].name, "Greece");
        // HOUSE.CPP scales Credits by 100.
        assert_eq!(scenario.houses[0].credits, 4000);
        assert_eq!(scenario.houses[0].tech_level, Some(7));
        assert_eq!(scenario.houses[0].max_infantry, Some(60));
        assert_eq!(scenario.houses[0].max_vessel, Some(3));
        assert_eq!(scenario.houses[1].name, "USSR");
        assert_eq!(scenario.houses[1].credits, 2000);

        assert_eq!(
            scenario.waypoints,
            vec![
                Waypoint { index: 0, cell: 17 },
                Waypoint {
                    index: 1,
                    cell: 240
                },
                Waypoint {
                    index: 2,
                    cell: 100
                },
            ]
        );

        assert_eq!(scenario.triggers.len(), 2);
        assert_eq!(scenario.triggers[0].name, "TRIGA");
        assert_eq!(scenario.triggers[0].display_name, "Alpha");
        assert_eq!(scenario.triggers[0].event, "Time");
        assert_eq!(scenario.triggers[0].data, 300);
        assert_eq!(scenario.triggers[0].action, "Win");
        assert_eq!(scenario.triggers[1].name, "TRIGB");

        assert_eq!(scenario.base_player.as_deref(), Some("Greece"));
        assert_eq!(
            scenario.base_nodes,
            vec![
                BaseNode {
                    type_name: "POWR".to_string(),
                    cell: 16
                },
                BaseNode {
                    type_name: "BARR".to_string(),
                    cell: 17
                },
            ]
        );

        assert_eq!(
            scenario.smudges,
            vec![SmudgeEntry {
                cell: 16,
                type_name: "CRATER1".to_string(),
                data: Some(3)
            }]
        );
        assert_eq!(scenario.terrain_objects.len(), 2);
        assert_eq!(scenario.terrain_objects[1].type_name, "TREE02");

        assert_eq!(scenario.objects_of(ObjectKind::Aircraft).count(), 1);
        assert_eq!(scenario.objects_of(ObjectKind::Ship).count(), 1);
        let mig = scenario.objects_of(ObjectKind::Aircraft).next().unwrap();
        assert_eq!(mig.type_name, "MIG");
        assert_eq!(mig.mission.as_deref(), Some("Hunt"));
        assert_eq!(mig.trigger, None, "aircraft carry no trigger field");
        let destroyer = scenario.objects_of(ObjectKind::Ship).next().unwrap();
        assert_eq!(destroyer.trigger.as_deref(), Some("SHIPTRIG"));
    }

    #[test]
    fn classic_single_line_trigger_record_parses() {
        let text = "[Basic]\nName=Classic\n\
            [Trigs]\nTRIGX=Time,Win,600,Greece,,0\n";
        let scenario = OriginalScenario::from_ini(text).expect("valid scenario");
        assert_eq!(scenario.triggers.len(), 1);
        assert_eq!(scenario.triggers[0].name, "TRIGX");
        assert_eq!(scenario.triggers[0].event, "Time");
        assert_eq!(scenario.triggers[0].action, "Win");
        assert_eq!(scenario.triggers[0].data, 600);
    }

    #[test]
    fn bad_mappack_is_an_error_naming_the_section() {
        // Valid base64, but the bytes are not an LCW stream.
        let text = "[Basic]\nName=Bad\n[MapPack]\n1=AAECAwQFBgc=\n";
        let error = OriginalScenario::from_ini(text).expect_err("must not decode");
        let message = format!("{error:#}");
        assert!(message.contains("MapPack"), "section missing: {message}");
        assert!(!message.contains("panic"), "{message}");
    }

    #[test]
    fn bad_base64_is_an_error_naming_the_section() {
        let text = "[Basic]\nName=Bad\n[OverlayPack]\n1=!!!!\n";
        let error = OriginalScenario::from_ini(text).expect_err("must not decode");
        assert!(format!("{error:#}").contains("OverlayPack"));
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
