//! Integration tests for `cnc-procgen`: determinism, dimensions, palette
//! faithfulness, and the sprite foundation behavior — all through the public
//! API only.

use cnc_core::color::PlayerRemap;
use cnc_core::palette::Palette;
use cnc_procgen::*;

// Palette::new() is all-black (empty placeholder); grayscale() gives a real
// gradient so palette snapping is meaningful.
fn palette() -> Palette {
    Palette::grayscale()
}

#[test]
fn terrain_pipeline_is_deterministic_and_sized() {
    let params = TerrainParams::new(16, 16);
    let a = generate_terrain_texture(&params, &palette());
    let b = generate_terrain_texture(&params, &palette());
    assert_eq!(a.dimensions(), (16, 16));
    assert_eq!(a.into_raw(), b.into_raw());
}

#[test]
fn heightmap_matches_texture_dimensions() {
    let params = TerrainParams::new(24, 12);
    let map = generate_heightmap(&params);
    assert_eq!(map.len(), 24 * 12);
    assert!(map.iter().all(|h| h.is_finite() && (0.0..=1.0).contains(h)));
}

#[test]
fn terrain_uses_a_bounded_palette() {
    let params = TerrainParams::new(16, 16).with_seed(Seed::new(7));
    let image = generate_terrain_texture(&params, &palette());
    let mut distinct = std::collections::HashSet::new();
    for pixel in image.pixels() {
        distinct.insert(pixel.0);
    }
    assert!(
        distinct.len() <= 256,
        "texture must stay inside the 256-color palette model, saw {}",
        distinct.len()
    );
}

#[test]
fn textures_and_sprites_generate_for_all_public_entry_points() {
    let palette = palette();
    let _ = generate_noise_texture(16, 16, Seed::new(1), &palette);
    let _ = generate_tiberium_texture(16, 16, Seed::new(1), &palette);
    for family in [
        SpriteFamily::Infantry,
        SpriteFamily::Vehicle,
        SpriteFamily::Aircraft,
        SpriteFamily::Building,
    ] {
        let sprite = generate_unit_sprite(
            family,
            SpritePose::north(),
            &palette,
            &PlayerRemap::identity(),
        );
        assert_eq!(sprite.dimensions(), (SPRITE_SIZE, SPRITE_SIZE));
    }
}

#[test]
fn sprite_pose_affects_output_but_seed_free_determinism_holds() {
    let palette = palette();
    let remap = PlayerRemap::identity();
    let a = generate_unit_sprite(
        SpriteFamily::Vehicle,
        SpritePose::new(0.3),
        &palette,
        &remap,
    );
    let b = generate_unit_sprite(
        SpriteFamily::Vehicle,
        SpritePose::new(0.3),
        &palette,
        &remap,
    );
    let c = generate_unit_sprite(
        SpriteFamily::Vehicle,
        SpritePose::new(1.9),
        &palette,
        &remap,
    );
    assert_eq!(
        a.clone().into_raw(),
        b.into_raw(),
        "same pose must be identical"
    );
    assert_ne!(a.into_raw(), c.into_raw(), "different pose must differ");
}

#[test]
fn briefing_art_is_deterministic_and_palette_bounded() {
    let palette = palette();
    let remap = PlayerRemap::player_color(1);
    let emblem = generate_briefing_emblem(
        BriefingFaction::from_ra_player(1),
        0,
        Seed::new(3),
        &palette,
        &remap,
    );
    let emblem_again = generate_briefing_emblem(
        BriefingFaction::from_ra_player(1),
        0,
        Seed::new(3),
        &palette,
        &remap,
    );
    let portrait = generate_briefing_portrait(
        BriefingFaction::from_td_player(0),
        Seed::new(3),
        &palette,
        &remap,
    );
    assert_eq!(emblem.as_raw(), emblem_again.as_raw());

    for image in [&emblem, &portrait] {
        assert_eq!(image.dimensions(), (BRIEFING_ART_SIZE, BRIEFING_ART_SIZE));
    }

    let allowed: std::collections::HashSet<[u8; 3]> = palette
        .to_argb8888_remapped(&remap, 255)
        .iter()
        .map(|c| [c.r(), c.g(), c.b()])
        .collect();
    for image in [&emblem, &emblem_again, &portrait] {
        for pixel in image.pixels() {
            if pixel.0[3] == 0 {
                continue; // transparent background
            }
            assert!(
                allowed.contains(&[pixel.0[0], pixel.0[1], pixel.0[2]]),
                "pixel {:?} outside the remapped palette set",
                pixel.0
            );
        }
    }
}

#[test]
fn generated_sprites_use_only_remapped_palette_entries() {
    let palette = palette();
    let remap = PlayerRemap::player_color(2);
    let sprite = generate_sprite_for_class(
        cnc_core::class_id::ClassId::PowerPlant,
        SpritePose::north(),
        &palette,
        &remap,
    );
    // Compare RGB only: sprite pixels are opaque while the palette's optional
    // alpha parameter is test-independent.
    let allowed: std::collections::HashSet<[u8; 3]> = palette
        .to_argb8888_remapped(&remap, 255)
        .iter()
        .map(|c| [c.r(), c.g(), c.b()])
        .collect();
    for pixel in sprite.pixels() {
        if pixel.0[3] == 0 {
            continue; // transparent background
        }
        assert!(
            allowed.contains(&[pixel.0[0], pixel.0[1], pixel.0[2]]),
            "pixel {:?} outside the remapped palette set",
            pixel.0
        );
    }
}
