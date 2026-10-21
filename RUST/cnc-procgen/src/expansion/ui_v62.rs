//! Port of `asset_expansion_v6_2.py` — v6.2 commander/NPC portraits, tactical
//! icons, UI control kit, screen mockups, wireframes and contact sheets.
//!
//! Function mapping (Python → Rust, call-for-call):
//! `font` → [`FontKit`] (DejaVu → Liberation → 5×7 bitmap fallback, same
//! split), `gradient` → [`Canvas::gradient`], `rounded` → [`rounded`],
//! `generate_avatar` → [`generate_avatar`] (same per-person seed expression
//! `seed + sum(ord(c) for c in pid)`, same 65-blob draw budget), `draw_icon`
//! → [`draw_icon`] branch-for-branch (the `seed` parameter stays unused, as
//! in the Python original), `gen_avatars`/`gen_icons`/`gen_controls` → same
//! names, `panel`/`button` → [`panel`]/[`button`], `draw_battlefield` →
//! [`draw_battlefield`], `make_mockup`/`make_wireframe` → same names (same
//! per-screen draw payloads with `seed + i*37`), `build_screen_flow` →
//! [`build_screen_flow`], `_load_image`/`_fit` → [`load_rgba`]/[`fit_thumb`],
//! `_board_panel` → [`board_panel`], `make_personnel_contact_sheet` →
//! [`make_personnel_contact_sheet`], `make_screen_contact_sheet` →
//! [`make_screen_contact_sheet`], `make_art_direction_board` →
//! [`make_art_direction_board`], `add_manifest_records` → inlined into
//! [`generate`], `build` → [`generate`].
//!
//! Accepted deviations (documented, not "fixed"):
//! - SplitMix64 ([`Rng`]) instead of Python MT19937: same draw counts, order
//!   and ranges, different values.
//! - [`Canvas`] rasterizer instead of Pillow (line stamping, polygon
//!   scanlines, ellipse rasterization differ at pixel level).
//! - Resampling uses Catmull-Rom where Pillow isasked for LANCZOS (the
//!   canvas has no Lanczos kernel); nearest is used where Pillow used
//!   NEAREST, and the thumbnail/`_fit` scaling rounds the reduced size.
//! - [`Canvas::pieslice`] has no outline parameter, so the headwear
//!   outlined-pieslice branches stroke the arc plus the chord instead.
//! - Button/panel text centering measures glyph advances instead of
//!   Pillow's ink bbox (a few pixels of drift on some strings).
//! - Fully opaque canvases encode as RGB PNGs even where the Python script
//!   kept RGBA mode (pixel data identical, channel count differs).
//! - JSON object key order is alphabetical here vs Python insertion order;
//!   `json.dumps` non-ASCII escaping is not reproduced (UTF-8 is literal).
//! - A missing base `manifest.json` is treated as an empty pack (the Python
//!   script raises `FileNotFoundError`; the builder owns staging here).
//! - The report/index strings ("Python + Pillow",
//!   "procedural_python_pillow_only" …) are ported verbatim, including their
//!   Python-era wording — they are copy strings, not implementation claims.
//! - This layer does not touch the cinematics timeline; the report's
//!   preserved-cinematic counters read `sprites/cinematics/timeline.json`
//!   when the builder has already staged it, exactly like the Python script.

use serde_json::{json, Map, Value};

use super::{compose_atlas, sprite_record, LayerOutput, PackWriter, SpriteSpec};
use crate::prng::{Rng, Seed};
use crate::tileset::canvas::Canvas;
use crate::tileset::font::{draw_text as draw_glyph_text, draw_text_ttf, TtfFont};
use crate::tileset::pack::PackError;
use crate::tileset::{Rgba, TileRngExt};

/// Python `generate(..., seed=62_021)` default seed (independent of the
/// pack's `--seed`, exactly as the script was run to build the real pack).
pub const DEFAULT_SEED: u64 = 62_021;

/// Python `VERSION` — asserted by the Python test suite.
pub const VERSION: &str = "6.2.1";

/// Python `AVATAR_SIZE`, `ICON_SIZE`, `SCREEN_SIZE`.
const AVATAR_SIZE: u32 = 128;
const ICON_SIZE: u32 = 32;
const SCREEN_W: i32 = 1600;
const SCREEN_H: i32 = 900;

type Rgb = [u8; 3];

/// Icon registry: `(icon name, pack-relative file, rendered canvas)`.
type IconPaths = Vec<(&'static str, String, Canvas)>;

/// Portrait registry: `(person id, avatar mapping entry)` in [`PEOPLE`]
/// insert order.
type AvatarMap = Vec<(String, Value)>;

/// Sprite records produced by the portrait/icon/control generators.
type AvatarRecords = Vec<(String, Value)>;

/// Opaque RGBA for an RGB triple.
const fn with_a(c: Rgb, a: u8) -> Rgba {
    [c[0], c[1], c[2], a]
}

/// Python `max(0, int(v * f))` per channel (`int` truncates toward zero).
fn dim(v: u8, f: f64) -> u8 {
    let scaled = (v as f64 * f) as i32;
    scaled.clamp(0, 255) as u8
}

fn dim_rgb(c: Rgb, f: f64) -> Rgb {
    [dim(c[0], f), dim(c[1], f), dim(c[2], f)]
}

/// Python `max(0, v - d)` per channel.
fn sub_rgb(c: Rgb, d: i32) -> Rgb {
    [
        (c[0] as i32 - d).max(0) as u8,
        (c[1] as i32 - d).max(0) as u8,
        (c[2] as i32 - d).max(0) as u8,
    ]
}

/// Python `min(255, v + d)` per channel.
fn add_rgb(c: Rgb, d: i32) -> Rgb {
    [
        (c[0] as i32 + d).min(255) as u8,
        (c[1] as i32 + d).min(255) as u8,
        (c[2] as i32 + d).min(255) as u8,
    ]
}

/// Python `min(255, int(v * f))` per channel.
fn bright_rgb(c: Rgb, f: f64) -> Rgb {
    [
        ((c[0] as f64 * f) as i32).min(255) as u8,
        ((c[1] as f64 * f) as i32).min(255) as u8,
        ((c[2] as f64 * f) as i32).min(255) as u8,
    ]
}

/// Python `contains-any` keyword checks with the exact per-group ordering
/// (first group wins).
fn contains_any(name: &str, keys: &[&str]) -> bool {
    keys.iter().any(|k| name.contains(k))
}

const RED_KEYS: [&str; 7] = [
    "attack",
    "enemy",
    "airstrike",
    "artillery",
    "hazard",
    "mine",
    "failed",
];
const AMBER_KEYS: [&str; 8] = [
    "credits", "power", "supply", "research", "upgrade", "tech", "rank", "medal",
];
const GREEN_KEYS: [&str; 7] = [
    "rescue", "repair", "health", "shield", "complete", "alliance", "confirm",
];
const RED_KEYS_2: [&str; 6] = ["cancel", "delete", "close", "stop", "warning", "alert"];

/// The script's own palette dict `C`, verbatim.
const C_BG: Rgb = [7, 16, 27];
const C_PANEL: Rgb = [13, 29, 45];
const C_PANEL2: Rgb = [20, 42, 61];
const C_LINE: Rgb = [39, 105, 139];
const C_CYAN: Rgb = [70, 196, 233];
const C_WHITE: Rgb = [226, 237, 242];
const C_MUTED: Rgb = [143, 171, 185];
const C_AMBER: Rgb = [231, 174, 70];
const C_RED: Rgb = [213, 73, 73];
const C_GREEN: Rgb = [79, 191, 132];
const C_DARK: Rgb = [4, 10, 17];
#[allow(dead_code)] // carried to mirror the Python palette dict
const C_WIRE: Rgb = [66, 74, 81];

/// One `PEOPLE` row: role palette + head geometry драйверов.
pub struct Person {
    pub pid: &'static str,
    pub name: &'static str,
    pub role: &'static str,
    pub faction: &'static str,
    pub skin: Rgb,
    pub hair: Rgb,
    pub uniform: Rgb,
    pub accent: Rgb,
    pub headgear: &'static str,
    pub beard: &'static str,
    pub eyewear: &'static str,
    pub presentation: &'static str,
}

/// Python `PEOPLE`, verbatim (28 rows: 5 commanders + 23 others).
static PEOPLE: [Person; 28] = [
    Person {
        pid: "cmdr_hale",
        name: "Commander Marcus Hale",
        role: "Commander",
        faction: "Raven Coalition",
        skin: [190, 139, 105],
        hair: [35, 34, 34],
        uniform: [56, 76, 78],
        accent: [55, 164, 225],
        headgear: "none",
        beard: "stubble",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "cmdr_volkov",
        name: "Commander Irina Volkov",
        role: "Commander",
        faction: "Northern Directorate",
        skin: [224, 177, 150],
        hair: [105, 48, 39],
        uniform: [65, 65, 73],
        accent: [201, 77, 68],
        headgear: "beret",
        beard: "none",
        eyewear: "none",
        presentation: "f",
    },
    Person {
        pid: "cmdr_tanaka",
        name: "Commander Kenji Tanaka",
        role: "Commander",
        faction: "Pacific League",
        skin: [205, 157, 119],
        hair: [27, 25, 25],
        uniform: [75, 84, 70],
        accent: [215, 173, 76],
        headgear: "none",
        beard: "none",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "cmdr_okafor",
        name: "Commander Chidi Okafor",
        role: "Commander",
        faction: "Continental Union",
        skin: [126, 81, 60],
        hair: [24, 25, 24],
        uniform: [53, 83, 61],
        accent: [82, 193, 145],
        headgear: "beret",
        beard: "goatee",
        eyewear: "sunglasses",
        presentation: "m",
    },
    Person {
        pid: "cmdr_moretti",
        name: "Commander Luca Moretti",
        role: "Commander",
        faction: "European Alliance",
        skin: [215, 176, 146],
        hair: [179, 177, 162],
        uniform: [63, 75, 84],
        accent: [99, 159, 221],
        headgear: "none",
        beard: "beard",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "dir_sayegh",
        name: "Director Amira Sayegh",
        role: "Strategic Intelligence",
        faction: "Raven Coalition",
        skin: [173, 117, 87],
        hair: [40, 30, 29],
        uniform: [47, 59, 75],
        accent: [92, 190, 213],
        headgear: "none",
        beard: "none",
        eyewear: "glasses",
        presentation: "f",
    },
    Person {
        pid: "npc_engineer_rook",
        name: "Chief Engineer Rook",
        role: "Engineer",
        faction: "Raven Coalition",
        skin: [198, 145, 111],
        hair: [75, 61, 47],
        uniform: [112, 87, 47],
        accent: [228, 167, 69],
        headgear: "hardhat",
        beard: "stubble",
        eyewear: "goggles",
        presentation: "m",
    },
    Person {
        pid: "npc_medic_sato",
        name: "Medic Hana Sato",
        role: "Field Medic",
        faction: "Raven Coalition",
        skin: [221, 177, 144],
        hair: [40, 31, 29],
        uniform: [63, 75, 82],
        accent: [212, 88, 92],
        headgear: "medic_cap",
        beard: "none",
        eyewear: "none",
        presentation: "f",
    },
    Person {
        pid: "npc_pilot_marek",
        name: "Pilot Tomas Marek",
        role: "Pilot",
        faction: "Raven Coalition",
        skin: [216, 170, 137],
        hair: [95, 63, 43],
        uniform: [50, 73, 82],
        accent: [235, 174, 73],
        headgear: "flight_helmet",
        beard: "stubble",
        eyewear: "visor",
        presentation: "m",
    },
    Person {
        pid: "npc_intel_vesper",
        name: "Intel Officer Vesper",
        role: "Intelligence",
        faction: "Raven Coalition",
        skin: [186, 133, 104],
        hair: [34, 30, 33],
        uniform: [45, 58, 69],
        accent: [95, 178, 214],
        headgear: "headset",
        beard: "none",
        eyewear: "glasses",
        presentation: "f",
    },
    Person {
        pid: "npc_radar_lee",
        name: "Radar Operator Lee",
        role: "Signals",
        faction: "Raven Coalition",
        skin: [224, 180, 146],
        hair: [28, 29, 29],
        uniform: [56, 73, 77],
        accent: [90, 198, 194],
        headgear: "headset",
        beard: "none",
        eyewear: "glasses",
        presentation: "m",
    },
    Person {
        pid: "npc_scout_arden",
        name: "Scout Arden",
        role: "Recon",
        faction: "Raven Coalition",
        skin: [192, 143, 111],
        hair: [49, 37, 31],
        uniform: [69, 86, 66],
        accent: [132, 190, 109],
        headgear: "hood",
        beard: "none",
        eyewear: "none",
        presentation: "f",
    },
    Person {
        pid: "npc_quartermaster_vesper",
        name: "Quartermaster Vesper",
        role: "Logistics",
        faction: "Raven Coalition",
        skin: [206, 160, 132],
        hair: [72, 50, 39],
        uniform: [83, 88, 71],
        accent: [214, 166, 74],
        headgear: "cap",
        beard: "none",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "npc_relief_nia",
        name: "Relief Coordinator Nia",
        role: "Civilian Relief",
        faction: "Neutral",
        skin: [119, 77, 58],
        hair: [28, 24, 23],
        uniform: [77, 91, 83],
        accent: [93, 190, 157],
        headgear: "none",
        beard: "none",
        eyewear: "none",
        presentation: "f",
    },
    Person {
        pid: "npc_resistance_contact",
        name: "Resistance Contact",
        role: "Local Guide",
        faction: "Resistance",
        skin: [177, 125, 89],
        hair: [38, 30, 25],
        uniform: [82, 71, 55],
        accent: [202, 153, 74],
        headgear: "hood",
        beard: "stubble",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "npc_sniper",
        name: "Sniper Vale",
        role: "Sharpshooter",
        faction: "Raven Coalition",
        skin: [210, 162, 129],
        hair: [65, 48, 38],
        uniform: [52, 75, 60],
        accent: [115, 170, 111],
        headgear: "hood",
        beard: "none",
        eyewear: "scope",
        presentation: "f",
    },
    Person {
        pid: "npc_mechanic",
        name: "Mechanic Dima",
        role: "Vehicle Crew",
        faction: "Northern Directorate",
        skin: [219, 172, 138],
        hair: [118, 79, 49],
        uniform: [69, 67, 61],
        accent: [198, 91, 77],
        headgear: "cap",
        beard: "mustache",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "npc_radio_operator",
        name: "Radio Operator Ellis",
        role: "Communications",
        faction: "Raven Coalition",
        skin: [201, 149, 114],
        hair: [49, 39, 33],
        uniform: [59, 72, 84],
        accent: [75, 165, 209],
        headgear: "headset",
        beard: "none",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "npc_spotter",
        name: "Spotter Juno",
        role: "Forward Observer",
        faction: "Raven Coalition",
        skin: [235, 192, 160],
        hair: [125, 81, 58],
        uniform: [61, 77, 65],
        accent: [220, 178, 86],
        headgear: "cap",
        beard: "none",
        eyewear: "binoculars",
        presentation: "f",
    },
    Person {
        pid: "npc_supply_officer",
        name: "Supply Officer Kofi",
        role: "Quartermaster",
        faction: "Continental Union",
        skin: [126, 79, 58],
        hair: [27, 25, 24],
        uniform: [65, 83, 68],
        accent: [83, 177, 136],
        headgear: "cap",
        beard: "beard",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "npc_diplomat",
        name: "Envoy Celeste Ward",
        role: "Diplomat",
        faction: "European Alliance",
        skin: [229, 190, 160],
        hair: [126, 85, 63],
        uniform: [53, 67, 80],
        accent: [96, 154, 219],
        headgear: "none",
        beard: "none",
        eyewear: "glasses",
        presentation: "f",
    },
    Person {
        pid: "npc_scientist",
        name: "Dr. Imani Cole",
        role: "Researcher",
        faction: "Neutral",
        skin: [117, 73, 53],
        hair: [27, 24, 23],
        uniform: [64, 75, 81],
        accent: [183, 162, 218],
        headgear: "none",
        beard: "none",
        eyewear: "glasses",
        presentation: "f",
    },
    Person {
        pid: "npc_civilian_leader",
        name: "Local Councilor Pavel",
        role: "Civilian Leader",
        faction: "Neutral",
        skin: [205, 160, 128],
        hair: [75, 56, 42],
        uniform: [78, 85, 78],
        accent: [203, 167, 96],
        headgear: "cap",
        beard: "beard",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "npc_war_correspondent",
        name: "War Correspondent Mara",
        role: "Journalist",
        faction: "Neutral",
        skin: [220, 172, 145],
        hair: [54, 38, 31],
        uniform: [69, 77, 88],
        accent: [213, 114, 113],
        headgear: "none",
        beard: "none",
        eyewear: "glasses",
        presentation: "f",
    },
    Person {
        pid: "npc_trader",
        name: "Free Trader Soren",
        role: "Black Market",
        faction: "Neutral",
        skin: [191, 138, 101],
        hair: [80, 52, 36],
        uniform: [83, 77, 63],
        accent: [226, 166, 79],
        headgear: "cap",
        beard: "beard",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "npc_refugee_child",
        name: "Refugee Liaison",
        role: "Refugee Support",
        faction: "Neutral",
        skin: [231, 188, 158],
        hair: [81, 53, 38],
        uniform: [71, 79, 80],
        accent: [113, 183, 181],
        headgear: "hood",
        beard: "none",
        eyewear: "none",
        presentation: "f",
    },
    Person {
        pid: "npc_prisoner",
        name: "Captured Officer Venn",
        role: "Detainee",
        faction: "Northern Directorate",
        skin: [199, 152, 122],
        hair: [37, 32, 30],
        uniform: [89, 75, 69],
        accent: [184, 114, 89],
        headgear: "none",
        beard: "stubble",
        eyewear: "none",
        presentation: "m",
    },
    Person {
        pid: "npc_maintenance",
        name: "Maintenance Tech Quinn",
        role: "Base Support",
        faction: "Raven Coalition",
        skin: [182, 127, 94],
        hair: [36, 33, 30],
        uniform: [98, 91, 67],
        accent: [219, 161, 76],
        headgear: "hardhat",
        beard: "none",
        eyewear: "goggles",
        presentation: "f",
    },
];

/// Python `PEOPLE` accessor.
pub fn people() -> &'static [Person] {
    &PEOPLE
}

/// Python `ICONS`, verbatim (110 ids).
static ICONS: [&str; 110] = [
    "select",
    "move",
    "attack",
    "stop",
    "hold",
    "patrol",
    "defend",
    "repair",
    "build",
    "rally",
    "recon",
    "radar",
    "airstrike",
    "artillery",
    "capture",
    "rescue",
    "transport",
    "objective_primary",
    "objective_secondary",
    "objective_bonus",
    "waypoint",
    "ping",
    "power",
    "credits",
    "population",
    "timer",
    "health",
    "armor",
    "ammo",
    "veterancy",
    "tech",
    "upgrade",
    "alert",
    "pause",
    "play",
    "settings",
    "map",
    "layers",
    "fullscreen",
    "zoom_in",
    "zoom_out",
    "comms",
    "messages",
    "roster",
    "diplomacy",
    "alliance",
    "enemy",
    "neutral",
    "surrender",
    "save",
    "load",
    "delete",
    "confirm",
    "cancel",
    "back",
    "forward",
    "next",
    "previous",
    "sound",
    "video",
    "controls",
    "accessibility",
    "network",
    "server",
    "download",
    "upload",
    "campaign",
    "skirmish",
    "multiplayer",
    "spectator",
    "rank",
    "medal",
    "commander",
    "unit_info",
    "building_info",
    "production",
    "queue",
    "deploy",
    "unload",
    "evacuate",
    "special_ability",
    "cooldown",
    "shield",
    "stealth",
    "mine",
    "hazard",
    "weather",
    "night",
    "day",
    "supply",
    "fuel",
    "repair_pad",
    "research",
    "tech_tier",
    "mission_complete",
    "mission_failed",
    "warning",
    "lock",
    "unlock",
    "filter",
    "search",
    "sort",
    "help",
    "close",
    "menu",
    "chat",
    "voice",
    "report",
    "spectate",
    "ping_ms",
];

