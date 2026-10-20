//! Campaign carry-over — Rust port of the original `CARRY.CPP`
//! (`CarryoverClass`): surviving units and credits persist between the
//! missions of a campaign.
//!
//! The struct is plain serde data; persistence uses the same versioned
//! header layout as `savegame.rs` (magic + version + length + FNV-1a +
//! bincode payload) so carry-over files are checksummed and self-describing.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::savegame::fnv1a64;
use crate::td_rules::UnitType;
use crate::unit::Unit;
use cnc_core::{Fixed16_16, Vec2};

/// File magic for carry-over files.
pub const CARRY_MAGIC: [u8; 8] = *b"CNCCARRY";
/// Current carry-over format version.
pub const CARRY_VERSION: u32 = 1;
/// Header size: magic(8) + version(4) + length(8) + checksum(8).
pub const CARRY_HEADER_LEN: usize = 8 + 4 + 8 + 8;
/// At most this many units survive into the next mission.
pub const MAX_CARRIED_UNITS: usize = 20;
/// Credits are capped on transfer (the original clamped too).
pub const CREDITS_CAP: i32 = 9_999;

/// One surviving unit carried into the next mission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarriedUnit {
    pub unit_type: UnitType,
    pub position: Vec2,
    pub health: Fixed16_16,
}

/// Surviving forces + treasury bridging two campaign missions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Carryover {
    pub owner: u8,
    pub credits: i32,
    pub units: Vec<CarriedUnit>,
}

impl Carryover {
    /// Capture the player's surviving units and treasury at mission end.
    pub fn from_game(game: &Game, owner: u8) -> Self {
        let units = game
            .units
            .iter()
            .filter(|unit| unit.owner == owner && unit.is_alive())
            .take(MAX_CARRIED_UNITS)
            .map(|unit| CarriedUnit {
                unit_type: unit.unit_type,
                position: unit.position,
                health: unit.health,
            })
            .collect();
        let credits = game
            .houses
            .get(owner as usize)
            .map(|house| house.credits)
            .unwrap_or(0);
        Self {
            owner,
            credits,
            units,
        }
    }

    /// Apply the carry-over to a freshly started mission: set the treasury
    /// to the carried amount (capped — campaign continuity replaces the
    /// scenario default, mirroring `CARRY.CPP`) and re-deploy the surviving
    /// units at their recorded positions (clamped into the map; units
    /// landing on impassable ground are dropped).
    pub fn apply_to_game(&self, game: &mut Game) {
        if let Some(house) = game.houses.get_mut(self.owner as usize) {
            house.credits = self.credits.min(CREDITS_CAP);
        }
        for carried in &self.units {
            let position = clamp_to_map(carried.position, &game.map.grid);
            let cell = Vec2::new(position.x, position.y);
            let cell_index = cnc_core::Vec2i::new(
                (position.x.to_f32() / crate::map::TILE_SIZE).floor() as i32,
                (position.y.to_f32() / crate::map::TILE_SIZE).floor() as i32,
            );
            let Some(cell_data) = game.map.grid.get(cell_index) else {
                continue;
            };
            if !cell_data.is_passable {
                continue;
            }
            let mut unit = Unit::new(carried.unit_type, cell, self.owner);
            unit.health = carried.health.min(unit.max_health);
            let id = game.units.len() as u32;
            if let Some(house) = game.houses.get_mut(self.owner as usize) {
                house.add_unit(id);
            }
            game.units.push(unit);
        }
    }

    /// Serialize to a versioned, checksummed byte buffer.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let payload = bincode::serialize(self).context("failed to encode carry-over")?;
        let mut bytes = Vec::with_capacity(CARRY_HEADER_LEN + payload.len());
        bytes.extend_from_slice(&CARRY_MAGIC);
        bytes.extend_from_slice(&CARRY_VERSION.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&fnv1a64(&payload).to_le_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    /// Parse a carry-over buffer (validates magic, version and checksum).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < CARRY_HEADER_LEN {
            bail!("carry-over buffer too short");
        }
        if bytes[0..8] != CARRY_MAGIC {
            bail!("not a carry-over file (bad magic)");
        }
        let version = u32::from_le_bytes(bytes[8..12].try_into()?);
        if version != CARRY_VERSION {
            bail!("unsupported carry-over version {version}");
        }
        let length = u64::from_le_bytes(bytes[12..20].try_into()?) as usize;
        if bytes.len() < CARRY_HEADER_LEN + length {
            bail!("carry-over payload truncated");
        }
        let payload = &bytes[CARRY_HEADER_LEN..CARRY_HEADER_LEN + length];
        let expected = u64::from_le_bytes(bytes[20..28].try_into()?);
        if fnv1a64(payload) != expected {
            bail!("carry-over checksum mismatch");
        }
        bincode::deserialize(payload).context("failed to decode carry-over")
    }

    /// Write a carry-over file (creates parent directories).
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
        }
        std::fs::write(path, self.to_bytes()?)
            .with_context(|| format!("failed to write {}", path.display()))
    }

    /// Read a carry-over file.
    pub fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let bytes =
            std::fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
        Self::from_bytes(&bytes)
    }
}

