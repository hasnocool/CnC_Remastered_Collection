//! Renderer trait and implementations
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use cnc_core::{Palette, ARGB8888, Rect, Vec2};
use font8x8::{UnicodeFonts, BASIC_FONTS};
use winit::window::Window;

use crate::camera::Camera;
use crate::sprite_batch::{SpriteBatch, SpriteInstance, PALETTE_NONE, TEXTURE_NONE};

pub trait Renderer: Send + Sync {
    fn begin_frame(&mut self);
    fn end_frame(&mut self) -> Result<()>;
    fn draw_sprite(&mut self, sprite: &Sprite);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]);
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: [f32; 4]);
    fn set_camera(&mut self, camera: &Camera);
    fn resize(&mut self, width: u32, height: u32);
    fn set_palette(&mut self, palette: &Palette);
    /// Upload a sprite texture atlas (RGBA8). Backends without sprites ignore it.
    fn set_atlas(&mut self, width: u32, height: u32, rgba: &[u8]) -> Result<()> {
        let _ = (width, height, rgba);
        Ok(())
    }

    /// True when draw_sprite is actually presented by this backend.
    fn uses_sprites(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub struct Sprite {
    pub position: Vec2,
    pub size: Vec2,
    pub texture_id: u32,
    pub uv_rect: Rect,
    pub color: ARGB8888,
    pub rotation: f32,
    pub z_order: i32,
}

impl Sprite {
    pub fn new(texture_id: u32, position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size,
            texture_id,
            uv_rect: Rect::new(0, 0, 1, 1),
            color: ARGB8888::WHITE,
            rotation: 0.0,
            z_order: 0,
        }
    }
}

fn argb_to_rgba(color: ARGB8888) -> [f32; 4] {
    [
        color.r() as f32 / 255.0,
        color.g() as f32 / 255.0,
        color.b() as f32 / 255.0,
        color.a() as f32 / 255.0,
    ]
}

/// Render text using the embedded 8x8 bitmap font.
///
/// Every lit glyph pixel becomes one quad, so this works identically for the
/// software and wgpu renderers without needing a separate glyph atlas.
pub fn draw_text_block(
    renderer: &mut dyn Renderer,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: [f32; 4],
) {
    let scale = (size / 8.0).max(0.5);
    let advance = scale * 10.0;
    let line_height = scale * 11.0;
    let mut cursor_x = x;
    let mut cursor_y = y;

    for ch in text.chars() {
        match ch {
            '\n' => {
                cursor_x = x;
                cursor_y += line_height;
                continue;
            }
            '\r' => continue,
            ' ' => {
                cursor_x += advance * 0.5;
                continue;
            }
            _ => {}
        }

        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (row, bits) in glyph.iter().enumerate() {
                for col in 0..8 {
                    if (bits >> col) & 1 == 1 {
                        renderer.draw_rect(
                            cursor_x + col as f32 * scale,
                            cursor_y + row as f32 * scale,
                            scale,
                            scale,
                            color,
                        );
                    }
                }
            }
        }

        cursor_x += advance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn software_renderer_draws_text_pixels() {
        let mut renderer = SoftwareRenderer::new(64, 16);
        renderer.draw_text("A", 0.0, 0.0, 8.0, [1.0, 1.0, 1.0, 1.0]);
        assert!(
            renderer.buffer().iter().any(|pixel| *pixel != 0),
            "expected draw_text to write non-zero pixels"
        );
    }
}

/// Real wgpu-backed 2D renderer.
pub struct WgpuRenderer {
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    sprite_batch: SpriteBatch,
    camera: Camera,
    current_palette: Option<Palette>,
    acquired_frame: Option<wgpu::SurfaceTexture>,
    frame_error: Option<anyhow::Error>,
    clear_color: wgpu::Color,
}

impl WgpuRenderer {
    /// Create a renderer for the supplied window.
    ///
    /// The window is passed as an Arc so the created surface can outlive this
    /// call and be stored as Surface<'static>.
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance
            .create_surface(window)
            .context("failed to create wgpu surface")?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context("no suitable GPU adapter found")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("CnC Renderer"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .context("failed to create wgpu device")?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|format| format.is_srgb())
            .or_else(|| caps.formats.first().copied())
            .unwrap_or(wgpu::TextureFormat::Bgra8UnormSrgb);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes.first().copied().unwrap_or(wgpu::CompositeAlphaMode::Auto),
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let sprite_batch = SpriteBatch::new(&device, &queue, format)?;

        let mut camera = Camera::default();
        camera.set_viewport(width, height);

        let mut renderer = Self {
            instance,
            device,
            queue,
            surface,
            config,
            size: (width, height),
            sprite_batch,
            camera,
            current_palette: None,
            acquired_frame: None,
            frame_error: None,
            clear_color: wgpu::Color { r: 0.05, g: 0.05, b: 0.08, a: 1.0 },
        };
        renderer
            .sprite_batch
            .set_camera(&renderer.queue, &renderer.camera);
        Ok(renderer)
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.config.format
    }

    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.clear_color = color;
    }

    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface(&self) -> &wgpu::Surface<'static> {
        &self.surface
    }
}

