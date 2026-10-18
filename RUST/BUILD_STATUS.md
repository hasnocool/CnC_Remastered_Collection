# Build Status

_Last verified: cargo 1.98.1 / rustc 1.98.1 on Linux._

## Verification

```
cargo check --workspace --all-targets   # clean (warnings only)
cargo test --workspace                 # 170 passed, 0 failed
```

Per-target tests:

| Target | Tests |
|---|---|
| cnc-core (unit + INI + Rules.ini) | 54 |
| cnc-asset (parsers + converters) | 18 |
| cnc-asset-cli (end-to-end integration) | 7 |
| cnc-tiberian-dawn (game logic + rules integration + lockstep) | 86 |
| cnc-renderer (camera + sprite batch) | 3 |
| cnc-renderer (GPU smoke, ignored unless adapter present) | 2 |

## Phase 2: Asset Pipeline - COMPLETE

### Implemented Parsers
- **archive.rs**: MIX v1/v2 (.mix) - Zlib decompression, name tables, extract/extract_all
  - Fixed payload offset calculation (data starts after the name table) and
    compressed-entry lengths; both layouts covered by tests.
- **shape.rs**: SHP (.shp) - Frame headers, RLE decoding, bounds, palette-to-RGBA
- **audio.rs**: VOC (.voc) - Creative Voice blocks 0x00-0x0A, AUD (.aud) - Westwood Audio, WAV export via hound
- **palette.rs**: PAL (.pal) - 768-byte RGB, gamma correction, grayscale, round-trip test

## Phase 3: Rendering - COMPLETE

### Implemented Components
- **sprite_batch.rs**: Instanced sprite batching (10K capacity), z-order + texture sorting, vertex/index buffers
- **shaders/sprite_batch.wgsl**: Palette remapping (256 colors), alpha blending, nearest-neighbor
- **camera.rs**: Orthographic camera, view/projection matrices, world↔screen, zoom/pan
- **renderer.rs**: Renderer trait, WgpuRenderer (async), SoftwareRenderer fallback

## Phase 4: Asset Converter CLI - COMPLETE

`cnc-asset/src/convert.rs` is the shared library behind the
`cnc-asset-cli` binary; `cnc-asset/tests/cli.rs` exercises the binary itself.

| Command | Behaviour |
|---|---|
| `mix -i X.MIX -o DIR [--list]` | List or extract every MIX entry |
| `shp -i X.SHP -o OUT [-p PAL] [--sprite-sheet] [--opaque]` | Per-frame PNGs, or one horizontal sheet + JSON manifest |
| `voc -i X.voc|X.aud -o X.wav` | VOC/AUD to PCM WAV |
| `pal -i X.PAL -o DIR [--format json|png|css]` | Palette export |
| `batch -i SRCDIR -o OUTDIR -g td|ra [-p PAL] [--sprite-sheet]` | Recursive conversion mirroring the source tree, with per-kind counts |
| `generate-atlas -o DIR [--style classic|neon]` | Placeholder sprite atlas |

Details:
- SHP frames honour palette index 0 as transparent (disable with `--opaque`).
- Sprite sheets are packed horizontally and described by a JSON manifest
  (frame index, x, y, w, h), ready for GPU upload.
- Batch conversion records failures per file and returns a non-zero exit status
  when any asset fails, without aborting the remaining files.
- OGG/Vorbis encoding with loop points is not implemented; audio is exported as WAV.

## Phase 5: Game Logic - Shared Foundation (IN PROGRESS)

### 5.1 Command queue & lockstep - STARTED
- `network.rs` now orders commands deterministically by (frame, player, hash)
  regardless of UDP arrival order.
- `LockstepSession`: validated scheduling, per-frame gating, ordered replay log,
  per-frame checksums, remote desync detection, and replay reconstruction.
- `NetPacket` checksums cover sequence, frame and command payloads.
- Still TODO: packet serialization over UDP, multi-peer broadcast, save/load of replays.

### 5.2 Rules/INI registry - DONE (first pass)
- `cnc-core/src/rules.rs` parses Rules.ini into typed data:
  `GeneralRules` (RULES.CPP defaults), `DifficultyRule` (Easy/Normal/Hard),
  `WeaponRule`, `WarheadRule`, `TypeRule` (units/buildings/aircraft),
  `HouseRule` and the `[Infantry]/[Units]/[Aircraft]/[Buildings]` categories.
- Missing referenced sections are reported as warnings instead of failing.
- INI parser fixes: decimal and 0x-hex integers, numeric/boolean disambiguation,
  case-insensitive key lookup, multi-level inheritance, cycle detection.
- Game integration: `Game::with_rules`/`set_rules`/`load_rules` plus
  `unit_cost`, `building_cost`, `unit_strength`, `building_strength`
  overrides used by production, spawning and building placement.
- INI names follow BDATA/UDATA/IDATA/AADATA: E1, MTNK, HTNK, HARV, JEEP, BGGY,
  ORCA, HELI; FACT, NUKE, PROC, SILO, PYLE, WEAP, HPAD, EYE, TMPL.
- Launcher accepts `--rules Rules.ini` and applies the overrides to the demo game.
- Still TODO: weapon/warhead overrides wired into combat, tech tree/prerequisites,
  and per-faction house sections.

## Core Types (Phase 1) - COMPLETE

- Fixed16_16 (16.16 fixed-point with deterministic trig)
- FloatBias (TD's biased float comparison)
- ClassId (300+ variant RTTI replacement)
- CodePointers/DecodePointers (save/load fixup)
- CCPtr/CCWeak/NoInit (smart pointers)
- ObjectPool (hot allocation)
- INI parser with inheritance and typed Rules registry

## Workspace Structure
- 8 crates: cnc-core, cnc-renderer, cnc-audio, cnc-asset, cnc-tiberian-dawn, cnc-red-alert, cnc-map-editor, cnc-launcher
- All dependencies in workspace Cargo.toml

## Next Steps
1. Complete Phase 5.1: UDP packet serialization, peer broadcast and replay save/load.
2. Complete Phase 5.2: wire weapon/warhead rules into combat and tech-tree prerequisites.
3. Phase 6: expand Tiberian Dawn game logic (houses, missions, network command execution).
