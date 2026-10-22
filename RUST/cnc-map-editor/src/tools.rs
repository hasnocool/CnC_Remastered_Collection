//! Authoring tools.
//!
//! Tools never mutate the [`MapModel`]. They inspect the model and return an
//! [`EditCommand`] for the caller to run through [`crate::history::History`].

use crate::history::{
    AddTrigger, AddWaypoint, EditCommand, EraseObject, PaintCells, PlaceObject, RemoveWaypoint,
};
use crate::map::{MapModel, MapObject, Trigger};
use cnc_core::{ClassId, Vec2i};

/// A primary mouse action: click or drag.
pub trait Tool {
    fn name(&self) -> &str;

    /// Build the command for one click; `None` means the click is a no-op.
    fn click(&mut self, model: &MapModel, cell: Vec2i) -> Option<Box<dyn EditCommand>>;

    /// Build the command for a drag gesture (line interpolation by default).
    fn drag(&mut self, model: &MapModel, from: Vec2i, to: Vec2i) -> Option<Box<dyn EditCommand>> {
        let _ = (from, to);
        let _ = model;
        None
    }
}

/// Brush that paints terrain or overlay ids.
pub struct TerrainTool {
    pub terrain_type: u8,
    pub overlay: bool,
}

impl TerrainTool {
    pub fn new(terrain_type: u8) -> Self {
        Self {
            terrain_type,
            overlay: false,
        }
    }

    pub fn overlay(overlay_type: u8) -> Self {
        Self {
            terrain_type: overlay_type,
            overlay: true,
        }
    }

    fn cells(&self, model: &MapModel, from: Vec2i, to: Vec2i) -> Vec<usize> {
        line_cells(from, to)
            .into_iter()
            .filter_map(|cell| model.index(cell))
            .collect()
    }

    fn paint(&self, model: &MapModel, cells: Vec<usize>) -> Box<dyn EditCommand> {
        if self.overlay {
            Box::new(PaintCells::overlay(model, cells, self.terrain_type))
        } else {
            Box::new(PaintCells::terrain(model, cells, self.terrain_type))
        }
    }
}

impl Tool for TerrainTool {
    fn name(&self) -> &str {
        "Terrain"
    }

    fn click(&mut self, model: &MapModel, cell: Vec2i) -> Option<Box<dyn EditCommand>> {
        let index = model.index(cell)?;
        Some(self.paint(model, vec![index]))
    }

    fn drag(&mut self, model: &MapModel, from: Vec2i, to: Vec2i) -> Option<Box<dyn EditCommand>> {
        let cells = self.cells(model, from, to);
        if cells.is_empty() {
            return None;
        }
        Some(self.paint(model, cells))
    }
}

fn is_infantry_class(class_id: ClassId) -> bool {
    matches!(
        class_id,
        ClassId::Infantry
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
    )
}

fn building_footprint(class_id: ClassId) -> (usize, usize, bool) {
    match class_id {
        ClassId::ConstructionYard
        | ClassId::WarFactory
        | ClassId::Barracks
        | ClassId::Refinery
        | ClassId::PowerPlant
        | ClassId::Silo
        | ClassId::Radar => (2, 2, true),
        _ if class_id.is_building() => (1, 1, false),
        _ => (1, 1, false),
    }
}

fn footprint_cells(origin: Vec2i, w: usize, h: usize, has_bib: bool) -> Vec<Vec2i> {
    let mut cells = Vec::new();
    for y in 0..h {
        for x in 0..w {
            cells.push(Vec2i::new(origin.x + x as i32, origin.y + y as i32));
        }
    }
    if has_bib {
        for x in 0..w {
            cells.push(Vec2i::new(origin.x + x as i32, origin.y + h as i32));
        }
    }
    cells
}

fn object_footprint_cells(object: &crate::map::MapObject) -> Vec<Vec2i> {
    if object.class_id.is_building() {
        let (w, h, has_bib) = building_footprint(object.class_id);
        footprint_cells(object.position, w, h, has_bib)
    } else {
        Vec::new()
    }
}

