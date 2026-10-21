//! The CNC32 asset-pack builder: generates every sprite category, composes
//! atlases, renders QA previews, validates integrity and writes
//! `manifest.json` + `validation_report.json`.
//!
//! Logic-faithful port of `build()` from `cnc32_asset_generator_v5.py`
//! (v5.5.0): identical category set, generation order, seed strings, ID
//! scheme, metadata keys, atlas layout (16 columns of 32×32), preview set
//! and validation gates. Output is deterministic within Rust but not
//! byte-identical to the Python pack (different PRNG and rasterizer).
//!
//! The builder stages into a sibling `.stage` directory and promotes it
//! over the output directory only after every validation passes, with a
//! backup-and-restore dance on replace — same failure semantics as Python.

pub mod audit;
pub mod generators;
pub mod manifest;
pub mod previews;

mod expansion;

pub use audit::{contrast_audit, ContrastReport};

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use serde::Serialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

use super::canvas::Canvas;
use super::connections::{
    audit_road_continuity, bridge, cliff, connected_road, connected_shore, road_mask_name, wall,
    DIRECTION_NAMES,
};
use super::terrain::{blob_masks, terrain_base, terrain_transition, BLOB_DIR_NAMES, TERRAIN_KINDS};
use super::units::{
    effect_duration_ms, effect_frame, frame_duration_ms, independent_turret_frame, state_frames,
    state_loops, unit_condition_frame, unit_frame, CONDITIONS, EFFECT_KINDS, FACTIONS, UNIT_STATES,
};
use super::{ATLAS_COLUMNS, TILE};

use generators::{
    building, decoration, resource, structure_footprint, BUILDING_CONDITIONS, DECORATION_PROPS,
    STRUCTURES,
};
use manifest::{
    AtlasRecord, DuplicateFrame, IntegrityValidation, Manifest, RoadContinuityReport, RoadFailure,
    SpriteRecord, TerrainAdjacencyValidation,
};

/// Validation report type returned by [`build`] (also written to
/// `validation_report.json`).
pub use manifest::ValidationReport;
use previews::{
    save_composed_map, save_contact_sheet, save_unit_condition_review, ComposedMapInputs, SheetItem,
};

/// Generator version carried in the manifest and report (matches the
/// Python pack's `VERSION`).
pub const GENERATOR_VERSION: &str = "5.5.0";

/// Manifest schema version (matches the Python pack).
pub const SCHEMA_VERSION: &str = "1.2.0";

/// Default stable seed (matches the Python CLI default).
pub const DEFAULT_SEED: &str = "cnc32";

/// Default contrast floor (matches the Python CLI default).
pub const DEFAULT_CONTRAST_THRESHOLD: f64 = 1.5;

/// The v5 base pack's planned sprite count (matches the Python dry-run
/// plan); the pack total is [`PLANNED_PACK_SPRITE_COUNT`].
pub const PLANNED_SPRITE_COUNT: usize = 3909;

/// Planned sprite count of the complete v6.2.1 pack: the v5 base plus the
/// five expansion layers (269 + 26 + 99 + 123 + 151 = 668).
pub const PLANNED_PACK_SPRITE_COUNT: usize = 4577;

/// Planned atlas count of the complete pack: the 14 base atlases plus two
/// each from v6.0 (campaign art + effects), v6.1 (briefing props + icons)
/// and v6.2 (UI icons + character portraits).
pub const PLANNED_PACK_ATLAS_COUNT: usize = 20;

/// Manifest schema version of the complete pack (stamped by the v6.1
/// layer; the v5 base writes 1.2.0).
pub const PACK_SCHEMA_VERSION: &str = "1.3.0";

/// Build configuration.
pub struct BuildOptions {
    pub out: PathBuf,
    pub seed: String,
    pub force: bool,
    pub contrast_threshold: f64,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            out: PathBuf::from("./cnc32_assets"),
            seed: DEFAULT_SEED.to_string(),
            force: false,
            contrast_threshold: DEFAULT_CONTRAST_THRESHOLD,
        }
    }
}

/// Pack build error.
#[derive(thiserror::Error, Debug)]
pub enum PackError {
    #[error("output already exists: {0} (use force to replace it)")]
    OutExists(PathBuf),
    #[error("contrast threshold must be >= 1.0")]
    Threshold,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("duplicate sprite id: {0}")]
    DuplicateId(String),
    #[error("unsafe sprite path: {0}")]
    UnsafePath(String),
    #[error("coverage failure for {category}: expected {expected}, got {got}")]
    Coverage {
        category: String,
        expected: usize,
        got: usize,
    },
    #[error("atlas/category coverage mismatch: {0} categories vs {1} atlases")]
    AtlasMismatch(usize, usize),
    #[error("integrity validation failed: {0}")]
    Integrity(String),
}

/// One generated sprite awaiting manifest assembly.
struct PendingSprite {
    id: String,
    category: &'static str,
    image: Canvas,
    meta: Map<String, Value>,
}

/// Removes the staged directory unless explicitly kept.
struct StageGuard(PathBuf);

