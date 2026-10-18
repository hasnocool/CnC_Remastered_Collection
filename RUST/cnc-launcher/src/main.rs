//! C&C Launcher - Main entry point
//!
//! Creates a winit 0.30 window and drives the renderer (wgpu by default, with
//! a software fallback) from the RedrawRequested event at roughly 60 FPS.
//! The demo scene is real Tiberian Dawn game state: a map, houses, buildings
//! and units owned by cnc-tiberian-dawn's Game type.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cnc_asset::{AssetRegistry, AtlasStyle, SpriteAtlas};
use cnc_audio::AudioMixer;
use cnc_core::rules::RulesDatabase;
use cnc_core::{Fixed16_16, Vec2, Vec2i};
use cnc_renderer::{Camera, Renderer, SoftwareRenderer, WgpuRenderer};
use cnc_tiberian_dawn::{Game, StructureType, UnitType};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::{Window, WindowId};

const FRAME_INTERVAL: Duration = Duration::from_millis(16);
const MAP_SIZE: u32 = 64;

#[derive(Parser)]
#[command(name = "cnc-launcher")]
#[command(about = "Command & Conquer Remastered Collection - Rust Clone")]
struct Args {
    /// Renderer backend: "wgpu" (default) or "software".
    #[arg(short, long, default_value = "wgpu")]
    renderer: String,

    #[arg(short, long, default_value_t = 1024)]
    width: u32,

    #[arg(short = 'H', long, default_value_t = 768)]
    height: u32,

