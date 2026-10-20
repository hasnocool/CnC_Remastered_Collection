//! EVA speech: the voice-over event vocabulary and its play queue.
//!
//! The original routes every EVA line through `Speak()` (`TIBERIANDAWN/AUDIO.CPP`)
//! which queues at most one pending voice while another is playing, and
//! `Speak_AI()` which starts the queued line once the previous one finishes.
//! The Remaster itself intercepts `Speak()` via `On_Speech`, so the sample
//! playback below is a behavioural reproduction rather than a byte-for-byte
//! port.
//!
//! Vocabulary and file names are transcribed verbatim from
//! `TIBERIANDAWN/DEFINES.H` (`VOX_*`) and `TIBERIANDAWN/AUDIO.CPP` (`Speech[]`).

use std::collections::VecDeque;

use crate::mixer::Channel;
use crate::speech::SpeechCategory;

/// One EVA speech event, in the original `VoxType` order.
///
/// Commented-out entries in the original enum are omitted, so the discriminant
/// values here are NOT the original `VOX_*` integers; use
/// [`EvaLine::voice_file`] or [`EvaLine::from_name`] rather than casting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvaLine {
    MissionAccomplished,
    MissionFailed,
    BuildingInProgress,
    ConstructionComplete,
    UnitReady,
    NewConstructionOptions,
    CannotDeployHere,
    GdiUnitDestroyed,
    NodUnitDestroyed,
    CivilianKilled,
    InsufficientFunds,
    BattleControlTerminated,
    ReinforcementsArrived,
    Canceled,
    Building,
    LowPower,
    InsufficientPower,
    NeedMoreFunds,
    BaseUnderAttack,
    IncomingMissile,
    EnemyPlanesApproaching,
    IncomingNuclearWeapon,
    UnableToBuildMore,
    PrimaryBuildingSelected,
    NodBuildingCaptured,
    GdiBuildingCaptured,
    IonCannonCharging,
    IonCannonReady,
    NuclearWeaponAvailable,
    NuclearWeaponLaunched,
    UnitLost,
    StructureLost,
    NeedHarvester,
    SelectTarget,
    AirstrikeReady,
    NotReady,
    TransportSighted,
    TransportLoaded,
    EnemyApproaching,
    SilosNeeded,
    OnHold,
    Repairing,
    EnemyStructureDestroyed,
    GdiStructureDestroyed,
    NodStructureDestroyed,
    EnemyUnitDestroyed,
}

