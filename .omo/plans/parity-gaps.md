# parity-gaps - Work Plan

## TL;DR (For humans)
**What you'll get:** The ten remaining parity gaps from the final re-audit closed: original maps' per-house setup and cell triggers actually drive the game, computer opponents plan builds and attacks like the originals, Red Alert combat gets real projectiles with travel time, misses and splash, defensive towers fire shells instead of instantly damaging, ships fight like ships, the full original trigger vocabulary is understood, voice warnings fire during play (both games), network commands do what the wire says, and the special units (dogs, medics, Tanya, MAD tank, engineers) actually behave.
**Why this approach:** Ordered by dependency — the broken replay crate is fixed first (it blocks workspace-wide gates), then map-data consumption (it unlocks testing AI/triggers against real content), then combat depth, then AI/triggers/EVA in parallel, with specials and wire commands as parallel slices.
**What it will NOT do:** Add new features beyond the ten audited gap rows; no GLA/Adlib emulation, no original .SAV compatibility (recorded intentional divergences).

## Scope
- In: gaps 1–10 from PARITY_AUDIT_2026-10-24 Appendix "Open Gaps".
- Out: intentional divergences (Appendix B of the audit), editor renderer layers, net-directory launcher hookup.

## Todos

- [x] 1. Fix cnc-replayer and rejoin the workspace
  What to do / Must NOT do: Fix the 9 compile errors in `RUST/cnc-replayer/src/lib.rs` (test-module encode/decode: `RaGameCommand::Produce` carries `seconds: i32`; `Fixed16_16` serializes via `to_bits()`/`from_bits()` not `to_le_bytes`; RA `Move.target` is `Vec2` fixed-point, TD `Command::Move.target` is `Vec2` not `Vec2i`; `game.sim_seed` is a method not a field; TD `Game` exposes its RNG via a public accessor — add one if absent, never via field; `Game::apply_command` takes `(owner, &Command)`). Remove `#[workspace]` from its Cargo.toml, delete the `exclude` entry in `RUST/Cargo.toml`, rejoin `[workspace] members`. Restore path/workspace deps (cnc-core etc. via `{ workspace = true }` where the root defines them). Keep the crate out of the pre-commit skip list.
  Must NOT do: Do not change replay format v3 or the `CNCR`/`CNRA` magics; do not weaken the two-identical-gates CRC contract; do not fake TD RNG access with a hardcoded seed.
  Parallelization: Wave 1 (alone) | Blocked by: none | Blocks: 13 | Parallel with: none
  References: RUST/cnc-replayer/src/lib.rs:534-656 (error sites); RUST/cnc-red-alert/src/game.rs:765-810 (GameCommand shape); RUST/cnc-tiberian-dawn/src/network.rs:12-90 (Command/NetCommand); RUST/cnc-core/src/fixed.rs:55 (to_bits); RUST/Cargo.toml (exclude array)
  Acceptance criteria (agent-executable): `cargo metadata --no-deps` lists cnc-replayer; `cargo test -p cnc-replayer` green; workspace `cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` green with the crate included.
  QA scenarios: happy - a TD and an RA replay record→encode→decode→replay round-trip reproduces the same state CRC; failure - corrupt replay rejected by checksum with typed error, Evidence .omo/evidence/gap1-replayer.log
  Commit: Y | fix(replayer): rejoin workspace, align command codecs

- [x] 2. TD `[CellTriggers]` end-to-end
  What to do / Must NOT do: Parse `[CellTriggers]` (`cell=TriggerName` lines, CELL.CPP read order) in scenario_loader.rs; attach the trigger to the cell; fire when a friendly unit enters/occupies the cell (visited-by check per `CELL.H IsTrigger` + `TRIGGER.CPP Spring`). Unknown trigger names warn + skip. Must NOT invent trigger semantics beyond enter/discover firing.
  Parallelization: Wave 2 | Blocked by: none | Blocks: 9 | Parallel with: 3,4
  References: RUST/cnc-tiberian-dawn/src/scenario_loader.rs (parse style, :771 parse_houses as pattern); original/TIBERIANDAWN/CELL.CPP; original/TIBERIANDAWN/TRIGGER.CPP:67 (EventText); RUST/cnc-tiberian-dawn/src/game.rs trigger_apply_tests module
  Acceptance criteria (agent-executable): `celltrigger_fires_when_unit_enters` green: scenario with a `[CellTriggers]` entry spawns the trigger and an entering unit fires it once (idempotent).
  QA scenarios: happy - unit enters bound cell → trigger action executes once; failure - malformed cell number or unknown trigger name → warning + skip, no panic, Evidence .omo/evidence/gap2-celltriggers.log
  Commit: Y | feat(td): cell triggers

