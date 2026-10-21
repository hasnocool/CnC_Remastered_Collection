//! Task 45 acceptance tests: VQA compressed-chunk decoders and the VXL
//! voxel decoder.
//!
//! * `video_decodes_compressed_variants` — a synthetic compressed VQA
//!   (`CBFZ` full codebook, `CBPZ` partial codebook, `VQFR` compressed
//!   frames, `SND1`/`SND2` ADPCM audio) decodes to exact expected frame
//!   pixels and exact expected PCM samples.
//! * `vxl_parses_and_converts_to_atlas` — a synthetic VXL parses to the
//!   expected volume and converts to atlas frames with the expected pixels.
//!
//! The expected values were derived from an independent reference
//! implementation of the documented rules (column-major Westwood block
//! expansion, pixel-level delta composition, z-descending layer
//! compositing) and are hard-coded here as concrete bytes and FNV-1a
//! hashes — never "it did not error".

use std::path::{Path, PathBuf};

use cnc_asset::format80;
use cnc_asset::video::{decode_frames, demux, demux_strict, VqaChunk, VqaError};
use cnc_asset::vxl::{VxlError, VxlFile};

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// FNV-1a (64-bit) so the frame hashes are concrete and comparable.
fn fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// One IFF chunk: tag, little-endian size, payload, pad byte when odd.
fn chunk(tag: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = tag.to_vec();
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
    if data.len() % 2 == 1 {
        out.push(0);
    }
    out
}

/// Wrap chunks in a `FORM`/`WVQA` container.
fn container(chunks: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"FORM");
    bytes.extend_from_slice(&[0, 0, 0, 0]);
    bytes.extend_from_slice(b"WVQA");
    for part in chunks {
        bytes.extend_from_slice(part);
    }
    let total = (bytes.len() - 8) as u32;
    bytes[4..8].copy_from_slice(&total.to_be_bytes());
    bytes
}

/// The `VQHD` payload for a 2x2-block, 3-frame movie.
fn vqhd(width_blocks: u16, height_blocks: u16, num_frames: u16) -> Vec<u8> {
    let mut header = Vec::new();
    header.extend_from_slice(&1u16.to_le_bytes()); // version
    header.extend_from_slice(&0u16.to_le_bytes()); // flags
    header.extend_from_slice(&num_frames.to_le_bytes());
    header.extend_from_slice(&15u16.to_le_bytes()); // frame rate
    header.extend_from_slice(&width_blocks.to_le_bytes());
    header.extend_from_slice(&height_blocks.to_le_bytes());
    header
}

/// A `CBFZ` chunk: a format80-compressed full codebook.
fn cbfz(entries: &[[u8; 4]]) -> Vec<u8> {
    let mut flat = Vec::with_capacity(entries.len() * 4);
    for entry in entries {
        flat.extend_from_slice(entry);
    }
    let packed = format80::compress(&flat, flat.len()).expect("compress full codebook");
    chunk(b"CBFZ", &packed)
}

/// A `CBPZ` chunk: a format80-compressed partial codebook.
fn cbpz(entries: &[[u8; 4]]) -> Vec<u8> {
    let mut flat = Vec::with_capacity(entries.len() * 4);
    for entry in entries {
        flat.extend_from_slice(entry);
    }
    let packed = format80::compress(&flat, flat.len()).expect("compress partial codebook");
    let mut payload = Vec::new();
    payload.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    payload.extend_from_slice(&0u16.to_le_bytes()); // reserved
    payload.extend_from_slice(&packed);
    chunk(b"CBPZ", &payload)
}

/// A `VQFR` chunk: a format80-compressed frame (one block byte per block).
fn vqfr(block_bytes: &[u8]) -> Vec<u8> {
    let packed = format80::compress(block_bytes, block_bytes.len()).expect("compress frame");
    chunk(b"VQFR", &packed)
}

// ---------------------------------------------------------------------------
// VQA compressed variants
// ---------------------------------------------------------------------------

/// The full codebook used by the synthetic movie: entry `i` holds the four
/// bytes `4i .. 4i+3` for the first four entries, zeros afterwards.
fn full_codebook() -> Vec<[u8; 4]> {
    let mut entries = vec![[0u8; 4]; 256];
    for (index, entry) in entries.iter_mut().take(4).enumerate() {
        *entry = [
            (index * 4) as u8,
            (index * 4 + 1) as u8,
            (index * 4 + 2) as u8,
            (index * 4 + 3) as u8,
        ];
    }
    entries
}

