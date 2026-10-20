//! Red Alert EVA / VOX speech vocabulary.
//!
//! Red Alert has its own `VoxType` enum and its own `Speech[]` file table,
//! distinct from Tiberian Dawn's ([crate::eva]). Everything below is
//! transcribed from `REDALERT/DEFINES.H` (`VOX_*`) and
//! `REDALERT/AUDIO.CPP` (`Speech[]`), slot for slot: the 116 compiled table
//! slots `0..116`, where the seven slots the game fills with the literal
//! string `"none"` are listed in [`RA_SILENT_SLOTS`] instead of in the
//! playable enum.
//!
//! Slot ids are kept explicit because the silent slots leave gaps: a line's
//! [`RaEvaLine::voice_id`] always equals its index in the original
//! `Speech[]` table.

use crate::speech::{EvaVocabulary, SpeechCategory, SpeechQueue};

/// Red Alert direction used by arrival and flare announcements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RaDirection {
    /// North.
    North,
    /// South.
    South,
    /// East.
    East,
    /// West.
    West,
}

/// Red Alert unit upgrade announcement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RaUpgrade {
    /// Armour upgrade.
    Armor,
    /// Firepower upgrade.
    Firepower,
    /// Speed upgrade.
    Speed,
}

/// One Red Alert EVA speech event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RaEvaLine {
    /// Slot 0: `MISNWON1` (mission).
    MissionAccomplished,
    /// Slot 1: `MISNLST1` (mission).
    MissionFailed,
    /// Slot 2: `PROGRES1` (production).
    UnableToComply,
    /// Slot 3: `CONSCMP1` (production).
    ConstructionComplete,
    /// Slot 4: `UNITRDY1` (production).
    UnitReady,
    /// Slot 5: `NEWOPT1` (production).
    NewConstructionOptions,
    /// Slot 6: `NODEPLY1` (production).
    CannotDeployHere,
    /// Slot 7: `STRCKIL1` (combat).
    StructureDestroyed,
    /// Slot 8: `NOPOWR1` (power).
    InsufficientPower,
    /// Slot 9: `NOFUNDS1` (economy).
    InsufficientFunds,
    /// Slot 10: `BCT1` (mission).
    BattleControlTerminated,
    /// Slot 11: `REINFOR1` (info).
    ReinforcementsArrived,
    /// Slot 12: `CANCLD1` (info).
    Canceled,
    /// Slot 13: `ABLDGIN1` (production).
    Building,
    /// Slot 14: `LOPOWER1` (power).
    LowPower,
    /// Slot 15: `NOFUNDS1` (economy).
    NeedMoreFunds,
    /// Slot 16: `BASEATK1` (baseattack).
    BaseUnderAttack,
    /// Slot 17: `NOBUILD1` (production).
    UnableToBuildMore,
    /// Slot 18: `PRIBLDG1` (production).
    PrimaryBuildingSelected,
    /// Slot 19: `TANK01` (superweapon).
    MadTankDeployed,
    /// Slot 21: `UNITLST1` (combat).
    UnitLost,
    /// Slot 22: `SLCTTGT1` (info).
    SelectTarget,
    /// Slot 23: `ENMYAPP1` (warning).
    EnemyApproaching,
    /// Slot 24: `SILOND1` (economy).
    SilosNeeded,
    /// Slot 25: `ONHOLD1` (info).
    OnHold,
    /// Slot 26: `REPAIR1` (info).
    Repairing,
    /// Slot 29: `AUNITL1` (combat).
    AircraftLost,
    /// Slot 31: `AAPPRO1` (warning).
    AlliedForcesApproaching,
    /// Slot 32: `AARRIVE1` (info).
    AlliedForcesArrived,
    /// Slot 35: `BLDGINF1` (intel).
    BuildingInfiltrated,
    /// Slot 36: `CHROCHR1` (superweapon).
    ChronosphereCharging,
    /// Slot 37: `CHRORDY1` (superweapon).
    ChronosphereReady,
    /// Slot 38: `CHROYES1` (superweapon).
    ChronosphereTest,
    /// Slot 39: `CMDCNTR1` (baseattack).
    CommandCenterUnderAttack,
    /// Slot 40: `CNTLDED1` (intel).
    ControlCenterDeactivated,
    /// Slot 41: `CONVYAP1` (warning).
    ConvoyApproaching,
    /// Slot 42: `CONVLST1` (combat).
    ConvoyUnitLost,
    /// Slot 43: `XPLOPLC1` (intel).
    ExplosivePlaced,
    /// Slot 44: `CREDIT1` (intel).
    MoneyStolen,
    /// Slot 45: `NAVYLST1` (combat).
    ShipLost,
    /// Slot 46: `SATLNCH1` (intel).
    SatelliteLaunched,
    /// Slot 47: `PULSE1` (intel).
    SonarAvailable,
    /// Slot 49: `SOVFAPP1` (warning).
    SovietForcesApproaching,
    /// Slot 50: `SOVREIN1` (info).
    SovietReinforcementsArrived,
    /// Slot 51: `TRAIN1` (info).
    Training,
    /// Slot 52: `AREADY1` (superweapon).
    AtomicBombReady,
    /// Slot 53: `ALAUNCH1` (superweapon).
    AtomicBombLaunched,
    /// Slot 54: `AARRIVN1` (info).
    AlliedForcesNorth,
    /// Slot 55: `AARRIVS1` (info).
    AlliedForcesSouth,
    /// Slot 56: `AARIVE1` (info).
    AlliedForcesEast,
    /// Slot 57: `AARRIVW1` (info).
    AlliedForcesWest,
    /// Slot 58: `1OBJMET1` (objective).
    Objective1Met,
    /// Slot 59: `2OBJMET1` (objective).
    Objective2Met,
    /// Slot 60: `3OBJMET1` (objective).
    Objective3Met,
    /// Slot 61: `IRONCHG1` (superweapon).
    IronCurtainCharging,
    /// Slot 62: `IRONRDY1` (superweapon).
    IronCurtainReady,
    /// Slot 63: `KOSYRES1` (rescue).
    Rescued,
    /// Slot 64: `OBJNMET1` (objective).
    ObjectiveNotMet,
    /// Slot 65: `FLAREN1` (warning).
    SignalNorth,
    /// Slot 66: `FLARES1` (warning).
    SignalSouth,
    /// Slot 67: `FLAREE1` (warning).
    SignalEast,
    /// Slot 68: `FLAREW1` (warning).
    SignalWest,
    /// Slot 69: `SPYPLN1` (intel).
    SpyPlane,
    /// Slot 70: `TANYAF1` (rescue).
    Freed,
    /// Slot 71: `ARMORUP1` (info).
    UpgradeArmor,
    /// Slot 72: `FIREPO1` (info).
    UpgradeFirepower,
    /// Slot 73: `UNITSPD1` (info).
    UpgradeSpeed,
    /// Slot 74: `MTIMEIN1` (time).
    MissionTimer,
    /// Slot 75: `UNITFUL1` (info).
    UnitFull,
    /// Slot 76: `UNITREP1` (info).
    UnitRepaired,
    /// Slot 77: `40MINR` (time).
    Time40,
    /// Slot 78: `30MINR` (time).
    Time30,
    /// Slot 79: `20MINR` (time).
    Time20,
    /// Slot 80: `10MINR` (time).
    Time10,
    /// Slot 81: `5MINR` (time).
    Time5,
    /// Slot 82: `4MINR` (time).
    Time4,
    /// Slot 83: `3MINR` (time).
    Time3,
    /// Slot 84: `2MINR` (time).
    Time2,
    /// Slot 85: `1MINR` (time).
    Time1,
    /// Slot 86: `TIMERNO1` (time).
    TimeStop,
    /// Slot 87: `UNITSLD1` (economy).
    UnitSold,
    /// Slot 88: `TIMERGO1` (time).
    TimerStarted,
    /// Slot 89: `TARGRES1` (rescue).
    TargetRescued,
    /// Slot 90: `TARGFRE1` (rescue).
    TargetFreed,
    /// Slot 91: `TANYAR1` (rescue).
    TanyaRescued,
    /// Slot 92: `STRUSLD1` (economy).
    StructureSold,
    /// Slot 93: `SOVFORC1` (mission).
    SovietForcesFallen,
    /// Slot 94: `SOVEMP1` (info).
    SovietSelected,
    /// Slot 95: `SOVEFAL1` (mission).
    SovietEmpireFallen,
    /// Slot 96: `OPTERM1` (mission).
    OperationTerminated,
    /// Slot 97: `OBJRCH1` (objective).
    ObjectiveReached,
    /// Slot 98: `OBJNRCH1` (objective).
    ObjectiveNotReached,
    /// Slot 99: `OBJMET1` (objective).
    ObjectiveCompleted,
    /// Slot 100: `MERCR1` (rescue).
    MercenaryRescued,
    /// Slot 101: `MERCF1` (rescue).
    MercenaryFreed,
    /// Slot 102: `KOSYFRE1` (rescue).
    KosoygenFreed,
    /// Slot 103: `FLARE1` (warning).
    FlareDetected,
    /// Slot 104: `COMNDOR1` (rescue).
    CommandoRescued,
    /// Slot 105: `COMNDOF1` (rescue).
    CommandoFreed,
    /// Slot 106: `BLDGPRG1` (production).
    BuildingInProgress,
    /// Slot 107: `ATPREP1` (superweapon).
    AtomicPrepping,
    /// Slot 108: `ASELECT1` (info).
    AlliedSelected,
    /// Slot 109: `APREP1` (superweapon).
    AtomicBombPrepping,
    /// Slot 110: `ATLNCH1` (superweapon).
    AtomLaunched,
    /// Slot 111: `AFALLEN1` (mission).
    AlliedForcesFallen,
    /// Slot 112: `AAVAIL1` (superweapon).
    AtomicBombAvailable,
    /// Slot 113: `AARRIVE1` (info).
    AlliedReinforcementsArrived,
    /// Slot 114: `SAVE1` (info).
    MissionSaved,
    /// Slot 115: `LOAD1` (info).
    MissionLoaded,
}