- [x] 3. RA `[Houses]` + `[Base]` consumption in `from_original_scenario`
  What to do / Must NOT do: Extend `Game::from_original_scenario` (game.rs:1195) to consume what the parser already produces: per-house `Credits`/`Edge`/`MaxUnit`/`MaxBuilding` from `OriginalScenario::houses` (original_scenario.rs:54), and `[Base]` build-order nodes (original_scenario.rs:96, parse at :583) placed as structures for `base_player`. Unknown base node type names warn + skip.
  Must NOT do: Do not re-parse INI in game.rs — the scenario struct is the contract; do not place base nodes on occupied cells silently (nearest free cell or skip with warning).
  Parallelization: Wave 2 | Blocked by: none | Blocks: 4 | Parallel with: 2,4
  References: RUST/cnc-red-alert/src/original_scenario.rs:54-110, :583; RUST/cnc-red-alert/src/game.rs:1195-1260; RUST/cnc-tiberian-dawn/src/scenario_loader.rs:1011 ([Houses]-wins pattern)
  Acceptance criteria (agent-executable): `ra_scenario_base_and_credits_applied` green: fixture MPR with `[Houses]` credits and `[Base]` nodes yields houses with those credits and structures at those cells.
  QA scenarios: happy - base nodes spawn POWR/BARR for the named player; failure - base node naming an unknown type or occupied cell → warning + skip, game still builds, Evidence .omo/evidence/gap3-ra-houses-base.log
  Commit: Y | feat(ra): consume houses and base sections

- [x] 4. Editor import UI for original maps
  What to do / Must NOT do: Add File→Import Original Map for both TD (`ini+bin` via scenario_loader types already in the model) and RA (`.MPR`/`.INI` via `MapModel::from_original_ra`), wired through a tool command (undoable where it mutates the model). Show parse warnings in the validation panel. Must NOT mutate the model outside `UndoRedoList` commands; must NOT restyle existing dialogs.
  Parallelization: Wave 2 | Blocked by: 3 (RA model completeness) | Blocks: none | Parallel with: 2
  References: RUST/cnc-map-editor/src/original.rs (from_original_ra, from_meg); RUST/cnc-map-editor/src/history.rs (command pattern); RUST/cnc-map-editor/src/app.rs (menu wiring); original/CnCTDRAMapEditor/Dialogs/ pattern
  Acceptance criteria (agent-executable): `editor_imports_original_maps_via_command` green: importing a fixture TD ini+bin and an RA MPR produces identical model state to the existing direct loaders, and Ctrl+Z reverts to the pre-import state in one step.
  QA scenarios: happy - import fixture → map cells/objects visible, minimap updates; failure - truncated .bin or missing MapPack section → error dialog listing the parse warning, model untouched, Evidence .omo/evidence/gap4-editor-import.log
  Commit: Y | feat(editor): original-map import UI

- [x] 5. RA projectiles: flight-time, miss rolls, splash
  What to do / Must NOT do: Replace instant `apply_radius_damage` weapon hits with a `Projectile` list on `Game`: spawn on fire, travel at speed toward target with facing, hit on arrival (or fuse expiry for fueled), roll to miss vs moving targets using the seeded RNG (`rng.rs` — deterministic, part of CRC), splash via existing `apply_radius_damage` on impact. Keep superweapon strikes and scripted `airstrike/nuke/ion` as-is (they are area effects, not weapon shots). Must NOT change damage_vs_armor math or warhead Verses; must NOT make RNG non-deterministic or bypass the command-hash/state-CRC replay gates.
  Parallelization: Wave 3 | Blocked by: none | Blocks: 13 | Parallel with: 6,7
  References: RUST/cnc-red-alert/src/game.rs:3393 (apply_radius_damage), :514 (damage_vs_armor), rng.rs; RUST/cnc-tiberian-dawn/src/weapon.rs (projectile archetype to mirror: fueled, arcing, retarget); original/REDALERT/COMBAT.CPP (miss/splash cadence)
  Acceptance criteria (agent-executable): `ra_projectile_travels_and_misses` green: shell fired at a distant moving target sometimes misses (seeded), same seed → same outcome; `ra_splash_hits_adjacent` green. Two-identical-games determinism test still passes with projectiles in flight.
  QA scenarios: happy - identical seeds give identical combat traces (replay CRC equal); failure - projectile targeting a dead/cleared target retargets or expires without panic, Evidence .omo/evidence/gap5-ra-projectiles.log
  Commit: Y | feat(ra): projectiles with flight time, misses, splash

