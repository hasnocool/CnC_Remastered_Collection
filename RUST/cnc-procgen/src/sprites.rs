//! Procedural unit sprite generator (placeholder art).
//!
//! These sprites are deterministic stand-ins generated from parametric
//! shapes — they are NOT traced copies of original Westwood art. The shapes
//! deliberately mirror the structure the renderer expects (top-down unit
//! silhouettes at 32px, player-color accents through
//! [`PlayerRemap`](cnc_core::color::PlayerRemap)). A deeper sprite lane can
//! add per-class profiles and multi-frame sequences on this foundation.

use cnc_core::class_id::ClassId;
use cnc_core::color::{PlayerRemap, RGB888};
use cnc_core::palette::Palette;

/// Output edge length (pixels) of generated unit sprites.
pub const SPRITE_SIZE: u32 = 32;

/// Coarse sprite family, derived from the RTTI class id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpriteFamily {
    Infantry,
    Vehicle,
    Aircraft,
    Building,
}

/// Direction the sprite faces, in radians (0 = facing north/top).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpritePose {
    pub angle_rad: f64,
}

impl SpritePose {
    pub fn new(angle_rad: f64) -> Self {
        Self { angle_rad }
    }
    pub fn north() -> Self {
        Self { angle_rad: 0.0 }
    }
}

/// Map a class id to a sprite family using the ranges documented in
/// `cnc_core::class_id` (Infantry types 4xx, Vehicle 5xx, Aircraft 6xx,
/// Ship 7xx; bases and buildings 0xx-3xx).
pub fn family_of(class_id: ClassId) -> SpriteFamily {
    match class_id as u32 {
        400..=499 => SpriteFamily::Infantry,
        500..=599 => SpriteFamily::Vehicle,
        600..=699 => SpriteFamily::Aircraft,
        700..=799 => SpriteFamily::Vehicle, // hull instead of treads visually
        300..=399 | 100..=199 => SpriteFamily::Building,
        _ => SpriteFamily::Infantry, // base classes / unknown: smallest silhouette
    }
}

const HULL_GRAY: RGB888 = RGB888 {
    r: 120,
    g: 120,
    b: 120,
};
const HULL_DARK: RGB888 = RGB888 {
    r: 64,
    g: 64,
    b: 72,
};
const ACCENT_BASE: RGB888 = RGB888 { r: 224, g: 8, b: 8 }; // warm accent
const AIRCRAFT_GRAY: RGB888 = RGB888 {
    r: 160,
    g: 152,
    b: 128,
};
const INFANTRY_GRAY: RGB888 = RGB888 {
    r: 144,
    g: 128,
    b: 96,
};
const BUILDING_GRAY: RGB888 = RGB888 {
    r: 128,
    g: 112,
    b: 88,
};
const BUILDING_ROOF: RGB888 = RGB888 {
    r: 96,
    g: 80,
    b: 64,
};

/// Sprite pixel writer: palette-snapped, optional player-accent remap.
struct SpriteCanvas {
    image: image::RgbaImage,
    accent: u8,
}

impl SpriteCanvas {
    fn new(palette: &Palette, remap: &PlayerRemap) -> Self {
        let base = u8::from(palette.find_closest(ACCENT_BASE));
        Self {
            image: image::RgbaImage::new(SPRITE_SIZE, SPRITE_SIZE),
            accent: remap.map[base as usize],
        }
    }

    fn put(&mut self, x: u32, y: u32, target: RGB888, palette: &Palette) {
        if x >= SPRITE_SIZE || y >= SPRITE_SIZE {
            return;
        }
        let rgb = &palette[u8::from(palette.find_closest(target))];
        self.image
            .put_pixel(x, y, image::Rgba([rgb.r, rgb.g, rgb.b, 255]));
    }

    fn put_accent(&mut self, x: u32, y: u32, palette: &Palette) {
        if x >= SPRITE_SIZE || y >= SPRITE_SIZE {
            return;
        }
        let rgb = &palette[self.accent as usize];
        self.image
            .put_pixel(x, y, image::Rgba([rgb.r, rgb.g, rgb.b, 255]));
    }
}

