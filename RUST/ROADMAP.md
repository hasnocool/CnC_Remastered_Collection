# Project Implementation Roadmap

**Scope:** Whole Rust reimplementation of the Command & Conquer Remastered Collection — Tiberian Dawn, Red Alert, map editor, launcher, shared systems, tools, and porting/reference work  
**Status:** Active project roadmap  
**Established:** 2026-11-14  
**Primary source of truth for current implementation status:** [Implemented Functionality Map](original-source-reference/IMPLEMENTED_FUNCTIONALITY_MAP.md)  
**Original behavior reference:** [Original Functionality Map](original-source-reference/ORIGINAL_FUNCTIONALITY_MAP.md)

> This is the project-wide roadmap. It is not a unit-graphics roadmap and it is not a claim that the original codebase has already been fully ported. It separates what exists today from incomplete behavior and the work required to reach the project's broader parity goals.

## 1. Project objective

Build a maintainable Rust implementation of the Command & Conquer Remastered Collection's Tiberian Dawn and Red Alert systems, with a usable launcher and map editor, while preserving the original games' important gameplay behavior, content formats, deterministic simulation rules, and authoring workflows.

Where exact legacy compatibility is not practical or desirable, use a documented Rust-native replacement. A replacement must state its compatibility boundary: for example, a modern renderer can replace DirectDraw without claiming DirectDraw ABI compatibility; modern UDP matchmaking can replace an obsolete service without claiming WOL/IPX wire compatibility.

The project's completion target is therefore **behavioral and content parity where required, plus explicit and tested compatibility boundaries where systems are intentionally replaced**. Merely compiling a similarly named module, extracting a function signature, or loading one file format is not enough to call a subsystem complete.

## 2. How to read project status

Use these status meanings consistently throughout this roadmap:

| Status | Meaning |
|---|---|
| Implemented | A working Rust behavior exists, is connected to the intended runtime path, and has focused evidence. |
| Partial | Real behavior exists, but material original semantics, data coverage, edge cases, or supported variants remain missing. |
| Integration-limited | A parser, model, helper, or service exists, but end-to-end runtime consumption is incomplete or unproven. |
| Stub / placeholder | A method returns success, a default value, or an empty result without performing the advertised behavior. |
| Not implemented | No working implementation has been identified. |
| Modern replacement | A Rust-native design intentionally replaces a legacy system; the legacy ABI/protocol/file compatibility is not implied. |
| Needs revalidation | The available evidence predates relevant repository changes, tests were not run, or the full live call path has not been rechecked. |

**Never treat these as equivalent:** original source documented, function signature catalogued, Rust behavior implemented, integration path proven, and test actually executed. They are separate stages.

## 3. Baseline: what is already in place

The latest [Implemented Functionality Map](original-source-reference/IMPLEMENTED_FUNCTIONALITY_MAP.md) is dated 2026-11-14. It records an audited Rust source snapshot of `8853580f03fb66159143136165ef24077865d780`, a pre-documentation-refresh repository HEAD of `42867ba9ae2576df44c9e074085da8ca134ae3ed`, and the pinned upstream source commit `f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae`.

That ledger is the current status reference, but its header does **not** certify that every later repository change has been re-audited. The first roadmap milestone is therefore to pin the actual working HEAD, compare it with the audited snapshot, and re-run the relevant build/test gates before making new completion claims.

### 3.1 Implemented foundation

