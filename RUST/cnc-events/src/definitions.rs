//! Event definitions and core types

use crate::{EventCategory, EventPriority};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Unique identifier for an event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub id: EventId,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    pub category: EventCategory,
    pub priority: EventPriority,
    pub source: String,
    #[serde(with = "smallvec_serde")]
    pub tags: SmallVec<[String; 4]>,
    pub correlation_id: Option<EventId>,
    pub causation_id: Option<EventId>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
}

/// Serde helper for SmallVec
mod smallvec_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use smallvec::SmallVec;

    pub fn serialize<S, T, const N: usize>(
        value: &SmallVec<[T; N]>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        serializer.collect_seq(value.iter())
    }

    pub fn deserialize<'de, D, T, const N: usize>(
        deserializer: D,
    ) -> Result<SmallVec<[T; N]>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        struct SmallVecVisitor<T> {
            marker: std::marker::PhantomData<T>,
        }

        impl<'de, T> serde::de::Visitor<'de> for SmallVecVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = SmallVec<[T; 4]>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut vec = SmallVec::new();
                while let Some(item) = seq.next_element()? {
                    vec.push(item);
                }
                Ok(vec)
            }
        }

        deserializer.deserialize_seq(SmallVecVisitor {
            marker: std::marker::PhantomData,
        })
    }
}

impl EventMetadata {
    pub fn new(category: EventCategory, source: &str) -> Self {
        Self {
            id: EventId::new(),
            timestamp: Utc::now(),
            category,
            priority: EventPriority::Normal,
            source: source.to_string(),
            tags: SmallVec::new(),
            correlation_id: None,
            causation_id: None,
            user_id: None,
            session_id: None,
        }
    }

    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }

    pub fn with_correlation_id(mut self, id: EventId) -> Self {
        self.correlation_id = Some(id);
        self
    }

    pub fn with_causation_id(mut self, id: EventId) -> Self {
        self.causation_id = Some(id);
        self
    }
}

/// Event data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventData {
    /// Empty event with no data
    Empty,
    /// String message
    Message(String),
    /// Structured key-value data
    Structured(HashMap<String, serde_json::Value>),
    /// Binary data
    Binary(Vec<u8>),
    /// Error event
    Error { message: String, code: Option<i32> },
    /// Progress event
    Progress {
        current: u64,
        total: u64,
        message: Option<String>,
    },
    /// Metric event
    Metric {
        name: String,
        value: f64,
        unit: Option<String>,
    },
    /// Custom typed data (serialized as JSON)
    Custom(serde_json::Value),
}

impl EventData {
    pub fn message(msg: &str) -> Self {
        Self::Message(msg.to_string())
    }

    pub fn structured(data: HashMap<String, serde_json::Value>) -> Self {
        Self::Structured(data)
    }

    pub fn error(message: &str, code: Option<i32>) -> Self {
        Self::Error {
            message: message.to_string(),
            code,
        }
    }

    pub fn progress(current: u64, total: u64, message: Option<String>) -> Self {
        Self::Progress {
            current,
            total,
            message,
        }
    }

    pub fn metric(name: &str, value: f64, unit: Option<String>) -> Self {
        Self::Metric {
            name: name.to_string(),
            value,
            unit,
        }
    }
}

/// Core event trait
pub trait Event: Send + Sync + 'static {
    fn id(&self) -> EventId;
    fn name(&self) -> &str;
    fn category(&self) -> EventCategory;
    fn priority(&self) -> EventPriority;
    fn timestamp(&self) -> DateTime<Utc>;
    fn source(&self) -> &str;
    fn data(&self) -> &EventData;
    fn metadata(&self) -> &EventMetadata;
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Base event implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEvent {
    pub metadata: EventMetadata,
    pub name: String,
    pub data: EventData,
}

impl BaseEvent {
    pub fn new(name: &str, category: EventCategory, data: EventData, source: &str) -> Self {
        Self {
            metadata: EventMetadata::new(category, source),
            name: name.to_string(),
            data,
        }
    }

    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.metadata.priority = priority;
        self
    }
}

impl Event for BaseEvent {
    fn id(&self) -> EventId {
        self.metadata.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn category(&self) -> EventCategory {
        self.metadata.category
    }

    fn priority(&self) -> EventPriority {
        self.metadata.priority
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.metadata.timestamp
    }

    fn source(&self) -> &str {
        &self.metadata.source
    }

    fn data(&self) -> &EventData {
        &self.data
    }

    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Game-specific events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub base: BaseEvent,
}

impl GameEvent {
    pub fn new(name: &str, data: EventData) -> Self {
        Self {
            base: BaseEvent::new(name, EventCategory::Gameplay, data, "game"),
        }
    }

    pub fn with_source(name: &str, data: EventData, source: &str) -> Self {
        Self {
            base: BaseEvent::new(name, EventCategory::Gameplay, data, source),
        }
    }

    pub fn with_category(name: &str, data: EventData, category: EventCategory) -> Self {
        Self {
            base: BaseEvent::new(name, category, data, "game"),
        }
    }
}

impl Event for GameEvent {
    fn id(&self) -> EventId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn category(&self) -> EventCategory {
        self.base.category()
    }

    fn priority(&self) -> EventPriority {
        self.base.priority()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.base.timestamp()
    }

    fn source(&self) -> &str {
        self.base.source()
    }

    fn data(&self) -> &EventData {
        self.base.data()
    }

