//! Port of `campaign_expansion_v6_1.py` — v6.1 briefing sequences, icons, props, personnel.
//!
//! Logic-faithful port of the CNC32 v6.2.1 asset pack's additive briefing
//! layer: eight six-frame tactical briefing/debrief sequences, eight
//! cinematic campaign cards, 16 transparent briefing props, 32 tactical UI
//! icons, eight personnel portraits, two art atlases, an overview sheet and
//! the `campaign_build_report_v6_1_0.json` build report. Structure, ids,
//! file paths, seed derivation, draw order and draw counts follow the
//! Python source; palette/colour constants are the script's own, verbatim.
//!
//! ## Deviations accepted by the fidelity contract (do not "fix")
//!
//! * [`Rng`](crate::prng::Rng) (SplitMix64) instead of Python MT19937: same
//!   draw counts/order/ranges, different values.
//! * [`Canvas`](crate::tileset::canvas::Canvas) rasterizer instead of
//!   Pillow. Multi-point PIL `d.line(points, ...)` polylines are drawn as
//!   consecutive two-point segments (the call sequence per segment is
//!   preserved).
//! * The 5x7 bitmap [`draw_text`] instead of Pillow's default bitmap font.
//! * JSON object keys are alphabetical (serde_json maps) versus the
//!   Python's insertion order.
//! * PIL `rounded_rectangle(fill=..., outline=...)` is reproduced as a
//!   [`Canvas::rounded_rect`] fill pass followed by a
//!   [`Canvas::rounded_rect_outline`] outline pass, and
//!   `d.pieslice(..., outline=..., width=...)` (personnel helmets) as a
//!   fill pieslice plus a rim arc with two radius strokes — close to, not
//!   pixel-identical with, Pillow's outline rendering.
//! * Python quirk preserved in `render_briefing`'s marker loop: the source
//!   compares an integer `x` against the *tuples* `route[-1]`/`route[1]`,
//!   which is always `False` in Python, so route markers are drawn without
//!   frame offset and never switch to the red fallback colour.
//! * The Python script mutates an existing `manifest.json` /
//!   `sprites/cinematics/timeline.json` in place; here the builder merges
//!   [`LayerOutput`] contributions in layer order. `manifest.json` and
//!   `timeline.json` therefore have no pre-existing-file check and are not
//!   written by this layer. The manifest top-level `cinematics` update is
//!   contributed as the layer-local keys only (`timeline`,
//!   `new_briefing_scene_count`, `new_briefing_frame_count`); the builder
//!   must merge it over the previous layers' `cinematics` object and
//!   recompute `sequence_count` / `frame_count` / `campaign_scene_count`
//!   from the merged timeline (exact Python semantics). Cross-layer note
//!   for the integration mode: the Python sorts the merged timeline
//!   `sequences` by `(0 if scene_type != "briefing" else 1, id)` —
//!   non-briefing sequences first (by id), then briefings (by id).
//! * The report's `previous_sequence_count` /
//!   `previous_sequences_retained` / `total_sequence_count` /
//!   `total_timeline_frames` counts are computed from a
//!   `sprites/cinematics/timeline.json` already present in the pack
//!   directory when `generate` runs (Python behaviour); on a fresh pack
//!   with no timeline yet they fall back to this layer's own contribution
//!   (0 previous, 8 total sequences, 48 total frames).
//! * The build report is contributed via [`LayerOutput::reports`] (the
//!   builder writes it); all PNG/JSON artefacts below are written directly.

use serde_json::{json, Map, Value};

use crate::prng::{Rng, Seed};
use crate::tileset::canvas::Canvas;
use crate::tileset::font::draw_text;
use crate::tileset::pack::PackError;
use crate::tileset::{Rgba, TileRngExt};

use super::{compose_atlas, LayerOutput, PackWriter, SpriteSpec, TimelineContribution};

/// Python `VERSION` / report `generator_version`.
pub const VERSION: &str = "6.1.0";

/// Python `generate(..., seed=61_021)` default seed (independent of the
/// pack's `--seed`, exactly as the script was run to build the real pack).
pub const DEFAULT_SEED: u64 = 61_021;

/// Python `W` / `H`: briefing frame and campaign-card canvas size.
const W: u32 = 320;
const H: u32 = 180;

/// The Python script's `generated_by` provenance string.
const GENERATED_BY: &str = "campaign_expansion_v6_1.py";

/// One `BRIEFINGS` row: `(scene_id, title, campaign_act, motif, accent, captions)`.
type BriefingSpec = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    [u8; 3],
    [&'static str; 6],
);

/// One `render_card` palette row: `(motif, sky, bright, ground)`.
type CardPalette = (&'static str, [u8; 3], [u8; 3], [u8; 3]);

/// Python `BRIEFINGS`.
const BRIEFINGS: &[BriefingSpec] = &[
    (
        "briefing_01_command_rundown",
        "Command Rundown",
        "Briefing — Act I",
        "command",
        [90, 190, 165],
        [
            "Review the latest frontline reports",
            "Recon confirms movement near the border",
            "Mark the safest approach corridor",
            "Assign armor to the exposed western flank",
            "Keep engineers in reserve for repairs",
            "Briefing complete — move to staging",
        ],
    ),
    (
        "briefing_02_rail_interdiction",
        "Rail Interdiction",
        "Briefing — Act IV",
        "rail",
        [229, 173, 84],
        [
            "Enemy supply traffic uses the eastern rail",
            "A disabled train blocks the north spur",
            "Recon marks two guarded crossings",
            "Cut the rail without damaging the bridge",
            "Secure the switching yard after contact",
            "Mission plan approved — rail corridor",
        ],
    ),
    (
        "briefing_03_floodplain_plan",
        "Floodplain Crossing",
        "Briefing — Act IV",
        "river",
        [90, 190, 208],
        [
            "Floodwater has closed the old ford",
            "Engineers identify a shallow approach",
            "Pontoon teams will build under smoke",
            "Armor must cross only after the test run",
            "Rescue boats keep the eastern bank clear",
            "Mission plan approved — river crossing",
        ],
    ),
    (
        "briefing_04_refinery_targeting",
        "Refinery Target Package",
        "Briefing — Act IV",
        "refinery",
        [236, 135, 72],
        [
            "The refinery feeds the enemy fuel reserve",
            "Three tanks and a pump station are confirmed",
            "Avoid fire near the storage manifold",
            "Approach through the western service road",
            "Capture the pump controls intact",
            "Target package locked — preserve the site",
        ],
    ),
    (
        "briefing_05_northwatch_signal",
        "Northwatch Signal Loss",
        "Briefing — Act IV",
        "snow",
        [130, 207, 190],
        [
            "Northwatch relay missed two check-ins",
            "Weather limits the flight window",
            "A patrol will follow the ridge line",
            "Restore backup power before the radar",
            "Identify the source of the false signal",
            "Mission plan approved — restore the relay",
        ],
    ),
    (
        "briefing_06_skyshield_intercept",
        "Skyshield Intercept",
        "Briefing — Act V",
        "airdefense",
        [235, 102, 85],
        [
            "Radar detects a low approach in storm cover",
            "Two possible flight corridors are active",
            "Move the battery outside the convoy lane",
            "Wait for a clean lock before launch",
            "Reserve a second shot for decoys",
            "Intercept authorized — protect the column",
        ],
    ),
    (
        "briefing_07_foundry_schematic",
        "Foundry Schematic",
        "Briefing — Act V",
        "foundry",
        [232, 160, 84],
        [
            "The foundry is the enemy's last vehicle works",
            "Thermal scans locate the assembly line",
            "Power and control rooms are separated",
            "Cut the control feed before entering the hall",
            "Keep the furnace infrastructure intact",
            "Mission plan approved — capture the foundry",
        ],
    ),
    (
        "briefing_08_homecoming_relief",
        "Homecoming Relief Plan",
        "Debrief — Act V",
        "relief",
        [124, 207, 146],
        [
            "The town has reopened its main road",
            "Medical and food stations need protection",
            "Repair crews can restore two power links",
            "Keep heavy vehicles outside the shelter zone",
            "Escort residents to the eastern square",
            "Debrief complete — protect the rebuild",
        ],
    ),
];

/// Python `PROP_NAMES`.
const PROP_NAMES: &[&str] = &[
    "command_table",
    "wall_radar",
    "projector_console",
    "field_map_board",
    "intel_desk",
    "mission_folder",
    "antenna_console",
    "satellite_terminal",
    "tactical_pointer",
    "radar_screen",
    "signal_flag",
    "briefing_lamp",
    "target_photo_stack",
    "comms_headset",
    "threat_marker_red",
    "waypoint_marker_blue",
];

/// Python `ICON_NAMES`.
const ICON_NAMES: &[&str] = &[
    "objective",
    "recon",
    "defend",
    "capture",
    "destroy",
    "extract",
    "supply",
    "escort",
    "radar",
    "airstrike",
    "armor",
    "infantry",
    "engineer",
    "medical",
    "comms",
    "threat",
    "fuel",
    "bridge",
    "rail",
    "convoy",
    "factory",
    "power",
    "water",
    "rescue",
    "civilians",
    "route",
    "checkpoint",
    "hazard",
    "clear",
    "locked",
    "completed",
    "priority",
];

/// Python `PERSONNEL`: `(name, role, skin, uniform)`.
const PERSONNEL: &[(&str, &str, [u8; 3], [u8; 3])] = &[
    (
        "commander_anya",
        "commander",
        [171, 116, 72],
        [95, 131, 105],
    ),
    (
        "chief_engineer_rook",
        "engineer",
        [192, 145, 94],
        [108, 125, 91],
    ),
    (
        "intel_officer_vesper",
        "intelligence",
        [183, 138, 102],
        [83, 111, 124],
    ),
    (
        "radar_operator_lee",
        "radar",
        [204, 160, 126],
        [75, 118, 124],
    ),
    ("pilot_marek", "pilot", [185, 128, 89], [99, 109, 119]),
    ("medic_sato", "medic", [210, 168, 136], [110, 128, 109]),
    ("scout_arden", "scout", [153, 111, 79], [112, 125, 73]),
    (
        "relief_coordinator_nia",
        "relief",
        [194, 145, 116],
        [121, 151, 108],
    ),
];

// ---------------------------------------------------------------------------
// Shared draw helpers (Python `clamp`, `line_dashes`, `draw_marker`).
// ---------------------------------------------------------------------------

/// Opaque RGBA from the script's own RGB triples.
fn rgba3(c: [u8; 3]) -> Rgba {
    [c[0], c[1], c[2], 255]
}

/// Python `clamp(c, offset)`: per-channel clamped offset.
fn clamp3(c: [u8; 3], offset: i32) -> [u8; 3] {
    c.map(|x| (x as i32 + offset).clamp(0, 255) as u8)
}

/// Python `round()` (round-half-even) on a value that is exactly
/// representable to a half fraction; used by [`line_dashes`].
fn py_round(v: f64) -> i32 {
    let a = v.abs();
    let floor = a.floor();
    let half = a - floor;
    let n = if half == 0.5 {
        // round-half-even: pick the even neighbor on exact .5
        if (floor as i64) % 2 == 0 {
            floor
        } else {
            floor + 1.0
        }
    } else if half > 0.5 {
        floor + 1.0
    } else {
        floor
    };
    (if v < 0.0 { -n } else { n }) as i32
}

/// Multi-point PIL `d.line(points, fill, width)`: consecutive segments.
fn polyline(c: &mut Canvas, points: &[(i32, i32)], color: Rgba, width: u32) {
    for pair in points.windows(2) {
        c.line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, color, width);
    }
}

