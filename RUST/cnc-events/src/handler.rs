//! Event handler registry and dispatch

use crate::{BoxedEvent, Event, EventFilter, SubscriptionId};
use parking_lot::RwLock;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

/// Type-erased event handler
pub type EventHandlerFn = Arc<dyn Fn(&BoxedEvent) + Send + Sync>;

/// Event handler registration
#[derive(Clone)]
pub struct EventHandler {
    pub id: SubscriptionId,
    pub filter: Option<EventFilter>,
    pub handler: EventHandlerFn,
}

impl std::fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Closures are not printable; the identity of the registration is what matters.
        f.debug_struct("EventHandler")
            .field("id", &self.id)
            .field("filter", &self.filter)
            .field("handler", &"<fn>")
            .finish()
    }
}

/// Registry for event handlers
pub struct HandlerRegistry {
    handlers: RwLock<HashMap<TypeId, Vec<EventHandler>>>,
    global_handlers: RwLock<Vec<EventHandler>>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
            global_handlers: RwLock::new(Vec::new()),
        }
    }

    /// Register a handler for a specific event type
    pub fn register<E: Event + 'static>(
        &self,
        filter: Option<EventFilter>,
        handler: impl Fn(&E) + Send + Sync + 'static,
    ) -> SubscriptionId {
        let id = SubscriptionId::new();
        let type_id = TypeId::of::<E>();

        let boxed_handler: EventHandlerFn = Arc::new(move |event: &BoxedEvent| {
            if let Some(typed) = event.as_any().downcast_ref::<E>() {
                handler(typed);
            }
        });

        let event_handler = EventHandler {
            id,
            filter,
            handler: boxed_handler,
        };

        self.handlers
            .write()
            .entry(type_id)
            .or_default()
            .push(event_handler);
        id
    }

    /// Register a global handler that receives all events
    pub fn register_global(
        &self,
        filter: Option<EventFilter>,
        handler: impl Fn(&BoxedEvent) + Send + Sync + 'static,
    ) -> SubscriptionId {
        let id = SubscriptionId::new();

        let event_handler = EventHandler {
            id,
            filter,
            handler: Arc::new(handler),
        };

        self.global_handlers.write().push(event_handler);
        id
    }

    /// Unregister a handler by ID
    pub fn unregister(&self, id: SubscriptionId) -> bool {
        let mut removed = false;

        // Check typed handlers
        for handlers in self.handlers.write().values_mut() {
            if let Some(pos) = handlers.iter().position(|h| h.id == id) {
                handlers.remove(pos);
                removed = true;
            }
        }

        // Check global handlers
        if let Some(pos) = self.global_handlers.write().iter().position(|h| h.id == id) {
            self.global_handlers.write().remove(pos);
            removed = true;
        }

        removed
    }

    /// Dispatch an event to all matching handlers
    pub fn dispatch(&self, event: &BoxedEvent) {
        let type_id = event.as_any().type_id();

        // Dispatch to typed handlers
        if let Some(handlers) = self.handlers.read().get(&type_id) {
            for handler in handlers {
                let should_handle = handler
                    .filter
                    .as_ref()
                    .is_none_or(|f| f.matches_boxed(event.as_ref()));
                if should_handle {
                    (handler.handler)(event);
                }
            }
        }

        // Dispatch to global handlers
        for handler in self.global_handlers.read().iter() {
            let should_handle = handler
                .filter
                .as_ref()
                .is_none_or(|f| f.matches_boxed(event.as_ref()));
            if should_handle {
                (handler.handler)(event);
            }
        }
    }

    /// Get handler count for a type
    pub fn handler_count<E: Event + 'static>(&self) -> usize {
        let type_id = TypeId::of::<E>();
        self.handlers
            .read()
            .get(&type_id)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Get global handler count
    pub fn global_handler_count(&self) -> usize {
        self.global_handlers.read().len()
    }

    /// Clear all handlers
    pub fn clear(&self) {
        self.handlers.write().clear();
        self.global_handlers.write().clear();
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for registering event handlers easily
#[macro_export]
macro_rules! on_event {
    ($registry:expr, $event_type:ty, $handler:expr) => {
        $registry.register::<$event_type>(None, $handler)
    };
    ($registry:expr, $event_type:ty, $filter:expr, $handler:expr) => {
        $registry.register::<$event_type>(Some($filter), $handler)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EventCategory, EventData, EventFilter, GameEvent};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    #[test]
    fn test_handler_registration() {
        let registry = HandlerRegistry::new();
        let received = Arc::new(Mutex::new(Vec::new()));

        let id = registry.register::<GameEvent>(None, {
            let received = Arc::clone(&received);
            move |event: &GameEvent| {
                received.lock().unwrap().push(event.name().to_string());
            }
        });

        assert_eq!(registry.handler_count::<GameEvent>(), 1);

        registry.dispatch(
            &(Box::new(GameEvent::new("test", EventData::message("test"))) as BoxedEvent),
        );

        let received = received.lock().unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0], "test");

        registry.unregister(id);
        assert_eq!(registry.handler_count::<GameEvent>(), 0);
    }

    #[test]
    fn test_global_handler() {
        let registry = HandlerRegistry::new();
        let count = Arc::new(AtomicUsize::new(0));

        registry.register_global(None, {
            let count = Arc::clone(&count);
            move |_| {
                count.fetch_add(1, Ordering::Relaxed);
            }
        });

        registry.dispatch(
            &(Box::new(GameEvent::new("test", EventData::message("test"))) as BoxedEvent),
        );

        assert_eq!(count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_filtered_handler() {
        let registry = HandlerRegistry::new();
        let received = Arc::new(Mutex::new(Vec::new()));

        let filter = EventFilter::new().with_category(EventCategory::Gameplay);
        registry.register::<GameEvent>(Some(filter), {
            let received = Arc::clone(&received);
            move |event: &GameEvent| {
                received.lock().unwrap().push(event.name().to_string());
            }
        });

        registry.dispatch(
            &(Box::new(GameEvent::new("test", EventData::message("test"))) as BoxedEvent),
        );

        assert_eq!(received.lock().unwrap().len(), 1);

        // System event should not be handled
        let sys_event = crate::SystemEvent::startup();
        registry.dispatch(&(Box::new(sys_event) as BoxedEvent));

        assert_eq!(received.lock().unwrap().len(), 1); // Still 1
    }
}
