//! Map system for Tiberian Dawn

use cnc_asset::tileset_pack::TilesetPack;
use cnc_core::{Fixed16_16, Rect, Vec2, Vec2i};
use cnc_renderer::Renderer;
use cnc_renderer::Sprite;
use cnc_renderer::TILESET_TEXTURE_ID;
use serde::{Deserialize, Serialize};

use crate::td_rules::{tiberium_amount_for_stage, tiberium_stage, Theater, TIBERIUM_STAGES};

/// texture_id passed to tileset sprite draws so backends sample the tileset
/// atlas slot, separate from the procedural unit atlas slot 0.
const TILESET_TEXTURE: u32 = TILESET_TEXTURE_ID;

/// Pixel size of one map cell when rendered.
pub const TILE_SIZE: f32 = 24.0;

/// Cell grid for the map
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellGrid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    pub terrain_type: u8,
    pub overlay_type: u8,
    pub tiberium_amount: u8,
    pub is_passable: bool,
    pub is_buildable: bool,
    pub has_tiberium: bool,
    pub tiberium_type: u8,
}

impl CellGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = Vec::with_capacity((width * height) as usize);
        for _ in 0..(width * height) {
            cells.push(Cell {
                terrain_type: 0,
                overlay_type: 0,
                tiberium_amount: 0,
                is_passable: true,
                is_buildable: true,
                has_tiberium: false,
                tiberium_type: 0,
            });
        }
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, pos: Vec2i) -> Option<&Cell> {
        if pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32 {
            let idx = (pos.y * self.width as i32 + pos.x) as usize;
            Some(&self.cells[idx])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, pos: Vec2i) -> Option<&mut Cell> {
        if pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32 {
            let idx = (pos.y * self.width as i32 + pos.x) as usize;
            Some(&mut self.cells[idx])
        } else {
            None
        }
    }

    pub fn set_terrain(&mut self, pos: Vec2i, terrain: u8) {
        if let Some(cell) = self.get_mut(pos) {
            cell.terrain_type = terrain;
        }
    }

    pub fn set_overlay(&mut self, pos: Vec2i, overlay: u8) {
        if let Some(cell) = self.get_mut(pos) {
            cell.overlay_type = overlay;
        }
    }

    pub fn set_tiberium(&mut self, pos: Vec2i, amount: u8, tiberium_type: u8) {
        if let Some(cell) = self.get_mut(pos) {
            cell.tiberium_amount = amount;
            cell.tiberium_type = tiberium_type;
            cell.has_tiberium = amount > 0;
        }
    }

    /// CELL.CPP CellClass::Reduce_Tiberium: lower the overlay level, returning
    /// how many levels were actually removed.
    pub fn reduce_tiberium(&mut self, pos: Vec2i, levels: u8) -> u8 {
        let Some(cell) = self.get_mut(pos) else {
            return 0;
        };
        if !cell.has_tiberium || levels == 0 {
            return 0;
        }
        let available = tiberium_stage(cell.tiberium_amount);
        let reduced = levels.min(available);
        let remaining = available - reduced;
        if remaining == 0 {
            cell.has_tiberium = false;
            cell.tiberium_amount = 0;
        } else {
            cell.tiberium_amount = tiberium_amount_for_stage(remaining);
        }
        reduced
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    /// Toggle movement passability, mirroring CellClass::Is_Clear_To_Move input.
    pub fn set_passable(&mut self, pos: Vec2i, passable: bool) {
        if let Some(cell) = self.get_mut(pos) {
            cell.is_passable = passable;
        }
    }

    /// MAP.CPP MapClass::Logic: one growth/spread cycle.
    ///
    /// The original scans 30 cells per tick and grows/spreads a couple of
    /// random cells per full map pass. Rust uses a deterministic bounded pass
    /// (growth quota of existing cells, up to four spreads) so tests and
    /// replays are stable.
    pub fn tiberium_cycle(&mut self) {
        let mut growth_targets = Vec::new();
        for (index, cell) in self.cells.iter().enumerate() {
            if cell.has_tiberium && tiberium_stage(cell.tiberium_amount) < TIBERIUM_STAGES - 1 {
                growth_targets.push(index);
            }
        }

        let growth_quota = (growth_targets.len() / 8).max(1);
        for &index in growth_targets.iter().take(growth_quota) {
            let cell = &mut self.cells[index];
            let stage = tiberium_stage(cell.tiberium_amount);
            cell.tiberium_amount = tiberium_amount_for_stage(stage + 1);
        }

        let width = self.width as i32;
        let offsets = [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ];

        let mut spreads = 0;
        for index in 0..self.cells.len() {
            if spreads >= 4 {
                break;
            }
            let cell = self.cells[index];
            if !cell.has_tiberium || tiberium_stage(cell.tiberium_amount) <= 6 {
                continue;
            }

            let x = (index as i32) % width;
            let y = (index as i32) / width;
            for (dx, dy) in offsets {
                let nx = x + dx;
                let ny = y + dy;
                if !self.in_bounds(nx, ny) {
                    continue;
                }
                let neighbour_index = (ny * width + nx) as usize;
                let neighbour = &mut self.cells[neighbour_index];
                if neighbour.has_tiberium || !neighbour.is_passable {
                    continue;
                }
                neighbour.has_tiberium = true;
                neighbour.tiberium_amount = tiberium_amount_for_stage(1);
                neighbour.tiberium_type = cell.tiberium_type;
                spreads += 1;
                break;
            }
        }
    }

    /// Backwards-compatible alias for one growth cycle.
    pub fn grow_tiberium(&mut self) {
        self.tiberium_cycle();
    }
}

/// Map structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub grid: CellGrid,
    pub player_starts: Vec<Vec2i>,
    pub trigger_zones: Vec<TriggerZone>,
    pub theater: Theater,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerZone {
    pub id: u32,
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Map {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            name,
            width,
            height,
            grid: CellGrid::new(width, height),
            player_starts: Vec::new(),
            trigger_zones: Vec::new(),
            theater: Theater::Temperate,
        }
    }

    /// Draw the whole map grid through any cnc_renderer::Renderer.
    ///
    /// Tiles are drawn as flat quads until terrain SHP art is wired in.
    pub fn render(&self, renderer: &mut dyn Renderer) {
        self.render_with_tiles(renderer, None);
    }

    /// Draw the map with RTS tileset sprites when a loaded pack is available
    /// and its atlas is bound on the tileset texture slot; fall back to the
    /// original flat quads otherwise.
    ///
    /// Mapping:
    /// - Water cells (`terrain_type >= TERRAIN_WATER_BASE`) pick a
    ///   `water_edges` family from the four-neighbour land bitmask
    ///   ([`Map::land_mask`] + [`water_edges_layout`]).
    /// - Land cells render the `terrain`/`rock` (terrain_type 0) or
    ///   `terrain`/`sand` families as before; tiberium cells swap their base
    ///   ground to the `tiberium_ground` family under the FX layer.
    /// - Tiberium cells draw a translucent `resources_fx` sprite over the
    ///   base tile, keyed by growth stage ([`tiberium_fx_tile`]); the legacy
    ///   green tint only remains as the no-pack fallback.
    ///
    /// All cells share the deterministic column variant `(x*7 + y*13) % 16`.
    pub fn render_with_tiles(&self, renderer: &mut dyn Renderer, tiles: Option<&TilesetPack>) {
        // Lazily bind the pack atlas on the tileset texture slot once per
        // renderer; flat quads keep working when no pack loads or the upload
        // fails.
        let mut tiles = tiles;
        if let Some(pack) = tiles.filter(|_| !renderer.texture_ready(TILESET_TEXTURE)) {
            match renderer.set_atlas_texture(
                TILESET_TEXTURE,
                pack.width(),
                pack.height(),
                pack.rgba(),
            ) {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("tileset atlas upload failed: {error}");
                    tiles = None;
                }
            }
        }
        let tiles = tiles.filter(|_| renderer.texture_ready(TILESET_TEXTURE));

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.grid.cells[(y * self.width + x) as usize];
                let water = is_water_terrain(cell.terrain_type);
                let column = ((x * 7 + y * 13) % 16) as u8;
                let tile_rect = tiles.and_then(|pack| {
                    let (category, family, fallback_row) = if water {
                        let (family, row) = water_edges_layout(self.land_mask(x, y));
                        ("water_edges", family, row)
                    } else if cell.has_tiberium {
                        // Tiberium ground underlayer visible beneath the FX
                        // crystal sprite.
                        ("terrain", "tiberium_ground", 5u8)
                    } else if cell.terrain_type == 0 {
                        ("terrain", "rock", 4u8)
                    } else {
                        ("terrain", "sand", 2u8)
                    };
                    let row = pack.family_row(category, family).unwrap_or(fallback_row);
                    pack.tile_px_rect(category, row, column)
                });
                if let Some((uv_x, uv_y, uv_w, uv_h)) = tile_rect {
                    let mut sprite = Sprite::new(
                        TILESET_TEXTURE,
                        Vec2::new(
                            Fixed16_16::from_f32(x as f32 * TILE_SIZE),
                            Fixed16_16::from_f32(y as f32 * TILE_SIZE),
                        ),
                        Vec2::new(
                            Fixed16_16::from_f32(TILE_SIZE + 0.5),
                            Fixed16_16::from_f32(TILE_SIZE + 0.5),
                        ),
                    );
                    sprite.uv_rect = Rect::new(uv_x as i32, uv_y as i32, uv_w as i32, uv_h as i32);
                    renderer.draw_sprite(&sprite);
                } else {
                    let color = if cell.has_tiberium {
                        [0.0, 0.6, 0.0, 1.0]
                    } else if water {
                        [0.13, 0.32, 0.55, 1.0]
                    } else if cell.terrain_type == 0 {
                        [0.30, 0.30, 0.30, 1.0]
                    } else {
                        [0.45, 0.40, 0.25, 1.0]
                    };
                    renderer.draw_rect(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        color,
                    );
                    // Tiberium is part of the flat-quad colour in the legacy
                    // layer; nothing further to draw for this cell.
                    continue;
                }
                if cell.has_tiberium {
                    // `resources_fx` is an RGBA sheet, so the sprite floats
                    // over the base tile; keyed by growth stage, with the
                    // legacy green tint only as the no-pack fallback.
                    let fx_rect = tiles.and_then(|pack| {
                        let (family, fallback_row) = tiberium_fx_tile(
                            cell.tiberium_type,
                            tiberium_stage(cell.tiberium_amount),
                        );
                        let row = pack
                            .family_row("resources_fx", family)
                            .unwrap_or(fallback_row);
                        pack.tile_px_rect("resources_fx", row, column)
                    });
                    if let Some((uv_x, uv_y, uv_w, uv_h)) = fx_rect {
                        let mut sprite = Sprite::new(
                            TILESET_TEXTURE,
                            Vec2::new(
                                Fixed16_16::from_f32(x as f32 * TILE_SIZE),
                                Fixed16_16::from_f32(y as f32 * TILE_SIZE),
                            ),
                            Vec2::new(
                                Fixed16_16::from_f32(TILE_SIZE + 0.5),
                                Fixed16_16::from_f32(TILE_SIZE + 0.5),
                            ),
                        );
                        sprite.uv_rect =
                            Rect::new(uv_x as i32, uv_y as i32, uv_w as i32, uv_h as i32);
                        renderer.draw_sprite(&sprite);
                    } else {
                        let color = [0.0, 0.6, 0.0, 0.45];
                        renderer.draw_rect(
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            TILE_SIZE,
                            TILE_SIZE,
                            color,
                        );
                    }
                }
            }
        }
    }

    /// Neighbour land bitmask for water-edge selection at a water cell:
    /// north bit 1, east bit 2, south bit 4, west bit 8
    /// (consumed by [`water_edges_layout`]).
    ///
    /// Off-map neighbours count as water so open sea continues across the
    /// map border instead of ringing it with shoreline.
    pub fn land_mask(&self, x: u32, y: u32) -> u8 {
        let width = self.width as usize;
        let height = self.height as usize;
        let (cx, cy) = (x as usize, y as usize);
        let is_land =
            |nx: usize, ny: usize| !is_water_terrain(self.grid.cells[ny * width + nx].terrain_type);
        let mut mask = 0u8;
        if cy > 0 && is_land(cx, cy - 1) {
            mask |= 1;
        }
        if cx + 1 < width && is_land(cx + 1, cy) {
            mask |= 2;
        }
        if cy + 1 < height && is_land(cx, cy + 1) {
            mask |= 4;
        }
        if cx > 0 && is_land(cx - 1, cy) {
            mask |= 8;
        }
        mask
    }
}