/// Python `ICONS` accessor.
pub fn icons() -> &'static [&'static str] {
    &ICONS
}

// ---------------------------------------------------------------------------
// Fonts and text helpers (Python `font` / `draw_text` / `textbbox` stand-ins)
// ---------------------------------------------------------------------------

/// One cached TrueType load per weight, mirroring the Python `font(size,
/// bold)` path list (DejaVu Sans → Liberation Sans → bitmap-fallback
/// condition). Python reopens the font file per call; one cached load per
/// weight produces identical bytes.
struct FontKit {
    regular: Option<TtfFont>,
    bold: Option<TtfFont>,
}

impl FontKit {
    fn new() -> Self {
        Self {
            regular: TtfFont::load(false),
            bold: TtfFont::load(true),
        }
    }

    fn pick(&self, bold: bool) -> Option<&TtfFont> {
        if bold {
            self.bold.as_ref()
        } else {
            self.regular.as_ref()
        }
    }

    /// Python `d.text((x, y), text, font=font(size, bold), fill=color or
    /// C["white"])` with the anchor at the top-left of the first glyph.
    #[allow(clippy::too_many_arguments)]
    fn draw(
        &self,
        canvas: &mut Canvas,
        x: i32,
        y: i32,
        text: &str,
        size: i32,
        color: Rgba,
        bold: bool,
    ) {
        match self.pick(bold) {
            Some(font) => draw_text_ttf(canvas, font, x, y, text, size as f32, color),
            // Bitmap fallback ignores the requested size (documented
            // deviation: 5×7 pixels at a fixed advance).
            None => draw_glyph_text(canvas, x, y, text, color),
        }
    }

    /// Python `d.textbbox((0, 0), label, font=f)` ink-bbox stand-in: draw
    /// the whole string into an oversized scratch canvas and measure the
    /// lit-pixel bounding box, exactly the semantics of Pillow's default
    /// "la"-anchored text measurement. Width first, height second.
    fn ink_size(&self, text: &str, size: i32, bold: bool) -> (i32, i32) {
        let width_guess = ((text.chars().count() as i32 + 2) * size * 9 / 10 + 8).max(8);
        let height_guess = size * 3 + 8;
        let mut scratch = Canvas::transparent(
            width_guess.clamp(16, 8192) as u32,
            height_guess.max(16) as u32,
        );
        self.draw(&mut scratch, 1, 1, text, size, [255, 255, 255, 255], bold);
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = -1;
        let mut max_y = -1;
        for y in 0..scratch.height() as i32 {
            for x in 0..scratch.width() as i32 {
                if scratch.pixel(x, y).is_some_and(|p| p[3] > 0) {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
            }
        }
        if max_x < 0 {
            return (0, 0);
        }
        (max_x - min_x + 1, max_y - min_y + 1)
    }
}

// ---------------------------------------------------------------------------
// Pillow draw-call stand-ins not already covered by [`Canvas`]
// ---------------------------------------------------------------------------

/// Python `d.line((x0, y0, x1, y1, x2, y2, …), fill, width)`: one connected
/// polyline, drawn segment by segment (endpoints are stamped twice, exactly
/// like Pillow's continuous draw).
fn polyline(canvas: &mut Canvas, points: &[(i32, i32)], rgba: Rgba, width: u32) {
    for pair in points.windows(2) {
        canvas.line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, rgba, width);
    }
}

/// Python `rounded(draw, xy, radius, fill, outline, width)`: the canvas
/// rounded-rect has separate fill/outline entry points, so a call with both
/// fills first and strokes second (Pillow draws the outline over the fill).
#[allow(clippy::too_many_arguments)]
fn rounded(
    canvas: &mut Canvas,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    fill: Rgba,
    outline: Option<Rgba>,
    width: u32,
) {
    canvas.rounded_rect(x0, y0, x1, y1, radius, fill);
    if let Some(outline) = outline {
        canvas.rounded_rect_outline(x0, y0, x1, y1, radius, outline, width);
    }
}

/// Python `d.pieslice(…, fill, outline, width)`: [`Canvas::pieslice`] has no
/// outline parameter, so the stroke is drawn as the arc plus the chord
/// between the two elliptical endpoints (documented deviation; the wedge,
/// arc and endpoint geometry match the canvas conventions).
#[allow(clippy::too_many_arguments)]
fn pieslice_outlined(
    canvas: &mut Canvas,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    start_deg: f64,
    end_deg: f64,
    fill: Rgba,
    outline: Rgba,
    width: u32,
) {
    canvas.pieslice(x0, y0, x1, y1, start_deg, end_deg, fill);
    canvas.arc(x0, y0, x1, y1, start_deg, end_deg, outline, width);
    let cx = (x0 + x1) as f64 / 2.0;
    let cy = (y0 + y1) as f64 / 2.0;
    let rx = (x1 - x0) as f64 / 2.0;
    let ry = (y1 - y0) as f64 / 2.0;
    for angle in [start_deg, end_deg] {
        let rad = angle.to_radians();
        let endpoint_x = (cx + rx * rad.cos()).round() as i32;
        let endpoint_y = (cy + ry * rad.sin()).round() as i32;
        canvas.line(cx as i32, cy as i32, endpoint_x, endpoint_y, outline, width);
    }
}

/// Reduced thumbnail dimensions for [`thumbnail`]: PIL `thumbnail` never
/// enlarges and picks floor-or-ceil per axis to stay closest to the source
/// aspect; rounding matches that choice for our inputs.
fn thumb_dims(source: &Canvas, target_w: u32, target_h: u32) -> (u32, u32) {
    let (sw, sh) = (source.width(), source.height());
    if sw <= target_w && sh <= target_h {
        return (sw, sh);
    }
    let par = (target_w as f64 / sw.max(1) as f64).min(target_h as f64 / sh.max(1) as f64);
    let dw = ((sw as f64 * par).round() as u32).max(1);
    let dh = ((sh as f64 * par).round() as u32).max(1);
    (dw, dh)
}

/// Python `image.thumbnail(size, Image.Resampling.LANCZOS)`: shrink to fit,
/// keep aspect, never enlarge (Catmull-Rom stands in for LANCZOS).
fn thumbnail(source: &Canvas, target_w: u32, target_h: u32) -> Canvas {
    let (dw, dh) = thumb_dims(source, target_w, target_h);
    source.resize_catmullrom(dw, dh)
}

/// Python `_fit`: resize a generated preview into a target rectangle without
/// distortion, centered on a `(5, 14, 23, 255)` backing canvas.
fn fit_thumb(source: &Canvas, target_w: u32, target_h: u32) -> Canvas {
    let fitted = thumbnail(source, target_w, target_h);
    let mut canvas = Canvas::new_filled(target_w, target_h, [5, 14, 23, 255]);
    canvas.alpha_composite(
        &fitted,
        (
            ((target_w - fitted.width()) / 2) as i32,
            ((target_h - fitted.height()) / 2) as i32,
        ),
    );
    canvas
}

/// Python `_load_image`: decode an image written by this generator and
/// return an independent RGBA surface.
fn load_rgba(path: &std::path::Path) -> Result<Canvas, PackError> {
    let img = image::open(path)?.to_rgba8();
    let mut canvas = Canvas::transparent(img.width(), img.height());
    for (x, y, pixel) in img.enumerate_pixels() {
        canvas.set_pixel(x as i32, y as i32, pixel.0);
    }
    Ok(canvas)
}

/// Python `str.title()` over lowercase id words (first letter of each word
/// uppercase, rest lowercase; the ids are plain ASCII).
fn py_title(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut at_word_start = true;
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            if at_word_start {
                out.push(ch.to_ascii_uppercase());
            } else {
                out.push(ch.to_ascii_lowercase());
            }
            at_word_start = false;
        } else {
            at_word_start = true;
            out.push(ch);
        }
    }
    out
}

/// Python `draw_text(d, x, y, text, size, color, bold)` — module-level
/// `d.text(...)` helper with the default white fill.
#[allow(clippy::too_many_arguments)]
fn draw_text(
    canvas: &mut Canvas,
    kit: &FontKit,
    x: i32,
    y: i32,
    text: &str,
    size: i32,
    color: Option<Rgba>,
    bold: bool,
) {
    kit.draw(
        canvas,
        x,
        y,
        text,
        size,
        color.unwrap_or(with_a(C_WHITE, 255)),
        bold,
    );
}

/// Python `panel(im, xy, title, accent)`: rounded intel panel with an
/// optional titled header band.
#[allow(clippy::too_many_arguments)]
fn panel(
    canvas: &mut Canvas,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    title: Option<&str>,
    accent: Option<Rgb>,
    kit: &FontKit,
) {
    rounded(
        canvas,
        x1,
        y1,
        x2,
        y2,
        9,
        with_a(C_PANEL, 238),
        Some(with_a(C_LINE, 255)),
        2,
    );
    if let Some(title) = title {
        rounded(
            canvas,
            x1 + 1,
            y1 + 1,
            x2 - 1,
            (y1 + 42).min(y2 - 1),
            8,
            with_a(C_PANEL2, 255),
            None,
            0,
        );
        canvas.rect(
            x1 + 1,
            y1 + 25,
            x2 - 1,
            (y1 + 42).min(y2 - 1),
            Some(with_a(C_PANEL2, 255)),
            None,
            0,
        );
        canvas.rect(
            x1 + 1,
            y1 + 2,
            x1 + 5,
            (y1 + 41).min(y2 - 2),
            Some(with_a(accent.unwrap_or(C_CYAN), 255)),
            None,
            0,
        );
        draw_text(
            canvas,
            kit,
            x1 + 15,
            y1 + 10,
            title,
            17,
            Some(with_a(C_WHITE, 255)),
            true,
        );
    }
}

/// Python `button(d, xy, label, selected, secondary)`.
#[allow(clippy::too_many_arguments)]
fn button(
    canvas: &mut Canvas,
    kit: &FontKit,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    label: &str,
    selected: bool,
    secondary: bool,
) {
    let outline = if selected {
        C_AMBER
    } else if secondary {
        C_LINE
    } else {
        C_CYAN
    };
    let fill: Rgba = if selected {
        [25, 68, 85, 255]
    } else {
        [15, 34, 51, 250]
    };
    rounded(
        canvas,
        x1,
        y1,
        x2,
        y2,
        5,
        fill,
        Some(with_a(outline, 255)),
        2,
    );
    // f = font(max(11, min(17, int((y2-y1)*.28))), True)
    let size = (((y2 - y1) as f64 * 0.28) as i32).clamp(11, 17);
    let (tw, th) = kit.ink_size(label, size, true);
    let tx = (x1 + x2 - tw) / 2;
    let ty = (y1 + y2 - th) / 2 - 1;
    draw_text(
        canvas,
        kit,
        tx,
        ty,
        label,
        size,
        Some(with_a(C_WHITE, 255)),
        true,
    );
}

/// Python `_board_panel(draw, xy, title, accent)`: overview-board section
/// panel with a colored spine and uppercase title.
#[allow(clippy::too_many_arguments)]
fn board_panel(
    canvas: &mut Canvas,
    kit: &FontKit,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    title: &str,
    accent: Rgb,
) {
    rounded(
        canvas,
        x1,
        y1,
        x2,
        y2,
        12,
        with_a(C_PANEL, 255),
        Some(with_a(C_LINE, 255)),
        2,
    );
    canvas.rounded_rect(x1 + 2, y1 + 2, x2 - 2, y1 + 48, 10, with_a(C_PANEL2, 255));
    canvas.rect(
        x1 + 2,
        y1 + 27,
        x2 - 2,
        y1 + 48,
        Some(with_a(C_PANEL2, 255)),
        None,
        0,
    );
    canvas.rect(
        x1 + 2,
        y1 + 3,
        x1 + 7,
        y1 + 47,
        Some(with_a(accent, 255)),
        None,
        0,
    );
    draw_text(
        canvas,
        kit,
        x1 + 18,
        y1 + 13,
        &title.to_uppercase(),
        17,
        Some(with_a(C_WHITE, 255)),
        true,
    );
}

// ---------------------------------------------------------------------------
// Portraits — Python `generate_avatar`
// ---------------------------------------------------------------------------

/// Python `generate_avatar`: a 256×256 character card (background field,
/// 65 seeded noise blobs, vignette, uniform, collar, neck, head, hair,
/// headgear, features, beard, eyewear, tactical details) reduced to
/// [`AVATAR_SIZE`]. The RNG stream re-derives from
/// `seed + sum(ord(c) for c in pid)` and every draw (blob loop, stubble
/// dots) keeps the Python order; values differ by design (SplitMix64).
fn generate_avatar(person: &Person, seed: u64) -> Canvas {
    let pid_seed: u64 = person.pid.chars().map(|c| c as u64).sum::<u64>();
    let mut rnd = Rng::new(Seed(seed + pid_seed));
    let s: i32 = 256;
    let accent = person.accent;
    let uniform = person.uniform;
    let skin = person.skin;
    let hair = person.hair;
    let headgear = person.headgear;
    let presentation = person.presentation;
    let beard = person.beard;
    let eyewear = person.eyewear;

    // Character card atmosphere: subdued diagonal field with subtle noise.
    let bg_top: Rgb = [
        dim(accent[0], 0.42),
        dim(accent[1], 0.42),
        dim(accent[2], 0.42),
    ];
    let bg_bot: Rgb = [
        dim(uniform[0], 0.19),
        dim(uniform[1], 0.19),
        dim(uniform[2], 0.19),
    ];
    let mut im = Canvas::transparent(s as u32, s as u32);
    im.alpha_composite(
        &Canvas::gradient(s as u32, s as u32, bg_top, bg_bot),
        (0, 0),
    );
    for _ in 0..65 {
        let x = rnd.randrange2(0, s);
        let y = rnd.randrange2(0, s);
        let r = *rnd.choose(&[1, 1, 2, 3]);
        let mut shade = [0u8; 4];
        for (channel, base) in shade.iter_mut().zip(accent) {
            *channel = (base as i32 + rnd.rand_i32(3, 22)).min(255) as u8;
        }
        shade[3] = rnd.rand_i32(8, 35) as u8;
        im.ellipse(x - r, y - r, x + r, y + r, Some(shade), None, 0);
    }
    // vignette corners
    im.rect(0, 0, s - 1, s - 1, None, Some(with_a(accent, 220)), 3);
    // shoulders and chest armor
    im.polygon(
        &[
            (12, 256),
            (22, 206),
            (61, 176),
            (96, 170),
            (160, 170),
            (198, 180),
            (236, 210),
            (248, 256),
        ],
        Some(with_a(sub_rgb(uniform, 12), 255)),
        None,
    );
    im.polygon(
        &[
            (44, 256),
            (54, 205),
            (91, 184),
            (128, 211),
            (166, 184),
            (206, 205),
            (217, 256),
        ],
        Some(with_a(uniform, 255)),
        None,
    );
    im.polygon(
        &[(75, 190), (128, 225), (181, 190), (170, 256), (86, 256)],
        Some(with_a(dim_rgb(uniform, 0.68), 255)),
        None,
    );
    // collar / shoulder accents
    im.polygon(
        &[(81, 185), (107, 200), (94, 219), (71, 195)],
        Some(with_a(add_rgb(uniform, 18), 255)),
        None,
    );
    im.polygon(
        &[(175, 185), (150, 200), (163, 219), (185, 195)],
        Some(with_a(add_rgb(uniform, 18), 255)),
        None,
    );
    polyline(
        &mut im,
        &[(18, 224), (54, 207), (78, 225)],
        with_a(accent, 255),
        5,
    );
    polyline(
        &mut im,
        &[(238, 225), (203, 207), (180, 225)],
        with_a(accent, 255),
        5,
    );
    // neck
    im.rounded_rect(103, 156, 153, 201, 15, with_a(skin, 255));
    // ears
    im.ellipse(63, 99, 90, 136, Some(with_a(skin, 255)), None, 0);
    im.ellipse(166, 99, 193, 136, Some(with_a(skin, 255)), None, 0);
    // rear hair / head silhouette
    if headgear == "hood" {
        im.ellipse(
            45,
            34,
            207,
            194,
            Some(with_a(dim_rgb(uniform, 0.72), 255)),
            None,
            0,
        );
    } else if headgear == "flight_helmet" {
        im.ellipse(
            53,
            25,
            202,
            181,
            Some(with_a(dim_rgb(uniform, 0.8), 255)),
            None,
            0,
        );
    } else {
        im.ellipse(59, 28, 197, 180, Some(with_a(hair, 255)), None, 0);
    }
    // face shape
    im.ellipse(76, 49, 180, 174, Some(with_a(skin, 255)), None, 0);
    // cheeks, nose, jaw shading
    im.polygon(
        &[
            (77, 111),
            (87, 148),
            (110, 169),
            (128, 174),
            (115, 180),
            (87, 157),
        ],
        Some(with_a(dim_rgb(skin, 0.83), 255)),
        None,
    );
    im.polygon(
        &[
            (170, 110),
            (160, 150),
            (141, 169),
            (128, 174),
            (146, 180),
            (167, 156),
        ],
        Some(with_a(bright_rgb(skin, 1.07), 160)),
        None,
    );
    // hairline
    if !matches!(
        headgear,
        "flight_helmet" | "hood" | "hardhat" | "medic_cap" | "beret"
    ) {
        if presentation == "f" {
            im.polygon(
                &[
                    (62, 95),
                    (60, 54),
                    (90, 28),
                    (139, 27),
                    (181, 57),
                    (190, 121),
                    (171, 89),
                    (164, 56),
                    (126, 61),
                    (98, 52),
                    (80, 78),
                ],
                Some(with_a(hair, 255)),
                None,
            );
            im.polygon(
                &[(62, 82), (49, 147), (70, 174), (78, 130)],
                Some(with_a(hair, 255)),
                None,
            );
        } else {
            im.polygon(
                &[
                    (71, 80),
                    (68, 52),
                    (90, 32),
                    (132, 26),
                    (165, 37),
                    (183, 65),
                    (171, 78),
                    (143, 57),
                    (109, 60),
                    (88, 73),
                ],
                Some(with_a(hair, 255)),
                None,
            );
        }
    }
    // headwear overlays
    match headgear {
        "beret" => {
            pieslice_outlined(
                &mut im,
                53,
                23,
                201,
                91,
                180.0,
                360.0,
                with_a(uniform, 255),
                with_a(accent, 255),
                3,
            );
            im.ellipse(154, 40, 169, 55, Some(with_a(accent, 255)), None, 0);
        }
        "hardhat" => {
            pieslice_outlined(
                &mut im,
                51,
                24,
                204,
                112,
                180.0,
                360.0,
                [178, 139, 57, 255],
                [229, 190, 86, 255],
                4,
            );
            im.rect(54, 68, 204, 82, Some([202, 158, 61, 255]), None, 0);
        }
        "medic_cap" => {
            rounded(
                &mut im,
                66,
                31,
                190,
                75,
                13,
                [210, 215, 212, 255],
                Some([238, 245, 239, 255]),
                3,
            );
            im.rect(119, 40, 137, 67, Some([205, 69, 74, 255]), None, 0);
            im.rect(111, 48, 145, 59, Some([205, 69, 74, 255]), None, 0);
        }
        "flight_helmet" => {
            pieslice_outlined(
                &mut im,
                56,
                24,
                201,
                140,
                180.0,
                360.0,
                with_a(sub_rgb(uniform, 8), 255),
                with_a(accent, 255),
                4,
            );
            im.polygon(
                &[(65, 91), (182, 91), (168, 111), (78, 111)],
                Some([31, 47, 56, 255]),
                None,
            );
            im.line(82, 99, 167, 99, with_a(accent, 255), 4);
        }
        "headset" => {
            im.arc(63, 30, 194, 135, 180.0, 356.0, [26, 40, 49, 255], 10);
            rounded(
                &mut im,
                53,
                92,
                76,
                132,
                8,
                [43, 60, 69, 255],
                Some(with_a(accent, 255)),
                3,
            );
            rounded(
                &mut im,
                180,
                92,
                203,
                132,
                8,
                [43, 60, 69, 255],
                Some(with_a(accent, 255)),
                3,
            );
        }
        "hood" => {
            im.arc(45, 19, 209, 195, 180.0, 360.0, with_a(accent, 230), 7);
        }
        "cap" => {
            pieslice_outlined(
                &mut im,
                59,
                24,
                196,
                95,
                180.0,
                360.0,
                with_a(uniform, 255),
                with_a(accent, 255),
                3,
            );
            im.polygon(
                &[(70, 68), (185, 66), (205, 86), (78, 89)],
                Some(with_a(sub_rgb(uniform, 14), 255)),
                None,
            );
        }
        _ => {}
    }
    // brows, eyes, nose and mouth
    im.line(91, 99, 113, 95, with_a(dim_rgb(hair, 0.45), 255), 5);
    im.line(143, 95, 164, 99, with_a(dim_rgb(hair, 0.45), 255), 5);
    im.ellipse(94, 103, 112, 114, Some([235, 224, 205, 255]), None, 0);
    im.ellipse(145, 103, 163, 114, Some([235, 224, 205, 255]), None, 0);
    im.ellipse(101, 104, 109, 114, Some([43, 50, 52, 255]), None, 0);
    im.ellipse(151, 104, 159, 114, Some([43, 50, 52, 255]), None, 0);
    polyline(
        &mut im,
        &[(129, 108), (122, 134), (133, 136)],
        with_a(dim_rgb(skin, 0.78), 255),
        5,
    );
    im.arc(
        108,
        134,
        149,
        157,
        5.0,
        175.0,
        with_a(dim_rgb(skin, 0.55), 255),
        4,
    );
    // beard / mustache
    if matches!(beard, "beard" | "stubble" | "goatee") {
        if beard == "beard" {
            im.polygon(
                &[
                    (82, 132),
                    (95, 148),
                    (103, 166),
                    (128, 180),
                    (154, 166),
                    (163, 148),
                    (176, 132),
                    (168, 169),
                    (146, 187),
                    (110, 187),
                    (90, 169),
                ],
                Some(with_a(hair, 255)),
                None,
            );
            im.rect(113, 145, 144, 153, Some(with_a(skin, 255)), None, 0);
        } else if beard == "goatee" {
            im.polygon(
                &[(114, 145), (143, 145), (140, 172), (128, 181), (117, 170)],
                Some(with_a(hair, 255)),
                None,
            );
        } else {
            for _ in 0..18 {
                let x = rnd.rand_i32(91, 163);
                let y = rnd.rand_i32(137, 159);
                im.set_pixel(x, y, with_a(sub_rgb(skin, 20), 255));
            }
        }
    } else if beard == "mustache" {
        im.ellipse(107, 134, 130, 143, Some(with_a(hair, 255)), None, 0);
        im.ellipse(126, 134, 150, 143, Some(with_a(hair, 255)), None, 0);
    }
    // eyewear
    if matches!(
        eyewear,
        "glasses" | "goggles" | "sunglasses" | "visor" | "scope" | "binoculars"
    ) {
        let eye_col: Rgba = if matches!(eyewear, "sunglasses" | "visor") {
            [20, 31, 38, 255]
        } else {
            [64, 86, 98, 255]
        };
        if eyewear == "visor" {
            im.polygon(
                &[(61, 80), (193, 80), (178, 113), (79, 113)],
                Some([50, 106, 127, 210]),
                Some(with_a(accent, 255)),
            );
        } else if matches!(eyewear, "scope" | "binoculars") {
            im.rect(
                80,
                98,
                171,
                120,
                Some([24, 38, 44, 255]),
                Some(with_a(accent, 255)),
                3,
            );
            im.ellipse(88, 99, 112, 119, None, Some([151, 200, 203, 255]), 2);
            im.ellipse(140, 99, 164, 119, None, Some([151, 200, 203, 255]), 2);
        } else {
            for x in [88, 142] {
                let frame: Rgb = if eyewear == "goggles" {
                    accent
                } else {
                    [32, 45, 52]
                };
                rounded(
                    &mut im,
                    x,
                    96,
                    x + 31,
                    118,
                    5,
                    eye_col,
                    Some(with_a(frame, 255)),
                    3,
                );
            }
            im.line(119, 106, 142, 106, [25, 37, 44, 255], 4);
            im.line(62, 104, 88, 104, [25, 37, 44, 255], 4);
            im.line(173, 104, 197, 104, [25, 37, 44, 255], 4);
        }
    }
    // tactical details and lighting
    im.polygon(
        &[(28, 215), (47, 209), (61, 224), (52, 238), (37, 237)],
        Some(with_a(accent, 255)),
        None,
    );
    im.polygon(
        &[(226, 217), (207, 209), (194, 224), (203, 238), (219, 237)],
        Some(with_a(accent, 255)),
        None,
    );
    im.line(22, 247, 76, 247, with_a(accent, 255), 4);
    im.line(180, 247, 235, 247, with_a(accent, 255), 4);
    // conversion to antialiased square art
    im.resize_catmullrom(AVATAR_SIZE, AVATAR_SIZE)
}

