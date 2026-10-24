//! Red Alert Game Logic
//!
//! The RA port shares Red Alert's fixed-point economy and unit identity but
//! keeps its own data model: two factions, five countries, naval and air
//! branches, country superweapons and infiltration.

pub mod events;
pub mod game;
pub mod gapgen;
pub mod house;
pub mod map;
pub mod mcv;
pub mod original_scenario;
pub mod pathfind;
pub mod render;
pub mod replay;
pub mod rng;
pub mod scenario;
pub mod scenario_mpr;
pub mod spy;
pub mod superweapon;
pub mod tesla;

pub use events::{names as event_names, OutcomeKind as EventOutcomeKind, SuperweaponKind};
pub use game::{
    armor_for, damage_vs_armor, unit_build_time, AiCommander, ArmorClass, BuildError, Game,
    GameCommand, GameStatus, GameStructure, GameUnit, ObjectiveState, RunningTrigger,
    ScenarioOutcome, Shot, TargetRef, UnitRole, WeaponSpec, AIRFIELD_CAPACITY, AIR_MAX_AMMO,
    REARM_INTERVAL, SUB_TORPEDOES,
};
pub use gapgen::GapGenerator;
pub use house::{Country, Faction, House, ProductionItem};
pub use map::{Map, Terrain, TILE_SIZE};
pub use pathfind::{find_path, MoveClass};
pub use render::{entity_asset, render_entity};
pub use rng::{SimRng, DEFAULT_SEED};
pub use scenario::{Scenario, ScenarioObject, ScenarioTrigger};
pub use scenario_mpr::{build_game as build_game_from_original, load_mpr, load_mpr_game};
pub use spy::{InfiltrationEffect, InfiltrationTarget, Spy, Thief};
pub use superweapon::{ArmoredUnit, Chronosphere, IronCurtain, Superweapon, SuperweaponError, GPS};
pub use tesla::{TeslaBolt, TeslaCoil};

#[cfg(test)]
mod mission_fsm_tests;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::mcv::{can_deploy_at, MCV};
    use cnc_core::{ClassId, Fixed16_16, Vec2};

    fn point(x: f32, y: f32) -> Vec2 {
        Vec2::new(Fixed16_16::from_f32(x), Fixed16_16::from_f32(y))
    }

    #[test]
    fn allied_country_can_charge_its_superweapon_and_defend_a_base() {
        let mut house = House::new(ClassId::HouseAllies, "England".to_string(), Faction::Allied)
            .with_country(Country::England);
        house.add_credits(5000);
        assert!(house.can_build_unique_structure(ClassId::GapGenerator));

        let mut gap = GapGenerator::new(point(100.0, 100.0));
        assert!(gap.toggle());
        house.power -= gap.power_drain();
        assert_eq!(house.power, -100);
        assert!(house.low_power());

        let mut coil = TeslaCoil::new(point(200.0, 200.0));
        coil.powered = !house.low_power() || true;
        coil.update(Fixed16_16::from_int(3));
        assert!(coil.is_ready());
        assert!(gap.shrouds(point(120.0, 120.0)));
    }

    #[test]
    fn soviet_mcv_deploys_into_a_yard_and_iron_curtain_protects_it() {
        let mut mcv = MCV::new(point(48.0, 48.0));
        assert!(can_deploy_at(2, 2, |_, _| false));
        assert!(mcv.begin_deploy());
        let yard = mcv.update(Fixed16_16::from_int(5)).unwrap();
        assert_eq!(yard, point(48.0, 48.0));

        let mut curtain = IronCurtain::new(yard, 1);
        curtain.base.update(Fixed16_16::from_int(600));
        let mut units = vec![ArmoredUnit::new(
            point(50.0, 50.0),
            Fixed16_16::from_int(200),
        )];
        assert_eq!(curtain.harden(&mut units, Fixed16_16::from_int(48)), Ok(1));
        units[0].take_damage(Fixed16_16::from_int(100));
        assert_eq!(units[0].health, Fixed16_16::from_int(200));
    }
}
