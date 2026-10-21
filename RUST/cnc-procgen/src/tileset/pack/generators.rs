//! Sprite generators that were inline in the Python `build()` loop:
//! resources, map decorations and faction buildings.
//!
//! Ported 1:1 from `cnc32_asset_generator_v5.py` lines 606–702 (resource /
//! decoration / building sections). Seed strings and RNG call order match
//! Python; pixels differ (SplitMix64 instead of MT19937).

use super::super::canvas::Canvas;
use super::super::terrain::terrain_base;
use super::super::{color, stable_seed, TileRngExt};
use crate::prng::{Rng, Seed};

/// Resource kinds, in pack order.
pub const RESOURCE_KINDS: [&str; 2] = ["tiberium", "ore"];

/// Map decoration props, in pack order.
pub const DECORATION_PROPS: [&str; 6] = ["tree", "boulder", "crater", "wreck", "bush", "debris"];

/// (structure name, silhouette shape, draw box) — Python `structure_specs`.
pub type StructureSpec = (&'static str, &'static str, (i32, i32, i32, i32));

/// Building structure names with their silhouette shape and draw box, in
/// pack order. (`structure_specs` in Python.)
pub const STRUCTURES: [StructureSpec; 6] = [
    ("power_plant", "rect", (5, 8, 27, 25)),
    ("barracks", "wide", (3, 10, 29, 26)),
    ("refinery", "industrial", (3, 7, 29, 27)),
    ("war_factory", "hangar", (2, 8, 30, 27)),
    ("communications", "tower", (9, 5, 23, 27)),
    ("defense_turret", "turret", (7, 8, 25, 27)),
];

/// Building render conditions, in pack order.
pub const BUILDING_CONDITIONS: [&str; 4] = ["construction", "intact", "damaged", "destroyed"];

/// Resource field tile: diamond clusters on a grass base.
pub fn resource(kind: &str, variant: u8, seed: &str) -> Canvas {
    let mut im = Canvas::new_filled(32, 32, color("grass"));
    let mut rng = Rng::new(Seed::new(stable_seed(&format!("{seed}:{kind}:{variant}"))));
    let (col_name, highlight_name) = if kind == "tiberium" {
        ("tiberium", "grass_light")
    } else {
        ("sand", "sand_light")
    };
    for _ in 0..(5 + variant as i32 % 4) {
        let x = rng.randrange2(5, 27);
        let y = rng.randrange2(5, 27);
        let r = rng.randrange2(2, 5);
        im.polygon(
            &[(x, y - r), (x + r, y), (x, y + r), (x - r, y)],
            Some(color(col_name)),
            Some(color(highlight_name)),
        );
        im.set_pixel(x, y - r + 1, color(highlight_name));
    }
    im
}

/// Decoration tile: prop silhouette on a grass base.
pub fn decoration(prop: &str, variant: u8, seed: &str) -> Canvas {
    let mut im = terrain_base("grass", &format!("{seed}:prop:{prop}:{variant}"));
    let mut rng = Rng::new(Seed::new(stable_seed(&format!(
        "{seed}:prop:{prop}:{variant}"
    ))));
    let x = rng.randrange2(9, 23);
    let y = rng.randrange2(9, 23);
    match prop {
        "tree" => {
            im.rect(x - 1, y, x + 1, y + 8, Some(color("dirt_dark")), None, 0);
            im.ellipse(
                x - 6,
                y - 7,
                x + 6,
                y + 4,
                Some(color("grass_dark")),
                Some(color("grass_light")),
                1,
            );
        }
        "boulder" => im.ellipse(
            x - 5,
            y - 4,
            x + 5,
            y + 4,
            Some(color("rock_dark")),
            Some(color("rock_light")),
            1,
        ),
        "crater" => im.ellipse(
            x - 7,
            y - 4,
            x + 7,
            y + 4,
            Some(color("dirt_dark")),
            Some(color("dirt_light")),
            1,
        ),
        "wreck" => im.polygon(
            &[
                (x - 6, y - 2),
                (x + 3, y - 5),
                (x + 6, y + 3),
                (x - 3, y + 5),
            ],
            Some(color("metal_dark")),
            Some(color("metal")),
        ),
        "bush" => im.ellipse(
            x - 5,
            y - 3,
            x + 5,
            y + 3,
            Some(color("grass_dark")),
            Some(color("grass_light")),
            1,
        ),
        // debris: scattered rock-dark pixels
        _ => {
            for _ in 0..8 {
                im.set_pixel(
                    x + rng.randrange2(-5, 6),
                    y + rng.randrange2(-4, 5),
                    color("rock_dark"),
                );
            }
        }
    }
    im
}

