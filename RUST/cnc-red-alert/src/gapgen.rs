//! Gap Generator: shroud a radius while consuming power.

use cnc_core::{ClassId, Fixed16_16, Vec2};

/// Radius in map cells.
pub const GAP_RADIUS_CELLS: f32 = 10.0;
/// Pixels per map cell.
pub const CELL_PX: f32 = 24.0;
/// Power drain while active.
pub const GAP_POWER_DRAIN: i32 = 100;

/// A Gap Generator structure.
#[derive(Debug, Clone)]
pub struct GapGenerator {
    pub class_id: ClassId,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    pub active: bool,
    pub powered: bool,
    pub radius: Fixed16_16,
}

impl GapGenerator {
    pub fn new(position: Vec2) -> Self {
        Self {
            class_id: ClassId::GapGenerator,
            position,
            health: Fixed16_16::from_int(300),
            max_health: Fixed16_16::from_int(300),
            active: false,
            powered: true,
            radius: Fixed16_16::from_f32(GAP_RADIUS_CELLS * CELL_PX),
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    /// Toggle the shroud; refuses to engage without power.
    pub fn toggle(&mut self) -> bool {
        if self.active {
            self.active = false;
        } else if self.powered && self.is_alive() {
            self.active = true;
        } else {
            return false;
        }
        self.active
    }

    /// True while the generator affects the map.
    pub fn shrouding(&self) -> bool {
        self.active && self.powered && self.is_alive()
    }

    /// Whether a point lies inside the shrouded circle.
    pub fn shrouds(&self, point: Vec2) -> bool {
        if !self.shrouding() {
            return false;
        }
        let distance = Fixed16_16::from_f32(self.position.distance(point).to_f32());
        distance <= self.radius
    }

    /// House power drain while shrouding.
    pub fn power_drain(&self) -> i32 {
        if self.shrouding() {
            GAP_POWER_DRAIN
        } else {
            0
        }
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
        if !self.is_alive() {
            self.active = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: f32, y: f32) -> Vec2 {
        Vec2::new(Fixed16_16::from_f32(x), Fixed16_16::from_f32(y))
    }

    #[test]
    fn gap_generator_shrouds_only_while_active_and_powered() {
        let mut gap = GapGenerator::new(point(0.0, 0.0));
        assert!(!gap.shrouds(point(10.0, 10.0)));
        assert!(gap.toggle());
        assert!(gap.shrouds(point(10.0, 10.0)));
        assert!(!gap.shrouds(point(1000.0, 0.0)));
        assert_eq!(gap.power_drain(), GAP_POWER_DRAIN);

        gap.powered = false;
        assert!(!gap.shrouding());
        assert!(!gap.shrouds(point(10.0, 10.0)));
        assert_eq!(gap.power_drain(), 0);
    }

    #[test]
    fn cannot_activate_without_power_and_dies_cleanly() {
        let mut gap = GapGenerator::new(point(0.0, 0.0));
        gap.powered = false;
        assert!(!gap.toggle());
        assert!(!gap.active);

        gap.powered = true;
        assert!(gap.toggle());
        gap.take_damage(Fixed16_16::from_int(10_000));
        assert!(!gap.is_alive());
        assert!(!gap.shrouding());
    }
}
