//! Original Tiberian Dawn scenario loader (INI + BIN).
//!
//! Reads the Westwood-era campaign files that shipped with C&C1:
//!
//! - `<name>.ini` - `[Basic]`, `[Wunits]`, `[Structures]`, `[Trigs]`, `[Waypoints]`
//! - `<name>.bin` - the map cell image, one 2-byte record per cell
//!   (`TemplateType`, `TIcon`).
//!
//! References: `TIBERIANDAWN/SCENARIO.CPP` (Read_Scenario_Ini),
//! `TIBERIANDAWN/MAP.CPP` (MapClass::Read_Binary).
//!
//! The loader is deliberately data-only: it produces a [`ParsedScenario`] plus
//! an optional [`ScenarioMap`]. Turning that into a live `Game` is the caller's
//! job, because object ownership rules differ per mission type.

use anyhow::{anyhow, Context, Result};
use std::path::{Path, PathBuf};

use cnc_core::trigger_attach::TriggerMode;
use cnc_core::{ClassId, Fixed16_16, IniFile, Vec2, Vec2i};

use crate::building::Building;
use crate::game::Game;
use crate::house::House;
use crate::map::TILE_SIZE;
use crate::mission::{Action, Comparison, Condition, Trigger};
use crate::td_rules::{structure_from_ini_name, UnitType};
use crate::unit::Unit;

/// Two bytes per map cell in a TD `.bin` image: template type then icon.
pub const BIN_CELL_BYTES: usize = 2;

/// One cell decoded from the `.bin` image.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BinCell {
    /// `TemplateType` - which terrain template occupies the cell.
    pub template: u8,
    /// `TIcon` - icon index within the template.
    pub icon: u8,
}

/// Decoded `.bin` terrain image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioMap {
    pub width: u32,
    pub height: u32,
    /// Row-major, `width * height` entries.
    pub cells: Vec<BinCell>,
}

impl ScenarioMap {
    /// Cell at `(x, y)`, or `None` when outside the map.
    pub fn get(&self, x: u32, y: u32) -> Option<BinCell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.cells.get((y * self.width + x) as usize).copied()
    }
}

/// A unit placed by `[Wunits]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedUnit {
    pub house: u8,
    /// Original INI type name ("E1", "MTNK", ...).
    pub unit_type: String,
    pub x: u16,
    pub y: u16,
    pub facing: u8,
    pub health: u8,
}

/// A structure placed by `[Structures]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedStructure {
    pub house: u8,
    /// Original INI type name ("FACT", "PYLE", ...).
    pub structure_type: String,
    pub x: u16,
    pub y: u16,
    pub facing: u8,
    pub health: u8,
}

/// A trigger declared under `[Trigs]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedTrigger {
    pub id: u32,
    pub name: String,
    pub enabled: bool,
    pub repeatable: bool,
    /// Raw `ConditionN=` values, verbatim.
    pub conditions: Vec<String>,
    /// Raw `ActionN=` values, verbatim.
    pub actions: Vec<String>,
}

// ---------------------------------------------------------------------------
// Trigger records -> executable triggers
// ---------------------------------------------------------------------------

/// Original TD event names, indexed by `EventType` (TRIGGER.CPP `EventText`).
pub const EVENT_NAMES: [&str; 17] = [
    "None",
    "Player Enters",
    "Discovered",
    "Attacked",
    "Destroyed",
    "Any",
    "House Discov.",
    "Units Destr.",
    "Bldgs Destr.",
    "All Destr.",
    "Credits",
    "Time",
    "# Bldgs Dstr.",
    "# Units Dstr.",
    "No Factories",
    "Civ. Evac.",
    "Built It",
];

/// Original TD action names, indexed by `ActionType` (TRIGGER.CPP `ActionText`).
pub const ACTION_NAMES: [&str; 18] = [
    "None",
    "Win",
    "Lose",
    "Production",
    "Create Team",
    "Dstry Teams",
    "All to Hunt",
    "Reinforce.",
    "DZ at 'Z'",
    "Airstrike",
    "Nuclear Missile",
    "Ion Cannon",
    "Dstry Trig 'XXXX'",
    "Dstry Trig 'YYYY'",
    "Dstry Trig 'ZZZZ'",
    "Autocreate",
    "Cap=Win/Des=Lose",
    "Allow Win",
];

/// One decoded original trigger record.
///
/// Encoded in the INI as
/// `Triggername = Eventname, Actionname, Data, Housename, TeamName, IsPersistant`
/// (TRIGGER.CPP `TriggerClass::Write_INI`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerRecord {
    pub name: String,
    /// Index into [`EVENT_NAMES`]; `None` when the name is not recognised.
    pub event: Option<usize>,
    /// Index into [`ACTION_NAMES`]; `None` when the name is not recognised.
    pub action: Option<usize>,
    /// The `Data` token (threshold, team id, trigger id, ...).
    pub data: i64,
    pub house: String,
    pub team: String,
    /// `IsPersistant`: 0 = volatile (fire once), >0 = persistent.
    pub persistent: i64,
    /// Map width used to decode the `Data` cell index row-major.
    pub width: u32,
}

impl TriggerRecord {
    /// One-shot/persistent semantics in the shared attachment model.
    ///
    /// The original field is `IsPersistant` (`TRIGGER.CPP`
    /// `TriggerClass::Write_INI`): `0` is volatile (fires once), anything
    /// greater keeps firing. Kept as a conversion so the loader and the
    /// engine agree on one definition of the flag.
    pub fn attachment_mode(&self) -> TriggerMode {
        if self.persistent <= 0 {
            TriggerMode::OneShot
        } else {
            TriggerMode::Persistent
        }
    }
}

/// Decode one original trigger record.
pub fn parse_trigger_record(trigger_name: &str, record: &str) -> TriggerRecord {
    let fields: Vec<&str> = record.split(',').map(str::trim).collect();
    let field = |index: usize| fields.get(index).copied().unwrap_or("");

    let event_name = field(0);
    let action_name = field(1);
    let event = EVENT_NAMES
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(event_name));
    let action = ACTION_NAMES
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(action_name));

    TriggerRecord {
        name: trigger_name.to_string(),
        event,
        action,
        data: field(2).parse::<i64>().unwrap_or(0),
        house: field(3).to_string(),
        team: field(4).to_string(),
        persistent: field(5).parse::<i64>().unwrap_or(0),
        width: MAP_FALLBACK_SIDE,
    }
}

