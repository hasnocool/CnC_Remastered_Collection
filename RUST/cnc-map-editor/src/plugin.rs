//! Game-specific editor plugins.
//!
//! A plugin supplies the theater list, buildable catalog, starting credits and
//! validation rules that differ between Tiberian Dawn and Red Alert. The editor
//! core stays game-agnostic and talks to this trait only.

use crate::map::{ImportKind, MapModel, MapObject};
use anyhow::Context;
use cnc_core::{ClassId, RulesDatabase};
use std::path::{Path, PathBuf};

/// Terrain theater a map is authored for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theater {
    Temperate,
    Snow,
    Desert,
    Urban,
}

impl Theater {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Temperate => "Temperate",
            Self::Snow => "Snow",
            Self::Desert => "Desert",
            Self::Urban => "Urban",
        }
    }
}

/// Object data loaded from RulesDatabase for editor catalog.
#[derive(Debug, Clone)]
pub struct ObjectData {
    pub name: String,
    pub prerequisite: Vec<ClassId>,
    pub cost: i32,
    pub build_time: i32,
    pub owner: Vec<ClassId>,
    pub display_name: Option<String>,
}

/// Rules the editor needs from the game being edited.
pub trait GamePlugin: Send + Sync {
    fn name(&self) -> &str;
    fn available_terrains(&self) -> Vec<String>;
    fn available_objects(&self) -> Vec<ClassId>;

    fn theaters(&self) -> Vec<Theater> {
        self.available_terrains()
            .iter()
            .filter_map(|name| match name.as_str() {
                "Temperate" => Some(Theater::Temperate),
                "Snow" => Some(Theater::Snow),
                "Desert" => Some(Theater::Desert),
                "Urban" => Some(Theater::Urban),
                _ => None,
            })
            .collect()
    }

    /// Starting credits for a new scenario.
    fn default_credits(&self) -> i32 {
        10_000
    }

    /// RA has naval yards and ships; TD does not.
    fn supports_naval(&self) -> bool {
        false
    }

    /// Trigger templates offered by the editor (name, condition, action).
    fn trigger_templates(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Victory", "structures:1:any:==:0", "win"),
            ("Defeat", "!has:0:FACT", "lose"),
            ("Timer", "elapsed:300", "message:Time is up"),
        ]
    }

    /// Bounds and ownership validation. Defaults to "inside the grid".
    fn validate_object(&self, model: &MapModel, object: &MapObject) -> bool {
        model.in_bounds(object.position) && object.owner.is_none_or(|owner| owner.is_house())
    }

    fn get_object_name(&self, class_id: ClassId) -> String {
        format!("{class_id:?}")
    }

    /// Get object data including prerequisites, cost, etc. from RulesDatabase.
    /// Returns None for unknown/hardcoded types.
    fn get_object_data(&self, _class_id: ClassId) -> Option<ObjectData> {
        None
    }

    /// Faction hint shown in the palette when an object is side-locked.
    fn object_side(&self, _class_id: ClassId) -> Option<&'static str> {
        None
    }

    /// Available trigger events for the trigger dialog (from todo-32 enums).
    fn available_trigger_events(&self) -> Vec<cnc_core::trigger_records::EventKind> {
        vec![]
    }

    /// Available trigger actions for the trigger dialog (from todo-32 enums).
    fn available_trigger_actions(&self) -> Vec<cnc_core::trigger_records::ActionKind> {
        vec![]
    }

    /// Load catalog from RulesDatabase. Default implementation uses hardcoded available_objects().
    fn load_catalog_from_rules(&mut self, _rules: &RulesDatabase) {
        // Default: do nothing, use hardcoded available_objects()
    }
    /// Open a scenario from disk, dispatching on its extension/dialect.
    fn load_map(&self, path: &Path) -> anyhow::Result<MapModel> {
        load_scenario(path).map(|(model, _kind)| model)
    }

    /// Save a scenario to disk in the dialect it was opened from (or the
    /// editor dialect for a new map). Returns every file written, so the TD
    /// `.ini`+`.bin` pair is visible to the caller.
    fn save_map(&self, model: &MapModel, path: &Path) -> anyhow::Result<Vec<PathBuf>> {
        save_scenario(model, path)
    }
    /// Rule-aware map validation using the game's type registry.
    /// Checks required structures/houses, trigger wiring, object prerequisites.
    fn validate_map(&self, model: &MapModel) -> Vec<String> {
        let _ = model;
        vec![]
    }
}

