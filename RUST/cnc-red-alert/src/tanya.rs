//! Tanya demolition (RA special).
//!
//! Tanya uses C4 to destroy structures.
use cnc_core::Fixed16_16;

/// Tanya demolition damage.
pub fn tanya_demolition_damage() -> Fixed16_16 {
    Fixed16_16::from_f32(500.0)
}

/// Whether the unit is Tanya.
pub fn is_tanya(class_id: cnc_core::ClassId) -> bool {
    matches!(class_id, cnc_core::ClassId::InfantryE11)
}