/// Python `line_dashes`: rasterized segmented route with stable spacing.
fn line_dashes(c: &mut Canvas, points: &[(i32, i32)], color: Rgba, phase: i32, dash: i32) {
    for pair in points.windows(2) {
        let (x1, y1) = pair[0];
        let (x2, y2) = pair[1];
        let length = (x2 - x1).abs().max((y2 - y1).abs()).max(1);
        // range(-phase % (dash*2), length, dash*2) — Python `%` is non-negative.
        let mut start = (-phase).rem_euclid(dash * 2);
        while start < length {
            let end = length.min(start + dash);
            if end > 0 {
                let ax =
                    py_round(x1 as f64 + (x2 - x1) as f64 * start.max(0) as f64 / length as f64);
                let ay =
                    py_round(y1 as f64 + (y2 - y1) as f64 * start.max(0) as f64 / length as f64);
                let bx = py_round(x1 as f64 + (x2 - x1) as f64 * end as f64 / length as f64);
                let by = py_round(y1 as f64 + (y2 - y1) as f64 * end as f64 / length as f64);
                c.line(ax, ay, bx, by, color, 2);
            }
            start += dash * 2;
        }
    }
}

/// Python `draw_marker`: diamond objective marker.
fn draw_marker(c: &mut Canvas, x: i32, y: i32, color: Rgba, radius: i32) {
    c.polygon(
        &[
            (x, y - radius - 2),
            (x + radius + 2, y),
            (x, y + radius + 2),
            (x - radius - 2, y),
        ],
        Some([15, 22, 23, 255]),
        Some(color),
    );
    c.rect(x - 1, y - 1, x + 1, y + 1, Some(color), None, 1);
}

/// Pieslice outline approximation (see module deviations): rim arc plus the
/// two straight radius strokes from the center.
#[allow(clippy::too_many_arguments)]
fn pieslice_outlined(
    c: &mut Canvas,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    start_deg: f64,
    end_deg: f64,
    color: Rgba,
    width: u32,
) {
    c.arc(x0, y0, x1, y1, start_deg, end_deg, color, width);
    let cx = (x0 + x1) as f64 / 2.0;
    let cy = (y0 + y1) as f64 / 2.0;
    let rx = (x1 - x0) as f64 / 2.0;
    let ry = (y1 - y0) as f64 / 2.0;
    for angle in [start_deg, end_deg] {
        let rad = angle.to_radians();
        let ex = (cx + rx * rad.cos()) as i32;
        let ey = (cy + ry * rad.sin()) as i32;
        c.line(cx as i32, cy as i32, ex, ey, color, width);
    }
}

/// Per-motif route polyline constants (Python `route_sets`), scripted verbatim.
fn route_for(motif: &str) -> &'static [(i32, i32)] {
    match motif {
        "river" => &[
            (30, 113),
            (65, 104),
            (89, 88),
            (119, 88),
            (155, 117),
            (185, 124),
        ],
        "rail" => &[
            (29, 125),
            (61, 110),
            (89, 98),
            (121, 83),
            (155, 72),
            (187, 55),
        ],
        "refinery" => &[
            (26, 128),
            (67, 112),
            (91, 94),
            (122, 104),
            (153, 80),
            (190, 88),
        ],
        "snow" => &[
            (31, 124),
            (65, 113),
            (87, 92),
            (121, 100),
            (149, 78),
            (182, 68),
        ],
        "airdefense" => &[
            (28, 122),
            (58, 111),
            (85, 87),
            (116, 76),
            (146, 101),
            (180, 91),
        ],
        "foundry" => &[
            (26, 126),
            (58, 109),
            (89, 116),
            (112, 91),
            (146, 94),
            (185, 72),
        ],
        "relief" => &[(28, 119), (66, 119), (105, 90), (141, 90), (181, 118)],
        _ => &[
            (28, 121),
            (58, 98),
            (82, 91),
            (116, 70),
            (150, 82),
            (183, 56),
        ],
    }
}

/// Right-panel stat rows per motif (Python `stat_lines`).
fn stat_lines_for(motif: &str) -> [&'static str; 3] {
    match motif {
        "river" => ["WATER LEVEL", "HIGH CURRENT", "PONTOON KIT"],
        "rail" => ["SUPPLY TRAFFIC", "GUARDED SPUR", "DEMOLITION"],
        "refinery" => ["FUEL NODE", "FLAMMABLE", "CAPTURE INTACT"],
        "snow" => ["SIGNAL LOSS", "WHITEOUT", "BACKUP POWER"],
        "airdefense" => ["AIR CONTACT", "LOW APPROACH", "HOLD FIRE"],
        "foundry" => ["PRODUCTION", "CONTROL ROOM", "MIN. DAMAGE"],
        "relief" => ["CIVILIANS", "AID CORRIDOR", "NO HEAVY ARMOR"],
        _ => ["FRONT STATUS", "CONTACT LIKELY", "RESERVE READY"],
    }
}

// ---------------------------------------------------------------------------
// render_briefing (Python lines 136–261).
// ---------------------------------------------------------------------------

/// Render one 320x180 briefing frame. Python seed expression:
/// `random.Random(seed + spec_index*997 + frame*67)`.
fn render_briefing(spec_index: usize, frame: i32, seed: u64) -> Canvas {
    let (_scene_id, title, _act, motif, accent, _captions) = BRIEFINGS[spec_index];
    let mut rng = Rng::new(Seed::new(
        seed + (spec_index as u64) * 997 + (frame as i64 as u64) * 67,
    ));
    let mut c = Canvas::new_filled(W, H, [9, 15, 18, 255]);

    // CRT command-room backdrop and framing.
    for y in 0..H as i32 {
        let shade = (2.0 * ((y as f64 + (frame * 3) as f64) / 14.0).sin()) as i32;
        c.line(0, y, W as i32, y, rgba3(clamp3([10, 16, 19], shade)), 1);
    }
    c.rect(0, 0, 319, 23, Some([16, 26, 29, 255]), None, 1);
    c.rect(0, 22, 319, 24, Some(rgba3(accent)), None, 1);
    draw_text(
        &mut c,
        8,
        7,
        &format!(
            "FIELD INTELLIGENCE  //  {}",
            take_chars(&title.to_uppercase(), 25)
        ),
        [226, 232, 213, 255],
    );
    draw_text(
        &mut c,
        267,
        8,
        &format!("B{:02}", spec_index + 1),
        rgba3(accent),
    );

    // Map surface.
    let (mx1, my1, mx2, my2) = (9, 30, 211, 151);
    c.rect(
        mx1,
        my1,
        mx2,
        my2,
        Some([20, 39, 39, 255]),
        Some([69, 104, 97, 255]),
        2,
    );
    for x in (mx1 + 8..mx2).step_by(12) {
        c.line(x, my1 + 1, x, my2 - 1, [29, 56, 52, 255], 1);
    }
    for y in (my1 + 8..my2).step_by(12) {
        c.line(mx1 + 1, y, mx2 - 1, y, [29, 56, 52, 255], 1);
    }

    // Topographic contour fragments and terrain variation.
    for _ in 0..8 {
        let cx = rng.rand_i32(mx1 + 12, mx2 - 12);
        let cy = rng.rand_i32(my1 + 12, my2 - 12);
        let rx = rng.rand_i32(8, 28);
        let ry = rng.rand_i32(6, 18);
        let arc_start = rng.rand_i32(0, 180) as f64;
        let arc_end = rng.rand_i32(190, 360) as f64;
        c.arc(
            cx - rx,
            cy - ry,
            cx + rx,
            cy + ry,
            arc_start,
            arc_end,
            [37, 72, 60, 255],
            1,
        );
    }
    match motif {
        "river" => {
            let river_pts = [
                (23, 36),
                (54, 56),
                (83, 68),
                (106, 94),
                (132, 108),
                (172, 139),
                (199, 146),
            ];
            polyline(&mut c, &river_pts, [45, 104, 128, 255], 11);
            polyline(&mut c, &river_pts, [75, 147, 160, 255], 2);
            for i in 0..8 {
                let x = 35 + i * 19;
                let y = 50 + (i * 13) % 83;
                c.line(x, y, x + 7, y - 2, [116, 177, 180, 255], 1);
            }
        }
        "rail" => {
            c.line(16, 132, 202, 49, [80, 83, 73, 255], 7);
            c.line(16, 129, 202, 46, [184, 179, 149, 255], 2);
            c.line(18, 136, 204, 53, [184, 179, 149, 255], 2);
            for t in (0..190).step_by(12) {
                let x = 17 + t;
                let y = 133 - (t as f64 * 0.44) as i32;
                c.line(x, y - 2, x + 2, y + 5, [108, 93, 68, 255], 2);
            }
        }
        "refinery" => {
            for &(x, y, r) in &[(52, 59, 11), (91, 48, 9), (157, 72, 13), (178, 112, 8)] {
                c.rect(
                    x - r,
                    y,
                    x + r,
                    y + 22,
                    Some([81, 94, 75, 255]),
                    Some([148, 163, 126, 255]),
                    1,
                );
                c.ellipse(
                    x - r,
                    y - 3,
                    x + r,
                    y + 4,
                    Some([120, 133, 95, 255]),
                    Some([184, 177, 126, 255]),
                    1,
                );
            }
            polyline(
                &mut c,
                &[(56, 84), (106, 84), (106, 99), (156, 99)],
                [180, 114, 68, 255],
                3,
            );
        }
        "snow" => {
            for i in 0..6 {
                let x = 16 + i * 31;
                c.polygon(
                    &[(x, 95), (x + 14, 65 - (i % 3) * 5), (x + 27, 95)],
                    Some([79, 107, 109, 255]),
                    Some([138, 168, 165, 255]),
                );
            }
            c.rect(
                151,
                57,
                180,
                90,
                Some([69, 91, 91, 255]),
                Some([142, 171, 157, 255]),
                1,
            );
            c.line(166, 58, 166, 39, [182, 209, 193, 255], 2);
            c.arc(
                151,
                36,
                181,
                63,
                (185 + frame * 4) as f64,
                (325 + frame * 4) as f64,
                [116, 208, 172, 255],
                2,
            );
        }
        "airdefense" => {
            for &(x, y) in &[(48, 92), (121, 50), (171, 113)] {
                c.line(x, y, x + 11, y - 25, [170, 178, 154, 255], 3);
                c.line(x + 4, y - 9, x - 3, y - 7, [170, 178, 154, 255], 2);
            }
            c.arc(49, 43, 183, 128, 190.0, 342.0, [104, 146, 119, 255], 1);
        }
        "foundry" => {
            for &(x, h) in &[
                (31, 22),
                (61, 38),
                (89, 28),
                (128, 47),
                (157, 32),
                (184, 25),
            ] {
                c.rect(
                    x,
                    111 - h,
                    x + 15,
                    111,
                    Some([78, 91, 80, 255]),
                    Some([137, 151, 125, 255]),
                    1,
                );
                c.rect(
                    x + 3,
                    108 - h,
                    x + 12,
                    113 - h,
                    Some([201, 128, 66, 255]),
                    None,
                    1,
                );
            }
            c.line(27, 116, 190, 116, [126, 103, 75, 255], 3);
        }
        "relief" => {
            c.line(16, 71, 197, 71, [157, 160, 113, 255], 3);
            c.line(105, 39, 105, 143, [157, 160, 113, 255], 3);
            for &(x, y) in &[(43, 50), (70, 103), (143, 52), (174, 98)] {
                c.rect(
                    x - 7,
                    y - 5,
                    x + 7,
                    y + 5,
                    Some([86, 116, 83, 255]),
                    Some([176, 194, 141, 255]),
                    1,
                );
                c.rect(
                    x - 3,
                    y - 2,
                    x + 3,
                    y + 2,
                    Some([205, 205, 153, 255]),
                    None,
                    1,
                );
            }
        }
        _ => {
            // command hub with sector rings
            for &r in &[19, 38, 56] {
                c.ellipse(
                    107 - r,
                    90 - r,
                    107 + r,
                    90 + r,
                    None,
                    Some([39, 83, 73, 255]),
                    1,
                );
            }
            c.line(22, 122, 184, 54, [58, 123, 89, 255], 3);
            c.line(39, 48, 164, 137, [57, 91, 81, 255], 2);
        }
    }

    // A moving sweep and objective route lend the scene subtle life.
    let sweep_x = mx1 + 20 + (frame * 19) % 168;
    c.line(sweep_x, my1 + 6, sweep_x, my2 - 6, [50, 91, 72, 255], 1);
    let route = route_for(motif);
    line_dashes(&mut c, route, rgba3(accent), frame * 2, 5);
    // Python quirk preserved: `x` is an int and `route[-1]`/`route[1]` are
    // tuples, so the offset test is always False and the accent-vs-red test
    // always True — markers render unshifted and always in the accent colour.
    for &(x, y) in &[route[1], route[3], route[route.len() - 1]] {
        draw_marker(&mut c, x, y, rgba3(accent), 3);
    }

    // Right briefing telemetry panel.
    c.rect(
        218,
        30,
        310,
        151,
        Some([18, 28, 31, 255]),
        Some([65, 93, 89, 255]),
        2,
    );
    draw_text(&mut c, 225, 37, "MISSION DATA", rgba3(accent));
    c.line(225, 49, 303, 49, [53, 74, 73, 255], 1);
    let stat_lines = stat_lines_for(motif);
    for (i, txt) in stat_lines.iter().enumerate() {
        let yy = 56 + i as i32 * 22;
        let color = if i < 2 {
            [180, 196, 179, 255]
        } else {
            rgba3(accent)
        };
        draw_text(&mut c, 225, yy, txt, color);
        c.rect(225, yy + 10, 301, yy + 13, Some([30, 45, 44, 255]), None, 1);
        let level = 39 + ((frame * 9 + i as i32 * 23 + spec_index as i32 * 11) % 55);
        let fill = if i != 1 {
            rgba3(accent)
        } else {
            [193, 111, 73, 255]
        };
        c.rect(225, yy + 10, 225 + level, yy + 13, Some(fill), None, 1);
    }

    // Animated radio waveform and frame index on the lower right.
    draw_text(&mut c, 225, 128, "RADIO", [102, 131, 124, 255]);
    for i in 0..18 {
        let x = 225 + i * 4;
        let h = 2 + ((i * 7 + frame * 5 + spec_index as i32 * 3) % 10);
        let color = if i % 4 != 0 {
            rgba3(accent)
        } else {
            [216, 202, 143, 255]
        };
        c.line(x, 146 - h, x, 146, color, 2);
    }
    draw_text(
        &mut c,
        286,
        140,
        &format!("{}/6", frame + 1),
        [216, 223, 205, 255],
    );

    // Foreground staff silhouettes to convey an in-room briefing rather than a menu.
    for &(bx, by, jacket) in &[
        (74, 164, [19, 27, 29]),
        (108, 162, [23, 32, 32]),
        (143, 165, [18, 25, 28]),
    ] {
        c.ellipse(
            bx - 6,
            by - 14,
            bx + 6,
            by - 2,
            Some([8, 12, 13, 255]),
            Some([49, 65, 61, 255]),
            1,
        );
        c.polygon(
            &[
                (bx - 11, by - 2),
                (bx + 11, by - 2),
                (bx + 17, 180),
                (bx - 16, 180),
            ],
            Some(rgba3(jacket)),
            None,
        );
    }
    c.line(15, 157, 210, 157, [87, 108, 93, 255], 2);
    c.line(19, 160, 205, 160, [30, 43, 41, 255], 2);
    c.rect(0, 174, 319, 179, Some([8, 12, 14, 255]), None, 1);
    c.rect(7, 176, 42, 177, Some(rgba3(accent)), None, 1);
    draw_text(
        &mut c,
        49,
        174,
        "SECURE CHANNEL  //  AUTHORIZED PERSONNEL ONLY",
        [100, 129, 119, 255],
    );
    c
}

