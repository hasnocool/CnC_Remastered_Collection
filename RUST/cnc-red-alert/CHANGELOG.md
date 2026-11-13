## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- `ore_ground` tile family and ore FX sprites scaling with remaining ore (resources_fx sheet)
- Shore/river tile families (`shore_*`, `river_flow_*`) with asymmetric quadrant mirroring
- Sand shore and grass/sand/rock transition rows; lazy-loaded RTS tileset pack with loader diagnostics
- Optional `briefing_image` scenario key (`[Map] BriefingImage`, with `BriefPicture` as an accepted alias) parsed as `Option<String>`; original scenarios never set it, so they parse to `None` (2 tests)
