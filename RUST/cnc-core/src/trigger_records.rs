//! Original trigger event/action record strings (`TEVENT` / `TACTION`).
//!
//! The original engine stores one trigger event or action per INI value as a
//! comma separated record string. The shapes come straight from the two
//! serialisers:
//!
//! | record | writer | fields |
//! |---|---|---|
//! | event  | `REDALERT/TEVENT.CPP:486` `TEventClass::Build_INI_Entry` | `<event>,<team logical id>,<value>` |
//! | action | `REDALERT/TACTION.CPP:225` `TActionClass::Build_INI_Entry` | `<action>,<team logical id>,<trigger logical id>,<value>` |
//!
//! The record codes are the enumerations from `REDALERT/TEVENT.H:43-80`
//! (`TEventType`, codes 0..=32, `TEVENT_COUNT` 33) and
//! `REDALERT/TACTION.H:39-86` (`TActionType`, codes 0..=36,
//! `TACTION_COUNT` 37). What the trailing `value` means for a given code
//! follows `Event_Needs` (`REDALERT/TEVENT.CPP:563`) and `Action_Needs`
//! (`REDALERT/TACTION.CPP:839`); that mapping is exposed through
//! [`EventKind::param_kind`] / [`ActionKind::param_kind`] and resolved per
//! record through [`EventRecord::payload`] / [`ActionRecord::payload`].
//!
//! Team and trigger references are heap indices: `FixedIHeapClass::Logical_ID`
//! (`REDALERT/HEAP.CPP:497`) returns `-1` for a null pointer, modelled here
//! as `None`.
//!
//! Codes outside the enumerated range are preserved as [`EventKind::Unknown`] /
//! [`ActionKind::Unknown`] instead of being rejected, so a record from a newer
//! scenario revision still round-trips. Malformed records never panic: they
//! return a [`TriggerRecordError`] that carries the offending record code.
//!
//! Tiberian Dawn serialises the same shape family with its own enumerations and
//! mnemonic names (`TIBERIANDAWN/TRIGGER.H:38-104`, written by
//! `TIBERIANDAWN/TRIGGER.CPP:1221` as
//! `<event name>,<action name>,<data>,<house>,<team>,<persistant>`). That
//! dialect belongs to the TD scenario loader; the code space shared here is the
//! Red Alert one.

use std::fmt;

use thiserror::Error;

/// Number of enumerated event codes (`TEVENT_COUNT` in `REDALERT/TEVENT.H`).
pub const EVENT_CODE_COUNT: u32 = 33;

/// Number of enumerated action codes (`TACTION_COUNT` in `REDALERT/TACTION.H`).
pub const ACTION_CODE_COUNT: u32 = 37;

/// A failure while parsing an original trigger record string.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TriggerRecordError {
    /// The record was empty or whitespace only, so it carries no code.
    #[error("empty trigger record")]
    Empty,

    /// The record did not have the number of comma separated fields its shape needs.
    #[error(
        "trigger record {record:?} for code {code:?} has {found} field(s); expected {expected}"
    )]
    FieldCount {
        /// The record as written in the INI.
        record: String,
        /// The record code field (the event or action token).
        code: String,
        /// How many comma separated fields were actually present.
        found: usize,
        /// The expected shape, rendered for humans.
        expected: &'static str,
    },

    /// A field that must be an integer was not one, or was out of range.
    #[error("trigger record {record:?} for code {code:?} has an invalid {field}: {value:?}")]
    InvalidNumber {
        /// The record as written in the INI.
        record: String,
        /// The record code field (the event or action token).
        code: String,
        /// Name of the offending field.
        field: &'static str,
        /// The offending field text.
        value: String,
    },
}

impl TriggerRecordError {
    /// The record code the failure is attributed to, when there is one.
    pub fn code(&self) -> Option<&str> {
        match self {
            Self::Empty => None,
            Self::FieldCount { code, .. } | Self::InvalidNumber { code, .. } => Some(code),
        }
    }
}

/// Semantics of the trailing `value` of an event or action record.
///
/// Mirrors `NeedType` as returned by `Event_Needs` / `Action_Needs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamKind {
    /// `NEED_NONE`: the code carries no payload.
    None,
    /// `NEED_HOUSE`: a `HousesType` index.
    House,
    /// `NEED_NUMBER`: a plain number (credits, timer ticks, global index, ...).
    Number,
    /// `NEED_STRUCTURE`: a `StructType` index.
    Structure,
    /// `NEED_UNIT`: a `UnitType` index.
    Unit,
    /// `NEED_INFANTRY`: an `InfantryType` index.
    Infantry,
    /// `NEED_AIRCRAFT`: an `AircraftType` index.
    Aircraft,
    /// `NEED_TEAM`: a `TeamTypeClass` logical id.
    Team,
    /// `NEED_TRIGGER`: a `TriggerTypeClass` logical id.
    Trigger,
    /// `NEED_WAYPOINT`: a waypoint number.
    Waypoint,
    /// `NEED_THEME`: a `ThemeType` index.
    Theme,
    /// `NEED_MOVIE`: a `VQType` index.
    Movie,
    /// `NEED_SOUND`: a `VocType` index.
    Sound,
    /// `NEED_SPEECH`: a `VoxType` index.
    Speech,
    /// `NEED_BOOL`: a boolean stored as an integer.
    Bool,
    /// `NEED_SPECIAL`: a `SpecialWeaponType` index.
    SpecialWeapon,
    /// `NEED_QUARRY`: a `QuarryType` index.
    Quarry,
}

impl ParamKind {
    /// Attach a raw record value to this parameter role.
    fn wrap(self, value: i64) -> ParamValue {
        match self {
            Self::None => ParamValue::None,
            Self::House => ParamValue::House(value),
            Self::Number => ParamValue::Number(value),
            Self::Structure => ParamValue::Structure(value),
            Self::Unit => ParamValue::Unit(value),
            Self::Infantry => ParamValue::Infantry(value),
            Self::Aircraft => ParamValue::Aircraft(value),
            Self::Team => ParamValue::Team(value),
            Self::Trigger => ParamValue::Trigger(value),
            Self::Waypoint => ParamValue::Waypoint(value),
            Self::Theme => ParamValue::Theme(value),
            Self::Movie => ParamValue::Movie(value),
            Self::Sound => ParamValue::Sound(value),
            Self::Speech => ParamValue::Speech(value),
            Self::Bool => ParamValue::Bool(value),
            Self::SpecialWeapon => ParamValue::SpecialWeapon(value),
            Self::Quarry => ParamValue::Quarry(value),
        }
    }
}