/// Convert decoded trigger records into executable [`Trigger`]s.
///
/// Events map onto the engine's [`Condition`] vocabulary; actions onto
/// [`Action`]. Vocabulary the engine cannot express yet is returned in
/// `unmapped` rather than being invented, so callers can see what is missing.
pub fn trigger_records_to_triggers(records: &[TriggerRecord]) -> (Vec<Trigger>, Vec<String>) {
    let mut triggers = Vec::new();
    let mut unmapped = Vec::new();

    for record in records {
        // The record's house token is an index; named houses fall back to
        // the scenario's player house.
        let actor: u8 = record.house.trim().parse::<u8>().unwrap_or_else(|_| {
            match record.house.to_ascii_lowercase().as_str() {
                "nod" | "brotherhood" => 1,
                _ => 0,
            }
        });
        let condition = match record.event {
            // EVENT_TIME (11): mission clock threshold in `Data`.
            Some(11) => Condition::Elapsed {
                seconds: record.data.clamp(0, i64::from(u32::MAX)) as u32,
            },
            // EVENT_CREDITS (10): player reaches this many credits.
            Some(10) => Condition::Credits {
                owner: 0,
                comparison: Comparison::AtLeast(record.data),
            },
            // EVENT_NONE (0): fires immediately.
            Some(0) => Condition::Always,
            // EVENT_UNITS_DESTROYED (7) / BUILDINGS_DESTROYED (8) /
            // ALL_DESTROYED (9): the Data house has lost everything.
            Some(7) => Condition::HouseEliminated {
                owner: actor,
                what: crate::mission::Eliminated::Units,
            },
            Some(8) => Condition::HouseEliminated {
                owner: actor,
                what: crate::mission::Eliminated::Buildings,
            },
            Some(9) => Condition::HouseEliminated {
                owner: actor,
                what: crate::mission::Eliminated::Both,
            },
            // EVENT_NBUILDINGS_DESTROYED (12) / NUNITS_DESTROYED (13).
            Some(12) => Condition::KillsAtLeast {
                owner: actor,
                what: crate::mission::Eliminated::Buildings,
                count: record.data,
            },
            Some(13) => Condition::KillsAtLeast {
                owner: actor,
                what: crate::mission::Eliminated::Units,
                count: record.data,
            },
            // EVENT_NOFACTORIES (14): house has no production structure.
            Some(14) => Condition::NoFactories { owner: actor },
            // EVENT_BUILD (16): Data names the structure that must exist.
            Some(16) => match crate::td_rules::structure_from_ini_name(&record.team) {
                Some(structure) => Condition::BuiltStructure {
                    owner: actor,
                    structure,
                },
                None => Condition::Never,
            },
            // EVENT_DISCOVERED (2): the player explored a new cell.
            // Data selects a cell when non-zero, otherwise any discovery.
            Some(2) => Condition::Discovered {
                cell: if record.data > 0 {
                    Some(Vec2i::new(
                        (record.data % i64::from(record.width.max(1))) as i32,
                        (record.data / i64::from(record.width.max(1))) as i32,
                    ))
                } else {
                    None
                },
            },
            Some(index) => {
                unmapped.push(format!("event: {}", EVENT_NAMES[index]));
                Condition::Never
            }
            None => {
                unmapped.push(format!("event: unknown ({})", record.name));
                Condition::Never
            }
        };

        // The action's target cell. ACTION_DZ and the strike actions carry a
        // cell number in `Data`; it is decoded row-major against the map.
        let action_cell = Vec2i::new(
            (record.data.max(0) % i64::from(record.width.max(1))) as i32,
            (record.data.max(0) / i64::from(record.width.max(1))) as i32,
        );
        // The record's house token is an index; named houses fall back to the
        // scenario's player house.

        let action = match record.action {
            // ACTION_WIN (1) / ACTION_LOSE (2).
            Some(1) => Action::Win,
            Some(2) => Action::Lose,
            // ACTION_AIRSTRIKE (9): strafe the target cell.
            Some(9) => Action::Airstrike {
                owner: actor,
                cell: action_cell,
            },
            // ACTION_NUKE (10).
            Some(10) => Action::Nuke {
                owner: actor,
                cell: action_cell,
            },
            // ACTION_ION (11).
            Some(11) => Action::IonCannon {
                owner: actor,
                cell: action_cell,
            },
            // ACTION_DZ (8): drop reinforcements.
            Some(8) => Action::Dropzone {
                owner: actor,
                cell: action_cell,
            },
            // ACTION_ALL_HUNT (6).
            Some(6) => Action::AllToHunt { owner: actor },
            // ACTION_CREATE_TEAM (4).
            Some(4) => Action::CreateTeam {
                owner: actor,
                team: record.name.clone(),
            },
            // ACTION_ALLOW_WIN etc. are gating flags the engine lacks.
            Some(index) => {
                unmapped.push(format!("action: {}", ACTION_NAMES[index]));
                Action::Message(format!("[unmapped action] {}", ACTION_NAMES[index]))
            }
            None => {
                unmapped.push(format!("action: unknown ({})", record.name));
                Action::Message(format!("[unknown action] {}", record.name))
            }
        };

        triggers.push(Trigger {
            id: triggers.len() as u32,
            name: record.name.clone(),
            condition,
            actions: vec![action],
            // `IsPersistant` > 0 means repeatable; the shared attachment
            // model owns the flag's meaning (`TriggerRecord::attachment_mode`).
            once: record.attachment_mode() == TriggerMode::OneShot,
            enabled: true,
            fired: false,
        });
    }

    (triggers, unmapped)
}

/// Everything recovered from a scenario's `.ini`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedScenario {
    pub name: String,
    /// `[Basic] Player` - house index the human drives.
    pub player: u8,
    pub credits: i32,
    pub tech_level: u8,
    /// `[Houses]` entries in the original dialect (`HQ.CPP`/`HOUSE.CPP`
    /// `Read_INI`): one per house with its own credits and limits.
    pub houses: Vec<HouseRecord>,
    /// `[Basic] Theater` (defaults to `Temperate`).
    pub theater: String,
    /// `[Basic] Briefing` plus any `BriefingN` continuation lines.
    pub briefing: Vec<String>,
    /// `[Objectives]` values in declaration order.
    pub objectives: Vec<String>,
    pub units: Vec<PlacedUnit>,
    pub structures: Vec<PlacedStructure>,
    pub triggers: Vec<ParsedTrigger>,
    /// `[Waypoints]` ids that were present.
    pub waypoint_ids: Vec<u32>,
    /// `[Waypoints]` id -> cell mappings, in id order.
    pub waypoints: Vec<Waypoint>,
    /// Original-format trigger records decoded from the TRIGGER table.
    pub trigger_records: Vec<TriggerRecord>,
    /// `[TeamTypes]` parsed via `cnc_core::entity` (empty when absent).
    pub team_types: Vec<cnc_core::entity::TeamType>,
    /// Recoverable problems found while parsing (missing sub-sections,
    /// non-numeric cells, unknown trigger vocabulary, ...). The loader never
    /// fails on a malformed optional section; it reports it here instead so
    /// callers can log the damage and keep playing what did parse.
    pub warnings: Vec<String>,
}

/// `true` when `text` looks like an original-format TD scenario.
///
/// The original keys campaign data off `[Basic]` plus `[Wunits]`; the Rust
/// dialect instead uses `[ScriptSetup]`/`[TriggerN]` sections.
pub fn looks_like_original_scenario(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("[basic]") && lower.contains("[wunits]")
}

/// Load `<stem>.ini` and, when present, the sibling `<stem>.bin`.
pub fn load_scenario(ini_path: impl AsRef<Path>) -> Result<(ParsedScenario, Option<ScenarioMap>)> {
    let ini_path = ini_path.as_ref();
    let text = std::fs::read_to_string(ini_path)
        .with_context(|| format!("reading scenario INI {}", ini_path.display()))?;
    let ini = IniFile::parse(&text)
        .with_context(|| format!("parsing scenario INI {}", ini_path.display()))?;
    let scenario = parse_scenario(&ini)?;
    let bin = bin_path_for(ini_path);
    let map = if bin.exists() {
        Some(load_bin(&bin, None)?)
    } else {
        None
    };
    Ok((scenario, map))
}

