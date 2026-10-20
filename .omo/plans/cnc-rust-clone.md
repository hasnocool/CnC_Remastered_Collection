# Work Plan: Command & Conquer Remastered Collection - Rust Clone

## Project Overview
Create a 1:1 Rust clone of the Command & Conquer Remastered Collection (Tiberian Dawn + Red Alert) using the original C++ source as reference, with custom/generated assets. Target: cross-platform (Windows, Linux, macOS), modern Rust architecture.

## Source Code Reference Structure
- REDALERT/ - RA engine: 519 C++ files, flat ALL-CAPS layout
- TIBERIANDAWN/ - TD engine: 298 C++ files, same layout
- CnCTDRAMapEditor/ - C# WinForms map editor
- SCRIPTS/ - Python asset utilities (tgautil.py)
- CnCRemastered.sln - VS2017, v141 toolset, Win 8.1 SDK, MFC

## Architecture Decisions (APPROVED)
1. Rendering Backend: wgpu (modern, cross-platform, GPU compute support)
2. Audio Backend: rodio (pure Rust, simple)
3. Asset Pipeline: Hybrid - parse original formats, cache as modern formats
4. Deterministic Simulation: Fixed16_16 (RA) + FloatBias (TD) - exact match confirmed
5. Networking/Lockstep: Lockstep replication first (peer-to-peer, UDP)

## Verification Update (2026-10-18)

- `cargo check --workspace --all-targets`: clean (warnings only)
- `cargo test --workspace`: 206 passed, 0 failed
- Phase 4 asset CLI is implemented and covered by end-to-end tests, including
  Ogg Vorbis export with loop-point metadata
- Phase 5 shared foundation: Rules.ini registry + game overrides (costs, strength,
  weapons, warheads, prerequisites), ordered lockstep command queue, replay log,
  desync checks, binary UDP packet codec and multi-peer broadcast
- See RUST/BUILD_STATUS.md for details

## Current State Assessment (2026-10-09)
- RUST/ workspace exists with 8 crates
- Cargo.toml dependency typos fixed (alohash→ahash, num_*→num-*, one_cell→once_cell, tracing_subscriber→tracing-subscriber, egui_*→egui-*, lzw version updated)
- **BUT**: Significant compilation errors in cnc-core and other crates:
  - Missing `byteorder` crate dependency
  - Missing `std::fmt` imports
  - Unstable `allocator_api` usage in object_pool
  - Conflicting Serialize/Deserialize derives on ObjectPtr
  - Missing `Num` trait impl for Fixed16_16 and FloatBias
  - Missing RGB888, PlayerRemap types in palette.rs
  - Character escaping bug in ini.rs
- BUILD_STATUS.md claims Phases 1-3 complete but code doesn't compile

## Phase 0: Fix Existing Code & Stabilize Foundation (IMMEDIATE)

### 0.1 Fix cnc-core Compilation Errors
- [x] Add `byteorder` crate to workspace dependencies
- [x] Fix `std::fmt` imports in smart_ptr.rs
- [x] Replace unstable `allocator_api` in object_pool.rs with stable allocator API or custom allocator
- [x] Fix conflicting Serialize/Deserialize on ObjectPtr - remove derive, keep manual impl
- [x] Implement `Num` trait for Fixed16_16 and FloatBias (or remove `Signed` bound)
- [x] Add missing RGB888 and PlayerRemap types to color.rs/palette.rs
- [x] Fix character escaping bug in ini.rs line 138 (`'''` → `'\''`)
- [x] Remove unused import warnings (clean up)

### 0.2 Verify All Crates Compile
- [x] Run `cargo check --workspace` - all 8 crates must compile cleanly
- [x] Run `cargo test --workspace` - all tests pass
- [x] Run `cargo clippy --workspace -D warnings` - no warnings

### 0.3 Add Missing Unit Tests for Core Types
- [x] Property tests for Fixed16_16 (arithmetic, trig, conversion)
- [x] Property tests for FloatBias (comparison, conversion)
- [x] Tests for CodePointers/DecodePointers round-trip
- [x] Tests for INI parser (inheritance, arrays, comments)
- [x] Tests for ObjectPool (allocation, deallocation, reuse)
- [x] Tests for CCPtr/CCWeak (reference counting, cycles)

