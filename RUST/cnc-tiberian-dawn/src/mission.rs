//! Campaign missions, trigger evaluation and scenario loading.
//!
//! Tiberian Dawn scenarios are INI files. The original engine kept triggers in
//! `[Triggers]` plus per-trigger sections with event/action records; the Rust
//! port uses a compact, declarative dialect that is easier to author and to
//! test while preserving the same semantics:
//!
//! - conditions are **level-triggered** (evaluated every simulation tick),
//! - a trigger fires once by default (`Once=yes`; set `Once=no` for repeatable
//!   triggers),
//! - `&` binds tighter than `|`, and `!` negates a condition,
//! - actions run in order and can end the mission (`win`/`lose`).
//!
//! Example:
//!
//! ```ini
//! [Basic]
//! Name=GDI 01 - X16
//! Player=0
//!
//! [Objectives]
//! 1=Destroy the Nod base
//! 2=Keep the GDI Construction Yard alive
//!
//! [Trigger1]
//! Name=Nod base destroyed
//! Condition=structures:1:any:==:0
//! Actions=message:Nod resistance eliminated|objective:1|win
//!
//! [Trigger2]
//! Name=GDI base lost
//! Condition=structures:0:any:==:0
//! Actions=lose
//! ```

use anyhow::{anyhow, bail, Context, Result};
use cnc_core::{Fixed16_16, IniFile, IniSection, Vec2i};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

use crate::game::Game;
use crate::map::TILE_SIZE;
use crate::td_rules::{structure_from_ini_name, StructureType, UnitType};

/// How a mission ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionOutcome {
    Victory,
    Defeat,
}

/// One line of the mission objectives list shown in the HUD.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Objective {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

/// Comparison operators used by trigger conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comparison {
    Less(i64),
    AtMost(i64),
    Equal(i64),
    AtLeast(i64),
    Greater(i64),
}

impl Comparison {
    fn parse(op: &str, value: i64) -> Result<Self> {
        match op.trim() {
            "<" => Ok(Self::Less(value)),
            "<=" => Ok(Self::AtMost(value)),
            "==" | "=" => Ok(Self::Equal(value)),
            ">=" => Ok(Self::AtLeast(value)),
            ">" => Ok(Self::Greater(value)),
            other => Err(anyhow!(
                "unknown comparison operator '{other}' (expected <, <=, ==, >= or >)"
            )),
        }
    }

    pub fn matches(&self, value: i64) -> bool {
        match *self {
            Self::Less(limit) => value < limit,
            Self::AtMost(limit) => value <= limit,
            Self::Equal(limit) => value == limit,
            Self::AtLeast(limit) => value >= limit,
            Self::Greater(limit) => value > limit,
        }
    }
}

/// A level-triggered campaign condition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Condition {
    /// Always true; useful for setup scripts that fire on tick 0.
    Always,
    /// Always false; useful to stage disabled triggers.
    Never,
    /// Mission clock has reached this many seconds.
    Elapsed { seconds: u32 },
    /// Alive units owned by `owner`, optionally filtered by type.
    UnitCount {
        owner: u8,
        unit_type: Option<UnitType>,
        comparison: Comparison,
    },
    /// Alive structures owned by `owner`, optionally filtered by type.
    StructureCount {
        owner: u8,
        structure: Option<StructureType>,
        comparison: Comparison,
    },
    /// `owner` has at least one live instance of the structure.
    HasStructure {
        owner: u8,
        structure: StructureType,
    },
    /// Spendable credits (stored tiberium + credits) for `owner`.
    Credits {
        owner: u8,
        comparison: Comparison,
    },
    /// Current power balance (production minus drain) for `owner`.
    Power {
        owner: u8,
        comparison: Comparison,
    },
    /// A unit (optionally of `owner`) stands inside a map trigger zone.
    ZoneOccupied { zone: u32, owner: Option<u8> },
    /// A named mission flag has been set by an earlier action.
    FlagSet(String),
    Not(Box<Condition>),
    All(Vec<Condition>),
    Any(Vec<Condition>),
}

