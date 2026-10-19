//! Red Alert superweapons: Chronosphere, Iron Curtain and GPS.
//!
//! Each weapon charges once over ten minutes, consumes its charge on use and
//! then starts over. Effects are deliberately data-oriented (positions and
//! timers) so they are deterministic in lockstep play.

use cnc_core::{ClassId, Fixed16_16, Vec2};

/// Seconds to charge any RA superweapon.
pub const CHARGE_SECONDS: i32 = 600;
/// Iron Curtain invulnerability window in seconds.
pub const IRON_CURTAIN_SECONDS: i32 = 45;

/// Failure modes shared by all superweapons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SuperweaponError {
    #[error("superweapon is not charged")]
    NotReady,
    #[error("superweapon structure has been destroyed")]
    Destroyed,
}

/// Shared charge/ready state.
#[derive(Debug, Clone)]
pub struct Superweapon {
    pub class_id: ClassId,
    pub position: Vec2,
    pub owner: u8,
    pub ready: bool,
    pub charge: Fixed16_16,
    /// Seconds left on a temporary effect (used by the Iron Curtain/GPS).
    pub cooldown: Fixed16_16,
}

impl Superweapon {
    pub fn new(class_id: ClassId, position: Vec2, owner: u8) -> Self {
        Self {
            class_id,
            position,
            owner,
            ready: false,
            charge: Fixed16_16::ZERO,
            cooldown: Fixed16_16::ZERO,
        }
    }

    /// Advance the charge; no-op once ready.
    pub fn update(&mut self, delta_time: Fixed16_16) {
        if self.ready {
            return;
        }
        self.charge =
            (self.charge + delta_time / Fixed16_16::from_int(CHARGE_SECONDS)).min(Fixed16_16::ONE);
        if self.charge >= Fixed16_16::ONE {
            self.ready = true;
        }
    }

    pub fn charge_fraction(&self) -> f32 {
        self.charge.to_f32()
    }

    /// Consume the charge; callers apply the effect only when this succeeds.
    pub fn consume(&mut self) -> Result<(), SuperweaponError> {
        if !self.ready {
            return Err(SuperweaponError::NotReady);
        }
        self.ready = false;
        self.charge = Fixed16_16::ZERO;
        Ok(())
    }
}

/// Teleports selected units to a target point.
#[derive(Debug, Clone)]
pub struct Chronosphere {
    pub base: Superweapon,
    /// Diagnostic range shown to the player (RA teleports map-wide).
    pub range: Fixed16_16,
}

impl Chronosphere {
    pub fn new(position: Vec2, owner: u8) -> Self {
        Self {
            base: Superweapon::new(ClassId::Chronosphere, position, owner),
            range: Fixed16_16::from_int(32),
        }
    }

    pub fn update(&mut self, delta_time: Fixed16_16) {
        self.base.update(delta_time);
    }

    /// Move every supplied unit to `target`, spread over a small grid.
    pub fn chronoshift(
        &mut self,
        units: &mut [Vec2],
        target: Vec2,
    ) -> Result<usize, SuperweaponError> {
        self.base.consume()?;
        for (index, unit) in units.iter_mut().enumerate() {
            let column = (index % 3) as f32;
            let row = (index / 3) as f32;
            *unit = Vec2::new(
                Fixed16_16::from_f32(target.x.to_f32() + column * 12.0),
                Fixed16_16::from_f32(target.y.to_f32() + row * 12.0),
            );
        }
        Ok(units.len())
    }
}

/// An entity that can be hardened by the Iron Curtain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArmoredUnit {
    pub position: Vec2,
    pub health: Fixed16_16,
    /// Seconds of remaining invulnerability.
    pub invulnerable: Fixed16_16,
}

impl ArmoredUnit {
    pub fn new(position: Vec2, health: Fixed16_16) -> Self {
        Self {
            position,
            health,
            invulnerable: Fixed16_16::ZERO,
        }
    }

    /// Damage application that respects Iron Curtain protection.
    pub fn take_damage(&mut self, amount: Fixed16_16) {
        if self.invulnerable > Fixed16_16::ZERO {
            return;
        }
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }

    /// Tick down the protection window.
    pub fn update(&mut self, delta_time: Fixed16_16) {
        self.invulnerable = (self.invulnerable - delta_time).max(Fixed16_16::ZERO);
    }
}

/// Grants temporary invulnerability inside a radius.
#[derive(Debug, Clone)]
pub struct IronCurtain {
    pub base: Superweapon,
    pub duration: Fixed16_16,
}

impl IronCurtain {
    pub fn new(position: Vec2, owner: u8) -> Self {
        Self {
            base: Superweapon::new(ClassId::IronCurtain, position, owner),
            duration: Fixed16_16::from_int(IRON_CURTAIN_SECONDS),
        }
    }