/// Resolve the sibling `.bin` path for an INI scenario path.
pub fn bin_path_for(ini_path: impl AsRef<Path>) -> PathBuf {
    ini_path.as_ref().with_extension("bin")
}

/// Parse an already-loaded `IniFile` into a [`ParsedScenario`].
pub fn parse_scenario(ini: &IniFile) -> Result<ParsedScenario> {
    let basic = ini
        .section("Basic")
        .context("scenario has no [Basic] section")?;

    let name = basic
        .get_string("Name")
        .unwrap_or("Unnamed Scenario")
        .to_string();
    let player = basic.get_i64("Player").unwrap_or(0).clamp(0, 255) as u8;
    let credits = basic.get_i64("Credits").unwrap_or(0) as i32;
    let tech_level = basic.get_i64("TechLevel").unwrap_or(0).clamp(0, 255) as u8;
    let theater = basic
        .get_string("Theater")
        .unwrap_or("Temperate")
        .to_string();

    let mut briefing = Vec::new();
    if let Some(first) = basic.get_string("Briefing") {
        if !first.is_empty() {
            briefing.push(first.to_string());
        }
    }
    for index in 1..=50 {
        if let Some(line) = basic.get_string(&format!("Briefing{index}")) {
            if !line.is_empty() {
                briefing.push(line.to_string());
            }
        }
    }

    let mut objectives = Vec::new();
    if let Some(section) = ini.section("Objectives") {
        let mut keyed: Vec<(i64, String)> = section
            .values
            .keys()
            .filter_map(|key| key.parse::<i64>().ok())
            .filter_map(|order| {
                let raw = section.get_string(&order.to_string())?;
                Some((order, raw.to_string()))
            })
            .collect();
        keyed.sort_by_key(|(order, _)| *order);
        objectives.extend(keyed.into_iter().map(|(_, text)| text));
    }

    Ok(ParsedScenario {
        name,
        player,
        credits,
        tech_level,
        theater,
        briefing,
        objectives,
        units: parse_units(ini),
        structures: parse_structures(ini),
        triggers: parse_triggers(ini),
        waypoint_ids: parse_waypoint_ids(ini),
        waypoints: parse_waypoints(ini),
        houses: parse_houses(ini),
        trigger_records: parse_trigger_records(ini),
        // Team types are optional: a scenario without [TeamTypes] yields an
        // empty roster and the game falls back to the built-in expert AI.
        team_types: cnc_core::entity::team_types_from_ini(ini).unwrap_or_default(),
        warnings: scenario_warnings(ini),
    })
}

fn parse_units(ini: &IniFile) -> Vec<PlacedUnit> {
    let Some(section) = ini.section("Wunits") else {
        return Vec::new();
    };
    let mut units = Vec::new();
    for index in 1..=9999 {
        let Some(object_name) = section.get_string(&format!("Unit{index}")) else {
            break;
        };
        let Some(object) = ini.section(object_name) else {
            continue;
        };
        units.push(PlacedUnit {
            house: object.get_i64("House").unwrap_or(0).clamp(0, 255) as u8,
            unit_type: object.get_string("Type").unwrap_or("").to_string(),
            x: object.get_i64("X").unwrap_or(0).clamp(0, u16::MAX as i64) as u16,
            y: object.get_i64("Y").unwrap_or(0).clamp(0, u16::MAX as i64) as u16,
            facing: object.get_i64("Facing").unwrap_or(0).clamp(0, 255) as u8,
            health: object.get_i64("Health").unwrap_or(256).clamp(0, 255) as u8,
        });
    }
    units
}

fn parse_structures(ini: &IniFile) -> Vec<PlacedStructure> {
    let Some(section) = ini.section("Structures") else {
        return Vec::new();
    };
    let mut structures = Vec::new();
    for index in 1..=9999 {
        let Some(object_name) = section.get_string(&format!("Structure{index}")) else {
            break;
        };
        let Some(object) = ini.section(object_name) else {
            continue;
        };
        structures.push(PlacedStructure {
            house: object.get_i64("House").unwrap_or(0).clamp(0, 255) as u8,
            structure_type: object.get_string("Type").unwrap_or("").to_string(),
            x: object.get_i64("X").unwrap_or(0).clamp(0, u16::MAX as i64) as u16,
            y: object.get_i64("Y").unwrap_or(0).clamp(0, u16::MAX as i64) as u16,
            facing: object.get_i64("Facing").unwrap_or(0).clamp(0, 255) as u8,
            health: object.get_i64("Health").unwrap_or(256).clamp(0, 255) as u8,
        });
    }
    structures
}

/// Read a value as text, re-joining comma-split arrays.
///
/// `IniFile` turns any value containing a comma into `IniValue::Array`
/// (see `IniFile::parse_value`), so original-format trigger/action records
/// such as `Elapsed,0,,>=,120` arrive as arrays and must be re-joined.
fn get_raw_text(section: &cnc_core::IniSection, key: &str) -> Option<String> {
    Some(scalar_text(section.get(key)?))
}

/// Render one `IniValue` as text; arrays join with commas.
fn scalar_text(value: &cnc_core::IniValue) -> String {
    use cnc_core::IniValue;
    match value {
        IniValue::String(s) => s.clone(),
        IniValue::Integer(i) => i.to_string(),
        IniValue::Float(f) => f.to_string(),
        IniValue::Boolean(b) => if *b { "yes" } else { "no" }.to_string(),
        IniValue::Array(items) => items.iter().map(scalar_text).collect::<Vec<_>>().join(","),
    }
}

fn parse_triggers(ini: &IniFile) -> Vec<ParsedTrigger> {
    let Some(section) = ini.section("Trigs") else {
        return Vec::new();
    };
    let mut triggers = Vec::new();
    for index in 1..=9999 {
        let Some(trigger_name) = section.get_string(&format!("Trigger{index}")) else {
            break;
        };
        let Some(trigger) = ini.section(trigger_name) else {
            continue;
        };
        let mut conditions = Vec::new();
        for slot in 1..=20 {
            if let Some(raw) = get_raw_text(trigger, &format!("Condition{slot}")) {
                if !raw.is_empty() {
                    conditions.push(raw);
                }
            }
        }
        let mut actions = Vec::new();
        for slot in 1..=20 {
            if let Some(raw) = get_raw_text(trigger, &format!("Action{slot}")) {
                if !raw.is_empty() {
                    actions.push(raw);
                }
            }
        }
        triggers.push(ParsedTrigger {
            id: index as u32,
            name: trigger
                .get_string("Name")
                .map(str::to_string)
                .unwrap_or_else(|| format!("Trigger{index}")),
            enabled: trigger.get_bool("Enabled").unwrap_or(true),
            repeatable: trigger.get_bool("Repeat").unwrap_or(false),
            conditions,
            actions,
        });
    }
    triggers
}

