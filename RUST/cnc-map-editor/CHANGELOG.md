## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- File→Import Original Map: the toolbar **Import** command loads a retail TD scenario (`.ini` plus its sibling `.bin`) or an RA scenario (`.mpr`/`.ini`), replacing the working map through an undoable `history::ImportOriginalMap` command, so one Ctrl+Z reverts to the previously open map. Parse warnings are listed in the validation panel; malformed input is reported without touching the model (2026-10-24)
- `examples/tileset_render.rs`: headless visual-QA harness that renders an editor-style map (grass/concrete/dirt/rock/sand/water with overlays) through the exact `tiles::base_draw`/`sprite_draws` picks and saves a PNG; optional `CNC_TILESET_QA_DUMP=1` prints per-cell tile picks (2026-10-19)
- `tiles.rs` terrain mapping: transition/shore/cliff rows, elevation-based cliff drops and new terrain families
- Road/wall overlay auto-connect (`road_family`/`wall_family` bitmask arms) plus tree/flag/FX sprite placement
- Sidebar palette compaction (terrain three-column, overlay a single row of six chips)