| Area | Existing implementation | Current assessment |
|---|---|---|
| Shared Rust core | Entity/object primitives, class identifiers, fixed-point arithmetic, rules/INI models, serialization helpers, object pools, smart pointers, shroud/visibility primitives, trigger models | Implemented foundation; full original object-model and behavior parity is not claimed |
| Tiberian Dawn runtime | Simulation/update loop, units/buildings, combat, economy/production, harvesters, mission/trigger system, pathfinding, shroud, save snapshots, lockstep command/network subsystem | Substantial working implementation; many original content and behavior details remain partial |
| Red Alert runtime | Simulation/update loop, ground/naval/air movement, projectiles, armor-scaled combat, production, harvesters, TeamTypes, special-unit modules, superweapons, shroud, save/replay state | Substantial working implementation; AI, rules coverage, triggers, and some special cases remain incomplete |
| Original TD scenarios | INI + sibling BIN loading, raw/LCW cell data, supported map/house/unit/structure/waypoint/mission conversion, launcher campaign discovery path | Implemented for supported records; not full scenario or trigger parity |
| Original RA scenarios | MPR/INI/MapPack/OverlayPack parsing and supported house/base/object/trigger conversion | Implemented for supported records; parsed fields still require a complete consumption audit |
| Map editor model | Terrain, overlays, icons, objects, houses, triggers, waypoints, TeamTypes, TaskForces, agent definitions; command-based edits and undo/redo | Working editor foundation |
| Original map import/export | RA MPR/INI and TD INI/BIN import/export, unknown-section preservation, import via UI/native dialog and undoable command | Working for supported records; not complete C# editor parity |
| Renderer | WGPU and software renderer interfaces, sprites/text/rectangles/camera/textures, batching, palette shader, multi-atlas sampling | Working modern renderer; not DirectDraw ABI or GlyphX render integration |
| Launcher | Modern frontend with menu/setup/campaign/options/score/game-entry paths | Working modern frontend; not all legacy GlyphX screens/workflows |
| Terrain assets | Generated TD/RA-inspired terrain packs; original Remastered texture-MEG/DDS terrain loading documented for launcher/editor; theater/template and terrain joining support | Meaningful generated and original-terrain support; original models, unit art, UI, audio, and movies remain separate gaps |
| Generated unit art | CNC32 generated unit atlas integration in TD/RA and launcher/editor paths | Working fallback path; frame advancement and damage-condition rendering remain gaps |
| Asset formats | MIX variants, SHP/frame/atlas paths, Format80/LCW, PAL, font parsing/rasterization, ADPCM, VOC/AUD paths, MEG/DDS covered paths, VXL parser/conversion | Broad format foundation; several formats/variants are partial or parser-only |
| Audio/media | Audio mixer, channel management, music queue/director, EVA vocabularies/queue, VQA parsing/frame decoding, launcher movie state | Partial runtime and content integration |
| Multiplayer/service systems | Lockstep command/packet/state-checksum systems, async UDP transport, handshake/replay/host-migration state, directory service, LAN discovery, matchmaking/LFG/rating records | Modern replacement foundation; production service operation and all end-to-end multiplayer paths need verification |
| AI/tooling | Shared House AI framework, TD AI, TeamType management, AgentScript runtime and editor tooling | Partial engine integration; RA expert AI contains no-op methods that must be fixed |
| Reference/porting catalog | Pinned 1,331-path upstream manifest; generated indexes with 15,219 callable/candidate symbols from 1,073 files; per-source documentation and porting-contract template | Inventory exists; signature extraction is not semantic porting or parity verification |

### 3.2 Explicitly incomplete or known problem areas

The current functionality ledger identifies the following high-confidence gaps. These are not speculative features; they are already recorded in source or audit evidence and must be resolved or deliberately scoped out.

