//! Standalone interactive map editor, plus a headless self-test mode.
//!
//! GUI:  cargo run -p cnc-map-editor
//! Test: cargo run -p cnc-map-editor -- --headless --seed 42 --screenshot /tmp/editor.png
//!
//! Terrain rendering defaults to the generated cnc32 32px asset pack
//! composed via `cnc_asset::classic_pack::PackTileset` (`--headless`
//! screenshots included): sprite flags: `--tileset <dir>` overrides pack
//! discovery (must be a cnc32-style pack root; an invalid override warns
//! and falls back to flat colored quads like `--no-tileset` does), and
//! `--no-tileset` forces the flat colored quads. Discovery when `--tileset`
//! is omitted follows `Cnc32Pack::fallback_dir()`: the `CNC_CNC32_PACK_DIR`
//! env var, then `assets/packs/cnc32_assets_v6_2_1` relative to the
//! workspace/`cnc-asset` crate, then the working directory. The legacy
//! 16px pack and its `--tileset-palette` flag are gone.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use cnc_core::{Fixed16_16, Vec2};
use cnc_map_editor::app::{EditorState, PointerButton};
use cnc_map_editor::generator::{generate, MapGenSettings, OverlayType, TerrainType};
use cnc_map_editor::pack_tileset::PackTileset;
use cnc_map_editor::plugin::Theater;
use cnc_map_editor::presenter::SoftPresenter;
use cnc_renderer::{Camera, Renderer, SoftwareRenderer, WgpuRenderer};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, ModifiersState, NamedKey};
use winit::window::{Window, WindowId};

const FRAME_TIME: Duration = Duration::from_millis(33);

#[derive(Debug, Clone)]
struct Options {
    headless: bool,
    width: u32,
    height: u32,
    seed: Option<u64>,
    size: Option<u32>,
    players: Option<u8>,
    theater: Option<Theater>,
    name: Option<String>,
    player: Option<String>,
    briefing: Option<String>,
    objectives: Vec<String>,
    out: Option<PathBuf>,
    screenshot: Option<PathBuf>,
    frames: Option<u64>,
    renderer: String,
    validate: Option<PathBuf>,
    export_templates: Option<PathBuf>,
    /// Tileset pack root directory override (cnc32-style pack); omit to
    /// discover via `Cnc32Pack::fallback_dir()`.
    tileset: Option<PathBuf>,
    /// Disable the tileset pack entirely (fall back to flat colored quads).
    no_tileset: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            headless: false,
            width: 1400,
            height: 900,
            seed: None,
            size: None,
            players: None,
            theater: None,
            name: None,
            player: None,
            briefing: None,
            objectives: Vec::new(),
            out: None,
            screenshot: None,
            frames: None,
            renderer: "auto".to_string(),
            validate: None,
            export_templates: None,
            tileset: None,
            no_tileset: false,
        }
    }
}

