//! Tileset pack addressing and per-cell tile decisions.
//!
//! Everything here mirrors the authoritative 16×16 RTS pack layout authored by
//! `assets/packs/rts_16x16_tileset/generate_rts_tileset.py`. Reference points
//! in that script:
//!
//! - terrain sheet rows (`GROUND_ROWS` ~line 75): 0 grass, 1 dirt, 2 sand,
//!   3 snow, 4 rock, 5 tiberium_ground, 6 ore_ground, 7 mud, 8 crater,
//!   9 rough, 10 forest_floor, 11 jungle_floor, 12 swamp, 13 scorched,
//!   14 concrete, 15 ice. Columns are texture variations.
//! - transitions (`TRANSITION_ROWS` ~line 86 / `transition_tile` ~line 279):
//!   the row picks the terrain pair (base A, invading B); the 16 columns are
//!   4-bit edge masks showing which sides render terrain B: **N=1, E=2,
//!   S=4, W=8**. Only rows whose base A matches the cell's own terrain can
//!   be used (the pack has no mirrored variants).
//! - water edges (`WATER_ROWS` ~line 105 / `water_tile` ~line 320): rows
//!   0-9 are deep/shallow/shores, 10-11 river flow, 12-13 ice, 14 wetland,
//!   15 reef. Shore family names point at the side the *water art* occupies
//!   a land tile; drawn on a WATER cell the regions rotate 180° — a water
//!   cell with a land neighbour at north needs water in its own south half,
//!   which is the `shore_south` art (row 4).
//! - cliffs (`CLIFF_ROWS` ~line 116 / `cliff_tile` ~line 391): faces draw on
//!   the LOW cell with the rock half attached to the HIGH side (`E` fills
//!   the east half, `W` the west, `S` the full column with the rim at mid
//!   height, `N` the bottom half with the rim near the top).
//! - roads (`ROAD_ROWS` ~line 127 / `road_tile` routes ~line 462) and walls
//!   (`WALL_ROWS` ~line 138): rows are named for the arms they connect —
//!   `curve_ne` joins the north and east edges, `t_n` is a T-junction open
//!   to the north (arms E/W/S), and so on.
//! - decorations (RGBA) row 0 cols 0-3 are tree sprites; resources_fx row 0:
//!   tiberium cols 0/1/14, ore cols 3/4, flag col 12.
//!
//! All per-cell column decisions derive from the deterministic variant hash
//! `(x*7 + y*13) % 16`, never from RNG.

use crate::generator::{OverlayType, TerrainType};
use crate::map::MapModel;
use cnc_core::Vec2i;

/// Category sheet names inside the pack.
pub const TERRAIN: &str = "terrain";
pub const TRANSITIONS: &str = "transitions";
pub const WATER_EDGES: &str = "water_edges";
pub const CLIFFS: &str = "cliffs";
pub const ROADS_BRIDGES: &str = "roads_bridges";
pub const WALLS_FENCES: &str = "walls_fences";
pub const DECORATIONS: &str = "decorations";
pub const RESOURCES_FX: &str = "resources_fx";

/// Edge bits shared by every mask family: transition columns, water shore
/// sides, road/wall arms and cliff drop directions all use N=1, E=2, S=4, W=8.
pub const BIT_N: u8 = 1;
pub const BIT_E: u8 = 2;
pub const BIT_S: u8 = 4;
pub const BIT_W: u8 = 8;

/// Elevation difference (0-255 scale) that counts as a sheer cliff.
pub const CLIFF_STEP: i32 = 3;
/// Slope band (1..=CLIFF_STEP-1) that renders as a ramp instead of a wall.
pub const RAMP_STEP: i32 = 1;

/// One tile pick: category, semantic family, fallback manifest row and the
/// deterministic column inside the row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileDraw {
    pub category: &'static str,
    pub family: &'static str,
    /// Row used when the palette manifest lacks `family`.
    pub row: u8,
    pub col: u8,
}

/// Deterministic per-cell texture variant: the pack's column-variant
/// convention `(x*7 + y*13) % 16`.
pub fn variant_col(x: i32, y: i32) -> u8 {
    (((x.max(0) as u32 % 16) * 7 + (y.max(0) as u32 % 16) * 13) % 16) as u8
}

