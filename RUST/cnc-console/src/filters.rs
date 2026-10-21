//! Filter configuration UI state.

use cnc_logs::LogLevel;

#[derive(Debug, Clone, Default)]
pub struct LogFilterState {
    pub min_level: Option<LogLevel>,
    pub text: String,
}

impl LogFilterState {
    pub fn matches(&self, level: LogLevel, message: &str, target: &str) -> bool {
        if let Some(min) = self.min_level {
            if level < min {
                return false;
            }
        }
        if !self.text.is_empty() {
            let t = self.text.to_lowercase();
            if !message.to_lowercase().contains(&t) && !target.to_lowercase().contains(&t) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct EventFilterState {
    pub category: Option<String>,
    pub priority: Option<String>,
    pub text: String,
}

impl EventFilterState {
    pub fn matches(&self, category: &str, priority: &str, source: &str, data: &str) -> bool {
        if let Some(c) = &self.category {
            if !c.is_empty() && c != category {
                return false;
            }
        }
        if let Some(p) = &self.priority {
            if !p.is_empty() && p != priority {
                return false;
            }
        }
        if !self.text.is_empty() {
            let t = self.text.to_lowercase();
            if !source.to_lowercase().contains(&t) && !data.to_lowercase().contains(&t) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_logs::LogLevel;

    #[test]
    fn log_filter_level() {
        let f = LogFilterState {
            min_level: Some(LogLevel::Warn),
            text: String::new(),
        };
        assert!(f.matches(LogLevel::Warn, "msg", "tgt"));
        assert!(f.matches(LogLevel::Error, "msg", "tgt"));
        assert!(!f.matches(LogLevel::Info, "msg", "tgt"));
    }

    #[test]
    fn log_filter_text() {
        let f = LogFilterState {
            min_level: None,
            text: "hello".to_string(),
        };
        assert!(f.matches(LogLevel::Info, "say hello", "tgt"));
        assert!(f.matches(LogLevel::Info, "msg", "hello-target"));
        assert!(!f.matches(LogLevel::Info, "msg", "tgt"));
    }

    #[test]
    fn event_filter_basic() {
        let f = EventFilterState {
            category: Some("Gameplay".to_string()),
            priority: None,
            text: String::new(),
        };
        assert!(f.matches("Gameplay", "Normal", "src", "data"));
        assert!(!f.matches("System", "Normal", "src", "data"));
    }
}
