# RUST/cnc-asset/

## Responsibility
Asset loading and parsing library for Command & Conquer file formats. Handles Westwood legacy formats (SHP, VXL, PAL, AUD/VOC, MIX archives) plus modern asset pipeline (texture atlases, animation registries, palette remapping). Provides CLI tool (cnc-asset-cli) for batch conversion.

## Design Patterns
- Format-Specific Modules: Each Westwood format has dedicated module (shp, vxl, pal, aud, mix, etc.)
- Registry Pattern: registry::AssetRegistry centralizes loaded assets with deduplication
- Palette Remapping: Runtime palette swapping via palette::remap for player colors, shadows, transparency
- Atlas Packing: atlas::AtlasBuilder packs sprites into GPU-friendly texture atlases
- Pipeline Stages: convert module orchestrates decode to transform to encode workflows
- CLI-First: cli.rs exposes all operations as scriptable commands

## Data & Control Flow
1. Load: archive::read_mix / shape::load_shp / vxl::load_vxl / audio::load_aud parse raw bytes
2. Decode: Format-specific decoders produce intermediate representations (frames, voxels, samples)
3. Transform: palette::remap, animation::AnimationPlayer, atlas::AtlasBuilder process intermediates
4. Encode/Export: convert writes modern formats (PNG, WAV, JSON manifests, GPU atlas textures)
5. Registry: AssetRegistry caches and deduplicates by content hash

## Public API (Key Re-exports)
- adpcm: AdpcmDecoder, AdpcmError
- anim: AnimationSpec, AnimationInstance, AnimationEvent, FrameEvent, LoopMode, PaletteId
- animation: Animation, AnimationPlayer, AnimFrame, PlaybackMode, frame_to_rgba_remapped
- archive: read_mix, MixArchive, MixEntry
- atlas: AtlasBuilder, Atlas, SpriteEntry
- audio: AudioClip, load_aud, load_voc
- classic_pack: ClassicPack, ClassicPackError
- convert: convert_shp, convert_vxl, convert_audio, convert_palette
- font: Font, FontGlyph, load_fnt
- format80: Format80Decoder, Format80Frame
- original_assets: OriginalAssets, Game, Theater, index_shipped_archive, write_shipped_archive, normalize_dds, encode_dds_dxt5, steam_install_root, MAX_MIP_LEVELS
- original_templates: OriginalTemplate, LandType, ORIGINAL_TEMPLATES
- palette: Palette, remap, player_color_map, shadow_map
- registry: AssetRegistry, AssetHandle
- shape: Shape, ShapeFrame, load_shp
- string_table: StringTable, load_str
- tile_join: JoinTable, JoinCandidate, JoinMaterial, EdgeProfile, EdgeProfiles, material_for, measure
- tileset_pack: TilesetPack, TilesetTile, TilesetPackError
- vxl: Vxl, VxlVoxel, load_vxl
- video: VideoFrame, VideoDecoder

## Integration Points
- Dependencies: cnc-core, byteorder, nom, flate2, lzw, image, audio, hound, symphonia, vorbis_rs, parking_lot, once_cell, clap, thiserror, anyhow, serde, bytemuck, bitflags
- Consumed by: cnc-renderer (atlases, palettes), cnc-tiberian-dawn/cnc-red-alert (game assets), cnc-procgen (generated assets), cnc-asset-cli (batch tooling)
- CLI Binary: cnc-asset-cli (src/cli.rs) for headless asset processing

## Key Files
- src/lib.rs - Module declarations, 20+ format modules re-exported
- src/archive.rs - MIX archive reader (Westwood container format)
- src/shape.rs - SHP frame decoder (2D sprites)
- src/vxl.rs - VXL voxel model decoder (3D units)
- src/palette.rs - PAL palette loading, remapping, player color maps
- src/audio.rs - AUD/VOC audio loading, ADPCM decoding
- src/atlas.rs - Texture atlas builder for GPU upload
- src/animation.rs - Animation playback, frame blending
- src/registry.rs - Asset registry with deduplication
- src/convert.rs - High-level conversion pipelines
- src/cli.rs - Command-line interface
- src/tileset_pack.rs - Tileset packaging for terrain
- src/tile_join.rs - Measured seamless tile matching for the original art: classifies each icon's four edges (transparent/opaque/water), scores candidates against an eight-neighbour material context. Overlay slivers (≥92% transparent edges) are excluded from being a base. `measure(game, theater)` decodes every shipped icon once at tileset load
- src/classic_pack.rs - Legacy pack format support