/// Pick one entry of `choices` with the scenery hash; deterministic per cell.
fn pick(choices: &[u8], x: i32, y: i32) -> u8 {
    choices[(variant_col(x, y) as usize) % choices.len()]
}

/// Terrain id -> (category, family, fallback manifest row).
pub fn terrain_family(id: u8) -> Option<(&'static str, &'static str, u8)> {
    match TerrainType::from_id(id)? {
        TerrainType::Grass => Some((TERRAIN, "grass", 0)),
        TerrainType::Water => Some((WATER_EDGES, "deep_water", 0)),
        TerrainType::Sand => Some((TERRAIN, "sand", 2)),
        TerrainType::Dirt => Some((TERRAIN, "dirt", 1)),
        TerrainType::Rock => Some((TERRAIN, "rock", 4)),
        TerrainType::Snow => Some((TERRAIN, "snow", 3)),
        TerrainType::Rough => Some((TERRAIN, "rough", 9)),
        TerrainType::Mud => Some((TERRAIN, "mud", 7)),
        TerrainType::Crater => Some((TERRAIN, "crater", 8)),
        TerrainType::Scorched => Some((TERRAIN, "scorched", 13)),
        TerrainType::Swamp => Some((TERRAIN, "swamp", 12)),
        TerrainType::Ice => Some((TERRAIN, "ice", 15)),
        TerrainType::Concrete => Some((TERRAIN, "concrete", 14)),
        TerrainType::ForestFloor => Some((TERRAIN, "forest_floor", 10)),
        TerrainType::JungleFloor => Some((TERRAIN, "jungle_floor", 11)),
    }
}

/// Ground underlay row for a resource/tree overlay, or None.
fn underlay_family(overlay: u8) -> Option<(&'static str, u8)> {
    if overlay == OverlayType::Tiberium.id() {
        Some(("tiberium_ground", 5))
    } else if overlay == OverlayType::Ore.id() {
        Some(("ore_ground", 6))
    } else if overlay == OverlayType::Trees.id() {
        Some(("forest_floor", 10))
    } else {
        None
    }
}

/// Shore tile for a WATER cell given the bitmask of land neighbours
/// (N=1, E=2, S=4, W=8). None means open water (deep/shallow).
pub fn water_shore_family(land_mask: u8) -> Option<(&'static str, u8)> {
    // Masks are literal edge-mask values: N=1 E=2 S=4 W=8.
    match land_mask {
        0b0001 => Some(("shore_south", 4)), // land at north
        0b0010 => Some(("shore_west", 5)),  // land at east
        0b0100 => Some(("shore_north", 2)), // land at south
        0b1000 => Some(("shore_east", 3)),  // land at west
        0b0011 => Some(("shore_sw", 8)),    // land at north+east
        0b0110 => Some(("shore_nw", 6)),    // land at east+south
        0b1001 => Some(("shore_se", 9)),    // land at north+west
        0b1100 => Some(("shore_ne", 7)),    // land at south+west
        // Three land sides: only one side stays open to water.
        0b1110 => Some(("shore_north", 2)),
        0b1101 => Some(("shore_east", 3)),
        0b1011 => Some(("shore_south", 4)),
        0b0111 => Some(("shore_west", 5)),
        // Water ringed by land on all four sides has no fitting shore art.
        _ => None,
    }
}

/// What a transition row's invading terrain B can be on a neighbour cell.
#[derive(Clone, Copy)]
enum Pair {
    /// Neighbour terrain id.
    Terrain(u8),
    /// Neighbour carries a Tiberium overlay (pack row 12).
    Tiberium,
    /// Neighbour carries an Ore overlay (pack row 13).
    Ore,
}

