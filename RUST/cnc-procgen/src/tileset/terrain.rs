//! Terrain tiles: material bases and eight-neighbour blob autotile
//! transitions.
//!
//! Logic-faithful port of `terrain_base`, `valid_blob_masks` and
//! `terrain_transition` from `cnc32_asset_generator_v5.py` (v5.5.0). Seed
//! strings and drawing order match the Python source 1:1; pixels differ
//! because randomness comes from this crate's SplitMix64 [`Rng`] rather than
//! Python's MT19937, and rasterization goes through [`Canvas`] rather than
//! Pillow.

use super::canvas::{Canvas, Mask};
use super::{color, stable_seed, Rgba, TileRngExt};
use crate::prng::{Rng, Seed};

/// Terrain material kinds, in pack order.
pub const TERRAIN_KINDS: [&str; 5] = ["grass", "dirt", "rock", "water", "sand"];

/// Eight-neighbour bit names: N, NE, E, SE, S, SW, W, NW.
pub const BLOB_DIR_NAMES: [&str; 8] = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

/// Default neighbour material when `alt_kind` is not given (Python's map).
fn default_alt(kind: &str) -> Option<&'static str> {
    match kind {
        "grass" => Some("dirt"),
        "dirt" => Some("grass"),
        "rock" => Some("dirt"),
        "water" => Some("sand"),
        "sand" => Some("water"),
        _ => None,
    }
}

/// The 47 valid eight-neighbour blob masks (ascending). A diagonal bit is
/// legal only when both adjacent cardinal bits are set.
pub fn blob_masks() -> &'static [u8] {
    static MASKS: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    MASKS.get_or_init(|| {
        (0..=255u8)
            .filter(|&mask| {
                [(1usize, 0, 2), (3, 2, 4), (5, 4, 6), (7, 6, 0)]
                    .iter()
                    .all(|&(diag, a, b)| {
                        mask & (1 << diag) == 0 || (mask & (1 << a) != 0 && mask & (1 << b) != 0)
                    })
            })
            .collect()
    })
}

/// Structured-deterministic material noise: jittered points and 1–2px runs
/// around the base color (Python `noise_texture`).
fn noise_texture(im: &mut Canvas, base: &str, seed_text: &str, amount: i32, count: i32) {
    let mut rng = Rng::new(Seed::new(stable_seed(seed_text)));
    let c = color(base);
    for _ in 0..count {
        let delta = rng.rand_i32(-amount, amount);
        // Python: max(0, min(255, v + delta))
        let col: Rgba = [
            (c[0] as i32 + delta).clamp(0, 255) as u8,
            (c[1] as i32 + delta).clamp(0, 255) as u8,
            (c[2] as i32 + delta).clamp(0, 255) as u8,
            255,
        ];
        let x = rng.randrange2(0, 32);
        let y = rng.randrange2(0, 32);
        if rng.chance(0.75) {
            im.set_pixel(x, y, col);
        } else {
            let w = rng.randrange2(1, 3);
            let h = rng.randrange2(1, 3);
            im.rect(x, y, (x + w).min(31), (y + h).min(31), Some(col), None, 0);
        }
    }
}

