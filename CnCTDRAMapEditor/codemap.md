# CnCTDRAMapEditor/ — C# Map Editor Application (WinForms)

## Responsibility
Windows Forms desktop application (**MobiusEditor** namespace) for creating and editing maps for both **Tiberian Dawn** and **Red Alert**. Provides a visual tile-based map editor with terrain painting, object placement, trigger scripting, team/waypoint management, and Steam Workshop publishing integration. Targets .NET Framework 4.6.2, x64 only.

## Design Patterns

### Plugin Architecture (Game Abstraction)
**`Interface/IGamePlugin.cs`** defines the contract for game-specific logic:
```csharp
interface IGamePlugin {
    // Type registries
    IEnumerable<ITechnoType> GetInfantryTypes();
    IEnumerable<ITechnoType> GetUnitTypes();
    IEnumerable<ITechnoType> GetBuildingTypes();
    IEnumerable<ITechnoType> GetAircraftTypes();
    // Map format handlers
    Map LoadMap(string path);
    void SaveMap(Map map, string path);
    // INI parsing
    void ParseRulesIni(string path);
}
```
Two implementations:
- `TiberianDawn/GamePlugin.cs` — TD type definitions, map format, theater
- `RedAlert/GamePlugin.cs` — RA type definitions, map format, theater

**Factory pattern**: `MainForm` instantiates active plugin based on user selection (TD vs RA).

### Model-View Separation
| Layer | Key Types | Responsibility |
|-------|-----------|----------------|
| **Model** | `Model/Map.cs`, `Model/CellGrid.cs`, `Model/Building.cs`, `Model/Unit.cs`, `Model/Infantry.cs`, `Model/Trigger.cs`, `Model/TeamType.cs` | Pure data structures, serialization, undo/redo |
| **View/Controls** | `Controls/MapPanel.cs`, `Controls/TerrainProperties.cs`, `Controls/ObjectProperties.cs`, `Controls/PlayerSettings.cs` | Rendering, property inspectors, palette UI |
| **Tools** | `Tools/BuildingTool.cs`, `Tools/TerrainTool.cs`, `Tools/UnitTool.cs`, `Tools/CellTriggersTool.cs` | Input handling, editing operations (Strategy pattern via `Interface/ITool.cs`) |

### Undo/Redo (Command Pattern)
- `Utility/UndoRedoList.cs` — Command stack with `IUndoRedoCommand` (Execute/Undo)
- `Event/UndoRedoEventArgs.cs` — Notifies UI of stack changes
- Every tool operation (paint, place, delete) pushes a command

### Event-Driven UI
- `Event/RenderEventArgs.cs` — MapPanel requests render with dirty rects
- Property changes via `INotifyPropertyChanged` on model objects
- `Utility/PropertyTracker.cs` — Tracks dirty state for save prompts

## Data & Control Flow

### Application Startup
```
Program.cs → MainForm constructor
  → Load settings (Properties/Settings.settings)
  → Initialize Steamworks.NET (SteamAPI.Init)
  → Load MRU (Utility/MRU.cs)
  → Create default Map (Model/Map.cs)
  → Set active GamePlugin (TD or RA)
  → Show MapPanel with CellGrid rendering
```

### Map Editing Loop
```
User Input (MapPanel mouse/keyboard)
  → Active ITool.MouseDown/MouseMove/MouseUp
  → Tool mutates Model (Map, CellGrid, Objects)
  → Tool creates UndoRedoCommand → pushes to UndoRedoList
  → MapPanel.Invalidate(dirtyRect) → RenderEventArgs
  → MapRenderer.Draw(Map, Graphics) → tile/object/overlay rendering
```

### Map Format (Model/Map.cs)
- **Sections**: `BasicSection` (metadata), `TheaterType`, `MapSection` (tiles), `OverlaySection`, `SmudgeSection`, `TriggerSection`, `TeamTypeSection`, `WaypointSection`, `BriefingSection`, `SteamSection`
- **CellGrid**: 2D array of `Cell` with `TemplateType` (base terrain), `OverlayType` + data, `SmudgeType` + data
- **Objects**: Buildings, Units, Infantry — each with position, owner (House), facing, health, custom properties
- **Triggers**: Event/Condition/Action triples (mirrors engine `TriggerClass`)

### Game Data Loading
- **INI Parser**: `Utility/INI.cs` — Parses `rules.ini`, `art.ini` for type definitions
- **Type Registries**: `*Types.cs` files (e.g., `RedAlert/BuildingTypes.cs`) — Static dictionaries mapping ID → `ITechnoType` with name, icon, prerequisites, cost
- **Theaters**: `TheaterType.cs` — Tileset definitions per theater (Temperate, Snow, Urban, etc.)
- **Tilesets**: `Utility/TilesetManager.cs` + `Tileset.cs` — Loads `.tem`/.`tem` data from game `.mix` archives via `Utility/Megafile.cs`

### Steam Workshop Integration
- `Utility/SteamworksUGC.cs` — High-level wrapper for UGC (User Generated Content)
- `Steamworks.NET/autogen/` — **Auto-generated** P/Invoke bindings for Steamworks SDK (ISteamUGC, ISteamRemoteStorage, etc.)
- `Dialogs/SteamDialog.cs` — Publish UI: title, description, tags, preview image, visibility
- **Workflow**: Serialize map → compress → `SteamUGC.CreateItem` → `StartItemUpdate` → `SetItemContent` → `SubmitItemUpdate`

