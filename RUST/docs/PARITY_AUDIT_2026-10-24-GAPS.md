# Parity Gap Closure Audit — parity-gaps todos 1–13

**Date:** 2026-10-24
**Scope:** the ten functional gaps ranked in `PARITY_AUDIT_2026-10-20.md` §3, worked by
`.omo/plans/parity-gaps.md`. Every row below names its commit and its acceptance evidence.

**Method:** each row re-verified against the live tree — the named acceptance test exists and passes, or the
pre-existing state is quoted with its anchor. A row is PASS only on a green gate (fmt + clippy + the crate's
suite). Rows whose plan target exceeded what the engine's own rules allow are marked *adjusted* with the
reason, never silently narrowed.

## 1. Row-by-row verdict

| # | Gap (parity-gaps) | Verdict | Commit | Evidence / notes |
|---|---|---|---|---|
| 1 | Baseline reconciliation + anchor refresh | PASS | `16715fa` | Todo 1 of the gaps plan; anchors refreshed. |
| 2 | TD `[CellTriggers]` end-to-end | PASS | `0d79710` | Trigger evaluation over cell bindings. |
| 3 | RA `[Houses]` + `[Base]` consumption | PASS | `479c372`, `23e9d77` | Houses/base objects instantiated. |
| 4 | Editor import UI for original maps | PASS | `748baa1` | `editor_imports_original_maps_via_command` — import through the undo command. |
| 5 | RA projectiles: flight-time, miss rolls, splash | PASS | `7090265` + `1f1c162` | `ra_projectile_travels_and_misses`, `ra_splash_hits_adjacent`. **Correction:** gap 5 shipped red — its refactor dropped `update_unit_combat` from `Game::update`, disabling all unit combat; fixed in `1f1c162` (see `.omo/evidence/gap5-correction.log`). |
| 6 | TD building fire: projectiles | PASS | `3e5e946` | `defence_fires_projectile_not_direct_damage`. |
| 7 | Naval depth | PASS | `b092bf2` + `436e6af` | `td_gunboat_bombards_land_target`; carrier semantics grounded in NAVAL.CPP (launches onboard aircraft as its weapon). |
| 8 | TeamType planner both engines | PASS | `9fe8643` | `td_planner_fills_taskforce_by_building`, `ra_each_ai_house_commands_independently`, `ra_difficulty_scales_attack_cadence`. |
| 9 | Trigger vocabulary (full TEVENT + TACTION) | PASS | `609e2c2` + `a19f500` | Mapping tables pin every row (RA 33 events / 37 actions, TD 17 events); `ra_trigger_actions_execute` fires ten actions end-to-end. 12 action families without engine support stay **documented-only** (trait defaults call `reject()`), per the plan's "never invented" rule. |
| 10 | EVA emission both engines | PASS | `16915b4` (+ queue earlier) | `ra_eva_emits_on_gameplay_events` — ten distinct `RaEvaLine`s from real sites. TD reaches 13 distinct lines through in-engine conditions (`td_eva_table_coverage`). *Adjusted:* the plan's "TD ≥20 of 45" assumed ~29 more in-engine conditions; the engine tracks 16 emitters and the plan forbids emitting a line without a condition, so the remainder is documented-only. |
| 11 | RA specials | PASS | `4b3625d` | `dog_kills_infantry_one_bite`, `medic_heals_adjacent_infantry`, `tanya_demolishes_building`, `madtank_detonates_after_charge`, `engineer_captures_structure_and_repairs_friendly`. *Adjusted:* `d3fda61` shipped red (10 compile errors, its five named tests were nested inside another fn and never ran); repaired in `4b3625d`. Tanya's twin-pistols-vs-infantry needs a weapon row the roster lacks — documented, not faked. |
| 12 | TD wire command surface + state CRC | PASS | `2f2d1c4` | All 17 `Command` variants route through `try_apply_command` (no catch-all); FNV-1a `state_crc()` already existed. `every_command_variant_applies` + `state_crc_detects_divergence` added. |
| 13 | Final gap re-audit | PASS | this doc | Gates below. |

## 2. Corrections that the re-audit caught (drives trust over optimism)

1. **Gap 5 committed a dead combat loop** (`7090265`): unit combat never ran through `Game::update`; the suite
   was left red at commit time. Fixed `1f1c162`.
2. **Gap 11 committed unbuildable code with fake tests** (`d3fda61`): the module did not compile, and the
   "5 named TDD tests" were nested fn bodies inside another test — never collected. Fixed `4b3625d`
   (extricated to real `#[test]` fns, dog bite resolution, Tanya C4 arm, weapon-gate reorder).
3. **The plan's premises were partly stale** (todos 7, 9, 12): the naval/trigger/command rows described work
   that already existed at their commit dates; the delivered value was the acceptance tests plus real gaps
   (carrier/gunboat behavior, RA action dispatch, EVA sites).
4. **Plan targets bounded by engine rules** (todos 9, 10): the trigger and EVA tables have rows with no
   in-engine condition; both plans explicitly forbid emitting rows without one, so the unmapped remainder is
   documented (not invented) and the tests pin the reachable sets.

## 3. Gate state at closure

- `cargo fmt --check`: PASS (workspace)
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS (0 errors)
- `cargo test --workspace`: PASS (0 failures) — see `.omo/evidence/task-13-gaps-audit.log` for the summed count.
- One integration fix required by the closure: `ae89959` (launcher TD lane EVA variant) — the RA queue change
  altered the RA game's `eva` type, and the launcher's TD theme check needed the original TD variant restored.

## 4. Remaining known gaps (documented, out of scope for parity-gaps)

- Tanya twin pistols vs infantry (no weapon row for the commando slot).
- `madtank::is_mad_tank` returns false until a MAD-tank ClassId exists in the roster; the detonation path is
  implemented but unreachable.
- TD `ConstructionComplete` emitter exists but no scenario yet drives a queued building to completion in a
  quiet game (placement gating).
- 12 RA trigger action families + ~29 TD EVA table rows without in-engine conditions.
