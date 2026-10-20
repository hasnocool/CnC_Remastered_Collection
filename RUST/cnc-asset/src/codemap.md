# RUST/cnc-asset/src/

## Responsibility

Modules declared in `lib.rs` by responsibility: format parsers (`archive`, `shape`, `palette`, `audio`, `string_table`), converters/batch pipeline (`convert`), CLI (`cli.rs` as `[[bin]] cnc-asset-cli`), runtime-friendly outputs (`atlas`, `tileset_pack`, `registry`), pack loaders (`classic_pack`).

- `classic_pack.rs` — typed read-only loader for `assets/packs/cnc32_assets_v6_2_1`: `Cnc32Pack` (manifest schema 1.3.0 + `ui/ui_asset_index_v6_2.json`), `mask_bits_value`, no-dep PNG helpers (`decode_png_rgba`/`crop_rgba`), plus `PackTileset` — a single deterministic RGBA atlas composed from the editor-kit atlases (terrain_base/transition, road, shore, cliff, wall, resource, bridge, decoration), stable per-sprite-id addressing via `rect`/`blob_rect`/`road_rect`/`shore_rect`, and blob helpers `valid_blob_masks` (47 masks)/`repair_blob_mask`/`blob_mask_is_valid`. Composed rects are in composed-atlas coordinates, not the manifest's original `atlas_rect`.

## Design

- `archive.rs` (230 L) — `MixArchive::parse/open`, `MixEntry` offsets honour the name-table boundary and compressed entry lengths; `extract/extract_all`.
- `shape.rs` (169 L) — `ShpFile`/`ShpFrame`: header parse, RLE decode, per-frame bounds, palette→RGBA.
- `palette.rs` (77 L) — `PalFile`: 768-byte RGB, gamma correction, grayscale, round-trip.
- `audio.rs` (248 L) — `VocFile` (Creative Voice blocks 0x00–0x0A), `AudFile` (Westwood ADPCM-style frames).
- `string_table.rs` (17 L) — `CsfFile` label/string table.
- `registry.rs` (28 L) — `AssetRegistry`: resolves logical asset names to files.
- `convert.rs` (1192 L) — the real workhorse: `SpriteSheet`, `BatchOptions` (naming, scale, trim), `BatchReport` (per-file results), functions producing PNG frames/sheets, JSON manifests, WAV/OGG transcodes; documented as "shared library behind cnc-asset-cli".
- `cli.rs` (341 L) — clap `Commands` enum (`Mix`, `Shp`, `Voc`, `Pal`, `Batch`, `GenerateAtlas`) mapping args to `convert` calls.
- `atlas.rs` (408 L) — `SpriteAtlas::generate`/`load`, `SpriteFrame` rectangles, `AtlasStyle` presets; written as `atlas.png` + `atlas.json`.
- `tileset_pack.rs` (505 L) — `TilesetPack` for `assets/packs/rts_16x16_tileset/<palette>/` — 8 category sheets (terrain, transitions, water_edges, cliffs, roads_bridges, walls_fences, decorations, resources_fx).

## Flow

CLI/programmatic caller picks a module; parser produces a struct (`MixArchive`, `ShpFile`, `PalFile`, `VocFile`/`AudFile`, `CsfFile`); `convert` writes derived artifacts to disk. `GenerateAtlas`/`TilesetPack` skip the original formats and build/load atlas data directly for the renderer.

## Integration

- Consumed by both engine crates and `cnc-launcher` (atlas + tileset pack); `cnc-core` supplies low-level primitives only.
- No cross-module hidden state; modules are independent, side-effectful only when writing converted files.