fn parse_args() -> Result<Options, String> {
    let mut options = Options::default();
    let mut args = std::env::args().skip(1).peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--headless" => options.headless = true,
            "--validate" => {
                options.validate =
                    Some(PathBuf::from(args.next().ok_or("--validate needs a path")?))
            }
            "--export-templates" => {
                options.export_templates = Some(PathBuf::from(
                    args.next().ok_or("--export-templates needs a directory")?,
                ))
            }
            "--seed" => {
                options.seed = Some(
                    args.next()
                        .ok_or("--seed needs a value")?
                        .parse()
                        .map_err(|_| "--seed must be an integer")?,
                )
            }
            "--size" => {
                options.size = Some(
                    args.next()
                        .ok_or("--size needs a value")?
                        .parse()
                        .map_err(|_| "--size must be an integer")?,
                )
            }
            "--players" => {
                options.players = Some(
                    args.next()
                        .ok_or("--players needs a value")?
                        .parse()
                        .map_err(|_| "--players must be 1..8")?,
                )
            }
            "--theater" => {
                let name = args.next().ok_or("--theater needs a value")?;
                options.theater = Some(match name.to_ascii_lowercase().as_str() {
                    "temperate" => Theater::Temperate,
                    "snow" => Theater::Snow,
                    "desert" => Theater::Desert,
                    "urban" => Theater::Urban,
                    other => return Err(format!("unknown theater '{other}'")),
                });
            }
            "--name" => options.name = Some(args.next().ok_or("--name needs text")?.to_string()),
            "--player" => {
                options.player = Some(args.next().ok_or("--player needs text")?.to_string())
            }
            "--briefing" => {
                options.briefing = Some(args.next().ok_or("--briefing needs text")?.to_string())
            }
            "--objective" => {
                options
                    .objectives
                    .push(args.next().ok_or("--objective needs text")?.to_string());
            }
            "--out" => options.out = Some(PathBuf::from(args.next().ok_or("--out needs a path")?)),
            "--screenshot" => {
                options.screenshot = Some(PathBuf::from(
                    args.next().ok_or("--screenshot needs a path")?,
                ))
            }
            "--frames" => {
                options.frames = Some(
                    args.next()
                        .ok_or("--frames needs a value")?
                        .parse()
                        .map_err(|_| "--frames must be an integer")?,
                )
            }
            "--renderer" => options.renderer = args.next().ok_or("--renderer needs a value")?,
            "--width" => {
                options.width = args
                    .next()
                    .ok_or("--width needs a value")?
                    .parse()
                    .map_err(|_| "--width must be an integer")?
            }
            "--height" => {
                options.height = args
                    .next()
                    .ok_or("--height needs a value")?
                    .parse()
                    .map_err(|_| "--height must be an integer")?
            }
            "--tileset" => {
                options.tileset = Some(PathBuf::from(args.next().ok_or("--tileset needs a path")?));
            }
            "--no-tileset" => options.no_tileset = true,
            "--help" | "-h" => {
                println!(
                    "cnc-map-editor [--headless] [--seed N] [--size N] [--players N]\n\
                     \t[--theater temperate|snow|desert|urban] [--name TEXT] [--player SIDE]\n\
                     \t[--briefing TEXT] [--objective TEXT]... [--out MAP.ini]\n\
                     \t[--screenshot PNG] [--frames N] [--width N] [--height N]\n\
                     \t[--renderer auto|wgpu|software] [--validate MAP.ini|DIR]\n\
                     \t[--export-templates DIR] [--tileset DIR] [--no-tileset]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument '{other}'")),
        }
    }
    Ok(options)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = parse_args().map_err(|error| {
        eprintln!("argument error: {error}");
        error
    })?;
    match options.renderer.to_ascii_lowercase().as_str() {
        "auto" | "wgpu" | "software" => {}
        other => {
            eprintln!("argument error: unknown renderer '{other}' (expected auto|wgpu|software)");
            return Err("invalid --renderer value".into());
        }
    }
    if let Some(dir) = &options.export_templates {
        return run_export_templates(dir);
    }
    if let Some(path) = &options.validate {
        return run_validate(path);
    }
    if options.headless {
        run_headless(options)
    } else {
        run_gui(options)
    }
}

/// Validate a scenario file without opening a window.
fn run_validate(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    if path.is_dir() {
        return run_validate_dir(path);
    }
    let text = std::fs::read_to_string(path)?;
    let map = cnc_map_editor::map::MapModel::from_ini(&text)?;
    let issues = map.validate();
    println!("validated: {} ({})", path.display(), map.summary());
    if issues.is_empty() {
        println!("OK: no validation issues");
        Ok(())
    } else {
        for issue in &issues {
            eprintln!("issue: {issue}");
        }
        Err(format!("{} validation issue(s)", issues.len()).into())
    }
}

fn run_validate_dir(dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
        })
        .collect();
    paths.sort();
    if paths.is_empty() {
        return Err(format!("no .ini scenarios found in {}", dir.display()).into());
    }
    let mut failures = 0usize;
    let mut valid = 0usize;
    for path in &paths {
        let text = std::fs::read_to_string(path)?;
        let map = cnc_map_editor::map::MapModel::from_ini(&text)?;
        let issues = map.validate();
        if issues.is_empty() {
            valid += 1;
            println!("{}: OK ({})", path.display(), map.summary());
        } else {
            failures += issues.len();
            eprintln!("{}: {} issue(s)", path.display(), issues.len());
            for issue in issues {
                eprintln!("  - {issue}");
            }
        }
    }
    if failures == 0 {
        println!("OK: {valid}/{} scenarios valid", paths.len());
        Ok(())
    } else {
        Err(format!("{failures} issue(s) across {} scenario(s)", paths.len()).into())
    }
}

