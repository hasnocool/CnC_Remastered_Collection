//! Map model: terrain, overlay, actors, triggers and scenario INI I/O.
//!
//! The editor model is intentionally plain data. Every mutation performed by a
//! tool goes through [`crate::history`], never by writing to these fields
//! directly, so undo/redo stays consistent (see AGENTS.md).

use cnc_ai::AgentDefinition;
use cnc_core::{ClassId, IniFile, IniValue, Vec2, Vec2i};
use std::collections::HashMap;

/// Pixels per map cell, matching the engines' ICON_PIXEL_W.
pub const TILE_SIZE: f32 = 24.0;

/// One placed unit/structure.
#[derive(Clone, Debug, PartialEq)]
pub struct MapObject {
    pub id: u32,
    pub class_id: ClassId,
    pub position: Vec2i,
    pub owner: Option<ClassId>,
}

impl MapObject {
    pub fn class_name(&self) -> String {
        format!("{:?}", self.class_id)
    }
}

/// A scenario trigger authored in the editor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trigger {
    pub id: u32,
    pub name: String,
    pub condition: String,
    pub action: String,
    /// Optional folder/group shown in the trigger list.
    pub group: String,
}

/// One authored waypoint: the numeric [Waypoints] key and the cell it
/// marks.
///
/// Original scenarios store the section as "<id>=<row-major cell index>"
/// (DISPLAY.CPP, RA GamePlugin.cs Write), so the id is explicit rather than
/// positional.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Waypoint {
    pub id: u32,
    pub cell: Vec2i,
}

impl Waypoint {
    /// Display name used by tools and the history panel: A-Z for the first 26
    /// ids, then the decimal id.
    pub fn label(&self) -> String {
        if self.id < 26 {
            char::from(b'A' + self.id as u8).to_string()
        } else {
            self.id.to_string()
        }
    }
}

/// Editable scenario.
#[derive(Clone, Debug)]
pub struct MapModel {
    pub width: u32,
    pub height: u32,
    /// Scenario display name stored under `[Map] Name`.
    pub name: String,
    /// RA side string (`Allies`/`Soviet`); empty when absent.
    pub player: String,
    /// Scenario briefing text (`[Map] Briefing`); empty when absent.
    pub briefing: String,
    /// Ordered `[Objectives]` text preserved from the scenario.
    pub objectives: Vec<String>,
    pub theater: String,
    pub terrain: Vec<u8>,
    pub overlay: Vec<u8>,
    /// [Houses] entries retained from an original import.
    pub houses: Vec<HouseEntry>,
    /// Per-cell template icon index (TD binary TIcon): the sub-tile offset
    /// within the terrain template. Zero everywhere when a scenario carries
    /// no icon data.
    pub icon: Vec<u8>,
    /// Normalised elevation per cell (0-255). Zero everywhere means "flat /
    /// unpainted"; the renderer draws cliffs from adjacent differences.
    pub elevation: Vec<u8>,
    pub objects: HashMap<u32, MapObject>,
    pub triggers: Vec<Trigger>,
    /// Authored AI commanders, stored alongside the scenario.
    pub agents: Vec<AgentDefinition>,
    pub player_starts: Vec<Vec2i>,
    /// Authored waypoints, ordered by id.
    pub waypoints: Vec<Waypoint>,
    /// Original-format import metadata (sections + object origins), when this
    /// model was opened from a retail scenario. `None` for editor-authored maps.
    pub original: Option<crate::original::OriginalImport>,
    next_object_id: u32,
    next_agent_id: u32,
}

/// Decode one numbered-line base64 pack section into cell entries.
fn decode_pack_section(
    section: &cnc_core::IniSection,
) -> anyhow::Result<Vec<cnc_asset::ra_pack::PackedCell>> {
    let entries: Vec<(String, String)> = section
        .values
        .iter()
        .map(|(key, value)| (key.clone(), value.as_string().unwrap_or("").to_string()))
        .collect();
    cnc_asset::ra_pack::decode_pack(&entries)
}

/// Read a [Waypoints] section: numeric key = waypoint id, value = row-major
/// cell index. Out-of-range cells and non-numeric keys are skipped instead of
/// poisoning the model, mirroring the loaders that read DISPLAY.CPP.
fn parse_waypoints(ini: &IniFile, width: u32, height: u32) -> Vec<Waypoint> {
    if width == 0 || height == 0 {
        return Vec::new();
    }
    let Some(section) = ini.section("Waypoints") else {
        return Vec::new();
    };
    let mut waypoints = Vec::new();
    for (key, value) in &section.values {
        let Ok(id) = key.trim().parse::<u32>() else {
            continue;
        };
        let Some(index) = value_text(value).and_then(|text| text.trim().parse::<u32>().ok()) else {
            continue;
        };
        let (x, y) = (index % width, index / width);
        if x >= width || y >= height {
            continue;
        }
        waypoints.push(Waypoint {
            id,
            cell: Vec2i::new(x as i32, y as i32),
        });
    }
    waypoints.sort_by_key(|waypoint| waypoint.id);
    waypoints
}

