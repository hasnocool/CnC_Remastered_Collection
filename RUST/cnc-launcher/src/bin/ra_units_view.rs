//! Screenshot capture for the RA game view with pack unit sprites.
//!
//! Drives the RA engine's own render path through a software renderer, after
//! uploading the pack unit atlas into texture slot 0 exactly like the GUI's
//! resume does.
//!
//! Note: `SoftwareRenderer::set_camera` is a no-op, so this capture renders at
//! raw world coordinates - entity sprites appear at their world pixel position
//! (top-left of the frame). The wgpu path the GUI uses honours the camera.
//! Zoom in on a unit's world x/y to inspect its sprite.
//!
//! Run: cargo run -p cnc-launcher --bin ra_units_view -- [out.png]
use anyhow::Result;
use cnc_red_alert::Game;
use cnc_renderer::{Camera, Renderer, SoftwareRenderer};

fn main() -> Result<()> {
    let out = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/ra_units_view.png".to_string());

    let mut renderer = SoftwareRenderer::new(1280, 720);
    let mut camera = Camera::default();
    camera.set_viewport(1280, 720);

    // Pack unit atlas into texture slot 0 (the GUI uploads it on resume).
    match cnc_asset::unit_atlas::UnitAtlas::from_fallback() {
        Some(pack) => {
            let (width, height) = pack.animation_dims();
            renderer
                .set_atlas(width, height, pack.animation_rgba())
                .map_err(|e| anyhow::anyhow!("unit atlas upload failed: {e}"))?;
            println!(
                "pack unit atlas ready: {width}x{height}, {} unit kinds",
                pack.units().count()
            );
        }
        None => {
            println!("pack unit atlas unavailable - units fall back to flat quads");
            return Ok(());
        }
    }

    let mut game = Game::new(64, 64);
    // Reveal around every unit so the shroud gate passes (the launcher reveals
    // as the player explores; a static capture needs it up front).
    for unit in game.units.iter() {
        game.shroud.reveal(0, unit.cell, 4);
    }

    println!(
        "map {}x{} tile {} ; {} units, {} structures",
        game.map.width,
        game.map.height,
        cnc_red_alert::TILE_SIZE,
        game.units.len(),
        game.structures.len()
    );
    for unit in game.units.iter().take(4) {
        println!(
            "  cell {:?} world ({:.0},{:.0}) visible {}",
            unit.cell,
            unit.position.x.to_f32(),
            unit.position.y.to_f32(),
            game.shroud.is_visible(0, unit.cell)
        );
    }

    renderer.begin_frame();
    renderer.set_camera(&camera);
    game.render(&mut renderer);
    renderer.end_frame()?;

    let buffer = renderer.buffer();
    let mut img = image::RgbaImage::new(1280, 720);
    for (index, &pixel) in buffer.iter().enumerate() {
        let x = (index % 1280) as u32;
        let y = (index / 1280) as u32;
        img.put_pixel(
            x,
            y,
            image::Rgba([
                ((pixel >> 16) & 0xFF) as u8,
                ((pixel >> 8) & 0xFF) as u8,
                (pixel & 0xFF) as u8,
                ((pixel >> 24) & 0xFF) as u8,
            ]),
        );
    }
    img.save(&out)?;
    println!("Saved {out}");
    Ok(())
}