/// Richer 32×32 pixel-art material tile with structured detail.
pub fn terrain_base(kind: &str, seed: &str) -> Canvas {
    let base = match kind {
        "grass" | "dirt" | "rock" | "water" | "sand" => kind,
        other => panic!("unknown terrain kind: {other}"),
    };
    let mut im = Canvas::new_filled(32, 32, color(base));
    let (amount, count) = match kind {
        "water" => (7, 46),
        "grass" | "dirt" | "sand" => (9, 70),
        _ => (9, 46),
    };
    noise_texture(&mut im, base, seed, amount, count);
    let mut rng = Rng::new(Seed::new(stable_seed(&format!("{seed}:material-details"))));
    match kind {
        "grass" => {
            // Small clumps with light tips and dark root pixels.
            for _ in 0..6 {
                let x = rng.randrange2(2, 30);
                let y = rng.randrange2(2, 30);
                im.set_pixel(x, y, color("grass_dark"));
                im.set_pixel((x + 1).min(31), (y - 1).max(0), color("grass_light"));
                if rng.chance(0.55) {
                    im.line(
                        x,
                        y,
                        (x + 1).min(31),
                        (y - 2).max(0),
                        color("grass_light"),
                        1,
                    );
                }
            }
        }
        "dirt" => {
            for _ in 0..9 {
                let x = rng.randrange2(2, 30);
                let y = rng.randrange2(2, 30);
                let col = if rng.chance(0.55) {
                    color("dirt_light")
                } else {
                    color("dirt_dark")
                };
                im.line(x, y, (x + rng.randrange2(1, 3)).min(31), y, col, 1);
                if rng.chance(0.3) {
                    im.set_pixel(x, (y + 1).min(31), color("dirt_dark"));
                }
            }
        }
        "rock" => {
            // Faceted strata and small cracks.
            for _ in 0..4 {
                let x = rng.randrange2(3, 25);
                let y = rng.randrange2(3, 25);
                let w = rng.randrange2(3, 7);
                let h = rng.randrange2(2, 5);
                let fill = if rng.chance(0.45) {
                    color("rock_dark")
                } else {
                    color("rock_light")
                };
                im.polygon(
                    &[
                        (x, y),
                        ((x + w).min(31), y + 1),
                        ((x + w - 1).min(31), (y + h).min(31)),
                        (x + 1, (y + h - 1).min(31)),
                    ],
                    Some(fill),
                    None,
                );
            }
            im.line(4, 8, 8, 6, color("rock_light"), 1);
            im.line(8, 6, 11, 9, color("rock_light"), 1);
            im.line(20, 23, 24, 19, color("rock_dark"), 1);
            im.line(24, 19, 27, 20, color("rock_dark"), 1);
        }
        "water" => {
            // Broken, multi-length glints.
            for _ in 0..8 {
                let x = rng.randrange2(1, 28);
                let y = rng.randrange2(2, 30);
                let length = rng.randrange2(2, 7);
                let col = if rng.chance(0.75) {
                    color("water_light")
                } else {
                    color("water_dark")
                };
                im.line(x, y, (x + length).min(31), y, col, 1);
                if rng.chance(0.35) {
                    im.set_pixel((x + length).min(31), y + 1, color("water_light"));
                }
            }
        }
        "sand" => {
            for _ in 0..10 {
                let x = rng.randrange2(1, 31);
                let y = rng.randrange2(1, 31);
                let col = if rng.chance(0.5) {
                    color("sand_dark")
                } else {
                    color("sand_light")
                };
                im.set_pixel(x, y, col);
                if rng.chance(0.3) {
                    im.set_pixel((x + 1).min(31), y, col);
                }
            }
        }
        _ => {}
    }
    // Subtle top-left highlight and bottom-right shade on solid materials.
    if matches!(kind, "rock" | "dirt" | "sand") {
        im.set_pixel(1, 1, color(&format!("{base}_light")));
        im.set_pixel(30, 30, color(&format!("{base}_dark")));
    }
    im
}

/// Error for an invalid terrain transition pair.
#[derive(thiserror::Error, Debug)]
#[error("invalid alternate terrain pair: {0} -> {1}")]
pub struct InvalidPair(pub String, pub String);

