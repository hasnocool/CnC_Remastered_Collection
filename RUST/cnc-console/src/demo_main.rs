//! Minimal standalone demo console for UI preview

use std::sync::{Arc, Mutex};
use std::time::Instant;
use anyhow::Result;
use egui::{CentralPanel, Context, SidePanel, TopBottomPanel, ViewportBuilder, ScrollArea, RichText, Color32};
use std::collections::VecDeque;

/// Log entry for demo
#[derive(Debug, Clone)]
struct DemoLogEntry {
    timestamp: Instant,
    level: DemoLogLevel,
    target: String,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DemoLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl DemoLogLevel {
    fn color(&self) -> Color32 {
        match self {
            DemoLogLevel::Trace => Color32::GRAY,
            DemoLogLevel::Debug => Color32::BLUE,
            DemoLogLevel::Info => Color32::GREEN,
            DemoLogLevel::Warn => Color32::YELLOW,
            DemoLogLevel::Error => Color32::RED,
        }
    }
}

/// Event for demo
#[derive(Debug, Clone)]
struct DemoEvent {
    timestamp: Instant,
    category: DemoEventCategory,
    priority: DemoEventPriority,
    source: String,
    data: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DemoEventCategory {
    Gameplay,
    System,
    Network,
    UI,
    Audio,
    Render,
    AI,
    Debug,
}

impl DemoEventCategory {
    fn color(&self) -> Color32 {
        match self {
            DemoEventCategory::Gameplay => Color32::GREEN,
            DemoEventCategory::System => Color32::BLUE,
            DemoEventCategory::Network => Color32::YELLOW,
            DemoEventCategory::UI => Color32::LIGHT_BLUE,
            DemoEventCategory::Audio => Color32::KHAKI,
            DemoEventCategory::Render => Color32::LIGHT_RED,
            DemoEventCategory::AI => Color32::LIGHT_GRAY,
            DemoEventCategory::Debug => Color32::GRAY,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DemoEventPriority {
    Background,
    Low,
    Normal,
    High,
    Critical,
}

impl DemoEventPriority {
    fn color(&self) -> Color32 {
        match self {
            DemoEventPriority::Background => Color32::GRAY,
            DemoEventPriority::Low => Color32::LIGHT_BLUE,
            DemoEventPriority::Normal => Color32::WHITE,
            DemoEventPriority::High => Color32::YELLOW,
            DemoEventPriority::Critical => Color32::RED,
        }
    }
}

/// Main demo application
struct DemoConsoleApp {
    logs: Arc<Mutex<VecDeque<DemoLogEntry>>>,
    events: Arc<Mutex<VecDeque<DemoEvent>>>,
    filter_text: String,
    filter_level: Option<DemoLogLevel>,
    auto_scroll_logs: bool,
    auto_scroll_events: bool,
    show_logs: bool,
    show_events: bool,
    show_filters: bool,
    show_stats: bool,
    theme: egui::Theme,
    start_time: Instant,
    frame_count: u64,
    last_log_count: usize,
    last_event_count: usize,
    log_filter_text: String,
    event_filter_text: String,
    event_filter_category: Option<DemoEventCategory>,
    event_filter_priority: Option<DemoEventPriority>,
}

impl DemoConsoleApp {
    fn new() -> Self {
        let logs = Arc::new(Mutex::new(VecDeque::with_capacity(10000)));
        let events = Arc::new(Mutex::new(VecDeque::with_capacity(5000)));
        
        // Add some demo data
        {
            let mut demo_logs = logs.lock().unwrap();
            demo_logs.push_back(DemoLogEntry {
                timestamp: Instant::now(),
                level: DemoLogLevel::Info,
                target: "cnc_console".to_string(),
                message: "C&C Console started".to_string(),
            });
            demo_logs.push_back(DemoLogEntry {
                timestamp: Instant::now(),
                level: DemoLogLevel::Debug,
                target: "cnc_logs".to_string(),
                message: "Log buffer initialized with capacity 10000".to_string(),
            });
            demo_logs.push_back(DemoLogEntry {
                timestamp: Instant::now(),
                level: DemoLogLevel::Warn,
                target: "cnc_events".to_string(),
                message: "Event bus created with capacity 10000".to_string(),
            });
            demo_logs.push_back(DemoLogEntry {
                timestamp: Instant::now(),
                level: DemoLogLevel::Info,
                target: "cnc_console".to_string(),
                message: "GUI initialized successfully".to_string(),
            });
            demo_logs.push_back(DemoLogEntry {
                timestamp: Instant::now(),
                level: DemoLogLevel::Error,
                target: "network".to_string(),
                message: "Failed to connect to game server: Connection refused".to_string(),
            });
        }
        
        {
            let mut demo_events = events.lock().unwrap();
            demo_events.push_back(DemoEvent {
                timestamp: Instant::now(),
                category: DemoEventCategory::System,
                priority: DemoEventPriority::High,
                source: "cnc_console".to_string(),
                data: r#"{"event": "startup", "version": "0.1.0"}"#.to_string(),
            });
            demo_events.push_back(DemoEvent {
                timestamp: Instant::now(),
                category: DemoEventCategory::Gameplay,
                priority: DemoEventPriority::Normal,
                source: "game_engine".to_string(),
                data: r#"{"unit_created": {"type": "Tank", "player": 1, "position": {"x": 100, "y": 200}}}"#.to_string(),
            });
            demo_events.push_back(DemoEvent {
                timestamp: Instant::now(),
                category: DemoEventCategory::Network,
                priority: DemoEventPriority::Normal,
                source: "network".to_string(),
                data: r#"{"packet_received": {"size": 1024, "from": "192.168.1.100:54321"}}"#.to_string(),
            });
            demo_events.push_back(DemoEvent {
                timestamp: Instant::now(),
                category: DemoEventCategory::UI,
                priority: DemoEventPriority::Low,
                source: "ui".to_string(),
                data: r#"{"button_clicked": {"id": "new_game", "screen": "main_menu"}}"#.to_string(),
            });
            demo_events.push_back(DemoEvent {
                timestamp: Instant::now(),
                category: DemoEventCategory::Audio,
                priority: DemoEventPriority::Low,
                source: "audio".to_string(),
                data: r#"{"sound_played": {"name": "explosion", "volume": 0.8}}"#.to_string(),
            });
        }

        Self {
            logs,
            events,
            filter_text: String::new(),
            filter_level: None,
            auto_scroll_logs: true,
            auto_scroll_events: true,
            show_logs: true,
            show_events: true,
            show_filters: true,
            show_stats: true,
            theme: egui::Theme::Dark,
            start_time: Instant::now(),
            frame_count: 0,
            last_log_count: 0,
            last_event_count: 0,
            log_filter_text: String::new(),
            event_filter_text: String::new(),
            event_filter_category: None,
            event_filter_priority: None,
        }
    }

    fn update(&mut self, ctx: &Context) {
        self.frame_count += 1;
    }

    fn render(&mut self, ctx: &Context) {
        // Top toolbar
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new("🎮 C&C Console").color(Color32::GOLD));
                ui.separator();

                // View toggles
                ui.checkbox(&mut self.show_logs, "📋 Logs");
                ui.checkbox(&mut self.show_events, "📡 Events");
                ui.checkbox(&mut self.show_filters, "🔍 Filters");
                ui.checkbox(&mut self.show_stats, "📊 Stats");

                ui.separator();

                // Auto-scroll toggles
                ui.checkbox(&mut self.auto_scroll_logs, "⬇ Auto-scroll Logs");
                ui.checkbox(&mut self.auto_scroll_events, "⬇ Auto-scroll Events");

                ui.separator();

                // Clear buttons
                if ui.button("🗑 Clear Logs").clicked() {
                    self.logs.lock().unwrap().clear();
                }
                if ui.button("🗑 Clear Events").clicked() {
                    self.events.lock().unwrap().clear();
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Settings button
                    if ui.button("⚙ Settings").clicked() {
                        // Would open settings
                    }

                    // Theme toggle
                    let theme_text = match self.theme {
                        egui::Theme::Light => "☀ Light",
                        egui::Theme::Dark => "🌙 Dark",
                    };
                    if ui.button(theme_text).clicked() {
                        self.theme = match self.theme {
                            egui::Theme::Light => egui::Theme::Dark,
                            egui::Theme::Dark => egui::Theme::Light,
                        };
                        ctx.set_theme(self.theme);
                    }
                });
            });
        });