/// `Speech[]` entries that are playable: `(voice id, line, sample file)`.
///
/// Ordered by voice id; the ids are the original `VOX_*` ordinals.
const RA_SPEECH: &[(u16, RaEvaLine, &'static str)] = &[
    (0, RaEvaLine::MissionAccomplished, "MISNWON1"),
    (1, RaEvaLine::MissionFailed, "MISNLST1"),
    (2, RaEvaLine::UnableToComply, "PROGRES1"),
    (3, RaEvaLine::ConstructionComplete, "CONSCMP1"),
    (4, RaEvaLine::UnitReady, "UNITRDY1"),
    (5, RaEvaLine::NewConstructionOptions, "NEWOPT1"),
    (6, RaEvaLine::CannotDeployHere, "NODEPLY1"),
    (7, RaEvaLine::StructureDestroyed, "STRCKIL1"),
    (8, RaEvaLine::InsufficientPower, "NOPOWR1"),
    (9, RaEvaLine::InsufficientFunds, "NOFUNDS1"),
    (10, RaEvaLine::BattleControlTerminated, "BCT1"),
    (11, RaEvaLine::ReinforcementsArrived, "REINFOR1"),
    (12, RaEvaLine::Canceled, "CANCLD1"),
    (13, RaEvaLine::Building, "ABLDGIN1"),
    (14, RaEvaLine::LowPower, "LOPOWER1"),
    (15, RaEvaLine::NeedMoreFunds, "NOFUNDS1"),
    (16, RaEvaLine::BaseUnderAttack, "BASEATK1"),
    (17, RaEvaLine::UnableToBuildMore, "NOBUILD1"),
    (18, RaEvaLine::PrimaryBuildingSelected, "PRIBLDG1"),
    (19, RaEvaLine::MadTankDeployed, "TANK01"),
    (21, RaEvaLine::UnitLost, "UNITLST1"),
    (22, RaEvaLine::SelectTarget, "SLCTTGT1"),
    (23, RaEvaLine::EnemyApproaching, "ENMYAPP1"),
    (24, RaEvaLine::SilosNeeded, "SILOND1"),
    (25, RaEvaLine::OnHold, "ONHOLD1"),
    (26, RaEvaLine::Repairing, "REPAIR1"),
    (29, RaEvaLine::AircraftLost, "AUNITL1"),
    (31, RaEvaLine::AlliedForcesApproaching, "AAPPRO1"),
    (32, RaEvaLine::AlliedForcesArrived, "AARRIVE1"),
    (35, RaEvaLine::BuildingInfiltrated, "BLDGINF1"),
    (36, RaEvaLine::ChronosphereCharging, "CHROCHR1"),
    (37, RaEvaLine::ChronosphereReady, "CHRORDY1"),
    (38, RaEvaLine::ChronosphereTest, "CHROYES1"),
    (39, RaEvaLine::CommandCenterUnderAttack, "CMDCNTR1"),
    (40, RaEvaLine::ControlCenterDeactivated, "CNTLDED1"),
    (41, RaEvaLine::ConvoyApproaching, "CONVYAP1"),
    (42, RaEvaLine::ConvoyUnitLost, "CONVLST1"),
    (43, RaEvaLine::ExplosivePlaced, "XPLOPLC1"),
    (44, RaEvaLine::MoneyStolen, "CREDIT1"),
    (45, RaEvaLine::ShipLost, "NAVYLST1"),
    (46, RaEvaLine::SatelliteLaunched, "SATLNCH1"),
    (47, RaEvaLine::SonarAvailable, "PULSE1"),
    (49, RaEvaLine::SovietForcesApproaching, "SOVFAPP1"),
    (50, RaEvaLine::SovietReinforcementsArrived, "SOVREIN1"),
    (51, RaEvaLine::Training, "TRAIN1"),
    (52, RaEvaLine::AtomicBombReady, "AREADY1"),
    (53, RaEvaLine::AtomicBombLaunched, "ALAUNCH1"),
    (54, RaEvaLine::AlliedForcesNorth, "AARRIVN1"),
    (55, RaEvaLine::AlliedForcesSouth, "AARRIVS1"),
    (56, RaEvaLine::AlliedForcesEast, "AARIVE1"),
    (57, RaEvaLine::AlliedForcesWest, "AARRIVW1"),
    (58, RaEvaLine::Objective1Met, "1OBJMET1"),
    (59, RaEvaLine::Objective2Met, "2OBJMET1"),
    (60, RaEvaLine::Objective3Met, "3OBJMET1"),
    (61, RaEvaLine::IronCurtainCharging, "IRONCHG1"),
    (62, RaEvaLine::IronCurtainReady, "IRONRDY1"),
    (63, RaEvaLine::Rescued, "KOSYRES1"),
    (64, RaEvaLine::ObjectiveNotMet, "OBJNMET1"),
    (65, RaEvaLine::SignalNorth, "FLAREN1"),
    (66, RaEvaLine::SignalSouth, "FLARES1"),
    (67, RaEvaLine::SignalEast, "FLAREE1"),
    (68, RaEvaLine::SignalWest, "FLAREW1"),
    (69, RaEvaLine::SpyPlane, "SPYPLN1"),
    (70, RaEvaLine::Freed, "TANYAF1"),
    (71, RaEvaLine::UpgradeArmor, "ARMORUP1"),
    (72, RaEvaLine::UpgradeFirepower, "FIREPO1"),
    (73, RaEvaLine::UpgradeSpeed, "UNITSPD1"),
    (74, RaEvaLine::MissionTimer, "MTIMEIN1"),
    (75, RaEvaLine::UnitFull, "UNITFUL1"),
    (76, RaEvaLine::UnitRepaired, "UNITREP1"),
    (77, RaEvaLine::Time40, "40MINR"),
    (78, RaEvaLine::Time30, "30MINR"),
    (79, RaEvaLine::Time20, "20MINR"),
    (80, RaEvaLine::Time10, "10MINR"),
    (81, RaEvaLine::Time5, "5MINR"),
    (82, RaEvaLine::Time4, "4MINR"),
    (83, RaEvaLine::Time3, "3MINR"),
    (84, RaEvaLine::Time2, "2MINR"),
    (85, RaEvaLine::Time1, "1MINR"),
    (86, RaEvaLine::TimeStop, "TIMERNO1"),
    (87, RaEvaLine::UnitSold, "UNITSLD1"),
    (88, RaEvaLine::TimerStarted, "TIMERGO1"),
    (89, RaEvaLine::TargetRescued, "TARGRES1"),
    (90, RaEvaLine::TargetFreed, "TARGFRE1"),
    (91, RaEvaLine::TanyaRescued, "TANYAR1"),
    (92, RaEvaLine::StructureSold, "STRUSLD1"),
    (93, RaEvaLine::SovietForcesFallen, "SOVFORC1"),
    (94, RaEvaLine::SovietSelected, "SOVEMP1"),
    (95, RaEvaLine::SovietEmpireFallen, "SOVEFAL1"),
    (96, RaEvaLine::OperationTerminated, "OPTERM1"),
    (97, RaEvaLine::ObjectiveReached, "OBJRCH1"),
    (98, RaEvaLine::ObjectiveNotReached, "OBJNRCH1"),
    (99, RaEvaLine::ObjectiveCompleted, "OBJMET1"),
    (100, RaEvaLine::MercenaryRescued, "MERCR1"),
    (101, RaEvaLine::MercenaryFreed, "MERCF1"),
    (102, RaEvaLine::KosoygenFreed, "KOSYFRE1"),
    (103, RaEvaLine::FlareDetected, "FLARE1"),
    (104, RaEvaLine::CommandoRescued, "COMNDOR1"),
    (105, RaEvaLine::CommandoFreed, "COMNDOF1"),
    (106, RaEvaLine::BuildingInProgress, "BLDGPRG1"),
    (107, RaEvaLine::AtomicPrepping, "ATPREP1"),
    (108, RaEvaLine::AlliedSelected, "ASELECT1"),
    (109, RaEvaLine::AtomicBombPrepping, "APREP1"),
    (110, RaEvaLine::AtomLaunched, "ATLNCH1"),
    (111, RaEvaLine::AlliedForcesFallen, "AFALLEN1"),
    (112, RaEvaLine::AtomicBombAvailable, "AAVAIL1"),
    (113, RaEvaLine::AlliedReinforcementsArrived, "AARRIVE1"),
    (114, RaEvaLine::MissionSaved, "SAVE1"),
    (115, RaEvaLine::MissionLoaded, "LOAD1"),
];

/// `Speech[]` slots the original fills with the literal string `"none"`.
///
/// These occupy a table id but never play; scripts may still ask for them.
pub const RA_SILENT_SLOTS: &[(u16, &'static str)] = &[
    (20, "VOX_SOVIET_CAPTURED"),
    (27, "VOX_none5"),
    (28, "VOX_none6"),
    (30, "VOX_none7"),
    (33, "VOX_none8"),
    (34, "VOX_none9"),
    (48, "VOX_none10"),
];

/// Number of compiled slots in Red Alert's `Speech[]` table.
pub const RA_VOX_COUNT: u16 = 116;

impl RaEvaLine {
    /// Number of playable Red Alert lines.
    pub const COUNT: usize = RA_SPEECH.len();

    /// The line's `VOX_*` ordinal inside the original `Speech[]` table.
    pub fn voice_id(self) -> u16 {
        RA_SPEECH
            .iter()
            .find(|(_, line, _)| *line == self)
            .map(|(id, _, _)| *id)
            .expect("every RaEvaLine has a speech slot")
    }

    /// The line stored at a `VOX_*` ordinal, if that slot is playable.
    pub fn from_voice_id(id: u16) -> Option<Self> {
        RA_SPEECH
            .iter()
            .find(|(slot, _, _)| *slot == id)
            .map(|(_, line, _)| *line)
    }

    /// Base file name of the sample, without the `.AUD` extension.
    pub fn voice_file(self) -> &'static str {
        RA_SPEECH
            .iter()
            .find(|(_, line, _)| *line == self)
            .map(|(_, _, file)| *file)
            .expect("every RaEvaLine has a speech entry")
    }

    /// Full sample name, e.g. `MISNWON1.AUD`.
    pub fn sample_name(self) -> String {
        format!("{}.AUD", self.voice_file())
    }

    /// Every playable line, in table order.
    pub fn all() -> impl Iterator<Item = RaEvaLine> {
        RA_SPEECH.iter().map(|(_, line, _)| *line)
    }

    /// Look a line up by its `Speech[]` file name (case-insensitive).
    pub fn from_voice_file(name: &str) -> Option<Self> {
        RA_SPEECH
            .iter()
            .find(|(_, _, candidate)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, line, _)| *line)
    }

    /// Speech category, and through it the playback priority.
    pub fn category(self) -> SpeechCategory {
        match self {
            Self::MissionAccomplished => SpeechCategory::Mission,
            Self::MissionFailed => SpeechCategory::Mission,
            Self::UnableToComply => SpeechCategory::Production,
            Self::ConstructionComplete => SpeechCategory::Production,
            Self::UnitReady => SpeechCategory::Production,
            Self::NewConstructionOptions => SpeechCategory::Production,
            Self::CannotDeployHere => SpeechCategory::Production,
            Self::StructureDestroyed => SpeechCategory::Combat,
            Self::InsufficientPower => SpeechCategory::Power,
            Self::InsufficientFunds => SpeechCategory::Economy,
            Self::BattleControlTerminated => SpeechCategory::Mission,
            Self::ReinforcementsArrived => SpeechCategory::Info,
            Self::Canceled => SpeechCategory::Info,
            Self::Building => SpeechCategory::Production,
            Self::LowPower => SpeechCategory::Power,
            Self::NeedMoreFunds => SpeechCategory::Economy,
            Self::BaseUnderAttack => SpeechCategory::BaseAttack,
            Self::UnableToBuildMore => SpeechCategory::Production,
            Self::PrimaryBuildingSelected => SpeechCategory::Production,
            Self::MadTankDeployed => SpeechCategory::Superweapon,
            Self::UnitLost => SpeechCategory::Combat,
            Self::SelectTarget => SpeechCategory::Info,
            Self::EnemyApproaching => SpeechCategory::Warning,
            Self::SilosNeeded => SpeechCategory::Economy,
            Self::OnHold => SpeechCategory::Info,
            Self::Repairing => SpeechCategory::Info,
            Self::AircraftLost => SpeechCategory::Combat,
            Self::AlliedForcesApproaching => SpeechCategory::Warning,
            Self::AlliedForcesArrived => SpeechCategory::Info,
            Self::BuildingInfiltrated => SpeechCategory::Intel,
            Self::ChronosphereCharging => SpeechCategory::Superweapon,
            Self::ChronosphereReady => SpeechCategory::Superweapon,
            Self::ChronosphereTest => SpeechCategory::Superweapon,
            Self::CommandCenterUnderAttack => SpeechCategory::BaseAttack,
            Self::ControlCenterDeactivated => SpeechCategory::Intel,
            Self::ConvoyApproaching => SpeechCategory::Warning,
            Self::ConvoyUnitLost => SpeechCategory::Combat,
            Self::ExplosivePlaced => SpeechCategory::Intel,
            Self::MoneyStolen => SpeechCategory::Intel,
            Self::ShipLost => SpeechCategory::Combat,
            Self::SatelliteLaunched => SpeechCategory::Intel,
            Self::SonarAvailable => SpeechCategory::Intel,
            Self::SovietForcesApproaching => SpeechCategory::Warning,
            Self::SovietReinforcementsArrived => SpeechCategory::Info,
            Self::Training => SpeechCategory::Info,
            Self::AtomicBombReady => SpeechCategory::Superweapon,
            Self::AtomicBombLaunched => SpeechCategory::Superweapon,
            Self::AlliedForcesNorth => SpeechCategory::Info,
            Self::AlliedForcesSouth => SpeechCategory::Info,
            Self::AlliedForcesEast => SpeechCategory::Info,
            Self::AlliedForcesWest => SpeechCategory::Info,
            Self::Objective1Met => SpeechCategory::Objective,
            Self::Objective2Met => SpeechCategory::Objective,
            Self::Objective3Met => SpeechCategory::Objective,
            Self::IronCurtainCharging => SpeechCategory::Superweapon,
            Self::IronCurtainReady => SpeechCategory::Superweapon,
            Self::Rescued => SpeechCategory::Rescue,
            Self::ObjectiveNotMet => SpeechCategory::Objective,
            Self::SignalNorth => SpeechCategory::Warning,
            Self::SignalSouth => SpeechCategory::Warning,
            Self::SignalEast => SpeechCategory::Warning,
            Self::SignalWest => SpeechCategory::Warning,
            Self::SpyPlane => SpeechCategory::Intel,
            Self::Freed => SpeechCategory::Rescue,
            Self::UpgradeArmor => SpeechCategory::Info,
            Self::UpgradeFirepower => SpeechCategory::Info,
            Self::UpgradeSpeed => SpeechCategory::Info,
            Self::MissionTimer => SpeechCategory::Time,
            Self::UnitFull => SpeechCategory::Info,
            Self::UnitRepaired => SpeechCategory::Info,
            Self::Time40 => SpeechCategory::Time,
            Self::Time30 => SpeechCategory::Time,
            Self::Time20 => SpeechCategory::Time,
            Self::Time10 => SpeechCategory::Time,
            Self::Time5 => SpeechCategory::Time,
            Self::Time4 => SpeechCategory::Time,
            Self::Time3 => SpeechCategory::Time,
            Self::Time2 => SpeechCategory::Time,
            Self::Time1 => SpeechCategory::Time,
            Self::TimeStop => SpeechCategory::Time,
            Self::UnitSold => SpeechCategory::Economy,
            Self::TimerStarted => SpeechCategory::Time,
            Self::TargetRescued => SpeechCategory::Rescue,
            Self::TargetFreed => SpeechCategory::Rescue,
            Self::TanyaRescued => SpeechCategory::Rescue,
            Self::StructureSold => SpeechCategory::Economy,
            Self::SovietForcesFallen => SpeechCategory::Mission,
            Self::SovietSelected => SpeechCategory::Info,
            Self::SovietEmpireFallen => SpeechCategory::Mission,
            Self::OperationTerminated => SpeechCategory::Mission,
            Self::ObjectiveReached => SpeechCategory::Objective,
            Self::ObjectiveNotReached => SpeechCategory::Objective,
            Self::ObjectiveCompleted => SpeechCategory::Objective,
            Self::MercenaryRescued => SpeechCategory::Rescue,
            Self::MercenaryFreed => SpeechCategory::Rescue,
            Self::KosoygenFreed => SpeechCategory::Rescue,
            Self::FlareDetected => SpeechCategory::Warning,
            Self::CommandoRescued => SpeechCategory::Rescue,
            Self::CommandoFreed => SpeechCategory::Rescue,
            Self::BuildingInProgress => SpeechCategory::Production,
            Self::AtomicPrepping => SpeechCategory::Superweapon,
            Self::AlliedSelected => SpeechCategory::Info,
            Self::AtomicBombPrepping => SpeechCategory::Superweapon,
            Self::AtomLaunched => SpeechCategory::Superweapon,
            Self::AlliedForcesFallen => SpeechCategory::Mission,
            Self::AtomicBombAvailable => SpeechCategory::Superweapon,
            Self::AlliedReinforcementsArrived => SpeechCategory::Info,
            Self::MissionSaved => SpeechCategory::Info,
            Self::MissionLoaded => SpeechCategory::Info,
        }
    }
}

