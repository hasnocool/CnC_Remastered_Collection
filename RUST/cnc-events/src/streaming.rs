//! Event streaming for real-time monitoring

use crate::{BoxedEvent, Event, EventBus, EventFilter, SubscriptionId};
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Configuration for event streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    /// Buffer size for the stream
    pub buffer_size: usize,
    /// Enable backpressure handling
    pub backpressure: bool,
    /// Maximum events per second (0 = unlimited)
    pub max_events_per_second: u32,
    /// Include event metadata in stream
    pub include_metadata: bool,
    /// Compress stream data
    pub compress: bool,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            buffer_size: 10000,
            backpressure: true,
            max_events_per_second: 0,
            include_metadata: true,
            compress: false,
        }
    }
}

/// Stream filter configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StreamFilter {
    pub event_filter: EventFilter,
    /// Only include events from these sources
    pub sources: Vec<String>,
    /// Only include events with these tags
    pub tags: Vec<String>,
    /// Minimum time between events from same source (ms)
    pub dedup_interval_ms: u64,
}

/// Event stream for real-time monitoring
pub struct EventStream {
    config: StreamConfig,
    filter: StreamFilter,
    receiver: Receiver<StreamEvent>,
    sender: Sender<StreamEvent>,
    subscription_id: Option<SubscriptionId>,
    bus: Option<Arc<EventBus>>,
    stats: Arc<StreamStats>,
}

/// Event wrapper for streaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    pub event_json: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: String,
    pub category: crate::EventCategory,
    pub priority: crate::EventPriority,
}

impl EventStream {
    /// Create a new event stream
    pub fn new(config: StreamConfig) -> Self {
        let (sender, receiver) = if config.backpressure {
            bounded(config.buffer_size)
        } else {
            unbounded()
        };

        Self {
            config,
            filter: StreamFilter::default(),
            receiver,
            sender,
            subscription_id: None,
            bus: None,
            stats: Arc::new(StreamStats::default()),
        }
    }

    /// Create a new event stream connected to an event bus
    pub fn from_bus(bus: Arc<EventBus>, config: StreamConfig, filter: StreamFilter) -> Self {
        let mut stream = Self::new(config);
        stream.filter = filter;
        stream.bus = Some(bus.clone());
        stream.subscribe_to_bus(bus);
        stream
    }

    /// Subscribe to the event bus
    fn subscribe_to_bus(&mut self, bus: Arc<EventBus>) {
        let sender = self.sender.clone();
        let filter = self.filter.clone();
        let stats = self.stats.clone();
        let config = self.config.clone();

        let subscription = bus.subscribe_filtered(Some(filter.event_filter.clone()));
        self.subscription_id = Some(subscription.id());

        // Spawn a task to forward events
        std::thread::spawn(move || {
            let mut last_event_time = HashMap::new();

            while let Ok(event) = subscription.recv() {
                // Apply stream filter
                if !filter.sources.is_empty()
                    && !filter.sources.contains(&event.source().to_string())
                {
                    continue;
                }
                if !filter.tags.is_empty() {
                    let event_tags = &event.metadata().tags;
                    if !filter
                        .tags
                        .iter()
                        .any(|t| event_tags.iter().any(|et| et == t))
                    {
                        continue;
                    }
                }

                // Deduplication
                if filter.dedup_interval_ms > 0 {
                    let now = std::time::Instant::now();
                    let key = (event.source().to_string(), event.name().to_string());
                    if let Some(last) = last_event_time.get(&key) {
                        if now.duration_since(*last)
                            < Duration::from_millis(filter.dedup_interval_ms)
                        {
                            continue;
                        }
                    }
                    last_event_time.insert(key, now);
                }

                // Serialize event
                let event_json = if config.include_metadata {
                    serde_json::to_string(&event).unwrap_or_default()
                } else {
                    // Only serialize data
                    match event.data() {
                        crate::EventData::Empty => "{}".to_string(),
                        crate::EventData::Message(msg) => {
                            format!("{{"message":"{}"}}", escape_json(msg))
                        }
                        crate::EventData::Structured(map) => {
                            serde_json::to_string(map).unwrap_or_default()
                        }
                        crate::EventData::Binary(_) => "{\"binary\":true}".to_string(),
                        crate::EventData::Error { message, code } => {
                            format!("{{"error":"{}","code":{}}}", escape_json(message), code.unwrap_or(0))
                        }
                        crate::EventData::Progress {
                            current,
                            total,
                            message,
                        } => {
                            format!("{{"current":{},"total":{},"message":{}}}", current, total, message.map(|m| format!("\"{}\"", escape_json(&m))).unwrap_or("null".to_string()))
                        }
                        crate::EventData::Metric { name, value, unit } => {
                            format!("{{"metric":"{}","value":{},"unit":{}}}", escape_json(name), value, unit.map(|u| format!("\"{}\"", escape_json(&u))).unwrap_or("null".to_string()))
                        }
                        crate::EventData::Custom(val) => {
                            serde_json::to_string(val).unwrap_or_default()
                        }
                    }
                };

                let stream_event = StreamEvent {
                    event_json,
                    timestamp: event.timestamp(),
                    source: event.source().to_string(),
                    category: event.category(),
                    priority: event.priority(),
                };

                if sender.send(stream_event).is_err() {
                    stats
                        .dropped
                        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    break;
                }

                stats
                    .forwarded
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                // Rate limiting
                if config.max_events_per_second > 0 {
                    let interval =
                        Duration::from_millis(1000 / config.max_events_per_second as u64);
                    std::thread::sleep(interval);
                }
            }
        });
    }