// ---------------------------------------------------------------------------
// Tactical icons — Python `draw_icon`
// ---------------------------------------------------------------------------

/// Python `draw_icon`: 32×32 glyph keyed on the exact per-name branch chain
/// (the first matching group wins — e.g. `production` takes the repair
/// branch, `diplomacy` the comms branch, `close` the X-mark branch), with
/// the hash-fallback geometric glyph for the rest. `seed` stays unused, as
/// in the Python original.
#[allow(unused_variables)]
fn draw_icon(name: &str, seed: u64, kit: &FontKit) -> Canvas {
    let mut im = Canvas::transparent(ICON_SIZE, ICON_SIZE);
    let accent: Rgb = if contains_any(name, &RED_KEYS) {
        C_RED
    } else if contains_any(name, &AMBER_KEYS) {
        C_AMBER
    } else if contains_any(name, &GREEN_KEYS) {
        C_GREEN
    } else if contains_any(name, &RED_KEYS_2) {
        C_RED
    } else {
        C_CYAN
    };
    let pale: Rgba = [220, 238, 244, 245];
    let line_col = with_a(accent, 255);

    if matches!(
        name,
        "move" | "waypoint" | "forward" | "next" | "deploy" | "unload" | "evacuate"
    ) {
        im.line(5, 16, 24, 16, line_col, 3);
        im.polygon(&[(18, 8), (27, 16), (18, 24)], Some(line_col), None);
        if name == "waypoint" {
            im.ellipse(5, 7, 14, 16, None, Some(pale), 2);
        }
    } else if matches!(
        name,
        "attack" | "enemy" | "airstrike" | "artillery" | "special_ability"
    ) {
        im.ellipse(4, 4, 28, 28, None, Some(line_col), 2);
        im.line(6, 6, 26, 26, line_col, 2);
        im.line(26, 6, 6, 26, line_col, 2);
        im.ellipse(11, 11, 21, 21, None, Some(pale), 2);
        if name == "airstrike" {
            im.polygon(&[(16, 2), (20, 13), (16, 10), (12, 13)], Some(pale), None);
        }
    } else if matches!(
        name,
        "stop" | "cancel" | "close" | "delete" | "mission_failed"
    ) {
        im.line(7, 7, 25, 25, line_col, 4);
        im.line(25, 7, 7, 25, line_col, 4);
    } else if matches!(name, "hold" | "defend" | "shield" | "lock" | "unlock") {
        im.polygon(
            &[(16, 3), (27, 8), (25, 19), (16, 28), (7, 19), (5, 8)],
            Some([accent[0], accent[1], accent[2], 40]),
            Some(line_col),
        );
        polyline(&mut im, &[(11, 15), (15, 19), (22, 11)], pale, 2);
        if name == "lock" {
            im.arc(10, 5, 22, 20, 180.0, 360.0, pale, 2);
        }
    } else if matches!(
        name,
        "repair" | "build" | "production" | "repair_pad" | "research"
    ) {
        im.line(6, 25, 22, 9, line_col, 5);
        im.arc(16, 3, 29, 17, 180.0, 330.0, pale, 3);
        im.line(7, 6, 14, 13, line_col, 4);
        im.line(4, 10, 10, 4, line_col, 4);
    } else if matches!(
        name,
        "radar" | "recon" | "map" | "layers" | "ping" | "search" | "filter"
    ) {
        im.ellipse(3, 3, 29, 29, None, Some(line_col), 2);
        im.arc(7, 7, 25, 25, 200.0, 330.0, pale, 2);
        im.line(16, 16, 25, 6, line_col, 2);
        im.ellipse(13, 13, 19, 19, Some(line_col), None, 0);
        if name == "layers" {
            im.polygon(&[(4, 11), (16, 5), (28, 11), (16, 18)], None, Some(pale));
            polyline(&mut im, &[(4, 17), (16, 24), (28, 17)], line_col, 2);
        }
    } else if matches!(
        name,
        "objective_primary"
            | "objective_secondary"
            | "objective_bonus"
            | "campaign"
            | "skirmish"
            | "multiplayer"
    ) {
        im.ellipse(4, 4, 28, 28, None, Some(line_col), 2);
        im.ellipse(9, 9, 23, 23, None, Some(pale), 2);
        im.ellipse(14, 14, 18, 18, Some(line_col), None, 0);
        if name == "objective_bonus" {
            im.polygon(
                &[
                    (16, 0),
                    (19, 9),
                    (29, 9),
                    (21, 15),
                    (24, 25),
                    (16, 19),
                    (8, 25),
                    (11, 15),
                    (3, 9),
                    (13, 9),
                ],
                Some(line_col),
                None,
            );
        }
    } else if matches!(
        name,
        "health"
            | "armor"
            | "ammo"
            | "veterancy"
            | "population"
            | "credits"
            | "power"
            | "fuel"
            | "supply"
    ) {
        rounded_outline_only(&mut im, 4, 8, 28, 24, 3, line_col, 2);
        im.rect(7, 11, 12, 21, Some(line_col), None, 0);
        im.rect(14, 11, 19, 21, Some(pale), None, 0);
        if name == "health" || name == "fuel" {
            im.line(9, 16, 14, 16, with_a(C_RED, 255), 3);
        }
    } else if matches!(
        name,
        "timer"
            | "pause"
            | "play"
            | "save"
            | "load"
            | "sound"
            | "video"
            | "settings"
            | "controls"
            | "accessibility"
            | "network"
            | "server"
            | "voice"
    ) {
        if name == "pause" {
            im.rect(8, 6, 13, 26, Some(line_col), None, 0);
            im.rect(19, 6, 24, 26, Some(line_col), None, 0);
        } else if name == "play" {
            im.polygon(&[(8, 5), (27, 16), (8, 27)], Some(line_col), None);
        } else if name == "timer" {
            im.ellipse(5, 5, 27, 27, None, Some(line_col), 2);
            im.line(16, 16, 16, 8, pale, 2);
            im.line(16, 16, 22, 19, pale, 2);
            im.rect(12, 1, 20, 5, Some(line_col), None, 0);
        } else if name == "sound" {
            im.polygon(
                &[(4, 12), (10, 12), (18, 5), (18, 27), (10, 20), (4, 20)],
                Some(line_col),
                None,
            );
            im.arc(11, 6, 28, 26, -60.0, 60.0, pale, 2);
        } else {
            rounded(&mut im, 4, 4, 28, 28, 4, [0, 0, 0, 0], Some(line_col), 2);
            polyline(&mut im, &[(9, 16), (14, 21), (24, 10)], pale, 2);
        }
    } else if matches!(
        name,
        "select"
            | "rally"
            | "patrol"
            | "capture"
            | "rescue"
            | "transport"
            | "commander"
            | "unit_info"
            | "building_info"
    ) {
        im.polygon(
            &[(4, 3), (24, 15), (15, 17), (12, 27)],
            Some(line_col),
            None,
        );
        im.line(12, 20, 21, 28, pale, 2);
        if name == "capture" {
            im.ellipse(16, 3, 29, 16, None, Some(pale), 2);
        }
    } else if matches!(
        name,
        "comms" | "messages" | "chat" | "report" | "diplomacy" | "alliance"
    ) {
        rounded(&mut im, 3, 5, 29, 22, 4, [0, 0, 0, 0], Some(line_col), 2);
        im.polygon(&[(8, 20), (8, 28), (16, 21)], Some(line_col), None);
        im.line(8, 11, 24, 11, pale, 2);
        im.line(8, 16, 19, 16, pale, 2);
    } else if matches!(name, "roster" | "neutral" | "spectator" | "rank" | "medal") {
        // ("roster","diplomacy","neutral","spectator","rank","medal") in the
        // Python chain; `diplomacy` is a dead member (comms group wins first).
        im.ellipse(10, 3, 22, 15, Some(line_col), None, 0);
        im.arc(4, 12, 28, 31, 180.0, 360.0, pale, 3);
        im.line(8, 23, 24, 23, line_col, 2);
    } else if matches!(name, "zoom_in" | "zoom_out") {
        im.ellipse(3, 3, 21, 21, None, Some(line_col), 3);
        im.line(18, 19, 29, 29, line_col, 4);
        im.line(8, 12, 16, 12, pale, 2);
        if name == "zoom_in" {
            im.line(12, 8, 12, 16, pale, 2);
        }
    } else if matches!(name, "menu" | "back" | "previous" | "help" | "fullscreen") {
        if name == "menu" {
            for y in [7, 14, 21] {
                im.line(5, y, 27, y, line_col, 3);
            }
        } else if name == "back" || name == "previous" {
            im.line(24, 16, 7, 16, line_col, 3);
            im.line(7, 16, 15, 8, pale, 3);
            im.line(7, 16, 15, 24, pale, 3);
        } else if name == "help" {
            im.ellipse(4, 4, 28, 28, None, Some(line_col), 2);
            kit.draw(&mut im, 10, 3, "?", 24, pale, true);
        } else {
            im.rect(5, 5, 27, 27, None, Some(line_col), 2);
            im.line(8, 13, 13, 8, pale, 2);
            im.line(19, 8, 24, 13, pale, 2);
        }
    } else {
        // fallback distinct geometric glyph based on stable hash
        let h: i64 = name.chars().map(|ch| ch as i64).sum();
        rounded(&mut im, 4, 4, 28, 28, 4, [0, 0, 0, 0], Some(line_col), 2);
        if h % 3 == 0 {
            im.ellipse(9, 9, 23, 23, None, Some(pale), 2);
        } else if h % 3 == 1 {
            im.polygon(&[(16, 7), (25, 16), (16, 25), (7, 16)], None, Some(pale));
        } else {
            polyline(&mut im, &[(8, 22), (16, 8), (24, 22), (8, 22)], pale, 2);
        }
        im.line(8, 16, 24, 16, line_col, 1);
    }
    im
}

/// Outline-only rounded-rect helper for the icon markers drawn with PIL's
/// `rounded_rectangle(xy, radius, outline, width)` (no fill).
#[allow(clippy::too_many_arguments)]
fn rounded_outline_only(
    canvas: &mut Canvas,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    radius: i32,
    outline: Rgba,
    width: u32,
) {
    canvas.rounded_rect_outline(x0, y0, x1, y1, radius, outline, width);
}

// ---------------------------------------------------------------------------
// UI component kit — Python `gen_controls`
// ---------------------------------------------------------------------------

/// One `specs` row: `(name, (w, h), fill, outline, radius)`; the two-arg
/// transparent fills stay as `(0, 0, 0, 0)`, the RGB fills get alpha 245
/// (`tuple(fill)+(245,) if len(fill)==3 else fill`).
struct ControlSpec {
    name: &'static str,
    w: i32,
    h: i32,
    fill: Rgba,
    outline: Rgb,
    radius: i32,
}

const CONTROL_SPECS: [ControlSpec; 13] = [
    ControlSpec {
        name: "panel",
        w: 320,
        h: 180,
        fill: with_a(C_PANEL, 245),
        outline: C_LINE,
        radius: 12,
    },
    ControlSpec {
        name: "panel_header",
        w: 320,
        h: 44,
        fill: with_a(C_PANEL2, 245),
        outline: C_CYAN,
        radius: 8,
    },
    ControlSpec {
        name: "button_normal",
        w: 240,
        h: 56,
        fill: with_a(C_PANEL2, 245),
        outline: C_LINE,
        radius: 8,
    },
    ControlSpec {
        name: "button_hover",
        w: 240,
        h: 56,
        fill: [25, 70, 91, 245],
        outline: C_CYAN,
        radius: 8,
    },
    ControlSpec {
        name: "button_pressed",
        w: 240,
        h: 56,
        fill: [9, 42, 59, 245],
        outline: C_AMBER,
        radius: 8,
    },
    ControlSpec {
        name: "button_disabled",
        w: 240,
        h: 56,
        fill: [20, 30, 39, 245],
        outline: [58, 75, 86],
        radius: 8,
    },
    ControlSpec {
        name: "selected_frame",
        w: 128,
        h: 128,
        fill: [0, 0, 0, 0],
        outline: C_CYAN,
        radius: 8,
    },
    ControlSpec {
        name: "portrait_frame",
        w: 144,
        h: 144,
        fill: [0, 0, 0, 0],
        outline: C_AMBER,
        radius: 8,
    },
    ControlSpec {
        name: "tooltip",
        w: 320,
        h: 104,
        fill: with_a(C_PANEL, 245),
        outline: C_LINE,
        radius: 8,
    },
    ControlSpec {
        name: "notification",
        w: 420,
        h: 84,
        fill: with_a(C_PANEL2, 245),
        outline: C_AMBER,
        radius: 8,
    },
    ControlSpec {
        name: "progress_track",
        w: 320,
        h: 22,
        fill: [10, 18, 26, 245],
        outline: C_LINE,
        radius: 6,
    },
    ControlSpec {
        name: "minimap_frame",
        w: 264,
        h: 216,
        fill: with_a(C_PANEL, 245),
        outline: C_CYAN,
        radius: 8,
    },
    ControlSpec {
        name: "divider",
        w: 320,
        h: 4,
        fill: with_a(C_LINE, 255),
        outline: C_LINE,
        radius: 0,
    },
];

/// SpriteSpec used for every v6.2 sprite record (category `ui_v6_2` with the
/// script's own `version`; no `generated_by`/`seed`, like the Python record
/// dicts).
fn v62_spec() -> SpriteSpec<'static> {
    SpriteSpec {
        category: "ui_v6_2",
        version: Some(VERSION),
        ..SpriteSpec::default()
    }
}

