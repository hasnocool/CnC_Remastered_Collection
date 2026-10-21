//! Units for Tiberian Dawn

use cnc_asset::{AssetRegistry, SpriteAtlas};
use cnc_core::rules::RulesDatabase;
use cnc_core::{ClassId, Fixed16_16, Rect, Vec2, Vec2i, ARGB8888};
use cnc_renderer::{Renderer, Sprite};
use serde::{Deserialize, Serialize};

use crate::td_rules::{unit_class_id, ArmorType, UnitStats, UnitType, TICKS_PER_SECOND};
use crate::weapon::Weapon;

const TURRET_TOLERANCE: f32 = 0.10; // ~5.7 degrees

/// Smallest signed angle between two headings.
pub fn facing_difference(a: f32, b: f32) -> f32 {
    let mut difference = (b - a) % (std::f32::consts::TAU);
    if difference > std::f32::consts::PI {
        difference -= std::f32::consts::TAU;
    } else if difference < -std::f32::consts::PI {
        difference += std::f32::consts::TAU;
    }
    difference
}

/// Infantry stance states (C++ `DoType` / `InfantryClass::IsProne`).
///
/// `Standing` is the default upright stance; `Prone` is the lowered stance
/// infantry adopt under fire (half damage, crawl movement).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Stance {
    #[default]
    Standing,
    Prone,
    Crawling,
    Firing,
    FiringProne,
}

/// Door state for units that carry a `DoorClass` (transports, factories).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DoorState {
    #[default]
    Closed,
    Opening,
    Open,
    Closing,
}

/// Sub-cell slot index inside a cell (0..5), matching `StoppingCoordAbs`.
pub type SubCellSlot = u8;

/// Base unit structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub class_id: ClassId,
    pub unit_type: UnitType,
    pub armor: ArmorType,
    pub position: Vec2,
    pub health: Fixed16_16,
    pub max_health: Fixed16_16,
    pub speed: Fixed16_16,
    pub owner: u8,
    pub is_selected: bool,
    /// Optional move-order destination set by the player.
    pub target: Option<Vec2>,
    /// True once the player has given this unit an order; suppresses idle patrol.
    pub ordered: bool,
    /// Weapon this unit fires, if any.
    pub weapon: Option<Weapon>,
    /// UDATA.CPP secondary weapon (Mammoth tusk).
    pub secondary_weapon: Option<Weapon>,
    /// Index into Game::units of the enemy this unit is attacking.
    pub attack_target: Option<usize>,
    /// Seconds remaining before the weapon can fire again.
    pub reload: Fixed16_16,
    /// UDATA.CPP "Is it equipped with a combat turret?".
    pub has_turret: bool,
    /// UDATA.CPP "Fires multiple shots in quick succession?".
    pub two_shooter: bool,
    /// Alternates between the short and long rearm delay for two-shooters.
    pub second_shot: bool,
    /// Body heading in radians.
    pub primary_facing: f32,
    /// Turret heading in radians.
    pub turret_facing: f32,
    /// Turret/body rotation rate in radians per second (ROT * 15 ticks).
    pub rot_per_sec: f32,
    /// Cell-centre waypoints produced by the pathfinder.
    pub path: Vec<Vec2>,
    pub path_index: usize,
    /// Cell reserved for this unit's next path step (dynamic reservation).
    pub reserved_cell: Option<Vec2i>,
    /// Sub-cell slot (0..5) occupied by an infantry unit inside its cell.
    /// `None` for vehicles/aircraft or unassigned infantry.
    #[serde(default)]
    pub sub_cell: Option<u8>,
    #[serde(default)]
    pub sub_cell_slot: Option<u8>,
    #[serde(default)]
    pub sub_slot: Option<u8>,
    #[serde(default)]
    pub cell_slot: Option<u8>,
    #[serde(default)]
    pub slot: Option<u8>,
    /// Infantry stance (standing/prone/...).
    #[serde(default)]
    pub stance: Stance,
    #[serde(default)]
    pub infantry_stance: Stance,
    #[serde(default)]
    pub is_prone: bool,
    /// Door state for transports/factories.
    #[serde(default)]
    pub door_state: DoorState,
    #[serde(default)]
    pub door: DoorState,
    /// True for harvesters.
    pub is_harvester: bool,
    /// Carried tiberium steps (0..HARVESTER_STEP_COUNT).
    pub tiberium: u8,
    /// Countdown until the next harvest or unload action.
    pub harvest_timer: Fixed16_16,
    /// True while the harvester is unloading at a refinery.
    pub unloading: bool,
    /// IDATA.CPP commando row (INFANTRY_RAMBO): may sabotage a building with
    /// vertical C4 (INFANTRY.CPP `What_Action` -> `ACTION_SABOTAGE`).
    pub is_commando: bool,
    /// IDATA.CPP engineer row ("E6"): `IsCapture` is true, so this infantry can
    /// take a building over (INFANTRY.CPP `MISSION_CAPTURE`).
    pub is_engineer: bool,
    /// `MISSION_SABOTAGE` / `MISSION_CAPTURE` destination: index into
    /// `Game::buildings`. None for every ordinary unit.
    pub special_target: Option<usize>,
    /// AADATA.CPP aircraft flag.
    pub is_aircraft: bool,
    /// Remaining ammunition (AADATA.CPP).
    pub ammo: i16,
    pub max_ammo: i16,
    /// Rotor rotation phase for aircraft (radians).
    pub rotor_phase: f32,
    /// Track/tread animation phase for ground vehicles.
    pub track_phase: f32,
    /// Countdown between rearm steps while docked at a helipad.
    pub rearm_timer: Fixed16_16,
    /// Seconds of halved incoming damage granted by a crate armor bonus.
    #[serde(default)]
    pub shield: Fixed16_16,
    /// Passenger unit indices currently aboard this transport, in LIFO boarding
    /// order (CARGO.CPP CargoClass::Attach semantics). Only valid for
    /// `is_transport()` units (APC, Chinook).
    #[serde(default)]
    pub passengers: Vec<usize>,
    /// Cargo capacity: maximum number of passengers. 5 for APC and Chinook,
    /// 0 for all other unit types.
    #[serde(default)]
    pub capacity: usize,
    /// True once the unit has deployed (MCV -> Construction Yard, etc.).
    /// Deployed units stop moving and act as fixed emplacements.
    #[serde(default)]
    pub deployed: bool,
}