/// Brush that places a new unit or structure.
pub struct ObjectTool {
    pub class_id: ClassId,
    pub owner: Option<ClassId>,
}

impl ObjectTool {
    pub fn new(class_id: ClassId) -> Self {
        Self {
            class_id,
            owner: None,
        }
    }

    pub fn with_owner(class_id: ClassId, owner: ClassId) -> Self {
        Self {
            class_id,
            owner: Some(owner),
        }
    }
}

impl Tool for ObjectTool {
    fn name(&self) -> &str {
        "Object"
    }

    fn click(&mut self, model: &MapModel, cell: Vec2i) -> Option<Box<dyn EditCommand>> {
        if !model.in_bounds(cell) {
            return None;
        }
        if self.class_id.is_building() {
            let (w, h, has_bib) = building_footprint(self.class_id);
            let cells = footprint_cells(cell, w, h, has_bib);
            for c in &cells {
                if !model.in_bounds(*c) {
                    return None;
                }
            }
            for obj in model.objects.values() {
                let existing = object_footprint_cells(obj);
                for ec in &existing {
                    if cells.contains(ec) {
                        return None;
                    }
                }
            }
            for c in &cells {
                if model.object_at(*c).is_some() {
                    return None;
                }
            }
            Some(Box::new(PlaceObject::new(MapObject {
                id: 0,
                class_id: self.class_id,
                position: cell,
                owner: self.owner,
                ..Default::default()
            })))
        } else if is_infantry_class(self.class_id) {
            for obj in model.objects.values() {
                let existing = object_footprint_cells(obj);
                if existing.contains(&cell) {
                    return None;
                }
            }
            let infantry_count = model
                .objects
                .values()
                .filter(|o| o.position == cell && is_infantry_class(o.class_id))
                .count();
            if infantry_count >= 5 {
                return None;
            }
            let used: std::collections::BTreeSet<u8> = model
                .objects
                .values()
                .filter(|o| o.position == cell && is_infantry_class(o.class_id))
                .filter_map(|o| o.sub_cell)
                .collect();
            let slot = (0..5u8).find(|s| !used.contains(s))?;
            Some(Box::new(PlaceObject::new(MapObject {
                id: 0,
                class_id: self.class_id,
                position: cell,
                owner: self.owner,
                sub_cell: Some(slot),
                ..Default::default()
            })))
        } else {
            for obj in model.objects.values() {
                let existing = object_footprint_cells(obj);
                if existing.contains(&cell) {
                    return None;
                }
            }
            if model.object_at(cell).is_some() {
                return None;
            }
            Some(Box::new(PlaceObject::new(MapObject {
                id: 0,
                class_id: self.class_id,
                position: cell,
                owner: self.owner,
                ..Default::default()
            })))
        }
    }
}

/// Eraser: deletes the object under the cursor.
pub struct EraseTool;

impl Tool for EraseTool {
    fn name(&self) -> &str {
        "Erase"
    }

    fn click(&mut self, model: &MapModel, cell: Vec2i) -> Option<Box<dyn EditCommand>> {
        let object = model.object_at(cell)?;
        Some(Box::new(EraseObject::new(object.id)))
    }
}

/// Selection tool: clicking an object arms the drag-to-move gesture handled by
/// the app layer, so this tool itself never builds a command.
pub struct SelectTool;

impl Tool for SelectTool {
    fn name(&self) -> &str {
        "Select"
    }

    fn click(&mut self, _model: &MapModel, _cell: Vec2i) -> Option<Box<dyn EditCommand>> {
        None
    }
}

/// Places scenario triggers.
pub struct TriggerTool {
    pub name: String,
    pub condition: String,
    pub action: String,
}

impl TriggerTool {
    pub fn new(
        name: impl Into<String>,
        condition: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            condition: condition.into(),
            action: action.into(),
        }
    }
}

impl Default for TriggerTool {
    fn default() -> Self {
        Self::new("New Trigger", "Always", "None")
    }
}

impl Tool for TriggerTool {
    fn name(&self) -> &str {
        "Trigger"
    }