/// The `Speech[]` table: base file name (extension `.AUD`) for each line.
const SPEECH_FILES: &[(&str, EvaLine)] = &[
    ("ACCOM1", EvaLine::MissionAccomplished),
    ("FAIL1", EvaLine::MissionFailed),
    ("BLDG1", EvaLine::BuildingInProgress),
    ("CONSTRU1", EvaLine::ConstructionComplete),
    ("UNITREDY", EvaLine::UnitReady),
    ("NEWOPT1", EvaLine::NewConstructionOptions),
    ("DEPLOY1", EvaLine::CannotDeployHere),
    ("GDIDEAD1", EvaLine::GdiUnitDestroyed),
    ("NODDEAD1", EvaLine::NodUnitDestroyed),
    ("CIVDEAD1", EvaLine::CivilianKilled),
    ("NOCASH1", EvaLine::InsufficientFunds),
    ("BATLCON1", EvaLine::BattleControlTerminated),
    ("REINFOR1", EvaLine::ReinforcementsArrived),
    ("CANCEL1", EvaLine::Canceled),
    ("BLDGING1", EvaLine::Building),
    ("LOPOWER1", EvaLine::LowPower),
    ("NOPOWER1", EvaLine::InsufficientPower),
    ("MOCASH1", EvaLine::NeedMoreFunds),
    ("BASEATK1", EvaLine::BaseUnderAttack),
    ("INCOME1", EvaLine::IncomingMissile),
    ("ENEMYA", EvaLine::EnemyPlanesApproaching),
    ("NUKE1", EvaLine::IncomingNuclearWeapon),
    ("NOBUILD1", EvaLine::UnableToBuildMore),
    ("PRIBLDG1", EvaLine::PrimaryBuildingSelected),
    ("NODCAPT1", EvaLine::NodBuildingCaptured),
    ("GDICAPT1", EvaLine::GdiBuildingCaptured),
    ("IONCHRG1", EvaLine::IonCannonCharging),
    ("IONREDY1", EvaLine::IonCannonReady),
    ("NUKAVAIL", EvaLine::NuclearWeaponAvailable),
    ("NUKLNCH1", EvaLine::NuclearWeaponLaunched),
    ("UNITLOST", EvaLine::UnitLost),
    ("STRCLOST", EvaLine::StructureLost),
    ("NEEDHARV", EvaLine::NeedHarvester),
    ("SELECT1", EvaLine::SelectTarget),
    ("AIRREDY1", EvaLine::AirstrikeReady),
    ("NOREDY1", EvaLine::NotReady),
    ("TRANSSEE", EvaLine::TransportSighted),
    ("TRANLOAD", EvaLine::TransportLoaded),
    ("ENMYAPP1", EvaLine::EnemyApproaching),
    ("SILOS1", EvaLine::SilosNeeded),
    ("ONHOLD1", EvaLine::OnHold),
    ("REPAIR1", EvaLine::Repairing),
    ("ESTRUCX", EvaLine::EnemyStructureDestroyed),
    ("GSTRUC1", EvaLine::GdiStructureDestroyed),
    ("NSTRUC1", EvaLine::NodStructureDestroyed),
    ("ENMYUNIT", EvaLine::EnemyUnitDestroyed),
];

impl EvaLine {
    /// Base file name of the sample (`Speech[]`), without the `.AUD` extension.
    pub fn voice_file(self) -> &'static str {
        SPEECH_FILES
            .iter()
            .find(|(_, line)| *line == self)
            .map(|(name, _)| *name)
            .expect("every EvaLine has a speech entry")
    }

    /// Full sample name, e.g. `ACCOM1.AUD`.
    pub fn sample_name(self) -> String {
        format!("{}.AUD", self.voice_file())
    }

    /// Every line that has a speech sample, in the original table order.
    pub fn all() -> impl Iterator<Item = EvaLine> {
        SPEECH_FILES.iter().map(|(_, line)| *line)
    }

    /// Look a line up by its `Speech[]` file name (case-insensitive).
    pub fn from_voice_file(name: &str) -> Option<Self> {
        SPEECH_FILES
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, line)| *line)
    }
}

/// Every Tiberian Dawn EVA line, in the engine's `VOX_*` order.
impl EvaLine {
    /// Number of lines in the TD vocabulary.
    pub const COUNT: usize = SPEECH_FILES.len();

    /// The `VOX_*` ordinal of the line inside the original `Speech[]` table.
    ///
    /// The table is transcribed in engine order, so the index is the engine's
    /// own enum value (`VOX_ACCOMPLISHED` = 0 ... `VOX_ENEMY_UNIT` = 44).
    pub fn voice_id(self) -> u16 {
        SPEECH_FILES
            .iter()
            .position(|(_, line)| *line == self)
            .expect("every EvaLine has a speech entry") as u16
    }

    /// The line stored at a `VOX_*` ordinal.
    pub fn from_voice_id(id: u16) -> Option<Self> {
        SPEECH_FILES.get(id as usize).map(|(_, line)| *line)
    }

