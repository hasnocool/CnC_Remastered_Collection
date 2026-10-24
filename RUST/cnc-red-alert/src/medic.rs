//! Medic healing infantry (RA special).
//!
//! Medics restore health to injured infantry units.
use cnc_core::Fixed16_16;

/// Amount healed per medkit application.
pub fn medic_heal_amount() -> Fixed16_16 {
    Fixed16_16::from_f32(50.0)
}

/// Heal an infantry unit.
pub fn heal_infantry(health: &mut Fixed16_16, max_health: Fixed16_16) {
    *health = (*health + medic_heal_amount()).min(max_health);
}

/// Whether the unit is a medic.
pub fn is_medic(class_id: cnc_core::ClassId) -> bool {
    matches!(class_id, cnc_core::ClassId::InfantryE7)
}
