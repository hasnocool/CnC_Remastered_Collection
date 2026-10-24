//! Red Alert houses, factions and production.
//!
//! Red Alert uses the same 16.16 fixed-point economy as Tiberian Dawn but has
//! two factions and five countries with a unique structure each. This module
//! models the house-level state that the scenario layer needs: treasury,
//! power balance, tech level and per-factory production queues with deterministic
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
            Self::France => ClassId::GrandCannon,
            Self::Russia => ClassId::IronCurtain,
            Self::Ukraine => ClassId::TeslaCoil,
        }
    }
}

/// Factory type for per-factory production queues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactoryType {
    Barracks,
    WarFactory,
    Shipyard,
    Helipad,
}

/// One queued production item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionItem {
    pub class_id: ClassId,
    pub progress: Fixed16_16,
    pub total_time: Fixed16_16,
    pub cost: i32,
}

/// Error when queuing production.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueueError {
    /// Insufficient credits.
    InsufficientFunds,
    /// Prerequisite structure not owned.
    PrerequisiteMissing(String),
    /// Factory type not available (not built).
    FactoryNotAvailable(FactoryType),
}

/// Silo capacity per silo (RA original: 2000 credits per silo).
pub const SILO_CAPACITY: i32 = 2000;

#[derive(Debug, Clone)]
pub struct House {
    pub id: ClassId,
    pub name: String,
    pub faction: Faction,
    pub country: Option<Country>,
    pub credits: i32,
    /// Net power: production minus drain (negative means low power).
    pub power: i32,
    /// Per-factory production queues.
    barracks_queue: VecDeque<ProductionItem>,
    war_factory_queue: VecDeque<ProductionItem>,
    shipyard_queue: VecDeque<ProductionItem>,
    helipad_queue: VecDeque<ProductionItem>,
    pub tech_level: u8,
    pub credits_spent: i64,
    /// Number of silos owned (each adds SILO_CAPACITY to credit storage).
    silo_count: usize,
    /// Alliance bitfield: bit N set means this house is allied with house N.
    /// Mirror of original HOUSE.CPP `Allies` field.
    pub allies: u32,
    /// Whether AI is enabled for this house.
    pub ai_enabled: bool,
    /// AI IQ level — mirrors HOUSE.CPP `IQ` (HOUSE.H:869).
    /// Controls what behaviors the AI understands: harvest, repair, production, etc.
    pub iq: u8,
    /// Multiplayer spawn slot (start-location index); `None` for campaign houses.
    pub spawn_slot: Option<u8>,
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
            barracks_queue: VecDeque::new(),
            war_factory_queue: VecDeque::new(),
            shipyard_queue: VecDeque::new(),
            helipad_queue: VecDeque::new(),
            tech_level: 98,
            credits_spent: 0,
            silo_count: 0,
            allies: 0,
            ai_enabled: true,
            iq: 0,
            spawn_slot: None,
        }
    }

    /// Attach a country; the faction is derived from it.
    pub fn with_country(mut self, country: Country) -> Self {
        self.country = Some(country);
        self.faction = country.faction();
        self
    }

    /// Assign a multiplayer spawn slot (start location index).
    pub fn with_spawn_slot(mut self, slot: u8) -> Self {
        self.spawn_slot = Some(slot);
        self
    }

    /// Current credit count (capped by silo capacity).
    pub fn credits(&self) -> i32 {
        self.credits
    }

    /// Maximum credits this house can hold (based on silo count).
    pub fn max_credits(&self) -> i32 {
        if self.silo_count == 0 {
            i32::MAX // No silo = unlimited (or very high limit in original)
        } else {
            self.silo_count as i32 * SILO_CAPACITY
        }
    }

    /// Add a silo, increasing credit capacity.
    pub fn add_silo(&mut self) {
        self.silo_count += 1;
    }

    /// Number of silos owned.
    pub fn silo_count(&self) -> usize {
        self.silo_count
    }

    pub fn add_credits(&mut self, amount: i32) {
        let max = self.max_credits();
        let new_credits = self.credits.saturating_add(amount);
        self.credits = new_credits.min(max);
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

    /// Get a mutable reference to a factory queue.
    fn queue_for(&mut self, factory: FactoryType) -> &mut VecDeque<ProductionItem> {
        match factory {
            FactoryType::Barracks => &mut self.barracks_queue,
            FactoryType::WarFactory => &mut self.war_factory_queue,
            FactoryType::Shipyard => &mut self.shipyard_queue,
            FactoryType::Helipad => &mut self.helipad_queue,
        }
    }

    /// Get an immutable reference to a factory queue.
    fn queue_for_ref(&self, factory: FactoryType) -> &VecDeque<ProductionItem> {
        match factory {
            FactoryType::Barracks => &self.barracks_queue,
            FactoryType::WarFactory => &self.war_factory_queue,
            FactoryType::Shipyard => &self.shipyard_queue,
            FactoryType::Helipad => &self.helipad_queue,
        }
    }

    /// Queue an item in a specific factory, charging its cost up front.
    pub fn queue_in(
        &mut self,
        factory: FactoryType,
        item: ProductionItem,
    ) -> Result<(), QueueError> {
        if !self.spend(item.cost) {
            return Err(QueueError::InsufficientFunds);
        }
        self.queue_for(factory).push_back(item);
        Ok(())
    }

    /// First queued item across all factory queues (barracks, then war
    /// factory, then shipyard, then helipad), for HUD display.
    pub fn first_queued(&self) -> Option<&ProductionItem> {
        [
            FactoryType::Barracks,
            FactoryType::WarFactory,
            FactoryType::Shipyard,
            FactoryType::Helipad,
        ]
        .iter()
        .find_map(|factory| self.queue_for_ref(*factory).front())
    }

    /// Total number of items queued across all factory queues.
    pub fn total_queued(&self) -> usize {
        [
            FactoryType::Barracks,
            FactoryType::WarFactory,
            FactoryType::Shipyard,
            FactoryType::Helipad,
        ]
        .iter()
        .map(|factory| self.queue_for_ref(*factory).len())
        .sum()
    }

    /// Last queued item across all factory queues (reverse factory order of
    /// [`first_queued`]), for tests asserting queue tails.
    pub fn last_queued(&self) -> Option<&ProductionItem> {
        [
            FactoryType::Helipad,
            FactoryType::Shipyard,
            FactoryType::WarFactory,
            FactoryType::Barracks,
        ]
        .iter()
        .find_map(|factory| self.queue_for_ref(*factory).back())
    }

    /// Queue an item in barracks (infantry).
    pub fn queue_barracks(&mut self, item: ProductionItem) -> Result<(), QueueError> {
        self.queue_in(FactoryType::Barracks, item)
    }

    /// Queue an item in war factory (vehicles).
    pub fn queue_war_factory(&mut self, item: ProductionItem) -> Result<(), QueueError> {
        self.queue_in(FactoryType::WarFactory, item)
    }

    /// Queue an item in shipyard (naval).
    pub fn queue_shipyard(&mut self, item: ProductionItem) -> Result<(), QueueError> {
        self.queue_in(FactoryType::Shipyard, item)
    }

    /// Queue an item in helipad (aircraft).
    pub fn queue_helipad(&mut self, item: ProductionItem) -> Result<(), QueueError> {
        self.queue_in(FactoryType::Helipad, item)
    }

    /// Try to queue with prerequisite checking (requires owned structures list and rules).
    /// The `check_prereq` closure returns true if the prerequisite is met.
    pub fn try_queue_in<F>(
        &mut self,
        factory: FactoryType,
        item: ProductionItem,
        owned_structures: &[ClassId],
        rules: &cnc_core::RulesDatabase,
        check_prereq: F,
    ) -> Result<(), QueueError>
    where
        F: Fn(&str, &[ClassId]) -> bool,
    {
        // Check prerequisites using the provided closure
        if let Some(rule_name) = crate::game::rules_type_name(item.class_id) {
            if let Some(rule) = rules.type_rule(rule_name) {
                for prereq in &rule.prerequisite {
                    if !check_prereq(prereq, owned_structures) {
                        return Err(QueueError::PrerequisiteMissing(format!(
                            "Missing prerequisite: {}",
                            prereq
                        )));
                    }
                }
            }
        }
        self.queue_in(factory, item)
    }

    /// Try to queue in barracks with prerequisite checking.
    pub fn try_queue_barracks(
        &mut self,
        item: ProductionItem,
        owned_structures: &[ClassId],
        rules: &cnc_core::RulesDatabase,
    ) -> Result<(), QueueError> {
        self.try_queue_in(
            FactoryType::Barracks,
            item,
            owned_structures,
            rules,
            crate::game::ra_prerequisite_met,
        )
    }

    /// Try to queue in war factory with prerequisite checking.
    pub fn try_queue_war_factory(
        &mut self,
        item: ProductionItem,
        owned_structures: &[ClassId],
        rules: &cnc_core::RulesDatabase,
    ) -> Result<(), QueueError> {
        self.try_queue_in(
            FactoryType::WarFactory,
            item,
            owned_structures,
            rules,
            crate::game::ra_prerequisite_met,
        )
    }

    /// Try to queue in shipyard with prerequisite checking.
    pub fn try_queue_shipyard(
        &mut self,
        item: ProductionItem,
        owned_structures: &[ClassId],
        rules: &cnc_core::RulesDatabase,
    ) -> Result<(), QueueError> {
        self.try_queue_in(
            FactoryType::Shipyard,
            item,
            owned_structures,
            rules,
            crate::game::ra_prerequisite_met,
        )
    }

    /// Try to queue in helipad with prerequisite checking.
    pub fn try_queue_helipad(
        &mut self,
        item: ProductionItem,
        owned_structures: &[ClassId],
        rules: &cnc_core::RulesDatabase,
    ) -> Result<(), QueueError> {
        self.try_queue_in(
            FactoryType::Helipad,
            item,
            owned_structures,
            rules,
            crate::game::ra_prerequisite_met,
        )
    }

    /// Cancel the front item in a specific factory and refund its cost.
    pub fn cancel_front_in(&mut self, factory: FactoryType) -> Option<ProductionItem> {
        let item = self.queue_for(factory).pop_front()?;
        self.credits += item.cost;
        Some(item)
    }

    /// Cancel the front barracks item.
    pub fn cancel_front_barracks(&mut self) -> Option<ProductionItem> {
        self.cancel_front_in(FactoryType::Barracks)
    }

    /// Cancel the front war factory item.
    pub fn cancel_front_war_factory(&mut self) -> Option<ProductionItem> {
        self.cancel_front_in(FactoryType::WarFactory)
    }

    /// Cancel the front shipyard item.
    pub fn cancel_front_shipyard(&mut self) -> Option<ProductionItem> {
        self.cancel_front_in(FactoryType::Shipyard)
    }

    /// Cancel the front helipad item.
    pub fn cancel_front_helipad(&mut self) -> Option<ProductionItem> {
        self.cancel_front_in(FactoryType::Helipad)
    }

    /// Advance the front item in a specific factory; low power halves progress.
    pub fn update_factory(
        &mut self,
        factory: FactoryType,
        delta_time: Fixed16_16,
    ) -> Option<ProductionItem> {
        let factor = Fixed16_16::from_f32(self.power_fraction());
        let queue = self.queue_for(factory);
        let item = queue.front_mut()?;
        item.progress += delta_time * factor;
        if item.progress >= item.total_time {
            return queue.pop_front();
        }
        None
    }

    /// Update barracks production.
    pub fn update_barracks(&mut self, delta_time: Fixed16_16) -> Option<ProductionItem> {
        self.update_factory(FactoryType::Barracks, delta_time)
    }

    /// Update war factory production.
    pub fn update_war_factory(&mut self, delta_time: Fixed16_16) -> Option<ProductionItem> {
        self.update_factory(FactoryType::WarFactory, delta_time)
    }

    /// Update shipyard production.
    pub fn update_shipyard(&mut self, delta_time: Fixed16_16) -> Option<ProductionItem> {
        self.update_factory(FactoryType::Shipyard, delta_time)
    }

    /// Update helipad production.
    pub fn update_helipad(&mut self, delta_time: Fixed16_16) -> Option<ProductionItem> {
        self.update_factory(FactoryType::Helipad, delta_time)
    }

    /// Get immutable reference to barracks queue.
    pub fn barracks_queue(&self) -> &VecDeque<ProductionItem> {
        self.queue_for_ref(FactoryType::Barracks)
    }

    /// Get immutable reference to war factory queue.
    pub fn war_factory_queue(&self) -> &VecDeque<ProductionItem> {
        self.queue_for_ref(FactoryType::WarFactory)
    }

    /// Get immutable reference to shipyard queue.
    pub fn shipyard_queue(&self) -> &VecDeque<ProductionItem> {
        self.queue_for_ref(FactoryType::Shipyard)
    }

    /// Get immutable reference to helipad queue.
    pub fn helipad_queue(&self) -> &VecDeque<ProductionItem> {
        self.queue_for_ref(FactoryType::Helipad)
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

    /// Check if this house is allied with another house.
    pub fn is_ally(&self, other: usize) -> bool {
        if other == 0 {
            (self.allies & 1) != 0
        } else {
            (self.allies & (1u32 << other)) != 0
        }
    }

    /// Make another house an ally (sets the bit in allies bitfield).
    pub fn make_ally(&mut self, other: usize) {
        if other == 0 {
            self.allies |= 1;
        } else {
            self.allies |= 1u32 << other;
        }
    }

    /// Make another house an enemy (clears the bit in allies bitfield).
    pub fn make_enemy(&mut self, other: usize) {
        if other == 0 {
            self.allies &= !1;
        } else {
            self.allies &= !(1u32 << other);
        }
    }

    /// Enable or disable AI for this house.
    pub fn set_ai_enabled(&mut self, enabled: bool) {
        self.ai_enabled = enabled;
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
        assert_eq!(Country::France.unique_structure(), ClassId::GrandCannon);
        assert_eq!(Country::Germany.unique_structure(), ClassId::Chronosphere);
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
        // Use war factory queue for legacy test
        assert!(house.queue_war_factory(item).is_ok());
        assert_eq!(house.credits, 400);
        let refunded = house.cancel_front_war_factory().unwrap();
        assert_eq!(refunded.cost, 200);
        assert_eq!(house.credits, 600);
    }

    #[test]
    fn low_power_halves_production_speed() {
        let mut house = allied();
        house.add_credits(500);
        house.power = -10;
        // Use war factory queue for legacy test
        house
            .queue_war_factory(ProductionItem {
                class_id: ClassId::VehicleE1,
                progress: Fixed16_16::ZERO,
                total_time: Fixed16_16::from_int(10),
                cost: 100,
            })
            .expect("queue");
        let half = Fixed16_16::from_int(4);
        assert!(house.update_war_factory(half).is_none());
        assert_eq!(
            house.war_factory_queue()[0].progress,
            Fixed16_16::from_int(2)
        );
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

    /// Two factory queues (barracks + war factory) build in parallel.
    /// Infantry from barracks and vehicle from war factory should progress simultaneously.
    #[test]
    fn parallel_factory_queues_build() {
        let mut house = allied();
        house.add_credits(10000);

        // Queue infantry (barracks) and vehicle (war factory) simultaneously
        house
            .queue_barracks(ProductionItem {
                class_id: ClassId::InfantryE1,
                progress: Fixed16_16::ZERO,
                total_time: Fixed16_16::from_int(10),
                cost: 120,
            })
            .expect("barracks queue");
        house
            .queue_war_factory(ProductionItem {
                class_id: ClassId::VehicleE1,
                progress: Fixed16_16::ZERO,
                total_time: Fixed16_16::from_int(20),
                cost: 600,
            })
            .expect("war factory queue");

        // Both should be in their respective queues
        assert_eq!(house.barracks_queue().len(), 1);
        assert_eq!(house.war_factory_queue().len(), 1);

        // Advance time - both should progress
        let delta = Fixed16_16::from_int(5);
        let barracks_done = house.update_barracks(delta);
        let war_factory_done = house.update_war_factory(delta);

        assert!(barracks_done.is_none(), "barracks not done yet at 5s");
        assert!(war_factory_done.is_none(), "war factory not done yet at 5s");

        // Check both progressed
        assert_eq!(house.barracks_queue()[0].progress, Fixed16_16::from_int(5));
        assert_eq!(
            house.war_factory_queue()[0].progress,
            Fixed16_16::from_int(5)
        );

        // Complete both
        let delta2 = Fixed16_16::from_int(5);
        let barracks_done = house.update_barracks(delta2);
        let war_factory_done = house.update_war_factory(delta2);

        assert!(barracks_done.is_some(), "barracks done at 10s");
        assert!(war_factory_done.is_none(), "war factory needs 20s total");

        let delta3 = Fixed16_16::from_int(10);
        let war_factory_done = house.update_war_factory(delta3);
        assert!(war_factory_done.is_some(), "war factory done at 20s");
    }

    /// Production blocked when prerequisite structure not owned.
    #[test]
    fn prereq_unmet_production_blocked_with_reason() {
        let mut house = allied();
        house.add_credits(10000);

        // Create a rules database with a Tech Center prerequisite for Heavy Tank
        let ini = cnc_core::IniFile::parse("[MTNK]\nPrerequisite=TECH\n").expect("ini");
        let rules = cnc_core::RulesDatabase::from_ini(&ini);

        // Try to queue a unit requiring a Tech Center (not owned)
        // This should fail with a reason
        let result = house.try_queue_war_factory(
            ProductionItem {
                class_id: ClassId::VehicleE2, // Heavy Tank - requires Tech Center
                progress: Fixed16_16::ZERO,
                total_time: Fixed16_16::from_int(30),
                cost: 1200,
            },
            &[],
            &rules,
        );

        assert!(result.is_err(), "should fail without Tech Center");
        match result {
            Err(QueueError::PrerequisiteMissing(msg)) => {
                assert!(
                    msg.contains("Tech Center")
                        || msg.contains("prerequisite")
                        || msg.contains("TECH")
                );
            }
            _ => panic!("expected PrerequisiteMissing error"),
        }
    }

    /// Silo capacity: when credits exceed silo capacity, overflow is lost.
    #[test]
    fn silo_overflow_loses_credits_per_pinned_rule() {
        let mut house = allied();
        // Add a silo (capacity 2000 per silo in RA)
        house.add_silo();
        house.add_silo(); // 2 silos = 4000 capacity

        house.add_credits(3000);
        assert_eq!(house.credits(), 3000);

        // Add 2000 more - should hit capacity at 4000, lose 1000
        house.add_credits(2000);
        assert_eq!(house.credits(), 4000, "credits capped at silo capacity");

        // Adding more should lose the overflow
        house.add_credits(5000);
        assert_eq!(house.credits(), 4000, "overflow lost");
    }
}
