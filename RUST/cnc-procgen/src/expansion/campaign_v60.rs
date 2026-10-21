//! Port of `campaign_expansion_v6_0.py` (v6.2.1 asset pack, v6.0.0) — ten
//! additional campaign scenes in Acts IV and V, storyboards, 48 reusable
//! prop sprites, eight looping effect animations, portraits, act key-art
//! cards and the two v6 atlases.
//!
//! The Python file is the contract: this port keeps its structure,
//! decomposition, ids, file paths, seed derivation, palette, draw order and
//! draw counts line-for-line. It chains onto the complete v5.9 layer
//! exactly like the Python script does: scene backgrounds come from
//! [`campaign_v59::render_scene`] and portraits from
//! [`campaign_v59::portrait`] (with the Python `i+8` index shift).
//!
//! Documented deviations (accepted, not "fixed"):
//! - SplitMix64 [`Rng`] instead of Python MT19937: identical draw counts,
//!   order and ranges, but different values, so pixels differ from the
//!   Python pack while remaining deterministic in Rust.
//! - [`Canvas`] rasterizer instead of Pillow: wide lines use a square
//!   stamp, polylines are consecutive two-point segments, and fractional
//!   coordinates are truncated toward zero exactly like Pillow's C-level
//!   `int()` cast.
//! - Python `round()` maps to `round_ties_even` (Pillow's banker rounding);
//!   `int()` maps to `trunc`.
//! - [`draw_text`] (built-in 5×7 bitmap font, unsupported glyphs such as
//!   the em dash render blank and still advance) stands in for Pillow's
//!   default bitmap font on the storyboard labels, act-card titles and the
//!   overview labels.
//! - Storyboard and overview thumbnails are composited from the in-memory
//!   canvases instead of re-opening the saved PNGs (same source pixels).
//! - `Image.paste` for the opaque thumbnails is realized as
//!   [`Canvas::alpha_composite`], which is pixel-equivalent for opaque
//!   sources.
//! - serde_json key order (alphabetical) replaces Python's insertion order,
//!   so JSON artifacts are byte-different but semantically identical.
//! - Python raises `FileNotFoundError` when `manifest.json` or the shared
//!   cinematics timeline is missing; this port surfaces the same condition
//!   as a `PackError::Io(NotFound)`.

use std::collections::HashSet;

use serde_json::{json, Map, Value};

use super::{
    compose_atlas, sha256_hex, sprite_record, LayerOutput, PackWriter, SpriteSpec,
    TimelineContribution,
};
use crate::expansion::campaign_v59;
use crate::prng::{Rng, Seed};
use crate::tileset::canvas::Canvas;
use crate::tileset::font::draw_text;
use crate::tileset::pack::PackError;
use crate::tileset::{Rgba, TileRngExt};

/// Python `generate(..., seed=60_017)` default seed (independent of the
/// pack's `--seed`, exactly as the script was run to build the real pack).
pub const DEFAULT_SEED: u64 = 60_017;

/// Python `VERSION` (`report["asset_pack_version"]` / `generator_version`).
pub const VERSION: &str = "6.0.0";

/// Python `generated_by` stamp on every registered sprite record.
const GENERATED_BY: &str = "campaign_expansion_v6_0.py";

// Python `W, H = 320, 180`; `TILE = 32`.
const W: i32 = 320;
const H: i32 = 180;
const TILE: u32 = 32;

/// Python `SCENES`: `(id, title, act, environment, motif, captions)`.
#[allow(clippy::type_complexity)] // shape mirrors the Python tuple rows
const SCENES: [(&str, &str, &str, &str, &str, [&str; 8]); 10] = [
    (
        "19_burned_rail",
        "The Burned Road",
        "Act IV — Counterpunch",
        "forest",
        "railway",
        [
            "The eastern rail line is cut",
            "Scouts find a wrecked supply train",
            "Repair crews move under cover",
            "An enemy patrol reaches the crossing",
            "Armor blocks the road beyond the tracks",
            "The rail crane lifts the shattered span",
            "The supply line is running again",
            "The counterpunch can begin",
        ],
    ),
    (
        "20_floodplain_crossing",
        "Floodplain Crossing",
        "Act IV — Counterpunch",
        "river",
        "floodplain",
        [
            "The old ford has vanished under floodwater",
            "Engineers mark a safe crossing",
            "Pontoon sections arrive at first light",
            "The river current tears at the cables",
            "The lead carrier tests the span",
            "A rescue boat clears debris from the bank",
            "The last vehicle reaches dry ground",
            "The floodplain route is secured",
        ],
    ),
    (
        "21_red_dust_refinery",
        "Red Dust Refinery",
        "Act IV — Counterpunch",
        "desert",
        "refinery",
        [
            "A refinery controls the fuel corridor",
            "Scouts locate the storage tanks",
            "The perimeter searchlights come alive",
            "Smoke hides the western service road",
            "A tanker convoy enters the yard",
            "The pump station loses power",
            "The fuel pumps are secured intact",
            "The armored column can move again",
        ],
    ),
    (
        "22_northwatch_relay",
        "Northwatch Relay",
        "Act IV — Counterpunch",
        "snow",
        "northwatch",
        [
            "Northwatch has missed two scheduled calls",
            "The radar dish turns without a signal",
            "A patrol climbs through the whiteout",
            "A faint beacon appears beyond the ridge",
            "The relay hut is found abandoned",
            "Technicians restore the backup generator",
            "The radar paints the northern approach",
            "The warning network is back online",
        ],
    ),
    (
        "23_long_night_convoy",
        "The Long Night",
        "Act IV — Counterpunch",
        "night",
        "night_column",
        [
            "The convoy leaves after last light",
            "Headlamps are reduced to blackout slits",
            "A flare crosses the distant treeline",
            "The lead vehicle brakes hard",
            "Scouts mark an ambush on the road",
            "The column moves through a side cut",
            "The radio confirms the rendezvous",
            "The reinforcements arrive before dawn",
        ],
    ),
    (
        "24_skyshield",
        "Skyshield",
        "Act V — Final Offensive",
        "storm",
        "air_defense",
        [
            "The storm masks a hostile air approach",
            "Radar operators track a low silhouette",
            "The battery locks onto the target",
            "A missile climbs through the clouds",
            "Countermeasures scatter across the sky",
            "The aircraft turns away from the convoy",
            "The air-defense crew reports clear skies",
            "The ground offensive continues",
        ],
    ),
    (
        "25_foundry_zero",
        "Foundry Zero",
        "Act V — Final Offensive",
        "city",
        "foundry",
        [
            "The foundry is the enemy's last vehicle works",
            "Recon maps the furnace hall",
            "A damaged loader blocks the entrance",
            "Sparks shower from the assembly gantry",
            "Our engineers cut the control feed",
            "The production line winds down",
            "The foundry is secured with minimal damage",
            "The captured works will serve the rebuild",
        ],
    ),
    (
        "26_last_battery",
        "The Last Battery",
        "Act V — Final Offensive",
        "storm",
        "battery",
        [
            "One artillery battery guards the final pass",
            "Its range covers every approach",
            "Forward observers set a safe corridor",
            "A smoke screen rolls over the ridge",
            "The battery fires its final salvo",
            "Friendly armor crosses the dead ground",
            "The guns fall silent",
            "The pass is open to the main force",
        ],
    ),
    (
        "27_homecoming",
        "Homecoming",
        "Act V — Final Offensive",
        "dawn",
        "homecoming",
        [
            "The convoy returns to a battered town",
            "Medics set up a field station",
            "Residents clear the main street",
            "Engineers reconnect the power lines",
            "A mobile kitchen serves the first meal",
            "The school shelter opens its doors",
            "The town square fills with people",
            "The long campaign begins to heal",
        ],
    ),
    (
        "28_new_frontier",
        "A New Frontier",
        "Act V — Final Offensive",
        "dawn",
        "frontier",
        [
            "The last checkpoint is dismantled",
            "Survey crews mark the new road",
            "A relay beacon rises above the ridge",
            "The first construction vehicles arrive",
            "Power and water reach the outpost",
            "The watchtower signals the valley",
            "A new settlement appears on the horizon",
            "The next chapter belongs to its people",
        ],
    ),
];

/// The 48 reusable prop sprite names (Python `PROP_NAMES`).
const PROP_NAMES: [&str; 48] = [
    "oil_storage_tank",
    "pumpjack",
    "portable_radar",
    "radar_truck",
    "mobile_repair_unit",
    "rail_crane",
    "rail_crossing",
    "pontoon_bridge",
    "pontoon_segment",
    "ammunition_stack",
    "field_generator",
    "signal_lamp",
    "searchlight",
    "missile_launcher",
    "aa_gun",
    "damaged_helicopter",
    "barbed_wire_coil",
    "sandbag_command_post",
    "communications_tower",
    "relay_dish",
    "antenna_mast",
    "repair_bay",
    "reconstruction_house",
    "civilian_shelter",
    "water_tower",
    "power_substation",
    "refinery_pipework",
    "warehouse_door",
    "snowmobile",
    "tracked_hauler",
    "field_kitchen",
    "medical_tent",
    "evacuation_bus",
    "supply_sled",
    "flak_battery",
    "wrecked_carrier",
    "command_table",
    "road_sign",
    "supply_depot",
    "forward_observer",
    "radio_beacon",
    "fuel_barrels",
    "crane_hook",
    "bunker_console",
    "small_hangar",
    "rescue_boat",
    "radar_sweep",
    "portable_work_light",
];

/// The 8 new character portraits (Python `PORTRAITS`).
const PORTRAITS: [&str; 8] = [
    "captain_anya_v6",
    "engineer_rook_v6",
    "medic_sato_v6",
    "scout_arden_v6",
    "radar_operator_lee_v6",
    "quartermaster_vesper_v6",
    "pilot_marek_v6",
    "town_coordinator_nia_v6",
];

/// The 8 looping effect animations (Python `EFFECTS`).
const EFFECTS: [&str; 8] = [
    "muzzle_flash",
    "shell_burst",
    "dust_plume",
    "water_splash",
    "radar_pulse",
    "fire_burst",
    "smoke_plume",
    "electric_arc",
];

/// Act key-art cards: `(card id, title, base rgb, accent rgb)` — the Python
/// `ACT_CARD_SPECS` verbatim.
type ActCardSpec = (&'static str, &'static str, [u8; 3], [u8; 3]);
const ACT_CARD_SPECS: [ActCardSpec; 2] = [
    (
        "act_4_counterpunch",
        "Act IV — Counterpunch",
        [91, 103, 78],
        [220, 156, 78],
    ),
    (
        "act_5_final_offensive",
        "Act V — Final Offensive",
        [69, 83, 86],
        [209, 88, 63],
    ),
];

/// Free-form notes appended to the timeline (Python `added_notes`).
const ADDED_NOTES: [&str; 7] = [
    "Each added scene has 8 deterministic 320x180 frames, camera treatment metadata, caption metadata, and cue labels.",
    "Campaign props and portrait cards are standalone reusable PNG sprites; prop rects are described by campaign_art_atlas.json.",
    "v6.0 adds ten new 8-frame scenes across Acts IV and V while retaining all earlier sequences.",
    "v6.0 adds 48 reusable 64x64 prop sprites, eight six-frame 32x32 effects, eight character portraits, two act cards, and atlases.",
    "Camera paths, captions, transition durations, effects cues, and audio cues are metadata; audio files are not included.",
    "Automated validation verifies file existence, dimensions, hashes, unique scene IDs, atlas bounds, and animation frame variety.",
    "Actual Godot playback, subtitle layout, alpha compositing, audio timing, and game-engine atlas import remain manual integration checks.",
];

/// Python `int(f)` (truncation toward zero) on an `f64`.
fn py_int(v: f64) -> i32 {
    v.trunc() as i32
}

/// Opaque RGBA from an RGB triple.
fn rgb(r: u8, g: u8, b: u8) -> Rgba {
    [r, g, b, 255]
}

/// Clamp to the 0..=255 byte range (Python `max(0, min(255, v))`).
fn clamp_byte(v: impl Into<i64>) -> u8 {
    v.into().clamp(0, 255) as u8
}

// ---------------------------------------------------------------------------
// Pillow-flavored draw shorthands (one wrapper per PIL `ImageDraw` call shape)
// ---------------------------------------------------------------------------

