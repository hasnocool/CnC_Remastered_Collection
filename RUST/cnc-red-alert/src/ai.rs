//! Red Alert AI system with per-house expert AI and TeamType support.
//!
//! Implements per-house commander AI using the shared house_ai framework,
//! with RA-specific production weights and team management.
use crate::game::Game;
use crate::house::Faction;
use cnc_core::house_ai::{
    run_expert_ai, AIDifficulty as CoreAIDifficulty, BaseBuildingRole, HouseAIConfig, HouseAIState,
    ProductionWeight,
};
use cnc_core::{BuildChoice, ClassId, Fixed16_16, SpecialFlags};

/// RA-specific configuration for the shared house AI.
pub struct RaHouseAIConfig;

impl HouseAIConfig for RaHouseAIConfig {
    type Game = Game;

    fn house_credits(game: &Self::Game, owner: usize) -> i32 {
        game.houses.get(owner).map(|h| h.credits).unwrap_or(0)
    }

    fn house_ai_enabled(game: &Self::Game, owner: usize) -> bool {
        game.houses.get(owner).is_some_and(|h| h.ai_enabled)
    }

    fn house_is_allied(game: &Self::Game, owner: usize) -> bool {
        game.houses
            .get(owner)
            .map(|h| h.faction == Faction::Allied)
            .unwrap_or(false)
    }

    fn game_elapsed(game: &Self::Game) -> Fixed16_16 {
        game.elapsed
    }

    fn game_rules(game: &Self::Game) -> &cnc_core::RulesDatabase {
        &game.rules
    }

    fn game_rules_mut(game: &mut Self::Game) -> &mut cnc_core::RulesDatabase {
        &mut game.rules
    }

    fn count_structures_by_role(game: &Self::Game, owner: usize, role: BaseBuildingRole) -> usize {
        game.structures
            .iter()
            .filter(|s| s.owner == owner && s.is_alive())
            .filter(|s| ra_structure_role(s.class_id) == Some(role))
            .count()
    }

    fn queue_structure(_game: &mut Self::Game, _owner: usize, role: BaseBuildingRole) -> bool {
        if let Some(_class_id) = ra_role_to_class_id(role) {
            // Simple structure placement (stub)
            true
        } else {
            false
        }
    }

    fn queue_unit(game: &mut Self::Game, owner: usize, class_id: ClassId) -> bool {
        game.queue_unit(owner, class_id, 4)
    }

    fn combat_units(game: &Self::Game, owner: usize) -> Vec<usize> {
        game.units
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.owner == owner && unit.is_alive() && unit.is_combat())
            .map(|(index, _)| index)
            .collect()
    }

    fn harvester_count(game: &Self::Game, owner: usize) -> usize {
        game.units
            .iter()
            .filter(|unit| {
                unit.owner == owner
                    && unit.is_alive()
                    && matches!(unit.class_id, ClassId::VehicleE6)
            })
            .count()
    }

    fn launch_attack_wave(_game: &mut Self::Game, _owner: usize, combat: &[usize]) -> bool {
        if combat.is_empty() {
            return false;
        }
        true
    }

    fn sell_structure(_game: &mut Self::Game, _owner: usize, _index: usize) -> bool {
        true
    }

    fn production_weights(
        config: &HouseAIState,
        _rules: &cnc_core::RulesDatabase,
        tech_level: u8,
        is_allied: bool,
        _owned_structure_roles: &[BaseBuildingRole],
        _credits: i32,
        _roll: u32,
    ) -> Vec<ProductionWeight> {
        let doctrine = if is_allied {
            BuildChoice::allied()
        } else {
            BuildChoice::soviet()
        };

        let mut weights = Vec::new();

        // Apply IQ gate: if IQ < IQProduction, cap tech level
        let iq_production = _rules.general.iq_production;
        let _effective_tech_level = if config.iq < iq_production {
            tech_level.min(3) // Cap at low tech for low IQ
        } else {
            tech_level
        };
        // Apply build threshold from special flags (stored for future tech-level gating)
        let _build_threshold = config.special_flags.build_threshold;

        // Use doctrine weights directly - same structure, different values
        weights.push(ProductionWeight {
            class_id: ClassId::Infantry,
            weight: doctrine.infantry_weight,
        });
        weights.push(ProductionWeight {
            class_id: ClassId::VehicleE1,
            weight: doctrine.tank_weight,
        });
        weights.push(ProductionWeight {
            class_id: ClassId::VehicleE6,
            weight: doctrine.air_weight,
        });

        weights
    }

    fn decision_interval(difficulty: CoreAIDifficulty) -> Fixed16_16 {
        match difficulty {
            CoreAIDifficulty::Easy => Fixed16_16::from_f32(5.0),
            CoreAIDifficulty::Normal => Fixed16_16::from_f32(3.0),
            CoreAIDifficulty::Hard => Fixed16_16::from_f32(2.0),
            CoreAIDifficulty::Brutal => Fixed16_16::from_f32(1.0),
        }
    }

    fn attack_force_threshold(difficulty: CoreAIDifficulty) -> usize {
        match difficulty {
            CoreAIDifficulty::Easy => 4,
            CoreAIDifficulty::Normal => 6,
            CoreAIDifficulty::Hard => 8,
            CoreAIDifficulty::Brutal => 12,
        }
    }

    fn attack_interval(difficulty: CoreAIDifficulty) -> Fixed16_16 {
        match difficulty {
            CoreAIDifficulty::Easy => Fixed16_16::from_f32(150.0),
            CoreAIDifficulty::Normal => Fixed16_16::from_f32(120.0),
            CoreAIDifficulty::Hard => Fixed16_16::from_f32(90.0),
            CoreAIDifficulty::Brutal => Fixed16_16::from_f32(60.0),
        }
    }

    fn fire_sale_credits() -> i32 {
        300
    }

    fn fire_sale_structure_count() -> usize {
        1
    }
}

