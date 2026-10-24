//! Data-driven Rules.ini registry (plan Phase 5.2).
//!
//! The original engines load global balance data from Rules.ini at startup:
//! [General] AI tuning, the [Easy]/[Normal]/[Hard] difficulty biases and the
//! per-type weapon/warhead/unit/building sections. This module parses that file
//! into typed Rust structures that both game crates can share.
//!
//! Defaults mirror the constructors in TIBERIANDAWN/RULES.CPP, so a missing or
//! partial Rules.ini still yields the original engine values.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::ini::{IniFile, IniSection, IniValue};

fn get_i32(section: &IniSection, key: &str) -> Option<i32> {
    section
        .get_ci(key)
        .and_then(|value| value.as_i64().ok())
        .map(|value| value as i32)
}

fn get_f64(section: &IniSection, key: &str) -> Option<f64> {
    section.get_ci(key).and_then(|value| value.as_f64().ok())
}

fn get_bool(section: &IniSection, key: &str) -> Option<bool> {
    section.get_ci(key).and_then(|value| value.as_bool().ok())
}

fn get_string(section: &IniSection, key: &str) -> Option<String> {
    match section.get_ci(key)? {
        IniValue::String(value) => Some(value.clone()),
        IniValue::Integer(value) => Some(value.to_string()),
        IniValue::Float(value) => Some(value.to_string()),
        IniValue::Boolean(value) => Some(value.to_string()),
        IniValue::Array(values) => Some(
            values
                .iter()
                .filter_map(|value| value.as_string().map(str::to_string))
                .collect::<Vec<_>>()
                .join(","),
        ),
    }
}

/// Comma-separated or array-valued string list.
fn get_string_list(section: &IniSection, key: &str) -> Vec<String> {
    match section.get_ci(key) {
        Some(IniValue::Array(values)) => values
            .iter()
            .filter_map(|value| value.as_string().map(|text| text.trim().to_string()))
            .filter(|text| !text.is_empty())
            .collect(),
        Some(IniValue::String(value)) => value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(str::to_string)
            .collect(),
        _ => Vec::new(),
    }
}

/// General section AI and economy tuning.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralRules {
    pub base_size_add: i32,
    pub power_surplus: i32,
    pub attack_interval: i32,
    pub attack_delay: i32,
    pub power_emergency_fraction: u16,
    pub helipad_ratio: u16,
    pub helipad_limit: i32,
    pub tesla_ratio: u16,
    pub tesla_limit: i32,
    pub aa_ratio: u16,
    pub aa_limit: i32,
    pub defense_ratio: u16,
    pub defense_limit: i32,
    pub war_ratio: u16,
    pub war_limit: i32,
    pub barracks_ratio: u16,
    pub barracks_limit: i32,
    pub refinery_ratio: u16,
    pub refinery_limit: i32,
    pub airstrip_ratio: u16,
    pub airstrip_limit: i32,
    pub max_iq: u8,
    pub iq_super_weapons: u8,
    pub iq_production: u8,
    pub iq_guard_area: u8,
    pub iq_repair_sell: u8,
    pub iq_crush: u8,
    pub iq_scatter: u8,
    pub iq_content_scan: u8,
    pub iq_aircraft: u8,
    pub iq_harvester: u8,
    pub iq_sell_back: u8,
    pub infantry_reserve: i32,
    pub infantry_base_mult: i32,
    pub is_computer_paranoid: bool,
    pub allow_super_weapons: bool,
}

impl Default for GeneralRules {
    fn default() -> Self {
        // TIBERIANDAWN/RULES.CPP RulesClass::RulesClass.
        Self {
            base_size_add: 3,
            power_surplus: 50,
            attack_interval: 3,
            attack_delay: 5,
            power_emergency_fraction: 0x00C0,
            helipad_ratio: 0x1E,
            helipad_limit: 5,
            tesla_ratio: 0,
            tesla_limit: 0,
            aa_ratio: 0x24,
            aa_limit: 10,
            defense_ratio: 0x66,
            defense_limit: 40,
            war_ratio: 0x19,
            war_limit: 2,
            barracks_ratio: 0x28,
            barracks_limit: 2,
            refinery_ratio: 0x28,
            refinery_limit: 4,
            airstrip_ratio: 0x1E,
            airstrip_limit: 5,
            max_iq: 5,
            iq_super_weapons: 4,
            iq_production: 5,
            iq_guard_area: 4,
            iq_repair_sell: 1,
            iq_crush: 2,
            iq_scatter: 3,
            iq_content_scan: 4,
            iq_aircraft: 4,
            iq_harvester: 2,
            iq_sell_back: 2,
            infantry_reserve: 3000,
            infantry_base_mult: 1,
            is_computer_paranoid: true,
            allow_super_weapons: true,
        }
    }
}

