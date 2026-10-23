# CnCTDRAMapEditor/Tools/Dialogs/

## Responsibility
Small companion `Form` dialogs for the palette tools (namespace `MobiusEditor.Tools.Dialogs`). Each active tool gets exactly one modeless companion window that hosts its type-palette / options and is shown as the tool's `activeToolForm`.

## Design
- **Factory/selector shell pattern:** dialogs are thin View-shells exposing their controls via public properties (e.g. `GenericToolDialog.GenericTypeComboBox` → `TypeComboBox`, `GenericTypeMapPanel`; `ObjectToolDialog.ObjectTypeComboBox`, `ObjectProperties`; `TerrainToolDialog.TerrainTypeComboBox`, `TerrainProperties`) — `MainForm` instantiates the right dialog per `ToolType` (the Factory role), populates the combo from the plugin's type registry (`plugin.Map.OverlayTypes`, `InfantryTypes`, `UnitTypes`, `BuildingTypes`, filtered by theater/placeability), and constructs the tool from the exposed controls. All dialogs live only as long as the tool.
- **One dialog per control set:** `GenericToolDialog` (reused by Smudge/Overlay/Walls — a generic type selector), `ObjectToolDialog` (Infantry/Unit/Building — type selector + object-property grid), `TerrainToolDialog` (terrain + `TerrainProperties`), `ResourcesToolDialog` (resource brush: `TotalResourcesLbl`, `ResourceBrushSizeNud`, `GemsCheckBox`), `TemplateToolDialog` (`TemplateTypeListView` + `TemplateTypeMapPanel` — the only template-based tool), `WaypointsToolDialog` (`WaypointCombo`), `CellTriggersToolDialog` (`TriggerCombo`).
- **Position persistence:** every dialog persists its own `Location` to `Properties.Settings.Default.ToolDialogPosition` in the constructor and on `OnMove`, so the palette floats where the user left it.

## Flow
`MainForm.ActivateTool` (`ToolType.Map/Smudge/Overlay/Resources/Terrain/Infantry/Unit/Building/Wall/Waypoints/CellTriggers`) → `new <X>Dialog()` → configure combos/lists from the current `IGamePlugin` map data → construct the corresponding `Tools/*Tool` passing the dialog's combo/map-panel/labels + `UndoRedoList` → keep both alive until tool switch; dialog properties are read back during tool operation (e.g. resources brush size).

## Integration
Bridges `MainForm` ⇄ `Tools/*Tool`; the tools never reference `Form` internals beyond the publicly exposed controls, keeping View and interaction logic separate. Depends on `Controls/TypeComboBox`, `Controls/MapPanel`, `Properties/Settings`. Note `GenericToolDialog` is intentionally generic — one instance shape covers three tools, so combo population (types + flags) is entirely the caller's job.
