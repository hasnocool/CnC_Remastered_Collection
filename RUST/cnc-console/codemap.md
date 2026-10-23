# RUST/cnc-console/

## Responsibility
Real-time log and event monitor GUI for C&C Remastered Collection. Egui/eframe desktop application displaying structured logs (via cnc-logs) and game events (via cnc-events) with filtering, search, and live tail.

## Design Patterns
- Egui Immediate Mode UI: Frame-by-frame UI construction, reactive to log/event streams
- Crossbeam Channels: crossbeam-channel for log/event ingestion from background tasks
- Config-Driven: ConsoleConfig struct with CLI overrides (clap), window params, log level
- Headless Mode: --headless flag for CI/automation without window
- Async Runtime: Tokio for async log/event processing, ctrlc for graceful shutdown

## Data & Control Flow
1. Config Parse: clap parses Args to ConsoleConfig
2. Log/Event Subscription: Subscribes to cnc-logs global logger, cnc-events global event bus
3. UI Loop: eframe::run_native to cnc_console::run_console(config) to per-frame update()
4. Ingestion: Background tasks push log records/events via crossbeam-channel to UI thread
5. Display: Filterable table (logs), event timeline (events), search, level filter, pause/resume
6. Shutdown: ctrlc handler sets shutdown flag, flushes buffers

## Public API
- Binary: cnc-console - CLI args: width, height, title, log-level, headless
- Library: ConsoleConfig, run_console(config) -> anyhow::Result<()>

## Integration Points
- Dependencies: cnc-logs, cnc-events, eframe, egui, winit, tokio, parking_lot, once_cell, chrono, uuid, crossbeam-channel, clap, config, dotenv, ctrlc, epi
- Consumed by: Developers, QA, live ops monitoring
- Data Sources: cnc-logs (tracing subscriber), cnc-events (EventBus)

## Key Files
- src/main.rs - Binary entry, clap config, eframe bootstrap
- src/lib.rs - ConsoleConfig, run_console(), UI implementation
- Cargo.toml - Binary + library dual, eframe/egui dependencies