    /// Speech category, and through it the playback priority.
    pub fn category(self) -> SpeechCategory {
        use SpeechCategory::{
            BaseAttack, Combat, Economy, Info, Intel, Mission, Power, Production, Superweapon,
            Warning,
        };
        match self {
            Self::MissionAccomplished | Self::MissionFailed | Self::BattleControlTerminated => {
                Mission
            }
            Self::BuildingInProgress
            | Self::ConstructionComplete
            | Self::UnitReady
            | Self::NewConstructionOptions
            | Self::CannotDeployHere
            | Self::Building
            | Self::UnableToBuildMore
            | Self::PrimaryBuildingSelected => Production,
            Self::InsufficientFunds | Self::NeedMoreFunds | Self::SilosNeeded => Economy,
            Self::LowPower | Self::InsufficientPower => Power,
            Self::BaseUnderAttack => BaseAttack,
            Self::IncomingMissile
            | Self::EnemyPlanesApproaching
            | Self::IncomingNuclearWeapon
            | Self::TransportSighted
            | Self::EnemyApproaching => Warning,
            Self::NodBuildingCaptured | Self::GdiBuildingCaptured => Intel,
            Self::IonCannonCharging
            | Self::IonCannonReady
            | Self::NuclearWeaponAvailable
            | Self::NuclearWeaponLaunched
            | Self::AirstrikeReady => Superweapon,
            Self::GdiUnitDestroyed
            | Self::NodUnitDestroyed
            | Self::CivilianKilled
            | Self::UnitLost
            | Self::StructureLost
            | Self::EnemyUnitDestroyed
            | Self::EnemyStructureDestroyed
            | Self::GdiStructureDestroyed
            | Self::NodStructureDestroyed => Combat,
            Self::ReinforcementsArrived
            | Self::Canceled
            | Self::NeedHarvester
            | Self::SelectTarget
            | Self::NotReady
            | Self::TransportLoaded
            | Self::OnHold
            | Self::Repairing => Info,
        }
    }

    /// The EVA line a game event produces, if any.
    pub fn from_event(event: TdEvent) -> Option<Self> {
        let line = match event {
            TdEvent::MissionAccomplished => Self::MissionAccomplished,
            TdEvent::MissionFailed => Self::MissionFailed,
            TdEvent::ProductionStarted => Self::BuildingInProgress,
            TdEvent::ConstructionComplete => Self::ConstructionComplete,
            TdEvent::UnitReady => Self::UnitReady,
            TdEvent::NewConstructionOptions => Self::NewConstructionOptions,
            TdEvent::CannotDeployHere => Self::CannotDeployHere,
            TdEvent::UnitDestroyed(faction) => match faction {
                TdFaction::Gdi => Self::GdiUnitDestroyed,
                TdFaction::Nod => Self::NodUnitDestroyed,
                TdFaction::Civilian => Self::CivilianKilled,
            },
            TdEvent::InsufficientFunds => Self::InsufficientFunds,
            TdEvent::NeedMoreFunds => Self::NeedMoreFunds,
            TdEvent::BattleControlTerminated => Self::BattleControlTerminated,
            TdEvent::ReinforcementsArrived => Self::ReinforcementsArrived,
            TdEvent::Canceled => Self::Canceled,
            TdEvent::Building => Self::Building,
            TdEvent::LowPower => Self::LowPower,
            TdEvent::InsufficientPower => Self::InsufficientPower,
            TdEvent::BaseUnderAttack => Self::BaseUnderAttack,
            TdEvent::IncomingMissile => Self::IncomingMissile,
            TdEvent::EnemyPlanesApproaching => Self::EnemyPlanesApproaching,
            TdEvent::IncomingNuclearWeapon => Self::IncomingNuclearWeapon,
            TdEvent::UnableToBuildMore => Self::UnableToBuildMore,
            TdEvent::PrimaryBuildingSelected => Self::PrimaryBuildingSelected,
            TdEvent::BuildingCaptured(faction) => match faction {
                TdFaction::Gdi => Self::GdiBuildingCaptured,
                TdFaction::Nod => Self::NodBuildingCaptured,
                TdFaction::Civilian => return None,
            },
            TdEvent::IonCannonCharging => Self::IonCannonCharging,
            TdEvent::IonCannonReady => Self::IonCannonReady,
            TdEvent::NuclearWeaponAvailable => Self::NuclearWeaponAvailable,
            TdEvent::NuclearWeaponLaunched => Self::NuclearWeaponLaunched,
            TdEvent::UnitLost => Self::UnitLost,
            TdEvent::StructureLost => Self::StructureLost,
            TdEvent::NeedHarvester => Self::NeedHarvester,
            TdEvent::SelectTarget => Self::SelectTarget,
            TdEvent::AirstrikeReady => Self::AirstrikeReady,
            TdEvent::NotReady => Self::NotReady,
            TdEvent::TransportSighted => Self::TransportSighted,
            TdEvent::TransportLoaded => Self::TransportLoaded,
            TdEvent::EnemyApproaching => Self::EnemyApproaching,
            TdEvent::SilosNeeded => Self::SilosNeeded,
            TdEvent::OnHold => Self::OnHold,
            TdEvent::Repairing => Self::Repairing,
            TdEvent::StructureDestroyed(faction) => match faction {
                TdFaction::Gdi => Self::GdiStructureDestroyed,
                TdFaction::Nod => Self::NodStructureDestroyed,
                TdFaction::Civilian => Self::EnemyStructureDestroyed,
            },
            TdEvent::EnemyUnitDestroyed => Self::EnemyUnitDestroyed,
        };
        Some(line)
    }
}

