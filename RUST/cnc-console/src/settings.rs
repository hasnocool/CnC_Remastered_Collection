//! Settings panel state.

#[derive(Debug, Clone, Default)]
pub struct SettingsPanel {
    pub window_width: f32,
    pub window_height: f32,
    pub open: bool,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            window_width: 1200.0,
            window_height: 800.0,
            open: false,
        }
    }

    pub fn render(&self, ui: &mut egui::Ui) {
        ui.label("Settings");
        let _ = ui;
    }
}