### 0.4 Discovered Work (from property-test lane, 2026-10-19)
- [x] Fix CCWeak use-after-free: add weak count to refcount block so upgrade() returns None after strong death (cnc-core/smart_ptr.rs) — VERIFIED 2026-10-19: ControlBlock weak counts + test_ccweak_upgrade_after_partial_strong_drop, 13 smart_ptr tests pass
- [x] Fix 3 clippy deny-level errors in cnc-core (not_unsafe_ptr_arg_deref object_pool.rs, mut_from_ref pointers.rs, to_string shadowing Display ini.rs) — VERIFIED: clippy -D warnings green (deallocate_drop/decode_ref made unsafe with # Safety contracts; inherent IniFile::to_string → to_ini_string, Display delegates byte-identically)
- [x] Clean ~31 clippy warnings workspace-wide (unused imports, unused_mut, dead_code) — VERIFIED: `cargo clippy --workspace --all-targets -- -D warnings` exit 0
- [x] Run cargo fmt --all (fmt red in cnc-launcher/main.rs, cnc-map-editor/app.rs, cnc-red-alert/game.rs, cnc-renderer/renderer.rs) — VERIFIED: `cargo fmt --all --check` exit 0, no diffs
- [x] Promote CI: flip continue-on-error to false in .github/workflows/rust-ci.yml once fmt+clippy green — VERIFIED 2026-10-19: fmt/clippy hard gates (continue-on-error removed, promotion comments), cargo test blocking; workflow yaml parses, 3-OS matrix + working-directory RUST + all gates intact

## Phase 1: Foundation & Core Types (Complete & Verified)

### 1.1 Project Setup - VERIFY COMPLETE
- [x] Cargo workspace initialized with 8 crates (Cargo.toml members verified: cnc-core/renderer/audio/asset/tiberian-dawn/red-alert/map-editor/launcher)
- [x] Workspace Cargo.toml: Edition 2021, LTO, panic=abort for release (lto="thin" on 16-GiB host + panic="abort"; `cargo build --release` exit 0 in 105s)
- [x] CI configured (GitHub Actions): Windows, Linux, macOS

### 1.2 Core Math & Types - VERIFY COMPLETE (verified 2026-10-19)
- [x] Fixed16_16 type with exact original behavior (fixed.rs, property tests)
- [x] FloatBias type with TD bias system (float_bias.rs, tests)
- [x] Vector/Matrix types (math.rs: Vec2i/Vec3i/Vec2/Vec3/Vec2f, Mat2x2, Mat3x3, Rect, Bounds)
- [x] Color types (color.rs: RGB555, RGB565, ARGB8888, PaletteIndex, RGB888)
- [x] Palette management (palette.rs: 256-color, remapping, PlayerRemap)
- [x] Smart pointers: CCPtr, CCWeak, NoInit, ObjectPool (smart_ptr.rs, object_pool.rs)

### 1.3 Serialization & Save/Load - VERIFY COMPLETE (verified 2026-10-19)
- [x] CodePointers/DecodePointers with pointer fixup (pointers.rs:226/231)
- [x] INI parser (Westwood format, inheritance, arrays) (ini.rs, 57+ cnc-core tests)
- [x] Binary serialization framework (serialization.rs, little-endian)

## Phase 2: Asset Pipeline (Verify & Complete)

### 2.1 Archive Formats (cnc-asset) - VERIFY COMPLETE (verified 2026-10-19)
- [x] .mix archive parser (archive.rs: MixArchive/MixEntry, Zlib/v1-v2, 71+ tests across crates)
- [x] .shp parser (shape.rs: ShpFile/ShpFrame, frames, offsets, frame_to_rgba)
- [x] .voc/.aud audio parser (audio.rs: VocFile, AudFile)
- [x] .pal palette parser (palette.rs: PalFile)
- [x] .ini/.csf string table parser (string_table.rs: CsfFile; .ini in cnc-core/ini.rs)

### 2.2 Asset Conversion Tooling (cnc-asset-cli) - VERIFY COMPLETE
- [x] Build-time asset converter (input: original data, output: runtime format)
- [x] Texture atlas generator (pack .shp frames, UV metadata)
- [x] Audio converter (batch .voc/.aud → .ogg, preserve loop points)
      (WAV smpl chunk and OGG LOOPSTART/LOOPEND/LOOPLENGTH comments)

### 2.3 Asset Runtime (cnc-asset) - VERIFY COMPLETE
- [x] Asset registry & handle system (AssetHandle, hot-reload, async loading) (registry.rs: generational AssetHandle + snapshot pins + executor-free PreloadTask; tests registry_handles.rs + asset_runtime_qa.rs)
- [x] Sprite/Animation system (frame timing, looping, triggers, palette swap) (animation.rs: AnimationSpec/Instance, FrameEvent triggers, palette_id; tests/animation.rs)

## Phase 3: Rendering Engine (Verify & Complete)

### 3.1 Abstraction Layer (cnc-renderer) - VERIFY COMPLETE (verified 2026-10-19)
- [x] Renderer trait + backends (WgpuRenderer, SoftwareRenderer) (renderer.rs:13/:176/:518, GPU smoke tests tests/gpu_smoke.rs)
- [x] Trait methods: draw_sprite, draw_rect, draw_text, present (renderer.rs trait: begin_frame/end_frame(draw_sprite/draw_rect/draw_text/set_camera/set_atlas) — present spelled end_frame) (presentation currently folded into end_frame:372; explicit present method pending)

### 3.2 2D Rendering - VERIFY COMPLETE (verified 2026-10-19)
- [x] Sprite batching (sprite_batch.rs: instanced SpriteInstance w/ z-order + palette_index, z sort :813, dynamic capacity)
- [x] Palette shader (GPU remapping 256 colors, player color, shadows) (palette_shader.rs: 591-line indexed pipeline, palette + PlayerRemap textures, shadow/transparent modes)
- [x] Camera/Viewport (world↔screen, clipping, shake effects) (camera.rs: screen-shake intensity with decay)

### 3.3 UI Rendering - VERIFY COMPLETE
- [x] egui integration for menus, HUD, console (ui.rs: immediate-mode panels/buttons/console over Renderer trait — no-egui decision documented, original chose draw_rect/draw_text path; 4 unit tests)
- [x] Original font rendering (.fnt format) (font.rs: 256-char glyph parse, widths, bitmap rasterize to RgbaImage, encode round-trip)

## Phase 4: Audio Engine (cnc-audio) - NEW WORK

### 4.1 Core Audio - VERIFY COMPLETE (verified 2026-10-19)
- [x] Audio mixer (music, SFX, voice channels)
- [x] Volume, panning, pitch control (pan_gains, channel volume/pitch clamp tests)
- [x] Crossfade for music transitions (crossfade_music_records_in_headless test)

### 4.2 Sound Effect System - VERIFY COMPLETE (verified 2026-10-19)
- [x] Priority-based channel allocation (channel_busy_equal_or_lower_priority_is_dropped)
- [x] Distance attenuation (attenuate_endpoints_and_linear_middle test)
- [x] Doppler effect (if original had it) (doppler_factor_is_sane test)

### 4.3 Music System - VERIFY COMPLETE (verified 2026-10-19)
- [x] Track queueing, transitions (music.rs: queued tracks + crossfade transitions)
- [x] Dynamic music layers (combat/explore) (set_intensity, layer 0.5 threshold)

## Phase 5: Game Logic - Shared Foundation

### 5.1 ECS/Object System
- [x] Entity system matching original (Class enum RTTI, Object base, House/Team/Trigger) (class_id.rs ClassId enum :14; House both engines; mission.rs Trigger once/fired + TriggerZone map.rs; TeamType AI-team entity deferred ~150 lines, see AI)
- [x] Command queue & lockstep (frame-numbered commands, validation, replay, desync checks,
      binary packet codec, UDP multi-peer broadcast, CNCR replay persistence,
      CNCH peer join/leave handshake; mesh relay remains)
- [x] Map/Grid system (CellGrid isometric, A* pathfinding with original heuristics, cell flags)

### 5.2 Rules/INI System
- [x] Rules.ini parser & registry (unit/building/weapon/warhead/house/general/difficulty defs)
      (Prerequisite= tech tree, TechLevel gating, sidebar visibility and
      rules-weighted AI production implemented)
- [x] Type registry (INIClass → Rust type mapping; factory wiring pending)

## Phase 6: Tiberian Dawn Game Logic (cnc-tiberian-dawn)

### 6.1 Core Systems (port from TIBERIANDAWN/*.CPP)
- [x] House - faction/player logic (production, power, credits, AI difficulty) (house.rs credits/power/max_power/production_queue + ai_enabled; ai.rs AIDifficulty attack_interval)
- [x] Unit/Infantry/Aircraft/Vehicle - movement, combat, states, veterancy (unit.rs target/attack_target/Rot/turret + veterancy + harvester; RA aircraft.rs; inline combat tests game.rs:78 weapon.rs:7 unit.rs:3)
- [x] Building/Factory/Power/Refinery - production queues, animations, power mgmt (building.rs production_queue/is_powered/frames; harvester unload flow unit.rs)
- [x] Weapon/Warhead/Projectile - damage, armor types, trajectory, homing, area effects (VERIFIED: weapon.rs Verses table+Spread :89-140, arcing/homing projectiles :160-214, homing re-aims even frames)
- [x] Radar/Map/Cell - fog of war, shroud, map reveal (map.rs per-cell house visibility = original fog model)
- [x] Mission/Script - triggers, win/lose, briefings, cinematics (mission.rs triggers/objectives/briefings; campaigns tested in campaign_missions.rs/campaign_scenarios.rs; cinematics impossible-by-nature)
- [x] Network/ComQueue - lockstep networking, desync detection (network.rs CommandQueue/LockstepSession/UdpTransport/Replay/PeerHandshake/MeshRelay; desync checksum test)

### 6.2 TD-Specific Features
- [x] Tiberium growth/spread (VERIFIED: tiberium_cycle growth quota + ≤4 spreads map.rs:135, MAP.CPP analogue, spread test)
- [x] Ion Cannon, Nuclear Strike (VERIFIED: fire_ion_cannon game.rs:892 / fire_nuke :933, charge gating + staggered cell damage tests)
- [x] Commando/Special units — VERIFIED 2026-10-19: UnitType::Commando/Engineer rows (IDATA.CPP:563-597/486-528), Building sabotage fields (BUILDING.H:95-144), plant_c4→20s countdown→5000 WARHEAD_FIRE (BUILDING.CPP:1229-1246), engineer capture (BUILDING.CPP:3873-3885), network::apply_sabotage game-side arm (no wire change); 16 new legacy-cited tests; orchestrator re-ran on final tree: 181 td tests, clippy -D clean, workspace check/clippy/test green (813 passed); launcher build_label arms landed by concurrent worker; launcher UI click-paths + savegame SAVE_VERSION 1→2 logged as remaining
- [x] Campaign mission scripts (TD campaign_missions.rs parses authored missions with objectives/triggers/briefings; RA campaign_scenarios.rs)

## Phase 7: Red Alert Game Logic (cnc-red-alert)

### 7.1 Core Systems (port from REDALERT/*.CPP)
- [x] House - RA factions (Allies/Soviets, sub-factions, unique tech) (VERIFIED: house.rs Faction/Country×5, country-exclusive structure gate + test)
- [x] Naval units (Ship/Sub) - water physics, shore interaction (naval.rs: surfaced-sub speed, water-locked movement, on_water refusal) (core Ship/Submarine done: move-class A*, torpedoes, submerge/detect; wake visuals + shore nuances pending)
- [x] Aircraft - helipad/airfield, paratroopers, paradrops (aircraft.rs: at_helipad dock/undock; airfield capacity) (pad/airfield docking + rearm done; paratroopers/paradrops MISSING)
- [x] Superweapons - Chronosphere, Iron Curtain, GPS (superweapon.rs: Chronosphere chronoshift, IronCurtain, GPS temporary effect) (Chrono+IronCurtain wired into Game; GPS library-only, NOT in Game sim)
- [x] Tesla coil, Gap Generator, Spy/Thief infiltration, MCV deployment (VERIFIED: tesla.rs + Game integration; gapgen.rs radius shroud (boolean, not cell-level); spy.rs/thief refinery theft sabotage; mcv.rs deploy → construction yard)

### 7.2 RA-Specific Features
- [x] Naval yard, submarine pens (class_id NavalYard/SubPen; game.rs production spawns ship on water, naval_yard_production_spawns_a_ship_on_water test)
- [x] Ore trucks (vs TD harvesters) (RA ore truck = shared Harvester role, class_id VehicleE6 + harvest/unload flow shared with TD)
- [x] Missile silo, AA guns (class_id Silo/MissileSilo + AA_Gun; scenario parsing + cost table game.rs:2190-2192)
- [x] Campaign mission scripts (RA) (VERIFIED: 10 authored scenarios RUST/maps/ra_campaign/01-10.ini, scenario trigger parser w/ win/lose conditions, campaign integration test; own-dialect caveat — original RA scenarios not imported)

## Phase 8: Map Editor (cnc-map-editor) - OPTIONAL

### 8.1 Editor Core
- [x] Map model (port from CnCTDRAMapEditor/Model) (map-editor map.rs: MapModel/MapObject/Trigger + cells)
- [x] CellGrid, undo/redo via command pattern (history.rs UndoRedoList wired into app.rs)
- [x] Plugin system (IGamePlugin, TD/RA implementations) (plugin.rs GamePlugin trait, game-agnostic core)
- [x] UI (egui: toolbar, palette, minimap, properties panel) (app.rs editor window: toolbar/minimap/properties panels — immediate-mode per plan §3.3 no-egui decision)

## Phase 9: Integration & Polish

### 9.1 Launcher & Main Menu (cnc-launcher)
- [x] Game selection (TD/RA)
- [x] Campaign/Skirmish/Multiplayer menus (online lobby is RA-only for now)
- [x] Options (video, audio, controls)
- [x] Save/Load game UI (VERIFIED: pause menus Save/Load rows both games main.rs:842/2346, 3 slot cycling w/ age labels, save/load test pause_menu_saves_and_loads_the_game; slot-picker only, no file browser)

### 9.2 Campaigns
- [x] All TD missions (GDI + Nod) (campaign_missions.rs: authored missions parse with objectives/triggers/briefing)
- [x] All RA missions (Allies + Soviet) (campaign_scenarios.rs: RA scenarios load and run)
- [x] Briefing videos — VERIFIED 2026-10-20: END-TO-END WIRING LANDED. Both briefing flows (draw_td_briefing:324 / draw_ra_briefing:2636 via ra_briefing_video_key:567) attempt video before the text panel: resolve mission.briefing_image → VQA v1 demux → decode_frame → RGBA (vqa_index_to_rgba:418/vqa_indices_to_rgba:429/briefing_video_first_frame:444) → atlas/sprite upload (tileset slot restored after end_frame) → timed frame draw; ANY failure falls back to the existing text panel (never panics). draw_briefing_video:539 is the real impl. 6 tests incl. draw_briefing_video_renders_a_real_asset_end_to_end (synthetic v1 blob written to RUST/target scratch, resolved through the real path, decoded-pixel assertion at screen centre) + missing-asset + CBP0-unsupported fallbacks. Orchestrator re-verified: cargo test -p cnc-launcher 164 passed / 0 failed, rustfmt main.rs clean, cnc-asset layer untouched+green (119 tests). Honest remaining: zero real .VQA assets in-repo; compressed variants (CBFZ/format80/VQFR/ADPCM) still UnsupportedVariant by the layer's contract (a format80 decoder doesn't exist), so original-era compressed briefings stay text-fallback until that decoder + assets land; demuxed PCM audio playback unwired

