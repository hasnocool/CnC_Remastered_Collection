//! GPU sprite batching for wgpu 22
//!
//! Instanced quad rendering: one instance per sprite, six vertices generated in
//! the vertex shader. A sprite texture atlas is sampled when texture_id is
//! non-zero, and a 256-entry palette texture can tint instances by index.

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
    atlas_size: [f32; 2],
    _pad: [f32; 2],
}

/// Sentinel palette index meaning "use the instance color directly".
pub const PALETTE_NONE: u32 = u32::MAX;

/// texture_id value meaning "draw the instance color without sampling a texture".
pub const TEXTURE_NONE: u32 = 0;

const INITIAL_INSTANCE_CAPACITY: usize = 1024;
const COPY_ROW_ALIGNMENT: usize = 256;

const SHADER: &str = r#"
struct FrameUniform {
    view_proj: mat4x4<f32>,
    atlas_size: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> frame: FrameUniform;

@group(1) @binding(0)
var palette_texture: texture_2d<f32>;
@group(1) @binding(1)
var palette_sampler: sampler;

@group(2) @binding(0)
var sprite_texture: texture_2d<f32>;
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

    let uv_pixels = mix(instance.uv_rect.xy, instance.uv_rect.zw, corner);
    let atlas = max(frame.atlas_size, vec2<f32>(1.0, 1.0));

    var out: VertexOutput;
    out.clip_position = frame.view_proj * vec4<f32>(world, 0.0, 1.0);
    out.color = instance.color;
    out.palette_index = instance.palette_index;
    out.uv = uv_pixels / atlas;
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
    if (in.texture_id != 0u) {
        tex = textureSample(sprite_texture, sprite_sampler, in.uv);
    }

    return base * tex * in.color;
}
"#;

const INSTANCE_ATTRIBUTES: [wgpu::VertexAttribute; 8] = [
    wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 },
    wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x2 },
    wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
    wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32x4 },
    wgpu::VertexAttribute { offset: 48, shader_location: 4, format: wgpu::VertexFormat::Float32 },
    wgpu::VertexAttribute { offset: 52, shader_location: 5, format: wgpu::VertexFormat::Uint32 },
    wgpu::VertexAttribute { offset: 56, shader_location: 6, format: wgpu::VertexFormat::Uint32 },
    wgpu::VertexAttribute { offset: 60, shader_location: 7, format: wgpu::VertexFormat::Sint32 },
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
    let zoom = if camera.zoom.to_f32() == 0.0 { 1.0 } else { camera.zoom.to_f32() };
    let half_w = (view_w / (2.0 * zoom)).max(1.0e-6);
    let half_h = (view_h / (2.0 * zoom)).max(1.0e-6);
    let cx = camera.position.x.to_f32();
    let cy = camera.position.y.to_f32();

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
    atlas_size: [f32; 2],
    camera: Camera,
    instance_buffer: wgpu::Buffer,
    instance_capacity: usize,
    pending: Vec<SpriteInstance>,
    draw_calls: u32,
    vertices_count: u32,
}

impl SpriteBatch {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Result<Self> {
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
            size: wgpu::Extent3d { width: 256, height: 1, depth_or_array_layers: 1 },
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

        let white_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("sprite_default_white_texture"),
            size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &white_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &[255u8, 255, 255, 255],
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        );
        let white_view = white_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite_texture_bind_group"),
            layout: &texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&white_view),
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
            wgpu::Extent3d { width: 256, height: 1, depth_or_array_layers: 1 },
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
            atlas_size: [1.0, 1.0],
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

    pub fn atlas_size(&self) -> [f32; 2] {
        self.atlas_size
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
            wgpu::Extent3d { width: 256, height: 1, depth_or_array_layers: 1 },
        );
    }

    /// Replace the sprite atlas sampled by instances whose texture_id is not 0.
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

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("sprite_atlas_texture"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let unpadded_row = width as usize * 4;
        let aligned_row = ((unpadded_row + COPY_ROW_ALIGNMENT - 1) / COPY_ROW_ALIGNMENT) * COPY_ROW_ALIGNMENT;
        let mut upload = vec![0u8; aligned_row * height as usize];
        for row in 0..height as usize {
            let src = row * unpadded_row;
            let dst = row * aligned_row;
            upload[dst..dst + unpadded_row].copy_from_slice(&rgba[src..src + unpadded_row]);
        }

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &upload,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(aligned_row as u32),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
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

        self.texture_bind_group = bind_group;
        self.atlas_size = [width as f32, height as f32];
        self.write_uniform(queue);
        Ok(())
    }

    pub fn set_camera(&mut self, queue: &wgpu::Queue, camera: &Camera) {
        self.camera = *camera;
        self.write_uniform(queue);
    }

    fn write_uniform(&mut self, queue: &wgpu::Queue) {
        let uniform = FrameUniform {
            view_proj: ortho_matrix(&self.camera),
            atlas_size: self.atlas_size,
            _pad: [0.0, 0.0],
        };
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(&uniform));
    }

    pub fn stats(&self) -> (u32, u32) {
        (self.draw_calls, self.vertices_count)
    }

    /// Record the batch into an existing encoder, clearing the target first.
    pub fn flush(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clear: wgpu::Color,
    ) {
        self.pending.sort_by_key(|instance| instance.z_order);
        let count = self.pending.len();

        if count > self.instance_capacity {
            self.instance_capacity = count.next_power_of_two().max(INITIAL_INSTANCE_CAPACITY);
            self.instance_buffer = create_instance_buffer(device, self.instance_capacity);
        }

        if count > 0 {
            queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.pending));
        }

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("sprite_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shader_is_valid_wgsl() {
        let module = naga::front::wgsl::parse_str(SHADER).expect("sprite shader should parse as WGSL");
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        validator.validate(&module).expect("sprite shader should pass naga validation");
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
}
