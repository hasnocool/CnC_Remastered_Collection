//! Entity foundation types (plan Phase 5.1): the shared `Object` base,
//! `TaskForce`/`TeamType`/`Team` mission-teams, and the core-level `Trigger`.
//!
//! These are the additive, crate-shared building blocks that mirror the
//! Westwood `OBJECT.H` / Team / Trigger families so the game crates
//! (`cnc-tiberian-dawn`, `cnc-red-alert`) can adopt them without replacing
//! their existing dedicated house/team logic:
//!
//! - [`Object`] pairs an [`ObjectId`] (pointer-fixup id) with an owner house
//!   [`ClassId`], health/strength and OBJECT.H-style state flags.
//! - [`TaskForce`] members + [`TeamObjective`] compose into a [`TeamType`]
//!   (the "class"); [`Team`] is the running instance (members, objective
//!   progress), mirroring the original TeamClass/TeamTypeClass split.
//! - [`Trigger`] is the foundation event/action record so game crates can
//!   map their compact dialects onto it later.
//!
//! Everything here is plain serializable data (`serde`) with no init loops
//! and no unsafe; INI parsing helpers speak the original
//! `[TaskForces]`/`[TeamTypes]`/`[Triggers]` section dialect.

use crate::ini::{IniFile, IniSection};
use crate::pointers::ObjectId;
use crate::ClassId;
use anyhow::{bail, Context, Result};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use std::fmt;

bitflags! {
    /// OBJECT.H-style state flags (representative subset of the original
    /// `ObjectClass`/`AbstractClass` state bits).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct ObjectFlags: u16 {
        /// `DESTROYED`: object is dead. Held in sync by [`Object::take_damage`].
        const DESTROYED = 0x0001;
        /// `SELECTED`: user selection marker.
        const SELECTED = 0x0002;
        /// `IN_LINK`: object is participating in a link (building/vehicle).
        const IN_LINK = 0x0004;
        /// `ACTIVE`: object is logically alive and processing.
        const ACTIVE = 0x0008;
        /// `ATTACKING`: object is engaged in an attack.
        const ATTACKING = 0x0010;
        /// `FALLING`: spatial object is falling (aircraft/parachute).
        const FALLING = 0x0020;
    }
}

impl Serialize for ObjectFlags {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Serialize as raw bits; unknown bits are dropped on load
        // (forward-compatible with the flag-subset guarantee).
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for ObjectFlags {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u16::deserialize(deserializer)?))
    }
}

/// The shared Object base: `ObjectId` + owner house + health/strength +
/// state flags. Composition base for the game crates' concrete types, in
/// the Westwood `OBJECT.H` spirit.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object {
    /// Pointer-fixup id (`CodePointers`/`DecodePointers`).
    pub id: ObjectId,
    /// RTTI class (`ClassId`; concrete game domains never carry `Object` itself).
    pub class: ClassId,
    /// Owner house flavor (`HouseGDI`/`HouseNod`/`HouseAllies`/`HouseSoviet`/`House`).
    pub owner: ClassId,
    /// Maximum health from the type rule.
    pub strength: i32,
    /// Current health, always clamped to `0..=strength`.
    pub health: i32,
    /// State flags ([`ObjectFlags`]).
    pub flags: ObjectFlags,
}

impl Object {
    /// Create the object at full health; non-positive strength starts dead.
    pub fn new(id: ObjectId, class: ClassId, owner: ClassId, strength: i32) -> Self {
        let strength = strength.max(0);
        Self {
            id,
            class,
            owner,
            strength,
            health: strength,
            flags: ObjectFlags::default(),
        }
    }

    #[inline]
    pub fn is_destroyed(&self) -> bool {
        self.health <= 0
    }

    /// Apply damage; returns the portion actually applied (clamped to the
    /// remaining health). Dead objects take no further damage.
    pub fn take_damage(&mut self, amount: i32) -> i32 {
        if self.is_destroyed() {
            return 0;
        }
        let applied = amount.max(0).min(self.health);
        self.health -= applied;
        if self.health == 0 {
            self.flags.insert(ObjectFlags::DESTROYED);
        }
        applied
    }

    /// Restore health; returns the amount actually applied (never exceeds
    /// `strength`). Destroyed objects cannot be resurrected this way.
    pub fn heal(&mut self, amount: i32) -> i32 {
        if self.is_destroyed() {
            return 0;
        }
        let room = (self.strength - self.health).max(0);
        let applied = amount.max(0).min(room);
        self.health += applied;
        applied
    }

    /// Force death regardless of remaining health.
    pub fn mark_destroyed(&mut self) {
        self.health = 0;
        self.flags.insert(ObjectFlags::DESTROYED);
    }
}

/// Composition trait mirroring the Westwood `Object` base class: game
/// crates wrap an [`Object`] field and delegate through these methods.
pub trait HasObject {
    fn object(&self) -> &Object;
    fn object_mut(&mut self) -> &mut Object;

    /// Destroyed shortcut (defaults delegate to the wrapped [`Object`]).
    fn is_destroyed(&self) -> bool {
        self.object().is_destroyed()
    }

    /// Damage delegate; see [`Object::take_damage`].
    fn take_damage(&mut self, amount: i32) -> i32 {
        self.object_mut().take_damage(amount)
    }

    /// Heal delegate; see [`Object::heal`].
    fn heal(&mut self, amount: i32) -> i32 {
        self.object_mut().heal(amount)
    }

    /// Owner house shortcut.
    fn owner(&self) -> ClassId {
        self.object().owner
    }
}

impl HasObject for Object {
    fn object(&self) -> &Object {
        self
    }
    fn object_mut(&mut self) -> &mut Object {
        self
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Object({:?}#{:?} owner={:?} hp={}/{})",
            self.class, self.id, self.owner, self.health, self.strength
        )
    }
}

/// One taskforce row: `count` units of `class` ("1=MTNK,3" in the original
/// TASKFORCE.INI dialect).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamMember {
    pub class: ClassId,
    pub count: u8,
}

/// The taskforce half of a [`TeamType`]: named member composition.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskForce {
    pub name: String,
    pub members: Vec<TeamMember>,
}

impl TaskForce {
    /// Total units the taskforce composes.
    pub fn unit_count(&self) -> u32 {
        self.members
            .iter()
            .map(|member| u32::from(member.count))
            .sum()
    }
}

impl fmt::Display for TaskForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TaskForce({} units={})", self.name, self.unit_count())
    }
}

