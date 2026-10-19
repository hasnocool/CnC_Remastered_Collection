//! GPU palette remapping: indexed sprites through a 256-color palette with
//! player-color remap plus shadow/transparent modes.
//!
//! The main sprite path (`sprite_batch`) tints whole sprites by a single
//! palette entry (RGBA atlases). True C&C sprites are *indexed*: every pixel
//! selects a palette entry, player colors flow through a per-house remap
//! table, and shadow/transparent variants alter the output. This module
//! implements the indexed pipeline:
//!
//! - a **palette texture** (256×1 RGBA) — the actual colors;
//! - a **remap texture** (256×1 R8Uint) — per-house `PlayerRemap` table;
//! - an **index texture** (R8Uint, replaceable per draw) — sprite pixels as
//!   palette indices;
//! - a 16-byte uniform carrying the mode flags (shadow / transparent /
//!   remap-enabled);
//! - a fullscreen-quad pipeline, alpha-blended over an existing pass.
//!
//! [`sample_palette_cpu`] mirrors the shader semantics 1:1 in Rust so tests
//! pin the behavior headlessly; the WGSL is validated by naga in the same
//! style as `sprite_batch`'s.

use anyhow::Result;
use cnc_core::Palette;
use wgpu::Queue;

/// WGSL for the indexed palette pipeline (also stored as a const so other
/// pipelines can bind it standalone).
pub const PALETTE_SHADER_WGSL: &str = r#"
struct PaletteUniform {
    // bit 0: shadow mode (50% black); bit 1: transparent mode (no output);
    // bit 2: remap enabled (per-index remap texture lookup)
    mode_flags: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0)
var<uniform> palette_flags: PaletteUniform;
@group(0) @binding(1)
var palette_texture: texture_2d<f32>;
@group(0) @binding(2)
var remap_texture: texture_2d<u32>;
@group(0) @binding(3)
var index_texture: texture_2d<u32>;

const FLAG_SHADOW: u32 = 1u;
const FLAG_TRANSPARENT: u32 = 2u;
const FLAG_REMAP: u32 = 4u;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
    );
    let corner = corners[vertex_index];
    var out: VertexOutput;
    out.clip_position = vec4<f32>(
        corner * vec2<f32>(2.0, 2.0) - vec2<f32>(1.0, 1.0),
        0.0,
        1.0,
    );
    out.uv = corner;
    return out;
}

@fragment
fn fs_palette_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let flags = palette_flags.mode_flags;
    if ((flags & FLAG_TRANSPARENT) != 0u) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    // Raw palette index for this pixel (integer texel coordinates from the
    // 0..1 uv over the index texture itself).
    let size = textureDimensions(index_texture);
    let coords = vec2<i32>(in.uv * vec2<f32>(size) - vec2<f32>(0.5, 0.5));
    let raw_index = textureLoad(index_texture, coords, 0).r;
    let remapped = select(
        raw_index,
        textureLoad(remap_texture, vec2<i32>(i32(raw_index), 0), 0).r,
        (flags & FLAG_REMAP) != 0u,
    );
    var color = textureLoad(palette_texture, vec2<i32>(i32(remapped), 0), 0);
    if ((flags & FLAG_SHADOW) != 0u) {
        color = vec4<f32>(0.0, 0.0, 0.0, 0.5);
    }
    return color;
}
"#;

/// How the indexed pipeline renders a sprite variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PaletteDrawMode {
    /// Normal: palette color, opaque.
    #[default]
    Normal,
    /// C&C shadow: 50% black silhouette.
    Shadow,
    /// Hidden (the original "transparent" mode: no output at all).
    Transparent,
}