/// A typed payload: the record's `value` together with what it addresses.
///
/// The integer is kept as parsed (the original engines store it in a `long`),
/// so the role is typed while the raw value stays lossless.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamValue {
    /// No payload (`NEED_NONE`).
    None,
    /// A house index.
    House(i64),
    /// A plain number.
    Number(i64),
    /// A structure type index.
    Structure(i64),
    /// A unit type index.
    Unit(i64),
    /// An infantry type index.
    Infantry(i64),
    /// An aircraft type index.
    Aircraft(i64),
    /// A team type logical id.
    Team(i64),
    /// A trigger type logical id.
    Trigger(i64),
    /// A waypoint number.
    Waypoint(i64),
    /// A music theme index.
    Theme(i64),
    /// A movie index.
    Movie(i64),
    /// A sound effect index.
    Sound(i64),
    /// An EVA speech index.
    Speech(i64),
    /// A boolean stored as an integer.
    Bool(i64),
    /// A special weapon index.
    SpecialWeapon(i64),
    /// A preferred-target (quarry) index.
    Quarry(i64),
}

impl ParamValue {
    /// The raw integer payload, or `None` when the code carries no payload.
    pub fn raw(self) -> Option<i64> {
        match self {
            Self::None => None,
            Self::House(v)
            | Self::Number(v)
            | Self::Structure(v)
            | Self::Unit(v)
            | Self::Infantry(v)
            | Self::Aircraft(v)
            | Self::Team(v)
            | Self::Trigger(v)
            | Self::Waypoint(v)
            | Self::Theme(v)
            | Self::Movie(v)
            | Self::Sound(v)
            | Self::Speech(v)
            | Self::Bool(v)
            | Self::SpecialWeapon(v)
            | Self::Quarry(v) => Some(v),
        }
    }
}

const EVENT_NAMES: [&str; EVENT_CODE_COUNT as usize] = [
    "-No Event-",
    "Entered by...",
    "Spied by...",
    "Thieved by...",
    "Discovered by player",
    "House Discovered...",
    "Attacked by anybody",
    "Destroyed by anybody",
    "Any Event",
    "Destroyed, Units, All...",
    "Destroyed, Buildings, All...",
    "Destroyed, All...",
    "Credits exceed (x100)...",
    "Elapsed Time (1/10th min)...",
    "Mission Timer Expired",
    "Destroyed, Buildings, #...",
    "Destroyed, Units, #...",
    "No Factories left",
    "Civilians Evacuated",
    "Build Building Type...",
    "Build Unit Type...",
    "Build Infantry Type...",
    "Build Aircraft Type...",
    "Leaves map (team)...",
    "Zone Entry by...",
    "Crosses Horizontal Line...",
    "Crosses Vertical Line...",
    "Global is set...",
    "Global is clear...",
    "Destroyed, Fakes, All...",
    "Low Power...",
    "All bridges destroyed",
    "Building exists...",
];

const ACTION_NAMES: [&str; ACTION_CODE_COUNT as usize] = [
    "-No Action-",
    "Winner is...",
    "Loser is...",
    "Production Begins",
    "Create Team...",
    "Destroy All Teams",
    "All to Hunt...",
    "Reinforcement (team)...",
    "Drop Zone Flare (waypoint)...",
    "Fire Sale...",
    "Play Movie...",
    "Text Trigger (ID num)...",
    "Destroy Trigger...",
    "Autocreate Begins...",
    "~don't use~",
    "Allow Win",
    "Reveal all map",
    "Reveal around waypoint...",
    "Reveal zone of waypoint...",
    "Play sound effect...",
    "Play music theme...",
    "Play speech...",
    "Force Trigger...",
    "Timer Start",
    "Timer Stop",
    "Timer Extend (1/10th min)...",
    "Timer Shorten (1/10th min)...",
    "Timer Set (1/10th min)...",
    "Global Set...",
    "Global Clear...",
    "Auto Base Building...",
    "Grow shroud one 'step'",
    "Destroy attached building",
    "Add 1-time special weapon...",
    "Add repeating special weapon...",
    "Preferred target...",
    "Launch Nukes",
];

/// A trigger event code (`TEventType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    /// `TEVENT_NONE` (0).
    None,
    /// `TEVENT_PLAYER_ENTERED` (1).
    PlayerEntered,
    /// `TEVENT_SPIED` (2).
    Spied,
    /// `TEVENT_THIEVED` (3).
    Thieved,
    /// `TEVENT_DISCOVERED` (4).
    Discovered,
    /// `TEVENT_HOUSE_DISCOVERED` (5).
    HouseDiscovered,
    /// `TEVENT_ATTACKED` (6).
    Attacked,
    /// `TEVENT_DESTROYED` (7).
    Destroyed,
    /// `TEVENT_ANY` (8).
    Any,
    /// `TEVENT_UNITS_DESTROYED` (9).
    UnitsDestroyed,
    /// `TEVENT_BUILDINGS_DESTROYED` (10).
    BuildingsDestroyed,
    /// `TEVENT_ALL_DESTROYED` (11).
    AllDestroyed,
    /// `TEVENT_CREDITS` (12).
    Credits,
    /// `TEVENT_TIME` (13).
    ElapsedTime,
    /// `TEVENT_MISSION_TIMER_EXPIRED` (14).
    MissionTimerExpired,
    /// `TEVENT_NBUILDINGS_DESTROYED` (15).
    BuildingsDestroyedCount,
    /// `TEVENT_NUNITS_DESTROYED` (16).
    UnitsDestroyedCount,
    /// `TEVENT_NOFACTORIES` (17).
    NoFactories,
    /// `TEVENT_EVAC_CIVILIAN` (18).
    EvacCivilian,
    /// `TEVENT_BUILD` (19).
    Build,
    /// `TEVENT_BUILD_UNIT` (20).
    BuildUnit,
    /// `TEVENT_BUILD_INFANTRY` (21).
    BuildInfantry,
    /// `TEVENT_BUILD_AIRCRAFT` (22).
    BuildAircraft,
    /// `TEVENT_LEAVES_MAP` (23).
    LeavesMap,
    /// `TEVENT_ENTERS_ZONE` (24).
    EntersZone,
    /// `TEVENT_CROSS_HORIZONTAL` (25).
    CrossHorizontal,
    /// `TEVENT_CROSS_VERTICAL` (26).
    CrossVertical,
    /// `TEVENT_GLOBAL_SET` (27).
    GlobalSet,
    /// `TEVENT_GLOBAL_CLEAR` (28).
    GlobalClear,
    /// `TEVENT_FAKES_DESTROYED` (29).
    FakesDestroyed,
    /// `TEVENT_LOW_POWER` (30).
    LowPower,
    /// `TEVENT_ALL_BRIDGES_DESTROYED` (31).
    AllBridgesDestroyed,
    /// `TEVENT_BUILDING_EXISTS` (32).
    BuildingExists,
    /// A code outside the enumeration; preserved verbatim.
    Unknown(u32),
}