impl Unit {
    /// Create a unit from the original TD type stats (IDATA.CPP / UDATA.CPP).
    pub fn new(unit_type: UnitType, position: Vec2, owner: u8) -> Self {
        let stats = unit_type.stats();
        Self {
            class_id: unit_class_id(unit_type),
            unit_type,
            armor: stats.armor,
            position,
            health: stats.health(),
            max_health: stats.health(),
            speed: Fixed16_16::from_f32(stats.speed_px_per_sec()),
            owner,
            is_selected: false,
            target: None,
            ordered: false,
            weapon: stats.weapon.map(Weapon::from_type),
            secondary_weapon: unit_type.secondary_weapon().map(Weapon::from_type),
            attack_target: None,
            reload: Fixed16_16::ZERO,
            has_turret: stats.has_turret,
            two_shooter: stats.two_shooter,
            second_shot: !stats.two_shooter,
            primary_facing: 0.0,
            turret_facing: 0.0,
            rot_per_sec: stats.rot_degrees_per_tick as f32
                * TICKS_PER_SECOND as f32
                * std::f32::consts::PI
                / 180.0,
            path: Vec::new(),
            path_index: 0,
            reserved_cell: None,
            sub_cell: None,
            sub_cell_slot: None,
            sub_slot: None,
            cell_slot: None,
            slot: None,
            stance: Stance::Standing,
            infantry_stance: Stance::Standing,
            is_prone: false,
            door_state: DoorState::Closed,
            door: DoorState::Closed,
            is_harvester: unit_type == UnitType::Harvester,
            tiberium: 0,
            harvest_timer: Fixed16_16::ZERO,
            unloading: false,
            is_commando: unit_type == UnitType::Commando,
            is_engineer: unit_type == UnitType::Engineer,
            special_target: None,
            is_aircraft: stats.is_aircraft,
            ammo: stats.ammo,
            max_ammo: stats.ammo,
            rotor_phase: 0.0,
            track_phase: 0.0,
            rearm_timer: Fixed16_16::ZERO,
            shield: Fixed16_16::ZERO,
            passengers: Vec::new(),
            capacity: match unit_type {
                UnitType::APC | UnitType::Chinook => 5,
                _ => 0,
            },
            deployed: false,
        }
    }