impl MapModel {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            name: "Untitled Map".to_string(),
            player: String::new(),
            briefing: String::new(),
            objectives: Vec::new(),
            theater: "Temperate".to_string(),
            terrain: vec![0; (width * height) as usize],
            overlay: vec![0; (width * height) as usize],
            icon: vec![0; (width * height) as usize],
            houses: Vec::new(),
            elevation: vec![0; (width * height) as usize],
            objects: HashMap::new(),
            triggers: Vec::new(),
            agents: Vec::new(),
            player_starts: Vec::new(),
            waypoints: Vec::new(),
            original: None,
            next_object_id: 1,
            next_agent_id: 1,
        }
    }

    pub fn in_bounds(&self, cell: Vec2i) -> bool {
        cell.x >= 0 && cell.y >= 0 && (cell.x as u32) < self.width && (cell.y as u32) < self.height
    }

    pub fn index(&self, cell: Vec2i) -> Option<usize> {
        if self.in_bounds(cell) {
            Some((cell.y as u32 * self.width + cell.x as u32) as usize)
        } else {
            None
        }
    }

    pub fn terrain_at(&self, cell: Vec2i) -> Option<u8> {
        self.index(cell).map(|index| self.terrain[index])
    }

    pub fn overlay_at(&self, cell: Vec2i) -> Option<u8> {
        self.index(cell).map(|index| self.overlay[index])
    }

    /// Convert world pixels to a cell (floor division, no clamping).
    pub fn cell_at_world(&self, world: Vec2) -> Vec2i {
        Vec2i::new(
            (world.x.to_f32() / TILE_SIZE).floor() as i32,
            (world.y.to_f32() / TILE_SIZE).floor() as i32,
        )
    }

    /// Cell centre in world pixels.
    pub fn world_center(cell: Vec2i) -> Vec2 {
        Vec2::new(
            cnc_core::Fixed16_16::from_f32(cell.x as f32 * TILE_SIZE + TILE_SIZE * 0.5),
            cnc_core::Fixed16_16::from_f32(cell.y as f32 * TILE_SIZE + TILE_SIZE * 0.5),
        )
    }

    /// Insert an object, assigning a fresh id when `id` is zero.
    pub fn insert_object(&mut self, mut object: MapObject) -> u32 {
        if object.id == 0 {
            object.id = self.next_object_id;
            self.next_object_id += 1;
        } else {
            self.next_object_id = self.next_object_id.max(object.id + 1);
        }
        let id = object.id;
        self.objects.insert(id, object);
        id
    }

    pub fn remove_object(&mut self, id: u32) -> Option<MapObject> {
        self.objects.remove(&id)
    }

    pub fn object_at(&self, cell: Vec2i) -> Option<&MapObject> {
        self.objects.values().find(|object| object.position == cell)
    }

    pub fn add_trigger(&mut self, mut trigger: Trigger) -> u32 {
        if trigger.id == 0 {
            trigger.id = self.triggers.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        }
        let id = trigger.id;
        self.triggers.push(trigger);
        id
    }

    pub fn remove_trigger(&mut self, id: u32) -> Option<Trigger> {
        let index = self.triggers.iter().position(|trigger| trigger.id == id)?;
        Some(self.triggers.remove(index))
    }

    /// Insert an agent, assigning a fresh id when `id` is zero.
    pub fn add_agent(&mut self, mut agent: AgentDefinition) -> u32 {
        if agent.id == 0 {
            agent.id = self.next_agent_id;
        }
        self.next_agent_id = self.next_agent_id.max(agent.id + 1);
        let id = agent.id;
        self.agents.push(agent);
        id
    }

    pub fn remove_agent(&mut self, id: u32) -> Option<AgentDefinition> {
        let index = self.agents.iter().position(|agent| agent.id == id)?;
        Some(self.agents.remove(index))
    }

    pub fn agent(&self, id: u32) -> Option<&AgentDefinition> {
        self.agents.iter().find(|agent| agent.id == id)
    }

    pub fn agent_mut(&mut self, id: u32) -> Option<&mut AgentDefinition> {
        self.agents.iter_mut().find(|agent| agent.id == id)
    }

    /// Waypoint with the given id (the key written under [Waypoints]).
    pub fn waypoint(&self, id: u32) -> Option<&Waypoint> {
        self.waypoints.iter().find(|waypoint| waypoint.id == id)
    }

    /// Waypoint marking the given cell, if any.
    pub fn waypoint_at(&self, cell: Vec2i) -> Option<&Waypoint> {
        self.waypoints.iter().find(|waypoint| waypoint.cell == cell)
    }

    /// Lowest id not used by an existing waypoint, so auto-assignment stays
    /// stable across save/load rather than tracking a counter.
    pub fn next_waypoint_id(&self) -> u32 {
        let mut id = 0;
        while self.waypoint(id).is_some() {
            id += 1;
        }
        id
    }

    /// Insert or replace a waypoint, keeping the list ordered by id so a load
    /// compares equal to what was saved.
    pub fn insert_waypoint(&mut self, waypoint: Waypoint) {
        if let Some(slot) = self
            .waypoints
            .iter_mut()
            .find(|existing| existing.id == waypoint.id)
        {
            *slot = waypoint;
        } else {
            self.waypoints.push(waypoint);
            self.waypoints.sort_by_key(|waypoint| waypoint.id);
        }
    }

    /// Remove the waypoint with the given id, returning it for an exact undo.
    pub fn remove_waypoint(&mut self, id: u32) -> Option<Waypoint> {
        let index = self
            .waypoints
            .iter()
            .position(|waypoint| waypoint.id == id)?;
        Some(self.waypoints.remove(index))
    }

    /// Structural problems that block saving.
    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();
        if self.width == 0 || self.height == 0 {
            issues.push("map has no cells".to_string());
        }
        if self.player_starts.is_empty() {
            issues.push("map has no player start".to_string());
        }
        for start in &self.player_starts {
            if !self.in_bounds(*start) {
                issues.push(format!(
                    "player start ({}, {}) is out of bounds",
                    start.x, start.y
                ));
            }
        }
        let mut waypoint_ids = std::collections::BTreeSet::new();
        for waypoint in &self.waypoints {
            if !waypoint_ids.insert(waypoint.id) {
                issues.push(format!("duplicate waypoint id {}", waypoint.id));
            }
            if !self.in_bounds(waypoint.cell) {
                issues.push(format!(
                    "waypoint {} at ({}, {}) is out of bounds",
                    waypoint.label(),
                    waypoint.cell.x,
                    waypoint.cell.y
                ));
            }
        }
        for object in self.objects.values() {
            if !self.in_bounds(object.position) {
                issues.push(format!(
                    "object {} at ({}, {}) is out of bounds",
                    object.id, object.position.x, object.position.y
                ));
            }
        }
        let mut seen = std::collections::BTreeSet::new();
        if self.elevation.len() != (self.width * self.height) as usize {
            issues.push("elevation grid does not match the map size".to_string());
        }
        for trigger in &self.triggers {
            if !seen.insert(trigger.id) {
                issues.push(format!("duplicate trigger id {}", trigger.id));
            }
            if trigger.name.trim().is_empty() {
                issues.push(format!("trigger {} has no name", trigger.id));
            }
        }
        let mut agent_ids = std::collections::BTreeSet::new();
        let mut owners: std::collections::BTreeMap<String, String> =
            std::collections::BTreeMap::new();
        for agent in &self.agents {
            if !agent_ids.insert(agent.id) {
                issues.push(format!("duplicate agent id {}", agent.id));
            }
            if !agent.owner.trim().is_empty() {
                match parse_class_id(&agent.owner) {
                    Some(class_id) if class_id.is_house() => {
                        if let Some(other) = owners.insert(agent.owner.clone(), agent.name.clone())
                        {
                            issues.push(format!(
                                "agents {} and {} share owner {}",
                                other, agent.name, agent.owner
                            ));
                        }
                    }
                    _ => issues.push(format!(
                        "agent {} has unknown owner house {}",
                        agent.name, agent.owner
                    )),
                }
            }
            issues.extend(agent.validate(&[]));
        }
        issues
    }

    pub fn summary(&self) -> String {
        format!(
            "{}x{} {} - {} objects, {} triggers, {} agents, {} starts",
            self.width,
            self.height,
            self.theater,
            self.objects.len(),
            self.triggers.len(),
            self.agents.len(),
            self.player_starts.len()
        )
    }

    /// Serialize to the scenario INI dialect used by both engines.
    pub fn to_ini(&self) -> String {
        let mut out = String::new();
        out.push_str("[Map]\n");
        out.push_str(&format!("Width={}\nHeight={}\n", self.width, self.height));
        out.push_str(&format!("Theater={}\n", self.theater));
        out.push_str(&format!("Name={}\n", self.name));
        if !self.player.is_empty() {
            out.push_str(&format!("Player={}\n", self.player));
        }
        if !self.briefing.is_empty() {
            out.push_str(&format!("Briefing={}\n", self.briefing));
        }
        out.push_str("[PlayerStarts]\n");
        for (index, start) in self.player_starts.iter().enumerate() {
            out.push_str(&format!("{}=({},{})\n", index, start.x, start.y));
        }
        // Omitted entirely when the scenario has no waypoints, matching the
        // original writer (RA GamePlugin.cs only emits cells that are set).
        if !self.waypoints.is_empty() {
            out.push_str("[Waypoints]\n");
            let mut waypoints = self.waypoints.clone();
            waypoints.sort_by_key(|waypoint| waypoint.id);
            for waypoint in waypoints {
                let index = waypoint.cell.y as u32 * self.width + waypoint.cell.x as u32;
                out.push_str(&format!("{}={}\n", waypoint.id, index));
            }
        }
        out.push_str("[Terrain]\n");
        for y in 0..self.height {
            let row: Vec<String> = (0..self.width)
                .map(|x| self.terrain[(y * self.width + x) as usize].to_string())
                .collect();
            out.push_str(&format!("{}={}\n", y, row.join(" ")));
        }
        out.push_str("[Overlay]\n");
        for y in 0..self.height {
            let row: Vec<String> = (0..self.width)
                .map(|x| self.overlay[(y * self.width + x) as usize].to_string())
                .collect();
            out.push_str(&format!("{}={}\n", y, row.join(" ")));
        }
        // Only persisted when painted, so exports of unpainted maps stay
        // byte-compatible with older releases.
        if self.elevation.iter().any(|level| *level != 0) {
            out.push_str("[Elevation]\n");
            for y in 0..self.height {
                let row: Vec<String> = (0..self.width)
                    .map(|x| self.elevation[(y * self.width + x) as usize].to_string())
                    .collect();
                out.push_str(&format!("{}={}\n", y, row.join(" ")));
            }
        }
        out.push_str("[Objects]\n");
        let mut objects: Vec<&MapObject> = self.objects.values().collect();
        objects.sort_by_key(|object| object.id);
        for object in objects {
            let owner = object
                .owner
                .map(|owner| format!("{owner:?}"))
                .unwrap_or_else(|| "None".to_string());
            out.push_str(&format!(
                "{}={:?},{},{},{}\n",
                object.id, object.class_id, object.position.x, object.position.y, owner
            ));
        }
        if !self.objectives.is_empty() {
            out.push_str("[Objectives]\n");
            for (index, objective) in self.objectives.iter().enumerate() {
                out.push_str(&format!("{index}={objective}\n"));
            }
        }
        out.push_str("[Triggers]\n");
        let mut triggers = self.triggers.clone();
        triggers.sort_by_key(|trigger| trigger.id);
        for trigger in triggers {
            out.push_str(&format!(
                "{}={}|{}|{}|{}\n",
                trigger.id, trigger.name, trigger.condition, trigger.action, trigger.group
            ));
        }
        out.push_str("[AIAgents]\n");
        out.push_str(&format!("Count={}\n", self.agents.len()));
        let mut agents = self.agents.clone();
        agents.sort_by_key(|agent| agent.id);
        for (index, agent) in agents.iter().enumerate() {
            out.push_str(&format!("\n[AIAgent{}]\n", index + 1));
            out.push_str(&format!("Id={}\n", agent.id));
            out.push_str(&format!("Name={}\n", encode_ini_text(&agent.name)));
            out.push_str(&format!("Owner={}\n", encode_ini_text(&agent.owner)));
            out.push_str(&format!("Doctrine={}\n", encode_ini_text(&agent.doctrine)));
            out.push_str(&format!("Script={}\n", encode_ini_text(&agent.script)));
        }
        out
    }

    /// Import any supported scenario text, picking the reader by dialect.
    ///
    /// Tries the original RA dialect (`[Basic]` + `[MapPack]`), then the
    /// editor dialect (`[Map]`), so an `open` command can accept either
    /// without the user choosing. Returns the model and which reader ran.
    pub fn import_text_with(input: &str, bin: Option<&[u8]>) -> anyhow::Result<(Self, ImportKind)> {
        if looks_like_original_ra(input) {
            return Ok((Self::from_original_ra(input)?, ImportKind::OriginalRa));
        }
        if looks_like_original_td(input) {
            // A TD scenario is useless without its cell image; fail with a
            // clear message rather than a confusing dialect error.
            return Self::import_td(input, bin);
        }
        Ok((Self::from_ini(input)?, ImportKind::EditorDialect))
    }

    /// Import text with no sibling map image.
    pub fn import_text(input: &str) -> anyhow::Result<(Self, ImportKind)> {
        Self::import_text_with(input, None)
    }

    /// Import an original TD scenario, reading the sibling `.bin` if given.
    pub fn import_td(ini: &str, bin: Option<&[u8]>) -> anyhow::Result<(Self, ImportKind)> {
        let Some(bin) = bin else {
            anyhow::bail!("an original TD scenario needs its .bin map image");
        };
        Ok((Self::from_original_td(ini, bin)?, ImportKind::OriginalTd))
    }

    /// Import an **original-format** RA scenario (`.mpr`/`.ini`) into the model.
    ///
    /// Original files store the cell grid in base64 `MapPack`/`OverlayPack`
    /// blocks rather than the editor dialect's `[Terrain]`/`[Overlay]` text
    /// rows (`REDALERT/DISPLAY.CPP`, `OVERLAY.CPP`). Each decoded entry is
    /// `(u16 little-endian cell index, u8 id)`; the id becomes that cell's
    /// terrain template or overlay id.
    ///
    /// Returns an error when the text is not original-format, so callers can
    /// fall back to [`Self::from_ini`].
    pub fn from_original_ra(input: &str) -> anyhow::Result<Self> {
        let ini = IniFile::parse(input)?;
        let basic = ini
            .section("Basic")
            .ok_or_else(|| anyhow::anyhow!("original RA scenario needs a [Basic] section"))?;
        let map_pack = ini
            .section("MapPack")
            .ok_or_else(|| anyhow::anyhow!("original RA scenario needs a [MapPack] section"))?;

        let map_cells = decode_pack_section(map_pack)?;
        let overlay_cells = match ini.section("OverlayPack") {
            Some(section) => decode_pack_section(section)?,
            None => Vec::new(),
        };

        // The original format carries no dimensions; infer the smallest square
        // that contains every referenced cell index.
        let highest = map_cells
            .iter()
            .map(|cell| u32::from(cell.index))
            .chain(overlay_cells.iter().map(|cell| u32::from(cell.index)))
            .max()
            .unwrap_or(0);
        let side = ((highest + 1) as f64).sqrt().ceil().max(1.0) as u32;

        let mut model = Self::new(side, side);
        model.name = basic
            .get_string("Name")
            .unwrap_or("Imported Scenario")
            .to_string();
        // SCENARIO.CPP:2392 defaults the player house to Greece.
        model.player = basic
            .values
            .get("Player")
            .map(ini_value_text)
            .unwrap_or_default();
        model.briefing = basic.get_string("Briefing").unwrap_or("").to_string();
        model.theater = "Temperate".to_string();

        for cell in &map_cells {
            if let Some(slot) = model.terrain.get_mut(cell.index as usize) {
                *slot = cell.id;
            }
        }
        model.waypoints = parse_waypoints(&ini, side, side);
        for cell in &overlay_cells {
            if let Some(slot) = model.overlay.get_mut(cell.index as usize) {
                *slot = cell.id;
            }
        }
        // Object sections: Units/Infantry/Structures, record layout per
        // REDALERT/UNIT.CPP and INFANTRY.CPP:
        //   House,Type,Strength,Cell[,extra...]
        let captured_sections = crate::original::capture_sections(input);
        let mut object_origins = Vec::new();
        let mut object_id = 0u32;
        for section_name in ["Units", "Infantry", "Structures"] {
            let Some(section) = ini.section(section_name) else {
                continue;
            };
            for (key, value) in &section.values {
                let text = ini_value_text(value);
                let fields: Vec<&str> = text.split(',').map(str::trim).collect();
                if fields.len() < 4 {
                    continue;
                }
                // RA rules codes (HTNK, E1, POWR) resolve through the same
                // shared vocabulary the engines use.
                let class_id =
                    cnc_core::class_id_from_token(fields[1]).unwrap_or(cnc_core::ClassId::Unknown);
                let Ok(cell) = fields[3].parse::<u32>() else {
                    continue;
                };
                if cell >= side * side {
                    continue;
                }
                let (x, y) = ((cell % side) as i32, (cell / side) as i32);
                object_id += 1;
                let owner = cnc_core::house_class_from_token(fields[0]).ok();
                object_origins.push(crate::original::ObjectOrigin {
                    section: section_name.to_string(),
                    key: key.clone(),
                    object_id,
                    cell,
                    owner,
                    fields: fields.iter().map(|s| s.to_string()).collect(),
                    named_section: None,
                });
                model.insert_object(MapObject {
                    id: object_id,
                    class_id,
                    position: Vec2i::new(x, y),
                    owner,
                });
            }
        }
        model.houses = parse_house_entries(&ini);
        model.original = Some(crate::original::OriginalImport {
            dialect: ImportKind::OriginalRa,
            sections: captured_sections,
            objects: object_origins,
        });
        Ok(model)
    }
    /// Import an **original-format** TD scenario (`.ini` + `.bin`).
    ///
    /// The original TD map image is one 2-byte record per cell
    /// (`TemplateType`, `TIcon`) - raw, or LCW-compressed when the payload is
    /// shorter than `side * side * 2` (`TIBERIANDAWN/DISPLAY.CPP`). The
    /// template byte becomes the cell terrain id.
    ///
    /// Returns an error when the text is not original-format, so callers can
    /// fall back to [`Self::from_ini`].
    pub fn from_original_td(input: &str, bin: &[u8]) -> anyhow::Result<Self> {
        let ini = IniFile::parse(input)?;
        ini.section("Basic")
            .ok_or_else(|| anyhow::anyhow!("original TD scenario needs a [Basic] section"))?;

        let (cells, icons, side) = decode_td_bin(bin)?;
        let mut model = Self::new(side, side);
        let basic = ini.section("Basic").expect("checked above");
        model.name = basic
            .get_string("Name")
            .unwrap_or("Imported Scenario")
            .to_string();
        model.player = basic
            .values
            .get("Player")
            .map(ini_value_text)
            .unwrap_or_default();
        model.briefing = basic.get_string("Briefing").unwrap_or("").to_string();
        model.theater = basic
            .get_string("Theater")
            .unwrap_or("Temperate")
            .to_string();

        for (slot, template) in model.terrain.iter_mut().zip(&cells) {
            *slot = *template;
        }
        for (slot, icon) in model.icon.iter_mut().zip(icons) {
            *slot = icon;
        }
        // Object sections: [Wunits] (units) and [Structures]. Each entry
        // references a named section carrying House/Type/X/Y.
        let captured_sections = crate::original::capture_sections(input);
        let mut object_origins = Vec::new();
        // House ordinals in the object sections index the [Houses] list.
        let house_names: Vec<String> = ini
            .section("Houses")
            .map(|section| {
                section
                    .values
                    .values()
                    .map(|value| ini_value_text(value).trim().to_string())
                    .collect()
            })
            .unwrap_or_default();
        let mut object_id = 0u32;
        for section_name in ["Wunits", "Structures"] {
            let Some(section) = ini.section(section_name) else {
                continue;
            };
            for (key, value) in &section.values {
                let raw = ini_value_text(value).trim().to_string();
                // Named-section form (retail TD): value references a section.
                if let Some(obj_section) = ini.section(&raw) {
                    let type_token = obj_section
                        .values
                        .get("Type")
                        .map(ini_value_text)
                        .unwrap_or_default();
                    let class_id = cnc_core::class_id_from_token(&type_token)
                        .unwrap_or(cnc_core::ClassId::Unknown);
                    let x: i32 = obj_section
                        .values
                        .get("X")
                        .map(ini_value_text)
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0);
                    let y: i32 = obj_section
                        .values
                        .get("Y")
                        .map(ini_value_text)
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0);
                    if x < 0 || y < 0 || (x as u32) >= side || (y as u32) >= side {
                        continue;
                    }
                    let cell = y as u32 * side + x as u32;
                    let owner = obj_section
                        .values
                        .get("House")
                        .map(ini_value_text)
                        .and_then(|v| v.parse::<usize>().ok())
                        .and_then(|index| house_names.get(index))
                        .and_then(|name| cnc_core::house_class_from_token(name).ok());
                    object_id += 1;
                    object_origins.push(crate::original::ObjectOrigin {
                        section: section_name.to_string(),
                        key: key.clone(),
                        object_id,
                        cell,
                        owner,
                        fields: Vec::new(),
                        named_section: Some(raw.clone()),
                    });
                    model.insert_object(MapObject {
                        id: object_id,
                        class_id,
                        position: Vec2i::new(x, y),
                        owner,
                    });
                    continue;
                }

                // Comma-record form (synthetic/tests): House,Type,Strength,Cell,...
                let fields: Vec<&str> = raw.split(',').map(str::trim).collect();
                if fields.len() < 4 {
                    continue;
                }
                let class_id =
                    cnc_core::class_id_from_token(fields[1]).unwrap_or(cnc_core::ClassId::Unknown);
                let Ok(cell) = fields[3].parse::<u32>() else {
                    continue;
                };
                if cell >= side * side {
                    continue;
                }
                let (x, y) = ((cell % side) as i32, (cell / side) as i32);
                let owner = cnc_core::house_class_from_token(fields[0]).ok();
                object_id += 1;
                object_origins.push(crate::original::ObjectOrigin {
                    section: section_name.to_string(),
                    key: key.clone(),
                    object_id,
                    cell,
                    owner,
                    fields: Vec::new(),
                    named_section: None,
                });
                model.insert_object(MapObject {
                    id: object_id,
                    class_id,
                    position: Vec2i::new(x, y),
                    owner,
                });
            }
        }
        // [Trigs] lists `<ordinal>=<section>`; each section carries
        // `ConditionN=`/`ActionN=` slots (SCENARIO.CPP trigger dialect).
        // The editor keeps the first condition/action verbatim so an
        // imported scenario is inspectable even though the dialects differ.
        if let Some(list) = ini.section("Trigs") {
            let mut trigger_id = 0u32;
            // The list may be keyed by bare ordinal ("1=TRIGA") or by
            // "TriggerN"; accept either by walking every value.
            for value in list.values.values() {
                let Some(section_name) = value.as_string() else {
                    continue;
                };
                let section_name = section_name.trim().to_string();
                if section_name.is_empty() {
                    continue;
                }
                let Some(section) = ini.section(&section_name) else {
                    continue;
                };
                let join = |prefix: &str| -> String {
                    let mut parts = Vec::new();
                    for slot in 1..=20 {
                        if let Some(text) = get_raw_text(section, &format!("{prefix}{slot}")) {
                            if !text.is_empty() {
                                parts.push(text);
                            }
                        }
                    }
                    parts.join("; ")
                };
                let condition = join("Condition");
                let action = join("Action");
                if condition.is_empty() && action.is_empty() {
                    continue;
                }
                trigger_id += 1;
                model.add_trigger(Trigger {
                    id: trigger_id,
                    name: section_name.to_string(),
                    condition,
                    action,
                    group: "Imported".to_string(),
                });
            }
        }
        // [Houses]: see parse_house_entries.
        model.waypoints = parse_waypoints(&ini, side, side);
        model.houses = parse_house_entries(&ini);
        model.original = Some(crate::original::OriginalImport {
            dialect: ImportKind::OriginalTd,
            sections: captured_sections,
            objects: object_origins,
        });
        Ok(model)
    }

    /// Parse a scenario produced by [`Self::to_ini`].
    pub fn from_ini(input: &str) -> anyhow::Result<Self> {
        /// Which grid a scenario section fills.
        enum Target {
            Terrain,
            Overlay,
            Elevation,
        }
        let ini = IniFile::parse(input)?;
        let map = ini.sections.get("Map");
        let width = map
            .and_then(|section| section.get_i64("Width").ok())
            .unwrap_or(0) as u32;
        let height = map
            .and_then(|section| section.get_i64("Height").ok())
            .unwrap_or(0) as u32;
        let mut model = Self::new(width, height);
        if let Some(theater) = map.and_then(|section| section.get_string("Theater")) {
            model.theater = theater.to_string();
        }
        if let Some(name) = map.and_then(|section| section.get_string("Name")) {
            model.name = name.to_string();
        }
        if let Some(player) = map.and_then(|section| section.get_string("Player")) {
            model.player = player.to_string();
        }
        if let Some(briefing) = map.and_then(|section| section.get_string("Briefing")) {
            model.briefing = briefing.to_string();
        }
        if let Some(objectives) = ini.sections.get("Objectives") {
            let mut entries: Vec<(u32, String)> = objectives
                .values
                .iter()
                .filter_map(|(key, value)| Some((key.parse::<u32>().ok()?, value_text(value)?)))
                .collect();
            entries.sort_by_key(|(id, _)| *id);
            model.objectives = entries.into_iter().map(|(_, text)| text).collect();
        }
        if let Some(starts) = ini.sections.get("PlayerStarts") {
            for value in starts.values.values() {
                if let Some(text) = value_text(value) {
                    if let Some(cell) = parse_point(&text) {
                        model.player_starts.push(cell);
                    }
                }
            }
        }
        model.waypoints = parse_waypoints(&ini, width, height);
        for (section_name, target) in [
            ("Terrain", Target::Terrain),
            ("Overlay", Target::Overlay),
            ("Elevation", Target::Elevation),
        ] {
            if let Some(section) = ini.sections.get(section_name) {
                for (row_key, value) in &section.values {
                    let Ok(y) = row_key.parse::<u32>() else {
                        continue;
                    };
                    let Some(text) = value_text(value) else {
                        continue;
                    };
                    let cells = text
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(|c: char| c == ',' || c.is_whitespace())
                        .filter_map(|part| part.trim().parse::<i64>().ok())
                        .collect::<Vec<_>>();
                    for (x, cell) in cells.into_iter().enumerate() {
                        if x as u32 >= model.width || y >= model.height {
                            continue;
                        }
                        let index = (y * model.width + x as u32) as usize;
                        match target {
                            Target::Terrain => {
                                model.terrain[index] = cell.clamp(0, 255) as u8;
                            }
                            Target::Overlay => {
                                model.overlay[index] = cell.clamp(0, 255) as u8;
                            }
                            Target::Elevation => {
                                model.elevation[index] = cell.clamp(0, 255) as u8;
                            }
                        }
                    }
                }
            }
        }
        if let Some(section) = ini.sections.get("Objects") {
            for (key, value) in &section.values {
                let Ok(id) = key.parse::<u32>() else { continue };
                let Some(text) = value_text(value) else {
                    continue;
                };
                let parts: Vec<&str> = text.split(',').map(str::trim).collect();
                if parts.len() < 3 {
                    continue;
                }
                let Some(class_id) = parse_class_id(parts[0]) else {
                    continue;
                };
                let position =
                    Vec2i::new(parts[1].parse().unwrap_or(0), parts[2].parse().unwrap_or(0));
                let owner = parts
                    .get(3)
                    .filter(|token| !token.eq_ignore_ascii_case("none"))
                    .and_then(|token| parse_class_id(token));
                model.insert_object(MapObject {
                    id,
                    class_id,
                    position,
                    owner,
                });
            }
        }
        if let Some(section) = ini.sections.get("AIAgents") {
            let count = section.get_i64("Count").unwrap_or(0).clamp(0, 1_000) as usize;
            for index in 1..=count {
                let Some(agent_section) = ini.sections.get(&format!("AIAgent{index}")) else {
                    continue;
                };
                let id = agent_section
                    .get_i64("Id")
                    .ok()
                    .filter(|id| *id > 0 && *id <= i64::from(u32::MAX))
                    .map_or(index as u32, |id| id as u32);
                let name = agent_section
                    .get_string("Name")
                    .and_then(decode_ini_text)
                    .unwrap_or_else(|| format!("Agent {id}"));
                let mut agent = AgentDefinition::new(id, name);
                if let Some(owner) = agent_section.get_string("Owner").and_then(decode_ini_text) {
                    agent.owner = owner;
                }
                if let Some(doctrine) = agent_section
                    .get_string("Doctrine")
                    .and_then(decode_ini_text)
                {
                    agent.doctrine = doctrine;
                }
                if let Some(script) = agent_section.get_string("Script").and_then(decode_ini_text) {
                    agent.script = script;
                }
                model.add_agent(agent);
            }
        }
        if let Some(section) = ini.sections.get("Triggers") {
            for (key, value) in &section.values {
                let Ok(id) = key.parse::<u32>() else { continue };
                let Some(text) = value_text(value) else {
                    continue;
                };
                let parts: Vec<&str> = text.split('|').map(str::trim).collect();
                if parts.len() < 3 {
                    continue;
                }
                model.add_trigger(Trigger {
                    id,
                    name: parts[0].to_string(),
                    condition: parts[1].to_string(),
                    action: parts[2].to_string(),
                    group: parts
                        .get(3)
                        .map_or(String::new(), |group| group.to_string()),
                });
            }
        }
        Ok(model)
    }
}

