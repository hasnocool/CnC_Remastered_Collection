//! Out-of-band advisor values: LLM and vision results that feed agent scripts.
//!
//! A model never runs inside the lockstep simulation. Instead the host asks an
//! oracle (a local model, a hosted model or a recording) for values ahead of
//! time, records them in the match, and hands them to [crate::runtime] as an
//! [IntentCache]. Scripts read the values with intent() / intent_text() and
//! always provide a deterministic fallback, so a missing or late answer simply
//! falls back instead of stalling the match.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::runtime::Observation;
use crate::script::{Expr, ExprKind, HandlerKind, Program, Stmt};

/// Version of the JSON oracle protocol.
pub const ORACLE_PROTOCOL_VERSION: u32 = 1;

/// One advisor answer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntentValue {
    Int(i64),
    Text(String),
}

/// Answers keyed by the literal key used in the script.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentCache {
    pub values: BTreeMap<String, IntentValue>,
}

impl IntentCache {
    pub fn insert_int(&mut self, key: impl Into<String>, value: i64) {
        self.values.insert(key.into(), IntentValue::Int(value));
    }

    pub fn insert_text(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values
            .insert(key.into(), IntentValue::Text(value.into()));
    }
}

/// Whether an advisor slot expects a number or text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentKind {
    Int,
    Text,
}

/// A decision the host should prefetch before a match.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntentSlot {
    pub key: String,
    pub kind: IntentKind,
    /// Deterministic value used when the oracle has no answer.
    pub fallback: String,
}

/// Extract every intent() / intent_text() key, in source order, deduplicated.
pub fn intent_slots(program: &Program) -> Vec<IntentSlot> {
    let mut slots: Vec<IntentSlot> = Vec::new();
    for handler in &program.handlers {
        if let HandlerKind::Every(expression) = &handler.kind {
            walk_expr(expression, &mut slots);
        }
        walk_block(&handler.body, &mut slots);
    }
    slots
}

fn walk_block(block: &[Stmt], slots: &mut Vec<IntentSlot>) {
    for statement in block {
        match statement {
            Stmt::Let { value, .. } | Stmt::Assign { value, .. } => walk_expr(value, slots),
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                walk_expr(condition, slots);
                walk_block(then_body, slots);
                if let Some(body) = else_body {
                    walk_block(body, slots);
                }
            }
            Stmt::While { condition, body } => {
                walk_expr(condition, slots);
                walk_block(body, slots);
            }
            Stmt::Expr(expression) => walk_expr(expression, slots),
        }
    }
}

fn walk_expr(expression: &Expr, slots: &mut Vec<IntentSlot>) {
    if let ExprKind::Call(name, arguments) = &expression.kind {
        if (name == "intent" || name == "intent_text") && arguments.len() >= 2 {
            if let ExprKind::Str(key) = &arguments[0].kind {
                if !slots.iter().any(|slot| slot.key == *key) {
                    let fallback = match &arguments[1].kind {
                        ExprKind::Str(value) => value.clone(),
                        ExprKind::Int(value) => value.to_string(),
                        ExprKind::Bool(value) => value.to_string(),
                        _ => String::new(),
                    };
                    slots.push(IntentSlot {
                        key: key.clone(),
                        kind: if name == "intent" {
                            IntentKind::Int
                        } else {
                            IntentKind::Text
                        },
                        fallback,
                    });
                }
            }
        }
    }
    match &expression.kind {
        ExprKind::Call(_, arguments) => {
            for argument in arguments {
                walk_expr(argument, slots);
            }
        }
        ExprKind::Unary(_, operand) => walk_expr(operand, slots),
        ExprKind::Binary(_, left, right) => {
            walk_expr(left, slots);
            walk_expr(right, slots);
        }
        _ => {}
    }
}

/// A coarse, downsampled perception frame an LLM can read as text.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vision {
    pub width: u32,
    pub height: u32,
    /// Row-major cells using the VISION_* constants below.
    pub cells: Vec<u8>,
}

