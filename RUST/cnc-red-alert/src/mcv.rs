// MCV
use cnc_core::{ClassId, Fixed16_16, Vec2};

pub struct MCV { pub class_id: ClassId, pub position: Vec2, pub health: Fixed16_16, pub deployed: bool }

impl MCV { pub fn new(position: Vec2) -> Self { Self { class_id: ClassId::VehicleE7, position, health: Fixed16_16::from_int(200), deployed: false } } pub fn deploy(&mut self) -> Option<Vec2> { if !self.deployed { self.deployed = true; Some(self.position) } else { None } } }