1. **Red Alert expert AI has success-only placeholders.** `queue_structure` does not queue/construct a structure, `launch_attack_wave` does not issue attack orders, and `sell_structure` does not sell a structure. Per-house AI state is recreated locally, so persistent cadence and one-time decisions are not established.
2. **Trigger/action coverage is incomplete.** Some event/action records are mapped and tested, but 12 RA action families were documented as unsupported in the 2026-10-24 closure audit. The current mapping and runtime support must be checked against the latest code; default host actions reject unsupported behavior.
3. **CSF string loading is a confirmed stub.** `cnc-asset/src/string_table.rs::CsfFile::load` currently returns an empty map.
4. **Some scenario data is parsed but not fully consumed.** TD and RA import/runtime paths support meaningful subsets, but parser coverage must not be mistaken for complete field-to-game-state behavior.
5. **Rules integration is incomplete.** A parsed RULES.INI value only counts as implemented when the relevant engine consumes it and a test proves the runtime effect.
6. **The MAD Tank path is unreachable until its class identifier exists.** The detonation behavior exists, but the current roster check prevents the advertised special-unit path from being reached.
7. **Original unit/building/UI assets are not equivalent to original terrain support.** The proven Remastered MEG/DDS terrain integration does not establish original unit-art, turret, building, UI, or effects rendering.
8. **Generated unit animation is incomplete.** The generated atlas has frame duration/loop metadata, but existing game paths have been recorded as selecting frame zero; condition sprites are indexed but not consumed.
9. **Audio and video coverage is partial.** Complete original EVA emission, music/theme routing, in-game SFX, all legacy audio variants, and movie compatibility are not proven.
10. **Legacy compatibility is bounded.** Rust save snapshots are not proven byte-compatible with original saves; modern networking is not IPX/modem/WOL wire compatibility; WGPU/software rendering is not DirectDraw/GlyphX ABI compatibility.
11. **Full original C# editor parity is not complete.** Import/export exists, but every tool, dialog, validation rule, plugin, and Workshop interaction is not reproduced.
12. **Function catalog semantics remain largely unreviewed.** All extracted symbols are marked `SIGNATURE_CATALOGUED`; the catalog itself states that behavior, state effects, call relationships, Rust mapping, and parity tests remain to be completed.

## 4. Project roadmap and priorities

The roadmap is ordered to reduce the chance of building on stale status information or on paths that only appear complete. Workstreams can run in parallel after the relevant baseline and dependency gates are satisfied.

### P0 — Re-establish one trustworthy project baseline

**Why first:** The implementation map is recent, but it explicitly pins an audited source snapshot rather than asserting that every later source commit has been rechecked. The older `PARITY.md` and `BUILD_STATUS.md` contain historical figures and must not be used as current test results without remeasurement.

**Work**

- [ ] Pin actual repository HEAD and working-tree state.
- [ ] Compare all Rust source changes since the source SHA in the implementation map.
- [ ] Run `cargo metadata --no-deps` and record resolved workspace packages and excluded/orphan crates.
- [ ] Run formatting, clippy, focused crate tests, and the workspace test suite.
- [ ] Separate newly failing tests from pre-existing failures; reproduce each failure on the exact pinned checkout.
- [ ] Run launcher/editor/game smoke tests and capture the current default, generated-assets, and original-terrain paths where available.
- [ ] Refresh `IMPLEMENTED_FUNCTIONALITY_MAP.md` against the current source HEAD; update `ORIGINAL_FUNCTIONALITY_MAP.md` where the port status changed.
- [ ] Reconcile stale summary claims in `PARITY.md`, `BUILD_STATUS.md`, and supporting evidence documents.

**Exit criteria:** One commit SHA, one source baseline, reproducible workspace/package counts, known test status, and an implementation ledger that matches the inspected source. Do not use unverified historical pass counts as a release gate.

### P1 — Remove real no-ops and unreachable advertised behavior

**Why next:** Success-only placeholders and unreachable branches can make systems appear more complete than they are, while silently failing at runtime.

**Work**

- [ ] Implement Red Alert `queue_structure` using actual RA production/build placement rules.
- [ ] Implement Red Alert `launch_attack_wave` so units receive valid orders and attack waves have observable game effects.
- [ ] Implement Red Alert `sell_structure` with real ownership, refund, destruction, and map-state semantics.
- [ ] Persist `HouseAIState` per house across updates; prove cadence, doctrine, and one-time decisions survive multiple AI ticks.
- [ ] Add tests that fail when each AI adapter returns success without changing the game.
- [ ] Add the missing MAD Tank class identity or explicitly remove/defer the unreachable behavior with a documented reason.
- [ ] Replace the CSF empty-map stub with a fixture-tested parser, including malformed/truncated input handling and string lookup semantics.
- [ ] Search all engine/support crates for other success-only methods, empty implementations, default no-op handlers, and unreachable class/type branches.

