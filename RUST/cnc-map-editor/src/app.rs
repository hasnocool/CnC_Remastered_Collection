//! Editor application state, layout and rendering.
//!
//! The UI is intentionally immediate-mode and renderer-agnostic: [`EditorState`]
//! draws through any `cnc_renderer::Renderer` using screen-space coordinates, so
//! the same code drives the interactive wgpu window and the headless software
//! screenshot used for verification. Rendering is done with plain quads and the
//! built-in bitmap font; there is no extra GUI toolkit dependency.

use std::path::{Path, PathBuf};

use cnc_ai::{
    class_references, compile, intent_slots, observation_prompt, parse_response, AgentDefinition,
    AgentRuntime, Contact, Observation, OracleRequest, Pos, Vision, DOCTRINES, VISION_BLOCKED,
    VISION_EMPTY, VISION_ENEMY, VISION_OWN, VISION_RESOURCE,
};
use cnc_core::{ClassId, Fixed16_16, Vec2, Vec2i};
use cnc_renderer::{Renderer, Sprite};

use crate::generator::{generate, MapGenSettings, TerrainType};
use crate::history::{
    AddAgent, EditCommand, RemoveAgent, RemoveObject, RemoveTrigger, ScenarioInfo, SetAgent,
    SetObjectClass, SetScenarioInfo, SetTrigger,
};
use crate::map::{ImportKind, MapModel, MapObject, Trigger, TILE_SIZE};
use crate::pack_tileset::PackTileset;
use crate::plugin::{GamePlugin, RedAlertPlugin, Theater, TiberianDawnPlugin};
use crate::tools::{EraseTool, ObjectTool, TerrainTool, TriggerTool};
use crate::ui::EditorUI;

/// Height of the top toolbar.
pub const TOP_H: f32 = 36.0;
/// Width of the right inspector panel.
pub const RIGHT_W: f32 = 336.0;
/// Height of the bottom status bar.
pub const STATUS_H: f32 = 24.0;