/// Encode text for a single INI value. The dialect splits values on `;` and
/// keeps a line per key, so scripts (which contain both) are stored as hex.
fn encode_ini_text(text: &str) -> String {
    if text.contains(';') || text.contains('\n') || text.contains('\r') {
        format!("hex:{}", hex_encode(text.as_bytes()))
    } else {
        text.to_string()
    }
}

fn decode_ini_text(text: &str) -> Option<String> {
    if let Some(encoded) = text.strip_prefix("hex:") {
        let bytes = hex_decode(encoded)?;
        String::from_utf8(bytes).ok()
    } else {
        Some(text.to_string())
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(DIGITS[(byte >> 4) as usize] as char);
        out.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    out
}

fn hex_decode(text: &str) -> Option<Vec<u8>> {
    if !text.len().is_multiple_of(2) {
        return None;
    }
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    let nibble = |byte: u8| -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    };
    for pair in bytes.as_chunks::<2>().0 {
        out.push((nibble(pair[0])? << 4) | nibble(pair[1])?);
    }
    Some(out)
}

/// Flatten a string/integer/array INI value into comma-joined text.
fn value_text(value: &IniValue) -> Option<String> {
    match value {
        IniValue::String(text) => Some(text.clone()),
        IniValue::Integer(number) => Some(number.to_string()),
        IniValue::Float(number) => Some(number.to_string()),
        IniValue::Boolean(flag) => Some(flag.to_string()),
        IniValue::Array(values) => Some(
            values
                .iter()
                .filter_map(value_text)
                .collect::<Vec<_>>()
                .join(","),
        ),
    }
}

