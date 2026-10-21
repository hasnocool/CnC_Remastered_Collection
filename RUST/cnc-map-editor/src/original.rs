//! Original-format scenario writers for both games.
//!
//! Retail TD maps are an `.ini` plus a sibling binary cell image; retail RA
//! maps carry their grids in base64 `[MapPack]`/`[OverlayPack]` blocks. Both
//! cell containers are encoded and decoded by `cnc-asset` (format80 /
//! ra_pack); this module only arranges the INI sections around them.
//!
//! Sections the model does not represent field-by-field are retained verbatim
//! from the import and written back unchanged, so opening and saving an
//! original scenario does not silently discard data the editor does not
//! understand yet. Sections the model does represent (the `[Basic]` family, the
//! cell grids, `[Map]` dimensions when present, `[Waypoints]`, and the object
//! records that were imported) are regenerated.

use crate::map::{HouseEntry, ImportKind, MapModel, MapObject};
use anyhow::{bail, Context};
use cnc_core::ClassId;

/// One INI section retained verbatim from an original-format import.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreservedSection {
    pub name: String,
    /// Key/value text exactly as it appeared in the source file.
    pub entries: Vec<(String, String)>,
}

/// Where one imported object came from, so a save can rewrite it in place
/// instead of rebuilding (and thereby damaging) the original record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectOrigin {
    /// `Wunits`/`Structures` for TD, `Units`/`Infantry`/`Structures` for RA.
    pub section: String,
    /// Key inside [`section`] ("1", "Unit1", ...).
    pub key: String,
    pub object_id: u32,
    /// Row-major cell index the record originally pointed at.
    pub cell: u32,
    pub owner: Option<ClassId>,
    /// Comma-split fields for the inline RA record (empty for named TD records).
    pub fields: Vec<String>,
    /// Named TD object section (`[UNIT_GDI_E1]`), when the dialect uses one.
    pub named_section: Option<String>,
}

/// Everything retained from an original-format import.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalImport {
    pub dialect: ImportKind,
    pub sections: Vec<PreservedSection>,
    pub objects: Vec<ObjectOrigin>,
}

/// Capture every INI section verbatim, including ones the model ignores.
///
/// This is a raw line scanner, not a parser: comments and blank lines are
/// dropped, everything else is preserved as source text so a save can hand it
/// back unchanged.
pub fn capture_sections(text: &str) -> Vec<PreservedSection> {
    let mut sections: Vec<PreservedSection> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            sections.push(PreservedSection {
                name: line[1..line.len() - 1].trim().to_string(),
                entries: Vec::new(),
            });
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let Some(section) = sections.last_mut() else {
            continue;
        };
        section
            .entries
            .push((key.trim().to_string(), value.trim().to_string()));
    }
    sections
}

/// Sections the writers regenerate from the model.
const REGENERATED: &[&str] = &["Basic", "Map", "MapPack", "OverlayPack", "Waypoints"];

fn find_section<'a>(
    sections: &'a mut [PreservedSection],
    name: &str,
) -> Option<&'a mut PreservedSection> {
    sections
        .iter_mut()
        .find(|section| section.name.eq_ignore_ascii_case(name))
}

fn has_section(sections: &[PreservedSection], name: &str) -> bool {
    sections
        .iter()
        .any(|section| section.name.eq_ignore_ascii_case(name))
}

fn upsert_section(
    sections: &mut Vec<PreservedSection>,
    name: &str,
    entries: Vec<(String, String)>,
) {
    if let Some(section) = find_section(sections, name) {
        section.entries = entries;
    } else {
        sections.push(PreservedSection {
            name: name.to_string(),
            entries,
        });
    }
}

fn remove_section(sections: &mut Vec<PreservedSection>, name: &str) {
    sections.retain(|section| !section.name.eq_ignore_ascii_case(name));
}

fn set_entry(entries: &mut Vec<(String, String)>, key: &str, value: String) {
    if let Some((_, slot)) = entries
        .iter_mut()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
    {
        *slot = value;
    } else {
        entries.push((key.to_string(), value));
    }
}