const SIZES: [u32; 5] = [48, 64, 96, 128, 192];
const THEATERS: [Theater; 4] = [
    Theater::Temperate,
    Theater::Snow,
    Theater::Desert,
    Theater::Urban,
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectF {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl RectF {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x <= self.x + self.w && y <= self.y + self.h
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerButton {
    Left,
    Right,
    Middle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Focus {
    None,
    Seed,
    Path,
    TriggerName,
    TriggerCondition,
    TriggerAction,
    TriggerGroup,
    TriggerFilter,
    AgentName,
    AgentScript,
    PlacedSearch,
    MapWidth,
    MapHeight,
    ObjectSearch,
    MapName,
    MapPlayer,
    MapBriefing,
    MapObjectives,
}

/// Which inspector the lower half of the right panel shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InspectorTab {
    Objects,
    Triggers,
    History,
    Validate,
    Files,
}

/// Cached terrain thumbnail for a scenario file.
#[derive(Clone, Debug)]
pub struct ScenarioPreview {
    pub width: u32,
    pub height: u32,
    pub terrain: Vec<u8>,
}

/// Which owner/class the placed-object list shows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlacedFilter {
    All,
    Unassigned,
    Player,
    Enemy,
    Class(ClassId),
}

impl PlacedFilter {
    pub fn label(self) -> String {
        match self {
            Self::All => "All".to_string(),
            Self::Unassigned => "None".to_string(),
            Self::Player => "Player".to_string(),
            Self::Enemy => "Enemy".to_string(),
            Self::Class(class_id) => format!("{class_id:?}"),
        }
    }
}

/// Coarse object-palette grouping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectCategory {
    All,
    Infantry,
    Vehicles,
    Aircraft,
    Buildings,
}

impl ObjectCategory {
    pub const ALL: [ObjectCategory; 5] = [
        Self::All,
        Self::Infantry,
        Self::Vehicles,
        Self::Aircraft,
        Self::Buildings,
    ];

    pub fn next(self) -> Self {
        let index = Self::ALL
            .iter()
            .position(|category| *category == self)
            .unwrap_or(0);
        Self::ALL[(index + 1) % Self::ALL.len()]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Infantry => "Infantry",
            Self::Vehicles => "Vehicles",
            Self::Aircraft => "Aircraft",
            Self::Buildings => "Buildings",
        }
    }

    pub fn matches(self, class_id: ClassId) -> bool {
        match self {
            Self::All => true,
            Self::Infantry => matches!(
                class_id,
                ClassId::InfantryE1
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
            ),
            Self::Vehicles => matches!(
                class_id,
                ClassId::VehicleE1
                    | ClassId::VehicleE2
                    | ClassId::VehicleE3
                    | ClassId::VehicleE4
                    | ClassId::VehicleE5
                    | ClassId::VehicleE6
                    | ClassId::VehicleE7
                    | ClassId::VehicleE8
                    | ClassId::VehicleE9
                    | ClassId::VehicleE10
                    | ClassId::VehicleE11
                    | ClassId::VehicleE12
                    | ClassId::VehicleE13
                    | ClassId::VehicleE14
            ),
            Self::Aircraft => matches!(
                class_id,
                ClassId::AircraftE1
                    | ClassId::AircraftE2
                    | ClassId::AircraftE3
                    | ClassId::AircraftE4
                    | ClassId::AircraftE5
                    | ClassId::AircraftE6
                    | ClassId::AircraftE7
                    | ClassId::AircraftE8
            ),
            Self::Buildings => class_id.is_building(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    New,
    OpenAgentStudio,
    Generate,
    Open,
    /// File→Import Original Map: load a retail TD or RA scenario, replacing
    /// the working map through an undoable command.
    ImportOriginalMap,
    Save,
    Undo,
    Redo,
    ToggleGrid,
    ZoomIn,
    ZoomOut,
    SizeDecrease,
    SizeIncrease,
    PlayersDecrease,
    PlayersIncrease,
    TheaterNext,
    TogglePlugin,
    FocusSeed,
    FocusPath,
    SelectTool,
    EraseTool,
    TriggerTool,
    AddTrigger,
    CommitTrigger,
    NextTriggerTemplate,
    InspectorObjects,
    InspectorTriggers,
    InspectorHistory,
    InspectorValidate,
    InspectorFiles,
    NextObjectCategory,
    NextPlacedOwnerFilter,
    RefreshScenarioFiles,
    CopyObject,
    PasteObject,
    ObjectOwnerNext,
    ObjectNudge(i32, i32),
    DeleteObject,
    DeleteTrigger,
    Terrain(u8),
    /// Paint the overlay brush with an [`crate::generator::OverlayType`] id.
    Overlay(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlKind {
    Button(Action),
    Field(Focus),
    Terrain(u8),
    Overlay(u8),
    Object(ClassId),
    Trigger(u32),
    PlacedObject(u32),
    ScenarioFile(usize),
}

/// Mutable trigger being edited through the inspector fields. Edits are
/// buffered here and only committed to the model through a `SetTrigger`
/// command, so undo sees one step per finished edit rather than one per key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TriggerEdit {
    pub id: u32,
    pub name: String,
    pub condition: String,
    pub action: String,
    pub group: String,
}

impl TriggerEdit {
    fn from_trigger(trigger: &Trigger) -> Self {
        Self {
            id: trigger.id,
            name: trigger.name.clone(),
            condition: trigger.condition.clone(),
            action: trigger.action.clone(),
            group: trigger.group.clone(),
        }
    }

    fn to_trigger(&self) -> Trigger {
        Trigger {
            id: self.id,
            name: self.name.clone(),
            condition: self.condition.clone(),
            action: self.action.clone(),
            group: self.group.clone(),
        }
    }
}

/// Editable buffer for one agent inside the Agent Studio.
///
/// The buffer is committed as a single undoable command when the selection
/// changes or the studio closes, so typing never floods the undo stack.
#[derive(Clone, Debug)]
pub struct AgentEdit {
    pub id: u32,
    pub name: String,
    pub owner: Option<ClassId>,
    pub doctrine: String,
    pub script: String,
    /// Caret position as a character index into script.
    pub cursor: usize,
    pub scroll: usize,
}

impl AgentEdit {
    pub fn from_agent(agent: &AgentDefinition) -> Self {
        Self {
            id: agent.id,
            name: agent.name.clone(),
            owner: crate::map::parse_class_id(&agent.owner),
            doctrine: agent.doctrine.clone(),
            script: agent.script.clone(),
            cursor: agent.script.chars().count(),
            scroll: 0,
        }
    }

    fn to_agent(&self) -> AgentDefinition {
        AgentDefinition {
            id: self.id,
            name: self.name.clone(),
            owner: self
                .owner
                .map(|owner| format!("{owner:?}"))
                .unwrap_or_default(),
            doctrine: self.doctrine.clone(),
            script: self.script.clone(),
        }
    }

    fn line_ranges(&self) -> Vec<(usize, usize)> {
        line_ranges(&self.script)
    }

    fn cursor_line(&self) -> usize {
        let ranges = self.line_ranges();
        ranges
            .iter()
            .rposition(|(start, _)| *start <= self.cursor)
            .unwrap_or(0)
    }

    pub fn insert_text(&mut self, text: &str) {
        let mut chars: Vec<char> = self.script.chars().collect();
        let at = self.cursor.min(chars.len());
        let inserted: Vec<char> = text
            .chars()
            .filter(|character| *character != '\r')
            .collect();
        let count = inserted.len();
        if count == 0 {
            return;
        }
        chars.splice(at..at, inserted);
        self.script = chars.into_iter().collect();
        self.cursor = at + count;
    }

    pub fn insert_newline(&mut self) {
        self.insert_text("\n");
    }

    pub fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let mut chars: Vec<char> = self.script.chars().collect();
        let at = self.cursor.min(chars.len());
        chars.remove(at - 1);
        self.script = chars.into_iter().collect();
        self.cursor = at - 1;
    }

    pub fn delete(&mut self) {
        let mut chars: Vec<char> = self.script.chars().collect();
        let at = self.cursor.min(chars.len());
        if at < chars.len() {
            chars.remove(at);
            self.script = chars.into_iter().collect();
        }
    }

    pub fn move_horizontal(&mut self, delta: isize) {
        let length = self.script.chars().count();
        self.cursor = (self.cursor as isize + delta).clamp(0, length as isize) as usize;
    }

    pub fn move_vertical(&mut self, delta: isize) {
        let ranges = self.line_ranges();
        let line = ranges
            .iter()
            .rposition(|(start, _)| *start <= self.cursor)
            .unwrap_or(0);
        let (start, _) = ranges[line];
        let column = self.cursor.saturating_sub(start);
        let target = if delta < 0 {
            line.saturating_sub(1)
        } else {
            (line + 1).min(ranges.len() - 1)
        };
        let (target_start, target_end) = ranges[target];
        self.cursor = (target_start + column).min(target_end);
    }

    pub fn move_home(&mut self) {
        let line = self.cursor_line();
        self.cursor = self.line_ranges()[line].0;
    }

    pub fn move_end(&mut self) {
        let line = self.cursor_line();
        self.cursor = self.line_ranges()[line].1;
    }

    pub fn ensure_visible(&mut self, visible: usize) {
        let line = self.cursor_line();
        if line < self.scroll {
            self.scroll = line;
        } else if line >= self.scroll + visible {
            self.scroll = line + 1 - visible;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StudioButton {
    New,
    Duplicate,
    Delete,
    Check,
    CheckAll,
    Run,
    Oracle,
    Close,
}

struct StudioLayout {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    name: RectF,
    owner: RectF,
    doctrine: RectF,
    list: RectF,
    code: RectF,
    report: RectF,
    buttons: [(StudioButton, RectF); 8],
}

fn line_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = 0;
    for (index, character) in text.chars().enumerate() {
        if character == '\n' {
            ranges.push((start, index));
            start = index + 1;
        }
    }
    ranges.push((start, text.chars().count()));
    ranges
}

#[derive(Clone, Debug)]
pub struct Control {
    pub rect: RectF,
    pub kind: ControlKind,
}

/// All non-window editor state.
pub struct EditorState {
    pub map: MapModel,
    pub editor: EditorUI,
    pub plugin_index: usize,
    pub seed: String,
    pub path: String,
    pub players: u8,
    pub map_width: u32,
    pub map_height: u32,
    pub map_width_text: String,
    pub map_height_text: String,
    pub theater: Theater,
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub show_grid: bool,
    /// Tileset pack used for terrain sprites (None = flat colored quads).
    pub tileset: Option<std::sync::Arc<PackTileset>>,
    /// Shift is held (bulk palette replacement while a selection exists).
    pub shift_held: bool,
    pub focus: Focus,
    pub mouse: Option<(f32, f32)>,
    pub cursor_cell: Vec2i,
    pub painting: bool,
    pub panning: bool,
    pub last_cell: Option<Vec2i>,
    pub last_pointer: (f32, f32),
    pub status: String,
    pub brush_terrain: u8,
    pub overlay_mode: bool,
    pub selected_object_tool: Option<ClassId>,
    pub selected_trigger: Option<u32>,
    pub trigger_edit: Option<TriggerEdit>,
    pub select_mode: bool,
    pub dragging_object: Option<u32>,
    pub inspector_tab: InspectorTab,
    /// The Agent Studio modal is open.
    pub agent_studio: bool,
    /// Shortcut help overlay is visible.
    pub show_help: bool,
    /// Map-info modal is open.
    pub map_info_open: bool,
    pub map_info_name: String,
    pub map_info_player: String,
    pub map_info_briefing: String,
    pub map_info_objectives: String,
    /// Agent selected in the studio.
    pub selected_agent: Option<u32>,
    pub agent_edit: Option<AgentEdit>,
    pub agent_report: Vec<String>,
    pub agent_message: String,
    /// Agent ids whose last check failed (tinted in the studio list).
    pub agent_bad: std::collections::BTreeSet<u32>,
    pub clipboard_objects: Vec<MapObject>,
    pub clipboard_trigger: Option<Trigger>,
    pub trigger_template_index: usize,
    pub trigger_filter: String,
    pub object_search: String,
    pub object_category: ObjectCategory,
    pub placed_filter: PlacedFilter,
    pub placed_search: String,
    pub scenario_files: Vec<PathBuf>,
    pub scenario_previews: Vec<Option<ScenarioPreview>>,
    pub scenario_hover: Option<usize>,
    pub marquee_start: Option<Vec2i>,
    pub marquee_end: Option<Vec2i>,
    pub marquee_active: bool,
    /// Warnings produced by the most recent original-format import, shown in
    /// the validation panel and cleared by the next import (or a new/open).
    pub import_warnings: Vec<String>,
    pub width: f32,
    pub height: f32,
}

impl EditorState {
    pub fn new(width: f32, height: f32) -> Self {
        let settings = MapGenSettings::new(1);
        let map = generate(&settings);
        let mut state = Self {
            map,
            editor: EditorUI::new(),
            plugin_index: 0,
            seed: "1".to_string(),
            path: "maps/editor_map.ini".to_string(),
            players: 2,
            map_width: 96,
            map_height: 96,
            map_width_text: "96".to_string(),
            map_height_text: "96".to_string(),
            theater: Theater::Temperate,
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            show_grid: true,
            tileset: None,
            shift_held: false,
            focus: Focus::None,
            mouse: None,
            cursor_cell: Vec2i::new(0, 0),
            painting: false,
            panning: false,
            last_cell: None,
            last_pointer: (0.0, 0.0),
            status: "generated map (seed 1)".to_string(),
            brush_terrain: TerrainType::Grass.id(),
            overlay_mode: false,
            selected_object_tool: None,
            selected_trigger: None,
            trigger_edit: None,
            select_mode: false,
            dragging_object: None,
            inspector_tab: InspectorTab::Triggers,
            agent_studio: false,
            show_help: false,
            map_info_open: false,
            map_info_name: String::new(),
            map_info_player: String::new(),
            map_info_briefing: String::new(),
            map_info_objectives: String::new(),
            selected_agent: None,
            agent_edit: None,
            agent_report: Vec::new(),
            agent_message: String::new(),
            agent_bad: std::collections::BTreeSet::new(),
            clipboard_objects: Vec::new(),
            clipboard_trigger: None,
            trigger_template_index: 0,
            trigger_filter: String::new(),
            object_search: String::new(),
            object_category: ObjectCategory::All,
            placed_filter: PlacedFilter::All,
            placed_search: String::new(),
            scenario_files: Vec::new(),
            scenario_previews: Vec::new(),
            scenario_hover: None,
            marquee_start: None,
            marquee_end: None,
            marquee_active: false,
            import_warnings: Vec::new(),
            width,
            height,
        };
        state.scenario_files = scenario_files_in(std::path::Path::new("maps"));
        state.fit();
        state.set_terrain_tool();
        state
    }

    pub fn plugin(&self) -> Box<dyn GamePlugin> {
        match self.plugin_index {
            0 => Box::new(TiberianDawnPlugin::default()),
            _ => Box::new(RedAlertPlugin::default()),
        }
    }

    pub fn plugin_name(&self) -> &'static str {
        if self.plugin_index == 0 {
            "Tiberian Dawn"
        } else {
            "Red Alert"
        }
    }

    fn owner_house(&self) -> ClassId {
        if self.plugin_index == 0 {
            ClassId::HouseGDI
        } else {
            ClassId::HouseAllies
        }
    }

    /// Fit the whole map into the map viewport.
    pub fn fit(&mut self) {
        let area_w = (self.width - RIGHT_W).max(64.0);
        let area_h = (self.height - TOP_H - STATUS_H).max(64.0);
        let map_w = self.map.width.max(1) as f32 * TILE_SIZE;
        let map_h = self.map.height.max(1) as f32 * TILE_SIZE;
        self.zoom = ((area_w / map_w).min(area_h / map_h) * 0.96).clamp(0.05, 8.0);
        self.pan_x = (area_w - map_w * self.zoom) * 0.5;
        self.pan_y = (area_h - map_h * self.zoom) * 0.5;
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.fit();
    }

    fn map_area(&self) -> RectF {
        RectF::new(
            0.0,
            TOP_H,
            (self.width - RIGHT_W).max(1.0),
            (self.height - TOP_H - STATUS_H).max(1.0),
        )
    }

    fn in_map_area(&self, x: f32, y: f32) -> bool {
        self.map_area().contains(x, y) && x < self.width - RIGHT_W
    }

    /// Screen position -> map cell (may be outside the grid).
    pub fn cell_at(&self, x: f32, y: f32) -> Vec2i {
        let area = self.map_area();
        let scale = TILE_SIZE * self.zoom;
        Vec2i::new(
            ((x - area.x - self.pan_x) / scale).floor() as i32,
            ((y - area.y - self.pan_y) / scale).floor() as i32,
        )
    }

    /// Bottom-left minimap rectangle inside the map viewport.
    fn minimap_rect(&self) -> RectF {
        let area = self.map_area();
        let width = (area.w * 0.22).clamp(90.0, 200.0);
        let aspect = self.map.height as f32 / self.map.width.max(1) as f32;
        let height = (width * aspect).clamp(60.0, area.h * 0.4);
        RectF::new(area.x + 8.0, area.y + area.h - height - 8.0, width, height)
    }

    fn minimap_cell_at(&self, x: f32, y: f32) -> Option<Vec2i> {
        let rect = self.minimap_rect();
        if !rect.contains(x, y) {
            return None;
        }
        let fx = ((x - rect.x) / rect.w).clamp(0.0, 1.0);
        let fy = ((y - rect.y) / rect.h).clamp(0.0, 1.0);
        let cell = Vec2i::new(
            (fx * self.map.width as f32).floor() as i32,
            (fy * self.map.height as f32).floor() as i32,
        );
        self.map.in_bounds(cell).then_some(cell)
    }

    /// Centre the viewport on a minimap cell.
    fn jump_to_cell(&mut self, cell: Vec2i) {
        let area = self.map_area();
        let scale = TILE_SIZE * self.zoom;
        self.pan_x = area.w * 0.5 - cell.x as f32 * scale - scale * 0.5;
        self.pan_y = area.h * 0.5 - cell.y as f32 * scale - scale * 0.5;
        self.cursor_cell = cell;
        self.status = format!("jumped to cell ({},{})", cell.x, cell.y);
    }

    fn cell_screen(&self, cell: Vec2i) -> (f32, f32) {
        let area = self.map_area();
        let scale = TILE_SIZE * self.zoom;
        (
            area.x + self.pan_x + cell.x as f32 * scale,
            area.y + self.pan_y + cell.y as f32 * scale,
        )
    }

    fn set_terrain_tool(&mut self) {
        self.selected_object_tool = None;
        self.select_mode = false;
        if self.overlay_mode {
            self.editor
                .set_tool(Box::new(TerrainTool::overlay(self.brush_terrain)));
        } else {
            self.editor
                .set_tool(Box::new(TerrainTool::new(self.brush_terrain)));
        }
    }

    /// Update both custom dimensions and their editable text.
    fn set_dimensions(&mut self, width: u32, height: u32) {
        self.map_width = width.clamp(8, 512);
        self.map_height = height.clamp(8, 512);
        self.map_width_text = self.map_width.to_string();
        self.map_height_text = self.map_height.to_string();
    }

    fn sync_dimensions(&mut self) {
        self.map_width = parse_dimension(&self.map_width_text, self.map_width);
        self.map_height = parse_dimension(&self.map_height_text, self.map_height);
    }

    /// Replace the class of every selected object (Shift + palette click).
    pub fn replace_selected_class(&mut self, class_id: ClassId) {
        let ids = self.selected_object_ids();
        if ids.is_empty() {
            self.status = "replace: select objects first".to_string();
            return;
        }
        let Some(command) = SetObjectClass::new(&self.map, &ids, class_id) else {
            self.status = "replace: selection already uses that class".to_string();
            return;
        };
        let count = command.before.len();
        self.editor
            .history
            .execute(&mut self.map, Box::new(command));
        self.status = format!("changed {count} object(s) to {class_id:?}");
    }

    fn set_object_tool(&mut self, class_id: ClassId) {
        self.selected_object_tool = Some(class_id);
        self.select_mode = false;
        self.editor.set_tool(Box::new(ObjectTool::with_owner(
            class_id,
            self.owner_house(),
        )));
    }

    pub fn generate(&mut self) {
        self.apply_generate();
    }

    fn apply_generate(&mut self) {
        self.reset_agent_editor();
        let settings = MapGenSettings {
            width: self.map_width,
            height: self.map_height,
            seed: self.seed.trim().parse().unwrap_or(1),
            player_count: self.players,
            theater: self.theater,
            ..MapGenSettings::default()
        };
        self.map = generate(&settings);
        self.editor = EditorUI::new();
        self.selected_trigger = None;
        self.trigger_edit = None;
        self.set_terrain_tool();
        self.fit();
        self.status = format!(
            "generated {} (seed {}, {} players)",
            self.map.summary(),
            settings.seed,
            settings.player_count
        );
    }

    fn new_map(&mut self) {
        self.reset_agent_editor();
        self.import_warnings.clear();
        self.map = MapModel::new(self.map_width, self.map_height);
        self.map.theater = self.theater.name().to_string();
        self.editor = EditorUI::new();
        self.selected_trigger = None;
        self.trigger_edit = None;
        self.set_terrain_tool();
        self.fit();
        self.status = format!("new empty {}x{} map", self.map_width, self.map_height);
    }

    fn resolve_path(&self) -> PathBuf {
        let text = if self.path.trim().is_empty() {
            "maps/editor_map.ini"
        } else {
            self.path.trim()
        };
        PathBuf::from(text)
    }

    pub fn save(&mut self) {
        // Flush in-progress trigger field edits so what is on screen is what
        // gets serialized.
        self.commit_trigger_edit();
        self.commit_agent_edit();
        let path = self.resolve_path();
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        match self.plugin().save_map(&self.map, &path) {
            Ok(written) => {
                let files = written
                    .iter()
                    .map(|file| file.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                self.status = format!("saved {files} ({})", self.map.summary());
            }
            Err(error) => self.status = format!("save failed: {error}"),
        }
    }

    fn reset_agent_editor(&mut self) {
        self.agent_studio = false;
        self.agent_edit = None;
        self.selected_agent = None;
        self.agent_report.clear();
        self.agent_message.clear();
        if matches!(self.focus, Focus::AgentName | Focus::AgentScript) {
            self.focus = Focus::None;
        }
    }

    pub fn open(&mut self) {
        self.reset_agent_editor();
        self.import_warnings.clear();
        let path = self.resolve_path();
        // The plugin dispatches on extension/dialect and pairs a TD .ini with
        // its sibling .bin, so both games open through one path.
        match self.plugin().load_map(&path) {
            Ok(map) => {
                let kind = map.original_dialect().unwrap_or(ImportKind::EditorDialect);
                self.plugin_index = match kind {
                    ImportKind::OriginalTd => 0,
                    ImportKind::OriginalRa => 1,
                    ImportKind::EditorDialect => self.plugin_index,
                };
                self.map = map;
                self.editor = EditorUI::new();
                self.selected_trigger = None;
                self.trigger_edit = None;
                self.set_terrain_tool();
                self.fit();
                self.status = format!(
                    "opened {} as {} ({})",
                    path.display(),
                    kind.label(),
                    self.map.summary()
                );
            }
            Err(error) => self.status = format!("open failed: {error}"),
        }
    }

    /// Import an original-format scenario (File→Import Original Map).
    ///
    /// The text is sniffed for its dialect: a base64 `[MapPack]` block means
    /// RA, otherwise a `[Basic]` scenario is TD, which additionally needs its
    /// sibling cell image. The freshly parsed model replaces the working map
    /// through [`crate::history::ImportOriginalMap`], so a single Ctrl+Z
    /// reverts to the previously open map.
    ///
    /// Malformed input (a truncated `.bin`, an editor-dialect scenario, or a
    /// TD `.ini` with no `.bin`) is reported through `self.status` and listed
    /// in the validation panel, and leaves the model and the undo stack
    /// exactly as they were: nothing is executed, so there is nothing to undo.
    pub fn import_original_map(&mut self, ini: &str, bin: Option<&[u8]>) {
        self.import_warnings.clear();
        let lower = ini.to_ascii_lowercase();
        // RA carries its grids in base64 [MapPack]; TD is the [Basic] dialect
        // that ships its cell image as a sibling .bin.
        let is_ra = crate::map::looks_like_original_ra(ini);
        let is_td = !is_ra && lower.contains("[basic]");

        let (imported, kind) = if is_ra {
            (MapModel::from_original_ra(ini), ImportKind::OriginalRa)
        } else if is_td {
            match bin {
                Some(bin) => (MapModel::from_original_td(ini, bin), ImportKind::OriginalTd),
                None => {
                    let message = "an original TD scenario needs its .bin map image".to_string();
                    self.import_warnings.push(message.clone());
                    self.status = format!("import failed: {message}");
                    return;
                }
            }
        } else {
            let message = "not an original TD or RA scenario (no [MapPack] and no [Basic] section)"
                .to_string();
            self.import_warnings.push(message.clone());
            self.status = format!("import failed: {message}");
            return;
        };

        let imported = match imported {
            Ok(imported) => imported,
            Err(error) => {
                // Model untouched: the command is never built or executed, so
                // there is no partial state to undo.
                let message = error.to_string();
                self.import_warnings.push(message.clone());
                self.status = format!("import failed: {message}");
                return;
            }
        };

        // The loaders keep objects whose type token they cannot resolve as
        // Unknown instead of dropping the placement, so those are the parse
        // warnings an import can surface without inventing semantics.
        let mut unknowns: Vec<String> = imported
            .objects
            .values()
            .filter(|object| object.class_id == ClassId::Unknown)
            .map(|object| {
                format!(
                    "object #{} at ({},{}) has an unresolvable type",
                    object.id, object.position.x, object.position.y
                )
            })
            .collect();
        unknowns.sort();
        unknowns.dedup();
        self.import_warnings = unknowns;

        let previous = self.map.clone();
        let command = crate::history::ImportOriginalMap::new(previous, imported, kind);
        self.editor
            .history
            .execute(&mut self.map, Box::new(command));
        self.plugin_index = match kind {
            ImportKind::OriginalTd => 0,
            ImportKind::OriginalRa => 1,
            ImportKind::EditorDialect => self.plugin_index,
        };
        self.apply_import_reset(kind);
    }

    /// Reset the view state that belongs to the map that was open before the
    /// import, so the imported map starts from a clean selection and fits the
    /// view. Called after the command executed, never instead of it.
    ///
    /// The `EditorUI` itself is deliberately preserved: it owns the undo
    /// stack the import command was just pushed onto.
    fn apply_import_reset(&mut self, kind: ImportKind) {
        self.reset_agent_editor();
        self.editor.selected_object = None;
        self.editor.selected_objects = Vec::new();
        self.selected_trigger = None;
        self.trigger_edit = None;
        self.set_terrain_tool();
        self.fit();
        let warnings = if self.import_warnings.is_empty() {
            String::new()
        } else {
            format!(
                " ({} warning{})",
                self.import_warnings.len(),
                if self.import_warnings.len() == 1 {
                    ""
                } else {
                    "s"
                }
            )
        };
        self.status = format!(
            "imported {} as {} ({}){}",
            self.path,
            kind.label(),
            self.map.summary(),
            warnings
        );
    }

    /// Native import dialog: the scenario INI/MPR plus, for a TD scenario, its
    /// sibling `.bin` cell image. Falls back to the typed path when the
    /// platform has no dialog helper.
    pub fn import_original_with_dialog(&mut self) {
        let current = self.resolve_path();
        let directory = self.dialog_directory();
        let mut builder = native_dialog::FileDialogBuilder::default()
            .set_title("Import original map")
            .add_filter("Original scenario", ["ini", "mpr"]);
        if directory.exists() {
            builder = builder.set_location(&directory);
        }
        if let Some(name) = current.file_name() {
            builder = builder.set_filename(name.to_string_lossy());
        }
        let choice = builder
            .open_single_file()
            .show()
            .map_err(|error| error.to_string());
        self.handle_import_choice(choice);
    }

    /// Apply an import dialog outcome. Split out from
    /// [`Self::import_original_with_dialog`] so the success, cancel and
    /// "no helper installed" paths are testable without a display.
    pub fn handle_import_choice(&mut self, choice: Result<Option<PathBuf>, String>) {
        let path = match choice {
            Ok(Some(path)) => path,
            Ok(None) => {
                self.status = "import cancelled".to_string();
                return;
            }
            Err(error) => {
                // No dialog available: fall back to the path field, same as
                // the open dialog does.
                let note =
                    format!("native import dialog unavailable ({error}); using path field: ");
                let fallback = self.resolve_path();
                self.import_original_path(&fallback);
                self.status = format!("{note}{}", self.status);
                return;
            }
        };
        self.path = path.display().to_string();
        self.import_original_path(&path);
    }

    /// Import an original scenario from disk, pairing a TD `.ini` with its
    /// sibling `.bin` of the same stem.
    pub fn import_original_path(&mut self, path: &Path) {
        let ini = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) => {
                self.status = format!("import failed: read {}: {error}", path.display());
                return;
            }
        };
        // A TD scenario's cell image is a sibling file with the same stem.
        let bin = path.with_extension("bin");
        let bin = std::fs::read(&bin).ok();
        self.import_original_map(&ini, bin.as_deref());
    }

    /// Directory the file dialogs start in, derived from the path field.
    fn dialog_directory(&self) -> PathBuf {
        let path = self.resolve_path();
        path.parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(std::path::Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("maps"))
    }

    /// Native open dialog. Falls back to the typed path when the platform has
    /// no dialog helper (for example a bare Linux box without zenity/kdialog).
    pub fn open_with_dialog(&mut self) {
        let current = self.resolve_path();
        let directory = self.dialog_directory();
        let mut builder = native_dialog::FileDialogBuilder::default()
            .set_title("Open scenario")
            .add_filter("Scenario INI", ["ini"]);
        if directory.exists() {
            builder = builder.set_location(&directory);
        }
        if let Some(name) = current.file_name() {
            builder = builder.set_filename(name.to_string_lossy());
        }
        let choice = builder
            .open_single_file()
            .show()
            .map_err(|error| error.to_string());
        self.handle_open_choice(choice);
    }

    /// Apply a dialog outcome. Split out from open_with_dialog so the success,
    /// cancel and "no helper installed" paths are testable without a display.
    fn handle_open_choice(&mut self, choice: Result<Option<PathBuf>, String>) {
        match choice {
            Ok(Some(path)) => {
                self.path = path.display().to_string();
                self.open();
            }
            Ok(None) => self.status = "open cancelled".to_string(),
            Err(error) => {
                let note = format!("native open dialog unavailable ({error}); using path field: ");
                self.open();
                self.status = format!("{note}{}", self.status);
            }
        }
    }

    /// Native save-as dialog; keeps the path field in sync with the choice.
    pub fn save_with_dialog(&mut self) {
        let current = self.resolve_path();
        let directory = self.dialog_directory();
        let mut builder = native_dialog::FileDialogBuilder::default()
            .set_title("Save scenario as")
            .add_filter("Scenario INI", ["ini"]);
        if directory.exists() {
            builder = builder.set_location(&directory);
        }
        if let Some(name) = current.file_name() {
            builder = builder.set_filename(name.to_string_lossy());
        }
        let choice = builder
            .save_single_file()
            .show()
            .map_err(|error| error.to_string());
        self.handle_save_choice(choice);
    }

    fn handle_save_choice(&mut self, choice: Result<Option<PathBuf>, String>) {
        match choice {
            Ok(Some(path)) => {
                self.path = path.display().to_string();
                self.save();
            }
            Ok(None) => self.status = "save cancelled".to_string(),
            Err(error) => {
                let note = format!("native save dialog unavailable ({error}); using path field: ");
                self.save();
                self.status = format!("{note}{}", self.status);
            }
        }
    }

    /// Refresh the scenario-file list from the default maps directory.
    fn load_scenario_files(&mut self) {
        self.scenario_files = scenario_files_in(std::path::Path::new("maps"));
        self.scenario_previews = self
            .scenario_files
            .iter()
            .map(|path| scenario_preview(path))
            .collect();
        self.scenario_hover = None;
    }

    /// Trigger ids in draw/selection order.
    fn trigger_ids(&self) -> Vec<u32> {
        let mut ids: Vec<u32> = self.map.triggers.iter().map(|trigger| trigger.id).collect();
        ids.sort_unstable();
        ids
    }

    /// Trigger ids matching the trigger-folder filter.
    #[allow(dead_code)]
    fn visible_trigger_ids(&self) -> Vec<u32> {
        let query = self.trigger_filter.trim().to_ascii_lowercase();
        self.trigger_ids()
            .into_iter()
            .filter(|id| {
                if query.is_empty() {
                    return true;
                }
                self.map
                    .triggers
                    .iter()
                    .find(|trigger| trigger.id == *id)
                    .is_some_and(|trigger| {
                        trigger.name.to_ascii_lowercase().contains(&query)
                            || trigger.group.to_ascii_lowercase().contains(&query)
                            || trigger.condition.to_ascii_lowercase().contains(&query)
                    })
            })
            .collect()
    }

    fn placed_filter_options(&self) -> Vec<PlacedFilter> {
        let mut classes: Vec<ClassId> = self
            .map
            .objects
            .values()
            .map(|object| object.class_id)
            .collect();
        classes.sort_by_key(|class_id| format!("{class_id:?}"));
        classes.dedup();
        let mut options = vec![
            PlacedFilter::All,
            PlacedFilter::Unassigned,
            PlacedFilter::Player,
            PlacedFilter::Enemy,
        ];
        options.extend(classes.into_iter().map(PlacedFilter::Class));
        options
    }

    fn next_placed_filter(&mut self) {
        let options = self.placed_filter_options();
        let index = options
            .iter()
            .position(|filter| *filter == self.placed_filter)
            .unwrap_or(0);
        self.placed_filter = options[(index + 1) % options.len()];
        self.status = format!("object list: {}", self.placed_filter.label());
    }

    /// Does a placed object match the Objects-tab search box?
    ///
    /// Plain text matches the class name or owner label ("player", "enemy",
    /// "none"/"neutral"); `#123` matches one object id and `@x,y` one cell.
    fn placed_object_matches(
        &self,
        id: u32,
        object: &MapObject,
        query: &str,
        player: ClassId,
        enemy: ClassId,
    ) -> bool {
        let query = query.trim().to_ascii_lowercase();
        if query.is_empty() {
            return true;
        }
        if let Some(id_query) = query.strip_prefix('#') {
            return id.to_string() == id_query.trim();
        }
        if let Some(cell_query) = query.strip_prefix('@') {
            return format!("{},{}", object.position.x, object.position.y) == cell_query.trim();
        }
        let owner = match object.owner {
            None => "none neutral".to_string(),
            Some(owner) if owner == player => "player".to_string(),
            Some(owner) if owner == enemy => "enemy".to_string(),
            Some(owner) => format!("{owner:?}").to_ascii_lowercase(),
        };
        format!("{:?}", object.class_id)
            .to_ascii_lowercase()
            .contains(query.as_str())
            || owner.contains(query.as_str())
    }

    /// Placed object ids matching the active owner/class filter, in id order.
    fn object_ids(&self) -> Vec<u32> {
        let player = self.owner_house();
        let enemy = if self.plugin_index == 0 {
            ClassId::HouseNod
        } else {
            ClassId::HouseSoviet
        };
        let mut ids: Vec<u32> = self
            .map
            .objects
            .iter()
            .filter(|(_, object)| match self.placed_filter {
                PlacedFilter::All => true,
                PlacedFilter::Unassigned => object.owner.is_none(),
                PlacedFilter::Player => object.owner == Some(player),
                PlacedFilter::Enemy => object.owner == Some(enemy),
                PlacedFilter::Class(class_id) => object.class_id == class_id,
            })
            .filter(|(id, object)| {
                self.placed_object_matches(**id, object, &self.placed_search, player, enemy)
            })
            .map(|(id, _)| *id)
            .collect();
        ids.sort_unstable();
        ids
    }

    /// Object catalog filtered by the palette search box.
    fn visible_objects(&self) -> Vec<ClassId> {
        let plugin = self.plugin();
        let query = self.object_search.trim().to_ascii_lowercase();
        plugin
            .available_objects()
            .into_iter()
            .filter(|class_id| self.object_category.matches(*class_id))
            .filter(|class_id| {
                if query.is_empty() {
                    return true;
                }
                let label = plugin.get_object_name(*class_id).to_ascii_lowercase();
                label.contains(&query)
                    || format!("{class_id:?}")
                        .to_ascii_lowercase()
                        .contains(&query)
                    || plugin
                        .object_side(*class_id)
                        .is_some_and(|side| side.to_ascii_lowercase().contains(&query))
            })
            .collect()
    }

    /// Select every placed object inside the inclusive cell rectangle.
    fn select_in_rect(&mut self, a: Vec2i, b: Vec2i) -> usize {
        let (x0, x1) = (a.x.min(b.x), a.x.max(b.x));
        let (y0, y1) = (a.y.min(b.y), a.y.max(b.y));
        let mut ids: Vec<u32> = self
            .map
            .objects
            .values()
            .filter(|object| {
                object.position.x >= x0
                    && object.position.x <= x1
                    && object.position.y >= y0
                    && object.position.y <= y1
            })
            .map(|object| object.id)
            .collect();
        ids.sort_unstable();
        self.editor.selected_objects = ids.clone();
        self.editor.selected_object = ids.first().copied();
        ids.len()
    }

    /// Finish a marquee drag and apply its selection.
    fn finish_marquee(&mut self) {
        if !self.marquee_active {
            return;
        }
        if let (Some(start), Some(end)) = (self.marquee_start, self.marquee_end) {
            let count = self.select_in_rect(start, end);
            self.status = format!("marquee selected {count} object(s)");
        }
        self.marquee_active = false;
        self.marquee_start = None;
        self.marquee_end = None;
    }

    /// Trigger templates come from the active game plugin, so TD and RA offer
    /// their own authored conditions/actions.
    fn trigger_templates(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        self.plugin().trigger_templates()
    }

    fn current_trigger_template(&self) -> (String, String, String) {
        let templates = self.trigger_templates();
        if templates.is_empty() {
            return (
                "New Trigger".to_string(),
                "Always".to_string(),
                "None".to_string(),
            );
        }
        let index = self.trigger_template_index % templates.len();
        let (name, condition, action) = templates[index];
        (name.to_string(), condition.to_string(), action.to_string())
    }

    fn next_trigger_template(&mut self) {
        let count = self.trigger_templates().len().max(1);
        self.trigger_template_index = (self.trigger_template_index + 1) % count;
        let (name, condition, action) = self.current_trigger_template();
        if self.editor.tool_name() == Some("Trigger") {
            self.editor.set_tool(Box::new(TriggerTool::new(
                name.clone(),
                condition.clone(),
                action.clone(),
            )));
        }
        self.status = format!("trigger template: {name} ({condition} / {action})");
    }

    /// Remember every selected object so the group can be pasted elsewhere.
    pub fn copy_selected_object(&mut self) {
        let mut ids = if self.editor.selected_objects.is_empty() {
            self.editor
                .selected_object
                .into_iter()
                .collect::<Vec<u32>>()
        } else {
            self.editor.selected_objects.clone()
        };
        ids.sort_unstable();
        ids.dedup();
        let objects: Vec<MapObject> = ids
            .iter()
            .filter_map(|id| self.map.objects.get(id).cloned())
            .collect();
        if objects.is_empty() {
            self.status = "copy: select an object first".to_string();
            return;
        }
        self.status = format!("copied {} object(s)", objects.len());
        self.clipboard_objects = objects;
    }

    /// Paste the copied group at the cursor as one undoable step.
    pub fn paste_object(&mut self) {
        if self.clipboard_objects.is_empty() {
            self.status = "paste: clipboard is empty (Ctrl+C to copy an object)".to_string();
            return;
        }
        let cell = self.cursor_cell;
        if !self.map.in_bounds(cell) {
            self.status = "paste: cursor is outside the map".to_string();
            return;
        }
        let anchor_x = self
            .clipboard_objects
            .iter()
            .map(|object| object.position.x)
            .min()
            .unwrap_or(cell.x);
        let anchor_y = self
            .clipboard_objects
            .iter()
            .map(|object| object.position.y)
            .min()
            .unwrap_or(cell.y);
        let delta = Vec2i::new(cell.x - anchor_x, cell.y - anchor_y);

        let mut placed = Vec::new();
        let mut skipped = 0usize;
        self.editor.history.begin_group("Paste objects".to_string());
        for template in self.clipboard_objects.clone() {
            let target = Vec2i::new(template.position.x + delta.x, template.position.y + delta.y);
            if !self.map.in_bounds(target) || self.map.object_at(target).is_some() {
                skipped += 1;
                continue;
            }
            let command = crate::history::PlaceObject::new(MapObject {
                id: 0,
                class_id: template.class_id,
                position: target,
                owner: template.owner,
                ..Default::default()
            });
            self.editor
                .history
                .execute(&mut self.map, Box::new(command));
            if let Some(id) = self.map.object_at(target).map(|object| object.id) {
                placed.push(id);
            }
        }
        self.editor.history.end_group();
        self.editor.selected_objects = placed.clone();
        self.editor.selected_object = placed.first().copied();
        self.status = format!(
            "pasted {} object(s) at ({},{}){}",
            placed.len(),
            cell.x,
            cell.y,
            if skipped > 0 {
                format!(", skipped {skipped} occupied/out-of-bounds")
            } else {
                String::new()
            }
        );
    }

    /// Duplicate the current selection one cell down-right (Ctrl+D).
    pub fn duplicate_selection(&mut self) {
        let ids = self.selected_object_ids();
        if ids.is_empty() {
            self.status = "duplicate: select an object first".to_string();
            return;
        }
        let templates: Vec<MapObject> = ids
            .iter()
            .filter_map(|id| self.map.objects.get(id).cloned())
            .collect();
        let delta = Vec2i::new(1, 1);
        let mut placed = Vec::new();
        let mut skipped = 0usize;
        self.editor
            .history
            .begin_group(format!("Duplicate {} objects", templates.len()));
        for template in templates {
            let target = Vec2i::new(template.position.x + delta.x, template.position.y + delta.y);
            if !self.map.in_bounds(target) || self.map.object_at(target).is_some() {
                skipped += 1;
                continue;
            }
            let command = crate::history::PlaceObject::new(MapObject {
                id: 0,
                class_id: template.class_id,
                position: target,
                owner: template.owner,
                ..Default::default()
            });
            self.editor
                .history
                .execute(&mut self.map, Box::new(command));
            if let Some(id) = self.map.object_at(target).map(|object| object.id) {
                placed.push(id);
            }
        }
        self.editor.history.end_group();
        if placed.is_empty() {
            self.status = "duplicate: no free cells".to_string();
            return;
        }
        self.editor.selected_objects = placed.clone();
        self.editor.selected_object = placed.first().copied();
        self.status = format!(
            "duplicated {} object(s){}",
            placed.len(),
            if skipped > 0 {
                format!(", skipped {skipped} occupied/out-of-bounds")
            } else {
                String::new()
            }
        );
    }

    /// Objects affected by a property edit: the marquee selection, else the
    /// primary object.
    fn selected_object_ids(&self) -> Vec<u32> {
        if self.editor.selected_objects.is_empty() {
            self.editor.selected_object.into_iter().collect()
        } else {
            self.editor.selected_objects.clone()
        }
    }

    fn owner_options(&self) -> Vec<Option<ClassId>> {
        let enemy = if self.plugin_index == 0 {
            ClassId::HouseNod
        } else {
            ClassId::HouseSoviet
        };
        vec![None, Some(self.owner_house()), Some(enemy)]
    }

    /// Cycle the owning house on every selected object (undoable group).
    fn cycle_object_owner(&mut self) {
        let ids = self.selected_object_ids();
        if ids.is_empty() {
            self.status = "owner: select an object first".to_string();
            return;
        }
        let options = self.owner_options();
        let mut changed = 0;
        self.editor
            .history
            .begin_group("Set object owner".to_string());
        for id in ids {
            let Some(owner) = self.map.objects.get(&id).map(|object| object.owner) else {
                continue;
            };
            let index = options
                .iter()
                .position(|option| *option == owner)
                .unwrap_or(0);
            let after = options[(index + 1) % options.len()];
            if let Some(command) = crate::history::SetObjectOwner::new(&self.map, id, after) {
                self.editor
                    .history
                    .execute(&mut self.map, Box::new(command));
                changed += 1;
            }
        }
        self.editor.history.end_group();
        let owner = self
            .editor
            .selected_object
            .and_then(|id| self.map.objects.get(&id))
            .map(|object| format!("{:?}", object.owner))
            .unwrap_or_else(|| "none".to_string());
        self.status = format!("owner set on {changed} object(s) -> {owner}");
    }

    /// Move every selected object by one cell (undoable group).
    fn nudge_selected_object(&mut self, dx: i32, dy: i32) {
        let ids = self.selected_object_ids();
        if ids.is_empty() {
            self.status = "nudge: select an object first".to_string();
            return;
        }
        let mut moved = 0;
        let mut blocked = 0;
        self.editor.history.begin_group("Move object".to_string());
        for id in ids {
            let Some(from) = self.map.objects.get(&id).map(|object| object.position) else {
                continue;
            };
            let to = Vec2i::new(from.x + dx, from.y + dy);
            if !self.map.in_bounds(to) || self.map.object_at(to).is_some() {
                blocked += 1;
                continue;
            }
            let command = crate::history::MoveObject { id, from, to };
            self.editor
                .history
                .execute(&mut self.map, Box::new(command));
            moved += 1;
        }
        self.editor.history.end_group();
        self.status = format!("nudged {moved} object(s), {blocked} blocked");
    }

    /// Ctrl+C dispatches on the active inspector tab.
    pub fn copy_selection(&mut self) {
        if self.inspector_tab == InspectorTab::Triggers {
            self.copy_selected_trigger();
        } else {
            self.copy_selected_object();
        }
    }

    /// Ctrl+V dispatches on the active inspector tab.
    pub fn paste_selection(&mut self) {
        if self.inspector_tab == InspectorTab::Triggers {
            self.paste_trigger();
        } else {
            self.paste_object();
        }
    }

    fn copy_selected_trigger(&mut self) {
        let Some(id) = self.selected_trigger else {
            self.status = "trigger copy: select a trigger first".to_string();
            return;
        };
        let Some(trigger) = self
            .map
            .triggers
            .iter()
            .find(|trigger| trigger.id == id)
            .cloned()
        else {
            self.status = "trigger copy: selection no longer exists".to_string();
            return;
        };
        self.clipboard_trigger = Some(trigger);
        self.status = format!("copied trigger {id}");
    }

    fn paste_trigger(&mut self) {
        let Some(template) = self.clipboard_trigger.clone() else {
            self.status = "trigger paste: clipboard is empty (Ctrl+C on a trigger)".to_string();
            return;
        };
        self.commit_trigger_edit();
        let command = crate::history::AddTrigger {
            trigger: Trigger {
                id: 0,
                name: format!("{} copy", template.name),
                condition: template.condition,
                action: template.action,
                group: template.group,
            },
        };
        self.editor
            .history
            .execute(&mut self.map, Box::new(command));
        if let Some(id) = self.map.triggers.last().map(|trigger| trigger.id) {
            self.select_trigger(id);
            self.status = format!("pasted trigger as {id}");
        }
    }

    /// Map validation plus plugin-level object checks (owner/bounds).
    ///
    /// Parse warnings from the most recent original-format import are listed
    /// first so the validation panel explains what the importer had to skip.
    fn validation_issues(&self) -> Vec<String> {
        let mut issues: Vec<String> = self
            .import_warnings
            .iter()
            .map(|warning| format!("import: {warning}"))
            .collect();
        issues.extend(self.map.validate());
        let plugin = self.plugin();
        for object in self.map.objects.values() {
            if !plugin.validate_object(&self.map, object) {
                issues.push(format!(
                    "object #{} {} at ({},{}) has an invalid owner/position",
                    object.id,
                    object.class_name(),
                    object.position.x,
                    object.position.y
                ));
            }
        }
        issues
    }

    /// Human-readable scenario statistics for the Validate tab.
    fn scenario_stats(&self) -> Vec<String> {
        let buildings = self
            .map
            .objects
            .values()
            .filter(|object| object.class_id.is_building())
            .count();
        let units = self.map.objects.len() - buildings;
        let mut owner_counts: std::collections::BTreeMap<&'static str, usize> =
            std::collections::BTreeMap::new();
        for object in self.map.objects.values() {
            *owner_counts.entry(owner_label(object.owner)).or_insert(0) += 1;
        }
        let owners = owner_counts
            .iter()
            .map(|(owner, count)| format!("{owner} {count}"))
            .collect::<Vec<_>>()
            .join(", ");
        let mut counts = vec![0usize; 256];
        for id in &self.map.terrain {
            counts[*id as usize] += 1;
        }
        let mut entries: Vec<(usize, usize)> = counts
            .into_iter()
            .enumerate()
            .filter(|(_, count)| *count > 0)
            .collect();
        entries.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        let terrain = entries
            .iter()
            .take(4)
            .map(|(id, count)| {
                format!(
                    "{} {}",
                    TerrainType::from_id(*id as u8).map_or("?", TerrainType::label),
                    count
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        vec![
            format!("map: {}", self.map.summary()),
            format!(
                "objects: {} ({} buildings, {} units)",
                self.map.objects.len(),
                buildings,
                units
            ),
            format!(
                "triggers: {}  starts: {}",
                self.map.triggers.len(),
                self.map.player_starts.len()
            ),
            format!("terrain: {terrain}"),
            format!(
                "owners: {}",
                if owners.is_empty() { "none" } else { &owners }
            ),
        ]
    }

    /// Select a trigger and load its fields into the edit buffer.
    pub fn select_trigger(&mut self, id: u32) {
        if self.selected_trigger == Some(id) && self.trigger_edit.is_some() {
            return;
        }
        self.commit_trigger_edit();
        let Some(trigger) = self.map.triggers.iter().find(|trigger| trigger.id == id) else {
            return;
        };
        self.trigger_edit = Some(TriggerEdit::from_trigger(trigger));
        self.selected_trigger = Some(id);
        self.focus = Focus::None;
        self.status = format!(
            "editing trigger {} ({} / {})",
            trigger.name, trigger.condition, trigger.action
        );
    }

    /// Flush buffered trigger field edits as one undoable command.
    pub fn commit_trigger_edit(&mut self) -> bool {
        let Some(edit) = self.trigger_edit.clone() else {
            return false;
        };
        let Some(command) = SetTrigger::new(&self.map, edit.to_trigger()) else {
            return false;
        };
        let label = command.label();
        self.editor
            .history
            .execute(&mut self.map, Box::new(command));
        self.status = format!(
            "{label} (condition: {}, action: {})",
            edit.condition, edit.action
        );
        true
    }

    /// Reload the edit buffer from the model (after undo/redo, add or delete).
    fn refresh_trigger_edit(&mut self) {
        let selected = self
            .selected_trigger
            .filter(|id| self.map.triggers.iter().any(|trigger| trigger.id == *id));
        self.selected_trigger = selected;
        self.trigger_edit = selected.and_then(|id| {
            self.map
                .triggers
                .iter()
                .find(|trigger| trigger.id == id)
                .map(TriggerEdit::from_trigger)
        });
        if self.trigger_edit.is_none() {
            self.selected_trigger = None;
        }
    }

    // ------------------------------------------------------------------
    // Agent Studio
    // ------------------------------------------------------------------

    pub fn open_agent_studio(&mut self) {
        if self.agent_studio {
            return;
        }
        self.commit_trigger_edit();
        self.agent_studio = true;
        self.agent_report.clear();
        let selected = self
            .selected_agent
            .filter(|id| self.map.agent(*id).is_some())
            .or_else(|| self.map.agents.first().map(|agent| agent.id));
        self.selected_agent = selected;
        self.refresh_agent_edit();
        self.focus = if self.agent_edit.is_some() {
            Focus::AgentScript
        } else {
            Focus::None
        };
        self.agent_message = if self.agent_edit.is_some() {
            "type a script; Ctrl+S saves the map".to_string()
        } else {
            "no agents yet: press New to create one".to_string()
        };
    }

    pub fn close_agent_studio(&mut self) {
        self.commit_agent_edit();
        self.agent_studio = false;
        self.focus = Focus::None;
        self.agent_message = "agent studio closed".to_string();
    }

    pub fn commit_agent_edit(&mut self) -> bool {
        let Some(edit) = self.agent_edit.clone() else {
            return false;
        };
        let after = edit.to_agent();
        let Some(command) = SetAgent::new(&self.map, after) else {
            return false;
        };
        let label = command.label();
        self.editor
            .history
            .execute(&mut self.map, Box::new(command));
        self.status = label;
        true
    }

    fn refresh_agent_edit(&mut self) {
        self.agent_edit = self
            .selected_agent
            .and_then(|id| self.map.agent(id))
            .map(AgentEdit::from_agent);
    }

    pub fn select_agent(&mut self, id: u32) {
        if self.selected_agent == Some(id) {
            return;
        }
        self.commit_agent_edit();
        self.selected_agent = Some(id);
        self.refresh_agent_edit();
        if let Some(edit) = self.agent_edit.as_mut() {
            edit.ensure_visible(18);
        }
    }

    pub fn new_agent(&mut self) {
        self.commit_agent_edit();
        let mut index = self.map.agents.len() + 1;
        let name = loop {
            let candidate = format!("Agent {index}");
            if !self.map.agents.iter().any(|agent| agent.name == candidate) {
                break candidate;
            }
            index += 1;
        };
        let next_id = self
            .map
            .agents
            .iter()
            .map(|agent| agent.id)
            .max()
            .unwrap_or(0)
            + 1;
        let mut agent = AgentDefinition::new(next_id, name);
        agent.owner = format!("{:?}", self.owner_house());
        let id = agent.id;
        self.editor
            .history
            .execute(&mut self.map, Box::new(AddAgent { agent }));
        self.selected_agent = Some(id);
        self.refresh_agent_edit();
        self.focus = Focus::AgentScript;
        self.agent_message = format!("created agent {id}");
    }

    pub fn duplicate_agent(&mut self) {
        let Some(edit) = self.agent_edit.clone() else {
            return;
        };
        let mut agent = edit.to_agent();
        agent.id = 0;
        agent.name = format!("{} copy", agent.name);
        let command = AddAgent { agent };
        self.editor
            .history
            .execute(&mut self.map, Box::new(command));
        let id = self.map.agents.last().map(|agent| agent.id).unwrap_or(0);
        self.selected_agent = Some(id);
        self.refresh_agent_edit();
        self.agent_message = "duplicated agent".to_string();
    }

    pub fn delete_agent(&mut self) {
        let Some(id) = self.selected_agent else {
            return;
        };
        let Some(command) = RemoveAgent::new(&self.map, id) else {
            return;
        };
        let label = command.label();
        self.editor
            .history
            .execute(&mut self.map, Box::new(command));
        self.selected_agent = self.map.agents.first().map(|agent| agent.id);
        self.refresh_agent_edit();
        self.agent_message = label;
    }

    pub fn cycle_agent_owner(&mut self) {
        let options = self.owner_options();
        let Some(edit) = self.agent_edit.as_mut() else {
            return;
        };
        let index = options
            .iter()
            .position(|option| *option == edit.owner)
            .unwrap_or(0);
        edit.owner = options[(index + 1) % options.len()];
        self.agent_message = format!("owner -> {:?}", edit.owner);
    }

    pub fn cycle_agent_doctrine(&mut self) {
        let Some(edit) = self.agent_edit.as_mut() else {
            return;
        };
        let index = DOCTRINES
            .iter()
            .position(|doctrine| *doctrine == edit.doctrine)
            .unwrap_or(0);
        edit.doctrine = DOCTRINES[(index + 1) % DOCTRINES.len()].to_string();
        self.agent_message = format!("doctrine -> {}", edit.doctrine);
    }

    pub fn check_agent(&mut self) {
        let Some(edit) = self.agent_edit.clone() else {
            return;
        };
        let mut report = Vec::new();
        self.agent_bad.remove(&edit.id);
        match compile(&edit.script) {
            Ok(program) => {
                let slots = intent_slots(&program);
                report.push(format!(
                    "OK: {} handler(s), {} intent slot(s)",
                    program.handlers.len(),
                    slots.len()
                ));
                let catalog: Vec<String> = self
                    .plugin()
                    .available_objects()
                    .iter()
                    .map(|class| format!("{class:?}"))
                    .collect();
                for reference in class_references(&program) {
                    if !catalog
                        .iter()
                        .any(|class| class.eq_ignore_ascii_case(&reference.name))
                    {
                        report.push(format!(
                            "warning {}:{}: {} is not in this game catalog",
                            reference.line, reference.column, reference.name
                        ));
                    }
                }
            }
            Err(diagnostics) => {
                self.agent_bad.insert(edit.id);
                report.push("script has errors".to_string());
                for diagnostic in diagnostics {
                    report.push(format!("error {diagnostic}"));
                }
            }
        }
        self.agent_report = report;
        self.agent_message = "checked script".to_string();
    }

    /// Compile every authored agent and report pass/fail per agent.
    pub fn check_all_agents(&mut self) {
        let mut report = Vec::new();
        self.agent_bad.clear();
        if self.map.agents.is_empty() {
            self.agent_message = "no agents to check".to_string();
            self.agent_report = vec!["No agents in this scenario".to_string()];
            return;
        }
        for agent in &self.map.agents {
            match compile(&agent.script) {
                Ok(program) => report.push(format!(
                    "OK #{} {}: {} handler(s)",
                    agent.id,
                    agent.name,
                    program.handlers.len()
                )),
                Err(diagnostics) => {
                    self.agent_bad.insert(agent.id);
                    let first = diagnostics
                        .first()
                        .map(|diagnostic| diagnostic.to_string())
                        .unwrap_or_else(|| "unknown error".to_string());
                    report.push(format!(
                        "FAIL #{} {}: {} error(s), first: {first}",
                        agent.id,
                        agent.name,
                        diagnostics.len()
                    ));
                }
            }
        }
        self.agent_message = format!(
            "checked {} agents, {} failed",
            self.map.agents.len(),
            self.agent_bad.len()
        );
        self.agent_report = report;
    }

    pub fn run_agent(&mut self) {
        let Some(edit) = self.agent_edit.clone() else {
            return;
        };
        let program = match compile(&edit.script) {
            Ok(program) => program,
            Err(diagnostics) => {
                self.agent_report = diagnostics
                    .iter()
                    .map(|diagnostic| format!("error {diagnostic}"))
                    .collect();
                self.agent_message = "run blocked by compile errors".to_string();
                return;
            }
        };
        let request = self.build_oracle_request(&edit, &program);
        let mut runtime = AgentRuntime::new(program, u64::from(edit.id) ^ 0x00C0_FFEE);
        if let Some(request) = &request {
            let path = self.oracle_dir().join("ai_oracle_response.json");
            if let Ok(text) = std::fs::read_to_string(&path) {
                match parse_response(request, &text) {
                    Ok(cache) => {
                        runtime.set_intents(cache);
                    }
                    Err(error) => {
                        self.agent_report
                            .push(format!("ignored {}: {error}", path.display()));
                    }
                }
            }
        }

        let mut report = Vec::new();
        let mut commands = 0usize;
        let mut logs = 0usize;
        let mut missing: Vec<String> = Vec::new();
        let mut diagnostics: Vec<String> = Vec::new();
        let start = runtime.on_start(&self.agent_observation(&edit, 0, 3_000));
        commands += start.commands.len();
        logs += start.logs.len();
        diagnostics.extend(start.diagnostics);
        for tick in 0..300u64 {
            let observation =
                self.agent_observation(&edit, tick, 3_000 + i64::try_from(tick).unwrap_or(0) * 15);
            let step = runtime.on_tick(&observation);
            commands += step.commands.len();
            logs += step.logs.len();
            diagnostics.extend(step.diagnostics);
            for key in step.intents_missing {
                if !missing.contains(&key) {
                    missing.push(key);
                }
            }
            if commands > 0 || !diagnostics.is_empty() {
                if let Some(command) = step.commands.first() {
                    if report.len() < 10 {
                        report.push(format!("tick {tick}: {command:?}"));
                    }
                }
            }
        }
        report.insert(
            0,
            format!("dry run: 300 ticks, {commands} command(s), {logs} log line(s)"),
        );
        if !missing.is_empty() {
            report.push(format!(
                "{} intent key(s) had no recorded answer: {}",
                missing.len(),
                missing.join(", ")
            ));
        }
        for diagnostic in diagnostics.into_iter().take(5) {
            report.push(format!("runtime {diagnostic}"));
        }
        self.agent_report = report;
        self.agent_message = "dry run finished (static world)".to_string();
    }

    fn build_oracle_request(
        &self,
        edit: &AgentEdit,
        program: &cnc_ai::Program,
    ) -> Option<OracleRequest> {
        let owner = edit
            .owner
            .map(|owner| format!("{owner:?}"))
            .unwrap_or_default();
        Some(OracleRequest::new(
            format!("editor-{}-{}", edit.id, edit.name.replace(' ', "_")),
            edit.name.clone(),
            owner,
            edit.doctrine.clone(),
            self.agent_observation(edit, 0, 3_000),
            Some(self.agent_vision(edit.owner)),
            intent_slots(program),
        ))
    }

    pub fn write_agent_request(&mut self) {
        let Some(edit) = self.agent_edit.clone() else {
            return;
        };
        let program = match compile(&edit.script) {
            Ok(program) => program,
            Err(diagnostics) => {
                self.agent_report = diagnostics
                    .iter()
                    .map(|diagnostic| format!("error {diagnostic}"))
                    .collect();
                self.agent_message = "fix compile errors before exporting a request".to_string();
                return;
            }
        };
        let Some(mut request) = self.build_oracle_request(&edit, &program) else {
            return;
        };
        let dir = self.oracle_dir();
        let png_path = dir.join("ai_vision.png");
        let mut image_warning = None;
        if let Some(vision) = &request.vision {
            match self.write_vision_png(vision, &png_path) {
                Ok(()) => {
                    request = request.with_vision_image(png_path.display().to_string());
                }
                Err(error) => image_warning = Some(error),
            }
        }
        let json = match serde_json::to_string_pretty(&request) {
            Ok(json) => json,
            Err(error) => {
                self.agent_message = format!("could not encode request: {error}");
                return;
            }
        };
        let json_path = dir.join("ai_oracle_request.json");
        let prompt_path = dir.join("ai_oracle_prompt.txt");
        let vision = request
            .vision
            .as_ref()
            .map(|vision| (vision.width, vision.height))
            .unwrap_or((0, 0));
        let written = std::fs::write(&json_path, json)
            .and_then(|_| std::fs::write(&prompt_path, observation_prompt(&request)));
        match written {
            Ok(()) => {
                self.agent_report = vec![
                    format!("request: {}", json_path.display()),
                    format!("prompt:  {}", prompt_path.display()),
                    format!(
                        "vision:  {}x{} ({})",
                        vision.0,
                        vision.1,
                        png_path.display()
                    ),
                    "run scripts/ai_oracle.py to produce ai_oracle_response.json".to_string(),
                ];
                if let Some(error) = image_warning {
                    self.agent_report.push(error);
                }
                self.agent_message = "oracle request exported".to_string();
            }
            Err(error) => {
                self.agent_message = format!("could not write request: {error}");
            }
        }
    }

    fn oracle_dir(&self) -> PathBuf {
        Path::new(&self.path)
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."))
    }

    fn write_vision_png(&self, vision: &Vision, path: &Path) -> Result<(), String> {
        const SCALE: u32 = 8;
        let mut image = image::RgbaImage::new(vision.width * SCALE, vision.height * SCALE);
        for y in 0..vision.height {
            for x in 0..vision.width {
                let cell = vision
                    .cells
                    .get((y * vision.width + x) as usize)
                    .copied()
                    .unwrap_or(VISION_EMPTY);
                let color = match cell {
                    VISION_OWN => [60u8, 120, 255, 255],
                    VISION_ENEMY => [230, 70, 70, 255],
                    VISION_RESOURCE => [240, 210, 80, 255],
                    VISION_BLOCKED => [70, 78, 90, 255],
                    _ => [30, 60, 40, 255],
                };
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        image.put_pixel(x * SCALE + dx, y * SCALE + dy, image::Rgba(color));
                    }
                }
            }
        }
        image
            .save(path)
            .map_err(|error| format!("could not write {}: {error}", path.display()))
    }

    fn agent_observation(&self, edit: &AgentEdit, tick: u64, credits: i64) -> Observation {
        let mut units = std::collections::BTreeMap::new();
        let mut buildings = std::collections::BTreeMap::new();
        let mut base_sum = (0i64, 0i64, 0i64);
        let mut contacts = Vec::new();
        let mut threat = 0i64;
        for object in self.map.objects.values() {
            let name = object.class_name();
            if edit.owner.is_some() && object.owner == edit.owner {
                if object.class_id.is_building() {
                    *buildings.entry(name).or_insert(0i64) += 1;
                    base_sum.0 += i64::from(object.position.x);
                    base_sum.1 += i64::from(object.position.y);
                    base_sum.2 += 1;
                } else {
                    *units.entry(name).or_insert(0i64) += 1;
                }
            } else if object.owner.is_some() && object.owner != edit.owner {
                let danger = if object.class_id.is_building() { 3 } else { 1 };
                threat += danger;
                contacts.push(Contact {
                    x: object.position.x,
                    y: object.position.y,
                    class: name,
                    threat: danger,
                    visible: true,
                });
            }
        }
        contacts.sort_by_key(|contact| (contact.x, contact.y));
        let base = if base_sum.2 > 0 {
            Some(Pos::new(
                (base_sum.0 / base_sum.2) as i32,
                (base_sum.1 / base_sum.2) as i32,
            ))
        } else {
            Some(Pos::new(
                (self.map.width / 2) as i32,
                (self.map.height / 2) as i32,
            ))
        };
        Observation {
            tick,
            credits,
            power: 100,
            power_ratio: 100,
            stance: "defensive".to_string(),
            map_width: self.map.width as i32,
            map_height: self.map.height as i32,
            base,
            units,
            buildings,
            contacts,
            threat,
        }
    }

    fn agent_vision(&self, owner: Option<ClassId>) -> Vision {
        let width = 32u32;
        let height = 24u32;
        let mut cells = vec![VISION_EMPTY; (width * height) as usize];
        for y in 0..height {
            for x in 0..width {
                let mx = x * self.map.width / width;
                let my = y * self.map.height / height;
                let index = (my * self.map.width + mx) as usize;
                let terrain = self.map.terrain.get(index).copied().unwrap_or(0);
                let overlay = self.map.overlay.get(index).copied().unwrap_or(0);
                let cell = if terrain == 1 {
                    VISION_BLOCKED
                } else if overlay != 0 {
                    VISION_RESOURCE
                } else {
                    VISION_EMPTY
                };
                cells[(y * width + x) as usize] = cell;
            }
        }
        for object in self.map.objects.values() {
            let vx =
                (object.position.x.max(0) as u32 * width / self.map.width.max(1)).min(width - 1);
            let vy =
                (object.position.y.max(0) as u32 * height / self.map.height.max(1)).min(height - 1);
            let cell = if object.owner == owner {
                VISION_OWN
            } else {
                VISION_ENEMY
            };
            cells[(vy * width + vx) as usize] = cell;
        }
        Vision {
            width,
            height,
            cells,
        }
    }

    fn studio_layout(&self) -> StudioLayout {
        let x = 12.0;
        let y = 8.0;
        let w = (self.width - 24.0).max(560.0);
        let h = (self.height - STATUS_H - 16.0).max(420.0);
        let button_y = y + 8.0;
        // Widths keep every label at the full 8px font (chars * 10px + 6px);
        // the row is right-anchored with an 8px inset from the panel edge.
        let widths = [44.0, 44.0, 44.0, 56.0, 48.0, 40.0, 40.0, 56.0];
        let kinds = [
            StudioButton::New,
            StudioButton::Duplicate,
            StudioButton::Delete,
            StudioButton::Check,
            StudioButton::CheckAll,
            StudioButton::Run,
            StudioButton::Oracle,
            StudioButton::Close,
        ];
        let mut buttons = [(StudioButton::Close, RectF::new(0.0, 0.0, 0.0, 0.0)); 8];
        let total = widths.iter().sum::<f32>() + (widths.len() - 1) as f32 * 6.0;
        let mut button_x = x + w - 8.0 - total;
        for index in 0..8 {
            buttons[index] = (
                kinds[index],
                RectF::new(button_x, button_y, widths[index], 22.0),
            );
            button_x += widths[index] + 6.0;
        }
        StudioLayout {
            x,
            y,
            w,
            h,
            name: RectF::new(x + 52.0, y + 40.0, 182.0, 22.0),
            owner: RectF::new(x + 246.0, y + 40.0, 122.0, 22.0),
            doctrine: RectF::new(x + 378.0, y + 40.0, 122.0, 22.0),
            list: RectF::new(x + 10.0, y + 72.0, 232.0, h - 182.0),
            code: RectF::new(x + 250.0, y + 72.0, w - 260.0, h - 182.0),
            report: RectF::new(x + 10.0, y + h - 104.0, w - 20.0, 96.0),
            buttons,
        }
    }

    pub fn pointer_down(&mut self, x: f32, y: f32, button: PointerButton) {
        self.mouse = Some((x, y));
        if self.agent_studio {
            if matches!(button, PointerButton::Left) {
                self.agent_studio_click(x, y);
            }
            return;
        }
        self.pointer_down_map(x, y, button);
    }

    fn agent_studio_click(&mut self, x: f32, y: f32) {
        let layout = self.studio_layout();
        for (kind, rect) in layout.buttons {
            if rect.contains(x, y) {
                match kind {
                    StudioButton::New => self.new_agent(),
                    StudioButton::Duplicate => self.duplicate_agent(),
                    StudioButton::Delete => self.delete_agent(),
                    StudioButton::Check => self.check_agent(),
                    StudioButton::CheckAll => self.check_all_agents(),
                    StudioButton::Run => self.run_agent(),
                    StudioButton::Oracle => self.write_agent_request(),
                    StudioButton::Close => self.close_agent_studio(),
                }
                return;
            }
        }
        if layout.name.contains(x, y) {
            self.focus = Focus::AgentName;
            return;
        }
        if layout.owner.contains(x, y) {
            self.cycle_agent_owner();
            return;
        }
        if layout.doctrine.contains(x, y) {
            self.cycle_agent_doctrine();
            return;
        }
        if layout.list.contains(x, y) {
            let index = ((y - layout.list.y) / 18.0).floor() as usize;
            if let Some(agent) = self.map.agents.get(index) {
                let id = agent.id;
                self.select_agent(id);
            }
            return;
        }
        if layout.code.contains(x, y) {
            let Some(edit) = self.agent_edit.as_mut() else {
                return;
            };
            let line = (((y - layout.code.y - 2.0) / 12.0).floor().max(0.0) as usize) + edit.scroll;
            let ranges = edit.line_ranges();
            let line = line.min(ranges.len() - 1);
            let column = (((x - layout.code.x - 30.0) / 5.0).floor().max(0.0) as usize)
                .min(ranges[line].1 - ranges[line].0);
            edit.cursor = ranges[line].0 + column;
            edit.ensure_visible(18);
            self.focus = Focus::AgentScript;
        }
    }

    fn draw_agent_studio(&self, renderer: &mut dyn Renderer) {
        let layout = self.studio_layout();
        renderer.draw_rect(
            layout.x,
            layout.y,
            layout.w,
            layout.h,
            [0.07, 0.08, 0.11, 1.0],
        );
        renderer.draw_rect(layout.x, layout.y, layout.w, 1.0, [0.30, 0.42, 0.55, 1.0]);
        text(
            renderer,
            "AI AGENT STUDIO",
            layout.x + 10.0,
            layout.y + 14.0,
            [0.62, 0.82, 1.0, 1.0],
        );
        text(
            renderer,
            "Esc close  Ctrl+S save",
            layout.x + 130.0,
            layout.y + 14.0,
            [0.48, 0.56, 0.66, 1.0],
        );
        let labels = ["New", "Dup", "Del", "Check", "All", "Run", "LLM", "Close"];
        for (index, (_, rect)) in layout.buttons.iter().enumerate() {
            draw_button(renderer, *rect, labels[index], false);
        }

        text(
            renderer,
            "Name",
            layout.x + 10.0,
            layout.y + 46.0,
            [0.62, 0.72, 0.84, 1.0],
        );
        let name = self
            .agent_edit
            .as_ref()
            .map(|edit| edit.name.as_str())
            .unwrap_or("(no agent)");
        let name_rect = layout.name;
        renderer.draw_rect(
            name_rect.x,
            name_rect.y,
            name_rect.w,
            name_rect.h,
            [0.05, 0.06, 0.09, 1.0],
        );
        if self.focus == Focus::AgentName {
            renderer.draw_rect(
                name_rect.x,
                name_rect.y,
                name_rect.w,
                1.0,
                [0.55, 0.80, 1.0, 1.0],
            );
            renderer.draw_rect(
                name_rect.x,
                name_rect.y + name_rect.h - 1.0,
                name_rect.w,
                1.0,
                [0.55, 0.80, 1.0, 1.0],
            );
        }
        fit_text(
            renderer,
            name,
            name_rect.x + 4.0,
            name_rect.y + 7.0,
            name_rect.w,
            [0.88, 0.92, 0.98, 1.0],
        );

        let owner = self
            .agent_edit
            .as_ref()
            .map(|edit| format!("{:?}", edit.owner))
            .unwrap_or_default();
        draw_button(renderer, layout.owner, &owner, false);
        let doctrine = self
            .agent_edit
            .as_ref()
            .map(|edit| edit.doctrine.clone())
            .unwrap_or_default();
        draw_button(renderer, layout.doctrine, &doctrine, false);

        // Agent list.
        renderer.draw_rect(
            layout.list.x,
            layout.list.y,
            layout.list.w,
            layout.list.h,
            [0.05, 0.06, 0.09, 1.0],
        );
        let row_height = 18.0;
        let visible = (layout.list.h / row_height).floor().max(1.0) as usize;
        for (index, agent) in self.map.agents.iter().take(visible).enumerate() {
            let row = RectF::new(
                layout.list.x + 1.0,
                layout.list.y + 1.0 + index as f32 * row_height,
                layout.list.w - 2.0,
                row_height - 1.0,
            );
            let selected = self.selected_agent == Some(agent.id);
            let failing = self.agent_bad.contains(&agent.id);
            let fill = if selected {
                [0.20, 0.32, 0.46, 1.0]
            } else if failing {
                [0.30, 0.12, 0.12, 1.0]
            } else {
                [0.09, 0.11, 0.15, 1.0]
            };
            renderer.draw_rect(row.x, row.y, row.w, row.h, fill);
            fit_text(
                renderer,
                &format!("#{} {}", agent.id, agent.name),
                row.x + 4.0,
                row.y + 5.0,
                (row.w - 8.0).max(0.0),
                [0.85, 0.90, 0.97, 1.0],
            );
        }

        // Script editor.
        renderer.draw_rect(
            layout.code.x,
            layout.code.y,
            layout.code.w,
            layout.code.h,
            [0.04, 0.05, 0.07, 1.0],
        );
        let Some(edit) = self.agent_edit.as_ref() else {
            text(
                renderer,
                "no agent selected - press New to create one",
                layout.code.x + 8.0,
                layout.code.y + 8.0,
                [0.55, 0.62, 0.72, 1.0],
            );
            self.draw_agent_report(renderer, &layout);
            return;
        };
        let error_lines: std::collections::BTreeSet<usize> = compile(&edit.script)
            .err()
            .map(|diagnostics| {
                diagnostics
                    .into_iter()
                    .map(|diagnostic| diagnostic.line)
                    .collect()
            })
            .unwrap_or_default();
        let ranges = edit.line_ranges();
        let line_height = 12.0;
        let visible_lines = ((layout.code.h - 6.0) / line_height).floor().max(1.0) as usize;
        for (offset, line_index) in (edit.scroll..ranges.len()).take(visible_lines).enumerate() {
            let (start, end) = ranges[line_index];
            let source: String = edit.script.chars().skip(start).take(end - start).collect();
            let y = layout.code.y + 3.0 + offset as f32 * line_height;
            let number_color = if error_lines.contains(&(line_index + 1)) {
                [1.0, 0.42, 0.42, 1.0]
            } else {
                [0.40, 0.48, 0.58, 1.0]
            };
            text(
                renderer,
                &format!("{:>3}", line_index + 1),
                layout.code.x + 4.0,
                y,
                number_color,
            );
            let color = if source.trim_start().starts_with('#') {
                [0.45, 0.62, 0.45, 1.0]
            } else {
                [0.86, 0.90, 0.96, 1.0]
            };
            fit_text(
                renderer,
                &source,
                layout.code.x + 30.0,
                y,
                (layout.code.w - 34.0).max(0.0),
                color,
            );
        }
        if self.focus == Focus::AgentScript {
            let line = edit.cursor_line();
            let (start, _) = ranges[line];
            let column = edit.cursor.saturating_sub(start);
            if line >= edit.scroll && line < edit.scroll + visible_lines {
                let caret_y = layout.code.y + 2.0 + (line - edit.scroll) as f32 * line_height;
                let caret_x = layout.code.x + 30.0 + column as f32 * 5.0;
                renderer.draw_rect(caret_x, caret_y, 1.0, 10.0, [0.95, 0.98, 1.0, 1.0]);
            }
        }

        self.draw_agent_report(renderer, &layout);
    }

    fn draw_agent_report(&self, renderer: &mut dyn Renderer, layout: &StudioLayout) {
        renderer.draw_rect(
            layout.report.x,
            layout.report.y,
            layout.report.w,
            layout.report.h,
            [0.05, 0.06, 0.09, 1.0],
        );
        fit_text(
            renderer,
            &self.agent_message,
            layout.report.x + 4.0,
            layout.report.y + 4.0,
            (layout.report.w - 8.0).max(0.0),
            [0.60, 0.78, 0.95, 1.0],
        );
        for (index, line) in self.agent_report.iter().take(6).enumerate() {
            let color = if line.starts_with("error") {
                [1.0, 0.55, 0.50, 1.0]
            } else {
                [0.72, 0.78, 0.86, 1.0]
            };
            fit_text(
                renderer,
                line,
                layout.report.x + 4.0,
                layout.report.y + 18.0 + index as f32 * 12.0,
                (layout.report.w - 8.0).max(0.0),
                color,
            );
        }
    }

    fn add_trigger(&mut self) {
        self.commit_trigger_edit();
        let (name, condition, action) = self.current_trigger_template();
        let command = crate::history::AddTrigger {
            trigger: Trigger {
                id: 0,
                name,
                condition,
                action,
                group: String::new(),
            },
        };
        self.editor
            .history
            .execute(&mut self.map, Box::new(command));
        let id = self.map.triggers.last().map(|trigger| trigger.id);
        if let Some(id) = id {
            self.selected_trigger = Some(id);
            self.refresh_trigger_edit();
            self.status = format!("added trigger {id}; edit the fields below");
        }
    }

    fn delete_selected_trigger(&mut self) {
        self.commit_trigger_edit();
        let id = self
            .selected_trigger
            .or_else(|| self.map.triggers.last().map(|trigger| trigger.id));
        let Some(id) = id else {
            self.status = "no triggers to delete".to_string();
            return;
        };
        if let Some(command) = RemoveTrigger::new(&self.map, id) {
            self.editor
                .history
                .execute(&mut self.map, Box::new(command));
            self.selected_trigger = None;
            self.trigger_edit = None;
            self.refresh_trigger_edit();
            self.status = format!("deleted trigger {id}");
        }
    }

    fn set_select_tool(&mut self) {
        self.selected_object_tool = None;
        self.editor.set_tool(Box::new(crate::tools::SelectTool));
        self.select_mode = true;
        self.status = "tool: select / drag objects".to_string();
    }

    pub fn undo(&mut self) {
        if self.editor.undo(&mut self.map) {
            self.status = self.editor.status.clone();
            self.refresh_trigger_edit();
            if self.agent_studio {
                self.refresh_agent_edit();
            }
        }
    }

    pub fn redo(&mut self) {
        if self.editor.redo(&mut self.map) {
            self.status = self.editor.status.clone();
            self.refresh_trigger_edit();
            if self.agent_studio {
                self.refresh_agent_edit();
            }
        }
    }

    fn zoom_at(&mut self, x: f32, y: f32, factor: f32) {
        let before = self.cell_at(x, y);
        let area = self.map_area();
        let scale_before = TILE_SIZE * self.zoom;
        let local_x = x - area.x - self.pan_x;
        let local_y = y - area.y - self.pan_y;
        self.zoom = (self.zoom * factor).clamp(0.05, 8.0);
        let scale_after = TILE_SIZE * self.zoom;
        // Keep the world point under the cursor pinned.
        let world_x = local_x / scale_before;
        let world_y = local_y / scale_before;
        self.pan_x = x - area.x - world_x * scale_after;
        self.pan_y = y - area.y - world_y * scale_after;
        let _ = before;
    }

    pub fn scroll(&mut self, delta: f32) {
        let Some((x, y)) = self.mouse else { return };
        if x < self.width - RIGHT_W && y > TOP_H && y < self.height - STATUS_H {
            let factor = if delta > 0.0 { 1.12 } else { 0.89 };
            self.zoom_at(x, y, factor);
        }
    }

    fn activate(&mut self, action: Action) {
        self.focus = match action {
            Action::FocusSeed => Focus::Seed,
            Action::FocusPath => Focus::Path,
            _ => self.focus,
        };
        match action {
            Action::New => self.new_map(),
            Action::OpenAgentStudio => self.open_agent_studio(),
            Action::Generate => self.apply_generate(),
            Action::Open => self.open_with_dialog(),
            Action::ImportOriginalMap => self.import_original_with_dialog(),
            Action::Save => self.save_with_dialog(),
            Action::Undo => self.undo(),
            Action::Redo => self.redo(),
            Action::ToggleGrid => self.show_grid = !self.show_grid,
            Action::ZoomIn => {
                let (x, y) = self.mouse.unwrap_or((self.width * 0.5, self.height * 0.5));
                self.zoom_at(x, y, 1.2);
            }
            Action::ZoomOut => {
                let (x, y) = self.mouse.unwrap_or((self.width * 0.5, self.height * 0.5));
                self.zoom_at(x, y, 1.0 / 1.2);
            }
            Action::SizeDecrease => {
                let index = SIZES
                    .iter()
                    .position(|size| *size == self.map_width && *size == self.map_height)
                    .unwrap_or(2);
                self.set_dimensions(
                    SIZES[index.saturating_sub(1)],
                    SIZES[index.saturating_sub(1)],
                );
            }
            Action::SizeIncrease => {
                let index = SIZES
                    .iter()
                    .position(|size| *size == self.map_width && *size == self.map_height)
                    .unwrap_or(2);
                let size = SIZES[(index + 1).min(SIZES.len() - 1)];
                self.set_dimensions(size, size);
            }
            Action::PlayersDecrease => self.players = self.players.saturating_sub(1).max(1),
            Action::PlayersIncrease => self.players = (self.players + 1).min(8),
            Action::TheaterNext => {
                let index = THEATERS
                    .iter()
                    .position(|theater| *theater == self.theater)
                    .unwrap_or(0);
                self.theater = THEATERS[(index + 1) % THEATERS.len()];
            }
            Action::TogglePlugin => {
                self.plugin_index ^= 1;
                self.trigger_template_index = 0;
                self.status = format!("editor rules: {}", self.plugin_name());
            }
            Action::SelectTool => self.set_select_tool(),
            Action::EraseTool => {
                self.selected_object_tool = None;
                self.select_mode = false;
                self.editor.set_tool(Box::new(EraseTool));
                self.status = "tool: erase".to_string();
            }
            Action::TriggerTool => {
                self.selected_object_tool = None;
                self.select_mode = false;
                let (name, condition, action) = self.current_trigger_template();
                self.editor
                    .set_tool(Box::new(TriggerTool::new(name, condition, action)));
                self.status = "tool: trigger".to_string();
            }
            Action::AddTrigger => self.add_trigger(),
            Action::CommitTrigger => {
                if !self.commit_trigger_edit() {
                    self.status = "trigger fields unchanged".to_string();
                }
            }
            Action::NextTriggerTemplate => self.next_trigger_template(),
            Action::InspectorObjects => self.inspector_tab = InspectorTab::Objects,
            Action::InspectorTriggers => self.inspector_tab = InspectorTab::Triggers,
            Action::InspectorHistory => self.inspector_tab = InspectorTab::History,
            Action::InspectorValidate => self.inspector_tab = InspectorTab::Validate,
            Action::InspectorFiles => {
                self.inspector_tab = InspectorTab::Files;
                self.load_scenario_files();
            }
            Action::NextObjectCategory => {
                self.object_category = self.object_category.next();
                self.status = format!("object palette: {}", self.object_category.label());
            }
            Action::NextPlacedOwnerFilter => self.next_placed_filter(),
            Action::RefreshScenarioFiles => self.load_scenario_files(),
            Action::CopyObject => self.copy_selected_object(),
            Action::PasteObject => self.paste_object(),
            Action::ObjectOwnerNext => self.cycle_object_owner(),
            Action::ObjectNudge(dx, dy) => self.nudge_selected_object(dx, dy),
            Action::DeleteObject => self.delete_selected_objects(),
            Action::DeleteTrigger => self.delete_selected_trigger(),
            Action::FocusSeed | Action::FocusPath => {}
            Action::Terrain(id) => {
                self.brush_terrain = id;
                self.overlay_mode = false;
                self.set_terrain_tool();
                let name = TerrainType::from_id(id).map_or("?", TerrainType::label);
                self.status = format!("brush: {name}");
            }
            Action::Overlay(id) => {
                self.brush_terrain = id;
                self.overlay_mode = true;
                self.set_terrain_tool();
                let name = crate::generator::OverlayType::from_id(id)
                    .map_or("?", crate::generator::OverlayType::label);
                self.status = format!("brush: overlay {name}");
            }
        }
    }

    fn controls(&self) -> Vec<Control> {
        let mut controls = Vec::new();
        let mut push = |x: f32, y: f32, w: f32, h: f32, kind: ControlKind| {
            controls.push(Control {
                rect: RectF::new(x, y, w, h),
                kind,
            });
        };

        // Top toolbar. Button widths keep every label legible at the full
        // 8px font (chars * 10px + 6px padding) — smaller fit_size steps
        // render as sub-pixel quads the software backend cannot draw.
        let mut x = 8.0;
        for (label, action, width) in [
            ("New", Action::New, 44.0),
            ("Generate", Action::Generate, 86.0),
            ("Open", Action::Open, 46.0),
            ("Import", Action::ImportOriginalMap, 66.0),
            ("Save", Action::Save, 46.0),
            ("Undo", Action::Undo, 46.0),
            ("Redo", Action::Redo, 46.0),
            ("Grid", Action::ToggleGrid, 46.0),
            ("AI", Action::OpenAgentStudio, 30.0),
        ] {
            let _ = label;
            push(x, 6.0, width, 24.0, ControlKind::Button(action));
            x += width + 4.0;
        }
        push(x, 6.0, 26.0, 24.0, ControlKind::Button(Action::ZoomOut));
        push(
            x + 30.0,
            6.0,
            26.0,
            24.0,
            ControlKind::Button(Action::ZoomIn),
        );
        // The floor keeps the game button clear of the widened left toolbar.
        push(
            (self.width - RIGHT_W - 122.0).max(560.0),
            6.0,
            114.0,
            24.0,
            ControlKind::Button(Action::TogglePlugin),
        );

        // Right panel.
        let px = self.width - RIGHT_W + 10.0;
        let pw = RIGHT_W - 20.0;
        let mut y = TOP_H + 24.0;
        push(
            px + 40.0,
            y,
            pw - 40.0,
            22.0,
            ControlKind::Button(Action::FocusSeed),
        );
        y += 26.0;
        push(
            px + 40.0,
            y,
            26.0,
            22.0,
            ControlKind::Button(Action::SizeDecrease),
        );
        push(
            px + 72.0,
            y,
            26.0,
            22.0,
            ControlKind::Button(Action::SizeIncrease),
        );
        push(
            px + 104.0,
            y,
            26.0,
            22.0,
            ControlKind::Button(Action::PlayersDecrease),
        );
        push(
            px + 136.0,
            y,
            26.0,
            22.0,
            ControlKind::Button(Action::PlayersIncrease),
        );
        y += 26.0;
        // W/H fields. The second field starts 10px further right so the "H"
        // caption fits between the two fields instead of overlapping the
        // width field (captions drawn in draw_panel).
        push(
            px + 40.0,
            y,
            90.0,
            22.0,
            ControlKind::Field(Focus::MapWidth),
        );
        push(
            px + 150.0,
            y,
            90.0,
            22.0,
            ControlKind::Field(Focus::MapHeight),
        );
        y += 26.0;
        push(px, y, pw, 22.0, ControlKind::Button(Action::TheaterNext));
        y += 26.0;
        push(px, y, pw, 26.0, ControlKind::Button(Action::Generate));
        y += 32.0;

        // Terrain palette: three columns, dynamic row advance so the
        // extended terrain list grows the palette without overflow math.
        for (index, terrain) in TerrainType::ALL.iter().enumerate() {
            let column = index % 3;
            let row = index / 3;
            push(
                px + column as f32 * (pw / 3.0 + 1.0),
                y + row as f32 * 20.0,
                pw / 3.0 - 2.0,
                19.0,
                ControlKind::Terrain(terrain.id()),
            );
        }
        let terrain_rows = TerrainType::ALL.len().div_ceil(3);
        y += terrain_rows as f32 * 20.0 + 3.0;

        // Overlay palette (two compact rows of three chips): None, Tiberium,
        // Trees, Ore and the additive Road/Wall brushes. Three columns so
        // "Tiberium" fits at the full 8px font (chars * 10px + 18px swatch
        // column). Overlay clicks flip the paint palette into overlay mode;
        // terrain clicks flip it back.
        for (index, overlay) in crate::generator::OverlayType::ALL.iter().enumerate() {
            let column = index % 3;
            let row = index / 3;
            push(
                px + column as f32 * (pw / 3.0 + 1.0),
                y + row as f32 * 17.0,
                pw / 3.0 - 2.0,
                17.0,
                ControlKind::Overlay(overlay.id()),
            );
        }
        y += 36.0; // two chip rows (17px pitch) + gap

        y += 16.0; // "OBJECTS" header
                   // Category button sized for "Cat: Buildings" at the full 8px font
                   // (chars * 10px + 6px padding); search field takes the remainder.
        let search_w = pw - 148.0;
        push(
            px,
            y,
            search_w,
            18.0,
            ControlKind::Field(Focus::ObjectSearch),
        );
        push(
            px + search_w + 4.0,
            y,
            144.0,
            18.0,
            ControlKind::Button(Action::NextObjectCategory),
        );
        y += 24.0;
        let objects = self.visible_objects();
        for (index, class_id) in objects.iter().enumerate() {
            let column = index % 2;
            let row = index / 2;
            push(
                px + column as f32 * (pw * 0.5 + 2.0),
                y + row as f32 * 19.0,
                pw * 0.5 - 3.0,
                17.0,
                ControlKind::Object(*class_id),
            );
        }
        let rows = objects.len().div_ceil(2) as f32;
        y += rows * 19.0 + 8.0;

        // Tools row: select/move, erase, place trigger, delete trigger/object.
        // Widths are per-label so every caption renders at the full 8px font
        // ("Del Trg"/"Del Obj" include a half-width space); "Trigger" uses the
        // tab's existing two-letter abbreviation to keep the row inside pw.
        let tool_labels: [(f32, ControlKind); 5] = [
            (66.0, ControlKind::Button(Action::SelectTool)), // "Select"
            (56.0, ControlKind::Button(Action::EraseTool)),  // "Erase"
            (37.0, ControlKind::Button(Action::TriggerTool)), // "Trg"
            (71.0, ControlKind::Button(Action::DeleteTrigger)), // "Del Trg"
            (72.0, ControlKind::Button(Action::DeleteObject)), // "Del Obj"
        ];
        let mut tool_x = px;
        for (width, kind) in tool_labels {
            push(tool_x, y, width, 22.0, kind);
            tool_x += width + 2.0;
        }
        y += 26.0;
        y += 16.0; // "FILE PATH" header band

        // File path field (still works for typed paths and keyboard shortcuts).
        push(px, y, pw, 22.0, ControlKind::Field(Focus::Path));
        y += 26.0;

        // Inspector tabs (objects / triggers / undo history).
        let half = pw * 0.5 - 2.0;
        let tab = pw / 5.0 - 2.0;
        push(
            px,
            y,
            tab,
            20.0,
            ControlKind::Button(Action::InspectorObjects),
        );
        push(
            px + pw / 5.0,
            y,
            tab,
            20.0,
            ControlKind::Button(Action::InspectorTriggers),
        );
        push(
            px + 2.0 * pw / 5.0,
            y,
            tab,
            20.0,
            ControlKind::Button(Action::InspectorHistory),
        );
        push(
            px + 3.0 * pw / 5.0,
            y,
            tab,
            20.0,
            ControlKind::Button(Action::InspectorValidate),
        );
        push(
            px + 4.0 * pw / 5.0,
            y,
            tab,
            20.0,
            ControlKind::Button(Action::InspectorFiles),
        );
        y += 24.0;
        y += 12.0; // section heading band

        match self.inspector_tab {
            InspectorTab::Objects => {
                push(px, y, pw, 18.0, ControlKind::Field(Focus::PlacedSearch));
                y += 24.0;
                for id in self.object_ids().into_iter().take(4) {
                    push(px, y, pw, 16.0, ControlKind::PlacedObject(id));
                    y += 18.0;
                }
                y += 4.0;
                // Copy/Paste sized to their labels; the owner-filter button is
                // flush right and wide enough for "List: Player" at 8px.
                push(px, y, 46.0, 20.0, ControlKind::Button(Action::CopyObject));
                push(
                    px + 48.0,
                    y,
                    56.0,
                    20.0,
                    ControlKind::Button(Action::PasteObject),
                );
                push(
                    px + pw - 208.0,
                    y,
                    208.0,
                    20.0,
                    ControlKind::Button(Action::NextPlacedOwnerFilter),
                );
                y += 26.0;
                push(
                    px,
                    y,
                    pw,
                    20.0,
                    ControlKind::Button(Action::ObjectOwnerNext),
                );
                y += 24.0;
                let quarter = pw * 0.25 - 2.0;
                // Keep the nudge row inside the window floor.
                let y = y.min(self.height - 20.0);
                push(
                    px,
                    y,
                    quarter,
                    20.0,
                    ControlKind::Button(Action::ObjectNudge(-1, 0)),
                );
                push(
                    px + pw * 0.25,
                    y,
                    quarter,
                    20.0,
                    ControlKind::Button(Action::ObjectNudge(0, -1)),
                );
                push(
                    px + pw * 0.5,
                    y,
                    quarter,
                    20.0,
                    ControlKind::Button(Action::ObjectNudge(0, 1)),
                );
                push(
                    px + pw * 0.75,
                    y,
                    quarter,
                    20.0,
                    ControlKind::Button(Action::ObjectNudge(1, 0)),
                );
            }
            InspectorTab::History => {
                push(px, y, half, 20.0, ControlKind::Button(Action::Undo));
                push(
                    px + pw * 0.5 + 2.0,
                    y,
                    half,
                    20.0,
                    ControlKind::Button(Action::Redo),
                );
            }
            InspectorTab::Validate => {}
            InspectorTab::Files => {
                push(
                    px,
                    y,
                    pw,
                    20.0,
                    ControlKind::Button(Action::RefreshScenarioFiles),
                );
                y += 24.0;
                for index in 0..self.scenario_files.len().min(5) {
                    push(px, y, pw, 16.0, ControlKind::ScenarioFile(index));
                    y += 18.0;
                }
            }
            InspectorTab::Triggers => {
                let template_w = pw - 128.0;
                push(
                    px,
                    y,
                    template_w,
                    20.0,
                    ControlKind::Button(Action::NextTriggerTemplate),
                );
                push(
                    px + template_w + 4.0,
                    y,
                    60.0,
                    20.0,
                    ControlKind::Button(Action::AddTrigger),
                );
                push(
                    px + template_w + 68.0,
                    y,
                    60.0,
                    20.0,
                    ControlKind::Button(Action::CommitTrigger),
                );
                y += 22.0;
                push(px, y, pw, 18.0, ControlKind::Field(Focus::TriggerFilter));
                y += 22.0;
                push(
                    px + 46.0,
                    y,
                    pw - 46.0,
                    20.0,
                    ControlKind::Field(Focus::TriggerName),
                );
                y += 22.0;
                push(
                    px + 46.0,
                    y,
                    pw - 46.0,
                    19.0,
                    ControlKind::Field(Focus::TriggerCondition),
                );
                y += 22.0;
                push(
                    px + 46.0,
                    y,
                    pw - 46.0,
                    19.0,
                    ControlKind::Field(Focus::TriggerAction),
                );
                y += 22.0;
                push(
                    px + 46.0,
                    y,
                    pw - 46.0,
                    19.0,
                    ControlKind::Field(Focus::TriggerGroup),
                );
                y += 22.0;
                // Filtered trigger folder rows; stop before the window floor.
                for id in self.visible_trigger_ids() {
                    if y + 18.0 > self.height {
                        break;
                    }
                    push(px, y, pw, 18.0, ControlKind::Trigger(id));
                    y += 20.0;
                }
            }
        }

        controls
    }

    fn hit_control(&self, x: f32, y: f32) -> Option<Control> {
        self.controls()
            .into_iter()
            .find(|control| control.rect.contains(x, y))
    }

    fn pointer_down_map(&mut self, x: f32, y: f32, button: PointerButton) {
        self.mouse = Some((x, y));
        match button {
            PointerButton::Right | PointerButton::Middle => {
                self.panning = true;
                self.last_pointer = (x, y);
            }
            PointerButton::Left => {
                let editing_trigger = matches!(
                    self.focus,
                    Focus::TriggerName
                        | Focus::TriggerCondition
                        | Focus::TriggerAction
                        | Focus::TriggerGroup
                );
                if let Some(control) = self.hit_control(x, y) {
                    if editing_trigger {
                        self.commit_trigger_edit();
                    }
                    match control.kind {
                        ControlKind::Button(action) => self.activate(action),
                        ControlKind::Field(focus) => {
                            if matches!(
                                focus,
                                Focus::TriggerName | Focus::TriggerCondition | Focus::TriggerAction
                            ) {
                                if self.trigger_edit.is_none() {
                                    if let Some(id) = self.trigger_ids().first().copied() {
                                        self.select_trigger(id);
                                    }
                                }
                                self.focus = focus;
                            } else {
                                self.focus = focus;
                            }
                        }
                        ControlKind::Terrain(id) => self.activate(Action::Terrain(id)),
                        ControlKind::Overlay(id) => self.activate(Action::Overlay(id)),
                        ControlKind::Object(class_id) => {
                            if self.shift_held && !self.editor.selected_objects.is_empty() {
                                self.replace_selected_class(class_id);
                            } else {
                                self.set_object_tool(class_id);
                                self.focus = Focus::None;
                                self.status = format!("tool: place {class_id:?}");
                            }
                        }
                        ControlKind::Trigger(id) => self.select_trigger(id),
                        ControlKind::ScenarioFile(index) => {
                            if let Some(path) = self.scenario_files.get(index).cloned() {
                                self.path = path.display().to_string();
                                self.open();
                            }
                        }
                        ControlKind::PlacedObject(id) => {
                            self.editor.selected_object = Some(id);
                            self.editor.selected_objects = vec![id];
                            if let Some(object) = self.map.objects.get(&id) {
                                self.status = format!(
                                    "selected object #{} {} at ({},{})",
                                    object.id,
                                    object.class_name(),
                                    object.position.x,
                                    object.position.y
                                );
                            }
                        }
                    }
                    return;
                }
                if editing_trigger {
                    self.commit_trigger_edit();
                }
                self.focus = Focus::None;
                if self.in_map_area(x, y) {
                    if let Some(cell) = self.minimap_cell_at(x, y) {
                        self.jump_to_cell(cell);
                        return;
                    }
                    let cell = self.cell_at(x, y);
                    if self.select_mode {
                        self.pointer_select(cell);
                        return;
                    }
                    self.painting = true;
                    self.editor.history.begin_group("Editor stroke".to_string());
                    self.last_cell = Some(cell);
                    let triggers_before = self.map.triggers.len();
                    self.editor.click(&mut self.map, cell);
                    self.status = self.editor.status.clone();
                    if self.map.triggers.len() > triggers_before {
                        // Placing with the trigger tool selects the new trigger
                        // so its condition/action can be edited immediately.
                        if let Some(id) = self.map.triggers.last().map(|trigger| trigger.id) {
                            self.select_trigger(id);
                        }
                    }
                }
            }
        }
    }

    /// Begin dragging the object under the cell (select mode).
    fn pointer_select(&mut self, cell: Vec2i) {
        if !self.map.in_bounds(cell) {
            return;
        }
        match self.map.object_at(cell).map(|object| object.id) {
            Some(id) => {
                self.editor.selected_object = Some(id);
                self.editor.selected_objects = vec![id];
                self.dragging_object = Some(id);
                self.marquee_active = false;
                self.last_cell = Some(cell);
                self.editor.history.begin_group("Move object".to_string());
                self.status = format!("dragging object {id}");
            }
            None => {
                self.editor.selected_object = None;
                self.editor.selected_objects.clear();
                self.dragging_object = None;
                self.marquee_start = Some(cell);
                self.marquee_end = Some(cell);
                self.marquee_active = true;
                self.status = "drag to marquee-select objects".to_string();
            }
        }
    }

    pub fn pointer_move(&mut self, x: f32, y: f32) {
        self.mouse = Some((x, y));
        if self.panning {
            self.pan_x += x - self.last_pointer.0;
            self.pan_y += y - self.last_pointer.1;
            self.last_pointer = (x, y);
            return;
        }
        if self.in_map_area(x, y) {
            self.cursor_cell = self.cell_at(x, y);
        }
        if self.inspector_tab == InspectorTab::Files {
            self.scenario_hover = self
                .hit_control(x, y)
                .and_then(|control| match control.kind {
                    ControlKind::ScenarioFile(index) => Some(index),
                    _ => None,
                });
        }
        if self.marquee_active {
            self.marquee_end = Some(self.cell_at(x, y));
            return;
        }
        if let Some(id) = self.dragging_object {
            let cell = self.cell_at(x, y);
            if !self.map.in_bounds(cell) || Some(cell) == self.last_cell {
                return;
            }
            if self
                .map
                .object_at(cell)
                .is_some_and(|object| object.id != id)
            {
                return;
            }
            let from = match self.map.objects.get(&id) {
                Some(object) => object.position,
                None => {
                    self.dragging_object = None;
                    return;
                }
            };
            if from == cell {
                self.last_cell = Some(cell);
                return;
            }
            let command = crate::history::MoveObject { id, from, to: cell };
            self.editor
                .history
                .execute(&mut self.map, Box::new(command));
            self.last_cell = Some(cell);
            self.status = format!("object {id} -> ({},{})", cell.x, cell.y);
            return;
        }
        if self.painting {
            let cell = self.cell_at(x, y);
            if !self.map.in_bounds(cell) {
                return;
            }
            if Some(cell) != self.last_cell {
                let from = self.last_cell.unwrap_or(cell);
                if self.editor.drag(&mut self.map, from, cell) {
                    self.status = self.editor.status.clone();
                }
                self.last_cell = Some(cell);
            }
        }
    }

    pub fn pointer_up(&mut self, _button: PointerButton) {
        self.painting = false;
        self.panning = false;
        self.dragging_object = None;
        self.finish_marquee();
        if self.editor.history.in_group() {
            self.editor.history.end_group();
        }
        if self.status.is_empty() {
            self.status = self.editor.status.clone();
        }
    }

    pub fn key_text(&mut self, text: &str) {
        if self.map_info_open {
            let focus = self.focus;
            let target = match focus {
                Focus::MapName => &mut self.map_info_name,
                Focus::MapPlayer => &mut self.map_info_player,
                Focus::MapBriefing => &mut self.map_info_briefing,
                Focus::MapObjectives => &mut self.map_info_objectives,
                _ => return,
            };
            for ch in text.chars().filter(|ch| !ch.is_control()) {
                if target.chars().count() < 240 {
                    target.push(ch);
                }
            }
            return;
        }
        if self.agent_studio {
            self.agent_text(text);
            return;
        }
        let focus = self.focus;
        let target = match focus {
            Focus::None => return,
            Focus::Seed => &mut self.seed,
            Focus::Path => &mut self.path,
            Focus::MapWidth => &mut self.map_width_text,
            Focus::MapHeight => &mut self.map_height_text,
            Focus::TriggerFilter => &mut self.trigger_filter,
            Focus::ObjectSearch => &mut self.object_search,
            Focus::PlacedSearch => &mut self.placed_search,
            Focus::AgentName
            | Focus::AgentScript
            | Focus::MapName
            | Focus::MapPlayer
            | Focus::MapBriefing
            | Focus::MapObjectives => return,
            Focus::TriggerName
            | Focus::TriggerCondition
            | Focus::TriggerAction
            | Focus::TriggerGroup => {
                let Some(edit) = self.trigger_edit.as_mut() else {
                    return;
                };
                match focus {
                    Focus::TriggerName => &mut edit.name,
                    Focus::TriggerCondition => &mut edit.condition,
                    Focus::TriggerGroup => &mut edit.group,
                    _ => &mut edit.action,
                }
            }
        };
        for ch in text.chars() {
            if ch.is_control() {
                continue;
            }
            if matches!(focus, Focus::Seed | Focus::MapWidth | Focus::MapHeight)
                && !ch.is_ascii_digit()
            {
                continue;
            }
            if target.chars().count() < 120 {
                target.push(ch);
            }
        }
        if matches!(focus, Focus::MapWidth | Focus::MapHeight) {
            self.sync_dimensions();
        }
    }

    pub fn key_backspace(&mut self) {
        if self.map_info_open {
            match self.focus {
                Focus::MapName => {
                    self.map_info_name.pop();
                }
                Focus::MapPlayer => {
                    self.map_info_player.pop();
                }
                Focus::MapBriefing => {
                    self.map_info_briefing.pop();
                }
                Focus::MapObjectives => {
                    self.map_info_objectives.pop();
                }
                _ => {}
            }
            return;
        }
        if self.agent_studio {
            if let Some(edit) = self.agent_edit.as_mut() {
                match self.focus {
                    Focus::AgentScript => {
                        edit.backspace();
                        edit.ensure_visible(18);
                    }
                    Focus::AgentName => {
                        edit.name.pop();
                    }
                    _ => {}
                }
            }
            return;
        }
        let focus = self.focus;
        let target = match focus {
            Focus::None => return,
            Focus::Seed => &mut self.seed,
            Focus::Path => &mut self.path,
            Focus::MapWidth => &mut self.map_width_text,
            Focus::MapHeight => &mut self.map_height_text,
            Focus::TriggerFilter => &mut self.trigger_filter,
            Focus::ObjectSearch => &mut self.object_search,
            Focus::PlacedSearch => &mut self.placed_search,
            Focus::AgentName
            | Focus::AgentScript
            | Focus::MapName
            | Focus::MapPlayer
            | Focus::MapBriefing
            | Focus::MapObjectives => return,
            Focus::TriggerName
            | Focus::TriggerCondition
            | Focus::TriggerAction
            | Focus::TriggerGroup => {
                let Some(edit) = self.trigger_edit.as_mut() else {
                    return;
                };
                match focus {
                    Focus::TriggerName => &mut edit.name,
                    Focus::TriggerCondition => &mut edit.condition,
                    Focus::TriggerGroup => &mut edit.group,
                    _ => &mut edit.action,
                }
            }
        };
        target.pop();
        if matches!(focus, Focus::MapWidth | Focus::MapHeight) {
            self.sync_dimensions();
        }
    }

    pub fn key_enter(&mut self) {
        if self.map_info_open {
            self.focus = match self.focus {
                Focus::MapName => Focus::MapPlayer,
                Focus::MapPlayer => Focus::MapBriefing,
                Focus::MapBriefing => Focus::MapObjectives,
                Focus::MapObjectives => {
                    self.close_map_info(true);
                    return;
                }
                _ => Focus::MapName,
            };
            return;
        }
        if self.agent_studio {
            if let Some(edit) = self.agent_edit.as_mut() {
                if self.focus == Focus::AgentScript {
                    edit.insert_newline();
                    edit.ensure_visible(18);
                }
            }
            if self.focus == Focus::AgentName {
                self.focus = Focus::AgentScript;
            }
            return;
        }
        if matches!(self.focus, Focus::MapWidth | Focus::MapHeight) {
            self.sync_dimensions();
        }
        if self.focus == Focus::Seed {
            self.apply_generate();
        } else if matches!(
            self.focus,
            Focus::TriggerName
                | Focus::TriggerCondition
                | Focus::TriggerAction
                | Focus::TriggerGroup
        ) {
            self.commit_trigger_edit();
        }
        self.focus = Focus::None;
    }

    pub fn key_escape(&mut self) {
        if self.map_info_open {
            self.close_map_info(false);
            return;
        }
        if self.show_help {
            self.show_help = false;
            return;
        }
        if self.agent_studio {
            self.close_agent_studio();
            return;
        }
        if matches!(
            self.focus,
            Focus::TriggerName
                | Focus::TriggerCondition
                | Focus::TriggerAction
                | Focus::TriggerGroup
        ) {
            self.commit_trigger_edit();
        }
        self.focus = Focus::None;
        self.status = "focus cleared".to_string();
    }

    /// Select every placed object (Ctrl+A).
    pub fn select_all_objects(&mut self) {
        let mut ids: Vec<u32> = self.map.objects.keys().copied().collect();
        ids.sort_unstable();
        self.editor.selected_objects = ids;
        self.editor.selected_object = self.editor.selected_objects.first().copied();
        self.status = format!("selected {} object(s)", self.editor.selected_objects.len());
    }

    /// Clear the object selection (Ctrl+Shift+A).
    pub fn clear_object_selection(&mut self) {
        self.editor.selected_objects.clear();
        self.editor.selected_object = None;
        self.status = "selection cleared".to_string();
    }

    /// Delete every selected object (one undo step), or the cursor object.
    pub fn delete_selected_objects(&mut self) {
        if self.agent_studio {
            return;
        }
        let ids = self.editor.selected_objects.clone();
        if ids.is_empty() {
            let cell = self.cursor_cell;
            if self.editor.delete_at(&mut self.map, cell) {
                self.status = self.editor.status.clone();
            } else {
                self.status = "nothing to delete".to_string();
            }
            return;
        }
        self.editor
            .history
            .begin_group(format!("Delete {} objects", ids.len()));
        let mut removed = 0;
        for id in ids {
            if let Some(command) = RemoveObject::new(&self.map, id) {
                self.editor
                    .history
                    .execute(&mut self.map, Box::new(command));
                removed += 1;
            }
        }
        self.editor.history.end_group();
        self.editor.selected_objects.clear();
        self.editor.selected_object = None;
        self.status = format!("deleted {removed} object(s)");
    }

    pub fn delete_at_cursor(&mut self) {
        if self.agent_studio {
            if let Some(edit) = self.agent_edit.as_mut() {
                if self.focus == Focus::AgentScript {
                    edit.delete();
                    edit.ensure_visible(18);
                }
            }
            return;
        }
        self.activate(Action::DeleteObject);
    }

    fn agent_text(&mut self, text: &str) {
        let Some(edit) = self.agent_edit.as_mut() else {
            return;
        };
        match self.focus {
            Focus::AgentName => {
                for character in text.chars().filter(|character| !character.is_control()) {
                    if edit.name.chars().count() < 64 {
                        edit.name.push(character);
                    }
                }
            }
            Focus::AgentScript => {
                edit.insert_text(text);
                edit.ensure_visible(18);
            }
            _ => {}
        }
    }

    pub fn key_left(&mut self) {
        if self.agent_studio && self.focus == Focus::AgentScript {
            if let Some(edit) = self.agent_edit.as_mut() {
                edit.move_horizontal(-1);
                edit.ensure_visible(18);
            }
        }
    }

    pub fn key_right(&mut self) {
        if self.agent_studio && self.focus == Focus::AgentScript {
            if let Some(edit) = self.agent_edit.as_mut() {
                edit.move_horizontal(1);
                edit.ensure_visible(18);
            }
        }
    }

    pub fn key_up(&mut self) {
        if self.agent_studio && self.focus == Focus::AgentScript {
            if let Some(edit) = self.agent_edit.as_mut() {
                edit.move_vertical(-1);
                edit.ensure_visible(18);
            }
        }
    }

    pub fn key_down(&mut self) {
        if self.agent_studio && self.focus == Focus::AgentScript {
            if let Some(edit) = self.agent_edit.as_mut() {
                edit.move_vertical(1);
                edit.ensure_visible(18);
            }
        }
    }

    pub fn key_home(&mut self) {
        if self.agent_studio && self.focus == Focus::AgentScript {
            if let Some(edit) = self.agent_edit.as_mut() {
                edit.move_home();
                edit.ensure_visible(18);
            }
        }
    }

    pub fn key_end(&mut self) {
        if self.agent_studio && self.focus == Focus::AgentScript {
            if let Some(edit) = self.agent_edit.as_mut() {
                edit.move_end();
                edit.ensure_visible(18);
            }
        }
    }

    pub fn toggle_grid(&mut self) {
        self.show_grid = !self.show_grid;
    }

    pub fn summary_line(&self) -> String {
        let cell = self.cursor_cell;
        let terrain = self
            .map
            .terrain_at(cell)
            .and_then(TerrainType::from_id)
            .map_or("?", TerrainType::label);
        let cell_text = if self.map.in_bounds(cell) {
            format!("x {} y {} {}", cell.x, cell.y, terrain)
        } else {
            "out of bounds".to_string()
        };
        let selection = if self.editor.selected_objects.len() > 1 {
            format!(" | selected {} objects", self.editor.selected_objects.len())
        } else {
            self.editor
                .selected_object
                .and_then(|id| self.map.objects.get(&id))
                .map_or(String::new(), |object| {
                    format!(" | selected #{} {}", object.id, object.class_name())
                })
        };
        format!(
            "{} | {} | triggers {} | {} | {}{}",
            self.plugin_name(),
            self.map.summary(),
            self.map.triggers.len(),
            cell_text,
            self.status,
            selection
        )
    }

    /// Draw the entire editor in screen coordinates.
    pub fn render(&mut self, renderer: &mut dyn Renderer, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        let controls = self.controls();
        let hover = self.mouse.and_then(|(x, y)| {
            controls
                .iter()
                .position(|control| control.rect.contains(x, y))
        });

        self.draw_background(renderer, width, height);
        self.draw_map(renderer);
        self.draw_toolbar(renderer, &controls, hover);
        self.draw_panel(renderer, &controls, hover);
        self.draw_status(renderer, width, height);
        if self.show_help {
            self.draw_help(renderer, width, height);
        }
        if self.agent_studio {
            self.draw_agent_studio(renderer);
        }
        if self.map_info_open {
            self.draw_map_info(renderer, width, height);
        }
    }

    fn draw_background(&self, renderer: &mut dyn Renderer, width: f32, height: f32) {
        renderer.draw_rect(0.0, 0.0, width, height, [0.06, 0.07, 0.09, 1.0]);
    }

    fn draw_map(&self, renderer: &mut dyn Renderer) {
        let area = self.map_area();
        let scale = TILE_SIZE * self.zoom;
        if scale <= 0.0 {
            return;
        }
        let start_x = ((-self.pan_x) / scale).floor().max(0.0) as i32;
        let start_y = ((-self.pan_y) / scale).floor().max(0.0) as i32;
        let end_x = (((area.w - self.pan_x) / scale).ceil() as i32 + 1).min(self.map.width as i32);
        let end_y = (((area.h - self.pan_y) / scale).ceil() as i32 + 1).min(self.map.height as i32);
        let max_cells = 200_000usize;
        if end_x <= start_x || end_y <= start_y {
            return;
        }
        let count = ((end_x - start_x) as usize) * ((end_y - start_y) as usize);
        if count > max_cells {
            return;
        }

        // Lazy atlas upload: draw_sprite only samples once the tileset texture
        // slot exists, and it stays out of the way until then.
        let mut draw_tiles = self.tileset.is_some() && renderer.texture_ready(TERRAIN_TILE_TEXTURE);
        if let Some(tileset) = self
            .tileset
            .as_ref()
            .map(|pack| pack.as_ref())
            .filter(|_| !renderer.texture_ready(TERRAIN_TILE_TEXTURE))
        {
            match renderer.set_atlas_texture(
                TERRAIN_TILE_TEXTURE,
                tileset.width(),
                tileset.height(),
                tileset.rgba(),
            ) {
                Ok(()) => draw_tiles = true,
                Err(error) => eprintln!("tileset atlas upload failed: {error}"),
            }
        }

        let pack_ref = draw_tiles.then_some(self.tileset.as_ref()).flatten();
        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = (y as u32 * self.map.width + x as u32) as usize;
                let terrain = self.map.terrain.get(index).copied().unwrap_or(0);
                let overlay = self.map.overlay.get(index).copied().unwrap_or(0);
                let (sx, sy) = self.cell_screen(Vec2i::new(x, y));

                // Base tile: waters/shoreline, resource underlays, cliff
                // bands, blob terrain transitions and auto-connected
                // road/wall tiles replace the plain ground where applicable
                // (the pack adapter decides, priority order preserved).
                let cell = pack_ref.and_then(|pack| pack.cell_draw(&self.map, x, y));
                let base_rect = cell.as_ref().and_then(|tile| {
                    tile.base
                        .as_deref()
                        .and_then(|sprite_id| pack_ref.and_then(|pack| pack.sprite_rect(sprite_id)))
                });
                match base_rect {
                    Some(rect) => {
                        let mut sprite = Sprite::new(
                            TERRAIN_TILE_TEXTURE,
                            Vec2::new(Fixed16_16::from_f32(sx), Fixed16_16::from_f32(sy)),
                            Vec2::new(
                                Fixed16_16::from_f32(scale + 0.5),
                                Fixed16_16::from_f32(scale + 0.5),
                            ),
                        );
                        sprite.uv_rect = rect;
                        renderer.draw_sprite(&sprite);
                    }
                    None => {
                        let color = terrain_color(terrain);
                        renderer.draw_rect(sx, sy, scale + 0.5, scale + 0.5, color);
                    }
                }

                // RGBA sprite layers (trees, resource crystals) alpha-
                // composited over the base tile; the old flat overlay dot
                // stays as fallback for missing pack art.
                if let Some(pack) = pack_ref {
                    let mut drew_sprite = false;
                    if let Some(tile) = cell.as_ref() {
                        for layer in &tile.layers {
                            if let Some(rect) = pack.sprite_rect(layer) {
                                let mut sprite = Sprite::new(
                                    TERRAIN_TILE_TEXTURE,
                                    Vec2::new(Fixed16_16::from_f32(sx), Fixed16_16::from_f32(sy)),
                                    Vec2::new(
                                        Fixed16_16::from_f32(scale + 0.5),
                                        Fixed16_16::from_f32(scale + 0.5),
                                    ),
                                );
                                sprite.uv_rect = rect;
                                renderer.draw_sprite(&sprite);
                                drew_sprite = true;
                            }
                        }
                    }
                    // Road/Wall tiles self-describe; resource/tree overlays
                    // only keep the dot when their sprite art was unavailable.
                    let self_describing = overlay == crate::generator::OverlayType::Road.id()
                        || overlay == crate::generator::OverlayType::Wall.id();
                    if overlay != 0 && !drew_sprite && !self_describing {
                        let dot = (scale * 0.45).max(2.0);
                        let color = overlay_color(overlay);
                        renderer.draw_rect(
                            sx + (scale - dot) * 0.5,
                            sy + (scale - dot) * 0.5,
                            dot,
                            dot,
                            color,
                        );
                    }
                } else if overlay != 0 {
                    let dot = (scale * 0.45).max(2.0);
                    let color = overlay_color(overlay);
                    renderer.draw_rect(
                        sx + (scale - dot) * 0.5,
                        sy + (scale - dot) * 0.5,
                        dot,
                        dot,
                        color,
                    );
                }
            }
        }

        // Grid.
        if self.show_grid && scale >= 9.0 {
            let line = [0.0, 0.0, 0.0, 0.16];
            for x in start_x..=end_x {
                let (sx, _) = self.cell_screen(Vec2i::new(x, start_y));
                renderer.draw_rect(sx, area.y, 1.0, area.h, line);
            }
            for y in start_y..=end_y {
                let (_, sy) = self.cell_screen(Vec2i::new(start_x, y));
                renderer.draw_rect(area.x, sy, area.w, 1.0, line);
            }
        }

        // Player starts.
        for (index, start) in self.map.player_starts.iter().enumerate() {
            let (sx, sy) = self.cell_screen(*start);
            let color = start_color(index);
            renderer.draw_rect(
                sx + scale * 0.2,
                sy + scale * 0.2,
                scale * 0.6,
                scale * 0.6,
                color,
            );
            renderer.draw_rect(sx, sy + scale * 0.45, scale, scale * 0.1, color);
            renderer.draw_rect(sx + scale * 0.45, sy, scale * 0.1, scale, color);
        }

        // Objects, sorted for stable draw order.
        let mut objects: Vec<&MapObject> = self.map.objects.values().collect();
        objects.sort_by_key(|object| object.id);
        for object in objects {
            let (sx, sy) = self.cell_screen(object.position);
            let selected = self.editor.selected_object == Some(object.id)
                || self.editor.selected_objects.contains(&object.id);
            let color = if selected {
                [1.0, 1.0, 1.0, 1.0]
            } else if object.class_id.is_building() {
                [0.95, 0.62, 0.20, 1.0]
            } else {
                [0.30, 0.78, 1.0, 1.0]
            };
            let size = (scale * 0.55).max(3.0);
            renderer.draw_rect(
                sx + (scale - size) * 0.5,
                sy + (scale - size) * 0.5,
                size,
                size,
                color,
            );
            if selected {
                renderer.draw_rect(sx, sy, scale, 1.0, [1.0, 1.0, 1.0, 1.0]);
                renderer.draw_rect(sx, sy + scale - 1.0, scale, 1.0, [1.0, 1.0, 1.0, 1.0]);
                renderer.draw_rect(sx, sy, 1.0, scale, [1.0, 1.0, 1.0, 1.0]);
                renderer.draw_rect(sx + scale - 1.0, sy, 1.0, scale, [1.0, 1.0, 1.0, 1.0]);
            }
        }

        // Marquee selection rectangle.
        if self.marquee_active {
            if let (Some(start), Some(end)) = (self.marquee_start, self.marquee_end) {
                let x0 = start.x.min(end.x);
                let x1 = start.x.max(end.x);
                let y0 = start.y.min(end.y);
                let y1 = start.y.max(end.y);
                let (sx, sy) = self.cell_screen(Vec2i::new(x0, y0));
                let (ex, ey) = self.cell_screen(Vec2i::new(x1 + 1, y1 + 1));
                let width = (ex - sx).max(1.0);
                let height = (ey - sy).max(1.0);
                renderer.draw_rect(sx, sy, width, height, [0.20, 0.80, 1.00, 0.18]);
                let border = [0.55, 0.90, 1.00, 0.9];
                renderer.draw_rect(sx, sy, width, 1.0, border);
                renderer.draw_rect(sx, sy + height - 1.0, width, 1.0, border);
                renderer.draw_rect(sx, sy, 1.0, height, border);
                renderer.draw_rect(sx + width - 1.0, sy, 1.0, height, border);
            }
        }

        // Cursor cell.
        if self.map.in_bounds(self.cursor_cell)
            && self.mouse.is_some_and(|(x, y)| self.in_map_area(x, y))
        {
            let (sx, sy) = self.cell_screen(self.cursor_cell);
            let color = if self.painting {
                [1.0, 0.45, 0.35, 0.95]
            } else {
                [1.0, 0.85, 0.35, 0.95]
            };
            renderer.draw_rect(sx, sy, scale, 1.0, color);
            renderer.draw_rect(sx, sy + scale - 1.0, scale, 1.0, color);
            renderer.draw_rect(sx, sy, 1.0, scale, color);
            renderer.draw_rect(sx + scale - 1.0, sy, 1.0, scale, color);
        }

        self.draw_minimap(renderer);
    }

    fn draw_minimap(&self, renderer: &mut dyn Renderer) {
        let rect = self.minimap_rect();
        renderer.draw_rect(rect.x, rect.y, rect.w, rect.h, [0.03, 0.04, 0.06, 0.92]);

        let step = ((self.map.width.max(self.map.height) as f32 / 80.0).ceil() as u32).max(1);
        let cell_w = rect.w / self.map.width.max(1) as f32;
        let cell_h = rect.h / self.map.height.max(1) as f32;
        let block_w = (cell_w * step as f32).max(1.0);
        let block_h = (cell_h * step as f32).max(1.0);
        let mut y = 0u32;
        while y < self.map.height {
            let mut x = 0u32;
            while x < self.map.width {
                let index = (y as usize) * self.map.width as usize + x as usize;
                let terrain = self.map.terrain.get(index).copied().unwrap_or(0);
                renderer.draw_rect(
                    rect.x + x as f32 * cell_w,
                    rect.y + y as f32 * cell_h,
                    block_w,
                    block_h,
                    terrain_color(terrain),
                );
                if self.map.overlay.get(index).copied().unwrap_or(0) != 0 {
                    renderer.draw_rect(
                        rect.x + x as f32 * cell_w + block_w * 0.3,
                        rect.y + y as f32 * cell_h + block_h * 0.3,
                        (block_w * 0.4).max(1.0),
                        (block_h * 0.4).max(1.0),
                        [0.85, 0.72, 0.22, 1.0],
                    );
                }
                x += step;
            }
            y += step;
        }

        for object in self.map.objects.values() {
            let ox =
                rect.x + (object.position.x as f32 + 0.5) / self.map.width.max(1) as f32 * rect.w;
            let oy =
                rect.y + (object.position.y as f32 + 0.5) / self.map.height.max(1) as f32 * rect.h;
            let color = if object.class_id.is_building() {
                [0.95, 0.62, 0.20, 1.0]
            } else {
                [0.30, 0.78, 1.0, 1.0]
            };
            renderer.draw_rect(ox - 1.0, oy - 1.0, 2.5, 2.5, color);
        }
        for (index, start) in self.map.player_starts.iter().enumerate() {
            let sx = rect.x + (start.x as f32 + 0.5) / self.map.width.max(1) as f32 * rect.w;
            let sy = rect.y + (start.y as f32 + 0.5) / self.map.height.max(1) as f32 * rect.h;
            renderer.draw_rect(sx - 1.5, sy - 1.5, 3.0, 3.0, start_color(index));
        }

        // Visible viewport box.
        let area = self.map_area();
        let scale = TILE_SIZE * self.zoom;
        if scale > 0.0 {
            let world_x0 = (-self.pan_x / scale).clamp(0.0, self.map.width as f32);
            let world_y0 = (-self.pan_y / scale).clamp(0.0, self.map.height as f32);
            let world_x1 = ((area.w - self.pan_x) / scale).clamp(0.0, self.map.width as f32);
            let world_y1 = ((area.h - self.pan_y) / scale).clamp(0.0, self.map.height as f32);
            let vx = rect.x + world_x0 / self.map.width.max(1) as f32 * rect.w;
            let vy = rect.y + world_y0 / self.map.height.max(1) as f32 * rect.h;
            let vw = ((world_x1 - world_x0) / self.map.width.max(1) as f32 * rect.w).max(2.0);
            let vh = ((world_y1 - world_y0) / self.map.height.max(1) as f32 * rect.h).max(2.0);
            renderer.draw_rect(vx, vy, vw, 1.0, [1.0, 1.0, 1.0, 0.7]);
            renderer.draw_rect(vx, vy + vh - 1.0, vw, 1.0, [1.0, 1.0, 1.0, 0.7]);
            renderer.draw_rect(vx, vy, 1.0, vh, [1.0, 1.0, 1.0, 0.7]);
            renderer.draw_rect(vx + vw - 1.0, vy, 1.0, vh, [1.0, 1.0, 1.0, 0.7]);
        }

        let border = [0.35, 0.50, 0.65, 1.0];
        renderer.draw_rect(rect.x, rect.y, rect.w, 1.0, border);
        renderer.draw_rect(rect.x, rect.y + rect.h - 1.0, rect.w, 1.0, border);
        renderer.draw_rect(rect.x, rect.y, 1.0, rect.h, border);
        renderer.draw_rect(rect.x + rect.w - 1.0, rect.y, 1.0, rect.h, border);
    }

    fn draw_toolbar(
        &self,
        renderer: &mut dyn Renderer,
        controls: &[Control],
        hover: Option<usize>,
    ) {
        renderer.draw_rect(0.0, 0.0, self.width, TOP_H, [0.10, 0.12, 0.16, 1.0]);
        renderer.draw_rect(0.0, TOP_H - 1.0, self.width, 1.0, [0.22, 0.30, 0.40, 1.0]);
        let labels = [
            "New", "Generate", "Open", "Import", "Save", "Undo", "Redo", "Grid", "AI", "-", "+",
        ];
        for (index, control) in controls.iter().enumerate() {
            if control.rect.y > TOP_H {
                continue;
            }
            let label = match control.kind {
                ControlKind::Button(Action::TogglePlugin) => {
                    if self.plugin_index == 0 {
                        "Game: TD"
                    } else {
                        "Game: RA"
                    }
                }
                _ => labels.get(index).copied().unwrap_or(""),
            };
            draw_button(renderer, control.rect, label, hover == Some(index));
        }
        // Group separators: file / edit / view / AI / zoom clusters.
        for control in controls.iter().filter(|control| control.rect.y <= TOP_H) {
            if matches!(
                control.kind,
                ControlKind::Button(Action::Generate)
                    | ControlKind::Button(Action::Save)
                    | ControlKind::Button(Action::Redo)
                    | ControlKind::Button(Action::OpenAgentStudio)
            ) {
                renderer.draw_rect(
                    control.rect.x + control.rect.w + 2.0,
                    10.0,
                    1.0,
                    16.0,
                    [0.30, 0.36, 0.45, 1.0],
                );
            }
        }
    }

    fn draw_panel(&self, renderer: &mut dyn Renderer, controls: &[Control], hover: Option<usize>) {
        let panel_x = self.width - RIGHT_W;
        renderer.draw_rect(
            panel_x,
            TOP_H,
            RIGHT_W,
            self.height - TOP_H - STATUS_H,
            [0.09, 0.10, 0.13, 1.0],
        );
        renderer.draw_rect(
            panel_x,
            TOP_H,
            1.0,
            self.height - TOP_H - STATUS_H,
            [0.22, 0.30, 0.40, 1.0],
        );

        let px = panel_x + 10.0;
        // Row cascade mirrors controls() exactly (26px rows, header bands,
        // palette pitch) so captions sit precisely above their rows.
        let terrain_rows = TerrainType::ALL.len().div_ceil(3) as f32;
        let visible_objects = self.visible_objects();
        let object_rows = visible_objects.len().div_ceil(2) as f32;
        let seed_y = TOP_H + 24.0; // seed field row
        let dims_y = seed_y + 26.0; // size/player steppers
        let fields_y = seed_y + 52.0; // W/H fields
        let terrain_y = seed_y + 136.0; // generate row ends at seed_y + 130
        let overlay_y = terrain_y + terrain_rows * 20.0 + 3.0;
        let objects_y = overlay_y + 36.0 + 16.0; // "OBJECTS" header + search row
        let tools_y = objects_y + 24.0 + object_rows * 19.0 + 8.0;
        let path_y = tools_y + 26.0 + 16.0; // "FILE PATH" header band

        text(
            renderer,
            "GENERATOR",
            px,
            TOP_H + 8.0,
            [0.55, 0.70, 0.85, 1.0],
        );
        text(renderer, "Seed", px, seed_y + 7.0, [0.75, 0.82, 0.92, 1.0]);
        // Stepper legend sits right of the four +/- buttons, in the free
        // space of the stepper row (the 26px row pitch leaves no gap above).
        // Two 8px lines fit the 22px band: size readout over player count.
        text(
            renderer,
            &format!("Size {}x{}", self.map_width, self.map_height),
            px + 178.0,
            dims_y + 1.0,
            [0.62, 0.72, 0.84, 1.0],
        );
        text(
            renderer,
            &format!("Players {}", self.players),
            px + 178.0,
            dims_y + 12.0,
            [0.62, 0.72, 0.84, 1.0],
        );
        text(renderer, "W", px, fields_y + 7.0, [0.62, 0.72, 0.84, 1.0]);
        text(
            renderer,
            "H",
            px + 134.0,
            fields_y + 7.0,
            [0.62, 0.72, 0.84, 1.0],
        );
        text(
            renderer,
            "TERRAIN BRUSH",
            px,
            terrain_y - 12.0,
            [0.55, 0.70, 0.85, 1.0],
        );
        text(
            renderer,
            &format!(
                "OBJECTS ({}/{})",
                visible_objects.len(),
                self.plugin().available_objects().len()
            ),
            px,
            objects_y - 12.0,
            [0.55, 0.70, 0.85, 1.0],
        );
        text(
            renderer,
            "TOOLS",
            px,
            tools_y - 10.0,
            [0.55, 0.70, 0.85, 1.0],
        );
        text(
            renderer,
            "FILE PATH (Ctrl+S save / Ctrl+O open)",
            px,
            path_y - 12.0,
            [0.55, 0.70, 0.85, 1.0],
        );
        let content_y = path_y + 26.0 + 24.0 + 12.0;
        match self.inspector_tab {
            InspectorTab::Objects => text(
                renderer,
                "PLACED OBJECTS (Ctrl+C copy / Ctrl+V paste)",
                px,
                content_y - 12.0,
                [0.55, 0.70, 0.85, 1.0],
            ),
            InspectorTab::Triggers => text(
                renderer,
                "TRIGGERS (template, list, fields)",
                px,
                content_y - 12.0,
                [0.55, 0.70, 0.85, 1.0],
            ),
            InspectorTab::History => text(
                renderer,
                "HISTORY (most recent first)",
                px,
                content_y - 12.0,
                [0.55, 0.70, 0.85, 1.0],
            ),
            InspectorTab::Validate => {
                let issues = self.validation_issues().len();
                text(
                    renderer,
                    &format!(
                        "VALIDATION ({} issue{})",
                        issues,
                        if issues == 1 { "" } else { "s" }
                    ),
                    px,
                    content_y - 12.0,
                    if issues == 0 {
                        [0.55, 0.82, 0.62, 1.0]
                    } else {
                        [0.92, 0.72, 0.45, 1.0]
                    },
                );
            }
            InspectorTab::Files => text(
                renderer,
                "SCENARIOS (maps/*.ini)",
                px,
                content_y - 12.0,
                [0.55, 0.70, 0.85, 1.0],
            ),
        }

        // Live player count next to the stepper caption (the steppers have no
        // value display; the status bar carries the full map summary).
        text(
            renderer,
            &format!("Players {}", self.players),
            px + 222.0,
            dims_y + 7.0,
            [0.62, 0.72, 0.84, 1.0],
        );

        for (index, control) in controls.iter().enumerate() {
            if control.rect.y < TOP_H {
                continue;
            }
            let hovered = hover == Some(index);
            match control.kind {
                ControlKind::Button(Action::FocusSeed) => {
                    draw_field(
                        renderer,
                        control.rect,
                        &self.seed,
                        self.focus == Focus::Seed,
                        hovered,
                    );
                }
                ControlKind::Button(Action::SizeDecrease)
                | ControlKind::Button(Action::PlayersDecrease) => {
                    draw_button(renderer, control.rect, "-", hovered);
                }
                ControlKind::Button(Action::SizeIncrease)
                | ControlKind::Button(Action::PlayersIncrease) => {
                    draw_button(renderer, control.rect, "+", hovered);
                }
                ControlKind::Button(Action::TheaterNext) => {
                    draw_button(
                        renderer,
                        control.rect,
                        &format!("Theater: {}", self.theater.name()),
                        hovered,
                    );
                }
                ControlKind::Button(Action::Generate) => {
                    draw_button(renderer, control.rect, "Generate Map", hovered);
                }
                ControlKind::Button(Action::SelectTool) => {
                    draw_button(
                        renderer,
                        control.rect,
                        "Select",
                        hovered || self.select_mode,
                    );
                }
                ControlKind::Button(Action::EraseTool) => {
                    draw_button(renderer, control.rect, "Erase", hovered);
                }
                ControlKind::Button(Action::InspectorObjects) => {
                    draw_button(
                        renderer,
                        control.rect,
                        "Obj",
                        hovered || self.inspector_tab == InspectorTab::Objects,
                    );
                }
                ControlKind::Button(Action::InspectorTriggers) => {
                    draw_button(
                        renderer,
                        control.rect,
                        "Trg",
                        hovered || self.inspector_tab == InspectorTab::Triggers,
                    );
                }
                ControlKind::Button(Action::InspectorHistory) => {
                    draw_button(
                        renderer,
                        control.rect,
                        "Hist",
                        hovered || self.inspector_tab == InspectorTab::History,
                    );
                }
                ControlKind::Button(Action::InspectorValidate) => {
                    draw_button(
                        renderer,
                        control.rect,
                        "Valid",
                        hovered || self.inspector_tab == InspectorTab::Validate,
                    );
                }
                ControlKind::Button(Action::InspectorFiles) => {
                    draw_button(
                        renderer,
                        control.rect,
                        "Files",
                        hovered || self.inspector_tab == InspectorTab::Files,
                    );
                }
                ControlKind::Button(Action::NextObjectCategory) => {
                    draw_button(
                        renderer,
                        control.rect,
                        &format!("Cat: {}", self.object_category.label()),
                        hovered,
                    );
                }
                ControlKind::Button(Action::RefreshScenarioFiles) => {
                    draw_button(renderer, control.rect, "Refresh", hovered);
                }
                ControlKind::ScenarioFile(index) => {
                    let label = self
                        .scenario_files
                        .get(index)
                        .and_then(|path| path.file_name())
                        .map_or_else(
                            || "?".to_string(),
                            |name| name.to_string_lossy().to_string(),
                        );
                    draw_button(renderer, control.rect, &label, hovered);
                }
                ControlKind::Button(Action::Undo) => {
                    draw_button(renderer, control.rect, "Undo", hovered);
                }
                ControlKind::Button(Action::Redo) => {
                    draw_button(renderer, control.rect, "Redo", hovered);
                }
                ControlKind::Button(Action::NextTriggerTemplate) => {
                    let (name, _, _) = self.current_trigger_template();
                    draw_button(renderer, control.rect, &format!("Tpl: {name}"), hovered);
                }
                ControlKind::Button(Action::AddTrigger) => {
                    draw_button(renderer, control.rect, "Add", hovered);
                }
                ControlKind::Button(Action::CommitTrigger) => {
                    draw_button(renderer, control.rect, "Apply", hovered);
                }
                ControlKind::Button(Action::CopyObject) => {
                    draw_button(renderer, control.rect, "Copy", hovered);
                }
                ControlKind::Button(Action::PasteObject) => {
                    draw_button(renderer, control.rect, "Paste", hovered);
                }
                ControlKind::Button(Action::NextPlacedOwnerFilter) => {
                    draw_button(
                        renderer,
                        control.rect,
                        &format!("List: {}", self.placed_filter.label()),
                        hovered,
                    );
                }
                ControlKind::Button(Action::ObjectOwnerNext) => {
                    let owner = self
                        .editor
                        .selected_object
                        .and_then(|id| self.map.objects.get(&id))
                        .and_then(|object| object.owner);
                    draw_button(
                        renderer,
                        control.rect,
                        &format!("Owner: {}", owner_label(owner)),
                        hovered,
                    );
                }
                ControlKind::Button(Action::ObjectNudge(dx, dy)) => {
                    let label = match (dx, dy) {
                        (-1, 0) => "X-",
                        (1, 0) => "X+",
                        (0, -1) => "Y-",
                        (0, 1) => "Y+",
                        _ => "?",
                    };
                    draw_button(renderer, control.rect, label, hovered);
                }
                ControlKind::Button(Action::TriggerTool) => {
                    draw_button(renderer, control.rect, "Trg", hovered);
                }
                ControlKind::Button(Action::DeleteObject) => {
                    draw_button(renderer, control.rect, "Del Obj", hovered);
                }
                ControlKind::Button(Action::DeleteTrigger) => {
                    draw_button(renderer, control.rect, "Del Trg", hovered);
                }
                ControlKind::Field(Focus::Path) => {
                    draw_field(
                        renderer,
                        control.rect,
                        &self.path,
                        self.focus == Focus::Path,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::MapWidth) => {
                    draw_field(
                        renderer,
                        control.rect,
                        &self.map_width_text,
                        self.focus == Focus::MapWidth,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::MapHeight) => {
                    draw_field(
                        renderer,
                        control.rect,
                        &self.map_height_text,
                        self.focus == Focus::MapHeight,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::TriggerFilter) => {
                    draw_field(
                        renderer,
                        control.rect,
                        &self.trigger_filter,
                        self.focus == Focus::TriggerFilter,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::PlacedSearch) => {
                    draw_field(
                        renderer,
                        control.rect,
                        &self.placed_search,
                        self.focus == Focus::PlacedSearch,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::ObjectSearch) => {
                    draw_field(
                        renderer,
                        control.rect,
                        &self.object_search,
                        self.focus == Focus::ObjectSearch,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::TriggerName) => {
                    text(
                        renderer,
                        "Name",
                        px,
                        control.rect.y + 6.0,
                        [0.62, 0.72, 0.84, 1.0],
                    );
                    let value = self
                        .trigger_edit
                        .as_ref()
                        .map_or("", |edit| edit.name.as_str());
                    draw_field(
                        renderer,
                        control.rect,
                        value,
                        self.focus == Focus::TriggerName,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::TriggerCondition) => {
                    text(
                        renderer,
                        "Cond",
                        px,
                        control.rect.y + 6.0,
                        [0.62, 0.72, 0.84, 1.0],
                    );
                    let value = self
                        .trigger_edit
                        .as_ref()
                        .map_or("", |edit| edit.condition.as_str());
                    draw_field(
                        renderer,
                        control.rect,
                        value,
                        self.focus == Focus::TriggerCondition,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::TriggerAction) => {
                    text(
                        renderer,
                        "Action",
                        px,
                        control.rect.y + 6.0,
                        [0.62, 0.72, 0.84, 1.0],
                    );
                    let value = self
                        .trigger_edit
                        .as_ref()
                        .map_or("", |edit| edit.action.as_str());
                    draw_field(
                        renderer,
                        control.rect,
                        value,
                        self.focus == Focus::TriggerAction,
                        hovered,
                    );
                }
                ControlKind::Field(Focus::TriggerGroup) => {
                    text(
                        renderer,
                        "Group",
                        px,
                        control.rect.y + 6.0,
                        [0.62, 0.72, 0.84, 1.0],
                    );
                    let value = self
                        .trigger_edit
                        .as_ref()
                        .map_or("", |edit| edit.group.as_str());
                    draw_field(
                        renderer,
                        control.rect,
                        value,
                        self.focus == Focus::TriggerGroup,
                        hovered,
                    );
                }
                ControlKind::PlacedObject(id) => {
                    let label = self.map.objects.get(&id).map_or("?".to_string(), |object| {
                        let owner = match object.owner {
                            None => "none".to_string(),
                            Some(owner) => format!("{owner:?}").replace("House", ""),
                        };
                        format!(
                            "#{} {} {} @{},{}",
                            object.id,
                            object.class_name(),
                            owner,
                            object.position.x,
                            object.position.y
                        )
                    });
                    let building = self
                        .map
                        .objects
                        .get(&id)
                        .is_some_and(|object| object.class_id.is_building());
                    draw_color_button(
                        renderer,
                        control.rect,
                        if building {
                            [0.70, 0.46, 0.16, 1.0]
                        } else {
                            [0.18, 0.46, 0.62, 1.0]
                        },
                        &label,
                        self.editor.selected_object == Some(id),
                        hovered,
                    );
                }
                ControlKind::Trigger(id) => {
                    let label = self
                        .map
                        .triggers
                        .iter()
                        .find(|trigger| trigger.id == id)
                        .map_or("?".to_string(), |trigger| {
                            if trigger.group.is_empty() {
                                trigger.name.clone()
                            } else {
                                format!("{}/{}", trigger.group, trigger.name)
                            }
                        });
                    draw_color_button(
                        renderer,
                        control.rect,
                        [0.30, 0.62, 0.42, 1.0],
                        &label,
                        self.selected_trigger == Some(id),
                        hovered,
                    );
                }
                ControlKind::Terrain(id) => {
                    let active = self.brush_terrain == id && !self.overlay_mode;
                    draw_color_button(
                        renderer,
                        control.rect,
                        terrain_color(id),
                        terrain_chip_label(id),
                        active,
                        hovered,
                    );
                }
                ControlKind::Overlay(id) => {
                    let active = self.brush_terrain == id && self.overlay_mode;
                    let label = crate::generator::OverlayType::from_id(id)
                        .map_or("?", crate::generator::OverlayType::label);
                    let mut color = overlay_color(id);
                    if color[3] == 0.0 {
                        color = [0.28, 0.30, 0.33, 1.0];
                    }
                    draw_color_button(renderer, control.rect, color, label, active, hovered);
                }
                ControlKind::Object(class_id) => {
                    let active = self.selected_object_tool == Some(class_id);
                    let mut label = self.plugin().get_object_name(class_id);
                    if let Some(side) = self.plugin().object_side(class_id) {
                        label = format!("{label} [{side}]");
                    }
                    draw_object_button(renderer, control.rect, class_id, &label, active, hovered);
                }
                _ => {}
            }
        }

        if self.inspector_tab == InspectorTab::Files {
            if let Some(preview) = self
                .scenario_hover
                .and_then(|index| self.scenario_previews.get(index))
                .and_then(|preview| preview.as_ref())
            {
                let rows = self.scenario_files.len().min(5) as f32;
                let preview_y = content_y + 24.0 + rows * 18.0 + 8.0;
                let preview_w = 240.0f32;
                let preview_h = (preview_w * preview.height as f32 / preview.width.max(1) as f32)
                    .clamp(50.0, 70.0);
                let origin_x = px;
                let cell_w = preview_w / preview.width.max(1) as f32;
                let cell_h = preview_h / preview.height.max(1) as f32;
                let step = ((preview.width.max(preview.height) as f32 / 80.0).ceil() as u32).max(1);
                let mut py = 0u32;
                while py < preview.height {
                    let mut cell_x = 0u32;
                    while cell_x < preview.width {
                        let index = py as usize * preview.width as usize + cell_x as usize;
                        let terrain = preview.terrain.get(index).copied().unwrap_or(0);
                        renderer.draw_rect(
                            origin_x + cell_x as f32 * cell_w,
                            preview_y + py as f32 * cell_h,
                            (cell_w * step as f32).max(1.0),
                            (cell_h * step as f32).max(1.0),
                            terrain_color(terrain),
                        );
                        cell_x += step;
                    }
                    py += step;
                }
                let border = [0.35, 0.50, 0.65, 1.0];
                renderer.draw_rect(origin_x, preview_y, preview_w, 1.0, border);
                renderer.draw_rect(
                    origin_x,
                    preview_y + preview_h - 1.0,
                    preview_w,
                    1.0,
                    border,
                );
                renderer.draw_rect(origin_x, preview_y, 1.0, preview_h, border);
                renderer.draw_rect(
                    origin_x + preview_w - 1.0,
                    preview_y,
                    1.0,
                    preview_h,
                    border,
                );
            }
        }

        if self.inspector_tab == InspectorTab::History {
            let undo = self.editor.history.undo_labels();
            let redo = self.editor.history.redo_labels();
            let mut row_y = content_y + 30.0;
            if undo.is_empty() {
                text(
                    renderer,
                    "(no undo history)",
                    px,
                    row_y,
                    [0.55, 0.62, 0.72, 1.0],
                );
                row_y += 12.0;
            } else {
                for (index, label) in undo.iter().take(7).enumerate() {
                    fit_text(
                        renderer,
                        &format!("{}. {}", index + 1, label),
                        px,
                        row_y,
                        (RIGHT_W - 28.0).max(0.0),
                        [0.78, 0.86, 0.94, 1.0],
                    );
                    row_y += 12.0;
                }
            }
            if !redo.is_empty() {
                row_y += 4.0;
                text(renderer, "REDO", px, row_y, [0.55, 0.70, 0.85, 1.0]);
                row_y += 12.0;
                for (index, label) in redo.iter().take(3).enumerate() {
                    fit_text(
                        renderer,
                        &format!("{}. {}", index + 1, label),
                        px,
                        row_y,
                        (RIGHT_W - 28.0).max(0.0),
                        [0.62, 0.72, 0.84, 1.0],
                    );
                    row_y += 12.0;
                }
            }
        }

        if self.inspector_tab == InspectorTab::Validate {
            let issues = self.validation_issues();
            let mut row_y = content_y + 30.0;
            if issues.is_empty() {
                text(
                    renderer,
                    "map looks valid",
                    px,
                    row_y,
                    [0.55, 0.82, 0.62, 1.0],
                );
                row_y += 12.0;
            } else {
                for (index, issue) in issues.iter().take(5).enumerate() {
                    fit_text(
                        renderer,
                        &format!("{}. {}", index + 1, issue),
                        px,
                        row_y,
                        (RIGHT_W - 28.0).max(0.0),
                        [0.92, 0.78, 0.55, 1.0],
                    );
                    row_y += 12.0;
                }
            }
            row_y += 8.0;
            text(renderer, "STATS", px, row_y, [0.55, 0.70, 0.85, 1.0]);
            row_y += 12.0;
            for line in self.scenario_stats() {
                fit_text(
                    renderer,
                    &line,
                    px,
                    row_y,
                    (RIGHT_W - 28.0).max(0.0),
                    [0.72, 0.80, 0.90, 1.0],
                );
                row_y += 12.0;
            }
        }
    }

    /// Open the map-info modal with the current scenario metadata.
    pub fn open_map_info(&mut self) {
        self.map_info_open = true;
        self.map_info_name = self.map.name.clone();
        self.map_info_player = self.map.player.clone();
        self.map_info_briefing = self.map.briefing.clone();
        self.map_info_objectives = self.map.objectives.join(" | ");
        self.focus = Focus::MapName;
    }

    /// Close the modal, optionally committing the edited metadata first.
    pub fn close_map_info(&mut self, commit: bool) {
        if commit {
            self.commit_map_info();
        }
        self.map_info_open = false;
        self.focus = Focus::None;
    }

    pub fn map_info_open(&self) -> bool {
        self.map_info_open
    }

    /// Commit the modal's fields as one undoable scenario-info edit.
    pub fn commit_map_info(&mut self) {
        let before = ScenarioInfo::from_model(&self.map);
        let objectives = self
            .map_info_objectives
            .split('|')
            .map(str::trim)
            .filter(|objective| !objective.is_empty())
            .map(str::to_string)
            .collect();
        let after = ScenarioInfo {
            name: self.map_info_name.trim().to_string(),
            player: self.map_info_player.trim().to_string(),
            briefing: self.map_info_briefing.trim().to_string(),
            objectives,
        };
        if let Some(command) = SetScenarioInfo::new(before, after) {
            self.editor
                .history
                .execute(&mut self.map, Box::new(command));
            self.status = "scenario info updated".to_string();
        } else {
            self.status = "scenario info unchanged".to_string();
        }
    }

    fn draw_map_info(&self, renderer: &mut dyn Renderer, width: f32, height: f32) {
        renderer.draw_rect(0.0, 0.0, width, height, [0.01, 0.02, 0.04, 0.88]);
        let panel_w = 680.0f32.min(width - 40.0);
        let panel_h = 300.0f32.min(height - 40.0);
        let x = (width - panel_w) * 0.5;
        let y = (height - panel_h) * 0.5;
        renderer.draw_rect(x, y, panel_w, panel_h, [0.06, 0.08, 0.12, 1.0]);
        renderer.draw_rect(x, y, panel_w, 2.0, [0.30, 0.55, 0.85, 1.0]);
        text(
            renderer,
            "MAP INFO",
            x + 20.0,
            y + 16.0,
            [0.85, 0.92, 1.0, 1.0],
        );
        let rows = [
            ("Name", &self.map_info_name, Focus::MapName),
            ("Side", &self.map_info_player, Focus::MapPlayer),
            ("Briefing", &self.map_info_briefing, Focus::MapBriefing),
            (
                "Objectives (| separated)",
                &self.map_info_objectives,
                Focus::MapObjectives,
            ),
        ];
        let mut row_y = y + 46.0;
        for (label, value, focus) in rows {
            let active = self.focus == focus;
            text(
                renderer,
                &format!("{}{}:", if active { ">" } else { " " }, label),
                x + 20.0,
                row_y,
                if active {
                    [1.0, 0.85, 0.45, 1.0]
                } else {
                    [0.62, 0.72, 0.85, 1.0]
                },
            );
            fit_text(
                renderer,
                value,
                x + 20.0,
                row_y + 14.0,
                (RIGHT_W - 48.0).max(0.0),
                if active {
                    [0.95, 0.97, 1.0, 1.0]
                } else {
                    [0.78, 0.84, 0.92, 1.0]
                },
            );
            row_y += 44.0;
        }
        text(
            renderer,
            "Enter: next field (saves on the last)   Esc: cancel   Ctrl+I: close",
            x + 20.0,
            y + panel_h - 26.0,
            [0.60, 0.70, 0.85, 1.0],
        );
    }

    /// Toggle the shortcut overlay (F1).
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn help_open(&self) -> bool {
        self.show_help
    }

    fn draw_help(&self, renderer: &mut dyn Renderer, width: f32, height: f32) {
        renderer.draw_rect(0.0, 0.0, width, height, [0.01, 0.02, 0.04, 0.90]);
        let panel_w = 640.0f32.min(width - 40.0);
        let panel_h = 330.0f32.min(height - 40.0);
        let x = (width - panel_w) * 0.5;
        let y = (height - panel_h) * 0.5;
        renderer.draw_rect(x, y, panel_w, panel_h, [0.06, 0.08, 0.12, 1.0]);
        renderer.draw_rect(x, y, panel_w, 2.0, [0.55, 0.45, 0.22, 1.0]);
        text(
            renderer,
            "SHORTCUTS",
            x + 20.0,
            y + 16.0,
            [0.95, 0.82, 0.45, 1.0],
        );
        let lines = [
            "Ctrl+S save    Ctrl+O open    Ctrl+N new    Ctrl+G generate",
            "Ctrl+Z undo    Ctrl+Y redo    Ctrl+C copy    Ctrl+V paste",
            "Tab grid    F5 regenerate    Ctrl+I map info    Ctrl+A select all    Ctrl+D duplicate",
            "LMB paint/select    RMB/MMB pan    wheel zoom",
            "Tabs: Objects / Triggers / History / Validate / Files",
            "Trigger fields commit with Enter; copy/paste uses Ctrl+C/V.",
        ];
        let mut row = y + 48.0;
        for line in lines {
            text(renderer, line, x + 20.0, row, [0.78, 0.85, 0.94, 1.0]);
            row += 20.0;
        }
        text(
            renderer,
            "Esc or F1 closes this help",
            x + 20.0,
            y + panel_h - 24.0,
            [0.62, 0.72, 0.84, 1.0],
        );
    }

    fn draw_status(&self, renderer: &mut dyn Renderer, width: f32, height: f32) {
        renderer.draw_rect(
            0.0,
            height - STATUS_H,
            width,
            STATUS_H,
            [0.10, 0.12, 0.16, 1.0],
        );
        renderer.draw_rect(0.0, height - STATUS_H, width, 1.0, [0.22, 0.30, 0.40, 1.0]);
        let line = self.summary_line();
        fit_text(
            renderer,
            &line,
            8.0,
            height - STATUS_H + 8.0,
            (width - 16.0).max(0.0),
            [0.78, 0.86, 0.94, 1.0],
        );
    }
}

fn text(renderer: &mut dyn Renderer, text: &str, x: f32, y: f32, color: [f32; 4]) {
    renderer.draw_text(text, x, y, 8.0, color);
}

fn text_sized(renderer: &mut dyn Renderer, text: &str, x: f32, y: f32, size: f32, color: [f32; 4]) {
    renderer.draw_text(text, x, y, size, color);
}

fn draw_button(renderer: &mut dyn Renderer, rect: RectF, label: &str, hovered: bool) {
    let fill = if hovered {
        [0.22, 0.30, 0.40, 1.0]
    } else {
        [0.15, 0.19, 0.25, 1.0]
    };
    renderer.draw_rect(rect.x, rect.y, rect.w, rect.h, fill);
    renderer.draw_rect(rect.x, rect.y, rect.w, 1.0, [0.30, 0.40, 0.52, 1.0]);
    renderer.draw_rect(
        rect.x,
        rect.y + rect.h - 1.0,
        rect.w,
        1.0,
        [0.30, 0.40, 0.52, 1.0],
    );
    renderer.draw_rect(rect.x, rect.y, 1.0, rect.h, [0.30, 0.40, 0.52, 1.0]);
    renderer.draw_rect(
        rect.x + rect.w - 1.0,
        rect.y,
        1.0,
        rect.h,
        [0.30, 0.40, 0.52, 1.0],
    );
    let size = fit_size(label, 8.0, (rect.w - 6.0).max(0.0));
    let tw = text_w(label, size);
    let th = text_h(size);
    // Truly centered: (rect.x + 1.0) is only a safety net for a label that
    // does not fit even at the minimum font size.
    let centered = (rect.x + (rect.w - tw) * 0.5).max(rect.x + 1.0);
    text_sized(
        renderer,
        label,
        centered,
        rect.y + rect.h * 0.5 - th * 0.5,
        size,
        [0.88, 0.92, 0.98, 1.0],
    );
}

/// Palette button with a small category glyph (infantry/vehicle/air/building).
fn draw_object_button(
    renderer: &mut dyn Renderer,
    rect: RectF,
    class_id: ClassId,
    label: &str,
    active: bool,
    hovered: bool,
) {
    let fill = if active {
        [0.30, 0.44, 0.30, 1.0]
    } else if hovered {
        [0.22, 0.30, 0.40, 1.0]
    } else {
        [0.14, 0.18, 0.24, 1.0]
    };
    renderer.draw_rect(rect.x, rect.y, rect.w, rect.h, fill);
    let color = if class_id.is_building() {
        [0.95, 0.65, 0.25, 1.0]
    } else {
        [0.35, 0.80, 1.00, 1.0]
    };
    let x = rect.x + 3.0;
    let y = rect.y + 3.0;
    if class_id.is_building() {
        renderer.draw_rect(x, y, 9.0, 7.0, color);
        renderer.draw_rect(x + 2.0, y + 2.0, 5.0, 3.0, [0.10, 0.12, 0.16, 1.0]);
    } else if ObjectCategory::Aircraft.matches(class_id) {
        renderer.draw_rect(x + 3.0, y, 3.0, 10.0, color);
        renderer.draw_rect(x, y + 3.0, 9.0, 2.0, color);
    } else if ObjectCategory::Infantry.matches(class_id) {
        renderer.draw_rect(x + 3.0, y, 3.0, 3.0, color);
        renderer.draw_rect(x + 2.0, y + 4.0, 5.0, 6.0, color);
    } else {
        renderer.draw_rect(x, y + 2.0, 9.0, 5.0, color);
        renderer.draw_rect(x + 3.0, y, 3.0, 3.0, color);
    }
    let size = fit_size(label, 8.0, (rect.w - 14.0).max(0.0));
    let tw = text_w(label, size);
    let th = text_h(size);
    // Centered in the space right of the category glyph.
    let centered = (rect.x + 16.0 + (rect.w - 14.0 - tw) * 0.5).max(rect.x + 16.0);
    text_sized(
        renderer,
        label,
        centered,
        rect.y + rect.h * 0.5 - th * 0.5,
        size,
        [0.88, 0.92, 0.98, 1.0],
    );
}

fn draw_color_button(
    renderer: &mut dyn Renderer,
    rect: RectF,
    swatch: [f32; 4],
    label: &str,
    active: bool,
    hovered: bool,
) {
    let fill = if active {
        [0.30, 0.44, 0.30, 1.0]
    } else if hovered {
        [0.22, 0.30, 0.40, 1.0]
    } else {
        [0.14, 0.18, 0.24, 1.0]
    };
    renderer.draw_rect(rect.x, rect.y, rect.w, rect.h, fill);
    renderer.draw_rect(rect.x + 2.0, rect.y + 2.0, 10.0, 10.0, swatch);
    let size = fit_size(label, 8.0, (rect.w - 18.0).max(0.0));
    let tw = text_w(label, size);
    let th = text_h(size);
    // Centered in the space right of the color swatch.
    let centered = (rect.x + 16.0 + (rect.w - 18.0 - tw) * 0.5).max(rect.x + 16.0);
    text_sized(
        renderer,
        label,
        centered,
        rect.y + rect.h * 0.5 - th * 0.5,
        size,
        [0.86, 0.90, 0.96, 1.0],
    );
}

fn draw_field(renderer: &mut dyn Renderer, rect: RectF, value: &str, focused: bool, hovered: bool) {
    let fill = if focused {
        [0.18, 0.24, 0.30, 1.0]
    } else if hovered {
        [0.18, 0.22, 0.28, 1.0]
    } else {
        [0.12, 0.15, 0.20, 1.0]
    };
    renderer.draw_rect(rect.x, rect.y, rect.w, rect.h, fill);
    renderer.draw_rect(rect.x, rect.y, rect.w, 1.0, [0.30, 0.42, 0.55, 1.0]);
    let size = fit_size(value, 8.0, (rect.w - 8.0).max(0.0));
    let tw = text_w(value, size);
    let th = text_h(size);
    // Center short numeric values (seed, map width/height); free-form text
    // (paths, search queries, trigger fields) stays left aligned.
    let centered = !value.is_empty() && value.chars().all(|c| c.is_ascii_digit());
    let x = if centered {
        (rect.x + (rect.w - tw) * 0.5).max(rect.x + 2.0)
    } else {
        rect.x + 4.0
    };
    text_sized(
        renderer,
        value,
        x,
        rect.y + rect.h * 0.5 - th * 0.5,
        size,
        [0.92, 0.94, 0.98, 1.0],
    );
    if focused {
        renderer.draw_rect(
            rect.x,
            rect.y + rect.h - 2.0,
            rect.w,
            2.0,
            [0.45, 0.75, 1.0, 1.0],
        );
    }
}

/// Pixel width of `text` at the given font size (fixed-width font).
///
/// Matches `cnc_renderer::draw_text_block`: advance = 10 * size / 8 per
/// character, spaces advance at half width. Keeping this in sync with the
/// renderer is what makes fit_size thresholds and centering positions exact.
fn text_w(text: &str, size: f32) -> f32 {
    let advance = 10.0 * size / 8.0;
    text.chars()
        .map(|c| if c == ' ' { advance * 0.5 } else { advance })
        .sum()
}

/// Line height for `size`-pixel font (bitmap font: size * 11 / 8).
fn text_h(size: f32) -> f32 {
    size * 11.0 / 8.0
}

/// Largest size (step 0.5, floor 4.0) at which `text` fits within `max_w` pixels.
fn fit_size(text: &str, size: f32, max_w: f32) -> f32 {
    let mut s = size;
    while s > 4.0 && text_w(text, s) > max_w {
        s -= 0.5;
    }
    s
}

/// Palette chip label for a terrain brush. The three-column palette chips
/// cannot fit "Forest Floor"/"Jungle Floor" at the full 8px font, so the
/// desktop chips use the one-word forms; status messages keep the full label.
fn terrain_chip_label(id: u8) -> &'static str {
    match TerrainType::from_id(id) {
        Some(TerrainType::ForestFloor) => "Forest",
        Some(TerrainType::JungleFloor) => "Jungle",
        other => other.map_or("?", TerrainType::label),
    }
}

/// Draw `text` at `(x, y)`, shrinking the font (minimum 4.0) until it fits
/// within `max_w` pixels. Never truncates or drops characters.
fn fit_text(renderer: &mut dyn Renderer, text: &str, x: f32, y: f32, max_w: f32, color: [f32; 4]) {
    let size = fit_size(text, 8.0, max_w);
    text_sized(renderer, text, x, y, size, color);
}

/// Every scenario INI in a directory, sorted by filename.
pub fn scenario_files_in(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
        })
        .collect();
    paths.sort();
    paths
}

/// Parse a scenario file into a terrain thumbnail.
pub fn scenario_preview(path: &Path) -> Option<ScenarioPreview> {
    let text = std::fs::read_to_string(path).ok()?;
    let map = MapModel::from_ini(&text).ok()?;
    if map.width == 0 || map.height == 0 {
        return None;
    }
    Some(ScenarioPreview {
        width: map.width,
        height: map.height,
        terrain: map.terrain,
    })
}

fn parse_dimension(text: &str, fallback: u32) -> u32 {
    text.trim()
        .parse::<u32>()
        .map(|value| value.clamp(8, 512))
        .unwrap_or(fallback)
}

fn owner_label(owner: Option<ClassId>) -> &'static str {
    match owner {
        None => "None",
        Some(ClassId::HouseGDI) => "GDI",
        Some(ClassId::HouseNod) => "Nod",
        Some(ClassId::HouseAllies) => "Allies",
        Some(ClassId::HouseSoviet) => "Soviet",
        Some(_) => "Other",
    }
}

/// Texture id used for terrain tiles: the renderer's dedicated tileset
/// texture slot, separate from the procedural unit atlas on slot 0.
const TERRAIN_TILE_TEXTURE: u32 = cnc_renderer::TILESET_TEXTURE_ID;

pub fn terrain_color(id: u8) -> [f32; 4] {
    match TerrainType::from_id(id) {
        Some(TerrainType::Grass) => [0.22, 0.44, 0.22, 1.0],
        Some(TerrainType::Water) => [0.10, 0.24, 0.46, 1.0],
        Some(TerrainType::Sand) => [0.62, 0.56, 0.34, 1.0],
        Some(TerrainType::Dirt) => [0.38, 0.30, 0.20, 1.0],
        Some(TerrainType::Rock) => [0.34, 0.34, 0.36, 1.0],
        Some(TerrainType::Snow) => [0.78, 0.82, 0.86, 1.0],
        Some(TerrainType::Rough) => [0.28, 0.38, 0.24, 1.0],
        Some(TerrainType::Mud) => [0.30, 0.24, 0.16, 1.0],
        Some(TerrainType::Crater) => [0.20, 0.21, 0.22, 1.0],
        Some(TerrainType::Scorched) => [0.24, 0.18, 0.13, 1.0],
        Some(TerrainType::Swamp) => [0.16, 0.24, 0.18, 1.0],
        Some(TerrainType::Ice) => [0.55, 0.70, 0.80, 1.0],
        Some(TerrainType::Concrete) => [0.44, 0.46, 0.47, 1.0],
        Some(TerrainType::ForestFloor) => [0.13, 0.21, 0.13, 1.0],
        Some(TerrainType::JungleFloor) => [0.11, 0.26, 0.15, 1.0],
        None => [0.45, 0.45, 0.45, 1.0],
    }
}

pub fn overlay_color(id: u8) -> [f32; 4] {
    match id {
        1 => [0.30, 0.90, 0.30, 1.0],
        2 => [0.10, 0.35, 0.12, 1.0],
        3 => [0.80, 0.62, 0.20, 1.0],
        4 => [0.36, 0.37, 0.40, 1.0],
        5 => [0.66, 0.66, 0.64, 1.0],
        _ => [0.0, 0.0, 0.0, 0.0],
    }
}

fn start_color(index: usize) -> [f32; 4] {
    const COLORS: [[f32; 4]; 6] = [
        [1.0, 0.85, 0.25, 1.0],
        [1.0, 0.45, 0.35, 1.0],
        [0.45, 0.85, 1.0, 1.0],
        [0.85, 0.55, 1.0, 1.0],
        [0.55, 1.0, 0.55, 1.0],
        [1.0, 0.65, 0.85, 1.0],
    ];
    COLORS[index % COLORS.len()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_renderer::SoftwareRenderer;

    #[test]
    fn layout_controls_stay_inside_the_window() {
        let mut state = EditorState::new(1280.0, 800.0);
        for index in 0..4 {
            state.map.add_trigger(Trigger {
                id: 0,
                name: format!("trigger {index}"),
                condition: "always".to_string(),
                action: "win".to_string(),
                group: "Wave".to_string(),
            });
        }
        for index in 0..6 {
            state.map.insert_object(MapObject {
                id: 0,
                class_id: ClassId::VehicleE1,
                position: Vec2i::new(index, 0),
                owner: None,
                ..Default::default()
            });
        }
        for tab in [
            InspectorTab::Triggers,
            InspectorTab::Objects,
            InspectorTab::History,
            InspectorTab::Validate,
            InspectorTab::Files,
        ] {
            state.inspector_tab = tab;
            for control in state.controls() {
                assert!(control.rect.x >= 0.0);
                assert!(control.rect.x + control.rect.w <= 1280.0);
                assert!(
                    control.rect.y + control.rect.h <= 800.0,
                    "{tab:?} control {:?} overflows: {:?}",
                    control.kind,
                    control.rect
                );
            }
        }
    }

    #[test]
    fn fit_shows_the_whole_map_and_cell_conversion_roundtrips() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.fit();
        let area = state.map_area();
        assert!(
            state.pan_x > 0.0 || state.map.width as f32 * TILE_SIZE * state.zoom >= area.w - 1.0
        );
        let cell = Vec2i::new(5, 7);
        let (sx, sy) = state.cell_screen(cell);
        assert_eq!(state.cell_at(sx + 2.0, sy + 2.0), cell);
    }

    #[test]
    fn painting_is_one_undo_step_and_restores() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(32, 32);
        state.fit();
        state.brush_terrain = TerrainType::Rock.id();
        state.set_terrain_tool();
        let start = Vec2i::new(4, 4);
        let (sx, sy) = state.cell_screen(start);
        state.pointer_down(sx + 2.0, sy + 2.0, PointerButton::Left);
        for step in 1..=3 {
            let (px, py) = state.cell_screen(Vec2i::new(4 + step, 4));
            state.pointer_move(px + 2.0, py + 2.0);
        }
        state.pointer_up(PointerButton::Left);
        assert_eq!(state.map.terrain[4 * 32 + 4], TerrainType::Rock.id());
        assert_eq!(state.map.terrain[4 * 32 + 7], TerrainType::Rock.id());
        assert_eq!(state.editor.history.undo_len(), 1);
        state.undo();
        assert_eq!(state.map.terrain[4 * 32 + 4], 0);
    }

    #[test]
    fn generate_button_replaces_the_map_and_resets_history() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.seed = "99".to_string();
        state.set_dimensions(48, 48);
        state.players = 3;
        state.generate();
        assert_eq!(state.map.width, 48);
        assert_eq!(state.map.player_starts.len(), 3);
        assert!(!state.editor.history.can_undo());
        assert!(state.status.contains("seed 99"));
    }

    #[test]
    fn save_and_open_roundtrip_through_the_path_field() {
        let dir = std::env::temp_dir().join(format!("cnc-editor-ui-{}", std::process::id()));
        let path = dir.join("map.ini");
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(24, 24);
        state.map.terrain[3] = TerrainType::Sand.id();
        state.path = path.display().to_string();
        state.save();
        assert!(path.exists(), "{}", state.status);
        state.map = MapModel::new(8, 8);
        state.open();
        assert_eq!(state.map.width, 24);
        assert_eq!(state.map.terrain[3], TerrainType::Sand.id());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn renderer_draws_a_non_empty_frame() {
        let mut state = EditorState::new(800.0, 600.0);
        let mut renderer = SoftwareRenderer::new(800, 600);
        state.render(&mut renderer, 800.0, 600.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(
            non_black > 1000,
            "expected a populated frame, got {non_black}"
        );
    }

    #[test]
    fn help_overlay_toggles_closes_and_renders() {
        let mut state = EditorState::new(800.0, 600.0);
        assert!(!state.help_open());
        state.toggle_help();
        assert!(state.help_open());
        let mut renderer = SoftwareRenderer::new(800, 600);
        state.render(&mut renderer, 800.0, 600.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(non_black > 1000, "expected a populated help frame");
        state.key_escape();
        assert!(!state.help_open(), "Escape closes help");
    }

    #[test]
    fn select_all_and_clear_cover_the_map() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(8, 8);
        for cell in [(1, 1), (3, 3), (5, 5)] {
            state.map.insert_object(MapObject {
                id: 0,
                class_id: ClassId::InfantryE1,
                position: Vec2i::new(cell.0, cell.1),
                owner: None,
                ..Default::default()
            });
        }
        state.select_all_objects();
        assert_eq!(state.editor.selected_objects.len(), 3);
        assert_eq!(state.editor.selected_object, Some(1));
        state.clear_object_selection();
        assert!(state.editor.selected_objects.is_empty());
        assert!(state.editor.selected_object.is_none());
    }

    #[test]
    fn replace_selected_class_changes_all_and_undoes() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(8, 8);
        let first = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(2, 2),
            owner: Some(ClassId::HouseGDI),
            ..Default::default()
        });
        let second = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(4, 4),
            owner: Some(ClassId::HouseNod),
            ..Default::default()
        });
        state.editor.selected_objects = vec![first, second];
        state.replace_selected_class(ClassId::VehicleE2);
        assert_eq!(state.map.objects[&first].class_id, ClassId::VehicleE2);
        assert_eq!(state.map.objects[&second].class_id, ClassId::VehicleE2);
        assert_eq!(state.map.objects[&first].position, Vec2i::new(2, 2));
        assert_eq!(state.map.objects[&second].owner, Some(ClassId::HouseNod));
        state.undo();
        assert_eq!(state.map.objects[&first].class_id, ClassId::VehicleE1);
        assert_eq!(state.map.objects[&second].class_id, ClassId::VehicleE1);
    }

    #[test]
    fn duplicate_selection_copies_offset_and_undoes() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(8, 8);
        let id = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(2, 3),
            owner: Some(ClassId::HouseGDI),
            ..Default::default()
        });
        state.editor.selected_objects = vec![id];
        state.editor.selected_object = Some(id);
        state.duplicate_selection();
        assert_eq!(state.map.objects.len(), 2);
        let copy = state
            .map
            .object_at(Vec2i::new(3, 4))
            .expect("copy at offset cell");
        assert_ne!(copy.id, id);
        assert_eq!(copy.class_id, ClassId::VehicleE1);
        assert_eq!(copy.owner, Some(ClassId::HouseGDI));
        assert_eq!(state.editor.selected_objects, vec![copy.id]);
        let copy_id = copy.id;
        state.undo();
        assert_eq!(state.map.objects.len(), 1);
        assert!(!state.map.objects.contains_key(&copy_id));
        assert!(state.map.objects.contains_key(&id));
    }

    #[test]
    fn delete_selected_objects_is_one_undo_step() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(8, 8);
        let mut ids = Vec::new();
        for (index, cell) in [(1, 1), (3, 3), (5, 5)].into_iter().enumerate() {
            ids.push(state.map.insert_object(MapObject {
                id: 0,
                class_id: ClassId::InfantryE1,
                position: Vec2i::new(cell.0, cell.1),
                owner: None,
                ..Default::default()
            }));
            let _ = index;
        }
        state.editor.selected_objects = ids.clone();
        state.delete_selected_objects();
        assert!(state.map.objects.is_empty());
        assert!(state.editor.selected_objects.is_empty());
        state.undo();
        assert_eq!(state.map.objects.len(), 3, "one undo restores all");
        for id in ids {
            assert!(state.map.objects.contains_key(&id), "object {id} restored");
        }
    }

    #[test]
    fn map_info_modal_commits_and_undoes() {
        let mut state = EditorState::new(1280.0, 800.0);
        assert!(!state.map_info_open());
        state.open_map_info();
        assert!(state.map_info_open());
        assert_eq!(state.focus, Focus::MapName);
        state.map_info_name = "New Name".to_string();
        state.map_info_player = "Soviet".to_string();
        state.map_info_briefing = "Brief".to_string();
        state.map_info_objectives = "A | B".to_string();
        state.commit_map_info();
        assert_eq!(state.map.name, "New Name");
        assert_eq!(state.map.player, "Soviet");
        assert_eq!(state.map.briefing, "Brief");
        assert_eq!(state.map.objectives, vec!["A", "B"]);
        state.undo();
        assert_eq!(state.map.name, "Untitled Map");
        assert!(state.map.objectives.is_empty());

        // Enter on the last field saves and closes.
        state.open_map_info();
        state.focus = Focus::MapObjectives;
        state.map_info_objectives = "Only".to_string();
        state.key_enter();
        assert!(!state.map_info_open());
        assert_eq!(state.map.objectives, vec!["Only"]);

        // Escape discards edits.
        state.open_map_info();
        state.map_info_name = "Discard".to_string();
        state.key_escape();
        assert!(!state.map_info_open());
        assert_ne!(state.map.name, "Discard");

        state.open_map_info();
        let mut renderer = SoftwareRenderer::new(800, 600);
        state.render(&mut renderer, 800.0, 600.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(non_black > 1000);
    }

    #[test]
    fn check_all_agents_reports_each_script() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map.add_agent(AgentDefinition::new(1, "Good"));
        let mut bad = AgentDefinition::new(2, "Bad");
        bad.script = "this is not a valid script".to_string();
        state.map.add_agent(bad);
        state.check_all_agents();
        assert!(state
            .agent_report
            .iter()
            .any(|line| line.starts_with("OK #1")));
        assert!(state
            .agent_report
            .iter()
            .any(|line| line.starts_with("FAIL #2")));
        assert!(state.agent_bad.contains(&2));
        assert!(!state.agent_bad.contains(&1));
        assert!(state.agent_message.contains("2 agents, 1 failed"));
        assert_eq!(
            state.agent_report.len(),
            2,
            "one report line per agent: {:?}",
            state.agent_report
        );
    }

    #[test]
    fn text_input_respects_focus_and_seed_digits() {
        let mut state = EditorState::new(800.0, 600.0);
        state.focus = Focus::Seed;
        state.seed.clear();
        state.key_text("12a3");
        assert_eq!(state.seed, "123");
        state.key_backspace();
        assert_eq!(state.seed, "12");
        state.focus = Focus::Path;
        state.path.clear();
        state.key_text("maps/x.ini");
        assert_eq!(state.path, "maps/x.ini");
    }

    #[test]
    fn trigger_condition_and_action_fields_are_editable_and_undoable() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.add_trigger();
        let id = state.selected_trigger.expect("new trigger is selected");

        state.focus = Focus::TriggerCondition;
        state.trigger_edit.as_mut().unwrap().condition.clear();
        state.key_text("elapsed:60");
        state.focus = Focus::TriggerAction;
        state.trigger_edit.as_mut().unwrap().action.clear();
        state.key_text("win");
        state.focus = Focus::TriggerGroup;
        state.trigger_edit.as_mut().unwrap().group.clear();
        state.key_text("Act 1");
        state.key_enter();

        let trigger = state
            .map
            .triggers
            .iter()
            .find(|trigger| trigger.id == id)
            .unwrap();
        assert_eq!(trigger.condition, "elapsed:60");
        assert_eq!(trigger.action, "win");
        assert_eq!(trigger.group, "Act 1");
        assert_eq!(state.editor.history.undo_len(), 2, "add + one edit command");

        state.undo();
        let trigger = state
            .map
            .triggers
            .iter()
            .find(|trigger| trigger.id == id)
            .unwrap();
        assert_eq!(trigger.condition, "structures:1:any:==:0");
        assert_eq!(trigger.action, "win");
        assert_eq!(trigger.group, "");

        state.undo();
        assert!(state.map.triggers.is_empty());
        assert_eq!(state.selected_trigger, None);
        assert!(state.trigger_edit.is_none());
    }

    #[test]
    fn selecting_a_trigger_loads_its_fields() {
        let mut state = EditorState::new(1280.0, 800.0);
        let id = state.map.add_trigger(Trigger {
            id: 0,
            name: "victory".to_string(),
            condition: "structures:1:any:==:0".to_string(),
            action: "win".to_string(),
            group: "Victory".to_string(),
        });
        state.select_trigger(id);
        assert_eq!(state.selected_trigger, Some(id));
        let edit = state.trigger_edit.as_ref().unwrap();
        assert_eq!(edit.name, "victory");
        assert_eq!(edit.condition, "structures:1:any:==:0");
        assert_eq!(edit.action, "win");
    }

    #[test]
    fn select_tool_drags_objects_as_one_undo_step() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        let id = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(2, 2),
            owner: None,
            ..Default::default()
        });
        state.set_select_tool();

        let (sx, sy) = state.cell_screen(Vec2i::new(2, 2));
        state.pointer_down(sx + 2.0, sy + 2.0, PointerButton::Left);
        assert_eq!(state.dragging_object, Some(id));
        assert_eq!(state.editor.selected_object, Some(id));

        let (mx, my) = state.cell_screen(Vec2i::new(5, 2));
        state.pointer_move(mx + 2.0, my + 2.0);
        assert_eq!(state.map.objects[&id].position, Vec2i::new(5, 2));

        let (ex, ey) = state.cell_screen(Vec2i::new(5, 4));
        state.pointer_move(ex + 2.0, ey + 2.0);
        state.pointer_up(PointerButton::Left);
        assert_eq!(state.map.objects[&id].position, Vec2i::new(5, 4));
        assert_eq!(state.dragging_object, None);
        assert_eq!(
            state.editor.history.undo_len(),
            1,
            "a whole drag is one undo step"
        );

        state.undo();
        assert_eq!(state.map.objects[&id].position, Vec2i::new(2, 2));
    }

    #[test]
    fn dragging_cannot_stack_objects_on_an_occupied_cell() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        let mover = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(1, 1),
            owner: None,
            ..Default::default()
        });
        state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(4, 1),
            owner: None,
            ..Default::default()
        });
        state.set_select_tool();
        let (sx, sy) = state.cell_screen(Vec2i::new(1, 1));
        state.pointer_down(sx + 2.0, sy + 2.0, PointerButton::Left);
        let (tx, ty) = state.cell_screen(Vec2i::new(4, 1));
        state.pointer_move(tx + 2.0, ty + 2.0);
        assert_eq!(state.map.objects[&mover].position, Vec2i::new(1, 1));
        state.pointer_up(PointerButton::Left);
        assert!(!state.editor.history.can_undo());
    }

    #[test]
    fn trigger_tool_placement_selects_the_new_trigger_for_editing() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(8, 8);
        state.activate(Action::TriggerTool);
        let (sx, sy) = state.cell_screen(Vec2i::new(2, 3));
        state.pointer_down(sx + 2.0, sy + 2.0, PointerButton::Left);
        state.pointer_up(PointerButton::Left);
        assert_eq!(state.map.triggers.len(), 1);
        let id = state.map.triggers[0].id;
        assert_eq!(state.selected_trigger, Some(id));
        assert_eq!(state.trigger_edit.as_ref().map(|edit| edit.id), Some(id));

        state.focus = Focus::TriggerAction;
        state.trigger_edit.as_mut().unwrap().action = "reinforce".to_string();
        state.key_enter();
        assert_eq!(state.map.triggers[0].action, "reinforce");
        state.undo();
        assert_eq!(state.map.triggers[0].action, "win");
        assert!(
            state.map.triggers[0].name.starts_with("Victory"),
            "{}",
            state.map.triggers[0].name
        );
    }

    #[test]
    fn trigger_templates_follow_the_active_plugin() {
        let mut state = EditorState::new(1280.0, 800.0);
        assert_eq!(state.current_trigger_template().0, "Victory");
        state.next_trigger_template();
        assert_eq!(state.current_trigger_template().0, "Defeat");
        state.next_trigger_template();
        assert_eq!(state.current_trigger_template().0, "Timer");

        // Switching to Red Alert resets to that plugin's first template and
        // exposes its own third entry.
        state.activate(Action::TogglePlugin);
        assert_eq!(state.current_trigger_template().0, "Victory");
        state.trigger_template_index = 2;
        assert_eq!(state.current_trigger_template().0, "Chronosphere");

        state.add_trigger();
        let trigger = state.map.triggers.last().unwrap();
        assert_eq!(trigger.name, "Chronosphere");
        assert_eq!(trigger.condition, "elapsed:600");
        assert_eq!(trigger.action, "message:Chronosphere ready");
    }

    #[test]
    fn custom_dimensions_drive_new_and_generated_maps() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.focus = Focus::MapWidth;
        state.map_width_text.clear();
        state.key_text("40");
        state.focus = Focus::MapHeight;
        state.map_height_text.clear();
        state.key_text("28");
        state.key_enter();
        assert_eq!((state.map_width, state.map_height), (40, 28));

        state.new_map();
        assert_eq!((state.map.width, state.map.height), (40, 28));

        state.set_dimensions(72, 48);
        state.generate();
        assert_eq!((state.map.width, state.map.height), (72, 48));

        // Out-of-range values clamp when applied.
        state.set_dimensions(2, 9999);
        assert_eq!((state.map_width, state.map_height), (8, 512));
    }

    #[test]
    fn trigger_filter_matches_group_and_name() {
        let mut state = EditorState::new(1280.0, 800.0);
        for (index, group) in ["Wave", "Victory", "Wave"].iter().enumerate() {
            state.map.add_trigger(Trigger {
                id: 0,
                name: format!("trigger {index}"),
                condition: "always".to_string(),
                action: "win".to_string(),
                group: group.to_string(),
            });
        }
        assert_eq!(state.visible_trigger_ids().len(), 3);
        state.trigger_filter = "wave".to_string();
        assert_eq!(state.visible_trigger_ids().len(), 2);
        state.trigger_filter = "trigger 1".to_string();
        assert_eq!(state.visible_trigger_ids(), vec![state.map.triggers[1].id]);
        let listed = state
            .controls()
            .iter()
            .filter(|control| matches!(control.kind, ControlKind::Trigger(_)))
            .count();
        assert_eq!(listed, 1);
    }

    #[test]
    fn object_categories_filter_the_palette() {
        let mut state = EditorState::new(1280.0, 800.0);
        let total = state.visible_objects().len();
        assert_eq!(state.object_category, ObjectCategory::All);

        state.activate(Action::NextObjectCategory);
        assert_eq!(state.object_category, ObjectCategory::Infantry);
        assert_eq!(state.visible_objects().len(), 3);
        state.activate(Action::NextObjectCategory);
        assert_eq!(state.object_category, ObjectCategory::Vehicles);
        assert_eq!(state.visible_objects().len(), 4);
        state.activate(Action::NextObjectCategory);
        assert_eq!(state.object_category, ObjectCategory::Aircraft);
        assert_eq!(state.visible_objects().len(), 1);
        state.activate(Action::NextObjectCategory);
        assert_eq!(state.object_category, ObjectCategory::Buildings);
        assert_eq!(
            state.visible_objects().len(),
            total - 8,
            "buildings are the remaining catalog entries"
        );
        state.activate(Action::NextObjectCategory);
        assert_eq!(state.visible_objects().len(), total);
    }

    #[test]
    fn placed_object_owner_and_class_filter_cycles() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        for (index, owner) in [None, Some(ClassId::HouseGDI), Some(ClassId::HouseNod)]
            .into_iter()
            .enumerate()
        {
            state.map.insert_object(MapObject {
                id: 0,
                class_id: ClassId::InfantryE1,
                position: Vec2i::new(index as i32, 1),
                owner,
                ..Default::default()
            });
        }
        assert_eq!(state.object_ids().len(), 3);
        state.activate(Action::NextPlacedOwnerFilter);
        assert_eq!(state.placed_filter, PlacedFilter::Unassigned);
        assert_eq!(state.object_ids().len(), 1);
        state.activate(Action::NextPlacedOwnerFilter);
        assert_eq!(state.placed_filter, PlacedFilter::Player);
        assert_eq!(state.object_ids().len(), 1);
        state.activate(Action::NextPlacedOwnerFilter);
        assert_eq!(state.placed_filter, PlacedFilter::Enemy);
        assert_eq!(state.object_ids().len(), 1);
        // The remaining stops are the classes present in the map.
        state.activate(Action::NextPlacedOwnerFilter);
        assert_eq!(
            state.placed_filter,
            PlacedFilter::Class(ClassId::InfantryE1)
        );
        assert_eq!(state.object_ids().len(), 3);
        state.activate(Action::NextPlacedOwnerFilter);
        assert_eq!(state.placed_filter, PlacedFilter::All);
        assert_eq!(state.object_ids().len(), 3);
    }

    #[test]
    fn placed_search_filters_the_object_list() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(1, 1),
            owner: None,
            ..Default::default()
        });
        state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE2,
            position: Vec2i::new(2, 2),
            owner: None,
            ..Default::default()
        });
        assert_eq!(state.object_ids().len(), 2);
        state.focus = Focus::PlacedSearch;
        state.key_text("vehic");
        assert_eq!(state.object_ids().len(), 1);
        assert_eq!(
            state.map.objects[&state.object_ids()[0]].class_id,
            ClassId::VehicleE2
        );
        state.key_text("zzz");
        assert!(state.object_ids().is_empty());
        for _ in 0..8 {
            state.key_backspace();
        }
        assert_eq!(state.object_ids().len(), 2);

        // #id and @x,y select one object; owner labels match both.
        let ids = state.object_ids();
        let first = ids[0];
        state.key_text(&format!("#{first}"));
        assert_eq!(state.object_ids(), vec![first]);
        while !state.placed_search.is_empty() {
            state.key_backspace();
        }
        state.key_text("@2,2");
        assert_eq!(state.object_ids().len(), 1);
        assert_eq!(
            state.map.objects[&state.object_ids()[0]].class_id,
            ClassId::VehicleE2
        );
        while !state.placed_search.is_empty() {
            state.key_backspace();
        }
        state.key_text("neutral");
        assert_eq!(state.object_ids().len(), 2);
        state.key_text("zzz");
        assert!(state.object_ids().is_empty());
    }

    #[test]
    fn scenario_preview_reads_terrain() {
        let dir = std::env::temp_dir().join(format!("cnc-preview-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("map.ini");
        std::fs::write(
            &path,
            "[Map]\nWidth=4\nHeight=3\n[Terrain]\n0=1 0 0 0\n1=0 0 0 0\n2=0 0 0 0\n",
        )
        .unwrap();
        let preview = scenario_preview(&path).expect("preview");
        assert_eq!(preview.width, 4);
        assert_eq!(preview.height, 3);
        assert_eq!(preview.terrain[0], 1);

        let broken = dir.join("broken.ini");
        std::fs::write(&broken, "not a scenario").unwrap();
        assert!(scenario_preview(&broken).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn scenario_browser_lists_ini_files() {
        let dir = std::env::temp_dir().join(format!("cnc-editor-files-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("b.ini"), "[Map]\nWidth=8\nHeight=8\n").unwrap();
        std::fs::write(dir.join("a.ini"), "[Map]\nWidth=8\nHeight=8\n").unwrap();
        std::fs::write(dir.join("notes.txt"), "ignore").unwrap();

        let files = scenario_files_in(&dir);
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].file_name().unwrap(), "a.ini");
        assert_eq!(files[1].file_name().unwrap(), "b.ini");

        let mut state = EditorState::new(1280.0, 800.0);
        state.scenario_files = files;
        state.inspector_tab = InspectorTab::Files;
        assert_eq!(state.inspector_tab, InspectorTab::Files);
        let controls = state.controls();
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::ScenarioFile(0)));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn object_search_filters_the_palette_and_controls() {
        let mut state = EditorState::new(1280.0, 800.0);
        let all = state.visible_objects().len();
        assert!(all >= 16);

        state.focus = Focus::ObjectSearch;
        state.key_text("infan");
        let filtered = state.visible_objects();
        assert_eq!(filtered.len(), 3);
        assert!(filtered
            .iter()
            .all(|class_id| format!("{class_id:?}").contains("Infantry")));
        let object_controls = state
            .controls()
            .iter()
            .filter(|control| matches!(control.kind, ControlKind::Object(_)))
            .count();
        assert_eq!(object_controls, 3);

        state.key_text("zzz");
        assert!(state.visible_objects().is_empty());
        for _ in 0..9 {
            state.key_backspace();
        }
        assert!(state.object_search.is_empty());
        assert_eq!(state.visible_objects().len(), all);

        // Side hints are searchable: "gdi" finds the side-locked TD objects.
        state.key_text("gdi");
        let gdi = state.visible_objects();
        assert_eq!(gdi.len(), 2, "{gdi:?}");
        assert!(gdi.contains(&ClassId::VehicleE2));
        assert!(gdi.contains(&ClassId::AircraftE1));
        for _ in 0..3 {
            state.key_backspace();
        }
        assert!(state.object_search.is_empty());
    }

    #[test]
    fn marquee_selects_every_object_inside_the_rectangle() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        let first = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(2, 2),
            owner: None,
            ..Default::default()
        });
        let second = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(3, 3),
            owner: None,
            ..Default::default()
        });
        let outside = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE2,
            position: Vec2i::new(8, 8),
            owner: None,
            ..Default::default()
        });
        state.set_select_tool();

        let (sx, sy) = state.cell_screen(Vec2i::new(1, 1));
        state.pointer_down(sx + 2.0, sy + 2.0, PointerButton::Left);
        assert!(state.marquee_active);
        let (ex, ey) = state.cell_screen(Vec2i::new(4, 4));
        state.pointer_move(ex + 2.0, ey + 2.0);
        state.pointer_up(PointerButton::Left);

        assert!(!state.marquee_active);
        assert_eq!(
            state.editor.selected_objects,
            vec![first.min(second), first.max(second)]
        );
        assert!(!state.editor.selected_objects.contains(&outside));
        assert_eq!(state.editor.selected_object, Some(first));
        assert!(state.status.contains("marquee selected 2"));
    }

    #[test]
    fn history_tab_exposes_undo_redo_and_labels() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.activate(Action::InspectorHistory);
        assert_eq!(state.inspector_tab, InspectorTab::History);
        let controls = state.controls();
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::Button(Action::Undo)));
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::Button(Action::Redo)));

        let command = Box::new(crate::history::PaintCells::terrain(&state.map, [0], 2));
        state.editor.history.execute(&mut state.map, command);
        assert_eq!(state.editor.history.undo_labels().len(), 1);
        state.undo();
        assert!(state.editor.history.undo_labels().is_empty());
        assert_eq!(state.editor.history.redo_labels().len(), 1);

        let mut renderer = SoftwareRenderer::new(1280, 800);
        state.render(&mut renderer, 1280.0, 800.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(non_black > 10_000);
    }

    #[test]
    fn copy_paste_object_is_undoable_and_rejects_occupied_cells() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        let source = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(2, 2),
            owner: Some(ClassId::HouseGDI),
            ..Default::default()
        });
        state.editor.selected_object = Some(source);

        state.copy_selected_object();
        assert!(!state.clipboard_objects.is_empty());
        assert_eq!(state.map.objects.len(), 1);

        state.cursor_cell = Vec2i::new(6, 4);
        state.paste_object();
        assert_eq!(state.map.objects.len(), 2);
        let pasted = state.map.object_at(Vec2i::new(6, 4)).unwrap();
        assert_eq!(pasted.class_id, ClassId::VehicleE1);
        assert_eq!(pasted.owner, Some(ClassId::HouseGDI));
        assert_eq!(state.editor.selected_object, Some(pasted.id));
        let pasted_id = pasted.id;

        // Occupied cells are rejected and add no undo step.
        let before = state.editor.history.undo_len();
        state.paste_object();
        assert_eq!(state.editor.history.undo_len(), before);

        state.undo();
        assert!(!state.map.objects.contains_key(&pasted_id));
        assert_eq!(state.map.objects.len(), 1);
    }

    #[test]
    fn multi_object_copy_paste_preserves_offsets_and_is_one_undo_step() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        let first = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(2, 2),
            owner: None,
            ..Default::default()
        });
        let second = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(4, 3),
            owner: Some(ClassId::HouseGDI),
            ..Default::default()
        });
        state.editor.selected_objects = vec![first, second];
        state.editor.selected_object = Some(first);

        state.copy_selected_object();
        assert_eq!(state.clipboard_objects.len(), 2);

        state.cursor_cell = Vec2i::new(9, 9);
        state.paste_object();
        assert_eq!(state.map.objects.len(), 4);
        assert!(state.map.object_at(Vec2i::new(9, 9)).is_some());
        assert!(state.map.object_at(Vec2i::new(11, 10)).is_some());
        assert_eq!(state.editor.selected_objects.len(), 2);
        assert_eq!(
            state.editor.history.undo_len(),
            1,
            "the whole paste is one undo step"
        );

        state.undo();
        assert_eq!(state.map.objects.len(), 2);
    }

    #[test]
    fn validation_tab_reports_map_issues() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(8, 8);
        assert!(!state.validation_issues().is_empty());

        state.activate(Action::InspectorValidate);
        assert_eq!(state.inspector_tab, InspectorTab::Validate);
        assert!(state
            .controls()
            .iter()
            .any(|control| control.kind == ControlKind::Button(Action::InspectorValidate)));

        let mut renderer = SoftwareRenderer::new(1280, 800);
        state.render(&mut renderer, 1280.0, 800.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(non_black > 10_000);

        state.map.player_starts.push(Vec2i::new(1, 1));
        assert!(state.validation_issues().is_empty());
        let id = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(2, 2),
            owner: Some(ClassId::InfantryE1),
            ..Default::default()
        });
        let issues = state.validation_issues();
        assert!(
            issues.iter().any(|issue| issue.contains("invalid owner")),
            "{issues:?}"
        );
        let stats = state.scenario_stats();
        assert!(stats.iter().any(|line| line.contains("objects:")));
        assert!(stats.iter().any(|line| line.contains("terrain:")));
        assert!(stats.iter().any(|line| line.contains("owners:")));
        state.map.objects.get_mut(&id).unwrap().owner = None;
        assert!(state.validation_issues().is_empty());
    }

    #[test]
    fn minimap_maps_clicks_to_cells_and_centers_the_view() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(32, 32);
        state.fit();
        let rect = state.minimap_rect();
        assert!(rect.x >= 0.0 && rect.y >= 0.0);
        assert!(rect.x + rect.w <= state.width - RIGHT_W);

        let x = rect.x + rect.w * 0.75;
        let y = rect.y + rect.h * 0.25;
        let cell = state.minimap_cell_at(x, y).unwrap();
        assert_eq!(cell, Vec2i::new(24, 8));

        let old_pan = state.pan_x;
        state.pointer_down(x, y, PointerButton::Left);
        assert!(state.status.contains("jumped"), "{}", state.status);
        assert_ne!(state.pan_x, old_pan);

        let mut renderer = SoftwareRenderer::new(1280, 800);
        state.render(&mut renderer, 1280.0, 800.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(non_black > 10_000);
    }

    #[test]
    fn object_owner_cycle_and_nudge_are_undoable() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        let id = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(5, 5),
            owner: None,
            ..Default::default()
        });
        state.editor.selected_object = Some(id);
        state.editor.selected_objects = vec![id];

        state.cycle_object_owner();
        assert_eq!(state.map.objects[&id].owner, Some(ClassId::HouseGDI));
        state.cycle_object_owner();
        assert_eq!(state.map.objects[&id].owner, Some(ClassId::HouseNod));
        state.cycle_object_owner();
        assert_eq!(state.map.objects[&id].owner, None);

        state.nudge_selected_object(1, 0);
        assert_eq!(state.map.objects[&id].position, Vec2i::new(6, 5));
        state.undo();
        assert_eq!(state.map.objects[&id].position, Vec2i::new(5, 5));

        // Blocked nudges move nothing and add no undo step.
        state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(6, 5),
            owner: None,
            ..Default::default()
        });
        let before = state.editor.history.undo_len();
        state.nudge_selected_object(1, 0);
        assert_eq!(state.map.objects[&id].position, Vec2i::new(5, 5));
        assert_eq!(state.editor.history.undo_len(), before);
    }

    #[test]
    fn trigger_copy_paste_duplicates_fields_and_is_undoable() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(8, 8);
        let id = state.map.add_trigger(Trigger {
            id: 0,
            name: "victory".to_string(),
            condition: "elapsed:60".to_string(),
            action: "win".to_string(),
            group: "Victory".to_string(),
        });
        state.inspector_tab = InspectorTab::Triggers;
        state.select_trigger(id);

        state.copy_selection();
        assert!(state.clipboard_trigger.is_some());
        state.paste_selection();
        assert_eq!(state.map.triggers.len(), 2);
        let copy = state.map.triggers.last().unwrap();
        assert_eq!(copy.name, "victory copy");
        assert_eq!(copy.condition, "elapsed:60");
        assert_eq!(copy.action, "win");
        assert_ne!(copy.id, id);
        assert_eq!(state.selected_trigger, Some(copy.id));
        state.undo();
        assert_eq!(state.map.triggers.len(), 1);
    }

    #[test]
    fn inspector_tabs_show_object_and_trigger_controls() {
        let mut state = EditorState::new(1280.0, 800.0);
        let id = state.map.insert_object(MapObject {
            id: 0,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(1, 1),
            owner: None,
            ..Default::default()
        });

        // Default tab is the trigger inspector.
        assert_eq!(state.inspector_tab, InspectorTab::Triggers);
        assert!(state
            .controls()
            .iter()
            .any(|control| control.kind == ControlKind::Field(Focus::TriggerName)));

        state.activate(Action::InspectorObjects);
        assert_eq!(state.inspector_tab, InspectorTab::Objects);
        let controls = state.controls();
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::PlacedObject(id)));
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::Button(Action::CopyObject)));
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::Button(Action::PasteObject)));
        assert!(!controls
            .iter()
            .any(|control| control.kind == ControlKind::Field(Focus::TriggerName)));

        let mut renderer = SoftwareRenderer::new(1280, 800);
        state.render(&mut renderer, 1280.0, 800.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(non_black > 10_000);
    }

    #[test]
    fn native_dialog_success_cancel_and_fallback_paths() {
        let dir = std::env::temp_dir().join(format!("cnc-editor-dialog-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // Save-as result: writes to the chosen path and syncs the path field.
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(12, 9);
        let chosen = dir.join("chosen.ini");
        state.handle_save_choice(Ok(Some(chosen.clone())));
        assert!(chosen.exists(), "{}", state.status);
        assert_eq!(state.path, chosen.display().to_string());

        // Open result: loads the chosen file.
        state.map = MapModel::new(3, 3);
        state.handle_open_choice(Ok(Some(chosen.clone())));
        assert_eq!(state.map.width, 12);
        assert_eq!(state.map.height, 9);

        // Dialog cancelled: nothing changes.
        state.handle_open_choice(Ok(None));
        assert!(state.status.contains("cancelled"));

        // Missing helper: fall back to the typed path for open and save.
        let fallback = dir.join("fallback.ini");
        let mut saved = MapModel::new(7, 5);
        saved.theater = "Snow".to_string();
        std::fs::write(&fallback, saved.to_ini()).unwrap();
        state.path = fallback.display().to_string();
        state.map = MapModel::new(2, 2);
        state.handle_open_choice(Err("missing zenity".to_string()));
        assert_eq!(state.map.width, 7);
        assert!(state.status.contains("missing zenity"), "{}", state.status);

        let fallback_out = dir.join("fallback_out.ini");
        state.path = fallback_out.display().to_string();
        state.handle_save_choice(Err("missing kdialog".to_string()));
        assert!(fallback_out.exists());
        assert!(state.status.contains("missing kdialog"), "{}", state.status);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn trigger_inspector_controls_and_frame_render() {
        let mut state = EditorState::new(1280.0, 800.0);
        let id = state.map.add_trigger(Trigger {
            id: 0,
            name: "victory".to_string(),
            condition: "always".to_string(),
            action: "win".to_string(),
            group: "Victory".to_string(),
        });
        state.select_trigger(id);
        state.focus = Focus::TriggerCondition;

        let controls = state.controls();
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::Trigger(id)));
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::Field(Focus::TriggerName)));
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::Field(Focus::TriggerCondition)));
        assert!(controls
            .iter()
            .any(|control| control.kind == ControlKind::Field(Focus::TriggerAction)));

        let mut renderer = SoftwareRenderer::new(1280, 800);
        state.render(&mut renderer, 1280.0, 800.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(
            non_black > 10_000,
            "expected a populated frame, got {non_black}"
        );
    }

    #[test]
    fn agent_studio_creates_checks_commits_and_undoes() {
        let mut state = EditorState::new(1000.0, 700.0);
        state.open_agent_studio();
        assert!(state.agent_studio);
        assert!(state.map.agents.is_empty());
        state.new_agent();
        assert_eq!(state.map.agents.len(), 1);
        assert_eq!(state.selected_agent, Some(1));

        state.focus = Focus::AgentScript;
        state.agent_edit.as_mut().unwrap().script =
            "on start { train(\"InfantryE1\") }".to_string();
        state.check_agent();
        assert!(
            state.agent_report[0].starts_with("OK"),
            "{:?}",
            state.agent_report
        );

        state.close_agent_studio();
        assert!(!state.agent_studio);
        assert!(state.map.agents[0].script.contains("InfantryE1"));
        state.undo();
        assert!(state.map.agents[0].script.contains("Balanced"));
        state.undo();
        assert!(state.map.agents.is_empty());
    }

    #[test]
    fn agent_studio_editor_accepts_typing_and_navigation() {
        let mut state = EditorState::new(1000.0, 700.0);
        state.open_agent_studio();
        state.new_agent();
        state.focus = Focus::AgentScript;
        {
            let edit = state.agent_edit.as_mut().unwrap();
            edit.script = "abc".to_string();
            edit.cursor = 0;
        }
        state.key_text("XY");
        assert_eq!(state.agent_edit.as_ref().unwrap().script, "XYabc");
        state.key_left();
        state.key_enter();
        assert_eq!(state.agent_edit.as_ref().unwrap().script, "X\nYabc");
        state.key_backspace();
        assert_eq!(state.agent_edit.as_ref().unwrap().script, "XYabc");
        state.key_end();
        state.key_text("!");
        assert_eq!(state.agent_edit.as_ref().unwrap().script, "XYabc!");
        state.key_home();
        assert_eq!(state.agent_edit.as_ref().unwrap().cursor, 0);
    }

    #[test]
    fn agent_studio_reports_compile_errors_and_dry_run() {
        let mut state = EditorState::new(1000.0, 700.0);
        state.open_agent_studio();
        state.new_agent();
        state.focus = Focus::AgentScript;
        state.agent_edit.as_mut().unwrap().script = "on start { teleport() }".to_string();
        state.check_agent();
        assert!(state
            .agent_report
            .iter()
            .any(|line| line.starts_with("error")));

        state.agent_edit.as_mut().unwrap().script =
            "every 10 { attack(nearest_enemy_x(), nearest_enemy_y()) }".to_string();
        state.run_agent();
        assert!(
            state.agent_report[0].starts_with("dry run"),
            "{:?}",
            state.agent_report
        );
        assert!(
            state
                .agent_report
                .iter()
                .any(|line| line.contains("command(s)")),
            "{:?}",
            state.agent_report
        );
    }

    #[test]
    fn agent_studio_renders_overlay_and_still_saves() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.open_agent_studio();
        state.new_agent();
        let mut renderer = SoftwareRenderer::new(1280, 800);
        state.render(&mut renderer, 1280.0, 800.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(
            non_black > 10_000,
            "expected a populated frame, got {non_black}"
        );

        let dir = std::env::temp_dir().join(format!("cnc-agent-save-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        state.path = dir.join("scenario.ini").display().to_string();
        state.save();
        assert!(dir.join("scenario.ini").exists());
        let text = std::fs::read_to_string(dir.join("scenario.ini")).unwrap();
        assert!(text.contains("[AIAgents]"));
        assert!(text.contains("Script=hex:"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn oracle_export_writes_request_prompt_and_vision_image() {
        let dir = std::env::temp_dir().join(format!("cnc-oracle-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut state = EditorState::new(1000.0, 700.0);
        state.path = dir.join("scenario.ini").display().to_string();
        state.open_agent_studio();
        state.new_agent();
        state.focus = Focus::AgentScript;
        state.agent_edit.as_mut().unwrap().script =
            "every 10 { if intent(\"push\", 0) == 1 { attack(1, 2) } }".to_string();
        state.write_agent_request();

        let request_path = dir.join("ai_oracle_request.json");
        let prompt_path = dir.join("ai_oracle_prompt.txt");
        let image_path = dir.join("ai_vision.png");
        assert!(request_path.exists(), "{:?}", state.agent_report);
        assert!(prompt_path.exists());
        assert!(image_path.exists());
        let json: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&request_path).unwrap()).unwrap();
        assert_eq!(json["intents"][0]["key"], "push");
        assert!(json["vision_image"]
            .as_str()
            .unwrap()
            .ends_with("ai_vision.png"));
        let prompt = std::fs::read_to_string(&prompt_path).unwrap();
        assert!(prompt.contains("Vision grid 32x24"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn native_dialog_defaults_follow_the_path_field() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.path = "maps/custom/scenario.ini".to_string();
        assert_eq!(state.dialog_directory(), PathBuf::from("maps/custom"));
        assert_eq!(
            state.resolve_path().file_name().unwrap(),
            std::ffi::OsStr::new("scenario.ini")
        );
    }

    #[test]
    fn editor_imports_original_maps_via_command() {
        use crate::fixtures;

        // Direct loaders are the contract.
        let td_ini = fixtures::td_scenario_text();
        let td_bin = fixtures::td_bin_bytes();
        let (direct_td, td_kind) =
            MapModel::import_text_with(&td_ini, Some(&td_bin)).expect("direct TD import");
        assert_eq!(td_kind, ImportKind::OriginalTd);

        let ra_text = fixtures::ra_scenario_text();
        let (direct_ra, ra_kind) =
            MapModel::import_text_with(&ra_text, None).expect("direct RA import");
        assert_eq!(ra_kind, ImportKind::OriginalRa);

        // Import through the editor's File→Import Original Map command.
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        let pre_import = state.map.clone();
        assert_eq!(state.editor.history.undo_len(), 0);

        // TD: ini path plus its sibling .bin.
        state.import_original_map(&td_ini, Some(&td_bin));
        assert_eq!(
            &state.map, &direct_td,
            "TD import through the command must match the direct loader"
        );
        assert_eq!(state.map.original_dialect(), Some(ImportKind::OriginalTd));
        assert_eq!(
            state.editor.history.undo_len(),
            1,
            "import is one undoable step"
        );
        assert_eq!(state.plugin_index, 0, "TD plugin is selected");

        // Ctrl+Z reverts to the pre-import state in one step.
        state.undo();
        assert_eq!(&state.map, &pre_import, "undo restores the pre-import map");
        assert!(state.map.original.is_none());

        // RA: the .mpr/.ini text alone.
        let pre_ra = state.map.clone();
        state.import_original_map(&ra_text, None);
        assert_eq!(
            &state.map, &direct_ra,
            "RA import through the command must match the direct loader"
        );
        assert_eq!(state.map.original_dialect(), Some(ImportKind::OriginalRa));
        assert_eq!(state.plugin_index, 1, "RA plugin is selected");

        state.undo();
        assert_eq!(&state.map, &pre_ra, "undo restores the pre-import map");
    }

    #[test]
    fn import_rejects_malformed_input_and_leaves_the_model_untouched() {
        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);
        state.map.player_starts.push(Vec2i::new(1, 1));
        let before = state.map.clone();
        let history_before = state.editor.history.undo_len();

        // A truncated TD .bin cannot be decoded.
        state.import_original_map("[Basic]\nName=Truncated\n", Some(&[9u8; 7]));
        assert_eq!(
            &state.map, &before,
            "failed import leaves the model untouched"
        );
        assert_eq!(state.editor.history.undo_len(), history_before);
        assert!(
            state.status.contains("import failed"),
            "the status line reports the parse failure: {}",
            state.status
        );
        // The parse failure is also listed in the validation panel.
        assert!(
            state
                .validation_issues()
                .iter()
                .any(|issue| issue.starts_with("import:")),
            "{:?}",
            state.validation_issues()
        );

        // An RA scenario without [MapPack] is not original-format.
        state.import_original_map("[Map]\nWidth=8\n[Terrain]\n0=0 0\n", None);
        assert_eq!(&state.map, &before, "model still untouched");
        assert_eq!(state.editor.history.undo_len(), history_before);
        assert!(state.status.contains("import failed"), "{}", state.status);

        // Opening a different map clears the stale import warning rather than
        // leaving it attached to a map that never had one.
        state.activate(Action::New);
        assert!(
            state.import_warnings.is_empty(),
            "{:?}",
            state.import_warnings
        );
        assert!(
            state
                .validation_issues()
                .iter()
                .all(|issue| !issue.starts_with("import:")),
            "{:?}",
            state.validation_issues()
        );
    }

    #[test]
    fn import_dialog_and_path_flow_reads_both_original_dialects() {
        use crate::fixtures;

        let dir = std::env::temp_dir().join(format!("cnc-editor-import-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // A TD scenario on disk: the .ini plus its sibling .bin.
        let td_stem = dir.join("td_map");
        std::fs::write(td_stem.with_extension("ini"), fixtures::td_scenario_text()).unwrap();
        std::fs::write(td_stem.with_extension("bin"), fixtures::td_bin_bytes()).unwrap();
        let (direct_td, _) = MapModel::import_text_with(
            &fixtures::td_scenario_text(),
            Some(&fixtures::td_bin_bytes()),
        )
        .unwrap();

        // An RA scenario on disk: the .mpr alone.
        let ra_path = dir.join("ra_map.mpr");
        std::fs::write(&ra_path, fixtures::ra_scenario_text()).unwrap();
        let (direct_ra, _) =
            MapModel::import_text_with(&fixtures::ra_scenario_text(), None).unwrap();

        let mut state = EditorState::new(1280.0, 800.0);
        state.map = MapModel::new(16, 16);

        // Dialog success picks the file and pairs the TD .bin automatically.
        state.handle_import_choice(Ok(Some(td_stem.with_extension("ini"))));
        assert_eq!(&state.map, &direct_td, "TD imported from disk");
        assert_eq!(state.map.original_dialect(), Some(ImportKind::OriginalTd));
        assert!(state.status.contains("imported"), "{}", state.status);
        assert_eq!(
            state.path,
            td_stem.with_extension("ini").display().to_string()
        );

        // RA through the same command path.
        state.handle_import_choice(Ok(Some(ra_path.clone())));
        assert_eq!(&state.map, &direct_ra, "RA imported from disk");
        assert_eq!(state.map.original_dialect(), Some(ImportKind::OriginalRa));
        assert_eq!(state.path, ra_path.display().to_string());

        // Cancelled dialog: the imported map stays put.
        state.handle_import_choice(Ok(None));
        assert_eq!(&state.map, &direct_ra, "cancel keeps the current map");
        assert!(state.status.contains("cancelled"), "{}", state.status);

        // A TD .ini with a missing sibling .bin is reported, model untouched.
        let orphan = dir.join("orphan");
        std::fs::write(orphan.with_extension("ini"), fixtures::td_scenario_text()).unwrap();
        state.handle_import_choice(Ok(Some(orphan.with_extension("ini"))));
        assert_eq!(&state.map, &direct_ra, "missing .bin leaves the map alone");
        assert!(state.status.contains("needs its .bin"), "{}", state.status);

        // Missing dialog helper: falls back to the path field.
        state.path = ra_path.display().to_string();
        let before_fallback = state.map.clone();
        let undo_before = state.editor.history.undo_len();
        state.handle_import_choice(Err("missing zenity".to_string()));
        assert!(state.status.contains("missing zenity"), "{}", state.status);
        assert_eq!(state.map.original_dialect(), Some(ImportKind::OriginalRa));
        assert_eq!(state.editor.history.undo_len(), undo_before + 1);
        state.undo();
        assert_eq!(
            &state.map, &before_fallback,
            "fallback import is undoable too"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn import_button_is_on_the_toolbar_and_renders() {
        let mut state = EditorState::new(1280.0, 800.0);
        let controls = state.controls();
        assert!(
            controls
                .iter()
                .any(|control| control.kind == ControlKind::Button(Action::ImportOriginalMap)),
            "the toolbar exposes File→Import Original Map"
        );

        let mut renderer = SoftwareRenderer::new(1280, 800);
        state.render(&mut renderer, 1280.0, 800.0);
        let non_black = renderer
            .buffer()
            .iter()
            .filter(|pixel| **pixel != 0)
            .count();
        assert!(non_black > 10_000);
    }
}
