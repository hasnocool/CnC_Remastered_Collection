//! Integration tests for the trigger attachment model.
//!
//! Covers the contract an engine runtime relies on: attach/detach identity,
//! one-shot versus persistent semantics, deterministic condition evaluation
//! against a host, container scoping, and the "unknown event code never
//! fires, never panics" forward-compatibility rule.

use cnc_core::trigger_apply::TriggerHost;
use cnc_core::trigger_attach::{
    AttachTarget, AttachmentSpec, TriggerAttachments, TriggerMode, TriggerSubject,
};
use cnc_core::trigger_records::{ActionRecord, EventKind, EventRecord};
use cnc_core::Vec2i;

fn at(x: i32, y: i32) -> Vec2i {
    Vec2i::new(x, y)
}

fn event(record: &str) -> EventRecord {
    EventRecord::parse(record).expect("test event record must parse")
}

fn action(record: &str) -> ActionRecord {
    ActionRecord::parse(record).expect("test action record must parse")
}

/// Answers `ElapsedTime` from a flag, records applied wins, and sinks logs.
struct StubHost {
    elapsed: bool,
    wins: Vec<u8>,
    notes: Vec<String>,
}

impl StubHost {
    fn new(elapsed: bool) -> Self {
        Self {
            elapsed,
            wins: Vec::new(),
            notes: Vec::new(),
        }
    }
}

impl TriggerHost for StubHost {
    fn note(&mut self, message: String) {
        self.notes.push(message);
    }

    fn map_size(&self) -> (u32, u32) {
        (16, 16)
    }

    fn default_house(&self) -> u8 {
        0
    }

    fn house_count(&self) -> usize {
        2
    }

    fn evaluate_event(&mut self, kind: EventKind, value: i64) -> Option<bool> {
        match kind {
            EventKind::ElapsedTime => Some(self.elapsed && value >= 0),
            _ => None,
        }
    }

    fn win(&mut self, house: u8) -> bool {
        self.wins.push(house);
        true
    }
}

fn cell_trigger(trigger: u16, cell: Vec2i) -> AttachmentSpec {
    AttachmentSpec::new(trigger, AttachTarget::Cell(cell))
        .with_events(vec![event("13,-1,0")])
        .with_actions(vec![action("1,-1,-1,0")])
}

#[test]
fn attach_and_detach_preserve_binding_identity() {
    let cell = at(3, 4);
    let mut attachments = TriggerAttachments::new();

    let first = attachments.attach(cell_trigger(7, cell));
    let second = attachments.attach(cell_trigger(8, cell));
    assert_ne!(first, second, "each attach gets a fresh identity");
    assert_eq!(attachments.len(), 2);
    assert!(attachments.contains(first) && attachments.contains(second));

    let removed = attachments.detach(first).expect("first is still attached");
    assert_eq!(removed.id, first);
    assert_eq!(removed.trigger, 7);
    assert_eq!(removed.target, AttachTarget::Cell(cell));
    assert!(
        !attachments.contains(first),
        "detach removes exactly that id"
    );
    assert!(attachments.contains(second), "the sibling stays attached");
    assert_eq!(attachments.len(), 1);
    assert!(
        attachments.detach(first).is_none(),
        "a stale id cannot detach twice"
    );

    let third = attachments.attach(cell_trigger(7, cell));
    assert_ne!(third, first, "ids are never reused");
    assert_ne!(third, second);
    assert_eq!(attachments.for_trigger(7).count(), 1);
    assert_eq!(attachments.for_target(AttachTarget::Cell(cell)).count(), 2);
    assert_eq!(
        attachments.detach_target(AttachTarget::Cell(cell)),
        2,
        "target detach removes every binding to that cell"
    );
    assert!(attachments.is_empty());
}