fn render(sections: &[PreservedSection]) -> String {
    let mut out = String::new();
    for section in sections {
        if section.name.is_empty() {
            continue;
        }
        out.push_str(&format!("[{}]\n", section.name));
        for (key, value) in &section.entries {
            out.push_str(&format!("{key}={value}\n"));
        }
    }
    out
}

fn cell_index(model: &MapModel, object: &MapObject) -> Option<u32> {
    model
        .index(object.position)
        .and_then(|index| u32::try_from(index).ok())
}

/// Map a house class back onto the token the dialect writes.
fn house_token(dialect: ImportKind, owner: Option<ClassId>) -> String {
    match owner {
        Some(ClassId::HouseGDI) => "GDI".to_string(),
        Some(ClassId::HouseNod) => "Nod".to_string(),
        Some(ClassId::HouseAllies) => "Allies".to_string(),
        Some(ClassId::HouseSoviet) => "Soviet".to_string(),
        Some(other) => format!("{other:?}"),
        None => match dialect {
            ImportKind::OriginalTd => "GDI".to_string(),
            _ => "Greece".to_string(),
        },
    }
}

/// Map a class id back onto its INI shorthand where the vocabulary has one.
fn class_token(class: ClassId) -> String {
    const SHORTHANDS: &[&str] = &[
        "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "E9", "E10", "E11", "LTNK", "MTNK", "HTNK",
        "MAMM", "APC", "HARV", "MCV", "RANG", "HUMV", "MLRS", "ARTY", "V2", "ORCA", "TRAN",
        "GUNBOAT",
    ];
    for token in SHORTHANDS {
        if cnc_core::class_id_from_token(token).ok() == Some(class) {
            return (*token).to_string();
        }
    }
    class.as_u32().to_string()
}

fn is_infantry(class: ClassId) -> bool {
    let id = class.as_u32();
    class == ClassId::Infantry || (400..=410).contains(&id)
}

fn object_section(dialect: ImportKind, class: ClassId) -> &'static str {
    match dialect {
        ImportKind::OriginalTd => {
            if class.is_building() {
                "Structures"
            } else {
                "Wunits"
            }
        }
        _ => {
            if class.is_building() {
                "Structures"
            } else if is_infantry(class) {
                "Infantry"
            } else {
                "Units"
            }
        }
    }
}

fn is_object_section(name: &str) -> bool {
    ["Wunits", "Structures", "Units", "Infantry"]
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(name))
}

