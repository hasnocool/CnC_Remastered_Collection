//! Log display and filtering.

use cnc_logs::{LogBuffer, LogEntry};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LogPanel {
    pub entries: Vec<LogEntry>,
    pub auto_scroll: bool,
    pub max_entries: usize,
}

impl Default for LogPanel {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            auto_scroll: true,
            max_entries: 10000,
        }
    }
}

impl LogPanel {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            auto_scroll: true,
            max_entries,
        }
    }

    /// Pull new entries from a shared buffer.
    pub fn sync_from_buffer(&mut self, buffer: &Arc<LogBuffer>) {
        let drained = buffer.drain();
        for e in drained {
            self.entries.push(e);
        }
        if self.entries.len() > self.max_entries {
            let excess = self.entries.len() - self.max_entries;
            self.entries.drain(0..excess);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for e in &self.entries {
                ui.horizontal(|ui| {
                    ui.label(format!("[{}]", e.level));
                    ui.label(&e.target);
                    ui.label(&e.message);
                });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_trims() {
        let mut p = LogPanel::new(2);
        for i in 0..5 {
            p.entries.push(cnc_logs::LogEntry::new(
                cnc_logs::LogLevel::Info,
                "t",
                &format!("m{i}"),
                None,
            ));
            if p.entries.len() > p.max_entries {
                let excess = p.entries.len() - p.max_entries;
                p.entries.drain(0..excess);
            }
        }
        assert_eq!(p.count(), 2);
    }

    #[test]
    fn clear_works() {
        let mut p = LogPanel::default();
        p.entries.push(cnc_logs::LogEntry::new(
            cnc_logs::LogLevel::Info,
            "t",
            "m",
            None,
        ));
        p.clear();
        assert_eq!(p.count(), 0);
    }
}
