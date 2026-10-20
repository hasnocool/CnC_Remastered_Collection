//! Undo/redo command stack.
//!
//! Every editor mutation is an [`EditCommand`]. Tools only *build* commands;
//! [`History::execute`] applies them, so the model can never be mutated outside
//! the undo system (see AGENTS.md). Commands capture the state they replace so
//! `undo` is exact, including object ids.

use cnc_ai::AgentDefinition;
use cnc_core::ClassId;

use crate::map::{MapModel, MapObject, Trigger};

/// One reversible model mutation.
pub trait EditCommand: Send {
    /// Human-readable name for the history panel.
    fn label(&self) -> String;
    fn apply(&mut self, model: &mut MapModel);
    fn undo(&mut self, model: &mut MapModel);
}

/// Bounded undo/redo stacks.
/// A group of commands recorded as one undo step (e.g. one brush stroke).
pub struct GroupCommand {
    pub label: String,
    pub commands: Vec<Box<dyn EditCommand>>,
}

impl EditCommand for GroupCommand {
    fn label(&self) -> String {
        self.label.clone()
    }

    fn apply(&mut self, model: &mut MapModel) {
        for command in &mut self.commands {
            command.apply(model);
        }
    }

    fn undo(&mut self, model: &mut MapModel) {
        for command in self.commands.iter_mut().rev() {
            command.undo(model);
        }
    }
}

struct OpenGroup {
    label: String,
    commands: Vec<Box<dyn EditCommand>>,
}

#[derive(Default)]
pub struct History {
    undo_stack: Vec<Box<dyn EditCommand>>,
    redo_stack: Vec<Box<dyn EditCommand>>,
    group: Option<OpenGroup>,
    limit: usize,
}

impl History {
    pub fn new(limit: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            group: None,
            limit: limit.max(1),
        }
    }

    /// Apply a command and push it onto the undo stack (or the open group).
    pub fn execute(&mut self, model: &mut MapModel, mut command: Box<dyn EditCommand>) {
        command.apply(model);
        if let Some(group) = self.group.as_mut() {
            group.commands.push(command);
            return;
        }
        self.push_undo(command);
    }

    /// Start coalescing every executed command into one undo step.
    pub fn begin_group(&mut self, label: impl Into<String>) {
        if self.group.is_none() {
            self.group = Some(OpenGroup {
                label: label.into(),
                commands: Vec::new(),
            });
        }
    }

    /// Close the open group, pushing it as a single undo step.
    pub fn end_group(&mut self) {
        let Some(group) = self.group.take() else {
            return;
        };
        if group.commands.is_empty() {
            return;
        }
        self.push_undo(Box::new(GroupCommand {
            label: group.label,
            commands: group.commands,
        }));
    }

    pub fn in_group(&self) -> bool {
        self.group.is_some()
    }

    fn push_undo(&mut self, command: Box<dyn EditCommand>) {
        self.undo_stack.push(command);
        self.redo_stack.clear();
        if self.undo_stack.len() > self.limit {
            self.undo_stack.remove(0);
        }
    }

    pub fn undo(&mut self, model: &mut MapModel) -> Option<String> {
        let mut command = self.undo_stack.pop()?;
        command.undo(model);
        let label = command.label();
        self.redo_stack.push(command);
        Some(label)
    }

    pub fn redo(&mut self, model: &mut MapModel) -> Option<String> {
        let mut command = self.redo_stack.pop()?;
        command.apply(model);
        let label = command.label();
        self.undo_stack.push(command);
        Some(label)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn undo_len(&self) -> usize {
        self.undo_stack.len()
    }

    pub fn redo_len(&self) -> usize {
        self.redo_stack.len()
    }

    pub fn next_undo_label(&self) -> Option<String> {
        self.undo_stack.last().map(|command| command.label())
    }

    pub fn next_redo_label(&self) -> Option<String> {
        self.redo_stack.last().map(|command| command.label())
    }

    /// Undo-stack labels, most recent first (for the history panel).
    pub fn undo_labels(&self) -> Vec<String> {
        self.undo_stack
            .iter()
            .rev()
            .map(|command| command.label())
            .collect()
    }

    /// Redo-stack labels, next redo first.
    pub fn redo_labels(&self) -> Vec<String> {
        self.redo_stack
            .iter()
            .rev()
            .map(|command| command.label())
            .collect()
    }
}

/// Paint terrain or overlay cells back to captured values.
pub struct PaintCells {
    /// (cell index, old value, new value)
    pub cells: Vec<(usize, u8, u8)>,
    pub overlay: bool,
}

