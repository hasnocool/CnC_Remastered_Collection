// Public EVA emission queue with one-slot pending semantics and per-line cooldowns.
//
// Engine-independent. The game loop owns the time advance (`tick`) and the per-line
// cooldown manager prevents re-triggering the same line too frequently. One-slot pending
// semantics: a second line while one is pending is dropped, not queued.
//
// Original anchors:
//   * AUDIO.CPP: Speak() queues at most one pending voice; a second call while
//     SpeakQueue is non-none is ignored (line is dropped, not stacked).
//   * HOUSE.CPP SpeakAttackDelay = (TICKS_PER_MINUTE/2)+(TICKS_PER_SECOND*5) => ~35 s
//   * HOUSE.CPP SpeakMoneyDelay for insufficient funds = 1 minute (TD remaster) / 2 minutes
//     original; tests use the plan-named 60 s constant.
//
// Design notes:
//   * The queue mirrors `EvaQueue` (one current + one pending) but adds
//     per-line cooldown tracking and a `tick(seconds)` advance to expire cooldowns.
//   * Cooldowns are per-line (not per-category) matching the original per-voice delay
//     tables. Values chosen per plan: BaseUnderAttack 35 s, InsufficientFunds 60 s.
//   * Duplicate line suppression: `speak` rejects a request while a line is already
//     pending or identical to current (mirrors AUDIO.CPP Speak guard).
//   * Public API exposes `speak`, `pending()`, `current()`, `tick`, `advance` (Speak_AI
//     equivalent), `finish_current`, `stop`, `history`.
use crate::speech::EvaVocabulary;
use std::collections::HashMap;
use std::collections::VecDeque;

/// Cooldown configuration for a line; seconds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cooldown {
    pub seconds: f32,
}

/// One-slot EVA emission queue with per-line cooldowns.
///
/// The queue mirrors the original `Speak`/`Speak_AI` protocol:
///   * `speak(line)` returns `true` iff the line was accepted into the pending slot.
///     A second call while `pending` is occupied returns `false` (dropped).
///   * `advance()` moves the pending line into current (Speak_AI start).
///   * `finish_current()` clears the current line (sample ended).
///   * `tick(dt)` advances internal cooldown timers.
///
/// Cooldowns are keyed by the line itself; a line that is still on cooldown is silently
/// rejected by `speak`. The game layer decides *when* to call `speak`; the queue does not
/// invent events.
#[derive(Debug)]
pub struct EvaEmissionQueue<L: EvaVocabulary + std::hash::Hash> {
    current: Option<L>,
    pending: Option<L>,
    last_spoken_at: HashMap<L, f32>,
    cooldowns: HashMap<L, f32>,
    time: f32,
    history: VecDeque<L>,
}
impl<L: EvaVocabulary + std::hash::Hash> Default for EvaEmissionQueue<L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: EvaVocabulary + std::hash::Hash> EvaEmissionQueue<L> {
    pub const MAX_HISTORY: usize = 64;

    /// Create a new queue with no cooldown configuration.
    pub fn new() -> Self {
        Self {
            current: None,
            pending: None,
            last_spoken_at: HashMap::new(),
            cooldowns: HashMap::new(),
            time: 0.0,
            history: VecDeque::new(),
        }
    }

    /// Set a per-line cooldown (seconds). Called by the engine at startup.
    pub fn set_cooldown(&mut self, line: L, seconds: f32) {
        if seconds > 0.0 {
            self.cooldowns.insert(line, seconds);
        } else {
            self.cooldowns.remove(&line);
        }
    }

    /// Advance time. Cooldown state is time-based; call once per game tick.
    pub fn tick(&mut self, dt_seconds: f32) {
        self.time += dt_seconds;
    }

    /// Request a line. Returns `true` if accepted into pending slot.
    ///
    /// Rejection reasons:
    ///   * a pending line already exists (one-slot semantics)
    ///   * the same line is already playing
    ///   * line is on cooldown
    pub fn speak(&mut self, line: L) -> bool {
        // Idempotence: same line playing
        if self.current == Some(line) {
            return false;
        }
        // One-slot pending
        if self.pending.is_some() {
            return false;
        }
        // Cooldown check
        if let Some(&cooldown) = self.cooldowns.get(&line) {
            if let Some(&last) = self.last_spoken_at.get(&line) {
                if self.time - last < cooldown {
                    return false;
                }
            }
        }
        self.pending = Some(line);
        true
    }

    /// The line currently "playing", if any.
    pub fn current(&self) -> Option<L> {
        self.current
    }

    /// The line waiting for the current one to finish.
    pub fn pending(&self) -> Option<L> {
        self.pending
    }

    /// Advance to the next line (`Speak_AI`). Returns the line that started.
    pub fn advance(&mut self) -> Option<L> {
        if self.current.is_some() {
            return None;
        }
        let line = self.pending.take()?;
        self.current = Some(line);
        self.last_spoken_at.insert(line, self.time);
        self.history.push_back(line);
        if self.history.len() > Self::MAX_HISTORY {
            self.history.pop_front();
        }
        Some(line)
    }

    /// Mark the playing line finished so the next one may start.
    pub fn finish_current(&mut self) {
        self.current = None;
    }

