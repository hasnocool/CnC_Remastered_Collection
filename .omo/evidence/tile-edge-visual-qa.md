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

---

# Round 3 — per-theater visual QA (2026-10-25)

Generated and visually reviewed all four theaters (seed 42, zoom 1.0 and 2.0,
1400x900, grid off): Temperate, Snow, Desert, Urban.

## Defects found and fixed

### A. Terrain flat-colour palette was indexed modulo table size
`ui.rs::terrain_color` used `PALETTE[value % 6]` against a 6-entry table while
`TerrainType` has 15 discriminants, so every id >= 6 wrapped onto the wrong
colour (Rough -> grass green, Mud/Swamp -> water blue, Ice -> sand).
Snow (id 5) also got a tan rather than a snow colour. Fixed with an explicit
15-entry palette indexed by the discriminant.

### B. Snow/Swamp/Ice/Concrete have no pack art and rendered blank
Those kinds return `None` from `terrain_base_name` (documented gap), so they
fall back to flat colour. Symptom before the palette fix: large pure-white
regions (the wrong slot's colour) — after the palette fix all kinds render
their own flat colour with 0% near-white pixels (Snow now 98 distinct colours).

### C. Cliff bands tiled into a straight lattice on Desert/Urban
Round 2 quantised elevation to 4 levels 85 apart, so every level boundary
crossed `CLIFF_STEP = 3` and the renderer drew cliff art (5px dark rectangle +
light border) on nearly every boundary → long straight/right-angle dark lines
across the map. Fixed by persisting a FLAT elevation field: the generator
produces no authored height, so the renderer now sees level ground until
elevation is authored. `elevation_field_is_persisted_and_deterministic` updated
to pin the flat contract (its old assertion required positive variation, which
was the very behaviour causing the lattice).

## Result per theater (colour count / dark-seam fraction)
| Theater | Before | After |
|---|---|---|
| Temperate | 18.0% seam | 151 colours / 8.7% |
| Snow | blank white regions | 98 colours / 8.9% |
| Desert | straight dark lattice | 130 colours / 8.9% |
| Urban | straight dark lattice | 131 colours / 8.7% |

Remaining known-good characteristic: 4 rock variants cycle per cell (visible
speckle), which is intended pixel-art variance, not a defect.

## Gates
- cargo fmt --check (cnc-map-editor, cnc-renderer): PASS
- cargo clippy --all-targets -- -D warnings: 0 errors
- cargo test -p cnc-map-editor -p cnc-renderer: 184 passed, 0 failed

## Also fixed this round (renderer, wgpu path only)
`sprite_batch.rs` inset the UV rect by a fixed half texel on every sprite. That
is only correct when the quad is drawn at the sprite's native texel size; at
other zooms it crops the art. The inset now scales with
texels-per-quad-pixel and is clamped to a quarter extent. Note: the headless
editor uses the software renderer, which already clamped sampling to the
sprite's own rect, so this change is verified by code reading only, not by a
rendered diff.

---

# Rounded terrain borders (2026-10-25, user request)

User asked for terrain transitions (grass/dirt/etc.) to be rounded like the
water shoreline instead of square-edged.

## Cause
`terrain_transition()` used two square devices:
1. `profile()` with steps only every 3rd pixel - correct-ish, but the corner
   cut-outs were straight diagonals: `if not ne and n and e and x >= TILE-5 and
   y < 5 and x+y < 8` cuts a 45-degree *square stair* at each open diagonal.
2. Water's `connected_shore()` reads organic because its banks use per-column
   depths (5..9 with x%3 variation) and corners overlap into coves.

## Fix (pack generator + regenerated atlas)
- Corner cut-outs replaced with quarter-circle arcs: each open diagonal with
  both adjacent cardinals connected now tests distance from a corner centre
  `(x-cx)^2 + (y-cy)^2 < r^2` with a per-corner jittered radius 6..10, so the
  seam between two transition edges is rounded like the shoreline's coves.
- Depth profiles kept (they were fine); the arc + jitter is what removes the
  square look.
- Regenerated all 940 blob tiles in `terrain_transition.png`.

## Verification
- Before/after crop on `grass_to_dirt/blob_14`: the NEW tile shows a curved
  grass lobe (quarter-arc bottom-right + a rounded hole) where the OLD tile had
  straight diagonals and square stair-steps; 279 px changed.
- In-game editor render (seed 42, zoom 1.0): terrain borders now curve (dirt
  lobes arc, corners round).
- Gates: cnc-asset 16/16 engine tests (incl. composition determinism),
  cnc-map-editor suite green.
NOTE: the grey region in the render is the rock base art (mean 115,115,107 with
32 colours) - verified terrain, not a regression from this change.

---

# All tile transition families (2026-10-25)

User asked to fix every tile transition family, not just terrain blobs.

## Families audited
| Family | Count | Before | After |
|---|---|---|---|
| terrain_transition | 940 | square 45-degree diagonal cut-outs (fixed in the previous commit) | quarter-arc corners + irregular per-column depth |
| shoreline | 16 | already organic (per-column depths, coves) | no change needed - reference implementation |
| cliff_connection | 16 | flat rectangle bands + straight light crest lines | irregular per-column escarpment depth with BROKEN highlight crest (never a ruled line) |
| wall_connection | 16 | flat rectangles, square ends | tapered ragged end caps (low density, last 5 px) so the masonry core stays dominant |
| road_connection | 16 | already organic (tapered arms, bevelled caps, centre ellipse) | no change needed |
| bridge | 6 | rectangles + straight planks | left as-is (bridges are built structures; straight reads correctly) |

## Cliff/wall implementation
- Cliffs: `rock_dark` band drawn column-by-column with depth 5..8 varying
  every 3rd column; `rock_light` crest pixels with every 3rd skipped so the
  top edge is broken, not a straight line.
- Walls: end caps feather into the ground over the last 5 px of each open arm at
  35% density (first attempt used 35% over the full length and eroded the
  metal band - metal_dark share dropped, so it was retuned to the last 5 px and
  restored to ~49% band coverage).

## Verification
- Colour audits per family: wall_0a metal_dark share 0.487, wall_0f cross 0.492
  (masonry dominant, dirt caps taper), cliff_05 top colours rock / rock_dark
  band / rock_light crest present.
- In-game editor render (seed 42, zoom 2.0): terrain borders curve, cliff tops
  show broken crests, wall bands stay solid with feathered ends.
- Gates: cnc-asset clippy 0 warnings; cnc-asset `cnc32_engine_tests` 16/16
  including composition determinism after atlas regeneration.

---

# Visual QA of all transitions x all theaters (2026-10-25)

Captured all four theaters (Temperate/Desert/Snow/Urban) at zoom 2.0 and 3.0,
centred on a transition-rich region, grid off.

## Result
| Theater | Transitions | Notes |
|---|---|---|
| Temperate | rounded, organic | sand/grass/dirt boundaries all curve |
| Desert | rounded, organic | sand<->rock boundaries curve with quarter-arcs |
| Urban | rounded, organic | corner arcs visible at junctions (round discs) |
| Snow | flat colour | documented gap: no pack art for Snow (terrain_base_name -> None) |

## Verification
- Straightness scan over the map viewport: ZERO straight boundaries longer than
  40 px (square edges are gone).
- Corner tapling on rock_to_dirt/0x14: per-row rock width tapers 15 -> 8 -> 10
  (progressive arc, not stepped).
- Family assignment on a full 96x96 map: 8875 terrain blobs + 341 shoreline,
  zero cliff/road/wall misassignment (probe count by family).
- Colour audit per tile: correct three-tone materials everywhere sampled.

## Known remaining (documented, not regressions)
- Snow/Swamp/Ice/Concrete render flat (no pack art) - pre-existing gap.
- Rock base variants cycle per cell, so rock fields show tile-to-tile variance
  at high zoom; that is pixel-art variant behaviour, not an edge defect.