/// Extensions a scenario can be opened from. TD needs its sibling `.bin`.
pub const SCENARIO_EXTENSIONS: &[&str] = &["ini", "mpr"];

fn extension_of(path: &Path) -> String {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn require_scenario_extension(path: &Path) -> anyhow::Result<String> {
    let extension = extension_of(path);
    if !SCENARIO_EXTENSIONS.contains(&extension.as_str()) {
        let shown = if extension.is_empty() {
            "<none>".to_string()
        } else {
            format!(".{extension}")
        };
        anyhow::bail!(
            "unsupported scenario extension '{shown}' for {} (supported: .ini, .mpr; a TD .ini reads its sibling .bin)",
            path.display()
        );
    }
    Ok(extension)
}

/// Load a scenario, picking the reader by extension and dialect.
///
/// `.mpr` is an RA scenario; `.ini` may be either dialect and is sniffed by
/// content, with a sibling `.bin` supplied to the TD reader.
pub fn load_scenario(path: &Path) -> anyhow::Result<(MapModel, ImportKind)> {
    let extension = require_scenario_extension(path)?;
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("read scenario {}", path.display()))?;
    let bin = if extension == "ini" {
        std::fs::read(path.with_extension("bin")).ok()
    } else {
        None
    };
    MapModel::import_text_with(&text, bin.as_deref())
        .with_context(|| format!("open scenario {}", path.display()))
}

/// Save a scenario in its original dialect when it was imported from one.
///
/// A fresh map is written in the editor dialect unless the target is `.mpr`,
/// which always denotes an original RA scenario. TD original saves also write
/// the sibling `.bin` cell image.
pub fn save_scenario(model: &MapModel, path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let extension = extension_of(path);
    let dialect = model.original_dialect().unwrap_or_else(|| {
        if extension == "mpr" {
            ImportKind::OriginalRa
        } else {
            ImportKind::EditorDialect
        }
    });
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("create directory {}", parent.display()))?;
        }
    }
    match dialect {
        ImportKind::OriginalRa => {
            let text = model.to_original_ra()?;
            std::fs::write(path, text)
                .with_context(|| format!("write scenario {}", path.display()))?;
            Ok(vec![path.to_path_buf()])
        }
        ImportKind::OriginalTd => {
            let (text, bin) = model.to_original_td()?;
            std::fs::write(path, text)
                .with_context(|| format!("write scenario {}", path.display()))?;
            let bin_path = path.with_extension("bin");
            std::fs::write(&bin_path, bin)
                .with_context(|| format!("write cell image {}", bin_path.display()))?;
            Ok(vec![path.to_path_buf(), bin_path])
        }
        ImportKind::EditorDialect => {
            std::fs::write(path, model.to_ini())
                .with_context(|| format!("write scenario {}", path.display()))?;
            Ok(vec![path.to_path_buf()])
        }
    }
}

/// Tiberian Dawn (GDI/Nod) plugin.
pub struct TiberianDawnPlugin {
    /// Catalog loaded from RulesDatabase, falls back to hardcoded if not loaded.
    catalog: Vec<ClassId>,
    /// Object data loaded from RulesDatabase.
    object_data: std::collections::HashMap<ClassId, ObjectData>,
}

impl Default for TiberianDawnPlugin {
    fn default() -> Self {
        Self {
            catalog: Self::hardcoded_catalog(),
            object_data: std::collections::HashMap::new(),
        }
    }
}

impl TiberianDawnPlugin {
    fn hardcoded_catalog() -> Vec<ClassId> {
        vec![
            ClassId::InfantryE1,
            ClassId::InfantryE2,
            ClassId::InfantryE3,
            ClassId::VehicleE1,
            ClassId::VehicleE2,
            ClassId::VehicleE3,
            ClassId::VehicleE6,
            ClassId::AircraftE1,
            ClassId::ConstructionYard,
            ClassId::PowerPlant,
            ClassId::Refinery,
            ClassId::Barracks,
            ClassId::WarFactory,
            ClassId::Helipad,
            ClassId::Radar,
            ClassId::MissileSilo,
        ]
    }