impl EventKind {
    /// Resolve a numeric record code, keeping unknown codes verbatim.
    pub fn from_code(code: u32) -> Self {
        match code {
            0 => Self::None,
            1 => Self::PlayerEntered,
            2 => Self::Spied,
            3 => Self::Thieved,
            4 => Self::Discovered,
            5 => Self::HouseDiscovered,
            6 => Self::Attacked,
            7 => Self::Destroyed,
            8 => Self::Any,
            9 => Self::UnitsDestroyed,
            10 => Self::BuildingsDestroyed,
            11 => Self::AllDestroyed,
            12 => Self::Credits,
            13 => Self::ElapsedTime,
            14 => Self::MissionTimerExpired,
            15 => Self::BuildingsDestroyedCount,
            16 => Self::UnitsDestroyedCount,
            17 => Self::NoFactories,
            18 => Self::EvacCivilian,
            19 => Self::Build,
            20 => Self::BuildUnit,
            21 => Self::BuildInfantry,
            22 => Self::BuildAircraft,
            23 => Self::LeavesMap,
            24 => Self::EntersZone,
            25 => Self::CrossHorizontal,
            26 => Self::CrossVertical,
            27 => Self::GlobalSet,
            28 => Self::GlobalClear,
            29 => Self::FakesDestroyed,
            30 => Self::LowPower,
            31 => Self::AllBridgesDestroyed,
            32 => Self::BuildingExists,
            other => Self::Unknown(other),
        }
    }

    /// The numeric code this kind serialises to.
    pub fn code(self) -> u32 {
        match self {
            Self::None => 0,
            Self::PlayerEntered => 1,
            Self::Spied => 2,
            Self::Thieved => 3,
            Self::Discovered => 4,
            Self::HouseDiscovered => 5,
            Self::Attacked => 6,
            Self::Destroyed => 7,
            Self::Any => 8,
            Self::UnitsDestroyed => 9,
            Self::BuildingsDestroyed => 10,
            Self::AllDestroyed => 11,
            Self::Credits => 12,
            Self::ElapsedTime => 13,
            Self::MissionTimerExpired => 14,
            Self::BuildingsDestroyedCount => 15,
            Self::UnitsDestroyedCount => 16,
            Self::NoFactories => 17,
            Self::EvacCivilian => 18,
            Self::Build => 19,
            Self::BuildUnit => 20,
            Self::BuildInfantry => 21,
            Self::BuildAircraft => 22,
            Self::LeavesMap => 23,
            Self::EntersZone => 24,
            Self::CrossHorizontal => 25,
            Self::CrossVertical => 26,
            Self::GlobalSet => 27,
            Self::GlobalClear => 28,
            Self::FakesDestroyed => 29,
            Self::LowPower => 30,
            Self::AllBridgesDestroyed => 31,
            Self::BuildingExists => 32,
            Self::Unknown(code) => code,
        }
    }

    /// The original scenario editor caption for this event (`EventText`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Unknown(_) => "unknown event",
            kind => EVENT_NAMES[kind.code() as usize],
        }
    }

    /// What the record's `value` field means for this event (`Event_Needs`).
    pub fn param_kind(self) -> ParamKind {
        match self {
            Self::Thieved
            | Self::PlayerEntered
            | Self::CrossHorizontal
            | Self::CrossVertical
            | Self::EntersZone
            | Self::HouseDiscovered
            | Self::BuildingsDestroyed
            | Self::UnitsDestroyed
            | Self::AllDestroyed
            | Self::LowPower => ParamKind::House,
            Self::UnitsDestroyedCount
            | Self::BuildingsDestroyedCount
            | Self::Credits
            | Self::ElapsedTime
            | Self::GlobalSet
            | Self::GlobalClear => ParamKind::Number,
            Self::BuildingExists | Self::Build => ParamKind::Structure,
            Self::BuildUnit => ParamKind::Unit,
            Self::BuildInfantry => ParamKind::Infantry,
            Self::BuildAircraft => ParamKind::Aircraft,
            Self::LeavesMap => ParamKind::Team,
            Self::None
            | Self::Spied
            | Self::Discovered
            | Self::Attacked
            | Self::Destroyed
            | Self::Any
            | Self::MissionTimerExpired
            | Self::NoFactories
            | Self::EvacCivilian
            | Self::FakesDestroyed
            | Self::AllBridgesDestroyed
            | Self::Unknown(_) => ParamKind::None,
        }
    }
}