## Integration Points

### Dependencies
| Package | Version | Purpose |
|---------|---------|---------|
| `Newtonsoft.Json` | 12.0.3 | Steam workshop metadata, settings serialization |
| `Pfim` | 0.9.1 | Image format decoding (DDS, TGA, etc.) for preview/thumbnails |
| `System.ValueTuple` | 4.5.0 | Tuple syntax support (C# 7.3) |
| `Steamworks.NET` | Source | Steam API P/Invoke — **vendored in repo** (`Steamworks.NET/`) |
| `steam_api.dll` | Runtime | Native Steam client library (copied to output) |

### Native Interop
- `Steamworks.NET/InteropHelp.cs` — Marshaling helpers (string↔UTF8, struct↔IntPtr)
- `Steamworks.NET/NativeMethods.cs` — `DllImport("steam_api.dll")` signatures
- `Steamworks.NET/CallbackDispatcher.cs` — Routes Steam callbacks to managed delegates on threadpool

### Game Engine Data Consumption
- Reads **same INI formats** as `REDALERT/` and `TIBERIANDAWN/` engines
- `Utility/Megafile.cs` / `MegafileBuilder.cs` — Reads/writes Westwood `.mix` archives
- `Utility/WWCompression.cs` — Westwood LZSS compression (for map pak files)
- `Utility/TGASharpLib.cs` — TGA texture loading (also used by `SCRIPTS/tgautil.py`)

### Output
- **Map files**: Custom binary format (`.map`) compatible with game engines
- **Steam Workshop**: Published as UGC items (AppID 1213210)
- **No code generation** — editor is pure consumer of engine data formats

## Subdirectory Maps

| Directory | Responsibility |
|-----------|----------------|
| `Controls/` | Reusable WinForms user controls: `MapPanel` (main viewport), property inspectors, custom combo boxes, tooltips |
| `Dialogs/` | Modal dialogs: map settings, new map, triggers, team types, Steam publish, error/invite messages |
| `Event/` | Event args: `RenderEventArgs` (paint), `UndoRedoEventArgs` (stack changes) |
| `Interface/` | Abstractions: `IGamePlugin`, `ITool`, `IWidget`, `ICellOccupier`, `ICellOverlapper`, `ITechnoType`, `INamedType`, `IBrowsableType` |
| `Model/` | **Core data model**: `Map`, `CellGrid`, `CellMetrics`, `Building/Unit/Infantry`, `Trigger`, `TeamType`, `Waypoint`, `Template/Overlay/Smudge` types, `House`, sections |
| `Properties/` | Assembly metadata, resources (`Resources.resx`), user settings (`Settings.settings`) |
| `RedAlert/` | RA-specific `IGamePlugin` impl + type registries (`*Types.cs`), constants, direction/enum mappings |
| `Render/` | `MapRenderer.cs` — GDI+ rendering pipeline: base tiles → overlays → smudges → objects → grid → selection |
| `Resources/` | Embedded icons, default preview image |
| `Steamworks.NET/` | **Vendored Steamworks.NET** — autogen bindings, callback dispatcher, native interop, type definitions |
| `TiberianDawn/` | TD-specific `IGamePlugin` impl + type registries (parallel structure to `RedAlert/`) |
| `Tools/` | **Editing tools** (Strategy pattern): `TerrainTool`, `BuildingTool`, `UnitTool`, `InfantryTool`, `OverlayTool`, `SmudgeTool`, `TemplateTool`, `WallsTool`, `CellTriggersTool`, `WaypointsTool`, `ViewTool`, `ResourcesTool` — each with dedicated dialog |
| `Utility/` | **Shared helpers**: `INI` parser, `Megafile` (`.mix` archive), `TilesetManager`, `TextureManager`, `UndoRedoList`, `PropertyTracker`, `CRC`, `TeamColorManager`, `SteamworksUGC`, `ExtensionMethods`, `WWCompression`, `TGASharpLib`, `GameTextManager`, `MRU` |
| `Widgets/` | Composite UI: `NavigationWidget` (minimap/overview) |

---

## Key Files Reference
| File | Role |
|------|------|
| `Program.cs` | Entry point, STAThread, application initialization |
| `MainForm.cs` | Main window, menu/toolbar, tool switching, plugin management, MRU |
| `Globals.cs` | Global state: active plugin, current map, undo/redo list, settings |
| `Interface/IGamePlugin.cs` | **Critical abstraction** — enables dual-game support |
| `Model/Map.cs` | Root model — sections, serialization, validation |
| `Model/CellGrid.cs` | Spatial index, cell access, bounds checking |
| `Controls/MapPanel.cs` | Viewport — input routing to tools, render invalidation |
| `Render/MapRenderer.cs` | Drawing pipeline — layered rendering order |
| `Utility/INI.cs` | INI parsing (rules.ini, art.ini, map.ini) |
| `Utility/Megafile.cs` | `.mix` archive read/write (game asset access) |
| `Utility/SteamworksUGC.cs` | Workshop publish/update/query wrapper |
| `Utility/UndoRedoList.cs` | Command stack — enables reversible editing |
| `TiberianDawn/GamePlugin.cs` / `RedAlert/GamePlugin.cs` | Game-specific type data, map I/O |
| `Steamworks.NET/Steam.cs` | SteamAPI init/shutdown, callback dispatch |