    /// Rotate toward a target heading using the ROT rate.
    pub fn rotate_toward(&mut self, desired: f32, delta_time: Fixed16_16) {
        let step = self.rot_per_sec * delta_time.to_f32();
        if self.has_turret {
            let difference = facing_difference(self.turret_facing, desired);
            self.turret_facing += difference.clamp(-step, step);
        } else if !self.is_infantry() {
            let difference = facing_difference(self.primary_facing, desired);
            self.primary_facing += difference.clamp(-step, step);
        }
    }

    /// TurretClass::Fire_At facing gate: infantry fire instantly, vehicles
    /// need their turret or body aligned with the target.
    pub fn aiming_at(&self, desired: f32) -> bool {
        if self.is_infantry() {
            return true;
        }
        let facing = if self.has_turret {
            self.turret_facing
        } else {
            self.primary_facing
        };
        facing_difference(facing, desired).abs() <= TURRET_TOLERANCE
    }

    pub fn is_infantry(&self) -> bool {
        self.unit_type.stats().is_infantry
    }

    /// Effective sub-cell slot, checking every alias field.
    pub fn effective_sub_cell(&self) -> Option<u8> {
        self.sub_cell
            .or(self.sub_cell_slot)
            .or(self.sub_slot)
            .or(self.cell_slot)
            .or(self.slot)
    }

    /// Assign one sub-cell slot to all alias fields.
    pub fn set_sub_cell(&mut self, slot: Option<u8>) {
        self.sub_cell = slot;
        self.sub_cell_slot = slot;
        self.sub_slot = slot;
        self.cell_slot = slot;
        self.slot = slot;
    }

    /// Pixel offsets inside a cell for the five `StoppingCoordAbs` slots.
    pub const STOPPING_OFFSETS_PX: [(f32, f32); 5] = [
        (12.0, 12.0),
        (6.0, 6.0),
        (18.0, 6.0),
        (6.0, 18.0),
        (18.0, 18.0),
    ];

    /// Original C++ `StoppingCoordAbs` values (lepton-packed COORDINATE).
    pub const STOPPING_COORD_ABS: [u32; 5] = [
        0x0080_0080,
        0x0040_0040,
        0x0040_00C0,
        0x00C0_0040,
        0x00C0_00C0,
    ];

    pub fn stopping_offset(slot: u8) -> Option<cnc_core::Vec2> {
        let (x, y) = Self::STOPPING_OFFSETS_PX.get(slot as usize).copied()?;
        Some(cnc_core::Vec2::new(
            cnc_core::Fixed16_16::from_f32(x),
            cnc_core::Fixed16_16::from_f32(y),
        ))
    }

    pub fn set_stance(&mut self, stance: Stance) {
        self.stance = stance;
        self.infantry_stance = stance;
        self.is_prone = stance == Stance::Prone;
    }

    /// Stance-dependent movement speed multiplier.
    pub fn stance_speed_multiplier(&self) -> f32 {
        match self.effective_stance() {
            Stance::Standing | Stance::Firing => 1.0,
            Stance::Prone | Stance::FiringProne => 0.0,
            Stance::Crawling => 0.5,
        }
    }

    pub fn effective_stance(&self) -> Stance {
        if self.is_prone {
            Stance::Prone
        } else {
            self.stance
        }
    }

    pub fn resolved_speed(&self) -> cnc_core::Fixed16_16 {
        self.speed * cnc_core::Fixed16_16::from_f32(self.stance_speed_multiplier())
    }

    pub fn open_door(&mut self) -> bool {
        if self.is_transport() {
            self.door_state = DoorState::Open;
            self.door = DoorState::Open;
            true
        } else {
            false
        }
    }

    pub fn close_door(&mut self) -> bool {
        if self.is_transport() {
            self.door_state = DoorState::Closed;
            self.door = DoorState::Closed;
            true
        } else {
            false
        }
    }

    pub fn effective_door_state(&self) -> DoorState {
        if self.door_state != DoorState::Closed {
            self.door_state
        } else {
            self.door
        }
    }

    pub fn is_door_open(&self) -> bool {
        matches!(
            self.effective_door_state(),
            DoorState::Open | DoorState::Opening
        )
    }