impl crate::speech::EvaVocabulary for EvaLine {
    fn voice_id(self) -> u16 {
        self.voice_id()
    }

    fn sample_file(self) -> Option<&'static str> {
        Some(EvaLine::voice_file(self))
    }

    fn category(self) -> SpeechCategory {
        self.category()
    }
}

/// Which army an event concerns (Tiberian Dawn speaks lines that name the
/// side of the destroyed/captured object).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TdFaction {
    /// Global Defense Initiative.
    Gdi,
    /// Brotherhood of Nod.
    Nod,
    /// Civilians / neutral houses (they share the generic "enemy" lines).
    Civilian,
}

/// A game event that can make EVA speak.
///
/// This is the presentation-facing vocabulary: the simulation reports what
/// happened and this layer decides which line (if any) it produces. Side
/// dependent lines take a [`TdFaction`] instead of inventing one event per
/// side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TdEvent {
    /// Mission won.
    MissionAccomplished,
    /// Mission lost.
    MissionFailed,
    /// Production request accepted while the factory is busy
    /// (`VOX_NO_FACTORY`).
    ProductionStarted,
    /// A structure finished building.
    ConstructionComplete,
    /// A unit rolled off the line.
    UnitReady,
    /// A new build option became available.
    NewConstructionOptions,
    /// Deployment of the selected unit is impossible here.
    CannotDeployHere,
    /// A unit belonging to `faction` died.
    UnitDestroyed(TdFaction),
    /// The player cannot pay for the current production.
    InsufficientFunds,
    /// The player is close to broke.
    NeedMoreFunds,
    /// The mission/network session is over.
    BattleControlTerminated,
    /// Friendly reinforcements arrived.
    ReinforcementsArrived,
    /// The player canceled production.
    Canceled,
    /// A structure is being built (production tick).
    Building,
    /// Power is below demand.
    LowPower,
    /// Power is not enough for the current load at all.
    InsufficientPower,
    /// Own base is taking damage.
    BaseUnderAttack,
    /// An enemy missile is inbound.
    IncomingMissile,
    /// Enemy aircraft are inbound.
    EnemyPlanesApproaching,
    /// An enemy nuclear warhead is inbound.
    IncomingNuclearWeapon,
    /// The build queue is full.
    UnableToBuildMore,
    /// A primary structure was selected.
    PrimaryBuildingSelected,
    /// A building belonging to `faction` was captured by the player.
    BuildingCaptured(TdFaction),
    /// The ion cannon started charging.
    IonCannonCharging,
    /// The ion cannon is ready to fire.
    IonCannonReady,
    /// A nuclear weapon became available.
    NuclearWeaponAvailable,
    /// A nuclear weapon was launched.
    NuclearWeaponLaunched,
    /// An own unit was lost.
    UnitLost,
    /// An own structure was lost.
    StructureLost,
    /// No harvester is available.
    NeedHarvester,
    /// The player must pick a target for a support power.
    SelectTarget,
    /// Airstrike support is ready.
    AirstrikeReady,
    /// A requested support power is not ready yet.
    NotReady,
    /// An enemy transport was spotted.
    TransportSighted,
    /// An enemy transport finished loading.
    TransportLoaded,
    /// Enemy forces are approaching.
    EnemyApproaching,
    /// More silos are needed for storage.
    SilosNeeded,
    /// The current production is on hold.
    OnHold,
    /// A structure is being repaired.
    Repairing,
    /// A structure belonging to `faction` was destroyed by the player.
    StructureDestroyed(TdFaction),
    /// Any enemy unit was destroyed.
    EnemyUnitDestroyed,
}

