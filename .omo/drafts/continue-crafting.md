---
slug: continue-crafting
status: awaiting-approval
intent: unclear
review_required: true
plan_path: .omo/plans/continue-crafting.md
plan_sha256: null
review_round_id: null
pending-action: write and review .omo/plans/continue-crafting.md
review:
  momus:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-crafting.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
  independent:
    status: pending
    workspace_root: null
    runtime_home: null
    target: .omo/plans/continue-crafting.md
    round_id: null
    plan_sha256: null
    launch_id: null
    session: null
    result: null
approach: Implement CRAFTING.md: affix/item model, workshop screen, salvage economy, loadout equipping via scoped RuleTweaks, fingerprint handshake + determinism tests, uniques/sets/runewords, in-match crate drops. Depends on continue-meta-foundation & RuleTweak table. One decision-complete plan.
---

# Draft: continue-crafting

## Components (topology ledger)
- item-model | affix table JSON + {base, seed, affixes} item model + name generator | active | CRAFTING.md Layer 1
- workshop | reforge/upgrade/identify + rarity-colored UI | active | CRAFTING.md Layer 3
- salvage | salvage economy from match telemetry + pity counter | active | CRAFTING.md Layer 3
- equip | loadout equipping -> scoped RuleTweaks | active | CRAFTING.md Layer 2
- integrity | fingerprint handshake + per-affix determinism tests | active | CRAFTING.md economy/anti-abuse
- content | uniques + sets + runewords | active | CRAFTING.md Layer 1
- crate-drops | in-match crate drops via hash_coord + Command::Pickup | active | CRAFTING.md Layer 2

## Open assumptions (announced defaults)
- Items are pure data; stats are a pure function of {base, seed, affixes} + versioned affix table | default | CRAFTING.md pillar 1 | reversible
- Reuse pinned SplitMix64 PRNG; no rand dependency | default | CRAFTING.md pillar 2 | reversible
- Meta layer ships first; battlefield salvage phase 2 | default | CRAFTING.md Layer 2 | reversible

## Findings (cited - path:lines)
- CRAFTING.md is a full design proposal, never implemented
- prng.rs ships pinned SplitMix64 + hash_coord (cnc-procgen)
- RuleTweak injection contract shared with PROGRESSION/TECHTREE

## Decisions (with rationale)
- Depends on continue-meta-foundation and shared RuleTweak table
- review_required=true (UNCLEAR, non-trivial)

## Scope IN
- Affix table, item model, workshop, salvage, equipping, handshake, uniques/sets/runewords, crate drops

## Scope OUT (Must NOT have)
- No runtime RNG in simulation; no pay-for-items; no items in ranked; no unbounded affix tables; no trading until server-authoritative escrow

## Open questions
- None blocking

## Approval gate
status: awaiting-approval
Approach above. Next: on explicit user approval, rerun scaffold without --draft-only, run Metis, append todos, fill TL;DR, dual high-accuracy review automatically.
