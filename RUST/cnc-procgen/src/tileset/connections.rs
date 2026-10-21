//! Connection tiles: roads, shorelines, cliffs, walls, bridges, plus the
//! road-continuity QA audit.
//!
//! Logic-faithful port of `connected_road`, `road_edge_signature`,
//! `audit_road_continuity`, `connected_shore` and the cliff/wall/bridge
//! loops from `build()` in `cnc32_asset_generator_v5.py` (v5.5.0). Seed
//! strings and draw order match Python 1:1; pixels differ (SplitMix64 +
//! local rasterization). Cardinal mask convention: bit 0=N, 1=E, 2=S, 3=W;
//! a set bit means the connection continues through that tile edge.

use super::canvas::Canvas;
use super::terrain::terrain_base;
use super::{color, stable_seed, Rgba, TileRngExt};
use crate::prng::{Rng, Seed};

/// Cardinal direction names in bit order N, E, S, W.
pub const DIRECTION_NAMES: [&str; 4] = ["N", "E", "S", "W"];

/// The pack's descriptive name for each of the 16 road masks.
pub fn road_mask_name(mask: u8) -> &'static str {
    match mask {
        0 => "isolated",
        1 => "north_end",
        2 => "east_end",
        4 => "south_end",
        8 => "west_end",
        5 => "vertical",
        10 => "horizontal",
        3 => "north_east_corner",
        6 => "east_south_corner",
        12 => "south_west_corner",
        9 => "west_north_corner",
        7 => "t_north_missing_west",
        11 => "t_east_missing_south",
        13 => "t_south_missing_east",
        14 => "t_west_missing_north",
        15 => "cross",
        _ => "invalid",
    }
}

/// Whether a pixel is road material (any road shade, opaque).
pub fn is_road_pixel(rgba: Rgba) -> bool {
    rgba[3] > 0
        && matches!(
            [rgba[0], rgba[1], rgba[2]],
            _ if [rgba[0], rgba[1], rgba[2]] == color("road")[..3]
                || [rgba[0], rgba[1], rgba[2]] == color("road_dark")[..3]
                || [rgba[0], rgba[1], rgba[2]] == color("road_light")[..3]
        )
}

/// Connected dirt road with edge-to-edge arms, shoulders, caps and gravel.
///
/// A set bit extends the arm to the tile border; a clear bit ends it at a
/// bevelled cap near the center with edge wear.
pub fn connected_road(mask: u8, seed: &str) -> Canvas {
    let mut im = terrain_base("dirt", &format!("{seed}:roadbed"));
    let (cx, cy) = (15, 15);
    // Arm endpoints N/E/S/W; shortened per disconnected arm below.
    let endpoints = [(15, -1), (32, 15), (15, 32), (-1, 15)];
    let cap = |i: usize, mut ex: i32, mut ey: i32| -> (i32, i32) {
        match i {
            0 => ey = 8,
            1 => ex = 23,
            2 => ey = 23,
            _ => ex = 8,
        }
        (ex, ey)
    };
    // Dark shoulders first, then the road surface.
    for (i, &(end_x, end_y)) in endpoints.iter().enumerate() {
        let (ex, ey) = if mask & (1 << i) == 0 {
            cap(i, end_x, end_y)
        } else {
            (end_x, end_y)
        };
        im.line(cx, cy, ex, ey, color("road_dark"), 15);
    }
    im.ellipse(
        cx - 7,
        cy - 7,
        cx + 7,
        cy + 7,
        Some(color("road_dark")),
        None,
        0,
    );
    for (i, &(end_x, end_y)) in endpoints.iter().enumerate() {
        let (ex, ey) = if mask & (1 << i) == 0 {
            cap(i, end_x, end_y)
        } else {
            (end_x, end_y)
        };
        im.line(cx, cy, ex, ey, color("road"), 11);
    }
    im.ellipse(cx - 5, cy - 5, cx + 5, cy + 5, Some(color("road")), None, 0);
    // Broken gravel highlights on road material only. Python gates on the
    // base road shades {road, road_dark} only — NOT road_light — so
    // previously placed gravel never attracts more gravel.
    let mut rng = Rng::new(Seed::new(stable_seed(&format!("{seed}:gravel"))));
    for _ in 0..15 {
        let x = rng.randrange2(3, 29);
        let y = rng.randrange2(3, 29);
        if let Some(px) = im.pixel(x, y) {
            let base_road = px[3] > 0
                && ([px[0], px[1], px[2]] == color("road")[..3]
                    || [px[0], px[1], px[2]] == color("road_dark")[..3]);
            if base_road {
                let col = if rng.chance(0.7) {
                    color("road_light")
                } else {
                    color("road_dark")
                };
                im.set_pixel(x, y, col);
            }
        }
    }
    // Fine edge wear at disconnected arm ends.
    for i in 0..4 {
        if mask & (1 << i) == 0 {
            let (x0, y0, x1, y1) = match i {
                0 => (10, 8, 20, 8),
                1 => (23, 10, 23, 20),
                2 => (10, 23, 20, 23),
                _ => (8, 10, 8, 20),
            };
            im.line(x0, y0, x1, y1, color("dirt_dark"), 1);
        }
    }
    im
}