impl GeneralRules {
    pub fn from_section(section: &IniSection) -> Self {
        let defaults = Self::default();
        let read = |key: &str, fallback: i32| get_i32(section, key).unwrap_or(fallback);
        let read_u16 = |key: &str, fallback: u16| {
            get_i32(section, key)
                .map(|value| value.clamp(0, u16::MAX as i32) as u16)
                .unwrap_or(fallback)
        };
        let read_u8 = |key: &str, fallback: u8| {
            get_i32(section, key)
                .map(|value| value.clamp(0, u8::MAX as i32) as u8)
                .unwrap_or(fallback)
        };
        Self {
            base_size_add: read("BaseSizeAdd", defaults.base_size_add),
            power_surplus: read("PowerSurplus", defaults.power_surplus),
            attack_interval: read("AttackInterval", defaults.attack_interval),
            attack_delay: read("AttackDelay", defaults.attack_delay),
            power_emergency_fraction: read_u16(
                "PowerEmergencyFraction",
                defaults.power_emergency_fraction,
            ),
            helipad_ratio: read_u16("HelipadRatio", defaults.helipad_ratio),
            helipad_limit: read("HelipadLimit", defaults.helipad_limit),
            tesla_ratio: read_u16("TeslaRatio", defaults.tesla_ratio),
            tesla_limit: read("TeslaLimit", defaults.tesla_limit),
            aa_ratio: read_u16("AARatio", defaults.aa_ratio),
            aa_limit: read("AALimit", defaults.aa_limit),
            defense_ratio: read_u16("DefenseRatio", defaults.defense_ratio),
            defense_limit: read("DefenseLimit", defaults.defense_limit),
            war_ratio: read_u16("WarRatio", defaults.war_ratio),
            war_limit: read("WarLimit", defaults.war_limit),
            barracks_ratio: read_u16("BarracksRatio", defaults.barracks_ratio),
            barracks_limit: read("BarracksLimit", defaults.barracks_limit),
            refinery_ratio: read_u16("RefineryRatio", defaults.refinery_ratio),
            refinery_limit: read("RefineryLimit", defaults.refinery_limit),
            airstrip_ratio: read_u16("AirstripRatio", defaults.airstrip_ratio),
            airstrip_limit: read("AirstripLimit", defaults.airstrip_limit),
            max_iq: read_u8("MaxIQ", defaults.max_iq),
            iq_super_weapons: read_u8("IQSuperWeapons", defaults.iq_super_weapons),
            iq_production: read_u8("IQProduction", defaults.iq_production),
            iq_guard_area: read_u8("IQGuardArea", defaults.iq_guard_area),
            iq_repair_sell: read_u8("IQRepairSell", defaults.iq_repair_sell),
            iq_crush: read_u8("IQCrush", defaults.iq_crush),
            iq_scatter: read_u8("IQScatter", defaults.iq_scatter),
            iq_content_scan: read_u8("IQContentScan", defaults.iq_content_scan),
            iq_aircraft: read_u8("IQAircraft", defaults.iq_aircraft),
            iq_harvester: read_u8("IQHarvester", defaults.iq_harvester),
            iq_sell_back: read_u8("IQSellBack", defaults.iq_sell_back),
            infantry_reserve: read("InfantryReserve", defaults.infantry_reserve),
            infantry_base_mult: read("InfantryBaseMult", defaults.infantry_base_mult),
            is_computer_paranoid: get_bool(section, "IsComputerParanoid")
                .unwrap_or(defaults.is_computer_paranoid),
            allow_super_weapons: get_bool(section, "AllowSuperWeapons")
                .unwrap_or(defaults.allow_super_weapons),
        }
    }
}

/// Difficulty selector used by the AI and combat biases.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Easy,
    #[default]
    Normal,
    Hard,
}

impl DifficultyLevel {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "easy" => Some(Self::Easy),
            "normal" | "medium" => Some(Self::Normal),
            "hard" | "difficult" => Some(Self::Hard),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
        }
    }
}

