//! Renderer + asset-system bridge for Red Alert entities.
//!
//! Red Alert's entity structs are independent types rather than a single
//! inheritance tree, so this module exposes small adapters instead of adding a
//! method to every struct.

use cnc_asset::AssetRegistry;
use cnc_core::{ClassId, Vec2};
use cnc_renderer::Renderer;

/// Draw one entity as a placeholder quad through any renderer implementation.
/// Pack unit kind for an RA class id. The pack ships 7 kinds; everything else
/// keeps the flat quad. RA ids follow `class_id.rs`: E1..E11 infantry,
/// VehicleE1..E9 vehicles, AircraftE1/E7 aircraft.
pub fn pack_unit_kind(class_id: ClassId) -> Option<&'static str> {
    use cnc_core::ClassId;
    Some(match class_id {
        ClassId::InfantryE1 | ClassId::InfantryE2 | ClassId::InfantryE3 => "infantry",
        ClassId::InfantryE4 | ClassId::InfantryE7 => "rocket_soldier",
        ClassId::InfantryE6 => "engineer",
        ClassId::VehicleE1
        | ClassId::VehicleE2
        | ClassId::VehicleE3
        | ClassId::VehicleE4
        | ClassId::VehicleE7 => "tank",
        ClassId::VehicleE6 => "harvester",
        ClassId::VehicleE5 => "buggy",
        _ => return None,
    })
}

/// The pack's unit atlas, loaded once per process (None keeps flat quads).
static UNIT_ATLAS: std::sync::LazyLock<Option<cnc_asset::unit_atlas::UnitAtlas>> =
    std::sync::LazyLock::new(cnc_asset::unit_atlas::UnitAtlas::from_fallback);

pub fn render_entity(
    renderer: &mut dyn Renderer,
    class_id: ClassId,
    position: Vec2,
    size: f32,
    color: [f32; 4],
) {
    // Pack unit art first: one animated sprite per entity. The atlas lives in
    // texture slot 0 (the caller uploads it once per renderer).
    if let Some(kind) = pack_unit_kind(class_id) {
        if let Some(frame) = UNIT_ATLAS
            .as_ref()
            .and_then(|pack| pack.first_frame(kind, cnc_asset::unit_atlas::UnitAnim::Idle, 0))
        {
            if renderer.texture_ready(UNIT_TEXTURE_ID) {
                let mut sprite = cnc_renderer::Sprite::new(
                    UNIT_TEXTURE_ID,
                    Vec2::new(position.x, position.y),
                    cnc_core::Vec2::new(
                        cnc_core::Fixed16_16::from_f32(size),
                        cnc_core::Fixed16_16::from_f32(size),
                    ),
                );
                sprite.uv_rect = frame.rect;
                let tint = [
                    (color[3] * 255.0) as u8,
                    (color[0] * 255.0) as u8,
                    (color[1] * 255.0) as u8,
                    (color[2] * 255.0) as u8,
                ];
                sprite.color = cnc_core::ARGB8888::new(tint[0], tint[1], tint[2], tint[3]);
                renderer.draw_sprite(&sprite);
                return;
            }
        }
    }
    renderer.draw_rect(position.x.to_f32(), position.y.to_f32(), size, size, color);
}

/// Texture slot carrying the pack unit atlas (slot 0).
const UNIT_TEXTURE_ID: u32 = 0;

/// Resolve an entity's sprite bytes from the asset registry.
pub fn entity_asset(assets: &AssetRegistry, class_id: ClassId) -> Option<Vec<u8>> {
    assets.get(&format!("units/{:?}.shp", class_id)).cloned()
}