/// `(base terrain A, invading B, family, fallback row)` for every usable
/// transition row. Rows 4/8/13 (mud, concrete, ore-ground pairs) extend the
/// flat terrain palette; rows 12/14 match resource alongside terrain.
const TRANSITION_ROWS: &[(u8, Pair, &str, u8)] = &[
    (
        TerrainType::Grass.id(),
        Pair::Terrain(TerrainType::Dirt.id()),
        "grass_dirt",
        0,
    ),
    (
        TerrainType::Grass.id(),
        Pair::Terrain(TerrainType::Sand.id()),
        "grass_sand",
        1,
    ),
    (
        TerrainType::Grass.id(),
        Pair::Terrain(TerrainType::Rock.id()),
        "grass_rock",
        2,
    ),
    (
        TerrainType::Grass.id(),
        Pair::Terrain(TerrainType::Snow.id()),
        "grass_snow",
        3,
    ),
    (
        TerrainType::Grass.id(),
        Pair::Terrain(TerrainType::Mud.id()),
        "grass_mud",
        4,
    ),
    (
        TerrainType::Grass.id(),
        Pair::Tiberium,
        "grass_tiberium",
        12,
    ),
    (
        TerrainType::Grass.id(),
        Pair::Terrain(TerrainType::Swamp.id()),
        "grass_swamp",
        14,
    ),
    (
        TerrainType::Dirt.id(),
        Pair::Terrain(TerrainType::Sand.id()),
        "dirt_sand",
        5,
    ),
    (
        TerrainType::Dirt.id(),
        Pair::Terrain(TerrainType::Rock.id()),
        "dirt_rock",
        6,
    ),
    (
        TerrainType::Dirt.id(),
        Pair::Terrain(TerrainType::Snow.id()),
        "dirt_snow",
        7,
    ),
    (
        TerrainType::Dirt.id(),
        Pair::Terrain(TerrainType::Concrete.id()),
        "dirt_concrete",
        8,
    ),
    (TerrainType::Dirt.id(), Pair::Ore, "dirt_ore", 13),
    (
        TerrainType::Sand.id(),
        Pair::Terrain(TerrainType::Rock.id()),
        "sand_rock",
        9,
    ),
    (
        TerrainType::Sand.id(),
        Pair::Terrain(TerrainType::Ice.id()),
        "sand_shore",
        10,
    ),
    (
        TerrainType::Rock.id(),
        Pair::Terrain(TerrainType::Snow.id()),
        "rock_snow",
        11,
    ),
    (
        TerrainType::Rough.id(),
        Pair::Terrain(TerrainType::Scorched.id()),
        "rough_scorch",
        15,
    ),
];

/// Whether the neighbour (terrain id, overlay id) matches a transition pair.
fn pair_hit(pair: Pair, neighbour: (u8, u8)) -> bool {
    let (terrain, overlay) = neighbour;
    match pair {
        Pair::Terrain(id) => terrain == id,
        Pair::Tiberium => overlay == OverlayType::Tiberium.id(),
        Pair::Ore => overlay == OverlayType::Ore.id(),
    }
}

