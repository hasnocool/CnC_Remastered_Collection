// Spy/Thief
use cnc_core::{ClassId, Fixed16_16, Vec2};

pub struct Spy { pub class_id: ClassId, pub position: Vec2, pub health: Fixed16_16, pub disguised_as: Option<ClassId> }

pub struct Thief { pub class_id: ClassId, pub position: Vec2, pub health: Fixed16_16, pub stolen_credits: i32 }

impl Spy { pub fn new(position: Vec2) -> Self { Self { class_id: ClassId::InfantryE8, position, health: Fixed16_16::from_int(40), disguised_as: None } } pub fn disguise(&mut self, as_class: ClassId) { self.disguised_as = Some(as_class); } }

impl Thief { pub fn new(position: Vec2) -> Self { Self { class_id: ClassId::InfantryE9, position, health: Fixed16_16::from_int(40), stolen_credits: 0 } } pub fn steal(&mut self, amount: i32) { self.stolen_credits += amount; } }