    pub fn update(&mut self, delta_time: Fixed16_16) {
        self.base.update(delta_time);
    }

    /// Harden friendly units within `radius` of the structure.
    pub fn harden(
        &mut self,
        units: &mut [ArmoredUnit],
        radius: Fixed16_16,
    ) -> Result<usize, SuperweaponError> {
        self.base.consume()?;
        let mut hardened = 0;
        for unit in units.iter_mut() {
            let distance =
                Fixed16_16::from_f32(self.base.position.distance(unit.position).to_f32());
            if distance <= radius {
                unit.invulnerable = self.duration;
                hardened += 1;
            }
        }
        Ok(hardened)
    }
}

/// Reveals enemy positions for a window.
#[derive(Debug, Clone)]
pub struct GPS {
    pub base: Superweapon,
    pub reveal_radius: Fixed16_16,
    pub revealed: Vec<Vec2>,
}

impl GPS {
    pub fn new(position: Vec2, owner: u8) -> Self {
        Self {
            base: Superweapon::new(ClassId::Radar, position, owner),
            reveal_radius: Fixed16_16::from_int(64),
            revealed: Vec::new(),
        }
    }

    pub fn update(&mut self, delta_time: Fixed16_16) {
        self.base.update(delta_time);
    }

    /// Full-map scan: every enemy position is revealed.
    pub fn scan(&mut self, enemies: &[Vec2]) -> Result<Vec<Vec2>, SuperweaponError> {
        self.base.consume()?;
        self.revealed = enemies.to_vec();
        Ok(self.revealed.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: f32, y: f32) -> Vec2 {
        Vec2::new(Fixed16_16::from_f32(x), Fixed16_16::from_f32(y))
    }

    fn charge<T: Chargeable>(weapon: &mut T) {
        weapon.charge_to_ready();
    }

    trait Chargeable {
        fn charge_to_ready(&mut self);
    }

    impl Chargeable for Superweapon {
        fn charge_to_ready(&mut self) {
            self.update(Fixed16_16::from_int(CHARGE_SECONDS));
        }
    }

    #[test]
    fn charging_needs_the_full_duration() {
        let mut weapon = Superweapon::new(ClassId::Chronosphere, point(0.0, 0.0), 0);
        weapon.update(Fixed16_16::from_int(CHARGE_SECONDS - 1));
        assert!(!weapon.ready);
        assert!(weapon.consume().is_err());
        weapon.update(Fixed16_16::from_int(2));
        assert!(weapon.ready);
        assert_eq!(weapon.charge_fraction(), 1.0);
        assert!(weapon.consume().is_ok());
        assert!(!weapon.ready);
    }

    #[test]
    fn chronosphere_moves_units_then_consumes_charge() {
        let mut chrono = Chronosphere::new(point(0.0, 0.0), 0);
        let mut units = vec![point(1.0, 1.0), point(2.0, 2.0)];
        assert_eq!(
            chrono.chronoshift(&mut units, point(100.0, 100.0)),
            Err(SuperweaponError::NotReady)
        );

        charge(&mut chrono.base);
        assert_eq!(chrono.chronoshift(&mut units, point(100.0, 100.0)), Ok(2));
        assert_eq!(&units[..2], &[point(100.0, 100.0), point(112.0, 100.0)]);
        assert!(!chrono.base.ready);
    }

    #[test]
    fn iron_curtain_blocks_damage_for_its_window() {
        let mut curtain = IronCurtain::new(point(0.0, 0.0), 0);
        let mut units = vec![
            ArmoredUnit::new(point(10.0, 0.0), Fixed16_16::from_int(100)),
            ArmoredUnit::new(point(1000.0, 0.0), Fixed16_16::from_int(100)),
        ];
        charge(&mut curtain.base);
        assert_eq!(curtain.harden(&mut units, Fixed16_16::from_int(100)), Ok(1));
        units[0].take_damage(Fixed16_16::from_int(500));
        assert_eq!(units[0].health, Fixed16_16::from_int(100));
        units[1].take_damage(Fixed16_16::from_int(30));
        assert_eq!(units[1].health, Fixed16_16::from_int(70));

        units[0].update(Fixed16_16::from_int(IRON_CURTAIN_SECONDS + 1));
        units[0].take_damage(Fixed16_16::from_int(30));
        assert_eq!(units[0].health, Fixed16_16::from_int(70));
    }

    #[test]
    fn gps_reveals_every_enemy_and_resets() {
        let mut gps = GPS::new(point(0.0, 0.0), 0);
        let enemies = vec![point(10.0, 10.0), point(500.0, 500.0)];
        assert!(gps.scan(&enemies).is_err());
        charge(&mut gps.base);
        assert_eq!(gps.scan(&enemies), Ok(enemies.clone()));
        assert_eq!(gps.revealed.len(), 2);
        assert!(!gps.base.ready);
    }
}
