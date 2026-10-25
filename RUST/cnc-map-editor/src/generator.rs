//! Procedural 2D map generation.
//!
//! The generator is deliberately Minecraft-shaped rather than Minecraft-copied:
//! a coordinate-hashed value-noise field is summed octave by octave (fBm) to
//! produce elevation, a second field produces moisture, and the combination is
//! classified into water, beach, grass, dirt, rock and snow. Resource fields
//! (tiberium/ore) and tree overlays are scattered with independent noise, and
//! player starts use farthest-point sampling so they are both on land and far
//! apart. Everything is a pure function of the seed, so a seed reproduces the
//! identical map on every machine.

use crate::map::{MapModel, MapObject};
use crate::plugin::Theater;
use cnc_core::{ClassId, Vec2i};

/// Terrain ids written into [`MapModel::terrain`].
///
/// Extension ids 7..14 are additive; existing scenarios keep loading because
/// [`MapModel::from_ini`] stores raw ids and old maps only emit 0..=6.
/// Passability mirrors the remastered games sensibly: water, bare rock,
/// swamps and dense forest floors block ground units; craters, mud and
/// scorched/ice/concrete ground are traversable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TerrainType {
    Grass = 0,
    Water = 1,
    Sand = 2,
    Dirt = 3,
    Rock = 4,
    Snow = 5,
    Rough = 6,
    Mud = 7,
    Crater = 8,
    Scorched = 9,
    Swamp = 10,
    Ice = 11,
    Concrete = 12,
    ForestFloor = 13,
    JungleFloor = 14,
}

impl TerrainType {
    pub const ALL: [TerrainType; 15] = [
        TerrainType::Grass,
        TerrainType::Water,
        TerrainType::Sand,
        TerrainType::Dirt,
        TerrainType::Rock,
        TerrainType::Snow,
        TerrainType::Rough,
        TerrainType::Mud,
        TerrainType::Crater,
        TerrainType::Scorched,
        TerrainType::Swamp,
        TerrainType::Ice,
        TerrainType::Concrete,
        TerrainType::ForestFloor,
        TerrainType::JungleFloor,
    ];

    pub const fn id(self) -> u8 {
        self as u8
    }

    pub const fn label(self) -> &'static str {
        match self {
            TerrainType::Grass => "Grass",
            TerrainType::Water => "Water",
            TerrainType::Sand => "Sand",
            TerrainType::Dirt => "Dirt",
            TerrainType::Rock => "Rock",
            TerrainType::Snow => "Snow",
            TerrainType::Rough => "Rough",
            TerrainType::Mud => "Mud",
            TerrainType::Crater => "Crater",
            TerrainType::Scorched => "Scorched",
            TerrainType::Swamp => "Swamp",
            TerrainType::Ice => "Ice",
            TerrainType::Concrete => "Concrete",
            TerrainType::ForestFloor => "Forest Floor",
            TerrainType::JungleFloor => "Jungle Floor",
        }
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::ALL.into_iter().find(|terrain| terrain.id() == id)
    }

    /// True when ground units can stand on it.
    ///
    /// Impassable: water, bare rock faces, swamps (mire) and forest/jungle
    /// floors (dense trees). Craters remain passable — they are depressions,
    /// not obstacles.
    pub const fn passable(self) -> bool {
        !matches!(
            self,
            TerrainType::Water
                | TerrainType::Rock
                | TerrainType::Swamp
                | TerrainType::ForestFloor
                | TerrainType::JungleFloor
        )
    }
}

/// Overlay ids written into [`MapModel::overlay`].
///
/// Road (4) and Wall (5) are additive extension kinds; the INI `[Overlay]`
/// section stores numeric ids, so old scenarios stay valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OverlayType {
    None = 0,
    Tiberium = 1,
    Trees = 2,
    Ore = 3,
    Road = 4,
    Wall = 5,
}

impl OverlayType {
    pub const ALL: [OverlayType; 6] = [
        OverlayType::None,
        OverlayType::Tiberium,
        OverlayType::Trees,
        OverlayType::Ore,
        OverlayType::Road,
        OverlayType::Wall,
    ];

    pub const fn id(self) -> u8 {
        self as u8
    }

