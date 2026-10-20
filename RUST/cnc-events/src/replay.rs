//! Event replay functionality

use crate::{
    BoxedEvent, Event, EventBus, EventBusError, EventCategory, EventFilter, EventPriority,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;

/// Replay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayConfig {
    /// Replay speed multiplier (1.0 = real-time, 2.0 = 2x, 0.5 = half speed)
    pub speed: f64,
    /// Start time for replay (None = beginning)
    pub start_time: Option<DateTime<Utc>>,
    /// End time for replay (None = end)
    pub end_time: Option<DateTime<Utc>>,
    /// Filter for events to replay
    pub filter: EventFilter,
    /// Loop replay
    pub loop_replay: bool,
    /// Emit events to bus during replay
    pub emit_to_bus: bool,
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            speed: 1.0,
            start_time: None,
            end_time: None,
            filter: EventFilter::default(),
            loop_replay: false,
            emit_to_bus: true,
        }
    }
}

/// Replay speed presets
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplaySpeed {
    Quarter = 1,
    Half = 2,
    Normal = 3,
    Double = 4,
    Quadruple = 5,
    Max = 6,
}

impl ReplaySpeed {
    pub fn multiplier(&self) -> f64 {
        match self {
            ReplaySpeed::Quarter => 0.25,
            ReplaySpeed::Half => 0.5,
            ReplaySpeed::Normal => 1.0,
            ReplaySpeed::Double => 2.0,
            ReplaySpeed::Quadruple => 4.0,
            ReplaySpeed::Max => 0.0, // 0 means as fast as possible
        }
    }
}

/// Event replay engine
pub struct EventReplay {
    events: VecDeque<ReplayEvent>,
    config: ReplayConfig,
    bus: Option<Arc<EventBus>>,
    position: usize,
    is_playing: bool,
    stats: ReplayStats,
}

/// Event stored for replay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub event_json: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub original_timestamp: DateTime<Utc>,
}

impl EventReplay {
    /// Create a new event replay from a list of events
    pub fn new(events: Vec<BoxedEvent>, config: ReplayConfig) -> Self {
        let replay_events: VecDeque<_> = events
            .into_iter()
            .map(|e| ReplayEvent {
                event_json: serde_json::to_string(&e).unwrap_or_default(),
                timestamp: e.timestamp(),
                original_timestamp: e.timestamp(),
            })
            .collect();

        Self {
            events: replay_events,
            config,
            bus: None,
            position: 0,
            is_playing: false,
            stats: ReplayStats::default(),
        }
    }

    /// Create a replay from JSON strings
    pub fn from_json(events: Vec<String>, config: ReplayConfig) -> Self {
        let replay_events: VecDeque<_> = events
            .into_iter()
            .enumerate()
            .map(|(i, json)| ReplayEvent {
                event_json: json,
                timestamp: Utc::now() + chrono::Duration::milliseconds(i as i64 * 16),
                original_timestamp: Utc::now() + chrono::Duration::milliseconds(i as i64 * 16),
            })
            .collect();

        Self {
            events: replay_events,
            config,
            bus: None,
            position: 0,
            is_playing: false,
            stats: ReplayStats::default(),
        }
    }

    /// Set the event bus for emitting replayed events
    pub fn set_bus(&mut self, bus: Arc<EventBus>) {
        self.bus = Some(bus);
    }

    /// Start replay
    pub fn play(&mut self) {
        self.is_playing = true;
        self.position = self.find_start_position();
    }