/// `d.rectangle(bbox, fill=...)` (inclusive bounds).
#[allow(clippy::too_many_arguments)]
fn d_rect(d: &mut Canvas, x0: i32, y0: i32, x1: i32, y1: i32, fill: Rgba) {
    d.rect(x0, y0, x1, y1, Some(fill), None, 0);
}

/// `d.rectangle(bbox, fill=..., outline=...)` (outline width 1).
#[allow(clippy::too_many_arguments)]
fn d_rect_out(d: &mut Canvas, x0: i32, y0: i32, x1: i32, y1: i32, fill: Rgba, outline: Rgba) {
    d.rect(x0, y0, x1, y1, Some(fill), Some(outline), 1);
}

/// `d.rectangle(bbox, fill=..., outline=..., width=w)`.
#[allow(clippy::too_many_arguments)]
fn d_rect_out_w(
    d: &mut Canvas,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    fill: Rgba,
    outline: Rgba,
    width: u32,
) {
    d.rect(x0, y0, x1, y1, Some(fill), Some(outline), width);
}

/// `d.point((x, y), fill=...)`.
fn d_point(d: &mut Canvas, x: i32, y: i32, fill: Rgba) {
    d.set_pixel(x, y, fill);
}

/// `d.line((x0, y0, x1, y1), fill=...)` (width 1).
#[allow(clippy::too_many_arguments)]
fn d_line(d: &mut Canvas, x0: i32, y0: i32, x1: i32, y1: i32, fill: Rgba) {
    d.line(x0, y0, x1, y1, fill, 1);
}

/// `d.line(..., width=w)`.
#[allow(clippy::too_many_arguments)]
fn d_line_w(d: &mut Canvas, x0: i32, y0: i32, x1: i32, y1: i32, fill: Rgba, width: u32) {
    d.line(x0, y0, x1, y1, fill, width);
}

/// `d.line((pt, pt, ...), fill=..., width=w)` — polyline via consecutive
/// two-point segments.
fn d_polyline(d: &mut Canvas, pts: &[(i32, i32)], fill: Rgba, width: u32) {
    for pair in pts.windows(2) {
        d.line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, fill, width);
    }
}

/// `d.ellipse(bbox, fill=...)`.
#[allow(clippy::too_many_arguments)]
fn d_ellipse(d: &mut Canvas, x0: i32, y0: i32, x1: i32, y1: i32, fill: Rgba) {
    d.ellipse(x0, y0, x1, y1, Some(fill), None, 1);
}

/// `d.ellipse(bbox, fill=..., outline=..., width=w)`.
#[allow(clippy::too_many_arguments)]
fn d_ellipse_out_w(
    d: &mut Canvas,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    fill: Rgba,
    outline: Rgba,
    width: u32,
) {
    d.ellipse(x0, y0, x1, y1, Some(fill), Some(outline), width);
}

/// `d.ellipse(bbox, fill=..., outline=...)` (width 1).
#[allow(clippy::too_many_arguments)]
fn d_ellipse_out(d: &mut Canvas, x0: i32, y0: i32, x1: i32, y1: i32, fill: Rgba, outline: Rgba) {
    d.ellipse(x0, y0, x1, y1, Some(fill), Some(outline), 1);
}

/// `d.polygon(pts, fill=...)`.
fn d_polygon(d: &mut Canvas, pts: &[(i32, i32)], fill: Rgba) {
    d.polygon(pts, Some(fill), None);
}

/// `d.polygon(pts, fill=..., outline=...)`.
fn d_polygon_out(d: &mut Canvas, pts: &[(i32, i32)], fill: Rgba, outline: Rgba) {
    d.polygon(pts, Some(fill), Some(outline));
}

/// `d.rounded_rectangle(bbox, radius=r, fill=..., outline=...)` (outline
/// width 1): PIL draws the fill before the outline.
#[allow(clippy::too_many_arguments)]
fn d_rounded_rect_out(
    d: &mut Canvas,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    fill: Rgba,
    outline: Rgba,
) {
    d.rounded_rect(x0, y0, x1, y1, radius, fill);
    d.rounded_rect_outline(x0, y0, x1, y1, radius, outline, 1);
}

/// `d.rounded_rectangle(bbox, radius=r, fill=..., outline=..., width=w)`.
#[allow(clippy::too_many_arguments)]
fn d_rounded_rect_out_w(
    d: &mut Canvas,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    fill: Rgba,
    outline: Rgba,
    width: u32,
) {
    d.rounded_rect(x0, y0, x1, y1, radius, fill);
    d.rounded_rect_outline(x0, y0, x1, y1, radius, outline, width);
}

/// Python `draw_glow(d, x, y, radius, color, steps)` — shading halo for
/// light sources, drawn largest-and-dimmest first.
fn draw_glow(d: &mut Canvas, x: i32, y: i32, radius: i32, color: [u8; 3], steps: i32) {
    for i in (1..=steps).rev() {
        let r = radius * i / steps;
        let shade = color.map(|c| {
            let k = 0.45 + 0.55 * (steps - i + 1) as f64 / steps as f64;
            clamp_byte(py_int(c as f64 * k))
        });
        d_ellipse(
            d,
            x - r,
            y - r,
            x + r,
            y + r,
            rgb(shade[0], shade[1], shade[2]),
        );
    }
}

// ---------------------------------------------------------------------------
// Python function ports
// ---------------------------------------------------------------------------