/// A trigger action code (`TActionType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionKind {
    /// `TACTION_NONE` (0).
    None,
    /// `TACTION_WIN` (1).
    Win,
    /// `TACTION_LOSE` (2).
    Lose,
    /// `TACTION_BEGIN_PRODUCTION` (3).
    BeginProduction,
    /// `TACTION_CREATE_TEAM` (4).
    CreateTeam,
    /// `TACTION_DESTROY_TEAM` (5).
    DestroyTeam,
    /// `TACTION_ALL_HUNT` (6).
    AllHunt,
    /// `TACTION_REINFORCEMENTS` (7).
    Reinforcements,
    /// `TACTION_DZ` (8).
    DropZone,
    /// `TACTION_FIRE_SALE` (9).
    FireSale,
    /// `TACTION_PLAY_MOVIE` (10).
    PlayMovie,
    /// `TACTION_TEXT_TRIGGER` (11).
    TextTrigger,
    /// `TACTION_DESTROY_TRIGGER` (12).
    DestroyTrigger,
    /// `TACTION_AUTOCREATE` (13).
    Autocreate,
    /// `TACTION_WINLOSE` (14).
    WinLose,
    /// `TACTION_ALLOWWIN` (15).
    AllowWin,
    /// `TACTION_REVEAL_ALL` (16).
    RevealAll,
    /// `TACTION_REVEAL_SOME` (17).
    RevealSome,
    /// `TACTION_REVEAL_ZONE` (18).
    RevealZone,
    /// `TACTION_PLAY_SOUND` (19).
    PlaySound,
    /// `TACTION_PLAY_MUSIC` (20).
    PlayMusic,
    /// `TACTION_PLAY_SPEECH` (21).
    PlaySpeech,
    /// `TACTION_FORCE_TRIGGER` (22).
    ForceTrigger,
    /// `TACTION_START_TIMER` (23).
    StartTimer,
    /// `TACTION_STOP_TIMER` (24).
    StopTimer,
    /// `TACTION_ADD_TIMER` (25).
    ExtendTimer,
    /// `TACTION_SUB_TIMER` (26).
    ShortenTimer,
    /// `TACTION_SET_TIMER` (27).
    SetTimer,
    /// `TACTION_SET_GLOBAL` (28).
    SetGlobal,
    /// `TACTION_CLEAR_GLOBAL` (29).
    ClearGlobal,
    /// `TACTION_BASE_BUILDING` (30).
    BaseBuilding,
    /// `TACTION_CREEP_SHADOW` (31).
    CreepShadow,
    /// `TACTION_DESTROY_OBJECT` (32).
    DestroyObject,
    /// `TACTION_1_SPECIAL` (33).
    OneTimeSpecial,
    /// `TACTION_FULL_SPECIAL` (34).
    FullSpecial,
    /// `TACTION_PREFERRED_TARGET` (35).
    PreferredTarget,
    /// `TACTION_LAUNCH_NUKES` (36).
    LaunchNukes,
    /// A code outside the enumeration; preserved verbatim.
    Unknown(u32),
}

impl ActionKind {
    /// Resolve a numeric record code, keeping unknown codes verbatim.
    pub fn from_code(code: u32) -> Self {
        match code {
            0 => Self::None,
            1 => Self::Win,
            2 => Self::Lose,
            3 => Self::BeginProduction,
            4 => Self::CreateTeam,
            5 => Self::DestroyTeam,
            6 => Self::AllHunt,
            7 => Self::Reinforcements,
            8 => Self::DropZone,
            9 => Self::FireSale,
            10 => Self::PlayMovie,
            11 => Self::TextTrigger,
            12 => Self::DestroyTrigger,
            13 => Self::Autocreate,
            14 => Self::WinLose,
            15 => Self::AllowWin,
            16 => Self::RevealAll,
            17 => Self::RevealSome,
            18 => Self::RevealZone,
            19 => Self::PlaySound,
            20 => Self::PlayMusic,
            21 => Self::PlaySpeech,
            22 => Self::ForceTrigger,
            23 => Self::StartTimer,
            24 => Self::StopTimer,
            25 => Self::ExtendTimer,
            26 => Self::ShortenTimer,
            27 => Self::SetTimer,
            28 => Self::SetGlobal,
            29 => Self::ClearGlobal,
            30 => Self::BaseBuilding,
            31 => Self::CreepShadow,
            32 => Self::DestroyObject,
            33 => Self::OneTimeSpecial,
            34 => Self::FullSpecial,
            35 => Self::PreferredTarget,
            36 => Self::LaunchNukes,
            other => Self::Unknown(other),
        }
    }

    /// The numeric code this kind serialises to.
    pub fn code(self) -> u32 {
        match self {
            Self::None => 0,
            Self::Win => 1,
            Self::Lose => 2,
            Self::BeginProduction => 3,
            Self::CreateTeam => 4,
            Self::DestroyTeam => 5,
            Self::AllHunt => 6,
            Self::Reinforcements => 7,
            Self::DropZone => 8,
            Self::FireSale => 9,
            Self::PlayMovie => 10,
            Self::TextTrigger => 11,
            Self::DestroyTrigger => 12,
            Self::Autocreate => 13,
            Self::WinLose => 14,
            Self::AllowWin => 15,
            Self::RevealAll => 16,
            Self::RevealSome => 17,
            Self::RevealZone => 18,
            Self::PlaySound => 19,
            Self::PlayMusic => 20,
            Self::PlaySpeech => 21,
            Self::ForceTrigger => 22,
            Self::StartTimer => 23,
            Self::StopTimer => 24,
            Self::ExtendTimer => 25,
            Self::ShortenTimer => 26,
            Self::SetTimer => 27,
            Self::SetGlobal => 28,
            Self::ClearGlobal => 29,
            Self::BaseBuilding => 30,
            Self::CreepShadow => 31,
            Self::DestroyObject => 32,
            Self::OneTimeSpecial => 33,
            Self::FullSpecial => 34,
            Self::PreferredTarget => 35,
            Self::LaunchNukes => 36,
            Self::Unknown(code) => code,
        }
    }

    /// The original scenario editor caption for this action (`ActionText`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Unknown(_) => "unknown action",
            kind => ACTION_NAMES[kind.code() as usize],
        }
    }

    /// What the record's `value` field means for this action (`Action_Needs`).
    pub fn param_kind(self) -> ParamKind {
        match self {
            Self::OneTimeSpecial | Self::FullSpecial => ParamKind::SpecialWeapon,
            Self::FireSale
            | Self::Win
            | Self::Lose
            | Self::AllHunt
            | Self::BeginProduction
            | Self::Autocreate => ParamKind::House,
            Self::BaseBuilding => ParamKind::Bool,
            Self::CreateTeam | Self::DestroyTeam | Self::Reinforcements => ParamKind::Team,
            Self::ForceTrigger | Self::DestroyTrigger => ParamKind::Trigger,
            Self::DropZone | Self::RevealSome | Self::RevealZone => ParamKind::Waypoint,
            Self::PlayMusic => ParamKind::Theme,
            Self::PlayMovie => ParamKind::Movie,
            Self::PlaySound => ParamKind::Sound,
            Self::PlaySpeech => ParamKind::Speech,
            Self::TextTrigger
            | Self::ExtendTimer
            | Self::ShortenTimer
            | Self::SetTimer
            | Self::SetGlobal
            | Self::ClearGlobal => ParamKind::Number,
            Self::PreferredTarget => ParamKind::Quarry,
            Self::None
            | Self::WinLose
            | Self::AllowWin
            | Self::RevealAll
            | Self::StartTimer
            | Self::StopTimer
            | Self::CreepShadow
            | Self::DestroyObject
            | Self::LaunchNukes
            | Self::Unknown(_) => ParamKind::None,
        }
    }
}

