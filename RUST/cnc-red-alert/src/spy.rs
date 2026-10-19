//! Red Alert Spy and Thief infiltration.
//!
//! A spy enters an enemy structure and produces a one-shot effect depending on
//! the target: refineries leak credits, radars reveal the map, power plants can
//! be sabotaged and war factories leak production data. The thief simply steals
//! a share of the target's treasury.

use cnc_core::{ClassId, Fixed16_16, Vec2};

/// Fraction of a refinery's credits a spy steals (percent).
pub const SPY_REFINERY_PERCENT: i32 = 50;
/// Maximum credits a thief can carry away.
pub const THIEF_MAX_STEAL: i32 = 1000;

/// A structure a spy can infiltrate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfiltrationTarget {
    Refinery,
    Radar,
    PowerPlant,
    WarFactory,
}

/// Result of a successful infiltration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InfiltrationEffect {
    /// Credits removed from the target house.
    StoleCredits(i32),
    /// The target house's map coverage is revealed.
    RevealedMap,
    /// The target house loses power for a while.
    SabotagedPower,
    /// The target house's current production is reset.
    SabotagedProduction,
}

/// A spy infantry unit.
#[derive(Debug, Clone)]
pub struct Spy {
    pub class_id: ClassId,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    pub disguised_as: Option<ClassId>,
    /// Effects produced so far (telemetry / HUD).
    pub infiltrations: u32,
}

impl Spy {
    pub fn new(position: Vec2) -> Self {
        Self {
            class_id: ClassId::InfantryE8,
            position,
            health: Fixed16_16::from_int(40),
            max_health: Fixed16_16::from_int(40),
            disguised_as: None,
            infiltrations: 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    /// Disguise as an enemy unit class; only possible while alive.
    pub fn disguise(&mut self, as_class: ClassId) -> bool {
        if !self.is_alive() {
            return false;
        }
        self.disguised_as = Some(as_class);
        true
    }

    /// Infiltrate a structure. `target_credits` is read for refineries.
    pub fn infiltrate(
        &mut self,
        target: InfiltrationTarget,
        target_credits: i32,
    ) -> Option<InfiltrationEffect> {
        if !self.is_alive() {
            return None;
        }
        self.infiltrations += 1;
        Some(match target {
            InfiltrationTarget::Refinery => {
                let stolen = (target_credits.max(0) * SPY_REFINERY_PERCENT) / 100;
                InfiltrationEffect::StoleCredits(stolen)
            }
            InfiltrationTarget::Radar => InfiltrationEffect::RevealedMap,
            InfiltrationTarget::PowerPlant => InfiltrationEffect::SabotagedPower,
            InfiltrationTarget::WarFactory => InfiltrationEffect::SabotagedProduction,
        })
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }
}

/// A thief that steals a capped share of credits.
#[derive(Debug, Clone)]
pub struct Thief {
    pub class_id: ClassId,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    pub stolen_credits: i32,
    /// Remaining carrying capacity.
    pub carrying: i32,
}

impl Thief {
    pub fn new(position: Vec2) -> Self {
        Self {
            class_id: ClassId::InfantryE9,
            position,
            health: Fixed16_16::from_int(40),
            max_health: Fixed16_16::from_int(40),
            stolen_credits: 0,
            carrying: 0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    /// Steal up to [`THIEF_MAX_STEAL`] credits from `available`; returns the
    /// amount actually taken.
    pub fn steal(&mut self, available: i32) -> i32 {
        let room = (THIEF_MAX_STEAL - self.carrying).max(0);
        let taken = available.max(0).min(room).min(THIEF_MAX_STEAL);
        self.carrying += taken;
        self.stolen_credits += taken;
        taken
    }

    /// Bank carried credits; returns them.
    pub fn deposit(&mut self) -> i32 {
        let amount = self.carrying;
        self.carrying = 0;
        amount
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: f32, y: f32) -> Vec2 {
        Vec2::new(Fixed16_16::from_f32(x), Fixed16_16::from_f32(y))
    }

    #[test]
    fn spy_effects_depend_on_the_target() {
        let mut spy = Spy::new(point(0.0, 0.0));
        assert_eq!(
            spy.infiltrate(InfiltrationTarget::Refinery, 1000),
            Some(InfiltrationEffect::StoleCredits(500))
        );
        assert_eq!(
            spy.infiltrate(InfiltrationTarget::Radar, 0),
            Some(InfiltrationEffect::RevealedMap)
        );
        assert_eq!(
            spy.infiltrate(InfiltrationTarget::PowerPlant, 0),
            Some(InfiltrationEffect::SabotagedPower)
        );
        assert_eq!(
            spy.infiltrate(InfiltrationTarget::WarFactory, 0),
            Some(InfiltrationEffect::SabotagedProduction)
        );
        assert_eq!(spy.infiltrations, 4);
    }

    #[test]
    fn disguise_requires_a_living_spy() {
        let mut spy = Spy::new(point(0.0, 0.0));
        assert!(spy.disguise(ClassId::InfantryE1));
        assert_eq!(spy.disguised_as, Some(ClassId::InfantryE1));
        spy.take_damage(Fixed16_16::from_int(100));
        assert!(!spy.disguise(ClassId::InfantryE2));
        assert_eq!(
            spy.infiltrate(InfiltrationTarget::Radar, 0),
            None
        );
    }

    #[test]
    fn thief_caps_total_carryings() {
        let mut thief = Thief::new(point(0.0, 0.0));
        assert_eq!(thief.steal(100), 100);
        assert_eq!(thief.steal(5000), 900);
        assert_eq!(thief.steal(5000), 0);
        assert_eq!(thief.stolen_credits, THIEF_MAX_STEAL);
        assert_eq!(thief.deposit(), THIEF_MAX_STEAL);
        assert_eq!(thief.carrying, 0);
        assert_eq!(thief.steal(200), 200);
    }
}