impl Drop for StageGuard {
    fn drop(&mut self) {
        if !self.0.as_os_str().is_empty() {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn pixel_sha256(image: &Canvas) -> String {
    sha256_bytes(image.as_rgba().as_raw())
}

fn save_png(image: &Canvas, path: &Path) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let file = std::fs::File::create(path)?;
    let encoder = PngEncoder::new_with_quality(file, CompressionType::Best, FilterType::Adaptive);
    image
        .as_rgba()
        .clone()
        .write_with_encoder(encoder)
        .map_err(|e| match e {
            image::ImageError::IoError(io) => io,
            other => std::io::Error::other(other.to_string()),
        })
}

fn u(v: u64) -> Value {
    json!(v)
}

fn s(v: &str) -> Value {
    json!(v)
}

fn str_list(items: &[&str]) -> Value {
    Value::Array(items.iter().map(|v| json!(v)).collect())
}

fn connections_list(mask: u8) -> Value {
    let names: Vec<&str> = (0..4u8)
        .filter(|i| mask & (1 << i) != 0)
        .map(|i| DIRECTION_NAMES[i as usize])
        .collect();
    str_list(&names)
}

fn meta_from(pairs: &[(&str, Value)]) -> Map<String, Value> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}

fn push(
    sprites: &mut Vec<PendingSprite>,
    id: String,
    category: &'static str,
    image: Canvas,
    meta: Map<String, Value>,
) {
    sprites.push(PendingSprite {
        id,
        category,
        image,
        meta,
    });
}

/// Generate the full sprite set in pack order (Python `build()` lines
/// 588–753). Always emits [`PLANNED_SPRITE_COUNT`] sprites.
fn generate_sprites(seed: &str) -> Vec<PendingSprite> {
    let mut sprites = Vec::with_capacity(PLANNED_SPRITE_COUNT);
    let blob = blob_masks();

    // Complete directed terrain-pair transitions (20 pairs × 47 masks).
    for terrain in TERRAIN_KINDS {
        for other in TERRAIN_KINDS {
            if other == terrain {
                continue;
            }
            for &mask in blob {
                let tseed = format!("{seed}:terrain:{terrain}:{other}:{mask:02x}");
                let im = terrain_transition(terrain, mask, &tseed, Some(other))
                    .expect("valid terrain pair");
                let mask_bits = BLOB_DIR_NAMES
                    .iter()
                    .enumerate()
                    .map(|(i, name)| (name.to_string(), json!(mask & (1 << i) != 0)))
                    .collect::<Map<String, Value>>();
                let meta = meta_from(&[
                    ("terrain", json!(terrain)),
                    ("alternate_terrain", json!(other)),
                    ("mask", u(u64::from(mask))),
                    ("mask_bits", Value::Object(mask_bits)),
                    (
                        "semantics",
                        s("set bit = same-terrain neighbor; diagonals are valid only when both adjacent cardinal bits are set"),
                    ),
                ]);
                push(
                    &mut sprites,
                    format!("terrain/transitions/{terrain}_to_{other}/blob_{mask:02x}"),
                    "terrain_transition",
                    im,
                    meta,
                );
            }
        }
    }

    // Standalone base terrain variants.
    for terrain in TERRAIN_KINDS {
        for variant in 0..4u8 {
            let im = terrain_base(terrain, &format!("{seed}:base:{terrain}:{variant}"));
            let meta = meta_from(&[
                ("terrain", json!(terrain)),
                ("variant", u(u64::from(variant))),
            ]);
            push(
                &mut sprites,
                format!("terrain/base/{terrain}/variant_{variant:02}"),
                "terrain_base",
                im,
                meta,
            );
        }
    }

    // Resources.
    for kind in generators::RESOURCE_KINDS {
        for variant in 0..8u8 {
            let im = resource(kind, variant, seed);
            let meta = meta_from(&[
                ("resource", json!(kind)),
                ("variant", u(u64::from(variant))),
                ("footprint", json!([1, 1])),
            ]);
            push(
                &mut sprites,
                format!("resources/{kind}/variant_{variant:02}"),
                "resource",
                im,
                meta,
            );
        }
    }

    // Cliffs and walls (16 cardinal masks each).
    for mask in 0..16u8 {
        // cliff() appends ":cliff:{mask}" itself, matching Python's
        // f"{seed}:cliff:{mask}" exactly.
        let im = cliff(mask, seed);
        let meta = meta_from(&[
            ("mask", u(u64::from(mask))),
            ("connections", connections_list(mask)),
        ]);
        push(
            &mut sprites,
            format!("cliffs/cliff_{mask:02x}"),
            "cliff_connection",
            im,
            meta,
        );
    }
    for mask in 0..16u8 {
        let im = wall(mask, seed);
        let meta = meta_from(&[
            ("mask", u(u64::from(mask))),
            ("connections", connections_list(mask)),
        ]);
        push(
            &mut sprites,
            format!("walls/wall_{mask:02x}"),
            "wall_connection",
            im,
            meta,
        );
    }

    // Bridges.
    for orientation in ["horizontal", "vertical"] {
        for condition in ["intact", "damaged", "destroyed"] {
            let im = bridge(orientation, condition, seed);
            let meta = meta_from(&[
                ("orientation", json!(orientation)),
                ("condition", json!(condition)),
            ]);
            push(
                &mut sprites,
                format!("bridges/{orientation}/{condition}"),
                "bridge",
                im,
                meta,
            );
        }
    }

    // Map decorations.
    for prop in DECORATION_PROPS {
        for variant in 0..4u8 {
            let im = decoration(prop, variant, seed);
            let meta = meta_from(&[
                ("prop", json!(prop)),
                ("variant", u(u64::from(variant))),
                ("footprint", json!([1, 1])),
            ]);
            push(
                &mut sprites,
                format!("decorations/{prop}/variant_{variant:02}"),
                "decoration",
                im,
                meta,
            );
        }
    }

    // Faction buildings.
    for faction in FACTIONS {
        for (structure, shape, _) in STRUCTURES {
            for condition in BUILDING_CONDITIONS {
                let im = building(faction, structure, condition, seed);
                let footprint = structure_footprint(structure);
                let meta = meta_from(&[
                    ("faction", json!(faction)),
                    ("building", json!(structure)),
                    ("condition", json!(condition)),
                    ("footprint", json!([footprint[0], footprint[1]])),
                    ("silhouette", json!(shape)),
                    ("construction_stage", json!(condition)),
                ]);
                push(
                    &mut sprites,
                    format!("buildings/{faction}/{structure}/{condition}"),
                    "building",
                    im,
                    meta,
                );
            }
        }
    }

    // Roads (16 cardinal masks with descriptive names).
    for mask in 0..16u8 {
        let im = connected_road(mask, &format!("{seed}:road:{mask}"));
        let meta = meta_from(&[
            ("mask", u(u64::from(mask))),
            ("connections", connections_list(mask)),
            ("semantics", s("set bit = road continues through tile edge")),
        ]);
        push(
            &mut sprites,
            format!("roads/road_{mask:02x}_{}", road_mask_name(mask)),
            "road_connection",
            im,
            meta,
        );
    }

    // Shorelines.
    for mask in 0..16u8 {
        let im = connected_shore(mask, &format!("{seed}:shore:{mask}"));
        let meta = meta_from(&[
            ("land_edges", connections_list(mask)),
            ("semantics", s("set bit = land/sand shore on that edge")),
        ]);
        push(
            &mut sprites,
            format!("shoreline/water_landmask_{mask:02x}"),
            "shoreline",
            im,
            meta,
        );
    }

    // Full state/facing/frame unit animation matrix.
    for faction in FACTIONS {
        for (unit, states) in UNIT_STATES {
            for state in *states {
                let count = state_frames(state);
                for facing in 0..8u8 {
                    for frame in 0..count {
                        let im = unit_frame(faction, unit, facing, frame, state);
                        let meta = meta_from(&[
                            ("faction", json!(faction)),
                            ("unit", json!(unit)),
                            ("state", json!(state)),
                            ("facing", u(u64::from(facing))),
                            ("frame", u(u64::from(frame))),
                            ("frame_count", u(u64::from(count))),
                            ("duration_ms", u(u64::from(frame_duration_ms(state)))),
                            ("loop", json!(state_loops(state))),
                            ("pivot", json!([16, 16])),
                            ("animation", json!(state)),
                            ("pose_variant", json!(format!("{state}_{frame:02}"))),
                        ]);
                        push(
                            &mut sprites,
                            format!(
                                "units/{faction}/{unit}/{state}/facing_{facing}/frame_{frame:02}"
                            ),
                            "unit_animation",
                            im,
                            meta,
                        );
                    }
                }
            }
        }
    }

    // Full-frame damage condition matrix.
    for faction in FACTIONS {
        for (unit, _) in UNIT_STATES {
            for facing in 0..8u8 {
                for condition in CONDITIONS {
                    let im = unit_condition_frame(faction, unit, facing, condition)
                        .expect("valid condition");
                    let meta = meta_from(&[
                        ("faction", json!(faction)),
                        ("unit", json!(unit)),
                        ("condition", json!(condition)),
                        ("facing", u(u64::from(facing))),
                        ("pivot", json!([16, 16])),
                        ("frame_count", u(1)),
                        ("duration_ms", u(120)),
                        ("loop", json!(false)),
                        (
                            "semantics",
                            s("complete 32x32 sprite for unit damage condition"),
                        ),
                    ]);
                    push(
                        &mut sprites,
                        format!("units/{faction}/{unit}/condition_{condition}/facing_{facing}"),
                        "unit_condition",
                        im,
                        meta,
                    );
                }
            }
        }
    }

    // Independent turret overlays.
    for faction in FACTIONS {
        for unit in ["tank", "harvester", "artillery"] {
            for turret_facing in 0..8u8 {
                for recoil_frame in 0..3u8 {
                    let im = independent_turret_frame(faction, unit, turret_facing, recoil_frame);
                    let meta = meta_from(&[
                        ("faction", json!(faction)),
                        ("unit", json!(unit)),
                        ("turret_facing", u(u64::from(turret_facing))),
                        ("recoil_frame", u(u64::from(recoil_frame))),
                        ("frame_count", u(3)),
                        ("loop", json!(false)),
                        ("pivot", json!([16, 16])),
                        (
                            "semantics",
                            s("transparent turret overlay; rotate/aim independently of hull-facing sprite"),
                        ),
                    ]);
                    push(
                        &mut sprites,
                        format!(
                            "units/{faction}/{unit}/turret/facing_{turret_facing}/recoil_{recoil_frame}"
                        ),
                        "turret_animation",
                        im,
                        meta,
                    );
                }
            }
        }
    }

    // Effect animations.
    for (kind, count) in EFFECT_KINDS {
        for frame in 0..count {
            let im = effect_frame(kind, frame);
            let meta = meta_from(&[
                ("effect", json!(kind)),
                ("frame", u(u64::from(frame))),
                ("frame_count", u(u64::from(count))),
                ("duration_ms", u(u64::from(effect_duration_ms(kind)))),
                ("loop", json!(kind == "smoke")),
                ("pivot", json!([16, 16])),
            ]);
            push(
                &mut sprites,
                format!("effects/{kind}/frame_{frame:02}"),
                "effect_animation",
                im,
                meta,
            );
        }
    }

    debug_assert_eq!(sprites.len(), PLANNED_SPRITE_COUNT);
    sprites
}

/// (sprite id, atlas file name, atlas rect) assignment produced per sprite.
type AtlasAssignment = (String, String, [i64; 4]);

/// Compose per-category atlases (16 columns, sorted groups) and return the
/// atlas records plus per-sprite atlas assignments (Python lines 754–769).
fn compose_atlases(
    stage: &Path,
    sprites: &[PendingSprite],
) -> Result<(Vec<AtlasRecord>, Vec<AtlasAssignment>), PackError> {
    std::fs::create_dir_all(stage.join("atlases"))?;

    let mut groups: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (idx, sprite) in sprites.iter().enumerate() {
        groups.entry(sprite.category).or_default().push(idx);
    }
    let mut records = Vec::with_capacity(groups.len());
    let mut assignments: Vec<(String, String, [i64; 4])> = Vec::with_capacity(sprites.len());

    for (category, mut group) in groups {
        group.sort_by(|&a, &b| sprites[a].id.cmp(&sprites[b].id));
        let cols = ATLAS_COLUMNS;
        let rows = group.len().div_ceil(cols).max(1);
        let mut atlas = Canvas::transparent(cols as u32 * TILE, rows as u32 * TILE);
        for (i, &idx) in group.iter().enumerate() {
            let x = (i % cols) as i32 * TILE as i32;
            let y = (i / cols) as i32 * TILE as i32;
            atlas.alpha_composite(&sprites[idx].image, (x, y));
            assignments.push((
                sprites[idx].id.clone(),
                format!("{category}.png"),
                [x as i64, y as i64, TILE as i64, TILE as i64],
            ));
        }
        let atlas_name = format!("{category}.png");
        let atlas_path = stage.join("atlases").join(&atlas_name);
        save_png(&atlas, &atlas_path)?;
        let bytes = std::fs::read(&atlas_path)?;
        records.push(AtlasRecord {
            file: format!("atlases/{atlas_name}"),
            category: category.to_string(),
            columns: cols,
            rows,
            cell_size: [TILE, TILE],
            sprite_count: group.len(),
            sha256_file: sha256_bytes(&bytes),
        });
    }
    Ok((records, assignments))
}

/// Detect adjacent duplicate frames in unit and effect animations
/// (Python lines 867–878).
fn find_duplicate_frames(sprites: &[PendingSprite]) -> Vec<DuplicateFrame> {
    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (idx, sprite) in sprites.iter().enumerate() {
        let key = match sprite.category {
            "unit_animation" => {
                let get = |k: &str| sprite.meta[k].as_str().unwrap_or_default().to_string();
                format!(
                    "{}/{}/{}/facing_{}",
                    get("faction"),
                    get("unit"),
                    get("state"),
                    sprite.meta["facing"].as_u64().unwrap_or(0)
                )
            }
            "effect_animation" => sprite.meta["effect"]
                .as_str()
                .unwrap_or_default()
                .to_string(),
            _ => continue,
        };
        groups.entry(key).or_default().push(idx);
    }
    let mut duplicates = Vec::new();
    for (key, mut group) in groups {
        group.sort_by_key(|&idx| sprites[idx].meta["frame"].as_u64().unwrap_or(0));
        for pair in group.windows(2) {
            let (a, b) = (&sprites[pair[0]], &sprites[pair[1]]);
            if pixel_sha256(&a.image) == pixel_sha256(&b.image) {
                duplicates.push(DuplicateFrame {
                    animation: key.clone(),
                    frames: [
                        a.meta["frame"].as_u64().unwrap_or(0) as u8,
                        b.meta["frame"].as_u64().unwrap_or(0) as u8,
                    ],
                });
            }
        }
    }
    duplicates
}

/// Render every visual-QA preview into the stage directory
/// (Python lines 770–866). Returns the report's preview path list.
fn render_previews(stage: &Path, sprites: &[PendingSprite]) -> Result<Vec<String>, PackError> {
    let preview_dir = stage.join("previews");
    std::fs::create_dir_all(&preview_dir)?;

    let by_category = |cat: &str| -> Vec<usize> {
        sprites
            .iter()
            .enumerate()
            .filter(|(_, sp)| sp.category == cat)
            .map(|(i, _)| i)
            .collect()
    };

    // Compact terrain overview (first 96), cols=12, scale=3.
    let terrain_idx = by_category("terrain_transition");
    let head: Vec<SheetItem> = terrain_idx
        .iter()
        .take(96)
        .map(|&i| SheetItem {
            image: &sprites[i].image,
        })
        .collect();
    save_contact_sheet(
        &preview_dir.join("terrain_transitions_review.png"),
        &head,
        12,
        3,
    )?;

    // Per directed terrain-pair sheets (cols=7, scale=3).
    for terrain in TERRAIN_KINDS {
        for other in TERRAIN_KINDS {
            if other == terrain {
                continue;
            }
            let items: Vec<SheetItem> = terrain_idx
                .iter()
                .filter(|&&i| {
                    sprites[i].meta["terrain"].as_str() == Some(terrain)
                        && sprites[i].meta["alternate_terrain"].as_str() == Some(other)
                })
                .map(|&i| SheetItem {
                    image: &sprites[i].image,
                })
                .collect();
            save_contact_sheet(
                &preview_dir.join(format!("terrain_pairs/{terrain}_to_{other}.png")),
                &items,
                7,
                3,
            )?;
        }
    }

    // Road / shoreline mask sheets (cols=4, scale=6).
    let roads: Vec<SheetItem> = by_category("road_connection")
        .into_iter()
        .map(|i| SheetItem {
            image: &sprites[i].image,
        })
        .collect();
    save_contact_sheet(&preview_dir.join("road_masks_review.png"), &roads, 4, 6)?;
    let shores: Vec<SheetItem> = by_category("shoreline")
        .into_iter()
        .map(|i| SheetItem {
            image: &sprites[i].image,
        })
        .collect();
    save_contact_sheet(
        &preview_dir.join("shoreline_masks_review.png"),
        &shores,
        4,
        6,
    )?;

    // GDI tank move facing-0 animation strip (cols=4, scale=6).
    let tank: Vec<SheetItem> = by_category("unit_animation")
        .into_iter()
        .filter(|&i| {
            sprites[i].meta["faction"].as_str() == Some("gdi")
                && sprites[i].meta["unit"].as_str() == Some("tank")
                && sprites[i].meta["state"].as_str() == Some("move")
                && sprites[i].meta["facing"].as_u64() == Some(0)
        })
        .map(|i| SheetItem {
            image: &sprites[i].image,
        })
        .collect();
    save_contact_sheet(&preview_dir.join("tank_animation_review.png"), &tank, 4, 6)?;

    // Building faction/damage review (cols=8, scale=4).
    let buildings: Vec<SheetItem> = by_category("building")
        .into_iter()
        .map(|i| SheetItem {
            image: &sprites[i].image,
        })
        .collect();
    save_contact_sheet(
        &preview_dir.join("building_faction_damage_review.png"),
        &buildings,
        8,
        4,
    )?;

    // Unit damage-state review grid (scale=5).
    let mut condition_lookup: HashMap<(String, String, u8, String), &Canvas> = HashMap::new();
    for sprite in sprites.iter().filter(|sp| sp.category == "unit_condition") {
        condition_lookup.insert(
            (
                sprite.meta["faction"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                sprite.meta["unit"].as_str().unwrap_or_default().to_string(),
                sprite.meta["facing"].as_u64().unwrap_or(0) as u8,
                sprite.meta["condition"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            ),
            &sprite.image,
        );
    }
    let unit_order: Vec<&str> = UNIT_STATES.iter().map(|(unit, _)| *unit).collect();
    save_unit_condition_review(
        &preview_dir.join("unit_damage_state_review.png"),
        &condition_lookup,
        &unit_order,
        &FACTIONS,
        &CONDITIONS,
        5,
    )?;

    // 12×8 composed continuity map.
    let mut base_tiles: HashMap<&str, &Canvas> = HashMap::new();
    let mut road_tiles: HashMap<u8, &Canvas> = HashMap::new();
    let mut shore_tiles: HashMap<u8, &Canvas> = HashMap::new();
    for sprite in sprites.iter() {
        match sprite.category {
            "terrain_base" if sprite.meta["variant"].as_u64() == Some(0) => {
                if let Some(kind) = sprite.meta["terrain"].as_str() {
                    base_tiles.insert(kind, &sprite.image);
                }
            }
            "road_connection" => {
                road_tiles.insert(
                    sprite.meta["mask"].as_u64().unwrap_or(0) as u8,
                    &sprite.image,
                );
            }
            "shoreline" => {
                // Shoreline records carry land_edges (name list), not a
                // numeric mask — reconstruct the mask from the edge names.
                let mask = sprite.meta["land_edges"]
                    .as_array()
                    .map(|edges| {
                        edges
                            .iter()
                            .filter_map(|e| e.as_str())
                            .fold(0u8, |acc, name| {
                                acc | DIRECTION_NAMES
                                    .iter()
                                    .position(|d| *d == name)
                                    .map(|i| 1u8 << i)
                                    .unwrap_or(0)
                            })
                    })
                    .unwrap_or(0);
                shore_tiles.insert(mask, &sprite.image);
            }
            _ => {}
        }
    }
    save_composed_map(
        &preview_dir.join("composed_map_continuity_review.png"),
        &ComposedMapInputs {
            base_tiles,
            road_tiles,
            shore_tiles,
        },
    )?;

    let mut paths: Vec<String> = [
        "terrain_transitions_review.png",
        "road_masks_review.png",
        "shoreline_masks_review.png",
        "tank_animation_review.png",
        "building_faction_damage_review.png",
        "unit_damage_state_review.png",
        "composed_map_continuity_review.png",
    ]
    .iter()
    .map(|name| format!("previews/{name}"))
    .collect();
    for terrain in TERRAIN_KINDS {
        for other in TERRAIN_KINDS {
            if terrain != other {
                paths.push(format!("previews/terrain_pairs/{terrain}_to_{other}.png"));
            }
        }
    }
    Ok(paths)
}

/// Strict pack validation over the written stage: source PNGs, atlas
/// hashes, every atlas cell's pixel content, path safety, unique ids and
/// required category coverage (Python lines 914–971).
fn validate_stage(
    stage: &Path,
    records: &[SpriteRecord],
    atlas_records: &[AtlasRecord],
) -> Result<IntegrityValidation, PackError> {
    let mut ids: HashSet<&str> = HashSet::new();
    for record in records {
        if !ids.insert(record.id.as_str()) {
            return Err(PackError::DuplicateId(record.id.clone()));
        }
    }

    let mut by_atlas: HashMap<&str, &AtlasRecord> = HashMap::new();
    for record in atlas_records {
        by_atlas.insert(record.file.as_str(), record);
    }
    let mut atlas_images: HashMap<String, image::RgbaImage> = HashMap::new();

    for record in records {
        let rel = Path::new(&record.file);
        if rel.is_absolute()
            || rel
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(PackError::UnsafePath(record.file.clone()));
        }
        let path = stage.join(rel);
        let bytes = std::fs::read(&path)?;
        if sha256_bytes(&bytes) != record.sha256_file {
            return Err(PackError::Integrity(format!(
                "file hash mismatch: {}",
                record.id
            )));
        }
        let img = image::open(&path)?.into_rgba8();
        if img.width() != TILE || img.height() != TILE {
            return Err(PackError::Integrity(format!(
                "invalid dimensions: {} {}x{}",
                record.id,
                img.width(),
                img.height()
            )));
        }
        if sha256_bytes(img.as_raw()) != record.pixel_sha256 {
            return Err(PackError::Integrity(format!(
                "pixel hash mismatch: {}",
                record.id
            )));
        }

        let atlas_rel = format!("atlases/{}", record.atlas);
        let atlas_record = by_atlas.get(atlas_rel.as_str()).ok_or_else(|| {
            PackError::Integrity(format!(
                "missing atlas record for {}: {atlas_rel}",
                record.id
            ))
        })?;
        if !atlas_images.contains_key(&atlas_rel) {
            let atlas_path = stage.join(&atlas_rel);
            let atlas_bytes = std::fs::read(&atlas_path)?;
            if sha256_bytes(&atlas_bytes) != atlas_record.sha256_file {
                return Err(PackError::Integrity(format!(
                    "atlas file hash mismatch: {atlas_rel}"
                )));
            }
            atlas_images.insert(atlas_rel.clone(), image::open(&atlas_path)?.into_rgba8());
        }
        let atlas_img = atlas_images
            .get(&atlas_rel)
            .expect("atlas image loaded above");
        let [x, y, w, h] = record.atlas_rect;
        if w != i64::from(TILE)
            || h != i64::from(TILE)
            || x < 0
            || y < 0
            || x + w > i64::from(atlas_img.width())
            || y + h > i64::from(atlas_img.height())
        {
            return Err(PackError::Integrity(format!(
                "invalid atlas rectangle: {} -> {:?}",
                record.id, record.atlas_rect
            )));
        }
        let cell =
            image::imageops::crop_imm(atlas_img, x as u32, y as u32, w as u32, h as u32).to_image();
        if cell.as_raw() != img.as_raw() {
            return Err(PackError::Integrity(format!(
                "atlas cell differs from source sprite: {}",
                record.id
            )));
        }
    }

    // Required category coverage.
    let required: BTreeMap<&str, usize> = BTreeMap::from([
        ("terrain_transition", 940),
        ("road_connection", 16),
        ("shoreline", 16),
        ("cliff_connection", 16),
        ("wall_connection", 16),
        ("bridge", 6),
    ]);
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for record in records {
        *counts.entry(record.category.as_str()).or_default() += 1;
    }
    for (category, expected) in &required {
        let got = counts.get(category).copied().unwrap_or(0);
        if got != *expected {
            return Err(PackError::Coverage {
                category: (*category).to_string(),
                expected: *expected,
                got,
            });
        }
    }
    if atlas_records.len() != counts.len() {
        return Err(PackError::AtlasMismatch(counts.len(), atlas_records.len()));
    }
    let expected_conditions = 2 * UNIT_STATES.len() * 8 * CONDITIONS.len();
    let got_conditions = counts.get("unit_condition").copied().unwrap_or(0);
    if got_conditions != expected_conditions {
        return Err(PackError::Coverage {
            category: "unit_condition".to_string(),
            expected: expected_conditions,
            got: got_conditions,
        });
    }

    Ok(IntegrityValidation {
        unique_sprite_ids: ids.len(),
        source_files_verified: records.len(),
        atlas_files_verified: atlas_records.len(),
        atlas_cells_pixel_verified: records.len(),
        required_category_counts: required
            .iter()
            .map(|(k, v)| (k.to_string(), json!(*v)))
            .collect::<Map<String, Value>>(),
        category_counts: counts
            .iter()
            .map(|(k, v)| (k.to_string(), json!(*v)))
            .collect::<Map<String, Value>>(),
        pass: true,
    })
}

fn mask_convention() -> Map<String, Value> {
    let mut m = Map::new();
    m.insert("bit_order".to_string(), str_list(&DIRECTION_NAMES));
    m.insert("blob_bit_order".to_string(), str_list(&BLOB_DIR_NAMES));
    m.insert("roads".to_string(), s("set bit = connected road edge"));
    m.insert("shoreline".to_string(), s("set bit = land edge"));
    m.insert(
        "terrain_transition".to_string(),
        s("set bit = same-terrain neighbor; 47 valid blob masks per terrain"),
    );
    m
}

/// Build the complete asset pack at `opts.out`.
pub fn build(opts: &BuildOptions) -> Result<ValidationReport, PackError> {
    if opts.out.exists() && !opts.force {
        return Err(PackError::OutExists(opts.out.clone()));
    }
    if opts.contrast_threshold < 1.0 {
        return Err(PackError::Threshold);
    }
    let parent = opts
        .out
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;
    let out_name = opts
        .out
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("pack");
    let stage = parent.join(format!(".{out_name}.stage-{}", std::process::id()));
    if stage.exists() {
        std::fs::remove_dir_all(&stage)?;
    }
    std::fs::create_dir_all(&stage)?;
    let mut guard = StageGuard(stage.clone());

    // 1. Generate sprites in pack order.
    let sprites = generate_sprites(&opts.seed);

    // 2. Write sprite PNGs and collect manifest records.
    let mut records: Vec<SpriteRecord> = Vec::with_capacity(sprites.len());
    for sprite in &sprites {
        let rel = format!("sprites/{}.png", sprite.id);
        let path = stage.join(&rel);
        save_png(&sprite.image, &path)?;
        let bytes = std::fs::read(&path)?;
        records.push(SpriteRecord {
            id: sprite.id.clone(),
            category: sprite.category.to_string(),
            file: rel,
            size: [TILE, TILE],
            sha256_file: sha256_bytes(&bytes),
            pixel_sha256: pixel_sha256(&sprite.image),
            meta: sprite.meta.clone(),
            atlas: String::new(),
            atlas_rect: [0; 4],
        });
    }

    // 3. Atlases.
    let (atlas_records, assignments) = compose_atlases(&stage, &sprites)?;
    let mut by_id: HashMap<String, (String, [i64; 4])> = HashMap::new();
    for (id, atlas, rect) in assignments {
        by_id.insert(id, (atlas, rect));
    }
    for record in &mut records {
        let (atlas, rect) = by_id
            .remove(&record.id)
            .expect("every sprite gets an atlas slot");
        record.atlas = atlas;
        record.atlas_rect = rect;
    }

    // 4. QA previews.
    let visual_qa_previews = render_previews(&stage, &sprites)?;

    // 5. Audits.
    let duplicate_frames = find_duplicate_frames(&sprites);
    let road = audit_road_continuity(&format!("{}:road-continuity", opts.seed));
    let contrast = contrast_audit(opts.contrast_threshold);
    let contrast_failures = contrast.failures.len();

    // 6. Directed terrain-pair coverage.
    let mut directed_pairs: HashSet<(String, String)> = HashSet::new();
    let mut per_pair: HashMap<(String, String), usize> = HashMap::new();
    for record in &records {
        if record.category == "terrain_transition" {
            let pair = (
                record.meta["terrain"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                record.meta["alternate_terrain"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
            );
            directed_pairs.insert(pair.clone());
            *per_pair.entry(pair).or_default() += 1;
        }
    }
    let masks_per_pair_min = per_pair.values().copied().min().unwrap_or(0);
    let adjacency = TerrainAdjacencyValidation {
        directed_pairs_expected: 20,
        directed_pairs_found: directed_pairs.len(),
        masks_per_pair_expected: blob_masks().len(),
        masks_per_pair_min,
        pass: directed_pairs.len() == 20 && per_pair.values().all(|&c| c == blob_masks().len()),
    };

    // 7. Report assembly.
    let road_continuity = RoadContinuityReport {
        checks: road.checks,
        failures: road
            .failures
            .iter()
            .map(|f| RoadFailure {
                mask: f.mask,
                edge: f.edge.to_string(),
                connected: f.connected,
                edge_reached: f.edge_reached,
                pass: false,
            })
            .collect(),
        pass: road.pass,
    };
    let unit_state_definitions: Map<String, Value> = UNIT_STATES
        .iter()
        .map(|(unit, states)| (unit.to_string(), str_list(states)))
        .collect();
    let mut unit_condition_coverage = Map::new();
    for condition in CONDITIONS {
        let n = records
            .iter()
            .filter(|r| {
                r.category == "unit_condition" && r.meta["condition"].as_str() == Some(condition)
            })
            .count();
        unit_condition_coverage.insert(condition.to_string(), json!(n as u64));
    }
    let turret_overlay_count = records
        .iter()
        .filter(|r| r.category == "turret_animation")
        .count();
    let unit_condition_count = records
        .iter()
        .filter(|r| r.category == "unit_condition")
        .count();
    let sprite_count = records.len();
    let atlas_count = atlas_records.len();
    let terrain_pair_masks_previewed = records
        .iter()
        .filter(|r| r.category == "terrain_transition")
        .count();
    // Python's top-level gate counts directed pairs only; the stricter
    // 47-masks-per-pair coverage check lives inside
    // terrain_adjacency_validation.pass, not in the top-level pass.
    let pass = duplicate_frames.is_empty()
        && contrast_failures == 0
        && road.pass
        && adjacency.directed_pairs_found == adjacency.directed_pairs_expected;
    let mut report = ValidationReport {
        generator_version: GENERATOR_VERSION,
        tile_size: TILE,
        sprite_count,
        atlas_count,
        animation_duplicate_frames: duplicate_frames,
        contrast,
        road_continuity,
        terrain_adjacency_validation: adjacency,
        pass,
        warnings: vec![
            "Procedural C&C-inspired placeholders; not original Westwood artwork.".to_string(),
            "Contact sheets are provided under previews/ for visual QA.".to_string(),
            "Visual review in the target engine is still required for artistic quality and gameplay readability.".to_string(),
        ],
        visual_qa_previews,
        terrain_pair_preview_count: 20,
        terrain_pair_masks_previewed,
        terrain_blob_masks_per_family: blob_masks().len(),
        terrain_blob_mask_count: blob_masks().len() * 20,
        road_masks: 16,
        shoreline_masks: 16,
        independent_turret_frames: turret_overlay_count,
        unit_state_definitions,
        turret_overlay_count,
        unit_condition_count,
        unit_condition_coverage,
        contrast_failures,
        integrity_validation: None,
        expansion: None,
    };

    // 8. Manifest + README into the stage.
    let manifest = Manifest {
        schema_version: SCHEMA_VERSION,
        generator_version: GENERATOR_VERSION,
        seed: opts.seed.clone(),
        tile_size: TILE,
        mask_convention: mask_convention(),
        sprites: records,
        atlases: atlas_records,
        contrast_report: "validation_report.json",
    };
    write_json(&stage.join("manifest.json"), &manifest)?;
    write_readme(&stage.join("README.md"))?;

    // 8b. Expansion layers (v5.8 → v6.2) over the staged base pack, in
    // Python run order. Each layer mutates manifest.json and
    // sprites/cinematics/timeline.json in place and writes its own
    // artefacts; the base sprites above are never modified, so the base
    // integrity validation below still covers them exactly.
    report.expansion = Some(expansion::run(&stage)?);

    // 9. Integrity validation over the written files, then the report.
    let integrity = validate_stage(&stage, &manifest.sprites, &manifest.atlases)?;
    report.integrity_validation = Some(integrity);
    write_json(&stage.join("validation_report.json"), &report)?;

    // 10. Promote the stage over the output directory.
    promote(&stage, &opts.out, out_name)?;
    guard.0 = PathBuf::new();
    Ok(report)
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), PackError> {
    let text = serde_json::to_string_pretty(value)?;
    std::fs::write(path, format!("{text}\n"))?;
    Ok(())
}

/// Pack README (ported from the Python generator's text, adjusted to name
/// the Rust port).
fn write_readme(path: &Path) -> Result<(), PackError> {
    let text = format!(
        r#"# CNC32 generated asset pack

Generator: {GENERATOR_VERSION} (cnc-procgen Rust port)
Tile size: {TILE}x{TILE}

## Mask conventions
- Terrain transition: canonical 47-mask eight-neighbour blob set; bit order N, NE, E, SE, S, SW, W, NW; diagonals are constrained by adjacent cardinal bits.
- Road: a set N/E/S/W bit means the road connects through that edge.
- Shoreline: a set N/E/S/W bit means land/sand is on that edge.

## Animation
Unit poses are drawn direction-aware for eight facings and each unit's supported states (idle, move, turn, attack, harvest, unload, deploy, repair, death). Frame timings, loop flags, pivots and state names are in `manifest.json`. Inspect `validation_report.json` for duplicate-frame and palette contrast failures.

## Files
- `sprites/`: individual RGBA PNGs
- `atlases/`: category atlases, each cell 32x32
- `manifest.json`: sprite paths, hashes, frame metadata, pivots, and atlas rectangles
- `validation_report.json`: contrast audit and animation validation

This is a procedural C&C-inspired asset pack, not original game art. It is a logic-faithful Rust port of the CNC32 tileset generator (v5.5.0): same categories, mask conventions, seed layout and validation gates, rendered with the crate's pinned SplitMix64 PRNG, so pixel output is deterministic within Rust but not byte-identical to the Python pack. Inspect `previews/` before integrating; a visual preview is a QA aid, not proof every possible runtime scenario is seamless.
"#
    );
    std::fs::write(path, text)?;
    Ok(())
}

/// Promote the staged pack over `out`, replacing an existing directory
/// where the OS allows, with backup-and-restore on failure (Python lines
/// 973–984).
fn promote(stage: &Path, out: &Path, out_name: &str) -> Result<(), PackError> {
    let backup = out.with_file_name(format!("{out_name}.backup"));
    if backup.exists() {
        remove_any(&backup)?;
    }
    if out.exists() {
        std::fs::rename(out, &backup)?;
    }
    if let Err(e) = std::fs::rename(stage, out) {
        if backup.exists() && !out.exists() {
            let _ = std::fs::rename(&backup, out);
        }
        return Err(e.into());
    }
    if backup.exists() {
        let _ = remove_any(&backup);
    }
    Ok(())
}

fn remove_any(path: &Path) -> std::io::Result<()> {
    if path.is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
    }
}

/// Dry-run plan: planned sprite counts without writing anything (mirrors
/// the Python `--dry-run` output keys).
pub fn dry_run_plan() -> Value {
    let unit_frames: u64 = UNIT_STATES
        .iter()
        .flat_map(|(_, states)| states.iter())
        .map(|st| 2 * 8 * u64::from(state_frames(st)))
        .sum();
    let conditions = 2 * UNIT_STATES.len() as u64 * 8 * CONDITIONS.len() as u64;
    let other: u64 = 20 // base variants
        + 24 // decorations
        + 48 // buildings
        + 16 // cliffs
        + 16 // walls
        + 16 // roads
        + 6 // bridges
        + 16 // shoreline
        + 23 // effect frames
        + 144 // turrets
        + 16; // resources
    let planned = 20 * blob_masks().len() as u64 + unit_frames + conditions + other;
    let states: Map<String, Value> = UNIT_STATES
        .iter()
        .map(|(unit, unit_states)| (unit.to_string(), str_list(unit_states)))
        .collect();
    let expansion_layers = expansion::layer_plan();
    let expansion_sprites: u64 = expansion_layers
        .iter()
        .filter_map(|l| l.get("sprite_count").and_then(Value::as_u64))
        .sum();
    json!({
        "generator_version": GENERATOR_VERSION,
        "tile_size": TILE,
        "planned_sprite_count": PLANNED_PACK_SPRITE_COUNT,
        "planned_atlas_count": PLANNED_PACK_ATLAS_COUNT,
        "manifest_schema_version": PACK_SCHEMA_VERSION,
        "base_planned_sprite_count": planned,
        "base_planned_atlas_count": 14,
        "expansion_planned_sprite_count": expansion_sprites,
        "expansion_layers": expansion_layers,
        "unit_condition_sprites": conditions,
        "terrain_transitions": 20 * blob_masks().len() as u64,
        "valid_blob_masks_per_terrain": blob_masks().len() as u64,
        "road_masks": 16,
        "shoreline_masks": 16,
        "independent_turret_frames": 144,
        "unit_state_definitions": states,
        "unit_frames": unit_frames,
        "effect_frames": 23,
        "destruction_frames": 8,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dry_run_plan_matches_python_numbers() {
        let plan = dry_run_plan();
        // Pack totals: v5 base (3909) + five expansion layers (668).
        assert_eq!(plan["planned_sprite_count"], json!(4577));
        assert_eq!(plan["planned_atlas_count"], json!(20));
        assert_eq!(plan["base_planned_sprite_count"], json!(3909));
        assert_eq!(plan["base_planned_atlas_count"], json!(14));
        assert_eq!(plan["expansion_planned_sprite_count"], json!(668));
        assert_eq!(plan["manifest_schema_version"], "1.3.0");
        let layers = plan["expansion_layers"].as_array().expect("layers");
        assert_eq!(layers.len(), 5);
        assert_eq!(layers[0]["layer"], "v5.8");
        assert_eq!(layers[0]["seed"], json!(58017));
        assert_eq!(layers[4]["layer"], "v6.2");
        assert_eq!(layers[4]["seed"], json!(62021));
        // Base breakdown (Python v5.5.0 numbers).
        assert_eq!(plan["unit_frames"], json!(2176));
        assert_eq!(plan["unit_condition_sprites"], json!(448));
        assert_eq!(plan["terrain_transitions"], json!(940));
        assert_eq!(plan["valid_blob_masks_per_terrain"], json!(47));
        assert_eq!(plan["independent_turret_frames"], json!(144));
        assert_eq!(plan["effect_frames"], json!(23));
    }

    #[test]
    fn default_options_match_python_cli_defaults() {
        let opts = BuildOptions::default();
        assert_eq!(opts.seed, "cnc32");
        assert!((opts.contrast_threshold - 1.5).abs() < f64::EPSILON);
        assert!(!opts.force);
    }

    #[test]
    fn rejects_low_contrast_threshold() {
        let opts = BuildOptions {
            out: std::env::temp_dir().join("cnc-procgen-never"),
            seed: "s".to_string(),
            force: false,
            contrast_threshold: 0.5,
        };
        assert!(matches!(build(&opts), Err(PackError::Threshold)));
    }
}
