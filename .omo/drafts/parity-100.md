---
slug: parity-100
status: review_round_initialized
intent: clear
review_required: true
plan_path: .omo/plans/parity-100.md
plan_sha256: 34dcb9f25c207849859dbad67993a7450d85764f7524e17f0705fbe3da333a6b
review_round_id: rr4-parity100-20261020
round_status: active
pending-action: review .omo/plans/parity-100.md
approach: Wave-based parity closure across 10 components (content I/O first, then engine depth, AI/triggers, shroud, presentation, video, editor, infra hygiene), every todo pinned to a gap row of the 2026-10-20 parity audit with parity-fixture tests; final wave re-runs the full parity audit to verify 100% of listed gaps closed.
---

# Draft: parity-100

## Components (topology ledger)
<!-- id | outcome (one line) | status: active|deferred | evidence path -->

| id | outcome | status | evidence path |
|---|---|---|---|
| C1 | Original-format content I/O: SCENARIO.INI/MPR/ini+bin loaders + LCW/Format80 codecs + editor open/save of original maps | active | RUST/docs/PARITY_AUDIT_2026-10-20.md §3.1 |
| C2 | TD engine depth: roster to 100%, buildings-fire, transports/passengers, apply_command arms, game-state CRC, FloatBias, credit tick | active | PARITY_AUDIT §4, exp-1 |
| C3 | RA engine depth: verses/armor combat, Rules wiring, air/naval consolidation, specials (crates/vortex/nuke/GPS/dogs/medics/Tanya), engineer capture, France bug | active | PARITY_AUDIT §5, exp-2 |
| C4 | TeamType/Team AI + trigger vocabulary to ~100% TEVENT/TACTION in both engines, superweapon trigger actions | active | PARITY_AUDIT §3.2, §3.7 |
| C5 | Shroud/fog-of-war per-house visibility layer consulted by targeting/AI + GPS/gapgen integration | active | PARITY_AUDIT §3.3 |
| C6 | Presentation: EVA/VOX speech pipeline, radar/minimap logic, sidebar depth, RA score screen, credits tick, MP dialogs/options | active | PARITY_AUDIT §3.8 |
| C7 | Video/VQ playback: format80 codec, compressed VQA variants, launcher briefing/ending screens | active | PARITY.md §3.1, PARITY_AUDIT §7 |
| C8 | Infra hygiene: wire net-directory+ready-up into launcher, fix cnc-console build, member-ize events/logs, fmt fix, stale codemap/doc corrections | active | PARITY_AUDIT §7, §8 |
| C9 | Map editor closure: waypoints, TeamTypes tab, footprints/infantry groups, original-format I/O, INI type registries, remaining 6 tools, smudge layer | active | PARITY_AUDIT §6, exp-3 |
| C10 | Cross-engine parity re-audit to 100%: rerun audit methodology against ORIGINAL_FUNCTIONALITY_MAP.md, close residual gaps | active | PARITY_AUDIT §3, ORIGINAL_FUNCTIONALITY_MAP.md |

## Open assumptions (announced defaults)
<!-- assumption | adopted default | rationale | reversible? -->

| assumption | adopted default | rationale | reversible? |
|---|---|---|---|
| "these gaps" = the gap list in RUST/docs/PARITY_AUDIT_2026-10-20.md (+ PARITY.md §3), not Appendix-B intentional divergences | OUT of scope | User's wording points at the identified gaps; repo docs mark divergences as deliberate | yes (fork surfaced as the one open question) |
| Parity standard | Functional parity of the C++ behavior as shipped in this repo (remastered source), verified by grep/read on both sides + determinism tests | Matches audit methodology; GlyphX-client-internal behavior has no source to port | yes |
| Test strategy | TDD on new subsystems, tests-after on fixes; every todo carries parity fixtures (tables pinned vs CONST.CPP/BDATA etc.) + deterministic sim tests; agent-executed QA | Matches existing crate patterns (td_rules tests pin C++ constants) | yes |
| Execution order | C8 hygiene first (cheap, compounding), then C1 (unlocks content), C2+C3 in parallel, C4→C5, C6/C7/C9 parallel, C10 last | Audit §9 roadmap | yes |

## Findings (cited - path:lines)