fn next_parent_key(entries: &[(String, String)]) -> String {
    for (key, _) in entries {
        let prefix = if key.starts_with("Unit") {
            "Unit"
        } else if key.starts_with("Structure") {
            "Structure"
        } else {
            ""
        };
        let rest = &key[prefix.len()..];
        if rest.is_empty() || !rest.bytes().all(|byte| byte.is_ascii_digit()) {
            continue;
        }
        let max = entries
            .iter()
            .filter_map(|(candidate, _)| {
                candidate
                    .strip_prefix(prefix)
                    .and_then(|candidate| candidate.parse::<u32>().ok())
                    .filter(|_| candidate.starts_with(prefix) || prefix.is_empty())
            })
            .max()
            .unwrap_or(0);
        return format!("{prefix}{}", max + 1);
    }
    let max = entries
        .iter()
        .filter_map(|(key, _)| key.parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    format!("{}", max + 1)
}

fn ra_pack_entries(model: &MapModel, overlay_only: bool) -> anyhow::Result<Vec<(String, String)>> {
    let mut cells = Vec::new();
    for index in 0..(model.width as usize * model.height as usize) {
        let id = if overlay_only {
            model.overlay[index]
        } else {
            model.terrain[index]
        };
        if overlay_only && id == 0 {
            continue;
        }
        let Ok(index) = u16::try_from(index) else {
            bail!("RA map packs address cells as u16; the map is too large");
        };
        cells.push(cnc_asset::ra_pack::PackedCell { index, id });
    }
    Ok(cnc_asset::ra_pack::encode_pack(&cells))
}

fn td_bin(model: &MapModel) -> anyhow::Result<Vec<u8>> {
    let mut raw = Vec::with_capacity(model.terrain.len() * 2);
    for index in 0..model.terrain.len() {
        raw.push(model.terrain[index]);
        raw.push(model.icon.get(index).copied().unwrap_or(0));
    }
    cnc_asset::format80::compress(&raw, raw.len()).context("compress the TD cell image")
}

fn upsert_basic(model: &MapModel, sections: &mut Vec<PreservedSection>) {
    if !has_section(sections, "Basic") {
        sections.insert(
            0,
            PreservedSection {
                name: "Basic".to_string(),
                entries: Vec::new(),
            },
        );
    }
    let Some(basic) = find_section(sections, "Basic") else {
        return;
    };
    set_entry(&mut basic.entries, "Name", model.name.clone());
    if !model.player.is_empty() {
        set_entry(&mut basic.entries, "Player", model.player.clone());
    }
    if !model.theater.is_empty() {
        set_entry(&mut basic.entries, "Theater", model.theater.clone());
    }
    if !model.briefing.is_empty() {
        set_entry(&mut basic.entries, "Briefing", model.briefing.clone());
    }
}

fn upsert_map_dimensions(model: &MapModel, sections: &mut [PreservedSection]) {
    // Only touch a [Map] block that is already there: the retail dialects infer
    // the grid from the cell image and never write one.
    if let Some(section) = find_section(sections, "Map") {
        set_entry(&mut section.entries, "Width", model.width.to_string());
        set_entry(&mut section.entries, "Height", model.height.to_string());
    }
}

fn upsert_waypoints(model: &MapModel, sections: &mut Vec<PreservedSection>, imported: bool) {
    if model.waypoints.is_empty() && !imported {
        return;
    }
    let mut entries = Vec::new();
    let mut waypoints = model.waypoints.clone();
    waypoints.sort_by_key(|waypoint| waypoint.id);
    for waypoint in waypoints {
        let index = waypoint.cell.y as u32 * model.width + waypoint.cell.x as u32;
        entries.push((waypoint.id.to_string(), index.to_string()));
    }
    upsert_section(sections, "Waypoints", entries);
}

fn house_ordinal(model: &MapModel, owner: Option<ClassId>) -> String {
    owner
        .and_then(|owner| {
            model
                .houses
                .iter()
                .position(|house| cnc_core::house_class_from_token(&house.name).ok() == Some(owner))
        })
        .map(|index| index.to_string())
        .unwrap_or_else(|| "0".to_string())
}

fn house_section_entries(dialect: ImportKind, house: &HouseEntry) -> Vec<(String, String)> {
    let mut entries = Vec::new();
    entries.push(("Credits".to_string(), (house.credits / 100).to_string()));
    if house.max_units != 150 {
        entries.push(("MaxUnit".to_string(), house.max_units.to_string()));
    }
    if house.max_buildings != 150 {
        entries.push(("MaxBuilding".to_string(), house.max_buildings.to_string()));
    }
    if !house.edge.is_empty() {
        entries.push(("Edge".to_string(), house.edge.clone()));
    }
    if dialect == ImportKind::OriginalRa {
        if house.tech_level > 0 {
            entries.push(("TechLevel".to_string(), house.tech_level.to_string()));
        }
        if house.max_infantry > 0 {
            entries.push(("MaxInfantry".to_string(), house.max_infantry.to_string()));
        }
        if house.max_vessel > 0 {
            entries.push(("MaxVessel".to_string(), house.max_vessel.to_string()));
        }
    }
    entries
}

fn generate_house_sections(
    model: &MapModel,
    dialect: ImportKind,
    sections: &mut Vec<PreservedSection>,
) {
    if model.houses.is_empty() {
        return;
    }
    let mut list = Vec::new();
    for (index, house) in model.houses.iter().enumerate() {
        list.push((index.to_string(), house.name.clone()));
    }
    upsert_section(sections, "Houses", list);
    for house in &model.houses {
        upsert_section(sections, &house.name, house_section_entries(dialect, house));
    }
}

fn inline_record(dialect: ImportKind, object: &MapObject, model: &MapModel) -> Option<String> {
    let cell = cell_index(model, object)?;
    let house = house_token(dialect, object.owner);
    let class = class_token(object.class_id);
    Some(if object.class_id.is_building() {
        format!("{house},{class},256,{cell},0,,1,1")
    } else if is_infantry(object.class_id) {
        format!("{house},{class},256,{cell},0,Guard,0,")
    } else {
        format!("{house},{class},256,{cell},0,Guard,")
    })
}

fn updated_inline_record(
    dialect: ImportKind,
    origin: &ObjectOrigin,
    object: &MapObject,
    model: &MapModel,
) -> String {
    let mut fields = origin.fields.clone();
    if fields.len() >= 4 {
        if let Some(cell) = cell_index(model, object) {
            if origin.cell != cell {
                fields[3] = cell.to_string();
            }
        }
        if origin.owner != object.owner {
            fields[0] = house_token(dialect, object.owner);
        }
        fields.join(",")
    } else {
        inline_record(dialect, object, model).unwrap_or_default()
    }
}

fn append_new_object(
    dialect: ImportKind,
    model: &MapModel,
    sections: &mut Vec<PreservedSection>,
    object: &MapObject,
) {
    let section_name = object_section(dialect, object.class_id);
    if cell_index(model, object).is_none() {
        return;
    }
    if dialect == ImportKind::OriginalTd {
        let stem = if object.class_id.is_building() {
            "STRUCT"
        } else {
            "UNIT"
        };
        let named = format!("{stem}_{}", object.id);
        let entries = vec![
            ("House".to_string(), house_ordinal(model, object.owner)),
            ("Type".to_string(), class_token(object.class_id)),
            ("X".to_string(), object.position.x.to_string()),
            ("Y".to_string(), object.position.y.to_string()),
            ("Facing".to_string(), "0".to_string()),
            ("Health".to_string(), "256".to_string()),
        ];
        upsert_section(sections, &named, entries);
        if !has_section(sections, section_name) {
            upsert_section(sections, section_name, Vec::new());
        }
        if let Some(section) = find_section(sections, section_name) {
            let key = next_parent_key(&section.entries);
            section.entries.push((key, named));
        }
        return;
    }
    let Some(record) = inline_record(dialect, object, model) else {
        return;
    };
    if !has_section(sections, section_name) {
        upsert_section(sections, section_name, Vec::new());
    }
    if let Some(section) = find_section(sections, section_name) {
        let key = next_parent_key(&section.entries);
        section.entries.push((key, record));
    }
}

fn rewrite_object_sections(
    model: &MapModel,
    dialect: ImportKind,
    sections: &mut Vec<PreservedSection>,
    origin: &OriginalImport,
) {
    // Inline records: rewrite in place, drop references to deleted objects and
    // leave entries the importer never understood untouched.
    for section in sections
        .iter_mut()
        .filter(|section| is_object_section(&section.name))
    {
        let mut entries = Vec::new();
        let section_name = section.name.clone();
        for (key, value) in section.entries.drain(..) {
            let Some(record_origin) = origin.objects.iter().find(|candidate| {
                candidate.section.eq_ignore_ascii_case(&section_name) && candidate.key == key
            }) else {
                entries.push((key, value));
                continue;
            };
            let Some(object) = model.objects.get(&record_origin.object_id) else {
                continue;
            };
            if record_origin.named_section.is_some() {
                // The value is a section reference; the data lives there.
                entries.push((key, value));
            } else {
                entries.push((
                    key,
                    updated_inline_record(dialect, record_origin, object, model),
                ));
            }
        }
        section.entries = entries;
    }
    // Named TD object sections: keep the section aligned with the model.
    for record_origin in &origin.objects {
        let Some(name) = &record_origin.named_section else {
            continue;
        };
        match model.objects.get(&record_origin.object_id) {
            Some(object) => {
                if let Some(section) = find_section(sections, name) {
                    set_entry(
                        &mut section.entries,
                        "House",
                        house_ordinal(model, object.owner),
                    );
                    set_entry(&mut section.entries, "X", object.position.x.to_string());
                    set_entry(&mut section.entries, "Y", object.position.y.to_string());
                }
            }
            None => remove_section(sections, name),
        }
    }
    let known: Vec<u32> = origin.objects.iter().map(|entry| entry.object_id).collect();
    let mut new_ids: Vec<u32> = model
        .objects
        .keys()
        .copied()
        .filter(|id| !known.contains(id))
        .collect();
    new_ids.sort_unstable();
    for id in new_ids {
        if let Some(object) = model.objects.get(&id) {
            append_new_object(dialect, model, sections, object);
        }
    }
}

fn generate_object_sections(
    model: &MapModel,
    dialect: ImportKind,
    sections: &mut Vec<PreservedSection>,
) {
    let mut ids: Vec<u32> = model.objects.keys().copied().collect();
    ids.sort_unstable();
    for id in ids {
        if let Some(object) = model.objects.get(&id) {
            append_new_object(dialect, model, sections, object);
        }
    }
}

impl MapModel {
    /// Dialect this model was imported from, when it came from a retail file.
    pub fn original_dialect(&self) -> Option<ImportKind> {
        self.original.as_ref().map(|original| original.dialect)
    }

    /// Write the model back as a retail RA `.mpr`.
    pub fn to_original_ra(&self) -> anyhow::Result<String> {
        self.build_original(ImportKind::OriginalRa)
            .map(|(text, _)| text)
    }

    /// Write the model back as a retail TD scenario and its cell image.
    pub fn to_original_td(&self) -> anyhow::Result<(String, Vec<u8>)> {
        let (text, bin) = self.build_original(ImportKind::OriginalTd)?;
        let bin = bin.expect("the TD writer always produces a cell image");
        Ok((text, bin))
    }

    /// Sections that are regenerated from the model rather than preserved.
    pub fn regenerated_sections(&self) -> &'static [&'static str] {
        REGENERATED
    }

    fn build_original(&self, dialect: ImportKind) -> anyhow::Result<(String, Option<Vec<u8>>)> {
        if !matches!(dialect, ImportKind::OriginalTd | ImportKind::OriginalRa) {
            bail!("'{}' is not an original-format dialect", dialect.label());
        }
        if self.width == 0 || self.height == 0 {
            bail!("cannot save an empty map as '{}'", dialect.label());
        }
        if self.width != self.height {
            bail!(
                "original-format maps are square, so {}x{} cannot be written",
                self.width,
                self.height
            );
        }
        let origin = self
            .original
            .as_ref()
            .filter(|original| original.dialect == dialect)
            .cloned();
        let mut sections = origin
            .as_ref()
            .map(|original| original.sections.clone())
            .unwrap_or_default();

        match &origin {
            Some(original) => rewrite_object_sections(self, dialect, &mut sections, original),
            None => {
                generate_house_sections(self, dialect, &mut sections);
                generate_object_sections(self, dialect, &mut sections);
            }
        }
        upsert_basic(self, &mut sections);
        upsert_map_dimensions(self, &mut sections);
        match dialect {
            ImportKind::OriginalRa => {
                upsert_section(&mut sections, "MapPack", ra_pack_entries(self, false)?);
                let overlay = ra_pack_entries(self, true)?;
                if overlay.is_empty() {
                    remove_section(&mut sections, "OverlayPack");
                } else {
                    upsert_section(&mut sections, "OverlayPack", overlay);
                }
            }
            ImportKind::OriginalTd => {}
            ImportKind::EditorDialect => unreachable!("rejected above"),
        }
        upsert_waypoints(self, &mut sections, origin.is_some());
        let bin = if dialect == ImportKind::OriginalTd {
            Some(td_bin(self)?)
        } else {
            None
        };
        Ok((render(&sections), bin))
    }
}
