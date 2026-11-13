---
slug: continue-progression
status: awaiting-approval
intent: unclear
review_required: true
plan_path: .omo/plans/continue-progression.md
plan_sha256: null
review_round_id: null
pending-action: write and review .omo/plans/continue-progression.md
review:
  momus:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-progression.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
  independent:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-progression.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
approach: Implement PROGRESSION.md Systems 2-7: campaign stars+unlocks, daily rewards/streaks, in-match RuleTweak buffs (unranked only), XP/levels, social/AI hooks, feedback presentation. Depends on continue-meta-foundation. One decision-complete plan.
---

# Draft: continue-progression

## Components (topology ledger)
- campaign-stars | 1-3 star ratings per mission + unlocks | active | PROGRESSION.md System 2
- streaks-daily | daily rewards, streak logic, daily challenge seed | active | PROGRESSION.md System 3
- buffs | RuleTweak buff table + determinism tests | active | PROGRESSION.md System 4
- xp-levels | XP sources, level gating of cosmetics | active | PROGRESSION.md System 5
- social-ai | community AI ladder + co-op ally | active | PROGRESSION.md System 6
- feedback | match-end screen, streak rituals, audio signature | active | PROGRESSION.md System 7

## Open assumptions (announced defaults)
- Determinism contract: buffs mutate RulesDatabase in GameSnapshot before frame 0; never client-side | default | PROGRESSION.md pillar 1 | reversible
- Ranked stays clean: buffs apply only to unranked/campaign/co-op | default | PROGRESSION.md pillar 2 | reversible
- Local-trust profile with HMAC for rating-adjacent fields; server authoritative for MMR | default | PROGRESSION.md integrity | reversible

## Findings (cited - path:lines)
- PROGRESSION.md is a full design proposal, never implemented
- Required primitives exist: RulesDatabase (cnc-core/src/rules.rs), GameSnapshot (savegame.rs:64), AudioMixer jingle support, net-directory ladder (matchmaking.rs)
- No cross-session player persistence exists (PROGRESSION.md System 1 current state)

## Decisions (with rationale)
- Depends on continue-meta-foundation; do not start before that crate lands
- review_required=true (UNCLEAR, non-trivial)

## Scope IN
- Stars, unlocks, streaks, daily challenge, buffs, XP, levels, feedback, AI-ladder integration

## Scope OUT (Must NOT have)
- No purchasable power; no buffs in ranked; no mid-match buff injection; no per-player sim RNG; no crafting items (continue-crafting); no doctrine tree (continue-techtree)

## Open questions
- None blocking

## Approval gate
status: awaiting-approval
Approach above. Next: on explicit user approval, rerun scaffold without --draft-only, run Metis, append todos, fill TL;DR, dual high-accuracy review automatically.
