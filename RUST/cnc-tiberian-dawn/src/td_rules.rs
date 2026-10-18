//! Game rules ported from the original Tiberian Dawn source.
//!
//! Sources in this repository:
//! - CONST.CPP: Weapons[] and Warheads[] tables
//! - BBDATA.CPP: BulletTypeClass definitions (speed + warhead)
//! - IDATA.CPP / UDATA.CPP: infantry and unit type stats
//! - DEFINES.H: MPHType speed values
//! - DISPLAY.H: lepton / pixel constants
//! - COMBAT.CPP: Modify_Damage
//! - TECHNO.CPP: Rearm_Delay, Fire_At
//! - FOOT.CPP: Approach_Target
//! - MAP.CPP: Nearby_Location (RA backport)

use cnc_core::{ClassId, Fixed16_16};

/// Leptons per map cell (DISPLAY.H: ICON_LEPTON_W).
pub const CELL_LEPTON_W: i32 = 256;
/// Pixels per map cell (DISPLAY.H: ICON_PIXEL_W).
pub const ICON_PIXEL_W: i32 = 24;
/// Leptons per pixel; integer division exactly as DISPLAY.H does it.
pub const PIXEL_LEPTON_W: i32 = CELL_LEPTON_W / ICON_PIXEL_W;
/// Simulation ticks per second (DEFINES.H).
pub const TICKS_PER_SECOND: i32 = 15;
/// Explosion damage radius from COMBAT.CPP Explosion_Damage:
/// ICON_LEPTON_W + (ICON_LEPTON_W >> 1) = 384 leptons (1.5 cells).
pub const EXPLOSION_RANGE_LEPTONS: i32 = CELL_LEPTON_W + (CELL_LEPTON_W >> 1);

/// Armor classes from DEFINES.H.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmorType {
    None = 0,
    Wood = 1,
    Aluminum = 2,
    Steel = 3,
    Concrete = 4,
}

impl ArmorType {
    pub const fn index(self) -> usize {
        self as usize
    }
}

/// Warhead classes from DEFINES.H.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarheadType {
    Sa,
    He,
    Ap,
    Fire,
    Laser,
    Pb,
    Fist,
    Foot,
    HollowPoint,
    Spore,
    HeadButt,
    FeedMe,
}

impl WarheadType {
    /// Spread factor from CONST.CPP Warheads[].
    pub const fn spread_factor(self) -> i32 {
        match self {
            WarheadType::Sa => 2,
            WarheadType::He => 6,
            WarheadType::Ap => 6,
            WarheadType::Fire => 8,
            WarheadType::Laser => 4,
            WarheadType::Pb => 7,
            WarheadType::Fist => 4,
            WarheadType::Foot => 4,
            WarheadType::HollowPoint => 4,
            WarheadType::Spore => 255,
            WarheadType::HeadButt => 1,
            WarheadType::FeedMe => 1,
        }
    }

    /// Armor modifiers from CONST.CPP Warheads[] (1/256 scale).
    pub const fn modifiers(self) -> [i32; 5] {
        match self {
            WarheadType::Sa => [0x100, 0x80, 0x90, 0x40, 0x40],
            WarheadType::He => [0xE0, 0xC0, 0x90, 0x40, 0x100],
            WarheadType::Ap => [0x40, 0xC0, 0xC0, 0x100, 0x80],
            WarheadType::Fire => [0xE0, 0x100, 0xB0, 0x40, 0x80],
            WarheadType::Laser => [0x100, 0x100, 0x100, 0x100, 0x100],
            WarheadType::Pb => [0x100, 0x100, 0xC0, 0xC0, 0xC0],
            WarheadType::Fist => [0x100, 0x20, 0x20, 0x10, 0x10],
            WarheadType::Foot => [0x100, 0x20, 0x20, 0x10, 0x10],
            WarheadType::HollowPoint => [0x100, 0x08, 0x08, 0x08, 0x08],
            WarheadType::Spore => [0x100, 0x01, 0x01, 0x01, 0x01],
            WarheadType::HeadButt => [0x100, 0xC0, 0x80, 0x20, 0x08],
            WarheadType::FeedMe => [0x100, 0xC0, 0x80, 0x20, 0x08],
        }
    }
}

/// COORDA.ASM Fixed_To_Cardinal.
pub fn fixed_to_cardinal(base: i32, fixed: i32) -> i32 {
    (((base as i64 * fixed as i64) + 0x80) >> 8) as i32
}

