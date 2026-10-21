//! Minimal built-in 5×7 bitmap font for preview-sheet labels only, plus a
//! TrueType drawing layer for the asset-expansion scripts.
//!
//! The Python generator draws contact-sheet captions with Pillow's default
//! bitmap font and card/sheet headers with Pillow's default anchor "la" text
//! from a TTF; this module provides both roles. Glyphs in the built-in font
//! are 5 pixels wide, 7 tall, with a 1-pixel gap (6-pixel advance). Lowercase
//! input is uppercased; unsupported glyphs render blank.

use super::canvas::Canvas;
use super::Rgba;

const GLYPH_W: i32 = 5;
const GLYPH_H: i32 = 7;
const ADVANCE: i32 = GLYPH_W + 1;

/// Render `text` at `(x, y)` (top-left of the first glyph) in `color`.
pub fn draw_text(canvas: &mut Canvas, x: i32, y: i32, text: &str, color: Rgba) {
    let mut cx = x;
    for ch in text.chars() {
        if let Some(rows) = glyph(ch) {
            for (row, bits) in rows.iter().enumerate() {
                for col in 0..GLYPH_W {
                    if bits & (1 << (GLYPH_W - 1 - col)) != 0 {
                        canvas.set_pixel(cx + col, y + row as i32, color);
                    }
                }
            }
        }
        cx += ADVANCE;
    }
}

/// Width in pixels of `text` when drawn with [`draw_text`].
pub fn text_width(text: &str) -> i32 {
    text.chars().count() as i32 * ADVANCE - 1
}

const TTF_PATHS_REGULAR: &[&str] = &[
    // The Python generator's exact search order (asset_expansion_v6_2.py
    // FONT_PATHS), first hit wins.
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
    // Distro-specific locations appended after the generator's own list so
    // the feature also works where fonts live elsewhere (e.g. Arch).
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
    "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
];

const TTF_PATHS_BOLD: &[&str] = &[
    "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf",
    "/usr/share/fonts/TTF/DejaVuSans-Bold.ttf",
    "/usr/share/fonts/liberation/LiberationSans-Bold.ttf",
];

/// A TrueType font loaded from the same font files the Python generator's
/// `font()` helper uses (DejaVu Sans, then Liberation Sans; bold or
/// regular). `None` mirrors the Python `ImageFont.load_default()` fallback
/// condition: no candidate font file exists on this host.
#[derive(Debug)]
pub struct TtfFont {
    font: fontdue::Font,
}

impl TtfFont {
    /// Try to load a TTF from the generator's font paths, bold or regular.
    pub fn load(bold: bool) -> Option<Self> {
        let paths = if bold {
            TTF_PATHS_BOLD
        } else {
            TTF_PATHS_REGULAR
        };
        for path in paths {
            if let Ok(data) = std::fs::read(path) {
                if let Ok(font) = fontdue::Font::from_bytes(data, fontdue::FontSettings::default())
                {
                    return Some(Self { font });
                }
            }
        }
        None
    }
}

/// Draw `text` at `(x, y)` (top-left anchor, like PIL's default "la") in
/// `size` px line height with per-glyph coverage blended source-over onto
/// the canvas. No kerning or shaping, mirroring the generator's plain
/// per-character Pillow draws; the baseline comes from the font's own
/// horizontal line metrics. Writes outside the canvas clip silently.
pub fn draw_text_ttf(
    canvas: &mut Canvas,
    font: &TtfFont,
    x: i32,
    y: i32,
    text: &str,
    size: f32,
    color: Rgba,
) {
    let ascent = font
        .font
        .horizontal_line_metrics(size)
        .map(|m| m.ascent)
        .unwrap_or(size * 0.8);
    let baseline = (y as f32 + ascent).round() as i32;
    let mut pen_x = x as f32;
    for ch in text.chars() {
        let (metrics, bitmap) = font.font.rasterize(ch, size);
        if metrics.width > 0 && metrics.height > 0 {
            // fontdue bitmap row 0 is the top row; `ymin` is the bottom edge
            // offset above/below the baseline in y-up units.
            let left = pen_x.round() as i32 + metrics.xmin;
            let top = baseline - metrics.ymin - metrics.height as i32;
            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let coverage = bitmap[row * metrics.width + col] as u32;
                    if coverage == 0 {
                        continue;
                    }
                    let sa = coverage;
                    let (tx, ty) = (left + col as i32, top + row as i32);
                    let dc = canvas.pixel(tx, ty).unwrap_or([0, 0, 0, 0]);
                    let da = dc[3] as u32;
                    let out_a = sa + da * (255 - sa) / 255;
                    let mut out = [0u8; 4];
                    for i in 0..3 {
                        let top_rgb = color[i] as u32 * sa + dc[i] as u32 * da * (255 - sa) / 255;
                        out[i] = top_rgb.checked_div(out_a).unwrap_or(0) as u8;
                    }
                    out[3] = out_a as u8;
                    canvas.set_pixel(tx, ty, out);
                }
            }
        }
        pen_x += metrics.advance_width;
    }
}