impl Renderer for WgpuRenderer {
    fn begin_frame(&mut self) {
        self.acquired_frame = None;
        self.frame_error = None;
        match self.surface.get_current_texture() {
            Ok(frame) => self.acquired_frame = Some(frame),
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                self.frame_error = Some(anyhow!("surface was lost/outdated and has been reconfigured"));
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                self.frame_error = Some(anyhow!("surface ran out of memory"));
            }
            Err(wgpu::SurfaceError::Timeout) => {
                self.frame_error = Some(anyhow!("timed out acquiring the next surface frame"));
            }
        }
    }

    fn end_frame(&mut self) -> Result<()> {
        let frame = match self.acquired_frame.take() {
            Some(frame) => frame,
            None => {
                return Err(self
                    .frame_error
                    .take()
                    .unwrap_or_else(|| anyhow!("no surface frame was acquired for this frame")))
            }
        };

        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("cnc_frame_encoder"),
        });

        self.sprite_batch.flush(
            &self.device,
            &self.queue,
            &mut encoder,
            &view,
            self.clear_color,
        );

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        self.sprite_batch.clear();
        Ok(())
    }

    fn draw_sprite(&mut self, sprite: &Sprite) {
        let instance = SpriteInstance {
            position: [sprite.position.x.to_f32(), sprite.position.y.to_f32()],
            size: [sprite.size.x.to_f32().max(1.0), sprite.size.y.to_f32().max(1.0)],
            uv_rect: [
                sprite.uv_rect.x as f32,
                sprite.uv_rect.y as f32,
                (sprite.uv_rect.x + sprite.uv_rect.width) as f32,
                (sprite.uv_rect.y + sprite.uv_rect.height) as f32,
            ],
            color: argb_to_rgba(sprite.color),
            rotation: sprite.rotation,
            palette_index: PALETTE_NONE,
            texture_id: if sprite.texture_id == 0 { TEXTURE_NONE } else { sprite.texture_id },
            z_order: sprite.z_order,
        };
        self.sprite_batch.add(instance);
    }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let instance = SpriteInstance {
            position: [x, y],
            size: [w.max(1.0), h.max(1.0)],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
            color,
            rotation: 0.0,
            palette_index: PALETTE_NONE,
            texture_id: TEXTURE_NONE,
            z_order: 0,
        };
        self.sprite_batch.add(instance);
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: [f32; 4]) {
        draw_text_block(self, text, x, y, size, color);
    }

    fn set_camera(&mut self, camera: &Camera) {
        self.camera = *camera;
        self.camera.set_viewport(self.size.0, self.size.1);
        self.sprite_batch.set_camera(&self.queue, &self.camera);
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.size = (width, height);
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.camera.set_viewport(width, height);
        self.sprite_batch.set_camera(&self.queue, &self.camera);
    }

    fn set_palette(&mut self, palette: &Palette) {
        self.sprite_batch.set_palette(&self.queue, palette);
        self.current_palette = Some(palette.clone());
    }

    fn set_atlas(&mut self, width: u32, height: u32, rgba: &[u8]) -> Result<()> {
        self.sprite_batch
            .set_texture(&self.device, &self.queue, width, height, rgba)
    }

    fn uses_sprites(&self) -> bool {
        true
    }
}

pub struct SoftwareRenderer {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
}

impl SoftwareRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, buffer: vec![0; (width * height) as usize] }
    }
    pub fn buffer(&self) -> &[u32] { &self.buffer }
}

impl Renderer for SoftwareRenderer {
    fn begin_frame(&mut self) { self.buffer.fill(0); }
    fn end_frame(&mut self) -> Result<()> { Ok(()) }
    fn draw_sprite(&mut self, _sprite: &Sprite) {}
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let x = x as i32; let y = y as i32; let w = w as i32; let h = h as i32;
        let c = (((color[3]*255.0) as u32) << 24) | (((color[0]*255.0) as u32) << 16) | (((color[1]*255.0) as u32) << 8) | ((color[2]*255.0) as u32);
        for dy in 0..h { for dx in 0..w { let px=x+dx; let py=y+dy; if px>=0 && px<self.width as i32 && py>=0 && py<self.height as i32 { self.buffer[(py*self.width as i32+px) as usize] = c; } } }
    }
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: [f32; 4]) {
        draw_text_block(self, text, x, y, size, color);
    }
    fn set_camera(&mut self, _camera: &Camera) {}
    fn resize(&mut self, width: u32, height: u32) { self.width=width; self.height=height; self.buffer.resize((width*height) as usize, 0); }
    fn set_palette(&mut self, _palette: &Palette) {}
}
