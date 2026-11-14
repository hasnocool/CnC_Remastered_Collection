# Implemented Functionality Map — Rust Port

**Audit date:** 2026-11-14  
**Audited Rust source snapshot:** [`8853580f03fb66159143136165ef24077865d780`](https://github.com/hasnocool/CnC_Remastered_Collection/commit/8853580f03fb66159143136165ef24077865d780)  
**Repository HEAD before this documentation refresh:** [`42867ba9ae2576df44c9e074085da8ca134ae3ed`](https://github.com/hasnocool/CnC_Remastered_Collection/commit/42867ba9ae2576df44c9e074085da8ca134ae3ed) (documentation-only commit; parent is the audited Rust source snapshot)  
**Pinned original source:** [`f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`](https://github.com/electronicarts/CnC_Remastered_Collection/commit/f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae)  
**Scope:** targeted source-level audit of Rust implementation, live call paths and focused test definitions, compared with original engine/editor source at the pinned upstream commit.

This file is the evidence-based list of functionality present in the Rust port. It is not a claim of complete 1:1 parity. The companion [ORIGINAL_FUNCTIONALITY_MAP.md](ORIGINAL_FUNCTIONALITY_MAP.md) describes the original systems and records the current gaps and compatibility boundaries.

## Status rules

- **Implemented — focused evidence:** concrete behavior, call path and focused tests are present in the current source tree. This audit inspected test definitions; it does not imply the tests were executed during this pass.
- **Partial implementation:** real behavior exists, but the original system has known missing semantics, edge cases, or incomplete coverage.
- **Integration-limited:** the parser/data model/helper exists, but complete runtime use is absent or unproven.
- **Modern replacement:** a different implementation replaces a legacy API/platform. Protocol, ABI, save-file or binary compatibility is not implied.
- **Not implemented:** the current implementation is a stub, explicit reject path, or absent from the inspected code.

A type, module, filename, TODO comment, parser, or historical parity percentage is not enough to claim a feature is implemented. Trace the code into the game/runtime and identify an executable test or fixture.

## Function-level porting inventory

The companion [Function and Porting Contract Catalog](FUNCTION_CATALOG/README.md) provides a complete pinned source-file manifest and a generator for per-file callable-symbol indexes. It is the function-level surface for a future-language/platform port; a signature extraction is not a behavioral contract or parity result. Use its contract template to document inputs, outputs, state effects, dependencies, edge cases and target-language mapping.

## 1. Audit baseline and repository shape

| Item | Verified state |
|---|---|
| Audited Rust source snapshot | `8853580f03fb66159143136165ef24077865d780` (the Rust source parent of the documentation-only pre-audit HEAD) |
| Repository HEAD before this documentation refresh | `42867ba9ae2576df44c9e074085da8ca134ae3ed` (documentation-only; no Rust source changes relative to the audited snapshot) |
| Rust source files | 267 `.rs` blobs under `RUST/` in the audited Git tree |
| Cargo manifests | 19 crate-level `RUST/*/Cargo.toml` files plus `RUST/Cargo.toml` (20 manifests under `RUST/` total); this is not a resolved workspace-package count |
| Workspace manifest caveat | `RUST/Cargo.toml` has 19 member entries and also lists `cnc-console` under `exclude`; actual package resolution must be obtained with `cargo metadata --no-deps` |
| Original upstream source files | 1,331 blobs at pinned upstream commit `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae` |
| Original RA / TD source | RA: 628 files (301 `.CPP`, 295 `.H`, 14 `.ASM`, 14 `.INC`); TD: 411 files (193 `.CPP`, 184 `.H`, 13 `.ASM`, 15 `.INC`). `.ASM` + `.INC` totals 28 assembly-source/include files in each engine. |
| Build/test verification in this audit | Source and test definitions were inspected remotely; Cargo commands were **not run** because the requested local checkout is not mounted in this session. `cargo metadata --no-deps`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace --no-fail-fast` therefore remain unverified for this pass. |

## 2. Shared core

| Feature | Current Rust implementation | Assessment |
|---|---|---|
| Shared object identity, ownership, health and destruction | `cnc-core/src/entity.rs` (`Object`, `ObjectFlags`, `HasObject`) | **Implemented — focused evidence.** Damage/healing bounds, destroyed state and serialization tests exist. This is not a binary/layout reproduction of the original C++ inheritance model. |
| Class/type identifiers and token parsing | `cnc-core/src/class_id.rs`, `entity.rs` | **Partial implementation.** The Rust roster and token translation exist; full parity with every original RTTI/type-table record is not established. |
| Deterministic fixed-point arithmetic | `cnc-core/src/fixed.rs` and both engines | **Implemented.** Both engines use the shared `Fixed16_16` primitive. TD's full original float-bias semantics are not automatically equivalent. |
| INI parsing and typed rules database | `cnc-core/src/ini.rs`, `rules.rs` | **Implemented — focused evidence.** Sections, values, type/weapon/warhead/house and difficulty records are parsed. Per-engine use of all original fields remains partial. |
| TeamTypes, TaskForces and trigger record models | `cnc-core/src/entity.rs`, `trigger_records.rs`, `trigger_attach.rs` | **Implemented — focused evidence.** Parsing/model/attachment machinery exists; game-specific runtime execution remains action- and engine-dependent. |
| Shared trigger/action dispatcher | `cnc-core/src/trigger_apply.rs` | **Partial implementation.** Supported actions have dispatch paths and tests; default host behavior rejects unsupported actions. An enum entry or parser arm is not proof the action executes. |
| House expert-AI framework | `cnc-core/src/house_ai.rs` | **Implemented framework, partial game integration.** Difficulty cadence, urgency ladder, build task state and weighted production helpers exist. Each engine's adapter must perform real world mutations. |
| Visibility and shroud primitives | `cnc-core/src/shroud.rs`, `visibility.rs` | **Implemented — focused evidence.** Per-house discovered/visible state and snapshot support exist; both engine renderers use visibility state. |
| Shared math, serialization and memory helpers | `math.rs`, `serialization.rs`, `object_pool.rs`, `pointers.rs`, `smart_ptr.rs` | **Implemented primitives.** These provide Rust-side equivalents, not exact C++ allocator, ABI or pointer-fixup compatibility. |

## 3. Tiberian Dawn engine

| Feature | Current Rust implementation | Assessment |
|---|---|---|
| Game state and simulation update | `cnc-tiberian-dawn/src/game.rs` | **Partial implementation.** A real simulation loop updates economy, production, combat, triggers, visibility and outcomes. Full original object lifecycle parity is not claimed. |
| Map and cell grid | `map.rs` | **Implemented — partial parity.** Map/cell state is used by movement, scenario loading and rendering. |
| Unit, infantry, vehicle, aircraft and building behavior | `unit.rs`, `building.rs`, `game.rs` | **Partial implementation.** Core movement, damage, production, construction, cargo/transport and selected class-specific behavior exist; the complete original roster/mission behavior matrix remains open. |
| Weapon, warhead and projectile behavior | `weapon.rs`, `td_rules.rs`, `game.rs` | **Implemented — focused evidence.** Weapon stats, armor-adjusted damage, travelling projectiles and impact handling have focused tests. |
| Defensive building fire | `game.rs` combat update | **Implemented — focused regression evidence.** A test verifies that a defensive structure fires a projectile rather than applying instant direct damage. |
| Economy, tiberium, production and construction | `game.rs`, `house.rs`, `building.rs` | **Partial implementation.** Credit spending, queues, prerequisite checks, power, harvester collection/unloading, storage and construction placement are represented and tested. |
| Original TD scenario INI/BIN loading | `cnc-tiberian-dawn/src/scenario_loader.rs::{load_scenario,parse_scenario,scenario_to_game}` | **Implemented for supported records; not full scenario parity.** Reads the INI and sibling BIN, handles raw/LCW cell data, builds live map/house/unit/structure/waypoint/mission state, and reports unsupported records. This is a live path: `cnc-launcher/src/lib.rs::original_campaign::load_campaign` calls `load_scenario` and `scenario_to_game`. `cnc-tiberian-dawn/tests/original_scenario_game.rs::original_td_scenario_builds_game_with_terrain_overlay_units` covers the production conversion path; it was inspected, not executed in this audit. |
| Original TD cell-trigger bindings | `scenario_loader.rs`, `mission.rs`, `game.rs` | **Implemented for supported bindings; trigger vocabulary remains partial.** `[CellTriggers]` names are resolved to mission trigger IDs and installed in the live mission; unknown names are reported and skipped. The integration fixture also demonstrates an unsupported `Player Enters` event remains explicitly unmapped rather than being guessed. |
| Mission, objectives and trigger execution | `mission.rs`, `scenario_loader.rs`, `events.rs` | **Partial implementation.** Rust mission dialect and supported original records execute; unknown/unmapped event and action names are reported rather than guessed. |
| TeamTypes and TaskForce runtime | `ai.rs`, `cnc-core/src/entity.rs` | **Implemented — focused evidence.** TeamTypes are parsed, teams spawn when task-force requirements are met, members are assigned, objectives issue commands and completed teams retire. |
| TD expert AI | `ai.rs`, `cnc-core/src/house_ai.rs` | **Partial implementation.** There is a working TD adapter and tests for power/income/harvester/attack decisions; full original urgency, base-layout and special-flag behavior is not established. |
| Pathfinding | `pathfind.rs` | **Implemented — focused evidence, partial parity.** Deterministic grid routing, diagonal movement, obstacle avoidance and no-path behavior are tested. Original zone/region and all terrain-cost semantics remain open. |
| Fog of war and hidden-entity rendering | `game.rs`, `cnc-core/src/shroud.rs` | **Implemented — focused evidence.** Discovery, explored-but-unseen treatment and suppressing unseen entities during render are tested. |
| Snapshot save/load | `savegame.rs` | **Implemented — focused evidence.** Rust snapshots round-trip live state, mission progress, unit targets and TeamType state; malformed/truncated saves are rejected. This is not proof of original save-file byte compatibility. |
| Lockstep command queue, packet codec and transport | `network.rs` | **Implemented subsystem, partial legacy parity.** Deterministic frame queue, checksums/state CRC, packet encoding, async UDP transport, handshake, replay and host-migration state machines exist and have focused tests. Original IPX/modem/WOL protocols are not reproduced. |

## 4. Red Alert engine

| Feature | Current Rust implementation | Assessment |
|---|---|---|
| Game state and main update loop | `cnc-red-alert/src/game.rs` | **Partial implementation.** Current update loop calls production, AI, TeamType, rearm, trigger, structure/unit combat, special-unit, projectile, movement and harvest updates. |
| Original RA MPR/INI parsing | `original_scenario.rs`, `scenario_mpr.rs` | **Implemented for supported records.** MapPack/OverlayPack, original object records, houses, waypoints, triggers, base nodes, smudges and terrain records are parsed with malformed-input tests. Parsing a field does not prove the game consumes it. |
| Original RA scenario-to-game conversion | `scenario_mpr.rs`, `game.rs` | **Partial implementation.** Supported houses/base/trigger/object data is applied to game state; complete semantics for every parsed field and object record remain open. |
| RA `[Houses]` and `[Base]` | `original_scenario.rs`, `scenario_mpr.rs` | **Implemented for covered fields.** Player/faction selection, house credits and base-node construction have focused tests. |
| RA production, ownership and selected RULES.INI overrides | `game.rs`, `house.rs`, `cnc-core/src/rules.rs` | **Partial implementation.** Selected cost, strength/armor, prerequisite, tech-level, owner and sight values reach runtime decisions. Full type inheritance, rule coverage and all original fields are not established. |
| Armor-scaled combat, projectiles and splash | `game.rs` | **Implemented — focused evidence.** Armor/warhead scaling, projectile travel/miss behavior and splash falloff are tested. |
| Ground, naval, air and amphibious pathfinding | `pathfind.rs`, `game.rs` | **Implemented — focused evidence, partial parity.** Distinct movement classes and water/land rules are tested. |
| Aircraft ammo and rearming | `game.rs` | **Implemented — focused evidence.** Ammo use and pad capacity/reload-timer behavior have tests. |
| Ships and carrier behavior | `game.rs` | **Implemented for covered behavior.** Naval production, gunboat behavior and carrier aircraft launch/recovery have focused tests recorded in the parity closure. |
| RA TeamTypes | `game.rs`, `cnc-core/src/entity.rs` | **Implemented — focused evidence.** Team spawning, assignment, attack behavior and retirement paths have tests. |
| RA expert House AI | `ai.rs`, `cnc-core/src/house_ai.rs` | **Not complete.** The shared framework is called, but the RA adapter's `queue_structure` is a success-only placeholder, `launch_attack_wave` reports success without issuing attacks, and `sell_structure` returns success without selling. The AI state is recreated locally on each call, so persistence of cadence/one-time decisions is also not established. |
| RA trigger execution | `game.rs`, `cnc-core/src/trigger_apply.rs`, `events.rs` | **Partial implementation.** Supported event/action paths have tests; unsupported host action families are rejected/logged. Do not describe all original TACTION/TEVENT vocabulary as implemented. |
| Special units and superweapons | `dog.rs`, `engineer.rs`, `medic.rs`, `tanya.rs`, `spy.rs`, `madtank.rs`, `mcv.rs`, `tesla.rs`, `vortex.rs`, `superweapon.rs`, `crates.rs` | **Partial implementation.** Several dedicated behavior modules and game call paths exist; all roster variants, weapon combinations and rare state transitions need a source-to-test matrix. |
| Shroud and discovery triggers | `game.rs`, `cnc-core/src/shroud.rs` | **Implemented — focused evidence.** Per-house visibility and cell-discovery trigger flow have tests. |
| RA save/load | `game.rs` | **Implemented for Rust serialization.** Save strings round-trip live game state, objects, credits, elapsed time, triggers, objectives and superweapon readiness. Original save-file compatibility is not proven. |
| RA replay and deterministic RNG | `replay.rs`, `rng.rs` | **Implemented subsystem, partial compatibility.** Version 3 stores seed, RNG state, command frames and state-CRC trace; unsupported pre-RNG versions are explicitly rejected. |

## 5. Map editor

| Feature | Current Rust implementation | Assessment |
|---|---|---|
| Editable map model | `cnc-map-editor/src/map.rs` | **Implemented.** Terrain, overlays, icons, objects, houses, triggers, waypoints, TeamTypes, TaskForces and agent definitions have model support. |
| Undo/redo and command edits | `history.rs`, `tools.rs`, `app.rs` | **Implemented — focused evidence.** Editing operations are command-based and undoable/redone. |
| Terrain, overlay, object, erase, selection and waypoint tools | `tools.rs`, `app.rs` | **Implemented for current tool set.** Painting, drag, selection, occupancy and bounds behaviors have focused tests. |
| Trigger inspector and authoring | `app.rs`, `map.rs`, `tools.rs` | **Implemented — partial parity.** Trigger data and inspector controls are editable; exact parity with every original dialog/control is not claimed. |
| TeamType/TaskForce model operations and INI serialization | `map.rs`, `history.rs`, `tests/teamtypes.rs` | **Implemented at model/serialization level.** This does not establish a complete counterpart for the original TeamTypes dialog. |
| Original RA MPR/INI import/export | `cnc-map-editor/src/{map,original,plugin}.rs` | **Implemented for supported records with fixture round-trip tests.** The plugin dispatches `.mpr`/original RA INI to the RA reader; `tests/original_formats.rs::editor_roundtrips_original_ra_fixture` reopens and compares the model and preserved sections. |
| Original TD INI/BIN import/export | `cnc-map-editor/src/{map,original,plugin}.rs` | **Implemented for supported records.** The editor reads the sibling `.BIN`, decodes raw or LCW-compressed cell data, and writes both `.INI` and `.BIN`; fixture tests cover round-trip and raw/LCW equivalence. |
| Original-format import through the editor UI | `cnc-map-editor/src/app.rs::{import_original_with_dialog,handle_import_choice,import_original_path}` and `plugin.rs::{load_scenario,save_scenario}` | **Implemented — focused UI and file-flow evidence.** `Action::ImportOriginalMap` opens a native dialog; the app pairs a TD `.INI` with its sibling `.BIN`, dispatches RA `.MPR`/INI, and applies import as an undoable command. `app.rs::import_dialog_and_path_flow_reads_both_original_dialects`, `app.rs::import_button_is_on_the_toolbar_and_renders`, and `tests/original_formats.rs` cover the UI/file path, round-trip and error handling. These tests were inspected, not run. Full C# editor parity is not implied. |
| Preserve unknown INI sections | `original.rs`, `tests/original_formats.rs` | **Implemented — focused evidence.** The editor preserves sections the model does not understand during round-trip. |
| Agent Studio | `app.rs`, `cnc-ai/src/` | **Implemented as a Rust-specific feature, partial parity.** Script editing, checks, dry-run feedback and save/undo paths exist; it is not a port of an original C# dialog. |

## 6. Renderer, launcher and presentation

| Feature | Current Rust implementation | Assessment |
|---|---|---|
| Wgpu and software renderer | `cnc-renderer/src/renderer.rs` | **Implemented.** GPU/software renderer interfaces support sprites, text, rectangles, camera and texture resources. This is a modern replacement for DirectDraw, not its ABI. |
| Sprite batching, palette shader and texture layers | `sprite_batch.rs`, `palette_shader.rs` | **Implemented — focused evidence.** WGSL validation, instance layout, stable z-order and multi-atlas sampling have tests. |
| Launcher/game frontend | `cnc-launcher/src/` | **Implemented as a modern frontend, partial parity.** Menu, setup, campaign, options, score and game-entry flows exist; GlyphX UI and every legacy screen are not reproduced. |
| Lobby screen and directory browsing | `lobby_screen.rs`, `lobby.rs`, `ready.rs` | **Implemented as modern replacement, partial integration.** Directory listings, LAN discovery and direct-connect fallback are represented. |
| VQA demux and frame decode | `cnc-asset/src/video.rs` | **Implemented for covered VQA paths.** Container/chunk parsing, frame decode, compressed codebook paths and malformed-input handling have focused tests. Full movie compatibility remains open. |
| Launcher movie playback | `cnc-launcher/src/movie.rs` | **Partial implementation.** Frame timing, playback/skip/stop state and fallback briefing generation exist; full original MCI/movie integration is not reproduced. |
| Audio mixer and channel management | `cnc-audio/src/mixer.rs` | **Implemented — focused evidence.** SFX/voice/music channels, priority arbitration, volume buses, pause/resume, fades and cross-fades are tested in headless mode. |
| Music queue and theme routing | `cnc-audio/src/music.rs`, `cnc-launcher/src/audio_director.rs` | **Partial implementation.** Explore/combat queue layers, intensity and launcher event routing exist; complete original theme/event/sample coverage is not established. |
| TD and RA EVA vocabularies / speech queue | `cnc-audio/src/{eva,eva_ra,speech}.rs` | **Partial implementation.** Distinct vocabularies and a shared queue/director protocol exist. The October 24 historical audit records only a subset of lines reached from real emitters; re-count current coverage before claiming more. |
| Procedural briefing, avatars, icons and UI wireframes | `cnc-procgen/src/`, including `expansion/ui_v62.rs` | **Implemented as procedural replacement for covered assets.** Deterministic avatar/icon/screen/wireframe generation and tests exist; this is not original artwork or original asset parity. |

## 7. Asset formats and conversion

| Feature | Current Rust implementation | Assessment |
|---|---|---|
| MIX archive reading/extraction | `cnc-asset/src/archive.rs`, `classic_pack.rs` | **Implemented for covered variants.** Compressed/uncompressed entry tests exist; unusual archive flags/encryption still require compatibility coverage. |
| Shape frames, sprite atlases and animation | `shape.rs`, `atlas.rs`, `anim.rs`, `animation.rs`, `unit_atlas.rs` | **Implemented, partial format/content parity.** Decoders, frames, playback and atlas generation exist; not every original asset variant is proven. |
| Palette and original template metadata | `palette.rs`, `original_templates.rs`, `registry.rs`, `tileset_pack.rs`, `tile_join.rs` | **Implemented, partial parity.** Parsing, registry, pack and join logic exist; exhaustively matching all original theater/resource definitions remains open. |
| Format80/LCW compression | `format80.rs` | **Implemented.** Encode/decode paths exist and LCW is used by TD cell data and VQA paths. |
| MEG/DDS and remastered assets | `megafile.rs`, `original_assets.rs` | **Implemented for covered paths, partial compatibility.** Archive/resource access exists; availability/distribution of proprietary content is a separate issue. |
| Font parser, glyph metrics, rasterizer and encoder | `font.rs` | **Implemented — focused evidence.** The old cross-reference's “planned font.rs” claim is stale. Exact original font rendering/UI integration is not established by parser tests. |
| Westwood ADPCM decoder | `adpcm.rs` | **Implemented.** Decoder tables and mono/stereo decode routines exist; verify every original stream/channel convention with reference fixtures. |
| VOC/AUD loading and WAV conversion | `audio.rs` | **Partial implementation.** Load and conversion paths exist; all legacy block types, compression variants and loop semantics are not established. |
| VXL parser and atlas conversion | `vxl.rs` | **Implemented at parser/conversion level.** Header/layer/frame parsing and RGBA/atlas conversion are covered by synthetic tests; runtime use through every game rendering path is not proven. |
| CSF string-table loading | `string_table.rs` | **Not implemented.** `CsfFile::load` currently returns an empty `HashMap` irrespective of input. Do not count CSF support as implemented. |

## 8. Networking, replay and support systems

| Feature | Current Rust implementation | Assessment |
|---|---|---|
| TD lockstep commands, packet codec, checksums and state CRC | `cnc-tiberian-dawn/src/network.rs` | **Implemented subsystem, partial legacy parity.** Frame ordering, packet encode/decode, checksums, replay and state-CRC desync checks have focused tests. |
| UDP peer transport and handshake | `network.rs` | **Implemented subsystem.** Async datagram send/receive, peer handshake and host migration state are present with focused tests. This is not IPX, modem or WOL compatibility. |
| Directory client and service API | `cnc-net-directory/src/client.rs`, `api.rs`, `node.rs` | **Implemented as modern service.** Async publish/browse/heartbeat/withdraw and supporting endpoints exist. |
| LAN discovery, matchmaking and ratings | `cnc-net-directory/src/discovery/`, `matchmaking.rs` | **Implemented as modern replacement.** Discovery, LFG, ranked match records, ratings and persistence have tests. Do not confuse these APIs with original WOL. |
| Scripted AI runtime | `cnc-ai/src/` | **Implemented as a new Rust subsystem, partial game integration.** It does not automatically provide original House AI behavior. |
| Logging, events and benchmarks | `cnc-logs/src/`, `cnc-events/src/`, `cnc-bench/src/` | **Implemented support subsystems.** Check active workspace membership with Cargo metadata rather than counting folder names. |

## 9. Confirmed or high-confidence gaps

These are intentionally listed here so this file cannot be mistaken for a blanket parity claim.

1. **Red Alert expert AI is incomplete.** In `cnc-red-alert/src/ai.rs`, `queue_structure` is marked as a stub and returns success without placing/queuing a structure; `launch_attack_wave` returns success for non-empty forces without issuing orders; `sell_structure` returns success without selling. `run_per_house_ai` constructs a local `HouseAIState` per call, so persistent cadence/one-time state must also be fixed.
2. **Trigger action vocabulary is incomplete.** `cnc-core/src/trigger_apply.rs` has host methods whose default implementations reject actions. The 2026-10-24 parity-gap report recorded 12 RA action families as documented-only; reconcile this count with current HEAD and retain per-game distinctions.
3. **CSF loading is a confirmed stub.** `cnc-asset/src/string_table.rs` returns an empty string map.
4. **Rules consumption is partial.** A parsed rule field is complete only after tracing it to an effective runtime decision and a focused test.
5. **Original scenario coverage is partial.** TD/RA importers intentionally report or skip unknown records; every parsed RA section still needs a field-to-game-state consumption audit.
6. **Original save compatibility is unproven.** Current Rust save snapshots/strings and replay formats are not byte-compatible copies of original Westwood serialization.
7. **Original networking/platform APIs are not reproduced.** IPX, modem/dial-up, DDE, WOL, DirectDraw and GlyphX ABI are modernized/replaced or absent; do not claim wire/ABI compatibility.
8. **Full C# map-editor parity is not complete.** Original TD/RA format import/export exists, but every legacy dialog, control, validation rule and Steam Workshop interaction has not been ported.
9. **Audio and media format edge coverage is incomplete.** Parser/mixer presence is not proof of all original codecs, event emitters or movie behavior.
10. **Source documentation coverage is separate from implementation.** The 1,331-file upstream inventory contains pages at different review depths; a completed documentation tranche does not mean the Rust behavior is complete.

## 10. Audit corrections to earlier documentation

The following earlier claims are stale and must not be repeated:

- Original TD scenario loading is implemented for supported records in `cnc-tiberian-dawn/src/scenario_loader.rs` and is called by the launcher's original-campaign discovery path; it is not wholly “not ported.”
- Original TD and RA map import/export is connected to the editor's file/plugin path. `Action::ImportOriginalMap` reaches the native dialog and dialect-specific import; fixtures cover both formats, including TD raw/LCW BIN data and round-trip behavior.
- Keep the two TD flows distinct: the engine `scenario_loader` constructs live game state, while the map-editor importer constructs an editable `MapModel`. Neither proves complete original scenario-field, trigger or full C# editor parity.
- RA original scenario parsing/construction includes `[Houses]` and `[Base]` supported paths in `original_scenario.rs` and `scenario_mpr.rs`.
- Font parsing is implemented in `cnc-asset/src/font.rs`; it is not merely planned.
- Older Rust module paths such as `object.rs`, `cell.rs`, `classic_mix.rs`, `iff.rs`, `iconset.rs` and various standalone editor models do not describe the current module layout. Always point to current source paths.
- The October 24, 2026 parity audit is historical evidence. Its claims must be checked against current HEAD before reuse.

## 11. Targeted end-to-end audit evidence (2026-11-14)

| Original contract | Rust path and runtime caller | Focused evidence found | Audit conclusion |
|---|---|---|---|
| TD scenario lifecycle and cell image: `TIBERIANDAWN/SCENARIO.CPP::Read_Scenario` / `Read_Scenario_Ini`; `TIBERIANDAWN/MAP.CPP::Read_Binary` | `cnc-tiberian-dawn/src/scenario_loader.rs::{load_scenario,scenario_to_game}`; launcher `cnc-launcher/src/lib.rs::original_campaign::load_campaign` | `cnc-tiberian-dawn/tests/original_scenario_game.rs::original_td_scenario_builds_game_with_terrain_overlay_units` loads the fixture pair and checks terrain/icon data, houses, units, buildings, waypoints, objectives and mapped/unmapped triggers | **Real engine/launcher integration for supported records.** The unsupported `Player Enters` event remains a recorded gap; the test was inspected, not run. |
| Original map editor open/save: `CnCTDRAMapEditor/MainForm.cs::LoadFile` / `SaveFile` and `Model/Map.cs` | `cnc-map-editor/src/app.rs` import actions/dialog/path; `plugin.rs::{load_scenario,save_scenario}`; `map.rs::{from_original_td,from_original_ra}` | `cnc-map-editor/tests/original_formats.rs::{editor_roundtrips_original_td_fixture,editor_roundtrips_original_ra_fixture}`; app tests for dialog/path, toolbar, missing BIN and undo | **Real editor UI/file flow for supported TD/RA formats.** This does not establish complete dialog/tool/Steam Workshop parity with the original C# editor. |

The original sources were fetched from the pinned upstream commit, not inferred from filenames alone. In this session the local `/home/hyperion/...` checkout was unavailable, so this was a remote source audit; test definitions were inspected but none were executed.

## 12. Verification required before changing a status

For any feature promoted to “implemented”:

1. Record the original source path and exact original symbol(s).
2. Record the current Rust file and exact Rust symbol(s).
3. Trace at least one real call path from input/event/scenario to the runtime effect.
4. Identify focused unit/integration tests and fixtures.
5. Run the relevant crate test(s), then workspace gates when practical:
   - `cargo metadata --no-deps`
   - `cargo fmt --all -- --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace --no-fail-fast`
6. Record the exact commit SHA, command, date and result. If a gate is not run, say “not run”; never infer pass from an old log or test definition.
7. Update this ledger and [ORIGINAL_FUNCTIONALITY_MAP.md](ORIGINAL_FUNCTIONALITY_MAP.md) together. Keep original-source documentation review state separate from Rust implementation status.
