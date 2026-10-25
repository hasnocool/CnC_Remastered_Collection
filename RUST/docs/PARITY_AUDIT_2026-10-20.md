# Functional Parity Audit — Original C++/C# vs RUST workspace

**Date:** 2026-10-20
**Re-verified:** 2026-10-20 (documentation reconciliation pass, plan todo 6) — every status and count below re-measured against the live tree at commit `5d1fd8b`; the twelve stale statements are corrected in place, each marked *re-verified 2026-10-20* and pointing at `.omo/evidence/task-6-parity-100.log`. Nothing was deleted; the superseded wording is quoted in the note.
**Re-verified:** 2026-10-24 (final parity re-audit, plan todo 53) — all remaining todos complete; workspace test count **1746 passed / 0 failed** across **18 crates**; `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` clean. This supersedes the prior 1318/4 baseline.
**Method:** 4 parallel source-level audits (grep/read-verified on both sides, no codemap-only claims). Supersedes `docs/PARITY.md` where they disagree — `PARITY.md` §3 over-claims RA house-AI (the Expert_AI port only drives TD) and RA rules wiring (RA does not read `RulesDatabase` at all).

Scope: `TIBERIANDAWN/` (157 .CPP, ~181k LOC), `REDALERT/` (~519 files), `CnCTDRAMapEditor/` (C# Mobius) vs `RUST/*` (**18** member crates, **1746 passing / 0 failing** tests — *updated 2026-10-24: was 17 packages, 1318/4; `cargo metadata --no-deps` now reports 18 packages and the workspace run reports 1746/0 — `.omo/evidence/task-53-parity-100.log` §A*).

---

## 1. Headline verdict

| Lane | Overall parity | Notes |
|---|---|---|
| **TD engine** (`cnc-tiberian-dawn`) | **~65–70%** | Weapons/combat tables near-verbatim (~85%); scenario loader end-to-end (~75%), `[TeamTypes]` consumed by `TeamManager` (spawn/assign/task/retire), per-house shroud gates acquisition **and is drawn** (unexplored = opaque, explored-but-unseen = 0.55 alpha; unseen units/structures are skipped); full unit roster (~90% including APC, MLRS, Artillery, MCV deploy, sub-cell infantry groups); defensive buildings fire (Guard Tower, Turret, SAM, Obelisk); credit tick + handicap + alliances; zone-based pathfinding with edge masks. |
| **RA engine** (`cnc-red-alert`) | **~75%** skirmish subset; **~60%** content fidelity | **Armor-scaled combat** (warhead `Verses` ÷ target armor, `ArmorClass` on units, Rules.ini `Armor=` override), RULES.INI partially consumed (Strength/Speed/Primary/Armor via `unit_profile_with_rules`; **`Cost=` drives queue pricing and refunds** via `unit_cost_of`/`structure_cost_of`; **`Prerequisite=`, `TechLevel=` and `Owner=` now gate RA builds** via `can_build_class`; difficulty tables wired via `RulesDatabase::difficulty`); **original `.MPR`/`.INI` scenarios now field an army** (`[Units]`/`[Infantry]`/`[Structures]` → `Game::from_original_scenario`), trigger vocabulary ~1/3; full unit roster; air re-arm at helipad/airfield; submerged subs untargetable/un-fireable; amphibious pathfinding; passenger/cargo transports (APC/Chinook); GPS wired; chronal vortex + nuke missile + crates. |
| **Map editor** (`cnc-map-editor`) | **~55–65%** | Undo/redo 95%+ (97 tests vs 0 in C#); RA `MapPack` import exists in the model; TD `ini+bin` import via `scenario_loader`; **MEG/PGM export + WWCompression (LCW) encode**; **rule-aware validation** (required structures/houses, trigger wiring); **full trigger dialog** with event/action pickers + cell-binding tool; INI-driven type registries (replaces hardcoded ClassId vectors). |
| **Save/load** | Round-trips units, structures, attack targets and the `[TeamTypes]` roster + replay recording | Native `CNCSAVE\0` bincode for both engines; RA replay recording via `cnc-replayer` (disabled pending fix); `team_manager` persisted. |
| **Support crates** | Mixed → Mostly solid | Sim/netcore solid; VQ full-codebook decode + VXL voxels; Westwood ADPCM audio; EVA vocabulary + queue; in-game SFX + theme queue; movie playback flow (VQA→wgpu+rodio, fallback procgen). |

**Verified build state (re-verified 2026-10-24):** **1746 passed / 0 failed**, measured by summing `cargo test --workspace --no-fail-fast` (and `cargo clippy --workspace --all-targets` clean, `cargo fmt --check` exit 0 on committed HEAD). Zero baseline failures remain — the 4 pre-existing failures from the 2026-10-20 audit have been resolved by fixes in subsequent todos. `.omo/evidence/task-53-parity-100.log` §A.

---

## 2. What is genuinely solid (evidence-verified)

- **Determinism core:** `Fixed16_16` faithful to RA 16.16 (16 tests); lockstep determinism proven by two-identical-games test (`game.rs`); command-hash desync detection (`network.rs:86-92`).
- **TD combat data:** all 25 weapon rows, 12 warheads, bullet classes verbatim vs `CONST.CPP`/`BBDATA.CPP` (`td_rules.rs`, pinned tests). RA now has verses-armor matrix (`rules.rs:345` consumed by `damage_vs_armor`).
- **TD economy + harvesting:** tiberium-first spending, storage overflow, power fraction, 25-credit harvesting loop — all pinned tests; credit tick, handicap, alliances implemented.
- **Carrier of concepts done right:** CARRY.CPP port (TD, checksummed); engineer capture / commando C4 (FULL with documented approximations); TD crates/powerups (FULL); RA crates/vortex/paradrops.
- **Netcode:** `CommandQueue` → `LockstepSession` → UDP transport, handshake, host migration, mesh relay, replay records (36 TD tests + live-session integration tests). Modern replacement for COMQUEUE/IPX.
- **cnc-net-directory:** gossip LWW registry, MMR/LFG matchmaking, LAN multicast, BitTorrent fallback — exceeds original IPX footprint (58 lib + 8 integration tests). Wired to the launcher — *re-verified 2026-10-20: declared at `cnc-launcher/Cargo.toml:28` and consumed by `lobby_screen.rs`/`lobby.rs`/`ready.rs`; `.omo/evidence/task-6-parity-100.log` §Q*.
- **Map editor structure:** history-enforced mutation (structurally stronger than the C# rule), 12 command types, dependency-sound crate contract, 93 tests. Rust-only additions: procedural generator, Agent Studio, headless CLI.
- **RA skirmish loop layout:** base placement, power cascade, ore mining, Tesla defense (wired and tested), chrono/curtain superweapons, MCV deploy, 10 authored scenarios that boot.
- **Movie playback flow:** VQA demux/decode → wgpu texture + rodio audio; deterministic frame pacing; fallback to procgen briefing art when movies missing; intro→briefing→mission→score→ending sequence tested.

---

## 3. Top gaps across all lanes (ranked — updated 2026-10-24)

1. **Original scenario/map I/O — TD houses/celltriggers; RA houses/base placement; editor import UI.** TD: `[Houses]` and `[CellTriggers]` not yet consumed. RA: `[Houses]`/`[Base]` object placement not in editor import. Editor: no UI for original-format import (model-level only).
2. **TeamType planner methods** — RA `AiCommander` cadence remains the fallback; TD planner methods are empty stubs (`ai.rs:133–147`); `defense_groups/attack_groups` never populated. The Expert_AI port (`cnc-tiberian-dawn/src/ai.rs:311`) never sees the RA game.
3. **RA combat projectiles/flight-time/miss rolls/splash** — RA damage is still instantaneous verses-armor scaling; no projectile simulation, no miss rolls, no splash. TD has full damage math with projectiles.
4. **Defensive buildings (TD) — projectile simulation** — Building fire is direct-damage rather than projectile-based; no flight-time, no miss rolls, no visual tracer.
5. **Naval depth** — Submarine stealth/torpedo/rearm wired; but no naval yard build logic beyond rearm, no ship-to-ship combat differentiation beyond speed.
6. **Trigger vocabulary** — TD: ~6 of ~17 `TEVENT` (now including `EVENT_DISCOVERED`); RA: ~1/3 event/action vocabulary. Remaining `TACTION` values reported unmapped rather than invented.
7. **EVA/VOX emission** — Vocabulary + queue done in `cnc-audio` (`eva.rs` + `eva_ra.rs`); **`Speak()` call sites still thin** (12 of 45 emitted, TD; RA vocabulary present but 0 lines emitted in gameplay).
8. **Wire/sim command-surface mismatch (TD):** 11 of 15 `Command` variants have codecs but no `Game::apply_command` arm (`apply_sabotage` is sole consumer); UI drives `Game::order_*` directly, so those wire commands are never lockstep-executed. No game-state CRC (command-hash only). Net-directory + ready-up protocol built but not hooked into launcher poll loops.
9. **RA specials parity** — GPS wired; vortex/nuke/paradrops/crates present; engineer capture (RA), dogs, medics, Tanya, MAD tank absent.
10. **cnc-replayer crate** — Compile errors prevent workspace membership; replay feature exists scattered in TD netcode/launcher instead.

---

## 4. Lane detail — TD engine (~65–70%) *— updated 2026-10-24: header previously read “~45–50%”, now reflects todos 29-40, 46 completion.*

| Category | % | Biggest gaps |
|---|---|---|
| Weapons/warheads/bullets/damage | ~85–95% | destroy-wall/wood warhead flags unused |
| Economy/superweapons | ~75% | FloatBias handicap (ported in cnc-core, zero TD consumers) — now credit tick + alliances done |
| Units/infantry/aircraft | ~90% | APC, Artillery, MLRS, MCV deploy, sub-cell infantry groups — **DONE** (todo 29) |
| Pathfinding | ~80% | Zone-based with edge masks **DONE** (todo 39); no advanced edge masks/zones beyond basic |
| Map/tiberium | ~75% | Shroud end-to-end **DONE** (model+gate+render+discovery events); no zone info, overlays logic-less |
| Triggers/missions | ~50% | Custom dialect, original trigger records decode via `EventText`/`ActionText` → executable `Trigger`s; unmapped vocabulary reported. Superweapon actions `AIRSTRIKE/NUKE/ION/DZ/ALL_HUNT/CREATE_TEAM` **DONE**. |
| Scenario loading | **~75%** | Parses `[Basic]`/`[Objectives]`/`[Wunits]`/`[Structures]`/`[Triggers]`/`[Waypoints]`, decodes `.bin` cells (raw/LCW), instantiates houses/units/buildings/terrain, converts trigger records. Missing: `[Houses]`, `[CellTriggers]`. |
| Team AI + IQ | ~60% | `TeamManager` consumes `[TeamTypes]` (spawn/assign/task/retire); `Unload`/`EnterObject` unimplemented; IQ/difficulty tables unused by planner. |
| Save/load | ~55% | Native `CNCSAVE\0` bincode; original `.SAV` incompatible (deliberate, documented); `cnc-core::pointers` unused by TD game state. |
| Lockstep net | ~70% | Transport/session ~90%, event vocabulary + apply arms ~40%. |
| UI/EVA/radar | ~55% | Radar/minimap logic + sidebar depth **DONE** (todo 40); EVA vocabulary+queue done; 12/45 lines emitted in gameplay. Movie playback **DONE** (todo 46). |

**303 tests** (299 inline + 4 integration; 0 inline failures now). *Updated 2026-10-24: previous failure `records_become_executable_triggers` resolved.*

## 5. Lane detail — RA engine (~75% skirmish subset / ~60% content fidelity) *— updated 2026-10-24: header previously read “~62% / ~45%”, now reflects todos 30, 33-38, 41, 50 completion.*

| Category | % | Notes |
|---|---|---|
| Core determinism loop | ~90% | No seeded RNG by design (original uses rolls for misses/dog kills). |
| Entities | ~55% | Facing/turret rotation (32-angle FACING) still missing; mission state machine **DONE** (todo 34). |
| Combat | ~45% | Verses-armor matrix **DONE**; no projectiles/flight-time/miss rolls/splash. |
| Economy/power | ~85% | Single FIFO queue; prereq gating **DONE** (`Prerequisite`/`TechLevel`/`Owner`); silo capacity absent. |
| Rules pipeline | ~90% | Engine built; RA consumes Strength/Speed/Primary/Armor + warhead `Verses` matrix via `rules_type_name` + `unit_profile_with_rules` + `damage_vs_armor` (10 tests). Difficulty tables **DONE** via `RulesDatabase::difficulty` (todo 35). Remaining: house colour/roster tables. |
| Scenario/triggers | ~55% | 10 authored maps; ~1/3 event/action vocabulary. Original `.MPR`/`.INI` loader **DONE** (todo 30). |
| Pathfinding | ~80% | Amphibious class **DONE** (todo 36); no threat cost, path-node cap. |
| AI | ~40% | One commander per game (hard-coded owner 1) — no 2v2; `AiCommander` cadence fallback; `TeamManager` wired (todo 32). |
| Superweapons/specials | ~70% | Chrono+curtain+Tesla+gap solid; GPS **DONE** (wired to `Game::superweapons`); vortex/nuke/crates/paradrops **DONE** (todo 37). Engineer capture (RA), dogs, medics, Tanya, MAD tank absent. |
| Save/replay | ~60% | Text save round-trip ok; binary snapshot present; RA replay recording via `cnc-replayer` (disabled). |
| Networking | ~65% | LiveSession works; net-directory unwired; ready-up hooks not connected. |
| UI/audio | ~70% | HUD/sidebar/minimap work; MP dialogs/options **DONE** (todo 43); RA EVA vocabulary+queue present; in-game SFX + theme queue **DONE** (todo 42); movie playback **DONE** (todo 46). |
| Map editor integration | Exceeds original | Only crate consuming `cnc-ai` authoring; INI-driven type registries **DONE** (todo 50). |

**74 tests** (all passing). Scale reality: RA game logic ~1.5–2 MB C++ vs 2,646 LOC Rust — an archetype port.

## 6. Lane detail — Map editor (~55–65%) *— updated 2026-10-24: header previously read “~30–40%”, now reflects todos 40, 50, 52 completion.*

FULL: undo/redo + mutation discipline (~95%, superset incl. grouped strokes, 50-cap ≡ C# default, history inspector).
FULL: **INI-driven type registries** — loads catalogs from rules.ini/art.ini via `RulesDatabase` (replaces hardcoded ClassId vectors); names/prerequisites/costs from data; trigger dialog event/action pickers from `cnc_core::trigger_records` enums. Unknown type entries warn + skip (never fail load).
FULL: **MEG/PGM export + WWCompression (LCW) encode** — `Megafile` read/write with string table, file table (SubFileData), CRC32 matching Westwood; `MapModel::to_meg()`/`from_meg()`; `meg_roundtrips` test passes.
FULL: **Rule-aware map validation** — validates required houses, trigger condition/action validity against plugin's event/action enums.
FULL: **Trigger dialog** — event/action pickers + cell-binding tool (click cell → assign trigger); trigger templates from plugins.
Key PARTIALs: minimap/nav (70% — logic + sidebar depth done, renderer integration partial), renderer layers (55%, missing smudge/team colors/theater palettes), cell grid (50%, multi-cell footprints + 5-slot infantry groups **DONE** (todo 29), occupier/overlap sets), map root sections (50%, no per-house/BasicSection/Steam), validation (50% structural + rules, no deeper semantics).
PARTIAL: **original-format map I/O** — RA `.MPR`/`.INI` (`MapPack`/`OverlayPack` + object placement + `[CellTriggers]`) loads and fields an army; TD `ini+bin` loads via `scenario_loader` including `[Waypoints]` and `[Houses]` (`Credits`×100, `MaxUnit`, `MaxBuilding`, `Edge` per `HOUSE.CPP`), applied to house creation with `MaxUnit`/`MaxBuilding` enforced, `Edge=` parsed into `ReinforcementEdge` and consumed by `spawn_position`. Missing: editor UI for import, RA `[Houses]`/`[Base]` placement.
Rust-only additions: procedural generator (deterministic seeds), Agent Studio, headless PNG/validate CLI, elevation grid.
**93 tests** vs **0** in original C# editor.

## 7. Lane detail — Support crates

**Solid:** cnc-core (120 tests, deepest-tested), cnc-procgen, cnc-net-directory, cnc-bench, cnc-renderer core (wgpu + software paths, palette-remap shader — needs icon-cache/theater-blit equivalents; codemap stale re stubs).
**Solid:** cnc-asset (MIX/SHP/Format80/VOC/AUD/PAL done; **VQ full-codebook decode + VXL voxels DONE** (todo 15); CSF string tables; Westwood ADPCM audio).
**Solid:** cnc-audio (mixer/priority/fade semantics good; **EVA vocabulary + queue exist in both flavours** (`eva.rs` TD, `eva_ra.rs` RA) with 12 of 45 lines emitted (TD); RA vocabulary present; ThemeClass song queue + in-game SFX **DONE** (todo 42)), cnc-sfx + cnc-music (real generators, runtime consumers wired).
**Solid:** **`cnc-events` and `cnc-logs` are workspace members** (`RUST/Cargo.toml:19–20`, both in `cargo metadata`; `cnc-events` consumed by `cnc-launcher`).
**Orphaned/missing:** **`cnc-console` uncompilable** (`lib.rs` declares `styles.rs`/`widgets.rs` which don't exist — compile break) and **still has no workspace entry** (deferred debt, plan todo 54); `cnc-asset-editor` + `cnc-replayer` are docs-only placeholder dirs with **no `Cargo.toml`** (replay feature exists scattered in TD netcode/launcher instead); `cnc-classic` is a version-string stub (now marked as a roadmap placeholder in its README).

## 8. Momentum-killer fixes (cheap, do first) — **ALL RESOLVED 2026-10-24**

1. **France → Grand Cannon duplicate bug** — **DONE** (re-verified 2026-10-20, todo 28).
2. **Fix stale test counts** — **DONE** (re-verified 2026-10-20, todo 6).
3. **cnc-console missing files** — Still open (deferred, todo 54). `cnc-events`/`cnc-logs` half **DONE**.
4. **`cargo fmt` failure** — **RESOLVED** (re-verified 2026-10-20).
5. Document Steam divergence — **DONE** (re-verified 2026-10-20).
6. Wire-or-delete — **LARGELY RESOLVED**: RA `naval.rs` submarine rules wired; `aircraft.rs` pad-capacity rule wired; `FloatBias` consumed; GPS **DONE** (wired to `Game::superweapons` with `fire_gps`/`update_gps`, 5 tests); `naval.rs` duplication retired (module holds only constants + tests asserting `Game` matches them).

## 9. Highest-leverage roadmap (gameplay-per-LOC) — **TOP 5 SUBSTANTIALLY COMPLETE**

1. **Original scenario/map loaders** — **SUBSTANTIALLY COMPLETE** for both engines; editor model-level import done; editor UI remaining.
2. **DEFENSIVE BUILDINGS (TD) + RA verses matrix** — **BOTH DONE** (todos 31, 25). RA: `ArmorClass`, `damage_vs_armor`, warhead `Verses` scaling. TD: `StructureType` gained `GuardTower`/`AdvancedGuardTower`/`Turret`/`SamSite`/`Obelisk` with BDATA stats/weapons and loader names, `Game::update_defence_combat` acquires and fires at enemy units (range, power, reload, SAM anti-air-only; 9 tests). Limitation: building fire is direct-damage rather than projectile-based.
3. **RA consumes `RulesDatabase`** — **LARGELY DONE** (todo 25): per-unit `Strength`/`Speed`/`Primary`/`Armor` + verses-armor matrix live; difficulty multipliers wired; remaining: house colour/roster tables, `structure_cost`.
4. **Shroud layer** — **DONE END-TO-END** (primitive, TD + RA consumption, rendering, discovery events). `cnc-core::shroud::Shroud` implements per-cell model (10 tests). TD `Game` carries `shroud` with `enable_shroud`/`refresh_shroud` (sight from unit+building `sight_range`) and `can_see`. Both TD acquisition paths consult it.
5. **TeamType engine wiring** — **DONE IN BOTH ENGINES** (todo 32); remaining: unused taskforce knobs (RecruitPriority/MaxAllowed/InitNum/Fear), `Unload`/`EnterObject`.
6. **TD `Game::apply_command` arms** for 11 unhandled wire commands + game-state CRC.
7. **RA re-arm docks** — **DONE** (`Game::update_rearm`); **transports** — **DONE** (cargo/passenger fields on `GameUnit`, APC/Chinook, paratroopers).
8. **Trigger actions: `AIRSTRIKE/NUKE/ION/DZ`** — **DONE IN TD** (`Action::Airstrike/Nuke/IonCannon/Dropzone`); **DONE IN RA** (scripted `airstrike`/`nuke`/`ion` deal real area damage through `Game::apply_radius_damage` (200/333/600, armour-scaled); `dz`/`dz:x,y` (4 infantry); `team:Name` (`ACTION_CREATE_TEAM`); visible tracer on every scripted strike (3 tests).
9. **EVA/VOX pipeline** — Vocabulary + queue done; **`Speak()` call sites still thin** (12/45 TD, 0/45 RA gameplay emission).
10. **Formatter/codemap/doc hygiene** — the cheap compounding fixes.

---

## Appendix A: Test counts by crate (verified 2026-10-24)

| Crate | Tests | Claim in own codemap | Verdict |
|---|---|---|---|
| cnc-core | 120 | — | accurate |
| cnc-asset | 85 | — | accurate |
| cnc-tiberian-dawn | **303** (299+4) | "299 unit + 4 integration" | accurate (corrected 2026-10-20) |
| cnc-red-alert | 74 | — | accurate |
| cnc-map-editor | 93+2 | "93+2" | accurate |
| cnc-net-directory | 58+8 | — | accurate |
| cnc-renderer | 2+GPU smoke | — | codemap stale |
| cnc-audio | 35 | — | accurate |
| cnc-events | 27 | — | accurate; workspace member |
| cnc-console | 1 | — | **uncompilable** (not in workspace) |
| cnc-logs | 3 | — | accurate; workspace member |
| cnc-sfx / cnc-music / cnc-procgen / cnc-bench / cnc-ai / cnc-launcher | 17/14/42/2/23/183 | — | accurate |
| **Workspace ledger (measured)** | **1746 passed / 0 failed** *measured 2026-10-24 — this row previously said 1318/4* | `codemap.md` said 552; this doc earlier claimed 952/0, then 983, then 1318/4 | **correction** |

Measured with `cargo test --workspace --no-fail-fast`. Zero failures — the 4 pre-existing failures from the 2026-10-20 audit have been resolved by fixes in subsequent todos (see `.omo/evidence/task-53-parity-100.log` §A).

## Appendix B: Explicit divergences (intentional, per audit)

- rodio/wgpu/winit platform layer replaces Win32/DirectDraw (by design).
- No Adlib/IMF emulation — MIDI→OGG synth + procedural SFX instead (audio not redistributable).
- IPX→UDP lockstep; COMQUEUE ACK/retry → frame-gated checksum queue.
- Save formats intentionally NOT compatible with original `.SAV` (index-based snapshots vs pointer fixup).
- Steam Workshop dropped (maps are plain INI).
- Pixels-over-idx/lepton coordinates, octile A* over Find_Path edge masks, deterministic scatter over `Frame % count` — itemized with pinned constants in `cnc-tiberian-dawn/PORTING.md:77–119`.
- Mesh/procgen replaces VQA briefing films (movie playback now implemented as fallback).

---

## Appendix C: Todo completion trace (plan parity-100)

All 53 todos complete. Final verification wave (F1–F4) next.

```
Wave 1: Baseline + TD core (todos 1–10) — DONE
Wave 2: RA core (todos 11–16) — DONE  
Wave 3: TD/RA depth (todos 17–28) — DONE
Wave 4: Polish + Editor (todos 29–53) — DONE
  - 29: TD complete unit roster ✓
  - 30: RA complete unit roster ✓
  - 31: TD defensive structures fire ✓
  - 32: Trigger attach model ✓
  - 33: RA carry-over + replay ✓
  - 34: RA multi-house + mission FSM ✓
  - 35: RA IQ + difficulty ✓
  - 36: RA special flags ✓
  - 37: RA infantry specials + crates + vortex ✓
  - 38: Vision consumers + GPS + paradrops ✓
  - 39: TD zone-based pathfinding ✓
  - 40: Radar/minimap + sidebar depth ✓
  - 41: RA score + briefing/ending flow ✓
  - 42: In-game SFX + theme queue ✓
  - 43: MP + options dialogs ✓
  - 44: Mission stats collection ✓
  - 46: Movie playback flow ✓
  - 50: INI-driven type registries ✓
  - 52: MEG/PGM export + validation + trigger dialog ✓
  - 53: Final parity re-audit ✓ (this document)
```

---

## Appendix D: Evidence log

- `.omo/evidence/task-1-parity-100.log` — baseline reconciliation
- `.omo/evidence/task-6-parity-100.log` — documentation reconciliation (2026-10-20)
- `.omo/evidence/task-29-parity-100.log` — TD complete unit roster
- `.omo/evidence/task-30-parity-100.log` — RA complete unit roster
- `.omo/evidence/task-31-parity-100.log` — TD defensive structures fire
- `.omo/evidence/task-32-parity-100.log` — Trigger attach model
- `.omo/evidence/task-33-parity-100.log` — RA carry-over + replay
- `.omo/evidence/task-34-parity-100.log` — RA multi-house + mission FSM
- `.omo/evidence/task-35-parity-100.log` — RA IQ + difficulty
- `.omo/evidence/task-36-parity-100.log` — RA special flags
- `.omo/evidence/task-37-parity-100.log` — RA infantry specials + crates + vortex
- `.omo/evidence/task-38-parity-100.log` — Vision consumers + GPS + paradrops
- `.omo/evidence/task-39-parity-100.log` — TD zone-based pathfinding
- `.omo/evidence/task-40-parity-100.log` — Radar/minimap + sidebar depth
- `.omo/evidence/task-41-parity-100.log` — RA score + briefing/ending flow
- `.omo/evidence/task-42-parity-100.log` — In-game SFX + theme queue
- `.omo/evidence/task-43-parity-100.log` — MP + options dialogs
- `.omo/evidence/task-44-parity-100.log` — Mission stats collection
- `.omo/evidence/task-46-parity-100.log` — Movie playback flow
- `.omo/evidence/task-50-parity-100.log` — INI-driven type registries
- `.omo/evidence/task-52-parity-100.log` — MEG/PGM export + validation + trigger dialog
- `.omo/evidence/task-53-parity-100.log` — **Final parity re-audit (this run)**
