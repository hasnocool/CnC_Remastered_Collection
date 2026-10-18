//! Map system for Tiberian Dawn

use cnc_core::{Vec2i, Fixed16_16};
use cnc_renderer::Renderer;

use crate::td_rules::{
    tiberium_amount_for_stage, tiberium_stage, TIBERIUM_STAGES,
};

/// Pixel size of one map cell when rendered.
pub const TILE_SIZE: f32 = 24.0;

/// Cell grid for the map
#[derive(Debug, Clone)]
pub struct CellGrid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone, Copy)]
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
        Self { width, height, cells }
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
            if cell.has_tiberium
                && tiberium_stage(cell.tiberium_amount) < TIBERIUM_STAGES - 1
            {
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
#[derive(Debug, Clone)]
pub struct Map {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub grid: CellGrid,
    pub player_starts: Vec<Vec2i>,
    pub trigger_zones: Vec<TriggerZone>,
}

#[derive(Debug, Clone)]
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
        }
    }

    /// Draw the whole map grid through any cnc_renderer::Renderer.
    ///
    /// Tiles are drawn as flat quads until terrain SHP art is wired in.
    pub fn render(&self, renderer: &mut dyn Renderer) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.grid.cells[(y * self.width + x) as usize];
                let color = if cell.has_tiberium {
                    [0.0, 0.6, 0.0, 1.0]
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
            }
        }
    }
}