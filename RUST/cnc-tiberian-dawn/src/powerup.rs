//! Crate powerups — Rust port of the original `CRATE.CPP` (`CrateClass`).
//!
//! Crates spawn periodically on free land cells; a unit driving over one
//! collects a deterministic bonus (credits, a free infantry unit, or a timed
//! armor shield). All rolls derive from a seeded SplitMix64 stream that is
//! persisted with the game, so every peer and every save/replay reproduces
//! the same spawn and pickup sequence — the simulation itself stays
//! deterministic (the original used the shared deterministic RNG for the
//! same reason).

use serde::{Deserialize, Serialize};

use crate::map::{is_water_terrain, CellGrid, TILE_SIZE};
use crate::unit::Unit;
use cnc_core::{Vec2, Vec2i};

/// SplitMix64 bit-mix constant (shared with `cnc-procgen::prng`).
const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// Fixed seed so every peer, save and replay rolls the same stream.
const SEED: u64 = 0x5EED_C0DE;

/// Update calls between crate spawns (~30 s at 60 fps; `Game::update` runs
/// once per frame, matching the `tiberium_tick` cadence).
pub const SPAWN_INTERVAL_TICKS: u32 = 1_800;
/// Update calls before an uncollected crate disappears (~90 s).
pub const LIFETIME_TICKS: u32 = 5_400;
/// At most this many crates on the map at once.
pub const MAX_ACTIVE: usize = 2;
/// Credits granted by a money crate.
pub const CREDITS_BONUS: i32 = 1_000;
/// Seconds of halved incoming damage from an armor crate.
pub const SHIELD_SECONDS: f32 = 15.0;
/// Bounded scan for a free cell so a crowded map can never stall the tick.
const SCAN_LIMIT: i64 = 256;

/// SplitMix64 next output; advances the state.
fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(GAMMA);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// What a collected crate gives the driving unit's house.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerupKind {
    /// Flat credit grant to the owning house.
    Credits,
    /// A free infantry unit spawns on the crate cell.
    Unit,
    /// The collecting unit takes half damage for [`SHIELD_SECONDS`].
    Armor,
}

/// A crate sitting on the map waiting to be driven over.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Crate {
    pub id: u32,
    pub cell: Vec2i,
    pub kind: PowerupKind,
    pub spawned_tick: u32,
    pub expires_tick: u32,
}

/// A drive-over collection reported to the game for effect application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pickup {
    /// Index into `Game::units` of the collecting unit.
    pub unit_index: usize,
    pub owner: u8,
    pub kind: PowerupKind,
    pub cell: Vec2i,
}

/// Deterministic crate spawns, lifetimes and collections.
///
/// Persisted inside `GameSnapshot`; the internal tick advances once per
/// `Game::update` call, exactly like `tiberium_tick`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerupState {
    pub tick: u32,
    pub next_spawn_at: u32,
    pub next_crate_id: u32,
    pub rng_state: u64,
    pub crates: Vec<Crate>,
}

impl Default for PowerupState {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerupState {
    pub fn new() -> Self {
        Self {
            tick: 0,
            next_spawn_at: SPAWN_INTERVAL_TICKS,
            next_crate_id: 1,
            rng_state: SEED,
            crates: Vec::new(),
        }
    }

    /// Advance one update: expire stale crates, maybe spawn, then collect
    /// any crates a unit drove onto this update. Returns the pickups in
    /// unit order (deterministic).
    pub fn tick(&mut self, grid: &CellGrid, units: &[Unit]) -> Vec<Pickup> {
        self.tick = self.tick.wrapping_add(1);

        self.crates.retain(|crate_| self.tick < crate_.expires_tick);

        if self.tick >= self.next_spawn_at && self.crates.len() < MAX_ACTIVE {
            if let Some(cell) = self.find_spawn_cell(grid, units) {
                let kind = self.roll_kind();
                self.crates.push(Crate {
                    id: self.next_crate_id,
                    cell,
                    kind,
                    spawned_tick: self.tick,
                    expires_tick: self.tick + LIFETIME_TICKS,
                });
                self.next_crate_id = self.next_crate_id.wrapping_add(1);
                self.next_spawn_at = self.tick + SPAWN_INTERVAL_TICKS;
            }
        }

        let mut pickups = Vec::new();
        for (unit_index, unit) in units.iter().enumerate() {
            if !unit.is_alive() {
                continue;
            }
            let cell = cell_for(unit.position);
            if let Some(index) = self.crates.iter().position(|crate_| crate_.cell == cell) {
                let crate_ = self.crates.remove(index);
                pickups.push(Pickup {
                    unit_index,
                    owner: unit.owner,
                    kind: crate_.kind,
                    cell: crate_.cell,
                });
            }
        }
        pickups
    }

    /// Deterministic free-land scan: pick a pseudo-random start cell, then
    /// walk forward at most [`SCAN_LIMIT`] cells for a valid spot.
    fn find_spawn_cell(&mut self, grid: &CellGrid, units: &[Unit]) -> Option<Vec2i> {
        let width = i64::from(grid.width);
        let height = i64::from(grid.height);
        if width <= 0 || height <= 0 {
            return None;
        }
        let total = width * height;
        let start = (self.next_u64() % total as u64) as i64;
        for offset in 0..SCAN_LIMIT {
            let index = (start + offset) % total;
            let cell = Vec2i::new((index % width) as i32, (index / width) as i32);
            if !self.cell_free(grid, units, cell) {
                continue;
            }
            return Some(cell);
        }
        None
    }

