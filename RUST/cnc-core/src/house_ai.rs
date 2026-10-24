//! Shared house commander AI for Tiberian Dawn and Red Alert.
//!
//! This module provides a parameterized `run_expert_ai` that both engines can
//! use. Each engine supplies its own production weights, base building checks,
//! and faction logic through the `HouseAIConfig` trait.

use crate::{ClassId, Fixed16_16, RulesDatabase, SpecialFlags};
use std::collections::VecDeque;

/// AI difficulty levels shared by both games.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AIDifficulty {
    Easy,
    Normal,
    Hard,
    Brutal,
}

/// Production weight entry for weighted random selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductionWeight {
    pub class_id: ClassId,
    pub weight: u32,
}

/// Base building roles that the expert AI checks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseBuildingRole {
    ConstructionYard,
    PowerPlant,
    Refinery,
    Barracks,
    WarFactory,
    Radar,
    TechCenter,
}

/// Configuration trait that each game implements to parameterize the shared AI.
pub trait HouseAIConfig {
    /// The game's concrete type (e.g., td::Game or ra::Game).
    type Game;

    /// Get the house's credit count.
    fn house_credits(game: &Self::Game, owner: usize) -> i32;

    /// Check if AI is enabled for this house.
    fn house_ai_enabled(game: &Self::Game, owner: usize) -> bool;

    /// Get the house's faction/allied status.
    fn house_is_allied(game: &Self::Game, owner: usize) -> bool;

    /// Get the game's elapsed time.
    fn game_elapsed(game: &Self::Game) -> Fixed16_16;

    /// Get the game's rules database.
    fn game_rules(game: &Self::Game) -> &RulesDatabase;

    /// Get mutable game rules database.
    fn game_rules_mut(game: &mut Self::Game) -> &mut RulesDatabase;

    /// Count alive structures of a specific role for a house.
    fn count_structures_by_role(game: &Self::Game, owner: usize, role: BaseBuildingRole) -> usize;

    /// Check if house has at least one alive structure of a role.
    fn has_structure_role(game: &Self::Game, owner: usize, role: BaseBuildingRole) -> bool {
        Self::count_structures_by_role(game, owner, role) > 0
    }

    /// Queue a structure for construction. Returns true if queued.
    fn queue_structure(game: &mut Self::Game, owner: usize, role: BaseBuildingRole) -> bool;

    /// Queue a unit for production. Returns true if queued.
    fn queue_unit(game: &mut Self::Game, owner: usize, class_id: ClassId) -> bool;

    /// Get all alive combat units for a house.
    fn combat_units(game: &Self::Game, owner: usize) -> Vec<usize>;

    /// Count alive harvesters for a house.
    fn harvester_count(game: &Self::Game, owner: usize) -> usize;

    /// Launch an attack wave with the given combat units. Returns true if launched.
    fn launch_attack_wave(game: &mut Self::Game, owner: usize, combat: &[usize]) -> bool;

    /// Sell a structure (for fire sale).
    fn sell_structure(game: &mut Self::Game, owner: usize, index: usize) -> bool;

    /// Production weights for the current tech level and faction.
    /// The roll is a deterministic counter from the AI state.
    fn production_weights(
        config: &HouseAIState,
        rules: &RulesDatabase,
        tech_level: u8,
        is_allied: bool,
        owned_structure_roles: &[BaseBuildingRole],
        credits: i32,
        roll: u32,
    ) -> Vec<ProductionWeight>;

    /// Difficulty-scaled decision interval (seconds between urgency checks).
    fn decision_interval(difficulty: AIDifficulty) -> Fixed16_16;

    /// Difficulty-scaled attack force threshold (minimum combat units before attacking).
    fn attack_force_threshold(difficulty: AIDifficulty) -> usize;

    /// Difficulty-scaled attack interval (seconds between attack waves).
    fn attack_interval(difficulty: AIDifficulty) -> Fixed16_16;

    /// Credits below which fire sale triggers.
    fn fire_sale_credits() -> i32;

    /// Maximum structures remaining before fire sale triggers.
    fn fire_sale_structure_count() -> usize;
}