/// A Red Alert game event that can make EVA speak.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RaEvent {
    /// Mission won.
    MissionAccomplished,
    /// Mission lost.
    MissionFailed,
    /// Production request accepted while the factory is busy.
    ProductionStarted,
    /// A structure finished building.
    ConstructionComplete,
    /// A unit rolled off the line.
    UnitReady,
    /// A new build option became available.
    NewConstructionOptions,
    /// Deployment is impossible here.
    CannotDeployHere,
    /// One of the player's structures was destroyed.
    StructureDestroyed,
    /// Power is below demand.
    InsufficientPower,
    /// The player cannot pay.
    InsufficientFunds,
    /// The mission/session is over.
    BattleControlTerminated,
    /// Reinforcements arrived.
    ReinforcementsArrived,
    /// Production was canceled.
    Canceled,
    /// A structure is being built.
    Building,
    /// Low power.
    LowPower,
    /// Funds are almost gone.
    NeedMoreFunds,
    /// Own base is under attack.
    BaseUnderAttack,
    /// The build queue is full.
    UnableToBuildMore,
    /// A primary building was selected.
    PrimaryBuildingSelected,
    /// M.A.D. tank deployed.
    MadTankDeployed,
    /// An own unit was lost.
    UnitLost,
    /// A support power needs a target.
    SelectTarget,
    /// Enemy forces are approaching.
    EnemyApproaching,
    /// More silos are needed.
    SilosNeeded,
    /// Production is on hold.
    OnHold,
    /// A structure is being repaired.
    Repairing,
    /// An aircraft was lost.
    AircraftLost,
    /// Allied forces are approaching.
    AlliedForcesApproaching,
    /// Allied forces arrived.
    AlliedForcesArrived,
    /// A building was infiltrated.
    BuildingInfiltrated,
    /// An Allied building was captured (silent slot: no sample in English).
    BuildingCaptured,
    /// The chronosphere started charging.
    ChronosphereCharging,
    /// The chronosphere is ready.
    ChronosphereReady,
    /// The chronosphere test succeeded.
    ChronosphereTest,
    /// The command center is under attack.
    CommandCenterUnderAttack,
    /// The control center was deactivated.
    ControlCenterDeactivated,
    /// A convoy is approaching.
    ConvoyApproaching,
    /// A convoy unit was lost.
    ConvoyUnitLost,
    /// An explosive charge was placed.
    ExplosivePlaced,
    /// Credits were stolen.
    MoneyStolen,
    /// A naval unit was lost.
    ShipLost,
    /// A satellite was launched.
    SatelliteLaunched,
    /// Sonar is available.
    SonarAvailable,
    /// Soviet forces are approaching.
    SovietForcesApproaching,
    /// Soviet reinforcements arrived.
    SovietReinforcementsArrived,
    /// A unit finished training.
    Training,
    /// The atomic bomb is ready.
    AtomicBombReady,
    /// The atomic bomb was launched.
    AtomicBombLaunched,
    /// Allied forces arrived from a direction.
    AlliedForcesArrivedFrom(RaDirection),
    /// Campaign objective 1-3 was met (values outside 1..=3 have no line).
    ObjectiveMet(u8),
    /// The iron curtain started charging.
    IronCurtainCharging,
    /// The iron curtain is ready.
    IronCurtainReady,
    /// Someone was rescued.
    Rescued,
    /// An objective failed.
    ObjectiveNotMet,
    /// A flare signal from a direction.
    Signal(RaDirection),
    /// The spy plane is available.
    SpyPlane,
    /// A target was freed.
    Freed,
    /// A unit upgrade finished.
    Upgrade(RaUpgrade),
    /// The mission timer was set.
    MissionTimer,
    /// A unit is fully loaded.
    UnitFull,
    /// A unit was repaired.
    UnitRepaired,
    /// A countdown warning; only 40/30/20/10/5/4/3/2/1 minutes have samples.
    TimeWarning(u8),
    /// The countdown stopped.
    TimeStopped,
    /// A unit was sold.
    UnitSold,
    /// The mission timer started.
    TimerStarted,
    /// A target was rescued.
    TargetRescued,
    /// A target was freed.
    TargetFreed,
    /// Tanya was rescued.
    TanyaRescued,
    /// A structure was sold.
    StructureSold,
    /// Soviet forces have fallen.
    SovietForcesFallen,
    /// The Soviet side was selected.
    SovietSelected,
    /// The Soviet empire has fallen.
    SovietEmpireFallen,
    /// The operation was terminated.
    OperationTerminated,
    /// An objective was reached.
    ObjectiveReached,
    /// An objective was not reached.
    ObjectiveNotReached,
    /// The objective completion goal was reached.
    ObjectiveCompleted,
    /// A mercenary was rescued.
    MercenaryRescued,
    /// A mercenary was freed.
    MercenaryFreed,
    /// Kosoygen was freed.
    KosoygenFreed,
    /// A flare was detected.
    FlareDetected,
    /// A commando was rescued.
    CommandoRescued,
    /// A commando was freed.
    CommandoFreed,
    /// A structure is in progress.
    BuildingInProgress,
    /// The atomic bomb is preparing.
    AtomicPrepping,
    /// The Allied side was selected.
    AlliedSelected,
    /// The atomic bomb is preparing (Allied variant).
    AtomicBombPrepping,
    /// The Allied atomic bomb was launched.
    AtomLaunched,
    /// Allied forces have fallen.
    AlliedForcesFallen,
    /// The atomic bomb is available.
    AtomicBombAvailable,
    /// Allied reinforcements arrived.
    AlliedReinforcementsArrived,
    /// The mission was saved.
    MissionSaved,
    /// The mission was loaded.
    MissionLoaded,
}