/// Per-difficulty combat and production biases (RULES.H DifficultyClass).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DifficultyRule {
    pub firepower_bias: f64,
    pub groundspeed_bias: f64,
    pub airspeed_bias: f64,
    pub armor_bias: f64,
    pub rof_bias: f64,
    pub cost_bias: f64,
    pub build_speed_bias: f64,
    pub repair_delay: f64,
    pub build_delay: f64,
    pub is_build_slowdown: bool,
    pub is_wall_destroyer: bool,
    pub is_content_scan: bool,
}

impl Default for DifficultyRule {
    fn default() -> Self {
        // TIBERIANDAWN/RULES.CPP DifficultyClass::DifficultyClass.
        Self {
            firepower_bias: 1.0,
            groundspeed_bias: 1.0,
            airspeed_bias: 1.0,
            armor_bias: 1.0,
            rof_bias: 1.0,
            cost_bias: 1.0,
            build_speed_bias: 1.0,
            repair_delay: 0.02,
            build_delay: 0.03,
            is_build_slowdown: false,
            is_wall_destroyer: true,
            is_content_scan: false,
        }
    }
}

impl DifficultyRule {
    pub fn from_section(section: &IniSection) -> Self {
        let defaults = Self::default();
        let read = |key: &str, fallback: f64| get_f64(section, key).unwrap_or(fallback);
        Self {
            firepower_bias: read("FirepowerBias", defaults.firepower_bias),
            groundspeed_bias: read("GroundspeedBias", defaults.groundspeed_bias),
            airspeed_bias: read("AirspeedBias", defaults.airspeed_bias),
            armor_bias: read("ArmorBias", defaults.armor_bias),
            rof_bias: read("ROFBias", defaults.rof_bias),
            cost_bias: read("CostBias", defaults.cost_bias),
            build_speed_bias: read("BuildSpeedBias", defaults.build_speed_bias),
            repair_delay: read("RepairDelay", defaults.repair_delay),
            build_delay: read("BuildDelay", defaults.build_delay),
            is_build_slowdown: get_bool(section, "IsBuildSlowdown")
                .unwrap_or(defaults.is_build_slowdown),
            is_wall_destroyer: get_bool(section, "IsWallDestroyer")
                .unwrap_or(defaults.is_wall_destroyer),
            is_content_scan: get_bool(section, "IsContentScan").unwrap_or(defaults.is_content_scan),
        }
    }
}

/// Special gameplay flags consumed by both engine AI systems.
/// Mirrors the RA Rules.ini / scenario options: build threshold (tech cap),
/// shroud-off (fog disabled), one-tick strikes, and game-speed modifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SpecialFlags {
    /// Maximum buildable tech level (0 = no cap). Matches `BuildLevel`.
    pub build_threshold: u8,
    /// Shroud / fog disabled (`IsShadowGrow` / `ShroudOff`).
    pub shroud_off: bool,
    /// One-tick strikes enabled (instant attack execution).
    pub one_tick_strikes: bool,
    /// Game speed modifier (0 = default, positive = faster, negative = slower).
    pub game_speed: i8,
}

/// Per-faction BuildChoice doctrine table (replaces generic production weights).
/// GDI = balanced infantry + medium tanks; NOD = stealth + bikes; Soviet = heavy tanks;
/// Allied = infantry + light armor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BuildChoice {
    /// Weight multiplier for infantry production in this doctrine.
    pub infantry_weight: u32,
    /// Weight multiplier for medium/heavy tank production.
    pub tank_weight: u32,
    /// Weight multiplier for aircraft production.
    pub air_weight: u32,
}

impl BuildChoice {
    pub fn gdi() -> Self {
        Self {
            infantry_weight: 40,
            tank_weight: 30,
            air_weight: 14,
        }
    }
    pub fn nod() -> Self {
        Self {
            infantry_weight: 40,
            tank_weight: 28,
            air_weight: 14,
        }
    }
    pub fn soviet() -> Self {
        Self {
            infantry_weight: 35,
            tank_weight: 35,
            air_weight: 10,
        }
    }
    pub fn allied() -> Self {
        Self {
            infantry_weight: 45,
            tank_weight: 25,
            air_weight: 12,
        }
    }
}

/// One weapon section, e.g. [Cannon105] in Rules.ini / CONST.CPP.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeaponRule {
    pub name: String,
    pub damage: i32,
    pub rof: i32,
    pub range: i32,
    pub speed: i32,
    pub burst: i32,
    pub projectile: Option<String>,
    pub warhead: Option<String>,
    pub report: Option<String>,
    pub anim: Option<String>,
    pub is_arcing: bool,
    pub is_high: bool,
    pub is_invisible: bool,
}