    pub fn is_transport(&self) -> bool {
        matches!(
            self.unit_type,
            crate::td_rules::UnitType::APC | crate::td_rules::UnitType::Chinook
        )
    }

    /// Cargo capacity: max passengers this transport can carry.
    pub fn cargo_capacity(&self) -> usize {
        self.capacity
    }

    /// Number of passengers currently aboard.
    pub fn passenger_count(&self) -> usize {
        self.passengers.len()
    }

    /// True if there is room for at least one more passenger.
    pub fn has_cargo_space(&self) -> bool {
        self.passengers.len() < self.capacity
    }

    /// True if the given unit index is a passenger aboard this transport.
    pub fn has_passenger(&self, unit_index: usize) -> bool {
        self.passengers.contains(&unit_index)
    }

    pub fn next_rearm_delay(&self, weapon: &Weapon) -> Fixed16_16 {
        if self.second_shot {
            weapon.rate_of_fire
        } else {
            weapon.first_shot_delay
        }
    }

    pub fn note_shot_fired(&mut self) {
        if self.two_shooter {
            self.second_shot = !self.second_shot;
        } else {
            self.second_shot = true;
        }
    }

    pub fn set_path(&mut self, path: Vec<Vec2>) {
        self.path = path;
        self.path_index = 0;
    }

    pub fn clear_path(&mut self) {
        self.path.clear();
        self.path_index = 0;
    }

    pub fn stats(&self) -> UnitStats {
        self.unit_type.stats()
    }

    pub fn equip_weapon(&mut self, weapon: Weapon) {
        self.weapon = Some(weapon);
    }

    /// Rebuild carried weapons from Rules.ini overrides, keeping their types.
    pub fn apply_weapon_rules(&mut self, rules: &RulesDatabase) {
        if let Some(weapon) = self.weapon.as_ref() {
            self.weapon = Some(Weapon::from_type_with_rules(weapon.weapon_type, rules));
        }
        if let Some(weapon) = self.secondary_weapon.as_ref() {
            self.secondary_weapon = Some(Weapon::from_type_with_rules(weapon.weapon_type, rules));
        }
    }

    pub fn take_damage(&mut self, amount: Fixed16_16) {
        // Crate armor bonus halves incoming damage while the shield holds.
        let amount = if self.shield > Fixed16_16::ZERO {
            amount * Fixed16_16::from_f32(0.5)
        } else {
            amount
        };
        self.health = (self.health - amount).max(Fixed16_16::ZERO);
    }

    pub fn heal(&mut self, amount: Fixed16_16) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn is_alive(&self) -> bool {
        self.health > Fixed16_16::ZERO
    }

    pub fn move_to(&mut self, target: Vec2, delta_time: Fixed16_16) {
        let direction = (target - self.position).normalize();
        let movement = direction * self.speed * delta_time;
        self.position = self.position + movement;
    }

    /// Advance one simulation step toward the current move order.
    ///
    /// Follows the precomputed waypoint path when present, otherwise falls back
    /// to straight-line movement toward the target.
    /// Returns true while the unit still has an active order.
    pub fn advance_order(&mut self, delta_time: Fixed16_16) -> bool {
        if self.path_index < self.path.len() {
            let waypoint = self.path[self.path_index];
            let offset = waypoint - self.position;
            let distance = offset.length();
            let step = self.speed * delta_time;

            if distance <= Fixed16_16::from_f32(1.0) || step >= distance {
                self.position = waypoint;
                self.path_index += 1;
                if self.path_index >= self.path.len() {
                    self.path.clear();
                    self.path_index = 0;
                    self.target = None;
                    return false;
                }
                return true;
            }

            self.position = self.position + offset.normalize() * step;
            return true;
        }

        let Some(target) = self.target else {
            return false;
        };
        let offset = target - self.position;
        let distance = offset.length();
        if distance <= Fixed16_16::from_f32(1.0) {
            self.target = None;
            return false;
        }
        let step = self.speed * delta_time;
        if step >= distance {
            self.position = target;
            self.target = None;
            false
        } else {
            self.position = self.position + offset.normalize() * step;
            true
        }
    }

