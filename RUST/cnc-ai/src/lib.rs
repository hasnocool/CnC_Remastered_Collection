//! Deterministic AI agents for Command & Conquer scenarios.
//!
//! The crate has three layers:
//!
//! * [script] compiles the small AgentScript language to a checked AST.
//! * [runtime] executes agents against an [runtime::Observation] and emits
//!   serializable [runtime::Command]s. The runtime is deterministic and
//!   lockstep-safe.
//! * [intent] carries out-of-band LLM/vision advice as pre-recorded values so
//!   a non-deterministic model never touches the simulation directly.
//! * [ladder] turns an authored agent into a registration card for the
//!   directory's AI-only ranked ladder.
//!
//! [agent::AgentDefinition] is the authoring record stored in a scenario.

pub mod agent;
pub mod intent;
pub mod ladder;
pub mod runtime;
pub mod script;

pub use agent::{starter_script, AgentDefinition, DOCTRINES};
pub use intent::{
    intent_slots, observation_prompt, parse_response, IntentCache, IntentSlot, IntentValue,
    OracleRequest, OracleResponse, Vision, VISION_BLOCKED, VISION_EMPTY, VISION_ENEMY, VISION_OWN,
    VISION_RESOURCE,
};
pub use ladder::{registration, AgentLadderCard, AiLadderRegistration};
pub use runtime::{AgentRuntime, Command, Contact, Limits, Observation, Pos, Report, RuntimeFault};
pub use script::{class_references, compile, Diagnostic, Program};
