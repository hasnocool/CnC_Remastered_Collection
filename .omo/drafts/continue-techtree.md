---
slug: continue-techtree
status: awaiting-approval
intent: unclear
review_required: true
plan_path: .omo/plans/continue-techtree.md
plan_sha256: null
review_round_id: null
pending-action: write and review .omo/plans/continue-techtree.md
review:
  momus:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-techtree.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
  independent:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-techtree.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
approach: Implement TECHTREE.md: doctrine web (static JSON tree, RuleTweak scoped application), in-match Command::Research, capstones/prestige loadouts, ranked pinning + fingerprint handshake. Depends on continue-meta-foundation (+ RuleTweak table). One decision-complete plan.
---

# Draft: continue-techtree

## Components (topology ledger)
- tree-schema | doctrine tree JSON schema + loader | active | TECHTREE.md Layer 1
- rule-tweaks | scoped RuleTweak application + determinism tests | active | TECHTREE.md Layer 1/2
- web-ui | doctrine web screen + point granting | active | TECHTREE.md Layer 1
- research-command | Command::Research + tech buildings + choose-1-of-3 | active | TECHTREE.md Layer 2
- capstones | capstones/prestige loadouts + pre-match display | active | TECHTREE.md Layer 3
- ranked-pinning | stock ruleset + empty tree in ranked + handshake | active | TECHTREE.md ranked integrity

## Open assumptions (announced defaults)
- Static JSON tree definitions loaded like INI/JSON assets | default | TECHTREE.md data format | reversible
- Every node effect is a code-reviewed RuleTweak with a determinism test | default | TECHTREE.md integrity | reversible
- RuleTweak table shared with PROGRESSION buffs (continue-progression) | default | companion docs | reversible

## Findings (cited - path:lines)
- TECHTREE.md is a full design proposal, never implemented
- Command::Sabotage proves the lockstep-safe mid-match mutation pattern (network.rs:190-209)
- RulesDatabase mutation points exist (cnc-core/src/rules.rs); handshake surface in NetworkConfig

## Decisions (with rationale)
- Depends on continue-meta-foundation and the shared RuleTweak table (continue-progression supplies it; if not landed first, TECHTREE defines its own table)
- review_required=true (UNCLEAR, non-trivial)

## Scope IN
- Tree schema, loader, RuleTweak application, web UI, research command, capstones, ranked pinning

## Scope OUT (Must NOT have)
- No pay-for-points; no tree effects in ranked; no random node rolls; no mid-match tree editing; no community trees yet

## Open questions
- None blocking

## Approval gate
status: awaiting-approval
Approach above. Next: on explicit user approval, rerun scaffold without --draft-only, run Metis, append todos, fill TL;DR, dual high-accuracy review automatically.
