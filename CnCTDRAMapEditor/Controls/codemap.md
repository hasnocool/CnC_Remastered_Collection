# CnCTDRAMapEditor/Controls/ — Reusable WinForms Controls

## Responsibility
Reusable UI components for the map editor. Each control encapsulates a specific editing concern: the main map viewport, property inspectors for terrain/objects/players, and custom input controls.

## Design Patterns

### MapPanel (Primary Viewport)
**`MapPanel.cs`** — Custom `UserControl`, **not** a `PictureBox`. Owns:
- **Input routing**: Mouse/keyboard → active `ITool` (from `Tools/`)
- **Render loop**: `OnPaint` → `RenderEventArgs` → `MapRenderer.Draw()`
- **Viewport management**: Pan (middle-mouse), zoom (Ctrl+wheel), center-on-selection
- **Selection rendering**: Highlights selected objects, shows placement preview ghost
- **Grid overlay**: Toggleable cell grid, coordinate labels

**Double-buffering**: `SetStyle(ControlStyles.AllPaintingInWmPaint | ControlStyles.UserPaint | ControlStyles.OptimizedDoubleBuffer, true)`

### Property Inspectors (Data-Bound)
| Control | Binds To | Edits |
|---------|----------|-------|
| `TerrainProperties.cs` | `Cell` (via `MapPanel.SelectedCell`) | Base template, overlay, smudge |
| `ObjectProperties.cs` | `Building`/`Unit`/`Infantry` (selection) | Type, owner, facing, health, veterancy, custom fields |
| `PlayerSettings.cs` | `House[]` (map players) | Color, name, AI/IQ, credits, team, handicap |
| `BasicSettings.cs` | `BasicSection` (map metadata) | Name, author, description, dimensions, theater |

**Pattern**: Each inspector subscribes to `INotifyPropertyChanged` on model → updates UI; UI changes → mutate model → push undo command.

### Custom Input Controls
| Control | Purpose |
|---------|---------|
| `TypeComboBox.cs` | `ComboBox` populated with `ITechnoType` — shows icon + name, filters by category (building/unit/infantry/aircraft) |
| `PropertiesComboBox.cs` | Dynamic property grid for object-specific custom fields |
| `MenuButton.cs` | `Button` with attached `ContextMenuStrip` — toolbar dropdowns |
| `ImageTooltip.cs` | `Component` — shows preview image on hover (tileset icons, unit portraits) |

## Data & Control Flow

### MapPanel Event Flow
```
MouseDown → MapPanel.OnMouseDown
  → Convert screen → cell coords (CellMetrics)
  → ActiveTool.MouseDown(cell, modifierKeys)
    → Tool mutates Model
    → Tool creates UndoCommand → Globals.UndoRedo.Push()
  → Invalidate(dirtyRect)

Paint → MapPanel.OnPaint
  → RenderEventArgs(Graphics, Viewport, Zoom)
  → MapRenderer.Draw(Map, args)
  → Draw selection/preview overlays
```

### Property Inspector Binding
```
SelectionChanged (MapPanel/Dialog)
  → Inspector.Bind(targetObject)
    → Subscribe target.PropertyChanged → UpdateUI()
    → UI control.ValueChanged → target.Property = value
      → PropertyTracker.MarkDirty()
      → UndoCommand pushed (if not already tracking)
```

## Integration Points

### Dependencies
- `Model/` — `Map`, `CellGrid`, `Cell`, `Building`, `Unit`, `Infantry`, `House`, `ITechnoType`
- `Interface/` — `ITool`, `ITechnoType`, `INamedType`
- `Render/` — `MapRenderer`
- `Utility/` — `PropertyTracker`, `UndoRedoList`, `TeamColorManager`, `TextureManager`
- `Event/` — `RenderEventArgs`, `UndoRedoEventArgs`

### Consumers
- `MainForm.cs` — Hosts `MapPanel` in split container, toolbars use `MenuButton`, dialogs use inspectors
- `Tools/*Tool.cs` — Receive input via `MapPanel` → `ITool` interface
- `Dialogs/*Dialog.cs` — Embed inspectors (`ObjectProperties`, `PlayerSettings`, etc.)

## Key Files
| File | Role |
|------|------|
| `MapPanel.cs` | **Main viewport** — input, render, viewport, selection |
| `MapPanel.Designer.cs` | Designer-generated layout |
| `TerrainProperties.cs` | Terrain/overlay/smudge editor for selected cell |
| `ObjectProperties.cs` | Object editor (building/unit/infantry) |
| `PlayerSettings.cs` | Player/faction configuration grid |
| `BasicSettings.cs` | Map metadata editor |
| `TypeComboBox.cs` | Icon+name combo for type selection |
| `PropertiesComboBox.cs` | Dynamic property grid |
| `MenuButton.cs` | Toolbar dropdown button |
| `ImageTooltip.cs` | Hover preview images |