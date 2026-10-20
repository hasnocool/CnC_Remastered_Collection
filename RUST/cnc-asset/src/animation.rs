//! Sprite/Animation runtime: frame timing, looping, triggers, palette swap.
//!
//! An [`Animation`] is a definition (ordered frames with per-frame durations
//! in game ticks, a playback mode, timed triggers, and an optional
//! [`PlayerRemap`] palette swap). An [`AnimationPlayer`] is the mutable state
//! advanced by the simulation: calling [`AnimationPlayer::advance`] steps the
//! cursor deterministically and returns the event ids of every frame step
//! *entered* during the advance — the trigger mechanism (e.g. "play a sound
//! when the muzzle flash frame is reached").
//!
//! All timing is in whole game ticks so the animation is locked to the
//! deterministic simulation.

use anyhow::{Context, Result};
use image::RgbaImage;
use std::collections::BTreeMap;

use cnc_core::{PlayerRemap, RGB888};

use crate::atlas::SpriteFrame;
use crate::convert::frame_to_rgba;
use crate::palette::PalFile;
use crate::shape::ShpFile;

/// How the cursor behaves when it walks past the last frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackMode {
    /// Wrap back to the first frame after the last one.
    Loop,
    /// Stop on the last frame (`finished`).
    OneShot,
    /// Reverse direction on the last and first frames.
    PingPong,
}

/// One frame of an animation: which atlas rect to draw and for how long.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnimFrame {
    /// Atlas rect to draw (see `SpriteAtlas::frames`).
    pub sprite: SpriteFrame,
    /// Duration in whole game ticks (never zero).
    pub duration_ticks: u32,
}

/// Immutable animation definition.
#[derive(Clone, Debug)]
pub struct Animation {
    pub name: String,
    frames: Vec<AnimFrame>,
    pub mode: PlaybackMode,
    /// step index -> event id, fired once each time the cursor enters the step.
    triggers: BTreeMap<usize, u32>,
    /// Palette-swap variant: when set, indexed rendering goes through this
    /// remap (e.g. `PlayerRemap::player_color(house)` for unit colors).
    pub player_remap: Option<PlayerRemap>,
}

impl Animation {
    /// Build an animation, clamping every duration to at least one tick so
    /// the cursor always makes progress.
    pub fn new(
        name: impl Into<String>,
        frames: impl IntoIterator<Item = AnimFrame>,
        mode: PlaybackMode,
    ) -> Self {
        let mut frames: Vec<AnimFrame> = frames.into_iter().collect();
        for frame in &mut frames {
            frame.duration_ticks = frame.duration_ticks.max(1);
        }
        Self {
            name: name.into(),
            frames,
            mode,
            triggers: BTreeMap::new(),
            player_remap: None,
        }
    }

    /// Convenience builder from parallel slice of sprites + durations.
    pub fn from_durations(
        name: impl Into<String>,
        sprites: &[SpriteFrame],
        durations_ticks: &[u32],
        mode: PlaybackMode,
    ) -> Result<Self> {
        anyhow::ensure!(
            sprites.len() == durations_ticks.len(),
            "duration count ({}) must match sprite count ({})",
            durations_ticks.len(),
            sprites.len()
        );
        Ok(Self::new(
            name,
            sprites
                .iter()
                .copied()
                .zip(durations_ticks.iter().copied())
                .map(|(sprite, duration_ticks)| AnimFrame {
                    sprite,
                    duration_ticks,
                }),
            mode,
        ))
    }

    /// Register a trigger: event id fired once each time the cursor enters
    /// `step` during an advance.
    pub fn set_trigger(&mut self, step: usize, event: u32) {
        self.triggers.insert(step, event);
    }

    pub fn triggers(&self) -> &BTreeMap<usize, u32> {
        &self.triggers
    }

    /// Attach (or replace) the palette-swap variant.
    pub fn with_player_remap(mut self, remap: Option<PlayerRemap>) -> Self {
        self.player_remap = remap;
        self
    }

    pub fn frames(&self) -> &[AnimFrame] {
        &self.frames
    }

    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Total duration of one uninterrupted forward sweep in game ticks.
    pub fn total_ticks(&self) -> u32 {
        self.frames.iter().map(|frame| frame.duration_ticks).sum()
    }

    /// Fresh cursor starting on step 0.
    pub fn start(&self) -> AnimationPlayer {
        AnimationPlayer {
            step: 0,
            tick_in_frame: 0,
            forward: true,
            finished: self.frames.is_empty(),
        }
    }
}

