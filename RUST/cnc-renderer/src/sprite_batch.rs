//! GPU sprite batching for wgpu 22
//!
//! Instanced quad rendering: one instance per sprite, six vertices generated in
//! the vertex shader. Instances carry a texture index that selects a layer of
//! a 2D texture array: layer 0 is the procedural unit/building atlas
//! (texture slot 0), layer 1 is the RTS tileset atlas
//! (`TILESET_TEXTURE_ID`). [`TEXTURE_NONE`] draws the instance color without
//! sampling. A 256-entry palette texture can tint instances by index.

use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use cnc_core::Palette;

use crate::camera::Camera;

/// Per-sprite data uploaded to the GPU instance buffer.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SpriteInstance {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub uv_rect: [f32; 4],
    pub color: [f32; 4],
    pub rotation: f32,
    pub palette_index: u32,
    pub texture_id: u32,
    pub z_order: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct FrameUniform {
    view_proj: [[f32; 4]; 4],
    /// Atlas pixel dimensions per texture slot (slot id = array index).
    atlas_sizes: [[f32; 4]; MAX_ATLAS_TEXTURES],
}

/// Sentinel palette index meaning "use the instance color directly".
pub const PALETTE_NONE: u32 = u32::MAX;

/// texture_id value meaning "draw the instance color without sampling a
/// texture". Any other value indexes the atlas texture array (layer 0 = unit
/// atlas slot, layer 1 = tileset slot; see [`crate::renderer::TILESET_TEXTURE_ID`]).
pub const TEXTURE_NONE: u32 = u32::MAX;

/// Number of atlas texture slots carried in one batch. Slot 0 is the
/// procedural unit/building atlas (`SpriteBatch::set_texture`), slot 1 is the
/// RTS tileset atlas (see `TILESET_TEXTURE_ID` in [`crate::renderer`]).
pub const MAX_ATLAS_TEXTURES: usize = 2;

const INITIAL_INSTANCE_CAPACITY: usize = 1024;
const COPY_ROW_ALIGNMENT: usize = 256;

const SHADER: &str = r#"
struct FrameUniform {
    view_proj: mat4x4<f32>,
    // Atlas pixel dimensions per texture slot; slot id = array index.
    atlas_sizes: array<vec4<f32>, 2>,
};

@group(0) @binding(0)
var<uniform> frame: FrameUniform;

@group(1) @binding(0)
var palette_texture: texture_2d<f32>;
@group(1) @binding(1)
var palette_sampler: sampler;

@group(2) @binding(0)
var sprite_textures: texture_2d_array<f32>;
@group(2) @binding(1)
var sprite_sampler: sampler;

