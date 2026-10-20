# CnCTDRAMapEditor/Model/ — Core Data Model

## Responsibility
Pure data structures representing the **map document** and all its components. No UI logic, no rendering — only state, serialization, and validation. This is the single source of truth for what a map contains.

## Design Patterns

### Composite Section Pattern
`Map.cs` aggregates typed sections implementing `IMapSection`:
```csharp
interface IMapSection {
    string Name { get; }
    void Load(INIParser ini);
    void Save(INIParser ini);
    void Validate(Map map, List<string> errors);
}
```
Sections (all in `Model/`):
| Section | Purpose |
|---------|---------|
| `BasicSection` | Map metadata: name, author, description, dimensions, theater |
| `MapSection` | Tile grid: `TemplateType` per cell (base terrain) |
| `OverlaySection` | Overlays: `OverlayType` + data per cell (concrete, walls, rivers) |
| `SmudgeSection` | Temporary marks: `SmudgeType` + data (scorch, craters, bodies) |
| `TriggerSection` | Mission triggers: Event/Condition/Action triples |
| `TeamTypeSection` | AI team definitions: composition, orders, waypoints |
| `WaypointSection` | Named waypoints for team paths |
| `BriefingSection` | Campaign briefing text, voiceover cues |
| `SteamSection` | Workshop metadata: title, description, tags, preview image |

### Spatial Index
- `CellGrid.cs` — 2D array (`Cell[,]`) with bounds-checked accessors
- `CellMetrics.cs` — Precomputed cell dimensions, isometric projections, screen↔map conversion
- `Cell.cs` — Single cell: `TemplateType`, `OverlayType`+`OverlayData`, `SmudgeType`+`SmudgeData`, `Trigger` reference, `Waypoint` index, occupancy flags

### Game Objects (Polymorphic via Interface)
All map objects implement `ICellOccupier` (occupies ≥1 cell) and/or `ICellOverlapper` (visual overlap only):
| Type | Base | Occupies Cells | Key Properties |
|------|------|----------------|----------------|
| `Building` | `MapObject` | Yes (footprint) | `BuildingType`, `Owner` (House), `Facing`, `Health`, `Sold` |
| `Unit` | `MapObject` | Yes (1 cell) | `UnitType`, `Owner`, `Facing`, `Health`, `Veterancy` |
| `Infantry` | `MapObject` | Yes (1 cell) | `InfantryType`, `Owner`, `Facing`, `Health`, `Veterancy` |
| `Overlay` | `MapObject` | No (visual) | `OverlayType`, `Data` |
| `Smudge` | `MapObject` | No (visual) | `SmudgeType`, `Data` |

### Type System (Game-Agnostic)
- `TemplateType`, `OverlayType`, `SmudgeType`, `TerrainType` — **enums** defined per-game in `TiberianDawn/` or `RedAlert/`
- `House.cs` / `HouseType.cs` — Player faction slot (color, name, AI/IQ, starting credits)
- `DirectionType.cs` — 8-direction facing (N, NE, E, ...)
- `TypeItem.cs` — Base for all typed entries: `ID`, `Name`, `IconIndex`

### Undo/Redo Integration
- `UndoRedoList` (in `Utility/`) tracks `IUndoRedoCommand`
- Model mutations (add/remove/move objects, change terrain) emit commands via `PropertyTracker`
- `Map` exposes `UndoRedoList` for tools to push commands

## Data & Control Flow

### Serialization (INI-based)
```
Map.Save(path):
  1. Create INIParser
  2. For each section in order: section.Save(ini)
  3. Write INI to file (Windows line endings)

Map.Load(path):
  1. Parse INI → INIParser
  2. Detect game (TD vs RA) from [Basic] Theater key
  3. Instantiate correct GamePlugin
  4. For each section: section.Load(ini)
  5. Post-load: resolve cross-refs (triggers → objects, teams → waypoints)
  6. Validate → collect errors/warnings
```

### Validation
- `Map.Validate()` → runs all `IMapSection.Validate()`
- Checks: object within bounds, valid type IDs, trigger references exist, waypoint connectivity, house count matches players

### Change Notification
- Model classes implement `INotifyPropertyChanged`
- `PropertyTracker` subscribes → marks map dirty → enables Save menu item
- `UndoRedoEventArgs` fires on stack changes → updates Undo/Redo button states

## Integration Points

### Consumers
| Consumer | Usage |
|----------|-------|
| `Controls/MapPanel.cs` | Reads `CellGrid`, `Objects` for hit-testing, selection |
| `Render/MapRenderer.cs` | Iterates sections for layered drawing |
| `Tools/*Tool.cs` | Mutate model via `Map` methods, push undo commands |
| `Dialogs/TriggersDialog.cs` | Edits `TriggerSection` with type-safe combo boxes |
| `Utility/SteamworksUGC.cs` | Serializes `Map` → binary → Steam UGC |

### Producers
| Source | Data Flow |
|--------|-----------|
| `TiberianDawn/GamePlugin.cs` | Provides type registries (`BuildingTypes`, `UnitTypes`, etc.) for validation |
| `RedAlert/GamePlugin.cs` | Same for RA |
| `Utility/INI.cs` | Low-level INI parsing |
| `Utility/Megafile.cs` | Reads game `.mix` for tileset/template data |

## Key Files
| File | Role |
|------|------|
| `Map.cs` | Root aggregate — sections, serialization, validation, game plugin reference |
| `CellGrid.cs` | Spatial index — cell access, bounds, neighborhood queries |
| `CellMetrics.cs` | Coordinate math — isometric projection, screen↔cell conversion |
| `Building/Unit/Infantry.cs` | Game objects — properties, occupancy, serialization |
| `Trigger.cs` | Trigger definition — event, conditions, actions, enable/disable |
| `TeamType.cs` | AI team — unit list, script, waypoint list, house ownership |
| `MapSection.cs` | Base terrain grid — `TemplateType[,]` |
| `OverlaySection.cs` / `SmudgeSection.cs` | Visual layers |
| `BasicSection.cs` | Metadata — name, theater, dimensions, player count |
| `SteamSection.cs` | Workshop publish metadata |