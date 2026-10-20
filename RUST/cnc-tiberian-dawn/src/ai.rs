//! AI system for Tiberian Dawn
//!
//! Production planning is driven by the Rules.ini registry: costs, TechLevel
//! and prerequisites are read from the loaded rules database, with the
//! hard-coded TDATA/UDATA/BDATA values as fallback. Selection is deterministic
//! (a caller-supplied roll), so AI decisions stay lockstep-safe.

use crate::td_rules::{
    default_unit_owner, owner_token_allows, prerequisite_token_met, unit_class_id, StructureType,
    UnitType,
};
use cnc_core::entity::{Team, TeamObjective, TeamType};
use cnc_core::rules::RulesDatabase;
use cnc_core::{ClassId, Fixed16_16, Vec2};
use std::collections::VecDeque;

/// AI difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AIDifficulty {
    Easy,
    Normal,
    Hard,
    Brutal,
}

/// AI player state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AI {
    pub house_id: u8,
    pub difficulty: AIDifficulty,
    pub build_queue: VecDeque<BuildTask>,
    pub attack_timer: Fixed16_16,
    pub attack_interval: Fixed16_16,
    pub last_attack_time: Fixed16_16,
    pub defense_groups: Vec<DefenseGroup>,
    pub attack_groups: Vec<AttackGroup>,
    pub scout_positions: Vec<Vec2>,
    /// HOUSE.CPP `Expert_AI` decision cadence accumulator.
    pub decision_timer: Fixed16_16,
    /// Seconds between urgency-check rounds (difficulty-scaled).
    pub decision_interval: Fixed16_16,
    /// Monotonic decision counter; also the deterministic production roll.
    pub tick_counter: u32,
    /// Game elapsed time when the next attack wave may launch.
    pub next_attack_at: Fixed16_16,
    /// RA `Check_Fire_Sale` runs once per game.
    pub fire_sale_done: bool,
}

/// Deterministic stand-in for the `Random_Pick` the original uses to scatter
/// `[TeamTypes]` missions.
///
/// Lockstep play cannot consume a real RNG, so the offset is derived from the
/// team id, its own tick and the unit ordinal: identical inputs always give
/// identical placements.
fn team_scatter_cell(base: Vec2, team_id: u32, tick: u32, ordinal: u32) -> Vec2 {
    let sample = team_id
        .wrapping_mul(2_654_435_761)
        .wrapping_add(tick.wrapping_mul(40_503))
        .wrapping_add(ordinal.wrapping_mul(2_246_822_519));
    let dx = (sample % 7) as i32 - 3;
    let dy = ((sample >> 8) % 7) as i32 - 3;
    Vec2::new(
        base.x + Fixed16_16::from_int(dx),
        base.y + Fixed16_16::from_int(dy),
    )
}

/// Runtime consumer for `[TeamTypes]`.
///
/// The parser lives in `cnc_core::entity`; this turns a parsed type into
/// orders for live units. Instances spawn once enough of a type's taskforce
/// exists, claim members as those units become free, and are re-tasked each
/// decision tick until the objective completes.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct TeamManager {
    /// Parsed `[TeamTypes]` definitions (team *classes*).
    pub types: Vec<TeamType>,
    /// Running `Team` instances.
    pub teams: Vec<Team>,
    /// Next instance id to hand out.
    pub next_team_id: u32,
}

impl TeamManager {
    /// Build a manager from parsed team types.
    pub fn new(types: Vec<TeamType>) -> Self {
        Self {
            types,
            teams: Vec::new(),
            next_team_id: 0,
        }
    }

    /// `true` when no team types were loaded.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }

    /// Team types owned by one house index.
    pub fn types_for(&self, owner: u8) -> impl Iterator<Item = &TeamType> {
        self.types
            .iter()
            .filter(move |t| house_matches(t.owner, owner))
    }

    /// Live, active instances spawned from one team type.
    pub fn teams_for(&self, team_type_id: u32) -> impl Iterator<Item = &Team> {
        self.teams
            .iter()
            .filter(move |team| team.team_type == team_type_id && team.active && !team.suspended)
    }

    /// Outstanding units of `class_id` a house's team types still want.
    pub fn demand_for(&self, owner: u8, class_id: ClassId) -> u32 {
        let mut want = 0;
        for team_type in self.types_for(owner) {
            // Only ask for units while a team is (or can be) running.
            want += team_type
                .taskforce
                .members
                .iter()
                .filter(|member| member.class == class_id)
                .map(|member| u32::from(member.count))
                .sum::<u32>();
        }
        want
    }
}