/// Clamp a world position into the map bounds.
fn clamp_to_map(position: Vec2, grid: &crate::map::CellGrid) -> Vec2 {
    let max_x =
        (grid.width.saturating_sub(1)) as f32 * crate::map::TILE_SIZE + crate::map::TILE_SIZE * 0.5;
    let max_y = (grid.height.saturating_sub(1)) as f32 * crate::map::TILE_SIZE
        + crate::map::TILE_SIZE * 0.5;
    Vec2::new(
        position
            .x
            .clamp(Fixed16_16::from_f32(0.0), Fixed16_16::from_f32(max_x)),
        position
            .y
            .clamp(Fixed16_16::from_f32(0.0), Fixed16_16::from_f32(max_y)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::house::House;
    use crate::td_rules::UnitType;
    use cnc_core::ClassId;

    fn sample_game() -> Game {
        let mut game = Game::new("test", 20, 20);
        let mut gdi = House::new(ClassId::HouseGDI, "GDI".to_string());
        gdi.add_credits(4_321);
        game.houses.push(gdi);
        game
    }

    #[test]
    fn from_game_captures_only_surviving_owner_units() {
        let mut game = sample_game();
        let origin = Vec2::new(Fixed16_16::ZERO, Fixed16_16::ZERO);
        let alive = Unit::new(UnitType::MediumTank, origin, 0);
        let mut enemy = Unit::new(UnitType::MediumTank, origin, 1);
        enemy.health = Fixed16_16::ZERO;
        let mut dead = Unit::new(UnitType::MediumTank, origin, 0);
        dead.health = Fixed16_16::ZERO;
        game.units.push(alive.clone());
        game.units.push(enemy);
        game.units.push(dead);

        let carryover = Carryover::from_game(&game, 0);
        assert_eq!(carryover.units.len(), 1);
        assert_eq!(carryover.units[0].unit_type, UnitType::MediumTank);
        assert_eq!(carryover.credits, 4_321);
    }

    #[test]
    fn apply_grants_capped_credits_and_redeploys_units() {
        let mut game = sample_game();
        let carryover = Carryover {
            owner: 0,
            credits: 50_000,
            units: vec![CarriedUnit {
                unit_type: UnitType::MediumTank,
                position: Vec2::new(
                    Fixed16_16::from_f32(5.0 * crate::map::TILE_SIZE),
                    Fixed16_16::from_f32(5.0 * crate::map::TILE_SIZE),
                ),
                health: Fixed16_16::from_f32(30.0),
            }],
        };
        carryover.apply_to_game(&mut game);
        assert_eq!(game.houses[0].credits, CREDITS_CAP);
        assert_eq!(game.units.len(), 1);
        assert_eq!(game.units[0].health, carryover.units[0].health);
    }

    #[test]
    fn bytes_round_trip_preserves_the_carryover() {
        let carryover = Carryover {
            owner: 1,
            credits: 1_234,
            units: vec![CarriedUnit {
                unit_type: UnitType::Jeep,
                position: Vec2::new(Fixed16_16::from_int(64), Fixed16_16::from_int(96)),
                health: Fixed16_16::from_f32(42.5),
            }],
        };
        let bytes = carryover.to_bytes().expect("encode");
        let decoded = Carryover::from_bytes(&bytes).expect("decode");
        assert_eq!(decoded, carryover);
    }

    #[test]
    fn corrupted_buffers_are_rejected_cleanly() {
        let carryover = Carryover {
            owner: 0,
            credits: 0,
            units: Vec::new(),
        };
        let mut bytes = carryover.to_bytes().expect("encode");
        // Flip a payload byte: checksum must catch it.
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        assert!(Carryover::from_bytes(&bytes).is_err());
        // Bad magic.
        let mut bytes = carryover.to_bytes().expect("encode");
        bytes[0] = b'X';
        assert!(Carryover::from_bytes(&bytes).is_err());
        // Truncated.
        assert!(Carryover::from_bytes(&bytes[..10]).is_err());
    }

    #[test]
    fn file_round_trip_through_a_temp_path() {
        let carryover = Carryover {
            owner: 0,
            credits: 777,
            units: Vec::new(),
        };
        let path =
            std::env::temp_dir().join(format!("cnc-carryover-test-{}.bin", std::process::id()));
        carryover.save_to_file(&path).expect("save");
        let loaded = Carryover::load_from_file(&path).expect("load");
        let _ = std::fs::remove_file(&path);
        assert_eq!(loaded, carryover);
    }
}
