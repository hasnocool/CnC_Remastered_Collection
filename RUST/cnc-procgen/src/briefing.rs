//! Static briefing-art generators: faction emblems and commander portraits.
//!
//! Same family as [`crate::sprites`] (placeholder unit sprites),
//! [`crate::terrain`] (heightmaps) and [`crate::texture`] (noise/Tiberium):
//! **deterministic, palette-snapped placeholder art** for the optional
//! generated-asset lane. The original Westwood briefing screens played VQA
//! movies (`REDALERT/SCENARIO.CPP` drives `Scen.BriefMovie`;
//! `REDALERT/GOPTIONS.CPP:451` calls `Play_Movie(Scen.BriefMovie)`), but those
//! decoders are licensed stubs that were never redistributed, so this port
//! shows **static** briefing art next to the existing text panels instead.
//! Nothing here produces or plays video, and none of these images are traced
//! copies of original Westwood art — they are parametric stand-ins in the
//! placeholder-sprite spirit of this crate.
//!
//! Every written pixel is snapped through a
//! [`Palette`](cnc_core::palette::Palette) (exactly like the terrain/texture
//! generators) and player-color accents flow through
//! [`PlayerRemap`](cnc_core::color::PlayerRemap) the way real sprites do.
//! Generation is a pure function of `(faction, mission index, seed)`: no file
//! I/O, no `HashMap` iteration, bit-identical bytes for identical inputs, so
//! art can be baked offline and cached.
//!
//! ```
//! use cnc_core::color::PlayerRemap;
//! use cnc_core::palette::Palette;
//! use cnc_procgen::Seed;
//! use cnc_procgen::briefing::{generate_briefing_emblem, BriefingFaction};
//!
//! let palette = Palette::grayscale();
//! let faction = BriefingFaction::from_ra_player(1); // Soviet
//! let remap = PlayerRemap::player_color(faction.player_slot());
//! let emblem = generate_briefing_emblem(faction, 0, Seed::new(7), &palette, &remap);
//! assert_eq!(emblem.dimensions(), (48, 48));
//! ```

use crate::prng::{hash_f64, Seed};
use cnc_core::color::{PlayerRemap, RGB888};
use cnc_core::palette::Palette;

/// Edge length (pixels) of generated briefing art (emblem *and* portrait).
///
/// Kept inside the 32–64px band of the crate's placeholder sprites so the
/// future renderer lane can blit it through the same atlas path as unit
/// sprites.
pub const BRIEFING_ART_SIZE: u32 = 48;

/// Warm accent base color snapped to a palette entry, then remapped by
/// [`PlayerRemap`]. Same value as the sprite lane's accent base so both
/// generators share one player-color source.
const ACCENT_BASE: RGB888 = RGB888 { r: 224, g: 8, b: 8 };

/// Emblem metal and trim colors.
const EMBLEM_STEEL: RGB888 = RGB888 {
    r: 104,
    g: 112,
    b: 120,
};
const EMBLEM_STEEL_DARK: RGB888 = RGB888 {
    r: 60,
    g: 66,
    b: 74,
};
const EMBLEM_GOLD: RGB888 = RGB888 {
    r: 216,
    g: 176,
    b: 72,
};
const PLAQUE_DARK: RGB888 = RGB888 {
    r: 40,
    g: 44,
    b: 48,
};

/// Portrait base colors (skin, uniform, gear).
const SKIN: RGB888 = RGB888 {
    r: 176,
    g: 136,
    b: 104,
};
const SKIN_SHADOW: RGB888 = RGB888 {
    r: 132,
    g: 100,
    b: 76,
};
const UNIFORM: RGB888 = RGB888 {
    r: 72,
    g: 80,
    b: 92,
};
const GEAR: RGB888 = RGB888 {
    r: 48,
    g: 52,
    b: 56,
};

/// Which side a briefing emblem/portrait belongs to.
///
/// The two campaigns spell their sides differently (`[Basic] Player=0` is GDI
/// in Tiberian Dawn, `[Map] Player=0` is the Allies in Red Alert — see
/// `Mission::player` / `Scenario::player`), so the constructors below keep
/// that mapping explicit instead of guessing from a bare index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BriefingFaction {
    /// Tiberian Dawn, player 0.
    Gdi,
    /// Tiberian Dawn, player 1.
    Nod,
    /// Red Alert, player 0.
    Allies,
    /// Red Alert, player 1.
    Soviet,
}

