//! Editor view state: active tool, history, selection and grid rendering.
//!
//! The UI is renderer-agnostic: [`EditorUI::render`] draws through any
//! `cnc_renderer::Renderer`. Mouse handlers take a map cell and route the
//! tool's command through [`crate::history::History`], so the model is only
//! ever changed by undoable commands.

use std::sync::Arc;

use crate::history::History;
use crate::map::{MapModel, TILE_SIZE};
use crate::pack_tileset::PackTileset;
use crate::tools::Tool;
use cnc_core::{Fixed16_16, Vec2, Vec2i};
use cnc_renderer::{Renderer, Sprite, TILESET_TEXTURE_ID};

/// Colour for a terrain id (editor-only placeholder palette).
pub fn terrain_color(value: u8) -> [f32; 4] {
    const PALETTE: [[f32; 4]; 6] = [
        [0.28, 0.32, 0.22, 1.0],
        [0.42, 0.36, 0.22, 1.0],
        [0.30, 0.42, 0.30, 1.0],
        [0.36, 0.38, 0.44, 1.0],
        [0.22, 0.30, 0.46, 1.0],
        [0.52, 0.48, 0.30, 1.0],
    ];
    PALETTE[(value as usize) % PALETTE.len()]
}

/// Colour for a placed object: buildings warm, units cool.
pub fn object_color(model: &MapModel, id: u32) -> [f32; 4] {
    match model.objects.get(&id) {
        Some(object) if object.class_id.is_building() => [0.95, 0.65, 0.25, 1.0],
        _ => [0.35, 0.80, 1.00, 1.0],
    }
}

/// One unit-sized textured quad sampled from the tileset atlas.
fn draw_uv_sprite(renderer: &mut dyn Renderer, sx: f32, sy: f32, size: f32, rect: cnc_core::Rect) {
    let mut sprite = Sprite::new(
        TILESET_TEXTURE_ID,
        Vec2::new(Fixed16_16::from_f32(sx), Fixed16_16::from_f32(sy)),
        Vec2::new(Fixed16_16::from_f32(size), Fixed16_16::from_f32(size)),
    );
    sprite.uv_rect = rect;
    renderer.draw_sprite(&sprite);
}

/// Interactive editor state.
pub struct EditorUI {
    pub active_tool: Option<Box<dyn Tool>>,
    pub show_grid: bool,
    pub zoom: f32,
    pub history: History,
    pub cursor: Vec2i,
    pub selected_object: Option<u32>,
    /// Multi-selection produced by the marquee; the primary object above is
    /// the first entry so single-object tools keep working unchanged.
    pub selected_objects: Vec<u32>,
    pub status: String,
    /// Optional cnc32 pack tileset: when set, cells draw the same tile layers
    /// as the main editor (shores, blob transitions, resource sprites); when
    /// absent, this view keeps its flat-coloured fallback quads.
    pub tileset: Option<Arc<PackTileset>>,
}

impl Default for EditorUI {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorUI {
    pub fn new() -> Self {
        Self {
            active_tool: None,
            show_grid: true,
            zoom: 1.0,
            history: History::new(256),
            cursor: Vec2i::new(0, 0),
            selected_object: None,
            selected_objects: Vec::new(),
            status: String::new(),
            tileset: None,
        }
    }

    pub fn set_tool(&mut self, tool: Box<dyn Tool>) {
        self.status = format!("tool: {}", tool.name());
        self.active_tool = Some(tool);
    }

    pub fn has_tool(&self) -> bool {
        self.active_tool.is_some()
    }

    /// Name of the active tool, if any (e.g. "Select" or "Trigger").
    pub fn tool_name(&self) -> Option<&str> {
        self.active_tool.as_ref().map(|tool| tool.name())
    }

    /// Route a click to the active tool; returns whether the model changed.
    pub fn click(&mut self, model: &mut MapModel, cell: Vec2i) -> bool {
        self.cursor = cell;
        let Some(tool) = self.active_tool.as_mut() else {
            return false;
        };
        let Some(command) = tool.click(model, cell) else {
            self.status = format!("no-op at ({}, {})", cell.x, cell.y);
            return false;
        };
        self.status = command.label();
        self.history.execute(model, command);
        if let Some(object) = model.object_at(cell) {
            self.selected_object = Some(object.id);
        }
        true
    }

