//! MAD tank seismic charge + self-destruct sequence (RA special).
//!
//! MAD tank deploys seismic charge, then self-destructs.
use cnc_core::Fixed16_16;

/// Seismic charge damage radius.
pub const MAD_RADIUS: f32 = 8.0;
/// Seismic charge raw damage.
pub const MAD_DAMAGE: i32 = 300;

/// Whether the unit is a MAD tank.
pub fn is_mad_tank(_class_id: cnc_core::ClassId) -> bool {
    false // MAD tank class not present in base ClassId; handled externally
}

/// MAD tank self-destruct sequence: charge then detonation.
pub fn mad_tank_sequence(_health: Fixed16_16) -> Fixed16_16 {
    // Self-destruct: health goes to zero.
    Fixed16_16::ZERO
}
