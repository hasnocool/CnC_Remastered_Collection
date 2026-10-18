// Plugin System
use cnc_core::*;
use crate::map::{MapModel, MapObject};

pub trait GamePlugin: Send + Sync {
    fn name(&self) -> &str;
    fn available_terrains(&self) -> Vec<String>;
    fn available_objects(&self) -> Vec<ClassId>;
    fn validate_object(&self, _model: &MapModel, _object: &MapObject) -> bool { true }
    fn get_object_name(&self, class_id: ClassId) -> String { format!("{:?}", class_id) }
}

pub struct TiberianDawnPlugin;

impl GamePlugin for TiberianDawnPlugin {
    fn name(&self) -> &str { "Tiberian Dawn" }
    fn available_terrains(&self) -> Vec<String> { vec!["Temperate".into(), "Snow".into(), "Desert".into()] }
    fn available_objects(&self) -> Vec<ClassId> { vec![ClassId::InfantryE1, ClassId::VehicleE1, ClassId::ConstructionYard] }
}

pub struct RedAlertPlugin;

impl GamePlugin for RedAlertPlugin {
    fn name(&self) -> &str { "Red Alert" }
    fn available_terrains(&self) -> Vec<String> { vec!["Temperate".into(), "Snow".into(), "Urban".into()] }
    fn available_objects(&self) -> Vec<ClassId> { vec![ClassId::InfantryE1, ClassId::VehicleE1, ClassId::ShipE1, ClassId::ConstructionYard] }
}
