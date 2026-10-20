//! Logging configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing_core::metadata::LevelFilter;

/// Main logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    /// Minimum log level
    #[serde(default = "default_level")]
    pub level: LogLevelConfig,

    /// Log output destinations
    #[serde(default)]
    pub outputs: Vec<LogOutput>,

    /// Log rotation configuration
    #[serde(default)]
    pub rotation: RotationConfig,

    /// Per-target level overrides
    #[serde(default)]
    pub target_levels: std::collections::HashMap<String, LogLevelConfig>,

    /// Enable structured JSON logging
    #[serde(default = "default_true")]
    pub structured: bool,

    /// Enable ANSI colors in terminal output
    #[serde(default = "default_true")]
    pub colors: bool,

    /// Include timestamps in output
    #[serde(default = "default_true")]
    pub timestamps: bool,

    /// Include thread names in output
    #[serde(default)]
    pub thread_names: bool,

    /// Include file/line info in output
    #[serde(default)]
    pub file_line: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevelConfig::default(),
            outputs: vec![LogOutput::Stdout],
            rotation: RotationConfig::default(),
            target_levels: std::collections::HashMap::new(),
            structured: true,
            colors: true,
            timestamps: true,
            thread_names: false,
            file_line: false,
        }
    }
}

fn default_level() -> LogLevelConfig {
    LogLevelConfig::default()
}

fn default_true() -> bool {
    true
}

/// Log level configuration
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

impl From<LevelFilter> for LogLevelConfig {
    fn from(level: LevelFilter) -> Self {
        match level {
            LevelFilter::TRACE => LogLevelConfig::Trace,
            LevelFilter::DEBUG => LogLevelConfig::Debug,
            LevelFilter::INFO => LogLevelConfig::Info,
            LevelFilter::WARN => LogLevelConfig::Warn,
            LevelFilter::ERROR => LogLevelConfig::Error,
            LevelFilter::OFF => LogLevelConfig::Off,
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

/// Log output destination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LogOutput {
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
    /// File output with path
    File { path: PathBuf },
    /// Rotating file output
    RotatingFile { path: PathBuf },
    /// Syslog (Unix only)
    #[cfg(target_os = "linux")]
    Syslog { facility: String },
    /// Windows Event Log (Windows only)
    #[cfg(target_os = "windows")]
    WindowsEventLog { source: String },
    /// In-memory buffer for monitoring
    Buffer { capacity: usize },
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    /// Maximum file size before rotation (bytes)
    #[serde(default = "default_max_size")]
    pub max_size: u64,

    /// Maximum number of rotated files to keep
    #[serde(default = "default_max_files")]
    pub max_files: usize,

    /// Rotate daily at midnight
    #[serde(default)]
    pub daily: bool,

    /// Compress rotated files
    #[serde(default = "default_true")]
    pub compress: bool,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            max_size: default_max_size(),
            max_files: default_max_files(),
            daily: false,
            compress: true,
        }
    }
}

fn default_max_size() -> u64 {
    100 * 1024 * 1024 // 100 MB
}

fn default_max_files() -> usize {
    10
}
