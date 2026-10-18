//! C&C Map Editor

pub mod map;
pub mod plugin;
pub mod tools;
pub mod ui;

pub use map::MapModel;
pub use plugin::{GamePlugin, TiberianDawnPlugin, RedAlertPlugin};
pub use tools::{Tool, TerrainTool, ObjectTool, TriggerTool};
pub use ui::EditorUI;