impl Condition {
    /// Evaluate against the live simulation.
    pub fn matches(&self, game: &Game, flags: &BTreeSet<String>) -> bool {
        match self {
            Self::Always => true,
            Self::Never => false,
            Self::Elapsed { seconds } => game.elapsed >= Fixed16_16::from_int(*seconds as i32),
            Self::UnitCount {
                owner,
                unit_type,
                comparison,
            } => {
                let count = game
                    .units
                    .iter()
                    .filter(|unit| {
                        unit.is_alive()
                            && unit.owner == *owner
                            && unit_type.is_none_or(|wanted| wanted == unit.unit_type)
                    })
                    .count() as i64;
                comparison.matches(count)
            }
            Self::StructureCount {
                owner,
                structure,
                comparison,
            } => {
                let count = game
                    .buildings
                    .iter()
                    .filter(|building| {
                        building.is_alive()
                            && building.owner == *owner
                            && structure.is_none_or(|wanted| {
                                building.structure_type() == Some(wanted)
                            })
                    })
                    .count() as i64;
                comparison.matches(count)
            }
            Self::HasStructure { owner, structure } => game.buildings.iter().any(|building| {
                building.is_alive()
                    && building.owner == *owner
                    && building.structure_type() == Some(*structure)
            }),
            Self::Credits { owner, comparison } => {
                let money = game
                    .houses
                    .get(*owner as usize)
                    .map_or(0, |house| house.available_money() as i64);
                comparison.matches(money)
            }
            Self::Power { owner, comparison } => {
                let power = game
                    .houses
                    .get(*owner as usize)
                    .map_or(0, |house| house.power as i64);
                comparison.matches(power)
            }
            Self::ZoneOccupied { zone, owner } => {
                let Some(zone) = game.map.trigger_zones.iter().find(|z| z.id == *zone) else {
                    return false;
                };
                game.units.iter().any(|unit| {
                    if !unit.is_alive() || owner.is_some_and(|wanted| wanted != unit.owner) {
                        return false;
                    }
                    let x = unit.position.x.to_f32();
                    let y = unit.position.y.to_f32();
                    x >= zone.x1 as f32 * TILE_SIZE
                        && x <= (zone.x2 + 1) as f32 * TILE_SIZE
                        && y >= zone.y1 as f32 * TILE_SIZE
                        && y <= (zone.y2 + 1) as f32 * TILE_SIZE
                })
            }
            Self::FlagSet(name) => flags.contains(name),
            Self::Not(inner) => !inner.matches(game, flags),
            Self::All(conditions) => conditions.iter().all(|c| c.matches(game, flags)),
            Self::Any(conditions) => conditions.iter().any(|c| c.matches(game, flags)),
        }
    }
}

/// A side effect executed when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Win,
    Lose,
    /// Add credits to a house (negative values are allowed).
    Credits { owner: u8, amount: i32 },
    /// Spawn units near a map cell; they are spread over a small block.
    SpawnUnits {
        owner: u8,
        unit_type: UnitType,
        count: usize,
        cell: Vec2i,
    },
    /// Place a finished structure on a map cell.
    SpawnStructure {
        owner: u8,
        structure: StructureType,
        cell: Vec2i,
    },
    /// Append a line to the mission log / HUD.
    Message(String),
    /// Set a mission flag queried by `flag:name` conditions.
    SetFlag(String),
    /// Re-enable a trigger by id.
    EnableTrigger(u32),
    /// Disable a trigger by id.
    DisableTrigger(u32),
    /// Mark an objective complete by id.
    CompleteObjective(u32),
    /// Run a named `[Script*]` action list.
    RunScript(String),
}

/// One campaign trigger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub id: u32,
    pub name: String,
    pub condition: Condition,
    pub actions: Vec<Action>,
    /// Fire at most once (default). Set `Once=no` for repeatable triggers.
    pub once: bool,
    pub enabled: bool,
    pub fired: bool,
}

