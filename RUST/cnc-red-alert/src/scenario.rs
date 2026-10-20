//! Red Alert scenario files.
//!
//! Scenarios use the same INI dialect the Mobius editor writes: [Map],
//! [PlayerStarts], [Terrain], [Overlay], [Objects] and [Triggers]. Terrain ids
//! follow the editor's TerrainType enum, overlay id 3 is ore and object classes
//! are Debug names such as "VehicleE2" or numeric ClassIds.

use std::path::Path;

use anyhow::{bail, Context, Result};
use cnc_core::{ClassId, IniFile, IniValue, Vec2i};

use crate::map::Terrain;

/// One unit or structure placed by the scenario.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioObject {
    pub class_id: ClassId,
    pub position: Vec2i,
    pub owner: Option<ClassId>,
}

/// One authored trigger.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScenarioTrigger {
    pub id: u32,
    pub name: String,
    pub condition: String,
    pub action: String,
}

/// Parsed RA scenario.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scenario {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub theater: String,
    /// Player side: 0 = Allied, 1 = Soviet.
    pub player: usize,
    pub briefing: String,
    /// Optional generated static briefing-art key (`[Map] BriefingImage`, or
    /// the `BriefPicture` spelling). Original scenarios never set it, so the
    /// parsed value is `None` for them — fully backward compatible. The value
    /// names art produced by `cnc_procgen::briefing` (static emblems/
    /// portraits; the original VQA briefing movies are out of reach because
    /// the licensed decoders are stubs).
    pub briefing_image: Option<String>,
    pub objectives: Vec<String>,
    /// Editor terrain ids, row-major.
    pub terrain: Vec<u8>,
    /// Ore amount per cell.
    pub ore: Vec<u32>,
    pub starts: Vec<Vec2i>,
    pub objects: Vec<ScenarioObject>,
    pub triggers: Vec<ScenarioTrigger>,
    /// `[CellTriggers]`: trigger name bound to a cell (row-major index).
    pub cell_triggers: Vec<(u32, String)>,
}

impl Scenario {
    pub fn from_file(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read scenario {}", path.display()))?;
        Self::from_ini(&text)
    }

