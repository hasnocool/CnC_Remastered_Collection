# RUST/cnc-procgen/

## Responsibility
Deterministic procedural asset generation for the C&C Rust port. Generates terrain textures, placeholder unit sprites, campaign expansions (briefings, cutscenes, UI), and tileset packs from seed-driven algorithms. All output is bit-identical for identical seeds - reproducible bakes committed as assets.

## Design Patterns
- Seed-Driven Determinism: All generators take u64 seed; sha2 hashing for domain separation
- Expansion Modules: Versioned campaign content generators (v5.8 cutscenes, v5.9/v6.0/v6.1 campaigns, v6.2 UI)
- Tileset System: tileset:: module generates terrain transitions, cliff edges, shore blending
- Pack Format: tileset::pack:: serializes generated tilesets for cnc-asset consumption
- Font Rendering: fontdue for procedural text in briefings/UI
- CLI Interface: clap commands for each generation target

## Data & Control Flow
1. Seed Input: CLI or library caller provides u64 seed
2. Domain Separation: sha2::Sha256 hashes seed + domain tag (terrain, unit, briefing, etc.)
3. Generate: Domain module produces raw asset data (pixels, voxels, INI text)
4. Package: tileset_pack::write / classic_pack::write / custom serializers
5. Output: Files written to asset directory, consumed by cnc-asset / cnc-renderer

## Public API
- Terrain: generate_terrain(seed, size) -> ImageBuffer<Rgba>
- Units: generate_unit_sprite(seed, unit_type) -> ImageBuffer<Rgba>
- Tileset: tileset::generate(seed) -> TilesetPack
- Expansions:
  - expansion::briefing_v61::generate(seed) -> briefing INI
  - expansion::cutscene_v58::generate(seed) -> cutscene script
  - expansion::campaign_v59/60/61::generate(seed) -> campaign INI
  - expansion::ui_v62::generate(seed) -> UI layout JSON
- Pack: tileset::pack::write_tileset(), tileset::pack::read_tileset()

## Integration Points
- Dependencies: cnc-core, image, serde, sha2, thiserror, clap, fontdue
- Consumed by: cnc-asset (tileset packs), cnc-renderer (terrain textures), cnc-music (briefing music cues), cnc-tiberian-dawn/cnc-red-alert (procedural placeholders)
- CLI: cnc-procgen binary for batch generation

## Key Files
- src/lib.rs - Module declarations, terrain/unit/tileset generators
- src/tileset/mod.rs - Tileset generation, transitions, blending
- src/tileset/pack.rs - Tileset serialization format
- src/expansion/mod.rs - Expansion module registry
- src/expansion/briefing_v61.rs - Briefing generator
- src/expansion/cutscene_v58.rs - Cutscene generator
- src/expansion/campaign_v59.rs / v60.rs / v61.rs - Campaign generators
- src/expansion/ui_v62.rs - UI layout generator