impl BriefingFaction {
    /// Map a Tiberian Dawn `[Basic] Player=` value (0 = GDI, 1 = Nod).
    pub fn from_td_player(player: u8) -> Self {
        if player == 0 {
            Self::Gdi
        } else {
            Self::Nod
        }
    }

    /// Map a Red Alert `[Map] Player=` value (0 = Allies, 1 = Soviet).
    pub fn from_ra_player(player: usize) -> Self {
        if player == 0 {
            Self::Allies
        } else {
            Self::Soviet
        }
    }

    /// Player slot handed to [`PlayerRemap::player_color`] on the briefing
    /// screen (matches the scenario/mission dialect: first side 0, second
    /// side 1).
    pub fn player_slot(self) -> u8 {
        match self {
            Self::Gdi | Self::Allies => 0,
            Self::Nod | Self::Soviet => 1,
        }
    }

    /// Display name used by the (text) briefing panels.
    pub fn label(self) -> &'static str {
        match self {
            Self::Gdi => "GDI",
            Self::Nod => "Nod",
            Self::Allies => "Allies",
            Self::Soviet => "Soviet",
        }
    }
}

/// Palette-snapped RGBA canvas with a player-accent slot.
///
/// Mirrors `sprites::SpriteCanvas`: `put` snaps a target color to the closest
/// palette entry and `put_accent` writes the remapped player-color entry, so
/// every emitted pixel is a palette entry and never a raw RGB constant.
struct BriefingCanvas {
    image: image::RgbaImage,
    accent: u8,
}

impl BriefingCanvas {
    fn new(palette: &Palette, remap: &PlayerRemap) -> Self {
        let base = u8::from(palette.find_closest(ACCENT_BASE));
        Self {
            image: image::RgbaImage::new(BRIEFING_ART_SIZE, BRIEFING_ART_SIZE),
            accent: remap.map[base as usize],
        }
    }

    /// The accent's own palette color, usable as a snap target: finding the
    /// closest entry to an existing entry returns that entry, so polygons
    /// filled with `accent_target` are byte-identical to `put_accent`.
    fn accent_target(&self, palette: &Palette) -> RGB888 {
        palette.colors[self.accent as usize]
    }

    fn put(&mut self, x: u32, y: u32, target: RGB888, palette: &Palette) {
        if x >= BRIEFING_ART_SIZE || y >= BRIEFING_ART_SIZE {
            return;
        }
        let rgb = &palette[u8::from(palette.find_closest(target))];
        self.image
            .put_pixel(x, y, image::Rgba([rgb.r, rgb.g, rgb.b, 255]));
    }

    fn put_accent(&mut self, x: u32, y: u32, palette: &Palette) {
        if x >= BRIEFING_ART_SIZE || y >= BRIEFING_ART_SIZE {
            return;
        }
        let rgb = &palette[self.accent as usize];
        self.image
            .put_pixel(x, y, image::Rgba([rgb.r, rgb.g, rgb.b, 255]));
    }
}

fn fill_rect(
    canvas: &mut BriefingCanvas,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    color: RGB888,
    palette: &Palette,
) {
    for py in y..y + h {
        for px in x..x + w {
            canvas.put(px, py, color, palette);
        }
    }
}

fn fill_disc(
    canvas: &mut BriefingCanvas,
    cx: f64,
    cy: f64,
    radius: f64,
    color: RGB888,
    palette: &Palette,
) {
    for py in 0..BRIEFING_ART_SIZE {
        for px in 0..BRIEFING_ART_SIZE {
            let dx = px as f64 - cx;
            let dy = py as f64 - cy;
            if dx * dx + dy * dy <= radius * radius {
                canvas.put(px, py, color, palette);
            }
        }
    }
}

/// Filled ellipse (heads, shoulders).
fn fill_ellipse(
    canvas: &mut BriefingCanvas,
    cx: f64,
    cy: f64,
    rx: f64,
    ry: f64,
    color: RGB888,
    palette: &Palette,
) {
    let x0 = (cx - rx).max(0.0).ceil() as u32;
    let x1 = (cx + rx).min(BRIEFING_ART_SIZE as f64 - 1.0).floor() as u32;
    let y0 = (cy - ry).max(0.0).ceil() as u32;
    let y1 = (cy + ry).min(BRIEFING_ART_SIZE as f64 - 1.0).floor() as u32;
    for py in y0..=y1 {
        for px in x0..=x1 {
            let nx = (px as f64 - cx) / rx;
            let ny = (py as f64 - cy) / ry;
            if nx * nx + ny * ny <= 1.0 {
                canvas.put(px, py, color, palette);
            }
        }
    }
}