    pub fn from_ini(input: &str) -> Result<Self> {
        let ini = IniFile::parse(input)?;
        let map = ini.sections.get("Map");
        let width = map
            .and_then(|section| section.get_i64("Width").ok())
            .unwrap_or(0) as u32;
        let height = map
            .and_then(|section| section.get_i64("Height").ok())
            .unwrap_or(0) as u32;
        if width == 0 || height == 0 {
            bail!("scenario [Map] needs positive Width and Height");
        }

        let mut scenario = Self {
            name: map
                .and_then(|section| section.get_string("Name"))
                .unwrap_or("Red Alert Scenario")
                .to_string(),
            width,
            height,
            theater: map
                .and_then(|section| section.get_string("Theater"))
                .unwrap_or("Temperate")
                .to_string(),
            player: match map
                .and_then(|section| section.get_string("Player"))
                .unwrap_or("Allies")
                .to_ascii_lowercase()
                .as_str()
            {
                "soviet" | "1" | "soviets" => 1,
                _ => 0,
            },
            briefing: map
                .and_then(|section| section.get_string("Briefing"))
                .unwrap_or("")
                .to_string(),
            briefing_image: map
                .and_then(|section| {
                    section
                        .get_string("BriefingImage")
                        .or_else(|| section.get_string("BriefPicture"))
                })
                .map(str::to_string),
            objectives: Vec::new(),
            terrain: vec![0; (width * height) as usize],
            ore: vec![0; (width * height) as usize],
            starts: Vec::new(),
            objects: Vec::new(),
            triggers: Vec::new(),
            cell_triggers: Vec::new(),
        };

        if let Some(starts) = ini.sections.get("PlayerStarts") {
            for value in starts.values.values() {
                if let Some(text) = value_text(value) {
                    if let Some(cell) = parse_point(&text) {
                        scenario.starts.push(cell);
                    }
                }
            }
        }

        for (section_name, is_terrain) in [("Terrain", true), ("Overlay", false)] {
            let Some(section) = ini.sections.get(section_name) else {
                continue;
            };
            for (row_key, value) in &section.values {
                let Ok(y) = row_key.parse::<u32>() else {
                    continue;
                };
                if y >= scenario.height {
                    continue;
                }
                let Some(text) = value_text(value) else {
                    continue;
                };
                let cells: Vec<i64> = text
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(|c: char| c == ',' || c.is_whitespace())
                    .filter_map(|part| part.trim().parse::<i64>().ok())
                    .collect();
                for (x, value) in cells.into_iter().enumerate() {
                    if x as u32 >= scenario.width {
                        break;
                    }
                    let index = (y as usize) * scenario.width as usize + x;
                    if is_terrain {
                        scenario.terrain[index] = value.clamp(0, 255) as u8;
                    } else if value == 3 {
                        // OverlayType::Ore from the editor.
                        scenario.ore[index] = scenario.ore[index].saturating_add(400);
                    }
                }
            }
        }

        if let Some(objectives) = ini.sections.get("Objectives") {
            let mut rows: Vec<(i64, String)> = objectives
                .values
                .iter()
                .filter_map(|(key, value)| Some((key.parse::<i64>().ok()?, value_text(value)?)))
                .collect();
            rows.sort_by_key(|(index, _)| *index);
            scenario.objectives = rows.into_iter().map(|(_, text)| text).collect();
        }

        if let Some(objects) = ini.sections.get("Objects") {
            for value in objects.values.values() {
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
                scenario.objects.push(ScenarioObject {
                    class_id,
                    position,
                    owner,
                });
            }
        }

        if let Some(triggers) = ini.sections.get("Triggers") {
            for (key, value) in &triggers.values {
                let Some(text) = value_text(value) else {
                    continue;
                };
                let parts: Vec<&str> = text.split('|').map(str::trim).collect();
                if parts.len() < 3 {
                    continue;
                }
                scenario.triggers.push(ScenarioTrigger {
                    id: key.parse().unwrap_or(0),
                    name: parts[0].to_string(),
                    condition: parts[1].to_string(),
                    action: parts[2].to_string(),
                });
            }
        }

        // [CellTriggers] binds a trigger name to a cell (row-major index),
        // the same dialect the original reader uses in DISPLAY.CPP.
        if let Some(cells) = ini.sections.get("CellTriggers") {
            for (key, value) in &cells.values {
                let Some(name) = value_text(value) else {
                    continue;
                };
                if let Ok(index) = key.trim().parse::<u32>() {
                    scenario
                        .cell_triggers
                        .push((index, name.trim().to_string()));
                }
            }
        }

        Ok(scenario)
    }

    /// Editor terrain id -> RA simulation terrain.
    pub fn terrain_at(&self, cell: Vec2i) -> Terrain {
        if cell.x < 0 || cell.y < 0 || cell.x as u32 >= self.width || cell.y as u32 >= self.height {
            return Terrain::Grass;
        }
        let index = cell.y as usize * self.width as usize + cell.x as usize;
        match self.terrain.get(index).copied().unwrap_or(0) {
            1 => Terrain::Water,
            2 => Terrain::Beach,
            4 => Terrain::Rock,
            _ => Terrain::Grass,
        }
    }
}

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