/// Sample road-material presence along a tile edge (32 booleans).
pub fn road_edge_signature(im: &Canvas, direction: &str) -> [bool; 32] {
    let mut sig = [false; 32];
    for i in 0..32i32 {
        let (x, y) = match direction {
            "N" => (i, 0),
            "E" => (31, i),
            "S" => (i, 31),
            _ => (0, i), // "W"
        };
        sig[i as usize] = im.pixel(x, y).is_some_and(is_road_pixel);
    }
    sig
}

/// One road-continuity audit failure.
#[derive(Debug)]
pub struct RoadAuditFailure {
    pub mask: u8,
    pub edge: &'static str,
    pub connected: bool,
    pub edge_reached: bool,
}

/// Result of [`audit_road_continuity`].
#[derive(Debug)]
pub struct RoadAudit {
    pub checks: usize,
    pub failures: Vec<RoadAuditFailure>,
    pub pass: bool,
}

/// Verify each connected road arm reaches its edge center band and
/// disconnected arms do not (the Python pack's own QA gate).
pub fn audit_road_continuity(seed: &str) -> RoadAudit {
    let mut failures = Vec::new();
    let mut checks = 0;
    for mask in 0..16u8 {
        let im = connected_road(mask, &format!("{seed}:{mask}"));
        for (bit, edge) in [(0, "N"), (1, "E"), (2, "S"), (3, "W")] {
            let sig = road_edge_signature(&im, edge);
            let connected = mask & (1 << bit) != 0;
            // Python: any(sig[TILE//2-5 : TILE//2+6]) -> indices 11..=21.
            let reaches = sig[11..=21].iter().any(|&b| b);
            let ok = reaches == connected;
            checks += 1;
            if !ok {
                failures.push(RoadAuditFailure {
                    mask,
                    edge,
                    connected,
                    edge_reached: reaches,
                });
            }
        }
    }
    RoadAudit {
        checks,
        pass: failures.is_empty(),
        failures,
    }
}

/// Water tile with irregular sand banks and broken foam on land-facing edges.
///
/// A set bit means land/sand is on that edge (the inverse of roads).
pub fn connected_shore(mask: u8, seed: &str) -> Canvas {
    let mut im = terrain_base("water", &format!("{seed}:water"));
    let mut rng = Rng::new(Seed::new(stable_seed(&format!("{seed}:shoreline"))));
    for edge in 0..4 {
        if mask & (1 << edge) == 0 {
            continue;
        }
        if edge == 0 || edge == 2 {
            // North/South banks: per-column depth, jittered every third column.
            let depths: Vec<i32> = (0..32)
                .map(|i| if i % 3 == 0 { rng.randrange2(5, 9) } else { 6 })
                .collect();
            for (x, depth) in depths.iter().enumerate() {
                let x = x as i32;
                let (y0, y1) = if edge == 0 {
                    (0, depth - 1)
                } else {
                    (32 - depth, 31)
                };
                im.line(x, y0, x, y1, color("sand"), 1);
                if x % 4 == 0 {
                    let yy = if edge == 0 { depth - 1 } else { 32 - depth };
                    im.set_pixel(x, yy, color("sand_light"));
                }
            }
            // Dark wet-sand contour plus discontinuous foam water-side.
            for (x, depth) in depths.iter().enumerate() {
                let x = x as i32;
                let boundary = if edge == 0 { *depth } else { 31 - *depth };
                im.set_pixel(x, boundary, color("sand_dark"));
                if x % 5 == 0 || x % 5 == 1 {
                    let fy = if edge == 0 {
                        boundary + 1
                    } else {
                        boundary - 1
                    };
                    if (0..32).contains(&fy) {
                        im.set_pixel(x, fy, color("white"));
                    }
                }
            }
        } else {
            // West/East banks: per-row depth.
            let depths: Vec<i32> = (0..32)
                .map(|i| if i % 3 == 0 { rng.randrange2(5, 9) } else { 6 })
                .collect();
            for (y, depth) in depths.iter().enumerate() {
                let y = y as i32;
                let (x0, x1) = if edge == 3 {
                    (0, depth - 1)
                } else {
                    (32 - depth, 31)
                };
                im.line(x0, y, x1, y, color("sand"), 1);
                if y % 4 == 0 {
                    let xx = if edge == 3 { depth - 1 } else { 32 - depth };
                    im.set_pixel(xx, y, color("sand_light"));
                }
            }
            for (y, depth) in depths.iter().enumerate() {
                let y = y as i32;
                let boundary = if edge == 3 { *depth } else { 31 - *depth };
                im.set_pixel(boundary, y, color("sand_dark"));
                if y % 5 == 0 || y % 5 == 1 {
                    let fx = if edge == 3 {
                        boundary + 1
                    } else {
                        boundary - 1
                    };
                    if (0..32).contains(&fx) {
                        im.set_pixel(fx, y, color("white"));
                    }
                }
            }
        }
    }
    im
}

