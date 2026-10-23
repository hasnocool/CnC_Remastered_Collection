# RUST/cnc-events/

## Responsibility
Event system and event tracking for C&C Remastered Collection. Provides typed GameEvent with categories, priorities, structured data; EventBus for publish/subscribe with bounded channels; global singleton accessor for cross-crate coordination.

## Design Patterns
- Typed Events: GameEvent with name: String, category: EventCategory, priority: EventPriority, data: EventData, timestamp, correlation_id
- Category Taxonomy: Gameplay, System, Network, UI, Audio, Render, AI, Debug (u8 discriminant)
- Priority Levels: Critical > High > Normal > Low > Background
- Bounded MPSC Bus: EventBus uses crossbeam-channel with configurable capacity, backpressure via try_send
- Global Singleton: GLOBAL_EVENT_BUS: OnceLock<Arc<EventBus>> for cross-crate access without explicit wiring
- Structured Data: EventData enum (Message, Json, Binary, Metric, Error) for flexible payloads

## Data & Control Flow
1. Bus Creation: EventBus::new(capacity) or create_default_event_bus() (10k capacity)
2. Global Registration: set_global_event_bus(bus) - called once at app init
3. Publish: bus.publish(event) or GameEvent::new(...).publish() - non-blocking, drops on full
4. Subscribe: bus.subscribe() to Receiver<GameEvent> - multiple subscribers supported
5. Receive: sub.try_recv() / sub.recv() - non-blocking / blocking
6. Convenience: global_event_bus() returns Option<Arc<EventBus>> for optional participation

## Public API
- GameEvent: new(name, data), name(), category(), priority(), data(), timestamp(), correlation_id()
- EventCategory: Gameplay, System, Network, UI, Audio, Render, AI, Debug
- EventPriority: Critical, High, Normal, Low, Background
- EventData: Message(String), Json(serde_json::Value), Binary(Vec<u8>), Metric{f64}, Error(String)
- EventBus: new(capacity), publish(), subscribe(), try_publish()
- Globals: set_global_event_bus(), global_event_bus(), create_default_event_bus(), create_event_bus(capacity)

## Integration Points
- Dependencies: cnc-core, cnc-logs, tokio, parking_lot, once_cell, chrono, uuid, crossbeam-channel, dashmap, smallvec, anyhow, thiserror, serde
- Consumed by: cnc-console (UI), cnc-tiberian-dawn, cnc-red-alert (gameplay events), cnc-launcher (system events), cnc-audio (audio events), cnc-ai (AI events)
- Logging Bridge: Integrates with cnc-logs for structured log correlation

## Key Files
- src/lib.rs - All types, EventBus, globals, re-exports
- EventCategory/EventPriority enums with explicit discriminants for FFI stability

