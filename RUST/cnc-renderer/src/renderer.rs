//! Renderer trait and implementations
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use cnc_core::{Palette, Rect, Vec2, ARGB8888};
use font8x8::{UnicodeFonts, BASIC_FONTS};
use winit::window::Window;

use crate::camera::Camera;
use crate::palette_shader::{PaletteDrawMode, PaletteShader};
use crate::sprite_batch::{SpriteBatch, SpriteInstance, PALETTE_NONE, TEXTURE_NONE};

pub trait Renderer: Send + Sync {
    fn begin_frame(&mut self);
    fn end_frame(&mut self) -> Result<()>;
    fn draw_sprite(&mut self, sprite: &Sprite);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]);
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: [f32; 4]);
    fn set_camera(&mut self, camera: &Camera);
    /// Submit everything queued so far with the current camera, so draws after
    /// the next camera change composite on top instead of sharing its transform.
    /// Backends that apply the camera per draw can leave this empty.
    fn begin_layer(&mut self) {}
    fn resize(&mut self, width: u32, height: u32);
    fn set_palette(&mut self, palette: &Palette);
    /// Upload the procedural unit/building sprite atlas (RGBA8) into texture
    /// slot 0. Equivalent to `set_atlas_texture(0, ..)`. Backends without
    /// sprites ignore it.
    fn set_atlas(&mut self, width: u32, height: u32, rgba: &[u8]) -> Result<()> {
        let _ = (width, height, rgba);
        Ok(())
    }

    /// Upload a named atlas texture slot (RGBA8).
    ///
    /// Slot 0 is the procedural unit/building atlas (see [`Renderer::set_atlas`]);
    /// `TILESET_TEXTURE_ID` is the RTS tileset. `Sprite::texture_id` selects
    /// which slot a sprite samples. Backends without atlas support report an
    /// error so callers fall back to flat colored quads.
    fn set_atlas_texture(
        &mut self,
        texture_id: u32,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) -> Result<()> {
        let _ = (texture_id, width, height, rgba);
        Err(anyhow!("atlas textures unsupported"))
    }

    /// True when `texture_id` has an uploaded atlas and draw_sprite samples
    /// from it.
    fn texture_ready(&self, texture_id: u32) -> bool {
        let _ = texture_id;
        false
    }

    /// Monotonic count of successful atlas uploads on this backend.
    ///
    /// Zero means no sprite atlas has been uploaded yet. Callers that own a
    /// specific atlas (terrain tileset packs) use this to upload it lazily,
    /// exactly once per renderer, instead of per frame.
    fn atlas_generation(&self) -> u64 {
        0
    }

    /// True when slot 0 (the unit atlas) has been uploaded; see
    /// [`Renderer::set_atlas`]. Prefer [`Renderer::texture_ready`] for
    /// per-slot ownership checks.
    fn atlas_ready(&self) -> bool {
        false
    }

    /// True when draw_sprite is actually presented by this backend.
    fn uses_sprites(&self) -> bool {
        false
    }
}

/// Texture id of the RTS tileset atlas. Slot 0 (any other upload via
/// [`Renderer::set_atlas`]) is the procedural unit/building atlas; sprites
/// select their atlas through [`Sprite::texture_id`].
pub const TILESET_TEXTURE_ID: u32 = 1;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub position: Vec2,
    pub size: Vec2,
    /// Which atlas this sprite samples: 0 = procedural unit atlas (slot 0),
    /// `TILESET_TEXTURE_ID` (1) = RTS tileset atlas. Sprites idling on a slot
    /// that was never uploaded are treated as flat.
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

/// Real wgpu-backed 2D renderer.
pub struct WgpuRenderer {
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    sprite_batch: SpriteBatch,
    /// Indexed-sprite palette pipeline (palette + player remap textures,
    /// shadow/transparent modes). See `palette_shader`.
    palette_shader: Option<PaletteShader>,
    camera: Camera,
    current_palette: Option<Palette>,
    acquired_frame: Option<wgpu::SurfaceTexture>,
    frame_error: Option<anyhow::Error>,
    clear_color: wgpu::Color,
    /// Count of successful atlas uploads; see Renderer::atlas_generation.
    atlas_generation: u64,
    /// True once a layer of the current frame has been submitted; later layers
    /// load instead of clearing the surface.
    layer_submitted: bool,
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
            alpha_mode: caps
                .alpha_modes
                .first()
                .copied()
                .unwrap_or(wgpu::CompositeAlphaMode::Auto),
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let sprite_batch = SpriteBatch::new(&device, &queue, format)?;

