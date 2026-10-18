//! Color types and palette management

use crate::Fixed16_16;
use core::ops::{Add, Sub, Mul, Div};
use serde::{Serialize, Deserialize};
use bytemuck::{Pod, Zeroable};

// ============================================================================
// 15-bit Color (RGB555) - Original C&C format
// ============================================================================

/// 15-bit RGB color (5 bits per channel) - Westwood format
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable)]
pub struct RGB555(pub u16);

impl RGB555 {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self((((r as u16) & 0x1F) << 10) | (((g as u16) & 0x1F) << 5) | ((b as u16) & 0x1F))
    }

    #[inline]
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    #[inline]
    pub const fn to_bits(self) -> u16 {
        self.0
    }

    #[inline]
    pub const fn r(self) -> u8 {
        ((self.0 >> 10) & 0x1F) as u8
    }

    #[inline]
    pub const fn g(self) -> u8 {
        ((self.0 >> 5) & 0x1F) as u8
    }

    #[inline]
    pub const fn b(self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    #[inline]
    pub fn to_rgb888(self) -> RGB888 {
        RGB888::new(
            (self.r() as u32 * 255 / 31) as u8,
            (self.g() as u32 * 255 / 31) as u8,
            (self.b() as u32 * 255 / 31) as u8,
        )
    }

    #[inline]
    pub fn to_argb8888(self, alpha: u8) -> ARGB8888 {
        let rgb = self.to_rgb888();
        ARGB8888::new(alpha, rgb.r, rgb.g, rgb.b)
    }
}

// ============================================================================
// 16-bit Color (RGB565)
// ============================================================================

/// 16-bit RGB color (5-6-5 bits per channel)
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable)]
pub struct RGB565(pub u16);

impl RGB565 {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self((((r as u16) & 0x1F) << 11) | (((g as u16) & 0x3F) << 5) | ((b as u16) & 0x1F))
    }

    #[inline]
    pub const fn r(self) -> u8 {
        ((self.0 >> 11) & 0x1F) as u8
    }

    #[inline]
    pub const fn g(self) -> u8 {
        ((self.0 >> 5) & 0x3F) as u8
    }

    #[inline]
    pub const fn b(self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    #[inline]
    pub fn to_rgb888(self) -> RGB888 {
        RGB888::new(
            (self.r() as u32 * 255 / 31) as u8,
            (self.g() as u32 * 255 / 63) as u8,
            (self.b() as u32 * 255 / 31) as u8,
        )
    }
}

// ============================================================================
// 24-bit Color (RGB888)
// ============================================================================

/// 24-bit RGB color (8 bits per channel)
#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable, Debug)]
pub struct RGB888 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB888 {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn to_rgb555(self) -> RGB555 {
        RGB555::new(self.r >> 3, self.g >> 3, self.b >> 3)
    }

    #[inline]
    pub fn to_rgb565(self) -> RGB565 {
        RGB565::new(self.r >> 3, self.g >> 2, self.b >> 3)
    }

    #[inline]
    pub fn to_argb8888(self, alpha: u8) -> ARGB8888 {
        ARGB8888::new(alpha, self.r, self.g, self.b)
    }

    /// Lerp between two colors
    pub fn lerp(self, other: Self, t: Fixed16_16) -> Self {
        let t_f32 = t.to_f32();
        let inv_t = 1.0 - t_f32;
        Self {
            r: (self.r as f32 * inv_t + other.r as f32 * t_f32) as u8,
            g: (self.g as f32 * inv_t + other.g as f32 * t_f32) as u8,
            b: (self.b as f32 * inv_t + other.b as f32 * t_f32) as u8,
        }
    }
}

// ============================================================================
// 32-bit Color (ARGB8888)
// ============================================================================

/// 32-bit ARGB color (8 bits per channel)
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable, Debug)]
pub struct ARGB8888(pub u32);

impl ARGB8888 {
    #[inline]
    pub const fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    #[inline]
    pub const fn a(self) -> u8 {
        (self.0 >> 24) as u8
    }

    #[inline]
    pub const fn r(self) -> u8 {
        (self.0 >> 16) as u8
    }