/// Mirrors `Speak()` + `Speak_AI()`: one playing line plus one queued line.
///
/// `Speak()` only stores the request when nothing is already queued, so a burst
/// of events does not pile up — the newest line is dropped rather than
/// lengthening the backlog. That is the original behaviour, reproduced.
#[derive(Debug, Default)]
pub struct EvaQueue {
    /// Line started by the last `update`.
    current: Option<EvaLine>,
    /// Line waiting for the current one to finish.
    pending: Option<EvaLine>,
    /// Anything the caller has no sample for.
    log: VecDeque<EvaLine>,
}

impl EvaQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Request a line (`Speak`). Returns `false` when the request was dropped
    /// because another line is already queued.
    pub fn speak(&mut self, line: EvaLine) -> bool {
        if self.pending.is_some() {
            return false;
        }
        self.pending = Some(line);
        true
    }

    /// The line currently "playing", if any.
    pub fn current(&self) -> Option<EvaLine> {
        self.current
    }

    /// The line waiting to play, if any.
    pub fn pending(&self) -> Option<EvaLine> {
        self.pending
    }

    /// Finish the current line and start the pending one (`Speak_AI`).
    ///
    /// Returns the line that just started, if any.
    pub fn update(&mut self) -> Option<EvaLine> {
        if self.current.is_some() {
            return None;
        }
        let started = self.pending.take();
        self.current = started;
        if let Some(line) = started {
            self.log.push_back(line);
            if self.log.len() > MAX_LOG {
                self.log.pop_front();
            }
        }
        started
    }

    /// Mark the playing line as finished (`Stop_Speaking` on completion).
    pub fn finish_current(&mut self) {
        self.current = None;
    }

    /// Drop everything (`Stop_Speaking`).
    pub fn stop(&mut self) {
        self.current = None;
        self.pending = None;
    }

    /// Every line that has actually started, oldest first (bounded).
    pub fn history(&self) -> &VecDeque<EvaLine> {
        &self.log
    }
}

/// How many finished lines the history keeps.
const MAX_LOG: usize = 64;

/// An EVA queue bound to a mixer, so a started line actually plays.
pub struct EvaPlayer {
    queue: EvaQueue,
    /// Volume applied to the voice channel.
    pub volume: f32,
}

impl Default for EvaPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl EvaPlayer {
    pub fn new() -> Self {
        Self {
            queue: EvaQueue::new(),
            volume: 0.8,
        }
    }

    /// The underlying queue (for inspection in tests and HUD code).
    pub fn queue(&self) -> &EvaQueue {
        &self.queue
    }

    /// Queue a line; see [`EvaQueue::speak`].
    pub fn speak(&mut self, line: EvaLine) -> bool {
        self.queue.speak(line)
    }

    /// Advance the queue and report which line became current.
    ///
    /// The caller plays [`EvaLine::sample_name`] on [`Channel::Voice`]. The
    /// decoder stays outside this type so the audio backend's concrete types
    /// do not leak into the public API.
    pub fn advance(&mut self) -> Option<EvaLine> {
        self.queue.update()
    }