// ---------------------------------------------------------------------------
// render_card (Python lines 264–352).
// ---------------------------------------------------------------------------

/// Render one 320x180 cinematic campaign card. Python seed expression:
/// `random.Random(seed + spec_index*431)` with `seed = SEED + 9100 + si`.
fn render_card(spec_index: usize, seed: u64) -> Canvas {
    let (_scene_id, title, act, motif, _accent, _captions) = BRIEFINGS[spec_index];
    let mut rng = Rng::new(Seed::new(seed + (spec_index as u64) * 431));
    let palettes: &[CardPalette] = &[
        ("command", [66, 93, 80], [173, 166, 115], [32, 54, 47]),
        ("rail", [98, 88, 66], [209, 155, 80], [46, 46, 37]),
        ("river", [60, 102, 116], [145, 190, 180], [32, 67, 77]),
        ("refinery", [132, 82, 52], [234, 161, 70], [56, 43, 37]),
        ("snow", [93, 126, 135], [199, 218, 209], [52, 77, 86]),
        ("airdefense", [53, 73, 91], [184, 111, 83], [29, 40, 54]),
        ("foundry", [111, 82, 58], [242, 143, 67], [48, 43, 36]),
        ("relief", [127, 146, 101], [221, 196, 126], [60, 84, 62]),
    ];
    let (_motif, sky, bright, ground) = palettes
        .iter()
        .find(|(id, ..)| *id == motif)
        .copied()
        .expect("known motif");

    let mut c = Canvas::new_filled(320, 180, rgba3(sky));
    for y in 0..120 {
        let t = y as f64 / 120.0;
        let sky_d = clamp3(sky, -30);
        let col: [u8; 3] =
            std::array::from_fn(|k| (sky[k] as f64 * (1.0 - t) + sky_d[k] as f64 * t) as i64 as u8);
        c.line(0, y, 319, y, rgba3(col), 1);
    }
    // Horizon, ridge layers, and wind/weather marks.
    for i in 0..7 {
        let x = i * 54 - 20;
        let peak = rng.rand_i32(45, 92);
        let col = clamp3(ground, (i % 3) * 5);
        c.polygon(
            &[(x, 119), (x + 31, peak), (x + 77, 119)],
            Some(rgba3(col)),
            None,
        );
    }
    c.rect(0, 117, 319, 179, Some(rgba3(ground)), None, 1);

    // Roads / objective corridor.
    match motif {
        "river" => {
            c.polygon(
                &[
                    (0, 148),
                    (45, 134),
                    (89, 151),
                    (137, 128),
                    (181, 146),
                    (231, 132),
                    (320, 153),
                    (320, 180),
                    (0, 180),
                ],
                Some([35, 74, 84, 255]),
                None,
            );
            polyline(
                &mut c,
                &[
                    (0, 148),
                    (45, 134),
                    (89, 151),
                    (137, 128),
                    (181, 146),
                    (231, 132),
                    (320, 153),
                ],
                [107, 163, 164, 255],
                3,
            );
            for x in (80..242).step_by(24) {
                c.line(x, 145, x + 8, 140, [179, 200, 172, 255], 2);
            }
        }
        "rail" | "relief" | "command" => {
            c.polygon(
                &[(104, 118), (175, 118), (252, 179), (32, 179)],
                Some(rgba3(clamp3(ground, 18))),
                None,
            );
            c.line(102, 119, 47, 179, [190, 177, 136, 255], 3);
            c.line(175, 119, 265, 179, [190, 177, 136, 255], 3);
            for y in (132..177).step_by(10) {
                let xx = 89 - (y - 132);
                c.line(xx, y, xx + 96 + (y - 132), y, [117, 104, 79, 255], 2);
            }
        }
        _ => {
            c.line(0, 153, 320, 130, rgba3(clamp3(ground, 22)), 8);
            c.line(0, 151, 320, 128, [167, 150, 109, 255], 1);
        }
    }

    // Distinctive motif silhouettes.
    match motif {
        "refinery" => {
            for &(x, y, r) in &[(44, 98, 18), (86, 88, 13), (245, 96, 23)] {
                c.rect(
                    x - r,
                    y,
                    x + r,
                    y + 35,
                    Some([75, 75, 62, 255]),
                    Some([203, 156, 94, 255]),
                    2,
                );
                c.ellipse(
                    x - r,
                    y - 5,
                    x + r,
                    y + 5,
                    Some([140, 128, 92, 255]),
                    Some([213, 177, 112, 255]),
                    1,
                );
            }
            for &(x, h) in &[(139, 35), (164, 47), (188, 28)] {
                c.rect(
                    x,
                    116 - h,
                    x + 15,
                    116,
                    Some([62, 70, 64, 255]),
                    Some([165, 138, 101, 255]),
                    2,
                );
                c.ellipse(
                    x + 4,
                    108 - h,
                    x + 13,
                    109 - h,
                    Some(rgba3(bright)),
                    None,
                    1,
                );
            }
        }
        "snow" => {
            for &(x, h) in &[(35, 26), (83, 42), (149, 30), (218, 48), (276, 28)] {
                c.polygon(
                    &[(x - 23, 118), (x, 118 - h), (x + 22, 118)],
                    Some([166, 192, 191, 255]),
                    Some([220, 226, 210, 255]),
                );
            }
            c.rect(
                217,
                79,
                241,
                116,
                Some([61, 83, 86, 255]),
                Some([176, 201, 185, 255]),
                2,
            );
            c.line(229, 79, 229, 52, [207, 221, 197, 255], 3);
            c.arc(
                211,
                51,
                247,
                82,
                190.0,
                345.0,
                rgba3(accent_of(spec_index)),
                2,
            );
        }
        "airdefense" => {
            c.polygon(
                &[
                    (0, 130),
                    (62, 114),
                    (104, 131),
                    (161, 113),
                    (226, 130),
                    (320, 109),
                    (320, 180),
                    (0, 180),
                ],
                Some([41, 51, 60, 255]),
                None,
            );
            c.line(102, 130, 121, 75, [177, 179, 161, 255], 5);
            c.polygon(
                &[(113, 83), (122, 64), (129, 86)],
                Some([221, 223, 201, 255]),
                None,
            );
            c.line(123, 66, 140, 50, rgba3(accent_of(spec_index)), 2);
            for &(x, y) in &[(194, 95), (222, 75), (250, 91)] {
                c.ellipse(
                    x - 3,
                    y - 3,
                    x + 3,
                    y + 3,
                    Some([235, 125, 91, 255]),
                    None,
                    1,
                );
                c.line(x - 7, y - 7, x + 7, y + 7, [235, 125, 91, 255], 1);
            }
        }
        "foundry" => {
            for &(x, h) in &[
                (30, 43),
                (61, 25),
                (93, 52),
                (131, 36),
                (175, 46),
                (216, 28),
                (261, 41),
            ] {
                c.rect(
                    x,
                    118 - h,
                    x + 20,
                    118,
                    Some([65, 68, 59, 255]),
                    Some([165, 131, 85, 255]),
                    2,
                );
                c.rect(
                    x + 5,
                    116 - h,
                    x + 15,
                    119 - h,
                    Some(rgba3(bright)),
                    None,
                    1,
                );
            }
            c.line(30, 112, 286, 112, [203, 119, 60, 255], 3);
        }
        "relief" => {
            for &(x, y) in &[(32, 111), (82, 95), (234, 104), (275, 119)] {
                c.rect(
                    x - 12,
                    y - 10,
                    x + 12,
                    y + 14,
                    Some([89, 121, 83, 255]),
                    Some([212, 190, 126, 255]),
                    2,
                );
                c.polygon(
                    &[(x - 15, y - 10), (x, y - 22), (x + 15, y - 10)],
                    Some([166, 126, 82, 255]),
                    None,
                );
            }
            c.rect(
                122,
                125,
                196,
                145,
                Some([66, 82, 68, 255]),
                Some([212, 190, 126, 255]),
                2,
            );
            c.rect(137, 130, 181, 141, Some([185, 186, 139, 255]), None, 1);
        }
        _ => {
            // Tank convoy / command silhouettes anchor the common RTS visual language.
            for &(x, y) in &[(52, 125), (176, 118), (236, 128)] {
                c.rect(
                    x,
                    y,
                    x + 43,
                    y + 20,
                    Some([35, 48, 41, 255]),
                    Some([160, 170, 132, 255]),
                    2,
                );
                c.rect(
                    x + 8,
                    y - 8,
                    x + 27,
                    y + 2,
                    Some([65, 91, 62, 255]),
                    Some([173, 181, 137, 255]),
                    1,
                );
                c.line(x + 19, y - 3, x + 44, y - 10, [188, 190, 149, 255], 3);
                for &wx in &[x + 6, x + 31] {
                    c.ellipse(
                        wx,
                        y + 16,
                        wx + 10,
                        y + 25,
                        Some([12, 17, 17, 255]),
                        Some([139, 147, 116, 255]),
                        1,
                    );
                }
            }
        }
    }

    // Weather, lights and frame-edge details.
    for i in 0..28 {
        let x = rng.rand_i32(0, 319);
        let y = rng.rand_i32(24, 116);
        if motif == "snow" {
            c.line(x, y, x - 3, y + 5, [198, 220, 212, 255], 1);
        } else if motif == "airdefense" && i % 3 == 0 {
            c.line(x, y, x + 4, y - 4, rgba3(accent_of(spec_index)), 1);
        } else if motif == "relief" && i % 4 == 0 {
            c.rect(x, y, x + 2, y + 2, Some([201, 185, 120, 255]), None, 1);
        }
    }

    c.rect(0, 0, 319, 22, Some([10, 15, 18, 255]), None, 1);
    c.rect(0, 21, 319, 23, Some(rgba3(accent_of(spec_index))), None, 1);
    draw_text(
        &mut c,
        9,
        7,
        "OPERATION FILE  //  TACTICAL BRIEFING",
        [222, 226, 210, 255],
    );
    c.rect(0, 147, 319, 179, Some([8, 13, 15, 255]), None, 1);
    c.rect(9, 153, 13, 172, Some(rgba3(accent_of(spec_index))), None, 1);
    draw_text(
        &mut c,
        20,
        151,
        &take_chars(title, 34),
        [236, 235, 214, 255],
    );
    draw_text(
        &mut c,
        20,
        166,
        &take_chars(&act.to_uppercase(), 42),
        [144, 163, 149, 255],
    );
    draw_text(
        &mut c,
        268,
        166,
        &format!("{:02}/08", spec_index + 1),
        rgba3(accent_of(spec_index)),
    );
    c
}