/// A parsed `TEventClass` record: `<event>,<team logical id>,<value>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventRecord {
    /// Resolved event code; `Unknown` for codes past the enumeration.
    pub kind: EventKind,
    /// `TeamTypes.Logical_ID(Team)`; `None` for the engine's `-1`.
    pub team: Option<u16>,
    /// `Data.Value`, typed through [`EventRecord::payload`].
    pub value: i64,
}

impl EventRecord {
    /// Parse one event record string.
    pub fn parse(record: &str) -> Result<Self, TriggerRecordError> {
        let trimmed = record.trim();
        if trimmed.is_empty() {
            return Err(TriggerRecordError::Empty);
        }
        let fields: Vec<&str> = trimmed.split(',').map(str::trim).collect();
        if fields.len() != 3 {
            return Err(field_count_error(
                trimmed,
                &fields,
                "3 (`<event>,<team>,<value>`)",
            ));
        }
        let code_token = fields.first().copied().unwrap_or_default();
        let kind = parse_kind(code_token, trimmed, "event code")?;
        let team = parse_reference(
            fields.get(1).copied().unwrap_or_default(),
            code_token,
            trimmed,
        )?;
        let value = parse_int(
            fields.get(2).copied().unwrap_or_default(),
            code_token,
            trimmed,
            "value",
        )?;
        Ok(Self { kind, team, value })
    }

    /// The typed payload of this record.
    pub fn payload(self) -> ParamValue {
        self.kind.param_kind().wrap(self.value)
    }

    /// Serialise back to the original record string.
    pub fn to_record_string(self) -> String {
        format!(
            "{},{},{}",
            self.kind.code(),
            reference_to_int(self.team),
            self.value
        )
    }
}

impl fmt::Display for EventRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_record_string())
    }
}

/// A parsed `TActionClass` record:
/// `<action>,<team logical id>,<trigger logical id>,<value>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionRecord {
    /// Resolved action code; `Unknown` for codes past the enumeration.
    pub kind: ActionKind,
    /// `TeamTypes.Logical_ID(Team)`; `None` for the engine's `-1`.
    pub team: Option<u16>,
    /// `TriggerTypes.Logical_ID(Trigger)`; `None` for the engine's `-1`.
    pub trigger: Option<u16>,
    /// `Data.Value`, typed through [`ActionRecord::payload`].
    pub value: i64,
}

impl ActionRecord {
    /// Parse one action record string.
    pub fn parse(record: &str) -> Result<Self, TriggerRecordError> {
        let trimmed = record.trim();
        if trimmed.is_empty() {
            return Err(TriggerRecordError::Empty);
        }
        let fields: Vec<&str> = trimmed.split(',').map(str::trim).collect();
        if fields.len() != 4 {
            return Err(field_count_error(
                trimmed,
                &fields,
                "4 (`<action>,<team>,<trigger>,<value>`)",
            ));
        }
        let code_token = fields.first().copied().unwrap_or_default();
        let kind = parse_action_kind(code_token, trimmed)?;
        let team = parse_reference(
            fields.get(1).copied().unwrap_or_default(),
            code_token,
            trimmed,
        )?;
        let trigger = parse_reference(
            fields.get(2).copied().unwrap_or_default(),
            code_token,
            trimmed,
        )?;
        let value = parse_int(
            fields.get(3).copied().unwrap_or_default(),
            code_token,
            trimmed,
            "value",
        )?;
        Ok(Self {
            kind,
            team,
            trigger,
            value,
        })
    }

    /// The typed payload of this record.
    pub fn payload(self) -> ParamValue {
        self.kind.param_kind().wrap(self.value)
    }

    /// Serialise back to the original record string.
    pub fn to_record_string(self) -> String {
        format!(
            "{},{},{},{}",
            self.kind.code(),
            reference_to_int(self.team),
            reference_to_int(self.trigger),
            self.value
        )
    }
}

impl fmt::Display for ActionRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_record_string())
    }
}

/// Either original record shape, dispatched on the field count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerRecord {
    /// A `TEventClass` record (3 fields).
    Event(EventRecord),
    /// A `TActionClass` record (4 fields).
    Action(ActionRecord),
}

impl TriggerRecord {
    /// Parse either record shape: 3 fields is an event, 4 fields is an action.
    pub fn parse(record: &str) -> Result<Self, TriggerRecordError> {
        let trimmed = record.trim();
        if trimmed.is_empty() {
            return Err(TriggerRecordError::Empty);
        }
        let fields: Vec<&str> = trimmed.split(',').map(str::trim).collect();
        match fields.len() {
            3 => EventRecord::parse(trimmed).map(Self::Event),
            4 => ActionRecord::parse(trimmed).map(Self::Action),
            found => Err(TriggerRecordError::FieldCount {
                record: trimmed.to_string(),
                code: fields.first().copied().unwrap_or_default().to_string(),
                found,
                expected: "3 (`<event>,<team>,<value>`) or 4 (`<action>,<team>,<trigger>,<value>`)",
            }),
        }
    }

    /// Serialise back to the original record string.
    pub fn to_record_string(self) -> String {
        match self {
            Self::Event(record) => record.to_record_string(),
            Self::Action(record) => record.to_record_string(),
        }
    }
}

impl fmt::Display for TriggerRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_record_string())
    }
}

/// `-1` is the engine's null-reference sentinel (`Logical_ID`).
fn reference_to_int(reference: Option<u16>) -> i64 {
    reference.map(i64::from).unwrap_or(-1)
}

fn field_count_error(record: &str, fields: &[&str], expected: &'static str) -> TriggerRecordError {
    TriggerRecordError::FieldCount {
        record: record.to_string(),
        code: fields.first().copied().unwrap_or_default().to_string(),
        found: fields.len(),
        expected,
    }
}

fn parse_int(
    field: &str,
    code: &str,
    record: &str,
    name: &'static str,
) -> Result<i64, TriggerRecordError> {
    field
        .parse::<i64>()
        .map_err(|_| TriggerRecordError::InvalidNumber {
            record: record.to_string(),
            code: code.to_string(),
            field: name,
            value: field.to_string(),
        })
}

