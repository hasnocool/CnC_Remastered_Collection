//! Class ID system (Westwood RTTI replacement)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Class ID - replaces C++ RTTI
/// Each game object type gets a unique ID
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Default)]
// `AA_Gun` intentionally mirrors the Westwood 3-letter INI shorthand; renaming
// would break the C++ compatible naming scheme.
#[allow(non_camel_case_types)]
pub enum ClassId {
    // Base classes
    Abstract = 0,
    Object = 1,

    // House/Faction
    House = 100,
    HouseGDI = 101,
    HouseNod = 102,
    HouseAllies = 103,
    HouseSoviet = 104,

    // Units
    Unit = 200,
    Infantry = 201,
    Vehicle = 202,
    Aircraft = 203,
    Ship = 204,

    // Buildings
    Building = 300,
    Factory = 301,
    PowerPlant = 302,
    Refinery = 303,
    Barracks = 304,
    WarFactory = 305,
    Helipad = 306,
    Airfield = 307,
    NavalYard = 308,
    SubPen = 309,
    ConstructionYard = 310,
    Radar = 311,
    Silo = 312,
    BioLab = 313,
    TechCenter = 314,
    MissileSilo = 315,
    AA_Gun = 316,
    TeslaCoil = 317,
    GapGenerator = 318,
    IronCurtain = 319,
    Chronosphere = 320,
    GrandCannon = 321,

    // Infantry types
    InfantryE1 = 400,  // Rifleman
    InfantryE2 = 401,  // Grenadier
    InfantryE3 = 402,  // Flamethrower
    InfantryE4 = 403,  // Rocket Soldier
    InfantryE5 = 404,  // Chem Warrior
    InfantryE6 = 405,  // Engineer
    InfantryE7 = 406,  // Medic
    InfantryE8 = 407,  // Spy
    InfantryE9 = 408,  // Thief
    InfantryE10 = 409, // Commando
    InfantryE11 = 410, // Tanya

    // Vehicle types
    VehicleE1 = 500,  // Light Tank
    VehicleE2 = 501,  // Medium Tank
    VehicleE3 = 502,  // Heavy Tank
    VehicleE4 = 503,  // Mammoth Tank
    VehicleE5 = 504,  // APC
    VehicleE6 = 505,  // Harvester
    VehicleE7 = 506,  // MCV
    VehicleE8 = 507,  // Ranger
    VehicleE9 = 508,  // Humvee
    VehicleE10 = 509, // MLRS
    VehicleE11 = 510, // Artillery
    VehicleE12 = 511, // V2 Rocket
    VehicleE13 = 512, // Demo Truck
    VehicleE14 = 513, // Tesla Tank

    // Aircraft types
    AircraftE1 = 600, // Helicopter
    AircraftE2 = 601, // Transport Heli
    AircraftE3 = 602, // Fighter
    AircraftE4 = 603, // Bomber
    AircraftE5 = 604, // A-10
    AircraftE6 = 605, // Yak
    AircraftE7 = 606, // MiG
    AircraftE8 = 607, // Chinook

    // Ship types
    ShipE1 = 700, // Gunboat
    ShipE2 = 701, // Destroyer
    ShipE3 = 702, // Cruiser
    ShipE4 = 703, // Submarine
    ShipE5 = 704, // Transport
    ShipE6 = 705, // Carrier

    // Projectiles/Weapons
    Projectile = 800,
    Bullet = 801,
    Rocket = 802,
    Shell = 803,
    Grenade = 804,
    Flame = 805,
    TeslaBolt = 806,
    Nuke = 807,
    IonBeam = 808,

    // Special effects
    Animation = 900,
    Particle = 901,
    Smoke = 902,
    Fire = 903,
    Explosion = 904,
    Tiberium = 905,
    Crate = 906,

    // Map/Cell
    Cell = 1000,
    CellGrid = 1001,
    Map = 1002,

    // AI/Teams
    Team = 1100,
    TeamType = 1101,
    Script = 1102,
    Trigger = 1103,
    TaskForce = 1104,

    // Network
    NetPacket = 1200,
    NetCommand = 1201,

    // Unknown/Invalid
    #[default]
    Unknown = u32::MAX,
}

