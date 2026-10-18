//! Weapons for Tiberian Dawn.
//!
//! Stats come from the original CONST.CPP / BBDATA.CPP tables in this
//! repository (see td_rules.rs). This module only converts them into the
//! pixel/fixed-point world used by the Rust simulation.

use cnc_core::{ClassId, Fixed16_16, Vec2};

use crate::td_rules::{
    modify_damage, ArmorType, BulletType, WarheadType, WeaponType,
};

/// A weapon instance carried by a unit.
#[derive(Debug, Clone)]
pub struct Weapon {
    pub class_id: ClassId,
    pub weapon_type: WeaponType,
    /// Raw CONST.CPP Attack value (damage points).
    pub damage: Fixed16_16,
    /// CONST.CPP Range converted from leptons to world pixels.
    pub range: Fixed16_16,
    /// Normal shot cooldown: (ROF + 3) ticks, from TECHNO.CPP Rearm_Delay.
    pub rate_of_fire: Fixed16_16,
    /// First shot after locking a target: 9 ticks, from TECHNO.CPP Rearm_Delay.
    pub first_shot_delay: Fixed16_16,
    /// BulletClass speed in world pixels per second.
    pub projectile_speed: Fixed16_16,
    pub warhead: Warhead,
    pub bullet: BulletType,
    /// BULLET.CPP teleports light-speed invisible bullets to the target.
    pub instant: bool,
}

impl Weapon {
    pub fn from_type(weapon_type: WeaponType) -> Self {
        let stats = weapon_type.stats();
        let speed = if stats.instant() {
            // Large but finite, so Projectile::update snaps to the target.
            Fixed16_16::from_int(1_000_000)
        } else {
            Fixed16_16::from_f32(stats.bullet_speed_px_per_sec())
        };

        Self {
            class_id: ClassId::Projectile,
            weapon_type,
            damage: Fixed16_16::from_int(stats.damage),
            range: Fixed16_16::from_f32(stats.range_px()),
            rate_of_fire: Fixed16_16::from_f32(stats.cooldown_seconds()),
            first_shot_delay: Fixed16_16::from_f32(stats.first_shot_seconds()),
            projectile_speed: speed,
            warhead: Warhead {
                warhead_type: stats.warhead(),
            },
            bullet: stats.bullet,
            instant: stats.instant(),
        }
    }
}

/// Warhead wrapper around the CONST.CPP warhead table.
#[derive(Clone, Copy, Debug)]
pub struct Warhead {
    pub warhead_type: WarheadType,
}

impl Warhead {
    /// COMBAT.CPP Modify_Damage applied to this weapon's base damage.
    pub fn damage(
        &self,
        base_damage: Fixed16_16,
        armor: ArmorType,
        distance_leptons: i32,
    ) -> Fixed16_16 {
        let modified = modify_damage(base_damage.int(), self.warhead_type, armor, distance_leptons);
        Fixed16_16::from_int(modified.max(0))
    }
}

/// Projectile in flight.
#[derive(Debug, Clone)]
pub struct Projectile {
    pub weapon: Weapon,
    pub position: Vec2,
    pub target: Vec2,
    pub distance_traveled: Fixed16_16,
    pub source: u32,
    pub target_entity: Option<u32>,
    /// Frames since launch (homing missiles re-aim on even frames).
    pub age: u32,
    /// Visual ballistic height above the ground, in pixels.
    pub altitude: Fixed16_16,
    /// Peak height for arcing projectiles.
    pub arc_height: f32,
    /// Straight-line distance at launch, used for the arc curve.
    pub initial_distance: f32,
    /// Remaining fuelled frames; -1 means unlimited (BULLET.CPP fuel).
    pub ticks_remaining: i32,
    pub homing: bool,
    pub arcing: bool,
}

impl Projectile {
    pub fn new(weapon: Weapon, position: Vec2, target: Vec2, source: u32, target_entity: Option<u32>) -> Self {
        let dx = target.x.to_f32() - position.x.to_f32();
        let dy = target.y.to_f32() - position.y.to_f32();
        let distance = (dx * dx + dy * dy).sqrt();
        let speed = weapon.projectile_speed.to_f32().max(1.0);

        let ticks_remaining = if weapon.bullet.is_fueled() {
            ((distance / speed * 15.0) as i32) + 4
        } else {
            -1
        };

        let homing = weapon.bullet.is_homing();
        let arcing = weapon.bullet.is_arcing();
        let arc_height = if arcing {
            (distance * 0.35).clamp(10.0, 120.0)
        } else {
            0.0
        };

        Self {
            weapon,
            position,
            target,
            distance_traveled: Fixed16_16::ZERO,
            source,
            target_entity,
            age: 0,
            altitude: Fixed16_16::ZERO,
            arc_height,
            initial_distance: distance,
            ticks_remaining,
            homing,
            arcing,
        }
    }

