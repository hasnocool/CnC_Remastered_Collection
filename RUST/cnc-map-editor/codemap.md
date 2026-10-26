# RUST/cnc-map-editor/

## Responsibility
Map editor application for the C&C Remastered Collection Rust port. Provides terrain painting, object placement, trigger editing, tileset management, and map testing for both Tiberian Dawn and Red Alert. Integrates with cnc-procgen for procedural generation.

## Design Patterns
- **Egui/eframe GUI**: Immediate-mode editor UI with viewport, tool palette, property panels
- **Plugin Architecture**: `plugin.rs` - `EditorPlugin` trait for game-specific logic (TD/RA)
- **Presenter Pattern**: `presenter.rs` - mediates between UI and map model, handles commands
- **History/Undo**: `history.rs` - command-based undo/redo stack for all edits
- **Tileset Management**: `tiles.rs` - terrain tileset loading, brush preview, tile picking
- **Map Model**: `map.rs` - `Map`, `CellGrid`, `Overlay`, `Object` (buildings, units, triggers)
- **Terrain Tools**: `tools.rs` - brush, fill, smooth, cliff, shore tools
- **Trigger Editor**: Visual trigger editing (conditions/actions) with INI export
- **Procedural Generation**: `generator.rs` - integrates `cnc-procgen` for terrain/base generation
- **Tileset Packing**: `pack_tileset.rs` - exports tilesets for `cnc-asset` consumption
- **Original Art Source**: `original_tileset.rs` - composites the shipped Remaster MEG textures into the same combined-atlas layout, selected by `--original-tiles`; `ui::TileSource` abstracts generated vs original so the render loop is unchanged
- **Original Format**: `original.rs` - reads/writes legacy Westwood map format
- **Fixtures**: `fixtures.rs` - test maps, scenario templates

## Data & Control Flow
1. **Init**: `main.rs` → `App::new()` → loads default tileset, initializes `Presenter`
2. **Load Map**: `presenter.load_map(path)` → parses INI/binary → `Map` model
3. **Edit Loop**: UI events → `Presenter::handle_action()` → `Command` → `History::push()` → model mutation → viewport redraw
4. **Terrain Paint**: `tools::TerrainTool` → `CellGrid::set_tile()` → auto-transition (cliff/shore) → `tileset::transitions`
5. **Object Place**: `tools::ObjectTool` → `Map::add_object()` → `Object` with `ClassId`, position, owner
6. **Trigger Edit**: Trigger UI → `Trigger`/`Action`/`Condition` → INI serialization
7. **Generate**: `generator.rs` → `cnc-procgen::terrain`/`base` → applies to `CellGrid`
8. **Save/Export**: `presenter.save_map()` → INI + binary → file; `pack_tileset` → `cnc-asset` format
9. **Test Play**: Spawns `cnc-tiberian-dawn`/`cnc-red-alert` `Game` with current map

## Public API
- **app**: `App`, `AppConfig`
- **presenter**: `Presenter`, `EditorAction`, `EditorCommand`
- **map**: `Map`, `CellGrid`, `Overlay`, `Object`, `MapMetadata`
- **tiles**: `Tileset`, `Tile`, `TileBrush`, `TransitionRule`
- **tools**: `Tool`, `TerrainTool`, `ObjectTool`, `TriggerTool`, `SelectTool`
- **history**: `History`, `Command`, `UndoRedo`
- **plugin**: `EditorPlugin`, `TiberianDawnPlugin`, `RedAlertPlugin`
- **generator**: `MapGenerator`, `GeneratorConfig`
- **pack_tileset**: `pack_tileset`, `write_packed_tileset`
- **original**: `load_original_map`, `write_original_map`
- **fixtures**: `test_maps`, `scenario_templates`

## Integration Points
- **Dependencies**: cnc-core, cnc-renderer, cnc-asset, cnc-procgen, cnc-tiberian-dawn, cnc-red-alert, eframe, egui, winit, image, serde, anyhow, thiserror, parking_lot, once_cell
- **Consumed by**: Developers, map makers, QA
- **Game Plugins**: `TiberianDawnPlugin`, `RedAlertPlugin` implement `EditorPlugin`
- **Asset Pipeline**: Exports to `cnc-asset` tileset packs

## Key Files
- src/main.rs - Application entry, eframe bootstrap
- src/lib.rs - Module declarations, re-exports
- src/app.rs - App struct, configuration, main loop integration
- src/presenter.rs - Presenter, EditorAction, EditorCommand, mediator
- src/map.rs - Map, CellGrid, Overlay, Object, metadata
- src/tiles.rs - Tileset, Tile, brush, transitions
- src/tools.rs - Tool trait, TerrainTool, ObjectTool, TriggerTool, SelectTool
- src/history.rs - History, Command, undo/redo
- src/plugin.rs - EditorPlugin trait, TD/RA implementations
- src/generator.rs - MapGenerator, procedural integration
- src/pack_tileset.rs - Tileset export for cnc-asset
- src/original_tileset.rs - Original Remaster texture source (native MEG reads)
- src/original.rs - Legacy Westwood map format I/O
- src/fixtures.rs - Test fixtures, scenario templates
- src/ui.rs - Egui UI components, viewport, panels

