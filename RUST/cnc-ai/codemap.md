# RUST/cnc-ai/

## Responsibility

Deterministic agent-scripting crate for C&C scenarios: a small AgentScript language (`script.rs`), its lockstep-safe execution runtime, an out-of-band LLM/vision intent oracle bridge, and a publish path for AI-only ranked ladders. Description in its Cargo.toml: "Deterministic agent scripting language, runtime and LLM intent bridge for CnC scenarios".

## Design

Three deliberate layers, all real (see lib.rs docs):

- `script.rs` (1338 lines) — lexer/parser/checker with diagnostics: `Program`, `Handler`, `HandlerKind`, `Stmt`, `Expr`, `ExprKind`, `UnOp`, `BinOp`, `Builtin`/`BuiltinKind` (a static builtin table via `builtin()`), `ClassReference`, `Diagnostic`; entry points `compile(source) -> Result<Program, Vec<Diagnostic>>` and `class_references(program)`.
- `runtime.rs` — host-driven deterministic execution: `AgentRuntime` with `on_start`/`on_tick`/`on_event`; per-call `Observation` in, `Report` of serializable `Command`s out; `Contact`, `Pos`, `Limits` (step/instruction caps), `RuntimeFault` termination record. No wall clock, no floats, no unbounded loops ⇒ lockstep-safe.
- `intent.rs` — the only non-determinism inlet, as pre-recorded values: `IntentCache`, `IntentSlot`/`IntentKind`, `IntentValue`, `OracleRequest`/`OracleResponse`/`OracleAnswer`, `intent_slots(program)`, `observation_prompt(request)`, `parse_response(request, json)`, plus a 4×4 `Vision` grid with `VISION_EMPTY/OWN/ENEMY/RESOURCE/BLOCKED` and `ORACLE_PROTOCOL_VERSION = 1`.
- `agent.rs` — scenario authoring record `AgentDefinition` + `DOCTRINES` (5: Balanced, Aggressive, Defensive, Economy, Advisor) + `starter_script(doctrine)`.
- `ladder.rs` — directory-bridge card: `AgentLadderCard`, `AiLadderRegistration`, `registration(...)`, fingerprinted by script hash via `sha1`.
- Tiny dependency footprint on purpose: serde, serde_json, sha1 only.

## Flow

Host (engine tick loop or editor/launcher test harness) calls `AgentRuntime::on_tick(Observation)` → `Report{Command}` list → commands applied to game state. Oracle path: host builds `OracleRequest` (`observation_prompt`), an external process (e.g. `scripts/ai_oracle.py`) answers, `parse_response` validates JSON into `IntentCache`, runtime reads recorded values — the model itself never touches the simulation.

## Integration

- Workspace consumption: only `cnc-map-editor` depends on it (`MapModel.agents: Vec<AgentDefinition>`, agent add/edit commands); the launcher/editor may then load `ai_oracle_response.json` as IntentCache for play tests.
- `cnc-net-directory`'s matchmaking (`ai-only ladder`) is the conceptual target of `ladder.rs`; that crate does not depend on cnc-ai, so the card must be carried by a host process.
- Evidence of real use: 30+ references to AgentDefinition in cnc-map-editor (map.rs, history.rs, app.rs), integration test `authoring_session_roundtrips_through_ini` exercises agents.
