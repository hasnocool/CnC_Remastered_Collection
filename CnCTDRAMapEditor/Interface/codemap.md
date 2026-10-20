# CnCTDRAMapEditor/Interface/ — Core Abstractions

## Responsibility
Defines **contracts (interfaces)** that decouple the editor's UI/tools from game-specific implementations. Enables dual-game support (Tiberian Dawn + Red Alert) through a single plugin architecture.

## Design Patterns

### Plugin Contract: IGamePlugin
**Primary abstraction** — each game provides an implementation:
```csharp
interface IGamePlugin {
    // Type registries (read-only dictionaries)
    IReadOnlyDictionary<int, ITechnoType> InfantryTypes { get; }
    IReadOnlyDictionary<int, ITechnoType> UnitTypes { get; }
    IReadOnlyDictionary<int, ITechnoType> BuildingTypes { get; }
    IReadOnlyDictionary<int, ITechnoType> AircraftTypes { get; }
    IReadOnlyDictionary<int, ITechnoType> VesselTypes { get; }  // RA only
    
    // Map I/O
    Map LoadMap(string path);
    void SaveMap(Map map, string path);
    
    // INI parsing
    void ParseRulesIni(string path);
    void ParseArtIni(string path);
    
    // Theater/tileset
    IEnumerable<TheaterType> Theaters { get; }
    Tileset GetTileset(TheaterType theater);
    
    // Game identification
    GameType Game { get; }  // TiberianDawn | RedAlert
    string GameName { get; }
}
```

### Type System Abstractions
| Interface | Purpose | Implementations |
|-----------|---------|-----------------|
| `ITechnoType` | Base for all buildable types | `BuildingType`, `UnitType`, `InfantryType`, `AircraftType`, `VesselType` |
| `INamedType` | Has `int ID` + `string Name` | All type classes |
| `IBrowsableType` | Has `IconIndex` for UI lists | All type classes |

### Spatial Abstractions
| Interface | Purpose |
|-----------|---------|
| `ICellOccupier` | Object occupies ≥1 cell (buildings, units, infantry) — `Cell[] Footprint` |
| `ICellOverlapper` | Visual-only overlap (overlays, smudges) — `Cell Cell` |

### Tool Abstraction
```csharp
interface ITool {
    string Name { get; }
    ToolCategory Category { get; }
    Cursor Cursor { get; }
    void Activate(MapPanel panel, Map map);
    void Deactivate();
    void MouseDown(Cell cell, MouseButtons btn, Keys modifiers);
    void MouseMove(Cell cell, Keys modifiers);
    void MouseUp(Cell cell, MouseButtons btn, Keys modifiers);
    void KeyDown(Keys key);
    void KeyUp(Keys key);
    void Render(Graphics g, RenderEventArgs args);
}
```

### Widget Abstraction
```csharp
interface IWidget {
    Control Control { get; }  // WinForms control to host
    void Update(Map map);     // Refresh from model
    event Action Changed;     // Notify host of changes
}
```

## Data & Control Flow

### Plugin Resolution
```
MainForm.LoadGamePlugin(GameType.TiberianDawn)
  → Activator.CreateInstance<TiberianDawn.GamePlugin>()
  → Plugin.ParseRulesIni(path) → populates type dictionaries
  → Plugin.ParseArtIni(path) → populates icon/art references
  → MainForm.Tools[] refreshed with plugin's type lists
  → MapPanel.Map = plugin.CreateEmptyMap()
```

### Tool Activation
```
MainForm.SetActiveTool(ITool tool)
  → tool.Activate(mapPanel, map)
  → tool subscribes to mapPanel events if needed
```

## Integration Points

### Implementations
| Interface | TiberianDawn | RedAlert |
|-----------|--------------|----------|
| `IGamePlugin` | `TiberianDawn/GamePlugin.cs` | `RedAlert/GamePlugin.cs` |
| `ITechnoType` | `TiberianDawn/*Types.cs` | `RedAlert/*Types.cs` |
| `ITool` | `Tools/*Tool.cs` (shared) | `Tools/*Tool.cs` (shared) |

### Consumers
| Consumer | Uses |
|----------|------|
| `MainForm.cs` | `IGamePlugin`, `ITool` |
| `MapPanel.cs` | `ITool`, `RenderEventArgs` |
| `Controls/*` | `ITechnoType`, `INamedType`, `IBrowsableType` |
| `Tools/*` | `ITool`, `ICellOccupier`, `ICellOverlapper` |
| `Model/` | `ICellOccupier`, `ICellOverlapper` on entity classes |

## Key Files
| File | Role |
|------|------|
| `IGamePlugin.cs` | **Core plugin contract** — enables dual-game support |
| `ITechnoType.cs` | Base type interface — ID, Name, Icon, Prerequisites, Cost |
| `INamedType.cs` | Minimal ID+Name |
| `IBrowsableType.cs` | UI list support — IconIndex |
| `ICellOccupier.cs` | Spatial occupancy — Footprint cells |
| `ICellOverlapper.cs` | Visual overlap — single cell |
| `ITool.cs` | Tool strategy contract |
| `IWidget.cs` | Embeddable UI component contract |