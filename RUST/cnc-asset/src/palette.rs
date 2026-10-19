//! Palette (.pal) Parser
use anyhow::{bail, Context, Result};
use cnc_core::RGB888;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct PalFile {
    pub colors: [RGB888; 256],
}

impl PalFile {
    pub fn load(data: &[u8]) -> Result<Self> {
        if data.len() < 768 {
            bail!("PAL too small: need 768 bytes, got {}", data.len());
        }
        let mut colors = [RGB888::new(0, 0, 0); 256];
        for (i, color) in colors.iter_mut().enumerate() {
            let offset = i * 3;
            *color = RGB888::new(data[offset], data[offset + 1], data[offset + 2]);
        }
        Ok(Self { colors })
    }

    pub fn load_file(path: &Path) -> Result<Self> {
        let data =
            std::fs::read(path).with_context(|| format!("Failed to read PAL: {:?}", path))?;
        Self::load(&data)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(768);
        for c in &self.colors {
            data.push(c.r);
            data.push(c.g);
            data.push(c.b);
        }
        data
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        std::fs::write(path, self.to_bytes())
            .with_context(|| format!("Failed to write PAL: {:?}", path))?;
        Ok(())
    }

    /// Create grayscale palette
    pub fn grayscale() -> Self {
        let mut colors = [RGB888::new(0, 0, 0); 256];
        for (i, color) in colors.iter_mut().enumerate() {
            *color = RGB888::new(i as u8, i as u8, i as u8);
        }
        Self { colors }
    }

    /// Apply gamma correction
    pub fn with_gamma(mut self, gamma: f32) -> Self {
        for c in &mut self.colors {
            let inv_gamma = 1.0 / gamma;
            c.r = ((c.r as f32 / 255.0).powf(inv_gamma) * 255.0) as u8;
            c.g = ((c.g as f32 / 255.0).powf(inv_gamma) * 255.0) as u8;
            c.b = ((c.b as f32 / 255.0).powf(inv_gamma) * 255.0) as u8;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pal_roundtrip() {
        let pal = PalFile::grayscale();
        let bytes = pal.to_bytes();
        let loaded = PalFile::load(&bytes).unwrap();
        assert_eq!(pal.colors, loaded.colors);
    }
}
