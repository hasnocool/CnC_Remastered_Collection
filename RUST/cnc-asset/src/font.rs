//! Westwood `.fnt` bitmap-font parser.
//!
//! Wire layout (community-documented TD/RA font format; the remastered C++
//! release carries no font loader, so this mirrors the DOS-era format used by
//! the two engines):
//!
//! | offset              | content                              |
//! |---------------------|--------------------------------------|
//! | 0                   | u16 LE max height (pixels)           |
//! | 2                   | 256 × i8 signed glyph widths         |
//! | 258                 | 256 × u16 LE glyph data offsets      |
//! | per glyph           | 1 control byte, then 1bpp rows       |
//!
//! A width ≤ 0 or an offset equal to [`NO_DATA_OFFSET`] means the character
//! has no glyph raster. Otherwise the glyph block starts at its offset: one
//! control byte (VRAM back-reference, skipped here), then
//! `max_height` rows of `ceil(width / 8)` bytes each, 1 bit per pixel,
//! MSB-first (bit 7 of the first row byte is the leftmost pixel).
//!
//! [`FontFile::encode`] writes this exact layout, so the parser round-trips
//! and asset tooling can bake modern glyph sheets into `.fnt`.
//!
//! Assumptions flagged for the first load of a real game font: the control
//! byte size and the MSB-first bit order. `parse` validates every glyph row
//! against the remaining bytes and fails loudly instead of decoding garbage.

use anyhow::{bail, Context, Result};
use image::RgbaImage;

/// Offset sentinel meaning "no glyph data" for a character.
pub const NO_DATA_OFFSET: u16 = 0x7FFF;

pub const CHAR_COUNT: usize = 256;

/// One character raster: a `width`-wide, `max_height`-tall 1bpp bitmap.
/// Layout is row-major; every row is `ceil(width / 8)` bytes, MSB-first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontGlyph {
    pub width: u8,
    pub bytes: Vec<u8>,
}

impl FontGlyph {
    /// Set/clear one pixel for builders (encode test fonts, bake sheets).
    pub fn set_pixel(&mut self, x: usize, y: usize, on: bool) {
        debug_assert!((x as u16) < self.width as u16 && y < self.row_count());
        let row = self.bytes_per_row();
        let bit = 7 - (x % 8);
        if on {
            self.bytes[y * row + x / 8] |= 1 << bit;
        } else {
            self.bytes[y * row + x / 8] &= !(1 << bit);
        }
    }

    fn bytes_per_row(&self) -> usize {
        self.width.div_ceil(8) as usize
    }

    pub fn row_count(&self) -> usize {
        self.bytes.len() / self.bytes_per_row().max(1)
    }
}

#[derive(Clone, Debug)]
pub struct FontFile {
    pub max_height: u16,
    pub widths: [i8; CHAR_COUNT],
    /// Parallel raster for each character code, `None` where the header marks
    /// the glyph blank or missing.
    pub glyphs: [Option<FontGlyph>; CHAR_COUNT],
}

impl FontFile {
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < 2 + CHAR_COUNT {
            bail!(
                ".fnt too small: need header ({} bytes) + width table, got {}",
                2 + CHAR_COUNT,
                data.len()
            );
        }
        let max_height = u16::from_le_bytes([data[0], data[1]]);
        if max_height == 0 {
            bail!(".fnt header declares max height 0");
        }
        let mut widths = [0i8; CHAR_COUNT];
        for (slot, &raw) in widths.iter_mut().zip(data[2..2 + CHAR_COUNT].iter()) {
            *slot = raw as i8;
        }
        let offsets_start = 2 + CHAR_COUNT;
        if data.len() < offsets_start + 2 * CHAR_COUNT {
            bail!(
                ".fnt missing offset table: need {} bytes, got {}",
                offsets_start + 2 * CHAR_COUNT,
                data.len()
            );
        }
        let mut offsets = [0u16; CHAR_COUNT];
        for (index, slot) in offsets.iter_mut().enumerate() {
            let at = offsets_start + index * 2;
            *slot = u16::from_le_bytes([data[at], data[at + 1]]);
        }

