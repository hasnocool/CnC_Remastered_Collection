//! Versioned binary savegames for live [`Game`] state.
//!
//! The original engines persist every object and then run
//! `Code_Pointers`/`Decode_Pointers` to turn raw pointers back into indices.
//! The Rust port stores entity references as indices already, so a snapshot is
//! a plain serialized value plus a file header:
//!
//! ```text
//! offset  size  field
//! 0       8     magic "CNCSAVE\0"
//! 8       4     format version (little endian)
//! 12      8     payload length
//! 20      8     FNV-1a 64 checksum of the payload
//! 28      ...   bincode payload
//! ```
//!
//! Transient visuals (blasts, missile strikes, ion beams and queued delayed
//! blasts) are deliberately not persisted; gameplay state, production queues,
//! projectiles in flight, mission flags/triggers and Rules.ini overrides are.

use anyhow::{bail, Context, Result};
use cnc_core::rules::RulesDatabase;
use cnc_core::{Fixed16_16, Vec2};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::building::Building;
use crate::game::Game;
use crate::house::House;
use crate::map::Map;
use crate::mission::Mission;
use crate::unit::Unit;
use crate::weapon::Projectile;

/// File magic; the trailing NUL makes text-mode mangling obvious.
pub const SAVE_MAGIC: [u8; 8] = *b"CNCSAVE\0";
/// Current save format version. Bump when [`GameSnapshot`] changes shape.
pub const SAVE_VERSION: u32 = 5;
/// Header size in bytes.
pub const SAVE_HEADER_LEN: usize = 8 + 4 + 8 + 8;

/// Everything needed to resume a session exactly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSnapshot {
    pub version: u32,
    pub map: Map,
    pub houses: Vec<House>,
    pub units: Vec<Unit>,
    pub buildings: Vec<Building>,
    pub mission: Mission,
    pub elapsed: Fixed16_16,
    pub patrol_center: Vec2,
    pub patrol_radius: Fixed16_16,
    pub projectiles: Vec<Projectile>,
    pub projectiles_fired: u64,
    pub tiberium_tick: u32,
    pub units_produced: u64,
    pub harvested_credits: i64,
    pub ready_buildings: Vec<(u8, crate::td_rules::StructureType)>,
    pub aircraft_shots: u64,
    pub aircraft_rearmed: u64,
    pub secondary_shots: u64,
    /// CRATE.CPP crate powerup state (deterministic spawns + pickups).
    #[serde(default)]
    pub powerups: crate::powerup::PowerupState,
    /// Per-house expert AI state (index == house index).
    #[serde(default)]
    pub ai: Vec<crate::ai::AI>,
    /// Cells the player has explored, so `EVENT_DISCOVERED` triggers keep
    /// working across a save/load.
    #[serde(default)]
    pub discovered_cells: Vec<cnc_core::Vec2i>,
    /// Cumulative kills, so kill-count triggers survive a save/load.
    #[serde(default)]
    pub kills: Vec<((u8, crate::mission::Eliminated), i64)>,
    /// Balance overrides loaded from Rules.ini, so a save is self-contained.
    pub rules: RulesDatabase,
    /// `[TeamTypes]` roster plus running instances. Defaulted so saves
    /// written before this field existed still load.
    #[serde(default)]
    pub teams: crate::ai::TeamManager,
}

impl Game {
    /// Build a serializable snapshot of the live simulation.
    pub fn snapshot(&self) -> GameSnapshot {
        GameSnapshot {
            version: SAVE_VERSION,
            map: self.map.clone(),
            houses: self.houses.clone(),
            units: self.units.clone(),
            buildings: self.buildings.clone(),
            mission: self.mission.clone(),
            elapsed: self.elapsed,
            patrol_center: self.patrol_center,
            patrol_radius: self.patrol_radius,
            projectiles: self.projectiles.clone(),
            projectiles_fired: self.projectiles_fired,
            tiberium_tick: self.tiberium_tick,
            units_produced: self.units_produced,
            harvested_credits: self.harvested_credits,
            ready_buildings: self.ready_buildings.clone(),
            aircraft_shots: self.aircraft_shots,
            aircraft_rearmed: self.aircraft_rearmed,
            secondary_shots: self.secondary_shots,
            powerups: self.powerups.clone(),
            ai: self.ai.clone(),
            kills: self.kills.iter().map(|(k, v)| (*k, *v)).collect(),
            discovered_cells: self.discovered_cells.iter().copied().collect(),
            rules: self.rules.clone(),
            teams: self.team_manager.clone(),
        }
    }