        // Bottom status bar
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let log_count = self.logs.lock().unwrap().len();
                ui.label(format!("📋 Logs: {}", log_count));
                ui.separator();

                let event_count = self.events.lock().unwrap().len();
                ui.label(format!("📡 Events: {}", event_count));
                ui.separator();

                let elapsed = self.start_time.elapsed().as_secs_f32();
                let fps = if elapsed > 0.0 { self.frame_count as f32 / elapsed } else { 0.0 };
                ui.label(format!("FPS: {:.1}", fps));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(Color32::GREEN, "● Demo Mode");
                });
            });
        });

        // Left sidebar - filters
        if self.show_filters {
            SidePanel::left("filters").resizable(true).default_width(250.0).show(ctx, |ui| {
                ui.label(RichText::new("🔍 Filters").heading());
                ui.separator();

                ui.collapsing("📋 Log Filters", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Level:");
                        egui::ComboBox::from_id_source("log_level")
                            .selected_text(self.filter_level.map(|l| format!("{:?}", l)).unwrap_or_else(|| "All".to_string()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.filter_level, None, "All");
                                ui.selectable_value(&mut self.filter_level, Some(DemoLogLevel::Trace), "Trace");
                                ui.selectable_value(&mut self.filter_level, Some(DemoLogLevel::Debug), "Debug");
                                ui.selectable_value(&mut self.filter_level, Some(DemoLogLevel::Info), "Info");
                                ui.selectable_value(&mut self.filter_level, Some(DemoLogLevel::Warn), "Warn");
                                ui.selectable_value(&mut self.filter_level, Some(DemoLogLevel::Error), "Error");
                            });
                    });
                    ui.add(egui::TextEdit::singleline(&mut self.log_filter_text).hint_text("🔍 Filter..."));
                });

                ui.separator();

                ui.collapsing("📡 Event Filters", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Category:");
                        egui::ComboBox::from_id_source("event_category")
                            .selected_text(self.event_filter_category.map(|c| format!("{:?}", c)).unwrap_or_else(|| "All".to_string()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.event_filter_category, None, "All");
                                ui.selectable_value(&mut self.event_filter_category, Some(DemoEventCategory::Gameplay), "Gameplay");
                                ui.selectable_value(&mut self.event_filter_category, Some(DemoEventCategory::System), "System");
                                ui.selectable_value(&mut self.event_filter_category, Some(DemoEventCategory::Network), "Network");
                                ui.selectable_value(&mut self.event_filter_category, Some(DemoEventCategory::UI), "UI");
                                ui.selectable_value(&mut self.event_filter_category, Some(DemoEventCategory::Audio), "Audio");
                                ui.selectable_value(&mut self.event_filter_category, Some(DemoEventCategory::Render), "Render");
                                ui.selectable_value(&mut self.event_filter_category, Some(DemoEventCategory::AI), "AI");
                                ui.selectable_value(&mut self.event_filter_category, Some(DemoEventCategory::Debug), "Debug");
                            });
                    });

                    ui.horizontal(|ui| {
                        ui.label("Priority:");
                        egui::ComboBox::from_id_source("event_priority")
                            .selected_text(self.event_filter_priority.map(|p| format!("{:?}", p)).unwrap_or_else(|| "All".to_string()))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.event_filter_priority, None, "All");
                                ui.selectable_value(&mut self.event_filter_priority, Some(DemoEventPriority::Background), "Background");
                                ui.selectable_value(&mut self.event_filter_priority, Some(DemoEventPriority::Low), "Low");
                                ui.selectable_value(&mut self.event_filter_priority, Some(DemoEventPriority::Normal), "Normal");
                                ui.selectable_value(&mut self.event_filter_priority, Some(DemoEventPriority::High), "High");
                                ui.selectable_value(&mut self.event_filter_priority, Some(DemoEventPriority::Critical), "Critical");
                            });
                    });

                    ui.add(egui::TextEdit::singleline(&mut self.event_filter_text).hint_text("🔍 Filter..."));
                });
            });
        }

        // Main area - logs and events
        CentralPanel::default().show(ctx, |ui| {
            if self.show_logs && self.show_events {
                egui::TopBottomPanel::top("logs_panel")
                    .resizable(true)
                    .default_height(350.0)
                    .show_inside(ui, |ui| {
                        self.render_logs_panel(ui);
                    });

                egui::TopBottomPanel::bottom("events_panel")
                    .resizable(true)
                    .show_inside(ui, |ui| {
                        self.render_events_panel(ui);
                    });
            } else if self.show_logs {
                self.render_logs_panel(ui);
            } else if self.show_events {
                self.render_events_panel(ui);
            }
        });

        // Stats window
        if self.show_stats {
            egui::Window::new("📊 Statistics")
                .default_pos([10.0, 50.0])
                .collapsible(true)
                .resizable(false)
                .show(ctx, |ui| {
                    self.render_stats(ui);
                });
        }
    }

    fn render_logs_panel(&mut self, ui: &mut egui::Ui) {
        let logs = self.logs.lock().unwrap();
        let filtered_logs: Vec<_> = logs
            .iter()
            .filter(|entry| {
                let mut keep = true;
                if let Some(min_level) = self.filter_level {
                    if (entry.level as u8) < (min_level as u8) {
                        keep = false;
                    }
                }
                if keep && !self.log_filter_text.is_empty() {
                    let filter = self.log_filter_text.to_lowercase();
                    if !entry.message.to_lowercase().contains(&filter) &&
                       !entry.target.to_lowercase().contains(&filter) {
                        keep = false;
                    }
                }
                keep
            })
            .collect();

        ScrollArea::vertical()
            .auto_shrink([false, true])
            .stick_to_bottom(self.auto_scroll_logs)
            .show(ui, |ui| {
                for entry in filtered_logs {
                    ui.horizontal(|ui| {
                        let time_str = format!("{:02}:{:02}:{:02}.{:03}", 
                            (entry.timestamp.elapsed().as_secs() / 3600) % 60,
                            (entry.timestamp.elapsed().as_secs() / 60) % 60,
                            entry.timestamp.elapsed().as_secs() % 60,
                            entry.timestamp.elapsed().subsec_millis()
                        );
                        ui.label(RichText::new(time_str).monospace().color(Color32::GRAY).size(10.0));
                        ui.label(RichText::new(format!("[{:?}]", entry.level)).color(entry.level.color()).strong().monospace().size(10.0));
                        ui.label(RichText::new(&entry.target).color(Color32::LIGHT_BLUE).monospace().size(10.0));
                        ui.label(entry.message.clone());
                    });
                }
            });
    }

    fn render_events_panel(&mut self, ui: &mut egui::Ui) {
        let events = self.events.lock().unwrap();
        let filtered_events: Vec<_> = events
            .iter()
            .filter(|entry| {
                if let Some(cat) = self.event_filter_category {
                    if entry.category != cat { return false; }
                }
                if let Some(pri) = self.event_filter_priority {
                    if entry.priority != pri { return false; }
                }
                if !self.event_filter_text.is_empty() {
                    let filter = self.event_filter_text.to_lowercase();
                    if !entry.data.to_lowercase().contains(&filter) &&
                       !entry.source.to_lowercase().contains(&filter) {
                        return false;
                    }
                }
                true
            })
            .collect();

        ScrollArea::vertical()
            .auto_shrink([false, true])
            .stick_to_bottom(self.auto_scroll_events)
            .show(ui, |ui| {
                for entry in filtered_events {
                    ui.horizontal(|ui| {
                        let time_str = format!("{:02}:{:02}:{:02}.{:03}",
                            (entry.timestamp.elapsed().as_secs() / 3600) % 60,
                            (entry.timestamp.elapsed().as_secs() / 60) % 60,
                            entry.timestamp.elapsed().as_secs() % 60,
                            entry.timestamp.elapsed().subsec_millis()
                        );
                        ui.label(RichText::new(time_str).monospace().color(Color32::GRAY).size(10.0));
                        ui.label(RichText::new(format!("[{:?}]", entry.category)).color(entry.category.color()).strong().monospace().size(10.0));
                        ui.label(RichText::new(format!("[{:?}]", entry.priority)).color(entry.priority.color()).strong().monospace().size(10.0));
                        ui.label(RichText::new(&entry.source).color(Color32::LIGHT_BLUE).monospace().size(10.0));
                        ui.label(RichText::new(&entry.data).size(11.0).monospace());
                    });
                }
            });
    }

    fn render_stats(&self, ui: &mut egui::Ui) {
        let log_count = self.logs.lock().unwrap().len();
        let event_count = self.events.lock().unwrap().len();
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let fps = if elapsed > 0.0 { self.frame_count as f32 / elapsed } else { 0.0 };

        egui::Grid::new("stats_grid").num_columns(2).show(ui, |ui| {
            ui.label("Total Logs:");
            ui.label(RichText::new(log_count.to_string()).strong());
            ui.end_row();
            ui.label("Total Events:");
            ui.label(RichText::new(event_count.to_string()).strong());
            ui.end_row();
            ui.label("FPS:");
            ui.label(RichText::new(format!("{:.1}", fps)).monospace());
            ui.end_row();
            ui.label("Uptime:");
            ui.label(RichText::new(format!("{:.1}s", elapsed)).monospace());
            ui.end_row();
        });
    }
}

impl eframe::App for DemoConsoleApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;
        self.render(ctx);
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("C&C Console - Demo Mode"),
        ..Default::default()
    };

    eframe::run_native(
        "C&C Console - Demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(DemoConsoleApp::new()))
        }),
    )
}
