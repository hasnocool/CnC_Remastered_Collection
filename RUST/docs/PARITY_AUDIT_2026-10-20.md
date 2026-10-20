# Functional Parity Audit — Original C++/C# vs RUST workspace

**Date:** 2026-10-20
**Method:** 4 parallel source-level audits (grep/read-verified on both sides, no codemap-only claims). Supersedes `docs/PARITY.md` where they disagree — `PARITY.md` §3 over-claims RA house-AI (the Expert_AI port only drives TD) and RA rules wiring (RA does not read `RulesDatabase` at all).

Scope: `TIBERIANDAWN/` (157 .CPP, ~181k LOC), `REDALERT/` (~519 files), `CnCTDRAMapEditor/` (C# Mobius) vs `RUST/*` (15 member crates, 1253 passing tests).

---

## 1. Headline verdict

| Lane | Overall parity | Notes |
|---|---|---|
| **TD engine** (`cnc-tiberian-dawn`) | **~45–50%** | Weapons/combat tables near-verbatim (~85%); scenario loader end-to-end (~65%), `[TeamTypes]` now consumed by a `TeamManager` (spawn/assign/task/retire), per-house shroud gates acquisition **and is drawn** (unexplored = opaque, explored-but-unseen = 0.55 alpha; unseen units/structures are skipped) |
| **RA engine** (`cnc-red-alert`) | **~62%** measured for the skirmish-shaped subset; **~45% force/content fidelity** (was ~35%) | **Armor-scaled combat** (warhead `Verses` ÷ target armor, `ArmorClass` on units, Rules.ini `Armor=` override), RULES.INI partially consumed (Strength/Speed/Primary/Armor via `unit_profile_with_rules`; **`Cost=` now drives queue pricing and refunds** via `unit_cost_of`/`structure_cost_of`; **`Prerequisite=`, `TechLevel=` and `Owner=` now gate RA builds** via `can_build_class`; the house + difficulty tables remain unused), **original `.MPR`/`.INI` scenarios now field an army** (`[Units]`/`[Infantry]`/`[Structures]` → `Game::from_original_scenario`), ~1/6 trigger vocabulary |
| **Map editor** (`cnc-map-editor`) | **~30–40%** | Undo/redo 95%+ (97 tests vs 0 in C#); RA `MapPack` import exists in the model, but there is no import UI and nothing for TD `ini+bin` |
| **Save/load** | Round-trips units, structures, attack targets and the `[TeamTypes]` roster | `team_manager` was silently dropped on save until this round |
| **Support crates** | Mixed | Sim/netcore solid; VQ and in-game SFX still partial; EVA vocabulary + queue landed |

**Verified build state:** **1253 passed / 3 failed**, measured by summing `cargo test --workspace --no-fail-fast` (2026-10-20). The 3 failures are the pre-existing `cnc-map-editor` ones (`layout_controls_stay_inside_the_window`, `trigger_filter_matches_group_and_name`, `trigger_inspector_controls_and_frame_render`), which reproduce with every pending change reverted. Any earlier figure — 952 (ledger-derived, not reproducible), 983, or `RUST/codemap.md`'s 552 — is stale.

---

## 2. What is genuinely solid (evidence-verified)

- **Determinism core:** `Fixed16_16` faithful to RA 16.16 (16 tests); lockstep determinism proven by two-identical-games test (`game.rs`); command-hash desync detection (`network.rs:86-92`).
- **TD combat data:** all 25 weapon rows, 12 warheads, bullet classes verbatim vs `CONST.CPP`/`BBDATA.CPP` (`td_rules.rs`, pinned tests). RA lacks this entirely (flat `damage` subtraction, `game.rs:1852–2046`).
- **TD economy + harvesting:** tiberium-first spending, storage overflow, power fraction, 25-credit harvesting loop — all pinned tests.
- **Carrier of concepts done right:** CARRY.CPP port (TD, checksummed); engineer capture / commando C4 (FULL with documented approximations); TD crates/powerups (FULL).
- **Netcode:** `CommandQueue` → `LockstepSession` → UDP transport, handshake, host migration, mesh relay, replay records (36 TD tests + live-session integration tests). Modern replacement for COMQUEUE/IPX.
- **cnc-net-directory:** gossip LWW registry, MMR/LFG matchmaking, LAN multicast, BitTorrent fallback — exceeds original IPX footprint (58 lib + 8 integration tests). BUT not wired to launcher (absent from `cnc-launcher/Cargo.toml`).
- **Map editor structure:** history-enforced mutation (structurally stronger than the C# rule), 12 command types, dependency-sound crate contract, 93 tests. Rust-only additions: procedural generator, Agent Studio, headless CLI.
- **RA skirmish loop layout:** base placement, power cascade, ore mining, Tesla defense (wired and tested), chrono/curtain superweapons, MCV deploy, 10 authored scenarios that boot.

---

## 3. Top gaps across all lanes (ranked)

1. **Original scenario/map I/O — TD full pipeline; RA file + editor import landed; RA object placement missing.** TD: `cnc-tiberian-dawn/src/scenario_loader.rs` parses original `[Basic]`/`[Objectives]`/`[Wunits]`/`[Structures]`/`[Waypoints]`, decodes `.bin` cells (raw or LCW), instantiates houses/units/buildings/terrain, and converts trigger records into executable `Trigger`s (23 tests). RA: `cnc-asset::{base64,ra_pack}` implement the numbered-line base64 container and 3-byte packed cells; `cnc-red-alert/src/original_scenario.rs` consumes them for `[Basic]`/`MapPack`/`OverlayPack`/`[CellTriggers]` (8 tests); and the **map editor** now imports original RA files via `MapModel::from_original_ra` (4 tests). Still missing: RA `[Houses]`/`[Base]` object placement and scenario→`Game` wiring (RA scenarios load terrain but field no army), TD `[Houses]`/`[CellTriggers]`/waypoint coords, most TD event/action vocabulary, TD `ini+bin` import in the editor, and **any original-format fixture in-tree** (all 19 shipped maps are the new Rust dialect).
2. **TeamType is now consumed by both engines.** `cnc_tiberian_dawn::ai::TeamManager` (8 tests) and `cnc_red_alert::game::TeamManager` (6 tests) both parse `[TaskForces]`/`[TeamTypes]` via `cnc_core::entity`, spawn an instance once a taskforce is available, claim units exclusively, issue the objective, and retire teams whose members died. The RA consumer is driven from `Game::update_team_manager`. Still thin: RA's `AiCommander` cadence remains the fallback, TD's planner methods are empty stubs (`ai.rs:133–147`); `defense_groups/attack_groups` never populated. The Expert_AI port (`cnc-tiberian-dawn/src/ai.rs:311`) never sees the RA game despite `docs/PARITY.md` claiming otherwise.
3. **Both engines model, gate and draw a shroud.** `cnc-core::shroud` implements the per-house model with the original's **diamond** reveal shape (`MAP.CPP` `RadiusOffset`: `dist = larger/2 + smaller`, so radius 1 is a full 3x3 while larger radii omit corners; radii above 10 are ignored as `Sight_From` does); TD `Game` gates unit auto-acquire and defensive fire on it and now **renders it** — `Game::render_shroud` covers every cell not currently visible, fully black when unexplored and `SHROUD_MEMORY_ALPHA` when only explored, and `render_with_sprites` skips units and structures on cells the player cannot see (4 tests). **`EVENT_DISCOVERED` is now wired**: `refresh_shroud` snapshots the mapped history and records cells that become mapped, `Condition::Discovered { cell }` matches them, and discoveries are drained after each trigger evaluation so one discovery fires once rather than every tick (5 tests; the explored set is persisted in saves). **RA now has a shroud too**: `Game` gained a per-house `Shroud`, `enable_shroud`/`refresh_shroud` compute sight from units and structures, and `render` hides unseen objects and draws the same overlay TD does (5 tests). It defaults to fully revealed, so existing skirmishes are unchanged until fog is enabled. **`EVENT_DISCOVERED` is wired in RA too**: `discovered:` (any) and `discovered:x,y` (specific cell) conditions read the same discovery set, drained after `update_triggers` (4 tests). **RA trigger actions were win/lose/message only and now include scripted `airstrike`/`nuke`/`ion`**, and RA announces `MissionAccomplished`/`MissionFailed` on those actions and `NuclearWeaponLaunched` on a scripted nuke (3 tests). **`[CellTriggers]` are now consumed**: `RunningTrigger` gained a `cell` binding resolved by trigger name from `OriginalScenario::cell_triggers` (the `DISPLAY.CPP` `Read_INI` mapping), and a bound trigger fires when a *player* unit stands on that cell, ignoring enemy units (3 tests). The binding round-trips through the text save format, and **both** scenario readers now carry `cell_triggers` (`Scenario` gained the field), with the end-to-end name→trigger bind covered by a test plus an unmatched-name case (6 tests). **Sight is now rules-driven**: both engines read `Sight=` via `unit_sight_range`/`structure_sight_range`, clamped to `MAX_SIGHT_RADIUS`, instead of hard-coded stats or role approximations (4 tests).
4. **RA combat is flat damage subtraction** — no verses-armor matrix (`rules.rs:345` exists, unconsumed by RA), no projectiles/flight-time, no miss rolls, no splash. TD has the full damage math; RA is a numeric stub. RA also **reads zero rules data** (stats hard-coded in `game.rs:2126–2240`; launcher build times hard-coded).
5. **Defensive buildings cannot fire (TD).** `building.rs` has no weapon; `update_attacks` iterates units only — towers/Obelisk absent from roster anyway.
6. **RA aircraft now re-arm; transports/passengers still missing in both engines.** RA's `GameUnit` gained `max_ammo` and `Game` gained `aircraft_rearmed`; `Game::update_rearm` returns dry aircraft to an owned `Airfield`/`Helipad` and reloads one round per `REARM_INTERVAL`, driven from `Game::update` (6 tests). Absent: `CARGO.CPP` passenger/cargo state, APC/Chinook transports, paratroopers. `naval.rs`'s submarine rules are now **consumed rather than duplicated**: `GameUnit` gained `detected`/`torpedoes` plus `is_targetable()`, so a submerged, undetected submarine is excluded from `enemies_of` (previously `submerged` was a purely cosmetic flag and hidden boats were fully targetable), cannot fire, spends a torpedo per shot, and rearms at a naval yard (5 tests). **`aircraft.rs` duplication retired**: the module is now only the shared constants (`REARM_SECONDS`/`DEFAULT_AMMO`/`AIRFIELD_CAPACITY`) plus tests asserting `Game` still uses them; the parallel `Aircraft`/`Airfield`/`Helipad` structs are gone. That cross-check caught a real drift — the module said 2 rounds while `Game` used 6. The **pad-capacity rule is consumed**: `Game::update_rearm` tracks occupancy across a whole pass, so a `Helipad` parks exactly one aircraft and an `Airfield` up to `AIRFIELD_CAPACITY` — previously every aircraft rearmed simultaneously regardless of pad occupancy (test added). The standalone `Aircraft`/`Airfield`/`Helipad` structs remain as the documented home of those rules with their own tests.
7. **Trigger vocabulary thin, but the superweapon actions now exist.** TD implements `ACTION_AIRSTRIKE` (9), `ACTION_NUKE` (10), `ACTION_ION` (11), `ACTION_DZ` (8), `ACTION_ALL_HUNT` (6) and `ACTION_CREATE_TEAM` (4) as executable `Action` variants that deal real damage or spawn real units, mapped from the original `ActionText` names (7 mapping + 6 runtime tests). Conditions are ~6 of ~17 `TEVENT` (now including `EVENT_DISCOVERED`), and the remaining `TACTION` values are still reported unmapped rather than invented.
8. **EVA vocabulary + queue now exist; radar/minimap in-game logic absent.** `cnc_audio::eva` carries the full `Speech[]` table from `TIBERIANDAWN/AUDIO.CPP` (45 lines, e.g. `ACCOM1`, `BASEATK1`, `NUKLNCH1`) plus `EvaQueue`, which reproduces `Speak()`/`Speak_AI()`'s one-slot pending semantics exactly (a second request while one is pending is dropped, not queued) — 9 tests. TD `Game` owns an `eva: EvaQueue` and posts `MissionAccomplished`/`MissionFailed` from the Win/Lose actions (2 tests). TD now emits 12 distinct lines from real gameplay: `BaseUnderAttack` (any damaged player structure, 35 s cooldown per `HOUSE.CPP`'s remaster timing), `InsufficientFunds` (< 100 credits with a factory present, 60 s cooldown), `IonCannonReady`/`NuclearWeaponAvailable` (edge-triggered on the charging→ready transition), `MissionAccomplished`/`MissionFailed` (Win/Lose actions), `ConstructionComplete`/`UnitReady` (production completion for the player), `LowPower` (edge-triggered on the transition into deficit), and `UnitLost`/`EnemyUnitDestroyed`/`StructureLost`/`EnemyStructureDestroyed` (death announcements, sampled per frame so no damage path is missed, announced once per corpse). Note: `Game::new` leaves `houses` empty — only `setup_demo` populates them — so any test touching credits, power or EVA must seed houses first; three EVA tests were silently passing without asserting and now build their own fixtures. RA now has its own vocabulary (`cnc_audio::eva_ra::RaEvaLine`, 45 entries transcribed from `REDALERT/AUDIO.CPP` `Speech[]`) and an `eva: EvaQueue` on `Game`; it emits `UnitReady`/`ConstructionComplete` on production, `UnitLost`/`StructureLost` on player losses, and `LowPower` edge-triggered (5 tests). The two games use **different recordings for the same line** (TD `UNITREDY` vs RA `UNITRDY1`), pinned by a test. TD also announces `InsufficientFunds`/`UnableToBuildMore` when `queue_unit`/`queue_building` refuse, and RA announces `Canceled` on a cancelled production (3 tests). Absent: no sample is decoded or played in either engine, because the Remaster intercepts `Speak()` at `On_Speech`. radar/minimap in-game logic absent; sidebar is a thin option list; credits tick animation missing (instant money); RA score screen wired TD-only (`main.rs:2612`).
9. **Wire/sim command-surface mismatch (TD):** 11 of 15 `Command` variants have codecs but no `Game::apply_command` arm (`apply_sabotage` is sole consumer); UI drives `Game::order_*` directly, so those wire commands are never lockstep-executed. No game-state CRC (command-hash only). Net-directory + ready-up protocol built but not hooked into launcher poll loops.
10. **RA specials absent:** crates, chronal vortex, nuke missile launch, GPS (struct tested but never pushed into `Game::superweapons`), engineer capture (RA), dogs, medics, Tanya, MAD tank. France's unique building silently maps to Germany's Chronosphere (`house.rs:67–72`) — Grand Cannon doesn't exist in `ClassId`. **Known fidelity bug.**

---

## 4. Lane detail — TD engine (~40–45%)

| Category | % | Biggest gaps |
|---|---|---|
| Weapons/warheads/bullets/damage | ~85–95% | destroy-wall/wood warhead flags unused |
| Economy/superweapons | ~60–65% | credit tick, FloatBias handicap (ported in cnc-core, zero TD consumers), alliances |
| Units/infantry/aircraft | ~50% | ~⅔ of roster missing (APC, Artillery, MLRS, Stealth/Flame/SSM, MHQ, MCV deploy, Chinook, Grenadier, dinos); sub-cell infantry |
| Pathfinding | ~60% | no edge masks/zones, `is_passable` flag only |
| Map/tiberium | ~65% | shroud primitive + TD per-house visibility wired (`cnc-core::shroud`, `Game::refresh_shroud`); no zone info, overlays logic-less, renderer not drawing it yet |
| Triggers/missions | ~30% | custom dialect; original map data unparseable |
| Scenario loading | **~65%** | `scenario_loader` parses original `[Basic]`/`[Objectives]`/`[Wunits]`/`[Structures]`/`[Triggers]`/`[Waypoints]`, decodes `.bin` cells (raw or LCW), and `scenario_to_game` instantiates houses/units/buildings + terrain. Original trigger records decode via the real `EventText`/`ActionText` tables and convert into executable `Trigger`s, with unmapped vocabulary reported (23 tests). Missing: `[Houses]`, `[CellTriggers]`, waypoint coords, RA originals, original-format fixture in-tree |
| Team AI + IQ | ~45% | Both engines consume `[TeamTypes]` (attack/guard/move/patrol/retreat); `Unload`/`EnterObject` unimplemented; IQ/difficulty tables unused |
| Save/load | ~45% | native `CNCSAVE\0` bincode; original `.SAV` incompatible (deliberate, documented); `cnc-core::pointers` unused by TD game state |
| Lockstep net | ~70% | transport/session ~90%, event vocabulary + apply arms ~40% |
| UI/EVA/radar | ~10% | option list only |

194 tests (190 inline + 4 integration). Codemap's "154 unit + 4 integration" claim stale.

## 5. Lane detail — RA engine (~55% subset / ~35% content)

| Category | % | Notes |
|---|---|---|
| Core determinism loop | ~90% | no seeded RNG by design (original uses rolls for misses/dog kills) |
| Entities | ~40% | no facing/turret rotation (32-angle FACING missing), no mission state machine |
| Combat | ~30% | see §3.4 |
| Economy/power | ~70% | single FIFO queue (no per-factory), no prereq gating, no silo capacity |
| Rules pipeline | ~80% | engine built; **RA consumes Strength/Speed/Primary/Armor and the warhead `Verses` matrix** via `rules_type_name` + `unit_profile_with_rules` + `damage_vs_armor` (10 tests). Fixed a real parser bug on the way: `Verses=..50%..` percent signs were rejected, silently shifting armor columns. Remaining unused: house roster and difficulty tables |
| Scenario/triggers | ~40% | 10 authored maps; ~1/6 event/action vocabulary |
| Pathfinding | ~60% | no amphibious class, no threat cost, path-node cap |
| AI | ~25% | one commander per game (hard-coded owner 1) — no 2v2 |
| Superweapons/specials | ~45% | chrono+curtain+Tesla+gap solid; GPS unwired; vortex/nuke/crates/paradrops missing |
| Save/replay | ~50% | text save round-trip ok; TD has binary snapshot, RA doesn't; RA replay recording absent |
| Networking | ~65% | LiveSession works; net-directory unwired; ready-up hooks not connected |
| UI/audio | ~50% | HUD/sidebar/minimap work; no MP dialogs/options; no EVA |
| Map editor integration | exceeds original | only crate consuming `cnc-ai` authoring |

74 tests. Scale reality: RA game logic ~1.5–2 MB C++ vs 2,646 LOC Rust — an archetype port.

## 6. Lane detail — Map editor (~30–40%)

FULL: undo/redo + mutation discipline (~95%, superset incl. grouped strokes, 50-cap ≡ C# default, history inspector).
Key PARTIALs: minimap/nav (55%), renderer layers (45%, missing smudge/team colors/theater palettes), cell grid (40%, no multi-cell footprints, no 5-slot infantry groups, no occupier/overlap sets), triggers (35%, simplified strings, no TEVENT/TACTION enums, no cell binding), map root sections (35%, no per-house/BasicSection/Steam), validation (35% structural only).
PARTIAL: **original-format map I/O** — RA `.MPR`/`.INI` (`MapPack`/`OverlayPack` + object placement + `[CellTriggers]`) loads and fields an army; TD `ini+bin` loads via `scenario_loader` including `[Waypoints]` (numeric id → cell, as `DISPLAY.CPP` reads it) and `[Houses]` (`Credits`×100, `MaxUnit`, `MaxBuilding`, `Edge` per `HOUSE.CPP`), applied to house creation with **`MaxUnit`/`MaxBuilding` now enforced** in `can_queue_unit`/`can_queue_building` (`within_unit_limit`/`within_building_limit` count live plus queued), and `Edge=` is parsed into `ReinforcementEdge` (North/East/South/West/Air/Visible/EnemyBase/HomeBase) and **consumed** by `spawn_position` via `edge_entry_cell`, so a house with no factory receives reinforcements on its configured side (compass edges move the spawn; Air/Visible/base-relative modes still fall back to the centre); **the editor model can now import both** — `MapModel::from_original_ra` (MapPack/OverlayPack) and the new `MapModel::from_original_td(ini, bin)` (raw or LCW `.bin`, 3 tests) — and **`open()` now dispatches by dialect** via `MapModel::import_text` (original RA → `from_original_ra`, else editor dialect), so a real `.mpr` opens through the same command as a native file and reports which reader ran (3 tests). **TD opens the same way**: `open()` looks up the sibling `<stem>.bin` and `import_text_with` routes `[Basic]`+`[Wunits]`/`[Structures]` text to the TD reader, failing with a message that names the missing `.bin` rather than a confusing dialect error (3 tests). **TD import now reads objects and `[Trigs]`** as well (`[Wunits]`/`[Structures]` and the `[Trigs]` list, joining `ConditionN`/`ActionN` slots into the editor trigger list; 3 tests). **RA import now reads `[Units]`/`[Infantry]`/`[Structures]`** into the same object list (unknown types kept as `Unknown`, off-map cells dropped; 3 tests). The TD `TIcon` byte is also imported now (`MapModel::icon`, one sub-tile index per cell, carried raw from the `.bin`; 2 tests). `[Houses]` is imported by **both** importers through one `parse_house_entries` (`MapModel::houses`: Credits×100, MaxUnit, MaxBuilding, Edge, plus RA's TechLevel/MaxInfantry/MaxVessel). TD has **no `[Overlay]` INI section** — overlays live in the cell image and the object system — so there is nothing further to import on that axis. **Byte-exact fixtures now exist** (`cnc-map-editor/src/fixtures.rs`): an RA scenario using the real numbered-base64 `Get_UUBlock` container with 256 map cells, an overlay block, objects/houses/cell-triggers, and a TD `.bin` whose `TIcon` bytes are non-zero so a reader that drops them fails. Both importers are exercised end-to-end against them (7 tests), which retires the previous "all evidence is synthetic" caveat. **LCW is now covered too**: `cnc_asset::format80::compress` was added (RLE + literal + absolute-copy opcodes, round-trip tested against the existing decoder), so a compressed `.bin` fixture exists and decodes identically to the raw one. Caveat: the encoder emits valid streams for the *decoder's* opcode set, not a byte-identical reproduction of Westwood's `LCWCOMP` output. MISSING entirely: `.MEG`/BIN/PGM, WWCompression, **waypoints**, **TeamTypes tab**, smudge layer, template-ribbon tool, walls/resources brushes, INI-driven type registries (catalogs are 14–21 hardcoded ClassIds vs hundreds of types), Steam Workshop (**intentional divergence — should be documented in crate README**).
Rust-only additions: procedural generator (deterministic seeds), Agent Studio, headless PNG/validate CLI, elevation grid.
93 tests vs **0** in original C# editor.

## 7. Lane detail — Support crates

**Solid:** cnc-core (120 tests, deepest-tested), cnc-procgen, cnc-net-directory, cnc-bench, cnc-renderer core (wgpu + software paths, palette-remap shader — needs icon-cache/theater-blit equivalents; codemap stale re stubs).
**Partial:** cnc-asset (MIX/SHP/Format80/VOC/AUD/PAL done; **VQ video and VXL voxels unsupported** — `video.rs:182`; CSF is a 17-line stub), cnc-audio (mixer/priority/fade semantics good; **no speech/EVA VOX anywhere, no ThemeClass song queue, in-game SFX unwired — menu-only mixer `cnc-launcher/src/main.rs:8224`**), cnc-sfx + cnc-music (real generators, zero runtime consumers; recommended if Adlib/IMF emulation is out of scope).
**Orphaned/missing:** cnc-events (27 tests, not a workspace member, unconsumed), cnc-logs (same), **cnc-console uncompilable** (`lib.rs` declares `styles.rs`/`widgets.rs` which don't exist — compile break), cnc-replayer + cnc-viewer skeleton dirs (replay feature exists scattered in TD netcode/launcher instead), cnc-classic version-string stub.

## 8. Momentum-killer fixes (cheap, do first)

1. **France → Grand Cannon duplicate bug** (`cnc-red-alert/src/house.rs:67–72`).
2. **Fix stale test counts:** `RUST/codemap.md` + `scripts/sync-docs.sh` "552 tests" → ledger-recomputed 952; TD codemap "154 unit" → 190; renderer codemap stub claims.
3. **cnc-console missing files** break any workspace-wide build once membership is added; also cnc-events/cnc-logs declare `version = { workspace = true }` without membership.
4. **`cargo fmt` failure** at `cnc-tiberian-dawn/src/powerup.rs:209` (unresolved per BUILD_STATUS ledger at last run) — violates AGENTS SemVer/CI policy.
5. Document Steam divergence in `cnc-map-editor/README.md`.
6. Wire-or-delete — **partly retracted and partly resolved**. Verified 2026-10-20: RA `naval.rs` submarine rules are wired into `Game` (stealth targeting, torpedoes, naval-yard rearm); `aircraft.rs`'s pad-capacity rule is wired into `Game::update_rearm` (helipad = 1, airfield = `AIRFIELD_CAPACITY`), with the standalone structs retained as the rule's home, and `FloatBias` is consumed by `cnc-core::math` vector types plus `serialization`. **`GPS` is now wired**: `Game` carries a `gps: Vec<GPS>` field with `fire_gps`/`update_gps`, covered by 5 tests. **`naval.rs` duplication retired the same way**: the module now holds only `SHIP_SPEED_PX_PER_SEC`/`SUB_SPEED_PX_PER_SEC`/`SUB_TORPEDOES` plus tests asserting `Game` matches them and that stealth/torpedo/rearm behave; the parallel `Ship`/`Submarine` structs are gone. **`cnc-core::pointers` resolved by decision, not by wiring**: `CodePointers`/`DecodePointers` still have no engine consumer, and deliberately so — both engines address objects by index (`u32` into `units`/`buildings`), which survives save/load without pointer fixup, confirmed by a round-trip test that preserves attack targets. The module is retained as the `ObjectId` home (used by `Team::members`). One real defect was fixed while auditing it: `DecodePointers::decode_const_ref` was a **safe** function returning `&T` from a raw pointer with no lifetime guarantee — a memory-safety hole reachable without `unsafe`. It is now `unsafe` with a full `# Safety` contract, matching `decode_ref`.

## 9. Highest-leverage roadmap (gameplay-per-LOC)

1. **Original scenario/map loaders** (LCW + `[Basic]/[Houses]/[Wunits]/[Units]/[STRUCTURES]/[TERRAIN]/[OVERLAY]/[Trigs]/[Waypoints]`) — unlocks the entire original campaign for TD, RA, and the editor at once; single most ROI-positive item.
2. **DEFENSIVE BUILDINGS (TD) + RA verses matrix — both done.** RA: `ArmorClass`, `damage_vs_armor`, warhead `Verses` scaling. TD: `StructureType` gained `GuardTower`/`AdvancedGuardTower`/`Turret`/`SamSite`/`Obelisk` with BDATA stats/weapons and loader names (`ATWR`/`GTOWER`/`TURRET`/`SAM`/`OBLI`), and `Game::update_defence_combat` now acquires and fires at enemy units — honouring range, power, reload and SAM anti-air-only restriction (9 tests). Known limitation: building fire is direct-damage rather than projectile-based.
3. **RA consumes `RulesDatabase`** — *largely done*: per-unit `Strength`/`Speed`/`Primary`/`Armor` and the **verses-armor matrix** are live (`unit_profile_with_rules`, `damage_vs_armor`, `ArmorClass`). Remaining: `structure_cost`, house colour/roster tables, difficulty multipliers.
4. **Shroud layer — primitive + TD consumption done; rendering and RA remain.** `cnc-core::shroud::Shroud` implements the original per-cell model (`CELL.H` `IsVisible`/`IsMapped`, per-house 64-bit bitsets, `reveal`/`clear_visible`/`can_target`, 10 tests). TD `Game` carries a `shroud`, with `enable_shroud`/`refresh_shroud` (sight from unit+building `sight_range`) and `can_see`. Both TD acquisition paths now consult it: `update_auto_acquire` skips units whose target cell is not visible, and `update_defence_combat` gates fire on a per-house visibility snapshot. 4 gate tests cover unseen ≠ acquired, acquire-after-reveal, default-game (fully revealed) acquisition, and defences holding fire. Still unwired: the renderer draws no fog, RA has no shroud, and no `EVENT_DISCOVERED` emitter.
5. **TeamType engine wiring** — done in both engines; remaining work is the unused taskforce knobs (RecruitPriority/MaxAllowed/InitNum/Fear) and `Unload`/`EnterObject`.
6. **TD `Game::apply_command` arms** for the 11 unhandled wire commands + game-state CRC in FRAMESYNC equivalents.
7. **RA re-arm docks** — done (`Game::update_rearm`); **transports** (cargo/passenger fields on GameUnit, APC/Chinook) still absent.
8. **Trigger actions: `AIRSTRIKE/NUKE/ION/DZ`** — done in TD (`Action::Airstrike/Nuke/IonCannon/Dropzone`); RA's trigger actions are `win`/`lose`/`message:` plus scripted `airstrike`/`nuke`/`ion`, which now deal **real area damage** through `Game::apply_radius_damage` (200/333/600, armour-scaled). A trailing cell aims them (`nuke:30,30`), otherwise they land on the player base (4 tests). RA also gained `dz`/`dz:x,y` (4 infantry) and `team:Name` (`ACTION_CREATE_TEAM`), plus a visible tracer on every scripted strike (3 tests). Remaining: RA scripted strikes use TD's damage values rather than `RULES.INI`.
9. **EVA/VOX pipeline** — vocabulary + queue done in `cnc-audio`; **`Speak()` call sites largely absent** (2 of 45 emitted, TD only) and RA has no EVA.
10. **Formatter/codemap/doc hygiene** — the cheap compounding fixes.

---

## Appendix A: Test counts by crate (verified 2026-10-20)

| Crate | Tests | Claim in own codemap | Verdict |
|---|---|---|---|
| cnc-core | 120 | — | accurate |
| cnc-asset | 85 | — | accurate |
| cnc-tiberian-dawn | 194 | "154 unit + 4 integration" | stale |
| cnc-red-alert | 74 | — | accurate |
| cnc-map-editor | 93+2 | "91+2" | close enough |
| cnc-net-directory | 58+8 | — | accurate |
| cnc-renderer | 2+GPU smoke | — | codemap stale |
| cnc-audio | 35 | — | accurate |
| cnc-events | 27 | — | accurate, orphaned |
| cnc-console | 1 | — | **uncompilable** |
| cnc-logs | 3 | — | accurate, orphaned |
| cnc-sfx / cnc-music / cnc-procgen / cnc-bench / cnc-ai / cnc-launcher | 17/14/35+7/2/23/125+58 | — | accurate |
| **Workspace ledger (measured)** | **1253 passed / 3 failed** | `codemap.md` said 552; this doc earlier claimed 952/0, then 983 | **correction** |

Measured with `cargo test --workspace --no-fail-fast`. The 3 failures (`cnc-map-editor`: `layout_controls_stay_inside_the_window`, `trigger_filter_matches_group_and_name`, `trigger_inspector_controls_and_frame_render`) were verified to reproduce with all pending working-tree changes reverted, so they are pre-existing. The 952 figure aggregated `BUILD_STATUS.md` entries that are not reproducible via `cargo test --workspace`.

## Appendix B: Explicit divergences (intentional, per audit)

- rodio/wgpu/winit platform layer replaces Win32/DirectDraw (by design).
- No Adlib/IMF emulation — MIDI→OGG synth + procedural SFX instead (audio not redistributable).
- IPX→UDP lockstep; COMQUEUE ACK/retry → frame-gated checksum queue.
- Save formats intentionally NOT compatible with original `.SAV` (index-based snapshots vs pointer fixup).
- Steam Workshop dropped (maps are plain INI).
- Pixels-over-idx/lepton coordinates, octile A* over Find_Path edge masks, deterministic scatter over `Frame % count` — itemized with pinned constants in `cnc-tiberian-dawn/PORTING.md:77–119`.
- Mesh/procgen replaces VQA briefing films.