    /// Rebuild a game from a snapshot (transient visuals start empty).
    pub fn from_snapshot(snapshot: GameSnapshot) -> Result<Self> {
        if snapshot.version != SAVE_VERSION {
            bail!(
                "unsupported save version {} (this build reads {}; saves from \
                 earlier builds are incompatible — start a new mission)",
                snapshot.version,
                SAVE_VERSION
            );
        }
        let (map_width, map_height) = (snapshot.map.width, snapshot.map.height);
        let game = Self {
            player_owner: snapshot.mission.player,
            map: snapshot.map,
            houses: snapshot.houses,
            units: snapshot.units,
            buildings: snapshot.buildings,
            mission: snapshot.mission,
            elapsed: snapshot.elapsed,
            patrol_center: snapshot.patrol_center,
            patrol_radius: snapshot.patrol_radius,
            blasts: Vec::new(),
            strikes: Vec::new(),
            beams: Vec::new(),
            rules: snapshot.rules,
            // Saves predate the shroud grid; reload fully revealed so an old
            // game stays playable rather than fogging over.
            shroud: cnc_core::Shroud::fully_revealed(map_width, map_height),
            team_manager: snapshot.teams,
            // Speech is transient presentation state, not saved.
            eva: cnc_audio::EvaQueue::new(),
            // Derived on the next power refresh.
            low_power_announced: Vec::new(),
            kills: snapshot.kills.iter().copied().collect(),
            discovered_cells: snapshot.discovered_cells.into_iter().collect(),
            dead_units_announced: Vec::new(),
            dead_buildings_announced: Vec::new(),
            next_base_attack_speak: Fixed16_16::ZERO,
            next_funds_speak: Fixed16_16::ZERO,
            projectiles: snapshot.projectiles,
            projectiles_fired: snapshot.projectiles_fired,
            tiberium_tick: snapshot.tiberium_tick,
            units_produced: snapshot.units_produced,
            harvested_credits: snapshot.harvested_credits,
            ready_buildings: snapshot.ready_buildings,
            aircraft_shots: snapshot.aircraft_shots,
            aircraft_rearmed: snapshot.aircraft_rearmed,
            secondary_shots: snapshot.secondary_shots,
            powerups: snapshot.powerups,
            ai: snapshot.ai,
            pending_blasts: Vec::new(),
            radar_state: Vec::new(),
            production_queue: Vec::new(),
            // Loaded games start with no attached bus; the host re-attaches.
            event_source: None,
            mission_outcome_emitted: false,
        };
        // Validate all transport passenger lists refer to live, in-bounds,
        // infantry passengers and never exceed capacity.
        for unit in &game.units {
            if unit.is_transport() {
                if unit.passengers.len() > unit.capacity {
                    bail!(
                        "transport has {} passengers but capacity {}",
                        unit.passengers.len(),
                        unit.capacity
                    );
                }
                for &passenger_idx in &unit.passengers {
                    let Some(passenger) = game.units.get(passenger_idx) else {
                        bail!(
                            "transport passenger index {} is out of bounds",
                            passenger_idx
                        );
                    };
                    if !passenger.is_infantry() {
                        bail!("transport passenger {} is not infantry", passenger_idx);
                    }
                    if !passenger.is_alive() {
                        bail!("transport passenger {} is not alive", passenger_idx);
                    }
                }
            }
        }
        game.validate_sub_cells().map_err(|e| anyhow::anyhow!(e))?;
        Ok(game)
    }