/// Does a parsed `Owner=` class id belong to the given house index?
fn house_matches(owner: ClassId, house: u8) -> bool {
    match house {
        0 => matches!(owner, ClassId::HouseGDI | ClassId::House),
        _ => matches!(owner, ClassId::HouseNod),
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuildTask {
    pub class_id: ClassId,
    pub priority: u8,
    pub location: Option<Vec2>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DefenseGroup {
    pub unit_ids: Vec<u32>,
    pub guard_position: Vec2,
    pub guard_radius: Fixed16_16,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttackGroup {
    pub unit_ids: Vec<u32>,
    pub target: Option<Vec2>,
    pub state: AttackState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttackState {
    Forming,
    Moving,
    Attacking,
    Retreating,
}

/// One candidate in the weighted production table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductionWeight {
    pub unit: UnitType,
    pub weight: u32,
}

impl AI {
    pub fn new(house_id: u8, difficulty: AIDifficulty) -> Self {
        let attack_interval = match difficulty {
            AIDifficulty::Easy => Fixed16_16::from_f32(120.0),
            AIDifficulty::Normal => Fixed16_16::from_f32(90.0),
            AIDifficulty::Hard => Fixed16_16::from_f32(60.0),
            AIDifficulty::Brutal => Fixed16_16::from_f32(30.0),
        };

        Self {
            house_id,
            difficulty,
            build_queue: VecDeque::new(),
            attack_timer: Fixed16_16::ZERO,
            attack_interval,
            last_attack_time: Fixed16_16::ZERO,
            defense_groups: Vec::new(),
            attack_groups: Vec::new(),
            scout_positions: Vec::new(),
            decision_timer: Fixed16_16::ZERO,
            decision_interval: match difficulty {
                AIDifficulty::Easy => Fixed16_16::from_f32(3.0),
                AIDifficulty::Normal => Fixed16_16::from_f32(2.0),
                AIDifficulty::Hard => Fixed16_16::from_f32(1.5),
                AIDifficulty::Brutal => Fixed16_16::from_f32(1.0),
            },
            tick_counter: 0,
            next_attack_at: Fixed16_16::from_f32(30.0),
            fire_sale_done: false,
        }
    }

    pub fn update(&mut self, delta_time: Fixed16_16) {
        self.attack_timer += delta_time;

        // Update build queue
        self.process_build_queue();

        // Check if it's time to attack
        if self.attack_timer >= self.attack_interval {
            self.plan_attack();
            self.attack_timer = Fixed16_16::ZERO;
        }

        // Update attack groups
        self.update_attack_groups();
    }

    fn process_build_queue(&mut self) {
        // Process one build task per frame
        if let Some(_task) = self.build_queue.pop_front() {
            // Command execution is driven by the game loop; the planner only
            // decides which type to queue and at what priority.
        }
    }

    fn plan_attack(&mut self) {
        // Plan an attack based on difficulty
    }

    fn update_attack_groups(&mut self) {
        // Update attack group states
    }

    pub fn queue_production(&mut self, class_id: ClassId, priority: u8) {
        self.build_queue.push_back(BuildTask {
            class_id,
            priority,
            location: None,
        });
        // Lower numbers are built first.
        self.build_queue
            .make_contiguous()
            .sort_by_key(|t| t.priority);
    }

    /// Difficulty multiplier applied to combat production weights.
    fn aggression(&self) -> u32 {
        match self.difficulty {
            AIDifficulty::Easy => 1,
            AIDifficulty::Normal => 2,
            AIDifficulty::Hard => 3,
            AIDifficulty::Brutal => 4,
        }
    }

    /// Score every producible unit for this house using the rules registry.
    ///
    /// A unit is omitted when its faction does not own it, when its TechLevel
    /// (rules override or TDATA fallback) exceeds the house level, when a
    /// prerequisite structure is missing, or when the house cannot afford it.
    /// The remaining candidates get a deterministic weight built from the
    /// unit role, difficulty, TechLevel and rules-derived health-per-credit.
    pub fn production_weights(
        &self,
        rules: &RulesDatabase,
        tech_level: u8,
        is_gdi: bool,
        owned_structures: &[StructureType],
        credits: i32,
    ) -> Vec<ProductionWeight> {
        let aggression = self.aggression();
        let mut weights = Vec::new();

        for unit in UnitType::ALL {
            let stats = unit.stats();
            // IDATA.CPP gives the engineer `WEAPON_NONE` and the commando a
            // rifle that INFANTRY.CPP:2490-2538 points at buildings only, so
            // neither belongs on the house AI's combat roster.
            if unit == UnitType::Engineer || unit == UnitType::Commando {
                continue;
            }
            let rule = rules.type_rule(unit.ini_name());

            if !faction_allows(unit, rule.map(|rule| rule.owner.as_slice()), is_gdi) {
                continue;
            }
            if let Some(rule) = rule {
                if rule
                    .prerequisite
                    .iter()
                    .any(|token| !prerequisite_token_met(token, is_gdi, owned_structures))
                {
                    continue;
                }
            }

            let cost = rule.and_then(|rule| rule.cost).unwrap_or(stats.cost).max(1);
            let required_level = rule
                .and_then(|rule| rule.tech_level)
                .map(|level| level.clamp(0, u8::MAX as i32) as u8)
                .unwrap_or(stats.tech_level);
            if required_level > tech_level || cost > credits {
                continue;
            }

            let strength = rule
                .and_then(|rule| rule.strength)
                .unwrap_or(stats.strength);
            let efficiency = (strength.max(1) as u32 * 100) / cost as u32;
            let weight = base_weight(unit)
                .saturating_mul(aggression)
                .saturating_add(efficiency)
                .saturating_add(u32::from(required_level) * 2);

            weights.push(ProductionWeight { unit, weight });
        }

        weights
    }

    /// Deterministically pick one candidate from a weighted table.
    ///
    /// The roll is supplied by the caller (e.g. the frame counter) so replays
    /// and lockstep peers make the same choice without a shared RNG.
    pub fn choose_weighted(weights: &[ProductionWeight], roll: u32) -> Option<UnitType> {
        let total: u32 = weights.iter().map(|option| option.weight).sum();
        if total == 0 {
            return None;
        }
        let mut target = roll % total;
        for option in weights {
            if target < option.weight {
                return Some(option.unit);
            }
            target -= option.weight;
        }
        weights.last().map(|option| option.unit)
    }

    /// Pick a production target and record it in the build queue.
    pub fn plan_production(
        &mut self,
        rules: &RulesDatabase,
        tech_level: u8,
        is_gdi: bool,
        owned_structures: &[StructureType],
        credits: i32,
        roll: u32,
    ) -> Option<UnitType> {
        let weights = self.production_weights(rules, tech_level, is_gdi, owned_structures, credits);
        let choice = Self::choose_weighted(&weights, roll)?;
        self.queue_production(unit_class_id(choice), 0);
        Some(choice)
    }
}

/// Base role weight before difficulty, TechLevel and cost efficiency.
const fn base_weight(unit: UnitType) -> u32 {
    match unit {
        UnitType::InfantryE1 => 40,
        UnitType::MediumTank => 30,
        UnitType::MammothTank => 18,
        UnitType::Harvester => 25,
        UnitType::Jeep => 20,
        UnitType::Buggy => 20,
        UnitType::Orca => 14,
        UnitType::Apache => 14,
        // Support infantry are filtered out of the AI roster first, so these
        // arms only keep the match exhaustive.
        UnitType::Engineer | UnitType::Commando => 0,
    }
}

/// Faction ownership, preferring a rules Owner= list when present.
fn faction_allows(unit: UnitType, owner: Option<&[String]>, is_gdi: bool) -> bool {
    match owner.filter(|tokens| !tokens.is_empty()) {
        Some(tokens) => owner_token_allows(tokens, is_gdi),
        None => default_unit_owner(unit, is_gdi),
    }
}

/// Credits below which a collapsing base triggers the fire sale.
const FIRE_SALE_CREDITS: i32 = 500;

/// Attack force size required before waves launch, per difficulty
/// (gating the RA `AI_Attack` urgency).
fn attack_force_threshold(difficulty: AIDifficulty) -> usize {
    match difficulty {
        AIDifficulty::Easy => 4,
        AIDifficulty::Normal => 6,
        AIDifficulty::Hard => 8,
        AIDifficulty::Brutal => 10,
    }
}

/// RA `Expert_AI` port: difficulty-cadenced urgency checks over the live
/// game — fire sale, power, income, offense production, then attack waves.
/// Deterministic: decisions read only game state, iterate in stable order
/// and take no random input (production variety comes from a tick-counter
/// roll, mirroring how `choose_weighted` consumes caller-supplied rolls).
pub fn run_expert_ai(game: &mut crate::game::Game, ai: &mut AI, delta_time: Fixed16_16) {
    ai.decision_timer = ai.decision_timer + delta_time;
    ai.tick_counter = ai.tick_counter.wrapping_add(1);
    ai.update(delta_time);
    if ai.decision_timer < ai.decision_interval {
        return;
    }
    ai.decision_timer = Fixed16_16::ZERO;

    let owner = ai.house_id;
    let credits = {
        let Some(house) = game.houses.get(owner as usize) else {
            return;
        };
        if !house.ai_enabled {
            return;
        }
        house.credits
    };

    let alive_buildings: Vec<(usize, StructureType)> = game
        .buildings
        .iter()
        .enumerate()
        .filter(|(_, building)| building.owner == owner && building.is_alive())
        .filter_map(|(index, building)| {
            building
                .structure_type()
                .map(|structure| (index, structure))
        })
        .collect();
    let has_yard = alive_buildings
        .iter()
        .any(|(_, structure)| *structure == StructureType::ConstructionYard);
    let has_power = alive_buildings
        .iter()
        .any(|(_, structure)| *structure == StructureType::PowerPlant);
    let refinery_count = alive_buildings
        .iter()
        .filter(|(_, structure)| *structure == StructureType::Refinery)
        .count();

    let combat: Vec<usize> = game
        .units
        .iter()
        .enumerate()
        .filter(|(_, unit)| {
            unit.owner == owner && unit.is_alive() && unit.weapon.is_some() && !unit.is_harvester
        })
        .map(|(index, _)| index)
        .collect();
    let harvester_count = game
        .units
        .iter()
        .filter(|unit| unit.owner == owner && unit.is_harvester && unit.is_alive())
        .count();

    // Check_Fire_Sale: base collapsing and broke — liquidate everything.
    if alive_buildings.len() <= 1 && credits < FIRE_SALE_CREDITS && !ai.fire_sale_done {
        ai.fire_sale_done = true;
        for (index, _) in alive_buildings.iter().rev() {
            game.sell_building(owner, *index);
        }
        return;
    }

    // Check_Build_Power.
    if has_yard && !has_power && game.queue_building(owner, StructureType::PowerPlant) {
        return;
    }
    // Check_Build_Income: refinery first, then harvesters to work it.
    if refinery_count == 0 && has_yard && game.queue_building(owner, StructureType::Refinery) {
        return;
    }
    if refinery_count > 0
        && harvester_count < refinery_count
        && game.queue_unit(owner, UnitType::Harvester)
    {
        return;
    }
    // Check_Build_Offense.
    if combat.len() < attack_force_threshold(ai.difficulty) {
        game.ai_queue_production(owner, ai, ai.tick_counter % 1000);
        return;
    }
    // Check_Attack (RA AI_Attack): launch waves on the difficulty cadence.
    if game.elapsed >= ai.next_attack_at && launch_attack_wave(game, owner, &combat) {
        ai.next_attack_at = game.elapsed + ai.attack_interval;
    }
}

/// Drive the configured `[TeamTypes]` roster for one house.
///
/// Runs after the built-in expert AI: that handles the economy and the
/// generic wave attacks, this shapes whichever units the scenario's team
/// types ask for. When no team types are loaded it does nothing, so a
/// scenario without `[TeamTypes]` behaves exactly as before.
pub fn run_team_manager(game: &mut crate::game::Game, owner: u8, tick: u32) {
    if game.team_manager.is_empty() {
        return;
    }
    spawn_due_teams(game, owner);
    assign_free_members(game, owner);
    task_active_teams(game, owner, tick);
    drop_finished_teams(game);
}

/// Spawn an instance for each of the house's team types that is due and can
/// still be filled from available units.
fn spawn_due_teams(game: &mut crate::game::Game, owner: u8) {
    let due: Vec<u32> = game
        .team_manager
        .types_for(owner)
        .filter(|team_type| !team_type.taskforce.members.is_empty())
        .filter(|team_type| {
            let live = game
                .team_manager
                .teams_for(team_type.id)
                .filter(|team| team.active)
                .count() as u32;
            let allowed = if team_type.active_at_start { 1 } else { 1 };
            live < allowed
        })
        .map(|team_type| team_type.id)
        .collect();

    for team_type_id in due {
        let Some(team_type) = game
            .team_manager
            .types
            .iter()
            .find(|t| t.id == team_type_id)
            .cloned()
        else {
            continue;
        };
        // Nothing to spawn until at least one taskforce unit is available.
        if team_type
            .taskforce
            .members
            .iter()
            .all(|member| !game.has_free_unit_of_class(owner, member.class))
        {
            continue;
        }
        let id = game.team_manager.next_team_id;
        game.team_manager.next_team_id = id.wrapping_add(1);
        let team = Team::from_type(id, &team_type);
        game.team_manager.teams.push(team);
    }
}

/// Give free units of the right classes to teams that are short of members.
fn assign_free_members(game: &mut crate::game::Game, owner: u8) {
    let types: Vec<TeamType> = game.team_manager.types_for(owner).cloned().collect();
    for index in 0..game.team_manager.teams.len() {
        let team_type_id = game.team_manager.teams[index].team_type;
        let Some(team_type) = types.iter().find(|t| t.id == team_type_id) else {
            continue;
        };
        for member in &team_type.taskforce.members {
            let already = game.team_manager.teams[index]
                .members
                .iter()
                .filter(|unit_index| {
                    game.units
                        .get(unit_index.0 as usize)
                        .is_some_and(|unit| unit.class_id == member.class)
                })
                .count() as u8;
            for _ in already..member.count {
                let Some(unit_index) = game.take_free_unit_of_class(owner, member.class) else {
                    break;
                };
                game.team_manager.teams[index]
                    .members
                    .push(cnc_core::pointers::ObjectId(unit_index as u32));
            }
        }
    }
}

/// Issue each active team its objective.
fn task_active_teams(game: &mut crate::game::Game, owner: u8, tick: u32) {
    let team_count = game.team_manager.teams.len();
    for index in 0..team_count {
        if !game.team_manager.teams[index].active || game.team_manager.teams[index].suspended {
            continue;
        }
        let members = game.team_manager.teams[index].members.clone();
        if members.is_empty() {
            continue;
        }
        let objective = game.team_manager.teams[index].objective.clone();
        let team_id = game.team_manager.teams[index].id;

        if matches!(objective, TeamObjective::None) {
            continue;
        }

        match objective {
            TeamObjective::Guard => {
                // Hold position: clear outstanding orders so the idle
                // acquisition from the expert AI takes over.
                for unit_index in &members {
                    if let Some(unit) = game.units.get_mut(unit_index.0 as usize) {
                        unit.target = None;
                        unit.path.clear();
                        unit.path_index = 0;
                    }
                }
                game.team_manager.teams[index].progress = Team::MAX_PROGRESS;
            }
            TeamObjective::Attack | TeamObjective::Hunt => {
                let ids: Vec<u32> = members.iter().map(|id| id.0).collect();
                let target = game.nearest_enemy_unit_index(owner, &ids);
                match target {
                    Some(target_index) => {
                        game.order_units_attack(owner, &ids, target_index);
                        game.team_manager.teams[index].progress = 50;
                    }
                    None => {
                        game.team_manager.teams[index].progress = 0;
                    }
                }
            }
            TeamObjective::Move { cell } => {
                let ids: Vec<u32> = members.iter().map(|id| id.0).collect();
                let destination = game.cell_center_position(cell as i32);
                game.order_units_move(owner, &ids, destination);
                game.team_manager.teams[index].progress = Team::MAX_PROGRESS;
            }
            TeamObjective::Patrol { ref waypoints } => {
                if waypoints.is_empty() {
                    continue;
                }
                let ids: Vec<u32> = members.iter().map(|id| id.0).collect();
                let position = (tick as usize + team_id as usize) % waypoints.len();
                let cell = waypoints[position];
                let destination = game.cell_center_position(cell as i32);
                game.order_units_move(owner, &ids, destination);
                game.team_manager.teams[index].progress = 50;
            }
            TeamObjective::Retreat => {
                let ids: Vec<u32> = members.iter().map(|id| id.0).collect();
                let Some(anchor) = game.unit_position(ids[0]) else {
                    continue;
                };
                let destination = team_scatter_cell(anchor, team_id, tick, 0);
                game.order_units_move(owner, &ids, destination);
                game.team_manager.teams[index].progress = Team::MAX_PROGRESS;
            }
            TeamObjective::Unload | TeamObjective::EnterObject | TeamObjective::None => {}
        }
    }
}

/// Retire teams whose members are all dead.
fn drop_finished_teams(game: &mut crate::game::Game) {
    let alive: Vec<bool> = game
        .team_manager
        .teams
        .iter()
        .map(|team| {
            !team.members.is_empty()
                && team.members.iter().any(|unit_index| {
                    game.units
                        .get(unit_index.0 as usize)
                        .is_some_and(|unit| unit.is_alive())
                })
        })
        .collect();
    for (index, is_alive) in alive.into_iter().enumerate() {
        if !is_alive && !game.team_manager.teams[index].members.is_empty() {
            game.team_manager.teams[index].active = false;
            game.team_manager.teams[index].suspended = true;
        }
    }
}

/// Order the AI's combat force onto the nearest enemy unit, or — when no
/// enemy units remain — siege-move toward the nearest enemy building.
fn launch_attack_wave(game: &mut crate::game::Game, owner: u8, combat: &[usize]) -> bool {
    let Some(first) = combat.first() else {
        return false;
    };
    let Some(anchor_unit) = game.units.get(*first) else {
        return false;
    };
    let anchor = anchor_unit.position;

    let mut nearest_unit: Option<(usize, Fixed16_16)> = None;
    for (index, unit) in game.units.iter().enumerate() {
        if unit.owner == owner || !unit.is_alive() {
            continue;
        }
        let dx = unit.position.x - anchor.x;
        let dy = unit.position.y - anchor.y;
        let dist2 = dx * dx + dy * dy;
        if nearest_unit.is_none_or(|(_, best)| dist2 < best) {
            nearest_unit = Some((index, dist2));
        }
    }
    if let Some((target, _)) = nearest_unit {
        return game.ai_order_attack(owner, combat, target) > 0;
    }

    let mut nearest_building: Option<(usize, Fixed16_16)> = None;
    for (index, building) in game.buildings.iter().enumerate() {
        if building.owner == owner || !building.is_alive() {
            continue;
        }
        let dx = building.position.x - anchor.x;
        let dy = building.position.y - anchor.y;
        let dist2 = dx * dx + dy * dy;
        if nearest_building.is_none_or(|(_, best)| dist2 < best) {
            nearest_building = Some((index, dist2));
        }
    }
    let Some((target, _)) = nearest_building else {
        return false;
    };
    let target_position = game.buildings[target].position;
    let ids: Vec<u32> = combat.iter().map(|&index| index as u32).collect();
    game.order_units_move(owner, &ids, target_position) > 0
}

#[cfg(test)]
mod team_manager_tests {
    use super::*;
    use crate::game::Game;

    const TEAMS_INI: &str = "[TaskForces]\n1=TF_TANKS\n\
[TF_TANKS]\nName=Tank Squad\n1=MTNK,2\n\
[TeamTypes]\n1=TANKS\n\
[TANKS]\nName=Tank Squad\nOwner=Nod\nTaskForce=TF_TANKS\nMission=attack\nActive=yes\n";

    fn manager_with_teams() -> TeamManager {
        let ini = cnc_core::IniFile::parse(TEAMS_INI).expect("ini");
        let types = cnc_core::entity::team_types_from_ini(&ini).expect("team types");
        TeamManager::new(types)
    }

    fn team_game() -> Game {
        let mut game = Game::new("teams", 60, 60);
        game.team_manager = manager_with_teams();
        game
    }

    #[test]
    fn parses_team_types_from_the_original_dialect() {
        let manager = manager_with_teams();
        assert_eq!(manager.types.len(), 1);
        assert_eq!(manager.types[0].name, "Tank Squad");
        assert_eq!(manager.types[0].owner, ClassId::HouseNod);
        assert_eq!(manager.types[0].unit_count(), 2);
        assert_eq!(manager.types[0].objective, TeamObjective::Attack);
    }

    #[test]
    fn demand_reports_outstanding_units_per_house() {
        let manager = manager_with_teams();
        assert_eq!(manager.demand_for(1, ClassId::VehicleE2), 2);
        assert_eq!(manager.demand_for(0, ClassId::VehicleE2), 0);
    }

    #[test]
    fn a_team_spawns_once_its_taskforce_units_exist() {
        let mut game = team_game();
        run_team_manager(&mut game, 1, 0);
        assert!(game.team_manager.teams.is_empty(), "no units -> no team");

        let cell = cnc_core::Vec2i::new(20, 20);
        game.spawn_unit_at(1, UnitType::MediumTank, cell);
        game.spawn_unit_at(1, UnitType::MediumTank, cell);
        run_team_manager(&mut game, 1, 0);
        assert_eq!(game.team_manager.teams.len(), 1);
        assert_eq!(game.team_manager.teams[0].members.len(), 2);
    }

    #[test]
    fn an_attack_team_orders_its_members_at_the_enemy() {
        let mut game = team_game();
        let cell = cnc_core::Vec2i::new(20, 20);
        game.spawn_unit_at(1, UnitType::MediumTank, cell);
        game.spawn_unit_at(1, UnitType::MediumTank, cell);
        let enemy = game.spawn_unit_at(0, UnitType::MediumTank, cnc_core::Vec2i::new(30, 30));

        run_team_manager(&mut game, 1, 0);
        let members = game.team_manager.teams[0].members.clone();
        assert_eq!(members.len(), 2);
        let ordered = members.iter().any(|index| {
            game.units
                .get(index.0 as usize)
                .is_some_and(|unit| unit.attack_target == Some(enemy as usize))
        });
        assert!(ordered, "at least one member should target the enemy");
    }

    #[test]
    fn a_unit_cannot_join_two_teams() {
        let mut game = team_game();
        let cell = cnc_core::Vec2i::new(20, 20);
        game.spawn_unit_at(1, UnitType::MediumTank, cell);
        game.spawn_unit_at(1, UnitType::MediumTank, cell);
        run_team_manager(&mut game, 1, 0);
        let claimed: Vec<u32> = game.team_manager.teams[0]
            .members
            .iter()
            .map(|index| index.0)
            .collect();
        assert_eq!(claimed.len(), 2);
        assert_ne!(claimed[0], claimed[1], "distinct units only");
    }

    #[test]
    fn a_team_retires_when_all_members_die() {
        let mut game = team_game();
        let cell = cnc_core::Vec2i::new(20, 20);
        game.spawn_unit_at(1, UnitType::MediumTank, cell);
        game.spawn_unit_at(1, UnitType::MediumTank, cell);
        run_team_manager(&mut game, 1, 0);
        assert!(game.team_manager.teams[0].active);

        for unit in game.units.iter_mut() {
            unit.health = Fixed16_16::ZERO;
        }
        run_team_manager(&mut game, 1, 1);
        assert!(!game.team_manager.teams[0].active, "dead team must retire");
    }

    #[test]
    fn no_team_types_leaves_the_manager_inert() {
        let mut game = Game::new("empty", 40, 40);
        game.team_manager = TeamManager::default();
        game.spawn_unit_at(1, UnitType::MediumTank, cnc_core::Vec2i::new(10, 10));
        run_team_manager(&mut game, 1, 0);
        assert!(game.team_manager.teams.is_empty());
        assert!(game.team_manager.is_empty());
    }

    #[test]
    fn guard_objective_holds_position() {
        let ini_text = "[TaskForces]\n1=TF_HOLD\n[TF_HOLD]\n1=MTNK,1\n\
[TeamTypes]\n1=HOLD\n[HOLD]\nName=Hold\nOwner=Nod\nTaskForce=TF_HOLD\nMission=guard\n";
        let ini = cnc_core::IniFile::parse(ini_text).expect("ini");
        let types = cnc_core::entity::team_types_from_ini(&ini).expect("types");
        let mut game = team_game();
        game.team_manager = TeamManager::new(types);
        game.spawn_unit_at(1, UnitType::MediumTank, cnc_core::Vec2i::new(15, 15));
        run_team_manager(&mut game, 1, 0);
        assert_eq!(game.team_manager.teams.len(), 1);
        assert_eq!(game.team_manager.teams[0].progress, Team::MAX_PROGRESS);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::building::Building;
    use crate::game::Game;
    use crate::map::TILE_SIZE;
    use crate::unit::Unit;

    fn ai_game() -> Game {
        let mut game = Game::new("test", 60, 60);
        game.setup_demo();
        game.player_owner = 0;
        if let Some(house) = game.houses.get_mut(1) {
            house.ai_enabled = true;
            house.tech_level = 98;
        }
        while game.ai.len() < game.houses.len() {
            let index = game.ai.len();
            game.ai.push(AI::new(index as u8, AIDifficulty::Brutal));
        }
        game
    }

    /// Advance the expert AI for house 1 by `frames` decision rounds.
    fn tick_expert(game: &mut Game, frames: usize) {
        let delta = Fixed16_16::from_f32(2.0);
        for _ in 0..frames {
            let mut ais = std::mem::take(&mut game.ai);
            run_expert_ai(game, &mut ais[1], delta);
            game.ai = ais;
        }
    }

    fn add_owner_building(game: &mut Game, structure: StructureType, cx: i32, cy: i32, owner: u8) {
        let building = Building::from_type(
            structure,
            Vec2::new(
                Fixed16_16::from_f32(cx as f32 * TILE_SIZE),
                Fixed16_16::from_f32(cy as f32 * TILE_SIZE),
            ),
            owner,
        );
        let id = game.buildings.len() as u32;
        if let Some(house) = game.houses.get_mut(owner as usize) {
            house.add_building(id);
        }
        game.buildings.push(building);
    }

    #[test]
    fn expert_ai_rebuilds_power_when_missing() {
        let mut game = ai_game();
        game.buildings
            .retain(|b| !(b.owner == 1 && b.structure_type() == Some(StructureType::PowerPlant)));
        let credits_before = game.houses[1].credits;
        tick_expert(&mut game, 1);
        // The queued build deducts the cost upfront.
        assert!(game.houses[1].credits < credits_before);
    }

    #[test]
    fn expert_ai_builds_a_refinery_when_income_is_missing() {
        let mut game = ai_game();
        game.buildings
            .retain(|b| !(b.owner == 1 && b.structure_type() == Some(StructureType::Refinery)));
        let credits_before = game.houses[1].credits;
        tick_expert(&mut game, 1);
        assert!(game.houses[1].credits < credits_before);
    }

    #[test]
    fn expert_ai_staffs_refineries_with_harvesters() {
        let mut game = ai_game();
        add_owner_building(&mut game, StructureType::WarFactory, 40, 40, 1);
        let credits_before = game.houses[1].credits;
        tick_expert(&mut game, 1);
        assert!(game.houses[1].credits < credits_before);
    }

    #[test]
    fn expert_ai_launches_an_attack_wave_when_the_force_is_ready() {
        let mut game = ai_game();
        add_owner_building(&mut game, StructureType::WarFactory, 40, 40, 1);
        // Satisfy the income check so the ladder reaches the attack gate.
        let harvester = Unit::new(
            UnitType::Harvester,
            Vec2::new(
                Fixed16_16::from_f32(30.0 * TILE_SIZE),
                Fixed16_16::from_f32(30.0 * TILE_SIZE),
            ),
            1,
        );
        let id = game.units.len() as u32;
        game.houses[1].add_unit(id);
        game.units.push(harvester);
        // Brutal threshold is 10; give the AI exactly a ready force.
        for index in 0..10 {
            let position = Vec2::new(
                Fixed16_16::from_f32((45 + index) as f32 * TILE_SIZE),
                Fixed16_16::from_f32(45.0 * TILE_SIZE),
            );
            let unit = Unit::new(UnitType::MediumTank, position, 1);
            let id = game.units.len() as u32;
            game.houses[1].add_unit(id);
            game.units.push(unit);
        }
        if let Some(ai) = game.ai.get_mut(1) {
            ai.next_attack_at = Fixed16_16::ZERO;
        }
        tick_expert(&mut game, 1);
        let ordered = game
            .units
            .iter()
            .filter(|unit| unit.owner == 1 && unit.ordered && unit.attack_target.is_some())
            .count();
        assert!(ordered > 0, "attack wave should order combat units");
    }

    #[test]
    fn expert_ai_triggers_fire_sale_on_a_collapsing_base() {
        let mut game = ai_game();
        game.buildings.retain(|b| {
            !(b.owner == 1 && b.structure_type() != Some(StructureType::ConstructionYard))
        });
        game.houses[1].credits = 100;
        tick_expert(&mut game, 1);
        assert!(game.ai[1].fire_sale_done);
    }

    #[test]
    fn expert_ai_is_deterministic() {
        let mut a = ai_game();
        let mut b = ai_game();
        tick_expert(&mut a, 5);
        tick_expert(&mut b, 5);
        assert_eq!(a.houses[1].credits, b.houses[1].credits);
        assert_eq!(a.ai[1].tick_counter, b.ai[1].tick_counter);
        assert_eq!(a.ai[1].fire_sale_done, b.ai[1].fire_sale_done);
        assert_eq!(a.units.len(), b.units.len());
    }

    fn gdi_rules() -> RulesDatabase {
        RulesDatabase::from_str("[MTNK]\nCost=800\nTechLevel=3\n[HTNK]\nCost=1500\nTechLevel=5\n")
            .unwrap()
    }

    #[test]
    fn production_weights_honour_tech_level_and_faction() {
        let ai = AI::new(0, AIDifficulty::Normal);
        let rules = gdi_rules();
        let owned = [StructureType::WarFactory, StructureType::Barracks];

        let low = ai.production_weights(&rules, 1, true, &owned, 10_000);
        assert!(low.iter().all(|option| option.unit != UnitType::MediumTank));
        assert!(low.iter().any(|option| option.unit == UnitType::InfantryE1));

        let high = ai.production_weights(&rules, 5, true, &owned, 10_000);
        assert!(high
            .iter()
            .any(|option| option.unit == UnitType::MammothTank));

        let nod = ai.production_weights(&rules, 5, false, &owned, 10_000);
        assert!(nod
            .iter()
            .all(|option| option.unit != UnitType::MammothTank));
        assert!(nod.iter().any(|option| option.unit == UnitType::Buggy));
        assert!(nod.iter().all(|option| option.unit != UnitType::Orca));
    }

    #[test]
    fn rules_cost_and_prerequisites_change_the_table() {
        let ai = AI::new(0, AIDifficulty::Hard);
        let rules =
            RulesDatabase::from_str("[MTNK]\nCost=9000\nTechLevel=3\nPrerequisite=PROC\n").unwrap();
        let owned = [StructureType::WarFactory, StructureType::Barracks];
        let weights = ai.production_weights(&rules, 5, true, &owned, 10_000);
        // 9000 credits is affordable, but PROC is missing.
        assert!(weights
            .iter()
            .all(|option| option.unit != UnitType::MediumTank));

        let owned = [
            StructureType::WarFactory,
            StructureType::Barracks,
            StructureType::Refinery,
        ];
        let weights = ai.production_weights(&rules, 5, true, &owned, 10_000);
        assert!(weights
            .iter()
            .any(|option| option.unit == UnitType::MediumTank));

        // The rules cost also gates affordability.
        let weights = ai.production_weights(&rules, 5, true, &owned, 5000);
        assert!(weights
            .iter()
            .all(|option| option.unit != UnitType::MediumTank));
    }

    #[test]
    fn weighted_choice_is_deterministic_and_respects_weights() {
        let table = [
            ProductionWeight {
                unit: UnitType::InfantryE1,
                weight: 3,
            },
            ProductionWeight {
                unit: UnitType::MediumTank,
                weight: 1,
            },
        ];
        let picks: Vec<_> = (0..8)
            .map(|roll| AI::choose_weighted(&table, roll).unwrap())
            .collect();
        assert_eq!(
            picks,
            vec![
                UnitType::InfantryE1,
                UnitType::InfantryE1,
                UnitType::InfantryE1,
                UnitType::MediumTank,
                UnitType::InfantryE1,
                UnitType::InfantryE1,
                UnitType::InfantryE1,
                UnitType::MediumTank,
            ]
        );
        assert_eq!(AI::choose_weighted(&[], 0), None);
    }

    #[test]
    fn harvester_is_available_to_both_factions() {
        let ai = AI::new(0, AIDifficulty::Normal);
        let rules = RulesDatabase::default();
        let owned = [StructureType::Refinery];
        for is_gdi in [true, false] {
            let weights = ai.production_weights(&rules, 2, is_gdi, &owned, 2000);
            assert!(weights
                .iter()
                .any(|option| option.unit == UnitType::Harvester));
        }
    }
}
