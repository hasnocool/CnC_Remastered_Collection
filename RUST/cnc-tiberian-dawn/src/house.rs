//! House/Faction logic for Tiberian Dawn

use cnc_core::{ClassId, Fixed16_16};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Production queue item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionItem {
    pub class_id: ClassId,
    pub progress: Fixed16_16,
    pub total_time: Fixed16_16,
    pub cost: i32,
}

/// House/Faction (GDI or Nod)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct House {
    pub id: ClassId,
    pub name: String,
    pub credits: i32,
    /// Stored tiberium (HouseClass::Tiberium).
    pub tiberium: i32,
    /// Tiberium storage capacity from refineries and silos.
    pub capacity: i32,
    /// HOUSE.CPP MaxUnit: cap on simultaneously owned units.
    pub max_units: u32,
    /// HOUSE.CPP MaxBuilding: cap on simultaneously owned structures.
    pub max_buildings: u32,
    /// HOUSE.CPP Edge: where this house's reinforcements enter the map.
    #[serde(default)]
    pub edge: crate::game::ReinforcementEdge,
    pub harvested_credits: i64,
    pub credits_spent: i64,
    /// Ion Cannon charge (0..1), from HOUSE.CPP IonCannon.
    pub ion_charge: Fixed16_16,
    /// Nuclear strike charge (0..1), from HOUSE.CPP NukeStrike.
    pub nuke_charge: Fixed16_16,
    pub power: i32,
    pub max_power: i32,
    pub production_queue: VecDeque<ProductionItem>,
    pub buildings: Vec<u32>,
    pub units: Vec<u32>,
    /// Build level gating Rules.ini TechLevel. Multiplayer/skirmish uses 98
    /// (everything buildable); scenarios may lower it via the scenario setup.
    pub tech_level: u8,
    pub ai_enabled: bool,
    #[serde(default)]
    pub handicap: cnc_core::FloatBias,
    #[serde(default)]
    pub credit_display: i32,
    #[serde(default)]
    pub credit_countdown: i32,
    #[serde(default)]
    pub allies: Vec<u8>,
    #[serde(default)]
    pub alliance_members: Vec<u8>,
    #[serde(default)]
    pub allied_houses: Vec<u8>,
    #[serde(default)]
    pub members: Vec<u8>,
}

impl House {
    pub fn new(id: ClassId, name: String) -> Self {
        Self {
            id,
            name,
            credits: 0,
            tiberium: 0,
            capacity: 0,
            harvested_credits: 0,
            credits_spent: 0,
            ion_charge: Fixed16_16::ZERO,
            nuke_charge: Fixed16_16::ZERO,
            power: 0,
            max_power: 0,
            production_queue: VecDeque::new(),
            buildings: Vec::new(),
            units: Vec::new(),
            tech_level: 98,
            // HOUSE.CPP clamps MaxUnit/MaxBuilding to at least 150.
            max_units: 150,
            max_buildings: 150,
            edge: crate::game::ReinforcementEdge::North,
            ai_enabled: false,
            handicap: cnc_core::FloatBias::ZERO,
            credit_display: 0,
            credit_countdown: 0,
            allies: Vec::new(),
            alliance_members: Vec::new(),
            allied_houses: Vec::new(),
            members: Vec::new(),
        }
    }
    pub fn add_credits(&mut self, amount: i32) {
        self.credits += amount;
    }

    /// HOUSE.CPP HouseClass::Available_Money.
    pub fn available_money(&self) -> i32 {
        self.tiberium + self.credits
    }

    pub fn can_afford(&self, cost: i32) -> bool {
        self.available_money() >= cost
    }

    /// HOUSE.CPP HouseClass::Spend_Money: stored tiberium is spent first.
    pub fn spend_credits(&mut self, cost: i32) -> bool {
        if !self.can_afford(cost) {
            return false;
        }
        let mut remaining = cost;
        if remaining > self.tiberium {
            remaining -= self.tiberium;
            self.tiberium = 0;
            self.credits -= remaining;
        } else {
            self.tiberium -= remaining;
        }
        self.credits_spent += cost as i64;
        true
    }

