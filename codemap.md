# Repository Atlas: Command & Conquer Remastered Collection

## Project Responsibility
This repository contains the complete source code for the **Command & Conquer Remastered Collection**, comprising three primary components:
1. **TiberianDawn.dll** - The Tiberian Dawn (C&C1) game engine implemented in C++
2. **RedAlert.dll** - The Red Alert game engine implemented in C++
3. **CnCTDRAMapEditor** - A C# Windows Forms map editor application for both games

The codebase is structured for Windows-only compilation targeting the Steam Workshop integration (AppID 1213210). Licensed under GPL v3 with additional terms. The repository is archived for preservation purposes only and does not accept external contributions.

## System Entry Points
| File | Purpose |
|------|---------|
| `original/CnCRemastered.sln` | Primary Visual Studio solution building both game engines (TiberianDawn.dll, RedAlert.dll) |
| `original/CnCTDRAMapEditor.sln` | Visual Studio solution for the C# Map Editor application |
| `README.md` | Build instructions, dependencies, and license information |
| `LICENSE.md` | GPL v3 license with additional terms |
| `original/SCRIPTS/tgautil.py` | Python utility script for TGA texture processing |
| `original/SCRIPTS/download_openra_assets.py` | Optional OpenRA asset downloader (CC-BY-SA/GPL) |

**Build Output:** Compiled binaries are emitted to a root-level `bin/` directory after batch build in Visual Studio 2017 (recommended; later versions may report packing mismatch errors with Windows SDK headers).

**Required Dependencies:** Windows 8.1 SDK, MFC for Visual Studio C++.

---

## Directory Map (Aggregated)

| Directory | Responsibility Summary | Detailed Map |
|-----------|------------------------|--------------|
| `original/CnCTDRAMapEditor/` | **C# Map Editor Application** - Windows Forms desktop application for editing Tiberian Dawn and Red Alert maps. Contains UI controls, dialogs, rendering pipeline, model layer, Steamworks.NET integration, and game-specific logic for both titles. | [View Map](original/CnCTDRAMapEditor/codemap.md) |
| `original/REDALERT/` | **Red Alert Game Engine (C++)** - Core game logic for Command & Conquer: Red Alert. Includes unit/building AI, combat simulation, pathfinding, audio, animation, networking, resource management, and Win32 platform abstraction. | [View Map](original/REDALERT/codemap.md) |
| `original/TIBERIANDAWN/` | **Tiberian Dawn Game Engine (C++)** - Core game logic for Command & Conquer: Tiberian Dawn (C&C1). Shares architectural patterns with Red Alert but implements distinct unit rosters, campaigns, and balance. Includes house/faction logic, mission scripting, and legacy Westwood file format handlers. | [View Map](original/TIBERIANDAWN/codemap.md) |
| `original/SCRIPTS/` | **Build & Asset Utilities** - Python-based tooling for asset pipeline operations (`tgautil.py` for TGA texture manipulation, `download_openra_assets.py` for optional OpenRA asset fetching, `backup/restore_workspace.py` for workspace snapshots). | [View Map](original/SCRIPTS/codemap.md) |
| `RUST/` | **Rust Reimplementation Workspace** - Modern cross-platform client reimplementing both engines (20+ crates). Core simulation: `cnc-tiberian-dawn`, `cnc-red-alert`. Infrastructure: `cnc-core` (fixed math, rules, serialization), `cnc-renderer` (wgpu + software), `cnc-audio` (Westwood channel mixer), `cnc-asset` (Westwood format parsers), `cnc-procgen` (deterministic asset generation), `cnc-sfx` (procedural SFX), `cnc-music` (MIDI→OGG). Tools: `cnc-launcher` (GUI lobby), `cnc-map-editor` (map editor), `cnc-console` (log monitor), `cnc-classic` (classic mode), `cnc-bench` (benchmarks), `cnc-ai` (deterministic AI), `cnc-events`/`cnc-logs` (observability), `cnc-net-directory` (matchmaking). 952+ tests per `BUILD_STATUS.md`. | [View Map](RUST/codemap.md) |
| `RUST/cnc-core/` | **Foundation Primitives** - Fixed16_16, FloatBias, ClassId, CCPtr, ObjectPool, CodePointers, RulesDatabase, INI parser, Palette, serialization, math, shroud, triggers, visibility. Zero external deps. | [View Map](RUST/cnc-core/codemap.md) |
| `RUST/cnc-tiberian-dawn/` | **TD Game Logic** - Deterministic lockstep simulation: units, buildings, weapons, AI, carryover, powerups, missions, savegames, network, pathfinding. | [View Map](RUST/cnc-tiberian-dawn/codemap.md) |
| `RUST/cnc-red-alert/` | **RA Game Logic** - Deterministic lockstep simulation: naval, aircraft, Gap Generator, MCV, Spy, Tesla, Superweapons (Iron Curtain, Chronosphere, Nuke, GPS), RNG, replay, MPR scenarios. | [View Map](RUST/cnc-red-alert/codemap.md) |
| `RUST/cnc-renderer/` | **WGPU Renderer** - Palette-indexed sprite rendering, dual backend (WGPU/software), sprite batching, camera, immediate UI. | [View Map](RUST/cnc-renderer/codemap.md) |
| `RUST/cnc-audio/` | **Audio Engine** - Westwood channel model (SFX 0-8, Voice, Music), priority preemption, volume buses, crossfade, positional attenuation. | [View Map](RUST/cnc-audio/codemap.md) |
| `RUST/cnc-asset/` | **Asset Pipeline** - Westwood format parsers (SHP, VXL, PAL, AUD/VOC, MIX), atlas builder, animation registry, CLI (`cnc-asset-cli`). | [View Map](RUST/cnc-asset/codemap.md) |
| `RUST/cnc-procgen/` | **Procedural Generation** - Deterministic terrain, unit sprites, tilesets, campaign expansions (briefings, cutscenes, UI), seed-driven, bit-identical output. | [View Map](RUST/cnc-procgen/codemap.md) |
| `RUST/cnc-sfx/` | **Procedural SFX** - Pure-functional synthesis (explosion, gunfire, tesla, engine, construction, beep) → WAV. Seed-driven, deterministic. | [View Map](RUST/cnc-sfx/codemap.md) |
| `RUST/cnc-music/` | **Music Recreation** - MIDI parse → note pairing → additive synthesis → OGG Vorbis. Deterministic offline baking. | [View Map](RUST/cnc-music/codemap.md) |
| `RUST/cnc-launcher/` | **Launcher GUI** - Egui/eframe: game select, lobby, matchmaking, options (TOML), renderer selection, network test binaries. | [View Map](RUST/cnc-launcher/codemap.md) |
| `RUST/cnc-map-editor/` | **Map Editor** - Egui/eframe: terrain paint, object placement, trigger edit, tileset management, plugin arch (TD/RA), procedural gen, headless test. | [View Map](RUST/cnc-map-editor/codemap.md) |
| `RUST/cnc-console/` | **Log/Event Monitor** - Real-time Egui GUI for `cnc-logs` + `cnc-events`, filtering, search, headless CI mode. | [View Map](RUST/cnc-console/codemap.md) |
| `RUST/cnc-classic/` | **Classic Mode** - ABI-safe deterministic binary preserving original rules. Version-embedded, network-compatible. | [View Map](RUST/cnc-classic/codemap.md) |
| `RUST/cnc-bench/` | **Benchmarks** - Criterion suite: cold-start + throughput (600 frames, 10s sim) for TD/RA. Deterministic fixtures. | [View Map](RUST/cnc-bench/codemap.md) |
| `RUST/cnc-ai/` | **Deterministic AI** - AgentScript DSL → AST → lockstep runtime → serializable commands. LLM intent bridge (out-of-band). Ladder registration. | [View Map](RUST/cnc-ai/codemap.md) |
| `RUST/cnc-events/` | **Event System** - Typed GameEvent (category, priority, data), bounded MPSC EventBus, global singleton, cnc-logs correlation. | [View Map](RUST/cnc-events/codemap.md) |
| `RUST/cnc-logs/` | **Structured Logging** - Tracing subscriber, JSON output, file rotation, global accessor, correlation IDs. | [View Map](RUST/cnc-logs/codemap.md) |
| `RUST/cnc-net-directory/` | **Matchmaking** - Decentralized network directory for lockstep session discovery. | [View Map](RUST/cnc-net-directory/codemap.md) |
| `RUST/assets/packs/` | **Asset Packs** - Versioned asset collections: `cnc32_assets_v6_2_1` (campaign, QA, engine tests), `rts_16x16_tileset` (TD/RA terrain atlases), procedural test fixtures. | [View Map](RUST/assets/packs/cnc32_assets_v6_2_1/codemap.md) |