    /// Pause replay
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Stop replay and reset position
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.position = 0;
    }

    /// Step forward one event
    pub fn step(&mut self) -> Option<ReplayEvent> {
        if self.position < self.events.len() {
            let event = self.events[self.position].clone();
            self.position += 1;
            self.stats
                .events_replayed
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Some(event)
        } else {
            if self.config.loop_replay {
                self.position = 0;
                self.step()
            } else {
                self.is_playing = false;
                None
            }
        }
    }

    /// Run replay to completion (blocking)
    pub fn run(&mut self) -> ReplayResult {
        self.play();
        let mut last_time = None;

        while self.is_playing {
            if let Some(event) = self.step() {
                let now = std::time::Instant::now();

                if let Some(last) = last_time {
                    let elapsed = now.duration_since(last);
                    let event_interval = if self.position > 1 {
                        let prev = &self.events[self.position - 2];
                        let curr = &self.events[self.position - 1];
                        (curr.timestamp - prev.timestamp)
                            .to_std()
                            .unwrap_or(Duration::from_millis(16))
                    } else {
                        Duration::from_millis(16)
                    };

                    let adjusted_interval = if self.config.speed > 0.0 {
                        Duration::from_secs_f64(event_interval.as_secs_f64() / self.config.speed)
                    } else {
                        Duration::from_nanos(0)
                    };

                    if adjusted_interval > elapsed {
                        std::thread::sleep(adjusted_interval - elapsed);
                    }
                }

                // Emit to bus if configured
                if self.config.emit_to_bus {
                    if let Some(bus) = &self.bus {
                        // In a real implementation, we'd deserialize and publish
                        // For now, just count
                        self.stats
                            .events_emitted
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    }
                }

                last_time = Some(now);
            } else {
                break;
            }
        }

        ReplayResult {
            events_replayed: self
                .stats
                .events_replayed
                .load(std::sync::atomic::Ordering::Relaxed),
            events_emitted: self
                .stats
                .events_emitted
                .load(std::sync::atomic::Ordering::Relaxed),
        }
    }

    /// Get current position
    pub fn position(&self) -> usize {
        self.position
    }

    /// Get total events
    pub fn total_events(&self) -> usize {
        self.events.len()
    }

    /// Get replay progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.events.is_empty() {
            1.0
        } else {
            self.position as f64 / self.events.len() as f64
        }
    }

    /// Check if replay is playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Get replay statistics
    pub fn stats(&self) -> ReplayStatsSnapshot {
        ReplayStatsSnapshot {
            events_replayed: self
                .stats
                .events_replayed
                .load(std::sync::atomic::Ordering::Relaxed),
            events_emitted: self
                .stats
                .events_emitted
                .load(std::sync::atomic::Ordering::Relaxed),
            total_events: self.events.len(),
            current_position: self.position,
            is_playing: self.is_playing,
        }
    }

    /// Seek to a specific position
    pub fn seek(&mut self, position: usize) {
        self.position = position.min(self.events.len());
    }

    /// Seek to a specific time
    pub fn seek_time(&mut self, time: DateTime<Utc>) {
        self.position = self
            .events
            .iter()
            .position(|e| e.timestamp >= time)
            .unwrap_or(self.events.len());
    }

    fn find_start_position(&self) -> usize {
        if let Some(start) = self.config.start_time {
            self.events
                .iter()
                .position(|e| e.timestamp >= start)
                .unwrap_or(0)
        } else {
            0
        }
    }
}

/// Replay statistics
#[derive(Debug, Default)]
struct ReplayStats {
    events_replayed: std::sync::atomic::AtomicUsize,
    events_emitted: std::sync::atomic::AtomicUsize,
}

/// Snapshot of replay statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayStatsSnapshot {
    pub events_replayed: usize,
    pub events_emitted: usize,
    pub total_events: usize,
    pub current_position: usize,
    pub is_playing: bool,
}

/// Result of a replay run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayResult {
    pub events_replayed: usize,
    pub events_emitted: usize,
}

/// Replay recorder for capturing events for later replay
pub struct ReplayRecorder {
    events: Vec<BoxedEvent>,
    filter: EventFilter,
    max_events: usize,
}

impl ReplayRecorder {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            filter: EventFilter::default(),
            max_events,
        }
    }

    pub fn with_filter(mut self, filter: EventFilter) -> Self {
        self.filter = filter;
        self
    }

    pub fn record(&mut self, event: BoxedEvent) {
        if self.filter.matches(event.as_ref()) {
            if self.events.len() >= self.max_events {
                self.events.remove(0);
            }
            self.events.push(event);
        }
    }

    pub fn into_replay(self, config: ReplayConfig) -> EventReplay {
        EventReplay::new(self.events, config)
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventBus, EventData, GameEvent};

    #[test]
    fn test_replay_creation() {
        let events = vec![
            Box::new(GameEvent::new("event1", EventData::message("test"))) as BoxedEvent,
            Box::new(GameEvent::new("event2", EventData::message("test"))) as BoxedEvent,
        ];

        let replay = EventReplay::new(events, ReplayConfig::default());
        assert_eq!(replay.total_events(), 2);
        assert_eq!(replay.position(), 0);
        assert!(!replay.is_playing());
    }

    #[test]
    fn test_replay_step() {
        let events = vec![
            Box::new(GameEvent::new("event1", EventData::message("test"))) as BoxedEvent,
            Box::new(GameEvent::new("event2", EventData::message("test"))) as BoxedEvent,
        ];

        let mut replay = EventReplay::new(events, ReplayConfig::default());
        replay.play();

        let event1 = replay.step().unwrap();
        assert_eq!(replay.position(), 1);

        let event2 = replay.step().unwrap();
        assert_eq!(replay.position(), 2);

        assert!(replay.step().is_none());
    }

    #[test]
    fn test_replay_recorder() {
        let mut recorder = ReplayRecorder::new(100);
        recorder.record(Box::new(GameEvent::new(
            "event1",
            EventData::message("test"),
        )));
        recorder.record(Box::new(GameEvent::new(
            "event2",
            EventData::message("test"),
        )));

        assert_eq!(recorder.event_count(), 2);

        let replay = recorder.into_replay(ReplayConfig::default());
        assert_eq!(replay.total_events(), 2);
    }
}
