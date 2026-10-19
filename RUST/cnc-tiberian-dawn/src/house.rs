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
            ai_enabled: false,
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
}

impl Default for House {
    fn default() -> Self {
        Self::new(ClassId::House, "Default".to_string())
    }
}
