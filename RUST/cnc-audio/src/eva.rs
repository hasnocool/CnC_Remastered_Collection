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

/// One EVA speech event, in the original `VoxType` order.
///
/// Commented-out entries in the original enum are omitted, so the discriminant
/// values here are NOT the original `VOX_*` integers; use
/// [`EvaLine::voice_file`] or [`EvaLine::from_name`] rather than casting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