impl WeaponRule {
    pub fn from_section(name: &str, section: &IniSection) -> Self {
        Self {
            name: name.to_string(),
            damage: get_i32(section, "Damage").unwrap_or(0),
            rof: get_i32(section, "ROF").unwrap_or(0),
            range: get_i32(section, "Range").unwrap_or(0),
            speed: get_i32(section, "Speed").unwrap_or(0),
            burst: get_i32(section, "Burst").unwrap_or(1),
            projectile: get_string(section, "Projectile"),
            warhead: get_string(section, "Warhead"),
            report: get_string(section, "Report"),
            anim: get_string(section, "Anim"),
            is_arcing: get_bool(section, "IsArcing").unwrap_or(false),
            is_high: get_bool(section, "IsHigh").unwrap_or(false),
            is_invisible: get_bool(section, "IsInvisible").unwrap_or(false),
        }
    }

    /// CONST.CPP stores Range in leptons; the balance tables use map cells.
    pub fn range_cells(&self) -> i32 {
        self.range / 256
    }
}

/// One warhead section, e.g. [AP] in Rules.ini / CONST.CPP.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarheadRule {
    pub name: String,
    pub spread: i32,
    pub wall: bool,
    pub destroy_walls: bool,
    pub destroy_wood: bool,
    pub verses: Vec<u8>,
}

impl WarheadRule {
    pub fn from_section(name: &str, section: &IniSection) -> Self {
        // Rules.ini writes percentages ("50%"). A bare `as_i64()` rejects
        // those, which silently dropped the column and shifted every later
        // armor class, so strip the percent sign first.
        fn verse_value(value: &IniValue) -> Option<u8> {
            if let Ok(number) = value.as_i64() {
                return Some(number.clamp(0, 255) as u8);
            }
            let text = value.as_string()?.trim().trim_end_matches('%').trim();
            text.parse::<i64>()
                .ok()
                .map(|number| number.clamp(0, 255) as u8)
        }

        let verses = match section.get_ci("Verses") {
            Some(IniValue::Array(values)) => values.iter().filter_map(verse_value).collect(),
            Some(value) => verse_value(value).map(|v| vec![v]).unwrap_or_default(),
            None => Vec::new(),
        };
        Self {
            name: name.to_string(),
            spread: get_i32(section, "Spread").unwrap_or(0),
            wall: get_bool(section, "Wall").unwrap_or(false),
            destroy_walls: get_bool(section, "DestroyWalls").unwrap_or(false),
            destroy_wood: get_bool(section, "DestroyWood").unwrap_or(false),
            verses,
        }
    }

    /// CONQUER/COMBAT damage percentage against one armor class.
    pub fn verses_percent(&self, armor: usize) -> u8 {
        self.verses.get(armor).copied().unwrap_or(100)
    }
}

/// Generic unit/building/aircraft type definition overridden by Rules.ini.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeRule {
    pub name: String,
    pub cost: Option<i32>,
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub sight: Option<i32>,
    pub armor: Option<String>,
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub prerequisite: Vec<String>,
    pub owner: Vec<String>,
    pub tech_level: Option<i32>,
    pub display_name: Option<String>,
    pub raw: BTreeMap<String, IniValue>,
}

impl TypeRule {
    pub fn from_section(name: &str, section: &IniSection) -> Self {
        Self {
            name: name.to_string(),
            cost: get_i32(section, "Cost"),
            strength: get_i32(section, "Strength"),
            speed: get_i32(section, "Speed"),
            sight: get_i32(section, "Sight"),
            armor: get_string(section, "Armor"),
            primary: get_string(section, "Primary"),
            secondary: get_string(section, "Secondary"),
            prerequisite: get_string_list(section, "Prerequisite"),
            owner: get_string_list(section, "Owner"),
            tech_level: get_i32(section, "TechLevel"),
            display_name: get_string(section, "Name"),
            raw: section.values.clone(),
        }
    }

    /// Whether this rule looks like a playable object definition rather than a
    /// list/meta section.
    pub fn looks_like_type(section: &IniSection) -> bool {
        [
            "Cost",
            "Strength",
            "Armor",
            "Primary",
            "Secondary",
            "Speed",
            "Sight",
            "TechLevel",
            "Prerequisite",
            "Owner",
        ]
        .iter()
        .any(|key| section.get_ci(key).is_some())
    }
}

