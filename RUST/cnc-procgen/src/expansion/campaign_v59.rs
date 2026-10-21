//! Port of `campaign_expansion_v5_9.py` (v6.2.1 asset pack, v5.9.0) — twelve
//! campaign cinematic scenes in three narrative acts, storyboards, reusable
//! prop sprites, character portraits, act key-art cards and prop atlas.
//!
//! The Python file is the contract: this port keeps its structure,
//! decomposition, ids, file paths, seed derivation, palette, draw order and
//! draw counts line-for-line. Public seam functions (`base_scene`,
//! `render_scene`, `portrait`, `art_sprite`) mirror the Python free
//! functions so later layers (campaign_expansion_v6_0.py) can code against
//! them exactly like the Python chain does.
//!
//! Documented deviations (accepted, not "fixed"):
//! - SplitMix64 [`Rng`] instead of Python MT19937: identical draw counts,
//!   order and ranges, but different values, so pixels differ from the
//!   Python pack while remaining deterministic in Rust.
//! - [`Canvas`] rasterizer instead of Pillow (line width uses a square
//!   stamp; PNG polylines with more than two points are drawn as
//!   consecutive two-point segments with the same stroke width; the
//!   storyboard thumbnail `paste` is an alpha composite over an opaque
//!   background — pixel-equivalent).
//! - Python `round()` maps to `round_ties_even` (Pillow's banker rounding).
//! - Fully opaque RGBA canvases (the portraits) encode as RGB PNGs, matching
//!   what the encoder picks; pixel content is unaffected and the manifest
//!   records stay self-consistent.
//! - serde_json key order (alphabetical) replaces Python's insertion order.

use std::collections::HashSet;

use serde_json::{json, Map, Value};

use super::{compose_atlas, sha256_hex, LayerOutput, PackWriter, SpriteSpec, TimelineContribution};
use crate::prng::{Rng, Seed};
use crate::tileset::canvas::Canvas;
use crate::tileset::pack::PackError;
use crate::tileset::{Rgba, TileRngExt};

/// Python `generate(..., seed=59_017)` default seed (independent of the
/// pack's `--seed`, exactly as the script was run to build the real pack).
pub const DEFAULT_SEED: u64 = 59_017;

/// Python `VERSION` (`report["asset_pack_version"]` / `generator_version`).
pub const VERSION: &str = "5.9.0";

/// Python `generated_by` stamp on every registered sprite record.
const GENERATED_BY: &str = "campaign_expansion_v5_9.py";

// Python `W, H, SCALE = 160, 90, 2`; `SIZE = (320, 180)`.
const W: i32 = 160;
const H: i32 = 90;
const SCALE: u32 = 2;
const SIZE: (u32, u32) = (W as u32 * SCALE, H as u32 * SCALE);

// Python palette dict `C = {...}` verbatim (alpha 255 added).
const C_SKY: Rgba = [20, 30, 42, 255];
#[allow(dead_code)]
const C_SKY2: Rgba = [57, 66, 70, 255];
const C_GROUND: Rgba = [53, 61, 49, 255];
const C_DARK: Rgba = [14, 20, 23, 255];
const C_STEEL: Rgba = [87, 103, 104, 255];
const C_STEEL_HI: Rgba = [155, 163, 146, 255];
const C_OLIVE: Rgba = [91, 107, 62, 255];
const C_OLIVE_HI: Rgba = [151, 157, 88, 255];
const C_RED: Rgba = [179, 61, 45, 255];
const C_ORANGE: Rgba = [239, 142, 61, 255];
const C_YELLOW: Rgba = [243, 207, 113, 255];
const C_WATER: Rgba = [27, 65, 80, 255];
const C_FOAM: Rgba = [105, 149, 150, 255];
const C_WHITE: Rgba = [222, 221, 199, 255];
const C_SKIN: Rgba = [178, 132, 94, 255];
const C_PANEL: Rgba = [12, 19, 25, 255];
const C_GREEN: Rgba = [107, 151, 99, 255];
const C_SAND: Rgba = [132, 111, 74, 255];
const C_SAND_HI: Rgba = [176, 146, 93, 255];
#[allow(dead_code)]
const C_SNOW: Rgba = [179, 195, 194, 255];
#[allow(dead_code)]
const C_ICE: Rgba = [98, 133, 143, 255];
const C_TREE: Rgba = [30, 51, 44, 255];
const C_TREE_HI: Rgba = [50, 74, 51, 255];
const C_BRICK: Rgba = [95, 69, 58, 255];
const C_CONCRETE: Rgba = [86, 89, 83, 255];
const C_BLACK: Rgba = [4, 7, 9, 255];

/// `(id, display title, act, environment, motif, captions)` — the Python
/// `SCENES` list verbatim.
#[allow(clippy::type_complexity)] // shape mirrors the Python tuple rows
const SCENES: [(&str, &str, &str, &str, &str, [&str; 8]); 12] = [
    (
        "07_border_prologue",
        "Prologue: The Border",
        "Act I — The Frontline",
        "dawn",
        "border",
        [
            "The border stations have gone silent",
            "Recon spots movement along the ridge",
            "Command requests visual confirmation",
            "A convoy appears beyond the checkpoint",
            "The radio fills with static",
            "All units take defensive positions",
            "The first shots echo across the valley",
            "The campaign begins",
        ],
    ),
    (
        "08_sandstorm_convoy",
        "Sandstorm Convoy",
        "Act I — The Frontline",
        "desert",
        "convoy",
        [
            "A supply convoy is overdue",
            "Visibility drops in the dunes",
            "Scouts find tracks in the sand",
            "The lead truck takes fire",
            "Escort armor moves to the flank",
            "A buried roadblock is exposed",
            "The convoy pushes through the storm",
            "Supplies reach the forward line",
        ],
    ),
    (
        "09_black_forest_patrol",
        "Black Forest Patrol",
        "Act I — The Frontline",
        "forest",
        "patrol",
        [
            "Patrol enters the old forest road",
            "Movement detected between the trees",
            "Keep spacing and hold radio silence",
            "A disabled scout vehicle blocks the path",
            "Infantry clears the roadside",
            "Armor advances through the smoke",
            "The treeline is secure for now",
            "A signal flare marks the route",
        ],
    ),
    (
        "10_bridgehead",
        "Bridgehead",
        "Act I — The Frontline",
        "river",
        "bridge",
        [
            "The river separates the two fronts",
            "Engineers inspect the damaged bridge",
            "Smoke covers the far bank",
            "The first vehicle rolls onto the span",
            "Enemy fire strikes the supports",
            "Infantry secures the bridgehead",
            "Reinforcements cross under cover",
            "The route inland is open",
        ],
    ),
    (
        "11_iron_city",
        "The Iron City",
        "Act II — Behind the Lines",
        "city",
        "urban",
        [
            "Industrial district is still occupied",
            "Recon maps the factory perimeter",
            "Civilians shelter below the streets",
            "A barricade seals the avenue",
            "Armor breaks through the outer wall",
            "The signal tower goes dark",
            "The depot is secured",
            "The city grid is under our control",
        ],
    ),
    (
        "12_midnight_signal",
        "Midnight Signal",
        "Act II — Behind the Lines",
        "night",
        "raid",
        [
            "A coded signal flashes from the ridge",
            "A small team moves beyond the wire",
            "Searchlights sweep the valley",
            "Charges are placed at the relay",
            "The alarm sounds too early",
            "Flares illuminate the escape route",
            "The transmitter falls silent",
            "Extraction is inbound",
        ],
    ),
    (
        "13_siege_breaker",
        "Siege Breaker",
        "Act II — Behind the Lines",
        "storm",
        "siege",
        [
            "The garrison is running out of fuel",
            "Fortified guns dominate the approach",
            "Artillery marks the concrete works",
            "Smoke screens the advancing armor",
            "Engineers breach the outer barricade",
            "The bunker doors are forced open",
            "The siege line begins to collapse",
            "The garrison can move again",
        ],
    ),
    (
        "14_supply_lines",
        "The Lifeline",
        "Act II — Behind the Lines",
        "dusk",
        "supply",
        [
            "Fuel stocks have reached critical levels",
            "A depot is found beyond the rail yard",
            "Repair crews work through the night",
            "A convoy is assembled at dawn",
            "The road comes under attack",
            "An escort blocks the ambush",
            "The depot radios report delivery",
            "The front line can hold",
        ],
    ),
    (
        "15_storm_front",
        "Storm Front",
        "Act II — Behind the Lines",
        "rain",
        "weather",
        [
            "A storm rolls across the plateau",
            "Radar loses contact with the patrol",
            "Mud slows the heavy vehicles",
            "Lightning reveals movement ahead",
            "The column turns off the main road",
            "A bridge is washed out",
            "A new route appears on the map",
            "The operation continues in the rain",
        ],
    ),
    (
        "16_zero_hour_airfield",
        "Zero Hour at the Airfield",
        "Act III — The Turning Point",
        "dawn",
        "airfield",
        [
            "The airfield is the last enemy relay",
            "Ground crews prepare the runway",
            "Anti-aircraft guns track the horizon",
            "A transport lifts through the smoke",
            "Armor advances along the runway",
            "The control tower is evacuated",
            "The beacon changes to friendly code",
            "Air support is back online",
        ],
    ),
    (
        "17_final_assault",
        "Final Assault",
        "Act III — The Turning Point",
        "sunset",
        "assault",
        [
            "All remaining units converge on the fortress",
            "The enemy activates its outer defenses",
            "Our flanks move into position",
            "Artillery opens a corridor",
            "The main gate gives way",
            "Command orders the final advance",
            "The enemy banner comes down",
            "The fortress is secure",
        ],
    ),
    (
        "18_frontier_epilogue",
        "Beyond the Frontier",
        "Act III — The Turning Point",
        "dawn",
        "epilogue",
        [
            "The guns fall silent across the valley",
            "Engineers reopen the supply road",
            "Medics return to the forward post",
            "Scouts reach the distant ridge",
            "A new settlement appears on the horizon",
            "The command map extends east",
            "The radio carries a message of hope",
            "The next chapter awaits",
        ],
    ),
];