/// The briefing spec's accent colour at `spec_index` (script's `BRIEFINGS`).
fn accent_of(spec_index: usize) -> [u8; 3] {
    BRIEFINGS[spec_index].4
}

/// Python `s[:n]`: first `n` characters.
fn take_chars(text: &str, n: usize) -> String {
    text.chars().take(n).collect()
}

// ---------------------------------------------------------------------------
// render_prop (Python lines 355–414).
// ---------------------------------------------------------------------------

/// Render one 64x64 transparent briefing prop, seeded per prop.
fn render_prop(name: &str, seed: u64) -> Canvas {
    let mut rng = Rng::new(Seed::new(seed));
    let mut c = Canvas::transparent(64, 64);
    let dark: Rgba = [17, 25, 26, 255];
    let steel: Rgba = [105, 127, 121, 255];
    let hi: Rgba = [176, 189, 159, 255];
    let accent: Rgba = if !name.contains("threat") {
        [229, 174, 76, 255]
    } else {
        [224, 75, 59, 255]
    };

    // Contact shadow plus a visual-specific silhouette; all art is transparent RGBA.
    c.ellipse(7, 49, 57, 59, Some([0, 0, 0, 70]), None, 1);
    if ["command_table", "intel_desk", "field_map_board"].contains(&name) {
        c.polygon(
            &[(8, 22), (44, 11), (57, 20), (21, 32)],
            Some([78, 88, 71, 255]),
            Some(dark),
        );
        c.polygon(
            &[(8, 22), (21, 32), (21, 48), (8, 39)],
            Some([48, 57, 48, 255]),
            Some(dark),
        );
        c.polygon(
            &[(21, 32), (57, 20), (57, 37), (21, 48)],
            Some([59, 67, 56, 255]),
            Some(dark),
        );
        for i in 0..5 {
            c.line(17 + i * 6, 21 - i, 30 + i * 3, 30 + i, hi, 1);
        }
        c.line(24, 25, 45, 19, accent, 2);
    } else if ["wall_radar", "radar_screen", "antenna_console"].contains(&name) {
        c.rect(14, 21, 51, 47, Some([45, 63, 62, 255]), Some(dark), 2);
        c.rect(18, 25, 47, 42, Some([14, 36, 36, 255]), Some(steel), 1);
        c.arc(20, 26, 45, 50, 180.0, 350.0, [83, 191, 148, 255], 2);
        c.line(33, 34, 43, 27, [111, 222, 171, 255], 2);
        c.line(33, 45, 33, 53, steel, 3);
    } else if ["projector_console", "satellite_terminal"].contains(&name) {
        c.rect(13, 25, 51, 49, Some([55, 66, 63, 255]), Some(dark), 2);
        c.polygon(
            &[(19, 29), (43, 29), (39, 42), (22, 42)],
            Some([26, 62, 61, 255]),
            Some(steel),
        );
        polyline(&mut c, &[(22, 36), (28, 32), (33, 39), (40, 33)], accent, 2);
        c.ellipse(39, 15, 48, 24, Some(hi), Some(dark), 1);
        c.line(39, 20, 29, 31, steel, 2);
    } else if ["mission_folder", "target_photo_stack"].contains(&name) {
        for j in 0..3 {
            let x = 12 + j * 4;
            let y = 17 + j * 5;
            c.rect(
                x,
                y,
                x + 31,
                y + 27,
                Some([
                    (141 - j * 14) as u8,
                    (117 - j * 11) as u8,
                    (75 - j * 6) as u8,
                    255,
                ]),
                Some(dark),
                2,
            );
            c.line(x + 5, y + 8, x + 25, y + 8, [226, 205, 154, 255], 1);
            c.line(x + 5, y + 14, x + 21, y + 14, [226, 205, 154, 255], 1);
        }
    } else if [
        "tactical_pointer",
        "waypoint_marker_blue",
        "threat_marker_red",
    ]
    .contains(&name)
    {
        let color = if name.contains("blue") {
            [68, 171, 211, 255]
        } else if name.contains("red") {
            [230, 75, 56, 255]
        } else {
            accent
        };
        c.polygon(
            &[(32, 10), (46, 25), (37, 43), (32, 50), (27, 43), (18, 25)],
            Some(color),
            Some(dark),
        );
        c.ellipse(27, 20, 37, 30, Some([230, 225, 183, 255]), Some(dark), 1);
        c.line(32, 45, 32, 56, color, 3);
    } else if name == "signal_flag" {
        c.line(20, 11, 20, 54, steel, 3);
        c.polygon(&[(22, 13), (48, 20), (22, 29)], Some(accent), Some(dark));
        c.line(12, 55, 34, 55, dark, 3);
    } else if name == "briefing_lamp" {
        c.line(30, 29, 30, 48, steel, 4);
        c.polygon(
            &[(15, 28), (24, 14), (43, 14), (52, 28)],
            Some([99, 112, 95, 255]),
            Some(dark),
        );
        c.ellipse(24, 21, 42, 32, Some(accent), Some(dark), 1);
        c.polygon(
            &[(19, 32), (44, 32), (51, 48), (13, 48)],
            Some([237, 196, 97, 35]),
            None,
        );
    } else if name == "comms_headset" {
        c.arc(13, 11, 52, 52, 180.0, 360.0, steel, 6);
        c.rounded_rect(10, 28, 21, 43, 4, [49, 64, 63, 255]);
        c.rounded_rect_outline(10, 28, 21, 43, 4, dark, 2);
        c.rounded_rect(44, 28, 55, 43, 4, [49, 64, 63, 255]);
        c.rounded_rect_outline(44, 28, 55, 43, 4, dark, 2);
        polyline(&mut c, &[(49, 41), (44, 49), (33, 49)], accent, 2);
    } else {
        // General control console / tactical hardware variants.
        c.rect(11, 20, 53, 49, Some([50, 65, 62, 255]), Some(dark), 2);
        c.rect(16, 25, 48, 39, Some([17, 42, 39, 255]), Some(steel), 1);
        for i in 0..4i32 {
            let fill = if i == (seed % 4) as i32 { accent } else { hi };
            c.rect(18 + i * 7, 43, 22 + i * 7, 46, Some(fill), None, 1);
        }
        c.line(21, 17, 44, 17, hi, 2);
    }
    for _ in 0..3 {
        let x = rng.rand_i32(14, 49);
        let y = rng.rand_i32(15, 47);
        if c.pixel(x, y).is_none_or(|p| p[3] == 0) {
            c.set_pixel(x, y, hi);
        }
    }
    c
}

// ---------------------------------------------------------------------------
// render_icon (Python lines 417–473).
// ---------------------------------------------------------------------------

