# RUST/cnc-renderer/

## Responsibility

2D rendering crate: a `Renderer` abstraction with a wgpu implementation and a software fallback, instanced sprite batching over a 2D texture array (slot 0 = procedural unit/building atlas, slot `TILESET_TEXTURE_ID` = RTS tileset pack), palette-index shader with player remaps, an orthographic camera, and font8x8 text drawing (BUILD_STATUS Phase 3 complete: 3 camera/sprite-batch unit tests + 2 GPU smoke tests).

## Design

- `Renderer` trait (`renderer.rs`, 729 L): `Send + Sync` object-safe drawing surface; implementations `WgpuRenderer` (async device creation, `set_atlas`, texture-array uploads) and `SoftwareRenderer` (offscreen u32 buffer, used headless). `Sprite { texture_id, ... }` selects the atlas slot; `SpriteBatch`/`SpriteInstance` (`sprite_batch.rs`, 899 L) use instanced quads — 10k instance capacity, z-order + texture sorting, six vertices per sprite generated in the vertex shader; `draw_text_block` helper over `font8x8`.
- WGSL `shaders/sprite_batch.wgsl`: 256-colour palette remapping + alpha blending (per BUILD_STATUS); dev-dep `naga` validates WGSL in tests.
- `camera.rs` (113 L): `Camera` view/projection, world↔screen transforms, zoom/pan.
- `ui.rs`/`palette_shader.rs`: six-line placeholder modules, intentionally empty (evidence: 6 lines each).
- Tileset contract: `TILESET_TEXTURE_ID: u32 = 1`; `../../assets/packs/rts_16x16_tileset` loads via `cnc_asset::tileset_pack::TilesetPack` then uploads here.

## Flow

Caller builds `Sprite`s (entity or tile layer) → `SpriteBatch::queue` → sort by z/texture → one instanced draw per slot through the WGSL pipeline → GPU present (wgpu) or CPU pixel buffer (software). Camera transforms map between world coordinates and the render target.

## Integration

- Depends on: `cnc-core` + wgpu/winit/pixels/image/bytemuck/ahash; `wgpu 22` pinned workspace-wide.
- Consumed by `cnc-tiberian-dawn`, `cnc-red-alert`, `cnc-map-editor` (interactive + headless presenter) and `cnc-launcher` (window shell + screenshots); `tests/gpu_smoke.rs` verifies real-device behaviour when an adapter is present (otherwise ignored).