/// Timeline `campaigns` — `(campaign id, act title)` paired with the
/// matching `SCENES` act strings.
const CAMPAIGNS: [(&str, &str); 3] = [
    ("act_1_frontline", "Act I — The Frontline"),
    ("act_2_behind_the_lines", "Act II — Behind the Lines"),
    ("act_3_turning_point", "Act III — The Turning Point"),
];

/// The 22 reusable prop sprite names (Python `prop_names`).
const PROP_NAMES: [&str; 22] = [
    "supply_crate",
    "radar_dish",
    "bridge_segment",
    "sandbag_wall",
    "pine_tree",
    "burned_tree",
    "ruined_building",
    "field_hospital",
    "factory_stack",
    "bunker",
    "artillery_piece",
    "flak_gun",
    "fighter_plane",
    "transport_plane",
    "scout_car",
    "command_truck",
    "supply_truck",
    "ambulance",
    "flag_marker",
    "objective_beacon",
    "medic_bag",
    "radio_pack",
];

/// The 6 character portraits (Python `portrait_names`).
const PORTRAIT_NAMES: [&str; 6] = [
    "sniper",
    "pilot",
    "logistics_officer",
    "resistance_contact",
    "field_captain",
    "radio_operator",
];

/// Chapter key art: `(card id, source sequence, accent)` — one text-free
/// title card per campaign act (Python `card_specs`).
const CARD_SPECS: [(&str, &str, Rgba); 3] = [
    ("act_1_frontline", "07_border_prologue", C_ORANGE),
    ("act_2_behind_the_lines", "11_iron_city", C_RED),
    ("act_3_turning_point", "17_final_assault", C_YELLOW),
];

/// Free-form notes appended to the timeline (Python `new_notes`).
const NEW_NOTES: [&str; 5] = [
    "v5.9 adds 12 campaign scenes in three narrative acts; earlier cinematic sequences are retained.",
    "Each scene has 8 deterministic 320x180 frames, camera treatment metadata, caption metadata, and cue labels.",
    "Campaign props and portrait cards are standalone reusable PNG sprites.",
    "Audio cues are metadata labels; no audio files are bundled.",
    "Procedural generated art requires art review and engine-side playback validation before production release.",
];

/// Python `round()` (banker's rounding) on an `f64`.
fn py_round(v: f64) -> i32 {
    v.round_ties_even() as i32
}

