//! Red Alert naval units: surface movement, submerging and targeting rules.

use cnc_core::{ClassId, Fixed16_16, Vec2};

/// Base surface speed in pixels per second.
pub const SHIP_SPEED_PX_PER_SEC: f32 = 60.0;
/// Submarine speed while surfaced.
pub const SUB_SPEED_PX_PER_SEC: f32 = 48.0;

/// A surface ship (gunboat, destroyer, cruiser, transport, carrier).
#[derive(Debug, Clone)]
pub struct Ship {
    pub class_id: ClassId,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    pub on_water: bool,
    pub speed: Fixed16_16,
    pub target: Option<Vec2>,
}

impl Ship {
    pub fn new(class_id: ClassId, position: Vec2, health: Fixed16_16) -> Self {
        Self {
            class_id,
            position,
            health,
            max_health: health,
            on_water: true,
            speed: Fixed16_16::from_f32(SHIP_SPEED_PX_PER_SEC),
            target: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    /// Issue a move order; ships refuse to leave the water.
    pub fn move_to(&mut self, target: Vec2) -> bool {
        if !self.is_alive() || !self.on_water {
            return false;
        }
        self.target = Some(target);
        true
    }

    /// Advance toward the current order; returns true when the order completes.
    pub fn update(&mut self, delta_time: Fixed16_16) -> bool {
        let Some(target) = self.target else {
            return false;
        };
        if !self.is_alive() || !self.on_water {
            return false;
        }
        let dx = target.x.to_f32() - self.position.x.to_f32();
        let dy = target.y.to_f32() - self.position.y.to_f32();
        let distance = (dx * dx + dy * dy).sqrt();
        let step = self.speed.to_f32() * delta_time.to_f32();
        if distance <= step || distance <= f32::EPSILON {
            self.position = target;
            self.target = None;
            return true;
        }
        self.position = Vec2::new(
            Fixed16_16::from_f32(self.position.x.to_f32() + dx / distance * step),
            Fixed16_16::from_f32(self.position.y.to_f32() + dy / distance * step),
        );
        false
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }
}

/// A submarine: can submerge, becomes untargetable unless detected, and cannot
/// fire while submerged.
#[derive(Debug, Clone)]
pub struct Submarine {
    pub base: Ship,
    pub submerged: bool,
    pub detected: bool,
    pub torpedoes: u32,
}

impl Submarine {
    pub fn new(position: Vec2) -> Self {
        let mut base = Ship::new(ClassId::ShipE4, position, Fixed16_16::from_int(300));
        base.speed = Fixed16_16::from_f32(SUB_SPEED_PX_PER_SEC);
        Self {
            base,
            submerged: false,
            detected: false,
            torpedoes: 2,
        }
    }

    /// Surface or dive. A detected submarine may still dive and break lock.
    pub fn set_submerged(&mut self, submerged: bool) {
        if !self.base.is_alive() {
            return;
        }
        self.submerged = submerged;
        if submerged {
            self.detected = false;
        }
    }

    /// Can an enemy weapon select this submarine?
    pub fn targetable(&self) -> bool {
        self.base.is_alive() && (!self.submerged || self.detected)
    }

    /// Torpedo launch; requires surfaced or detected state and ammo.
    pub fn fire_torpedo(&mut self, target: Vec2) -> Option<Vec2> {
        if self.submerged && !self.detected {
            return None;
        }
        if self.torpedoes == 0 || !self.base.is_alive() {
            return None;
        }
        self.torpedoes -= 1;
        Some(target)
    }

    /// Reload torpedoes at a naval yard (abstracted as a count).
    pub fn rearm(&mut self, torpedoes: u32) {
        self.torpedoes = torpedoes.min(2);
    }

    /// Sonar contact from another unit.
    pub fn mark_detected(&mut self) {
        if self.submerged {
            self.detected = true;
        }
    }

    pub fn update(&mut self, delta_time: Fixed16_16) -> bool {
        self.base.update(delta_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: f32, y: f32) -> Vec2 {
        Vec2::new(Fixed16_16::from_f32(x), Fixed16_16::from_f32(y))
    }

    #[test]
    fn ship_moves_toward_its_order_and_stops() {
        let mut ship = Ship::new(ClassId::ShipE1, point(0.0, 0.0), Fixed16_16::from_int(100));
        assert!(ship.move_to(point(120.0, 0.0)));
        assert!(!ship.update(Fixed16_16::ONE));
        assert!(ship.position.x.to_f32() > 0.0);
        assert!(ship.update(Fixed16_16::from_int(10)));
        assert_eq!(ship.position, point(120.0, 0.0));
        assert!(ship.target.is_none());
    }

    #[test]
    fn submerged_submarine_cannot_be_targeted_or_fire() {
        let mut sub = Submarine::new(point(0.0, 0.0));
        assert!(sub.targetable());
        assert!(sub.fire_torpedo(point(10.0, 0.0)).is_some());
        sub.set_submerged(true);
        assert!(!sub.targetable());
        assert!(sub.fire_torpedo(point(10.0, 0.0)).is_none());
        sub.mark_detected();
        assert!(sub.targetable());
        assert!(sub.fire_torpedo(point(10.0, 0.0)).is_some());
        assert_eq!(sub.torpedoes, 0);
        assert!(sub.fire_torpedo(point(10.0, 0.0)).is_none());
        sub.rearm(5);
        assert_eq!(sub.torpedoes, 2);
    }

    #[test]
    fn dead_or_landlocked_ships_do_not_move() {
        let mut ship = Ship::new(ClassId::ShipE2, point(0.0, 0.0), Fixed16_16::from_int(50));
        ship.on_water = false;
        assert!(!ship.move_to(point(10.0, 0.0)));
        ship.on_water = true;
        ship.take_damage(Fixed16_16::from_int(100));
        assert!(!ship.is_alive());
        assert!(!ship.move_to(point(10.0, 0.0)));
    }
}
