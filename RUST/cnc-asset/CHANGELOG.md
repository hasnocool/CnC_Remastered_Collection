## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### Unreleased
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- VQA demuxer (`video.rs`): Westwood VQA 1.0 container walk, `VQHD` header parsing, full codebooks (`CBF0`), column-major 2×2-block full-frame decode (`VQFL`), and raw PCM audio (`SND0`); compressed variants (`CBP0/CBPZ/CBFZ/VQFR/SND1/SND2`) are detected and reported as typed `UnsupportedVariant` errors pending a format80 codec. 6 synthetic-vector tests
- Default RTS 16x16 tileset pack loader (`TilesetPack`, `assets/packs/rts_16x16_tileset`) surfaced as the default tileset, with `CNC_TILESET_DIR`/`CNC_TILESET_PALETTE` overrides and loader diagnostics