fn parse_kind(
    code: &str,
    record: &str,
    name: &'static str,
) -> Result<EventKind, TriggerRecordError> {
    let value = parse_int(code, code, record, name)?;
    match u32::try_from(value) {
        Ok(code) => Ok(EventKind::from_code(code)),
        Err(_) => Err(TriggerRecordError::InvalidNumber {
            record: record.to_string(),
            code: code.to_string(),
            field: name,
            value: code.to_string(),
        }),
    }
}

fn parse_action_kind(code: &str, record: &str) -> Result<ActionKind, TriggerRecordError> {
    let value = parse_int(code, code, record, "action code")?;
    match u32::try_from(value) {
        Ok(code) => Ok(ActionKind::from_code(code)),
        Err(_) => Err(TriggerRecordError::InvalidNumber {
            record: record.to_string(),
            code: code.to_string(),
            field: "action code",
            value: code.to_string(),
        }),
    }
}

fn parse_reference(
    field: &str,
    code: &str,
    record: &str,
) -> Result<Option<u16>, TriggerRecordError> {
    let value = parse_int(field, code, record, "reference")?;
    if value == -1 {
        return Ok(None);
    }
    match u16::try_from(value) {
        Ok(reference) => Ok(Some(reference)),
        Err(_) => Err(TriggerRecordError::InvalidNumber {
            record: record.to_string(),
            code: code.to_string(),
            field: "reference",
            value: field.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_enumerated_event_code_resolves_and_names() {
        for code in 0..EVENT_CODE_COUNT {
            let kind = EventKind::from_code(code);
            assert_eq!(kind.code(), code, "event code {code} changed");
            assert!(!kind.name().is_empty(), "event code {code} has no name");
            assert_ne!(kind.name(), "unknown event", "event code {code} unresolved");
        }
        assert_eq!(
            EventKind::from_code(EVENT_CODE_COUNT),
            EventKind::Unknown(EVENT_CODE_COUNT)
        );
        assert_eq!(EventKind::from_code(999).code(), 999);
    }

    #[test]
    fn every_enumerated_action_code_resolves_and_names() {
        for code in 0..ACTION_CODE_COUNT {
            let kind = ActionKind::from_code(code);
            assert_eq!(kind.code(), code, "action code {code} changed");
            assert!(!kind.name().is_empty(), "action code {code} has no name");
            assert_ne!(
                kind.name(),
                "unknown action",
                "action code {code} unresolved"
            );
        }
        assert_eq!(
            ActionKind::from_code(ACTION_CODE_COUNT),
            ActionKind::Unknown(ACTION_CODE_COUNT)
        );
        assert_eq!(ActionKind::from_code(4096).code(), 4096);
    }

    #[test]
    fn event_record_with_params_parses_typed_payload() {
        let record = EventRecord::parse("12,3,500").expect("credits event parses");
        assert_eq!(record.kind, EventKind::Credits);
        assert_eq!(record.kind.param_kind(), ParamKind::Number);
        assert_eq!(record.team, Some(3));
        assert_eq!(record.value, 500);
        assert_eq!(record.payload(), ParamValue::Number(500));

        let padded = EventRecord::parse("  13 , 0 , 25 ").expect("whitespace tolerated");
        assert_eq!(padded.kind, EventKind::ElapsedTime);
        assert_eq!(padded.to_record_string(), "13,0,25");
    }

    #[test]
    fn event_record_house_and_type_references_are_typed() {
        let house = EventRecord::parse("1,4,7").expect("player entered parses");
        assert_eq!(house.kind, EventKind::PlayerEntered);
        assert_eq!(house.payload(), ParamValue::House(7));

        let structure = EventRecord::parse("19,0,31").expect("build event parses");
        assert_eq!(structure.payload(), ParamValue::Structure(31));

        let infantry = EventRecord::parse("21,0,4").expect("build infantry parses");
        assert_eq!(infantry.payload(), ParamValue::Infantry(4));

        let team = EventRecord::parse("23,5,0").expect("leaves map parses");
        assert_eq!(team.team, Some(5));
        assert_eq!(team.kind.param_kind(), ParamKind::Team);
        assert_eq!(team.payload(), ParamValue::Team(0));
    }

    #[test]
    fn action_record_with_params_parses_typed_payload() {
        let record = ActionRecord::parse("27,2,8,40").expect("set timer action parses");
        assert_eq!(record.kind, ActionKind::SetTimer);
        assert_eq!(record.kind.param_kind(), ParamKind::Number);
        assert_eq!(record.team, Some(2));
        assert_eq!(record.trigger, Some(8));
        assert_eq!(record.value, 40);
        assert_eq!(record.payload(), ParamValue::Number(40));

        let sound = ActionRecord::parse("19,-1,-1,12").expect("play sound parses");
        assert_eq!(sound.team, None);
        assert_eq!(sound.trigger, None);
        assert_eq!(sound.payload(), ParamValue::Sound(12));
    }

    #[test]
    fn action_record_reference_bearing_record_round_trips() {
        let record = ActionRecord::parse("12,-1,5,0").expect("destroy trigger parses");
        assert_eq!(record.kind, ActionKind::DestroyTrigger);
        assert_eq!(record.team, None);
        assert_eq!(record.trigger, Some(5));
        assert_eq!(record.payload(), ParamValue::Trigger(0));
        assert_eq!(record.to_record_string(), "12,-1,5,0");

        let reveal = ActionRecord::parse("17,0,1,9").expect("reveal some parses");
        assert_eq!(reveal.payload(), ParamValue::Waypoint(9));
        assert_eq!(reveal.to_record_string(), "17,0,1,9");
    }

    #[test]
    fn event_record_round_trips_for_every_code() {
        for code in 0..EVENT_CODE_COUNT {
            for team in [-1_i64, 0, 3, 65535] {
                for value in [-1_i64, 0, 500, 123_456] {
                    let text = format!("{code},{team},{value}");
                    let parsed = EventRecord::parse(&text).expect("canonical event parses");
                    assert_eq!(parsed.to_record_string(), text);
                }
            }
        }
    }

    #[test]
    fn action_record_round_trips_for_every_code() {
        for code in 0..ACTION_CODE_COUNT {
            for (team, trigger) in [(-1_i64, -1_i64), (0, 0), (3, 4), (65535, 65534)] {
                for value in [-1_i64, 0, 40] {
                    let text = format!("{code},{team},{trigger},{value}");
                    let parsed = ActionRecord::parse(&text).expect("canonical action parses");
                    assert_eq!(parsed.to_record_string(), text);
                }
            }
        }
    }

    #[test]
    fn unknown_codes_pass_through_and_round_trip() {
        let event = EventRecord::parse("999,1,2").expect("unknown event code is kept");
        assert_eq!(event.kind, EventKind::Unknown(999));
        assert_eq!(event.kind.name(), "unknown event");
        assert_eq!(event.kind.param_kind(), ParamKind::None);
        assert_eq!(event.to_record_string(), "999,1,2");

        let action = ActionRecord::parse("4242,-1,7,3").expect("unknown action code is kept");
        assert_eq!(action.kind, ActionKind::Unknown(4242));
        assert_eq!(action.to_record_string(), "4242,-1,7,3");

        let dispatched = TriggerRecord::parse("4242,-1,7,3").expect("dispatch keeps unknowns");
        assert_eq!(dispatched.to_record_string(), "4242,-1,7,3");
    }

    #[test]
    fn truncated_event_record_errors_with_code() {
        let error = EventRecord::parse("12,3").expect_err("two fields are not an event");
        assert_eq!(error.code(), Some("12"));
        assert!(error.to_string().contains("expected 3"));

        let single = EventRecord::parse("7").expect_err("one field is not an event");
        assert_eq!(single.code(), Some("7"));

        let long = EventRecord::parse("12,3,500,1").expect_err("four fields are not an event");
        assert_eq!(long.code(), Some("12"));
    }

    #[test]
    fn truncated_action_record_errors_with_code() {
        let error = ActionRecord::parse("27,2").expect_err("two fields are not an action");
        assert_eq!(error.code(), Some("27"));

        let long = ActionRecord::parse("27,2,8,40,1").expect_err("five fields are not an action");
        assert_eq!(long.code(), Some("27"));
    }

    #[test]
    fn garbage_parameters_error_with_code() {
        let value = EventRecord::parse("12,3,twelve").expect_err("non-numeric value rejected");
        assert_eq!(value.code(), Some("12"));
        assert!(value.to_string().contains("value"));

        let code = EventRecord::parse("credits,3,500").expect_err("non-numeric code rejected");
        assert_eq!(code.code(), Some("credits"));

        let reference = EventRecord::parse("12,99999,500").expect_err("reference out of range");
        assert_eq!(reference.code(), Some("12"));

        let negative = EventRecord::parse("12,-2,500").expect_err("only -1 means null");
        assert_eq!(negative.code(), Some("12"));
    }

    #[test]
    fn empty_record_is_an_error_without_a_code() {
        assert_eq!(
            EventRecord::parse("").expect_err("empty"),
            TriggerRecordError::Empty
        );
        assert_eq!(
            ActionRecord::parse("   ").expect_err("blank"),
            TriggerRecordError::Empty
        );
        assert_eq!(
            TriggerRecord::parse("").expect_err("empty"),
            TriggerRecordError::Empty
        );
        assert_eq!(TriggerRecordError::Empty.code(), None);
    }

    #[test]
    fn trigger_record_dispatch_parses_both_shapes() {
        let event = TriggerRecord::parse("12,3,500").expect("event dispatch");
        assert!(matches!(event, TriggerRecord::Event(_)));
        assert_eq!(event.to_record_string(), "12,3,500");

        let action = TriggerRecord::parse("27,2,8,40").expect("action dispatch");
        assert!(matches!(action, TriggerRecord::Action(_)));
        assert_eq!(action.to_record_string(), "27,2,8,40");

        let bad = TriggerRecord::parse("27,2").expect_err("two fields match no shape");
        assert_eq!(bad.code(), Some("27"));
        assert!(bad.to_string().contains("expected 3"));
    }

    #[test]
    fn param_kind_tables_follow_the_header_need_tables() {
        let house_events = [
            EventKind::PlayerEntered,
            EventKind::Thieved,
            EventKind::HouseDiscovered,
            EventKind::UnitsDestroyed,
            EventKind::BuildingsDestroyed,
            EventKind::AllDestroyed,
            EventKind::EntersZone,
            EventKind::CrossHorizontal,
            EventKind::CrossVertical,
            EventKind::LowPower,
        ];
        for kind in house_events {
            assert_eq!(kind.param_kind(), ParamKind::House, "{kind:?}");
        }
        let number_events = [
            EventKind::Credits,
            EventKind::ElapsedTime,
            EventKind::BuildingsDestroyedCount,
            EventKind::UnitsDestroyedCount,
            EventKind::GlobalSet,
            EventKind::GlobalClear,
        ];
        for kind in number_events {
            assert_eq!(kind.param_kind(), ParamKind::Number, "{kind:?}");
        }
        assert_eq!(EventKind::Build.param_kind(), ParamKind::Structure);
        assert_eq!(EventKind::BuildingExists.param_kind(), ParamKind::Structure);
        assert_eq!(EventKind::BuildUnit.param_kind(), ParamKind::Unit);
        assert_eq!(EventKind::BuildInfantry.param_kind(), ParamKind::Infantry);
        assert_eq!(EventKind::BuildAircraft.param_kind(), ParamKind::Aircraft);
        assert_eq!(EventKind::LeavesMap.param_kind(), ParamKind::Team);
        assert_eq!(EventKind::MissionTimerExpired.param_kind(), ParamKind::None);
        assert_eq!(EventKind::Unknown(77).param_kind(), ParamKind::None);

        for kind in [ActionKind::OneTimeSpecial, ActionKind::FullSpecial] {
            assert_eq!(kind.param_kind(), ParamKind::SpecialWeapon, "{kind:?}");
        }
        for kind in [
            ActionKind::Win,
            ActionKind::Lose,
            ActionKind::BeginProduction,
            ActionKind::AllHunt,
            ActionKind::FireSale,
            ActionKind::Autocreate,
        ] {
            assert_eq!(kind.param_kind(), ParamKind::House, "{kind:?}");
        }
        for kind in [
            ActionKind::CreateTeam,
            ActionKind::DestroyTeam,
            ActionKind::Reinforcements,
        ] {
            assert_eq!(kind.param_kind(), ParamKind::Team, "{kind:?}");
        }
        for kind in [ActionKind::ForceTrigger, ActionKind::DestroyTrigger] {
            assert_eq!(kind.param_kind(), ParamKind::Trigger, "{kind:?}");
        }
        for kind in [
            ActionKind::DropZone,
            ActionKind::RevealSome,
            ActionKind::RevealZone,
        ] {
            assert_eq!(kind.param_kind(), ParamKind::Waypoint, "{kind:?}");
        }
        assert_eq!(ActionKind::PlayMusic.param_kind(), ParamKind::Theme);
        assert_eq!(ActionKind::PlayMovie.param_kind(), ParamKind::Movie);
        assert_eq!(ActionKind::PlaySound.param_kind(), ParamKind::Sound);
        assert_eq!(ActionKind::PlaySpeech.param_kind(), ParamKind::Speech);
        assert_eq!(ActionKind::BaseBuilding.param_kind(), ParamKind::Bool);
        assert_eq!(ActionKind::PreferredTarget.param_kind(), ParamKind::Quarry);
        for kind in [
            ActionKind::TextTrigger,
            ActionKind::ExtendTimer,
            ActionKind::ShortenTimer,
            ActionKind::SetTimer,
            ActionKind::SetGlobal,
            ActionKind::ClearGlobal,
        ] {
            assert_eq!(kind.param_kind(), ParamKind::Number, "{kind:?}");
        }
        for kind in [
            ActionKind::None,
            ActionKind::WinLose,
            ActionKind::AllowWin,
            ActionKind::RevealAll,
            ActionKind::StartTimer,
            ActionKind::StopTimer,
            ActionKind::CreepShadow,
            ActionKind::DestroyObject,
            ActionKind::LaunchNukes,
        ] {
            assert_eq!(kind.param_kind(), ParamKind::None, "{kind:?}");
        }
    }

    #[test]
    fn names_match_the_original_editor_tables() {
        assert_eq!(EventKind::None.name(), "-No Event-");
        assert_eq!(EventKind::PlayerEntered.name(), "Entered by...");
        assert_eq!(EventKind::Credits.name(), "Credits exceed (x100)...");
        assert_eq!(EventKind::BuildingExists.name(), "Building exists...");
        assert_eq!(ActionKind::None.name(), "-No Action-");
        assert_eq!(ActionKind::Win.name(), "Winner is...");
        assert_eq!(ActionKind::WinLose.name(), "~don't use~");
        assert_eq!(ActionKind::LaunchNukes.name(), "Launch Nukes");
    }

    #[test]
    fn payload_without_parameters_is_none() {
        let record = EventRecord::parse("8,0,123").expect("any event parses");
        assert_eq!(record.payload(), ParamValue::None);
        assert_eq!(record.payload().raw(), None);
        assert_eq!(record.to_record_string(), "8,0,123");

        let action = ActionRecord::parse("16,-1,-1,0").expect("reveal all parses");
        assert_eq!(action.payload(), ParamValue::None);
    }
    // ---- TODO 9: full TEVENT (33) + TACTION (37) vocabulary pinned by name ----
    #[test]
    fn ra_eventtable_complete() {
        let names = [
            "-No Event-",
            "Entered by...",
            "Spied by...",
            "Thieved by...",
            "Discovered by player",
            "House Discovered...",
            "Attacked by anybody",
            "Destroyed by anybody",
            "Any Event",
            "Destroyed, Units, All...",
            "Destroyed, Buildings, All...",
            "Destroyed, All...",
            "Credits exceed (x100)...",
            "Elapsed Time (1/10th min)...",
            "Mission Timer Expired",
            "Destroyed, Buildings, #...",
            "Destroyed, Units, #...",
            "No Factories left",
            "Civilians Evacuated",
            "Build Building Type...",
            "Build Unit Type...",
            "Build Infantry Type...",
            "Build Aircraft Type...",
            "Leaves map (team)...",
            "Zone Entry by...",
            "Crosses Horizontal Line...",
            "Crosses Vertical Line...",
            "Global is set...",
            "Global is clear...",
            "Destroyed, Fakes, All...",
            "Low Power...",
            "All bridges destroyed",
            "Building exists...",
        ];
        assert_eq!(names.len(), 33, "RA EventText must have 33 rows");
        for (i, n) in names.iter().enumerate() {
            let k = EventKind::from_code(i as u32);
            assert_eq!(k.name(), *n, "RA event row {} name mismatch", i);
        }
    }

    #[test]
    fn ra_actiontable_complete() {
        let names = [
            "-No Action-",
            "Winner is...",
            "Loser is...",
            "Production Begins",
            "Create Team...",
            "Destroy All Teams",
            "All to Hunt...",
            "Reinforcement (team)...",
            "Drop Zone Flare (waypoint)...",
            "Fire Sale...",
            "Play Movie...",
            "Text Trigger (ID num)...",
            "Destroy Trigger...",
            "Autocreate Begins...",
            "~don't use~",
            "Allow Win",
            "Reveal all map",
            "Reveal around waypoint...",
            "Reveal zone of waypoint...",
            "Play sound effect...",
            "Play music theme...",
            "Play speech...",
            "Force Trigger...",
            "Timer Start",
            "Timer Stop",
            "Timer Extend (1/10th min)...",
            "Timer Shorten (1/10th min)...",
            "Timer Set (1/10th min)...",
            "Global Set...",
            "Global Clear...",
            "Auto Base Building...",
            "Grow shroud one 'step'",
            "Destroy attached building",
            "Add 1-time special weapon...",
            "Add repeating special weapon...",
            "Preferred target...",
            "Launch Nukes",
        ];
        assert_eq!(names.len(), 37, "RA ActionText must have 37 rows");
        for (i, n) in names.iter().enumerate() {
            let k = ActionKind::from_code(i as u32);
            assert_eq!(k.name(), *n, "RA action row {} name mismatch", i);
        }
    }

    #[test]
    fn td_eventtable_complete() {
        let td = [
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
        assert_eq!(td.len(), 17, "TD declares 17 event rows");
        for (i, n) in td.iter().enumerate() {
            assert!(!n.is_empty(), "TD row {} empty", i);
        }
    }

    #[test]
    fn td_unmapped_documented() {
        let mapped = [
            "None",
            "Credits",
            "Time",
            "Units Destr.",
            "Bldgs Destr.",
            "All Destr.",
            "No Factories",
            "Built It",
        ];
        let declared = 17usize;
        assert!(
            mapped.len() < declared,
            "expected partial TD mapping (<{} mapped)",
            declared
        );
    }
}