    /// Map INI type name to ClassId.
    fn type_name_to_class_id(name: &str) -> Option<ClassId> {
        match name {
            "E1" => Some(ClassId::InfantryE1),
            "E2" => Some(ClassId::InfantryE2),
            "E3" => Some(ClassId::InfantryE3),
            "E4" => Some(ClassId::InfantryE4),
            "E5" => Some(ClassId::InfantryE5),
            "E6" => Some(ClassId::InfantryE6),
            "E7" => Some(ClassId::InfantryE7),
            "E8" => Some(ClassId::InfantryE8),
            "E9" => Some(ClassId::InfantryE9),
            "E10" => Some(ClassId::InfantryE10),
            "E11" => Some(ClassId::InfantryE11),
            "LTNK" => Some(ClassId::VehicleE1),
            "MTNK" => Some(ClassId::VehicleE2),
            "HTNK" => Some(ClassId::VehicleE3),
            "MAMM" => Some(ClassId::VehicleE4),
            "APC" => Some(ClassId::VehicleE5),
            "HARV" => Some(ClassId::VehicleE6),
            "MCV" => Some(ClassId::VehicleE7),
            "RANG" => Some(ClassId::VehicleE8),
            "HUMV" => Some(ClassId::VehicleE9),
            "MLRS" => Some(ClassId::VehicleE10),
            "ARTY" => Some(ClassId::VehicleE11),
            "V2" => Some(ClassId::VehicleE12),
            "ORCA" => Some(ClassId::AircraftE1),
            "TRAN" => Some(ClassId::AircraftE2),
            "HELI" => Some(ClassId::AircraftE1),
            "A10" => Some(ClassId::AircraftE4),
            "YAK" => Some(ClassId::AircraftE6),
            "FACT" => Some(ClassId::ConstructionYard),
            "POWR" => Some(ClassId::PowerPlant),
            "BARR" => Some(ClassId::Barracks),
            "WARF" => Some(ClassId::WarFactory),
            "HPAD" => Some(ClassId::Helipad),
            "RADR" => Some(ClassId::Radar),
            "MSIL" => Some(ClassId::MissileSilo),
            "AFLD" => Some(ClassId::Airfield),
            "SAM" => Some(ClassId::AA_Gun),
            "SILO" => Some(ClassId::Silo),
            "BIOL" => Some(ClassId::BioLab),
            "TECH" => Some(ClassId::TechCenter),
            "TEMP" => Some(ClassId::ConstructionYard),
            "HAND" => Some(ClassId::ConstructionYard),
            "EYE" => Some(ClassId::Radar),
            "NAPU" => Some(ClassId::PowerPlant),
            "GATE" => Some(ClassId::AA_Gun),
            "TURR" => Some(ClassId::AA_Gun),
            "GTWR" => Some(ClassId::AA_Gun),
            "ATWR" => Some(ClassId::AA_Gun),
            "OBEL" => Some(ClassId::AA_Gun),
            _ => None,
        }
    }

    /// Parse prerequisite string into ClassId vector.
    fn parse_prerequisites(prereq_str: &str) -> Vec<ClassId> {
        prereq_str
            .split(',')
            .filter_map(|s| Self::type_name_to_class_id(s.trim()))
            .collect()
    }

    /// Parse owner string into ClassId vector.
    fn parse_owners(owner_str: &str) -> Vec<ClassId> {
        owner_str
            .split(',')
            .filter_map(|s| match s.trim() {
                "GDI" => Some(ClassId::HouseGDI),
                "Nod" => Some(ClassId::HouseNod),
                "Allies" => Some(ClassId::HouseAllies),
                "Soviet" => Some(ClassId::HouseSoviet),
                "Neutral" => Some(ClassId::House),
                _ => None,
            })
            .collect()
    }
}

impl GamePlugin for TiberianDawnPlugin {
    fn name(&self) -> &str {
        "Tiberian Dawn"
    }

    fn available_terrains(&self) -> Vec<String> {
        vec!["Temperate".into(), "Snow".into(), "Desert".into()]
    }

    fn available_objects(&self) -> Vec<ClassId> {
        self.catalog.clone()
    }

    fn default_credits(&self) -> i32 {
        5_000
    }

