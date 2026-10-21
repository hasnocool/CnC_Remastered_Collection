//! End-to-end integration test for the CNC32 tileset pack builder.
//!
//! Builds the complete pack — the v5 base (3909 sprites, 14 atlases) plus
//! the five v6.x expansion layers (668 sprites, 6 atlases → 4577 sprites,
//! 20 atlases, schema 1.3.0) — into a temp directory and checks the same
//! gates the Python generator enforces, plus the expansion-layer invariants
//! of the shipped `cnc32_assets_v6_2_1` pack: category histogram, manifest
//! record order, cinematics timeline, layer build reports and pack layout.
//!
//! Note: two full builds run here (determinism check); the test takes a
//! few minutes in debug mode.

use std::collections::BTreeMap;

use cnc_procgen::tileset::pack::{build, BuildOptions};
use serde_json::json;

fn scratch_dir(tag: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("cnc-procgen-pack-{tag}-{}", std::process::id()))
}

/// Category histogram of the shipped `cnc32_assets_v6_2_1` pack: the v5
/// base categories plus the five expansion layers' categories.
fn expected_category_counts() -> BTreeMap<&'static str, usize> {
    BTreeMap::from([
        // v5 base (3909 sprites)
        ("bridge", 6),
        ("building", 48),
        ("cliff_connection", 16),
        ("decoration", 24),
        ("effect_animation", 71), // 23 base + 48 v6.0
        ("resource", 16),
        ("road_connection", 16),
        ("shoreline", 16),
        ("terrain_base", 20),
        ("terrain_transition", 940),
        ("turret_animation", 144),
        ("unit_animation", 2176),
        ("unit_condition", 448),
        ("wall_connection", 16),
        // v5.8 cutscenes (269)
        ("cinematic", 233),
        ("cinematic_preview", 22),
        ("cinematic_ui", 14),
        // v5.9 + v6.0 campaign art (75 + 2 atlas sprites)
        ("campaign_art", 75),
        ("campaign_atlas", 2),
        // v6.1 briefings (123)
        ("briefing_atlas", 2),
        ("briefing_campaign_card", 8),
        ("briefing_frame", 48),
        ("briefing_icon", 32),
        ("briefing_overview", 1),
        ("briefing_personnel", 8),
        ("briefing_prop", 16),
        ("briefing_storyboard", 8),
        // v6.2 portraits / UI / controls (151)
        ("ui_v6_2", 151),
    ])
}

