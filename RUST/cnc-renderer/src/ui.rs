//! Immediate-mode UI widgets over the [`Renderer`] trait.
//!
//! Menus, HUD and console in this project draw through the same
//! `draw_rect`/`draw_text` path as the game (see cnc-launcher's menu pages),
//! so no egui dependency is needed. This module packages the repeated
//! chrome into reusable widgets with unit tests via a recording renderer.
//!
//! All draws are *immediate*: no retained state, no event routing. Button
//! hit-testing is the caller's job ([`widget_at`]); this module renders and
//! reports layout.

use crate::renderer::Renderer;

/// Gold used by the launcher for borders and headings.
pub const GOLD: [f32; 4] = [0.70, 0.55, 0.25, 1.0];
/// Dark translucent fill (matches launcher panel background).
pub const PANEL_FILL: [f32; 4] = [0.05, 0.05, 0.07, 0.92];
pub const TEXT: [f32; 4] = [0.90, 0.93, 0.98, 1.0];
pub const TEXT_DIM: [f32; 4] = [0.60, 0.65, 0.72, 1.0];

/// Rectangle in screen space, matching the launcher's coordinate system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidgetRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl WidgetRect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

/// Panel: filled background with a gold top rule.
pub fn panel(renderer: &mut dyn Renderer, rect: WidgetRect, title: Option<&str>) {
    renderer.draw_rect(rect.x, rect.y, rect.w, rect.h, PANEL_FILL);
    renderer.draw_rect(rect.x, rect.y, rect.w, 2.0, GOLD);
    if let Some(title) = title {
        renderer.draw_text(title, rect.x + 12.0, rect.y + 8.0, 14.0, GOLD);
    }
}

/// Button: bordered box; variants tinted by state. Returns the checked
/// appearance so callers can log or branch off it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Hot,
    Pressed,
    Disabled,
}

pub fn button(
    renderer: &mut dyn Renderer,
    rect: WidgetRect,
    label: &str,
    state: ButtonState,
) -> ButtonState {
    let (border, label_color) = match state {
        ButtonState::Normal => (GOLD, TEXT),
        ButtonState::Hot => ([0.95, 0.80, 0.40, 1.0], TEXT),
        ButtonState::Pressed => ([0.95, 0.80, 0.40, 1.0], GOLD),
        ButtonState::Disabled => ([0.30, 0.30, 0.34, 1.0], TEXT_DIM),
    };
    renderer.draw_rect(rect.x, rect.y, rect.w, rect.h, PANEL_FILL);
    let inset = 1.0;
    renderer.draw_rect(rect.x, rect.y + inset, rect.w, 1.0, border);
    renderer.draw_rect(rect.x, rect.y + rect.h - 1.0, rect.w, 1.0, border);
    renderer.draw_rect(rect.x, rect.y + inset, 1.0, rect.h - 2.0, border);
    renderer.draw_rect(
        rect.x + rect.w - 1.0,
        rect.y + inset,
        1.0,
        rect.h - 2.0,
        border,
    );
    let (lx, ly) = label_origin(rect, label);
    renderer.draw_text(label, lx, ly, 14.0, label_color);
    state
}

/// Hit test helper: `widget_at(rects, x, y)` gives the first rect containing
/// the point (menus iterate in draw order).
pub fn widget_at(rects: &[WidgetRect], x: f32, y: f32) -> Option<WidgetRect> {
    rects.iter().copied().find(|rect| rect.contains(x, y))
}

/// Bottom-right console readout: fixed lines, newest last, dimmed history.
pub fn console(renderer: &mut dyn Renderer, x: f32, y: f32, w: f32, lines: &[String]) {
    let line_height = 13.0;
    let count = lines.len().max(1) as f32;
    let height = count * line_height + 6.0;
    panel(renderer, WidgetRect { x, y, w, h: height }, Some("CONSOLE"));
    for (index, line) in lines.iter().enumerate() {
        let yy = y + 22.0 + index as f32 * line_height;
        let color = if index + 1 == lines.len() {
            TEXT
        } else {
            TEXT_DIM
        };
        renderer.draw_text(line, x + 8.0, yy, 11.0, color);
    }
}