/// COMBAT.CPP Modify_Damage.
pub fn modify_damage(
    damage: i32,
    warhead: WarheadType,
    armor: ArmorType,
    distance_leptons: i32,
) -> i32 {
    if damage == 0 {
        return 0;
    }

    let mut damage = fixed_to_cardinal(damage, warhead.modifiers()[armor.index()]);
    if damage != 0 {
        let mut distance = distance_leptons >> warhead.spread_factor();
        distance = distance.clamp(0, 16);
        damage >>= distance;
    }
    damage
}

/// Bullet classes from BBDATA.CPP with their speed and warhead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BulletType {
    Sniper,
    Bullet,
    Apds,
    He,
    Ssm,
    Ssm2,
    Sam,
    Tow,
    Flame,
    ChemSpray,
    Napalm,
    Grenade,
    Laser,
    HonestJohn,
    Spreadfire,
    HeadButt,
    TrexBite,
}

impl BulletType {
    pub const fn warhead(self) -> WarheadType {
        match self {
            BulletType::Sniper => WarheadType::HollowPoint,
            BulletType::Bullet => WarheadType::Sa,
            BulletType::Apds => WarheadType::Ap,
            BulletType::He => WarheadType::He,
            BulletType::Ssm => WarheadType::He,
            BulletType::Ssm2 => WarheadType::He,
            BulletType::Sam => WarheadType::Ap,
            BulletType::Tow => WarheadType::Ap,
            BulletType::Flame => WarheadType::Fire,
            BulletType::ChemSpray => WarheadType::He,
            BulletType::Napalm => WarheadType::Fire,
            BulletType::Grenade => WarheadType::He,
            BulletType::Laser => WarheadType::Laser,
            BulletType::HonestJohn => WarheadType::Fire,
            BulletType::Spreadfire => WarheadType::He,
            BulletType::HeadButt => WarheadType::HeadButt,
            BulletType::TrexBite => WarheadType::FeedMe,
        }
    }

    /// BBDATA.CPP SPEED field (MPHType).
    pub const fn speed_mph(self) -> i32 {
        match self {
            BulletType::Sniper => 255,
            BulletType::Bullet => 255,
            BulletType::Apds => 100,
            BulletType::He => 30,
            BulletType::Ssm => 60,
            BulletType::Ssm2 => 60,
            BulletType::Sam => 100,
            BulletType::Tow => 60,
            BulletType::Flame => 40,
            BulletType::ChemSpray => 40,
            BulletType::Napalm => 12,
            BulletType::Grenade => 12,
            BulletType::Laser => 255,
            BulletType::HonestJohn => 40,
            BulletType::Spreadfire => 255,
            BulletType::HeadButt => 255,
            BulletType::TrexBite => 255,
        }
    }

    /// BBDATA.CPP IsHoming.
    pub const fn is_homing(self) -> bool {
        matches!(
            self,
            BulletType::Ssm
                | BulletType::Ssm2
                | BulletType::Sam
                | BulletType::Tow
                | BulletType::HonestJohn
        )
    }

    /// BBDATA.CPP IsArcing.
    pub const fn is_arcing(self) -> bool {
        matches!(self, BulletType::Grenade | BulletType::He)
    }

    /// BBDATA.CPP IsFueled.
    pub const fn is_fueled(self) -> bool {
        matches!(
            self,
            BulletType::Ssm
                | BulletType::Ssm2
                | BulletType::Sam
                | BulletType::Tow
                | BulletType::HonestJohn
        )
    }

    /// True when the projectile is invisible (BBDATA.CPP IsInvisible).
    pub const fn is_invisible(self) -> bool {
        matches!(
            self,
            BulletType::Sniper
                | BulletType::Bullet
                | BulletType::Laser
                | BulletType::Spreadfire
                | BulletType::HeadButt
                | BulletType::TrexBite
        )
    }
}

/// Weapon classes and their CONST.CPP stats (bullet, damage, ROF ticks, range leptons).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponType {
    Rifle,
    ChainGun,
    Pistol,
    M16,
    Dragon,
    Flamethrower,
    FlameTongue,
    ChemSpray,
    Grenade,
    Cannon75,
    Cannon105,
    Cannon120,
    TurretGun,
    MammothTusk,
    Mlrs,
    Cannon155,
    M60Mg,
    Tomahawk,
    TowTwo,
    Napalm,
    ObeliskLaser,
    Nike,
    HonestJohn,
    Steg,
    Trex,
}

#[derive(Clone, Copy, Debug)]
pub struct WeaponStats {
    pub bullet: BulletType,
    pub damage: i32,
    pub rof_ticks: i32,
    pub range_leptons: i32,
}