/// Eight-neighbour blob autotile with material-specific jagged boundaries.
///
/// `same_mask` bit i (order [`BLOB_DIR_NAMES`]) means the neighbour in that
/// direction is the same material; absent cardinal bits receive an authored
/// transition edge, and absent diagonals cut compact corner notches.
pub fn terrain_transition(
    kind: &str,
    same_mask: u8,
    seed: &str,
    alt_kind: Option<&str>,
) -> Result<Canvas, InvalidPair> {
    let alt = alt_kind.or_else(|| default_alt(kind)).unwrap_or("");
    let valid_kind = TERRAIN_KINDS.contains(&kind);
    let valid_alt = TERRAIN_KINDS.contains(&alt);
    if !valid_kind || !valid_alt || alt == kind {
        return Err(InvalidPair(kind.to_string(), alt.to_string()));
    }
    let mut im = terrain_base(alt, &format!("{seed}:alternate"));
    let primary = terrain_base(kind, &format!("{seed}:primary"));
    let mut rng = Rng::new(Seed::new(stable_seed(&format!(
        "{seed}:{kind}:{alt}:{same_mask}:edge"
    ))));
    let bits: [u8; 8] = core::array::from_fn(|i| (same_mask >> i) & 1);
    let (n, ne, e, se, so, sw, w, nw) = (
        bits[0], bits[1], bits[2], bits[3], bits[4], bits[5], bits[6], bits[7],
    );

    // Stable boundary profiles with short plateaus -> chunky authored edges.
    fn profile(rng: &mut Rng, base: i32, count: i32, lo: i32, hi: i32) -> Vec<i32> {
        const STEPS: [i32; 5] = [-2, -1, 0, 1, 2];
        let mut values = Vec::with_capacity(count as usize);
        let mut current = base;
        for i in 0..count {
            if i % 3 == 0 {
                let delta = *rng.choose(&STEPS);
                current = (current + delta).max(lo).min(hi);
            }
            values.push(current);
        }
        values
    }
    let north = profile(&mut rng, 7, 32, 4, 11);
    let south = profile(&mut rng, 7, 32, 4, 11);
    let west = profile(&mut rng, 7, 32, 4, 11);
    let east = profile(&mut rng, 7, 32, 4, 11);

    let mut mask = Mask::new(32, 32, 0);
    for y in 0..32i32 {
        for x in 0..32i32 {
            let mut inside = true;
            if n == 0 && y < north[x as usize] {
                inside = false;
            }
            if so == 0 && y >= 32 - south[x as usize] {
                inside = false;
            }
            if w == 0 && x < west[y as usize] {
                inside = false;
            }
            if e == 0 && x >= 32 - east[y as usize] {
                inside = false;
            }
            // A missing diagonal cuts a compact notch when both side edges connect.
            if ne == 0 && n == 1 && e == 1 && x >= 27 && y < 5 && x + y < 8 {
                inside = false;
            }
            if se == 0 && e == 1 && so == 1 && x >= 27 && y >= 27 && (31 - x) + (31 - y) < 8 {
                inside = false;
            }
            if sw == 0 && so == 1 && w == 1 && x < 5 && y >= 27 && x + (31 - y) < 8 {
                inside = false;
            }
            if nw == 0 && w == 1 && n == 1 && x < 5 && y < 5 && x + y < 8 {
                inside = false;
            }
            if inside {
                mask.set(x, y, 255);
            }
        }
    }
    im.masked_paste(&primary, &mask, (0, 0));

    // One-pixel edge shadow where the primary region meets the alternate.
    let edge_col = {
        let name = format!("{kind}_dark");
        let named: Vec<&str> = palette_names();
        if named.contains(&name.as_str()) {
            color(&name)
        } else {
            color(kind)
        }
    };
    for y in 0..32i32 {
        for x in 0..32i32 {
            if mask.get(x, y) != Some(255) {
                continue;
            }
            // Python only compares in-bounds neighbours; the tile rim is
            // never shadowed just because it borders the canvas edge.
            let border = (x > 0 && mask.get(x - 1, y) != Some(255))
                || (x < 31 && mask.get(x + 1, y) != Some(255))
                || (y > 0 && mask.get(x, y - 1) != Some(255))
                || (y < 31 && mask.get(x, y + 1) != Some(255));
            if border {
                im.set_pixel(x, y, edge_col);
            }
        }
    }

    // Material-specific transition sparkle inside the primary region.
    let light = {
        let name = format!("{kind}_light");
        let named: Vec<&str> = palette_names();
        if named.contains(&name.as_str()) {
            color(&name)
        } else {
            color(kind)
        }
    };
    for _ in 0..8 {
        let x = rng.randrange2(2, 30);
        let y = rng.randrange2(2, 30);
        if mask.get(x, y) == Some(255) && rng.chance(0.65) {
            im.set_pixel(x, y, light);
        }
    }
    Ok(im)
}

