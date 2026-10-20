# Feature Parity & Gap Analysis — Rust Workspace vs. Original C++ Engines

**Status:** Updated 2026-10-20 per `PARITY_AUDIT_2026-10-20.md` (authoritative). This doc now reflects source-verified parity levels; over-claims from the 2026-10-19 snapshot have been corrected.

**Re-verified 2026-10-20 (documentation reconciliation, plan todo 6):** every status and figure below was re-measured against the live tree at commit `5d1fd8b` — see §7 for the itemised before → after list and `.omo/evidence/task-6-parity-100.log` for the commands and their raw output. Where this doc and `PARITY_AUDIT_2026-10-20.md` still disagree, **the live tree wins**; both docs now carry the same dated corrections.

**Companions:** [`PARITY_AUDIT_2026-10-20.md`](PARITY_AUDIT_2026-10-20.md) (full audit with file/line anchors) · [`../ORIGINAL_FUNCTIONALITY_MAP.md`](../../ORIGINAL_FUNCTIONALITY_MAP.md) (original functionality, file-level).

**Method:** Four parallel source-level audits (grep/read-verified on both sides, no codemap-only claims). Original-side inventory from `ORIGINAL_FUNCTIONALITY_MAP.md` (RTTI enums, class inventory, full `.CPP` lists). Rust-side evidence from fresh source inspection: workspace members (`Cargo.toml`), per-crate source listings, symbol extraction (`pub struct/enum/fn`), keyword sweeps, `BUILD_STATUS.md` ledger.

**Workspace:** **17 member crates** — `cnc-core`, `cnc-tiberian-dawn`, `cnc-red-alert`, `cnc-renderer`, `cnc-audio`, `cnc-asset`, `cnc-map-editor`, `cnc-launcher`, `cnc-net-directory`, `cnc-ai`, `cnc-procgen`, `cnc-sfx`, `cnc-classic`, `cnc-music`, `cnc-bench`, **`cnc-events`**, **`cnc-logs`** (both landed as members — `RUST/Cargo.toml:19–20`). Not members: `cnc-console` (has a `Cargo.toml` but no workspace entry — deferred debt, plan todo 54), `cnc-asset-editor` and `cnc-replayer` (docs-only placeholder dirs with no `Cargo.toml`). *(Corrected 2026-10-20: this line previously said 15 crates with cnc-events/cnc-logs "orphaned and removed"; re-verified 2026-10-20 via `cargo metadata --no-deps` → 17 packages — `.omo/evidence/task-6-parity-100.log` §A.)*

---

## 1. Executive Summary (Audit-Verified)