struct InstanceInput {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) uv_rect: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) rotation: f32,
    @location(5) palette_index: u32,
    @location(6) texture_id: u32,
    @location(7) z_order: i32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(flat) palette_index: u32,
    @location(2) uv: vec2<f32>,
    @location(3) @interpolate(flat) texture_id: u32,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: InstanceInput,
) -> VertexOutput {
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
    );

    let corner = corners[vertex_index];
    let local = (corner - vec2<f32>(0.5, 0.5)) * instance.size;
    let c = cos(instance.rotation);
    let s = sin(instance.rotation);
    let rotated = vec2<f32>(local.x * c - local.y * s, local.x * s + local.y * c);
    // position is the top-left corner; rotation happens around the center.
    let world = instance.position + instance.size * 0.5 + rotated;

    // Texture index = texture array layer. Flat instances (TEXTURE_NONE) are
    // clamped to a valid layer here and skipped in the fragment shader.
    let layer = min(instance.texture_id, 1u);
    let atlas = max(frame.atlas_sizes[layer].xy, vec2<f32>(1.0, 1.0));
    // Inset the UV rect by half a texel on every side. Terrain tiles are drawn
    // at cell size + 0.5 to hide seams between neighbouring quads; without the
    // inset the outermost fragment sampled the first texel column of the NEXT
    // atlas tile, leaving a 1px sliver of foreign art on tile edges.
    let uv0 = (instance.uv_rect.xy + vec2<f32>(0.5, 0.5)) / atlas;
    let uv1 = max(instance.uv_rect.zw - vec2<f32>(0.5, 0.5), instance.uv_rect.xy) / atlas;
    let uv_pixels = mix(uv0, uv1, corner);
    var out: VertexOutput;
    out.clip_position = frame.view_proj * vec4<f32>(world, 0.0, 1.0);
    out.color = instance.color;
    out.palette_index = instance.palette_index;
    out.uv = uv_pixels;
    out.texture_id = instance.texture_id;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var base = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    if (in.palette_index != 4294967295u) {
        let u = (f32(in.palette_index) + 0.5) / 256.0;
        base = textureSample(palette_texture, palette_sampler, vec2<f32>(u, 0.5));
    }

    var tex = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    // 4294967295u is TEXTURE_NONE: draw the instance color without sampling.
    // textureSampleLevel keeps sampling well-defined for the dynamic layer
    // index; the atlases are mip-less so level 0 matches a plain sample.
    if (in.texture_id != 4294967295u) {
        tex = textureSampleLevel(sprite_textures, sprite_sampler, in.uv, i32(in.texture_id), 0.0);
    }

    return base * tex * in.color;
}
"#;

const INSTANCE_ATTRIBUTES: [wgpu::VertexAttribute; 8] = [
    wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Float32x2,
    },
    wgpu::VertexAttribute {
        offset: 8,
        shader_location: 1,
        format: wgpu::VertexFormat::Float32x2,
    },
    wgpu::VertexAttribute {
        offset: 16,
        shader_location: 2,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 32,
        shader_location: 3,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 48,
        shader_location: 4,
        format: wgpu::VertexFormat::Float32,
    },
    wgpu::VertexAttribute {
        offset: 52,
        shader_location: 5,
        format: wgpu::VertexFormat::Uint32,
    },
    wgpu::VertexAttribute {
        offset: 56,
        shader_location: 6,
        format: wgpu::VertexFormat::Uint32,
    },
    wgpu::VertexAttribute {
        offset: 60,
        shader_location: 7,
        format: wgpu::VertexFormat::Sint32,
    },
];

fn create_instance_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
    let size = (capacity.max(1) * std::mem::size_of::<SpriteInstance>()) as u64;
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sprite_instance_buffer"),
        size,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn ortho_matrix(camera: &Camera) -> [[f32; 4]; 4] {
    let view_w = camera.viewport_width.max(1) as f32;
    let view_h = camera.viewport_height.max(1) as f32;
    let zoom = if camera.zoom.to_f32() == 0.0 {
        1.0
    } else {
        camera.zoom.to_f32()
    };
    let half_w = (view_w / (2.0 * zoom)).max(1.0e-6);
    let half_h = (view_h / (2.0 * zoom)).max(1.0e-6);
    // Shake flows through effective_position; picking (screen_to_world) uses
    // the raw position, so jitter is visual-only.
    let effective = camera.effective_position();
    let cx = effective.x.to_f32();
    let cy = effective.y.to_f32();

    let sx = 1.0 / half_w;
    let sy = -1.0 / half_h;

    // Column-major 4x4 for WGSL: world -> clip.
    [
        [sx, 0.0, 0.0, 0.0],
        [0.0, sy, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-cx * sx, -cy * sy, 0.0, 1.0],
    ]
}

/// Per-slot uploaded atlas kept so the texture array can be rebuilt when a
/// later slot grows the shared layer dimensions.
struct LayerData {
    rgba: Vec<u8>,
    width: u32,
    height: u32,
}

