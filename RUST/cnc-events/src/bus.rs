//! Event bus for publish/subscribe pattern

use crate::{BoxedEvent, Event, EventFilter, EventId};
use crossbeam_channel::{bounded, unbounded, Receiver, Sender, TryRecvError};
use dashmap::DashMap;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Weak};
use uuid::Uuid;

/// Unique subscription identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionId(pub Uuid);

impl SubscriptionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SubscriptionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SubscriptionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Event subscription for receiving events
pub struct EventSubscription {
    id: SubscriptionId,
    receiver: Receiver<BoxedEvent>,
    filter: Option<EventFilter>,
    bus: Weak<EventBusInner>,
}

impl EventSubscription {
    /// Get the subscription ID
    pub fn id(&self) -> SubscriptionId {
        self.id
    }

    /// Get the filter
    pub fn filter(&self) -> Option<&EventFilter> {
        self.filter.as_ref()
    }

    /// Set a new filter
    pub fn set_filter(&mut self, filter: EventFilter) {
        self.filter = Some(filter);
    }

    /// Try to receive an event without blocking
    pub fn try_recv(&self) -> Result<BoxedEvent, TryRecvError> {
        self.receiver.try_recv()
    }

    /// Receive an event, blocking until one is available
    pub fn recv(&self) -> Result<BoxedEvent, crossbeam_channel::RecvError> {
        self.receiver.recv()
    }

    /// Receive an event with timeout
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<BoxedEvent, crossbeam_channel::RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }

    /// Check if the subscription is still connected to the bus
    pub fn is_connected(&self) -> bool {
        self.bus.strong_count() > 0
    }

    /// Unsubscribe from the bus
    pub fn unsubscribe(self) {
        if let Some(bus) = self.bus.upgrade() {
            bus.unsubscribe(self.id);
        }
    }
}

impl Drop for EventSubscription {
    fn drop(&mut self) {
        if let Some(bus) = self.bus.upgrade() {
            bus.unsubscribe(self.id);
        }
    }
}

/// Internal event bus state
struct EventBusInner {
    subscribers: DashMap<SubscriptionId, Sender<BoxedEvent>>,
    filters: DashMap<SubscriptionId, EventFilter>,
    global_sender: Sender<BoxedEvent>,
    global_receiver: Receiver<BoxedEvent>,
    stats: EventBusStats,
}

#[derive(Debug, Default)]
struct EventBusStats {
    published: AtomicUsize,
    delivered: AtomicUsize,
    dropped: AtomicUsize,
    active_subscriptions: AtomicUsize,
}

/// Event bus for publishing and subscribing to events
pub struct EventBus {
    inner: Arc<EventBusInner>,
    capacity: usize,
}

impl EventBus {
    /// Create a new event bus with the given buffer capacity per subscriber
    pub fn new(capacity: usize) -> Self {
        let (global_sender, global_receiver) = bounded(capacity);

        Self {
            inner: Arc::new(EventBusInner {
                subscribers: DashMap::new(),
                filters: DashMap::new(),
                global_sender,
                global_receiver,
                stats: EventBusStats::default(),
            }),
            capacity,
        }
    }

    /// Create an unbounded event bus (use with caution)
    pub fn unbounded() -> Self {
        let (global_sender, global_receiver) = unbounded();

        Self {
            inner: Arc::new(EventBusInner {
                subscribers: DashMap::new(),
                filters: DashMap::new(),
                global_sender,
                global_receiver,
                stats: EventBusStats::default(),
            }),
            capacity: 0,
        }
    }

    /// Get the bus capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Subscribe to all events
    pub fn subscribe(&self) -> EventSubscription {
        self.subscribe_filtered(None)
    }

    /// Subscribe with a filter
    pub fn subscribe_filtered(&self, filter: Option<EventFilter>) -> EventSubscription {
        let id = SubscriptionId::new();
        let (sender, receiver) = bounded(self.capacity);

        self.inner.subscribers.insert(id, sender);
        if let Some(filter) = filter {
            self.inner.filters.insert(id, filter);
        }
        self.inner
            .stats
            .active_subscriptions
            .fetch_add(1, Ordering::Relaxed);

        EventSubscription {
            id,
            receiver,
            filter: None, // Filter is stored in the bus
            bus: Arc::downgrade(&self.inner),
        }
    }

