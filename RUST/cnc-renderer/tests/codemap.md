# RUST/cnc-renderer/tests/

## Responsibility

`gpu_smoke.rs` (199 lines): end-to-end GPU verification — create a real wgpu device, render through `SpriteBatch`, read back the framebuffer and check pixels (its module doc says exactly that; 2 tests, ignored unless an adapter is present, per BUILD_STATUS).

## Design

- `try_device() -> Option<(wgpu::Device, wgpu::Queue)>` — substitutes a synthetic device when no GPU exists so the suite can skip cleanly.
- `render_and_readback(async move closure)` — maps the framebuffer into host memory for pixel assertions; `pixel_at(data, width, x, y) -> [u8; 4]` decodes texels; `quad_batch(device, queue, width, height)` builds a `SpriteBatch` fixture.

## Flow

Tests: `sprite_batch_renders_a_red_quad_on_gpu` (instanced quad → expected single-colour texels) and `sprite_atlas_uv_selection_is_sampled_on_gpu` (atlas frame UV → sampled tile appears at the expected pixels); async wgpu calls are blocked via dev-dep `pollster`.

## Integration

Validates the actual pipeline in `cnc-renderer/src/sprite_batch.rs` + `shaders/sprite_batch.wgsl` on real hardware — the strongest test in the crate (unit tests cover only camera + batching math, 3 tests); complements `naga` WGSL-in validation with execution.
