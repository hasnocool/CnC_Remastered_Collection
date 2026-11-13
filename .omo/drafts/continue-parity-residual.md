---
slug: continue-parity-residual
status: awaiting-approval
intent: unclear
review_required: true
plan_path: .omo/plans/continue-parity-residual.md
plan_sha256: null
review_round_id: null
pending-action: write and review .omo/plans/continue-parity-residual.md
review:
  momus:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-parity-residual.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
  independent:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-parity-residual.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
approach: Close the audit's remaining engine gaps: TD [Houses]/[CellTriggers] consumption completion, RA [Houses]/[Base] object placement, editor original-format import UI, EVA Speak() call sites (TD 12/45, RA 0), TD planner stubs (ai.rs:133-147), RA projectile/flight-time polish, RA 2v2 multi-house AI, original-format fixtures in-tree, plus wiring cnc-events/cnc-logs consumers. One decision-complete plan.
---

# Draft: continue-parity-residual

## Components (topology ledger)
- engine-scenario-io | TD [Houses]/[CellTriggers] + RA [Houses]/[Base] placement | active | PARITY_AUDIT_2026-10-24-GAPS.md §1 rows 2-4
- editor-import-ui | original-format import UI in cnc-map-editor | active | PARITY_AUDIT_2026-10-20.md §6
- eva-callsites | Speak() call sites both engines | active | PARITY.md §2.9, audit §3.7
- ai-planner | TD planner stubs + RA multi-house AI | active | PARITY.md §2.5
- combat-fidelity | RA projectiles/flight-time/miss-rolls/splash | active | audit §3.4
- fixtures | original-format golden fixtures in-tree | active | PARITY.md §2.6

## Open assumptions (announced defaults)
- Build on the current 18-member workspace; do not restructure crate boundaries | default | matches proven parity-100/gaps pattern | reversible
- Original-format fixtures are synthetic license-clean in-repo, env-gated for real data | default | matches todo-11 precedent | reversible
- Commit discipline: per-todo owner-labeled commits, nothing pushed without Forgejo CI | default | AGENTS policy | reversible

## Findings (cited - path:lines)
- Parity-100 (53 todos) and parity-gaps (13 todos) complete; 1746/0 tests (PARITY_AUDIT_2026-10-20.md §1, Appendix A)
- cnc-console restored + compiles; cnc-replayer rejoined + compiles (cargo check, 2026-11-05)
- Dirty worktree = tiletools Python lane only (untracked desert_refs/, extracted_tiles/, generated_tiles/, M tiletools/)
- Remaining gaps enumerated in RUST/docs/PARITY_AUDIT_2026-10-24-GAPS.md §1 and PARITY.md §3

## Decisions (with rationale)
- Target the evidence-listed gaps only; the three gameplay proposals (PROGRESSION/TECHTREE/CRAFTING) are separate plans
- review_required=true (UNCLEAR, non-trivial)

## Scope IN
- Engine scenario I/O completion, editor import UI, EVA call sites, AI planner, RA combat fidelity, fixtures

## Scope OUT (Must NOT have)
- No edits to original C++/C# source; no Adlib/IMF; no original .SAV compat; no Steam Workshop; no gameplay-system proposals; no tiletools work

## Open questions
- None blocking; plan proceeds on announced defaults

## Approval gate
status: awaiting-approval
Approach above. Next: on explicit user approval, rerun scaffold without --draft-only to create the plan, run Metis, append todos, fill TL;DR, then run dual high-accuracy review (momus + oracle) automatically.