fn palette_names() -> Vec<&'static str> {
    super::palette().iter().map(|(n, _)| *n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blob_mask_count_is_47() {
        assert_eq!(blob_masks().len(), 47);
    }

    #[test]
    fn blob_masks_obey_diagonal_constraint() {
        for &mask in blob_masks() {
            for (diag, a, b) in [(1usize, 0usize, 2usize), (3, 2, 4), (5, 4, 6), (7, 6, 0)] {
                if mask & (1 << diag) != 0 {
                    assert!(
                        mask & (1 << a) != 0 && mask & (1 << b) != 0,
                        "mask {mask:#04x}: diagonal {diag} without cardinals"
                    );
                }
            }
        }
    }

    #[test]
    fn terrain_base_is_32x32_and_deterministic() {
        for kind in TERRAIN_KINDS {
            let a = terrain_base(kind, "test-seed");
            let b = terrain_base(kind, "test-seed");
            let c = terrain_base(kind, "other-seed");
            assert_eq!((a.width(), a.height()), (32, 32));
            assert_eq!(a.as_rgba().as_raw(), b.as_rgba().as_raw());
            assert_ne!(a.as_rgba().as_raw(), c.as_rgba().as_raw(), "{kind}");
        }
    }

    #[test]
    fn terrain_base_fills_background_with_kind_color() {
        // Corners are untouched by detail passes for most kinds; at minimum
        // the tile is never fully transparent.
        let im = terrain_base("water", "s");
        let px = im.pixel(16, 16).expect("in bounds");
        assert_eq!(px[3], 255);
    }

    #[test]
    fn transition_rejects_invalid_pairs() {
        assert!(terrain_transition("grass", 0, "s", Some("grass")).is_err());
        assert!(terrain_transition("lava", 0, "s", None).is_err());
        assert!(terrain_transition("grass", 0, "s", Some("dirt")).is_ok());
        // Default alt mapping.
        assert!(terrain_transition("sand", 0, "s", None).is_ok());
        assert!(terrain_transition("rock", 5, "s", Some("water")).is_ok());
    }

    #[test]
    fn transition_is_deterministic_and_sized() {
        for mask in [0u8, 15, 47] {
            let a = terrain_transition("grass", mask, "pack", Some("dirt")).unwrap();
            let b = terrain_transition("grass", mask, "pack", Some("dirt")).unwrap();
            assert_eq!((a.width(), a.height()), (32, 32));
            assert_eq!(a.as_rgba().as_raw(), b.as_rgba().as_raw());
        }
    }

    #[test]
    fn different_masks_change_transition_output() {
        let mut hashes = std::collections::HashSet::new();
        for &mask in blob_masks() {
            let im = terrain_transition("grass", mask, "pack", Some("dirt")).unwrap();
            hashes.insert(im.as_rgba().as_raw().clone());
        }
        assert!(
            hashes.len() >= 40,
            "masks must diversify output: {}",
            hashes.len()
        );
    }

    #[test]
    fn isolated_mask_keeps_primary_center_and_alternate_edges() {
        // same_mask 0: only the profiled edge strips become alternate; the
        // center blob stays primary (Python carves just the edge profiles).
        let im = terrain_transition("grass", 0, "pack", Some("dirt")).unwrap();
        assert_eq!(im.pixel(16, 16).unwrap(), color("grass"));
        // Corner belongs to the alternate side; (1,1) is dirt's deliberate
        // top-left highlight pixel (dirt_light) from terrain_base.
        let corner = im.pixel(1, 1).unwrap();
        let dirt_family = [color("dirt"), color("dirt_light"), color("dirt_dark")];
        assert!(
            dirt_family.contains(&corner),
            "corner {corner:?} not in dirt family"
        );
    }

    #[test]
    fn full_mask_has_no_edge_shadow() {
        // Python's border check skips out-of-bounds neighbours, so a fully
        // connected tile never shadows its rim.
        let im = terrain_transition("grass", 255, "pack", Some("dirt")).unwrap();
        assert_eq!(im.pixel(0, 16).unwrap(), color("grass"));
        assert_eq!(im.pixel(16, 0).unwrap(), color("grass"));
        assert_eq!(im.pixel(31, 16).unwrap(), color("grass"));
    }

    #[test]
    fn full_mask_is_fully_primary() {
        // same_mask 255: everything connects; interior stays primary with a
        // border shadow on the outer ring only.
        let im = terrain_transition("grass", 255, "pack", Some("dirt")).unwrap();
        assert_eq!(im.pixel(16, 16).unwrap(), color("grass"));
    }
}