impl ClassId {
    /// Get class ID from u32
    pub fn from_u32(id: u32) -> Self {
        // SAFETY: ClassId is repr(u32) so this is valid for known values
        // Unknown values become Unknown
        match id {
            0 => ClassId::Abstract,
            1 => ClassId::Object,
            100 => ClassId::House,
            101 => ClassId::HouseGDI,
            102 => ClassId::HouseNod,
            103 => ClassId::HouseAllies,
            104 => ClassId::HouseSoviet,
            200 => ClassId::Unit,
            201 => ClassId::Infantry,
            202 => ClassId::Vehicle,
            203 => ClassId::Aircraft,
            204 => ClassId::Ship,
            300 => ClassId::Building,
            301 => ClassId::Factory,
            302 => ClassId::PowerPlant,
            303 => ClassId::Refinery,
            304 => ClassId::Barracks,
            305 => ClassId::WarFactory,
            306 => ClassId::Helipad,
            307 => ClassId::Airfield,
            308 => ClassId::NavalYard,
            309 => ClassId::SubPen,
            310 => ClassId::ConstructionYard,
            311 => ClassId::Radar,
            312 => ClassId::Silo,
            313 => ClassId::BioLab,
            314 => ClassId::TechCenter,
            315 => ClassId::MissileSilo,
            316 => ClassId::AA_Gun,
            317 => ClassId::TeslaCoil,
            318 => ClassId::GapGenerator,
            319 => ClassId::IronCurtain,
            320 => ClassId::Chronosphere,
            321 => ClassId::GrandCannon,
            400..=410 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            500..=514 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            600..=607 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            700..=705 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            800..=808 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            900..=906 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            1000..=1002 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            1100..=1104 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            1200..=1201 => unsafe { std::mem::transmute::<u32, ClassId>(id) },
            _ => ClassId::Unknown,
        }
    }

    /// Get raw u32 value
    #[inline]
    pub fn as_u32(self) -> u32 {
        self as u32
    }

    /// Check if this is a unit type
    pub fn is_unit(self) -> bool {
        matches!(
            self,
            ClassId::Unit
                | ClassId::Infantry
                | ClassId::Vehicle
                | ClassId::Aircraft
                | ClassId::Ship
                | ClassId::InfantryE1
                | ClassId::InfantryE2
                | ClassId::InfantryE3
                | ClassId::InfantryE4
                | ClassId::InfantryE5
                | ClassId::InfantryE6
                | ClassId::InfantryE7
                | ClassId::InfantryE8
                | ClassId::InfantryE9
                | ClassId::InfantryE10
                | ClassId::InfantryE11
                | ClassId::VehicleE1
                | ClassId::VehicleE2
                | ClassId::VehicleE3
                | ClassId::VehicleE4
                | ClassId::VehicleE5
                | ClassId::VehicleE6
                | ClassId::VehicleE7
                | ClassId::VehicleE8
                | ClassId::VehicleE9
                | ClassId::VehicleE10
                | ClassId::VehicleE11
                | ClassId::VehicleE12
                | ClassId::VehicleE13
                | ClassId::VehicleE14
                | ClassId::AircraftE1
                | ClassId::AircraftE2
                | ClassId::AircraftE3
                | ClassId::AircraftE4
                | ClassId::AircraftE5
                | ClassId::AircraftE6
                | ClassId::AircraftE7
                | ClassId::AircraftE8
                | ClassId::ShipE1
                | ClassId::ShipE2
                | ClassId::ShipE3
                | ClassId::ShipE4
                | ClassId::ShipE5
                | ClassId::ShipE6
        )
    }

    /// Check if this is a building type
    pub fn is_building(self) -> bool {
        matches!(
            self,
            ClassId::Building
                | ClassId::Factory
                | ClassId::PowerPlant
                | ClassId::Refinery
                | ClassId::Barracks
                | ClassId::WarFactory
                | ClassId::Helipad
                | ClassId::Airfield
                | ClassId::NavalYard
                | ClassId::SubPen
                | ClassId::ConstructionYard
                | ClassId::Radar
                | ClassId::Silo
                | ClassId::BioLab
                | ClassId::TechCenter
                | ClassId::MissileSilo
                | ClassId::AA_Gun
                | ClassId::TeslaCoil
                | ClassId::GapGenerator
                | ClassId::IronCurtain
                | ClassId::Chronosphere
                | ClassId::GrandCannon
        )
    }

    /// Check if this is a house/faction
    pub fn is_house(self) -> bool {
        matches!(
            self,
            ClassId::House
                | ClassId::HouseGDI
                | ClassId::HouseNod
                | ClassId::HouseAllies
                | ClassId::HouseSoviet
        )
    }

    /// Check if this is a projectile
    pub fn is_projectile(self) -> bool {
        matches!(
            self,
            ClassId::Projectile
                | ClassId::Bullet
                | ClassId::Rocket
                | ClassId::Shell
                | ClassId::Grenade
                | ClassId::Flame
                | ClassId::TeslaBolt
                | ClassId::Nuke
                | ClassId::IonBeam
        )
    }

    /// Get category name
    pub fn category(self) -> &'static str {
        match self.as_u32() {
            0..=1 => "Base",
            100..=199 => "House",
            200..=299 => "Unit",
            300..=399 => "Building",
            400..=499 => "Infantry",
            500..=599 => "Vehicle",
            600..=699 => "Aircraft",
            700..=799 => "Ship",
            800..=899 => "Projectile",
            900..=999 => "Effect",
            1000..=1099 => "Map",
            1100..=1199 => "AI",
            1200..=1299 => "Network",
            _ => "Unknown",
        }
    }
}

impl fmt::Display for ClassId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self, self.as_u32())
    }
}

