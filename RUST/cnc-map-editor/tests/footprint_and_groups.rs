//! Footprint, bib, infantry group and property undo acceptance tests for parity-100.

use cnc_core::{ClassId, Vec2i};
use cnc_map_editor::history::SetObjectProperties;
use cnc_map_editor::{History, MapModel, ObjectTool, Tool};

fn place(model: &mut MapModel, history: &mut History, tool: &mut ObjectTool, cell: Vec2i) -> bool {
    let cmd = tool.click(model, cell);
    if let Some(c) = cmd {
        history.execute(model, c);
        true
    } else {
        false
    }
}

#[test]
fn building_footprint_occupies_cells() {
    let mut model = MapModel::new(8, 8);
    let mut history = History::new(16);
    let mut tool = ObjectTool::new(ClassId::ConstructionYard);
    // Place at (2,2)
    assert!(place(&mut model, &mut history, &mut tool, Vec2i::new(2, 2)));
    // The object exists
    assert!(model
        .objects
        .values()
        .any(|o| o.class_id == ClassId::ConstructionYard && o.position == Vec2i::new(2, 2)));
    // Attempt to place another object overlapping footprint (2,3) should be rejected
    let mut other = ObjectTool::new(ClassId::VehicleE1);
    assert!(
        !place(&mut model, &mut history, &mut other, Vec2i::new(3, 2)),
        "footprint cell (3,2) should be blocked"
    );
    assert!(
        !place(&mut model, &mut history, &mut other, Vec2i::new(2, 3)),
        "footprint cell (2,3) should be blocked"
    );
    // Bib row at (2,4) should also be blocked
    assert!(!place(
        &mut model,
        &mut history,
        &mut other,
        Vec2i::new(2, 4)
    ));
    assert!(!place(
        &mut model,
        &mut history,
        &mut other,
        Vec2i::new(3, 4)
    ));
}

#[test]
fn building_bib_blocks_placement() {
    let mut model = MapModel::new(6, 6);
    let mut history = History::new(16);
    let mut tool = ObjectTool::new(ClassId::Barracks);
    assert!(place(&mut model, &mut history, &mut tool, Vec2i::new(1, 1)));
    // Bib occupies row y=3 for x=1,2
    let mut other = ObjectTool::new(ClassId::InfantryE1);
    assert!(!place(
        &mut model,
        &mut history,
        &mut other,
        Vec2i::new(1, 3)
    ));
}

#[test]
fn infantry_group_limits_to_five_per_cell_with_sub_cell_slots() {
    let mut model = MapModel::new(8, 8);
    let mut history = History::new(16);
    let mut tool = ObjectTool::new(ClassId::InfantryE1);
    let cell = Vec2i::new(4, 4);
    for i in 0..5 {
        assert!(
            place(&mut model, &mut history, &mut tool, cell),
            "slot {i} should succeed"
        );
    }
    // Sixth should be rejected
    assert!(
        !place(&mut model, &mut history, &mut tool, cell),
        "sixth infantry should be rejected"
    );
    // Verify sub-cell slots are distinct 0..4
    let mut slots = std::collections::BTreeSet::new();
    for obj in model
        .objects
        .values()
        .filter(|o| o.position == cell && o.class_id == ClassId::InfantryE1)
    {
        let s = obj.sub_cell.unwrap_or(255);
        assert!(slots.insert(s), "duplicate sub-cell slot {s}");
    }
    assert_eq!(slots.len(), 5);
}

#[test]
fn property_edit_is_one_undo_step() {
    let mut model = MapModel::new(8, 8);
    let mut history = History::new(16);
    let mut tool = ObjectTool::new(ClassId::VehicleE1);
    place(&mut model, &mut history, &mut tool, Vec2i::new(0, 0));
    let id = model
        .objects
        .values()
        .find(|o| o.position == Vec2i::new(0, 0))
        .unwrap()
        .id;
    // Edit facing, strength, sub_cell together
    let cmd = SetObjectProperties::new(&model, id, 2, Some(75), Some(3)).expect("command");
    history.execute(&mut model, Box::new(cmd));
    let obj = model.objects.get(&id).unwrap();
    assert_eq!(obj.facing, 2);
    assert_eq!(obj.strength, Some(75));
    assert_eq!(obj.sub_cell, Some(3));
    // Undo should revert all three at once
    history.undo(&mut model);
    let obj = model.objects.get(&id).unwrap();
    assert_eq!(obj.facing, 0);
    assert_eq!(obj.strength, None);
    assert_eq!(obj.sub_cell, None);
    // Redo
    history.redo(&mut model);
    let obj = model.objects.get(&id).unwrap();
    assert_eq!(obj.facing, 2);
    assert_eq!(obj.strength, Some(75));
    assert_eq!(obj.sub_cell, Some(3));
}

#[test]
fn building_placement_rejects_overlap_with_existing_footprint() {
    let mut model = MapModel::new(10, 10);
    let mut history = History::new(16);
    let mut tool_a = ObjectTool::new(ClassId::WarFactory);
    assert!(place(
        &mut model,
        &mut history,
        &mut tool_a,
        Vec2i::new(5, 5)
    ));
    // Try to place another building overlapping footprint at (6,5)
    let mut tool_b = ObjectTool::new(ClassId::Refinery);
    assert!(
        !place(&mut model, &mut history, &mut tool_b, Vec2i::new(6, 5)),
        "overlapping footprint should be rejected"
    );
}
