//! End-to-end GPU smoke tests: create a real device, render through
//! SpriteBatch, read the framebuffer back and check pixels.

use cnc_renderer::{Camera, SpriteBatch, SpriteInstance};

fn try_device() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))?;
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("cnc-renderer-gpu-smoke"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    ))
    .ok()?;
    Some((device, queue))
}

/// Render the batch into a fresh RGBA8 target and return the mapped pixels.
fn render_and_readback(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    batch: &mut SpriteBatch,
    width: u32,
    height: u32,
) -> Vec<u8> {
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("gpu-smoke-target"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let bytes_per_row = width * 4; // 256 for a 64px wide target: already aligned
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("gpu-smoke-readback"),
        size: (bytes_per_row * height) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("gpu-smoke-encoder"),
    });
    batch.flush(device, queue, &mut encoder, &view, wgpu::Color::BLACK, false);
    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &readback,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
    );
    queue.submit(Some(encoder.finish()));

    let slice = readback.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = sender.send(result);
    });
    device.poll(wgpu::Maintain::Wait);
    receiver.recv().expect("map callback fired").expect("buffer map succeeded");

    let data = slice.get_mapped_range().to_vec();
    readback.unmap();
    data
}

fn pixel_at(data: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
    let index = ((y * width + x) * 4) as usize;
    [data[index], data[index + 1], data[index + 2], data[index + 3]]
}

fn quad_batch(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32) -> SpriteBatch {
    let mut batch = SpriteBatch::new(device, queue, wgpu::TextureFormat::Rgba8UnormSrgb)
        .expect("create sprite batch");
    let mut camera = Camera::default();
    camera.set_viewport(width, height);
    batch.set_camera(queue, &camera);
    batch
}

#[test]
fn sprite_batch_renders_a_red_quad_on_gpu() {
    let Some((device, queue)) = try_device() else {
        eprintln!("skipping GPU smoke test: no wgpu adapter available");
        return;
    };

    let (width, height) = (64u32, 64u32);
    let mut batch = quad_batch(&device, &queue, width, height);
    batch.add(SpriteInstance {
        position: [-16.0, -16.0],
        size: [32.0, 32.0],
        uv_rect: [0.0, 0.0, 1.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
        rotation: 0.0,
        palette_index: u32::MAX,
        texture_id: 0,
        z_order: 0,
    });

    let data = render_and_readback(&device, &queue, &mut batch, width, height);
    let center = pixel_at(&data, width, width / 2, height / 2);
    let corner = pixel_at(&data, width, 0, 0);
    assert!(center[0] > 200 && center[1] < 40 && center[2] < 40, "expected red center, got {:?}", center);
    assert!(corner[0] < 16 && corner[1] < 16 && corner[2] < 16, "expected black corner, got {:?}", corner);
}

#[test]
fn sprite_atlas_uv_selection_is_sampled_on_gpu() {
    let Some((device, queue)) = try_device() else {
        eprintln!("skipping GPU smoke test: no wgpu adapter available");
        return;
    };

    let (width, height) = (64u32, 64u32);
    let mut batch = quad_batch(&device, &queue, width, height);

    // 2x1 atlas: texel 0 red, texel 1 green.
    let atlas = vec![255u8, 0, 0, 255, 0, 255, 0, 255];
    batch
        .set_texture(&device, &queue, 2, 1, &atlas)
        .expect("upload atlas");

    // Sample atlas pixel 1 (green); uv_rect is in atlas pixels.
    batch.add(SpriteInstance {
        position: [-16.0, -16.0],
        size: [32.0, 32.0],
        uv_rect: [1.0, 0.0, 2.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
        rotation: 0.0,
        palette_index: u32::MAX,
        texture_id: 1,
        z_order: 0,
    });

    let data = render_and_readback(&device, &queue, &mut batch, width, height);
    let center = pixel_at(&data, width, width / 2, height / 2);
    assert!(center[1] > 200 && center[0] < 40 && center[2] < 40, "expected green atlas texel, got {:?}", center);
}