impl PaintCells {
    pub fn terrain(model: &MapModel, cells: impl IntoIterator<Item = usize>, value: u8) -> Self {
        Self::build(model, cells, value, false)
    }

    pub fn overlay(model: &MapModel, cells: impl IntoIterator<Item = usize>, value: u8) -> Self {
        Self::build(model, cells, value, true)
    }

    fn build(
        model: &MapModel,
        cells: impl IntoIterator<Item = usize>,
        value: u8,
        overlay: bool,
    ) -> Self {
        let source = if overlay {
            &model.overlay
        } else {
            &model.terrain
        };
        let cells = cells
            .into_iter()
            .filter(|index| *index < source.len())
            .map(|index| (index, source[index], value))
            .collect();
        Self { cells, overlay }
    }

    fn write(model: &mut MapModel, cells: &[(usize, u8, u8)], new: bool, overlay: bool) {
        let target = if overlay {
            &mut model.overlay
        } else {
            &mut model.terrain
        };
        for (index, old, value) in cells {
            if *index < target.len() {
                target[*index] = if new { *value } else { *old };
            }
        }
    }
}

impl EditCommand for PaintCells {
    fn label(&self) -> String {
        if self.overlay {
            format!("Paint overlay ({} cells)", self.cells.len())
        } else {
            format!("Paint terrain ({} cells)", self.cells.len())
        }
    }

    fn apply(&mut self, model: &mut MapModel) {
        Self::write(model, &self.cells, true, self.overlay);
    }

    fn undo(&mut self, model: &mut MapModel) {
        Self::write(model, &self.cells, false, self.overlay);
    }
}

/// Insert an object. A zero id is allocated by the model on first apply.
pub struct PlaceObject {
    pub object: MapObject,
}

impl PlaceObject {
    pub fn new(object: MapObject) -> Self {
        Self { object }
    }
}

impl EditCommand for PlaceObject {
    fn label(&self) -> String {
        format!("Place {}", self.object.class_name())
    }

    fn apply(&mut self, model: &mut MapModel) {
        self.object.id = model.insert_object(self.object.clone());
    }

    fn undo(&mut self, model: &mut MapModel) {
        model.remove_object(self.object.id);
    }
}

/// Remove an existing object, restoring it with its original id on undo.
pub struct RemoveObject {
    pub object: MapObject,
}

impl RemoveObject {
    pub fn new(model: &MapModel, id: u32) -> Option<Self> {
        model
            .objects
            .get(&id)
            .cloned()
            .map(|object| Self { object })
    }
}

impl EditCommand for RemoveObject {
    fn label(&self) -> String {
        format!("Delete {}", self.object.class_name())
    }

    fn apply(&mut self, model: &mut MapModel) {
        model.remove_object(self.object.id);
    }

    fn undo(&mut self, model: &mut MapModel) {
        model.insert_object(self.object.clone());
    }
}

/// Erase an object by id, remembering it for undo.
pub struct EraseObject {
    pub id: u32,
    removed: Option<MapObject>,
}

impl EraseObject {
    pub fn new(id: u32) -> Self {
        Self { id, removed: None }
    }
}

impl EditCommand for EraseObject {
    fn label(&self) -> String {
        "Erase object".to_string()
    }

    fn apply(&mut self, model: &mut MapModel) {
        self.removed = model.remove_object(self.id);
    }

    fn undo(&mut self, model: &mut MapModel) {
        if let Some(object) = self.removed.take() {
            model.insert_object(object);
        }
    }
}

/// Move an object between two cells.
pub struct MoveObject {
    pub id: u32,
    pub from: cnc_core::Vec2i,
    pub to: cnc_core::Vec2i,
}

impl EditCommand for MoveObject {
    fn label(&self) -> String {
        "Move object".to_string()
    }

    fn apply(&mut self, model: &mut MapModel) {
        if let Some(object) = model.objects.get_mut(&self.id) {
            object.position = self.to;
        }
    }

    fn undo(&mut self, model: &mut MapModel) {
        if let Some(object) = model.objects.get_mut(&self.id) {
            object.position = self.from;
        }
    }
}

/// Change an object's owning house.
pub struct SetObjectOwner {
    pub id: u32,
    pub before: Option<cnc_core::ClassId>,
    pub after: Option<cnc_core::ClassId>,
}

impl SetObjectOwner {
    pub fn new(model: &MapModel, id: u32, after: Option<cnc_core::ClassId>) -> Option<Self> {
        let before = model.objects.get(&id)?.owner;
        (before != after).then_some(Self { id, before, after })
    }
}

impl EditCommand for SetObjectOwner {
    fn label(&self) -> String {
        "Set object owner".to_string()
    }