    /// Route a drag to the active tool; returns whether the model changed.
    pub fn drag(&mut self, model: &mut MapModel, from: Vec2i, to: Vec2i) -> bool {
        self.cursor = to;
        let Some(tool) = self.active_tool.as_mut() else {
            return false;
        };
        let Some(command) = tool.drag(model, from, to) else {
            return false;
        };
        self.status = command.label();
        self.history.execute(model, command);
        true
    }

    pub fn undo(&mut self, model: &mut MapModel) -> bool {
        let Some(label) = self.history.undo(model) else {
            self.status = "nothing to undo".to_string();
            return false;
        };
        self.status = format!("undo: {label}");
        true
    }

    pub fn redo(&mut self, model: &mut MapModel) -> bool {
        let Some(label) = self.history.redo(model) else {
            self.status = "nothing to redo".to_string();
            return false;
        };
        self.status = format!("redo: {label}");
        true
    }

    /// Delete the object under the cursor as an undoable command.
    pub fn delete_at(&mut self, model: &mut MapModel, cell: Vec2i) -> bool {
        let Some(object) = model.object_at(cell) else {
            return false;
        };
        let id = object.id;
        let command = crate::history::EraseObject::new(id);
        self.history.execute(model, Box::new(command));
        self.status = format!("deleted object {id}");
        self.selected_objects.retain(|selected| *selected != id);
        if self.selected_object == Some(id) {
            self.selected_object = self.selected_objects.first().copied();
        }
        true
    }

