# RUST/cnc-asset/

## Responsibility

Asset pipeline crate: parsers for the Westwood-era formats shipped with the Remastered Collection (MIX/SHP/PALETTE/VOC/AUD/CSF), converters to runtime formats (PNG frames, sprite sheets, JSON manifests, WAV/OGG), procedural and AI-sprite atlas generation, and the RTS 16×16 tileset loader. Ships the `cnc-asset-cli` binary (see BUILD_STATUS Phase 2 + Phase 4: complete, 26 unit + 8 end-to-end integration tests).

## Design

- Modular format parsers, each in its own file with round-trip tests: `archive.rs` (`MixArchive`/`MixEntry`, MIX v1/v2 with zlib, name tables), `shape.rs` (`ShpFile`/`ShpFrame`, frame headers + RLE), `palette.rs` (`PalFile`, 768-byte RGB + gamma/grayscale), `audio.rs` (`VocFile` blocks 0x00–0x0A, `AudFile` Westwood audio), `string_table.rs` (`CsfFile` CSF strings), `registry.rs` (`AssetRegistry`).
- Conversion layer `convert.rs` (1192 lines, largest): shared library behind the CLI — `BatchOptions`/`BatchReport`/`SpriteSheet`; binary `cli.rs` (341 lines) defines `Commands`: `Mix`, `Shp`, `Voc`, `Pal`, `Batch`, `GenerateAtlas`.
- Output-side builders: `atlas.rs` (`SpriteAtlas`, `SpriteFrame`, `AtlasStyle` — procedural replacement spritesheet since original SHP/VXL art is not shipped, PNG+JSON round-trip) and `tileset_pack.rs` (505 lines; `TilesetPack` loading `assets/packs/rts_16x16_tileset/<palette>/` category sheets with no gutters).
- Audio conversion: depends on `symphonia` (vorbis/ogg), `vorbis_rs`, `hound` (wav), `image` for PNG; parsing uses `nom`, `byteorder`, `flate2`, `lzw`.

## Flow

Input: files from the Remastered Collection install (e.g. `X.MIX`, `*.shp`, `*.pal`, `*.voc`, `*.aud`). `cli.rs` dispatches a `Commands` subcommand → `convert::*/atlas` helper performs decode→re-encode → output PNG/JSON/WAV/OGG or an extracted directory(`mix -i X.MIX -o DIR [--list]`). `tests/cli.rs` invokes the actual binary end-to-end with synthesized format bytes.

## Integration

- Depends on: `cnc-core` (byte/serialization/palette primitives) only.
- Consumed by `cnc-tiberian-dawn`, `cnc-red-alert` and `cnc-launcher` (atlas/tileset loading), and by `scripts/generate_assets.sh` (CLI `GenerateAtlas` produces `assets/atlas.png` + `atlas.json` that the renderer uses as texture slot 0).