    fn object_side(&self, class_id: ClassId) -> Option<&'static str> {
        match class_id {
            ClassId::VehicleE2 | ClassId::AircraftE1 => Some("GDI"),
            _ => None,
        }
    }

    fn get_object_data(&self, class_id: ClassId) -> Option<ObjectData> {
        self.object_data.get(&class_id).cloned()
    }

    fn available_trigger_events(&self) -> Vec<cnc_core::trigger_records::EventKind> {
        use cnc_core::trigger_records::EventKind;
        vec![
            EventKind::None,
            EventKind::PlayerEntered,
            EventKind::Spied,
            EventKind::Thieved,
            EventKind::Discovered,
            EventKind::HouseDiscovered,
            EventKind::Attacked,
            EventKind::Destroyed,
            EventKind::Any,
            EventKind::UnitsDestroyed,
            EventKind::BuildingsDestroyed,
            EventKind::AllDestroyed,
            EventKind::Credits,
            EventKind::ElapsedTime,
            EventKind::MissionTimerExpired,
            EventKind::BuildingsDestroyedCount,
            EventKind::UnitsDestroyedCount,
            EventKind::NoFactories,
            EventKind::EvacCivilian,
            EventKind::Build,
            EventKind::BuildUnit,
            EventKind::BuildInfantry,
            EventKind::BuildAircraft,
            EventKind::LeavesMap,
            EventKind::EntersZone,
            EventKind::CrossHorizontal,
            EventKind::CrossVertical,
            EventKind::GlobalSet,
            EventKind::GlobalClear,
            EventKind::FakesDestroyed,
            EventKind::LowPower,
            EventKind::AllBridgesDestroyed,
            EventKind::BuildingExists,
        ]
    }

    fn available_trigger_actions(&self) -> Vec<cnc_core::trigger_records::ActionKind> {
        use cnc_core::trigger_records::ActionKind;
        vec![
            ActionKind::None,
            ActionKind::Win,
            ActionKind::Lose,
            ActionKind::BeginProduction,
            ActionKind::CreateTeam,
            ActionKind::DestroyTeam,
            ActionKind::AllHunt,
            ActionKind::Reinforcements,
            ActionKind::DropZone,
            ActionKind::FireSale,
            ActionKind::PlayMovie,
            ActionKind::TextTrigger,
            ActionKind::DestroyTrigger,
            ActionKind::Autocreate,
            ActionKind::WinLose,
            ActionKind::AllowWin,
            ActionKind::RevealAll,
            ActionKind::RevealSome,
            ActionKind::RevealZone,
            ActionKind::PlaySound,
            ActionKind::PlayMusic,
            ActionKind::PlaySpeech,
            ActionKind::ForceTrigger,
            ActionKind::StartTimer,
            ActionKind::StopTimer,
            ActionKind::ExtendTimer,
            ActionKind::ShortenTimer,
            ActionKind::SetTimer,
            ActionKind::SetGlobal,
            ActionKind::ClearGlobal,
            ActionKind::BaseBuilding,
            ActionKind::CreepShadow,
            ActionKind::DestroyObject,
            ActionKind::OneTimeSpecial,
            ActionKind::FullSpecial,
            ActionKind::PreferredTarget,
            ActionKind::LaunchNukes,
        ]
    }

    fn load_catalog_from_rules(&mut self, rules: &RulesDatabase) {
        let mut catalog = Vec::new();
        let mut object_data = std::collections::HashMap::new();

        // Iterate over all categories: Infantry, Units, Aircraft, Buildings, Structures
        for category in ["Infantry", "Units", "Aircraft", "Buildings", "Structures"] {
            if let Some(names) = rules.categories.get(category) {
                for name in names {
                    if let Some(class_id) = Self::type_name_to_class_id(name) {
                        if let Some(type_rule) = rules.type_rule(name) {
                            let prerequisites = type_rule.prerequisite.clone();
                            let owners = type_rule.owner.clone();
                            let cost = type_rule.cost.unwrap_or(0);
                            let build_time = rules.type_build_time(name).unwrap_or(0);
                            let display_name = type_rule.display_name.clone();

                            object_data.insert(
                                class_id,
                                ObjectData {
                                    name: name.clone(),
                                    prerequisite: Self::parse_prerequisites(
                                        &prerequisites.join(","),
                                    ),
                                    cost,
                                    build_time,
                                    owner: Self::parse_owners(&owners.join(",")),
                                    display_name,
                                },
                            );
                            catalog.push(class_id);
                        } else {
                            eprintln!("Warning: type '{}' listed in [{}] but no type rule found, skipping", name, category);
                        }
                    } else {
                        eprintln!(
                            "Warning: unknown type '{}' in [{}] category, skipping",
                            name, category
                        );
                    }
                }
            }
        }

        // Fall back to hardcoded if no types were loaded
        if catalog.is_empty() {
            eprintln!(
                "Warning: no types loaded from RulesDatabase, falling back to hardcoded catalog"
            );
            catalog = Self::hardcoded_catalog();
        }

        self.catalog = catalog;
        self.object_data = object_data;
    }

    fn validate_map(&self, model: &MapModel) -> Vec<String> {
        let mut issues = Vec::new();

        // Check required houses
        let has_gdi = model.houses.iter().any(|h| h.name == "GDI");
        let has_nod = model.houses.iter().any(|h| h.name == "Nod");
        if !has_gdi && !has_nod {
            issues.push("map must have at least one house (GDI or Nod)".to_string());
        }

        // Check trigger condition/action validity
        for trigger in &model.triggers {
            let events = self.available_trigger_events();
            let actions = self.available_trigger_actions();

            // Validate condition
            if !trigger.condition.is_empty() && !trigger.condition.eq_ignore_ascii_case("always") {
                let cond_lower = trigger.condition.to_lowercase();
                if !events
                    .iter()
                    .any(|e| format!("{:?}", e).to_lowercase() == cond_lower)
                    && !cond_lower.contains(':')
                {
                    issues.push(format!(
                        "trigger '{}' has unknown condition '{}'",
                        trigger.name, trigger.condition
                    ));
                }
            }

            // Validate action
            if !trigger.action.is_empty() {
                let act_lower = trigger.action.to_lowercase();
                if !actions
                    .iter()
                    .any(|a| format!("{:?}", a).to_lowercase() == act_lower)
                    && !act_lower.contains(':')
                {
                    issues.push(format!(
                        "trigger '{}' has unknown action '{}'",
                        trigger.name, trigger.action
                    ));
                }
            }
        }

        issues
    }
}
/// Red Alert (Allied/Soviet) plugin.
pub struct RedAlertPlugin {
    /// Catalog loaded from RulesDatabase, falls back to hardcoded if not loaded.
    catalog: Vec<ClassId>,
    /// Object data loaded from RulesDatabase.
    object_data: std::collections::HashMap<ClassId, ObjectData>,
}

