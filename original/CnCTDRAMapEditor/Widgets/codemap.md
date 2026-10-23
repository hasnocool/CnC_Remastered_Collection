# CnCTDRAMapEditor/Widgets/

## Responsibility
Hosted overlay widgets for the map canvas, namespace `MobiusEditor.Widgets`. Currently a single widget: `NavigationWidget`, which tracks the mouse over the map grid and renders the cell-hover highlight.

## Design
- **Decorator/composition over `MapPanel`:** the widget subscribes to the panel's MouseDown/MouseMove events, keeps its own state, and draws on demand — it owns no child controls. Implements `MobiusEditor.Interface.IWidget` (`void Render(Graphics)` + `IDisposable`), the common interface under which all canvas overlays are registered.
- **Observation:** raises typed `MouseCellChangedEventArgs` (`OldCell`/`NewCell`) via `MouseCellChanged`; tools such as `TerrainTool` subscribe to reposition size-dependent overlays (`MouseoverWidget_MouseCellChanged`).
- **Cell math:** converts client pixels → map coordinates via `mapPanel.ClientToMap` and `CellMetrics`; `MouseSubPixel` computes the in-cell pixel offset against `Globals.PixelWidth/Height` for fine-grained cursor feedback. `MouseoverSize` is normalized to odd dimensions (`value.Width | 1`) so the highlight centers on the cursor.
- **Middle-drag panning:** middle mouse button drag updates `mapPanel.AutoScrollPosition` (viewport scroll), independent from cell selection.
- Hover outline drawn with a static shared `defaultMouseoverPen` (yellow, width 4) — reusable GDI resource, one instance per app domain.

## Flow
`MapPanel.MouseMove` → `OnMouseMove(location)` → recompute `MouseSubPixel`/`MouseCell`; if the cell changed and is within `CellMetrics.Contains`, fire `MouseCellChanged`, then `mapPanel.Invalidate()`. On repaint the panel calls `Render(graphics)`, which inflates the hovered cell rect by `MouseoverSize` and draws the outline.

## Integration
Constructed by `Tools/*Tool` (each tool owns one, e.g. `navigationWidget` referenced throughout `Tools/`), whose lifetime matches tool activation (unsubscribe on dispose to avoid leaked event handlers). Depends on `Controls/MapPanel`, `Model/CellMetrics`, and `Globals`. The Strategy-like tool/widget split means new canvas features implement `IWidget` here and get wired by the owning tool — no registration center exists.