    pub const fn label(self) -> &'static str {
        match self {
            OverlayType::None => "None",
            OverlayType::Tiberium => "Tiberium",
            OverlayType::Trees => "Trees",
            OverlayType::Ore => "Ore",
            OverlayType::Road => "Road",
            OverlayType::Wall => "Wall",
        }
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::ALL.into_iter().find(|overlay| overlay.id() == id)
    }
}

/// Knobs for [`generate`].
#[derive(Debug, Clone)]
pub struct MapGenSettings {
    pub width: u32,
    pub height: u32,
    pub seed: u64,
    /// Normalised sea level in 0..1 (0.34 yields roughly 30% water).
    pub water_level: f32,
    /// Base frequency of the elevation field.
    pub elevation_scale: f32,
    /// Base frequency of the moisture field.
    pub moisture_scale: f32,
    /// 0..1 chance weight for resource overlays.
    pub resource_density: f32,
    /// 0..1 chance weight for tree overlays.
    pub tree_density: f32,
    pub player_count: u8,
    pub theater: Theater,
    /// Pull elevation down near the map border so the playable area is an island.
    pub island: bool,
}

impl Default for MapGenSettings {
    fn default() -> Self {
        Self {
            width: 96,
            height: 96,
            seed: 1,
            water_level: 0.34,
            elevation_scale: 0.045,
            moisture_scale: 0.06,
            resource_density: 0.55,
            tree_density: 0.40,
            player_count: 2,
            theater: Theater::Temperate,
            island: true,
        }
    }
}

impl MapGenSettings {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            ..Self::default()
        }
    }
}

/// Generate a complete, validated scenario.
pub fn generate(settings: &MapGenSettings) -> MapModel {
    let width = settings.width.max(16);
    let height = settings.height.max(16);
    let mut model = MapModel::new(width, height);
    model.theater = settings.theater.name().to_string();

    let mut elevation = vec![0.0f32; (width * height) as usize];
    let mut moisture = vec![0.0f32; (width * height) as usize];
    let (mut min_e, mut max_e) = (f32::MAX, f32::MIN);

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let fx = x as f32 * settings.elevation_scale;
            let fy = y as f32 * settings.elevation_scale;
            let mut value = fbm(settings.seed ^ 0x9E37_79B9_7F4A_7C15, fx, fy, 5);
            if settings.island {
                value *= island_falloff(x, y, width, height);
            }
            elevation[index] = value;
            min_e = min_e.min(value);
            max_e = max_e.max(value);

            moisture[index] = fbm(
                settings.seed ^ 0xC2B2_AE3D_27D4_EB4F,
                x as f32 * settings.moisture_scale,
                y as f32 * settings.moisture_scale,
                3,
            );
        }
    }

    let range = (max_e - min_e).max(1.0e-5);
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let e = (elevation[index] - min_e) / range;
            let m = moisture[index].clamp(0.0, 1.0);
            let terrain = classify(e, m, settings);
            model.terrain[index] = terrain.id();
            // Persist a FLAT elevation field: the generated map has no
            // authored height, and any variation here is fbm noise the
            // renderer turns into cliff/ramp art. Earlier revisions persisted
            // either the raw continuous field or 4 quantised levels; both put
            // nearly every cell through the CLIFF_STEP threshold, so cliff
            // bands (dark rectangle fills + light borders) tiled into a hatch
            // lattice across the whole map. The continuous field above is
            // still what classifies terrain; the renderer now sees level
            // ground until elevation is authored.
            model.elevation[index] = 0;

            if terrain == TerrainType::Water {
                continue;
            }
            let forest = fbm(
                settings.seed ^ 0x1656_67B1_9E37_79F9,
                x as f32 * 0.08,
                y as f32 * 0.08,
                2,
            );
            if matches!(
                terrain,
                TerrainType::Grass | TerrainType::Dirt | TerrainType::Rough
            ) && m > 0.55
                && forest > 0.62 - settings.tree_density * 0.25
            {
                model.overlay[index] = OverlayType::Trees.id();
            }

            let ore = fbm(
                settings.seed ^ 0xD1B5_4A32_D192_ED03,
                x as f32 * 0.12,
                y as f32 * 0.12,
                2,
            );
            if matches!(
                terrain,
                TerrainType::Sand | TerrainType::Dirt | TerrainType::Rock | TerrainType::Rough
            ) && ore > 0.72 - settings.resource_density * 0.20
            {
                model.overlay[index] = OverlayType::Tiberium.id();
            }
        }
    }

    // Guarantee the generated map can host a game: if normalisation produced
    // too much water, lower the effective sea level and reclassify.
    let land_fraction = model
        .terrain
        .iter()
        .filter(|id| **id != TerrainType::Water.id())
        .count() as f32
        / (width * height) as f32;
    if land_fraction < 0.45 {
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize;
                let e = (elevation[index] - min_e) / range;
                if model.terrain[index] == TerrainType::Water.id() && e > 0.12 {
                    model.terrain[index] = TerrainType::Sand.id();
                }
            }
        }
    }

    place_player_starts(&mut model, settings);
    seed_starting_resources(&mut model, settings);
    model
}

