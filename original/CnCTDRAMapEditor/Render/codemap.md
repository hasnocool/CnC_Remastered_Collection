# CnCTDRAMapEditor/Render/ — Rendering Pipeline

## Responsibility
**GDI+-based rendering** of the map for the editor viewport. Implements layered drawing order matching the game engines: base terrain → overlays → smudges → objects → grid → selection → tool preview. Pure rendering — no mutation, no game logic.

## Design Patterns

### MapRenderer (Static Pipeline)
**`MapRenderer.cs`** — Static class with `Draw(Map map, RenderEventArgs args)` entry point. Pipeline stages:

```
1. Clear (viewport background)
2. Base Terrain     → MapSection.TemplateType per cell → TilesetManager.GetTile()
3. Overlays         → OverlaySection.OverlayType+Data → TilesetManager.GetOverlay()
4. Smudges          → SmudgeSection.SmudgeType+Data   → TilesetManager.GetSmudge()
5. Map Objects      → Buildings/Units/Infantry → TextureManager.GetSprite() + TeamColorManager.Apply()
6. Grid Overlay     → CellGrid lines (if enabled)
7. Selection        → Highlight selected cells/objects
8. Tool Preview     → ActiveTool.Render() (ghost, brush outline)
```

### Coordinate Systems
| Space | Origin | Units | Conversion |
|-------|--------|-------|------------|
| **Cell** | Top-left (0,0) | Cell indices | `CellMetrics.CellToScreen()` |
| **Screen** | Viewport top-left | Pixels | `CellMetrics.ScreenToCell()` |
| **World** | Map center | Leptons (1/256 cell) | Engine internal |

`CellMetrics.cs` (in `Model/`) handles all projections: isometric diamond → screen rectangle.

### Texture Management
- `TextureManager.cs` (Utility) — Caches `Bitmap` per tileset frame, sprite, icon
- `TilesetManager.cs` (Utility) — Loads `.tem`/`.til` data from game `.mix` via `Megafile.cs`
- **Team color remapping**: `TeamColorManager.Apply(sprite, House)` — swaps remappable palette indices to player color

### RenderEventArgs (Render Context)
```csharp
class RenderEventArgs {
    Graphics Graphics;           // Target GDI+ surface
    Rectangle Viewport;          // Visible screen rect
    float Zoom;                  // 1.0 = 1:1 pixels
    Point ViewOffset;            // World→screen translation
    CellMetrics Metrics;         // Projection helper
    Rectangle DirtyRect;         // Invalidation region (optional)
}
```

## Data & Control Flow

### Draw Call
```
MapPanel.OnPaint(e)
  → args = new RenderEventArgs(e.Graphics, Viewport, Zoom, ViewOffset, Metrics)
  → MapRenderer.Draw(Globals.CurrentMap, args)
```

### Layer Drawing (Simplified)
```csharp
// Base terrain - single pass, batched by tileset
foreach (cell in visibleCells) {
    var tile = TilesetManager.GetTile(map.CellGrid[cell].Template);
    g.DrawImage(tile, Metrics.CellToScreen(cell));
}

// Objects - sorted by Y (painter's algorithm for isometric)
var objects = map.GetAllObjects().OrderBy(o => o.Cell.Y + o.Cell.X);
foreach (obj in objects) {
    var sprite = TextureManager.GetSprite(obj.Type, obj.Facing);
    sprite = TeamColorManager.Apply(sprite, obj.Owner);
    g.DrawImage(sprite, Metrics.CellToScreen(obj.Cell) + obj.RenderOffset);
}
```

### Invalidation
- `MapPanel.Invalidate(dirtyRect)` → `Paint` only redraws `DirtyRect`
- Tools call `Invalidate` on preview change
- `PropertyTracker` marks full map dirty on structural changes

## Integration Points

### Dependencies
- `Model/` — `Map`, `CellGrid`, `Cell`, `Building`, `Unit`, `Infantry`, `TemplateType`, `OverlayType`, `SmudgeType`
- `Utility/` — `TextureManager`, `TilesetManager`, `TeamColorManager`, `CellMetrics`
- `Controls/MapPanel.cs` — Consumer, provides `RenderEventArgs`
- `Interface/ITool.cs` — `ActiveTool.Render()` for preview overlay

### Consumers
- `Controls/MapPanel.cs` — Sole consumer, calls `Draw()` in `OnPaint`

## Key Files
| File | Role |
|------|------|
| `MapRenderer.cs` | **Only file** — entire rendering pipeline, static methods |