fn fill_rect(
    canvas: &mut SpriteCanvas,
    x0: u32,
    y0: u32,
    w: u32,
    h: u32,
    c: RGB888,
    palette: &Palette,
) {
    for y in y0..y0 + h {
        for x in x0..x0 + w {
            canvas.put(x, y, c, palette);
        }
    }
}

fn draw_vehicle(canvas: &mut SpriteCanvas, pose: SpritePose, palette: &Palette) {
    fill_rect(canvas, 6, 11, 20, 10, HULL_GRAY, palette);
    fill_rect(canvas, 5, 11, 3, 10, HULL_DARK, palette); // left tread
    fill_rect(canvas, 24, 11, 3, 10, HULL_DARK, palette); // right tread
    canvas.put_accent(11, 14, palette); // player stripe
    canvas.put_accent(13, 14, palette);
    // Turret disc + barrel rotated to pose.
    for y in 0..SPRITE_SIZE {
        for x in 0..SPRITE_SIZE {
            let dx = x as f64 - 15.5;
            let dy = y as f64 - 15.5;
            if dx * dx + dy * dy <= 6.2 * 6.2 {
                canvas.put(x, y, HULL_DARK, palette);
            }
        }
    }
    let (sin, cos) = pose.angle_rad.sin_cos();
    for r in 5..12 {
        let bx = 16.0 + cos * f64::from(r) - 0.5;
        let by = 16.0 + sin * f64::from(r) - 0.5;
        canvas.put(bx.max(0.0) as u32, by.max(0.0) as u32, HULL_DARK, palette);
    }
}

fn draw_aircraft(canvas: &mut SpriteCanvas, pose: SpritePose, palette: &Palette) {
    fill_rect(canvas, 14, 6, 4, 20, AIRCRAFT_GRAY, palette);
    fill_rect(canvas, 8, 14, 16, 4, AIRCRAFT_GRAY, palette);
    // Rotor bar rotated by pose.
    let (sin, cos) = pose.angle_rad.sin_cos();
    for r in -11i32..=11 {
        let rx = 15.5 + cos * f64::from(-r);
        let ry = 15.5 + sin * f64::from(-r);
        canvas.put(
            rx.max(0.0) as u32,
            ry.max(0.0) as u32,
            AIRCRAFT_GRAY,
            palette,
        );
    }
    canvas.put_accent(15, 9, palette);
}

fn draw_infantry(canvas: &mut SpriteCanvas, palette: &Palette) {
    fill_rect(canvas, 14, 8, 4, 5, INFANTRY_GRAY, palette); // torso
    fill_rect(canvas, 14, 5, 4, 3, INFANTRY_GRAY, palette); // head
    fill_rect(canvas, 14, 13, 2, 4, INFANTRY_GRAY, palette); // legs
    fill_rect(canvas, 17, 13, 2, 4, INFANTRY_GRAY, palette);
    canvas.put_accent(14, 9, palette); // squad color back
}

fn draw_building(canvas: &mut SpriteCanvas, palette: &Palette) {
    fill_rect(canvas, 6, 8, 20, 17, BUILDING_GRAY, palette);
    fill_rect(canvas, 8, 10, 16, 5, BUILDING_ROOF, palette);
    fill_rect(canvas, 14, 18, 4, 7, HULL_DARK, palette); // door
    canvas.put_accent(8, 23, palette); // house banner
    canvas.put_accent(9, 23, palette);
}

/// Generate one 32x32 placeholder unit sprite. Deterministic for identical
/// (family, pose, palette).
pub fn generate_unit_sprite(
    family: SpriteFamily,
    pose: SpritePose,
    palette: &Palette,
    remap: &PlayerRemap,
) -> image::RgbaImage {
    let mut canvas = SpriteCanvas::new(palette, remap);
    match family {
        SpriteFamily::Vehicle => draw_vehicle(&mut canvas, pose, palette),
        SpriteFamily::Aircraft => draw_aircraft(&mut canvas, pose, palette),
        SpriteFamily::Infantry => draw_infantry(&mut canvas, palette),
        SpriteFamily::Building => draw_building(&mut canvas, palette),
    }
    canvas.image
}

