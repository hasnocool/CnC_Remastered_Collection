---
slug: token-efficiency
status: awaiting-approval
intent: clear
review_required: false
pending-action: write .omo/plans/token-efficiency.md
approach: Apply AGENTS.md §11 token efficiency practices (compact, targeted reads, NOTES.md persistence, machine-readable outputs, cached discovery) to existing CnC_Remastered_Collection DeepSeek Harness sessions
---

# Draft: token-efficiency

## Components (topology ledger)
- Compaction (/compact >15-20 exchanges) | active | AGENTS.md 432-443 + UI (117 sessions, 3.4B tokens)
- Targeted reads (rg/fd/grep/limited ranges) | active | AGENTS.md 432-443
- NOTES.md persistence | active | AGENTS.md 432-443
- Machine-readable outputs | active | AGENTS.md 432-443
- Cached discovery | active | AGENTS.md 432-443

## Open assumptions (announced defaults)
- Apply to CnC_Remastered_Collection sessions only | yes | User selected option 1; highest token usage in UI | reversible
- Manual practices first (no rtk) | yes | 432-443 sufficient; deferred/reversible | reversible
- Evidence not deleted | yes (mandatory) | 432-443 prohibits; destructive/irreversible excluded | N/A (excluded)
- No code/build changes | yes | Session-local optimization only | reversible

## Findings (cited - path:lines)
- AGENTS.md 432-443 (token efficiency rules)
- User UI (117 sessions, 3.4B tokens, 95.44% cache hit, 20h33m tool runs)
- Project AGENTS.md (519 REDALERT + 298 TIBERIANDAWN C++ files — whole dumps expensive)
- Existing parity-100.md (519 lines, reducible by targeted reads)

## Decisions (with rationale)
1. /compact for CnC_Remastered_Collection sessions >15-20 exchanges (reversible).
2. Targeted rg/fd/grep reads instead of whole-file dumps (reversible).
3. NOTES.md persistence with goal/evidence/decisions/open questions (reversible).
4. Machine-readable outputs: --porcelain, -o json, --message-format=short (reversible).
5. Cache stable discovery (paths/versions/crate names) in NOTES.md (reversible).
6. Never delete session evidence/logs (§11 prohibits; destructive/irreversible — excluded).

## Scope IN
- /compact existing CnC_Remastered_Collection DeepSeek Harness sessions (evidence preserved)
- Targeted read practices: rg/fd/grep/limited-range reads
- NOTES.md persistence mechanism in .omo/
- Machine-readable output preferences (--porcelain, -o json)
- Cache stable discovery (paths/versions/crate names)
- Verify evidence preservation before/after

## Scope OUT (Must NOT have)
- No evidence/log/session deletion (§11 prohibits; destructive/irreversible)
- No new automated token-reduction tooling in this phase (deferred/reversible)
- No source/build/code changes (session-local optimization only)
- No build/test/CI changes
- No subagent/concurrency modifications
- No git mirror/push/rewrite (out of scope)

## Open questions
- None. Intent CLEAR (user selected option 1: optimize existing sessions). All actions reversible; evidence preservation mandatory; no ambiguous contracts. No high-accuracy review required (discoverable AGENTS.md rules + observable UI data only).

## Approval gate
status: awaiting-approval
approach: Apply AGENTS.md 432-443 practices to existing CnC_Remastered_Collection DeepSeek Harness sessions: /compact (>15-20 exchanges), targeted rg/fd/grep reads, NOTES.md persistence, machine-readable outputs (--porcelain, -o json), cached stable discovery. Evidence preserved (no deletion per 432-443). Fully reversible; no destructive actions; no new tooling; no source/build changes.
next-workflow-action: After approval: (1) apply /compact and targeted-read practices, (2) create NOTES.md with tracking, (3) verify with before/after session context observation, (4) record results and evidence.
<!-- When exploration is exhausted and unknowns are answered, set status: awaiting-approval. -->
<!-- That durable record is the loop guard: on a later turn read it and resume at the gate instead of re-running exploration. -->