/// Axis-free thick line as a quad polygon (chevrons, straps, brims).
fn thick_line(x0: f64, y0: f64, x1: f64, y1: f64, thickness: f64) -> Vec<(f64, f64)> {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-9 {
        return Vec::new();
    }
    let nx = -dy / len * thickness * 0.5;
    let ny = dx / len * thickness * 0.5;
    vec![
        (x0 + nx, y0 + ny),
        (x1 + nx, y1 + ny),
        (x1 - nx, y1 - ny),
        (x0 - nx, y0 - ny),
    ]
}

/// Even-odd scanline polygon fill.
///
/// Deterministic by construction: crossings are a pure function of the input
/// vertices and are ordered with a total order (`f64::total_cmp`), never
/// through hash iteration.
fn fill_polygon(
    canvas: &mut BriefingCanvas,
    points: &[(f64, f64)],
    color: RGB888,
    palette: &Palette,
) {
    if points.len() < 3 {
        return;
    }
    let limit = BRIEFING_ART_SIZE as f64 - 1.0;
    for py in 0..BRIEFING_ART_SIZE {
        let sy = py as f64 + 0.5;
        let mut crossings: Vec<f64> = Vec::with_capacity(points.len());
        for edge in 0..points.len() {
            let (x0, y0) = points[edge];
            let (x1, y1) = points[(edge + 1) % points.len()];
            if (y0 <= sy && y1 > sy) || (y1 <= sy && y0 > sy) {
                crossings.push(x0 + (sy - y0) / (y1 - y0) * (x1 - x0));
            }
        }
        crossings.sort_by(f64::total_cmp);
        for pair in crossings.chunks(2) {
            let (left, right) = match pair {
                [a, b] => (*a, *b),
                _ => continue,
            };
            let from = left.max(0.0).ceil() as u32;
            let to = right.min(limit).floor() as u32;
            for px in from..=to {
                canvas.put(px, py, color, palette);
            }
        }
    }
}

/// Five-point star vertices (0 = pointing up).
fn star_points(cx: f64, cy: f64, outer: f64, inner: f64, rotation: f64) -> Vec<(f64, f64)> {
    let mut points = Vec::with_capacity(10);
    for k in 0..10 {
        let radius = if k % 2 == 0 { outer } else { inner };
        let angle = rotation + (k as f64) * std::f64::consts::PI / 5.0;
        points.push((cx + radius * angle.sin(), cy - radius * angle.cos()));
    }
    points
}

/// Half-width of the shield silhouette at row `y`: an elliptical dome over an
/// elliptical taper, joined smoothly (both halves equal `widest` at `dome`
/// with zero slope there).
fn shield_half_width(y: u32) -> f64 {
    let size = BRIEFING_ART_SIZE as f64;
    let fy = y as f64;
    let dome = size * 0.18;
    let widest = size * 0.46;
    let (offset, span) = if fy <= dome {
        (fy - dome, dome)
    } else {
        (fy - dome, size - dome)
    };
    let t = (offset / span).clamp(-1.0, 1.0);
    widest * (1.0 - t * t).max(0.0).sqrt()
}

/// 3x5 bitmap digit glyphs (bit 2 = leftmost column).
const DIGIT_GLYPHS: [[u8; 5]; 10] = [
    [0b111, 0b101, 0b101, 0b101, 0b111],
    [0b010, 0b110, 0b010, 0b010, 0b111],
    [0b111, 0b001, 0b111, 0b100, 0b111],
    [0b111, 0b001, 0b111, 0b001, 0b111],
    [0b101, 0b101, 0b111, 0b001, 0b001],
    [0b111, 0b100, 0b111, 0b001, 0b111],
    [0b111, 0b100, 0b111, 0b101, 0b111],
    [0b111, 0b001, 0b010, 0b010, 0b010],
    [0b111, 0b101, 0b111, 0b101, 0b111],
    [0b111, 0b101, 0b111, 0b001, 0b111],
];