**Exit criteria:** No advertised AI/parser/special-unit feature is counted as implemented while its live path is a no-op or unreachable. Each repaired feature has an end-to-end test demonstrating the effect.

### P1 — Complete scenario, mission, and trigger execution

**Why:** Loading an original map is only the beginning. A playable scenario needs the parsed records to produce the correct game state and the trigger/event system to execute the expected outcomes.

**Work**

- [ ] Create separate TD and RA scenario-field matrices: source section/key → parser → model → game-state consumer → test/fixture.
- [ ] Audit all TD INI/BIN fields and RA MPR/INI/MapPack/OverlayPack/Houses/Base fields for parsed-but-unused data.
- [ ] Expand campaign fixtures so original-format scenarios are tested through the engine/launcher path, not only parser tests.
- [ ] Complete supported TD and RA event-condition coverage, preserving engine-specific differences.
- [ ] Implement remaining RA trigger action families where original semantics can be supported; retain explicit reject/documentation for unsupported actions rather than inventing behavior.
- [ ] Add tests for TD construction-complete trigger delivery and other known emitter paths that currently lack end-to-end scenarios.
- [ ] Verify objective, reinforcement, team, waypoint, ownership, and mission-outcome transitions from original scenario inputs.
- [ ] Ensure unknown data is reported and preserved where appropriate; avoid silent field loss on editor round trips.

**Exit criteria:** Every claimed scenario feature has an end-to-end fixture proving the path from original input record to runtime effect. Unsupported records are reported precisely and remain visible in the gap ledger.

### P1 — Finish shared rules, roster, and core gameplay contracts

**Work**

- [ ] Build a per-game rules-consumption matrix for unit/infantry/building/weapon/warhead/projectile/house/difficulty/general settings.
- [ ] Prioritize fields that directly affect combat, armor, speed, health, cost, prerequisites, ownership, tech limits, sight, power, and production.
- [ ] Add tests showing that changing a source rules field changes the expected runtime decision.
- [ ] Compare TD and RA class/type rosters against the pinned original RTTI tables and identify missing, aliased, or incorrectly mapped classes.
- [ ] Complete core infantry, vehicle, aircraft, naval, building, cargo/transport, repair, capture, and destruction behavior matrices.
- [ ] Audit pathfinding and locomotion rules by movement type and terrain, including blocked paths, water/land rules, docking, rearming, and carrier behavior.
- [ ] Complete AI base planning, defense, production priorities, and house-specific strategy only where original behavior and runtime support are understood.

**Exit criteria:** Core gameplay changes are source-backed and covered by simulation tests; TD and RA remain independently validated.

### P2 — Build the complete original asset and visual pipeline

**Existing foundation:** Generated terrain and unit packs already render. Original Remastered terrain MEG/DDS support is documented for both games' renderer/editor paths. Those capabilities do not imply original unit-art or all original asset parity.

**Work**

- [ ] Extend original asset discovery beyond terrain: identify unit, infantry, turret, building, animation, UI, cursor, icon, and effects resources from actual installed game data.
- [ ] Keep different archive formats as separate adapters unless fixtures prove they are compatible.
- [ ] Complete original SHP/image decoding and metadata support against verified fixtures.
- [ ] Add original unit and building catalogs with source-backed mappings, facing, layer, animation, offsets, damage state, and faction data.
- [ ] Render original TD and RA unit bodies, turrets, infantry, buildings, and effects where assets are available.
- [ ] Implement animation frame stepping using frame timing; advance from a deterministic simulation/presentation clock, never from the draw call.
- [ ] Connect damaged/critical/destroyed visual states to authoritative game health/state.
- [ ] Implement team-colour transformations from original metadata/behavior rather than generic tinting.
- [ ] Integrate projectile, muzzle flash, impact, smoke, fire, and destruction presentation.
- [ ] Preserve generated packs as optional fallback and retain no-original-assets startup.
- [ ] Add deterministic contact sheets, per-game screenshot comparisons, pixel/metadata checks, and load/cache performance measurements.