/// Port of `scene_overlay(im, motif, frame, seed)` — adds a moving,
/// scene-specific foreground to the v5.9 rendered scene and restores the
/// letterbox framing plus a scene-progress meter.
pub fn scene_overlay(mut im: Canvas, motif: &str, frame: usize, seed: u64) -> Canvas {
    let d = &mut im;
    let fi = frame as i32;
    // Python: rng = random.Random(seed + frame * 97)
    let mut rng = Rng::new(Seed::new(seed + frame as u64 * 97));
    let dark = rgb(13, 19, 21);
    let steel = rgb(113, 130, 128);
    let hi = rgb(181, 188, 166);
    let olive = rgb(91, 108, 65);
    // Declared in the Python function and unused there too; kept verbatim.
    #[allow(unused_variables)]
    let rust = rgb(160, 67, 42);
    let amber = rgb(242, 168, 66);
    // Python `ground = 132` is declared but never used; kept for parity.
    #[allow(unused_variables)]
    let ground: i32 = 132;

    if motif == "railway" {
        // Parallel rails and sleepers provide a strong foreground silhouette.
        for y in [137, 151] {
            d_line_w(d, 0, y, 320, y - 3, steel, 4);
            d_line_w(d, 0, y - 2, 320, y - 5, dark, 2);
        }
        let mut x = -5 + (fi * 3) % 22;
        while x < 330 {
            d_line_w(d, x, 133, x + 3, 153, rgb(91, 79, 60), 4);
            x += 22;
        }
        let tx = 176 - fi * 3;
        d_rect_out_w(d, tx, 112, tx + 72, 136, rgb(80, 73, 61), dark, 3);
        d_rect(d, tx + 8, 117, tx + 28, 128, rgb(36, 47, 48));
        d_rect(d, tx + 39, 117, tx + 59, 128, rgb(36, 47, 48));
        for wx in [tx + 12, tx + 55] {
            d_ellipse_out_w(d, wx, 132, wx + 10, 143, dark, steel, 2);
        }
        if frame == 2 || frame == 3 || frame == 4 {
            for j in 0..3 {
                draw_glow(d, tx + 52 + j * 7, 101 - j * 6, 7 + j, [104, 103, 88], 2);
            }
        }
        let rx = 28 + fi * 8;
        d_rect_out_w(d, rx, 121, rx + 32, 138, olive, dark, 3);
        d_rect_out_w(d, rx + 7, 114, rx + 23, 122, steel, dark, 2);
        d_line_w(d, rx + 20, 117, rx + 36, 111, hi, 3);
    } else if motif == "floodplain" {
        d_rect(d, 0, 125, 320, 154, rgb(29, 66, 82));
        for i in 0..9 {
            let x = (i * 41 + fi * 13) % 330 - 10;
            let y = 130 + (i % 3) * 7;
            d_line_w(d, x, y, x + 18, y - 1, rgb(111, 157, 162), 2);
        }
        d_rect_out_w(d, 18, 111, 300, 119, rgb(107, 104, 91), dark, 3);
        let mut x = 24;
        while x < 300 {
            d_line_w(d, x, 111, x + 10, 119, steel, 2);
            d_line_w(d, x + 10, 119, x + 21, 111, steel, 2);
            x += 21;
        }
        d_rect(d, 34, 119, 42, 139, rgb(89, 92, 80));
        d_rect(d, 268, 119, 276, 140, rgb(89, 92, 80));
        let vx = 64 + fi * 8;
        d_rect_out_w(d, vx, 101, vx + 40, 115, olive, dark, 3);
        d_rect_out_w(d, vx + 9, 96, vx + 24, 102, rgb(60, 86, 90), dark, 2);
        for x in [vx + 5, vx + 29] {
            d_ellipse_out_w(d, x, 111, x + 9, 120, dark, steel, 2);
        }
    } else if motif == "refinery" {
        for (x, y, r) in [(38, 101, 23), (84, 91, 18), (255, 100, 25)] {
            d_rect_out_w(d, x - r, y, x + r, 137, rgb(91, 86, 71), dark, 3);
            d_ellipse_out_w(d, x - r, y - 7, x + r, y + 7, rgb(129, 121, 95), dark, 2);
            d_line_w(d, x - r + 3, y + 18, x + r - 3, y + 18, rgb(59, 60, 54), 2);
        }
        for (x, ht) in [(143, 30), (171, 43), (205, 34)] {
            d_rect_out_w(d, x, 138 - ht, x + 13, 138, rgb(74, 83, 78), dark, 3);
            if frame.is_multiple_of(2) {
                draw_glow(d, x + 8, 99 - ht / 2, 5, [94, 99, 90], 2);
            }
        }
        d_polyline(
            d,
            &[(89, 128), (142, 128), (142, 118), (184, 118)],
            rgb(183, 102, 57),
            5,
        );
        if frame.is_multiple_of(3) {
            draw_glow(d, 190, 112, 8, [242, 168, 66], 3);
        }
        let vx = 108 + fi * 6;
        d_rect_out_w(d, vx, 130, vx + 38, 144, olive, dark, 3);
        d_rect_out_w(d, vx + 7, 123, vx + 22, 131, steel, dark, 2);
    } else if motif == "northwatch" {
        // Snow bank, communications mast, rotating radar dish, blinking beacon.
        d_polygon(
            d,
            &[
                (0, 142),
                (47, 127),
                (102, 141),
                (171, 130),
                (240, 140),
                (320, 128),
                (320, 180),
                (0, 180),
            ],
            rgb(173, 193, 192),
        );
        d_rect_out_w(d, 236, 83, 279, 139, rgb(82, 94, 92), dark, 3);
        d_rect_out_w(d, 245, 93, 270, 111, rgb(34, 61, 68), steel, 2);
        d_line_w(d, 260, 84, 260, 49, hi, 3);
        d_line_w(d, 260, 61, 229 + fi * 2, 49, hi, 4);
        d.arc(
            218,
            38,
            278,
            75,
            190.0 + (fi * 5) as f64,
            340.0 + (fi * 5) as f64,
            rgb(111, 183, 164),
            2,
        );
        draw_glow(
            d,
            259,
            80,
            if fi % 2 != 0 { 5 } else { 3 },
            [110, 211, 160],
            2,
        );
        d_rect_out_w(d, 60, 126, 103, 143, olive, dark, 3);
        d_rect_out_w(d, 68, 118, 88, 127, steel, dark, 2);
        for _ in 0..16 {
            let x = (rng.randrange2(0, 320) + fi * 3) % 320;
            let y = rng.randrange2(80, 145);
            d_line(d, x, y, x - 2, y + 7, rgb(220, 231, 225));
        }
    } else if motif == "night_column" {
        d_rect(d, 0, 129, 320, 155, rgb(20, 31, 32));
        d_line_w(d, 0, 151, 320, 142, rgb(104, 94, 70), 4);
        for i in 0..4usize {
            let x = 8 + i as i32 * 81 - (fi * 5) % 30;
            d_rect_out_w(
                d,
                x,
                116 - (i % 2) as i32 * 4,
                x + 47,
                139,
                [60 + i as u8 * 4, 72 + i as u8 * 2, 57, 255],
                dark,
                3,
            );
            d_rect_out_w(
                d,
                x + 8,
                108 - (i % 2) as i32 * 4,
                x + 27,
                117,
                rgb(80, 96, 95),
                dark,
                2,
            );
            for wx in [x + 7, x + 34] {
                d_ellipse_out_w(d, wx, 133, wx + 10, 143, dark, steel, 2);
            }
            draw_glow(
                d,
                x + 43,
                126,
                if frame < 4 { 5 } else { 3 },
                [240, 211, 139],
                2,
            );
        }
        for i in 0..3 {
            d_line_w(d, 9, 99 + i * 5, 62, 99 + i * 5, rgb(61, 77, 73), 2);
        }
    } else if motif == "air_defense" {
        // Flight silhouette with a moving exhaust and flak burst.
        let ax = 38 + fi * 24;
        let ay = 61 - py_int((fi as f64 / 7.0 * std::f64::consts::PI).sin() * 13.0);
        d_polygon_out(
            d,
            &[
                (ax, ay),
                (ax + 26, ay - 5),
                (ax + 47, ay),
                (ax + 27, ay + 5),
            ],
            rgb(166, 178, 171),
            dark,
        );
        d_polygon_out(
            d,
            &[
                (ax + 16, ay),
                (ax + 24, ay - 20),
                (ax + 32, ay),
                (ax + 24, ay + 2),
            ],
            steel,
            dark,
        );
        d_line_w(
            d,
            (ax - 28).max(0),
            ay + 2,
            ax - 2,
            ay + 2,
            if frame < 3 { amber } else { rgb(119, 120, 112) },
            3,
        );
        if frame == 3 || frame == 4 || frame == 5 {
            let cx = (ax + 22).min(300);
            let cy = ay + 9;
            for j in 0..8i32 {
                let a = j as f64 * std::f64::consts::PI / 4.0 + (fi % 2) as f64 * 0.12;
                d_line_w(
                    d,
                    cx,
                    cy,
                    py_int(cx as f64 + a.cos() * 14.0),
                    py_int(cy as f64 + a.sin() * 14.0),
                    rgb(235, 179, 93),
                    2,
                );
            }
        }
        d_rect_out_w(d, 18, 127, 92, 143, olive, dark, 3);
        d_rect_out_w(d, 29, 119, 55, 128, steel, dark, 2);
        d_line_w(d, 55, 124, 74, 103, hi, 5);
    } else if motif == "foundry" {
        d_rect(d, 0, 112, 320, 158, rgb(42, 46, 42));
        d_rect_out_w(d, 30, 77, 287, 132, rgb(73, 78, 73), dark, 4);
        for x in [45, 92, 141, 190, 239] {
            d_rect_out_w(
                d,
                x,
                90,
                x + 24,
                122,
                rgb(34, 45, 45),
                rgb(109, 120, 112),
                2,
            );
        }
        d_rect_out_w(d, 72, 62, 89, 99, rgb(77, 83, 79), dark, 3);
        d_rect_out_w(d, 217, 54, 234, 98, rgb(77, 83, 79), dark, 3);
        for i in 0..5 {
            let x = 122 + i * 4 + fi % 3;
            let y = 104 + (i % 2) * 5;
            d_line_w(d, x, y, x + 10, y - 8, amber, 2);
        }
        d_rect_out_w(d, 108, 126, 178, 141, rgb(61, 70, 59), dark, 3);
        d_rect_out_w(d, 124, 118, 156, 127, steel, dark, 2);
        if frame == 4 || frame == 5 {
            draw_glow(d, 197, 104, 9, [230, 124, 52], 3);
        }
    } else if motif == "battery" {
        d_polygon(
            d,
            &[
                (0, 125),
                (35, 96),
                (73, 124),
                (129, 91),
                (171, 121),
                (226, 96),
                (320, 125),
                (320, 180),
                (0, 180),
            ],
            rgb(42, 52, 45),
        );
        d_rect_out_w(d, 222, 96, 307, 135, rgb(82, 88, 78), dark, 4);
        d_polygon_out(
            d,
            &[(218, 98), (237, 77), (292, 77), (311, 98)],
            rgb(99, 106, 94),
            dark,
        );
        d_rect_out_w(d, 246, 109, 278, 126, rgb(21, 29, 29), steel, 2);
        d_ellipse_out_w(d, 39, 132, 57, 150, dark, steel, 2);
        d_ellipse_out_w(d, 74, 132, 92, 150, dark, steel, 2);
        d_rect_out_w(d, 43, 123, 88, 137, olive, dark, 3);
        d_line_w(d, 81, 127, 126, 107, hi, 5);
        if frame == 1 || frame == 2 || frame == 3 || frame == 5 {
            let cx = 147 + fi * 13;
            let cy = 117 - (fi % 3) * 6;
            draw_glow(d, cx, cy, 8 + fi, [224, 119, 51], 3);
            d_ellipse(d, cx - 3, cy - 3, cx + 3, cy + 3, rgb(255, 209, 109));
        }
    } else if motif == "homecoming" {
        d_rect(d, 0, 131, 320, 159, rgb(50, 63, 48));
        d_line_w(d, 0, 154, 320, 139, rgb(157, 136, 96), 5);
        for (x, y) in [(18, 101), (65, 92), (228, 99), (265, 89)] {
            d_rect_out_w(d, x, y, x + 35, 137, rgb(106, 91, 74), dark, 3);
            d_polygon_out(
                d,
                &[(x - 4, y), (x + 16, y - 14), (x + 39, y)],
                rgb(91, 71, 58),
                dark,
            );
            d_rect(d, x + 7, y + 10, x + 14, y + 19, rgb(33, 52, 54));
        }
        d_rect_out_w(d, 118, 122, 172, 140, olive, dark, 3);
        d_rect_out_w(d, 127, 114, 151, 123, steel, dark, 2);
        // Walkers and medical cross are separate pixels, not text baked into art.
        for i in 0..4 {
            let x = 82 + i * 16 - fi * 2;
            d_ellipse(d, x, 123, x + 6, 129, rgb(185, 142, 104));
            d_rect(d, x + 1, 129, x + 6, 139, rgb(96, 111, 81));
        }
        d_rect(d, 202, 111, 212, 137, rgb(218, 216, 191));
        d_rect(d, 194, 119, 220, 128, rgb(218, 216, 191));
    } else if motif == "frontier" {
        d_polygon(
            d,
            &[
                (0, 133),
                (40, 115),
                (94, 132),
                (154, 108),
                (204, 132),
                (264, 113),
                (320, 130),
                (320, 180),
                (0, 180),
            ],
            rgb(47, 64, 48),
        );
        d_polygon(
            d,
            &[
                (0, 153),
                (70, 140),
                (136, 152),
                (205, 139),
                (320, 149),
                (320, 180),
                (0, 180),
            ],
            rgb(83, 85, 64),
        );
        d_polyline(
            d,
            &[(143, 179), (160, 147), (165, 127), (173, 109)],
            rgb(177, 154, 105),
            7,
        );
        d_rect_out_w(d, 233, 101, 279, 133, rgb(87, 94, 76), dark, 3);
        d_polygon_out(
            d,
            &[(229, 101), (256, 83), (284, 101)],
            rgb(106, 104, 81),
            dark,
        );
        d_line_w(d, 254, 87, 254, 55, hi, 3);
        d_polygon_out(
            d,
            &[(257, 57), (284, 64), (257, 72)],
            rgb(87, 155, 110),
            dark,
        );
        for x in [39, 53, 67] {
            d_rect_out(d, x, 131, x + 8, 139, rgb(174, 148, 96), dark);
            draw_glow(d, x + 4, 129, 3, [242, 168, 66], 2);
        }
    }
    // Little signal / dust marks animate even in the later half of each scene.
    if frame == 1 || frame == 3 || frame == 6 {
        for _ in 0..8 {
            let x = rng.randrange2(14, 306);
            let y = rng.randrange2(99, 148);
            d_point(d, x, y, rgb(165, 157, 122));
        }
    }
    // Restore clean letterbox framing and a scene-progress meter.
    d_rect(d, 0, 0, 319, 12, rgb(4, 7, 9));
    d_rect(d, 0, 160, 319, 179, rgb(4, 7, 9));
    d_rect(d, 13, 166, 306, 169, rgb(36, 45, 45));
    let meter_end = 13 + py_int(293.0 * (frame as f64 + 1.0) / 8.0);
    d_rect(d, 13, 166, meter_end, 169, rgb(239, 142, 61));
    im
}

