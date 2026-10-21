# parity-100 - Work Plan

## TL;DR (For humans)
<!-- Fill this LAST, after the detailed plan below is written, so it summarizes the REAL plan. -->
<!-- Plain English for a non-engineer: NO file paths, NO todo numbers, NO wave/agent/tool names. -->

**What you'll get:** The Rust rebuild reaches full functional parity with the original Command & Conquer source drop: both games load and play the ORIGINAL campaign maps and campaigns, combat feels like C&C (armor types, projectiles, base defenses that fire), computer opponents run real team-based AI, maps have fog-of-war, missions get voice warnings, briefing movies play, and the map editor gains the missing tools and reads/writes original map formats.

**Why this approach:** The gap list comes from a source-verified audit, so work is ordered by dependency — original map loading first (it unlocks testing everything else against real content), engine depth next, then AI/fog, then presentation, with a finite final re-audit as the proof of "100%". An independent reviewer pass fixed stale assumptions before a single line of work was planned (several "missing" items were already done; the plan verifies before it builds).

**What it will NOT do:** No Steam Workshop integration. No authentic Adlib/IMF sound-card emulation and no byte-compatible loading of original save files (you chose to keep those divergences). No changes to the original C++/C# source — it is reference material only.

**Effort:** XL
**Risk:** High - determinism contract changes (seeded combat randomness) and save/replay format version bumps touch every engine system; mitigated by two-identical-games gates per change and wave-level build/test gates.
**Decisions to sanity-check:** (1) Intentional divergences stay out of scope — you confirmed this; (2) Red Alert gains seeded combat randomness, which invalidates old recorded replays; (3) GPS superweapon and paratroops deliberately deferred until fog-of-war exists; (4) the modernization crates (event bus, structured logs) get wired in rather than deleted.

Your next move: start execution in a worker session, or run a high-accuracy plan review first. Full execution detail follows below.

---

> TL;DR (machine): XL effort, high risk (determinism/save-format contracts); 53 todos + 4 verifiers close 100% of audit-identified parity gaps across TD, RA, editor, presentation, video; divergence list frozen by user decision.