impl WeaponType {
    /// Exact rows from CONST.CPP Weapons[].
    pub const fn stats(self) -> WeaponStats {
        match self {
            WeaponType::Rifle => WeaponStats { bullet: BulletType::Sniper, damage: 125, rof_ticks: 40, range_leptons: 0x0580 },
            WeaponType::ChainGun => WeaponStats { bullet: BulletType::Spreadfire, damage: 25, rof_ticks: 50, range_leptons: 0x0400 },
            WeaponType::Pistol => WeaponStats { bullet: BulletType::Bullet, damage: 1, rof_ticks: 7, range_leptons: 0x01C0 },
            WeaponType::M16 => WeaponStats { bullet: BulletType::Bullet, damage: 15, rof_ticks: 20, range_leptons: 0x0200 },
            WeaponType::Dragon => WeaponStats { bullet: BulletType::Tow, damage: 30, rof_ticks: 60, range_leptons: 0x0400 },
            WeaponType::Flamethrower => WeaponStats { bullet: BulletType::Flame, damage: 35, rof_ticks: 50, range_leptons: 0x0200 },
            WeaponType::FlameTongue => WeaponStats { bullet: BulletType::Flame, damage: 50, rof_ticks: 50, range_leptons: 0x0200 },
            WeaponType::ChemSpray => WeaponStats { bullet: BulletType::ChemSpray, damage: 80, rof_ticks: 70, range_leptons: 0x0200 },
            WeaponType::Grenade => WeaponStats { bullet: BulletType::Grenade, damage: 50, rof_ticks: 60, range_leptons: 0x0340 },
            WeaponType::Cannon75 => WeaponStats { bullet: BulletType::Apds, damage: 25, rof_ticks: 60, range_leptons: 0x0400 },
            WeaponType::Cannon105 => WeaponStats { bullet: BulletType::Apds, damage: 30, rof_ticks: 50, range_leptons: 0x04C0 },
            WeaponType::Cannon120 => WeaponStats { bullet: BulletType::Apds, damage: 40, rof_ticks: 80, range_leptons: 0x04C0 },
            WeaponType::TurretGun => WeaponStats { bullet: BulletType::Apds, damage: 40, rof_ticks: 60, range_leptons: 0x0600 },
            WeaponType::MammothTusk => WeaponStats { bullet: BulletType::Ssm, damage: 75, rof_ticks: 80, range_leptons: 0x0500 },
            WeaponType::Mlrs => WeaponStats { bullet: BulletType::Ssm2, damage: 75, rof_ticks: 80, range_leptons: 0x0600 },
            WeaponType::Cannon155 => WeaponStats { bullet: BulletType::He, damage: 150, rof_ticks: 65, range_leptons: 0x0600 },
            WeaponType::M60Mg => WeaponStats { bullet: BulletType::Bullet, damage: 15, rof_ticks: 30, range_leptons: 0x0400 },
            WeaponType::Tomahawk => WeaponStats { bullet: BulletType::Ssm, damage: 60, rof_ticks: 35, range_leptons: 0x0780 },
            WeaponType::TowTwo => WeaponStats { bullet: BulletType::Ssm, damage: 60, rof_ticks: 40, range_leptons: 0x0680 },
            WeaponType::Napalm => WeaponStats { bullet: BulletType::Napalm, damage: 100, rof_ticks: 20, range_leptons: 0x0480 },
            WeaponType::ObeliskLaser => WeaponStats { bullet: BulletType::Laser, damage: 200, rof_ticks: 90, range_leptons: 0x0780 },
            WeaponType::Nike => WeaponStats { bullet: BulletType::Sam, damage: 50, rof_ticks: 50, range_leptons: 0x0780 },
            WeaponType::HonestJohn => WeaponStats { bullet: BulletType::HonestJohn, damage: 100, rof_ticks: 200, range_leptons: 0x0A00 },
            WeaponType::Steg => WeaponStats { bullet: BulletType::HeadButt, damage: 100, rof_ticks: 30, range_leptons: 0x0180 },
            WeaponType::Trex => WeaponStats { bullet: BulletType::TrexBite, damage: 155, rof_ticks: 30, range_leptons: 0x0180 },
        }
    }
}

impl WeaponStats {
    pub fn range_px(self) -> f32 {
        leptons_to_px(self.range_leptons)
    }

    /// TECHNO.CPP Rearm_Delay: ROF + 3 ticks for the normal shot.
    pub fn cooldown_seconds(self) -> f32 {
        ticks_to_seconds(self.rof_ticks + 3)
    }

