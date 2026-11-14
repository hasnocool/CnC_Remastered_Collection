# Function Porting Contract Template

Use one contract per original callable symbol, or one grouped contract when several symbols jointly implement an indivisible behavior. Keep original symbol names and source locations even when the target implementation uses a different architecture.

## Identity

- Review state: UNSEEN
- Original file: <relative path>
- Original source revision: f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae
- Original symbol: <qualified name>
- Original signature: <exact declaration/signature>
- Source location: <line range>
- Visibility: public / exported / protected / private / file-local / callback / virtual override / macro / assembly entry
- Original callers: <verified callers>
- Original callees: <verified calls or subsystem dependencies>

## Responsibility

Describe the behavior, not just the function name. State the user-visible, simulation-visible, data-visible, or platform-visible result the original code produces.

## Inputs and preconditions

| Input | Type / format | Meaning | Valid range / precondition | Failure behavior |
|---|---|---|---|---|
| <name> | <type> | <meaning> | <constraints> | <reject / clamp / ignore / fallback> |

Include implicit inputs: global state, object ownership, current frame, random seed, difficulty, configuration, platform services, filesystem, and timing.

## Outputs and postconditions

| Output / result | Meaning | Required postcondition |
|---|---|---|
| <value or state> | <meaning> | <guarantee> |

Record return values, output parameters, emitted events, packets, files, UI changes, and state transitions.

## State reads and side effects

- Reads: <objects, globals, rules, map cells, timers, files, network state>
- Mutates: <objects, ownership, health, queues, map, campaign state>
- Emits: <events, audio, visual effects, packets, log entries>
- I/O: <files, devices, sockets, platform APIs>
- Concurrency/ordering: <thread affinity, lock ownership, callback order, frame/tick constraints>

## Algorithm and invariants

Document the actual decision sequence, arithmetic/rounding, iteration order, sorting/tie-breaking, random-number consumption, timer behavior, coordinate conventions, and invariants that affect observable behavior.

## Errors and edge cases

List malformed input, missing dependencies, invalid object references, out-of-bounds values, destroyed objects, resource exhaustion, cancellation, network loss, and intentional legacy quirks.

## Reproduction requirements

A target implementation must preserve:
- <required behavior 1>
- <required behavior 2>
- <serialization / file / network format, where applicable>
- <determinism / ordering / timing contract, where applicable>
- <failure and recovery semantics>

State what can be redesigned internally and what is externally constrained by save data, replay determinism, network peers, map files, asset formats, modding, or UI workflows.

## Target-language mapping

- Target file(s): <path>
- Target symbol(s): <qualified name(s)>
- Mapping type: direct / split / merged / replaced / absent
- Known semantic differences: <details>

## Verification

- Source-derived fixture(s): <path>
- Unit/integration/runtime test(s): <test names>
- Command and result: <exact command, commit, date, output>
- Parity state: not run / failing / passing / manually reviewed

Do not use PARITY_VERIFIED until the evidence and target commit are recorded.