        let palette_shader = PaletteShader::new(&device, &queue, format).ok();

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
            palette_shader,
            camera,
            current_palette: None,
            acquired_frame: None,
            frame_error: None,
            clear_color: wgpu::Color {
                r: 0.05,
                g: 0.05,
                b: 0.08,
                a: 1.0,
            },
            atlas_generation: 0,
            layer_submitted: false,
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
    /// Upload a per-house `PlayerRemap` table into the indexed palette
    /// pipeline (identity keeps the remap flag off; see `palette_shader`).
    pub fn set_player_remap(&mut self, map: [u8; 256]) {
        if let Some(shader) = self.palette_shader.as_mut() {
            shader.set_remap(&self.queue, map);
        }
    }

    /// Set the indexed palette pipeline's draw mode (shadow/transparent).
    pub fn set_palette_mode(&mut self, mode: PaletteDrawMode) {
        if let Some(shader) = self.palette_shader.as_mut() {
            shader.set_mode(&self.queue, mode);
        }
    }

    /// Replace the indexed pipeline's 1-byte-per-pixel sprite texture.
    pub fn set_indexed_texture(&mut self, width: u32, height: u32, indices: &[u8]) {
        if let Some(shader) = self.palette_shader.as_mut() {
            shader.set_index_texture(&self.device, &self.queue, width, height, indices);
        }
    }

    /// Direct access to the indexed palette pipeline.
    pub fn palette_shader(&self) -> Option<&PaletteShader> {
        self.palette_shader.as_ref()
    }
}

impl Renderer for WgpuRenderer {
    fn begin_frame(&mut self) {
        self.acquired_frame = None;
        self.frame_error = None;
        self.layer_submitted = false;
        self.sprite_batch.clear();
        match self.surface.get_current_texture() {
            Ok(frame) => self.acquired_frame = Some(frame),
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                self.surface.configure(&self.device, &self.config);
                self.frame_error = Some(anyhow!(
                    "surface was lost/outdated and has been reconfigured"
                ));
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

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("cnc_frame_encoder"),
            });

        self.sprite_batch.flush(
            &self.device,
            &self.queue,
            &mut encoder,
            &view,
            self.clear_color,
            self.layer_submitted,
        );

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        self.sprite_batch.clear();
        self.layer_submitted = false;
        Ok(())
    }

    fn begin_layer(&mut self) {
        let Some(frame) = self.acquired_frame.as_ref() else {
            return;
        };
        if self.sprite_batch.is_empty() {
            return;
        }
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("cnc_layer_encoder"),
            });
        self.sprite_batch.flush(
            &self.device,
            &self.queue,
            &mut encoder,
            &view,
            self.clear_color,
            self.layer_submitted,
        );
        self.queue.submit(Some(encoder.finish()));
        self.sprite_batch.clear();
        self.layer_submitted = true;
    }

    fn draw_sprite(&mut self, sprite: &Sprite) {
        let instance = SpriteInstance {
            position: [sprite.position.x.to_f32(), sprite.position.y.to_f32()],
            size: [
                sprite.size.x.to_f32().max(1.0),
                sprite.size.y.to_f32().max(1.0),
            ],
            uv_rect: [
                sprite.uv_rect.x as f32,
                sprite.uv_rect.y as f32,
                (sprite.uv_rect.x + sprite.uv_rect.width) as f32,
                (sprite.uv_rect.y + sprite.uv_rect.height) as f32,
            ],
            color: argb_to_rgba(sprite.color),
            rotation: sprite.rotation,
            palette_index: PALETTE_NONE,
            // texture_id selects the texture array layer (0 = unit atlas
            // slot, 1 = tileset slot); TEXTURE_NONE stays flat.
            texture_id: sprite.texture_id,
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
        if let Some(shader) = self.palette_shader.as_mut() {
            shader.set_palette(&self.queue, palette);
        }
        self.current_palette = Some(palette.clone());
    }

    fn set_atlas(&mut self, width: u32, height: u32, rgba: &[u8]) -> Result<()> {
        self.set_atlas_texture(0, width, height, rgba)
    }

    fn set_atlas_texture(
        &mut self,
        texture_id: u32,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) -> Result<()> {
        self.sprite_batch.set_texture_layer(
            &self.device,
            &self.queue,
            texture_id,
            width,
            height,
            rgba,
        )?;
        self.atlas_generation += 1;
        Ok(())
    }

    fn texture_ready(&self, texture_id: u32) -> bool {
        self.sprite_batch.layer_ready(texture_id)
    }

    fn atlas_generation(&self) -> u64 {
        self.atlas_generation
    }

    fn atlas_ready(&self) -> bool {
        self.sprite_batch.texture_ready()
    }

    fn uses_sprites(&self) -> bool {
        true
    }
}