/// Port of `prop_sprite(name, seed)` — one transparent 64×64 prop using
/// reusable pixel-art construction rules. Seed derivation is literal:
/// `seed + sum(ord(c) for c in name)`.
pub fn prop_sprite(name: &str, seed: u64) -> Canvas {
    // The Python generator creates this rng and only draws from it at the
    // very end (seven texture speckles).
    let mut rng = Rng::new(Seed::new(
        seed + name.chars().map(|c| c as u64).sum::<u64>(),
    ));
    let mut im = Canvas::transparent(64, 64);
    let edge = rgb(13, 18, 19);
    let steel = rgb(105, 122, 119);
    let hi = rgb(177, 187, 162);
    let olive = rgb(88, 106, 64);
    let rust = rgb(157, 65, 43);
    let tan = rgb(149, 126, 85);

    // Each asset has a distinct silhouette, but uses a restrained shared
    // palette. Branch order mirrors the Python if/elif chain exactly (the
    // first match wins, which is why `antenna_mast`, `communications_tower`
    // and `civilian_shelter` follow their earlier branches).
    if name == "oil_storage_tank" || name == "water_tower" {
        d_rect_out_w(&mut im, 9, 24, 55, 55, rgb(91, 91, 76), edge, 2);
        d_ellipse_out_w(&mut im, 9, 17, 55, 31, rgb(133, 128, 100), edge, 2);
        d_rect(&mut im, 12, 31, 52, 34, rgb(58, 62, 56));
        if name == "water_tower" {
            d_rect(&mut im, 17, 48, 21, 62, steel);
            d_rect(&mut im, 43, 48, 47, 62, steel);
        }
    } else if name == "pumpjack" || name == "crane_hook" {
        d_rect_out(&mut im, 7, 53, 57, 59, rgb(79, 83, 74), edge);
        d_polyline(&mut im, &[(14, 50), (29, 18), (39, 18), (49, 50)], steel, 5);
        d_line_w(
            &mut im,
            14,
            22,
            50,
            32,
            if name == "pumpjack" { rust } else { hi },
            5,
        );
        d_line_w(&mut im, 32, 27, 32, 51, hi, 3);
        if name == "pumpjack" {
            d_ellipse_out(&mut im, 26, 47, 38, 58, rgb(56, 61, 55), edge);
        } else {
            d_polyline(&mut im, &[(31, 49), (26, 56), (32, 60)], steel, 2);
        }
    } else if name == "portable_radar"
        || name == "radar_sweep"
        || name == "relay_dish"
        || name == "communications_tower"
        || name == "antenna_mast"
    {
        d_rect_out(&mut im, 29, 25, 35, 58, steel, edge);
        d_rect_out(&mut im, 17, 56, 47, 61, rgb(87, 89, 78), edge);
        if name == "portable_radar" || name == "relay_dish" {
            im.arc(10, 6, 53, 45, 198.0, 342.0, hi, 3);
            d_ellipse_out(&mut im, 22, 18, 41, 30, rgb(130, 145, 133), edge);
            d_line_w(
                &mut im,
                31,
                24,
                45,
                14,
                if name == "relay_dish" { rust } else { hi },
                2,
            );
        } else {
            d_line_w(&mut im, 32, 28, 12, 13, hi, 2);
            d_line_w(&mut im, 32, 31, 52, 11, hi, 2);
            for x in [12, 52] {
                d_ellipse_out(&mut im, x - 3, 8, x + 3, 14, rgb(116, 199, 150), edge);
            }
        }
        if name == "radar_sweep" {
            im.arc(5, 3, 59, 57, 210.0, 300.0, rgb(98, 206, 163), 2);
        }
    } else if name == "radar_truck"
        || name == "mobile_repair_unit"
        || name == "tracked_hauler"
        || name == "snowmobile"
        || name == "evacuation_bus"
        || name == "wrecked_carrier"
    {
        let y0 = if name != "evacuation_bus" { 34 } else { 28 };
        d_rect_out_w(
            &mut im,
            5,
            y0,
            59,
            49,
            if name == "evacuation_bus" {
                rgb(113, 120, 100)
            } else {
                olive
            },
            edge,
            2,
        );
        d_rect_out_w(
            &mut im,
            13,
            y0 - 10,
            36,
            y0,
            if name == "radar_truck" || name == "mobile_repair_unit" {
                steel
            } else {
                olive
            },
            edge,
            2,
        );
        d_rect_out(&mut im, 17, y0 - 8, 29, y0 - 2, rgb(52, 79, 88), edge);
        for x in [11, 42] {
            d_ellipse_out_w(&mut im, x, 45, x + 11, 57, edge, steel, 2);
        }
        if name == "radar_truck" {
            d_line_w(&mut im, 33, y0 - 4, 42, 11, hi, 3);
            im.arc(29, 4, 55, 29, 190.0, 335.0, hi, 2);
        }
        if name == "mobile_repair_unit" {
            d_line_w(&mut im, 44, y0 + 4, 52, y0 + 13, hi, 3);
            d_line_w(&mut im, 52, y0 + 13, 44, y0 + 20, hi, 3);
        }
        if name == "snowmobile" {
            d_line_w(&mut im, 4, 56, 58, 56, steel, 3);
        }
        if name == "wrecked_carrier" {
            d_polyline(&mut im, &[(12, 30), (22, 38), (30, 28), (42, 38)], rust, 3);
        }
        if name == "evacuation_bus" {
            for x in [8, 21, 34, 47] {
                d_rect(&mut im, x, y0 + 3, x + 8, y0 + 10, rgb(37, 62, 65));
            }
        }
    } else if name == "rail_crane"
        || name == "searchlight"
        || name == "small_hangar"
        || name == "repair_bay"
    {
        d_rect_out_w(&mut im, 6, 45, 58, 58, rgb(89, 94, 85), edge, 2);
        d_rect_out(&mut im, 11, 38, 21, 47, steel, edge);
        d_rect_out(&mut im, 44, 38, 54, 47, steel, edge);
        d_polyline(&mut im, &[(15, 39), (33, 9), (54, 21)], hi, 4);
        if name == "searchlight" {
            // Python passes a four-tuple RGBA fill with alpha 115; Pillow
            // replaces pixels outright (no blending), so keep the alpha.
            im.polygon(
                &[(34, 12), (57, 2), (60, 13), (37, 20)],
                Some([225, 215, 157, 115]),
                None,
            );
        } else if name == "rail_crane" {
            d_polyline(&mut im, &[(53, 21), (53, 41), (47, 44)], hi, 3);
        } else if name == "small_hangar" || name == "repair_bay" {
            d_polygon_out(
                &mut im,
                &[(5, 28), (31, 9), (59, 28)],
                rgb(95, 101, 91),
                edge,
            );
            d_rect_out_w(&mut im, 14, 29, 49, 51, rgb(42, 52, 51), edge, 2);
        }
    } else if name == "pontoon_bridge"
        || name == "pontoon_segment"
        || name == "rescue_boat"
        || name == "rail_crossing"
        || name == "supply_sled"
    {
        d_polygon_out(
            &mut im,
            &[(4, 30), (58, 25), (61, 43), (8, 49)],
            rgb(109, 123, 118),
            edge,
        );
        let mut x = 10;
        while x < 57 {
            d_line_w(&mut im, x, 29, x + 1, 45, rgb(58, 71, 70), 2);
            x += 10;
        }
        if name == "rescue_boat" {
            d_polygon_out(
                &mut im,
                &[(5, 34), (28, 40), (55, 31), (46, 47), (16, 48)],
                rust,
                edge,
            );
        }
        if name == "rail_crossing" {
            d_line_w(&mut im, 21, 30, 48, 7, rust, 5);
            d_line_w(&mut im, 4, 22, 60, 47, hi, 3);
        }
        if name == "supply_sled" {
            d_line_w(&mut im, 6, 52, 56, 52, hi, 3);
        }
    } else if name == "missile_launcher" || name == "aa_gun" || name == "flak_battery" {
        d_ellipse_out_w(&mut im, 8, 42, 24, 58, edge, steel, 2);
        d_ellipse_out_w(&mut im, 40, 42, 56, 58, edge, steel, 2);
        d_rect_out_w(&mut im, 15, 34, 49, 46, olive, edge, 2);
        d_line_w(&mut im, 30, 36, 55, 12, hi, 5);
        if name == "missile_launcher" {
            d_line_w(&mut im, 25, 33, 44, 14, rust, 3);
            d_ellipse_out(&mut im, 48, 7, 57, 16, rgb(207, 164, 86), edge);
        }
        if name == "flak_battery" {
            d_line_w(&mut im, 32, 38, 47, 10, steel, 3);
        }
    } else if name == "field_generator"
        || name == "power_substation"
        || name == "portable_work_light"
    {
        d_rect_out_w(&mut im, 9, 28, 55, 55, rgb(82, 94, 87), edge, 2);
        for x in [16, 27, 38, 49] {
            d_rect(&mut im, x, 35, x + 4, 48, rgb(34, 48, 48));
        }
        d_line_w(&mut im, 12, 23, 53, 23, hi, 2);
        if name == "power_substation" {
            for x in [17, 32, 47] {
                d_line_w(&mut im, x, 7, x, 28, steel, 2);
            }
            d_line_w(&mut im, 12, 13, 53, 13, hi, 2);
        }
        if name == "portable_work_light" {
            d_line_w(&mut im, 32, 28, 32, 12, steel, 3);
            im.polygon(
                &[(22, 9), (42, 9), (38, 16), (26, 16)],
                Some(rgb(231, 220, 167)),
                None,
            );
        }
    } else if name == "fuel_barrels" || name == "ammunition_stack" || name == "supply_depot" {
        for row in 0..2usize {
            for col in 0..3usize {
                let x = 6 + col as i32 * 18;
                let y = 29 + row as i32 * 16;
                d_rect_out_w(
                    &mut im,
                    x,
                    y,
                    x + 14,
                    y + 15,
                    if name == "fuel_barrels" { rust } else { tan },
                    edge,
                    2,
                );
                d_line_w(&mut im, x + 2, y + 4, x + 12, y + 4, rgb(211, 177, 122), 2);
            }
        }
        if name == "supply_depot" {
            d_rect_out_w(&mut im, 3, 18, 61, 27, rgb(84, 89, 80), edge, 2);
        }
    } else if name == "field_kitchen"
        || name == "medical_tent"
        || name == "civilian_shelter"
        || name == "sandbag_command_post"
    {
        d_polygon_out(
            &mut im,
            &[(5, 33), (31, 11), (59, 33)],
            rgb(116, 110, 82),
            edge,
        );
        d_rect_out_w(&mut im, 11, 32, 53, 55, rgb(132, 121, 89), edge, 2);
        d_rect_out(&mut im, 26, 37, 39, 55, rgb(37, 48, 47), edge);
        if name == "medical_tent" {
            d_rect(&mut im, 29, 28, 35, 43, rgb(230, 226, 205));
            d_rect(&mut im, 24, 33, 40, 38, rgb(230, 226, 205));
        }
        if name == "sandbag_command_post" {
            let mut x = 6;
            while x < 57 {
                d_rounded_rect_out(&mut im, x, 44, x + 12, 53, 2, rgb(132, 119, 81), edge);
                x += 10;
            }
        }
    } else if name == "refinery_pipework"
        || name == "antenna_mast"
        || name == "communications_tower"
        || name == "portable_work_light"
    {
        // Unreachable for `antenna_mast` / `communications_tower` /
        // `portable_work_light` (earlier branches catch them first); kept
        // for chain fidelity.
        d_polyline(
            &mut im,
            &[
                (8, 53),
                (8, 25),
                (24, 25),
                (24, 43),
                (41, 43),
                (41, 17),
                (56, 17),
            ],
            steel,
            5,
        );
        d_line_w(&mut im, 8, 53, 56, 53, hi, 3);
    } else if name == "reconstruction_house"
        || name == "warehouse_door"
        || name == "civilian_shelter"
    {
        d_rect_out_w(&mut im, 7, 24, 57, 57, rgb(125, 102, 77), edge, 2);
        d_polygon_out(
            &mut im,
            &[(4, 24), (31, 7), (60, 24)],
            rgb(90, 70, 57),
            edge,
        );
        d_rect_out(&mut im, 26, 39, 38, 57, rgb(41, 51, 49), edge);
        for x in [13, 42] {
            d_rect_out(&mut im, x, 31, x + 8, 39, rgb(47, 70, 73), edge);
        }
        if name == "warehouse_door" {
            d_rect_out_w(&mut im, 18, 32, 46, 56, rgb(62, 73, 70), edge, 2);
        }
    } else if name == "damaged_helicopter" || name == "fighter_plane" || name == "forward_observer"
    {
        d_polygon_out(
            &mut im,
            &[
                (5, 32),
                (29, 27),
                (49, 31),
                (60, 34),
                (48, 37),
                (28, 39),
                (5, 35),
            ],
            steel,
            edge,
        );
        d_line_w(&mut im, 29, 31, 17, 13, hi, 4);
        d_line_w(&mut im, 29, 31, 47, 13, hi, 4);
        if name == "damaged_helicopter" {
            d_line_w(&mut im, 35, 38, 46, 49, rust, 3);
        }
        if name == "forward_observer" {
            d_rect_out(&mut im, 23, 26, 40, 37, olive, edge);
        }
    } else if name == "bunker_console" || name == "command_table" {
        d_rect_out_w(&mut im, 7, 25, 57, 53, rgb(70, 82, 77), edge, 2);
        d_rect_out(&mut im, 13, 29, 51, 43, rgb(28, 52, 56), edge);
        for x in [16, 25, 34, 43] {
            d_ellipse(&mut im, x, 46, x + 4, 50, rgb(126, 201, 145));
        }
        d_line_w(&mut im, 11, 55, 53, 55, steel, 3);
    } else if name == "road_sign" || name == "radio_beacon" || name == "signal_lamp" {
        d_rect_out(&mut im, 30, 17, 34, 58, steel, edge);
        d_rect_out_w(&mut im, 11, 15, 53, 34, rgb(95, 111, 94), edge, 2);
        if name == "road_sign" {
            d_line_w(&mut im, 17, 24, 46, 24, hi, 2);
        } else if name == "radio_beacon" {
            d_ellipse_out(&mut im, 21, 7, 43, 27, rgb(97, 204, 151), edge);
            im.arc(9, 1, 55, 41, 200.0, 340.0, rgb(97, 204, 151), 2);
        } else {
            d_ellipse_out(&mut im, 21, 10, 43, 29, rgb(235, 181, 81), edge);
        }
    } else {
        // Fallback remains visible and stable for future data-driven additions.
        d_rect_out_w(&mut im, 10, 17, 54, 56, olive, edge, 3);
        d_line_w(&mut im, 12, 19, 52, 54, hi, 3);
    }
    // Controlled texture, leaving silhouette contrast legible at native size.
    for _ in 0..7 {
        let x = rng.randrange2(8, 57);
        let y = rng.randrange2(18, 57);
        d_point(&mut im, x, y, rgb(170, 158, 120));
    }
    im
}