/// Mutable per-entity animation cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AnimationPlayer {
    step: usize,
    tick_in_frame: u32,
    forward: bool,
    finished: bool,
}

impl AnimationPlayer {
    /// Advance by `ticks` game ticks.
    ///
    /// Returns the event ids of every step *entered* while advancing, in
    /// visit order (a trigger is seen once per entry, so a looping animation
    /// re-fires it each cycle). The cursor never emits an event for the step
    /// it is parked on when the animation starts.
    ///
    /// For `Loop` animations without triggers, long advances are compressed
    /// with modular arithmetic; with triggers the event list grows once per
    /// cycle, so callers drive long replays in tick-sized chunks.
    pub fn advance(&mut self, anim: &Animation, ticks: u32) -> Vec<u32> {
        if anim.frames.is_empty() || self.finished {
            return Vec::new();
        }
        let mut remaining = ticks;
        let total = anim.total_ticks();
        // Compression fast path: triggerless loops wrap with no observable
        // difference (there are no events to preserve).
        if anim.mode == PlaybackMode::Loop
            && anim.triggers.is_empty()
            && total > 0
            && remaining > total
        {
            remaining %= total;
        }

        let mut events = Vec::new();
        while remaining > 0 {
            let duration = anim.frames[self.step].duration_ticks;
            if remaining < duration - self.tick_in_frame {
                self.tick_in_frame += remaining;
                break;
            }
            remaining -= duration - self.tick_in_frame;
            self.tick_in_frame = 0;
            if !self.step_into(anim) {
                break; // finished (OneShot last frame)
            }
            if let Some(event) = anim.triggers.get(&self.step) {
                events.push(*event);
            }
        }
        events
    }

    /// Move to the next step honoring mode + direction.
    /// Returns false once the animation is finished (OneShot).
    fn step_into(&mut self, anim: &Animation) -> bool {
        let last = anim.frames.len() - 1;
        match anim.mode {
            PlaybackMode::Loop => self.step = (self.step + 1) % anim.frames.len(),
            PlaybackMode::OneShot => {
                if self.step == last {
                    self.finished = true;
                    return false;
                }
                self.step += 1;
            }
            PlaybackMode::PingPong => {
                if anim.frames.len() == 1 {
                    return true; // park forever on a single frame
                }
                if self.forward {
                    if self.step == last {
                        self.forward = false;
                        self.step -= 1;
                    } else {
                        self.step += 1;
                    }
                } else if self.step == 0 {
                    self.forward = true;
                    self.step = 1;
                } else {
                    self.step -= 1;
                }
            }
        }
        true
    }

    /// True once a OneShot animation has played its last frame.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Current step index (0-based).
    pub fn step(&self) -> usize {
        self.step
    }

    /// Whole ticks elapsed inside the current step.
    pub fn tick_in_frame(&self) -> u32 {
        self.tick_in_frame
    }

    /// True when the cursor is moving backwards (PingPong only).
    pub fn is_backwards(&self) -> bool {
        !self.forward
    }

    /// Atlas rect to draw right now.
    pub fn current_frame(&self, anim: &Animation) -> Option<AnimFrame> {
        anim.frames.get(self.step).copied()
    }

    /// Alias matching `Sprite`-era naming: the sprite rect to draw now.
    pub fn current_sprite(&self, anim: &Animation) -> Option<SpriteFrame> {
        self.current_frame(anim).map(|frame| frame.sprite)
    }
}