impl Trigger {
    pub fn new(id: u32, condition: Condition, actions: Vec<Action>) -> Self {
        Self {
            id,
            name: format!("Trigger {id}"),
            condition,
            actions,
            once: true,
            enabled: true,
            fired: false,
        }
    }

    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn repeatable(mut self) -> Self {
        self.once = false;
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// A reusable action list, run by `script:name` or a trigger.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Script {
    pub name: String,
    pub actions: Vec<Action>,
}

/// The active campaign mission: objectives, triggers, scripts and outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub name: String,
    /// Optional briefing text shown before the mission.
    #[serde(default)]
    pub briefing: String,
    /// House index controlled by the player (0 = GDI, 1 = Nod).
    pub player: u8,
    pub triggers: Vec<Trigger>,
    pub scripts: Vec<Script>,
    pub objectives: Vec<Objective>,
    /// Log lines produced by `message:` actions, newest last.
    pub messages: Vec<String>,
    pub outcome: Option<MissionOutcome>,
    pub flags: BTreeSet<String>,
    /// Optional scenario settings applied by [`Game::start_mission`].
    pub tech_level: Option<u8>,
    pub starting_credits: Option<i32>,
    fired_order: Vec<u32>,
}

impl Mission {
    pub fn new(name: String) -> Self {
        Self {
            name,
            briefing: String::new(),
            player: 0,
            triggers: Vec::new(),
            scripts: Vec::new(),
            objectives: Vec::new(),
            messages: Vec::new(),
            outcome: None,
            flags: BTreeSet::new(),
            tech_level: None,
            starting_credits: None,
            fired_order: Vec::new(),
        }
    }

    pub fn add_trigger(&mut self, trigger: Trigger) {
        self.triggers.push(trigger);
        self.triggers.sort_by_key(|trigger| trigger.id);
    }

    pub fn add_objective(&mut self, id: u32, text: impl Into<String>) {
        self.objectives.push(Objective {
            id,
            text: text.into(),
            completed: false,
        });
        self.objectives.sort_by_key(|objective| objective.id);
    }

    pub fn add_script(&mut self, script: Script) {
        self.scripts.push(script);
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn finish(&mut self, outcome: MissionOutcome) {
        if self.outcome.is_none() {
            self.outcome = Some(outcome);
        }
    }

    pub fn complete_objective(&mut self, id: u32) {
        if let Some(objective) = self.objectives.iter_mut().find(|o| o.id == id) {
            objective.completed = true;
        }
    }

    pub fn objectives_done(&self) -> usize {
        self.objectives.iter().filter(|o| o.completed).count()
    }

    /// Ids of every trigger that should fire on this tick, in id order.
    pub fn due(&self, game: &Game) -> Vec<u32> {
        if self.is_over() {
            return Vec::new();
        }
        self.triggers
            .iter()
            .filter(|trigger| {
                trigger.enabled
                    && !(trigger.once && trigger.fired)
                    && trigger.condition.matches(game, &self.flags)
            })
            .map(|trigger| trigger.id)
            .collect()
    }

    /// Mark a trigger fired and return a copy of its actions.
    pub fn fire(&mut self, id: u32) -> Vec<Action> {
        let Some(trigger) = self.triggers.iter_mut().find(|trigger| trigger.id == id) else {
            return Vec::new();
        };
        if !trigger.enabled || (trigger.once && trigger.fired) {
            return Vec::new();
        }
        trigger.fired = true;
        let actions = trigger.actions.clone();
        self.fired_order.push(id);
        actions
    }

    pub fn script(&self, name: &str) -> Option<&Script> {
        self.scripts.iter().find(|script| script.name == name)
    }

    /// Trigger ids in the order they fired this mission.
    pub fn fired_order(&self) -> &[u32] {
        &self.fired_order
    }

    pub fn enable(&mut self, id: u32) {
        if let Some(trigger) = self.triggers.iter_mut().find(|trigger| trigger.id == id) {
            trigger.enabled = true;
        }
    }

    pub fn disable(&mut self, id: u32) {
        if let Some(trigger) = self.triggers.iter_mut().find(|trigger| trigger.id == id) {
            trigger.enabled = false;
        }
    }

    /// Load a mission from an INI file on disk.
    pub fn from_file(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read mission {}", path.display()))?;
        let mission = Self::from_ini_str(&text)
            .with_context(|| format!("failed to parse mission {}", path.display()))?;
        Ok(mission)
    }

    /// Parse the mission dialect documented at the top of this module.
    pub fn from_ini_str(input: &str) -> Result<Self> {
        let ini = IniFile::parse(input).context("invalid INI syntax")?;
        Self::from_ini(&ini)
    }

    pub fn from_ini(ini: &IniFile) -> Result<Self> {
        let basic = ini.sections.get("Basic");
        let name = basic
            .and_then(|section| text_value(section, &["Name", "MissionName"]))
            .unwrap_or_else(|| "Untitled Mission".to_string());
        let mut mission = Self::new(name);

        if let Some(basic) = basic {
            if let Some(briefing) = text_value(basic, &["Briefing"]) {
                mission.briefing = briefing;
            }
            if let Some(player) = int_value(basic, &["Player", "Side"]).transpose()? {
                mission.player = player as u8;
            }
            if let Some(tech) = int_value(basic, &["TechLevel"]).transpose()? {
                mission.tech_level = Some(tech as u8);
            }
            if let Some(credits) = int_value(basic, &["Credits", "StartingCredits"]).transpose()? {
                mission.starting_credits = Some(credits as i32);
            }
        }

        if let Some(objectives) = ini.sections.get("Objectives") {
            for (key, value) in &objectives.values {
                if let (Ok(id), Some(text)) = (key.parse::<u32>(), value.as_string()) {
                    mission.add_objective(id, text);
                }
            }
        }

        for section in ini.sections.values() {
            if section.name.eq_ignore_ascii_case("Basic")
                || section.name.eq_ignore_ascii_case("Objectives")
            {
                continue;
            }
            if section.name.to_ascii_lowercase().starts_with("trigger") {
                let trigger = parse_trigger(section, mission.triggers.len() as u32 + 1)
                    .with_context(|| format!("in section [{}]", section.name))?;
                mission.add_trigger(trigger);
            } else if section.name.to_ascii_lowercase().starts_with("script") {
                let actions = text_value(section, &["Actions", "Action"])
                    .map(|text| parse_actions(&text))
                    .transpose()?
                    .unwrap_or_default();
                let name = text_value(section, &["Name"]).unwrap_or_else(|| section.name.clone());
                mission.add_script(Script { name, actions });
            }
        }

        mission.triggers.sort_by_key(|trigger| trigger.id);
        Ok(mission)
    }
}

fn text_value(section: &IniSection, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        section
            .get_ci(key)
            .and_then(|value| value.as_string().map(str::to_string))
    })
}