    /// Get the next event from the stream (blocking)
    pub fn next(&self) -> Option<StreamEvent> {
        self.receiver.recv().ok()
    }

    /// Try to get the next event without blocking
    pub fn try_next(&self) -> Result<StreamEvent, crossbeam_channel::TryRecvError> {
        self.receiver.try_recv()
    }

    /// Get all available events
    pub fn drain(&self) -> Vec<StreamEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.receiver.try_recv() {
            events.push(event);
        }
        events
    }

    /// Update the stream filter
    pub fn set_filter(&mut self, filter: StreamFilter) {
        self.filter = filter;
        // Note: Would need to resubscribe to bus for filter changes to take effect
    }

    /// Get stream statistics
    pub fn stats(&self) -> StreamStatsSnapshot {
        StreamStatsSnapshot {
            forwarded: self
                .stats
                .forwarded
                .load(std::sync::atomic::Ordering::Relaxed),
            dropped: self
                .stats
                .dropped
                .load(std::sync::atomic::Ordering::Relaxed),
            buffer_size: self.config.buffer_size,
        }
    }

    /// Check if stream is connected to a bus
    pub fn is_connected(&self) -> bool {
        self.bus.is_some()
    }
}

/// Escape string for JSON
fn escape_json(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 10);
    for ch in s.chars() {
        match ch {
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c => result.push(c),
        }
    }
    result
}

/// Stream statistics
#[derive(Debug, Default)]
struct StreamStats {
    forwarded: std::sync::atomic::AtomicUsize,
    dropped: std::sync::atomic::AtomicUsize,
}

/// Snapshot of stream statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStatsSnapshot {
    pub forwarded: usize,
    pub dropped: usize,
    pub buffer_size: usize,
}

/// Multi-stream manager for handling multiple event streams
pub struct StreamManager {
    streams: HashMap<String, EventStream>,
    default_config: StreamConfig,
}

impl StreamManager {
    pub fn new(default_config: StreamConfig) -> Self {
        Self {
            streams: HashMap::new(),
            default_config,
        }
    }

    /// Create a new stream
    pub fn create_stream(
        &mut self,
        name: &str,
        config: Option<StreamConfig>,
        filter: StreamFilter,
        bus: Option<Arc<EventBus>>,
    ) -> &EventStream {
        let config = config.unwrap_or_else(|| self.default_config.clone());
        let stream = if let Some(bus) = bus {
            EventStream::from_bus(bus, config, filter)
        } else {
            EventStream::new(config)
        };
        self.streams.insert(name.to_string(), stream);
        self.streams.get(name).unwrap()
    }

    /// Get a stream by name
    pub fn get_stream(&self, name: &str) -> Option<&EventStream> {
        self.streams.get(name)
    }

    /// Get a mutable stream by name
    pub fn get_stream_mut(&mut self, name: &str) -> Option<&mut EventStream> {
        self.streams.get_mut(name)
    }

    /// Remove a stream
    pub fn remove_stream(&mut self, name: &str) -> bool {
        self.streams.remove(name).is_some()
    }

    /// List all stream names
    pub fn stream_names(&self) -> Vec<String> {
        self.streams.keys().cloned().collect()
    }

    /// Get combined statistics
    pub fn combined_stats(&self) -> StreamStatsSnapshot {
        let mut total = StreamStatsSnapshot {
            forwarded: 0,
            dropped: 0,
            buffer_size: 0,
        };

        for stream in self.streams.values() {
            let stats = stream.stats();
            total.forwarded += stats.forwarded;
            total.dropped += stats.dropped;
            total.buffer_size = total.buffer_size.max(stats.buffer_size);
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventBus;

    #[test]
    fn test_stream_creation() {
        let stream = EventStream::new(StreamConfig::default());
        assert!(!stream.is_connected());
    }

    #[test]
    fn test_stream_from_bus() {
        let bus = Arc::new(EventBus::new(100));
        let stream = EventStream::from_bus(
            bus.clone(),
            StreamConfig::default(),
            StreamFilter::default(),
        );
        assert!(stream.is_connected());
    }

    #[test]
    fn test_stream_manager() {
        let mut manager = StreamManager::new(StreamConfig::default());
        let bus = Arc::new(EventBus::new(100));

        manager.create_stream("test", None, StreamFilter::default(), Some(bus));
        assert!(manager.get_stream("test").is_some());
        assert_eq!(manager.stream_names().len(), 1);
    }
}
