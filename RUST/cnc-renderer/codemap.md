# RUST/cnc-renderer/

## Responsibility
WGPU-based renderer for the C&C Rust port. Implements palette-indexed sprite rendering (faithful to Westwood 8-bit palette model), software fallback renderer, camera system, sprite batching, and immediate-mode UI primitives. Dual backend: WGPU (GPU) and pixels (CPU software).

## Design Patterns
- Palette-Indexed Rendering: Sprites store 8-bit palette indices; PaletteShader resolves to RGBA at draw time (supports remap, shadow, transparency flags)
- Dual Backend: WgpuRenderer (GPU) and SoftwareRenderer (CPU) implement common Renderer trait
- Sprite Batching: SpriteBatch accumulates SpriteInstance (pos, size, palette_id, flags) to single draw call
- Camera: Orthographic camera with zoom, pan, world<->screen transforms
- Immediate UI: ui module provides button, panel, console, widget_at for debug/tools UI
- Texture Atlas: Single TILESET_TEXTURE_ID atlas binds all terrain/unit sprites

## Data & Control Flow
1. Init: Renderer::new() / WgpuRenderer::new() / SoftwareRenderer::new() to creates pipeline, atlas texture
2. Load Assets: renderer.load_atlas(atlas_data) uploads palette + sprite sheets to GPU
3. Frame Begin: renderer.begin_frame(camera) - clears, sets view/projection
4. Batch Submit: sprite_batch.add(SpriteInstance) - accumulates sprites
5. Draw: renderer.draw_batch(&sprite_batch) - single indexed draw call
6. UI Overlay: ui::button(), ui::console(), ui::panel() - immediate mode, drawn last
7. Present: renderer.end_frame() - swap/present

## Public API
- Camera: Camera::new(), world_to_screen(), screen_to_world(), zoom, pan
- Renderer Trait: begin_frame(), draw_batch(), load_atlas(), end_frame()
- WgpuRenderer: new(), GPU pipeline, bind groups
- SoftwareRenderer: new(), CPU rasterization via pixels crate
- PaletteShader: PALETTE_SHADER_WGSL, MODE_FLAG_REMAP, MODE_FLAG_SHADOW, MODE_FLAG_TRANSPARENT, sample_palette_cpu()
- SpriteBatch: new(), add(instance), clear(), len()
- SpriteInstance: position, size, palette_id, flags, texture_id
- UI: button(), panel(), console(), widget_at()

## Integration Points
- Dependencies: cnc-core, wgpu, winit, pixels, image, bytemuck, bitflags, ahash, parking_lot, once_cell, font8x8, naga (dev), pollster (dev), criterion (dev)
- Consumed by: cnc-tiberian-dawn, cnc-red-alert (game rendering), cnc-classic (classic mode), cnc-map-editor (editor viewport), cnc-console (UI)
- Asset Pipeline: Consumes atlases from cnc-asset, palettes from cnc-core

## Key Files
- src/lib.rs - Trait definitions, re-exports
- src/renderer.rs - Renderer trait, WgpuRenderer, SoftwareRenderer
- src/sprite_batch.rs - SpriteBatch, SpriteInstance, batching logic
- src/camera.rs - Camera, transform math
- src/palette_shader.rs - WGSL shader, palette sampling, mode flags
- src/ui.rs - Immediate-mode UI primitives

