# Feature Parity & Gap Analysis — Rust Workspace vs. Original C++ Engines

**Status:** Updated 2026-10-20 per `PARITY_AUDIT_2026-10-20.md` (authoritative). This doc now reflects source-verified parity levels; over-claims from the 2026-10-19 snapshot have been corrected.

**Companions:** [`PARITY_AUDIT_2026-10-20.md`](PARITY_AUDIT_2026-10-20.md) (full audit with file/line anchors) · [`../ORIGINAL_FUNCTIONALITY_MAP.md`](../../ORIGINAL_FUNCTIONALITY_MAP.md) (original functionality, file-level).

**Method:** Four parallel source-level audits (grep/read-verified on both sides, no codemap-only claims). Original-side inventory from `ORIGINAL_FUNCTIONALITY_MAP.md` (RTTI enums, class inventory, full `.CPP` lists). Rust-side evidence from fresh source inspection: workspace members (`Cargo.toml`), per-crate source listings, symbol extraction (`pub struct/enum/fn`), keyword sweeps, `BUILD_STATUS.md` ledger.

**Workspace:** 15 crates — `cnc-core`, `cnc-tiberian-dawn`, `cnc-red-alert`, `cnc-renderer`, `cnc-audio`, `cnc-asset`, `cnc-map-editor`, `cnc-launcher`, `cnc-net-directory`, `cnc-ai`, `cnc-procgen`, `cnc-sfx`, `cnc-classic`, `cnc-music`, `cnc-bench`. (cnc-console, cnc-events, cnc-logs are orphaned and removed from workspace — see audit §6.)

---

## 1. Executive Summary (Audit-Verified)