pub const MODE_FLAG_SHADOW: u32 = 1 << 0;
pub const MODE_FLAG_TRANSPARENT: u32 = 1 << 1;
pub const MODE_FLAG_REMAP: u32 = 1 << 2;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
struct PaletteUniform {
    mode_flags: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

impl PaletteUniform {
    fn new(mode_flags: u32) -> Self {
        Self {
            mode_flags,
            _pad0: 0,
            _pad1: 0,
            _pad2: 0,
        }
    }
}

fn mode_flags(mode: PaletteDrawMode, remap: bool) -> u32 {
    let mut flags = if remap { MODE_FLAG_REMAP } else { 0 };
    match mode {
        PaletteDrawMode::Normal => {}
        PaletteDrawMode::Shadow => flags |= MODE_FLAG_SHADOW,
        PaletteDrawMode::Transparent => flags |= MODE_FLAG_TRANSPARENT,
    }
    flags
}

/// CPU mirror of `fs_palette_main` — the behavior contract the shader must
/// keep (unit-tested; see `cpu_contract_*` tests).
pub fn sample_palette_cpu(
    palette: &Palette,
    index: u8,
    remap: Option<&[u8; 256]>,
    mode: PaletteDrawMode,
) -> [u8; 4] {
    match mode {
        PaletteDrawMode::Transparent => [0, 0, 0, 0],
        PaletteDrawMode::Shadow => [0, 0, 0, 128],
        PaletteDrawMode::Normal => {
            let remap_index = match remap {
                Some(table) => table[index as usize],
                None => index,
            };
            let color = palette.colors[remap_index as usize];
            [color.r, color.g, color.b, 255]
        }
    }
}

/// Indexed sprite pipeline: palette + remap textures and one fullscreen-quad
/// pass blended over whatever is already in the target.
pub struct PaletteShader {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    layout: wgpu::BindGroupLayout,
    palette_texture: wgpu::Texture,
    palette_view: wgpu::TextureView,
    remap_texture: wgpu::Texture,
    remap_view: wgpu::TextureView,
    index_texture: wgpu::Texture,
    index_view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
    mode: PaletteDrawMode,
    remap_active: bool,
}

impl PaletteShader {
    /// Build the pipeline. Remap starts as identity and the palette starts
    /// empty (black); callers upload a real palette via `set_palette`.
    pub fn new(device: &wgpu::Device, queue: &Queue, format: wgpu::TextureFormat) -> Result<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("palette_shader"),
            source: wgpu::ShaderSource::Wgsl(PALETTE_SHADER_WGSL.into()),
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("palette_shader_uniform"),
            size: std::mem::size_of::<PaletteUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("palette_shader_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });

        let palette_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("palette_shader_palette_texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let remap_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("palette_shader_remap_texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        // 1×1 start; `set_index_texture` replaces it with the real size.
        let index_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("palette_shader_index_texture"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let palette_view = palette_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let remap_view = remap_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let index_view = index_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("palette_shader_pipeline_layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("palette_shader_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_palette_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("palette_shader_bind_group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&palette_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&remap_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&index_view),
                },
            ],
        });

        let mut shader = Self {
            pipeline,
            uniform_buffer,
            layout,
            palette_texture,
            palette_view,
            remap_texture,
            remap_view,
            index_texture,
            index_view,
            bind_group,
            mode: PaletteDrawMode::Normal,
            remap_active: false,
        };
        // Identity remap keeps the shader remap flag off.
        shader.set_remap(queue, cnc_core::PlayerRemap::identity().map);
        Ok(shader)
    }