/// Opaque RGBA from an RGB triple literal.
fn rgb(r: u8, g: u8, b: u8) -> Rgba {
    [r, g, b, 255]
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

/// `d.rectangle(bbox, outline=...)` (no fill).
#[allow(clippy::too_many_arguments)]
fn d_rect_border(d: &mut Canvas, x0: i32, y0: i32, x1: i32, y1: i32, outline: Rgba, width: u32) {
    d.rect(x0, y0, x1, y1, None, Some(outline), width);
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

/// `d.line((pt, pt, ...), fill=...)` — polyline via consecutive segments.
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

// ---------------------------------------------------------------------------
// Python function ports
// ---------------------------------------------------------------------------

/// Port of `base_scene(seed, env, frame)` — 160x90 background: sky gradient,
/// environment silhouette, drifting smoke columns and weather particles.
pub fn base_scene(seed: u64, env: &str, frame: usize) -> Canvas {
    let mut rng = Rng::new(Seed::new(seed));
    let mut im = Canvas::new_filled(W as u32, H as u32, C_SKY);
    // Sky palette and gradient bands
    let sky_top: [u8; 3] = if env == "night" || env == "storm" || env == "rain" {
        [9, 16, 29]
    } else {
        [42, 48, 49]
    };
    let mut sky_low: [u8; 3] = if env == "dawn" || env == "sunset" || env == "dusk" {
        [76, 79, 70]
    } else {
        [47, 61, 64]
    };
    if env == "desert" {
        sky_low = [111, 92, 65];
    }
    if env == "snow" {
        sky_low = [95, 117, 128];
    }
    for y in 0..H {
        let col: Rgba = if y < 59 {
            let t = y as f64 / 59.0;
            rgb(
                py_round(sky_top[0] as f64 * (1.0 - t) + sky_low[0] as f64 * t) as u8,
                py_round(sky_top[1] as f64 * (1.0 - t) + sky_low[1] as f64 * t) as u8,
                py_round(sky_top[2] as f64 * (1.0 - t) + sky_low[2] as f64 * t) as u8,
            )
        } else {
            C_GROUND
        };
        d_line(&mut im, 0, y, W, y, col);
    }
    if env == "dawn" || env == "sunset" || env == "dusk" {
        let sun = if env != "sunset" {
            rgb(239, 150, 78)
        } else {
            rgb(201, 86, 52)
        };
        d_ellipse(&mut im, 112, 15, 139, 42, sun);
        d_rect(&mut im, 0, 44, W, 56, rgb(88, 68, 53));
    } else if env == "night" {
        d_ellipse(&mut im, 123, 9, 137, 23, rgb(192, 196, 173));
        for _ in 0..42 {
            let x = rng.randrange2(0, W);
            let y = rng.randrange2(0, 43);
            d_point(&mut im, x, y, rgb(178, 190, 181));
        }
    } else if env == "desert" {
        d_polygon(
            &mut im,
            &[
                (0, 57),
                (25, 43),
                (57, 58),
                (93, 39),
                (131, 57),
                (W, 44),
                (W, H),
                (0, H),
            ],
            rgb(111, 91, 61),
        );
        d_polygon(
            &mut im,
            &[
                (0, 67),
                (40, 57),
                (80, 67),
                (122, 55),
                (W, 66),
                (W, H),
                (0, H),
            ],
            C_SAND,
        );
    } else if env == "snow" {
        d_polygon(
            &mut im,
            &[
                (0, 58),
                (24, 35),
                (50, 58),
                (82, 31),
                (110, 58),
                (140, 39),
                (W, 58),
                (W, H),
                (0, H),
            ],
            rgb(84, 111, 122),
        );
        d_polygon(
            &mut im,
            &[
                (0, 64),
                (35, 58),
                (69, 66),
                (106, 58),
                (W, 64),
                (W, H),
                (0, H),
            ],
            C_SNOW,
        );
    } else if env == "forest" {
        let mut x = -4;
        while x < W + 8 {
            let ht = rng.randrange2(20, 37);
            d_polygon(&mut im, &[(x, 60), (x + 8, 60 - ht), (x + 16, 60)], C_TREE);
            d_polygon(
                &mut im,
                &[(x + 4, 50), (x + 8, 60 - ht), (x + 13, 51)],
                C_TREE_HI,
            );
            x += 17;
        }
        d_rect(&mut im, 0, 62, W, H, rgb(40, 53, 41));
    } else if env == "river" {
        d_polygon(&mut im, &[(0, 51), (W, 50), (W, 71), (0, 70)], C_WATER);
        for _ in 0..16 {
            let x = rng.randrange2(0, W);
            let y = rng.randrange2(52, 70);
            let len = rng.randrange2(4, 13);
            d_line(&mut im, x, y, x + len, y, C_FOAM);
        }
        d_polygon(
            &mut im,
            &[
                (0, 68),
                (32, 63),
                (60, 72),
                (92, 65),
                (W, 70),
                (W, H),
                (0, H),
            ],
            rgb(59, 68, 51),
        );
    } else if env == "city" {
        let mut x = 0;
        while x < W {
            let ht = rng.randrange2(16, 34);
            d_rect(&mut im, x, 57 - ht, x + 14, 61, C_CONCRETE);
            d_rect(&mut im, x + 3, 60 - ht, x + 5, 54, C_DARK);
            let window = if rng.chance(0.25) {
                rgb(180, 139, 72)
            } else {
                C_DARK
            };
            d_rect(&mut im, x + 9, 60 - ht, x + 11, 54, window);
            x += 19;
        }
        d_rect(&mut im, 0, 62, W, H, rgb(54, 54, 48));
    } else {
        let mut rngline: Vec<(i32, i32)> = Vec::new();
        let mut x = 0;
        while x <= W {
            rngline.push((x, 54 + rng.rand_i32(-5, 6)));
            x += 5;
        }
        let mut pts = vec![(0, 64)];
        pts.extend_from_slice(&rngline);
        pts.extend_from_slice(&[(W, 65), (W, H), (0, H)]);
        d_polygon(&mut im, &pts, rgb(38, 49, 47));
    }
    // smoke columns drift a few pixels each frame, but all randomness is seeded
    for j in 0..3usize {
        let x = 24 + j as i32 * 55 + (frame * 2 + j * 3) as i32 % 7;
        let y = 28 + j as i32 * 4;
        for k in 0..3usize {
            let r = 4 + k as i32 * 2;
            d_ellipse(
                &mut im,
                x - r,
                y - k as i32 * 6 - r,
                x + r,
                y - k as i32 * 6 + r,
                rgb(51 + k as u8 * 5, 55 + k as u8 * 5, 53 + k as u8 * 5),
            );
        }
    }
    if env == "rain" || env == "storm" {
        for _ in 0..65 {
            let x = rng.randrange2(0, W);
            let y = rng.randrange2(0, H);
            d_line(&mut im, x, y, x - 2, y + 5, rgb(87, 113, 125));
        }
        if env == "storm" && (frame == 2 || frame == 5) {
            d_polyline(
                &mut im,
                &[(118, 0), (109, 16), (118, 18), (106, 36)],
                rgb(207, 214, 187),
                2,
            );
        }
    }
    if env == "snow" {
        for _ in 0..40 {
            let x = rng.randrange2(0, W);
            let y = rng.randrange2(0, H);
            d_point(&mut im, x, y, rgb(210, 221, 216));
        }
    }
    if env == "desert" {
        for _ in 0..28 {
            let x = rng.randrange2(0, W);
            let y = rng.randrange2(44, 80);
            d_point(&mut im, x, y, rgb(172, 142, 91));
        }
    }
    im
}

/// Python `tank(d, x, y, damaged=False, scale=1)`.
fn tank(d: &mut Canvas, x: i32, y: i32, damaged: bool, scale: i32) {
    d.rounded_rect(x, y, x + 27 * scale, y + 9 * scale, 2, C_DARK);
    d.rounded_rect_outline(x, y, x + 27 * scale, y + 9 * scale, 2, C_STEEL_HI, 1);
    for i in 0..5 {
        let xx = x + (3 + i * 4) * scale;
        d_ellipse(d, xx, y + 2 * scale, xx + 3 * scale, y + 7 * scale, C_STEEL);
    }
    d_polygon_out(
        d,
        &[
            (x + 4 * scale, y),
            (x + 8 * scale, y - 5 * scale),
            (x + 20 * scale, y - 5 * scale),
            (x + 24 * scale, y),
        ],
        C_OLIVE,
        C_OLIVE_HI,
    );
    d_ellipse(
        d,
        x + 11 * scale,
        y - 9 * scale,
        x + 19 * scale,
        y - 3 * scale,
        C_OLIVE_HI,
    );
    d_line_w(
        d,
        x + 17 * scale,
        y - 7 * scale,
        x + 33 * scale,
        y - 7 * scale,
        C_STEEL_HI,
        scale.max(1) as u32,
    );
    if damaged {
        d_line_w(
            d,
            x + 5 * scale,
            y - 2 * scale,
            x + 12 * scale,
            y + 6 * scale,
            C_ORANGE,
            scale.max(1) as u32,
        );
        d_ellipse(
            d,
            x + 22 * scale,
            y - 13 * scale,
            x + 29 * scale,
            y - 8 * scale,
            rgb(88, 69, 54),
        );
    }
}

/// Python `soldier(d, x, y, phase=0, accent=None)`.
fn soldier(d: &mut Canvas, x: i32, y: i32, phase: usize, accent: Option<Rgba>) {
    d_ellipse(d, x + 2, y - 12, x + 7, y - 7, C_SKIN);
    d_rect(d, x + 1, y - 7, x + 8, y + 1, accent.unwrap_or(C_OLIVE));
    let lx = if phase % 2 == 1 { 1 } else { 5 };
    d_line(d, x + 3, y, x + lx, y + 6, C_DARK);
    let rx = if phase % 2 == 1 { 9 } else { 6 };
    d_line(d, x + 6, y, x + rx, y + 6, C_DARK);
    d_line(d, x + 6, y - 5, x + 13, y - 8, C_STEEL_HI);
}

/// Python `vehicle(d, x, y, kind="truck")`.
fn vehicle(d: &mut Canvas, x: i32, y: i32, kind: &str) {
    let body = if kind != "ambulance" {
        C_OLIVE
    } else {
        C_WHITE
    };
    d_rect_out(d, x, y - 8, x + 29, y, body, C_DARK);
    d_rect_out(d, x + 5, y - 14, x + 17, y - 8, body, C_DARK);
    d_rect(d, x + 7, y - 13, x + 11, y - 10, rgb(63, 91, 101));
    for xx in [x + 4, x + 22] {
        d_ellipse_out(d, xx, y - 3, xx + 6, y + 3, C_DARK, C_STEEL);
    }
    if kind == "supply" {
        d_rect_out(d, x + 18, y - 13, x + 28, y - 8, C_SAND_HI, C_DARK);
    } else if kind == "ambulance" {
        d_rect(d, x + 12, y - 7, x + 15, y - 1, C_RED);
        d_rect(d, x + 10, y - 5, x + 17, y - 3, C_RED);
    }
}

/// Python `building(d, x, y, style="ruin")`.
fn building(d: &mut Canvas, x: i32, y: i32, style: &str) {
    let col = if style == "ruin" { C_BRICK } else { C_CONCRETE };
    d_rect_out(d, x, y - 22, x + 25, y, col, C_DARK);
    d_rect(d, x + 3, y - 18, x + 8, y - 12, C_DARK);
    d_rect(d, x + 15, y - 18, x + 20, y - 12, C_DARK);
    d_rect(d, x + 5, y - 8, x + 10, y, C_DARK);
    if style == "ruin" {
        d_polygon(
            d,
            &[
                (x, y - 22),
                (x + 10, y - 29),
                (x + 17, y - 21),
                (x + 25, y - 25),
                (x + 25, y - 22),
            ],
            col,
        );
        d_polyline(
            d,
            &[(x + 14, y - 19), (x + 9, y - 10), (x + 17, y - 7)],
            rgb(124, 92, 69),
            1,
        );
    } else if style == "factory" {
        d_rect(d, x + 17, y - 34, x + 23, y - 22, C_STEEL);
        d_ellipse(d, x + 16, y - 40, x + 28, y - 31, rgb(62, 67, 64));
    }
}

/// Port of `motif(im, motif_name, frame, rng)` — horizon props layered
/// back-to-front plus the letterbox/subtitle chrome. The `rng` parameter is
/// kept for signature parity with the Python function, which creates it but
/// never draws from it (all motion keys off `frame` parity).
fn motif(_rng: &mut Rng, im: &mut Canvas, motif_name: &str, frame: usize) {
    if motif_name == "convoy" {
        for i in 0..3usize {
            let kind = if i == 1 { "supply" } else { "truck" };
            vehicle(
                im,
                10 + i as i32 * 48 + (frame % 3) as i32,
                70 - (i % 2) as i32 * 4,
                kind,
            );
        }
        for x in [32, 82, 132] {
            d_ellipse(im, x - 4, 48, x + 4, 56, rgb(130, 106, 74));
        }
    } else if motif_name == "patrol" {
        for x in [5, 28, 120, 142] {
            let peak = 30 + (x % 3) * 3;
            d_polygon(im, &[(x, 69), (x + 10, peak), (x + 21, 69)], C_TREE);
            d_polygon(im, &[(x + 5, 48), (x + 10, peak), (x + 16, 49)], C_TREE_HI);
        }
        tank(im, 16, 75, true, 1);
        for i in 0..5usize {
            soldier(im, 53 + i as i32 * 15, 74, frame + i, None);
        }
    } else if motif_name == "bridge" {
        // bridge deck and truss with piers
        d_rect_out(im, 0, 56, W, 62, C_CONCRETE, C_DARK);
        d_line_w(im, 0, 55, W, 55, C_STEEL_HI, 2);
        let mut x = 9;
        while x < W {
            d_line_w(im, x, 55, x + 9, 62, C_STEEL, 2);
            d_line_w(im, x + 9, 62, x + 18, 55, C_STEEL, 2);
            x += 18;
        }
        d_rect(im, 35, 62, 43, 74, C_CONCRETE);
        d_rect(im, 112, 62, 120, 74, C_CONCRETE);
        tank(im, 60 + (frame % 3) as i32, 54, false, 1);
        for i in 0..3usize {
            soldier(im, 25 + i as i32 * 10, 75, frame + i, None);
        }
    } else if motif_name == "urban" {
        building(im, 4, 73, "ruin");
        building(im, 43, 74, "factory");
        building(im, 112, 73, "ruin");
        tank(im, 68 + (frame % 3) as i32, 76, frame > 4, 1);
        d_rect(im, 0, 77, W, H, rgb(37, 42, 40));
        let mut x = 17;
        while x < 155 {
            d_line(im, x, 79, x + 8, 79, rgb(180, 157, 107));
            x += 24;
        }
    } else if motif_name == "raid" {
        d_rect(im, 0, 67, W, H, rgb(28, 39, 38));
        d_rect_out(im, 94, 32, 112, 68, C_DARK, C_STEEL);
        d_ellipse(im, 96, 25, 110, 39, C_STEEL);
        d_line_w(im, 103, 28, 103, 14, C_STEEL_HI, 2);
        for i in 0..6usize {
            soldier(
                im,
                16 + i as i32 * 22 + (frame % 2) as i32,
                75,
                frame + i,
                Some(rgb(59, 74, 71)),
            );
        }
        d_polygon(
            im,
            &[(100, 43), (105, 38), (110, 43), (105, 48)],
            if !frame.is_multiple_of(3) {
                C_RED
            } else {
                C_YELLOW
            },
        );
    } else if motif_name == "siege" {
        d_rect(im, 0, 67, W, H, rgb(39, 45, 40));
        d_rect_out(im, 95, 43, 153, 72, C_CONCRETE, C_DARK);
        d_polygon(im, &[(89, 43), (103, 34), (148, 34), (159, 43)], C_STEEL);
        d_rect(im, 111, 52, 137, 62, C_DARK);
        // artillery carriage
        d_ellipse_out(im, 28, 69, 40, 81, C_DARK, C_STEEL);
        d_ellipse_out(im, 46, 69, 58, 81, C_DARK, C_STEEL);
        d_rect(im, 31, 62, 52, 70, C_OLIVE);
        d_line_w(im, 45, 64, 72, 54, C_STEEL_HI, 3);
        tank(im, 8, 77, frame > 5, 1);
    } else if motif_name == "supply" {
        d_rect(im, 0, 70, W, H, rgb(45, 54, 43));
        d_rect_out(im, 13, 40, 67, 70, C_CONCRETE, C_DARK);
        d_rect(im, 20, 48, 31, 59, C_DARK);
        d_rect(im, 42, 48, 55, 59, C_DARK);
        for x in [75, 91, 107] {
            d_rect_out(im, x, 63, x + 10, 72, C_SAND_HI, C_DARK);
            d_line(im, x, 67, x + 10, 67, C_SAND);
        }
        vehicle(im, 112 + (frame % 3) as i32, 76, "supply");
    } else if motif_name == "weather" {
        tank(im, 13, 76, frame.is_multiple_of(4), 1);
        tank(im, 92 + (frame % 4) as i32, 73, false, 1);
        for x in [54, 71, 125] {
            d_ellipse(
                im,
                x - 4,
                44,
                x + 4,
                52,
                if !frame.is_multiple_of(2) {
                    C_ORANGE
                } else {
                    C_YELLOW
                },
            );
        }
        d_polygon(im, &[(20, 52), (28, 48), (36, 51), (30, 55)], C_STEEL);
    } else if motif_name == "airfield" {
        d_rect(im, 0, 68, W, H, rgb(44, 51, 49));
        d_polygon(
            im,
            &[(12, 83), (80, 64), (155, 79), (155, 85), (12, 90)],
            rgb(61, 66, 62),
        );
        let mut x = 30;
        while x < 151 {
            d_line(im, x, 80, x + 10, 77, C_WHITE);
            x += 24;
        }
        d_rect_out(im, 115, 40, 144, 67, C_CONCRETE, C_DARK);
        d_rect_out(im, 120, 47, 139, 60, rgb(49, 75, 80), C_STEEL);
        // small side-on aircraft
        let ax = 34 + (frame % 3) as i32 * 2;
        let ay = 61 - (frame % 2) as i32;
        d_polygon_out(
            im,
            &[
                (ax, ay),
                (ax + 15, ay - 3),
                (ax + 32, ay),
                (ax + 15, ay + 3),
            ],
            C_STEEL_HI,
            C_DARK,
        );
        d_polygon(
            im,
            &[
                (ax + 10, ay),
                (ax + 15, ay - 9),
                (ax + 20, ay),
                (ax + 15, ay + 2),
            ],
            C_STEEL,
        );
        d_line(im, ax + 25, ay, ax + 34, ay - 1, C_STEEL_HI);
        for i in 0..2usize {
            vehicle(im, 18 + i as i32 * 39, 77, "truck");
        }
    } else if motif_name == "assault" {
        d_rect(im, 0, 71, W, H, rgb(36, 43, 38));
        d_rect_out(im, 110, 40, 155, 72, C_CONCRETE, C_DARK);
        for x in [116, 132, 148] {
            d_polygon(im, &[(x, 40), (x + 5, 31), (x + 10, 40)], C_STEEL);
        }
        d_rect(im, 126, 52, 141, 71, C_DARK);
        for i in 0..4usize {
            tank(im, 5 + i as i32 * 37 + (frame % 2) as i32, 78, frame > 5, 1);
        }
        if frame >= 3 {
            for (x, y) in [(52, 51), (91, 45), (137, 56)] {
                let r = 3 + (frame % 3) as i32;
                d_ellipse(im, x - r, y - r, x + r, y + r, C_ORANGE);
                d_ellipse(im, x - 1, y - 1, x + 1, y + 1, C_YELLOW);
            }
        }
        d_line_w(im, 145, 31, 145, 14, C_STEEL_HI, 2);
        d_polygon(
            im,
            &[(146, 15), (159, 19), (146, 24)],
            if frame < 6 { C_RED } else { C_GREEN },
        );
    } else if motif_name == "epilogue" {
        d_rect(im, 0, 70, W, H, rgb(34, 49, 38));
        d_rect_out(im, 10, 50, 46, 73, C_CONCRETE, C_DARK);
        d_rect(im, 16, 56, 24, 65, rgb(30, 50, 56));
        d_rect(im, 32, 56, 40, 65, rgb(30, 50, 56));
        d_line_w(im, 0, 81, W, 74, C_SAND_HI, 3);
        for i in 0..3usize {
            soldier(im, 49 + i as i32 * 15, 77, frame + i, Some(C_OLIVE));
        }
        d_line_w(im, 129, 62, 129, 31, C_STEEL_HI, 2);
        d_polygon(im, &[(130, 32), (151, 38), (130, 44)], C_GREEN);
        // distant frontier settlement
        for x in [92, 101, 110] {
            d_rect(im, x, 57, x + 6, 65, C_SAND_HI);
        }
    } else {
        // border prologue
        building(im, 112, 70, "ruin");
        d_rect(im, 0, 72, W, H, rgb(43, 51, 42));
        tank(im, 18 + (frame % 3) as i32, 76, frame >= 3, 1);
        vehicle(im, 100, 76, "truck");
        for i in 0..3usize {
            soldier(im, 47 + i as i32 * 15, 76, frame + i, None);
        }
        d_line(im, 80, 51, 80, 70, C_STEEL_HI);
        d_polygon(im, &[(81, 51), (94, 55), (81, 59)], C_RED);
    }
    // smoke, explosions and small debris move with frame index
    if matches!(
        motif_name,
        "raid" | "siege" | "urban" | "assault" | "weather"
    ) && (frame == 2 || frame == 4 || frame == 6)
    {
        for k in 0..3usize {
            let x = (37 + frame * 13 + k * 6) as i32 % W;
            let y = 53 + k as i32 * 3;
            d_ellipse(
                im,
                x - 3 - k as i32,
                y - 5 - k as i32,
                x + 3 + k as i32,
                y + 2,
                rgb(69 + k as u8 * 5, 65 + k as u8 * 4, 57 + k as u8 * 4),
            );
        }
    }
    // Letterbox marks and subtitle placeholder, actual captions remain metadata.
    d_rect(im, 0, 0, W, 6, C_BLACK);
    d_rect(im, 0, H - 10, W, H, C_BLACK);
    d_rect(im, 6, 2, 25, 4, C_RED);
    d_rect(im, W - 25, 2, W - 6, 4, C_STEEL);
    let progress = ((W - 16) as f64 * (frame + 1) as f64 / 8.0) as i32;
    d_rect(im, 8, H - 5, 8 + progress, H - 4, C_ORANGE);
}

/// Port of `render_scene(seed, env, motif_name, frame)` — base scene plus
/// motif, then the camera treatment: slow pan + zoom crop, nearest-resample
/// to 160x90 and finally the 2x pixel-art upscale to 320x180.
pub fn render_scene(seed: u64, env: &str, motif_name: &str, frame: usize) -> Canvas {
    let mut im = base_scene(seed, env, frame);
    let mut rng = Rng::new(Seed::new(seed ^ (frame as u64 * 7919)));
    motif(&mut rng, &mut im, motif_name, frame);
    let zoom = 1.0 + 0.009 * frame as f64;
    let cw = (W as f64 / zoom) as i32;
    let ch = (H as f64 / zoom) as i32;
    let left = ((W - cw) as f64 * frame as f64 / 7.0) as i32;
    let top = ((H - ch) as f64 * 0.5) as i32;
    im.crop(left, top, cw as u32, ch as u32)
        .resize_nearest(W as u32, H as u32)
        .resize_nearest(SIZE.0, SIZE.1)
}

/// Port of `art_sprite(name, seed=59000)` — a standalone transparent 64x64
/// pixel-art prop sprite. Seed derivation is literal:
/// `seed + sum(ord(c) for c in name)`.
pub fn art_sprite(name: &str, seed: u64) -> Canvas {
    // Python creates this generator but never draws from it; kept for
    // contract parity.
    let _rng = Rng::new(Seed::new(
        seed + name.chars().map(|c| c as u64).sum::<u64>(),
    ));
    let mut im = Canvas::transparent(64, 64);
    let edge = C_DARK;
    if name == "supply_crate" || name == "cargo_crate" || name == "field_rations" {
        im.rect(9, 24, 55, 57, Some(C_SAND_HI), Some(edge), 2);
        d_line_w(&mut im, 9, 34, 55, 34, rgb(101, 79, 51), 2);
        d_line_w(&mut im, 23, 24, 23, 57, rgb(101, 79, 51), 2);
        d_line_w(&mut im, 9, 24, 55, 57, rgb(195, 161, 105), 2);
    } else if name == "radar_dish" || name == "signal_tower" {
        d_rect_out(&mut im, 29, 30, 35, 59, C_STEEL, edge);
        d_line_w(&mut im, 32, 30, 16, 18, C_STEEL_HI, 3);
        im.arc(10, 8, 40, 34, 190.0, 340.0, C_GREEN, 2);
        d_ellipse_out(&mut im, 12, 11, 21, 20, C_STEEL_HI, edge);
        d_rect_out(&mut im, 24, 57, 40, 61, C_CONCRETE, edge);
    } else if name == "bridge_segment" || name == "bridge_deck" {
        im.rect(2, 34, 62, 44, Some(C_CONCRETE), Some(edge), 2);
        let mut x = 4;
        while x < 60 {
            d_line_w(&mut im, x, 34, x + 8, 44, C_STEEL, 2);
            x += 9;
        }
        d_rect_out(&mut im, 8, 44, 15, 59, C_CONCRETE, edge);
        d_rect_out(&mut im, 49, 44, 56, 59, C_CONCRETE, edge);
    } else if name == "sandbag_wall" || name == "barricade" {
        for row in 0..3usize {
            for col in 0..5usize {
                let x = 4 + col as i32 * 11 + (row % 2) as i32 * 4;
                let y = 24 + row as i32 * 10;
                im.rounded_rect(x, y, x + 12, y + 10, 2, rgb(116, 105, 74));
                im.rounded_rect_outline(x, y, x + 12, y + 10, 2, edge, 1);
                d_line(&mut im, x + 3, y + 3, x + 8, y + 3, rgb(158, 143, 99));
            }
        }
    } else if name == "pine_tree" || name == "burned_tree" {
        let col = if name == "pine_tree" {
            C_TREE
        } else {
            rgb(54, 43, 37)
        };
        d_rect_out(&mut im, 29, 43, 35, 62, rgb(75, 57, 40), edge);
        for (y, ww) in [(8, 16), (21, 22), (34, 28)] {
            d_polygon_out(
                &mut im,
                &[(32, y), (32 - ww, y + 21), (32 + ww, y + 21)],
                col,
                edge,
            );
        }
        if name == "burned_tree" {
            d_polyline(&mut im, &[(30, 47), (40, 35), (34, 26)], C_ORANGE, 2);
        }
    } else if name == "ruined_building" || name == "field_hospital" || name == "factory_stack" {
        let col = if name == "ruined_building" {
            C_BRICK
        } else {
            C_CONCRETE
        };
        im.rect(5, 22, 59, 58, Some(col), Some(edge), 2);
        d_rect(&mut im, 13, 31, 23, 42, C_DARK);
        d_rect(&mut im, 39, 31, 49, 42, C_DARK);
        d_rect(&mut im, 27, 43, 38, 58, C_DARK);
        if name == "ruined_building" {
            d_polygon_out(
                &mut im,
                &[(5, 22), (18, 11), (31, 22), (43, 14), (59, 22)],
                col,
                edge,
            );
        }
        if name == "field_hospital" {
            d_rect(&mut im, 28, 28, 36, 42, C_WHITE);
            d_rect(&mut im, 24, 32, 40, 38, C_WHITE);
        }
        if name == "factory_stack" {
            im.rect(43, 5, 52, 22, Some(C_STEEL), Some(edge), 1);
        }
    } else if name == "bunker" || name == "fuel_depot" {
        im.rect(6, 30, 58, 58, Some(C_CONCRETE), Some(edge), 2);
        d_polygon_out(
            &mut im,
            &[(3, 31), (15, 16), (49, 16), (61, 31)],
            C_STEEL,
            edge,
        );
        d_rect(&mut im, 22, 38, 42, 47, C_DARK);
        if name == "fuel_depot" {
            im.rect(17, 8, 26, 24, Some(C_RED), Some(edge), 1);
            im.rect(38, 8, 47, 24, Some(C_RED), Some(edge), 1);
        }
    } else if name == "artillery_piece" || name == "flak_gun" {
        d_ellipse_out(&mut im, 7, 42, 23, 58, C_DARK, C_STEEL);
        d_ellipse_out(&mut im, 38, 42, 54, 58, C_DARK, C_STEEL);
        d_rect_out(&mut im, 13, 34, 47, 45, C_OLIVE, edge);
        d_line_w(&mut im, 35, 36, 59, 17, C_STEEL_HI, 5);
        if name == "flak_gun" {
            d_line_w(&mut im, 34, 37, 50, 12, C_STEEL_HI, 3);
        }
    } else if name == "fighter_plane" || name == "transport_plane" {
        d_polygon_out(
            &mut im,
            &[
                (4, 31),
                (24, 27),
                (39, 9),
                (47, 11),
                (42, 28),
                (60, 31),
                (60, 36),
                (42, 39),
                (47, 53),
                (39, 55),
                (24, 38),
                (4, 35),
            ],
            C_STEEL_HI,
            edge,
        );
        d_line_w(&mut im, 12, 32, 53, 32, C_STEEL, 2);
        if name == "transport_plane" {
            d_rect(&mut im, 18, 28, 39, 38, C_OLIVE);
        }
    } else if name == "scout_car"
        || name == "command_truck"
        || name == "supply_truck"
        || name == "ambulance"
    {
        let kind = if name == "ambulance" {
            "ambulance"
        } else if name == "supply_truck" {
            "supply"
        } else {
            "truck"
        };
        vehicle(&mut im, 14, 45, kind);
        if name == "command_truck" {
            d_rect(&mut im, 30, 27, 36, 34, C_GREEN);
        }
    } else if name == "flag_marker" || name == "objective_beacon" {
        d_rect_out(&mut im, 18, 15, 22, 59, C_STEEL_HI, edge);
        d_polygon_out(
            &mut im,
            &[(23, 16), (53, 22), (23, 31)],
            if name == "flag_marker" {
                C_RED
            } else {
                C_GREEN
            },
            edge,
        );
        d_ellipse_out(&mut im, 14, 56, 28, 61, C_CONCRETE, edge);
        if name == "objective_beacon" {
            d_ellipse(&mut im, 18, 3, 28, 13, C_YELLOW);
        }
    } else if name == "medic_bag" || name == "radio_pack" {
        let col = if name == "medic_bag" { C_RED } else { C_OLIVE };
        im.rounded_rect(14, 20, 50, 53, 5, col);
        im.rounded_rect_outline(14, 20, 50, 53, 5, edge, 2);
        if name == "medic_bag" {
            d_rect(&mut im, 29, 28, 35, 45, C_WHITE);
            d_rect(&mut im, 23, 34, 41, 40, C_WHITE);
        } else {
            d_rect_out(&mut im, 22, 13, 42, 24, C_STEEL, edge);
            d_line_w(&mut im, 32, 14, 43, 5, C_STEEL_HI, 2);
        }
    } else {
        // Safe visible fallback to keep extension data-driven.
        im.rect(12, 16, 52, 56, Some(C_OLIVE), Some(edge), 2);
        d_line_w(&mut im, 12, 16, 52, 56, C_OLIVE_HI, 2);
    }
    im
}

/// Port of `portrait(name, index)` — a 96x96 character portrait card with
/// palette-coded accent, helmet, role plate and campaign dot (no baked-in
/// letters).
pub fn portrait(name: &str, index: usize) -> Canvas {
    let mut im = Canvas::new_filled(96, 96, rgb(18, 26, 29));
    let accents = [
        C_OLIVE, C_STEEL, C_WHITE, C_ORANGE, C_GREEN, C_BRICK, C_SAND_HI, C_RED,
    ];
    let accent = accents[index % accents.len()];
    d_rect_border(&mut im, 5, 5, 90, 90, C_STEEL_HI, 2);
    d_rect_border(&mut im, 10, 10, 86, 86, C_DARK, 1);
    d_ellipse_out(&mut im, 31, 18, 64, 53, C_SKIN, C_DARK);
    d_polygon(
        &mut im,
        &[(25, 27), (30, 12), (64, 12), (70, 29), (61, 22), (34, 22)],
        if index.is_multiple_of(2) {
            C_OLIVE_HI
        } else {
            C_STEEL
        },
    );
    d_polygon(
        &mut im,
        &[(18, 88), (22, 59), (39, 51), (57, 51), (74, 59), (79, 88)],
        accent,
    );
    // small role emblem, no baked-in letters
    d_rect(&mut im, 12, 73, 84, 84, C_PANEL);
    d_rect(
        &mut im,
        18,
        77,
        18 + (name.chars().count() * 5).min(48) as i32,
        79,
        C_WHITE,
    );
    d_ellipse(
        &mut im,
        71,
        13,
        81,
        23,
        if index.is_multiple_of(3) {
            C_RED
        } else {
            C_GREEN
        },
    );
    im
}

/// Insert or replace a JSON list member by its `"id"` string (the Python
/// scripts' `by_id` / `register` merge semantics).
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
    let num = if !prefix.is_empty() && prefix.bytes().all(|b| b.is_ascii_digit()) {
        prefix.parse().unwrap_or(999)
    } else {
        999
    };
    (num, id.to_string())
}

/// Generate this layer into `out` and return its contributions, matching the
/// Python `generate(asset_root, seed=59017)` write set:
/// 96 cinematic frames + 12 storyboards + 22 props + 6 portraits + 3 key-art
/// cards + 1 atlas, the atlas mapping JSON, the merged timeline and manifest,
/// and the pack-root build report.
pub fn generate(out: &PackWriter, seed: u64) -> Result<LayerOutput, PackError> {
    let cut = "sprites/cinematics";
    let frames_root = format!("{cut}/frames");
    let preview = format!("{cut}/previews");
    let art_root = "sprites/campaign_art";
    let ui = "sprites/cinematic_ui";
    let cards_root = format!("{cut}/campaign_cards");
    let timeline_path = format!("{cut}/timeline.json");
    let manifest_path = "manifest.json";

    let mut timeline = out.read_json(&timeline_path)?.unwrap_or_else(|| {
        json!({
            "schema_version": "1.0.0",
            "seed": seed,
            "canvas": [SIZE.0, SIZE.1],
            "pixel_art_scale": SCALE,
            "sequences": [],
            "supporting_assets": [],
            "notes": [],
        })
    });
    let existing_sequences: Vec<Value> = timeline
        .get("sequences")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let old_ids: HashSet<String> = existing_sequences
        .iter()
        .filter_map(|s| s.get("id").and_then(Value::as_str).map(str::to_string))
        .collect();
    let scene_ids: HashSet<&str> = SCENES.iter().map(|(id, ..)| *id).collect();

    let mut output = LayerOutput::default();
    // Rendered frames per sequence, reused for storyboard thumbnails and the
    // key-art source frames (the Python scripts re-open the saved PNGs).
    let mut rendered: Vec<(&str, Vec<Canvas>)> = Vec::with_capacity(SCENES.len());
    let mut seq_list: Vec<Value> = existing_sequences;
    let mut new_sequences: Vec<Value> = Vec::new();
    // Every generated PNG path, in Python `generated` order.
    let mut generated: Vec<String> = Vec::new();
    // Scene sprite records are buffered: the shipped pack registers this
    // layer's reusable art (props, portraits, cards, atlas) BEFORE the
    // scene frames/storyboards, even though the archived generator script
    // renders the scenes first — the shipped manifest is the parity target.
    let mut scene_records: Vec<Value> = Vec::new();

    for (si, (seq_id, title, act, env, motif_name, captions)) in SCENES.iter().enumerate() {
        let mut frame_records: Vec<Value> = Vec::new();
        let mut frames: Vec<Canvas> = Vec::with_capacity(captions.len());
        for (fi, caption) in captions.iter().enumerate() {
            let frame_seed = seed + si as u64 * 2029 + fi as u64 * 131;
            let rel = format!("{frames_root}/{seq_id}/frame_{fi:03}.png");
            let im = render_scene(frame_seed, env, motif_name, fi);
            let record = out.write_sprite(
                &rel,
                &im,
                SpriteSpec {
                    category: "cinematic",
                    generated_by: Some(GENERATED_BY),
                    seed: Some(seed),
                    extra: vec![("art_role", json!("campaign_cinematic_frame"))],
                    ..SpriteSpec::default()
                },
            )?;
            generated.push(rel.clone());
            frames.push(im);
            scene_records.push(record.clone());
            frame_records.push(json!({
                "index": fi,
                "file": rel,
                "duration_ms": if fi < 7 { 900 } else { 1500 },
                "caption": caption,
                "seed": frame_seed,
                "sha256": record.get("sha256_file").cloned().unwrap_or(Value::Null),
            }));
        }
        // Storyboard: 4 columns x 2 rows, each tile uses a 160x90 pixel-art thumbnail.
        let mut sheet = Canvas::new_filled(640, 240, rgb(7, 12, 16));
        for (i, _fr) in frame_records.iter().enumerate() {
            let (col, row) = (i % 4, i / 4);
            let (x, y) = (col as i32 * 160, row as i32 * 120);
            d_rect(&mut sheet, x, y, x + 159, y + 11, rgb(12, 20, 25));
            d_rect(&mut sheet, x, y + 102, x + 159, y + 119, rgb(12, 20, 25));
            let progress = ((145 * (i + 1)) as f64 / 8.0) as i32;
            d_line_w(
                &mut sheet,
                x + 5,
                y + 106,
                x + 5 + progress,
                y + 106,
                C_ORANGE,
                2,
            );
            d_rect_border(&mut sheet, x + 151, y + 104, x + 155, y + 113, C_STEEL, 1);
            let thumb = frames[i].resize_nearest(160, 90);
            sheet.alpha_composite(&thumb, (x, y + 12));
        }
        let board_rel = format!("{preview}/{seq_id}_storyboard.png");
        let board_record = out.write_sprite(
            &board_rel,
            &sheet,
            SpriteSpec {
                category: "cinematic_preview",
                generated_by: Some(GENERATED_BY),
                seed: Some(seed),
                extra: vec![("art_role", json!("campaign_storyboard"))],
                ..SpriteSpec::default()
            },
        )?;
        generated.push(board_rel);
        scene_records.push(board_record);

        let seq = json!({
            "id": seq_id,
            "title": title,
            "campaign_act": act,
            "fps": 10,
            "loop": false,
            "canvas": [SIZE.0, SIZE.1],
            "environment": env,
            "motif": motif_name,
            "frames": frame_records,
            "transitions": {"in": "fade_from_black", "out": "fade_to_black", "camera": "slow_pan_zoom"},
            "audio_cues": ["ambience", "radio_or_combat", "chapter_stinger"],
            "captions_are_metadata_only": true,
        });
        upsert_by_id(&mut seq_list, seq.clone());
        new_sequences.push(seq);
        rendered.push((seq_id, frames));
    }

    // Twelve campaign props as independently reusable sprites.
    for name in PROP_NAMES {
        let rel = format!("{art_root}/{name}.png");
        let art = art_sprite(name, seed);
        let record = out.write_sprite(
            &rel,
            &art,
            SpriteSpec {
                category: "campaign_art",
                generated_by: Some(GENERATED_BY),
                seed: Some(seed),
                extra: vec![("art_role", json!("reusable_prop"))],
                ..SpriteSpec::default()
            },
        )?;
        generated.push(rel);
        output.sprites.push(record);
    }

    let mut prop_canvases: Vec<Canvas> = Vec::with_capacity(PROP_NAMES.len());
    for name in PROP_NAMES {
        prop_canvases.push(art_sprite(name, seed));
    }

    let new_portrait_count = PORTRAIT_NAMES.len();
    for (i, name) in PORTRAIT_NAMES.iter().enumerate() {
        let rel = format!("{ui}/portrait_{name}.png");
        let art = portrait(name, i + 8);
        let record = out.write_sprite(
            &rel,
            &art,
            SpriteSpec {
                category: "cinematic_ui",
                generated_by: Some(GENERATED_BY),
                seed: Some(seed),
                extra: vec![("art_role", json!("character_portrait"))],
                ..SpriteSpec::default()
            },
        )?;
        generated.push(rel);
        output.sprites.push(record);
    }

    // Chapter key art: one distinctive, text-free title card per campaign act.
    for (card_id, source_id, accent) in CARD_SPECS {
        let frames = rendered
            .iter()
            .find(|(sid, _)| *sid == source_id)
            .map(|(_, frames)| frames)
            .expect("card source sequence was rendered");
        let mut card = frames[6].clone();
        d_rect(&mut card, 0, 0, SIZE.0 as i32, 12, [5, 8, 10, 230]);
        d_rect(
            &mut card,
            0,
            SIZE.1 as i32 - 30,
            SIZE.0 as i32,
            SIZE.1 as i32,
            [5, 8, 10, 232],
        );
        d_rect(
            &mut card,
            9,
            SIZE.1 as i32 - 23,
            16,
            SIZE.1 as i32 - 7,
            accent,
        );
        d_rect_border(
            &mut card,
            23,
            SIZE.1 as i32 - 23,
            34,
            SIZE.1 as i32 - 7,
            accent,
            2,
        );
        d_line_w(
            &mut card,
            43,
            SIZE.1 as i32 - 18,
            SIZE.0 as i32 - 12,
            SIZE.1 as i32 - 18,
            accent,
            2,
        );
        d_line_w(
            &mut card,
            43,
            SIZE.1 as i32 - 11,
            SIZE.0 as i32 - 45,
            SIZE.1 as i32 - 11,
            rgb(124, 137, 130),
            1,
        );
        let card_rel = format!("{cards_root}/{card_id}.png");
        let record = out.write_sprite(
            &card_rel,
            &card,
            SpriteSpec {
                category: "campaign_art",
                generated_by: Some(GENERATED_BY),
                seed: Some(seed),
                extra: vec![
                    ("art_role", json!("campaign_act_key_art")),
                    ("campaign_act", json!(card_id)),
                ],
                ..SpriteSpec::default()
            },
        )?;
        generated.push(card_rel);
        output.sprites.push(record);
    }

    // Pack reusable props into a texture atlas and emit an engine-friendly rect map.
    let atlas_cols = 6usize;
    let atlas_rows = PROP_NAMES.len().div_ceil(atlas_cols);
    let cells: Vec<&Canvas> = prop_canvases.iter().collect();
    let atlas = compose_atlas(&cells, atlas_cols, 64, false);
    let mut atlas_rects = Map::new();
    for (i, name) in PROP_NAMES.iter().enumerate() {
        atlas_rects.insert(
            (*name).to_string(),
            json!({
                "x": (i % atlas_cols) * 64,
                "y": (i / atlas_cols) * 64,
                "width": 64,
                "height": 64,
            }),
        );
    }
    let atlas_rel = format!("{art_root}/campaign_art_atlas.png");
    let atlas_record = out.write_sprite(
        &atlas_rel,
        &atlas,
        SpriteSpec {
            category: "campaign_atlas",
            generated_by: Some(GENERATED_BY),
            seed: Some(seed),
            extra: vec![
                ("art_role", json!("campaign_prop_texture_atlas")),
                ("tile_size", json!([64, 64])),
                ("columns", json!(atlas_cols)),
                ("rows", json!(atlas_rows)),
            ],
            ..SpriteSpec::default()
        },
    )?;
    generated.push(atlas_rel.clone());
    output.sprites.push(atlas_record);
    // Shipped-pack registration order: reusable art first, scene
    // frames/storyboards last (see the `scene_records` note above).
    output.sprites.extend(scene_records);

    out.write_json(
        &format!("{art_root}/campaign_art_atlas.json"),
        &json!({
            "version": VERSION,
            "image": "campaign_art_atlas.png",
            "size": [atlas.width(), atlas.height()],
            "tile_size": [64, 64],
            "columns": atlas_cols,
            "rows": atlas_rows,
            "sprites": atlas_rects,
        }),
    )?;

    // Build exact supporting-asset list from all timeline entries + this revision.
    let mut supporting_assets: Vec<String> = timeline
        .get("supporting_assets")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    for rel in &generated {
        if rel.starts_with(ui) || rel.starts_with(art_root) || rel.starts_with(&cards_root) {
            supporting_assets.push(rel.clone());
        }
    }
    supporting_assets.sort();
    supporting_assets.dedup();

    let campaigns: Vec<Value> = CAMPAIGNS
        .iter()
        .map(|(cid, act)| {
            json!({
                "id": cid,
                "title": act,
                "sequence_ids": SCENES.iter()
                    .filter(|(_, _, scene_act, ..)| *scene_act == *act)
                    .map(|(id, ..)| id)
                    .collect::<Vec<_>>(),
            })
        })
        .collect();

    // Sequences sorted by (numeric id prefix or 999, id), Python style.
    seq_list.sort_by_key(sequence_sort_key);
    let seqs = seq_list;
    let total_cinematic_frames: usize = seqs
        .iter()
        .map(|s| {
            s.get("frames")
                .and_then(Value::as_array)
                .map(|a| a.len())
                .unwrap_or(0)
        })
        .sum();
    let mut notes: Vec<String> = timeline
        .get("notes")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    for note in NEW_NOTES {
        if !notes.iter().any(|n| n == note) {
            notes.push((*note).to_string());
        }
    }
    {
        let obj = timeline.as_object_mut().expect("timeline is an object");
        obj.insert("schema_version".to_string(), json!("1.1.0"));
        obj.insert("generator_version".to_string(), json!(VERSION));
        obj.insert("seed".to_string(), json!(seed));
        obj.insert("canvas".to_string(), json!([SIZE.0, SIZE.1]));
        obj.insert("pixel_art_scale".to_string(), json!(SCALE));
        obj.insert("sequences".to_string(), Value::Array(seqs.clone()));
        obj.insert("campaigns".to_string(), Value::Array(campaigns.clone()));
        obj.insert("supporting_assets".to_string(), json!(supporting_assets));
        obj.insert("notes".to_string(), json!(notes));
    }
    out.write_json(&timeline_path, &timeline)?;

    // Manifest: retain every existing record, replace by id otherwise.
    let mut manifest = out
        .read_json(manifest_path)?
        .unwrap_or_else(|| json!({"schema_version": "1.2.0", "sprites": []}));
    {
        let mut merged: Vec<Value> = manifest
            .get("sprites")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for record in output.sprites.clone() {
            upsert_by_id(&mut merged, record);
        }
        let obj = manifest.as_object_mut().expect("manifest is an object");
        obj.insert("sprites".to_string(), Value::Array(merged));
        obj.insert("generator_version".to_string(), json!(VERSION));
        let campaign_scene_count: usize = campaigns
            .iter()
            .map(|c| {
                c.get("sequence_ids")
                    .and_then(Value::as_array)
                    .map(|a| a.len())
                    .unwrap_or(0)
            })
            .sum();
        let cinematics = json!({
            "timeline": &timeline_path,
            "sequence_count": seqs.len(),
            "frame_count": total_cinematic_frames,
            "campaign_scene_count": campaign_scene_count,
            "campaign_art_count": PROP_NAMES.len(),
            "new_portrait_count": new_portrait_count,
            "campaign_storyboard_count": new_sequences.len(),
            "campaign_key_art_count": CARD_SPECS.len(),
        });
        obj.insert("cinematics".to_string(), cinematics.clone());
        let atlas_key = json!({
            "image": format!("{art_root}/campaign_art_atlas.png"),
            "mapping": format!("{art_root}/campaign_art_atlas.json"),
            "tile_size": [64, 64],
            "sprite_count": PROP_NAMES.len(),
        });
        obj.insert("campaign_art_atlas".to_string(), atlas_key.clone());
        for (key, value) in [
            ("generator_version", json!(VERSION)),
            ("cinematics", cinematics),
            ("campaign_art_atlas", atlas_key),
        ] {
            output.manifest_extra.insert(key.to_string(), value);
        }
    }
    out.write_json(manifest_path, &manifest)?;

    // Build report at the pack root.
    let previous_sequence_count = old_ids
        .iter()
        .filter(|id| !scene_ids.contains(id.as_str()))
        .count();
    let unique_new_frame_hashes: HashSet<String> = new_sequences
        .iter()
        .flat_map(|s| {
            s.get("frames")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
        })
        .filter_map(|fr| fr.get("sha256").and_then(Value::as_str).map(str::to_string))
        .collect();
    let all_generated_files_exist = generated.iter().all(|rel| out.path(rel).is_file());
    let all_frame_hashes_match = new_sequences
        .iter()
        .flat_map(|s| {
            s.get("frames")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
        })
        .all(|fr| {
            let file = fr.get("file").and_then(Value::as_str).unwrap_or("");
            let expected = fr.get("sha256").and_then(Value::as_str).unwrap_or_default();
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
        "previous_sequence_count": previous_sequence_count,
        "campaign_scene_count": SCENES.len(),
        "campaign_acts": campaigns.len(),
        "total_sequence_count": seqs.len(),
        "new_campaign_frames": new_sequences
            .iter()
            .map(|s| s.get("frames").and_then(Value::as_array).map(|a| a.len()).unwrap_or(0))
            .sum::<usize>(),
        "total_cinematic_frames": total_cinematic_frames,
        "new_storyboards": new_sequences.len(),
        "new_reusable_props": PROP_NAMES.len(),
        "new_character_portraits": new_portrait_count,
        "generated_files": generated.len(),
        "unique_new_frame_hashes": unique_new_frame_hashes.len(),
        "all_generated_files_exist": all_generated_files_exist,
        "all_frame_hashes_match": all_frame_hashes_match,
        "previous_scenes_retained": previous_scenes_retained,
        "asset_atlas_rect_count": atlas_rects.len(),
        "runtime_validation": "NOT_RUN — Godot playback, compositing, and audio synchronization require engine verification.",
    });
    let report_rel = format!(
        "campaign_build_report_v{ver}.json",
        ver = VERSION.replace('.', "_")
    );
    out.write_json(&report_rel, &report)?;
    output.reports.push((report_rel, report.clone()));

    output.timeline = Some(TimelineContribution {
        sequences: new_sequences,
        campaigns,
        notes: NEW_NOTES.iter().map(|n| (*n).to_string()).collect(),
        schema_version: "1.1.0",
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
    /// `tempfile.TemporaryDirectory`.
    struct TempPack(PathBuf);

    impl TempPack {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "cnc-procgen-campaign_v59-{}-{}",
                std::process::id(),
                TMP_SEQ.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(root.join("sprites/cinematics")).unwrap();
            Self(root)
        }

        /// Seed a v5.8-style timeline + empty manifest, mimicking the Python
        /// `setUp` so existing-scene retention is exercised.
        fn setup_legacy(&self) {
            let mut old_sequences = Vec::new();
            for i in 1..=6usize {
                old_sequences.push(json!({
                    "id": format!("0{}_legacy_scene_{}", i, i),
                    "title": format!("Legacy {}", i),
                    "frames": [],
                }));
            }
            std::fs::write(
                self.0.join("sprites/cinematics/timeline.json"),
                serde_json::to_vec_pretty(&json!({
                    "schema_version": "1.0.0",
                    "generator_version": "5.8.0",
                    "canvas": [320, 180],
                    "sequences": old_sequences,
                    "supporting_assets": [],
                    "notes": [],
                }))
                .unwrap(),
            )
            .unwrap();
            std::fs::write(
                self.0.join("manifest.json"),
                serde_json::to_vec_pretty(&json!({
                    "schema_version": "1.2.0",
                    "sprites": [],
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

    fn png_dims(path: &std::path::Path) -> (u32, u32) {
        let bytes = std::fs::read(path).unwrap();
        let img = image::load_from_memory(&bytes).unwrap();
        (img.width(), img.height())
    }

    /// Sequence ids newly added by this layer have numeric prefixes 07..18.
    fn is_campaign_sequence(id: &str) -> bool {
        let prefix = id.split('_').next().unwrap_or("");
        match prefix.parse::<u32>() {
            Ok(n) => (7..=18).contains(&n),
            Err(_) => false,
        }
    }

    #[test]
    fn adds_twelve_campaign_scenes_and_retains_existing() {
        let pack = TempPack::new();
        pack.setup_legacy();
        let out = PackWriter::new(&pack.0);
        generate(&out, 77).unwrap();
        let report = pack.read_json_rel("campaign_build_report_v5_9_0.json");
        assert_eq!(report["campaign_scene_count"], 12);
        assert_eq!(report["total_sequence_count"], 18);
        assert_eq!(report["new_campaign_frames"], 96);
        assert_eq!(report["campaign_acts"], 3);
        assert_eq!(report["previous_scenes_retained"], true);

        let timeline = pack.read_json_rel("sprites/cinematics/timeline.json");
        assert_eq!(timeline["sequences"].as_array().unwrap().len(), 18);
        assert_eq!(timeline["campaigns"].as_array().unwrap().len(), 3);
        for seq in timeline["sequences"].as_array().unwrap() {
            let id = seq["id"].as_str().unwrap();
            if !is_campaign_sequence(id) {
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
    }

    #[test]
    fn art_atlas_and_art_assets_are_registered() {
        let pack = TempPack::new();
        pack.setup_legacy();
        let out = PackWriter::new(&pack.0);
        generate(&out, 77).unwrap();
        let atlas = pack.0.join("sprites/campaign_art/campaign_art_atlas.png");
        let dims = png_dims(&atlas);
        assert_eq!(dims, (384, 256));
        let mapping = pack.read_json_rel("sprites/campaign_art/campaign_art_atlas.json");
        assert_eq!(mapping["sprites"].as_object().unwrap().len(), 22);

        let manifest = pack.read_json_rel("manifest.json");
        let sprites = manifest["sprites"].as_array().unwrap();
        let ids: Vec<&str> = sprites.iter().map(|s| s["id"].as_str().unwrap()).collect();
        let unique: HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len());
        let portraits = sprites
            .iter()
            .filter(|s| s.get("art_role").and_then(Value::as_str) == Some("character_portrait"))
            .count();
        assert!(portraits >= 6);
        for entry in sprites {
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
    }

    #[test]
    fn repeat_run_does_not_duplicate_manifest_records_or_sequences() {
        let pack = TempPack::new();
        pack.setup_legacy();
        let out = PackWriter::new(&pack.0);
        generate(&out, 77).unwrap();
        let first_manifest = pack.read_json_rel("manifest.json");
        let first_timeline = pack.read_json_rel("sprites/cinematics/timeline.json");
        let first_frame = std::fs::read(
            pack.0.join(
                first_timeline["sequences"][6]["frames"][0]["file"]
                    .as_str()
                    .unwrap(),
            ),
        )
        .unwrap();
        generate(&out, 77).unwrap();
        let second_manifest = pack.read_json_rel("manifest.json");
        let second_timeline = pack.read_json_rel("sprites/cinematics/timeline.json");
        let second_frame = std::fs::read(
            pack.0.join(
                second_timeline["sequences"][6]["frames"][0]["file"]
                    .as_str()
                    .unwrap(),
            ),
        )
        .unwrap();
        assert_eq!(
            first_manifest["sprites"].as_array().unwrap().len(),
            second_manifest["sprites"].as_array().unwrap().len()
        );
        assert_eq!(
            first_timeline["sequences"].as_array().unwrap().len(),
            second_timeline["sequences"].as_array().unwrap().len()
        );
        assert_eq!(first_frame, second_frame);
        let notes: Vec<&str> = second_timeline["notes"]
            .as_array()
            .unwrap()
            .iter()
            .map(Value::as_str)
            .collect::<Option<Vec<_>>>()
            .unwrap();
        let unique: HashSet<&str> = notes.iter().copied().collect();
        assert_eq!(
            second_timeline["notes"].as_array().unwrap().len(),
            unique.len()
        );
    }

    #[test]
    fn public_api_is_deterministic_and_correctly_sized() {
        // Repeated calls return identical canvases.
        assert_eq!(
            render_scene(77, "dawn", "border", 3),
            render_scene(77, "dawn", "border", 3)
        );
        assert_eq!(base_scene(77, "night", 0), base_scene(77, "night", 0));
        assert_eq!(portrait("sniper", 8), portrait("sniper", 8));
        // Explicit seed drives art_sprite (Python default seed=59000).
        assert_eq!(
            art_sprite("medic_bag", 59000),
            art_sprite("medic_bag", 59000)
        );
        // Frame dimensions.
        let frame = render_scene(DEFAULT_SEED, "desert", "convoy", 0);
        assert_eq!((frame.width(), frame.height()), (320, 180));
        let art = art_sprite("bunker", DEFAULT_SEED);
        assert_eq!((art.width(), art.height()), (64, 64));
        let face = portrait("pilot", 9);
        assert_eq!((face.width(), face.height()), (96, 96));
    }
}