fn parse_point(text: &str) -> Option<Vec2i> {
    let text = text.trim().trim_matches(|c| c == '(' || c == ')');
    let mut parts = text.split(',');
    Some(Vec2i::new(
        parts.next()?.trim().parse().ok()?,
        parts.next()?.trim().parse().ok()?,
    ))
}

/// Resolve a numeric id or a `Debug` class name such as `ConstructionYard`.
pub(crate) fn parse_class_id(text: &str) -> Option<ClassId> {
    let text = text.trim().trim_matches('"').trim();
    if let Ok(value) = text.parse::<u32>() {
        return Some(ClassId::from_u32(value));
    }
    let canonical = match text.to_ascii_lowercase().as_str() {
        "infantrye1" => ClassId::InfantryE1,
        "infantrye2" => ClassId::InfantryE2,
        "infantrye3" => ClassId::InfantryE3,
        "infantrye8" => ClassId::InfantryE8,
        "infantrye9" => ClassId::InfantryE9,
        "vehiclee1" => ClassId::VehicleE1,
        "vehiclee2" => ClassId::VehicleE2,
        "vehiclee3" => ClassId::VehicleE3,
        "vehiclee7" => ClassId::VehicleE7,
        "aircrafte1" => ClassId::AircraftE1,
        "aircrafte2" => ClassId::AircraftE2,
        "shipe1" => ClassId::ShipE1,
        "shipe2" => ClassId::ShipE2,
        "shipe4" => ClassId::ShipE4,
        "constructionyard" => ClassId::ConstructionYard,
        "powerplant" => ClassId::PowerPlant,
        "refinery" => ClassId::Refinery,
        "barracks" => ClassId::Barracks,
        "warfactory" => ClassId::WarFactory,
        "helipad" => ClassId::Helipad,
        "airfield" => ClassId::Airfield,
        "radar" => ClassId::Radar,
        "teslacoil" => ClassId::TeslaCoil,
        "gapgenerator" => ClassId::GapGenerator,
        "chronosphere" => ClassId::Chronosphere,
        "ironcurtain" => ClassId::IronCurtain,
        "housegdi" => ClassId::HouseGDI,
        "housenod" => ClassId::HouseNod,
        "houseallies" => ClassId::HouseAllies,
        "housesoviet" => ClassId::HouseSoviet,
        _ => return None,
    };
    Some(canonical)
}

