// Tools
use cnc_core::*;
use crate::map::{MapModel, MapObject, Trigger};

pub trait Tool {
    fn name(&self) -> &str;
    fn on_click(&mut self, _model: &mut MapModel, _pos: Vec2);
    fn on_drag(&mut self, _model: &mut MapModel, _from: Vec2, _to: Vec2);
    fn on_release(&mut self, _model: &mut MapModel, _pos: Vec2);
}

pub struct TerrainTool { pub terrain_type: u8 }

impl Tool for TerrainTool {
    fn name(&self) -> &str { "Terrain" }
    fn on_click(&mut self, model: &mut MapModel, pos: Vec2) { let x = pos.x.int() as usize; let y = pos.y.int() as usize; if x < model.width as usize && y < model.height as usize { model.terrain[y * model.width as usize + x] = self.terrain_type; } }
    fn on_drag(&mut self, model: &mut MapModel, from: Vec2, to: Vec2) { self.on_click(model, to); }
    fn on_release(&mut self, _model: &mut MapModel, _pos: Vec2) {}
}

pub struct ObjectTool { pub class_id: ClassId }

impl Tool for ObjectTool {
    fn name(&self) -> &str { "Object" }
    fn on_click(&mut self, model: &mut MapModel, pos: Vec2) { let id = model.objects.len() as u32 + 1; model.objects.insert(id, MapObject { id, class_id: self.class_id, position: pos, owner: None }); }
    fn on_drag(&mut self, _model: &mut MapModel, _from: Vec2, _to: Vec2) {}
    fn on_release(&mut self, _model: &mut MapModel, _pos: Vec2) {}
}

pub struct TriggerTool;

impl Tool for TriggerTool {
    fn name(&self) -> &str { "Trigger" }
    fn on_click(&mut self, model: &mut MapModel, pos: Vec2) { model.triggers.push(Trigger { id: model.triggers.len() as u32, name: format!("Trigger_{}", pos.x), condition: "Always".into(), action: "None".into() }); }
    fn on_drag(&mut self, _model: &mut MapModel, _from: Vec2, _to: Vec2) {}
    fn on_release(&mut self, _model: &mut MapModel, _pos: Vec2) {}
}