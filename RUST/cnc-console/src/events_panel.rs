//! Event display and filtering.

use cnc_events::record::EventRecord;
use cnc_events::BoxedEvent;

#[derive(Debug, Clone)]
pub struct EventPanelEntry {
    pub category: String,
    pub priority: String,
    pub source: String,
    pub data: String,
}

#[derive(Debug, Clone, Default)]
pub struct EventsPanel {
    pub entries: Vec<EventPanelEntry>,
    pub auto_scroll: bool,
    pub max_entries: usize,
}

impl EventsPanel {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            auto_scroll: true,
            max_entries,
        }
    }

    pub fn push_event(&mut self, event: &BoxedEvent) {
        let record = EventRecord::from_event(event.as_ref());
        let entry = EventPanelEntry {
            category: record.category.to_string(),
            // EventPriority doesn't impl Display? use Debug
            priority: record.priority.to_string(),
            data: record.to_json(),
            source: record.source,
        };
        self.entries.push(entry);
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
                    ui.label(&e.category);
                    ui.label(&e.priority);
                    ui.label(&e.source);
                    ui.label(&e.data);
                });
            }
        });
    }
}
