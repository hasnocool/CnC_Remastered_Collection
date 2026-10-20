//! Headless animation playback for sprite frame sequences.
//!
//! This module is the phase 2.3 animation runtime. An [`AnimationSpec`]
//! describes a frame sequence — frame indices into any frame list, e.g. the
//! frames of a parsed [`crate::ShpFile`] — with a per-frame duration and a
//! loop mode. An [`AnimationInstance`] advances deterministically via
//! [`AnimationInstance::advance`]: no wall-clock time, no rendering, fully
//! unit-testable headless.
//!
//! Behaviour:
//! - [`AnimationInstance::current_frame`] reports the sprite-level frame value
//!   at the playhead.
//! - [`FrameEvent`] triggers fire exactly once per arrival at their frame
//!   position, including the initial entry into frame 0 on the first advance.
//! - [`AnimationInstance::palette_id`] is a palette/player-remap passthrough
//!   carried independently of progression; thread it through sprite lookups
//!   with [`AnimationInstance::lookup_current`].
//!
//! Loop semantics (`n` = frame count, `k` = integer arrivals since start,
//! `d` = frame duration):
//! - [`LoopMode::Loop`]: position `p = k % n`, wraps forever.
//! - [`LoopMode::Once`]: position `p = min(k, n - 1)`; the instance finishes
//!   when elapsed time reaches `n * d`, emitting
//!   [`AnimationEvent::Finished`] exactly once thereafter.
//! - [`LoopMode::PingPong`]: cycle length `2n - 2` (`n >= 2`); positions run
//!   `0..n-1`, reverse to `n-2..1`, and repeat. `n == 1` degenerates to `Loop`.

use std::collections::BTreeMap;
use std::time::Duration;

/// Player-remap / palette-swap passthrough id carried by an
/// [`AnimationInstance`]. `None` means "use the owning entity's default".
pub type PaletteId = u16;

/// Duration floor for degenerate specs (zero / negative / NaN frame times) so
/// an instance can never stall on a non-progressing frame.
pub const MIN_FRAME_DURATION: f32 = 1e-6;

/// Largest accepted accumulated elapsed time (float headroom guard).
const MAX_ELAPSED: f32 = f32::MAX / 2.0;

/// Cap on frame arrivals processed as per-position events within one
/// [`AnimationInstance::advance`] call. Only reachable by pathologically huge
/// `dt`; inputs with sane frame times never hit it (deterministic either way).
const MAX_ARRIVALS_PER_ADVANCE: u64 = 1024;

/// How an animation behaves when its frame sequence reaches the end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopMode {
    /// Wrap around to the first frame indefinitely.
    Loop,
    /// Play to the last frame, then finish.
    Once,
    /// Play forward to the last frame, reverse at both ends, repeat.
    PingPong,
}

/// A trigger attached to one frame of an [`AnimationSpec`]. Fires when the
/// animation *enters* that frame position (once per arrival).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FrameEvent {
    /// Frame position within the spec's frame sequence (0-based index into
    /// the sequence, not a sprite-sheet frame number).
    pub at_position: u32,
    /// Caller-defined trigger id (game event code).
    pub id: u32,
}

/// Immutable description of a frame animation.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationSpec {
    frames: Vec<u32>,
    frame_duration: f32,
    loop_mode: LoopMode,
    events: Vec<FrameEvent>,
}

impl AnimationSpec {
    /// Build a spec. Invalid durations are clamped to [`MIN_FRAME_DURATION`];
    /// an empty `frames` sequence yields an instance that finishes on its
    /// first advance.
    pub fn new(frames: Vec<u32>, frame_duration_secs: f32, loop_mode: LoopMode) -> Self {
        Self {
            frames,
            frame_duration: clamp_duration(frame_duration_secs),
            loop_mode,
            events: Vec::new(),
        }
    }

    /// Attach frame events (builder style).
    pub fn with_events(mut self, events: Vec<FrameEvent>) -> Self {
        self.events = events;
        self
    }

    /// The spec frame values (sprite-level frame indices).
    pub fn frames(&self) -> &[u32] {
        &self.frames
    }

    /// Seconds each frame is shown.
    pub fn frame_duration(&self) -> f32 {
        self.frame_duration
    }

    /// The loop mode of the spec.
    pub fn loop_mode(&self) -> LoopMode {
        self.loop_mode
    }

    /// Attached frame events.
    pub fn events(&self) -> &[FrameEvent] {
        &self.events
    }

    /// Number of frames in the sequence.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}

fn clamp_duration(secs: f32) -> f32 {
    if secs.is_finite() && secs > MIN_FRAME_DURATION {
        secs
    } else {
        MIN_FRAME_DURATION
    }
}

/// Events produced by [`AnimationInstance::advance`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationEvent {
    /// A [`FrameEvent`] trigger fired on arrival at its frame position.
    Triggered(FrameEvent),
    /// A `Once` (or empty) animation reached its end — emitted exactly once.
    Finished,
}