**Recommended order:** first prove one TD infantry unit and one TD turreted vehicle end-to-end; then add RA equivalents with explicit RA-specific mappings. Extend to the Harvester and special families after the core pipeline works.

**Exit criteria:** Original assets resolve from user-provided game data, render correctly in TD and RA, and do not introduce disk I/O or blocking decode work in render calls. The game remains usable without original data.

### P2 — Complete map editor workflows

**Existing foundation:** Editable map model, core terrain/object tools, undo/redo, trigger inspector, TeamType/TaskForce model support, original TD/RA import/export, unknown-section preservation, and a working import UI path.

**Work**

- [ ] Audit every original C# editor tool/dialog against the Rust editor and classify it as implemented, partial, replaced, not required, or missing.
- [ ] Complete editing and validation for triggers, events/actions, TeamTypes, TaskForces, houses, waypoints, overlays, objects, terrain, and map metadata.
- [ ] Complete original-format round trips for supported fields without losing unknown sections.
- [ ] Add focused interaction tests for every editing tool and dialog, including undo/redo, keyboard/mouse, selection, bounds, and malformed input.
- [ ] Complete plugin/game-specific differences between TD and RA.
- [ ] Verify editor visual output against launcher/game terrain and object rendering.
- [ ] Decide and document the required level of parity for Steamworks/Workshop features; do not count a local file workflow as Workshop integration.

**Exit criteria:** All required editing workflows have testable Rust equivalents and original TD/RA map files round-trip without silent loss of supported data.

### P2 — Finish audio, speech, and video presentation

**Work**

- [ ] Connect game events to the audio mixer for actual in-game SFX, not only menu playback.
- [ ] Complete TD/RA EVA line emission from real engine conditions, preserving game-specific vocabularies and avoiding lines with no valid source condition.
- [ ] Complete music theme selection, battle/exploration transitions, loop points, intensity, and queue lifecycle.
- [ ] Verify AUD/VOC/ADPCM variants, channel conventions, loop points, and conversion behavior against fixtures.
- [ ] Complete VQA codec/frame coverage and launcher movie playback, including timing, skip, stop, and fallback behavior.
- [ ] Add audio/video regression fixtures and headless playback tests.

**Exit criteria:** Real gameplay events drive correct sound/speech/music, and supported campaign movies play with correct timing and failure handling.

### P2 — Complete multiplayer and platform integration

**Existing foundation:** Lockstep command/packet systems, state CRC, replay, async UDP transport, handshake, host-migration state, LAN discovery, directory services, matchmaking, LFG, and rating records.

**Work**

- [ ] Prove host/client game sessions end-to-end with deterministic state checks and explicit disconnect/reconnect behavior.
- [ ] Verify that every player command is validated, queued, ordered, applied, and included in deterministic state checks.
- [ ] Exercise packet loss, peer timeout, duplicate/out-of-order messages, host loss, migration, and shutdown.
- [ ] Verify launcher lobby, directory, LAN, ready-up, and direct-connect paths against a running service.
- [ ] Add server deployment/configuration/observability and abuse/rate-limit controls where required.
- [ ] Document the boundary between modern Rust networking and original IPX/modem/WOL behavior; implement legacy compatibility only if it is a real project requirement.
- [ ] Verify save/replay compatibility within the Rust formats and explicitly document non-compatibility with original save formats where it remains.

**Exit criteria:** Supported multiplayer modes work from launcher to game runtime under reproducible network tests; compatibility boundaries are explicit.

### P3 — Finish source-level port contracts and documentation coverage

**Current inventory:** The Function Catalog contains the complete 1,331-path pinned upstream file manifest and 15,219 extracted callable/candidate symbols from 1,073 files. All extracted symbols are currently marked `SIGNATURE_CATALOGUED`; that means signatures were indexed, not that their behavior has been understood or ported. The generated summary records detailed docs linked for 266 callable-source files.

