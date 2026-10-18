// Superweapons
use cnc_core::{ClassId, Fixed16_16, Vec2};

pub struct Chronosphere { pub base: Superweapon, pub range: Fixed16_16 }

pub struct IronCurtain { pub base: Superweapon, pub duration: Fixed16_16 }

pub struct GPS { pub base: Superweapon, pub reveal_radius: Fixed16_16 }

pub struct Superweapon { pub class_id: ClassId, pub position: Vec2, pub ready: bool, pub cooldown: Fixed16_16 }
