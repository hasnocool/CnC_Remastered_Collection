//! Deterministic terrain heightmaps and biome textures.
//!
//! Value-noise fBm over a seeded lattice, with integer lattice wrapping so
//! textures tile seamlessly. Colors are snapped to a supplied
//! [`Palette`](cnc_core::palette::Palette) via `find_closest`, so output
//! stays faithful to the game's 256-color palette.

use crate::math::{clamp01, mix, smoothstep};
use crate::prng::{hash_f64, Seed};
use cnc_core::color::RGB888;
use cnc_core::palette::Palette;

/// Band target colors (RGB) snapped through the palette.
pub const WATER_DEEP: RGB888 = RGB888 {
    r: 24,
    g: 40,
    b: 96,
};
pub const WATER_SHALLOW: RGB888 = RGB888 {
    r: 56,
    g: 88,
    b: 152,
};
pub const SAND: RGB888 = RGB888 {
    r: 200,
    g: 176,
    b: 120,
};
pub const GRASS_LOW: RGB888 = RGB888 {
    r: 72,
    g: 116,
    b: 64,
};
pub const GRASS_HIGH: RGB888 = RGB888 {
    r: 96,
    g: 136,
    b: 72,
};
pub const ROCK: RGB888 = RGB888 {
    r: 128,
    g: 128,
    b: 128,
};

/// Parameters for terrain generation. All values are plain data; the same
/// params always produce byte-identical output.
#[derive(Clone, Debug, PartialEq)]
pub struct TerrainParams {
    pub width: u32,
    pub height: u32,
    pub seed: Seed,
    /// Lattice cells of the first (base) octave across the whole texture.
    pub base_scale: f64,
    pub octaves: u32,
    pub persistence: f64,
    pub lacunarity: f64,
    /// Heights below this value are water.
    pub sea_level: f64,
    /// Slope above which surfaces render as rock regardless of height.
    pub rock_slope: f64,
}

impl Default for TerrainParams {
    fn default() -> Self {
        Self {
            width: 64,
            height: 64,
            seed: Seed::new(0x5EED_0001),
            base_scale: 6.0,
            octaves: 5,
            persistence: 0.5,
            lacunarity: 2.0,
            sea_level: 0.35,
            rock_slope: 0.22,
        }
    }
}

impl TerrainParams {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Self::default()
        }
    }

    pub fn with_seed(mut self, seed: Seed) -> Self {
        self.seed = seed;
        self
    }
}

/// One value-noise octave: bilinear interpolation of lattice hashes with
/// smoothstep easing, wrapping at `cells` for seamless tiling.
fn value_noise(nx: f64, ny: f64, cells: i32, seed: Seed) -> f64 {
    let cell_span = cells.max(1);
    let x0 = nx.floor();
    let y0 = ny.floor();
    let fx = nx - x0;
    let fy = ny - y0;
    let ix = x0 as i32;
    let iy = y0 as i32;
    let wrap = |v: i32| v.rem_euclid(cell_span);
    let corner = |dx: i32, dy: i32| hash_f64(wrap(ix + dx), wrap(iy + dy), seed);
    let u = smoothstep(fx);
    let v = smoothstep(fy);
    let top = mix(corner(0, 0), corner(1, 0), u);
    let bottom = mix(corner(0, 1), corner(1, 1), u);
    mix(top, bottom, v)
}

/// Fractal brownian motion at normalized texture coordinates (in cell units).
fn fbm(nx: f64, ny: f64, params: &TerrainParams) -> f64 {
    let mut total = 0.0_f64;
    let mut amplitude = 1.0_f64;
    let mut frequency = params.base_scale;
    let mut norm = 0.0_f64;
    for octave in 0..params.octaves {
        // Per-octave seed offset keeps octaves decorrelated.
        let octave_seed = Seed::new(params.seed.0.wrapping_add(octave as u64));
        total += amplitude
            * value_noise(
                nx * frequency,
                ny * frequency,
                cells_for(frequency),
                octave_seed,
            );
        norm += amplitude;
        amplitude *= params.persistence;
        frequency *= params.lacunarity;
    }
    if norm <= f64::EPSILON {
        0.0
    } else {
        total / norm
    }
}

/// Integer lattice wrap count per octave frequency.
fn cells_for(frequency: f64) -> i32 {
    frequency.round().max(1.0) as i32
}

/// Normalized heightmap (0.0..1.0), row-major, `width * height` entries.
pub fn generate_heightmap(params: &TerrainParams) -> Vec<f64> {
    assert!(
        params.width > 0 && params.height > 0,
        "non-zero texture size"
    );
    let mut out = Vec::with_capacity((params.width * params.height) as usize);
    for py in 0..params.height {
        for px in 0..params.width {
            // Sample in [-0.5, 0.5) cell units; the wrap in value_noise
            // handles edges, so tiles remain seamless.
            let nx = px as f64 / params.width as f64 - 0.5;
            let ny = py as f64 / params.height as f64 - 0.5;
            out.push(clamp01(fbm(nx, ny, params)));
        }
    }
    out
}

