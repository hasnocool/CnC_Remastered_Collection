// Naval units
use cnc_core::{ClassId, Fixed16_16, Vec2};

pub struct Ship { pub class_id: ClassId, pub position: Vec2, pub health: Fixed16_16, pub on_water: bool }

pub struct Submarine { pub base: Ship, pub submerged: bool }

impl Ship { pub fn new(class_id: ClassId, position: Vec2, health: Fixed16_16) -> Self { Self { class_id, position, health, on_water: true } } }