/// A playing animation. Progression is driven entirely by
/// [`AnimationInstance::advance`]: deterministic, headless, no real clock.
#[derive(Clone, Debug)]
pub struct AnimationInstance {
    spec: AnimationSpec,
    elapsed: f32,
    palette_id: Option<PaletteId>,
    finished: bool,
    entered_initial: bool,
    wrap_count: u32,
    fired: BTreeMap<u32, u32>,
}

impl AnimationInstance {
    pub fn new(spec: AnimationSpec) -> Self {
        Self {
            spec,
            elapsed: 0.0,
            palette_id: None,
            finished: false,
            entered_initial: false,
            wrap_count: 0,
            fired: BTreeMap::new(),
        }
    }

    /// Carry a palette id (player remap) from the start.
    pub fn with_palette_id(mut self, palette_id: Option<PaletteId>) -> Self {
        self.palette_id = palette_id;
        self
    }

    /// The immutable spec backing this instance.
    pub fn spec(&self) -> &AnimationSpec {
        &self.spec
    }

    /// Seconds elapsed since start (or since the last [`Self::reset`]).
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// Palette/player-remap id passthrough, independent of progression.
    pub fn palette_id(&self) -> Option<PaletteId> {
        self.palette_id
    }

    /// Update the palette id mid-animation (e.g. on team colour change).
    pub fn set_palette_id(&mut self, palette_id: Option<PaletteId>) {
        self.palette_id = palette_id;
    }

    /// Sprite-level frame value at the current position (`None` for an empty
    /// spec).
    pub fn current_frame(&self) -> Option<u32> {
        self.spec.frames.get(self.position() as usize).copied()
    }

    /// 0-based position within the spec's frame sequence (cycle-mapped).
    pub fn position(&self) -> u32 {
        self.map_arrival(self.arrival())
    }

    /// True once a `Once`/empty animation reached its end. `Loop`/`PingPong`
    /// instances never finish.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Number of loop wraps (cycle-boundary crossings) so far.
    pub fn wrap_count(&self) -> u32 {
        self.wrap_count
    }

    /// How many times the event with `id` has fired (reset by
    /// [`Self::reset`]).
    pub fn fired_count(&self, id: u32) -> u32 {
        self.fired.get(&id).copied().unwrap_or(0)
    }

    /// Advance the animation by `dt` seconds and return the events generated
    /// during this step (frame triggers and, at most once ever, `Finished`).
    /// Non-positive `dt` is a no-op.
    pub fn advance(&mut self, dt: f32) -> Vec<AnimationEvent> {
        if dt <= 0.0 {
            return Vec::new();
        }
        if self.spec.frames.is_empty() {
            if self.finished {
                return Vec::new();
            }
            self.finished = true;
            return vec![AnimationEvent::Finished];
        }

        let prev_arrival = self.arrival();
        let prev_finished = self.finished;

        self.elapsed = (self.elapsed + dt).min(MAX_ELAPSED);

        self.advance_state(prev_arrival, prev_finished)
    }

    /// Advance by repeated fixed steps and return all generated events.
    /// Useful for deterministic time baking over a total duration.
    pub fn advance_by(&mut self, total: f32, step: f32) -> Vec<AnimationEvent> {
        let mut events = Vec::new();
        let step = clamp_duration(step);
        let mut remaining = total.max(0.0);
        while remaining > 0.0 {
            let tick = remaining.min(step);
            events.extend(self.advance(tick));
            remaining -= tick;
        }
        events
    }

    /// Runtime-friendly tick using [`Duration`] instead of `f32` seconds.
    pub fn advance_duration(&mut self, dt: Duration) -> Vec<AnimationEvent> {
        let secs = dt.as_secs_f32();
        if secs.is_finite() && secs > 0.0 {
            self.advance(secs)
        } else {
            Vec::new()
        }
    }

    /// Thread a lookup through the current frame: receives the sprite-level
    /// frame value plus the palette id (player remap) verbatim. `None` for an
    /// empty spec.
    pub fn lookup_current<T>(&self, lookup: impl FnOnce(u32, Option<PaletteId>) -> T) -> Option<T> {
        self.current_frame()
            .map(|frame| lookup(frame, self.palette_id))
    }