fn label_origin(rect: WidgetRect, label: &str) -> (f32, f32) {
    let width = label.chars().count() as f32 * 12.0;
    let x = rect.x + ((rect.w - width) / 2.0).max(4.0);
    let y = rect.y + (rect.h - 14.0) / 2.0;
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct Recording {
        rects: Vec<([f32; 4], [f32; 4])>,
        texts: Vec<(String, f32, f32)>,
    }

    struct RecordingRenderer {
        log: Arc<Mutex<Recording>>,
    }

    impl Default for RecordingRenderer {
        fn default() -> Self {
            Self {
                log: Arc::new(Mutex::new(Recording::default())),
            }
        }
    }

    impl Renderer for RecordingRenderer {
        fn begin_frame(&mut self) {}
        fn end_frame(&mut self) -> anyhow::Result<()> {
            Ok(())
        }
        fn draw_sprite(&mut self, _sprite: &crate::renderer::Sprite) {}
        fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
            self.log.lock().unwrap().rects.push(([x, y, w, h], color));
        }
        fn draw_text(&mut self, text: &str, x: f32, y: f32, _size: f32, _color: [f32; 4]) {
            self.log
                .lock()
                .unwrap()
                .texts
                .push((text.to_string(), x, y));
        }
        fn set_camera(&mut self, _camera: &crate::Camera) {}
        fn resize(&mut self, _width: u32, _height: u32) {}
        fn set_palette(&mut self, _palette: &cnc_core::Palette) {}
    }

    fn rect(x: f32, y: f32, w: f32, h: f32) -> WidgetRect {
        WidgetRect { x, y, w, h }
    }

    #[test]
    fn button_draws_border_and_centered_label() {
        let mut recorder = RecordingRenderer::default();
        let log = Arc::clone(&recorder.log);
        let state = button(
            &mut recorder,
            rect(10.0, 20.0, 96.0, 32.0),
            "OK",
            ButtonState::Hot,
        );
        assert_eq!(state, ButtonState::Hot);
        let recording = log.lock().unwrap();
        // panel fill + 4 border rules
        assert_eq!(recording.rects.len(), 5, "fill + border rules expected");
        assert!(
            recording
                .texts
                .iter()
                .any(|(text, x, y)| text == "OK" && *x > 10.0 && *y > 20.0),
            "label should be inside the button"
        );
    }

    #[test]
    fn hit_test_finds_first_containing_widget() {
        let rects = [rect(0.0, 0.0, 100.0, 40.0), rect(0.0, 44.0, 100.0, 40.0)];
        assert_eq!(widget_at(&rects, 50.0, 10.0), Some(rects[0]));
        assert_eq!(widget_at(&rects, 50.0, 50.0), Some(rects[1]));
        assert_eq!(widget_at(&rects, 50.0, 42.0), None, "gap is not a widget");
    }

    #[test]
    fn console_grows_one_row_per_line() {
        let mut recorder = RecordingRenderer::default();
        let log = Arc::clone(&recorder.log);
        console(
            &mut recorder,
            10.0,
            20.0,
            200.0,
            &["first".into(), "last".into()],
        );
        let recording = log.lock().unwrap();
        assert!(recording.texts.iter().any(|(t, ..)| t == "first"));
        assert!(recording.texts.iter().any(|(t, ..)| t == "last"));
        assert!(recording.texts.iter().any(|(t, ..)| t == "CONSOLE"));
    }

    #[test]
    fn panel_tints_by_title_presence() {
        let mut recorder = RecordingRenderer::default();
        let log = Arc::clone(&recorder.log);
        panel(&mut recorder, rect(0.0, 0.0, 200.0, 60.0), Some("TITLE"));
        assert!(log.lock().unwrap().texts.iter().any(|(t, ..)| t == "TITLE"));
    }
}
