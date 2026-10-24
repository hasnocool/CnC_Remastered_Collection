//! Campaign carry-over for Red Alert — adapted from the TD `Carryover` module.
//!
//! RA uses a text-based save format (INI-like) rather than the binary CNCCARRY
//! format used by TD. This module twins the TD semantics: surviving units and
//! credits persist between campaign missions. The on-disk format mirrors the
//! RA savegame text layout for consistency.

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::game::Game;
use cnc_core::{ClassId, Fixed16_16, Vec2};

/// File magic for RA carry-over files.
pub const RA_CARRY_MAGIC: [u8; 4] = *b"RACR";
/// Current carry-over format version.
pub const RA_CARRY_VERSION: u32 = 1;
/// Header size: magic(4) + version(4) + length(4) + checksum(8).
pub const RA_CARRY_HEADER_LEN: usize = 4 + 4 + 4 + 8;
/// At most this many units survive into the next mission.
pub const MAX_CARRIED_UNITS: usize = 20;
/// Credits are capped on transfer.
pub const CREDITS_CAP: i32 = 9_999;

/// One surviving unit carried into the next mission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarriedUnit {
    pub class_id: ClassId,
    pub position: Vec2,
    pub health: Fixed16_16,
}

/// Surviving forces + treasury bridging two campaign missions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaCarryover {
    pub owner: u8,
    pub credits: i32,
    pub units: Vec<CarriedUnit>,
    /// The RNG state at mission end (for deterministic replay continuation).
    pub rng_state: u64,
    /// The simulation tick when the mission ended.
    pub end_tick: i32,
}

impl RaCarryover {
    /// Capture the player's surviving units and treasury at mission end.
    pub fn from_game(game: &Game, owner: u8) -> Self {
        let units = game
            .units
            .iter()
            .filter(|unit| unit.owner as u8 == owner && unit.is_alive())
            .take(MAX_CARRIED_UNITS)
            .map(|unit| CarriedUnit {
                class_id: unit.class_id,
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
            rng_state: game.rng.state(),
            end_tick: game.elapsed.to_bits(),
        }
    }

    /// Apply the carry-over to a freshly started mission: set the treasury
    pub fn apply_to_game(&self, game: &mut Game) {
        if let Some(house) = game.houses.get_mut(self.owner as usize) {
            house.credits = self.credits.min(CREDITS_CAP);
        }
        for carried in &self.units {
            let position = clamp_to_map(carried.position, &game.map);
            let x = (position.x.to_f32() / crate::map::TILE_SIZE).floor() as u32;
            let y = (position.y.to_f32() / crate::map::TILE_SIZE).floor() as u32;
            if x >= game.map.width || y >= game.map.height {
                continue;
            }
            let idx = (y * game.map.width + x) as usize;
            let terrain = game.map.terrain[idx];
            if terrain == crate::map::Terrain::Water {
                continue;
            }
            let cell = cnc_core::Vec2i::new(x as i32, y as i32);
            let id = game.spawn_unit(self.owner as usize, carried.class_id, cell);
            if let Some(unit) = game.units.iter_mut().find(|u| u.id == id) {
                unit.health = carried.health.min(unit.max_health);
            }
        }
        // Restore RNG state for deterministic continuation
        game.rng.set_state(self.rng_state);
    }
    /// Serialize to a versioned, checksummed byte buffer.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let payload = bincode::serialize(self).context("failed to encode RA carry-over")?;
        let mut bytes = Vec::with_capacity(RA_CARRY_HEADER_LEN + payload.len());
        bytes.extend_from_slice(&RA_CARRY_MAGIC);
        bytes.extend_from_slice(&RA_CARRY_VERSION.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&fnv1a64(&payload).to_le_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    /// Parse a carry-over buffer (validates magic, version and checksum).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < RA_CARRY_HEADER_LEN {
            bail!("RA carry-over buffer too short");
        }
        if bytes[0..4] != RA_CARRY_MAGIC {
            bail!("not an RA carry-over file (bad magic)");
        }
        let version = u32::from_le_bytes(bytes[4..8].try_into()?);
        if version != RA_CARRY_VERSION {
            bail!("unsupported RA carry-over version {version}");
        }
        let length = u32::from_le_bytes(bytes[8..12].try_into()?) as usize;
        if bytes.len() < RA_CARRY_HEADER_LEN + length {
            bail!("RA carry-over payload truncated");
        }
        let payload = &bytes[RA_CARRY_HEADER_LEN..RA_CARRY_HEADER_LEN + length];
        let expected = u64::from_le_bytes(bytes[12..20].try_into()?);
        if fnv1a64(payload) != expected {
            bail!("RA carry-over checksum mismatch");
        }
        bincode::deserialize(payload).context("failed to decode RA carry-over")
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

    /// Convert to text-based INI format for human-readable inspection / compatibility.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str("[Carryover]\n");
        out.push_str(&format!("Owner={}\n", self.owner));
        out.push_str(&format!("Credits={}\n", self.credits));
        out.push_str(&format!("RngState={}\n", self.rng_state));
        out.push_str(&format!("EndTick={}\n", self.end_tick));
        out.push_str(&format!("UnitCount={}\n", self.units.len()));
        for (i, unit) in self.units.iter().enumerate() {
            out.push_str(&format!(
                "Unit{}={},{},{},{}\n",
                i,
                unit.class_id as u32,
                unit.position.x.to_bits(),
                unit.position.y.to_bits(),
                unit.health.to_bits()
            ));
        }
        out
    }

