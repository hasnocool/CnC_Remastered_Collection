// Palette shader
use crate::Palette;

pub struct PaletteShader { pub palette_texture: u32 }

impl PaletteShader { pub fn new(palette: &Palette) -> Self { Self { palette_texture: 0 } } pub fn update(&mut self, palette: &Palette) {} }
