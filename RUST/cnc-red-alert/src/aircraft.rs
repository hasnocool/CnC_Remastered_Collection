//! Red Alert aircraft tuning constants.
//!
//! Aircraft carry a finite ammunition load, must dock to rearm and cannot fire
//! while parked. A helipad holds one aircraft; an airfield holds up to
//! AIRFIELD_CAPACITY.
//!
//! The behaviour lives in [`crate::game`], which operates on `GameUnit`
//! rather than a parallel `Aircraft` type: `Game::update_rearm` handles
//! docking, one round per `REARM_INTERVAL`, and the halipad/airfield slot
//! split; the firing gate is in `Game::update_unit_combat`. This module is the
//! single source of the numbers so the two cannot drift.

/// Default seconds to rearm one full load.
pub const REARM_SECONDS: i32 = 4;
/// Ammunition an aircraft carries when fully loaded.
///
/// Must equal the simulation's value; the module previously claimed 2 while
/// Game used 6, which is the drift the cross-check test guards against.
pub const DEFAULT_AMMO: u32 = 6;
/// Airfield parking capacity; a helipad parks exactly one aircraft.
pub const AIRFIELD_CAPACITY: usize = 4;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game;
    use cnc_core::{ClassId, Fixed16_16, Vec2i};

    #[test]
    fn game_uses_the_constants_defined_here() {
        // Retuning one value without the other fails here.
        assert_eq!(game::AIR_MAX_AMMO as u32, DEFAULT_AMMO);
        assert_eq!(game::AIRFIELD_CAPACITY, AIRFIELD_CAPACITY);
    }

    #[test]
    fn a_helipad_parks_exactly_one_aircraft() {
        let mut game = game::Game::new(40, 40);
        game.structures
            .retain(|s| !matches!(s.class_id, ClassId::Airfield | ClassId::Helipad));
        let cell = Vec2i::new(20, 20);
        game.spawn_structure(1, ClassId::Helipad, cell);
        let a = game.spawn_unit(1, ClassId::AircraftE7, cell);
        let b = game.spawn_unit(1, ClassId::AircraftE7, cell);
        for id in [a, b] {
            let unit = game.units.iter_mut().find(|u| u.id == id).unwrap();
            unit.ammo = 0;
            unit.reload = Fixed16_16::ZERO;
        }
        game.update(Fixed16_16::from_f32(0.1));
        let reloaded = game
            .units
            .iter()
            .filter(|u| [a, b].contains(&u.id) && u.ammo > 0)
            .count();
        assert_eq!(reloaded, 1, "a helipad holds one aircraft");
    }

    #[test]
    fn an_aircraft_burns_ammo_and_rearms_at_a_pad() {
        let mut game = game::Game::new(40, 40);
        let cell = Vec2i::new(20, 20);
        game.spawn_structure(1, ClassId::Airfield, cell);
        let id = game.spawn_unit(1, ClassId::AircraftE7, cell);
        assert_eq!(
            game.units.iter().find(|u| u.id == id).unwrap().max_ammo,
            DEFAULT_AMMO as u8
        );
        let unit = game.units.iter_mut().find(|u| u.id == id).unwrap();
        unit.ammo = 0;
        unit.reload = Fixed16_16::ZERO;
        let before = game.aircraft_rearmed;
        game.update(Fixed16_16::from_f32(0.1));
        assert!(game.aircraft_rearmed > before, "docked aircraft rearms");
    }
}
