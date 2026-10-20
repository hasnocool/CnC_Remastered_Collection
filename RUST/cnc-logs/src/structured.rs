//! Structured logging support

use crate::LogLevel;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// A structured log entry with typed fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLog {
    pub timestamp: u64,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub fields: Map<String, Value>,
    pub span_name: Option<String>,
    pub span_id: Option<u64>,
    pub trace_id: Option<String>,
    pub span_id_hex: Option<String>,
}

impl StructuredLog {
    /// Create a new structured log from a log entry
    pub fn from_entry(entry: &crate::LogEntry) -> Self {
        let fields = entry
            .fields
            .clone()
            .unwrap_or_else(|| Value::Object(Map::new()));
        let fields_map = if let Value::Object(map) = fields {
            map
        } else {
            Map::new()
        };

        Self {
            timestamp: entry.timestamp,
            level: entry.level,
            target: entry.target.clone(),
            message: entry.message.clone(),
            fields: fields_map,
            span_name: entry.span_name.clone(),
            span_id: entry.span_id,
            trace_id: None,
            span_id_hex: entry.span_id.map(|id| format!("{:016x}", id)),
        }
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Convert to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Builder for structured logs
#[derive(Debug, Default)]
pub struct StructuredLogBuilder {
    level: Option<LogLevel>,
    target: Option<String>,
    message: Option<String>,
    fields: Map<String, Value>,
    span_name: Option<String>,
    span_id: Option<u64>,
    trace_id: Option<String>,
}

impl StructuredLogBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the log level
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = Some(level);
        self
    }

    /// Set the target
    pub fn target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    /// Set the message
    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    /// Add a field
    pub fn field(mut self, key: &str, value: impl Into<Value>) -> Self {
        self.fields.insert(key.to_string(), value.into());
        self
    }

    /// Add multiple fields
    pub fn fields(mut self, fields: HashMap<String, Value>) -> Self {
        self.fields.extend(fields);
        self
    }

    /// Set span name
    pub fn span_name(mut self, name: &str) -> Self {
        self.span_name = Some(name.to_string());
        self
    }

    /// Set span ID
    pub fn span_id(mut self, id: u64) -> Self {
        self.span_id = Some(id);
        self
    }

    /// Set trace ID
    pub fn trace_id(mut self, id: &str) -> Self {
        self.trace_id = Some(id.to_string());
        self
    }

    /// Build the structured log
    pub fn build(self) -> StructuredLog {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        StructuredLog {
            timestamp: now,
            level: self.level.unwrap_or(LogLevel::Info),
            target: self.target.unwrap_or_else(|| "unknown".to_string()),
            message: self.message.unwrap_or_else(|| "".to_string()),
            fields: self.fields,
            span_name: self.span_name,
            span_id: self.span_id,
            trace_id: self.trace_id,
            span_id_hex: self.span_id.map(|id| format!("{:016x}", id)),
        }
    }
}

/// Trait for types that can be logged structurally
pub trait StructuredLoggable {
    /// Convert to structured fields
    fn to_structured_fields(&self) -> Map<String, Value>;
}