    /// The first shot after acquiring a target uses a shorter 9-tick delay.
    pub fn first_shot_seconds(self) -> f32 {
        ticks_to_seconds(9)
    }

    /// BulletClass physics: one pixel per PIXEL_LEPTON_W accumulated speed.
    pub fn bullet_speed_px_per_sec(self) -> f32 {
        if self.bullet.is_invisible() && self.bullet.speed_mph() == 255 {
            // BULLET.CPP teleports light-speed invisible bullets to the target.
            f32::MAX
        } else {
            mph_to_px_per_sec_vehicle(self.bullet.speed_mph())
        }
    }

    pub fn warhead(self) -> WarheadType {
        self.bullet.warhead()
    }

    pub fn instant(self) -> bool {
        self.bullet.is_invisible() && self.bullet.speed_mph() == 255
    }
}

/// HOUSE.CPP Power_Fraction: power * 256 / drain, or 256 with no drain.
pub fn power_fraction(power: i32, drain: i32) -> i32 {
    if power <= 0 {
        return 0;
    }
    if drain <= 0 {
        return 0x0100;
    }
    ((power * 256) / drain).min(0x0100)
}

/// TECHNO.CPP TechnoTypeClass::Time_To_Build: cost ticks scaled by power.
pub fn production_seconds(cost: i32, power: i32, drain: i32) -> f32 {
    let fraction = power_fraction(power, drain);
    let inv_scale = if fraction == 0 {
        64
    } else if fraction < 0x0080 {
        103
    } else if fraction < 0x0100 {
        171
    } else {
        256
    };
    (cost as f32 * 256.0 / inv_scale as f32) / TICKS_PER_SECOND as f32
}

/// TYPE.H UnitTypeClass::TIBERIUM_STEP / STEP_COUNT / FULL_LOAD_CREDITS.
pub const HARVESTER_STEP_CREDITS: i32 = 25;
pub const HARVESTER_STEP_COUNT: u8 = 28;
pub const HARVESTER_FULL_LOAD: i32 = HARVESTER_STEP_CREDITS * HARVESTER_STEP_COUNT as i32;

/// Tiberium overlay stages used by the growth rules (0..11).
pub const TIBERIUM_STAGES: u8 = 12;

pub fn tiberium_stage(amount: u8) -> u8 {
    ((amount as u32 * TIBERIUM_STAGES as u32) / 256) as u8
}

pub fn tiberium_amount_for_stage(stage: u8) -> u8 {
    ((stage as u32 * 256) / TIBERIUM_STAGES as u32).min(255) as u8
}

/// Convert leptons to our pixel-based world coordinates.
pub fn leptons_to_px(leptons: i32) -> f32 {
    leptons as f32 * ICON_PIXEL_W as f32 / CELL_LEPTON_W as f32
}

pub fn px_to_leptons(px: f32) -> i32 {
    (px * CELL_LEPTON_W as f32 / ICON_PIXEL_W as f32) as i32
}

/// Ticks to seconds (15 ticks per second).
pub fn ticks_to_seconds(ticks: i32) -> f32 {
    ticks as f32 / TICKS_PER_SECOND as f32
}

/// DriveClass / FlyClass movement speed: mph pixels of accumulation per tick is
/// divided by PIXEL_LEPTON_W, so px per second = mph * 15 / 10 = mph * 1.5.
pub fn mph_to_px_per_sec_vehicle(mph: i32) -> f32 {
    mph as f32 * TICKS_PER_SECOND as f32 / PIXEL_LEPTON_W as f32
}

/// InfantryClass moves directly in leptons per tick.
pub fn mph_to_px_per_sec_infantry(mph: i32) -> f32 {
    leptons_to_px(mph) * TICKS_PER_SECOND as f32
}

/// Unit stats from IDATA.CPP / UDATA.CPP for the demo roster.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitType {
    InfantryE1,
    MediumTank,
    MammothTank,
    Harvester,
    Jeep,
    Buggy,
    Orca,
    Apache,
}

#[derive(Clone, Copy, Debug)]
pub struct UnitStats {
    pub strength: i32,
    pub max_speed_mph: i32,
    pub weapon: Option<WeaponType>,
    pub armor: ArmorType,
    pub is_infantry: bool,
    pub cost: i32,
    pub sight_range: i32,
    /// UDATA.CPP "Is it equipped with a combat turret?".
    pub has_turret: bool,
    /// UDATA.CPP ROT (degrees per tick).
    pub rot_degrees_per_tick: i32,
    /// UDATA.CPP "Fires multiple shots in quick succession?".
    pub two_shooter: bool,
    /// AADATA.CPP aircraft (fly over terrain, ignore pathfinding).
    pub is_aircraft: bool,
    /// AADATA.CPP starting ammunition.
    pub ammo: i16,
}

