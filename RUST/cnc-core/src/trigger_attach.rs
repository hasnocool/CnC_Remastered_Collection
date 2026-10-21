//! Attachment model for parsed trigger records.
//!
//! [`crate::trigger_records`] owns the vocabulary and
//! [`crate::trigger_apply`] owns execution, but neither answers the two
//! questions a mission runtime asks every tick: *what is this trigger bound
//! to?* and *has it already had its turn?* This module answers both.
//!
//! * [`AttachTarget`] names the container a trigger is bound to: the whole
//!   scenario, one house, a team type, one object instance, or one map cell
//!   (`[CellTriggers]`). [`TriggerSubject`] is the occurrence an engine
//!   reports for one evaluation pass; an attachment only evaluates when its
//!   target accepts the subject, so two triggers bound to different cells
//!   cannot cross-fire.
//! * [`TriggerAttachments`] is the registry: `attach` returns a monotonic
//!   [`AttachmentId`], `detach` removes exactly that binding, and duplicate
//!   bindings of the same trigger stay independent. Identity is the id, never
//!   the trigger number or the target, so detaching one cell binding cannot
//!   silently disarm another.
//! * [`TriggerMode`] separates the original engines' two semantics: a
//!   `OneShot` attachment latches after its first successful fire, a
//!   `Persistent` attachment (`IsPersistant` > 0 in the original records)
//!   stays armed and fires again every time its condition holds.
//!
//! Evaluation is deliberately thin: [`TriggerAttachments::dispatch`] delegates
//! conditions to [`crate::trigger_apply::events_hold`] and actions to
//! [`crate::trigger_apply::apply_actions`], which means an
//! [unknown event code](crate::trigger_records::EventKind::Unknown) never
//! fires (and is logged, not panicked on), exactly like the record layer.
//! Ordering is registration order, so identical scenarios replay identically.
//!
//! Reference: `REDALERT/TACTION.H`, `REDALERT/TEVENT.H`,
//! `TIBERIANDAWN/TRIGGER.CPP` (`TriggerClass::IsPersistant`),
//! `REDALERT/DISPLAY.CPP` (`[CellTriggers]` binding).

use std::fmt;

use crate::math::Vec2i;
use crate::trigger_apply::{apply_actions, events_hold, ApplyReport, TriggerHost};
use crate::trigger_records::{ActionRecord, EventRecord};

/// Stable identity of one attachment in a [`TriggerAttachments`] registry.
///
/// Ids are monotonic and never reused, so a stale id from a detached (or
/// destroyed) trigger cannot address a newer attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentId(u64);

impl AttachmentId {
    /// The raw counter value (useful for save games and log lines).
    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl fmt::Display for AttachmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "attachment#{}", self.0)
    }
}

/// The container a trigger is bound to.
///
/// `Global` covers mission triggers that are not attached to anything (the
/// original `TriggerClass` with no house/team/object owner); the other
/// variants mirror the containers the engines actually have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttachTarget {
    /// Scenario-global trigger, evaluated whenever the engine reports a
    /// global occurrence.
    Global,
    /// A house's trigger, evaluated for that house's occurrences.
    House(u8),
    /// A team-type trigger (`[TeamTypes]` logical id).
    Team(u16),
    /// A single object instance trigger (unit/building).
    Object(u32),
    /// A cell trigger (`[CellTriggers]`).
    Cell(Vec2i),
}

impl AttachTarget {
    /// True when `subject` is the kind of occurrence this target listens to.
    ///
    /// Comparison is exact: a cell trigger does not react to a house-level
    /// occurrence and vice versa.
    pub fn accepts(self, subject: TriggerSubject) -> bool {
        match (self, subject) {
            (Self::Global, TriggerSubject::Global) => true,
            (Self::House(attached), TriggerSubject::House(reported)) => attached == reported,
            (Self::Team(attached), TriggerSubject::Team(reported)) => attached == reported,
            (Self::Object(attached), TriggerSubject::Object(reported)) => attached == reported,
            (Self::Cell(attached), TriggerSubject::Cell(reported)) => {
                attached.x == reported.x && attached.y == reported.y
            }
            _ => false,
        }
    }
}

/// One occurrence an engine reports for an evaluation pass.
///
/// The engine chooses the subject from its own state (a unit stepping onto a
/// cell, a destroyed object, a house credits milestone, ...) and dispatches;
/// every attachment bound to a matching target gets a chance to fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerSubject {
    /// A scenario-global occurrence (timer, credits, ...).
    Global,
    /// An occurrence belonging to one house.
    House(u8),
    /// An occurrence belonging to one team type.
    Team(u16),
    /// An occurrence involving one object instance.
    Object(u32),
    /// A player unit entered/occupied a cell.
    Cell(Vec2i),
}