/// Batches sprite instances and draws them in a single instanced draw call.
pub struct SpriteBatch {
    pipeline: wgpu::RenderPipeline,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    palette_texture: wgpu::Texture,
    palette_bind_group: wgpu::BindGroup,
    texture_layout: wgpu::BindGroupLayout,
    texture_sampler: wgpu::Sampler,
    texture_bind_group: wgpu::BindGroup,
    /// Per-slot uploaded atlases. Slot 0 is the procedural unit atlas, slot 1
    /// is the RTS tileset atlas (see `TILESET_TEXTURE_ID` in [`crate::renderer`]).
    layers: [Option<LayerData>; MAX_ATLAS_TEXTURES],
    /// Atlas pixel dimensions per slot, mirrored into the frame uniform.
    atlas_sizes: [[f32; 4]; MAX_ATLAS_TEXTURES],
    camera: Camera,
    instance_buffer: wgpu::Buffer,
    instance_capacity: usize,
    pending: Vec<SpriteInstance>,
    draw_calls: u32,
    vertices_count: u32,
}

impl SpriteBatch {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
    ) -> Result<Self> {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sprite_shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sprite_frame_uniform"),
            size: std::mem::size_of::<FrameUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprite_frame_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite_frame_bind_group"),
            layout: &camera_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let palette_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("sprite_palette_texture"),
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
        let palette_view = palette_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let palette_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sprite_palette_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let palette_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprite_palette_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let palette_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite_palette_bind_group"),
            layout: &palette_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&palette_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&palette_sampler),
                },
            ],
        });

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprite_texture_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sprite_atlas_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Texture array backing all atlas slots. Every layer starts white so
        // sprites sampling a slot without an uploaded atlas (or TEXTURE_NONE,
        // skipped in the shader) keep the old flat-color fallback behavior.
        let atlas_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("sprite_atlas_texture_array"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: MAX_ATLAS_TEXTURES as u32,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let white_row = vec![255u8; 4];
        for slot in 0..MAX_ATLAS_TEXTURES {
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &atlas_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: slot as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &white_row,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4),
                    rows_per_image: Some(1),
                },
                wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
            );
        }
        let atlas_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite_texture_bind_group"),
            layout: &texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sprite_pipeline_layout"),
            bind_group_layouts: &[&camera_layout, &palette_layout, &texture_layout],
            push_constant_ranges: &[],
        });

        let instance_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteInstance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &INSTANCE_ATTRIBUTES,
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sprite_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[instance_layout],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
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

        let instance_capacity = INITIAL_INSTANCE_CAPACITY;
        let instance_buffer = create_instance_buffer(device, instance_capacity);

        let white = vec![255u8; 256 * 4];
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &palette_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &white,
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

        let mut batch = Self {
            pipeline,
            camera_buffer,
            camera_bind_group,
            palette_texture,
            palette_bind_group,
            texture_layout,
            texture_sampler,
            texture_bind_group,
            layers: std::array::from_fn(|_| None),
            atlas_sizes: [[1.0, 1.0, 0.0, 0.0]; MAX_ATLAS_TEXTURES],
            camera: Camera::default(),
            instance_buffer,
            instance_capacity,
            pending: Vec::new(),
            draw_calls: 0,
            vertices_count: 0,
        };
        batch.set_camera(queue, &Camera::default());
        Ok(batch)
    }

    pub fn add(&mut self, instance: SpriteInstance) {
        self.pending.push(instance);
    }

    pub fn clear(&mut self) {
        self.pending.clear();
        self.draw_calls = 0;
        self.vertices_count = 0;
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Upload a 256-entry palette used by instances with a palette index.
    pub fn set_palette(&mut self, queue: &wgpu::Queue, palette: &Palette) {
        let colors = palette.to_argb8888(255);
        let mut data = Vec::with_capacity(256 * 4);
        for color in colors {
            data.push(color.r());
            data.push(color.g());
            data.push(color.b());
            data.push(color.a());
        }
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.palette_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
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

    /// True once an atlas has been uploaded into slot 0 via `set_texture`.
    pub fn texture_ready(&self) -> bool {
        self.layers[0].is_some()
    }

    /// True when `texture_id` has an uploaded atlas layer.
    pub fn layer_ready(&self, texture_id: u32) -> bool {
        (texture_id as usize) < MAX_ATLAS_TEXTURES && self.layers[texture_id as usize].is_some()
    }

    /// Replace the slot-0 sprite atlas (the procedural unit/building atlas).
    ///
    /// uv_rect values on SpriteInstance are expressed in atlas pixels.
    pub fn set_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) -> Result<()> {
        self.set_texture_layer(device, queue, 0, width, height, rgba)
    }

    /// Upload an RGBA8 atlas into texture slot `texture_id`, replacing
    /// whatever was bound there before.
    ///
    /// uv_rect values on SpriteInstance are expressed in atlas pixels of the
    /// slot they sample. All uploaded slots share one texture array sized to
    /// the largest atlas seen so far, so a later upload rebuilds it.
    pub fn set_texture_layer(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture_id: u32,
        width: u32,
        height: u32,
        rgba: &[u8],
    ) -> Result<()> {
        let slot = texture_id as usize;
        if slot >= MAX_ATLAS_TEXTURES {
            return Err(anyhow::anyhow!(
                "atlas texture id {texture_id} unsupported: this batch carries {MAX_ATLAS_TEXTURES} slots"
            ));
        }
        if width == 0 || height == 0 {
            return Ok(());
        }
        let expected = width as usize * height as usize * 4;
        if rgba.len() < expected {
            return Err(anyhow::anyhow!(
                "sprite atlas data too small: need {} bytes, got {}",
                expected,
                rgba.len()
            ));
        }
        self.layers[slot] = Some(LayerData {
            rgba: rgba.to_vec(),
            width,
            height,
        });
        self.atlas_sizes[slot] = [width as f32, height as f32, 0.0, 0.0];
        self.rebuild_atlas(device, queue);
        Ok(())
    }

    /// Recreate the atlas texture array (sized to the largest uploaded
    /// layer), white-filling any slot without real atlas data.
    fn rebuild_atlas(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let width = self
            .layers
            .iter()
            .filter_map(|layer| layer.as_ref())
            .map(|layer| layer.width)
            .max()
            .unwrap_or(1);
        let height = self
            .layers
            .iter()
            .filter_map(|layer| layer.as_ref())
            .map(|layer| layer.height)
            .max()
            .unwrap_or(1);

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("sprite_atlas_texture_array"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: MAX_ATLAS_TEXTURES as u32,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // White-fill layers without an uploaded atlas so sampling them keeps
        // the flat-color fallback behavior.
        let white = vec![255u8; width as usize * height as usize * 4];
        for slot in 0..MAX_ATLAS_TEXTURES {
            if self.layers[slot].is_some() {
                continue;
            }
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: slot as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &white,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(width * 4),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }

        let unpadded_row = width as usize * 4;
        let aligned_row = unpadded_row.div_ceil(COPY_ROW_ALIGNMENT) * COPY_ROW_ALIGNMENT;
        for slot in 0..MAX_ATLAS_TEXTURES {
            let Some(layer) = self.layers[slot].as_ref() else {
                continue;
            };
            let mut upload = vec![0u8; aligned_row * height as usize];
            for row in 0..layer.height as usize {
                let src = row * layer.width as usize * 4;
                let dst = row * aligned_row;
                upload[dst..dst + layer.width as usize * 4]
                    .copy_from_slice(&layer.rgba[src..src + layer.width as usize * 4]);
            }
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: slot as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &upload,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(aligned_row as u32),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite_texture_bind_group"),
            layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture_sampler),
                },
            ],
        });
        self.write_uniform(queue);
    }

    pub fn set_camera(&mut self, queue: &wgpu::Queue, camera: &Camera) {
        self.camera = *camera;
        self.write_uniform(queue);
    }

    fn write_uniform(&mut self, queue: &wgpu::Queue) {
        let uniform = FrameUniform {
            view_proj: ortho_matrix(&self.camera),
            atlas_sizes: self.atlas_sizes,
        };
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(&uniform));
    }

    pub fn stats(&self) -> (u32, u32) {
        (self.draw_calls, self.vertices_count)
    }

    /// Record the batch into an existing encoder.
    ///
    /// The first layer of a frame clears the target; later layers pass
    /// `load = true` so earlier layers survive underneath.
    pub fn flush(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clear: wgpu::Color,
        load: bool,
    ) {
        sort_instances_for_draw(&mut self.pending);
        let count = self.pending.len();

        if count > self.instance_capacity {
            self.instance_capacity = count.next_power_of_two().max(INITIAL_INSTANCE_CAPACITY);
            self.instance_buffer = create_instance_buffer(device, self.instance_capacity);
        }

        if count > 0 {
            queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&self.pending),
            );
        }

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("sprite_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if load {
                        wgpu::LoadOp::Load
                    } else {
                        wgpu::LoadOp::Clear(clear)
                    },
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if count > 0 {
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.camera_bind_group, &[]);
            pass.set_bind_group(1, &self.palette_bind_group, &[]);
            pass.set_bind_group(2, &self.texture_bind_group, &[]);
            pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
            pass.draw(0..6, 0..count as u32);
            self.draw_calls += 1;
            self.vertices_count += count as u32 * 6;
        }
    }
}

