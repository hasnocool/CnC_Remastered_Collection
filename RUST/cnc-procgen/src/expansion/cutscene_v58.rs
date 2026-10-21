//! Port of `cutscene_generator_v5_8.py` — v5.8 cutscene sequences, overlays, wipes, portrait/objective cards.
//!
//! Deterministic procedural pixel-art cutscene generator: six story
//! sequences of six `320×180` frames each, plus reusable UI layers
//! (overlays, transition wipes, portrait cards, objective cards) under
//! `sprites/cinematic_ui/` and storyboard contact sheets under
//! `sprites/cinematics/previews/`. Structure, ids, file paths, seed
//! derivation (`seed + si*1009 + fi*97`), palette, draw order and draw
//! counts are ported call-for-call from the Python source.
//!
//! Accepted deviations (field-tested pack semantics, not "fixes"):
//! - SplitMix64 ([`Rng`]) instead of Python `MT19937`: same draw counts,
//!   order and ranges, different values. Both `_scene_base` and the frame
//!   loop seed their own `Random(s)` exactly like the Python script.
//! - [`Canvas`] rasterizer instead of Pillow (line stamping, polygon
//!   scanline fill, ellipse rasterization differ at pixel level).
//! - JSON object key order is alphabetical here vs Python insertion order.
//! - `sha256`/`sha256_file` are computed over this crate's own PNG output.
//!
//! Like the Python script, `_scene_base` uses an internal `Random(s)` and
//! `generate` passes an independently seeded `Random(s)` for film grain,
//! so the two streams must stay distinct ([`scene_base`] builds its own
//! [`Rng`]; `generate` builds the grain one).

use std::collections::HashSet;

use serde_json::{json, Map, Value};

use super::{LayerOutput, PackWriter, SpriteSpec, TimelineContribution};
use crate::prng::{Rng, Seed};
use crate::tileset::canvas::Canvas;
use crate::tileset::pack::PackError;
use crate::tileset::{Rgba, TileRngExt};

/// Python `generate(..., seed=58_017)` default seed (independent of the
/// pack's `--seed`, exactly as the script was run to build the real pack).
pub const DEFAULT_SEED: u64 = 58_017;

/// Script identity the Python pack carries in every sprite record.
const GENERATED_BY: &str = "cutscene_generator_v5_8.py";

/// Python `manifest["generator_version"]` for this layer.
const GENERATOR_VERSION: &str = "5.8.0";

/// Scene canvas in scene pixels (`W, H, SCALE = 160, 90, 2`).
const W: i32 = 160;
const H: i32 = 90;
/// Nearest-neighbour pixel-art upscale factor.
const SCALE: u32 = 2;
/// Final frame size `OUT_SIZE = (W*SCALE, H*SCALE)`.
const OUT_W: u32 = (W as u32) * SCALE;
const OUT_H: u32 = (H as u32) * SCALE;

// The script's own palette dict, verbatim (alpha 255 on opaque surfaces).
const SKY0: Rgb = [11, 19, 35];
const SKY1: Rgb = [27, 39, 56];
const GROUND: Rgba = [54, 67, 55, 255];
const DARK: Rgba = [19, 26, 29, 255];
const STEEL: Rgba = [87, 101, 102, 255];
const STEEL_HI: Rgba = [142, 151, 139, 255];
const OLIVE: Rgba = [91, 105, 59, 255];
const OLIVE_HI: Rgba = [145, 150, 83, 255];
const RED: Rgba = [172, 63, 47, 255];
const ORANGE: Rgba = [239, 147, 65, 255];
const YELLOW: Rgba = [238, 203, 105, 255];
const WATER: Rgba = [28, 69, 83, 255];
const FOAM: Rgba = [105, 144, 142, 255];
const WHITE: Rgba = [218, 218, 194, 255];
const SKIN: Rgba = [177, 133, 94, 255];
const PANEL: Rgba = [14, 21, 27, 255];
const GREEN: Rgba = [112, 148, 99, 255];
const BLACK: Rgba = [5, 8, 10, 255];

/// Opaque RGBA view of an RGB triple (the scene canvas is opaque RGB).
const fn rgb(c: Rgb) -> Rgba {
    [c[0], c[1], c[2], 255]
}

/// RGB triple, as the palette entries feed the sky gradient math.
type Rgb = [u8; 3];

