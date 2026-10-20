//! cnc-logs - Structured logging and log management for C&C Remastered Collection
//!
//! This crate provides a unified logging infrastructure built on top of `tracing` with
//! support for structured logging, log rotation, filtering, and in-memory log buffers
//! for real-time monitoring via the cnc-console crate.

pub mod buffer;
pub mod config;
pub mod filter;
pub mod init;
pub mod layer;
pub mod rotation;
pub mod structured;

use std::sync::Arc;

pub use buffer::{LogBuffer, LogEntry, LogLevel};
pub use config::{LogConfig, LogOutput, RotationConfig};
pub use filter::{FilterDirective, LogFilter};
pub use init::{init_global_logger, init_logging, LoggingGuard};
pub use layer::CncLogLayer;
pub use rotation::LogRotator;
pub use structured::{StructuredLog, StructuredLogBuilder};

/// Re-export commonly used tracing types
pub use tracing::{debug, error, info, trace, warn, Level, Metadata, Subscriber};
pub use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};

/// Global log buffer for real-time monitoring
static GLOBAL_LOG_BUFFER: once_cell::sync::OnceCell<Arc<LogBuffer>> =
    once_cell::sync::OnceCell::new();

/// Initialize the global log buffer
pub fn init_global_log_buffer(buffer: LogBuffer) -> Result<(), LogBufferError> {
    GLOBAL_LOG_BUFFER
        .set(Arc::new(buffer))
        .map_err(|_| LogBufferError::AlreadyInitialized)
}

/// Get the global log buffer for monitoring
pub fn global_log_buffer() -> Option<Arc<LogBuffer>> {
    GLOBAL_LOG_BUFFER.get().cloned()
}

/// Errors related to log buffer management
#[derive(Debug, thiserror::Error)]
pub enum LogBufferError {
    #[error("Global log buffer already initialized")]
    AlreadyInitialized,
    #[error("Global log buffer not initialized")]
    NotInitialized,
}

/// Convenience function to initialize logging with default configuration
pub fn init_default_logging() -> Result<LoggingGuard, anyhow::Error> {
    let config = LogConfig::default();
    init_logging(config)
}

/// Initialize logging with a custom configuration and return a guard that
/// keeps the logging system alive until dropped
pub fn init_logging_with_buffer(
    config: LogConfig,
    buffer: LogBuffer,
) -> Result<LoggingGuard, anyhow::Error> {
    init_global_log_buffer(buffer.clone())?;
    init_logging(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_log_buffer_basic() {
        let buffer = LogBuffer::new(100);
        buffer.push(LogEntry::new(
            LogLevel::Info,
            "test-target",
            "Test message",
            None,
        ));

        let entries = buffer.drain();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].level, LogLevel::Info);
        assert_eq!(entries[0].message, "Test message");
    }

    #[test]
    fn test_log_buffer_capacity() {
        let buffer = LogBuffer::new(3);
        for i in 0..5 {
            buffer.push(LogEntry::new(
                LogLevel::Debug,
                "test",
                &format!("Message {i}"),
                None,
            ));
        }
        let entries = buffer.drain();
        assert_eq!(entries.len(), 3);
        // Should keep the last 3 entries
        assert_eq!(entries[0].message, "Message 2");
        assert_eq!(entries[2].message, "Message 4");
    }

    #[test]
    fn test_structured_log() {
        let log = StructuredLogBuilder::new()
            .level(LogLevel::Info)
            .target("test")
            .message("Structured message")
            .field("key1", "value1")
            .field("key2", 42)
            .build();

        assert_eq!(log.level, LogLevel::Info);
        assert_eq!(log.target, "test");
        assert_eq!(log.message, "Structured message");
        assert_eq!(
            log.fields.get("key1"),
            Some(&serde_json::Value::String("value1".to_string()))
        );
        assert_eq!(
            log.fields.get("key2"),
            Some(&serde_json::Value::Number(42.into()))
        );
    }
}