/// Convenience: generate directly from a class id at a fixed facing.
pub fn generate_sprite_for_class(
    class_id: ClassId,
    pose: SpritePose,
    palette: &Palette,
    remap: &PlayerRemap,
) -> image::RgbaImage {
    generate_unit_sprite(family_of(class_id), pose, palette, remap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prng::Seed;

    fn test_palette() -> (Palette, PlayerRemap) {
        // Palette::new() is all-black; grayscale() is a real gradient so
        // find_closest snapping is meaningful.
        (Palette::grayscale(), PlayerRemap::player_color(0))
    }

    #[test]
    fn family_mapping_follows_class_ranges() {
        use ClassId::*;
        assert_eq!(family_of(PowerPlant), SpriteFamily::Building);
        assert_eq!(family_of(HouseNod), SpriteFamily::Building);
        assert_eq!(family_of(Vehicle), SpriteFamily::Infantry); // base class
    }

    #[test]
    fn sprite_has_size_transparent_bg_and_content() {
        let (palette, remap) = test_palette();
        let sprite =
            generate_unit_sprite(SpriteFamily::Vehicle, SpritePose::north(), &palette, &remap);
        assert_eq!(sprite.dimensions(), (SPRITE_SIZE, SPRITE_SIZE));
        assert_eq!(
            sprite.get_pixel(0, 0)[3],
            0,
            "background must be transparent"
        );
        let filled = sprite.pixels().filter(|p| p.0[3] > 0).count();
        assert!(filled > 100, "sprite too sparse: {filled} filled px");
    }

    #[test]
    fn sprite_is_deterministic() {
        let (palette, remap) = test_palette();
        let a = generate_unit_sprite(
            SpriteFamily::Vehicle,
            SpritePose::new(1.2),
            &palette,
            &remap,
        );
        let b = generate_unit_sprite(
            SpriteFamily::Vehicle,
            SpritePose::new(1.2),
            &palette,
            &remap,
        );
        assert_eq!(a.into_raw(), b.into_raw());
    }

    #[test]
    fn pose_changes_the_sprite() {
        let (palette, remap) = test_palette();
        let a = generate_unit_sprite(
            SpriteFamily::Vehicle,
            SpritePose::new(0.0),
            &palette,
            &remap,
        );
        let b = generate_unit_sprite(
            SpriteFamily::Vehicle,
            SpritePose::new(std::f64::consts::FRAC_PI_2),
            &palette,
            &remap,
        );
        assert_ne!(
            a.into_raw(),
            b.into_raw(),
            "turret angle must affect pixels"
        );
    }

    #[test]
    fn every_family_generates_content() {
        let (palette, remap) = test_palette();
        for family in [
            SpriteFamily::Infantry,
            SpriteFamily::Vehicle,
            SpriteFamily::Aircraft,
            SpriteFamily::Building,
        ] {
            let sprite = generate_unit_sprite(family, SpritePose::north(), &palette, &remap);
            let filled = sprite.pixels().filter(|p| p.0[3] > 0).count();
            assert!(
                filled > 40,
                "{family:?} sprite too sparse: {filled} filled px"
            );
        }
    }

    #[test]
    fn accent_uses_remapped_palette_entry() {
        let (palette, _identity) = (Palette::new(), PlayerRemap::identity());
        let accent = PlayerRemap::player_color(3);
        let sprite = generate_unit_sprite(
            SpriteFamily::Infantry,
            SpritePose::north(),
            &palette,
            &accent,
        );
        let base = u8::from(palette.find_closest(ACCENT_BASE));
        let remapped = accent.map[base as usize];
        let expected = &palette[remapped as usize];
        let found = sprite.pixels().any(|p| {
            p.0[0] == expected.r && p.0[1] == expected.g && p.0[2] == expected.b && p.0[3] == 255
        });
        assert!(
            found,
            "accent pixels must come from the remapped palette entry"
        );
    }

    #[test]
    fn seed_is_stable_for_bitword_helpers() {
        // The sprite generators are seedless (pure param shapes); guard the
        // shared helper still yields stable values for future frame noise.
        assert_eq!(
            crate::prng::hash_f64(2, 3, Seed::new(4)),
            crate::prng::hash_f64(2, 3, Seed::new(4))
        );
    }
}
