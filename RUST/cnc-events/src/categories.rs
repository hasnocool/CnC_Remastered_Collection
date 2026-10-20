//! Event categories and priorities

use std::fmt;

/// Event category classification
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
pub enum EventCategory {
    /// Gameplay events (unit actions, combat, production, etc.)
    Gameplay = 1,
    /// System events (startup, shutdown, configuration, etc.)
    System = 2,
    /// Network events (connections, packets, sync, etc.)
    Network = 3,
    /// UI events (button clicks, screen changes, input, etc.)
    UI = 4,
    /// Audio events (sound playback, music, volume, etc.)
    Audio = 5,
    /// Render events (frame timing, resource loading, etc.)
    Render = 6,
    /// AI events (decisions, pathfinding, threats, etc.)
    AI = 7,
    /// Debug events (profiling, diagnostics, etc.)
    Debug = 8,
}

impl EventCategory {
    /// Get all categories
    pub fn all() -> &'static [EventCategory] {
        &[
            EventCategory::Gameplay,
            EventCategory::System,
            EventCategory::Network,
            EventCategory::UI,
            EventCategory::Audio,
            EventCategory::Render,
            EventCategory::AI,
            EventCategory::Debug,
        ]
    }

    /// Get category name
    pub fn name(&self) -> &'static str {
        match self {
            EventCategory::Gameplay => "gameplay",
            EventCategory::System => "system",
            EventCategory::Network => "network",
            EventCategory::UI => "ui",
            EventCategory::Audio => "audio",
            EventCategory::Render => "render",
            EventCategory::AI => "ai",
            EventCategory::Debug => "debug",
        }
    }

    /// Get category description
    pub fn description(&self) -> &'static str {
        match self {
            EventCategory::Gameplay => "Gameplay events (unit actions, combat, production)",
            EventCategory::System => "System events (startup, shutdown, configuration)",
            EventCategory::Network => "Network events (connections, packets, sync)",
            EventCategory::UI => "UI events (button clicks, screen changes, input)",
            EventCategory::Audio => "Audio events (sound playback, music, volume)",
            EventCategory::Render => "Render events (frame timing, resource loading)",
            EventCategory::AI => "AI events (decisions, pathfinding, threats)",
            EventCategory::Debug => "Debug events (profiling, diagnostics)",
        }
    }

    /// Get default priority for this category
    pub fn default_priority(&self) -> EventPriority {
        match self {
            EventCategory::Gameplay => EventPriority::Normal,
            EventCategory::System => EventPriority::High,
            EventCategory::Network => EventPriority::Normal,
            EventCategory::UI => EventPriority::Low,
            EventCategory::Audio => EventPriority::Low,
            EventCategory::Render => EventPriority::Background,
            EventCategory::AI => EventPriority::Normal,
            EventCategory::Debug => EventPriority::Background,
        }
    }
}

impl fmt::Display for EventCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for EventCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gameplay" => Ok(EventCategory::Gameplay),
            "system" => Ok(EventCategory::System),
            "network" => Ok(EventCategory::Network),
            "ui" => Ok(EventCategory::UI),
            "audio" => Ok(EventCategory::Audio),
            "render" => Ok(EventCategory::Render),
            "ai" => Ok(EventCategory::AI),
            "debug" => Ok(EventCategory::Debug),
            _ => Err(format!("Unknown event category: {}", s)),
        }
    }
}

/// Event priority levels
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
#[repr(u8)]
#[serde(rename_all = "snake_case")]
pub enum EventPriority {
    /// Background priority - for high-frequency, low-importance events
    Background = 0,
    /// Low priority - for informational events
    Low = 1,
    /// Normal priority - default for most events
    Normal = 2,
    /// High priority - for important events that should be noticed
    High = 3,
    /// Critical priority - for urgent events requiring immediate attention
    Critical = 4,
}

impl EventPriority {
    /// Get all priorities in order
    pub fn all() -> &'static [EventPriority] {
        &[
            EventPriority::Background,
            EventPriority::Low,
            EventPriority::Normal,
            EventPriority::High,
            EventPriority::Critical,
        ]
    }

    /// Get priority name
    pub fn name(&self) -> &'static str {
        match self {
            EventPriority::Background => "background",
            EventPriority::Low => "low",
            EventPriority::Normal => "normal",
            EventPriority::High => "high",
            EventPriority::Critical => "critical",
        }
    }

    /// Get priority description
    pub fn description(&self) -> &'static str {
        match self {
            EventPriority::Background => "Background - high frequency, low importance",
            EventPriority::Low => "Low - informational",
            EventPriority::Normal => "Normal - default priority",
            EventPriority::High => "High - important events",
            EventPriority::Critical => "Critical - urgent attention required",
        }
    }
}

