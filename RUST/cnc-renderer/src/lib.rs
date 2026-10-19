//! C&C Renderer - wgpu-based 2D rendering
//!
//! Atlases: slot 0 is the procedural unit/building sprite atlas
//! (`Renderer::set_atlas`); slot `TILESET_TEXTURE_ID` is the RTS 16x16
//! tileset pack. `Sprite::texture_id` selects which slot a sprite samples.

pub mod camera;
pub mod palette_shader;
pub mod renderer;
pub mod sprite_batch;
pub mod ui;

pub use camera::Camera;
pub use palette_shader::{
    sample_palette_cpu, PaletteDrawMode, PaletteShader, MODE_FLAG_REMAP, MODE_FLAG_SHADOW,
    MODE_FLAG_TRANSPARENT, PALETTE_SHADER_WGSL,
};
pub use renderer::{Renderer, SoftwareRenderer, Sprite, WgpuRenderer, TILESET_TEXTURE_ID};
pub use sprite_batch::{SpriteBatch, SpriteInstance};
pub use ui::{button, console, panel, widget_at, ButtonState, WidgetRect};