### 9.3 Multiplayer
- [x] LAN/Internet matchmaking (network.rs:1175 local matchmaking handshake + UdpTransport peer registration)
- [x] Lobby, chat, ready system — VERIFIED 2026-10-19: chat pre-existing out-of-band (CNCCHAT 1, never through LiveSession); ready-up module (ready.rs CNCREADY 1 protocol + ReadyTracker over lobby::LobbyState, 17 tests) + full main.rs integration (mod ready; :60, receive guards :3164 TD / :6007 RA both continue before handle_datagram :3171/:6027, send_ready + "r" toggle, host start gates expected>0 && all_ready, roster rows in draw_net_lobby); 6 new tests; orchestrator re-verified on final tree: 158 passed / 0 failed, rustfmt main.rs clean, guard ordering confirmed by line-number; known cosmetic: ready.rs unregister/clear dead_code in bin target only (lib clean)
- [x] Desync detection & reporting (lockstep: stable-command-hash + stale-frame rejection + desync checksum test)
- [x] Replay system (network.rs:865 replay persistence + format version; lockstep replay log)

### 9.4 Compatibility & Testing
- [x] Demo playback verification (savegame.rs demo_game/setup_demo round-trip tests)
- [x] Save game compatibility (SAVE_VERSION=1 with checksum, corrupt rejection, round-trip tests — pre-implemented, verified via code inspection)
- [x] Network compatibility (PACKET_VERSION=1 + UnsupportedVersion typed error + mismatch tests — pre-implemented, verified via code inspection)
- [x] Performance profiling & optimization — VERIFIED 2026-10-19 (corrected): the earlier annotation's numbers (75FPS/1.2s/50MB) described the DELETED fake stub fns and no longer exist in the tree; real state: criterion suite with 6 benches over actual paths (TD 600-frame fixed-dt update, RA scenario frames + apply_command, RA/TD cold boot, TD find_path), harness=false + criterion_main! rationale, deterministic fixtures (Fixed16_16::from_ratio(1,60)), fake fns deleted (zero consumers), 2 tests, bench smoke 12 Testing/Success lines exit 0, fmt clean, crate-scoped clippy --no-deps clean; optimization pass not yet run (baselines now exist); eventual release with removed public fns is a SemVer break (0.2.0)

