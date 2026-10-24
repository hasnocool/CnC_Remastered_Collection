//! Crate pickups (RA special).
//!
//! Grants money, unit, or invulnerability per CRATE.CPP.
use cnc_core::Fixed16_16;

/// Crate effect type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrateEffect {
    Money(i32),
    Unit,
    Invulnerability,
}

/// Deterministic crate effect based on crate index (for deterministic games).
pub fn crate_effect(crate_index: u32) -> CrateEffect {
    match crate_index % 3 {
        0 => CrateEffect::Money(2000),
        1 => CrateEffect::Unit,
        2 => CrateEffect::Invulnerability,
        _ => CrateEffect::Money(2000),
    }
}

/// Apply crate effect.
pub fn apply_crate_effect(
    effect: CrateEffect,
    credits: &mut i32,
    invulnerable_until: &mut Fixed16_16,
    elapsed: Fixed16_16,
) {
    match effect {
        CrateEffect::Money(amount) => *credits += amount,
        CrateEffect::Unit => {} // Spawn handled by caller
        CrateEffect::Invulnerability => *invulnerable_until = elapsed + Fixed16_16::from_int(30),
    }
}