- Fresh 4-lane audit (2026-10-20): RUST/docs/PARITY_AUDIT_2026-10-20.md — TD ~40–45%, RA ~55% subset/~35% content, editor ~30–40%, 952 tests/0 failed.
- Prior snapshot (2026-10-19): RUST/docs/PARITY.md — over-claims corrected by audit: "AI ✅" (Expert_AI port drives TD only, `cnc-tiberian-dawn/src/ai.rs:311`), "combat ✅ largely" (RA = flat damage, `cnc-red-alert/src/game.rs:1852–2046`), "552 tests" (actual 952).
- Original-side inventory: ORIGINAL_FUNCTIONALITY_MAP.md (RTTI enums, class inventory, full .CPP lists).
- Biggest single gap: original scenario/map loaders absent in both engines + editor (PARITY_AUDIT §3.1).
- Tested-but-unreachable code to wire-or-delete: RA aircraft.rs/naval.rs structs, GPS superweapon, FloatBias, cnc-core::pointers, TeamType parsers (PARITY_AUDIT §3.2, §8.6).
- Known bug: France→Chronosphere duplicate (cnc-red-alert/src/house.rs:67–72).
- Known break: cnc-console lib.rs declares styles.rs/widgets.rs that do not exist (cnc-console/src/lib.rs:7–17).
- fmt failure: cnc-tiberian-dawn/src/powerup.rs:209 per BUILD_STATUS.md ledger.

## Decisions (with rationale)

- D1: Treat PARITY_AUDIT_2026-10-20.md as the canonical gap list; PARITY.md §3 items 1–6 that remain gaps fold in (VQA compressed codecs, carry-over launcher wiring, skirmish AI enable). Rationale: audit is grep-verified this session; PARITY.md has identified over-claims.
- D2: RESOLVED (user, 2026-10-20): keep intentional divergences OUT of scope (Steam, Adlib/IMF, IPX/DDE, original .SAV byte compatibility, GlyphX interop).
- D3: LCW codec = NEW work in cnc-asset (none exists; only a TODO reference in TD scenario_loader.rs:470–472). Format80 already exists (`cnc-asset/src/format80.rs`) — reuse, never reimplement. Blowfish/SHA stays out (MP-auth only). (Metis finding 5)
- D4: Plan waves sized for parallel fixer/designer lanes with deterministic sim tests as the gate; every wave ends with fmt+clippy+test gates run from RUST/.
- D5 (Metis 1,2,3): Working tree is dirty (192 entries) and the audit's baseline partially stale (France already fixed in house.rs; cnc-console compiles; workspace membership done; TD scenario_loader.rs 776L exists, unwired, LCW-bin TODOs at :459–461; fmt now fails elsewhere — last known classic_pack.rs:314,351). Plan todo 1 = baseline reconciliation + anchor refresh before any lane starts.
- D6 (Metis 6): RA seeded-RNG policy defined up front: per-frame deterministic rolls, seed traceable from replay header, RNG state included in game-state CRC and the two-identical-games gate, replay format version bump (pre-RNG replays explicitly invalidated).
- D7 (Metis 7): GPS superweapon + paradrops move to Wave 3 (after shroud primitive); multi-house precedes team wiring. TD loader = finish (not build).
- D8 (Metis 9,10): Per-wave file-ownership contracts: C4 owns ai.rs/house.rs, C5 owns cnc-core/src/visibility.rs + map seams; W4 launcher lanes serialized (C6 → C7) over main.rs; editor consumes cnc-asset codecs exclusively (no parallel codec implementations).

## Scope IN

All gaps enumerated in RUST/docs/PARITY_AUDIT_2026-10-20.md §3–§8 (each todo binds its audit row) and PARITY.md §3.1 remaining items (VQA compressed variants, carry-over launcher wiring), including the rows Metis found unowned: TD pathfinding edge masks/zones, RA 32-angle facing/turret rotation, RA per-factory queues/prereq/silo capacity, RA amphibious pathfinding, VXL voxel parsing, full CSF string-table, editor MEG/BIN/PGM + WWCompression encode, cnc-events/cnc-logs consumer wiring. Plus: baseline reconciliation, synthetic original-format golden fixtures (real game data env-gated, never committed).

## Scope OUT (Must NOT have)

- No Steam Workshop integration (intentional divergence, user-confirmed 2026-10-20).
- No Adlib/IMF hardware emulation (music stays MIDI-synth/procedural, user-confirmed).
- No IPX/null-modem/DDE/Westwood-Online protocol reimplementation.
- No byte-compatible original .SAV save loading (user-confirmed).
- No C++ engine source modifications (TIBERIANDAWN/REDALERT are read-only reference).
- No GlyphX-client ABI work (closed-source client, out of repo).
- No new gameplay features that the original engines don't have (beyond the Rust-native lanes already present, which are left intact).
- No committing of unrelated in-flight user work found in the dirty tree (inventory + preserve only).

## Open questions

- Q1 (owner-decision, the only surviving fork): RESOLVED 2026-10-20 — keep divergences divergent; scope = audit gap list.

