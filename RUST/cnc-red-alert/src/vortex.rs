//! Chronal vortex (RA special).
//!
//! Ramp through base destroying structures in sequence.
use cnc_core::Fixed16_16;

/// Vortex state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VortexPhase {
    /// Waiting to start.
    Idle,
    /// Moving to next target.
    Ramp,
    /// Vortex ended.
    Ended,
}

/// Damage dealt per structure.
pub const VORTEX_DAMAGE: i32 = 999;

/// Check if vortex should target a structure.
pub fn vortex_should_destroy(class_id: cnc_core::ClassId) -> bool {
    matches!(
        class_id,
        cnc_core::ClassId::PowerPlant
            | cnc_core::ClassId::Barracks
            | cnc_core::ClassId::Refinery
            | cnc_core::ClassId::WarFactory
            | cnc_core::ClassId::ConstructionYard
    )
}

/// Advance vortex phase deterministically.
pub fn advance_vortex(
    phase: VortexPhase,
    elapsed: Fixed16_16,
    interval: Fixed16_16,
) -> VortexPhase {
    match phase {
        VortexPhase::Idle => VortexPhase::Ramp,
        VortexPhase::Ramp if elapsed >= interval => VortexPhase::Ended,
        _ => phase,
    }
}