/// Faction building tile with construction/intact/damaged/destroyed states.
///
/// Silhouettes, roof profiles, faction colors and damage marks follow the
/// Python `structure_specs` drawing exactly.
pub fn building(faction: &str, structure: &str, condition: &str, _seed: &str) -> Canvas {
    let entry = STRUCTURES
        .iter()
        .find(|(name, _, _)| *name == structure)
        .unwrap_or_else(|| panic!("unknown structure: {structure}"));
    let shape = entry.1;
    let (x0, y0, x1, y1) = entry.2;
    let mut im = Canvas::new_filled(32, 32, color("grass"));
    let faction_color = color(if faction == "gdi" { "gdi" } else { "nod" });
    let faction_dark = color(if faction == "gdi" {
        "gdi_dark"
    } else {
        "nod_dark"
    });
    if condition == "destroyed" {
        im.polygon(
            &[
                (x0 + 2, y1 - 2),
                (x0 + 5, y0 + 8),
                (x0 + 10, y1 - 5),
                (x1 - 3, y0 + 10),
                (x1 - 1, y1 - 1),
            ],
            Some(color("metal_dark")),
            Some(color("rock_dark")),
        );
        im.line(x0 + 4, y1 - 4, x1 - 4, y0 + 8, color("rock_light"), 1);
        return im;
    }
    if condition == "construction" {
        im.rect(x0, y0, x1, y1, None, Some(color("sand_light")), 2);
        let mut xx = x0 + 2;
        while xx < x1 {
            im.line(xx, y0, xx, y1, color("sand"), 1);
            xx += 5;
        }
        im.rect(
            x0 + 2,
            y1 - 4,
            x1 - 2,
            y1 - 2,
            Some(color("rock_dark")),
            None,
            0,
        );
        return im;
    }
    // Intact and damaged share the main structure drawing.
    im.rect(
        x0,
        y0,
        x1,
        y1,
        Some(faction_dark),
        Some(color("metal_dark")),
        1,
    );
    let roof = if faction == "gdi" {
        faction_color
    } else {
        color("nod")
    };
    match shape {
        "tower" => {
            im.rect(x0 + 4, y0 + 4, x1 - 4, y1 - 3, Some(roof), None, 0);
            im.line(16, y0 - 2, 16, y0 + 5, color("metal"), 2);
        }
        "turret" => {
            im.polygon(
                &[
                    (x0 + 2, y1 - 2),
                    (x0 + 4, y0 + 6),
                    (16, y0 + 1),
                    (x1 - 4, y0 + 6),
                    (x1 - 2, y1 - 2),
                ],
                Some(roof),
                None,
            );
            im.ellipse(
                12,
                y0 + 5,
                20,
                y0 + 13,
                Some(color("metal")),
                Some(color("metal_dark")),
                1,
            );
            im.line(16, y0 + 9, 16, y0 + 1, color("metal"), 2);
        }
        "hangar" => {
            im.polygon(
                &[(x0 + 1, y0 + 6), (16, y0), (x1 - 1, y0 + 6)],
                Some(roof),
                None,
            );
            im.rect(12, y0 + 10, 20, y1 - 1, Some(color("metal_dark")), None, 0);
        }
        "industrial" => {
            im.rect(
                x0 + 3,
                y0 + 3,
                x0 + 9,
                y0 + 10,
                Some(color("metal")),
                None,
                0,
            );
            im.rect(
                x1 - 9,
                y0 + 2,
                x1 - 4,
                y0 + 12,
                Some(color("metal_dark")),
                None,
                0,
            );
            im.rect(x0 + 4, y1 - 8, x1 - 4, y1 - 3, Some(roof), None, 0);
        }
        // "rect" and "wide" share the plain block profile.
        _ => {
            im.rect(x0 + 3, y0 + 4, x1 - 3, y1 - 4, Some(roof), None, 0);
            im.rect(14, y1 - 8, 18, y1 - 1, Some(color("metal_dark")), None, 0);
        }
    }
    im.rect(
        x0 + 2,
        y0 + 2,
        x1 - 2,
        y0 + 4,
        Some(color("metal")),
        None,
        0,
    );
    if condition == "damaged" {
        im.polygon(
            &[
                (x0 + 3, y0 + 2),
                (x0 + 9, y0 + 5),
                (x0 + 6, y0 + 11),
                (x0 + 12, y0 + 14),
                (x0 + 8, y0 + 18),
            ],
            Some(color("rock_dark")),
            None,
        );
        im.set_pixel(x1 - 4, y0 + 5, color("fire"));
    }
    im
}

