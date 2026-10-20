//! Custom tracing layer for C&C logging

use crate::{LogBuffer, LogConfig, LogEntry, LogLevel};
use std::sync::Arc;
use tracing::{Event, Metadata, Subscriber};
use tracing_core::metadata::LevelFilter;
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

/// Custom layer that captures logs into an in-memory buffer and formats them
pub struct CncLogLayer {
    buffer: Option<Arc<LogBuffer>>,
    config: LogConfig,
}

impl CncLogLayer {
    /// Create a new CncLogLayer
    pub fn new(config: LogConfig) -> Self {
        Self {
            buffer: None,
            config,
        }
    }

    /// Create a new CncLogLayer with a buffer
    pub fn with_buffer(config: LogConfig, buffer: Arc<LogBuffer>) -> Self {
        Self {
            buffer: Some(buffer),
            config,
        }
    }

    /// Get the buffer if available
    pub fn buffer(&self) -> Option<Arc<LogBuffer>> {
        self.buffer.clone()
    }

    /// Format an event into a log entry
    fn format_event<S>(&self, event: &Event<'_>, ctx: Context<'_, S>) -> LogEntry
    where
        S: Subscriber + for<'a> LookupSpan<'a>,
    {
        let metadata = event.metadata();
        let level = LogLevel::from(*metadata.level());
        let target = metadata.target().to_string();

        // Format the message
        let mut visitor = MessageVisitor::default();
        event.record(&mut visitor);
        let message = visitor.message;

        // Extract structured fields
        let mut field_visitor = FieldVisitor::default();
        event.record(&mut field_visitor);
        let fields = if field_visitor.fields.is_empty() {
            None
        } else {
            Some(serde_json::Value::Object(field_visitor.fields))
        };

        // Get span context
        let span = ctx.current_span();
        let span_name = span.metadata().map(|m| m.name().to_string());
        let span_id = span.id().map(|id| id.into_u64());

        // Get source location
        let file = metadata.file().map(|s| s.to_string());
        let line = metadata.line();
        let module_path = metadata.module_path().map(|s| s.to_string());

        // Get thread info
        let thread = std::thread::current();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        LogEntry {
            timestamp: now,
            level,
            target,
            message,
            fields,
            span_name,
            span_id,
            thread_name: thread.name().map(|s| s.to_string()),
            file,
            line,
            module_path,
        }
    }
}

impl<S> Layer<S> for CncLogLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let entry = self.format_event(event, ctx);

        // Store in buffer if available
        if let Some(buffer) = &self.buffer {
            buffer.push(entry.clone());
        }

        // Also store in global buffer if available
        if let Some(global) = crate::global_log_buffer() {
            global.push(entry);
        }
    }

    fn enabled(&self, metadata: &Metadata<'_>, _ctx: Context<'_, S>) -> bool {
        // Check target-specific levels
        if let Some(level) = self.config.target_levels.get(metadata.target()) {
            let filter_level: LevelFilter = (*level).into();
            metadata.level() <= &filter_level
        } else {
            let filter_level: LevelFilter = self.config.level.into();
            metadata.level() <= &filter_level
        }
    }
}

/// Visitor to extract the message from an event
#[derive(Default)]
struct MessageVisitor {
    message: String,
}

impl tracing::field::Visit for MessageVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }
}

/// Visitor to extract structured fields from an event
#[derive(Default)]
struct FieldVisitor {
    fields: serde_json::Map<String, serde_json::Value>,
}

impl tracing::field::Visit for FieldVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() != "message" {
            self.fields.insert(
                field.name().to_string(),
                serde_json::Value::String(format!("{:?}", value)),
            );
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() != "message" {
            self.fields.insert(
                field.name().to_string(),
                serde_json::Value::String(value.to_string()),
            );
        }
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        if field.name() != "message" {
            self.fields.insert(
                field.name().to_string(),
                serde_json::Value::Number(value.into()),
            );
        }
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        if field.name() != "message" {
            self.fields.insert(
                field.name().to_string(),
                serde_json::Value::Number(value.into()),
            );
        }
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        if field.name() != "message" {
            if let Some(num) = serde_json::Number::from_f64(value) {
                self.fields
                    .insert(field.name().to_string(), serde_json::Value::Number(num));
            }
        }
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        if field.name() != "message" {
            self.fields
                .insert(field.name().to_string(), serde_json::Value::Bool(value));
        }
    }
}