---

## Cross-Cutting Architectural Notes

### Shared Patterns Between Game Engines
Both `original/REDALERT/` and `original/TIBERIANDAWN/` follow a similar **Westwood-era C++ architecture**:
- **Class hierarchies** rooted in abstract base classes (e.g., `AbstractClass`, `BuildingClass`, `AircraftClass`)
- **Manager singletons** for global systems (Audio, Animation, Network, Resource)
- **Flat file organization** — all `.cpp`/`.h` files at directory root with `RESOURCE/` and `WIN32LIB/` subdirectories
- **Manual memory management** with custom allocators (`ALLOC.CPP` in Tiberian Dawn)
- **Deterministic simulation** suitable for lockstep multiplayer

### Map Editor Integration
The `original/CnCTDRAMapEditor/` consumes game data structures via:
- **Game-specific namespaces** (`TiberianDawn/`, `RedAlert/`) mirroring engine types
- **Shared model layer** (`Model/`) defining map format, terrain, triggers, and object placement
- **Steamworks.NET** for Workshop publishing (autogenerated bindings in `original/CnCTDRAMapEditor/Steamworks.NET/autogen/`)

### Build System
- **Dual solution approach**: Engine DLLs built via `original/CnCRemastered.sln` (C++), Editor via `original/CnCTDRAMapEditor.sln` (C#)
- **Batch Build** required for all configurations (Debug/Release, x86/x64)

### Rust Workspace Architecture
The `RUST/` workspace follows a **layered dependency DAG**:
- **Leaf**: `cnc-core` (no intra-workspace deps) → consumed by all
- **Mid**: `cnc-renderer`, `cnc-audio`, `cnc-asset`, `cnc-procgen`, `cnc-sfx`, `cnc-music`, `cnc-events`, `cnc-logs` (depend on core)
- **Top**: `cnc-tiberian-dawn`, `cnc-red-alert` (depend on mid + core)
- **Apps**: `cnc-launcher`, `cnc-map-editor`, `cnc-console`, `cnc-classic`, `cnc-bench`, `cnc-ai`, `cnc-net-directory` (depend on engines + mid)
- **Determinism invariant**: All simulation crates use `Fixed16_16` (RA) or `FloatBias` (TD); no RNG in sim; procedural generators are pure functions of seed; savegames use `CodePointers` fixup for pointer serialization.

---