fn int_value(section: &IniSection, keys: &[&str]) -> Option<Result<i64>> {
    keys.iter().find_map(|key| {
        section
            .get_ci(key)
            .map(|value| value.as_i64().map_err(anyhow::Error::from))
    })
}

fn parse_trigger(section: &IniSection, fallback_id: u32) -> Result<Trigger> {
    let digits: String = section
        .name
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    let id = digits
        .parse::<u32>()
        .ok()
        .filter(|id| *id > 0)
        .unwrap_or(fallback_id);

    let condition_text = text_value(section, &["Condition", "Event", "Events"])
        .unwrap_or_else(|| "always".to_string());
    let condition = parse_condition(&condition_text)
        .with_context(|| format!("invalid Condition '{condition_text}'"))?;

    let actions_text =
        text_value(section, &["Actions", "Action"]).unwrap_or_else(|| "win".to_string());
    let actions =
        parse_actions(&actions_text).with_context(|| format!("invalid Actions '{actions_text}'"))?;

    let once = bool_value(section, &["Once", "SingleShot"])?.unwrap_or(true);
    let enabled = bool_value(section, &["Enabled"])?.unwrap_or(true);
    let name = text_value(section, &["Name"]).unwrap_or_else(|| section.name.clone());

    Ok(Trigger {
        id,
        name,
        condition,
        actions,
        once,
        enabled,
        fired: false,
    })
}

fn bool_value(section: &IniSection, keys: &[&str]) -> Result<Option<bool>> {
    for key in keys {
        if let Some(value) = section.get_ci(key) {
            return value.as_bool().map(Some).map_err(anyhow::Error::from);
        }
    }
    Ok(None)
}

