//! # cnc-procgen — deterministic procedural asset generation for the C&C
//! Rust port.
//!
//! This crate synthesizes **optional** replacement art for the original
//! Westwood binaries (which are not redistributable): terrain heightmaps and
//! biome textures, noise/Tiberium textures, and parametric 32px placeholder
//! unit sprites. Everything is a pure function of its seed and parameters:
//! the same input always yields byte-identical output, so assets can be
//! baked offline and committed or cached without build-time flakiness.
//!
//! All colors are snapped through a
//! [`Palette`](cnc_core::palette::Palette), keeping output faithful to the
//! game's 256-color model; player-color accents flow through
//! [`PlayerRemap`](cnc_core::color::PlayerRemap) exactly like real sprites.
//!
//! ## Modules
//! - [`prng`] — pinned SplitMix64 PRNG + lattice hashing (no version-drifted
//!   `rand` dependency: determinism is load-bearing here).
//! - [`math`] — tiny deterministic interpolation helpers.
//! - [`terrain`] — fBm heightmaps + biome band textures.
//! - [`texture`] — noise ramps + Tiberium patch fields.
//! - [`sprites`] — parametric placeholder unit sprites (foundation; a deeper
//!   sprite lane can add per-class profiles on top).
//! - [`briefing`] — static briefing emblems and commander portraits (the VQA
//!   briefing movies are out of reach: the original decoders are licensed
//!   stubs, so this is static art only).
//!
//! Nothing here touches game simulation state; generation is offline-only.

pub mod briefing;
pub mod math;
pub mod prng;
pub mod sprites;
pub mod terrain;
pub mod texture;

pub use briefing::{
    generate_briefing_emblem, generate_briefing_portrait, BriefingFaction, BRIEFING_ART_SIZE,
};
pub use prng::{hash_coord, hash_f64, Rng, Seed};
pub use sprites::{
    family_of, generate_sprite_for_class, generate_unit_sprite, SpriteFamily, SpritePose,
    SPRITE_SIZE,
};
pub use terrain::{generate_heightmap, generate_terrain_texture, TerrainParams};
pub use texture::{generate_noise_texture, generate_tiberium_texture};