/// Mission objective on the shared foundation (mirrors the original Mission
/// enum family loosely).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamObjective {
    #[default]
    None,
    Unload,
    Move {
        cell: u16,
    },
    Attack,
    Guard,
    Patrol {
        waypoints: Vec<u16>,
    },
    Hunt,
    Retreat,
    EnterObject,
}

impl fmt::Display for TeamObjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Unload => write!(f, "Unload"),
            Self::Move { cell } => write!(f, "Move({cell})"),
            Self::Attack => write!(f, "Attack"),
            Self::Guard => write!(f, "Guard"),
            Self::Patrol { waypoints } => write!(f, "Patrol({waypoints:?})"),
            Self::Hunt => write!(f, "Hunt"),
            Self::Retreat => write!(f, "Retreat"),
            Self::EnterObject => write!(f, "EnterObject"),
        }
    }
}

/// Team class (`TeamTypeClass`): immutable composition of taskforce members
/// and the team objective; many [`Team`] instances can run from one type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamType {
    pub id: u32,
    pub name: String,
    /// Owner house flavor (same vocabulary as [`Object::owner`]).
    pub owner: ClassId,
    pub taskforce: TaskForce,
    pub objective: TeamObjective,
    /// `Active=` INI flag (matches at scenario start).
    pub active_at_start: bool,
}

impl TeamType {
    pub fn new(id: u32, name: impl Into<String>, owner: ClassId) -> Self {
        let name = name.into();
        Self {
            id,
            name: name.clone(),
            owner,
            taskforce: TaskForce {
                name,
                members: Vec::new(),
            },
            objective: TeamObjective::None,
            active_at_start: true,
        }
    }

    /// Replace the taskforce members.
    pub fn with_taskforce(mut self, members: Vec<TeamMember>) -> Self {
        self.taskforce.members = members;
        self
    }

    /// Set the `Active=` scenario-start flag.
    pub fn with_active_at_start(mut self, active: bool) -> Self {
        self.active_at_start = active;
        self
    }

    /// Replace the objective.
    pub fn with_objective(mut self, objective: TeamObjective) -> Self {
        self.objective = objective;
        self
    }

    /// Total units composed by the taskforce.
    pub fn unit_count(&self) -> u32 {
        self.taskforce.unit_count()
    }
}

impl fmt::Display for TeamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TeamType({} '{}' owner={:?} units={} objective={})",
            self.id,
            self.name,
            self.owner,
            self.unit_count(),
            self.objective
        )
    }
}

/// Team instance (`TeamClass`): running members, a copy of the type's
/// objective, and percent-complete progress (0..=100, deterministic clamp).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    /// Which [`TeamType`] this ran from (by id).
    pub team_type: u32,
    /// Live members by their [`ObjectId`]s.
    pub members: Vec<ObjectId>,
    /// Current instance objective (copied from the type at spawn).
    pub objective: TeamObjective,
    /// Percent progress toward the objective (0..=100).
    pub progress: u8,
    pub active: bool,
    pub suspended: bool,
}

impl Team {
    pub const MAX_PROGRESS: u8 = 100;

    /// Spawn a team instance from a type: copies the objective and honors
    /// the type's `Active=` flag.
    pub fn from_type(id: u32, team_type: &TeamType) -> Self {
        Self {
            id,
            team_type: team_type.id,
            members: Vec::new(),
            objective: team_type.objective.clone(),
            progress: 0,
            active: team_type.active_at_start,
            suspended: false,
        }
    }

    /// Default: inactive, suspended placeholder team.
    pub fn placeholder(id: u32, team_type: u32, objective: TeamObjective) -> Self {
        Self {
            id,
            team_type,
            members: Vec::new(),
            objective,
            progress: 0,
            active: false,
            suspended: true,
        }
    }

    /// Add a member; rejects duplicates and invalid ids, returns whether
    /// the membership changed.
    pub fn add_member(&mut self, id: ObjectId) -> bool {
        if !id.is_valid() {
            return false;
        }
        if self.contains_member(id) {
            return false;
        }
        self.members.push(id);
        true
    }

    /// Remove a member; returns whether it was present.
    pub fn remove_member(&mut self, id: ObjectId) -> bool {
        let before = self.members.len();
        self.members.retain(|&member| member != id);
        self.members.len() != before
    }

    #[inline]
    pub fn contains_member(&self, id: ObjectId) -> bool {
        self.members.contains(&id)
    }

    #[inline]
    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    /// Clear the roster (disband).
    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    /// Change the instance objective (the type's stays untouched).
    pub fn set_objective(&mut self, objective: TeamObjective) {
        self.objective = objective;
    }

    /// Advance/settle progress with a deterministic clamp to 0..=100.
    pub fn balance_progress(&mut self, delta: i32) {
        self.progress =
            (i32::from(self.progress) + delta).clamp(0, i32::from(Self::MAX_PROGRESS)) as u8;
    }

    #[inline]
    pub fn is_complete(&self) -> bool {
        self.progress >= Self::MAX_PROGRESS
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Team({} type={} members={} progress={}% objective={})",
            self.id,
            self.team_type,
            self.member_count(),
            self.progress,
            self.objective
        )
    }
}

/// Trigger event family on the shared foundation (modeled on the original
/// Trigger event records: attached-object/team, house and timer events).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerEvent {
    None,
    /// Attached object was created.
    Created,
    /// Enemy house discovered by the trigger owner.
    Discovered,
    HouseDiscovered {
        house: ClassId,
    },
    /// Someone entered the attached object.
    Entered,
    /// Attached object was attacked.
    Attacked,
    /// The attached building was destroyed.
    BuildingDestroyed,
    /// All of `house`'s buildings were destroyed.
    AllBuildingsDestroyed {
        house: ClassId,
    },
    /// The attached unit was destroyed.
    UnitDestroyed,
    /// All of `house`'s units were destroyed.
    AllUnitsDestroyed {
        house: ClassId,
    },
    /// Internal trigger timer expired (`repeat: false` springs once).
    TimerExpired {
        ticks: u32,
        repeat: bool,
    },
}