impl TriggerSubject {
    /// The [`AttachTarget`] with the same shape and value, if any.
    pub fn as_target(self) -> AttachTarget {
        match self {
            Self::Global => AttachTarget::Global,
            Self::House(house) => AttachTarget::House(house),
            Self::Team(team) => AttachTarget::Team(team),
            Self::Object(object) => AttachTarget::Object(object),
            Self::Cell(cell) => AttachTarget::Cell(cell),
        }
    }
}

/// How often an attachment may fire.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum TriggerMode {
    /// Fire at most once; latched by the first successful fire.
    #[default]
    OneShot,
    /// Stay armed after firing and fire again on every later pass whose
    /// condition holds (`IsPersistant` > 0 in the original records).
    Persistent,
}

/// Everything needed to register one attachment.
///
/// Fields are public because engines build specs from their own trigger
/// tables; ids are assigned by the registry, never by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentSpec {
    /// Trigger logical id in the scenario's trigger table.
    pub trigger: u16,
    /// Container this trigger is bound to.
    pub target: AttachTarget,
    /// One-shot or persistent semantics.
    pub mode: TriggerMode,
    /// Parsed event records the condition needs; empty means "always holds".
    pub events: Vec<EventRecord>,
    /// Parsed action records applied on a fire, in written order.
    pub actions: Vec<ActionRecord>,
    /// `false` for a trigger that exists but is disarmed
    /// (`TACTION_DESTROY_TRIGGER` in the original engines clears the enabled
    /// flag).
    pub enabled: bool,
}

impl AttachmentSpec {
    /// A one-shot, enabled, condition-free attachment for `trigger`.
    pub fn new(trigger: u16, target: AttachTarget) -> Self {
        Self {
            trigger,
            target,
            mode: TriggerMode::OneShot,
            events: Vec::new(),
            actions: Vec::new(),
            enabled: true,
        }
    }

    /// Switch to persistent semantics.
    pub fn persistent(mut self) -> Self {
        self.mode = TriggerMode::Persistent;
        self
    }

    /// Switch to one-shot semantics.
    pub fn one_shot(mut self) -> Self {
        self.mode = TriggerMode::OneShot;
        self
    }

    /// Attach the condition records.
    pub fn with_events(mut self, events: Vec<EventRecord>) -> Self {
        self.events = events;
        self
    }

    /// Attach the action records.
    pub fn with_actions(mut self, actions: Vec<ActionRecord>) -> Self {
        self.actions = actions;
        self
    }

    /// Register disarmed.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// One live attachment: the spec plus its identity and latch state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerAttachment {
    /// Registry-assigned identity.
    pub id: AttachmentId,
    /// Trigger logical id in the scenario's trigger table.
    pub trigger: u16,
    /// Container this trigger is bound to.
    pub target: AttachTarget,
    /// One-shot or persistent semantics.
    pub mode: TriggerMode,
    /// Condition records.
    pub events: Vec<EventRecord>,
    /// Action records.
    pub actions: Vec<ActionRecord>,
    /// Enabled flag; a disabled attachment is skipped without evaluating.
    pub enabled: bool,
    /// Latched on the first successful fire. Cleared only by `rearm`.
    pub fired: bool,
}

impl TriggerAttachment {
    /// True when this attachment listens to `subject`.
    pub fn accepts(&self, subject: TriggerSubject) -> bool {
        self.target.accepts(subject)
    }

    /// True when the attachment may fire on this pass: enabled, and either
    /// persistent or not yet latched.
    pub fn is_armed(&self) -> bool {
        self.enabled && (self.mode == TriggerMode::Persistent || !self.fired)
    }

    /// Evaluate the condition records against the host.
    ///
    /// Delegates to [`events_hold`]: an empty condition holds, an event the
    /// host cannot evaluate does not hold, and an unknown event code does not
    /// hold (it is logged through the host instead of panicking).
    pub fn condition_holds<H: TriggerHost + ?Sized>(&self, host: &mut H) -> bool {
        events_hold(host, &self.events)
    }
}

/// The result of one attachment firing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentFiring {
    /// Which attachment fired.
    pub attachment: AttachmentId,
    /// Its trigger logical id.
    pub trigger: u16,
    /// Its container.
    pub target: AttachTarget,
    /// One report per action record, in written order.
    pub reports: Vec<ApplyReport>,
}

impl AttachmentFiring {
    /// Number of action records the host actually performed.
    pub fn applied(&self) -> usize {
        self.reports.iter().filter(|report| report.applied).count()
    }
}

/// A registry of live attachments, evaluated in registration order.
///
/// The registry owns only binding bookkeeping; the game state lives in the
/// [`TriggerHost`] passed to [`dispatch`](Self::dispatch).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TriggerAttachments {
    entries: Vec<TriggerAttachment>,
    next_id: u64,
}