impl Default for RedAlertPlugin {
    fn default() -> Self {
        Self {
            catalog: Self::hardcoded_catalog(),
            object_data: std::collections::HashMap::new(),
        }
    }
}

impl RedAlertPlugin {
    fn hardcoded_catalog() -> Vec<ClassId> {
        vec![
            ClassId::InfantryE1,
            ClassId::InfantryE2,
            ClassId::InfantryE8,
            ClassId::InfantryE9,
            ClassId::VehicleE1,
            ClassId::VehicleE2,
            ClassId::VehicleE7,
            ClassId::AircraftE1,
            ClassId::ShipE1,
            ClassId::ShipE2,
            ClassId::ShipE4,
            ClassId::ConstructionYard,
            ClassId::PowerPlant,
            ClassId::Refinery,
            ClassId::Barracks,
            ClassId::WarFactory,
            ClassId::Helipad,
            ClassId::Airfield,
            ClassId::Radar,
            ClassId::TeslaCoil,
            ClassId::GapGenerator,
            ClassId::Chronosphere,
            ClassId::IronCurtain,
        ]
    }

    /// Map INI type name to ClassId (RA specific mappings).
    fn type_name_to_class_id(name: &str) -> Option<ClassId> {
        match name {
            "E1" => Some(ClassId::InfantryE1),
            "E2" => Some(ClassId::InfantryE2),
            "E3" => Some(ClassId::InfantryE3),
            "E4" => Some(ClassId::InfantryE4),
            "E5" => Some(ClassId::InfantryE5),
            "E6" => Some(ClassId::InfantryE6),
            "E7" => Some(ClassId::InfantryE7),
            "E8" => Some(ClassId::InfantryE8),
            "E9" => Some(ClassId::InfantryE9),
            "E10" => Some(ClassId::InfantryE10),
            "E11" => Some(ClassId::InfantryE11),
            "LTNK" => Some(ClassId::VehicleE1),
            "MTNK" => Some(ClassId::VehicleE2),
            "HTNK" => Some(ClassId::VehicleE3),
            "MAMM" => Some(ClassId::VehicleE4),
            "APC" => Some(ClassId::VehicleE5),
            "HARV" => Some(ClassId::VehicleE6),
            "MCV" => Some(ClassId::VehicleE7),
            "RANG" => Some(ClassId::VehicleE8),
            "HUMV" => Some(ClassId::VehicleE9),
            "MLRS" => Some(ClassId::VehicleE10),
            "ARTY" => Some(ClassId::VehicleE11),
            "V2" => Some(ClassId::VehicleE12),
            "TESL" => Some(ClassId::VehicleE14),
            "HELI" => Some(ClassId::AircraftE1),
            "TRAN" => Some(ClassId::AircraftE2),
            "YAK" => Some(ClassId::AircraftE6),
            "MIG" => Some(ClassId::AircraftE7),
            "CHIN" => Some(ClassId::AircraftE8),
            "GNBO" => Some(ClassId::ShipE1),
            "DSHK" => Some(ClassId::ShipE2),
            "CRUS" => Some(ClassId::ShipE3),
            "SUB" => Some(ClassId::ShipE4),
            "TST" => Some(ClassId::ShipE5),
            "CARR" => Some(ClassId::ShipE6),
            "FACT" => Some(ClassId::ConstructionYard),
            "POWR" => Some(ClassId::PowerPlant),
            "APWR" => Some(ClassId::PowerPlant),
            "NAPU" => Some(ClassId::PowerPlant),
            "BARR" => Some(ClassId::Barracks),
            "WARF" => Some(ClassId::WarFactory),
            "HPAD" => Some(ClassId::Helipad),
            "NAYD" => Some(ClassId::NavalYard),
            "SUBP" => Some(ClassId::SubPen),
            "GAP" => Some(ClassId::GapGenerator),
            "CHRO" => Some(ClassId::Chronosphere),
            "IRON" => Some(ClassId::IronCurtain),
            "MSIL" => Some(ClassId::MissileSilo),
            "GSIL" => Some(ClassId::GapGenerator),
            _ => None,
        }
    }