/// Port of `effect_frame(name, frame)` — one 32×32 frame of a looping
/// combat/weather/signal effect.
pub fn effect_frame(name: &str, frame: usize) -> Canvas {
    let mut im = Canvas::transparent(TILE, TILE);
    let fi = frame as i32;
    // Python computes `t = frame / 5` but never uses it; kept as a comment
    // for parity instead of dead code.
    if name == "muzzle_flash" {
        let r = (10 - fi).max(2);
        d_polygon(
            &mut im,
            &[(4, 15), (16, 15 - r), (27, 16), (16, 17 + r)],
            [250, 211, 108, clamp_byte(245 - fi * 38)],
        );
        d_ellipse(
            &mut im,
            12,
            11,
            21,
            20,
            [255, 241, 177, clamp_byte(240 - fi * 37)],
        );
    } else if name == "shell_burst" {
        let r = 4 + fi * 3;
        for i in 0..8i32 {
            let a = i as f64 * std::f64::consts::PI / 4.0;
            let x = py_int(16.0 + a.cos() * r as f64);
            let y = py_int(16.0 + a.sin() * r as f64);
            d_line_w(
                &mut im,
                16,
                16,
                x,
                y,
                [
                    246,
                    clamp_byte(151 + fi * 10),
                    65,
                    clamp_byte(240 - fi * 36),
                ],
                if frame < 4 { 2 } else { 1 },
            );
        }
        d_ellipse(
            &mut im,
            16 - r / 2,
            16 - r / 2,
            16 + r / 2,
            16 + r / 2,
            [252, 206, 108, clamp_byte(230 - fi * 35)],
        );
    } else if name == "dust_plume" {
        for i in 0..5i32 {
            let r = 3 + fi + i % 2;
            let x = 5 + i * 5 + fi * 2;
            let y = 27 - fi * 3 - (i % 3) * 3;
            d_ellipse(
                &mut im,
                x - r,
                y - r,
                x + r,
                y + r,
                [156, 140, 99, clamp_byte(180 - fi * 29)],
            );
        }
    } else if name == "water_splash" {
        let height = 4 + fi * 3;
        im.arc(
            3,
            19 - height,
            29,
            31 + height / 2,
            190.0,
            350.0,
            [127, 202, 209, clamp_byte(230 - fi * 31)],
            2,
        );
        for i in 0..5i32 {
            let x = 5 + i * 5;
            let y = 22 - (fi + i % 3) * 2;
            d_ellipse(
                &mut im,
                x,
                y,
                x + 2,
                y + 3,
                [160, 220, 222, clamp_byte(230 - fi * 34)],
            );
        }
    } else if name == "radar_pulse" {
        let r = 3 + fi * 4;
        im.arc(
            16 - r,
            16 - r,
            16 + r,
            16 + r,
            195.0,
            345.0,
            [93, 223, 164, clamp_byte(240 - fi * 34)],
            2,
        );
        d_ellipse(
            &mut im,
            14,
            14,
            18,
            18,
            [176, 255, 193, clamp_byte(245 - fi * 30)],
        );
    } else if name == "fire_burst" {
        let points = [
            (16, 29),
            (7, 21 - fi / 2),
            (10, 13 - fi / 2),
            (15, 18 - fi),
            (19, 5 + fi),
            (24, 15 - fi / 2),
            (28, 20),
            (22, 29),
        ];
        d_polygon(
            &mut im,
            &points,
            [229, clamp_byte(89 + fi * 11), 45, clamp_byte(240 - fi * 29)],
        );
        d_polygon(
            &mut im,
            &[(16, 27), (14, 20), (18, 15 - fi / 2), (21, 23), (20, 28)],
            [255, 205, 94, clamp_byte(238 - fi * 31)],
        );
    } else if name == "smoke_plume" {
        for i in 0..5i32 {
            let r = 3 + i + fi / 2;
            let x = 15 + py_int((fi as f64 + i as f64).sin() * 3.0) + i * 2;
            let y = 28 - fi * 3 - i * 2;
            let c: Rgba = [
                clamp_byte(64 + i * 7),
                clamp_byte(70 + i * 6),
                clamp_byte(67 + i * 5),
                clamp_byte(195 - fi * 28 - i * 8),
            ];
            d_ellipse(&mut im, x - r, y - r, x + r, y + r, c);
        }
    } else {
        // electric_arc
        let pts = [
            (3, 7),
            (10, 12 + fi % 3),
            (7, 18),
            (18, 19 - (fi % 2) * 4),
            (14, 26),
            (28, 28),
        ];
        d_polyline(&mut im, &pts, [139, 221, 246, clamp_byte(245 - fi * 35)], 2);
        let shifted: Vec<(i32, i32)> = pts.iter().map(|(x, y)| (x + 1, *y)).collect();
        d_polyline(
            &mut im,
            &shifted,
            [239, 249, 255, clamp_byte(225 - fi * 36)],
            1,
        );
    }
    im
}

/// Port of `storyboard(asset_root, scene_id, frame_paths, title)` — a
/// 640×205 4×2 contact sheet with a labeled footer band.
fn storyboard(scene_id: &str, frames: &[Canvas], title: &str) -> Canvas {
    let (thumb_w, thumb_h) = (160, 90);
    let mut sheet = Canvas::new_filled(
        (thumb_w * 4) as u32,
        (thumb_h * 2 + 25) as u32,
        rgb(10, 15, 18),
    );
    for (i, frame) in frames.iter().enumerate() {
        let thumb = frame.resize_nearest(thumb_w as u32, thumb_h as u32);
        let (x, y) = ((i % 4) as i32 * thumb_w, (i / 4) as i32 * thumb_h);
        sheet.alpha_composite(&thumb, (x, y));
    }
    d_rect(
        &mut sheet,
        0,
        thumb_h * 2,
        thumb_w * 4,
        thumb_h * 2 + 25,
        rgb(8, 12, 15),
    );
    draw_text(
        &mut sheet,
        8,
        thumb_h * 2 + 6,
        &format!("{scene_id}  |  {title}"),
        rgb(221, 225, 210),
    );
    sheet
}

/// Port of `make_key_art(asset_root, card_id, title, base, accent, seed)` —
/// a 320×180 act card: gradient sky, randomized map terrain, armor
/// silhouettes and the deliberate chapter-card label.
fn make_key_art(card_id: &str, title: &str, base: [u8; 3], accent: [u8; 3], seed: u64) -> Canvas {
    let mut rng = Rng::new(Seed::new(seed));
    let mut im = Canvas::new_filled(320, 180, rgb(base[0], base[1], base[2]));
    for y in 0..94 {
        let t = y as f64 / 94.0;
        let col: [u8; 3] = base.map(|c| {
            clamp_byte(py_int(
                c as f64 * (1.0 - t) + (c as f64 - 24.0).max(0.0) * t,
            ))
        });
        d_line_w(&mut im, 0, y, 319, y, rgb(col[0], col[1], col[2]), 1);
    }
    // Abstract high-contrast command-map terrain, armor silhouettes, act mark.
    for _ in 0..12 {
        let x = rng.randrange2(-30, 340);
        let y = rng.randrange2(66, 170);
        let fill: [u8; 3] = base.map(|c| clamp_byte(c as i64 + i64::from(rng.randrange2(-12, 13))));
        d_polygon(
            &mut im,
            &[(x, y + 20), (x + 32, y - 18), (x + 73, y + 20)],
            rgb(fill[0], fill[1], fill[2]),
        );
    }
    d_rect(&mut im, 0, 140, 319, 179, rgb(9, 14, 17));
    for i in 0..4usize {
        let x = 23 + i as i32 * 73;
        let dy = (i % 2) as i32 * 8;
        d_rounded_rect_out_w(
            &mut im,
            x,
            104 - dy,
            x + 44,
            131 - dy,
            3,
            rgb(24, 32, 31),
            rgb(129, 143, 125),
            2,
        );
        d_rect_out(
            &mut im,
            x + 12,
            99 - dy,
            x + 31,
            106 - dy,
            rgb(80, 101, 69),
            rgb(20, 28, 26),
        );
        d_line_w(
            &mut im,
            x + 24,
            101 - dy,
            x + 47,
            97 - dy,
            rgb(167, 179, 159),
            3,
        );
    }
    d_rect(
        &mut im,
        14,
        13,
        20,
        130,
        rgb(accent[0], accent[1], accent[2]),
    );
    d_rect(
        &mut im,
        27,
        20,
        116,
        24,
        rgb(accent[0], accent[1], accent[2]),
    );
    d_rect(&mut im, 27, 30, 91, 33, rgb(209, 214, 194));
    // Title is metadata in the full timeline too; text here is a deliberate
    // chapter-card label.
    draw_text(&mut im, 29, 148, title, rgb(229, 230, 211));
    let _ = card_id;
    im
}

/// Port of `build_atlas(...)` — tile `cells` into a grid, write the atlas
/// PNG + rect-map JSON, and return the manifest `atlases` record together
/// with the atlas canvas (for the register step).
#[allow(clippy::too_many_arguments)]
fn build_atlas(
    out: &PackWriter,
    names: &[&str],
    cells: &[&Canvas],
    tile_size: u32,
    cols: usize,
    output_rel: &str,
    mapping_rel: &str,
    kind: &str,
) -> Result<(Value, Canvas), PackError> {
    let rows = cells.len().div_ceil(cols);
    let atlas = compose_atlas(cells, cols, tile_size, true);
    out.write_png(output_rel, &atlas)?;
    let mut rects = Map::new();
    for (i, name) in names.iter().enumerate() {
        rects.insert(
            (*name).to_string(),
            json!({
                "x": (i % cols) * tile_size as usize,
                "y": (i / cols) * tile_size as usize,
                "width": tile_size,
                "height": tile_size,
            }),
        );
    }
    out.write_json(
        mapping_rel,
        &json!({
            "version": VERSION,
            "kind": kind,
            "image": output_rel,
            "size": [atlas.width(), atlas.height()],
            "tile_size": tile_size,
            "columns": cols,
            "rows": rows,
            "sprites": rects,
        }),
    )?;
    let sha = out.sha256_file(output_rel)?;
    Ok((
        json!({
            "file": output_rel,
            "category": kind,
            "columns": cols,
            "rows": rows,
            "cell_size": [tile_size, tile_size],
            "sprite_count": cells.len(),
            "sha256_file": sha,
            "mapping": mapping_rel,
        }),
        atlas,
    ))
}

/// Insert or replace a JSON list member by its `"id"` string (the Python
/// scripts' `register` merge semantics).
fn upsert_by_id(list: &mut Vec<Value>, rec: Value) {
    let id = rec
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    if let Some(slot) = list
        .iter_mut()
        .find(|s| s.get("id").and_then(Value::as_str) == Some(id.as_str()))
    {
        *slot = rec;
    } else {
        list.push(rec);
    }
}

/// Python sort key for sequence records:
/// `(int(id.split("_")[0]) if isdigit else 999, id)`.
fn sequence_sort_key(rec: &Value) -> (u64, String) {
    let id = rec.get("id").and_then(Value::as_str).unwrap_or("");
    let prefix = id.split('_').next().unwrap_or("");
    let num = if prefix.bytes().all(|b| b.is_ascii_digit()) && !prefix.is_empty() {
        prefix.parse().unwrap_or(999)
    } else {
        999
    };
    (num, id.to_string())
}

/// Register an already-written PNG (the Python `atlas` files) as a manifest
/// sprite record without rewriting the file.
fn registered_record(
    out: &PackWriter,
    rel: &str,
    canvas: &Canvas,
    spec: SpriteSpec<'_>,
) -> Result<Value, PackError> {
    let bytes = std::fs::read(out.path(rel))?;
    Ok(sprite_record(rel, canvas, &bytes, spec))
}