impl fmt::Display for TriggerEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Created => write!(f, "Created"),
            Self::Discovered => write!(f, "Discovered"),
            Self::HouseDiscovered { house } => write!(f, "HouseDiscovered({house:?})"),
            Self::Entered => write!(f, "Entered"),
            Self::Attacked => write!(f, "Attacked"),
            Self::BuildingDestroyed => write!(f, "BuildingDestroyed"),
            Self::AllBuildingsDestroyed { house } => {
                write!(f, "AllBuildingsDestroyed({house:?})")
            }
            Self::UnitDestroyed => write!(f, "UnitDestroyed"),
            Self::AllUnitsDestroyed { house } => write!(f, "AllUnitsDestroyed({house:?})"),
            Self::TimerExpired { ticks, repeat } => {
                write!(f, "TimerExpired({ticks} repeat={repeat})")
            }
        }
    }
}

/// Trigger action family on the shared foundation (mission-level actions so
/// game crates can map their compact dialects on top).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerAction {
    None,
    Win,
    Lose,
    Message(String),
    NextScenario(String),
    Reinforce {
        house: ClassId,
        cell: u16,
        count: u8,
    },
    DisableTrigger(u32),
    EnableTrigger(u32),
}

/// Core-level trigger record: owner house, attached-object/team flags, one
/// event and an ordered action list. `once` (default true) latches `fired`
/// on first spring (original behavior); repeatable triggers never latch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Trigger {
    pub id: u32,
    pub name: String,
    /// Owner house flavor (`HouseNod`, ...) executing on behalf of.
    pub owner: ClassId,
    /// Trigger attaches to a specific object (unit/building).
    pub attaches_to_object: bool,
    /// Trigger attaches to a team.
    pub attaches_to_team: bool,
    pub event: TriggerEvent,
    pub actions: Vec<TriggerAction>,
    /// Fire at most once (`Once=no` in INI for repeatable triggers).
    pub once: bool,
    pub enabled: bool,
    pub fired: bool,
}

impl Trigger {
    pub fn new(id: u32, event: TriggerEvent, actions: Vec<TriggerAction>) -> Self {
        Self {
            id,
            name: format!("Trigger {id}"),
            owner: ClassId::House,
            attaches_to_object: false,
            attaches_to_team: false,
            event,
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

    pub fn for_object(mut self) -> Self {
        self.attaches_to_object = true;
        self
    }

    pub fn for_team(mut self) -> Self {
        self.attaches_to_team = true;
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

    /// Would this trigger (considered fresh and enabled) match `event`?
    /// Exhausted once-triggers are excluded.
    pub fn matches(&self, event: &TriggerEvent) -> bool {
        self.enabled && !self.fired && self.event == *event
    }

    /// Evaluate and spring: returns whether the action list should run,
    /// latching `fired` for once-triggers.
    pub fn fire(&mut self, event: &TriggerEvent) -> bool {
        if !self.matches(event) {
            return false;
        }
        if self.once {
            self.fired = true;
        }
        true
    }
}

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trigger({} '{}' owner={:?} event={} actions={} attached_to=[object:{},team:{}])",
            self.id,
            self.name,
            self.owner,
            self.event,
            self.actions.len(),
            self.attaches_to_object,
            self.attaches_to_team
        )
    }
}

// ----------------------------------------------------------------------
// Original INI dialect parsing ([TaskForces] / [TeamTypes] / [Triggers])
// ----------------------------------------------------------------------

/// Parse every `[TaskForces]`-listed taskforce. Missing section → empty.
pub fn taskforces_from_ini(ini: &IniFile) -> Result<Vec<TaskForce>> {
    let list = match ini.section("TaskForces") {
        Some(section) => numbered_entries(section),
        None => return Ok(Vec::new()),
    };
    let mut taskforces = Vec::new();
    for (index, token) in list {
        let section = ini.section(&token).with_context(|| {
            format!("TaskForces entry {index} references missing taskforce '{token}'")
        })?;
        // The [TaskForces] list token (e.g. TF_ALPHA) is the canonical name
        // that TeamTypes' TaskForce= references; the section Name= is a
        // display-only label and is not used for resolution.
        let name = token.clone();
        let mut members = Vec::new();
        for (order, entry) in numbered_entries(section) {
            let (class_token, count) = split_class_entry(&entry);
            members.push(TeamMember {
                class: class_id_from_token(class_token)
                    .with_context(|| format!("taskforce '{token}' member {order}"))?,
                count,
            });
        }
        taskforces.push(TaskForce { name, members });
    }
    Ok(taskforces)
}

/// Parse every `[TeamTypes]`-listed team type, resolving its `TaskForce=`
/// against the parsed taskforces.
pub fn team_types_from_ini(ini: &IniFile) -> Result<Vec<TeamType>> {
    let taskforces = taskforces_from_ini(ini)?;
    let list = match ini.section("TeamTypes") {
        Some(section) => numbered_entries(section),
        None => bail!("no [TeamTypes] section in INI"),
    };
    let mut types = Vec::new();
    for (index, token) in list {
        let section = ini.section(&token).with_context(|| {
            format!("TeamTypes entry {index} references missing section '{token}'")
        })?;
        let name = section_string(section, "Name")
            .unwrap_or(token.as_str())
            .to_string();
        let owner = match section_string(section, "Owner") {
            Some(owner) => house_class_from_token(owner)
                .with_context(|| format!("team type '{token}' owner"))?,
            None => ClassId::House,
        };
        let taskforce = match section_string(section, "TaskForce") {
            Some(tf_name) => taskforces
                .iter()
                .find(|tf| tf.name.eq_ignore_ascii_case(tf_name))
                .cloned()
                .with_context(|| format!("team type '{token}' references taskforce '{tf_name}'"))?,
            None => TaskForce::default(),
        };
        let objective = match section_string(section, "Mission") {
            Some(mission) => team_objective_from_token(mission)
                .with_context(|| format!("team type '{token}' mission"))?,
            None => TeamObjective::None,
        };
        types.push(TeamType {
            id: index,
            name,
            owner,
            taskforce,
            objective,
            active_at_start: section_bool(section, "Active", true),
        });
    }
    Ok(types)
}

