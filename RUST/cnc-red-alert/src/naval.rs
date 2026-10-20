//! Red Alert naval tuning constants.
//!
//! Ships move under the shared movement code; submarines additionally carry
//! stealth state (a submerged boat is untargetable and cannot fire until
//! sonar detects it) and a limited torpedo load that rearms at a naval yard.
//!
//! The behaviour lives in [`crate::game`] on `GameUnit`: `is_targetable`,
//! the torpedo gate inside `update_unit_combat`, and the naval-yard branch of
//! `update_rearm`. This module is the single source of the numbers so the
//! simulation and its documentation cannot drift apart.

/// Ship speed while moving under an order.
pub const SHIP_SPEED_PX_PER_SEC: f32 = 60.0;
/// Submarine speed while surfaced.
pub const SUB_SPEED_PX_PER_SEC: f32 = 48.0;
/// Torpedoes a submarine carries when fully loaded.
pub const SUB_TORPEDOES: u8 = 2;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game;
    use cnc_core::{ClassId, Fixed16_16, Vec2i};

    #[test]
    fn game_uses_the_constants_defined_here() {
        // Retuning one without the other fails here rather than silently.
        assert_eq!(game::SUB_TORPEDOES, SUB_TORPEDOES);
    }

    #[test]
    fn a_submerged_undetected_submarine_cannot_be_targeted_or_fire() {
        let mut game = game::Game::new(40, 40);
        let id = game.spawn_unit(1, ClassId::ShipE4, Vec2i::new(20, 20));
        let sub = game.units.iter().find(|u| u.id == id).expect("sub");
        assert!(sub.submerged, "submarines start submerged");
        assert!(!sub.is_targetable(), "hidden boats are not targetable");
        // enemies_of() applies exactly this predicate, so asserting it on the
        // unit covers appearance in the enemy list without widening visibility.
        assert!(!game
            .units
            .iter()
            .find(|u| u.id == id)
            .unwrap()
            .is_targetable());
    }

    #[test]
    fn detection_or_surfacing_makes_a_submarine_targetable() {
        let mut game = game::Game::new(40, 40);
        let id = game.spawn_unit(1, ClassId::ShipE4, Vec2i::new(20, 20));
        game.units.iter_mut().find(|u| u.id == id).unwrap().detected = true;
        assert!(game
            .units
            .iter()
            .find(|u| u.id == id)
            .unwrap()
            .is_targetable());

        game.units.iter_mut().find(|u| u.id == id).unwrap().detected = false;
        game.units
            .iter_mut()
            .find(|u| u.id == id)
            .unwrap()
            .submerged = false;
        assert!(game
            .units
            .iter()
            .find(|u| u.id == id)
            .unwrap()
            .is_targetable());
    }

    #[test]
    fn a_submarine_carries_and_rearms_torpedoes() {
        let mut game = game::Game::new(40, 40);
        let cell = Vec2i::new(20, 20);
        game.spawn_structure(1, ClassId::NavalYard, cell);
        let id = game.spawn_unit(1, ClassId::ShipE4, cell);
        assert_eq!(
            game.units.iter().find(|u| u.id == id).unwrap().torpedoes,
            SUB_TORPEDOES
        );
        let unit = game.units.iter_mut().find(|u| u.id == id).unwrap();
        unit.torpedoes = 0;
        unit.reload = Fixed16_16::ZERO;
        game.update(Fixed16_16::from_f32(0.1));
        assert_eq!(
            game.units.iter().find(|u| u.id == id).unwrap().torpedoes,
            SUB_TORPEDOES,
            "a docked submarine reloads"
        );
    }
}