/// House/faction definition from a section listed in [Houses].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HouseRule {
    pub name: String,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub raw: BTreeMap<String, IniValue>,
}

impl HouseRule {
    pub fn from_section(name: &str, section: &IniSection) -> Self {
        Self {
            name: name.to_string(),
            display_name: get_string(section, "Name"),
            color: get_string(section, "Color"),
            raw: section.values.clone(),
        }
    }
}

/// Sections that describe the file itself rather than a playable type.
const META_SECTIONS: &[&str] = &[
    "General",
    "Basic",
    "Weapons",
    "Warheads",
    "Houses",
    "Countries",
    "Colors",
    "Easy",
    "Normal",
    "Hard",
    "Difficult",
    "Multiplayer",
    "AI",
    "Sounds",
    "Movies",
    "Smudge",
    "Tiberium",
    "Infantry",
    "Units",
    "Aircraft",
    "Buildings",
    "Structures",
];

/// Fully parsed Rules.ini database shared by the game crates.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RulesDatabase {
    pub general: GeneralRules,
    pub easy: DifficultyRule,
    pub normal: DifficultyRule,
    pub hard: DifficultyRule,
    pub weapons: BTreeMap<String, WeaponRule>,
    pub warheads: BTreeMap<String, WarheadRule>,
    pub types: BTreeMap<String, TypeRule>,
    pub houses: BTreeMap<String, HouseRule>,
    /// Category name (Infantry/Units/Aircraft/Buildings) to type names.
    pub categories: BTreeMap<String, Vec<String>>,
    /// Non-fatal problems found while loading (missing referenced sections).
    pub warnings: Vec<String>,
    /// SPECIAL.CPP-style gameplay toggles (build threshold, shroud-off,
    /// one-tick strikes, speed modifier) shared by both engines (todo 36).
    pub special_flags: SpecialFlags,
}

impl RulesDatabase {
    // Retained as an inherent method: ~15 external call sites use
    // `RulesDatabase::from_str(...)` path-style, which would break if this
    // became a `std::str::FromStr` trait impl without importing the trait.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(input: &str) -> Result<Self> {
        let ini = IniFile::parse(input).context("failed to parse Rules.ini")?;
        Ok(Self::from_ini(&ini))
    }

    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read rules file {:?}", path))?;
        Self::from_str(&text).with_context(|| format!("failed to load rules file {:?}", path))
    }

    pub fn from_ini(ini: &IniFile) -> Self {
        let mut database = Self {
            general: ini
                .section("General")
                .map(GeneralRules::from_section)
                .unwrap_or_default(),
            ..Self::default()
        };

        database.easy = difficulty_from(ini, &["Easy"]);
        database.normal = difficulty_from(ini, &["Normal"]);
        database.hard = difficulty_from(ini, &["Hard", "Difficult"]);

        for name in list_names(ini, "Weapons") {
            match ini.section(&name) {
                Some(section) => {
                    database
                        .weapons
                        .insert(name.clone(), WeaponRule::from_section(&name, section));
                }
                None => database
                    .warnings
                    .push(format!("weapon '{}' listed but section is missing", name)),
            }
        }

        for name in list_names(ini, "Warheads") {
            match ini.section(&name) {
                Some(section) => {
                    database
                        .warheads
                        .insert(name.clone(), WarheadRule::from_section(&name, section));
                }
                None => database
                    .warnings
                    .push(format!("warhead '{}' listed but section is missing", name)),
            }
        }

        for name in list_names(ini, "Houses") {
            if let Some(section) = ini.section(&name) {
                database
                    .houses
                    .insert(name.clone(), HouseRule::from_section(&name, section));
            } else {
                database
                    .warnings
                    .push(format!("house '{}' listed but section is missing", name));
            }
        }

        for category in ["Infantry", "Units", "Aircraft", "Buildings", "Structures"] {
            let names = list_names(ini, category);
            if !names.is_empty() {
                database.categories.insert(category.to_string(), names);
            }
        }

        let known: BTreeSet<&str> = META_SECTIONS.iter().copied().collect();
        for (name, section) in &ini.sections {
            if known.contains(name.as_str())
                || database.weapons.contains_key(name)
                || database.warheads.contains_key(name)
                || database.houses.contains_key(name)
            {
                continue;
            }
            // Sections may also be defined without appearing in the list
            // sections (hand-written or partial Rules.ini files). Classify by
            // the keys they carry so lookups stay consistent.
            if (section.get_ci("Damage").is_some()
                || section.get_ci("ROF").is_some()
                || section.get_ci("Range").is_some()
                || section.get_ci("Warhead").is_some()
                || section.get_ci("Projectile").is_some()
                || section.get_ci("Burst").is_some())
                && section.get_ci("Cost").is_none()
                && section.get_ci("Strength").is_none()
            {
                database
                    .weapons
                    .insert(name.clone(), WeaponRule::from_section(name, section));
                continue;
            }
            if (section.get_ci("Verses").is_some() || section.get_ci("Spread").is_some())
                && !database.weapons.contains_key(name)
            {
                database
                    .warheads
                    .insert(name.clone(), WarheadRule::from_section(name, section));
                continue;
            }
            if TypeRule::looks_like_type(section) {
                database
                    .types
                    .insert(name.clone(), TypeRule::from_section(name, section));
            }
        }

        database
    }

    pub fn weapon(&self, name: &str) -> Option<&WeaponRule> {
        self.weapons.get(name)
    }

    pub fn warhead(&self, name: &str) -> Option<&WarheadRule> {
        self.warheads.get(name)
    }

    pub fn type_rule(&self, name: &str) -> Option<&TypeRule> {
        self.types.get(name)
    }

    pub fn house(&self, name: &str) -> Option<&HouseRule> {
        self.houses.get(name)
    }

    /// Cost override for a type section, if the rules file provides one.
    pub fn type_cost(&self, name: &str) -> Option<i32> {
        self.types.get(name).and_then(|rule| rule.cost)
    }

    pub fn type_strength(&self, name: &str) -> Option<i32> {
        self.types.get(name).and_then(|rule| rule.strength)
    }

    /// Build time override for a type section, if the rules file provides one.
    pub fn type_build_time(&self, name: &str) -> Option<i32> {
        self.types
            .get(name)
            .and_then(|rule| rule.raw.get("BuildTime"))
            .and_then(|v| v.as_i64().ok())
            .map(|v| v as i32)
    }

    pub fn difficulty(&self, level: DifficultyLevel) -> &DifficultyRule {
        match level {
            DifficultyLevel::Easy => &self.easy,
            DifficultyLevel::Normal => &self.normal,
            DifficultyLevel::Hard => &self.hard,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.weapons.is_empty()
            && self.warheads.is_empty()
            && self.types.is_empty()
            && self.houses.is_empty()
    }

    pub fn type_count(&self) -> usize {
        self.types.len()
    }
}

