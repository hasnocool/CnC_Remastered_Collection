# RUST/cnc-ai/src/

## Responsibility

Five modules: language (`script`), executor (`runtime`), recorded-advisor bridge (`intent`), authoring record (`agent`), directory ladder card (`ladder`), declared in `lib.rs` with extensive re-exports.

## Design

- `script.rs` — `compile()` lexes/parses into `Program` (list of `Handler`s with `HandlerKind` like condition/tick handlers), validates static rules (no floats, no clocks) and returns `Diagnostic`s with spans; `Builtin` table registers builtins by name (`builtin("...")`).
- `runtime.rs` — `AgentRuntime::on_start/on_tick/on_event` are the only entry points; each returns `Report` = serializable `Command`s + optional `RuntimeFault`. `Limits` bounds step counts so a script cannot hang lockstep.
- `intent.rs` — `intent_slots(program)` extracts the `IntentSlot`s a script asks the oracle for; `observation_prompt()` renders `Observation` + `Vision` grid to prompt text; `parse_response()` turns JSON into `IntentCache` (unknown/missing keys → deterministic fallback). `ORACLE_PROTOCOL_VERSION` guards the request/response envelope.
- `agent.rs` — `AgentDefinition {id, doctrine, script, ...}` stored in scenarios; `starter_script()` returns per-doctrine template sources.
- `ladder.rs` — `registration(...)` builds `AgentLadderCard`/`AiLadderRegistration` (owner, doctrine, sha1 script fingerprint, labels) for matching against the directory's AI ladder services.

## Flow

Authoring: editor inserts `AgentDefinition` → scenario INI/data blocks. Play: scenario loads definitions → `compile()` → `AgentRuntime` per agent on every tick: Observation in, Commands out; oracle requests fulfilled from `IntentCache` loaded at match start.

## Integration

- Consumed by `cnc-map-editor` (model + command layer: inserting/editing agents is undoable via history commands).
- Consumes nothing internal (serde/sha1 only) — deliberately standalone so engines or the directory can adopt it independently.