    fn apply(&mut self, model: &mut MapModel) {
        if let Some(object) = model.objects.get_mut(&self.id) {
            object.owner = self.after;
        }
    }

    fn undo(&mut self, model: &mut MapModel) {
        if let Some(object) = model.objects.get_mut(&self.id) {
            object.owner = self.before;
        }
    }
}

/// Add a trigger.
pub struct AddTrigger {
    pub trigger: Trigger,
}

impl EditCommand for AddTrigger {
    fn label(&self) -> String {
        format!("Add trigger {}", self.trigger.name)
    }

    fn apply(&mut self, model: &mut MapModel) {
        self.trigger.id = model.add_trigger(self.trigger.clone());
    }

    fn undo(&mut self, model: &mut MapModel) {
        model.remove_trigger(self.trigger.id);
    }
}

/// Remove a trigger.
pub struct RemoveTrigger {
    pub trigger: Trigger,
}

impl RemoveTrigger {
    pub fn new(model: &MapModel, id: u32) -> Option<Self> {
        model
            .triggers
            .iter()
            .find(|trigger| trigger.id == id)
            .cloned()
            .map(|trigger| Self { trigger })
    }
}

impl EditCommand for RemoveTrigger {
    fn label(&self) -> String {
        format!("Delete trigger {}", self.trigger.name)
    }

    fn apply(&mut self, model: &mut MapModel) {
        model.remove_trigger(self.trigger.id);
    }

    fn undo(&mut self, model: &mut MapModel) {
        model.add_trigger(self.trigger.clone());
    }
}

/// Change the class of a set of placed objects, preserving ids/owners/cells.
pub struct SetObjectClass {
    pub before: Vec<(u32, ClassId)>,
    pub after: ClassId,
}

impl SetObjectClass {
    pub fn new(model: &MapModel, ids: &[u32], after: ClassId) -> Option<Self> {
        let mut before = Vec::new();
        for id in ids {
            if let Some(object) = model.objects.get(id) {
                before.push((*id, object.class_id));
            }
        }
        if before.is_empty() || before.iter().all(|(_, class)| *class == after) {
            None
        } else {
            Some(Self { before, after })
        }
    }
}

impl EditCommand for SetObjectClass {
    fn label(&self) -> String {
        format!("Change class of {} object(s)", self.before.len())
    }

    fn apply(&mut self, model: &mut MapModel) {
        for (id, _) in &self.before {
            if let Some(object) = model.objects.get_mut(id) {
                object.class_id = self.after;
            }
        }
    }

    fn undo(&mut self, model: &mut MapModel) {
        for (id, class) in &self.before {
            if let Some(object) = model.objects.get_mut(id) {
                object.class_id = *class;
            }
        }
    }
}

/// Editable scenario metadata (name, side, briefing and objectives).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioInfo {
    pub name: String,
    pub player: String,
    pub briefing: String,
    pub objectives: Vec<String>,
}

impl ScenarioInfo {
    pub fn from_model(model: &MapModel) -> Self {
        Self {
            name: model.name.clone(),
            player: model.player.clone(),
            briefing: model.briefing.clone(),
            objectives: model.objectives.clone(),
        }
    }
}

/// Replace the scenario metadata in one undoable step.
pub struct SetScenarioInfo {
    pub before: ScenarioInfo,
    pub after: ScenarioInfo,
}

impl SetScenarioInfo {
    pub fn new(before: ScenarioInfo, after: ScenarioInfo) -> Option<Self> {
        if before == after {
            None
        } else {
            Some(Self { before, after })
        }
    }
}

impl EditCommand for SetScenarioInfo {
    fn label(&self) -> String {
        "Edit scenario info".to_string()
    }

    fn apply(&mut self, model: &mut MapModel) {
        model.name = self.after.name.clone();
        model.player = self.after.player.clone();
        model.briefing = self.after.briefing.clone();
        model.objectives = self.after.objectives.clone();
    }

    fn undo(&mut self, model: &mut MapModel) {
        model.name = self.before.name.clone();
        model.player = self.before.player.clone();
        model.briefing = self.before.briefing.clone();
        model.objectives = self.before.objectives.clone();
    }
}

/// Edit an existing trigger's authored fields (name, condition, action).
pub struct SetTrigger {
    pub before: Trigger,
    pub after: Trigger,
}

impl SetTrigger {
    /// Build a command when the edited trigger differs from the stored one.
    pub fn new(model: &MapModel, after: Trigger) -> Option<Self> {
        let before = model
            .triggers
            .iter()
            .find(|trigger| trigger.id == after.id)?
            .clone();
        if before == after {
            None
        } else {
            Some(Self { before, after })
        }
    }
}