    /// HOUSE.CPP HouseClass::Harvested: add tiberium up to capacity.
    pub fn harvested(&mut self, amount: i32) -> i32 {
        let before = self.tiberium;
        self.tiberium = (self.tiberium + amount).min(self.capacity);
        self.harvested_credits += amount as i64;
        self.tiberium - before
    }

    /// Advance superweapon timers.
    ///
    /// DEFINES.H: ION_CANNON_GONE_TIME = 10 minutes, NUKE_GONE_TIME = 14 minutes.
    pub fn charge_superweapons(
        &mut self,
        ion_ready: bool,
        temple_ready: bool,
        delta_time: Fixed16_16,
    ) {
        if ion_ready {
            self.ion_charge =
                (self.ion_charge + delta_time / Fixed16_16::from_int(600)).min(Fixed16_16::ONE);
        }
        if temple_ready {
            self.nuke_charge =
                (self.nuke_charge + delta_time / Fixed16_16::from_int(840)).min(Fixed16_16::ONE);
        }
    }

    /// HOUSE.CPP HouseClass::Adjust_Capacity: returns credits lost to overflow.
    pub fn adjust_capacity(&mut self, adjust: i32) -> i32 {
        self.capacity += adjust;
        self.capacity = self.capacity.max(0);
        if self.tiberium > self.capacity {
            let lost = self.tiberium - self.capacity;
            self.tiberium = self.capacity;
            lost
        } else {
            0
        }
    }
    pub fn add_power(&mut self, amount: i32) {
        self.power += amount;
        self.max_power += amount;
    }
    pub fn remove_power(&mut self, amount: i32) {
        self.power = self.power.saturating_sub(amount);
        self.max_power = self.max_power.saturating_sub(amount);
    }
    pub fn has_power(&self) -> bool {
        self.power >= 0
    }
    pub fn queue_production(&mut self, item: ProductionItem) -> bool {
        let mut item = item;
        item.cost = self.handicap_cost(item.cost);
        item.total_time = self.handicap_time(item.total_time);
        if self.spend_credits(item.cost) {
            self.production_queue.push_back(item);
            true
        } else {
            false
        }
    }
    pub fn update_production(&mut self, delta_time: Fixed16_16) -> Option<ProductionItem> {
        if let Some(item) = self.production_queue.front_mut() {
            item.progress += delta_time;
            if item.progress >= item.total_time {
                return Some(self.production_queue.pop_front().unwrap());
            }
        }
        None
    }
    pub fn add_building(&mut self, entity_id: u32) {
        self.buildings.push(entity_id);
    }
    pub fn remove_building(&mut self, entity_id: u32) {
        self.buildings.retain(|&id| id != entity_id);
    }
    pub fn add_unit(&mut self, entity_id: u32) {
        self.units.push(entity_id);
    }
    pub fn remove_unit(&mut self, entity_id: u32) {
        self.units.retain(|&id| id != entity_id);
    }
    pub fn building_count(&self) -> usize {
        self.buildings.len()
    }
    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    // --- Handicap ---
    pub fn assign_handicap(&mut self, handicap: cnc_core::FloatBias) -> cnc_core::FloatBias {
        let old = self.handicap;
        self.handicap = handicap;
        old
    }
    pub fn set_handicap(&mut self, handicap: cnc_core::FloatBias) {
        self.handicap = handicap;
    }
    pub fn with_handicap(mut self, handicap: cnc_core::FloatBias) -> Self {
        self.handicap = handicap;
        self
    }
    pub fn has_handicap(&self) -> bool {
        self.handicap != cnc_core::FloatBias::ZERO
    }
    pub fn handicap_cost(&self, cost: i32) -> i32 {
        let bias = self.handicap.to_f32();
        if !bias.is_finite() || bias <= 0.0 || self.handicap == cnc_core::FloatBias::ZERO {
            return cost;
        }
        if cost <= 0 {
            return cost;
        }
        let value = (cost as f64) * (bias as f64);
        if !value.is_finite() {
            return cost;
        }
        let rounded = value.round();
        let clamped = rounded.clamp(0.0, i32::MAX as f64);
        clamped as i32
    }
    pub fn handicap_time(&self, time: Fixed16_16) -> Fixed16_16 {
        let bias = self.handicap.to_f32();
        if !bias.is_finite() || bias <= 0.0 || self.handicap == cnc_core::FloatBias::ZERO {
            return time;
        }
        let value = time.to_f32() * bias;
        if !value.is_finite() {
            return time;
        }
        Fixed16_16::from_f32(value)
    }