/// Per-house AI runtime state (mirrors TD AI struct but generic).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HouseAIState {
    pub house_id: usize,
    pub difficulty: AIDifficulty,
    /// AI IQ level from the house — gates advanced behaviors (harvest, repair, production).
    pub iq: u8,
    /// Special gameplay flags (build threshold, shroud-off, one-tick strikes, game speed).
    pub special_flags: SpecialFlags,
    pub build_queue: VecDeque<BuildTask>,
    pub attack_timer: Fixed16_16,
    pub attack_interval: Fixed16_16,
    pub last_attack_time: Fixed16_16,
    /// Decision cadence accumulator (Expert_AI urgency ladder).
    pub decision_timer: Fixed16_16,
    /// Seconds between urgency-check rounds (difficulty-scaled).
    pub decision_interval: Fixed16_16,
    /// Monotonic decision counter; also the deterministic production roll.
    pub tick_counter: u32,
    /// Game elapsed time when the next attack wave may launch.
    pub next_attack_at: Fixed16_16,
    /// Fire sale runs once per game.
    pub fire_sale_done: bool,
}

/// A build task in the queue.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildTask {
    pub class_id: ClassId,
    pub priority: u8,
    pub location: Option<(i32, i32)>, // cell coordinates
}

impl HouseAIState {
    /// Create a new AI state for a house.
    pub fn new(house_id: usize, difficulty: AIDifficulty) -> Self {
        let decision_interval = match difficulty {
            AIDifficulty::Easy => Fixed16_16::from_f32(5.0),
            AIDifficulty::Normal => Fixed16_16::from_f32(3.0),
            AIDifficulty::Hard => Fixed16_16::from_f32(2.0),
            AIDifficulty::Brutal => Fixed16_16::from_f32(1.0),
        };
        let attack_interval = match difficulty {
            AIDifficulty::Easy => Fixed16_16::from_f32(150.0),
            AIDifficulty::Normal => Fixed16_16::from_f32(120.0),
            AIDifficulty::Hard => Fixed16_16::from_f32(90.0),
            AIDifficulty::Brutal => Fixed16_16::from_f32(60.0),
        };
        Self {
            house_id,
            difficulty,
            iq: 0,
            special_flags: SpecialFlags::default(),
            build_queue: VecDeque::new(),
            attack_timer: Fixed16_16::ZERO,
            attack_interval,
            last_attack_time: Fixed16_16::ZERO,
            decision_timer: Fixed16_16::ZERO,
            decision_interval,
            tick_counter: 0,
            next_attack_at: Fixed16_16::ZERO,
            fire_sale_done: false,
        }
    }

    /// Update timers.
    pub fn update(&mut self, delta_time: Fixed16_16) {
        self.decision_timer += delta_time;
        self.attack_timer += delta_time;
        self.tick_counter = self.tick_counter.wrapping_add(1);
    }
}

/// Weighted random choice from a production table (deterministic via roll).
pub fn choose_weighted(table: &[ProductionWeight], roll: u32) -> Option<ClassId> {
    if table.is_empty() {
        return None;
    }
    let total: u32 = table.iter().map(|w| w.weight).sum();
    if total == 0 {
        return None;
    }
    let mut pick = roll % total;
    for weight in table {
        if pick < weight.weight {
            return Some(weight.class_id);
        }
        pick -= weight.weight;
    }
    table.last().map(|w| w.class_id)
}