/// 7 row bitmaps (bit 4 = leftmost column) for the label charset.
fn glyph(ch: char) -> Option<[u8; GLYPH_H as usize]> {
    let rows = match ch.to_ascii_uppercase() {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100,
        ],
        '/' => [
            0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000,
        ],
        '_' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b01100,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '(' => [
            0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010,
        ],
        ')' => [
            0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000,
        ],
        ':' => [
            0b00000, 0b01100, 0b01100, 0b00000, 0b01100, 0b01100, 0b00000,
        ],
        ' ' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
        // Pillow renders these in the shipped pack; the 5x7 fallback must not leave them blank.
        '%' => [
            0b11001, 0b11010, 0b00100, 0b01000, 0b10100, 0b01011, 0b10011,
        ],
        ',' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b01100, 0b00100,
        ],
        '{' => [
            0b00010, 0b00100, 0b00100, 0b01000, 0b00100, 0b00100, 0b00010,
        ],
        '|' => [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        '}' => [
            0b01000, 0b00100, 0b00100, 0b00010, 0b00100, 0b00100, 0b01000,
        ],
        '\u{b7}' => [
            0b00000, 0b00000, 0b00000, 0b00100, 0b00000, 0b00000, 0b00000,
        ],
        '\u{d7}' => [
            0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001,
        ],
        '\u{2014}' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        '\u{20ac}' => [
            0b00111, 0b01000, 0b01000, 0b11110, 0b01000, 0b01000, 0b00111,
        ],
        '\u{2726}' => [
            0b00100, 0b01110, 0b11111, 0b01110, 0b00100, 0b00000, 0b00000,
        ],
        _ => return None,
    };
    Some(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_pixels_for_known_glyph() {
        let mut c = Canvas::transparent(32, 16);
        draw_text(&mut c, 0, 0, "A", [255, 255, 255, 255]);
        // 'A' top row is 0b01110: columns 1..=3 set at row 0.
        assert_eq!(c.pixel(0, 0), Some([0, 0, 0, 0]));
        assert_eq!(c.pixel(1, 0), Some([255, 255, 255, 255]));
        assert_eq!(c.pixel(3, 0), Some([255, 255, 255, 255]));
        assert_eq!(c.pixel(4, 0), Some([0, 0, 0, 0]));
        // Crossbar row (row 3) is 0b11111: all five columns.
        assert_eq!(c.pixel(0, 3), Some([255, 255, 255, 255]));
        assert_eq!(c.pixel(4, 3), Some([255, 255, 255, 255]));
        // Below the glyph: empty.
        assert_eq!(c.pixel(1, 7), Some([0, 0, 0, 0]));
    }

    #[test]
    fn advances_six_pixels_per_char_and_supports_lowercase() {
        let mut c = Canvas::transparent(32, 16);
        draw_text(&mut c, 0, 0, "a", [255, 255, 255, 255]);
        let mut d = Canvas::transparent(32, 16);
        draw_text(&mut d, 0, 0, "A", [255, 255, 255, 255]);
        assert_eq!(c.as_rgba(), d.as_rgba());

        let mut e = Canvas::transparent(32, 16);
        draw_text(&mut e, 0, 0, "AB", [255, 255, 255, 255]);
        assert_eq!(e.pixel(6, 0), Some([255, 255, 255, 255])); // B starts at 6
    }

    #[test]
    fn unknown_glyphs_are_blank_but_still_advance() {
        // '\u{0007}' (BEL) is deliberately outside the glyph table and must
        // remain unsupported so this regression guard keeps meaning.
        let mut c = Canvas::transparent(32, 16);
        draw_text(&mut c, 0, 0, "\u{0007}", [255, 255, 255, 255]);
        assert_eq!(
            c.as_rgba(),
            &image::RgbaImage::from_pixel(32, 16, image::Rgba([0, 0, 0, 0]))
        );
        assert_eq!(text_width("\u{0007}"), ADVANCE - 1);
    }

    #[test]
    fn text_width_matches_draw_extent() {
        assert_eq!(text_width(""), -1);
        assert_eq!(text_width("A"), 5);
        assert_eq!(text_width("AB"), 11);
    }
    #[test]
    fn ttf_load_then_draw_lights_pixels_and_stays_in_bounds() {
        // DejaVu is typically installed; if not, None is the documented
        // fallback condition and the test still passes.
        if let Some(font) = TtfFont::load(false) {
            let mut c = Canvas::transparent(64, 16);
            draw_text_ttf(&mut c, &font, 0, 0, "AB", 12.0, [255, 255, 255, 255]);
            assert!(
                c.as_rgba().pixels().any(|p| p.0[3] > 0),
                "draw_text_ttf must light at least one pixel"
            );
        }
    }

    #[test]
    fn ttf_draw_multi_char_and_whitespace_is_deterministic() {
        if let Some(font) = TtfFont::load(false) {
            let mut a = Canvas::transparent(96, 24);
            let mut b = Canvas::transparent(96, 24);
            draw_text_ttf(&mut a, &font, 2, 2, "GDI 99", 12.0, [70, 196, 233, 255]);
            draw_text_ttf(&mut b, &font, 2, 2, "GDI 99", 12.0, [70, 196, 233, 255]);
            assert_eq!(a.as_rgba(), b.as_rgba());
            // Drawing with a fully-clipped origin must not panic either.
            let mut c = Canvas::transparent(4, 4);
            draw_text_ttf(&mut c, &font, 0, 0, "WWWWWWWW", 9.0, [255, 0, 0, 255]);
        }
    }

    #[test]
    fn ttf_draw_advances_glyphs_without_overlap() {
        if let Some(font) = TtfFont::load(false) {
            let mut c = Canvas::transparent(96, 24);
            draw_text_ttf(&mut c, &font, 0, 4, "JJJ", 10.0, [255, 0, 0, 255]);
            // Rightmost lit pixel: three advanced 'J' glyphs at size 10 must
            // reach at least one full advance grid cell wide.
            let mut last_lit = 0i32;
            for x in 0..96 {
                for y in 0..24 {
                    if c.pixel(x, y).is_some_and(|p| p[3] > 0) {
                        last_lit = x;
                    }
                }
            }
            // Three advanced glyphs must span at least two advances.
            let advance = font.font.metrics('J', 10.0).advance_width;
            assert!(
                last_lit as f32 >= 2.0 * advance - 1.0,
                "last lit pixel {last_lit} did not reach two advances ({advance})"
            );
        }
    }

    /// Characters the pack's rendered labels use. Pinned so a future label
    /// that needs a new glyph fails loudly instead of rendering blank.
    const LABEL_CHARSET: &str =
        "%(),-./012345689:ABCDEFGHIKLMNOPRSTUVWYZ_abcdefghiklmnopqrstuvwxy{|}·×—€✦";

    /// Label strings the pack actually renders (the Python generator draws
    /// them with Pillow's default font; the 5x7 fallback must not leave any
    /// character blank).
    const RENDERED_LABELS: &[&str] = &[
        "Act IV \u{2014} Counterpunch",
        "BRIEFING \u{2014} ACT I",
        "\u{2726} CNC32",
        "07_border_prologue  |  Prologue: The Border",
    ];

    #[test]
    fn every_label_character_has_a_glyph() {
        for ch in LABEL_CHARSET.chars() {
            assert!(
                glyph(ch).is_some(),
                "missing glyph for {:?} U+{:04X}",
                ch,
                ch as u32
            );
        }
    }

    #[test]
    fn rendered_label_strings_are_fully_covered() {
        for label in RENDERED_LABELS {
            for ch in label.chars() {
                if ch == ' ' {
                    continue;
                }
                assert!(
                    glyph(ch).is_some(),
                    "missing glyph for {:?} U+{:04X} in label {:?}",
                    ch,
                    ch as u32,
                    label
                );
            }
        }
    }
}