- [x] 6. TD building fire: projectiles
  What to do / Must NOT do: Convert `update_defence_combat` (game.rs:3380) direct damage to fire through the existing TD `weapon.rs` projectile system (spawn projectile at barrel, travel, impact applies damage). Keep the existing 9 defence tests' semantics (range/power/reload/SAM anti-air gate) by updating them to observe projectile spawn + impact, not weakening assertions. Must NOT change `td_rules.rs` weapon tables.
  Parallelization: Wave 3 | Blocked by: none | Blocks: 13 | Parallel with: 5,7
  References: RUST/cnc-tiberian-dawn/src/game.rs:3380 (update_defence_combat), tests at :7649-7742; RUST/cnc-tiberian-dawn/src/weapon.rs; original/TIBERIANDAWN/BUILDING.CPP (fire timing)
  Acceptance criteria (agent-executable): `defence_fires_projectile_not_direct_damage` green: tower acquires and a projectile entity exists, target takes damage only after projectile arrival; all prior defence tests updated, none deleted.
  QA scenarios: happy - Guard Tower kills an approaching tank via projectile within its range; failure - target leaves range mid-flight → projectile expires without damage, Evidence .omo/evidence/gap6-td-building-fire.log
  Commit: Y | feat(td): defensive structures fire projectiles

- [x] 7. Naval depth: ship classes and naval-yard production
  What to do / Must NOT do: RA: per-ship stats from rules (`ShipE1..E6` gunboat/destroyer/cruiser/sub/transport/carrier), carrier launches aircraft, transport carries passengers (existing cargo/passenger fields), naval yard queues ships (production path like other factories, ship spawn on water only). TD: gunboat patrols and bombards land targets (the only TD warship). Ships pathfind only on water cells (MoveClass extension). Must NOT give land units water pathing; must not break amphibious transport behavior from the earlier todo.
  Parallelization: Wave 3 | Blocked by: none | Blocks: 13 | Parallel with: 5,6
  References: RUST/cnc-core/src/class_id.rs:97-102 (ShipE1-E6); RUST/cnc-red-alert/src/game.rs:3763 (naval rearm), :1331 (naval yard spawns); RUST/cnc-red-alert/src/pathfind.rs (MoveClass); original/REDALERT/NAVAL.CPP; original/TIBERIANDAWN/UNIT.CPP (gunboat)
  Acceptance criteria (agent-executable): `ra_naval_yard_produces_ships_on_water` green; `ra_carrier_launches_and_recovers_aircraft` green; `td_gunboat_bombards_land_target` green; ships never path onto land cells.
  QA scenarios: happy - cruiser shells shore structures from water; failure - ship production without a naval yard or land spawn → rejected with typed error, Evidence .omo/evidence/gap7-naval.log
  Commit: Y | feat: naval classes, production, bombardment

- [x] 8. TeamType planner: TD build planner + RA commander per house
  What to do / Must NOT do: TD: implement the empty planner stubs (ai.rs:133-147) — defense_groups/attack_groups populated from team types, build tasks queued toward taskforce requirements (BuildTask exists), using MaxUnit/MaxBuilding limits. RA: run `AiCommander` per AI house (not hardcoded owner 1) with difficulty-scaled cadence from `RulesDatabase::difficulty` (todo-25 plumbing exists), and honor taskforce RecruitPriority/MaxAllowed when recruiting. Must NOT remove the existing wave-attack fallback for scenarios without `[TeamTypes]`.
  Parallelization: Wave 4 | Blocked by: 3 (real scenarios exercise it) | Blocks: 13 | Parallel with: 9,10,11,12
  References: RUST/cnc-tiberian-dawn/src/ai.rs:125-160; RUST/cnc-red-alert/src/game.rs:2856-2900 (ai loop), rules difficulty at :7454-7471; RUST/cnc-core/src/rules.rs (DifficultyRule); original/TIBERIANDAWN/TEAM.CPP; original/REDALERT/HOUSE.CPP (AI_Attack cadence)
  Acceptance criteria (agent-executable): `td_planner_fills_taskforce_by_building` green; `ra_each_ai_house_commands_independently` green (2v2 skirmish: four AI houses each build/economize/attack); difficulty Easy vs Hard produce measurably different attack cadence.
  QA scenarios: happy - scenario with TeamTypes sees teams recruited to strength and attacking; failure - scenario without TeamTypes falls back unchanged, Evidence .omo/evidence/gap8-planner.log
  Commit: Y | feat: teamtype planners in both engines