    /// Clear both slots.
    pub fn stop(&mut self) {
        self.current = None;
        self.pending = None;
    }

    /// History of started lines.
    pub fn history(&self) -> &VecDeque<L> {
        &self.history
    }

    /// Time accessor (useful for tests).
    #[cfg(test)]
    pub fn time(&self) -> f32 {
        self.time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eva::EvaLine;
    use crate::eva_ra::RaEvaLine;

    #[test]
    fn eva_queue_drops_second_line_when_slot_busy() {
        let mut q: EvaEmissionQueue<RaEvaLine> = EvaEmissionQueue::new();
        assert!(q.speak(RaEvaLine::UnitReady));
        assert!(!q.speak(RaEvaLine::ConstructionComplete));
        assert_eq!(q.pending(), Some(RaEvaLine::UnitReady));
    }

    #[test]
    fn eva_queue_suppresses_repeat_during_cooldown() {
        let mut q: EvaEmissionQueue<RaEvaLine> = EvaEmissionQueue::new();
        q.set_cooldown(RaEvaLine::BaseUnderAttack, 35.0);
        assert!(q.speak(RaEvaLine::BaseUnderAttack));
        q.advance();
        q.finish_current();
        assert!(!q.speak(RaEvaLine::BaseUnderAttack));
        q.tick(35.0);
        assert!(q.speak(RaEvaLine::BaseUnderAttack));
    }

    #[test]
    fn eva_queue_reports_pending_line_for_duration() {
        let mut q: EvaEmissionQueue<EvaLine> = EvaEmissionQueue::new();
        assert!(q.pending().is_none());
        q.speak(EvaLine::UnitReady);
        assert_eq!(q.pending(), Some(EvaLine::UnitReady));
        assert_eq!(q.current(), None);
        let started = q.advance();
        assert_eq!(started, Some(EvaLine::UnitReady));
        assert_eq!(q.current(), Some(EvaLine::UnitReady));
        assert_eq!(q.pending(), None);
    }

    #[test]
    fn insufficient_funds_cooldown_60s() {
        let mut q: EvaEmissionQueue<EvaLine> = EvaEmissionQueue::new();
        q.set_cooldown(EvaLine::InsufficientFunds, 60.0);
        assert!(q.speak(EvaLine::InsufficientFunds));
        q.advance();
        q.finish_current();
        assert!(!q.speak(EvaLine::InsufficientFunds));
        q.tick(59.9);
        assert!(!q.speak(EvaLine::InsufficientFunds));
        q.tick(0.2);
        assert!(q.speak(EvaLine::InsufficientFunds));
    }

    #[test]
    fn ra_from_event_structure_destroyed_maps_to_structure_lost() {
        use crate::eva_ra::{RaEvaLine, RaEvent};
        let line = RaEvaLine::from_event(RaEvent::StructureDestroyed).unwrap();
        assert_eq!(line, RaEvaLine::StructureDestroyed);
    }

    #[test]
    fn ra_from_event_insufficient_funds_maps_correctly() {
        use crate::eva_ra::{RaEvaLine, RaEvent};
        let line = RaEvaLine::from_event(RaEvent::InsufficientFunds).unwrap();
        assert_eq!(line, RaEvaLine::InsufficientFunds);
    }

    #[test]
    fn ra_from_event_mission_accomplished_maps_correctly() {
        use crate::eva_ra::{RaEvaLine, RaEvent};
        let line = RaEvaLine::from_event(RaEvent::MissionAccomplished).unwrap();
        assert_eq!(line, RaEvaLine::MissionAccomplished);
    }

    #[test]
    fn ra_from_event_construction_complete_maps_correctly() {
        use crate::eva_ra::{RaEvaLine, RaEvent};
        let line = RaEvaLine::from_event(RaEvent::ConstructionComplete).unwrap();
        assert_eq!(line, RaEvaLine::ConstructionComplete);
    }

    #[test]
    fn ra_from_event_unit_lost_maps_correctly() {
        use crate::eva_ra::{RaEvaLine, RaEvent};
        let line = RaEvaLine::from_event(RaEvent::UnitLost).unwrap();
        assert_eq!(line, RaEvaLine::UnitLost);
    }

    #[test]
    fn ra_from_event_base_under_attack_maps_correctly() {
        use crate::eva_ra::{RaEvaLine, RaEvent};
        let line = RaEvaLine::from_event(RaEvent::BaseUnderAttack).unwrap();
        assert_eq!(line, RaEvaLine::BaseUnderAttack);
    }

    #[test]
    fn ra_from_event_low_power_maps_correctly() {
        use crate::eva_ra::{RaEvaLine, RaEvent};
        let line = RaEvaLine::from_event(RaEvent::LowPower).unwrap();
        assert_eq!(line, RaEvaLine::LowPower);
    }

    #[test]
    fn ra_from_event_mission_failed_maps_correctly() {
        use crate::eva_ra::{RaEvaLine, RaEvent};
        let line = RaEvaLine::from_event(RaEvent::MissionFailed).unwrap();
        assert_eq!(line, RaEvaLine::MissionFailed);
    }
}