## Scope
### Must have
Close 100% of the gaps enumerated in `RUST/docs/PARITY_AUDIT_2026-10-20.md` (§1 headline verdicts, §3 top-10 cross-lane gaps, §4–§7 per-lane tables, §8 hygiene items) plus the two still-open items in `RUST/docs/PARITY.md` §3 (compressed VQA variants, carry-over launcher wiring). Concretely:
- C1 Original-format content I/O: LCW codec (**live: `cnc-asset/src/format80.rs` ALREADY provides `decompress` (:46) + `compress` (:270) with 11 tests** — verify/reuse, do NOT reimplement; the "none exists" claim was stale); TD `SCENARIO.CPP`-dialect loader COMPLETED (**live: 1468-line `cnc-tiberian-dawn/src/scenario_loader.rs`, 0 TODO markers — the ":459–506" anchors were stale, that work is done**); RA `.MPR`/original SCENARIO.INI loader (MapPack/OverlayPack — confirmed absent); original TEVENT/TACTION record parser (cnc-core); editor open/save of original maps incl. per-house and `[Basic]`-family sections (cnc-asset codecs exclusively); launcher original-campaign discovery + carry-over flow. Fixtures: synthetic golden fixtures committed in-repo; real game data env-gated (`CNC_ORIGINAL_DATA_DIR`), never committed.
- C2 TD engine depth: full unit/aircraft/infantry roster (incl. mission dinosaurs), transports/cargo, defensive buildings that fire, `Game::apply_command` arms for all 15 wire commands + game-state CRC, FloatBias handicap, credit tick, alliances, overlay/wall/terrain-object combat logic, sub-cell infantry.
- C3 RA engine depth: seeded-deterministic combat rolls, verses-armor + projectiles, RulesDatabase consumption (stats/build times/difficulty), air+naval consolidation + RA passenger/cargo transports, RA specials (crates, nuke launch, paradrops, vortex, GPS, dogs, medics, Tanya, MAD tank, engineer capture, Spy/Thief wiring), mission state machine, multi-house, RA carry-over + replay recording (extracted into cnc-replayer).
- C4 TeamType/TaskForce engine wiring in BOTH engines + full TEVENT (~30) / TACTION (~31) enums + trigger attachment model + IQ/difficulty/special flags.
- C5 Per-house shroud/fog layer consulted by targeting/AI + radar/GPS/gap-generator integration.
- C6 Presentation: EVA/VOX speech pipeline + call sites, radar/minimap + sidebar depth (incl. Commando/Engineer rows), RA score screen, in-game SFX wiring + theme queue, MP/options dialogs, stats collection.
- C7 VQA compressed-chunk decode (CBPZ/CBFZ/VQFR/SND) + VXL voxel parsing/conversion + launcher briefing/intro/ending playback screens.
- C8 Hygiene (live-verified at review rr2): baseline reconciliation of the dirty tree (192 entries at planning; re-verify count at todo 1) + anchor refresh; `cargo fmt --check` clean (currently failing — last known `cnc-asset/src/classic_pack.rs:314,351`; run from `RUST/`); net-directory + ready.rs poll hooks wired into launcher UDP loops; cnc-events + cnc-logs member-ized and given real consumers (todos 4/5); cnc-console member-ized + compile-restored (its consumer disposition is recorded at the re-audit's §8 row) — membership is NOT yet done (live-verified: three crates untracked + absent from `RUST/Cargo.toml` members; `cnc-console/src` has no `lib.rs`, only `.bak` remnants); one-time PARITY.md ↔ audit doc reconciliation + `RUST/codemap.md` stale test-count fix; France bug IS fixed in the working tree (verified) — todo 1 re-verifies it and LANDS the membership/compile repairs as owner-labeled commits.
- C9 Editor closure: waypoints tool, TeamTypes tab, multi-cell footprints + 5-slot infantry groups, INI-driven type registries, TemplateTool/smudge/walls/resources tools, MIX theater tilesets, team colors, rule-aware validation, full trigger dialog.
- C10 Final re-audit vs `ORIGINAL_FUNCTIONALITY_MAP.md` proving every gap row closed (or explicitly recorded divergence).
- Every gap-fix ships parity fixtures pinned to C++ constants and keeps the test suite green against the Wave-0 recorded baseline (count + fmt + clippy), not a frozen figure (Metis finding 13).

### Must NOT have (guardrails, anti-slop, scope boundaries)
- NO edits to `TIBERIANDAWN/`, `REDALERT/`, `CnCTDRAMapEditor/`, `SCRIPTS/` (read-only reference).
- NO Steam Workshop integration; NO Adlib/IMF emulation; NO IPX/null-modem/DDE/Westwood-Online protocols; NO byte-compatible original `.SAV` loading; NO GlyphX-client ABI work (user decision 2026-10-20: keep divergences divergent).
- NO new gameplay features absent from the original engines; existing Rust-native lanes (cnc-ai Agent Studio, procgen, net-directory) stay intact.
- NO wholesale "modernization" of working modules (e.g., do not replace `Game` flat structs with cnc-core entity hierarchy mid-plan; consolidation todos are scoped precisely).
- NO `git reset --hard`, history rewrites, or pushes without explicit user authorization (AGENTS §24). Commits land locally on the current branch.
- NO weakening/deleting existing tests to make parity tests pass (AGENTS §17.2). Deliberate behavior changes may update pinned expectation values in the same commit; coverage must not shrink.
- NO mixing of unrelated in-flight user work into single commits (todo 1 lands the dirty tree as owner-labeled per-feature commits; nothing pushed).
- NO cnc-viewer spectator build-out (Rust-native reserved lane with no original-engine counterpart — recorded as documented divergence at re-audit).
- NO pointer-fixup save rework: `cnc-core::pointers` stays as tested core infra; engines' index-based snapshots remain the documented divergence (user-approved: no original .SAV compat).

## Verification strategy
> Zero human intervention - all verification is agent-executed.
- Test decision: TDD for new subsystems (codec, loaders, shroud, RNG, enums); tests-after for bug fixes (France bug, console break, fmt). Framework: built-in `#[test]` + `cargo test` per crate, mirroring existing patterns (`td_rules.rs` pins C++ constants; `game.rs` two-identical-games determinism test; `history.rs` round-trips).
- Global gates every wave: `cargo fmt --check`, `cargo clippy --workspace --all-targets`, `cargo test --workspace` — all must pass before a wave is declared done (local pre-push CI parity, AGENTS/Forgejo policy).
- Host-resource governance (AGENTS §7, 16-GiB shared host): wave-gate builds run SERIALLY — never more than 2 concurrent cargo-heavy jobs, long gates wrapped in `systemd-run --user --scope -p MemoryMax=4G`; parallelism applies to editing lanes, never to concurrent full-workspace builds.
- Parity evidence: each todo cites its audit gap row; C10 re-audit re-runs the 4-lane methodology and must find zero open gap rows.
- Evidence: `<attemptDir>/task-<N>-parity-100.log` (attemptDir = currentAttemptDir from 'omo ulw-loop status --json', .omo/evidence/ulw/<session>/<goalId>/a<attempt>; outside ulw-loop use .omo/evidence/)

## Execution strategy
### Parallel execution waves
> Target 5-8 todos per wave. Fewer than 3 (except the final) means you under-split.
- **Wave 0 (C8 hygiene + baseline):** todo 1 (inventory + owner-labeled commit plan — runs ALONE first) LANDS the dirty tree as per-owner feature commits (one feature per commit, message labels the owner lane, nothing pushed, no mixing) → todos 2–6 in parallel (fmt, net-directory+ready-up, cnc-events wiring, cnc-logs wiring, doc reconciliation) on a clean, owned tree; each later todo commits only its own diff.
- **Wave 1 (C1 content I/O):** todos 7 + 10 + 11 first (parallel — fixtures BEFORE loaders so todo 8/9 acceptance is executable) → todos 8/9 parallel (TD finish, RA build) → 12 (editor I/O) → 13 TD-half; 13's RA-half completes after todo 31 lands (Wave 2).
- **Wave 2 (engine depth):** TWO serial chains only — TD chain 14→15→16→17→18→19→20→21 and RA chain 22→23→24→25→26→27→28→29→30→31 — because both crates' `game.rs` monoliths are cited by most todos; serial-per-crate eliminates write contention (review round 1, both lanes). Parallelism = TD chain ∥ RA chain. **GPS + paradrops are NOT here — moved to Wave 3 (Metis finding 7).**
- **Wave 3 (AI + shroud):** C4 chain 32 → 33 → 34 → 35 → 36 (34 before 35 — both edit `ai.rs`) ∥ C5 chain 37 → 38; 38 runs STRICTLY after the C4 chain (32–36) (its targeting-call-site edits touch `ai.rs`/`house.rs`, and its `special_shroud_off_disables_fog` acceptance consumes todo-36's flag wiring). File-ownership contract (Metis finding 9): the shroud lane owns `cnc-core/src/visibility.rs` + map seams; the C4 chain owns `ai.rs`/`house.rs`; 38's targeting-call-site carve-out is the only exception and applies only after the C4 chain completes.
- **Wave 4 (presentation / video / editor):** C6 explicit total chain 39 → 40 → 44 → 41 → 42 → 43 (strict `main.rs` serialization; 44 before 41 because the score screen consumes stats), then 45 → 46 (C7; 46 after the whole C6 chain) ∥ editor SERIAL chain 47 → 48 → 49 → 50 → 51 → 52 (single lane — todos share plugin.rs registrations, map-model section I/O, and undo plumbing; serialization replaces the lane-A/B split whose file-disjointness could not be evidenced). Metis findings 9–10 + review rounds 1–2.
- **Wave 5 (proof):** todo 53 — finite re-audit: every actionable gap row/sub-item across PARITY_AUDIT §1–§8 re-verified with live anchors (PASS + evidence, or recorded divergence per Scope OUT — incl. cnc-viewer and cnc-core::pointers) + PARITY.md reconciled; F-map used for spot-check only (Metis finding 12). Punch-list residuals recorded, none silently dropped.
- Orchestrator reconciles per wave: run `cd RUST && cargo fmt --check && cargo clippy --workspace --all-targets && cargo test --workspace`, reconcile partial failures, then unblock the next wave. Deterministic-sim changes additionally gated by the two-identical-games test pattern extended to RA (and now including RNG state in the compared state — Metis finding 6).

### Dependency matrix
| Todo | Depends on | Blocks | Can parallelize with |
| --- | --- | --- | --- |
| 1 baseline + owner-labeled commits | none | 2–53 (all lanes) | none (runs alone) |
| 2 fmt | 1 (tree landed) | all (gate) | 3,4,5,6 |
| 3 net-directory+ready-up | 1 | 43 | 2,4,5,6 |
| 4 cnc-events wiring | 1 | 44 | 2,3,5,6 |
| 5 cnc-logs wiring | 1 | — | 2,3,4,6 |
| 6 doc reconciliation | 1 | — | 2,3,4,5 |
| 7 LCW codec | 1 | 8,9,12,45,52 | 10,11 |
| 8 TD loader finish | 7,10,11 | 12,13,34,35 (TeamType data) | 9 |
| 9 RA MPR loader | 7,10,11 | 12,13,34,35 | 8 |
| 10 record parser | 1 | 8,9,32,33 | 7,11 |
| 11 fixtures | 1 | 8,9,12,13 | 7,10 |
| 12 editor original I/O | 8,9,11 | 13 | W2 chains |
| 13 campaign+carryover wiring | 12,31 (TD-half after 12; RA-half after 31) | 53 | — |
| 14 TD roster (chain head) | 1 | 15 | 22 (RA chain) |
| 15 TD sub-cell infantry | 14 | 16 | 22–31 (RA chain) |
| 16 TD transports | 15 | 17,38 (paradrops) | 22–31 (RA chain) |
| 17 TD defensive buildings | 16 | 18 | 22–31 (RA chain) |
| 18 TD apply arms + CRC | 17 | 19 | 22–31 (RA chain) |
| 19 TD economy | 18 | 20 | 22–31 (RA chain) |
| 20 TD overlays/walls | 19 | 21 | 22–31 (RA chain) |
| 21 TD pathfinding | 20 | 53 | 22 (RA chain) |
| 22 RA RNG policy (chain head) | 1 | 23 | 14–21 (TD chain) |
| 23 RA verses combat | 22 | 24 | 14–21 (TD chain) |
| 24 RA projectiles + facing | 23 | 25 | 14–21 (TD chain) |
| 25 RA rules wiring | 24 | 26,36,50 | — |
| 26 RA air/naval + transports + amphib | 25 | 27 | — |
| 27 RA economy depth | 26 | 28 | 14–21 (TD chain) |
| 28 RA mission FSM | 27 | 29 | 14–21 (TD chain) |
| 29 RA multi-house | 28 | 30,35 | 14–21 (TD chain) |
| 30 RA specials I (incl. MAD tank) | 29 | 31 | 14–21 (TD chain) |
| 31 RA carryover + replay (→ cnc-replayer) | 30 | 13,53 | — |
| 32 TEVENT/TACTION | 10 | 33,38,48 | 37 |
| 33 attached triggers | 32 | 34 | 37 |
| 34 TD teams | 33,8 (TeamType data) | 35,38 | 37 |
| 35 RA teams + Expert_AI | 34,29 | 36,38 | 37 |
| 36 IQ/difficulty | 25,35 | 43 | 37 |
| 37 shroud primitive | 1 | 38,40 | 32–36 |
| 38 shroud consumers + GPS + paradrops + special-flag acceptance | 37,32,16,34,35,36 | 40 | — (strictly after C4 chain 32–36) |
| 39 EVA/VOX + CSF | 1 | 40 | editor chain, 45 |
| 40 radar/minimap + sidebar | 39,38 | 44 | editor chain, 45 |
| 41 RA score + briefing | 44,38 | 42 | editor chain, 45 |
| 42 SFX + theme queue | 41 | 43 | editor chain, 45 |
| 43 MP/options dialogs | 42,3,36 | 53 | editor chain, 45 |
| 44 stats collection | 40,4 | 41 | editor chain, 45 |
| 45 VQA chunks + VXL | 7 | 46 | editor chain, C6 |
| 46 playback screens | 45, C6 chain done | 53 | — (main.rs serial) |
| 47 waypoints tool (C9 serial head) | 1 | 48 | C6 chain, 45 |
| 48 TeamTypes tab (C9 serial) | 47,32 | 49 | C6 chain, 45 |
| 49 footprints + groups (C9 serial) | 48 | 50 | C6 chain, 45 |
| 50 type registries (C9 serial) | 49,25 | 51 | C6 chain, 45 |
| 51 tools + MIX + team colors + renderer icon-cache (C9 serial) | 50 | 52 | C6 chain, 45 |
| 52 MEG/PGM + validation + trigger dialog (C9 serial tail) | 51,7 (LCW write-back) | 53 | C6 chain, 45 |
| 53 re-audit §1–§8 | all above | F1–F4 | none |
| 54 cnc-console + cnc-events build (pre-existing debt) | 3,4,5 | none | 6,7 |

## Todos
> Implementation + Test = ONE todo. Never separate.
<!-- APPEND TASK BATCHES BELOW THIS LINE WITH edit/apply_patch - never rewrite the headers above. -->
- [x] 1. Baseline reconciliation + anchor refresh
  What to do / Must NOT do: Inventory the dirty tree (`git status --porcelain` at repo root; **live 211 entries / 381 with `-uall`** — the "192 at planning" figure was stale); map every dirty/untracked source path to its owning feature (scenario_loader → TD-content lane; lobby/ready/score/menu/options → launcher lane; format80/video/classic_pack → asset lane; entity.rs → team/trigger lane; workspace `RUST/Cargo.toml` → infra/membership lane; `cnc-events/` → events lane; `cnc-logs/` → logs lane; `cnc-console/` → console lane). Produce an owner-labeled commit plan, then LAND the tree as per-owner feature commits — one commit per owner feature, commit message labels the owner lane, NOTHING pushed, no feature mixing in one commit. Do NOT revert or restage. Re-verify stale audit claims against live source: France→GrandCannon in `cnc-red-alert/src/house.rs` (verified fixed), workspace membership + cnc-console compile (live-verified BROKEN at review rr2: three crates untracked, absent from members, `cnc-console/src` lacks `lib.rs` — REPAIR in this todo: add the three members to `RUST/Cargo.toml`, restore/recreate `cnc-console/src/lib.rs` from `.bak` remnants, landed as owner-labeled infra commits), actual fmt-failure locations (**live: one violation at `cnc-tiberian-dawn/src/scenario_loader.rs:201`** — the classic_pack.rs:314,351 claim was stale), TD `scenario_loader.rs` TODOs (**live: none; 1468 lines, 0 TODO/todo! markers — those anchors were stale**) — and REWRITE any stale numbers cited in this plan file (doc-only edits allowed here). Record baseline: test count, fmt/clippy status.
  Parallelization: Wave 0 | Blocked by: none (runs ALONE) | Blocks: 2–53
  References (executor has NO interview context - be exhaustive): `git status --porcelain` (**211 entries / 381 with `-uall`**, verified at todo 1 — the "192" figure was stale); RUST/docs/PARITY_AUDIT_2026-10-20.md §8; RUST/docs/PARITY.md §5; RUST/cnc-tiberian-dawn/src/scenario_loader.rs (**no TODO markers remain**); Metis review findings 1–4,13 (recorded in .omo/drafts/parity-100.md D5); review round 1 findings 5/10/11 (commit entanglement, anchor drift).
  Acceptance criteria (agent-executable): `.omo/evidence/baseline-report.md` maps every dirty path → owner AND lists the landed per-owner commits (hash + message); `cd RUST && cargo test --workspace 2>&1 | tail -3` recorded verbatim; every anchor cited in this plan grep-verified and stale in-plan numbers rewritten (scripted check, output in evidence log).
  QA scenarios (name the exact tool + invocation): happy — gates run and recorded even if red (report states actuals); failure — a cited anchor is missing → correct the citing plan row in this same todo (doc-only edit allowed), Evidence .omo/evidence/task-1-parity-100.log
  Commit: Y | chore(baseline): land dirty tree as owner-labeled per-feature commits
- [x] 2. Workspace fmt + clippy green
  What to do / Must NOT do: `cd RUST && cargo fmt`, then fix remaining `cargo fmt --check` violations (**live: none — fmt was already clean at todo-1 close**; the classic_pack.rs:314,351 anchor was stale); `cargo clippy --workspace --all-targets` clean (no blanket allow-attributes; justify any local allow). Do NOT touch unrelated dirty files beyond rustfmt scope.
  Parallelization: Wave 0 | Blocked by: 1 | Blocks: all (wave gate)
  References: Metis findings 2/13; RUST/BUILD_STATUS.md Formatting section; pre-push CI policy (AGENTS §17.5).
  Acceptance criteria (agent-executable): `cd RUST && cargo fmt --check && cargo clippy --workspace --all-targets` both exit 0.
  QA scenarios: happy — both exit 0; failure — each violation listed then fixed before close, Evidence .omo/evidence/task-2-parity-100.log
  Commit: Y | style(workspace): make fmt and clippy gates green
- [x] 3. Wire cnc-net-directory + ready-up into launcher
  What to do / Must NOT do: Add `cnc-net-directory` dep to `cnc-launcher/Cargo.toml`; lobby screen invoking DirectoryClient (LAN multicast discovery minimum, LFG optional); wire `ready.rs` ReadyTracker poll/send hooks into the launcher UDP poll loops. Do NOT alter LiveSession protocol semantics.
  Parallelization: Wave 0 | Blocked by: 1 | Blocks: 43
  References: RUST/cnc-net-directory/src/ (Distributor, matchmaking.rs, discovery/); RUST/cnc-launcher/Cargo.toml:22–27 (dep absent); cnc-launcher/src/codemap.md "READY hooks not wired"; RUST/cnc-launcher/src/main.rs UDP poll loops (~5902–6454); AUDIT §3.9/§7.
  Acceptance criteria (agent-executable): `grep -n "cnc-net-directory" RUST/cnc-launcher/Cargo.toml` matches (re-verified 2026-10-20: Cargo dependency keys use the hyphen spelling; the original `cnc_net_directory` grep was stale); tests `lobby_lists_directory_services` and `ready_tracker_polls_in_udp_loop` pass.
  QA scenarios: happy — two local instances discover each other via directory in integration test; failure — directory timeout → direct-connect fallback without panic, Evidence .omo/evidence/task-3-parity-100.log
  Commit: Y | feat(launcher): wire net-directory lobby and ready-up hooks
- [x] 4. Give cnc-events real consumers
  What to do / Must NOT do: Emit typed game events (unit produced/lost, structure captured, superweapon fired, mission outcome) through the existing EventBus from both engines' update paths; launcher subscribes for HUD/log. Precondition: cnc-events must be a workspace member (todo 1's infra commit lands this if still missing). Do NOT restructure the EventBus API.
  Parallelization: Wave 0 | Blocked by: 1 | Blocks: 44
  References: RUST/cnc-events/src/ (bus, definitions, emission); AUDIT §7 "orphaned" row (27 tests, zero consumers).
  Acceptance criteria (agent-executable): ≥3 publish call sites across cnc-tiberian-dawn/cnc-red-alert/cnc-launcher (grep evidence); test `engine_events_reach_launcher_subscriber` passes.
  QA scenarios: happy — scripted tick publishes ≥5 event types, subscriber receives in order; failure — dropped subscriber → engine continues, no panic, Evidence .omo/evidence/task-4-parity-100.log
  Commit: Y | feat(events): plumb game events from engines to launcher
- [x] 5. Give cnc-logs a real consumer
  What to do / Must NOT do: Initialize the LogBuffer tracing layer in launcher + engines with per-target filters and bounded memory. Init + subscriber only; no wholesale rewrite of log call sites. Precondition: cnc-logs must be a workspace member (todo 1's infra commit lands this if still missing).
  Parallelization: Wave 0 | Blocked by: 1 | Blocks: none
  References: RUST/cnc-logs/src/ (buffer, init, layer, rotation); AUDIT §7.
  Acceptance criteria (agent-executable): `grep -n "cnc_logs::init" RUST/cnc-launcher/src/main.rs` matches; test `log_buffer_bounded_under_flood` passes; launcher boot writes ≥1 structured entry.
  QA scenarios: happy — boot emits structured logs into buffer; failure — flood path → rotation drops oldest, no panic, Evidence .omo/evidence/task-5-parity-100.log
  Commit: Y | feat(logs): initialize structured logging in launcher
- [x] 6. Documentation reconciliation pass
  What to do / Must NOT do: Re-verify and correct PARITY.md vs PARITY_AUDIT_2026-10-20.md contradictions (fmt "RESOLVED" claim, France "FIXED", loader status, AI/combat over-claims) with dated "re-verified" notes; fix TD codemap test-count claim ("154 unit" → actual) and renderer codemap stub claims; add Steam-divergence note to cnc-map-editor/README.md; mark cnc-classic as roadmap placeholder. Do NOT delete either doc.
  Parallelization: Wave 0 | Blocked by: 1 | Blocks: none
  References: RUST/docs/PARITY.md (Metis finding 4); RUST/cnc-tiberian-dawn/codemap.md; RUST/cnc-renderer/src/codemap.md; RUST/cnc-map-editor/README.md; RUST/cnc-classic/README.md; RUST/codemap.md + RUST/scripts/sync-docs.sh (stale 552-test figure — fix already applied in working tree 2026-10-20; verify it survived todo 1's commit landing, update if re-baselined).
  Acceptance criteria (agent-executable): every corrected claim carries "re-verified 2026-10-20" + evidence path; `grep -i steam RUST/cnc-map-editor/README.md` matches divergence note.
  QA scenarios: happy — every doc claim traced to a live grep; failure — contradiction found → corrected in place with evidence, Evidence .omo/evidence/task-6-parity-100.log
  Commit: Y | docs: reconcile parity docs with live-tree verification
- [x] 7. LCW codec in cnc-asset
  What to do / Must NOT do: New `cnc-asset/src/lcw.rs` — Westwood LCW decode + encode (mode-byte semantics per the C# reference); round-trip property tests; known-vector decode test against a hand-built MapPack-style fixture. Reuse `format80.rs` module/test patterns. Do NOT reimplement Format80 — it exists; do NOT wire consumers yet (that is todos 8/9/12).
  Parallelization: Wave 1 | Blocked by: 1 | Blocks: 8,9,12,45,52 | Parallel with: 10,11
  References (executor has NO interview context - be exhaustive): CnCTDRAMapEditor/Utility/WWCompression.cs:1–688 (LCW reference implementation); RUST/cnc-asset/src/format80.rs (**decompress :46 / compress :270, 11 tests — this IS the LCW codec; verify and extend it rather than re-adding it**); Metis finding 5 (**REFUTED at todo 1: LCW is present in format80.rs**).
  Acceptance criteria (agent-executable): `cd RUST && cargo test -p cnc-asset lcw` green with ≥8 tests incl. `lcw_round_trips_random_buffers` and `lcw_decode_known_vector`.
  QA scenarios (name the exact tool + invocation): happy — `lcw_decode(fixture) == expected plaintext bytes`; failure — 10k fuzzed corrupt buffers → Err, no panic/UB, Evidence .omo/evidence/task-7-parity-100.log
  Commit: Y | feat(asset): LCW decode/encode codec
- [x] 8. Finish TD original-scenario loader + wire launcher
  What to do / Must NOT do: Complete `cnc-tiberian-dawn/src/scenario_loader.rs`: apply LCW `.bin` terrain/overlay (**done — no TODOs remain**), finish unit/structure/trigger/house instantiation (**done**), add `Game::from_original_scenario` (**present**); **live: `scenario_loader.rs` is 1468 lines, 0 TODO markers**; launcher loads original campaign files when `CNC_ORIGINAL_DATA_DIR` set (skip + warn when unset). Do NOT break the 9 authored-dialect maps (`RUST/maps/td_campaign/`, tests/campaign_missions.rs must stay green).
  Parallelization: Wave 1 | Blocked by: 7,10,11 | Blocks: 12,13,34,35 | Parallel with: 9
  References: RUST/cnc-tiberian-dawn/src/scenario_loader.rs:1–776 (section parsing, inline tests); TIBERIANDAWN/SCENARIO.CPP (original dialect semantics); AUDIT §3.1 TD half, §4 scenario row; Metis finding 3 (loader exists, unwired).
  Acceptance criteria (agent-executable): `cargo test -p cnc-tiberian-dawn scenario_loader` green incl. new `original_td_scenario_builds_game_with_terrain_overlay_units` (todo-11 fixture — 11 lands before this todo's acceptance runs); existing campaign_missions tests stay green; env-gated integration test discovers real campaign dir when provided.
  QA scenarios: happy — todo-11 fixture loads with non-empty TERRAIN/OVERLAY cells + units with correct house/health; failure — malformed section → error list returned (never panic), Evidence .omo/evidence/task-8-parity-100.log
  Commit: Y | feat(td): complete original scenario loading
- [x] 9. RA original .MPR/SCENARIO.INI loader
  What to do / Must NOT do: New `cnc-red-alert/src/scenario_mpr.rs`: parse [Basic]/[MapPack](LCW)/[OverlayPack](LCW)/[Houses]/[Infantry]/[Structures]/[Units]/[Aircraft]/[Ships]/[Waypoints]/[Trigs]/[TeamTypes]/[Base]/[SMUDGE]/[TERRAIN]; build Game. Keep existing new-dialect `scenario.rs` untouched and green.
  Parallelization: Wave 1 | Blocked by: 7,10,11 | Blocks: 12,13,34,35 | Parallel with: 8
  References: CnCTDRAMapEditor/RedAlert/GamePlugin.cs:226–268,1152–1523 (section layout + LCW usage); REDALERT/SCENARIO.CPP; RUST/cnc-red-alert/src/scenario.rs (existing dialect, keep intact); AUDIT §3.1 RA half.
  Acceptance criteria (agent-executable): `cargo test -p cnc-red-alert scenario_mpr` green incl. `original_ra_mpr_builds_game` (houses/waypoints/triggers populated from todo-11 fixture).
  QA scenarios: happy — todo-11 fixture .MPR loads with expected entity counts (11 lands first); failure — bad MapPack checksum → Err naming the section, Evidence .omo/evidence/task-9-parity-100.log
  Commit: Y | feat(ra): original MPR scenario loader
- [x] 10. Original TEVENT/TACTION record parser (cnc-core)
  What to do / Must NOT do: New `cnc-core/src/trigger_records.rs`: parse original trigger record strings (`<event>,<params>` / `<action>,<params>` incl. house/unit/trigger references) into typed payloads; handle ALL ~30 event + ~31 action record codes with `Unknown(code)` passthrough for not-yet-enum'd ones (full enums land in todo 32). Do NOT wire into engines yet.
  Parallelization: Wave 1 | Blocked by: 1 | Blocks: 8,9,32,33 | Parallel with: 7,11
  References: REDALERT/TEVENT.H, TACTION.H; TIBERIANDAWN/TRIGGER.H:38–104 (record formats); AUDIT §3.7.
  Acceptance criteria (agent-executable): `cargo test -p cnc-core trigger_records` green with ≥6 tests round-tripping each record shape family.
  QA scenarios: happy — every record code parses or passes through as Unknown(code); failure — truncated record → Err with record code, Evidence .omo/evidence/task-10-parity-100.log
  Commit: Y | feat(core): original trigger record parser
- [x] 11. Golden fixture set (synthetic in-repo, real data env-gated)
  What to do / Must NOT do: Commit license-clean synthetic original-format fixtures under `RUST/fixtures/`: TD .INI+.BIN pair and RA .MPR with hand-verified contents (terrain counts, entity lists, trigger records); loader tests optionally read real game data via `CNC_ORIGINAL_DATA_DIR` (skipped when unset). NEVER commit original game data (non-redistributable).
  Parallelization: Wave 1 | Blocked by: 1 | Blocks: 8,9,12,13 | Parallel with: 7,10
  References: Metis finding 5 (zero game data in TIBERIANDAWN//REDALERT — verified); RUST/scripts/download_openra_assets.py (license-clean precedent); AUDIT §3.1.
  Acceptance criteria (agent-executable): fixture files exist with committed checksums; `cargo test -p cnc-tiberian-dawn fixture` passes WITHOUT the env var set.
  QA scenarios: happy — CI green without env var; failure — env var points at garbage → tests skip with warning (not fail), Evidence .omo/evidence/task-11-parity-100.log
  Commit: Y | test: synthetic original-format golden fixtures
- [ ] 12. Editor opens/saves original maps
  What to do / Must NOT do: `cnc-map-editor` GamePlugin gains load/save dispatch for TD ini+bin and RA .MPR consuming cnc-asset codecs EXCLUSIVELY (no local codec copies — Metis finding 10); semantic round-trip: open fixture → save → section-equal.
  Parallelization: Wave 1 | Blocked by: 8,9,11 | Blocks: 13 | Parallel with: W2 chains
  References: RUST/cnc-map-editor/src/plugin.rs (trait shape); CnCTDRAMapEditor/TiberianDawn/GamePlugin.cs:115–160,858–957; CnCTDRAMapEditor/RedAlert/GamePlugin.cs:226–268; AUDIT §6 (original-format map I/O row).
  Acceptance criteria (agent-executable): tests `editor_roundtrips_original_td_fixture` and `editor_roundtrips_original_ra_fixture` pass; per-house and `[Basic]`-family sections survive round-trip (`editor_preserves_basic_and_house_sections`); `grep -rn "lcw" RUST/cnc-map-editor/src` = 0 matches (no local codec).
  QA scenarios: happy — round-trip produces section-equal INI; failure — unsupported extension → error listing supported formats, Evidence .omo/evidence/task-12-parity-100.log
  Commit: Y | feat(editor): original-format map I/O via cnc-asset codecs
- [ ] 13. Launcher original-campaign flow + carry-over wiring
  What to do / Must NOT do: Mission select lists original campaigns (env-gated dir from todos 8/9); mission end → score → next mission; TD `carryover::Carryover` written at mission end and loaded at next start (PARITY.md §3.4 remaining wiring); RA flow consumes todo-31 carryover when present. Split: TD-half lands in Wave 1 (blocked by 12); RA-half completes after todo 31 lands (Wave 2). Do NOT regress authored-campaign flow.
  Parallelization: Wave 1 (TD-half), completing in Wave 2 (RA-half after 31) | Blocked by: 12 (TD-half), 31 (RA-half) | Blocks: 53
  References: RUST/cnc-launcher/src/{menu.rs,score.rs}; RUST/cnc-tiberian-dawn/src/carryover.rs (CNCCARRY, checksummed); RUST/docs/PARITY.md §3.4; AUDIT §3.1.
  Acceptance criteria (agent-executable): test `campaign_flow_advances_and_carries_credits` (TD) passes; RA variant passes once 31 lands.
  QA scenarios: happy — scripted 2-mission run carries credits+surviving units; failure — missing next mission file → completion screen, no panic, Evidence .omo/evidence/task-13-parity-100.log
  Commit: Y | feat(launcher): original campaign progression + carry-over
- [x] 14. TD roster completion
  What to do / Must NOT do: Add missing UnitType/AircraftType rows (APC, Artillery, MLRS, Stealth Tank, Flame Tank, SSM Launcher, Mobile HQ, Grenadier, Rocket Soldier, Flamethrower, Chem Warrior, Chinook/Transport, mission dinosaurs) with stats pinned to TIBERIANDAWN/UNIT.CPP + IDATA constants; MCV deploy gets a simulation apply arm (Command::Deploy is wire-only). Do NOT touch RA crate files.
  Parallelization: Wave 2 (TD chain head) | Blocked by: 1 | Blocks: 15 | Parallel with: 22–31 (RA chain)
  References: RUST/cnc-tiberian-dawn/src/td_rules.rs:613–641 (existing row pattern + test style); AUDIT §4 roster row (~⅔ of roster missing — reconcile the exact row count against UNIT.CPP at authoring time); TIBERIANDAWN/UNIT.CPP, AIRCRAFT.CPP (stats source).
  Acceptance criteria (agent-executable): td_rules roster tests pin every new row vs C++ constants; per-unit in-game spawn/build/fire test `td_roster_per_unit_checks` in game.rs.
  QA scenarios (name the exact tool + invocation): happy — each unit builds, moves, fires per pinned stats; failure — unknown ini name → deterministic error (never silently skipped), Evidence .omo/evidence/task-14-parity-100.log
  Commit: Y | feat(td): complete unit roster
- [x] 15. TD sub-cell infantry + stances + doors
  What to do / Must NOT do: Implement StoppingCoordAbs 5-slot per-cell infantry groups (CONST.CPP:59–65), stance states incl. prone, door-tile open/close semantics for transport pairs. Replaces the `reserved_cell` single-occupancy mitigation. Snapshot format must version-bump (new per-cell sub-slots).
  Parallelization: Wave 2 (TD chain) | Blocked by: 14 | Blocks: 16 | Parallel with: 22–31 (RA chain)
  References: TIBERIANDAWN/CONST.CPP:59–65 (StoppingCoordAbs); TIBERIANDAWN/INFANTRY.CPP:2891–2901 region; RUST/cnc-tiberian-dawn/src/unit.rs; AUDIT §4 infantry row.
  Acceptance criteria (agent-executable): 5 infantry occupy one cell deterministically; save/replay round-trip includes sub-cell positions; `five_infantry_share_cell_deterministically` passes.
  QA scenarios: happy — five E1 in one cell with pinned offsets; failure — 6th entry rejected deterministically, Evidence .omo/evidence/task-15-parity-100.log
  Commit: Y | feat(td): sub-cell infantry groups
- [x] 16. TD transports/cargo system
  What to do / Must NOT do: CARGO.CPP semantics: passenger list field, boarding/unload orders for APC/Chinook/Transport (roster from 14); passengers die with destroyed transport; dock/unload at waypoint. Do NOT implement paradrops here (todo 38).
  Parallelization: Wave 2 (TD chain) | Blocked by: 15 | Blocks: 17,38 | Parallel with: 22–31 (RA chain)
  References: TIBERIANDAWN/CARGO.CPP; AUDIT §3.6 (transport passengers MISSING — both engines), §4 transports row (cited by category; audit row numbers drift).
  Acceptance criteria (agent-executable): `transport_loads_unloads_deterministically` passes; destroyed transport kills passengers.
  QA scenarios: happy — Chinook boards 5 infantry, unloads at waypoint; failure — overloaded boarding rejected deterministically, Evidence .omo/evidence/task-16-parity-100.log
  Commit: Y | feat(td): passenger transports
- [ ] 17. TD defensive buildings fire
  What to do / Must NOT do: Add weapon field to Building; `update_attacks` (game.rs:1825) iterates buildings too; roster additions GTWR/DTWR towers, Obelisk, SAML, GASF, Airdrome, Fix; turret rotation for buildings; bibs for placement. Do NOT reuse unit turret code where ROF/recharge differs (SAML missile cadence per BUILDING.CPP).
  Parallelization: Wave 2 (TD chain) | Blocked by: 16 | Blocks: 18 | Parallel with: 22–31 (RA chain)
  References: RUST/cnc-tiberian-dawn/src/building.rs (311 L, no weapon today); RUST/cnc-tiberian-dawn/src/game.rs:1825 (update_attacks units-only); TIBERIANDAWN/BUILDING.CPP (towers, Obelisk, SAML); AUDIT §3.5, §4 defensive-buildings row (cited by category).
  Acceptance criteria (agent-executable): tower engages in-range enemy without player units (`tower_fires_without_player_units`); SAML fires missiles at aircraft; Obelisk charge-up cadence pinned.
  QA scenarios: happy — base defense kills raider with no player input; failure — unpowered defense holds fire (power gating), Evidence .omo/evidence/task-17-parity-100.log
  Commit: Y | feat(td): defensive structures fire
- [ ] 18. TD apply_command arms + game-state CRC
  What to do / Must NOT do: `Game::apply_command` arms for all 15 wire Command variants (11 unapplied today — only Sabotage applied, network.rs:197–209); add world-state CRC to lockstep frames (beyond command hash, network.rs:86–92) exchanged like FRAMESYNC. Exhaustive match = compile-enforced completeness. Do NOT change transport semantics (todo 16 owns those).
  Parallelization: Wave 2 (TD chain) | Blocked by: 17 | Blocks: 19 | Parallel with: 22–31 (RA chain)
  References: RUST/cnc-tiberian-dawn/src/network.rs:86–92,197–209,230–283; RUST/cnc-tiberian-dawn/src/game.rs:1708–1712; AUDIT §3.8.
  Acceptance criteria (agent-executable): exhaustive match compiles (every variant has an arm); `two_identical_games_compare_state_crcs` passes; injected divergence → `desynced()` true.
  QA scenarios: happy — identical command streams → identical CRCs across processes; failure — one mutated command → desync flagged deterministically, Evidence .omo/evidence/task-18-parity-100.log
  Commit: Y | feat(td): full command application + state CRC
- [ ] 19. TD economy fidelity (FloatBias, credit tick, alliances)
  What to do / Must NOT do: Wire `cnc_core::FloatBias` handicap into TD House (exists in cnc-core, zero TD consumers — AUDIT §3.10); credit tick accumulation display state (CREDITS.CPP semantics: gradual 1-per-tick-ish roll-up, snapshot-serialized); ALLY alliance events + member tracking. Do NOT change instant-spending correctness (existing house tests stay green).
  Parallelization: Wave 2 (TD chain) | Blocked by: 18 | Blocks: 20 | Parallel with: 22–31 (RA chain)
  References: RUST/cnc-core/src/float_bias.rs (ported, unused — AUDIT §3.10); RUST/cnc-tiberian-dawn/src/house.rs:69–142; AUDIT §4 economy row, §3.10.
  Acceptance criteria (agent-executable): handicap bias alters production cost/speed per pinned ratio (`handicap_alters_economy`); credit display rolls up gradually in snapshot; ALLY event sets alliance in both houses.
  QA scenarios: happy — easy bias produces measurably cheaper/faster builds; failure — bias zero → identical to unhandicapped, Evidence .omo/evidence/task-19-parity-100.log
  Commit: Y | feat(td): handicap, credit tick, alliances
- [ ] 20. TD overlays/walls/terrain-object combat
  What to do / Must NOT do: Wall/vegetation overlay objects with hit points; warhead destroy-wall/wood flags applied in damage path (warhead rows exist in td_rules.rs:47–137 but flags unused); smudge/scorch persistence in snapshots; land-type semantics (water drown, tree block) replacing bare `is_passable`. Do NOT touch renderer art (editor lane owns visuals).
  Parallelization: Wave 2 (TD chain) | Blocked by: 19 | Blocks: 21 | Parallel with: 22–31 (RA chain)
  References: RUST/cnc-tiberian-dawn/src/td_rules.rs:47–137 (warhead rows; destroy-walls/wood triple unmodeled); TIBERIANDAWN/OVERLAY.CPP, TERRAIN.CPP, SMUDGE.CPP; AUDIT §3.7 (walls/overlays), §4 overlays row (cited by category).
  Acceptance criteria (agent-executable): HE destroys wall cell, sniper does not (`warhead_flags_gate_wall_damage`); scorch persists across save/replay; infantry drowned in water cell.
  QA scenarios: happy — sandbag wall absorbs rifle then falls to HE; failure — wrong flag → no-op, Evidence .omo/evidence/task-20-parity-100.log
  Commit: Y | feat(td): destructible overlays, smudges, land types
- [ ] 21. TD pathfinding depth
  What to do / Must NOT do: Zone-based search + edge-following per FINDPATH.CPP; `Can_Enter_Cell` per-terrain checks beyond the is_passable flag; remove/raise MAX_PATH_NODES stall on long detours; deterministic tie-breaks preserved. Do NOT change the octile A* public API consumers rely on without updating call sites in the same todo.
  Parallelization: Wave 2 (TD chain tail) | Blocked by: 20 | Blocks: 53 | Parallel with: 22–31 (RA chain)
  References: TIBERIANDAWN/FINDPATH.CPP; RUST/cnc-tiberian-dawn/src/pathfind.rs; RUST/cnc-tiberian-dawn/src/game.rs:2351–2450; AUDIT §4 pathfinding row (~60%).
  Acceptance criteria (agent-executable): unit paths around long wall lines without corner-cutting (`long_detour_succeeds`); water blocks infantry, allows boat; no stall on unreachable → deterministic no-move.
  QA scenarios: happy — 40-cell detour completes; failure — unreachable target → unit holds position, no spin, Evidence .omo/evidence/task-21-parity-100.log
  Commit: Y | feat(td): zone-based pathfinding
- [x] 22. RA seeded RNG policy + integration
  What to do / Must NOT do: Per-frame deterministic combat rolls via `cnc-procgen::prng` (SplitMix64); seed sourced from replay/scenario header; RNG state serialized in snapshot + included in CRC + the two-identical-games gate; replay format version bump with pre-RNG replays explicitly invalidated (documented in the module). Closes the no-RNG determinism-contract change deliberately (Metis finding 6).
  Parallelization: Wave 2 (RA chain head) | Blocked by: 1 | Blocks: 23 | Parallel with: 14–21 (TD chain)
  References: REDALERT/RANDOM.CPP (successive-resolution roll semantics); RUST/cnc-procgen/src/prng.rs (pinned SplitMix64); RUST/cnc-red-alert/src/game.rs (zero RNG today — verified); AUDIT §1 core-determinism row (audit has no §-subsections; anchor re-verified at todo 1); Metis finding 6.
  Acceptance criteria (agent-executable): same seed + command stream → identical outcomes across two processes (`seeded_rolls_identical_across_processes`); replay v3 header round-trips seed + rng state.
  QA scenarios (name the exact tool + invocation): happy — two identical games with rolls stay identical incl. CRC; failure — seed mismatch → deterministic desync error naming the frame, Evidence .omo/evidence/task-22-parity-100.log
  Commit: Y | feat(ra): seeded deterministic combat RNG
- [x] 23. RA verses/armor combat
  What to do / Must NOT do: Apply `WarheadRule.verses_percent` + ArmorType per GameUnit/GameStructure (exists in cnc-core rules.rs:345, unconsumed by RA); miss/deflection rolls via todo-22 RNG; splash radius per warhead. Do NOT touch TD damage code.
  Parallelization: Wave 2 (RA chain) | Blocked by: 22 | Blocks: 24 | Parallel with: 14–21 (TD chain)
  References: RUST/cnc-core/src/rules.rs:345 (verses_percent — live anchor :347/:376 at review rr3; re-verify at todo 1; 12 tests); REDALERT/WARHEAD.CPP, COMBAT.CPP; RUST/cnc-red-alert/src/game.rs:1852–2046 (flat damage today); AUDIT §3.4, §5 combat row.
  Acceptance criteria (agent-executable): `rifle_verses_heavy_armor_below_rifle_verses_infantry` (pinned percentages); splash damages adjacent cells; misses roll deterministically.
  QA scenarios: happy — heavy tank shrugs rifle fire per verse matrix; failure — unknown armor type → 100% passthrough + one-time warning, Evidence .omo/evidence/task-23-parity-100.log
  Commit: Y | feat(ra): verses-armor damage model
- [ ] 24. RA projectiles + 32-angle facing
  What to do / Must NOT do: Real projectile travel (speed/arc/homing per WeaponRule projectile refs) replacing instant-application Shot tracers; 32-angle FACING rotation for units and turrets (FACING.CPP/TURRET.CPP semantics). Projectile lifetime expiry must not leak.
  Parallelization: Wave 2 (RA chain) | Blocked by: 23 | Blocks: 25 | Parallel with: 14–21 (TD chain)
  References: REDALERT/BULLET.CPP (51K), FACING.CPP (32 angles), TURRET.CPP; RUST/cnc-tiberian-dawn/src/weapon.rs:36–152 (trajectory pattern to mirror); RUST/cnc-red-alert/src/game.rs:488 (visual-only Shot today); AUDIT §1 bullet/combat rows, §5 entities row (anchor re-verified at todo 1).
  Acceptance criteria (agent-executable): shells take ≥1 tick to arrive (`projectile_travels_over_ticks`); rear attacks land before rotation completes (`facing_rotation_delays_return_fire`); no shot leaks after expiry.
  QA scenarios: happy — destroyer shell visibly travels then hits; failure — target dies mid-flight → projectile fizzles without panic, Evidence .omo/evidence/task-24-parity-100.log
  Commit: Y | feat(ra): ballistics + facing rotation
- [ ] 25. RA rules wiring
  What to do / Must NOT do: All RA stats read from `RulesDatabase`: unit_profile (game.rs:2126), structure_cost/health/power (2180–2217), launcher build times (main.rs:5140) become data-driven; DifficultyRule applied. Keep hardcoded tables as fallback defaults with warning (never panic on missing section).
  Parallelization: Wave 2 (RA chain) | Blocked by: 24 | Blocks: 26,36,50 | Parallel with: 14–21 (TD chain)
  References: RUST/cnc-core/src/rules.rs (RulesDatabase, 12 tests incl. RA difficulty alias); RUST/cnc-red-alert/src/game.rs:2126–2240; RUST/cnc-launcher/src/main.rs:5140; AUDIT §3.4, §5 rules row (~50%).
  Acceptance criteria (agent-executable): `ra_stats_from_rules_fixture` asserts values from a fixture rules.ini override; build time from rules matches fixture.
  QA scenarios: happy — fixture rules change alter in-game stats; failure — missing section → defaults + warning, Evidence .omo/evidence/task-25-parity-100.log
  Commit: Y | feat(ra): rules-driven stats and difficulty
- [ ] 26. RA air/naval consolidation + amphibious pathfinding
  What to do / Must NOT do: Consolidate the dual implementations: keep dedicated `aircraft.rs`/`naval.rs` semantics by moving their tested behaviors into the in-sim GameUnit roles (single source of truth), delete the unwired dedicated structs; in-sim air re-arm at helipad/airfield; submerged subs untargetable/un-fireable; add Amphibious MoveClass to pathfind.rs; add RA passenger/cargo transports (APC/transport vehicles, CARGO.CPP semantics — closes the RA half of audit §3.6 transports); RA pathfinding threat cost + removal of the path-node cap (audit §5 pathfind sub-items). Do NOT leave two parallel implementations (Metis finding 11/audit §3.6).
  Parallelization: Wave 2 (RA chain) | Blocked by: 25 | Blocks: 27 | Parallel with: 14–21 (TD chain)
  References: RUST/cnc-red-alert/src/aircraft.rs (237 L, tested, unwired), naval.rs (190 L, same); RUST/cnc-red-alert/src/game.rs:1928–1931 (no re-arm), :423 (submerged hook only); RUST/cnc-red-alert/src/pathfind.rs (3 MoveClass, no Amphibious); REDALERT/CARGO.CPP; AUDIT §3.6, §5 entities+pathfind rows.
  Acceptance criteria (agent-executable): `heli_rearms_at_helipad_in_sim` passes; `submerged_sub_untargetable` enforced in targeting loop; amphibious unit crosses land+water in one path; `ra_transport_loads_unloads` passes (passengers board, ride, disembark; die with transport).
  QA scenarios: happy — helicopter cycles combat/rearm; failure — dock destroyed → aircraft fights until ammo 0 then retires to airfield, Evidence .omo/evidence/task-26-parity-100.log
  Commit: Y | feat(ra): consolidate air/naval + amphibious pathfinding
- [ ] 27. RA economy depth (per-factory queues, prereqs, silo capacity)
  What to do / Must NOT do: Split single FIFO (house.rs:79) into per-building-factory queues (barracks/war-factory/shipyard/helipad); prerequisite gating wired to rules (25); silo capacity + overflow credit loss. Do NOT regress exact-refund tests (spending_and_refunds_are_exact stays green).
  Parallelization: Wave 2 (RA chain) | Blocked by: 26 | Blocks: 28 | Parallel with: 14–21 (TD chain)
  References: RUST/cnc-red-alert/src/house.rs:79–188; REDALERT/FACTORY.CPP; AUDIT §5 economy row (~70%).
  Acceptance criteria (agent-executable): two queues build in parallel (`parallel_factory_queues_build`); prereq-unmet production blocked with reason; silo overflow loses credits per pinned rule.
  QA scenarios: happy — infantry and armor build simultaneously; failure — cancel refunds exact per queue, Evidence .omo/evidence/task-27-parity-100.log
  Commit: Y | feat(ra): per-factory queues, prereqs, silo capacity
- [ ] 28. RA mission state machine
  What to do / Must NOT do: Mission enum (Guard/Hunt/Enter/Unload/Attack/Retreat/Move/Sticky/Unload/Construction...) per MISSION.CPP with guard-area + retaliation semantics; orders transition the machine. Do NOT merge with order_* plumbing — machine sits under it.
  Parallelization: Wave 2 (RA chain) | Blocked by: 27 | Blocks: 29 | Parallel with: 14–21 (TD chain)
  References: REDALERT/MISSION.CPP; RUST/cnc-red-alert/src/game.rs (move/attack/harvest/scatter/stop only today); AUDIT §5 entities row (mission states MISSING).
  Acceptance criteria (agent-executable): attacked guard retaliates then returns to post (`guard_retaliates_then_returns`); Hunt engages nearest enemy.
  QA scenarios: happy — full guard→attack→return cycle deterministic; failure — invalid transition → logged no-op, state machine stable, Evidence .omo/evidence/task-28-parity-100.log
  Commit: Y | feat(ra): mission state machine
- [ ] 29. RA multi-house + alliances
  What to do / Must NOT do: Break the hard-coded 2-owner model (game.rs:651, ai owner 1 at :676); N houses with colors/spawn slots; alliance sets; per-house AI enable. Do NOT change single-player 2-house defaults.
  Parallelization: Wave 2 (RA chain) | Blocked by: 28 | Blocks: 30,35 | Parallel with: 14–21 (TD chain)
  References: RUST/cnc-red-alert/src/game.rs:651,676; RUST/cnc-red-alert/src/house.rs; AUDIT §5 houses row.
  Acceptance criteria (agent-executable): 4-player skirmish boots from scenario (`four_house_skirmish_boots`); allied units never friendly-fire.
  QA scenarios: happy — 2v2 with alliances resolves correctly; failure — house index OOB → deterministic error, Evidence .omo/evidence/task-29-parity-100.log
  Commit: Y | feat(ra): multi-house support
- [ ] 30. RA specials I (non-shroud-dependent)
  What to do / Must NOT do: Engineer building capture (IsCaptureable gate per BDATA), attack dogs, medics, Tanya demolition, MAD tank (seismic charge + self-destruct sequence), Spy/Thief wired into Game command flow (spy.rs exists isolated — infiltrate/steal effects land in sim), crates (CRATE.CPP money/unit/invulnerability), chronal vortex (VORTEX.CPP ramp). Do NOT do GPS/paradrops here (todo 38 owns them — shroud/transport dependencies, Metis finding 7).
  Parallelization: Wave 2 (RA chain) | Blocked by: 29 | Blocks: 31 | Parallel with: 14–21 (TD chain)
  References: RUST/cnc-red-alert/src/spy.rs (201 L, tests, unwired); REDALERT/CRATE.CPP, VORTEX.CPP; AUDIT §3.7, §3.10 (MAD tank listed), §5 specials row.
  Acceptance criteria (agent-executable): engineer captures structure ownership; dog kills infantry in one bite; crate pickups grant pinned effects; vortex ramps through base destroying in sequence; MAD tank charge/detonation sequence deterministic (named test `ra_specials_each_deterministic` covering each).
  QA scenarios: happy — each special deterministic across identical games; failure — capture refused on IsCaptureable=false, Evidence .omo/evidence/task-30-parity-100.log
  Commit: Y | feat(ra): infantry specials, crates, vortex
- [ ] 31. RA carry-over + replay recording
  What to do / Must NOT do: Port TD carryover semantics to RA (lift `carryover.rs` to cnc-core or twin it — decision: twin in RA crate, matching RA save being text-based); RA replay record path via LiveSession (from_replay exists; record side missing for RA); extract replay record/playback into the `cnc-replayer` crate so TD + RA both consume it (retires the skeleton status — closes audit §7 replayer row). Includes RNG seed in header (22). Do NOT change TD carryover behavior.
  Parallelization: Wave 2 (RA chain tail) | Blocked by: 30 | Blocks: 13,53 | Parallel with: 14–21 (TD chain)
  References: RUST/cnc-tiberian-dawn/src/carryover.rs (CNCCARRY reference); RUST/cnc-tiberian-dawn/src/network.rs from_replay; RUST/cnc-launcher replay paths (main.rs:5902–6454); RUST/cnc-replayer/codemap.md (skeleton); AUDIT §5 save/replay row, §3.10, §7.
  Acceptance criteria (agent-executable): `ra_mission_carry_preserves_credits_and_units`; `ra_replay_reproduces_identical_state` (seeded); cnc-replayer is a workspace member hosting the replay module with its tests passing.
  QA scenarios: happy — replay of recorded RA match ends in identical CRC; failure — corrupt replay → rejected with header error, Evidence .omo/evidence/task-31-parity-100.log
  Commit: Y | feat(ra): carry-over + replay recording
- [x] 32. Full TEVENT/TACTION enums + apply arms
  What to do / Must NOT do: Complete event (~30) and action (~31) enums with typed payloads in cnc-core (extends todo-10 parser); apply arms in BOTH engines including superweapon actions ACTION_AIRSTRIKE/NUKE/ION/DZ (campaigns script superweapons), ALL_TO_HUNT, AUTOCREATE, REVEAL_MAP, CREDITS_TRANSFER, EVAC_CIVILIAN, WIN/LOSE/ALLOWWIN. Exhaustive match = compile-enforced. Closes AUDIT §3.7, §4/§5 trigger rows.
  Parallelization: Wave 3 (C4 chain head) | Blocked by: 10 | Blocks: 33 | Parallel with: 37 (C5 chain)
  References: REDALERT/TEVENT.H, TACTION.H (full enumerations); TIBERIANDAWN/TRIGGER.H:38–104; AUDIT §3.7 (5/5 vs ~30/~31 today); RUST/cnc-core/src/trigger_records.rs (from todo 10).
  Acceptance criteria (agent-executable): every enum variant parses (todo-10 records) + has an apply arm in both engines; fixture trigger fires airstrike strike event (`trigger_airstrike_fires`).
  QA scenarios (name the exact tool + invocation): happy — scripted campaign trigger executes nuke action deterministically; failure — future unknown action code → Unknown(code) + warning, mission continues, Evidence .omo/evidence/task-32-parity-100.log
  Commit: Y | feat(core): full trigger event/action vocabulary
- [x] 33. Trigger attachment model
  What to do / Must NOT do: Attach triggers to cells, houses, and units (EVENT_DISCOVERED/ATTACKED/DESTROYED/ANY + house-filtered conditions); re-evaluation only on attachment events, not global polling. Do NOT regress elapsed/count triggers (mission tests stay green).
  Parallelization: Wave 3 (C4 chain) | Blocked by: 32 | Blocks: 34 | Parallel with: 37 (C5 chain)
  References: TIBERIANDAWN/TRIGGER.CPP (attachment semantics); REDALERT/TEVENT.CPP; AUDIT §3.7 (no cell binding today).
  Acceptance criteria (agent-executable): attacking a guarded structure fires its ATTACKED trigger once (`attacked_trigger_fires_once`); discovery trigger fires on first sight of attached house.
  QA scenarios: happy — cell-bound trigger fires when unit enters; failure — re-attack within cooldown → no refire, Evidence .omo/evidence/task-33-parity-100.log
  Commit: Y | feat: attached trigger model
- [ ] 34. TD TeamType/TaskForce engine
  What to do / Must NOT do: Replace the empty planner stubs (ai.rs:133–147) with cnc-core TeamType/TaskForce-driven teams: team formation from members, mission assignment, AUTOCREATE reinforcement, defense_groups/attack_groups actually populated and dispatched. Uses loaded TeamTypes (todo 8/9 data + todo 33 events). Closes AUDIT §3.2 TD half.
  Parallelization: Wave 3 (C4 chain) | Blocked by: 33,8 (TeamType data) | Blocks: 35 | Parallel with: 37 (C5 chain)
  References: RUST/cnc-core/src/entity.rs:616+ (TeamType/TaskForce/TeamMember parsers, 25 tests, zero consumers); RUST/cnc-tiberian-dawn/src/ai.rs:133–147 (no-op stubs), :316–452 (planner to keep); TIBERIANDAWN/TEAM.CPP, TEAMTYPE.CPP; AUDIT §3.2.
  Acceptance criteria (agent-executable): scenario TeamTypes spawn teams on AUTOCREATE (`teamtype_autocreate_spawns_team`); defense group intercepts a raider entering its zone.
  QA scenarios: happy — two-team scenario executes both objectives deterministically; failure — TeamType with zero members → skipped with warning, Evidence .omo/evidence/task-34-parity-100.log
  Commit: Y | feat(td): team-based AI
- [ ] 35. RA teams + Expert_AI + multi-AI
  What to do / Must NOT do: Parameterize `run_expert_ai` (currently TD-only, TD ai.rs:311) as a shared house-commander driving RA houses (lift to cnc-core or mirror in RA — decision: extract to cnc-core/src/house_ai.rs, consumed by both engines); one commander per house (requires todo-29 multi-house); RA TeamTypes spawn teams like todo 34. Corrects PARITY.md §3.3 over-claim. Closes AUDIT §3.2 RA half, §5 AI row.
  Parallelization: Wave 3 (C4 chain) | Blocked by: 34,29,9 (RA TeamType data) | Blocks: 36,38 | Parallel with: 37
  References: RUST/cnc-tiberian-dawn/src/ai.rs:311–452 (urgency ladder, 10 tests); RUST/cnc-red-alert/src/game.rs:1787–1830 (hard-coded cadence to replace) and AI/house init near :676 (anchor re-verified at todo 1); AUDIT §3.2, §5; Metis finding 3.
  Acceptance criteria (agent-executable): 2v2 skirmish: each AI house builds/economizes/attacks independently (`multi_ai_houses_build_independently`); RA team executes mission from TeamType.
  QA scenarios: happy — four AI houses each reach wave cadence with distinct queues; failure — defeated AI house → commander retires cleanly, no panic, Evidence .omo/evidence/task-35-parity-100.log
  Commit: Y | feat(ra): expert AI per house + teams
- [ ] 36. IQ + difficulty + special flags
  What to do / Must NOT do: HouseClass IQ scale gating behaviors (do-they-understand-harvest/repair etc.); RA difficulty consumed (requires 25); game-speed + Special flags (build threshold, shroud-off, one-tick strikes) wired through to both engines (shroud-off wiring lands here; its acceptance test lives in todo 38, which owns the fog layer); GDI/NOD/Soviet/Allied BuildChoice doctrine tables replacing generic weights.
  Parallelization: Wave 3 (C4 chain tail) | Blocked by: 35,25 | Blocks: 43 | Parallel with: 37
  References: RUST/cnc-core/src/rules.rs:214 (DifficultyRule); TIBERIANDAWN/HOUSE.CPP (IQ + BuildChoice); AUDIT §3.10, §4/§5 AI rows.
  Acceptance criteria (agent-executable): easy AI never builds above tech threshold (`iq_gates_advanced_behavior`); special flags (build threshold, one-tick strikes) change behavior in test game. (Shroud-off acceptance lives in todo 38.)
  QA scenarios: happy — hard vs easy AI produce measurably different build orders; failure — unknown difficulty key → default + warning, Evidence .omo/evidence/task-36-parity-100.log
  Commit: Y | feat: IQ, difficulty, special flags
- [x] 37. Shroud primitive (C5)
  What to do / Must NOT do: New `cnc-core/src/visibility.rs`: per-house shroud/fog grid (unsighted/shrouded/revealed states), reveal-on-sight-range integration seams into both engines' maps, snapshot-serialized. TD = permanent reveal on sight; RA = fog re-covers (per-engine flag). FILE OWNERSHIP: this lane owns visibility.rs + map seams ONLY (Metis finding 9) — ai.rs/house.rs belong to the C4 lane this wave.
  Parallelization: Wave 3 (C5 chain head) | Blocked by: 1 | Blocks: 38,40 | Parallel with: 32–36 (C4 chain)
  References: REDALERT/CELL.CPP (per-side shroud semantics); RUST/cnc-red-alert/src/map.rs, RUST/cnc-tiberian-dawn/src/map.rs (seams); AUDIT §3.3 (zero shroud — grep-verified); Metis findings 7,9.
  Acceptance criteria (agent-executable): moving scout reveals sight circle (`scout_reveals_circle`); RA fog re-covers after delay, TD stays revealed; snapshot round-trip preserves grid.
  QA scenarios (name the exact tool + invocation): happy — two identical games with fog produce identical CRCs; failure — sight range OOB → clamped, no panic, Evidence .omo/evidence/task-37-parity-100.log
  Commit: Y | feat(core): per-house visibility layer
- [ ] 38. Shroud consumers + GPS + paradrops
  What to do / Must NOT do: Targeting/AI consult shroud (negative test: fully shrouded enemy NOT targeted); GPS superweapon wired into Game::superweapons + one-shot mass reveal (struct exists, superweapon.rs:283, never instantiated — AUDIT §8.6 wire-or-wire decision: wire); gap generator shrouds its area (gapgen consulted by targeting); radar low-power → minimap dead; paradrops via Badger/transport (needs todo 16). Also accepts the special-flags shroud-off test moved from todo 36 (`special_shroud_off_disables_fog`). Do NOT let this lane edit ai.rs/house.rs beyond targeting call sites (ownership: this todo is the C5 lane's consumer wiring, and it runs strictly after the C4 chain completes).
  Parallelization: Wave 3 (C5 chain tail, strictly after C4 chain 32–36) | Blocked by: 37,32,16,34,35,36 | Blocks: 40 | Parallel with: none
  References: RUST/cnc-red-alert/src/superweapon.rs:283 (gps_reveals_every_enemy_and_resets test exists); RUST/cnc-red-alert/src/gapgen.rs (shrouds() never consulted); TIBERIANDAWN/AIRCRAFT.CPP (Badger paradrop); TIBERIANDAWN/SPECIAL.CPP (special flags); AUDIT §3.3, §3.7, §5 specials.
  Acceptance criteria (agent-executable): `shrouded_enemy_not_targeted` (negative test); GPS charge → map reveal once; paradrop delivers team at waypoint; `special_shroud_off_disables_fog` (moved from todo 36).
  QA scenarios: happy — gap generator hides base from AI targeting; failure — GPS without full charge → no-op + EVA warning, Evidence .omo/evidence/task-38-parity-100.log
  Commit: Y | feat: vision consumers, GPS, paradrops
- [x] 39. EVA/VOX speech pipeline + CSF
  What to do / Must NOT do: Full CSF string-table parsing in cnc-asset (replace the 17-line stub that ignores `_data` — Metis finding 11); speech channel + queue in cnc-audio; EVA event strings ("Construction complete", "Unit ready", "Radar online", "Our base is under attack", objective deltas) with `Speak`/`Speak_AI` call sites in both engines (TIBERIANDAWN/AUDIO.CPP:33–42 pattern). Speech output routes through the existing mixer.
  Parallelization: Wave 4 (C6 chain head) | Blocked by: 1 | Blocks: 40 | Parallel with: editor chain, 45
  References: RUST/cnc-asset/src/string_table.rs (17-line stub — verified); RUST/cnc-audio/src/mixer.rs (channel/priority infrastructure); TIBERIANDAWN/AUDIO.CPP:33–42, 269–273, 532–535 (Speak/Speak_AI semantics); AUDIT §3.8 (VOX absent workspace-wide).
  Acceptance criteria (agent-executable): CSF fixture parses labels + values with player-prefixed filtering (`csf_parses_labels_and_values`); scripted build completion emits a queued speech event (`speak_enqueues_eva_line`).
  QA scenarios (name the exact tool + invocation): happy — three queued EVA lines play in order, later priority preempts; failure — missing label → fallback key logged, no panic, Evidence .omo/evidence/task-39-parity-100.log
  Commit: Y | feat: EVA speech pipeline + full CSF
- [ ] 40. Radar/minimap logic + sidebar depth
  What to do / Must NOT do: Engine-side radar/minimap state (shroud-filtered per todo 38, low-power kill switch), sidebar depth: radar tabs, power indicator state, production queue UI, Commando/Engineer rows (PORTING.md:116–119). Launch lane serialization: C6 todos run one-at-a-time over cnc-launcher/src/main.rs (Metis finding 9).
  Parallelization: Wave 4 (C6 chain: 39→40→44→41→42→43) | Blocked by: 39,38 | Blocks: 44 | Parallel with: editor chain, 45
  References: RUST/cnc-tiberian-dawn/PORTING.md:116–119; RUST/cnc-tiberian-dawn/src/game.rs:589–641 (sidebar options today); TIBERIANDAWN/SIDEBAR.CPP; AUDIT §3.8, §4 sidebar row (~10%).
  Acceptance criteria (agent-executable): minimap reflects fog state (`minimap_reflects_shroud`); power-out blanks radar; all buildables incl. commando/engineer listed when prerequisites met.
  QA scenarios: happy — building radar structure enables minimap; failure — radar destroyed → minimap static + EVA offline, Evidence .omo/evidence/task-40-parity-100.log
  Commit: Y | feat(launcher): radar and sidebar depth
- [ ] 41. RA score + briefing/ending flow
  What to do / Must NOT do: RA score screen path (main.rs:2612 wires td_score only, `ra_score` grep = 0); RA briefing screen (briefing image infra exists, scenario.rs:48) + end-of-mission → score → next mission flow for both engines; stats feed from todo 44.
  Parallelization: Wave 4 (C6 chain position 4) | Blocked by: 44,38 | Blocks: 42 | Parallel with: editor chain, 45
  References: RUST/cnc-launcher/src/score.rs (ScoreReport, td_score); RUST/cnc-launcher/src/main.rs:2612; RUST/cnc-red-alert/src/scenario.rs:48; AUDIT §3.8, §5 UI row.
  Acceptance criteria (agent-executable): RA mission end renders animated score (`ra_mission_end_shows_score`); briefing renders before first mission.
  QA scenarios: happy — scripted RA mission completes through briefing→mission→score→next; failure — missing stats → zeros displayed, no panic, Evidence .omo/evidence/task-41-parity-100.log
  Commit: Y | feat(launcher): RA score and briefing flow
- [ ] 42. In-game SFX + theme queue
  What to do / Must NOT do: Engine sound events (fire/explosion/select/move/build-complete) routed to the launcher AudioMixer (menu-only today — main.rs:8224); ThemeClass scan/play/next tied to house state (TIBERIANDAWN/THEME.CPP) using the existing MusicPlayer explore/combat layers.
  Parallelization: Wave 4 (C6 chain position 5) | Blocked by: 41 | Blocks: 43 | Parallel with: editor chain, 45
  References: RUST/cnc-audio/src/{mixer.rs,music.rs}; RUST/cnc-sfx (procedural effects); TIBERIANDAWN/THEME.CPP; AUDIT §3.8, §7 (SFX unwired).
  Acceptance criteria (agent-executable): scripted combat emits ≥4 distinct sfx through mixer (`combat_emits_sfx`); base-attack event switches theme layer.
  QA scenarios: happy — sound events audible in headless recording mode (cnc-audio test infra); failure — audio device absent → silent fallback, sim unaffected, Evidence .omo/evidence/task-42-parity-100.log
  Commit: Y | feat(launcher): in-game audio wiring
- [ ] 43. MP + options dialogs
  What to do / Must NOT do: NETDLG/MPLAYER equivalent: multiplayer setup via net-directory lobby (todo 3) launching LiveSession; in-game options (GOPTIONS): game speed, volume, special toggles wired to engine state (uses todo 36 special flags). Persist settings.
  Parallelization: Wave 4 (C6 chain tail) | Blocked by: 42,3,36 | Blocks: 53 | Parallel with: editor chain, 45
  References: TIBERIANDAWN/GOPTIONS.CPP, NETDLG.CPP; RUST/cnc-launcher/src/options.rs (mixer options exist); AUDIT §3.8, §5 networking row.
  Acceptance criteria (agent-executable): options persist across restart and apply to a live game (`options_apply_to_running_game`); lobby selection boots a 2-player LiveSession.
  QA scenarios: happy — two clients join via lobby and lockstep; failure — invalid config → defaults restored, Evidence .omo/evidence/task-43-parity-100.log
  Commit: Y | feat(launcher): multiplayer and options dialogs
- [ ] 44. Mission stats collection
  What to do / Must NOT do: STATS.CPP-style counters in both engines (units built/lost/killed, structures built/lost, credits spent/earned, mission time) published through the EventBus (todo 4) and snapshot-serialized, feeding score.rs. No UI here (todo 41 consumes).
  Parallelization: Wave 4 (C6 chain position 3) | Blocked by: 40,4 | Blocks: 41 | Parallel with: editor chain, 45
  References: TIBERIANDAWN/STATS.CPP (stat semantics); RUST/cnc-launcher/src/score.rs; AUDIT §4 score row, §5 UI row.
  Acceptance criteria (agent-executable): scripted mission produces full stat report (`mission_stats_complete`); counters survive snapshot round-trip.
  QA scenarios: happy — every counter increments exactly once per event; failure — dropped event → counter unchanged, never corrupt, Evidence .omo/evidence/task-44-parity-100.log
  Commit: Y | feat: mission stats pipeline
- [x] 45. VQA compressed-chunk + VXL voxel decoders
  What to do / Must NOT do: CBPZ/CBFZ/VQFR/SND1/SND2 decoders in `cnc-asset/src/video.rs` building on the existing format80 codec (video.rs:182 currently refuses these variants); VXL voxel parsing + conversion to sprite-atlas frames (audit §7 VXL row); synthetic compressed fixture test + env-gated real-file golden test (CNC_ORIGINAL_DATA_DIR). Do NOT implement Adlib (divergence).
  Parallelization: Wave 4 (C7) | Blocked by: 7 | Blocks: 46 | Parallel with: editor chain, C6 chain
  References: RUST/cnc-asset/src/video.rs:182,369 ("compressed_variants_report_the_missing_codec"); RUST/cnc-asset/src/format80.rs; RUST/docs/PARITY.md §3.1 (remaining true gap); AUDIT §7 (VXL voxels unsupported).
  Acceptance criteria (agent-executable): `video_decodes_compressed_variants` green on synthetic fixture; `vxl_parses_and_converts_to_atlas` green; env-gated real VQA decodes frames+audio when provided.
  QA scenarios: happy — synthetic VQA decodes to expected frame hashes; failure — unknown chunk id → Err naming the chunk, no panic, Evidence .omo/evidence/task-45-parity-100.log
  Commit: Y | feat(asset): VQA compressed codecs
- [ ] 46. Briefing/intro/ending playback screens
  What to do / Must NOT do: Launcher playback screen (MCI equivalent) wired into campaign flow for both engines: intro → briefing → mission → score → ending; frames to texture via existing wgpu path, audio via rodio; falls back to cnc-procgen briefing art when no movie file. Runs AFTER C6 todos 39–44 (main.rs ownership serialization, Metis finding 9).
  Parallelization: Wave 4 (C7) | Blocked by: 45,39–44 | Blocks: 53 | Parallel with: editor chain
  References: TIBERIANDAWN/INTRO.CPP, ENDING.CPP (movie flow); RUST/cnc-procgen/src/briefing.rs (fallback art); RUST/cnc-launcher/src/main.rs; AUDIT §5 scenario row, §7.
  Acceptance criteria (agent-executable): scripted campaign plays a synthetic .VQA end-to-end (`campaign_plays_movie_flow`); missing movie → procgen fallback without error dialog.
  QA scenarios: happy — frame pacing deterministic, audio synced within a frame; failure — decode error mid-movie → skip to briefing with log, Evidence .omo/evidence/task-46-parity-100.log
  Commit: Y | feat(launcher): movie playback flow
- [x] 47. Waypoints tool + [Waypoints] I/O
  What to do / Must NOT do: Waypoint placement/erase tool with letter assignment (A–…), [Waypoints] INI section read/write matching RA plugin layout; undo/redo-safe via existing History commands. Loaders (todos 8/9) already parse the section.
  Parallelization: Wave 4 (C9 serial chain head) | Blocked by: 1 | Blocks: 48 | Parallel with: C6 chain, 45
  References: CnCTDRAMapEditor/Tools/WaypointsTool.cs; CnCTDRAMapEditor/RedAlert/GamePlugin.cs:1459 ([Waypoints] layout); AUDIT §6 (waypoints MISSING).
  Acceptance criteria (agent-executable): `waypoint_roundtrips_ini` passes; undo removes waypoint cleanly; duplicate letter rejected.
  QA scenarios (name the exact tool + invocation): happy — place A/B/C, save, reload identical; failure — duplicate letter → validation error, Evidence .omo/evidence/task-47-parity-100.log
  Commit: Y | feat(editor): waypoints tool
- [ ] 48. TeamTypes editor tab
  What to do / Must NOT do: Full TeamTypes editing dialog + [TeamTypes]/[TaskForces] INI sections (TeamTypesDialog.cs 504L, Model/TeamType.cs analog) using todo-32 enums for missions/params; undo-safe command pattern. Do NOT invent team behavior — engine consumes these in todos 34/35.
  Parallelization: Wave 4 (C9 serial chain) | Blocked by: 47,32 | Blocks: 49 | Parallel with: C6 chain, 45
  References: CnCTDRAMapEditor/Dialogs/TeamTypesDialog.cs; CnCTDRAMapEditor/Model/TeamType.cs; AUDIT §6 (TeamTypes tab MISSING).
  Acceptance criteria (agent-executable): authored TeamType survives save/reload section-equal (`teamtype_roundtrips`); every edit is undoable in one step.
  QA scenarios: happy — create team with taskforce, save, reload; failure — invalid house reference → validation error blocks save, Evidence .omo/evidence/task-48-parity-100.log
  Commit: Y | feat(editor): TeamTypes tab
- [ ] 49. Footprints + infantry groups + property dialogs
  What to do / Must NOT do: Multi-cell building placement with foundation/bib occupancy; 5-slot infantry groups per cell (mirrors todo 15 engine semantics); object property dialogs (facing, health, sub-cell position) per ObjectProperties.cs.
  Parallelization: Wave 4 (C9 serial chain) | Blocked by: 48 | Blocks: 50 | Parallel with: C6 chain, 45
  References: CnCTDRAMapEditor/Model/Building.cs, Model/Infantry.cs:98 (InfantryGroup); CnCTDRAMapEditor/Controls/ObjectProperties.cs; AUDIT §6 (objects-model row, ~30%).
  Acceptance criteria (agent-executable): 2x2 building occupies 4 cells with bib (`footprint_occupies_cells`); 5 infantry per cell; property edit is one undo step.
  QA scenarios: happy — place conyard, overlap rejected deterministically; failure — overlapping placement → rejected with reason, Evidence .omo/evidence/task-49-parity-100.log
  Commit: Y | feat(editor): building footprints + infantry groups
- [ ] 50. INI-driven type registries
  What to do / Must NOT do: Load editor catalogs from rules.ini/art.ini via cnc-core RulesDatabase (replaces hardcoded 14–21 ClassId vectors, plugin.rs:95–166); names/prerequisites/costs from data; trigger dialog event/action pickers from todo-32 enums. Unknown type entries warn + skip (never fail the load).
  Parallelization: Wave 4 (C9 serial chain) | Blocked by: 49,25 | Blocks: 51 | Parallel with: C6 chain, 45
  References: RUST/cnc-core/src/rules.rs (RulesDatabase public API); RUST/cnc-map-editor/src/plugin.rs:95–166 (hardcoded vectors); CnCTDRAMapEditor/TiberianDawn/BuildingTypes.cs (registry pattern); AUDIT §6 (type-registries row, ~15%).
  Acceptance criteria (agent-executable): catalog size derived from fixture rules sections (`catalog_from_rules_matches_fixture`); prerequisite chains render in placement validation.
  QA scenarios: happy — fixture rules yield expected per-class catalog counts; failure — malformed rules section → partial catalog + error list, Evidence .omo/evidence/task-50-parity-100.log
  Commit: Y | feat(editor): rules-driven type registries
- [x] 51. Tools completion + MIX tilesets + team colors
  What to do / Must NOT do: Port remaining brushes: TemplateTool (base-terrain ribbon painting), smudge layer, walls brush, resources brush with brush size (TemplateTool.cs 1008L, SmudgeTool.cs 337L, WallsTool.cs 384L, ResourcesTool.cs:364); theater tilesets loaded from MIX archives via cnc-asset archive.rs (parser exists, unwired from editor); team colors via palette PlayerRemap in rendering; renderer icon-cache + theater-blit equivalents of GBUFFER/ICONSET (audit §7 renderer sub-items) in cnc-renderer and the editor tileset path.
  Parallelization: Wave 4 (C9 serial chain) | Blocked by: 50 | Blocks: 52 | Parallel with: C6 chain, 45
  References: RUST/cnc-asset/src/archive.rs:25 (MixArchive); RUST/cnc-core/src/palette.rs (PlayerRemap); CnCTDRAMapEditor/Utility/TeamColor.cs; AUDIT §6 (tools/theaters/palette rows) + §7 (renderer icon-cache/theater-blit sub-items).
  Acceptance criteria (agent-executable): each brush paints + undoes in one step; tileset loads from fixture MIX (`tileset_loads_from_mix`); player-remap colors render distinctly.
  QA scenarios: happy — ore brush sizes 1/2/3 paint proportional cells; failure — missing theater tileset → fallback family + warning, Evidence .omo/evidence/task-51-parity-100.log
  Commit: Y | feat(editor): full toolset + mix tilesets + team colors
- [ ] 52. MEG/PGM export + WWCompression encode + validation + trigger dialog
  What to do / Must NOT do: .MEG/.BIN/.PGM export (Megafile/MegafileBuilder analog), WWCompression encode for MapPack/OverlayPack write-back (todo 7 provides LCW; add PKLITE container shape only if required by format); rule-aware map validation (required structures/houses, trigger wiring) using todo-50 registries; full trigger dialog with event/action pickers + cell-binding tool (select cell → assign trigger).
  Parallelization: Wave 4 (C9 serial chain tail) | Blocked by: 51,7 (LCW write-back) | Blocks: 53 | Parallel with: C6 chain, 45
  References: CnCTDRAMapEditor/Utility/{Megafile.cs,MegafileBuilder.cs,WWCompression.cs}; CnCTDRAMapEditor/Dialogs/TriggersDialog.cs (510L); AUDIT §6 (export/validation/trigger rows).
  Acceptance criteria (agent-executable): exported .MEG read back by own Megafile reader (`meg_roundtrips`); saved .MPR passes rule validation; cell-bound trigger fires in engine test (`editor_cell_trigger_fires_in_game`).
  QA scenarios: happy — full authoring loop: place, bind trigger, save MPR, engine loads and trigger fires; failure — validation failure blocks save with error list, Evidence .omo/evidence/task-52-parity-100.log
  Commit: Y | feat(editor): export, validation, trigger dialog
- [ ] 53. Final parity re-audit (finite, evidence-bound)
  What to do / Must NOT do: Re-run the 4-lane audit methodology; acceptance = EVERY actionable gap row/sub-item across PARITY_AUDIT §1–§8 re-verified with live anchors (PASS + evidence path, or recorded intentional divergence per Scope OUT — incl. cnc-viewer spectator lane and cnc-core::pointers documented divergences). F-map (`ORIGINAL_FUNCTIONALITY_MAP.md`) used for spot-check only — NOT an unbounded target (Metis finding 12). Update PARITY.md + audit doc; any open row becomes an appended fix todo BEFORE the F-wave starts. No new features beyond audit rows.
  Parallelization: Wave 5 | Blocked by: all above | Blocks: F1–F4 | Parallel with: none
  References: RUST/docs/PARITY_AUDIT_2026-10-20.md §1–§8 (row inventory); ORIGINAL_FUNCTIONALITY_MAP.md (spot-check); Metis finding 12; review round 1 both-lane findings.
  Acceptance criteria (agent-executable): re-audit document shows 0 open rows (each row: PASS+evidence or divergence-ref); `cd RUST && cargo fmt --check && cargo clippy --workspace --all-targets && cargo test --workspace` all green.
  QA scenarios (name the exact tool + invocation): happy — every gap row has PASS evidence; failure — any open row → appended fix todo executed + re-verified before F-wave, Evidence .omo/evidence/task-53-parity-100.log
  Commit: Y | docs: final parity re-audit

- [ ] 54. Make cnc-console + cnc-events build (pre-existing debt)
  What to do / Must NOT do: Bring the two orphan crates into the workspace and make them compile. `cnc-console`: its `src` module set (`app`, `events_panel`, `filters`, `log_panel`, `settings`, `styles`, `widgets`) existed only as untracked `*.rs.bak` drafts and was destroyed during todo 1 — **never tracked in git, unrecoverable** (no `/home` btrfs snapshots; absent from the local backup tarball); the surviving `src/main.rs` was a non-compiling winit 0.30 / egui 0.29 draft with its 19-line import header pasted 4×. Re-author the modules from the crate README + `Cargo.toml` surface. `cnc-events`: fix its 21 compile errors (unescaped `{`/`}` in `streaming.rs` `format!` JSON strings at :154,:161,:168,:171; `SmallVec` misuse in `definitions.rs` :60,:72,:103; missing `unsubscribe` on `Arc<EventBusInner>` at `bus.rs` :84,:92; `Box<dyn Event>` lacks `Clone` at `bus.rs` :196,:218,:347,:352; `dyn Fn`/`dyn Event` bounds in `handler.rs` :17,:112,:124; `dyn Event: serde::Serialize` at `replay.rs` :93,:347; missing `EventBusError::AlreadyInitialized` at `lib.rs` :44). Then add BOTH to `RUST/Cargo.toml` members. Do NOT add an empty `[workspace]` table to either crate (that silently excludes them from fmt/clippy/test). Do NOT re-land the lost drafts as if they were recovered — write real, tested code.
  Parallelization: Wave 1 | Blocked by: 3,4,5 | Blocks: none | Parallel with: 6,7
  References (executor has NO interview context - be exhaustive): .omo/evidence/baseline-report.md §2 (incident record + exact error inventory); RUST/docs/PARITY_AUDIT_2026-10-20.md Appendix A (`| cnc-console | 1 | — | **uncompilable** |`); RUST/cnc-console/{README.md,Cargo.toml}; RUST/cnc-console/src/main.rs; RUST/cnc-events/src/{streaming,definitions,bus,handler,replay,lib}.rs; RUST/cnc-logs (the sibling crate that DOES build, as the working pattern).
  Acceptance criteria (agent-executable): `cargo check -p cnc-console` exits 0 with the crate in `RUST/Cargo.toml` members; `cargo check -p cnc-events` exits 0 with the crate in `RUST/Cargo.toml` members; `cargo metadata --no-deps` reports 18 packages; `cargo test --workspace` includes both crates and their tests pass.
  QA scenarios (name the exact tool + invocation): happy — both `cargo check -p` invocations exit 0 and `cargo metadata --no-deps --format-version 1` lists 18 packages; failure — either crate still fails to compile → keep it OUT of members and record the exact errors, Evidence .omo/evidence/task-54-parity-100.log
  Commit: Y | feat(console): working console crate

## Final verification wave
> Runs in parallel after ALL todos. ALL must APPROVE. Surface results and wait for the user's explicit okay before declaring complete.
- [ ] F1. Plan compliance audit
  Verify: every todo row in `## Todos` matches `- [ ] N.` grammar at column zero with References + Acceptance + happy/failure QA + Commit; dependency matrix consistent with Parallelization lines; TL;DR present as first `## ` heading. Evidence .omo/evidence/f1-compliance.log
- [ ] F2. Code quality review
  Verify: `cd RUST && cargo fmt --check && cargo clippy --workspace --all-targets` clean; spot-review 6 high-risk diffs (shroud grid, RNG policy, LCW codec, apply_command arms, visibility seams, save version bumps) for no unwrap/panic in sim paths and no weakened tests. Evidence .omo/evidence/f2-quality.log
- [ ] F3. Real manual QA (agent-executed)
  Verify: launch the built client; play one TD original-campaign mission end-to-end (load → briefing → combat → score → carry-over) and one RA mission with fog + Tesla defense active; two-client lockstep smoke over LAN with directory lobby. Headless fallback: `cargo run -p cnc-launcher -- --headless` screenshot + replay CRC equality. Evidence .omo/evidence/f3-manual-qa.log
- [ ] F4. Scope fidelity
  Verify: re-audit doc shows every PARITY_AUDIT §1–§8 row/sub-item PASS-or-divergence with zero open rows; divergence entries (incl. cnc-viewer, cnc-core::pointers) carry rationale; no Scope-OUT item implemented (grep Steam/Adlib/IPX/SAV loaders absent); dirty-tree ownership respected (todo-1 per-owner commits reconciled, no feature mixing). Evidence .omo/evidence/f4-scope.log

## Commit strategy
One commit per todo, conventional messages (`type(scope): summary` per each todo's Commit line), local branch only — NO pushes without explicit user authorization (pre-push hook runs the full CI pipeline locally on any push). Todo 1 LANDS the dirty tree as owner-labeled per-feature commits (nothing pushed, no mixing). Todo 6 + todo 53 are doc-only commits. Save/replay format changes bump SAVE_VERSION/REPLAY_VERSION in the same commit as the format change (never split). Branch hygiene: wave boundaries may batch as merge commits only if per-todo commits are preserved.

## Success criteria
1. `cd RUST && cargo fmt --check && cargo clippy --workspace --all-targets && cargo test --workspace` all green at plan end, with every wave-gate run recorded in evidence logs.
2. Todo 53's re-audit: 0 open rows across PARITY_AUDIT §1–§8 — every gap closed with live-anchor evidence or explicitly recorded as user-approved divergence (Steam, Adlib/IMF, IPX/DDE, original .SAV, GlyphX, cnc-viewer spectator lane, pointer-fixup saves).
3. Baseline test count (todo 1) strictly increased; zero pre-existing tests weakened or deleted.
4. F1–F4 all APPROVE with evidence artifacts under `.omo/evidence/`.
5. Parity docs (`PARITY.md`, `PARITY_AUDIT_2026-10-20.md`, per-crate codemaps) reconciled and consistent with the final tree.