    #[inline]
    pub const fn g(self) -> u8 {
        (self.0 >> 8) as u8
    }

    #[inline]
    pub const fn b(self) -> u8 {
        self.0 as u8
    }

    #[inline]
    pub const fn rgb(self) -> RGB888 {
        RGB888::new(self.r(), self.g(), self.b())
    }

    #[inline]
    pub const fn with_alpha(self, alpha: u8) -> Self {
        Self((self.0 & 0x00FFFFFF) | ((alpha as u32) << 24))
    }

    /// Premultiply alpha
    pub fn premultiply(self) -> Self {
        let a = self.a() as f32 / 255.0;
        Self::new(
            self.a(),
            (self.r() as f32 * a) as u8,
            (self.g() as f32 * a) as u8,
            (self.b() as f32 * a) as u8,
        )
    }
}

// ============================================================================
// Palette Index
// ============================================================================

/// Index into a 256-color palette
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable)]
pub struct PaletteIndex(pub u8);

impl PaletteIndex {
    #[inline]
    pub const fn new(index: u8) -> Self {
        Self(index)
    }

    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }
}

impl From<u8> for PaletteIndex {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PaletteIndex> for u8 {
    #[inline]
    fn from(value: PaletteIndex) -> Self {
        value.0
    }
}

// ============================================================================
// Player Color Remapping
// ============================================================================

/// Player color remap table (maps palette indices to player colors)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerRemap {
    /// Mapping from original palette index to remapped index
    #[serde(with = "serde_big_array::BigArray")]
    pub map: [u8; 256],
}

impl PlayerRemap {
    /// Identity remap (no change)
    pub fn identity() -> Self {
        let mut map = [0u8; 256];
        for i in 0..256 {
            map[i] = i as u8;
        }
        Self { map }
    }

    /// Create remap for a specific player color
    /// Original C&C uses specific palette ranges for player colors
    pub fn player_color(player: u8) -> Self {
        // Player colors typically occupy palette indices 0-15 or similar ranges
        // This is a simplified version - actual implementation would use the game's remap tables
        let mut map = [0u8; 256];
        for i in 0..256 {
            map[i] = i as u8;
        }
        // Remap the player color range (indices 0-15 typically)
        for i in 0..16 {
            map[i] = player.saturating_mul(16).saturating_add(i as u8);
        }
        Self { map }
    }

    /// Apply remap to a palette index
    #[inline]
    pub fn apply(&self, index: PaletteIndex) -> PaletteIndex {
        PaletteIndex(self.map[index.0 as usize])
    }
}

// ============================================================================
// Common Colors
// ============================================================================

impl RGB888 {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
    pub const YELLOW: Self = Self::new(255, 255, 0);
    pub const CYAN: Self = Self::new(0, 255, 255);
    pub const MAGENTA: Self = Self::new(255, 0, 255);
    pub const GRAY: Self = Self::new(128, 128, 128);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0); // With alpha = 0
}

impl ARGB8888 {
    pub const TRANSPARENT: Self = Self(0x00000000);
    pub const BLACK: Self = Self(0xFF000000);
    pub const WHITE: Self = Self(0xFFFFFFFF);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb555() {
        let c = RGB555::new(31, 31, 31); // Max values
        assert_eq!(c.r(), 31);
        assert_eq!(c.g(), 31);
        assert_eq!(c.b(), 31);
    }

    #[test]
    fn test_rgb888_conversion() {
        let c555 = RGB555::new(31, 0, 0); // Red
        let c888 = c555.to_rgb888();
        assert_eq!(c888.r, 255);
        assert_eq!(c888.g, 0);
        assert_eq!(c888.b, 0);
    }

    #[test]
    fn test_argb8888() {
        let c = ARGB8888::new(128, 255, 128, 64);
        assert_eq!(c.a(), 128);
        assert_eq!(c.r(), 255);
        assert_eq!(c.g(), 128);
        assert_eq!(c.b(), 64);
    }

    #[test]
    fn test_player_remap() {
        let remap = PlayerRemap::player_color(1);
        assert_eq!(remap.apply(PaletteIndex(0)).0, 16);
        assert_eq!(remap.apply(PaletteIndex(15)).0, 31);
    }
}