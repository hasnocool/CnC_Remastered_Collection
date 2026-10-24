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
    /// AI IQ level — mirrors HOUSE.CPP `IQ` (HOUSE.H:869).
    /// Controls what behaviors the AI understands: harvest, repair, production, etc.
    #[serde(default)]
    pub iq: u8,
    pub ai_enabled: bool,
    /// Difficulty bias for this house — `HouseClass::Assign_Handicap` (HOUSE.CPP:239).
    ///
    /// `FloatBias` is a **multiplier**, never a delta: it is scaled into the
    /// purchase price (`TechnoClass::Refund_Amount`, TECHNO.CPP:4495
    /// `Raw_Cost() * House->CostBias`; `FactoryClass::Set`, FACTORY.CPP:402
    /// `object.Cost_Of() * house.CostBias`) and into the sidebar price
    /// (SIDEBAR.CPP:2292), and it scales build time the same way.
    ///
    /// `ZERO` is the "no handicap" sentinel as well as the `#[serde(default)]`
    /// value for a snapshot that predates this field, so it must behave exactly
    /// like an unhandicapped house. See [`House::bias_is_effective`].
    #[serde(default)]
    pub handicap: cnc_core::FloatBias,
    /// Credits currently shown on the credit tab — `CreditClass::Current`
    /// (CREDITS.CPP:61).
    ///
    /// Display state, not ledger state: it rolls up toward
    /// [`House::available_money`] a slice at a time
    /// (`CreditClass::AI`, CREDITS.CPP:138-177). It is snapshot-serialized so a
    /// replay reproduces the same roll-up.
    #[serde(default)]
    pub credit_display: i32,
    /// Ticks to wait before the next roll-up step — `CreditClass::Countdown`
    /// (CREDITS.CPP:63, :158-159, :171). Snapshot state for the same reason.
    #[serde(default)]
    pub credit_countdown: i32,
    /// The set of houses allied with this one — `HouseClass::Allies`
    /// (HOUSE.H:688, a bitmask; exposed by `Get_Allies`, HOUSE.H:447).
    ///
    /// One canonical field, deliberately: the original keeps a single bitmask,
    /// so every accessor spelling ([`House::allies_list`],
    /// [`House::alliance_members_list`], [`House::members_list`], …) reads this
    /// The set of houses allied with this one — `HouseClass::Allies`
    /// (HOUSE.H:688, a bitmask; exposed by `Get_Allies`, HOUSE.H:447).
    ///
    /// One canonical field, deliberately: the original keeps a single bitmask,
    /// so every accessor spelling ([`House::allies_list`],
    /// [`House::alliance_members_list`], [`House::members_list`], …) reads this
    /// same set and no second copy has to be kept in sync.
    #[serde(default)]
    pub allies: Vec<u8>,
    /// `Autocreate` flag — set by `TACTION_AUTOCREATE` (ACTION_AUTOCREATE).
    /// When true, the house's `Autocreate=yes` team types will spawn.
    /// Mirrors `HouseClass::IsAlerted` in the original (HOUSE.CPP).
    #[serde(default)]
    pub autocreate: bool,
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
            autocreate: false,
            iq: 0,
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
    /// Whether this house's handicap actually changes the economy.
    ///
    /// The bias is a multiplier over cost and build time (HOUSE.CPP's
    /// CostBias/BuildSpeedBias), so only a finite, strictly positive value can
    /// scale anything. A zero bias is the serde "unset" sentinel and must leave
    /// the unhandicapped value untouched so that pre-handicap snapshots stay
    /// byte-identical; a non-finite or negative bias has no defined meaning and
    /// is likewise treated as "no handicap" rather than propagating NaN into
    /// the ledger.
    pub fn bias_is_effective(&self) -> bool {
        let bias = self.handicap.to_f32();
        bias.is_finite() && bias > 0.0
    }
    pub fn has_handicap(&self) -> bool {
        self.bias_is_effective()
    }
    /// The biased purchase price for a raw cost.
    ///
    /// The multiplication mirrors TECHNO.CPP:4495 / FACTORY.CPP:402
    /// (`Raw_Cost() * House->CostBias`): the handicap IS the CostBias
    /// multiplier, so an easy house pays 0.75× the raw cost. The exact
    /// rounding is ours (the C++ stores an `int Balance`), so a test pins it
    /// rather than trusting it.
    pub fn handicap_cost(&self, cost: i32) -> i32 {
        if !self.bias_is_effective() || cost <= 0 {
            return cost;
        }
        let value = (cost as f64) * (self.handicap.to_f32() as f64);
        if !value.is_finite() {
            return cost;
        }
        let rounded = value.round();
        let clamped = rounded.clamp(0.0, i32::MAX as f64);
        clamped as i32
    }
    /// The biased build time, scaled by the same multiplier as the price.
    pub fn handicap_time(&self, time: Fixed16_16) -> Fixed16_16 {
        if !self.bias_is_effective() {
            return time;
        }
        let value = time.to_f32() * self.handicap.to_f32();
        if !value.is_finite() {
            return time;
        }
        Fixed16_16::from_f32(value)
    }

    // --- Credit display tick (CREDITS.CPP) ---
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
        // HOUSE.CPP:2095 — Make_Ally is a no-op when the house is already an
        // ally (`if (house != HOUSE_NONE && !Is_Ally(house))`), so a repeat
        // request cannot un-ally. Breaking an alliance is Make_Enemy's job
        // (HOUSE.CPP:2158), which is always bilateral (:2164-2166).
        if !self.allies.contains(&house) {
            self.allies.push(house);
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
        if self.bias_is_effective() {
            self.handicap
        } else {
            cnc_core::FloatBias::ONE
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
        let h = House::new(ClassId::House, "X".to_string());
        assert!(!h.is_ally(5));
    }
}
