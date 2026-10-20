//! Deterministic execution of a compiled agent program.
//!
//! The host calls [AgentRuntime::on_start] once, [AgentRuntime::on_tick] for
//! every simulated tick and [AgentRuntime::on_event] when something happens.
//! Each call returns a [Report] of serializable [Command]s. The runtime never
//! reads a clock or a random source other than its seeded generator, so two
//! clients replaying the same observation stream produce identical commands.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::intent::{IntentCache, IntentValue};
use crate::script::{BinOp, Block, Expr, ExprKind, HandlerKind, Program, Stmt, UnOp};

/// 2D map cell or world coordinate carried by an observation or command.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// A visible hostile contact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    pub x: i32,
    pub y: i32,
    pub class: String,
    /// Relative danger, 0 for harmless scouts.
    pub threat: i64,
    pub visible: bool,
}

/// Everything an agent is allowed to know on a given tick.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub tick: u64,
    pub credits: i64,
    pub power: i64,
    /// Power satisfaction as a percentage (0..=100).
    pub power_ratio: i64,
    pub stance: String,
    pub map_width: i32,
    pub map_height: i32,
    pub base: Option<Pos>,
    pub units: BTreeMap<String, i64>,
    pub buildings: BTreeMap<String, i64>,
    pub contacts: Vec<Contact>,
    pub threat: i64,
}

impl Observation {
    pub fn units_of(&self, class: &str) -> i64 {
        self.units.get(class).copied().unwrap_or(0)
    }

    pub fn buildings_of(&self, class: &str) -> i64 {
        self.buildings.get(class).copied().unwrap_or(0)
    }

    fn base_pos(&self) -> Pos {
        self.base.unwrap_or_default()
    }

    fn nearest(&self, only_threatening: bool) -> Option<&Contact> {
        let base = self.base_pos();
        self.contacts
            .iter()
            .filter(|contact| contact.visible && (!only_threatening || contact.threat > 0))
            .min_by_key(|contact| {
                let dx = i64::from(contact.x - base.x);
                let dy = i64::from(contact.y - base.y);
                (dx * dx + dy * dy, contact.x, contact.y)
            })
    }
}

/// A command emitted by an agent for the game to apply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Command {
    Train { class: String },
    Build { class: String },
    Attack { x: i32, y: i32 },
    Defend { x: i32, y: i32 },
    Move { x: i32, y: i32 },
    Harvest,
    Scout { x: i32, y: i32 },
    Retreat,
    SetStance { stance: String },
    Say { text: String },
}

/// Budget applied to each handler invocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Maximum evaluated expressions per handler invocation.
    pub fuel: u32,
    /// Maximum iterations of one while loop.
    pub max_loop: u32,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            fuel: 50_000,
            max_loop: 10_000,
        }
    }
}

/// A runtime failure that stopped one handler (the simulation keeps running).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeFault {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl fmt::Display for RuntimeFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

/// Result of one runtime step.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub commands: Vec<Command>,
    pub logs: Vec<String>,
    /// Runtime failures formatted as line:column: message.
    pub diagnostics: Vec<String>,
    /// Intent keys the script asked for but the cache did not contain.
    pub intents_missing: Vec<String>,
}