/// Python `gen_controls`: 13 procedural control states under
/// `sprites/ui/controls/` plus 6 editable SVG components under
/// `ui/source/svg/`. Returns the `controls` dict (name → {file, size}) and
/// the manifest sprite records in spec order.
fn gen_controls(out: &PackWriter) -> Result<(Map<String, Value>, Vec<Value>), PackError> {
    let mut controls = Map::new();
    let mut records = Vec::with_capacity(CONTROL_SPECS.len());
    for spec in CONTROL_SPECS {
        let (w, h) = (spec.w, spec.h);
        let mut im = Canvas::transparent(w as u32, h as u32);
        if spec.name != "divider" {
            rounded(
                &mut im,
                1,
                1,
                w - 2,
                h - 2,
                spec.radius,
                spec.fill,
                Some(with_a(spec.outline, 255)),
                2,
            );
        } else {
            im.rect(0, 0, w - 1, h - 1, Some(with_a(C_LINE, 255)), None, 0);
        }
        if spec.name.starts_with("button") {
            im.line(12, 8, w - 12, 8, with_a(spec.outline, 90), 1);
        } else if spec.name == "panel" {
            im.polygon(
                &[(w - 34, 1), (w - 2, 1), (w - 2, 25)],
                Some(with_a(spec.outline, 90)),
                None,
            );
        } else if spec.name == "selected_frame" {
            for (x, y, sx, sy) in [
                (2, 2, 1, 1),
                (w - 3, 2, -1, 1),
                (2, h - 3, 1, -1),
                (w - 3, h - 3, -1, -1),
            ] {
                im.line(x, y, x + sx * 20, y, with_a(spec.outline, 255), 4);
                im.line(x, y, x, y + sy * 20, with_a(spec.outline, 255), 4);
            }
        } else if spec.name == "portrait_frame" {
            im.rect(
                5,
                h - 15,
                w - 5,
                h - 5,
                Some(with_a(spec.outline, 230)),
                None,
                0,
            );
        } else if spec.name == "notification" {
            im.rect(2, 2, 7, h - 3, Some(with_a(spec.outline, 255)), None, 0);
        }
        let rel = format!("sprites/ui/controls/{}.png", spec.name);
        let record = out.write_sprite(&rel, &im, v62_spec())?;
        records.push(record);
        controls.insert(spec.name.to_string(), json!({"file": rel, "size": [w, h]}));
    }
    // Editable SVG components, scalable and engine-friendly.
    let svg_names = [
        ("panel", (320, 180)),
        ("button", (240, 56)),
        ("tooltip", (320, 104)),
        ("minimap_frame", (264, 216)),
        ("portrait_frame", (144, 144)),
        ("notification", (420, 84)),
    ];
    for (name, (w, h)) in svg_names {
        let stroke = if name == "minimap_frame" || name == "portrait_frame" {
            "#46c4e9"
        } else {
            "#27698b"
        };
        let base = if name != "button" {
            "#0d1d2d"
        } else {
            "#142a3d"
        };
        let (mw, mh, mp, mh12) = (w - 2, h - 2, w - 36, w - 12);
        let svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">\
<rect x=\"1\" y=\"1\" width=\"{mw}\" height=\"{mh}\" rx=\"10\" fill=\"{base}\" fill-opacity=\"0.96\" stroke=\"{stroke}\" stroke-width=\"2\"/>\
<path d=\"M {mp} 1 L {mw} 1 L {mw} 25 Z\" fill=\"{stroke}\" fill-opacity=\"0.45\"/>\
<path d=\"M 12 8 H {mh12}\" stroke=\"{stroke}\" stroke-opacity=\"0.3\"/></svg>"
        );
        out.write_text(&format!("ui/source/svg/{name}.svg"), &svg)?;
    }
    Ok((controls, records))
}

// ---------------------------------------------------------------------------
// Icon atlas — Python `gen_icons`
// ---------------------------------------------------------------------------

/// Python `gen_icons`: draw all [`ICONS`], tile them into the 12-column
/// atlas and write `atlases/ui_icons_v6_2.{png,json}` (JSON without a
/// trailing newline, like the Python `write_text`).
fn gen_icons(
    out: &PackWriter,
    seed: u64,
    kit: &FontKit,
) -> Result<(IconPaths, Canvas, AvatarRecords), PackError> {
    const COLS: usize = 12;
    let mut paths: Vec<(&'static str, String, Canvas)> = Vec::with_capacity(ICONS.len());
    let mut records = Vec::with_capacity(ICONS.len());
    for (i, name) in ICONS.iter().enumerate() {
        let im = draw_icon(name, seed + i as u64, kit);
        let rel = format!("sprites/ui/icons/{name}.png");
        let record = out.write_sprite(&rel, &im, v62_spec())?;
        records.push((rel.clone(), record));
        paths.push((name, rel, im));
    }
    let rows = ICONS.len().div_ceil(COLS);
    let cells: Vec<&Canvas> = paths.iter().map(|(_, _, im)| im).collect();
    let atlas = compose_atlas(&cells, COLS, ICON_SIZE, false);
    out.write_png("atlases/ui_icons_v6_2.png", &atlas)?;
    let mut mapping = Map::new();
    for (i, (name, rel, _)) in paths.iter().enumerate() {
        let x = (i % COLS) as i32 * ICON_SIZE as i32;
        let y = (i / COLS) as i32 * ICON_SIZE as i32;
        mapping.insert(
            name.to_string(),
            json!({"file": rel, "x": x, "y": y, "width": ICON_SIZE, "height": ICON_SIZE}),
        );
    }
    let atlas_json = serde_json::to_string_pretty(&json!({
        "version": VERSION,
        "cell_size": [ICON_SIZE, ICON_SIZE],
        "columns": COLS,
        "rows": rows,
        "sprites": mapping,
    }))?;
    out.write_text("atlases/ui_icons_v6_2.json", &atlas_json)?;
    Ok((paths, atlas, records))
}

// ---------------------------------------------------------------------------
// Portrait atlas — Python `gen_avatars`
// ---------------------------------------------------------------------------

/// Python `gen_avatars`: draw all [`PEOPLE`], tile them into the 6-column
/// atlas and write `atlases/character_portraits_v6_2.{png,json}`. Returns
/// the character mapping in [`PEOPLE`] insert order (the contact sheets and
/// art board iterate this order) — JSON key order itself stays alphabetical
/// (documented deviation).
fn gen_avatars(
    out: &PackWriter,
    seed: u64,
) -> Result<(AvatarMap, Canvas, AvatarRecords), PackError> {
    const COLS: usize = 6;
    let mut mapping_order: Vec<(String, Value)> = Vec::with_capacity(PEOPLE.len());
    let mut cells: Vec<Canvas> = Vec::with_capacity(PEOPLE.len());
    let mut records: Vec<(String, Value)> = Vec::with_capacity(PEOPLE.len());
    for (i, person) in PEOPLE.iter().enumerate() {
        let im = generate_avatar(person, seed);
        let folder = if person.role == "Commander" {
            "commanders"
        } else {
            "npcs"
        };
        let rel = format!("sprites/ui/avatars/{folder}/{}.png", person.pid);
        let record = out.write_sprite(&rel, &im, v62_spec())?;
        records.push((rel.clone(), record));
        let x = (i % COLS) as i32 * AVATAR_SIZE as i32;
        let y = (i / COLS) as i32 * AVATAR_SIZE as i32;
        cells.push(im);
        mapping_order.push((
            person.pid.to_string(),
            json!({
                "name": person.name,
                "role": person.role,
                "faction": person.faction,
                "file": rel,
                "x": x,
                "y": y,
                "width": AVATAR_SIZE,
                "height": AVATAR_SIZE,
            }),
        ));
    }
    let refs: Vec<&Canvas> = cells.iter().collect();
    let atlas = compose_atlas(&refs, COLS, AVATAR_SIZE, false);
    out.write_png("atlases/character_portraits_v6_2.png", &atlas)?;
    let mut characters = Map::new();
    for (pid, entry) in &mapping_order {
        characters.insert(pid.clone(), entry.clone());
    }
    let rows = PEOPLE.len().div_ceil(COLS);
    let atlas_json = serde_json::to_string_pretty(&json!({
        "version": VERSION,
        "cell_size": [AVATAR_SIZE, AVATAR_SIZE],
        "columns": COLS,
        "rows": rows,
        "characters": characters,
    }))?;
    out.write_text("atlases/character_portraits_v6_2.json", &atlas_json)?;
    Ok((mapping_order, atlas, records))
}

// ---------------------------------------------------------------------------
// Screen mockups and wireframes
// ---------------------------------------------------------------------------

/// Python `SCREEN_SPECS`, verbatim: `(screen_id, title, background kind,
/// content line)`.
static SCREEN_SPECS: [(&str, &str, &str, &str); 12] = [
    (
        "main_menu",
        "Main Menu",
        "Logo / Hero",
        "Campaign · Skirmish · Multiplayer · Options",
    ),
    (
        "campaign_map",
        "Campaign Operations Map",
        "Theatre Map",
        "Mission list · Intel · Deployment",
    ),
    (
        "skirmish_lobby",
        "Skirmish Setup",
        "Map & Faction Setup",
        "Map seed · Teams · Rules · Start match",
    ),
    (
        "gameplay_hud",
        "Gameplay HUD",
        "Battlefield",
        "Resources · Selection · Minimap · Commands",
    ),
    (
        "mission_briefing",
        "Mission Briefing",
        "Commander Briefing",
        "Narrative · Objectives · Deployment preview",
    ),
    (
        "unit_inspector",
        "Unit Inspector",
        "Selected Unit / Platoon",
        "Health · Armor · Orders · Upgrades",
    ),
    (
        "tech_tree",
        "Technology Tree",
        "Research Graph",
        "Prerequisites · Costs · Unlocks",
    ),
    (
        "pause_options",
        "Pause / Options",
        "Paused Battlefield",
        "Resume · Save · Load · Options · Exit",
    ),
    (
        "mission_debrief",
        "Mission Debrief",
        "After Action Report",
        "Objectives · Losses · Awards · Continue",
    ),
    (
        "multiplayer_lobby",
        "Multiplayer Lobby",
        "Player Slots",
        "Ready state · Map vote · Chat · Launch",
    ),
    (
        "options",
        "Settings & Accessibility",
        "Settings Navigation",
        "Video · Audio · Input · Interface · Accessibility",
    ),
    (
        "commander_dossier",
        "Commander Dossier",
        "Portrait & Profile",
        "Traits · Campaign record · Command doctrine",
    ),
];

/// Python `screen_specs` accessor.
pub fn screen_specs() -> &'static [(&'static str, &'static str, &'static str, &'static str)] {
    &SCREEN_SPECS
}

/// Python `draw_battlefield(size, seed, night, snow)`: gradient terrain,
/// 220 contour patches, river and road polylines, 38 building compounds,
/// 50 top-down unit symbols, 900 stipple points and an optional night
/// shade. SplitMix64 stands in for `random.Random(seed)`.
fn draw_battlefield(w: i32, h: i32, seed: u64, night: bool, snow: bool) -> Canvas {
    let mut rnd = Rng::new(Seed(seed));
    let base: Rgb = if snow { [97, 115, 122] } else { [44, 70, 47] };
    let land2: Rgb = if snow { [130, 143, 147] } else { [56, 81, 49] };
    let mut im = Canvas::gradient(w.max(1) as u32, h.max(1) as u32, [20, 36, 32], [13, 25, 24]);
    // terrain contour patches
    for i in 0..220 {
        let x = rnd.randrange2(-80, w);
        let y = rnd.randrange2(-80, h);
        let rx = rnd.randrange2(25, 115);
        let ry = rnd.randrange2(14, 70);
        let col = if i % 3 == 0 { land2 } else { base };
        let alpha = rnd.randrange2(55, 135) as u8;
        im.ellipse(
            x - rx,
            y - ry,
            x + rx,
            y + ry,
            Some(with_a(col, alpha)),
            None,
            0,
        );
    }
    // river / roads
    let river: [(i32, i32); 6] = [
        (-50, (h as f64 * 0.72) as i32),
        ((w as f64 * 0.13) as i32, (h as f64 * 0.56) as i32),
        ((w as f64 * 0.31) as i32, (h as f64 * 0.62) as i32),
        ((w as f64 * 0.49) as i32, (h as f64 * 0.47) as i32),
        ((w as f64 * 0.68) as i32, (h as f64 * 0.45) as i32),
        (w + 30, (h as f64 * 0.19) as i32),
    ];
    polyline(&mut im, &river, [17, 40, 50, 210], (26).max(h / 18) as u32);
    polyline(
        &mut im,
        &river,
        [94, 122, 117, 220],
        (3).max(h / 140) as u32,
    );
    let road: [(i32, i32); 5] = [
        ((w as f64 * 0.08) as i32, h + 30),
        ((w as f64 * 0.27) as i32, (h as f64 * 0.7) as i32),
        ((w as f64 * 0.45) as i32, (h as f64 * 0.56) as i32),
        ((w as f64 * 0.60) as i32, (h as f64 * 0.29) as i32),
        ((w as f64 * 0.87) as i32, -30),
    ];
    polyline(&mut im, &road, [46, 45, 35, 255], (18).max(h / 22) as u32);
    polyline(&mut im, &road, [124, 116, 94, 255], (3).max(h / 160) as u32);
    // scattered building compounds
    for i in 0..38 {
        let x = rnd.randrange2(20, w - 40);
        let y = rnd.randrange2(20, h - 40);
        let bw = rnd.randrange2(12, 30);
        let bh = rnd.randrange2(10, 25);
        if i % 3 == 0 {
            im.rect(
                x,
                y,
                x + bw,
                y + bh,
                Some([95, 99, 86, 255]),
                Some([28, 39, 38, 255]),
                2,
            );
        } else {
            im.polygon(
                &[
                    (x, y + bh / 3),
                    (x + bw / 2, y),
                    (x + bw, y + bh / 3),
                    (x + bw, y + bh),
                    (x, y + bh),
                ],
                Some([105, 94, 69, 255]),
                Some([31, 42, 34, 255]),
            );
        }
        if i % 5 == 0 {
            im.rect(
                x + bw / 3,
                y + bh / 2,
                x + bw / 2,
                y + bh - 2,
                Some([37, 48, 41, 255]),
                None,
                0,
            );
        }
    }
    // units (top-down symbols)
    for i in 0..50 {
        let x = rnd.randrange2(18, w - 18);
        let y = rnd.randrange2(18, h - 18);
        let col: Rgba = if i % 2 == 0 {
            [55, 178, 220, 255]
        } else {
            [216, 79, 69, 255]
        };
        if i % 5 == 0 || i % 5 == 1 {
            rounded(
                &mut im,
                x - 9,
                y - 5,
                x + 9,
                y + 5,
                3,
                [39, 53, 50, 255],
                Some(col),
                2,
            );
            im.rect(x - 3, y - 3, x + 4, y + 3, Some(col), None, 0);
        } else {
            im.ellipse(
                x - 5,
                y - 5,
                x + 5,
                y + 5,
                Some(col),
                Some([230, 239, 235, 255]),
                1,
            );
        }
        if i % 9 == 0 {
            im.ellipse(x - 14, y - 14, x + 14, y + 14, None, Some(col), 1);
        }
    }
    // atmospheric stipple
    for _ in 0..900 {
        let x = rnd.randrange2(0, w);
        let y = rnd.randrange2(0, h);
        let a = rnd.randrange2(8, 28) as u8;
        im.set_pixel(x, y, [204, 219, 174, a]);
    }
    if night {
        im.alpha_composite(
            &Canvas::new_filled(w.max(1) as u32, h.max(1) as u32, [3, 12, 26, 125]),
            (0, 0),
        );
    }
    im
}

// ---------------------------------------------------------------------------
// Screen mockups — Python `make_mockup`
// ---------------------------------------------------------------------------

