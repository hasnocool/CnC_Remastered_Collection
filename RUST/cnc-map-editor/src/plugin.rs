//! Game-specific editor plugins.
//!
//! A plugin supplies the theater list, buildable catalog, starting credits and
//! validation rules that differ between Tiberian Dawn and Red Alert. The editor
//! core stays game-agnostic and talks to this trait only.

use cnc_core::ClassId;
use crate::map::{MapModel, MapObject};

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
        model.in_bounds(object.position)
            && object.owner.map_or(true, |owner| owner.is_house())
    }

    fn get_object_name(&self, class_id: ClassId) -> String {
        format!("{class_id:?}")
    }

    /// Faction hint shown in the palette when an object is side-locked.
    fn object_side(&self, _class_id: ClassId) -> Option<&'static str> {
        None
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
        let mut templates = Vec::new();
        templates.push(("Victory", "structures:1:any:==:0", "win"));
        templates.push(("Defeat", "!has:0:FACT", "lose"));
        templates.push(("Chronosphere", "elapsed:600", "message:Chronosphere ready"));
        templates
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
        assert_eq!(td.object_side(ClassId::VehicleE1), None, "Jeep/Buggy share a class");
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
