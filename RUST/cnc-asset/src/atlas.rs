//! Procedural sprite atlas generation and loading.
//!
//! The source release does not ship the original SHP/VXL art, so this module
//! can generate a replacement sprite sheet in a simple top-down style and
//! round-trip it through PNG + JSON. The game uses the same atlas at runtime
//! via cnc_asset::SpriteAtlas.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

pub const ATLAS_SIZE: u32 = 512;
pub const CELL: u32 = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtlasStyle {
    Classic,
    Neon,
}

impl AtlasStyle {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "classic" => Some(Self::Classic),
            "neon" => Some(Self::Neon),
            _ => None,
        }
    }

    fn palette(self) -> Palette {
        match self {
            AtlasStyle::Classic => Palette {
                body: [128, 128, 122, 255],
                light: [205, 205, 195, 255],
                dark: [55, 55, 50, 255],
                accent: [235, 200, 80, 255],
                shadow: [0, 0, 0, 150],
            },
            AtlasStyle::Neon => Palette {
                body: [70, 78, 92, 255],
                light: [150, 160, 180, 255],
                dark: [30, 34, 44, 255],
                accent: [0, 220, 255, 255],
                shadow: [0, 0, 0, 160],
            },
        }
    }
}

#[derive(Clone, Copy)]
struct Palette {
    body: [u8; 4],
    light: [u8; 4],
    dark: [u8; 4],
    accent: [u8; 4],
    shadow: [u8; 4],
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpriteFrame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize, Deserialize)]
struct AtlasManifest {
    width: u32,
    height: u32,
    frames: HashMap<String, SpriteFrame>,
}

#[derive(Clone, Debug)]
pub struct SpriteAtlas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
    pub frames: HashMap<String, SpriteFrame>,
}

/// Frame keys match the Rust ClassId Debug names used by the game.
pub const FRAME_KEYS: &[&str] = &[
    "InfantryE1",
    "VehicleE1",
    "VehicleE2",
    "VehicleE4",
    "VehicleE6",
    "AircraftE1",
    "AircraftE2",
    "ConstructionYard",
    "PowerPlant",
    "Refinery",
    "Barracks",
    "WarFactory",
    "Helipad",
    "Silo",
    "Radar",
    "MissileSilo",
];

impl SpriteAtlas {
    pub fn generate(style: AtlasStyle) -> Self {
        let mut atlas = Self {
            width: ATLAS_SIZE,
            height: ATLAS_SIZE,
            pixels: vec![0; (ATLAS_SIZE * ATLAS_SIZE * 4) as usize],
            frames: HashMap::new(),
        };
        let palette = style.palette();
        let columns = ATLAS_SIZE / CELL;

        for (index, key) in FRAME_KEYS.iter().enumerate() {
            let column = (index as u32) % columns;
            let row = (index as u32) / columns;
            let x = column * CELL;
            let y = row * CELL;
            atlas.draw_icon(x as i32, y as i32, key, &palette);
            atlas.frames.insert(
                (*key).to_string(),
                SpriteFrame {
                    x,
                    y,
                    w: CELL,
                    h: CELL,
                },
            );
        }
        atlas
    }

    pub fn frame(&self, key: &str) -> Option<SpriteFrame> {
        self.frames.get(key).copied()
    }

    pub fn save(&self, dir: &Path) -> Result<()> {
        std::fs::create_dir_all(dir)?;
        let image = image::RgbaImage::from_raw(self.width, self.height, self.pixels.clone())
            .context("atlas buffer has the wrong size")?;
        image.save(dir.join("atlas.png"))?;
        let manifest = AtlasManifest {
            width: self.width,
            height: self.height,
            frames: self.frames.clone(),
        };
        std::fs::write(
            dir.join("atlas.json"),
            serde_json::to_vec_pretty(&manifest)?,
        )?;
        Ok(())
    }

    pub fn load(dir: &Path) -> Result<Self> {
        let image = image::open(dir.join("atlas.png"))
            .with_context(|| format!("failed to open atlas in {:?}", dir))?
            .to_rgba8();
        let (width, height) = image.dimensions();
        let manifest: AtlasManifest = serde_json::from_str(
            &std::fs::read_to_string(dir.join("atlas.json"))
                .with_context(|| format!("failed to read atlas manifest in {:?}", dir))?,
        )?;
        Ok(Self {
            width,
            height,
            pixels: image.into_raw(),
            frames: manifest.frames,
        })
    }

    pub fn load_or_generate(dir: &Path, style: AtlasStyle) -> Self {
        Self::load(dir).unwrap_or_else(|_| Self::generate(style))
    }