/// Read the original-format trigger table.
///
/// The original engine stores each trigger as one comma-separated record:
/// `Eventname, Actionname, Data, Housename, TeamName, IsPersistant`.
/// See [`parse_trigger_record`].
fn parse_trigger_records(ini: &IniFile) -> Vec<TriggerRecord> {
    // Strike/DZ actions carry a row-major cell index, so the width is needed
    // to decode it. Fall back to a square guess when the file omits it.
    let width = ini
        .section("Map")
        .and_then(|map| map.get_i64("Width").ok())
        .or_else(|| {
            ini.section("Basic")
                .and_then(|basic| basic.get_i64("Width").ok())
        })
        .filter(|width| *width > 0)
        .unwrap_or(MAP_FALLBACK_SIDE as i64) as u32;

    let section = ini.section("TRIGGER").or_else(|| ini.section("Basic"));
    let Some(section) = section else {
        return Vec::new();
    };
    let mut records = Vec::new();
    for (name, value) in &section.values {
        // Only records shaped like "Event, Action, ..." are triggers; the
        // [Basic] fallback also carries Name/Player/Credits and friends.
        let raw = scalar_text(value);
        if !raw.contains(',') {
            continue;
        }
        if name.to_ascii_lowercase().starts_with("briefing") {
            continue;
        }
        let mut record = parse_trigger_record(name, &raw);
        record.width = width;
        records.push(record);
    }
    records.sort_by(|a, b| a.name.cmp(&b.name));
    records
}

fn parse_waypoint_ids(ini: &IniFile) -> Vec<u32> {
    parse_waypoints(ini)
        .into_iter()
        .map(|waypoint| waypoint.id)
        .collect()
}

/// Collect recoverable problems from an original scenario INI.
///
/// Original campaign files in the wild carry truncated tables, stale
/// references and non-numeric values; `SCENARIO.CPP` tolerates all of these
/// and so does this loader. Every problem that had to be skipped is listed
/// here instead of panicking or aborting the whole file.
pub fn scenario_warnings(ini: &IniFile) -> Vec<String> {
    let mut warnings = Vec::new();
    collect_object_warnings(ini, "Wunits", "Unit", true, &mut warnings);
    collect_object_warnings(ini, "Structures", "Structure", true, &mut warnings);
    collect_object_warnings(ini, "Trigs", "Trigger", false, &mut warnings);
    collect_waypoint_warnings(ini, &mut warnings);
    collect_trigger_record_warnings(ini, &mut warnings);
    warnings
}

/// Warn about `[<table>]` entries whose object section is missing (or has no
/// `Type=` when `needs_type` is set).
fn collect_object_warnings(
    ini: &IniFile,
    table: &str,
    key_prefix: &str,
    needs_type: bool,
    warnings: &mut Vec<String>,
) {
    let Some(section) = ini.section(table) else {
        return;
    };
    for index in 1..=9999 {
        let Some(object_name) = section.get_string(&format!("{key_prefix}{index}")) else {
            break;
        };
        let Some(object) = ini.section(object_name) else {
            warnings.push(format!(
                "[{table}]: {object_name} has no matching [{object_name}] section"
            ));
            continue;
        };
        if needs_type
            && object
                .get_string("Type")
                .is_none_or(|value| value.trim().is_empty())
        {
            warnings.push(format!("[{table}]: [{object_name}] has no Type="));
        }
    }
}

/// Warn about waypoint keys/values that cannot be read (`DISPLAY.CPP` reads
/// each as an integer with `-1` meaning unset).
fn collect_waypoint_warnings(ini: &IniFile, warnings: &mut Vec<String>) {
    let Some(section) = ini.section("Waypoints") else {
        return;
    };
    for (key, value) in &section.values {
        if key.trim().parse::<u32>().is_err() {
            warnings.push(format!(
                "[Waypoints]: key '{key}' is not a numeric waypoint id"
            ));
            continue;
        }
        let raw = scalar_text(value);
        match raw.trim().parse::<i64>() {
            Ok(cell) if cell < 0 => {
                warnings.push(format!("[Waypoints]: {key}={cell} is unset"));
            }
            Ok(_) => {}
            Err(_) => {
                warnings.push(format!("[Waypoints]: {key}={raw} is not a cell number"));
            }
        }
    }
}

/// Warn about `[TRIGGER]` records whose event/action name is outside the
/// decoded vocabulary (they still parse to a default record).
fn collect_trigger_record_warnings(ini: &IniFile, warnings: &mut Vec<String>) {
    let Some(section) = ini.section("TRIGGER") else {
        return;
    };
    for (name, value) in &section.values {
        let raw = scalar_text(value);
        if !raw.contains(',') {
            continue;
        }
        let record = parse_trigger_record(name, &raw);
        if record.event.is_none() {
            let event = raw.split(',').next().unwrap_or("").trim();
            warnings.push(format!("[TRIGGER]: {name} has unknown event '{event}'"));
        }
        if record.action.is_none() {
            let action = raw.split(',').nth(1).unwrap_or("").trim();
            warnings.push(format!("[TRIGGER]: {name} has unknown action '{action}'"));
        }
    }
}

/// One `[Houses]` entry.
///
/// `HOUSE.CPP` `Read_INI` reads, per house section named by its `IniName`:
/// `Credits` (multiplied by 100), `MaxUnit`, `MaxBuilding` and `Edge`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HouseRecord {
    /// Section name, e.g. `GDI` or `Nod`.
    pub name: String,
    /// Credits scaled to the engine's internal units.
    pub credits: i32,
    pub max_units: u32,
    pub max_buildings: u32,
    /// Reinforcement arrival edge (`Edge=`); empty when unspecified.
    pub edge: String,
}

/// Read the `[Houses]` list: `0=GDI`, `1=Nod`, ... referencing sections that
/// name the house.
pub fn parse_houses(ini: &IniFile) -> Vec<HouseRecord> {
    let Some(list) = ini.section("Houses") else {
        return Vec::new();
    };
    let mut houses = Vec::new();
    for value in list.values.values() {
        let name = scalar_text(value).trim().to_string();
        if name.is_empty() {
            continue;
        }
        let section = ini.section(&name);
        let int =
            |key: &str, default: i64| section.and_then(|s| s.get_i64(key).ok()).unwrap_or(default);
        let credits = int("Credits", 0) as i32;
        houses.push(HouseRecord {
            name: name.clone(),
            // HOUSE.CPP stores Credits * 100.
            credits: credits.saturating_mul(100),
            max_units: int("MaxUnit", 150).clamp(0, u32::MAX as i64) as u32,
            max_buildings: int("MaxBuilding", 150).clamp(0, u32::MAX as i64) as u32,
            edge: section
                .and_then(|s| s.get_string("Edge"))
                .unwrap_or("")
                .to_string(),
        });
    }
    houses
}

/// One `[Waypoints]` entry: waypoint id -> map cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Waypoint {
    pub id: u32,
    /// Row-major cell index the waypoint points at.
    pub cell: u32,
}

/// Read `[Waypoints]` in the original dialect.
///
/// `DISPLAY.CPP` reads every entry as `WWGetPrivateProfileInt("Waypoints",
/// "<id>", -1)` — the key is the numeric waypoint id and the value is a cell
/// number, not a name (`-1` meaning unset). This previously looked for
/// `Waypoint1`-style string keys and so found nothing in real files.
pub fn parse_waypoints(ini: &IniFile) -> Vec<Waypoint> {
    let Some(section) = ini.section("Waypoints") else {
        return Vec::new();
    };
    let mut waypoints = Vec::new();
    for (key, value) in &section.values {
        let Ok(id) = key.trim().parse::<u32>() else {
            continue;
        };
        let Some(cell) = scalar_text(value).trim().parse::<i64>().ok() else {
            continue;
        };
        if cell < 0 {
            continue;
        }
        waypoints.push(Waypoint {
            id,
            cell: cell as u32,
        });
    }
    waypoints.sort_by_key(|waypoint| waypoint.id);
    waypoints
}