#[test]
fn one_shot_fires_once_and_persistent_refires() {
    let cell = at(2, 2);
    let mut attachments = TriggerAttachments::new();
    let once = attachments.attach(cell_trigger(1, cell));
    let repeating = attachments.attach(cell_trigger(2, cell).persistent());
    let mut host = StubHost::new(true);

    let first = attachments.dispatch(&mut host, TriggerSubject::Cell(cell));
    let first_ids: Vec<_> = first.iter().map(|firing| firing.attachment).collect();
    assert_eq!(first_ids, vec![once, repeating], "registration order");
    assert_eq!(host.wins.len(), 2, "both actions applied");
    assert!(attachments.get(once).expect("once").fired);
    assert!(attachments.get(repeating).expect("repeating").fired);
    assert_eq!(first[0].reports[0].action, action("1,-1,-1,0").kind);

    let second = attachments.dispatch(&mut host, TriggerSubject::Cell(cell));
    let second_ids: Vec<_> = second.iter().map(|firing| firing.attachment).collect();
    assert_eq!(
        second_ids,
        vec![repeating],
        "one-shot stays spent, persistent fires again"
    );
    assert_eq!(host.wins.len(), 3);
    assert_eq!(
        attachments.get(once).expect("once").mode,
        TriggerMode::OneShot
    );

    let mut fresh = StubHost::new(true);
    let third = attachments.dispatch(&mut fresh, TriggerSubject::Cell(cell));
    assert_eq!(third.len(), 1, "persistent keeps firing on every pass");
}

#[test]
fn condition_evaluation_against_a_stub_host_is_deterministic() {
    let cell = at(1, 1);
    let build = || {
        let mut attachments = TriggerAttachments::new();
        let id = attachments.attach(cell_trigger(3, cell));
        (attachments, id)
    };

    let (attachments, id) = build();
    let mut host = StubHost::new(true);
    let entry = attachments.get(id).expect("attached");
    assert!(entry.condition_holds(&mut host));
    assert!(entry.condition_holds(&mut host), "same host, same verdict");
    assert!(host.notes.is_empty(), "a known event logs nothing");
    assert_eq!(
        entry.condition_holds(&mut host) as u8,
        entry.condition_holds(&mut host) as u8
    );

    let run = || {
        let (mut attachments, _) = build();
        let mut host = StubHost::new(true);
        let firings = attachments.dispatch(&mut host, TriggerSubject::Cell(cell));
        let ids: Vec<_> = firings.iter().map(|firing| firing.attachment).collect();
        (ids, host.wins, host.notes)
    };
    assert_eq!(run(), run(), "identical inputs replay identically");

    let (mut attachments, id) = build();
    let mut host = StubHost::new(false);
    let entry = attachments.get(id).expect("attached");
    assert!(!entry.condition_holds(&mut host));
    assert!(!entry.condition_holds(&mut host));
    assert!(attachments
        .dispatch(&mut host, TriggerSubject::Cell(cell))
        .is_empty());
    assert!(!attachments.get(id).expect("attached").fired);
}

#[test]
fn unknown_event_code_never_fires_and_never_panics() {
    let cell = at(5, 6);
    let mut attachments = TriggerAttachments::new();
    let id = attachments.attach(
        AttachmentSpec::new(9, AttachTarget::Cell(cell))
            .with_events(vec![event("99,-1,0")])
            .with_actions(vec![action("1,-1,-1,0")]),
    );
    let mut host = StubHost::new(true);

    assert!(
        attachments
            .dispatch(&mut host, TriggerSubject::Cell(cell))
            .is_empty(),
        "an event code outside the enumeration must not fire"
    );
    assert!(!attachments.get(id).expect("attached").fired);
    assert_eq!(host.wins.len(), 0);
    assert!(
        host.notes
            .iter()
            .any(|note| note.contains("outside the enumerated range")),
        "the unknown code is logged, not silently dropped: {:?}",
        host.notes
    );

    // A second pass behaves identically; the latch was not set by the unknown
    // condition, and there is still no panic.
    assert!(attachments
        .dispatch(&mut host, TriggerSubject::Cell(cell))
        .is_empty());
}