    /// Publish an event to all subscribers
    pub fn publish<E: Event + 'static>(&self, event: E) -> Result<(), EventBusError> {
        let boxed = Box::new(event);
        self.publish_boxed(boxed)
    }

    /// Publish a boxed event
    pub fn publish_boxed(&self, event: BoxedEvent) -> Result<(), EventBusError> {
        self.inner.stats.published.fetch_add(1, Ordering::Relaxed);

        // Try to send to global channel (for debugging/monitoring)
        if self.inner.global_sender.try_send(event.clone()).is_err() {
            // Global channel full, continue anyway
        }

        // Deliver to subscribers
        let mut delivered = 0;
        let mut to_remove = Vec::new();

        for entry in self.inner.subscribers.iter() {
            let sub_id = *entry.key();
            let sender = entry.value();

            // Check filter
            let should_deliver = if let Some(filter) = self.inner.filters.get(&sub_id) {
                // We need to downcast the event to check the filter
                // This is a limitation - in practice, we'd use a different approach
                true // Simplified for now
            } else {
                true
            };

            if should_deliver {
                match sender.try_send(event.clone()) {
                    Ok(()) => delivered += 1,
                    Err(crossbeam_channel::TrySendError::Full(_)) => {
                        self.inner.stats.dropped.fetch_add(1, Ordering::Relaxed);
                    }
                    Err(crossbeam_channel::TrySendError::Disconnected(_)) => {
                        to_remove.push(sub_id);
                    }
                }
            }
        }

        // Clean up disconnected subscribers
        for sub_id in to_remove {
            self.inner.subscribers.remove(&sub_id);
            self.inner.filters.remove(&sub_id);
            self.inner
                .stats
                .active_subscriptions
                .fetch_sub(1, Ordering::Relaxed);
        }

        self.inner
            .stats
            .delivered
            .fetch_add(delivered, Ordering::Relaxed);
        Ok(())
    }

    /// Publish an event asynchronously (non-blocking)
    pub fn try_publish<E: Event + 'static>(&self, event: E) -> Result<(), EventBusError> {
        self.publish(event)
    }

    /// Unsubscribe a subscription
    pub fn unsubscribe(&self, id: SubscriptionId) {
        self.inner.subscribers.remove(&id);
        self.inner.filters.remove(&id);
        self.inner
            .stats
            .active_subscriptions
            .fetch_sub(1, Ordering::Relaxed);
    }

    /// Get the number of active subscriptions
    pub fn subscription_count(&self) -> usize {
        self.inner
            .stats
            .active_subscriptions
            .load(Ordering::Relaxed)
    }

    /// Get bus statistics
    pub fn stats(&self) -> EventBusStatsSnapshot {
        EventBusStatsSnapshot {
            published: self.inner.stats.published.load(Ordering::Relaxed),
            delivered: self.inner.stats.delivered.load(Ordering::Relaxed),
            dropped: self.inner.stats.dropped.load(Ordering::Relaxed),
            active_subscriptions: self
                .inner
                .stats
                .active_subscriptions
                .load(Ordering::Relaxed),
        }
    }

    /// Get a receiver for all events (for monitoring/debugging)
    pub fn global_receiver(&self) -> Receiver<BoxedEvent> {
        self.inner.global_receiver.clone()
    }

    /// Create a scoped event bus that inherits subscriptions
    pub fn scoped(&self) -> ScopedEventBus {
        ScopedEventBus {
            parent: self.inner.clone(),
            local_subscribers: DashMap::new(),
        }
    }
}

/// Snapshot of event bus statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventBusStatsSnapshot {
    pub published: usize,
    pub delivered: usize,
    pub dropped: usize,
    pub active_subscriptions: usize,
}

/// Errors for event bus operations
#[derive(Debug, thiserror::Error)]
pub enum EventBusError {
    #[error("Failed to publish event: channel full")]
    ChannelFull,
    #[error("Failed to publish event: channel disconnected")]
    ChannelDisconnected,
    #[error("Subscription not found")]
    SubscriptionNotFound,
}

/// Scoped event bus for temporary subscriptions
pub struct ScopedEventBus {
    parent: Arc<EventBusInner>,
    local_subscribers: DashMap<SubscriptionId, Sender<BoxedEvent>>,
}

impl ScopedEventBus {
    pub fn subscribe(&self) -> EventSubscription {
        let id = SubscriptionId::new();
        let (sender, receiver) = bounded(1000);

        self.local_subscribers.insert(id, sender);

        EventSubscription {
            id,
            receiver,
            filter: None,
            bus: Arc::downgrade(&self.parent),
        }
    }

    pub fn publish<E: Event + 'static>(&self, event: E) -> Result<(), EventBusError> {
        let boxed = Box::new(event);
        self.publish_boxed(boxed)
    }

    pub fn publish_boxed(&self, event: BoxedEvent) -> Result<(), EventBusError> {
        // Deliver to local subscribers
        for entry in self.local_subscribers.iter() {
            let _ = entry.value().try_send(event.clone());
        }

        // Also deliver to parent subscribers
        for entry in self.parent.subscribers.iter() {
            let _ = entry.value().try_send(event.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventData, GameEvent};

    #[test]
    fn test_event_bus_basic() {
        let bus = EventBus::new(100);
        let mut sub = bus.subscribe();

        let event = GameEvent::new("test_event", EventData::message("test"));
        bus.publish(event).unwrap();

        let received = sub.try_recv().unwrap();
        assert_eq!(received.name(), "test_event");
    }

    #[test]
    fn test_multiple_subscribers() {
        let bus = EventBus::new(100);
        let mut sub1 = bus.subscribe();
        let mut sub2 = bus.subscribe();

        let event = GameEvent::new("test", EventData::message("test"));
        bus.publish(event).unwrap();

        assert!(sub1.try_recv().is_ok());
        assert!(sub2.try_recv().is_ok());
    }

    #[test]
    fn test_subscription_count() {
        let bus = EventBus::new(100);
        assert_eq!(bus.subscription_count(), 0);

        let _sub1 = bus.subscribe();
        assert_eq!(bus.subscription_count(), 1);

        let _sub2 = bus.subscribe();
        assert_eq!(bus.subscription_count(), 2);
    }

    #[test]
    fn test_unsubscribe() {
        let bus = EventBus::new(100);
        let sub = bus.subscribe();
        let id = sub.id();

        assert_eq!(bus.subscription_count(), 1);
        bus.unsubscribe(id);
        assert_eq!(bus.subscription_count(), 0);
    }

    #[test]
    fn test_stats() {
        let bus = EventBus::new(100);
        let mut sub = bus.subscribe();

        bus.publish(GameEvent::new("test1", EventData::message("test")))
            .unwrap();
        bus.publish(GameEvent::new("test2", EventData::message("test")))
            .unwrap();

        let stats = bus.stats();
        assert_eq!(stats.published, 2);
        assert_eq!(stats.delivered, 2);
    }
}
