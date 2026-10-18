// Map Model
use cnc_core::*;
use std::collections::HashMap;

pub struct MapModel {
    pub width: u32,
    pub height: u32,
    pub terrain: Vec<u8>,
    pub overlay: Vec<u8>,
    pub objects: HashMap<u32, MapObject>,
    pub triggers: Vec<Trigger>,
}

#[derive(Clone, Debug)]
pub struct MapObject {
    pub id: u32,
    pub class_id: ClassId,
    pub position: Vec2,
    pub owner: Option<ClassId>,
}

#[derive(Clone, Debug)]
pub struct Trigger {
    pub id: u32,
    pub name: String,
    pub condition: String,
    pub action: String,
}

impl MapModel {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, terrain: vec![0; (width * height) as usize], overlay: vec![0; (width * height) as usize], objects: HashMap::new(), triggers: Vec::new() }
    }
}

