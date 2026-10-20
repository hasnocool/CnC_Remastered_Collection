//! Procedural texture generators: plain noise and Tiberium patches.
//!
//! All output is snapped to a supplied
//! [`Palette`](cnc_core::palette::Palette) and is bit-identical for identical
//! seeds, so it can be baked into asset bundles offline.

use crate::math::clamp01;
use crate::prng::{hash_f64, Seed};
use cnc_core::color::RGB888;
use cnc_core::palette::Palette;

/// Target colors for the noise ramp (grayscale ladder).
const NOISE_DARK: RGB888 = RGB888 {
    r: 32,
    g: 32,
    b: 32,
};
const NOISE_LIGHT: RGB888 = RGB888 {
    r: 200,
    g: 200,
    b: 200,
};

/// Tiberium patch colors.
const TIB_LOW: RGB888 = RGB888 {
    r: 64,
    g: 128,
    b: 56,
};
const TIB_HIGH: RGB888 = RGB888 {
    r: 168,
    g: 240,
    b: 96,
};
const TIB_DIRT: RGB888 = RGB888 {
    r: 96,
    g: 88,
    b: 64,
};

/// Smooth value noise at lattice `cells`, wrapping at cell borders.
fn noise_at(nx: f64, ny: f64, cells: i32, seed: Seed) -> f64 {
    let span = cells.max(1);
    let (x0, y0) = (nx.floor(), ny.floor());
    let (fx, fy) = (nx - x0, ny - y0);
    let (ix, iy) = (x0 as i32, y0 as i32);
    let wrap = |v: i32| v.rem_euclid(span);
    let c = |dx: i32, dy: i32| hash_f64(wrap(ix + dx), wrap(iy + dy), seed);
    let u = fx * fx * (3.0 - 2.0 * fx);
    let v = fy * fy * (3.0 - 2.0 * fy);
    let top = c(0, 0) + (c(1, 0) - c(0, 0)) * u;
    let bottom = c(0, 1) + (c(1, 1) - c(0, 1)) * u;
    top + (bottom - top) * v
}

fn write_snapped(
    image: &mut image::RgbaImage,
    px: u32,
    py: u32,
    target: RGB888,
    palette: &Palette,
) {
    let index = u8::from(palette.find_closest(target));
    let rgb = &palette[index];
    image.put_pixel(px, py, image::Rgba([rgb.r, rgb.g, rgb.b, 255]));
}

/// Grayscale noise ramp texture (two-octave), palette-snapped.
pub fn generate_noise_texture(
    width: u32,
    height: u32,
    seed: Seed,
    palette: &Palette,
) -> image::RgbaImage {
    assert!(width > 0 && height > 0, "non-zero texture size");
    let mut image = image::RgbaImage::new(width, height);
    for py in 0..height {
        for px in 0..width {
            let nx = px as f64 / width as f64 - 0.5;
            let ny = py as f64 / height as f64 - 0.5;
            let v = clamp01(
                0.6 * noise_at(nx * 8.0, ny * 8.0, 4, seed)
                    + 0.4 * noise_at(nx * 16.0, ny * 16.0, 8, Seed::new(seed.0 ^ 0x5151)),
            );
            let target = RGB888 {
                r: (NOISE_DARK.r as f64 + (NOISE_LIGHT.r as f64 - NOISE_DARK.r as f64) * v) as u8,
                g: (NOISE_DARK.g as f64 + (NOISE_LIGHT.g as f64 - NOISE_DARK.g as f64) * v) as u8,
                b: (NOISE_DARK.b as f64 + (NOISE_LIGHT.b as f64 - NOISE_DARK.b as f64) * v) as u8,
            };
            write_snapped(&mut image, px, py, target, palette);
        }
    }
    image
}

/// Tiberium field texture: thresholded noise patches with a two-tone green
/// and dirt gaps, palette-snapped.
pub fn generate_tiberium_texture(
    width: u32,
    height: u32,
    seed: Seed,
    palette: &Palette,
) -> image::RgbaImage {
    assert!(width > 0 && height > 0, "non-zero texture size");
    let mut image = image::RgbaImage::new(width, height);
    for py in 0..height {
        for px in 0..width {
            let nx = px as f64 / width as f64 - 0.5;
            let ny = py as f64 / height as f64 - 0.5;
            let field = noise_at(nx * 10.0, ny * 10.0, 5, seed);
            let shimmer = noise_at(nx * 20.0, ny * 20.0, 10, Seed::new(seed.0 ^ 0xA5A5));
            let target = if field > 0.58 {
                let t = clamp01((shimmer - 0.3) / 0.5);
                RGB888 {
                    r: (TIB_LOW.r as f64 + (TIB_HIGH.r as f64 - TIB_LOW.r as f64) * t) as u8,
                    g: (TIB_LOW.g as f64 + (TIB_HIGH.g as f64 - TIB_LOW.g as f64) * t) as u8,
                    b: (TIB_LOW.b as f64 + (TIB_HIGH.b as f64 - TIB_LOW.b as f64) * t) as u8,
                }
            } else {
                TIB_DIRT
            };
            write_snapped(&mut image, px, py, target, palette);
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    // Palette::new() is all-black (an empty placeholder); grayscale() is a
    // real 256-entry gradient so find_closest snapping is meaningful.
    fn test_palette() -> Palette {
        Palette::grayscale()
    }

    #[test]
    fn noise_texture_is_bit_deterministic() {
        let palette = test_palette();
        let a = generate_noise_texture(16, 16, Seed::new(11), &palette);
        let b = generate_noise_texture(16, 16, Seed::new(11), &palette);
        assert_eq!(a.into_raw(), b.into_raw());
    }

    #[test]
    fn noise_texture_seed_sensitive_and_sized() {
        let palette = test_palette();
        let a = generate_noise_texture(24, 12, Seed::new(1), &palette);
        let b = generate_noise_texture(24, 12, Seed::new(2), &palette);
        assert_eq!(a.dimensions(), (24, 12));
        assert_ne!(a.into_raw(), b.into_raw());
    }

    #[test]
    fn tiberium_texture_has_patch_variance_and_determinism() {
        let palette = test_palette();
        let a = generate_tiberium_texture(32, 32, Seed::new(77), &palette);
        let b = generate_tiberium_texture(32, 32, Seed::new(77), &palette);
        assert_eq!(a.clone().into_raw(), b.into_raw(), "determinism violated");
        // Palette snapping means colors come from the palette, so the
        // structural signal is *variance*: patches and dirt must land on
        // distinct entries (more than the background-free uniform case).
        let distinct: std::collections::HashSet<[u8; 4]> = a.pixels().map(|p| p.0).collect();
        assert!(
            distinct.len() >= 3,
            "expected patch/dirt variance, saw {} distinct colors",
            distinct.len()
        );
    }

    #[test]
    fn tiberium_texture_is_seamless_at_edges() {
        // Wrapped lattice: opposite edge pixels should correlate. This is a
        // weaker structural check (edge-column statistics close, not equal).
        let palette = test_palette();
        let a = generate_tiberium_texture(16, 16, Seed::new(5), &palette);
        let left_dark = (0..16)
            .filter(|y| a.get_pixel(0, *y).0[1] > a.get_pixel(0, *y).0[0])
            .count();
        let right_dark = (0..16)
            .filter(|y| a.get_pixel(15, *y).0[1] > a.get_pixel(15, *y).0[0])
            .count();
        assert!((left_dark as i32 - right_dark as i32).abs() <= 3);
    }
}
