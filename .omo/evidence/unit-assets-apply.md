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