/// Render one 32x32 transparent tactical UI icon (deterministic by index).
fn render_icon(name: &str, index: usize) -> Canvas {
    let mut c = Canvas::transparent(32, 32);
    let dark: Rgba = [14, 21, 22, 255];
    let hi: Rgba = [204, 216, 185, 255];
    let pal: [Rgba; 4] = [
        [105, 190, 144, 255],
        [221, 173, 83, 255],
        [93, 172, 209, 255],
        [219, 94, 75, 255],
    ];
    let color = pal[index % pal.len()];

    c.ellipse(
        3,
        3,
        28,
        28,
        Some([11, 18, 19, 230]),
        Some([65, 91, 83, 255]),
        1,
    );
    if ["objective", "priority", "completed", "locked"].contains(&name) {
        c.polygon(
            &[(16, 5), (25, 15), (16, 26), (7, 15)],
            Some(color),
            Some(dark),
        );
        if name == "completed" {
            polyline(&mut c, &[(10, 16), (14, 20), (22, 10)], hi, 2);
        } else if name == "locked" {
            c.rect(12, 15, 20, 22, Some(hi), Some(dark), 1);
            c.arc(13, 10, 19, 18, 180.0, 360.0, hi, 2);
        } else if name == "priority" {
            c.line(16, 9, 16, 18, hi, 2);
            c.ellipse(15, 21, 17, 23, Some(hi), None, 1);
        }
    } else if ["recon", "radar", "comms"].contains(&name) {
        c.arc(7, 7, 25, 25, 200.0, 340.0, color, 2);
        c.arc(11, 11, 21, 21, 200.0, 340.0, hi, 2);
        c.line(16, 16, 23, 9, color, 2);
        c.ellipse(14, 14, 18, 18, Some(hi), None, 1);
    } else if ["defend", "capture", "rescue", "medical", "civilians"].contains(&name) {
        if name == "medical" {
            c.rect(13, 7, 19, 25, Some(hi), None, 1);
            c.rect(7, 13, 25, 19, Some(hi), None, 1);
        } else if name == "defend" {
            c.polygon(
                &[(16, 6), (25, 10), (23, 21), (16, 26), (9, 21), (7, 10)],
                Some(color),
                Some(hi),
            );
        } else if name == "civilians" {
            for &(x, y) in &[(11, 10), (20, 10)] {
                c.ellipse(x - 2, y - 2, x + 2, y + 2, Some(hi), None, 1);
            }
            c.line(11, 13, 11, 23, color, 3);
            c.line(20, 13, 20, 23, color, 3);
        } else {
            c.rect(8, 8, 24, 24, None, Some(color), 2);
            c.line(8, 16, 24, 16, hi, 2);
            c.line(16, 8, 16, 24, hi, 2);
        }
    } else if ["armor", "convoy", "supply", "escort", "fuel"].contains(&name) {
        c.rect(6, 13, 25, 22, Some(color), Some(dark), 1);
        c.rect(10, 9, 19, 14, Some(hi), Some(dark), 1);
        c.line(17, 11, 27, 8, color, 2);
        for &x in &[9, 20] {
            c.ellipse(x, 21, x + 5, 26, Some(dark), Some(hi), 1);
        }
        if name == "fuel" {
            c.rect(22, 13, 26, 18, Some([205, 96, 66, 255]), None, 1);
        }
    } else if ["bridge", "rail", "route", "checkpoint"].contains(&name) {
        c.line(6, 23, 26, 8, color, 3);
        for t in 0..4 {
            let x = 7 + t * 5;
            let y = 22 - t * 4;
            c.line(x - 2, y + 3, x + 3, y - 1, hi, 1);
        }
        if name == "checkpoint" {
            c.ellipse(20, 6, 27, 13, Some(color), Some(hi), 1);
        }
    } else if ["destroy", "airstrike", "threat", "hazard"].contains(&name) {
        c.polygon(
            &[
                (16, 5),
                (19, 12),
                (27, 13),
                (21, 18),
                (23, 26),
                (16, 21),
                (9, 26),
                (11, 18),
                (5, 13),
                (13, 12),
            ],
            Some(color),
            Some(dark),
        );
        c.ellipse(14, 13, 18, 17, Some(hi), None, 1);
    } else if ["infantry", "engineer", "pilot"].contains(&name) {
        c.ellipse(12, 7, 20, 15, Some(hi), Some(dark), 1);
        c.polygon(
            &[(9, 16), (23, 16), (25, 25), (7, 25)],
            Some(color),
            Some(dark),
        );
        c.line(10, 18, 5, 22, hi, 2);
        c.line(22, 18, 27, 21, hi, 2);
    } else if ["factory", "power", "water"].contains(&name) {
        c.rect(6, 14, 26, 25, Some(color), Some(dark), 1);
        c.polygon(
            &[(6, 14), (12, 9), (12, 14), (18, 9), (18, 14), (26, 14)],
            Some(hi),
            Some(dark),
        );
        if name == "power" {
            c.polygon(
                &[(17, 14), (12, 20), (16, 20), (13, 24), (21, 18), (17, 18)],
                Some([247, 218, 113, 255]),
                None,
            );
        } else if name == "water" {
            c.polygon(
                &[(16, 13), (10, 20), (13, 24), (19, 24), (22, 20)],
                Some([80, 180, 222, 255]),
                None,
            );
        }
    } else if ["extract", "clear"].contains(&name) {
        c.rect(7, 8, 25, 25, None, Some(color), 2);
        polyline(&mut c, &[(10, 22), (16, 11), (22, 22)], hi, 2);
        c.polygon(&[(21, 6), (27, 6), (27, 12)], Some(color), None);
    } else {
        c.rect(8, 8, 24, 24, Some(color), Some(dark), 2);
        c.line(11, 12, 21, 12, hi, 2);
        c.line(11, 17, 19, 17, hi, 2);
    }
    c
}

// ---------------------------------------------------------------------------
// render_personnel (Python lines 476–505).
// ---------------------------------------------------------------------------

/// Render one 64x64 personnel portrait, seeded per portrait.
fn render_personnel(name: &str, role: &str, skin: [u8; 3], uniform: [u8; 3], seed: u64) -> Canvas {
    let _ = name;
    let mut rng = Rng::new(Seed::new(seed));
    let mut c = Canvas::transparent(64, 64);
    c.rounded_rect(3, 3, 60, 61, 5, [18, 28, 29, 255]);
    c.rounded_rect_outline(3, 3, 60, 61, 5, [80, 108, 94, 255], 2);
    // shoulders and uniform
    c.polygon(
        &[(8, 61), (11, 42), (22, 34), (42, 34), (54, 43), (58, 61)],
        Some(rgba3(uniform)),
        Some([14, 20, 21, 255]),
    );
    c.rect(26, 35, 38, 49, Some(rgba3(skin)), None, 1);
    // face, ears, jaw
    c.ellipse(
        17,
        12,
        47,
        40,
        Some(rgba3(skin)),
        Some([19, 24, 22, 255]),
        2,
    );
    c.rect(15, 23, 19, 30, Some(rgba3(skin)), None, 1);
    c.rect(45, 23, 49, 30, Some(rgba3(skin)), None, 1);
    // cap / hair / helmet changes with role
    let helmet = ["commander", "engineer", "radar", "pilot", "scout"].contains(&role);
    if helmet {
        c.pieslice(14, 7, 50, 29, 180.0, 360.0, rgba3(clamp3(uniform, 22)));
        pieslice_outlined(&mut c, 14, 7, 50, 29, 180.0, 360.0, [16, 21, 20, 255], 2);
        c.rect(13, 20, 51, 24, Some(rgba3(clamp3(uniform, 30))), None, 1);
        if role == "pilot" {
            c.rect(
                18,
                23,
                46,
                28,
                Some([58, 92, 94, 255]),
                Some([185, 195, 173, 255]),
                1,
            );
        }
    } else {
        c.pieslice(15, 8, 49, 31, 180.0, 360.0, [49, 36, 29, 255]);
        pieslice_outlined(&mut c, 15, 8, 49, 31, 180.0, 360.0, [16, 21, 20, 255], 1);
    }
    c.rect(23, 24, 27, 26, Some([25, 28, 25, 255]), None, 1);
    c.rect(37, 24, 41, 26, Some([25, 28, 25, 255]), None, 1);
    c.line(29, 33, 35, 33, [91, 50, 42, 255], 2);
    // role patch
    c.rect(
        11,
        44,
        22,
        53,
        Some([27, 40, 35, 255]),
        Some([174, 186, 145, 255]),
        1,
    );
    if role == "medic" {
        c.rect(15, 46, 18, 51, Some([218, 224, 197, 255]), None, 1);
        c.rect(13, 48, 20, 49, Some([218, 224, 197, 255]), None, 1);
    } else {
        c.line(13, 51, 20, 46, [218, 184, 94, 255], 2);
    }
    for _ in 0..6 {
        let x = rng.rand_i32(6, 57);
        let y = rng.rand_i32(5, 58);
        if c.pixel(x, y) == Some([0, 0, 0, 0]) {
            c.set_pixel(x, y, [93, 127, 105, 130]);
        }
    }
    c
}

// ---------------------------------------------------------------------------
// Atlas + storyboard helpers (Python `make_atlas`, `make_storyboard`).
// ---------------------------------------------------------------------------

/// Python `make_atlas`: composes the grid atlas (8 columns), and returns the
/// composed canvas plus the `manifest["atlases"]` record; the PNG is written
/// by the caller (`out.write_sprite`, same pack-relative path).
#[allow(clippy::too_many_arguments)]
fn make_atlas(
    out: &PackWriter,
    names: &[&str],
    cells: &[Canvas],
    cell: u32,
    cols: usize,
    rel_image: &str,
    rel_mapping: &str,
    kind: &str,
) -> Result<(Canvas, Value), PackError> {
    let refs: Vec<&Canvas> = cells.iter().collect();
    let atlas = compose_atlas(&refs, cols, cell, true);
    let rows = cells.len().div_ceil(cols).max(1);

    let mut rects = Map::new();
    for (i, name) in names.iter().enumerate() {
        let x = (i % cols) * cell as usize;
        let y = (i / cols) * cell as usize;
        rects.insert(
            (*name).to_string(),
            json!({"x": x as u64, "y": y as u64, "width": cell, "height": cell}),
        );
    }
    let mapping = json!({
        "version": VERSION,
        "kind": kind,
        "image": rel_image,
        "size": [atlas.width(), atlas.height()],
        "tile_size": cell,
        "columns": cols,
        "rows": rows,
        "sprites": Value::Object(rects),
    });
    let png_bytes = atlas.encode_png()?;
    out.write_bytes(rel_image, &png_bytes)?;
    out.write_json(rel_mapping, &mapping)?;
    let sha256_file = super::sha256_hex(&png_bytes);
    let record = json!({
        "file": rel_image,
        "category": kind,
        "columns": cols,
        "rows": rows,
        "cell_size": [cell, cell],
        "sprite_count": cells.len(),
        "sha256_file": sha256_file,
        "mapping": rel_mapping,
    });
    Ok((atlas, record))
}

/// Python `make_storyboard`: a 480x204 contact sheet of the six frames
/// (160x90 thumbnails, NEAREST) with a caption row. Returns `(path, canvas)`;
/// the caller writes and registers it.
fn make_storyboard(scene_id: &str, title: &str, frames: &[Canvas]) -> (String, Canvas) {
    let (w, h) = (160u32, 90u32);
    let mut sheet = Canvas::new_filled(w * 3, h * 2 + 24, [8, 13, 15, 255]);
    for (i, frame) in frames.iter().enumerate() {
        let thumb = frame.resize_nearest(w, h);
        sheet.alpha_composite(
            &thumb,
            (((i % 3) * w as usize) as i32, ((i / 3) * h as usize) as i32),
        );
    }
    // d.rectangle((0, h*2, sheet.width, sheet.height)) — PIL clips the
    // inclusive bottom/right edges to the last pixel.
    sheet.rect(
        0,
        (h * 2) as i32,
        (w * 3) as i32 - 1,
        (h * 2 + 24) as i32 - 1,
        Some([8, 13, 15, 255]),
        None,
        1,
    );
    draw_text(
        &mut sheet,
        6,
        h as i32 * 2 + 6,
        &format!("{scene_id} | {title}"),
        [221, 228, 211, 255],
    );
    let rel = format!("sprites/briefings/previews/{scene_id}_storyboard.png");
    (rel, sheet)
}

