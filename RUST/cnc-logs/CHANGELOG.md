## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13

#### Added
- `buffer` module: thread-safe bounded in-memory log buffer with real-time monitoring support
- `config` module: flexible logging configuration with multiple output targets
- `filter` module: per-target and global log level filtering with directive parsing
- `init` module: logging initialization with guard pattern for lifecycle management
- `layer` module: custom `tracing` layer for structured log capture
- `rotation` module: log file rotation with size/time-based policies and compression
- `structured` module: structured logging with JSON serialization and builder pattern
- Re-exports of `tracing` macros (info!, debug!, warn!, error!, trace!)
- Global log buffer for cross-crate monitoring via cnc-console
- Serde serialization for all log types