/// Ensure every player start has a workable tiberium patch nearby, so a
/// generated map is playable immediately instead of being a pure sandbox.
fn seed_starting_resources(model: &mut MapModel, settings: &MapGenSettings) {
    for (player, start) in model.player_starts.clone().into_iter().enumerate() {
        for dy in -7..=7i32 {
            for dx in -7..=7i32 {
                let distance = (dx * dx + dy * dy) as f32;
                if !(9.0..=49.0).contains(&distance) {
                    continue;
                }
                let cell = Vec2i::new(start.x + dx, start.y + dy);
                let Some(index) = model.index(cell) else {
                    continue;
                };
                let Some(terrain) = TerrainType::from_id(model.terrain[index]) else {
                    continue;
                };
                if !terrain.passable() || model.overlay[index] == OverlayType::Trees.id() {
                    continue;
                }
                let roll = lattice(
                    settings.seed ^ 0x51ED_2701_0C7A_11B5,
                    i64::from(cell.x) + player as i64 * 131,
                    i64::from(cell.y),
                );
                if roll < 0.45 {
                    model.overlay[index] = OverlayType::Tiberium.id();
                }
            }
        }
    }
}

fn classify(e: f32, moisture: f32, settings: &MapGenSettings) -> TerrainType {
    if e < settings.water_level {
        return TerrainType::Water;
    }
    if e < settings.water_level + 0.035 {
        return TerrainType::Sand;
    }
    if e > 0.80 {
        return TerrainType::Rock;
    }
    match settings.theater {
        Theater::Snow => {
            if e > 0.62 {
                TerrainType::Rock
            } else {
                TerrainType::Snow
            }
        }
        Theater::Desert => {
            if moisture < 0.45 || e > 0.62 {
                TerrainType::Sand
            } else {
                TerrainType::Dirt
            }
        }
        Theater::Urban => {
            if moisture < 0.30 {
                TerrainType::Sand
            } else if e > 0.65 {
                TerrainType::Rock
            } else {
                TerrainType::Dirt
            }
        }
        Theater::Temperate => {
            if moisture < 0.28 {
                TerrainType::Sand
            } else if e > 0.68 {
                TerrainType::Rough
            } else if moisture > 0.70 {
                TerrainType::Grass
            } else {
                TerrainType::Dirt
            }
        }
    }
}

fn place_player_starts(model: &mut MapModel, settings: &MapGenSettings) {
    let players = settings.player_count.clamp(1, 8) as usize;
    let mut candidates: Vec<Vec2i> = Vec::new();
    // Keep starts away from the water border so every player has room to build.
    let margin_x = (model.width / 10).clamp(2, 20) as i32;
    let margin_y = (model.height / 10).clamp(2, 20) as i32;
    for y in margin_y..(model.height as i32 - margin_y).max(margin_y + 1) {
        for x in margin_x..(model.width as i32 - margin_x).max(margin_x + 1) {
            let cell = Vec2i::new(x, y);
            let Some(index) = model.index(cell) else {
                continue;
            };
            let Some(terrain) = TerrainType::from_id(model.terrain[index]) else {
                continue;
            };
            if terrain.passable() && model.overlay[index] != OverlayType::Tiberium.id() {
                candidates.push(cell);
            }
        }
    }
    if candidates.is_empty() {
        return;
    }

    // First start near the upper-left quadrant for a predictable camera, then
    // farthest-point sampling for the rest.
    let target = Vec2i::new((model.width / 4) as i32, (model.height / 4) as i32);
    let mut starts = Vec::with_capacity(players);
    let first = *candidates
        .iter()
        .min_by_key(|cell| squared_distance(**cell, target))
        .unwrap();
    starts.push(first);

    while starts.len() < players {
        let best = candidates
            .iter()
            .copied()
            .filter(|candidate| {
                starts
                    .iter()
                    .all(|start| squared_distance(*candidate, *start) > 64)
            })
            .max_by_key(|candidate| {
                starts
                    .iter()
                    .map(|start| squared_distance(*candidate, *start))
                    .min()
                    .unwrap_or(0)
            });
        match best {
            Some(cell) => starts.push(cell),
            None => break,
        }
    }

    let houses = match settings.theater {
        Theater::Temperate | Theater::Snow | Theater::Urban | Theater::Desert => {
            [ClassId::HouseGDI, ClassId::HouseNod]
        }
    };
    for (index, start) in starts.iter().enumerate() {
        model.player_starts.push(*start);
        // Clear a small build pad and place each player's MCV so the scenario
        // is immediately editable and playable.
        for dy in -1..=1i32 {
            for dx in -1..=1i32 {
                let cell = Vec2i::new(start.x + dx, start.y + dy);
                if let Some(cell_index) = model.index(cell) {
                    if model.terrain[cell_index] == TerrainType::Water.id() {
                        model.terrain[cell_index] = TerrainType::Sand.id();
                    }
                    model.overlay[cell_index] = OverlayType::None.id();
                }
            }
        }
        model.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE7,
            position: *start,
            owner: Some(houses[index % houses.len()]),
            ..Default::default()
        });
    }
}

