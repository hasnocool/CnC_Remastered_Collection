//! C&C Map Editor
//!
//! Editable map model, undoable authoring tools, game plugins and a
//! renderer-agnostic editor view. All model changes go through
//! [`history::History`]; tools only build commands.

pub mod app;
/// Byte-exact original-format scenario fixtures for the importers.
pub mod fixtures;
pub mod generator;
pub mod history;
pub mod map;
pub mod pack_tileset;
pub mod plugin;
pub mod presenter;
pub mod tiles;
pub mod tools;
pub mod ui;

pub use generator::{fbm, generate, value_noise, MapGenSettings, OverlayType, TerrainType};
pub use history::{
    AddTrigger, AddWaypoint, EditCommand, EraseObject, History, MoveObject, PaintCells,
    PlaceObject, RemoveObject, RemoveTrigger, RemoveWaypoint, SetObjectOwner, SetTrigger,
};
pub use map::{MapModel, MapObject, Trigger, Waypoint, TILE_SIZE};
pub use plugin::{GamePlugin, RedAlertPlugin, Theater, TiberianDawnPlugin};
pub use tools::{
    line_cells, rect_cells, EraseTool, ObjectTool, SelectTool, TerrainTool, Tool, TriggerTool,
    WaypointTool,
};
pub use ui::EditorUI;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use cnc_core::{ClassId, Vec2i};

    #[test]
    fn authoring_session_roundtrips_through_ini() {
        let mut model = MapModel::new(16, 16);
        model.theater = "Temperate".to_string();
        model.player_starts.push(Vec2i::new(2, 2));

        let mut editor = EditorUI::new();
        editor.set_tool(Box::new(TerrainTool::new(1)));
        assert!(editor.click(&mut model, Vec2i::new(4, 4)));

        editor.set_tool(Box::new(ObjectTool::with_owner(
            ClassId::ConstructionYard,
            ClassId::HouseGDI,
        )));
        assert!(editor.click(&mut model, Vec2i::new(5, 5)));

        editor.set_tool(Box::new(TriggerTool::new("win", "always", "win")));
        assert!(editor.click(&mut model, Vec2i::new(1, 1)));

        let text = model.to_ini();
        let loaded = MapModel::from_ini(&text).unwrap();
        assert_eq!(loaded.terrain[4 * 16 + 4], 1);
        assert_eq!(loaded.objects.len(), 1);
        assert_eq!(loaded.triggers.len(), 1);
        assert!(loaded.validate().is_empty(), "{:?}", loaded.validate());

        // Undo after loading still works on the live model.
        editor.undo(&mut model);
        assert!(model.triggers.is_empty());
    }
}