/// Parse a condition expression. `|` is the loosest combinator, `&` next, then
/// a leading `!`. Primitive arguments are colon separated, e.g.
/// `units:1:MTNK:<=:2`.
pub fn parse_condition(text: &str) -> Result<Condition> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(Condition::Always);
    }
    if let Some(rest) = text.strip_prefix('!') {
        return Ok(Condition::Not(Box::new(parse_condition(rest)?)));
    }
    let any_parts = split_top_level(text, '|');
    if any_parts.len() > 1 {
        return Ok(Condition::Any(
            any_parts
                .into_iter()
                .map(parse_condition)
                .collect::<Result<Vec<_>>>()?,
        ));
    }
    let all_parts = split_top_level(text, '&');
    if all_parts.len() > 1 {
        return Ok(Condition::All(
            all_parts
                .into_iter()
                .map(parse_condition)
                .collect::<Result<Vec<_>>>()?,
        ));
    }
    parse_primitive_condition(text)
}

fn parse_primitive_condition(text: &str) -> Result<Condition> {
    let parts: Vec<&str> = text.split(':').map(str::trim).collect();
    let keyword = parts
        .first()
        .copied()
        .unwrap_or_default()
        .to_ascii_lowercase();
    match keyword.as_str() {
        "always" | "true" | "tick0" => Ok(Condition::Always),
        "never" | "false" => Ok(Condition::Never),
        "elapsed" | "time" => {
            let seconds = parse_number(parts.get(1).copied(), "elapsed seconds")?;
            Ok(Condition::Elapsed {
                seconds: seconds.max(0) as u32,
            })
        }
        "units" | "unit" => {
            require_len(&parts, 5, "units:owner:type|any:op:count")?;
            let owner = parse_owner(parts[1])?;
            let unit_type = parse_optional_unit(parts[2])?;
            let comparison = Comparison::parse(parts[3], parse_number(Some(parts[4]), "count")?)?;
            Ok(Condition::UnitCount {
                owner,
                unit_type,
                comparison,
            })
        }
        "structures" | "buildings" => {
            require_len(&parts, 5, "structures:owner:type|any:op:count")?;
            let owner = parse_owner(parts[1])?;
            let structure = parse_optional_structure(parts[2])?;
            let comparison = Comparison::parse(parts[3], parse_number(Some(parts[4]), "count")?)?;
            Ok(Condition::StructureCount {
                owner,
                structure,
                comparison,
            })
        }
        "has" | "hasstructure" => {
            require_len(&parts, 3, "has:owner:structure")?;
            Ok(Condition::HasStructure {
                owner: parse_owner(parts[1])?,
                structure: structure_from_ini_name(parts[2])
                    .ok_or_else(|| anyhow!("unknown structure '{}'", parts[2]))?,
            })
        }
        "credits" | "money" => {
            require_len(&parts, 4, "credits:owner:op:amount")?;
            Ok(Condition::Credits {
                owner: parse_owner(parts[1])?,
                comparison: Comparison::parse(parts[2], parse_number(Some(parts[3]), "amount")?)?,
            })
        }
        "power" => {
            require_len(&parts, 4, "power:owner:op:amount")?;
            Ok(Condition::Power {
                owner: parse_owner(parts[1])?,
                comparison: Comparison::parse(parts[2], parse_number(Some(parts[3]), "amount")?)?,
            })
        }
        "zone" => {
            require_len(&parts, 2, "zone:zoneId[:owner]")?;
            let zone = parse_number(parts.get(1).copied(), "zone id")? as u32;
            let owner = match parts.get(2) {
                Some(token) if !token.eq_ignore_ascii_case("any") => Some(parse_owner(token)?),
                _ => None,
            };
            Ok(Condition::ZoneOccupied { zone, owner })
        }
        "flag" => {
            require_len(&parts, 2, "flag:name")?;
            Ok(Condition::FlagSet(parts[1].to_string()))
        }
        other => bail!(
            "unknown condition '{other}' (expected elapsed, units, structures, has, credits, power, zone, flag, always, never)"
        ),
    }
}

/// Parse a `|`-separated action list.
pub fn parse_actions(text: &str) -> Result<Vec<Action>> {
    split_top_level(text, '|')
        .into_iter()
        .map(parse_action)
        .collect()
}