impl UnitType {
    pub const fn stats(self) -> UnitStats {
        match self {
            UnitType::InfantryE1 => UnitStats {
                strength: 50,
                max_speed_mph: 8,
                weapon: Some(WeaponType::M16),
                armor: ArmorType::None,
                is_infantry: true,
                cost: 100,
                sight_range: 1,
                has_turret: false,
                rot_degrees_per_tick: 0,
                two_shooter: false,
                is_aircraft: false,
                ammo: 0,
            },
            UnitType::MediumTank => UnitStats {
                strength: 400,
                max_speed_mph: 18,
                weapon: Some(WeaponType::Cannon105),
                armor: ArmorType::Steel,
                is_infantry: false,
                cost: 800,
                sight_range: 3,
                has_turret: true,
                rot_degrees_per_tick: 5,
                two_shooter: false,
                is_aircraft: false,
                ammo: 0,
            },
            UnitType::Harvester => UnitStats {
                strength: 600,
                max_speed_mph: 12,
                weapon: None,
                armor: ArmorType::Aluminum,
                is_infantry: false,
                cost: 1400,
                sight_range: 2,
                has_turret: false,
                rot_degrees_per_tick: 5,
                two_shooter: false,
                is_aircraft: false,
                ammo: 0,
            },
            UnitType::MammothTank => UnitStats {
                strength: 600,
                max_speed_mph: 12,
                weapon: Some(WeaponType::Cannon120),
                armor: ArmorType::Steel,
                is_infantry: false,
                cost: 1500,
                sight_range: 4,
                has_turret: true,
                rot_degrees_per_tick: 5,
                two_shooter: true,
                is_aircraft: false,
                ammo: 0,
            },
            UnitType::Jeep => UnitStats {
                strength: 150,
                max_speed_mph: 30,
                weapon: Some(WeaponType::M60Mg),
                armor: ArmorType::Aluminum,
                is_infantry: false,
                cost: 400,
                sight_range: 2,
                has_turret: true,
                rot_degrees_per_tick: 10,
                two_shooter: false,
                is_aircraft: false,
                ammo: 0,
            },
            UnitType::Buggy => UnitStats {
                strength: 140,
                max_speed_mph: 30,
                weapon: Some(WeaponType::M60Mg),
                armor: ArmorType::Aluminum,
                is_infantry: false,
                cost: 300,
                sight_range: 2,
                has_turret: true,
                rot_degrees_per_tick: 10,
                two_shooter: false,
                is_aircraft: false,
                ammo: 0,
            },
            UnitType::Orca => UnitStats {
                strength: 125,
                max_speed_mph: 40,
                weapon: Some(WeaponType::Dragon),
                armor: ArmorType::Steel,
                is_infantry: false,
                cost: 1200,
                sight_range: 0,
                has_turret: false,
                rot_degrees_per_tick: 4,
                two_shooter: true,
                is_aircraft: true,
                ammo: 6,
            },
            UnitType::Apache => UnitStats {
                strength: 125,
                max_speed_mph: 40,
                weapon: Some(WeaponType::ChainGun),
                armor: ArmorType::Steel,
                is_infantry: false,
                cost: 1200,
                sight_range: 0,
                has_turret: false,
                rot_degrees_per_tick: 4,
                two_shooter: true,
                is_aircraft: true,
                ammo: 15,
            },
        }
    }
}

impl UnitType {
    /// Rules.ini section name from UDATA.CPP / IDATA.CPP / AADATA.CPP.
    pub const fn ini_name(self) -> &'static str {
        match self {
            UnitType::InfantryE1 => "E1",
            UnitType::MediumTank => "MTNK",
            UnitType::MammothTank => "HTNK",
            UnitType::Harvester => "HARV",
            UnitType::Jeep => "JEEP",
            UnitType::Buggy => "BGGY",
            UnitType::Orca => "ORCA",
            UnitType::Apache => "HELI",
        }
    }
}