    /// Draw terrain, overlay markers, objects, grid and status line.
    pub fn render(&mut self, renderer: &mut dyn Renderer, model: &MapModel) {
        let tile = TILE_SIZE * self.zoom.max(0.1);

        // Tileset mode: upload the pack atlas once, then draw the same tile
        // layers (shores, cliffs, resource sprites) as the main editor.
        let mut tiled = self.tileset.is_some();
        if let Some(pack) = self.tileset.as_ref() {
            if !renderer.texture_ready(TILESET_TEXTURE_ID)
                && renderer
                    .set_atlas_texture(TILESET_TEXTURE_ID, pack.width(), pack.height(), pack.rgba())
                    .is_err()
            {
                tiled = false;
            }
        }
        for y in 0..model.height {
            for x in 0..model.width {
                let index = (y * model.width + x) as usize;
                let terrain = model.terrain[index];
                let overlay = model.overlay[index];
                let (sx, sy) = (x as f32 * tile, y as f32 * tile);

                let mut drew_sprite = false;
                let mut drew_overlay_sprite = false;
                if tiled {
                    if let Some(pack) = self.tileset.as_ref() {
                        // `pack` is an Arc<PackTileset>; deref for the borrow.
                        let pack = pack.as_ref();
                        if let Some(cell) = pack.cell_draw(model, x as i32, y as i32) {
                            if let Some(id) =
                                cell.base.as_deref().and_then(|id| pack.sprite_rect(id))
                            {
                                draw_uv_sprite(renderer, sx, sy, tile, id);
                                drew_sprite = true;
                            }
                            for layer in &cell.layers {
                                if let Some(rect) = pack.sprite_rect(layer) {
                                    draw_uv_sprite(renderer, sx, sy, tile, rect);
                                    drew_sprite = true;
                                    drew_overlay_sprite = true;
                                }
                            }
                        }
                    }
                }
                // Flat fallbacks (also per-layer when pack art is missing).
                match (drew_sprite, drew_overlay_sprite) {
                    (true, true) => {}
                    (true, false) => {
                        renderer.draw_rect(
                            sx + tile * 0.3,
                            sy + tile * 0.3,
                            tile * 0.4,
                            tile * 0.4,
                            [0.70, 0.55, 0.25, 1.0],
                        );
                    }
                    (false, _) => {
                        let mut color = terrain_color(terrain);
                        if overlay != 0 {
                            color = [0.70, 0.55, 0.25, 1.0];
                        }
                        renderer.draw_rect(sx, sy, tile, tile, color);
                    }
                }
            }
        }

        let mut objects: Vec<&crate::map::MapObject> = model.objects.values().collect();
        objects.sort_by_key(|object| object.id);
        for object in objects {
            let color = if self.selected_object == Some(object.id)
                || self.selected_objects.contains(&object.id)
            {
                [1.0, 1.0, 1.0, 1.0]
            } else if object.class_id.is_building() {
                [0.95, 0.65, 0.25, 1.0]
            } else {
                [0.35, 0.80, 1.00, 1.00]
            };
            let inset = tile * 0.15;
            renderer.draw_rect(
                object.position.x as f32 * tile + inset,
                object.position.y as f32 * tile + inset,
                tile - inset * 2.0,
                tile - inset * 2.0,
                color,
            );
        }

        if self.show_grid && tile >= 4.0 {
            let grid = [0.0, 0.0, 0.0, 0.25];
            for x in 0..=model.width {
                renderer.draw_rect(x as f32 * tile, 0.0, 1.0, model.height as f32 * tile, grid);
            }
            for y in 0..=model.height {
                renderer.draw_rect(0.0, y as f32 * tile, model.width as f32 * tile, 1.0, grid);
            }
        }

        let cursor_color = [1.0, 0.30, 0.30, 0.9];
        renderer.draw_rect(
            self.cursor.x as f32 * tile,
            self.cursor.y as f32 * tile,
            tile,
            tile,
            cursor_color,
        );

        if !self.status.is_empty() {
            renderer.draw_text(&self.status, 8.0, 8.0, 12.0, [0.95, 0.95, 0.95, 1.0]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{MapModel, TILE_SIZE};
    use crate::tools::TerrainTool;
    use cnc_core::Palette;
    use cnc_renderer::{Camera, Renderer, Sprite};

    struct CountingRenderer {
        rects: usize,
        texts: usize,
    }

    impl Renderer for CountingRenderer {
        fn begin_frame(&mut self) {}
        fn end_frame(&mut self) -> anyhow::Result<()> {
            Ok(())
        }
        fn resize(&mut self, _width: u32, _height: u32) {}
        fn set_palette(&mut self, _palette: &Palette) {}
        fn draw_sprite(&mut self, _sprite: &Sprite) {}
        fn draw_rect(&mut self, _x: f32, _y: f32, _w: f32, _h: f32, _color: [f32; 4]) {
            self.rects += 1;
        }
        fn draw_text(&mut self, _text: &str, _x: f32, _y: f32, _size: f32, _color: [f32; 4]) {
            self.texts += 1;
        }
        fn set_camera(&mut self, _camera: &Camera) {}
    }

    #[test]
    fn clicks_are_undoable_and_selection_tracks_objects() {
        let mut model = MapModel::new(4, 4);
        let mut ui = EditorUI::new();
        ui.set_tool(Box::new(TerrainTool::new(2)));
        assert!(ui.click(&mut model, Vec2i::new(1, 1)));
        assert_eq!(model.terrain[5], 2);
        assert!(ui.undo(&mut model));
        assert_eq!(model.terrain[5], 0);
        assert!(ui.redo(&mut model));
        assert_eq!(model.terrain[5], 2);
        assert!(ui.history.can_undo());
    }

    #[test]
    fn delete_at_is_undoable() {
        let mut model = MapModel::new(4, 4);
        let id = model.insert_object(crate::map::MapObject {
            id: 0,
            class_id: cnc_core::ClassId::VehicleE1,
            position: Vec2i::new(2, 2),
            owner: None,
            ..Default::default()
        });
        let mut ui = EditorUI::new();
        assert!(ui.delete_at(&mut model, Vec2i::new(2, 2)));
        assert!(model.objects.is_empty());
        ui.undo(&mut model);
        assert_eq!(model.objects[&id].position, Vec2i::new(2, 2));
    }

    #[test]
    fn render_draws_terrain_objects_grid_and_status() {
        let mut model = MapModel::new(2, 2);
        model.insert_object(crate::map::MapObject {
            id: 0,
            class_id: cnc_core::ClassId::ConstructionYard,
            position: Vec2i::new(1, 1),
            owner: None,
            ..Default::default()
        });
        let mut ui = EditorUI::new();
        ui.status = "ready".to_string();
        let mut renderer = CountingRenderer { rects: 0, texts: 0 };
        ui.render(&mut renderer, &model);
        assert!(renderer.rects >= 4 + 1 + 6);
        assert_eq!(renderer.texts, 1);
        let _ = TILE_SIZE;
    }
}
