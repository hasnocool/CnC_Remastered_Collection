//! Main application logic and rendering.

use crate::{
    events_panel::EventsPanel, filters, log_panel::LogPanel, settings::SettingsPanel, ConsoleConfig,
};
use std::sync::Arc;

pub struct ConsoleApp {
    pub config: ConsoleConfig,
    pub logs: LogPanel,
    pub events: EventsPanel,
    pub settings: SettingsPanel,
    pub log_filter: filters::LogFilterState,
    pub event_filter: filters::EventFilterState,
    pub frame_count: u64,
    pub start_time: std::time::Instant,
}

impl ConsoleApp {
    pub fn new(config: ConsoleConfig) -> Self {
        Self {
            logs: LogPanel::new(config.max_log_entries),
            events: EventsPanel::new(config.max_event_entries),
            settings: SettingsPanel::new(),
            log_filter: filters::LogFilterState::default(),
            event_filter: filters::EventFilterState::default(),
            frame_count: 0,
            start_time: std::time::Instant::now(),
            config,
        }
    }

    /// Optional shared buffer handle.
    pub fn log_buffer(&self) -> Option<Arc<cnc_logs::LogBuffer>> {
        self.config.log_buffer.clone()
    }
}

impl eframe::App for ConsoleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;

        // Sync logs from buffer if present.
        if let Some(buf) = &self.config.log_buffer {
            self.logs.sync_from_buffer(buf);
        }

        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("C&C Console");
                ui.checkbox(&mut self.config.show_logs, "Logs");
                ui.checkbox(&mut self.config.show_events, "Events");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.config.show_logs {
                self.logs.render(ui);
            }
            if self.config.show_events {
                self.events.render(ui);
            }
        });
    }
}
