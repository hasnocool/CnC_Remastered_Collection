//! Tesla Coil: charge, power gate, range check and discharge.
//!
//! RA's Tesla Coil needs base power, charges between shots and fires a
//! lightning bolt at a single target. Damage is in fixed-point hit points so
//! it can be fed to the shared combat math.

use cnc_core::{ClassId, Fixed16_16, Vec2};

/// RA Tesla Coil damage per bolt.
pub const TESLA_DAMAGE: i32 = 200;
/// Seconds to recharge after firing.
pub const TESLA_RECHARGE_SECONDS: i32 = 3;
/// Range in map cells (ICON_PIXEL_W = 24 px).
pub const TESLA_RANGE_CELLS: i32 = 6;
/// Pixels per map cell.
pub const CELL_PX: f32 = 24.0;

/// One Tesla discharge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TeslaBolt {
    pub from: Vec2,
    pub to: Vec2,
    pub damage: Fixed16_16,
}

/// A placed Tesla Coil.
#[derive(Debug, Clone)]
pub struct TeslaCoil {
    pub class_id: ClassId,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    /// 0..1 charge toward the next bolt.
    pub charge: Fixed16_16,
    /// Remaining recharge lock-out after a shot.
    pub reload: Fixed16_16,
    pub range: Fixed16_16,
    /// Set false while the owning house is low on power.
    pub powered: bool,
}

impl TeslaCoil {
    pub fn new(position: Vec2) -> Self {
        Self {
            class_id: ClassId::TeslaCoil,
            position,
            health: Fixed16_16::from_int(400),
            max_health: Fixed16_16::from_int(400),
            charge: Fixed16_16::ZERO,
            reload: Fixed16_16::ZERO,
            range: Fixed16_16::from_f32(TESLA_RANGE_CELLS as f32 * CELL_PX),
            powered: true,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    pub fn is_ready(&self) -> bool {
        self.powered && self.is_alive() && self.reload <= Fixed16_16::ZERO
            && self.charge >= Fixed16_16::ONE
    }

    /// Advance charge/reload. Unpowered coils hold their current charge but do
    /// not build more and cannot fire.
    pub fn update(&mut self, delta_time: Fixed16_16) {
        if !self.is_alive() {
            return;
        }
        let mut remaining = delta_time;
        if self.reload > Fixed16_16::ZERO {
            if remaining < self.reload {
                self.reload -= remaining;
                return;
            }
            remaining -= self.reload;
            self.reload = Fixed16_16::ZERO;
            // The lock-out doubles as the recharge window: when it ends the
            // coil is ready again (if the base still has power).
            if self.powered {
                self.charge = Fixed16_16::ONE;
            }
            if remaining <= Fixed16_16::ZERO {
                return;
            }
        }
        if !self.powered {
            return;
        }
        let rate = remaining / Fixed16_16::from_int(TESLA_RECHARGE_SECONDS);
        self.charge = (self.charge + rate).min(Fixed16_16::ONE);
    }

    pub fn distance_to(&self, target: Vec2) -> Fixed16_16 {
        Fixed16_16::from_f32(self.position.distance(target).to_f32())
    }

    pub fn in_range(&self, target: Vec2) -> bool {
        self.distance_to(target) <= self.range
    }

    /// Fire at a target if ready, powered and in range.
    pub fn try_fire(&mut self, target: Vec2) -> Option<TeslaBolt> {
        if !self.is_ready() || !self.in_range(target) {
            return None;
        }
        self.charge = Fixed16_16::ZERO;
        self.reload = Fixed16_16::from_int(TESLA_RECHARGE_SECONDS);
        Some(TeslaBolt {
            from: self.position,
            to: target,
            damage: Fixed16_16::from_int(TESLA_DAMAGE),
        })
    }

    /// Damage the coil itself.
    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
        if self.health == Fixed16_16::ZERO {
            self.charge = Fixed16_16::ZERO;
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
    fn coil_charges_then_fires_once_and_reloads() {
        let mut coil = TeslaCoil::new(point(0.0, 0.0));
        assert!(!coil.is_ready());
        coil.update(Fixed16_16::from_int(3));
        assert!(coil.is_ready());

        let bolt = coil.try_fire(point(48.0, 0.0)).expect("target in range");
        assert_eq!(bolt.damage, Fixed16_16::from_int(TESLA_DAMAGE));
        assert!(!coil.is_ready(), "firing empties the charge");
        assert!(coil.try_fire(point(10.0, 0.0)).is_none());

        coil.update(Fixed16_16::from_int(3));
        assert!(coil.is_ready());
    }

    #[test]
    fn out_of_range_and_unpowered_coils_do_not_fire() {
        let mut coil = TeslaCoil::new(point(0.0, 0.0));
        coil.update(Fixed16_16::from_int(3));
        assert!(coil.try_fire(point(1000.0, 0.0)).is_none());

        coil.powered = false;
        assert!(!coil.is_ready());
        assert!(coil.try_fire(point(24.0, 0.0)).is_none());
    }

    #[test]
    fn destroying_the_coil_disables_it() {
        let mut coil = TeslaCoil::new(point(0.0, 0.0));
        coil.update(Fixed16_16::from_int(3));
        coil.take_damage(Fixed16_16::from_int(10_000));
        assert!(!coil.is_alive());
        assert!(!coil.is_ready());
        coil.update(Fixed16_16::from_int(3));
        assert!(!coil.is_ready());
    }
}
