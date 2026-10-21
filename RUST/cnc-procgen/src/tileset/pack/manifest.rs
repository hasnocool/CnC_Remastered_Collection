//! Serializable manifest and validation-report shapes.
//!
//! Mirrors the Python pack's `manifest.json` (schema 1.2.0) and
//! `validation_report.json` key-for-key so downstream tooling sees the same
//! structure.

use serde::Serialize;
use serde_json::{Map, Value};

use super::audit::ContrastReport;

/// One sprite record in `manifest.json` (metadata flattened in, like the
/// Python dict built by `add(...)`).
#[derive(Serialize)]
pub struct SpriteRecord {
    pub id: String,
    pub category: String,
    pub file: String,
    pub size: [u32; 2],
    pub sha256_file: String,
    pub pixel_sha256: String,
    #[serde(flatten)]
    pub meta: Map<String, Value>,
    pub atlas: String,
    pub atlas_rect: [i64; 4],
}

/// One category atlas record in `manifest.json`.
#[derive(Serialize)]
pub struct AtlasRecord {
    pub file: String,
    pub category: String,
    pub columns: usize,
    pub rows: usize,
    pub cell_size: [u32; 2],
    pub sprite_count: usize,
    pub sha256_file: String,
}

/// The pack manifest.
#[derive(Serialize)]
pub struct Manifest {
    pub schema_version: &'static str,
    pub generator_version: &'static str,
    pub seed: String,
    pub tile_size: u32,
    pub mask_convention: Map<String, Value>,
    pub sprites: Vec<SpriteRecord>,
    pub atlases: Vec<AtlasRecord>,
    pub contrast_report: &'static str,
}

/// An adjacent-frame duplicate detected in an animation group.
#[derive(Serialize)]
pub struct DuplicateFrame {
    pub animation: String,
    pub frames: [u8; 2],
}

/// Directed terrain-pair coverage summary.
#[derive(Serialize)]
pub struct TerrainAdjacencyValidation {
    pub directed_pairs_expected: usize,
    pub directed_pairs_found: usize,
    pub masks_per_pair_expected: usize,
    pub masks_per_pair_min: usize,
    pub pass: bool,
}

/// Integrity-validation block appended to the report.
#[derive(Serialize)]
pub struct IntegrityValidation {
    pub unique_sprite_ids: usize,
    pub source_files_verified: usize,
    pub atlas_files_verified: usize,
    pub atlas_cells_pixel_verified: usize,
    pub required_category_counts: Map<String, Value>,
    pub category_counts: Map<String, Value>,
    pub pass: bool,
}

/// Road-continuity audit entry in the report (matches the Python dict).
#[derive(Serialize)]
pub struct RoadContinuityReport {
    pub checks: usize,
    pub failures: Vec<RoadFailure>,
    pub pass: bool,
}

/// One road-continuity failure entry (schema matches the Python report,
/// where every failure carries `pass: false`).
#[derive(Serialize)]
pub struct RoadFailure {
    pub mask: u8,
    pub edge: String,
    pub connected: bool,
    pub edge_reached: bool,
    pub pass: bool,
}

/// v5.8–v6.2 expansion pass summary appended to the validation report.
///
/// The shipped pack was assembled by running five generator scripts over a
/// v5 base pack; the builder reproduces that pipeline in one pass and
/// reports what it produced here.
#[derive(Serialize)]
pub struct ExpansionSummary {
    /// Layer names in Python run order.
    pub layers: Vec<&'static str>,
    /// Per-layer seeds (the Python scripts' argparse defaults).
    pub seeds: Vec<u64>,
    /// Sprite records in the base manifest the expansion started from.
    pub base_sprite_count: usize,
    /// Sprite records the expansion layers added.
    pub expansion_sprite_count: usize,
    /// Sprite records in the final manifest.
    pub sprite_count: usize,
    /// Atlas entries in the final manifest.
    pub atlas_count: usize,
    /// Final manifest schema version (stamped by the v6.1 layer).
    pub manifest_schema_version: String,
    /// Sequences in the merged cinematics timeline.
    pub timeline_sequence_count: usize,
    /// Campaign acts in the merged cinematics timeline.
    pub timeline_campaign_count: usize,
    /// Expansion sprite files verified (existence, hash, dimensions).
    pub verified_sprite_files: usize,
}

/// The pack validation report (`validation_report.json`).
#[derive(Serialize)]
pub struct ValidationReport {
    pub generator_version: &'static str,
    pub tile_size: u32,
    pub sprite_count: usize,
    pub atlas_count: usize,
    pub animation_duplicate_frames: Vec<DuplicateFrame>,
    pub contrast: ContrastReport,
    pub road_continuity: RoadContinuityReport,
    pub terrain_adjacency_validation: TerrainAdjacencyValidation,
    pub pass: bool,
    pub warnings: Vec<String>,
    pub visual_qa_previews: Vec<String>,
    pub terrain_pair_preview_count: usize,
    pub terrain_pair_masks_previewed: usize,
    pub terrain_blob_masks_per_family: usize,
    pub terrain_blob_mask_count: usize,
    pub road_masks: usize,
    pub shoreline_masks: usize,
    pub independent_turret_frames: usize,
    pub unit_state_definitions: Map<String, Value>,
    pub turret_overlay_count: usize,
    pub unit_condition_count: usize,
    pub unit_condition_coverage: Map<String, Value>,
    pub contrast_failures: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrity_validation: Option<IntegrityValidation>,
    /// v5.8–v6.2 expansion pass summary; absent when the expansion pass did
    /// not run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expansion: Option<ExpansionSummary>,
}