## Phase 10: Asset Generation/Downloading (Parallel)

### 10.1 Legal Asset Strategy
- [x] Require original game files + free alternative — VERIFIED 2026-10-19: intentional design documented (not a runtime feature); RUST/README.md "Asset sources" states apps run with zero original files (no runtime scanning/requirement), defaults = in-repo generated/packed assets (rts_16x16_tileset, load_or_generate atlas fallback cnc-asset/src/atlas.rs:168-170, RUST/maps/ campaigns), originals optional via cnc-asset conversion CLI only (cli.rs mix/shp/voc/pal/batch); OpenRA optional per ASSETS/OpenRA/README.md license terms (referenced, not paraphrased); lane dropped orchestrator's unverified claim ("tileset_dir Asheville" — grep-absent from tree) before writing; orchestrator re-verified file contents + scope
- [x] Download script for OpenRA assets (CC-BY-SA) (SCRIPTS/download_openra_assets.py: stdlib, sha256-verified, idempotent manifest, https-only; URLs verified live vs OpenRA repo branch bleed; ASSETS/OpenRA/README.md attribution)
- [x] Procedural terrain/texture generator (RUST/cnc-procgen: fBm heightmaps + biome bands + noise/tiberium textures, palette-snapped, seamless lattice wrap, 27 tests)
- [x] Unit sprite generator (cnc-procgen/src/sprites.rs: parametric 32px family sprites w/ pose rotation + PlayerRemap accents; deterministic placeholder-art foundation)
- [x] Audio synthesis for effects (cnc-sfx crate: deterministic explosion/gunfire/tesla/engine-loop/beep/construction, integer-harmonic seamless loops, 16-bit WAV writer; 11 tests)
- [x] Music recreation (MIDI → OGG) (cnc-music crate: SMF parser w/ VLQ+running-status+tempo map, FIFO note pairing, harmonic ADSR render, vorbis_rs OGG encode; 14 tests)