/// Height slope magnitude at a pixel (central differences, clamped edge).
fn slope_at(map: &[f64], px: u32, py: u32, params: &TerrainParams) -> f64 {
    let idx = |x: u32, y: u32| {
        map[(y.min(params.height - 1) * params.width + x.min(params.width - 1)) as usize]
    };
    let dx = idx(px + 1, py) - idx(px.saturating_sub(1), py);
    let dy = idx(px, py + 1) - idx(px, py.saturating_sub(1));
    (dx * dx + dy * dy).sqrt()
}

/// Terrain texture: water/sand/grass/rock bands snapped to the palette.
pub fn generate_terrain_texture(params: &TerrainParams, palette: &Palette) -> image::RgbaImage {
    let map = generate_heightmap(params);
    let mut image = image::RgbaImage::new(params.width, params.height);
    for py in 0..params.height {
        for px in 0..params.width {
            let h = map[(py * params.width + px) as usize];
            let slope = slope_at(&map, px, py, params);
            let target = if h < params.sea_level {
                // Deepen toward the lowest heights.
                let depth = h / params.sea_level.max(f64::EPSILON);
                if depth < 0.5 {
                    WATER_DEEP
                } else {
                    WATER_SHALLOW
                }
            } else if h < params.sea_level + 0.04 {
                SAND
            } else if slope > params.rock_slope || h > 0.86 {
                ROCK
            } else {
                // Grass darkens with height for readable relief.
                let t = clamp01((h - params.sea_level - 0.04) / 0.5);
                RGB888 {
                    r: (GRASS_LOW.r as f64 + (GRASS_HIGH.r as f64 - GRASS_LOW.r as f64) * t) as u8,
                    g: (GRASS_LOW.g as f64 + (GRASS_HIGH.g as f64 - GRASS_LOW.g as f64) * t) as u8,
                    b: (GRASS_LOW.b as f64 + (GRASS_HIGH.b as f64 - GRASS_LOW.b as f64) * t) as u8,
                }
            };
            let index = palette.find_closest(target);
            let rgb = &palette[u8::from(index) as usize];
            image.put_pixel(px, py, image::Rgba([rgb.r, rgb.g, rgb.b, 255]));
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heightmap_is_normalized_and_finite() {
        let params = TerrainParams::new(32, 32);
        let map = generate_heightmap(&params);
        assert_eq!(map.len(), 32 * 32);
        for h in &map {
            assert!(h.is_finite(), "NaN in heightmap");
            assert!((0.0..=1.0).contains(h));
        }
    }

    #[test]
    fn same_seed_is_bit_identical() {
        let params = TerrainParams::new(16, 16);
        let a = generate_heightmap(&params);
        let b = generate_heightmap(&params);
        assert_eq!(a, b, "determinism violated");
    }

    #[test]
    fn different_seed_differs() {
        let a = generate_heightmap(&TerrainParams::new(16, 16));
        let b = generate_heightmap(&TerrainParams::new(16, 16).with_seed(Seed::new(99)));
        assert_ne!(a, b, "seed had no effect");
    }

    #[test]
    fn water_band_exists_for_high_sea_level() {
        let params = TerrainParams::new(32, 32).with_seed(Seed::new(4));
        let map = generate_heightmap(&params);
        let water = map.iter().filter(|h| **h < params.sea_level).count();
        assert!(
            water > (map.len() / 10),
            "expected meaningful water fraction, got {water}/{}",
            map.len()
        );
    }

    #[test]
    fn tiles_wrap_seamlessly_edges_match_lattice() {
        // value_noise takes cell-unit coordinates: with a wrap span of 4,
        // x == +2 and x == -2 name the same lattice point (2 mod 4), so the
        // noise must agree there — this is the tiling seam.
        let seed = Seed::new(3);
        let a = value_noise(-2.0, -2.0, 4, seed);
        let b = value_noise(2.0, 2.0, 4, seed);
        assert!((a - b).abs() < 1e-9, "wrap seam detected: {a} vs {b}");
    }

    #[test]
    fn texture_matches_heightmap_dimensions_and_is_opaque() {
        let params = TerrainParams::new(16, 16);
        let palette = Palette::new();
        let image = generate_terrain_texture(&params, &palette);
        assert_eq!(image.dimensions(), (16, 16));
        for px in 0..16 {
            for py in 0..16 {
                let pixel = image.get_pixel(px, py);
                assert_eq!(pixel[3], 255, "opaque alpha expected");
            }
        }
    }
}