/// Map UnitType to the engine ClassId used by serialization/UI.
pub const fn unit_class_id(unit_type: UnitType) -> ClassId {
    match unit_type {
        UnitType::InfantryE1 => ClassId::InfantryE1,
        UnitType::MediumTank => ClassId::VehicleE2,
        UnitType::MammothTank => ClassId::VehicleE4,
        UnitType::Harvester => ClassId::VehicleE6,
        UnitType::Orca => ClassId::AircraftE1,
        UnitType::Apache => ClassId::AircraftE2,
        UnitType::Jeep | UnitType::Buggy => ClassId::VehicleE1,
    }
}

/// Reverse of unit_class_id for production completion.
pub fn unit_type_for_class(class_id: ClassId) -> Option<UnitType> {
    match class_id {
        ClassId::InfantryE1 => Some(UnitType::InfantryE1),
        ClassId::VehicleE1 => Some(UnitType::Jeep),
        ClassId::VehicleE2 => Some(UnitType::MediumTank),
        ClassId::VehicleE4 => Some(UnitType::MammothTank),
        ClassId::VehicleE6 => Some(UnitType::Harvester),
        ClassId::AircraftE1 => Some(UnitType::Orca),
        ClassId::AircraftE2 => Some(UnitType::Apache),
        _ => None,
    }
}

/// Structure types ported from BDATA.CPP for the demo bases.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructureType {
    ConstructionYard,
    PowerPlant,
    Refinery,
    Silo,
    Barracks,
    WarFactory,
    Helipad,
    IonCannon,
    TempleOfNod,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProductionKind {
    Infantry,
    Unit,
    Building,
    Aircraft,
}

#[derive(Clone, Copy, Debug)]
pub struct StructureStats {
    pub strength: i32,
    pub cost: i32,
    pub armor: ArmorType,
    pub power: i32,
    pub drain: i32,
    pub sight_range: i32,
    pub produces: Option<ProductionKind>,
    /// BDATA.CPP CAPACITY: tiberium storage capacity.
    pub capacity: i32,
}

impl StructureType {
    pub const fn stats(self) -> StructureStats {
        match self {
            StructureType::ConstructionYard => StructureStats {
                strength: 400,
                cost: 5000,
                armor: ArmorType::Wood,
                power: 30,
                drain: 15,
                sight_range: 3,
                produces: Some(ProductionKind::Building),
                capacity: 0,
            },
            StructureType::PowerPlant => StructureStats {
                strength: 200,
                cost: 300,
                armor: ArmorType::Wood,
                power: 100,
                drain: 0,
                sight_range: 2,
                produces: None,
                capacity: 0,
            },
            StructureType::Refinery => StructureStats {
                strength: 450,
                cost: 2000,
                armor: ArmorType::Wood,
                power: 10,
                drain: 40,
                sight_range: 4,
                produces: None,
                capacity: 1000,
            },
            StructureType::Silo => StructureStats {
                strength: 150,
                cost: 150,
                armor: ArmorType::Wood,
                power: 0,
                drain: 10,
                sight_range: 2,
                produces: None,
                capacity: 1500,
            },
            StructureType::Barracks => StructureStats {
                strength: 400,
                cost: 300,
                armor: ArmorType::Wood,
                power: 0,
                drain: 20,
                sight_range: 3,
                produces: Some(ProductionKind::Infantry),
                capacity: 0,
            },
            StructureType::WarFactory => StructureStats {
                strength: 500,
                cost: 2000,
                armor: ArmorType::Aluminum,
                power: 0,
                drain: 30,
                sight_range: 3,
                produces: Some(ProductionKind::Unit),
                capacity: 0,
            },
            StructureType::Helipad => StructureStats {
                strength: 400,
                cost: 1500,
                armor: ArmorType::Wood,
                power: 0,
                drain: 10,
                sight_range: 3,
                produces: Some(ProductionKind::Aircraft),
                capacity: 0,
            },
            StructureType::IonCannon => StructureStats {
                strength: 500,
                cost: 2800,
                armor: ArmorType::Wood,
                power: 0,
                drain: 200,
                sight_range: 3,
                produces: None,
                capacity: 0,
            },
            StructureType::TempleOfNod => StructureStats {
                strength: 1000,
                cost: 3000,
                armor: ArmorType::Aluminum,
                power: 0,
                drain: 150,
                sight_range: 4,
                produces: None,
                capacity: 0,
            },
        }
    }

