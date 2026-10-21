//! Custom themes and colors for the console.

use egui::Color32;

pub const GOLD: Color32 = Color32::from_rgb(255, 215, 0);
pub const LOG_TRACE: Color32 = Color32::GRAY;
pub const LOG_DEBUG: Color32 = Color32::from_rgb(100, 149, 237);
pub const LOG_INFO: Color32 = Color32::from_rgb(50, 205, 50);
pub const LOG_WARN: Color32 = Color32::from_rgb(255, 215, 0);
pub const LOG_ERROR: Color32 = Color32::from_rgb(255, 69, 0);

pub fn apply_theme(ctx: &egui::Context, theme: egui::Theme) {
    ctx.set_theme(theme);
}