/// Painter's-order sort: ascending `z_order`, stable so sprites sharing a
/// z level keep their submission order.
fn sort_instances_for_draw(instances: &mut [SpriteInstance]) {
    instances.sort_by_key(|instance| instance.z_order);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_is_valid_wgsl() {
        let module =
            naga::front::wgsl::parse_str(SHADER).expect("sprite shader should parse as WGSL");
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        validator
            .validate(&module)
            .expect("sprite shader should pass naga validation");
    }

    #[test]
    fn instance_layout_matches_struct_size() {
        assert_eq!(std::mem::size_of::<SpriteInstance>(), 64);
        let max_end = INSTANCE_ATTRIBUTES
            .iter()
            .map(|attribute| {
                let width = match attribute.format {
                    wgpu::VertexFormat::Float32 => 4,
                    wgpu::VertexFormat::Float32x2 => 8,
                    wgpu::VertexFormat::Float32x4 => 16,
                    wgpu::VertexFormat::Uint32 => 4,
                    wgpu::VertexFormat::Sint32 => 4,
                    other => panic!("unexpected vertex format in instance layout: {:?}", other),
                };
                attribute.offset + width
            })
            .max()
            .unwrap();
        assert_eq!(max_end, 64);
    }

    #[test]
    fn z_order_sort_is_stable_and_ascending() {
        let mut batch = [
            sprite_at_z(0, 1),
            sprite_at_z(-3, 2),
            sprite_at_z(0, 3),
            sprite_at_z(9, 4),
            sprite_at_z(0, 5),
        ];
        sort_instances_for_draw(&mut batch);
        assert_eq!(batch[0].z_order, -3);
        // Stable sort: sprites that share a z level keep submission order
        // (asserted via their distinct rotation values used as tags).
        assert_eq!(batch[1].rotation, 1.0);
        assert_eq!(batch[2].rotation, 3.0);
        assert_eq!(batch[3].rotation, 5.0);
        assert_eq!(batch[3].z_order, 0);
        assert_eq!(batch[4].z_order, 9);
    }

    fn sprite_at_z(z_order: i32, tag: u32) -> SpriteInstance {
        SpriteInstance {
            position: [0.0, 0.0],
            size: [1.0, 1.0],
            uv_rect: [0.0, 0.0, 1.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
            rotation: tag as f32,
            palette_index: PALETTE_NONE,
            texture_id: TEXTURE_NONE,
            z_order,
        }
    }
}