/// Footprint for a structure (2×2 for the big three, else 1×1).
pub fn structure_footprint(structure: &str) -> [u32; 2] {
    if matches!(structure, "refinery" | "war_factory" | "barracks") {
        [2, 2]
    } else {
        [1, 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash(im: &Canvas) -> Vec<u8> {
        im.as_rgba().as_raw().clone()
    }

    #[test]
    fn resources_are_deterministic_and_distinct() {
        let mut seen = std::collections::HashSet::new();
        for kind in RESOURCE_KINDS {
            let a = resource(kind, 0, "pack");
            assert_eq!(hash(&a), hash(&resource(kind, 0, "pack")));
            assert_eq!((a.width(), a.height()), (32, 32));
            for variant in 0..8 {
                seen.insert(hash(&resource(kind, variant, "pack")));
            }
        }
        assert_eq!(seen.len(), 16, "16 resource tiles must all differ");
    }

    #[test]
    fn decorations_are_deterministic_and_distinct() {
        let mut seen = std::collections::HashSet::new();
        for prop in DECORATION_PROPS {
            for variant in 0..4 {
                let a = decoration(prop, variant, "pack");
                assert_eq!(hash(&a), hash(&decoration(prop, variant, "pack")));
                assert_eq!((a.width(), a.height()), (32, 32));
                seen.insert(hash(&a));
            }
        }
        assert_eq!(seen.len(), 24, "24 decoration tiles must all differ");
    }

    #[test]
    fn buildings_are_deterministic_and_conditions_differ() {
        for faction in ["gdi", "nod"] {
            for (name, _, _) in STRUCTURES {
                let intact = hash(&building(faction, name, "intact", "pack"));
                assert_eq!(intact, hash(&building(faction, name, "intact", "pack")));
                for condition in ["construction", "damaged", "destroyed"] {
                    assert_ne!(
                        hash(&building(faction, name, condition, "pack")),
                        intact,
                        "{faction}/{name}/{condition}"
                    );
                }
            }
        }
    }

    #[test]
    fn factions_render_buildings_differently() {
        let gdi = hash(&building("gdi", "power_plant", "intact", "pack"));
        let nod = hash(&building("nod", "power_plant", "intact", "pack"));
        assert_ne!(gdi, nod);
    }

    #[test]
    fn big_structures_have_2x2_footprints() {
        assert_eq!(structure_footprint("refinery"), [2, 2]);
        assert_eq!(structure_footprint("war_factory"), [2, 2]);
        assert_eq!(structure_footprint("barracks"), [2, 2]);
        assert_eq!(structure_footprint("power_plant"), [1, 1]);
    }
}
