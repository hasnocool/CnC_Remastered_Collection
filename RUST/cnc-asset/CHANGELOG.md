## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13
- Enforced semantic versioning and documentation standards (2026-10-19)

#### Added
- VQA demuxer (video.rs): Westwood VQA 1.0 container walk, VQHD header parsing, full codebooks (CBF0/CBFZ), partial codebooks (CBP0/CBPZ), column-major 2x2-block full-frame decode (VQFL/VQFR delta frames), and raw PCM audio (SND0) plus headered ADPCM audio (SND1/SND2).
- Default RTS 16x16 tileset pack loader (TilesetPack, assets/packs/rts_16x16_tileset) surfaced as the default tileset, with CNC_TILESET_DIR/CNC_TILESET_PALETTE overrides and loader diagnostics.
- `original-fetch` CLI command: fetches original C&C Remastered texture archives (`TEXTURES_TD_SRGB.MEG` / `TEXTURES_RA_SRGB.MEG`) from the Steam install into the native asset store (`RUST/assets/original` or `CNC_ORIGINAL_ASSETS_DIR`), writing a `manifest.json` describing the fetched state.

#### Added
- `original-catalog` CLI command: catalogs original assets from the store — lists member templates, and renders preview PNGs and contact sheets via `preview_rendering::render_previews`.

#### Added
- New modules:
  - `iconset.rs` — TD DOS iconset MAP reader: parses classic TD theater `.TEM` iconsets from MIX archives (`TEMPERAT.MIX`, `WINTER.MIX`, `DESERT.MIX`) via the 32-byte `IControl_Type` header occupancy map (`0xFF` = intentional hole).
  - `classic_mix.rs` — Classic Westwood MIX archive parser (TD/RA DOS format): header `{u16 count; i32 payload_size}`, records `{u32 crc; i32 offset; i32 size}`.
  - `preview_rendering.rs` — Template preview and contact-sheet rendering (128x128 cells, magenta checkerboard for missing members, diagonal hatch for TD MAP holes).
  - `original_assets.rs` — Native reading of C&C Remastered Steam assets: locate Steam install (`steam_install_root`), open per-game texture archives, index shipped archives (`index_shipped_archive`), encode shipped-style archives (`write_shipped_archive`), encode standard single-level DXT5 (`encode_dds_dxt5`), normalize multi-mip chains (`normalize_dds`), plus `OriginalAssets::original_store_root()` honoring `CNC_ORIGINAL_ASSETS_DIR` with workspace fallback.

#### Removed
- Invented VXL voxel decoder: VXL is not a Westwood format used by either engine; the todo-45 scope line was based on a stale audit row. The decoder was removed rather than shipping an invented format.

### [0.1.0] - 2026-10-19
- Initial semantic versioning enforcement and documentation standards.