    fn click(&mut self, model: &MapModel, cell: Vec2i) -> Option<Box<dyn EditCommand>> {
        if !model.in_bounds(cell) {
            return None;
        }
        Some(Box::new(AddTrigger {
            trigger: Trigger {
                id: 0,
                name: format!("{} ({},{})", self.name, cell.x, cell.y),
                condition: self.condition.clone(),
                action: self.action.clone(),
                group: String::new(),
            },
        }))
    }
}

/// Places scenario waypoints, or erases the one under the cursor.
///
/// The numeric id is the key written under [Waypoints]; leave it `None` to
/// take the lowest free id (A, B, C, ...). Duplicate ids are rejected so the
/// stored section can never carry two entries for the same waypoint.
pub struct WaypointTool {
    pub id: Option<u32>,
    pub erase: bool,
}

impl WaypointTool {
    /// Placement tool; `id` pins a waypoint letter, `None` auto-assigns.
    pub fn new(id: Option<u32>) -> Self {
        Self { id, erase: false }
    }

    pub fn eraser() -> Self {
        Self {
            id: None,
            erase: true,
        }
    }
}

impl Tool for WaypointTool {
    fn name(&self) -> &str {
        "Waypoint"
    }

    fn click(&mut self, model: &MapModel, cell: Vec2i) -> Option<Box<dyn EditCommand>> {
        if !model.in_bounds(cell) {
            return None;
        }
        if self.erase {
            let waypoint = model.waypoint_at(cell)?;
            return Some(Box::new(RemoveWaypoint::new(model, waypoint.id)?));
        }
        let id = self.id.unwrap_or_else(|| model.next_waypoint_id());
        Some(Box::new(AddWaypoint::new(model, id, cell)?))
    }
}

/// Integer Bresenham line, inclusive of both endpoints.
pub fn line_cells(from: Vec2i, to: Vec2i) -> Vec<Vec2i> {
    let mut cells = Vec::new();
    let mut x = from.x;
    let mut y = from.y;
    let dx = (to.x - from.x).abs();
    let dy = -(to.y - from.y).abs();
    let sx = if from.x < to.x { 1 } else { -1 };
    let sy = if from.y < to.y { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        cells.push(Vec2i::new(x, y));
        if x == to.x && y == to.y {
            break;
        }
        let doubled = 2 * error;
        if doubled >= dy {
            error += dy;
            x += sx;
        }
        if doubled <= dx {
            error += dx;
            y += sy;
        }
    }
    cells
}

