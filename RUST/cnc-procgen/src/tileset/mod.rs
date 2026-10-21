//! Logic-faithful Rust port of the CNC32 procedural tileset generator
//! (`cnc32_asset_generator_v5.py`, v5.5.0) from the v6.2 asset pack.
//!
//! The port preserves the Python generator's structure 1:1: the same asset
//! categories, mask conventions (N/E/S/W cardinal bits; 47-mask eight-neighbour
//! blob set for terrain), seed layout, manifest schema, and validation-report
//! shape. Pixel output is rendered with this crate's pinned SplitMix64
//! [`Rng`] instead of Python's MT19937 and with local rasterization instead
//! of Pillow, so results are deterministic within Rust but **not**
//! byte-identical to the Python pack's PNGs.
//!
//! Unlike the rest of this crate, colors here are used directly rather than
//! snapped through `cnc_core::palette::Palette`: the Python pack is plain
//! RGBA output with its own curated palette (below), and the port stays
//! faithful to that contract. Engine-facing consumers may snap on load.

/// Tile edge length in pixels (the pack is a 32×32 system).
pub const TILE: u32 = 32;

/// Category atlases are laid out with this many columns of `TILE` cells.
pub const ATLAS_COLUMNS: usize = 16;

/// Straight (non-premultiplied) RGBA pixel, as used throughout the pack.
pub type Rgba = [u8; 4];

/// The pack palette, keyed exactly as the Python generator's `PALETTE` dict.
/// RGB triples; alpha is always 255 (see [`color`]).
pub fn palette() -> &'static [(&'static str, [u8; 3])] {
    &[
        ("grass", [82, 126, 57]),
        ("grass_light", [112, 153, 72]),
        ("grass_dark", [52, 88, 43]),
        ("dirt", [139, 99, 59]),
        ("dirt_light", [171, 128, 78]),
        ("dirt_dark", [91, 63, 43]),
        ("rock", [113, 113, 107]),
        ("rock_light", [157, 157, 147]),
        ("rock_dark", [67, 69, 67]),
        ("water", [39, 91, 139]),
        ("water_light", [70, 135, 177]),
        ("water_dark", [23, 57, 94]),
        ("sand", [194, 174, 112]),
        ("sand_light", [220, 203, 149]),
        ("sand_dark", [145, 123, 76]),
        ("road", [83, 79, 73]),
        ("road_light", [119, 113, 103]),
        ("road_dark", [48, 47, 45]),
        ("gdi", [211, 184, 88]),
        ("gdi_dark", [126, 105, 49]),
        ("nod", [210, 90, 80]),
        ("nod_dark", [99, 35, 36]),
        ("metal", [160, 171, 173]),
        ("metal_dark", [70, 81, 87]),
        ("track", [43, 45, 44]),
        ("infantry", [193, 185, 142]),
        ("skin", [199, 150, 111]),
        ("muzzle", [255, 238, 132]),
        ("white", [255, 255, 255]),
        ("black", [0, 0, 0]),
        ("smoke", [107, 107, 103]),
        ("fire", [255, 104, 26]),
        ("fire_light", [255, 207, 61]),
        ("tiberium", [64, 218, 114]),
    ]
}

/// Opaque RGBA for a palette entry; panics on unknown names (internal
/// invariant, matching the Python code's direct dict indexing).
pub fn color(name: &str) -> Rgba {
    let rgb = palette_rgb(name);
    [rgb[0], rgb[1], rgb[2], 255]
}

/// Raw RGB triple for a palette entry; panics on unknown names.
pub fn palette_rgb(name: &str) -> [u8; 3] {
    palette()
        .iter()
        .find(|(n, _)| *n == name)
        .unwrap_or_else(|| panic!("unknown palette color: {name}"))
        .1
}

/// Stable text seed: first 8 bytes of SHA-256 of the UTF-8 text, big-endian.
///
/// Identical to the Python generator's `stable_seed`, so seed strings hash to
/// the same integer (though the PRNG streams then diverge by design).
pub fn stable_seed(text: &str) -> u64 {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(text.as_bytes());
    u64::from_be_bytes(digest[..8].try_into().expect("8 bytes"))
}

/// Python-`random`-flavored helpers on the crate's pinned PRNG, mirroring the
/// exact call shapes the ported generator uses (`randrange`, `randint`,
/// `random`, `choice`).
pub trait TileRngExt {
    /// Half-open range `[lo, hi)`, matching Python `random.randrange`.
    fn randrange2(&mut self, lo: i32, hi: i32) -> i32;

    /// Inclusive range `[lo, hi]`, matching Python `random.randint`.
    fn rand_i32(&mut self, lo: i32, hi: i32) -> i32;

    /// `true` with probability `p`, matching Python `random.random() < p`.
    fn chance(&mut self, p: f64) -> bool;

    /// Uniform pick, matching Python `random.choice`.
    fn choose<'a, T>(&mut self, items: &'a [T]) -> &'a T;
}

impl TileRngExt for crate::prng::Rng {
    fn randrange2(&mut self, lo: i32, hi: i32) -> i32 {
        debug_assert!(hi > lo, "empty randrange");
        let span = (hi as i64 - lo as i64) as u64;
        lo.wrapping_add((self.next_u64() % span) as i32)
    }

    fn rand_i32(&mut self, lo: i32, hi: i32) -> i32 {
        debug_assert!(hi >= lo, "empty randint");
        self.randrange2(lo, hi + 1)
    }

    fn chance(&mut self, p: f64) -> bool {
        self.next_f64() < p
    }

    fn choose<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        debug_assert!(!items.is_empty(), "choice from empty slice");
        &items[(self.next_u64() % items.len() as u64) as usize]
    }
}

pub mod canvas;
pub mod connections;
pub mod font;
pub mod pack;
pub mod terrain;
pub mod units;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prng::{Rng, Seed};

    #[test]
    fn stable_seed_matches_python_vector() {
        // int.from_bytes(sha256(b"cnc32").digest()[:8], "big")
        assert_eq!(stable_seed("cnc32"), 12_161_523_906_384_641_407);
    }

    #[test]
    fn stable_seed_is_deterministic_and_text_sensitive() {
        assert_eq!(stable_seed("road:5"), stable_seed("road:5"));
        assert_ne!(stable_seed("road:5"), stable_seed("road:6"));
    }

    #[test]
    fn palette_has_no_duplicate_names_and_color_lookup_is_opaque() {
        let names: std::collections::HashSet<&str> = palette().iter().map(|(n, _)| *n).collect();
        assert_eq!(names.len(), palette().len());
        assert_eq!(color("grass"), [82, 126, 57, 255]);
        assert_eq!(color("white"), [255, 255, 255, 255]);
    }

    #[test]
    fn tile_rng_helpers_stay_in_bounds_and_deterministic() {
        let mut a = Rng::new(Seed::new(99));
        let mut b = Rng::new(Seed::new(99));
        for _ in 0..256 {
            // Both generators must consume identical streams to stay in
            // lockstep, so advance each with the same calls.
            let (x1, x2) = (a.randrange2(2, 30), b.randrange2(2, 30));
            assert_eq!(x1, x2);
            assert!((2..30).contains(&x1));
            let (y1, y2) = (a.rand_i32(-5, 5), b.rand_i32(-5, 5));
            assert_eq!(y1, y2);
            assert!((-5..=5).contains(&y1));
            let (c1, c2) = (a.chance(0.5), b.chance(0.5));
            assert_eq!(c1, c2);
        }
        let items = [10, 20, 30];
        let mut r = Rng::new(Seed::new(1));
        let picked = *r.choose(&items);
        assert!(items.contains(&picked));
    }
}
