//! Red Alert houses, factions and production.
//!
//! Red Alert uses the same 16.16 fixed-point economy as Tiberian Dawn but has
//! two factions and five countries with a unique structure each. This module
//! models the house-level state that the scenario layer needs: treasury,
//! power balance, tech level and a production queue with deterministic
//! completion order.

use cnc_core::{ClassId, Fixed16_16};
use std::collections::VecDeque;

/// The two Red Alert sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Faction {
    Allied,
    Soviet,
}

impl Faction {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Allied => "Allied",
            Self::Soviet => "Soviet",
        }
    }
}

/// Playable RA countries. `unique_structure` is the country-specific building
/// from the original game (France: Grand Cannon, Germany: Chronosphere,
/// Russia: Iron Curtain, England: Gap Generator, Ukraine: Tesla Coil).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Country {
    England,
    Germany,
    France,
    Russia,
    Ukraine,
}

impl Country {
    pub const ALL: [Country; 5] = [
        Country::England,
        Country::Germany,
        Country::France,
        Country::Russia,
        Country::Ukraine,
    ];

    pub const fn faction(self) -> Faction {
        match self {
            Self::England | Self::Germany | Self::France => Faction::Allied,
            Self::Russia | Self::Ukraine => Faction::Soviet,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::England => "England",
            Self::Germany => "Germany",
            Self::France => "France",
            Self::Russia => "Russia",
            Self::Ukraine => "Ukraine",
        }
    }

    pub const fn unique_structure(self) -> ClassId {
        match self {
            Self::England => ClassId::GapGenerator,
            Self::Germany => ClassId::Chronosphere,
            Self::France => ClassId::Chronosphere,
            Self::Russia => ClassId::IronCurtain,
            Self::Ukraine => ClassId::TeslaCoil,
        }
    }
}

/// One queued production item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionItem {
    pub class_id: ClassId,
    pub progress: Fixed16_16,
    pub total_time: Fixed16_16,
    pub cost: i32,
}

/// House state shared by campaign and skirmish games.
#[derive(Debug, Clone)]
pub struct House {
    pub id: ClassId,
    pub name: String,
    pub faction: Faction,
    pub country: Option<Country>,
    pub credits: i32,
    /// Net power: production minus drain (negative means low power).
    pub power: i32,
    pub production_queue: VecDeque<ProductionItem>,
    pub tech_level: u8,
    pub credits_spent: i64,
}

impl House {
    pub fn new(id: ClassId, name: String, faction: Faction) -> Self {
        Self {
            id,
            name,
            faction,
            country: None,
            credits: 0,
            power: 0,
            production_queue: VecDeque::new(),
            tech_level: 98,
            credits_spent: 0,
        }
    }

    /// Attach a country; the faction is derived from it.
    pub fn with_country(mut self, country: Country) -> Self {
        self.country = Some(country);
        self.faction = country.faction();
        self
    }

    pub fn add_credits(&mut self, amount: i32) {
        self.credits = self.credits.saturating_add(amount);
    }

    pub fn can_afford(&self, cost: i32) -> bool {
        self.credits >= cost
    }

    /// Spend credits; returns false (and changes nothing) when short.
    pub fn spend(&mut self, cost: i32) -> bool {
        if cost <= 0 || !self.can_afford(cost) {
            return false;
        }
        self.credits -= cost;
        self.credits_spent += i64::from(cost);
        true
    }

    /// Condition BLACK: drain exceeds production.
    pub fn low_power(&self) -> bool {
        self.power < 0
    }

    /// Power ratio used to scale production speed (RA caps at 2x surplus).
    pub fn power_fraction(&self) -> f32 {
        if self.low_power() {
            0.5
        } else {
            1.0
        }
    }

    /// Queue an item, charging its cost up front.
    pub fn queue(&mut self, item: ProductionItem) -> bool {
        if !self.spend(item.cost) {
            return false;
        }
        self.production_queue.push_back(item);
        true
    }

    /// Cancel the front item and refund its cost.
    pub fn cancel_front(&mut self) -> Option<ProductionItem> {
        let item = self.production_queue.pop_front()?;
        self.credits += item.cost;
        Some(item)
    }

    /// Advance the front item; low power halves progress.
    pub fn update_production(&mut self, delta_time: Fixed16_16) -> Option<ProductionItem> {
        let factor = Fixed16_16::from_f32(self.power_fraction());
        let item = self.production_queue.front_mut()?;
        item.progress += delta_time * factor;
        if item.progress >= item.total_time {
            return self.production_queue.pop_front();
        }
        None
    }

    /// Production time for a unit/structure given a base seconds value.
    pub fn production_time(base_seconds: i32) -> Fixed16_16 {
        Fixed16_16::from_int(base_seconds.max(1))
    }

    /// Whether this house can build its country's unique structure.
    pub fn can_build_unique_structure(&self, class_id: ClassId) -> bool {
        self.country
            .is_some_and(|country| country.unique_structure() == class_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn allied() -> House {
        House::new(ClassId::HouseAllies, "Allies".to_string(), Faction::Allied)
    }

    #[test]
    fn countries_map_to_factions_and_unique_structures() {
        assert_eq!(Country::Germany.faction(), Faction::Allied);
        assert_eq!(Country::Ukraine.faction(), Faction::Soviet);
        assert_eq!(Country::England.unique_structure(), ClassId::GapGenerator);
        assert_eq!(Country::Russia.unique_structure(), ClassId::IronCurtain);
        assert_eq!(Country::Ukraine.unique_structure(), ClassId::TeslaCoil);
    }

    #[test]
    fn spending_and_refunds_are_exact() {
        let mut house = allied();
        house.add_credits(1000);
        assert!(house.can_afford(400));
        assert!(house.spend(400));
        assert_eq!(house.credits, 600);
        assert!(!house.spend(601), "cannot overspend");
        assert_eq!(house.credits, 600);

        let item = ProductionItem {
            class_id: ClassId::VehicleE1,
            progress: Fixed16_16::ZERO,
            total_time: Fixed16_16::from_int(10),
            cost: 200,
        };
        assert!(house.queue(item));
        assert_eq!(house.credits, 400);
        let refunded = house.cancel_front().unwrap();
        assert_eq!(refunded.cost, 200);
        assert_eq!(house.credits, 600);
    }

    #[test]
    fn low_power_halves_production_speed() {
        let mut house = allied();
        house.add_credits(500);
        house.power = -10;
        house.queue(ProductionItem {
            class_id: ClassId::VehicleE1,
            progress: Fixed16_16::ZERO,
            total_time: Fixed16_16::from_int(10),
            cost: 100,
        });
        let half = Fixed16_16::from_int(4);
        assert!(house.update_production(half).is_none());
        assert_eq!(house.production_queue[0].progress, Fixed16_16::from_int(2));
        assert!(house.low_power());
        assert_eq!(house.power_fraction(), 0.5);
    }

    #[test]
    fn country_gate_allows_only_the_unique_structure() {
        let mut house = allied().with_country(Country::England);
        assert!(house.can_build_unique_structure(ClassId::GapGenerator));
        assert!(!house.can_build_unique_structure(ClassId::IronCurtain));
        house.country = None;
        assert!(!house.can_build_unique_structure(ClassId::GapGenerator));
    }
}