impl EditCommand for SetTrigger {
    fn label(&self) -> String {
        format!("Edit trigger {}", self.after.name)
    }

    fn apply(&mut self, model: &mut MapModel) {
        if let Some(slot) = model
            .triggers
            .iter_mut()
            .find(|trigger| trigger.id == self.after.id)
        {
            *slot = self.after.clone();
        }
    }

    fn undo(&mut self, model: &mut MapModel) {
        if let Some(slot) = model
            .triggers
            .iter_mut()
            .find(|trigger| trigger.id == self.before.id)
        {
            *slot = self.before.clone();
        }
    }
}

/// Add an AI agent.
pub struct AddAgent {
    pub agent: AgentDefinition,
}

impl EditCommand for AddAgent {
    fn label(&self) -> String {
        format!("Add agent {}", self.agent.name)
    }

    fn apply(&mut self, model: &mut MapModel) {
        self.agent.id = model.add_agent(self.agent.clone());
    }

    fn undo(&mut self, model: &mut MapModel) {
        model.remove_agent(self.agent.id);
    }
}

/// Delete an AI agent, remembering its position for an exact undo.
pub struct RemoveAgent {
    pub agent: AgentDefinition,
    pub index: usize,
}

impl RemoveAgent {
    pub fn new(model: &MapModel, id: u32) -> Option<Self> {
        let index = model.agents.iter().position(|agent| agent.id == id)?;
        Some(Self {
            agent: model.agents[index].clone(),
            index,
        })
    }
}

impl EditCommand for RemoveAgent {
    fn label(&self) -> String {
        format!("Delete agent {}", self.agent.name)
    }

    fn apply(&mut self, model: &mut MapModel) {
        model.remove_agent(self.agent.id);
    }

    fn undo(&mut self, model: &mut MapModel) {
        let index = self.index.min(model.agents.len());
        model.agents.insert(index, self.agent.clone());
    }
}

/// Edit an agent's authored fields as one undo step.
pub struct SetAgent {
    pub before: AgentDefinition,
    pub after: AgentDefinition,
}

impl SetAgent {
    pub fn new(model: &MapModel, after: AgentDefinition) -> Option<Self> {
        let before = model.agent(after.id)?.clone();
        if before == after {
            None
        } else {
            Some(Self { before, after })
        }
    }
}

impl EditCommand for SetAgent {
    fn label(&self) -> String {
        format!("Edit agent {}", self.after.name)
    }

    fn apply(&mut self, model: &mut MapModel) {
        if let Some(slot) = model.agent_mut(self.after.id) {
            *slot = self.after.clone();
        }
    }