    fn make_bind_group(
        &self,
        device: &wgpu::Device,
        index_view: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("palette_shader_bind_group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.palette_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&self.remap_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(index_view),
                },
            ],
        })
    }

    fn write_uniform(&self, queue: &Queue) {
        let uniform = PaletteUniform::new(mode_flags(self.mode, self.remap_active));
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniform));
    }

    /// Upload a 256-entry RGBA palette texture.
    pub fn set_palette(&mut self, queue: &Queue, palette: &Palette) {
        let mut data = Vec::with_capacity(256 * 4);
        for color in &palette.colors {
            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
            data.push(255);
        }
        queue.write_texture(
            row_copy(&self.palette_texture),
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(256 * 4),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
    }

    /// Upload a per-house remap table (`PlayerRemap::map`). A table equal to
    /// the identity keeps the remap shader flag off.
    pub fn set_remap(&mut self, queue: &Queue, map: [u8; 256]) {
        self.remap_active = map != cnc_core::PlayerRemap::identity().map;
        queue.write_texture(
            row_copy(&self.remap_texture),
            &map,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(256),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 256,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        self.write_uniform(queue);
    }

    /// Replace the 1-byte-per-pixel indexed sprite texture (row-major).
    pub fn set_index_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &Queue,
        width: u32,
        height: u32,
        indices: &[u8],
    ) {
        debug_assert!(width > 0 && height > 0);
        let width = width.max(1);
        let height = height.max(1);
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("palette_shader_index_texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let needed = width as usize * height as usize;
        let mut data = Vec::with_capacity(needed);
        data.extend_from_slice(&indices[..indices.len().min(needed)]);
        data.resize(needed, 0);
        queue.write_texture(
            row_copy(&texture),
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.bind_group = self.make_bind_group(device, &view);
        self.index_texture = texture;
        self.index_view = view;
    }

    /// Switch render mode (shadow / transparent / normal) by rewriting the
    /// uniform.
    pub fn set_mode(&mut self, queue: &Queue, mode: PaletteDrawMode) {
        self.mode = mode;
        self.write_uniform(queue);
    }

    /// Record the indexed pass into an existing encoder (caller chooses when
    /// it runs relative to the sprite batch pass).
    pub fn render<'pass>(&self, pass: &mut wgpu::RenderPass<'pass>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..6, 0..1);
    }
}

fn row_copy(texture: &wgpu::Texture) -> wgpu::ImageCopyTexture<'_> {
    wgpu::ImageCopyTexture {
        texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_is_valid_wgsl() {
        let module = naga::front::wgsl::parse_str(PALETTE_SHADER_WGSL)
            .expect("palette shader should parse as WGSL");
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        validator
            .validate(&module)
            .expect("palette shader should pass naga validation");
    }

    #[test]
    fn mode_flags_bits() {
        assert_eq!(mode_flags(PaletteDrawMode::Normal, false), 0);
        assert_eq!(mode_flags(PaletteDrawMode::Shadow, false), MODE_FLAG_SHADOW);
        assert_eq!(
            mode_flags(PaletteDrawMode::Transparent, true),
            MODE_FLAG_TRANSPARENT | MODE_FLAG_REMAP
        );
    }

    #[test]
    fn cpu_contract_normal_shadow_transparent() {
        let mut palette = Palette::grayscale();
        palette.set(
            cnc_core::PaletteIndex(7),
            cnc_core::RGB888::new(250, 10, 10),
        );

        // No remap.
        assert_eq!(
            sample_palette_cpu(&palette, 7, None, PaletteDrawMode::Normal),
            [250, 10, 10, 255]
        );
        // Shadow and transparent ignore the palette entry entirely.
        assert_eq!(
            sample_palette_cpu(&palette, 7, None, PaletteDrawMode::Shadow),
            [0, 0, 0, 128]
        );
        assert_eq!(
            sample_palette_cpu(&palette, 7, None, PaletteDrawMode::Transparent),
            [0, 0, 0, 0]
        );
    }

    #[test]
    fn cpu_contract_with_player_remap() {
        let palette = Palette::grayscale();
        // player_color(1) maps indices 0..16 -> 16..32.
        let remap = cnc_core::PlayerRemap::player_color(1);
        assert_eq!(
            sample_palette_cpu(&palette, 4, Some(&remap.map), PaletteDrawMode::Normal),
            [20, 20, 20, 255]
        );
        assert_eq!(
            sample_palette_cpu(&palette, 200, Some(&remap.map), PaletteDrawMode::Normal),
            [200, 200, 200, 255],
            "indices outside the remap range pass through"
        );
    }
}