**Work**

- [ ] Complete semantic review of every original source file, including function bodies, declaration/definition pairs, callbacks, virtual overrides, macros, message handlers, and assembly entry points.
- [ ] Convert symbols from `SIGNATURE_CATALOGUED` to `CONTRACT_DOCUMENTED` only after recording inputs, outputs, state mutations, dependencies, ordering, side effects, and edge cases.
- [ ] Map each contract to a Rust call path and one of: parity verified, partial, replaced with compatibility boundary, not required with rationale, or not implemented.
- [ ] Maintain separate TD, RA, and map-editor ledgers.
- [ ] Add source-derived fixtures/tests for contracts that matter to the Rust implementation.
- [ ] Use function/file inventories to find unimplemented behavior, not as a target to reproduce every private helper name or original file boundary.
- [ ] Refresh generated indexes from the exact pinned original source revision; never label generated output from an unpinned checkout as complete.

**Exit criteria:** Every required original behavior has a documented contract and status, and every implemented claim links to its Rust call path and supporting evidence. Functions intentionally not ported have a rationale.

### P3 — Release hardening and project completion

**Work**

- [ ] Define supported platforms, minimum runtime requirements, install layout, and optional original-data configuration.
- [ ] Establish reproducible builds and release packaging for launcher, games, editor, tools, and data packs.
- [ ] Add startup validation and actionable diagnostics for missing/corrupt assets, configs, maps, and service settings.
- [ ] Run deterministic simulation, original map import, map round-trip, rendering, audio/video, save/replay, and multiplayer regression suites.
- [ ] Run performance and memory profiling on large maps, high unit counts, long sessions, cache pressure, and network degradation.
- [ ] Test clean install, upgrade, no-original-data mode, original-data mode, and corrupted/missing-data recovery.
- [ ] Publish a release readiness report that distinguishes completed systems from documented compatibility limits.

**Exit criteria:** A clean checkout can be built, launched, played, edited, tested, and packaged using documented steps, with known limitations and passing release gates.

## 5. Immediate next implementation queue

This is the recommended order for the next concrete work, after P0 baseline reconciliation. It is intentionally more actionable than the broader milestones above.

| Order | ID | Work item | Reason | Completion evidence |
|---:|---|---|---|---|
| 1 | PR-001 | Re-audit current HEAD against the 2026-11-14 implementation ledger | Prevent roadmap decisions from relying on a stale source snapshot or historical test counts | Updated SHA-stamped functionality map and executed build/test report |
| 2 | PR-002 | Repair Red Alert expert AI no-op adapters and persist per-house AI state | Known runtime paths currently report success without performing their advertised action | Tests prove structure queueing, attack orders, selling/refunds, and multi-tick state persistence |
| 3 | PR-003 | Implement CSF string-table parsing | Confirmed empty parser stub; bounded, independently testable asset feature | Valid/malformed fixtures, key/value and label lookup tests |
| 4 | PR-004 | Build TD/RA scenario field-consumption matrices and close the highest-impact gaps | Avoids confusing parse success with correct live gameplay | End-to-end original scenario fixtures with source field → runtime effect assertions |
| 5 | PR-005 | Finish trigger event/action execution and TD construction-complete coverage | Mission scripting is central to campaign playability | Per-game event/action coverage matrix and executable trigger tests |
| 6 | PR-006 | Complete core rules/roster behavior coverage | Rules and class gaps affect balance, production, combat, and unit behavior | Source-backed rules matrix and per-class tests |
| 7 | PR-007 | Restore and verify the complete test/visual baseline | Establishes reliable regression protection before large feature expansion | Green or explicitly triaged CI, deterministic screenshots, documented existing failures |
| 8 | PR-008 | Implement original unit visual resources and animation playback | Extends the original terrain pipeline to the most visible missing gameplay art | TD and RA original-art vertical slices; timing/facing/pivot tests |
| 9 | PR-009 | Complete editor tool/dialog parity for required workflows | Turns format import/export into a full map-authoring workflow | Feature matrix and interaction/round-trip tests |
| 10 | PR-010 | Connect audio, EVA, music, and movie playback to live gameplay | Makes campaign presentation complete rather than menu-only or parser-only | Event-to-audio/video integration tests and capture evidence |

