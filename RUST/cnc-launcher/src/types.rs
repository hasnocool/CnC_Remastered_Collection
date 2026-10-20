//! Shared types for cnc-launcher

use cnc_asset::tileset_pack::TilesetPack;
use cnc_core::RulesDatabase;
use cnc_red_alert::Game as RaGame;
use cnc_renderer::Camera;
use cnc_tiberian_dawn::Game;

/// Renderer backend kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendererKind {
    Wgpu,
    Software,
}

/// Default map size for skirmish games.
pub const MAP_SIZE: u32 = 64;

/// Default game zoom level.
pub const DEFAULT_GAME_ZOOM: f32 = 2.0;

/// RA tile size (32 pixels).
pub const RA_TILE_SIZE: f32 = 32.0;

/// TD launcher application state.
pub struct LauncherApp {
    pub kind: RendererKind,
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub game: Game,
    pub td_tileset: Option<TilesetPack>,
    pub ra_tileset: Option<TilesetPack>,
}

impl LauncherApp {
    pub fn new(
        kind: RendererKind,
        width: u32,
        height: u32,
        rules: RulesDatabase,
    ) -> anyhow::Result<Self> {
        let mut game = Game::with_rules("GDI Mission 1", MAP_SIZE, MAP_SIZE, rules);
        game.setup_demo();

        let mut camera = Camera::default();
        camera.set_viewport(width.max(1), height.max(1));
        camera.position = game.world_center();
        camera.zoom = cnc_core::Fixed16_16::from_f32(DEFAULT_GAME_ZOOM);

        Ok(Self {
            kind,
            width,
            height,
            camera,
            game,
            td_tileset: cnc_asset::tileset_pack::load_default("td_inspired"),
            ra_tileset: cnc_asset::tileset_pack::load_default("ra_inspired"),
        })
    }
}

/// RA launcher application state.
pub struct RaLauncherApp {
    pub kind: RendererKind,
    pub width: u32,
    pub height: u32,
    pub camera: Camera,
    pub game: RaGame,
    pub td_tileset: Option<TilesetPack>,
    pub ra_tileset: Option<TilesetPack>,
}

impl RaLauncherApp {
    pub fn new(kind: RendererKind, width: u32, height: u32, _frame_limit: u64) -> Self {
        let game = RaGame::new(MAP_SIZE, MAP_SIZE);
        let mut camera = Camera::default();
        camera.set_viewport(width.max(1), height.max(1));
        camera.position = cnc_core::Vec2::new(
            cnc_core::Fixed16_16::from_f32(game.map.width as f32 * RA_TILE_SIZE * 0.5),
            cnc_core::Fixed16_16::from_f32(game.map.height as f32 * RA_TILE_SIZE * 0.5),
        );
        Self {
            kind,
            width,
            height,
            camera,
            game,
            td_tileset: cnc_asset::tileset_pack::load_default("td_inspired"),
            ra_tileset: cnc_asset::tileset_pack::load_default("ra_inspired"),
        }
    }

    pub fn start_skirmish(&mut self) {
        self.game.set_ai_tempo(12, 25);
    }
}