pub const VISION_EMPTY: u8 = 0;
pub const VISION_OWN: u8 = 1;
pub const VISION_ENEMY: u8 = 2;
pub const VISION_RESOURCE: u8 = 3;
pub const VISION_BLOCKED: u8 = 4;

impl Vision {
    /// Render the frame as characters for a text-only model.
    pub fn to_ascii(&self) -> String {
        let mut output = String::with_capacity((self.width as usize + 1) * self.height as usize);
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                let cell = self.cells.get(index).copied().unwrap_or(VISION_EMPTY);
                output.push(match cell {
                    VISION_OWN => '<',
                    VISION_ENEMY => '>',
                    VISION_RESOURCE => '*',
                    VISION_BLOCKED => '#',
                    _ => '.',
                });
            }
            output.push('\n');
        }
        output
    }

    pub fn legend() -> &'static str {
        "< own forces, > enemy forces, * resource, # blocked terrain, . empty"
    }
}

/// Request body written for an external oracle (or consumed directly by a
/// local model runner).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleRequest {
    pub version: u32,
    pub request_id: String,
    pub objective: String,
    pub agent: String,
    pub owner: String,
    pub doctrine: String,
    pub observation: Observation,
    pub vision: Option<Vision>,
    /// Optional path to a rendered vision PNG for multimodal models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vision_image: Option<String>,
    pub intents: Vec<IntentSlot>,
}

impl OracleRequest {
    pub fn new(
        request_id: impl Into<String>,
        agent: impl Into<String>,
        owner: impl Into<String>,
        doctrine: impl Into<String>,
        observation: Observation,
        vision: Option<Vision>,
        intents: Vec<IntentSlot>,
    ) -> Self {
        Self {
            version: ORACLE_PROTOCOL_VERSION,
            request_id: request_id.into(),
            objective: "Choose the next strategic intent for this AI commander.".to_string(),
            agent: agent.into(),
            owner: owner.into(),
            doctrine: doctrine.into(),
            observation,
            vision,
            vision_image: None,
            intents,
        }
    }

    /// Attach a rendered perception image (for multimodal endpoints).
    #[must_use]
    pub fn with_vision_image(mut self, path: impl Into<String>) -> Self {
        self.vision_image = Some(path.into());
        self
    }
}

/// One answer in an oracle response.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleAnswer {
    pub key: String,
    pub value: IntentValue,
}

/// Response body returned by an oracle.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleResponse {
    #[serde(default)]
    pub request_id: Option<String>,
    #[serde(default)]
    pub answers: Vec<OracleAnswer>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Validate an oracle response against the request and turn it into a cache.
///
/// Unknown keys and values of the wrong kind are rejected; omitted keys are
/// allowed and fall back inside the script.
pub fn parse_response(request: &OracleRequest, json: &str) -> Result<IntentCache, String> {
    let response: OracleResponse =
        serde_json::from_str(json).map_err(|error| format!("invalid oracle JSON: {error}"))?;
    if let Some(id) = &response.request_id {
        if id != &request.request_id {
            return Err(format!(
                "oracle answered request {id}, expected {}",
                request.request_id
            ));
        }
    }
    let mut cache = IntentCache::default();
    for answer in response.answers {
        let Some(slot) = request.intents.iter().find(|slot| slot.key == answer.key) else {
            return Err(format!("oracle returned unknown intent key {}", answer.key));
        };
        match (&slot.kind, &answer.value) {
            (IntentKind::Int, IntentValue::Int(_)) | (IntentKind::Text, IntentValue::Text(_)) => {
                cache.values.insert(answer.key, answer.value);
            }
            _ => {
                return Err(format!(
                    "oracle returned the wrong value type for intent {}",
                    answer.key
                ))
            }
        }
    }
    Ok(cache)
}

