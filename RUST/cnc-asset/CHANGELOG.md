## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- VQA demuxer (video.rs): Westwood VQA 1.0 container walk, VQHD header parsing, full codebooks (CBF0/CBFZ), partial codebooks (CBP0/CBPZ), column-major 2x2-block full-frame decode (VQFL/VQFR delta frames), and raw PCM audio (SND0) plus headered ADPCM audio (SND1/SND2).
- Default RTS 16x16 tileset pack loader (TilesetPack, assets/packs/rts_16x16_tileset) surfaced as the default tileset, with CNC_TILESET_DIR/CNC_TILESET_PALETTE overrides and loader diagnostics.

#### Removed
- Invented VXL voxel decoder: VXL is not a Westwood format used by either engine; the todo-45 scope line was based on a stale audit row. The decoder was removed rather than shipping an invented format.