/// Parse every `[Triggers]`-listed trigger.
pub fn triggers_from_ini(ini: &IniFile) -> Result<Vec<Trigger>> {
    let list = match ini.section("Triggers") {
        Some(section) => numbered_entries(section),
        None => bail!("no [Triggers] section in INI"),
    };
    let mut triggers = Vec::new();
    for (index, token) in list {
        let section = ini.section(&token).with_context(|| {
            format!("Triggers entry {index} references missing section '{token}'")
        })?;
        let name = section_string(section, "Name")
            .unwrap_or(token.as_str())
            .to_string();
        let owner = match section_string(section, "House") {
            Some(owner) => {
                house_class_from_token(owner).with_context(|| format!("trigger '{token}' house"))?
            }
            None => ClassId::House,
        };
        let event = match section_string(section, "Event") {
            Some(event) => trigger_event_from_token(event)
                .with_context(|| format!("trigger '{token}' event"))?,
            None => TriggerEvent::None,
        };
        let mut actions = Vec::new();
        for (order, entry) in numbered_entries(section) {
            actions.push(
                trigger_action_from_token(&entry)
                    .with_context(|| format!("trigger '{token}' action {order}"))?,
            );
        }
        if actions.is_empty() {
            if let Some(compact) = section_string(section, "Actions") {
                for action_token in compact.split(',') {
                    actions.push(
                        trigger_action_from_token(action_token)
                            .with_context(|| format!("trigger '{token}' action"))?,
                    );
                }
            }
        }
        let mut trigger = Trigger::new(index, event, actions).named(name);
        trigger.owner = owner;
        trigger.once = section_bool(section, "Once", true);
        trigger.enabled = section_bool(section, "Enabled", true);
        if section_bool(section, "AttachesToObject", false) {
            trigger = trigger.for_object();
        }
        if section_bool(section, "AttachesToTeam", false) {
            trigger = trigger.for_team();
        }
        triggers.push(trigger);
    }
    Ok(triggers)
}

// ---------------------------------------------------------------------
// Token helpers
// ---------------------------------------------------------------------

/// Numbered INI entry pairs ("1=MTNK,3") sorted numerically so output and
/// action order are deterministic. Comma-separated values stored as arrays
/// by the INI parser are re-joined into their original text form.
fn numbered_entries(section: &IniSection) -> Vec<(u32, String)> {
    let mut entries: Vec<(u32, String)> = section
        .values
        .iter()
        .filter_map(|(key, value)| {
            let index: u32 = key.parse().ok()?;
            Some((index, ini_value_text(value)?))
        })
        .collect();
    entries.sort_by_key(|(index, _)| *index);
    entries
}

/// Recover the textual form of an INI value; arrays re-join on commas.
///
/// The INI parser type-splits comma values ("MTNK,3" → [String, Integer]),
/// so numeric members are rendered back to text.
fn ini_value_text(value: &crate::ini::IniValue) -> Option<String> {
    match value {
        crate::ini::IniValue::String(text) => Some(text.clone()),
        crate::ini::IniValue::Array(items) => Some(
            items
                .iter()
                .map(ini_item_text)
                .collect::<Vec<_>>()
                .join(","),
        ),
        _ => None,
    }
}

fn ini_item_text(value: &crate::ini::IniValue) -> String {
    match value {
        crate::ini::IniValue::String(text) => text.clone(),
        crate::ini::IniValue::Integer(value) => value.to_string(),
        crate::ini::IniValue::Boolean(value) => {
            if *value {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        crate::ini::IniValue::Float(value) => value.to_string(),
        crate::ini::IniValue::Array(items) => items
            .iter()
            .map(ini_item_text)
            .collect::<Vec<_>>()
            .join(","),
    }
}

fn section_string<'s>(section: &'s IniSection, key: &str) -> Option<&'s str> {
    section.get_ci(key).and_then(|value| value.as_string())
}

fn section_bool(section: &IniSection, key: &str, default: bool) -> bool {
    section
        .get_ci(key)
        .and_then(|value| value.as_bool().ok())
        .unwrap_or(default)
}

/// Split a member entry "MTNK,3" into its class token and unit count.
fn split_class_entry(entry: &str) -> (&str, u8) {
    match entry.split_once(',') {
        Some((class, count)) => (class.trim(), count.trim().parse::<u8>().unwrap_or(1)),
        None => (entry.trim(), 1),
    }
}

/// Map a house INI token onto its [`ClassId`] flavor. Foundation vocabulary
/// (GDI/Nod/Allies/Soviet + neutral); game crates can extend with country
/// names.
pub fn house_class_from_token(token: &str) -> Result<ClassId> {
    match token.trim().to_ascii_lowercase().as_str() {
        "house" | "neutral" | "none" => Ok(ClassId::House),
        "gdi" | "gdic" | "housegdi" | "good" | "goodguy" => Ok(ClassId::HouseGDI),
        "nod" | "nodc" | "housenod" | "bad" | "badguy" => Ok(ClassId::HouseNod),
        "allies" | "allied" | "houseallies" | "greece" => Ok(ClassId::HouseAllies),
        "soviet" | "ussr" | "housesoviet" | "russia" | "eaussr" => Ok(ClassId::HouseSoviet),
        _ => bail!("unknown house token '{token}'"),
    }
}

/// Well-known INI shorthand → [`ClassId`] for taskforce members. Partial
/// by design: the common TD infantry/vehicle shorthands plus generic names;
/// extend when game crates adopt concrete vocabularies.
fn class_id_from_shorthand(name: &str) -> Option<ClassId> {
    let class = match name {
        "UNIT" | "VEHICLE" => ClassId::Unit,
        "INFANTRY" => ClassId::Infantry,
        "AIRCRAFT" => ClassId::Aircraft,
        "SHIP" => ClassId::Ship,
        "BUILDING" => ClassId::Building,
        "HOUSE" => ClassId::House,
        "E1" => ClassId::InfantryE1,
        "E2" => ClassId::InfantryE2,
        "E3" => ClassId::InfantryE3,
        "E4" => ClassId::InfantryE4,
        "E5" => ClassId::InfantryE5,
        "E6" => ClassId::InfantryE6,
        "E7" => ClassId::InfantryE7,
        "E8" => ClassId::InfantryE8,
        "E9" => ClassId::InfantryE9,
        "E10" => ClassId::InfantryE10,
        "E11" => ClassId::InfantryE11,
        "LTNK" => ClassId::VehicleE1,
        "MTNK" => ClassId::VehicleE2,
        "HTNK" => ClassId::VehicleE3,
        "MAMM" => ClassId::VehicleE4,
        "APC" => ClassId::VehicleE5,
        "HARV" => ClassId::VehicleE6,
        "MCV" => ClassId::VehicleE7,
        "RANG" => ClassId::VehicleE8,
        "HUMV" => ClassId::VehicleE9,
        "MLRS" => ClassId::VehicleE10,
        "ARTY" => ClassId::VehicleE11,
        "V2" => ClassId::VehicleE12,
        "ORCA" => ClassId::AircraftE1,
        "TRAN" => ClassId::AircraftE2,
        "GUNBOAT" => ClassId::ShipE1,
        _ => return None,
    };
    Some(class)
}

