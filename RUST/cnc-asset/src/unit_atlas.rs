//! Pack-anchored unit sprite atlas.
//!
//! The cnc32 pack ships 2176 `unit_animation` frames (7 unit kinds x 8 facings
//! x idle/move/turn/attack/death) plus 448 `unit_condition` sprites
//! (intact/damaged/critical/destroyed per facing). This module loads those two
//! atlas PNGs and answers direction/state-aware frame lookups so the engines
//! can draw pack art instead of the procedural neon atlas quads.

use std::collections::BTreeMap;
use std::path::Path;

use cnc_core::Rect;

use crate::classic_pack::{Cnc32Pack, Cnc32PackError};

/// One resolved animation frame in atlas-pixel space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitFrame {
    /// Rect inside the composed atlas (atlas pixels).
    pub rect: Rect,
    /// Frame duration in milliseconds (from the manifest `duration_ms`).
    pub duration_ms: u16,
    /// Whether the animation loops (`loop`).
    pub loop_: bool,
}

/// Which animation a unit is playing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitAnim {
    Idle,
    Move,
    Turn,
    Attack,
    Death,
}

impl UnitAnim {
    /// The pack's `state`/`animation` token for this animation.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Move => "move",
            Self::Turn => "turn",
            Self::Attack => "attack",
            Self::Death => "death",
        }
    }
}

/// Condition sprite for a unit's damage state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitCondition {
    Intact,
    Damaged,
    Critical,
    Destroyed,
}

impl UnitCondition {
    pub const fn key(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::Damaged => "damaged",
            Self::Critical => "critical",
            Self::Destroyed => "destroyed",
        }
    }
}

/// The pack's unit sprite tables, resolved into one atlas lookup.
#[derive(Debug)]
pub struct UnitAtlas {
    /// Composed animation atlas (all animation frames in one PNG).
    animation: Vec<u8>,
    animation_width: u32,
    animation_height: u32,
    /// Composed condition atlas (all condition sprites in one PNG).
    condition: Vec<u8>,
    condition_width: u32,
    condition_height: u32,
    /// `unit/state/facing/frame` index into `animation`.
    frames: BTreeMap<String, UnitFrame>,
    /// `unit/condition/facing` rect index into `condition`.
    conditions: BTreeMap<String, Rect>,
}

impl UnitAtlas {
    /// Load and compose the pack's unit atlases.
    ///
    /// Fails when either atlas PNG is missing so callers keep their existing
    /// procedural fallback rather than drawing flat quads.
    pub fn load(pack: &Cnc32Pack) -> Result<UnitAtlas, Cnc32PackError> {
        let mut frames: BTreeMap<String, UnitFrame> = BTreeMap::new();
        let mut conditions: BTreeMap<String, Rect> = BTreeMap::new();
        let mut animation_pngs: std::collections::BTreeSet<String> =
            std::collections::BTreeSet::new();
        let mut condition_pngs: std::collections::BTreeSet<String> =
            std::collections::BTreeSet::new();
        for entry in &pack.manifest.sprites {
            match entry.category.as_str() {
                "unit_animation" => {
                    let key = format!(
                        "{}/{}/{}/{:02}",
                        entry.unit.as_deref().unwrap_or("unknown"),
                        entry.state.as_deref().unwrap_or("idle"),
                        entry.facing.unwrap_or(0),
                        entry.frame.unwrap_or(0),
                    );
                    frames.insert(
                        key,
                        UnitFrame {
                            rect: atlas_rect(entry),
                            duration_ms: entry.duration_ms.unwrap_or(180),
                            loop_: entry.loop_.unwrap_or(true),
                        },
                    );
                    if let Some(atlas) = entry.atlas.as_deref() {
                        animation_pngs.insert(atlas.to_owned());
                    }
                }
                "unit_condition" => {
                    let key = format!(
                        "{}/{}/{}",
                        entry.unit.as_deref().unwrap_or("unknown"),
                        entry.condition.as_deref().unwrap_or("intact"),
                        entry.facing.unwrap_or(0),
                    );
                    conditions.insert(key, atlas_rect(entry));
                    if let Some(atlas) = entry.atlas.as_deref() {
                        condition_pngs.insert(atlas.to_owned());
                    }
                }
                _ => {}
            }
        }

        // Each atlas PNG is already a complete composed sheet: decode it whole
        // instead of re-compositing per-sprite rects.
        let (animation, animation_width, animation_height) =
            decode_whole(pack, &animation_pngs, "unit_animation")?;
        let (condition, condition_width, condition_height) =
            decode_whole(pack, &condition_pngs, "unit_condition")?;

        Ok(Self {
            animation,
            animation_width,
            animation_height,
            condition,
            condition_width,
            condition_height,
            frames,
            conditions,
        })
    }