    /// Return to the initial state, keeping spec and palette id.
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.finished = false;
        self.entered_initial = false;
        self.wrap_count = 0;
        self.fired.clear();
    }

    // ---- progression internals -------------------------------------------

    /// Integer number of frames shown since start: `floor(elapsed / d)`.
    /// The denominator is always clamped to at least [`MIN_FRAME_DURATION`],
    /// so this never divides by zero or produces NaN.
    fn arrival(&self) -> u64 {
        (self.elapsed / self.spec.frame_duration) as u64
    }

    /// Cycle-mapped position for arrival `k` (per loop mode). Empty specs
    /// map every arrival to position 0.
    fn map_arrival(&self, k: u64) -> u32 {
        let n = self.spec.frames.len() as u64;
        if n == 0 {
            return 0;
        }
        match self.spec.loop_mode {
            LoopMode::Loop => (k % n) as u32,
            LoopMode::Once => k.min(n - 1) as u32,
            LoopMode::PingPong if n == 1 => 0,
            LoopMode::PingPong if n == 2 => (k % 2) as u32,
            LoopMode::PingPong => {
                let cycle = 2 * n - 2;
                let p = k % cycle;
                if p < n {
                    p as u32
                } else {
                    (cycle - p) as u32
                }
            }
        }
    }

    /// Cycle length in arrivals for wrap counting (`1` for single-frame
    /// specs; never `0`).
    fn cycle_len(&self) -> Option<u64> {
        let n = self.spec.frames.len() as u64;
        match self.spec.loop_mode {
            LoopMode::Once => None,
            LoopMode::Loop => Some(n),
            // `2n - 2` is zero for single-frame pingpong; degrade to 1.
            LoopMode::PingPong => Some((2 * n).saturating_sub(2).max(1)),
        }
    }

    /// Apply an elapsed-time increase given the pre-update state.
    fn advance_state(&mut self, prev_arrival: u64, prev_finished: bool) -> Vec<AnimationEvent> {
        let new_arrival = self.arrival();
        let mut events = Vec::new();

        // Initial entry into position 0 fires its events once, on the very
        // first advance.
        if !self.entered_initial {
            self.entered_initial = true;
            let p = self.map_arrival(0);
            self.fire_position(p, &mut events);
        }

        // Arrivals strictly after `prev_arrival`, capped for `Once` (frames
        // beyond the final position are not re-shown / re-fired).
        let cap = match self.spec.loop_mode {
            LoopMode::Once => (self.spec.frames.len() as u64).saturating_sub(1),
            _ => u64::MAX,
        };
        let last_arrival = new_arrival.min(cap);

        // Keep wrap counting exact even across the arrival cap: only the
        // tail arrivals are event-expanded.
        if let Some(cycle) = self.cycle_len() {
            self.wrap_count +=
                (new_arrival / cycle - prev_arrival / cycle).min(u64::from(u32::MAX)) as u32;
        }

        // Only process the most recent arrivals if a pathological `dt` leapt
        // past the per-advance cap (still deterministic; astronomically large
        // steps lose only per-arrival triggering far in the past).
        let excess = last_arrival.saturating_sub(prev_arrival).saturating_sub(1);
        let skipped = excess.saturating_sub(MAX_ARRIVALS_PER_ADVANCE);
        let start = prev_arrival + 1 + skipped;

        for k in start..=last_arrival {
            self.fire_position(self.map_arrival(k), &mut events);
        }

        // `Once` finishing: reached when the total sequence time has elapsed.
        if self.spec.loop_mode == LoopMode::Once && !prev_finished {
            let total = self.spec.frames.len() as f32 * self.spec.frame_duration;
            if self.elapsed >= total {
                self.finished = true;
                events.push(AnimationEvent::Finished);
            }
        }

        events
    }

    /// Fire every event attached to position `p` (once per arrival) and
    /// record the trigger counts.
    fn fire_position(&mut self, p: u32, events: &mut Vec<AnimationEvent>) {
        for event in self.spec.events.iter().filter(|e| e.at_position == p) {
            *self.fired.entry(event.id).or_insert(0) += 1;
            events.push(AnimationEvent::Triggered(*event));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_clamps_degenerate_durations() {
        let spec = AnimationSpec::new(vec![0], 0.0, LoopMode::Once);
        assert!(spec.frame_duration() >= MIN_FRAME_DURATION);
        let spec = AnimationSpec::new(vec![0], f32::NAN, LoopMode::Loop);
        assert!(spec.frame_duration().is_finite());
        assert!(spec.frame_duration() > 0.0);
    }

    #[test]
    fn once_finishes_exactly_at_total_elapsed() {
        let mut inst = AnimationInstance::new(AnimationSpec::new(vec![0, 1], 1.0, LoopMode::Once));
        inst.advance(0.75);
        assert!(!inst.finished());
        inst.advance(0.25); // elapsed hits exactly 1.0 (two representable halves)
        assert!(!inst.finished(), "sequence end is 2.0s, not 1.0s");
        inst.advance(1.0); // elapsed = 2.0 exactly
        assert!(inst.finished());
        assert_eq!(inst.position(), 1);
    }

    #[test]
    fn single_frame_loop_never_finishes_and_wraps() {
        let mut inst = AnimationInstance::new(AnimationSpec::new(vec![42], 0.5, LoopMode::Loop));
        inst.advance(1.0);
        assert_eq!(inst.current_frame(), Some(42));
        assert_eq!(inst.wrap_count(), 2);
        assert!(!inst.finished());
    }

    #[test]
    fn extreme_dt_does_not_explode() {
        let mut inst =
            AnimationInstance::new(AnimationSpec::new(vec![0, 1, 2], 0.5, LoopMode::Loop));
        let events = inst.advance(1.0e9);
        // Position still well-defined; no panic, bounded work.
        assert!(inst.position() <= 2);
        assert!(events.len() <= (MAX_ARRIVALS_PER_ADVANCE as usize + 64));
    }
}