/// Generate this layer into `out` and return its contributions, matching the
/// Python `generate(asset_root, seed=60017)` write set: 80 cinematic frames,
/// 10 storyboards, 48 props, a prop atlas (+ map), 48 effect frames, an effect
/// atlas (+ map), 8 portraits, 2 act cards, the merged timeline, the full-pack
/// overview, the merged manifest, and the pack-root build report.
pub fn generate(out: &PackWriter, seed: u64) -> Result<LayerOutput, PackError> {
    let cut = "sprites/cinematics";
    let frames_root = format!("{cut}/frames");
    let previews_root = format!("{cut}/previews");
    let cards_root = format!("{cut}/campaign_cards");
    let art_root = "sprites/campaign_art";
    let ui_root = "sprites/cinematic_ui";
    let effect_root = "sprites/animations_v6";
    let timeline_path = format!("{cut}/timeline.json");
    let manifest_path = "manifest.json";

    let mut manifest = match out.read_json(manifest_path)? {
        Some(m) => m,
        None => {
            return Err(PackError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Expected existing asset pack with manifest.json: {}",
                    out.path(manifest_path).display()
                ),
            )))
        }
    };
    let mut timeline = match out.read_json(&timeline_path)? {
        Some(t) => t,
        None => {
            return Err(PackError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Expected existing cinematics timeline: {}",
                    out.path(&timeline_path).display()
                ),
            )))
        }
    };
    let stable_timeline = timeline.clone();
    let old_sequences: Vec<Value> = stable_timeline
        .get("sequences")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let old_ids: HashSet<String> = old_sequences
        .iter()
        .filter_map(|s| s.get("id").and_then(Value::as_str).map(str::to_string))
        .collect();
    let old_notes: Vec<String> = stable_timeline
        .get("notes")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    let old_campaigns: Vec<Value> = stable_timeline
        .get("campaigns")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let old_support: HashSet<String> = stable_timeline
        .get("supporting_assets")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();

    let mut output = LayerOutput::default();
    // Canvases per scene, reused for storyboard thumbnails and the overview
    // (the Python scripts re-open the saved PNGs).
    let mut rendered: Vec<(&str, Vec<Canvas>)> = Vec::with_capacity(SCENES.len());
    let mut new_sequences: Vec<Value> = Vec::new();
    // Every generated file path, in Python `generated` order.
    let mut generated: Vec<String> = Vec::new();

    // Sprite list maintained in Python `register()` order (old records kept,
    // same ids replaced in place).
    let mut sprites: Vec<Value> = manifest
        .get("sprites")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    // 1) Ten new 8-frame cinematic sequences, each with coherent moving
    //    foreground art.
    for (scene_index, (seq_id, title, act, env, motif, captions)) in SCENES.iter().enumerate() {
        let scene_seed = seed + (scene_index as u64 + 1) * 101;
        // Python hands v5.9's renderer one of its own motifs: the epilogue
        // scenes ride the v5.9 "epilogue" motif, everything else "border";
        // the v6-specific foreground is drawn by `scene_overlay`.
        let v59_motif = if *motif == "homecoming" || *motif == "frontier" {
            "epilogue"
        } else {
            "border"
        };
        let mut frame_records: Vec<Value> = Vec::new();
        let mut frames: Vec<Canvas> = Vec::with_capacity(captions.len());
        for (fi, caption) in captions.iter().enumerate() {
            let frame_seed = scene_seed + fi as u64 * 131;
            let rel = format!("{frames_root}/{seq_id}/frame_{fi:03}.png");
            let im = campaign_v59::render_scene(frame_seed, env, v59_motif, fi);
            let im = scene_overlay(im, motif, fi, scene_seed);
            let record = out.write_sprite(
                &rel,
                &im,
                SpriteSpec {
                    category: "cinematic",
                    generated_by: Some(GENERATED_BY),
                    seed: Some(frame_seed),
                    extra: vec![
                        ("campaign_act", json!(act)),
                        ("sequence_id", json!(seq_id)),
                        ("frame_index", json!(fi)),
                        ("caption", json!(caption)),
                    ],
                    ..SpriteSpec::default()
                },
            )?;
            generated.push(rel.clone());
            frames.push(im);
            upsert_by_id(&mut sprites, record.clone());
            output.sprites.push(record.clone());
            frame_records.push(json!({
                "index": fi,
                "file": rel,
                "duration_ms": if fi == 7 { 1200 } else { 850 },
                "caption": caption,
                "seed": frame_seed,
                "sha256": record.get("sha256_file").cloned().unwrap_or(Value::Null),
            }));
        }
        let sp = storyboard(seq_id, &frames, title);
        let board_rel = format!("{previews_root}/{seq_id}_storyboard.png");
        let board_record = out.write_sprite(
            &board_rel,
            &sp,
            SpriteSpec {
                category: "cinematic_preview",
                generated_by: Some(GENERATED_BY),
                seed: Some(scene_seed),
                extra: vec![("sequence_id", json!(seq_id)), ("frame_count", json!(8))],
                ..SpriteSpec::default()
            },
        )?;
        generated.push(board_rel);
        upsert_by_id(&mut sprites, board_record.clone());
        output.sprites.push(board_record);

        let seq = json!({
            "id": seq_id,
            "title": title,
            "campaign_act": act,
            "fps": 10,
            "loop": false,
            "canvas": [W, H],
            "environment": env,
            "motif": motif,
            "frames": frame_records,
            "transitions": {"in": "fade_from_black", "out": "fade_to_black", "camera": "slow_pan_zoom",
                            "transition_in_ms": 500, "transition_out_ms": 650},
            "camera_path": [
                {"frame": 0, "zoom": 1.00, "pan_x": 0, "pan_y": 0},
                {"frame": 3, "zoom": 1.025, "pan_x": 3, "pan_y": -1},
                {"frame": 7, "zoom": 1.045, "pan_x": 6, "pan_y": 0},
            ],
            "audio_cues": ["environment_ambience", "radio_or_vehicle", "scene_transition"],
            "effects_cues": ["moving_foreground", "weather_or_atmosphere", "impact_or_signal"],
            "captions_external": true,
        });
        new_sequences.push(seq.clone());
        rendered.push((*seq_id, frames));
    }

    // 2) Reusable prop art set and its engine-friendly atlas/rect map.
    let mut prop_canvases: Vec<Canvas> = Vec::with_capacity(PROP_NAMES.len());
    for (i, name) in PROP_NAMES.iter().enumerate() {
        let rel = format!("{art_root}/{name}.png");
        let art = prop_sprite(name, seed + 13_000 + i as u64);
        let record = out.write_sprite(
            &rel,
            &art,
            SpriteSpec {
                category: "campaign_art",
                generated_by: Some(GENERATED_BY),
                seed: Some(seed + 13_000 + i as u64),
                extra: vec![
                    ("art_role", json!("campaign_prop")),
                    ("tile_size", json!([64, 64])),
                    ("revision", json!("v6.0")),
                ],
                ..SpriteSpec::default()
            },
        )?;
        generated.push(rel);
        prop_canvases.push(art);
        upsert_by_id(&mut sprites, record.clone());
        output.sprites.push(record);
    }
    let atlas_path = format!("{art_root}/campaign_art_atlas_v6.png");
    let atlas_map = format!("{art_root}/campaign_art_atlas_v6.json");
    let prop_cells: Vec<&Canvas> = prop_canvases.iter().collect();
    let (prop_atlas_record, prop_atlas_canvas) = build_atlas(
        out,
        &PROP_NAMES,
        &prop_cells,
        64,
        8,
        &atlas_path,
        &atlas_map,
        "campaign_prop_atlas",
    )?;
    generated.push(atlas_path.clone());
    generated.push(atlas_map.clone());
    let atlas_sprite_record = registered_record(
        out,
        &atlas_path,
        &prop_atlas_canvas,
        SpriteSpec {
            category: "campaign_atlas",
            generated_by: Some(GENERATED_BY),
            seed: Some(seed),
            extra: vec![
                ("art_role", json!("v6_campaign_prop_atlas")),
                ("tile_size", json!([64, 64])),
                ("columns", json!(8)),
                ("rows", json!(PROP_NAMES.len().div_ceil(8))),
            ],
            ..SpriteSpec::default()
        },
    )?;
    upsert_by_id(&mut sprites, atlas_sprite_record.clone());
    output.sprites.push(atlas_sprite_record);

    // 3) Eight looping combat/weather/signal effects, six genuinely
    //    different frames each.
    let mut effect_paths: Vec<String> = Vec::new();
    let mut effect_names: Vec<String> = Vec::new();
    let mut effect_canvases: Vec<Canvas> = Vec::new();
    for (effect_index, effect) in EFFECTS.iter().enumerate() {
        for fi in 0..6usize {
            let rel = format!("{effect_root}/{effect}/frame_{fi:02}.png");
            let frame = effect_frame(effect, fi);
            let record = out.write_sprite(
                &rel,
                &frame,
                SpriteSpec {
                    category: "effect_animation",
                    generated_by: Some(GENERATED_BY),
                    seed: Some(seed + effect_index as u64 * 17 + fi as u64),
                    extra: vec![
                        ("animation_id", json!(effect)),
                        ("frame_index", json!(fi)),
                        ("frame_count", json!(6)),
                        ("tile_size", json!([32, 32])),
                        ("loop", json!(true)),
                    ],
                    ..SpriteSpec::default()
                },
            )?;
            generated.push(rel.clone());
            effect_paths.push(rel);
            effect_names.push(format!("{effect}/frame_{fi:02}"));
            effect_canvases.push(frame);
            upsert_by_id(&mut sprites, record.clone());
            output.sprites.push(record);
        }
    }
    let effect_atlas_path = "atlases/v6_effect_animation.png";
    let effect_map_path = "atlases/v6_effect_animation.json";
    let effect_name_refs: Vec<&str> = effect_names.iter().map(String::as_str).collect();
    let effect_cells: Vec<&Canvas> = effect_canvases.iter().collect();
    let (effect_atlas_record, _effect_atlas_canvas) = build_atlas(
        out,
        &effect_name_refs,
        &effect_cells,
        32,
        6,
        effect_atlas_path,
        effect_map_path,
        "effect_animation_atlas",
    )?;
    generated.push(effect_atlas_path.to_string());
    generated.push(effect_map_path.to_string());

    // 4) Eight new character portraits and two act cards.
    for (i, name) in PORTRAITS.iter().enumerate() {
        let rel = format!("{ui_root}/{name}.png");
        let art = campaign_v59::portrait(name, i + 8);
        let record = out.write_sprite(
            &rel,
            &art,
            SpriteSpec {
                category: "cinematic_ui",
                generated_by: Some(GENERATED_BY),
                seed: Some(seed + i as u64 + 7_000),
                extra: vec![
                    ("art_role", json!("campaign_portrait")),
                    ("revision", json!("v6.0")),
                ],
                ..SpriteSpec::default()
            },
        )?;
        generated.push(rel);
        upsert_by_id(&mut sprites, record.clone());
        output.sprites.push(record);
    }
    for (i, (card_id, title, base, accent)) in ACT_CARD_SPECS.iter().enumerate() {
        let rel = format!("{cards_root}/{card_id}.png");
        let art = make_key_art(card_id, title, *base, *accent, seed + 9_000 + i as u64);
        let record = out.write_sprite(
            &rel,
            &art,
            SpriteSpec {
                category: "campaign_art",
                generated_by: Some(GENERATED_BY),
                seed: Some(seed + 9_000 + i as u64),
                extra: vec![
                    ("art_role", json!("campaign_act_key_art")),
                    ("campaign_act", json!(title)),
                    ("revision", json!("v6.0")),
                ],
                ..SpriteSpec::default()
            },
        )?;
        generated.push(rel);
        upsert_by_id(&mut sprites, record.clone());
        output.sprites.push(record);
    }

    // 5) New indexes and timeline. Prior scenes are kept, the same v6 IDs
    //    are replaced.
    let mut all_sequences = old_sequences;
    for seq in &new_sequences {
        upsert_by_id(&mut all_sequences, seq.clone());
    }
    all_sequences.sort_by_key(sequence_sort_key);
    let seqs = all_sequences;
    let mut campaigns: Vec<Value> = old_campaigns
        .into_iter()
        .filter(|c| {
            !matches!(
                c.get("id").and_then(Value::as_str),
                Some("act_4_counterpunch") | Some("act_5_final_offensive")
            )
        })
        .collect();
    let mut campaign_records: Vec<Value> = Vec::new();
    for (cid, act_title) in [
        ("act_4_counterpunch", "Act IV — Counterpunch"),
        ("act_5_final_offensive", "Act V — Final Offensive"),
    ] {
        campaign_records.push(json!({
            "id": cid,
            "title": act_title,
            "sequence_ids": SCENES.iter()
                .filter(|(_, _, scene_act, ..)| *scene_act == act_title)
                .map(|(id, ..)| id)
                .collect::<Vec<_>>(),
        }));
    }
    campaigns.extend(campaign_records.clone());

    {
        let obj = timeline.as_object_mut().expect("timeline is an object");
        obj.insert("schema_version".to_string(), json!("1.2.0"));
        obj.insert("generator_version".to_string(), json!(VERSION));
        obj.insert("seed".to_string(), json!(seed));
        obj.insert("canvas".to_string(), json!([W, H]));
        obj.insert("sequences".to_string(), Value::Array(seqs.clone()));
        obj.insert("campaigns".to_string(), Value::Array(campaigns.clone()));
    }
    // Notes deduped keep-first (Python `dict.fromkeys`).
    let mut notes = old_notes;
    for note in ADDED_NOTES {
        if !notes.iter().any(|n| n == note) {
            notes.push((*note).to_string());
        }
    }
    timeline
        .as_object_mut()
        .expect("timeline is an object")
        .insert("notes".to_string(), json!(notes));
    // Full-pack overview preview: first frame from each new scene, 5×2 cards.
    // Support artefact only — not registered in the manifest.
    let mut preview = Canvas::new_filled(5 * 256, 2 * 165, rgb(7, 11, 14));
    for (i, seq) in new_sequences.iter().enumerate() {
        let first = rendered[i].1[0].resize_nearest(256, 144);
        let x = ((i % 5) * 256) as i32;
        let y = ((i / 5) * 165) as i32;
        preview.alpha_composite(&first, (x, y));
        d_rect(&mut preview, x, y + 144, x + 255, y + 164, rgb(7, 11, 14));
        draw_text(
            &mut preview,
            x + 5,
            y + 148,
            &format!(
                "{}  |  {}",
                seq["id"].as_str().unwrap_or(""),
                seq["title"].as_str().unwrap_or("")
            ),
            rgb(223, 227, 212),
        );
    }
    let overview_path = "previews/v6_campaign_overview.png";
    out.write_png(overview_path, &preview)?;
    generated.push(overview_path.to_string());

    // Supporting assets = sorted(set(union of old + every generated PNG)).
    // The overview preview is rendered after the timeline write in the
    // Python script, so the shipped pack's timeline does not list it.
    let mut all_support: HashSet<String> = old_support;
    for p in &generated {
        if p.ends_with(".png") && p.as_str() != overview_path {
            all_support.insert(p.clone());
        }
    }
    let mut supporting_assets: Vec<String> = all_support.into_iter().collect();
    supporting_assets.sort();
    timeline
        .as_object_mut()
        .expect("timeline is an object")
        .insert("supporting_assets".to_string(), json!(supporting_assets));
    out.write_json(&timeline_path, &timeline)?;
    generated.push(timeline_path.clone());

    // Update manifest references and atlas records without dropping prior
    // records.
    let total_cinematic_frames: usize = seqs
        .iter()
        .map(|s| {
            s.get("frames")
                .and_then(Value::as_array)
                .map(|a| a.len())
                .unwrap_or(0)
        })
        .sum();
    let campaign_scene_count: usize = campaigns
        .iter()
        .map(|c| {
            c.get("sequence_ids")
                .and_then(Value::as_array)
                .map(|a| a.len())
                .unwrap_or(0)
        })
        .sum();
    let previous_art_count = manifest
        .get("cinematics")
        .and_then(|c| c.get("campaign_art_count"))
        .and_then(Value::as_i64)
        .unwrap_or(22) as usize;
    let cinematics = json!({
        "timeline": timeline_path,
        "sequence_count": seqs.len(),
        "frame_count": total_cinematic_frames,
        "campaign_scene_count": campaign_scene_count,
        "campaign_art_count": previous_art_count + PROP_NAMES.len() + ACT_CARD_SPECS.len(),
        "new_portrait_count": PORTRAITS.len(),
        "new_campaign_scene_count": new_sequences.len(),
        "new_effect_animation_count": EFFECTS.len(),
        "effects_frames_per_animation": 6,
    });
    let campaign_atlas_key = json!({
        "image": atlas_path,
        "mapping": atlas_map,
        "tile_size": [64, 64],
        "sprite_count": PROP_NAMES.len(),
    });
    {
        let obj = manifest.as_object_mut().expect("manifest is an object");
        obj.insert("sprites".to_string(), Value::Array(sprites.clone()));
        obj.insert("generator_version".to_string(), json!(VERSION));
        obj.insert(
            "release_note".to_string(),
            json!("v6.0 additive campaign and effects expansion; v5.x sprite set retained."),
        );
        obj.insert("cinematics".to_string(), cinematics.clone());
        obj.insert(
            "campaign_art_atlas_v6".to_string(),
            campaign_atlas_key.clone(),
        );
        // atlases: replace-by-file, then append (Python setdefault semantics).
        let atlases = obj
            .entry("atlases")
            .or_insert_with(|| Value::Array(Vec::new()));
        if !atlases.is_array() {
            *atlases = Value::Array(Vec::new());
        }
        let list = atlases.as_array_mut().expect("atlases is an array");
        for record in [&prop_atlas_record, &effect_atlas_record] {
            let file = record.get("file").and_then(Value::as_str).unwrap_or("");
            list.retain(|a| a.get("file").and_then(Value::as_str) != Some(file));
            list.push(record.clone());
        }
    }
    for (key, value) in [
        ("generator_version", json!(VERSION)),
        (
            "release_note",
            json!("v6.0 additive campaign and effects expansion; v5.x sprite set retained."),
        ),
        ("cinematics", cinematics),
        ("campaign_art_atlas_v6", campaign_atlas_key),
    ] {
        output.manifest_extra.insert(key.to_string(), value);
    }
    out.write_json(manifest_path, &manifest)?;

    // Build report at the pack root.
    let new_frame_hashes: HashSet<String> = new_sequences
        .iter()
        .flat_map(|s| {
            s.get("frames")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
        })
        .filter_map(|f| f.get("sha256").and_then(Value::as_str).map(str::to_string))
        .collect();
    let all_generated_files_exist = generated.iter().all(|p| out.path(p).is_file());
    let all_frame_hashes_match = new_sequences
        .iter()
        .flat_map(|s| {
            s.get("frames")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
        })
        .all(|f| {
            let file = f.get("file").and_then(Value::as_str).unwrap_or("");
            let expected = f.get("sha256").and_then(Value::as_str).unwrap_or_default();
            std::fs::read(out.path(file))
                .map(|bytes| sha256_hex(&bytes) == expected)
                .unwrap_or(false)
        });
    let previous_scenes_retained = old_ids.iter().all(|id| {
        seqs.iter()
            .any(|s| s.get("id").and_then(Value::as_str) == Some(id.as_str()))
    });
    let report = json!({
        "asset_pack_version": VERSION,
        "generator_version": VERSION,
        "seed": seed,
        "previous_sequence_count": old_ids.len(),
        "new_campaign_scene_count": new_sequences.len(),
        "total_sequence_count": seqs.len(),
        "campaign_act_count": campaigns.len(),
        "new_campaign_frames": new_sequences
            .iter()
            .map(|s| s.get("frames").and_then(Value::as_array).map(|a| a.len()).unwrap_or(0))
            .sum::<usize>(),
        "total_cinematic_frames": total_cinematic_frames,
        "new_storyboards": new_sequences.len(),
        "new_reusable_props": PROP_NAMES.len(),
        "new_effect_animations": EFFECTS.len(),
        "new_effect_frames": effect_paths.len(),
        "new_character_portraits": PORTRAITS.len(),
        "new_campaign_cards": ACT_CARD_SPECS.len(),
        "generated_files": generated.len(),
        "new_frame_hashes": new_frame_hashes.len(),
        "all_generated_files_exist": all_generated_files_exist,
        "all_frame_hashes_match": all_frame_hashes_match,
        "previous_scenes_retained": previous_scenes_retained,
        "prop_atlas_sprite_count": PROP_NAMES.len(),
        "effect_atlas_sprite_count": effect_paths.len(),
        "runtime_validation": "NOT_RUN — Godot playback and integration are separate checks.",
    });
    let report_path = "campaign_build_report_v6_0_0.json";
    out.write_json(report_path, &report)?;
    output.reports.push((report_path.to_string(), report));

    output.atlases = vec![prop_atlas_record, effect_atlas_record];
    output.timeline = Some(TimelineContribution {
        sequences: new_sequences,
        campaigns: campaign_records,
        notes: ADDED_NOTES.iter().map(|n| (*n).to_string()).collect(),
        schema_version: "1.2.0",
        generator_version: VERSION,
        seed: Some(seed),
        extra: Map::new(),
    });
    output.supporting_assets = supporting_assets;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    static TMP_SEQ: AtomicU32 = AtomicU32::new(0);

    /// Panic-safe temporary pack dir, mirroring the Python tests'
    /// `tempfile` usage.
    struct TempPack(PathBuf);

    impl TempPack {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "cnc-procgen-campaign_v60-{}-{}",
                std::process::id(),
                TMP_SEQ.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(root.join("sprites/cinematics")).unwrap();
            Self(root)
        }

        /// Seed a v5.9-era timeline + manifest so existing-sequence
        /// retention and manifest upserts are exercised.
        fn setup_legacy(&self) {
            let mut old_sequences = Vec::new();
            for i in 1..=6usize {
                old_sequences.push(json!({
                    "id": format!("{}_legacy_scene_{}", i, i),
                    "title": format!("Legacy {}", i),
                    "frames": [],
                }));
            }
            std::fs::write(
                self.0.join("sprites/cinematics/timeline.json"),
                serde_json::to_vec_pretty(&json!({
                    "schema_version": "1.1.0",
                    "generator_version": "5.9.0",
                    "canvas": [320, 180],
                    "sequences": old_sequences,
                    "campaigns": [{"id": "act_0_legacy", "title": "Legacy", "sequence_ids": []}],
                    "supporting_assets": [],
                    "notes": ["legacy note"],
                }))
                .unwrap(),
            )
            .unwrap();
            std::fs::write(
                self.0.join("manifest.json"),
                serde_json::to_vec_pretty(&json!({
                    "schema_version": "1.2.0",
                    "sprites": [],
                    "cinematics": {"campaign_art_count": 22},
                    "atlases": [],
                }))
                .unwrap(),
            )
            .unwrap();
        }

        fn read_json_rel(&self, rel: &str) -> Value {
            serde_json::from_slice(&std::fs::read(self.0.join(rel)).unwrap()).unwrap()
        }
    }

    impl Drop for TempPack {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    // Show a bounded before/around/after slice at the first differing byte,
    // instead of printing whole JSON manifests as byte arrays.
    fn byte_diff(label: &str, a: &[u8], b: &[u8]) {
        if a == b {
            return;
        }
        let i = a
            .iter()
            .zip(b)
            .position(|(x, y)| x != y)
            .unwrap_or_else(|| a.len().min(b.len()));
        let snap = |v: &[u8]| -> String {
            let lo = i.saturating_sub(60);
            let hi = (i + 900).min(v.len());
            String::from_utf8_lossy(&v[lo..hi]).to_string()
        };
        panic!(
            "{label}: first diff at byte {i} of {}/{}\nFIRST  ...{}\nSECOND ...{}",
            a.len(),
            b.len(),
            snap(a),
            snap(b)
        );
    }

    fn png_dims(path: &std::path::Path) -> (u32, u32) {
        let bytes = std::fs::read(path).unwrap();
        let img = image::load_from_memory(&bytes).unwrap();
        (img.width(), img.height())
    }

    /// Sequence ids newly added by this layer have numeric prefixes 19..28.
    fn is_v6_sequence(id: &str) -> bool {
        match id.split('_').next().unwrap_or("").parse::<u32>() {
            Ok(n) => (19..=28).contains(&n),
            Err(_) => false,
        }
    }

    // ----- Python test file mirrors -----------------------------------------

    #[test]
    fn expected_scene_sequence_is_ten_scenes() {
        assert_eq!(SCENES.len(), 10);
        let ids: HashSet<&str> = SCENES.iter().map(|(id, ..)| *id).collect();
        assert_eq!(ids.len(), 10);
        assert!(SCENES.iter().all(|(.., captions)| captions.len() == 8));
    }

    #[test]
    fn effects_have_six_distinct_frames() {
        for effect in EFFECTS {
            let mut raws: HashSet<Vec<u8>> = HashSet::new();
            for i in 0..6usize {
                let im = effect_frame(effect, i);
                assert_eq!((im.width(), im.height()), (32, 32), "{effect}");
                raws.insert(im.as_rgba().as_raw().clone());
            }
            assert_eq!(raws.len(), 6, "{effect}");
        }
    }

    #[test]
    fn props_are_transparent_64px_art() {
        assert_eq!(PROP_NAMES.len(), 48);
        for name in PROP_NAMES {
            let im = prop_sprite(name, DEFAULT_SEED);
            assert_eq!((im.width(), im.height()), (64, 64), "{name}");
            let min_alpha = im.as_rgba().pixels().map(|p| p.0[3]).min().unwrap_or(255);
            assert!(min_alpha < 255, "{name} should have transparent pixels");
        }
    }

    #[test]
    fn scene_overlay_has_expected_canvas() {
        let base = campaign_v59::render_scene(60_017, "dawn", "border", 0);
        let motifs: HashSet<&str> = SCENES.iter().map(|(_, _, _, _, motif, _)| *motif).collect();
        for motif in motifs {
            let out = scene_overlay(base.clone(), motif, 4, 60_017);
            assert_eq!((out.width(), out.height()), (320, 180), "{motif}");
        }
    }

    #[test]
    fn v6_has_two_new_campaign_acts() {
        assert_eq!(ACT_CARD_SPECS.len(), 2);
        let acts: HashSet<&str> = SCENES.iter().map(|(_, _, act, ..)| *act).collect();
        assert_eq!(
            acts,
            HashSet::from(["Act IV — Counterpunch", "Act V — Final Offensive"])
        );
    }

    // ----- Full-generate assertions -----------------------------------------

    #[test]
    fn generate_produces_the_full_v6_layer() {
        let pack = TempPack::new();
        pack.setup_legacy();
        let out = PackWriter::new(&pack.0);
        let output = generate(&out, 77).unwrap();

        let report = pack.read_json_rel("campaign_build_report_v6_0_0.json");
        assert_eq!(report["new_campaign_scene_count"], 10);
        assert_eq!(report["total_sequence_count"], 16);
        // Merged campaign list = 1 legacy + 2 new acts.
        assert_eq!(report["campaign_act_count"], 3);
        assert_eq!(report["new_campaign_frames"], 80);
        assert_eq!(report["total_cinematic_frames"], 80);
        assert_eq!(report["new_storyboards"], 10);
        assert_eq!(report["new_reusable_props"], 48);
        assert_eq!(report["new_effect_animations"], 8);
        assert_eq!(report["new_effect_frames"], 48);
        assert_eq!(report["new_character_portraits"], 8);
        assert_eq!(report["new_campaign_cards"], 2);
        assert_eq!(report["generated_files"], 202);
        assert_eq!(report["new_frame_hashes"], 80);
        assert_eq!(report["all_generated_files_exist"], true);
        assert_eq!(report["all_frame_hashes_match"], true);
        assert_eq!(report["previous_scenes_retained"], true);
        assert_eq!(report["prop_atlas_sprite_count"], 48);
        assert_eq!(report["effect_atlas_sprite_count"], 48);

        // Timeline: 10 new sequences + 2 campaigns, sorted, notes deduped.
        let timeline = pack.read_json_rel("sprites/cinematics/timeline.json");
        let sequences = timeline["sequences"].as_array().unwrap();
        assert_eq!(sequences.len(), 16);
        for seq in sequences {
            let id = seq["id"].as_str().unwrap();
            if !is_v6_sequence(id) {
                continue;
            }
            let frames = seq["frames"].as_array().unwrap();
            assert_eq!(frames.len(), 8, "{id} frame count");
            for frame in frames {
                let path = pack.0.join(frame["file"].as_str().unwrap());
                assert!(path.is_file());
                let bytes = std::fs::read(&path).unwrap();
                let img = image::load_from_memory(&bytes).unwrap();
                assert_eq!((img.width(), img.height()), (320, 180));
                assert_eq!(
                    sha256_hex(&bytes),
                    frame["sha256"].as_str().unwrap(),
                    "{id} frame hash"
                );
            }
        }
        assert_eq!(timeline["campaigns"].as_array().unwrap().len(), 3);
        let notes = timeline["notes"].as_array().unwrap();
        let unique_notes: HashSet<&str> = notes.iter().filter_map(Value::as_str).collect();
        assert_eq!(notes.len(), unique_notes.len(), "notes deduped");
        let supporting = timeline["supporting_assets"].as_array().unwrap();
        let mut sorted_supporting: Vec<&str> =
            supporting.iter().filter_map(Value::as_str).collect();
        sorted_supporting.sort_unstable();
        let supporting_strs: Vec<&str> = supporting.iter().filter_map(Value::as_str).collect();
        assert_eq!(
            supporting_strs, sorted_supporting,
            "supporting_assets sorted"
        );
        assert_eq!(
            HashSet::<&str>::from_iter(supporting_strs.iter().copied()).len(),
            supporting.len(),
            "supporting_assets unique"
        );
        // 198 generated PNGs (202 generated files minus 2 atlas maps, the
        // timeline, and the overview preview — Python renders the overview
        // after the timeline write, so the shipped pack's timeline does not
        // list it as a supporting asset).
        assert!(supporting.len() >= 198);
        assert!(!supporting
            .iter()
            .any(|s| s.as_str() == Some("previews/v6_campaign_overview.png")));

        // Ten storyboards follow the ten scene ids.
        for (seq_id, ..) in SCENES {
            let path = pack.0.join(format!(
                "sprites/cinematics/previews/{seq_id}_storyboard.png"
            ));
            assert!(path.is_file(), "{seq_id} storyboard");
            assert_eq!(png_dims(&path), (640, 205), "{seq_id} storyboard dims");
        }

        // 48 props at 64px with atlas.
        for name in PROP_NAMES {
            let path = pack.0.join(format!("sprites/campaign_art/{name}.png"));
            assert!(path.is_file(), "{name}");
            assert_eq!(png_dims(&path), (64, 64), "{name}");
        }
        let prop_atlas = pack
            .0
            .join("sprites/campaign_art/campaign_art_atlas_v6.png");
        assert_eq!(png_dims(&prop_atlas), (512, 384));
        let prop_mapping = pack.read_json_rel("sprites/campaign_art/campaign_art_atlas_v6.json");
        assert_eq!(prop_mapping["sprites"].as_object().unwrap().len(), 48);
        assert_eq!(prop_mapping["rows"], 6);
        assert_eq!(prop_mapping["columns"], 8);

        // Effects: 8 kinds × 6 frames, each 32×32.
        for effect in EFFECTS {
            for fi in 0..6usize {
                let path = pack
                    .0
                    .join(format!("sprites/animations_v6/{effect}/frame_{fi:02}.png"));
                assert!(path.is_file(), "{effect} frame {fi}");
                assert_eq!(png_dims(&path), (32, 32));
            }
        }
        let effect_atlas = pack.0.join("atlases/v6_effect_animation.png");
        assert_eq!(png_dims(&effect_atlas), (192, 256));
        let effect_mapping = pack.read_json_rel("atlases/v6_effect_animation.json");
        assert_eq!(effect_mapping["sprites"].as_object().unwrap().len(), 48);
        assert_eq!(effect_mapping["rows"], 8);
        assert_eq!(effect_mapping["columns"], 6);

        // 8 portraits and 2 act cards.
        for name in PORTRAITS {
            let path = pack.0.join(format!("sprites/cinematic_ui/{name}.png"));
            assert!(path.is_file(), "{name}");
        }
        for (card_id, ..) in ACT_CARD_SPECS {
            let path = pack
                .0
                .join(format!("sprites/cinematics/campaign_cards/{card_id}.png"));
            assert!(path.is_file(), "{card_id}");
            assert_eq!(png_dims(&path), (320, 180), "{card_id}");
        }

        // Overview preview exists but is NOT a manifest sprite.
        let overview = pack.0.join("previews/v6_campaign_overview.png");
        assert_eq!(png_dims(&overview), (1280, 330));
        let manifest = pack.read_json_rel("manifest.json");
        let ids: Vec<&str> = manifest["sprites"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["id"].as_str().unwrap())
            .collect();
        assert!(!ids.contains(&"previews/v6_campaign_overview"));
        // Unique manifest sprite ids; every record's file exists with the
        // recorded size.
        assert_eq!(
            ids.len(),
            HashSet::<&str>::from_iter(ids.iter().copied()).len()
        );
        for entry in manifest["sprites"].as_array().unwrap() {
            let path = pack.0.join(entry["file"].as_str().unwrap());
            assert!(path.is_file(), "{}", entry["file"]);
            let dims = png_dims(&path);
            assert_eq!(json!([dims.0, dims.1]), entry["size"], "{}", entry["file"]);
            let bytes = std::fs::read(&path).unwrap();
            assert_eq!(
                sha256_hex(&bytes),
                entry["sha256_file"].as_str().unwrap(),
                "{}",
                entry["file"]
            );
        }
        // Manifest atlas records and top-level key.
        let atlases = manifest["atlases"].as_array().unwrap();
        assert_eq!(atlases.len(), 2);
        let atlas_files: HashSet<&str> = atlases
            .iter()
            .map(|a| a["file"].as_str().unwrap())
            .collect();
        assert!(atlas_files.contains("sprites/campaign_art/campaign_art_atlas_v6.png"));
        assert!(atlas_files.contains("atlases/v6_effect_animation.png"));
        assert_eq!(manifest["campaign_art_atlas_v6"]["sprite_count"], 48);
        assert_eq!(manifest["cinematics"]["campaign_art_count"], 72);
        assert_eq!(manifest["cinematics"]["new_effect_animation_count"], 8);

        // Returned contributions mirror the timeline writes.
        let timeline_contrib = output.timeline.as_ref().unwrap();
        assert_eq!(timeline_contrib.sequences.len(), 10);
        assert_eq!(timeline_contrib.campaigns.len(), 2);
        assert_eq!(timeline_contrib.notes.len(), 7);
        assert_eq!(output.atlases.len(), 2);
        // 80 scenes + 10 storyboards + 48 props + 1 prop-atlas record +
        // 48 effect frames + 8 portraits + 2 act cards (the effect atlas and
        // the overview preview are not registered in `sprites`).
        assert_eq!(output.sprites.len(), 197);
    }

    #[test]
    /// Crate-level determinism property: identical inputs produce
    /// byte-identical output. Compare two fresh runs instead of re-running
    /// into the same directory — the Python scripts are additive-in-place
    /// (e.g. `campaign_art_count` accumulates from the pre-existing manifest:
    /// `int(manifest.get("cinematics",{}).get("campaign_art_count",22)) + 50`),
    /// so same-dir re-runs are NOT byte-idempotent; that is faithful.
    fn repeated_generate_is_deterministic() {
        let mut first: Vec<(&'static str, Vec<u8>)> = Vec::new();
        let mut second: Vec<(&'static str, Vec<u8>)> = Vec::new();
        for run in 0..2 {
            let pack = TempPack::new();
            pack.setup_legacy();
            let out = PackWriter::new(&pack.0);
            generate(&out, 77).unwrap();
            let sink = if run == 0 { &mut first } else { &mut second };
            sink.push((
                "manifest",
                std::fs::read(pack.0.join("manifest.json")).unwrap(),
            ));
            sink.push((
                "timeline",
                std::fs::read(pack.0.join("sprites/cinematics/timeline.json")).unwrap(),
            ));
            sink.push((
                "frame",
                std::fs::read(
                    pack.0
                        .join("sprites/cinematics/frames/19_burned_rail/frame_000.png"),
                )
                .unwrap(),
            ));
            sink.push((
                "report",
                std::fs::read(pack.0.join("campaign_build_report_v6_0_0.json")).unwrap(),
            ));
        }
        assert_eq!(first.len(), 4);
        for ((key, lhs), (_, rhs)) in first.iter().zip(&second) {
            byte_diff(key, lhs, rhs); // no-op when identical
            assert_eq!(lhs, rhs, "{key}");
        }
    }

    #[test]
    fn production_seed_smoke() {
        // Production-config smoke: the real pack default seed also runs.
        let pack2 = TempPack::new();
        pack2.setup_legacy();
        let out2 = PackWriter::new(&pack2.0);
        generate(&out2, DEFAULT_SEED).unwrap();
        let report = pack2.read_json_rel("campaign_build_report_v6_0_0.json");
        assert_eq!(report["new_campaign_scene_count"], 10);
    }

    #[test]
    fn public_api_is_deterministic_and_correctly_sized() {
        // Repeated calls return identical canvases.
        assert_eq!(
            scene_overlay(
                campaign_v59::render_scene(77, "dawn", "border", 3),
                "railway",
                3,
                77
            ),
            scene_overlay(
                campaign_v59::render_scene(77, "dawn", "border", 3),
                "railway",
                3,
                77
            )
        );
        assert_eq!(prop_sprite("pumpjack", 77), prop_sprite("pumpjack", 77));
        assert_eq!(
            effect_frame("electric_arc", 2),
            effect_frame("electric_arc", 2)
        );
        // Frame dimensions.
        let frame = scene_overlay(
            campaign_v59::render_scene(DEFAULT_SEED, "dawn", "border", 0),
            "frontier",
            0,
            DEFAULT_SEED,
        );
        assert_eq!((frame.width(), frame.height()), (320, 180));
        let prop = prop_sprite("water_tower", DEFAULT_SEED);
        assert_eq!((prop.width(), prop.height()), (64, 64));
        let effect = effect_frame("fire_burst", 5);
        assert_eq!((effect.width(), effect.height()), (32, 32));
    }
}