    /// Serialize the game to a versioned, checksummed byte buffer.
    pub fn to_save_bytes(&self) -> Result<Vec<u8>> {
        let payload = bincode::serialize(&self.snapshot()).context("failed to encode savegame")?;
        let mut bytes = Vec::with_capacity(SAVE_HEADER_LEN + payload.len());
        bytes.extend_from_slice(&SAVE_MAGIC);
        bytes.extend_from_slice(&SAVE_VERSION.to_le_bytes());
        bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        bytes.extend_from_slice(&fnv1a64(&payload).to_le_bytes());
        bytes.extend_from_slice(&payload);
        Ok(bytes)
    }

    /// Write a savegame file (creates parent directories).
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
        }
        let bytes = self.to_save_bytes()?;
        std::fs::write(path, bytes)
            .with_context(|| format!("failed to write savegame {}", path.display()))
    }

    /// Load a game from a versioned byte buffer.
    pub fn from_save_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < SAVE_HEADER_LEN {
            bail!(
                "savegame is too short ({} bytes, header needs {SAVE_HEADER_LEN})",
                bytes.len()
            );
        }
        if bytes[..8] != SAVE_MAGIC {
            bail!("not a C&C savegame (bad magic)");
        }
        let version = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        if version != SAVE_VERSION {
            bail!("unsupported save version {version} (this build reads {SAVE_VERSION}; saves from earlier builds are incompatible — start a new mission)");
        }
        let length = u64::from_le_bytes(bytes[12..20].try_into().unwrap()) as usize;
        let checksum = u64::from_le_bytes(bytes[20..28].try_into().unwrap());
        let payload = &bytes[SAVE_HEADER_LEN..];
        if payload.len() != length {
            bail!(
                "savegame payload is {} bytes but header says {length}",
                payload.len()
            );
        }
        if fnv1a64(payload) != checksum {
            bail!("savegame checksum mismatch (file is corrupt)");
        }
        let snapshot: GameSnapshot =
            bincode::deserialize(payload).context("failed to decode savegame")?;
        Self::from_snapshot(snapshot)
    }

    /// Load a game from disk.
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read savegame {}", path.display()))?;
        Self::from_save_bytes(&bytes)
            .with_context(|| format!("failed to load savegame {}", path.display()))
    }
}