    /// Volume the caller should pass for the voice channel.
    pub fn channel_volume(&self) -> f32 {
        self.volume
    }

    /// The channel EVA speech belongs on.
    pub fn channel() -> Channel {
        Channel::Voice
    }

    /// Mark the playing line finished so the next one may start.
    pub fn finish(&mut self) {
        self.queue.finish_current();
    }

    /// Stop everything immediately.
    pub fn stop(&mut self) {
        self.queue.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_line_has_a_speech_file() {
        for line in EvaLine::all() {
            assert!(!line.voice_file().is_empty());
        }
        assert_eq!(EvaLine::all().count(), SPEECH_FILES.len());
    }

    #[test]
    fn known_voice_names_match_the_original_table() {
        assert_eq!(EvaLine::MissionAccomplished.voice_file(), "ACCOM1");
        assert_eq!(EvaLine::ConstructionComplete.voice_file(), "CONSTRU1");
        assert_eq!(EvaLine::BaseUnderAttack.voice_file(), "BASEATK1");
        assert_eq!(EvaLine::IncomingNuclearWeapon.voice_file(), "NUKE1");
        assert_eq!(EvaLine::NuclearWeaponLaunched.voice_file(), "NUKLNCH1");
        assert_eq!(EvaLine::sample_name(EvaLine::UnitReady), "UNITREDY.AUD");
    }

    #[test]
    fn voice_file_lookup_is_case_insensitive() {
        assert_eq!(
            EvaLine::from_voice_file("accom1"),
            Some(EvaLine::MissionAccomplished)
        );
        assert_eq!(EvaLine::from_voice_file("nope"), None);
    }

    #[test]
    fn voice_files_are_unique() {
        let mut names: Vec<&str> = EvaLine::all().map(|l| l.voice_file()).collect();
        names.sort_unstable();
        let before = names.len();
        names.dedup();
        assert_eq!(before, names.len(), "duplicate sample names");
    }

    #[test]
    fn speak_starts_on_the_next_update() {
        let mut queue = EvaQueue::new();
        assert!(queue.speak(EvaLine::UnitReady));
        assert_eq!(queue.current(), None);
        assert_eq!(queue.update(), Some(EvaLine::UnitReady));
        assert_eq!(queue.current(), Some(EvaLine::UnitReady));
    }

    #[test]
    fn a_second_speak_is_dropped_while_one_is_pending() {
        let mut queue = EvaQueue::new();
        assert!(queue.speak(EvaLine::UnitReady));
        assert!(!queue.speak(EvaLine::ConstructionComplete));
        assert_eq!(queue.pending(), Some(EvaLine::UnitReady));
    }

    #[test]
    fn no_new_line_starts_while_one_is_playing() {
        let mut queue = EvaQueue::new();
        queue.speak(EvaLine::UnitReady);
        queue.update();
        queue.speak(EvaLine::Canceled);
        assert_eq!(queue.update(), None, "current line still playing");
        queue.finish_current();
        assert_eq!(queue.update(), Some(EvaLine::Canceled));
    }

    #[test]
    fn stop_clears_both_slots() {
        let mut queue = EvaQueue::new();
        queue.speak(EvaLine::UnitReady);
        queue.update();
        queue.speak(EvaLine::Canceled);
        queue.stop();
        assert_eq!(queue.current(), None);
        assert_eq!(queue.pending(), None);
    }

    #[test]
    fn history_records_started_lines_in_order() {
        let mut queue = EvaQueue::new();
        for line in [EvaLine::UnitReady, EvaLine::Canceled] {
            queue.speak(line);
            queue.update();
            queue.finish_current();
        }
        assert_eq!(
            queue.history().iter().copied().collect::<Vec<_>>(),
            vec![EvaLine::UnitReady, EvaLine::Canceled]
        );
    }
}