    fn metadata(&self) -> &EventMetadata {
        self.base.metadata()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// System events (engine, initialization, shutdown, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEvent {
    pub base: BaseEvent,
}

impl SystemEvent {
    pub fn new(name: &str, data: EventData) -> Self {
        Self {
            base: BaseEvent::new(name, EventCategory::System, data, "system"),
        }
    }

    pub fn startup() -> Self {
        Self::new("system_startup", EventData::message("System starting up"))
    }

    pub fn shutdown() -> Self {
        Self::new(
            "system_shutdown",
            EventData::message("System shutting down"),
        )
    }

    pub fn error(message: &str) -> Self {
        Self::new("system_error", EventData::error(message, None))
    }
}

impl Event for SystemEvent {
    fn id(&self) -> EventId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn category(&self) -> EventCategory {
        self.base.category()
    }

    fn priority(&self) -> EventPriority {
        self.base.priority()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.base.timestamp()
    }

    fn source(&self) -> &str {
        self.base.source()
    }

    fn data(&self) -> &EventData {
        self.base.data()
    }

    fn metadata(&self) -> &EventMetadata {
        self.base.metadata()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Network events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEvent {
    pub base: BaseEvent,
}

impl NetworkEvent {
    pub fn new(name: &str, data: EventData) -> Self {
        Self {
            base: BaseEvent::new(name, EventCategory::Network, data, "network"),
        }
    }

    pub fn connected(address: &str) -> Self {
        Self::new(
            "network_connected",
            EventData::structured(
                [(
                    "address".to_string(),
                    serde_json::Value::String(address.to_string()),
                )]
                .into_iter()
                .collect(),
            ),
        )
    }

    pub fn disconnected(address: &str) -> Self {
        Self::new(
            "network_disconnected",
            EventData::structured(
                [(
                    "address".to_string(),
                    serde_json::Value::String(address.to_string()),
                )]
                .into_iter()
                .collect(),
            ),
        )
    }

    pub fn packet_received(size: usize) -> Self {
        Self::new(
            "network_packet_received",
            EventData::metric("packet_size", size as f64, Some("bytes".to_string())),
        )
    }

    pub fn packet_sent(size: usize) -> Self {
        Self::new(
            "network_packet_sent",
            EventData::metric("packet_size", size as f64, Some("bytes".to_string())),
        )
    }
}

impl Event for NetworkEvent {
    fn id(&self) -> EventId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn category(&self) -> EventCategory {
        self.base.category()
    }

    fn priority(&self) -> EventPriority {
        self.base.priority()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.base.timestamp()
    }

    fn source(&self) -> &str {
        self.base.source()
    }

    fn data(&self) -> &EventData {
        self.base.data()
    }

    fn metadata(&self) -> &EventMetadata {
        self.base.metadata()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// UI events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiEvent {
    pub base: BaseEvent,
}

impl UiEvent {
    pub fn new(name: &str, data: EventData) -> Self {
        Self {
            base: BaseEvent::new(name, EventCategory::UI, data, "ui"),
        }
    }

    pub fn button_clicked(button_id: &str) -> Self {
        Self::new(
            "ui_button_clicked",
            EventData::structured(
                [(
                    "button_id".to_string(),
                    serde_json::Value::String(button_id.to_string()),
                )]
                .into_iter()
                .collect(),
            ),
        )
    }

    pub fn screen_changed(from: &str, to: &str) -> Self {
        Self::new(
            "ui_screen_changed",
            EventData::structured(
                [
                    (
                        "from".to_string(),
                        serde_json::Value::String(from.to_string()),
                    ),
                    ("to".to_string(), serde_json::Value::String(to.to_string())),
                ]
                .into_iter()
                .collect(),
            ),
        )
    }
}

impl Event for UiEvent {
    fn id(&self) -> EventId {
        self.base.id()
    }

    fn name(&self) -> &str {
        self.base.name()
    }

    fn category(&self) -> EventCategory {
        self.base.category()
    }

    fn priority(&self) -> EventPriority {
        self.base.priority()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.base.timestamp()
    }

    fn source(&self) -> &str {
        self.base.source()
    }

    fn data(&self) -> &EventData {
        self.base.data()
    }

    fn metadata(&self) -> &EventMetadata {
        self.base.metadata()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Boxed event type for heterogeneous collections
pub type BoxedEvent = Box<dyn Event>;

/// Convert any event to a boxed event
pub fn box_event<E: Event + 'static>(event: E) -> BoxedEvent {
    Box::new(event)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_event() {
        let event = GameEvent::new("unit_created", EventData::message("Tank created"));
        assert_eq!(event.name(), "unit_created");
        assert_eq!(event.category(), EventCategory::Gameplay);
    }

    #[test]
    fn test_system_event() {
        let event = SystemEvent::startup();
        assert_eq!(event.name(), "system_startup");
        assert_eq!(event.category(), EventCategory::System);
    }

    #[test]
    fn test_network_event() {
        let event = NetworkEvent::connected("127.0.0.1:1234");
        assert_eq!(event.name(), "network_connected");
        assert_eq!(event.category(), EventCategory::Network);
    }

    #[test]
    fn test_event_data() {
        let data = EventData::structured(
            [(
                "key".to_string(),
                serde_json::Value::String("value".to_string()),
            )]
            .into_iter()
            .collect(),
        );
        match data {
            EventData::Structured(map) => {
                assert_eq!(
                    map.get("key"),
                    Some(&serde_json::Value::String("value".to_string()))
                );
            }
            _ => panic!("Wrong variant"),
        }
    }
}