- [x] 9. Trigger vocabulary: full TEVENT (RA 33) + TACTION (RA 37), TD to table parity
  What to do / Must NOT do: RA: port the full `EventText` (TEVENT.CPP:50, 33 rows) and `ActionText` (TACTION.CPP:56, 37 rows) into `original_scenario.rs`/trigger model with executable implementations where the engine already has the primitive (win/lose, create/destroy team, all-hunt, reinforcements, drop-zone flare, reveal, timer start/stop/extend/shorten/set, global set/clear, play speech/music/sound, allow-win, production begin, autocreate, fire sale, preferred target, launch nukes → wired to existing superweapon paths). TD: extend from ~6/17 to the full 17 TEVENT rows the TD table declares (TRIGGER.CPP:67 EventText[EVENT_COUNT]). Unimplementable originals stay explicitly reported as unmapped — never invented, never silently dropped.
  Parallelization: Wave 4 | Blocked by: 2 | Blocks: 13 | Parallel with: 8,10,11,12
  References: original/REDALERT/TEVENT.CPP:50; original/REDALERT/TACTION.CPP:56-105; original/TIBERIANDAWN/TRIGGER.CPP:67; RUST/cnc-tiberian-dawn/src/scenario_loader.rs:102 (TD table + mapping tests pattern); RUST/cnc-red-alert/src/original_scenario.rs (trigger records); RUST/cnc-core/src/trigger_records.rs
  Acceptance criteria (agent-executable): mapping-table tests pin every row of both tables (`ra_eventtable_complete`, `ra_actiontable_complete`, `td_eventtable_complete`); a replayable scenario exercising ≥10 of the RA actions fires them end-to-end (`ra_trigger_actions_execute`); unmapped count drops to documented-only.
  QA scenarios: happy - timer+global+reinforcement combo scenario behaves; failure - unknown action name warns and skips with the name in the message, Evidence .omo/evidence/gap9-triggers.log
  Commit: Y | feat: complete trigger vocabulary both engines

- [x] 10. EVA emission: TD to full table, RA from zero to gameplay
  What to do / Must NOT do: RA: emit `RaEvaLine` from real gameplay events mirroring TD's call sites — construction complete, unit lost, structure lost, base under attack (35s cooldown), insufficient funds (60s), mission accomplished/failed, superweapon ready/launched, low power. TD: extend from 12 toward the full 45-line table where the trigger condition exists in-engine (airstrike detected, timer expired, objective met...). Never emit a line without an in-engine condition; keep queue one-slot semantics.
  Parallelization: Wave 4 | Blocked by: none | Blocks: 13 | Parallel with: 8,9,11,12
  References: RUST/cnc-audio/src/eva_ra.rs:14 (RaEvaLine); RUST/cnc-tiberian-dawn/src/game.rs (14 existing speak sites — copy cooldown/idempotence patterns); RUST/cnc-red-alert/src/game.rs (event sites: structure lost, unit lost already tracked per gap-5 tests); original/REDALERT/AUDIO.CPP; original/TIBERIANDAWN/AUDIO.CPP
  Acceptance criteria (agent-executable): `ra_eva_emits_on_gameplay_events` green (≥8 distinct lines from a scripted skirmish: structure destroyed → StructureLost, funds drop → InsufficientFunds, win → MissionAccomplished); `td_eva_table_coverage` green (≥20 lines reachable, each pinned by a condition test).
  QA scenarios: happy - destroying a player structure announces within one tick, cooldown prevents spam; failure - two events same tick → one queued, second dropped per Speak semantics, Evidence .omo/evidence/gap10-eva.log
  Commit: Y | feat: EVA emission in both engines