## Approval gate
status: rr4_complete — DUAL APPROVED, live-digest validation PASSED (plan sha256 = 34dcb9f25c207849859dbad67993a7450d85764f7524e17f0705fbe3da333a6b matches both approvals); plan READY FOR EXECUTION (2026-10-20)
review: {
  "momus": { "status": "approved", "workspace_root": "/home/hyperion/Projects/Games/CnC_Remastered_Collection", "runtime_home": null, "target": ".omo/plans/parity-100.md", "round_id": "rr4-parity100-20261020", "plan_sha256": "34dcb9f25c207849859dbad67993a7450d85764f7524e17f0705fbe3da333a6b", "launch_id": "launch-momus-rr4-d5f6", "session": "ses_ea59fb8c0ffeMbvSr7OD1QW7mU", "result": "APPROVED — all 9 rev-4 fixes verified at line level; structural invariants intact; one conservative non-blocking residual (matrix rows 25–26 '—' Can-parallelize, strictly narrower than TD∥RA wave line)." },
  "independent": { "status": "approved", "workspace_root": "/home/hyperion/Projects/Games/CnC_Remastered_Collection", "runtime_home": null, "target": ".omo/plans/parity-100.md", "round_id": "rr4-parity100-20261020", "plan_sha256": "34dcb9f25c207849859dbad67993a7450d85764f7524e17f0705fbe3da333a6b", "launch_id": "launch-ind-rr4-d5f6", "session": "ses_ea59fb8bfffeotxMHA0VJuHpuA", "result": "APPROVED — conditions c1/c2/adv ✅ + minors m1–m6 ✅ all landed verbatim; structure intact (53+4, chains unchanged); 1 non-blocking pre-existing nit (todo-34 Blocks field '35' vs matrix '35,38' — graph enforced at todo 38; no action required this round, optional next doc pass)." }
}
validation: live canonical-path + SHA-256 check repeated immediately before handoff — MATCH (per review lifecycle contract; no drift, no fresh round needed).
review_history: Metis round 0 (ora-1 ses_ea610dfe3ffeVCxWhHRP1kE7uf, FIX-FIRST, 15 findings) → revision 1. rr1 (ora-2/ora-3) CHANGES_REQUESTED → revision 2. rr2 (ora-4/ora-5) CHANGES_REQUESTED → revision 3. rr3 (ora-6/ora-7) BOTH APPROVED + condition edits → revision 4. rr4 (ora-8/ora-9) BOTH APPROVED at final digest → READY.
next-action: execution belongs to a worker session — user runs $start-work parity-100. Planner never implements.
rr3_history: BOTH APPROVED at f8a4c875… (ora-6 ses_ea5aee050ffeLTAstLRZKxTtqx, ora-7 ses_ea5aee04fffecm5QE3DM5Snbka). Revision 4 = doc-only residuals both lanes enumerated: independent conditions (editor lanes→editor chain in todos 39–45 + matrix 39/45; W2 lanes→W2 chains todo 12/matrix 12; C8 cnc-console consumer wording) + advisory (todo 13 Blocked-by annotation); momus minors (todos 8/9 Blocks += 34,35; todo 35 Blocked-by += 9; matrix in-chain rows name cross-chain partners; scenario_loader :459–506; rules.rs:345 hedge; named test ids td_roster_per_unit_checks/ra_specials_each_deterministic). No structural changes.
history: Metis round 0 (ora-1) FIX-FIRST folded pre-plan. rr1 (ora-2/ora-3) CHANGES_REQUESTED → revision 2. rr2 (ora-4/ora-5) CHANGES_REQUESTED → revision 3. rr3 (ora-6/ora-7) BOTH APPROVED → revision 4 (doc-only) → rr4 line-diff round. User opted into dual high-accuracy review at delivery.
rr1_history: momus (ses_ea5e27a5cffeV8vHoNUPMeAvxA) CHANGES_REQUESTED 2B/7M/4m; independent (ses_ea5e27a5affec2ZjeaMVPo0g9f) CHANGES_REQUESTED 1B/4M/5m — ALL findings fixed in revision 2: re-audit scope §1–§8 + divergence routes (cnc-viewer, cnc-core::pointers), uncovered rows assigned (RA transports→26, MAD tank→30, dinos→14, per-house/Basic sections→12, VXL→45, replayer→31), Wave-2 serial per-crate chains, host-resource caps, todo-1 lands owner-labeled commits, 13 split TD/RA halves, 11 blocks 8/9, C6 total chain 39→40→44→41→42→43, editor lanes A/B, 34→35, 36-fog-acceptance→38, matrix regenerated, test-expectation carve-out, anchor drift corrected.
history: Metis round 0 (ora-1, ses_ea610dfe3ffeVCxWhHRP1kE7uf, FIX-FIRST, 15 findings) fully folded pre-plan. User opted into dual high-accuracy review at delivery.