/// The shared Expert AI urgency ladder.
///
/// Runs difficulty-cadenced checks: fire sale → power → income → offense → attack.
/// Returns true if an action was taken (structure queued, unit queued, or attack launched).
pub fn run_expert_ai<C: HouseAIConfig>(
    game: &mut C::Game,
    ai: &mut HouseAIState,
    delta_time: Fixed16_16,
) -> bool {
    ai.update(delta_time);

    // Decision cadence gate
    if ai.decision_timer < ai.decision_interval {
        return false;
    }
    ai.decision_timer = Fixed16_16::ZERO;

    let owner = ai.house_id;

    // Check if AI is enabled for this house
    if !<C as HouseAIConfig>::house_ai_enabled(game, owner) {
        return false;
    }

    let credits = <C as HouseAIConfig>::house_credits(game, owner);
    let is_allied = <C as HouseAIConfig>::house_is_allied(game, owner);
    let rules = <C as HouseAIConfig>::game_rules(game).clone();

    // Gather base building state
    let has_yard =
        <C as HouseAIConfig>::has_structure_role(game, owner, BaseBuildingRole::ConstructionYard);
    let has_power =
        <C as HouseAIConfig>::has_structure_role(game, owner, BaseBuildingRole::PowerPlant);
    let refinery_count =
        <C as HouseAIConfig>::count_structures_by_role(game, owner, BaseBuildingRole::Refinery);
    let barracks_count =
        <C as HouseAIConfig>::count_structures_by_role(game, owner, BaseBuildingRole::Barracks);
    let war_factory_count =
        <C as HouseAIConfig>::count_structures_by_role(game, owner, BaseBuildingRole::WarFactory);

    let combat = <C as HouseAIConfig>::combat_units(game, owner);
    let harvester_count = <C as HouseAIConfig>::harvester_count(game, owner);

    // Collect owned structure roles for production weight filtering
    let mut owned_roles = Vec::new();
    if has_yard {
        owned_roles.push(BaseBuildingRole::ConstructionYard);
    }
    if has_power {
        owned_roles.push(BaseBuildingRole::PowerPlant);
    }
    if refinery_count > 0 {
        owned_roles.push(BaseBuildingRole::Refinery);
    }
    if barracks_count > 0 {
        owned_roles.push(BaseBuildingRole::Barracks);
    }
    if war_factory_count > 0 {
        owned_roles.push(BaseBuildingRole::WarFactory);
    }

    // Tech level from house (both games use similar tech level system)
    // For TD this comes from house, for RA from house as well
    let tech_level = 10; // Default, will be overridden by game-specific config if needed

    // --- Urgency Ladder ---

    // 1. Check_Fire_Sale: base collapsing and broke — liquidate everything
    let alive_structure_count =
        <C as HouseAIConfig>::count_structures_by_role(
            game,
            owner,
            BaseBuildingRole::ConstructionYard,
        ) + <C as HouseAIConfig>::count_structures_by_role(
            game,
            owner,
            BaseBuildingRole::PowerPlant,
        ) + <C as HouseAIConfig>::count_structures_by_role(game, owner, BaseBuildingRole::Refinery)
            + <C as HouseAIConfig>::count_structures_by_role(
                game,
                owner,
                BaseBuildingRole::Barracks,
            )
            + <C as HouseAIConfig>::count_structures_by_role(
                game,
                owner,
                BaseBuildingRole::WarFactory,
            );
    if alive_structure_count <= <C as HouseAIConfig>::fire_sale_structure_count()
        && credits < <C as HouseAIConfig>::fire_sale_credits()
        && !ai.fire_sale_done
    {
        ai.fire_sale_done = true;
        // Sell all structures (in reverse order for deterministic behavior)
        // Note: This is simplified - the actual selling would need structure indices
        return true;
    }

    // 2. Check_Build_Power: need power plant if we have yard but no power
    if has_yard
        && !has_power
        && <C as HouseAIConfig>::queue_structure(game, owner, BaseBuildingRole::PowerPlant)
    {
        return true;
    }

    // 3. Check_Build_Income: refinery first, then harvesters to work it
    if refinery_count == 0
        && has_yard
        && <C as HouseAIConfig>::queue_structure(game, owner, BaseBuildingRole::Refinery)
    {
        return true;
    }
    if refinery_count > 0
        && harvester_count < refinery_count
        && <C as HouseAIConfig>::queue_unit(game, owner, ClassId::VehicleE6)
    {
        return true;
    }

    // 4. Check_Build_Offense: build combat units until force threshold met
    let force_threshold = <C as HouseAIConfig>::attack_force_threshold(ai.difficulty);
    if combat.len() < force_threshold {
        // Build production weights and pick one
        let weights = <C as HouseAIConfig>::production_weights(
            ai,
            &rules,
            tech_level,
            is_allied,
            &owned_roles,
            credits,
            ai.tick_counter,
        );
        if let Some(class_id) = choose_weighted(&weights, ai.tick_counter) {
            if <C as HouseAIConfig>::queue_unit(game, owner, class_id) {
                return true;
            }
        }
    }

    // 5. Check_Attack: launch waves on the difficulty cadence
    let elapsed = <C as HouseAIConfig>::game_elapsed(game);
    if elapsed >= ai.next_attack_at
        && <C as HouseAIConfig>::launch_attack_wave(game, owner, &combat)
    {
        ai.next_attack_at = elapsed + ai.attack_interval;
        return true;
    }

    false
}
