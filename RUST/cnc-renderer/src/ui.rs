// UI Renderer
use crate::Renderer;

pub struct UIRenderer;

impl UIRenderer { pub fn new() -> Self { Self } pub fn draw_button(&mut self, _renderer: &mut dyn Renderer, _x: f32, _y: f32, _w: f32, _h: f32, _text: &str) {} pub fn draw_text(&mut self, _renderer: &mut dyn Renderer, _text: &str, _x: f32, _y: f32, _size: f32) {} }