    // --- Credit display tick (CREDITS.CPP) ---
    fn sync_allies(&mut self) {
        self.alliance_members = self.allies.clone();
        self.allied_houses = self.allies.clone();
        self.members = self.allies.clone();
    }
    pub fn tick_credits(&mut self) {
        let target = self.available_money().max(0);
        if self.credit_display == target {
            return;
        }
        if self.credit_countdown > 0 {
            self.credit_countdown -= 1;
            if self.credit_countdown > 0 {
                return;
            }
        }
        let diff = (target - self.credit_display).abs();
        let mut adder = diff >> 5;
        adder = adder.clamp(1, 143);
        if self.credit_display > target {
            adder = -adder;
        }
        if adder.abs() > diff {
            adder = if target > self.credit_display {
                diff
            } else {
                -diff
            };
        }
        self.credit_display += adder;
        self.credit_countdown = 1;
    }
    pub fn credit_tick(&mut self) {
        self.tick_credits();
    }
    pub fn update_credit_display(&mut self) {
        self.tick_credits();
    }
    pub fn advance_credits(&mut self) {
        self.tick_credits();
    }
    pub fn advance_credit_display(&mut self) {
        self.tick_credits();
    }
    pub fn tick_credit_display(&mut self) {
        self.tick_credits();
    }
    pub fn force_credit_display(&mut self) {
        self.credit_display = self.available_money().max(0);
        self.credit_countdown = 0;
    }
    pub fn force_credits(&mut self) {
        self.force_credit_display();
    }
    pub fn reset_credit_display(&mut self) {
        self.force_credit_display();
    }

    // --- Alliances ---
    pub fn is_ally(&self, house: u8) -> bool {
        self.allies.contains(&house)
    }
    pub fn is_allied(&self, house: u8) -> bool {
        self.is_ally(house)
    }
    pub fn make_ally(&mut self, house: u8) {
        if !self.allies.contains(&house) {
            self.allies.push(house);
            self.sync_allies();
        }
    }
    pub fn make_allied(&mut self, house: u8) {
        self.make_ally(house);
    }
    pub fn ally(&mut self, house: u8) {
        self.make_ally(house);
    }
    pub fn set_ally(&mut self, house: u8) {
        self.make_ally(house);
    }
    pub fn add_ally(&mut self, house: u8) {
        self.make_ally(house);
    }
    pub fn add_alliance(&mut self, house: u8) {
        self.make_ally(house);
    }
    pub fn make_enemy(&mut self, house: u8) {
        self.allies.retain(|&h| h != house);
        self.sync_allies();
    }
    pub fn remove_ally(&mut self, house: u8) {
        self.make_enemy(house);
    }
    pub fn remove_alliance(&mut self, house: u8) {
        self.make_enemy(house);
    }
    pub fn allies_list(&self) -> &[u8] {
        &self.allies
    }
    pub fn alliance_list(&self) -> &[u8] {
        &self.allies
    }
    pub fn members_list(&self) -> &[u8] {
        &self.allies
    }