    /// Rules.ini section name from BDATA.CPP (short name field).
    ///
    /// Note: TD reuses NUKE for the power plant and PYLE/HAND for the two
    /// barracks; the generic Rust enum maps to the GDI spelling PYLE.
    pub const fn ini_name(self) -> &'static str {
        match self {
            StructureType::ConstructionYard => "FACT",
            StructureType::PowerPlant => "NUKE",
            StructureType::Refinery => "PROC",
            StructureType::Silo => "SILO",
            StructureType::Barracks => "PYLE",
            StructureType::WarFactory => "WEAP",
            StructureType::Helipad => "HPAD",
            StructureType::IonCannon => "EYE",
            StructureType::TempleOfNod => "TMPL",
        }
    }

    pub const fn class_id(self) -> ClassId {
        match self {
            StructureType::ConstructionYard => ClassId::ConstructionYard,
            StructureType::PowerPlant => ClassId::PowerPlant,
            StructureType::Refinery => ClassId::Refinery,
            StructureType::Silo => ClassId::Silo,
            StructureType::Barracks => ClassId::Barracks,
            StructureType::WarFactory => ClassId::WarFactory,
            StructureType::Helipad => ClassId::Helipad,
            StructureType::IonCannon => ClassId::Radar,
            StructureType::TempleOfNod => ClassId::MissileSilo,
        }
    }

    pub const fn from_class_id(class_id: ClassId) -> Option<Self> {
        match class_id {
            ClassId::ConstructionYard => Some(StructureType::ConstructionYard),
            ClassId::PowerPlant => Some(StructureType::PowerPlant),
            ClassId::Refinery => Some(StructureType::Refinery),
            ClassId::Silo => Some(StructureType::Silo),
            ClassId::Barracks => Some(StructureType::Barracks),
            ClassId::WarFactory => Some(StructureType::WarFactory),
            ClassId::Helipad => Some(StructureType::Helipad),
            ClassId::Radar => Some(StructureType::IonCannon),
            ClassId::MissileSilo => Some(StructureType::TempleOfNod),
            _ => None,
        }
    }
}

impl UnitType {
    /// UDATA.CPP secondary weapon column.
    pub const fn secondary_weapon(self) -> Option<WeaponType> {
        match self {
            UnitType::MammothTank => Some(WeaponType::MammothTusk),
            _ => None,
        }
    }
}

impl UnitStats {
    pub fn speed_px_per_sec(self) -> f32 {
        if self.is_infantry {
            mph_to_px_per_sec_infantry(self.max_speed_mph)
        } else {
            mph_to_px_per_sec_vehicle(self.max_speed_mph)
        }
    }

