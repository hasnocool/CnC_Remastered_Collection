//! Phase 2.3 animation semantics (headless, no real time, no rendering).
//!
//! Contract:
//! - `advance(dt)` is the only progression mechanism (deterministic).
//! - `LoopMode::Loop` wraps around, `Once` stops at the last frame and
//!   emits `AnimationEvent::Finished` exactly once, `PingPong` reverses at
//!   both ends and keeps cycling.
//! - Frame events fire exactly once per arrival at their frame position
//!   (including the initial entry at construction-through-first-advance).
//! - The instance carries a palette id (player-remap passthrough) that is
//!   independent of frame progression and survives `reset`.

use cnc_asset::{AnimationEvent, AnimationInstance, AnimationSpec, FrameEvent, LoopMode};

fn loop_spec(frames: &[u32], duration: f32) -> AnimationSpec {
    AnimationSpec::new(frames.to_vec(), duration, LoopMode::Loop)
}

#[test]
fn loop_mode_wraps_around() {
    let spec = loop_spec(&[10, 20, 30], 1.0);
    let mut inst = AnimationInstance::new(spec);

    assert_eq!(inst.current_frame(), Some(10));
    assert_eq!(inst.position(), 0);
    assert_eq!(inst.wrap_count(), 0);

    inst.advance(1.0); // k 0 -> 1
    assert_eq!(inst.position(), 1);
    assert_eq!(inst.current_frame(), Some(20));

    inst.advance(2.0); // k 1 -> 3: wraps once (k=3 is a multiple of 3)
    assert_eq!(inst.position(), 0);
    assert_eq!(inst.current_frame(), Some(10));
    assert_eq!(inst.wrap_count(), 1);

    inst.advance(1.0); // k 3 -> 4
    assert_eq!(inst.current_frame(), Some(20));
    assert_eq!(inst.wrap_count(), 1);

    inst.advance(2.0); // k 4 -> 6: wrapped twice in total
    assert_eq!(inst.current_frame(), Some(10));
    assert_eq!(inst.wrap_count(), 2);
    assert!(!inst.finished());
}

#[test]
fn once_mode_finishes_and_stays_finished() {
    // 4 frames * 0.5s = 2.0s total.
    let spec = AnimationSpec::new(vec![0, 1, 2, 3], 0.5, LoopMode::Once);
    let mut inst = AnimationInstance::new(spec);

    inst.advance(1.2); // k = 2
    assert_eq!(inst.current_frame(), Some(2));
    assert!(!inst.finished());

    let events = inst.advance(0.9); // t = 2.1 -> past the end
    assert!(inst.finished());
    assert_eq!(inst.current_frame(), Some(3), "clamped to last frame");
    assert!(
        events == vec![AnimationEvent::Finished],
        "exactly one Finished event, got {events:?}"
    );

    // Further advances are no-ops and re-emit nothing.
    let events = inst.advance(10.0);
    assert!(inst.finished());
    assert_eq!(inst.current_frame(), Some(3));
    assert!(events.is_empty(), "no duplicate finish, got {events:?}");
}

#[test]
fn pingpong_reverses_at_both_ends() {
    // Cycle length 2n-2 = 4: 0,1,2,1,0,1,... The first advance moves from k=0
    // to k=1, so the post-advance sequence is the cycle shifted by one step.
    let spec = AnimationSpec::new(vec![1, 2, 3], 1.0, LoopMode::PingPong);
    let mut inst = AnimationInstance::new(spec);

    assert_eq!(inst.current_frame(), Some(1), "pre-advance playhead at k=0");
    let expected: [u32; 6] = [2, 3, 2, 1, 2, 3];
    let mut seen: [u32; 6] = [0; 6];
    for slot in &mut seen {
        inst.advance(1.0);
        *slot = inst.current_frame().expect("pingpong never finishes");
    }
    assert_eq!(seen, expected);
    assert!(!inst.finished());
}

#[test]
fn event_fires_exactly_once_per_arrival() {
    let spec =
        AnimationSpec::new(vec![0, 1, 2], 1.0, LoopMode::Loop).with_events(vec![FrameEvent {
            at_position: 1,
            id: 7,
        }]);
    let mut inst = AnimationInstance::new(spec);

    // Initial entry (position 0, no event) then arrivals at 1 and 2.
    let events = inst.advance(2.0); // k 0 -> 2: arrivals {1, 2}
    assert_eq!(
        events,
        vec![AnimationEvent::Triggered(FrameEvent {
            at_position: 1,
            id: 7
        })],
        "fires exactly once per arrival"
    );

    // Crossing two frames with no event frame between: nothing fires.
    let events = inst.advance(2.0); // k 2 -> 4: arrivals {0 (wrap), 1}
                                    // Arrival at position 1 fires again on the new pass (once per pass).
    assert_eq!(
        events,
        vec![AnimationEvent::Triggered(FrameEvent {
            at_position: 1,
            id: 7
        })],
        "loop pass re-fires exactly once"
    );
}