// One-off colors the script hardcodes outside the palette dict.
const RIDGE: Rgba = [35, 48, 48, 255];
const SMOKE: Rgba = [46, 53, 53, 255];
const DAWN_BAND: Rgba = [91, 73, 57, 255];
const MOON: Rgba = [178, 188, 174, 255];
const STARS: Rgba = [169, 179, 166, 255];
const COMMAND_ROWS: Rgba = [26, 51, 51, 255];
const COMMAND_LINES: Rgba = [55, 95, 77, 255];
const VICTORY_GROUND: Rgba = [24, 39, 35, 255];
const DAMAGE_SMOKE: Rgba = [91, 79, 65, 255];
const RED_ALERT_OUTLINE: Rgba = [229, 90, 51, 150];
const WIPE_FILL: Rgba = [5, 8, 10, 230];
const PORTRAIT_BG: Rgba = [18, 26, 29, 255];
const OBJECTIVE_BG: Rgba = [13, 20, 25, 235];
const SHEET_BG: Rgba = [10, 14, 18, 255];

/// `(sequence id, mood, motif, captions)`, verbatim from the Python script.
const SEQUENCES: [(&str, &str, &str, [&str; 6]); 6] = [
    (
        "01_first_contact",
        "dawn",
        "armor",
        [
            "Scouts report movement beyond the ridge",
            "Enemy armor crosses the valley",
            "Hold position until the signal",
            "Contact confirmed",
            "The front line wakes",
            "War has begun",
        ],
    ),
    (
        "02_coastal_evacuation",
        "coast",
        "coast",
        [
            "The harbor is under fire",
            "Civilians move through the smoke",
            "Cover the retreating column",
            "The sea lane is still open",
            "Last transport approaching",
            "Get everyone aboard",
        ],
    ),
    (
        "03_counterattack",
        "night",
        "counterattack",
        [
            "Command authorizes the counterattack",
            "Infantry advances under cover",
            "Armor breaches the perimeter",
            "Enemy fire intensifies",
            "The line begins to break",
            "Push through before dawn",
        ],
    ),
    (
        "04_after_the_battle",
        "dawn",
        "ambush",
        [
            "Smoke settles over the battlefield",
            "Damaged armor is abandoned",
            "Medics search the ruins",
            "The cost becomes clear",
            "Survivors regroup",
            "The campaign continues",
        ],
    ),
    (
        "05_command_briefing",
        "night",
        "command",
        [
            "Incoming transmission",
            "Recon identifies a weak flank",
            "Supply routes are threatened",
            "Choose the next objective",
            "Orders are distributed",
            "Operation begins at first light",
        ],
    ),
    (
        "06_campaign_end",
        "dawn",
        "victory",
        [
            "Final resistance is silenced",
            "The battlefield grows quiet",
            "The flag rises over the outpost",
            "A costly victory",
            "The map opens toward the frontier",
            "End of chapter",
        ],
    ),
];

/// Portrait card labels, in the script's order.
const PORTRAIT_LABELS: [&str; 4] = ["commander", "scout", "medic", "engineer"];

/// Objective card labels and their accent colors, in the script's order.
const OBJECTIVE_LABELS: [&str; 4] = [
    "primary_objective",
    "bonus_objective",
    "mission_failed",
    "mission_complete",
];

/// `_scene_base(seed, mood)`: sky gradient, ridgeline, smoke silhouettes and
/// the mood-specific pass; its RNG stream is private to this call (the
/// Python script seeds its own `Random` here and never shares it).
fn scene_base(seed: u64, mood: &str) -> Canvas {
    let mut rng = Rng::new(Seed::new(seed));
    let mut im = Canvas::new_filled(W as u32, H as u32, rgb(SKY0));
    for y in 0..H {
        let col: Rgba = if y < 57 {
            let t = y as f64 / 57.0;
            let mut c: Rgba = [0, 0, 0, 255];
            for i in 0..3 {
                c[i] = (SKY0[i] as f64 * (1.0 - t) + SKY1[i] as f64 * t) as u8;
            }
            c
        } else {
            GROUND
        };
        im.rect(0, y, W, y, Some(col), None, 0);
    }
    // distant ridgeline / smoke silhouettes
    let mut ridge: Vec<(i32, i32)> = Vec::new();
    let mut x = 0;
    while x < W + 4 {
        ridge.push((x, 51 + rng.rand_i32(-5, 6)));
        x += 4;
    }
    let mut points = vec![(0, 61)];
    points.extend_from_slice(&ridge);
    points.extend_from_slice(&[(W, 65), (W, H), (0, H)]);
    im.polygon(&points, Some(RIDGE), None);
    for _ in 0..5 {
        let x = rng.randrange2(0, W);
        let y = rng.randrange2(29, 50);
        let r = rng.randrange2(3, 9);
        im.ellipse(x - r, y - r, x + r, y + r, Some(SMOKE), None, 0);
    }
    match mood {
        "dawn" => {
            im.ellipse(116, 14, 138, 36, Some(ORANGE), None, 0);
            im.rect(0, 46, W, 55, Some(DAWN_BAND), None, 0);
        }
        "night" => {
            im.ellipse(121, 10, 135, 24, Some(MOON), None, 0);
            for _ in 0..36 {
                let x = rng.randrange2(0, W);
                let y = rng.randrange2(0, 43);
                im.set_pixel(x, y, STARS);
            }
        }
        "coast" => {
            im.rect(0, 52, W, 73, Some(WATER), None, 0);
            for _ in 0..17 {
                let x = rng.randrange2(0, W);
                let y = rng.randrange2(54, 73);
                let x2 = x + rng.randrange2(3, 13);
                im.line(x, y, x2, y, FOAM, 1);
            }
            im.polygon(
                &[(0, 68), (42, 62), (73, 72), (W, 65), (W, H), (0, H)],
                Some(GROUND),
                None,
            );
        }
        _ => {}
    }
    im
}