/// FNV-1a 64-bit checksum; small and dependency-free.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    fn demo_game() -> Game {
        let mut game = Game::new("save-test", 32, 32);
        game.setup_demo();
        game
    }

    #[test]
    fn roundtrip_preserves_live_state() {
        let mut game = demo_game();
        game.elapsed = Fixed16_16::from_int(123);
        game.units[0].health = Fixed16_16::from_int(17);
        game.houses[0].credits = 4242;
        game.tiberium_tick = 7;
        game.units_produced = 3;
        game.ready_buildings
            .push((0, crate::td_rules::StructureType::Refinery));

        let bytes = game.to_save_bytes().unwrap();
        let loaded = Game::from_save_bytes(&bytes).unwrap();
        let a = game.snapshot();
        let b = loaded.snapshot();
        assert_eq!(a.units.len(), b.units.len());
        assert_eq!(a.buildings.len(), b.buildings.len());
        assert_eq!(a.elapsed, b.elapsed);
        assert_eq!(a.houses[0].credits, b.houses[0].credits);
        assert_eq!(a.units[0].health, b.units[0].health);
        assert_eq!(a.ready_buildings, b.ready_buildings);
        assert_eq!(
            bincode::serialize(&a).unwrap(),
            bincode::serialize(&b).unwrap(),
            "snapshot must round-trip byte-identically"
        );
    }

    #[test]
    fn roundtrip_preserves_mission_progress() {
        let mut game = demo_game();
        let mut mission = Mission::new("progress".to_string());
        mission.add_objective(1, "Hold the line");
        mission.add_trigger(crate::mission::Trigger::new(
            1,
            crate::mission::Condition::Always,
            vec![
                crate::mission::Action::SetFlag("started".to_string()),
                crate::mission::Action::Credits {
                    owner: 0,
                    amount: 500,
                },
            ],
        ));
        game.start_mission(mission);
        game.update_mission(Fixed16_16::ONE);

        let loaded = Game::from_save_bytes(&game.to_save_bytes().unwrap()).unwrap();
        assert!(loaded.mission.flags.contains("started"));
        assert_eq!(loaded.mission.fired_order(), &[1]);
        assert_eq!(loaded.mission.name, "progress");

        // Loaded trigger is already spent: no second credit grant.
        let mut loaded = loaded;
        let credits = loaded.houses[0].credits;
        loaded.update_mission(Fixed16_16::ONE);
        assert_eq!(loaded.houses[0].credits, credits);
    }

    #[test]
    fn round_trip_preserves_unit_targets() {
        let mut game = Game::new("save", 50, 50);
        game.setup_demo();
        // Give the first GDI unit an attack order against a Nod unit.
        let Some(target) = game.units.iter().position(|u| u.owner == 1) else {
            panic!("setup_demo must field Nod units");
        };
        let shooter = 0usize;
        game.units[shooter].attack_target = Some(target);
        let expected = game.units[shooter].attack_target;

        let bytes = game.to_save_bytes().expect("save");
        let loaded = Game::from_save_bytes(&bytes).expect("load");
        assert_eq!(
            loaded.units[shooter].attack_target, expected,
            "attack targets are indices and should survive a round trip"
        );
    }

    #[test]
    fn round_trip_preserves_the_team_roster() {
        // team_manager used to be dropped on save, silently losing the
        // scenario's [TeamTypes] mid-mission.
        let mut game = Game::new("teams", 50, 50);
        game.setup_demo();
        let ini = cnc_core::IniFile::parse(
            "[TaskForces]\n1=TF_T\n[TF_T]\n1=MTNK,1\n[TeamTypes]\n1=T\n\
             [T]\nName=T\nOwner=Nod\nTaskForce=TF_T\nMission=attack\n",
        )
        .expect("ini");
        let types = cnc_core::entity::team_types_from_ini(&ini).expect("types");
        game.team_manager = crate::ai::TeamManager::new(types);
        assert!(!game.team_manager.is_empty());

        let bytes = game.to_save_bytes().expect("save");
        let loaded = Game::from_save_bytes(&bytes).expect("load");
        assert!(!loaded.team_manager.is_empty(), "the roster must survive");
        assert_eq!(
            loaded.team_manager.types.len(),
            game.team_manager.types.len()
        );
        assert_eq!(
            loaded.team_manager.types[0].name,
            game.team_manager.types[0].name
        );
    }

    #[test]
    fn rejects_corrupt_or_truncated_saves() {
        let game = demo_game();
        let bytes = game.to_save_bytes().unwrap();

        assert!(Game::from_save_bytes(&bytes[..10]).is_err());

        let mut bad_magic = bytes.clone();
        bad_magic[0] = b'X';
        assert!(Game::from_save_bytes(&bad_magic).is_err());

        let mut corrupt = bytes;
        let last = corrupt.len() - 1;
        corrupt[last] ^= 0xff;
        let Err(error) = Game::from_save_bytes(&corrupt) else {
            panic!("corrupt save must be rejected");
        };
        let error = error.to_string();
        assert!(error.contains("checksum"), "unexpected error: {error}");
    }

    #[test]
    fn file_roundtrip_creates_directories() {
        let dir = std::env::temp_dir().join(format!("cnc-save-test-{}", std::process::id()));
        let path = dir.join("nested").join("slot1.sav");
        let game = demo_game();
        game.save_to_file(&path).unwrap();
        let loaded = Game::load_from_file(&path).unwrap();
        assert_eq!(loaded.units.len(), game.units.len());
        assert_eq!(loaded.map.name, game.map.name);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