#[test]
fn full_pack_build_passes_all_gates() {
    let out = scratch_dir("full");
    let _ = std::fs::remove_dir_all(&out);
    let opts = BuildOptions {
        out: out.clone(),
        seed: "cnc32".to_string(),
        force: true,
        contrast_threshold: 1.5,
    };
    let report = build(&opts).expect("pack build must succeed");

    // Top-level gates (the Python `report["pass"]` condition, base pack).
    assert!(report.pass, "validation report must pass");
    assert!(report.animation_duplicate_frames.is_empty());
    assert!(report.contrast.failures.is_empty());
    assert!(report.road_continuity.pass);
    assert!(report.terrain_adjacency_validation.pass);
    let integrity = report
        .integrity_validation
        .as_ref()
        .expect("integrity block");
    assert!(integrity.pass);
    assert_eq!(integrity.unique_sprite_ids, 3909);
    assert_eq!(integrity.atlas_cells_pixel_verified, 3909);

    // Coverage numbers the Python builder hard-asserts (base pack).
    assert_eq!(report.sprite_count, 3909);
    assert_eq!(report.atlas_count, 14);
    assert_eq!(report.terrain_pair_masks_previewed, 940);
    assert_eq!(report.terrain_blob_masks_per_family, 47);
    assert_eq!(report.terrain_blob_mask_count, 940);
    assert_eq!(report.road_masks, 16);
    assert_eq!(report.shoreline_masks, 16);
    assert_eq!(report.independent_turret_frames, 144);
    assert_eq!(report.turret_overlay_count, 144);
    assert_eq!(report.unit_condition_count, 448);
    assert_eq!(report.visual_qa_previews.len(), 27); // 7 sheets + 20 pairs

    // Expansion summary (v5.8 → v6.2 in Python run order).
    let expansion = report.expansion.as_ref().expect("expansion summary");
    assert_eq!(expansion.layers, ["v5.8", "v5.9", "v6.0", "v6.1", "v6.2"]);
    assert_eq!(expansion.seeds, [58017, 59017, 60017, 61021, 62021]);
    assert_eq!(expansion.base_sprite_count, 3909);
    assert_eq!(expansion.expansion_sprite_count, 668);
    assert_eq!(expansion.sprite_count, 4577);
    assert_eq!(expansion.atlas_count, 20);
    assert_eq!(expansion.manifest_schema_version, "1.3.0");
    assert_eq!(expansion.timeline_sequence_count, 36);
    assert_eq!(expansion.timeline_campaign_count, 5);
    assert_eq!(expansion.verified_sprite_files, 668);

    // Manifest: schema, counts and the final layer stamps.
    let manifest_text = std::fs::read_to_string(out.join("manifest.json")).expect("manifest");
    let manifest: serde_json::Value =
        serde_json::from_str(&manifest_text).expect("manifest parses");
    assert_eq!(manifest["schema_version"], "1.3.0"); // stamped by v6.1
    assert_eq!(manifest["generator_version"], "6.2.1"); // stamped by v6.2
    assert_eq!(manifest["seed"], "cnc32");
    assert_eq!(manifest["tile_size"], 32);
    assert_eq!(manifest["contrast_report"], "validation_report.json");
    let sprites = manifest["sprites"].as_array().unwrap();
    assert_eq!(sprites.len(), 4577);
    assert_eq!(manifest["atlases"].as_array().unwrap().len(), 20);

    // The 16 top-level manifest keys of the shipped pack.
    let top_keys: Vec<&str> = manifest
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        top_keys,
        [
            "atlases",
            "briefings",
            "campaign_art_atlas",
            "campaign_art_atlas_v6",
            "cinematics",
            "commanders_v6_2",
            "contrast_report",
            "generator_version",
            "mask_convention",
            "npc_roster_v6_2",
            "release_note",
            "schema_version",
            "seed",
            "sprites",
            "tile_size",
            "ui_v6_2",
        ]
    );
    assert_eq!(
        manifest["release_note"].as_str().unwrap(),
        "v6.2.1 procedural rebuild: portraits, UI icons, controls, screens, \
         wireframes and contact sheets generated by Python/Pillow; no \
         image-model artwork required; all prior v6.1 assets retained."
    );

    // Category histogram matches the shipped pack exactly.
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for sprite in sprites {
        *counts
            .entry(sprite["category"].as_str().unwrap())
            .or_default() += 1;
    }
    assert_eq!(counts, expected_category_counts());

    // Sprite order: base pack first, then the expansion layers in run
    // order, v6.2's UI records last (the shipped pack's order).
    assert_eq!(
        sprites[0]["id"],
        "terrain/transitions/grass_to_dirt/blob_00"
    );
    assert_eq!(
        sprites[3909]["id"],
        "sprites/cinematics/frames/01_first_contact/frame_000"
    );
    assert_eq!(sprites[4576]["id"], "sprites/ui/icons/zoom_out");

    // Atlas order: 14 base atlases, then one atlas entry per expansion
    // layer (v6.0 campaign + effect, v6.1 briefing props + icons, v6.2 UI
    // icons + portraits).
    let atlas_files: Vec<&str> = manifest["atlases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["file"].as_str().unwrap())
        .collect();
    assert_eq!(
        atlas_files,
        [
            "atlases/bridge.png",
            "atlases/building.png",
            "atlases/cliff_connection.png",
            "atlases/decoration.png",
            "atlases/effect_animation.png",
            "atlases/resource.png",
            "atlases/road_connection.png",
            "atlases/shoreline.png",
            "atlases/terrain_base.png",
            "atlases/terrain_transition.png",
            "atlases/turret_animation.png",
            "atlases/unit_animation.png",
            "atlases/unit_condition.png",
            "atlases/wall_connection.png",
            "sprites/campaign_art/campaign_art_atlas_v6.png",
            "atlases/v6_effect_animation.png",
            "atlases/briefing_props_v6_1.png",
            "atlases/briefing_icons_v6_1.png",
            "atlases/ui_icons_v6_2.png",
            "atlases/character_portraits_v6_2.png",
        ]
    );

    // Base record metadata keys are unchanged by the expansion (Python
    // layers only append records).
    let sprite0 = &sprites[0];
    assert_eq!(sprite0["category"], "terrain_transition");
    assert_eq!(sprite0["size"][0], 32);
    assert!(sprite0["sha256_file"].as_str().unwrap().len() == 64);
    assert!(sprite0["pixel_sha256"].as_str().unwrap().len() == 64);
    assert!(sprite0["mask_bits"]["N"].is_boolean());
    assert_eq!(sprite0["atlas"], "terrain_transition.png");
    assert_eq!(sprite0["atlas_rect"][2], 32);

    // Cinematics block: the merged counters over all four contributing
    // layers (Python v6.1's `{**old, ...}` + recomputed totals).
    let cinematics = &manifest["cinematics"];
    assert_eq!(cinematics["sequence_count"], 36);
    assert_eq!(cinematics["frame_count"], 260);
    assert_eq!(cinematics["campaign_scene_count"], 22);
    assert_eq!(cinematics["campaign_art_count"], 72);
    assert_eq!(cinematics["new_portrait_count"], 8);
    assert_eq!(cinematics["new_campaign_scene_count"], 10);
    assert_eq!(cinematics["new_effect_animation_count"], 8);
    assert_eq!(cinematics["effects_frames_per_animation"], 6);
    assert_eq!(cinematics["new_briefing_scene_count"], 8);
    assert_eq!(cinematics["new_briefing_frame_count"], 48);
    assert_eq!(cinematics["timeline"], "sprites/cinematics/timeline.json");

    // Briefings block (v6.1).
    let briefings = &manifest["briefings"];
    assert_eq!(briefings["sequence_count"], 8);
    assert_eq!(briefings["frame_count"], 48);
    assert_eq!(briefings["campaign_card_count"], 8);
    assert_eq!(briefings["icon_count"], 32);
    assert_eq!(briefings["prop_count"], 16);
    assert_eq!(briefings["personnel_portrait_count"], 8);
    assert_eq!(
        briefings["props_atlas"]["image"],
        "atlases/briefing_props_v6_1.png"
    );
    assert_eq!(
        briefings["props_atlas"]["mapping"],
        "atlases/briefing_props_v6_1.json"
    );
    assert_eq!(briefings["props_atlas"]["sprite_count"], 16);
    assert_eq!(
        briefings["icons_atlas"]["image"],
        "atlases/briefing_icons_v6_1.png"
    );
    assert_eq!(
        briefings["icons_atlas"]["mapping"],
        "atlases/briefing_icons_v6_1.json"
    );
    assert_eq!(briefings["icons_atlas"]["sprite_count"], 32);

    // Campaign atlas keys (v5.9 + v6.0): atlas-record objects.
    assert_eq!(
        manifest["campaign_art_atlas"]["image"],
        "sprites/campaign_art/campaign_art_atlas.png"
    );
    assert_eq!(
        manifest["campaign_art_atlas"]["mapping"],
        "sprites/campaign_art/campaign_art_atlas.json"
    );
    assert_eq!(
        manifest["campaign_art_atlas_v6"]["image"],
        "sprites/campaign_art/campaign_art_atlas_v6.png"
    );
    assert_eq!(
        manifest["campaign_art_atlas_v6"]["mapping"],
        "sprites/campaign_art/campaign_art_atlas_v6.json"
    );

    // v6.2 roster/UI block: rosters are id -> record maps in the shipped pack.
    assert_eq!(manifest["commanders_v6_2"].as_object().unwrap().len(), 5);
    assert_eq!(manifest["npc_roster_v6_2"].as_object().unwrap().len(), 23);
    let ui = &manifest["ui_v6_2"];
    assert_eq!(ui["icon_count"], 110);
    assert_eq!(ui["control_components"].as_array().unwrap().len(), 13);
    assert_eq!(ui["screen_resolution"], json!([1600, 900]));

    // Cinematics timeline: v6.1's final state (36 sequences, 5 campaign
    // acts, briefings index).
    let timeline: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(out.join("sprites/cinematics/timeline.json")).expect("timeline"),
    )
    .expect("timeline parses");
    assert_eq!(timeline["schema_version"], "1.3.0");
    assert_eq!(timeline["generator_version"], "6.1.0");
    assert_eq!(timeline["seed"], 61021);
    assert_eq!(timeline["canvas"], json!([320, 180]));
    assert_eq!(timeline["pixel_art_scale"], 2);
    assert_eq!(timeline["sequences"].as_array().unwrap().len(), 36);
    assert_eq!(timeline["campaigns"].as_array().unwrap().len(), 5);
    assert_eq!(timeline["supporting_assets"].as_array().unwrap().len(), 368);
    assert_eq!(timeline["notes"].as_array().unwrap().len(), 21);
    let tl_briefings = &timeline["briefings"];
    assert_eq!(tl_briefings["sequence_count"], 8);
    assert_eq!(tl_briefings["frame_count"], 48);
    assert_eq!(tl_briefings["sequence_ids"].as_array().unwrap().len(), 8);
    assert_eq!(tl_briefings["cards"].as_array().unwrap().len(), 8);
    assert_eq!(
        tl_briefings["props_atlas"],
        "atlases/briefing_props_v6_1.png"
    );
    assert_eq!(
        tl_briefings["icons_atlas"],
        "atlases/briefing_icons_v6_1.png"
    );

    // Timeline sequence ids: 28 narrative scenes (numeric order) then the
    // 8 briefing scenes (the shipped pack's order).
    let seq_ids: Vec<&str> = timeline["sequences"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["id"].as_str().unwrap())
        .collect();
    assert_eq!(seq_ids[0], "01_first_contact");
    assert_eq!(seq_ids[27], "28_new_frontier");
    assert_eq!(seq_ids[28], "briefing_01_command_rundown");
    assert_eq!(seq_ids[35], "briefing_08_homecoming_relief");

    // Validation report on disk: base-scoped counts (Python parity) plus
    // the expansion summary block.
    let report_text =
        std::fs::read_to_string(out.join("validation_report.json")).expect("report file");
    let on_disk: serde_json::Value = serde_json::from_str(&report_text).expect("report parses");
    assert_eq!(on_disk["pass"], true);
    assert_eq!(on_disk["sprite_count"], 3909);
    assert_eq!(on_disk["atlas_count"], 14);
    assert_eq!(on_disk["expansion"]["sprite_count"], 4577);
    assert_eq!(on_disk["expansion"]["atlas_count"], 20);
    assert_eq!(on_disk["expansion"]["manifest_schema_version"], "1.3.0");

    // Pack layout: base + expansion artefacts, README, per-layer reports.
    assert!(out.join("README.md").exists());
    assert!(out
        .join("sprites/terrain/transitions/grass_to_dirt/blob_00.png")
        .exists());
    assert!(out.join("atlases/terrain_transition.png").exists());
    assert!(out
        .join("previews/composed_map_continuity_review.png")
        .exists());
    assert!(out.join("previews/unit_damage_state_review.png").exists());
    assert!(out
        .join("previews/terrain_pairs/grass_to_water.png")
        .exists());
    assert!(out
        .join("sprites/cinematics/frames/01_first_contact/frame_000.png")
        .exists());
    assert!(out
        .join("sprites/campaign_art/campaign_art_atlas_v6.png")
        .exists());
    assert!(out
        .join("sprites/animations_v6/muzzle_flash/frame_00.png")
        .exists());
    assert!(out
        .join("sprites/briefings/frames/briefing_01_command_rundown/frame_000.png")
        .exists());
    assert!(out.join("sprites/ui/icons/zoom_out.png").exists());
    assert!(out
        .join("sprites/ui/avatars/commanders/cmdr_hale.png")
        .exists());
    assert!(out.join("atlases/character_portraits_v6_2.png").exists());
    for preview in [
        "previews/v6_campaign_overview.png",
        "previews/v6_1_briefing_campaign_art_overview.png",
        "previews/v6_2_art_direction_board.png",
        "previews/v6_2_personnel_roster.png",
        "previews/v6_2_ui_mockup_contact_sheet.png",
        "previews/v6_2_wireframe_contact_sheet.png",
    ] {
        assert!(out.join(preview).exists(), "missing {preview}");
    }
    for report_file in [
        "cutscene_build_report_v5_8.json",
        "campaign_build_report_v5_9_0.json",
        "campaign_build_report_v6_0_0.json",
        "campaign_build_report_v6_1_0.json",
        "build_report_v6_2.json",
        "ui/ui_asset_index_v6_2.json",
        "ui/screen_flow_v6_2.json",
    ] {
        assert!(out.join(report_file).exists(), "missing {report_file}");
    }
    let screen_flow: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(out.join("ui/screen_flow_v6_2.json")).expect("screen flow"),
    )
    .expect("screen flow parses");
    assert_eq!(screen_flow["nodes"].as_array().unwrap().len(), 12);
    assert_eq!(screen_flow["edges"].as_array().unwrap().len(), 17);

    // Determinism: a second build with the same seed reproduces the
    // manifest and the timeline byte-for-byte (sprite content feeds the
    // recorded hashes).
    let first_manifest = manifest_text.clone();
    let first_timeline =
        std::fs::read_to_string(out.join("sprites/cinematics/timeline.json")).expect("timeline 1");
    let report2 = build(&opts).expect("second build");
    assert!(report2.pass);
    let manifest2 = std::fs::read_to_string(out.join("manifest.json")).expect("manifest 2");
    let timeline2 =
        std::fs::read_to_string(out.join("sprites/cinematics/timeline.json")).expect("timeline 2");
    assert_eq!(
        first_manifest, manifest2,
        "same seed must reproduce an identical manifest"
    );
    assert_eq!(
        first_timeline, timeline2,
        "same seed must reproduce an identical timeline"
    );

    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn existing_output_requires_force() {
    let out = scratch_dir("exists");
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).unwrap();
    let opts = BuildOptions {
        out: out.clone(),
        seed: "cnc32".to_string(),
        force: false,
        contrast_threshold: 1.5,
    };
    assert!(build(&opts).is_err(), "existing output must be rejected");
    let _ = std::fs::remove_dir_all(&out);
}
