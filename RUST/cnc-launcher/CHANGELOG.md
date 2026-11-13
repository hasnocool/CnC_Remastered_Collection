## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Tile edge-matching fix round (2026-10-19)

#### Added
- End-of-mission scoreboard (`score.rs`, port of the original `SCORE.CPP` beat): deterministic integer scoring over Game telemetry, staged count-up overlay at mission end
- TD campaign carry-over flow (`CARRY.CPP` port completion): a won mission captures `Carryover::from_game`, persists it next to the campaign progress file (`td-carryover.bin`), and `activate_selected_mission` applies it once when the next mission starts

#### Fixed
- RA water shore selection now consumes the exact 4-bit land mask (N=1, E=2, S=4, W=8) via `water_shore_tile`, matching the TD engine's `water_edges_layout` and the map editor's `water_shore_family` and the pack's `water_edges` art convention (shore family names name where the WATER half sits, so land north selects `shore_south`). The previous priority-based pick was 180°-inverted and collapsed corner/river/ringed cases onto the first matching cardinal
- Beach cells render the `transitions` `sand_shore` row with the column set to the 4-bit mask of water neighbours (`col_override` on `draw_pack_tile_family`); the transitions sheet's columns are edge masks, and the previous deterministic variant hash randomized the sand/water blend direction per cell
- Off-map sides are never land for shore masks, so open map borders stay open water
- Updated the shore/campaign tile tests to the corrected convention and added regression tests for corner, river-flow, ringed, neighbour-mask bit order and Beach edge-mask columns

#### Changed
- Tileset pack `rts_16x16_tileset` regenerated at generation 1.1.0: ground textures use coarse 2x2 palette clusters instead of per-pixel static, shore art keeps its land half sand (was rock grey), and concrete/crater/ice family signatures moved to per-variant positions (was fixed-position marks in every variant)
