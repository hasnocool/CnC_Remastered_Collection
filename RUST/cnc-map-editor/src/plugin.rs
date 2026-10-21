//! Game-specific editor plugins.
//!
//! A plugin supplies the theater list, buildable catalog, starting credits and
//! validation rules that differ between Tiberian Dawn and Red Alert. The editor
//! core stays game-agnostic and talks to this trait only.

use crate::map::{ImportKind, MapModel, MapObject};
use anyhow::Context;
use cnc_core::ClassId;
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

    /// Faction hint shown in the palette when an object is side-locked.
    fn object_side(&self, _class_id: ClassId) -> Option<&'static str> {
        None
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
pub struct TiberianDawnPlugin;

impl GamePlugin for TiberianDawnPlugin {
    fn name(&self) -> &str {
        "Tiberian Dawn"
    }

    fn available_terrains(&self) -> Vec<String> {
        vec!["Temperate".into(), "Snow".into(), "Desert".into()]
    }

    fn available_objects(&self) -> Vec<ClassId> {
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

    fn default_credits(&self) -> i32 {
        5_000
    }

    fn object_side(&self, class_id: ClassId) -> Option<&'static str> {
        match class_id {
            ClassId::VehicleE2 | ClassId::AircraftE1 => Some("GDI"),
            _ => None,
        }
    }
}

/// Red Alert (Allied/Soviet) plugin.
pub struct RedAlertPlugin;

impl GamePlugin for RedAlertPlugin {
    fn name(&self) -> &str {
        "Red Alert"
    }

    fn available_terrains(&self) -> Vec<String> {
        vec!["Temperate".into(), "Snow".into(), "Urban".into()]
    }

    fn available_objects(&self) -> Vec<ClassId> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_core::Vec2i;

    #[test]
    fn plugins_expose_side_locked_objects() {
        let td = TiberianDawnPlugin;
        assert_eq!(td.object_side(ClassId::VehicleE2), Some("GDI"));
        assert_eq!(td.object_side(ClassId::AircraftE1), Some("GDI"));
        assert_eq!(td.object_side(ClassId::PowerPlant), None);
        assert_eq!(
            td.object_side(ClassId::VehicleE1),
            None,
            "Jeep/Buggy share a class"
        );
        let ra = RedAlertPlugin;
        assert_eq!(ra.object_side(ClassId::Chronosphere), Some("Allies"));
        assert_eq!(ra.object_side(ClassId::TeslaCoil), Some("Soviet"));
        assert_eq!(ra.object_side(ClassId::PowerPlant), None);
    }

    #[test]
    fn plugins_expose_distinct_catalogs() {
        let td = TiberianDawnPlugin;
        let ra = RedAlertPlugin;
        assert_eq!(td.name(), "Tiberian Dawn");
        assert_ne!(td.available_objects(), ra.available_objects());
        assert!(ra.available_objects().contains(&ClassId::ShipE4));
        assert!(!td.supports_naval());
        assert!(ra.supports_naval());
        assert_eq!(ra.default_credits(), 10_000);
    }

    #[test]
    fn theaters_and_templates_are_derived() {
        let ra = RedAlertPlugin;
        assert!(ra.theaters().contains(&Theater::Urban));
        assert!(!TiberianDawnPlugin.theaters().contains(&Theater::Urban));
        assert!(ra.trigger_templates().len() >= 3);
    }

    #[test]
    fn object_validation_checks_bounds_and_house_owner() {
        let model = MapModel::new(8, 8);
        let plugin = TiberianDawnPlugin;
        let mut object = MapObject {
            id: 1,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(1, 1),
            owner: Some(ClassId::HouseGDI),
        };
        assert!(plugin.validate_object(&model, &object));
        object.owner = Some(ClassId::InfantryE1);
        assert!(!plugin.validate_object(&model, &object));
        object.owner = Some(ClassId::HouseGDI);
        object.position = Vec2i::new(99, 1);
        assert!(!plugin.validate_object(&model, &object));
    }
}