    /// Load from a pack directory.
    pub fn load_dir(dir: &Path) -> Result<UnitAtlas, Cnc32PackError> {
        let pack = Cnc32Pack::load(dir)?;
        Self::load(&pack)
    }

    /// Convenience: load the default v6.2 pack if it can be found.
    pub fn from_fallback() -> Option<UnitAtlas> {
        let dir = crate::classic_pack::Cnc32Pack::fallback_dir()?;
        Self::load_dir(&dir).ok()
    }

    /// Animation frame for a unit kind, state and facing (facing wraps 0..8).
    pub fn frame(&self, unit: &str, anim: UnitAnim, facing: u8, index: u8) -> Option<UnitFrame> {
        let facing = facing % 8;
        self.frames
            .get(&format!("{unit}/{}/{facing}/{index:02}", anim.key()))
            .copied()
    }

    /// First frame of an animation (index 0), the common draw path.
    pub fn first_frame(&self, unit: &str, anim: UnitAnim, facing: u8) -> Option<UnitFrame> {
        self.frame(unit, anim, facing, 0)
    }

    /// Number of frames the pack carries for a unit/state/facing.
    pub fn frame_count(&self, unit: &str, anim: UnitAnim, facing: u8) -> usize {
        let mut count = 0usize;
        while self.frame(unit, anim, facing, count as u8).is_some() {
            count += 1;
        }
        count
    }

    /// Condition sprite for a unit kind and damage state.
    pub fn condition(&self, unit: &str, condition: UnitCondition, facing: u8) -> Option<Rect> {
        self.conditions
            .get(&format!("{unit}/{}/{}", condition.key(), facing % 8))
            .copied()
    }

    /// Animation atlas pixels (RGBA8), for renderer upload.
    pub fn animation_rgba(&self) -> &[u8] {
        &self.animation
    }

    /// Condition atlas pixels (RGBA8), for renderer upload.
    pub fn condition_rgba(&self) -> &[u8] {
        &self.condition
    }

    pub fn animation_dims(&self) -> (u32, u32) {
        (self.animation_width, self.animation_height)
    }

    pub fn condition_dims(&self) -> (u32, u32) {
        (self.condition_width, self.condition_height)
    }

    /// Unit kinds the pack ships.
    pub fn units(&self) -> impl Iterator<Item = &str> {
        let mut seen: Vec<&str> = Vec::new();
        for key in self.frames.keys() {
            let unit = key.split('/').next().unwrap_or("");
            if !seen.contains(&unit) {
                seen.push(unit);
            }
        }
        seen.into_iter()
    }
}

/// A sprite's rect inside its source atlas PNG.
fn atlas_rect(entry: &crate::classic_pack::SpriteEntry) -> Rect {
    let (x, y) = entry
        .atlas_rect
        .map(|(x, y, _, _)| (x, y))
        .unwrap_or((0, 0));
    let (w, h) = entry.size;
    Rect::new(x as i32, y as i32, w as i32, h as i32)
}

/// Decode the atlas PNGs whole (they are already composed sheets).
fn decode_whole(
    pack: &Cnc32Pack,
    names: &std::collections::BTreeSet<String>,
    category: &str,
) -> Result<(Vec<u8>, u32, u32), Cnc32PackError> {
    let name = names
        .iter()
        .next()
        .ok_or_else(|| Cnc32PackError::MissingAtlas(format!("{category}: no atlas referenced")))?;
    let path = pack.root.join("atlases").join(name);
    if !path.is_file() {
        return Err(Cnc32PackError::MissingAtlas(name.clone()));
    }
    let image = image::open(&path)
        .map_err(|e| Cnc32PackError::Decode(name.clone(), e))?
        .to_rgba8();
    let width = image.width();
    let height = image.height();
    Ok((image.into_raw(), width, height))
}
