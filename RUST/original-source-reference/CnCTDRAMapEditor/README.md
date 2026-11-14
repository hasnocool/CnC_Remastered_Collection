# Map Editor Source Guide

Complete documentation of all 286 files in `original/CnCTDRAMapEditor/` at pinned commit `f1f0d42`.

## Core Model

| File | Status | Rust Mapping |
|------|--------|--------------|
| [Program.cs](Program.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [MainForm.cs](MainForm.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/Map.cs](Model/Map.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Render/MapRenderer.cs](Render/MapRenderer.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |

## Grid Primitives & Contracts

| File | Status | Rust Mapping |
|------|--------|--------------|
| [Model/CellGrid.cs](Model/CellGrid.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/CellMetrics.cs](Model/CellMetrics.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Interface/IGamePlugin.cs](Interface/IGamePlugin.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Interface/ITool.cs](Interface/ITool.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |

## Scenario & Placed-Object Models

| File | Status | Rust Mapping |
|------|--------|--------------|
| [Model/BasicSection.cs](Model/BasicSection.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/BriefingSection.cs](Model/BriefingSection.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/MapSection.cs](Model/MapSection.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/Building.cs](Model/Building.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/Infantry.cs](Model/Infantry.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/Unit.cs](Model/Unit.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/Terrain.cs](Model/Terrain.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/Overlay.cs](Model/Overlay.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/Smudge.cs](Model/Smudge.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/Waypoint.cs](Model/Waypoint.cs.md) | Reviewed | cnc-map-editor, cnc-asset-editor |

## Dialogs & Controls

| File | Status | Rust Mapping |
|------|--------|--------------|
| [Dialogs/MapSettingsDialog.cs](Dialogs/MapSettingsDialog.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Dialogs/NewMapDialog.cs](Dialogs/NewMapDialog.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Dialogs/TeamTypesDialog.cs](Dialogs/TeamTypesDialog.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Dialogs/TriggersDialog.cs](Dialogs/TriggersDialog.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Dialogs/SteamDialog.cs](Dialogs/SteamDialog.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Controls/BasicSettings.cs](Controls/BasicSettings.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Controls/PlayerSettings.cs](Controls/PlayerSettings.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Controls/TerrainProperties.cs](Controls/TerrainProperties.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |

## Project & Controls

| File | Status | Rust Mapping |
|------|--------|--------------|
| [CnCTDRAMapEditor.csproj](CnCTDRAMapEditor.csproj.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Controls/MapPanel.cs](Controls/MapPanel.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Controls/BriefingSettings.cs](Controls/BriefingSettings.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Dialogs/ErrorMessageBox.cs](Dialogs/ErrorMessageBox.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Dialogs/InviteMessageBox.cs](Dialogs/InviteMessageBox.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |

## Type Definitions & Spatial Indexes

| File | Status | Rust Mapping |
|------|--------|--------------|
| [Model/BuildingType.cs](Model/BuildingType.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/CellTrigger.cs](Model/CellTrigger.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/DirectionType.cs](Model/DirectionType.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/House.cs](Model/House.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/HouseType.cs](Model/HouseType.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/InfantryType.cs](Model/InfantryType.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/OccupierSet.cs](Model/OccupierSet.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/OverlapperSet.cs](Model/OverlapperSet.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/OverlayType.cs](Model/OverlayType.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Model/SmudgeType.cs](Model/SmudgeType.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |

## Utility Routines

| File | Status | Rust Mapping |
|------|--------|--------------|
| [Utility/CRC.cs](Utility/CRC.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Utility/Megafile.cs](Utility/Megafile.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Utility/TGASharpLib.cs](Utility/TGASharpLib.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |
| [Utility/Tileset.cs](Utility/Tileset.cs.md) | Partially reviewed | cnc-map-editor, cnc-asset-editor |

## Remaining Files (to be documented)

Steamworks.NET autogen, resources, designer files, resx, project files — 280 files total.

## Quick Links

- [System Map](../01-system-map.md)
- [Cross-Reference](../03-rust-port-cross-reference.md)
- [Inventory](../02-original-file-inventory.md)