/// `_tank(d, x, y, scale=1, heading=1, damaged=False)`: side-on tracked
/// vehicle; independently authored turret and barrel.
fn tank(d: &mut Canvas, x: i32, y: i32, scale: i32, heading: i32, damaged: bool) {
    let w = 25 * scale;
    let h = 9 * scale;
    d.rounded_rect(x, y, x + w, y + h, 2, DARK);
    d.rounded_rect_outline(x, y, x + w, y + h, 2, STEEL_HI, 1);
    for i in 0..5 {
        let xx = x + 3 * scale + i * 4 * scale;
        d.ellipse(
            xx,
            y + 2 * scale,
            xx + 3 * scale,
            y + 7 * scale,
            Some(STEEL),
            None,
            0,
        );
    }
    d.polygon(
        &[
            (x + 4 * scale, y),
            (x + 8 * scale, y - 5 * scale),
            (x + 19 * scale, y - 5 * scale),
            (x + 23 * scale, y),
        ],
        Some(OLIVE),
        Some(OLIVE_HI),
    );
    d.ellipse(
        x + 10 * scale,
        y - 9 * scale,
        x + 19 * scale,
        y - 3 * scale,
        Some(OLIVE_HI),
        None,
        0,
    );
    if heading >= 0 {
        d.line(
            x + 17 * scale,
            y - 7 * scale,
            x + 31 * scale,
            y - 7 * scale,
            STEEL_HI,
            scale.max(1) as u32,
        );
    } else {
        d.line(
            x + 12 * scale,
            y - 7 * scale,
            x - 2 * scale,
            y - 7 * scale,
            STEEL_HI,
            scale.max(1) as u32,
        );
    }
    if damaged {
        d.line(
            x + 5 * scale,
            y - 3 * scale,
            x + 10 * scale,
            y + 3 * scale,
            ORANGE,
            scale.max(1) as u32,
        );
        d.ellipse(
            x + 20 * scale,
            y - 13 * scale,
            x + 27 * scale,
            y - 7 * scale,
            Some(DAMAGE_SMOKE),
            None,
            0,
        );
    }
}

/// `_soldier(d, x, y, phase=0, scale=1)`: walking trooper with two-frame legs.
fn soldier(d: &mut Canvas, x: i32, y: i32, phase: i32, scale: i32) {
    let leg = phase % 2;
    let width = scale.max(1) as u32;
    d.ellipse(
        x + 2 * scale,
        y - 12 * scale,
        x + 7 * scale,
        y - 7 * scale,
        Some(SKIN),
        None,
        0,
    );
    d.rect(
        x + scale,
        y - 7 * scale,
        x + 8 * scale,
        y + scale,
        Some(OLIVE),
        None,
        0,
    );
    let leg1 = if leg != 0 { 1 } else { 5 };
    let leg2 = if leg != 0 { 9 } else { 6 };
    d.line(
        x + 3 * scale,
        y,
        x + leg1 * scale,
        y + 6 * scale,
        DARK,
        width,
    );
    d.line(
        x + 6 * scale,
        y,
        x + leg2 * scale,
        y + 6 * scale,
        DARK,
        width,
    );
    d.line(
        x + 6 * scale,
        y - 5 * scale,
        x + 12 * scale,
        y - 7 * scale,
        STEEL_HI,
        width,
    );
}

