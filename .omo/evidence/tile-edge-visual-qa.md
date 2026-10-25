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

---

# Round 2 — the real defect (2026-10-25, user-reported regression)

The first round's conclusion ("not a bug") was WRONG. The user re-reported the
same grey hatches. Re-investigation found THREE genuine defects:

## Defect 1 — rock base tiles had hardcoded hatch strokes
`source/cnc32_asset_generator_v5.py` rock branch drew two fixed-coordinate
`d.line(...)` strokes on every rock variant (identical on all 4 → hatch lattice),
plus a `(1,1)` light + `(30,30)` dark dot on every tile (corner lattice).
Fix: replaced with per-variant 1px facet speckle; rock no longer gets the
per-tile corner highlight. Regenerated `terrain_base.png` rock rects (256-384).

## Defect 2 — blob edge shadow was a hard dark outline
`terrain_transition()` drew a 1px `kind_dark` border on every inside pixel
adjacent to outside, so every blob tile carried a black outline that formed a
continuous lattice once tiles butted together.
Fix: 50% blend of the two terrains' darks. Regenerated all 940
`terrain_transition.png` blob rects.

## Defect 3 (dominant) — generator persisted a CONTINUOUS elevation field
`generator.rs` stored the raw fbm field as 0-255, so `CLIFF_STEP = 3` was crossed
between almost every pair of adjacent cells → cliff art (filled dark bands +
fixed stroke lines) on nearly every tile → the dense hatch lattice covering the
whole map. Flat render proved terrain coverage was rock, not overlay.
Fix: quantise elevation to 4 discrete levels (0/85/170/255) before persisting.
Only genuine level steps now cross the threshold.

## Verification after the fix
- Rendered map (seed 42, zoom 1.0, grid off): hatches gone; terrain reads as
  coherent grass/dirt/rock/sand regions with proper transition borders.
- cnc-map-editor: 164 tests pass, fmt clean, clippy 0 warnings.
- cnc-asset: 154 tests pass, incl. `cnc32_engine_tests` 16/16
  (`composition_is_deterministic` — the composed atlas still validates against
  the manifest rects after regeneration).

## Repro
cd RUST && ./target/debug/cnc-map-editor --headless --seed 42 \
  --out /tmp/v.ini --screenshot /tmp/m.png \
  --zoom 1.0 --center 48,48 --no-grid --width 1400 --height 900
