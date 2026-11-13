## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13

#### Added
- `bus` module: high-performance event bus with crossbeam-channel, filtering, and statistics
- `categories` module: 8 event categories (Gameplay, System, Network, UI, Audio, Render, AI, Debug) and 5 priority levels
- `definitions` module: core Event trait, EventId, EventMetadata, EventData enum, GameEvent, SystemEvent, NetworkEvent, UiEvent
- `handler` module: type-safe HandlerRegistry with typed and global handlers, filtering support
- `source` module: EventSource and EventSourceBuilder for convenient event emission
- `streaming` module: EventStream for real-time monitoring with backpressure, rate limiting, deduplication
- `replay` module: EventReplay with variable speed, seeking, looping; ReplayRecorder for capturing events
- Global event bus for application-wide event distribution
- Serde serialization for all event types
- Integration with cnc-logs for structured logging