    /// Parse prerequisite string into ClassId vector.
    fn parse_prerequisites(prereq_str: &str) -> Vec<ClassId> {
        prereq_str
            .split(',')
            .filter_map(|s| Self::type_name_to_class_id(s.trim()))
            .collect()
    }

    /// Parse owner string into ClassId vector.
    fn parse_owners(owner_str: &str) -> Vec<ClassId> {
        owner_str
            .split(',')
            .filter_map(|s| match s.trim() {
                "GDI" => Some(ClassId::HouseGDI),
                "Nod" => Some(ClassId::HouseNod),
                "Allies" => Some(ClassId::HouseAllies),
                "Soviet" => Some(ClassId::HouseSoviet),
                "Neutral" => Some(ClassId::House),
                _ => None,
            })
            .collect()
    }
}

impl GamePlugin for RedAlertPlugin {
    fn name(&self) -> &str {
        "Red Alert"
    }

    fn available_terrains(&self) -> Vec<String> {
        vec!["Temperate".into(), "Snow".into(), "Urban".into()]
    }

    fn available_objects(&self) -> Vec<ClassId> {
        self.catalog.clone()
    }

    fn supports_naval(&self) -> bool {
        true
    }

    fn object_side(&self, class_id: ClassId) -> Option<&'static str> {
        match class_id {
            ClassId::Chronosphere | ClassId::GapGenerator => Some("Allies"),
            ClassId::IronCurtain | ClassId::TeslaCoil => Some("Soviet"),
            _ => None,
        }
    }

    fn default_credits(&self) -> i32 {
        10_000
    }

    fn trigger_templates(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Victory", "structures:1:any:==:0", "win"),
            ("Defeat", "!has:0:FACT", "lose"),
            ("Chronosphere", "elapsed:600", "message:Chronosphere ready"),
        ]
    }

    fn get_object_data(&self, class_id: ClassId) -> Option<ObjectData> {
        self.object_data.get(&class_id).cloned()
    }

    fn available_trigger_events(&self) -> Vec<cnc_core::trigger_records::EventKind> {
        use cnc_core::trigger_records::EventKind;
        vec![
            EventKind::None,
            EventKind::PlayerEntered,
            EventKind::Spied,
            EventKind::Thieved,
            EventKind::Discovered,
            EventKind::HouseDiscovered,
            EventKind::Attacked,
            EventKind::Destroyed,
            EventKind::Any,
            EventKind::UnitsDestroyed,
            EventKind::BuildingsDestroyed,
            EventKind::AllDestroyed,
            EventKind::Credits,
            EventKind::ElapsedTime,
            EventKind::MissionTimerExpired,
            EventKind::BuildingsDestroyedCount,
            EventKind::UnitsDestroyedCount,
            EventKind::NoFactories,
            EventKind::EvacCivilian,
            EventKind::Build,
            EventKind::BuildUnit,
            EventKind::BuildInfantry,
            EventKind::BuildAircraft,
            EventKind::LeavesMap,
            EventKind::EntersZone,
            EventKind::CrossHorizontal,
            EventKind::CrossVertical,
            EventKind::GlobalSet,
            EventKind::GlobalClear,
            EventKind::FakesDestroyed,
            EventKind::LowPower,
            EventKind::AllBridgesDestroyed,
            EventKind::BuildingExists,
        ]
    }

    fn available_trigger_actions(&self) -> Vec<cnc_core::trigger_records::ActionKind> {
        use cnc_core::trigger_records::ActionKind;
        vec![
            ActionKind::None,
            ActionKind::Win,
            ActionKind::Lose,
            ActionKind::BeginProduction,
            ActionKind::CreateTeam,
            ActionKind::DestroyTeam,
            ActionKind::AllHunt,
            ActionKind::Reinforcements,
            ActionKind::DropZone,
            ActionKind::FireSale,
            ActionKind::PlayMovie,
            ActionKind::TextTrigger,
            ActionKind::DestroyTrigger,
            ActionKind::Autocreate,
            ActionKind::WinLose,
            ActionKind::AllowWin,
            ActionKind::RevealAll,
            ActionKind::RevealSome,
            ActionKind::RevealZone,
            ActionKind::PlaySound,
            ActionKind::PlayMusic,
            ActionKind::PlaySpeech,
            ActionKind::ForceTrigger,
            ActionKind::StartTimer,
            ActionKind::StopTimer,
            ActionKind::ExtendTimer,
            ActionKind::ShortenTimer,
            ActionKind::SetTimer,
            ActionKind::SetGlobal,
            ActionKind::ClearGlobal,
            ActionKind::BaseBuilding,
            ActionKind::CreepShadow,
            ActionKind::DestroyObject,
            ActionKind::OneTimeSpecial,
            ActionKind::FullSpecial,
            ActionKind::PreferredTarget,
            ActionKind::LaunchNukes,
        ]
    }

    fn load_catalog_from_rules(&mut self, rules: &RulesDatabase) {
        let mut catalog = Vec::new();
        let mut object_data = std::collections::HashMap::new();

        // Iterate over all categories: Infantry, Units, Aircraft, Buildings, Structures
        for category in [
            "Infantry",
            "Units",
            "Aircraft",
            "Buildings",
            "Structures",
            "Ships",
        ] {
            if let Some(names) = rules.categories.get(category) {
                for name in names {
                    if let Some(class_id) = Self::type_name_to_class_id(name) {
                        if let Some(type_rule) = rules.type_rule(name) {
                            let prerequisites = type_rule.prerequisite.clone();
                            let owners = type_rule.owner.clone();
                            let cost = type_rule.cost.unwrap_or(0);
                            let build_time = rules.type_build_time(name).unwrap_or(0);
                            let display_name = type_rule.display_name.clone();

                            object_data.insert(
                                class_id,
                                ObjectData {
                                    name: name.clone(),
                                    prerequisite: Self::parse_prerequisites(
                                        &prerequisites.join(","),
                                    ),
                                    cost,
                                    build_time,
                                    owner: Self::parse_owners(&owners.join(",")),
                                    display_name,
                                },
                            );
                            catalog.push(class_id);
                        } else {
                            eprintln!("Warning: type '{}' listed in [{}] but no type rule found, skipping", name, category);
                        }
                    } else {
                        eprintln!(
                            "Warning: unknown type '{}' in [{}] category, skipping",
                            name, category
                        );
                    }
                }
            }
        }

        // Fall back to hardcoded if no types were loaded
        if catalog.is_empty() {
            eprintln!(
                "Warning: no types loaded from RulesDatabase, falling back to hardcoded catalog"
            );
            catalog = Self::hardcoded_catalog();
        }

        self.catalog = catalog;
        self.object_data = object_data;
    }

    fn validate_map(&self, model: &MapModel) -> Vec<String> {
        let mut issues = Vec::new();

        // Check required houses
        let has_allies = model.houses.iter().any(|h| h.name == "Allies");
        let has_soviet = model.houses.iter().any(|h| h.name == "Soviet");
        if !has_allies && !has_soviet {
            issues.push("map must have at least one house (Allies or Soviet)".to_string());
        }

        // Check trigger condition/action validity
        for trigger in &model.triggers {
            let events = self.available_trigger_events();
            let actions = self.available_trigger_actions();

            // Validate condition
            if !trigger.condition.is_empty() && !trigger.condition.eq_ignore_ascii_case("always") {
                let cond_lower = trigger.condition.to_lowercase();
                if !events
                    .iter()
                    .any(|e| format!("{:?}", e).to_lowercase() == cond_lower)
                    && !cond_lower.contains(':')
                {
                    issues.push(format!(
                        "trigger '{}' has unknown condition '{}'",
                        trigger.name, trigger.condition
                    ));
                }
            }

            // Validate action
            if !trigger.action.is_empty() {
                let act_lower = trigger.action.to_lowercase();
                if !actions
                    .iter()
                    .any(|a| format!("{:?}", a).to_lowercase() == act_lower)
                    && !act_lower.contains(':')
                {
                    issues.push(format!(
                        "trigger '{}' has unknown action '{}'",
                        trigger.name, trigger.action
                    ));
                }
            }
        }

        issues
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cnc_core::Vec2i;

    #[test]
    fn plugins_expose_side_locked_objects() {
        let td = TiberianDawnPlugin::default();
        assert_eq!(td.object_side(ClassId::VehicleE2), Some("GDI"));
        assert_eq!(td.object_side(ClassId::AircraftE1), Some("GDI"));
        assert_eq!(td.object_side(ClassId::PowerPlant), None);
        assert_eq!(
            td.object_side(ClassId::VehicleE1),
            None,
            "Jeep/Buggy share a class"
        );
        let ra = RedAlertPlugin::default();
        assert_eq!(ra.object_side(ClassId::Chronosphere), Some("Allies"));
        assert_eq!(ra.object_side(ClassId::TeslaCoil), Some("Soviet"));
        assert_eq!(ra.object_side(ClassId::PowerPlant), None);
    }

    #[test]
    fn plugins_expose_distinct_catalogs() {
        let td = TiberianDawnPlugin::default();
        let ra = RedAlertPlugin::default();
        assert_eq!(td.name(), "Tiberian Dawn");
        assert_ne!(td.available_objects(), ra.available_objects());
        assert!(ra.available_objects().contains(&ClassId::ShipE4));
        assert!(!td.supports_naval());
        assert!(ra.supports_naval());
        assert_eq!(ra.default_credits(), 10_000);
    }

    #[test]
    fn theaters_and_templates_are_derived() {
        let ra = RedAlertPlugin::default();
        assert!(ra.theaters().contains(&Theater::Urban));
        assert!(!TiberianDawnPlugin::default()
            .theaters()
            .contains(&Theater::Urban));
        assert!(ra.trigger_templates().len() >= 3);
    }

    #[test]
    fn object_validation_checks_bounds_and_house_owner() {
        let model = MapModel::new(8, 8);
        let plugin = TiberianDawnPlugin::default();
        let mut object = MapObject {
            id: 1,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(1, 1),
            owner: Some(ClassId::HouseGDI),
            ..Default::default()
        };
        assert!(plugin.validate_object(&model, &object));
        object.owner = Some(ClassId::InfantryE1);
        assert!(!plugin.validate_object(&model, &object));
        object.owner = Some(ClassId::HouseGDI);
        object.position = Vec2i::new(99, 1);
        assert!(!plugin.validate_object(&model, &object));
    }
}
