# RUST/cnc-ai/

## Responsibility
Deterministic AI agent system for Command & Conquer scenarios. Provides a three-layer architecture: script compilation (AgentScript DSL), deterministic runtime execution, and LLM/vision intent bridge for non-deterministic advice without breaking lockstep simulation.

## Design Patterns
- **DSL Compilation**: AgentScript language to AST via script module
- **Deterministic Runtime**: Lockstep-safe AgentRuntime executing against Observation, emitting serializable Commands
- **Intent Bridge**: Out-of-band LLM/vision advice as pre-recorded IntentValues - non-deterministic models never touch simulation directly
- **Ladder Registration**: ladder module converts authored agents into registration cards for AI-only ranked ladder
- **Doctrine System**: Pre-defined DOCTRINES and starter_script for common AI behaviors

## Data & Control Flow
1. **Authoring**: AgentDefinition stores script + metadata
2. **Compilation**: script::compile() to checked Program AST
2. **Runtime**: AgentRuntime::new(program) to step(observation) to Vec<Command>
3. **Intent (optional)**: intent::observation_prompt() to LLM to parse_response() to IntentCache to injected at runtime
4. **Ladder**: ladder::registration() to AgentLadderCard for directory ranking

## Public API (Re-exports from lib.rs)
- **agent**: AgentDefinition, DOCTRINES, starter_script
- **intent**: IntentSlot, IntentValue, IntentCache, OracleRequest, OracleResponse, Vision constants, intent_slots, observation_prompt, parse_response
- **ladder**: AgentLadderCard, AiLadderRegistration, registration
- **runtime**: AgentRuntime, Command, Contact, Limits, Observation, Pos, Report, RuntimeFault
- **script**: Diagnostic, Program, compile, class_references

## Integration Points
- **Dependencies**: serde, serde_json, sha1
- **Consumed by**: Scenario authoring tools, AI ladder directory (cnc-net-directory potential), future campaign AI
- **No direct game engine dependency** - operates on abstract Observation/Command types suitable for both TD and RA

## Key Files
- src/lib.rs - Module declarations and public API surface
- src/agent.rs - AgentDefinition, doctrines, starter scripts
- src/runtime.rs - AgentRuntime, Observation, Command, lockstep execution
- src/script.rs - AgentScript parser/compiler, AST, diagnostics
- src/intent.rs - LLM/vision bridge, intent caching, vision constants
- src/ladder.rs - Ladder registration cards for ranked AI directory

