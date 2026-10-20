//! Logging initialization and management

use crate::{CncLogLayer, LogBuffer, LogConfig, LogFilter, LogOutput};
use anyhow::Result;
use std::sync::Arc;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{prelude::*, util::SubscriberInitExt, EnvFilter, Registry};

/// Guard that keeps the logging system alive
pub struct LoggingGuard {
    _guards: Vec<WorkerGuard>,
    _buffer: Option<Arc<LogBuffer>>,
}

impl LoggingGuard {
    /// Create a new logging guard
    pub fn new(guards: Vec<WorkerGuard>, buffer: Option<Arc<LogBuffer>>) -> Self {
        Self {
            _guards: guards,
            _buffer: buffer,
        }
    }
}

/// Initialize global logging with the given configuration
pub fn init_global_logger(config: LogConfig) -> Result<LoggingGuard> {
    init_logging(config)
}

/// Initialize logging with the given configuration
pub fn init_logging(config: LogConfig) -> Result<LoggingGuard> {
    let filter = LogFilter::from_directives(
        &config
            .target_levels
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let env_filter = filter.to_env_filter();
    let level_filter: tracing_core::metadata::LevelFilter = config.level.into();
    let env_filter = env_filter.add_directive(level_filter.into());

    // Build subscriber with all layers - just stdout for now
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(config.colors)
        .with_thread_names(config.thread_names)
        .with_file(config.file_line)
        .with_line_number(config.file_line)
        .with_target(true)
        .with_level(true)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new(
            "%Y-%m-%d %H:%M:%S%.3f".into(),
        ))
        .with_writer(std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(CncLogLayer::new(config.clone()))
        .with(fmt_layer);

    subscriber.init();

    Ok(LoggingGuard::new(Vec::new(), None))
}