/// Decode a TD `.bin` map image from disk.
///
/// `expected_side` lets tests pin the decoded square size; production callers
/// pass `None` and the size is inferred from the payload.
pub fn load_bin(path: impl AsRef<Path>, expected_side: Option<u32>) -> Result<ScenarioMap> {
    let path = path.as_ref();
    let raw =
        std::fs::read(path).with_context(|| format!("reading map image {}", path.display()))?;
    decode_bin(&raw, expected_side)
        .with_context(|| format!("decoding map image {}", path.display()))
}

/// Decode an in-memory `.bin` payload.
///
/// Uncompressed images are exactly `side * side * 2` bytes. A payload that does
/// not fit a square grid is treated as an LCW stream and inflated.
pub fn decode_bin(raw: &[u8], expected_side: Option<u32>) -> Result<ScenarioMap> {
    if raw.is_empty() {
        return Err(anyhow!("map image is empty"));
    }

    if let Some(side) = expected_side {
        let want = (side as usize) * (side as usize) * BIN_CELL_BYTES;
        if raw.len() == want {
            return Ok(cells_to_map(raw, side, side));
        }
        let inflated = cnc_asset::format80::decompress(raw, want)
            .context("LCW decompression of map image failed")?;
        return Ok(cells_to_map(&inflated, side, side));
    }

    if let Some(side) = square_side(raw.len()) {
        return Ok(cells_to_map(raw, side, side));
    }

    let mut last_err = None;
    for side in 8..=512u32 {
        let want = (side as usize) * (side as usize) * BIN_CELL_BYTES;
        if want < raw.len() {
            continue;
        }
        match cnc_asset::format80::decompress(raw, want) {
            Ok(inflated) => return Ok(cells_to_map(&inflated, side, side)),
            Err(err) => last_err = Some(err),
        }
    }
    Err(anyhow!(
        "map image is neither a square cell grid nor decodable LCW: {last_err:?}"
    ))
}

/// Largest `side` with `side * side * 2 == byte_len`, if any.
fn square_side(byte_len: usize) -> Option<u32> {
    if !byte_len.is_multiple_of(BIN_CELL_BYTES) {
        return None;
    }
    let cells = byte_len / BIN_CELL_BYTES;
    let side = (cells as f64).sqrt() as u32;
    if side >= 2 && (side as usize) * (side as usize) == cells {
        Some(side)
    } else {
        None
    }
}

fn cells_to_map(bytes: &[u8], width: u32, height: u32) -> ScenarioMap {
    let cells = bytes
        .as_chunks::<BIN_CELL_BYTES>()
        .0
        .iter()
        .map(|pair| BinCell {
            template: pair[0],
            icon: pair[1],
        })
        .collect();
    ScenarioMap {
        width,
        height,
        cells,
    }
}
// ---------------------------------------------------------------------------
// Scenario -> Game wiring
// ---------------------------------------------------------------------------

/// Resolve an original INI unit type name ("E1", "MTNK", ...) to a [`UnitType`].
///
/// The original files use the UDATA/IDATA section names; the engine stores the
/// enum, so this is the one place that mapping happens.
pub fn unit_type_from_ini_name(name: &str) -> Option<UnitType> {
    UnitType::ALL
        .into_iter()
        .find(|candidate| candidate.ini_name().eq_ignore_ascii_case(name.trim()))
}

/// `true` when every placed unit and structure resolved to an engine type.
///
/// Callers can surface the unresolved names so a partially-supported mission
/// is visibly partial instead of silently missing objects.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConversionReport {
    pub units_placed: usize,
    pub structures_placed: usize,
    /// Executable triggers installed on `game.mission`.
    pub triggers_installed: usize,
    /// `[Waypoints]` mappings carried over for action spawn points.
    pub waypoints: Vec<Waypoint>,
    /// INI type names the engine has no `UnitType` for.
    pub unknown_units: Vec<String>,
    /// INI type names the engine has no `StructureType` for.
    pub unknown_structures: Vec<String>,
    /// Trigger conditions/actions the engine vocabulary could not express.
    pub unmapped_triggers: Vec<String>,
    /// Recoverable parse problems from the source INI (see
    /// [`scenario_warnings`]); this never aborts the conversion.
    pub warnings: Vec<String>,
}

impl ConversionReport {
    /// `true` when at least one object or trigger was skipped for lack of a
    /// mapping.
    pub fn has_gaps(&self) -> bool {
        !self.unknown_units.is_empty()
            || !self.unknown_structures.is_empty()
            || !self.unmapped_triggers.is_empty()
    }
}

/// Build a playable [`Game`] from a parsed scenario and its optional map image.
///
/// Placed objects are converted where the engine has a matching type; anything
/// unmapped is reported in the returned [`ConversionReport`] rather than
/// silently dropped.
pub fn scenario_to_game(
    scenario: &ParsedScenario,
    map: Option<&ScenarioMap>,
) -> (Game, ConversionReport) {
    let map = map.cloned();
    let (width, height) = match &map {
        Some(image) => (image.width, image.height),
        // Without a .bin the original scenario declares no dimensions, so fall
        // back to the smallest viable board rather than an empty map.
        None => (MAP_FALLBACK_SIDE, MAP_FALLBACK_SIDE),
    };

    let mut game = Game::new(&scenario.name, width, height);
    game.player_owner = scenario.player;
    // Hand the scenario's [TeamTypes] to the runtime consumer; an empty
    // roster leaves the built-in expert AI entirely in charge.
    if !scenario.team_types.is_empty() {
        game.team_manager = crate::ai::TeamManager::new(scenario.team_types.clone());
    }

    // Houses: the engine tracks them by index; create as many as the scenario
    // places objects for, at least two (player + one opponent).
    let house_count = scenario
        .units
        .iter()
        .map(|unit| unit.house)
        .chain(scenario.structures.iter().map(|structure| structure.house))
        .max()
        .map_or(2, |highest| usize::from(highest) + 1)
        .max(2);
    for index in 0..house_count {
        let id = if index == 0 {
            ClassId::HouseGDI
        } else {
            ClassId::HouseNod
        };
        let name = if index == 0 { "GDI" } else { "Nod" };
        let mut house = House::new(id, name.to_string());
        // [Houses] wins when present: HOUSE.CPP reads per-house Credits and
        // limits from the house's own section. [Basic] Credits remains the
        // fallback for files that carry no [Houses] table.
        if let Some(record) = scenario.houses.get(index).or_else(|| {
            scenario
                .houses
                .iter()
                .find(|h| h.name.eq_ignore_ascii_case(name))
        }) {
            house.credits = record.credits;
            house.max_units = record.max_units;
            house.max_buildings = record.max_buildings;
            house.edge =
                crate::game::ReinforcementEdge::from_name(&record.edge).unwrap_or_default();
        } else if index == usize::from(scenario.player) {
            house.credits = scenario.credits;
        }
        if scenario.tech_level > 0 {
            house.tech_level = scenario.tech_level;
        }
        game.houses.push(house);
    }

    let mut report = ConversionReport::default();

    for placed in &scenario.units {
        let Some(unit_type) = unit_type_from_ini_name(&placed.unit_type) else {
            report.unknown_units.push(placed.unit_type.clone());
            continue;
        };
        let position = cell_center(placed.x, placed.y);
        let mut unit = Unit::new(unit_type, position, placed.house);
        apply_health_fraction(&mut unit.health, &unit.max_health, placed.health);
        game.units.push(unit);
        report.units_placed += 1;
    }

    for placed in &scenario.structures {
        let Some(structure) = structure_from_ini_name(&placed.structure_type) else {
            report
                .unknown_structures
                .push(placed.structure_type.clone());
            continue;
        };
        // StructureStats carries `strength` (BDATA.CPP) rather than a
        // `health()` helper; both are the same integer strength value.
        let max_health = Fixed16_16::from_int(structure.stats().strength);
        let position = cell_center(placed.x, placed.y);
        let mut building = Building::new(structure.class_id(), position, max_health, placed.house);
        // Defences share ClassIds (OBLI/ATWR collapse onto TeslaCoil/AA_Gun),
        // so `Building::new` cannot recover the authored type on its own.
        building.structure = structure;
        apply_health_fraction(&mut building.health, &max_health, placed.health);
        game.buildings.push(building);
        report.structures_placed += 1;
    }

    if let Some(image) = map {
        apply_bin_terrain(&mut game, &image);
    }

    // Executable triggers come from the original `[TRIGGER]` table. The
    // name-keyed `[Trigs]` sections only carry human-readable condition text
    // ("Player Enters", "Time", ...), which the engine cannot execute, so
    // records are the source of truth. Records the vocabulary cannot express
    // are reported, never guessed.
    let (triggers, unmapped) = trigger_records_to_triggers(&scenario.trigger_records);
    report.triggers_installed = triggers.len();
    report.unmapped_triggers = unmapped;
    report.waypoints = scenario.waypoints.clone();
    report.warnings = scenario.warnings.clone();

    let mut mission = crate::mission::Mission::new(scenario.name.clone());
    mission.briefing = scenario.briefing.join("\n");
    mission.player = scenario.player;
    mission.triggers = triggers;
    for (index, text) in scenario.objectives.iter().enumerate() {
        mission.add_objective(index as u32 + 1, text.clone());
    }
    game.start_mission(mission);

    (game, report)
}

