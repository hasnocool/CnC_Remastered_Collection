//! In-memory log buffer for real-time monitoring

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Log levels matching tracing levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl From<tracing::Level> for LogLevel {
    fn from(level: tracing::Level) -> Self {
        match level {
            tracing::Level::TRACE => LogLevel::Trace,
            tracing::Level::DEBUG => LogLevel::Debug,
            tracing::Level::INFO => LogLevel::Info,
            tracing::Level::WARN => LogLevel::Warn,
            tracing::Level::ERROR => LogLevel::Error,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// A single log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: u64, // Unix timestamp in milliseconds
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub fields: Option<serde_json::Value>,
    pub span_name: Option<String>,
    pub span_id: Option<u64>,
    pub thread_name: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub module_path: Option<String>,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(
        level: LogLevel,
        target: &str,
        message: &str,
        fields: Option<serde_json::Value>,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            timestamp: now,
            level,
            target: target.to_string(),
            message: message.to_string(),
            fields,
            span_name: None,
            span_id: None,
            thread_name: None,
            file: None,
            line: None,
            module_path: None,
        }
    }

    /// Create a log entry with full metadata
    // Unused inside the workspace: kept because it is public API of the
    // log-entry type (external callers build entries from tracing metadata);
    // the nine plain options are the tracing field set, not a refactor target.
    #[allow(clippy::too_many_arguments)]
    pub fn with_metadata(
        level: LogLevel,
        target: &str,
        message: &str,
        fields: Option<serde_json::Value>,
        span_name: Option<String>,
        span_id: Option<u64>,
        file: Option<String>,
        line: Option<u32>,
        module_path: Option<String>,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let thread = std::thread::current();
        Self {
            timestamp: now,
            level,
            target: target.to_string(),
            message: message.to_string(),
            fields,
            span_name,
            span_id,
            thread_name: thread.name().map(|s| s.to_string()),
            file,
            line,
            module_path,
        }
    }
}

/// Thread-safe, bounded in-memory log buffer
#[derive(Debug, Clone)]
pub struct LogBuffer {
    inner: Arc<Mutex<VecDeque<LogEntry>>>,
    capacity: usize,
    dropped_count: Arc<std::sync::atomic::AtomicUsize>,
}

impl LogBuffer {
    /// Create a new log buffer with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(VecDeque::with_capacity(capacity))),
            capacity,
            dropped_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    /// Create a new log buffer with default capacity (10000 entries).
    ///
    /// Deliberately an inherent method, not a `Default` impl: the public
    /// signature returns `Self` (infallible), so it cannot be the trait method.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::new(10000)
    }

    /// Push a log entry into the buffer
    pub fn push(&self, entry: LogEntry) {
        let mut buffer = self.inner.lock().unwrap();
        if buffer.len() >= self.capacity {
            buffer.pop_front();
            self.dropped_count
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        buffer.push_back(entry);
    }

    /// Drain all entries from the buffer
    pub fn drain(&self) -> Vec<LogEntry> {
        let mut buffer = self.inner.lock().unwrap();
        buffer.drain(..).collect()
    }

    /// Get all entries without removing them
    pub fn snapshot(&self) -> Vec<LogEntry> {
        let buffer = self.inner.lock().unwrap();
        buffer.iter().cloned().collect()
    }

    /// Get entries since a given timestamp (milliseconds since epoch)
    pub fn since(&self, timestamp: u64) -> Vec<LogEntry> {
        let buffer = self.inner.lock().unwrap();
        buffer
            .iter()
            .filter(|e| e.timestamp >= timestamp)
            .cloned()
            .collect()
    }

    /// Get entries filtered by level
    pub fn filter_by_level(&self, min_level: LogLevel) -> Vec<LogEntry> {
        let buffer = self.inner.lock().unwrap();
        buffer
            .iter()
            .filter(|e| e.level >= min_level)
            .cloned()
            .collect()
    }

    /// Get entries filtered by target
    pub fn filter_by_target(&self, target: &str) -> Vec<LogEntry> {
        let buffer = self.inner.lock().unwrap();
        buffer
            .iter()
            .filter(|e| e.target.contains(target))
            .cloned()
            .collect()
    }

    /// Get the current number of entries in the buffer
    pub fn len(&self) -> usize {
        let buffer = self.inner.lock().unwrap();
        buffer.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get the number of dropped entries due to capacity limits
    pub fn dropped_count(&self) -> usize {
        self.dropped_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Clear the buffer
    pub fn clear(&self) {
        let mut buffer = self.inner.lock().unwrap();
        buffer.clear();
    }

    /// Subscribe to new log entries via a callback
    pub fn subscribe<F>(&self, callback: F) -> LogSubscription
    where
        F: Fn(&LogEntry) + Send + Sync + 'static,
    {
        LogSubscription {
            buffer: self.inner.clone(),
            callback: Arc::new(callback),
        }
    }
}

/// A subscription to log buffer updates
pub struct LogSubscription {
    buffer: Arc<Mutex<VecDeque<LogEntry>>>,
    callback: Arc<dyn Fn(&LogEntry) + Send + Sync>,
}

impl LogSubscription {
    /// Check for new entries and call the callback
    pub fn poll(&self) {
        let buffer = self.buffer.lock().unwrap();
        for entry in buffer.iter() {
            (self.callback)(entry);
        }
    }
}

impl Default for LogBuffer {
    fn default() -> Self {
        Self::default()
    }
}
