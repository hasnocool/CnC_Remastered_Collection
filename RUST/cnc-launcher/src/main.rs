//! C&C Launcher - Main entry point
//!
//! Creates a winit 0.30 window and drives the renderer (wgpu by default, with
//! a software fallback) from the RedrawRequested event at roughly 60 FPS.
//!
//! With no launch-mode flags the launcher boots into the **main menu**
//! (`menu.rs`), which offers Campaign, Local Game and Online Game for both
//! engines:
//! - Campaign opens the TD mission selector or the RA campaign selector loaded
//!   from the bundled `maps/td_campaign` / `maps/ra_campaign` directories
//!   (override with `CNC_TD_CAMPAIGN` / `CNC_RA_CAMPAIGN`).
//! - Local Game starts a TD skirmish or the RA skirmish setup screen.
//! - Online Game opens the RA host/join network lobby.
//!   Esc on a game's menu-owned screen (mission select, skirmish setup, campaign
//!   select, lobby) returns to the main menu; `--menu` requests it explicitly.
//!
//! Passing a launch-mode flag keeps the direct CLI paths: `--mission FILE`,
//! `--mission-dir DIR`, `--campaign DIR`, `--scenario FILE`, `--replay FILE`,
//! `--net-role` / `--net-lobby` or an explicit `--game td|ra` start straight
//! into that mode (`--game ra` opens the RA skirmish setup). `--frames N` caps
//! a headless harness run.
//!
//! The TD demo scene is real Tiberian Dawn game state: a map, houses, buildings
//! and units owned by cnc-tiberian-dawn's Game type. The RA mode runs
//! cnc-red-alert's Game with naval/air movement, production, Tesla defenses and
//! superweapons.

// The HUD/draw helpers thread renderer + tileset + cell geometry + fallback
// color explicitly; a struct would just reshuffle the same parameters across
// every screen.
#![allow(clippy::too_many_arguments)]
use std::collections::{HashMap, HashSet};
use std::net::{SocketAddr, UdpSocket};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cnc_asset::{AssetRegistry, AtlasStyle, SpriteAtlas, TilesetPack};
use cnc_audio::AudioMixer;
use cnc_core::rules::RulesDatabase;
use cnc_core::{Fixed16_16, Vec2, Vec2i};
use cnc_red_alert::{Country, Game as RaGame, GameCommand, TILE_SIZE as RA_TILE_SIZE};
use cnc_renderer::{Camera, Renderer, SoftwareRenderer, Sprite, WgpuRenderer};
use cnc_tiberian_dawn::network::{
    Command as NetCommandKind, LiveEvent, LiveSession, NetCommand, NetPacket, NetworkConfig,
};
use cnc_tiberian_dawn::{
    Buildable, Carryover, Game, Mission, MissionOutcome, StructureType, UnitType,
};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

mod lobby;
mod lobby_screen;
mod menu;
mod options;
mod ready;

use cnc_launcher::audio_director;
use cnc_launcher::campaign;
use cnc_launcher::events_subscriber;
use cnc_launcher::score;
use lobby_screen::LobbyScreen;
use menu::{
    GameChoice, LaunchTarget, MainMenu, MenuCommand, OptionsAction, OptionsModal, VolumeBus,
};
use options::LauncherOptions;
use ready::{format_ready_line, parse_ready_line, ReadyTracker};

const FRAME_INTERVAL: Duration = Duration::from_millis(16);
pub(crate) const MAP_SIZE: u32 = 64;

#[derive(Parser)]
#[command(name = "cnc-launcher")]
#[command(about = "Command & Conquer Remastered Collection - Rust Clone")]
struct Args {
    /// Renderer backend: "wgpu" (default) or "software". Omit it to take
    /// the persisted launcher-options choice.
    #[arg(short, long)]
    renderer: Option<String>,

    /// Optional launcher options INI (video/audio defaults); overrides the
    /// saves/launcher-options.ini location.
    #[arg(long)]
    options: Option<PathBuf>,

    #[arg(short, long, default_value_t = 1024)]
    width: u32,

    #[arg(short = 'H', long, default_value_t = 768)]
    height: u32,

    /// Optional Rules.ini path; its balance overrides replace the hard-coded
    /// Tiberian Dawn defaults.
    #[arg(long)]
    rules: Option<PathBuf>,

    /// Optional campaign mission INI; triggers and objectives start immediately.
    #[arg(long)]
    mission: Option<PathBuf>,

    /// List TD mission INIs in a directory and exit (no window).
    #[arg(long)]
    list_missions: Option<PathBuf>,

    /// List RA campaign scenario INIs in a directory and exit (no window).
    #[arg(long)]
    list_campaign: Option<PathBuf>,

    /// Open a TD mission-select screen for the INIs in this directory.
    #[arg(long)]
    mission_dir: Option<PathBuf>,

    /// RA network role: "host" or "join" (skirmish over UDP).
    #[arg(long)]
    net_role: Option<String>,
    /// Host address to join (with --net-role join).
    #[arg(long)]
    net_join: Option<String>,
    /// Local UDP port for RA network play.
    #[arg(long, default_value_t = 47770)]
    net_port: u16,
    /// Host peer addresses (repeatable) for RA network play.
    #[arg(long = "net-peer")]
    net_peers: Vec<String>,
    /// Local bind IP for RA network play.
    #[arg(long, default_value = "0.0.0.0")]
    net_bind: String,
    /// Advertised NAT address for RA network play.
    #[arg(long)]
    net_advertised: Option<String>,
    /// Open the RA host/join lobby on startup instead of the skirmish setup.
    #[arg(long)]
    net_lobby: bool,
    /// Write the network match replay here on exit (default saves/ra_network_replay.rep).
    #[arg(long)]
    net_replay: Option<PathBuf>,
    /// Play a recorded CNCR network replay in the RA view.
    #[arg(long)]
    replay: Option<PathBuf>,

    /// Open the main menu (the default when no launch-mode flag is given).
    #[arg(long)]
    menu: bool,

    /// Game to run directly: "td"/"tiberian-dawn" or "ra"/"red-alert".
    /// Omit it to open the main menu.
    #[arg(short = 'g', long)]
    game: Option<String>,

    /// Optional frame cap; 0 runs until the window is closed.
    #[arg(long, default_value_t = 0)]
    frames: u64,

    /// Editor-format scenario INI loaded by the Red Alert launcher mode.
    #[arg(long)]
    scenario: Option<PathBuf>,

    /// Directory of RA campaign scenarios (default "maps") for mission select.
    #[arg(long)]
    campaign: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GameKind {
    TiberianDawn,
    RedAlert,
}

impl GameKind {
    fn parse(value: &str) -> Result<Self> {
        match value.to_ascii_lowercase().as_str() {
            "td" | "tiberian-dawn" | "tiberiandawn" => Ok(Self::TiberianDawn),
            "ra" | "red-alert" | "redalert" => Ok(Self::RedAlert),
            other => Err(anyhow!("unknown game '{}' (expected 'td' or 'ra')", other)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendererKind {
    Wgpu,
    Software,
}

/// Shared renderer bootstrapping for the TD and RA launcher modes.
fn create_renderer(
    kind: RendererKind,
    width: u32,
    height: u32,
    window: Arc<Window>,
) -> Box<dyn Renderer> {
    match kind {
        RendererKind::Wgpu => match pollster::block_on(WgpuRenderer::new(window)) {
            Ok(renderer) => {
                tracing::info!("wgpu renderer initialized");
                Box::new(renderer)
            }
            Err(error) => {
                tracing::error!(%error, "wgpu renderer initialization failed; using software renderer");
                Box::new(SoftwareRenderer::new(width, height))
            }
        },
        RendererKind::Software => Box::new(SoftwareRenderer::new(width, height)),
    }
}

impl RendererKind {
    fn parse(value: &str) -> Result<Self> {
        match value.to_ascii_lowercase().as_str() {
            "wgpu" | "gpu" => Ok(Self::Wgpu),
            "software" | "cpu" => Ok(Self::Software),
            other => Err(anyhow!(
                "unknown renderer '{}' (expected 'wgpu' or 'software')",
                other
            )),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Wgpu => "wgpu",
            Self::Software => "software",
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum BuildAction {
    Unit(UnitType),
    Structure(StructureType),
}

struct SidebarEntry {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    action: BuildAction,
    label: &'static str,
    cost: i32,
    enabled: bool,
}

struct Sidebar {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    units_header_y: f32,
    structures_header_y: f32,
    entries: Vec<SidebarEntry>,
}

enum SidebarHit {
    Action(BuildAction),
    Panel,
    Outside,
}

const SIDEBAR_WIDTH: f32 = 210.0;
const SIDEBAR_ROW_HEIGHT: f32 = 20.0;

/// Human-readable sidebar label for a buildable type.
fn build_label(buildable: Buildable) -> &'static str {
    match buildable {
        Buildable::Unit(UnitType::InfantryE1) => "Infantry",
        Buildable::Unit(UnitType::MediumTank) => "Med Tank",
        Buildable::Unit(UnitType::MammothTank) => "Mammoth",
        Buildable::Unit(UnitType::Harvester) => "Harvester",
        Buildable::Unit(UnitType::Jeep) => "Humvee",
        Buildable::Unit(UnitType::Buggy) => "Buggy",
        Buildable::Unit(UnitType::Orca) => "Orca",
        Buildable::Unit(UnitType::Apache) => "Apache",
        Buildable::Unit(UnitType::Commando) => "Commando",
        Buildable::Unit(UnitType::Engineer) => "Engineer",
        Buildable::Unit(UnitType::APC) => "APC",
        Buildable::Unit(UnitType::Artillery) => "Artillery",
        Buildable::Unit(UnitType::MLRS) => "MLRS",
        Buildable::Unit(UnitType::StealthTank) => "Stealth Tank",
        Buildable::Unit(UnitType::FlameTank) => "Flame Tank",
        Buildable::Unit(UnitType::MobileHQ) => "Mobile HQ",
        Buildable::Unit(UnitType::Grenadier) => "Grenadier",
        Buildable::Unit(UnitType::RocketSoldier) => "Rocket Soldier",
        Buildable::Unit(UnitType::Flamethrower) => "Flamethrower",
        Buildable::Unit(UnitType::ChemWarrior) => "Chem Warrior",
        Buildable::Unit(UnitType::Chinook) => "Chinook",
        Buildable::Unit(UnitType::Triceratops) => "Triceratops",
        Buildable::Unit(UnitType::Trex) => "T-Rex",
        Buildable::Unit(UnitType::Raptor) => "Raptor",
        Buildable::Unit(UnitType::Stegosaurus) => "Stegosaurus",
        Buildable::Unit(UnitType::Gunboat) => "Gunboat",
        Buildable::Structure(StructureType::ConstructionYard) => "Con Yard",
        Buildable::Structure(StructureType::PowerPlant) => "Power Plant",
        Buildable::Structure(StructureType::Refinery) => "Refinery",
        Buildable::Structure(StructureType::Silo) => "Silo",
        Buildable::Structure(StructureType::Barracks) => "Barracks",
        Buildable::Structure(StructureType::WarFactory) => "War Factory",
        Buildable::Structure(StructureType::Helipad) => "Helipad",
        Buildable::Structure(StructureType::IonCannon) => "Ion Cannon",
        Buildable::Structure(StructureType::TempleOfNod) => "Temple",
        Buildable::Structure(StructureType::GuardTower) => "Guard Tower",
        Buildable::Structure(StructureType::AdvancedGuardTower) => "Adv Guard Tower",
        Buildable::Structure(StructureType::Turret) => "Turret",
        Buildable::Structure(StructureType::SamSite) => "SAM Site",
        Buildable::Structure(StructureType::Obelisk) => "Obelisk",
    }
}

fn default_campaign_progress_path() -> PathBuf {
    let dir = std::env::var("CNC_SAVE_DIR").unwrap_or_else(|_| "saves".to_string());
    PathBuf::from(dir).join("ra_campaign.ini")
}

fn default_td_progress_path() -> PathBuf {
    let dir = std::env::var("CNC_SAVE_DIR").unwrap_or_else(|_| "saves".to_string());
    PathBuf::from(dir).join("td_campaign.ini")
}

// Slot 0 wrapper for future save-menu wiring; the slot-aware variant is used
// by the quick-save hotkeys.
#[allow(dead_code)]
fn ra_save_path() -> PathBuf {
    ra_save_path_for(0)
}

/// Default location for the replay written after a network match.
fn default_network_replay_path() -> PathBuf {
    let dir = std::env::var("CNC_SAVE_DIR").unwrap_or_else(|_| "saves".to_string());
    PathBuf::from(dir).join("ra_network_replay.rep")
}

/// Path for RA save slot 0..2 (slot 0 keeps the historical filename).
fn ra_save_path_for(slot: usize) -> PathBuf {
    let dir = std::env::var("CNC_SAVE_DIR").unwrap_or_else(|_| "saves".to_string());
    let file = if slot == 0 {
        "ra_mission.sav".to_string()
    } else {
        format!("ra_mission{}.sav", slot + 1)
    };
    PathBuf::from(dir).join(file)
}

fn draw_td_briefing(
    renderer: &mut dyn Renderer,
    mission: &Mission,
    tileset: Option<&TilesetPack>,
    width: u32,
    height: u32,
) -> bool {
    // Video first (plan box 212): only drawn when the screen's tileset pack is
    // resident so the borrowed texture slot can be restored once the briefing
    // ends. Any miss (no asset, unreadable file, unsupported variant, no atlas
    // backend) falls through to the text panel below.
    if tileset.is_some()
        && draw_briefing_video(renderer, mission.briefing_image.as_deref(), width, height)
    {
        renderer.draw_text(&mission.name, 40.0, 22.0, 16.0, [0.95, 0.93, 0.88, 1.0]);
        renderer.draw_text(
            "Enter: begin mission    Esc: quit",
            40.0,
            height as f32 - 34.0,
            12.0,
            [0.78, 0.85, 0.95, 1.0],
        );
        return true;
    }
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.05, 0.05, 0.07, 1.0]);
    renderer.draw_rect(0.0, 110.0, w, 2.0, [0.70, 0.55, 0.25, 1.0]);
    renderer.draw_text(
        "MISSION BRIEFING",
        60.0,
        40.0,
        24.0,
        [0.95, 0.70, 0.30, 1.0],
    );
    renderer.draw_text(&mission.name, 60.0, 82.0, 15.0, [0.90, 0.93, 0.98, 1.0]);
    renderer.draw_text(
        &format!(
            "Player: {}",
            if mission.player == 0 { "GDI" } else { "Nod" }
        ),
        60.0,
        108.0,
        10.0,
        [0.75, 0.82, 0.92, 1.0],
    );
    let mut y = 150.0;
    if !mission.briefing.is_empty() {
        renderer.draw_text("BRIEFING", 60.0, y, 10.0, [0.60, 0.75, 0.95, 1.0]);
        y += 18.0;
        renderer.draw_text(&mission.briefing, 60.0, y, 11.0, [0.88, 0.90, 0.95, 1.0]);
        y += 28.0;
    }
    if !mission.objectives.is_empty() {
        renderer.draw_text("OBJECTIVES", 60.0, y, 10.0, [0.60, 0.75, 0.95, 1.0]);
        y += 18.0;
        for objective in &mission.objectives {
            renderer.draw_text(
                &format!("- {}", objective.text),
                60.0,
                y,
                11.0,
                [0.82, 0.88, 0.96, 1.0],
            );
            y += 16.0;
        }
    }
    renderer.draw_text(
        "Enter: begin mission    Esc: quit",
        60.0,
        h - 60.0,
        10.0,
        [0.70, 0.80, 0.92, 1.0],
    );
    false
}

/// Shared texture slot a briefing video borrows: the RTS tileset slot
/// ([`cnc_renderer::TILESET_TEXTURE_ID`]). A briefing screen draws no terrain
/// or units, so the slot is idle while a briefing is shown. The decoded frame
/// is uploaded there and drawn aspect-fit; the owning tileset atlas is
/// re-uploaded at the call site after the frame is presented so the mission
/// that follows still samples the real tileset (the batch carries only two
/// atlas slots; see cnc-renderer's `MAX_ATLAS_TEXTURES`). No `.VQA` assets are
/// bundled yet, so the runtime path always begins in the text fallback.
const BRIEFING_VIDEO_TEXTURE_ID: u32 = cnc_renderer::TILESET_TEXTURE_ID;

/// Deterministic synthetic display palette for a decoded VQA frame.
///
/// A v1 container carries a 256-entry codebook of 2×2 block pixel *indices*,
/// not a display palette, and the asset crate deliberately does not parse the
/// 768-byte VQA palette yet (see [`cnc_asset::video`]). Until a real palette
/// ships, each index maps to a fixed, non-black colour so a decoded frame
/// renders as visible "codebook colour" and asserts crisply headlessly.
fn vqa_index_to_rgba(index: u8) -> [u8; 4] {
    [
        index.saturating_add(16),
        index.wrapping_mul(2).saturating_add(24),
        index.wrapping_mul(3).saturating_add(48),
        255,
    ]
}

/// Expand a codebook-decoded 8-bit index image into RGBA8 using
/// [`vqa_index_to_rgba`]. Output is `4 * indices.len()` bytes.
fn vqa_indices_to_rgba(indices: &[u8]) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(indices.len() * 4);
    for &index in indices {
        rgba.extend_from_slice(&vqa_index_to_rgba(index));
    }
    rgba
}

/// Resolve a briefing video's first decoded RGBA frame from raw container bytes.
///
/// Demuxes the v1 IFF container, takes the first `CBF0` codebook and first
/// `VQFL` full frame, decodes it to 8-bit indices and converts them to RGBA.
/// Returns `(rgba, width_px, height_px)`; returns `None` on *any* demux or
/// decode error (missing codebook/frame, unsupported variant, dimension
/// mismatch) so callers fall back to the text briefing panel.
fn briefing_video_first_frame(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    use cnc_asset::video::{decode_frame, demux, VqaChunk};

    let vqa = demux(bytes).ok()?;
    let codebook = vqa.chunks.iter().find_map(|chunk| match chunk {
        VqaChunk::CodebookFull { data } => Some(data),
        _ => None,
    })?;
    let frame = vqa.chunks.iter().find_map(|chunk| match chunk {
        VqaChunk::FrameFull { data } => Some(data),
        _ => None,
    })?;
    let indices = decode_frame(
        codebook,
        frame,
        vqa.header.width_blocks,
        vqa.header.height_blocks,
    )
    .ok()?;
    let width_px = u32::from(vqa.header.width_blocks) * 2;
    let height_px = u32::from(vqa.header.height_blocks) * 2;
    Some((vqa_indices_to_rgba(&indices), width_px, height_px))
}

/// Resolve a briefing video's raw container bytes from disk.
///
/// Mirrors the launcher's other `CNC_ASSETS`-relative asset loads (see
/// `LauncherApp::resumed`): returns `None` when the key is absent/empty or the
/// file is unreadable. `Path::join` with an absolute key resolves to that
/// absolute path (a std path join quirk the headless tests lean on).
fn read_briefing_asset(image_key: Option<&str>) -> Option<Vec<u8>> {
    let key = image_key?;
    if key.is_empty() {
        return None;
    }
    let assets_dir = std::env::var("CNC_ASSETS").unwrap_or_else(|_| "assets".to_string());
    let path = Path::new(&assets_dir).join(key);
    match std::fs::read(&path) {
        Ok(bytes) => Some(bytes),
        Err(_) => {
            tracing::debug!(%key, dir = %assets_dir, "briefing video asset unreadable; text panel");
            None
        }
    }
}

/// Upload one decoded RGBA frame into the borrowed briefing slot and draw it
/// aspect-fit and centered over `width x height`.
///
/// Returns `true` when the frame uploaded and drew, `false` when the backend
/// rejected the upload (no atlas support) or the frame is empty — the caller
/// keeps the text panel. This is the render half only (no resolve, no decode),
/// so the headless tests drive it directly on [`SoftwareRenderer`].
fn draw_video_frame(
    renderer: &mut dyn Renderer,
    rgba: &[u8],
    frame_w: u32,
    frame_h: u32,
    width: u32,
    height: u32,
) -> bool {
    if frame_w == 0 || frame_h == 0 {
        return false;
    }
    if renderer
        .set_atlas_texture(BRIEFING_VIDEO_TEXTURE_ID, frame_w, frame_h, rgba)
        .is_err()
    {
        return false;
    }
    let (screen_w, screen_h) = (width.max(1) as f32, height.max(1) as f32);
    let scale = (screen_w / frame_w as f32).min(screen_h / frame_h as f32);
    let draw_w = (frame_w as f32 * scale).max(1.0);
    let draw_h = (frame_h as f32 * scale).max(1.0);
    let origin_x = (screen_w - draw_w) * 0.5;
    let origin_y = (screen_h - draw_h) * 0.5;
    let mut sprite = Sprite::new(
        BRIEFING_VIDEO_TEXTURE_ID,
        Vec2::new(
            Fixed16_16::from_f32(origin_x),
            Fixed16_16::from_f32(origin_y),
        ),
        Vec2::new(Fixed16_16::from_f32(draw_w), Fixed16_16::from_f32(draw_h)),
    );
    sprite.uv_rect = cnc_core::Rect::new(0, 0, frame_w as i32, frame_h as i32);
    renderer.draw_sprite(&sprite);
    true
}

/// Draw a briefing video still (the first decoded frame) for the mission.
///
/// Returns `true` when a real frame was drawn — the caller then adds its own
/// title/footer overlay. Returns `false` for a missing key, an unreadable
/// file, an unsupported/malformed container, or a backend without atlas
/// support, so the caller keeps the existing text panel. Never panics.
fn draw_briefing_video(
    renderer: &mut dyn Renderer,
    image_key: Option<&str>,
    width: u32,
    height: u32,
) -> bool {
    let Some(bytes) = read_briefing_asset(image_key) else {
        return false;
    };
    let Some((rgba, frame_w, frame_h)) = briefing_video_first_frame(&bytes) else {
        return false;
    };
    renderer.draw_rect(
        0.0,
        0.0,
        width as f32,
        height as f32,
        [0.03, 0.03, 0.04, 1.0],
    );
    draw_video_frame(renderer, &rgba, frame_w, frame_h, width, height)
}

/// Deterministic RA briefing-video asset key for a campaign mission index.
///
/// `RaGame` carries only `briefing`/`objectives`, not the scenario's
/// `BriefingImage` key, so the RA hook resolves a fixed convention. No bundled
/// `.VQA` matches it yet, so RA always takes the text fallback today; a real
/// asset (or surfacing `Scenario::briefing_image`) wires the path up.
fn ra_briefing_video_key(index: usize) -> String {
    format!("briefings/ra_{index:02}.vqa")
}

/// Tileset palette a screen needs. LauncherApp draws the TD screens and
/// RaLauncherApp the RA screens, so each shell binds exactly one palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TilesetPalette {
    Ra,
    Td,
}

/// Terrain class normalized for tileset family lookup. The row constants match
/// the mapping used by the map editor's `terrain_family` and the RA game's
/// `terrain_tile` helper (both in other crates): grass/sand/rock live in the
/// `terrain` category, deep water in `water_edges`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PackTerrain {
    Grass,
    Water,
    Sand,
    Rock,
}

impl PackTerrain {
    /// (category, family, fallback manifest row) for the tileset pack.
    fn tile(self) -> (&'static str, &'static str, u8) {
        match self {
            PackTerrain::Grass => ("terrain", "grass", 0),
            PackTerrain::Water => ("water_edges", "deep_water", 0),
            PackTerrain::Sand => ("terrain", "sand", 2),
            PackTerrain::Rock => ("terrain", "rock", 4),
        }
    }

    /// RA terrain cell class (Beach renders as the sand family, matching the
    /// RA game's terrain_tile helper).
    fn from_ra(terrain: cnc_red_alert::Terrain) -> Self {
        match terrain {
            cnc_red_alert::Terrain::Grass => PackTerrain::Grass,
            cnc_red_alert::Terrain::Water => PackTerrain::Water,
            cnc_red_alert::Terrain::Beach => PackTerrain::Sand,
            cnc_red_alert::Terrain::Rock => PackTerrain::Rock,
        }
    }

    /// TD terrain byte: `0` renders the rock family, every other value the
    /// sand family — the same split the engine's `render_with_tiles` uses and
    /// the launcher's flat minimap colours imply (grey vs. brown).
    fn from_td(terrain_type: u8) -> Self {
        if terrain_type == 0 {
            PackTerrain::Rock
        } else {
            PackTerrain::Sand
        }
    }

    /// Raw campaign INI terrain code (RA campaign preview): 1 water, 2 sand,
    /// 4 rock, everything else grass — matching the flat colours in
    /// `draw_ra_campaign`.
    fn from_campaign(id: u8) -> Self {
        match id {
            1 => PackTerrain::Water,
            2 => PackTerrain::Sand,
            4 => PackTerrain::Rock,
            _ => PackTerrain::Grass,
        }
    }
}

/// Draw one map cell as a tileset sprite when the pack is loaded and the
/// renderer's tileset texture slot is ready; otherwise fall back to the legacy
/// flat-coloured rect (`fallback_color`), preserving the pre-tileset look.
///
/// Returns true when a tileset sprite was drawn. Callers composite their ore /
/// tiberium markers *on top* only in that case; the flat fallback keeps the
/// original opaque marker colour semantics.
///
/// Callers pick the tileset family themselves via [`PackTerrain::tile`] for
/// base terrain (grass/sand/rock/water), water_edges shore rows for shore
/// water, and transitions sand_shore for Beach.
///
/// `col_override` pins the tile column: the transitions sheet's columns are
/// 4-bit edge masks (N=1, E=2, S=4, W=8), so mask-driven categories must pass
/// the computed mask here instead of receiving the deterministic per-cell
/// variant hash used for plain texture-variant rows.
fn draw_pack_tile_family(
    renderer: &mut dyn Renderer,
    tileset: Option<&TilesetPack>,
    category: &str,
    family: &str,
    fallback_row: u8,
    col_override: Option<u8>,
    cell_x: u32,
    cell_y: u32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    fallback_color: [f32; 4],
) -> bool {
    use cnc_asset::tileset_pack::GRID;
    let Some(pack) = tileset.filter(|_| renderer.texture_ready(cnc_renderer::TILESET_TEXTURE_ID))
    else {
        renderer.draw_rect(x, y, w, h, fallback_color);
        return false;
    };
    let row = pack
        .family_row(category, family)
        .unwrap_or(fallback_row)
        .min((GRID - 1) as u8);
    let grid = GRID as usize;
    // Deterministic per-cell column variant, same scheme as the engine and
    // editor terrain code: (x * 7 + y * 13) mod 16 — unless the caller pinned
    // a mask column for mask-driven categories (transitions).
    let col = col_override
        .map(|col| col as usize % grid)
        .unwrap_or_else(|| ((cell_x as usize % grid) * 7 + (cell_y as usize % grid) * 13) % grid);
    let Some((px, py, pw, ph)) = pack.tile_px_rect(category, row, col as u8) else {
        renderer.draw_rect(x, y, w, h, fallback_color);
        return false;
    };
    let mut sprite = Sprite::new(
        cnc_renderer::TILESET_TEXTURE_ID,
        Vec2::new(Fixed16_16::from_f32(x), Fixed16_16::from_f32(y)),
        Vec2::new(Fixed16_16::from_f32(w + 0.5), Fixed16_16::from_f32(h + 0.5)),
    );
    sprite.uv_rect = cnc_core::Rect::new(px as i32, py as i32, pw as i32, ph as i32);
    renderer.draw_sprite(&sprite);
    true
}

/// Draw a `resources_fx` sprite (ore chunks, tiberium growth stages) over an
/// already drawn terrain tile. Only draws when the pack is loaded and the
/// tileset texture is ready; returns false otherwise so callers keep their
/// legacy flat marker exactly as before (RGBA fx sheets composite over the
/// terrain tiles in the combined atlas).
fn draw_pack_fx(
    renderer: &mut dyn Renderer,
    tileset: Option<&TilesetPack>,
    family: &str,
    fallback_row: u8,
    cell_x: u32,
    cell_y: u32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) -> bool {
    let pack_ready = tileset.is_some() && renderer.texture_ready(cnc_renderer::TILESET_TEXTURE_ID);
    if !pack_ready {
        return false;
    }
    draw_pack_tile_family(
        renderer,
        tileset,
        "resources_fx",
        family,
        fallback_row,
        None,
        cell_x,
        cell_y,
        x,
        y,
        w,
        h,
        // Never visible: draw_pack_tile_family only reaches the fallback
        // rect when the tileset cannot supply the tile, and fx callers treat
        // that as "no sprite" and fall back themselves.
        [0.0, 0.0, 0.0, 0.0],
    )
}

/// Water-edge family for a water cell, keyed by the 4-bit land mask of its
/// orthogonal neighbours (N=1, E=2, S=4, W=8) — the same mask the TD engine's
/// `water_edges_layout` and the map editor's `water_shore_family` consume and
/// the pack's `water_edges` art was generated for. The pack's shore art names
/// the side its *water* half occupies, so land to the north selects
/// `shore_south` (water in the south half) and corners follow the same rule.
/// Opposite land pairs select the river-flow families; a water cell ringed by
/// land falls back to `shallow_water`. Returns None for open water (plain
/// deep_water).
fn water_shore_tile(land_mask: u8) -> Option<(&'static str, u8)> {
    match land_mask {
        0 => None,
        0b0001 => Some(("shore_south", 4)), // land at north
        0b0010 => Some(("shore_west", 5)),  // land at east
        0b0100 => Some(("shore_north", 2)), // land at south
        0b1000 => Some(("shore_east", 3)),  // land at west
        0b0011 => Some(("shore_sw", 8)),    // land at north+east
        0b0110 => Some(("shore_nw", 6)),    // land at east+south
        0b1001 => Some(("shore_se", 9)),    // land at north+west
        0b1100 => Some(("shore_ne", 7)),    // land at south+west
        // Opposite land pairs: the water flows through.
        0b0101 => Some(("river_flow_e_w", 11)),
        0b1010 => Some(("river_flow_n_s", 10)),
        // Three land sides: one side stays open to water.
        0b1110 => Some(("shore_north", 2)),
        0b1101 => Some(("shore_east", 3)),
        0b1011 => Some(("shore_south", 4)),
        0b0111 => Some(("shore_west", 5)),
        _ => Some(("shallow_water", 1)), // ringed by land on all sides
    }
}

/// Tileset family for one RA terrain cell at (`px`, `py`): water against land
/// picks water_edges shore rows computed directly from `map.terrain`, open
/// water stays deep_water; Beach renders the transitions sand_shore row with
/// the column set to the 4-bit mask of water neighbours (the transitions
/// sheet's columns are edge masks: N=1, E=2, S=4, W=8 — a set bit draws the
/// water half against that side); the rest use the base terrain families.
/// The fourth tuple element overrides the per-cell variant column for
/// mask-driven categories and is None for plain variant rows.
fn ra_terrain_tile(
    map: &cnc_red_alert::Map,
    px: u32,
    py: u32,
) -> (&'static str, &'static str, u8, Option<u8>) {
    let is_water = |cx: i32, cy: i32| -> bool {
        if cx < 0 || cy < 0 || cx as u32 >= map.width || cy as u32 >= map.height {
            return false;
        }
        map.terrain
            .get(cy as usize * map.width as usize + cx as usize)
            .copied()
            .unwrap_or(cnc_red_alert::Terrain::Water)
            == cnc_red_alert::Terrain::Water
    };
    let terrain = map
        .terrain
        .get(py as usize * map.width as usize + px as usize)
        .copied()
        .unwrap_or(cnc_red_alert::Terrain::Grass);
    if terrain == cnc_red_alert::Terrain::Water {
        // Off-map sides are never land, so shore edges only point at real
        // cells (open map borders stay open water).
        let is_land = |cx: i32, cy: i32| -> bool {
            cx >= 0
                && cy >= 0
                && (cx as u32) < map.width
                && (cy as u32) < map.height
                && !is_water(cx, cy)
        };
        let land_mask = land_mask_at(px as i32, py as i32, is_land);
        if let Some((family, row)) = water_shore_tile(land_mask) {
            return ("water_edges", family, row, None);
        }
        return ("water_edges", "deep_water", 0, None);
    }
    if terrain == cnc_red_alert::Terrain::Beach {
        // Edge mask of the water sides: the sand_shore art draws its water
        // half (terrain B) against every side whose bit is set, so the blend
        // meets the actual neighbouring water cells.
        let water_mask = land_mask_at(px as i32, py as i32, is_water);
        return ("transitions", "sand_shore", 10, Some(water_mask));
    }
    let (category, family, row) = PackTerrain::from_ra(terrain).tile();
    (category, family, row, None)
}

/// 4-bit orthogonal neighbour mask (N=1, E=2, S=4, W=8) for the cell at
/// (`cx`, `cy`) using `matches`. Out-of-map coordinates never match.
fn land_mask_at(cx: i32, cy: i32, matches: impl Fn(i32, i32) -> bool) -> u8 {
    let n = matches(cx, cy - 1) as u8;
    let e = matches(cx + 1, cy) as u8;
    let s = matches(cx, cy + 1) as u8;
    let w = matches(cx - 1, cy) as u8;
    n | (e << 1) | (s << 2) | (w << 3)
}

/// Tileset family for one raw campaign terrain cell (codes: 1 water, 2 sand,
/// 4 rock, otherwise grass): water against land picks water_edges shore rows
/// computed over the campaign grid, open water stays deep_water, the rest use
/// the base terrain families. Column override stays None: campaign previews
/// have no transition-category cells.
fn campaign_terrain_tile(
    terrain: &[u8],
    width: u32,
    height: u32,
    px: u32,
    py: u32,
) -> (&'static str, &'static str, u8, Option<u8>) {
    let code = |cx: i32, cy: i32| -> u8 {
        if cx < 0 || cy < 0 || cx as u32 >= width || cy as u32 >= height {
            return 0;
        }
        terrain
            .get(cy as usize * width as usize + cx as usize)
            .copied()
            .unwrap_or(0)
    };
    if code(px as i32, py as i32) == 1 {
        let is_land = |cx: i32, cy: i32| -> bool {
            let in_map = cx >= 0 && cy >= 0 && (cx as u32) < width && (cy as u32) < height;
            in_map && code(cx, cy) != 1
        };
        let land_mask = land_mask_at(px as i32, py as i32, is_land);
        if let Some((family, row)) = water_shore_tile(land_mask) {
            return ("water_edges", family, row, None);
        }
        return ("water_edges", "deep_water", 0, None);
    }
    let (category, family, row) = PackTerrain::from_campaign(code(px as i32, py as i32)).tile();
    (category, family, row, None)
}

/// resources_fx family for a TD tiberium cell chosen by amount bucket
/// (amount 0-255, the same range the engine's tiberium stages use): early
/// growth draws a sprout, mid growth a cluster, mature cells a bloom.
fn tiberium_fx_family(amount: u8) -> (&'static str, u8) {
    if amount < 86 {
        ("tiberium_sprout", 0)
    } else if amount < 171 {
        ("tiberium_cluster", 1)
    } else {
        ("tiberium_bloom", 14)
    }
}

/// Bottom-right terrain/unit minimap for the TD game view.
fn draw_td_minimap(
    renderer: &mut dyn Renderer,
    game: &Game,
    tileset: Option<&TilesetPack>,
    player: u8,
    center: Vec2,
    view_w: f32,
    view_h: f32,
    left: f32,
    top: f32,
    hud_w: f32,
    hud_h: f32,
) {
    let map_w = game.map.width.max(1);
    let map_h = game.map.height.max(1);
    let (step, scale) = minimap_layout(map_w, map_h, 150.0);
    // Bottom-left keeps the minimap clear of the right-hand build menu. The
    // panel is placed in screen-sized HUD space; the view box below uses the
    // real world view, which shrinks as the camera zooms in.
    let (x, y, panel_w, panel_h) = minimap_origin(map_w, map_h, left, top, hud_w, hud_h, false);
    renderer.draw_rect(
        x - 4.0,
        y - 4.0,
        panel_w + 8.0,
        panel_h + 8.0,
        [0.02, 0.03, 0.05, 1.0],
    );
    for py in (0..map_h).step_by(step as usize) {
        for px in (0..map_w).step_by(step as usize) {
            let index = py as usize * map_w as usize + px as usize;
            let Some(cell) = game.map.grid.cells.get(index) else {
                continue;
            };
            // Base terrain: rock family for terrain_type 0, sand otherwise —
            // mirroring the engine's tile mapping. Tiberium cells use the
            // tiberium_ground family as the underlay (terrain and resources_fx
            // category sheets compose over each other). The flat fallback
            // colours keep the legacy look when no tile art is available.
            let (category, family, fallback_row, col_override) = if cell.has_tiberium {
                ("terrain", "tiberium_ground", 5, None)
            } else {
                let (category, family, fallback_row) =
                    PackTerrain::from_td(cell.terrain_type).tile();
                (category, family, fallback_row, None)
            };
            let fallback = if cell.has_tiberium {
                [0.10, 0.60, 0.15, 1.0]
            } else if cell.terrain_type == 0 {
                [0.30, 0.30, 0.30, 1.0]
            } else {
                [0.45, 0.40, 0.25, 1.0]
            };
            let tile_x = x + px as f32 * scale;
            let tile_y = y + py as f32 * scale;
            let tile_s = scale * step as f32;
            let cell_x = px;
            let cell_y = py;
            let drew_tile = draw_pack_tile_family(
                renderer,
                tileset,
                category,
                family,
                fallback_row,
                col_override,
                cell_x,
                cell_y,
                tile_x,
                tile_y,
                tile_s,
                tile_s,
                fallback,
            );
            // Growth-stage sprite replaces the flat green tint; the legacy
            // translucent tint stays whenever no tile art is available.
            if cell.has_tiberium && drew_tile {
                let (fx_family, fx_row) = tiberium_fx_family(cell.tiberium_amount);
                if !draw_pack_fx(
                    renderer, tileset, fx_family, fx_row, cell_x, cell_y, tile_x, tile_y, tile_s,
                    tile_s,
                ) {
                    renderer.draw_rect(tile_x, tile_y, tile_s, tile_s, [0.10, 0.60, 0.15, 0.55]);
                }
            }
        }
    }
    for building in &game.buildings {
        if !building.is_alive() {
            continue;
        }
        let cell_x = (building.position.x.to_f32() / cnc_tiberian_dawn::map::TILE_SIZE).floor();
        let cell_y = (building.position.y.to_f32() / cnc_tiberian_dawn::map::TILE_SIZE).floor();
        let color = if building.owner == player {
            [0.35, 0.85, 1.0, 1.0]
        } else {
            [1.0, 0.35, 0.30, 1.0]
        };
        renderer.draw_rect(
            x + cell_x * scale,
            y + cell_y * scale,
            (scale * 2.0).max(2.0),
            (scale * 2.0).max(2.0),
            color,
        );
    }
    for unit in &game.units {
        if !unit.is_alive() {
            continue;
        }
        let cell_x = (unit.position.x.to_f32() / cnc_tiberian_dawn::map::TILE_SIZE).floor();
        let cell_y = (unit.position.y.to_f32() / cnc_tiberian_dawn::map::TILE_SIZE).floor();
        let color = if unit.owner == player {
            [0.65, 1.0, 0.75, 1.0]
        } else {
            [1.0, 0.55, 0.45, 1.0]
        };
        renderer.draw_rect(
            x + cell_x * scale,
            y + cell_y * scale,
            scale.max(1.5),
            scale.max(1.5),
            color,
        );
    }
    let view_left = (center.x.to_f32() - view_w * 0.5) / cnc_tiberian_dawn::map::TILE_SIZE;
    let view_top = (center.y.to_f32() - view_h * 0.5) / cnc_tiberian_dawn::map::TILE_SIZE;
    let view_cells_w = view_w / cnc_tiberian_dawn::map::TILE_SIZE;
    let view_cells_h = view_h / cnc_tiberian_dawn::map::TILE_SIZE;
    let border = [0.95, 0.90, 0.75, 1.0];
    renderer.draw_rect(
        x + view_left * scale,
        y + view_top * scale,
        view_cells_w * scale,
        1.0,
        border,
    );
    renderer.draw_rect(
        x + view_left * scale,
        y + (view_top + view_cells_h) * scale - 1.0,
        view_cells_w * scale,
        1.0,
        border,
    );
    renderer.draw_rect(
        x + view_left * scale,
        y + view_top * scale,
        1.0,
        view_cells_h * scale,
        border,
    );
    renderer.draw_rect(
        x + (view_left + view_cells_w) * scale - 1.0,
        y + view_top * scale,
        1.0,
        view_cells_h * scale,
        border,
    );
    renderer.draw_text("MINIMAP", x, y - 13.0, 8.0, [0.60, 0.75, 0.95, 1.0]);
}

/// TD pause overlay (screen space).
fn draw_td_pause(
    renderer: &mut dyn Renderer,
    game: &Game,
    choice: usize,
    save_slot: usize,
    slot_label: &str,
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.02, 0.03, 0.05, 1.0]);
    let panel_w = 460.0f32;
    let panel_h = 260.0f32;
    let panel_x = (w - panel_w) * 0.5;
    let panel_y = (h - panel_h) * 0.5;
    renderer.draw_rect(panel_x, panel_y, panel_w, panel_h, [0.07, 0.09, 0.13, 1.0]);
    renderer.draw_rect(panel_x, panel_y, panel_w, 2.0, [0.70, 0.55, 0.25, 1.0]);
    renderer.draw_text(
        "PAUSED",
        panel_x + 24.0,
        panel_y + 20.0,
        22.0,
        [0.95, 0.85, 0.60, 1.0],
    );
    let status = game.status();
    renderer.draw_text(
        &format!(
            "t={}s   {} units   {} structures   GDI {} credits",
            game.elapsed.int(),
            status.units,
            status.buildings,
            status.gdi_credits
        ),
        panel_x + 24.0,
        panel_y + 58.0,
        10.0,
        [0.78, 0.86, 0.95, 1.0],
    );
    let options = [
        "Resume".to_string(),
        "Save game".to_string(),
        "Load game".to_string(),
        "Abort mission".to_string(),
        format!("Save slot: {}/3 ({slot_label})", save_slot + 1),
    ];
    let mut y = panel_y + 100.0;
    for (index, label) in options.iter().enumerate() {
        let color = if index == choice {
            [1.0, 0.90, 0.45, 1.0]
        } else {
            [0.78, 0.86, 0.95, 1.0]
        };
        renderer.draw_text(
            &format!("{}{}", if index == choice { "> " } else { "  " }, label),
            panel_x + 24.0,
            y,
            12.0,
            color,
        );
        y += 24.0;
    }
    renderer.draw_text(
        "Up/Down: select   Enter: confirm   Esc: resume",
        panel_x + 24.0,
        panel_y + panel_h - 24.0,
        9.0,
        [0.60, 0.70, 0.85, 1.0],
    );
}

/// One spawn point extracted from a TD mission's scripts/triggers.
#[derive(Clone, Debug, PartialEq, Eq)]
struct MissionSpawn {
    owner: u8,
    cell: Vec2i,
    structure: bool,
}

/// A TD mission-select entry plus preview data.
#[derive(Clone, Debug)]
struct TdChoice {
    path: PathBuf,
    name: String,
    objectives: usize,
    triggers: usize,
    spawns: Vec<MissionSpawn>,
}

fn collect_action_spawns(action: &cnc_tiberian_dawn::Action, out: &mut Vec<MissionSpawn>) {
    match action {
        cnc_tiberian_dawn::Action::SpawnUnits {
            owner, count, cell, ..
        } => {
            for _ in 0..*count {
                out.push(MissionSpawn {
                    owner: *owner,
                    cell: *cell,
                    structure: false,
                });
            }
        }
        cnc_tiberian_dawn::Action::SpawnStructure { owner, cell, .. } => {
            out.push(MissionSpawn {
                owner: *owner,
                cell: *cell,
                structure: true,
            });
        }
        _ => {}
    }
}

fn mission_spawns(mission: &Mission) -> Vec<MissionSpawn> {
    let mut spawns = Vec::new();
    for script in &mission.scripts {
        for action in &script.actions {
            collect_action_spawns(action, &mut spawns);
        }
    }
    for trigger in &mission.triggers {
        for action in &trigger.actions {
            collect_action_spawns(action, &mut spawns);
        }
    }
    spawns
}

/// Draw a spawn-marker overview for the selected mission.
fn draw_td_mission_preview(
    renderer: &mut dyn Renderer,
    choice: &TdChoice,
    x: f32,
    y: f32,
    size: f32,
) {
    renderer.draw_rect(x, y, size, size + 58.0, [0.03, 0.04, 0.06, 1.0]);
    renderer.draw_rect(x, y, size, 1.5, [0.55, 0.45, 0.22, 1.0]);
    renderer.draw_text(
        "SPAWN OVERVIEW",
        x + 12.0,
        y + 10.0,
        10.0,
        [0.95, 0.82, 0.45, 1.0],
    );
    renderer.draw_text(
        &format!(
            "{} objectives   {} triggers",
            choice.objectives, choice.triggers
        ),
        x + 12.0,
        y + 26.0,
        9.0,
        [0.72, 0.78, 0.88, 1.0],
    );
    let extent = choice
        .spawns
        .iter()
        .map(|spawn| spawn.cell.x.max(spawn.cell.y))
        .max()
        .map_or(32, |max| (max + 4).clamp(16, 64));
    let scale = (size - 24.0) / extent as f32;
    let grid_x = x + 12.0;
    let grid_y = y + 44.0;
    renderer.draw_rect(
        grid_x,
        grid_y,
        extent as f32 * scale,
        extent as f32 * scale,
        [0.07, 0.08, 0.10, 1.0],
    );
    for step in (0..=extent).step_by(8) {
        let line = step as f32 * scale;
        renderer.draw_rect(
            grid_x + line,
            grid_y,
            1.0,
            extent as f32 * scale,
            [0.14, 0.16, 0.20, 1.0],
        );
        renderer.draw_rect(
            grid_x,
            grid_y + line,
            extent as f32 * scale,
            1.0,
            [0.14, 0.16, 0.20, 1.0],
        );
    }
    for spawn in &choice.spawns {
        let color = if spawn.owner == 0 {
            [0.35, 0.60, 1.0, 1.0]
        } else {
            [1.0, 0.38, 0.32, 1.0]
        };
        let marker = if spawn.structure { 5.0 } else { 3.5 };
        renderer.draw_rect(
            grid_x + spawn.cell.x as f32 * scale,
            grid_y + spawn.cell.y as f32 * scale,
            marker,
            marker,
            color,
        );
    }
    renderer.draw_text(
        "blue GDI   red Nod   square = structure",
        x + 12.0,
        y + size + 20.0,
        8.0,
        [0.62, 0.68, 0.78, 1.0],
    );
    renderer.draw_text(
        "preview from mission spawns (no terrain in INI)",
        x + 12.0,
        y + size + 34.0,
        8.0,
        [0.50, 0.56, 0.66, 1.0],
    );
}

fn draw_td_mission_select(
    renderer: &mut dyn Renderer,
    choices: &[TdChoice],
    completed: &[bool],
    best_times: &[Option<i32>],
    selected: usize,
    filter: CampaignFilter,
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.05, 0.05, 0.07, 1.0]);
    renderer.draw_rect(0.0, 110.0, w, 2.0, [0.70, 0.55, 0.25, 1.0]);
    renderer.draw_text("TIBERIAN DAWN", 60.0, 40.0, 28.0, [0.95, 0.70, 0.30, 1.0]);
    renderer.draw_text("Select Mission", 60.0, 84.0, 15.0, [0.88, 0.90, 0.95, 1.0]);
    if choices.is_empty() {
        renderer.draw_text(
            "No missions found in this directory.",
            60.0,
            160.0,
            12.0,
            [0.95, 0.70, 0.45, 1.0],
        );
    } else {
        let mut y = 150.0;
        let visible: Vec<usize> = (0..choices.len())
            .filter(|index| match filter {
                CampaignFilter::All => true,
                CampaignFilter::Incomplete => !completed.get(*index).copied().unwrap_or(false),
                CampaignFilter::Completed => completed.get(*index).copied().unwrap_or(false),
            })
            .collect();
        if visible.is_empty() {
            renderer.draw_text(
                "No missions match this filter.",
                60.0,
                y,
                12.0,
                [0.90, 0.75, 0.55, 1.0],
            );
            y += 24.0;
        }
        for index in visible {
            let name = &choices[index].name;
            let done = completed.get(index).copied().unwrap_or(false);
            let color = if index == selected {
                [1.0, 0.90, 0.45, 1.0]
            } else if done {
                [0.60, 0.85, 0.65, 1.0]
            } else {
                [0.82, 0.87, 0.94, 1.0]
            };
            let best = best_times
                .get(index)
                .and_then(|seconds| *seconds)
                .map_or_else(|| "--".to_string(), |seconds| format!("{seconds}s"));
            renderer.draw_text(
                &format!(
                    "{}{} {}. {}    best {}",
                    if index == selected { "> " } else { "  " },
                    if done { "[x]" } else { "[ ]" },
                    index + 1,
                    name,
                    best
                ),
                60.0,
                y,
                12.0,
                color,
            );
            y += 24.0;
        }
    }
    if let Some(choice) = choices.get(selected) {
        draw_td_mission_preview(renderer, choice, (w - 370.0).max(60.0), 150.0, 320.0);
    }
    renderer.draw_text(
        &format!(
            "Filter: {} (F)    Up/Down: select   Enter: deploy   S: stats   Esc: quit",
            filter.label()
        ),
        60.0,
        h - 60.0,
        10.0,
        [0.70, 0.78, 0.90, 1.0],
    );
}

fn draw_td_campaign_complete(
    renderer: &mut dyn Renderer,
    missions: usize,
    best_times: &[Option<i32>],
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.05, 0.05, 0.07, 1.0]);
    renderer.draw_rect(0.0, 110.0, w, 2.0, [0.70, 0.55, 0.25, 1.0]);
    renderer.draw_text("TIBERIAN DAWN", 60.0, 40.0, 28.0, [0.95, 0.70, 0.30, 1.0]);
    renderer.draw_text(
        "CAMPAIGN COMPLETE",
        60.0,
        84.0,
        22.0,
        [0.55, 0.95, 0.65, 1.0],
    );
    renderer.draw_text(
        &format!("{missions}/{missions} missions cleared"),
        60.0,
        150.0,
        13.0,
        [0.90, 0.93, 0.98, 1.0],
    );
    let total: i32 = best_times.iter().filter_map(|time| *time).sum();
    renderer.draw_text(
        &format!("Total best time: {total}s"),
        60.0,
        176.0,
        12.0,
        [0.85, 0.82, 0.65, 1.0],
    );
    renderer.draw_text(
        "Enter or Esc: back to mission select",
        60.0,
        h - 60.0,
        11.0,
        [0.70, 0.80, 0.92, 1.0],
    );
}

fn draw_td_stats(
    renderer: &mut dyn Renderer,
    choices: &[TdChoice],
    completed: &[bool],
    best_times: &[Option<i32>],
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.05, 0.05, 0.07, 1.0]);
    renderer.draw_rect(0.0, 110.0, w, 2.0, [0.70, 0.55, 0.25, 1.0]);
    renderer.draw_text(
        "TD CAMPAIGN STATS",
        60.0,
        40.0,
        24.0,
        [0.95, 0.70, 0.30, 1.0],
    );
    let cleared = completed.iter().filter(|done| **done).count();
    renderer.draw_text(
        &format!("{cleared}/{} missions cleared", choices.len()),
        60.0,
        82.0,
        13.0,
        [0.90, 0.93, 0.98, 1.0],
    );
    let mut y = 128.0;
    let mut total = 0i32;
    for (index, choice) in choices.iter().enumerate() {
        let name = &choice.name;
        let done = completed.get(index).copied().unwrap_or(false);
        let best = best_times.get(index).and_then(|seconds| *seconds);
        if let Some(seconds) = best {
            total += seconds;
        }
        let color = if done {
            [0.60, 0.85, 0.65, 1.0]
        } else {
            [0.82, 0.87, 0.94, 1.0]
        };
        renderer.draw_text(
            &format!(
                "{} {}. {}    {}",
                if done { "[x]" } else { "[ ]" },
                index + 1,
                name,
                best.map_or_else(|| "--".to_string(), |seconds| format!("{seconds}s"))
            ),
            60.0,
            y,
            11.0,
            color,
        );
        y += 20.0;
    }
    renderer.draw_text(
        &format!("Total best time: {total}s"),
        60.0,
        y + 6.0,
        11.0,
        [0.85, 0.82, 0.65, 1.0],
    );
    renderer.draw_text(
        "Enter/Esc: back to mission select",
        60.0,
        h - 60.0,
        10.0,
        [0.70, 0.80, 0.92, 1.0],
    );
}

/// Screen-space camera (world units == pixels) used by the setup screen.
fn screen_camera(width: u32, height: u32) -> Camera {
    let mut camera = Camera::default();
    camera.set_viewport(width.max(1), height.max(1));
    camera.position = Vec2::new(
        Fixed16_16::from_f32(width as f32 * 0.5),
        Fixed16_16::from_f32(height as f32 * 0.5),
    );
    camera.zoom = Fixed16_16::ONE;
    camera
}

/// World-view zoom limits. The HUD is drawn unzoomed, so these only bound how
/// much of the map the camera can show, not the size of menus or status text.
const MIN_GAME_ZOOM: f32 = 0.5;
const MAX_GAME_ZOOM: f32 = 4.0;
/// Starting zoom for a match. 24 px cells at 2x match the original games' field
/// of view much more closely than 1:1 pixels, which shows nearly a whole map.
pub(crate) const DEFAULT_GAME_ZOOM: f32 = 2.0;
/// Multiplier for one keyboard zoom step ("+" / "-").
const KEY_ZOOM_STEP: f32 = 1.2;

/// Screen-space HUD camera: the game camera's world centre without zoom, so
/// status text, the build menu and the minimap keep a fixed pixel size while
/// the map behind them is zoomed in or out.
fn hud_camera(camera: &Camera) -> Camera {
    let mut hud = *camera;
    hud.zoom = Fixed16_16::ONE;
    hud
}

/// Wheel delta to zoom factor; trackpads report fractional pixel deltas, so
/// this stays smooth instead of stepping once per event.
fn wheel_zoom_factor(amount: f32) -> f32 {
    1.15f32.powf(amount)
}

/// Zoom a camera around a screen-space anchor (usually the cursor), clamped to
/// the map so the view centre cannot drift into empty space.
fn zoom_camera_at(
    camera: &mut Camera,
    anchor: Option<(f32, f32)>,
    factor: f32,
    world_width: f32,
    world_height: f32,
) {
    let before = camera.zoom.to_f32().max(1.0e-6);
    let after = (before * factor).clamp(MIN_GAME_ZOOM, MAX_GAME_ZOOM);
    if (after - before).abs() <= f32::EPSILON {
        return;
    }
    let view_w = camera.viewport_width as f32;
    let view_h = camera.viewport_height as f32;
    let (mut px, mut py) = (camera.position.x.to_f32(), camera.position.y.to_f32());
    if let Some((sx, sy)) = anchor {
        // Keep the world point under the cursor pinned while the scale changes.
        let world_x = (sx - view_w * 0.5) / before + px;
        let world_y = (sy - view_h * 0.5) / before + py;
        px = world_x - (sx - view_w * 0.5) / after;
        py = world_y - (sy - view_h * 0.5) / after;
    }
    camera.zoom = Fixed16_16::from_f32(after);
    camera.position = Vec2::new(
        Fixed16_16::from_f32(px.clamp(0.0, world_width)),
        Fixed16_16::from_f32(py.clamp(0.0, world_height)),
    );
}

/// Edge-scroll direction for a screen cursor inside a window (-1, 0 or 1).
fn edge_pan_direction(
    cursor_x: f32,
    cursor_y: f32,
    width: f32,
    height: f32,
    margin: f32,
) -> (f32, f32) {
    if cursor_x < 0.0 || cursor_y < 0.0 || cursor_x > width || cursor_y > height {
        return (0.0, 0.0);
    }
    let mut dx = 0.0;
    let mut dy = 0.0;
    if cursor_x <= margin {
        dx = -1.0;
    } else if cursor_x >= width - margin {
        dx = 1.0;
    }
    if cursor_y <= margin {
        dy = -1.0;
    } else if cursor_y >= height - margin {
        dy = 1.0;
    }
    (dx, dy)
}

/// Production progress in percent (0 when the total is non-positive).
fn production_percent(progress: Fixed16_16, total: Fixed16_16) -> i32 {
    let total = total.to_f32();
    if total <= 0.0 {
        return 0;
    }
    ((progress.to_f32() / total) * 100.0).clamp(0.0, 100.0) as i32
}

/// Health fraction clamped to 0..1 (0 when the maximum is non-positive).
fn health_fraction(current: Fixed16_16, maximum: Fixed16_16) -> f32 {
    let max = maximum.to_f32();
    if max <= 0.0 {
        return 0.0;
    }
    (current.to_f32() / max).clamp(0.0, 1.0)
}

/// Colour for a health bar: green, yellow or red by remaining fraction.
fn health_bar_color(fraction: f32) -> [f32; 4] {
    if fraction > 0.5 {
        [0.35, 0.90, 0.45, 1.0]
    } else if fraction > 0.25 {
        [0.95, 0.82, 0.30, 1.0]
    } else {
        [0.95, 0.35, 0.30, 1.0]
    }
}

/// Draw a small health bar above a world-space entity.
fn draw_health_bar(renderer: &mut dyn Renderer, center_x: f32, y: f32, fraction: f32, width: f32) {
    if fraction >= 0.999 {
        return;
    }
    renderer.draw_rect(
        center_x - width * 0.5,
        y,
        width,
        3.0,
        [0.04, 0.04, 0.05, 1.0],
    );
    renderer.draw_rect(
        center_x - width * 0.5 + 1.0,
        y + 1.0,
        (width - 2.0) * fraction,
        1.0,
        health_bar_color(fraction),
    );
}

/// Short description of a save slot's age ("empty", "42s ago", "3m ago").
fn save_slot_label(path: &Path) -> String {
    let Ok(metadata) = std::fs::metadata(path) else {
        return "empty".to_string();
    };
    let Ok(modified) = metadata.modified() else {
        return "saved".to_string();
    };
    let elapsed = modified.elapsed().unwrap_or_default().as_secs();
    if elapsed < 60 {
        format!("{elapsed}s ago")
    } else if elapsed < 3600 {
        format!("{}m ago", elapsed / 60)
    } else {
        format!("{}h ago", elapsed / 3600)
    }
}

/// First incomplete mission after `current`, wrapping around; None if all done.
fn next_incomplete(completed: &[bool], current: usize) -> Option<usize> {
    let len = completed.len();
    if len == 0 {
        return None;
    }
    let start = (current + 1) % len;
    (0..len)
        .map(|offset| (start + offset) % len)
        .find(|index| !completed.get(*index).copied().unwrap_or(false))
}

/// Map a digit key to a camera-bookmark slot (Ctrl+1..4).
fn bookmark_slot(text: &str) -> Option<usize> {
    match text {
        "1" => Some(0),
        "2" => Some(1),
        "3" => Some(2),
        "4" => Some(3),
        _ => None,
    }
}

/// Deterministic skirmish map used for the setup-screen preview.
fn ra_setup_preview(size: u32) -> cnc_red_alert::Map {
    cnc_red_alert::Map::new(size, size)
}

/// Draw a terrain/ore preview of the skirmish map on the setup screen.
fn draw_ra_setup_preview(
    renderer: &mut dyn Renderer,
    map: &cnc_red_alert::Map,
    tileset: Option<&TilesetPack>,
    x: f32,
    y: f32,
    size: f32,
) {
    let (step, scale) = minimap_layout(map.width, map.height, size);
    let panel_w = map.width as f32 * scale;
    let panel_h = map.height as f32 * scale;
    renderer.draw_rect(
        x - 4.0,
        y - 4.0,
        panel_w + 8.0,
        panel_h + 8.0,
        [0.03, 0.04, 0.06, 1.0],
    );
    for py in (0..map.height).step_by(step as usize) {
        for px in (0..map.width).step_by(step as usize) {
            let index = py as usize * map.width as usize + px as usize;
            let Some(terrain) = map.terrain.get(index).copied() else {
                continue;
            };
            let ore = map.ore.get(index).copied().unwrap_or(0) > 0;
            let base = terrain.color();
            let fallback = if ore { [0.62, 0.55, 0.18, 1.0] } else { base };
            // Water against land gets shore edge tiles, Beach transitions to
            // sand_shore, the rest keep the base terrain family.
            let (category, family, fallback_row, col_override) = ra_terrain_tile(map, px, py);
            let tile_s = scale * step as f32;
            let drew_tile = draw_pack_tile_family(
                renderer,
                tileset,
                category,
                family,
                fallback_row,
                col_override,
                px,
                py,
                x + px as f32 * scale,
                y + py as f32 * scale,
                tile_s,
                tile_s,
                fallback,
            );
            // Ore chunks sprite on top of the terrain tile; without tileset
            // art the flat fallback colour already carries the ore marker
            // exactly as the pre-tileset renderer did.
            if ore && drew_tile {
                let fx_drawn = draw_pack_fx(
                    renderer,
                    tileset,
                    "ore_chunks",
                    3,
                    px,
                    py,
                    x + px as f32 * scale,
                    y + py as f32 * scale,
                    tile_s,
                    tile_s,
                );
                if !fx_drawn {
                    renderer.draw_rect(
                        x + px as f32 * scale,
                        y + py as f32 * scale,
                        tile_s,
                        tile_s,
                        [0.62, 0.55, 0.18, 0.7],
                    );
                }
            }
        }
    }
    renderer.draw_text("MAP PREVIEW", x, y - 14.0, 9.0, [0.60, 0.75, 0.95, 1.0]);
}

fn draw_ra_setup(
    renderer: &mut dyn Renderer,
    setup: &SkirmishSetup,
    tileset: Option<&TilesetPack>,
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.04, 0.05, 0.08, 1.0]);
    renderer.draw_rect(0.0, 112.0, w, 2.0, [0.25, 0.45, 0.75, 1.0]);
    renderer.draw_text("RED ALERT", 60.0, 42.0, 30.0, [0.95, 0.55, 0.35, 1.0]);
    renderer.draw_text("Skirmish Setup", 60.0, 84.0, 15.0, [0.85, 0.90, 1.0, 1.0]);
    let rows = [
        format!(
            "Map size: {} x {}          (+ / -)",
            setup.map_size, setup.map_size
        ),
        format!(
            "Side: {}          (S)",
            if setup.side == 0 { "Allies" } else { "Soviet" }
        ),
        format!("Your country: {}          (Tab)", setup.country.label()),
        format!(
            "AI difficulty: {}          (D)",
            DIFFICULTY_NAMES[setup.difficulty]
        ),
        String::new(),
        "N: network skirmish (host / join)".to_string(),
        "Enter: start skirmish".to_string(),
        "Esc: quit".to_string(),
    ];
    let mut y = 160.0;
    for (index, row) in rows.iter().enumerate() {
        let color = if index >= 5 {
            [0.70, 0.80, 0.92, 1.0]
        } else {
            [0.90, 0.93, 0.98, 1.0]
        };
        renderer.draw_text(row, 60.0, y, 12.0, color);
        y += 28.0;
    }
    let preview = ra_setup_preview(setup.map_size);
    draw_ra_setup_preview(
        renderer,
        &preview,
        tileset,
        (w - 320.0).max(60.0),
        150.0,
        280.0,
    );
    renderer.draw_text(
        "RA skirmish: Allied vs Soviet, AI builds and attacks.",
        60.0,
        h - 60.0,
        9.0,
        [0.55, 0.65, 0.78, 1.0],
    );
}

/// Format a ready-up roster for the network lobby, preserving
/// `ReadyTracker::roster`'s name-sorted order: one
/// `<player>: READY|waiting` row per peer.
fn net_lobby_roster_rows(roster: &[(String, bool)]) -> Vec<String> {
    roster
        .iter()
        .map(|(name, ready)| format!("{}: {}", name, if *ready { "READY" } else { "waiting" }))
        .collect()
}

/// Host/join lobby screen shared by the TD and RA network modes.
fn draw_net_lobby(
    renderer: &mut dyn Renderer,
    title: &str,
    lobby: &NetLobby,
    scan: Option<&LanScan>,
    directory: &LobbyScreen,
    error: Option<&str>,
    roster: &[(String, bool)],
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.04, 0.05, 0.08, 1.0]);
    renderer.draw_rect(0.0, 112.0, w, 2.0, [0.25, 0.45, 0.75, 1.0]);
    renderer.draw_text(title, 60.0, 42.0, 30.0, [0.95, 0.55, 0.35, 1.0]);
    renderer.draw_text(
        "Network Lobby - Host / Join",
        60.0,
        84.0,
        15.0,
        [0.85, 0.90, 1.0, 1.0],
    );

    let (address_label, address) = match lobby.role {
        NetRole::Host => ("Peer address", lobby.peer_address.as_str()),
        NetRole::Join => ("Host address", lobby.join_address.as_str()),
    };
    let port = match lobby.role {
        NetRole::Host => lobby.host_port,
        NetRole::Join => lobby.join_port,
    };
    let editing = if lobby.focus == 0 {
        "    (editing - Tab to leave)"
    } else {
        ""
    };
    let mut rows: Vec<String> = vec![
        format!("Role: {}    (R)", lobby.role_label()),
        format!("Local port: {}    ([ / ])", port),
        format!("{address_label}: {address}_ {editing}"),
        String::new(),
        "Tab: switch field   Backspace: delete".to_string(),
        "L: scan LAN   Up/Down: pick host".to_string(),
        "Enter: start network game".to_string(),
    ];
    rows.extend(net_lobby_roster_rows(roster));
    rows.push("Esc: stop scan / back to setup".to_string());
    let mut y = 160.0;
    for (index, row) in rows.iter().enumerate() {
        let color = if index >= 4 {
            [0.70, 0.80, 0.92, 1.0]
        } else {
            [0.90, 0.93, 0.98, 1.0]
        };
        renderer.draw_text(row, 60.0, y, 12.0, color);
        y += 28.0;
    }
    let mut lan_y = y + 6.0;
    renderer.draw_text("LAN HOSTS", 60.0, lan_y, 10.0, [0.60, 0.75, 0.95, 1.0]);
    lan_y += 14.0;
    match scan {
        None => {
            renderer.draw_text(
                "Press L to listen for host beacons.",
                60.0,
                lan_y,
                9.0,
                [0.55, 0.65, 0.78, 1.0],
            );
        }
        Some(scan) if scan.hosts.is_empty() => {
            renderer.draw_text(
                "Scanning UDP 47890... waiting for a host.",
                60.0,
                lan_y,
                9.0,
                [0.80, 0.75, 0.50, 1.0],
            );
        }
        Some(scan) => {
            for (index, host) in scan.hosts.iter().take(5).enumerate() {
                let selected = index == scan.selected;
                let color = if selected {
                    [0.55, 0.95, 0.65, 1.0]
                } else {
                    [0.82, 0.88, 0.96, 1.0]
                };
                renderer.draw_text(
                    &format!("{}{}", if selected { "> " } else { "  " }, host.label),
                    60.0,
                    lan_y,
                    9.0,
                    color,
                );
                lan_y += 12.0;
            }
        }
    }
    lan_y += 6.0;
    renderer.draw_text(
        &format!(
            "DIRECTORY ({})  D: switch backend  [{} row(s)]",
            directory.mode.label(),
            directory.row_count()
        ),
        60.0,
        lan_y,
        10.0,
        [0.60, 0.75, 0.95, 1.0],
    );
    lan_y += 14.0;
    for row in directory.scan.render_rows().iter().take(3) {
        renderer.draw_text(row, 60.0, lan_y, 9.0, [0.82, 0.88, 0.96, 1.0]);
        lan_y += 12.0;
    }
    renderer.draw_text(
        &directory.scan.status_line(),
        60.0,
        lan_y,
        9.0,
        [0.75, 0.78, 0.62, 1.0],
    );
    renderer.draw_text(
        match lobby.role {
            NetRole::Host => {
                "Host: peer address is the JOIN player's address (e.g. 127.0.0.1:47871)."
            }
            NetRole::Join => {
                "Join: host address is the HOST player's address (e.g. 127.0.0.1:47870)."
            }
        },
        60.0,
        h - 84.0,
        9.0,
        [0.55, 0.65, 0.78, 1.0],
    );
    if let Some(error) = error {
        renderer.draw_text(
            &format!("Network error: {error}"),
            60.0,
            h - 60.0,
            9.0,
            [1.0, 0.50, 0.45, 1.0],
        );
    }
}

fn draw_mission_emblem(
    renderer: &mut dyn Renderer,
    x: f32,
    y: f32,
    size: f32,
    player: usize,
    index: usize,
) {
    let color = if player == 0 {
        [0.24, 0.44, 0.85, 1.0]
    } else {
        [0.78, 0.20, 0.18, 1.0]
    };
    let cell = size / 7.0;
    let rows = [(3, 1), (2, 3), (1, 5), (1, 5), (2, 3), (3, 1)];
    for (row, (offset, width)) in rows.iter().enumerate() {
        renderer.draw_rect(
            x + *offset as f32 * cell,
            y + row as f32 * cell,
            *width as f32 * cell,
            cell,
            color,
        );
    }
    renderer.draw_rect(x + 0.0, y + 6.0 * cell, size, 1.5, [0.85, 0.75, 0.35, 1.0]);
    renderer.draw_text(
        &format!("{:02}", index + 1),
        x + size * 0.34,
        y + size * 0.36,
        size * 0.24,
        [0.98, 0.97, 0.92, 1.0],
    );
}

fn draw_ra_campaign(
    renderer: &mut dyn Renderer,
    campaign: &[CampaignEntry],
    tileset: Option<&TilesetPack>,
    selected: usize,
    completed: &[bool],
    best_times: &[Option<i32>],
    replay_prompt: bool,
    difficulty: usize,
    filter: CampaignFilter,
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.04, 0.05, 0.08, 1.0]);
    renderer.draw_rect(0.0, 112.0, w, 2.0, [0.25, 0.45, 0.75, 1.0]);
    renderer.draw_text("RED ALERT", 60.0, 42.0, 30.0, [0.95, 0.55, 0.35, 1.0]);
    renderer.draw_text(
        "Campaign - Select Mission",
        60.0,
        84.0,
        15.0,
        [0.85, 0.90, 1.0, 1.0],
    );
    if campaign.is_empty() {
        renderer.draw_text(
            "No missions found. Pass --campaign DIR.",
            60.0,
            160.0,
            12.0,
            [0.95, 0.70, 0.45, 1.0],
        );
    } else {
        let mut y = 150.0;
        let visible: Vec<usize> = (0..campaign.len())
            .filter(|index| match filter {
                CampaignFilter::All => true,
                CampaignFilter::Incomplete => !completed.get(*index).copied().unwrap_or(false),
                CampaignFilter::Completed => completed.get(*index).copied().unwrap_or(false),
            })
            .collect();
        if visible.is_empty() {
            renderer.draw_text(
                "No missions match this filter.",
                60.0,
                y,
                12.0,
                [0.90, 0.75, 0.55, 1.0],
            );
            y += 24.0;
        }
        for index in visible {
            let entry = &campaign[index];
            let done = completed.get(index).copied().unwrap_or(false);
            let color = if index == selected {
                [1.0, 0.90, 0.45, 1.0]
            } else if done {
                [0.60, 0.85, 0.65, 1.0]
            } else {
                [0.82, 0.87, 0.94, 1.0]
            };
            let prefix = if index == selected { "> " } else { "  " };
            let best = best_times
                .get(index)
                .and_then(|seconds| *seconds)
                .map_or_else(|| "--".to_string(), |seconds| format!("{seconds}s"));
            renderer.draw_text(
                &format!(
                    "{prefix}{} {}. {}    best {}",
                    if done { "[x]" } else { "[ ]" },
                    index + 1,
                    entry.name,
                    best
                ),
                60.0,
                y,
                12.0,
                color,
            );
            y += 24.0;
        }
        renderer.draw_text(
            &format!(
                "Difficulty: {} (D)    Filter: {} (F)",
                DIFFICULTY_NAMES[difficulty],
                filter.label()
            ),
            60.0,
            y + 4.0,
            10.0,
            [0.75, 0.82, 0.92, 1.0],
        );
        y += 24.0;
        if let Some(entry) = campaign.get(selected) {
            draw_mission_emblem(renderer, 420.0, 140.0, 96.0, entry.player, selected);
            y += 10.0;
            renderer.draw_text(
                &format!(
                    "Side: {}",
                    if entry.player == 0 {
                        "Allies"
                    } else {
                        "Soviet"
                    }
                ),
                60.0,
                y,
                10.0,
                [0.75, 0.82, 0.92, 1.0],
            );
            y += 18.0;
            renderer.draw_text(
                &format!(
                    "Map: {}x{}    Objectives: {}    Triggers: {}",
                    entry.width,
                    entry.height,
                    entry.objectives.len(),
                    entry.triggers
                ),
                60.0,
                y,
                10.0,
                [0.72, 0.80, 0.90, 1.0],
            );
            y += 18.0;
            if !entry.briefing.is_empty() {
                renderer.draw_text(
                    &format!("Briefing: {}", entry.briefing),
                    60.0,
                    y,
                    10.0,
                    [0.85, 0.82, 0.65, 1.0],
                );
                y += 16.0;
            }
            for objective in entry.objectives.iter().take(3) {
                renderer.draw_text(
                    &format!("- {objective}"),
                    60.0,
                    y,
                    9.0,
                    [0.72, 0.82, 0.92, 1.0],
                );
                y += 14.0;
            }

            if !entry.terrain.is_empty() && entry.width > 0 && entry.height > 0 {
                let preview_w = 240.0f32;
                let preview_h =
                    (preview_w * entry.height as f32 / entry.width as f32).clamp(90.0, 180.0);
                let preview_x = 60.0;
                let preview_y = y + 12.0;
                let cell_w = preview_w / entry.width as f32;
                let cell_h = preview_h / entry.height as f32;
                let step = ((entry.width.max(entry.height) as f32 / 80.0).ceil() as u32).max(1);
                let block_w = (cell_w * step as f32).max(1.0);
                let block_h = (cell_h * step as f32).max(1.0);
                let mut py = 0u32;
                while py < entry.height {
                    let mut px = 0u32;
                    while px < entry.width {
                        let index = py as usize * entry.width as usize + px as usize;
                        let terrain = entry.terrain.get(index).copied().unwrap_or(0);
                        let base = match terrain {
                            1 => [0.10, 0.24, 0.48, 1.0],
                            2 => [0.56, 0.50, 0.32, 1.0],
                            4 => [0.38, 0.38, 0.40, 1.0],
                            _ => [0.22, 0.36, 0.20, 1.0],
                        };
                        let ore = entry.ore.get(index).copied().unwrap_or(0) > 0;
                        let fallback = if ore { [0.85, 0.72, 0.22, 1.0] } else { base };
                        // Water against land gets shore edge tiles computed
                        // over the campaign grid, open water stays deep_water.
                        let (category, family, fallback_row, col_override) = campaign_terrain_tile(
                            &entry.terrain,
                            entry.width,
                            entry.height,
                            px,
                            py,
                        );
                        let drew_tile = draw_pack_tile_family(
                            renderer,
                            tileset,
                            category,
                            family,
                            fallback_row,
                            col_override,
                            px,
                            py,
                            preview_x + px as f32 * cell_w,
                            preview_y + py as f32 * cell_h,
                            block_w,
                            block_h,
                            fallback,
                        );
                        // Ore marker on top of the terrain tile; without
                        // tileset art the legacy flat inset marker stays
                        // exactly as before.
                        if ore && drew_tile {
                            let (fx_x, fx_y) = (
                                preview_x + px as f32 * cell_w + block_w * 0.25,
                                preview_y + py as f32 * cell_h + block_h * 0.25,
                            );
                            let (fx_w, fx_h) = ((block_w * 0.5).max(1.0), (block_h * 0.5).max(1.0));
                            let fx_drawn = draw_pack_fx(
                                renderer,
                                tileset,
                                "ore_chunks",
                                3,
                                px,
                                py,
                                fx_x,
                                fx_y,
                                fx_w,
                                fx_h,
                            );
                            if !fx_drawn {
                                renderer.draw_rect(fx_x, fx_y, fx_w, fx_h, [0.85, 0.72, 0.22, 0.8]);
                            }
                        }
                        px += step;
                    }
                    py += step;
                }
                let border = [0.35, 0.50, 0.65, 1.0];
                renderer.draw_rect(preview_x, preview_y, preview_w, 1.0, border);
                renderer.draw_rect(
                    preview_x,
                    preview_y + preview_h - 1.0,
                    preview_w,
                    1.0,
                    border,
                );
                renderer.draw_rect(preview_x, preview_y, 1.0, preview_h, border);
                renderer.draw_rect(
                    preview_x + preview_w - 1.0,
                    preview_y,
                    1.0,
                    preview_h,
                    border,
                );
            }
        }
    }
    renderer.draw_text(
        "Up/Down: select   Enter: deploy   D: difficulty   F: filter   S: stats   Esc: quit",
        60.0,
        h - 60.0,
        10.0,
        [0.65, 0.72, 0.82, 1.0],
    );

    if replay_prompt {
        let box_w = 400.0f32;
        let box_h = 90.0f32;
        let box_x = (w - box_w) * 0.5;
        let box_y = (h - box_h) * 0.5;
        renderer.draw_rect(box_x, box_y, box_w, box_h, [0.07, 0.09, 0.13, 1.0]);
        renderer.draw_rect(box_x, box_y, box_w, 2.0, [0.85, 0.70, 0.30, 1.0]);
        renderer.draw_text(
            "Mission already cleared. Replay?",
            box_x + 16.0,
            box_y + 18.0,
            11.0,
            [0.95, 0.90, 0.70, 1.0],
        );
        renderer.draw_text(
            "Enter: replay    Esc: cancel",
            box_x + 16.0,
            box_y + 52.0,
            10.0,
            [0.75, 0.82, 0.92, 1.0],
        );
    }
}

fn draw_ra_campaign_stats(
    renderer: &mut dyn Renderer,
    campaign: &[CampaignEntry],
    completed: &[bool],
    best_times: &[Option<i32>],
    difficulty: usize,
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.04, 0.05, 0.08, 1.0]);
    renderer.draw_rect(0.0, 112.0, w, 2.0, [0.25, 0.45, 0.75, 1.0]);
    renderer.draw_text("CAMPAIGN STATS", 60.0, 42.0, 26.0, [0.95, 0.55, 0.35, 1.0]);
    let cleared = completed.iter().filter(|done| **done).count();
    renderer.draw_text(
        &format!(
            "{cleared}/{} missions cleared    Difficulty: {}",
            campaign.len(),
            DIFFICULTY_NAMES[difficulty]
        ),
        60.0,
        92.0,
        13.0,
        [0.90, 0.93, 0.98, 1.0],
    );

    let mut y = 136.0;
    let mut total = 0i32;
    for (index, entry) in campaign.iter().enumerate() {
        let done = completed.get(index).copied().unwrap_or(false);
        let best = best_times.get(index).and_then(|seconds| *seconds);
        if let Some(seconds) = best {
            total += seconds;
        }
        let color = if done {
            [0.60, 0.85, 0.65, 1.0]
        } else {
            [0.82, 0.87, 0.94, 1.0]
        };
        renderer.draw_text(
            &format!(
                "{} {}. {}    {}",
                if done { "[x]" } else { "[ ]" },
                index + 1,
                entry.name,
                best.map_or_else(|| "--".to_string(), |seconds| format!("{seconds}s"))
            ),
            60.0,
            y,
            11.0,
            color,
        );
        y += 22.0;
    }
    renderer.draw_text(
        &format!("Total best time: {total}s"),
        60.0,
        y + 6.0,
        11.0,
        [0.85, 0.82, 0.65, 1.0],
    );
    renderer.draw_text(
        "Enter/Esc: back to mission select",
        60.0,
        h - 60.0,
        10.0,
        [0.70, 0.80, 0.92, 1.0],
    );
}

fn draw_ra_campaign_complete(
    renderer: &mut dyn Renderer,
    missions: usize,
    elapsed_seconds: i32,
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.04, 0.05, 0.08, 1.0]);
    renderer.draw_rect(0.0, 112.0, w, 2.0, [0.25, 0.45, 0.75, 1.0]);
    renderer.draw_text(
        "CAMPAIGN COMPLETE",
        60.0,
        42.0,
        28.0,
        [0.55, 1.0, 0.60, 1.0],
    );
    renderer.draw_text(
        &format!("All {missions} missions cleared"),
        60.0,
        92.0,
        14.0,
        [0.90, 0.93, 0.98, 1.0],
    );
    renderer.draw_text(
        &format!("Final mission time: {elapsed_seconds}s"),
        60.0,
        126.0,
        11.0,
        [0.72, 0.80, 0.90, 1.0],
    );
    renderer.draw_text(
        "Enter: mission select    Esc: quit",
        60.0,
        h - 60.0,
        11.0,
        [0.70, 0.80, 0.92, 1.0],
    );
}

/// Minimap step and per-cell scale for a map of `map_w x map_h` cells.
fn minimap_layout(map_w: u32, map_h: u32, size: f32) -> (u32, f32) {
    let max_dim = map_w.max(map_h).max(1);
    let step = ((max_dim as f32 / 48.0).ceil() as u32).max(1);
    (step, size / max_dim as f32)
}

/// Top-left corner and size of a minimap panel inside the visible world area.
fn minimap_origin(
    map_w: u32,
    map_h: u32,
    left: f32,
    top: f32,
    view_w: f32,
    view_h: f32,
    bottom_right: bool,
) -> (f32, f32, f32, f32) {
    let (_, scale) = minimap_layout(map_w, map_h, 150.0);
    let panel_w = map_w.max(1) as f32 * scale;
    let panel_h = map_h.max(1) as f32 * scale;
    let x = if bottom_right {
        left + view_w - panel_w - 18.0
    } else {
        left + 18.0
    };
    let y = top + view_h - panel_h - 18.0;
    (x, y, panel_w, panel_h)
}

/// Map cell under a world position on a minimap, if the click hit the panel.
fn minimap_cell_at(
    world: Vec2,
    map_w: u32,
    map_h: u32,
    left: f32,
    top: f32,
    view_w: f32,
    view_h: f32,
    bottom_right: bool,
) -> Option<Vec2i> {
    let (x, y, panel_w, panel_h) =
        minimap_origin(map_w, map_h, left, top, view_w, view_h, bottom_right);
    let rel_x = world.x.to_f32() - x;
    let rel_y = world.y.to_f32() - y;
    if rel_x < 0.0 || rel_y < 0.0 || rel_x >= panel_w || rel_y >= panel_h {
        return None;
    }
    let (_, scale) = minimap_layout(map_w, map_h, 150.0);
    Some(Vec2i::new(
        (rel_x / scale).floor() as i32,
        (rel_y / scale).floor() as i32,
    ))
}

/// Bottom-right terrain/unit minimap for the RA game view.
fn draw_ra_minimap(
    renderer: &mut dyn Renderer,
    game: &RaGame,
    tileset: Option<&TilesetPack>,
    player: usize,
    center: Vec2,
    view_w: f32,
    view_h: f32,
    left: f32,
    top: f32,
    hud_w: f32,
    hud_h: f32,
) {
    let map_w = game.map.width.max(1);
    let map_h = game.map.height.max(1);
    let (step, scale) = minimap_layout(map_w, map_h, 150.0);
    let (x, y, panel_w, panel_h) = minimap_origin(map_w, map_h, left, top, hud_w, hud_h, true);
    renderer.draw_rect(
        x - 4.0,
        y - 4.0,
        panel_w + 8.0,
        panel_h + 8.0,
        [0.02, 0.03, 0.05, 1.0],
    );
    for py in (0..map_h).step_by(step as usize) {
        for px in (0..map_w).step_by(step as usize) {
            let index = py as usize * map_w as usize + px as usize;
            let terrain = game
                .map
                .terrain
                .get(index)
                .copied()
                .unwrap_or(cnc_red_alert::Terrain::Grass);
            let ore = game.map.ore.get(index).copied().unwrap_or(0) > 0;
            let base = terrain.color();
            let fallback = if ore { [0.62, 0.55, 0.18, 1.0] } else { base };
            // Water against land gets shore edge tiles, Beach transitions to
            // sand_shore, the rest keep the base terrain family.
            let (category, family, fallback_row, col_override) = ra_terrain_tile(&game.map, px, py);
            let tile_s = scale * step as f32;
            let drew_tile = draw_pack_tile_family(
                renderer,
                tileset,
                category,
                family,
                fallback_row,
                col_override,
                px,
                py,
                x + px as f32 * scale,
                y + py as f32 * scale,
                tile_s,
                tile_s,
                fallback,
            );
            // Ore chunks sprite on top of terrain art; without tileset art
            // the legacy flat marker colours stay untouched (solid marker in
            // the fallback, marker rect preserved whenever fx drawing fails).
            if ore && drew_tile {
                let fx_drawn = draw_pack_fx(
                    renderer,
                    tileset,
                    "ore_chunks",
                    3,
                    px,
                    py,
                    x + px as f32 * scale,
                    y + py as f32 * scale,
                    tile_s,
                    tile_s,
                );
                if !fx_drawn {
                    renderer.draw_rect(
                        x + px as f32 * scale,
                        y + py as f32 * scale,
                        tile_s,
                        tile_s,
                        [0.62, 0.55, 0.18, 0.7],
                    );
                }
            }
        }
    }
    for structure in &game.structures {
        if !structure.is_alive() {
            continue;
        }
        let color = if structure.owner == player {
            [0.35, 0.85, 1.0, 1.0]
        } else {
            [1.0, 0.35, 0.30, 1.0]
        };
        renderer.draw_rect(
            x + structure.cell.x as f32 * scale,
            y + structure.cell.y as f32 * scale,
            (scale * 2.0).max(2.0),
            (scale * 2.0).max(2.0),
            color,
        );
    }
    for unit in &game.units {
        if !unit.is_alive() {
            continue;
        }
        let color = if unit.owner == player {
            [0.65, 1.0, 0.75, 1.0]
        } else {
            [1.0, 0.55, 0.45, 1.0]
        };
        renderer.draw_rect(
            x + unit.cell.x as f32 * scale,
            y + unit.cell.y as f32 * scale,
            scale.max(1.5),
            scale.max(1.5),
            color,
        );
    }
    let view_left = (center.x.to_f32() - view_w * 0.5) / RA_TILE_SIZE;
    let view_top = (center.y.to_f32() - view_h * 0.5) / RA_TILE_SIZE;
    let view_cells_w = view_w / RA_TILE_SIZE;
    let view_cells_h = view_h / RA_TILE_SIZE;
    let border = [0.95, 0.90, 0.75, 1.0];
    renderer.draw_rect(
        x + view_left * scale,
        y + view_top * scale,
        view_cells_w * scale,
        1.0,
        border,
    );
    renderer.draw_rect(
        x + view_left * scale,
        y + (view_top + view_cells_h) * scale - 1.0,
        view_cells_w * scale,
        1.0,
        border,
    );
    renderer.draw_rect(
        x + view_left * scale,
        y + view_top * scale,
        1.0,
        view_cells_h * scale,
        border,
    );
    renderer.draw_rect(
        x + (view_left + view_cells_w) * scale - 1.0,
        y + view_top * scale,
        1.0,
        view_cells_h * scale,
        border,
    );
    renderer.draw_text("MINIMAP", x, y - 13.0, 8.0, [0.60, 0.75, 0.95, 1.0]);
}

fn draw_ra_pause(
    renderer: &mut dyn Renderer,
    game: &RaGame,
    choice: usize,
    save_slot: usize,
    slot_label: &str,
    width: u32,
    height: u32,
) {
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.02, 0.03, 0.05, 1.0]);
    let panel_w = 420.0f32;
    let panel_h = 240.0f32;
    let panel_x = (w - panel_w) * 0.5;
    let panel_y = (h - panel_h) * 0.5;
    renderer.draw_rect(panel_x, panel_y, panel_w, panel_h, [0.07, 0.09, 0.13, 1.0]);
    renderer.draw_rect(panel_x, panel_y, panel_w, 2.0, [0.35, 0.55, 0.85, 1.0]);
    renderer.draw_text(
        "PAUSED",
        panel_x + 24.0,
        panel_y + 20.0,
        22.0,
        [0.95, 0.85, 0.60, 1.0],
    );
    renderer.draw_text(
        &format!(
            "t={}s   {} units   {} structures   {} credits",
            game.elapsed.int(),
            game.units.len(),
            game.structures.len(),
            game.houses
                .get(game.player_owner)
                .map_or(0, |house| house.credits)
        ),
        panel_x + 24.0,
        panel_y + 58.0,
        10.0,
        [0.78, 0.86, 0.95, 1.0],
    );
    let options = [
        "Resume".to_string(),
        "Restart mission".to_string(),
        "Abort to menu".to_string(),
        "Save game".to_string(),
        "Load game".to_string(),
        format!("Save slot: {}/3 ({slot_label})", save_slot + 1),
    ];
    let mut y = panel_y + 96.0;
    for (index, label) in options.iter().enumerate() {
        let color = if index == choice {
            [1.0, 0.90, 0.45, 1.0]
        } else {
            [0.78, 0.86, 0.95, 1.0]
        };
        renderer.draw_text(
            &format!("{}{}", if index == choice { "> " } else { "  " }, label),
            panel_x + 24.0,
            y,
            13.0,
            color,
        );
        y += 26.0;
    }
    renderer.draw_text(
        "Up/Down: choose   Enter: select   Esc: resume",
        panel_x + 24.0,
        panel_y + panel_h - 28.0,
        9.0,
        [0.65, 0.72, 0.82, 1.0],
    );
}

fn draw_ra_briefing(
    renderer: &mut dyn Renderer,
    mission: &str,
    game: &RaGame,
    index: usize,
    tileset: Option<&TilesetPack>,
    width: u32,
    height: u32,
) -> bool {
    // Video first (plan box 212); mirrors the TD briefing. Borrows the shared
    // tileset slot only when the RA tileset pack is resident so it can be
    // restored afterwards; any miss falls through to the text panel below.
    // `RaGame` carries no per-mission briefing-video key, so this resolves a
    // deterministic index-keyed asset that no bundle ships yet (text fallback).
    if tileset.is_some()
        && draw_briefing_video(renderer, Some(&ra_briefing_video_key(index)), width, height)
    {
        renderer.draw_text(mission, 40.0, 22.0, 16.0, [0.95, 0.93, 0.88, 1.0]);
        renderer.draw_text(
            "Enter: begin mission    Esc: quit",
            40.0,
            height as f32 - 34.0,
            12.0,
            [0.78, 0.85, 0.95, 1.0],
        );
        return true;
    }
    let w = width as f32;
    let h = height as f32;
    renderer.draw_rect(0.0, 0.0, w, h, [0.04, 0.05, 0.08, 1.0]);
    renderer.draw_rect(0.0, 112.0, w, 2.0, [0.25, 0.45, 0.75, 1.0]);
    renderer.draw_text(
        "MISSION BRIEFING",
        60.0,
        42.0,
        26.0,
        [0.95, 0.55, 0.35, 1.0],
    );
    renderer.draw_text(mission, 60.0, 82.0, 15.0, [0.90, 0.93, 0.98, 1.0]);
    draw_mission_emblem(renderer, w - 170.0, 34.0, 110.0, game.player_owner, index);
    renderer.draw_text(
        &format!(
            "Side: {}",
            if game.player_owner == 0 {
                "Allies"
            } else {
                "Soviet"
            }
        ),
        60.0,
        112.0,
        11.0,
        [0.70, 0.80, 0.92, 1.0],
    );

    let mut y = 154.0;
    if !game.briefing.is_empty() {
        renderer.draw_text("BRIEFING", 60.0, y, 11.0, [0.60, 0.75, 0.95, 1.0]);
        y += 18.0;
        renderer.draw_text(&game.briefing, 60.0, y, 11.0, [0.88, 0.90, 0.95, 1.0]);
        y += 28.0;
    }
    if !game.objectives.is_empty() {
        renderer.draw_text("OBJECTIVES", 60.0, y, 11.0, [0.60, 0.75, 0.95, 1.0]);
        y += 18.0;
        for objective in &game.objectives {
            renderer.draw_text(
                &format!("- {}", objective.text),
                60.0,
                y,
                11.0,
                [0.82, 0.88, 0.96, 1.0],
            );
            y += 16.0;
        }
    }

    renderer.draw_text(
        "Enter: begin mission    Esc: quit",
        60.0,
        h - 60.0,
        11.0,
        [0.70, 0.80, 0.92, 1.0],
    );
    false
}

pub struct LauncherApp {
    kind: RendererKind,
    width: u32,
    height: u32,
    window: Option<Arc<Window>>,
    renderer: Option<Box<dyn Renderer>>,
    pub camera: Camera,
    pub game: Game,
    sprite_atlas: Option<SpriteAtlas>,
    /// RTS 16×16 tileset packs, one per palette: `ra_inspired` backs the RA
    /// screens, `td_inspired` the TD screens. `load_default` honors
    /// `CNC_TILESET_DIR` / `CNC_TILESET_PALETTE`; None falls back to the flat
    /// terrain colours in the draw paths.
    ra_tileset: Option<TilesetPack>,
    td_tileset: Option<TilesetPack>,
    /// Palette currently resident in the renderer's tileset texture slot
    /// (1 = RA, 2 = TD): None until the first successful upload. Keeps the
    /// upload-once-per-screen-context behaviour instead of per-frame uploads.
    tileset_slot: Option<u8>,
    // Kept alive for the upcoming asset/audio integration pass.
    #[allow(dead_code)]
    assets: AssetRegistry,
    #[allow(dead_code)]
    mixer: AudioMixer,
    /// In-game SFX director (todo 42): routes engine events to the mixer.
    audio_director: audio_director::AudioDirector,
    /// Theme queue (todo 42): explore/combat layers driven by house state.
    theme_queue: audio_director::ThemeQueue,
    /// Engine event consumer feeding the director; created at startup so no
    /// pre-subscription events are missed.
    event_subscriber: events_subscriber::EventSubscriber,
    /// Persisted launcher options (plan line 206); the options-modal UI
    /// lane mutates this in place.
    #[allow(dead_code)] // Read by the Phase 9.1 options modal.
    pub options: LauncherOptions,
    cursor: Option<winit::dpi::PhysicalPosition<f64>>,
    drag_start: Option<winit::dpi::PhysicalPosition<f64>>,
    drag_current: Option<winit::dpi::PhysicalPosition<f64>>,
    shift_down: bool,
    ctrl_down: bool,
    alt_down: bool,
    bookmarks: [Option<Vec2>; 4],
    control_groups: [Vec<usize>; 4],
    selected_building: Option<usize>,
    save_slot: usize,
    game_speed: f32,
    frame_index: u64,
    last_produced: u64,
    last_harvested: i64,
    last_aircraft_shots: u64,
    last_aircraft_rearmed: u64,
    last_secondary_shots: u64,
    last_frame: Instant,
    next_frame: Instant,
    software_warning_emitted: bool,
    mission_dir: Option<PathBuf>,
    mission_choices: Vec<TdChoice>,
    mission_index: usize,
    selecting_mission: bool,
    td_completed: Vec<bool>,
    td_best: Vec<Option<i32>>,
    td_progress_path: PathBuf,
    td_recorded: bool,
    td_briefing_open: bool,
    td_filter: CampaignFilter,
    td_stats_open: bool,
    td_complete: bool,
    td_paused: bool,
    td_pause_choice: usize,
    td_score: Option<score::ScoreReport>,
    td_score_since: Option<Instant>,
    td_carryover: Option<Carryover>,
    td_notice: Option<(String, Instant)>,
    /// True while the TD host/join lobby owns the screen.
    td_lobby_open: bool,
    td_lobby: NetLobby,
    td_lobby_error: Option<String>,
    td_lan_scan: Option<LanScan>,
    /// Directory-backed lobby rows for the TD lobby (see `LobbyScreen`).
    td_lobby_screen: LobbyScreen,
    /// Live lockstep session while a TD network match runs.
    network: Option<TdNetwork>,
    /// Host-side ready roster; ready flags ride out-of-band, never the
    /// lockstep stream (see the `ready` module).
    ready_tracker: ReadyTracker,
    /// Set when a menu-owned screen wants the shell to reopen the main menu.
    return_to_menu: bool,
}

impl LauncherApp {
    pub fn new(
        kind: RendererKind,
        width: u32,
        height: u32,
        rules: RulesDatabase,
        options: LauncherOptions,
    ) -> Result<Self> {
        let mut game = Game::with_rules("GDI Mission 1", MAP_SIZE, MAP_SIZE, rules);
        game.setup_demo();

        // Attach the event bus before the game moves into the struct so the
        // subscriber sees events from the very first tick (todo 42).
        let event_bus = std::sync::Arc::new(cnc_events::EventBus::new(4096));
        game.set_event_bus(event_bus.clone());

        let mut camera = Camera::default();
        camera.set_viewport(width.max(1), height.max(1));
        camera.position = game.world_center();
        camera.zoom = Fixed16_16::from_f32(DEFAULT_GAME_ZOOM);

        // Extract game_speed before options is moved into mixer
        let game_speed = options.game_speed.clamp(0.25, 3.0);

        Ok(Self {
            kind,
            width,
            height,
            window: None,
            renderer: None,
            camera,
            game,
            sprite_atlas: None,
            ra_tileset: cnc_asset::tileset_pack::load_default("ra_inspired"),
            td_tileset: cnc_asset::tileset_pack::load_default("td_inspired"),
            tileset_slot: None,
            assets: AssetRegistry::new(),
            mixer: {
                // Apply the persisted volume buses where the mixer is built
                // (plan line 206; cnc-audio music/effects/master knobs).
                let mixer = AudioMixer::new()?;
                options.apply_to_mixer(&mixer);
                mixer
            },
            audio_director: audio_director::AudioDirector::new(),
            theme_queue: audio_director::ThemeQueue::new(),
            event_subscriber: events_subscriber::EventSubscriber::new(event_bus),
            options,
            cursor: None,
            drag_start: None,
            drag_current: None,
            shift_down: false,
            ctrl_down: false,
            alt_down: false,
            bookmarks: [None; 4],
            control_groups: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            selected_building: None,
            save_slot: 0,
            // In-game options applied from persistence (plan todo 43):
            // game speed drives the fixed-step multiplier and the special
            // toggles are pushed onto every house AI once the sim is running.
            game_speed,
            frame_index: 0,
            last_produced: 0,
            last_harvested: 0,
            last_aircraft_shots: 0,
            last_aircraft_rearmed: 0,
            last_secondary_shots: 0,
            last_frame: Instant::now(),
            next_frame: Instant::now() + FRAME_INTERVAL,
            software_warning_emitted: false,
            mission_dir: None,
            mission_choices: Vec::new(),
            mission_index: 0,
            selecting_mission: false,
            td_completed: Vec::new(),
            td_best: Vec::new(),
            td_progress_path: default_td_progress_path(),
            td_recorded: false,
            td_briefing_open: false,
            td_filter: CampaignFilter::All,
            td_stats_open: false,
            td_complete: false,
            td_paused: false,
            td_pause_choice: 0,
            td_score: None,
            td_score_since: None,
            td_carryover: None,
            td_notice: None,
            td_lobby_open: false,
            td_lobby: NetLobby::td_default(),
            td_lobby_error: None,
            td_lan_scan: None,
            td_lobby_screen: LobbyScreen::default(),
            network: None,
            ready_tracker: ReadyTracker::new(),
            return_to_menu: false,
        })
    }

    fn create_renderer(&mut self, window: Arc<Window>) -> Box<dyn Renderer> {
        create_renderer(self.kind, self.width, self.height, window)
    }

    fn screen_to_world(&self, position: winit::dpi::PhysicalPosition<f64>) -> Vec2 {
        self.camera.screen_to_world(Vec2::new(
            Fixed16_16::from_f32(position.x as f32),
            Fixed16_16::from_f32(position.y as f32),
        ))
    }

    fn cursor_world(&self) -> Option<Vec2> {
        self.cursor.map(|position| self.screen_to_world(position))
    }

    /// Map a world point from the zoomed game view into HUD space. The HUD is
    /// drawn unzoomed, so clicks, hovers and minimap hit tests need this to line
    /// up with the build menu and minimap.
    fn hud_world(&self, world: Vec2) -> Vec2 {
        let position = self.camera.position;
        let zoom = self.camera.zoom.to_f32().max(1.0e-6);
        Vec2::new(
            Fixed16_16::from_f32(
                position.x.to_f32() + (world.x.to_f32() - position.x.to_f32()) * zoom,
            ),
            Fixed16_16::from_f32(
                position.y.to_f32() + (world.y.to_f32() - position.y.to_f32()) * zoom,
            ),
        )
    }

    /// World under the cursor in HUD space (see hud_world).
    fn hud_cursor_world(&self) -> Option<Vec2> {
        self.cursor_world().map(|world| self.hud_world(world))
    }

    /// Zoom the TD world view by factor, anchored at the cursor.
    fn zoom_td(&mut self, factor: f32) {
        let world_w = self.game.map.width as f32 * cnc_tiberian_dawn::map::TILE_SIZE;
        let world_h = self.game.map.height as f32 * cnc_tiberian_dawn::map::TILE_SIZE;
        let anchor = self.cursor.map(|cursor| (cursor.x as f32, cursor.y as f32));
        zoom_camera_at(&mut self.camera, anchor, factor, world_w, world_h);
    }

    /// Center the TD camera when a click lands on the bottom-left minimap.
    fn td_minimap_click(&mut self, world: Vec2) -> bool {
        let world = self.hud_world(world);
        let view_w = self.camera.viewport_width as f32;
        let view_h = self.camera.viewport_height as f32;
        let left = self.camera.position.x.to_f32() - view_w * 0.5;
        let top = self.camera.position.y.to_f32() - view_h * 0.5;
        let Some(cell) = minimap_cell_at(
            world,
            self.game.map.width,
            self.game.map.height,
            left,
            top,
            view_w,
            view_h,
            false,
        ) else {
            return false;
        };
        self.camera.position = Vec2::new(
            Fixed16_16::from_f32((cell.x as f32 + 0.5) * cnc_tiberian_dawn::map::TILE_SIZE),
            Fixed16_16::from_f32((cell.y as f32 + 0.5) * cnc_tiberian_dawn::map::TILE_SIZE),
        );
        true
    }

    /// Pan the TD camera while the cursor rests near a window edge.
    fn apply_td_edge_pan(&mut self, delta: f32) {
        let Some(cursor) = self.cursor else {
            return;
        };
        let (dx, dy) = edge_pan_direction(
            cursor.x as f32,
            cursor.y as f32,
            self.width as f32,
            self.height as f32,
            8.0,
        );
        if dx == 0.0 && dy == 0.0 {
            return;
        }
        let zoom = self.camera.zoom.to_f32().max(1.0e-6);
        let step = 520.0 * delta / zoom;
        let max_x = self.game.map.width as f32 * cnc_tiberian_dawn::map::TILE_SIZE;
        let max_y = self.game.map.height as f32 * cnc_tiberian_dawn::map::TILE_SIZE;
        self.camera.position = Vec2::new(
            Fixed16_16::from_f32((self.camera.position.x.to_f32() + dx * step).clamp(0.0, max_x)),
            Fixed16_16::from_f32((self.camera.position.y.to_f32() + dy * step).clamp(0.0, max_y)),
        );
    }

    /// Repair the selected TD building, paying for missing health.
    fn repair_selected_building(&mut self) {
        let Some(index) = self.selected_building else {
            return;
        };
        let player = self.game.player_owner;
        match self.game.repair_building(player, index) {
            Some(cost) => {
                self.td_notice = Some((
                    format!("Building repaired (-{cost} credits)"),
                    Instant::now(),
                ));
                tracing::info!(index, cost, "TD building repaired");
            }
            None => {
                self.td_notice = Some(("No repair needed".to_string(), Instant::now()));
            }
        }
    }

    /// Cycle through the three TD save slots.
    fn cycle_td_save_slot(&mut self) {
        self.save_slot = (self.save_slot + 1) % 3;
        self.td_notice = Some((
            format!("Save slot {}/3", self.save_slot + 1),
            Instant::now(),
        ));
    }

    /// Leave the TD campaign-complete screen.
    fn dismiss_td_campaign_complete(&mut self) {
        self.td_complete = false;
        self.td_carryover = None;
        let carryover_path = self.td_progress_path.with_file_name("td-carryover.bin");
        let _ = std::fs::remove_file(&carryover_path);
        self.selecting_mission = !self.mission_choices.is_empty();
    }

    /// Cancel the front TD production item.
    fn cancel_td_production(&mut self) {
        let player = self.game.player_owner;
        match self.game.cancel_production(player) {
            Some(cost) => {
                self.td_notice = Some((
                    format!("Production cancelled (+{cost} credits)"),
                    Instant::now(),
                ));
            }
            None => {
                self.td_notice = Some(("No production to cancel".to_string(), Instant::now()));
            }
        }
    }

    /// Sell the building selected by a left click (TD).
    fn sell_selected_building(&mut self) {
        let Some(index) = self.selected_building.take() else {
            return;
        };
        let player = self.game.player_owner;
        match self.game.sell_building(player, index) {
            Some(refund) => {
                self.td_notice =
                    Some((format!("Building sold (+{refund} credits)"), Instant::now()));
                tracing::info!(index, refund, "TD building sold");
            }
            None => {
                self.td_notice = Some(("No building to sell".to_string(), Instant::now()));
            }
        }
    }

    /// Adjust the TD simulation speed (0.25x .. 3x).
    fn adjust_td_speed(&mut self, delta: f32) {
        self.game_speed = (self.game_speed + delta).clamp(0.25, 3.0);
        self.td_notice = Some((
            format!("Game speed {:.2}x", self.game_speed),
            Instant::now(),
        ));
    }

    /// Store the selected TD units in a control group (by unit index).
    fn store_td_control_group(&mut self, slot: usize) {
        let player = self.game.player_owner;
        let indices: Vec<usize> = self
            .game
            .units
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.is_selected && unit.owner == player && unit.is_alive())
            .map(|(index, _)| index)
            .collect();
        if let Some(group) = self.control_groups.get_mut(slot) {
            *group = indices;
            self.td_notice = Some((format!("Group {} stored", slot + 1), Instant::now()));
        }
    }

    /// Recall a stored TD control group; false when the slot is empty.
    fn recall_td_control_group(&mut self, slot: usize) -> bool {
        let Some(indices) = self.control_groups.get(slot).cloned() else {
            return false;
        };
        if indices.is_empty() {
            return false;
        }
        let player = self.game.player_owner;
        for (index, unit) in self.game.units.iter_mut().enumerate() {
            unit.is_selected = indices.contains(&index) && unit.owner == player && unit.is_alive();
        }
        self.td_notice = Some((format!("Group {} recalled", slot + 1), Instant::now()));
        true
    }

    /// Store the current camera view in a bookmark slot.
    fn save_td_bookmark(&mut self, slot: usize) {
        if let Some(bookmark) = self.bookmarks.get_mut(slot) {
            *bookmark = Some(self.camera.position);
            self.td_notice = Some((format!("View {} saved", slot + 1), Instant::now()));
        }
    }

    /// Jump to a stored camera bookmark; false when the slot is empty.
    fn jump_td_bookmark(&mut self, slot: usize) -> bool {
        if let Some(Some(position)) = self.bookmarks.get(slot).copied() {
            self.camera.position = position;
            true
        } else {
            false
        }
    }

    // Slot 0 wrapper kept alongside the slot-aware variant for the upcoming
    // quick-save sidebar button.
    #[allow(dead_code)]
    fn quick_save_path() -> PathBuf {
        Self::quick_save_path_for(0)
    }

    fn quick_save_path_for(slot: usize) -> PathBuf {
        let dir = std::env::var("CNC_SAVE_DIR").unwrap_or_else(|_| "saves".to_string());
        let file = if slot == 0 {
            "quicksave.sav".to_string()
        } else {
            format!("quicksave{}.sav", slot + 1)
        };
        PathBuf::from(dir).join(file)
    }

    fn quick_save(&mut self) -> bool {
        let path = Self::quick_save_path_for(self.save_slot);
        match self.game.save_to_file(&path) {
            Ok(()) => {
                self.td_notice = Some(("Game saved".to_string(), Instant::now()));
                tracing::info!(path = %path.display(), "quicksave written");
                true
            }
            Err(error) => {
                self.td_notice = Some((format!("Save failed: {error}"), Instant::now()));
                tracing::warn!(%error, "quicksave failed");
                false
            }
        }
    }

    fn quick_load(&mut self) -> bool {
        let path = Self::quick_save_path_for(self.save_slot);
        match Game::load_from_file(&path) {
            Ok(game) => {
                self.game = game;
                self.camera.position = self.game.world_center();
                self.td_score = None;
                self.td_score_since = None;
                self.last_produced = self.game.units_produced;
                self.last_harvested = self.game.harvested_credits;
                self.last_aircraft_shots = self.game.aircraft_shots;
                self.last_aircraft_rearmed = self.game.aircraft_rearmed;
                self.last_secondary_shots = self.game.secondary_shots;
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.set_camera(&self.camera);
                }
                self.td_notice = Some(("Game loaded".to_string(), Instant::now()));
                tracing::info!(path = %path.display(), "quicksave loaded");
                true
            }
            Err(error) => {
                self.td_notice = Some((format!("Load failed: {error}"), Instant::now()));
                tracing::warn!(%error, path = %path.display(), "quickload failed");
                false
            }
        }
    }

    /// True once when a screen asks the shell to return to the main menu.
    fn take_return_to_menu(&mut self) -> bool {
        std::mem::take(&mut self.return_to_menu)
    }

    // ----- Tiberian Dawn network lobby and lockstep -----------------------

    /// Open the TD host/join lobby (menu "Online Game" choice).
    // Wired by the menu shell integration that is still pending; kept so the
    // lobby state plumbing stays in place.
    #[allow(dead_code)]
    fn open_td_lobby(&mut self) {
        self.td_lobby_open = true;
        self.td_lobby_error = None;
        self.td_lan_scan = None;
        self.td_lobby.focus = 0;
        self.td_lobby_screen = LobbyScreen::new(&self.td_lobby.join_address);
        refresh_lobby_screen(&mut self.td_lobby_screen);
    }

    /// Switch the TD lobby between the directory and LAN-multicast backends.
    fn toggle_td_lobby_directory(&mut self) {
        self.td_lobby_screen.toggle_mode();
        let scan = &self.td_lobby_screen.scan;
        if scan.used_fallback() || !scan.has_directory_rows() {
            if self.td_lobby_error.is_none() {
                self.td_lobby_error = Some(scan.status_line());
            }
        } else {
            self.td_lobby_error = None;
        }
    }

    /// Broadcast the TD local player's ready flag to every known peer.
    ///
    /// The TD lane previously had no send hook at all: the gate in
    /// `start_td_lobby` could never open because only remote flags arrived.
    fn send_td_ready(&mut self, ready: bool) {
        let Some(net) = self.network.as_mut() else {
            return;
        };
        let local = net.session.local_player;
        // Tag the lane in the ready identity: the RA and TD lobbies both number
        // players 0/1, so a bare "Player1" from each lane would collapse onto
        // one roster row. The beacon text carries the same game tag, so a host
        // tracking both lanes keeps one row per player. Display names stay
        // single tokens (the ready line is space-delimited).
        let name = format!("TD-Player{local}");
        let _ = self.ready_tracker.mark_ready(&name, ready);
        let peers: Vec<SocketAddr> = net.session.peers.values().copied().collect();
        broadcast_ready(&net.socket, &name, ready, &peers);
    }

    fn td_network_active(&self) -> bool {
        self.network.is_some()
    }

    fn toggle_td_lan_scan(&mut self) {
        toggle_lan_scan(&mut self.td_lan_scan, &mut self.td_lobby_error);
    }

    fn poll_td_lan_scan(&mut self) {
        poll_lan_scan_socket(&mut self.td_lan_scan);
    }

    /// Start the match described by the TD lobby fields.
    fn start_td_lobby(&mut self) -> Result<()> {
        if self.td_lobby.role == NetRole::Host
            && self.ready_tracker.expected_count() > 0
            && !self.ready_tracker.all_ready()
        {
            self.td_lobby_error = Some("Waiting for all players to ready up".to_string());
            return Ok(());
        }
        let setup = match self.td_lobby.role {
            NetRole::Host => NetworkSetup {
                role: "host".to_string(),
                join: None,
                port: self.td_lobby.host_port,
                peers: vec![self.td_lobby.peer_address.trim().to_string()],
                bind: "0.0.0.0".to_string(),
                advertised: None,
            },
            NetRole::Join => NetworkSetup {
                role: "join".to_string(),
                join: Some(if self.td_lobby.join_address.trim().is_empty() {
                    // Directory fallback: the manual address row is the target.
                    lobby_direct_address(&self.td_lobby_screen)
                } else {
                    self.td_lobby.join_address.trim().to_string()
                }),
                port: self.td_lobby.join_port,
                peers: Vec::new(),
                bind: "0.0.0.0".to_string(),
                advertised: None,
            },
        };
        self.td_lobby_error = None;
        match self.enable_td_network(&setup) {
            Ok(()) => {
                self.td_lan_scan = None;
                self.td_lobby_open = false;
                self.td_notice = Some(("Network game started".to_string(), Instant::now()));
                let players = self.network.as_ref().map(|net| net.session.players);
                if let Some(players) = players {
                    self.ready_tracker.set_expected(players as usize);
                }
                Ok(())
            }
            Err(error) => {
                self.td_lobby_error = Some(error.to_string());
                Ok(())
            }
        }
    }

    /// Bind the UDP socket and attach a live lockstep session to the TD view.
    fn enable_td_network(&mut self, setup: &NetworkSetup) -> Result<()> {
        let config = NetworkConfig {
            bind_address: setup.bind.clone(),
            port: setup.port,
            advertised: setup.advertised.clone(),
            max_players: 2,
        };
        config.validate().map_err(|error| anyhow!(error))?;
        let local_player: u8 = match setup.role.as_str() {
            "host" => 0,
            "join" => 1,
            other => return Err(anyhow!("unknown net role '{}' (host|join)", other)),
        };
        let bind = format!("{}:{}", setup.bind.trim(), setup.port);
        let socket = UdpSocket::bind(&bind).with_context(|| format!("failed to bind {bind}"))?;
        socket
            .set_nonblocking(true)
            .context("failed to make the network socket non-blocking")?;
        let _ = socket.set_broadcast(true);
        let local_address = socket.local_addr().ok();
        let mut session = LiveSession::new(local_player, 2, local_address)
            .map_err(|error| anyhow!("network session setup failed: {error:?}"))?;
        if local_player == 0 {
            for (index, peer) in setup
                .peers
                .iter()
                .filter(|peer| !peer.trim().is_empty())
                .enumerate()
            {
                let address = peer
                    .parse::<SocketAddr>()
                    .with_context(|| format!("invalid peer '{peer}'"))?;
                session.register_peer((1 + index) as u8, address);
            }
        } else {
            let host = setup
                .join
                .clone()
                .ok_or_else(|| anyhow!("joining requires a host address"))?;
            let address = host
                .parse::<SocketAddr>()
                .with_context(|| format!("invalid host address '{host}'"))?;
            session.register_peer(0, address);
        }
        // Both clients rebuild the same demo so pre-match input cannot leak in.
        let rules = self.game.rules.clone();
        self.game = Game::with_rules("TD Network", MAP_SIZE, MAP_SIZE, rules);
        self.td_score = None;
        self.td_score_since = None;
        self.game.setup_demo();
        self.game.player_owner = local_player;
        self.selecting_mission = false;
        self.td_briefing_open = false;
        self.td_stats_open = false;
        self.td_complete = false;
        self.td_paused = false;
        self.selected_building = None;
        self.camera.position = self.game.world_center();
        let peers = session.peers.len();
        self.network = Some(TdNetwork {
            socket,
            session,
            last_tick: Instant::now(),
            sent: 0,
            applied: 0,
            seen: HashSet::new(),
            role: setup.role.clone(),
            last_beacon: Instant::now(),
        });
        println!(
            "td net role={} local_player={} addr={:?} peers={}",
            setup.role, local_player, local_address, peers
        );
        tracing::info!(
            role = %setup.role,
            local_player,
            peers,
            "TD network session started"
        );
        Ok(())
    }

    /// Drain incoming datagrams and advance one lockstep frame on a timer.
    fn poll_td_network(&mut self) {
        let Some(net) = self.network.as_mut() else {
            return;
        };
        let mut buffer = [0u8; 2048];
        loop {
            match net.socket.recv_from(&mut buffer) {
                Ok((size, source)) => {
                    if let Some((player, ready)) = parse_ready_line(&buffer[..size]) {
                        let _ = self.ready_tracker.mark_ready(&player, ready);
                        continue;
                    }
                    if parse_chat_line(&buffer[..size]).is_some() {
                        continue;
                    }
                    let event = net.session.handle_datagram(source, &buffer[..size]);
                    match event {
                        LiveEvent::Commands(packet) => {
                            for command in &packet.commands {
                                if !net.seen.insert(command.hash()) {
                                    continue;
                                }
                                if apply_td_command(&mut self.game, command) {
                                    net.applied += 1;
                                }
                            }
                        }
                        LiveEvent::Migration(_) | LiveEvent::Duplicate | LiveEvent::Invalid => {}
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(error) => {
                    tracing::warn!(%error, "TD network receive failed");
                    break;
                }
            }
        }

        if net.role == "host" && net.last_beacon.elapsed() >= Duration::from_secs(2) {
            net.last_beacon = Instant::now();
            if let Ok(local) = net.socket.local_addr() {
                let beacon = lan_beacon_text(local.port(), "TD");
                for address in [
                    std::net::Ipv4Addr::BROADCAST,
                    std::net::Ipv4Addr::new(127, 255, 255, 255),
                ] {
                    let _ = net
                        .socket
                        .send_to(beacon.as_bytes(), (address, LAN_DISCOVERY_PORT));
                }
            }
        }

        if net.last_tick.elapsed() < Duration::from_millis(250) {
            return;
        }
        net.last_tick = Instant::now();
        match net.session.advance() {
            Ok((_commands, bytes, targets)) => {
                for target in targets {
                    if net.socket.send_to(&bytes, target).is_ok() {
                        net.sent += 1;
                    }
                }
            }
            Err(error) => tracing::warn!(?error, "TD network packet encoding failed"),
        }
    }

    /// Queue one local command for the next lockstep frame.
    fn send_td_command(&mut self, command: NetCommandKind) {
        if let Some(net) = self.network.as_mut() {
            net.session.queue_local(command);
        }
    }

    /// Queue a sidebar unit/structure locally and broadcast the same command.
    fn queue_td_buildable(&mut self, buildable: Buildable) -> bool {
        let player = self.game.player_owner;
        let queued = match buildable {
            Buildable::Unit(unit) => self.game.queue_unit(player, unit),
            Buildable::Structure(structure) => self.game.queue_building(player, structure),
        };
        if queued {
            self.send_td_command(td_produce_command(buildable));
        }
        queued
    }

    /// Place a completed structure locally and broadcast the placement.
    // Engaged once map mirroring plumbing is complete (in progress).
    #[allow(dead_code)]
    fn place_td_building(&mut self, cell: Vec2i) -> bool {
        let player = self.game.player_owner;
        let placed = self.game.place_ready_building(player, cell);
        if placed {
            let tile = cnc_tiberian_dawn::map::TILE_SIZE;
            let location = Vec2::new(
                Fixed16_16::from_f32((cell.x as f32 + 0.5) * tile),
                Fixed16_16::from_f32((cell.y as f32 + 0.5) * tile),
            );
            self.send_td_command(NetCommandKind::PlaceBuilding { location });
        }
        placed
    }

    /// Pause menu methods for the TD game view.
    fn enter_td_pause(&mut self) {
        self.td_paused = true;
        self.td_pause_choice = 0;
    }

    fn activate_td_pause_choice(&mut self) {
        match self.td_pause_choice {
            0 => self.td_paused = false,
            1 => {
                self.quick_save();
            }
            2 => {
                if self.quick_load() {
                    self.td_paused = false;
                }
            }
            3 => {
                self.td_paused = false;
                if self.mission_choices.is_empty() {
                    // A skirmish has no mission selector to fall back to.
                    self.return_to_menu = true;
                } else {
                    self.selecting_mission = true;
                }
            }
            4 => self.cycle_td_save_slot(),
            _ => {}
        }
    }

    fn load_td_progress(&mut self) {
        let Ok(text) = std::fs::read_to_string(&self.td_progress_path) else {
            return;
        };
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            match key.trim().to_ascii_lowercase().as_str() {
                "completed" => {
                    for index in value
                        .split(',')
                        .filter_map(|index| index.trim().parse::<usize>().ok())
                    {
                        if let Some(slot) = self.td_completed.get_mut(index) {
                            *slot = true;
                        }
                    }
                }
                "besttimes" => {
                    for entry in value.split(',') {
                        let Some((index, seconds)) = entry.trim().split_once(':') else {
                            continue;
                        };
                        let (Ok(index), Ok(seconds)) =
                            (index.trim().parse::<usize>(), seconds.trim().parse::<i32>())
                        else {
                            continue;
                        };
                        if let Some(slot) = self.td_best.get_mut(index) {
                            *slot = Some(seconds);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn save_td_progress(&self) {
        if let Some(parent) = self.td_progress_path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        let completed: Vec<String> = self
            .td_completed
            .iter()
            .enumerate()
            .filter(|(_, done)| **done)
            .map(|(index, _)| index.to_string())
            .collect();
        let best: Vec<String> = self
            .td_best
            .iter()
            .enumerate()
            .filter_map(|(index, seconds)| seconds.map(|seconds| format!("{index}:{seconds}")))
            .collect();
        if let Err(error) = std::fs::write(
            &self.td_progress_path,
            format!(
                "[Campaign]\nCompleted={}\nBestTimes={}\n",
                completed.join(","),
                best.join(",")
            ),
        ) {
            tracing::warn!(%error, "failed to save TD campaign progress");
        }
    }

    /// Leave a finished TD mission: record it and return to the selector,
    /// selecting the next mission after a victory.
    fn finish_td_mission(&mut self) {
        if self.game.mission.outcome.is_none() {
            return;
        }
        self.record_mission_result();
        let won = self.game.mission.outcome == Some(MissionOutcome::Victory);
        if won {
            // CARRY.CPP: surviving units + treasury bridge to the next
            // mission; persisted so a reload of the selector keeps it.
            self.td_carryover = Some(Carryover::from_game(&self.game, self.game.player_owner));
            if let Some(carryover) = &self.td_carryover {
                let path = self
                    .td_progress_path
                    .with_file_name(campaign::CARRYOVER_FILE_NAME);
                match carryover.save_to_file(&path) {
                    Ok(()) => tracing::info!(path = %path.display(), "carry-over saved"),
                    Err(error) => tracing::warn!(%error, "failed to save carry-over"),
                }
            }
        }
        self.td_briefing_open = false;
        if won && self.td_completed.iter().all(|done| *done) {
            self.td_complete = true;
            self.selecting_mission = false;
            self.td_recorded = false;
            return;
        }
        if !won {
            self.selecting_mission = true;
            self.td_recorded = false;
            return;
        }
        // Victory: advance to the next incomplete mission. If there is no
        // such mission, or its file is missing/unreadable, the campaign is
        // complete (completion screen) instead of panicking on a missing file.
        let paths: Vec<PathBuf> = self
            .mission_choices
            .iter()
            .map(|choice| choice.path.clone())
            .collect();
        if let Some(next) = campaign::next_after(&self.td_completed, self.mission_index, &paths) {
            self.mission_index = next;
            self.selecting_mission = true;
        } else {
            self.td_complete = true;
            self.selecting_mission = false;
        }
        self.td_recorded = false;
    }

    /// First Enter after a mission ends shows the scoreboard (port of the
    /// original `SCORE.CPP` beat); the next Enter leaves the mission.
    fn show_td_score(&mut self) {
        let won = self.game.mission.outcome == Some(MissionOutcome::Victory);
        self.td_score = Some(score::compute_score(
            self.game.harvested_credits,
            self.game.units_produced,
            self.game.projectiles_fired,
            self.game.elapsed.int().max(0) as u64,
            won,
            score::DIFFICULTY_NEUTRAL_BP,
        ));
        self.td_score_since = Some(Instant::now());
    }

    fn record_mission_result(&mut self) {
        if self.td_recorded || self.game.mission.outcome != Some(MissionOutcome::Victory) {
            return;
        }
        let index = self.mission_index;
        if let Some(slot) = self.td_completed.get_mut(index) {
            *slot = true;
        }
        let elapsed = self.game.elapsed.int();
        if let Some(slot) = self.td_best.get_mut(index) {
            if slot.is_none_or(|best| elapsed < best) {
                *slot = Some(elapsed);
            }
        }
        self.td_recorded = true;
        self.save_td_progress();
        tracing::info!(mission = index + 1, elapsed, "TD mission complete");
    }

    fn load_mission_dir(&mut self, dir: &Path) -> Result<usize> {
        self.mission_dir = Some(dir.to_path_buf());
        self.mission_choices.clear();
        // A fresh mission directory starts a new campaign: drop any stale
        // carry-over so it cannot be applied to the first mission twice.
        self.td_carryover = None;
        let carryover_path = self.td_progress_path.with_file_name("td-carryover.bin");
        let _ = std::fs::remove_file(&carryover_path);
        let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
            .with_context(|| format!("failed to read mission directory {}", dir.display()))?
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
            })
            .collect();
        paths.sort();
        for path in paths {
            match Mission::from_file(&path) {
                Ok(mission) => {
                    let spawns = mission_spawns(&mission);
                    self.mission_choices.push(TdChoice {
                        path,
                        name: mission.name,
                        objectives: mission.objectives.len(),
                        triggers: mission.triggers.len(),
                        spawns,
                    });
                }
                Err(error) => {
                    tracing::warn!(path = %path.display(), %error, "skipping invalid mission");
                }
            }
        }
        self.mission_index = 0;
        self.selecting_mission = !self.mission_choices.is_empty();
        let len = self.mission_choices.len();
        self.td_completed = vec![false; len];
        self.td_best = vec![None; len];
        self.load_td_progress();
        // Restore the carry-over written by the previous mission so a selector
        // reload (or a restart) keeps the surviving units + treasury. A missing
        // or corrupt file just starts the next mission clean.
        self.load_td_carryover();
        Ok(len)
    }

    /// Read the persisted carry-over, degrading to `None` when the file is
    /// absent or unusable (never fatal, never a corrupt save).
    fn load_td_carryover(&mut self) {
        let path = self
            .td_progress_path
            .with_file_name(campaign::CARRYOVER_FILE_NAME);
        if !path.is_file() {
            return;
        }
        self.td_carryover = campaign::CampaignFlow::load_carryover(&path);
    }

    fn td_visible(&self) -> Vec<usize> {
        (0..self.mission_choices.len())
            .filter(|index| match self.td_filter {
                CampaignFilter::All => true,
                CampaignFilter::Incomplete => {
                    !self.td_completed.get(*index).copied().unwrap_or(false)
                }
                CampaignFilter::Completed => {
                    self.td_completed.get(*index).copied().unwrap_or(false)
                }
            })
            .collect()
    }

    fn cycle_mission(&mut self, delta: i32) {
        let visible = self.td_visible();
        if visible.is_empty() {
            return;
        }
        let position = visible
            .iter()
            .position(|index| *index == self.mission_index)
            .unwrap_or(0);
        let len = visible.len() as i32;
        let next = (((position as i32 + delta) % len) + len) % len;
        self.mission_index = visible[next as usize];
    }

    fn cycle_td_filter(&mut self) {
        self.td_filter = self.td_filter.next();
        let visible = self.td_visible();
        if !visible.is_empty() && !visible.contains(&self.mission_index) {
            self.mission_index = visible[0];
        }
    }

    fn activate_selected_mission(&mut self) -> Result<bool> {
        let Some(choice) = self.mission_choices.get(self.mission_index).cloned() else {
            return Ok(false);
        };
        let path = choice.path.clone();
        if !path.exists() {
            // Missing mission file: finish the campaign on the completion
            // screen instead of panicking.
            self.td_complete = true;
            self.selecting_mission = false;
            return Ok(false);
        }
        if let Err(error) = self.game.load_mission(&path) {
            tracing::warn!(path = %path.display(), %error, "failed to load mission");
            self.td_complete = true;
            self.selecting_mission = false;
            return Ok(false);
        }
        self.selecting_mission = false;
        self.selected_building = None;
        self.td_recorded = false;
        // Load a carry-over persisted by the previous mission when the
        // in-memory one is gone (e.g. after a restart), exactly once.
        if self.td_carryover.is_none() {
            let carryover_path = self.td_progress_path.with_file_name("td-carryover.bin");
            if carryover_path.exists() {
                match Carryover::load_from_file(&carryover_path) {
                    Ok(loaded) => self.td_carryover = Some(loaded),
                    Err(error) => {
                        tracing::warn!(path = %carryover_path.display(), %error, "invalid carry-over; skipping");
                        let _ = std::fs::remove_file(&carryover_path);
                    }
                }
            }
        }
        // Apply the previous mission's carry-over once, after the scenario
        // has set its default starting forces.
        if let Some(carryover) = self.td_carryover.take() {
            carryover.apply_to_game(&mut self.game);
            let carryover_path = self.td_progress_path.with_file_name("td-carryover.bin");
            let _ = std::fs::remove_file(&carryover_path);
        }
        self.td_briefing_open =
            !self.game.mission.briefing.is_empty() || !self.game.mission.objectives.is_empty();
        self.camera.position = self.game.world_center();
        self.last_produced = self.game.units_produced;
        self.last_harvested = self.game.harvested_credits;
        self.last_aircraft_shots = self.game.aircraft_shots;
        self.last_aircraft_rearmed = self.game.aircraft_rearmed;
        self.last_secondary_shots = self.game.secondary_shots;
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.set_camera(&self.camera);
        }
        tracing::info!(
            mission = %self.game.mission.name,
            path = %path.display(),
            "mission selected"
        );
        Ok(true)
    }

    fn sidebar(&self) -> Sidebar {
        // Screen-sized HUD space (unzoomed) so the menu does not grow with zoom.
        let half_w = self.camera.viewport_width as f32 * 0.5;
        let half_h = self.camera.viewport_height as f32 * 0.5;
        let left = self.camera.position.x.to_f32() - half_w;
        let top = self.camera.position.y.to_f32() - half_h;
        let x = left + 2.0 * half_w - SIDEBAR_WIDTH - 8.0;
        let y = top + 8.0;
        let units_header_y = y + 36.0;

        // Only rows whose TechLevel and prerequisites are satisfied appear;
        // rules-derived costs are shown so a Rules.ini override is visible.
        let options = self.game.visible_sidebar_options(0);
        let mut entries = Vec::new();
        let mut row_y = units_header_y + 12.0;

        for option in &options {
            let Buildable::Unit(unit) = option.buildable else {
                continue;
            };
            entries.push(SidebarEntry {
                x,
                y: row_y,
                w: SIDEBAR_WIDTH,
                h: 18.0,
                action: BuildAction::Unit(unit),
                label: build_label(option.buildable),
                cost: option.cost,
                enabled: self.game.can_queue_unit(self.game.player_owner, unit),
            });
            row_y += SIDEBAR_ROW_HEIGHT;
        }

        let structures_header_y = row_y + 8.0;
        row_y = structures_header_y + 12.0;
        for option in &options {
            let Buildable::Structure(structure) = option.buildable else {
                continue;
            };
            entries.push(SidebarEntry {
                x,
                y: row_y,
                w: SIDEBAR_WIDTH,
                h: 18.0,
                action: BuildAction::Structure(structure),
                label: build_label(option.buildable),
                cost: option.cost,
                enabled: self
                    .game
                    .can_queue_building(self.game.player_owner, structure),
            });
            row_y += SIDEBAR_ROW_HEIGHT;
        }

        Sidebar {
            x,
            y,
            w: SIDEBAR_WIDTH,
            h: (row_y - y) + 8.0,
            units_header_y,
            structures_header_y,
            entries,
        }
    }

    fn sidebar_hit(&self, world: Vec2) -> SidebarHit {
        let world = self.hud_world(world);
        let sidebar = self.sidebar();
        let x = world.x.to_f32();
        let y = world.y.to_f32();
        if x < sidebar.x || x > sidebar.x + sidebar.w || y < sidebar.y || y > sidebar.y + sidebar.h
        {
            return SidebarHit::Outside;
        }
        for entry in &sidebar.entries {
            if x >= entry.x && x <= entry.x + entry.w && y >= entry.y && y <= entry.y + entry.h {
                return SidebarHit::Action(entry.action);
            }
        }
        SidebarHit::Panel
    }

    fn handle_sidebar_click(&mut self, world: Vec2) -> bool {
        match self.sidebar_hit(world) {
            SidebarHit::Action(action) => {
                let queued = match action {
                    BuildAction::Unit(unit) => self.queue_td_buildable(Buildable::Unit(unit)),
                    BuildAction::Structure(structure) => {
                        self.queue_td_buildable(Buildable::Structure(structure))
                    }
                };
                tracing::debug!(?action, queued, "sidebar build");
                true
            }
            SidebarHit::Panel => true,
            SidebarHit::Outside => false,
        }
    }

    /// Bind the tileset pack a screen needs to the renderer's tileset texture
    /// slot. Uploads are tagged and rare: the slot keeps its contents across
    /// frames and is only (re)uploaded when the active palette changes, so this
    /// runs once per renderer lifetime for every shell whose screens share one
    /// palette — the same upload-once pattern as the SpriteAtlas in `resumed`.
    fn bind_tileset(&mut self, palette: TilesetPalette) {
        let tag = match palette {
            TilesetPalette::Ra => 1,
            TilesetPalette::Td => 2,
        };
        if self.tileset_slot == Some(tag) {
            return;
        }
        let pack = match palette {
            TilesetPalette::Ra => self.ra_tileset.as_ref(),
            TilesetPalette::Td => self.td_tileset.as_ref(),
        };
        let Some(pack) = pack else {
            return;
        };
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };
        if !renderer.uses_sprites() {
            // Sprites never draw on this backend; the draw paths keep flat
            // colours, so there is nothing to upload.
            return;
        }
        match renderer.set_atlas_texture(
            cnc_renderer::TILESET_TEXTURE_ID,
            pack.width(),
            pack.height(),
            pack.rgba(),
        ) {
            Ok(()) => {
                tracing::info!(palette = pack.palette(), "terrain tileset atlas ready");
                self.tileset_slot = Some(tag);
            }
            Err(error) => tracing::warn!(%error, "failed to upload terrain tileset atlas"),
        }
    }

    fn draw(&mut self) {
        let now = Instant::now();
        let delta = (now - self.last_frame).as_secs_f32().clamp(1.0e-4, 0.1);
        self.last_frame = now;
        if self.td_lobby_open {
            self.poll_td_lan_scan();
            let Some(renderer) = self.renderer.as_mut() else {
                return;
            };
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_net_lobby(
                renderer.as_mut(),
                "TIBERIAN DAWN",
                &self.td_lobby,
                self.td_lan_scan.as_ref(),
                &self.td_lobby_screen,
                self.td_lobby_error.as_deref(),
                &self.ready_tracker.roster(),
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "TD lobby frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }
        if self.network.is_some()
            && !self.selecting_mission
            && !self.td_paused
            && !self.td_briefing_open
        {
            self.poll_td_network();
        }
        if !self.selecting_mission && !self.td_paused && !self.td_briefing_open {
            self.apply_td_edge_pan(delta);
        }

        if self.td_complete {
            let Some(renderer) = self.renderer.as_mut() else {
                return;
            };
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_td_campaign_complete(
                renderer.as_mut(),
                self.mission_choices.len(),
                &self.td_best,
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "TD campaign complete frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if self.selecting_mission {
            let Some(renderer) = self.renderer.as_mut() else {
                return;
            };
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_td_mission_select(
                renderer.as_mut(),
                &self.mission_choices,
                &self.td_completed,
                &self.td_best,
                self.mission_index,
                self.td_filter,
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "mission select frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if self.td_stats_open {
            let Some(renderer) = self.renderer.as_mut() else {
                return;
            };
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_td_stats(
                renderer.as_mut(),
                &self.mission_choices,
                &self.td_completed,
                &self.td_best,
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "TD stats frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if self.td_briefing_open {
            let borrowed_tileset = {
                let Some(renderer) = self.renderer.as_mut() else {
                    return;
                };
                renderer.begin_frame();
                renderer.set_camera(&screen_camera(self.width, self.height));
                let borrowed = draw_td_briefing(
                    renderer.as_mut(),
                    &self.game.mission,
                    self.td_tileset.as_ref(),
                    self.width,
                    self.height,
                );
                if let Err(error) = renderer.end_frame() {
                    tracing::warn!(%error, "TD briefing frame presentation failed");
                }
                borrowed
            };
            // A playing briefing borrows the shared tileset texture slot (plan
            // box 212); re-upload the terrain atlas now that the video frame has
            // been presented so the mission that follows samples the real
            // tileset. Skipped when no video played.
            if borrowed_tileset {
                if let (Some(renderer), Some(pack)) =
                    (self.renderer.as_mut(), self.td_tileset.as_ref())
                {
                    let _ = renderer.set_atlas_texture(
                        cnc_renderer::TILESET_TEXTURE_ID,
                        pack.width(),
                        pack.height(),
                        pack.rgba(),
                    );
                }
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        self.record_mission_result();

        let drag_rect = match (self.drag_start, self.drag_current) {
            (Some(start), Some(end)) => {
                let a = self.camera.screen_to_world(Vec2::new(
                    Fixed16_16::from_f32(start.x as f32),
                    Fixed16_16::from_f32(start.y as f32),
                ));
                let b = self.camera.screen_to_world(Vec2::new(
                    Fixed16_16::from_f32(end.x as f32),
                    Fixed16_16::from_f32(end.y as f32),
                ));
                let min_x = a.x.to_f32().min(b.x.to_f32());
                let min_y = a.y.to_f32().min(b.y.to_f32());
                let max_x = a.x.to_f32().max(b.x.to_f32());
                let max_y = a.y.to_f32().max(b.y.to_f32());
                Some((
                    min_x,
                    min_y,
                    (max_x - min_x).max(1.0),
                    (max_y - min_y).max(1.0),
                ))
            }
            _ => None,
        };

        let sidebar = self.sidebar();
        let cursor = self.cursor_world();
        // Hover highlighting works in HUD space, which ignores world zoom.
        let hud_cursor = self.hud_cursor_world();

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        if !self.td_paused {
            self.game
                .update(Fixed16_16::from_f32(delta * self.game_speed));
            // In-game SFX: pump engine events through the audio director
            // (todo 42). The EVA voice queue is drained by the existing
            // speech wiring; this lane covers positional/UI effects.
            let events = self.event_subscriber.drain();
            self.audio_director.route_batch(&self.mixer, &events);
            // Theme escalation: a damaged player structure switches the
            // music layer to combat (THEME.CPP On_Base_Attack analogue).
            if self.game.eva.pending() == Some(cnc_audio::EvaLine::BaseUnderAttack) {
                self.theme_queue.on_base_attack();
            }
        }

        if self.game.units_produced != self.last_produced {
            self.last_produced = self.game.units_produced;
            tracing::info!(total = self.last_produced, "unit production complete");
        }

        if self.game.harvested_credits != self.last_harvested {
            self.last_harvested = self.game.harvested_credits;
            tracing::info!(credits = self.last_harvested, "harvester unloaded");
        }

        if self.game.aircraft_shots != self.last_aircraft_shots {
            self.last_aircraft_shots = self.game.aircraft_shots;
            tracing::info!(shots = self.last_aircraft_shots, "aircraft fired");
        }

        if self.game.aircraft_rearmed != self.last_aircraft_rearmed {
            self.last_aircraft_rearmed = self.game.aircraft_rearmed;
            tracing::info!(
                rounds = self.last_aircraft_rearmed,
                "aircraft rearmed at helipad"
            );
        }

        if self.game.secondary_shots != self.last_secondary_shots {
            self.last_secondary_shots = self.game.secondary_shots;
            tracing::info!(shots = self.last_secondary_shots, "secondary weapon fired");
        }

        renderer.begin_frame();
        renderer.set_camera(&self.camera);
        if renderer.uses_sprites() {
            self.game
                .render_with_sprites(renderer.as_mut(), self.sprite_atlas.as_ref());
        } else {
            self.game.render(renderer.as_mut());
        }

        for unit in &self.game.units {
            if unit.is_alive() {
                draw_health_bar(
                    renderer.as_mut(),
                    unit.position.x.to_f32(),
                    unit.position.y.to_f32() - 14.0,
                    health_fraction(unit.health, unit.max_health),
                    16.0,
                );
            }
        }
        for building in &self.game.buildings {
            if building.is_alive() {
                draw_health_bar(
                    renderer.as_mut(),
                    building.position.x.to_f32(),
                    building.position.y.to_f32() - 18.0,
                    health_fraction(building.health, building.max_health),
                    24.0,
                );
            }
        }

        if let Some(index) = self.selected_building {
            if let Some(building) = self.game.buildings.get(index) {
                if building.is_alive() {
                    let tile = cnc_tiberian_dawn::map::TILE_SIZE;
                    let x = building.position.x.to_f32() - tile * 0.5;
                    let y = building.position.y.to_f32() - tile * 0.5;
                    let color = [1.0, 0.82, 0.30, 1.0];
                    for (rx, ry, rw, rh) in [
                        (x, y, tile, 2.0),
                        (x, y + tile - 2.0, tile, 2.0),
                        (x, y, 2.0, tile),
                        (x + tile - 2.0, y, 2.0, tile),
                    ] {
                        renderer.draw_rect(rx, ry, rw, rh, color);
                    }
                }
            }
        }

        if self
            .game
            .ready_building_for(self.game.player_owner)
            .is_some()
        {
            if let Some(world) = cursor {
                let tile = cnc_tiberian_dawn::map::TILE_SIZE;
                let cell = Vec2i::new(
                    (world.x.to_f32() / tile).floor() as i32,
                    (world.y.to_f32() / tile).floor() as i32,
                );
                let color = if self.game.can_place_building(cell) {
                    [0.40, 1.0, 0.50, 1.0]
                } else {
                    [1.0, 0.35, 0.30, 1.0]
                };
                let x = cell.x as f32 * tile;
                let y = cell.y as f32 * tile;
                renderer.draw_rect(x, y, tile, 1.5, color);
                renderer.draw_rect(x, y + tile - 1.5, tile, 1.5, color);
                renderer.draw_rect(x, y, 1.5, tile, color);
                renderer.draw_rect(x + tile - 1.5, y, 1.5, tile, color);
            }
        }

        // Drag-selection rectangle.
        if let Some((x, y, width, height)) = drag_rect {
            renderer.draw_rect(x, y, width, height, [0.20, 0.80, 1.00, 0.18]);
        }

        // HUD. Its own layer, drawn with an unzoomed camera so status text, the
        // build menu and the minimap keep a constant pixel size while the map
        // zooms underneath.
        renderer.begin_layer();
        renderer.set_camera(&hud_camera(&self.camera));
        let zoom = self.camera.zoom.to_f32().max(1.0e-6);
        let half_w = self.camera.viewport_width as f32 * 0.5;
        let half_h = self.camera.viewport_height as f32 * 0.5;
        let left = self.camera.position.x.to_f32() - half_w;
        let top = self.camera.position.y.to_f32() - half_h;

        let status = self.game.status();
        let player_vs = if self.game.player_owner == 0 {
            "GDI vs Nod"
        } else {
            "Nod vs GDI"
        };
        renderer.draw_text(
            &format!("{} - {player_vs}", status.mission),
            left + 16.0,
            top + 16.0,
            16.0,
            [0.90, 0.95, 1.00, 1.0],
        );
        renderer.draw_text(
            &format!(
                "units {}  structures {}  selected {}  GDI {} tib {}/{}  Nod {} tib {}/{}  power G {}/{} N {}/{}  queue {}  ion {}% nuke {}%  ready {:?}",
                status.units,
                status.buildings,
                status.selected,
                status.gdi_credits,
                status.gdi_tiberium,
                status.gdi_capacity,
                status.nod_credits,
                status.nod_tiberium,
                status.nod_capacity,
                status.gdi_power,
                status.gdi_drain,
                status.nod_power,
                status.nod_drain,
                status.production,
                status.gdi_ion,
                status.gdi_nuke,
                self.game.ready_building_for(self.game.player_owner)
            ),
            left + 16.0,
            top + 40.0,
            8.0,
            [0.70, 0.75, 0.85, 1.0],
        );
        renderer.draw_text(
            &format!(
                "1 inf 2 tank 3 mammoth 4 harv 5-9 build 0 orca  I ion N nuke  LMB select/place  Shift add  RMB move/attack  edge pan  A all  Bksp cancel  F5 save F9 load  Ctrl+1-4 view  Alt+1-4 group  X sell R repair  +/- or wheel zoom  F6/F7 speed  ESC pause  frame {} {}  {:.2}x",
                self.frame_index,
                self.kind.label(),
                self.game_speed
            ),
            left + 16.0,
            top + 56.0,
            8.0,
            [0.70, 0.75, 0.85, 1.0],
        );
        if let Some((notice, at)) = &self.td_notice {
            if at.elapsed() < Duration::from_secs(4) {
                renderer.draw_text(notice, left + 16.0, top + 72.0, 9.0, [1.0, 0.76, 0.35, 1.0]);
            }
        }

        // Campaign objective / mission log line.
        let mission_line = if let Some(outcome) = self.game.mission.outcome {
            match outcome {
                cnc_tiberian_dawn::MissionOutcome::Victory => {
                    "MISSION COMPLETE - Enter: mission select".to_string()
                }
                cnc_tiberian_dawn::MissionOutcome::Defeat => {
                    "MISSION FAILED - Enter: mission select".to_string()
                }
            }
        } else if !self.game.mission.objectives.is_empty() {
            let pending = self
                .game
                .mission
                .objectives
                .iter()
                .find(|objective| !objective.completed)
                .map(|objective| objective.text.as_str())
                .unwrap_or("all objectives complete");
            format!(
                "OBJECTIVES {}/{}  {}",
                status.objectives_done, status.objectives_total, pending
            )
        } else {
            String::new()
        };
        if !mission_line.is_empty() {
            let color = match self.game.mission.outcome {
                Some(cnc_tiberian_dawn::MissionOutcome::Victory) => [0.45, 1.00, 0.55, 1.0],
                Some(cnc_tiberian_dawn::MissionOutcome::Defeat) => [1.00, 0.45, 0.45, 1.0],
                None => [0.95, 0.85, 0.45, 1.0],
            };
            renderer.draw_text(&mission_line, left + 16.0, top + 72.0, 9.0, color);
            if let Some(last) = self.game.mission.messages.last() {
                renderer.draw_text(last, left + 16.0, top + 88.0, 8.0, [0.75, 0.85, 0.95, 1.0]);
            }
        }

        // End-of-mission scoreboard (port of the original SCORE.CPP beat).
        if let (Some(report), Some(since)) = (&self.td_score, self.td_score_since) {
            score::draw_overlay(
                renderer.as_mut(),
                report,
                since.elapsed(),
                left + 24.0,
                top + 100.0,
            );
        }

        // Sidebar build menu (drawn in world space, anchored to the camera).
        renderer.draw_rect(
            sidebar.x,
            sidebar.y,
            sidebar.w,
            sidebar.h,
            [0.04, 0.05, 0.08, 0.92],
        );
        renderer.draw_rect(
            sidebar.x,
            sidebar.y,
            sidebar.w,
            1.5,
            [0.25, 0.45, 0.75, 1.0],
        );
        renderer.draw_text(
            "BUILD MENU",
            sidebar.x + 8.0,
            sidebar.y + 6.0,
            10.0,
            [0.80, 0.90, 1.00, 1.0],
        );
        let (superweapon, charge) = if self.game.player_owner == 0 {
            ("ion", status.gdi_ion)
        } else {
            ("nuke", status.nod_nuke)
        };
        let queue_label = self
            .game
            .houses
            .get(self.game.player_owner as usize)
            .and_then(|house| house.production_queue.front())
            .map(|item| {
                format!(
                    "{:?} {}%",
                    item.class_id,
                    production_percent(item.progress, item.total_time)
                )
            })
            .unwrap_or_else(|| "empty".to_string());
        renderer.draw_text(
            &format!("queue {queue_label}  {superweapon} {charge}%"),
            sidebar.x + 8.0,
            sidebar.y + 22.0,
            8.0,
            [0.60, 0.70, 0.85, 1.0],
        );
        renderer.draw_text(
            "UNITS",
            sidebar.x + 8.0,
            sidebar.units_header_y,
            8.0,
            [0.55, 0.70, 0.95, 1.0],
        );
        renderer.draw_text(
            "STRUCTURES",
            sidebar.x + 8.0,
            sidebar.structures_header_y,
            8.0,
            [0.55, 0.70, 0.95, 1.0],
        );

        for entry in &sidebar.entries {
            let hovered = hud_cursor.is_some_and(|hover| {
                let x = hover.x.to_f32();
                let y = hover.y.to_f32();
                x >= entry.x && x <= entry.x + entry.w && y >= entry.y && y <= entry.y + entry.h
            });
            let background = if hovered && entry.enabled {
                [0.16, 0.30, 0.46, 0.95]
            } else {
                [0.07, 0.09, 0.13, 0.85]
            };
            renderer.draw_rect(entry.x + 4.0, entry.y, entry.w - 8.0, entry.h, background);
            let color = if entry.enabled {
                [0.65, 0.92, 0.65, 1.0]
            } else {
                [0.55, 0.40, 0.40, 1.0]
            };
            renderer.draw_text(entry.label, entry.x + 8.0, entry.y + 4.0, 8.0, color);
            renderer.draw_text(
                &format!("$ {}", entry.cost),
                entry.x + entry.w - 52.0,
                entry.y + 4.0,
                8.0,
                color,
            );
        }

        draw_td_minimap(
            renderer.as_mut(),
            &self.game,
            self.td_tileset.as_ref(),
            self.game.player_owner,
            self.camera.position,
            self.camera.viewport_width as f32 / zoom,
            self.camera.viewport_height as f32 / zoom,
            left,
            top,
            self.camera.viewport_width as f32,
            self.camera.viewport_height as f32,
        );

        if self.td_paused {
            renderer.begin_layer();
            renderer.set_camera(&screen_camera(self.width, self.height));
            let slot_label = save_slot_label(&Self::quick_save_path_for(self.save_slot));
            draw_td_pause(
                renderer.as_mut(),
                &self.game,
                self.td_pause_choice,
                self.save_slot,
                &slot_label,
                self.width,
                self.height,
            );
        }

        if let Err(error) = renderer.end_frame() {
            tracing::warn!(%error, "frame presentation failed");
        }

        self.frame_index = self.frame_index.wrapping_add(1);

        if self.kind == RendererKind::Software && !self.software_warning_emitted {
            tracing::warn!(
                "software renderer draws offscreen only; use --renderer wgpu to present frames"
            );
            self.software_warning_emitted = true;
        }
    }
}

impl ApplicationHandler for LauncherApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title("C&C Remastered - Rust Clone")
            .with_inner_size(LogicalSize::new(self.width, self.height));

        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                tracing::error!(%error, "failed to create window");
                event_loop.exit();
                return;
            }
        };

        let size = window.inner_size();
        self.width = size.width.max(1);
        self.height = size.height.max(1);
        self.camera.set_viewport(self.width, self.height);
        self.camera.position = self.game.world_center();

        let renderer = self.create_renderer(window.clone());
        self.renderer = Some(renderer);

        if let Some(renderer) = self.renderer.as_mut() {
            renderer.set_camera(&self.camera);
        }

        let assets_dir = std::env::var("CNC_ASSETS").unwrap_or_else(|_| "assets".to_string());
        let atlas = SpriteAtlas::load_or_generate(Path::new(&assets_dir), AtlasStyle::Neon);
        if let Some(renderer) = self.renderer.as_mut() {
            if renderer.uses_sprites() {
                // The pack's unit atlas supersedes the procedural neon atlas:
                // slot 0 uploads the pack's animation frames so unit sprites
                // sample real art. The procedural atlas stays as the fallback
                // (and for kinds the pack does not ship).
                let unit_atlas = cnc_asset::unit_atlas::UnitAtlas::from_fallback();
                let uploaded = match &unit_atlas {
                    Some(pack) => {
                        let (width, height) = pack.animation_dims();
                        match renderer.set_atlas(width, height, pack.animation_rgba()) {
                            Ok(()) => {
                                tracing::info!(
                                    atlas = %"unit_animation",
                                    width, height,
                                    units = pack.units().count(),
                                    "pack unit atlas ready"
                                );
                                true
                            }
                            Err(error) => {
                                tracing::warn!(%error, "failed to upload pack unit atlas");
                                false
                            }
                        }
                    }
                    None => false,
                };
                if !uploaded {
                    match renderer.set_atlas(atlas.width, atlas.height, &atlas.pixels) {
                        Ok(()) => {
                            tracing::info!(
                                frames = atlas.frames.len(),
                                dir = %assets_dir,
                                "sprite atlas ready"
                            );
                            self.sprite_atlas = Some(atlas);
                        }
                        Err(error) => tracing::warn!(%error, "failed to upload sprite atlas"),
                    }
                }
            }
        }

        // Terrain screens in this shell are TD; bind the TD pack once here so
        // the first frame's tileset sprites sample a resident texture.
        self.bind_tileset(TilesetPalette::Td);

        self.window = Some(window);
        self.last_frame = Instant::now();
        self.next_frame = Instant::now() + FRAME_INTERVAL;
        tracing::info!(
            "C&C Rust Clone started at {}x{} - mission '{}' with {} units and {} structures",
            self.width,
            self.height,
            self.game.mission.name,
            self.game.unit_count(),
            self.game.building_count()
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.td_complete {
            match &event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. }
                    if event.state == ElementState::Pressed =>
                {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) | Key::Named(NamedKey::Enter) => {
                            self.dismiss_td_campaign_complete();
                        }
                        _ => {}
                    }
                }
                WindowEvent::RedrawRequested => self.draw(),
                _ => {}
            }
            return;
        }
        if self.selecting_mission {
            match &event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. }
                    if event.state == ElementState::Pressed =>
                {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => self.return_to_menu = true,
                        Key::Named(NamedKey::ArrowUp) => self.cycle_mission(-1),
                        Key::Named(NamedKey::ArrowDown) => self.cycle_mission(1),
                        Key::Named(NamedKey::Enter) => {
                            if let Err(error) = self.activate_selected_mission() {
                                tracing::warn!(%error, "mission selection failed");
                            }
                        }
                        Key::Character(text) => match text.to_ascii_lowercase().as_str() {
                            "f" => self.cycle_td_filter(),
                            "s" => self.td_stats_open = true,
                            _ => {}
                        },
                        _ => {}
                    }
                }
                WindowEvent::RedrawRequested => self.draw(),
                _ => {}
            }
            return;
        }
        if self.td_stats_open {
            match &event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. }
                    if event.state == ElementState::Pressed =>
                {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) | Key::Named(NamedKey::Enter) => {
                            self.td_stats_open = false;
                        }
                        _ => {}
                    }
                }
                WindowEvent::RedrawRequested => self.draw(),
                _ => {}
            }
            return;
        }
        if self.td_briefing_open {
            match &event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. }
                    if event.state == ElementState::Pressed =>
                {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => self.return_to_menu = true,
                        Key::Named(NamedKey::Enter) => self.td_briefing_open = false,
                        _ => {}
                    }
                }
                WindowEvent::RedrawRequested => self.draw(),
                _ => {}
            }
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                let width = size.width.max(1);
                let height = size.height.max(1);
                self.width = width;
                self.height = height;
                self.camera.set_viewport(width, height);
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(width, height);
                    renderer.set_camera(&self.camera);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = Some(position);
                if self.drag_start.is_some() {
                    self.drag_current = Some(position);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                use winit::event::MouseScrollDelta;
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(position) => position.y as f32 / 40.0,
                };
                if amount.abs() > f32::EPSILON {
                    self.zoom_td(wheel_zoom_factor(amount));
                }
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.shift_down = modifiers.state().shift_key();
                self.ctrl_down = modifiers.state().control_key();
                self.alt_down = modifiers.state().alt_key();
            }
            WindowEvent::MouseInput { state, button, .. } => match (state, button) {
                (ElementState::Pressed, MouseButton::Left) => {
                    self.drag_start = self.cursor;
                    self.drag_current = self.cursor;
                }
                (ElementState::Released, MouseButton::Left) => {
                    let start = self.drag_start.take();
                    let end = self.drag_current.take();
                    if let (Some(start), Some(end)) = (start, end) {
                        let world = self.screen_to_world(end);
                        if self.td_minimap_click(world) {
                            return;
                        }
                        let start_world = self.screen_to_world(start);
                        let over_sidebar = !matches!(self.sidebar_hit(world), SidebarHit::Outside);
                        let started_in_sidebar =
                            !matches!(self.sidebar_hit(start_world), SidebarHit::Outside);

                        if over_sidebar {
                            self.handle_sidebar_click(world);
                        } else if started_in_sidebar {
                            // Drag started on the build menu: ignore.
                        } else if self
                            .game
                            .ready_building_for(self.game.player_owner)
                            .is_some()
                        {
                            let tile = cnc_tiberian_dawn::map::TILE_SIZE;
                            let cell = Vec2i::new(
                                (world.x.to_f32() / tile).floor() as i32,
                                (world.y.to_f32() / tile).floor() as i32,
                            );
                            let player = self.game.player_owner;
                            let placed = self.game.place_ready_building(player, cell);
                            tracing::debug!(placed, "place player structure");
                        } else {
                            let dx = end.x - start.x;
                            let dy = end.y - start.y;
                            if dx * dx + dy * dy > 36.0 {
                                let count =
                                    self.game
                                        .select_in_rect(start_world, world, self.shift_down);
                                self.selected_building = None;
                                tracing::debug!(count, additive = self.shift_down, "box selection");
                            } else {
                                let selected = self.game.select_at_additive(world, self.shift_down);
                                if selected.is_some() {
                                    self.selected_building = None;
                                } else {
                                    let tile = cnc_tiberian_dawn::map::TILE_SIZE;
                                    let cell = Vec2i::new(
                                        (world.x.to_f32() / tile).floor() as i32,
                                        (world.y.to_f32() / tile).floor() as i32,
                                    );
                                    let player = self.game.player_owner;
                                    self.selected_building = self
                                        .game
                                        .buildings
                                        .iter()
                                        .enumerate()
                                        .find(|(_, building)| {
                                            let building_cell = Vec2i::new(
                                                (building.position.x.to_f32() / tile).floor()
                                                    as i32,
                                                (building.position.y.to_f32() / tile).floor()
                                                    as i32,
                                            );
                                            building.owner == player
                                                && building.is_alive()
                                                && building_cell == cell
                                        })
                                        .map(|(index, _)| index);
                                }
                                tracing::debug!(
                                    ?selected,
                                    additive = self.shift_down,
                                    "click selection"
                                );
                            }
                        }
                    }
                }
                (ElementState::Pressed, MouseButton::Right) => {
                    if let Some(world) = self.cursor_world() {
                        if matches!(self.sidebar_hit(world), SidebarHit::Outside) {
                            if let Some(target) = self.game.enemy_at(world) {
                                let ordered = self.game.order_attack(target);
                                tracing::debug!(ordered, target, "right click attack order");
                            } else {
                                let ordered = self.game.order_move(world);
                                tracing::debug!(ordered, "right click move order");
                            }
                        }
                    }
                }
                _ => {}
            },
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if self.td_lobby_open {
                        match &event.logical_key {
                            Key::Named(NamedKey::Escape) => {
                                if self.td_lan_scan.take().is_none() {
                                    self.return_to_menu = true;
                                }
                            }
                            Key::Named(NamedKey::Enter) => {
                                let _ = self.start_td_lobby();
                            }
                            Key::Named(NamedKey::Tab) => {
                                self.td_lobby.focus = (self.td_lobby.focus + 1) % 2;
                            }
                            Key::Named(NamedKey::Backspace) => self.td_lobby.backspace(),
                            Key::Named(NamedKey::ArrowUp) => {
                                if let Some(scan) = self.td_lan_scan.as_mut() {
                                    if !scan.hosts.is_empty() {
                                        scan.selected = (scan.selected + scan.hosts.len() - 1)
                                            % scan.hosts.len();
                                    }
                                }
                            }
                            Key::Named(NamedKey::ArrowDown) => {
                                if let Some(scan) = self.td_lan_scan.as_mut() {
                                    if !scan.hosts.is_empty() {
                                        scan.selected = (scan.selected + 1) % scan.hosts.len();
                                    }
                                }
                            }
                            Key::Character(text) => match text.as_str() {
                                "r" | "R" => self.td_lobby.toggle_role(),
                                "l" | "L" => self.toggle_td_lan_scan(),
                                "d" | "D" => self.toggle_td_lobby_directory(),
                                "[" => self.td_lobby.adjust_port(-1),
                                "]" => self.td_lobby.adjust_port(1),
                                other => {
                                    for character in other.chars() {
                                        self.td_lobby.push_char(character);
                                    }
                                }
                            },
                            _ => {}
                        }
                        return;
                    }
                    if self.game.mission.outcome.is_some() {
                        match &event.logical_key {
                            Key::Named(NamedKey::Enter) => {
                                if self.td_score.is_none() {
                                    self.show_td_score();
                                } else {
                                    self.td_score = None;
                                    self.td_score_since = None;
                                    self.finish_td_mission();
                                }
                                return;
                            }
                            Key::Named(NamedKey::Escape) => {
                                event_loop.exit();
                                return;
                            }
                            _ => return,
                        }
                    }
                    let player = self.game.player_owner;
                    if self.td_paused {
                        match &event.logical_key {
                            Key::Named(NamedKey::Escape) => self.td_paused = false,
                            Key::Named(NamedKey::Enter) => self.activate_td_pause_choice(),
                            Key::Named(NamedKey::ArrowUp) => {
                                self.td_pause_choice = (self.td_pause_choice + 4) % 5;
                            }
                            Key::Named(NamedKey::ArrowDown) => {
                                self.td_pause_choice = (self.td_pause_choice + 1) % 5;
                            }
                            _ => {}
                        }
                        return;
                    }
                    if self.ctrl_down {
                        if let Key::Character(text) = &event.logical_key {
                            if let Some(slot) = bookmark_slot(text) {
                                if self.shift_down {
                                    self.save_td_bookmark(slot);
                                } else {
                                    self.jump_td_bookmark(slot);
                                }
                                return;
                            }
                        }
                    }
                    if self.alt_down {
                        if let Key::Character(text) = &event.logical_key {
                            if let Some(slot) = bookmark_slot(text) {
                                if self.shift_down {
                                    self.store_td_control_group(slot);
                                } else {
                                    self.recall_td_control_group(slot);
                                }
                                return;
                            }
                        }
                    }
                    if self.ctrl_down && self.td_network_active() {
                        if let Key::Character(text) = &event.logical_key {
                            if text.eq_ignore_ascii_case("r") {
                                // Ready-up hook: Ctrl+R toggles the local flag and
                                // broadcasts it, mirroring the RA lane's R key.
                                let local = self
                                    .network
                                    .as_ref()
                                    .map(|net| net.session.local_player)
                                    .unwrap_or(0);
                                let name = format!("Player{local}");
                                let ready = !self.ready_tracker.is_ready(&name);
                                self.send_td_ready(ready);
                                return;
                            }
                        }
                    }
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => {
                            if self.td_network_active() {
                                self.td_notice = Some((
                                    "Pause is disabled in network games".to_string(),
                                    Instant::now(),
                                ));
                            } else {
                                self.enter_td_pause();
                            }
                        }
                        Key::Named(NamedKey::F5) => {
                            if self.td_network_active() {
                                self.td_notice = Some((
                                    "Save is disabled in network games".to_string(),
                                    Instant::now(),
                                ));
                            } else {
                                self.quick_save();
                            }
                        }
                        Key::Named(NamedKey::F9) => {
                            if self.td_network_active() {
                                self.td_notice = Some((
                                    "Load is disabled in network games".to_string(),
                                    Instant::now(),
                                ));
                            } else {
                                self.quick_load();
                            }
                        }
                        Key::Character(text) if text == "+" || text == "=" => {
                            self.zoom_td(KEY_ZOOM_STEP);
                        }
                        Key::Character(text) if text == "-" || text == "_" => {
                            self.zoom_td(1.0 / KEY_ZOOM_STEP);
                        }
                        Key::Named(NamedKey::F6) => {
                            if self.td_network_active() {
                                self.td_notice = Some((
                                    "Game speed is locked in network games".to_string(),
                                    Instant::now(),
                                ));
                            } else {
                                self.adjust_td_speed(-0.25);
                            }
                        }
                        Key::Named(NamedKey::F7) => {
                            if self.td_network_active() {
                                self.td_notice = Some((
                                    "Game speed is locked in network games".to_string(),
                                    Instant::now(),
                                ));
                            } else {
                                self.adjust_td_speed(0.25);
                            }
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("x") => {
                            self.sell_selected_building();
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("r") => {
                            self.repair_selected_building();
                        }
                        Key::Named(NamedKey::Backspace) => {
                            self.cancel_td_production();
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("a") => {
                            let count = self.game.select_all_owned(player);
                            tracing::debug!(count, "select all GDI units");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("1") => {
                            let queued =
                                self.queue_td_buildable(Buildable::Unit(UnitType::InfantryE1));
                            tracing::debug!(queued, "queue GDI infantry");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("2") => {
                            let queued =
                                self.queue_td_buildable(Buildable::Unit(UnitType::MediumTank));
                            tracing::debug!(queued, "queue GDI medium tank");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("3") => {
                            let queued =
                                self.queue_td_buildable(Buildable::Unit(UnitType::MammothTank));
                            tracing::debug!(queued, "queue GDI mammoth tank");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("4") => {
                            let queued =
                                self.queue_td_buildable(Buildable::Unit(UnitType::Harvester));
                            tracing::debug!(queued, "queue GDI harvester");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("5") => {
                            let queued = self.queue_td_buildable(Buildable::Structure(
                                StructureType::PowerPlant,
                            ));
                            let status = self.game.status();
                            tracing::debug!(
                                queued,
                                tib = status.gdi_tiberium,
                                credits = status.gdi_credits,
                                "queue GDI power plant"
                            );
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("6") => {
                            let queued = self
                                .queue_td_buildable(Buildable::Structure(StructureType::Refinery));
                            tracing::debug!(queued, "queue GDI refinery");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("7") => {
                            let queued = self
                                .queue_td_buildable(Buildable::Structure(StructureType::Barracks));
                            tracing::debug!(queued, "queue GDI barracks");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("8") => {
                            let queued = self.queue_td_buildable(Buildable::Structure(
                                StructureType::WarFactory,
                            ));
                            tracing::debug!(queued, "queue GDI war factory");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("9") => {
                            let queued =
                                self.queue_td_buildable(Buildable::Structure(StructureType::Silo));
                            tracing::debug!(queued, "queue GDI silo");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("0") => {
                            let queued = self.queue_td_buildable(Buildable::Unit(UnitType::Orca));
                            tracing::debug!(queued, "queue GDI orca");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("i") => {
                            if let Some(world) = self.cursor_world() {
                                let tile = cnc_tiberian_dawn::map::TILE_SIZE;
                                let cell = Vec2i::new(
                                    (world.x.to_f32() / tile).floor() as i32,
                                    (world.y.to_f32() / tile).floor() as i32,
                                );
                                let fired = self.game.fire_ion_cannon(player, cell);
                                tracing::debug!(fired, "fire ion cannon");
                            }
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("n") => {
                            if let Some(world) = self.cursor_world() {
                                let tile = cnc_tiberian_dawn::map::TILE_SIZE;
                                let cell = Vec2i::new(
                                    (world.x.to_f32() / tile).floor() as i32,
                                    (world.y.to_f32() / tile).floor() as i32,
                                );
                                let fired = self.game.fire_nuke(player, cell);
                                tracing::debug!(fired, "fire nuke");
                            }
                        }
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => self.draw(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if now >= self.next_frame {
            self.next_frame = now + FRAME_INTERVAL;
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

/// Interactive Red Alert skirmish mode (the RA counterpart of LauncherApp).
///
/// It uses the same window/renderer plumbing but drives cnc-red-alert's Game:
/// ground/naval/air pathfinding, house production, Tesla defenses, harvesting
/// and the Chronosphere. The HUD is intentionally minimal; the point is to run
/// the full RA simulation loop on screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RaMode {
    Setup,
    NetworkLobby,
    CampaignSelect,
    CampaignStats,
    CampaignComplete,
    Playing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CampaignFilter {
    All,
    Incomplete,
    Completed,
}

impl CampaignFilter {
    fn next(self) -> Self {
        match self {
            Self::All => Self::Incomplete,
            Self::Incomplete => Self::Completed,
            Self::Completed => Self::All,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Incomplete => "Incomplete",
            Self::Completed => "Completed",
        }
    }
}

#[derive(Clone, Debug)]
struct CampaignEntry {
    path: PathBuf,
    name: String,
    briefing: String,
    player: usize,
    width: u32,
    height: u32,
    terrain: Vec<u8>,
    ore: Vec<u32>,
    objectives: Vec<String>,
    triggers: usize,
}

const RA_MAP_SIZES: [u32; 3] = [48, 64, 96];
const ALLIED_COUNTRIES: [Country; 3] = [Country::England, Country::Germany, Country::France];
const SOVIET_COUNTRIES: [Country; 2] = [Country::Russia, Country::Ukraine];
const DIFFICULTY_NAMES: [&str; 3] = ["Easy", "Normal", "Hard"];

#[derive(Clone, Copy, Debug)]
struct SkirmishSetup {
    map_size: u32,
    side: usize,
    country: Country,
    difficulty: usize,
}

/// Host/join choice and addresses shown by the network lobby screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NetRole {
    Host,
    Join,
}

#[derive(Clone, Debug)]
struct NetLobby {
    role: NetRole,
    host_port: u16,
    join_port: u16,
    peer_address: String,
    join_address: String,
    focus: usize,
}

impl Default for NetLobby {
    fn default() -> Self {
        Self {
            role: NetRole::Host,
            host_port: 47870,
            join_port: 47871,
            peer_address: format!("127.0.0.1:{}", 47871),
            join_address: format!("127.0.0.1:{}", 47870),
            focus: 0,
        }
    }
}

impl NetLobby {
    /// Tiberian Dawn defaults; different ports from Red Alert so both lobbies
    /// can run on one machine.
    fn td_default() -> Self {
        Self {
            role: NetRole::Host,
            host_port: 47872,
            join_port: 47873,
            peer_address: format!("127.0.0.1:{}", 47873),
            join_address: format!("127.0.0.1:{}", 47872),
            focus: 0,
        }
    }

    fn role_label(&self) -> &'static str {
        match self.role {
            NetRole::Host => "Host",
            NetRole::Join => "Join",
        }
    }

    fn toggle_role(&mut self) {
        self.role = match self.role {
            NetRole::Host => NetRole::Join,
            NetRole::Join => NetRole::Host,
        };
        self.focus = 0;
    }

    fn active_port(&mut self) -> &mut u16 {
        match self.role {
            NetRole::Host => &mut self.host_port,
            NetRole::Join => &mut self.join_port,
        }
    }

    fn adjust_port(&mut self, delta: i32) {
        let port = self.active_port();
        let next = (*port as i32 + delta).clamp(1024, 65535);
        *port = next as u16;
    }

    fn active_address(&mut self) -> &mut String {
        match self.role {
            NetRole::Host => &mut self.peer_address,
            NetRole::Join => &mut self.join_address,
        }
    }

    fn push_char(&mut self, character: char) {
        if character.is_ascii_digit() || character == '.' || character == ':' {
            let address = self.active_address();
            if address.len() < 32 {
                address.push(character);
            }
        }
    }

    fn backspace(&mut self) {
        self.active_address().pop();
    }
}

impl Default for SkirmishSetup {
    fn default() -> Self {
        Self {
            map_size: 64,
            side: 0,
            country: Country::England,
            difficulty: 1,
        }
    }
}

/// CLI-driven RA network setup.
#[derive(Clone, Debug)]
struct NetworkSetup {
    role: String,
    join: Option<String>,
    port: u16,
    peers: Vec<String>,
    bind: String,
    advertised: Option<String>,
}

/// A peer that has been silent this long is reported lost (host migration).
const PEER_TIMEOUT: Duration = Duration::from_secs(8);

/// Text chat prefix; chat lines travel outside the lockstep packet stream.
const CHAT_PREFIX: &str = "CNCCHAT 1";

/// Parse `CNCCHAT 1 <player> <text>`.
fn parse_chat_line(bytes: &[u8]) -> Option<(u8, String)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let rest = text.strip_prefix(CHAT_PREFIX)?.trim_start();
    let (player, message) = rest.split_once(' ')?;
    let message = message.trim();
    if message.is_empty() {
        return None;
    }
    Some((player.parse().ok()?, message.to_string()))
}

/// Join peers announce their player id so a host can register a fresh or
/// reconnecting address even before any command is sent.
const JOIN_HANDSHAKE_PREFIX: &str = "CNCRAJOIN 1";

/// Well-known UDP port used for host beacons (does not carry commands).
const LAN_DISCOVERY_PORT: u16 = 47890;
const LAN_BEACON_PREFIX: &str = "CNCRALAN 1";

/// One host heard on the LAN.
#[derive(Clone, Debug, PartialEq, Eq)]
struct LanHost {
    address: String,
    label: String,
}

/// Passive listener for host beacons while the lobby scans the LAN.
struct LanScan {
    socket: UdpSocket,
    hosts: Vec<LanHost>,
    selected: usize,
}

impl LanScan {
    fn ingest(&mut self, source: SocketAddr, bytes: &[u8]) {
        let Some((port, name)) = parse_lan_beacon(bytes) else {
            return;
        };
        let address = format!("{}:{}", source.ip(), port);
        if self.hosts.iter().any(|host| host.address == address) {
            return;
        }
        self.hosts.push(LanHost {
            label: format!("{name}  {address}"),
            address,
        });
        self.hosts
            .sort_by(|left, right| left.address.cmp(&right.address));
        self.hosts.truncate(8);
        if self.selected >= self.hosts.len() {
            self.selected = self.hosts.len().saturating_sub(1);
        }
    }
}

/// Start or stop listening for host beacons on the LAN discovery port.
fn toggle_lan_scan(scan: &mut Option<LanScan>, error: &mut Option<String>) {
    if scan.is_some() {
        *scan = None;
        return;
    }
    match UdpSocket::bind(("0.0.0.0", LAN_DISCOVERY_PORT)) {
        Ok(socket) => match socket.set_nonblocking(true) {
            Ok(()) => {
                *scan = Some(LanScan {
                    socket,
                    hosts: Vec::new(),
                    selected: 0,
                });
                *error = None;
            }
            Err(err) => {
                *error = Some(format!("LAN scan setup failed: {err}"));
            }
        },
        Err(err) => {
            *error = Some(format!(
                "LAN scan could not bind UDP {LAN_DISCOVERY_PORT}: {err}"
            ));
        }
    }
}

/// Format the ready datagram for one peer; shared by both poll loops.
///
/// Mirrors `send_chat`: `CNCREADY 1 <player> <0|1>\n` goes out to every known
/// peer address and never enters the lockstep stream.
fn ready_datagram(player: &str, ready: bool) -> String {
    format_ready_line(player, ready)
}

/// Broadcast the local ready flag to `peers`, mirroring `send_chat`.
///
/// Returns how many datagrams the OS accepted, so the poll loops and their
/// tests can assert the hook really sent something.
fn broadcast_ready(socket: &UdpSocket, player: &str, ready: bool, peers: &[SocketAddr]) -> usize {
    let line = ready_datagram(player, ready);
    peers
        .iter()
        .filter(|peer| socket.send_to(line.as_bytes(), **peer).is_ok())
        .count()
}

/// Server address the lobby's direct-connect row dials.
///
/// The lobby screen keeps a manual address for exactly this: when no
/// directory answers, the value the player typed is the join target.
fn lobby_direct_address(screen: &LobbyScreen) -> String {
    screen.manual_address.trim().to_string()
}

/// Run a lobby refresh on the directory backends from the app's key handler.
///
/// `LobbyScreen::refresh` is the only caller of the blocking discovery path,
/// and `refresh_lobby` already caps it with `DEFAULT_DIRECTORY_TIMEOUT`, so a
/// dead directory costs one frame budget at most and never panics.
fn refresh_lobby_screen(screen: &mut LobbyScreen) {
    screen.refresh();
}

/// Collect every beacon that arrived since the last poll.
fn poll_lan_scan_socket(scan: &mut Option<LanScan>) {
    let Some(scan) = scan.as_mut() else {
        return;
    };
    let mut buffer = [0u8; 512];
    loop {
        match scan.socket.recv_from(&mut buffer) {
            Ok((size, source)) => scan.ingest(source, &buffer[..size]),
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(_) => break,
        }
    }
}

/// Parse `CNCRAJOIN 1 <player>`; returns the claimed player id.
fn parse_join_handshake(bytes: &[u8]) -> Option<u8> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut parts = text.split_whitespace();
    if parts.next()? != "CNCRAJOIN" || parts.next()? != "1" {
        return None;
    }
    parts.next()?.parse().ok()
}

/// Register a peer from its join handshake, reviving a previously lost slot.
fn register_join_peer(net: &mut RaNetwork, source: SocketAddr, player: u8) {
    if player >= net.session.players || player == net.session.local_player {
        return;
    }
    net.session.register_peer(player, source);
    net.last_seen.insert(player, Instant::now());
    let was_lost = net.reported.remove(&player);
    if was_lost || !net.session.migration.alive.contains(&player) {
        net.session.migration.alive.insert(player);
        // A reconnecting peer restarts its sequence numbers; stale relay
        // hashes would otherwise suppress its first packets.
        net.session.relay.clear();
        if was_lost {
            net.rejoined += 1;
            tracing::info!(player, "network peer reconnected");
        }
    }
}

/// Beacon text a host broadcasts on the LAN discovery port.
fn lan_beacon_text(port: u16, game: &str) -> String {
    format!("{LAN_BEACON_PREFIX} {port} {game}")
}

/// Parse `CNCRALAN 1 <game_port> [name]`; malformed beacons are ignored.
fn parse_lan_beacon(bytes: &[u8]) -> Option<(u16, String)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut parts = text.split_whitespace();
    if parts.next()? != "CNCRALAN" || parts.next()? != "1" {
        return None;
    }
    let port: u16 = parts.next()?.parse().ok()?;
    let name = parts.next().unwrap_or("RA").to_string();
    Some((port, name))
}

/// Playback state for a recorded network match.
struct ReplayState {
    commands: Vec<NetCommand>,
    next: usize,
    frame: u32,
    last_tick: Instant,
    paused: bool,
    done: bool,
}

/// Live UDP lockstep session attached to the RA game view.
struct RaNetwork {
    socket: UdpSocket,
    session: LiveSession,
    last_tick: Instant,
    sent: u64,
    applied: u64,
    seen: HashSet<u64>,
    role: String,
    last_beacon: Instant,
    last_handshake: Instant,
    last_seen: HashMap<u8, Instant>,
    reported: HashSet<u8>,
    migrations: u64,
    losses: u64,
    rejoined: u64,
    replay_commands: Vec<NetCommand>,
}

/// Apply one remote lockstep move to the RA game (shared by polling and tests).
fn apply_net_move(game: &mut RaGame, player: u8, units: &[u32], target: Vec2) {
    game.apply_command(&GameCommand::Move {
        owner: player as usize,
        units: units.to_vec(),
        target,
    });
}

/// Deploy a remote player's MCV.
fn apply_net_deploy(game: &mut RaGame, player: u8, unit: u32) -> bool {
    game.apply_command(&GameCommand::Deploy {
        owner: player as usize,
        unit,
    }) > 0
}

/// Sell a remote player's structure.
fn apply_net_sell(game: &mut RaGame, player: u8, building: u32) -> bool {
    game.apply_command(&GameCommand::Sell {
        owner: player as usize,
        building,
    }) > 0
}

/// Cancel a remote player's front production item.
fn apply_net_cancel(game: &mut RaGame, player: u8) -> bool {
    game.apply_command(&GameCommand::CancelProduction {
        owner: player as usize,
    }) > 0
}

/// Apply one network/replay command to the game; true when it had an effect.
fn apply_net_command(game: &mut RaGame, command: &NetCommand) -> bool {
    match &command.command {
        NetCommandKind::Move { units, target } => {
            apply_net_move(game, command.player, units, *target);
            true
        }
        NetCommandKind::Build { building, location } => {
            apply_net_build(game, command.player, *building, *location)
        }
        NetCommandKind::Produce { unit_type, .. } => {
            apply_net_produce(game, command.player, *unit_type)
        }
        NetCommandKind::Attack { units, target } => {
            apply_net_attack(game, command.player, units, *target) > 0
        }
        NetCommandKind::Sell { building } => apply_net_sell(game, command.player, *building),
        NetCommandKind::Deploy { unit } => apply_net_deploy(game, command.player, *unit),
        NetCommandKind::Stop { units } => apply_net_stop(game, command.player, units) > 0,
        NetCommandKind::RepairStructure { building } => {
            apply_net_repair(game, command.player, *building)
        }
        NetCommandKind::Power { building, on } => {
            apply_net_power(game, command.player, *building, *on)
        }
        NetCommandKind::CancelProduction => apply_net_cancel(game, command.player),
        _ => false,
    }
}

/// Toggle a remote player's structure power.
fn apply_net_power(game: &mut RaGame, player: u8, building: u32, on: bool) -> bool {
    game.apply_command(&GameCommand::Power {
        owner: player as usize,
        building,
        on,
    }) > 0
}

/// Repair a remote player's structure.
fn apply_net_repair(game: &mut RaGame, player: u8, building: u32) -> bool {
    game.apply_command(&GameCommand::Repair {
        owner: player as usize,
        building,
    }) > 0
}

/// Stop a remote player's listed units.
fn apply_net_stop(game: &mut RaGame, player: u8, units: &[u32]) -> usize {
    game.apply_command(&GameCommand::Stop {
        owner: player as usize,
        units: units.to_vec(),
    })
}

/// Order a remote player's units to attack a shared object id.
fn apply_net_attack(game: &mut RaGame, player: u8, units: &[u32], target: u32) -> usize {
    game.apply_command(&GameCommand::Attack {
        owner: player as usize,
        units: units.to_vec(),
        target,
    })
}

/// Queue a remote player's unit production.
fn apply_net_produce(game: &mut RaGame, player: u8, class_id: cnc_core::ClassId) -> bool {
    let seconds = game.unit_build_time_of(class_id);
    game.apply_command(&GameCommand::Produce {
        owner: player as usize,
        class_id,
        seconds,
    }) > 0
}

/// Place a remote player's structure; returns true when the placement stuck.
fn apply_net_build(
    game: &mut RaGame,
    player: u8,
    class_id: cnc_core::ClassId,
    location: Vec2,
) -> bool {
    let cell = Vec2i::new(
        (location.x.to_f32() / RA_TILE_SIZE).floor() as i32,
        (location.y.to_f32() / RA_TILE_SIZE).floor() as i32,
    );
    game.place_structure(player as usize, class_id, cell)
        .is_ok()
}

/// Discriminator used by TD Produce commands to mark structure production.
const TD_PRODUCE_STRUCTURE: u32 = u32::MAX;

/// Encode a TD sidebar choice as a lockstep Produce command.
fn td_produce_command(buildable: Buildable) -> NetCommandKind {
    match buildable {
        Buildable::Unit(unit) => {
            let index = UnitType::ALL
                .iter()
                .position(|candidate| *candidate == unit)
                .unwrap_or(0) as u32;
            NetCommandKind::Produce {
                factory: index,
                unit_type: buildable.class_id(),
            }
        }
        Buildable::Structure(structure) => NetCommandKind::Produce {
            factory: TD_PRODUCE_STRUCTURE,
            unit_type: structure.class_id(),
        },
    }
}

/// Decode a TD Produce command back into a sidebar choice.
fn td_produce_target(factory: u32, class_id: cnc_core::ClassId) -> Option<Buildable> {
    if factory == TD_PRODUCE_STRUCTURE {
        StructureType::from_class_id(class_id).map(Buildable::Structure)
    } else {
        UnitType::ALL
            .get(factory as usize)
            .copied()
            .map(Buildable::Unit)
    }
}

/// Apply one network/replay command to the TD game; true when it had an effect.
fn apply_td_command(game: &mut Game, command: &NetCommand) -> bool {
    let player = command.player;
    match &command.command {
        NetCommandKind::Move { units, target } => game.order_units_move(player, units, *target) > 0,
        NetCommandKind::Attack { units, target } => {
            game.order_units_attack(player, units, *target as usize) > 0
        }
        NetCommandKind::Stop { units } => game.order_units_stop(player, units) > 0,
        NetCommandKind::Produce { factory, unit_type } => {
            match td_produce_target(*factory, *unit_type) {
                Some(Buildable::Unit(unit)) => game.queue_unit(player, unit),
                Some(Buildable::Structure(structure)) => game.queue_building(player, structure),
                None => false,
            }
        }
        NetCommandKind::PlaceBuilding { location } => {
            let tile = cnc_tiberian_dawn::map::TILE_SIZE;
            let cell = Vec2i::new(
                (location.x.to_f32() / tile).floor() as i32,
                (location.y.to_f32() / tile).floor() as i32,
            );
            game.place_ready_building(player, cell)
        }
        NetCommandKind::Sell { building } => {
            game.sell_building(player, *building as usize).is_some()
        }
        NetCommandKind::RepairStructure { building } => {
            game.repair_building(player, *building as usize).is_some()
        }
        NetCommandKind::CancelProduction => game.cancel_production(player).is_some(),
        _ => false,
    }
}

/// Live UDP lockstep session attached to the TD game view.
struct TdNetwork {
    socket: UdpSocket,
    session: LiveSession,
    last_tick: Instant,
    sent: u64,
    applied: u64,
    seen: HashSet<u64>,
    role: String,
    last_beacon: Instant,
}

pub struct RaLauncherApp {
    kind: RendererKind,
    width: u32,
    height: u32,
    // Renderer must drop before the window: on Wayland the wgpu surface must
    // not outlive the native window.
    renderer: Option<Box<dyn Renderer>>,
    window: Option<Arc<Window>>,
    pub camera: Camera,
    pub game: RaGame,
    /// RTS 16×16 tileset packs, one per palette (RA screens use `ra_inspired`,
    /// TD screens `td_inspired`, honoring `CNC_TILESET_DIR` /
    /// `CNC_TILESET_PALETTE`); None falls back to the flat terrain colours.
    ra_tileset: Option<TilesetPack>,
    td_tileset: Option<TilesetPack>,
    /// Palette currently resident in the renderer's tileset texture slot
    /// (1 = RA, 2 = TD); None until the first successful upload, keeping
    /// uploads rare (screen context changes, never per frame).
    tileset_slot: Option<u8>,
    /// In-game SFX director (todo 42): routes engine events to the mixer.
    audio_director: audio_director::AudioDirector,
    /// Shared audio mixer (todo 42): headless fallback when no device exists.
    mixer: AudioMixer,
    /// Theme queue (todo 42): explore/combat layers driven by house state.
    theme_queue: audio_director::ThemeQueue,
    /// Engine event consumer feeding the director; created before the first
    /// tick so no pre-subscription events are missed.
    event_subscriber: events_subscriber::EventSubscriber,
    scenario_name: String,
    mode: RaMode,
    setup: SkirmishSetup,
    lobby: NetLobby,
    lobby_error: Option<String>,
    lan_scan: Option<LanScan>,
    /// Directory-backed lobby rows (LAN multicast / directory browse), with
    /// the direct-connect fallback folded in when no directory answers.
    lobby_screen: LobbyScreen,
    notice: Option<(String, Instant)>,
    campaign: Vec<CampaignEntry>,
    campaign_index: usize,
    campaign_active: bool,
    campaign_progress_path: PathBuf,
    campaign_completed: Vec<bool>,
    campaign_best: Vec<Option<i32>>,
    campaign_difficulty: usize,
    campaign_filter: CampaignFilter,
    replay_prompt: bool,
    briefing_open: bool,
    paused: bool,
    pause_choice: usize,
    /// Score screen state for end-of-mission scoreboard (RA).
    ra_score: Option<score::ScoreReport>,
    /// When the score overlay was shown (for animation).
    ra_score_since: Option<Instant>,
    active_scenario_path: Option<PathBuf>,
    network: Option<RaNetwork>,
    replay: Option<ReplayState>,
    net_replay_path: Option<PathBuf>,
    pending_commands: Vec<GameCommand>,
    placing: Option<cnc_core::ClassId>,
    selected_structure: Option<u32>,
    save_slot: usize,
    shift_down: bool,
    ctrl_down: bool,
    alt_down: bool,
    bookmarks: [Option<Vec2>; 4],
    control_groups: [Vec<u32>; 4],
    game_speed: f32,
    chat_open: bool,
    chat_input: String,
    chat_log: Vec<(u8, String)>,
    ready_tracker: ReadyTracker,
    cursor: Option<winit::dpi::PhysicalPosition<f64>>,
    selection_start: Option<winit::dpi::PhysicalPosition<f64>>,
    panning: bool,
    last_pointer: (f64, f64),
    frame_index: u64,
    frame_limit: u64,
    last_frame: Instant,
    next_frame: Instant,
    software_warning_emitted: bool,
    /// Set when a menu-owned screen wants the shell to reopen the main menu.
    return_to_menu: bool,
}

impl RaLauncherApp {
    pub fn new(kind: RendererKind, width: u32, height: u32, frame_limit: u64) -> Self {
        let game = RaGame::new(MAP_SIZE, MAP_SIZE);
        // Attach the event bus before anything ticks so the audio director
        // sees every engine event (todo 42). A missing device falls back to
        // a silent headless mixer: sim unaffected either way.
        let event_bus = std::sync::Arc::new(cnc_events::EventBus::new(4096));
        let mut game = game;
        game.set_event_bus(event_bus.clone());
        let mut camera = Camera::default();
        camera.set_viewport(width.max(1), height.max(1));
        camera.position = Vec2::new(
            Fixed16_16::from_f32(game.map.width as f32 * RA_TILE_SIZE * 0.5),
            Fixed16_16::from_f32(game.map.height as f32 * RA_TILE_SIZE * 0.5),
        );
        Self {
            kind,
            width,
            height,
            window: None,
            renderer: None,
            camera,
            game,
            audio_director: audio_director::AudioDirector::new(),
            mixer: AudioMixer::headless(),
            theme_queue: audio_director::ThemeQueue::new(),
            event_subscriber: events_subscriber::EventSubscriber::new(event_bus),
            ra_tileset: cnc_asset::tileset_pack::load_default("ra_inspired"),
            td_tileset: cnc_asset::tileset_pack::load_default("td_inspired"),
            tileset_slot: None,
            scenario_name: "Skirmish".to_string(),
            mode: RaMode::Setup,
            setup: SkirmishSetup::default(),
            lobby: NetLobby::default(),
            lobby_error: None,
            lan_scan: None,
            lobby_screen: LobbyScreen::default(),
            notice: None,
            campaign: Vec::new(),
            campaign_index: 0,
            campaign_active: false,
            campaign_progress_path: default_campaign_progress_path(),
            campaign_completed: Vec::new(),
            campaign_best: Vec::new(),
            campaign_difficulty: 1,
            campaign_filter: CampaignFilter::All,
            replay_prompt: false,
            briefing_open: false,
            paused: false,
            pause_choice: 0,
            ra_score: None,
            ra_score_since: None,
            active_scenario_path: None,
            network: None,
            replay: None,
            net_replay_path: None,
            pending_commands: Vec::new(),
            placing: None,
            selected_structure: None,
            save_slot: 0,
            shift_down: false,
            ctrl_down: false,
            alt_down: false,
            bookmarks: [None; 4],
            control_groups: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            game_speed: 1.0,
            chat_open: false,
            chat_input: String::new(),
            chat_log: Vec::new(),
            ready_tracker: ReadyTracker::new(),
            cursor: None,
            selection_start: None,
            panning: false,
            last_pointer: (0.0, 0.0),
            frame_index: 0,
            frame_limit,
            last_frame: Instant::now(),
            next_frame: Instant::now() + FRAME_INTERVAL,
            software_warning_emitted: false,
            return_to_menu: false,
        }
    }

    fn screen_to_world(&self, position: winit::dpi::PhysicalPosition<f64>) -> Vec2 {
        self.camera.screen_to_world(Vec2::new(
            Fixed16_16::from_f32(position.x as f32),
            Fixed16_16::from_f32(position.y as f32),
        ))
    }

    fn cursor_world(&self) -> Option<Vec2> {
        self.cursor.map(|position| self.screen_to_world(position))
    }

    /// Map a world point from the zoomed game view into HUD space (the HUD is
    /// drawn unzoomed, so this keeps clicks aligned with the minimap).
    fn hud_world(&self, world: Vec2) -> Vec2 {
        let position = self.camera.position;
        let zoom = self.camera.zoom.to_f32().max(1.0e-6);
        Vec2::new(
            Fixed16_16::from_f32(
                position.x.to_f32() + (world.x.to_f32() - position.x.to_f32()) * zoom,
            ),
            Fixed16_16::from_f32(
                position.y.to_f32() + (world.y.to_f32() - position.y.to_f32()) * zoom,
            ),
        )
    }

    /// Zoom the RA world view by factor, anchored at the cursor.
    fn zoom_ra(&mut self, factor: f32) {
        let world_w = self.game.map.width as f32 * RA_TILE_SIZE;
        let world_h = self.game.map.height as f32 * RA_TILE_SIZE;
        let anchor = self.cursor.map(|cursor| (cursor.x as f32, cursor.y as f32));
        zoom_camera_at(&mut self.camera, anchor, factor, world_w, world_h);
    }

    fn load_campaign_progress(&self) -> (usize, usize, Vec<usize>, Vec<(usize, i32)>) {
        let mut mission = 0usize;
        let mut difficulty = 1usize;
        let mut completed = Vec::new();
        let mut best = Vec::new();
        if let Ok(text) = std::fs::read_to_string(&self.campaign_progress_path) {
            for line in text.lines() {
                let Some((key, value)) = line.split_once('=') else {
                    continue;
                };
                match key.trim().to_ascii_lowercase().as_str() {
                    "mission" => mission = value.trim().parse().unwrap_or(0),
                    "difficulty" => difficulty = value.trim().parse().unwrap_or(1),
                    "completed" => {
                        completed = value
                            .split(',')
                            .filter_map(|index| index.trim().parse::<usize>().ok())
                            .collect();
                    }
                    "besttimes" => {
                        best = value
                            .split(',')
                            .filter_map(|entry| {
                                let (index, seconds) = entry.trim().split_once(':')?;
                                Some((
                                    index.trim().parse::<usize>().ok()?,
                                    seconds.trim().parse::<i32>().ok()?,
                                ))
                            })
                            .collect();
                    }
                    _ => {}
                }
            }
        }
        (mission, difficulty.min(2), completed, best)
    }

    fn save_campaign_progress(&self, index: usize) {
        if let Some(parent) = self.campaign_progress_path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        let completed: Vec<String> = self
            .campaign_completed
            .iter()
            .enumerate()
            .filter(|(_, completed)| **completed)
            .map(|(index, _)| index.to_string())
            .collect();
        let best: Vec<String> = self
            .campaign_best
            .iter()
            .enumerate()
            .filter_map(|(index, seconds)| seconds.map(|seconds| format!("{index}:{seconds}")))
            .collect();
        if let Err(error) = std::fs::write(
            &self.campaign_progress_path,
            format!(
                "[Campaign]\nMission={index}\nDifficulty={}\nCompleted={}\nBestTimes={}\n",
                self.campaign_difficulty,
                completed.join(","),
                best.join(",")
            ),
        ) {
            tracing::warn!(%error, "failed to save campaign progress");
        }
    }

    /// Populate the campaign list from every scenario INI in a directory.
    fn load_campaign(&mut self, dir: &Path) -> usize {
        self.campaign.clear();
        let Ok(entries) = std::fs::read_dir(dir) else {
            tracing::warn!(dir = %dir.display(), "campaign directory not found");
            self.mode = RaMode::Setup;
            return 0;
        };
        let mut paths: Vec<PathBuf> = entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
            })
            .collect();
        paths.sort();
        for path in paths {
            match cnc_red_alert::Scenario::from_file(&path) {
                Ok(scenario) => self.campaign.push(CampaignEntry {
                    path,
                    width: scenario.width,
                    height: scenario.height,
                    terrain: scenario.terrain,
                    ore: scenario.ore,
                    objectives: scenario.objectives,
                    triggers: scenario.triggers.len(),
                    name: scenario.name,
                    briefing: scenario.briefing,
                    player: scenario.player,
                }),
                Err(error) => {
                    tracing::warn!(path = %path.display(), %error, "skipping invalid mission");
                }
            }
        }
        if self.campaign.is_empty() {
            tracing::warn!(dir = %dir.display(), "campaign has no missions");
            self.mode = RaMode::Setup;
        } else {
            self.mode = RaMode::CampaignSelect;
            let (mission, difficulty, completed, best) = self.load_campaign_progress();
            self.campaign_index = mission.min(self.campaign.len() - 1);
            self.campaign_difficulty = difficulty.min(DIFFICULTY_NAMES.len() - 1);
            self.campaign_completed = vec![false; self.campaign.len()];
            for index in completed {
                if let Some(slot) = self.campaign_completed.get_mut(index) {
                    *slot = true;
                }
            }
            self.campaign_best = vec![None; self.campaign.len()];
            for (index, seconds) in best {
                if let Some(slot) = self.campaign_best.get_mut(index) {
                    *slot = Some(seconds);
                }
            }
            self.campaign_active = true;
            tracing::info!(
                mission = self.campaign_index + 1,
                total = self.campaign.len(),
                difficulty = DIFFICULTY_NAMES[self.campaign_difficulty],
                completed = self.campaign_completed.iter().filter(|done| **done).count(),
                "campaign progress loaded"
            );
        }
        self.campaign.len()
    }

    /// Mission indices matching the active completion filter.
    fn campaign_visible(&self) -> Vec<usize> {
        (0..self.campaign.len())
            .filter(|index| match self.campaign_filter {
                CampaignFilter::All => true,
                CampaignFilter::Incomplete => !self
                    .campaign_completed
                    .get(*index)
                    .copied()
                    .unwrap_or(false),
                CampaignFilter::Completed => self
                    .campaign_completed
                    .get(*index)
                    .copied()
                    .unwrap_or(false),
            })
            .collect()
    }

    fn cycle_campaign(&mut self, delta: i32) {
        let visible = self.campaign_visible();
        if visible.is_empty() {
            return;
        }
        let position = visible
            .iter()
            .position(|index| *index == self.campaign_index)
            .unwrap_or(0);
        let len = visible.len() as i32;
        let next = (((position as i32 + delta) % len) + len) % len;
        self.campaign_index = visible[next as usize];
    }

    fn cycle_campaign_filter(&mut self) {
        self.campaign_filter = self.campaign_filter.next();
        let visible = self.campaign_visible();
        if !visible.is_empty() && !visible.contains(&self.campaign_index) {
            self.campaign_index = visible[0];
        }
    }

    fn show_campaign_stats(&mut self) {
        self.mode = RaMode::CampaignStats;
    }

    fn dismiss_campaign_stats(&mut self) {
        self.mode = RaMode::CampaignSelect;
    }

    fn start_campaign_mission(&mut self, index: usize) -> bool {
        let Some(entry) = self.campaign.get(index).cloned() else {
            return false;
        };
        match cnc_red_alert::Scenario::from_file(&entry.path) {
            Ok(scenario) => {
                tracing::info!(
                    mission = %scenario.name,
                    path = %entry.path.display(),
                    "campaign mission loaded"
                );
                self.game = RaGame::from_scenario(&scenario);
                let (queue, attack) = self.campaign_tempo();
                self.game.set_ai_tempo(queue, attack);
                self.scenario_name = scenario.name;
                self.campaign_index = index;
                self.campaign_active = true;
                self.active_scenario_path = Some(entry.path);
                self.mode = RaMode::Playing;
                self.placing = None;
                self.briefing_open = !scenario.briefing.is_empty();
                self.paused = false;
                self.save_campaign_progress(index);
                self.center_camera();
                true
            }
            Err(error) => {
                tracing::warn!(path = %entry.path.display(), %error, "failed to load mission");
                false
            }
        }
    }

    /// Mark the current mission complete and advance (or finish the campaign).
    fn advance_campaign(&mut self) {
        if !self.campaign_active || !self.outcome_is_victory() {
            return;
        }
        if let Some(slot) = self.campaign_completed.get_mut(self.campaign_index) {
            *slot = true;
        }
        let time = self.game.elapsed.int();
        if let Some(slot) = self.campaign_best.get_mut(self.campaign_index) {
            if slot.is_none_or(|best| time < best) {
                *slot = Some(time);
            }
        }
        self.save_campaign_progress(self.campaign_index);
        if let Some(next) = next_incomplete(&self.campaign_completed, self.campaign_index) {
            self.start_campaign_mission(next);
        } else {
            tracing::info!("campaign complete");
            self.mode = RaMode::CampaignComplete;
        }
    }

    fn dismiss_campaign_complete(&mut self) {
        self.campaign_index = 0;
        self.mode = RaMode::CampaignSelect;
    }

    fn request_campaign_mission(&mut self, index: usize) {
        self.campaign_index = index;
        if self.campaign_completed.get(index).copied().unwrap_or(false) {
            self.replay_prompt = true;
        } else {
            self.start_campaign_mission(index);
        }
    }

    fn confirm_replay(&mut self) {
        self.replay_prompt = false;
        self.start_campaign_mission(self.campaign_index);
    }

    fn cancel_replay(&mut self) {
        self.replay_prompt = false;
    }

    fn campaign_tempo(&self) -> (i32, i32) {
        match self.campaign_difficulty {
            0 => (18, 40),
            2 => (7, 15),
            _ => (12, 25),
        }
    }

    /// Apply queued gameplay commands (the seam a network session will feed).
    fn drain_pending_commands(&mut self) -> usize {
        let commands: Vec<GameCommand> = std::mem::take(&mut self.pending_commands);
        let count = commands.len();
        for command in commands {
            self.game.apply_command(&command);
        }
        count
    }

    /// Start a two-player RA skirmish wired to a UDP lockstep session.
    fn enable_network(&mut self, setup: &NetworkSetup) -> Result<()> {
        let config = NetworkConfig {
            bind_address: setup.bind.clone(),
            port: setup.port,
            advertised: setup.advertised.clone(),
            max_players: 2,
        };
        config.validate().map_err(|error| anyhow!(error))?;
        let local_player: u8 = match setup.role.as_str() {
            "host" => 0,
            "join" => 1,
            other => return Err(anyhow!("unknown --net-role '{}' (host|join)", other)),
        };
        let bind = format!("{}:{}", setup.bind.trim(), setup.port);
        let socket = UdpSocket::bind(&bind).with_context(|| format!("failed to bind {bind}"))?;
        socket
            .set_nonblocking(true)
            .context("failed to make the network socket non-blocking")?;
        // Hosts advertise themselves to the LAN lobby via broadcast beacons.
        let _ = socket.set_broadcast(true);
        let local_address = socket.local_addr().ok();
        let mut session = LiveSession::new(local_player, 2, local_address)
            .map_err(|error| anyhow!("network session setup failed: {error:?}"))?;
        if local_player == 0 {
            for (index, peer) in setup
                .peers
                .iter()
                .filter(|peer| !peer.trim().is_empty())
                .enumerate()
            {
                let address = peer
                    .parse::<SocketAddr>()
                    .with_context(|| format!("invalid --net-peer '{peer}'"))?;
                session.register_peer((1 + index) as u8, address);
            }
        } else {
            let host = setup
                .join
                .clone()
                .ok_or_else(|| anyhow!("--net-role join requires --net-join HOST:PORT"))?;
            let address = host
                .parse::<SocketAddr>()
                .with_context(|| format!("invalid --net-join '{host}'"))?;
            session.register_peer(0, address);
            for (index, peer) in setup
                .peers
                .iter()
                .filter(|peer| !peer.trim().is_empty())
                .enumerate()
            {
                let address = peer
                    .parse::<SocketAddr>()
                    .with_context(|| format!("invalid --net-peer '{peer}'"))?;
                session.register_peer((2 + index) as u8, address);
            }
        }
        if local_player == 1 {
            if let Some(host) = session.peers.get(&0).copied() {
                let text = format!("{JOIN_HANDSHAKE_PREFIX} {local_player}\n");
                let _ = socket.send_to(text.as_bytes(), host);
            }
        }

        self.start_skirmish();
        self.campaign_active = false;
        self.game.set_player_owner(local_player as usize);
        self.game.disable_ai();
        self.scenario_name = format!("Network {}", setup.role);
        println!(
            "net role={} local_player={} addr={:?} peers={}",
            setup.role,
            local_player,
            local_address,
            session.peers.len()
        );
        let last_seen = session
            .peers
            .keys()
            .map(|player| (*player, Instant::now()))
            .collect();
        self.network = Some(RaNetwork {
            socket,
            session,
            last_tick: Instant::now(),
            sent: 0,
            applied: 0,
            seen: HashSet::new(),
            role: setup.role.clone(),
            last_beacon: Instant::now(),
            last_handshake: Instant::now(),
            last_seen,
            reported: HashSet::new(),
            migrations: 0,
            losses: 0,
            rejoined: 0,
            replay_commands: Vec::new(),
        });
        Ok(())
    }

    /// Start replaying a recorded network match.
    fn start_replay(&mut self, replay: cnc_tiberian_dawn::network::Replay) {
        self.start_skirmish();
        self.game.disable_ai();
        self.game.set_player_owner(0);
        self.mode = RaMode::Playing;
        self.briefing_open = false;
        self.paused = false;
        self.scenario_name = format!("Replay ({} commands)", replay.commands.len());
        self.center_camera();
        self.replay = Some(ReplayState {
            commands: replay.commands,
            next: 0,
            frame: 0,
            last_tick: Instant::now(),
            paused: false,
            done: false,
        });
    }

    /// Apply every command due in the next replay frame.
    fn step_replay_frame(&mut self) -> usize {
        let Some(state) = self.replay.as_mut() else {
            return 0;
        };
        let frame = state.frame;
        state.frame = state.frame.wrapping_add(1);
        let mut applied = 0;
        while let Some(command) = state.commands.get(state.next).cloned() {
            if command.frame > frame {
                break;
            }
            state.next += 1;
            if apply_net_command(&mut self.game, &command) {
                applied += 1;
            }
        }
        state.done = state.next >= state.commands.len();
        applied
    }

    /// Advance playback on the same 250 ms cadence used for live packets.
    fn poll_replay(&mut self) {
        let due = self.replay.as_ref().is_some_and(|state| {
            !state.paused && !state.done && state.last_tick.elapsed() >= Duration::from_millis(250)
        });
        if due {
            if let Some(state) = self.replay.as_mut() {
                state.last_tick = Instant::now();
            }
            self.step_replay_frame();
        }
    }

    /// Open the host/join lobby from the skirmish setup screen.
    fn open_lobby(&mut self) {
        self.mode = RaMode::NetworkLobby;
        self.lobby_error = None;
        self.lan_scan = None;
        self.lobby.focus = 0;
        self.lobby_screen = LobbyScreen::new(&self.lobby.join_address);
        self.refresh_lobby_directory();
    }

    /// Refresh the RA lobby's directory rows (LAN multicast or directory browse).
    fn refresh_lobby_directory(&mut self) {
        refresh_lobby_screen(&mut self.lobby_screen);
    }

    /// Switch the RA lobby between the directory and LAN-multicast backends.
    ///
    /// A directory outage is a warning, not an error: `used_fallback` rows let
    /// the player direct-connect, so the lobby stays usable.
    fn toggle_lobby_directory(&mut self) {
        self.lobby_screen.toggle_mode();
        let scan = &self.lobby_screen.scan;
        if scan.used_fallback() || !scan.has_directory_rows() {
            if self.lobby_error.is_none() {
                self.lobby_error = Some(scan.status_line());
            }
        } else {
            self.lobby_error = None;
        }
    }

    /// The direct-connect address the RA lobby will dial.
    fn lobby_direct_connect(&self) -> String {
        lobby_direct_address(&self.lobby_screen)
    }

    /// Start or stop listening for host beacons on the LAN discovery port.
    fn toggle_lan_scan(&mut self) {
        toggle_lan_scan(&mut self.lan_scan, &mut self.lobby_error);
    }

    /// Collect every beacon that arrived since the last poll.
    fn poll_lan_scan(&mut self) {
        poll_lan_scan_socket(&mut self.lan_scan);
    }

    /// Start the match current lobby settings describe (called from the UI).
    fn start_lobby(&mut self) -> Result<()> {
        if self.lobby.role == NetRole::Host
            && self.ready_tracker.expected_count() > 0
            && !self.ready_tracker.all_ready()
        {
            self.lobby_error = Some("Waiting for all players to ready up".to_string());
            return Ok(());
        }
        let role = self.lobby.role;
        let setup = match role {
            NetRole::Host => NetworkSetup {
                role: "host".to_string(),
                join: None,
                port: self.lobby.host_port,
                peers: vec![self.lobby.peer_address.trim().to_string()],
                bind: "0.0.0.0".to_string(),
                advertised: None,
            },
            NetRole::Join => NetworkSetup {
                role: "join".to_string(),
                join: Some(if self.lobby.join_address.trim().is_empty() {
                    // Directory fallback: the manual address row is the target.
                    self.lobby_direct_connect()
                } else {
                    self.lobby.join_address.trim().to_string()
                }),
                port: self.lobby.join_port,
                peers: Vec::new(),
                bind: "0.0.0.0".to_string(),
                advertised: None,
            },
        };
        self.lobby_error = None;
        match self.enable_network(&setup) {
            Ok(()) => {
                self.lan_scan = None;
                self.mode = RaMode::Playing;
                let players = self.network.as_ref().map(|net| net.session.players);
                if let Some(players) = players {
                    self.ready_tracker.set_expected(players as usize);
                }
                Ok(())
            }
            Err(error) => {
                self.lobby_error = Some(error.to_string());
                Ok(())
            }
        }
    }

    /// Drain incoming datagrams and advance/emit one lockstep frame on a timer.
    fn poll_network(&mut self) {
        let Some(net) = self.network.as_mut() else {
            return;
        };
        let mut buffer = [0u8; 2048];
        loop {
            match net.socket.recv_from(&mut buffer) {
                Ok((size, source)) => {
                    if let Some((player, ready)) = parse_ready_line(&buffer[..size]) {
                        let _ = self.ready_tracker.mark_ready(&player, ready);
                        continue;
                    }
                    if let Some((player, message)) = parse_chat_line(&buffer[..size]) {
                        self.chat_log.push((player, message));
                        if self.chat_log.len() > 20 {
                            self.chat_log.remove(0);
                        }
                        continue;
                    }
                    if let Some(player) = parse_join_handshake(&buffer[..size]) {
                        let before = net.rejoined;
                        register_join_peer(net, source, player);
                        if net.rejoined > before {
                            self.notice =
                                Some((format!("Peer {player} reconnected"), Instant::now()));
                        }
                        continue;
                    }
                    let event = net.session.handle_datagram(source, &buffer[..size]);
                    // A host that was started without --net-peer learns the peer
                    // from its first packet; a lost peer may also come back from
                    // a new port, in which case the packet's player id wins.
                    let mut seen_player = net
                        .session
                        .peers
                        .iter()
                        .find(|(_, address)| **address == source)
                        .map(|(player, _)| *player);
                    if seen_player.is_none() {
                        // Decode directly (not via the relay event) so a packet
                        // whose bytes were seen before a reconnect still counts.
                        let packet = NetPacket::decode(&buffer[..size])
                            .ok()
                            .filter(NetPacket::verify);
                        if let Some(packet) = packet {
                            let local = net.session.local_player;
                            let players = net.session.players;
                            let claimed = packet
                                .commands
                                .iter()
                                .map(|command| command.player)
                                .find(|player| *player != local && *player < players);
                            let rejoining = claimed.and_then(|player| {
                                (net.reported.contains(&player)
                                    || !net.session.migration.alive.contains(&player))
                                .then_some(player)
                            });
                            let player = rejoining.or_else(|| {
                                let taken: Vec<u8> = net.session.peers.keys().copied().collect();
                                (0..players)
                                    .find(|player| *player != local && !taken.contains(player))
                            });
                            if let Some(player) = player {
                                net.session.register_peer(player, source);
                                seen_player = Some(player);
                            }
                        }
                    }
                    if let Some(player) = seen_player {
                        net.last_seen.insert(player, Instant::now());
                        let was_lost = net.reported.remove(&player);
                        if was_lost || !net.session.migration.alive.contains(&player) {
                            net.session.migration.alive.insert(player);
                            // A reconnecting peer restarts its sequence numbers;
                            // drop stale relay hashes so its new packets pass.
                            net.session.relay.clear();
                            if was_lost {
                                net.rejoined += 1;
                                self.notice =
                                    Some((format!("Peer {player} reconnected"), Instant::now()));
                                tracing::info!(player, "network peer reconnected");
                            }
                        }
                    }
                    match event {
                        LiveEvent::Commands(packet) => {
                            for command in &packet.commands {
                                // Mesh relay/echoes can deliver the same command
                                // twice; apply each command exactly once.
                                if !net.seen.insert(command.hash()) {
                                    continue;
                                }
                                if apply_net_command(&mut self.game, command) {
                                    net.applied += 1;
                                }
                            }
                        }
                        LiveEvent::Migration(message) => {
                            net.migrations += 1;
                            self.notice =
                                Some(("Network host migration".to_string(), Instant::now()));
                            tracing::info!(?message, "network host migration");
                            if let Some((claim, targets)) = net.session.maybe_claim() {
                                let bytes = claim.encode();
                                for target in targets {
                                    let _ = net.socket.send_to(&bytes, target);
                                }
                                net.migrations += 1;
                            }
                        }
                        LiveEvent::Duplicate | LiveEvent::Invalid => {}
                    }
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(error) => {
                    tracing::warn!(%error, "network receive failed");
                    break;
                }
            }
        }

        let now = Instant::now();
        let lost: Vec<u8> = net
            .last_seen
            .iter()
            .filter(|(player, seen)| {
                **player != net.session.local_player
                    && now.duration_since(**seen) > PEER_TIMEOUT
                    && !net.reported.contains(player)
            })
            .map(|(player, _)| *player)
            .collect();
        for player in lost {
            net.losses += 1;
            net.reported.insert(player);
            self.notice = Some((
                format!("Peer {player} lost - election started"),
                Instant::now(),
            ));
            if let Some(message) = net.session.report_loss(player) {
                net.migrations += 1;
                let bytes = message.encode();
                for target in net.session.peers.values().copied() {
                    let _ = net.socket.send_to(&bytes, target);
                }
                tracing::info!(player, "network peer timed out; migration started");
                if let Some((claim, targets)) = net.session.maybe_claim() {
                    let bytes = claim.encode();
                    for target in targets {
                        let _ = net.socket.send_to(&bytes, target);
                    }
                    net.migrations += 1;
                }
            } else {
                tracing::warn!(player, "network peer timed out");
            }
        }

        if net.role == "join" && net.last_handshake.elapsed() >= Duration::from_secs(2) {
            net.last_handshake = Instant::now();
            if let Some(host) = net.session.peers.get(&0).copied() {
                let text = format!("{JOIN_HANDSHAKE_PREFIX} {}\n", net.session.local_player);
                let _ = net.socket.send_to(text.as_bytes(), host);
            }
        }

        if net.role == "host" && net.last_beacon.elapsed() >= Duration::from_secs(2) {
            net.last_beacon = Instant::now();
            if let Ok(local) = net.socket.local_addr() {
                let beacon = lan_beacon_text(local.port(), "RA");
                // Limited broadcast reaches the LAN; the loopback subnet
                // broadcast also reaches a lobby on the same machine.
                for address in [
                    std::net::Ipv4Addr::BROADCAST,
                    std::net::Ipv4Addr::new(127, 255, 255, 255),
                ] {
                    let _ = net
                        .socket
                        .send_to(beacon.as_bytes(), (address, LAN_DISCOVERY_PORT));
                }
            }
        }

        if net.last_tick.elapsed() < Duration::from_millis(250) {
            return;
        }
        net.last_tick = Instant::now();
        match net.session.advance() {
            Ok((commands, bytes, targets)) => {
                net.replay_commands.extend(commands.iter().cloned());
                for target in targets {
                    if net.socket.send_to(&bytes, target).is_ok() {
                        net.sent += 1;
                    }
                }
            }
            Err(error) => tracing::warn!(?error, "network packet encoding failed"),
        }
    }

    /// Queue a unit locally and broadcast the same order in a network game.
    fn produce_unit(&mut self, class_id: cnc_core::ClassId) {
        let player = self.game.player_owner;
        let seconds = self.game.unit_build_time_of(class_id);
        self.pending_commands.push(GameCommand::Produce {
            owner: player,
            class_id,
            seconds,
        });
        if let Some(net) = self.network.as_mut() {
            // RA derives the build time from the class, so factory stays unused.
            net.session.queue_local(NetCommandKind::Produce {
                factory: 0,
                unit_type: class_id,
            });
        }
    }

    /// Cancel the front production item (locally and over the network).
    fn cancel_production(&mut self) {
        if let Some(net) = self.network.as_mut() {
            net.session.queue_local(NetCommandKind::CancelProduction);
        }
        self.pending_commands.push(GameCommand::CancelProduction {
            owner: self.game.player_owner,
        });
        self.notice = Some(("Production cancelled".to_string(), Instant::now()));
    }

    /// Scatter the selected units to deterministic nearby cells.
    fn scatter_selected_units(&mut self) {
        let player = self.game.player_owner;
        let ids: Vec<u32> = self
            .game
            .units
            .iter()
            .filter(|unit| unit.selected && unit.owner == player && unit.is_alive())
            .map(|unit| unit.id)
            .collect();
        if ids.is_empty() {
            return;
        }
        let targets = self.game.scatter_targets(player, &ids);
        let count = targets.len();
        for (id, target) in targets {
            if let Some(net) = self.network.as_mut() {
                net.session.queue_local(NetCommandKind::Move {
                    units: vec![id],
                    target,
                });
            }
            self.pending_commands.push(GameCommand::Move {
                owner: player,
                units: vec![id],
                target,
            });
        }
        self.notice = Some((format!("Scattered {count} units"), Instant::now()));
        tracing::debug!(count, "RA scatter order queued");
    }

    /// Toggle power on the selected structure (locally and over the network).
    fn toggle_selected_power(&mut self) {
        let Some(id) = self.selected_structure else {
            return;
        };
        let player = self.game.player_owner;
        let Some(current) = self
            .game
            .structures
            .iter()
            .find(|structure| structure.id == id && structure.owner == player)
            .map(|structure| structure.powered)
        else {
            return;
        };
        let on = !current;
        if let Some(net) = self.network.as_mut() {
            net.session
                .queue_local(NetCommandKind::Power { building: id, on });
        }
        self.pending_commands.push(GameCommand::Power {
            owner: player,
            building: id,
            on,
        });
        self.notice = Some((
            format!("Structure power {}", if on { "on" } else { "off" }),
            Instant::now(),
        ));
    }

    /// Repair the selected structure (locally and over the network).
    fn repair_selected_structure(&mut self) {
        let Some(id) = self.selected_structure else {
            return;
        };
        let player = self.game.player_owner;
        let valid = self.game.structures.iter().any(|structure| {
            structure.id == id
                && structure.owner == player
                && structure.is_alive()
                && structure.health < structure.max_health
        });
        if !valid {
            self.notice = Some(("No repair needed".to_string(), Instant::now()));
            return;
        }
        if let Some(net) = self.network.as_mut() {
            net.session
                .queue_local(NetCommandKind::RepairStructure { building: id });
        }
        self.pending_commands.push(GameCommand::Repair {
            owner: player,
            building: id,
        });
        self.notice = Some(("Repair queued".to_string(), Instant::now()));
        tracing::info!(building = id, "RA structure repair queued");
    }

    /// Stop the selected units (locally and over the network).
    fn stop_selected_units(&mut self) {
        let player = self.game.player_owner;
        let units: Vec<u32> = self
            .game
            .units
            .iter()
            .filter(|unit| unit.selected && unit.owner == player && unit.is_alive())
            .map(|unit| unit.id)
            .collect();
        if units.is_empty() {
            return;
        }
        if let Some(net) = self.network.as_mut() {
            net.session.queue_local(NetCommandKind::Stop {
                units: units.clone(),
            });
        }
        self.pending_commands.push(GameCommand::Stop {
            owner: player,
            units,
        });
        tracing::debug!("RA stop order queued");
    }

    /// Deploy the selected MCV (locally and over the network).
    fn deploy_selected_mcv(&mut self) {
        let player = self.game.player_owner;
        let Some(id) = self
            .game
            .units
            .iter()
            .find(|unit| {
                unit.selected
                    && unit.owner == player
                    && unit.is_alive()
                    && unit.class_id == cnc_core::ClassId::VehicleE7
            })
            .map(|unit| unit.id)
        else {
            return;
        };
        if let Some(net) = self.network.as_mut() {
            net.session.queue_local(NetCommandKind::Deploy { unit: id });
        }
        self.pending_commands.push(GameCommand::Deploy {
            owner: player,
            unit: id,
        });
        tracing::info!(unit = id, "RA MCV deploy queued");
    }

    /// Sell the structure selected by a left click (locally and over the network).
    fn sell_selected_structure(&mut self) {
        let Some(id) = self.selected_structure.take() else {
            return;
        };
        let player = self.game.player_owner;
        let valid = self.game.structures.iter().any(|structure| {
            structure.id == id && structure.owner == player && structure.is_alive()
        });
        if !valid {
            return;
        }
        if let Some(net) = self.network.as_mut() {
            net.session
                .queue_local(NetCommandKind::Sell { building: id });
        }
        self.pending_commands.push(GameCommand::Sell {
            owner: player,
            building: id,
        });
        tracing::info!(id, "RA structure sold");
    }

    /// True once when a screen asks the shell to return to the main menu.
    fn take_return_to_menu(&mut self) -> bool {
        std::mem::take(&mut self.return_to_menu)
    }

    fn enter_pause(&mut self) {
        self.paused = true;
        self.pause_choice = 0;
    }

    fn activate_pause_choice(&mut self) {
        match self.pause_choice {
            0 => self.paused = false,
            1 => self.restart_mission(),
            2 => self.abort_mission(),
            3 => self.save_game(),
            4 => {
                if self.load_game() {
                    self.paused = false;
                }
            }
            5 => self.cycle_save_slot(),
            _ => {}
        }
    }

    /// Cycle through the three RA save slots.
    fn cycle_save_slot(&mut self) {
        self.save_slot = (self.save_slot + 1) % 3;
        self.notice = Some((
            format!("Save slot {}/3", self.save_slot + 1),
            Instant::now(),
        ));
    }

    /// Save the network replay to an explicit path; false without a session.
    fn save_network_replay_to(&mut self, path: &Path) -> bool {
        let Some(net) = self.network.as_ref() else {
            return false;
        };
        let replay = cnc_tiberian_dawn::network::Replay::new(
            net.session.players,
            net.replay_commands.clone(),
        );
        match replay.save(path) {
            Ok(()) => {
                self.notice = Some((
                    format!("Replay saved to {}", path.display()),
                    Instant::now(),
                ));
                tracing::info!(path = %path.display(), "network replay saved");
                true
            }
            Err(error) => {
                self.notice = Some((format!("Replay save failed: {error}"), Instant::now()));
                false
            }
        }
    }

    /// Save the network replay to the configured or default path.
    fn save_network_replay(&mut self) {
        let path = self
            .net_replay_path
            .clone()
            .unwrap_or_else(default_network_replay_path);
        self.save_network_replay_to(&path);
    }

    /// Save the current RA game (F5 / pause menu).
    fn save_game(&mut self) {
        let path = ra_save_path_for(self.save_slot);
        self.save_game_to(&path);
    }

    /// Save to an explicit path; returns true on success.
    fn save_game_to(&mut self, path: &Path) -> bool {
        match self.game.save_to_file(path) {
            Ok(()) => {
                self.notice = Some(("Game saved".to_string(), Instant::now()));
                tracing::info!(path = %path.display(), "RA mission saved");
                true
            }
            Err(error) => {
                self.notice = Some((format!("Save failed: {error}"), Instant::now()));
                tracing::warn!(%error, "RA mission save failed");
                false
            }
        }
    }

    /// Load the saved RA game; returns true when the load succeeded.
    fn load_game(&mut self) -> bool {
        let path = ra_save_path_for(self.save_slot);
        self.load_game_from(&path)
    }

    /// Load from an explicit path; returns true on success.
    fn load_game_from(&mut self, path: &Path) -> bool {
        match RaGame::load_from_file(path) {
            Ok(game) => {
                self.game = game;
                self.mode = RaMode::Playing;
                self.briefing_open = false;
                self.placing = None;
                self.center_camera();
                self.notice = Some(("Game loaded".to_string(), Instant::now()));
                tracing::info!(path = %path.display(), "RA mission loaded");
                true
            }
            Err(error) => {
                self.notice = Some((format!("Load failed: {error}"), Instant::now()));
                tracing::warn!(%error, "RA mission load failed");
                false
            }
        }
    }

    fn restart_mission(&mut self) {
        self.paused = false;
        if self.network.is_some() {
            // The UDP session cannot be re-created from a single-player restart.
            return;
        }
        if let Some(path) = self.active_scenario_path.clone() {
            if let Ok(scenario) = cnc_red_alert::Scenario::from_file(&path) {
                self.game = RaGame::from_scenario(&scenario);
                if self.campaign_active {
                    let (queue, attack) = self.campaign_tempo();
                    self.game.set_ai_tempo(queue, attack);
                }
                self.scenario_name = scenario.name;
                self.mode = RaMode::Playing;
                self.briefing_open = false;
                self.placing = None;
                self.center_camera();
                return;
            }
        }
        self.start_skirmish();
    }

    fn abort_mission(&mut self) {
        self.paused = false;
        self.placing = None;
        if self.network.is_some() {
            self.save_network_replay();
        }
        if self.network.take().is_some() {
            self.scenario_name = "Skirmish".to_string();
            self.mode = RaMode::Setup;
            return;
        }
        if self.campaign_active {
            self.mode = RaMode::CampaignSelect;
        } else {
            self.mode = RaMode::Setup;
        }
    }

    fn cycle_campaign_difficulty(&mut self) {
        self.campaign_difficulty = (self.campaign_difficulty + 1) % DIFFICULTY_NAMES.len();
        self.save_campaign_progress(self.campaign_index);
    }

    fn outcome_is_victory(&self) -> bool {
        let victory = if self.game.player_owner == 0 {
            cnc_red_alert::ScenarioOutcome::AlliesWin
        } else {
            cnc_red_alert::ScenarioOutcome::SovietsWin
        };
        self.game.outcome == Some(victory)
    }
    fn show_ra_score(&mut self) {
        let won = self.outcome_is_victory();
        // Use the RA game's elapsed time and zero stats for now (stats from todo 44)
        self.ra_score = Some(score::compute_score(
            0, // harvested_credits - not yet tracked
            0, // units_produced - not yet tracked
            0, // projectiles_fired - not yet tracked
            self.game.elapsed.int().max(0) as u64,
            won,
            score::DIFFICULTY_NEUTRAL_BP,
        ));
        self.ra_score_since = Some(Instant::now());
    }

    pub fn start_skirmish(&mut self) {
        self.game_speed = 1.0;
        let size = self.setup.map_size;
        let mut game = RaGame::new(size, size);
        game.set_player_owner(self.setup.side);
        let player_house = self.setup.side;
        if let Some(house) = game.houses.get_mut(player_house) {
            house.country = Some(self.setup.country);
            house.faction = self.setup.country.faction();
        }
        let (queue, attack) = match self.setup.difficulty {
            0 => (18, 40),
            2 => (7, 15),
            _ => (12, 25),
        };
        game.set_ai_tempo(queue, attack);
        self.game = game;
        self.scenario_name = format!(
            "Skirmish {} ({})",
            self.setup.country.label(),
            DIFFICULTY_NAMES[self.setup.difficulty]
        );
        self.mode = RaMode::Playing;
        self.placing = None;
        self.briefing_open = false;
        self.paused = false;
        self.active_scenario_path = None;
        self.center_camera();
        tracing::info!(
            map = %format!("{}x{}", size, size),
            country = self.setup.country.label(),
            difficulty = DIFFICULTY_NAMES[self.setup.difficulty],
            "Red Alert skirmish started"
        );
    }

    fn center_camera(&mut self) {
        self.camera
            .set_viewport(self.width.max(1), self.height.max(1));
        self.camera.zoom = Fixed16_16::from_f32(DEFAULT_GAME_ZOOM);
        self.camera.position = Vec2::new(
            Fixed16_16::from_f32(self.game.map.width as f32 * RA_TILE_SIZE * 0.5),
            Fixed16_16::from_f32(self.game.map.height as f32 * RA_TILE_SIZE * 0.5),
        );
    }

    fn cycle_map_size(&mut self, forward: bool) {
        let index = RA_MAP_SIZES
            .iter()
            .position(|size| *size == self.setup.map_size)
            .unwrap_or(1);
        let next = if forward {
            (index + 1) % RA_MAP_SIZES.len()
        } else {
            (index + RA_MAP_SIZES.len() - 1) % RA_MAP_SIZES.len()
        };
        self.setup.map_size = RA_MAP_SIZES[next];
    }

    fn cycle_country(&mut self) {
        let countries = if self.setup.side == 0 {
            &ALLIED_COUNTRIES[..]
        } else {
            &SOVIET_COUNTRIES[..]
        };
        let index = countries
            .iter()
            .position(|country| *country == self.setup.country)
            .unwrap_or(0);
        self.setup.country = countries[(index + 1) % countries.len()];
    }

    fn cycle_side(&mut self) {
        self.setup.side ^= 1;
        self.setup.country = if self.setup.side == 0 {
            ALLIED_COUNTRIES[0]
        } else {
            SOVIET_COUNTRIES[0]
        };
    }

    fn cycle_difficulty(&mut self) {
        self.setup.difficulty = (self.setup.difficulty + 1) % DIFFICULTY_NAMES.len();
    }

    /// Enemy unit or structure under the cursor (right-click attack target).
    fn enemy_target_at(&self, world: Vec2, player: usize) -> Option<u32> {
        const RADIUS: f32 = 18.0;
        let mut best: Option<(u32, f32)> = None;
        for unit in &self.game.units {
            if unit.owner == player || !unit.is_alive() {
                continue;
            }
            let distance = unit.position.distance(world).to_f32();
            if distance <= RADIUS && best.is_none_or(|(_, current)| distance < current) {
                best = Some((unit.id, distance));
            }
        }
        for structure in &self.game.structures {
            if structure.owner == player || !structure.is_alive() {
                continue;
            }
            let distance = structure.position.distance(world).to_f32();
            if distance <= RADIUS && best.is_none_or(|(_, current)| distance < current) {
                best = Some((structure.id, distance));
            }
        }
        best.map(|(id, _)| id)
    }

    /// Center the RA camera when a click lands on the minimap.
    fn ra_minimap_click(&mut self, world: Vec2) -> bool {
        let world = self.hud_world(world);
        let view_w = self.camera.viewport_width as f32;
        let view_h = self.camera.viewport_height as f32;
        let left = self.camera.position.x.to_f32() - view_w * 0.5;
        let top = self.camera.position.y.to_f32() - view_h * 0.5;
        let Some(cell) = minimap_cell_at(
            world,
            self.game.map.width,
            self.game.map.height,
            left,
            top,
            view_w,
            view_h,
            true,
        ) else {
            return false;
        };
        self.camera.position = Vec2::new(
            Fixed16_16::from_f32((cell.x as f32 + 0.5) * RA_TILE_SIZE),
            Fixed16_16::from_f32((cell.y as f32 + 0.5) * RA_TILE_SIZE),
        );
        true
    }

    /// Send the chat input to every known peer and log it locally.
    fn send_chat(&mut self) {
        let message = self.chat_input.trim().to_string();
        self.chat_input.clear();
        self.chat_open = false;
        if message.is_empty() {
            return;
        }
        let Some(net) = self.network.as_mut() else {
            return;
        };
        let local = net.session.local_player;
        self.chat_log.push((local, message.clone()));
        if self.chat_log.len() > 20 {
            self.chat_log.remove(0);
        }
        let line = format!("{CHAT_PREFIX} {local} {message}\n");
        for peer in net.session.peers.values().copied() {
            let _ = net.socket.send_to(line.as_bytes(), peer);
        }
    }

    /// Broadcast the local player's ready flag to every known peer and record
    /// it locally, mirroring `send_chat`. The local row keeps the roster in
    /// sync for the player toggling ready; peers key on the same
    /// `Player<index>` name so the receiver's `mark_ready` lines up with the
    /// roster rows and the start gate.
    fn send_ready(&mut self, ready: bool) {
        let Some(net) = self.network.as_mut() else {
            return;
        };
        let local = net.session.local_player;
        let name = format!("Player{local}");
        let _ = self.ready_tracker.mark_ready(&name, ready);
        let peers: Vec<SocketAddr> = net.session.peers.values().copied().collect();
        broadcast_ready(&net.socket, &name, ready, &peers);
    }

    /// Pan the RA camera while the cursor rests near a window edge.
    fn apply_ra_edge_pan(&mut self, delta: f32) {
        let Some(cursor) = self.cursor else {
            return;
        };
        let (dx, dy) = edge_pan_direction(
            cursor.x as f32,
            cursor.y as f32,
            self.width as f32,
            self.height as f32,
            8.0,
        );
        if dx == 0.0 && dy == 0.0 {
            return;
        }
        let zoom = self.camera.zoom.to_f32().max(1.0e-6);
        let step = 520.0 * delta / zoom;
        let max_x = self.game.map.width as f32 * RA_TILE_SIZE;
        let max_y = self.game.map.height as f32 * RA_TILE_SIZE;
        self.camera.position = Vec2::new(
            Fixed16_16::from_f32((self.camera.position.x.to_f32() + dx * step).clamp(0.0, max_x)),
            Fixed16_16::from_f32((self.camera.position.y.to_f32() + dy * step).clamp(0.0, max_y)),
        );
    }

    /// Adjust RA simulation speed; locked in network games for lockstep.
    fn adjust_ra_speed(&mut self, delta: f32) {
        if self.network.is_some() {
            self.notice = Some((
                "Speed is locked in network games".to_string(),
                Instant::now(),
            ));
            return;
        }
        self.game_speed = (self.game_speed + delta).clamp(0.25, 3.0);
        self.notice = Some((
            format!("Game speed {:.2}x", self.game_speed),
            Instant::now(),
        ));
    }

    /// Store the selected RA units in a control group.
    fn store_control_group(&mut self, slot: usize) {
        let player = self.game.player_owner;
        let ids: Vec<u32> = self
            .game
            .units
            .iter()
            .filter(|unit| unit.selected && unit.owner == player && unit.is_alive())
            .map(|unit| unit.id)
            .collect();
        if let Some(group) = self.control_groups.get_mut(slot) {
            *group = ids;
            self.notice = Some((format!("Group {} stored", slot + 1), Instant::now()));
        }
    }

    /// Recall a stored RA control group; false when the slot is empty.
    fn recall_control_group(&mut self, slot: usize) -> bool {
        let Some(ids) = self.control_groups.get(slot).cloned() else {
            return false;
        };
        if ids.is_empty() {
            return false;
        }
        let player = self.game.player_owner;
        for unit in &mut self.game.units {
            unit.selected = unit.owner == player && unit.is_alive() && ids.contains(&unit.id);
        }
        self.notice = Some((format!("Group {} recalled", slot + 1), Instant::now()));
        true
    }

    /// Store the current RA camera view in a bookmark slot.
    fn save_ra_bookmark(&mut self, slot: usize) {
        if let Some(bookmark) = self.bookmarks.get_mut(slot) {
            *bookmark = Some(self.camera.position);
            self.notice = Some((format!("View {} saved", slot + 1), Instant::now()));
        }
    }

    /// Jump to a stored RA camera bookmark; false when the slot is empty.
    fn jump_ra_bookmark(&mut self, slot: usize) -> bool {
        if let Some(Some(position)) = self.bookmarks.get(slot).copied() {
            self.camera.position = position;
            true
        } else {
            false
        }
    }

    fn cursor_cell(&self) -> Option<Vec2i> {
        self.cursor_world().map(|world| {
            Vec2i::new(
                (world.x.to_f32() / RA_TILE_SIZE).floor() as i32,
                (world.y.to_f32() / RA_TILE_SIZE).floor() as i32,
            )
        })
    }

    /// Bind the tileset pack a screen needs to the renderer's tileset texture
    /// slot. Uploads are tagged and rare: the slot keeps its contents across
    /// frames and is only (re)uploaded when the active palette changes, so the
    /// RA shell uploads once per renderer lifetime — the same upload-once
    /// pattern as the SpriteAtlas in the TD shell's `resumed`.
    fn bind_tileset(&mut self, palette: TilesetPalette) {
        let tag = match palette {
            TilesetPalette::Ra => 1,
            TilesetPalette::Td => 2,
        };
        if self.tileset_slot == Some(tag) {
            return;
        }
        let pack = match palette {
            TilesetPalette::Ra => self.ra_tileset.as_ref(),
            TilesetPalette::Td => self.td_tileset.as_ref(),
        };
        let Some(pack) = pack else {
            return;
        };
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };
        if !renderer.uses_sprites() {
            // Sprites never draw on this backend; the draw paths keep flat
            // colours, so there is nothing to upload.
            return;
        }
        match renderer.set_atlas_texture(
            cnc_renderer::TILESET_TEXTURE_ID,
            pack.width(),
            pack.height(),
            pack.rgba(),
        ) {
            Ok(()) => {
                tracing::info!(palette = pack.palette(), "terrain tileset atlas ready");
                self.tileset_slot = Some(tag);
            }
            Err(error) => tracing::warn!(%error, "failed to upload terrain tileset atlas"),
        }
    }

    fn draw(&mut self) {
        let now = Instant::now();
        let delta = (now - self.last_frame).as_secs_f32().clamp(1.0e-4, 0.1);
        self.last_frame = now;
        if self.mode == RaMode::Playing && !self.paused && !self.briefing_open {
            self.apply_ra_edge_pan(delta);
        }

        let placement_cell = if self.mode == RaMode::Playing {
            self.cursor_cell()
        } else {
            None
        };
        let victory = self.outcome_is_victory();
        if self.mode == RaMode::NetworkLobby {
            self.poll_lan_scan();
        }
        if self.mode == RaMode::Playing && !self.paused && !self.briefing_open {
            self.drain_pending_commands();
            self.poll_network();
            self.poll_replay();
        }

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        if self.mode == RaMode::Setup {
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_ra_setup(
                renderer.as_mut(),
                &self.setup,
                self.ra_tileset.as_ref(),
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "setup frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if self.mode == RaMode::NetworkLobby {
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_net_lobby(
                renderer.as_mut(),
                "RED ALERT",
                &self.lobby,
                self.lan_scan.as_ref(),
                &self.lobby_screen,
                self.lobby_error.as_deref(),
                &self.ready_tracker.roster(),
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "lobby frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if self.mode == RaMode::CampaignSelect {
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_ra_campaign(
                renderer.as_mut(),
                &self.campaign,
                self.ra_tileset.as_ref(),
                self.campaign_index,
                &self.campaign_completed,
                &self.campaign_best,
                self.replay_prompt,
                self.campaign_difficulty,
                self.campaign_filter,
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "campaign frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if self.mode == RaMode::CampaignStats {
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_ra_campaign_stats(
                renderer.as_mut(),
                &self.campaign,
                &self.campaign_completed,
                &self.campaign_best,
                self.campaign_difficulty,
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "campaign stats frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if self.mode == RaMode::CampaignComplete {
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            draw_ra_campaign_complete(
                renderer.as_mut(),
                self.campaign.len(),
                self.game.elapsed.int(),
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "campaign complete frame presentation failed");
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if self.briefing_open {
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(self.width, self.height));
            let borrowed = draw_ra_briefing(
                renderer.as_mut(),
                &self.scenario_name,
                &self.game,
                self.campaign_index,
                self.ra_tileset.as_ref(),
                self.width,
                self.height,
            );
            if let Err(error) = renderer.end_frame() {
                tracing::warn!(%error, "briefing frame presentation failed");
            }
            // A playing briefing borrows the shared tileset slot (plan box 212);
            // re-upload the RA terrain atlas now that the video frame has been
            // presented so the mission that follows samples the real tileset.
            if borrowed {
                if let Some(pack) = self.ra_tileset.as_ref() {
                    let _ = renderer.set_atlas_texture(
                        cnc_renderer::TILESET_TEXTURE_ID,
                        pack.width(),
                        pack.height(),
                        pack.rgba(),
                    );
                }
            }
            self.frame_index = self.frame_index.wrapping_add(1);
            return;
        }

        if !self.paused {
            self.game
                .update(Fixed16_16::from_f32(delta * self.game_speed));
            // In-game SFX: pump engine events through the audio director
            // (todo 42). A missing device falls back to a headless mixer;
            // route errors are swallowed either way.
            let events = self.event_subscriber.drain();
            self.audio_director.route_batch(&self.mixer, &events);
            // Theme escalation: a damaged player structure switches the
            // music layer to combat (THEME.CPP On_Base_Attack analogue).
            if self.game.eva.pending() == Some(cnc_audio::RaEvaLine::BaseUnderAttack) {
                self.theme_queue.on_base_attack();
            }
        }

        renderer.begin_frame();
        renderer.set_camera(&self.camera);
        self.game.render(renderer.as_mut());

        if let Some(id) = self.selected_structure {
            if let Some(structure) = self
                .game
                .structures
                .iter()
                .find(|structure| structure.id == id && structure.is_alive())
            {
                let half = RA_TILE_SIZE * 0.5;
                let x = structure.position.x.to_f32() - half;
                let y = structure.position.y.to_f32() - half;
                let color = [1.0, 0.82, 0.30, 1.0];
                for (rx, ry, rw, rh) in [
                    (x, y, RA_TILE_SIZE, 2.0),
                    (x, y + RA_TILE_SIZE - 2.0, RA_TILE_SIZE, 2.0),
                    (x, y, 2.0, RA_TILE_SIZE),
                    (x + RA_TILE_SIZE - 2.0, y, 2.0, RA_TILE_SIZE),
                ] {
                    renderer.draw_rect(rx, ry, rw, rh, color);
                }
            }
        }

        if let (Some(start), Some(end)) = (self.selection_start, self.cursor) {
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            if dx * dx + dy * dy > 36.0 {
                let a = self.camera.screen_to_world(Vec2::new(
                    Fixed16_16::from_f32(start.x as f32),
                    Fixed16_16::from_f32(start.y as f32),
                ));
                let b = self.camera.screen_to_world(Vec2::new(
                    Fixed16_16::from_f32(end.x as f32),
                    Fixed16_16::from_f32(end.y as f32),
                ));
                let x = a.x.to_f32().min(b.x.to_f32());
                let y = a.y.to_f32().min(b.y.to_f32());
                let w = (a.x.to_f32() - b.x.to_f32()).abs();
                let h = (a.y.to_f32() - b.y.to_f32()).abs();
                let color = [0.20, 0.80, 1.00, 1.0];
                renderer.draw_rect(x, y, w, 1.5, color);
                renderer.draw_rect(x, y + h - 1.5, w, 1.5, color);
                renderer.draw_rect(x, y, 1.5, h, color);
                renderer.draw_rect(x + w - 1.5, y, 1.5, h, color);
            }
        }

        if self.placing.is_some() {
            if let Some(cell) = placement_cell {
                let blocked = self
                    .game
                    .structures
                    .iter()
                    .any(|structure| structure.cell == cell && structure.is_alive())
                    || self
                        .game
                        .units
                        .iter()
                        .any(|unit| unit.cell == cell && unit.is_alive());
                let color = if blocked {
                    [1.0, 0.35, 0.30, 1.0]
                } else {
                    [0.40, 1.0, 0.50, 1.0]
                };
                let x = cell.x as f32 * RA_TILE_SIZE;
                let y = cell.y as f32 * RA_TILE_SIZE;
                renderer.draw_rect(x, y, RA_TILE_SIZE, 1.0, color);
                renderer.draw_rect(x, y + RA_TILE_SIZE - 1.0, RA_TILE_SIZE, 1.0, color);
                renderer.draw_rect(x, y, 1.0, RA_TILE_SIZE, color);
                renderer.draw_rect(x + RA_TILE_SIZE - 1.0, y, 1.0, RA_TILE_SIZE, color);
            }
        }

        // HUD. Its own layer, drawn with an unzoomed camera so status text,
        // objectives and the minimap keep a constant pixel size while the map
        // zooms underneath.
        renderer.begin_layer();
        renderer.set_camera(&hud_camera(&self.camera));
        let zoom = self.camera.zoom.to_f32().max(1.0e-6);
        let half_w = self.camera.viewport_width as f32 * 0.5;
        let half_h = self.camera.viewport_height as f32 * 0.5;
        let left = self.camera.position.x.to_f32() - half_w;
        let top = self.camera.position.y.to_f32() - half_h;
        let status = self.game.status();

        renderer.draw_text(
            &format!("RED ALERT {} - {}", self.scenario_name, self.game.summary()),
            left + 16.0,
            top + 16.0,
            14.0,
            [0.95, 0.90, 0.85, 1.0],
        );
        let queue_label = self
            .game
            .houses
            .get(self.game.player_owner)
            .and_then(|house| house.first_queued())
            .map(|item| {
                format!(
                    "{:?} {}%",
                    item.class_id,
                    production_percent(item.progress, item.total_time)
                )
            })
            .unwrap_or_else(|| "empty".to_string());
        renderer.draw_text(
            &format!(
                "Allies {}u {}b {}cr pwr {} | Soviets {}u {}b {}cr pwr {} | selected {} | shots {} | chrono {}% | queue {queue_label}",
                status.allied_units,
                status.allied_structures,
                status.allied_credits,
                status.allied_power,
                status.soviet_units,
                status.soviet_structures,
                status.soviet_credits,
                status.soviet_power,
                self.game.selected_count(self.game.player_owner),
                status.shots,
                self.game
                    .superweapons
                    .iter()
                    .find(|weapon| weapon.class_id == cnc_core::ClassId::Chronosphere)
                    .map_or(0, |weapon| (weapon.charge_fraction() * 100.0) as i32),
            ),
            left + 16.0,
            top + 40.0,
            8.0,
            [0.85, 0.88, 0.95, 1.0],
        );
        let chat_hint = if self.network.is_some() {
            "  Y chat"
        } else {
            ""
        };
        renderer.draw_text(
            &format!("LMB select / drag box  RMB move/attack  MMB drag pan  wheel zoom / edge-scroll  Ctrl+1-4 view  Ctrl+Shift+1-4 mark  1 rifle 2 tank 3 ship  Space light tank  A all  P/B/W/T build  LMB struct X sell R repair O power G scatter  Bksp cancel  S stop  Alt+1-4 group  D deploy MCV  C chrono  I curtain  F5/F9 save/load  +/- or wheel zoom  F6/F7 speed {:.2}x  ESC quit{chat_hint}", self.game_speed),
            left + 16.0,
            top + 56.0,
            8.0,
            [0.65, 0.72, 0.82, 1.0],
        );
        let mut info_y = top + 72.0;
        if !self.game.objectives.is_empty() {
            renderer.draw_text(
                "OBJECTIVES",
                left + 16.0,
                info_y,
                9.0,
                [0.60, 0.75, 0.95, 1.0],
            );
            info_y += 11.0;
            for objective in self.game.objectives.iter().take(4) {
                let mark = if objective.completed { "[x]" } else { "[ ]" };
                let color = if objective.completed {
                    [0.55, 0.85, 0.60, 1.0]
                } else {
                    [0.82, 0.88, 0.96, 1.0]
                };
                renderer.draw_text(
                    &format!("{mark} {}", objective.text),
                    left + 16.0,
                    info_y,
                    9.0,
                    color,
                );
                info_y += 10.0;
            }
            info_y += 4.0;
        }
        if self.game.elapsed.int() < 12 && !self.game.briefing.is_empty() {
            renderer.draw_text(
                &format!("BRIEFING: {}", self.game.briefing),
                left + 16.0,
                info_y,
                9.0,
                [0.95, 0.86, 0.55, 1.0],
            );
            info_y += 14.0;
        }
        if let Some(class_id) = self.placing {
            renderer.draw_text(
                &format!("PLACING {class_id:?} - LMB place, Esc cancel"),
                left + 16.0,
                info_y,
                9.0,
                [1.0, 0.85, 0.40, 1.0],
            );
            info_y += 14.0;
        }
        if let Some(net) = &self.network {
            renderer.draw_text(
                &format!(
                    "NET {} | applied {} sent {} | peers {} | lost {} mig {} rej {}",
                    net.role,
                    net.applied,
                    net.sent,
                    net.session.peers.len(),
                    net.losses,
                    net.migrations,
                    net.rejoined
                ),
                left + 16.0,
                info_y,
                8.0,
                [0.60, 0.90, 0.95, 1.0],
            );
            info_y += 11.0;
        }
        // End-of-mission scoreboard for RA.
        if let (Some(report), Some(since)) = (&self.ra_score, self.ra_score_since) {
            renderer.set_camera(&hud_camera(&self.camera));
            let half_w = self.camera.viewport_width as f32 * 0.5;
            let half_h = self.camera.viewport_height as f32 * 0.5;
            let left = self.camera.position.x.to_f32() - half_w;
            let top = self.camera.position.y.to_f32() - half_h;
            score::draw_overlay(
                renderer.as_mut(),
                report,
                since.elapsed(),
                left + 24.0,
                top + 100.0,
            );
        }

        if let Some((notice, at)) = &self.notice {
            if at.elapsed() < Duration::from_secs(4) {
                renderer.draw_text(notice, left + 16.0, info_y, 9.0, [1.0, 0.76, 0.35, 1.0]);
                info_y += 12.0;
            }
        }
        if let Some(state) = &self.replay {
            let status_text = if state.done {
                "END - Esc quits"
            } else if state.paused {
                "PAUSED - Space resumes"
            } else {
                "Space pauses, Esc quits"
            };
            renderer.draw_text(
                &format!(
                    "REPLAY frame {} / {} commands - {status_text}",
                    state.frame,
                    state.commands.len()
                ),
                left + 16.0,
                info_y,
                9.0,
                [0.95, 0.82, 0.35, 1.0],
            );
            info_y += 12.0;
        }
        if self.game.outcome.is_some() {
            let (text, color) = if victory {
                ("MISSION COMPLETE - Enter: continue", [0.45, 1.0, 0.55, 1.0])
            } else {
                ("MISSION FAILED - Enter: continue", [1.0, 0.45, 0.45, 1.0])
            };
            renderer.draw_text(text, left + 16.0, info_y, 10.0, color);
            info_y += 14.0;
        }
        if let Some(message) = self.game.messages.last() {
            renderer.draw_text(message, left + 16.0, info_y, 8.0, [0.85, 0.9, 0.95, 1.0]);
        }

        if !self.chat_log.is_empty() {
            let shown: Vec<&(u8, String)> = self.chat_log.iter().rev().take(4).collect();
            let mut chat_y = top + half_h * 2.0 - 40.0;
            for (player, text) in shown.iter().rev() {
                renderer.draw_text(
                    &format!("P{player}: {text}"),
                    left + 16.0,
                    chat_y,
                    9.0,
                    [0.90, 0.92, 0.98, 1.0],
                );
                chat_y += 12.0;
            }
            if self.chat_open {
                renderer.draw_text(
                    &format!("say> {}", self.chat_input),
                    left + 16.0,
                    chat_y,
                    9.0,
                    [1.0, 0.85, 0.35, 1.0],
                );
            }
        }

        draw_ra_minimap(
            renderer.as_mut(),
            &self.game,
            self.ra_tileset.as_ref(),
            self.game.player_owner,
            self.camera.position,
            self.camera.viewport_width as f32 / zoom,
            self.camera.viewport_height as f32 / zoom,
            left,
            top,
            self.camera.viewport_width as f32,
            self.camera.viewport_height as f32,
        );

        if self.paused {
            renderer.begin_layer();
            renderer.set_camera(&screen_camera(self.width, self.height));
            let slot_label = save_slot_label(&ra_save_path_for(self.save_slot));
            draw_ra_pause(
                renderer.as_mut(),
                &self.game,
                self.pause_choice,
                self.save_slot,
                &slot_label,
                self.width,
                self.height,
            );
        }

        if let Err(error) = renderer.end_frame() {
            tracing::warn!(%error, "frame presentation failed");
        }
        self.frame_index = self.frame_index.wrapping_add(1);
        if self.kind == RendererKind::Software && !self.software_warning_emitted {
            tracing::warn!(
                "software renderer draws offscreen only; use --renderer wgpu to present frames"
            );
            self.software_warning_emitted = true;
        }
        if self.frame_index.is_multiple_of(120) {
            tracing::info!("RA {}", self.game.summary());
        }
    }
}

impl ApplicationHandler for RaLauncherApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title("C&C Red Alert - Rust skirmish")
            .with_inner_size(LogicalSize::new(self.width, self.height));
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                tracing::error!(%error, "failed to create window");
                event_loop.exit();
                return;
            }
        };
        let size = window.inner_size();
        self.width = size.width.max(1);
        self.height = size.height.max(1);
        self.camera.set_viewport(self.width, self.height);
        self.camera.position = Vec2::new(
            Fixed16_16::from_f32(self.game.map.width as f32 * RA_TILE_SIZE * 0.5),
            Fixed16_16::from_f32(self.game.map.height as f32 * RA_TILE_SIZE * 0.5),
        );
        self.renderer = Some(create_renderer(
            self.kind,
            self.width,
            self.height,
            window.clone(),
        ));
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.set_camera(&self.camera);
        }
        // Terrain screens in this shell are RA; bind the RA pack once here so
        // setup/campaign previews and the minimap sample a resident texture.
        self.bind_tileset(TilesetPalette::Ra);
        self.window = Some(window);
        self.last_frame = Instant::now();
        self.next_frame = Instant::now() + FRAME_INTERVAL;
        tracing::info!(
            "Red Alert window ready at {}x{} - mode {:?}, {} units, {} structures, frame limit {}",
            self.width,
            self.height,
            self.mode,
            self.game.units.len(),
            self.game.structures.len(),
            self.frame_limit
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.replay.is_some() {
            match &event {
                WindowEvent::CloseRequested => event_loop.exit(),
                WindowEvent::KeyboardInput { event, .. }
                    if event.state == ElementState::Pressed =>
                {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => event_loop.exit(),
                        Key::Named(NamedKey::Space) => {
                            if let Some(state) = self.replay.as_mut() {
                                state.paused = !state.paused;
                            }
                        }
                        _ => {}
                    }
                }
                WindowEvent::RedrawRequested => self.draw(),
                _ => {}
            }
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::ModifiersChanged(modifiers) => {
                self.shift_down = modifiers.state().shift_key();
                self.ctrl_down = modifiers.state().control_key();
                self.alt_down = modifiers.state().alt_key();
            }
            WindowEvent::Resized(size) => {
                let width = size.width.max(1);
                let height = size.height.max(1);
                self.width = width;
                self.height = height;
                self.camera.set_viewport(width, height);
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(width, height);
                    renderer.set_camera(&self.camera);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = Some(position);
                if self.panning {
                    let dx = position.x - self.last_pointer.0;
                    let dy = position.y - self.last_pointer.1;
                    let zoom = self.camera.zoom.to_f32().max(1.0e-6);
                    self.camera.pan(Vec2::new(
                        Fixed16_16::from_f32(-dx as f32 / zoom),
                        Fixed16_16::from_f32(-dy as f32 / zoom),
                    ));
                }
                self.last_pointer = (position.x, position.y);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                use winit::event::MouseScrollDelta;
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(position) => position.y as f32 / 40.0,
                };
                if amount.abs() > f32::EPSILON {
                    self.zoom_ra(wheel_zoom_factor(amount));
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match (state, button) {
                (ElementState::Pressed, MouseButton::Left) => {
                    if self.mode == RaMode::Setup {
                        return;
                    }
                    if let Some(class_id) = self.placing {
                        if let Some(cell) = self.cursor_cell() {
                            match self
                                .game
                                .place_structure(self.game.player_owner, class_id, cell)
                            {
                                Ok(id) => {
                                    let location = Vec2::new(
                                        Fixed16_16::from_f32((cell.x as f32 + 0.5) * RA_TILE_SIZE),
                                        Fixed16_16::from_f32((cell.y as f32 + 0.5) * RA_TILE_SIZE),
                                    );
                                    if let Some(net) = self.network.as_mut() {
                                        net.session.queue_local(NetCommandKind::Build {
                                            building: class_id,
                                            location,
                                        });
                                    }
                                    self.placing = None;
                                    tracing::info!(?class_id, id, "RA structure placed");
                                }
                                Err(error) => {
                                    tracing::warn!(?error, ?class_id, "RA placement rejected");
                                }
                            }
                        }
                        return;
                    }
                    if let Some(world) = self.cursor_world() {
                        if self.ra_minimap_click(world) {
                            return;
                        }
                        let owner = self.game.player_owner;
                        let picked = self.game.select_at_owned(world, owner);
                        if picked.is_some() {
                            self.selected_structure = None;
                        } else if let Some(cell) = self.cursor_cell() {
                            self.selected_structure = self
                                .game
                                .structures
                                .iter()
                                .find(|structure| {
                                    structure.owner == owner
                                        && structure.is_alive()
                                        && structure.cell == cell
                                })
                                .map(|structure| structure.id);
                        }
                        tracing::debug!(?picked, "RA selection");
                    }
                    self.selection_start = self.cursor;
                }
                (ElementState::Released, MouseButton::Left) => {
                    if let (Some(start), Some(end)) = (self.selection_start.take(), self.cursor) {
                        let dx = end.x - start.x;
                        let dy = end.y - start.y;
                        if dx * dx + dy * dy > 36.0 {
                            let a = self.screen_to_world(start);
                            let b = self.screen_to_world(end);
                            let player = self.game.player_owner;
                            let count = self.game.select_in_rect(player, a, b, self.shift_down);
                            tracing::debug!(count, "RA box selection");
                        }
                    }
                }
                (ElementState::Pressed, MouseButton::Right) => {
                    if let Some(world) = self.cursor_world() {
                        let player = self.game.player_owner;
                        if self.game.selected_count(player) == 0 {
                            self.game.select_all_owned(player);
                        }
                        let units: Vec<u32> = self
                            .game
                            .units
                            .iter()
                            .filter(|unit| unit.selected && unit.owner == player && unit.is_alive())
                            .map(|unit| unit.id)
                            .collect();
                        if !units.is_empty() {
                            if let Some(target) = self.enemy_target_at(world, player) {
                                if let Some(net) = self.network.as_mut() {
                                    net.session.queue_local(NetCommandKind::Attack {
                                        units: units.clone(),
                                        target,
                                    });
                                }
                                self.pending_commands.push(GameCommand::Attack {
                                    owner: player,
                                    units,
                                    target,
                                });
                                tracing::debug!(target, "RA attack command queued");
                            } else {
                                if let Some(net) = self.network.as_mut() {
                                    net.session.queue_local(NetCommandKind::Move {
                                        units: units.clone(),
                                        target: world,
                                    });
                                }
                                self.pending_commands.push(GameCommand::Move {
                                    owner: player,
                                    units,
                                    target: world,
                                });
                                tracing::debug!("RA move command queued");
                            }
                        }
                    }
                }
                (ElementState::Pressed, MouseButton::Middle) => {
                    self.panning = true;
                    if let Some(position) = self.cursor {
                        self.last_pointer = (position.x, position.y);
                    }
                }
                (ElementState::Released, MouseButton::Middle) => self.panning = false,
                _ => {}
            },
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }
                if self.mode == RaMode::Setup {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => self.return_to_menu = true,
                        Key::Named(NamedKey::Enter) => self.start_skirmish(),
                        Key::Named(NamedKey::Tab) => self.cycle_country(),
                        Key::Character(text) => match text.as_str() {
                            "+" | "=" => self.cycle_map_size(true),
                            "-" | "_" => self.cycle_map_size(false),
                            "d" | "D" => self.cycle_difficulty(),
                            "s" | "S" => self.cycle_side(),
                            "n" | "N" => self.open_lobby(),
                            _ => {}
                        },
                        _ => {}
                    }
                    return;
                }
                if self.mode == RaMode::NetworkLobby {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => {
                            if self.lan_scan.take().is_none() {
                                self.mode = RaMode::Setup;
                                self.lobby_error = None;
                            }
                        }
                        Key::Named(NamedKey::Enter) => {
                            let selected_host = self
                                .lan_scan
                                .as_ref()
                                .and_then(|scan| scan.hosts.get(scan.selected))
                                .map(|host| host.address.clone());
                            if let Some(address) = selected_host {
                                self.lobby.role = NetRole::Join;
                                self.lobby.join_address = address;
                            }
                            let _ = self.start_lobby();
                        }
                        Key::Named(NamedKey::Tab) => {
                            self.lobby.focus = (self.lobby.focus + 1) % 2;
                        }
                        Key::Named(NamedKey::Backspace) => self.lobby.backspace(),
                        Key::Named(NamedKey::ArrowUp) => {
                            if let Some(scan) = self.lan_scan.as_mut() {
                                if !scan.hosts.is_empty() {
                                    scan.selected =
                                        (scan.selected + scan.hosts.len() - 1) % scan.hosts.len();
                                }
                            }
                        }
                        Key::Named(NamedKey::ArrowDown) => {
                            if let Some(scan) = self.lan_scan.as_mut() {
                                if !scan.hosts.is_empty() {
                                    scan.selected = (scan.selected + 1) % scan.hosts.len();
                                }
                            }
                        }
                        Key::Character(text) => match text.as_str() {
                            "r" | "R" => self.lobby.toggle_role(),
                            "l" | "L" => self.toggle_lan_scan(),
                            "d" | "D" => self.toggle_lobby_directory(),
                            "[" => self.lobby.adjust_port(-1),
                            "]" => self.lobby.adjust_port(1),
                            other => {
                                for character in other.chars() {
                                    self.lobby.push_char(character);
                                }
                            }
                        },
                        _ => {}
                    }
                    return;
                }
                if self.mode == RaMode::CampaignStats {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) | Key::Named(NamedKey::Enter) => {
                            self.dismiss_campaign_stats()
                        }
                        _ => {}
                    }
                    return;
                }
                if self.mode == RaMode::CampaignComplete {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => self.return_to_menu = true,
                        Key::Named(NamedKey::Enter) => self.dismiss_campaign_complete(),
                        _ => {}
                    }
                    return;
                }
                if self.mode == RaMode::CampaignSelect {
                    if self.replay_prompt {
                        match &event.logical_key {
                            Key::Named(NamedKey::Enter) => self.confirm_replay(),
                            Key::Named(NamedKey::Escape) => self.cancel_replay(),
                            _ => {}
                        }
                        return;
                    }
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => self.return_to_menu = true,
                        Key::Named(NamedKey::Enter) => {
                            self.request_campaign_mission(self.campaign_index);
                        }
                        Key::Named(NamedKey::ArrowUp) => self.cycle_campaign(-1),
                        Key::Named(NamedKey::ArrowDown) => self.cycle_campaign(1),
                        Key::Character(text) => match text.as_str() {
                            "+" | "=" => self.cycle_campaign(1),
                            "-" | "_" => self.cycle_campaign(-1),
                            "d" | "D" => self.cycle_campaign_difficulty(),
                            "f" | "F" => self.cycle_campaign_filter(),
                            "s" | "S" => self.show_campaign_stats(),
                            _ => {}
                        },
                        _ => {}
                    }
                    return;
                }
                if self.briefing_open {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => self.return_to_menu = true,
                        Key::Named(NamedKey::Enter) => self.briefing_open = false,
                        _ => {}
                    }
                    return;
                }
                if self.paused {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => self.paused = false,
                        Key::Named(NamedKey::Enter) => self.activate_pause_choice(),
                        Key::Named(NamedKey::ArrowUp) => {
                            self.pause_choice = (self.pause_choice + 5) % 6;
                        }
                        Key::Named(NamedKey::ArrowDown) => {
                            self.pause_choice = (self.pause_choice + 1) % 6;
                        }
                        _ => {}
                    }
                    return;
                }
                if self.ctrl_down {
                    if let Key::Character(text) = &event.logical_key {
                        if let Some(slot) = bookmark_slot(text) {
                            if self.shift_down {
                                self.save_ra_bookmark(slot);
                            } else {
                                self.jump_ra_bookmark(slot);
                            }
                            return;
                        }
                    }
                }
                if self.alt_down {
                    if let Key::Character(text) = &event.logical_key {
                        if let Some(slot) = bookmark_slot(text) {
                            if self.shift_down {
                                self.store_control_group(slot);
                            } else {
                                self.recall_control_group(slot);
                            }
                            return;
                        }
                    }
                }
                if let Key::Named(NamedKey::Enter) = &event.logical_key {
                    if self.game.outcome.is_some() {
                        if self.ra_score.is_none() {
                            self.show_ra_score();
                        } else {
                            self.ra_score = None;
                            self.ra_score_since = None;
                            if self.campaign_active && self.outcome_is_victory() {
                                self.advance_campaign();
                            } else {
                                self.abort_mission();
                            }
                        }
                        return;
                    }
                }
                if self.network.is_some() {
                    if self.chat_open {
                        match &event.logical_key {
                            Key::Named(NamedKey::Escape) => {
                                self.chat_open = false;
                                self.chat_input.clear();
                            }
                            Key::Named(NamedKey::Enter) => self.send_chat(),
                            Key::Named(NamedKey::Backspace) => {
                                self.chat_input.pop();
                            }
                            Key::Character(text) => {
                                for character in text.chars() {
                                    if !character.is_control() && self.chat_input.len() < 120 {
                                        self.chat_input.push(character);
                                    }
                                }
                            }
                            _ => {}
                        }
                        return;
                    }
                    if let Key::Character(text) = &event.logical_key {
                        if text.eq_ignore_ascii_case("y") {
                            self.chat_open = true;
                            self.chat_input.clear();
                            return;
                        }
                        if text.eq_ignore_ascii_case("r") {
                            let local = self
                                .network
                                .as_ref()
                                .map(|net| net.session.local_player)
                                .unwrap_or(0);
                            let name = format!("Player{local}");
                            let ready = !self.ready_tracker.is_ready(&name);
                            self.send_ready(ready);
                            return;
                        }
                    }
                }
                match &event.logical_key {
                    Key::Named(NamedKey::Escape) => {
                        if self.placing.take().is_none() {
                            self.enter_pause();
                        }
                    }
                    Key::Named(NamedKey::F5) => self.save_game(),
                    Key::Named(NamedKey::F9) => {
                        self.load_game();
                    }
                    Key::Named(NamedKey::Space) => {
                        self.produce_unit(cnc_core::ClassId::VehicleE1);
                    }
                    Key::Named(NamedKey::Backspace) => self.cancel_production(),
                    Key::Named(NamedKey::F6) => self.adjust_ra_speed(-0.25),
                    Key::Named(NamedKey::F7) => self.adjust_ra_speed(0.25),
                    Key::Character(text) => {
                        let player = self.game.player_owner;
                        match text.to_ascii_lowercase().as_str() {
                            "a" => {
                                let count = self.game.select_all_owned(player);
                                tracing::debug!(count, "RA select all");
                            }
                            "1" => self.produce_unit(cnc_core::ClassId::InfantryE1),
                            "2" => self.produce_unit(cnc_core::ClassId::VehicleE2),
                            "3" => self.produce_unit(cnc_core::ClassId::ShipE1),
                            "+" | "=" => self.zoom_ra(KEY_ZOOM_STEP),
                            "-" | "_" => self.zoom_ra(1.0 / KEY_ZOOM_STEP),
                            "g" => self.scatter_selected_units(),
                            "o" => self.toggle_selected_power(),
                            "r" => self.repair_selected_structure(),
                            "s" => self.stop_selected_units(),
                            "x" => self.sell_selected_structure(),
                            "d" => self.deploy_selected_mcv(),
                            "p" => self.placing = Some(cnc_core::ClassId::PowerPlant),
                            "b" => self.placing = Some(cnc_core::ClassId::Barracks),
                            "w" => self.placing = Some(cnc_core::ClassId::WarFactory),
                            "t" => self.placing = Some(cnc_core::ClassId::TeslaCoil),
                            "c" => {
                                if let Some(cell) = self.cursor_cell() {
                                    match self.game.fire_chronosphere(player, cell) {
                                        Ok(count) => tracing::info!(count, "RA chronoshift"),
                                        Err(error) => {
                                            tracing::warn!(%error, "RA chronoshift failed")
                                        }
                                    }
                                }
                            }
                            "i" => match self.game.fire_iron_curtain(player) {
                                Ok(count) => tracing::info!(count, "RA iron curtain"),
                                Err(error) => tracing::warn!(%error, "RA iron curtain failed"),
                            },
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            WindowEvent::RedrawRequested => self.draw(),
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if self.frame_limit > 0 && self.frame_index >= self.frame_limit {
            self.save_network_replay();
            if let Some(net) = &self.network {
                println!(
                    "net exit role={} applied={} sent={} peers={} frame={} lost={} mig={} rej={}",
                    net.role,
                    net.applied,
                    net.sent,
                    net.session.peers.len(),
                    net.session.frame,
                    net.losses,
                    net.migrations,
                    net.rejoined
                );
            }
            event_loop.exit();
            return;
        }
        if now >= self.next_frame {
            self.next_frame = now + FRAME_INTERVAL;
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

/// The active owner of the window: the menu or one of the two game engines.
enum ShellSession {
    Menu,
    Td(Box<LauncherApp>),
    Ra(Box<RaLauncherApp>),
}

impl ShellSession {
    fn window(&self) -> Option<&Arc<Window>> {
        match self {
            Self::Menu => None,
            Self::Td(app) => app.window.as_ref(),
            Self::Ra(app) => app.window.as_ref(),
        }
    }
}

/// What a menu window event still needs to do after the session borrow ends.
enum MenuAction {
    None,
    Resize(u32, u32),
    Redraw,
}

/// Front-end shell: shows the main menu, then hands the window to the chosen
/// engine. The menu window is kept alive (hidden) while a game runs so that
/// returning to the menu is instant and never recreates a surface.
struct LauncherShell {
    kind: RendererKind,
    width: u32,
    height: u32,
    frame_limit: u64,
    rules_path: Option<PathBuf>,
    /// Persisted launcher options handed to every game session this shell
    /// builds (plan line 206).
    options: LauncherOptions,
    /// CLI `--options` path captured at startup; the Options modal saves to
    /// the same file the boot path loaded.
    options_path: Option<PathBuf>,
    /// The Options modal while it owns the menu window's input.
    options_modal: Option<OptionsModal>,
    /// Persisted options snapshot taken when the modal opened; a cancel
    /// restores it, a save keeps the live-applied edits.
    options_backup: Option<LauncherOptions>,
    /// Live-preview mixer for the modal's volume rows (plan line 206); built
    /// on the first open and reused.
    menu_mixer: Option<AudioMixer>,
    menu: MainMenu,
    menu_window: Option<Arc<Window>>,
    menu_renderer: Option<Box<dyn Renderer>>,
    menu_cursor: (f32, f32),
    /// True once a menu frame was successfully presented. Wayland only maps a
    /// window after the first buffer commit, so redraws must be re-requested
    /// until then (a single request during `resumed` can be dropped).
    menu_presented: bool,
    session: ShellSession,
}

impl LauncherShell {
    fn new(
        kind: RendererKind,
        width: u32,
        height: u32,
        frame_limit: u64,
        rules_path: Option<PathBuf>,
        options: LauncherOptions,
        options_path: Option<PathBuf>,
    ) -> Self {
        let menu = MainMenu::discover();
        tracing::info!(
            td_scenarios = menu.campaign(GameChoice::TiberianDawn).scenarios,
            ra_scenarios = menu.campaign(GameChoice::RedAlert).scenarios,
            "main menu campaign libraries discovered"
        );
        Self {
            kind,
            width,
            height,
            frame_limit,
            rules_path,
            options,
            options_path,
            options_modal: None,
            options_backup: None,
            menu_mixer: None,
            menu,
            menu_window: None,
            menu_renderer: None,
            menu_cursor: (0.0, 0.0),
            menu_presented: false,
            session: ShellSession::Menu,
        }
    }

    fn create_menu_window(&mut self, event_loop: &ActiveEventLoop) {
        if self.menu_window.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title("C&C Remastered Collection - Rust Clone")
            .with_inner_size(LogicalSize::new(self.width, self.height));
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                tracing::error!(%error, "failed to create the main menu window");
                event_loop.exit();
                return;
            }
        };
        let size = window.inner_size();
        self.width = size.width.max(1);
        self.height = size.height.max(1);
        self.menu_renderer = Some(create_renderer(
            self.kind,
            self.width,
            self.height,
            window.clone(),
        ));
        if let Some(renderer) = self.menu_renderer.as_mut() {
            renderer.set_camera(&screen_camera(self.width, self.height));
        }
        self.menu_window = Some(window);
        tracing::info!(
            "main menu window ready at {}x{} (renderer={})",
            self.width,
            self.height,
            self.kind.label()
        );
    }

    fn resize_menu(&mut self, width: u32, height: u32) {
        self.width = width.max(1);
        self.height = height.max(1);
        if let Some(renderer) = self.menu_renderer.as_mut() {
            renderer.resize(self.width, self.height);
            renderer.set_camera(&screen_camera(self.width, self.height));
        }
    }

    fn draw_menu(&mut self) {
        let Some(renderer) = self.menu_renderer.as_mut() else {
            return;
        };
        renderer.begin_frame();
        renderer.set_camera(&screen_camera(self.width, self.height));
        self.menu.draw(renderer.as_mut(), self.width, self.height);
        if let Some(modal) = &self.options_modal {
            modal.draw(renderer.as_mut(), self.width, self.height);
        }
        match renderer.end_frame() {
            Ok(()) => self.menu_presented = true,
            Err(error) => tracing::warn!(%error, "menu frame presentation failed"),
        }
    }

    /// Build the game session for a menu launch request.
    fn build_session(&self, target: &LaunchTarget) -> Result<ShellSession> {
        match target {
            LaunchTarget::Campaign(game) => {
                let dir = self.menu.campaign(*game).dir.clone();
                match game {
                    GameChoice::TiberianDawn => {
                        let mut app = self.build_td()?;
                        let count = app.load_mission_dir(&dir)?;
                        if count == 0 {
                            return Err(anyhow!(
                                "no Tiberian Dawn scenarios found in {}",
                                dir.display()
                            ));
                        }
                        Ok(ShellSession::Td(Box::new(app)))
                    }
                    GameChoice::RedAlert => {
                        let mut app = RaLauncherApp::new(
                            self.kind,
                            self.width,
                            self.height,
                            self.frame_limit,
                        );
                        let count = app.load_campaign(&dir);
                        if count == 0 {
                            return Err(anyhow!(
                                "no Red Alert scenarios found in {}",
                                dir.display()
                            ));
                        }
                        Ok(ShellSession::Ra(Box::new(app)))
                    }
                }
            }
            LaunchTarget::Local(game) => match game {
                GameChoice::TiberianDawn => Ok(ShellSession::Td(Box::new(self.build_td()?))),
                GameChoice::RedAlert => Ok(ShellSession::Ra(Box::new(RaLauncherApp::new(
                    self.kind,
                    self.width,
                    self.height,
                    self.frame_limit,
                )))),
            },
            LaunchTarget::Online(game) => match game {
                GameChoice::RedAlert => {
                    let mut app =
                        RaLauncherApp::new(self.kind, self.width, self.height, self.frame_limit);
                    app.open_lobby();
                    Ok(ShellSession::Ra(Box::new(app)))
                }
                GameChoice::TiberianDawn => {
                    Err(anyhow!("Tiberian Dawn network play is not available"))
                }
            },
        }
    }

    fn build_td(&self) -> Result<LauncherApp> {
        let rules = match &self.rules_path {
            Some(path) => RulesDatabase::load(path)
                .with_context(|| format!("failed to load rules from {}", path.display()))?,
            None => RulesDatabase::default(),
        };
        LauncherApp::new(
            self.kind,
            self.width,
            self.height,
            rules,
            self.options.clone(),
        )
    }

    fn start_game(&mut self, event_loop: &ActiveEventLoop, target: LaunchTarget) {
        if let Some(window) = &self.menu_window {
            window.set_visible(false);
        }
        match self.build_session(&target) {
            Ok(mut session) => {
                match &mut session {
                    ShellSession::Td(app) => app.resumed(event_loop),
                    ShellSession::Ra(app) => app.resumed(event_loop),
                    ShellSession::Menu => {}
                }
                self.session = session;
                if let Some(window) = self.session.window() {
                    window.focus_window();
                }
                tracing::info!(target = ?target, "main menu launched a game session");
            }
            Err(error) => {
                tracing::warn!(%error, "main menu launch failed");
                self.menu.set_notice(format!("{error}"));
                self.session = ShellSession::Menu;
                self.menu_presented = false;
                if let Some(window) = &self.menu_window {
                    window.set_visible(true);
                    window.focus_window();
                    window.request_redraw();
                }
                event_loop.set_control_flow(ControlFlow::Wait);
            }
        }
    }

    fn open_menu(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("returning to the main menu");
        self.session = ShellSession::Menu;
        self.menu.reset();
        // The hidden menu window must present again after being re-shown.
        self.menu_presented = false;
        if let Some(window) = &self.menu_window {
            window.set_visible(true);
            window.focus_window();
            window.request_redraw();
        }
        event_loop.set_control_flow(ControlFlow::Wait);
    }

    fn handle_menu_command(&mut self, event_loop: &ActiveEventLoop, command: MenuCommand) {
        match command {
            MenuCommand::Quit => event_loop.exit(),
            MenuCommand::Launch(target) => self.start_game(event_loop, target),
            MenuCommand::Options => self.open_options_modal(),
        }
    }

    /// Open the Options modal over the main menu, seeded from the persisted
    /// settings (plan line 206).
    fn open_options_modal(&mut self) {
        let persisted = self.options.clone();
        let requested = persisted.renderer.trim().to_string();
        // An invalid persisted value never blocks boot or the modal: the
        // resolved choice is shown and the fallback is explained.
        let invalid = !requested.is_empty() && RendererKind::parse(&requested).is_err();
        let choice = match persisted.resolved_renderer_kind() {
            RendererKind::Wgpu => menu::RendererChoice::Wgpu,
            RendererKind::Software => menu::RendererChoice::Software,
        };
        let note = invalid.then(|| {
            format!(
                "Saved renderer '{}' is invalid - using {}.",
                requested,
                choice.label()
            )
        });
        if self.menu_mixer.is_none() {
            self.menu_mixer = Some(AudioMixer::new().unwrap_or_else(|_| AudioMixer::headless()));
        }
        self.options_backup = Some(persisted.clone());
        self.options_modal = Some(OptionsModal::new(
            choice,
            persisted.master_volume,
            persisted.music_volume,
            persisted.effects_volume,
            persisted.controls,
            note,
        ));
    }

    /// Apply one modal gesture: volume edits push through the live mixer,
    /// the renderer choice updates the persisted setting, and save/cancel
    /// close the modal (a cancel restores the persisted snapshot).
    fn apply_options_action(&mut self, action: OptionsAction) {
        match action {
            OptionsAction::Renderer(choice) => {
                self.options.renderer = choice.label().to_string();
            }
            OptionsAction::Volume(bus, volume) => {
                match bus {
                    VolumeBus::Master => self.options.master_volume = volume,
                    VolumeBus::Music => self.options.music_volume = volume,
                    VolumeBus::Effects => self.options.effects_volume = volume,
                }
                if let Some(mixer) = &self.menu_mixer {
                    self.options.apply_to_mixer(mixer);
                }
            }
            OptionsAction::Save => {
                let result = self.options.save(self.options_path.as_deref());
                match result {
                    Ok(()) => {
                        self.close_options_modal(false);
                        self.menu.set_notice("Options saved.");
                    }
                    Err(error) => {
                        tracing::warn!(%error, "saving launcher options failed");
                        if let Some(modal) = self.options_modal.as_mut() {
                            modal.set_status(format!("Save failed: {error}"));
                        }
                    }
                }
            }
            OptionsAction::Cancel => self.close_options_modal(true),
        }
    }

    /// Close the modal; `restore` puts the persisted options (and their
    /// mixer volumes) back when the edits are discarded.
    fn close_options_modal(&mut self, restore: bool) {
        self.options_modal = None;
        let backup = self.options_backup.take();
        if restore {
            if let Some(backup) = backup {
                self.options = backup;
                if let Some(mixer) = &self.menu_mixer {
                    self.options.apply_to_mixer(mixer);
                }
            }
        }
    }
}

impl ApplicationHandler for LauncherShell {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if matches!(self.session, ShellSession::Menu) {
            self.create_menu_window(event_loop);
            // Wayland only maps a window after the first buffer is committed,
            // so an initial redraw must be requested or the menu stays hidden.
            if let Some(window) = &self.menu_window {
                window.request_redraw();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let mut command = None;
        let mut action = MenuAction::None;
        let mut return_to_menu = false;
        match &mut self.session {
            ShellSession::Menu => {
                if self.menu_window.as_ref().map(|window| window.id()) != Some(window_id) {
                    return;
                }
                match event {
                    WindowEvent::CloseRequested => {
                        event_loop.exit();
                        return;
                    }
                    WindowEvent::Resized(size) => {
                        action = MenuAction::Resize(size.width, size.height);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.menu_cursor = (position.x as f32, position.y as f32);
                        let hovered = if let Some(modal) = self.options_modal.as_mut() {
                            modal.on_pointer(
                                self.menu_cursor.0,
                                self.menu_cursor.1,
                                self.width,
                                self.height,
                            )
                        } else {
                            self.menu
                                .on_pointer(self.menu_cursor.0, self.menu_cursor.1, self.width)
                        };
                        if hovered {
                            action = MenuAction::Redraw;
                        }
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if let Some(modal) = self.options_modal.as_mut() {
                            let clicked = modal.on_click(
                                self.menu_cursor.0,
                                self.menu_cursor.1,
                                self.width,
                                self.height,
                            );
                            if let Some(clicked) = clicked {
                                self.apply_options_action(clicked);
                            }
                            action = MenuAction::Redraw;
                        } else {
                            command = self.menu.on_click(
                                self.menu_cursor.0,
                                self.menu_cursor.1,
                                self.width,
                            );
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        // The wheel scrolls the modal's controls list while it
                        // owns the screen; the menu itself has nothing to scroll.
                        if let Some(modal) = self.options_modal.as_mut() {
                            use winit::event::MouseScrollDelta;
                            let notches = match delta {
                                MouseScrollDelta::LineDelta(_, y) => y,
                                MouseScrollDelta::PixelDelta(position) => position.y as f32 / 40.0,
                            };
                            modal.scroll_controls_wheel(notches);
                            modal.clamp_controls_scroll(self.height);
                            action = MenuAction::Redraw;
                        }
                    }
                    WindowEvent::KeyboardInput { event, .. }
                        if event.state == ElementState::Pressed =>
                    {
                        if let Some(modal) = self.options_modal.as_mut() {
                            let pressed = modal.on_key(&event.logical_key);
                            modal.clamp_controls_scroll(self.height);
                            if let Some(pressed) = pressed {
                                self.apply_options_action(pressed);
                            }
                        } else {
                            command = self.menu.on_key(&event.logical_key);
                        }
                        action = MenuAction::Redraw;
                    }
                    WindowEvent::RedrawRequested => action = MenuAction::Redraw,
                    _ => {}
                }
            }
            ShellSession::Td(app) => {
                if app.window.as_ref().map(|window| window.id()) != Some(window_id) {
                    return;
                }
                app.window_event(event_loop, window_id, event);
                return_to_menu = app.take_return_to_menu();
            }
            ShellSession::Ra(app) => {
                if app.window.as_ref().map(|window| window.id()) != Some(window_id) {
                    return;
                }
                app.window_event(event_loop, window_id, event);
                return_to_menu = app.take_return_to_menu();
            }
        }

        match action {
            MenuAction::None => {}
            MenuAction::Resize(width, height) => {
                self.resize_menu(width, height);
                if let Some(window) = &self.menu_window {
                    window.request_redraw();
                }
            }
            MenuAction::Redraw => self.draw_menu(),
        }
        if let Some(command) = command {
            self.handle_menu_command(event_loop, command);
        } else if return_to_menu {
            self.open_menu(event_loop);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        match &mut self.session {
            ShellSession::Menu => {
                if !self.menu_presented {
                    // Re-request redraws until the first frame is presented;
                    // Wayland drops requests made before the initial configure.
                    if let Some(window) = &self.menu_window {
                        window.request_redraw();
                    }
                    event_loop
                        .set_control_flow(ControlFlow::WaitUntil(Instant::now() + FRAME_INTERVAL));
                } else {
                    event_loop.set_control_flow(ControlFlow::Wait);
                }
            }
            ShellSession::Td(app) => app.about_to_wait(event_loop),
            ShellSession::Ra(app) => app.about_to_wait(event_loop),
        }
    }
}

fn list_ra_campaign(dir: &Path) -> Result<()> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("failed to read campaign directory {}", dir.display()))?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
        })
        .collect();
    paths.sort();
    let mut count = 0;
    for path in &paths {
        let scenario = cnc_red_alert::Scenario::from_file(path)
            .with_context(|| format!("invalid scenario {}", path.display()))?;
        println!(
            "{} - {} (player {}, {}x{}, {} starts, {} objectives, {} triggers)",
            path.file_name().unwrap_or_default().to_string_lossy(),
            scenario.name,
            if scenario.player == 0 {
                "Allies"
            } else {
                "Soviet"
            },
            scenario.width,
            scenario.height,
            scenario.starts.len(),
            scenario.objectives.len(),
            scenario.triggers.len()
        );
        count += 1;
    }
    println!(
        "listed {count} RA campaign scenario(s) from {}",
        dir.display()
    );
    Ok(())
}

fn list_td_missions(dir: &Path) -> Result<()> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("failed to read mission directory {}", dir.display()))?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
        })
        .collect();
    paths.sort();
    let mut count = 0;
    for path in &paths {
        let mission = Mission::from_file(path)
            .with_context(|| format!("invalid mission {}", path.display()))?;
        println!(
            "{} - {} (player {}, {} objectives, {} triggers)",
            path.file_name().unwrap_or_default().to_string_lossy(),
            mission.name,
            mission.player,
            mission.objectives.len(),
            mission.triggers.len()
        );
        count += 1;
    }
    println!("listed {count} TD mission(s) from {}", dir.display());
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    // Structured logging: `cnc-logs` installs the tracing subscriber with
    // per-target filters and mirrors every enabled event into a bounded
    // in-memory LogBuffer (oldest entries rotate out when it is full).
    let _logging_guard = cnc_logs::init_logging_with_buffer(
        cnc_launcher::logging::launcher_log_config(),
        cnc_logs::LogBuffer::new(cnc_launcher::logging::LOG_BUFFER_CAPACITY),
    )?;
    tracing::info!(
        capacity = cnc_launcher::logging::LOG_BUFFER_CAPACITY,
        "cnc-launcher structured logging initialized"
    );

    // Original-format TD campaign data is optional. When CNC_ORIGINAL_DATA_DIR
    // points at a retail install the launcher discovers and validates its
    // SC-*.INI/.BIN scenarios at boot; when the variable is unset this logs a
    // warning and the bundled `maps/td_campaign` missions stay in use.
    cnc_launcher::original_campaign::load_from_env();

    let options = LauncherOptions::load(args.options.as_deref());
    // Renderer precedence (plan line 206): explicit --renderer flag, then
    // the persisted options.renderer, then the wgpu default.
    let kind = match args.renderer.as_deref() {
        Some(requested) => RendererKind::parse(requested)?,
        None => options.resolved_renderer_kind(),
    };
    let game_kind = GameKind::parse(args.game.as_deref().unwrap_or("td"))?;

    if let Some(dir) = &args.list_missions {
        return list_td_missions(dir);
    }
    if let Some(dir) = &args.list_campaign {
        return list_ra_campaign(dir);
    }

    // A frame-capped run is the headless harness path and keeps its old
    // straight-to-game behaviour; otherwise the menu is the default entry point.
    let wants_menu = args.menu
        || (args.game.is_none()
            && args.frames == 0
            && args.mission.is_none()
            && args.mission_dir.is_none()
            && args.scenario.is_none()
            && args.campaign.is_none()
            && args.net_role.is_none()
            && !args.net_lobby
            && args.replay.is_none());
    if wants_menu {
        tracing::info!(
            "starting main menu (renderer={}, {}x{})",
            kind.label(),
            args.width,
            args.height
        );
        let mut shell = LauncherShell::new(
            kind,
            args.width.max(1),
            args.height.max(1),
            args.frames,
            args.rules.clone(),
            options.clone(),
            args.options.clone(),
        );
        let event_loop = EventLoop::new().context("failed to create the winit event loop")?;
        event_loop.set_control_flow(ControlFlow::Wait);
        return event_loop
            .run_app(&mut shell)
            .context("event loop terminated with an error");
    }

    if game_kind == GameKind::RedAlert {
        tracing::info!(
            "starting Red Alert launcher mode (renderer={}, {}x{})",
            kind.label(),
            args.width,
            args.height
        );
        let mut app = RaLauncherApp::new(kind, args.width.max(1), args.height.max(1), args.frames);
        if let Some(dir) = &args.campaign {
            let count = app.load_campaign(dir);
            tracing::info!(
                count,
                dir = %dir.display(),
                "Red Alert campaign missions loaded"
            );
        }
        if let Some(path) = &args.scenario {
            let scenario = cnc_red_alert::Scenario::from_file(path)
                .with_context(|| format!("failed to load scenario from {}", path.display()))?;
            tracing::info!(
                name = %scenario.name,
                map = %format!("{}x{}", scenario.width, scenario.height),
                objects = scenario.objects.len(),
                triggers = scenario.triggers.len(),
                "Red Alert scenario loaded"
            );
            app.scenario_name = scenario.name.clone();
            app.game = cnc_red_alert::Game::from_scenario(&scenario);
            app.mode = RaMode::Playing;
            app.campaign_active = false;
            app.active_scenario_path = Some(path.clone());
            app.briefing_open = !scenario.briefing.is_empty();
        }
        if let Some(path) = &args.replay {
            let replay = cnc_tiberian_dawn::network::Replay::load(path)
                .with_context(|| format!("failed to load replay {}", path.display()))?;
            tracing::info!(path = %path.display(), commands = replay.commands.len(), "RA replay loaded");
            app.start_replay(replay);
        } else if let Some(role) = &args.net_role {
            if args.scenario.is_some() {
                tracing::warn!("--net-role replaces --scenario with a network skirmish");
            }
            let setup = NetworkSetup {
                role: role.clone(),
                join: args.net_join.clone(),
                port: args.net_port,
                peers: args.net_peers.clone(),
                bind: args.net_bind.clone(),
                advertised: args.net_advertised.clone(),
            };
            app.net_replay_path = args.net_replay.clone();
            app.enable_network(&setup)?;
        } else if args.net_lobby {
            app.open_lobby();
        }
        let event_loop = EventLoop::new().context("failed to create the winit event loop")?;
        event_loop.set_control_flow(ControlFlow::Wait);
        return event_loop
            .run_app(&mut app)
            .context("event loop terminated with an error");
    }

    let rules = match &args.rules {
        Some(path) => RulesDatabase::load(path)
            .with_context(|| format!("failed to load rules from {}", path.display()))?,
        None => RulesDatabase::default(),
    };
    tracing::info!(
        "rules loaded ({} type overrides, {} weapons)",
        rules.type_count(),
        rules.weapons.len()
    );
    let mut app = LauncherApp::new(kind, args.width.max(1), args.height.max(1), rules, options)?;
    if let Some(dir) = &args.mission_dir {
        let count = app.load_mission_dir(dir)?;
        tracing::info!(count, dir = %dir.display(), "TD mission directory loaded");
    }
    if let Some(path) = &args.mission {
        app.game
            .load_mission(path)
            .with_context(|| format!("failed to load mission from {}", path.display()))?;
        app.selecting_mission = false;
        app.td_briefing_open =
            !app.game.mission.briefing.is_empty() || !app.game.mission.objectives.is_empty();
        tracing::info!(
            mission = %app.game.mission.name,
            triggers = app.game.mission.triggers.len(),
            objectives = app.game.mission.objectives.len(),
            "campaign mission loaded"
        );
    }

    tracing::info!(
        "starting launcher (renderer={}, {}x{})",
        kind.label(),
        args.width,
        args.height
    );

    let event_loop = EventLoop::new().context("failed to create the winit event loop")?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
        .run_app(&mut app)
        .context("event loop terminated with an error")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu::{scenario_count, CampaignInfo, Screen};

    fn menu_test_campaign(dir: &str, scenarios: usize) -> CampaignInfo {
        CampaignInfo {
            dir: PathBuf::from(dir),
            scenarios,
        }
    }

    fn menu_test_row(index: usize, width: u32) -> (f32, f32, f32, f32) {
        // Mirrors the layout constants used by MainMenu::draw.
        let item_width = (width as f32 - 120.0).clamp(320.0, 560.0);
        (60.0, 224.0 + index as f32 * (48.0 + 10.0), item_width, 48.0)
    }

    #[test]
    fn game_kind_accepts_td_and_ra_aliases() {
        assert_eq!(GameKind::parse("td").unwrap(), GameKind::TiberianDawn);
        assert_eq!(
            GameKind::parse("tiberian-dawn").unwrap(),
            GameKind::TiberianDawn
        );
        assert_eq!(GameKind::parse("RA").unwrap(), GameKind::RedAlert);
        assert_eq!(GameKind::parse("red-alert").unwrap(), GameKind::RedAlert);
        assert!(GameKind::parse("dune").is_err());
    }

    #[test]
    fn ra_finished_match_aborts_to_the_right_screen() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        app.game.outcome = Some(cnc_red_alert::ScenarioOutcome::AlliesWin);
        app.abort_mission();
        assert_eq!(app.mode, RaMode::Setup, "skirmish returns to setup");

        app.start_skirmish();
        app.campaign_active = true;
        app.game.outcome = Some(cnc_red_alert::ScenarioOutcome::SovietsWin);
        app.abort_mission();
        assert_eq!(
            app.mode,
            RaMode::CampaignSelect,
            "campaign returns to mission select"
        );
    }

    #[test]
    fn ra_launcher_bootstraps_a_skirmish_without_a_window() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 30);
        assert_eq!(app.frame_limit, 30);
        assert_eq!(app.mode, RaMode::Setup);

        app.setup.map_size = 48;
        app.cycle_country();
        app.cycle_difficulty();
        app.start_skirmish();
        assert_eq!(app.mode, RaMode::Playing);
        assert_eq!(app.game.map.width, 48);
        assert_eq!(app.game.houses[0].country, Some(Country::Germany));
        assert_eq!(app.game.ai.queue_timer, Fixed16_16::from_int(7));
        assert_eq!(app.game.houses.len(), 2);
        assert!(!app.game.units.is_empty());
        assert!(!app.game.structures.is_empty());
        assert!(app.game.status().allied_power > 0);
    }

    #[test]
    fn campaign_select_loads_and_advances_missions() {
        let dir = std::env::temp_dir().join(format!("ra-campaign-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps");
        std::fs::copy(maps.join("ra_campaign/01.ini"), dir.join("01.ini")).unwrap();
        std::fs::copy(maps.join("ra_campaign/02.ini"), dir.join("02.ini")).unwrap();

        let progress = dir.join("progress.ini");
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.campaign_progress_path = progress.clone();
        assert_eq!(app.load_campaign(&dir), 2);
        assert_eq!(app.mode, RaMode::CampaignSelect);
        assert_eq!(app.campaign[0].name, "01 - Holding the Line");
        assert_eq!(app.campaign[1].name, "02 - Counterstrike");
        assert_eq!(app.campaign[0].width, 24);
        assert_eq!(app.campaign[0].terrain.len(), 24 * 20);
        assert!(app.campaign[0].ore.iter().any(|amount| *amount > 0));
        assert_eq!(app.campaign[0].objectives.len(), 3);
        assert!(app.campaign[0].triggers >= 5);

        app.cycle_campaign(1);
        assert_eq!(app.campaign_index, 1);
        app.cycle_campaign(1);
        assert_eq!(app.campaign_index, 0);

        assert_eq!(app.campaign_completed.len(), 2);
        app.cycle_campaign_difficulty();
        assert_eq!(app.campaign_difficulty, 2);

        assert!(app.start_campaign_mission(0));
        assert_eq!(app.mode, RaMode::Playing);
        assert!(
            app.briefing_open,
            "missions with briefings open the overlay"
        );
        assert_eq!(app.game.map.width, 24);
        assert_eq!(app.game.ai.queue_timer, Fixed16_16::from_int(7));
        let saved = std::fs::read_to_string(&progress).unwrap();
        assert!(saved.contains("Mission=0"), "{saved}");

        app.game.outcome = Some(cnc_red_alert::ScenarioOutcome::AlliesWin);
        app.advance_campaign();
        assert_eq!(app.campaign_index, 1);
        assert_eq!(app.game.map.width, 26);
        assert_eq!(app.game.player_owner, 1);
        assert!(app.campaign_completed[0]);
        let saved = std::fs::read_to_string(&progress).unwrap();
        assert!(saved.contains("Mission=1"), "{saved}");
        assert!(saved.contains("Difficulty=2"), "{saved}");
        assert!(saved.contains("Completed=0"), "{saved}");

        // A fresh launch resumes at the saved mission.
        let mut resumed = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        resumed.campaign_progress_path = progress.clone();
        assert_eq!(resumed.load_campaign(&dir), 2);
        assert_eq!(resumed.campaign_index, 1);
        assert_eq!(resumed.campaign_difficulty, 2);
        assert!(resumed.campaign_completed[0]);

        resumed.start_campaign_mission(1);
        resumed.game.outcome = Some(cnc_red_alert::ScenarioOutcome::SovietsWin);
        resumed.advance_campaign();
        assert_eq!(resumed.mode, RaMode::CampaignComplete);
        assert!(resumed.campaign_completed.iter().all(|done| *done));
        let saved = std::fs::read_to_string(&progress).unwrap();
        assert!(saved.contains("Completed=0,1"), "{saved}");
        assert!(saved.contains("BestTimes=0:0,1:0"), "{saved}");
        assert!(resumed.campaign_best.iter().all(|time| time.is_some()));

        resumed.dismiss_campaign_complete();
        assert_eq!(resumed.mode, RaMode::CampaignSelect);
        assert_eq!(resumed.campaign_index, 0);
        assert!(
            resumed.campaign_completed.iter().all(|done| *done),
            "completion marks persist for replay"
        );
        assert!(progress.exists(), "best times/completion persist");

        // Replaying a cleared mission asks for confirmation first.
        resumed.request_campaign_mission(0);
        assert!(resumed.replay_prompt);
        assert_eq!(resumed.mode, RaMode::CampaignSelect);
        resumed.cancel_replay();
        assert!(!resumed.replay_prompt);
        resumed.request_campaign_mission(0);
        resumed.confirm_replay();
        assert_eq!(resumed.mode, RaMode::Playing);
        assert_eq!(resumed.campaign_index, 0);

        // Filtering and the stats screen operate on the completion set.
        resumed.mode = RaMode::CampaignSelect;
        resumed.campaign_filter = CampaignFilter::Completed;
        assert_eq!(resumed.campaign_visible(), vec![0, 1]);
        resumed.campaign_index = 1;
        resumed.cycle_campaign(1);
        assert_eq!(resumed.campaign_index, 0);
        resumed.cycle_campaign_filter();
        assert_eq!(resumed.campaign_filter, CampaignFilter::All);
        resumed.show_campaign_stats();
        assert_eq!(resumed.mode, RaMode::CampaignStats);
        resumed.dismiss_campaign_stats();
        assert_eq!(resumed.mode, RaMode::CampaignSelect);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn network_move_command_orders_remote_units() {
        let mut game = RaGame::new(32, 32);
        game.set_player_owner(1);
        let Some(unit) = game
            .units
            .iter()
            .find(|unit| unit.owner == 1 && unit.is_alive())
        else {
            return;
        };
        let id = unit.id;
        let cell = unit.cell;
        let target = Vec2::new(
            Fixed16_16::from_f32((cell.x as f32 + 3.0) * RA_TILE_SIZE),
            Fixed16_16::from_f32((cell.y as f32 + 3.0) * RA_TILE_SIZE),
        );
        apply_net_move(&mut game, 1, &[id], target);
        let unit = game.units.iter().find(|unit| unit.id == id).unwrap();
        assert!(
            !unit.path.is_empty() || unit.cell != cell,
            "a remote move command should order a path for the unit"
        );
    }

    #[test]
    fn enemy_target_picks_enemy_units_only() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        let enemy = app
            .game
            .units
            .iter()
            .find(|unit| unit.owner == 1 && unit.is_alive())
            .map(|unit| (unit.id, unit.position));
        let Some((enemy_id, enemy_position)) = enemy else {
            return;
        };
        assert_eq!(
            app.enemy_target_at(enemy_position, 0),
            Some(enemy_id),
            "enemy under the cursor is a target"
        );
        let own = app
            .game
            .units
            .iter()
            .find(|unit| unit.owner == 0 && unit.is_alive())
            .map(|unit| (unit.id, unit.position));
        if let Some((own_id, own_position)) = own {
            assert_ne!(
                app.enemy_target_at(own_position, 0),
                Some(own_id),
                "own units are never attack targets"
            );
        }
    }

    #[test]
    fn network_peer_timeout_reports_loss_once() {
        fn free_port() -> u16 {
            let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
            socket.local_addr().unwrap().port()
        }
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: free_port(),
            peers: vec!["127.0.0.1:9".to_string()],
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        {
            let net = app.network.as_mut().unwrap();
            net.last_seen
                .insert(1, Instant::now() - Duration::from_secs(30));
        }
        app.poll_network();
        let net = app.network.as_ref().unwrap();
        assert_eq!(net.losses, 1);
        assert!(net.reported.contains(&1));
        assert_eq!(net.session.migration.host, 0, "host stays host");
        assert!(
            app.notice
                .as_ref()
                .is_some_and(|(text, _)| text.contains("lost")),
            "a loss shows an on-screen notice"
        );
        app.poll_network();
        assert_eq!(
            app.network.as_ref().unwrap().losses,
            1,
            "the same peer is not reported twice"
        );
    }

    #[test]
    fn network_join_claims_host_after_timeout() {
        fn free_port() -> u16 {
            let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
            socket.local_addr().unwrap().port()
        }
        let host_port = free_port();
        let join_port = free_port();
        let mut host = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        host.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: host_port,
            peers: vec![format!("127.0.0.1:{join_port}")],
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        let mut join = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        join.enable_network(&NetworkSetup {
            role: "join".to_string(),
            join: Some(format!("127.0.0.1:{host_port}")),
            port: join_port,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        {
            let net = join.network.as_mut().unwrap();
            net.last_seen
                .insert(0, Instant::now() - Duration::from_secs(30));
        }
        join.poll_network();
        let net = join.network.as_ref().unwrap();
        assert_eq!(net.losses, 1);
        assert_eq!(net.session.migration.host, 1, "join wins the election");
        assert!(net.migrations >= 1);
        assert!(net.session.host_address.is_some());
        assert_eq!(host.game.player_owner, 0);
    }

    #[test]
    fn network_lost_peer_rejoins_from_a_new_port() {
        fn free_port() -> u16 {
            let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
            socket.local_addr().unwrap().port()
        }
        let host_port = free_port();
        let old_port = free_port();
        let mut host = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        host.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: host_port,
            peers: vec![format!("127.0.0.1:{old_port}")],
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        {
            let net = host.network.as_mut().unwrap();
            net.last_seen
                .insert(1, Instant::now() - Duration::from_secs(30));
        }
        host.poll_network();
        {
            let net = host.network.as_ref().unwrap();
            assert!(net.reported.contains(&1));
            assert!(!net.session.migration.alive.contains(&1));
        }

        // The same player returns from a fresh local port.
        let peer = UdpSocket::bind("127.0.0.1:0").unwrap();
        let packet = NetPacket::new(
            0,
            0,
            vec![NetCommand::new(
                0,
                1,
                NetCommandKind::Stop { units: Vec::new() },
            )],
        );
        let bytes = packet.encode().unwrap();
        peer.send_to(&bytes, ("127.0.0.1", host_port)).unwrap();
        for _ in 0..100 {
            host.poll_network();
            if host.network.as_ref().unwrap().rejoined > 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        let net = host.network.as_ref().unwrap();
        assert_eq!(net.rejoined, 1);
        assert!(!net.reported.contains(&1));
        assert!(net.session.migration.alive.contains(&1));
        assert_eq!(
            net.session.peers.get(&1).map(|address| address.port()),
            Some(peer.local_addr().unwrap().port()),
            "the peer address is replaced by the new port"
        );

        // The join handshake also revives a lost slot (this is what a real
        // reconnecting client sends before it has any command to transmit).
        assert_eq!(parse_join_handshake(b"CNCRAJOIN 1 1\n"), Some(1));
        assert_eq!(parse_join_handshake(b"CNCRAJOIN 2 1"), None);
        assert_eq!(parse_join_handshake(b"hello"), None);
        {
            let net = host.network.as_mut().unwrap();
            net.last_seen
                .insert(1, Instant::now() - Duration::from_secs(30));
        }
        host.poll_network();
        assert!(host.network.as_ref().unwrap().reported.contains(&1));
        let peer2 = UdpSocket::bind("127.0.0.1:0").unwrap();
        peer2
            .send_to(b"CNCRAJOIN 1 1\n", ("127.0.0.1", host_port))
            .unwrap();
        for _ in 0..100 {
            host.poll_network();
            if host.network.as_ref().unwrap().rejoined > 1 {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        let net = host.network.as_ref().unwrap();
        assert_eq!(net.rejoined, 2);
        assert_eq!(
            net.session.peers.get(&1).map(|address| address.port()),
            Some(peer2.local_addr().unwrap().port())
        );
        assert!(
            host.notice
                .as_ref()
                .is_some_and(|(text, _)| text.contains("reconnected")),
            "a reconnect shows an on-screen notice"
        );
    }

    #[test]
    fn lan_beacons_parse_and_deduplicate() {
        assert_eq!(
            parse_lan_beacon(b"CNCRALAN 1 47870 RA"),
            Some((47870, "RA".to_string()))
        );
        assert_eq!(
            parse_lan_beacon(b"CNCRALAN 1 47871"),
            Some((47871, "RA".to_string()))
        );
        assert_eq!(
            lan_beacon_text(47870, "RA").as_bytes(),
            b"CNCRALAN 1 47870 RA"
        );
        assert_eq!(parse_lan_beacon(b"hello"), None);
        assert_eq!(parse_lan_beacon(b"CNCRALAN 2 47870 RA"), None);
        assert_eq!(parse_lan_beacon(b"CNCRALAN 1 notaport"), None);

        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let mut scan = LanScan {
            socket,
            hosts: Vec::new(),
            selected: 0,
        };
        let first: SocketAddr = "192.168.1.5:5000".parse().unwrap();
        scan.ingest(first, b"CNCRALAN 1 47870 RA");
        scan.ingest(first, b"CNCRALAN 1 47870 RA");
        assert_eq!(scan.hosts.len(), 1, "duplicate beacons are ignored");
        assert_eq!(scan.hosts[0].address, "192.168.1.5:47870");
        let second: SocketAddr = "192.168.1.6:5000".parse().unwrap();
        scan.ingest(second, b"CNCRALAN 1 47871 Bob");
        assert_eq!(scan.hosts.len(), 2);
        assert!(scan.hosts.iter().any(|host| host.label.contains("Bob")));
        scan.selected = 1;
        scan.ingest(first, b"CNCRALAN 1 47872");
        assert_eq!(scan.selected, 1, "selection survives new arrivals");
        assert_eq!(scan.hosts.len(), 3);
    }

    #[test]
    fn network_peers_exchange_move_commands() {
        fn free_port() -> u16 {
            let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
            socket.local_addr().unwrap().port()
        }
        let host_port = free_port();
        let join_port = free_port();
        let mut host = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        host.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: host_port,
            // No --net-peer: the host must learn the joiner dynamically.
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        let mut join = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        assert!(join
            .enable_network(&NetworkSetup {
                role: "join".to_string(),
                join: Some(format!("127.0.0.1:{host_port}")),
                port: join_port,
                peers: Vec::new(),
                bind: "127.0.0.1".to_string(),
                advertised: None,
            })
            .is_ok());
        assert_eq!(host.game.player_owner, 0);
        assert_eq!(join.game.player_owner, 1);
        assert_eq!(
            host.game.ai.owner,
            usize::MAX,
            "AI disabled for network play"
        );

        let unit = join
            .game
            .units
            .iter()
            .find(|unit| unit.owner == 1 && unit.is_alive())
            .map(|unit| (unit.id, unit.cell));
        let Some((id, cell)) = unit else {
            return;
        };
        let target = Vec2::new(
            Fixed16_16::from_f32((cell.x as f32 + 3.0) * RA_TILE_SIZE),
            Fixed16_16::from_f32((cell.y as f32 + 3.0) * RA_TILE_SIZE),
        );
        let net = join.network.as_mut().unwrap();
        net.session.queue_local(NetCommandKind::Move {
            units: vec![id],
            target,
        });
        let (_commands, bytes, targets) = net.session.advance().unwrap();
        assert_eq!(targets.len(), 1, "join knows the host address");
        for peer in targets {
            net.socket.send_to(&bytes, peer).unwrap();
            // Mesh relay/echoes can repeat a packet; it must apply once.
            net.socket.send_to(&bytes, peer).unwrap();
        }
        for _ in 0..100 {
            host.poll_network();
            if host.network.as_ref().unwrap().applied > 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        host.poll_network();
        assert_eq!(host.network.as_ref().unwrap().applied, 1);
        let learned = host
            .network
            .as_ref()
            .unwrap()
            .session
            .peers
            .get(&1)
            .copied();
        assert_eq!(
            learned.map(|address| address.port()),
            Some(join_port),
            "the host registers a peer that dialled in"
        );
        let moved = host.game.units.iter().find(|unit| unit.id == id);
        assert!(
            moved.is_some_and(|unit| !unit.path.is_empty() || unit.cell != cell),
            "host applied the remote move order"
        );
    }

    #[test]
    fn network_build_command_places_remote_structure() {
        let mut game = RaGame::new(32, 32);
        let cell = Vec2i::new(8, 8);
        let location = Vec2::new(
            Fixed16_16::from_f32((cell.x as f32 + 0.5) * RA_TILE_SIZE),
            Fixed16_16::from_f32((cell.y as f32 + 0.5) * RA_TILE_SIZE),
        );
        assert!(apply_net_build(
            &mut game,
            0,
            cnc_core::ClassId::PowerPlant,
            location
        ));
        assert!(game.structures.iter().any(|structure| structure.owner == 0
            && structure.class_id == cnc_core::ClassId::PowerPlant
            && structure.cell == cell));
        assert!(
            !apply_net_build(&mut game, 0, cnc_core::ClassId::PowerPlant, location),
            "the same cell cannot be built on twice"
        );
        assert!(!apply_net_build(
            &mut game,
            0,
            cnc_core::ClassId::VehicleE1,
            location
        ));
    }

    #[test]
    fn scatter_orders_selected_units() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        let player = app.game.player_owner;
        let mut selected = 0;
        for unit in &mut app.game.units {
            let pick = unit.owner == player && unit.is_alive() && selected < 3;
            unit.selected = pick;
            if pick {
                selected += 1;
            }
        }
        assert!(selected > 0);
        app.scatter_selected_units();
        assert_eq!(app.pending_commands.len(), selected);
        assert!(app
            .notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("Scattered")));
        assert_eq!(app.drain_pending_commands(), selected);
        assert!(app
            .game
            .units
            .iter()
            .filter(|unit| unit.selected)
            .any(|unit| !unit.path.is_empty()));
    }

    #[test]
    fn network_power_command_toggles_remote_structure() {
        let mut game = RaGame::new(32, 32);
        let id = game
            .place_structure(0, cnc_core::ClassId::PowerPlant, Vec2i::new(8, 8))
            .unwrap();
        assert!(apply_net_power(&mut game, 0, id, false));
        assert!(
            !game
                .structures
                .iter()
                .find(|structure| structure.id == id)
                .unwrap()
                .powered
        );
        assert!(apply_net_power(&mut game, 0, id, true));
        assert!(
            game.structures
                .iter()
                .find(|structure| structure.id == id)
                .unwrap()
                .powered
        );
        assert!(!apply_net_power(&mut game, 1, id, false), "wrong owner");
    }

    #[test]
    fn network_repair_command_heals_remote_structure() {
        let mut game = RaGame::new(32, 32);
        let id = game
            .place_structure(0, cnc_core::ClassId::PowerPlant, Vec2i::new(8, 8))
            .unwrap();
        let max = game
            .structures
            .iter()
            .find(|structure| structure.id == id)
            .unwrap()
            .max_health;
        if let Some(structure) = game.structures.iter_mut().find(|s| s.id == id) {
            structure.health = max / Fixed16_16::from_int(2);
        }
        assert!(apply_net_repair(&mut game, 0, id));
        assert_eq!(
            game.structures
                .iter()
                .find(|structure| structure.id == id)
                .unwrap()
                .health,
            max
        );
        assert!(!apply_net_repair(&mut game, 0, id), "already repaired");
        assert!(!apply_net_repair(&mut game, 1, id), "wrong owner");
    }

    #[test]
    fn network_deploy_command_replaces_remote_mcv() {
        let mut game = RaGame::new(32, 32);
        let cell = Vec2i::new(25, 25);
        let mcv = game.spawn_unit(0, cnc_core::ClassId::VehicleE7, cell);
        assert!(apply_net_deploy(&mut game, 0, mcv));
        assert!(game.unit(mcv).is_none());
        assert!(game.structures.iter().any(|structure| {
            structure.class_id == cnc_core::ClassId::ConstructionYard
                && structure.cell == cell
                && structure.owner == 0
        }));
        assert!(!apply_net_deploy(&mut game, 0, mcv), "already deployed");
    }

    #[test]
    fn deploy_selected_mcv_queues_locally() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        let cell = Vec2i::new(26, 26);
        let mcv = app.game.spawn_unit(0, cnc_core::ClassId::VehicleE7, cell);
        for unit in &mut app.game.units {
            unit.selected = unit.id == mcv;
        }
        app.deploy_selected_mcv();
        assert_eq!(app.pending_commands.len(), 1);
        assert_eq!(app.drain_pending_commands(), 1);
        assert!(app.game.unit(mcv).is_none());
        assert!(app.game.structures.iter().any(|structure| {
            structure.class_id == cnc_core::ClassId::ConstructionYard && structure.cell == cell
        }));
    }

    #[test]
    fn network_sell_command_removes_remote_structure() {
        let mut game = RaGame::new(32, 32);
        let id = game
            .place_structure(0, cnc_core::ClassId::PowerPlant, Vec2i::new(8, 8))
            .unwrap();
        assert!(apply_net_sell(&mut game, 0, id));
        assert!(!game.structures.iter().any(|structure| structure.id == id));
        assert!(!apply_net_sell(&mut game, 0, id), "already sold");
    }

    #[test]
    fn selected_structure_sells_locally() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        let id = app
            .game
            .structures
            .iter()
            .find(|structure| structure.owner == 0 && structure.is_alive())
            .map(|structure| structure.id)
            .unwrap();
        app.selected_structure = Some(id);
        app.sell_selected_structure();
        assert_eq!(app.pending_commands.len(), 1);
        assert!(app
            .game
            .structures
            .iter()
            .any(|structure| structure.id == id));
        assert_eq!(app.drain_pending_commands(), 1);
        assert!(!app
            .game
            .structures
            .iter()
            .any(|structure| structure.id == id));
        assert!(app.selected_structure.is_none());
    }

    #[test]
    fn network_produce_command_queues_remote_unit() {
        let mut game = RaGame::new(32, 32);
        let before = game.houses[0].total_queued();
        assert!(apply_net_produce(
            &mut game,
            0,
            cnc_core::ClassId::VehicleE2
        ));
        assert_eq!(game.houses[0].total_queued(), before + 1);
        assert_eq!(
            game.houses[0].last_queued().unwrap().class_id,
            cnc_core::ClassId::VehicleE2
        );
        // Default build times from unit_build_time (mirrors old ra_unit_build_seconds)
        assert_eq!(
            cnc_red_alert::unit_build_time(cnc_core::ClassId::InfantryE1),
            4
        );
        assert_eq!(
            cnc_red_alert::unit_build_time(cnc_core::ClassId::VehicleE1),
            6
        );
        assert_eq!(
            cnc_red_alert::unit_build_time(cnc_core::ClassId::VehicleE2),
            8
        );
        assert_eq!(cnc_red_alert::unit_build_time(cnc_core::ClassId::ShipE1), 9);
        assert!(!apply_net_produce(
            &mut game,
            9,
            cnc_core::ClassId::InfantryE1
        ));
    }

    #[test]
    fn ra_produce_unit_queues_locally() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        let before = app.game.houses[0].total_queued();
        app.produce_unit(cnc_core::ClassId::InfantryE1);
        assert_eq!(app.pending_commands.len(), 1);
        assert_eq!(app.drain_pending_commands(), 1);
        assert_eq!(app.game.houses[0].total_queued(), before + 1);
        assert!(app.network.is_none());
    }

    #[test]
    fn network_lobby_edits_and_starts_both_roles() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.open_lobby();
        assert_eq!(app.mode, RaMode::NetworkLobby);
        assert_eq!(app.lobby.role, NetRole::Host);
        app.lobby.toggle_role();
        assert_eq!(app.lobby.role, NetRole::Join);
        app.lobby.toggle_role();

        let original = app.lobby.peer_address.clone();
        app.lobby.backspace();
        assert_eq!(app.lobby.peer_address.len(), original.len() - 1);
        app.lobby.push_char('7');
        assert!(app.lobby.peer_address.ends_with('7'));
        app.lobby.push_char('x');
        assert!(app.lobby.peer_address.ends_with('7'), "letters are ignored");
        let port = app.lobby.host_port;
        app.lobby.adjust_port(1);
        assert_eq!(app.lobby.host_port, port + 1);

        // A bad address keeps the lobby open with an inline error.
        app.lobby.peer_address = "not-an-address".to_string();
        let _ = app.start_lobby();
        assert_eq!(app.mode, RaMode::NetworkLobby);
        assert!(app.lobby_error.is_some());

        // A host with an ephemeral port starts a network game with house 0.
        app.lobby.peer_address = String::new();
        app.lobby.host_port = 0;
        app.lobby_error = None;
        let _ = app.start_lobby();
        assert_eq!(app.mode, RaMode::Playing);
        assert_eq!(app.game.player_owner, 0);
        assert_eq!(app.game.ai.owner, usize::MAX);
        assert!(app.network.is_some());
        app.abort_mission();
        assert_eq!(app.mode, RaMode::Setup);
        assert!(app.network.is_none());

        // Joining starts house 1 and remembers the host address.
        app.open_lobby();
        app.lobby.toggle_role();
        app.lobby.join_address = "127.0.0.1:9".to_string();
        app.lobby.join_port = 0;
        let _ = app.start_lobby();
        assert_eq!(app.mode, RaMode::Playing);
        assert_eq!(app.game.player_owner, 1);
        let net = app.network.as_ref().unwrap();
        assert_eq!(net.session.local_player, 1);
        assert_eq!(
            net.session.peers.get(&0).unwrap().ip().to_string(),
            "127.0.0.1"
        );
    }

    #[test]
    fn td_pause_menu_resumes_and_returns_to_selector() {
        let mut app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
        app.load_mission_dir(&maps).unwrap();
        assert!(app.activate_selected_mission().unwrap());
        assert!(!app.selecting_mission);

        app.enter_td_pause();
        assert!(app.td_paused);
        app.td_pause_choice = 0;
        app.activate_td_pause_choice();
        assert!(!app.td_paused);

        app.enter_td_pause();
        app.td_pause_choice = 3;
        app.activate_td_pause_choice();
        assert!(!app.td_paused);
        assert!(app.selecting_mission);
    }

    #[test]
    fn td_campaign_completion_shows_final_screen() {
        let progress = std::env::temp_dir().join(format!("td-done-{}.dat", std::process::id()));
        let _ = std::fs::remove_file(&progress);
        let mut app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        app.td_progress_path = progress.clone();
        let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
        assert_eq!(app.load_mission_dir(&maps).unwrap(), 9);
        app.td_completed = vec![true; 9];
        app.td_completed[8] = false;
        app.mission_index = 8;
        assert!(app.activate_selected_mission().unwrap());
        app.game.mission.outcome = Some(MissionOutcome::Victory);
        app.game.elapsed = Fixed16_16::from_int(42);
        app.finish_td_mission();
        assert!(app.td_complete, "last victory opens the campaign screen");
        assert!(!app.selecting_mission);
        assert!(app.td_completed.iter().all(|done| *done));
        app.dismiss_td_campaign_complete();
        assert!(!app.td_complete);
        assert!(app.selecting_mission);
        let _ = std::fs::remove_file(&progress);
    }

    #[test]
    fn td_mission_finish_returns_to_selector_and_advances() {
        let progress = std::env::temp_dir().join(format!("td-finish-{}.dat", std::process::id()));
        let _ = std::fs::remove_file(&progress);
        let mut app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        app.td_progress_path = progress.clone();
        let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
        assert_eq!(app.load_mission_dir(&maps).unwrap(), 9);
        app.mission_index = 0;
        assert!(app.activate_selected_mission().unwrap());
        app.game.mission.outcome = Some(MissionOutcome::Victory);
        app.game.elapsed = Fixed16_16::from_int(30);
        app.finish_td_mission();
        assert!(app.selecting_mission);
        assert_eq!(app.mission_index, 1, "a win advances to the next mission");
        assert!(app.td_completed[0]);
        assert_eq!(app.td_best[0], Some(30));

        assert!(app.activate_selected_mission().unwrap());
        app.game.mission.outcome = Some(MissionOutcome::Defeat);
        app.finish_td_mission();
        assert!(app.selecting_mission);
        assert_eq!(app.mission_index, 1, "a loss retries the same mission");
        assert!(!app.td_completed[1]);
        let _ = std::fs::remove_file(&progress);
    }

    #[test]
    fn td_mission_select_lists_and_loads() {
        let dir = std::env::temp_dir().join(format!("td-select-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
        std::fs::copy(maps.join("01.ini"), dir.join("01.ini")).unwrap();
        std::fs::copy(maps.join("02.ini"), dir.join("02.ini")).unwrap();

        let progress = dir.join("td_progress.dat");
        let mut app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        app.td_progress_path = progress.clone();
        assert_eq!(app.load_mission_dir(&dir).unwrap(), 2);
        assert!(app.selecting_mission);
        assert_eq!(app.mission_choices[0].name, "GDI 01 - Holding the Line");
        assert!(!app.mission_choices[0].spawns.is_empty());
        assert!(app.mission_choices[0]
            .spawns
            .iter()
            .all(|spawn| spawn.cell.x >= 0 && spawn.cell.x < 64));
        assert!(app.mission_choices[0]
            .spawns
            .iter()
            .any(|spawn| spawn.owner == 1 && !spawn.structure));
        assert!(app.mission_choices[0].objectives >= 2);
        app.cycle_mission(1);
        assert_eq!(app.mission_index, 1);
        assert!(app.activate_selected_mission().unwrap());
        assert!(!app.selecting_mission);
        assert!(app.td_briefing_open, "missions open the briefing overlay");
        assert_eq!(app.game.mission.name, "Nod 01 - Breakout");
        assert_eq!(app.game.player_owner, 1, "Nod mission controls Nod");
        assert!(
            app.game.select_all_owned(1) > 0,
            "Nod units are selectable in a Nod mission"
        );

        // Winning records completion + best time, keeping the faster run.
        app.game.mission.outcome = Some(MissionOutcome::Victory);
        app.game.elapsed = cnc_core::Fixed16_16::from_int(80);
        app.record_mission_result();
        assert!(app.td_completed[1]);
        assert_eq!(app.td_best[1], Some(80));
        app.td_recorded = false;
        app.game.elapsed = cnc_core::Fixed16_16::from_int(60);
        app.record_mission_result();
        assert_eq!(app.td_best[1], Some(60));
        app.td_recorded = false;
        app.game.elapsed = cnc_core::Fixed16_16::from_int(90);
        app.record_mission_result();
        assert_eq!(app.td_best[1], Some(60));
        let saved = std::fs::read_to_string(&progress).unwrap();
        assert!(saved.contains("Completed=1"), "{saved}");
        assert!(saved.contains("1:60"), "{saved}");

        // A fresh launcher resumes the saved completion/best time.
        let mut resumed = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        resumed.td_progress_path = progress.clone();
        assert_eq!(resumed.load_mission_dir(&dir).unwrap(), 2);
        assert!(resumed.td_completed[1]);
        assert_eq!(resumed.td_best[1], Some(60));

        resumed.td_filter = CampaignFilter::Completed;
        assert_eq!(resumed.td_visible(), vec![1]);
        resumed.td_filter = CampaignFilter::Incomplete;
        assert_eq!(resumed.td_visible(), vec![0]);
        resumed.cycle_td_filter();
        assert_eq!(resumed.td_filter, CampaignFilter::Completed);
        resumed.cycle_td_filter();
        assert_eq!(resumed.td_filter, CampaignFilter::All);
        resumed.td_stats_open = true;
        assert!(resumed.td_stats_open);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn campaign_flow_advances_and_carries_credits() {
        // Two-mission scripted run exercising score display, carry-over write,
        // file round-trip re-read at next mission start, and the one-shot
        // semantics that prevent double-apply.
        let dir = std::env::temp_dir().join(format!("td-carr-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
        std::fs::copy(maps.join("01.ini"), dir.join("01.ini")).unwrap();
        std::fs::copy(maps.join("03.ini"), dir.join("02.ini")).unwrap();

        let progress = dir.join("td_progress.dat");
        let mut app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        app.td_progress_path = progress.clone();
        assert_eq!(app.load_mission_dir(&dir).unwrap(), 2);

        // Mission 1 – load, give a distinctive credit amount, finish with victory.
        app.mission_index = 0;
        assert!(app.activate_selected_mission().unwrap());
        let owner = app.game.player_owner;
        app.game.houses[owner as usize].credits = 4321;
        // Ensure the scenario actually starts with at least one surviving unit.
        assert!(!app.game.units.is_empty());

        app.show_td_score();
        assert!(
            app.td_score.is_some(),
            "score screen must be shown after mission end"
        );

        app.game.mission.outcome = Some(MissionOutcome::Victory);
        app.game.elapsed = Fixed16_16::from_int(45);
        app.finish_td_mission();

        // Victory advances to next mission and records completion.
        assert!(app.selecting_mission);
        assert_eq!(app.mission_index, 1);
        assert!(app.td_completed[0]);

        // Carry-over file was written at mission end.
        let carry_path = app.td_progress_path.with_file_name("td-carryover.bin");
        let recorded = Carryover::load_from_file(&carry_path)
            .expect("carry-over file must exist after a victorious mission");
        assert_eq!(recorded.credits, 4321);
        assert!(
            !recorded.units.is_empty(),
            "carry-over must capture surviving units"
        );

        // Simulate a selector reload / fresh launch by clearing the in-memory cache.
        // The next activation must re-read the file, not panic, and apply exactly once.
        app.td_carryover = None;

        // Activate mission 2 – file round-trip.
        assert!(app.activate_selected_mission().unwrap());
        // Carry-over is applied to the owner stored in the file, which may differ
        // from the current player_owner if scenarios use different house indices.
        let credits_after = app.game.houses[recorded.owner as usize].credits;
        assert_eq!(
            credits_after,
            recorded
                .credits
                .min(cnc_tiberian_dawn::carryover::CREDITS_CAP),
            "credits must be carried from the file"
        );

        // All carried unit types must be present in the new game.
        for carried in &recorded.units {
            let found = app
                .game
                .units
                .iter()
                .any(|u| u.owner == recorded.owner && u.unit_type == carried.unit_type);
            assert!(
                found,
                "carried unit type {:?} must be deployed on mission start",
                carried.unit_type
            );
        }

        // One-shot semantics: a fresh launch must not re-apply the same carry-over.
        let mut resumed = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        resumed.td_progress_path = progress.clone();
        assert_eq!(resumed.load_mission_dir(&dir).unwrap(), 2);
        resumed.mission_index = 1;
        assert!(resumed.activate_selected_mission().unwrap());
        // The carry-over file should be gone after the first activation; a fresh
        // launch must use the scenario defaults and not re-load the old carry-over.
        assert!(
            !carry_path.exists(),
            "carry-over artifact must be consumed after first apply"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn queued_game_commands_are_routed_into_the_game() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        let player = app.game.player_owner;
        let (id, cell) = app
            .game
            .units
            .iter()
            .find(|unit| unit.owner == player)
            .map(|unit| (unit.id, unit.cell))
            .unwrap();
        let target = Vec2::new(
            Fixed16_16::from_f32((cell.x as f32 + 3.5) * RA_TILE_SIZE),
            Fixed16_16::from_f32((cell.y as f32 + 0.5) * RA_TILE_SIZE),
        );
        app.pending_commands.push(GameCommand::Move {
            owner: player,
            units: vec![id],
            target,
        });
        assert_eq!(app.drain_pending_commands(), 1);
        assert!(app.pending_commands.is_empty());
        assert!(
            !app.game.unit(id).unwrap().path.is_empty(),
            "command should set a movement path"
        );
    }

    #[test]
    fn pause_menu_restarts_and_aborts() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        assert_eq!(app.mode, RaMode::Playing);
        let scenario = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/ra_campaign/01.ini");
        app.active_scenario_path = Some(scenario);
        app.campaign_active = false;

        app.enter_pause();
        assert!(app.paused);
        app.pause_choice = 1;
        app.activate_pause_choice();
        assert!(!app.paused);
        assert_eq!(app.mode, RaMode::Playing);
        assert_eq!(app.game.map.width, 24, "restart reloaded the scenario");

        app.enter_pause();
        app.pause_choice = 2;
        app.activate_pause_choice();
        assert_eq!(app.mode, RaMode::Setup);
        assert!(!app.paused);
    }

    #[test]
    fn pause_menu_saves_and_loads_the_game() {
        let dir = std::env::temp_dir().join(format!("ra-pause-save-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("slot.sav");
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_skirmish();
        let width = app.game.map.width;
        let units = app.game.units.len();
        assert!(app.save_game_to(&path));
        assert!(path.exists());
        assert!(app
            .notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("saved")));

        app.game = RaGame::new(32, 32);
        assert!(app.load_game_from(&path));
        assert_eq!(app.mode, RaMode::Playing);
        assert_eq!(app.game.map.width, width);
        assert_eq!(app.game.units.len(), units);
        assert!(app
            .notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("loaded")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn chat_lines_parse_and_log() {
        assert_eq!(
            parse_chat_line(b"CNCCHAT 1 1 hello there"),
            Some((1, "hello there".to_string()))
        );
        assert_eq!(parse_chat_line(b"CNCCHAT 1 1 "), None, "empty text");
        assert_eq!(parse_chat_line(b"CNCCHAT 1 x hi"), None, "bad player");
        assert_eq!(parse_chat_line(b"hello"), None);

        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: 0,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        app.chat_open = true;
        app.chat_input = "  rally north  ".to_string();
        app.send_chat();
        assert!(!app.chat_open);
        assert!(app.chat_input.is_empty());
        assert_eq!(app.chat_log.last(), Some(&(0, "rally north".to_string())));
        let len = app.chat_log.len();
        app.chat_input.clear();
        app.send_chat();
        assert_eq!(app.chat_log.len(), len, "empty messages are ignored");
    }

    #[test]
    fn td_repair_selected_building_costs_and_notices() {
        let mut app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        let player = app.game.player_owner;
        let index = app
            .game
            .buildings
            .iter()
            .position(|building| building.owner == player && building.is_alive())
            .unwrap();
        let max = app.game.buildings[index].max_health;
        app.game.buildings[index].health = max / Fixed16_16::from_int(2);
        let credits = app.game.houses[player as usize].credits;
        app.selected_building = Some(index);
        app.repair_selected_building();
        assert_eq!(app.game.buildings[index].health, max);
        assert!(app.game.houses[player as usize].credits < credits);
        assert!(app
            .td_notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("repaired")));
        // Repairing a full building is a no-op.
        app.repair_selected_building();
        assert!(app
            .td_notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("No repair")));
    }

    #[test]
    fn td_sell_selected_building_refunds_and_notices() {
        let mut app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        let player = app.game.player_owner;
        let index = app
            .game
            .buildings
            .iter()
            .position(|building| building.owner == player && building.is_alive())
            .unwrap();
        let credits = app.game.houses[player as usize].credits;
        app.selected_building = Some(index);
        app.sell_selected_building();
        assert!(app.selected_building.is_none());
        assert!(app.game.houses[player as usize].credits > credits);
        assert!(!app.game.buildings[index].is_alive());
        assert!(app
            .td_notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("sold")));
    }

    #[test]
    fn control_groups_store_and_recall() {
        let mut ra = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        ra.start_skirmish();
        let player = ra.game.player_owner;
        let id = ra
            .game
            .units
            .iter()
            .find(|unit| unit.owner == player && unit.is_alive())
            .map(|unit| unit.id)
            .unwrap();
        for unit in &mut ra.game.units {
            unit.selected = unit.id == id;
        }
        ra.store_control_group(0);
        for unit in &mut ra.game.units {
            unit.selected = false;
        }
        assert!(ra.recall_control_group(0));
        assert_eq!(ra.game.selected_count(player), 1);
        assert!(ra.game.unit(id).unwrap().selected);
        assert!(!ra.recall_control_group(1), "empty group");

        let mut td = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        let player = td.game.player_owner;
        let index = td
            .game
            .units
            .iter()
            .enumerate()
            .find(|(_, unit)| unit.owner == player && unit.is_alive())
            .map(|(index, _)| index)
            .unwrap();
        for (i, unit) in td.game.units.iter_mut().enumerate() {
            unit.is_selected = i == index;
        }
        td.store_td_control_group(2);
        for unit in &mut td.game.units {
            unit.is_selected = false;
        }
        assert!(td.recall_td_control_group(2));
        assert!(td.game.units[index].is_selected);
        assert!(!td.recall_td_control_group(0));
    }

    #[test]
    fn edge_pan_detects_edges_and_moves_camera() {
        assert_eq!(
            edge_pan_direction(2.0, 300.0, 800.0, 600.0, 8.0),
            (-1.0, 0.0)
        );
        assert_eq!(
            edge_pan_direction(400.0, 598.0, 800.0, 600.0, 8.0),
            (0.0, 1.0)
        );
        assert_eq!(
            edge_pan_direction(400.0, 300.0, 800.0, 600.0, 8.0),
            (0.0, 0.0)
        );
        assert_eq!(
            edge_pan_direction(-5.0, 300.0, 800.0, 600.0, 8.0),
            (0.0, 0.0),
            "outside the window"
        );

        let mut ra = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        ra.start_skirmish();
        let before = ra.camera.position.x;
        ra.cursor = Some(winit::dpi::PhysicalPosition::new(2.0, 120.0));
        ra.apply_ra_edge_pan(0.1);
        assert!(ra.camera.position.x < before);

        let mut td = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        let before = td.camera.position.x;
        td.cursor = Some(winit::dpi::PhysicalPosition::new(318.0, 120.0));
        td.apply_td_edge_pan(0.1);
        assert!(td.camera.position.x > before);
    }

    #[test]
    fn save_slot_label_reports_age_and_empty() {
        let dir = std::env::temp_dir().join(format!("slot-label-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("slot.sav");
        assert_eq!(save_slot_label(&path), "empty");
        std::fs::write(&path, b"save").unwrap();
        let label = save_slot_label(&path);
        assert!(
            label.ends_with("s ago") || label.ends_with("m ago"),
            "{label}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn next_incomplete_skips_and_wraps() {
        let completed = [true, false, true, false];
        assert_eq!(next_incomplete(&completed, 0), Some(1));
        assert_eq!(next_incomplete(&completed, 1), Some(3));
        assert_eq!(next_incomplete(&completed, 3), Some(1));
        assert_eq!(next_incomplete(&[true; 3], 0), None);
        assert_eq!(next_incomplete(&[], 0), None);
    }

    #[test]
    fn production_percent_handles_progress() {
        assert_eq!(
            production_percent(Fixed16_16::from_int(5), Fixed16_16::from_int(10)),
            50
        );
        assert_eq!(
            production_percent(Fixed16_16::from_int(10), Fixed16_16::from_int(10)),
            100
        );
        assert_eq!(
            production_percent(Fixed16_16::from_int(20), Fixed16_16::from_int(10)),
            100,
            "clamped"
        );
        assert_eq!(production_percent(Fixed16_16::ZERO, Fixed16_16::ZERO), 0);
    }

    #[test]
    fn health_bars_report_fraction_and_color() {
        assert_eq!(
            health_fraction(Fixed16_16::from_int(50), Fixed16_16::from_int(100)),
            0.5
        );
        assert_eq!(
            health_fraction(Fixed16_16::from_int(0), Fixed16_16::from_int(100)),
            0.0
        );
        assert_eq!(
            health_fraction(Fixed16_16::from_int(10), Fixed16_16::ZERO),
            0.0,
            "zero maximum is treated as empty"
        );
        assert_eq!(health_bar_color(0.7), [0.35, 0.90, 0.45, 1.0]);
        assert_eq!(health_bar_color(0.3), [0.95, 0.82, 0.30, 1.0]);
        assert_eq!(health_bar_color(0.1), [0.95, 0.35, 0.30, 1.0]);
    }

    #[test]
    fn replay_playback_applies_frames_in_order() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.start_replay(cnc_tiberian_dawn::network::Replay::new(2, Vec::new()));
        assert_eq!(app.mode, RaMode::Playing);
        assert_eq!(app.game.player_owner, 0);
        assert!(app.replay.is_some());
        let unit = app
            .game
            .units
            .iter()
            .find(|unit| unit.owner == 0 && unit.is_alive())
            .map(|unit| unit.id)
            .unwrap();
        app.replay.as_mut().unwrap().commands = vec![
            NetCommand::new(0, 0, NetCommandKind::Stop { units: vec![unit] }),
            NetCommand::new(3, 0, NetCommandKind::Stop { units: vec![unit] }),
        ];
        assert_eq!(app.step_replay_frame(), 1, "frame 0 applies");
        assert_eq!(app.replay.as_ref().unwrap().frame, 1);
        assert!(!app.replay.as_ref().unwrap().done);
        assert_eq!(app.step_replay_frame(), 0);
        assert_eq!(app.step_replay_frame(), 0);
        assert_eq!(app.step_replay_frame(), 1, "frame 3 applies");
        assert!(app.replay.as_ref().unwrap().done);
    }

    #[test]
    fn network_replay_saves_command_log() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: 0,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        {
            let net = app.network.as_mut().unwrap();
            net.session
                .queue_local(NetCommandKind::Stop { units: Vec::new() });
            net.last_tick = Instant::now() - Duration::from_secs(1);
        }
        app.poll_network();
        let path = std::env::temp_dir().join(format!("cnc-replay-{}.rep", std::process::id()));
        let _ = std::fs::remove_file(&path);
        assert!(app.save_network_replay_to(&path));
        assert!(path.exists());
        let replay = cnc_tiberian_dawn::network::Replay::load(&path).unwrap();
        assert_eq!(replay.players, 2);
        assert_eq!(replay.commands.len(), 1);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_slots_cycle_and_use_distinct_paths() {
        let zero = ra_save_path_for(0);
        let one = ra_save_path_for(1);
        let two = ra_save_path_for(2);
        assert_ne!(zero, one);
        assert_ne!(one, two);
        assert!(zero.to_string_lossy().ends_with("ra_mission.sav"));
        assert!(one.to_string_lossy().ends_with("ra_mission2.sav"));

        let mut ra = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        ra.start_skirmish();
        assert_eq!(ra.save_slot, 0);
        ra.cycle_save_slot();
        assert_eq!(ra.save_slot, 1);
        assert!(ra
            .notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("Save slot 2/3")));
        ra.cycle_save_slot();
        ra.cycle_save_slot();
        assert_eq!(ra.save_slot, 0, "wraps around");
        ra.enter_pause();
        ra.pause_choice = 5;
        ra.activate_pause_choice();
        assert_eq!(ra.save_slot, 1, "pause menu cycles the slot");

        let mut td = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        assert_eq!(td.save_slot, 0);
        td.cycle_td_save_slot();
        assert_eq!(td.save_slot, 1);
        assert!(td
            .td_notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("Save slot 2/3")));
        td.enter_td_pause();
        td.td_pause_choice = 4;
        td.activate_td_pause_choice();
        assert_eq!(td.save_slot, 2);
    }

    #[test]
    fn game_speed_adjusts_and_clamps() {
        let mut ra = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        ra.start_skirmish();
        assert_eq!(ra.game_speed, 1.0);
        ra.adjust_ra_speed(0.25);
        assert_eq!(ra.game_speed, 1.25);
        ra.adjust_ra_speed(10.0);
        assert_eq!(ra.game_speed, 3.0);
        ra.adjust_ra_speed(-10.0);
        assert_eq!(ra.game_speed, 0.25);

        // Network games keep speed locked (lockstep).
        ra.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: 0,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        assert_eq!(ra.game_speed, 1.0, "network start resets speed");
        ra.adjust_ra_speed(0.25);
        assert_eq!(ra.game_speed, 1.0, "speed is locked in network games");
        assert!(ra
            .notice
            .as_ref()
            .is_some_and(|(text, _)| text.contains("locked")));

        let mut td = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        td.adjust_td_speed(0.5);
        assert_eq!(td.game_speed, 1.5);
        td.adjust_td_speed(10.0);
        assert_eq!(td.game_speed, 3.0);
    }

    #[test]
    fn camera_bookmarks_save_and_jump() {
        assert_eq!(bookmark_slot("1"), Some(0));
        assert_eq!(bookmark_slot("4"), Some(3));
        assert_eq!(bookmark_slot("5"), None);

        let mut ra = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        ra.start_skirmish();
        let saved = Vec2::new(Fixed16_16::from_int(300), Fixed16_16::from_int(400));
        ra.camera.position = saved;
        ra.save_ra_bookmark(0);
        ra.camera.position = Vec2::new(Fixed16_16::from_int(10), Fixed16_16::from_int(10));
        assert!(ra.jump_ra_bookmark(0));
        assert_eq!(ra.camera.position, saved);
        assert!(!ra.jump_ra_bookmark(1), "empty slot");

        let mut td = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        let saved = Vec2::new(Fixed16_16::from_int(120), Fixed16_16::from_int(240));
        td.camera.position = saved;
        td.save_td_bookmark(2);
        td.camera.position = Vec2::new(Fixed16_16::from_int(1), Fixed16_16::from_int(1));
        assert!(td.jump_td_bookmark(2));
        assert_eq!(td.camera.position, saved);
        assert!(!td.jump_td_bookmark(0));
    }

    #[test]
    fn minimap_clicks_center_the_camera() {
        let mut ra = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        ra.start_skirmish();
        // The minimap is laid out in unzoomed HUD space; pin the zoom so the
        // expected origin math below is readable.
        ra.camera.zoom = Fixed16_16::ONE;
        let zoom = ra.camera.zoom.to_f32().max(1.0e-6);
        let view_w = ra.camera.viewport_width as f32 / zoom;
        let view_h = ra.camera.viewport_height as f32 / zoom;
        let left = ra.camera.position.x.to_f32() - view_w * 0.5;
        let top = ra.camera.position.y.to_f32() - view_h * 0.5;
        let (_, scale) = minimap_layout(ra.game.map.width, ra.game.map.height, 150.0);
        let (x, y, _, _) = minimap_origin(
            ra.game.map.width,
            ra.game.map.height,
            left,
            top,
            view_w,
            view_h,
            true,
        );
        let target = Vec2::new(
            Fixed16_16::from_f32(x + 5.5 * scale),
            Fixed16_16::from_f32(y + 6.5 * scale),
        );
        assert!(ra.ra_minimap_click(target));
        assert!((ra.camera.position.x.to_f32() - 5.5 * RA_TILE_SIZE).abs() < 0.01);
        assert!((ra.camera.position.y.to_f32() - 6.5 * RA_TILE_SIZE).abs() < 0.01);
        assert!(!ra.ra_minimap_click(Vec2::new(Fixed16_16::ZERO, Fixed16_16::ZERO)));

        let mut td = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        td.camera.zoom = Fixed16_16::ONE;
        let zoom = td.camera.zoom.to_f32().max(1.0e-6);
        let view_w = td.camera.viewport_width as f32 / zoom;
        let view_h = td.camera.viewport_height as f32 / zoom;
        let left = td.camera.position.x.to_f32() - view_w * 0.5;
        let top = td.camera.position.y.to_f32() - view_h * 0.5;
        let (_, scale) = minimap_layout(td.game.map.width, td.game.map.height, 150.0);
        let (x, y, _, _) = minimap_origin(
            td.game.map.width,
            td.game.map.height,
            left,
            top,
            view_w,
            view_h,
            false,
        );
        let target = Vec2::new(
            Fixed16_16::from_f32(x + 3.5 * scale),
            Fixed16_16::from_f32(y + 4.5 * scale),
        );
        assert!(td.td_minimap_click(target));
        assert!(
            (td.camera.position.x.to_f32() - 3.5 * cnc_tiberian_dawn::map::TILE_SIZE).abs() < 0.01
        );
        assert!(
            (td.camera.position.y.to_f32() - 4.5 * cnc_tiberian_dawn::map::TILE_SIZE).abs() < 0.01
        );
    }

    #[test]
    fn minimap_layout_scales_to_the_panel() {
        let (step, scale) = minimap_layout(64, 64, 150.0);
        assert_eq!(step, 2);
        assert!((scale * 64.0 - 150.0).abs() < 0.01);
        let (step, scale) = minimap_layout(16, 16, 150.0);
        assert_eq!(step, 1);
        assert!((scale * 16.0 - 150.0).abs() < 0.01);
        assert_eq!(minimap_layout(128, 64, 150.0).0, 3);
        assert_eq!(minimap_layout(1, 1, 150.0).0, 1);
    }

    #[test]
    fn ra_setup_preview_maps_are_playable() {
        for size in RA_MAP_SIZES {
            let map = ra_setup_preview(size);
            assert_eq!(map.width, size);
            assert_eq!(map.height, size);
            assert_eq!(map.terrain.len() as u32, size * size);
            assert_eq!(map.ore.len() as u32, size * size);
            assert!(
                map.terrain.iter().any(|terrain| terrain.is_water()),
                "preview map {size} should have its river"
            );
            assert!(
                map.ore.iter().any(|amount| *amount > 0),
                "preview map {size} should have ore"
            );
        }
    }

    #[test]
    fn ra_setup_cycles_sizes_and_difficulty() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        assert_eq!(app.setup.map_size, 64);
        app.cycle_map_size(true);
        assert_eq!(app.setup.map_size, 96);
        app.cycle_map_size(false);
        app.cycle_map_size(false);
        assert_eq!(app.setup.map_size, 48);
        assert_eq!(app.setup.difficulty, 1);
        app.cycle_difficulty();
        assert_eq!(app.setup.difficulty, 2);

        app.cycle_side();
        assert_eq!(app.setup.side, 1);
        assert_eq!(app.setup.country, Country::Russia);
        app.start_skirmish();
        assert_eq!(app.game.player_owner, 1);
        assert_eq!(app.game.ai.owner, 0);
        assert_eq!(app.game.houses[1].country, Some(Country::Russia));
    }
    #[test]
    fn minimap_reflects_shroud() {
        use cnc_tiberian_dawn::game::Game;
        let mut game = Game::new("test", 16, 16);
        game.enable_shroud();
        // Initially, only starting position is visible
        let visible = game.shroud.is_visible(0, cnc_core::Vec2i::new(5, 5));
        assert!(!visible);
        // Reveal area around starting base
        game.shroud.reveal(0, cnc_core::Vec2i::new(8, 8), 2);
        assert!(game.shroud.is_visible(0, cnc_core::Vec2i::new(8, 8)));
        assert!(game.shroud.is_visible(0, cnc_core::Vec2i::new(9, 8)));
        assert!(!game.shroud.is_visible(0, cnc_core::Vec2i::new(10, 10)));
    }
    #[test]
    fn power_out_blanks_radar() {
        use cnc_tiberian_dawn::game::RadarState;
        let mut radar = RadarState::new();
        // Simulate radar built with sufficient power
        radar.update(true, 100, 50);
        assert!(radar.is_minimap_enabled());
        // Power drops below drain
        radar.update(true, 30, 50);
        assert!(!radar.is_minimap_enabled());
        assert!(radar.low_power_disabled);
    }

    #[test]
    fn main_menu_navigates_and_launches_local_game() {
        let mut menu = MainMenu::from_campaigns(
            menu_test_campaign("maps/td_campaign", 9),
            menu_test_campaign("maps/ra_campaign", 10),
        );
        assert_eq!(menu.screen(), Screen::Main);
        assert_eq!(menu.cursor(), 0);

        // Down moves to Local Game and Enter opens that page.
        assert_eq!(menu.on_key(&Key::Named(NamedKey::ArrowDown)), None);
        assert_eq!(menu.cursor(), 1);
        assert_eq!(menu.activate(), None);
        assert_eq!(menu.screen(), Screen::Local);

        // The first local entry starts a Tiberian Dawn skirmish.
        assert_eq!(
            menu.activate(),
            Some(MenuCommand::Launch(LaunchTarget::Local(
                GameChoice::TiberianDawn
            )))
        );

        // Down selects the Red Alert skirmish.
        menu.on_key(&Key::Named(NamedKey::ArrowDown));
        assert_eq!(
            menu.activate(),
            Some(MenuCommand::Launch(LaunchTarget::Local(
                GameChoice::RedAlert
            )))
        );
    }

    #[test]
    fn main_menu_campaign_page_opens_in_game_scenario_selectors() {
        let mut menu = MainMenu::from_campaigns(
            menu_test_campaign("maps/td_campaign", 9),
            menu_test_campaign("maps/ra_campaign", 10),
        );
        // Campaign is the first row on the main page.
        assert_eq!(menu.activate(), None);
        assert_eq!(menu.screen(), Screen::Campaign);
        assert_eq!(
            menu.activate(),
            Some(MenuCommand::Launch(LaunchTarget::Campaign(
                GameChoice::TiberianDawn
            )))
        );

        menu.on_key(&Key::Named(NamedKey::ArrowDown));
        assert_eq!(
            menu.activate(),
            Some(MenuCommand::Launch(LaunchTarget::Campaign(
                GameChoice::RedAlert
            )))
        );
    }

    #[test]
    fn campaign_without_scenarios_reports_how_to_fix_it() {
        let mut menu = MainMenu::from_campaigns(
            CampaignInfo {
                dir: PathBuf::from("maps/td_campaign"),
                scenarios: 0,
            },
            menu_test_campaign("maps/ra_campaign", 10),
        );
        assert_eq!(menu.activate(), None);
        assert_eq!(menu.screen(), Screen::Campaign);
        assert_eq!(menu.activate(), None, "missing scenarios must not launch");
        let notice = menu.notice().unwrap();
        assert!(notice.contains("CNC_TD_CAMPAIGN"), "{notice}");
    }

    #[test]
    fn online_page_offers_red_alert_only() {
        let mut menu = MainMenu::from_campaigns(
            menu_test_campaign("maps/td_campaign", 9),
            menu_test_campaign("maps/ra_campaign", 10),
        );
        menu.on_key(&Key::Named(NamedKey::ArrowDown));
        menu.on_key(&Key::Named(NamedKey::ArrowDown));
        assert_eq!(menu.activate(), None);
        assert_eq!(menu.screen(), Screen::Online);

        assert_eq!(
            menu.activate(),
            Some(MenuCommand::Launch(LaunchTarget::Online(
                GameChoice::RedAlert
            )))
        );

        // The only remaining row is Back: TD network play is out of scope for
        // the menu until a TD lobby plus lockstep wiring exists.
        menu.on_key(&Key::Named(NamedKey::ArrowDown));
        assert_eq!(menu.activate(), None);
        assert_eq!(menu.screen(), Screen::Main);
    }

    #[test]
    fn menu_escape_backs_out_and_quits_from_main() {
        let mut menu = MainMenu::from_campaigns(
            menu_test_campaign("maps/td_campaign", 9),
            menu_test_campaign("maps/ra_campaign", 10),
        );
        menu.activate();
        assert_eq!(menu.screen(), Screen::Campaign);
        assert_eq!(menu.on_key(&Key::Named(NamedKey::Escape)), None);
        assert_eq!(menu.screen(), Screen::Main);
        assert_eq!(
            menu.on_key(&Key::Named(NamedKey::Escape)),
            Some(MenuCommand::Quit)
        );

        // Up wraps to the Quit row (Options sits between Online Game and Quit).
        assert_eq!(menu.cursor(), 0);
        menu.on_key(&Key::Named(NamedKey::ArrowUp));
        assert_eq!(menu.cursor(), 4);
        assert_eq!(menu.activate(), Some(MenuCommand::Quit));
    }

    #[test]
    fn menu_mouse_click_selects_rows() {
        let mut menu = MainMenu::from_campaigns(
            menu_test_campaign("maps/td_campaign", 9),
            menu_test_campaign("maps/ra_campaign", 10),
        );
        let (x, y, w, h) = menu_test_row(1, 1024);
        assert!(menu.on_pointer(x + 10.0, y + h * 0.5, 1024));
        assert_eq!(menu.on_click(x + 10.0, y + h * 0.5, 1024), None);
        assert_eq!(menu.screen(), Screen::Local);
        assert_eq!(menu.cursor(), 0);
        let _ = w;

        let (x, y, _, h) = menu_test_row(0, 1024);
        assert_eq!(
            menu.on_click(x + 10.0, y + h * 0.5, 1024),
            Some(MenuCommand::Launch(LaunchTarget::Local(
                GameChoice::TiberianDawn
            )))
        );
    }

    // --- Options modal wiring ---

    #[test]
    fn main_menu_options_entry_opens_the_options_modal() {
        let mut menu = MainMenu::from_campaigns(
            menu_test_campaign("maps/td_campaign", 9),
            menu_test_campaign("maps/ra_campaign", 10),
        );
        // Campaign, Local Game, Online Game, then Options.
        for _ in 0..3 {
            menu.on_key(&Key::Named(NamedKey::ArrowDown));
        }
        assert_eq!(menu.activate(), Some(MenuCommand::Options));
    }

    /// Scratch directory for shell tests, rooted inside the workspace
    /// (`RUST/target/`) so no test writes outside the project tree.
    fn shell_scratch(tag: &str) -> PathBuf {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../target/scratch-launcher")
            .join(format!("{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        dir
    }

    fn test_shell(options_path: Option<PathBuf>) -> LauncherShell {
        LauncherShell::new(
            RendererKind::Software,
            1024,
            768,
            0,
            None,
            LauncherOptions::default(),
            options_path,
        )
    }

    #[test]
    fn options_modal_opens_seeded_from_the_persisted_options() {
        let mut shell = test_shell(None);
        shell.open_options_modal();
        let modal = shell.options_modal.as_ref().expect("modal opens");
        assert_eq!(modal.renderer(), menu::RendererChoice::Wgpu);
        assert_eq!(modal.volume(VolumeBus::Master), 1.0);
        assert_eq!(modal.volume(VolumeBus::Music), 1.0);
        assert_eq!(modal.volume(VolumeBus::Effects), 1.0);
        assert!(modal.note().is_none());

        // An invalid persisted renderer value seeds the resolved choice and
        // surfaces the fallback note instead of failing.
        shell.options.renderer = "vulkan".to_string();
        shell.open_options_modal();
        let modal = shell.options_modal.as_ref().expect("modal reopens");
        assert_eq!(modal.renderer(), menu::RendererChoice::Wgpu);
        let note = modal.note().expect("fallback note is shown");
        assert!(note.contains("vulkan"), "{note}");
        assert!(note.contains("using Wgpu"), "{note}");
    }

    #[test]
    fn options_modal_volume_edits_apply_live_and_save_persists_them() {
        let dir = shell_scratch("options-save");
        let path = dir.join("launcher-options.ini");
        let mut shell = test_shell(Some(path.clone()));
        shell.open_options_modal();

        // Focus the Master row and step it down through the modal.
        shell
            .options_modal
            .as_mut()
            .expect("modal opens")
            .on_key(&Key::Named(NamedKey::ArrowDown));
        let action = shell
            .options_modal
            .as_mut()
            .expect("modal stays open")
            .on_key(&Key::Named(NamedKey::ArrowLeft))
            .expect("left steps the volume");
        shell.apply_options_action(action);
        assert!((shell.options.master_volume - 0.95).abs() < 1.0e-6);
        let mixer = shell.menu_mixer.as_ref().expect("preview mixer exists");
        assert!(
            (mixer.master_volume() - 0.95).abs() < 1.0e-6,
            "volume edits push through the live mixer"
        );

        // Save persists the live-applied values and closes the modal.
        shell.apply_options_action(OptionsAction::Save);
        assert!(shell.options_modal.is_none(), "save closes the modal");
        assert_eq!(shell.menu.notice(), Some("Options saved."));
        let reloaded = LauncherOptions::load_from(&path);
        assert!((reloaded.master_volume - 0.95).abs() < 1.0e-6);
        assert_eq!(reloaded.renderer, "wgpu");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn options_modal_cancel_discards_edits_and_restores_the_mixer() {
        let mut shell = test_shell(None);
        shell.open_options_modal();
        // Toggle the renderer, then step the Master volume down.
        let action = shell
            .options_modal
            .as_mut()
            .expect("modal opens")
            .on_key(&Key::Named(NamedKey::ArrowRight))
            .expect("right toggles the renderer");
        shell.apply_options_action(action);
        assert_eq!(shell.options.renderer, "Software");
        shell
            .options_modal
            .as_mut()
            .expect("modal stays open")
            .on_key(&Key::Named(NamedKey::ArrowDown));
        let action = shell
            .options_modal
            .as_mut()
            .expect("modal stays open")
            .on_key(&Key::Named(NamedKey::ArrowLeft))
            .expect("left steps the volume");
        shell.apply_options_action(action);
        assert!(shell.options.master_volume < 1.0);

        // Escape-equivalent: cancel restores the persisted snapshot.
        shell.apply_options_action(OptionsAction::Cancel);
        assert!(shell.options_modal.is_none(), "cancel closes the modal");
        assert_eq!(shell.options, LauncherOptions::default());
        let mixer = shell.menu_mixer.as_ref().expect("preview mixer exists");
        assert_eq!(mixer.master_volume(), 1.0, "the live push is reverted");
    }

    #[test]
    fn options_modal_renderer_choice_is_persisted_on_save() {
        let dir = shell_scratch("options-renderer");
        let path = dir.join("launcher-options.ini");
        let mut shell = test_shell(Some(path.clone()));
        shell.open_options_modal();
        shell.apply_options_action(OptionsAction::Renderer(menu::RendererChoice::Software));
        shell.apply_options_action(OptionsAction::Save);
        assert_eq!(LauncherOptions::load_from(&path).renderer, "Software");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Headless Options-modal capture for visual QA: renders the menu plus
    /// the modal with the software renderer and writes a PNG under
    /// `RUST/target/` (never /tmp). Run with:
    /// `cargo test -p cnc-launcher options_modal_writes_a_screenshot -- --ignored --nocapture`
    #[test]
    #[ignore = "visual QA capture; writes RUST/target/scratch-launcher/options-modal.png"]
    fn options_modal_writes_a_screenshot_for_visual_qa() {
        const SHOT_W: u32 = 1280;
        const SHOT_H: u32 = 720;
        let menu = MainMenu::from_campaigns(
            menu_test_campaign("maps/td_campaign", 9),
            menu_test_campaign("maps/ra_campaign", 10),
        );
        let mut modal = OptionsModal::new(
            menu::RendererChoice::Wgpu,
            0.8,
            0.55,
            0.35,
            LauncherOptions::default_controls(),
            Some("Saved renderer 'vulkan' is invalid - using Wgpu.".to_string()),
        );
        // Focus the Save row so the capture shows the selection highlight.
        let (sx, sy, sw, sh) = menu::options_button_rect(0, SHOT_W, SHOT_H);
        let _ = modal.on_click(sx + sw * 0.5, sy + sh * 0.5, SHOT_W, SHOT_H);
        let mut renderer = SoftwareRenderer::new(SHOT_W, SHOT_H);
        renderer.begin_frame();
        renderer.set_camera(&screen_camera(SHOT_W, SHOT_H));
        menu.draw(&mut renderer, SHOT_W, SHOT_H);
        modal.draw(&mut renderer, SHOT_W, SHOT_H);
        renderer.end_frame().unwrap();

        let pixel = |x: u32, y: u32| {
            let value = renderer.buffer()[y as usize * SHOT_W as usize + x as usize];
            [
                ((value >> 16) & 0xFF) as i32,
                ((value >> 8) & 0xFF) as i32,
                (value & 0xFF) as i32,
            ]
        };
        // The panel's orange top rule sits at y = 121 on a 1280x720 capture.
        let accent = pixel(500, 121);
        assert!(
            accent[0] > 150 && accent[0] > accent[2],
            "the modal panel top rule should be orange: {accent:?}"
        );
        // The focused Save button shows the row highlight at its right end.
        let (bx, by, bw, _) = menu::options_button_rect(0, SHOT_W, SHOT_H);
        let save = pixel((bx + bw - 12.0) as u32, (by + 15.0) as u32);
        assert!(
            (save[0] - 33).abs() <= 12 && (save[1] - 48).abs() <= 12,
            "the focused Save row highlight should show: {save:?}"
        );

        let mut image_data = image::RgbaImage::new(SHOT_W, SHOT_H);
        for (index, &value) in renderer.buffer().iter().enumerate() {
            image_data.put_pixel(
                (index % SHOT_W as usize) as u32,
                (index / SHOT_W as usize) as u32,
                image::Rgba([
                    ((value >> 16) & 0xFF) as u8,
                    ((value >> 8) & 0xFF) as u8,
                    (value & 0xFF) as u8,
                    ((value >> 24) & 0xFF) as u8,
                ]),
            );
        }
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/scratch-launcher");
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        let path = dir.join("options-modal.png");
        image_data.save(&path).expect("write the screenshot");
        println!("Options modal screenshot: {}", path.display());
    }

    #[test]
    fn scenario_count_ignores_non_ini_files() {
        let dir = std::env::temp_dir().join(format!(
            "cnc-menu-scenarios-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("01.ini"), b"[Basic]\nName=One\n").unwrap();
        std::fs::write(dir.join("02.INI"), b"[Basic]\nName=Two\n").unwrap();
        std::fs::write(dir.join("readme.txt"), b"not a scenario").unwrap();
        assert_eq!(scenario_count(&dir), 2);
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(scenario_count(Path::new("definitely-not-a-directory")), 0);
    }

    #[test]
    fn main_menu_draws_every_page_without_a_window() {
        let mut menu = MainMenu::from_campaigns(
            menu_test_campaign("maps/td_campaign", 9),
            menu_test_campaign("maps/ra_campaign", 10),
        );
        let mut renderer = SoftwareRenderer::new(1024, 768);
        for (row, page) in [
            (0usize, Screen::Campaign),
            (1, Screen::Local),
            (2, Screen::Online),
        ] {
            menu.reset();
            for _ in 0..row {
                menu.on_key(&Key::Named(NamedKey::ArrowDown));
            }
            menu.activate();
            assert_eq!(menu.screen(), page);
            renderer.begin_frame();
            renderer.set_camera(&screen_camera(1024, 768));
            menu.draw(&mut renderer, 1024, 768);
            renderer.end_frame().unwrap();
            assert!(
                renderer.buffer().iter().any(|pixel| *pixel != 0),
                "page {page:?} should draw pixels"
            );
        }
    }

    #[test]
    fn return_to_menu_flag_is_consumed_once() {
        let mut td = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        assert!(!td.take_return_to_menu());
        td.return_to_menu = true;
        assert!(td.take_return_to_menu());
        assert!(
            !td.take_return_to_menu(),
            "flag is reset after the shell sees it"
        );

        let mut ra = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        assert!(!ra.take_return_to_menu());
        ra.return_to_menu = true;
        assert!(ra.take_return_to_menu());
        assert!(!ra.take_return_to_menu());
    }

    #[test]
    fn zoom_defaults_clamp_and_pin_the_world_under_the_cursor() {
        let mut td = LauncherApp::new(
            RendererKind::Software,
            1024,
            768,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        assert!((td.camera.zoom.to_f32() - DEFAULT_GAME_ZOOM).abs() < 1.0e-3);

        td.camera.position = Vec2::new(Fixed16_16::from_int(600), Fixed16_16::from_int(600));
        td.cursor = Some(winit::dpi::PhysicalPosition::new(820.0, 190.0));
        let before = td.cursor_world().unwrap();
        td.zoom_td(2.0);
        let after = td.cursor_world().unwrap();
        assert!((before.x.to_f32() - after.x.to_f32()).abs() < 1.0);
        assert!((before.y.to_f32() - after.y.to_f32()).abs() < 1.0);

        for _ in 0..20 {
            td.zoom_td(2.0);
        }
        assert!(td.camera.zoom.to_f32() <= MAX_GAME_ZOOM + 1.0e-4);
        for _ in 0..40 {
            td.zoom_td(0.5);
        }
        assert!(td.camera.zoom.to_f32() >= MIN_GAME_ZOOM - 1.0e-4);

        let mut ra = RaLauncherApp::new(RendererKind::Software, 1024, 768, 0);
        ra.start_skirmish();
        assert!((ra.camera.zoom.to_f32() - DEFAULT_GAME_ZOOM).abs() < 1.0e-3);
    }

    #[test]
    fn hud_world_maps_zoomed_clicks_to_the_same_screen_point() {
        let mut td = LauncherApp::new(
            RendererKind::Software,
            1024,
            768,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        td.camera.zoom = Fixed16_16::from_f32(2.5);
        let screen = winit::dpi::PhysicalPosition::new(700.0, 300.0);
        let hud = td.hud_world(td.screen_to_world(screen));
        let expected_x = td.camera.position.x.to_f32() + 700.0 - 1024.0 * 0.5;
        let expected_y = td.camera.position.y.to_f32() + 300.0 - 768.0 * 0.5;
        assert!((hud.x.to_f32() - expected_x).abs() < 0.5);
        assert!((hud.y.to_f32() - expected_y).abs() < 0.5);
    }

    #[test]
    fn water_shore_tile_matches_pack_art_convention() {
        // The pack's shore art names the side its WATER half occupies, so land
        // to the north selects shore_south; masks are N=1 E=2 S=4 W=8.
        assert_eq!(water_shore_tile(0), None);
        assert_eq!(water_shore_tile(0b0001), Some(("shore_south", 4)));
        assert_eq!(water_shore_tile(0b0010), Some(("shore_west", 5)));
        assert_eq!(water_shore_tile(0b0100), Some(("shore_north", 2)));
        assert_eq!(water_shore_tile(0b1000), Some(("shore_east", 3)));
        // Two orthogonal land neighbours pick the corner art whose water half
        // faces the open diagonal.
        assert_eq!(water_shore_tile(0b0011), Some(("shore_sw", 8)));
        assert_eq!(water_shore_tile(0b1001), Some(("shore_se", 9)));
        // Opposite pairs flow; a ringed cell falls back to shallow water.
        assert_eq!(water_shore_tile(0b0101), Some(("river_flow_e_w", 11)));
        assert_eq!(water_shore_tile(0b1010), Some(("river_flow_n_s", 10)));
        assert_eq!(water_shore_tile(0b1111), Some(("shallow_water", 1)));
    }

    #[test]
    fn land_mask_at_treats_out_of_map_as_unmatched() {
        // Land only at (0, 0); everything else counts as water.
        let is_land = |x: i32, y: i32| x == 0 && y == 0;
        // (0, 1): the north neighbour (0, 0) matches -> north bit 1.
        assert_eq!(land_mask_at(0, 1, is_land), 0b0001);
        // (1, 1): (0, 0) is only a diagonal neighbour; orthogonals are water
        // or off-map -> no bits.
        assert_eq!(land_mask_at(1, 1, is_land), 0);
        // (0, 0): all four orthogonal neighbours are off-map or water.
        assert_eq!(land_mask_at(0, 0, is_land), 0);
        // Bit order sanity: matched north neighbour sets bit 1 only.
        let is_north = |x: i32, y: i32| y == -1 && x == 5;
        assert_eq!(land_mask_at(5, 0, is_north), 0b0001);
    }

    #[test]
    fn ra_terrain_tile_shores_water_and_sand_shores_beach() {
        let mut map = cnc_red_alert::Map::new(3, 3);
        // Map::blank clamps to an 8x8 minimum; use the real width for row-major
        // writes so water/beach/grass occupy whole rows as intended.
        let w = map.width as usize;
        for x in 0..3usize {
            map.terrain[x] = cnc_red_alert::Terrain::Water;
            map.terrain[w + x] = cnc_red_alert::Terrain::Beach;
            map.terrain[2 * w + x] = cnc_red_alert::Terrain::Grass;
        }
        // Water against the beach row below picks the north-water shore art
        // (land south -> shore_north), and the grass east corner turns the
        // same cell into a north-west corner case.
        assert_eq!(
            ra_terrain_tile(&map, 1, 0),
            ("water_edges", "shore_north", 2, None)
        );
        // Beach renders the transitions sand_shore row; the only water
        // neighbour is to the north, so the edge-mask column is 0b0001.
        assert_eq!(
            ra_terrain_tile(&map, 1, 1),
            ("transitions", "sand_shore", 10, Some(0b0001))
        );
        // Grass keeps the base terrain family with the variant-hash column.
        assert_eq!(ra_terrain_tile(&map, 1, 2), ("terrain", "grass", 0, None));
    }

    #[test]
    fn ra_terrain_tile_beach_column_is_water_edge_mask() {
        let mut map = cnc_red_alert::Map::new(4, 4);
        let w = map.width as usize;
        // Beach at (1, 1); water to the north (1, 0) and west (0, 1).
        map.terrain[w + 1] = cnc_red_alert::Terrain::Beach;
        map.terrain[1] = cnc_red_alert::Terrain::Water;
        map.terrain[w] = cnc_red_alert::Terrain::Water;
        // N and W sides carry water: edge mask 0b1001.
        assert_eq!(
            ra_terrain_tile(&map, 1, 1),
            ("transitions", "sand_shore", 10, Some(0b1001))
        );
    }

    #[test]
    fn campaign_terrain_tile_shores_water_against_land() {
        // 4x4 grid, water column on the right (code 1), grass elsewhere.
        let mut terrain = vec![0u8; 16];
        for y in 0..4usize {
            terrain[y * 4 + 3] = 1;
        }
        // Water at (3, 0): west neighbour (2, 0) is grass land; the art names
        // its water half, so land west selects shore_east.
        assert_eq!(
            campaign_terrain_tile(&terrain, 4, 4, 3, 0),
            ("water_edges", "shore_east", 3, None)
        );
        // All-water grid away from any land stays deep_water.
        let all_water = vec![1u8; 25];
        assert_eq!(
            campaign_terrain_tile(&all_water, 5, 5, 2, 2),
            ("water_edges", "deep_water", 0, None)
        );
        // Non-water codes keep the base campaign families.
        assert_eq!(
            campaign_terrain_tile(&terrain, 4, 4, 0, 0),
            ("terrain", "grass", 0, None)
        );
        let mut sandy = vec![0u8; 16];
        sandy[0] = 2;
        assert_eq!(
            campaign_terrain_tile(&sandy, 4, 4, 0, 0),
            ("terrain", "sand", 2, None)
        );
    }

    #[test]
    fn tiberium_fx_family_buckets_by_amount() {
        assert_eq!(tiberium_fx_family(0), ("tiberium_sprout", 0));
        assert_eq!(tiberium_fx_family(85), ("tiberium_sprout", 0));
        assert_eq!(tiberium_fx_family(86), ("tiberium_cluster", 1));
        assert_eq!(tiberium_fx_family(170), ("tiberium_cluster", 1));
        assert_eq!(tiberium_fx_family(171), ("tiberium_bloom", 14));
        assert_eq!(tiberium_fx_family(u8::MAX), ("tiberium_bloom", 14));
    }

    #[test]
    fn ready_receive_guard_marks_tracker_in_poll() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: 0,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        let target = app.network.as_ref().unwrap().socket.local_addr().unwrap();
        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
        sender.send_to(b"CNCREADY 1 Player1 1\n", target).unwrap();
        app.poll_network();
        assert!(
            app.ready_tracker.is_ready("Player1"),
            "a ready datagram marks the roster before the lockstep handler"
        );
        assert_eq!(
            app.ready_tracker.roster(),
            vec![("Player1".to_string(), true)]
        );
        assert_eq!(
            app.network.as_ref().unwrap().applied,
            0,
            "ready lines are out-of-band: no lockstep frame is applied"
        );
    }

    #[test]
    fn ready_tracker_polls_in_udp_loop() {
        // End-to-end across the real UDP socket pair: the sender's `send_ready`
        // hook emits a line that the receiver's poll loop consumes into its
        // `ReadyTracker` roster, in both the RA and the TD lane.
        let mut host = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        host.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: 0,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        let host_addr = host.network.as_ref().unwrap().socket.local_addr().unwrap();

        // A join peer that only knows about the host address.
        let mut join = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        join.enable_network(&NetworkSetup {
            role: "join".to_string(),
            join: Some(host_addr.to_string()),
            port: 0,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();

        // Send hook: the joiner's local flag goes out to its known peers.
        join.ready_tracker.set_expected(2);
        join.send_ready(true);
        assert!(
            join.ready_tracker.is_ready("Player1"),
            "the send hook records the local flag"
        );

        // Poll hook: the host's UDP loop folds the datagram into its roster.
        let mut marked = false;
        for _ in 0..100 {
            host.poll_network();
            if host.ready_tracker.is_ready("Player1") {
                marked = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(marked, "the host poll loop marks the peer ready");
        assert_eq!(
            host.ready_tracker.roster(),
            vec![("Player1".to_string(), true)]
        );
        assert_eq!(
            host.network.as_ref().unwrap().applied,
            0,
            "ready lines stay out of the lockstep stream"
        );

        // The TD lane mirrors the same hooks through `send_td_ready`.
        let mut td = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .expect("TD launcher app");
        td.enable_td_network(&NetworkSetup {
            role: "join".to_string(),
            join: Some(host_addr.to_string()),
            port: 0,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        td.send_td_ready(true);
        let mut td_marked = false;
        for _ in 0..100 {
            host.poll_network();
            if host.ready_tracker.roster().len() > 1 {
                td_marked = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(
            td_marked,
            "the TD ready hook reaches the host's UDP poll loop too"
        );
    }

    #[test]
    fn lobby_lists_directory_services() {
        // The lobby screen renders what the directory reports, and falls back
        // to a direct-connect row (never a panic) when no directory answers.
        let mut screen = lobby_screen::LobbyScreen::new("10.0.0.5:47890");
        let found = lobby_screen::compose_scan(
            lobby_screen::RefreshMode::Http,
            Ok(vec![
                lobby_screen::LobbyListing {
                    name: "Nod vs GDI".to_string(),
                    address: "10.0.0.9:47870".to_string(),
                    occupancy: "2/4".to_string(),
                    game: "TD",
                    source: lobby_screen::ListingSource::Directory,
                },
                lobby_screen::LobbyListing {
                    name: "Allied Assault".to_string(),
                    address: "10.0.0.10:47872".to_string(),
                    occupancy: "1/2".to_string(),
                    game: "RA",
                    source: lobby_screen::ListingSource::Directory,
                },
            ]),
            &screen.manual_address,
        );
        screen.mode = lobby_screen::RefreshMode::Http;
        screen.scan = found;
        assert_eq!(screen.row_count(), 3, "two directory rows + the manual row");
        assert!(screen.scan.has_directory_rows());
        assert!(!screen.scan.used_fallback());
        let rows = screen.scan.render_rows();
        assert!(rows[0].contains("Nod vs GDI"), "rows: {rows:?}");
        assert!(rows[2].contains("DIRECT"), "rows: {rows:?}");

        // Second half: a directory that never answers degrades to a direct
        // connect instead of failing the lobby.
        let fallback = lobby_screen::refresh_lobby_with(
            lobby_screen::RefreshMode::Http,
            "10.0.0.5:47890",
            &["127.0.0.1:1".to_string()],
        );
        assert!(fallback.used_fallback());
        assert_eq!(fallback.listings.len(), 1);
        assert_eq!(fallback.listings[0].address, "10.0.0.5:47890");
        assert!(!fallback.has_directory_rows());
    }

    #[test]
    fn ready_send_marks_local_player_like_chat() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.enable_network(&NetworkSetup {
            role: "host".to_string(),
            join: None,
            port: 0,
            peers: Vec::new(),
            bind: "127.0.0.1".to_string(),
            advertised: None,
        })
        .unwrap();
        app.send_ready(true);
        assert!(
            app.ready_tracker.is_ready("Player0"),
            "send_ready records the local player like send_chat logs chat"
        );
        assert_eq!(
            app.ready_tracker.roster(),
            vec![("Player0".to_string(), true)]
        );
    }

    #[test]
    fn net_lobby_roster_rows_show_ready_state() {
        let roster = vec![
            ("Player0".to_string(), true),
            ("Player1".to_string(), false),
            ("Player2".to_string(), true),
        ];
        assert_eq!(
            net_lobby_roster_rows(&roster),
            vec![
                "Player0: READY".to_string(),
                "Player1: waiting".to_string(),
                "Player2: READY".to_string(),
            ],
            "roster rows carry each player's ready state in name-sorted order"
        );
        assert!(net_lobby_roster_rows(&[]).is_empty());
    }

    #[test]
    fn ra_untouched_host_lobby_starts_and_arms_expected() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.open_lobby();
        app.lobby.peer_address = String::new();
        app.lobby.host_port = 0;
        assert_eq!(app.ready_tracker.expected_count(), 0);
        let _ = app.start_lobby();
        assert_eq!(
            app.mode,
            RaMode::Playing,
            "a lobby expecting nobody is not gated even with no readies"
        );
        let expected = app.network.as_ref().unwrap().session.players as usize;
        assert_eq!(
            app.ready_tracker.expected_count(),
            expected,
            "starting arms the gate to the session player count"
        );
        assert!(expected > 0);
    }

    #[test]
    fn ra_host_start_gate_blocks_until_all_ready() {
        let mut app = RaLauncherApp::new(RendererKind::Software, 320, 240, 0);
        app.open_lobby();
        app.lobby.peer_address = String::new();
        app.lobby.host_port = 0;
        app.ready_tracker.set_expected(2);
        let _ = app.start_lobby();
        assert_eq!(app.mode, RaMode::NetworkLobby, "an unready host is gated");
        assert!(app.lobby_error.is_some());
        assert!(app.network.is_none());
        app.ready_tracker.mark_ready("Player0", true).unwrap();
        let _ = app.start_lobby();
        assert_eq!(
            app.mode,
            RaMode::NetworkLobby,
            "one of two ready is still gated"
        );
        app.ready_tracker.mark_ready("Player1", true).unwrap();
        let _ = app.start_lobby();
        assert_eq!(app.mode, RaMode::Playing, "all ready lets the host start");
    }

    #[test]
    fn td_host_start_gate_blocks_unready() {
        let mut app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            LauncherOptions::default(),
        )
        .unwrap();
        app.td_lobby_open = true;
        app.td_lobby.role = NetRole::Host;
        app.ready_tracker.set_expected(2);
        let _ = app.start_td_lobby();
        assert!(app.td_lobby_open, "a gated TD lobby stays open");
        assert!(app.td_lobby_error.is_some());
        assert!(app.network.is_none(), "a gated host never binds a socket");
    }

    // ---- Briefing VQA video path (plan box 212) ----

    /// Little-endian u16 IFF field, mirroring `cnc_asset::video`'s test helper.
    fn vqa_le16(value: u16) -> [u8; 2] {
        value.to_le_bytes()
    }

    /// One IFF chunk: 4-byte tag, u32le size, data, padded to even length.
    fn vqa_chunk(tag: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut out = tag.to_vec();
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(data);
        if data.len() % 2 == 1 {
            out.push(0);
        }
        out
    }

    /// A VQHD chunk: version 1, flags 0, 1 frame, rate 15, `w x h` blocks.
    fn vqa_vqhd(width_blocks: u16, height_blocks: u16) -> Vec<u8> {
        vqa_chunk(b"VQHD", &{
            let mut header = Vec::new();
            header.extend_from_slice(&vqa_le16(1)); // version
            header.extend_from_slice(&vqa_le16(0)); // flags
            header.extend_from_slice(&vqa_le16(1)); // num_frames
            header.extend_from_slice(&vqa_le16(15)); // frame_rate
            header.extend_from_slice(&vqa_le16(width_blocks));
            header.extend_from_slice(&vqa_le16(height_blocks));
            header
        })
    }

    /// Minimal v1 WVQA container: `VQHD + CBF0 + VQFL + SND`. The codebook's
    /// entry 0 is four copies of `block_index`, and every block in the frame
    /// points at entry 0, so `decode_frame` paints every pixel that one index
    /// and the decoded colour asserts crisply.
    fn synthetic_briefing_vqa(block_index: u8) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"FORM");
        bytes.extend_from_slice(&[0, 0, 0, 0]); // FORM size, patched below
        bytes.extend_from_slice(b"WVQA");
        bytes.extend_from_slice(&vqa_vqhd(2, 3)); // 2x3 blocks -> 4x6 px

        let mut codebook = vec![0u8; 1024]; // 256 entries x 4 indices
        for byte in codebook.iter_mut().take(4) {
            *byte = block_index;
        }
        bytes.extend_from_slice(&vqa_chunk(b"CBF0", &codebook));

        bytes.extend_from_slice(&vqa_chunk(b"VQFL", &[0u8; 6])); // 6 blocks, all entry 0
        bytes.extend_from_slice(&vqa_chunk(b"SND0", &[0x34, 0x12, 0x78, 0x56])); // raw PCM

        let total = (bytes.len() - 8) as u32;
        bytes[4..8].copy_from_slice(&total.to_be_bytes());
        bytes
    }

    /// Pack an `[r, g, b, a]` colour the way `SoftwareRenderer` presents it
    /// (0xFF_RRGGBB in the pixel buffer).
    fn vqa_packed(color: [u8; 4]) -> u32 {
        (u32::from(color[3]) << 24)
            | (u32::from(color[0]) << 16)
            | (u32::from(color[1]) << 8)
            | u32::from(color[2])
    }

    #[test]
    fn briefing_video_first_frame_decodes_codebook_colours() {
        let bytes = synthetic_briefing_vqa(200);
        let (rgba, width, height) =
            briefing_video_first_frame(&bytes).expect("a valid v1 blob decodes");
        assert_eq!((width, height), (4, 6), "2x3 blocks decode to 4x6 px");
        assert_eq!(rgba.len(), 4 * 6 * 4, "one RGBA pixel per decoded index");
        let expected = vqa_index_to_rgba(200);
        assert_eq!(
            rgba[..4],
            expected,
            "block index 0 paints its codebook colour"
        );
        assert!(
            rgba.as_chunks::<4>().0.iter().all(|px| px == &expected),
            "a uniform codebook entry paints every pixel the same colour"
        );
    }

    #[test]
    fn draw_video_frame_uploads_and_draws_headlessly() {
        let bytes = synthetic_briefing_vqa(200);
        let (rgba, width, height) = briefing_video_first_frame(&bytes).expect("decode");
        let mut renderer = SoftwareRenderer::new(64, 64);
        assert!(
            draw_video_frame(&mut renderer, &rgba, width, height, 64, 64),
            "the frame uploads to the borrowed slot and draws"
        );
        let expected = vqa_packed(vqa_index_to_rgba(200));
        assert_eq!(
            renderer.buffer()[32 * 64 + 32],
            expected,
            "the centre pixel is the codebook colour"
        );
        let painted = renderer.buffer().iter().filter(|p| **p == expected).count();
        assert!(
            painted > 100,
            "the frame rasterises a large region: {painted} px"
        );
    }

    #[test]
    fn draw_briefing_video_renders_a_real_asset_end_to_end() {
        // Scratch lives under RUST/target/, never /tmp (see shell_scratch).
        let dir = shell_scratch("briefing-vqfl");
        let path = dir.join("preware.vqa");
        std::fs::write(&path, synthetic_briefing_vqa(170)).unwrap();
        let key = path.to_str().unwrap().to_string();

        let mut renderer = SoftwareRenderer::new(64, 64);
        assert!(
            draw_briefing_video(&mut renderer, Some(&key), 64, 64),
            "a readable v1 asset renders its first frame headlessly"
        );
        assert_eq!(
            renderer.buffer()[32 * 64 + 32],
            vqa_packed(vqa_index_to_rgba(170)),
            "the briefing surface shows the decoded frame at centre"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn draw_briefing_video_falls_back_on_missing_asset() {
        let mut renderer = SoftwareRenderer::new(64, 64);
        // No key at all.
        assert!(!draw_briefing_video(&mut renderer, None, 64, 64));
        // Key set but no such file.
        assert!(!draw_briefing_video(
            &mut renderer,
            Some("no/such/briefing.vqa"),
            64,
            64
        ));
        assert!(
            renderer.buffer().iter().all(|p| *p == 0),
            "a fallback draws nothing into the surface"
        );
    }

    #[test]
    fn draw_briefing_video_falls_back_on_unsupported_variant() {
        // A v1 container whose only content chunk is a CBP0 partial codebook,
        // which demux reports as UnsupportedVariant.
        let mut bytes = synthetic_briefing_vqa(170);
        // Swap the CBF0 codebook chunk (1024 bytes) for a tiny CBP0 chunk.
        // synthetic_briefing_vqa seeds codebook entry 0 with the block index
        // (170 = 0xAA), so the expected payload mirrors that, not all zeros.
        let mut codebook = vec![0u8; 1024];
        codebook[..4].copy_from_slice(&[170; 4]);
        let cbf0 = vqa_chunk(b"CBF0", &codebook);
        let start = 12 + vqa_vqhd(2, 3).len();
        assert_eq!(
            &bytes[start..start + cbf0.len()],
            &cbf0[..],
            "CBF0 sits after VQHD"
        );
        bytes.splice(start..start + cbf0.len(), vqa_chunk(b"CBP0", &[1, 2, 3, 4]));

        assert!(
            briefing_video_first_frame(&bytes).is_none(),
            "an unsupported variant yields no frame"
        );

        let dir = shell_scratch("briefing-cbp0");
        let path = dir.join("partial.vqa");
        std::fs::write(&path, &bytes).unwrap();
        let key = path.to_str().unwrap().to_string();

        let mut renderer = SoftwareRenderer::new(64, 64);
        assert!(
            !draw_briefing_video(&mut renderer, Some(&key), 64, 64),
            "an unsupported-variant asset falls back without drawing"
        );
        assert!(
            renderer.buffer().iter().all(|p| *p == 0),
            "the unsupported variant draws nothing"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn draw_td_briefing_uses_text_panel_without_video() {
        let mut renderer = SoftwareRenderer::new(320, 200);
        let mut mission = Mission::new("GDI 01".to_string());
        mission.briefing_image = Some("briefings/gdi_01.vqa".to_string());
        // No tileset pack -> the video never borrows the slot; text panel path.
        assert!(!draw_td_briefing(&mut renderer, &mission, None, 320, 200));
        assert!(
            renderer.buffer().iter().any(|p| *p != 0),
            "the text panel draws pixels"
        );
    }
}
