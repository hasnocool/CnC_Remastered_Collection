//! Waypoint authoring: placement tool, undo/redo and [Waypoints] round-trip.

use cnc_core::Vec2i;
use cnc_map_editor::{
    AddWaypoint, History, MapModel, RemoveWaypoint, Tool, Waypoint, WaypointTool,
};

fn place(model: &mut MapModel, history: &mut History, tool: &mut dyn Tool, cell: Vec2i) {
    let command = tool.click(model, cell).expect("placement command");
    history.execute(model, command);
}

#[test]
fn waypoints_roundtrip_through_ini_save_and_load() {
    let mut model = MapModel::new(16, 16);
    model.player_starts.push(Vec2i::new(0, 0));
    let mut history = History::new(16);
    let mut tool = WaypointTool::new(None);
    place(&mut model, &mut history, &mut tool, Vec2i::new(1, 1)); // A -> cell 17
    place(&mut model, &mut history, &mut tool, Vec2i::new(0, 15)); // B -> cell 240
    place(&mut model, &mut history, &mut tool, Vec2i::new(15, 15)); // C -> cell 255

    let text = model.to_ini();
    let section = text
        .split("[Waypoints]\n")
        .nth(1)
        .and_then(|rest| rest.split('[').next())
        .expect("[Waypoints] section");
    assert!(section.contains("0=17"), "A writes cell 17: {section}");
    assert!(section.contains("1=240"), "B writes cell 240: {section}");
    assert!(section.contains("2=255"), "C writes cell 255: {section}");

    let loaded = MapModel::from_ini(&text).expect("reload");
    assert_eq!(loaded.waypoints, model.waypoints);
    assert_eq!(loaded.waypoint(0).unwrap().cell, Vec2i::new(1, 1));
    assert_eq!(loaded.waypoint(1).unwrap().cell, Vec2i::new(0, 15));
    assert_eq!(loaded.waypoint(2).unwrap().label(), "C");
    assert!(loaded.validate().is_empty(), "{:?}", loaded.validate());
}

#[test]
fn waypoint_placement_is_undoable_to_an_exact_redo() {
    let mut model = MapModel::new(8, 8);
    let mut history = History::new(8);
    let mut tool = WaypointTool::new(Some(4));
    place(&mut model, &mut history, &mut tool, Vec2i::new(5, 6));
    assert_eq!(model.waypoint(4).unwrap().cell, Vec2i::new(5, 6));

    history.undo(&mut model);
    assert!(model.waypoints.is_empty(), "undo removes the waypoint");
    history.redo(&mut model);
    assert_eq!(
        model.waypoints,
        vec![Waypoint {
            id: 4,
            cell: Vec2i::new(5, 6)
        }],
        "redo restores the exact id and cell"
    );

    let mut eraser = WaypointTool::eraser();
    place(&mut model, &mut history, &mut eraser, Vec2i::new(5, 6));
    assert!(model.waypoints.is_empty(), "eraser removes the waypoint");
    history.undo(&mut model);
    assert_eq!(model.waypoints.len(), 1);
}

#[test]
fn waypoints_outside_the_map_are_rejected() {
    let model = MapModel::new(4, 4);
    let mut tool = WaypointTool::new(None);
    assert!(tool.click(&model, Vec2i::new(4, 0)).is_none());
    assert!(tool.click(&model, Vec2i::new(-1, 0)).is_none());
    assert!(AddWaypoint::new(&model, 0, Vec2i::new(100, 100)).is_none());
    assert!(RemoveWaypoint::new(&model, 0).is_none());
    assert!(model.waypoints.is_empty());
}

#[test]
fn original_ra_import_reads_the_waypoints_section() {
    // Original layout: numeric key = waypoint id, value = row-major cell.
    let mut text = String::from("[Basic]\nName=WP\n");
    // Side is inferred from the highest packed index, so anchor it at 255 to
    // get a 16x16 grid (real maps always pack every cell).
    let cells = [
        cnc_asset::ra_pack::PackedCell { index: 0, id: 11 },
        cnc_asset::ra_pack::PackedCell { index: 255, id: 22 },
    ];
    text.push_str("[MapPack]\n");
    for (key, value) in cnc_asset::ra_pack::encode_pack(&cells) {
        text.push_str(&format!("{key}={value}\n"));
    }
    text.push_str("[Waypoints]\n0=17\n1=240\n2=-1\n");

    let (model, kind) = MapModel::import_text(&text).expect("import");
    assert_eq!(kind, cnc_map_editor::map::ImportKind::OriginalRa);
    assert_eq!(model.width, 16);
    assert_eq!(model.waypoint(0).unwrap().cell, Vec2i::new(1, 1));
    assert_eq!(model.waypoint(1).unwrap().cell, Vec2i::new(0, 15));
    assert!(
        model.waypoint(2).is_none(),
        "a -1 cell means the waypoint is unset"
    );
}