fn parse_action(text: &str) -> Result<Action> {
    let text = text.trim();
    let parts: Vec<&str> = text.splitn(2, ':').collect();
    let keyword = parts[0].trim().to_ascii_lowercase();
    let rest = parts.get(1).copied().map(str::trim);
    match keyword.as_str() {
        "win" | "victory" => Ok(Action::Win),
        "lose" | "defeat" => Ok(Action::Lose),
        "message" | "text" => Ok(Action::Message(rest.unwrap_or_default().to_string())),
        "flag" => Ok(Action::SetFlag(
            rest.ok_or_else(|| anyhow!("flag action needs a name"))?.to_string(),
        )),
        "credits" | "grant" => {
            let fields: Vec<&str> = rest
                .ok_or_else(|| anyhow!("credits action needs owner:amount"))?
                .split(':')
                .map(str::trim)
                .collect();
            require_len(&fields, 2, "credits:owner:amount")?;
            Ok(Action::Credits {
                owner: parse_owner(fields[0])?,
                amount: parse_number(Some(&fields[1]), "amount")? as i32,
            })
        }
        "spawn" => {
            let fields: Vec<&str> = rest
                .ok_or_else(|| anyhow!("spawn action needs owner:unit:count:x:y"))?
                .split(':')
                .map(str::trim)
                .collect();
            require_len(&fields, 5, "spawn:owner:unit:count:x:y")?;
            let unit_type = UnitType::ALL
                .into_iter()
                .find(|candidate| candidate.ini_name().eq_ignore_ascii_case(fields[1]))
                .ok_or_else(|| anyhow!("unknown unit type '{}'", fields[1]))?;
            Ok(Action::SpawnUnits {
                owner: parse_owner(fields[0])?,
                unit_type,
                count: parse_number(Some(&fields[2]), "count")?.max(1) as usize,
                cell: parse_cell(fields[3], fields[4])?,
            })
        }
        "build" | "structure" => {
            let fields: Vec<&str> = rest
                .ok_or_else(|| anyhow!("build action needs owner:structure:x:y"))?
                .split(':')
                .map(str::trim)
                .collect();
            require_len(&fields, 4, "build:owner:structure:x:y")?;
            let structure = structure_from_ini_name(fields[1])
                .ok_or_else(|| anyhow!("unknown structure '{}'", fields[1]))?;
            Ok(Action::SpawnStructure {
                owner: parse_owner(fields[0])?,
                structure,
                cell: parse_cell(fields[2], fields[3])?,
            })
        }
        "enable" => Ok(Action::EnableTrigger(
            parse_number(rest, "trigger id").map(|id| id as u32)?,
        )),
        "disable" => Ok(Action::DisableTrigger(
            parse_number(rest, "trigger id").map(|id| id as u32)?,
        )),
        "objective" => Ok(Action::CompleteObjective(
            parse_number(rest, "objective id").map(|id| id as u32)?,
        )),
        "script" => Ok(Action::RunScript(
            rest.ok_or_else(|| anyhow!("script action needs a name"))?.to_string(),
        )),
        other => bail!(
            "unknown action '{other}' (expected win, lose, message, flag, credits, spawn, build, enable, disable, objective, script)"
        ),
    }
}

