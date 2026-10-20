//! Stable JSON projection of an event for replay and streaming
//!
//! `dyn Event` is deliberately not `serde::Serialize` (the trait is object-safe and
//! has no serialization method), so replay buffers and monitoring streams persist
//! events through this lossless projection of the common `Event` surface instead.

use crate::Event;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Serialized projection of an event, usable for replay and streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventRecord {
    /// Stable event name (`Event::name`).
    pub name: String,
    /// Originating component (`Event::source`).
    pub source: String,
    /// Event family (`Event::category`).
    pub category: crate::EventCategory,
    /// Delivery priority (`Event::priority`).
    pub priority: crate::EventPriority,
    /// Timestamp as RFC 3339 with nanosecond precision.
    pub timestamp: String,
    /// The `EventData` payload as JSON.
    pub data: Value,
}

impl EventRecord {
    /// Project an event onto the serializable common surface.
    pub fn from_event<E: Event + ?Sized>(event: &E) -> Self {
        Self {
            name: event.name().to_string(),
            source: event.source().to_string(),
            category: event.category(),
            priority: event.priority(),
            timestamp: event.timestamp().to_rfc3339(),
            data: serde_json::to_value(event.data()).unwrap_or(Value::Null),
        }
    }

    /// Serialize this projection to a JSON string (never fails; falls back to `null`).
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "null".to_string())
    }

    /// Short human-readable summary used by log sinks and HUD lines.
    pub fn summary(&self) -> String {
        let payload = match &self.data {
            Value::String(s) => s.clone(),
            other => json!(other).to_string(),
        };
        format!("[{}] {}: {}", self.category, self.name, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventData, GameEvent};

    #[test]
    fn test_event_record_round_trip() {
        let event = GameEvent::with_source("unit_produced", EventData::message("rifle"), "td");
        let record = EventRecord::from_event(&event);
        assert_eq!(record.name, "unit_produced");
        assert_eq!(record.source, "td");

        let json = record.to_json();
        let parsed: EventRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, record);
    }

    #[test]
    fn test_event_record_summary() {
        let event = GameEvent::new("mission_outcome", EventData::message("victory"));
        let record = EventRecord::from_event(&event);
        assert!(record.summary().contains("mission_outcome"));
        assert!(record.summary().contains("victory"));
    }
}