/// Axis-aligned filled rectangle, inclusive of both corners.
pub fn rect_cells(from: Vec2i, to: Vec2i) -> Vec<Vec2i> {
    let (x0, x1) = (from.x.min(to.x), from.x.max(to.x));
    let (y0, y1) = (from.y.min(to.y), from.y.max(to.y));
    let mut cells = Vec::new();
    for y in y0..=y1 {
        for x in x0..=x1 {
            cells.push(Vec2i::new(x, y));
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::map::{MapModel, Waypoint};

    #[test]
    fn terrain_click_builds_a_command_without_mutating() {
        let model = MapModel::new(4, 4);
        let mut tool = TerrainTool::new(5);
        let command = tool.click(&model, Vec2i::new(1, 1)).unwrap();
        assert_eq!(model.terrain[5], 0, "tool must not mutate directly");

        let mut model = model;
        let mut history = History::new(4);
        history.execute(&mut model, command);
        assert_eq!(model.terrain[5], 5);
        history.undo(&mut model);
        assert_eq!(model.terrain[5], 0);
    }

    #[test]
    fn drag_paints_a_line() {
        let model = MapModel::new(8, 8);
        let mut tool = TerrainTool::new(7);
        let command = tool.drag(&model, Vec2i::new(0, 0), Vec2i::new(3, 0));
        let mut model = model;
        let mut history = History::new(4);
        history.execute(&mut model, command.unwrap());
        for x in 0..4 {
            assert_eq!(model.terrain[x], 7);
        }
        assert_eq!(line_cells(Vec2i::new(0, 0), Vec2i::new(2, 2)).len(), 3);
        assert_eq!(rect_cells(Vec2i::new(0, 0), Vec2i::new(1, 1)).len(), 4);
    }

    #[test]
    fn object_tool_rejects_out_of_bounds_and_occupied_cells() {
        let mut model = MapModel::new(8, 8);
        let mut tool = ObjectTool::new(ClassId::VehicleE1);
        assert!(tool.click(&model, Vec2i::new(100, 100)).is_none());

        let command = tool.click(&model, Vec2i::new(3, 3)).unwrap();
        let mut history = History::new(4);
        history.execute(&mut model, command);
        assert!(
            tool.click(&model, Vec2i::new(3, 3)).is_none(),
            "cell is occupied"
        );
    }

    #[test]
    fn trigger_tool_is_bounds_checked() {
        let model = MapModel::new(4, 4);
        let mut tool = TriggerTool::default();
        assert!(tool.click(&model, Vec2i::new(-1, 0)).is_none());
        assert!(tool.click(&model, Vec2i::new(1, 1)).is_some());
    }

    #[test]
    fn waypoint_click_places_undoes_and_redoes_exactly() {
        let model = MapModel::new(8, 8);
        let mut tool = WaypointTool::new(None);
        let command = tool.click(&model, Vec2i::new(2, 3)).expect("command");
        assert!(
            model.waypoints.is_empty(),
            "the tool must not mutate the model directly"
        );

        let mut model = model;
        let mut history = History::new(4);
        history.execute(&mut model, command);
        assert_eq!(
            model.waypoints,
            vec![Waypoint {
                id: 0,
                cell: Vec2i::new(2, 3)
            }]
        );
        history.undo(&mut model);
        assert!(model.waypoints.is_empty());
        history.redo(&mut model);
        assert_eq!(
            model.waypoints,
            vec![Waypoint {
                id: 0,
                cell: Vec2i::new(2, 3)
            }],
            "redo must restore the exact id and cell"
        );
    }

    #[test]
    fn waypoint_ids_are_assigned_in_letter_order_and_erased_cleanly() {
        let mut model = MapModel::new(8, 8);
        let mut tool = WaypointTool::new(None);
        let mut history = History::new(8);
        for cell in [Vec2i::new(1, 1), Vec2i::new(2, 2), Vec2i::new(3, 3)] {
            let command = tool.click(&model, cell).expect("command");
            history.execute(&mut model, command);
        }
        let ids: Vec<u32> = model.waypoints.iter().map(|waypoint| waypoint.id).collect();
        assert_eq!(ids, vec![0, 1, 2]);
        assert_eq!(model.waypoint(0).unwrap().label(), "A");
        assert_eq!(model.waypoint(2).unwrap().label(), "C");

        let mut eraser = WaypointTool::eraser();
        let command = eraser.click(&model, Vec2i::new(2, 2)).expect("command");
        history.execute(&mut model, command);
        assert!(model.waypoint(1).is_none());
        history.undo(&mut model);
        assert_eq!(model.waypoint(1).unwrap().cell, Vec2i::new(2, 2));
    }

    #[test]
    fn waypoint_click_rejects_out_of_bounds_and_duplicate_ids() {
        let mut model = MapModel::new(4, 4);
        let mut tool = WaypointTool::new(Some(3));
        assert!(
            tool.click(&model, Vec2i::new(100, 100)).is_none(),
            "out-of-bounds placement is rejected"
        );
        let command = tool.click(&model, Vec2i::new(1, 1)).expect("first place");
        let mut history = History::new(4);
        history.execute(&mut model, command);
        assert!(
            tool.click(&model, Vec2i::new(2, 2)).is_none(),
            "an already-used waypoint id is rejected"
        );
        let mut auto = WaypointTool::new(None);
        let command = auto.click(&model, Vec2i::new(2, 2)).expect("auto id");
        history.execute(&mut model, command);
        assert_eq!(model.waypoints.len(), 2);
    }
}