/// A compact prompt including the observation and the vision frame.
pub fn observation_prompt(request: &OracleRequest) -> String {
    let mut prompt = String::new();
    prompt.push_str("You are the strategic advisor for a Command & Conquer AI commander.\n");
    prompt.push_str(&format!(
        "Agent: {} | Owner: {} | Doctrine: {}\n",
        request.agent, request.owner, request.doctrine
    ));
    prompt.push_str(&format!("Objective: {}\n\n", request.objective));
    prompt.push_str("Observation (JSON):\n");
    prompt.push_str(&serde_json::to_string_pretty(&request.observation).unwrap_or_default());
    prompt.push_str("\n\n");
    if let Some(vision) = &request.vision {
        prompt.push_str(&format!(
            "Vision grid {}x{} ({}):\n",
            vision.width,
            vision.height,
            Vision::legend()
        ));
        prompt.push_str(&vision.to_ascii());
        prompt.push('\n');
    }
    if let Some(path) = &request.vision_image {
        prompt.push_str(&format!("Rendered vision image: {path}\n"));
    }
    prompt.push_str("Choose a value for each requested intent:\n");
    for slot in &request.intents {
        prompt.push_str(&format!(
            "- key {:?}, kind {:?}, deterministic fallback {:?}\n",
            slot.key, slot.kind, slot.fallback
        ));
    }
    prompt.push_str(
        "\nReply with JSON only: {\"request_id\": \"...\", \"answers\": [{\"key\": \"...\", \"value\": 0}]}.\n",
    );
    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::script::compile;

    #[test]
    fn slots_are_extracted_in_order_and_deduplicated() {
        let program = compile(
            r#"
            on start { if intent("push", 0) == 1 { attack(1, 2) } }
            every 10 { log(intent_text("mood", "calm")) if intent("push", 0) == 0 { harvest() } }
            "#,
        )
        .unwrap();
        let slots = intent_slots(&program);
        assert_eq!(slots.len(), 2);
        assert_eq!(slots[0].key, "push");
        assert_eq!(slots[0].kind, IntentKind::Int);
        assert_eq!(slots[1].key, "mood");
        assert_eq!(slots[1].kind, IntentKind::Text);
    }

    #[test]
    fn vision_ascii_roundtrip() {
        let vision = Vision {
            width: 3,
            height: 2,
            cells: vec![
                VISION_OWN,
                VISION_ENEMY,
                VISION_EMPTY,
                VISION_RESOURCE,
                VISION_BLOCKED,
                VISION_EMPTY,
            ],
        };
        assert_eq!(vision.to_ascii(), "<>.\n*#.\n");
    }

    #[test]
    fn response_is_validated_and_converted() {
        let program = compile("on start { log(intent_text(\"mood\", \"calm\")) }").unwrap();
        let request = OracleRequest::new(
            "req-1",
            "Iron Fist",
            "HouseNod",
            "Aggressive",
            Observation::default(),
            None,
            intent_slots(&program),
        );
        let cache = parse_response(
            &request,
            r#"{"request_id":"req-1","answers":[{"key":"mood","value":"angry"}]}"#,
        )
        .unwrap();
        assert_eq!(cache.values["mood"], IntentValue::Text("angry".to_string()));

        assert!(parse_response(&request, r#"{"answers":[{"key":"nope","value":1}]}"#).is_err());
        assert!(parse_response(&request, r#"{"answers":[{"key":"mood","value":4}]}"#).is_err());
    }

    #[test]
    fn vision_image_is_optional_and_prompted() {
        let request = OracleRequest::new(
            "req-3",
            "Scout",
            "HouseGDI",
            "Balanced",
            Observation::default(),
            None,
            vec![],
        );
        let json = serde_json::to_value(&request).unwrap();
        assert!(json.get("vision_image").is_none());
        let request = request.with_vision_image("maps/ai_vision.png");
        assert!(observation_prompt(&request).contains("Rendered vision image: maps/ai_vision.png"));
    }

    #[test]
    fn prompt_contains_vision_and_observation() {
        let request = OracleRequest::new(
            "req-2",
            "Scout",
            "HouseGDI",
            "Balanced",
            Observation {
                tick: 7,
                credits: 500,
                ..Observation::default()
            },
            Some(Vision {
                width: 1,
                height: 1,
                cells: vec![VISION_ENEMY],
            }),
            vec![],
        );
        let prompt = observation_prompt(&request);
        assert!(prompt.contains("Vision grid 1x1"));
        assert!(prompt.contains(">"));
        assert!(prompt.contains("\"tick\": 7"));
    }
}