    /// Parse from text-based INI format.
    pub fn from_text(text: &str) -> Result<Self> {
        let mut owner = 0u8;
        let mut credits = 0i32;
        let mut rng_state = 0u64;
        let mut end_tick = 0i32;
        let mut units = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('[') || line.starts_with(';') {
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            match key.trim().to_ascii_lowercase().as_str() {
                "owner" => owner = value.trim().parse().unwrap_or(0),
                "credits" => credits = value.trim().parse().unwrap_or(0),
                "rngstate" => rng_state = value.trim().parse().unwrap_or(0),
                "endtick" => end_tick = value.trim().parse().unwrap_or(0),
                "unitcount" => {
                    // We'll parse units individually
                }
                key if key.starts_with("unit") => {
                    let parts: Vec<&str> = value.split(',').collect();
                    if parts.len() == 4 {
                        let class_id =
                            ClassId::from_u32(parts[0].trim().parse::<u32>().unwrap_or(0));
                        let pos_x = Fixed16_16::from_bits(parts[1].trim().parse().unwrap_or(0));
                        let pos_y = Fixed16_16::from_bits(parts[2].trim().parse().unwrap_or(0));
                        let health = Fixed16_16::from_bits(parts[3].trim().parse().unwrap_or(0));
                        units.push(CarriedUnit {
                            class_id,
                            position: Vec2::new(pos_x, pos_y),
                            health,
                        });
                    }
                }
                _ => {}
            }
        }

        Ok(Self {
            owner,
            credits,
            units,
            rng_state,
            end_tick,
        })
    }
}

/// Clamp a world position into the map bounds.
fn clamp_to_map(position: Vec2, map: &crate::map::Map) -> Vec2 {
    let max_x =
        (map.width.saturating_sub(1)) as f32 * crate::map::TILE_SIZE + crate::map::TILE_SIZE * 0.5;
    let max_y =
        (map.height.saturating_sub(1)) as f32 * crate::map::TILE_SIZE + crate::map::TILE_SIZE * 0.5;
    Vec2::new(
        position
            .x
            .clamp(Fixed16_16::from_f32(0.0), Fixed16_16::from_f32(max_x)),
        position
            .y
            .clamp(Fixed16_16::from_f32(0.0), Fixed16_16::from_f32(max_y)),
    )
}

