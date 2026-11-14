# Original Source to Rust Port Cross-Reference

## Cross-Reference Table

| Original File | Status | Rust Crate(s) | Module/Path | Notes |
|---------------|--------|---------------|-------------|-------|
| TIBERIANDAWN/OBJECT.CPP | Partial | cnc-tiberian-dawn, cnc-core | object.rs, entity.rs | Base object hierarchy |
| TIBERIANDAWN/CELL.CPP | Partial | cnc-tiberian-dawn, cnc-core | cell.rs, map.rs | Cell grid, Tiberium logic |
| TIBERIANDAWN/TECHNO.CPP | Partial | cnc-tiberian-dawn, cnc-core | techno.rs | TechnoClass hierarchy |
| TIBERIANDAWN/FOOT.CPP | Partial | cnc-tiberian-dawn, cnc-core | foot.rs | Infantry movement |
| TIBERIANDAWN/UNIT.CPP | Partial | cnc-tiberian-dawn, cnc-core | unit.rs | Unit movement/combat |
| TIBERIANDAWN/INFANTRY.CPP | Partial | cnc-tiberian-dawn, cnc-core | infantry.rs | Infantry logic |
| TIBERIANDAWN/SCENARIO.CPP | Partial | cnc-tiberian-dawn, cnc-core | scenario.rs | Scenario parsing |
| TIBERIANDAWN/EVENT.CPP | Partial | cnc-tiberian-dawn, cnc-core, cnc-events | event.rs | Event system |
| TIBERIANDAWN/MAP.CPP | Partial | cnc-tiberian-dawn, cnc-core | map.rs | MapCellGrid, tiberium cycle |
| TIBERIANDAWN/MAP.H | Not ported | — | — | No header equivalent |
| TIBERIANDAWN/TEMPLATE.CPP | Partial | cnc-asset | original_templates.rs | 216-entry template table |
| TIBERIANDAWN/CDATA.CPP | Partial | cnc-asset | original_templates.rs | Template registry |
| TIBERIANDAWN/INI.CPP | Not ported | — | — | Scenario parsing incomplete |
| TIBERIANDAWN/MIXFILE.CPP | Partial | cnc-asset | classic_mix.rs | MIX archive parsing |
| TIBERIANDAWN/MIXFILE.H | Not ported | — | — | Header not ported |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | Partial | cnc-asset | iconset.rs | TD .TEM MAP reader |
| TIBERIANDAWN/WIN32LIB/ICONSET.H | Partial | cnc-asset | iconset.rs | Header only |
| TIBERIANDAWN/WIN32LIB/TILE.H | Partial | cnc-asset | iconset.rs | IControl_Type header |
| TIBERIANDAWN/WIN32LIB/PALETTE.CPP | Partial | cnc-asset | palette.rs | Palette loading |
| TIBERIANDAWN/WIN32LIB/PALETTE.H | Partial | cnc-asset | palette.rs | Header only |
| TIBERIANDAWN/WIN32LIB/IFF.CPP | Partial | cnc-asset | iff.rs | IFF container parsing |
| TIBERIANDAWN/WIN32LIB/IFF.H | Partial | cnc-asset | iff.rs | Header only |
| TIBERIANDAWN/WIN32LIB/FONT.CPP | Not ported | cnc-asset | font.rs (planned) | Font loading |
| TIBERIANDAWN/WIN32LIB/FONT.H | Not ported | — | — | Header not ported |
| TIBERIANDAWN/WIN32LIB/SHAPE.H | Not ported | — | — | Header not ported |
| TIBERIANDAWN/WIN32LIB/SHAPE.INC | Not ported | — | — | Assembly not ported |
| TIBERIANDAWN/WIN32LIB/AUDIO.H | Partial | cnc-audio, cnc-sfx | audio.rs | Audio system |
| TIBERIANDAWN/WIN32LIB/LOAD.CPP | Partial | cnc-asset | load.rs | Resource loading |
| TIBERIANDAWN/MAP.CPP | Partial | cnc-tiberian-dawn, cnc-core | map.rs | MapCellGrid, tiberium cycle |
| TIBERIANDAWN/TEMPLATE.CPP | Partial | cnc-asset | original_templates.rs | Template table |
| TIBERIANDAWN/CDATA.CPP | Partial | cnc-asset | original_templates.rs | Template registry |
| TIBERIANDAWN/INI.CPP | Not ported | — | — | Scenario parsing incomplete |
| TIBERIANDAWN/MIXFILE.CPP | Partial | cnc-asset | classic_mix.rs | MIX archive parsing |
| REDALERT/OBJECT.CPP | Partial | cnc-red-alert, cnc-core | object.rs | RA object hierarchy |
| REDALERT/CELL.CPP | Partial | cnc-red-alert, cnc-core | cell.rs | RA cell, Tiberium growth |
| REDALERT/TECHNO.CPP | Partial | cnc-red-alert, cnc-core | techno.rs | RA TechnoClass |
| REDALERT/FOOT.CPP | Partial | cnc-red-alert, cnc-core | foot.rs | RA infantry movement |
| REDALERT/UNIT.CPP | Partial | cnc-red-alert, cnc-core | unit.rs | RA unit logic |
| REDALERT/INFANTRY.CPP | Partial | cnc-red-alert, cnc-core | infantry.rs | RA infantry |
| REDALERT/BUILDING.CPP | Partial | cnc-red-alert, cnc-core | building.rs | RA building logic |
| REDALERT/SCENARIO.CPP | Partial | cnc-red-alert, cnc-core | scenario.rs | RA scenario |
| REDALERT/EVENT.CPP | Partial | cnc-red-alert, cnc-core, cnc-events | event.rs | RA event system |
| REDALERT/LOGIC.CPP | Partial | cnc-red-alert, cnc-core | logic.rs | RA simulation logic |
| REDALERT/MISSION.CPP | Partial | cnc-red-alert, cnc-core | mission.rs | RA mission parsing |
| REDALERT/TEAM.CPP | Partial | cnc-red-alert, cnc-core | team.rs | RA team types |
| REDALERT/WEAPON.CPP | Partial | cnc-red-alert, cnc-core | weapon.rs | RA weapons |
| REDALERT/WARHEAD.CPP | Partial | cnc-red-alert, cnc-core | warhead.rs | RA warheads |
| REDALERT/ANIM.CPP | Partial | cnc-red-alert, cnc-core | anim.rs | RA animation |
| REDALERT/COMBAT.CPP | Partial | cnc-red-alert, cnc-core | combat.rs | RA combat |
| REDALERT/FINDPATH.CPP | Partial | cnc-red-alert, cnc-core | findpath.rs | RA pathfinding |
| REDALERT/SAVELOAD.CPP | Partial | cnc-red-alert, cnc-core | saveload.rs | RA save/load |
| REDALERT/AUDIO.CPP | Partial | cnc-red-alert, cnc-audio, cnc-sfx | audio.rs | RA audio |
| REDALERT/MIXFILE.CPP | Partial | cnc-asset | classic_mix.rs | RA MIX parsing |
| REDALERT/INTERNET.CPP | Partial | cnc-red-alert, cnc-net-directory | internet.rs | RA multiplayer |
| REDALERT/NETDLG.CPP | Not ported | cnc-net-directory | netdlg.rs (planned) | RA network dialog |
| REDALERT/LOADDLG.CPP | Not ported | — | — | UI dialog not ported |
| REDALERT/SOUNDDLG.CPP | Not ported | — | — | UI dialog not ported |
| REDALERT/WIN32LIB/GBUFFER.CPP | Partial | cnc-renderer, cnc-asset | gbuffer.rs | Graphics buffer |
| REDALERT/WIN32LIB/ICONSET.CPP | Partial | cnc-asset | iconset.rs | RA iconset |
| REDALERT/WIN32LIB/PALETTE.CPP | Partial | cnc-asset | palette.rs | RA palette |
| REDALERT/WIN32LIB/TIMER.CPP | Partial | cnc-core | timer.rs | Timer system |
| REDALERT/WIN32LIB/DDRAW.CPP | Partial | cnc-renderer | ddraw.rs | DirectDraw wrapper |
| REDALERT/WIN32LIB/BUFFER.CPP | Partial | cnc-asset | buffer.rs | Buffer management |
| REDALERT/WIN32LIB/IFF.CPP | Partial | cnc-asset | iff.rs | RA IFF parsing |
| REDALERT/WIN32LIB/LOAD.CPP | Partial | cnc-asset | load.rs | RA resource loading |
| REDALERT/WIN32LIB/FONT.CPP | Not ported | cnc-asset | font.rs (planned) | RA font |
| REDALERT/WIN32LIB/KEYBOARD.CPP | Partial | cnc-core | keyboard.rs | Input handling |
| REDALERT/WIN32LIB/MOUSEWW.CPP | Partial | cnc-core | mouse.rs | Mouse input |
| CnCTDRAMapEditor/Program.cs | Partial | cnc-map-editor, cnc-asset-editor | program.rs | Entry point |
| CnCTDRAMapEditor/MainForm.cs | Partial | cnc-map-editor, cnc-asset-editor | main_form.rs | Main window |
| CnCTDRAMapEditor/Model/Map.cs | Partial | cnc-map-editor, cnc-asset-editor | map.rs | Map model |
| CnCTDRAMapEditor/Render/MapRenderer.cs | Partial | cnc-map-editor, cnc-asset-editor | renderer.rs | Rendering |
| CnCTDRAMapEditor/Model/CellGrid.cs | Partial | cnc-map-editor, cnc-asset-editor | cell_grid.rs | Grid system |
| CnCTDRAMapEditor/Model/CellMetrics.cs | Partial | cnc-map-editor, cnc-asset-editor | cell_metrics.rs | Cell metrics |
| CnCTDRAMapEditor/Interface/IGamePlugin.cs | Partial | cnc-map-editor, cnc-asset-editor | plugin.rs | Plugin interface |
| CnCTDRAMapEditor/Interface/ITool.cs | Partial | cnc-map-editor, cnc-asset-editor | tool.rs | Tool interface |
| CnCTDRAMapEditor/Model/CellGrid.cs | Partial | cnc-map-editor, cnc-asset-editor | cell_grid.rs | Grid system |
| CnCTDRAMapEditor/Model/BasicSection.cs | Reviewed | cnc-map-editor, cnc-asset-editor | section.rs | Basic section |
| CnCTDRAMapEditor/Model/BriefingSection.cs | Reviewed | cnc-map-editor, cnc-asset-editor | briefing.rs | Briefing |
| CnCTDRAMapEditor/Model/MapSection.cs | Reviewed | cnc-map-editor, cnc-asset-editor | map_section.rs | Map section |
| CnCTDRAMapEditor/Model/Building.cs | Reviewed | cnc-map-editor, cnc-asset-editor | building.rs | Building |
| CnCTDRAMapEditor/Model/Infantry.cs | Reviewed | cnc-map-editor, cnc-asset-editor | infantry.rs | Infantry |
| CnCTDRAMapEditor/Model/Unit.cs | Reviewed | cnc-map-editor, cnc-asset-editor | unit.rs | Unit |
| CnCTDRAMapEditor/Model/Terrain.cs | Reviewed | cnc-map-editor, cnc-asset-editor | terrain.rs | Terrain |
| CnCTDRAMapEditor/Model/Overlay.cs | Reviewed | cnc-map-editor, cnc-asset-editor | overlay.rs | Overlay |
| CnCTDRAMapEditor/Model/Smudge.cs | Reviewed | cnc-map-editor, cnc-asset-editor | smudge.rs | Smudge |
| CnCTDRAMapEditor/Model/Waypoint.cs | Reviewed | cnc-map-editor, cnc-asset-editor | waypoint.rs | Waypoint |
| CnCTDRAMapEditor/Dialogs/MapSettingsDialog.cs | Partial | cnc-map-editor, cnc-asset-editor | settings_dialog.rs | Settings dialog |
| CnCTDRAMapEditor/Dialogs/NewMapDialog.cs | Partial | cnc-map-editor, cnc-asset-editor | new_map_dialog.rs | New map dialog |
| CnCTDRAMapEditor/Dialogs/TeamTypesDialog.cs | Partial | cnc-map-editor, cnc-asset-editor | team_types_dialog.rs | Team types dialog |
| CnCTDRAMapEditor/Dialogs/TriggersDialog.cs | Partial | cnc-map-editor, cnc-asset-editor | triggers_dialog.rs | Triggers dialog |
| CnCTDRAMapEditor/Dialogs/SteamDialog.cs | Partial | cnc-map-editor, cnc-asset-editor | steam_dialog.rs | Steam dialog |
| CnCTDRAMapEditor/Controls/BasicSettings.cs | Partial | cnc-map-editor, cnc-asset-editor | basic_settings.rs | Basic settings |
| CnCTDRAMapEditor/Controls/PlayerSettings.cs | Partial | cnc-map-editor, cnc-asset-editor | player_settings.rs | Player settings |
| CnCTDRAMapEditor/Controls/TerrainProperties.cs | Partial | cnc-map-editor, cnc-asset-editor | terrain_properties.rs | Terrain properties |
| CnCTDRAMapEditor/CnCTDRAMapEditor.csproj | Partial | cnc-map-editor | csproj parsing | Project config |
| CnCTDRAMapEditor/Controls/MapPanel.cs | Partial | cnc-map-editor, cnc-asset-editor | map_panel.rs | Map panel |
| CnCTDRAMapEditor/Controls/BriefingSettings.cs | Partial | cnc-map-editor, cnc-asset-editor | briefing_settings.rs | Briefing settings |
| CnCTDRAMapEditor/Dialogs/ErrorMessageBox.cs | Partial | cnc-map-editor, cnc-asset-editor | error_dialog.rs | Error dialog |
| CnCTDRAMapEditor/Dialogs/InviteMessageBox.cs | Partial | cnc-map-editor, cnc-asset-editor | invite_dialog.rs | Invite dialog |
| CnCTDRAMapEditor/Model/BuildingType.cs | Partial | cnc-map-editor, cnc-asset-editor | building_type.rs | Building type |
| CnCTDRAMapEditor/Model/CellTrigger.cs | Partial | cnc-map-editor, cnc-asset-editor | cell_trigger.rs | Cell trigger |
| CnCTDRAMapEditor/Model/DirectionType.cs | Partial | cnc-map-editor, cnc-asset-editor | direction_type.rs | Direction type |
| CnCTDRAMapEditor/Model/House.cs | Partial | cnc-map-editor, cnc-asset-editor | house.rs | House |
| CnCTDRAMapEditor/Model/HouseType.cs | Partial | cnc-map-editor, cnc-asset-editor | house_type.rs | House type |
| CnCTDRAMapEditor/Model/InfantryType.cs | Partial | cnc-map-editor, cnc-asset-editor | infantry_type.rs | Infantry type |
| CnCTDRAMapEditor/Model/OccupierSet.cs | Partial | cnc-map-editor, cnc-asset-editor | occupier_set.rs | Occupier set |
| CnCTDRAMapEditor/Model/OverlapperSet.cs | Partial | cnc-map-editor, cnc-asset-editor | overlapper_set.rs | Overlapper set |
| CnCTDRAMapEditor/Model/OverlayType.cs | Partial | cnc-map-editor, cnc-asset-editor | overlay_type.rs | Overlay type |
| CnCTDRAMapEditor/Model/SmudgeType.cs | Partial | cnc-map-editor, cnc-asset-editor | smudge_type.rs | Smudge type |
| CnCTDRAMapEditor/Utility/CRC.cs | Partial | cnc-map-editor, cnc-asset-editor | crc.rs | CRC utility |
| CnCTDRAMapEditor/Utility/Megafile.cs | Partial | cnc-map-editor, cnc-asset-editor | megafile.rs | Megafile utility |
| CnCTDRAMapEditor/Utility/TGASharpLib.cs | Partial | cnc-asset | tga.rs | TGA loader |
| CnCTDRAMapEditor/Utility/Tileset.cs | Partial | cnc-map-editor, cnc-asset-editor | tileset.rs | Tileset utility |
| SHARED/ICONSET.CPP | Reviewed | cnc-asset | iconset.rs | IFF container, frames |
| SHARED/TILE.H | Reviewed | cnc-asset | iconset.rs | Control structure |
| SHARED/MIXFILE.CPP | Reviewed | cnc-asset | classic_mix.rs | MIX archive |
| SHARED/IFF.CPP | Partially reviewed | cnc-asset | iff.rs | IFF container |
| SHARED/PALETTE.CPP | Partially reviewed | cnc-asset | palette.rs | Palette loading |
| SHARED/CDATA.CPP | Reviewed | cnc-asset | original_templates.rs | Template registry |
| SHARED/TEMPLATE.CPP | Reviewed | cnc-asset | original_templates.rs | Template logic |
| SHARED/TEMPLATE.H | Reviewed | cnc-asset | original_templates.rs | Template header |
| SHARED/SCRIPTS/tgautil.py | Inventory only | — | — | TGA utility script |
| SHARED/FONT.CPP | Inventory only | cnc-asset | font.rs (planned) | Font handling |
| SHARED/SHAPE.H | Inventory only | — | — | Shape header |
| SHARED/SHAPE.INC | Inventory only | — | — | Shape assembly |
| SHARED/AUDIO.H | Partially reviewed | cnc-audio, cnc-sfx | audio.rs | Audio container |
| SHARED/LOAD.CPP | Partially reviewed | cnc-asset | load.rs | VOC/AUD loading |

## Summary

**Total Mappings: 107** (covering core files across all 4 subdirectories)

**17 Rust Crates at HEAD:**
1. cnc-core — Core engine, entities, events
2. cnc-tiberian-dawn — TD game logic
3. cnc-red-alert — RA game logic
4. cnc-asset — Asset formats (MIX, ICONSET, palette, IFF, templates)
5. cnc-asset-editor — Editor asset handling
6. cnc-map-editor — Map editor model/rendering
7. cnc-renderer — Graphics rendering
8. cnc-audio — Audio system
9. cnc-sfx — Sound effects
10. cnc-music — Music playback
11. cnc-events — Event system
12. cnc-events — Event system
13. cnc-ai — AI/pathfinding
14. cnc-classic — Classic game integration
15. cnc-launcher — Game launcher
16. cnc-net-directory — Network directory
17. cnc-console — Console/debug tools
18. cnc-logs — Logging
19. cnc-bench — Benchmarking
20. cnc-procgen — Procedural generation