    /// Optional Rules.ini path; its balance overrides replace the hard-coded
    /// Tiberian Dawn defaults.
    #[arg(long)]
    rules: Option<PathBuf>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RendererKind {
    Wgpu,
    Software,
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
const SIDEBAR_PANEL_HEIGHT: f32 = 274.0;

struct LauncherApp {
    kind: RendererKind,
    width: u32,
    height: u32,
    window: Option<Arc<Window>>,
    renderer: Option<Box<dyn Renderer>>,
    camera: Camera,
    game: Game,
    sprite_atlas: Option<SpriteAtlas>,
    // Kept alive for the upcoming asset/audio integration pass.
    #[allow(dead_code)]
    assets: AssetRegistry,
    #[allow(dead_code)]
    mixer: AudioMixer,
    cursor: Option<winit::dpi::PhysicalPosition<f64>>,
    drag_start: Option<winit::dpi::PhysicalPosition<f64>>,
    drag_current: Option<winit::dpi::PhysicalPosition<f64>>,
    shift_down: bool,
    frame_index: u64,
    last_produced: u64,
    last_harvested: i64,
    last_aircraft_shots: u64,
    last_aircraft_rearmed: u64,
    last_secondary_shots: u64,
    last_frame: Instant,
    next_frame: Instant,
    software_warning_emitted: bool,
}

impl LauncherApp {
    fn new(kind: RendererKind, width: u32, height: u32, rules: RulesDatabase) -> Result<Self> {
        let mut game = Game::with_rules("GDI Mission 1", MAP_SIZE, MAP_SIZE, rules);
        game.setup_demo();

        let mut camera = Camera::default();
        camera.set_viewport(width.max(1), height.max(1));
        camera.position = game.world_center();

        Ok(Self {
            kind,
            width,
            height,
            window: None,
            renderer: None,
            camera,
            game,
            sprite_atlas: None,
            assets: AssetRegistry::new(),
            mixer: AudioMixer::new()?,
            cursor: None,
            drag_start: None,
            drag_current: None,
            shift_down: false,
            frame_index: 0,
            last_produced: 0,
            last_harvested: 0,
            last_aircraft_shots: 0,
            last_aircraft_rearmed: 0,
            last_secondary_shots: 0,
            last_frame: Instant::now(),
            next_frame: Instant::now() + FRAME_INTERVAL,
            software_warning_emitted: false,
        })
    }

    fn create_renderer(&mut self, window: Arc<Window>) -> Box<dyn Renderer> {
        match self.kind {
            RendererKind::Wgpu => match pollster::block_on(WgpuRenderer::new(window)) {
                Ok(renderer) => {
                    tracing::info!("wgpu renderer initialized");
                    Box::new(renderer)
                }
                Err(error) => {
                    tracing::error!(%error, "wgpu renderer initialization failed; using software renderer");
                    Box::new(SoftwareRenderer::new(self.width, self.height))
                }
            },
            RendererKind::Software => Box::new(SoftwareRenderer::new(self.width, self.height)),
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

    fn sidebar(&self) -> Sidebar {
        let zoom = self.camera.zoom.to_f32().max(1.0e-6);
        let half_w = self.camera.viewport_width as f32 / (2.0 * zoom);
        let half_h = self.camera.viewport_height as f32 / (2.0 * zoom);
        let left = self.camera.position.x.to_f32() - half_w;
        let top = self.camera.position.y.to_f32() - half_h;
        let x = left + 2.0 * half_w - SIDEBAR_WIDTH - 8.0;
        let y = top + 8.0;
        let units_header_y = y + 36.0;
        let structures_header_y = y + 152.0;

        let mut entries = Vec::new();
        let mut row_y = y + 48.0;
        const UNITS: [(&str, UnitType); 5] = [
            ("Infantry", UnitType::InfantryE1),
            ("Med Tank", UnitType::MediumTank),
            ("Mammoth", UnitType::MammothTank),
            ("Harvester", UnitType::Harvester),
            ("Orca", UnitType::Orca),
        ];
        for (label, unit) in UNITS {
            entries.push(SidebarEntry {
                x,
                y: row_y,
                w: SIDEBAR_WIDTH,
                h: 18.0,
                action: BuildAction::Unit(unit),
                label,
                cost: unit.stats().cost,
                enabled: self.game.can_queue_unit(0, unit),
            });
            row_y += SIDEBAR_ROW_HEIGHT;
        }

        row_y = structures_header_y + 16.0;
        const STRUCTURES: [(&str, StructureType); 5] = [
            ("Power Plant", StructureType::PowerPlant),
            ("Refinery", StructureType::Refinery),
            ("Barracks", StructureType::Barracks),
            ("War Factory", StructureType::WarFactory),
            ("Silo", StructureType::Silo),
        ];
        for (label, structure) in STRUCTURES {
            entries.push(SidebarEntry {
                x,
                y: row_y,
                w: SIDEBAR_WIDTH,
                h: 18.0,
                action: BuildAction::Structure(structure),
                label,
                cost: structure.stats().cost,
                enabled: self.game.can_queue_building(0, structure),
            });
            row_y += SIDEBAR_ROW_HEIGHT;
        }

        Sidebar {
            x,
            y,
            w: SIDEBAR_WIDTH,
            h: SIDEBAR_PANEL_HEIGHT,
            units_header_y,
            structures_header_y,
            entries,
        }
    }

    fn sidebar_hit(&self, world: Vec2) -> SidebarHit {
        let sidebar = self.sidebar();
        let x = world.x.to_f32();
        let y = world.y.to_f32();
        if x < sidebar.x || x > sidebar.x + sidebar.w || y < sidebar.y || y > sidebar.y + sidebar.h {
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
                    BuildAction::Unit(unit) => self.game.queue_unit(0, unit),
                    BuildAction::Structure(structure) => self.game.queue_building(0, structure),
                };
                tracing::debug!(?action, queued, "sidebar build");
                true
            }
            SidebarHit::Panel => true,
            SidebarHit::Outside => false,
        }
    }

    fn draw(&mut self) {
        let now = Instant::now();
        let delta = (now - self.last_frame).as_secs_f32().min(0.1).max(1.0e-4);
        self.last_frame = now;

        let drag_rect = match (self.drag_start, self.drag_current) {
            (Some(start), Some(end)) => {
                let a = self.screen_to_world(start);
                let b = self.screen_to_world(end);
                let min_x = a.x.to_f32().min(b.x.to_f32());
                let min_y = a.y.to_f32().min(b.y.to_f32());
                let max_x = a.x.to_f32().max(b.x.to_f32());
                let max_y = a.y.to_f32().max(b.y.to_f32());
                Some((min_x, min_y, (max_x - min_x).max(1.0), (max_y - min_y).max(1.0)))
            }
            _ => None,
        };

        let sidebar = self.sidebar();
        let cursor = self.cursor_world();

        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        self.game.update(Fixed16_16::from_f32(delta));

        if self.game.units_produced != self.last_produced {
            self.last_produced = self.game.units_produced;
            tracing::info!(
                total = self.last_produced,
                "unit production complete"
            );
        }

        if self.game.harvested_credits != self.last_harvested {
            self.last_harvested = self.game.harvested_credits;
            tracing::info!(
                credits = self.last_harvested,
                "harvester unloaded"
            );
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
            tracing::info!(
                shots = self.last_secondary_shots,
                "secondary weapon fired"
            );
        }

        renderer.begin_frame();
        renderer.set_camera(&self.camera);
        if renderer.uses_sprites() {
            self.game
                .render_with_sprites(renderer.as_mut(), self.sprite_atlas.as_ref());
        } else {
            self.game.render(renderer.as_mut());
        }

        // Drag-selection rectangle.
        if let Some((x, y, width, height)) = drag_rect {
            renderer.draw_rect(x, y, width, height, [0.20, 0.80, 1.00, 0.18]);
        }

        // HUD
        let zoom = self.camera.zoom.to_f32().max(1.0e-6);
        let half_w = self.camera.viewport_width as f32 / (2.0 * zoom);
        let half_h = self.camera.viewport_height as f32 / (2.0 * zoom);
        let left = self.camera.position.x.to_f32() - half_w;
        let top = self.camera.position.y.to_f32() - half_h;

        let status = self.game.status();
        renderer.draw_text(
            &format!("{} - GDI vs Nod", status.mission),
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
                self.game.ready_building_for(0)
            ),
            left + 16.0,
            top + 40.0,
            8.0,
            [0.70, 0.75, 0.85, 1.0],
        );
        renderer.draw_text(
            &format!(
                "1 inf 2 tank 3 mammoth 4 harv 5-9 build 0 orca  I ion N nuke  LMB select/place  Shift add  RMB move/attack  A all  ESC quit  frame {} {}",
                self.frame_index,
                self.kind.label()
            ),
            left + 16.0,
            top + 56.0,
            8.0,
            [0.70, 0.75, 0.85, 1.0],
        );

        // Sidebar build menu (drawn in world space, anchored to the camera).
        renderer.draw_rect(sidebar.x, sidebar.y, sidebar.w, sidebar.h, [0.04, 0.05, 0.08, 0.92]);
        renderer.draw_rect(sidebar.x, sidebar.y, sidebar.w, 1.5, [0.25, 0.45, 0.75, 1.0]);
        renderer.draw_text(
            "BUILD MENU",
            sidebar.x + 8.0,
            sidebar.y + 6.0,
            10.0,
            [0.80, 0.90, 1.00, 1.0],
        );
        renderer.draw_text(
            &format!("queue {}  ion {}%", status.production, status.gdi_ion),
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
            let hovered = cursor.map_or(false, |hover| {
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

        if let Err(error) = renderer.end_frame() {
            tracing::warn!(%error, "frame presentation failed");
        }

        self.frame_index = self.frame_index.wrapping_add(1);

        if self.kind == RendererKind::Software && !self.software_warning_emitted {
            tracing::warn!("software renderer draws offscreen only; use --renderer wgpu to present frames");
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

        let assets_dir =
            std::env::var("CNC_ASSETS").unwrap_or_else(|_| "assets".to_string());
        let atlas = SpriteAtlas::load_or_generate(Path::new(&assets_dir), AtlasStyle::Neon);
        if let Some(renderer) = self.renderer.as_mut() {
            if renderer.uses_sprites() {
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
            WindowEvent::ModifiersChanged(modifiers) => {
                self.shift_down = modifiers.state().shift_key();
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
                        let start_world = self.screen_to_world(start);
                        let over_sidebar =
                            !matches!(self.sidebar_hit(world), SidebarHit::Outside);
                        let started_in_sidebar =
                            !matches!(self.sidebar_hit(start_world), SidebarHit::Outside);

                        if over_sidebar {
                            self.handle_sidebar_click(world);
                        } else if started_in_sidebar {
                            // Drag started on the build menu: ignore.
                        } else if self.game.ready_building_for(0).is_some() {
                            let tile = cnc_tiberian_dawn::map::TILE_SIZE;
                            let cell = Vec2i::new(
                                (world.x.to_f32() / tile).floor() as i32,
                                (world.y.to_f32() / tile).floor() as i32,
                            );
                            let placed = self.game.place_ready_building(0, cell);
                            tracing::debug!(placed, "place GDI structure");
                        } else {
                            let dx = end.x - start.x;
                            let dy = end.y - start.y;
                            if dx * dx + dy * dy > 36.0 {
                                let count = self.game.select_in_rect(start_world, world, self.shift_down);
                                tracing::debug!(count, additive = self.shift_down, "box selection");
                            } else {
                                let selected = self.game.select_at_additive(world, self.shift_down);
                                tracing::debug!(?selected, additive = self.shift_down, "click selection");
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
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    match &event.logical_key {
                        Key::Named(NamedKey::Escape) => event_loop.exit(),
                        Key::Character(text) if text.eq_ignore_ascii_case("a") => {
                            let count = self.game.select_all_owned(0);
                            tracing::debug!(count, "select all GDI units");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("1") => {
                            let queued = self.game.queue_unit(0, UnitType::InfantryE1);
                            tracing::debug!(queued, "queue GDI infantry");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("2") => {
                            let queued = self.game.queue_unit(0, UnitType::MediumTank);
                            tracing::debug!(queued, "queue GDI medium tank");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("3") => {
                            let queued = self.game.queue_unit(0, UnitType::MammothTank);
                            tracing::debug!(queued, "queue GDI mammoth tank");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("4") => {
                            let queued = self.game.queue_unit(0, UnitType::Harvester);
                            tracing::debug!(queued, "queue GDI harvester");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("5") => {
                            let queued = self.game.queue_building(0, StructureType::PowerPlant);
                            let status = self.game.status();
                            tracing::debug!(
                                queued,
                                tib = status.gdi_tiberium,
                                credits = status.gdi_credits,
                                "queue GDI power plant"
                            );
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("6") => {
                            let queued = self.game.queue_building(0, StructureType::Refinery);
                            tracing::debug!(queued, "queue GDI refinery");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("7") => {
                            let queued = self.game.queue_building(0, StructureType::Barracks);
                            tracing::debug!(queued, "queue GDI barracks");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("8") => {
                            let queued = self.game.queue_building(0, StructureType::WarFactory);
                            tracing::debug!(queued, "queue GDI war factory");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("9") => {
                            let queued = self.game.queue_building(0, StructureType::Silo);
                            tracing::debug!(queued, "queue GDI silo");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("0") => {
                            let queued = self.game.queue_unit(0, UnitType::Orca);
                            tracing::debug!(queued, "queue GDI orca");
                        }
                        Key::Character(text) if text.eq_ignore_ascii_case("i") => {
                            if let Some(world) = self.cursor_world() {
                                let tile = cnc_tiberian_dawn::map::TILE_SIZE;
                                let cell = Vec2i::new(
                                    (world.x.to_f32() / tile).floor() as i32,
                                    (world.y.to_f32() / tile).floor() as i32,
                                );
                                let fired = self.game.fire_ion_cannon(0, cell);
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
                                let fired = self.game.fire_nuke(0, cell);
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

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();

    let kind = RendererKind::parse(&args.renderer)?;
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
    let mut app = LauncherApp::new(kind, args.width.max(1), args.height.max(1), rules)?;

    tracing::info!(
        "starting launcher (renderer={}, {}x{})",
        kind.label(),
        args.width,
        args.height
    );

    let event_loop = EventLoop::new().context("failed to create the winit event loop")?;
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app).context("event loop terminated with an error")
}