/// Registry for class information (from INI files)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ClassRegistry {
    /// Name to ClassId mapping
    name_to_id: HashMap<String, ClassId>,
    /// ClassId to name mapping
    id_to_name: HashMap<ClassId, String>,
    /// ClassId to parent ClassId (for inheritance)
    parent: HashMap<ClassId, ClassId>,
    /// Class-specific data
    data: HashMap<ClassId, ClassData>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ClassData {
    pub name: String,
    pub prerequisite: Vec<ClassId>,
    pub buildable: bool,
    pub cost: i32,
    pub build_time: i32,
    pub owner: Vec<ClassId>, // Which houses can build this
    pub custom: HashMap<String, String>,
}

impl ClassRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a class
    pub fn register(&mut self, id: ClassId, name: String, parent: Option<ClassId>) {
        self.name_to_id.insert(name.clone(), id);
        self.id_to_name.insert(id, name);
        if let Some(p) = parent {
            self.parent.insert(id, p);
        }
    }

    /// Get ClassId by name
    pub fn get_id(&self, name: &str) -> Option<ClassId> {
        self.name_to_id.get(name).copied()
    }

    /// Get name by ClassId
    pub fn get_name(&self, id: ClassId) -> Option<&String> {
        self.id_to_name.get(&id)
    }

    /// Get parent class
    pub fn get_parent(&self, id: ClassId) -> Option<ClassId> {
        self.parent.get(&id).copied()
    }

    /// Check if class inherits from another
    pub fn inherits_from(&self, child: ClassId, ancestor: ClassId) -> bool {
        let mut current = Some(child);
        while let Some(c) = current {
            if c == ancestor {
                return true;
            }
            current = self.get_parent(c);
        }
        false
    }

    /// Set class data
    pub fn set_data(&mut self, id: ClassId, data: ClassData) {
        self.data.insert(id, data);
    }

    /// Get class data
    pub fn get_data(&self, id: ClassId) -> Option<&ClassData> {
        self.data.get(&id)
    }

    /// Load from INI (Rules.ini style)
    pub fn load_from_ini(&mut self, ini: &crate::ini::IniFile) -> Result<(), String> {
        for (name, section) in &ini.sections {
            // Parse class ID from section name or value
            let id = if let Some(id_str) = section.get_string("ID") {
                if let Ok(num) = id_str.parse::<u32>() {
                    ClassId::from_u32(num)
                } else {
                    continue;
                }
            } else {
                continue;
            };

            let parent = section.get_string("Parent").and_then(|s| self.get_id(s));
            self.register(id, name.clone(), parent);

            // Load class data
            let mut data = ClassData {
                name: name.clone(),
                buildable: section.get_bool("Buildable").unwrap_or(false),
                cost: section.get_i64("Cost").unwrap_or(0) as i32,
                build_time: section.get_i64("BuildTime").unwrap_or(0) as i32,
                ..Default::default()
            };

            if let Some(owner_str) = section.get_string("Owner") {
                for owner in owner_str.split(',') {
                    if let Some(owner_id) = self.get_id(owner.trim()) {
                        data.owner.push(owner_id);
                    }
                }
            }

            if let Some(prereq_str) = section.get_string("Prerequisite") {
                for prereq in prereq_str.split(',') {
                    if let Some(prereq_id) = self.get_id(prereq.trim()) {
                        data.prerequisite.push(prereq_id);
                    }
                }
            }

            // Custom values
            for (key, value) in &section.values {
                if let Some(s) = value.as_string() {
                    data.custom.insert(key.clone(), s.to_string());
                }
            }

            self.set_data(id, data);
        }
        Ok(())
    }

    /// Get global registry (thread-safe singleton)
    pub fn global() -> &'static ClassRegistry {
        static REGISTRY: OnceLock<ClassRegistry> = OnceLock::new();
        REGISTRY.get_or_init(ClassRegistry::new)
    }
}

impl fmt::Display for ClassRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ClassRegistry({} classes)", self.name_to_id.len())
    }
}

use std::fmt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_id() {
        assert_eq!(ClassId::Infantry.as_u32(), 201);
        assert_eq!(ClassId::from_u32(201), ClassId::Infantry);
        assert!(ClassId::Infantry.is_unit());
        assert!(!ClassId::Infantry.is_building());
        assert_eq!(ClassId::Infantry.category(), "Unit");
    }

    #[test]
    fn test_class_id_unknown() {
        assert_eq!(ClassId::from_u32(9999), ClassId::Unknown);
    }

    #[test]
    fn test_registry() {
        let mut registry = ClassRegistry::new();
        registry.register(
            ClassId::InfantryE1,
            "E1".to_string(),
            Some(ClassId::Infantry),
        );

        assert_eq!(registry.get_id("E1"), Some(ClassId::InfantryE1));
        assert_eq!(
            registry.get_name(ClassId::InfantryE1),
            Some(&"E1".to_string())
        );
        assert_eq!(
            registry.get_parent(ClassId::InfantryE1),
            Some(ClassId::Infantry)
        );
        assert!(registry.inherits_from(ClassId::InfantryE1, ClassId::Infantry));
        assert!(!registry.inherits_from(ClassId::InfantryE1, ClassId::Vehicle));
    }
}
