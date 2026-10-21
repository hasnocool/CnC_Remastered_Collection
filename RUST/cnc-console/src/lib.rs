//! cnc-console - Real-time log and event monitor GUI for C&C Remastered Collection
//!
//! Provides a standalone GUI application (and library surface) for monitoring logs
//! and events in real time. Integrates with `cnc-logs` and `cnc-events`.

pub mod app;
pub mod events_panel;
pub mod filters;
pub mod log_panel;
pub mod settings;
pub mod styles;
pub mod widgets;

use std::sync::Arc;

use cnc_events::EventBus;
use cnc_logs::LogBuffer;

/// Console configuration.
#[derive(Clone)]
pub struct ConsoleConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub window_title: String,
    pub show_logs: bool,
    pub show_events: bool,
    pub show_filters: bool,
    pub show_settings: bool,
    pub show_stats: bool,
    pub theme: egui::Theme,
    pub auto_scroll_logs: bool,
    pub auto_scroll_events: bool,
    pub max_log_entries: usize,
    pub max_event_entries: usize,
    pub log_buffer: Option<Arc<LogBuffer>>,
    pub event_bus: Option<Arc<EventBus>>,
    pub log_level: String,
    pub headless: bool,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self {
            window_width: 1200.0,
            window_height: 800.0,
            window_title: "C&C Console - Log & Event Monitor".to_string(),
            show_logs: true,
            show_events: true,
            show_filters: true,
            show_settings: false,
            show_stats: true,
            theme: egui::Theme::Dark,
            auto_scroll_logs: true,
            auto_scroll_events: true,
            max_log_entries: 10000,
            max_event_entries: 5000,
            log_buffer: None,
            event_bus: None,
            log_level: "info".to_string(),
            headless: false,
        }
    }
}

/// Run the console with the given configuration.
pub fn run_console(config: ConsoleConfig) -> anyhow::Result<()> {
    if config.headless {
        // Headless: no display required. Report readiness and exit.
        println!(
            "cnc-console: headless mode, config loaded ({}x{})",
            config.window_width, config.window_height
        );
        return Ok(());
    }
    let title = config.window_title.clone();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config.window_width, config.window_height])
            .with_title(&title),
        ..Default::default()
    };
    eframe::run_native(
        &title,
        options,
        Box::new(move |cc| {
            cc.egui_ctx.set_theme(config.theme);
            Ok(Box::new(app::ConsoleApp::new(config)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}

/// Run the console with an existing log buffer.
pub fn run_console_with_buffer(buffer: Arc<LogBuffer>) -> anyhow::Result<()> {
    let cfg = ConsoleConfig {
        log_buffer: Some(buffer),
        ..ConsoleConfig::default()
    };
    run_console(cfg)
}

/// Run the console with an existing event bus.
pub fn run_console_with_bus(bus: Arc<EventBus>) -> anyhow::Result<()> {
    let cfg = ConsoleConfig {
        event_bus: Some(bus),
        ..ConsoleConfig::default()
    };
    run_console(cfg)
}