/// Build the synthetic compressed movie.
///
/// * frame 1 — `VQFR` against the `CBFZ` full codebook, blocks `[0,1,2,3]`;
/// * frame 2 — `CBPZ` partial codebook A, blocks `[0,1,1,0]`;
/// * frame 3 — `CBPZ` partial codebook B, blocks `[1, 0xFF, 1, 0]` where
///   `0xFF` marks block 1 as *unchanged* from frame 2.
fn synthetic_compressed_vqa() -> Vec<u8> {
    let partial_a = [[20u8, 21, 22, 23], [30, 31, 32, 33]];
    let partial_b = [[40u8, 41, 42, 43], [50, 51, 52, 53]];

    container(&[
        chunk(b"VQHD", &vqhd(2, 2, 3)),
        cbfz(&full_codebook()),
        vqfr(&[0, 1, 2, 3]),
        cbpz(&partial_a),
        vqfr(&[0, 1, 1, 0]),
        cbpz(&partial_b),
        vqfr(&[1, 0xFF, 1, 0]),
        // Mono ADPCM: 4 bytes -> 8 samples.
        chunk(b"SND1", &[0x12, 0x34, 0x56, 0x78]),
        // Stereo ADPCM: even byte -> left, odd byte -> right.
        chunk(b"SND2", &[0x10, 0x20]),
    ])
}

/// The expected pixel planes, derived from the documented Westwood layout
/// (column-major 2x2 blocks) and pixel-level delta composition.
fn expected_planes() -> Vec<Vec<u8>> {
    vec![
        // Frame 1: full codebook entries 0..3.
        vec![0, 1, 8, 9, 2, 3, 10, 11, 4, 5, 12, 13, 6, 7, 14, 15],
        // Frame 2: partial codebook A, blocks [0,1,1,0].
        vec![
            20, 21, 30, 31, 22, 23, 32, 33, 30, 31, 20, 21, 32, 33, 22, 23,
        ],
        // Frame 3: partial codebook B, blocks [1, unchanged, 1, 0].
        // Block 1 keeps frame 2's pixels (30,31 / 32,33) even though the
        // codebook changed underneath it — that is the delta composition.
        vec![
            50, 51, 50, 51, 52, 53, 52, 53, 30, 31, 40, 41, 32, 33, 42, 43,
        ],
    ]
}