/// Map a class token (numeric id or well-known INI shorthand) onto a
/// [`ClassId`].
pub fn class_id_from_token(token: &str) -> Result<ClassId> {
    let token = token.trim();
    if let Ok(numeric) = token.parse::<u32>() {
        return Ok(ClassId::from_u32(numeric));
    }
    class_id_from_shorthand(token.to_ascii_uppercase().as_str())
        .with_context(|| format!("unknown class token '{token}'"))
}

/// Parse a `Mission=` token: `none|unload|move:CELL|attack|guard|
/// patrol:WP[,WP...]|hunt|retreat|enter`.
pub fn team_objective_from_token(token: &str) -> Result<TeamObjective> {
    let token = token.trim();
    let (head, _rest) = match token.split_once(':') {
        Some(parts) => parts,
        None => (token, ""),
    };
    let objective = match head.to_ascii_lowercase().as_str() {
        "none" => TeamObjective::None,
        "unload" => TeamObjective::Unload,
        "move" => TeamObjective::Move {
            cell: u16::try_from(cell_arg(token)?).context("move cell out of u16 range")?,
        },
        "attack" => TeamObjective::Attack,
        "guard" => TeamObjective::Guard,
        "patrol" => {
            let args = token
                .split_once(':')
                .map(|(_, rest)| rest)
                .unwrap_or_default();
            let mut waypoints = Vec::new();
            for waypoint in args.split(',') {
                let value = waypoint.trim();
                if value.is_empty() {
                    continue;
                }
                waypoints.push(
                    u16::try_from(
                        crate::ini::IniValue::parse_integer(value)
                            .context("patrol waypoint out of range")?,
                    )
                    .context("patrol waypoint out of u16 range")?,
                );
            }
            TeamObjective::Patrol { waypoints }
        }
        "hunt" => TeamObjective::Hunt,
        "retreat" => TeamObjective::Retreat,
        "enter" | "enterobject" => TeamObjective::EnterObject,
        _ => bail!("unknown team mission token '{token}'"),
    };
    Ok(objective)
}

fn cell_arg(token: &str) -> Result<i64> {
    let arg = token
        .split_once(':')
        .map(|(_, rest)| rest.trim())
        .with_context(|| format!("mission token '{token}' requires a cell argument"))?;
    crate::ini::IniValue::parse_integer(arg).context("cell argument out of range")
}

/// Parse an `Event=` token from the foundation dialect.
pub fn trigger_event_from_token(token: &str) -> Result<TriggerEvent> {
    let (head, arg) = match token.split_once(':') {
        Some((head, arg)) => (head, arg),
        None => (token, ""),
    };
    let event = match head.to_ascii_lowercase().as_str() {
        "none" => TriggerEvent::None,
        "created" => TriggerEvent::Created,
        "discovered" => TriggerEvent::Discovered,
        "house_discovered" | "housediscovered" => TriggerEvent::HouseDiscovered {
            house: house_class_from_token(arg).context("house_discovered argument")?,
        },
        "entered" => TriggerEvent::Entered,
        "attacked" => TriggerEvent::Attacked,
        "building_destroyed" => TriggerEvent::BuildingDestroyed,
        "all_buildings_destroyed" => TriggerEvent::AllBuildingsDestroyed {
            house: house_class_from_token(arg)?,
        },
        "unit_destroyed" => TriggerEvent::UnitDestroyed,
        "all_units_destroyed" => TriggerEvent::AllUnitsDestroyed {
            house: house_class_from_token(arg)?,
        },
        "timer" => TriggerEvent::TimerExpired {
            ticks: u32::try_from(
                crate::ini::IniValue::parse_integer(arg.trim())
                    .context("timer ticks argument out of range")?,
            )
            .context("timer ticks out of u32 range")?,
            repeat: false,
        },
        _ => bail!("unknown trigger event token '{token}'"),
    };
    Ok(event)
}