/// Draw the two-digit mission number ("01" for index 0) in a dark plaque.
fn draw_mission_plaque(canvas: &mut BriefingCanvas, mission_index: u32, palette: &Palette) {
    let value = mission_index % 100;
    let digits = [value / 10, value % 10];
    let px = BRIEFING_ART_SIZE / 2 - 4;
    let py = BRIEFING_ART_SIZE * 2 / 3 + 1;
    fill_rect(canvas, px, py, 8, 7, PLAQUE_DARK, palette);
    fill_rect(canvas, px, py, 8, 1, EMBLEM_GOLD, palette);
    for (slot, digit) in digits.iter().enumerate() {
        let glyph = &DIGIT_GLYPHS[*digit as usize];
        for (row, bits) in glyph.iter().enumerate() {
            for column in 0..3u32 {
                if bits & (1 << (2 - column)) != 0 {
                    let x = px + 2 + slot as u32 + column;
                    let y = py + 2 + row as u32;
                    canvas.put(x, y, EMBLEM_GOLD, palette);
                }
            }
        }
    }
}

/// Faction sigil painted in the shield centre (player-color accent).
fn draw_faction_glyph(canvas: &mut BriefingCanvas, faction: BriefingFaction, palette: &Palette) {
    let size = BRIEFING_ART_SIZE as f64;
    let cx = size / 2.0;
    let cy = size * 0.38;
    let accent = canvas.accent_target(palette);

    match faction {
        BriefingFaction::Gdi => {
            // Upward chevron pair.
            let mut chevron = thick_line(cx - 10.0, cy + 7.0, cx, cy - 5.0, 5.0);
            chevron.extend(thick_line(cx, cy - 5.0, cx + 10.0, cy + 7.0, 5.0));
            fill_polygon(canvas, &chevron, accent, palette);
        }
        BriefingFaction::Nod => {
            // Downward chevron pair with a centre pip.
            let mut chevron = thick_line(cx - 10.0, cy - 6.0, cx, cy + 6.0, 5.0);
            chevron.extend(thick_line(cx, cy + 6.0, cx + 10.0, cy - 6.0, 5.0));
            fill_polygon(canvas, &chevron, accent, palette);
            fill_disc(canvas, cx, cy, 3.0, EMBLEM_GOLD, palette);
        }
        BriefingFaction::Allies => {
            // Ringed star.
            for py in 0..BRIEFING_ART_SIZE {
                for px in 0..BRIEFING_ART_SIZE {
                    let dx = px as f64 - cx;
                    let dy = py as f64 - cy;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if (12.0..14.0).contains(&dist) {
                        canvas.put_accent(px, py, palette);
                    }
                }
            }
            fill_polygon(
                canvas,
                &star_points(cx, cy, 10.0, 4.2, 0.0),
                accent,
                palette,
            );
        }
        BriefingFaction::Soviet => {
            // Solid star.
            fill_polygon(
                canvas,
                &star_points(cx, cy, 11.0, 4.6, 0.0),
                accent,
                palette,
            );
        }
    }
}

/// Generate one deterministic faction emblem for the static briefing screen.
///
/// `mission_index` is drawn into the bottom plaque as a two-digit number, so
/// consecutive missions of a campaign get visually distinct art. Identical
/// inputs (faction, index, seed, palette, remap) always yield byte-identical
/// pixels.
pub fn generate_briefing_emblem(
    faction: BriefingFaction,
    mission_index: u32,
    seed: Seed,
    palette: &Palette,
    remap: &PlayerRemap,
) -> image::RgbaImage {
    let mut canvas = BriefingCanvas::new(palette, remap);
    let size = BRIEFING_ART_SIZE;
    let cx = size as f64 / 2.0;

    // Gold rim, then the steel shield body inset by 2px.
    for y in 0..size {
        let outer = shield_half_width(y);
        let inner = shield_half_width(y) - 2.0;
        for x in 0..size {
            let dx = (x as f64 - cx).abs();
            if dx <= outer {
                canvas.put(x, y, EMBLEM_GOLD, palette);
            }
            if dx <= inner {
                canvas.put(x, y, EMBLEM_STEEL, palette);
            }
        }
    }
    // Riveted inner band: deterministic per-seed dither at the shield edge.
    for y in 2..size {
        let inner = shield_half_width(y) - 2.0;
        for x in 0..size {
            let dx = (x as f64 - cx).abs();
            if inner - 4.0 <= dx && dx <= inner {
                let jitter = hash_f64(x as i32, y as i32, seed);
                let target = if jitter > 0.6 {
                    EMBLEM_STEEL_DARK
                } else {
                    EMBLEM_STEEL
                };
                canvas.put(x, y, target, palette);
            }
        }
    }

    draw_faction_glyph(&mut canvas, faction, palette);
    draw_mission_plaque(&mut canvas, mission_index, palette);
    canvas.image
}