        let mut glyphs: [Option<FontGlyph>; CHAR_COUNT] = std::array::from_fn(|_| None);
        for (index, &offset) in offsets.iter().enumerate() {
            let width = widths[index];
            if width <= 0 || offset == NO_DATA_OFFSET {
                continue;
            }
            let width = width as u8 as usize;
            let bytes_per_row = width.div_ceil(8);
            let needed = bytes_per_row * max_height as usize;
            let start = offset as usize;
            // +1 control byte guarding: start of data after the control byte.
            if start >= data.len() {
                bail!("glyph {} offset {} beyond end of file", index, start);
            }
            let data_start = start + 1;
            if data.len() < data_start + needed {
                bail!(
                    "glyph {} truncated: need {} raster bytes at {}, have {}",
                    index,
                    needed,
                    data_start,
                    data.len() - data_start
                );
            }
            glyphs[index] = Some(FontGlyph {
                width: width as u8,
                bytes: data[data_start..data_start + needed].to_vec(),
            });
        }

        Ok(Self {
            max_height,
            widths,
            glyphs,
        })
    }

    /// Glyph raster for `ch` (Latin-1 subset; no glyph -> `None`).
    pub fn glyph(&self, ch: u8) -> Option<&FontGlyph> {
        self.glyphs[ch as usize].as_ref()
    }

    /// Advance of one char in pixels (0 for blank glyphs).
    pub fn char_width(&self, ch: u8) -> u16 {
        self.widths[ch as usize].max(0) as u16
    }

    /// Total advance of `text` in pixels for vertical layout.
    pub fn text_width(&self, text: &str) -> u16 {
        text.chars()
            .map(|ch| {
                if ch.is_ascii() {
                    self.char_width(ch as u8)
                } else {
                    0
                }
            })
            .sum()
    }

    /// Rasterize `text` to RGBA (alpha=0 for `bg` unset pixels).
    /// Returns (row stride tokens: stride, height, pixels).
    pub fn rasterize(&self, text: &str, fg: [u8; 4]) -> Result<RgbaImage> {
        let width = self.text_width(text);
        let height = self.max_height;
        if width == 0 {
            bail!("nothing to rasterize: all glyph widths are blank");
        }
        let mut image = RgbaImage::new(width.max(1) as u32, height.max(1) as u32);
        let mut cursor = 0usize;
        for ch in text.chars() {
            if !ch.is_ascii() {
                continue;
            }
            let Some(glyph) = self.glyph(ch as u8) else {
                continue;
            };
            let gwidth = glyph.width as usize;
            let row_len = glyph.bytes_per_row();
            for (row, row_bytes) in glyph.bytes.chunks(row_len).enumerate() {
                for col in 0..gwidth {
                    let bit = 7 - (col % 8);
                    if row_bytes
                        .get(col / 8)
                        .is_some_and(|byte| (byte >> bit) & 1 == 1)
                    {
                        let x = cursor + col;
                        if (x as u32) < image.width() && row < image.height() as usize {
                            image.get_pixel_mut(x as u32, row as u32).0 = fg;
                        }
                    }
                }
            }
            cursor += gwidth;
        }
        Ok(image)
    }

    /// Encode a font to the same wire layout [`FontFile::parse`] reads.
    /// A zero `max_height` is written as-is; [`FontFile::parse`] rejects it,
    /// so malformed headers can be round-tripped and then rejected on read.
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.max_height.to_le_bytes());
        for &width in &self.widths {
            out.push(width as u8);
        }
        // Reserve the offset table, then fill glyph blobs.
        let offsets_at = out.len();
        out.extend(std::iter::repeat_n(0u8, 2 * CHAR_COUNT));

        let mut offsets = vec![NO_DATA_OFFSET; CHAR_COUNT];
        for (index, glyph) in self.glyphs.iter().enumerate() {
            let Some(glyph) = glyph else {
                continue;
            };
            let width = self.widths[index];
            if width <= 0 {
                continue;
            }
            let bytes_per_row = usize::from(glyph.width.div_ceil(8));
            if glyph.bytes.len() < bytes_per_row * self.max_height as usize {
                bail!(
                    "glyph {} is too small for max_height: {} bytes at {} per row",
                    index,
                    glyph.bytes.len(),
                    bytes_per_row
                );
            }
            offsets[index] = out
                .len()
                .try_into()
                .with_context(|| format!("glyph {index} offset overflows u16"))?;
            out.push(0); // control byte
            let needed = bytes_per_row * self.max_height as usize;
            out.extend_from_slice(&glyph.bytes[..needed]);
        }
        let write = bytes_of_u16(&offsets);
        out[offsets_at..offsets_at + write.len()].copy_from_slice(&write);
        Ok(out)
    }
}