impl RaEvaLine {
    /// The EVA line a game event produces, if it has one.
    ///
    /// `None` means the event is silent in Red Alert: either the format's
    /// silent slot (`BuildingCaptured`) or an argument the table has no
    /// sample for (an out-of-range objective or countdown).
    pub fn from_event(event: RaEvent) -> Option<Self> {
        let line = match event {
            RaEvent::MissionAccomplished => Self::MissionAccomplished,
            RaEvent::MissionFailed => Self::MissionFailed,
            RaEvent::ProductionStarted => Self::UnableToComply,
            RaEvent::ConstructionComplete => Self::ConstructionComplete,
            RaEvent::UnitReady => Self::UnitReady,
            RaEvent::NewConstructionOptions => Self::NewConstructionOptions,
            RaEvent::CannotDeployHere => Self::CannotDeployHere,
            RaEvent::StructureDestroyed => Self::StructureDestroyed,
            RaEvent::InsufficientPower => Self::InsufficientPower,
            RaEvent::InsufficientFunds => Self::InsufficientFunds,
            RaEvent::BattleControlTerminated => Self::BattleControlTerminated,
            RaEvent::ReinforcementsArrived => Self::ReinforcementsArrived,
            RaEvent::Canceled => Self::Canceled,
            RaEvent::Building => Self::Building,
            RaEvent::LowPower => Self::LowPower,
            RaEvent::NeedMoreFunds => Self::NeedMoreFunds,
            RaEvent::BaseUnderAttack => Self::BaseUnderAttack,
            RaEvent::UnableToBuildMore => Self::UnableToBuildMore,
            RaEvent::PrimaryBuildingSelected => Self::PrimaryBuildingSelected,
            RaEvent::MadTankDeployed => Self::MadTankDeployed,
            RaEvent::UnitLost => Self::UnitLost,
            RaEvent::SelectTarget => Self::SelectTarget,
            RaEvent::EnemyApproaching => Self::EnemyApproaching,
            RaEvent::SilosNeeded => Self::SilosNeeded,
            RaEvent::OnHold => Self::OnHold,
            RaEvent::Repairing => Self::Repairing,
            RaEvent::AircraftLost => Self::AircraftLost,
            RaEvent::AlliedForcesApproaching => Self::AlliedForcesApproaching,
            RaEvent::AlliedForcesArrived => Self::AlliedForcesArrived,
            RaEvent::BuildingInfiltrated => Self::BuildingInfiltrated,
            RaEvent::BuildingCaptured => return None,
            RaEvent::ChronosphereCharging => Self::ChronosphereCharging,
            RaEvent::ChronosphereReady => Self::ChronosphereReady,
            RaEvent::ChronosphereTest => Self::ChronosphereTest,
            RaEvent::CommandCenterUnderAttack => Self::CommandCenterUnderAttack,
            RaEvent::ControlCenterDeactivated => Self::ControlCenterDeactivated,
            RaEvent::ConvoyApproaching => Self::ConvoyApproaching,
            RaEvent::ConvoyUnitLost => Self::ConvoyUnitLost,
            RaEvent::ExplosivePlaced => Self::ExplosivePlaced,
            RaEvent::MoneyStolen => Self::MoneyStolen,
            RaEvent::ShipLost => Self::ShipLost,
            RaEvent::SatelliteLaunched => Self::SatelliteLaunched,
            RaEvent::SonarAvailable => Self::SonarAvailable,
            RaEvent::SovietForcesApproaching => Self::SovietForcesApproaching,
            RaEvent::SovietReinforcementsArrived => Self::SovietReinforcementsArrived,
            RaEvent::Training => Self::Training,
            RaEvent::AtomicBombReady => Self::AtomicBombReady,
            RaEvent::AtomicBombLaunched => Self::AtomicBombLaunched,
            RaEvent::AlliedForcesArrivedFrom(direction) => match direction {
                RaDirection::North => Self::AlliedForcesNorth,
                RaDirection::South => Self::AlliedForcesSouth,
                RaDirection::East => Self::AlliedForcesEast,
                RaDirection::West => Self::AlliedForcesWest,
            },
            RaEvent::ObjectiveMet(index) => match index {
                1 => Self::Objective1Met,
                2 => Self::Objective2Met,
                3 => Self::Objective3Met,
                _ => return None,
            },
            RaEvent::IronCurtainCharging => Self::IronCurtainCharging,
            RaEvent::IronCurtainReady => Self::IronCurtainReady,
            RaEvent::Rescued => Self::Rescued,
            RaEvent::ObjectiveNotMet => Self::ObjectiveNotMet,
            RaEvent::Signal(direction) => match direction {
                RaDirection::North => Self::SignalNorth,
                RaDirection::South => Self::SignalSouth,
                RaDirection::East => Self::SignalEast,
                RaDirection::West => Self::SignalWest,
            },
            RaEvent::SpyPlane => Self::SpyPlane,
            RaEvent::Freed => Self::Freed,
            RaEvent::Upgrade(upgrade) => match upgrade {
                RaUpgrade::Armor => Self::UpgradeArmor,
                RaUpgrade::Firepower => Self::UpgradeFirepower,
                RaUpgrade::Speed => Self::UpgradeSpeed,
            },
            RaEvent::MissionTimer => Self::MissionTimer,
            RaEvent::UnitFull => Self::UnitFull,
            RaEvent::UnitRepaired => Self::UnitRepaired,
            RaEvent::TimeWarning(minutes) => match minutes {
                40 => Self::Time40,
                30 => Self::Time30,
                20 => Self::Time20,
                10 => Self::Time10,
                5 => Self::Time5,
                4 => Self::Time4,
                3 => Self::Time3,
                2 => Self::Time2,
                1 => Self::Time1,
                _ => return None,
            },
            RaEvent::TimeStopped => Self::TimeStop,
            RaEvent::UnitSold => Self::UnitSold,
            RaEvent::TimerStarted => Self::TimerStarted,
            RaEvent::TargetRescued => Self::TargetRescued,
            RaEvent::TargetFreed => Self::TargetFreed,
            RaEvent::TanyaRescued => Self::TanyaRescued,
            RaEvent::StructureSold => Self::StructureSold,
            RaEvent::SovietForcesFallen => Self::SovietForcesFallen,
            RaEvent::SovietSelected => Self::SovietSelected,
            RaEvent::SovietEmpireFallen => Self::SovietEmpireFallen,
            RaEvent::OperationTerminated => Self::OperationTerminated,
            RaEvent::ObjectiveReached => Self::ObjectiveReached,
            RaEvent::ObjectiveNotReached => Self::ObjectiveNotReached,
            RaEvent::ObjectiveCompleted => Self::ObjectiveCompleted,
            RaEvent::MercenaryRescued => Self::MercenaryRescued,
            RaEvent::MercenaryFreed => Self::MercenaryFreed,
            RaEvent::KosoygenFreed => Self::KosoygenFreed,
            RaEvent::FlareDetected => Self::FlareDetected,
            RaEvent::CommandoRescued => Self::CommandoRescued,
            RaEvent::CommandoFreed => Self::CommandoFreed,
            RaEvent::BuildingInProgress => Self::BuildingInProgress,
            RaEvent::AtomicPrepping => Self::AtomicPrepping,
            RaEvent::AlliedSelected => Self::AlliedSelected,
            RaEvent::AtomicBombPrepping => Self::AtomicBombPrepping,
            RaEvent::AtomLaunched => Self::AtomLaunched,
            RaEvent::AlliedForcesFallen => Self::AlliedForcesFallen,
            RaEvent::AtomicBombAvailable => Self::AtomicBombAvailable,
            RaEvent::AlliedReinforcementsArrived => Self::AlliedReinforcementsArrived,
            RaEvent::MissionSaved => Self::MissionSaved,
            RaEvent::MissionLoaded => Self::MissionLoaded,
        };
        Some(line)
    }
}

