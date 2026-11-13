## Changelog

All notable changes to this crate will be documented here following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format and [SemVer](https://semver.org/) rules.

### [0.2.0] - 2026-11-13

#### Added
- `app` module: main ConsoleApp with toolbar, status bar, panel management
- `log_panel` module: real-time log display with filtering, search, and details
- `events_panel` module: real-time event display with category/priority/source filters
- `filters` module: unified filter configuration for logs and events
- `settings` module: comprehensive settings panel with persistence
- `styles` module: custom dark/light themes with C&C branding
- `widgets` module: reusable UI components (LogEntryWidget, EventWidget, StatsWidget)
- CLI argument parsing with clap
- Headless mode for CI/testing
- Integration with cnc-logs LogBuffer and cnc-events EventBus
- Global log buffer and event bus for cross-thread monitoring
