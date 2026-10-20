//! Visual QA harness: render an editor-style map through the exact
//! `tiles::base_draw` / `sprite_draws` pick logic and save a PNG so tile edge
//! matching can be inspected without a window.
//!
//! Layout: two panels side by side.
//! - LEFT: sprites drawn at the exact cell size (16 px for a 16 px cell).
//! - RIGHT: sprites drawn at `TILE_SIZE + 0.5`, matching the launcher and TD
//!   engine paths, so half-texel bleed is directly comparable.
//!
//! Terrain: grass field with a concrete patch, a water lake with shore, a rock
//! ridge, dirt/sand/rock boundaries and a tiberium overlay corner.
//!
//! Run from `RUST/` so `load_default` resolves the in-repo pack:
//! ```bash
//! cargo run -p cnc-map-editor --example tileset_render
//! ```

// The QA harness draw helpers thread pack/model/geometry parameters explicitly,
// matching the launcher/editor convention; a struct would just reshuffle them.
#![allow(clippy::too_many_arguments)]
use cnc_asset::tileset_pack::{load_default, TilesetPack, GRID};
use cnc_core::{Fixed16_16, Rect, Vec2};
use cnc_map_editor::generator::TerrainType;
use cnc_map_editor::map::MapModel;
use cnc_map_editor::tiles::{self, DECORATIONS, RESOURCES_FX};
use cnc_renderer::{Renderer, SoftwareRenderer, Sprite, TILESET_TEXTURE_ID};

const CELL: u32 = 16;
const MAP: u32 = 40;
const PANEL_GAP: u32 = 24;
const MARGIN: u32 = 16;
const SCALE: u32 = 2;

fn point(model: &mut MapModel, x: u32, y: u32, terrain: TerrainType) {
    let index = (y * model.width + x) as usize;
    model.terrain[index] = terrain.id();
}

fn build_map() -> MapModel {
    let mut model = MapModel::new(MAP, MAP);
    model.name = "tileset qa".to_string();
    model.elevation = vec![0; (MAP * MAP) as usize];
    model.overlay = vec![0; (MAP * MAP) as usize];
    for y in 0..MAP {
        for x in 0..MAP {
            point(&mut model, x, y, TerrainType::Grass);
        }
    }
    // Concrete (urban) patch in the upper middle.
    for y in 4..14 {
        for x in 10..24 {
            point(&mut model, x, y, TerrainType::Concrete);
        }
    }
    // Dirt band under the concrete.
    for y in 14..20 {
        for x in 6..28 {
            point(&mut model, x, y, TerrainType::Dirt);
        }
    }
    // Rock ridge on the right.
    for y in 6..30 {
        for x in 30..MAP {
            point(&mut model, x, y, TerrainType::Rock);
        }
    }
    // Sand strip and water lake in the lower left.
    for y in 24..32 {
        for x in 2..18 {
            point(&mut model, x, y, TerrainType::Sand);
        }
    }
    for y in 28..36 {
        for x in 4..14 {
            point(&mut model, x, y, TerrainType::Water);
        }
    }
    // Tiberium overlay cells in the grass.
    for (x, y) in [(4, 2), (5, 2), (4, 3)] {
        model.overlay[(y * MAP + x) as usize] = 1;
    }
    model
}

fn pack_rect(pack: &TilesetPack, draw: &tiles::TileDraw) -> Option<Rect> {
    let row = pack
        .family_row(draw.category, draw.family)
        .unwrap_or(draw.row)
        .min((GRID - 1) as u8);
    let col = draw.col % (GRID as u8);
    pack.tile_px_rect(draw.category, row, col)
        .map(|(px, py, pw, ph)| Rect::new(px as i32, py as i32, pw as i32, ph as i32))
}