// ---------------------------------------------------------------------------
// generate (Python `generate`, lines 534–658).
// ---------------------------------------------------------------------------

/// Generate this layer into `out` and return its contributions.
pub fn generate(out: &PackWriter, seed: u64) -> Result<LayerOutput, PackError> {
    let mut result = LayerOutput::default();
    let mut timeline = TimelineContribution {
        schema_version: "1.3.0",
        ..TimelineContribution::default()
    };
    let mut generated_pngs: Vec<String> = Vec::new();
    let mut frame_records_all: Vec<Value> = Vec::new();

    // 1. Eight briefing/debrief sequences, six frames each, deterministic and
    //    additive (Python seed: `scene_seed = seed + (si+1)*109`, frames
    //    `scene_seed + fi*53`).
    for (si, &(scene_id, title, act, motif, ..)) in BRIEFINGS.iter().enumerate() {
        let captions = BRIEFINGS[si].5;
        let scene_seed = seed + (si as u64 + 1) * 109;
        let mut frames: Vec<Canvas> = Vec::with_capacity(6);
        let mut frame_records: Vec<Value> = Vec::with_capacity(6);
        for (fi, caption) in captions.iter().enumerate() {
            let rel = format!("sprites/briefings/frames/{scene_id}/frame_{fi:03}.png");
            let frame_seed = scene_seed + fi as u64 * 53;
            let canvas = render_briefing(si, fi as i32, frame_seed);
            let file_sha = write_and_hash(out, &rel, &canvas)?;
            let rec = json!({
                "index": fi,
                "file": rel,
                "duration_ms": if fi < 5 { 900u64 } else { 1200u64 },
                "caption": caption,
                "seed": frame_seed,
                "sha256": file_sha,
            });
            result.sprites.push(out.write_sprite(
                &rel,
                &canvas,
                SpriteSpec {
                    category: "briefing_frame",
                    generated_by: Some(GENERATED_BY),
                    seed: Some(frame_seed),
                    version: None,
                    extra: vec![
                        ("scene_type", json!("briefing")),
                        ("sequence_id", json!(scene_id)),
                        ("frame_index", json!(fi)),
                        ("campaign_act", json!(act)),
                        ("caption", json!(caption)),
                        ("canvas", json!([W, H])),
                    ],
                },
            )?);
            frames.push(canvas);
            frame_records.push(rec.clone());
            frame_records_all.push(rec);
            generated_pngs.push(rel);
        }
        timeline.sequences.push(json!({
            "id": scene_id,
            "title": title,
            "campaign_act": act,
            "scene_type": "briefing",
            "fps": 6,
            "loop": true,
            "canvas": [W, H],
            "environment": "command_room",
            "motif": motif,
            "frames": frame_records,
            "transitions": {
                "in": "fade_from_black",
                "out": "fade_to_black",
                "camera": "subtle_monitor_push",
                "transition_in_ms": 350,
                "transition_out_ms": 450,
            },
            "camera_path": [
                {"frame": 0, "zoom": 1.0, "pan_x": 0, "pan_y": 0},
                {"frame": 3, "zoom": 1.012, "pan_x": 1, "pan_y": 0},
                {"frame": 5, "zoom": 1.018, "pan_x": 1, "pan_y": -1},
            ],
            "audio_cues": ["command_room_ambience", "radio_voiceover", "soft_console_beeps"],
            "effects_cues": ["radar_sweep", "route_pulse", "telemetry_blink"],
            "captions_external": true,
            "briefing_fields": {
                "mission_intent": captions[0],
                "primary_objective": captions[5],
                "intel_motif": motif,
                "sequence_role": if act.starts_with("Debrief") {
                    "post-mission debrief"
                } else {
                    "pre-mission briefing"
                },
            },
        }));
        let (board_rel, board) = make_storyboard(scene_id, title, &frames);
        result.sprites.push(out.write_sprite(
            &board_rel,
            &board,
            SpriteSpec {
                category: "briefing_storyboard",
                generated_by: Some(GENERATED_BY),
                seed: Some(scene_seed),
                version: None,
                extra: vec![
                    ("sequence_id", json!(scene_id)),
                    ("frame_count", json!(6u64)),
                    ("scene_type", json!("briefing")),
                ],
            },
        )?);
        generated_pngs.push(board_rel);
    }

    // 2. Additional cinematic mission cards / campaign key art (Python seed:
    //    `SEED + 9100 + si`).
    let mut card_canvases: Vec<Canvas> = Vec::with_capacity(BRIEFINGS.len());
    let mut card_rels: Vec<String> = Vec::with_capacity(BRIEFINGS.len());
    for (si, spec) in BRIEFINGS.iter().enumerate() {
        let (scene_id, act) = (spec.0, spec.2);
        let card_seed = seed + 9100 + si as u64;
        let canvas = render_card(si, card_seed);
        let rel = format!("sprites/briefings/campaign_cards/{scene_id}_card.png");
        result.sprites.push(out.write_sprite(
            &rel,
            &canvas,
            SpriteSpec {
                category: "briefing_campaign_card",
                generated_by: Some(GENERATED_BY),
                seed: Some(card_seed),
                version: None,
                extra: vec![
                    ("art_role", json!("briefing_key_art")),
                    ("sequence_id", json!(scene_id)),
                    ("campaign_act", json!(act)),
                    ("canvas", json!([W, H])),
                ],
            },
        )?);
        card_canvases.push(canvas);
        generated_pngs.push(rel.clone());
        card_rels.push(rel);
    }
    // 3. Reusable transparent props, UI icons, and briefing personnel
    //    portraits (Python seeds `12000+i`, `13000+i`, `14000+i`).
    let mut prop_canvases: Vec<Canvas> = Vec::with_capacity(PROP_NAMES.len());
    for (i, name) in PROP_NAMES.iter().enumerate() {
        let prop_seed = seed + 12000 + i as u64;
        let canvas = render_prop(name, prop_seed);
        let rel = format!("sprites/briefings/art/props/{name}.png");
        result.sprites.push(out.write_sprite(
            &rel,
            &canvas,
            SpriteSpec {
                category: "briefing_prop",
                generated_by: Some(GENERATED_BY),
                seed: Some(prop_seed),
                version: None,
                extra: vec![
                    ("art_role", json!("briefing_prop")),
                    ("tile_size", json!([64u64, 64u64])),
                    ("revision", json!("v6.1")),
                ],
            },
        )?);
        prop_canvases.push(canvas);
        generated_pngs.push(rel);
    }
    let mut icon_canvases: Vec<Canvas> = Vec::with_capacity(ICON_NAMES.len());
    for (i, name) in ICON_NAMES.iter().enumerate() {
        let icon_seed = seed + 13000 + i as u64;
        let canvas = render_icon(name, i);
        let rel = format!("sprites/briefings/art/icons/icon_{name}.png");
        result.sprites.push(out.write_sprite(
            &rel,
            &canvas,
            SpriteSpec {
                category: "briefing_icon",
                generated_by: Some(GENERATED_BY),
                seed: Some(icon_seed),
                version: None,
                extra: vec![
                    ("art_role", json!("briefing_ui_icon")),
                    ("tile_size", json!([32u64, 32u64])),
                    ("revision", json!("v6.1")),
                ],
            },
        )?);
        icon_canvases.push(canvas);
        generated_pngs.push(rel);
    }
    for (i, &(name, role, skin, uniform)) in PERSONNEL.iter().enumerate() {
        let portrait_seed = seed + 14000 + i as u64;
        let canvas = render_personnel(name, role, skin, uniform, portrait_seed);
        let rel = format!("sprites/briefings/art/personnel/{name}.png");
        result.sprites.push(out.write_sprite(
            &rel,
            &canvas,
            SpriteSpec {
                category: "briefing_personnel",
                generated_by: Some(GENERATED_BY),
                seed: Some(portrait_seed),
                version: None,
                extra: vec![
                    ("art_role", json!("briefing_personnel_portrait")),
                    ("role", json!(role)),
                    ("tile_size", json!([64u64, 64u64])),
                    ("revision", json!("v6.1")),
                ],
            },
        )?);
        generated_pngs.push(rel);
    }

    // Atlases (Python `make_atlas`, dual registration: a `briefing_atlas`
    // sprite record plus a `manifest["atlases"]` record).
    let (prop_atlas_canvas, prop_atlas_record) = make_atlas(
        out,
        PROP_NAMES,
        &prop_canvases,
        64,
        8,
        "atlases/briefing_props_v6_1.png",
        "atlases/briefing_props_v6_1.json",
        "briefing_prop_atlas",
    )?;
    let (icon_atlas_canvas, icon_atlas_record) = make_atlas(
        out,
        ICON_NAMES,
        &icon_canvases,
        32,
        8,
        "atlases/briefing_icons_v6_1.png",
        "atlases/briefing_icons_v6_1.json",
        "briefing_icon_atlas",
    )?;
    for (canvas, record, rel, role, tile, count, atlas_seed) in [
        (
            &prop_atlas_canvas,
            &prop_atlas_record,
            "atlases/briefing_props_v6_1.png",
            "briefing_prop_atlas",
            64u64,
            PROP_NAMES.len(),
            seed,
        ),
        (
            &icon_atlas_canvas,
            &icon_atlas_record,
            "atlases/briefing_icons_v6_1.png",
            "briefing_icon_atlas",
            32u64,
            ICON_NAMES.len(),
            seed + 1,
        ),
    ] {
        result.sprites.push(out.write_sprite(
            rel,
            canvas,
            SpriteSpec {
                category: "briefing_atlas",
                generated_by: Some(GENERATED_BY),
                seed: Some(atlas_seed),
                version: None,
                extra: vec![
                    ("art_role", json!(role)),
                    ("tile_size", json!([tile, tile])),
                    ("sprite_count", json!(count)),
                ],
            },
        )?);
        result.atlases.push(record.clone());
        generated_pngs.push(rel.to_string());
    }

    // 4. Overview sheet of the new scenes and extra card-art.
    let mut overview = Canvas::new_filled(4 * W, 2 * H, [6, 11, 13, 255]);
    for (i, card) in card_canvases.iter().enumerate() {
        let thumb = card.resize_nearest(W, H);
        overview.alpha_composite(
            &thumb,
            (((i % 4) * W as usize) as i32, ((i / 4) * H as usize) as i32),
        );
    }
    let overview_rel = "previews/v6_1_briefing_campaign_art_overview.png";
    result.sprites.push(out.write_sprite(
        overview_rel,
        &overview,
        SpriteSpec {
            category: "briefing_overview",
            generated_by: Some(GENERATED_BY),
            seed: Some(seed),
            version: None,
            extra: vec![
                ("art_role", json!("v6.1_briefing_campaign_overview")),
                ("card_count", json!(card_canvases.len())),
            ],
        },
    )?);
    generated_pngs.push(overview_rel.to_string());

    // 5. Timeline contribution: previous cinematics and campaign membership
    //    survive via the builder's merge (replace-by-id). The Python sorts
    //    the merged list by (`scene_type != "briefing"` first, then id); this
    //    layer's sequences are all briefings and already id-sorted.
    timeline.sequences.sort_unstable_by(|a, b| {
        let ka = (a["id"].as_str().unwrap_or_default(), true);
        let kb = (b["id"].as_str().unwrap_or_default(), true);
        ka.cmp(&kb)
    });
    timeline.notes = vec![
        "v6.1 adds eight tactical briefing/debrief sequences, each with six frames and externally timed captions.".to_string(),
        "Briefing sequences are indexed separately and are not added to the numbered narrative campaign acts.".to_string(),
        "v6.1 adds eight painted-style pixel-art mission cards, 16 transparent briefing props, 32 tactical UI icons, and eight personnel portraits.".to_string(),
        "Briefing ambience/radio cues are metadata only; no audio files are bundled.".to_string(),
        "Automated checks validate assets and metadata; Godot playback and on-screen reading still require engine-side review.".to_string(),
    ];
    generated_pngs.sort();
    generated_pngs.dedup();
    result.supporting_assets = generated_pngs.clone();

    // Manifest top-level keys this layer asserts.
    let mut briefing_count_summary = Map::new();
    briefing_count_summary.insert(
        "timeline".to_string(),
        json!("sprites/cinematics/timeline.json"),
    );
    briefing_count_summary.insert("sequence_count".to_string(), json!(BRIEFINGS.len()));
    briefing_count_summary.insert("frame_count".to_string(), json!(frame_records_all.len()));
    briefing_count_summary.insert("canvas".to_string(), json!([W, H]));
    briefing_count_summary.insert(
        "frame_duration_ms".to_string(),
        json!({"default": 900u64, "final": 1200u64}),
    );
    briefing_count_summary.insert(
        "campaign_card_count".to_string(),
        json!(card_canvases.len()),
    );
    briefing_count_summary.insert("prop_count".to_string(), json!(PROP_NAMES.len()));
    briefing_count_summary.insert("icon_count".to_string(), json!(ICON_NAMES.len()));
    briefing_count_summary.insert(
        "personnel_portrait_count".to_string(),
        json!(PERSONNEL.len()),
    );
    briefing_count_summary.insert(
        "props_atlas".to_string(),
        json!({
            "image": prop_atlas_record["file"],
            "mapping": prop_atlas_record["mapping"],
            "cell_size": [64u64, 64u64],
            "sprite_count": PROP_NAMES.len(),
        }),
    );
    briefing_count_summary.insert(
        "icons_atlas".to_string(),
        json!({
            "image": icon_atlas_record["file"],
            "mapping": icon_atlas_record["mapping"],
            "cell_size": [32u64, 32u64],
            "sprite_count": ICON_NAMES.len(),
        }),
    );
    result.manifest_extra = Map::from_iter([
        ("schema_version".to_string(), json!("1.3.0")),
        ("generator_version".to_string(), json!(VERSION)),
        (
            "release_note".to_string(),
            json!(
                "v6.1 additive tactical briefing/debrief scenes and campaign art; all v5.x and v6.0 assets retained."
            ),
        ),
        // Partial `cinematics` update: the builder merges this over previous
        // layers' object and recomputes `sequence_count` / `frame_count` /
        // `campaign_scene_count` from the merged timeline (Python semantics).
        (
            "cinematics".to_string(),
            json!({
                "timeline": "sprites/cinematics/timeline.json",
                "new_briefing_scene_count": BRIEFINGS.len(),
                "new_briefing_frame_count": frame_records_all.len(),
            }),
        ),
        (
            "briefings".to_string(),
            Value::Object(briefing_count_summary),
        ),
    ]);
    // Report merge stats must be computed before `timeline` is moved into
    // the layer output below.
    let existing = out.read_json("sprites/cinematics/timeline.json")?;
    let (
        previous_sequence_count,
        previous_sequences_retained,
        total_sequence_count,
        total_timeline_frames,
    ) = merge_stats(existing.as_ref(), &timeline.sequences);
    // Timeline `briefings` index: the 8 briefing sequence ids and their
    // frames, the painted card paths, and the two atlas files.
    let mut timeline_extra = Map::new();
    timeline_extra.insert(
        "briefings".to_string(),
        json!({
            "sequence_ids": timeline
                .sequences
                .iter()
                .map(|s| s["id"].clone())
                .collect::<Vec<Value>>(),
            "sequence_count": timeline.sequences.len(),
            "frame_count": timeline
                .sequences
                .iter()
                .map(|s| s["frames"].as_array().map_or(0, Vec::len))
                .sum::<usize>(),
            "cards": card_rels,
            "props_atlas": prop_atlas_record["file"],
            "icons_atlas": icon_atlas_record["file"],
        }),
    );
    timeline.generator_version = VERSION;
    timeline.seed = Some(seed);
    timeline.extra = timeline_extra;
    result.timeline = Some(timeline);

    // Build report (Python `campaign_build_report_v6_1_0.json`).
    let all_new_files_exist = generated_pngs.iter().all(|rel| out.path(rel).is_file());
    let mut all_briefing_frame_hashes_match = true;
    for rec in &frame_records_all {
        let rel = rec["file"].as_str().unwrap_or_default();
        match out.sha256_file(rel) {
            Ok(sha) => {
                if sha != rec["sha256"].as_str().unwrap_or_default() {
                    all_briefing_frame_hashes_match = false;
                }
            }
            Err(_) => all_briefing_frame_hashes_match = false,
        }
    }
    let unique_frame_hash_count = frame_records_all
        .iter()
        .map(|r| r["sha256"].as_str().unwrap_or_default().to_string())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let report = json!({
        "asset_pack_version": VERSION,
        "generator_version": VERSION,
        "seed": seed,
        "previous_sequence_count": previous_sequence_count,
        "previous_sequences_retained": previous_sequences_retained,
        "new_briefing_scene_count": BRIEFINGS.len(),
        "new_briefing_frames": frame_records_all.len(),
        "total_sequence_count": total_sequence_count,
        "total_timeline_frames": total_timeline_frames,
        "new_storyboards": BRIEFINGS.len(),
        "new_campaign_cards": card_canvases.len(),
        "new_briefing_props": PROP_NAMES.len(),
        "new_briefing_icons": ICON_NAMES.len(),
        "new_personnel_portraits": PERSONNEL.len(),
        "new_atlases": 2,
        "generated_png_count": generated_pngs.len(),
        "all_new_files_exist": all_new_files_exist,
        "all_briefing_frame_hashes_match": all_briefing_frame_hashes_match,
        "unique_frame_hash_count": unique_frame_hash_count,
        "runtime_validation": "NOT_RUN — Godot playback and integration are separate checks.",
    });
    result
        .reports
        .push(("campaign_build_report_v6_1_0.json".to_string(), report));

    Ok(result)
}

