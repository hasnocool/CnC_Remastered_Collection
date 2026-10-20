//! Screenshot capture for C&C Launcher and Map Editor
use std::path::PathBuf;
use anyhow::Result;
use cnc_core::{Fixed16_16, RulesDatabase, Vec2};
use cnc_renderer::{Camera, Renderer, SoftwareRenderer, WgpuRenderer};
use cnc_launcher::menu::{Menu, Screen};

fn save_screenshot(renderer: &mut dyn Renderer, width: u32, height: u32, path: &str) -> Result<()> {
    // For SoftwareRenderer, we can access the buffer directly
    if let Some(software) = renderer.as_any().downcast_ref::<SoftwareRenderer>() {
        let buffer = software.buffer();
        let mut img = image::RgbaImage::new(width, height);
        for (i, &pixel) in buffer.iter().enumerate() {
            let x = (i % width as usize) as u32;
            let y = (i / width as usize) as u32;
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            let a = ((pixel >> 24) & 0xFF) as u8;
            img.put_pixel(x, y, image::Rgba([r, g, b, a]));
        }
        img.save(path)?;
        println!("Saved screenshot to {}", path);
    }
    Ok(())
}

fn main() -> Result<()> {
    // Create software renderer
    let mut renderer = SoftwareRenderer::new(1280, 720);
    
    // Create camera
    let mut camera = Camera::default();
    camera.set_viewport(1280, 720);
    
    // Create menu
    let rules = RulesDatabase::default();
    let mut menu = Menu::new(rules, 1280, 720);
    
    // Render main menu
    renderer.begin_frame();
    renderer.set_camera(&camera);
    menu.draw(&mut renderer, 1280, 720);
    renderer.end_frame().unwrap();
    
    save_screenshot(&mut renderer, 1280, 720, "/tmp/main_menu.png")?;
    
    // Navigate to Campaign screen
    menu.on_key(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown));
    menu.on_key(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown));
    menu.activate();
    
    renderer.begin_frame();
    renderer.set_camera(&camera);
    menu.draw(&mut renderer, 1280, 720);
    renderer.end_frame().unwrap();
    
    save_screenshot(&mut renderer, 1280, 720, "/tmp/campaign_menu.png")?;
    
    // Navigate to Local Game screen
    menu.on_key(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown));
    menu.activate();
    
    renderer.begin_frame();
    renderer.set_camera(&camera);
    menu.draw(&mut renderer, 1280, 720);
    renderer.end_frame().unwrap();
    
    save_screenshot(&mut renderer, 1280, 720, "/tmp/local_game_menu.png")?;
    
    // Navigate to Online Game screen
    menu.on_key(&winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown));
    menu.activate();
    
    renderer.begin_frame();
    renderer.set_camera(&camera);
    menu.draw(&mut renderer, 1280, 720);
    renderer.end_frame().unwrap();
    
    save_screenshot(&mut renderer, 1280, 720, "/tmp/online_game_menu.png")?;
    
    println!("All screenshots saved!");
    Ok(())
}
