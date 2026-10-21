//! Runs the five v6.x expansion layers over a staged v5 base pack.
//!
//! The Python pack was assembled by applying five generator scripts to an
//! extracted v5-compatible pack, in this order:
//!
//! 1. `cutscene_generator_v5_8.py` — cinematics (six story sequences)
//! 2. `campaign_expansion_v5_9.py` — campaign art (twelve scenes)
//! 3. `campaign_expansion_v6_0.py` — campaign art and effects (ten scenes)
//! 4. `campaign_expansion_v6_1.py` — tactical briefings and debriefs
//! 5. `asset_expansion_v6_2.py` — portraits, UI icons, controls, mockups
//!
//! Every script mutates `manifest.json` and
//! `sprites/cinematics/timeline.json` in place and keeps the records of the
//! layers that ran before it. This module reproduces that: each layer runs
//! against the same staging directory, and afterwards the layer's
//! contributions are merged into the manifest and timeline that are on disk.
//!
//! The merge is deliberately tolerant of both authoring styles the ported
//! layers use: some write `manifest.json` and the timeline themselves
//! (v5.9, v6.0, v6.2), others only return their contributions (v5.8, v6.1).
//! After every layer the files are re-read from disk and the contribution is
//! merged on top, so the result does not depend on which style a layer used.
//! For the layers that write the files themselves the merge is idempotent —
//! their contributions mirror what they wrote.
//!
//! Merge rules (Python semantics):
//!
//! * sprite records — replace by `id`, append when new (base pack records
//!   keep their order; each layer's new records append after the previous
//!   layer's, matching the shipped pack);
//! * atlas entries — replace by `file`, append when new;
//! * timeline `sequences` / `campaigns` — replace by `id`, append when new;
//!   an empty campaign contribution leaves the key alone (Python v5.8
//!   creates no `campaigns` key, v6.1 preserves the five acts);
//! * timeline notes / supporting assets — de-duplicated union; assets are
//!   sorted like every Python layer that merges them;
//! * `cinematics` — merged key by key over the previous object (Python
//!   v6.1's `{**old_cinematics, ...}`; v5.9/v6.0 replace the whole key in
//!   their own manifest write, and their contributions mirror that), then
//!   the `sequence_count` / `frame_count` / `campaign_scene_count` counters
//!   are recomputed from the merged timeline — exactly how every Python
//!   layer that touches `cinematics` derives them. At the v5.8 stage this
//!   also stamps a `campaign_scene_count` of 0 (no campaigns yet); the
//!   v5.9 layer's own manifest write replaces the whole object, so the
//!   final pack matches Python.

use std::collections::HashSet;
use std::path::Path;

use serde_json::{json, Map, Value};

use crate::expansion::{
    briefing_v61, campaign_v59, campaign_v60, cutscene_v58, ui_v62, LayerOutput, PackWriter,
};
use crate::tileset::pack::manifest::ExpansionSummary;
use crate::tileset::pack::PackError;

/// Where the cinematics timeline lives inside the pack.
const TIMELINE_REL: &str = "sprites/cinematics/timeline.json";

/// A layer generator entry point.
type LayerFn = fn(&PackWriter, u64) -> Result<LayerOutput, PackError>;

/// The five layers in Python run order with their argparse default seeds.
const LAYERS: [(&str, u64, LayerFn); 5] = [
    ("v5.8", cutscene_v58::DEFAULT_SEED, cutscene_v58::generate),
    ("v5.9", campaign_v59::DEFAULT_SEED, campaign_v59::generate),
    ("v6.0", campaign_v60::DEFAULT_SEED, campaign_v60::generate),
    ("v6.1", briefing_v61::DEFAULT_SEED, briefing_v61::generate),
    ("v6.2", ui_v62::DEFAULT_SEED, ui_v62::generate),
];