/// Reserved `terrain_type` encoding for water cells.
///
/// Savegames serialise `Cell` verbatim (bincode) and the version-1 format
/// must stay byte-compatible, so water reuses the existing
/// `terrain_type: u8` instead of adding a struct field: values `0xa0..=0xff`
/// mean water. `0..=0x9f` remain available to existing and future terrain
/// kinds (0 rock, 1 sand today), so legacy saves and maps cannot collide.
pub const TERRAIN_WATER_BASE: u8 = 0xa0;

/// True when a `terrain_type` value encodes a water cell.
pub fn is_water_terrain(terrain_type: u8) -> bool {
    terrain_type >= TERRAIN_WATER_BASE
}

/// `resources_fx` family (plus fallback manifest row) for a tiberium cell.
///
/// Growth stages 0–1 render the sprout family, stage 2 the cluster family,
/// and stage 3+ the bloom family; blue tiberium (`tiberium_type == 1`) always
/// renders the dedicated blue family regardless of stage.
pub fn tiberium_fx_tile(tiberium_type: u8, stage: u8) -> (&'static str, u8) {
    if tiberium_type == 1 {
        ("blue_tiberium", 2)
    } else if stage <= 1 {
        ("tiberium_sprout", 0)
    } else if stage == 2 {
        ("tiberium_cluster", 1)
    } else {
        ("tiberium_bloom", 14)
    }
}