    fn put(&mut self, ox: i32, oy: i32, x: i32, y: i32, color: [u8; 4]) {
        if x < 0 || y < 0 || x >= CELL as i32 || y >= CELL as i32 {
            return;
        }
        let ax = (ox + x) as u32;
        let ay = (oy + y) as u32;
        if ax >= self.width || ay >= self.height {
            return;
        }
        let index = ((ay * self.width + ax) * 4) as usize;
        self.pixels[index..index + 4].copy_from_slice(&color);
    }

    fn fill(&mut self, ox: i32, oy: i32, x: i32, y: i32, w: i32, h: i32, color: [u8; 4]) {
        for dy in 0..h {
            for dx in 0..w {
                self.put(ox, oy, x + dx, y + dy, color);
            }
        }
    }

    fn outline(&mut self, ox: i32, oy: i32, x: i32, y: i32, w: i32, h: i32, color: [u8; 4]) {
        self.fill(ox, oy, x, y, w, 1, color);
        self.fill(ox, oy, x, y + h - 1, w, 1, color);
        self.fill(ox, oy, x, y, 1, h, color);
        self.fill(ox, oy, x + w - 1, y, 1, h, color);
    }

    fn disc(&mut self, ox: i32, oy: i32, cx: i32, cy: i32, radius: i32, color: [u8; 4]) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    self.put(ox, oy, cx + dx, cy + dy, color);
                }
            }
        }
    }

    fn line(
        &mut self,
        ox: i32,
        oy: i32,
        mut x0: i32,
        mut y0: i32,
        x1: i32,
        y1: i32,
        color: [u8; 4],
    ) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;
        loop {
            self.put(ox, oy, x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let doubled = 2 * error;
            if doubled >= dy {
                error += dy;
                x0 += sx;
            }
            if doubled <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }

    fn draw_icon(&mut self, ox: i32, oy: i32, key: &str, p: &Palette) {
        match key {
            "InfantryE1" => {
                self.disc(ox, oy, 16, 9, 4, p.light);
                self.fill(ox, oy, 11, 13, 10, 11, p.body);
                self.outline(ox, oy, 11, 13, 10, 11, p.dark);
                self.line(ox, oy, 19, 16, 26, 11, p.accent);
                self.fill(ox, oy, 11, 24, 3, 4, p.dark);
                self.fill(ox, oy, 18, 24, 3, 4, p.dark);
            }
            "VehicleE1" => {
                self.fill(ox, oy, 5, 8, 4, 6, p.dark);
                self.fill(ox, oy, 23, 8, 4, 6, p.dark);
                self.fill(ox, oy, 5, 20, 4, 6, p.dark);
                self.fill(ox, oy, 23, 20, 4, 6, p.dark);
                self.fill(ox, oy, 8, 10, 16, 14, p.light);
                self.outline(ox, oy, 8, 10, 16, 14, p.dark);
                self.fill(ox, oy, 12, 12, 8, 4, p.accent);
            }
            "VehicleE2" => {
                self.fill(ox, oy, 4, 7, 4, 18, p.dark);
                self.fill(ox, oy, 24, 7, 4, 18, p.dark);
                self.fill(ox, oy, 7, 9, 18, 14, p.body);
                self.outline(ox, oy, 7, 9, 18, 14, p.dark);
                self.disc(ox, oy, 16, 16, 6, p.light);
                self.line(ox, oy, 16, 13, 16, 3, p.accent);
                self.disc(ox, oy, 16, 3, 2, p.accent);
            }
            "VehicleE4" => {
                self.fill(ox, oy, 3, 7, 4, 18, p.dark);
                self.fill(ox, oy, 25, 7, 4, 18, p.dark);
                self.fill(ox, oy, 5, 8, 22, 16, p.body);
                self.outline(ox, oy, 5, 8, 22, 16, p.dark);
                self.disc(ox, oy, 16, 16, 7, p.light);
                self.line(ox, oy, 13, 13, 13, 3, p.accent);
                self.line(ox, oy, 19, 13, 19, 3, p.accent);
            }
            "VehicleE6" => {
                self.fill(ox, oy, 4, 8, 4, 16, p.dark);
                self.fill(ox, oy, 24, 8, 4, 16, p.dark);
                self.fill(ox, oy, 6, 10, 20, 14, p.body);
                self.outline(ox, oy, 6, 10, 20, 14, p.dark);
                self.fill(ox, oy, 6, 12, 5, 10, p.dark);
                self.fill(ox, oy, 10, 14, 12, 3, p.accent);
                self.fill(ox, oy, 10, 19, 12, 2, p.accent);
            }
            "AircraftE1" | "AircraftE2" => {
                self.disc(ox, oy, 16, 16, 3, p.light);
                self.line(ox, oy, 4, 10, 28, 22, p.accent);
                self.line(ox, oy, 28, 10, 4, 22, p.accent);
                self.fill(ox, oy, 14, 9, 4, 18, p.body);
                self.outline(ox, oy, 14, 9, 4, 18, p.dark);
                self.fill(ox, oy, 15, 25, 2, 4, p.light);
                self.fill(ox, oy, 10, 7, 12, 2, p.light);
            }
            "ConstructionYard" => {
                self.fill(ox, oy, 3, 10, 26, 18, p.body);
                self.outline(ox, oy, 3, 10, 26, 18, p.dark);
                self.fill(ox, oy, 6, 14, 10, 8, p.light);
                self.line(ox, oy, 24, 10, 24, 4, p.accent);
                self.line(ox, oy, 24, 4, 17, 4, p.accent);
                self.disc(ox, oy, 17, 4, 2, p.accent);
            }
            "PowerPlant" => {
                self.fill(ox, oy, 4, 12, 24, 16, p.body);
                self.outline(ox, oy, 4, 12, 24, 16, p.dark);
                self.disc(ox, oy, 16, 20, 7, p.light);
                self.line(ox, oy, 16, 13, 13, 18, p.accent);
                self.line(ox, oy, 13, 18, 18, 18, p.accent);
                self.line(ox, oy, 18, 18, 14, 25, p.accent);
            }
            "Refinery" => {
                self.fill(ox, oy, 4, 12, 24, 16, p.body);
                self.outline(ox, oy, 4, 12, 24, 16, p.dark);
                self.disc(ox, oy, 10, 20, 5, p.light);
                self.disc(ox, oy, 22, 20, 5, p.light);
                self.line(ox, oy, 10, 15, 22, 15, p.accent);
            }
            "Barracks" => {
                self.fill(ox, oy, 5, 10, 22, 18, p.body);
                self.outline(ox, oy, 5, 10, 22, 18, p.dark);
                self.fill(ox, oy, 13, 20, 6, 8, p.dark);
                self.fill(ox, oy, 8, 13, 16, 2, p.accent);
            }
            "WarFactory" => {
                self.fill(ox, oy, 3, 10, 26, 18, p.body);
                self.outline(ox, oy, 3, 10, 26, 18, p.dark);
                self.fill(ox, oy, 8, 20, 16, 8, p.dark);
                self.line(ox, oy, 8, 22, 24, 22, p.accent);
                self.fill(ox, oy, 22, 5, 4, 5, p.dark);
            }
            "Helipad" => {
                self.fill(ox, oy, 4, 14, 24, 14, p.body);
                self.outline(ox, oy, 4, 14, 24, 14, p.dark);
                self.line(ox, oy, 11, 18, 11, 25, p.accent);
                self.line(ox, oy, 20, 18, 20, 25, p.accent);
                self.line(ox, oy, 11, 21, 20, 21, p.accent);
            }
            "Silo" => {
                self.disc(ox, oy, 16, 20, 9, p.body);
                self.disc(ox, oy, 16, 20, 5, p.light);
                self.line(ox, oy, 7, 20, 25, 20, p.dark);
                self.fill(ox, oy, 9, 27, 3, 3, p.dark);
                self.fill(ox, oy, 20, 27, 3, 3, p.dark);
            }
            "Radar" => {
                self.fill(ox, oy, 8, 22, 16, 6, p.body);
                self.outline(ox, oy, 8, 22, 16, 6, p.dark);
                self.line(ox, oy, 16, 22, 16, 10, p.light);
                self.disc(ox, oy, 16, 9, 6, p.accent);
                self.line(ox, oy, 12, 9, 20, 9, p.dark);
            }
            "MissileSilo" => {
                for step in 0..8 {
                    self.fill(
                        ox,
                        oy,
                        4 + step * 2,
                        10 + step * 2,
                        24 - step * 4,
                        2,
                        p.body,
                    );
                }
                self.outline(ox, oy, 4, 10, 24, 2, p.dark);
                self.disc(ox, oy, 16, 8, 2, p.accent);
                self.fill(ox, oy, 13, 22, 6, 6, p.dark);
            }
            _ => {
                self.fill(ox, oy, 8, 8, 16, 16, p.body);
                self.outline(ox, oy, 8, 8, 16, 16, p.accent);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_populates_all_frames() {
        let atlas = SpriteAtlas::generate(AtlasStyle::Neon);
        assert_eq!(atlas.width, ATLAS_SIZE);
        assert_eq!(atlas.height, ATLAS_SIZE);
        for key in FRAME_KEYS {
            assert!(atlas.frame(key).is_some(), "missing frame {key}");
        }
        assert!(atlas.pixels.iter().any(|byte| *byte != 0));
    }

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join("cnc_atlas_test");
        let _ = std::fs::remove_dir_all(&dir);
        let atlas = SpriteAtlas::generate(AtlasStyle::Classic);
        atlas.save(&dir).expect("save atlas");
        let loaded = SpriteAtlas::load(&dir).expect("load atlas");
        assert_eq!(loaded.width, atlas.width);
        assert_eq!(loaded.height, atlas.height);
        assert_eq!(loaded.frames, atlas.frames);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
