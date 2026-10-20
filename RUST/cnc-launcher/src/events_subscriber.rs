//! Launcher-side consumer for the engines' observational event streams.
//!
//! Both engines publish typed observation events through the shared
//! cnc_events::EventBus (see cnc_tiberian_dawn::events and
//! cnc_red_alert::events). The bus is a live channel, not a log: an event is
//! only delivered to subscriptions that exist before it is published. The
//! launcher therefore creates one EventSubscriber at startup, before the first
//! simulation tick, and pumps it once per frame.
//!
//! The subscriber is deliberately passive. It never feeds anything back into the
//! simulation; it only drains the channel into bounded HUD/log state:
//!
//! * drain() returns the events received since the previous call, in publish
//!   order, and folds them into the running counters and the recent-activity
//!   ring buffer.
//! * counts() / count_of() let the HUD show per-event totals.
//! * recent() is the bounded activity log the launcher can render or write out
//!   as text.
//!
//! Dropping the subscriber unsubscribes from the bus; publishing to a bus with
//! no live receivers is inert, so the engine keeps ticking unchanged.

use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use cnc_events::{Event, EventBus, EventSubscription};

/// How many events the recent-activity ring buffer keeps.
pub const DEFAULT_RECENT_CAPACITY: usize = 256;

/// One engine event as the launcher HUD/log needs to see it.
///
/// The payload is intentionally reduced to owned strings: the launcher renders
/// text, and an owned snapshot decouples the HUD from the bus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceivedEvent {
    /// Canonical event name, e.g. "td_unit_produced".
    pub name: String,
    /// Emitting engine, e.g. "cnc-tiberian-dawn".
    pub source: String,
    /// Event category rendered in its debug form (gameplay, system, ui, ...).
    pub category: String,
}

impl ReceivedEvent {
    fn from_event(event: &dyn Event) -> Self {
        Self {
            name: event.name().to_string(),
            source: event.source().to_string(),
            category: format!("{:?}", event.category()),
        }
    }
}

/// Drains engine events from an EventBus into launcher-visible state.
pub struct EventSubscriber {
    bus: Arc<EventBus>,
    subscription: EventSubscription,
    counts: BTreeMap<String, u64>,
    total: u64,
    recent: VecDeque<ReceivedEvent>,
    recent_capacity: usize,
}

impl EventSubscriber {
    /// Subscribe to the given bus immediately.
    ///
    /// Create this before the engine starts stepping: the bus does not retain
    /// history, so events published before this call are not seen.
    pub fn new(bus: Arc<EventBus>) -> Self {
        let subscription = bus.subscribe();
        Self {
            bus,
            subscription,
            counts: BTreeMap::new(),
            total: 0,
            recent: VecDeque::with_capacity(DEFAULT_RECENT_CAPACITY),
            recent_capacity: DEFAULT_RECENT_CAPACITY,
        }
    }

    /// The bus this subscriber is attached to.
    pub fn bus(&self) -> &Arc<EventBus> {
        &self.bus
    }

    /// Drain every event queued since the last call, in publish order.
    ///
    /// Returned events are also folded into the counters and the
    /// recent-activity buffer, so a caller may ignore the return value.
    pub fn drain(&mut self) -> Vec<ReceivedEvent> {
        let mut received = Vec::new();
        while let Ok(event) = self.subscription.try_recv() {
            let item = ReceivedEvent::from_event(event.as_ref());
            *self.counts.entry(item.name.clone()).or_insert(0) += 1;
            self.total += 1;
            if self.recent.len() >= self.recent_capacity {
                self.recent.pop_front();
            }
            self.recent.push_back(item.clone());
            received.push(item);
        }
        received
    }

    /// Total number of events received since construction.
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Per-name received counts (stable, sorted by event name).
    pub fn counts(&self) -> &BTreeMap<String, u64> {
        &self.counts
    }

    /// How many events with the given name have been received.
    pub fn count_of(&self, name: &str) -> u64 {
        self.counts.get(name).copied().unwrap_or(0)
    }

    /// Most recent events, oldest first, bounded by the recent capacity.
    pub fn recent(&self) -> impl Iterator<Item = &ReceivedEvent> {
        self.recent.iter()
    }

    /// The most recent event, if any has been received.
    pub fn last(&self) -> Option<&ReceivedEvent> {
        self.recent.back()
    }

    /// True when no event has ever been received.
    pub fn is_empty(&self) -> bool {
        self.total == 0
    }
}