    pub fn health(self) -> Fixed16_16 {
        Fixed16_16::from_int(self.strength)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weapon_rows_match_const_cpp() {
        let m16 = WeaponType::M16.stats();
        assert_eq!(m16.damage, 15);
        assert_eq!(m16.rof_ticks, 20);
        assert_eq!(m16.range_leptons, 0x0200);

        let cannon = WeaponType::Cannon105.stats();
        assert_eq!(cannon.damage, 30);
        assert_eq!(cannon.rof_ticks, 50);
        assert_eq!(cannon.range_leptons, 0x04C0);

        let rifle = WeaponType::Rifle.stats();
        assert_eq!(rifle.damage, 125);
        assert_eq!(rifle.range_leptons, 0x0580);
    }

    #[test]
    fn conversions_match_display_constants() {
        assert_eq!(PIXEL_LEPTON_W, 10);
        assert!((leptons_to_px(0x0200) - 48.0).abs() < f32::EPSILON);
        assert!((mph_to_px_per_sec_infantry(8) - 11.25).abs() < 1.0e-4);
        assert!((mph_to_px_per_sec_vehicle(18) - 27.0).abs() < 1.0e-4);
        assert!((ticks_to_seconds(20) - 1.333_333_4).abs() < 1.0e-5);
    }

    #[test]
    fn modify_damage_matches_combat_cpp() {
        // AP against infantry armor: 0x40 modifier then no distance loss.
        assert_eq!(modify_damage(30, WarheadType::Ap, ArmorType::None, 0), 8);
        // AP against steel is full damage.
        assert_eq!(modify_damage(30, WarheadType::Ap, ArmorType::Steel, 0), 30);
        // Small arms against infantry is full damage.
        assert_eq!(modify_damage(15, WarheadType::Sa, ArmorType::None, 0), 15);
        // Distance attenuation is a right shift by spread factor.
        // 256 leptons >> 6 = 4, then damage >> 4.
        assert_eq!(modify_damage(30, WarheadType::Ap, ArmorType::Steel, 256), 1);
    }

    #[test]
    fn infantry_speeds_are_slower_than_tanks() {
        let infantry = UnitType::InfantryE1.stats();
        let tank = UnitType::MediumTank.stats();
        assert_eq!(infantry.health().to_bits(), 50 * 65536);
        assert_eq!(tank.health().to_bits(), 400 * 65536);
        assert!(infantry.speed_px_per_sec() < tank.speed_px_per_sec());
    }

    #[test]
    fn superweapon_structure_stats_match_bdata() {
        let ion = StructureType::IonCannon.stats();
        assert_eq!((ion.strength, ion.cost, ion.drain), (500, 2800, 200));
        let temple = StructureType::TempleOfNod.stats();
        assert_eq!((temple.strength, temple.cost, temple.drain), (1000, 3000, 150));
    }

    #[test]
    fn bullet_flight_flags_match_bbdata() {
        assert!(BulletType::Tow.is_homing());
        assert!(BulletType::Tow.is_fueled());
        assert!(BulletType::Ssm.is_homing());
        assert!(BulletType::Grenade.is_arcing());
        assert!(BulletType::He.is_arcing());
        assert!(!BulletType::Apds.is_arcing());
        assert!(!BulletType::Bullet.is_homing());
        assert!(!BulletType::Bullet.is_fueled());
    }

    #[test]
    fn aircraft_stats_match_aadata() {
        let orca = UnitType::Orca.stats();
        assert_eq!(orca.strength, 125);
        assert_eq!(orca.cost, 1200);
        assert_eq!(orca.ammo, 6);
        assert_eq!(orca.weapon, Some(WeaponType::Dragon));
        assert_eq!(orca.armor, ArmorType::Steel);
        assert!(orca.is_aircraft);
        assert!((orca.speed_px_per_sec() - 60.0).abs() < 1.0e-4);

        let apache = UnitType::Apache.stats();
        assert_eq!(apache.ammo, 15);
        assert_eq!(apache.weapon, Some(WeaponType::ChainGun));
        assert!(apache.is_aircraft);
    }

    #[test]
    fn harvester_stats_match_udata() {
        let stats = UnitType::Harvester.stats();
        assert_eq!(stats.strength, 600);
        assert_eq!(stats.max_speed_mph, 12);
        assert_eq!(stats.weapon, None);
        assert_eq!(stats.armor, ArmorType::Aluminum);
        assert_eq!(stats.cost, 1400);
        assert_eq!(HARVESTER_STEP_COUNT, 28);
        assert_eq!(HARVESTER_STEP_CREDITS, 25);
        assert_eq!(HARVESTER_FULL_LOAD, 700);
    }

    #[test]
    fn production_time_scales_with_power_fraction() {
        // Full power: cost ticks / 15.
        assert!((production_seconds(100, 140, 105) - 100.0 / 15.0).abs() < 1.0e-4);
        // Low power fraction (< 0x80): 256/103 slowdown.
        assert!((production_seconds(100, 40, 95) - 100.0 * 256.0 / 103.0 / 15.0).abs() < 1.0e-4);
        // No power at all: 256/64 = 4x slowdown.
        assert!((production_seconds(100, 0, 95) - 100.0 * 4.0 / 15.0).abs() < 1.0e-4);
    }

    #[test]
    fn structure_stats_match_bdata() {
        let yard = StructureType::ConstructionYard.stats();
        assert_eq!((yard.strength, yard.cost, yard.power, yard.drain), (400, 5000, 30, 15));
        let power = StructureType::PowerPlant.stats();
        assert_eq!((power.strength, power.cost, power.power, power.drain), (200, 300, 100, 0));
        let refinery = StructureType::Refinery.stats();
        assert_eq!((refinery.strength, refinery.cost, refinery.power, refinery.drain), (450, 2000, 10, 40));
        let barracks = StructureType::Barracks.stats();
        assert_eq!((barracks.strength, barracks.cost, barracks.power, barracks.drain), (400, 300, 0, 20));
        let factory = StructureType::WarFactory.stats();
        assert_eq!((factory.strength, factory.cost, factory.power, factory.drain), (500, 2000, 0, 30));
    }

    #[test]
    fn mammoth_tank_stats_match_udata() {
        let stats = UnitType::MammothTank.stats();
        assert_eq!(stats.strength, 600);
        assert_eq!(stats.max_speed_mph, 12);
        assert_eq!(stats.weapon, Some(WeaponType::Cannon120));
        assert_eq!(stats.armor, ArmorType::Steel);
        assert!(stats.has_turret);
        assert!(stats.two_shooter);
        assert_eq!(stats.rot_degrees_per_tick, 5);
        assert!((stats.speed_px_per_sec() - 18.0).abs() < 1.0e-4);
    }
}
