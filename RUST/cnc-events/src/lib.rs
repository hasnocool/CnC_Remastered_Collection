//! cnc-events - Event system and event tracking for C&C Remastered Collection
//!
//! This crate provides a comprehensive event system for game events, including:
//! - Event definitions and categories
//! - Event bus for publish/subscribe pattern
//! - Event sourcing and replay
//! - Real-time event streaming for monitoring
//! - Integration with cnc-logs for structured logging

pub mod bus;
pub mod categories;
pub mod definitions;
pub mod handler;
pub mod record;
pub mod replay;
pub mod source;
pub mod streaming;

use std::sync::Arc;

pub use bus::{EventBus, EventBusError, EventSubscription, SubscriptionId};
pub use categories::{EventCategory, EventFilter, EventPriority};
pub use definitions::{
    BoxedEvent, Event, EventData, EventId, EventMetadata, GameEvent, NetworkEvent, SystemEvent,
    UiEvent,
};
pub use handler::{EventHandler, EventHandlerFn, HandlerRegistry};
pub use record::EventRecord;
pub use replay::{EventReplay, ReplayConfig, ReplaySpeed};
pub use source::{EventSource, EventSourceBuilder};
pub use streaming::{EventStream, StreamConfig, StreamFilter};

pub use chrono::{DateTime, Utc};
/// Re-export commonly used types
pub use cnc_logs::{LogBuffer, LogLevel};
pub use uuid::Uuid;

/// Global event bus for application-wide event distribution
static GLOBAL_EVENT_BUS: once_cell::sync::OnceCell<Arc<EventBus>> =
    once_cell::sync::OnceCell::new();

/// Initialize the global event bus
pub fn init_global_event_bus(bus: EventBus) -> Result<(), EventBusError> {
    GLOBAL_EVENT_BUS
        .set(Arc::new(bus))
        .map_err(|_| EventBusError::AlreadyInitialized)
}

/// Get the global event bus
pub fn global_event_bus() -> Option<Arc<EventBus>> {
    GLOBAL_EVENT_BUS.get().cloned()
}

/// Convenience function to create a default event bus
pub fn create_default_event_bus() -> EventBus {
    EventBus::new(10000)
}

/// Create an event bus with custom capacity
pub fn create_event_bus(capacity: usize) -> EventBus {
    EventBus::new(capacity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus_basic() {
        let bus = EventBus::new(100);
        let sub = bus.subscribe();

        let event = GameEvent::new("test_event", EventData::message("Test event data"));
        bus.publish(event).unwrap();

        let received = sub.try_recv().unwrap();
        assert_eq!(received.name(), "test_event");
    }

    #[test]
    fn test_event_categories() {
        assert_eq!(EventCategory::Gameplay as u8, 1);
        assert_eq!(EventCategory::System as u8, 2);
        assert_eq!(EventCategory::Network as u8, 3);
        assert_eq!(EventCategory::UI as u8, 4);
        assert_eq!(EventCategory::Audio as u8, 5);
        assert_eq!(EventCategory::Render as u8, 6);
        assert_eq!(EventCategory::AI as u8, 7);
        assert_eq!(EventCategory::Debug as u8, 8);
    }

    #[test]
    fn test_event_priority() {
        assert!(EventPriority::Critical > EventPriority::High);
        assert!(EventPriority::High > EventPriority::Normal);
        assert!(EventPriority::Normal > EventPriority::Low);
        assert!(EventPriority::Low > EventPriority::Background);
    }
}
