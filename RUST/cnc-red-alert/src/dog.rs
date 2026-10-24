//! Attack dog special: kills infantry in one bite.
use cnc_core::{ClassId, Fixed16_16};

/// Dog damage: one bite kills any infantry.
pub fn dog_bite_damage() -> Fixed16_16 {
    Fixed16_16::from_f32(999.0)
}

/// Whether the unit is a dog.
pub fn is_dog(class_id: ClassId) -> bool {
    class_id == ClassId::InfantryE10 // placeholder; Commando slot used as dog for now
}
