//! Log filtering directives

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing_core::metadata::LevelFilter;
use tracing_subscriber::filter::Directive;

/// Log filter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilter {
    /// Global minimum level
    #[serde(default)]
    pub global_level: Option<LogLevelConfig>,

    /// Per-target level overrides
    #[serde(default)]
    pub targets: HashMap<String, LogLevelConfig>,

    /// Per-module level overrides (for module_path filtering)
    #[serde(default)]
    pub modules: HashMap<String, LogLevelConfig>,

    /// Directive strings (e.g., "info,my_crate=debug,noisy_module=warn")
    #[serde(default)]
    pub directives: Vec<String>,
}

/// Log level configuration for filters
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevelConfig {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
    Off,
}

impl From<LogLevelConfig> for LevelFilter {
    fn from(level: LogLevelConfig) -> Self {
        match level {
            LogLevelConfig::Trace => LevelFilter::TRACE,
            LogLevelConfig::Debug => LevelFilter::DEBUG,
            LogLevelConfig::Info => LevelFilter::INFO,
            LogLevelConfig::Warn => LevelFilter::WARN,
            LogLevelConfig::Error => LevelFilter::ERROR,
            LogLevelConfig::Off => LevelFilter::OFF,
        }
    }
}

/// A single filter directive
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterDirective {
    pub target: Option<String>,
    pub level: LogLevelConfig,
}

impl FilterDirective {
    /// Parse a directive string like "target=level" or just "level"
    pub fn parse(s: &str) -> Result<Self, String> {
        if let Some((target, level_str)) = s.split_once('=') {
            let level = parse_level(level_str.trim())?;
            Ok(Self {
                target: Some(target.trim().to_string()),
                level,
            })
        } else {
            let level = parse_level(s.trim())?;
            Ok(Self {
                target: None,
                level,
            })
        }
    }
}

fn parse_level(s: &str) -> Result<LogLevelConfig, String> {
    match s.to_lowercase().as_str() {
        "trace" => Ok(LogLevelConfig::Trace),
        "debug" => Ok(LogLevelConfig::Debug),
        "info" => Ok(LogLevelConfig::Info),
        "warn" | "warning" => Ok(LogLevelConfig::Warn),
        "error" => Ok(LogLevelConfig::Error),
        "off" => Ok(LogLevelConfig::Off),
        _ => Err(format!("Invalid log level: {}", s)),
    }
}

impl std::fmt::Display for FilterDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(target) = &self.target {
            write!(f, "{}={}", target, self.level)
        } else {
            write!(f, "{}", self.level)
        }
    }
}

impl std::fmt::Display for LogLevelConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevelConfig::Trace => write!(f, "trace"),
            LogLevelConfig::Debug => write!(f, "debug"),
            LogLevelConfig::Info => write!(f, "info"),
            LogLevelConfig::Warn => write!(f, "warn"),
            LogLevelConfig::Error => write!(f, "error"),
            LogLevelConfig::Off => write!(f, "off"),
        }
    }
}

impl LogFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self {
            global_level: None,
            targets: HashMap::new(),
            modules: HashMap::new(),
            directives: Vec::new(),
        }
    }

    /// Create a filter from directive strings
    pub fn from_directives(directives: &[String]) -> Result<Self, String> {
        let mut filter = Self::new();
        for d in directives {
            filter.add_directive(d)?;
        }
        Ok(filter)
    }

    /// Add a directive to the filter
    pub fn add_directive(&mut self, directive: &str) -> Result<(), String> {
        let parsed = FilterDirective::parse(directive)?;
        if let Some(target) = &parsed.target {
            self.targets.insert(target.clone(), parsed.level);
        } else {
            self.global_level = Some(parsed.level);
        }
        self.directives.push(directive.to_string());
        Ok(())
    }

    /// Get the effective level for a target
    pub fn level_for_target(&self, target: &str) -> LogLevelConfig {
        // Check exact target match first
        if let Some(level) = self.targets.get(target) {
            return *level;
        }

        // Check prefix matches (e.g., "my_crate::module" matches "my_crate")
        for (prefix, level) in &self.targets {
            if target.starts_with(prefix)
                && (target.len() == prefix.len() || target.as_bytes()[prefix.len()] == b':')
            {
                return *level;
            }
        }

        self.global_level.unwrap_or_default()
    }

    /// Convert to tracing EnvFilter
    pub fn to_env_filter(&self) -> tracing_subscriber::EnvFilter {
        let mut filter = tracing_subscriber::EnvFilter::new("");

        if let Some(global) = self.global_level {
            let level_filter: LevelFilter = global.into();
            filter = filter.add_directive(level_filter.into());
        }

        for (target, level) in &self.targets {
            let level_filter: LevelFilter = (*level).into();
            let directive = format!("{}={}", target, level_filter)
                .parse::<Directive>()
                .unwrap();
            filter = filter.add_directive(directive);
        }

        for (module, level) in &self.modules {
            let level_filter: LevelFilter = (*level).into();
            let directive = format!("{}={}", module, level_filter)
                .parse::<Directive>()
                .unwrap();
            filter = filter.add_directive(directive);
        }

        for directive in &self.directives {
            if let Ok(d) = directive.parse::<Directive>() {
                filter = filter.add_directive(d);
            }
        }

        filter
    }
}

impl Default for LogFilter {
    fn default() -> Self {
        Self::new()
    }
}