/// Parse a numeric-key or `Actions=` action token from the foundation
/// dialect: `none|win|lose|message:TEXT|next_scenario:NAME|
/// reinforce:HOUSE:CELL:COUNT|disable_trigger:ID|enable_trigger:ID`.
pub fn trigger_action_from_token(token: &str) -> Result<TriggerAction> {
    let token = token.trim();
    let (head, arg) = match token.split_once(':') {
        Some((head, arg)) => (head, arg),
        None => (token, ""),
    };
    let action = match head.to_ascii_lowercase().as_str() {
        "none" => TriggerAction::None,
        "win" => TriggerAction::Win,
        "lose" => TriggerAction::Lose,
        "message" => TriggerAction::Message(arg.trim().to_string()),
        "next_scenario" | "nextscenario" => TriggerAction::NextScenario(arg.trim().to_string()),
        "reinforce" | "reinforcements" => {
            let mut parts = arg.split(':');
            let house = parts
                .next()
                .with_context(|| format!("reinforce token '{token}' requires a house"))?;
            let cell = parts
                .next()
                .and_then(crate::ini::IniValue::parse_integer)
                .context("reinforce cell argument required")?;
            let count = parts
                .next()
                .and_then(|count| crate::ini::IniValue::parse_integer(count.trim()))
                .context("reinforce count argument required")?;
            TriggerAction::Reinforce {
                house: house_class_from_token(house)?,
                cell: u16::try_from(cell).context("reinforce cell out of u16 range")?,
                count: u8::try_from(count).context("reinforce count out of u8 range")?,
            }
        }
        "disable_trigger" | "disabletrigger" => TriggerAction::DisableTrigger(
            u32::try_from(
                crate::ini::IniValue::parse_integer(arg.trim())
                    .context("disable_trigger id argument required")?,
            )
            .context("disable_trigger id out of u32 range")?,
        ),
        "enable_trigger" | "enabletrigger" => TriggerAction::EnableTrigger(
            u32::try_from(
                crate::ini::IniValue::parse_integer(arg.trim())
                    .context("enable_trigger id argument required")?,
            )
            .context("enable_trigger id out of u32 range")?,
        ),
        _ => bail!("unknown trigger action token '{token}'"),
    };
    Ok(action)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{de::DeserializeOwned, Serialize};

    fn round_trip<T: Serialize + DeserializeOwned>(value: T) -> T {
        let bytes = bincode::serialize(&value).expect("serialize");
        bincode::deserialize(&bytes).expect("deserialize")
    }

    // ------------------------------------------------------------------
    // Object foundation
    // ------------------------------------------------------------------

    #[test]
    fn object_new_starts_at_full_health() {
        let obj = Object::new(ObjectId::new(7), ClassId::VehicleE2, ClassId::HouseGDI, 400);
        assert_eq!(obj.id, ObjectId::new(7));
        assert_eq!(obj.class, ClassId::VehicleE2);
        assert_eq!(obj.owner, ClassId::HouseGDI);
        assert_eq!(obj.strength, 400);
        assert_eq!(obj.health, 400);
        assert!(!obj.is_destroyed());
    }

    #[test]
    fn object_take_damage_clamps_at_zero_and_marks_destroyed() {
        let mut obj = Object::new(ObjectId::new(1), ClassId::InfantryE1, ClassId::HouseNod, 60);
        assert_eq!(obj.take_damage(25), 25);
        assert_eq!(obj.health, 35);
        assert!(!obj.is_destroyed());

        // Overkill damage: only 35 applied, then the destroyed flag latches.
        assert_eq!(obj.take_damage(100), 35);
        assert_eq!(obj.health, 0);
        assert!(obj.is_destroyed());
        assert!(obj.flags.contains(ObjectFlags::DESTROYED));
    }

    #[test]
    fn object_heal_never_exceeds_strength() {
        let mut obj = Object::new(ObjectId::new(1), ClassId::VehicleE2, ClassId::HouseGDI, 400);
        assert_eq!(obj.take_damage(250), 250);
        assert_eq!(obj.health, 150);
        assert_eq!(obj.heal(150), 150);
        assert_eq!(obj.health, 300);
        assert_eq!(obj.heal(10_000), 100);
        assert_eq!(obj.health, 400);
    }

    #[test]
    fn object_dead_objects_take_no_damage_and_cannot_heal() {
        let mut obj = Object::new(ObjectId::new(1), ClassId::Vehicle, ClassId::HouseGDI, 400);
        obj.mark_destroyed();
        assert!(obj.is_destroyed());
        assert_eq!(obj.take_damage(50), 0);
        assert_eq!(obj.heal(50), 0);
        assert_eq!(obj.health, 0);
    }

    #[test]
    fn object_serde_round_trip() {
        let mut obj = Object::new(
            ObjectId::new(9),
            ClassId::VehicleE2,
            ClassId::HouseSoviet,
            400,
        );
        obj.take_damage(120);
        obj.flags.insert(ObjectFlags::SELECTED);
        let out: Object = round_trip(obj.clone());
        assert_eq!(out, obj);
        assert_eq!(out.health, 280);
        assert_eq!(out.owner, ClassId::HouseSoviet);
        assert!(out.flags.contains(ObjectFlags::SELECTED));
    }

    /// The Westwood `Object` base composes: game crates wrap an `Object`
    /// field and delegate through the `HasObject` trait.
    struct Unit {
        base: Object,
        xp: u32,
    }

    impl HasObject for Unit {
        fn object(&self) -> &Object {
            &self.base
        }
        fn object_mut(&mut self) -> &mut Object {
            &mut self.base
        }
    }

    #[test]
    fn has_object_trait_delegates_health_ops() {
        let mut unit = Unit {
            base: Object::new(ObjectId::new(2), ClassId::VehicleE1, ClassId::HouseNod, 300),
            xp: 0,
        };
        assert_eq!(unit.take_damage(100), 100);
        assert!(!unit.is_destroyed());
        assert_eq!(unit.owner(), ClassId::HouseNod);
        assert_ne!(
            unit.xp,
            u32::MAX,
            "extra fields compose beside the Object base"
        );
        assert_eq!(unit.heal(40), 40);
        assert_eq!(unit.base.health, 240);
        unit.base.mark_destroyed();
        assert!(unit.is_destroyed());
    }

    // ------------------------------------------------------------------
    // TaskForce / TeamType
    // ------------------------------------------------------------------

    fn sample_team_type() -> TeamType {
        TeamType::new(1, "Alpha Squad", ClassId::HouseGDI)
            .with_taskforce(vec![
                TeamMember {
                    class: ClassId::VehicleE2,
                    count: 3,
                },
                TeamMember {
                    class: ClassId::InfantryE1,
                    count: 2,
                },
            ])
            .with_objective(TeamObjective::Move { cell: 12 })
            .with_active_at_start(false)
    }

    #[test]
    fn taskforce_unit_count_sums_members() {
        let tt = sample_team_type();
        assert_eq!(tt.unit_count(), 5);
        assert_eq!(tt.taskforce.name, "Alpha Squad");
    }

    #[test]
    fn team_type_serde_round_trip() {
        let tt = sample_team_type();
        let out: TeamType = round_trip(tt.clone());
        assert_eq!(out, tt);
    }

    #[test]
    fn team_type_from_ini_parses_members_owner_and_mission() {
        let ini = IniFile::parse(
            "[TaskForces]\n1=TF_ALPHA\n[TF_ALPHA]\nName=Alpha Squad\n1=MTNK,3\n2=E1,2\n\
             [TeamTypes]\n1=ALPHA\n[ALPHA]\nName=Alpha Squad\nOwner=GDI\nTaskForce=TF_ALPHA\n\
             Mission=move:12\nActive=no\n",
        )
        .unwrap();

        let types = team_types_from_ini(&ini).unwrap();
        assert_eq!(types.len(), 1);
        let tt = &types[0];
        assert_eq!(tt.id, 1);
        assert_eq!(tt.name, "Alpha Squad");
        assert_eq!(tt.owner, ClassId::HouseGDI);
        assert!(!tt.active_at_start);
        assert_eq!(
            tt.taskforce.members,
            vec![
                TeamMember {
                    class: ClassId::VehicleE2,
                    count: 3
                },
                TeamMember {
                    class: ClassId::InfantryE1,
                    count: 2
                },
            ]
        );
        assert_eq!(tt.objective, TeamObjective::Move { cell: 12 });
    }

    // ------------------------------------------------------------------
    // Team instance
    // ------------------------------------------------------------------

    #[test]
    fn team_from_type_copies_objective_and_start_state() {
        let tt = sample_team_type();
        let team = Team::from_type(1, &tt);
        assert_eq!(team.team_type, 1);
        assert_eq!(team.objective, TeamObjective::Move { cell: 12 });
        assert!(!team.active, "Active=no type must spawn suspended");
        assert_eq!(team.progress, 0);
    }

    #[test]
    fn team_add_remove_members_is_deduplicating() {
        let tt = sample_team_type();
        let mut team = Team::from_type(1, &tt);
        let a = ObjectId::new(1);
        let b = ObjectId::new(2);

        assert!(team.add_member(a));
        assert!(team.add_member(b));
        assert!(!team.add_member(a), "duplicate member rejected");
        assert_eq!(team.member_count(), 2);
        assert!(team.contains_member(a));

        assert!(team.remove_member(a));
        assert!(!team.remove_member(a), "absent member remove rejected");
        assert_eq!(team.member_count(), 1);
        assert!(team.contains_member(b));
    }

    #[test]
    fn team_set_objective_changes_instance_copy_only() {
        let tt = sample_team_type();
        let mut team = Team::from_type(1, &tt);
        team.set_objective(TeamObjective::Hunt);
        assert_eq!(team.objective, TeamObjective::Hunt);
        assert_eq!(tt.objective, TeamObjective::Move { cell: 12 });
    }

    #[test]
    fn team_balance_progress_clamps_at_hundred() {
        let tt = sample_team_type();
        let mut team = Team::from_type(1, &tt);
        team.balance_progress(30);
        assert_eq!(team.progress, 30);
        team.balance_progress(-10);
        assert_eq!(team.progress, 20);
        team.balance_progress(10_000);
        assert_eq!(team.progress, 100);
        team.balance_progress(-1_000_000);
        assert_eq!(team.progress, 0);
        assert!(!team.is_complete());
    }

    #[test]
    fn team_is_complete_at_hundred_and_progress_is_clearable() {
        let tt = sample_team_type();
        let mut team = Team::from_type(1, &tt);
        team.balance_progress(100);
        assert!(team.is_complete());
        team.clear_members();
        assert_eq!(team.member_count(), 0);
    }

    #[test]
    fn team_serde_round_trip() {
        let tt = sample_team_type();
        let mut team = Team::from_type(1, &tt);
        team.add_member(ObjectId::new(3));
        team.add_member(ObjectId::new(4));
        team.balance_progress(40);
        let out: Team = round_trip(team.clone());
        assert_eq!(out, team);
        assert_eq!(out.members, vec![ObjectId::new(3), ObjectId::new(4)]);
        assert_eq!(out.progress, 40);
    }

    // ------------------------------------------------------------------
    // Core-level Trigger
    // ------------------------------------------------------------------

    #[test]
    fn trigger_matches_only_its_event() {
        let mut trigger = Trigger::new(1, TriggerEvent::Discovered, vec![TriggerAction::Win]);
        assert!(trigger.matches(&TriggerEvent::Discovered));
        assert!(!trigger.matches(&TriggerEvent::UnitDestroyed));
        assert!(!trigger.matches(&TriggerEvent::None));
        trigger.fire(&TriggerEvent::Discovered);
        assert!(trigger.fired);
        assert!(!trigger.matches(&TriggerEvent::Discovered));
    }

    #[test]
    fn trigger_with_payload_event_matches_exact_payload_only() {
        let trigger = Trigger::new(
            1,
            TriggerEvent::TimerExpired {
                ticks: 45,
                repeat: false,
            },
            vec![TriggerAction::Message("Tick 45".to_string())],
        );
        assert!(trigger.matches(&TriggerEvent::TimerExpired {
            ticks: 45,
            repeat: false
        }));
        assert!(!trigger.matches(&TriggerEvent::TimerExpired {
            ticks: 46,
            repeat: false
        }));
        assert!(!trigger.matches(&TriggerEvent::TimerExpired {
            ticks: 45,
            repeat: true
        }));
    }

    #[test]
    fn trigger_repeatable_fires_every_time() {
        let mut trigger = Trigger::new(
            2,
            TriggerEvent::TimerExpired {
                ticks: 30,
                repeat: true,
            },
            vec![TriggerAction::None],
        )
        .repeatable();
        assert!(trigger.fire(&TriggerEvent::TimerExpired {
            ticks: 30,
            repeat: true
        }));
        assert!(!trigger.fired);
        assert!(trigger.fire(&TriggerEvent::TimerExpired {
            ticks: 30,
            repeat: true
        }));
        assert!(!trigger.fire(&TriggerEvent::TimerExpired {
            ticks: 31,
            repeat: true
        }));
    }

    #[test]
    fn trigger_disabled_never_matches_nor_fires() {
        let mut trigger =
            Trigger::new(3, TriggerEvent::Attacked, vec![TriggerAction::Lose]).disabled();
        assert!(!trigger.matches(&TriggerEvent::Attacked));
        assert!(!trigger.fire(&TriggerEvent::Attacked));
        assert!(!trigger.fired);
    }

    #[test]
    fn trigger_serde_round_trip() {
        let trigger = Trigger::new(
            4,
            TriggerEvent::Discovered,
            vec![
                TriggerAction::Message("The enemy is here".to_string()),
                TriggerAction::Reinforce {
                    house: ClassId::HouseAllies,
                    cell: 77,
                    count: 3,
                },
            ],
        )
        .named("Base Found")
        .for_team();
        let out: Trigger = round_trip(trigger.clone());
        assert_eq!(out, trigger);
        assert_eq!(out.owner, ClassId::House);
        assert!(out.attaches_to_team);
        assert!(!out.attaches_to_object);
    }

    #[test]
    fn trigger_from_ini_parses_record_events_and_actions() {
        let ini = IniFile::parse(
            "[Triggers]\n1=A_BASE\n[A_BASE]\nName=Base Found\nHouse=Nod\n\
             Event=house_discovered:GDI\nAttachesToTeam=yes\nOnce=no\n\
             1=win\n2=message:GDI base discovered\n3=reinforce:Nod:77:3\n",
        )
        .unwrap();

        let triggers = triggers_from_ini(&ini).unwrap();
        assert_eq!(triggers.len(), 1);
        let t = &triggers[0];
        assert_eq!(t.id, 1);
        assert_eq!(t.name, "Base Found");
        assert_eq!(t.owner, ClassId::HouseNod);
        assert!(t.attaches_to_team);
        assert!(!t.once, "Once=no in the fixture disables latching");
        assert_eq!(
            t.event,
            TriggerEvent::HouseDiscovered {
                house: ClassId::HouseGDI
            }
        );
        assert_eq!(
            t.actions,
            vec![
                TriggerAction::Win,
                TriggerAction::Message("GDI base discovered".to_string()),
                TriggerAction::Reinforce {
                    house: ClassId::HouseNod,
                    cell: 77,
                    count: 3
                },
            ]
        );
        assert!(t.matches(&TriggerEvent::HouseDiscovered {
            house: ClassId::HouseGDI
        }));
    }

    // ------------------------------------------------------------------
    // Manual-QA debug dump (data-shaped; deterministic one-line summary)
    // ------------------------------------------------------------------

    #[test]
    fn debug_entity_foundation_dump() {
        // The shared Rules.ini fixture style used across cnc-core tests is
        // parsed both as a Rules database and as raw INI for the team and
        // trigger records, then summarized in one deterministic line.
        let rules =
            crate::RulesDatabase::from_str(RULES_INI_FIXTURE).expect("Rules fixture parses");
        let ini = IniFile::parse(RULES_INI_FIXTURE).expect("fixture parses as INI");
        let team_types = team_types_from_ini(&ini).unwrap();
        let triggers = triggers_from_ini(&ini).unwrap();
        assert_eq!(team_types.len(), 1);
        assert_eq!(triggers.len(), 1);

        let summary = format!(
            "QA entity-foundation: houses={}; {} | {}",
            rules.houses.len(),
            team_types[0],
            triggers[0]
        );
        println!("{summary}");
        assert!(summary.contains("TeamType"));
        assert!(summary.contains("Trigger"));
    }

    /// Sample shared Rules.ini fixture (same inline style used by the
    /// `rules.rs` tests) extended with team/trigger records.
    const RULES_INI_FIXTURE: &str = "[General]\nBaseSizeAdd=7\n\
        [Houses]\n1=GDI\n2=Nod\n\
        [GDI]\nName=Global Defense Initiative\nColor=Gold\n\
        [Nod]\nName=Brotherhood of Nod\nColor=Red\n\
        [MTNK]\nCost=900\nStrength=400\nSpeed=8\nPrimary=Cannon105\n\
        [Units]\n1=MTNK\n\
        [TaskForces]\n1=TF_ALPHA\n\
        [TF_ALPHA]\nName=Alpha Squad\n1=MTNK,3\n2=E1,2\n\
        [TeamTypes]\n1=ALPHA\n\
        [ALPHA]\nName=Alpha Squad\nOwner=GDI\nTaskForce=TF_ALPHA\nMission=move:12\nActive=no\n\
        [Triggers]\n1=A_BASE\n\
        [A_BASE]\nName=Base Found\nHouse=Nod\nEvent=house_discovered:GDI\nAttachesToTeam=yes\nOnce=no\n1=win\n2=message:GDI base discovered\n";

    // ------------------------------------------------------------------
    // Property tests (fixed seed + ChaCha: fully deterministic runs)
    // ------------------------------------------------------------------

    #[cfg(test)]
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config({
                let mut config = proptest::prelude::ProptestConfig::with_cases(256);
                config.rng_seed = proptest::test_runner::RngSeed::Fixed(0xC0FFEE);
                config.rng_algorithm = proptest::test_runner::RngAlgorithm::ChaCha;
                config
            })]

            #[test]
            fn object_health_stays_clamped(
                strength in 1..=5000u32,
                deltas in proptest::collection::vec(-1000i32..=1000i32, 0..24)
            ) {
                let mut obj = Object::new(
                    ObjectId::new(1),
                    ClassId::Vehicle,
                    ClassId::HouseGDI,
                    strength as i32,
                );
                for delta in deltas {
                    if delta < 0 {
                        obj.take_damage(-delta);
                    } else {
                        obj.heal(delta);
                    }
                    prop_assert!(obj.health >= 0, "negative health after delta {delta}");
                    prop_assert!(obj.health <= strength as i32, "healed past strength");
                    prop_assert_eq!(obj.is_destroyed(), obj.health == 0);
                }
                prop_assert_eq!(obj.is_destroyed(), obj.flags.contains(ObjectFlags::DESTROYED));
            }

            #[test]
            fn team_membership_tracks_set_oracle(
                ops in proptest::collection::vec((proptest::bool::ANY, 1u32..=8), 0..16)
            ) {
                let team_type = TeamType::new(1, "Oracle", ClassId::HouseGDI);
                let mut team = Team::from_type(1, &team_type);
                let mut oracle = std::collections::HashSet::new();
                for (add, raw) in ops {
                    let id = ObjectId::new(raw);
                    let changed = if add {
                        team.add_member(id)
                    } else {
                        team.remove_member(id)
                    };
                    prop_assert_eq!(changed, if add { oracle.insert(raw) } else { oracle.remove(&raw) });
                    prop_assert_eq!(team.member_count(), oracle.len());
                    let unique: std::collections::HashSet<_> = team.members.iter().copied().collect();
                    prop_assert_eq!(unique.len(), team.members.len(), "duplicate member leaked");
                    prop_assert_eq!(team.contains_member(id), oracle.contains(&raw));
                }
            }

            #[test]
            fn trigger_matches_exactly_when_events_equal(
                (event, other) in (any_trigger_event(), any_trigger_event())
            ) {
                let mut trigger =
                    Trigger::new(1, event.clone(), vec![TriggerAction::None]).repeatable();
                prop_assert_eq!(trigger.matches(&other), event == other);
                prop_assert_eq!(trigger.fire(&other), event == other);
                let mut once_trigger = Trigger::new(1, event.clone(), vec![TriggerAction::None]);
                prop_assert_eq!(once_trigger.fire(&other), event == other);
                if event == other {
                    prop_assert!(!once_trigger.matches(&other), "once trigger must go silent");
                }
            }
        }

        fn any_trigger_event() -> impl proptest::prelude::Strategy<Value = TriggerEvent> {
            use proptest::prelude::*;
            prop_oneof![
                Just(TriggerEvent::None),
                Just(TriggerEvent::Created),
                Just(TriggerEvent::Discovered),
                Just(TriggerEvent::Entered),
                Just(TriggerEvent::Attacked),
                Just(TriggerEvent::BuildingDestroyed),
                Just(TriggerEvent::UnitDestroyed),
                any::<u32>().prop_map(|ticks| TriggerEvent::TimerExpired {
                    ticks,
                    repeat: false,
                }),
                (0u32..5).prop_map(|slot| TriggerEvent::HouseDiscovered {
                    house: ClassId::from_u32(100 + slot),
                }),
            ]
        }
    }
}
