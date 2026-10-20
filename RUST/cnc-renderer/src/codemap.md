# RUST/cnc-renderer/src/

## Responsibility

Five implemented modules — `camera`, `renderer`, `sprite_batch`, `ui` (immediate-mode widget kit, 217 L) and `palette_shader` (indexed-palette + per-house remap pipeline, 591 L; *corrected 2026-10-20: this line previously called `ui.rs`/`palette_shader.rs` “two stubs … 6 lines each”, which was false — re-verified 2026-10-20, `.omo/evidence/task-6-parity-100.log` §J*); `lib.rs` re-exports `Camera`, `Renderer`, `Sprite`, `WgpuRenderer`, `SoftwareRenderer`, `TILESET_TEXTURE_ID`, `SpriteBatch`, `SpriteInstance`.

## Design

- `renderer.rs` (782 L — *re-verified 2026-10-20, was “729 L”*) — `pub trait Renderer: Send + Sync` (object-safe draw surface used across engines/editor), `pub const TILESET_TEXTURE_ID: u32 = 1`, `Sprite` descriptor, `draw_text_block` bitmap-font helper (font8x8), `WgpuRenderer` (async init, `set_atlas`, instanced batching, present) and `SoftwareRenderer` (CPU raster to u32 buffer; enables deterministic headless testing via the presenter).
- `sprite_batch.rs` (945 L — *re-verified 2026-10-20, was “899 L”*) — `SpriteBatch` (10k instances, z-order + texture sorting; per-instance transform + sprite-rect + palette/player-remap fields in `SpriteInstance`, sent via bytemuck straight to the GPU); vertex shader constructs 6 vertices per sprite.
- `camera.rs` (284 L — *re-verified 2026-10-20, was “113 L”*) — `Camera`: WORLD matrices with `world_to_screen`/`screen_to_world` and zoom/pan state.
- `ui.rs` (217 L) — immediate-mode UI widget kit over the `Renderer` trait (buttons/labels/hit-testing helpers + unit tests via a recording renderer); no egui dependency.
- `palette_shader.rs` (591 L) — indexed-palette pipeline: 256×1 palette texture + 256×1 R8Uint per-house remap texture + replaceable R8Uint index texture, with shadow/transparent modes; the WGSL lives in `shaders/sprite_batch.wgsl`.

*Stub claim corrected 2026-10-20 (previous text: “`ui.rs`, `palette_shader.rs` — empty placeholders (only `//!` header-less stubs…)”, which was false — both ship implemented modules with tests; the line counts above were also stale). Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §J.*

## Flow

Frame: camera set → sprites queued into `SpriteBatch` → `WgpuRenderer` sorts/binds atlas slot textures → draw arrays; `SoftwareRenderer` mirrors the same sprite feed to rasterization for `cnc-map-editor::presenter`. Loser of the GPU race (no adapter) transparently falls back to software.

## Integration

- Sole crate dependency (`cnc-core`) + wgpu/winit/pixels/image; dev-deps add `naga` (WGSL-in validation) and `pollster` for blocking async init in tests.
- Consumed by both engines, map editor, launcher. WGSL source is compiled at runtime from a shader include — signatures of the two renderer paths must stay in lockstep with `tests/gpu_smoke.rs` expectations.