The queue is a starting sequence, not permission to claim that lower-ranked work is unnecessary. If a fresh audit finds a more severe blocker, update the order with evidence and record why.

## 6. Cross-cutting acceptance rules

A task is not complete until all applicable conditions are met:

1. **Source contract:** Name the pinned original file(s) and exact symbol(s), or explain why the feature is a modern replacement.
2. **Rust implementation:** Name the Rust file(s), symbols, and live caller/call path.
3. **Behavior:** Prove observable state changes or output. A parsed field, enum arm, type, or success return is not sufficient.
4. **Tests:** Add focused unit/integration tests and fixtures. Record whether they were actually executed.
5. **Per-game correctness:** Test TD and RA separately wherever their original behavior differs.
6. **Determinism:** Do not allow render cadence, thread scheduling, asset load order, or nondeterministic iteration to alter lockstep gameplay results.
7. **I/O and concurrency:** Keep blocking archive/file/image work off the render path; use bounded worker queues/caches and thread-safe shared state; perform GPU uploads on the renderer-owning thread.
8. **Compatibility boundaries:** Clearly state what is intentionally replaced and what file/protocol/ABI compatibility is not provided.
9. **Fallback:** Missing optional original assets or services must not make the core game/editor unusable.
10. **Evidence:** Record commit SHA, commands, exit status, screenshots/fixtures, known gaps, and actual test execution.

## 7. Progress tracking and document ownership

Keep responsibilities separate so there is one clear place to look for each kind of information:

| Document | Owns |
|---|---|
| `RUST/ROADMAP.md` | Project-wide phases, priorities, and next implementation queue |
| `RUST/original-source-reference/IMPLEMENTED_FUNCTIONALITY_MAP.md` | Current Rust implementation status with code paths and evidence |
| `RUST/original-source-reference/ORIGINAL_FUNCTIONALITY_MAP.md` | Original TD/RA/editor behavior inventory and feature gaps |
| `RUST/original-source-reference/FUNCTION_CATALOG/` | Pinned source-file/symbol index and porting contract workflow |
| `RUST/docs/PARITY.md` | Broader feature parity summary; reconcile it with the newer implementation ledger |
| `RUST/BUILD_STATUS.md` and `.omo/evidence/` | Executed commands, test transcripts, screenshots, and QA evidence |
| `RUST/TODO.md` (if introduced/used by the active workflow) | Short-horizon execution checklist; it must not become a competing implementation-status ledger |

Update the roadmap when phase order or project priorities change. Update the implementation map whenever verified implementation status changes. Update original-functionality records when the source contract or scope changes. Do not duplicate detailed test transcripts in the roadmap.

## 8. Definition of project completion

The Rust project can be described as complete only when:

- Required TD and RA gameplay systems have source-backed, per-game status and end-to-end behavior evidence.
- Required original scenario, rules, trigger, roster, and campaign behavior is implemented or explicitly scoped out with rationale.
- The map editor covers the required authoring workflows and preserves supported original map data.
- Required graphics, unit/building assets, animation, audio, speech, and movie paths are integrated and verified.
- Multiplayer/replay behavior is reliable for the supported modern architecture, with legacy compatibility limits explicit.
- Every required original function contract is semantically reviewed and mapped to a Rust implementation, intentional replacement, or documented gap.
- The test, determinism, performance, memory, install, and release gates are reproducible.
- The implementation map, original functionality map, roadmap, build records, and actual code all agree.

**Immediate instruction:** Begin with **PR-001**, then proceed to the RA AI no-op repairs in **PR-002**. Do not start by expanding the unit renderer or porting arbitrary files until the baseline confirms where the highest-value remaining gaps actually are.
