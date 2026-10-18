// Aircraft
use cnc_core::{ClassId, Fixed16_16, Vec2};

pub struct Aircraft { pub class_id: ClassId, pub position: Vec2, pub health: Fixed16_16, pub at_helipad: bool }

pub struct Helipad { pub position: Vec2, pub aircraft: Option<u32> }

pub struct Airfield { pub position: Vec2, pub aircraft: Vec<u32> }