impl fmt::Display for EventPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::str::FromStr for EventPriority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "background" => Ok(EventPriority::Background),
            "low" => Ok(EventPriority::Low),
            "normal" => Ok(EventPriority::Normal),
            "high" => Ok(EventPriority::High),
            "critical" => Ok(EventPriority::Critical),
            _ => Err(format!("Unknown event priority: {}", s)),
        }
    }
}

/// Filter for event categories and priorities
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EventFilter {
    pub categories: Vec<EventCategory>,
    pub min_priority: Option<EventPriority>,
    pub max_priority: Option<EventPriority>,
    pub sources: Vec<String>,
    pub tags: Vec<String>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_category(mut self, category: EventCategory) -> Self {
        self.categories.push(category);
        self
    }

    pub fn with_min_priority(mut self, priority: EventPriority) -> Self {
        self.min_priority = Some(priority);
        self
    }

    pub fn with_max_priority(mut self, priority: EventPriority) -> Self {
        self.max_priority = Some(priority);
        self
    }

    pub fn with_source(mut self, source: &str) -> Self {
        self.sources.push(source.to_string());
        self
    }

    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Match an event behind an unsized `dyn Event` reference.
    ///
    /// `matches` is generic over `E: Event` and therefore cannot be called with
    /// `&dyn Event`; this helper keeps one implementation for both call shapes.
    pub fn matches_boxed(&self, event: &dyn crate::Event) -> bool {
        if !self.categories.is_empty() && !self.categories.contains(&event.category()) {
            return false;
        }

        if let Some(min) = self.min_priority {
            if event.priority() < min {
                return false;
            }
        }
        if let Some(max) = self.max_priority {
            if event.priority() > max {
                return false;
            }
        }

        if !self.sources.is_empty() && !self.sources.contains(&event.source().to_string()) {
            return false;
        }

        if !self.tags.is_empty() {
            let event_tags = &event.metadata().tags;
            if !self
                .tags
                .iter()
                .any(|t| event_tags.iter().any(|et| et == t))
            {
                return false;
            }
        }

        true
    }

    pub fn matches<E: crate::Event>(&self, event: &E) -> bool {
        // Check category
        if !self.categories.is_empty() && !self.categories.contains(&event.category()) {
            return false;
        }

        // Check priority bounds
        if let Some(min) = self.min_priority {
            if event.priority() < min {
                return false;
            }
        }
        if let Some(max) = self.max_priority {
            if event.priority() > max {
                return false;
            }
        }

        // Check source
        if !self.sources.is_empty() && !self.sources.contains(&event.source().to_string()) {
            return false;
        }

        // Check tags
        if !self.tags.is_empty() {
            let event_tags = &event.metadata().tags;
            if !self
                .tags
                .iter()
                .any(|t| event_tags.iter().any(|et| et == t))
            {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_ordering() {
        assert!(EventCategory::Gameplay < EventCategory::System);
        assert!(EventCategory::System < EventCategory::Network);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(EventPriority::Background < EventPriority::Low);
        assert!(EventPriority::Low < EventPriority::Normal);
        assert!(EventPriority::Normal < EventPriority::High);
        assert!(EventPriority::High < EventPriority::Critical);
    }

    #[test]
    fn test_category_from_str() {
        assert_eq!(
            "gameplay".parse::<EventCategory>().unwrap(),
            EventCategory::Gameplay
        );
        assert_eq!(
            "SYSTEM".parse::<EventCategory>().unwrap(),
            EventCategory::System
        );
        assert!("invalid".parse::<EventCategory>().is_err());
    }

    #[test]
    fn test_filter_matching() {
        let filter = EventFilter::new()
            .with_category(EventCategory::Gameplay)
            .with_min_priority(EventPriority::Normal);

        let event = crate::GameEvent::new("test", crate::EventData::message("test"));
        assert!(filter.matches(&event));

        // Low-priority events are rejected by the min-priority bound.
        let low_priority_event = crate::GameEvent::new("test", crate::EventData::message("test"));
        let mut base_metadata = low_priority_event.base.metadata.clone();
        base_metadata.priority = EventPriority::Low;
        let low_priority_event = crate::GameEvent {
            base: crate::definitions::BaseEvent {
                metadata: base_metadata,
                name: low_priority_event.base.name.clone(),
                data: low_priority_event.base.data.clone(),
            },
        };
        assert!(!filter.matches(&low_priority_event));
    }
}