#[test]
fn target_scoping_prevents_cross_fire() {
    let cell = at(4, 4);
    let mut attachments = TriggerAttachments::new();
    let global = attachments.attach(AttachmentSpec::new(1, AttachTarget::Global));
    let house0 = attachments.attach(AttachmentSpec::new(2, AttachTarget::House(0)));
    let house1 = attachments.attach(AttachmentSpec::new(3, AttachTarget::House(1)));
    let team = attachments.attach(AttachmentSpec::new(4, AttachTarget::Team(3)));
    let object = attachments.attach(AttachmentSpec::new(5, AttachTarget::Object(11)));
    let cell_trigger_id = attachments.attach(AttachmentSpec::new(6, AttachTarget::Cell(cell)));
    let mut host = StubHost::new(true);

    let ids = |firings: Vec<cnc_core::trigger_attach::AttachmentFiring>| {
        firings
            .into_iter()
            .map(|firing| firing.attachment)
            .collect::<Vec<_>>()
    };

    assert_eq!(
        ids(attachments.dispatch(&mut host, TriggerSubject::Cell(cell))),
        vec![cell_trigger_id]
    );
    assert_eq!(
        ids(attachments.dispatch(&mut host, TriggerSubject::House(1))),
        vec![house1]
    );
    assert_eq!(
        ids(attachments.dispatch(&mut host, TriggerSubject::Global)),
        vec![global]
    );
    assert_eq!(
        ids(attachments.dispatch(&mut host, TriggerSubject::Team(3))),
        vec![team]
    );
    assert_eq!(
        ids(attachments.dispatch(&mut host, TriggerSubject::Object(11))),
        vec![object]
    );
    assert_eq!(
        ids(attachments.dispatch(&mut host, TriggerSubject::House(0))),
        vec![house0]
    );
    assert!(
        host.notes.is_empty(),
        "scoped-out triggers are not evaluated"
    );
}

#[test]
fn disabled_attachment_does_not_fire_until_armed() {
    let cell = at(7, 7);
    let mut attachments = TriggerAttachments::new();
    let id = attachments.attach(AttachmentSpec::new(8, AttachTarget::Cell(cell)).disabled());
    let mut host = StubHost::new(true);

    assert!(attachments
        .dispatch(&mut host, TriggerSubject::Cell(cell))
        .is_empty());
    assert!(attachments.set_enabled(id, true));
    assert_eq!(
        attachments
            .dispatch(&mut host, TriggerSubject::Cell(cell))
            .len(),
        1
    );
    assert!(attachments.contains(id), "arming keeps the same identity");
    assert!(attachments.rearm(id), "rearm clears the one-shot latch");
    assert_eq!(
        attachments
            .dispatch(&mut host, TriggerSubject::Cell(cell))
            .len(),
        1,
        "a rearmed one-shot fires once more"
    );
}

/// A host that reports success for a configured set of event kinds.
struct EventHost {
    true_events: Vec<cnc_core::trigger_records::EventKind>,
    wins: Vec<u8>,
    notes: Vec<String>,
}

impl EventHost {
    fn new(true_events: Vec<cnc_core::trigger_records::EventKind>) -> Self {
        Self {
            true_events,
            wins: Vec::new(),
            notes: Vec::new(),
        }
    }
}

impl cnc_core::trigger_apply::TriggerHost for EventHost {
    fn note(&mut self, message: String) {
        self.notes.push(message);
    }
    fn map_size(&self) -> (u32, u32) {
        (16, 16)
    }
    fn default_house(&self) -> u8 {
        0
    }
    fn house_count(&self) -> usize {
        2
    }
    fn evaluate_event(
        &mut self,
        kind: cnc_core::trigger_records::EventKind,
        _value: i64,
    ) -> Option<bool> {
        if self.true_events.contains(&kind) {
            Some(true)
        } else {
            None
        }
    }
    fn win(&mut self, house: u8) -> bool {
        self.wins.push(house);
        true
    }
}

#[test]
fn attacked_trigger_fires_once() {
    use cnc_core::trigger_attach::{
        AttachTarget, AttachmentSpec, TriggerAttachments, TriggerSubject,
    };
    use cnc_core::trigger_records::{ActionRecord, EventKind, EventRecord};
    let mut attachments = TriggerAttachments::new();
    let id = attachments.attach(
        AttachmentSpec::new(1, AttachTarget::Object(42))
            .with_events(vec![EventRecord::parse("6,-1,0").expect("attacked")])
            .with_actions(vec![ActionRecord::parse("1,-1,-1,0").expect("win")]),
    );
    let mut host = EventHost::new(vec![EventKind::Attacked]);
    let first = attachments.dispatch(&mut host, TriggerSubject::Object(42));
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].attachment, id);
    assert_eq!(host.wins.len(), 1);
    // Re-attack must not refire (one-shot latch).
    let second = attachments.dispatch(&mut host, TriggerSubject::Object(42));
    assert!(second.is_empty(), "attacked trigger fires only once");
    assert_eq!(host.wins.len(), 1);
}

