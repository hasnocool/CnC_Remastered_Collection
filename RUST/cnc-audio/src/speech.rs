//! Speech (EVA / VOX) priority model and the deterministic voice queue.
//!
//! The two games have separate vocabularies ([crate::eva::EvaLine] for
//! Tiberian Dawn, [crate::eva_ra::RaEvaLine] for Red Alert) but share one
//! protocol, reproduced here:
//!
//! * a request only *queues*; playback starts from
//!   [`SpeechQueue::advance`], which the game loop calls once per tick
//!   (`Speak_AI` in `AUDIO.CPP`);
//! * a line that has no sample is skipped silently instead of stalling the
//!   queue (the retail tables contain `"none"` slots);
//! * a strictly higher-priority request preempts the playing line and the
//!   pending queue is kept ordered by `(priority desc, arrival asc)`, so the
//!   outcome of any event sequence is deterministic;
//! * duplicate lines are dropped: EVA never repeats the line it is already
//!   saying or one that is already waiting.
//!
//! The original speaks every line at card priority `254` and keeps exactly
//! one pending voice. This port keeps a small ordered backlog instead, which
//! lets a slow line be interrupted by an important one (a base attack must
//! not wait behind three "unit ready" calls). The priorities below are the
//! port's model, in Westwood's 0-255 priority space, and are exported so the
//! game/launcher layers (todos 40/42) can reason about them.

use std::collections::VecDeque;
use std::fmt;

use crate::eva::{EvaLine, TdEvent};
use crate::eva_ra::{RaEvaLine, RaEvent};
use crate::mixer::{AudioError, AudioMixer, Channel, SfxHandle};
use crate::sfx::SoundEffect;

/// Default number of lines that may wait behind the playing one.
pub const DEFAULT_PENDING_CAPACITY: usize = 3;
/// How many finished/interrupted/skipped lines the queue remembers.
pub const DEFAULT_HISTORY_LEN: usize = 64;

/// Why a line is being spoken; selects the playback priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeechCategory {
    /// End of mission, victory, defeat.
    Mission,
    /// Campaign objective completed or failed.
    Objective,
    /// Base/HQ under attack.
    BaseAttack,
    /// Superweapon charge/ready/launch (ion cannon, nuke, chronosphere,
    /// iron curtain).
    Superweapon,
    /// Incoming threats and sightings (missiles, planes, convoys, flares).
    Warning,
    /// Own losses: units, structures, aircraft, ships.
    Combat,
    /// Rescues and infiltrations.
    Rescue,
    /// Intel assets: spy plane, sonar, satellite, stolen credits.
    Intel,
    /// Power level complaints.
    Power,
    /// Money problems.
    Economy,
    /// Production feedback ("construction complete", "unit ready").
    Production,
    /// Mission countdown warnings.
    Time,
    /// Everything else ("canceled", "on hold", "select target").
    Info,
}

impl SpeechCategory {
    /// Westwood-style playback priority (0-255); higher wins.
    pub fn priority(self) -> u8 {
        match self {
            Self::Mission => 250,
            Self::Objective => 240,
            Self::BaseAttack => 232,
            Self::Superweapon => 224,
            Self::Warning => 216,
            Self::Time => 208,
            Self::Combat => 200,
            Self::Rescue => 196,
            Self::Intel => 192,
            Self::Power => 184,
            Self::Economy => 176,
            Self::Production => 168,
            Self::Info => 128,
        }
    }

    /// Short stable name (logging, saved log inspection).
    pub fn name(self) -> &'static str {
        match self {
            Self::Mission => "mission",
            Self::Objective => "objective",
            Self::BaseAttack => "base-attack",
            Self::Superweapon => "superweapon",
            Self::Warning => "warning",
            Self::Combat => "combat",
            Self::Rescue => "rescue",
            Self::Intel => "intel",
            Self::Power => "power",
            Self::Economy => "economy",
            Self::Production => "production",
            Self::Time => "time",
            Self::Info => "info",
        }
    }
}

impl fmt::Display for SpeechCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Everything the queue needs from a game's speech vocabulary.
pub trait EvaVocabulary: Copy + Eq + Ord + fmt::Debug + 'static {
    /// The `VOX_*` ordinal of the line inside the game's `Speech[]` table.
    fn voice_id(self) -> u16;
    /// Sample file base name, `None` when the game table holds `"none"`.
    fn sample_file(self) -> Option<&'static str>;
    /// Speech category, and through it the playback priority.
    fn category(self) -> SpeechCategory;
    /// Sample file with the `.AUD` extension.
    fn sample_name(self) -> Option<String> {
        self.sample_file().map(|file| format!("{file}.AUD"))
    }
    /// Playback priority of the line.
    fn priority(self) -> u8 {
        self.category().priority()
    }
}

/// What happened to a [`SpeechQueue::speak`] request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeakOutcome {
    /// The line is waiting in the pending queue.
    Queued,
    /// The same line is already playing or waiting; the request was dropped.
    Duplicate,
    /// The queue was full and the new line lost, so nothing changed.
    Dropped,
    /// The queue was full and the lowest-priority waiting line was replaced.
    Replaced,
}

