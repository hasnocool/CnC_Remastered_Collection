//! Palette management (256-color with remapping)

use crate::{PaletteIndex, PlayerRemap, ARGB8888, RGB888};
use core::ops::Index;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 256-color palette (Westwood format)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Palette {
    /// Color entries (256 colors)
    #[serde(with = "serde_big_array::BigArray")]
    pub colors: [RGB888; 256],
    /// Optional name
    pub name: Option<String>,
}

impl Palette {
    /// Create empty (black) palette
    pub fn new() -> Self {
        Self {
            colors: [RGB888::BLACK; 256],
            name: None,
        }
    }

    /// Create from raw RGB888 array
    pub fn from_colors(colors: [RGB888; 256]) -> Self {
        Self { colors, name: None }
    }

    /// Create grayscale palette
    pub fn grayscale() -> Self {
        let mut colors = [RGB888::BLACK; 256];
        for (i, color) in colors.iter_mut().enumerate() {
            *color = RGB888::new(i as u8, i as u8, i as u8);
        }
        Self {
            colors,
            name: Some("Grayscale".to_string()),
        }
    }

    /// Get color by index
    #[inline]
    pub fn get(&self, index: PaletteIndex) -> RGB888 {
        self.colors[index.0 as usize]
    }

    /// Get color by index (mutable)
    #[inline]
    pub fn get_mut(&mut self, index: PaletteIndex) -> &mut RGB888 {
        &mut self.colors[index.0 as usize]
    }

    /// Set color at index
    #[inline]
    pub fn set(&mut self, index: PaletteIndex, color: RGB888) {
        self.colors[index.0 as usize] = color;
    }

    /// Convert entire palette to ARGB8888 (for GPU upload)
    pub fn to_argb8888(&self, alpha: u8) -> [ARGB8888; 256] {
        let mut result = [ARGB8888::TRANSPARENT; 256];
        for (entry, color) in result.iter_mut().zip(self.colors.iter()) {
            *entry = color.to_argb8888(alpha);
        }
        result
    }

    /// Apply player remap and convert to ARGB8888
    pub fn to_argb8888_remapped(&self, remap: &PlayerRemap, alpha: u8) -> [ARGB8888; 256] {
        let mut result = [ARGB8888::TRANSPARENT; 256];
        for (i, entry) in result.iter_mut().enumerate() {
            let remapped = remap.apply(PaletteIndex(i as u8));
            *entry = self.colors[remapped.0 as usize].to_argb8888(alpha);
        }
        result
    }

    /// Find closest color index (for quantization)
    pub fn find_closest(&self, target: RGB888) -> PaletteIndex {
        let mut best_idx = 0;
        let mut best_dist = u32::MAX;

        for i in 0..256 {
            let c = self.colors[i];
            let dr = c.r as i32 - target.r as i32;
            let dg = c.g as i32 - target.g as i32;
            let db = c.b as i32 - target.b as i32;
            let dist = (dr * dr + dg * dg + db * db) as u32;
            if dist < best_dist {
                best_dist = dist;
                best_idx = i;
                if dist == 0 {
                    break;
                }
            }
        }

        PaletteIndex(best_idx as u8)
    }

    /// Blend two palettes
    pub fn blend(&self, other: &Palette, t: f32) -> Self {
        let mut colors = [RGB888::BLACK; 256];
        let inv_t = 1.0 - t;
        for ((a, b), entry) in self
            .colors
            .iter()
            .zip(other.colors.iter())
            .zip(colors.iter_mut())
        {
            *entry = RGB888::new(
                (a.r as f32 * inv_t + b.r as f32 * t) as u8,
                (a.g as f32 * inv_t + b.g as f32 * t) as u8,
                (a.b as f32 * inv_t + b.b as f32 * t) as u8,
            );
        }
        Self { colors, name: None }
    }

    /// Load from Westwood .pal file format
    pub fn from_pal_data(data: &[u8]) -> Result<Self, &'static str> {
        // .pal format: 256 * 3 bytes (RGB)
        if data.len() < 256 * 3 {
            return Err("Invalid .pal file: too small");
        }

