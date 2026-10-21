# cnc-console

Real-time log and event monitor GUI for C&C Remastered Collection.

This crate provides a standalone GUI application for monitoring logs and events
in real-time. It integrates with cnc-logs and cnc-events to provide a unified
view of application activity.

## Features

- **Real-time log monitoring** - View structured logs as they happen
- **Event streaming** - Watch game events flow through the event bus
- **Advanced filtering** - Filter by level, category, priority, source, tags, and text
- **Customizable layout** - Resizable panels, show/hide sections
- **Dark/Light themes** - Easy on the eyes for long monitoring sessions
- **Statistics dashboard** - Track log/event rates, uptime, dropped messages
- **Export/Import** - Save and load log/event sessions
- **Headless mode** - Run without GUI for automated testing
- **Integration with cnc-logs and cnc-events** - Seamless monitoring of both systems

## Usage

### As a standalone application

```bash
# Build and run
cargo run --bin cnc-console

# With custom window size
cargo run --bin cnc-console -- --width 1600 --height 900

# With debug logging
cargo run --bin cnc-console -- --log-level debug

# Save logs to file
cargo run --bin cnc-console -- --log-to-file --log-file my_session.log

# Headless mode (for CI/testing)
cargo run --bin cnc-console -- --headless
```

### As a library in your application

```rust
use cnc_console::{ConsoleConfig, run_console, run_console_with_buffer, run_console_with_bus};
use cnc_logs::LogBuffer;
use cnc_events::EventBus;
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    // Option 1: Run with default settings (creates its own log buffer and event bus)
    run_console(ConsoleConfig::default())?;

    // Option 2: Run with your existing log buffer
    let log_buffer = Arc::new(LogBuffer::new(10000));
    run_console_with_buffer(log_buffer)?;

    // Option 3: Run with your existing event bus
    let event_bus = Arc::new(EventBus::new(10000));
    run_console_with_bus(event_bus)?;

    // Option 4: Full custom configuration
    let config = ConsoleConfig {
        window_width: 1600.0,
        window_height: 900.0,
        window_title: "My Game Console".to_string(),
        show_logs: true,
        show_events: true,
        show_filters: true,
        show_stats: true,
        theme: egui::Theme::Dark,
        auto_scroll_logs: true,
        auto_scroll_events: true,
        ..Default::default()
    };
    run_console(config)?;

    Ok(())
}
```

### Integrating with your game

To monitor your game's logs and events, share the same LogBuffer and EventBus:

```rust
// In your game initialization
use cnc_logs::{LogBuffer, init_logging_with_buffer, LogConfig};
use cnc_events::EventBus;
use cnc_console::run_console_with_buffer;
use std::sync::Arc;

let log_buffer = Arc::new(LogBuffer::new(50000));
let event_bus = Arc::new(EventBus::new(50000));

// Initialize your game's logging with the shared buffer
let _log_guard = init_logging_with_buffer(LogConfig::default(), log_buffer.clone())?;

// Spawn the console in a separate thread
std::thread::spawn(move || {
    let config = cnc_console::ConsoleConfig {
        log_buffer: Some(log_buffer),
        event_bus: Some(event_bus),
        ..Default::default()
    };
    cnc_console::run_console(config).unwrap();
});

// Your game continues running...
game_loop();
```

## UI Overview

### Toolbar
- **View toggles** - Show/hide Logs, Events, Filters, Statistics
- **Auto-scroll** - Automatically scroll to new entries
- **Clear buttons** - Clear logs or events
- **Settings** - Open settings panel
- **Theme toggle** - Switch between Dark/Light themes

### Logs Panel
- **Level filter** - Show only logs at or above a level (Trace/Debug/Info/Warn/Error)
- **Text search** - Filter by message content, target, or structured fields
- **Display options** - Toggle timestamp, level, target, thread columns
- **Entry details** - Click an entry to expand and view structured fields

### Events Panel
- **Category filter** - Filter by Gameplay, System, Network, UI, Audio, Render, AI, Debug
- **Priority filter** - Filter by Background, Low, Normal, High, Critical
- **Source filter** - Filter by event source name
- **Text search** - Filter by event JSON content
- **Event details** - Click an event to view full JSON and metadata

### Filters Panel
- **Log filters** - Configure global and per-target log levels
- **Event filters** - Configure category, priority, source, and tag filters
- **Advanced options** - Filter syntax help, preset management

### Settings Panel
- **Window** - Size, title
- **Display** - Panel visibility, theme
- **Logs** - Auto-scroll, max entries
- **Events** - Auto-scroll, max entries
- **Stream** - Buffer size, backpressure, rate limiting
- **Data** - Clear, export, import

### Statistics Window
- Total logs/events
- Current rate (per second)
- Uptime
- Dropped message count

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Ctrl+F | Focus search filter |
| Ctrl+L | Clear logs |
| Ctrl+E | Clear events |
| Ctrl+S | Open settings |
| Ctrl+T | Toggle theme |
| Esc | Close popups/deselect |

## Configuration

The console can be configured via:

1. **Command-line arguments** (see --help)
2. **Config file** (JSON, specified with --config)
3. **Runtime UI** (Settings panel)

Config file example:

```json
{
  "window_width": 1600,
  "window_height": 900,
  "window_title": "C&C Console",
  "show_logs": true,
  "show_events": true,
  "show_filters": true,
  "show_settings": false,
  "show_stats": true,
  "theme": "Dark",
  "auto_scroll_logs": true,
  "auto_scroll_events": true,
  "max_log_entries": 10000,
  "max_event_entries": 5000
}
```

## Modules

- `app` - Main application logic and rendering
- `log_panel` - Log display and filtering
- `events_panel` - Event display and filtering
- `filters` - Filter configuration UI
- `settings` - Settings panel
- `styles` - Custom themes and colors
- `widgets` - Reusable UI components

## Documentation

- CHANGELOG.md records releases.
- codemap.md for architecture docs.

## CI / Git

- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
