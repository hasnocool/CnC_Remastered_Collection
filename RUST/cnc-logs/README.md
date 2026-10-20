# cnc-logs

Structured logging and log management for C&C Remastered Collection.

This crate provides a unified logging infrastructure built on top of `tracing` with support for:

- **Structured logging** - JSON-formatted logs with typed fields
- **In-memory log buffers** - Real-time log capture for monitoring via cnc-console
- **Log rotation** - Automatic file rotation with compression
- **Flexible filtering** - Per-target and global log level control
- **Multiple outputs** - stdout, stderr, files, rotating files, syslog, Windows Event Log
- **Integration with tracing** - Compatible with the standard tracing ecosystem

## Features

- `tracing`-based structured logging
- Thread-safe in-memory log buffer (bounded, with drop counting)
- Real-time log streaming for GUI monitoring
- Configurable log rotation (size-based, time-based, compression)
- Environment variable configuration support
- Serde serialization for log entries
- Zero-cost abstraction when logging is disabled

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
cnc-logs = { path = "../cnc-logs" }
```

Basic initialization:

```rust
use cnc_logs::{init_default_logging, LogConfig, LogOutput};

fn main() -> anyhow::Result<()> {
    // Simple initialization
    let _guard = cnc_logs::init_default_logging()?;

    // Or with custom config
    let config = LogConfig {
        level: cnc_logs::LogLevelConfig::Debug,
        outputs: vec![
            LogOutput::Stdout,
            LogOutput::File { path: "logs/app.log".into() },
        ],
        ..Default::default()
    };
    let _guard = cnc_logs::init_logging(config)?;

    // Logging macros (re-exported from tracing)
    cnc_logs::info!("Application started");
    cnc_logs::debug!(user_id = 42, "User logged in");
    cnc_logs::error!(error = %err, "Failed to connect");

    Ok(())
}
```

Structured logging:

```rust
use cnc_logs::{StructuredLogBuilder, LogLevel};

let log = StructuredLogBuilder::new()
    .level(LogLevel::Info)
    .target("my_app")
    .message("User action")
    .field("user_id", 123)
    .field("action", "login")
    .field("success", true)
    .build();

println!("{}", log.to_json()?);
```

Real-time monitoring with cnc-console:

```rust
use cnc_logs::{LogBuffer, init_logging_with_buffer, LogConfig};

let buffer = LogBuffer::new(10000);
let _guard = init_logging_with_buffer(LogConfig::default(), buffer.clone())?;

// In another thread or async task, monitor the buffer
loop {
    let entries = buffer.drain();
    for entry in entries {
        println!("[{}] {}: {}", entry.level, entry.target, entry.message);
    }
    std::thread::sleep(std::time::Duration::from_millis(100));
}
```

## Configuration

LogConfig can be loaded from environment variables or config files:

```rust
use cnc_logs::LogConfig;
use std::env;

let config = LogConfig {
    level: cnc_logs::LogLevelConfig::Info,
    outputs: vec![cnc_logs::LogOutput::Stdout],
    structured: true,
    colors: true,
    timestamps: true,
    ..Default::default()
};
```

## Modules

- `buffer` - In-memory log buffer for real-time monitoring
- `config` - Logging configuration types
- `filter` - Log filtering and directives
- `init` - Logging initialization
- `layer` - Custom tracing layer
- `rotation` - Log rotation utilities
- `structured` - Structured logging support

## Documentation

- CHANGELOG.md records releases.
- codemap.md for architecture docs.

## CI / Git

- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