    /// Draw this unit through any cnc_renderer::Renderer implementation.
    ///
    /// Real SHP sprites are not wired yet, so the unit is drawn as a colored
    /// placeholder quad; the position uses the same fixed-point world space as
    /// the simulation.
    pub fn render(&self, renderer: &mut dyn Renderer) {
        self.render_with(renderer, None);
    }

    pub fn render_with(&self, renderer: &mut dyn Renderer, atlas: Option<&SpriteAtlas>) {
        if !self.is_alive() {
            renderer.draw_rect(
                self.position.x.to_f32(),
                self.position.y.to_f32(),
                16.0,
                16.0,
                [0.25, 0.25, 0.25, 1.0],
            );
            return;
        }

        if let Some(atlas) = atlas {
            if let Some(frame) = atlas.frame(&format!("{:?}", self.class_id)) {
                let tint = if self.is_selected {
                    [0.25, 1.0, 0.30, 1.0]
                } else if self.owner != 0 {
                    [1.0, 0.55, 0.50, 1.0]
                } else {
                    [1.0, 1.0, 1.0, 1.0]
                };
                let y = self.position.y.to_f32() + if self.is_aircraft { -14.0 } else { 0.0 };

                if self.is_aircraft {
                    renderer.draw_rect(
                        self.position.x.to_f32() + 4.0,
                        self.position.y.to_f32() + 4.0,
                        12.0,
                        12.0,
                        [0.10, 0.10, 0.10, 0.7],
                    );
                }

                let mut sprite = Sprite::new(
                    0, // texture slot 0 = procedural unit atlas
                    Vec2::new(self.position.x, Fixed16_16::from_f32(y)),
                    Vec2::new(Fixed16_16::from_int(16), Fixed16_16::from_int(16)),
                );
                sprite.uv_rect = Rect::new(
                    frame.x as i32,
                    frame.y as i32,
                    frame.w as i32,
                    frame.h as i32,
                );
                sprite.color = ARGB8888::new(
                    (tint[3] * 255.0) as u8,
                    (tint[0] * 255.0) as u8,
                    (tint[1] * 255.0) as u8,
                    (tint[2] * 255.0) as u8,
                );
                renderer.draw_sprite(&sprite);
                return;
            }
        }

        let color = if self.is_selected {
            [0.0, 1.0, 0.0, 1.0]
        } else if self.owner != 0 {
            [0.90, 0.20, 0.20, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        };
        if self.is_aircraft {
            let x = self.position.x.to_f32();
            let y = self.position.y.to_f32();
            renderer.draw_rect(x + 4.0, y + 4.0, 12.0, 12.0, [0.10, 0.10, 0.10, 0.7]);
            renderer.draw_rect(x, y - 14.0, 16.0, 16.0, color);
            // Two rotor-blade tips spinning around the body centre.
            let center_x = x + 8.0;
            let center_y = y - 6.0;
            for offset in [0.0f32, std::f32::consts::FRAC_PI_2] {
                let angle = self.rotor_phase + offset;
                renderer.draw_rect(
                    center_x + angle.cos() * 12.0 - 1.0,
                    center_y + angle.sin() * 12.0 - 1.0,
                    2.0,
                    2.0,
                    [0.70, 0.80, 0.90, 1.0],
                );
            }
            return;
        }

        if !self.is_infantry() {
            let tread = if ((self.track_phase * 2.0) as i32) % 2 == 0 {
                [0.22, 0.22, 0.22, 1.0]
            } else {
                [0.58, 0.58, 0.58, 1.0]
            };
            renderer.draw_rect(
                self.position.x.to_f32() + 1.0,
                self.position.y.to_f32() + 2.0,
                2.0,
                12.0,
                tread,
            );
            renderer.draw_rect(
                self.position.x.to_f32() + 13.0,
                self.position.y.to_f32() + 2.0,
                2.0,
                12.0,
                tread,
            );
        }

        renderer.draw_rect(
            self.position.x.to_f32(),
            self.position.y.to_f32(),
            16.0,
            16.0,
            color,
        );

        if self.has_turret {
            let center_x = self.position.x.to_f32() + 8.0;
            let center_y = self.position.y.to_f32() + 8.0;
            renderer.draw_rect(
                center_x + self.turret_facing.cos() * 10.0 - 2.0,
                center_y + self.turret_facing.sin() * 10.0 - 2.0,
                4.0,
                4.0,
                [0.75, 0.75, 0.75, 1.0],
            );
        }
    }

    /// Resolve this unit's sprite bytes from the asset registry.
    pub fn asset(&self, assets: &AssetRegistry) -> Option<Vec<u8>> {
        assets
            .get(&format!("units/{:?}.shp", self.class_id))
            .cloned()
    }
}

/// Infantry unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Infantry {
    pub base: Unit,
    pub veterancy: u8,
}