/// One `[Houses]` entry retained from an original import.
///
/// TD (`HOUSE.CPP`) and RA both name houses by section and read per-house
/// economy settings; RA additionally carries TechLevel/IQ/MaxInfantry/
/// MaxVessel. Fields a game does not define stay at their defaults.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseEntry {
    pub name: String,
    pub credits: i64,
    pub max_units: i64,
    pub max_buildings: i64,
    pub edge: String,
    /// RA `TechLevel=` (0 when absent).
    pub tech_level: i64,
    /// RA `MaxInfantry=`.
    pub max_infantry: i64,
    /// RA `MaxVessel=`.
    pub max_vessel: i64,
}

/// Which reader produced a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportKind {
    /// The editor's own `[Map]` dialect.
    EditorDialect,
    /// Original RA: `[Basic]` + base64 `MapPack`/`OverlayPack`.
    OriginalRa,
    /// Original TD: `[Basic]` + binary `.bin` map image.
    OriginalTd,
}

impl ImportKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::EditorDialect => "editor dialect",
            Self::OriginalRa => "original RA",
            Self::OriginalTd => "original TD",
        }
    }
}

/// Read every `[Houses]` entry: an ordinal list of house section names plus
/// each house section's economy settings (HOUSE.CPP, both games).
fn parse_house_entries(ini: &IniFile) -> Vec<HouseEntry> {
    let Some(list) = ini.section("Houses") else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    for value in list.values.values() {
        let name = ini_value_text(value).trim().to_string();
        if name.is_empty() {
            continue;
        }
        let section = ini.section(&name);
        let int =
            |key: &str, default: i64| section.and_then(|s| s.get_i64(key).ok()).unwrap_or(default);
        entries.push(HouseEntry {
            name,
            // Both engines store Credits * 100.
            credits: int("Credits", 0).saturating_mul(100),
            max_units: int("MaxUnit", 150),
            max_buildings: int("MaxBuilding", 150),
            edge: section
                .and_then(|s| s.get_string("Edge"))
                .unwrap_or("")
                .to_string(),
            tech_level: int("TechLevel", 0),
            max_infantry: int("MaxInfantry", 0),
            max_vessel: int("MaxVessel", 0),
        });
    }
    entries
}
/// Read a section value as plain text (arrays rejoined with commas).
fn get_raw_text(section: &cnc_core::IniSection, key: &str) -> Option<String> {
    section.values.get(key).map(ini_value_text)
}
/// Render an IniValue back to the text form the record parsers split on.
fn ini_value_text(value: &cnc_core::IniValue) -> String {
    match value {
        cnc_core::IniValue::String(s) => s.clone(),
        cnc_core::IniValue::Integer(i) => i.to_string(),
        cnc_core::IniValue::Float(f) => f.to_string(),
        cnc_core::IniValue::Boolean(b) => if *b { "1" } else { "0" }.to_string(),
        cnc_core::IniValue::Array(parts) => parts
            .iter()
            .map(ini_value_text)
            .collect::<Vec<_>>()
            .join(","),
    }
}
/// True when the text looks like an original-format TD scenario
/// (TIBERIANDAWN/SCENARIO.CPP: [Basic] plus object sections).
fn looks_like_original_td(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("[basic]")
        && (lower.contains("[wunits]")
            || lower.contains("[structures]")
            || lower.contains("[trigs]"))
}

/// True when the text looks like an original-format RA scenario.
fn looks_like_original_ra(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("[basic]") && lower.contains("[mappack]")
}
/// Decode a TD binary map image into (terrain template per cell, side length).
///
/// Uncompressed images are exactly `side * side * 2` bytes; anything else is
/// treated as LCW-compressed (`cnc_asset::format80`) and inflated to the next
/// square size that fits (`TIBERIANDAWN/DISPLAY.CPP`).
fn decode_td_bin(raw: &[u8]) -> anyhow::Result<(Vec<u8>, Vec<u8>, u32)> {
    if raw.is_empty() {
        anyhow::bail!("TD map image is empty");
    }
    const RECORD: usize = 2;
    // Exact square: plain uncompressed image.
    if raw.len().is_multiple_of(RECORD) {
        let total = raw.len() / RECORD;
        let root = (total as f64).sqrt() as u32;
        if root > 0 && (root as usize) * (root as usize) == total {
            return Ok((td_byte(raw, root, 0), td_byte(raw, root, 1), root));
        }
    }
    // Otherwise LCW: find the smallest square that inflates exactly.
    for side in 8..=512u32 {
        let want = (side as usize) * (side as usize) * RECORD;
        if let Ok(inflated) = cnc_asset::format80::decompress(raw, want) {
            if inflated.len() == want {
                return Ok((
                    td_byte(&inflated, side, 0),
                    td_byte(&inflated, side, 1),
                    side,
                ));
            }
        }
    }
    anyhow::bail!("TD map image is neither a square payload nor decodable LCW")
}