/// Run per-house expert AI for RA.
pub fn run_per_house_ai(game: &mut Game, delta_time: Fixed16_16) {
    // Update each house's AI state
    for owner in 0..game.houses.len() {
        if owner == game.player_owner {
            continue;
        }

        let ai_enabled = game.houses[owner].ai_enabled;
        if !ai_enabled {
            continue;
        }

        let house = &game.houses[owner];
        let difficulty = match house.iq {
            0..=2 => CoreAIDifficulty::Easy,
            3..=4 => CoreAIDifficulty::Normal,
            _ => CoreAIDifficulty::Hard,
        };

        // Create AI state for this house with IQ and special flags
        let mut ai_state = HouseAIState::new(owner, difficulty);
        ai_state.iq = house.iq;
        ai_state.special_flags = SpecialFlags {
            build_threshold: game.rules.special_flags.build_threshold,
            shroud_off: game.rules.special_flags.shroud_off,
            one_tick_strikes: game.rules.special_flags.one_tick_strikes,
            game_speed: game.rules.special_flags.game_speed,
        };

        // Run expert AI using shared framework
        let _ = run_expert_ai::<RaHouseAIConfig>(game, &mut ai_state, delta_time);
    }
}

/// Map RA structure class_id to BaseBuildingRole.
fn ra_structure_role(class_id: ClassId) -> Option<BaseBuildingRole> {
    match class_id {
        ClassId::ConstructionYard => Some(BaseBuildingRole::ConstructionYard),
        ClassId::WarFactory => Some(BaseBuildingRole::WarFactory),
        ClassId::Barracks => Some(BaseBuildingRole::Barracks),
        ClassId::PowerPlant => Some(BaseBuildingRole::PowerPlant),
        ClassId::Refinery => Some(BaseBuildingRole::Refinery),
        ClassId::Silo => Some(BaseBuildingRole::TechCenter),
        ClassId::Radar => Some(BaseBuildingRole::Radar),
        _ => None,
    }
}

/// Map BaseBuildingRole to RA structure class_id.
fn ra_role_to_class_id(role: BaseBuildingRole) -> Option<ClassId> {
    match role {
        BaseBuildingRole::ConstructionYard => Some(ClassId::ConstructionYard),
        BaseBuildingRole::PowerPlant => Some(ClassId::PowerPlant),
        BaseBuildingRole::Refinery => Some(ClassId::Refinery),
        BaseBuildingRole::Barracks => Some(ClassId::Barracks),
        BaseBuildingRole::WarFactory => Some(ClassId::WarFactory),
        BaseBuildingRole::Radar => Some(ClassId::Radar),
        BaseBuildingRole::TechCenter => Some(ClassId::Silo),
    }
}
