//! C&C Renderer - wgpu-based 2D rendering

pub mod renderer;
pub mod sprite_batch;
pub mod camera;

pub use renderer::{Renderer, WgpuRenderer, SoftwareRenderer, Sprite};
pub use sprite_batch::{SpriteBatch, SpriteInstance};
pub use camera::Camera;