#[test]
fn video_decodes_compressed_variants() {
    let bytes = synthetic_compressed_vqa();
    let vqa = demux(&bytes).expect("demux the compressed VQA");

    // --- chunk kinds -----------------------------------------------------
    assert_eq!(vqa.header.width_blocks, 2);
    assert_eq!(vqa.header.height_blocks, 2);
    assert_eq!(vqa.header.num_frames, 3);

    let full = vqa
        .chunks
        .iter()
        .find_map(|chunk| match chunk {
            VqaChunk::CodebookFull { data } => Some(data.clone()),
            _ => None,
        })
        .expect("the full codebook must be demuxed");
    assert_eq!(full.len(), 1024);
    assert_eq!(
        &full[0..16],
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );

    let partials: Vec<Vec<u8>> = vqa
        .chunks
        .iter()
        .filter_map(|chunk| match chunk {
            VqaChunk::CodebookPartial { entries } => Some(entries.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(partials.len(), 2);
    assert_eq!(partials[0], vec![20, 21, 22, 23, 30, 31, 32, 33]);
    assert_eq!(partials[1], vec![40, 41, 42, 43, 50, 51, 52, 53]);

    // --- decoded frames --------------------------------------------------
    let planes = decode_frames(&vqa).expect("decode the frames");
    let expected = expected_planes();
    assert_eq!(planes.len(), expected.len(), "one plane per frame");
    for (index, (plane, want)) in planes.iter().zip(expected.iter()).enumerate() {
        assert_eq!(plane.len(), 16, "frame {index} must be 4x4 pixels");
        assert_eq!(plane, want, "frame {index} pixels");
        assert_eq!(
            fnv1a(plane),
            match index {
                0 => 0x1420_a989_0845_9f25,
                1 => 0xd5fa_d421_b141_63f5,
                _ => 0x18fb_8ef2_4e1b_671d,
            },
            "frame {index} hash"
        );
    }

    // --- decoded audio ---------------------------------------------------
    let mono = vqa
        .chunks
        .iter()
        .find_map(|chunk| match chunk {
            VqaChunk::AudioAdpcm {
                sample_rate,
                channels,
            } if channels.len() == 1 => Some((*sample_rate, channels[0].clone())),
            _ => None,
        })
        .expect("the mono ADPCM track must be demuxed");
    assert_eq!(mono.0, 22050);
    assert_eq!(
        mono.1,
        vec![3, 4, 11, 18, 31, 49, 47, 81],
        "the ADPCM samples must match the original engine's algorithm"
    );

    let stereo = vqa
        .chunks
        .iter()
        .find_map(|chunk| match chunk {
            VqaChunk::AudioAdpcm { channels, .. } if channels.len() == 2 => Some(channels.clone()),
            _ => None,
        })
        .expect("the stereo ADPCM track must be demuxed");
    assert_eq!(stereo[0], vec![0, 1]);
    assert_eq!(stereo[1], vec![0, 3]);

    // --- determinism -----------------------------------------------------
    let again = decode_frames(&demux(&bytes).expect("demux again")).expect("decode again");
    assert_eq!(again, planes, "decoding the same bytes must be identical");
}

#[test]
fn an_unknown_chunk_id_is_an_error_naming_the_chunk() {
    let mut bytes = container(&[
        chunk(b"VQHD", &vqhd(2, 2, 1)),
        chunk(b"ZZZZ", &[1, 2, 3, 4]),
    ]);
    // Make sure the chunk tag is really the one we think it is:
    // FORM(4) + size(4) + "WVQA"(4) + VQHD chunk (4 + 4 + 12) = 32.
    assert_eq!(&bytes[32..36], b"ZZZZ");

    match demux_strict(&bytes) {
        Err(VqaError::UnknownChunk { tag }) => assert_eq!(&tag, b"ZZZZ"),
        other => panic!("expected a named unknown-chunk error, got {other:?}"),
    }

    // The lenient demuxer keeps the chunk instead of failing, and never
    // panics.
    let lenient = demux(&bytes).expect("lenient demux");
    assert!(matches!(lenient.chunks[0], VqaChunk::Other { .. }));

    // A truncated container is an error, not a panic.
    bytes.truncate(bytes.len() - 3);
    assert!(demux_strict(&bytes).is_err());
}

// ---------------------------------------------------------------------------
// VXL voxel decoder
// ---------------------------------------------------------------------------

/// Build the synthetic VXL: 4x3 voxels, 2 layers, 2 frames, 4 palette entries.
fn synthetic_vxl() -> Vec<u8> {
    let palette: [[u8; 3]; 4] = [[0, 0, 0], [10, 20, 30], [40, 50, 60], [70, 80, 90]];
    let layers: [[Vec<u8>; 2]; 2] = [
        // layer z = 0
        [
            vec![0, 1, 0, 2, 1, 0, 2, 0, 0, 0, 3, 0],
            vec![3, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 2],
        ],
        // layer z = 1
        [
            vec![0, 2, 0, 0, 0, 0, 1, 0, 3, 0, 0, 0],
            vec![0, 0, 3, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        ],
    ];

    let palette_offset: u32 = 32;
    let layer_offset: u32 = palette_offset + palette.len() as u32 * 3;
    let voxel_offset: u32 = layer_offset + layers.len() as u32 * 2;

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"VXL\0");
    bytes.extend_from_slice(&1u16.to_le_bytes()); // version
    bytes.extend_from_slice(&4u16.to_le_bytes()); // width
    bytes.extend_from_slice(&3u16.to_le_bytes()); // height
    bytes.extend_from_slice(&2u16.to_le_bytes()); // depth
    bytes.extend_from_slice(&2u16.to_le_bytes()); // num_layers
    bytes.extend_from_slice(&2u16.to_le_bytes()); // num_frames
    bytes.extend_from_slice(&(palette.len() as u16).to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes()); // reserved
    bytes.extend_from_slice(&palette_offset.to_le_bytes());
    bytes.extend_from_slice(&layer_offset.to_le_bytes());
    bytes.extend_from_slice(&voxel_offset.to_le_bytes());
    debug_assert_eq!(bytes.len(), 32);

    for entry in palette {
        bytes.extend_from_slice(&entry);
    }
    for layer in 0..layers.len() {
        bytes.extend_from_slice(&(layer as u16).to_le_bytes());
    }
    for layer in &layers {
        for frame in layer {
            bytes.extend_from_slice(frame);
        }
    }
    bytes
}

/// Read one frame's RGBA pixels out of an atlas, row by row.
fn atlas_frame_pixels(
    atlas: &cnc_asset::atlas::SpriteAtlas,
    frame: &cnc_asset::atlas::SpriteFrame,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(frame.w as usize * frame.h as usize * 4);
    for row in 0..frame.h as usize {
        let start = ((frame.y as usize + row) * atlas.width as usize + frame.x as usize) * 4;
        let end = start + frame.w as usize * 4;
        out.extend_from_slice(&atlas.pixels[start..end]);
    }
    out
}

#[test]
fn vxl_parses_and_converts_to_atlas() {
    let bytes = synthetic_vxl();
    let vxl = VxlFile::parse(&bytes).expect("parse the VXL");

    // --- the parsed volume ------------------------------------------------
    assert_eq!(vxl.header.width, 4);
    assert_eq!(vxl.header.height, 3);
    assert_eq!(vxl.header.depth, 2);
    assert_eq!(vxl.header.num_layers, 2);
    assert_eq!(vxl.header.num_frames, 2);
    assert_eq!(
        vxl.palette,
        vec![[0, 0, 0], [10, 20, 30], [40, 50, 60], [70, 80, 90]]
    );
    assert_eq!(vxl.layers.len(), 2);
    assert_eq!(vxl.layers[0].z, 0);
    assert_eq!(vxl.layers[1].z, 1);
    assert_eq!(
        vxl.layers[0].frames[0],
        vec![0, 1, 0, 2, 1, 0, 2, 0, 0, 0, 3, 0]
    );
    assert_eq!(
        vxl.layers[1].frames[1],
        vec![0, 0, 3, 0, 0, 0, 0, 0, 0, 1, 0, 0]
    );

    // --- the layer frames -------------------------------------------------
    let layer0 = vxl.layer_frame_rgba(0, 0).expect("layer 0 frame 0");
    assert_eq!(layer0.len(), 4 * 3 * 4);
    assert_eq!(
        layer0,
        vec![
            0, 0, 0, 0, // index 0 -> transparent
            10, 20, 30, 255, // index 1
            0, 0, 0, 0, // index 0
            40, 50, 60, 255, // index 2
            10, 20, 30, 255, // index 1
            0, 0, 0, 0, //
            40, 50, 60, 255, // index 2
            0, 0, 0, 0, //
            0, 0, 0, 0, //
            0, 0, 0, 0, //
            70, 80, 90, 255, // index 3
            0, 0, 0, 0, //
        ]
    );
    assert_eq!(fnv1a(&layer0), 0x80d5_7f9c_8c23_a90c);

    // --- the composited frames -------------------------------------------
    // Layers composite from the highest z to the lowest, so z = 0 ends up on
    // top and an empty voxel never overwrites a voxel behind it.
    let composite0 = vxl.frame_rgba(0).expect("composite frame 0");
    assert_eq!(fnv1a(&composite0), 0xcfc7_785b_e3e7_604d);
    assert_eq!(composite0[4..8], [10, 20, 30, 255]); // layer 0 wins at pixel 1
    assert_eq!(composite0[12..16], [40, 50, 60, 255]); // layer 0 wins at pixel 3
    assert_eq!(composite0[32..36], [70, 80, 90, 255]); // layer 1 shows through at pixel 8

    let composite1 = vxl.frame_rgba(1).expect("composite frame 1");
    assert_eq!(fnv1a(&composite1), 0xaa95_e558_e255_a535);

    // --- the atlas --------------------------------------------------------
    let atlas = vxl.to_atlas().expect("atlas");
    assert_eq!(
        atlas.frames.len(),
        6,
        "two layers x two frames plus two composites"
    );
    for key in [
        "layer0/frame0",
        "layer0/frame1",
        "layer1/frame0",
        "layer1/frame1",
        "frame0",
        "frame1",
    ] {
        let frame = atlas.frame(key).expect("atlas frame");
        assert_eq!((frame.w, frame.h), (4, 3), "frame {key} size");
    }

    // The atlas pixels for "layer0/frame0" must equal the layer's RGBA.
    // (The atlas row is wider than one frame, so the frame is read row by
    // row rather than as one contiguous slice.)
    let frame = atlas.frame("layer0/frame0").expect("atlas frame");
    assert_eq!(atlas_frame_pixels(&atlas, &frame), layer0);

    // --- determinism ------------------------------------------------------
    let again = VxlFile::parse(&bytes).expect("parse again");
    assert_eq!(again, vxl);
    let atlas_again = again.to_atlas().expect("atlas again");
    assert_eq!(atlas_again.pixels, atlas.pixels);
    assert_eq!(atlas_again.frames, atlas.frames);
}

#[test]
fn a_malformed_vxl_is_rejected_by_name() {
    let bytes = synthetic_vxl();

    // Bad magic.
    let mut bad = bytes.clone();
    bad[0] = b'X';
    match VxlFile::parse(&bad) {
        Err(VxlError::BadMagic { .. }) => {}
        other => panic!("expected a bad-magic error, got {other:?}"),
    }

    // Truncated voxel section.
    let truncated = &bytes[..bytes.len() - 4];
    assert!(matches!(
        VxlFile::parse(truncated),
        Err(VxlError::Truncated { field: "voxel", .. })
    ));

    // A voxel that points outside the palette.
    let mut bad = bytes.clone();
    let voxel_start = bad.len() - 48;
    bad[voxel_start] = 200;
    assert!(matches!(
        VxlFile::parse(&bad),
        Err(VxlError::PaletteIndexOutOfRange { index: 200, .. })
    ));

    // Too short for a header.
    assert!(matches!(
        VxlFile::parse(&[0u8; 8]),
        Err(VxlError::TooShort { .. })
    ));
}

// ---------------------------------------------------------------------------
// env-gated real-file golden test
// ---------------------------------------------------------------------------

/// Collect `*.vqa` files under `root`, bounded depth.
fn collect_vqa(root: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    if depth == 0 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_vqa(&path, depth - 1, out);
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("vqa"))
        {
            out.push(path);
        }
    }
}

/// Real original `.VQA` files decode to frames and audio.
///
/// The test is **skipped** (not failed) when `CNC_ORIGINAL_DATA_DIR` is
/// unset, missing, or does not contain any `.vqa` file — the original game
/// data is non-redistributable and is never committed.
#[test]
fn real_vqa_files_decode_when_the_original_data_is_available() {
    let Ok(dir) = std::env::var("CNC_ORIGINAL_DATA_DIR") else {
        eprintln!("skipping: CNC_ORIGINAL_DATA_DIR is not set");
        return;
    };
    let root = Path::new(&dir);
    if !root.is_dir() {
        eprintln!("skipping: CNC_ORIGINAL_DATA_DIR={dir:?} is not a directory");
        return;
    }
    let mut files = Vec::new();
    collect_vqa(root, 4, &mut files);
    files.sort();
    if files.is_empty() {
        eprintln!("skipping: no .vqa files under {dir:?}");
        return;
    }

    for path in files {
        let bytes = std::fs::read(&path).expect("read the VQA file");
        let vqa = demux(&bytes)
            .unwrap_or_else(|error| panic!("{} failed to demux: {error}", path.display()));

        let planes = decode_frames(&vqa)
            .unwrap_or_else(|error| panic!("{} failed to decode: {error}", path.display()));
        assert!(
            !planes.is_empty(),
            "{} must decode to at least one frame",
            path.display()
        );
        for plane in &planes {
            assert_eq!(
                plane.len(),
                usize::from(vqa.header.width_blocks)
                    * 2
                    * usize::from(vqa.header.height_blocks)
                    * 2
            );
        }

        // The audio track must decode when the file carries one.
        for chunk in &vqa.chunks {
            if let VqaChunk::AudioAdpcm { channels, .. } = chunk {
                assert!(
                    !channels.is_empty(),
                    "{} has an empty ADPCM track",
                    path.display()
                );
                for samples in channels {
                    assert!(
                        samples
                            .iter()
                            .all(|sample| (-32768..=32767).contains(sample)),
                        "{} has an out-of-range ADPCM sample",
                        path.display()
                    );
                }
            }
        }
    }
}