    fn undo(&mut self, model: &mut MapModel) {
        if let Some(slot) = model.agent_mut(self.before.id) {
            *slot = self.before.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::MapModel;
    use cnc_core::{ClassId, Vec2i};

    #[test]
    fn agent_commands_undo_and_redo_exactly() {
        let mut model = MapModel::new(8, 8);
        let mut history = History::new(16);
        history.execute(
            &mut model,
            Box::new(AddAgent {
                agent: AgentDefinition::new(0, "Fist"),
            }),
        );
        assert_eq!(model.agents.len(), 1);
        let mut edited = model.agents[0].clone();
        edited.script = "on start { harvest() }".to_string();
        let set_agent = SetAgent::new(&model, edited).unwrap();
        history.execute(&mut model, Box::new(set_agent));
        assert!(model.agents[0].script.contains("harvest"));
        let remove = RemoveAgent::new(&model, model.agents[0].id).unwrap();
        history.execute(&mut model, Box::new(remove));
        assert!(model.agents.is_empty());

        history.undo(&mut model);
        assert_eq!(model.agents.len(), 1);
        history.undo(&mut model);
        assert!(model.agents[0].script.contains("Balanced"));
        history.undo(&mut model);
        assert!(model.agents.is_empty());
        history.redo(&mut model);
        assert_eq!(model.agents.len(), 1);
    }

    #[test]
    fn paint_undo_redo_restores_exact_values() {
        let mut model = MapModel::new(4, 4);
        model.terrain[5] = 3;
        let mut history = History::new(16);
        let command = Box::new(PaintCells::terrain(&model, [5, 6], 9));
        history.execute(&mut model, command);
        assert_eq!(model.terrain[5], 9);
        assert_eq!(model.terrain[6], 9);
        assert!(history.can_undo());

        assert_eq!(
            history.undo(&mut model).as_deref(),
            Some("Paint terrain (2 cells)")
        );
        assert_eq!(model.terrain[5], 3);
        assert_eq!(model.terrain[6], 0);
        assert!(history.can_redo());

        history.redo(&mut model);
        assert_eq!(model.terrain[5], 9);
    }

    #[test]
    fn place_and_remove_restore_object_ids() {
        let mut model = MapModel::new(8, 8);
        let mut history = History::new(8);
        history.execute(
            &mut model,
            Box::new(PlaceObject::new(MapObject {
                id: 0,
                class_id: ClassId::InfantryE1,
                position: Vec2i::new(2, 2),
                owner: None,
            })),
        );
        assert_eq!(model.objects.len(), 1);
        let id = *model.objects.keys().next().unwrap();

        let command = Box::new(RemoveObject::new(&model, id).unwrap());
        history.execute(&mut model, command);
        assert!(model.objects.is_empty());
        history.undo(&mut model);
        assert_eq!(model.objects[&id].position, Vec2i::new(2, 2));

        history.undo(&mut model);
        assert!(model.objects.is_empty());
        assert!(!history.can_undo());
    }

    #[test]
    fn redo_stack_clears_on_new_edit_and_limit_trims() {
        let mut model = MapModel::new(4, 4);
        let mut history = History::new(2);
        for index in 0..4 {
            let command = Box::new(PaintCells::terrain(&model, [index], index as u8 + 1));
            history.execute(&mut model, command);
        }
        assert_eq!(history.undo_len(), 2, "history is bounded");
        history.undo(&mut model);
        assert!(history.can_redo());
        let command = Box::new(PaintCells::terrain(&model, [0], 42));
        history.execute(&mut model, command);
        assert!(!history.can_redo(), "new edit clears redo");
        assert_eq!(model.terrain[0], 42);
    }

    #[test]
    fn grouped_stroke_is_one_undo_step() {
        let mut model = MapModel::new(4, 4);
        let mut history = History::new(16);
        history.begin_group("Brush stroke");
        for index in 0..3 {
            let command = Box::new(PaintCells::terrain(&model, [index], index as u8 + 1));
            history.execute(&mut model, command);
        }
        assert!(history.in_group());
        assert!(!history.can_undo(), "group is not committed until it ends");
        history.end_group();
        assert!(history.can_undo());
        assert_eq!(history.undo_len(), 1);
        assert_eq!(model.terrain[0], 1);
        history.undo(&mut model);
        for index in 0..3 {
            assert_eq!(model.terrain[index], 0);
        }
        assert_eq!(history.next_redo_label().as_deref(), Some("Brush stroke"));
    }

    #[test]
    fn labels_report_undo_and_redo_stacks() {
        let mut model = MapModel::new(4, 4);
        let mut history = History::new(8);
        assert!(history.undo_labels().is_empty());
        let command = Box::new(PaintCells::terrain(&model, [0], 1));
        history.execute(&mut model, command);
        assert_eq!(
            history.undo_labels(),
            vec!["Paint terrain (1 cells)".to_string()]
        );
        history.undo(&mut model);
        assert!(history.undo_labels().is_empty());
        assert_eq!(
            history.redo_labels(),
            vec!["Paint terrain (1 cells)".to_string()]
        );
    }

    #[test]
    fn object_owner_change_is_undoable() {
        let mut model = MapModel::new(4, 4);
        let id = model.insert_object(MapObject {
            id: 0,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(1, 1),
            owner: None,
        });
        let mut history = History::new(8);
        let command = Box::new(SetObjectOwner::new(&model, id, Some(ClassId::HouseGDI)).unwrap());
        history.execute(&mut model, command);
        assert_eq!(model.objects[&id].owner, Some(ClassId::HouseGDI));
        history.undo(&mut model);
        assert_eq!(model.objects[&id].owner, None);
        // No-op changes produce no command.
        assert!(SetObjectOwner::new(&model, id, None).is_none());
    }

    #[test]
    fn trigger_add_and_remove_roundtrip() {
        let mut model = MapModel::new(4, 4);
        let mut history = History::new(8);
        history.execute(
            &mut model,
            Box::new(AddTrigger {
                trigger: Trigger {
                    id: 0,
                    name: "victory".to_string(),
                    condition: "always".to_string(),
                    action: "win".to_string(),
                    group: String::new(),
                },
            }),
        );
        let id = model.triggers[0].id;
        assert_eq!(id, 1);
        history.undo(&mut model);
        assert!(model.triggers.is_empty());
        history.redo(&mut model);
        assert_eq!(model.triggers[0].name, "victory");
    }
}