    fn cell_free(&self, grid: &CellGrid, units: &[Unit], cell: Vec2i) -> bool {
        let Some(cell_data) = grid.get(cell) else {
            return false;
        };
        if !cell_data.is_passable
            || is_water_terrain(cell_data.terrain_type)
            || cell_data.tiberium_amount > 0
        {
            return false;
        }
        if self.crates.iter().any(|crate_| crate_.cell == cell) {
            return false;
        }
        !units
            .iter()
            .any(|unit| unit.is_alive() && cell_for(unit.position) == cell)
    }

    fn roll_kind(&mut self) -> PowerupKind {
        match self.next_u64() % 100 {
            0..=49 => PowerupKind::Credits,
            50..=84 => PowerupKind::Unit,
            _ => PowerupKind::Armor,
        }
    }

    fn next_u64(&mut self) -> u64 {
        splitmix(&mut self.rng_state)
    }
}

/// World position to the containing cell (mirrors `Game::world_cell`).
fn cell_for(position: Vec2) -> Vec2i {
    Vec2i::new(
        (position.x.to_f32() / TILE_SIZE).floor() as i32,
        (position.y.to_f32() / TILE_SIZE).floor() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::{CellGrid, Map};
    use crate::td_rules::UnitType;
    use cnc_core::Fixed16_16;

    fn sample_grid() -> CellGrid {
        Map::new("test".to_string(), 20, 20).grid
    }

    #[test]
    fn splitmix_matches_pinned_vectors() {
        // Pinned SplitMix64(seed=0) outputs — same contract as
        // cnc-procgen::prng; changing the algorithm breaks determinism.
        let mut state = 0u64;
        assert_eq!(splitmix(&mut state), 16_294_208_416_658_607_535);
        assert_eq!(splitmix(&mut state), 7_960_286_522_194_355_700);
        assert_eq!(splitmix(&mut state), 487_617_019_471_545_679);
    }

    #[test]
    fn spawns_are_deterministic_and_cadenced() {
        let grid = sample_grid();
        let mut a = PowerupState::new();
        let mut b = PowerupState::new();
        // No spawn before the interval elapses.
        for _ in 0..SPAWN_INTERVAL_TICKS - 1 {
            a.tick(&grid, &[]);
        }
        assert_eq!(a.crates.len(), 0);
        a.tick(&grid, &[]);
        assert_eq!(a.crates.len(), 1);
        // Same seed, same schedule: two independent runs agree.
        for _ in 0..SPAWN_INTERVAL_TICKS {
            b.tick(&grid, &[]);
        }
        assert_eq!(a.crates, b.crates);
        assert_eq!(a.next_spawn_at, b.next_spawn_at);
    }

    #[test]
    fn drive_over_collects_and_removes_the_crate() {
        let mut state = PowerupState::new();
        let position = Vec2::new(
            Fixed16_16::from_f32(5.0 * TILE_SIZE + TILE_SIZE * 0.5),
            Fixed16_16::from_f32(5.0 * TILE_SIZE + TILE_SIZE * 0.5),
        );
        let unit = Unit::new(UnitType::InfantryE1, position, 0);
        state.crates.push(Crate {
            id: 1,
            cell: Vec2i::new(5, 5),
            kind: PowerupKind::Credits,
            spawned_tick: 0,
            expires_tick: u32::MAX,
        });
        let grid = sample_grid();
        let pickups = state.tick(&grid, &[unit.clone(), unit]);
        assert_eq!(pickups.len(), 1);
        assert_eq!(pickups[0].unit_index, 0);
        assert_eq!(pickups[0].owner, 0);
        assert_eq!(pickups[0].kind, PowerupKind::Credits);
        assert!(state.crates.is_empty());
    }

    #[test]
    fn crates_expire_after_their_lifetime() {
        let mut state = PowerupState::new();
        state.crates.push(Crate {
            id: 1,
            cell: Vec2i::new(5, 5),
            kind: PowerupKind::Armor,
            spawned_tick: 0,
            expires_tick: 2,
        });
        let grid = sample_grid();
        state.tick(&grid, &[]);
        assert_eq!(state.crates.len(), 1);
        state.tick(&grid, &[]);
        assert!(state.crates.is_empty());
    }

    #[test]
    fn dead_units_never_collect() {
        let mut state = PowerupState::new();
        let mut unit = Unit::new(
            UnitType::InfantryE1,
            Vec2::new(Fixed16_16::ZERO, Fixed16_16::ZERO),
            0,
        );
        unit.health = Fixed16_16::ZERO;
        state.crates.push(Crate {
            id: 1,
            cell: Vec2i::new(0, 0),
            kind: PowerupKind::Unit,
            spawned_tick: 0,
            expires_tick: u32::MAX,
        });
        let grid = sample_grid();
        let pickups = state.tick(&grid, &[unit]);
        assert!(pickups.is_empty());
        assert_eq!(state.crates.len(), 1);
    }
}
