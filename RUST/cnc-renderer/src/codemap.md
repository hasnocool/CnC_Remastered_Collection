# RUST/cnc-renderer/src/

## Responsibility

Three real modules (`camera`, `renderer`, `sprite_batch`) plus two stubs (`ui.rs`, `palette_shader.rs` — 6 lines each); `lib.rs` re-exports `Camera`, `Renderer`, `Sprite`, `WgpuRenderer`, `SoftwareRenderer`, `TILESET_TEXTURE_ID`, `SpriteBatch`, `SpriteInstance`.

## Design

- `renderer.rs` (729 L) — `pub trait Renderer: Send + Sync` (object-safe draw surface used across engines/editor), `pub const TILESET_TEXTURE_ID: u32 = 1`, `Sprite` descriptor, `draw_text_block` bitmap-font helper (font8x8), `WgpuRenderer` (async init, `set_atlas`, instanced batching, present) and `SoftwareRenderer` (CPU raster to u32 buffer; enables deterministic headless testing via the presenter).
- `sprite_batch.rs` (899 L) — `SpriteBatch` (10k instances, z-order + texture sorting; per-instance transform + sprite-rect + palette/player-remap fields in `SpriteInstance`, sent via bytemuck straight to the GPU); vertex shader constructs 6 vertices per sprite.
- `camera.rs` (113 L) — `Camera`: WORLD matrices with `world_to_screen`/`screen_to_world` and zoom/pan state.
- `ui.rs`, `palette_shader.rs` — empty placeholders (only `//!` header-less stubs; palette shader functionality lives inside `shaders/sprite_batch.wgsl` instead).

## Flow

Frame: camera set → sprites queued into `SpriteBatch` → `WgpuRenderer` sorts/binds atlas slot textures → draw arrays; `SoftwareRenderer` mirrors the same sprite feed to rasterization for `cnc-map-editor::presenter`. Loser of the GPU race (no adapter) transparently falls back to software.

## Integration

- Sole crate dependency (`cnc-core`) + wgpu/winit/pixels/image; dev-deps add `naga` (WGSL-in validation) and `pollster` for blocking async init in tests.
- Consumed by both engines, map editor, launcher. WGSL source is compiled at runtime from a shader include — signatures of the two renderer paths must stay in lockstep with `tests/gpu_smoke.rs` expectations.