/// Transition pick for a cell whose terrain id is `base`: chooses the family
/// whose invading terrain B surrounds the cell on the most sides and returns
/// `(family, fallback row, edge mask)`. Ties break on the lowest row.
fn transition_pick(
    base: u8,
    neighbour: impl Fn(i32, i32) -> Option<(u8, u8)>,
) -> Option<(&'static str, u8, u8)> {
    let mut best: Option<(u32, u8, &'static str, u8)> = None;
    for &(a, pair, family, row) in TRANSITION_ROWS {
        if a != base {
            continue;
        }
        let mask = (0..4).fold(0u8, |acc, dir| {
            let (dx, dy, bit) = match dir {
                0 => (0, -1, BIT_N),
                1 => (1, 0, BIT_E),
                2 => (0, 1, BIT_S),
                _ => (-1, 0, BIT_W),
            };
            if neighbour(dx, dy).is_some_and(|cell| pair_hit(pair, cell)) {
                acc | bit
            } else {
                acc
            }
        });
        if mask == 0 {
            continue;
        }
        let sides = mask.count_ones();
        let better = match best {
            None => true,
            Some((best_sides, best_row, _, _)) => {
                sides > best_sides || (sides == best_sides && row < best_row)
            }
        };
        if better {
            best = Some((sides, row, family, mask));
        }
    }
    best.map(|(_, row, family, mask)| (family, row, mask))
}

/// Road tile family from the bitmask of road-connected arms
/// (N=1, E=2, S=4, W=8). Straight and isolated stretches default to the
/// pack's dirt-track rows (11/12/13); corners, T-junctions and curves have
/// no dirt art and use the paved rows 0-10 (`road_tile` routes ~line 462).
pub fn road_family(arms: u8) -> (&'static str, u8) {
    // Masks are literal arm values: N=1 E=2 S=4 W=8.
    match arms {
        0 | 0b0001 | 0b0100 | 0b0101 => ("dirt_track_ns", 11),
        0b0010 | 0b1000 | 0b1010 => ("dirt_track_ew", 12),
        0b0011 => ("paved_curve_ne", 3), // N+E
        0b1001 => ("paved_curve_nw", 4), // N+W
        0b0110 => ("paved_curve_se", 5), // S+E
        0b1100 => ("paved_curve_sw", 6), // S+W
        0b1110 => ("paved_t_n", 7),      // open north
        0b1101 => ("paved_t_e", 8),      // open east
        0b0111 => ("paved_t_w", 10),     // open west
        0b1011 => ("paved_t_s", 9),      // open south
        _ => ("dirt_track_cross", 13),   // all four arms
    }
}

/// Wall tile family from the bitmask of wall-connected arms.
pub fn wall_family(arms: u8) -> (&'static str, u8) {
    // Masks are literal arm values: N=1 E=2 S=4 W=8.
    match arms {
        0 | 0b0001 | 0b0100 | 0b0101 => ("concrete_ns", 0),
        0b0010 | 0b1000 | 0b1010 => ("concrete_ew", 1),
        0b0011 => ("concrete_corner_ne", 2), // N+E
        0b1001 => ("concrete_corner_nw", 3), // N+W
        0b0110 => ("concrete_corner_se", 4), // S+E
        0b1100 => ("concrete_corner_sw", 5), // S+W
        0b1110 | 0b1101 | 0b0111 | 0b1011 => ("concrete_t", 6),
        _ => ("concrete_cross", 7), // all four arms
    }
}

/// Cliff face family for a LOW cell given the bitmask of HIGH neighbours
/// (elevation at least [`CLIFF_STEP`] above this cell). Corner pairs pick
/// the quadrant art whose two sides are high; three or more high sides are
/// interiors.
pub fn cliff_family(high_mask: u8) -> Option<(&'static str, u8)> {
    // Masks are literal edge-mask values: N=1 E=2 S=4 W=8.
    match high_mask {
        0 => None,
        0b0001 => Some(("cliff_south", 1)), // high at north
        0b0010 => Some(("cliff_east", 2)),  // high at east
        0b0100 => Some(("cliff_north", 0)), // high at south
        0b1000 => Some(("cliff_west", 3)),  // high at west
        0b0011 => Some(("cliff_corner_ne", 5)),
        0b1001 => Some(("cliff_corner_nw", 4)),
        0b0110 => Some(("cliff_corner_se", 7)),
        0b1100 => Some(("cliff_corner_sw", 6)),
        _ => Some(("cliff_interior", 8)),
    }
}

/// Base tile for the cell (x, y): auto-connected overlays, water shores,
/// reefs, ice edges, resource underlays, elevation cliffs/ramps/plateaus
/// and terrain transition masks. Returns None only for out-of-bounds cells.
/// Callers resolve [`TileDraw`] through the pack and fall back to flat
/// colours whenever the pack lacks the addressed tile.
pub fn base_draw(model: &MapModel, x: i32, y: i32) -> Option<TileDraw> {
    let here = Vec2i::new(x, y);
    let index = model.index(here)?;
    let terrain = model.terrain[index];
    let overlay = model.overlay[index];
    let variant = variant_col(x, y);

    let neighbour = |dx: i32, dy: i32| -> Option<(u8, u8, i32)> {
        model.index(Vec2i::new(x + dx, y + dy)).map(|i| {
            (
                model.terrain[i],
                model.overlay[i],
                model.elevation.get(i).copied().unwrap_or(0) as i32,
            )
        })
    };
    let neighbour_terrain = |dx: i32, dy: i32| neighbour(dx, dy).map(|cell| cell.0);

    let mask_over = |predicate: &dyn Fn(Option<(u8, u8, i32)>) -> bool| -> u8 {
        let mut mask = 0u8;
        if predicate(neighbour(0, -1)) {
            mask |= BIT_N;
        }
        if predicate(neighbour(1, 0)) {
            mask |= BIT_E;
        }
        if predicate(neighbour(0, 1)) {
            mask |= BIT_S;
        }
        if predicate(neighbour(-1, 0)) {
            mask |= BIT_W;
        }
        mask
    };

    // 1. Auto-connected overlay kinds render their own tiles outright.
    if overlay == OverlayType::Road.id() {
        let mask = mask_over(&|cell| cell.is_some_and(|(_, o, _)| o == OverlayType::Road.id()));
        let (family, row) = road_family(mask);
        return Some(TileDraw {
            category: ROADS_BRIDGES,
            family,
            row,
            col: variant,
        });
    }
    if overlay == OverlayType::Wall.id() {
        let mask = mask_over(&|cell| cell.is_some_and(|(_, o, _)| o == OverlayType::Wall.id()));
        let (family, row) = wall_family(mask);
        return Some(TileDraw {
            category: WALLS_FENCES,
            family,
            row,
            col: variant,
        });
    }

    // 2. Terrain-specific adjacency: shores, reefs, ice edges.
    if terrain == TerrainType::Water.id() {
        let land = mask_over(&|cell| cell.is_some_and(|(t, _, _)| t != TerrainType::Water.id()));
        let (family, row) = match land {
            0 => {
                let diagonal_land = [(1, 1), (1, -1), (-1, 1), (-1, -1)]
                    .iter()
                    .any(|&(dx, dy)| {
                        neighbour_terrain(dx, dy).is_some_and(|t| t != TerrainType::Water.id())
                    });
                if diagonal_land {
                    ("shallow_water", 1)
                } else {
                    ("deep_water", 0)
                }
            }
            mask => water_shore_family(mask).unwrap_or(("deep_water", 0)),
        };
        return Some(TileDraw {
            category: WATER_EDGES,
            family,
            row,
            col: variant,
        });
    }

    let water_mask = mask_over(&|cell| cell.is_some_and(|(t, _, _)| t == TerrainType::Water.id()));
    if terrain == TerrainType::Rock.id() && water_mask != 0 {
        // Coastal rock draws the reef art instead of bare rock.
        return Some(TileDraw {
            category: WATER_EDGES,
            family: "reef_rocks",
            row: 15,
            col: variant,
        });
    }
    if terrain == TerrainType::Snow.id() && water_mask != 0 {
        // Snow meeting water renders as pack ice with an edge channel.
        return Some(TileDraw {
            category: WATER_EDGES,
            family: "ice_edge",
            row: 13,
            col: variant,
        });
    }

    // 3. Resource/tree underlays own their base ground.
    if let Some((family, row)) = underlay_family(overlay) {
        return Some(TileDraw {
            category: TERRAIN,
            family,
            row,
            col: variant,
        });
    }

    // 4. Elevation cliffs, ramps and plateaus (only where elevation is
    // painted; all-zero fields degrade to plain ground).
    let e = model.elevation.get(index).copied().unwrap_or(0) as i32;
    let clamp_e =
        |dx: i32, dy: i32| -> i32 { neighbour(dx, dy).map(|cell| cell.2).unwrap_or(e.max(0)) };
    let high_mask = mask_over(&|cell: Option<(u8, u8, i32)>| {
        cell.is_some_and(|(_, _, ne)| ne - e >= CLIFF_STEP)
    });
    if let Some((family, row)) = cliff_family(high_mask) {
        return Some(TileDraw {
            category: CLIFFS,
            family,
            row,
            col: variant,
        });
    }
    let (dn, de, ds, dw) = (
        (clamp_e(0, -1) - e).abs(),
        (clamp_e(1, 0) - e).abs(),
        (clamp_e(0, 1) - e).abs(),
        (clamp_e(-1, 0) - e).abs(),
    );
    let slope = |d: i32| (RAMP_STEP..=CLIFF_STEP - 1).contains(&d);
    if (slope(dn) || slope(ds)) && de == 0 && dw == 0 {
        // Moderate vertical gradient: run the ramp north–south.
        return Some(TileDraw {
            category: CLIFFS,
            family: "ramp_ns",
            row: 10,
            col: variant,
        });
    }
    if (slope(de) || slope(dw)) && dn == 0 && ds == 0 {
        // Moderate horizontal gradient: run the ramp east–west.
        return Some(TileDraw {
            category: CLIFFS,
            family: "ramp_ew",
            row: 11,
            col: variant,
        });
    }
    if dn <= 1
        && ds <= 1
        && de <= 1
        && dw <= 1
        && [(1, 1), (1, -1), (-1, 1), (-1, -1)]
            .iter()
            .any(|&(dx, dy)| e - clamp_e(dx, dy) >= CLIFF_STEP)
    {
        // Flat top bracketed by a steep diagonal drop: plateau cap.
        return Some(TileDraw {
            category: CLIFFS,
            family: "plateau_top",
            row: 9,
            col: variant,
        });
    }

    // 5. Terrain transition masks between open terrain pairs.
    if let Some((family, row, mask)) = transition_pick(terrain, |dx, dy| {
        neighbour(dx, dy).map(|cell| (cell.0, cell.1))
    }) {
        return Some(TileDraw {
            category: TRANSITIONS,
            family,
            row,
            col: mask,
        });
    }

    // 6. Plain terrain family.
    terrain_family(terrain).map(|(category, family, row)| TileDraw {
        category,
        family,
        row,
        col: variant,
    })
}

/// RGBA sprite layers stacked on the base tile (trees, resource crystals and
/// the player-start flag).
pub fn sprite_draws(model: &MapModel, x: i32, y: i32) -> Vec<TileDraw> {
    let here = Vec2i::new(x, y);
    let Some(index) = model.index(here) else {
        return Vec::new();
    };
    let mut draws = Vec::new();
    let overlay = model.overlay[index];
    if overlay == OverlayType::Tiberium.id() {
        draws.push(TileDraw {
            category: RESOURCES_FX,
            family: "tiberium",
            row: 0,
            col: pick(&[0, 1, 14], x, y),
        });
    } else if overlay == OverlayType::Ore.id() {
        draws.push(TileDraw {
            category: RESOURCES_FX,
            family: "ore",
            row: 0,
            col: pick(&[3, 4], x, y),
        });
    } else if overlay == OverlayType::Trees.id() {
        draws.push(TileDraw {
            category: DECORATIONS,
            family: "scenery",
            row: 0,
            col: pick(&[0, 1, 2, 3], x, y),
        });
    }
    if model.player_starts.contains(&here) {
        draws.push(TileDraw {
            category: RESOURCES_FX,
            family: "flag",
            row: 0,
            col: 12,
        });
    }
    draws
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(model: &mut MapModel, x: u32, y: u32, terrain: u8) {
        model.terrain[(y * model.width + x) as usize] = terrain;
    }

    fn overlay_at(model: &mut MapModel, x: u32, y: u32, overlay: u8) {
        model.overlay[(y * model.width + x) as usize] = overlay;
    }

    #[test]
    fn variant_hash_is_deterministic() {
        assert_eq!(variant_col(3, 4), ((3_u32 * 7 + 4 * 13) % 16) as u8);
        assert_eq!(variant_col(0, 0), 0);
        assert_eq!(variant_col(-1, -1), variant_col(0, 0));
        assert_eq!(variant_col(3, 4), variant_col(3, 4));
    }

    #[test]
    fn transition_family_pick_for_a_two_terrain_adjacency() {
        let mut model = MapModel::new(4, 4);
        model.elevation = vec![0; 16];
        for y in 0..4u32 {
            for x in 0..4u32 {
                at(
                    &mut model,
                    x,
                    y,
                    if x < 2 {
                        TerrainType::Grass.id()
                    } else {
                        TerrainType::Dirt.id()
                    },
                );
            }
        }
        // Grass cell bordered by dirt to the east only.
        let east = base_draw(&model, 1, 1).expect("grass edge cell draws");
        assert_eq!(east.category, TRANSITIONS);
        assert_eq!(east.family, "grass_dirt");
        assert_eq!(east.row, 0);
        assert_eq!(east.col, BIT_E, "only the east side invades");

        // Interior-west cell with pure grass neighbours stays plain terrain.
        let plain = base_draw(&model, 0, 0).unwrap();
        assert_eq!(plain.category, TERRAIN);
        assert_eq!(plain.family, "grass");

        // The dirt side of the border has no mirrored row: plain dirt.
        let dirt = base_draw(&model, 2, 1).unwrap();
        assert_eq!(dirt.category, TERRAIN);
        assert_eq!(dirt.family, "dirt");
    }

    #[test]
    fn transition_tie_breaks_on_lowest_row() {
        let mut model = MapModel::new(4, 1);
        model.terrain = vec![
            TerrainType::Grass.id(),
            TerrainType::Grass.id(),
            TerrainType::Sand.id(),
            TerrainType::Rock.id(),
        ];
        model.elevation = vec![0; 4];
        // Grass cell with sand east and rock west: grass_sand (row 1) wins
        // over grass_rock (row 2) on the tie. The column carries the chosen
        // family's own edge mask: only the sand side invades.
        let draw = base_draw(&model, 1, 0).unwrap();
        assert_eq!(draw.family, "grass_sand");
        assert_eq!(draw.col, BIT_E);
    }

    #[test]
    fn shore_bitmask_picks_shore_family() {
        assert_eq!(water_shore_family(BIT_N).map(|(_, r)| r), Some(4));
        assert_eq!(water_shore_family(BIT_S).map(|(_, r)| r), Some(2));
        assert_eq!(water_shore_family(BIT_E).map(|(_, r)| r), Some(5));
        assert_eq!(water_shore_family(BIT_W).map(|(_, r)| r), Some(3));
        assert_eq!(
            water_shore_family(BIT_N | BIT_E).map(|f| f.0),
            Some("shore_sw")
        );
        assert_eq!(
            water_shore_family(BIT_N | BIT_W).map(|f| f.0),
            Some("shore_se")
        );
        assert_eq!(
            water_shore_family(BIT_S | BIT_E).map(|f| f.0),
            Some("shore_nw")
        );
        assert_eq!(
            water_shore_family(BIT_S | BIT_W).map(|f| f.0),
            Some("shore_ne")
        );
        assert_eq!(water_shore_family(0), None, "open water has no shore");
        assert_eq!(
            water_shore_family(0b1111),
            None,
            "inland pond keeps deep water"
        );

        // End-to-end: the water cell at (1,2) with land to the north draws
        // the shore tile whose water half faces the open water (south).
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        for x in 0..3u32 {
            at(&mut model, x, 2, TerrainType::Water.id());
        }
        let south = base_draw(&model, 1, 2).unwrap();
        assert_eq!(south.category, WATER_EDGES);
        assert_eq!(south.family, "shore_south");
        // Fully surrounded water stays deep.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Water.id(); 9];
        let deep = base_draw(&model, 1, 1).unwrap();
        assert_eq!(deep.family, "deep_water");
    }

    #[test]
    fn road_and_wall_overlay_auto_connect() {
        // Vertical road: ns family.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        for y in 0..3u32 {
            overlay_at(&mut model, 1, y, OverlayType::Road.id());
        }
        let middle = base_draw(&model, 1, 1).unwrap();
        assert_eq!(middle.category, ROADS_BRIDGES);
        assert_eq!(middle.family, "dirt_track_ns");

        // Horizontal road: ew family.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        for x in 0..3u32 {
            overlay_at(&mut model, x, 1, OverlayType::Road.id());
        }
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "dirt_track_ew");

        // Four-way junction: cross family.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
            overlay_at(
                &mut model,
                (1 + dx) as u32,
                (1 + dy) as u32,
                OverlayType::Road.id(),
            );
        }
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "dirt_track_cross");

        // Curve connecting the north and east arms.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        overlay_at(&mut model, 1, 1, OverlayType::Road.id());
        overlay_at(&mut model, 1, 0, OverlayType::Road.id());
        overlay_at(&mut model, 2, 1, OverlayType::Road.id());
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "paved_curve_ne");

        // Walls follow the same bitmask logic.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        for y in 0..3u32 {
            overlay_at(&mut model, 1, y, OverlayType::Wall.id());
        }
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "concrete_ns");

        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        for x in 0..3u32 {
            overlay_at(&mut model, x, 1, OverlayType::Wall.id());
        }
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "concrete_ew");

        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        for (dx, dy) in [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)] {
            overlay_at(
                &mut model,
                (1 + dx) as u32,
                (1 + dy) as u32,
                OverlayType::Wall.id(),
            );
        }
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "concrete_cross");
    }

    #[test]
    fn resource_underlays_and_sprites_layer_over_terrain() {
        let mut model = MapModel::new(2, 2);
        model.terrain = vec![TerrainType::Grass.id(); 4];
        model.elevation = vec![0; 4];
        model.player_starts.push(Vec2i::new(0, 0));
        overlay_at(&mut model, 0, 0, OverlayType::Tiberium.id());
        overlay_at(&mut model, 1, 1, OverlayType::Ore.id());
        overlay_at(&mut model, 1, 0, OverlayType::Trees.id());

        let tib = base_draw(&model, 0, 0).unwrap();
        assert_eq!(tib.category, TERRAIN);
        assert_eq!(tib.family, "tiberium_ground");
        assert_eq!(tib.row, 5);

        let sprites = sprite_draws(&model, 0, 0);
        assert_eq!(sprites.len(), 2, "tiberium crystals plus the start flag");
        assert_eq!(sprites[0].category, RESOURCES_FX);
        assert!(matches!(sprites[0].col, 0 | 1 | 14));
        assert_eq!(sprites[1].category, RESOURCES_FX);
        assert_eq!(sprites[1].col, 12, "player start draws the flag sprite");

        let ore = base_draw(&model, 1, 1).unwrap();
        assert_eq!(ore.family, "ore_ground");
        let ore_sprite = &sprite_draws(&model, 1, 1)[0];
        assert!(matches!(ore_sprite.col, 3 | 4));

        let trees = base_draw(&model, 1, 0).unwrap();
        assert_eq!(trees.family, "forest_floor");
        let tree_sprite = &sprite_draws(&model, 1, 0)[0];
        assert_eq!(tree_sprite.category, DECORATIONS);
        assert!(matches!(tree_sprite.col, 0..=3));
    }

    #[test]
    fn elevation_draws_cliff_faces_ramps_and_plateaus() {
        // High mass north of a low centre: the wall band attaches to the
        // high side (full rock column, `cliff_south`).
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        model.elevation = vec![0; 9];
        model.elevation[1] = 9;
        model.elevation[4] = 4;
        let face = base_draw(&model, 1, 1).unwrap();
        assert_eq!(face.category, CLIFFS);
        assert_eq!(face.family, "cliff_south");
        let top = base_draw(&model, 1, 0).unwrap();
        assert_eq!(top.category, TERRAIN, "flat surroundings stay plain ground");

        // Moderate north slope: ramp north–south.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        model.elevation = vec![0; 9];
        model.elevation[1] = 2;
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "ramp_ns");

        // Moderate east slope: ramp east–west.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        model.elevation = vec![0; 9];
        model.elevation[5] = 2;
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "ramp_ew");

        // Flat top with a steep diagonal drop: plateau cap. The cardinal
        // neighbours stay level with the cap so only the diagonal sinks.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        model.elevation = vec![0; 9];
        model.elevation[4] = 9;
        model.elevation[1] = 9;
        model.elevation[3] = 9;
        model.elevation[5] = 9;
        model.elevation[7] = 9;
        model.elevation[8] = 3;
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "plateau_top");
    }

    #[test]
    fn unpainted_elevation_renders_plain_and_water_adjacent_work() {
        // Old maps without an [Elevation] section render cliff-free.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Grass.id(); 9];
        let plain = base_draw(&model, 1, 1).unwrap();
        assert_eq!(plain.category, TERRAIN);

        // Rock beside water draws the reef art.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Water.id(); 9];
        at(&mut model, 1, 1, TerrainType::Rock.id());
        let reef = base_draw(&model, 1, 1).unwrap();
        assert_eq!(reef.category, WATER_EDGES);
        assert_eq!(reef.family, "reef_rocks");

        // Snow beside water draws the ice edge; snow inland keeps its ground.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Water.id(); 9];
        at(&mut model, 1, 1, TerrainType::Snow.id());
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "ice_edge");
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Snow.id(); 9];
        let snow = base_draw(&model, 1, 1).unwrap();
        assert_eq!(snow.category, TERRAIN);
        assert_eq!(snow.family, "snow");

        // Diagonal-only land near water picks shallow water.
        let mut model = MapModel::new(3, 3);
        model.terrain = vec![TerrainType::Water.id(); 9];
        at(&mut model, 0, 0, TerrainType::Sand.id());
        assert_eq!(base_draw(&model, 1, 1).unwrap().family, "shallow_water");
    }
}
