# RUST/cnc-logs/

## Responsibility
Structured logging and log management for C&C Remastered Collection. Tracing-based subscriber with file rotation, JSON formatting, global accessor, and log event correlation with cnc-events.

## Design Patterns
- Tracing Subscriber: Layered tracing_subscriber with fmt, env-filter, chrono, file appender
- File Rotation: tracing-appender rolling files (daily/hourly), configurable retention
- JSON Output: Structured JSON lines for log aggregation (ELK, Loki, etc.)
- Global Registry: GLOBAL_LOGGER: OnceLock<LoggerHandle> for cross-crate access
- Correlation IDs: Log spans carry correlation_id matching cnc-events for traceability
- Level Control: RUST_LOG / env-filter runtime level adjustment

## Data & Control Flow
1. Init: Logger::init(config) to builds subscriber layers to sets global to installs as default
2. Config: LogConfig - file path, rotation, format (json/text), level filter, max files
3. Emit: Standard tracing::info!(), warn!(), error!(), debug!(), trace!() macros
4. Correlation: tracing::info_span!(operation, correlation_id = %id) links logs to events
5. File Rotation: Background worker rolls files per config, compresses old (flate2)
6. Shutdown: Logger::shutdown() flushes, drops guard, releases file handles

## Public API
- Logger: init(config), shutdown(), flush()
- LogConfig: file_path, rotation, format, level, max_files, max_size_mb
- Global: global_logger() -> Option<LoggerHandle>
- Re-exports: tracing macros (info, warn, error, debug, trace, span)
- Correlation: with_correlation_id(id, closure) helper

## Integration Points
- Dependencies: cnc-core, tracing, tracing-subscriber, tracing-appender, tracing-core, flate2, parking_lot, once_cell, chrono, uuid, anyhow, thiserror, serde
- Consumed by: All crates (cnc-tiberian-dawn, cnc-red-alert, cnc-launcher, cnc-console, cnc-audio, cnc-events, cnc-ai, cnc-renderer)
- Event Correlation: cnc-events GameEvent carries correlation_id matching log spans

## Key Files
- src/lib.rs - Logger, LogConfig, global accessor, tracing setup
- src/rotation.rs - File rotation logic, compression
- src/json_format.rs - JSON log formatter