#[test]
fn discovery_trigger_fires_on_first_sight_of_attached_house() {
    use cnc_core::trigger_attach::{
        AttachTarget, AttachmentSpec, TriggerAttachments, TriggerSubject,
    };
    use cnc_core::trigger_records::{ActionRecord, EventKind, EventRecord};
    let mut attachments = TriggerAttachments::new();
    let id = attachments.attach(
        AttachmentSpec::new(2, AttachTarget::House(1))
            .with_events(vec![EventRecord::parse("5,-1,1").expect("house discovered")])
            .with_actions(vec![ActionRecord::parse("1,-1,-1,0").expect("win")]),
    );
    let mut host = EventHost::new(vec![EventKind::HouseDiscovered]);
    let first = attachments.dispatch(&mut host, TriggerSubject::House(1));
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].attachment, id);
    let second = attachments.dispatch(&mut host, TriggerSubject::House(1));
    assert!(second.is_empty(), "discovery fires on first sight only");
}

#[test]
fn cell_bound_trigger_fires_when_unit_enters() {
    use cnc_core::trigger_attach::{
        AttachTarget, AttachmentSpec, TriggerAttachments, TriggerSubject,
    };
    use cnc_core::trigger_records::ActionRecord;
    use cnc_core::Vec2i;
    let cell = Vec2i::new(5, 5);
    let mut attachments = TriggerAttachments::new();
    let id = attachments.attach(
        AttachmentSpec::new(3, AttachTarget::Cell(cell))
            .with_events(vec![])
            .with_actions(vec![ActionRecord::parse("1,-1,-1,0").expect("win")]),
    );
    let mut host = EventHost::new(vec![]);
    let firings = attachments.dispatch(&mut host, TriggerSubject::Cell(cell));
    assert_eq!(firings.len(), 1);
    assert_eq!(firings[0].attachment, id);
}

#[test]
fn house_filtered_trigger_does_not_fire_for_non_matching_house() {
    use cnc_core::trigger_attach::{
        AttachTarget, AttachmentSpec, TriggerAttachments, TriggerSubject,
    };
    use cnc_core::trigger_records::{ActionRecord, EventKind, EventRecord};
    let mut attachments = TriggerAttachments::new();
    let _id = attachments.attach(
        AttachmentSpec::new(4, AttachTarget::House(0))
            .with_events(vec![EventRecord::parse("4,-1,0").expect("discovered")])
            .with_actions(vec![ActionRecord::parse("1,-1,-1,0").expect("win")]),
    );
    let mut host = EventHost::new(vec![EventKind::Discovered]);
    // Non-matching house must not fire.
    let wrong = attachments.dispatch(&mut host, TriggerSubject::House(1));
    assert!(
        wrong.is_empty(),
        "house filter suppresses non-matching house"
    );
    // Matching house fires.
    let right = attachments.dispatch(&mut host, TriggerSubject::House(0));
    assert_eq!(right.len(), 1);
}

#[test]
fn reattack_within_cooldown_does_not_refire() {
    use cnc_core::trigger_attach::{
        AttachTarget, AttachmentSpec, TriggerAttachments, TriggerSubject,
    };
    use cnc_core::trigger_records::{ActionRecord, EventKind, EventRecord};
    let mut attachments = TriggerAttachments::new();
    attachments.attach(
        AttachmentSpec::new(5, AttachTarget::Object(7))
            .with_events(vec![EventRecord::parse("6,-1,0").expect("attacked")])
            .with_actions(vec![ActionRecord::parse("1,-1,-1,0").expect("win")]),
    );
    let mut host = EventHost::new(vec![EventKind::Attacked]);
    let first = attachments.dispatch(&mut host, TriggerSubject::Object(7));
    assert_eq!(first.len(), 1);
    let second = attachments.dispatch(&mut host, TriggerSubject::Object(7));
    assert!(
        second.is_empty(),
        "re-attack within cooldown does not refire"
    );
}
