//! Reusable UI components.

use egui::Ui;

pub fn heading(ui: &mut Ui, text: &str) {
    ui.heading(text);
}

pub fn separator(ui: &mut Ui) {
    ui.separator();
}

pub fn labeled_checkbox(ui: &mut Ui, label: &str, value: &mut bool) {
    ui.checkbox(value, label);
}