impl Infantry {
    pub fn new(base: Unit) -> Self {
        Self { base, veterancy: 0 }
    }
}

/// Vehicle unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub base: Unit,
    pub turret_rotation: Fixed16_16,
}

impl Vehicle {
    pub fn new(base: Unit) -> Self {
        Self {
            base,
            turret_rotation: Fixed16_16::ZERO,
        }
    }
}

/// Aircraft unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aircraft {
    pub base: Unit,
    pub altitude: Fixed16_16,
    pub target_position: Option<Vec2>,
}

impl Aircraft {
    pub fn new(base: Unit) -> Self {
        Self {
            base,
            altitude: Fixed16_16::ZERO,
            target_position: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_shooter_alternates_short_and_long_rearm() {
        let mut unit = Unit::new(UnitType::MammothTank, Vec2::zero(), 0);
        let weapon = unit.weapon.clone().expect("Mammoth has a weapon");

        assert!(!unit.second_shot, "two-shooters start with the short delay");
        assert_eq!(unit.next_rearm_delay(&weapon), weapon.first_shot_delay);

        unit.note_shot_fired();
        assert!(unit.second_shot);
        assert_eq!(unit.next_rearm_delay(&weapon), weapon.rate_of_fire);

        unit.note_shot_fired();
        assert!(!unit.second_shot);
        assert_eq!(unit.next_rearm_delay(&weapon), weapon.first_shot_delay);
    }

    #[test]
    fn single_shooter_always_uses_normal_rearm() {
        let mut unit = Unit::new(UnitType::MediumTank, Vec2::zero(), 0);
        let weapon = unit.weapon.clone().expect("tank has a weapon");
        assert!(
            unit.second_shot,
            "single shooters start on the normal delay"
        );
        assert_eq!(unit.next_rearm_delay(&weapon), weapon.rate_of_fire);
        unit.note_shot_fired();
        assert!(unit.second_shot);
        assert_eq!(unit.next_rearm_delay(&weapon), weapon.rate_of_fire);
    }

    #[test]
    fn turret_turns_toward_heading() {
        let mut unit = Unit::new(UnitType::MediumTank, Vec2::zero(), 0);
        let start = unit.turret_facing;
        // Aim north.
        unit.rotate_toward(-std::f32::consts::FRAC_PI_2, Fixed16_16::from_f32(0.5));
        assert!(unit.turret_facing < start);
        assert!(
            facing_difference(unit.turret_facing, -std::f32::consts::FRAC_PI_2).abs()
                < std::f32::consts::FRAC_PI_2
        );
    }

    #[test]
    fn special_infantry_carry_role_flags() {
        // IDATA.CPP:582 marks the commando `Is a leader type?` and keeps
        // `Can this infantry type capture a building?` false; the engineer row
        // (IDATA.CPP:501) is the mirror image.
        let commando = Unit::new(UnitType::Commando, Vec2::zero(), 0);
        assert!(commando.is_commando);
        assert!(!commando.is_engineer);
        assert_eq!(commando.class_id, ClassId::InfantryE10);
        assert!(
            commando.weapon.is_some(),
            "WEAPON_RIFLE row keeps its primary weapon"
        );

        let engineer = Unit::new(UnitType::Engineer, Vec2::zero(), 0);
        assert!(engineer.is_engineer);
        assert!(!engineer.is_commando);
        assert_eq!(engineer.class_id, ClassId::InfantryE6);
        assert!(engineer.weapon.is_none(), "WEAPON_NONE carries no weapon");

        // Ordinary infantry keep neither role and start without a mission.
        let rifleman = Unit::new(UnitType::InfantryE1, Vec2::zero(), 0);
        assert!(!rifleman.is_commando);
        assert!(!rifleman.is_engineer);
        assert_eq!(rifleman.special_target, None);
    }
}