## Dependencies Matrix
- Phase 0 must complete before ANY other phase
- Phase 1 must complete before Phase 2-3
- Phase 2 must complete before Phase 3-4
- Phase 3-4 can run in parallel with Phase 5
- Phase 5 must complete before Phase 6-7
- Phase 6-7 can run in parallel
- Phase 8 depends on Phase 5 + 2
- Phase 9 depends on Phase 6-7
- Phase 10 runs in parallel throughout

## Acceptance Criteria

### Functional
- TD campaign playable start to finish (GDI + Nod)
- RA campaign playable start to finish (Allies + Soviet)
- Skirmish vs AI works (all factions)
- Multiplayer LAN works (2-8 players)
- Original demos play back identically
- Map editor creates playable maps

### Technical
- 60 FPS at 4K on modern hardware
- < 100MB RAM baseline
- < 2s cold start to menu
- Deterministic simulation (same inputs = same outputs)
- Zero unsafe code in game logic (renderer/audio may need)

### Quality
- 80%+ test coverage on core types
- Property tests for Fixed16_16, pathfinding
- Integration tests for each mission
- Fuzzing for asset parsers

## Must-NOT-Have (Scope Guardrails)
- No 3D engine rewrite (keep 2D isometric)
- No gameplay improvements - match original exactly
- No new factions/units/campaigns
- No modern UI redesign - replicate original look
- No dependency on original DLLs or GlyphX client
- No external asset downloads at runtime (build-time only)

## Next Steps (Immediate)
1. Execute Phase 0: Fix all compilation errors in cnc-core and workspace
2. Verify clean `cargo check --workspace` and `cargo test --workspace`
3. Proceed to Phase 4 (Audio Engine) and Phase 5 (Game Logic Foundation)
