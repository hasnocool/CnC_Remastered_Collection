# RUST/cnc-procgen/tests/

## Responsibility
Integration tests for cnc-procgen procedural generation algorithms. Validates terrain generation, tileset packing, and campaign content generation determinism.

## Design
- **procgen.rs** (5,149 lines): Tests for procedural terrain, unit sprite, and UI generation — seed determinism, output dimensions, pixel value ranges, biome distribution, transition correctness.
- **tileset_pack.rs** (17,780 lines): Comprehensive tests for tileset pack serialization/deserialization — tile indexing, transition rule encoding, atlas manifest generation, round-trip fidelity, catalog.json validation.

## Flow
1. Tests invoke generators with fixed seeds
2. Validate output structure, dimensions, determinism (bit-identical for same seed)
3. Test tileset pack read/write round-trips
4. Verify atlas manifest correctness for cnc-renderer consumption

## Integration
- Exercises cnc-procgen/src/lib.rs generators (terrain, units, tileset)
- Exercises cnc-procgen/src/tileset/pack.rs serialization
- Validates output consumed by cnc-asset, cnc-renderer, cnc-map-editor
- Ensures procedural assets are cacheable/committable

## Key Files
- procgen.rs - Procedural generation determinism tests
- tileset_pack.rs - Tileset pack serialization tests