fn draw_cell(
    renderer: &mut dyn Renderer,
    pack: &TilesetPack,
    model: &MapModel,
    x: i32,
    y: i32,
    origin_x: f32,
    origin_y: f32,
    size: f32,
) {
    if let Some(draw) = tiles::base_draw(model, x, y) {
        if let Some(rect) = pack_rect(pack, &draw) {
            let mut sprite = Sprite::new(
                TILESET_TEXTURE_ID,
                Vec2::new(
                    Fixed16_16::from_f32(origin_x),
                    Fixed16_16::from_f32(origin_y),
                ),
                Vec2::new(Fixed16_16::from_f32(size), Fixed16_16::from_f32(size)),
            );
            sprite.uv_rect = rect;
            renderer.draw_sprite(&sprite);
        }
    }
    for draw in tiles::sprite_draws(model, x, y) {
        if draw.category != RESOURCES_FX && draw.category != DECORATIONS {
            continue;
        }
        if let Some(rect) = pack_rect(pack, &draw) {
            let mut sprite = Sprite::new(
                TILESET_TEXTURE_ID,
                Vec2::new(
                    Fixed16_16::from_f32(origin_x),
                    Fixed16_16::from_f32(origin_y),
                ),
                Vec2::new(Fixed16_16::from_f32(size), Fixed16_16::from_f32(size)),
            );
            sprite.uv_rect = rect;
            renderer.draw_sprite(&sprite);
        }
    }
}

fn save(renderer: &SoftwareRenderer, width: u32, height: u32, path: &str) -> anyhow::Result<()> {
    let buffer = renderer.buffer();
    let mut img = image::RgbaImage::new(width, height);
    for (i, &pixel) in buffer.iter().enumerate() {
        let px = (i % width as usize) as u32;
        let py = (i / width as usize) as u32;
        img.put_pixel(
            px,
            py,
            image::Rgba([
                ((pixel >> 16) & 0xff) as u8,
                ((pixel >> 8) & 0xff) as u8,
                (pixel & 0xff) as u8,
                255,
            ]),
        );
    }
    img.save(path)?;
    println!("saved {path}");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Optional CLI arg: render a real editor map INI instead of the synthetic
    // QA map. Uses the same MapModel::from_ini -> tiles::base_draw ->
    // ui_tile_rect pipeline the editor UI draws with.
    let model = match std::env::args().nth(1) {
        Some(path) => {
            let text = std::fs::read_to_string(&path)?;
            let m = MapModel::from_ini(&text)?;
            println!(
                "loaded {path}: {}x{}, theater '{}'",
                m.width, m.height, m.theater
            );
            m
        }
        None => build_map(),
    };
    // Optional tile-pick dump for the lake / sand / shore region (visual QA
    // aid): set CNC_TILESET_QA_DUMP=1 to print base_draw picks.
    if std::env::var("CNC_TILESET_QA_DUMP").as_deref() == Ok("1") {
        for y in 24..38i32 {
            for x in 2..20i32 {
                if let Some(draw) = tiles::base_draw(&model, x, y) {
                    println!(
                        "({x:2},{y:2}) {} / {} row={} col={:2}",
                        draw.category, draw.family, draw.row, draw.col
                    );
                }
            }
        }
    }
    let pack = load_default("td_inspired").expect("in-repo pack must load from RUST/");
    let map_w = model.width.max(1);
    let map_h = model.height.max(1);
    let panel_w = map_w * CELL * SCALE;
    let panel_h = map_h * CELL * SCALE;
    let width = MARGIN * 2 + panel_w * 2 + PANEL_GAP;
    let height = MARGIN * 2 + panel_h;
    let mut renderer = SoftwareRenderer::new(width, height);
    renderer.set_atlas_texture(TILESET_TEXTURE_ID, pack.width(), pack.height(), pack.rgba())?;

    renderer.begin_frame();
    renderer.set_camera(&cnc_renderer::Camera::default());
    // Left panel: exact cell size.
    for y in 0..map_h as i32 {
        for x in 0..map_w as i32 {
            draw_cell(
                &mut renderer,
                &pack,
                &model,
                x,
                y,
                (MARGIN + x as u32 * CELL * SCALE) as f32,
                (MARGIN + y as u32 * CELL * SCALE) as f32,
                (CELL * SCALE) as f32,
            );
        }
    }
    // Right panel: TILE_SIZE + 0.5 in 1:1 texel space, scaled up by SCALE.
    let right_x = (MARGIN + panel_w + PANEL_GAP) as f32;
    for y in 0..map_h as i32 {
        for x in 0..map_w as i32 {
            let cell = (CELL as f32 + 0.5) * SCALE as f32;
            draw_cell(
                &mut renderer,
                &pack,
                &model,
                x,
                y,
                right_x + x as f32 * cell,
                (MARGIN + y as u32 * CELL * SCALE) as f32,
                cell,
            );
        }
    }
    renderer.end_frame()?;
    save(&renderer, width, height, "/tmp/opencode/tileset_qa.png")?;
    Ok(())
}
