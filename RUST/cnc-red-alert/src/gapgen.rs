// Gap Generator
use cnc_core::{ClassId, Fixed16_16, Vec2};

pub struct GapGenerator { pub class_id: ClassId, pub position: Vec2, pub health: Fixed16_16, pub active: bool, pub radius: Fixed16_16 }

impl GapGenerator { pub fn new(position: Vec2) -> Self { Self { class_id: ClassId::GapGenerator, position, health: Fixed16_16::from_int(300), active: false, radius: Fixed16_16::from_int(8) } } pub fn toggle(&mut self) { self.active = !self.active; } }
