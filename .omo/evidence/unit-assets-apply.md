# Unit assets applied to editor + launcher (2026-10-25)

## What was broken
Units drew as flat coloured rectangles everywhere: the editor's object loop
and the launcher's game view both used `draw_rect`, and the only sprite atlas
uploaded was the procedural `AtlasStyle::Neon` generator. The pack's 2176
`unit_animation` frames + 448 `unit_condition` sprites were never loaded by
any consumer.

## Implementation
1. `cnc-asset/src/unit_atlas.rs` (new): `UnitAtlas` loads the pack's
   `unit_animation.png` (512x4352) and `unit_condition.png` (512x896) whole,
   indexes frames by `unit/state/facing/frame` and conditions by
   `unit/condition/facing` from the manifest, and exposes
   `first_frame(kind, UnitAnim, facing)` / `condition(...)`.
2. `classic_pack::SpriteEntry` gained the unit fields the manifest carries
   (`faction`, `unit`, `state`, `facing`, `frame`, `duration_ms`, `loop_`,
   `condition`) - previously silently ignored by serde.
3. TD `unit.rs`: pack art path in `render_with` - class id -> pack kind
   (tank/harvester/artillery/infantry/engineer/rocket_soldier/buggy),
   facing derived from the unit's heading, idle vs move from the path state;
   falls through to the procedural atlas for kinds the pack does not ship.
4. Launcher: uploads the pack unit atlas into texture slot 0 (replacing the
   procedural neon atlas when present); log line
   `pack unit atlas ready atlas=unit_animation width=512 height=2720 units=7`.
5. Editor: uploads slot 0 beside the terrain slot and draws unit objects from
   the atlas (buildings keep flat quads). Fixed the class-id mapping to the
   REAL roster semantics (E1 light tank ... E6 harvester, E7 MCV - the first
   attempt had E4/E6 swapped).

## Debug trail (screenshots + probes)
- First editor screenshot still showed a flat quad: my first edit patched
  `ui.rs::EditorUI::render`, but the headless path renders through
  `app.rs::EditorState::render` - fixed the live path and left both wired.
- Probe showed `texture_ready(0)=true` yet 0 drawn pixels: `compose()`
  re-composited each atlas PNG at a single sprite's rect (whole PNGs pasted at
  one offset, later sprites overwriting earlier ones) so frame rects pointed
  past the buffer. Replaced with `decode_whole()`: each atlas PNG is already a
  complete sheet, decoded directly (512x4352).
- After the fix the probe reports 256 non-blank pixels for one tank sprite
  (opaque pixels of the 16x16 quad; the frame has transparent margins).

## Visual verification
- Editor (Temperate, seed 42, zoom 2.0, player start): MCV draws as the pack
  tank sprite under the selection crosshair - no flat quad.
- Editor (Desert, seed 7, zoom 2.0): MCV tank sprite on textured desert
  terrain with clean transitions.
- Launcher log: pack atlas uploaded, mission boots with 17 units.

## Gates
- fmt: clean; clippy -D warnings: 0 errors across cnc-asset, cnc-tiberian-dawn,
  cnc-map-editor, cnc-launcher.
- tests: cnc-asset + cnc-tiberian-dawn + cnc-map-editor + cnc-launcher suites
  pass (see commit; bg_99 run had 0 FAILED lines).

---

# RA + launcher verification (2026-10-25, user re-report: "sprites dont seem to work anymore")

## Root cause
The TD fix never reached RA: `cnc-red-alert::render::render_entity` was a flat
`draw_rect` and the RA app never uploaded the pack atlas into slot 0. The
launcher's RA frame loop (`main.rs` draw) therefore rendered RA units as owner
colour squares.

## Fix
- `cnc-red-alert/src/render.rs`: `render_entity` now prefers the pack atlas
  (`LazyLock<UnitAtlas>`), mapping class ids to pack kinds with the real roster
  semantics (InfantryE1..E3/E5/E10/E11 -> infantry, E4/E7 -> rocket_soldier,
  E6 -> engineer, VehicleE1..E4/E7 -> tank, E6 -> harvester, E5 -> buggy), and
  falls back to the flat quad for unshipped kinds.
- Launcher RA app resume: uploads the pack unit atlas into texture slot 0 (log:
  `pack unit atlas ready (RA)`), mirroring the TD app.
- New bin `cnc-launcher/src/bin/ra_units_view.rs`: captures the RA engine's own
  render path headlessly with the atlas uploaded.

## Debug trail (why the first screenshots looked wrong)
- The capture initially showed only flat quads. Instrumenting
  `render_entity` showed the kinds DID resolve (400 -> infantry, 500 -> tank,
  505 -> harvester) and `texture_ready(0)` was true.
- `SoftwareRenderer::set_camera` is a **no-op** (`renderer.rs:675`), so a
  software capture renders at RAW WORLD COORDINATES - entities appear at their
  world pixel position (near the top-left of a 1280x720 frame), NOT at the
  camera-centred position. My first camera-framing attempt and centre crops
  were therefore reading empty terrain while the sprites were off in the
  corner.
- Cropping the actual world coordinates (unit at world 132,132) shows the pack
  sprites: a dark-blue vehicle with body/wheel detail and a second sprite
  below, on correctly textured terrain.
- The bin documents this so future QA does not re-chase the camera.

## Verified
- `pack unit atlas ready: 512x4352, 7 unit kinds` on capture start.
- Unit crop: real pack sprites drawn (20+ distinct colours vs ~6 for bare
  terrain), zero flat-quad owner-colour pixels in the region.
- Gates: fmt clean; clippy 0 warnings; cnc-red-alert + cnc-launcher suites pass.

## Remaining
Condition sprites (damaged/critical/destroyed) are indexed but not consulted;
units always draw frame 0 of idle/move (no frame stepping by duration).