/// Cliff tile: rock base with visible escarpments on unconnected edges.
pub fn cliff(mask: u8, seed: &str) -> Canvas {
    let mut im = terrain_base("rock", &format!("{seed}:cliff:{mask}"));
    // Edge bands N/E/S/W.
    let boxes = [
        (0, 0, 31, 5),
        (26, 0, 31, 31),
        (0, 26, 31, 31),
        (0, 0, 5, 31),
    ];
    for (i, &(x0, y0, x1, y1)) in boxes.iter().enumerate() {
        if mask & (1 << i) == 0 {
            im.rect(x0, y0, x1, y1, Some(color("rock_dark")), None, 0);
            // Rock-light lip along the inner side of the escarpment.
            match i {
                0 => im.line(0, 5, 31, 5, color("rock_light"), 1),
                1 => im.line(26, 0, 26, 31, color("rock_light"), 1),
                2 => im.line(0, 26, 31, 26, color("rock_light"), 1),
                _ => im.line(5, 0, 5, 31, color("rock_light"), 1),
            };
        }
    }
    im
}

/// Wall segment tile: dirt base with metal strips on connected edges.
pub fn wall(mask: u8, _seed: &str) -> Canvas {
    // Python uses a plain dirt tile here (new_tile), not terrain_base.
    let mut im = Canvas::new_filled(32, 32, color("dirt"));
    im.rect(11, 0, 20, 31, Some(color("metal_dark")), None, 0);
    im.rect(0, 11, 31, 20, Some(color("metal_dark")), None, 0);
    // Arm stubs repainted dirt where the wall does not connect.
    let stubs = [
        (11, 0, 20, 7),
        (24, 11, 31, 20),
        (11, 24, 20, 31),
        (0, 11, 7, 20),
    ];
    for (i, &(x0, y0, x1, y1)) in stubs.iter().enumerate() {
        if mask & (1 << i) == 0 {
            im.rect(x0, y0, x1, y1, Some(color("dirt")), None, 0);
        }
    }
    im.rect(13, 13, 18, 18, Some(color("metal")), None, 0);
    im
}