fn squared_distance(a: Vec2i, b: Vec2i) -> i64 {
    let dx = i64::from(a.x - b.x);
    let dy = i64::from(a.y - b.y);
    dx * dx + dy * dy
}

fn island_falloff(x: u32, y: u32, width: u32, height: u32) -> f32 {
    let nx = x as f32 / width.max(1) as f32 * 2.0 - 1.0;
    let ny = y as f32 / height.max(1) as f32 * 2.0 - 1.0;
    // Squircle-ish falloff: keeps a broad interior and drowns the corners.
    let distance = (nx * nx + ny * ny).sqrt() / std::f32::consts::SQRT_2;
    (1.0 - distance.powi(3)).clamp(0.0, 1.0)
}

/// Smooth 2D value noise in the range 0..1.
pub fn value_noise(seed: u64, x: f32, y: f32) -> f32 {
    let x0 = x.floor();
    let y0 = y.floor();
    let tx = smoothstep(x - x0);
    let ty = smoothstep(y - y0);
    let x0 = x0 as i64;
    let y0 = y0 as i64;

    let a = lattice(seed, x0, y0);
    let b = lattice(seed, x0 + 1, y0);
    let c = lattice(seed, x0, y0 + 1);
    let d = lattice(seed, x0 + 1, y0 + 1);
    let top = lerp(a, b, tx);
    let bottom = lerp(c, d, tx);
    lerp(top, bottom, ty)
}

