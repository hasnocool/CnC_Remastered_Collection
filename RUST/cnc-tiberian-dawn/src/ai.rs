//! AI system for Tiberian Dawn
//!
//! Production planning is driven by the Rules.ini registry: costs, TechLevel
//! and prerequisites are read from the loaded rules database, with the
//! hard-coded TDATA/UDATA/BDATA values as fallback. Selection is deterministic
//! (a caller-supplied roll), so AI decisions stay lockstep-safe.

use crate::td_rules::{prerequisite_token_met, unit_class_id, StructureType, UnitType};
use cnc_core::rules::RulesDatabase;
use cnc_core::{ClassId, Fixed16_16, Vec2};
use std::collections::VecDeque;

/// AI difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AIDifficulty {
    Easy,
    Normal,
    Hard,
    Brutal,
}

/// AI player state
#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct BuildTask {
    pub class_id: ClassId,
    pub priority: u8,
    pub location: Option<Vec2>,
}

#[derive(Debug, Clone)]
pub struct DefenseGroup {
    pub unit_ids: Vec<u32>,
    pub guard_position: Vec2,
    pub guard_radius: Fixed16_16,
}

#[derive(Debug, Clone)]
pub struct AttackGroup {
    pub unit_ids: Vec<u32>,
    pub target: Option<Vec2>,
    pub state: AttackState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    }
}

/// Faction ownership, preferring a rules Owner= list when present.
fn faction_allows(unit: UnitType, owner: Option<&[String]>, is_gdi: bool) -> bool {
    let Some(tokens) = owner.filter(|tokens| !tokens.is_empty()) else {
        return default_owner(unit, is_gdi);
    };

    tokens
        .iter()
        .any(|token| match token.trim().to_ascii_uppercase().as_str() {
            "GDI" | "GOOD" => is_gdi,
            "NOD" | "BAD" => !is_gdi,
            "ANY" | "ALL" => true,
            _ => false,
        })
}

/// Hard-coded TD ownership for rules files that omit Owner=.
const fn default_owner(unit: UnitType, is_gdi: bool) -> bool {
    match unit {
        UnitType::InfantryE1 | UnitType::Harvester => true,
        UnitType::MediumTank | UnitType::MammothTank | UnitType::Jeep | UnitType::Orca => is_gdi,
        UnitType::Buggy | UnitType::Apache => !is_gdi,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
