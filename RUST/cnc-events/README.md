# cnc-events

Event system and event tracking for C&C Remastered Collection.

This crate provides a comprehensive event system for game events, including:

- **Event definitions** - Strongly-typed events with categories, priorities, and structured data
- **Event bus** - High-performance publish/subscribe with filtering
- **Event handlers** - Type-safe handler registration and dispatch
- **Event sources** - Convenient event emission from game systems
- **Event streaming** - Real-time event streaming for monitoring (cnc-console)
- **Event replay** - Record and replay events for debugging and analysis

## Features

- Zero-cost event definitions with Serde serialization
- High-performance lock-free event bus using crossbeam-channel
- Type-safe event handling with automatic downcasting
- Flexible filtering by category, priority, source, and tags
- Real-time streaming with backpressure and rate limiting
- Event replay with variable speed control
- Integration with cnc-logs for structured logging
- Global event bus for application-wide event distribution

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
cnc-events = { path = "../cnc-events" }
```

Basic usage:

```rust
use cnc_events::{EventBus, GameEvent, EventData, EventCategory, EventPriority};

// Create an event bus
let bus = EventBus::new(10000);

// Subscribe to events
let mut subscription = bus.subscribe();

// Publish a game event
let event = GameEvent::new("unit_created", EventData::structured(
    [("unit_type".to_string(), serde_json::json!("Tank")),
     ("player".to_string(), serde_json::json!(1))]
    .into_iter().collect()
));
bus.publish(event).unwrap();

// Receive events
while let Ok(event) = subscription.try_recv() {
    println!("Event: {} - {}", event.name(), event.category());
}
```

Using event sources:

```rust
use cnc_events::{EventBus, EventSource, EventCategory};
use std::sync::Arc;

let bus = Arc::new(EventBus::new(10000));
let source = EventSource::new(bus.clone(), "game_engine");

// Emit events easily
source.emit_message(EventCategory::Gameplay, "building_placed", "Power Plant placed").unwrap();
source.emit_structured(EventCategory::Network, "player_joined", 
    [("player_id", serde_json::json!(42)), ("name", serde_json::json!("Player1"))]
    .into_iter().collect()
).unwrap();
```

Event handlers:

```rust
use cnc_events::{HandlerRegistry, GameEvent, EventFilter, EventCategory};

let registry = HandlerRegistry::new();

// Register typed handler
registry.register::<GameEvent>(None, |event| {
    println!("Game event: {}", event.name());
});

// Register filtered handler
let filter = EventFilter::new().with_category(EventCategory::Network);
registry.register::<GameEvent>(Some(filter), |event| {
    println!("Network event: {}", event.name());
});

// Dispatch events
registry.dispatch(&Box::new(GameEvent::new("test", EventData::message("test"))));
```

Event streaming for cnc-console:

```rust
use cnc_events::{EventBus, EventStream, StreamConfig, StreamFilter};
use std::sync::Arc;

let bus = Arc::new(EventBus::new(10000));
let stream = EventStream::from_bus(
    bus,
    StreamConfig::default(),
    StreamFilter::default(),
);

// Monitor events in real-time
loop {
    if let Some(event) = stream.try_next() {
        println!("[{}] {}: {}", event.category, event.source, event.event_json);
    }
    std::thread::sleep(std::time::Duration::from_millis(16));
}
```

Event replay:

```rust
use cnc_events::{ReplayRecorder, ReplayConfig, GameEvent, EventData};
use std::sync::Arc;

let mut recorder = ReplayRecorder::new(10000);

// Record events during gameplay
recorder.record(Box::new(GameEvent::new("unit_moved", EventData::structured(...))));

// Later, replay them
let replay = recorder.into_replay(ReplayConfig {
    speed: 2.0, // 2x speed
    ..Default::default()
});

let result = replay.run();
println!("Replayed {} events", result.events_replayed);
```

## Modules

- `bus` - Event bus for publish/subscribe
- `categories` - Event categories and priorities
- `definitions` - Core event types and traits
- `handler` - Event handler registry
- `source` - Event emission helpers
- `streaming` - Real-time event streaming
- `replay` - Event recording and replay

## Documentation

- CHANGELOG.md records releases.
- codemap.md for architecture docs.

## CI / Git

- Build/test enforced via Forgejo (local.lan) using ~/APIKEYS.md credentials.
- All pushes must pass cargo fmt, clippy, test, SemVer check.