/// FNV-1a 64-bit hash.
fn fnv1a64(bytes: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = FNV_OFFSET;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::house::{Country, Faction, House};
    use cnc_core::{ClassId, Fixed16_16};

    fn sample_game() -> Game {
        let mut game = Game::new(20, 20);
        game.houses.clear();
        let mut house = House::new(ClassId::HouseAllies, "England".to_string(), Faction::Allied)
            .with_country(Country::England);
        house.add_credits(4_321);
        game.houses.push(house);
        game.units.clear();
        game
    }

    #[test]
    fn ra_mission_carry_preserves_credits_and_units() {
        let mut game = sample_game();
        let cell = cnc_core::Vec2i::new(5, 5);

        // Add a surviving unit
        let id1 = game.spawn_unit(0, ClassId::from_u32(10), cell);
        if let Some(unit) = game.units.iter_mut().find(|u| u.id == id1) {
            unit.health = Fixed16_16::from_int(80);
        }

        // Add a dead unit (should not carry)
        let id2 = game.spawn_unit(0, ClassId::from_u32(10), cell);
        if let Some(unit) = game.units.iter_mut().find(|u| u.id == id2) {
            unit.health = Fixed16_16::ZERO;
        }

        // Add enemy unit (should not carry)
        let _id3 = game.spawn_unit(1, ClassId::from_u32(11), cell);

        let carryover = RaCarryover::from_game(&game, 0);
        eprintln!("units: {:?}", carryover.units.len());
        for u in &carryover.units {
            eprintln!("unit class {} owner", u.class_id as u32);
        }
        assert_eq!(carryover.units.len(), 1);
        assert_eq!(carryover.units[0].class_id, ClassId::from_u32(10));
        assert_eq!(carryover.credits, 4_321);
    }

    #[test]
    fn ra_apply_grants_capped_credits_and_redeploys_units() {
        let mut game = sample_game();
        let pos = Vec2::new(
            Fixed16_16::from_f32(5.0 * crate::map::TILE_SIZE),
            Fixed16_16::from_f32(5.0 * crate::map::TILE_SIZE),
        );
        let carryover = RaCarryover {
            owner: 0,
            credits: 50_000,
            units: vec![CarriedUnit {
                class_id: ClassId::from_u32(10),
                position: pos,
                health: Fixed16_16::from_f32(30.0),
            }],
            rng_state: 0xDEADBEEF,
            end_tick: 1000,
        };
        carryover.apply_to_game(&mut game);
        assert_eq!(game.houses[0].credits, CREDITS_CAP);
        assert_eq!(game.units.len(), 1);
        assert_eq!(game.units[0].health, carryover.units[0].health);
        assert_eq!(game.rng.state(), 0xDEADBEEF);
    }

    #[test]
    fn ra_bytes_round_trip_preserves_the_carryover() {
        let carryover = RaCarryover {
            owner: 1,
            credits: 1_234,
            units: vec![CarriedUnit {
                class_id: ClassId::from_u32(5),
                position: Vec2::new(Fixed16_16::from_int(64), Fixed16_16::from_int(96)),
                health: Fixed16_16::from_f32(42.5),
            }],
            rng_state: 0xCAFEBABE,
            end_tick: 500,
        };
        let bytes = carryover.to_bytes().expect("encode");
        let decoded = RaCarryover::from_bytes(&bytes).expect("decode");
        assert_eq!(decoded, carryover);
    }

    #[test]
    fn ra_text_round_trip_preserves_the_carryover() {
        let carryover = RaCarryover {
            owner: 0,
            credits: 777,
            units: vec![
                CarriedUnit {
                    class_id: ClassId::from_u32(10),
                    position: Vec2::new(Fixed16_16::from_int(100), Fixed16_16::from_int(200)),
                    health: Fixed16_16::from_int(50),
                },
                CarriedUnit {
                    class_id: ClassId::from_u32(5),
                    position: Vec2::new(Fixed16_16::from_int(300), Fixed16_16::from_int(400)),
                    health: Fixed16_16::from_int(80),
                },
            ],
            rng_state: 0x12345678,
            end_tick: 1234,
        };
        let text = carryover.to_text();
        let decoded = RaCarryover::from_text(&text).expect("decode text");
        assert_eq!(decoded.owner, carryover.owner);
        assert_eq!(decoded.credits, carryover.credits);
        assert_eq!(decoded.rng_state, carryover.rng_state);
        assert_eq!(decoded.end_tick, carryover.end_tick);
        assert_eq!(decoded.units.len(), carryover.units.len());
        for (a, b) in decoded.units.iter().zip(carryover.units.iter()) {
            assert_eq!(a.class_id, b.class_id);
            assert_eq!(a.position.x.to_bits(), b.position.x.to_bits());
            assert_eq!(a.position.y.to_bits(), b.position.y.to_bits());
            assert_eq!(a.health.to_bits(), b.health.to_bits());
        }
    }

    #[test]
    fn ra_corrupted_buffers_are_rejected_cleanly() {
        let carryover = RaCarryover {
            owner: 0,
            credits: 0,
            units: Vec::new(),
            rng_state: 0,
            end_tick: 0,
        };
        let mut bytes = carryover.to_bytes().expect("encode");
        // Flip a payload byte: checksum must catch it.
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;
        assert!(RaCarryover::from_bytes(&bytes).is_err());
        // Bad magic.
        let mut bytes = carryover.to_bytes().expect("encode");
        bytes[0] = b'X';
        assert!(RaCarryover::from_bytes(&bytes).is_err());
        // Truncated.
        assert!(RaCarryover::from_bytes(&bytes[..10]).is_err());
    }

    #[test]
    fn ra_file_round_trip_through_a_temp_path() {
        let carryover = RaCarryover {
            owner: 0,
            credits: 777,
            units: Vec::new(),
            rng_state: 0,
            end_tick: 0,
        };
        let path =
            std::env::temp_dir().join(format!("cnc-ra-carryover-test-{}.bin", std::process::id()));
        carryover.save_to_file(&path).expect("save");
        let loaded = RaCarryover::load_from_file(&path).expect("load");
        let _ = std::fs::remove_file(&path);
        assert_eq!(loaded, carryover);
    }
}