impl Report {
    fn missing(&mut self, key: &str) {
        if !self.intents_missing.iter().any(|existing| existing == key) {
            self.intents_missing.push(key.to_string());
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    Int(i64),
    Bool(bool),
    Str(String),
}

impl Value {
    fn truthy(&self) -> bool {
        match self {
            Self::Int(value) => *value != 0,
            Self::Bool(value) => *value,
            Self::Str(value) => !value.is_empty(),
        }
    }

    fn as_int(&self) -> i64 {
        match self {
            Self::Int(value) => *value,
            Self::Bool(value) => i64::from(*value),
            Self::Str(_) => 0,
        }
    }

    fn as_text(&self) -> String {
        match self {
            Self::Int(value) => value.to_string(),
            Self::Bool(value) => value.to_string(),
            Self::Str(value) => value.clone(),
        }
    }
}

/// Running agent state. One instance per agent per match.
pub struct AgentRuntime {
    program: Program,
    variables: BTreeMap<String, Value>,
    rng: u64,
    next_run: Vec<Option<u64>>,
    intents: IntentCache,
    limits: Limits,
    fuel: u32,
    calls: BTreeSet<String>,
}

impl AgentRuntime {
    pub fn new(program: Program, seed: u64) -> Self {
        let next_run = vec![None; program.handlers.len()];
        let rng = seed ^ 0x9E37_79B9_7F4A_7C15;
        Self {
            program,
            variables: BTreeMap::new(),
            rng: if rng == 0 { 1 } else { rng },
            next_run,
            intents: IntentCache::default(),
            limits: Limits::default(),
            fuel: 0,
            calls: BTreeSet::new(),
        }
    }

    pub fn with_limits(mut self, limits: Limits) -> Self {
        self.limits = limits;
        self
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    /// Attach pre-recorded (or freshly fetched) advisor values.
    pub fn set_intents(&mut self, intents: IntentCache) {
        self.intents = intents;
    }

    pub fn intents(&self) -> &IntentCache {
        &self.intents
    }

    /// Run every on start handler once.
    pub fn on_start(&mut self, observation: &Observation) -> Report {
        let mut report = Report::default();
        for index in 0..self.program.handlers.len() {
            if !matches!(self.program.handlers[index].kind, HandlerKind::Start) {
                continue;
            }
            let body = self.program.handlers[index].body.clone();
            self.run_body(&body, observation, &mut report);
        }
        report
    }

    /// Run every due handler. Call once per simulation tick.
    pub fn on_tick(&mut self, observation: &Observation) -> Report {
        let mut report = Report::default();
        for index in 0..self.program.handlers.len() {
            let due = match &self.program.handlers[index].kind {
                HandlerKind::Every(_) => match self.next_run[index] {
                    None => true,
                    Some(next) => observation.tick >= next,
                },
                _ => false,
            };
            if !due {
                continue;
            }
            let interval_expr = match &self.program.handlers[index].kind {
                HandlerKind::Every(expr) => expr.clone(),
                _ => continue,
            };
            self.fuel = self.limits.fuel;
            let interval = match self.eval(&interval_expr, observation, &mut report) {
                Ok(value) => value.as_int().clamp(1, 60_000),
                Err(fault) => {
                    report.diagnostics.push(fault.to_string());
                    self.next_run[index] = Some(observation.tick.saturating_add(1));
                    continue;
                }
            };
            let body = self.program.handlers[index].body.clone();
            self.run_body(&body, observation, &mut report);
            self.next_run[index] = Some(observation.tick.saturating_add(interval as u64));
        }
        report
    }

    /// Run handlers for a named event. Cancelled callers keep their state.
    pub fn on_event(&mut self, observation: &Observation, event: &str) -> Report {
        let mut report = Report::default();
        for index in 0..self.program.handlers.len() {
            let matches = match &self.program.handlers[index].kind {
                HandlerKind::Event(name) => name.eq_ignore_ascii_case(event),
                _ => false,
            };
            if !matches {
                continue;
            }
            let body = self.program.handlers[index].body.clone();
            self.run_body(&body, observation, &mut report);
        }
        report
    }

    /// Commands emitted inside a while loop are bounded by configuration, but
    /// this reports whether the last invocation hit the instruction budget.
    pub fn last_call_was_truncated(&self) -> bool {
        self.fuel == 0
    }

    /// Keys referenced by this agent so far, useful for oracle prefetching.
    pub fn requested_intents(&self) -> Vec<String> {
        self.calls.iter().cloned().collect()
    }

    fn run_body(&mut self, body: &Block, observation: &Observation, report: &mut Report) {
        self.fuel = self.limits.fuel;
        if let Err(fault) = self.exec_block(body, observation, report) {
            report.diagnostics.push(fault.to_string());
        }
    }

    fn burn(&mut self, line: usize, column: usize) -> Result<(), RuntimeFault> {
        if self.fuel == 0 {
            return Err(RuntimeFault {
                line,
                column,
                message: "instruction budget exceeded for this tick".to_string(),
            });
        }
        self.fuel -= 1;
        Ok(())
    }

    fn exec_block(
        &mut self,
        block: &Block,
        observation: &Observation,
        report: &mut Report,
    ) -> Result<(), RuntimeFault> {
        for statement in block {
            match statement {
                Stmt::Let { name, value } => {
                    let evaluated = self.eval(value, observation, report)?;
                    self.variables.insert(name.clone(), evaluated);
                }
                Stmt::Assign { name, value } => {
                    let evaluated = self.eval(value, observation, report)?;
                    self.variables.insert(name.clone(), evaluated);
                }
                Stmt::If {
                    condition,
                    then_body,
                    else_body,
                } => {
                    let evaluated = self.eval(condition, observation, report)?;
                    if evaluated.truthy() {
                        self.exec_block(then_body, observation, report)?;
                    } else if let Some(body) = else_body {
                        self.exec_block(body, observation, report)?;
                    }
                }
                Stmt::While { condition, body } => {
                    let mut iterations = 0u32;
                    loop {
                        let evaluated = self.eval(condition, observation, report)?;
                        if !evaluated.truthy() {
                            break;
                        }
                        iterations += 1;
                        if iterations > self.limits.max_loop {
                            return Err(RuntimeFault {
                                line: statement_line(condition),
                                column: 1,
                                message: format!(
                                    "while loop exceeded {} iterations",
                                    self.limits.max_loop
                                ),
                            });
                        }
                        self.burn(statement_line(condition), 1)?;
                        self.exec_block(body, observation, report)?;
                    }
                }
                Stmt::Expr(expression) => {
                    self.eval(expression, observation, report)?;
                }
            }
        }
        Ok(())
    }

    fn eval(
        &mut self,
        expression: &Expr,
        observation: &Observation,
        report: &mut Report,
    ) -> Result<Value, RuntimeFault> {
        self.burn(expression.line, expression.column)?;
        match &expression.kind {
            ExprKind::Int(value) => Ok(Value::Int(*value)),
            ExprKind::Str(value) => Ok(Value::Str(value.clone())),
            ExprKind::Bool(value) => Ok(Value::Bool(*value)),
            ExprKind::Var(name) => Ok(self.variables.get(name).cloned().unwrap_or(Value::Int(0))),
            ExprKind::Unary(operator, operand) => {
                let value = self.eval(operand, observation, report)?;
                Ok(match operator {
                    UnOp::Not => Value::Bool(!value.truthy()),
                    UnOp::Neg => Value::Int(-value.as_int()),
                })
            }
            ExprKind::Binary(operator, left, right) => {
                if *operator == BinOp::And {
                    let left = self.eval(left, observation, report)?;
                    if !left.truthy() {
                        return Ok(Value::Bool(false));
                    }
                    let right = self.eval(right, observation, report)?;
                    return Ok(Value::Bool(right.truthy()));
                }
                if *operator == BinOp::Or {
                    let left = self.eval(left, observation, report)?;
                    if left.truthy() {
                        return Ok(Value::Bool(true));
                    }
                    let right = self.eval(right, observation, report)?;
                    return Ok(Value::Bool(right.truthy()));
                }
                let left = self.eval(left, observation, report)?;
                let right = self.eval(right, observation, report)?;
                self.binary(*operator, left, right, expression)
            }
            ExprKind::Call(name, arguments) => {
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.eval(argument, observation, report)?);
                }
                self.call(name, &values, observation, report, expression)
            }
        }
    }

    fn binary(
        &self,
        operator: BinOp,
        left: Value,
        right: Value,
        expression: &Expr,
    ) -> Result<Value, RuntimeFault> {
        let fault = |message: &str| RuntimeFault {
            line: expression.line,
            column: expression.column,
            message: message.to_string(),
        };
        Ok(match operator {
            BinOp::Add => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Value::Int(a.wrapping_add(b)),
                (Value::Str(a), Value::Str(b)) => Value::Str(a + &b),
                _ => return Err(fault("+ needs two numbers or two strings")),
            },
            BinOp::Sub => Value::Int(left.as_int().wrapping_sub(right.as_int())),
            BinOp::Mul => Value::Int(left.as_int().wrapping_mul(right.as_int())),
            BinOp::Div => {
                let divisor = right.as_int();
                Value::Int(if divisor == 0 {
                    0
                } else {
                    left.as_int() / divisor
                })
            }
            BinOp::Rem => {
                let divisor = right.as_int();
                Value::Int(if divisor == 0 {
                    0
                } else {
                    left.as_int() % divisor
                })
            }
            BinOp::Eq => Value::Bool(equal(&left, &right)),
            BinOp::Ne => Value::Bool(!equal(&left, &right)),
            BinOp::Lt => Value::Bool(left.as_int() < right.as_int()),
            BinOp::Le => Value::Bool(left.as_int() <= right.as_int()),
            BinOp::Gt => Value::Bool(left.as_int() > right.as_int()),
            BinOp::Ge => Value::Bool(left.as_int() >= right.as_int()),
            BinOp::And | BinOp::Or => unreachable!("short-circuited above"),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn call(
        &mut self,
        name: &str,
        arguments: &[Value],
        observation: &Observation,
        report: &mut Report,
        expression: &Expr,
    ) -> Result<Value, RuntimeFault> {
        let int = |index: usize| arguments.get(index).map(Value::as_int).unwrap_or(0);
        let text = |index: usize| arguments.get(index).map(Value::as_text).unwrap_or_default();
        let base = observation.base_pos();
        let state = match name {
            "tick" => Value::Int(observation.tick as i64),
            "credits" => Value::Int(observation.credits),
            "power" => Value::Int(observation.power),
            "power_ratio" => Value::Int(observation.power_ratio.clamp(0, 100)),
            "threat" => Value::Int(observation.threat),
            "enemies" => Value::Int(
                observation
                    .contacts
                    .iter()
                    .filter(|contact| contact.visible)
                    .count() as i64,
            ),
            "units" => Value::Int(observation.units_of(&text(0))),
            "buildings" => Value::Int(observation.buildings_of(&text(0))),
            "map_width" => Value::Int(i64::from(observation.map_width)),
            "map_height" => Value::Int(i64::from(observation.map_height)),
            "enemy_x" => Value::Int(
                observation
                    .contacts
                    .get(int(0).max(0) as usize)
                    .map(|contact| i64::from(contact.x))
                    .unwrap_or(0),
            ),
            "enemy_y" => Value::Int(
                observation
                    .contacts
                    .get(int(0).max(0) as usize)
                    .map(|contact| i64::from(contact.y))
                    .unwrap_or(0),
            ),
            "nearest_enemy_x" => Value::Int(i64::from(
                observation
                    .nearest(false)
                    .map(|contact| contact.x)
                    .unwrap_or(base.x),
            )),
            "nearest_enemy_y" => Value::Int(i64::from(
                observation
                    .nearest(false)
                    .map(|contact| contact.y)
                    .unwrap_or(base.y),
            )),
            "nearest_threat_x" => Value::Int(i64::from(
                observation
                    .nearest(true)
                    .map(|contact| contact.x)
                    .unwrap_or(base.x),
            )),
            "nearest_threat_y" => Value::Int(i64::from(
                observation
                    .nearest(true)
                    .map(|contact| contact.y)
                    .unwrap_or(base.y),
            )),
            "base_x" => Value::Int(i64::from(base.x)),
            "base_y" => Value::Int(i64::from(base.y)),
            "stance" => Value::Str(observation.stance.clone()),
            "random" => {
                let limit = int(0);
                Value::Int(if limit <= 0 {
                    0
                } else {
                    (self.next_random() % limit as u64) as i64
                })
            }
            "abs" => Value::Int(int(0).saturating_abs()),
            "min" => Value::Int(int(0).min(int(1))),
            "max" => Value::Int(int(0).max(int(1))),
            "clamp" => Value::Int(int(0).max(int(1)).min(int(2))),
            "intent" => {
                let key = text(0);
                self.calls.insert(key.clone());
                match self.intents.values.get(&key) {
                    Some(IntentValue::Int(value)) => Value::Int(*value),
                    _ => {
                        report.missing(&key);
                        Value::Int(int(1))
                    }
                }
            }
            "intent_text" => {
                let key = text(0);
                self.calls.insert(key.clone());
                match self.intents.values.get(&key) {
                    Some(IntentValue::Text(value)) => Value::Str(value.clone()),
                    _ => {
                        report.missing(&key);
                        Value::Str(text(1))
                    }
                }
            }
            "train" => {
                report.commands.push(Command::Train { class: text(0) });
                Value::Int(1)
            }
            "build" => {
                report.commands.push(Command::Build { class: text(0) });
                Value::Int(1)
            }
            "attack" => {
                report.commands.push(Command::Attack {
                    x: int(0) as i32,
                    y: int(1) as i32,
                });
                Value::Int(1)
            }
            "defend" => {
                report.commands.push(Command::Defend {
                    x: int(0) as i32,
                    y: int(1) as i32,
                });
                Value::Int(1)
            }
            "move" => {
                report.commands.push(Command::Move {
                    x: int(0) as i32,
                    y: int(1) as i32,
                });
                Value::Int(1)
            }
            "harvest" => {
                report.commands.push(Command::Harvest);
                Value::Int(1)
            }
            "scout" => {
                report.commands.push(Command::Scout {
                    x: int(0) as i32,
                    y: int(1) as i32,
                });
                Value::Int(1)
            }
            "retreat" => {
                report.commands.push(Command::Retreat);
                Value::Int(1)
            }
            "set_stance" => {
                report.commands.push(Command::SetStance { stance: text(0) });
                Value::Int(1)
            }
            "say" => {
                let message = text(0);
                report.commands.push(Command::Say {
                    text: message.clone(),
                });
                report.logs.push(message);
                Value::Int(1)
            }
            "log" => {
                report.logs.push(text(0));
                Value::Int(1)
            }
            other => {
                return Err(RuntimeFault {
                    line: expression.line,
                    column: expression.column,
                    message: format!("unknown function {other}"),
                })
            }
        };
        Ok(state)
    }

    fn next_random(&mut self) -> u64 {
        let mut value = self.rng;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.rng = value;
        value
    }
}

fn equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Str(a), Value::Str(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        _ => left.as_int() == right.as_int(),
    }
}

fn statement_line(expression: &Expr) -> usize {
    expression.line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::script::compile;

    fn runtime(source: &str) -> AgentRuntime {
        AgentRuntime::new(compile(source).expect("script should compile"), 42)
    }

    fn observation(tick: u64) -> Observation {
        Observation {
            tick,
            credits: 1_000,
            power: 100,
            power_ratio: 100,
            stance: "defensive".to_string(),
            map_width: 64,
            map_height: 64,
            base: Some(Pos::new(10, 10)),
            units: BTreeMap::from([("InfantryE1".to_string(), 2)]),
            buildings: BTreeMap::new(),
            contacts: vec![
                Contact {
                    x: 20,
                    y: 20,
                    class: "InfantryE1".to_string(),
                    threat: 1,
                    visible: true,
                },
                Contact {
                    x: 12,
                    y: 12,
                    class: "VehicleE1".to_string(),
                    threat: 5,
                    visible: true,
                },
            ],
            threat: 6,
        }
    }

    #[test]
    fn every_handler_runs_on_its_schedule() {
        let mut agent = runtime("every 10 { say(\"tick\") }");
        let mut emitted = Vec::new();
        for tick in 0..25 {
            let report = agent.on_tick(&observation(tick));
            emitted.push((tick, report.commands.len()));
        }
        assert_eq!(emitted[0], (0, 1));
        assert_eq!(emitted[9], (9, 0));
        assert_eq!(emitted[10], (10, 1));
        assert_eq!(emitted[20], (20, 1));
    }

    #[test]
    fn variables_persist_across_ticks() {
        let mut agent = runtime("every 1 { n = n + 1 if n == 3 { say(\"three\") } }");
        let first = agent.on_tick(&observation(0));
        assert!(first.commands.is_empty());
        agent.on_tick(&observation(1));
        let third = agent.on_tick(&observation(2));
        assert_eq!(third.logs, ["three"]);
    }

    #[test]
    fn deterministic_random_is_seeded() {
        let script = "every 1 { say(\"x\") if random(100) > 50 { attack(1, 2) } }";
        let mut a = runtime(script);
        let mut b = runtime(script);
        let first: Vec<_> = (0..20)
            .map(|tick| a.on_tick(&observation(tick)).commands)
            .collect();
        let second: Vec<_> = (0..20)
            .map(|tick| b.on_tick(&observation(tick)).commands)
            .collect();
        assert_eq!(first, second);
    }

    #[test]
    fn while_loop_is_bounded() {
        let mut agent = AgentRuntime::new(
            compile("on start { while true { log(\"spin\") } }").unwrap(),
            1,
        )
        .with_limits(Limits {
            fuel: 1_000,
            max_loop: 50,
        });
        let report = agent.on_start(&observation(0));
        assert!(report
            .diagnostics
            .iter()
            .any(|line| line.contains("while loop exceeded")));
    }

    #[test]
    fn intents_use_cache_or_fallback() {
        let source = "every 1 { if intent(\"push\", 0) == 1 { attack(5, 6) } log(intent_text(\"mood\", \"calm\")) }";
        let mut agent = runtime(source);
        let report = agent.on_tick(&observation(0));
        assert!(report.commands.is_empty());
        assert_eq!(report.logs, ["calm"]);
        assert_eq!(report.intents_missing, ["push", "mood"]);

        let mut cache = IntentCache::default();
        cache.values.insert("push".to_string(), IntentValue::Int(1));
        cache
            .values
            .insert("mood".to_string(), IntentValue::Text("angry".to_string()));
        agent.set_intents(cache);
        let report = agent.on_tick(&observation(1));
        assert_eq!(report.commands, [Command::Attack { x: 5, y: 6 }]);
        assert_eq!(report.logs, ["angry"]);
        assert!(report.intents_missing.is_empty());
    }

    #[test]
    fn nearest_queries_use_distance_and_threat() {
        let mut agent = runtime("every 1 { log(nearest_enemy_x()) log(nearest_threat_x()) }");
        let report = agent.on_tick(&observation(0));
        assert_eq!(report.logs, ["12", "12"]);
    }

    #[test]
    fn division_by_zero_yields_zero() {
        let mut agent = runtime("every 1 { log(10 / 0) }");
        let report = agent.on_tick(&observation(0));
        assert_eq!(report.logs, ["0"]);
    }
}
