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
    pub objects: HashMap<u32, MapObject>,
    pub triggers: Vec<Trigger>,
    /// Authored AI commanders, stored alongside the scenario.
    pub agents: Vec<AgentDefinition>,
    pub player_starts: Vec<Vec2i>,
    next_object_id: u32,
    next_agent_id: u32,
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
            objects: HashMap::new(),
            triggers: Vec::new(),
            agents: Vec::new(),
            player_starts: Vec::new(),
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
                issues.push(format!("player start ({}, {}) is out of bounds", start.x, start.y));
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
                        if let Some(other) = owners.insert(agent.owner.clone(), agent.name.clone()) {
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

    /// Parse a scenario produced by [`Self::to_ini`].
    pub fn from_ini(input: &str) -> anyhow::Result<Self> {
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
                .filter_map(|(key, value)| {
                    Some((key.parse::<u32>().ok()?, value_text(value)?))
                })
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
        for (section_name, target) in [("Terrain", true), ("Overlay", false)] {
            if let Some(section) = ini.sections.get(section_name) {
                for (row_key, value) in &section.values {
                    let Ok(y) = row_key.parse::<u32>() else { continue };
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
                        if target {
                            model.terrain[index] = cell.clamp(0, 255) as u8;
                        } else {
                            model.overlay[index] = cell.clamp(0, 255) as u8;
                        }
                    }
                }
            }
        }
        if let Some(section) = ini.sections.get("Objects") {
            for (key, value) in &section.values {
                let Ok(id) = key.parse::<u32>() else { continue };
                let Some(text) = value_text(value) else { continue };
                let parts: Vec<&str> = text.split(',').map(str::trim).collect();
                if parts.len() < 3 {
                    continue;
                }
                let Some(class_id) = parse_class_id(parts[0]) else { continue };
                let position = Vec2i::new(
                    parts[1].parse().unwrap_or(0),
                    parts[2].parse().unwrap_or(0),
                );
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
                let Some(text) = value_text(value) else { continue };
                let parts: Vec<&str> = text.split('|').map(str::trim).collect();
                if parts.len() < 3 {
                    continue;
                }
                model.add_trigger(Trigger {
                    id,
                    name: parts[0].to_string(),
                    condition: parts[1].to_string(),
                    action: parts[2].to_string(),
                    group: parts.get(3).map_or(String::new(), |group| group.to_string()),
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
    if text.len() % 2 != 0 {
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
    for pair in bytes.chunks_exact(2) {
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(model.remove_object(second).unwrap().class_id, ClassId::VehicleE1);
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
        assert_eq!(
            loaded.objects[&3].class_id,
            ClassId::ConstructionYard
        );
        assert_eq!(loaded.triggers[0].name, "win");
        assert_eq!(loaded.triggers[0].group, "Victory");
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
        assert_eq!(issues.iter().filter(|i| i.contains("player start")).count(), 0);
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
        assert_eq!(loaded.summary().contains("1 agents"), true);
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
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../maps/ra_campaign/01.ini");
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
}