/// A line the queue decided to play.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Playback<L: EvaVocabulary> {
    /// The line itself.
    pub line: L,
    /// `VOX_*` ordinal of the line.
    pub voice_id: u16,
    /// Sample base name (always present; silent lines never get this far).
    pub sample: &'static str,
    /// Speech category / priority bucket.
    pub category: SpeechCategory,
    /// Playback priority (0-255).
    pub priority: u8,
}

impl<L: EvaVocabulary> Playback<L> {
    /// Sample file with extension, e.g. `ACCOM1.AUD`.
    pub fn sample_name(&self) -> String {
        format!("{}.AUD", self.sample)
    }

    /// The mixer channel EVA always uses.
    pub fn channel(&self) -> Channel {
        Channel::Voice
    }

    /// Play the line's sample through an existing mixer on the voice channel.
    ///
    /// The decoder is supplied by the caller so the audio backend's concrete
    /// sample type does not leak into the speech API.
    pub fn play(&self, mixer: &AudioMixer, sample: &SoundEffect) -> Result<SfxHandle, AudioError> {
        sample.play_on(mixer, self.channel(), self.priority, 1.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pending<L> {
    line: L,
    priority: u8,
    seq: u64,
}

/// Deterministic EVA voice queue shared by both games.
///
/// Ordering rule: waiting lines are sorted by priority (descending) and
/// arrival sequence (ascending), so equal priorities play in request order.
/// Capacity overflow drops the newest lowest-priority line, and a line that
/// is already playing/waiting is never queued twice.
#[derive(Debug, Clone)]
pub struct SpeechQueue<L: EvaVocabulary> {
    capacity: usize,
    history_len: usize,
    current: Option<Pending<L>>,
    pending: Vec<Pending<L>>,
    next_seq: u64,
    skipped: VecDeque<L>,
    history: VecDeque<L>,
}

impl<L: EvaVocabulary> Default for SpeechQueue<L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: EvaVocabulary> SpeechQueue<L> {
    /// Queue with [`DEFAULT_PENDING_CAPACITY`].
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_PENDING_CAPACITY)
    }

    /// Queue with an explicit pending capacity (clamped to at least 1).
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            history_len: DEFAULT_HISTORY_LEN,
            current: None,
            pending: Vec::new(),
            next_seq: 0,
            skipped: VecDeque::new(),
            history: VecDeque::new(),
        }
    }

    /// Request a line (`Speak`).
    pub fn speak(&mut self, line: L) -> SpeakOutcome {
        if self.current.is_some_and(|current| current.line == line)
            || self.pending.iter().any(|entry| entry.line == line)
        {
            return SpeakOutcome::Duplicate;
        }
        let priority = line.priority();
        let seq = self.next_seq;
        self.next_seq += 1;
        self.pending.push(Pending {
            line,
            priority,
            seq,
        });
        self.sort_pending();
        if self.pending.len() <= self.capacity {
            return SpeakOutcome::Queued;
        }
        // The last entry is the newest lowest-priority request.
        let dropped = self.pending.pop();
        debug_assert!(dropped.is_some());
        if dropped.is_some_and(|entry| entry.seq == seq) {
            SpeakOutcome::Dropped
        } else {
            SpeakOutcome::Replaced
        }
    }

    fn sort_pending(&mut self) {
        self.pending
            .sort_by(|a, b| b.priority.cmp(&a.priority).then(a.seq.cmp(&b.seq)));
    }

    /// The line currently playing, if any.
    pub fn current(&self) -> Option<L> {
        self.current.map(|entry| entry.line)
    }

    /// Every waiting line, in play order.
    pub fn pending(&self) -> impl ExactSizeIterator<Item = L> + '_ {
        self.pending.iter().map(|entry| entry.line)
    }

    /// Number of waiting lines.
    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    /// Whether something is playing or waiting.
    pub fn is_busy(&self) -> bool {
        self.current.is_some() || !self.pending.is_empty()
    }

    /// Advance the queue (`Speak_AI`).
    ///
    /// `current_finished` is the caller's report that the playing sample
    /// ended. A waiting line of strictly higher priority preempts the current
    /// one even when `current_finished` is false. Lines without a sample are
    /// consumed silently and recorded in [`Self::skipped`], never returned as
    /// playback and never a panic.
    pub fn advance(&mut self, current_finished: bool) -> Option<Playback<L>> {
        if current_finished {
            self.current = None;
        } else if let (Some(current), Some(next)) = (self.current, self.pending.first()) {
            if next.priority > current.priority {
                self.current = None;
            }
        }
        if self.current.is_some() {
            return None;
        }
        while let Some(entry) = self.pending.first().copied() {
            self.pending.remove(0);
            if entry.line.sample_file().is_none() {
                push_bounded(&mut self.skipped, entry.line, DEFAULT_HISTORY_LEN);
                continue;
            }
            self.current = Some(entry);
            push_bounded(&mut self.history, entry.line, self.history_len);
            return Some(Playback {
                line: entry.line,
                voice_id: entry.line.voice_id(),
                sample: entry
                    .line
                    .sample_file()
                    .expect("silent lines were filtered out"),
                category: entry.line.category(),
                priority: entry.priority,
            });
        }
        None
    }

    /// Drop everything (`Stop_Speaking`).
    pub fn stop(&mut self) {
        self.current = None;
        self.pending.clear();
    }

    /// Silent lines consumed by [`Self::advance`], oldest first.
    pub fn skipped(&self) -> &VecDeque<L> {
        &self.skipped
    }

    /// Lines that started playing (or were preempted), oldest first.
    pub fn history(&self) -> &VecDeque<L> {
        &self.history
    }
}