- [x] 11. RA specials: wire the stub modules into the sim
  What to do / Must NOT do: Wire `dog.rs` (dogs kill infantry in one bite, cannot attack vehicles/structures, detect spies), `medic.rs` (heals adjacent infantry, not self, no weapons), `tanya.rs` (demolition on buildings via C4, twin pistols vs infantry), `madtank.rs` (charge-up then area detonation), `engineer.rs` (capture enemy structures + repair friendly — engineer.rs:16 already lists NavalYard) into `Game` spawn/attack/update paths. spy.rs (201L) is already wired — verify, don't redo. Each special gets behavior tests; damage constants come from the modules, not invented in game.rs.
  Must NOT do: Do not delete the modules and re-implement inline; do not give medics attack capability; do not let dogs target vehicles.
  Parallelization: Wave 4 | Blocked by: 5 (projectile pipeline affects combat) | Blocks: 13 | Parallel with: 8,9,10,12
  References: RUST/cnc-red-alert/src/{dog,medic,tanya,madtank,engineer,spy}.rs (current sizes: 12/19/14/20/26/201 L); RUST/cnc-red-alert/src/game.rs (spawn_unit, combat loop, capture path); original/REDALERT/{DOG,MEDIC,TANYA,MAD,ENGINEER}.CPP
  Acceptance criteria (agent-executable): five named tests — `dog_kills_infantry_one_bite`, `medic_heals_adjacent_infantry`, `tanya_demolishes_building`, `madtank_detonates_after_charge`, `engineer_captures_structure_and_repairs_friendly` — all green; TD engineer/commando behavior unchanged.
  QA scenarios: happy - mixed force with medic sustains an assault longer than without (deterministic seed comparison); failure - dog ordered at a tank → refuses/no-op with reason, Evidence .omo/evidence/gap11-specials.log
  Commit: Y | feat(ra): special units wired into simulation

- [x] 12. TD wire/sim: apply_command arms for all Command variants
  What to do / Must NOT do: Give every `cnc-tiberian-dawn::network::Command` variant (network.rs:12, ~17 variants; 11 lack arms) a `Game::apply_command` arm that routes to the same `order_*` path the UI uses; add the game-state CRC beside the command-hash in the framesync path so replay verification covers state, not just commands. Board/Unload must go through the passenger system.
  Must NOT do: Do not create a second command path divergent from `order_*`; do not break existing codec round-trip tests.
  Parallelization: Wave 4 | Blocked by: 1 | Blocks: 13 | Parallel with: 8,9,10,11
  References: RUST/cnc-tiberian-dawn/src/network.rs:12-90 (Command variants); RUST/cnc-tiberian-dawn/src/game.rs (apply_command + order_* fns; apply_sabotage as the existing pattern); RUST/cnc-tiberian-dawn/src/network.rs:86-92 (command-hash)
  Acceptance criteria (agent-executable): `every_command_variant_applies` green (exhaustive match drives each variant through apply_command on a live game and asserts the observable state change); `state_crc_detects_divergence` green (two games, one diverged command application → CRC mismatch reported).
  QA scenarios: happy - scripted command stream via apply_command produces the same final state as direct order_* calls (state equality); failure - unknown/corrupt payload → typed error, no panic, Evidence .omo/evidence/gap12-apply-command.log
  Commit: Y | feat(td): full wire command surface + state CRC

- [x] 13. Final gap re-audit
  What to do / Must NOT do: Re-verify each of the ten gap rows with live anchors; update PARITY_AUDIT_2026-10-24.md (or successor doc) marking rows PASS with evidence paths or documented divergence. Baseline test count must strictly increase from 1746; zero pre-existing tests weakened.
  Parallelization: Wave 5 | Blocked by: 1-12 | Blocks: none | Parallel with: none
  References: RUST/docs/PARITY_AUDIT_2026-10-20.md; .omo/evidence/gap*.log
  Acceptance criteria (agent-executable): re-audit shows 0 open actionable rows; `cd RUST && cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` green.
  QA scenarios: happy - every gap row has PASS evidence; failure - any open row becomes an appended fix todo before completion, Evidence .omo/evidence/gap13-reaudit.log
  Commit: Y | docs: gap closure re-audit

## Gates (every todo)
- TDD: named failing test first.
- `cargo fmt --check`; `CARGO_BUILD_JOBS=2 cargo clippy -p <crates> --all-targets -- -D warnings`; `CARGO_BUILD_JOBS=2 cargo test -p <crates>`.
- Evidence log per todo under `.omo/evidence/`; commit message as listed.
- Never weaken/delete pre-existing tests; never panic on malformed input (warn + skip).
