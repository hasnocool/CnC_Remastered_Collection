# CnCTDRAMapEditor/Event/

## Responsibility
Argument-passing types for two cross-cutting editor events, namespace `MobiusEditor.Event`: `RenderEventArgs` (custom rendering) and `UndoRedoEventArgs` (undo/redo replay). Both are tiny immutable DTOs — no logic, no state mutation.

## Design
- **`RenderEventArgs : EventArgs`** — carries `Graphics` plus an `ISet<Point>` `Cells` (the map cells that must be redrawn). Maps the WinForms paint request onto the game grid so listeners can redraw only dirty cells instead of the whole map. Fired from `Controls/MapPanel` as `PreRender` / `PostRender` events, which `Tools/*Tool` weave into their render pipeline (`TemplateTool`, `ViewTool`).
- **`UndoRedoEventArgs`** — plain (non-`EventArgs`) class carrying the `MapPanel` being displayed and the `Map` being edited. It is the generic parameter `T` of `Utility/UndoRedoList<UndoRedoEventArgs>` (`MainForm.url`), so recorded `Action<UndoRedoEventArgs> Undo/Redo` closures receive enough context to re-apply or roll back a command and invalidate the affected panel region.

## Flow
1. `MapPanel.OnPaint` → `PreRender`/`PostRender` `RenderEventArgs` → each active tool/widget's `Render(Graphics)` draws its overlay for the given cells; panel `Invalidate(Map, Rectangle)` populates the cell set.
2. A tool modification → `mapPanel.Map` change recorded via `UndoRedoList.Track(undo, redo)`; user presses Ctrl+Z/Ctrl+Y → `UndoRedoList` invokes the stored action with its `UndoRedoEventArgs` → the command repatches `Map` and calls `MapPanel.Invalidate` from the arg.

## Integration
Consumed by `Controls/MapPanel` (event source), `Tools/` (render path, each tool embeds `UndoRedoList<UndoRedoEventArgs> url` handed over by `MainForm`), and `Widgets/IWidget` renderers. Note the asymmetry: `RenderEventArgs` derives from `System.EventArgs`; `UndoRedoEventArgs` deliberately does not because it is a command payload, not an event payload.