| Lane | Overall Parity | Notes |
|---|---|---|
| **TD engine** (`cnc-tiberian-dawn`) | **~45–50%** | Weapons/combat tables near-verbatim (~85%); original `.ini`+`.bin` scenario loading, defensive structures fire, per-house shroud gating acquisition; team AI ~25%, no fog rendering |
| **RA engine** (`cnc-red-alert`) | **~55% subset / ~35% content fidelity** | Flat damage combat (~30%), zero RULES.INI consumption, ~1/6 trigger vocabulary |
| **Map editor** (`cnc-map-editor`) | **~30–40%** | Undo/redo 95%+ (93 tests vs 0 in C#); original-format map I/O 0–20% |
| **Support crates** | Mixed | Sim/netcore solid; presentation (speech, VQ, in-game SFX) partial/absent |

**Verified build state (measured 2026-10-20):** `cargo check --workspace --all-targets` clean (0 errors); `cargo fmt --check` clean; `cargo test --workspace --no-fail-fast` = **1253 passed / 3 failed**. All 3 failures are in `cnc-map-editor` (`layout_controls_stay_inside_the_window`, `trigger_filter_matches_group_and_name`, `trigger_inspector_controls_and_frame_render`) and **reproduce with every pending working-tree change reverted**, i.e. they pre-date this work. A fourth failure (`cnc-launcher::draw_briefing_video_falls_back_on_unsupported_variant`) was a stale test expectation and has been fixed. The earlier "952 passed / 0 failed" figure came from a `BUILD_STATUS.md` ledger and was not reproducible by `cargo test`.

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
- RA: `mcv.rs` (deploy), hard-coded costs; no INI-driven production data.

### 2.5 AI & navigation — ⚠️ Partial (significant gaps)
- **Present:** TD `ai.rs` (12.3K, difficulty-aware), `pathfind.rs` in both engines, trigger-driven `Team`/`TeamType`/`TaskForce` in `cnc-core::entity` (25 tests, **zero consumers** in either engine), `cnc-ai` AgentScript runtime (LLM/scripted commanders — Rust-native advantage).
- **Gaps (audit-verified):**
  - **No `Expert_AI` for RA** — previous claim "RA-depth house AI implemented" was incorrect; the urgency ladder (`run_expert_ai`) only drives TD.
  - **RA AI is a hard-coded 25 s cadence** (`cnc-red-alert/src/ai.rs`).
  - **TD planner methods are no-op stubs** — TeamTypes parsed but never executed.
  - Base-placement AI, zone defense, production-priority expert system absent.

### 2.6 Missions & scenario — ⚠️ Partial (major gap: original format I/O)
- TD `mission.rs` 36K: `Mission`, `Trigger`/`Condition`/`Action`, objectives — **but no `SCENARIO.CPP` loader** for original `.ini`+`bin` campaign files.
- RA `scenario.rs` 13.9K — **no `.MPR`/LCW loader** for original RA missions.
- Content: Hand-authored new-dialect missions only; **zero original campaign missions playable**.
- Trigger vocabulary: ~1/6 of original C++ coverage in RA.

### 2.7 Networking — ✅ Modern equivalent (but unwired from launcher)
- Lockstep UDP + `LiveSession`/`HostMigration`/`MeshRelay` — features beyond original.
- `cnc-net-directory`: MMR matchmaking, LFG, torrent distribution.
- **Gap:** `net-directory` + ready-up protocol built but **unwired from launcher**.

### 2.8 Graphics & UI — ⚠️ Different stack, partial parity
- Present: `cnc-renderer` wgpu 22 (`sprite_batch.rs`, `palette_shader.rs`), launcher `app.rs` 230K.
- **Gaps:** Shroud is implemented and gates TD targeting, but is not drawn and RA has none; no icon-cache/theater-blit shader equivalents; score screens implemented but briefing/movie playback partial (VQA uncompressed only).

### 2.9 Audio & video — ⚠️ Partial
- Audio: `cnc-audio` mixer (good semantics); **no speech/EVA VOX anywhere, no ThemeClass song queue, in-game SFX unwired** (menu-only mixer `cnc-launcher/src/main.rs:8224`).
- `cnc-sfx` + `cnc-music` (real generators) have **zero runtime consumers**.
- Video: VQA demuxer uncompressed only; compressed variants need format80.

### 2.10 File formats — ⚠️ Partial
- `cnc-asset`: MIX/SHP/Format80/VOC/AUD/PAL done; **VQ video and VXL voxels unsupported** (`video.rs:182`); CSF is 17-line stub.
- **Original format I/O missing everywhere:** no SCENARIO.CPP loader, no `.MPR`/LCW/MapPack, editor can't open TD ini+bin or RA .MPR.

### 2.11 Platform layer & editors — ♻️ Replaced / ⚠️ Partial
- winit/wgpu/rodio replace WIN32LIB (intentional divergence).
- Editor: `cnc-map-editor` has undo/redo 95%+ superset, procgen, Agent Studio; **but no original-format map I/O, no waypoints, no TeamTypes tab, hardcoded ClassIds vs hundreds of INI types**, Steam Workshop dropped (intentional — document in README).

---

## 3. The 6 Biggest Cross-Lane Gaps (Audit §6)

1. **Original scenario/map I/O missing everywhere** — No SCENARIO.CPP loader, no `.MPR`/LCW/MapPack, editor can't open TD ini+bin or RA .MPR; only hand-authored new-dialect maps play. **Highest-ROI fix.**
2. **TeamType AI parsers exist (`cnc-core::entity`, 25 tests) but zero consumers** in either engine; RA AI is hard-coded 25 s cadence; TD planner methods are no-op stubs.
3. **Fog-of-war is not rendered, and RA has no shroud.** TD models visibility per house and gates acquisition/defensive fire on it, but nothing draws it.
4. **RA combat is flat damage** — verses-armor matrix exists in `cnc-core/rules.rs:345` but RA never reads `RulesDatabase` at all; TD has full damage math.
5. **TD defensive buildings can't fire; RA air units never re-arm; transports/passengers absent** in both.
6. **Orphaned/broken plumbing** — `cnc-console` missing styles/widgets (uncompilable when added to workspace); `cnc-events`/`cnc-logs` not workspace members; `net-directory` + ready-up protocol built but unwired from launcher; GPS superweapon and RA `aircraft.rs`/`naval.rs` structs tested-but-unreachable.

---

## 4. Bugs + Hygiene Flagged (Audit §7)

- **France's unique building silently maps to Germany's Chronosphere** (`cnc-red-alert/src/house.rs:67–72`) — **FIXED:** GrandCannon added to `ClassId`, France now maps correctly.
- `docs/PARITY.md` over-claims (Expert_AI "closed" for RA — it only drives TD; RA rules wiring) — **this document now corrects those claims**.
- `cargo fmt` failure at `powerup.rs:209` — **RESOLVED** via `cargo fmt`.
- Stale codemap claims (TD "154 tests", renderer stub claims) — recorded in audit appendix.

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
- Test counts per crate (verified 2026-10-20 by summing `--no-fail-fast` output): cnc-tiberian-dawn 291, cnc-launcher 163, cnc-core 132, cnc-map-editor 132 (129 pass / 3 pre-existing fail), cnc-red-alert 170, cnc-asset 126, cnc-net-directory 66, cnc-audio 49, cnc-procgen 42, cnc-ai 23, cnc-renderer 20, cnc-sfx 17, cnc-music 14, cnc-bench 2, cnc-classic 0. Total: 1253 passed / 3 failed.
- The previous "552 tests" claim in codemap.md and sync-docs.sh was stale; corrected in both.
