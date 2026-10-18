//! Renderer + asset-system bridge for Red Alert entities.
//!
//! Red Alert's entity structs are independent types rather than a single
//! inheritance tree, so this module exposes small adapters instead of adding a
//! method to every struct.

use cnc_asset::AssetRegistry;
use cnc_core::{ClassId, Vec2};
use cnc_renderer::Renderer;

/// Draw one entity as a placeholder quad through any renderer implementation.
pub fn render_entity(
    renderer: &mut dyn Renderer,
    class_id: ClassId,
    position: Vec2,
    size: f32,
    color: [f32; 4],
) {
    renderer.draw_rect(position.x.to_f32(), position.y.to_f32(), size, size, color);
}

/// Resolve an entity's sprite bytes from the asset registry.
pub fn entity_asset(assets: &AssetRegistry, class_id: ClassId) -> Option<Vec<u8>> {
    assets.get(&format!("units/{:?}.shp", class_id)).cloned()
}