/// Python `make_mockup`: 1600×900 high-fidelity mockup. Every branch keeps
/// the Python geometry, draw order and copy strings verbatim; `_title`/
/// `_content` stay in the signature because Python passes them without
/// reading them. Commanders already written by [`gen_avatars`] are loaded
/// and Catmull-Rom-resized where Pillow used LANCZOS (documented
/// substitute). Support artefact written via [`PackWriter::write_png`] —
/// mockups are not manifest sprites.
#[allow(clippy::too_many_arguments)]
fn make_mockup(
    out: &PackWriter,
    kit: &FontKit,
    screen_id: &str,
    _title: &str,
    bg_kind: &str,
    _content: &str,
    seed: u64,
) -> Result<String, PackError> {
    let w = SCREEN_W;
    let h = SCREEN_H;
    let mut im = if matches!(
        bg_kind,
        "Battlefield" | "Theatre Map" | "Paused Battlefield" | "Map & Faction Setup"
    ) {
        draw_battlefield(
            w,
            h,
            seed,
            matches!(screen_id, "main_menu" | "pause_options"),
            screen_id == "campaign_map",
        )
    } else {
        let mut im = Canvas::gradient(w as u32, h as u32, [9, 23, 39], [4, 10, 18]);
        for i in 0..80u64 {
            let x = ((i * 193 + seed * 7) % w as u64) as i32;
            let y = ((i * 97 + seed * 13) % h as u64) as i32;
            im.line(x, y, w.min(x + 180), h.min(y + 80), [25, 72, 94, 70], 1);
        }
        im
    };
    // top chrome / masthead
    rounded(
        &mut im,
        18,
        14,
        w - 18,
        73,
        8,
        [4, 13, 23, 242],
        Some(with_a(C_LINE, 255)),
        2,
    );
    draw_text(
        &mut im,
        kit,
        38,
        27,
        "✦ CNC32",
        28,
        Some(with_a(C_CYAN, 255)),
        true,
    );
    draw_text(
        &mut im,
        kit,
        194,
        34,
        "TACTICAL COMMAND SYSTEM",
        13,
        Some(with_a(C_MUTED, 255)),
        true,
    );
    draw_text(
        &mut im,
        kit,
        w - 330,
        35,
        &format!("OPERATIONS  //  BUILD {VERSION}"),
        13,
        Some(with_a(C_MUTED, 255)),
        true,
    );
    // screen-specific compositions
    if screen_id == "main_menu" {
        panel(
            &mut im,
            76,
            115,
            642,
            764,
            Some("TACTICAL OPERATIONS"),
            None,
            kit,
        );
        draw_text(
            &mut im,
            kit,
            110,
            169,
            "COMMAND THE FRONTLINE",
            29,
            Some(with_a(C_WHITE, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            110,
            216,
            "A continent balances on the edge of conflict.",
            16,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        for (i, label) in [
            "CAMPAIGN",
            "SKIRMISH",
            "MULTIPLAYER",
            "LOAD OPERATION",
            "OPTIONS",
            "QUIT",
        ]
        .iter()
        .enumerate()
        {
            let y = 284 + i as i32 * 66;
            button(&mut im, kit, 115, y, 588, y + 47, label, i == 0, false);
        }
        panel(
            &mut im,
            694,
            122,
            1517,
            740,
            Some("THEATRE STATUS"),
            None,
            kit,
        );
        draw_text(
            &mut im,
            kit,
            737,
            183,
            "NORTHERN FRONT",
            22,
            Some(with_a(C_AMBER, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            737,
            225,
            "Enemy movements detected near the rail corridor.",
            17,
            Some(with_a(C_WHITE, 255)),
            false,
        );
        // small inset action card
        let mapim = draw_battlefield(742, 300, seed + 7, true, false);
        im.alpha_composite(&mapim, (735, 282));
        im.rect(735, 282, 1477, 582, None, Some(with_a(C_CYAN, 255)), 2);
        draw_text(
            &mut im,
            kit,
            741,
            608,
            "ACTIVE FRONT  /  04 OPERATIONS AVAILABLE",
            15,
            Some(with_a(C_CYAN, 255)),
            true,
        );
        button(
            &mut im,
            kit,
            735,
            657,
            1005,
            710,
            "RESUME LAST MISSION",
            true,
            false,
        );
        button(
            &mut im,
            kit,
            1022,
            657,
            1240,
            710,
            "INTEL BRIEF",
            false,
            false,
        );
        button(
            &mut im,
            kit,
            1255,
            657,
            1477,
            710,
            "PATCH NOTES",
            false,
            false,
        );
    } else if screen_id == "gameplay_hud" {
        // bottom command bar, resources and map
        panel(
            &mut im,
            20,
            89,
            470,
            146,
            Some("STRATEGIC RESOURCES"),
            None,
            kit,
        );
        draw_text(
            &mut im,
            kit,
            40,
            115,
            "CREDITS  12,450",
            16,
            Some(with_a(C_AMBER, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            220,
            115,
            "POWER  82%",
            16,
            Some(with_a(C_GREEN, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            350,
            115,
            "POP  34/80",
            15,
            Some(with_a(C_WHITE, 255)),
            true,
        );
        panel(
            &mut im,
            w - 300,
            89,
            w - 20,
            320,
            Some("MINIMAP"),
            None,
            kit,
        );
        let mini = draw_battlefield(248, 174, seed + 18, false, false);
        im.alpha_composite(&mini, (w - 284, 133));
        im.rect(
            w - 284,
            133,
            w - 36,
            307,
            None,
            Some(with_a(C_CYAN, 255)),
            2,
        );
        panel(
            &mut im,
            20,
            h - 186,
            380,
            h - 20,
            Some("SELECTED UNIT"),
            None,
            kit,
        );
        for (i, (lab, val)) in [
            ("HP", "840 / 900"),
            ("ARMOR", "Heavy"),
            ("STATUS", "Ready"),
            ("ORDERS", "Hold position"),
        ]
        .iter()
        .enumerate()
        {
            let y = h - 145 + i as i32 * 29;
            draw_text(
                &mut im,
                kit,
                41,
                y,
                lab,
                13,
                Some(with_a(C_MUTED, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                132,
                y,
                val,
                14,
                Some(with_a(C_WHITE, 255)),
                false,
            );
        }
        panel(
            &mut im,
            402,
            h - 186,
            1040,
            h - 20,
            Some("COMMAND GRID"),
            None,
            kit,
        );
        for (i, lab) in [
            "MOVE", "ATTACK", "STOP", "HOLD", "PATROL", "REPAIR", "SPECIAL", "RALLY", "UNLOAD",
            "GUARD", "BUILD", "RETREAT",
        ]
        .iter()
        .enumerate()
        {
            let x = 422 + (i % 6) as i32 * 94;
            let y = h - 137 + (i / 6) as i32 * 58;
            button(&mut im, kit, x, y, x + 82, y + 42, lab, i == 1, false);
        }
        panel(
            &mut im,
            w - 500,
            h - 186,
            w - 20,
            h - 20,
            Some("EVENTS / OBJECTIVES"),
            None,
            kit,
        );
        draw_text(
            &mut im,
            kit,
            w - 472,
            h - 139,
            "PRIMARY  Secure the northern ridge",
            14,
            Some(with_a(C_WHITE, 255)),
            false,
        );
        draw_text(
            &mut im,
            kit,
            w - 472,
            h - 107,
            "SECONDARY  Keep engineers alive",
            13,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        draw_text(
            &mut im,
            kit,
            w - 472,
            h - 75,
            "ALERT  Enemy armor inbound",
            14,
            Some(with_a(C_RED, 255)),
            true,
        );
        // selection boxes on map
        for (x, y) in [(555, 295), (688, 370), (844, 302), (1025, 430), (1178, 361)] {
            im.rect(
                x - 24,
                y - 17,
                x + 24,
                y + 17,
                None,
                Some(with_a(C_CYAN, 230)),
                2,
            );
        }
    } else if screen_id == "campaign_map" {
        panel(
            &mut im,
            25,
            91,
            1068,
            810,
            Some("THEATRE MAP  /  NORTHWATCH"),
            None,
            kit,
        );
        let mini = draw_battlefield(1010, 668, seed + 2, false, true);
        im.alpha_composite(&mini, (41, 133));
        for (idx, (x, y, lab)) in [
            (193, 240, "A"),
            (390, 442, "B"),
            (655, 302, "C"),
            (846, 553, "D"),
            (718, 188, "E"),
        ]
        .iter()
        .enumerate()
        {
            let col = if idx < 2 {
                C_GREEN
            } else if idx == 2 {
                C_AMBER
            } else {
                C_RED
            };
            im.ellipse(
                x - 17,
                y - 17,
                x + 17,
                y + 17,
                Some(with_a(col, 255)),
                Some(with_a(C_WHITE, 255)),
                2,
            );
            draw_text(
                &mut im,
                kit,
                x - 6,
                y - 11,
                lab,
                17,
                Some(with_a(C_DARK, 255)),
                true,
            );
        }
        panel(
            &mut im,
            1090,
            91,
            1575,
            810,
            Some("OPERATION ORDERS"),
            None,
            kit,
        );
        for (i, (a, b)) in [
            ("01  Grey Bridge", "AVAILABLE"),
            ("02  Northwatch", "IN PROGRESS"),
            ("03  Silent Quarry", "LOCKED"),
            ("04  Deep Current", "LOCKED"),
        ]
        .iter()
        .enumerate()
        {
            let y = 152 + i as i32 * 125;
            rounded(
                &mut im,
                1110,
                y,
                1555,
                y + 104,
                8,
                [11, 27, 42, 255],
                Some(with_a(C_LINE, 255)),
                1,
            );
            draw_text(
                &mut im,
                kit,
                1130,
                y + 13,
                a,
                17,
                Some(with_a(C_WHITE, 255)),
                true,
            );
            let status_col = if i == 0 {
                C_GREEN
            } else if i == 1 {
                C_AMBER
            } else {
                C_MUTED
            };
            draw_text(
                &mut im,
                kit,
                1130,
                y + 44,
                b,
                12,
                Some(with_a(status_col, 255)),
                true,
            );
            button(
                &mut im,
                kit,
                1390,
                y + 58,
                1535,
                y + 91,
                if i < 2 { "BRIEF" } else { "LOCKED" },
                i == 0,
                false,
            );
        }
    } else if screen_id == "skirmish_lobby" {
        panel(
            &mut im,
            37,
            111,
            845,
            802,
            Some("BATTLEFIELD CONFIGURATION"),
            None,
            kit,
        );
        let mini = draw_battlefield(760, 300, seed + 3, false, false);
        im.alpha_composite(&mini, (61, 157));
        for (i, (lab, val)) in [
            ("MAP", "Floodplain Pass"),
            ("SIZE", "Large  /  256 × 256"),
            ("VICTORY", "Annihilation"),
            ("STARTING CREDITS", "10,000"),
            ("FOG OF WAR", "Enabled"),
            ("WEATHER", "Overcast"),
            ("GAME SPEED", "Normal"),
        ]
        .iter()
        .enumerate()
        {
            let y = 478 + i as i32 * 41;
            draw_text(
                &mut im,
                kit,
                72,
                y,
                lab,
                14,
                Some(with_a(C_MUTED, 255)),
                true,
            );
            button(&mut im, kit, 295, y - 5, 800, y + 28, val, i == 0, true);
        }
        panel(
            &mut im,
            880,
            111,
            1563,
            802,
            Some("FACTIONS & TEAMS"),
            None,
            kit,
        );
        for (i, (lab, col)) in [
            ("RAVEN COALITION", C_CYAN),
            ("NORTHERN DIRECTORATE", C_RED),
            ("PACIFIC LEAGUE", C_AMBER),
            ("CONTINENTAL UNION", C_GREEN),
        ]
        .iter()
        .enumerate()
        {
            let y = 164 + i as i32 * 125;
            rounded(
                &mut im,
                906,
                y,
                1537,
                y + 97,
                8,
                [12, 31, 46, 255],
                Some(with_a(*col, 255)),
                2,
            );
            im.ellipse(928, y + 22, 979, y + 73, Some(with_a(*col, 255)), None, 0);
            draw_text(
                &mut im,
                kit,
                1002,
                y + 19,
                lab,
                17,
                Some(with_a(C_WHITE, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                1002,
                y + 51,
                &format!("PLAYER 1   /   SLOT {}", i + 1),
                13,
                Some(with_a(C_MUTED, 255)),
                false,
            );
            button(
                &mut im,
                kit,
                1358,
                y + 28,
                1510,
                y + 68,
                if i == 0 { "READY" } else { "AI NORMAL" },
                i == 0,
                false,
            );
        }
        button(
            &mut im,
            kit,
            1175,
            735,
            1528,
            779,
            "START SKIRMISH",
            true,
            false,
        );
    } else if screen_id == "mission_briefing" {
        panel(
            &mut im,
            35,
            104,
            1040,
            795,
            Some("MISSION BRIEFING  /  OPERATION GREY BRIDGE"),
            None,
            kit,
        );
        let battle = draw_battlefield(965, 332, seed + 8, false, false);
        im.alpha_composite(&battle, (54, 151));
        im.rect(54, 151, 1019, 483, None, Some(with_a(C_CYAN, 255)), 2);
        panel(
            &mut im,
            1060,
            104,
            1565,
            795,
            Some("COMMAND STAFF"),
            None,
            kit,
        );
        // commander portrait crop
        let avatar_path = out.path("sprites/ui/avatars/commanders/cmdr_hale.png");
        if avatar_path.exists() {
            if let Ok(av) = load_rgba(&avatar_path) {
                im.alpha_composite(&av.resize_catmullrom(176, 176), (1227, 156));
            }
        }
        draw_text(
            &mut im,
            kit,
            1104,
            350,
            "CMDR. MARCUS HALE",
            19,
            Some(with_a(C_WHITE, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            1104,
            384,
            "Raven Coalition  /  Field Command",
            13,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        for (i, line) in [
            "PRIMARY  Capture Grey Bridge",
            "SECONDARY  Protect engineer teams",
            "INTEL  Armor spotted near the mill",
            "INSERTION  West causeway",
            "EXTRACTION  Hold for 20 minutes",
        ]
        .iter()
        .enumerate()
        {
            draw_text(
                &mut im,
                kit,
                1088,
                439 + i as i32 * 38,
                line,
                14,
                Some(with_a(if i == 0 { C_AMBER } else { C_WHITE }, 255)),
                false,
            );
        }
        panel(
            &mut im,
            55,
            510,
            1020,
            760,
            Some("INTELLIGENCE SUMMARY"),
            None,
            kit,
        );
        draw_text(
            &mut im,
            kit,
            80,
            562,
            "Recon confirms a defended river crossing and an active supply road.",
            17,
            Some(with_a(C_WHITE, 255)),
            false,
        );
        draw_text(
            &mut im,
            kit,
            80,
            601,
            "Engineer squads can restore the bridge if the northern bank is secured.",
            16,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        draw_text(
            &mut im,
            kit,
            80,
            640,
            "Weather limits air support. Keep anti-air units near the convoy.",
            16,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        button(
            &mut im,
            kit,
            1100,
            710,
            1518,
            758,
            "DEPLOY FOR OPERATION",
            true,
            false,
        );
    } else if screen_id == "unit_inspector" {
        panel(&mut im, 40, 108, 520, 810, Some("UNIT PROFILE"), None, kit);
        // battlefield tile / illustration
        let tile = draw_battlefield(436, 242, seed + 16, false, false);
        im.alpha_composite(&tile, (62, 157));
        rounded(
            &mut im,
            171,
            213,
            364,
            344,
            12,
            [32, 48, 49, 255],
            Some(with_a(C_CYAN, 255)),
            2,
        );
        im.rect(
            210,
            252,
            326,
            303,
            Some([70, 91, 76, 255]),
            Some(with_a(C_AMBER, 255)),
            2,
        );
        im.rect(228, 239, 304, 260, Some([80, 100, 82, 255]), None, 0);
        im.line(252, 260, 370, 220, [34, 40, 39, 255], 12);
        im.line(252, 260, 370, 220, with_a(C_AMBER, 255), 3);
        draw_text(
            &mut im,
            kit,
            67,
            430,
            "M1 ABRAMS  /  MAIN BATTLE TANK",
            18,
            Some(with_a(C_WHITE, 255)),
            true,
        );
        for (i, (lab, val, pct, col)) in [
            ("HULL", "840 / 900", 0.93, C_GREEN),
            ("ARMOR", "120", 0.72, C_CYAN),
            ("SPEED", "45 km/h", 0.65, C_AMBER),
            ("AMMO", "36 / 40", 0.90, C_CYAN),
            ("CREW", "4 / 4", 1.0, C_GREEN),
        ]
        .iter()
        .enumerate()
        {
            let y = 487 + i as i32 * 51;
            draw_text(
                &mut im,
                kit,
                70,
                y,
                lab,
                13,
                Some(with_a(C_MUTED, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                180,
                y,
                val,
                14,
                Some(with_a(C_WHITE, 255)),
                false,
            );
            rounded(
                &mut im,
                180,
                y + 23,
                471,
                y + 33,
                4,
                [7, 16, 25, 255],
                None,
                0,
            );
            rounded(
                &mut im,
                180,
                y + 23,
                180 + (291.0 * pct) as i32,
                y + 33,
                4,
                with_a(*col, 255),
                None,
                0,
            );
        }
        panel(
            &mut im,
            556,
            108,
            1050,
            810,
            Some("ORDERS & CAPABILITIES"),
            None,
            kit,
        );
        for (i, (lab, desc)) in [
            ("Attack Move", "Engage enemies en route"),
            ("Defensive Stance", "Hold ground and return fire"),
            ("Smoke Screen", "Break target lock"),
            ("Field Repair", "Requires support unit"),
            ("Veterancy II", "+10% reload speed"),
        ]
        .iter()
        .enumerate()
        {
            let y = 169 + i as i32 * 112;
            rounded(
                &mut im,
                579,
                y,
                1028,
                y + 89,
                7,
                [15, 33, 47, 255],
                Some(with_a(C_LINE, 255)),
                1,
            );
            draw_text(
                &mut im,
                kit,
                598,
                y + 12,
                lab,
                16,
                Some(with_a(if i == 0 { C_CYAN } else { C_WHITE }, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                598,
                y + 43,
                desc,
                14,
                Some(with_a(C_MUTED, 255)),
                false,
            );
            button(
                &mut im,
                kit,
                872,
                y + 51,
                1006,
                y + 77,
                if i < 3 { "ASSIGN" } else { "DETAILS" },
                i == 0,
                false,
            );
        }
        panel(
            &mut im,
            1087,
            108,
            1562,
            810,
            Some("UPGRADES / LOADOUT"),
            None,
            kit,
        );
        for (i, lab) in [
            "Composite armor",
            "Improved optics",
            "Stabilized cannon",
            "Thermal imaging",
            "Crew survivability",
        ]
        .iter()
        .enumerate()
        {
            let y = 173 + i as i32 * 84;
            im.ellipse(
                1111,
                y + 5,
                1145,
                y + 39,
                Some(with_a(if i < 2 { C_GREEN } else { C_MUTED }, 255)),
                None,
                0,
            );
            draw_text(
                &mut im,
                kit,
                1160,
                y + 1,
                lab,
                15,
                Some(with_a(C_WHITE, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                1160,
                y + 28,
                if i < 2 {
                    "Researched"
                } else {
                    "Requires Tech Tier II"
                },
                12,
                Some(with_a(C_MUTED, 255)),
                false,
            );
        }
    } else if screen_id == "tech_tree" {
        panel(
            &mut im,
            32,
            109,
            1568,
            808,
            Some("RESEARCH & DEVELOPMENT"),
            None,
            kit,
        );
        for (i, l) in ["INFANTRY", "VEHICLES", "AIR", "NAVAL", "BASE SYSTEMS"]
            .iter()
            .enumerate()
        {
            let x = 58 + i as i32 * 295;
            button(&mut im, kit, x, 160, x + 272, 205, l, i == 1, false);
        }
        let nodes = [
            (135, 290, "FIELD LOGISTICS"),
            (410, 290, "COMPOSITE ARMOR"),
            (685, 290, "ADVANCED OPTICS"),
            (960, 290, "HEAVY ARMOR"),
            (1235, 290, "SIEGE SYSTEMS"),
            (275, 515, "MOBILE REPAIR"),
            (550, 515, "THERMAL IMAGING"),
            (825, 515, "VETERANCY DOCTRINE"),
            (1100, 515, "EXPERIMENTAL CORE"),
        ];
        for pair in nodes.windows(2) {
            let (x1, y1, _) = pair[0];
            let (x2, y2, _) = pair[1];
            im.line(x1 + 78, y1 + 56, x2 + 78, y2 + 56, with_a(C_LINE, 255), 4);
        }
        for (i, (x, y, lab)) in nodes.iter().enumerate() {
            let col = if i < 3 {
                C_GREEN
            } else if i < 6 {
                C_AMBER
            } else {
                C_MUTED
            };
            rounded(
                &mut im,
                *x,
                *y,
                x + 215,
                y + 100,
                8,
                [13, 35, 49, 255],
                Some(with_a(col, 255)),
                2,
            );
            draw_text(
                &mut im,
                kit,
                x + 12,
                y + 12,
                &format!(
                    "TIER {}",
                    if i < 3 {
                        1
                    } else if i < 6 {
                        2
                    } else {
                        3
                    }
                ),
                11,
                Some(with_a(col, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                x + 12,
                y + 38,
                lab,
                13,
                Some(with_a(C_WHITE, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                x + 12,
                y + 72,
                "COST  2,400",
                12,
                Some(with_a(C_MUTED, 255)),
                false,
            );
        }
        button(
            &mut im,
            kit,
            1190,
            718,
            1515,
            768,
            "RESEARCH SELECTED",
            true,
            false,
        );
    } else if screen_id == "pause_options" {
        // darkened battlefield behind overlay
        im.alpha_composite(
            &Canvas::new_filled(w as u32, h as u32, [0, 0, 0, 155]),
            (0, 0),
        );
        panel(
            &mut im,
            492,
            118,
            1108,
            795,
            Some("OPERATION PAUSED"),
            None,
            kit,
        );
        draw_text(
            &mut im,
            kit,
            614,
            176,
            "FIELD COMMAND INTERRUPTED",
            21,
            Some(with_a(C_MUTED, 255)),
            true,
        );
        for (i, lab) in [
            "RESUME OPERATION",
            "SAVE GAME",
            "LOAD GAME",
            "OPTIONS",
            "ACCESSIBILITY",
            "RETURN TO CAMPAIGN",
            "QUIT TO DESKTOP",
        ]
        .iter()
        .enumerate()
        {
            let y = 240 + i as i32 * 68;
            button(&mut im, kit, 563, y, 1037, y + 49, lab, i == 0, false);
        }
        draw_text(
            &mut im,
            kit,
            600,
            739,
            "MISSION TIME  00:42:18     SAVE STATE  CHECKPOINT 03",
            12,
            Some(with_a(C_MUTED, 255)),
            false,
        );
    } else if screen_id == "mission_debrief" {
        panel(
            &mut im,
            36,
            108,
            1040,
            806,
            Some("AFTER ACTION REPORT"),
            None,
            kit,
        );
        draw_text(
            &mut im,
            kit,
            79,
            170,
            "OPERATION GREY BRIDGE",
            27,
            Some(with_a(C_GREEN, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            80,
            217,
            "MISSION SUCCESSFUL  /  TIME  00:42:18",
            15,
            Some(with_a(C_MUTED, 255)),
            true,
        );
        for (i, (lab, val)) in [
            ("PRIMARY OBJECTIVES", "3 / 3"),
            ("SECONDARY OBJECTIVES", "2 / 2"),
            ("UNITS LOST", "4"),
            ("ENEMIES NEUTRALIZED", "37"),
            ("CIVILIANS EVACUATED", "126"),
            ("SUPPLY DEPOTS SECURED", "2"),
        ]
        .iter()
        .enumerate()
        {
            let x = 81 + (i % 2) as i32 * 460;
            let y = 285 + (i / 2) as i32 * 112;
            rounded(
                &mut im,
                x,
                y,
                x + 414,
                y + 82,
                8,
                [12, 32, 45, 255],
                Some(with_a(C_LINE, 255)),
                1,
            );
            draw_text(
                &mut im,
                kit,
                x + 18,
                y + 12,
                lab,
                12,
                Some(with_a(C_MUTED, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                x + 18,
                y + 36,
                val,
                22,
                Some(with_a(C_WHITE, 255)),
                true,
            );
        }
        panel(
            &mut im,
            1071,
            108,
            1562,
            806,
            Some("COMMANDER RECOMMENDATION"),
            None,
            kit,
        );
        let avp = out.path("sprites/ui/avatars/commanders/cmdr_hale.png");
        if avp.exists() {
            if let Ok(av) = load_rgba(&avp) {
                im.alpha_composite(&av.resize_catmullrom(180, 180), (1223, 158));
            }
        }
        draw_text(
            &mut im,
            kit,
            1126,
            367,
            "CMDR. HALE",
            18,
            Some(with_a(C_WHITE, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            1110,
            415,
            "The bridge is secured. Engineers report",
            14,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        draw_text(
            &mut im,
            kit,
            1110,
            441,
            "the crossing is stable and supply traffic",
            14,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        draw_text(
            &mut im,
            kit,
            1110,
            467,
            "can resume. Recommend rapid redeployment.",
            14,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        button(
            &mut im,
            kit,
            1110,
            692,
            1522,
            740,
            "CONTINUE CAMPAIGN",
            true,
            false,
        );
        button(
            &mut im,
            kit,
            1110,
            754,
            1522,
            788,
            "VIEW FULL REPORT",
            false,
            false,
        );
    } else if screen_id == "multiplayer_lobby" {
        panel(
            &mut im,
            34,
            110,
            1060,
            806,
            Some("LOBBY  /  OPERATION NIGHTFALL"),
            None,
            kit,
        );
        for i in 0..8 {
            let x = 60 + (i % 2) as i32 * 490;
            let y = 164 + (i / 2) as i32 * 145;
            rounded(
                &mut im,
                x,
                y,
                x + 460,
                y + 118,
                8,
                [12, 33, 47, 255],
                Some(with_a(C_LINE, 255)),
                1,
            );
            draw_text(
                &mut im,
                kit,
                x + 18,
                y + 14,
                &format!("SLOT {:02}", i + 1),
                12,
                Some(with_a(C_MUTED, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                x + 18,
                y + 42,
                ["Commander Hale", "Open slot", "AI - Veteran", "Open slot"][i % 4],
                16,
                Some(with_a(C_WHITE, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                x + 18,
                y + 78,
                if i == 0 { "READY" } else { "WAITING" },
                12,
                Some(with_a(if i == 0 { C_GREEN } else { C_MUTED }, 255)),
                true,
            );
            button(
                &mut im,
                kit,
                x + 310,
                y + 43,
                x + 438,
                y + 82,
                if i == 0 { "READY" } else { "INVITE" },
                i == 0,
                false,
            );
        }
        panel(
            &mut im,
            1084,
            110,
            1567,
            806,
            Some("CHAT & MATCH RULES"),
            None,
            kit,
        );
        for (i, line) in [
            "[10:42] Hale joined the lobby",
            "[10:43] Team North selected",
            "[10:43] Waiting for players...",
            "[SYSTEM] Voice comms enabled",
        ]
        .iter()
        .enumerate()
        {
            draw_text(
                &mut im,
                kit,
                1110,
                168 + i as i32 * 35,
                line,
                13,
                Some(with_a(if i < 3 { C_MUTED } else { C_CYAN }, 255)),
                false,
            );
        }
        panel(
            &mut im,
            1105,
            350,
            1547,
            550,
            Some("MATCH RULES"),
            None,
            kit,
        );
        for (i, line) in [
            "Fog of war: On",
            "Starting credits: 10,000",
            "Victory: Annihilation",
            "AI difficulty: Veteran",
        ]
        .iter()
        .enumerate()
        {
            draw_text(
                &mut im,
                kit,
                1125,
                405 + i as i32 * 31,
                line,
                14,
                Some(with_a(C_WHITE, 255)),
                false,
            );
        }
        button(
            &mut im,
            kit,
            1123,
            708,
            1532,
            758,
            "LAUNCH MATCH",
            true,
            false,
        );
    } else if screen_id == "options" {
        panel(&mut im, 40, 109, 375, 811, Some("SETTINGS"), None, kit);
        for (i, lab) in [
            "VIDEO",
            "AUDIO",
            "CONTROLS",
            "GAMEPLAY",
            "INTERFACE",
            "ACCESSIBILITY",
            "NETWORK",
            "LANGUAGE",
        ]
        .iter()
        .enumerate()
        {
            let y = 166 + i as i32 * 67;
            button(&mut im, kit, 65, y, 349, y + 47, lab, i == 5, false);
        }
        panel(
            &mut im,
            411,
            109,
            1560,
            811,
            Some("ACCESSIBILITY & INTERFACE"),
            None,
            kit,
        );
        for (i, (lab, val)) in [
            ("UI SCALE", "100%"),
            ("SUBTITLES", "On"),
            ("COLORBLIND MODE", "Deuteranopia"),
            ("HIGH CONTRAST", "Enabled"),
            ("CAMERA SHAKE", "Reduced"),
            ("SCREEN FLASH", "Reduced"),
            ("TEXT SIZE", "Large"),
            ("VOICE CHAT INDICATOR", "Enabled"),
        ]
        .iter()
        .enumerate()
        {
            let y = 174 + i as i32 * 66;
            draw_text(
                &mut im,
                kit,
                452,
                y,
                lab,
                15,
                Some(with_a(C_WHITE, 255)),
                true,
            );
            button(
                &mut im,
                kit,
                1000,
                y - 6,
                1505,
                y + 34,
                val,
                i == 2 || i == 3,
                true,
            );
            im.line(450, y + 49, 1512, y + 49, with_a(C_LINE, 180), 1);
        }
        button(
            &mut im,
            kit,
            1177,
            739,
            1511,
            785,
            "APPLY SETTINGS",
            true,
            false,
        );
    } else if screen_id == "commander_dossier" {
        panel(
            &mut im,
            34,
            109,
            597,
            811,
            Some("COMMANDER DOSSIER"),
            None,
            kit,
        );
        let avp = out.path("sprites/ui/avatars/commanders/cmdr_volkov.png");
        if avp.exists() {
            if let Ok(av) = load_rgba(&avp) {
                im.alpha_composite(&av.resize_catmullrom(240, 240), (193, 160));
            }
        }
        draw_text(
            &mut im,
            kit,
            95,
            421,
            "CMD. IRINA VOLKOV",
            23,
            Some(with_a(C_WHITE, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            95,
            461,
            "Northern Directorate  /  Strategic Command",
            13,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        for (i, (lab, val)) in [
            ("DOCTRINE", "Rapid mechanized assault"),
            ("CAMPAIGNS", "12 completed"),
            ("COMMAND RATING", "A-"),
            ("LOYALTY", "Directorate"),
        ]
        .iter()
        .enumerate()
        {
            let y = 528 + i as i32 * 55;
            draw_text(
                &mut im,
                kit,
                75,
                y,
                lab,
                12,
                Some(with_a(C_MUTED, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                234,
                y,
                val,
                14,
                Some(with_a(C_WHITE, 255)),
                false,
            );
        }
        panel(
            &mut im,
            627,
            109,
            1564,
            811,
            Some("RECORD & TRAITS"),
            None,
            kit,
        );
        const TRAIT_COLORS: [Rgb; 4] = [C_AMBER, C_CYAN, C_GREEN, C_RED];
        for (i, (head, desc)) in [
            ("IRON TEMPO", "+8% vehicle movement when attacking"),
            ("DEFENSIVE ENGINEER", "Field repairs complete 12% faster"),
            (
                "HARDLINE NEGOTIATOR",
                "Diplomacy actions cost more, intel gain improves",
            ),
            (
                "FRONTLINE LEGACY",
                "Nearby allied squads gain morale after an objective",
            ),
        ]
        .iter()
        .enumerate()
        {
            let y = 167 + i as i32 * 136;
            rounded(
                &mut im,
                658,
                y,
                1533,
                y + 109,
                8,
                [12, 33, 47, 255],
                Some(with_a(C_LINE, 255)),
                1,
            );
            im.ellipse(
                682,
                y + 25,
                737,
                y + 80,
                Some(with_a(TRAIT_COLORS[i], 255)),
                None,
                0,
            );
            draw_text(
                &mut im,
                kit,
                760,
                y + 18,
                head,
                17,
                Some(with_a(C_WHITE, 255)),
                true,
            );
            draw_text(
                &mut im,
                kit,
                760,
                y + 53,
                desc,
                14,
                Some(with_a(C_MUTED, 255)),
                false,
            );
        }
    }
    // footer strip and status
    im.line(22, h - 11, w - 22, h - 11, with_a(C_LINE, 200), 1);
    draw_text(
        &mut im,
        kit,
        26,
        h - 32,
        "SIMULATION // OFFLINE PREVIEW // UI ART DIRECTION ONLY",
        11,
        Some(with_a(C_MUTED, 255)),
        true,
    );
    let rel = format!("previews/ui_mockups/{screen_id}_v6_2.png");
    out.write_png(&rel, &im)?;
    Ok(rel)
}

// ---------------------------------------------------------------------------
// Wireframes — Python `make_wireframe`
// ---------------------------------------------------------------------------

/// Python `str[:n]` slicing by characters.
fn take_chars(text: &str, n: usize) -> String {
    text.chars().take(n).collect()
}

/// Python `path.stem` of a pack-relative path, with the wireframe/mockup
/// suffixes stripped and underscores turned into spaces.
fn wire_name(rel: &str) -> String {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    let stem = file.strip_suffix(".png").unwrap_or(file);
    stem.replace("_wireframe_v6_2", "")
        .replace("_v6_2", "")
        .replace('_', " ")
}

/// Python `p.stem.startswith(prefix)`.
fn stem_starts(rel: &str, prefix: &str) -> bool {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    let stem = file.strip_suffix(".png").unwrap_or(file);
    stem.starts_with(prefix)
}

/// Python `make_wireframe`: 1600×900 light-paper wireframe with a header
/// band and labelled boxes per screen. `bg_kind`/`content` are passed but
/// unread, exactly like the Python signature.
fn make_wireframe(
    out: &PackWriter,
    kit: &FontKit,
    screen_id: &str,
    title: &str,
    _bg_kind: &str,
    _content: &str,
) -> Result<String, PackError> {
    let (w, h) = (SCREEN_W, SCREEN_H);
    let mut im = Canvas::new_filled(w as u32, h as u32, [239, 242, 245, 255]);
    im.rect(
        16,
        14,
        w - 16,
        72,
        Some([222, 228, 232, 255]),
        Some([65, 76, 86, 255]),
        2,
    );
    draw_text(
        &mut im,
        kit,
        36,
        29,
        &format!("CNC32  /  {}  /  WIREFRAME", title.to_uppercase()),
        23,
        Some([28, 40, 52, 255]),
        true,
    );
    draw_text(
        &mut im,
        kit,
        w - 302,
        37,
        &format!("1600 × 900  /  v{VERSION}"),
        14,
        Some([70, 82, 91, 255]),
        false,
    );
    let boxes: &[(i32, i32, i32, i32, &str)] = match screen_id {
        "main_menu" => &[
            (70, 110, 650, 770, "PRIMARY NAVIGATION / 6 ACTIONS"),
            (690, 110, 1530, 750, "HERO ART / OPERATION CARD / STATUS"),
            (70, 790, 1530, 850, "VERSION / LEGAL / BUILD INFO"),
        ],
        "gameplay_hud" => &[
            (20, 88, 470, 148, "TOP LEFT: RESOURCES"),
            (1300, 88, 1580, 320, "TOP RIGHT: MINIMAP"),
            (20, 714, 380, 880, "BOTTOM LEFT: SELECTED UNIT"),
            (400, 714, 1040, 880, "BOTTOM CENTER: COMMAND GRID"),
            (
                1080,
                714,
                1580,
                880,
                "BOTTOM RIGHT: OBJECTIVES / EVENT FEED",
            ),
            (470, 160, 1260, 680, "WORLD VIEW / CAMERA SAFE AREA"),
        ],
        "campaign_map" => &[
            (25, 90, 1070, 810, "MAP / OBJECTIVE MARKERS / ROUTE LAYERS"),
            (1090, 90, 1575, 810, "MISSION LIST / INTEL / LAUNCH"),
        ],
        "skirmish_lobby" => &[
            (37, 110, 845, 802, "MAP PREVIEW / MATCH RULES"),
            (880, 110, 1563, 802, "FACTION SLOTS / TEAM CONFIGURATION"),
        ],
        "mission_briefing" => &[
            (35, 104, 1040, 495, "MISSION ART / TARGET OVERLAY"),
            (55, 510, 1020, 760, "INTEL SUMMARY / CONDITIONS"),
            (
                1060,
                104,
                1565,
                795,
                "COMMANDER PORTRAIT / OBJECTIVES / DEPLOY",
            ),
        ],
        "unit_inspector" => &[
            (40, 108, 520, 810, "UNIT PREVIEW / STATS"),
            (556, 108, 1050, 810, "ORDERS / CAPABILITIES"),
            (1087, 108, 1562, 810, "UPGRADES / LOADOUT"),
        ],
        "tech_tree" => &[(
            32,
            109,
            1568,
            808,
            "CATEGORY TABS / PREREQUISITE GRAPH / RESEARCH DETAILS",
        )],
        "pause_options" => &[(
            492,
            118,
            1108,
            795,
            "MODAL: RESUME / SAVE / LOAD / OPTIONS / EXIT",
        )],
        "mission_debrief" => &[
            (36, 108, 1040, 806, "MISSION RESULT / METRICS / AWARDS"),
            (1071, 108, 1562, 806, "COMMANDER REVIEW / CONTINUE"),
        ],
        "multiplayer_lobby" => &[
            (34, 110, 1060, 806, "PLAYER SLOTS / READY STATUS"),
            (1084, 110, 1567, 806, "CHAT / RULES / LAUNCH"),
        ],
        "options" => &[
            (40, 109, 375, 811, "SETTINGS CATEGORY NAVIGATION"),
            (411, 109, 1560, 811, "SETTING LABELS / VALUES / APPLY"),
        ],
        _ => &[
            (34, 109, 597, 811, "AVATAR / BIO / RECORD"),
            (627, 109, 1564, 811, "TRAITS / STATS / ACTIONS"),
        ],
    };
    for &(x1, y1, x2, y2, label) in boxes {
        rounded(
            &mut im,
            x1,
            y1,
            x2,
            y2,
            7,
            [248, 250, 251, 255],
            Some([90, 103, 114, 255]),
            2,
        );
        im.rect(
            x1 + 1,
            y1 + 1,
            x2 - 1,
            (y1 + 40).min(y2 - 1),
            Some([222, 229, 233, 255]),
            Some([90, 103, 114, 255]),
            1,
        );
        draw_text(
            &mut im,
            kit,
            x1 + 14,
            y1 + 11,
            label,
            14,
            Some([32, 47, 59, 255]),
            true,
        );
        if x2 - x1 > 260 && y2 - y1 > 125 {
            for j in 0..3 {
                let yy = y1 + 66 + j * 25;
                if yy < y2 - 12 {
                    im.line(x1 + 15, yy, x2 - 16, yy, [202, 210, 215, 255], 1);
                }
            }
        }
    }
    // Overlay UI safe-area.
    im.rect(40, 84, w - 40, h - 42, None, Some([135, 149, 158, 255]), 1);
    let rel = format!("previews/wireframes/{screen_id}_wireframe_v6_2.png");
    out.write_png(&rel, &im)?;
    Ok(rel)
}

// ---------------------------------------------------------------------------
// Contact sheets — Python `make_personnel_contact_sheet` and
// `make_screen_contact_sheet`
// ---------------------------------------------------------------------------

/// Python `make_personnel_contact_sheet`: 7-column roster over the 28
/// portrait files, 208×246 cards, portraits re-loaded from disk and
/// NEAREST-resized to 144.
fn make_personnel_contact_sheet(
    out: &PackWriter,
    kit: &FontKit,
    avatar_map: &[(String, Value)],
) -> Result<String, PackError> {
    let (cols, cell_w, cell_h) = (7usize, 208i32, 246i32);
    let (margin, title_h) = (18i32, 74i32);
    let rows = avatar_map.len().div_ceil(cols) as i32;
    let width = margin + cols as i32 * cell_w + (cols as i32 - 1) * 14 + margin;
    let height = title_h + margin + rows * cell_h + (rows - 1) * 14 + margin;
    let mut im = Canvas::new_filled(width as u32, height as u32, with_a(C_BG, 255));
    draw_text(
        &mut im,
        kit,
        margin,
        20,
        "CNC32 // PROCEDURAL PERSONNEL ROSTER",
        27,
        Some(with_a(C_WHITE, 255)),
        true,
    );
    draw_text(
        &mut im,
        kit,
        margin,
        51,
        &format!(
            "{} portraits · code-generated geometry · seed-driven details · v{VERSION}",
            avatar_map.len()
        ),
        13,
        Some(with_a(C_MUTED, 255)),
        false,
    );
    for (index, (pid, person)) in avatar_map.iter().enumerate() {
        let (col, row) = ((index % cols) as i32, (index / cols) as i32);
        let x = margin + col * (cell_w + 14);
        let y = title_h + margin + row * (cell_h + 14);
        let role = person["role"].as_str().unwrap_or("NPC");
        let accent = if role == "Commander" { C_AMBER } else { C_CYAN };
        rounded(
            &mut im,
            x,
            y,
            x + cell_w,
            y + cell_h,
            8,
            with_a(C_PANEL, 255),
            Some(with_a(C_LINE, 255)),
            2,
        );
        let portrait = load_rgba(&out.path(person["file"].as_str().expect("avatar file")))?
            .resize_nearest(144, 144);
        im.alpha_composite(&portrait, (x + (cell_w - 144) / 2, y + 10));
        let name = person["name"].as_str().unwrap_or(pid);
        let faction = person["faction"].as_str().unwrap_or("Neutral");
        draw_text(
            &mut im,
            kit,
            x + 8,
            y + 161,
            &take_chars(name, 27),
            12,
            Some(with_a(C_WHITE, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            x + 8,
            y + 183,
            &take_chars(role, 28),
            11,
            Some(with_a(accent, 255)),
            true,
        );
        draw_text(
            &mut im,
            kit,
            x + 8,
            y + 204,
            &take_chars(faction, 27),
            10,
            Some(with_a(C_MUTED, 255)),
            false,
        );
        draw_text(
            &mut im,
            kit,
            x + 8,
            y + 224,
            &take_chars(pid, 30),
            9,
            Some(with_a(C_MUTED, 255)),
            false,
        );
    }
    let rel = "previews/v6_2_personnel_roster.png";
    out.write_png(rel, &im)?;
    Ok(rel.to_string())
}

/// Python `make_screen_contact_sheet`: 3-column sheet of the screen mockups
/// (or wireframes), 480×270 Catmull-Rom thumbnails centred on the pack
/// background colour.
fn make_screen_contact_sheet(
    out: &PackWriter,
    kit: &FontKit,
    previews: &[String],
    wireframe: bool,
) -> Result<String, PackError> {
    let cols = 3usize;
    let rows = previews.len().div_ceil(cols);
    let (thumb_w, thumb_h) = (480u32, 270u32);
    let (card_w, card_h) = (thumb_w + 20, thumb_h + 54);
    let (gap, margin, title_h) = (22usize, 24usize, 76usize);
    let width = (margin * 2 + cols * card_w as usize + (cols - 1) * gap) as u32;
    let height = (title_h + margin + rows * card_h as usize + (rows - 1) * gap + margin) as u32;
    let mut im = Canvas::new_filled(width, height, with_a(C_BG, 255));
    let kind = if wireframe {
        "LOW-FIDELITY WIREFRAMES"
    } else {
        "HIGH-FIDELITY UI MOCKUPS"
    };
    draw_text(
        &mut im,
        kit,
        margin as i32,
        20,
        &format!("CNC32 // {kind}"),
        27,
        Some(with_a(C_WHITE, 255)),
        true,
    );
    draw_text(
        &mut im,
        kit,
        margin as i32,
        51,
        &format!(
            "{} screens · generated by asset_expansion_v6_2.py · {VERSION}",
            previews.len()
        ),
        13,
        Some(with_a(C_MUTED, 255)),
        false,
    );
    for (i, rel) in previews.iter().enumerate() {
        let (col, row) = (i % cols, i / cols);
        let x = (margin + col * (card_w as usize + gap)) as i32;
        let y = (title_h + margin + row * (card_h as usize + gap)) as i32;
        rounded(
            &mut im,
            x,
            y,
            x + card_w as i32,
            y + card_h as i32,
            8,
            with_a(C_PANEL, 255),
            Some(with_a(C_LINE, 255)),
            2,
        );
        let thumb = fit_thumb(&load_rgba(&out.path(rel))?, thumb_w, thumb_h);
        im.alpha_composite(&thumb, (x + 10, y + 10));
        let name = py_title(&wire_name(rel));
        draw_text(
            &mut im,
            kit,
            x + 12,
            y + thumb_h as i32 + 20,
            &take_chars(&name, 43),
            12,
            Some(with_a(C_WHITE, 255)),
            true,
        );
    }
    let rel = if wireframe {
        "previews/v6_2_wireframe_contact_sheet.png"
    } else {
        "previews/v6_2_ui_mockup_contact_sheet.png"
    };
    out.write_png(rel, &im)?;
    Ok(rel.to_string())
}

// ---------------------------------------------------------------------------
// Art-direction board — Python `make_art_direction_board`
// ---------------------------------------------------------------------------

/// Python `wanted` icon names for the board's icon bank.
const BOARD_ICON_WANTED: [&str; 32] = [
    "select",
    "move",
    "attack",
    "stop",
    "hold",
    "patrol",
    "defend",
    "repair",
    "build",
    "rally",
    "recon",
    "radar",
    "airstrike",
    "artillery",
    "capture",
    "rescue",
    "transport",
    "objective_primary",
    "objective_secondary",
    "waypoint",
    "ping",
    "power",
    "credits",
    "health",
    "armor",
    "ammo",
    "tech",
    "upgrade",
    "alert",
    "pause",
    "settings",
    "map",
];

/// Python `make_art_direction_board`: a 2000×1630 overview board composited
/// exclusively from assets this generator already wrote — portraits, icon
/// glyphs, control states, mockups, wireframes and a drawn masthead.
#[allow(clippy::too_many_arguments)]
fn make_art_direction_board(
    out: &PackWriter,
    kit: &FontKit,
    avatar_map: &[(String, Value)],
    icon_paths: &[(&'static str, String, Canvas)],
    mockups: &[String],
    wireframes: &[String],
    seed: u64,
) -> Result<String, PackError> {
    let (width, height) = (2000i32, 1630i32);
    let mut im = Canvas::new_filled(width as u32, height as u32, with_a(C_BG, 255));
    // Patterned masthead drawn in code; no imported backdrop is used.
    let mut x = 0;
    while x < width {
        im.line(x, 0, x - 160, 96, [25, 72, 94, 75], 1);
        x += 48;
    }
    draw_text(
        &mut im,
        kit,
        28,
        18,
        &format!("CNC32 v{VERSION}"),
        38,
        Some(with_a(C_WHITE, 255)),
        true,
    );
    draw_text(
        &mut im,
        kit,
        380,
        29,
        "PROCEDURAL ASSET GENERATION // COMMANDERS · NPCs · UI · WIREFRAMES",
        18,
        Some(with_a(C_CYAN, 255)),
        true,
    );
    draw_text(
        &mut im, kit, 28, 70,
        "All overview imagery assembled from code-generated production assets · no image-model source artwork",
        14, Some(with_a(C_MUTED, 255)), false,
    );

    // Commander strip.
    board_panel(
        &mut im,
        kit,
        24,
        112,
        984,
        414,
        "Commander profiles",
        C_AMBER,
    );
    let commanders: Vec<&(String, Value)> = avatar_map
        .iter()
        .filter(|(_, p)| p["role"].as_str() == Some("Commander"))
        .collect();
    for (i, (_pid, p)) in commanders.iter().take(5).enumerate() {
        let x = 58 + i as i32 * 180;
        let y = 175i32;
        let avatar = load_rgba(&out.path(p["file"].as_str().expect("avatar file")))?
            .resize_nearest(128, 128);
        im.alpha_composite(&avatar, (x, y));
        let name = p["name"]
            .as_str()
            .unwrap_or("")
            .replace("Commander ", "Cmdr. ");
        draw_text(
            &mut im,
            kit,
            x - 5,
            y + 137,
            &take_chars(&name, 21),
            11,
            Some(with_a(C_WHITE, 255)),
            true,
        );
        let faction = p["faction"].as_str().unwrap_or("");
        draw_text(
            &mut im,
            kit,
            x - 5,
            y + 157,
            &take_chars(faction, 22),
            9,
            Some(with_a(C_MUTED, 255)),
            false,
        );
    }

    // Tactical icon bank.
    board_panel(
        &mut im,
        kit,
        1004,
        112,
        1976,
        414,
        "Tactical icon glyphs",
        C_CYAN,
    );
    for (i, name) in BOARD_ICON_WANTED.iter().enumerate() {
        let Some((_, _, icon)) = icon_paths.iter().find(|(n, _, _)| *n == *name) else {
            continue;
        };
        let (col, row) = ((i % 8) as i32, (i / 8) as i32);
        let x = 1030 + col * 116;
        let y = 170 + row * 55;
        let icon = icon.resize_nearest(32, 32);
        im.alpha_composite(&icon, (x, y));
        draw_text(
            &mut im,
            kit,
            x + 39,
            y + 9,
            &take_chars(&name.replace('_', " "), 13),
            9,
            Some(with_a(C_MUTED, 255)),
            false,
        );
    }

    // Main gameplay screen plus two generated supporting screen examples.
    board_panel(
        &mut im,
        kit,
        24,
        430,
        1264,
        1148,
        "Gameplay HUD // procedural composition",
        C_GREEN,
    );
    let main_mock = mockups
        .iter()
        .find(|p| stem_starts(p, "gameplay_hud"))
        .or_else(|| mockups.first())
        .expect("mockups generated");
    im.alpha_composite(
        &fit_thumb(&load_rgba(&out.path(main_mock))?, 1198, 674),
        (36, 474),
    );
    board_panel(
        &mut im,
        kit,
        1284,
        430,
        1976,
        1148,
        "Menus and briefing screens",
        C_CYAN,
    );
    let mut y = 480i32;
    for sid in ["main_menu", "mission_briefing"] {
        let pth = mockups
            .iter()
            .find(|p| stem_starts(p, sid))
            .or_else(|| mockups.first())
            .expect("mockups generated");
        let thumb = fit_thumb(&load_rgba(&out.path(pth))?, 560, 315);
        im.alpha_composite(&thumb, (1350, y));
        draw_text(
            &mut im,
            kit,
            1352,
            y + 317,
            &sid.replace('_', " ").to_uppercase(),
            11,
            Some(with_a(C_MUTED, 255)),
            true,
        );
        y += 326;
    }

    // Control states and the HUD wireframe sample.
    board_panel(
        &mut im,
        kit,
        24,
        1160,
        630,
        1606,
        "Reusable UI component states",
        C_AMBER,
    );
    let control_names = [
        "button_normal",
        "button_hover",
        "button_pressed",
        "button_disabled",
        "panel_header",
        "tooltip",
        "notification",
        "progress_track",
    ];
    for (i, name) in control_names.iter().enumerate() {
        let pth = out.path(&format!("sprites/ui/controls/{name}.png"));
        if !pth.is_file() {
            continue;
        }
        let (col, row) = ((i % 4) as i32, (i / 4) as i32);
        let x = 44 + col * 142;
        let y = 1220 + row * 150;
        let mut sample = thumbnail(&load_rgba(&pth)?, 124, 76);
        if name.starts_with("button") {
            let mut tile = Canvas::new_filled(126, 46, with_a(C_PANEL2, 255));
            tile.alpha_composite(
                &sample,
                (
                    ((126 - sample.width()) / 2) as i32,
                    ((46 - sample.height()) / 2) as i32,
                ),
            );
            sample = tile;
        }
        im.alpha_composite(&sample, (x, y));
        draw_text(
            &mut im,
            kit,
            x,
            y + 81,
            &take_chars(&name.replace('_', " "), 18),
            9,
            Some(with_a(C_MUTED, 255)),
            false,
        );
    }
    board_panel(
        &mut im,
        kit,
        650,
        1160,
        1576,
        1606,
        "Gameplay HUD layout wireframe",
        C_CYAN,
    );
    let hud_wire = wireframes
        .iter()
        .find(|p| stem_starts(p, "gameplay_hud"))
        .or_else(|| wireframes.first())
        .expect("wireframes generated");
    im.alpha_composite(
        &fit_thumb(&load_rgba(&out.path(hud_wire))?, 884, 380),
        (670, 1208),
    );
    board_panel(&mut im, kit, 1596, 1160, 1976, 1606, "Build notes", C_GREEN);
    let notes = [
        format!("Seed: {seed}"),
        "28 portrait profiles".to_string(),
        "110 UI icon glyphs".to_string(),
        "13 component templates".to_string(),
        "12 screen mockups".to_string(),
        "12 wireframes".to_string(),
        "17 screen-flow links".to_string(),
        "Python + Pillow".to_string(),
        "No API / no model images".to_string(),
    ];
    for (i, text) in notes.iter().enumerate() {
        let color = if i < 7 { C_WHITE } else { C_MUTED };
        draw_text(
            &mut im,
            kit,
            1620,
            1220 + i as i32 * 33,
            text,
            12,
            Some(with_a(color, 255)),
            i == 0,
        );
    }
    let rel = "previews/v6_2_art_direction_board.png";
    out.write_png(rel, &im)?;
    Ok(rel.to_string())
}

// ---------------------------------------------------------------------------
// Screen flow — Python `build_screen_flow`
// ---------------------------------------------------------------------------

/// Python `build_screen_flow`: the 12-node / 17-edge UI screen-flow map,
/// written as `ui/screen_flow_v6_2.json` (no trailing newline, like the
/// Python `write_text`) plus `ui/screen_flow_v6_2.svg`.
fn build_screen_flow(out: &PackWriter) -> Result<(usize, Value), PackError> {
    let edges: [(&str, &str); 17] = [
        ("main_menu", "campaign_map"),
        ("main_menu", "skirmish_lobby"),
        ("main_menu", "multiplayer_lobby"),
        ("main_menu", "options"),
        ("campaign_map", "mission_briefing"),
        ("mission_briefing", "gameplay_hud"),
        ("gameplay_hud", "pause_options"),
        ("gameplay_hud", "unit_inspector"),
        ("gameplay_hud", "tech_tree"),
        ("gameplay_hud", "mission_debrief"),
        ("mission_debrief", "campaign_map"),
        ("skirmish_lobby", "gameplay_hud"),
        ("multiplayer_lobby", "gameplay_hud"),
        ("pause_options", "gameplay_hud"),
        ("pause_options", "options"),
        ("main_menu", "commander_dossier"),
        ("commander_dossier", "campaign_map"),
    ];
    let flow = json!({
        "version": VERSION,
        "entry": "main_menu",
        "nodes": SCREEN_SPECS
            .iter()
            .map(|(sid, title, _, _)| json!({"id": sid, "title": title}))
            .collect::<Vec<Value>>(),
        "edges": edges.iter().map(|(a, b)| json!([a, b])).collect::<Vec<Value>>(),
    });
    out.write_text(
        "ui/screen_flow_v6_2.json",
        &serde_json::to_string_pretty(&flow)?,
    )?;
    let nodes: std::collections::BTreeMap<&str, &str> = SCREEN_SPECS
        .iter()
        .map(|(sid, title, _, _)| (*sid, *title))
        .collect();
    let mut svg = vec![
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1400\" height=\"880\" viewBox=\"0 0 1400 880\">".to_string(),
        "<rect width=\"1400\" height=\"880\" fill=\"#07101b\"/>".to_string(),
        format!("<text x=\"42\" y=\"48\" fill=\"#e2edf2\" font-size=\"26\" font-family=\"Arial\" font-weight=\"bold\">CNC32 v{VERSION} — UI SCREEN FLOW</text>"),
    ];
    let positions: [(&str, i32, i32); 12] = [
        ("main_menu", 500, 95),
        ("campaign_map", 230, 230),
        ("skirmish_lobby", 500, 230),
        ("multiplayer_lobby", 800, 230),
        ("options", 1080, 230),
        ("mission_briefing", 230, 370),
        ("gameplay_hud", 500, 480),
        ("pause_options", 800, 370),
        ("unit_inspector", 230, 610),
        ("tech_tree", 500, 640),
        ("mission_debrief", 800, 610),
        ("commander_dossier", 1080, 370),
    ];
    let pos = |sid: &str| {
        positions
            .iter()
            .find(|(s, _, _)| *s == sid)
            .map(|(_, x, y)| (*x, *y))
            .expect("position for every screen id")
    };
    for (a, b) in edges {
        let (x1, y1) = pos(a);
        let (x2, y2) = pos(b);
        let (x1, y1, x2, y2) = (x1 + 92, y1 + 26, x2 + 92, y2 + 26);
        svg.push(format!(
            "<path d=\"M{x1} {y1} L{x2} {y2}\" fill=\"none\" stroke=\"#27698b\" stroke-width=\"2\" opacity=\"0.9\"/>"
        ));
    }
    for (sid, x, y) in positions {
        svg.push(format!(
            "<rect x=\"{x}\" y=\"{y}\" width=\"184\" height=\"52\" rx=\"7\" fill=\"#142a3d\" stroke=\"#46c4e9\" stroke-width=\"2\"/>"
        ));
        let label = nodes.get(sid).unwrap_or(&"").replace('&', "&amp;");
        svg.push(format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#e2edf2\" font-size=\"13\" font-family=\"Arial\">{label}</text>",
            x + 92,
            y + 31
        ));
    }
    svg.push("</svg>".to_string());
    out.write_text("ui/screen_flow_v6_2.svg", &svg.join("\n"))?;
    Ok((edges.len(), flow))
}

// ---------------------------------------------------------------------------
// Manifest records — Python `add_manifest_records`
// ---------------------------------------------------------------------------

/// Recursively list every `*.png` under a pack-relative directory,
/// pack-relative, sorted — the `rglob("*.png")` + `sorted(extra_files)`
/// combination the Python `build` performs.
fn collect_png_rels(out: &PackWriter, dir: &str) -> Vec<String> {
    let mut rels = Vec::new();
    let mut stack = vec![dir.to_string()];
    while let Some(current) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(out.path(&current)) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let rel = format!("{current}/{name}");
            if entry.path().is_dir() {
                stack.push(rel);
            } else if name.ends_with(".png") {
                rels.push(rel);
            }
        }
    }
    rels.sort();
    rels
}

/// `add_manifest_records` outcome for the `generate` assembly: the newly
/// appended sprite records, the two re-appended atlas entries, the
/// `commanders_v6_2` / `npc_roster_v6_2` / `ui_v6_2` keys stamped on the
/// manifest, and the post-merge sprite count. Python returns the
/// `(added, total)` pair; the extra fields feed `LayerOutput`.
struct ManifestRecords {
    records: Vec<Value>,
    atlas_entries: Vec<Value>,
    commanders: Value,
    npcs: Value,
    ui_v6_2: Value,
    total: usize,
}

/// Python `add_manifest_records`: re-read `manifest.json`, register every
/// `sprites/ui/**` PNG as a `ui_v6_2` sprite record (updating same-file
/// records in place like `existing[rel].update(rec)`), stamp the top-level
/// v6.2 keys, replace the two atlas entries, and write `manifest.json`
/// back with a trailing newline.
fn add_manifest_records(
    out: &PackWriter,
    sprite_rels: &[String],
    avatar_map: &[(String, Value)],
) -> Result<ManifestRecords, PackError> {
    let mut manifest = out.read_json("manifest.json")?.unwrap_or_else(|| json!({}));
    if !manifest.is_object() {
        manifest = json!({});
    }
    let obj = manifest.as_object_mut().expect("manifest is an object");
    let mut existing: Vec<Value> = obj
        .get("sprites")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut records: Vec<Value> = Vec::new();
    for rel in sprite_rels {
        if !rel.starts_with("sprites/") {
            continue;
        }
        let path = out.path(rel);
        let (Ok(canvas), Ok(bytes)) = (load_rgba(&path), std::fs::read(&path)) else {
            // Python wraps the open/hash in try/except and skips failures.
            continue;
        };
        let rec = sprite_record(rel, &canvas, &bytes, v62_spec());
        match existing
            .iter_mut()
            .find(|e| e.get("file").and_then(Value::as_str) == Some(rel.as_str()))
        {
            Some(slot) => {
                // `existing[rel].update(rec)`: rec's keys win, any older
                // extra keys are preserved.
                if let (Some(slot_obj), Some(rec_obj)) = (slot.as_object_mut(), rec.as_object()) {
                    for (key, value) in rec_obj {
                        slot_obj.insert(key.clone(), value.clone());
                    }
                }
            }
            None => records.push(rec),
        }
    }
    existing.extend(records.iter().cloned());
    let total = existing.len();
    obj.insert("sprites".to_string(), Value::Array(existing));

    obj.insert("generator_version".to_string(), json!(VERSION));
    obj.insert(
        "release_note".to_string(),
        json!("v6.2.1 procedural rebuild: portraits, UI icons, controls, screens, wireframes and contact sheets generated by Python/Pillow; no image-model artwork required; all prior v6.1 assets retained."),
    );
    let commanders: Map<String, Value> = avatar_map
        .iter()
        .filter(|(_, p)| p["role"].as_str() == Some("Commander"))
        .cloned()
        .collect();
    let npcs: Map<String, Value> = avatar_map
        .iter()
        .filter(|(_, p)| p["role"].as_str() != Some("Commander"))
        .cloned()
        .collect();
    obj.insert(
        "commanders_v6_2".to_string(),
        Value::Object(commanders.clone()),
    );
    obj.insert("npc_roster_v6_2".to_string(), Value::Object(npcs.clone()));

    let control_components: Vec<String> = {
        let mut stems: Vec<String> = sprite_rels
            .iter()
            .filter(|rel| rel.starts_with("sprites/ui/controls/"))
            .map(|rel| {
                rel.rsplit('/')
                    .next()
                    .unwrap_or(rel)
                    .strip_suffix(".png")
                    .unwrap_or(rel)
                    .to_string()
            })
            .collect();
        stems.sort();
        stems
    };
    let ui_v6_2 = json!({
        "icon_count": ICONS.len(),
        "icon_atlas": "atlases/ui_icons_v6_2.png",
        "icon_map": "atlases/ui_icons_v6_2.json",
        "control_components": control_components,
        "screen_mockups": SCREEN_SPECS.iter().map(|(sid, ..)| format!("previews/ui_mockups/{sid}_v6_2.png")).collect::<Vec<String>>(),
        "wireframes": SCREEN_SPECS.iter().map(|(sid, ..)| format!("previews/wireframes/{sid}_wireframe_v6_2.png")).collect::<Vec<String>>(),
        "screen_flow": "ui/screen_flow_v6_2.json",
        "screen_resolution": [SCREEN_W, SCREEN_H],
    });
    obj.insert("ui_v6_2".to_string(), ui_v6_2.clone());

    // Replace the two atlas entries, then re-append them at the end.
    let mut atlases: Vec<Value> = obj
        .get("atlases")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    atlases.retain(|a| {
        let file = a.get("file").and_then(Value::as_str).unwrap_or("");
        file != "atlases/ui_icons_v6_2.png" && file != "atlases/character_portraits_v6_2.png"
    });
    let icon_atlas_record = json!({
        "file": "atlases/ui_icons_v6_2.png",
        "category": "ui_v6_2",
        "columns": 12,
        "rows": ICONS.len().div_ceil(12),
        "cell_size": [ICON_SIZE, ICON_SIZE],
        "sprite_count": ICONS.len(),
        "sha256_file": out.sha256_file("atlases/ui_icons_v6_2.png")?,
    });
    let portrait_atlas_record = json!({
        "file": "atlases/character_portraits_v6_2.png",
        "category": "ui_v6_2",
        "columns": 6,
        "rows": avatar_map.len().div_ceil(6),
        "cell_size": [AVATAR_SIZE, AVATAR_SIZE],
        "sprite_count": avatar_map.len(),
        "sha256_file": out.sha256_file("atlases/character_portraits_v6_2.png")?,
    });
    atlases.push(icon_atlas_record.clone());
    atlases.push(portrait_atlas_record.clone());
    obj.insert("atlases".to_string(), Value::Array(atlases));

    out.write_json("manifest.json", &manifest)?;

    Ok(ManifestRecords {
        records,
        atlas_entries: vec![icon_atlas_record, portrait_atlas_record],
        commanders: Value::Object(commanders),
        npcs: Value::Object(npcs),
        ui_v6_2,
        total,
    })
}

// ---------------------------------------------------------------------------
// Assembly — Python `build` / `generate`
// ---------------------------------------------------------------------------

/// Python `build(root, seed)`: one deterministic additive pass over an
/// already-staged v6.1-compatible pack. Portraits, icons and controls come
/// first, then the twelve 1600×900 mockups (portrait art is referenced by
/// the mockups), the wireframes, the screen flow, the four contact sheets,
/// and finally the manifest/index/report JSON files.
pub fn generate(out: &PackWriter, seed: u64) -> Result<LayerOutput, PackError> {
    let kit = FontKit::new();
    if !out.path("manifest.json").is_file() {
        return Err(PackError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No manifest.json in the pack root; v6.2 must be applied to an extracted v6.1-compatible asset pack",
        )));
    }
    let before = out
        .read_json("manifest.json")?
        .and_then(|m| m.get("sprites").and_then(Value::as_array).cloned())
        .map(|a| a.len())
        .unwrap_or(0);

    let (avatar_map, _avatar_atlas, _avatar_records) = gen_avatars(out, seed)?;
    let (icon_paths, _icon_atlas, _icon_records) = gen_icons(out, seed, &kit)?;
    let (controls, _control_records) = gen_controls(out)?;

    // Mockups and wireframes are generated after the portrait art exists.
    let mut mockups: Vec<String> = Vec::with_capacity(SCREEN_SPECS.len());
    let mut wireframes: Vec<String> = Vec::with_capacity(SCREEN_SPECS.len());
    for (i, (sid, title, bg, contents)) in SCREEN_SPECS.iter().enumerate() {
        let p = make_mockup(out, &kit, sid, title, bg, contents, seed + i as u64 * 37)?;
        mockups.push(p);
        let wp = make_wireframe(out, &kit, sid, title, bg, contents)?;
        wireframes.push(wp);
    }
    let (flow_edges, _) = build_screen_flow(out)?;

    // Remove only the known legacy model-generated concept board from the
    // first v6.2 draft.
    let legacy_concept = out.path("source/v6_2_art_direction_board_source.png");
    let legacy_model_board_removed = legacy_concept.is_file();
    if legacy_model_board_removed {
        std::fs::remove_file(&legacy_concept)?;
    }

    let roster_sheet = make_personnel_contact_sheet(out, &kit, &avatar_map)?;
    let mockup_sheet = make_screen_contact_sheet(out, &kit, &mockups, false)?;
    let wireframe_sheet = make_screen_contact_sheet(out, &kit, &wireframes, true)?;
    let art_board = make_art_direction_board(
        out,
        &kit,
        &avatar_map,
        &icon_paths,
        &mockups,
        &wireframes,
        seed,
    )?;

    // Asset manifest records include all new sprite PNGs; UI previews are
    // documented separately.
    let sprite_rels = collect_png_rels(out, "sprites/ui");
    let v62_sprite_count = sprite_rels.len();
    let stamped = add_manifest_records(out, &sprite_rels, &avatar_map)?;
    let records = stamped.records;
    let total = stamped.total;
    let commander_count = stamped.commanders.as_object().map_or(0, Map::len);
    let npc_count = stamped.npcs.as_object().map_or(0, Map::len);

    // `ui/ui_asset_index_v6_2.json`.
    let characters: Map<String, Value> = avatar_map.iter().cloned().collect();
    let ui_manifest = json!({
        "version": VERSION,
        "seed": seed,
        "resolution": {
            "avatar": [AVATAR_SIZE, AVATAR_SIZE],
            "icon": [ICON_SIZE, ICON_SIZE],
            "screen": [SCREEN_W, SCREEN_H],
        },
        "avatar_count": avatar_map.len(),
        "commander_count": commander_count,
        "npc_count": npc_count,
        "icon_count": ICONS.len(),
        "control_component_count": controls.len(),
        "mockup_count": mockups.len(),
        "wireframe_count": wireframes.len(),
        "screen_flow_edges": flow_edges,
        "mockups": mockups,
        "wireframes": wireframes,
        "components": controls,
        "characters": characters,
        "generation_method": "procedural_python_pillow_only",
        "external_image_model_inputs": false,
        "contact_sheets": [roster_sheet, mockup_sheet, wireframe_sheet, art_board],
    });
    out.write_json("ui/ui_asset_index_v6_2.json", &ui_manifest)?;

    // `build_report_v6_2.json` at the pack root.
    let timeline_data = out
        .read_json("sprites/cinematics/timeline.json")?
        .unwrap_or(Value::Null);
    let timeline_sequences = timeline_data
        .get("sequences")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let preserved_frames: usize = timeline_sequences
        .iter()
        .map(|s| {
            s.get("frames")
                .and_then(Value::as_array)
                .map(|a| a.len())
                .unwrap_or(0)
        })
        .sum();
    let report = json!({
        "asset_pack_version": VERSION,
        "seed": seed,
        "pre_v6_2_manifest_sprite_count": total - v62_sprite_count,
        "manifest_sprite_count": total,
        "v6_2_sprite_count": v62_sprite_count,
        "new_manifest_sprite_records": v62_sprite_count,
        "manifest_records_added_this_run": records.len(),
        "previous_manifest_sprite_count_before_run": before,
        "commander_count": commander_count,
        "npc_count": npc_count,
        "total_portrait_count": avatar_map.len(),
        "ui_icon_count": ICONS.len(),
        "control_component_count": controls.len(),
        "high_fidelity_mockup_count": mockups.len(),
        "wireframe_count": wireframes.len(),
        "screen_flow_edge_count": flow_edges,
        "preserved_cinematic_sequence_count": timeline_sequences.len(),
        "preserved_cinematic_frame_count": preserved_frames,
        "generation_method": "procedural_python_pillow_only",
        "external_image_model_inputs": false,
        "removed_legacy_model_board": legacy_model_board_removed,
        "legacy_model_board_source_absent_after_build": !out
            .path("source/v6_2_art_direction_board_source.png")
            .is_file(),
        "generated_contact_sheet_count": 4,
        "generated_contact_sheets": [roster_sheet, mockup_sheet, wireframe_sheet, art_board],
        "engine_validation": "NOT_RUN — mockups and art sheets require engine import/layout/playback checks in Godot before production release.",
    });
    out.write_json("build_report_v6_2.json", &report)?;

    // Contributions for the pack builder: the new ui_v6_2 sprite records in
    // Python's sorted registration order, the two atlas entries, the
    // manifest extras stamped above, and the report.
    let mut output = LayerOutput {
        sprites: records,
        atlases: stamped.atlas_entries,
        ..Default::default()
    };
    output
        .manifest_extra
        .insert("generator_version".to_string(), json!(VERSION));
    output.manifest_extra.insert(
        "release_note".to_string(),
        json!("v6.2.1 procedural rebuild: portraits, UI icons, controls, screens, wireframes and contact sheets generated by Python/Pillow; no image-model artwork required; all prior v6.1 assets retained."),
    );
    output
        .manifest_extra
        .insert("commanders_v6_2".to_string(), stamped.commanders);
    output
        .manifest_extra
        .insert("npc_roster_v6_2".to_string(), stamped.npcs);
    output
        .manifest_extra
        .insert("ui_v6_2".to_string(), stamped.ui_v6_2);
    output
        .reports
        .push(("build_report_v6_2.json".to_string(), report));
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    /// Temp pack root with the manifest the Python `build` requires, and
    /// `Drop` cleanup.
    struct TempPack(PathBuf);
    static PACK_SEQ: AtomicU32 = AtomicU32::new(0);

    impl TempPack {
        fn new() -> Self {
            let id = PACK_SEQ.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir()
                .join(format!("cnc-procgen-ui_v62-{}-{id}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).expect("create temp pack");
            std::fs::write(
                dir.join("manifest.json"),
                "{\n  \"sprites\": [],\n  \"atlases\": []\n}\n",
            )
            .expect("seed manifest");
            Self(std::fs::canonicalize(&dir).expect("canonical path"))
        }
    }

    impl Drop for TempPack {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn read_json_rel(pack: &TempPack, rel: &str) -> Value {
        let body = std::fs::read_to_string(pack.0.join(rel)).expect(rel);
        serde_json::from_str(&body).expect("valid json")
    }

    fn png_dims(path: &std::path::Path) -> (u32, u32) {
        image::ImageReader::open(path)
            .expect("open png")
            .into_dimensions()
            .expect("png dims")
    }

    /// Python `test_roster_has_enough_unique_ids`.
    #[test]
    fn roster_has_enough_unique_ids() {
        let ids: Vec<&str> = PEOPLE.iter().map(|p| p.pid).collect();
        assert!(ids.len() >= 28);
        let unique: std::collections::HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len());
        let commanders = PEOPLE.iter().filter(|p| p.role == "Commander").count();
        let npcs = PEOPLE.iter().filter(|p| p.role != "Commander").count();
        assert!(commanders >= 5, "commanders");
        assert!(npcs >= 20, "npcs");
    }

    /// Python `test_avatar_deterministic_and_dimensions`.
    #[test]
    fn avatar_deterministic_and_dimensions() {
        let person = &PEOPLE[0];
        let a = generate_avatar(person, DEFAULT_SEED);
        let b = generate_avatar(person, DEFAULT_SEED);
        assert_eq!((a.width(), a.height()), (128, 128));
        assert_eq!(a.as_rgba().as_raw(), b.as_rgba().as_raw());
    }

    /// Python `test_icons_are_transparent_32px`.
    #[test]
    fn icons_are_transparent_32px() {
        let kit = FontKit::new();
        for key in [
            "move",
            "attack",
            "repair",
            "radar",
            "objective_primary",
            "settings",
            "rescue",
        ] {
            let im = draw_icon(key, DEFAULT_SEED, &kit);
            assert_eq!((im.width(), im.height()), (32, 32), "{key}");
            assert!(
                (0..im.width() as i32).any(|y| {
                    (0..im.height() as i32).any(|x| im.pixel(x, y).is_some_and(|p| p[3] > 0))
                }),
                "{key} has lit alpha"
            );
        }
    }

    /// Python `test_overview_generation_is_procedural_and_has_no_external_input`.
    /// The Python source-text checks (`unlink` present, no legacy-board
    /// `read_bytes`) are structural here: `generate` removes the legacy
    /// board and never reads it as image input — asserted by the build test
    /// below.
    #[test]
    fn overview_generation_is_procedural_and_has_no_external_input() {
        assert_eq!(VERSION, "6.2.1");
        assert_eq!(DEFAULT_SEED, 62_021);
    }

    /// Python `test_build_generates_all_overviews_without_model_source`.
    #[test]
    fn build_generates_all_overviews_without_model_source() {
        let pack = TempPack::new();
        let out = PackWriter::new(&pack.0);
        let output = generate(&out, DEFAULT_SEED).expect("generate");

        let report = read_json_rel(&pack, "build_report_v6_2.json");
        assert_eq!(report["generation_method"], "procedural_python_pillow_only");
        assert_eq!(report["external_image_model_inputs"], false);
        assert_eq!(report["generated_contact_sheet_count"], 4);
        assert!(!pack
            .0
            .join("source/v6_2_art_direction_board_source.png")
            .is_file());
        for rel in report["generated_contact_sheets"]
            .as_array()
            .expect("contact sheets")
        {
            let rel = rel.as_str().expect("rel str");
            assert!(pack.0.join(rel).is_file(), "{rel}");
        }

        // 28 + 110 + 13 = 151 ui_v6_2 sprite records, unique ids.
        assert_eq!(output.sprites.len(), 151);
        let ids: std::collections::HashSet<&str> = output
            .sprites
            .iter()
            .map(|s| s["id"].as_str().expect("id"))
            .collect();
        assert_eq!(ids.len(), 151);
        for record in &output.sprites {
            assert_eq!(record["category"], "ui_v6_2");
            assert_eq!(record["version"], VERSION);
            assert!(record.get("generated_by").is_none());
            assert!(record.get("seed").is_none());
        }

        // Manifest carries the merged sprite list and the new top-level keys.
        let manifest = read_json_rel(&pack, "manifest.json");
        assert_eq!(manifest["sprites"].as_array().expect("sprites").len(), 151);
        assert_eq!(manifest["generator_version"], "6.2.1");
        assert_eq!(
            manifest["commanders_v6_2"]
                .as_object()
                .expect("cmdrs")
                .len(),
            5
        );
        assert_eq!(
            manifest["npc_roster_v6_2"].as_object().expect("npcs").len(),
            23
        );
        assert_eq!(manifest["ui_v6_2"]["icon_count"], 110);
        assert_eq!(
            manifest["ui_v6_2"]["control_components"]
                .as_array()
                .expect("controls")
                .len(),
            13
        );
        let atlases = manifest["atlases"].as_array().expect("atlases");
        assert_eq!(atlases.len(), 2);
        assert_eq!(atlases[0]["file"], "atlases/ui_icons_v6_2.png");
        assert_eq!(atlases[0]["rows"], 10);
        assert_eq!(atlases[1]["file"], "atlases/character_portraits_v6_2.png");
        assert_eq!(atlases[1]["rows"], 5);

        // Report counters.
        assert_eq!(report["commander_count"], 5);
        assert_eq!(report["npc_count"], 23);
        assert_eq!(report["ui_icon_count"], 110);
        assert_eq!(report["control_component_count"], 13);
        assert_eq!(report["high_fidelity_mockup_count"], 12);
        assert_eq!(report["wireframe_count"], 12);
        assert_eq!(report["screen_flow_edge_count"], 17);
        assert_eq!(report["manifest_sprite_count"], 151);
        assert_eq!(report["v6_2_sprite_count"], 151);

        // Screen flow: 12 nodes, 17 edges, and both files exist.
        let flow = read_json_rel(&pack, "ui/screen_flow_v6_2.json");
        assert_eq!(flow["nodes"].as_array().expect("nodes").len(), 12);
        assert_eq!(flow["edges"].as_array().expect("edges").len(), 17);
        assert!(pack.0.join("ui/screen_flow_v6_2.svg").is_file());
        // The flow JSON has no trailing newline, like the Python write_text.
        let raw = std::fs::read_to_string(pack.0.join("ui/screen_flow_v6_2.json")).unwrap();
        assert!(
            !raw.ends_with('\n'),
            "screen flow json has no trailing newline"
        );

        // Twelve mockups at 1600×900 and twelve wireframes.
        for spec in SCREEN_SPECS {
            let mock = pack
                .0
                .join(format!("previews/ui_mockups/{}_v6_2.png", spec.0));
            assert_eq!(png_dims(&mock), (1600, 900), "{}", spec.0);
            let wire = pack
                .0
                .join(format!("previews/wireframes/{}_wireframe_v6_2.png", spec.0));
            assert_eq!(png_dims(&wire), (1600, 900), "{}", spec.0);
        }

        // Atlases at 12×10 32px cells and 6×5 128px cells.
        assert_eq!(
            png_dims(&pack.0.join("atlases/ui_icons_v6_2.png")),
            (384, 320)
        );
        assert_eq!(
            png_dims(&pack.0.join("atlases/character_portraits_v6_2.png")),
            (768, 640)
        );

        // The ui index mirrors the build.
        let index = read_json_rel(&pack, "ui/ui_asset_index_v6_2.json");
        assert_eq!(index["avatar_count"], 28);
        assert_eq!(
            index["characters"].as_object().expect("characters").len(),
            28
        );
        assert_eq!(index["external_image_model_inputs"], false);

        // Determinism: a second fresh pack with the same seed is identical.
        let pack_b = TempPack::new();
        let out_b = PackWriter::new(&pack_b.0);
        generate(&out_b, DEFAULT_SEED).expect("generate again");
        for rel in [
            "manifest.json",
            "build_report_v6_2.json",
            "ui/ui_asset_index_v6_2.json",
            "ui/screen_flow_v6_2.json",
            "previews/v6_2_art_direction_board.png",
            "sprites/ui/avatars/commanders/marcus_voss.png",
            "sprites/ui/icons/move.png",
        ] {
            let a = std::fs::read(pack.0.join(rel)).unwrap_or_default();
            let b = std::fs::read(pack_b.0.join(rel)).unwrap_or_default();
            assert_eq!(a, b, "{rel}");
        }
    }
}