    pub fn credit_ticks(&mut self, ticks: i32) {
        if ticks <= 0 {
            return;
        }
        let n = ticks.min(100_000);
        for _ in 0..n {
            self.tick_credits();
        }
    }
    pub fn tick_credits_n(&mut self, ticks: i32) {
        self.credit_ticks(ticks);
    }
    pub fn tick_n_credits(&mut self, ticks: i32) {
        self.credit_ticks(ticks);
    }
    pub fn credits_tick(&mut self, ticks: i32) {
        self.credit_ticks(ticks);
    }
    pub fn credits_ticks(&mut self, ticks: i32) {
        self.credit_ticks(ticks);
    }
    pub fn apply_credit_ticks(&mut self, ticks: i32) {
        self.credit_ticks(ticks);
    }
    pub fn tick_credit(&mut self, ticks: i32) {
        self.credit_ticks(ticks);
    }

    pub fn credit_display_value(&self) -> i32 {
        self.credit_display
    }
    pub fn credits_display(&self) -> i32 {
        self.credit_display
    }
    pub fn displayed_credits(&self) -> i32 {
        self.credit_display
    }
    pub fn current_credits(&self) -> i32 {
        self.credit_display
    }

    pub fn handicap_value(&self) -> cnc_core::FloatBias {
        self.handicap
    }
    pub fn handicap_bias(&self) -> cnc_core::FloatBias {
        self.handicap
    }
    pub fn cost_bias(&self) -> cnc_core::FloatBias {
        if self.handicap == cnc_core::FloatBias::ZERO {
            cnc_core::FloatBias::ONE
        } else {
            self.handicap
        }
    }
    pub fn build_speed_bias(&self) -> cnc_core::FloatBias {
        self.cost_bias()
    }

    pub fn is_allied_with(&self, house: u8) -> bool {
        self.is_ally(house)
    }
    pub fn alliance_members_list(&self) -> &[u8] {
        &self.allies
    }
    pub fn allied_members_list(&self) -> &[u8] {
        &self.allies
    }
}

impl Default for House {
    fn default() -> Self {
        Self::new(ClassId::House, "Default".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_core::ClassId;
    use cnc_core::Fixed16_16;
    use cnc_core::FloatBias;

    #[test]
    fn handicap_alters_economy() {
        let h_easy =
            House::new(ClassId::House, "Easy".to_string()).with_handicap(FloatBias::from_f32(0.75));
        let h_normal =
            House::new(ClassId::House, "Normal".to_string()).with_handicap(FloatBias::ZERO);
        let cost = 200;
        let time = Fixed16_16::from_int(120);
        let cost_easy = h_easy.handicap_cost(cost);
        let time_easy = h_easy.handicap_time(time);
        let cost_normal = h_normal.handicap_cost(cost);
        let time_normal = h_normal.handicap_time(time);
        assert_eq!(cost_normal, cost);
        assert_eq!(time_normal, time);
        let expected_cost = ((cost as f32) * 0.75).round() as i32;
        assert_eq!(cost_easy, expected_cost);
        assert!((time_easy.to_f32() - time.to_f32() * 0.75).abs() < 1e-3);
    }

    #[test]
    fn credit_tick_rolls_up_gradually() {
        let mut h = House::new(ClassId::House, "G".to_string());
        h.credits = 1000;
        h.credit_display = 0;
        h.credit_countdown = 0;
        let mut steps = 0;
        while h.credit_display < h.available_money() && steps < 10000 {
            h.tick_credits();
            steps += 1;
        }
        assert_eq!(h.credit_display, h.available_money());
        let mut h2 = House::new(ClassId::House, "G".to_string());
        h2.credits = 1000;
        h2.credit_display = 0;
        h2.tick_credits();
        assert!(h2.credit_display > 0 && h2.credit_display < 1000);
    }

    #[test]
    fn ally_event_sets_both_houses() {
        let mut game = crate::game::Game::new("t", 10, 10);
        game.setup_demo();
        assert!(game.houses.len() >= 2);
        let a = 0u8;
        let b = 1u8;
        let ok = game.ally_houses(a, b);
        assert!(ok);
        assert!(game.houses[a as usize].is_ally(b));
        assert!(game.houses[b as usize].is_ally(a));
        let mut h = House::new(ClassId::House, "X".to_string());
        assert!(!h.is_ally(5));
    }
}
