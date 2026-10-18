// Tesla Coil
use cnc_core::{ClassId, Fixed16_16, Vec2};

pub struct TeslaCoil { pub class_id: ClassId, pub position: Vec2, pub health: Fixed16_16, pub charge: Fixed16_16, pub range: Fixed16_16 }

impl TeslaCoil { pub fn new(position: Vec2) -> Self { Self { class_id: ClassId::TeslaCoil, position, health: Fixed16_16::from_int(400), charge: Fixed16_16::ZERO, range: Fixed16_16::from_int(5) } } pub fn fire(&mut self, target: Vec2) {} }
