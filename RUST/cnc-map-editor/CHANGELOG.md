## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### Unreleased
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- `examples/tileset_render.rs`: headless visual-QA harness that renders an editor-style map (grass/concrete/dirt/rock/sand/water with overlays) through the exact `tiles::base_draw`/`sprite_draws` picks and saves a PNG; optional `CNC_TILESET_QA_DUMP=1` prints per-cell tile picks (2026-10-19)
- `tiles.rs` terrain mapping: transition/shore/cliff rows, elevation-based cliff drops and new terrain families
- Road/wall overlay auto-connect (`road_family`/`wall_family` bitmask arms) plus tree/flag/FX sprite placement
- Sidebar palette compaction (terrain three-column, overlay a single row of six chips)