/// Bridge deck over water: `orientation` is `"horizontal"` or `"vertical"`;
/// `condition` is `"intact"`, `"damaged"` or `"destroyed"`.
pub fn bridge(orientation: &str, condition: &str, seed: &str) -> Canvas {
    let mut im = terrain_base("water", &format!("{seed}:bridge:{orientation}:{condition}"));
    let horizontal = orientation != "vertical";
    type Rect = (i32, i32, i32, i32);
    let (box_, planks): (Rect, Vec<Rect>) = if horizontal {
        (
            (0, 9, 31, 22),
            (0..32).step_by(4).map(|x| (x, 10, x + 2, 21)).collect(),
        )
    } else {
        (
            (9, 0, 22, 31),
            (0..32).step_by(4).map(|y| (10, y, 21, y + 2)).collect(),
        )
    };
    let deck = if condition == "destroyed" {
        color("dirt_dark")
    } else {
        color("dirt")
    };
    im.rect(
        box_.0,
        box_.1,
        box_.2,
        box_.3,
        Some(deck),
        Some(color("metal_dark")),
        1,
    );
    if condition != "destroyed" {
        let plank = if condition == "intact" {
            color("dirt_light")
        } else {
            color("dirt_dark")
        };
        for (x0, y0, x1, y1) in planks {
            im.rect(x0, y0, x1, y1, Some(plank), None, 0);
        }
    }
    if condition != "intact" {
        // Damage cracks.
        for (x, y) in [(7, 7), (24, 24), (8, 23)] {
            im.line(x, y, x + 4, y + 4, color("rock_dark"), 2);
        }
    }
    im
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash(im: &Canvas) -> Vec<u8> {
        im.as_rgba().as_raw().clone()
    }

    #[test]
    fn road_continuity_audit_passes() {
        let audit = audit_road_continuity("road-audit");
        assert_eq!(audit.checks, 64);
        assert!(
            audit.pass,
            "road continuity failures: {:?}",
            audit.failures.iter().take(4).collect::<Vec<_>>()
        );
    }

    #[test]
    fn roads_are_deterministic_and_distinct_per_mask() {
        let mut seen = std::collections::HashSet::new();
        for mask in 0..16u8 {
            let a = connected_road(mask, "pack:road");
            let b = connected_road(mask, "pack:road");
            assert_eq!((a.width(), a.height()), (32, 32));
            assert_eq!(hash(&a), hash(&b));
            seen.insert(hash(&a));
        }
        assert_eq!(seen.len(), 16, "each road mask must render differently");
    }

    #[test]
    fn cross_reaches_all_edges_isolated_reaches_none() {
        let cross = connected_road(15, "pack:road");
        for edge in DIRECTION_NAMES {
            let sig = road_edge_signature(&cross, edge);
            assert!(sig[11..=21].iter().any(|&b| b), "cross misses edge {edge}");
        }
        let isolated = connected_road(0, "pack:road");
        for edge in DIRECTION_NAMES {
            let sig = road_edge_signature(&isolated, edge);
            assert!(!sig.iter().any(|&b| b), "isolated reaches edge {edge}");
        }
    }

    #[test]
    fn shoreline_is_deterministic_and_edge_aware() {
        let a = connected_shore(1, "pack:shore");
        let b = connected_shore(1, "pack:shore");
        assert_eq!(hash(&a), hash(&b));
        // Mask 0 is a pure water base (no land edges).
        let pure = connected_shore(0, "pack:shore");
        assert_eq!(pure.pixel(1, 1), Some(color("water")));
        // All-land mask has sand at every corner edge.
        let full = connected_shore(15, "pack:shore");
        let sand_family = [color("sand"), color("sand_light"), color("sand_dark")];
        for (x, y) in [(1, 1), (30, 1), (1, 30), (30, 30)] {
            let px = full.pixel(x, y).unwrap();
            assert!(sand_family.contains(&px), "corner ({x},{y}) = {px:?}");
        }
    }

    #[test]
    fn cliffs_and_walls_follow_masks() {
        let a = cliff(5, "pack:cliff");
        let b = cliff(5, "pack:cliff");
        assert_eq!(hash(&a), hash(&b));
        assert_ne!(
            hash(&cliff(0, "pack:cliff")),
            hash(&cliff(15, "pack:cliff"))
        );
        // Mask 0: all four escarpments present -> rock_dark band at south edge.
        assert_eq!(cliff(0, "c").pixel(1, 30), Some(color("rock_dark")));
        // Mask 15: no escarpments -> plain rock base.
        assert_ne!(cliff(15, "c").pixel(1, 30), Some(color("rock_dark")));

        let w0 = wall(0, "c");
        let w15 = wall(15, "c");
        assert_eq!(hash(&w0), hash(&wall(0, "c")));
        assert_ne!(hash(&w0), hash(&w15));
        // Mask 0: nothing connected -> all stubs repainted dirt over the
        // metal cross; mask 15: all connected -> metal strips intact.
        assert_eq!(w0.pixel(15, 2), Some(color("dirt"))); // N stub, mask 0
        assert_eq!(w0.pixel(2, 15), Some(color("dirt"))); // W stub, mask 0
        assert_eq!(w15.pixel(15, 2), Some(color("metal_dark"))); // N connected
        assert_eq!(w15.pixel(2, 15), Some(color("metal_dark"))); // W connected
    }

    #[test]
    fn bridges_cover_conditions_and_orientations() {
        let mut seen = std::collections::HashSet::new();
        for orientation in ["horizontal", "vertical"] {
            for condition in ["intact", "damaged", "destroyed"] {
                let im = bridge(orientation, condition, "pack:bridge");
                assert_eq!((im.width(), im.height()), (32, 32));
                let again = bridge(orientation, condition, "pack:bridge");
                assert_eq!(hash(&im), hash(&again));
                seen.insert(hash(&im));
            }
        }
        assert_eq!(seen.len(), 6, "all 6 bridge variants must differ");
    }

    #[test]
    fn road_mask_names_cover_all_masks_uniquely() {
        let names: Vec<&str> = (0..16u8).map(road_mask_name).collect();
        let unique: std::collections::HashSet<&&str> = names.iter().collect();
        assert_eq!(names.len(), 16);
        assert_eq!(unique.len(), 16);
        assert_eq!(road_mask_name(15), "cross");
        assert_eq!(road_mask_name(0), "isolated");
    }
}
