// Sprite Batch Shader with Palette Support

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct PaletteUniform {
    colors: array<vec4<f32>, 256>,
};

@group(1) @binding(0)
var<uniform> palette: PaletteUniform;

@group(2) @binding(0)
var atlas: texture_2d<f32>;

@group(2) @binding(1)
var sampler: sampler;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
    @location(3) palette_index: u32,
    @location(4) texture_id: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) palette_index: u32,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = camera.view_proj * vec4<f32>(input.position, 0.0, 1.0);
    output.uv = input.uv;
    output.color = input.color;
    output.palette_index = input.palette_index;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let atlas_color = textureSample(atlas, sampler, input.uv);
    if input.palette_index > 0u {
        let palette_color = palette.colors[input.palette_index];
        return vec4<f32>(palette_color.rgb, atlas_color.a * input.color.a);
    }
    return atlas_color * input.color;
}