/// Generate one deterministic commander portrait placeholder.
///
/// A bust silhouette (head, shoulders, faction headgear) with seeded shading
/// dither and a player-accent collar, snapped through the palette like every
/// other generator in this crate.
pub fn generate_briefing_portrait(
    faction: BriefingFaction,
    seed: Seed,
    palette: &Palette,
    remap: &PlayerRemap,
) -> image::RgbaImage {
    let mut canvas = BriefingCanvas::new(palette, remap);
    let size = BRIEFING_ART_SIZE as f64;
    let cx = size / 2.0;
    let accent = canvas.accent_target(palette);
    let inset = 1u32;
    let span = BRIEFING_ART_SIZE - 2 * inset;

    // Inset frame: the outer 1px ring stays transparent.
    fill_rect(&mut canvas, inset, inset, span, 1, PLAQUE_DARK, palette);
    fill_rect(
        &mut canvas,
        inset,
        BRIEFING_ART_SIZE - inset - 1,
        span,
        1,
        PLAQUE_DARK,
        palette,
    );
    fill_rect(&mut canvas, inset, inset, 1, span, PLAQUE_DARK, palette);
    fill_rect(
        &mut canvas,
        BRIEFING_ART_SIZE - inset - 1,
        inset,
        1,
        span,
        PLAQUE_DARK,
        palette,
    );

    // Torso + neck.
    fill_ellipse(
        &mut canvas,
        cx,
        size * 1.02,
        size * 0.42,
        size * 0.30,
        UNIFORM,
        palette,
    );
    fill_rect(
        &mut canvas,
        (cx - 4.0) as u32,
        (size * 0.52) as u32,
        8,
        8,
        SKIN,
        palette,
    );

    // Head with seeded shadow dither on the shaded side.
    fill_ellipse(&mut canvas, cx, size * 0.36, 9.5, 11.0, SKIN, palette);
    for py in 0..BRIEFING_ART_SIZE {
        for px in 0..BRIEFING_ART_SIZE {
            let nx = (px as f64 - cx) / 9.5;
            let ny = (py as f64 - size * 0.36) / 11.0;
            if nx * nx + ny * ny <= 1.0 {
                let shade = hash_f64(px as i32, py as i32, seed);
                let deep = nx > 0.25 && shade > 0.35;
                let target = if deep { SKIN_SHADOW } else { SKIN };
                canvas.put(px, py, target, palette);
            }
        }
    }

    // Faction headgear.
    match faction {
        BriefingFaction::Gdi => {
            fill_ellipse(&mut canvas, cx, size * 0.26, 10.2, 7.0, UNIFORM, palette);
            fill_rect(
                &mut canvas,
                (cx - 10.0) as u32,
                (size * 0.30) as u32,
                20,
                2,
                UNIFORM,
                palette,
            );
        }
        BriefingFaction::Nod => {
            let hood = star_points(cx, size * 0.24, 12.5, 5.0, 0.0);
            fill_ellipse(&mut canvas, cx, size * 0.26, 11.5, 10.0, GEAR, palette);
            fill_polygon(&mut canvas, &hood, GEAR, palette);
            let wrap = thick_line(cx - 7.0, size * 0.33, cx + 7.0, size * 0.27, 2.0);
            fill_polygon(&mut canvas, &wrap, EMBLEM_GOLD, palette);
        }
        BriefingFaction::Allies => {
            fill_ellipse(&mut canvas, cx, size * 0.26, 11.5, 9.0, UNIFORM, palette);
            fill_rect(
                &mut canvas,
                (cx - 12.0) as u32,
                (size * 0.30) as u32,
                24,
                2,
                GEAR,
                palette,
            );
        }
        BriefingFaction::Soviet => {
            fill_ellipse(&mut canvas, cx, size * 0.26, 12.5, 7.5, GEAR, palette);
            fill_rect(
                &mut canvas,
                (cx - 12.5) as u32,
                (size * 0.23) as u32,
                25,
                4,
                GEAR,
                palette,
            );
        }
    }

    // Player-color collar pip (accent flows through PlayerRemap like sprites).
    fill_rect(
        &mut canvas,
        (cx - 6.0) as u32,
        (size * 0.62) as u32,
        13,
        1,
        accent,
        palette,
    );
    canvas.image
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_palette() -> (Palette, PlayerRemap) {
        // Palette::new() is all-black; grayscale() is a real gradient so
        // find_closest snapping is meaningful.
        (Palette::grayscale(), PlayerRemap::player_color(0))
    }

    fn distinct_colors(image: &image::RgbaImage) -> Vec<[u8; 4]> {
        let mut seen: Vec<[u8; 4]> = Vec::new();
        for pixel in image.pixels() {
            if !seen.contains(&pixel.0) {
                seen.push(pixel.0);
            }
        }
        seen.sort_unstable();
        seen
    }

    fn filled(image: &image::RgbaImage) -> usize {
        image.pixels().filter(|p| p.0[3] > 0).count()
    }

    #[test]
    fn faction_constructors_map_player_indices() {
        assert_eq!(BriefingFaction::from_td_player(0), BriefingFaction::Gdi);
        assert_eq!(BriefingFaction::from_td_player(1), BriefingFaction::Nod);
        assert_eq!(BriefingFaction::from_ra_player(0), BriefingFaction::Allies);
        assert_eq!(BriefingFaction::from_ra_player(1), BriefingFaction::Soviet);
        assert_eq!(BriefingFaction::from_ra_player(7), BriefingFaction::Soviet);
        assert_eq!(BriefingFaction::Gdi.player_slot(), 0);
        assert_eq!(BriefingFaction::Nod.player_slot(), 1);
        assert_eq!(BriefingFaction::Allies.label(), "Allies");
    }

    #[test]
    fn briefing_art_is_sized_with_transparent_background() {
        let (palette, remap) = test_palette();
        for image in [
            generate_briefing_emblem(BriefingFaction::Gdi, 0, Seed::new(1), &palette, &remap),
            generate_briefing_portrait(BriefingFaction::Nod, Seed::new(1), &palette, &remap),
        ] {
            assert_eq!(image.dimensions(), (BRIEFING_ART_SIZE, BRIEFING_ART_SIZE));
            assert!(
                (32..=64).contains(&BRIEFING_ART_SIZE),
                "briefing art must stay in the placeholder-sprite size band"
            );
            assert_eq!(
                image.get_pixel(0, 0)[3],
                0,
                "background must be transparent"
            );
            assert!(
                filled(&image) > 200,
                "art too sparse: {} filled px",
                filled(&image)
            );
        }
    }

    #[test]
    fn briefing_art_is_bit_deterministic() {
        let (palette, remap) = test_palette();
        let emblem =
            generate_briefing_emblem(BriefingFaction::Soviet, 3, Seed::new(9), &palette, &remap);
        let emblem_again =
            generate_briefing_emblem(BriefingFaction::Soviet, 3, Seed::new(9), &palette, &remap);
        assert_eq!(emblem.into_raw(), emblem_again.into_raw());

        let portrait =
            generate_briefing_portrait(BriefingFaction::Gdi, Seed::new(9), &palette, &remap);
        let portrait_again =
            generate_briefing_portrait(BriefingFaction::Gdi, Seed::new(9), &palette, &remap);
        assert_eq!(portrait.into_raw(), portrait_again.into_raw());
    }

    #[test]
    fn briefing_art_is_palette_bounded() {
        // Palette snapping means every emitted color must be an existing
        // palette entry (the accent is a remapped *entry*, so it is included).
        let (palette, remap) = test_palette();
        let allowed: Vec<[u8; 3]> = palette
            .colors
            .iter()
            .map(|color| [color.r, color.g, color.b])
            .collect();
        for faction in [
            BriefingFaction::Gdi,
            BriefingFaction::Nod,
            BriefingFaction::Allies,
            BriefingFaction::Soviet,
        ] {
            for image in [
                generate_briefing_emblem(faction, 1, Seed::new(4), &palette, &remap),
                generate_briefing_portrait(faction, Seed::new(4), &palette, &remap),
            ] {
                for pixel in image.pixels() {
                    if pixel.0[3] == 0 {
                        continue;
                    }
                    let rgb = [pixel.0[0], pixel.0[1], pixel.0[2]];
                    assert!(
                        allowed.contains(&rgb),
                        "{faction:?} emitted off-palette color {rgb:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn factions_and_mission_index_change_the_art() {
        let (palette, remap) = test_palette();
        let variants: Vec<Vec<u8>> = [
            generate_briefing_emblem(BriefingFaction::Gdi, 0, Seed::new(2), &palette, &remap),
            generate_briefing_emblem(BriefingFaction::Nod, 0, Seed::new(2), &palette, &remap),
            generate_briefing_emblem(BriefingFaction::Allies, 0, Seed::new(2), &palette, &remap),
            generate_briefing_emblem(BriefingFaction::Soviet, 0, Seed::new(2), &palette, &remap),
        ]
        .into_iter()
        .map(|image| image.into_raw())
        .collect();
        for (index, variant) in variants.iter().enumerate() {
            for other in &variants[index + 1..] {
                assert_ne!(variant, other, "faction sigils must differ");
            }
        }

        // The mission-number plaque differentiates consecutive missions.
        let first =
            generate_briefing_emblem(BriefingFaction::Gdi, 0, Seed::new(2), &palette, &remap);
        let second =
            generate_briefing_emblem(BriefingFaction::Gdi, 1, Seed::new(2), &palette, &remap);
        assert_ne!(first.into_raw(), second.into_raw());
    }

    #[test]
    fn seed_changes_emblem_dither_and_portrait_shading() {
        let (palette, remap) = test_palette();
        let emblem =
            generate_briefing_emblem(BriefingFaction::Gdi, 0, Seed::new(11), &palette, &remap);
        let other =
            generate_briefing_emblem(BriefingFaction::Gdi, 0, Seed::new(12), &palette, &remap);
        assert_ne!(
            emblem.into_raw(),
            other.into_raw(),
            "rivet dither must vary with the seed"
        );

        let portrait =
            generate_briefing_portrait(BriefingFaction::Nod, Seed::new(11), &palette, &remap);
        let portrait_other =
            generate_briefing_portrait(BriefingFaction::Nod, Seed::new(12), &palette, &remap);
        assert_ne!(
            portrait.into_raw(),
            portrait_other.into_raw(),
            "shading dither must vary with the seed"
        );
    }

    #[test]
    fn portraits_carry_player_accent_collar() {
        let palette = Palette::grayscale();
        let neutral = PlayerRemap::identity();
        let remapped = PlayerRemap::player_color(3);
        let base = u8::from(palette.find_closest(ACCENT_BASE));

        let portrait =
            generate_briefing_portrait(BriefingFaction::Allies, Seed::new(5), &palette, &remapped);
        let remapped_index = remapped.map[base as usize];
        let remapped_rgb = {
            let color = palette.colors[remapped_index as usize];
            [color.r, color.g, color.b, 255u8]
        };
        assert!(
            portrait.pixels().any(|p| p.0 == remapped_rgb),
            "collar pip must use the remapped player-color entry"
        );

        // Identity remap keeps the base accent color.
        let plain =
            generate_briefing_portrait(BriefingFaction::Allies, Seed::new(5), &palette, &neutral);
        let base_rgb = {
            let color = palette.colors[base as usize];
            [color.r, color.g, color.b, 255u8]
        };
        assert!(plain.pixels().any(|p| p.0 == base_rgb));
    }

    #[test]
    fn every_faction_emblem_has_glyph_contrast() {
        // Guards the sigil/plaque drawing (a degenerate polygon helper would
        // leave a flat shield behind).
        let (palette, remap) = test_palette();
        for faction in [
            BriefingFaction::Gdi,
            BriefingFaction::Nod,
            BriefingFaction::Allies,
            BriefingFaction::Soviet,
        ] {
            let image = generate_briefing_emblem(faction, 2, Seed::new(6), &palette, &remap);
            let tones = distinct_colors(&image);
            assert!(
                tones.len() >= 4,
                "{faction:?} emblem too flat: {}",
                tones.len()
            );
        }
    }
}
