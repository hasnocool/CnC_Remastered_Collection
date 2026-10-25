# Tile edge / grey-annotations visual QA (2026-10-25)

## Reported defect
"Map editor shows a lot of grey hash marks, placed everywhere, looks off."

## Investigation (grounded, each step observed)

1. Multi-zoom capture added to the headless editor: `--zoom N`, `--center X,Y`,
   `--no-grid` (main.rs). Captured 0.5x / 1.0x / 2.0x / fit, grid on and off.
2. Grid measurement: grid overlay = 9.0% of screen pixels at 2x; grid-off removes
   ~2.8% of dark pixels. Confirmed as a contributor, not the cause.
3. Sprite-resolution audit: 9216/9216 cells resolved a base sprite and 2941/2941
   layers resolved on a real generated map (seed 42, 96x96). No fallback gaps, no
   id collisions (20 base sprites → 20 distinct rects).
4. Source-atlas inspection: terrain_base.png tiles are clean pixel-art
   (dirt std 6.4, grass std 4.2, rock std 11.5 — rock is genuinely grey/noisy).
5. Seam quantification: dark-seam fraction is 12.4% in the editor fit view vs
   12.0% in the tileset_render reference harness (identical tile path). The
   editor adds no seam artifact of its own.
6. Grey share at fit zoom: 26.2% of the viewport is R≈G≈B grey — the seed-42 map
   is ~50% Rock/Rough, and the pack's rock sprite is grey [113,113,107] by
   design (terrain_base_name maps Rough→rock).

## Conclusion
Not a mapping bug. The "grey hash marks" are three expected叠加 effects:
(a) genuine grey rock/rough terrain dominating the generated map,
(b) per-tile pixel-art speckle from the 32px source sprites (nearest-neighbour
upscale at zoom ≠ 1.0 emphasises it), (c) the 1px cell grid, on by default.

## Reproduce
cd RUST && cargo run -p cnc-map-editor -- --headless --seed 42 \
  --screenshot /tmp/fit.png --width 1400 --height 900          # grid on, fit
cd RUST && ./target/debug/cnc-map-editor --headless --seed 42 \
  --screenshot /tmp/nogrid.png --no-grid --zoom 2 --center 48,48

## Gates
- cargo fmt --check -p cnc-map-editor: PASS
- cargo clippy -p cnc-map-editor --all-targets -- -D warnings: 0 errors
- cargo test -p cnc-map-editor: 164 passed, 0 failed
