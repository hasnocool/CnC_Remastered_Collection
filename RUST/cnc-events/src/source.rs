//! Event source for generating events

use crate::{
    Event, EventBus, EventBusError, EventCategory, EventData, GameEvent, NetworkEvent, SystemEvent,
    UiEvent,
};
use std::sync::Arc;

/// Event source for creating and publishing events
pub struct EventSource {
    bus: Arc<EventBus>,
    source_name: String,
}

impl EventSource {
    /// Create a new event source
    pub fn new(bus: Arc<EventBus>, source_name: &str) -> Self {
        Self {
            bus,
            source_name: source_name.to_string(),
        }
    }

    /// Get the source name
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Get the underlying event bus
    pub fn bus(&self) -> &Arc<EventBus> {
        &self.bus
    }

    /// Publish a generic game event
    pub fn publish_game_event(&self, name: &str, data: EventData) -> Result<(), EventBusError> {
        let event = GameEvent::with_source(name, data, &self.source_name);
        self.bus.publish(event)
    }

    /// Publish a system event
    pub fn publish_system_event(&self, name: &str, data: EventData) -> Result<(), EventBusError> {
        let event = SystemEvent::new(name, data);
        self.bus.publish(event)
    }

    /// Publish a network event
    pub fn publish_network_event(&self, name: &str, data: EventData) -> Result<(), EventBusError> {
        let event = NetworkEvent::new(name, data);
        self.bus.publish(event)
    }

    /// Publish a UI event
    pub fn publish_ui_event(&self, name: &str, data: EventData) -> Result<(), EventBusError> {
        let event = UiEvent::new(name, data);
        self.bus.publish(event)
    }

    /// Publish a custom event
    pub fn publish<E: Event + 'static>(&self, event: E) -> Result<(), EventBusError> {
        self.bus.publish(event)
    }

    /// Emit a simple message event
    pub fn emit_message(
        &self,
        category: EventCategory,
        name: &str,
        message: &str,
    ) -> Result<(), EventBusError> {
        let data = EventData::message(message);
        match category {
            EventCategory::Gameplay => self.publish_game_event(name, data),
            EventCategory::System => self.publish_system_event(name, data),
            EventCategory::Network => self.publish_network_event(name, data),
            EventCategory::UI => self.publish_ui_event(name, data),
            _ => self.publish(GameEvent::with_source(name, data, &self.source_name)),
        }
    }

    /// Emit a structured event
    pub fn emit_structured(
        &self,
        category: EventCategory,
        name: &str,
        data: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<(), EventBusError> {
        let data = EventData::structured(data);
        match category {
            EventCategory::Gameplay => self.publish_game_event(name, data),
            EventCategory::System => self.publish_system_event(name, data),
            EventCategory::Network => self.publish_network_event(name, data),
            EventCategory::UI => self.publish_ui_event(name, data),
            _ => self.publish(GameEvent::with_source(name, data, &self.source_name)),
        }
    }

    /// Emit an error event
    pub fn emit_error(
        &self,
        name: &str,
        message: &str,
        code: Option<i32>,
    ) -> Result<(), EventBusError> {
        let data = EventData::error(message, code);
        self.publish_system_event(name, data)
    }

    /// Emit a progress event
    pub fn emit_progress(
        &self,
        name: &str,
        current: u64,
        total: u64,
        message: Option<String>,
    ) -> Result<(), EventBusError> {
        let data = EventData::progress(current, total, message);
        self.publish_system_event(name, data)
    }

    /// Emit a metric event
    pub fn emit_metric(
        &self,
        name: &str,
        metric_name: &str,
        value: f64,
        unit: Option<String>,
    ) -> Result<(), EventBusError> {
        let data = EventData::metric(metric_name, value, unit);
        self.publish_system_event(name, data)
    }
}

/// Builder for event sources
pub struct EventSourceBuilder {
    bus: Option<Arc<EventBus>>,
    source_name: String,
}

impl EventSourceBuilder {
    pub fn new(source_name: &str) -> Self {
        Self {
            bus: None,
            source_name: source_name.to_string(),
        }
    }

    pub fn bus(mut self, bus: Arc<EventBus>) -> Self {
        self.bus = Some(bus);
        self
    }

    pub fn build(self) -> Result<EventSource, String> {
        let bus = self.bus.ok_or("Event bus is required")?;
        Ok(EventSource::new(bus, &self.source_name))
    }
}

/// Trait for types that can emit events
pub trait EventEmitter {
    fn source(&self) -> &EventSource;

    fn emit(&self, name: &str, data: EventData) -> Result<(), EventBusError> {
        self.source().publish_game_event(name, data)
    }

    fn emit_message(
        &self,
        category: EventCategory,
        name: &str,
        message: &str,
    ) -> Result<(), EventBusError> {
        self.source().emit_message(category, name, message)
    }

    fn emit_structured(
        &self,
        category: EventCategory,
        name: &str,
        data: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<(), EventBusError> {
        self.source().emit_structured(category, name, data)
    }

    fn emit_error(
        &self,
        name: &str,
        message: &str,
        code: Option<i32>,
    ) -> Result<(), EventBusError> {
        self.source().emit_error(name, message, code)
    }

    fn emit_progress(
        &self,
        name: &str,
        current: u64,
        total: u64,
        message: Option<String>,
    ) -> Result<(), EventBusError> {
        self.source().emit_progress(name, current, total, message)
    }

    fn emit_metric(
        &self,
        name: &str,
        metric_name: &str,
        value: f64,
        unit: Option<String>,
    ) -> Result<(), EventBusError> {
        self.source().emit_metric(name, metric_name, value, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventBus;

    #[test]
    fn test_event_source() {
        let bus = Arc::new(EventBus::new(100));
        let source = EventSource::new(bus.clone(), "test_source");

        assert_eq!(source.source_name(), "test_source");

        // Subscribe first: the bus is a live channel and does not retain history.
        let sub = bus.subscribe();
        source
            .emit_message(crate::EventCategory::Gameplay, "test_event", "Hello")
            .unwrap();

        let event = sub.try_recv().unwrap();
        assert_eq!(event.name(), "test_event");
    }

    #[test]
    fn test_event_source_builder() {
        let bus = Arc::new(EventBus::new(100));
        let source = EventSourceBuilder::new("builder_test")
            .bus(bus.clone())
            .build()
            .unwrap();

        let sub = bus.subscribe();
        source
            .emit_message(crate::EventCategory::System, "startup", "Starting up")
            .unwrap();

        let event = sub.try_recv().unwrap();
        assert_eq!(event.name(), "startup");
    }
}