impl EvaVocabulary for RaEvaLine {
    fn voice_id(self) -> u16 {
        self.voice_id()
    }

    fn sample_file(self) -> Option<&'static str> {
        Some(self.voice_file())
    }

    fn category(self) -> SpeechCategory {
        self.category()
    }
}

/// Red Alert's EVA queue over the shared deterministic queue.
pub type RaEvaQueue = SpeechQueue<RaEvaLine>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eva::{EvaLine, EvaQueue};

    #[test]
    fn every_ra_line_has_a_speech_file() {
        for line in RaEvaLine::all() {
            assert!(!line.voice_file().is_empty());
            assert!(line.sample_name().ends_with(".AUD"));
        }
    }

    #[test]
    fn ra_voice_names_match_the_original_table() {
        assert_eq!(RaEvaLine::MissionAccomplished.voice_file(), "MISNWON1");
        assert_eq!(RaEvaLine::UnitReady.voice_file(), "UNITRDY1");
        assert_eq!(RaEvaLine::BaseUnderAttack.voice_file(), "BASEATK1");
        assert_eq!(RaEvaLine::sample_name(RaEvaLine::LowPower), "LOPOWER1.AUD");
    }

    #[test]
    fn ra_and_td_use_different_samples_for_the_same_line() {
        // The two games ship different recordings; a mix-up here would be a
        // silent wrong-voice bug.
        assert_eq!(crate::eva::EvaLine::UnitReady.voice_file(), "UNITREDY");
        assert_eq!(RaEvaLine::UnitReady.voice_file(), "UNITRDY1");
    }

    #[test]
    fn ra_voice_file_lookup_is_case_insensitive() {
        assert_eq!(
            RaEvaLine::from_voice_file("misnwon1"),
            Some(RaEvaLine::MissionAccomplished)
        );
        assert_eq!(RaEvaLine::from_voice_file("nope"), None);
    }

    #[test]
    fn ra_queue_uses_the_shared_protocol() {
        let mut queue = RaEvaQueue::new();
        assert_eq!(
            queue.speak(RaEvaLine::UnitReady),
            crate::speech::SpeakOutcome::Queued
        );
        let started = queue.advance(false).expect("unit ready starts");
        assert_eq!(started.line, RaEvaLine::UnitReady);
        assert_eq!(started.sample, "UNITRDY1");
    }

    #[test]
    fn td_and_ra_queues_are_distinct_types() {
        // Guard against the old bug where the RA queue alias queued TD lines.
        let mut td = EvaQueue::new();
        assert!(td.speak(EvaLine::UnitReady));
        let mut ra = RaEvaQueue::new();
        assert_eq!(
            ra.speak(RaEvaLine::UnitReady),
            crate::speech::SpeakOutcome::Queued
        );
    }
}