impl TriggerAttachments {
    /// An empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register one spec and return its fresh identity.
    ///
    /// Attaching the same trigger to the same target twice creates two
    /// independent bindings with distinct ids; detaching one leaves the
    /// other armed (the original engines can do the same through
    /// `[CellTriggers]` plus object attachment).
    pub fn attach(&mut self, spec: AttachmentSpec) -> AttachmentId {
        let id = AttachmentId(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);
        self.entries.push(TriggerAttachment {
            id,
            trigger: spec.trigger,
            target: spec.target,
            mode: spec.mode,
            events: spec.events,
            actions: spec.actions,
            enabled: spec.enabled,
            fired: false,
        });
        id
    }

    /// Remove exactly one attachment by identity.
    ///
    /// Returns the removed attachment, or `None` when the id is stale.
    pub fn detach(&mut self, id: AttachmentId) -> Option<TriggerAttachment> {
        let index = self.entries.iter().position(|entry| entry.id == id)?;
        Some(self.entries.remove(index))
    }

    /// Remove every attachment bound to `target`; returns how many were
    /// removed.
    pub fn detach_target(&mut self, target: AttachTarget) -> usize {
        let before = self.entries.len();
        self.entries.retain(|entry| entry.target != target);
        before - self.entries.len()
    }

    /// Remove every attachment for one trigger logical id; returns how many
    /// were removed.
    pub fn detach_trigger(&mut self, trigger: u16) -> usize {
        let before = self.entries.len();
        self.entries.retain(|entry| entry.trigger != trigger);
        before - self.entries.len()
    }

    /// Look one attachment up by identity.
    pub fn get(&self, id: AttachmentId) -> Option<&TriggerAttachment> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    /// Mutable lookup, for engine-side state sync (without changing identity).
    pub fn get_mut(&mut self, id: AttachmentId) -> Option<&mut TriggerAttachment> {
        self.entries.iter_mut().find(|entry| entry.id == id)
    }

    /// Whether `id` is still registered.
    pub fn contains(&self, id: AttachmentId) -> bool {
        self.get(id).is_some()
    }

    /// Number of registered attachments.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Every attachment, in registration order.
    pub fn iter(&self) -> impl Iterator<Item = &TriggerAttachment> {
        self.entries.iter()
    }

    /// Every attachment bound to `target`, in registration order.
    pub fn for_target(&self, target: AttachTarget) -> impl Iterator<Item = &TriggerAttachment> {
        self.entries
            .iter()
            .filter(move |entry| entry.target == target)
    }

    /// Every attachment for one trigger logical id, in registration order.
    pub fn for_trigger(&self, trigger: u16) -> impl Iterator<Item = &TriggerAttachment> {
        self.entries
            .iter()
            .filter(move |entry| entry.trigger == trigger)
    }

    /// Arm or disarm one attachment without touching its identity or latch.
    pub fn set_enabled(&mut self, id: AttachmentId, enabled: bool) -> bool {
        match self.get_mut(id) {
            Some(entry) => {
                entry.enabled = enabled;
                true
            }
            None => false,
        }
    }

    /// Clear the one-shot latch so the attachment can fire again.
    pub fn rearm(&mut self, id: AttachmentId) -> bool {
        match self.get_mut(id) {
            Some(entry) => {
                entry.fired = false;
                true
            }
            None => false,
        }
    }

    /// Evaluate and fire every attachment that listens to `subject`.
    ///
    /// For each attachment in registration order: skip when disabled, bound
    /// to another container, or already spent (one-shot); otherwise evaluate
    /// its conditions against `host` and, when they hold, apply its actions in
    /// order and latch `fired`. Persistent attachments stay armed and can
    /// appear again on the next dispatch.
    pub fn dispatch<H: TriggerHost + ?Sized>(
        &mut self,
        host: &mut H,
        subject: TriggerSubject,
    ) -> Vec<AttachmentFiring> {
        let mut firings = Vec::new();
        for entry in &mut self.entries {
            if !entry.is_armed() || !entry.accepts(subject) {
                continue;
            }
            if !entry.condition_holds(host) {
                continue;
            }
            let reports = apply_actions(host, &entry.actions);
            entry.fired = true;
            firings.push(AttachmentFiring {
                attachment: entry.id,
                trigger: entry.trigger,
                target: entry.target,
                reports,
            });
        }
        firings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_accepts_only_its_own_subject_shape() {
        let cell = Vec2i::new(3, 4);
        assert!(AttachTarget::Cell(cell).accepts(TriggerSubject::Cell(cell)));
        assert!(!AttachTarget::Cell(cell).accepts(TriggerSubject::Cell(Vec2i::new(4, 3))));
        assert!(!AttachTarget::Cell(cell).accepts(TriggerSubject::Global));
        assert!(AttachTarget::House(1).accepts(TriggerSubject::House(1)));
        assert!(!AttachTarget::House(1).accepts(TriggerSubject::House(0)));
        assert!(AttachTarget::Global.accepts(TriggerSubject::Global));
        assert!(!AttachTarget::Global.accepts(TriggerSubject::House(0)));
        assert_eq!(
            TriggerSubject::Cell(cell).as_target(),
            AttachTarget::Cell(cell)
        );
    }
}