        let mut colors = [RGB888::BLACK; 256];
        for (i, color) in colors.iter_mut().enumerate() {
            let offset = i * 3;
            *color = RGB888::new(data[offset], data[offset + 1], data[offset + 2]);
        }

        Ok(Self {
            colors,
            name: Some(".pal".to_string()),
        })
    }

    /// Save to Westwood .pal file format
    pub fn to_pal_data(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(256 * 3);
        for c in &self.colors {
            data.push(c.r);
            data.push(c.g);
            data.push(c.b);
        }
        data
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<PaletteIndex> for Palette {
    type Output = RGB888;
    #[inline]
    fn index(&self, index: PaletteIndex) -> &Self::Output {
        &self.colors[index.0 as usize]
    }
}

impl Index<u8> for Palette {
    type Output = RGB888;
    #[inline]
    fn index(&self, index: u8) -> &Self::Output {
        &self.colors[index as usize]
    }
}

impl Index<usize> for Palette {
    type Output = RGB888;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.colors[index]
    }
}

/// Palette registry for managing multiple palettes
#[derive(Default, Clone, Debug)]
pub struct PaletteRegistry {
    palettes: HashMap<String, Palette>,
    default_name: Option<String>,
}

impl PaletteRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn register(&mut self, name: String, palette: Palette) {
        if self.default_name.is_none() {
            self.default_name = Some(name.clone());
        }
        self.palettes.insert(name, palette);
    }

    pub fn get(&self, name: &str) -> Option<&Palette> {
        self.palettes.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Palette> {
        self.palettes.get_mut(name)
    }

    pub fn get_default(&self) -> Option<&Palette> {
        self.default_name
            .as_ref()
            .and_then(|n| self.palettes.get(n))
    }

    pub fn set_default(&mut self, name: &str) -> bool {
        if self.palettes.contains_key(name) {
            self.default_name = Some(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn names(&self) -> impl Iterator<Item = &String> {
        self.palettes.keys()
    }
}

/// Palette animation (cycling colors)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaletteAnimation {
    /// Start index (inclusive)
    pub start: u8,
    /// End index (inclusive)
    pub end: u8,
    /// Animation speed (frames per cycle)
    pub speed: u16,
    /// Current frame
    #[serde(skip)]
    pub frame: u16,
    /// Direction (1 = forward, -1 = backward)
    pub direction: i8,
}

impl PaletteAnimation {
    pub fn new(start: u8, end: u8, speed: u16) -> Self {
        Self {
            start,
            end,
            speed,
            frame: 0,
            direction: 1,
        }
    }

    /// Update animation and apply to palette
    pub fn update(&mut self, palette: &mut Palette) {
        self.frame = self.frame.wrapping_add(1);
        if self.frame >= self.speed {
            self.frame = 0;

            // Rotate the color range
            let start = self.start as usize;
            let end = self.end as usize;
            if start <= end {
                let temp = palette.colors[start];
                for i in start..end {
                    palette.colors[i] = palette.colors[i + 1];
                }
                palette.colors[end] = temp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_creation() {
        let palette = Palette::new();
        assert_eq!(palette.get(PaletteIndex(0)), RGB888::BLACK);
    }

    #[test]
    fn test_grayscale() {
        let palette = Palette::grayscale();
        assert_eq!(palette.get(PaletteIndex(0)), RGB888::new(0, 0, 0));
        assert_eq!(palette.get(PaletteIndex(128)), RGB888::new(128, 128, 128));
        assert_eq!(palette.get(PaletteIndex(255)), RGB888::new(255, 255, 255));
    }

    #[test]
    fn test_find_closest() {
        let palette = Palette::grayscale();
        let idx = palette.find_closest(RGB888::new(100, 100, 100));
        assert_eq!(idx.0, 100);
    }

    #[test]
    fn test_pal_roundtrip() {
        let palette = Palette::grayscale();
        let data = palette.to_pal_data();
        let loaded = Palette::from_pal_data(&data).unwrap();
        assert_eq!(palette.colors, loaded.colors);
    }
}