/// Parse a numeric ClassId or a Debug variant name used by editor scenarios.
pub fn parse_class_id(text: &str) -> Option<ClassId> {
    let text = text.trim().trim_matches('"').trim();
    if let Ok(value) = text.parse::<u32>() {
        let class_id = ClassId::from_u32(value);
        if class_id != ClassId::Unknown {
            return Some(class_id);
        }
    }
    Some(match text.to_ascii_lowercase().as_str() {
        "housegdi" => ClassId::HouseGDI,
        "housenod" => ClassId::HouseNod,
        "houseallies" => ClassId::HouseAllies,
        "housesoviet" => ClassId::HouseSoviet,
        "infantrye1" => ClassId::InfantryE1,
        "infantrye2" => ClassId::InfantryE2,
        "infantrye3" => ClassId::InfantryE3,
        "infantrye4" => ClassId::InfantryE4,
        "infantrye5" => ClassId::InfantryE5,
        "infantrye6" => ClassId::InfantryE6,
        "infantrye7" => ClassId::InfantryE7,
        "infantrye8" => ClassId::InfantryE8,
        "infantrye9" => ClassId::InfantryE9,
        "infantrye10" => ClassId::InfantryE10,
        "infantrye11" => ClassId::InfantryE11,
        "vehiclee1" => ClassId::VehicleE1,
        "vehiclee2" => ClassId::VehicleE2,
        "vehiclee3" => ClassId::VehicleE3,
        "vehiclee4" => ClassId::VehicleE4,
        "vehiclee5" => ClassId::VehicleE5,
        "vehiclee6" => ClassId::VehicleE6,
        "vehiclee7" => ClassId::VehicleE7,
        "vehiclee8" => ClassId::VehicleE8,
        "vehiclee9" => ClassId::VehicleE9,
        "vehiclee10" => ClassId::VehicleE10,
        "vehiclee11" => ClassId::VehicleE11,
        "vehiclee12" => ClassId::VehicleE12,
        "vehiclee13" => ClassId::VehicleE13,
        "vehiclee14" => ClassId::VehicleE14,
        "aircrafte1" => ClassId::AircraftE1,
        "aircrafte2" => ClassId::AircraftE2,
        "aircrafte3" => ClassId::AircraftE3,
        "aircrafte4" => ClassId::AircraftE4,
        "aircrafte5" => ClassId::AircraftE5,
        "aircrafte6" => ClassId::AircraftE6,
        "aircrafte7" => ClassId::AircraftE7,
        "aircrafte8" => ClassId::AircraftE8,
        "shipe1" => ClassId::ShipE1,
        "shipe2" => ClassId::ShipE2,
        "shipe3" => ClassId::ShipE3,
        "shipe4" => ClassId::ShipE4,
        "shipe5" => ClassId::ShipE5,
        "shipe6" => ClassId::ShipE6,
        "constructionyard" => ClassId::ConstructionYard,
        "powerplant" => ClassId::PowerPlant,
        "refinery" => ClassId::Refinery,
        "barracks" => ClassId::Barracks,
        "warfactory" => ClassId::WarFactory,
        "navalyard" => ClassId::NavalYard,
        "subpen" => ClassId::SubPen,
        "helipad" => ClassId::Helipad,
        "airfield" => ClassId::Airfield,
        "radar" => ClassId::Radar,
        "silo" => ClassId::Silo,
        "missilesilo" => ClassId::MissileSilo,
        "aa_gun" => ClassId::AA_Gun,
        "teslacoil" => ClassId::TeslaCoil,
        "gapgenerator" => ClassId::GapGenerator,
        "ironcurtain" => ClassId::IronCurtain,
        "chronosphere" => ClassId::Chronosphere,
        "techcenter" => ClassId::TechCenter,
        "biolab" => ClassId::BioLab,
        "factory" => ClassId::Factory,
        "building" => ClassId::Building,
        "fact" => ClassId::ConstructionYard,
        "proc" => ClassId::Refinery,
        "powr" => ClassId::PowerPlant,
        "weap" => ClassId::WarFactory,
        "tent" => ClassId::Barracks,
        "barr" => ClassId::Barracks,
        // Original-format scenarios name units by their Rules.ini type
        // (HTNK, MTNK, E1, ...) rather than by Debug class name.
        "e1" => ClassId::InfantryE1,
        "e2" => ClassId::InfantryE2,
        "e3" => ClassId::InfantryE3,
        "e4" => ClassId::InfantryE4,
        "e5" => ClassId::InfantryE5,
        "e6" => ClassId::InfantryE6,
        "e7" => ClassId::InfantryE7,
        "e8" => ClassId::InfantryE8,
        "e9" => ClassId::InfantryE9,
        "rmbo" => ClassId::InfantryE10,
        "ltnk" => ClassId::VehicleE1,
        "mtnk" => ClassId::VehicleE2,
        "htnk" => ClassId::VehicleE3,
        "mnly" => ClassId::VehicleE4,
        "mrj" => ClassId::VehicleE4,
        "apc" => ClassId::VehicleE5,
        "harv" => ClassId::VehicleE6,
        "mcv" => ClassId::VehicleE7,
        "jeep" => ClassId::VehicleE8,
        "fv" => ClassId::VehicleE9,
        "mlrs" => ClassId::VehicleE10,
        "arty" => ClassId::VehicleE11,
        "v2rl" => ClassId::VehicleE12,
        "dtrk" => ClassId::VehicleE13,
        "ttnk" => ClassId::VehicleE14,
        "heli" => ClassId::AircraftE1,
        "yak" => ClassId::AircraftE6,
        "mig" => ClassId::AircraftE7,
        "tran" => ClassId::AircraftE8,
        "gunb" => ClassId::ShipE1,
        "dest" => ClassId::ShipE2,
        "cruz" => ClassId::ShipE3,
        "sub" => ClassId::ShipE4,
        "lst" => ClassId::ShipE5,
        "carrier" => ClassId::ShipE6,
        "afld" => ClassId::Airfield,
        "hpad" => ClassId::Helipad,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCENARIO: &str = "[Map]
Width=16
Height=12
Theater=Snow
Name=Test Scenario
Player=Soviet
Briefing=Hold the line at all costs
[PlayerStarts]
0=(2,2)
1=(13,9)
[Terrain]
0=0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
1=0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
2=0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
3=0 0 0 1 1 1 0 0 0 0 0 0 0 0 0 0
4=0 0 0 1 1 1 0 0 0 0 0 0 0 0 0 0
5=0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
6=0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
[Overlay]
0=0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
1=0 0 0 0 3 0 0 0 0 0 0 0 0 0 0 0
[Objects]
1=ConstructionYard,2,2,HouseAllies
2=VehicleE2,5,5,HouseSoviet
3=InfantryE1,4,4,None
[Triggers]
1=Victory|elapsed:600|win
[Objectives]
0=Build a base
1=Destroy the enemy
";

    #[test]
    fn parses_map_objects_ore_and_triggers() {
        let scenario = Scenario::from_ini(SCENARIO).unwrap();
        assert_eq!(scenario.width, 16);
        assert_eq!(scenario.height, 12);
        assert_eq!(scenario.theater, "Snow");
        assert_eq!(scenario.starts, vec![Vec2i::new(2, 2), Vec2i::new(13, 9)]);
        assert_eq!(scenario.terrain_at(Vec2i::new(3, 3)), Terrain::Water);
        assert_eq!(scenario.ore[16 + 4], 400);
        assert_eq!(scenario.objects.len(), 3);
        assert_eq!(scenario.objects[0].class_id, ClassId::ConstructionYard);
        assert_eq!(scenario.objects[1].owner, Some(ClassId::HouseSoviet));
        assert_eq!(scenario.triggers.len(), 1);
        assert_eq!(scenario.triggers[0].action, "win");
        assert_eq!(scenario.player, 1);
        assert_eq!(scenario.briefing, "Hold the line at all costs");
        assert_eq!(
            scenario.objectives,
            vec!["Build a base", "Destroy the enemy"]
        );
    }

    #[test]
    fn class_names_and_numeric_ids_both_parse() {
        assert_eq!(parse_class_id("VehicleE2"), Some(ClassId::VehicleE2));
        assert_eq!(parse_class_id("navalyard"), Some(ClassId::NavalYard));
        assert_eq!(parse_class_id("501"), Some(ClassId::VehicleE2));
        assert_eq!(parse_class_id("not-a-class"), None);
    }

    #[test]
    fn rejects_scenarios_without_map_dimensions() {
        assert!(Scenario::from_ini("[Map]\nTheater=Temperate\n").is_err());
    }

    #[test]
    fn briefing_image_absent_is_none() {
        // Backward compatibility: existing RA scenarios carry no such key.
        let scenario = Scenario::from_ini(SCENARIO).unwrap();
        assert_eq!(scenario.briefing_image, None);
    }

    #[test]
    fn briefing_image_present_round_trips() {
        let ini = "[Map]
Width=16
Height=12
Briefing=Push east
BriefingImage=ra_soviet_emblem
[PlayerStarts]
0=(2,2)
";
        let scenario = Scenario::from_ini(ini).unwrap();
        assert_eq!(scenario.briefing, "Push east");
        assert_eq!(scenario.briefing_image.as_deref(), Some("ra_soviet_emblem"));

        // The `BriefPicture` spelling is accepted as an alias.
        let aliased = "[Map]
Width=8
Height=8
BriefPicture=td_gdi_portrait
";
        assert_eq!(
            Scenario::from_ini(aliased)
                .unwrap()
                .briefing_image
                .as_deref(),
            Some("td_gdi_portrait")
        );
    }
}
