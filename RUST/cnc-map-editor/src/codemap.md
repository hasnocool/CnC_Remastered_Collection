# RUST/cnc-map-editor/src/

## Responsibility

Five modules + two in-module integration-test modules: `lib.rs` declares and re-exports toolkit types (commands, tools, plugin trait, generator settings, `EditorUI`, `MapModel`).

## Design

- `map.rs` (769 L) — data model: `MapModel`, `MapObject`, `Trigger`, `TILE_SIZE`; INI (de)serialization `to_ini`/`from_ini`, `validate()`; agents add/lookup (`add_agent` etc.) over `cnc_ai::AgentDefinition`.
- `pack_tileset.rs` — HARD-SWITCH tile adapter for the `cnc32_assets_v6_2_1` 32px pack: owns `cnc_asset::classic_pack::{Cnc32Pack, PackTileset}` + the composed-atlas address space; per-cell matcher `cell_draw` mirrors `tiles.rs`'s priority chain with pack-native sprite ids (8-bit blob terrain transitions with `repair_blob_mask`, road/wall/shore 4-bit masks, cliff bands via the inverted-connected mask, resource/tree sprite layers); `terrain_base_name()` maps the 15 editor `TerrainType`s onto the pack's `dirt/grass/rock/sand/water` (snow/swamp/ice/concrete stay flat-colour); `--tileset <dir>` override + `--tileset-palette` was removed with the legacy 16px pack.
- `tiles.rs` (895 L) — legacy 16px pack decision pipeline (`base_draw`, `TileDraw`) retained for the legacy pack QA example and unit tests; the editor itself no longer draws through it.
- `history.rs` (848 L) — `History` command stack; commands capture replaced state incl. after INI reload: `PaintCells`, `PlaceObject`, `RemoveObject`, `MoveObject`, `EraseObject`, `SetObjectOwner`, `AddTrigger`, `SetTrigger`, `RemoveTrigger` under `EditCommand`.
- `tools.rs` (297 L) — `Tool` trait, `TerrainTool`, `ObjectTool` (with_owner), `SelectTool`, `EraseTool`, `TriggerTool`; helpers `line_cells`, `rect_cells`.
- `plugin.rs` (252 L) — `GamePlugin` trait, `TiberianDawnPlugin`, `RedAlertPlugin`, `Theater`.
- `ui.rs` (287 L) — `EditorUI`: active tool, history, selection, grid rendering through `cnc_renderer::Renderer`, mouse handlers routing commands to `History`.
- `app.rs` (6460 L) — `EditorState`/layout/rendering immediate-mode view (contains its own `integration_tests` module: `authoring_session_roundtrips_through_ini`).
- `generator.rs` (591 L) — `generate(settings)`, `fbm`, `value_noise`, `MapGenSettings`, `TerrainType`, `OverlayType` for deterministic seeds.
- `presenter.rs` (106 L) — `SoftwareRenderer` buffer → winit via `softbuffer`.
- `main.rs` (856 L) — CLI: interactive window or `--headless --seed --screenshot` self-test, `--validate <path>` scenario checker.

## Flow

Input (mouse/CLI args/random seed) → tool command → `History::execute` → `MapModel` fields → renderer draw (`App`/`EditorUI`) / INI output. Save/load = INI text; the model, not History, owns the file I/O.

## Integration

- App crates: `cnc-ai` is referenced from `map.rs`, `history.rs`, `app.rs` (agent placement + agent edit commands).
- `cnc-renderer` Renderer trait is the only drawing surface (used by preview/interactive/presenter); `cnc-core` supplies `ClassId`/`Vec2i`/fixed-point.
- Guides referenced in docs: root AGENTS.md; per-module red-power/menus etc. documented in `app.rs`.
