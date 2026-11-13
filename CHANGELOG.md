# Changelog

All notable changes to this workspace will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

## [0.2.0] - 2026-11-13

### Added
- **Rust workspace** (`RUST/`) with 26 crates implementing C&C game logic, rendering, audio, asset handling, map editor, launcher, and procedural generation
- **cnc-core**: Shared foundation crate with smart pointers (`CCPtr`, `CCWeak`, `CUPtr`), fixed-point math, float-bias math, INI parsing, serialization, entity system (Object, TaskForce, TeamType, Team, Trigger), palettes, pointers, rules parsing, shroud/fog-of-war
- **cnc-tiberian-dawn**: Complete TD engine port — game simulation, AI (expert AI with difficulty cadence), savegames (version 2), crate powerups, campaign carry-over, building sabotage/capture, commando/engineer infantry, naval units, zone-based pathfinding, destructible overlays, EVA emissions, trigger vocabulary
- **cnc-red-alert**: Complete RA engine port — mission state machine, multi-house support, per-factory production queues, air/naval/amphibious pathfinding, rules-driven stats/difficulty, verses-armor damage model, infantry specials, crates, vortex, naval classes with bombardment, teamtype planners, projectiles with flight time/splash, carry-over, replay recording, special flags, IQ/difficulty
- **cnc-renderer**: wgpu-based renderer with palette remapping, tile rendering, unit sprite rendering, theater support (Temperate/Snow/Desert)
- **cnc-audio**: ADPCM decoding, VOC/AUD/OGG conversion, EVA emission queues, CSF string tables, briefing/movie playback flow
- **cnc-asset**: Original asset decoding (MIX archives, TEM/VQA/SHP formats), DDS/DXT5 roundtrip, palette handling, shader asset pipeline
- **cnc-procgen**: Logic-faithful port of CNC32 procedural tileset generator (v5.5.0 → v6.2.1): 47-mask terrain autotiles, 20 directed pairs, 940 transitions, unit animation matrices, effect frames, expansion layers (cutscenes, campaigns, briefings, UI), pack builder with validation reports, WCAG contrast auditing
- **cnc-map-editor**: C# WinForms → Rust/egui port: terrain painting, object placement, trigger editing, undo/redo via command pattern, original map import (TD .ini+.bin, RA .mpr), map generation, validation panel, agent studio, headless rendering for QA
- **cnc-launcher**: Game launcher with campaign progression, mission briefing/score flow, multiplayer lobby, options modal (renderer/audio/controls), radar/sidebar depth, movie playback, carry-over persistence
- **cnc-net-directory**: Network directory server/client for matchmaking
- **cnc-ai**: Scriptable AI agent framework with validation, intent parsing, runtime execution
- **cnc-replayer**: Deterministic replay recording/playback with command codecs
- **cnc-sfx**: Sound effect system with priority channels
- **cnc-classic**: Classic game data structures and compatibility
- **cnc-music**: Music playback and playlist management
- **cnc-bench**: Criterion benchmarks for core systems
- **cnc-logs**: Structured logging infrastructure
- **cnc-console**: Developer console with command registry
- **cnc-events**: Event bus for decoupled game systems

### Changed
- Workspace version bumped to 0.2.0
- All crates enforce semantic versioning and documentation standards (README.md, CHANGELOG.md, codemap.md)
- CI/CD pipeline via Forgejo (local.lan) with pre-push hook enforcing fmt, clippy, tests, SemVer, docs
- Tile edge-matching fixes: shore selection via 4-bit land mask, beach transitions via edge-mask columns, off-map sides never land
- Tileset pack `rts_16x16_tileset` regenerated at generation 1.1.0: coarse 2x2 palette clusters, shore art keeps land half sand, concrete/crater/ice signatures moved to per-variant positions
- Map editor: Import Original Map command (toolbar), tileset render example for visual QA, terrain mapping with transition/shore/cliff rows, road/wall overlay auto-connect, sidebar palette compaction
- Procedural generation: briefing art (faction emblems, portraits), campaign expansion layers, cinematic story sequences, tactical briefings/cards/personnel, portraits/UI/controls/mockups

### Fixed
- **cnc-core**: CCWeak use-after-free (ControlBlock tracks strong/weak counts), Clippy deny-level lints (unsafe contracts on ObjectPool::deallocate_drop, pointer decode_ref)
- **cnc-tiberian-dawn**: RA water shore selection now consumes exact 4-bit land mask matching TD engine and map editor conventions; beach cells render sand_shore row with column set to 4-bit water-neighbour mask; off-map sides never land for shore masks
- **cnc-procgen**: v5.9 sprite registration order matches shipped cnc32_assets_v6_2_1 manifest; v5.9/v6.0 timeline notes match shipped pack's 21 entries; v6.0 overview preview no longer listed as timeline supporting asset
- **cnc-map-editor**: Import rejects malformed input without touching model; parse warnings listed in validation panel

### Removed
- Stray generated artifact directories in tiletools

### Security
- All pointer operations marked with explicit `# Safety` contracts
- Savegame format version 2 rejects older saves with explicit "incompatible — start a new mission" message

---

## [0.1.1-classic] - 2026-10-09

### Added
- Original C++ source release: TiberianDawn.dll, RedAlert.dll, MobiusEditor (C# WinForms)
- Initial Rust workspace structure
- Basic CI configuration