/// `_draw_motif(im, motif, frame, rng)`: motif actors, then 28 film-grain
/// specks whose RNG is the frame-seeded stream from `generate`.
fn draw_motif(im: &mut Canvas, motif: &str, frame: i32, rng: &mut Rng) {
    if matches!(motif, "armor" | "ambush" | "counterattack" | "evacuation") {
        for i in 0..3 {
            tank(
                im,
                12 + i * 43 + (frame % 3 - 1) * 2,
                68 - (i % 2) * 4,
                1,
                1,
                motif == "ambush" && i == 1,
            );
        }
    }
    if matches!(motif, "infantry" | "evacuation" | "ambush") {
        for i in 0..5 {
            soldier(
                im,
                35 + i * 18 + (frame % 2),
                73 + (i % 2) * 2,
                frame + i,
                1,
            );
        }
    }
    if matches!(motif, "ambush" | "counterattack") {
        for &(x, y) in &[(57, 48), (103, 42), (130, 56)] {
            let f = frame % 2;
            im.ellipse(
                x - 3 - f,
                y - 3 - f,
                x + 3 + f,
                y + 3 + f,
                Some(ORANGE),
                None,
                0,
            );
            im.ellipse(x - 1, y - 1, x + 1, y + 1, Some(YELLOW), None, 0);
        }
    }
    if motif == "coast" {
        im.polygon(
            &[
                (0, 76),
                (28, 70),
                (59, 77),
                (89, 69),
                (W, 74),
                (W, H),
                (0, H),
            ],
            Some(GROUND),
            None,
        );
        im.rect(0, 52, W, 70, Some(WATER), None, 0);
        let mut x = (frame * 5) % 17;
        while x < W {
            im.line(x, 56, x + 8, 56, FOAM, 1);
            x += 17;
        }
    }
    if motif == "command" {
        im.rect(8, 27, 72, 64, Some(PANEL), Some(STEEL), 1);
        im.rect(13, 32, 67, 47, Some(COMMAND_ROWS), Some(GREEN), 1);
        for i in 0..4 {
            im.line(16, 35 + i * 3, 62, 35 + i * 3, COMMAND_LINES, 1);
        }
        im.ellipse(100, 34, 117, 51, Some(SKIN), None, 0);
        im.rect(98, 50, 120, 72, Some(OLIVE), None, 0);
    }
    if motif == "victory" {
        im.polygon(
            &[
                (80, 22),
                (88, 37),
                (104, 38),
                (92, 48),
                (96, 64),
                (80, 54),
                (64, 64),
                (68, 48),
                (56, 38),
                (72, 37),
            ],
            Some(YELLOW),
            None,
        );
        im.rect(0, 72, W, H, Some(VICTORY_GROUND), None, 0);
    }
    // film grain specks are deterministic and sparse
    for _ in 0..28 {
        let x = rng.randrange2(0, W);
        let y = rng.randrange2(0, H);
        let c = im.pixel(x, y).unwrap_or([0, 0, 0, 255]);
        if rng.chance(0.15) {
            im.set_pixel(
                x,
                y,
                [
                    c[0].saturating_add(9),
                    c[1].saturating_add(9),
                    c[2].saturating_add(9),
                    c[3],
                ],
            );
        }
    }
}

/// `_letterbox(im, frame, total, subtitle)`: letterbox bars, markers and
/// font-safe subtitle/progress blocks.
fn letterbox(im: &mut Canvas, frame: i32, total: i32, subtitle: &str) {
    im.rect(0, 0, W, 8, Some(BLACK), None, 0);
    im.rect(0, H - 13, W, H, Some(BLACK), None, 0);
    im.rect(5, 3, 28, 5, Some(RED), None, 0);
    im.rect(W - 29, 3, W - 5, 5, Some(STEEL), None, 0);
    // subtitle marker blocks are language-agnostic and font-safe
    let n = (subtitle.len() * 2).clamp(10, 86) as i32;
    im.rect(8, H - 9, 8 + n, H - 7, Some(WHITE), None, 0);
    im.rect(W - 28, H - 9, W - 8, H - 7, None, Some(ORANGE), 1);
    let progress = (16.0 * (f64::from(frame) + 1.0) / f64::from(total)) as i32;
    im.rect(
        W - 26,
        H - 8,
        W - 26 + progress,
        H - 8,
        Some(ORANGE),
        None,
        0,
    );
}

/// Common manifest sprite spec for every file this layer registers.
fn cinematic_spec(seed: u64) -> SpriteSpec<'static> {
    SpriteSpec {
        category: "cinematic",
        generated_by: Some(GENERATED_BY),
        seed: Some(seed),
        ..SpriteSpec::default()
    }
}

/// Save a generated canvas and append its manifest record unless the id is
/// already registered in the existing `manifest.json` (the Python manifest
/// loop skips duplicates without skipping the file).
fn register(
    out: &PackWriter,
    rel: &str,
    canvas: &Canvas,
    seed: u64,
    existing: &HashSet<String>,
    output: &mut LayerOutput,
) -> Result<(), PackError> {
    let record = out.write_sprite(rel, canvas, cinematic_spec(seed))?;
    let id = record["id"].as_str().unwrap_or_default().to_string();
    if !existing.contains(&id) {
        output.sprites.push(record);
    }
    Ok(())
}