/// Fractal Brownian motion, 0..1.
pub fn fbm(seed: u64, x: f32, y: f32, octaves: u32) -> f32 {
    let mut amplitude = 0.5f32;
    let mut frequency = 1.0f32;
    let mut total = 0.0f32;
    let mut norm = 0.0f32;
    for octave in 0..octaves {
        total += value_noise(
            seed.wrapping_add(u64::from(octave) * 0x9E37_79B9),
            x * frequency,
            y * frequency,
        ) * amplitude;
        norm += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    (total / norm).clamp(0.0, 1.0)
}

fn lattice(seed: u64, x: i64, y: i64) -> f32 {
    let mut hash = seed
        ^ (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    hash ^= hash >> 30;
    hash = hash.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    hash ^= hash >> 27;
    hash = hash.wrapping_mul(0x94D0_49BB_1331_11EB);
    hash ^= hash >> 31;
    (hash & 0x00FF_FFFF) as f32 / 16_777_215.0
}

fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn land_fraction(model: &MapModel) -> f32 {
        model
            .terrain
            .iter()
            .filter(|id| **id != TerrainType::Water.id())
            .count() as f32
            / model.terrain.len() as f32
    }

    #[test]
    fn same_seed_produces_identical_maps() {
        let settings = MapGenSettings::new(42);
        let a = generate(&settings);
        let b = generate(&settings);
        assert_eq!(a.terrain, b.terrain);
        assert_eq!(a.overlay, b.overlay);
        assert_eq!(a.player_starts, b.player_starts);
        assert_eq!(a.objects.len(), b.objects.len());
    }

    #[test]
    fn different_seeds_produce_different_terrain() {
        let a = generate(&MapGenSettings::new(1));
        let b = generate(&MapGenSettings::new(2));
        assert_ne!(a.terrain, b.terrain);
    }

    #[test]
    fn generated_map_has_water_land_and_resources() {
        let model = generate(&MapGenSettings::new(7));
        let fraction = land_fraction(&model);
        assert!(
            (0.45..=0.95).contains(&fraction),
            "land fraction {fraction} outside expected range"
        );
        assert!(model
            .overlay
            .iter()
            .any(|id| *id == OverlayType::Tiberium.id()));
        assert!(model
            .terrain
            .iter()
            .any(|id| *id == TerrainType::Water.id()));
    }

    #[test]
    fn starts_are_on_land_distinct_and_spawn_mcvs() {
        let mut settings = MapGenSettings::new(11);
        settings.player_count = 4;
        let model = generate(&settings);
        assert_eq!(model.player_starts.len(), 4);
        for (index, start) in model.player_starts.iter().enumerate() {
            let cell = model.terrain_at(*start).unwrap();
            assert!(
                TerrainType::from_id(cell).is_some_and(|terrain| terrain.passable()),
                "start {index} is on water/rock"
            );
            assert!(!model
                .terrain_at(*start)
                .is_some_and(|id| id == TerrainType::Water.id()));
            for other in &model.player_starts[index + 1..] {
                assert_ne!(start, other);
            }
        }
        assert_eq!(model.objects.len(), 4);
        assert!(model.validate().is_empty(), "{:?}", model.validate());

        // Every start must have tiberium within a short drive.
        for start in &model.player_starts {
            let nearby = (-8..=8)
                .flat_map(|dy| (-8..=8).map(move |dx| (dx, dy)))
                .filter(|(dx, dy)| {
                    let cell = Vec2i::new(start.x + dx, start.y + dy);
                    model
                        .index(cell)
                        .is_some_and(|index| model.overlay[index] == OverlayType::Tiberium.id())
                })
                .count();
            assert!(
                nearby >= 5,
                "start {:?} has only {nearby} resource cells",
                start
            );
        }
    }

    #[test]
    fn noise_is_deterministic_and_bounded() {
        for (x, y) in [(0.0, 0.0), (1.25, 3.75), (-4.5, 9.25)] {
            let a = fbm(99, x, y, 4);
            assert_eq!(a, fbm(99, x, y, 4));
            assert!((0.0..=1.0).contains(&a));
        }
        assert_ne!(value_noise(1, 0.5, 0.5), value_noise(2, 0.5, 0.5));
    }

    #[test]
    fn theater_controls_snow_and_desert_classification() {
        let mut snow = MapGenSettings::new(5);
        snow.theater = Theater::Snow;
        let map = generate(&snow);
        assert!(map.terrain.iter().any(|id| *id == TerrainType::Snow.id()));

        let mut desert = MapGenSettings::new(5);
        desert.theater = Theater::Desert;
        let map = generate(&desert);
        assert!(map.terrain.iter().any(|id| *id == TerrainType::Sand.id()));
    }

    #[test]
    fn generated_map_roundtrips_through_ini() {
        let model = generate(&MapGenSettings::new(123));
        let text = model.to_ini();
        let loaded = MapModel::from_ini(&text).unwrap();
        assert_eq!(loaded.terrain, model.terrain);
        assert_eq!(loaded.overlay, model.overlay);
        assert_eq!(loaded.elevation, model.elevation);
        assert_eq!(loaded.objects.len(), model.objects.len());
    }

    #[test]
    fn elevation_field_is_persisted_and_deterministic() {
        let a = generate(&MapGenSettings::new(42));
        let b = generate(&MapGenSettings::new(42));
        assert_eq!(a.elevation.len(), (a.width * a.height) as usize);
        assert_eq!(
            a.elevation, b.elevation,
            "same seed must reproduce the elevation field"
        );
        // Generated maps have no authored height, so the field is flat: any
        // variation from the fbm would drive the renderer's CLIFF_STEP gate
        // and tile cliff bands across the whole map.
        assert!(
            a.elevation.iter().all(|level| *level == 0),
            "generated elevation stays flat until height is authored"
        );
        assert!(a.elevation.iter().all(|level| u16::from(*level) <= 255));
    }
}