/// `water_edges` family (plus fallback manifest row) for a water cell's
/// neighbour land bitmask — north bit 1, east bit 2, south bit 4, west bit 8
/// (produced by [`Map::land_mask`]).
///
/// The pack's shore art keeps its land half against the tile edge that
/// borders the land cell; family names name where the *water* half sits in
/// the art, so land to the north selects `shore_south` and corners follow the
/// same rule (`shore_nw` art = water corner northwest = land to S+E).
/// Land on two opposite sides selects the river-flow families; three or four
/// land neighbours fall back to the shallow-water family.
pub fn water_edges_layout(land_mask: u8) -> (&'static str, u8) {
    match land_mask {
        0 => ("deep_water", 0),
        1 => ("shore_south", 4),
        2 => ("shore_west", 5),
        4 => ("shore_north", 2),
        8 => ("shore_east", 3),
        3 => ("shore_sw", 8),
        6 => ("shore_nw", 6),
        9 => ("shore_se", 9),
        12 => ("shore_ne", 7),
        5 => ("river_flow_e_w", 11),
        10 => ("river_flow_n_s", 10),
        _ => ("shallow_water", 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiberium_stage_maps_to_fx_family() {
        assert_eq!(tiberium_fx_tile(0, 0), ("tiberium_sprout", 0));
        assert_eq!(tiberium_fx_tile(0, 1), ("tiberium_sprout", 0));
        assert_eq!(tiberium_fx_tile(0, 2), ("tiberium_cluster", 1));
        assert_eq!(tiberium_fx_tile(0, 3), ("tiberium_bloom", 14));
        assert_eq!(
            tiberium_fx_tile(0, TIBERIUM_STAGES - 1),
            ("tiberium_bloom", 14)
        );
        // Blue tiberium (type 1) renders the dedicated blue family at any stage.
        assert_eq!(tiberium_fx_tile(1, 0), ("blue_tiberium", 2));
        assert_eq!(tiberium_fx_tile(1, 5), ("blue_tiberium", 2));
    }

    #[test]
    fn water_shore_family_follows_land_neighbor_mask() {
        // Bits: N=1, E=2, S=4, W=8. Exact table match per mask family.
        assert_eq!(water_edges_layout(0), ("deep_water", 0));
        assert_eq!(water_edges_layout(1), ("shore_south", 4));
        assert_eq!(water_edges_layout(2), ("shore_west", 5));
        assert_eq!(water_edges_layout(4), ("shore_north", 2));
        assert_eq!(water_edges_layout(8), ("shore_east", 3));
        // Adjacent land pairs map to the corner whose art keeps the land half
        // hugging both land edges.
        assert_eq!(water_edges_layout(3), ("shore_sw", 8)); // land N+E
        assert_eq!(water_edges_layout(6), ("shore_nw", 6)); // land S+E
        assert_eq!(water_edges_layout(9), ("shore_se", 9)); // land N+W
        assert_eq!(water_edges_layout(12), ("shore_ne", 7)); // land S+W
                                                             // Opposite land pairs form a river channel along the free axis.
        assert_eq!(water_edges_layout(5), ("river_flow_e_w", 11)); // land N+S
        assert_eq!(water_edges_layout(10), ("river_flow_n_s", 10)); // land E+W
                                                                    // Three or four land neighbours fall back to shallow water.
        for mask in [7u8, 11, 13, 14, 15] {
            assert_eq!(
                water_edges_layout(mask),
                ("shallow_water", 1),
                "mask {mask}"
            );
        }
    }

    #[test]
    fn water_terrain_encoding_has_no_collision() {
        // Existing terrain values (and any future sub-kind) below 0xa0 never
        // read as water.
        for terrain in [0u8, 1, 2, 15, 0x7f, 0x9f] {
            assert!(!is_water_terrain(terrain), "terrain {terrain:#x}");
        }
        // The reserved 0xa0..=0xff band always reads as water.
        for terrain in [0xa0u8, 0xa1, 0xf0, 0xff] {
            assert!(is_water_terrain(terrain), "terrain {terrain:#x}");
        }
    }

    #[test]
    fn land_mask_counts_cardinal_land_only() {
        let mut map = Map::new("water-mask".to_string(), 3, 3);
        map.grid.set_terrain(Vec2i::new(1, 1), TERRAIN_WATER_BASE);
        // Land to the north and east; west/south are still land-less (default
        // terrain 0 counts as land, so place water there too).
        map.grid.set_terrain(Vec2i::new(1, 0), 1);
        map.grid.set_terrain(Vec2i::new(2, 1), 1);
        map.grid.set_terrain(Vec2i::new(0, 1), TERRAIN_WATER_BASE);
        map.grid.set_terrain(Vec2i::new(1, 2), TERRAIN_WATER_BASE);
        // Off-map neighbours count as water, never land.
        assert_eq!(map.land_mask(1, 1), 1 | 2);

        let mut sea = Map::new("sea".to_string(), 4, 4);
        for cell in &mut sea.grid.cells {
            cell.terrain_type = TERRAIN_WATER_BASE;
        }
        assert_eq!(sea.land_mask(0, 0), 0); // border cell: OOB = water
        assert_eq!(sea.land_mask(1, 2), 0); // interior cell: water around
    }

    #[test]
    fn water_terrain_survives_serialization() {
        // Water encoding rides on the existing `terrain_type: u8`, so savegame
        // cells persist it without any format change.
        let mut grid = CellGrid::new(2, 1);
        grid.set_terrain(Vec2i::new(1, 0), TERRAIN_WATER_BASE + 5);
        let bytes = bincode::serialize(&grid).unwrap();
        let back: CellGrid = bincode::deserialize(&bytes).unwrap();
        let cell = back.get(Vec2i::new(1, 0)).unwrap();
        assert!(is_water_terrain(cell.terrain_type));
    }
}