/// Take the first byte of every 2-byte cell record (the template id).
fn td_byte(cells: &[u8], side: u32, index: usize) -> Vec<u8> {
    cells
        .as_chunks::<2>()
        .0
        .iter()
        .take((side * side) as usize)
        .map(|record| record[index])
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::{OverlayType, TerrainType};

    #[test]
    fn inserts_objects_with_unique_ids_and_finds_them() {
        let mut model = MapModel::new(16, 16);
        let first = model.insert_object(MapObject {
            id: 0,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(2, 3),
            owner: None,
        });
        let second = model.insert_object(MapObject {
            id: 0,
            class_id: ClassId::VehicleE1,
            position: Vec2i::new(4, 5),
            owner: Some(ClassId::HouseGDI),
        });
        assert_ne!(first, second);
        assert_eq!(model.object_at(Vec2i::new(2, 3)).unwrap().id, first);
        assert_eq!(
            model.remove_object(second).unwrap().class_id,
            ClassId::VehicleE1
        );
        assert_eq!(model.objects.len(), 1);
    }

    #[test]
    fn ini_roundtrip_preserves_cells_objects_and_triggers() {
        let mut model = MapModel::new(4, 3);
        model.name = "Snow Patrol".to_string();
        model.player = "Soviet".to_string();
        model.briefing = "Hold the northern pass".to_string();
        model.objectives = vec!["Build a base".to_string(), "Destroy radar".to_string()];
        model.theater = "Snow".to_string();
        model.terrain[0] = 7;
        model.terrain[5] = 9;
        model.overlay[1] = 3;
        model.player_starts.push(Vec2i::new(1, 2));
        model.insert_object(MapObject {
            id: 3,
            class_id: ClassId::ConstructionYard,
            position: Vec2i::new(2, 2),
            owner: Some(ClassId::HouseGDI),
        });
        model.add_trigger(Trigger {
            id: 1,
            name: "win".to_string(),
            condition: "always".to_string(),
            action: "win".to_string(),
            group: "Victory".to_string(),
        });

        let text = model.to_ini();
        let loaded = MapModel::from_ini(&text).unwrap();
        assert_eq!(loaded.width, 4);
        assert_eq!(loaded.height, 3);
        assert_eq!(loaded.name, "Snow Patrol");
        assert_eq!(loaded.player, "Soviet");
        assert_eq!(loaded.briefing, "Hold the northern pass");
        assert_eq!(loaded.objectives, model.objectives);
        assert_eq!(loaded.theater, "Snow");
        assert_eq!(loaded.terrain[0], 7);
        assert_eq!(loaded.terrain[5], 9);
        assert_eq!(loaded.overlay[1], 3);
        assert_eq!(loaded.player_starts, vec![Vec2i::new(1, 2)]);
        assert_eq!(loaded.objects.len(), 1);
        assert_eq!(loaded.objects[&3].class_id, ClassId::ConstructionYard);
        assert_eq!(loaded.triggers[0].name, "win");
        assert_eq!(loaded.triggers[0].group, "Victory");
    }

    #[test]
    fn extension_sections_roundtrip_terrain_overlay_and_elevation() {
        let mut model = MapModel::new(4, 4);
        model.player_starts.push(Vec2i::new(0, 0));
        for (index, (terrain, overlay)) in [
            (TerrainType::Mud.id(), OverlayType::Road.id()),
            (TerrainType::Crater.id(), OverlayType::Wall.id()),
            (TerrainType::Scorched.id(), 0),
            (TerrainType::Swamp.id(), 0),
            (TerrainType::Ice.id(), 0),
            (TerrainType::Concrete.id(), 0),
            (TerrainType::ForestFloor.id(), 0),
            (TerrainType::JungleFloor.id(), 0),
        ]
        .into_iter()
        .enumerate()
        {
            model.terrain[index] = terrain;
            model.overlay[index] = overlay;
        }
        for index in 0..model.elevation.len() {
            model.elevation[index] = ((index * 13) % 256) as u8;
        }

        let text = model.to_ini();
        assert!(
            text.contains("[Elevation]"),
            "painted elevation is exported"
        );
        let loaded = MapModel::from_ini(&text).unwrap();
        assert_eq!(loaded.terrain, model.terrain);
        assert_eq!(loaded.overlay, model.overlay);
        assert_eq!(loaded.elevation, model.elevation);
        assert!(
            loaded.validate().is_empty(),
            "extended ids keep the map valid: {:?}",
            loaded.validate()
        );
    }

    #[test]
    fn scenarios_without_optional_sections_load_flat() {
        // A map exported by an older build: no [Elevation] at all.
        let text = "[Map]\nWidth=2\nHeight=2\n[PlayerStarts]\n0=(0,0)\n\
                    [Terrain]\n0=0 0\n1=0 0\n[Overlay]\n0=0 0\n1=0 0\n";
        let loaded = MapModel::from_ini(text).unwrap();
        assert!(loaded.elevation.iter().all(|level| *level == 0));
        assert!(loaded.validate().is_empty());
        // Re-exporting drops the empty section again.
        assert!(!loaded.to_ini().contains("[Elevation]"));
    }

    #[test]
    fn validation_flags_missing_starts_and_out_of_bounds_objects() {
        let mut model = MapModel::new(8, 8);
        let issues = model.validate();
        assert!(issues.iter().any(|issue| issue.contains("no player start")));

        model.player_starts.push(Vec2i::new(1, 1));
        model.insert_object(MapObject {
            id: 2,
            class_id: ClassId::InfantryE1,
            position: Vec2i::new(99, 99),
            owner: None,
        });
        let issues = model.validate();
        assert!(issues.iter().any(|issue| issue.contains("out of bounds")));
        assert_eq!(
            issues.iter().filter(|i| i.contains("player start")).count(),
            0
        );
    }

    #[test]
    fn agents_roundtrip_with_scripts_containing_ini_metacharacters() {
        let mut model = MapModel::new(8, 8);
        model.player_starts.push(Vec2i::new(1, 1));
        let mut agent = AgentDefinition::new(0, "Iron; Fist");
        agent.owner = "HouseNod".to_string();
        agent.doctrine = "Aggressive".to_string();
        agent.script = "# semicolons; and newlines\non start { log(\"a;b\") }\n".to_string();
        let script = agent.script.clone();
        let id = model.add_agent(agent);
        assert_eq!(id, 1);

        let ini = model.to_ini();
        assert!(ini.contains("[AIAgents]"));
        let loaded = MapModel::from_ini(&ini).expect("scenario should parse");
        assert_eq!(loaded.agents.len(), 1);
        let loaded_agent = &loaded.agents[0];
        assert_eq!(loaded_agent.id, 1);
        assert_eq!(loaded_agent.name, "Iron; Fist");
        assert_eq!(loaded_agent.owner, "HouseNod");
        assert_eq!(loaded_agent.doctrine, "Aggressive");
        assert_eq!(loaded_agent.script, script);
        assert!(loaded.summary().contains("1 agents"));
    }

    #[test]
    fn validation_reports_agent_problems() {
        let mut model = MapModel::new(8, 8);
        model.player_starts.push(Vec2i::new(1, 1));
        let mut agent = AgentDefinition::new(0, "Fist");
        agent.owner = "HouseNod".to_string();
        agent.script = "on start { train( }".to_string();
        model.add_agent(agent);
        let mut second = AgentDefinition::new(0, "Claw");
        second.owner = "HouseNod".to_string();
        model.add_agent(second);

        let issues = model.validate();
        assert!(issues.iter().any(|issue| issue.contains("agent Fist")));
        assert!(issues
            .iter()
            .any(|issue| issue.contains("share owner HouseNod")));
    }

    #[test]
    fn ra_campaign_metadata_survives_editor_roundtrip() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/ra_campaign/01.ini");
        let text = std::fs::read_to_string(&path).unwrap();
        let model = MapModel::from_ini(&text).unwrap();
        assert_eq!(model.name, "01 - Holding the Line");
        assert_eq!(model.player, "Allies");
        assert!(!model.briefing.is_empty());
        assert_eq!(model.objectives.len(), 3);

        let reopened = MapModel::from_ini(&model.to_ini()).unwrap();
        assert_eq!(reopened.name, model.name);
        assert_eq!(reopened.player, model.player);
        assert_eq!(reopened.briefing, model.briefing);
        assert_eq!(reopened.objectives, model.objectives);
        assert_eq!(reopened.width, model.width);
        assert_eq!(reopened.height, model.height);
    }

    #[test]
    fn world_and_cell_coordinates_roundtrip_through_a_cell_centre() {
        let model = MapModel::new(10, 10);
        let world = MapModel::world_center(Vec2i::new(3, 4));
        assert_eq!(model.cell_at_world(world), Vec2i::new(3, 4));
        assert!(model.in_bounds(Vec2i::new(9, 9)));
        assert!(!model.in_bounds(Vec2i::new(10, 0)));
        assert!(model.index(Vec2i::new(-1, 0)).is_none());
        assert!(model.index(Vec2i::new(i32::MIN, i32::MIN)).is_none());
        assert!(model.terrain_at(Vec2i::new(-99, -99)).is_none());
    }

    fn original_ra_text(
        map: &[cnc_asset::ra_pack::PackedCell],
        overlay: &[cnc_asset::ra_pack::PackedCell],
    ) -> String {
        let mut text = String::from("[Basic]\nName=Imported\nPlayer=Greece\nBriefing=Go.\n");
        for (section, cells) in [("MapPack", map), ("OverlayPack", overlay)] {
            text.push_str(&format!("[{section}]\n"));
            for (key, value) in cnc_asset::ra_pack::encode_pack(cells) {
                text.push_str(&format!("{key}={value}\n"));
            }
        }
        text
    }

    #[test]
    fn import_text_routes_td_when_a_bin_is_supplied() {
        // [Basic] + [Wunits] is the original TD shape.
        let text = "[Basic]\nName=TD\n[Wunits]\n1=E1\n";
        let bin = [1u8, 0, 2, 1, 3, 2, 4, 3];
        let (model, kind) = MapModel::import_text_with(text, Some(&bin)).expect("td");
        assert_eq!(kind, ImportKind::OriginalTd);
        assert_eq!(model.terrain, vec![1, 2, 3, 4]);
    }

    #[test]
    fn import_text_fails_clearly_for_td_without_a_bin() {
        let text = "[Basic]\nName=TD\n[Wunits]\n1=E1\n";
        let error = MapModel::import_text(text).unwrap_err().to_string();
        assert!(
            error.contains("bin"),
            "the message should name the missing .bin, got: {error}"
        );
    }

    #[test]
    fn td_shaped_text_does_not_masquerade_as_the_editor_dialect() {
        // Without TD detection this falls through to from_ini and fails with
        // a confusing dialect error; the dispatch must catch it first.
        let text = "[Basic]\nName=TD\n[Structures]\n1=PROC\n";
        assert!(MapModel::import_text(text).is_err());
        assert!(looks_like_original_td(text));
    }
    #[test]
    fn import_text_dispatches_by_dialect() {
        let dialect = "[Map]\nWidth=8\nHeight=8\nTheater=Temperate\nName=Ed\n";
        let (model, kind) = MapModel::import_text(dialect).expect("editor dialect");
        assert_eq!(kind, ImportKind::EditorDialect);
        assert_eq!(model.name, "Ed");
    }

    #[test]
    fn import_text_routes_original_ra_before_the_editor_dialect() {
        use cnc_asset::ra_pack::PackedCell;
        let map = vec![PackedCell { index: 0, id: 11 }];
        let text = original_ra_text(&map, &[]);
        let (_, kind) = MapModel::import_text(&text).expect("original RA");
        assert_eq!(kind, ImportKind::OriginalRa);
    }

    #[test]
    fn import_td_requires_a_bin() {
        let text = "[Basic]\nName=TD\n";
        assert!(MapModel::import_td(text, None).is_err());
        let (_, kind) =
            MapModel::import_td(text, Some(&[1u8, 0, 2, 1, 3, 2, 4, 3])).expect("td import");
        assert_eq!(kind, ImportKind::OriginalTd);
    }

    #[test]
    fn imports_the_byte_exact_ra_fixture() {
        use crate::fixtures;
        let text = fixtures::ra_scenario_text();
        let (model, kind) = MapModel::import_text(&text).expect("import fixture");
        assert_eq!(kind, ImportKind::OriginalRa, "routed to the RA reader");
        assert_eq!((model.width, model.height), (16, 16));
        assert_eq!(model.name, "Fixture One");
        // Diagonal cells carry template 11; the four corners carry 22.
        assert_eq!(model.terrain[0], 11, "cell 0 is on the diagonal");
        assert_eq!(model.terrain[15], 22, "top-right corner");
        assert_eq!(model.overlay[0], 3, "overlay decoded");
        assert_eq!(model.overlay[255], 3);
        // Objects from all three sections.
        assert_eq!(model.objects.len(), 5, "2 units, 1 infantry, 2 structures");
        // Houses with RA-only fields.
        assert_eq!(model.houses.len(), 2);
        assert_eq!(model.houses[0].credits, 4000);
        assert_eq!(model.houses[0].tech_level, 7);
    }

    #[test]
    fn imports_a_byte_exact_lcw_compressed_td_bin() {
        use crate::fixtures;
        let text = fixtures::td_scenario_text();
        let raw = fixtures::td_bin_bytes();
        let compressed = fixtures::td_bin_bytes_lcw();
        assert!(
            compressed.len() < raw.len(),
            "the fixture must actually be compressed"
        );

        let (packed_model, _) =
            MapModel::import_text_with(&text, Some(&compressed)).expect("lcw import");
        let (raw_model, _) = MapModel::import_text_with(&text, Some(&raw)).expect("raw import");

        // The whole point: compressed and raw decode to the same map.
        assert_eq!(packed_model.terrain, raw_model.terrain);
        assert_eq!(packed_model.icon, raw_model.icon);
        assert_eq!((packed_model.width, packed_model.height), (16, 16));
    }
    #[test]
    fn imports_the_byte_exact_td_fixture() {
        use crate::fixtures;
        let text = fixtures::td_scenario_text();
        let bin = fixtures::td_bin_bytes();
        let (model, kind) = MapModel::import_text_with(&text, Some(&bin)).expect("import");
        assert_eq!(kind, ImportKind::OriginalTd);
        assert_eq!((model.width, model.height), (16, 16));
        assert_eq!(model.name, "Fixture TD");
        // Diagonal is template 12, elsewhere 7.
        assert_eq!(model.terrain[0], 12);
        assert_eq!(model.terrain[1], 7);
        // Icon byte is the low nibble of the index — this is the assertion
        // that would fail if TIcon were dropped.
        assert_eq!(model.icon[0], 0);
        assert_eq!(model.icon[3], 3);
        assert_eq!(model.icon[240], 0, "240 % 16 == 0");
        assert_eq!(model.objects.len(), 3, "2 units + 1 structure");
        assert_eq!(model.triggers.len(), 1);
        assert_eq!(model.triggers[0].action, "Win");
        assert_eq!(model.houses.len(), 2);
        assert_eq!(model.houses[0].edge, "West");
    }
    #[test]
    fn ra_import_reads_houses_and_their_ra_only_fields() {
        use cnc_asset::ra_pack::PackedCell;
        let map = vec![PackedCell { index: 0, id: 11 }];
        let mut text = original_ra_text(&map, &[]);
        // Unique ordinals: INI keys must not repeat within a section.
        text.push_str("[Houses]\n0=Greece\n1=Soviet\n");
        text.push_str("[Greece]\nCredits=40\nTechLevel=7\nMaxInfantry=60\nMaxVessel=3\n");
        text.push_str("[Soviet]\nCredits=20\n");
        let model = MapModel::from_original_ra(&text).expect("import");
        assert_eq!(model.houses.len(), 2);
        let greece = &model.houses[0];
        assert_eq!(greece.credits, 4000, "scaled by 100");
        assert_eq!(greece.tech_level, 7, "RA-only field");
        assert_eq!(greece.max_infantry, 60);
        assert_eq!(greece.max_vessel, 3);
        assert_eq!(model.houses[1].tech_level, 0, "absent in that section");
    }
    #[test]
    fn td_import_reads_houses_with_scaled_credits() {
        let bin = vec![0u8; 8];
        let text = "[Basic]\nName=TD\n\
[Houses]\n0=GDI\n1=Nod\n\
[GDI]\nCredits=50\nMaxUnit=120\nMaxBuilding=90\nEdge=West\n\
[Nod]\nCredits=25\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        assert_eq!(model.houses.len(), 2);
        assert_eq!(model.houses[0].name, "GDI");
        assert_eq!(model.houses[0].credits, 5000, "HOUSE.CPP scales by 100");
        assert_eq!(model.houses[0].edge, "West");
        assert_eq!(model.houses[1].max_units, 150, "default when absent");
    }

    #[test]
    fn a_model_without_houses_starts_empty() {
        let model = MapModel::new(4, 4);
        assert!(model.houses.is_empty());
    }
    #[test]
    fn td_import_carries_the_icon_byte() {
        // 2x2: records (template, icon).
        let bin = vec![7u8, 0, 7, 1, 7, 2, 7, 3];
        let text = "[Basic]\nName=TD\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        assert_eq!(model.terrain, vec![7, 7, 7, 7], "template byte");
        assert_eq!(
            model.icon,
            vec![0, 1, 2, 3],
            "icon byte, previously dropped"
        );
    }

    #[test]
    fn a_model_without_icon_data_defaults_to_zero() {
        let model = MapModel::new(4, 4);
        assert_eq!(model.icon.len(), 16);
        assert!(model.icon.iter().all(|i| *i == 0));
    }
    #[test]
    fn td_import_reads_trigs() {
        let bin = vec![0u8; 8];
        let text = "[Basic]\nName=TD\n\
[Trigs]\n1=TRIGA\n\
[TRIGA]\nName=First\nCondition1=Player Enters\nAction1=Win\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        assert_eq!(model.triggers.len(), 1);
        let trigger = &model.triggers[0];
        assert_eq!(trigger.name, "TRIGA");
        assert_eq!(trigger.condition, "Player Enters");
        assert_eq!(trigger.action, "Win");
    }

    #[test]
    fn td_import_joins_multiple_trigger_slots() {
        let bin = vec![0u8; 8];
        let text = "[Basic]\nName=TD\n\
[Trigs]\n1=TRIGA\n\
[TRIGA]\nCondition1=Time\nCondition2=Credits\nAction1=Win\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        assert_eq!(model.triggers[0].condition, "Time; Credits");
    }

    #[test]
    fn td_import_skips_empty_trigger_sections() {
        let bin = vec![0u8; 8];
        let text = "[Basic]\nName=TD\n[Trigs]\n1=EMPTY\n[EMPTY]\nName=None\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        assert!(
            model.triggers.is_empty(),
            "no condition or action = no trigger"
        );
    }
    #[test]
    fn td_import_reads_units_and_structures() {
        let bin = vec![0u8; 8]; // 2x2
        let text = "[Basic]\nName=TD\n\
[Wunits]\n1=GDI,E1,256,1,2,Guard,\n\
[Structures]\n1=GDI,PROC,256,3,0,\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        assert_eq!(model.objects.len(), 2, "both sections contribute");
        let cells: std::collections::BTreeSet<(i32, i32)> = model
            .objects
            .values()
            .map(|o| (o.position.x, o.position.y))
            .collect();
        assert!(cells.contains(&(1, 0)), "cell 1 on a 2-wide grid");
        assert!(cells.contains(&(1, 1)), "cell 3 on a 2-wide grid");
        assert!(
            model
                .objects
                .values()
                .all(|o| o.owner == Some(ClassId::HouseGDI)),
            "settler house parsed"
        );
    }

    #[test]
    fn td_import_keeps_unknown_types_as_unknown() {
        let bin = vec![1u8, 0, 2, 1, 3, 2, 4, 3];
        let text = "[Basic]\nName=TD\n[Wunits]\n1=GDI,NOT_A_REAL_UNIT,256,1,2,Guard,\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        // The type is unresolvable but the placement is still real, so the
        // object is kept as Unknown rather than dropped.
        assert_eq!(model.objects.len(), 1);
        assert_eq!(
            model.objects.values().next().unwrap().class_id,
            ClassId::Unknown
        );
        assert_eq!(model.terrain, vec![1, 2, 3, 4], "terrain still imported");
    }

    #[test]
    fn td_import_resolves_shared_unit_codes() {
        let bin = vec![0u8; 8];
        let text = "[Basic]\nName=TD\n[Wunits]\n1=GDI,E1,256,1,2,Guard,\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        let object = model.objects.values().next().expect("an object");
        assert_eq!(object.class_id, ClassId::InfantryE1, "E1 resolves");
    }

    #[test]
    fn td_import_ignores_truncated_object_records() {
        let bin = vec![0u8; 8];
        let text = "[Basic]\nName=TD\n[Wunits]\n1=GDI\n2=GDI,E1\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        assert!(model.objects.is_empty(), "short records are skipped");
    }
    #[test]
    fn ra_import_reads_units_and_structures() {
        use cnc_asset::ra_pack::PackedCell;
        let map = vec![PackedCell { index: 0, id: 11 }];
        let mut text = original_ra_text(&map, &[]);
        text.push_str("[Units]\n1=Greece,HTNK,256,0,0,Guard,\n");
        text.push_str("[Structures]\n1=Greece,POWR,256,0,0,\n");
        let model = MapModel::from_original_ra(&text).expect("import");
        assert_eq!(model.objects.len(), 2, "both sections contribute");
        assert!(
            model
                .objects
                .values()
                .all(|o| o.owner == Some(ClassId::HouseAllies)),
            "Greece is an Allied house"
        );
    }

    #[test]
    fn ra_import_keeps_unknown_types_as_unknown() {
        use cnc_asset::ra_pack::PackedCell;
        let map = vec![PackedCell { index: 0, id: 11 }];
        let mut text = original_ra_text(&map, &[]);
        text.push_str("[Units]\n1=Greece,NOPE,256,0\n");
        let model = MapModel::from_original_ra(&text).expect("import");
        assert_eq!(model.objects.len(), 1);
        assert_eq!(
            model.objects.values().next().unwrap().class_id,
            ClassId::Unknown
        );
    }

    #[test]
    fn ra_import_skips_out_of_bounds_cells() {
        use cnc_asset::ra_pack::PackedCell;
        let map = vec![PackedCell { index: 0, id: 11 }];
        let mut text = original_ra_text(&map, &[]);
        // A 1x1 map (index 0 only), so cell 99 is out of bounds.
        text.push_str("[Units]\n1=Greece,HTNK,256,99\n");
        let model = MapModel::from_original_ra(&text).expect("import");
        assert!(model.objects.is_empty(), "cell 99 is off-map");
    }
    #[test]
    fn imports_an_uncompressed_td_bin() {
        // 2x2 grid, one 2-byte record per cell: template + icon.
        let bin = vec![1u8, 0, 2, 1, 3, 2, 4, 3];
        let text = "[Basic]\nName=TD Map\nPlayer=GDI\nTheater=Temperate\n";
        let model = MapModel::from_original_td(text, &bin).expect("import");
        assert_eq!((model.width, model.height), (2, 2));
        assert_eq!(model.terrain, vec![1, 2, 3, 4]);
        assert_eq!(model.name, "TD Map");
        assert_eq!(model.player, "GDI");
    }

    #[test]
    fn td_import_rejects_empty_and_garbage_bins() {
        let text = "[Basic]\nName=X\n";
        assert!(MapModel::from_original_td(text, &[]).is_err());
        assert!(MapModel::from_original_td(text, &[9u8; 7]).is_err());
    }

    #[test]
    fn td_import_requires_a_basic_section() {
        let bin = vec![0u8; 8];
        assert!(MapModel::from_original_td("[Map]\nWidth=2\n", &bin).is_err());
    }

    #[test]
    fn imports_original_ra_terrain_and_overlay() {
        use cnc_asset::ra_pack::PackedCell;
        let map = vec![
            PackedCell { index: 0, id: 11 },
            PackedCell { index: 5, id: 22 },
        ];
        let overlay = vec![PackedCell { index: 5, id: 3 }];
        let model = MapModel::from_original_ra(&original_ra_text(&map, &overlay)).unwrap();

        assert_eq!(model.name, "Imported");
        assert_eq!(model.player, "Greece");
        assert_eq!(model.briefing, "Go.");
        assert_eq!(model.terrain[0], 11);
        assert_eq!(model.terrain[5], 22);
        assert_eq!(model.overlay[5], 3);
    }

    #[test]
    fn original_import_infers_a_square_side() {
        use cnc_asset::ra_pack::PackedCell;
        // Highest index 8 needs a 3x3 grid.
        let map = vec![PackedCell { index: 8, id: 1 }];
        let model = MapModel::from_original_ra(&original_ra_text(&map, &[])).unwrap();
        assert_eq!((model.width, model.height), (3, 3));
        assert_eq!(model.terrain.len(), 9);
    }

    #[test]
    fn original_import_rejects_editor_dialect() {
        let dialect = "[Map]\nWidth=8\nHeight=8\n[Terrain]\n0=0 0 0\n";
        assert!(MapModel::from_original_ra(dialect).is_err());
    }

    #[test]
    fn original_import_requires_map_pack() {
        assert!(MapModel::from_original_ra("[Basic]\nName=X\n").is_err());
    }
}
