// Editor UI
use cnc_renderer::Renderer;
use crate::map::MapModel;
use crate::tools::Tool;

pub struct EditorUI {
    pub active_tool: Option<Box<dyn Tool>>,
    pub show_grid: bool,
    pub zoom: f32,
}

impl EditorUI {
    pub fn new() -> Self { Self { active_tool: None, show_grid: true, zoom: 1.0 } }
    pub fn set_tool(&mut self, tool: Box<dyn Tool>) { self.active_tool = Some(tool); }
    pub fn render(&mut self, _renderer: &mut dyn Renderer, _model: &MapModel) {}
}