/// Dry-run layer table: name, default seed and the deterministic sprite
/// output size of each Python script (269 = 233 cinematics + 22 previews +
/// 14 cinematic UI; 26 = 25 campaign art + 1 atlas sprite; 99 = 50 campaign
/// art + 48 effect frames + 1 atlas sprite; 123 briefing sprites; 151 UI
/// sprites). Sums to [`crate::tileset::pack::PLANNED_PACK_SPRITE_COUNT`]
/// minus the v5 base.
pub fn layer_plan() -> Vec<Value> {
    let planned: [u64; 5] = [269, 26, 99, 123, 151];
    LAYERS
        .iter()
        .zip(planned)
        .map(|((layer, seed, _), sprites)| {
            json!({"layer": layer, "seed": seed, "sprite_count": sprites})
        })
        .collect()
}

/// Apply the five expansion layers to `stage`, in Python run order, and
/// return the summary for the validation report.
///
/// `stage` must already contain the v5 base pack (sprites, atlases,
/// `manifest.json`): the layers read exactly the same starting point the
/// Python scripts did.
pub fn run(stage: &Path) -> Result<ExpansionSummary, PackError> {
    let writer = PackWriter::new(stage);

    // The base manifest decides which records count as expansion records
    // for the verification pass below.
    let base_manifest = read_manifest(&writer)?;
    let base_sprite_count = base_manifest
        .get("sprites")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let base_ids: HashSet<String> = base_manifest
        .get("sprites")
        .and_then(Value::as_array)
        .map(|sprites| {
            sprites
                .iter()
                .filter_map(|s| s.get("id").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();

    // Python run order; each script keeps its own argparse default seed.
    let mut manifest = base_manifest;
    // The timeline does not exist in the base pack; the v5.8 contribution
    // (canvas, pixel scale, header fields) creates it.
    let mut timeline = Value::Object(Map::new());

    for (_, seed, generate) in LAYERS {
        let output = generate(&writer, seed)?;
        // The layer may have rewritten the manifest and the timeline
        // itself; start from what is on disk so the merge never loses its
        // work.
        manifest = read_manifest(&writer)?;
        if let Some(on_disk) = writer.read_json(TIMELINE_REL)? {
            timeline = on_disk;
        }
        merge_layer(&mut manifest, &mut timeline, &output);
        writer.write_json("manifest.json", &manifest)?;
        writer.write_json(TIMELINE_REL, &timeline)?;
        // Standalone build reports live at the pack root (Python writes
        // them next to the manifest).
        for (rel, body) in &output.reports {
            writer.write_json(rel, body)?;
        }
    }

    let sprite_count = manifest
        .get("sprites")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let atlas_count = manifest
        .get("atlases")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let manifest_schema_version = manifest
        .get("schema_version")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let timeline_sequence_count = timeline
        .get("sequences")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let timeline_campaign_count = timeline
        .get("campaigns")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let verified_sprite_files = verify_expansion_sprites(&writer, &manifest, &base_ids)?;

    Ok(ExpansionSummary {
        layers: LAYERS.iter().map(|(name, _, _)| *name).collect(),
        seeds: LAYERS.iter().map(|(_, seed, _)| *seed).collect(),
        base_sprite_count,
        expansion_sprite_count: sprite_count.saturating_sub(base_sprite_count),
        sprite_count,
        atlas_count,
        manifest_schema_version,
        timeline_sequence_count,
        timeline_campaign_count,
        verified_sprite_files,
    })
}

fn read_manifest(writer: &PackWriter) -> Result<Value, PackError> {
    let mut manifest = writer
        .read_json("manifest.json")?
        .unwrap_or_else(|| Value::Object(Map::new()));
    if !manifest.is_object() {
        manifest = Value::Object(Map::new());
    }
    Ok(manifest)
}

/// Merge one layer's contributions into the manifest and the timeline.
///
/// The timeline is merged first: the manifest merge recomputes the
/// `cinematics` counters from the merged timeline.
fn merge_layer(manifest: &mut Value, timeline: &mut Value, output: &LayerOutput) {
    if let Some(contribution) = &output.timeline {
        let obj = timeline.as_object_mut().expect("timeline is an object");

        // Sequences: replace by id, append when new (Python merges by id).
        if !contribution.sequences.is_empty() {
            let mut sequences = obj
                .get("sequences")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            for record in &contribution.sequences {
                upsert_by_id(&mut sequences, record.clone());
            }
            obj.insert("sequences".to_string(), Value::Array(sequences));
        }

        // Campaigns: same, but an empty contribution leaves the key alone —
        // Python v5.8 creates no `campaigns` key and v6.1 preserves the
        // existing five acts.
        if !contribution.campaigns.is_empty() {
            let mut campaigns = obj
                .get("campaigns")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            for record in &contribution.campaigns {
                upsert_by_id(&mut campaigns, record.clone());
            }
            obj.insert("campaigns".to_string(), Value::Array(campaigns));
        }

        // Notes: de-duplicated, first occurrence wins (Python `dict.fromkeys`).
        let mut notes: Vec<String> = obj
            .get("notes")
            .and_then(Value::as_array)
            .map(|notes| {
                notes
                    .iter()
                    .filter_map(|n| n.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        for note in &contribution.notes {
            if !notes.iter().any(|n| n == note) {
                notes.push(note.clone());
            }
        }
        obj.insert(
            "notes".to_string(),
            Value::Array(notes.into_iter().map(Value::String).collect()),
        );

        // Supporting assets: the union of every layer's artefacts, sorted
        // like the Python layers that merge them.
        let mut supporting: Vec<String> = obj
            .get("supporting_assets")
            .and_then(Value::as_array)
            .map(|assets| {
                assets
                    .iter()
                    .filter_map(|a| a.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        for asset in &output.supporting_assets {
            if !supporting.iter().any(|a| a == asset) {
                supporting.push(asset.clone());
            }
        }
        supporting.sort();
        supporting.dedup();
        obj.insert(
            "supporting_assets".to_string(),
            Value::Array(supporting.into_iter().map(Value::String).collect()),
        );

        // Header fields the layer asserts.
        if !contribution.schema_version.is_empty() {
            obj.insert(
                "schema_version".to_string(),
                Value::String(contribution.schema_version.to_string()),
            );
        }
        if !contribution.generator_version.is_empty() {
            obj.insert(
                "generator_version".to_string(),
                Value::String(contribution.generator_version.to_string()),
            );
        }
        if let Some(seed) = contribution.seed {
            obj.insert("seed".to_string(), Value::from(seed));
        }
        for (key, value) in &contribution.extra {
            obj.insert(key.clone(), value.clone());
        }
    }

    let obj = manifest.as_object_mut().expect("manifest is an object");

    // Sprite records: replace by id, append when new (Python `register`).
    let mut sprites = obj
        .get("sprites")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for record in &output.sprites {
        upsert_by_id(&mut sprites, record.clone());
    }
    obj.insert("sprites".to_string(), Value::Array(sprites));

    // Atlas entries: replace by file, append when new (Python v6.0/v6.2).
    if !output.atlases.is_empty() {
        let mut atlases = obj
            .get("atlases")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for record in &output.atlases {
            let file = record.get("file").and_then(Value::as_str).unwrap_or("");
            atlases.retain(|a| a.get("file").and_then(Value::as_str) != Some(file));
            atlases.push(record.clone());
        }
        obj.insert("atlases".to_string(), Value::Array(atlases));
    }

    // Top-level keys. `cinematics` is a partial update in v6.1 (Python
    // `manifest["cinematics"] = {**old_cinematics, ...}`), so it merges key
    // by key over the previous object; every other key replaces.
    for (key, value) in &output.manifest_extra {
        if key == "cinematics" {
            let mut merged = obj
                .get("cinematics")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_default();
            if let Some(new) = value.as_object() {
                for (k, v) in new {
                    merged.insert(k.clone(), v.clone());
                }
            }
            recompute_cinematics_counters(&mut merged, timeline);
            obj.insert("cinematics".to_string(), Value::Object(merged));
        } else {
            obj.insert(key.clone(), value.clone());
        }
    }
}

/// Recompute the timeline-derived counters every Python layer that touches
/// `manifest["cinematics"]` derives from the merged timeline: the sequence
/// count, the total frame count over all sequences, and how many scenes the
/// campaign acts reference.
fn recompute_cinematics_counters(cinematics: &mut Map<String, Value>, timeline: &Value) {
    let sequences = timeline.get("sequences").and_then(Value::as_array);
    let sequence_count = sequences.map_or(0, Vec::len);
    let frame_count: usize = sequences.map_or(0, |seqs| {
        seqs.iter()
            .map(|s| {
                s.get("frames")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len)
            })
            .sum()
    });
    let campaign_scene_count: usize =
        timeline
            .get("campaigns")
            .and_then(Value::as_array)
            .map_or(0, |campaigns| {
                campaigns
                    .iter()
                    .map(|c| {
                        c.get("sequence_ids")
                            .and_then(Value::as_array)
                            .map_or(0, Vec::len)
                    })
                    .sum()
            });
    cinematics.insert("sequence_count".to_string(), json!(sequence_count));
    cinematics.insert("frame_count".to_string(), json!(frame_count));
    cinematics.insert(
        "campaign_scene_count".to_string(),
        json!(campaign_scene_count),
    );
}

/// Replace the record with the same `id`, append when the id is new.
fn upsert_by_id(list: &mut Vec<Value>, record: Value) {
    let id = record.get("id").and_then(Value::as_str).unwrap_or("");
    match list
        .iter_mut()
        .find(|r| r.get("id").and_then(Value::as_str) == Some(id))
    {
        Some(slot) => *slot = record,
        None => list.push(record),
    }
}

/// Verify every expansion sprite record against the file on disk.
///
/// The v5 base pack is verified by the base builder itself; this pass
/// covers the records the expansion layers added — every id that was not in
/// the base manifest. The file must exist, its SHA-256 must match the
/// manifest and its dimensions must match the recorded size. Any mismatch
/// fails the build.
fn verify_expansion_sprites(
    writer: &PackWriter,
    manifest: &Value,
    base_ids: &HashSet<String>,
) -> Result<usize, PackError> {
    let mut verified = 0usize;
    for record in manifest
        .get("sprites")
        .and_then(Value::as_array)
        .map_or(&[] as &[Value], Vec::as_slice)
    {
        let Some(id) = record.get("id").and_then(Value::as_str) else {
            continue;
        };
        if base_ids.contains(id) {
            continue;
        }
        let Some(rel) = record.get("file").and_then(Value::as_str) else {
            continue;
        };
        let path = writer.path(rel);
        if !path.is_file() {
            return Err(PackError::Integrity(format!(
                "expansion sprite is missing: {rel}"
            )));
        }
        let expected_hash = record
            .get("sha256_file")
            .and_then(Value::as_str)
            .unwrap_or("");
        let actual_hash = writer.sha256_file(rel)?;
        if expected_hash != actual_hash {
            return Err(PackError::Integrity(format!(
                "expansion sprite hash mismatch: {rel}"
            )));
        }
        if let Some(expected) = record.get("size").and_then(Value::as_array) {
            let want = (
                expected.first().and_then(Value::as_u64).unwrap_or(0) as u32,
                expected.get(1).and_then(Value::as_u64).unwrap_or(0) as u32,
            );
            match image::image_dimensions(&path) {
                Ok(got) if got == want => {}
                _ => {
                    return Err(PackError::Integrity(format!(
                        "expansion sprite size mismatch: {rel}"
                    )));
                }
            }
        }
        verified += 1;
    }
    Ok(verified)
}