fn run_export_templates(dir: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(dir)?;
    let mut empty = cnc_map_editor::map::MapModel::new(32, 32);
    empty.player_starts.push(cnc_core::Vec2i::new(2, 2));
    empty.theater = "Temperate".to_string();

    let duel = generate(&MapGenSettings {
        seed: 1,
        width: 64,
        height: 64,
        player_count: 2,
        ..MapGenSettings::default()
    });
    let arena = generate(&MapGenSettings {
        seed: 42,
        width: 48,
        height: 48,
        player_count: 4,
        ..MapGenSettings::default()
    });

    for (name, map) in [
        ("template_empty_32.ini", &empty),
        ("template_duel_64.ini", &duel),
        ("template_arena_48.ini", &arena),
    ] {
        let path = dir.join(name);
        std::fs::write(&path, map.to_ini())?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

/// Load the configured cnc32 pack tileset; on failure warn once and fall
/// back to flat colored terrain quads. `--tileset <dir>` is an explicit
/// override that must be a cnc32-style pack; omission enables discovery
/// (`Cnc32Pack::fallback_dir()`).
fn load_tileset(options: &Options) -> Option<Arc<PackTileset>> {
    if options.no_tileset {
        return None;
    }
    let loaded = if let Some(dir) = options.tileset.as_deref() {
        PackTileset::load(std::path::Path::new(dir))
    } else {
        PackTileset::from_fallback().ok_or_else(|| String::from("no cnc32 pack directory found"))
    };
    match loaded {
        Ok(pack) => {
            println!(
                "tileset pack loaded: {}x{} composed atlas (tile size {})",
                pack.width(),
                pack.height(),
                pack.tile_size()
            );
            Some(Arc::new(pack))
        }
        Err(_) => {
            eprintln!(
                "warning: cnc32 asset pack unavailable{}; \
                 falling back to flat colored terrain",
                match &options.tileset {
                    Some(dir) => format!(" (override '{}')", dir.display()),
                    None => String::new(),
                }
            );
            None
        }
    }
}

fn configure_state(options: &Options) -> EditorState {
    let mut state = EditorState::new(options.width as f32, options.height as f32);
    state.tileset = load_tileset(options);
    if let Some(size) = options.size {
        state.map_width = size;
        state.map_height = size;
        state.map_width_text = size.to_string();
        state.map_height_text = size.to_string();
    }
    if let Some(players) = options.players {
        state.players = players.clamp(1, 8);
    }
    if let Some(theater) = options.theater {
        state.theater = theater;
    }
    if let Some(seed) = options.seed {
        state.seed = seed.to_string();
    }
    state.generate();
    if let Some(name) = &options.name {
        state.map.name = name.clone();
    }
    if let Some(player) = &options.player {
        state.map.player = player.clone();
    }
    if let Some(briefing) = &options.briefing {
        state.map.briefing = briefing.clone();
    }
    if !options.objectives.is_empty() {
        state.map.objectives = options.objectives.clone();
    }
    state
}

fn run_headless(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = configure_state(&options);

    // Exercise an edit through the same undoable path the GUI uses.
    let probe = cnc_core::Vec2i::new(2, 2);
    if state.map.in_bounds(probe) {
        let id = state.map.insert_object(cnc_map_editor::map::MapObject {
            id: 0,
            class_id: cnc_core::ClassId::InfantryE1,
            position: probe,
            owner: None,
            ..Default::default()
        });
        state.editor.delete_at(&mut state.map, probe);
        state.editor.undo(&mut state.map);
        if !state.map.objects.contains_key(&id) {
            return Err("headless edit/undo roundtrip failed".into());
        }
        // Remove the probe again so the saved demo map only contains the
        // generated scenario (MCVs at the player starts).
        state.editor.delete_at(&mut state.map, probe);
        if state.map.objects.contains_key(&id) {
            return Err("headless probe cleanup failed".into());
        }
    }

    let issues = state.map.validate();
    let mut histogram = std::collections::BTreeMap::new();
    for id in &state.map.terrain {
        *histogram.entry(*id).or_insert(0usize) += 1;
    }
    let overlays = state
        .map
        .overlay
        .iter()
        .filter(|id| **id != OverlayType::None.id())
        .count();
    println!(
        "generated: {} (seed {}, players {})",
        state.map.summary(),
        state.seed,
        state.players
    );
    println!(
        "terrain histogram: {:?}",
        histogram
            .iter()
            .map(|(id, count)| (
                TerrainType::from_id(*id).map_or("?", TerrainType::label),
                *count
            ))
            .collect::<Vec<_>>()
    );
    println!("overlay cells: {overlays}");
    println!("player starts: {:?}", state.map.player_starts);
    if !issues.is_empty() {
        eprintln!("validation failed: {issues:?}");
        return Err("generated map failed validation".into());
    }

    let out = options
        .out
        .clone()
        .unwrap_or_else(|| PathBuf::from("maps/headless_map.ini"));
    state.path = out.display().to_string();
    state.save();
    if !out.exists() {
        return Err(format!("failed to save {}: {}", out.display(), state.status).into());
    }
    println!(
        "saved: {} ({} bytes)",
        out.display(),
        std::fs::metadata(&out)?.len()
    );

    if let Some(path) = &options.screenshot {
        let width = options.width.max(320);
        let height = options.height.max(240);
        let mut renderer = SoftwareRenderer::new(width, height);
        state.render(&mut renderer, width as f32, height as f32);
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let mut rgba = Vec::with_capacity((width * height * 4) as usize);
        for pixel in renderer.buffer() {
            rgba.push(((pixel >> 16) & 0xff) as u8);
            rgba.push(((pixel >> 8) & 0xff) as u8);
            rgba.push((pixel & 0xff) as u8);
            rgba.push(((pixel >> 24) & 0xff) as u8);
        }
        image::save_buffer(path, &rgba, width, height, image::ColorType::Rgba8)?;
        println!("screenshot: {}", path.display());
    }

    println!("headless self-test OK");
    Ok(())
}

fn screen_camera(width: u32, height: u32) -> Camera {
    let mut camera = Camera::default();
    camera.set_viewport(width.max(1), height.max(1));
    // With position = viewport centre and zoom 1, world coordinates equal
    // screen pixels, so the editor can draw in screen space for both backends.
    camera.position = Vec2::new(
        Fixed16_16::from_f32(width as f32 * 0.5),
        Fixed16_16::from_f32(height as f32 * 0.5),
    );
    camera.zoom = Fixed16_16::ONE;
    camera
}

/// Rendering/presentation backend for the interactive window.
enum Backend {
    Wgpu(Box<dyn Renderer>),
    Software {
        renderer: SoftwareRenderer,
        presenter: SoftPresenter,
    },
    /// Last-resort fallback when neither wgpu nor a CPU window surface works.
    Offscreen(SoftwareRenderer),
}

impl Backend {
    fn label(&self) -> &'static str {
        match self {
            Backend::Wgpu(_) => "wgpu",
            Backend::Software { .. } => "software",
            Backend::Offscreen(_) => "software (offscreen)",
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        match self {
            Backend::Wgpu(renderer) => renderer.resize(width, height),
            Backend::Software {
                renderer,
                presenter,
            } => {
                renderer.resize(width, height);
                if let Err(error) = presenter.resize(width, height) {
                    eprintln!("software surface resize failed: {error}");
                }
            }
            Backend::Offscreen(renderer) => renderer.resize(width, height),
        }
    }
}

fn create_backend(mode: &str, window: Arc<Window>, width: u32, height: u32) -> Backend {
    let forced_software = mode.eq_ignore_ascii_case("software");
    if !forced_software {
        match pollster::block_on(WgpuRenderer::new(window.clone())) {
            Ok(renderer) => {
                println!("wgpu renderer ready ({width}x{height})");
                return Backend::Wgpu(Box::new(renderer));
            }
            Err(error) => {
                eprintln!(
                    "wgpu unavailable ({error}); falling back to the presenting software renderer"
                );
            }
        }
    } else {
        println!("software renderer forced; presenting CPU frames through softbuffer");
    }

    match SoftPresenter::new(window) {
        Ok(mut presenter) => {
            let _ = presenter.resize(width, height);
            println!("softbuffer presenter ready ({width}x{height})");
            Backend::Software {
                renderer: SoftwareRenderer::new(width, height),
                presenter,
            }
        }
        Err(error) => {
            eprintln!("softbuffer presentation unavailable ({error}); rendering offscreen only");
            Backend::Offscreen(SoftwareRenderer::new(width, height))
        }
    }
}

struct EditorApp {
    state: EditorState,
    window: Option<Arc<Window>>,
    backend: Option<Backend>,
    renderer_mode: String,
    width: u32,
    height: u32,
    frames: u64,
    frame_limit: Option<u64>,
    next_frame: Instant,
    modifiers: ModifiersState,
}
impl EditorApp {
    fn new(options: &Options) -> Self {
        let state = configure_state(options);
        Self {
            state,
            window: None,
            backend: None,
            renderer_mode: options.renderer.clone(),
            width: options.width.max(640),
            height: options.height.max(480),
            frames: 0,
            frame_limit: options.frames,
            next_frame: Instant::now(),
            modifiers: ModifiersState::empty(),
        }
    }

    fn draw(&mut self) {
        let width = self.width;
        let height = self.height;
        let camera = screen_camera(width, height);
        let limit = self.frame_limit;
        let Some(backend) = self.backend.as_mut() else {
            return;
        };
        match backend {
            Backend::Wgpu(renderer) => {
                renderer.set_camera(&camera);
                renderer.begin_frame();
                self.state
                    .render(&mut **renderer, width as f32, height as f32);
                if let Err(error) = renderer.end_frame() {
                    eprintln!("frame presentation failed: {error}");
                }
            }
            Backend::Software {
                renderer,
                presenter,
            } => {
                renderer.begin_frame();
                self.state.render(renderer, width as f32, height as f32);
                if let Err(error) = renderer.end_frame() {
                    eprintln!("software frame failed: {error}");
                }
                if let Err(error) = presenter.present(renderer.buffer(), width, height) {
                    eprintln!("software presentation failed: {error}");
                }
            }
            Backend::Offscreen(renderer) => {
                renderer.begin_frame();
                self.state.render(renderer, width as f32, height as f32);
                let _ = renderer.end_frame();
            }
        }
        self.frames += 1;
        if limit.is_some_and(|limit| self.frames >= limit) {
            if let Some(window) = &self.window {
                window.set_title("cnc-map-editor (finished)");
            }
        }
    }
}

impl ApplicationHandler for EditorApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title("C&C Remastered - Map Editor (procedural 2D)")
            .with_inner_size(LogicalSize::new(self.width, self.height));
        let window = match event_loop.create_window(attributes) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                eprintln!("failed to create window: {error}");
                event_loop.exit();
                return;
            }
        };
        let size = window.inner_size();
        self.width = size.width.max(1);
        self.height = size.height.max(1);
        self.state.resize(self.width as f32, self.height as f32);
        if let Err(error) = self.state_map_save_dir() {
            eprintln!("{error}");
        }

        let backend = create_backend(&self.renderer_mode, window.clone(), self.width, self.height);
        println!("presentation backend: {}", backend.label());
        self.backend = Some(backend);
        self.window = Some(window);
        self.next_frame = Instant::now() + FRAME_TIME;
        println!(
            "editor ready: {} seed {} players {}",
            self.state.map.summary(),
            self.state.seed,
            self.state.players
        );
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let scale = self
            .window
            .as_ref()
            .map_or(1.0, |window| window.scale_factor() as f32);
        let _ = scale;
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.width = size.width.max(1);
                self.height = size.height.max(1);
                if let Some(backend) = self.backend.as_mut() {
                    backend.resize(self.width, self.height);
                }
                self.state.resize(self.width as f32, self.height as f32);
            }
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::CursorMoved { position, .. } => {
                self.state
                    .pointer_move(position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pointer = match button {
                    MouseButton::Left => PointerButton::Left,
                    MouseButton::Right => PointerButton::Right,
                    MouseButton::Middle => PointerButton::Middle,
                    _ => return,
                };
                match state {
                    ElementState::Pressed => {
                        if let Some(position) = self.state.mouse {
                            self.state.pointer_down(position.0, position.1, pointer);
                        }
                    }
                    ElementState::Released => self.state.pointer_up(pointer),
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let amount = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(position) => position.y as f32 / 40.0,
                };
                self.state.scroll(amount);
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
                self.state.shift_held = self.modifiers.shift_key();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state != ElementState::Pressed {
                    return;
                }
                let ctrl = self.modifiers.control_key();
                match &event.logical_key {
                    Key::Named(NamedKey::Escape) => {
                        if self.state.map_info_open() {
                            self.state.close_map_info(false);
                        } else if self.state.help_open() {
                            self.state.toggle_help();
                        } else if self.state.focus == cnc_map_editor::app::Focus::None
                            && !self.state.agent_studio
                        {
                            event_loop.exit();
                        } else {
                            self.state.key_escape();
                        }
                    }
                    Key::Named(NamedKey::F1) => self.state.toggle_help(),
                    Key::Named(NamedKey::Backspace) => self.state.key_backspace(),
                    Key::Named(NamedKey::Enter) => self.state.key_enter(),
                    Key::Named(NamedKey::Delete) => self.state.delete_at_cursor(),
                    Key::Named(NamedKey::Tab) => self.state.toggle_grid(),
                    Key::Named(NamedKey::ArrowLeft) => self.state.key_left(),
                    Key::Named(NamedKey::ArrowRight) => self.state.key_right(),
                    Key::Named(NamedKey::ArrowUp) => self.state.key_up(),
                    Key::Named(NamedKey::ArrowDown) => self.state.key_down(),
                    Key::Named(NamedKey::Home) => self.state.key_home(),
                    Key::Named(NamedKey::End) => self.state.key_end(),
                    Key::Named(NamedKey::F5) => self.state.generate(),
                    Key::Character(text) if ctrl => match text.to_ascii_lowercase().as_str() {
                        "s" => self.state.save_with_dialog(),
                        "o" => self.state.open_with_dialog(),
                        "z" => self.state.undo(),
                        "y" => self.state.redo(),
                        "n" => {
                            state_new(&mut self.state);
                        }
                        "i" => {
                            if self.state.map_info_open() {
                                self.state.close_map_info(false);
                            } else {
                                self.state.open_map_info();
                            }
                        }
                        "g" => self.state.generate(),
                        "c" => self.state.copy_selection(),
                        "v" => self.state.paste_selection(),
                        "d" => self.state.duplicate_selection(),
                        "a" => {
                            if self.modifiers.shift_key() {
                                self.state.clear_object_selection();
                            } else {
                                self.state.select_all_objects();
                            }
                        }
                        _ => {}
                    },
                    Key::Character(text) => self.state.key_text(text),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if now >= self.next_frame {
            self.next_frame = now + FRAME_TIME;
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
        if self
            .frame_limit
            .is_some_and(|limit| self.frames >= limit && self.frames > 0)
        {
            event_loop.exit();
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

fn state_new(state: &mut EditorState) {
    let width = state.map_width;
    let height = state.map_height;
    let theater = state.theater;
    state.import_warnings.clear();
    state.map = cnc_map_editor::map::MapModel::new(width, height);
    state.map.theater = theater.name().to_string();
    state.editor = cnc_map_editor::ui::EditorUI::new();
    // Give the legacy view the same tileset pack for its fallback renderer.
    state.editor.tileset = state.tileset.clone();
    state.status = format!("new empty {width}x{height} map");
    state.fit();
}

impl EditorApp {
    fn state_map_save_dir(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all("maps")
    }

    /// Tear down GPU state before the winit window. On Wayland the surface must
    /// not outlive the window, and dropping the renderer first avoids a
    /// segfault in the graphics driver during process teardown.
    fn shutdown(&mut self) {
        self.backend = None;
        self.window = None;
    }
}

fn run_gui(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = EditorApp::new(&options);
    event_loop.run_app(&mut app)?;
    println!(
        "editor closed after {} frames (map: {})",
        app.frames,
        app.state.map.summary()
    );
    app.shutdown();
    Ok(())
}
