# Grouped Index: Header Files (.H)

## Purpose
All header files from the original source tree grouped by subdirectory for easy navigation.

## Source
Generated from `../02-original-file-inventory.md` - all .H files at pinned commit `f1f0d42`.

## Tiberian Dawn Headers

| File | Status | Rust Mapping |
|------|--------|--------------|
| TIBERIANDAWN/OBJECT.H | Reviewed | cnc-tiberian-dawn, cnc-core |
| TIBERIANDAWN/CELL.H | Reviewed | cnc-tiberian-dawn, cnc-core |
| TIBERIANDAWN/TECHNO.H | Reviewed | cnc-tiberian-dawn, cnc-core |
| TIBERIANDAWN/FOOT.H | Reviewed | cnc-tiberian-dawn, cnc-core |
| TIBERIANDAWN/UNIT.H | Reviewed | cnc-tiberian-dawn, cnc-core |
| TIBERIANDAWN/INFANTRY.H | Reviewed | cnc-tiberian-dawn, cnc-core |
| TIBERIANDAWN/SCENARIO.H | Reviewed | cnc-tiberian-dawn, cnc-core |
| TIBERIANDAWN/EVENT.H | Reviewed | cnc-tiberian-dawn, cnc-core |
| TIBERIANDAWN/MAP.H | Inventory only | — |
| TIBERIANDAWN/TEMPLATE.H | Reviewed | cnc-asset |
| TIBERIANDAWN/CDATA.H | Reviewed | cnc-asset |
| TIBERIANDAWN/INI.H | Inventory only | — |
| TIBERIANDAWN/MIXFILE.H | Inventory only | — |
| TIBERIANDAWN/WIN32LIB/ICONSET.H | Reviewed | cnc-asset |
| TIBERIANDAWN/WIN32LIB/TILE.H | Reviewed | cnc-asset |
| TIBERIANDAWN/WIN32LIB/PALETTE.H | Reviewed | cnc-asset |
| TIBERIANDAWN/WIN32LIB/IFF.H | Partially reviewed | cnc-asset |
| TIBERIANDAWN/WIN32LIB/FONT.H | Inventory only | — |
| TIBERIANDAWN/WIN32LIB/SHAPE.H | Inventory only | — |
| TIBERIANDAWN/WIN32LIB/AUDIO.H | Partially reviewed | cnc-audio, cnc-sfx |
| TIBERIANDAWN/WIN32LIB/LOAD.H | Partially reviewed | cnc-asset |

## Red Alert Headers

| File | Status | Rust Mapping |
|------|--------|--------------|
| REDALERT/OBJECT.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/CELL.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/TECHNO.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/FOOT.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/UNIT.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/INFANTRY.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/BUILDING.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/SCENARIO.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/EVENT.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/LOGIC.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/MISSION.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/TEAM.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/WEAPON.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/WARHEAD.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/ANIM.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/COMBAT.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/FINDPATH.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/SAVELOAD.H | Reviewed | cnc-red-alert, cnc-core |
| REDALERT/AUDIO.H | Reviewed | cnc-red-alert, cnc-audio, cnc-sfx |
| REDALERT/MIXFILE.H | Reviewed | cnc-asset |
| REDALERT/INTERNET.H | Reviewed | cnc-red-alert, cnc-net-directory |
| REDALERT/NETDLG.H | Inventory only | — |
| REDALERT/LOADDLG.H | Inventory only | — |
| REDALERT/SOUNDDLG.H | Inventory only | — |
| REDALERT/WIN32LIB/GBUFFER.H | Reviewed | cnc-renderer, cnc-asset |
| REDALERT/WIN32LIB/ICONSET.H | Reviewed | cnc-asset |
| REDALERT/WIN32LIB/PALETTE.H | Reviewed | cnc-asset |
| REDALERT/WIN32LIB/TIMER.H | Reviewed | cnc-core |
| REDALERT/WIN32LIB/DDRAW.H | Reviewed | cnc-renderer |
| REDALERT/WIN32LIB/BUFFER.H | Reviewed | cnc-asset |
| REDALERT/WIN32LIB/IFF.H | Reviewed | cnc-asset |
| REDALERT/WIN32LIB/LOAD.H | Reviewed | cnc-asset |
| REDALERT/WIN32LIB/FONT.H | Inventory only | — |
| REDALERT/WIN32LIB/KEYBOARD.H | Reviewed | cnc-core |
| REDALERT/WIN32LIB/MOUSEWW.H | Reviewed | cnc-core |

## Map Editor Headers (.cs files)

| File | Status | Rust Mapping |
|------|--------|--------------|
| All .cs files in CnCTDRAMapEditor/ | Various | cnc-map-editor, cnc-asset-editor |

## Shared Headers

| File | Status | Rust Mapping |
|------|--------|--------------|
| SHARED/TILE.H | Reviewed | cnc-asset |
| SHARED/TEMPLATE.H | Reviewed | cnc-asset |
| SHARED/SHAPE.H | Inventory only | — |

---

*Total: ~90 header files documented*


## Where it fits in the game
This index provides a navigable reference to all header files across the original source tree.

## Classes, structs, enums and important data
All header files organized by subdirectory and game variant.

## Functions and methods
N/A - index file

## Inputs, outputs and side effects
N/A - index file

## Calls into / called by
N/A - index file

## Related original files
All .H files from TIBERIANDAWN/, REDALERT/, CnCTDRAMapEditor/, SHARED/

## Rust port mapping
Map to Rust crates: cnc-asset

## Edge cases and compatibility notes
Some headers are inventory-only (not ported). See individual pages for details.

## Evidence and review status
Status: Inventory only
Source: ../02-original-file-inventory.md at f1f0d42