fn split_top_level(text: &str, separator: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0usize;
    for (index, ch) in text.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ if ch == separator && depth == 0 => {
                parts.push(text[start..index].trim());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(text[start..].trim());
    parts.into_iter().filter(|part| !part.is_empty()).collect()
}

fn require_len(fields: &[&str], expected: usize, form: &str) -> Result<()> {
    if fields.len() < expected {
        bail!("expected {form}");
    }
    Ok(())
}

fn parse_number(value: Option<&str>, what: &str) -> Result<i64> {
    let value = value.ok_or_else(|| anyhow!("missing {what}"))?;
    value
        .parse::<i64>()
        .map_err(|_| anyhow!("invalid {what} '{value}'"))
}

fn parse_owner(value: &str) -> Result<u8> {
    match value.trim().to_ascii_lowercase().as_str() {
        "gdi" | "player" => Ok(0),
        "nod" | "enemy" => Ok(1),
        digits => digits
            .parse::<u8>()
            .map_err(|_| anyhow!("invalid owner '{value}' (expected 0/1, gdi or nod)")),
    }
}

fn parse_optional_unit(value: &str) -> Result<Option<UnitType>> {
    if value.eq_ignore_ascii_case("any") || value == "*" {
        return Ok(None);
    }
    UnitType::ALL
        .into_iter()
        .find(|candidate| candidate.ini_name().eq_ignore_ascii_case(value))
        .map(Some)
        .ok_or_else(|| anyhow!("unknown unit type '{value}'"))
}

fn parse_optional_structure(value: &str) -> Result<Option<StructureType>> {
    if value.eq_ignore_ascii_case("any") || value == "*" {
        return Ok(None);
    }
    structure_from_ini_name(value)
        .map(Some)
        .ok_or_else(|| anyhow!("unknown structure '{value}'"))
}

fn parse_cell(x: &str, y: &str) -> Result<Vec2i> {
    Ok(Vec2i::new(
        parse_number(Some(x), "cell x")? as i32,
        parse_number(Some(y), "cell y")? as i32,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;

    fn mission_ini() -> &'static str {
        r#"
[Basic]
Name=GDI 01 - X16
Player=0
TechLevel=98
Credits=5000

[Objectives]
1=Destroy the Nod base
2=Survive

[Script1]
Name=Victory
Actions=message:Nod resistance eliminated|objective:1|win

[Trigger1]
Name=Nod base destroyed
Condition=structures:1:any:==:0
Actions=script:Victory

[Trigger2]
Name=First minute
Condition=elapsed:60
Actions=message:One minute elapsed|credits:0:250
"#
    }

    #[test]
    fn parses_mission_ini() {
        let mission = Mission::from_ini_str(mission_ini()).unwrap();
        assert_eq!(mission.name, "GDI 01 - X16");
        assert_eq!(mission.player, 0);
        assert_eq!(mission.tech_level, Some(98));
        assert_eq!(mission.starting_credits, Some(5000));
        assert_eq!(mission.objectives.len(), 2);
        assert_eq!(mission.triggers.len(), 2);
        assert_eq!(mission.scripts.len(), 1);
        assert_eq!(mission.triggers[0].id, 1);
        assert!(mission.triggers[0].once);
        assert_eq!(
            mission.triggers[0].condition,
            Condition::StructureCount {
                owner: 1,
                structure: None,
                comparison: Comparison::Equal(0)
            }
        );
        assert_eq!(
            mission.triggers[1].actions,
            vec![
                Action::Message("One minute elapsed".to_string()),
                Action::Credits {
                    owner: 0,
                    amount: 250
                }
            ]
        );
    }

    #[test]
    fn condition_combinators_and_negation() {
        let condition =
            parse_condition("elapsed:10 & !has:1:PROC | credits:0:>=:1000").unwrap();
        match condition {
            Condition::Any(parts) => {
                assert_eq!(parts.len(), 2);
                match &parts[0] {
                    Condition::All(inner) => {
                        assert_eq!(inner.len(), 2);
                        assert!(matches!(inner[1], Condition::Not(_)));
                    }
                    other => panic!("expected All, got {other:?}"),
                }
            }
            other => panic!("expected Any, got {other:?}"),
        }
        assert!(parse_condition("bogus:1").is_err());
        assert!(parse_actions("win|nope").is_err());
    }

    #[test]
    fn elapsed_trigger_fires_once_and_grants_credits() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let mission = Mission::from_ini_str(mission_ini()).unwrap();
        game.start_mission(mission);
        let before = game.houses[0].credits;

        game.elapsed = Fixed16_16::from_int(59);
        game.update_mission(Fixed16_16::ONE);
        assert!(game.mission.fired_order().is_empty());

        game.elapsed = Fixed16_16::from_int(61);
        game.update_mission(Fixed16_16::ONE);
        assert_eq!(game.mission.fired_order(), &[2]);
        assert_eq!(game.houses[0].credits, before + 250);
        assert_eq!(game.mission.messages.last().unwrap(), "One minute elapsed");

        game.update_mission(Fixed16_16::ONE);
        assert_eq!(game.houses[0].credits, before + 250, "Once trigger must not repeat");
    }

    #[test]
    fn destroying_nod_structures_runs_script_and_wins() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.start_mission(Mission::from_ini_str(mission_ini()).unwrap());

        for building in &mut game.buildings {
            if building.owner == 1 {
                building.health = Fixed16_16::ZERO;
            }
        }
        game.update_mission(Fixed16_16::ONE);

        assert_eq!(game.mission.outcome, Some(MissionOutcome::Victory));
        assert!(game.mission.objectives[0].completed);
        assert_eq!(
            game.mission.messages.last().unwrap(),
            "Nod resistance eliminated"
        );
    }

    #[test]
    fn spawn_and_build_actions_create_entities() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let mut mission = Mission::new("reinforcements".to_string());
        mission.add_trigger(Trigger::new(
            1,
            Condition::Always,
            vec![
                Action::SpawnUnits {
                    owner: 0,
                    unit_type: UnitType::MediumTank,
                    count: 2,
                    cell: Vec2i::new(30, 30),
                },
                Action::SpawnStructure {
                    owner: 0,
                    structure: StructureType::PowerPlant,
                    cell: Vec2i::new(29, 29),
                },
                Action::SetFlag("reinforced".to_string()),
            ],
        ));
        mission.add_trigger(Trigger::new(
            2,
            Condition::FlagSet("reinforced".to_string()),
            vec![Action::Message("flag seen".to_string())],
        ));
        game.start_mission(mission);

        let units_before = game.units.len();
        let buildings_before = game.buildings.len();
        game.update_mission(Fixed16_16::ONE);
        game.update_mission(Fixed16_16::ONE);

        assert_eq!(game.units.len(), units_before + 2);
        assert_eq!(game.buildings.len(), buildings_before + 1);
        assert!(game.mission.messages.iter().any(|line| line == "flag seen"));
    }

    #[test]
    fn zone_condition_detects_units() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.map.trigger_zones.push(crate::map::TriggerZone {
            id: 7,
            x1: 25,
            y1: 25,
            x2: 27,
            y2: 27,
        });
        let condition = Condition::ZoneOccupied {
            zone: 7,
            owner: Some(0),
        };
        let mut unit = crate::unit::Unit::new(
            UnitType::MediumTank,
            cnc_core::Vec2::new(
                Fixed16_16::from_f32(26.5 * TILE_SIZE),
                Fixed16_16::from_f32(26.5 * TILE_SIZE),
            ),
            0,
        );
        unit.position = cnc_core::Vec2::new(
            Fixed16_16::from_f32(26.5 * TILE_SIZE),
            Fixed16_16::from_f32(26.5 * TILE_SIZE),
        );
        game.units.push(unit);
        assert!(condition.matches(&game, &BTreeSet::new()));
        assert!(!Condition::ZoneOccupied {
            zone: 99,
            owner: None
        }
        .matches(&game, &BTreeSet::new()));
    }

    #[test]
    fn shipped_sample_mission_parses() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../assets/missions/gdi_01.ini");
        let mission = Mission::from_file(&path).unwrap();
        assert_eq!(mission.name, "GDI 01 - X16");
        assert_eq!(mission.objectives.len(), 3);
        assert_eq!(mission.triggers.len(), 5);
        assert_eq!(mission.scripts.len(), 2);
        assert!(mission.triggers.iter().any(|trigger| !trigger.once));
    }

    #[test]
    fn repeatable_trigger_fires_every_tick() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let mut mission = Mission::new("repeat".to_string());
        mission.add_trigger(
            Trigger::new(
                1,
                Condition::Always,
                vec![Action::Credits {
                    owner: 0,
                    amount: 10,
                }],
            )
            .repeatable(),
        );
        game.start_mission(mission);
        let before = game.houses[0].credits;
        game.update_mission(Fixed16_16::ONE);
        game.update_mission(Fixed16_16::ONE);
        assert_eq!(game.houses[0].credits, before + 20);
    }

    #[test]
    fn disable_action_stops_trigger() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let mut mission = Mission::new("disable".to_string());
        mission.add_trigger(Trigger::new(
            1,
            Condition::Always,
            vec![Action::DisableTrigger(2)],
        ));
        mission.add_trigger(
            Trigger::new(
                2,
                Condition::Always,
                vec![Action::Message("must not run".to_string())],
            )
            .repeatable(),
        );
        game.start_mission(mission);
        game.update_mission(Fixed16_16::ONE);
        game.update_mission(Fixed16_16::ONE);
        assert!(!game.mission.messages.iter().any(|line| line == "must not run"));
    }
}