/// Write a PNG through the pack writer and return the SHA-256 of the file.
fn write_and_hash(out: &PackWriter, rel: &str, canvas: &Canvas) -> Result<String, PackError> {
    let bytes = canvas.encode_png()?;
    out.write_bytes(rel, &bytes)?;
    Ok(super::sha256_hex(&bytes))
}

/// Merge-statistics for the build report, Python lines 647–652. When an
/// existing `timeline.json` is present its sequences are merged by id
/// (replaced by this layer's records) exactly like the Python; on a fresh
/// pack the totals fall back to this layer's own contribution.
fn merge_stats(existing: Option<&Value>, layer_sequences: &[Value]) -> (usize, bool, usize, usize) {
    let Some(existing) = existing else {
        return (0, true, layer_sequences.len(), 48);
    };
    let existing_sequences = existing["sequences"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let existing_ids: std::collections::BTreeSet<String> = existing_sequences
        .iter()
        .filter_map(|s| s["id"].as_str().map(str::to_string))
        .collect();
    let old_briefings: std::collections::BTreeSet<String> = existing_sequences
        .iter()
        .filter(|s| s.get("scene_type").and_then(Value::as_str) == Some("briefing"))
        .filter_map(|s| s["id"].as_str().map(str::to_string))
        .collect();
    let mut merged: Vec<(String, Value)> = existing_sequences
        .iter()
        .filter_map(|s| {
            let id = s["id"].as_str()?.to_string();
            Some((id, s.clone()))
        })
        .collect();
    let replaced: std::collections::BTreeSet<String> = layer_sequences
        .iter()
        .filter_map(|s| s["id"].as_str().map(str::to_string))
        .collect();
    merged.retain(|(id, _)| !replaced.contains(id));
    merged.extend(
        layer_sequences
            .iter()
            .filter_map(|s| {
                let id = s["id"].as_str()?.to_string();
                Some((id, s.clone()))
            })
            .collect::<Vec<_>>(),
    );
    let total_timeline_frames = merged
        .iter()
        .map(|(_, s)| s["frames"].as_array().map_or(0, Vec::len))
        .sum();
    (
        existing_ids.difference(&old_briefings).count(),
        merged.len() >= existing_ids.len(),
        merged.len(),
        total_timeline_frames,
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::*;

    static TMP_SEQ: AtomicU32 = AtomicU32::new(0);

    /// Fresh staging root per test, mirroring the Python tests' temp dir.
    fn temp_root(tag: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "cnc-procgen-briefing_v61-{}-{}-{}",
            tag,
            std::process::id(),
            TMP_SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(path.join("sprites/cinematics")).expect("create temp pack root");
        path
    }

    /// Cleanup helper: remove the staged pack tree.
    fn cleanup(root: &PathBuf) {
        std::fs::remove_dir_all(root).ok();
    }

    /// Minimum alpha across every pixel (Python `getchannel("A").getextrema()[0]`).
    fn min_alpha(canvas: &Canvas) -> u8 {
        canvas
            .as_rgba()
            .pixels()
            .map(|pixel| pixel.0[3])
            .min()
            .unwrap_or(0)
    }

    /// Python test 1: 8 specs, unique scene ids, 6 captions each.
    #[test]
    fn briefings_table_shape() {
        assert_eq!(BRIEFINGS.len(), 8);
        let ids: std::collections::BTreeSet<&str> = BRIEFINGS.iter().map(|spec| spec.0).collect();
        assert_eq!(ids.len(), 8);
        assert!(BRIEFINGS.iter().all(|spec| spec.5.len() == 6));
    }

    /// Python test 2: each briefing animation has six distinct 320x180 frames
    /// (8 x 6 = 48 frames).
    #[test]
    fn each_briefing_animation_has_distinct_frames() {
        for (i, spec) in BRIEFINGS.iter().enumerate() {
            let mut hashes = std::collections::BTreeSet::new();
            for frame in 0..6i32 {
                let img =
                    render_briefing(i, frame, DEFAULT_SEED + i as u64 * 109 + frame as u64 * 53);
                assert_eq!(img.width(), 320);
                assert_eq!(img.height(), 180);
                hashes.insert(img.as_rgba().as_raw().clone());
            }
            assert_eq!(hashes.len(), 6, "{}", spec.0);
        }
    }

    /// Python test 3: fixed-seed rendering is deterministic (frames + cards).
    #[test]
    fn fixed_seed_rendering_is_deterministic() {
        let a = render_briefing(3, 2, 98765);
        let b = render_briefing(3, 2, 98765);
        assert_eq!(a.as_rgba().as_raw(), b.as_rgba().as_raw());
        let c = render_card(6, 4321);
        let d = render_card(6, 4321);
        assert_eq!(c.as_rgba().as_raw(), d.as_rgba().as_raw());
    }

    /// Python test 4: eight distinct 320x180 card illustrations.
    #[test]
    fn cards_are_distinct_illustrations() {
        let mut hashes = std::collections::BTreeSet::new();
        for i in 0..8usize {
            let img = render_card(i, 70100 + i as u64);
            assert_eq!(img.width(), 320);
            assert_eq!(img.height(), 180);
            hashes.insert(img.as_rgba().as_raw().clone());
        }
        assert_eq!(hashes.len(), 8);
    }

    /// Python test 5: reusable art dimensions and partial alpha transparency.
    #[test]
    fn reusable_art_dimensions_and_alpha() {
        assert_eq!(PROP_NAMES.len(), 16);
        assert_eq!(ICON_NAMES.len(), 32);
        assert_eq!(PERSONNEL.len(), 8);
        for (i, name) in PROP_NAMES.iter().enumerate() {
            let img = render_prop(name, DEFAULT_SEED + i as u64);
            assert_eq!(img.width(), 64);
            assert_eq!(img.height(), 64);
            assert!(min_alpha(&img) < 255, "{name}: transparent pixels expected");
        }
        for (i, name) in ICON_NAMES.iter().enumerate() {
            let img = render_icon(name, i);
            assert_eq!(img.width(), 32);
            assert_eq!(img.height(), 32);
            assert!(min_alpha(&img) < 255, "{name}: transparent pixels expected");
        }
        for (i, (name, role, skin, uniform)) in PERSONNEL.iter().enumerate() {
            let img = render_personnel(name, role, *skin, *uniform, DEFAULT_SEED + i as u64);
            assert_eq!(img.width(), 64);
            assert_eq!(img.height(), 64, "{name}");
        }
    }

    /// Python test 6, adapted to the [`LayerOutput`] seam: with a pre-existing
    /// v6.0 baseline sequence, regeneration preserves it and adds the 48
    /// briefing frames; report and atlas artefacts land at the pack-root
    /// pack-relative paths.
    #[test]
    fn generator_preserves_baseline_sequence_and_adds_briefings() {
        let root = temp_root("baseline");
        let out = PackWriter::new(root.clone());
        // Pre-existing v6.0 timeline, exactly the Python test's baseline.
        out.write_json(
            "sprites/cinematics/timeline.json",
            &json!({
                "schema_version": "1.2.0",
                "sequences": [{"id": "07_baseline", "title": "Retained baseline", "frames": [], "canvas": [320, 180]}],
                "campaigns": [],
                "supporting_assets": [],
                "notes": [],
            }),
        )
        .expect("seed baseline timeline");

        let result = generate(&out, DEFAULT_SEED).expect("generate");
        let timeline = result.timeline.as_ref().expect("timeline contribution");

        // 8 briefing sequences; the baseline belongs to the merged timeline
        // (asserted via the report's merge stats below).
        assert_eq!(timeline.sequences.len(), 8);
        for spec in BRIEFINGS {
            assert!(
                timeline.sequences.iter().any(|s| s["id"] == spec.0),
                "{} contributed",
                spec.0
            );
        }

        // report arithmetic as asserted by the Python test
        // The layer's timeline contribution holds only its own 8 sequences;
        // the builder merges them over the pre-existing baseline by id. The
        // report's merge statistics expose the Python's retention checks.
        let report = result
            .reports
            .iter()
            .find(|(rel, _)| rel == "campaign_build_report_v6_1_0.json")
            .expect("report contribution")
            .1
            .clone();
        assert_eq!(report["new_briefing_frames"], 48);
        assert_eq!(report["previous_sequence_count"], 1);
        assert_eq!(report["previous_sequences_retained"], true);
        assert_eq!(report["total_sequence_count"], 9);
        assert_eq!(report["total_timeline_frames"], 48);

        // 48 briefing_frame sprite records
        let frame_sprites = result
            .sprites
            .iter()
            .filter(|sprite| sprite["category"] == "briefing_frame")
            .count();
        assert_eq!(frame_sprites, 48);
        cleanup(&root);
    }

    /// Combined post-generate inspection over one staged pack (kept to a
    /// single `generate` run): category counts + unique ids, 320x180 frame
    /// geometry with timing metadata, timeline shape (8 briefing sequences,
    /// schema 1.3.0) and atlas composition/mappings — mirroring the Python
    /// suite's manifest assertions.
    #[test]
    fn full_generate_manifest_timeline_and_atlas_checks() {
        let root = temp_root("combined");
        let out = PackWriter::new(root.clone());
        let result = generate(&out, DEFAULT_SEED).expect("generate");

        // --- sprite records: counts, unique ids, provenance stamps
        assert_eq!(result.sprites.len(), 123); // 48+8+8+16+32+8+2+1
        let mut ids = std::collections::BTreeSet::new();
        for sprite in &result.sprites {
            let id = sprite["id"].as_str().expect("sprite id").to_string();
            assert!(ids.insert(id), "duplicate sprite id: {}", sprite["id"]);
            assert_eq!(sprite["generated_by"], GENERATED_BY);
        }
        let count = |category: &str| {
            result
                .sprites
                .iter()
                .filter(|sprite| sprite["category"].as_str() == Some(category))
                .count()
        };
        assert_eq!(count("briefing_frame"), 48);
        assert_eq!(count("briefing_storyboard"), 8);
        assert_eq!(count("briefing_campaign_card"), 8);
        assert_eq!(count("briefing_prop"), 16);
        assert_eq!(count("briefing_icon"), 32);
        assert_eq!(count("briefing_personnel"), 8);
        assert_eq!(count("briefing_atlas"), 2);
        assert_eq!(count("briefing_overview"), 1);

        // --- timeline: schema, 8 briefing sequences with frame metadata
        let timeline = result.timeline.as_ref().expect("timeline contribution");
        assert_eq!(timeline.schema_version, "1.3.0");
        assert_eq!(timeline.sequences.len(), 8);
        let total_frames = timeline
            .sequences
            .iter()
            .map(|seq| seq["frames"].as_array().expect("frames").len())
            .sum::<usize>();
        assert_eq!(total_frames, 48);
        for seq in &timeline.sequences {
            assert_eq!(seq["scene_type"], "briefing");
            assert_eq!(seq["canvas"], json!([320, 180]));
            assert_eq!(seq["fps"], 6);
            assert_eq!(seq["loop"], true);
            for (fi, frame) in seq["frames"].as_array().expect("frames").iter().enumerate() {
                assert_eq!(frame["duration_ms"], if fi < 5 { 900 } else { 1200 });
                let rel = frame["file"].as_str().expect("frame file");
                assert!(rel.starts_with("sprites/briefings/frames/"), "{rel}");
                let sprite = result
                    .sprites
                    .iter()
                    .find(|s| s["file"].as_str() == Some(rel))
                    .expect("frame sprite record");
                assert_eq!(sprite["size"], json!([320, 180]));
                assert!(out.path(rel).is_file(), "{rel} must exist");
            }
        }

        // --- manifest["atlases"] holds both atlas records
        assert_eq!(result.atlases.len(), 2);
        for (file, cols, rows, cell) in [
            ("atlases/briefing_props_v6_1.png", 8usize, 2usize, 64u64),
            ("atlases/briefing_icons_v6_1.png", 8, 4, 32),
        ] {
            let record = result
                .atlases
                .iter()
                .find(|a| a["file"].as_str() == Some(file))
                .expect("atlas record");
            assert_eq!(record["columns"], cols);
            assert_eq!(record["rows"], rows);
            assert_eq!(record["cell_size"], json!([cell, cell]));
            let png = std::fs::read(out.path(file)).expect("atlas png");
            let img = image::load_from_memory(&png).expect("atlas image");
            assert_eq!(img.width(), cols as u32 * cell as u32);
            assert_eq!(img.height(), rows as u32 * cell as u32);
        }
        cleanup(&root);
    }

    /// Repeated generate() with the same seed produces byte-identical output.
    #[test]
    fn repeated_generate_is_deterministic() {
        let first_root = temp_root("determinism-a");
        let second_root = temp_root("determinism-b");
        let rel = "sprites/briefings/frames/briefing_01_command_rundown/frame_000.png";
        let atlas = "atlases/briefing_icons_v6_1.png";
        let (first_frame, first_atlas, first_report) = {
            let out = PackWriter::new(first_root.clone());
            let result = generate(&out, DEFAULT_SEED).expect("generate");
            (
                std::fs::read(out.path(rel)).expect("frame bytes"),
                std::fs::read(out.path(atlas)).expect("atlas bytes"),
                result.reports[0].1.clone(),
            )
        };
        let (second_frame, second_atlas, second_report) = {
            let out = PackWriter::new(second_root.clone());
            let result = generate(&out, DEFAULT_SEED).expect("generate");
            (
                std::fs::read(out.path(rel)).expect("frame bytes"),
                std::fs::read(out.path(atlas)).expect("atlas bytes"),
                result.reports[0].1.clone(),
            )
        };
        assert_eq!(first_frame, second_frame);
        assert_eq!(first_atlas, second_atlas);
        assert_eq!(first_report, second_report);
        cleanup(&first_root);
        cleanup(&second_root);
    }

    /// Timeline ordering contract: non-briefing sequences first (by id), then
    /// briefings (by id) — applied by the builder against the merged list;
    /// this layer's contribution is id-sorted briefings only.
    #[test]
    fn timeline_sort_order_contract() {
        let root = temp_root("ordering");
        let out = PackWriter::new(root.clone());
        out.write_json(
            "sprites/cinematics/timeline.json",
            &json!({
                "schema_version": "1.2.0",
                "sequences": [
                    {"id": "zz_narrative", "frames": [], "scene_type": "narrative"},
                    {"id": "aa_narrative", "frames": [], "scene_type": "narrative"},
                ],
                "campaigns": [],
                "supporting_assets": [],
                "notes": [],
            }),
        )
        .expect("seed baseline timeline");
        let result = generate(&out, DEFAULT_SEED).expect("generate");

        // Merge like the builder does: replace by id, then sort by
        // (scene_type == "briefing", id).
        let timeline = result.timeline.as_ref().expect("timeline");
        let mut merged: Vec<(String, Value)> = Vec::new();
        let layer_ids: Vec<String> = timeline
            .sequences
            .iter()
            .filter_map(|s| s["id"].as_str().map(str::to_string))
            .collect();
        for s in out
            .read_json("sprites/cinematics/timeline.json")
            .expect("read timeline")
            .expect("baseline")["sequences"]
            .as_array()
            .expect("sequences")
            .clone()
        {
            let id = s["id"].as_str().unwrap_or_default().to_string();
            if !layer_ids.contains(&id) {
                merged.push((id, s));
            }
        }
        for s in &timeline.sequences {
            merged.push((s["id"].as_str().unwrap_or_default().to_string(), s.clone()));
        }
        merged.sort_by(|a, b| {
            let key = |item: &(String, Value)| {
                (
                    item.1["scene_type"].as_str() == Some("briefing"),
                    item.0.clone(),
                )
            };
            key(a).cmp(&key(b))
        });
        let ids: Vec<&str> = merged
            .iter()
            .filter_map(|(_, s)| s["id"].as_str())
            .collect();
        assert_eq!(
            ids,
            vec!["aa_narrative", "zz_narrative"]
                .into_iter()
                .chain(BRIEFINGS.iter().map(|spec| spec.0))
                .collect::<Vec<_>>()
        );
        cleanup(&root);
    }
}
