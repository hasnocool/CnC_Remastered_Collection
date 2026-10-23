# CnCTDRAMapEditor/Tools/ — Editing Tools (Strategy Pattern)

## Responsibility
Concrete implementations of `Interface/ITool.cs` — each tool handles a specific editing mode (terrain painting, object placement, trigger editing, etc.). Tools are **stateless strategies** activated by `MainForm` toolbar buttons; they receive input from `MapPanel` and mutate the `Model`.

## Design Patterns

### ITool Interface (Strategy)
```csharp
interface ITool {
    string Name { get; }
    ToolCategory Category { get; }  // Terrain, Object, Trigger, View, etc.
    Cursor Cursor { get; }
    void Activate(MapPanel panel, Map map);
    void Deactivate();
    void MouseDown(Cell cell, MouseButtons btn, Keys modifiers);
    void MouseMove(Cell cell, Keys modifiers);
    void MouseUp(Cell cell, MouseButtons btn, Keys modifiers);
    void KeyDown(Keys key);
    void KeyUp(Keys key);
    void Render(Graphics g, RenderEventArgs args);  // Overlay rendering (preview, ghost)
}
```
**Lifecycle**: `MainForm` calls `Activate` on tool switch → tool caches `MapPanel` + `Map` refs → handles input → `Deactivate` on switch.

### Tool Categories (Toolbar Groups)
| Category | Tools | Purpose |
|----------|-------|---------|
| **Terrain** | `TerrainTool`, `OverlayTool`, `SmudgeTool`, `TemplateTool`, `WallsTool` | Paint base terrain, overlays (concrete/walls), smudges, templates |
| **Objects** | `BuildingTool`, `UnitTool`, `InfantryTool`, `ResourcesTool` | Place/remove game objects with type picker |
| **Triggers** | `CellTriggersTool`, `WaypointsTool` | Edit trigger events/conditions/actions, place waypoints |
| **View** | `ViewTool` | Pan/zoom only (no mutation) |

### Undo/Redo Integration
Every mutating operation creates an `IUndoRedoCommand`:
```csharp
// Example: TerrainTool painting
var oldTemplate = map.CellGrid[cell].Template;
var cmd = new UndoRedoCommand(
    execute: () => map.CellGrid[cell].Template = newTemplate,
    undo: () => map.CellGrid[cell].Template = oldTemplate
);
Globals.UndoRedo.Push(cmd);
```
- `Utility/UndoRedoList.cs` manages stack
- `Utility/PropertyTracker.cs` auto-generates commands for property changes

### Dialog-Backed Tools
Complex tools spawn dedicated dialogs (in `Tools/Dialogs/`):
| Tool | Dialog | Configures |
|------|--------|------------|
| `BuildingTool` | `ObjectToolDialog` | Building type, owner, facing, health |
| `UnitTool` | `ObjectToolDialog` | Unit type, owner, facing, veterancy |
| `TerrainTool` | `TerrainToolDialog` | Template palette, brush size, fill mode |
| `CellTriggersTool` | `CellTriggersToolDialog` | Trigger event/condition/action builder |
| `TemplateTool` | `TemplateToolDialog` | Template brush, rotation, mirror |

## Data & Control Flow

### Activation Flow
```
MainForm.ToolbarButton.Click
  → MainForm.SetActiveTool(new TerrainTool())
    → OldTool.Deactivate()
    → NewTool.Activate(mapPanel, map)
      → Tool subscribes to MapPanel events (if needed)
      → Tool configures cursor, toolbar state
```

### Input Handling (Example: BuildingTool)
```
MouseDown(cell, Left, None)
  → If placement mode:
       previewGhost = new Building(selectedType, cell, owner, facing)
       MapPanel.Invalidate(ghostBounds)
  → If confirm mode (Shift+Click):
       cmd = PlaceBuildingCommand(map, type, cell, owner, facing)
       Globals.UndoRedo.Push(cmd)
       MapPanel.Invalidate(affectedCells)

MouseMove(cell, None)
  → Update previewGhost position
  → MapPanel.Invalidate(oldGhostBounds ∪ newGhostBounds)

KeyDown(Escape)
  → Clear previewGhost
  → MapPanel.Invalidate(ghostBounds)
```

### Render Overlay
```
MapPanel.Paint
  → MapRenderer.Draw(map, args)
  → ActiveTool.Render(args.Graphics, args)  // Draw preview, ghost, grid highlights
```

## Integration Points

### Dependencies
- `Interface/ITool.cs` — Contract
- `Model/` — `Map`, `CellGrid`, `Building`, `Unit`, `Infantry`, `Cell`, `Trigger`, `Waypoint`, `TemplateType`, `OverlayType`, `SmudgeType`
- `Controls/MapPanel.cs` — Input source, render target, `CellMetrics` for coords
- `Utility/` — `UndoRedoList`, `PropertyTracker`, `TeamColorManager`, `TextureManager`
- `Dialogs/` — Tool-specific dialogs for configuration

### Consumers
- `MainForm.cs` — Tool registry, activation, toolbar binding
- `MapPanel.cs` — Routes input to active tool

## Key Files
| File | Role |
|------|------|
| `TerrainTool.cs` | Base terrain painting — brush, fill, template picker |
| `OverlayTool.cs` | Overlay painting (concrete, walls, rivers) — multi-cell patterns |
| `SmudgeTool.cs` | Smudge painting (scorch, craters) — random variation |
| `TemplateTool.cs` | Template brush — pre-built structure stamps |
| `WallsTool.cs` | Wall drawing — auto-connect, corner detection |
| `BuildingTool.cs` | Building placement — footprint validation, snap-to-grid |
| `UnitTool.cs` | Vehicle placement — single cell, facing |
| `InfantryTool.cs` | Infantry placement — single cell, facing |
| `ResourcesTool.cs` | Tiberium/ore placement — value, growth |
| `CellTriggersTool.cs` | Cell-trigger attachment — event picker |
| `WaypointsTool.cs` | Waypoint placement/editing — path visualization |
| `ViewTool.cs` | Pan/zoom only — no mutation |
| `Dialogs/*ToolDialog.cs` | Configuration UI for each tool |