/// Side length used when no `.bin` image accompanies the scenario.
pub const MAP_FALLBACK_SIDE: u32 = 64;

/// Original files store health as a 0..=255 fraction of the type maximum.
fn apply_health_fraction(health: &mut Fixed16_16, max_health: &Fixed16_16, fraction: u8) {
    if fraction == 0 {
        return;
    }
    *health = Fixed16_16::from_f32(max_health.to_f32() * (f32::from(fraction) / 256.0));
}

/// Centre pixel of a map cell, matching the engine's own cell placement.
fn cell_center(x: u16, y: u16) -> Vec2 {
    Vec2::new(
        Fixed16_16::from_f32(f32::from(x) * TILE_SIZE + TILE_SIZE * 0.5),
        Fixed16_16::from_f32(f32::from(y) * TILE_SIZE + TILE_SIZE * 0.5),
    )
}

/// Copy `.bin` template/icon bytes into the playable cell grid.
fn apply_bin_terrain(game: &mut Game, image: &ScenarioMap) {
    for y in 0..image.height.min(game.map.height) {
        for x in 0..image.width.min(game.map.width) {
            let Some(cell) = image.get(x, y) else {
                continue;
            };
            if let Some(grid_cell) = game.map.grid.get_mut(Vec2i::new(x as i32, y as i32)) {
                grid_cell.terrain_type = cell.template;
                grid_cell.overlay_type = cell.icon;
            }
        }
    }
}

#[cfg(test)]
mod action_mapping_tests {
    use super::*;

    fn map_action(record: &str) -> Option<Action> {
        let mut rec = parse_trigger_record("T1", record);
        rec.width = 64;
        let (triggers, _) = trigger_records_to_triggers(&[rec]);
        triggers.first()?.actions.first().cloned()
    }

    #[test]
    fn airstrike_maps_to_the_strike_action() {
        assert_eq!(
            map_action("0, Airstrike, 100, 0, , 0"),
            Some(Action::Airstrike {
                owner: 0,
                cell: Vec2i::new(36, 1)
            })
        );
    }

    #[test]
    fn nuclear_missile_maps_to_nuke() {
        assert!(matches!(
            map_action("0, Nuclear Missile, 0, 0, , 0"),
            Some(Action::Nuke { .. })
        ));
    }

    #[test]
    fn ion_cannon_maps_to_ion() {
        assert!(matches!(
            map_action("0, Ion Cannon, 0, 0, , 0"),
            Some(Action::IonCannon { .. })
        ));
    }

    #[test]
    fn dropzone_maps_to_dz() {
        assert!(matches!(
            map_action("0, DZ at 'Z', 5, 0, , 0"),
            Some(Action::Dropzone { .. })
        ));
    }

    #[test]
    fn all_to_hunt_maps_to_hunt() {
        assert_eq!(
            map_action("0, All to Hunt, 0, 1, , 0"),
            Some(Action::AllToHunt { owner: 1 })
        );
    }

    #[test]
    fn cell_index_decodes_row_major() {
        // Data 130 on a 64-wide map is row 2, column 2.
        assert_eq!(
            map_action("0, Airstrike, 130, 0, , 0"),
            Some(Action::Airstrike {
                owner: 0,
                cell: Vec2i::new(2, 2)
            })
        );
    }

