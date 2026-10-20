//! Manual-QA channel (data-shaped): end-to-end animation over a real
//! MIX+SHP+PAL fixture through the upgraded [`AssetRegistry`].
//!
//! Flow: pack a small uncompressed MIX archive containing a 3-frame SHP and a
//! palette, register the MIX bytes in the registry, extract + parse them,
//! build an [`AnimationSpec`] from the parsed frame count, advance it by
//! deterministic fixed ticks, then print the single QA line:
//!
//! ```text
//! anim frame=i/N looped=b palette=IDX
//! ```

use cnc_asset::{
    AnimationInstance, AnimationSpec, AssetRegistry, LoopMode, MixArchive, PalFile, ShpFile,
};

/// Pack one uncompressed MIX (orange textbook layout used by archive tests).
fn pack_uncompressed_mix(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"MIX\x00");
    out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    let mut names = Vec::new();
    let mut offsets = Vec::new();
    for (name, _) in entries {
        offsets.push(names.len() as u32);
        names.extend_from_slice(name.as_bytes());
        names.push(0);
    }
    for offset in offsets {
        out.extend_from_slice(&offset.to_le_bytes());
    }
    for (_, data) in entries {
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    }
    out.extend_from_slice(&names);
    for (_, data) in entries {
        out.extend_from_slice(data);
    }
    out
}

/// Build a valid untracked-SHP fixture: `frame_count` palette-index frames.
fn build_shp(frame_count: u16, width: u16, height: u16) -> Vec<u8> {
    let pixel_count = usize::from(width) * usize::from(height);
    let mut data = Vec::new();
    data.extend_from_slice(&0u16.to_le_bytes()); // version
    data.extend_from_slice(&0u16.to_le_bytes()); // flags
    data.extend_from_slice(&frame_count.to_le_bytes());
    data.extend_from_slice(&0u16.to_le_bytes()); // unknown
    for frame in 0..frame_count {
        let _ = frame;
        data.extend_from_slice(&width.to_le_bytes());
        data.extend_from_slice(&height.to_le_bytes());
        data.extend_from_slice(&0i16.to_le_bytes()); // offset_x
        data.extend_from_slice(&0i16.to_le_bytes()); // offset_y
    }
    let first_offset = data.len() as u32 + 4 * u32::from(frame_count);
    let mut offset = first_offset;
    for _ in 0..frame_count {
        data.extend_from_slice(&offset.to_le_bytes());
        offset += pixel_count as u32;
    }
    for frame in 0..frame_count {
        for pixel in 0..pixel_count as u16 {
            // Distinct grey ramp per frame so frames are visually writable.
            data.push((frame * 64 + pixel) as u8);
        }
    }
    data
}

#[test]
fn qa_anim_over_registered_mix_ships_one_line() {
    // --- fixture: 3-frame SHP + grayscale PAL packed into a MIX -----------
    let frame_count = 3;
    let (width, height) = (4u16, 2u16);
    let shp = build_shp(frame_count, width, height);
    let pal = PalFile::grayscale().to_bytes();
    let mix_bytes = pack_uncompressed_mix(&[("QA.SHP", &shp), ("QA.PAL", &pal)]);

    // --- registry: the existing keyed-bytes API, then real parsing -------
    let mut registry = AssetRegistry::new();
    registry.register("qa/level.mix".to_string(), mix_bytes.clone());
    let stored = registry.get("qa/level.mix").expect("registered");
    assert_eq!(&stored[..], &mix_bytes[..]);

    // Hot-reload semantics stay observable through the generation counter.
    assert_eq!(registry.generation("qa/level.mix"), Some(0));
    let handle = registry.handle("qa/level.mix").expect("handle");
    assert!(!handle.is_stale(&registry));

    let archive = MixArchive::from_bytes(registry.get("qa/level.mix").unwrap()).expect("mix");
    let shp = ShpFile::load(&archive.extract("QA.SHP").expect("shp")).expect("parse shp");
    let pal = PalFile::load(&archive.extract("QA.PAL").expect("pal")).expect("parse pal");
    assert_eq!(shp.frames.len(), frame_count as usize);
    assert_eq!(pal.colors[255].r, 255);

    // --- animation built from the parsed density -------------------------
    let n = shp.frames.len() as u32;
    // 0.125s/frame keeps every accumulated sum exact in f32 (pow2 fraction).
    let spec = AnimationSpec::new((0..n).collect(), 0.125, LoopMode::Loop);
    let mut anim = AnimationInstance::new(spec).with_palette_id(Some(7));

    // 16 ticks of 0.125s => k = 16 arrivals; wraps at k = 3,6,9,12,15.
    for _ in 0..16 {
        anim.advance(0.125);
    }
    let frame = anim.current_frame().expect("3-frame loop");
    let looped = anim.wrap_count() > 0;
    let palette_index = anim.palette_id().expect("remap carried");

    println!("anim frame={frame}/{n} looped={looped} palette={palette_index}");

    assert_eq!(frame, 1, "k=16 mod 3 -> position 1 -> frames[1]");
    assert!(looped, "five wraps across 16 arrivals");
    assert_eq!(palette_index, 7);
    assert_eq!(anim.wrap_count(), 5);
}
