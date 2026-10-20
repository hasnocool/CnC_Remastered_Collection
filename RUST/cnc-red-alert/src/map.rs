//! Red Alert cell map: terrain, ore fields and passability.
//!
//! RA keeps its own map type: naval units need water, ground units need land
//! and aircraft ignore the terrain entirely. The editor crate has a richer
//! scenario map; this one is the minimal simulation grid the game loop needs.

use cnc_core::Vec2i;

/// Pixels per map cell, matching the renderers' ICON_PIXEL_W.
pub const TILE_SIZE: f32 = 24.0;

/// Terrain classes that affect movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terrain {
    Grass,
    Water,
    Beach,
    Rock,
}

impl Terrain {
    pub fn is_water(self) -> bool {
        matches!(self, Terrain::Water)
    }

    pub fn is_land(self) -> bool {
        !self.is_water()
    }

    pub fn color(self) -> [f32; 4] {
        match self {
            Terrain::Grass => [0.22, 0.36, 0.20, 1.0],
            Terrain::Water => [0.10, 0.24, 0.48, 1.0],
            Terrain::Beach => [0.56, 0.50, 0.32, 1.0],
            Terrain::Rock => [0.38, 0.38, 0.40, 1.0],
        }
    }
}

/// A deterministic RA skirmish grid with a central river and two ore fields.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub terrain: Vec<Terrain>,
    pub ore: Vec<u32>,
}

impl Map {
    /// Empty grass map with no river or ore (used by scenario loading).
    pub fn blank(width: u32, height: u32) -> Self {
        let width = width.max(8);
        let height = height.max(8);
        Self {
            width,
            height,
            terrain: vec![Terrain::Grass; (width * height) as usize],
            ore: vec![0; (width * height) as usize],
        }
    }

    /// Build a map with a north/south river and ore near both starting bases.
    pub fn new(width: u32, height: u32) -> Self {
        let mut map = Self::blank(width, height);
        let width = map.width;
        let height = map.height;
        let river = width as i32 / 2;
        for y in 0..height as i32 {
            map.set_terrain(Vec2i::new(river, y), Terrain::Water);
            map.set_terrain(Vec2i::new(river + 1, y), Terrain::Water);
            map.set_terrain(Vec2i::new(river - 1, y), Terrain::Beach);
            map.set_terrain(Vec2i::new(river + 2, y), Terrain::Beach);
        }

        for &(cx, cy) in &[
            (4, 4),
            (6, 4),
            (4, 6),
            (width as i32 - 6, height as i32 - 6),
            (width as i32 - 4, height as i32 - 6),
            (width as i32 - 6, height as i32 - 4),
        ] {
            for dy in 0..2 {
                for dx in 0..2 {
                    map.add_ore(Vec2i::new(cx + dx, cy + dy), 400);
                }
            }
        }
        map
    }

    pub fn in_bounds(&self, cell: Vec2i) -> bool {
        cell.x >= 0 && cell.y >= 0 && (cell.x as u32) < self.width && (cell.y as u32) < self.height
    }

    pub fn index(&self, cell: Vec2i) -> Option<usize> {
        if self.in_bounds(cell) {
            Some(cell.y as usize * self.width as usize + cell.x as usize)
        } else {
            None
        }
    }

    pub fn terrain_at(&self, cell: Vec2i) -> Option<Terrain> {
        self.index(cell).map(|index| self.terrain[index])
    }

    pub fn set_terrain(&mut self, cell: Vec2i, terrain: Terrain) {
        if let Some(index) = self.index(cell) {
            self.terrain[index] = terrain;
        }
    }

    pub fn is_water(&self, cell: Vec2i) -> bool {
        self.terrain_at(cell).is_some_and(Terrain::is_water)
    }

    pub fn is_land(&self, cell: Vec2i) -> bool {
        self.terrain_at(cell).is_some_and(Terrain::is_land)
    }

    pub fn ore_at(&self, cell: Vec2i) -> u32 {
        self.index(cell).map_or(0, |index| self.ore[index])
    }

    pub fn add_ore(&mut self, cell: Vec2i, amount: u32) {
        if let Some(index) = self.index(cell) {
            self.ore[index] = self.ore[index].saturating_add(amount);
        }
    }

    /// Mine up to the requested amount; returns the amount actually taken.
    pub fn take_ore(&mut self, cell: Vec2i, wanted: u32) -> u32 {
        let Some(index) = self.index(cell) else {
            return 0;
        };
        let taken = self.ore[index].min(wanted);
        self.ore[index] -= taken;
        taken
    }

    pub fn ore_cells(&self) -> Vec<Vec2i> {
        let mut cells = Vec::new();
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let cell = Vec2i::new(x, y);
                if self.ore_at(cell) > 0 {
                    cells.push(cell);
                }
            }
        }
        cells
    }

    pub fn water_cells(&self) -> usize {
        self.terrain
            .iter()
            .filter(|terrain| terrain.is_water())
            .count()
    }

    pub fn summary(&self) -> String {
        format!(
            "{}x{} - {} water cells, {} ore cells",
            self.width,
            self.height,
            self.water_cells(),
            self.ore_cells().len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_map_has_a_river_and_ore_fields() {
        let map = Map::new(32, 24);
        assert_eq!(map.width, 32);
        assert!(map.water_cells() >= 24);
        let river = Vec2i::new(16, 4);
        assert!(map.is_water(river));
        assert!(map.is_land(Vec2i::new(4, 4)));
        assert!(map.ore_at(Vec2i::new(4, 4)) > 0);
        assert!(map.ore_at(Vec2i::new(26, 18)) > 0);
    }

    #[test]
    fn ore_can_be_mined_until_empty() {
        let mut map = Map::new(16, 16);
        let cell = Vec2i::new(4, 4);
        let available = map.ore_at(cell);
        assert!(available > 0);
        assert_eq!(map.take_ore(cell, 100), 100);
        assert_eq!(map.ore_at(cell), available - 100);
        assert_eq!(map.take_ore(cell, u32::MAX), available - 100);
        assert_eq!(map.ore_at(cell), 0);
        assert_eq!(map.take_ore(cell, 10), 0);
    }
}