/// Python `str.title()` on a snake/word string: capitalize after every
/// non-letter boundary, which for these ids means per-`_`-word initials.
fn title_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// One sequence dict for the timeline, verbatim fields.
///
/// Captions only ride inside `frames` records, so no separate copy here.
fn sequence_record(name: &str, frames: Vec<Value>) -> Value {
    json!({
        "id": name,
        "title": title_case(name),
        "fps": 10,
        "loop": false,
        "canvas": [OUT_W, OUT_H],
        "frames": frames,
        "transitions": {
            "in": "fade_from_black",
            "out": "fade_to_black",
            "camera": "slow_pan_zoom",
        },
        "audio_cues": ["ambience", "combat_or_radio", "stinger"],
        "captions_are_metadata_only": true,
    })
}

/// Generate this layer into `out` and return its contributions.
pub fn generate(out: &PackWriter, seed: u64) -> Result<LayerOutput, PackError> {
    let mut output = LayerOutput::default();

    // The Python script reads the existing manifest and only appends ids it
    // has not registered yet; the builder merges records by id, but the skip
    // semantics are kept for packs built incrementally.
    let existing: HashSet<String> = match out.read_json("manifest.json")? {
        Some(manifest) => manifest
            .get("sprites")
            .and_then(Value::as_array)
            .map(|sprites| {
                sprites
                    .iter()
                    .filter_map(|sprite| sprite.get("id").and_then(Value::as_str))
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default(),
        None => HashSet::new(),
    };

    let mut sequences_out: Vec<Value> = Vec::new();
    // Storyboard sources: the Python contact sheets re-open the saved frame
    // PNGs, but PNG encode/decode is lossless, so the staged canvases are
    // byte-equivalent sources.
    let mut staged_frames: Vec<Vec<Canvas>> = Vec::new();
    // `generated`: every PNG path created, in Python append order.
    let mut generated: Vec<String> = Vec::new();

    for (si, (name, mood, motif, captions)) in SEQUENCES.iter().enumerate() {
        let mut frame_records = Vec::new();
        let mut frames = Vec::new();
        for (fi, caption) in captions.iter().enumerate() {
            let s = seed + si as u64 * 1009 + fi as u64 * 97;
            let mut rng = Rng::new(Seed::new(s));
            let mut im = scene_base(s, mood);
            draw_motif(&mut im, motif, fi as i32, &mut rng);
            // gradual camera pan/zoom effect via nearest-neighbour crop
            let zoom = 1.0 + 0.012 * fi as f64;
            let cw = (W as f64 / zoom) as i32;
            let ch = (H as f64 / zoom) as i32;
            let left = ((W - cw) as f64 * (fi as f64 / (captions.len() - 1) as f64)) as i32;
            let top = ((H - ch) as f64 * 0.45) as i32;
            im = im
                .crop(left, top, cw as u32, ch as u32)
                .resize_nearest(W as u32, H as u32);
            letterbox(&mut im, fi as i32, captions.len() as i32, caption);
            let final_frame = im.resize_nearest(OUT_W, OUT_H);
            let rel = format!("sprites/cinematics/frames/{name}/frame_{fi:03}.png");
            register(out, &rel, &final_frame, seed, &existing, &mut output)?;
            let sha = out.sha256_file(&rel)?;
            let duration_ms = if fi + 1 < captions.len() { 900 } else { 1400 };
            frame_records.push(json!({
                "index": fi,
                "file": rel,
                "duration_ms": duration_ms,
                "caption": caption,
                "seed": s,
                "sha256": sha,
            }));
            frames.push(final_frame);
            generated.push(rel);
        }
        sequences_out.push(sequence_record(name, frame_records));
        staged_frames.push(frames);
    }

    // reusable visual overlays
    let mut supporting_assets: Vec<String> = Vec::new();
    let overlays: [(&str, Rgba); 4] = [
        ("fade_black", [0, 0, 0, 255]),
        ("red_alert", [112, 20, 12, 86]),
        ("radio_scan", [32, 170, 140, 24]),
        ("vignette", [0, 0, 0, 100]),
    ];
    for &(name, rgba) in &overlays {
        let mut im = Canvas::transparent(OUT_W, OUT_H);
        match name {
            "fade_black" => im.rect(0, 0, OUT_W as i32, OUT_H as i32, Some(rgba), None, 0),
            "red_alert" => {
                im.rect(0, 0, OUT_W as i32, OUT_H as i32, Some(rgba), None, 0);
                im.rect(
                    10,
                    10,
                    OUT_W as i32 - 10,
                    OUT_H as i32 - 10,
                    None,
                    Some(RED_ALERT_OUTLINE),
                    3,
                );
            }
            "radio_scan" => {
                let mut y = 0;
                while y < OUT_H as i32 {
                    im.line(0, y, OUT_W as i32, y, rgba, 1);
                    y += 6;
                }
            }
            _ => {
                // simple edge vignette bands
                for i in 0..18 {
                    let alpha = (5.0 + f64::from(i) * 2.2) as i32 as u8;
                    im.rect(
                        i * 3,
                        i * 2,
                        OUT_W as i32 - i * 3,
                        OUT_H as i32 - i * 2,
                        None,
                        Some([0, 0, 0, alpha]),
                        3,
                    );
                }
            }
        }
        let rel = format!("sprites/cinematic_ui/{name}.png");
        register(out, &rel, &im, seed, &existing, &mut output)?;
        generated.push(rel.clone());
        supporting_assets.push(rel);
    }

    // transition wipes
    for &(name, kind) in [
        ("wipe_horizontal", "horizontal"),
        ("wipe_vertical", "vertical"),
        ("wipe_diagonal", "diagonal"),
    ]
    .iter()
    {
        let mut im = Canvas::transparent(OUT_W, OUT_H);
        match kind {
            "horizontal" => {
                im.rect(
                    0,
                    0,
                    OUT_W as i32 / 2,
                    OUT_H as i32,
                    Some(WIPE_FILL),
                    None,
                    0,
                );
            }
            "vertical" => {
                im.rect(
                    0,
                    0,
                    OUT_W as i32,
                    OUT_H as i32 / 2,
                    Some(WIPE_FILL),
                    None,
                    0,
                );
            }
            _ => {
                im.polygon(
                    &[(0, 0), (OUT_W as i32 / 2, 0), (0, OUT_H as i32 / 2)],
                    Some(WIPE_FILL),
                    None,
                );
            }
        }
        let rel = format!("sprites/cinematic_ui/{name}.png");
        register(out, &rel, &im, seed, &existing, &mut output)?;
        generated.push(rel.clone());
        supporting_assets.push(rel);
    }

    // portrait cards
    for (i, label) in PORTRAIT_LABELS.iter().enumerate() {
        let mut im = Canvas::new_filled(96, 96, PORTRAIT_BG);
        im.rect(8, 8, 87, 87, None, Some(STEEL), 2);
        im.ellipse(31, 19, 64, 52, Some(SKIN), Some(DARK), 1);
        im.polygon(
            &[(26, 23), (31, 12), (63, 12), (69, 27), (61, 22), (34, 22)],
            Some(if i % 2 == 0 { OLIVE_HI } else { STEEL }),
            None,
        );
        let uniform = [OLIVE, STEEL, WHITE, ORANGE][i];
        im.polygon(
            &[(20, 86), (25, 58), (39, 50), (57, 50), (72, 58), (78, 86)],
            Some(uniform),
            None,
        );
        im.rect(13, 72, 83, 83, Some(PANEL), None, 0);
        let bar_end = 18 + (52.min(label.len() * 7)) as i32;
        im.rect(18, 77, bar_end, 79, Some(WHITE), None, 0);
        let rel = format!("sprites/cinematic_ui/portrait_{label}.png");
        register(out, &rel, &im, seed, &existing, &mut output)?;
        generated.push(rel.clone());
        supporting_assets.push(rel);
    }

    // objective cards
    for (i, label) in OBJECTIVE_LABELS.iter().enumerate() {
        let mut im = Canvas::new_filled(OUT_W, 64, OBJECTIVE_BG);
        let accent = [ORANGE, GREEN, RED, YELLOW][i];
        im.rect(2, 2, 317, 61, None, Some(accent), 2);
        im.rect(12, 12, 20, 52, Some(accent), None, 0);
        im.rect(32, 18, 160 + i as i32 * 13, 22, Some(WHITE), None, 0);
        im.rect(32, 30, 230, 33, Some(STEEL), None, 0);
        let rel = format!("sprites/cinematic_ui/{label}.png");
        register(out, &rel, &im, seed, &existing, &mut output)?;
        generated.push(rel.clone());
        supporting_assets.push(rel);
    }

    // contact sheets for quick human review
    for (seq, frames) in sequences_out.iter().zip(&staged_frames) {
        let name = seq["id"].as_str().expect("sequence id");
        let mut sheet = Canvas::new_filled(640, 540, SHEET_BG);
        for (i, frame) in frames.iter().enumerate() {
            let cell = frame.resize_nearest(320, 180);
            sheet.alpha_composite(&cell, (((i % 2) * 320) as i32, ((i / 2) * 180) as i32));
        }
        let rel = format!("sprites/cinematics/previews/{name}_storyboard.png");
        register(out, &rel, &sheet, seed, &existing, &mut output)?;
        generated.push(rel);
    }

    // timeline contribution: the builder writes sprites/cinematics/timeline.json
    output.timeline = Some(TimelineContribution {
        sequences: sequences_out.clone(),
        campaigns: Vec::new(),
        notes: TIMELINE_NOTES
            .iter()
            .map(|note| (*note).to_string())
            .collect(),
        schema_version: "1.0.0",
        generator_version: GENERATOR_VERSION,
        seed: Some(seed),
        extra: Map::new(),
    });
    // The Python script's `supporting_assets` timeline key lists exactly the
    // generated `cinematic_ui` files (not previews or frames).
    output.supporting_assets = supporting_assets.clone();

    // `cinematics` manifest summary and this layer's generator version stamp
    // (later layers stamp their own version on top, in layer order).
    let frame_count: usize = sequences_out
        .iter()
        .map(|seq| seq["frames"].as_array().map_or(0, Vec::len))
        .sum();
    output.manifest_extra.insert(
        "generator_version".to_string(),
        Value::String(GENERATOR_VERSION.to_string()),
    );
    output.manifest_extra.insert(
        "cinematics".to_string(),
        json!({
            "timeline": "sprites/cinematics/timeline.json",
            "sequence_count": sequences_out.len(),
            "frame_count": frame_count,
            "supporting_asset_count": supporting_assets.len(),
        }),
    );

    // standalone build report at the pack root
    let unique_frame_hashes: HashSet<&str> = sequences_out
        .iter()
        .flat_map(|seq| seq["frames"].as_array().expect("frames"))
        .filter_map(|frame| frame["sha256"].as_str())
        .collect();
    let all_files_exist = generated.iter().all(|rel| out.path(rel).is_file());
    let report = json!({
        "generator_version": GENERATOR_VERSION,
        "seed": seed,
        "sequences": sequences_out.len(),
        "frames": frame_count,
        "supporting_assets": supporting_assets.len(),
        "generated_files": generated.len() + 2,
        "unique_frame_hashes": unique_frame_hashes.len(),
        "all_files_exist": all_files_exist,
        "notes": REPORT_NOTES.iter().map(|note| Value::from(*note)).collect::<Vec<Value>>(),
    });
    output
        .reports
        .push(("cutscene_build_report_v5_8.json".to_string(), report));

    Ok(output)
}

/// `timeline["notes"]`, verbatim from the Python script.
const TIMELINE_NOTES: [&str; 4] = [
    "Procedural pixel-art scenes generated from deterministic primitives.",
    "Captions are metadata; localize with engine text rendering.",
    "Audio cues are event labels, not included audio files.",
    "Storyboard previews are for visual review, not runtime atlases.",
];

/// `report["notes"]`, verbatim from the Python script.
const REPORT_NOTES: [&str; 2] = [
    "Image-level deterministic generation verified.",
    "Godot playback and audio synchronization require engine-side verification.",
];

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::expansion::sha256_hex;

    use super::*;

    static TMP_SEQ: AtomicU32 = AtomicU32::new(0);

    /// Fresh staging root per test, mirroring the Python tests' temp dir.
    fn temp_root(tag: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "cnc-procgen-cutscene_v58-{}-{}-{}",
            tag,
            std::process::id(),
            TMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&path).expect("create temp pack root");
        path
    }

    /// Cleanup helper: remove the staged pack tree.
    fn cleanup(root: &PathBuf) {
        std::fs::remove_dir_all(root).ok();
    }

    /// Determinism oracle: regenerate the layer and compare the first frame's
    /// file bytes (Python test 3 compares the same path's bytes).
    #[test]
    fn seed_is_deterministic() {
        let first_root = temp_root("determinism-a");
        let second_root = temp_root("determinism-b");
        let rel = "sprites/cinematics/frames/01_first_contact/frame_000.png";
        let first = {
            let out = PackWriter::new(first_root.clone());
            generate(&out, 123).expect("generate");
            std::fs::read(out.path(rel)).expect("frame bytes")
        };
        let second = {
            let out = PackWriter::new(second_root.clone());
            generate(&out, 123).expect("generate");
            std::fs::read(out.path(rel)).expect("frame bytes")
        };
        assert_eq!(first, second);
        cleanup(&first_root);
        cleanup(&second_root);
    }

    /// Python test 1: 6 sequences × 6 unique frames, files exist, hashes
    /// recorded in the timeline match the written PNGs, frames are 320×180.
    #[test]
    fn generates_all_sequences_and_unique_frames() {
        let root = temp_root("all-frames");
        let out = PackWriter::new(root.clone());
        let result = generate(&out, 44).expect("generate");

        // report arithmetic, as asserted by the Python test file
        let report = &result.reports[0].1;
        assert_eq!(report["generator_version"], "5.8.0");
        assert_eq!(report["sequences"], 6);
        assert_eq!(report["frames"], 36);
        assert_eq!(report["supporting_assets"], 15);
        assert_eq!(report["generated_files"], 59);
        assert_eq!(report["unique_frame_hashes"], 36);
        assert_eq!(report["all_files_exist"], true);

        // timeline contribution: schema, sequence and frame counts
        let timeline = result.timeline.as_ref().expect("timeline contribution");
        assert_eq!(timeline.schema_version, "1.0.0");
        assert_eq!(timeline.sequences.len(), 6);
        let mut frame_hashes = HashSet::new();
        for seq in &timeline.sequences {
            assert_eq!(seq["id"].as_str().map_or(0, caption_shape), 1);
            let frames = seq["frames"].as_array().expect("frame records");
            assert_eq!(frames.len(), 6);
            for frame in frames {
                let rel = frame["file"].as_str().expect("frame file");
                let path = out.path(rel);
                assert!(path.is_file(), "{rel} must exist");
                let bytes = std::fs::read(&path).expect("read frame");
                assert_eq!(
                    sha256_hex(&bytes),
                    frame["sha256"].as_str().expect("frame sha256")
                );
                frame_hashes.insert(frame["sha256"].as_str().expect("hash").to_string());
                // sprite record for the frame must record the 320×180 size
                let sprite = result
                    .sprites
                    .iter()
                    .find(|sprite| sprite["file"].as_str() == Some(rel))
                    .expect("frame sprite record");
                assert_eq!(sprite["size"], json!([320, 180]));
            }
        }
        assert_eq!(frame_hashes.len(), 36);
        cleanup(&root);
    }

    /// Trivial marker so the per-sequence id assertion stays explicit.
    fn caption_shape(id: &str) -> usize {
        if id.is_empty() {
            0
        } else {
            1
        }
    }

    /// Python test 2: manifest-side artifacts — sprite records exist with
    /// matching integrity hashes, category/generated_by stamps, and the
    /// `cinematics` summary block.
    #[test]
    fn manifest_contains_supporting_assets_and_integrity_hashes() {
        let root = temp_root("manifest");
        let out = PackWriter::new(root.clone());
        let result = generate(&out, 44).expect("generate");

        assert_eq!(result.manifest_extra["generator_version"], "5.8.0");
        assert_eq!(result.manifest_extra["cinematics"]["sequence_count"], 6);
        assert_eq!(result.manifest_extra["cinematics"]["frame_count"], 36);
        assert!(
            result.manifest_extra["cinematics"]["supporting_asset_count"]
                .as_u64()
                .expect("supporting_asset_count")
                >= 15
        );

        // 33 frames+extras arithmetic: 36 frames + 15 cinematic_ui + 6 previews
        assert_eq!(result.sprites.len(), 57);
        for sprite in &result.sprites {
            let rel = sprite["file"].as_str().expect("sprite file");
            assert_eq!(sprite["category"], "cinematic");
            assert_eq!(sprite["generated_by"], GENERATED_BY);
            let path = out.path(rel);
            assert!(path.is_file(), "{rel} must exist");
            let bytes = std::fs::read(&path).expect("read sprite");
            assert_eq!(
                sha256_hex(&bytes),
                sprite["sha256_file"].as_str().expect("sha256_file")
            );
        }

        // timeline.supporting_assets carrying list: exactly the cinematic_ui paths
        let expected_ui = [
            "sprites/cinematic_ui/fade_black.png",
            "sprites/cinematic_ui/red_alert.png",
            "sprites/cinematic_ui/radio_scan.png",
            "sprites/cinematic_ui/vignette.png",
            "sprites/cinematic_ui/wipe_horizontal.png",
            "sprites/cinematic_ui/wipe_vertical.png",
            "sprites/cinematic_ui/wipe_diagonal.png",
            "sprites/cinematic_ui/portrait_commander.png",
            "sprites/cinematic_ui/portrait_scout.png",
            "sprites/cinematic_ui/portrait_medic.png",
            "sprites/cinematic_ui/portrait_engineer.png",
            "sprites/cinematic_ui/primary_objective.png",
            "sprites/cinematic_ui/bonus_objective.png",
            "sprites/cinematic_ui/mission_failed.png",
            "sprites/cinematic_ui/mission_complete.png",
        ];
        assert_eq!(result.supporting_assets, expected_ui.to_vec());
        cleanup(&root);
    }
}