fn difficulty_from(ini: &IniFile, candidates: &[&str]) -> DifficultyRule {
    candidates
        .iter()
        .find_map(|name| ini.section(name))
        .map(DifficultyRule::from_section)
        .unwrap_or_default()
}

/// Read a Westwood list section such as [Weapons] into ordered names.
fn list_names(ini: &IniFile, section_name: &str) -> Vec<String> {
    let Some(section) = ini.section(section_name) else {
        return Vec::new();
    };
    let mut entries: Vec<(i64, String)> = section
        .values
        .iter()
        .filter_map(|(key, value)| {
            let order = key.parse::<i64>().unwrap_or(i64::MAX);
            match value {
                IniValue::String(name) => Some((order, name.clone())),
                IniValue::Integer(name) => Some((order, name.to_string())),
                _ => None,
            }
        })
        .collect();
    entries.sort_by_key(|(order, _)| *order);
    entries.into_iter().map(|(_, name)| name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_rules_cpp() {
        let rules = RulesDatabase::default();
        assert_eq!(rules.general.base_size_add, 3);
        assert_eq!(rules.general.power_surplus, 50);
        assert_eq!(rules.general.attack_interval, 3);
        assert_eq!(rules.general.attack_delay, 5);
        assert_eq!(rules.general.power_emergency_fraction, 0x00C0);
        assert_eq!(rules.general.helipad_ratio, 0x1E);
        assert_eq!(rules.general.aa_ratio, 0x24);
        assert_eq!(rules.general.defense_ratio, 0x66);
        assert_eq!(rules.general.defense_limit, 40);
        assert_eq!(rules.general.infantry_reserve, 3000);
        assert!(rules.general.is_computer_paranoid);
        assert!(rules.general.allow_super_weapons);

        assert_eq!(rules.normal.firepower_bias, 1.0);
        assert_eq!(rules.normal.repair_delay, 0.02);
        assert_eq!(rules.normal.build_delay, 0.03);
        assert!(rules.normal.is_wall_destroyer);
        assert!(!rules.normal.is_build_slowdown);

        assert!(rules.is_empty());
    }

    #[test]
    fn general_section_overrides_defaults() {
        let rules = RulesDatabase::from_str(
            "[General]\nBaseSizeAdd=7\nPowerEmergencyFraction=0x00FF\nAllowSuperWeapons=no\nMaxIQ=3\n",
        )
        .unwrap();
        assert_eq!(rules.general.base_size_add, 7);
        assert_eq!(rules.general.power_emergency_fraction, 0x00FF);
        assert_eq!(rules.general.max_iq, 3);
        assert!(!rules.general.allow_super_weapons);
        // Untouched values keep the RULES.CPP default.
        assert_eq!(rules.general.attack_interval, 3);
        assert_eq!(rules.general.infantry_reserve, 3000);
    }

    #[test]
    fn difficulty_sections_are_typed() {
        let rules = RulesDatabase::from_str(
            "[Easy]\nFirepowerBias=0.8\nCostBias=0.9\nIsWallDestroyer=no\n[Hard]\nFirepowerBias=1.2\nBuildSpeedBias=0.85\n",
        )
        .unwrap();
        let easy = rules.difficulty(DifficultyLevel::Easy);
        assert_eq!(easy.firepower_bias, 0.8);
        assert_eq!(easy.cost_bias, 0.9);
        assert!(!easy.is_wall_destroyer);
        assert_eq!(easy.groundspeed_bias, 1.0);

        let normal = rules.difficulty(DifficultyLevel::Normal);
        assert_eq!(normal.firepower_bias, 1.0);

        let hard = rules.difficulty(DifficultyLevel::Hard);
        assert_eq!(hard.firepower_bias, 1.2);
        assert_eq!(hard.build_speed_bias, 0.85);
    }

    #[test]
    fn difficulty_accepts_red_alert_difficult_alias() {
        let rules = RulesDatabase::from_str("[Difficult]\nFirepowerBias=1.1\n").unwrap();
        assert_eq!(rules.difficulty(DifficultyLevel::Hard).firepower_bias, 1.1);
    }

    #[test]
    fn parses_weapon_and_warhead_tables() {
        let rules = RulesDatabase::from_str(
            "[Weapons]\n1=M16\n2=Cannon105\n3=Missing\n[M16]\nDamage=15\nROF=20\nRange=768\nWarhead=SA\nProjectile=Invisible\nIsInvisible=yes\n[Cannon105]\nDamage=75\nROF=80\nRange=1024\nWarhead=AP\nBurst=2\n[Warheads]\n1=SA\n2=AP\n[SA]\nSpread=1\nVerses=100,80,60,40,20\n[AP]\nSpread=0\nWall=yes\nVerses=25,35,50,75,100\n",
        )
        .unwrap();

        let m16 = rules.weapon("M16").unwrap();
        assert_eq!(m16.damage, 15);
        assert_eq!(m16.rof, 20);
        assert_eq!(m16.range_cells(), 3);
        assert_eq!(m16.warhead.as_deref(), Some("SA"));
        assert!(m16.is_invisible);

        let cannon = rules.weapon("Cannon105").unwrap();
        assert_eq!(cannon.range_cells(), 4);
        assert_eq!(cannon.burst, 2);
        assert!(!cannon.is_invisible);

        let sa = rules.warhead("SA").unwrap();
        assert_eq!(sa.spread, 1);
        assert_eq!(sa.verses_percent(1), 80);
        assert_eq!(sa.verses_percent(9), 100);

        let ap = rules.warhead("AP").unwrap();
        assert!(ap.wall);
        assert_eq!(ap.verses_percent(4), 100);
        assert_eq!(ap.verses_percent(0), 25);

        assert_eq!(rules.warnings.len(), 1);
        assert!(rules.warnings[0].contains("Missing"));
    }

    #[test]
    fn parses_type_sections_and_costs() {
        let rules = RulesDatabase::from_str(
            "[E1]\nCost=150\nStrength=60\nSpeed=8\nSight=1\nPrimary=M16\nOwner=GDI,Nod\nPrerequisite=BARRACKS\nTechLevel=1\n[MTNK]\nCost=900\nStrength=400\nPrimary=Cannon105\nArmor=Steel\nSecondary=M60MG\n[FACT]\nCost=5000\nStrength=400\n[Infantry]\n1=E1\n[Units]\n1=MTNK\n",
        )
        .unwrap();
        assert_eq!(rules.type_count(), 3);
        assert_eq!(rules.type_cost("E1"), Some(150));
        assert_eq!(rules.type_strength("MTNK"), Some(400));
        assert_eq!(rules.type_cost("MISSING"), None);

        let e1 = rules.type_rule("E1").unwrap();
        assert_eq!(e1.owner, vec!["GDI", "Nod"]);
        assert_eq!(e1.prerequisite, vec!["BARRACKS"]);
        assert_eq!(e1.primary.as_deref(), Some("M16"));
        assert_eq!(e1.sight, Some(1));

        let mtnk = rules.type_rule("MTNK").unwrap();
        assert_eq!(mtnk.armor.as_deref(), Some("Steel"));
        assert_eq!(mtnk.secondary.as_deref(), Some("M60MG"));

        assert_eq!(
            rules.categories.get("Infantry"),
            Some(&vec!["E1".to_string()])
        );
        assert_eq!(
            rules.categories.get("Units"),
            Some(&vec!["MTNK".to_string()])
        );
        // Meta/list sections never become playable types.
        assert!(rules.type_rule("Infantry").is_none());
    }

    #[test]
    fn parses_houses() {
        let rules = RulesDatabase::from_str(
            "[Houses]\n1=GDI\n2=Nod\n[GDI]\nName=Global Defense Initiative\nColor=Gold\n[Nod]\nName=Brotherhood of Nod\nColor=Red\n",
        )
        .unwrap();
        assert_eq!(rules.houses.len(), 2);
        let gdi = rules.house("GDI").unwrap();
        assert_eq!(
            gdi.display_name.as_deref(),
            Some("Global Defense Initiative")
        );
        assert_eq!(gdi.color.as_deref(), Some("Gold"));
        assert_eq!(rules.house("Nod").unwrap().color.as_deref(), Some("Red"));
    }

    #[test]
    fn parses_inheritance_inside_rules() {
        let rules = RulesDatabase::from_str(
            "[BaseUnit]\nCost=100\nStrength=50\nSpeed=10\n[E1:BaseUnit]\nCost=120\n",
        )
        .unwrap();
        let e1 = rules.type_rule("E1").unwrap();
        assert_eq!(e1.cost, Some(120));
        assert_eq!(e1.strength, Some(50));
        assert_eq!(e1.speed, Some(10));
    }

    #[test]
    fn level_parser_is_case_insensitive() {
        assert_eq!(DifficultyLevel::parse("easy"), Some(DifficultyLevel::Easy));
        assert_eq!(
            DifficultyLevel::parse("NORMAL"),
            Some(DifficultyLevel::Normal)
        );
        assert_eq!(
            DifficultyLevel::parse("Difficult"),
            Some(DifficultyLevel::Hard)
        );
        assert_eq!(DifficultyLevel::parse("brutal"), None);
    }
}

#[cfg(test)]
mod classification_tests {
    use super::*;

    #[test]
    fn parses_unlisted_weapon_and_warhead_sections() {
        let rules = RulesDatabase::from_str(
            "[M16]\nDamage=25\nROF=20\nRange=1024\nWarhead=AP\n[AP]\nSpread=0\nVerses=10,20,30,40,50\n",
        )
        .unwrap();
        assert_eq!(rules.weapon("M16").unwrap().damage, 25);
        assert_eq!(rules.weapon("M16").unwrap().warhead.as_deref(), Some("AP"));
        assert_eq!(rules.warhead("AP").unwrap().verses_percent(3), 40);
        assert!(rules.types.is_empty());
    }

    #[test]
    fn section_classification_prefers_types_with_prerequisites() {
        let rules = RulesDatabase::from_str("[E1]\nCost=100\nPrerequisite=WEAP\n").unwrap();
        assert_eq!(rules.type_rule("E1").unwrap().prerequisite, vec!["WEAP"]);
        assert!(rules.weapons.is_empty());
    }

    #[test]
    fn structures_are_types_not_weapons() {
        let rules = RulesDatabase::from_str(
            "[NUKE]\nCost=300\nStrength=200\nPower=100\n[M16]\nDamage=15\nROF=20\n",
        )
        .unwrap();
        assert_eq!(rules.type_cost("NUKE"), Some(300));
        assert!(rules.weapon("NUKE").is_none());
        assert_eq!(rules.weapon("M16").unwrap().damage, 15);
        assert!(rules.type_rule("M16").is_none());
    }
}
