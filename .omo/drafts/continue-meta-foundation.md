---
slug: continue-meta-foundation
status: awaiting-approval
intent: unclear
review_required: true
plan_path: .omo/plans/continue-meta-foundation.md
plan_sha256: null
review_round_id: null
pending-action: write and review .omo/plans/continue-meta-foundation.md
review:
  momus:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-meta-foundation.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
  independent:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-meta-foundation.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
approach: Build the cnc-meta leaf crate (PlayerProfile JSON persistence, match-end telemetry extraction from GameSnapshot) per PROGRESSION.md P1 as the shared foundation for the progression/techtree/crafting systems. One decision-complete plan.
---

# Draft: continue-meta-foundation

## Components (topology ledger)
- meta-crate | cnc-meta workspace member leaf crate | active | PROGRESSION.md P1
- profile-json | PlayerProfile persistence at platform config dir | active | PROGRESSION.md System 1
- telemetry-extract | extract GameSnapshot fields at match end into profile stats | active | PROGRESSION.md System 1

## Open assumptions (announced defaults)
- New leaf crate depending only on serde/thiserror | default | PROGRESSION.md explicitly specifies | reversible
- JSON at platform config dir, same lifecycle as net-directory ladder | default | PROGRESSION.md specifies | reversible
- PLAYER_NAME_MAX=32 contract from matchmaking.rs | default | named contract | reversible

## Findings (cited - path:lines)
- No cnc-meta crate exists; zero cross-session persistence outside saves (PROGRESSION.md System 1)
- GameSnapshot already records telemetry fields (savegame.rs:44-64) => zero new instrumentation needed
- Workspace has 18 members; new crate must be added with SemVer + README/CHANGELOG/codemap (AGENTS)

## Decisions (with rationale)
- This plan is the dependency root for continue-progression/techtree/crafting
- review_required=true (UNCLEAR, non-trivial)

## Scope IN
- cnc-meta crate, PlayerProfile model, JSON persistence, match-end telemetry extraction

## Scope OUT (Must NOT have)
- No XP/levels/stars/streaks/buffs yet (those are continue-progression); no in-game instrumentation changes; no rules mutation

## Open questions
- None blocking

## Approval gate
status: awaiting-approval
Approach above. Next: on explicit user approval, rerun scaffold without --draft-only, run Metis, append todos, fill TL;DR, dual high-accuracy review automatically.