    /// Advance one simulation step. Returns true on impact.
    pub fn update(&mut self, delta_time: Fixed16_16) -> bool {
        self.age = self.age.wrapping_add(1);

        // BULLET.CPP fuel: fuelled projectiles detonate when their fuse expires.
        if self.ticks_remaining > 0 {
            self.ticks_remaining -= 1;
            if self.ticks_remaining == 0 {
                return true;
            }
        }

        let offset = self.target - self.position;
        let distance = offset.length();
        let step = self.weapon.projectile_speed * delta_time;

        let impact = distance <= Fixed16_16::from_f32(1.0) || step >= distance;
        if impact {
            self.position = self.target;
            self.distance_traveled += distance;
        } else {
            self.position = self.position + offset.normalize() * step;
            self.distance_traveled += step;
        }

        if self.arcing {
            let remaining = (self.target - self.position).length().to_f32();
            let progress = if self.initial_distance > 0.0 {
                (1.0 - remaining / self.initial_distance).clamp(0.0, 1.0)
            } else {
                1.0
            };
            self.altitude = Fixed16_16::from_f32(
                (progress * std::f32::consts::PI).sin() * self.arc_height,
            );
        } else {
            self.altitude = Fixed16_16::ZERO;
        }

        impact
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m16_weapon_uses_td_stats() {
        let weapon = Weapon::from_type(WeaponType::M16);
        let stats = WeaponType::M16.stats();
        assert_eq!(weapon.damage.int(), stats.damage);
        assert!((weapon.range.to_f32() - stats.range_px()).abs() < 0.01);
        assert!((weapon.rate_of_fire.to_f32() - stats.cooldown_seconds()).abs() < 0.01);
        assert_eq!(weapon.warhead.warhead_type, WarheadType::Sa);
    }

    #[test]
    fn cannon_uses_ap_warhead_and_is_not_instant() {
        let weapon = Weapon::from_type(WeaponType::Cannon105);
        assert_eq!(weapon.warhead.warhead_type, WarheadType::Ap);
        assert!(!weapon.instant);
        // APDS speed is 100 mph -> 150 px/s.
        assert!((weapon.projectile_speed.to_f32() - 150.0).abs() < 0.01);
    }

    #[test]
    fn rifle_is_instant() {
        let weapon = Weapon::from_type(WeaponType::M16);
        assert!(weapon.instant);
    }

    #[test]
    fn ap_damage_is_reduced_against_infantry_armor() {
        let weapon = Weapon::from_type(WeaponType::Cannon105);
        let versus_infantry = weapon.warhead.damage(weapon.damage, ArmorType::None, 0);
        let versus_tank = weapon.warhead.damage(weapon.damage, ArmorType::Steel, 0);
        assert_eq!(versus_infantry.int(), 8);
        assert_eq!(versus_tank.int(), 30);
    }

    #[test]
    fn arcing_projectile_gains_and_loses_altitude() {
        let weapon = Weapon::from_type(WeaponType::Grenade);
        let mut projectile = Projectile::new(
            weapon,
            Vec2::zero(),
            Vec2::new(Fixed16_16::from_int(100), Fixed16_16::ZERO),
            0,
            None,
        );
        assert!(projectile.arcing);

        let mut max_altitude = 0.0f32;
        let mut hit = false;
        for _ in 0..500 {
            if projectile.update(Fixed16_16::from_f32(0.05)) {
                hit = true;
                break;
            }
            max_altitude = max_altitude.max(projectile.altitude.to_f32());
        }

        assert!(hit, "grenade should land");
        assert!(max_altitude > 1.0, "grenade should arc upward, max {max_altitude}");
    }

    #[test]
    fn fueled_projectile_expires_when_fuse_runs_out() {
        let weapon = Weapon::from_type(WeaponType::Dragon);
        let mut projectile = Projectile::new(
            weapon,
            Vec2::zero(),
            Vec2::new(Fixed16_16::from_int(500), Fixed16_16::ZERO),
            0,
            None,
        );
        assert!(projectile.homing);
        assert!(projectile.ticks_remaining > 0);

        projectile.ticks_remaining = 2;
        assert!(!projectile.update(Fixed16_16::from_f32(0.05)));
        assert!(projectile.update(Fixed16_16::from_f32(0.05)));
    }

    #[test]
    fn projectile_reaches_target() {
        let weapon = Weapon::from_type(WeaponType::Cannon105);
        let mut projectile = Projectile::new(
            weapon,
            Vec2::new(Fixed16_16::from_int(0), Fixed16_16::from_int(0)),
            Vec2::new(Fixed16_16::from_int(150), Fixed16_16::from_int(0)),
            0,
            Some(1),
        );
        let mut hit = false;
        for _ in 0..100 {
            if projectile.update(Fixed16_16::from_f32(0.05)) {
                hit = true;
                break;
            }
        }
        assert!(hit);
        assert!((projectile.position.x.to_f32() - 150.0).abs() < 0.01);
    }
}