fn bytes_of_u16(values: &[u16]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 8x8 'A' box outline raster (width 8 fits one byte per row).
    fn glyph_a() -> FontGlyph {
        let bytes_per_row = 1;
        let rows = 8;
        let mut glyph = FontGlyph {
            width: 8,
            bytes: vec![0; bytes_per_row * rows],
        };
        for x in 0..8 {
            glyph.set_pixel(x, 0, true);
            glyph.set_pixel(x, 7, true);
        }
        for y in 1..7 {
            glyph.set_pixel(0, y, true);
            glyph.set_pixel(7, y, true);
        }
        glyph
    }

    fn font_with_a() -> FontFile {
        let mut widths = [0i8; CHAR_COUNT];
        widths[b'A' as usize] = 8;
        widths[b'.' as usize] = 2;
        let mut glyphs: [Option<FontGlyph>; CHAR_COUNT] = std::array::from_fn(|_| None);
        glyphs[b'A' as usize] = Some(glyph_a());
        glyphs[b'.' as usize] = Some(FontGlyph {
            width: 2,
            // '.' MSB-first: first column lit on the top two rows only
            // (second column must stay dark).
            bytes: vec![0x80, 0x80, 0, 0, 0, 0, 0, 0],
        });
        FontFile {
            max_height: 8,
            widths,
            glyphs,
        }
    }

    #[test]
    fn encode_parse_roundtrip() {
        let font = font_with_a();
        let bytes = font.encode().expect("encode");
        let parsed = FontFile::parse(&bytes).expect("parse");
        assert_eq!(parsed.max_height, 8);
        assert_eq!(parsed.widths[b'A' as usize], 8);
        assert_eq!(parsed.glyph(b'A'), font.glyph(b'A'), "A raster intact");
        assert_eq!(parsed.glyph(b'.'), font.glyph(b'.'));
        assert_eq!(parsed.glyph(b'z'), None, "untouched chars blank");
    }

    #[test]
    fn blank_chars_resolve_no_glyph() {
        let font = font_with_a();
        let bytes = font.encode().expect("encode");
        let parsed = FontFile::parse(&bytes).expect("parse");
        assert_eq!(parsed.char_width(b'z'), 0);
        assert_eq!(parsed.glyph(b'z'), None);
        assert!(parsed.text_width("A..") == 8 + 4);
    }

    #[test]
    fn rasterize_places_glyph_pixels() {
        let font = font_with_a();
        let image = font
            .rasterize("A.", [255, 255, 255, 255])
            .expect("rasterize");
        assert_eq!(image.width(), 10);
        assert_eq!(image.height(), 8);
        let lit = |x: u32, y: u32| image.get_pixel(x, y).0[3] != 0;
        // 'A' outline box at 0..8
        assert!(lit(0, 0) && lit(7, 0) && lit(0, 7) && lit(7, 7));
        assert!(!lit(3, 3), "box interior stays empty");
        // '.': first column lit on the top two rows, second column dark.
        assert!(lit(8, 0) && lit(8, 1), "'.' first column lit rows 0-1");
        assert!(!lit(8, 2) && !lit(8, 7), "'.' rows 2-7 dark");
        assert!(
            !(0..8).all(|y| lit(9, y)),
            "'.' second column must stay dark: {:?}",
            font.glyph(b'.')
        );
    }

    #[test]
    fn truncated_glyph_fails_loudly() {
        let mut bytes = font_with_a().encode().expect("encode");
        let last = bytes.len();
        bytes.truncate(last.saturating_sub(4));
        let result = FontFile::parse(&bytes);
        assert!(result.is_err(), "truncated raster must be rejected");
    }

    #[test]
    fn too_small_header_fails() {
        assert!(FontFile::parse(&[8, 0]).is_err());
        assert!(FontFile::parse(&[]).is_err());
    }

    #[test]
    fn zero_max_height_fails() {
        let mut bytes = font_with_a().encode().expect("encode");
        bytes[0] = 0;
        bytes[1] = 0;
        assert!(FontFile::parse(&bytes).is_err(), "max_height 0 rejected");
    }
}