/// Decode one SHP frame to RGBA through `palette` with a player-color
/// palette swap applied.
///
/// The raw palette index of each source pixel decides transparency (a raw
/// index equal to `transparent` never gets remapped, so the transparent slot
/// is not disturbed), while the *color* is looked up via the remap table:
/// `remapped_colors[i] = palette[remap.map[i]]`.
pub fn frame_to_rgba_remapped(
    shp: &ShpFile,
    index: usize,
    palette: &PalFile,
    remap: &PlayerRemap,
    transparent: Option<u8>,
) -> Result<RgbaImage> {
    let frame = shp
        .frame(index)
        .with_context(|| format!("SHP frame {index} out of range"))?;
    let mut remapped_colors = [RGB888::BLACK; 256];
    for (slot, map_to) in remapped_colors.iter_mut().zip(remap.map.iter()) {
        *slot = palette.colors[*map_to as usize];
    }
    let mut image =
        frame_to_rgba(frame, &remapped_colors, transparent).context("failed to decode frame")?;

    // The transparent raw index is exempt from the color remap (see doc
    // comment): restore the raw palette color for those pixels so the
    // transparent slot itself is not disturbed.
    if let Some(t) = transparent {
        let pixel_count = (frame.width as usize) * (frame.height as usize);
        for (pixel, &raw) in image.pixels_mut().zip(frame.data[..pixel_count].iter()) {
            if raw == t {
                let color = palette.colors[raw as usize];
                *pixel = [color.r, color.g, color.b, 0].into();
            }
        }
    }
    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame_at(x: u32, y: u32, w: u32, h: u32, duration_ticks: u32) -> AnimFrame {
        AnimFrame {
            sprite: SpriteFrame { x, y, w, h },
            duration_ticks,
        }
    }

    fn loop_anim() -> Animation {
        Animation::new(
            "walk",
            [
                frame_at(0, 0, 8, 8, 2),
                frame_at(8, 0, 8, 8, 1),
                frame_at(16, 0, 8, 8, 3),
            ],
            PlaybackMode::Loop,
        )
    }

    #[test]
    fn durations_never_cancel_progress() {
        let anim = Animation::new(
            "clamped",
            [frame_at(0, 0, 1, 1, 0), frame_at(0, 0, 1, 1, 2)],
            PlaybackMode::Loop,
        );
        assert_eq!(anim.frames()[0].duration_ticks, 1, "zero durations clamped");
        assert_eq!(anim.total_ticks(), 3);
    }

    #[test]
    fn loop_mode_cycles_and_returns_to_start() {
        let anim = loop_anim();
        let mut player = anim.start();
        assert_eq!(player.step(), 0);
        // Durations: [2, 1, 3], cycle = 6.
        player.advance(&anim, 2);
        assert_eq!((player.step(), player.tick_in_frame()), (1, 0));
        player.advance(&anim, 1);
        assert_eq!(player.step(), 2);
        player.advance(&anim, 3);
        assert_eq!(player.step(), 0, "wrapped to first frame");
        player.advance(&anim, 6);
        assert_eq!(player.step(), 0, "full cycle returns to start");
    }

    #[test]
    fn mid_frame_advance_keeps_tick_counter() {
        let anim = loop_anim();
        let mut player = anim.start();
        player.advance(&anim, 1);
        assert_eq!((player.step(), player.tick_in_frame()), (0, 1));
        player.advance(&anim, 1);
        assert_eq!((player.step(), player.tick_in_frame()), (1, 0));
    }

    #[test]
    fn trigger_fires_once_per_step_entry() {
        let mut anim = loop_anim();
        anim.set_trigger(2, 7); // muzzle-flash style event on frame 2
        let mut player = anim.start();
        // Cycle = [2,1,3]: advancing 6 ticks enters steps 1,2,0 in one cycle.
        let events = player.advance(&anim, 6);
        assert_eq!(events, vec![7], "step 2 entered exactly once");
        // Next full cycle re-enters step 2 -> another event.
        let events = player.advance(&anim, 6);
        assert_eq!(events, vec![7]);
    }

    #[test]
    fn trigger_fires_each_loop_of_long_advance() {
        let mut anim = loop_anim();
        anim.set_trigger(1, 3);
        let mut player = anim.start();
        // 13 ticks = 6 (cycle) * 2 + 1: enters step 1 twice and step 2 once.
        let events = player.advance(&anim, 13);
        assert_eq!(events, vec![3, 3]);
    }

    #[test]
    fn one_shot_stops_on_last_frame_and_finishes() {
        let mut anim = Animation::new(
            "buildup",
            [
                frame_at(0, 0, 8, 8, 1),
                frame_at(8, 0, 8, 8, 1),
                frame_at(16, 0, 8, 8, 2),
            ],
            PlaybackMode::OneShot,
        );
        anim.set_trigger(2, 5);
        let mut player = anim.start();
        let events = player.advance(&anim, 99);
        assert_eq!(events, vec![5], "final step entered");
        assert!(player.finished());
        assert_eq!(player.step(), 2, "parks on the last frame");
        assert!(
            player.advance(&anim, 50).is_empty(),
            "no events when finished"
        );
        assert_eq!(player.current_frame(&anim), Some(frame_at(16, 0, 8, 8, 2)));
    }

    #[test]
    fn ping_pong_reverses_at_edges() {
        let anim = Animation::new(
            "hover",
            [
                frame_at(0, 0, 8, 8, 1),
                frame_at(8, 0, 8, 8, 1),
                frame_at(16, 0, 8, 8, 1),
            ],
            PlaybackMode::PingPong,
        );
        let mut player = anim.start();
        let path = player.advance(&anim, 7);
        assert!(path.is_empty());
        // 7 ticks: steps visited 1(f) 2(f) 1(b) 0(b) 1(f) 2(f) 1(b)
        // cursor lands on step 1 moving backwards, 0 ticks into it.
        assert_eq!(player.step(), 1);
        assert!(player.is_backwards());
        assert!(!player.finished(), "ping-pong never finishes");
        // Repeatable deterministic path: restart and walk again.
        let mut player2 = anim.start();
        player2.advance(&anim, 7);
        assert_eq!(player, player2);
    }

    #[test]
    fn single_frame_ping_pong_never_moves() {
        let anim = Animation::new("idle", [frame_at(0, 0, 4, 4, 5)], PlaybackMode::PingPong);
        let mut player = anim.start();
        player.advance(&anim, 42);
        assert_eq!(player.step(), 0);
        assert!(!player.finished());
    }

    #[test]
    fn from_durations_validates_lengths() {
        let sprites = [SpriteFrame {
            x: 0,
            y: 0,
            w: 1,
            h: 1,
        }];
        assert!(Animation::from_durations("bad", &sprites, &[], PlaybackMode::Loop).is_err());
        let anim = Animation::from_durations("ok", &sprites, &[3], PlaybackMode::Loop).expect("ok");
        assert_eq!(anim.frame_count(), 1);
    }

    #[test]
    fn palette_swap_decodes_through_player_remap() {
        use crate::palette::PalFile;
        use crate::shape::ShpFile;

        let palette = PalFile::grayscale();
        let remap = PlayerRemap::player_color(1);
        // player_color(1) maps indices 0..16 -> 16..32; identity elsewhere.

        // SHP v0: 1 untracked frame, header 8 bytes + frame headers 8 bytes +
        // offset 4 bytes + indices (no RLE since len == expected).
        let raw = [0u8, 5u8, 7u8, 200u8, 4u8];
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_le_bytes()); // version
        data.extend_from_slice(&0u16.to_le_bytes()); // flags
        data.extend_from_slice(&1u16.to_le_bytes()); // frame count
        data.extend_from_slice(&0u16.to_le_bytes()); // unknown
        data.extend_from_slice(&3u16.to_le_bytes()); // width
        data.extend_from_slice(&1u16.to_le_bytes()); // height
        data.extend_from_slice(&0i16.to_le_bytes()); // offset_x
        data.extend_from_slice(&0i16.to_le_bytes()); // offset_y
        data.extend_from_slice(&20u32.to_le_bytes()); // data offset (file-absolute)
        data.extend_from_slice(&raw);
        let shp = ShpFile::load(&data).expect("parse shp");

        let image =
            frame_to_rgba_remapped(&shp, 0, &palette, &remap, Some(5)).expect("decode with remap");
        let pixels = image.into_raw();
        // Pixel 0: raw index 0 -> color palette[16] = gray 16.
        assert_eq!(&pixels[0..4], &[16, 16, 16, 255]);
        // Pixel 1: raw index 5 == transparent -> alpha 0 (transparent wins).
        assert_eq!(
            &pixels[4..8],
            &[5, 5, 5, 0],
            "transparent index keeps alpha 0"
        );
        // Pixel 2: raw index 7 -> remap 16+7=23 -> palette[23] = gray 23.
        assert_eq!(&pixels[8..12], &[23, 23, 23, 255]);
    }
}

#[cfg(test)]
mod dbg {
    #[test]
    fn probe_parse() {
        use crate::shape::ShpFile;
        let raw = [0u8, 5u8, 7u8, 200u8, 4u8];
        let mut data = Vec::new();
        data.extend_from_slice(&0u16.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());
        data.extend_from_slice(&3u16.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&0i16.to_le_bytes());
        data.extend_from_slice(&0i16.to_le_bytes());
        data.extend_from_slice(&16u32.to_le_bytes());
        data.extend_from_slice(&raw);
        let shp = ShpFile::load(&data).expect("parse shp");
        let f = &shp.frames[0];
        println!("w={} h={} data={:?}", f.width, f.height, f.data);
    }
}