#[test]
fn event_at_initial_position_fires_on_first_advance() {
    let spec =
        AnimationSpec::new(vec![0, 1, 2], 1.0, LoopMode::Loop).with_events(vec![FrameEvent {
            at_position: 0,
            id: 42,
        }]);
    let mut inst = AnimationInstance::new(spec);

    // Not yet advanced: nothing fired.
    assert_eq!(inst.fired_count(42), 0);

    // First advance performs the initial entry into position 0 exactly once.
    let events = inst.advance(0.2); // k stays 0
    assert_eq!(
        events,
        vec![AnimationEvent::Triggered(FrameEvent {
            at_position: 0,
            id: 42
        })],
    );
    assert_eq!(inst.fired_count(42), 1);

    // Later arrives at position 0 (wrap) fire again, once per arrival.
    let events = inst.advance(2.8); // k 0 -> 3 wraps into position 0
    assert_eq!(
        events,
        vec![AnimationEvent::Triggered(FrameEvent {
            at_position: 0,
            id: 42
        })],
    );
    assert_eq!(inst.fired_count(42), 2);
}

#[test]
fn once_mode_events_fire_before_finish() {
    // 2 frames * 1.0s; event on the final frame.
    let spec = AnimationSpec::new(vec![5, 6], 1.0, LoopMode::Once).with_events(vec![FrameEvent {
        at_position: 1,
        id: 9,
    }]);
    let mut inst = AnimationInstance::new(spec);

    let events = inst.advance(2.5); // past the end
    assert!(inst.finished());
    assert_eq!(
        events,
        vec![
            AnimationEvent::Triggered(FrameEvent {
                at_position: 1,
                id: 9
            }),
            AnimationEvent::Finished,
        ],
        "final-frame event then finish, each exactly once"
    );
}

#[test]
fn palette_id_is_passthrough_independent_of_frames() {
    let spec = loop_spec(&[0, 1, 2], 0.1);
    let mut inst = AnimationInstance::new(spec).with_palette_id(Some(3));
    assert_eq!(inst.palette_id(), Some(3));

    inst.advance(0.25); // k = 2
    assert_eq!(inst.current_frame(), Some(2));
    assert_eq!(inst.palette_id(), Some(3), "survives progression");

    // Generic lookup receives both the spec-frame value and the palette id.
    let looked = inst.lookup_current(|frame, palette| (frame, palette));
    assert_eq!(looked, Some((2, Some(3))));

    inst.set_palette_id(Some(9));
    assert_eq!(
        inst.lookup_current(|frame, palette| (frame, palette)),
        Some((2, Some(9)))
    );

    let plain = AnimationInstance::new(loop_spec(&[4], 1.0));
    assert_eq!(plain.palette_id(), None);
    assert_eq!(plain.lookup_current(|_, palette| palette), Some(None));
}

#[test]
fn non_positive_dt_is_a_noop() {
    let spec = loop_spec(&[0, 1], 1.0);
    let mut inst = AnimationInstance::new(spec);
    inst.advance(0.5); // k = 0, halfway into the first frame
    let before = (inst.position(), inst.current_frame(), inst.wrap_count());

    let events = inst.advance(-1.0);
    assert!(events.is_empty());
    assert_eq!(
        (inst.position(), inst.current_frame(), inst.wrap_count()),
        before,
        "negative dt changes nothing",
    );
    let events = inst.advance(0.0);
    assert!(events.is_empty());
}

#[test]
fn reset_restores_initial_state_keeping_palette() {
    let spec =
        AnimationSpec::new(vec![0, 1, 2], 1.0, LoopMode::Once).with_events(vec![FrameEvent {
            at_position: 2,
            id: 3,
        }]);
    let mut inst = AnimationInstance::new(spec).with_palette_id(Some(5));

    inst.advance(3.0);
    assert!(inst.finished());
    assert_eq!(inst.fired_count(3), 1);

    inst.reset();
    assert_eq!(inst.current_frame(), Some(0));
    assert!(!inst.finished());
    assert_eq!(inst.position(), 0);
    assert_eq!(inst.fired_count(3), 0);
    assert_eq!(inst.palette_id(), Some(5));
}

#[test]
fn empty_spec_is_always_finished() {
    let mut inst = AnimationInstance::new(AnimationSpec::new(Vec::new(), 1.0, LoopMode::Loop));
    assert_eq!(inst.current_frame(), None);
    let events = inst.advance(2.0);
    assert!(inst.finished());
    assert_eq!(
        events,
        vec![AnimationEvent::Finished],
        "finishes on first advance"
    );
    // Sequential frame-duration clamp: invalid durations never stall the anim.
    let clamped = AnimationSpec::new(vec![0], 0.0, LoopMode::Once);
    assert!(clamped.frame_duration() > 0.0);
    let clamped = AnimationSpec::new(vec![0], -3.0, LoopMode::Once);
    assert!(clamped.frame_duration() > 0.0);
}