    #[test]
    fn airstrike_action_is_no_longer_reported_unmapped() {
        let mut rec = parse_trigger_record("T1", "0, Airstrike, 0, 0, , 0");
        rec.width = 64;
        let (_, unmapped) = trigger_records_to_triggers(&[rec]);
        // "T1" is not a valid event name, so an event entry is expected; what
        // must not appear is an unmapped *action*.
        assert!(
            !unmapped.iter().any(|entry| entry.starts_with("action:")),
            "airstrike should be mapped: {unmapped:?}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIO: &str = r#"
[Basic]
Name=GDI 01 - X16
Player=0
Credits=8000
TechLevel=7
Theater=Temperate
Briefing=Nod armour is massing east of your base.
Briefing1=Hold the line.

[Objectives]
1=Survive the assault
2=Keep the yard alive

[Wunits]
Unit1=U1
Unit2=U2

[U1]
House=0
Type=E1
X=10
Y=12
Facing=64
Health=200

[U2]
House=1
Type=BGGY
X=20
Y=9
Facing=0
Health=256

[Structures]
Structure1=S1

[S1]
House=0
Type=FACT
X=15
Y=15
Facing=0
Health=256

[Trigs]
Trigger1=T1

[T1]
Name=Held
Enabled=yes
Repeat=no
Condition1=Elapsed,0,,>=,120
Action1=Win

[Waypoints]
1=260
"#;

    fn parsed() -> ParsedScenario {
        parse_scenario(&IniFile::parse(SCENARIO).unwrap()).unwrap()
    }

    #[test]
    fn basic_section_is_read() {
        let s = parsed();
        assert_eq!(s.name, "GDI 01 - X16");
        assert_eq!(s.player, 0);
        assert_eq!(s.credits, 8000);
        assert_eq!(s.tech_level, 7);
        assert_eq!(s.theater, "Temperate");
    }

    #[test]
    fn briefing_collects_both_forms() {
        assert_eq!(
            parsed().briefing,
            vec![
                "Nod armour is massing east of your base.".to_string(),
                "Hold the line.".to_string()
            ]
        );
    }

    #[test]
    fn objectives_preserve_declaration_order() {
        assert_eq!(
            parsed().objectives,
            vec![
                "Survive the assault".to_string(),
                "Keep the yard alive".to_string()
            ]
        );
    }

    #[test]
    fn units_and_structures_are_placed() {
        let s = parsed();
        assert_eq!(s.units.len(), 2);
        assert_eq!(s.units[0].unit_type, "E1");
        assert_eq!((s.units[0].x, s.units[0].y), (10, 12));
        assert_eq!(s.units[1].unit_type, "BGGY");
        assert_eq!(s.units[1].house, 1);

        assert_eq!(s.structures.len(), 1);
        assert_eq!(s.structures[0].structure_type, "FACT");
        assert_eq!(s.structures[0].x, 15);
    }

    #[test]
    fn triggers_carry_raw_conditions_and_actions() {
        let s = parsed();
        assert_eq!(s.triggers.len(), 1);
        let trigger = &s.triggers[0];
        assert_eq!(trigger.name, "Held");
        assert!(trigger.enabled);
        assert!(!trigger.repeatable);
        assert_eq!(trigger.conditions, vec!["Elapsed,0,,>=,120".to_string()]);
        assert_eq!(trigger.actions, vec!["Win".to_string()]);
    }

    #[test]
    fn waypoints_use_numeric_keys_and_cell_values() {
        // DISPLAY.CPP: WWGetPrivateProfileInt("Waypoints", "<id>", -1).
        let ini = IniFile::parse("[Waypoints]\n0=132\n1=200\n2=-1\n").expect("ini");
        let waypoints = parse_waypoints(&ini);
        assert_eq!(
            waypoints,
            vec![Waypoint { id: 0, cell: 132 }, Waypoint { id: 1, cell: 200 },],
            "id 2 is -1 (unset) and must be skipped"
        );
    }

    #[test]
    fn missing_waypoint_section_yields_nothing() {
        let ini = IniFile::parse("[Basic]\nName=X\n").expect("ini");
        assert!(parse_waypoints(&ini).is_empty());
    }

    #[test]
    fn houses_scale_credits_and_read_limits() {
        let ini = IniFile::parse(
            "[Houses]\n0=GDI\n1=Nod\n\
             [GDI]\nCredits=50\nMaxUnit=120\nMaxBuilding=90\nEdge=North\n\
             [Nod]\nCredits=25\n",
        )
        .expect("ini");
        let houses = parse_houses(&ini);
        assert_eq!(houses.len(), 2);
        assert_eq!(houses[0].name, "GDI");
        assert_eq!(houses[0].credits, 5000, "HOUSE.CPP multiplies by 100");
        assert_eq!(houses[0].max_units, 120);
        assert_eq!(houses[0].max_buildings, 90);
        assert_eq!(houses[0].edge, "North");
        assert_eq!(houses[1].credits, 2500);
        assert_eq!(houses[1].max_units, 150, "default when absent");
    }
    #[test]
    fn waypoints_are_listed() {
        assert_eq!(parsed().waypoint_ids, vec![1]);
    }

    #[test]
    fn bin_round_trips_a_square_grid() {
        let bytes = [1u8, 0, 2, 1, 3, 2, 4, 3];
        let map = decode_bin(&bytes, Some(2)).unwrap();
        assert_eq!((map.width, map.height), (2, 2));
        assert_eq!(
            map.get(0, 0),
            Some(BinCell {
                template: 1,
                icon: 0
            })
        );
        assert_eq!(
            map.get(1, 1),
            Some(BinCell {
                template: 4,
                icon: 3
            })
        );
        assert_eq!(map.get(2, 0), None);
    }

    #[test]
    fn bin_infers_square_side_from_length() {
        let bytes = vec![7u8; 3 * 3 * BIN_CELL_BYTES];
        let map = decode_bin(&bytes, None).unwrap();
        assert_eq!((map.width, map.height), (3, 3));
        assert_eq!(map.cells.len(), 9);
    }

    #[test]
    fn bin_rejects_garbage_of_odd_length() {
        assert!(decode_bin(&[0u8; 7], None).is_err());
    }

    #[test]
    fn loads_ini_and_bin_from_disk_end_to_end() {
        let dir = std::env::temp_dir().join(format!(
            "cnc-scenario-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let ini_path = dir.join("scen01.ini");
        std::fs::write(&ini_path, SCENARIO).unwrap();
        // 4x4 uncompressed cell image.
        let bin = vec![3u8; 4 * 4 * BIN_CELL_BYTES];
        std::fs::write(dir.join("scen01.bin"), &bin).unwrap();

        let (scenario, map) = load_scenario(&ini_path).unwrap();
        assert_eq!(scenario.name, "GDI 01 - X16");
        let map = map.expect("the sibling .bin must be picked up");
        assert_eq!((map.width, map.height), (4, 4));
        assert_eq!(
            map.get(3, 3),
            Some(BinCell {
                template: 3,
                icon: 3
            })
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_bin_leaves_the_map_absent() {
        let dir = std::env::temp_dir().join(format!(
            "cnc-scenario-nobin-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let ini_path = dir.join("scen02.ini");
        std::fs::write(&ini_path, SCENARIO).unwrap();

        let (_, map) = load_scenario(&ini_path).unwrap();
        assert!(map.is_none());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn unit_names_resolve_against_engine_types() {
        assert_eq!(unit_type_from_ini_name("E1"), Some(UnitType::InfantryE1));
        assert_eq!(unit_type_from_ini_name("mtnk"), Some(UnitType::MediumTank));
        assert_eq!(unit_type_from_ini_name("BGGY"), Some(UnitType::Buggy));
        // "RMBO" is the commando's UDATA name.
        assert_eq!(unit_type_from_ini_name("RMBO"), Some(UnitType::Commando));
        assert_eq!(unit_type_from_ini_name("NOPE"), None);
    }

    #[test]
    fn scenario_to_game_places_units_and_structures() {
        let (game, report) = scenario_to_game(&parsed(), None);
        assert_eq!(report.units_placed, 2, "both units resolve (E1, BGGY)");
        assert_eq!(
            report.structures_placed, 1,
            "FACT (construction yard) resolves"
        );
        assert!(!report.has_gaps());

        assert_eq!(game.units.len(), 2);
        assert_eq!(game.units[0].unit_type, UnitType::InfantryE1);
        assert_eq!(game.units[0].owner, 0);
        assert_eq!(game.units[1].unit_type, UnitType::Buggy);
        assert_eq!(game.units[1].owner, 1);

        assert_eq!(game.buildings.len(), 1);
        assert_eq!(game.buildings[0].owner, 0);
    }

    #[test]
    fn placed_objects_land_on_their_cell() {
        let (game, _) = scenario_to_game(&parsed(), None);
        // U1 sits at cell (10, 12); the engine positions units at cell centres.
        let unit = &game.units[0];
        let expected_x = 10.0 * TILE_SIZE + TILE_SIZE * 0.5;
        let expected_y = 12.0 * TILE_SIZE + TILE_SIZE * 0.5;
        assert!((unit.position.x.to_f32() - expected_x).abs() < 0.01);
        assert!((unit.position.y.to_f32() - expected_y).abs() < 0.01);
    }

    #[test]
    fn houses_track_player_credits_and_tech_level() {
        let (game, _) = scenario_to_game(&parsed(), None);
        assert!(game.houses.len() >= 2, "a player house and an opponent");
        assert_eq!(game.player_owner, 0);
        assert_eq!(game.houses[0].credits, 8000);
        assert_eq!(game.houses[0].tech_level, 7);
    }

    #[test]
    fn unmapped_types_are_reported_not_dropped_silently() {
        let ini = SCENARIO
            .replace("Type=BGGY", "Type=NOT_A_UNIT")
            .replace("Type=FACT", "Type=NOT_A_BUILDING");
        let scenario = parse_scenario(&IniFile::parse(&ini).unwrap()).unwrap();
        let (game, report) = scenario_to_game(&scenario, None);

        assert!(report.has_gaps());
        assert_eq!(report.unknown_units, vec!["NOT_A_UNIT".to_string()]);
        assert_eq!(
            report.unknown_structures,
            vec!["NOT_A_BUILDING".to_string()]
        );
        // The E1 still lands; only the unmapped ones are skipped.
        assert_eq!(game.units.len(), 1);
        assert!(game.buildings.is_empty());
    }

    #[test]
    fn bin_terrain_reaches_the_playable_grid() {
        let image = ScenarioMap {
            width: 2,
            height: 2,
            cells: vec![
                BinCell {
                    template: 9,
                    icon: 1,
                },
                BinCell {
                    template: 0,
                    icon: 0,
                },
                BinCell {
                    template: 0,
                    icon: 0,
                },
                BinCell {
                    template: 4,
                    icon: 2,
                },
            ],
        };
        let (game, _) = scenario_to_game(&parsed(), Some(&image));
        let corner = game.map.grid.get(Vec2i::new(0, 0)).unwrap();
        assert_eq!(corner.terrain_type, 9);
        assert_eq!(corner.overlay_type, 1);
        let far = game.map.grid.get(Vec2i::new(1, 1)).unwrap();
        assert_eq!(far.terrain_type, 4);
        assert_eq!(game.map.width, 2, "the map takes the .bin dimensions");
    }

    #[test]
    fn original_trigger_record_decodes_into_fields() {
        // Shape from TRIGGER.CPP TriggerClass::Write_INI.
        let record = parse_trigger_record("Held", "Time, Win, 120, GoodGuy, , 0");
        assert_eq!(record.name, "Held");
        assert_eq!(record.event, Some(11), "EVENT_TIME");
        assert_eq!(record.action, Some(1), "ACTION_WIN");
        assert_eq!(record.data, 120);
        assert_eq!(record.house, "GoodGuy");
        assert_eq!(record.persistent, 0);
    }

    #[test]
    fn unknown_event_and_action_names_are_not_guessed() {
        let record = parse_trigger_record("Odd", "Not An Event, Not An Action, 5, , , 0");
        assert_eq!(record.event, None);
        assert_eq!(record.action, None);
    }

    #[test]
    fn records_become_executable_triggers() {
        let records = vec![
            parse_trigger_record("Held", "Time, Win, 120, GoodGuy, , 0"),
            parse_trigger_record("Lost", "All Destr., Lose, 0, BadGuy, , 0"),
        ];
        let (triggers, unmapped) = trigger_records_to_triggers(&records);

        assert_eq!(triggers.len(), 2);
        assert_eq!(triggers[0].name, "Held");
        assert_eq!(triggers[0].condition, Condition::Elapsed { seconds: 120 });
        assert_eq!(triggers[0].actions, vec![Action::Win]);
        assert!(triggers[0].once, "persistent=0 means fire once");
        assert_eq!(triggers[1].actions, vec![Action::Lose]);

        // EVENT_ALL_DESTROYED (record code 9 = `EventText[9]` in TRIGGER.CPP;
        // TRIGGER.H:63 "all house's units & buildings destroyed") is detected in
        // HOUSE.CPP:1519 only when that house has no buildings, units, infantry
        // or aircraft left -- i.e. house elimination. `Condition::HouseEliminated
        // { what: Both }` (mission.rs) means exactly units == 0 && buildings == 0,
        // so this mapping is faithful, not a gap: "All Destr." must NOT be
        // reported as unmapped.
        match &triggers[1].condition {
            Condition::HouseEliminated {
                what: crate::mission::Eliminated::Both,
                ..
            } => {}
            ref other => {
                panic!("ALL_DESTROYED must map to HouseEliminated{{Both}}, got {other:?}")
            }
        }
        assert!(
            !unmapped.iter().any(|note| note.contains("All Destr.")),
            "ALL_DESTROYED has a real Condition and must not be unmapped: {unmapped:?}"
        );
    }

    #[test]
    fn genuinely_unmapped_event_is_reported() {
        // EVENT_ATTACKED (record code 3 = `EventText[3]`) is a real TD event that
        // has no Condition in the engine vocabulary, so it must still be surfaced
        // in `unmapped` rather than silently dropped.
        let records = vec![parse_trigger_record("Odd", "Attacked, Win, 0, BadGuy, , 0")];
        let (triggers, unmapped) = trigger_records_to_triggers(&records);
        assert_eq!(triggers.len(), 1);
        assert_eq!(triggers[0].condition, Condition::Never);
        assert!(
            unmapped.iter().any(|note| note == "event: Attacked"),
            "an unmapped event must be reported: {unmapped:?}"
        );
    }

    #[test]
    fn persistent_records_are_repeatable() {
        let records = vec![parse_trigger_record("Repeating", "Time, Win, 5, , , 1")];
        let (triggers, _) = trigger_records_to_triggers(&records);
        assert!(!triggers[0].once, "persistent > 0 means repeatable");
    }

    #[test]
    fn attachment_mode_tracks_is_persistant() {
        let once = parse_trigger_record("Once", "Time, Win, 5, , , 0");
        let repeat = parse_trigger_record("Repeat", "Time, Win, 5, , , 1");
        let negative = parse_trigger_record("Odd", "Time, Win, 5, , , -3");
        assert_eq!(once.attachment_mode(), TriggerMode::OneShot);
        assert_eq!(repeat.attachment_mode(), TriggerMode::Persistent);
        assert_eq!(
            negative.attachment_mode(),
            TriggerMode::OneShot,
            "anything not greater than zero is volatile, as in TriggerClass"
        );
    }

    #[test]
    fn credit_threshold_becomes_a_credits_condition() {
        let records = vec![parse_trigger_record("Rich", "Credits, Win, 5000, , , 0")];
        let (triggers, _) = trigger_records_to_triggers(&records);
        assert_eq!(
            triggers[0].condition,
            Condition::Credits {
                owner: 0,
                comparison: Comparison::AtLeast(5000)
            }
        );
    }

    #[test]
    fn recognises_original_format_vs_new_dialect() {
        assert!(looks_like_original_scenario(SCENARIO));
        let new_dialect = "[Basic]\nName=X\nPlayer=0\n[ScriptSetup]\nActions=win\n";
        assert!(!looks_like_original_scenario(new_dialect));
    }
}