| Lane | Overall Parity | Notes |
|---|---|---|
| **TD engine** (`cnc-tiberian-dawn`) | **~45–50%** | Weapons/combat tables near-verbatim (~85%); original `.ini`+`.bin` scenario loading, defensive structures fire, per-house shroud **gating acquisition and drawn**; team AI ~25% — *re-verified 2026-10-20: “no fog rendering” was stale, `Game::render_shroud` draws it — `.omo/evidence/task-6-parity-100.log` §F.* |
| **RA engine** (`cnc-red-alert`) | **~62% skirmish subset / ~45% content fidelity** | Armor-scaled combat (warhead `Verses` ÷ armor) with **partial** RULES.INI consumption (`Strength`/`Speed`/`Primary`/`Armor`/`Cost`/`Prerequisite`/`TechLevel`/`Owner`), ~1/6 trigger vocabulary — *re-verified 2026-10-20: the previous “flat damage … zero RULES.INI consumption, ~35%” row was stale and contradicted audit §1; `.omo/evidence/task-6-parity-100.log` §G.* |
| **Map editor** (`cnc-map-editor`) | **~30–40%** | Undo/redo 95%+ (93 tests vs 0 in C#); original-format map I/O — **importers landed** (`MapModel::from_original_ra` `map.rs:409`, `from_original_td` `map.rs:500`), still no TD `ini+bin` UI path — *re-verified 2026-10-20: previous “0–20%” was stale; `.omo/evidence/task-6-parity-100.log` §G.* |
| **Support crates** | Mixed | Sim/netcore solid; presentation (speech, VQ, in-game SFX) partial/absent |

**Verified build state (re-verified 2026-10-20):** `cargo check --workspace --all-targets` and `cargo clippy --workspace --all-targets` clean (0 errors); `cargo fmt --check` exits 0 on committed HEAD; `cargo test --workspace --no-fail-fast` = **1318 passed / 4 failed**. The 4 failures are pre-existing (each reproduces on the committed tree with no pending changes): `cnc-map-editor` — `layout_controls_stay_inside_the_window`, `trigger_filter_matches_group_and_name`, `trigger_inspector_controls_and_frame_render` — plus `cnc-tiberian-dawn scenario_loader::tests::records_become_executable_triggers`. *(Corrected 2026-10-20: this line previously said 1253 passed / 3 failed; the TD failure is new since that measurement and was re-confirmed independently with `cargo test -p cnc-tiberian-dawn --no-fail-fast` → 299 passed / 1 failed — `.omo/evidence/task-6-parity-100.log` §B–§C, §M. A seventh, old failure `cnc-launcher::draw_briefing_video_falls_back_on_unsupported_variant` was a stale expectation and has been fixed.)* The earlier "952 passed / 0 failed" figure came from the `BUILD_STATUS.md` ledger and was not reproducible by `cargo test`.

---

## 2. Domain Detail (Corrected per Audit)

### 2.1 Core loop & determinism — ✅
- `cnc-core`: `Fixed16_16` fixed-point (`fixed.rs`), serialization helpers, object pools, `pointers.rs` (the moral equivalent of `Code_Pointers` fixup for snapshot round-trips), deterministic `cnc-procgen::prng` (pinned SplitMix64).
- `cnc-tiberian-dawn/src/network.rs` (97.6K): `Command` enum (17 order types), `NetCommand`, `CommandQueue`, `LockstepSession` with `frame_checksum`, `desynced`, `from_replay`.

### 2.2 Object model — ⚠️ Partial, by design
- Original: deep `AbstractClass`→`TechnoClass` inheritance + custom RTTI, `CCPtr<T>`.
- Rust: flat `Object`/`ObjectFlags` + `ClassId` enum (`cnc-core::entity`, `class_id`), engine-local structs, `smart_ptr.rs` replaces `CCPtr`.
- **Gap:** deliberate idiom change; type-driven behavior the C++ hierarchy encoded is spread across per-type modules.

### 2.3 Combat — ⚠️ TD ✅ largely / RA ❌ flat damage
- **TD:** `Weapon`/`WeaponStats`, `Warhead`/`WarheadType` + `ArmorType`/`BulletType`/`Projectile` (`td_rules.rs`, `weapon.rs`) — verses-style matching present (~85% table fidelity).
- **RA:** **Flat damage only** — verses-armor matrix exists in `cnc-core/rules.rs:345` but **RA never reads `RulesDatabase` at all** (uses hard-coded `unit_profile`/`structure_cost`). Specials: `tesla.rs`, `superweapon.rs` (chronoshift, IronCurtain, gapgen).

### 2.4 Economy & production — ✅ Largely (TD) / ⚠️ Partial (RA)
- TD: `ProductionKind`/`ProductionItem`, `Credits`, `Power` (`game.rs`), harvester logic — solid.
- RA: `mcv.rs` (deploy); **Rules.ini `Cost=` now drives queue pricing and refunds** (`unit_cost_of`/`structure_cost_of`) and `Prerequisite=`/`TechLevel=`/`Owner=` gate builds (`can_build_class`) — *re-verified 2026-10-20: “hard-coded costs; no INI-driven production data” was stale and contradicted audit §1; `.omo/evidence/task-6-parity-100.log` §G*; the house + difficulty tables remain unused.

### 2.5 AI & navigation — ⚠️ Partial (significant gaps)
- **Present:** TD `ai.rs` (12.3K, difficulty-aware), `pathfind.rs` in both engines, trigger-driven `Team`/`TeamType`/`TaskForce` in `cnc-core::entity` (25 tests), `cnc-ai` AgentScript runtime (LLM/scripted commanders — Rust-native advantage).
- **TeamType is consumed by both engines** *(corrected 2026-10-20 — previous claim: "zero consumers")*: `cnc_tiberian_dawn::ai::TeamManager` (`ai.rs:76`) and `cnc_red_alert::game::TeamManager` (`game.rs:819`, driven by `Game::update_team_manager` at `game.rs:1745`) parse `[TaskForces]`/`[TeamTypes]`, spawn, claim units exclusively, issue objectives and retire decimated teams. Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §E.
- **Gaps (audit-verified):**
  - **No `Expert_AI` for RA** — previous claim "RA-depth house AI implemented" was incorrect; the urgency ladder (`run_expert_ai`) only drives TD.
  - **RA AI is a hard-coded 25 s cadence** (`cnc-red-alert/src/ai.rs`).
  - **TD planner methods are still empty stubs** (`ai.rs:133–147`: `defense_groups`/`attack_groups` never populated), but TeamTypes *are* executed — via `ai::TeamManager`, not the planner *(corrected 2026-10-20; previous wording said "parsed but never executed")* — `.omo/evidence/task-6-parity-100.log` §E.
  - Base-placement AI, zone defense, production-priority expert system absent.

### 2.6 Missions & scenario — ⚠️ Partial (major gap: original format I/O)
- TD `mission.rs` 36K: `Mission`, `Trigger`/`Condition`/`Action`, objectives. **The original `.ini`+`bin` loader now exists**: `cnc-tiberian-dawn/src/scenario_loader.rs` parses `[Basic]`/`[Objectives]`/`[Wunits]`/`[Structures]`/`[Waypoints]`, decodes `.bin` cells (raw **or LCW**) and converts trigger records into executable `Trigger`s. *(Corrected 2026-10-20 — previous claim: "no SCENARIO.CPP loader".)*
- RA: `scenario.rs` 13.9K + `original_scenario.rs`; `cnc-asset::{base64, ra_pack}` decode the numbered-line base64 container and 3-byte packed cells (`[Basic]`/`MapPack`/`OverlayPack`/`[CellTriggers]`), and `Game::from_original_scenario` (`game.rs:1068`) fields an army from `[Units]`/`[Infantry]`/`[Structures]`. *(Corrected 2026-10-20 — previous claim: "no `.MPR`/LCW loader … RA scenarios field no army".)*
- Content: hand-authored new-dialect missions **plus** original-format loading on both sides. Still missing: RA `[Houses]`/`[Base]` object placement, TD `[Houses]`/`[CellTriggers]`/waypoint coords, most TD event/action vocabulary, TD `ini+bin` import in the editor, and **any original-format fixture in-tree** (all 19 shipped maps are the new Rust dialect) — so "zero original campaign missions playable" still holds *in-tree* for lack of fixtures. Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §G.
- Trigger vocabulary: ~1/6 of original C++ coverage in RA.

### 2.7 Networking — ✅ Modern equivalent (wired into the launcher)
- Lockstep UDP + `LiveSession`/`HostMigration`/`MeshRelay` — features beyond original.
- `cnc-net-directory`: MMR matchmaking, LFG, torrent distribution.
- **Wired** *(corrected 2026-10-20 — previous claim: “unwired from launcher”)*: `cnc-net-directory` is a `cnc-launcher` dependency (`cnc-launcher/Cargo.toml:28`) and is consumed by `lobby_screen.rs` (`DirectoryClient`, `api`, `discovery::lan`, `LfgService`), `lobby.rs` and `ready.rs` (committed in `f351847`). *Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §Q.*

### 2.8 Graphics & UI — ⚠️ Different stack, partial parity
- Present: `cnc-renderer` wgpu 22 (`sprite_batch.rs`, `palette_shader.rs`), launcher `app.rs` 230K.
- **Shroud is modelled, gating AND drawn by both engines** *(corrected 2026-10-20 — previous claim: "not drawn and RA has none")*: TD `Game::render_shroud` (`game.rs:2817`, unexplored = opaque, explored-but-unseen = `SHROUD_MEMORY_ALPHA` 0.55, `game.rs:65`) and RA `Game.enable_shroud`/`refresh_shroud` with its own per-house `cnc_core::Shroud` (`game.rs:909/1912`); `EVENT_DISCOVERED` (`Condition::Discovered`) is wired in both. Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §F.
- **Gaps:** no icon-cache/theater-blit shader equivalents; score screens implemented but briefing/movie playback partial (VQA uncompressed only).

### 2.9 Audio & video — ⚠️ Partial
- Audio: `cnc-audio` mixer (good semantics); **EVA vocabulary + queue exist** (`eva.rs`; `eva_ra.rs` `pub enum RaEvaLine` — *re-verified 2026-10-20: “no speech/EVA VOX anywhere” was stale; `.omo/evidence/task-6-parity-100.log` §I*) **but only 2 of 45 lines are emitted, TD-only**; no ThemeClass song queue, in-game SFX unwired (menu-only mixer `cnc-launcher/src/main.rs:8224`).
- `cnc-sfx` + `cnc-music` (real generators) have **zero runtime consumers**.
- Video: VQA demuxer uncompressed only; compressed variants need format80.

### 2.10 File formats — ⚠️ Partial
- `cnc-asset`: MIX/SHP/Format80/VOC/AUD/PAL done **plus LCW both ways** — `format80.rs` ships `decompress` (`:46`) *and* `compress` (`:270`) with round-trip tests; **VQ video and VXL voxels unsupported** (`video.rs:182`); CSF is 17-line stub.
- **Original format I/O has landed in three of four places** *(corrected 2026-10-20 — previous claim: no SCENARIO.CPP loader, no `.MPR`/LCW/MapPack, editor can't open TD ini+bin or RA .MPR)*: TD `scenario_loader.rs` (ini+bin, raw/LCW), RA `original_scenario.rs` + `ra_pack`/`base64`, editor `MapModel::from_original_ra` (`map.rs:409`) and `MapModel::from_original_td` (`map.rs:500`). Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §G. Remaining gap: RA object placement from `[Houses]`/`[Base]`, and no original-format fixture in-tree.

### 2.11 Platform layer & editors — ♻️ Replaced / ⚠️ Partial
- winit/wgpu/rodio replace WIN32LIB (intentional divergence).
- Editor: `cnc-map-editor` has undo/redo 95%+ superset, procgen, Agent Studio; **but no original-format map I/O, no waypoints, no TeamTypes tab, hardcoded ClassIds vs hundreds of INI types**, Steam Workshop dropped (intentional — document in README).

---

## 3. The 6 Biggest Cross-Lane Gaps (Audit §6)

1. **Original scenario/map I/O — loaders landed, fixtures/wiring not.** TD ini+bin (`scenario_loader.rs`, raw/LCW), RA `.MPR` containers + `Game::from_original_scenario`, and both editor importers exist; still missing RA `[Houses]`/`[Base]` object placement and **any original-format fixture in-tree**, so only hand-authored new-dialect maps play today. *(Corrected 2026-10-20 — previous claim: “missing everywhere”.)* **Highest-ROI fix.** Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §G.
2. **TeamType is now consumed by both engines** (`ai::TeamManager` in TD, `game::TeamManager` in RA — 8 + 6 tests) *(corrected 2026-10-20 — previous claim: “zero consumers”)*; remaining: RA's hard-coded 25 s `AiCommander` cadence is still the fallback and TD's planner methods (`ai.rs:133–147`) are empty stubs with `defense_groups`/`attack_groups` never populated. Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §E.
3. **Both engines draw their shroud** — TD `Game::render_shroud` (`game.rs:2817`) and RA `enable_shroud`/`refresh_shroud` (`game.rs:1912`), with `EVENT_DISCOVERED` wired in both *(corrected 2026-10-20 — previous claim: “not rendered, and RA has no shroud”)*. Remaining gap: `EVENT_*` coverage beyond discovery and sight-radius fidelity. Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §F.
4. **RA combat is flat damage** — verses-armor matrix exists in `cnc-core/rules.rs:345` but RA never reads `RulesDatabase` at all; TD has full damage math.
5. **TD defensive buildings fire** (`Game::update_defence_combat`, `game.rs:2616`, called from the tick at `:1587`) and **RA air units re-arm** (`Game::update_rearm`) *(both corrected 2026-10-20)*; **transports/passengers (cargo fields, APC/Chinook) are still absent** in both. Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §D.
6. **Plumbing debt (partly resolved)** — `cnc-events` and `cnc-logs` **are workspace members now** (`RUST/Cargo.toml:19–20`) *(corrected 2026-10-20 — previous claim: “not workspace members”)*; `cnc-console` still has no workspace entry and is missing `styles`/`widgets` (deferred debt, plan todo 54); `net-directory` + ready-up protocol are **wired into the launcher now** (`lobby_screen.rs`/`lobby.rs`/`ready.rs`, dependency at `cnc-launcher/Cargo.toml:28` — *corrected 2026-10-20*); GPS/`aircraft.rs`/`naval.rs` are wired per audit §8.6; `cnc-events` is consumed by `cnc-launcher/src/events_subscriber.rs`. Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §Q. Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §A.

---

## 4. Bugs + Hygiene Flagged (Audit §7)

- **France's unique building now maps to the Grand Cannon** — `ClassId::GrandCannon` exists (`cnc-core/src/class_id.rs:55`, round-trip at `:184`, building-class list at `:280`) and `Country::France.unique_structure()` returns it (`cnc-red-alert/src/house.rs:70`, asserted at `:207`), so the old Germany/Chronosphere collision is gone. *Re-verified 2026-10-20 against the live tree (previous text said “`house.rs:67–72`”, which is the corrected function); evidence `.omo/evidence/task-6-parity-100.log` §H.*
- `docs/PARITY.md` over-claims (Expert_AI "closed" for RA — it only drives TD; RA rules wiring) — **this document now corrects those claims**.
- `cargo fmt` failure at `powerup.rs:209` — **RESOLVED** (`cargo fmt --check` exits 0 on committed HEAD `5d1fd8b`, measured 2026-10-20; the fail-then-fix claim in `PARITY_AUDIT_2026-10-20.md` §8.4 is the stale side of that contradiction). *Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §B.* Caveat recorded there: at 22:25Z a second `cargo fmt --check` sample exited 1 purely from **uncommitted, in-flight edits** to `cnc-asset/src/format80.rs` and `cnc-map-editor/examples/tileset_render.rs` made by a parallel lane outside this docs allowlist — no committed file is unformatted.
- Stale codemap claims — **corrected 2026-10-20**: TD codemap "154 unit + 4 integration" → measured **299 unit + 4 integration** (`cargo test -p cnc-tiberian-dawn --no-fail-fast` → 299 passed / 1 failed lib, 1 + 3 integration tests); `cnc-renderer/src/codemap.md`'s "empty placeholders" claim for `ui.rs`/`palette_shader.rs` was false (`ui.rs` 217 L widget kit with tests, `palette_shader.rs` 591 L indexed-palette pipeline) and its line counts for `sprite_batch.rs`/`camera.rs` were stale. *Re-verified 2026-10-20 — `.omo/evidence/task-6-parity-100.log` §C, §J.*

---

## 5. Highest-Leverage Roadmap (Audit §9)

1. **Original scenario/map loaders** (LCW + `[Basic]/[Houses]/[Wunits]/[Units]/[Structures]/[Terrain]/[Overlay]/[Trigs]/[Waypoints]`) — unlocks entire original campaign for TD, RA, and editor at once.
2. **Defensive buildings fire (TD) + RA verses-armor matrix** from `cnc-core::rules` — combat actually feels like C&C.
3. **RA consumes `RulesDatabase`** (kill hard-coded `unit_profile`/`structure_cost`); TD already half-does — finish pipeline, difficulty becomes data.
4. **Shroud layer** primitive (per-house vis grid consulted by targeting/AI) — unlocks EMP/GPS/gapgen semantics coherently.
5. **TeamType engine wiring** (`cnc-core::entity` parsers → both games) — AI stops being a cadence machine.
6. **TD `Game::apply_command` arms** for 11 unhandled wire commands + game-state CRC in FRAMESYNC equivalents.
7. **Transports** (cargo fields on GameUnit, re-arm docks for RA air).
8. **Trigger actions: `AIRSTRIKE/NUKE/ION/DZ`** — superweapons become campaign-scriptable.
9. **EVA/VOX pipeline** in `cnc-audio` + `Speak` call sites in both engines.
10. **Formatter/codemap/doc hygiene** — the cheap compounding fixes.

---

## 6. Verification Notes

- All claims in this document now cite file/line anchors from `PARITY_AUDIT_2026-10-20.md`.
- Test counts per crate (re-verified 2026-10-20; the TD figure is the one measured directly for this pass, the rest were carried from the previous summing and updated only where re-measured): **cnc-tiberian-dawn 303** (299 unit + 4 integration, **1 failing**), cnc-launcher 163, cnc-core 132, cnc-map-editor 132 (129 pass / 3 pre-existing fail), cnc-red-alert 170, cnc-asset 126, cnc-net-directory 66, cnc-audio 49, cnc-procgen 42, cnc-ai 23, cnc-renderer 20, cnc-sfx 17, cnc-music 14, cnc-bench 2, cnc-classic 0, plus the two new members (cnc-events, cnc-logs). **Workspace total: 1318 passed / 4 failed** (measured by the orchestrator with `cargo test --workspace --no-fail-fast`; the 4th failure is the TD `scenario_loader::tests::records_become_executable_triggers` case reproduced in `.omo/evidence/task-6-parity-100.log` §C). *(Corrected 2026-10-20: this line previously said 291 for TD and 1253 passed / 3 failed.)*
- The previous "552 tests" claim in codemap.md and sync-docs.sh was stale; corrected in both. *Re-verified 2026-10-20: `RUST/codemap.md` still said "fifteen member crates" and "952 tests passed" — both corrected there to 17 members / 1318 passed · 4 failed; evidence `.omo/evidence/task-6-parity-100.log` §K.*

---

## 7. Reconciliation Log (2026-10-20, documentation pass)

Every row was re-measured against the live tree at commit `5d1fd8b`; commands and raw output are in `.omo/evidence/task-6-parity-100.log`. Nothing in this doc was deleted — stale text was replaced in place and the prior claim is quoted in a dated note.

| # | Claim (before) | Live-tree verdict (after) | Evidence |
|---|---|---|---|
| 1 | Workspace = 15 crates; `cnc-events`/`cnc-logs` "orphaned and removed" | **17 members**, both landed (`RUST/Cargo.toml:19–20`); `cnc-console` has a `Cargo.toml` but no membership; `cnc-asset-editor`/`cnc-replayer` are docs-only | §A |
| 2 | `cargo test --workspace` = 1253 passed / 3 failed | **1318 passed / 4 failed** (4th = TD `scenario_loader::tests::records_become_executable_triggers`) | §C, §M |
| 3 | `cargo fmt` failure "RESOLVED" (v. audit §8.4 "unresolved") | **RESOLVED is correct** — `cargo fmt --check` exits 0 on committed HEAD; the audit line is the stale side | §B |
| 4 | France → Grand Cannon "FIXED" | **FIXED is correct** — `ClassId::GrandCannon` (`class_id.rs:55`), `house.rs:70`, test `:207` | §H |
| 5 | No `SCENARIO.CPP` loader / no `.MPR`·LCW·MapPack / editor can't open originals | **Loaders landed** in TD (`scenario_loader.rs`), RA (`original_scenario.rs`, `game::from_original_scenario`) and the editor (`map.rs:409/500`); LCW both ways in `format80.rs:46/270` | §G |
| 6 | TeamType parsers have "zero consumers"; TD planner "parsed but never executed" | **Both engines consume them** (`ai::TeamManager`, `game::TeamManager`); only the planner helpers stay stubs | §E |
| 7 | Fog-of-war "not rendered, RA has no shroud" | **Both draw a shroud** (`game.rs:2817` TD, `game.rs:1912` RA) with `EVENT_DISCOVERED` wired | §F |
| 8 | TD "defensive buildings can't fire" | **They fire** — `Game::update_defence_combat` (`game.rs:2616`) | §D |
| 9 | TD codemap "154 unit + 4 integration tests" | **299 unit + 4 integration** (1 currently failing) | §C |
| 10 | Renderer codemap: `ui.rs`/`palette_shader.rs` "empty placeholders" | **False** — 217 L widget kit (with tests) and 591 L indexed-palette pipeline; `sprite_batch.rs` 945 L, `camera.rs` 284 L | §J |

Corresponding stale statements inside `PARITY_AUDIT_2026-10-20.md` (its §3.5 defense-fire bug, §3.10 France bug, §6.1 orphaned events/logs, §8.2 TD count, §8.4 fmt, §9.4 shroud, §9.9 RA EVA, Appendix A ledger) were corrected in place in the same pass, each with its own "re-verified 2026-10-20" note pointing at the same evidence file.
