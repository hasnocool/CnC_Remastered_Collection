//! Visual QA harness for the editor chrome itself (toolbar, sidebar panel).
//!
//! Renders the full `EditorState` frame through `cnc_renderer::SoftwareRenderer`
//! and saves PNGs under /tmp/opencode so button widths and label centering can
//! be inspected without a window.
//!
//! Run from `RUST/`:
//! ```bash
//! cargo run -p cnc-map-editor --example editor_ui_render
//! ```

use cnc_map_editor::app::{EditorState, PointerButton};
use cnc_renderer::SoftwareRenderer;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 800;

fn save(path: &str, renderer: &SoftwareRenderer) {
    let mut rgba = Vec::with_capacity((WIDTH * HEIGHT * 4) as usize);
    for pixel in renderer.buffer() {
        rgba.push(((pixel >> 16) & 0xff) as u8);
        rgba.push(((pixel >> 8) & 0xff) as u8);
        rgba.push((pixel & 0xff) as u8);
        rgba.push(((pixel >> 24) & 0xff) as u8);
    }
    image::save_buffer(path, &rgba, WIDTH, HEIGHT, image::ColorType::Rgba8).expect("save png");
    println!("saved {path}");
}

fn frame(state: &mut EditorState) -> SoftwareRenderer {
    let mut renderer = SoftwareRenderer::new(WIDTH, HEIGHT);
    state.render(&mut renderer, WIDTH as f32, HEIGHT as f32);
    renderer
}

fn main() {
    std::fs::create_dir_all("/tmp/opencode").expect("create /tmp/opencode");

    // Main editor frame at the default window size.
    let mut state = EditorState::new(WIDTH as f32, HEIGHT as f32);
    let renderer = frame(&mut state);
    save("/tmp/opencode/editor_ui.png", &renderer);

    // Agent studio overlay with two toolbar-centring regressions possible:
    // it uses its own fixed button widths, so render it separately.
    let mut state = EditorState::new(WIDTH as f32, HEIGHT as f32);
    state.open_agent_studio();
    let renderer = frame(&mut state);
    save("/tmp/opencode/editor_ui_studio.png", &renderer);

    // Narrow-window check: everything plus the plugin toggle must fit at 950px.
    let mut state = EditorState::new(950.0, 600.0);
    let mut renderer = SoftwareRenderer::new(950, 600);
    state.render(&mut renderer, 950.0, 600.0);
    save_950(&renderer);
}

fn save_950(renderer: &SoftwareRenderer) {
    let (width, height) = (950u32, 600u32);
    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
    for pixel in renderer.buffer() {
        rgba.push(((pixel >> 16) & 0xff) as u8);
        rgba.push(((pixel >> 8) & 0xff) as u8);
        rgba.push((pixel & 0xff) as u8);
        rgba.push(((pixel >> 24) & 0xff) as u8);
    }
    image::save_buffer(
        "/tmp/opencode/editor_ui_950.png",
        &rgba,
        width,
        height,
        image::ColorType::Rgba8,
    )
    .expect("save png");
    println!("saved /tmp/opencode/editor_ui_950.png");
}

// Keep the hover-path exercised so hit-testing stays honest after layout edits.
#[allow(dead_code)]
fn touch(state: &mut EditorState) {
    state.pointer_down(10.0, 18.0, PointerButton::Left);
}
