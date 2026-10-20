# RUST/cnc-map-editor/

## Responsibility

Standalone map editor for both engines: an editable, game-agnostic map model with strict undo/redo, authoring tools, game (theater/catalog/validation) plugins, procedural map generation with INI+PNG output, and two front-ends — an interactive winit/wgpu window (plus `softbuffer`-presented software fallback) and headless/validate CLI modes (BUILD_STATUS: 80 unit + 2 integration tests).

## Design

- Command-pattern undo system is the architectural spine (`history.rs`, 848 L): every mutation is an `EditCommand` (`PaintCells`, `PlaceObject`, `RemoveObject`, `MoveObject`, `EraseObject`, `SetObjectOwner`, `AddTrigger`, `SetTrigger`, `RemoveTrigger`), executed only via `History::execute` — the model can never be mutated outside the undo stack (matches root AGENTS.md rule).
- `MapModel` (`map.rs`, 769 L) is plain data: terrain/overlay grids, `MapObject`s (with owners), `Trigger`s, player starts, `AgentDefinition`s (from `cnc-ai`), scenario INI I/O (`to_ini`/`from_ini`) and `validate()`.
- Tools (`tools.rs`) never mutate: they return `EditCommand`s. Plugin architecture (`plugin.rs`): `GamePlugin` trait implemented by `TiberianDawnPlugin` and `RedAlertPlugin` supplying `Theater` lists, buildable catalogs, starting credits, validation rules.
- Presenter/view split: `app.rs` (6460 L) immediate-mode, renderer-agnostic `EditorState` drawing via any `cnc_renderer::Renderer`; `ui.rs` view state (`EditorUI` tools/history/selection/grid); `presenter.rs` blits `SoftwareRenderer`'s u32 buffer to winit via `softbuffer` (headless-capable).
- `generator.rs`: deterministic fBm value-noise terrain generator (`generate`, `fbm`, `value_noise`, `MapGenSettings`, `TerrainType`, `OverlayType`).

## Flow

CLI `main.rs` parses args (`--headless`, `--validate <INI>`, --seed/--screenshot) → builds `MapModel` + `EditorUI` + plugin → click/keyboard input → tool returns command → `History::execute` mutates model → `App` renders via renderer; menus save/load INI (round-trip proven by the integration tests) and headless mode writes scenario INI + PNG screenshot.

## Integration

- Depends on: `cnc-core`, `cnc-renderer`, `cnc-asset`, and `cnc-ai` (agents embedded in maps); `image`, `wgpu`, `winit`, `softbuffer`, `native-dialog`, `pollster`.
- Consumed by nothing (leaf application crate); scenario INIs it writes are loaded back by `cnc-red-alert::scenario` and the launcher's game-select paths — the INI dialect is the cross-crate contract.