/// One uploaded RGBA8 atlas, addressed by its texture id.
struct SoftwareAtlas {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
}

pub struct SoftwareRenderer {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
    /// Per-texture-id RGBA atlases: slot 0 is the procedural unit atlas
    /// (see `set_atlas`), `TILESET_TEXTURE_ID` the RTS tileset.
    atlases: HashMap<u32, SoftwareAtlas>,
    /// Count of successful atlas uploads; see Renderer::atlas_generation.
    atlas_generation: u64,
}

impl SoftwareRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height) as usize],
            atlases: HashMap::new(),
            atlas_generation: 0,
        }
    }
    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }
}

impl Renderer for SoftwareRenderer {
    fn begin_frame(&mut self) {
        self.buffer.fill(0);
    }
    fn end_frame(&mut self) -> Result<()> {
        Ok(())
    }
    fn draw_sprite(&mut self, sprite: &Sprite) {
        let Some(atlas) = self.atlases.get(&sprite.texture_id) else {
            // Flat id (or a texture slot that was never uploaded): parity with
            // the GPU path renders nothing extra.
            return;
        };
        let size_x = sprite.size.x.to_f32().max(1.0);
        let size_y = sprite.size.y.to_f32().max(1.0);
        let origin_x = sprite.position.x.to_f32();
        let origin_y = sprite.position.y.to_f32();
        let tint = argb_to_rgba(sprite.color);
        let (uv_x, uv_y) = (sprite.uv_rect.x as f32, sprite.uv_rect.y as f32);
        let (uv_w, uv_h) = (sprite.uv_rect.width as f32, sprite.uv_rect.height as f32);
        let atlas_w = atlas.width.max(1) as usize;
        let atlas_h = atlas.height.max(1);
        for dy in 0..size_y.ceil() as i32 {
            for dx in 0..size_x.ceil() as i32 {
                let px = origin_x as i32 + dx;
                let py = origin_y as i32 + dy;
                if px < 0 || py < 0 || px >= self.width as i32 || py >= self.height as i32 {
                    continue;
                }
                // Nearest-neighbour texel lookup in atlas pixel space.
                let fu = (dx as f32 + 0.5) / size_x;
                let fv = (dy as f32 + 0.5) / size_y;
                let tx = (uv_x + uv_w * fu).clamp(0.0, atlas_w as f32 - 1.0) as usize;
                let ty = (uv_y + uv_h * fv).clamp(0.0, atlas_h as f32 - 1.0) as usize;
                let src = (ty * atlas_w + tx) * 4;
                if src + 3 >= atlas.rgba.len() {
                    continue;
                }
                let (sr, sg, sb, sa) = (
                    atlas.rgba[src] as f32 / 255.0,
                    atlas.rgba[src + 1] as f32 / 255.0,
                    atlas.rgba[src + 2] as f32 / 255.0,
                    atlas.rgba[src + 3] as f32 / 255.0,
                );
                let alpha = sa.clamp(0.0, 1.0);
                let sr = sr * tint[0];
                let sg = sg * tint[1];
                let sb = sb * tint[2];
                let dst = py as usize * self.width as usize + px as usize;
                let base = self.buffer[dst];
                let (br, bg, bb) = (
                    ((base >> 16) & 0xff) as f32 / 255.0,
                    ((base >> 8) & 0xff) as f32 / 255.0,
                    (base & 0xff) as f32 / 255.0,
                );
                let mix = |src: f32, dst: f32| src * alpha + dst * (1.0 - alpha);
                let packed = (255u32 << 24)
                    | ((mix(sr, br) * 255.0 + 0.5) as u32).min(255) << 16
                    | ((mix(sg, bg) * 255.0 + 0.5) as u32).min(255) << 8
                    | ((mix(sb, bb) * 255.0 + 0.5) as u32).min(255);
                self.buffer[dst] = packed;
            }
        }
    }
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let x = x as i32;
        let y = y as i32;
        let w = w as i32;
        let h = h as i32;
        let c = (((color[3] * 255.0) as u32) << 24)
            | (((color[0] * 255.0) as u32) << 16)
            | (((color[1] * 255.0) as u32) << 8)
            | ((color[2] * 255.0) as u32);
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                    self.buffer[(py * self.width as i32 + px) as usize] = c;
                }
            }
        }
    }
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, color: [f32; 4]) {
        draw_text_block(self, text, x, y, size, color);
    }
    fn set_camera(&mut self, _camera: &Camera) {}
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.buffer.resize((width * height) as usize, 0);
    }
    fn set_palette(&mut self, _palette: &Palette) {}
    fn set_atlas(&mut self, width: u32, height: u32, rgba: &[u8]) -> Result<()> {
        self.set_atlas_texture(0, width, height, rgba)
    }
    fn set_atlas_texture(
        &mut self,
        texture_id: u32,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) -> Result<()> {
        let expected = width as usize * height as usize * 4;
        if width == 0 || height == 0 || rgba.len() < expected {
            return Err(anyhow!(
                "sprite atlas data too small: need {expected} bytes for {width}x{height}, got {}",
                rgba.len()
            ));
        }
        self.atlases.insert(
            texture_id,
            SoftwareAtlas {
                rgba: rgba.to_vec(),
                width,
                height,
            },
        );
        self.atlas_generation += 1;
        Ok(())
    }
    fn texture_ready(&self, texture_id: u32) -> bool {
        self.atlases.contains_key(&texture_id)
    }
    fn atlas_ready(&self) -> bool {
        self.atlases.contains_key(&0)
    }
    fn atlas_generation(&self) -> u64 {
        self.atlas_generation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_core::Fixed16_16;

    #[test]
    fn software_renderer_draws_text_pixels() {
        let mut renderer = SoftwareRenderer::new(64, 16);
        renderer.draw_text("A", 0.0, 0.0, 8.0, [1.0, 1.0, 1.0, 1.0]);
        assert!(
            renderer.buffer().iter().any(|pixel| *pixel != 0),
            "expected draw_text to write non-zero pixels"
        );
    }

    #[test]
    fn software_renderer_samples_two_atlases() {
        let mut red = [0u8; 4];
        red[0] = 255;
        red[3] = 255;
        let mut blue = [0u8; 4];
        blue[2] = 255;
        blue[3] = 255;

        let mut renderer = SoftwareRenderer::new(8, 8);
        renderer
            .set_atlas(1, 1, &red)
            .expect("unit atlas uploads into slot 0");
        renderer
            .set_atlas_texture(TILESET_TEXTURE_ID, 1, 1, &blue)
            .expect("tileset atlas uploads into its own slot");
        assert!(renderer.texture_ready(0), "slot 0 should be uploaded");
        assert!(
            renderer.texture_ready(TILESET_TEXTURE_ID),
            "tileset slot should be uploaded"
        );

        // A unit sprite (texture id 0) and a terrain tile
        // (TILESET_TEXTURE_ID) side by side must sample different atlases.
        let mut unit = Sprite::new(
            0,
            Vec2::new(Fixed16_16::ZERO, Fixed16_16::ZERO),
            Vec2::new(Fixed16_16::from_f32(4.0), Fixed16_16::from_f32(4.0)),
        );
        unit.uv_rect = Rect::new(0, 0, 1, 1);
        renderer.draw_sprite(&unit);

        let mut tile = Sprite::new(
            TILESET_TEXTURE_ID,
            Vec2::new(Fixed16_16::from_f32(4.0), Fixed16_16::ZERO),
            Vec2::new(Fixed16_16::from_f32(4.0), Fixed16_16::from_f32(4.0)),
        );
        tile.uv_rect = Rect::new(0, 0, 1, 1);
        renderer.draw_sprite(&tile);

        let at = |x: usize, y: usize| renderer.buffer()[y * 8 + x];
        assert_eq!(
            at(2, 2),
            0xFF_FF_00_00,
            "unit sprite samples the red unit atlas"
        );
        assert_eq!(
            at(6, 2),
            0xFF_00_00_FF,
            "tile sprite samples the blue tileset atlas"
        );
    }
}