fn push_bounded<L>(log: &mut VecDeque<L>, item: L, limit: usize) {
    log.push_back(item);
    if log.len() > limit {
        log.pop_front();
    }
}

/// A game's EVA front end: event mapping + queue + mixer plumbing.
#[derive(Debug, Clone)]
pub struct SpeechDirector<L: EvaVocabulary> {
    queue: SpeechQueue<L>,
}

impl<L: EvaVocabulary> Default for SpeechDirector<L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: EvaVocabulary> SpeechDirector<L> {
    /// Director with the default backlog size.
    pub fn new() -> Self {
        Self {
            queue: SpeechQueue::new(),
        }
    }

    /// Director with an explicit backlog size.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: SpeechQueue::with_capacity(capacity),
        }
    }

    /// The underlying queue.
    pub fn queue(&self) -> &SpeechQueue<L> {
        &self.queue
    }

    /// Mutable access to the queue.
    pub fn queue_mut(&mut self) -> &mut SpeechQueue<L> {
        &mut self.queue
    }

    /// Queue one line.
    pub fn speak(&mut self, line: L) -> SpeakOutcome {
        self.queue.speak(line)
    }

    /// Advance the queue; see [`SpeechQueue::advance`].
    pub fn advance(&mut self, current_finished: bool) -> Option<Playback<L>> {
        self.queue.advance(current_finished)
    }

    /// Drop everything.
    pub fn stop(&mut self) {
        self.queue.stop();
    }
}

/// Tiberian Dawn EVA: maps game events onto [`EvaLine`]s.
#[derive(Debug, Clone, Default)]
pub struct TdSpeechDirector {
    inner: SpeechDirector<EvaLine>,
}

impl TdSpeechDirector {
    /// New director.
    pub fn new() -> Self {
        Self::default()
    }

    /// New director with an explicit backlog size.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: SpeechDirector::with_capacity(capacity),
        }
    }

    /// Queue the EVA line for a game event.
    ///
    /// Returns `None` when the event has no EVA line at all (e.g. a line the
    /// game never speaks), otherwise the queue outcome.
    pub fn event(&mut self, event: TdEvent) -> Option<SpeakOutcome> {
        let line = EvaLine::from_event(event)?;
        Some(self.inner.speak(line))
    }

    /// Queue a line directly.
    pub fn speak(&mut self, line: EvaLine) -> SpeakOutcome {
        self.inner.speak(line)
    }

    /// Advance the queue.
    pub fn advance(&mut self, current_finished: bool) -> Option<Playback<EvaLine>> {
        self.inner.advance(current_finished)
    }

    /// Drop everything.
    pub fn stop(&mut self) {
        self.inner.stop();
    }

    /// The underlying queue.
    pub fn queue(&self) -> &SpeechQueue<EvaLine> {
        self.inner.queue()
    }
}

/// Red Alert EVA: maps game events onto [`RaEvaLine`]s.
#[derive(Debug, Clone, Default)]
pub struct RaSpeechDirector {
    inner: SpeechDirector<RaEvaLine>,
}

impl RaSpeechDirector {
    /// New director.
    pub fn new() -> Self {
        Self::default()
    }

    /// New director with an explicit backlog size.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: SpeechDirector::with_capacity(capacity),
        }
    }

    /// Queue the EVA line for a game event.
    ///
    /// Returns `None` when the event has no audible line in Red Alert, or the
    /// mapped `VOX_*` slot is one of the game's `"none"` entries.
    pub fn event(&mut self, event: RaEvent) -> Option<SpeakOutcome> {
        let line = RaEvaLine::from_event(event)?;
        Some(self.inner.speak(line))
    }

    /// Queue a line directly.
    pub fn speak(&mut self, line: RaEvaLine) -> SpeakOutcome {
        self.inner.speak(line)
    }

    /// Advance the queue.
    pub fn advance(&mut self, current_finished: bool) -> Option<Playback<RaEvaLine>> {
        self.inner.advance(current_finished)
    }

    /// Drop everything.
    pub fn stop(&mut self) {
        self.inner.stop();
    }

    /// The underlying queue.
    pub fn queue(&self) -> &SpeechQueue<RaEvaLine> {
        self.inner.queue()
    }
}
