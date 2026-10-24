//! Top-level Tiberian Dawn game state.
//!
//! Owns the map, houses, units and buildings, advances a small demo
//! simulation and renders the whole scene through cnc_renderer.

use crate::ai::run_expert_ai;
use crate::Building;
use cnc_asset::SpriteAtlas;
use cnc_core::rules::RulesDatabase;
use cnc_core::{ClassId, Fixed16_16, Vec2, Vec2i};
use cnc_renderer::Renderer;
use std::sync::OnceLock;

/// Lazily loaded RTS tileset pack (shared across render calls), mirroring the
/// red-alert crate. Loaded once per process from the default pack directory
/// (`CNC_TILESET_DIR` / `CNC_TILESET_PALETTE` environment overrides apply).
static TD_TILESET: OnceLock<Option<cnc_asset::tileset_pack::TilesetPack>> = OnceLock::new();

use crate::ai::{AIDifficulty, AI};
use crate::house::House;
use crate::house::ProductionItem;
use crate::map::{Map, TILE_SIZE};
use crate::mission::{Action, Mission, MissionOutcome};
use crate::network::Command;
use crate::pathfind::find_path;
use crate::powerup::{self, PowerupKind, PowerupState};
use crate::td_rules::{
    default_structure_owner, default_unit_owner, leptons_to_px, modify_damage, owner_token_allows,
    prerequisite_token_met, production_seconds, px_to_leptons, tiberium_stage, unit_class_id,
    unit_type_for_class, ProductionKind, StructureType, UnitType, WarheadType, WeaponType,
    EXPLOSION_RANGE_LEPTONS, HARVESTER_STEP_COUNT, HARVESTER_STEP_CREDITS,
};
use crate::unit::Unit;
use crate::weapon::{Projectile, Warhead, Weapon};
use std::collections::{BTreeMap, BTreeSet};

/// Error returned when a sub-cell slot cannot be assigned or a snapshot
/// contains an invalid sub-cell assignment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubCellError {
    InvalidUnit,
    NotInfantry,
    CellFull,
    OutOfRange,
    DuplicateSlot,
}

impl std::fmt::Display for SubCellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUnit => write!(f, "invalid unit index"),
            Self::NotInfantry => write!(f, "unit is not infantry"),
            Self::CellFull => write!(f, "cell already holds 5 infantry"),
            Self::OutOfRange => write!(f, "sub-slot index out of range"),
            Self::DuplicateSlot => write!(f, "duplicate sub-slot in cell"),
        }
    }
}

impl std::error::Error for SubCellError {}

/// Error returned when a lockstep command carries an out-of-range or otherwise
/// malformed payload.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApplyCommandError {
    #[error("unit index {0} is out of range")]
    InvalidUnit(u32),
    #[error("building index {0} is out of range")]
    InvalidBuilding(u32),
    #[error("target index {0} is out of range")]
    InvalidTarget(u32),
    #[error("cell is out of bounds")]
    InvalidCell,
    #[error("class id {0:?} is not a unit type")]
    InvalidUnitType(ClassId),
    #[error("class id {0:?} is not a structure type")]
    InvalidStructure(ClassId),
}
/// Error returned by transport/cargo operations (boarding, unloading).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportError {
    InvalidTransportIndex,
    NotATransport,
    TransportNotAlive,
    InvalidPassengerIndex,
    PassengerNotAlive,
    PassengerNotInfantry,
    AlreadyAboard,
    AtCapacity,
    NoPassengers,
    CellFull,
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTransportIndex => write!(f, "invalid transport unit index"),
            Self::NotATransport => write!(f, "unit is not a transport (APC/Chinook)"),
            Self::TransportNotAlive => write!(f, "transport is not alive"),
            Self::InvalidPassengerIndex => write!(f, "invalid transport unit index"),
            Self::PassengerNotAlive => write!(f, "passenger is not alive"),
            Self::PassengerNotInfantry => write!(f, "passenger is not infantry"),
            Self::AlreadyAboard => write!(f, "unit is already aboard this transport"),
            Self::AtCapacity => write!(f, "transport cargo hold is full"),
            Self::NoPassengers => write!(f, "transport has no passengers"),
            Self::CellFull => write!(f, "target cell has no free sub-cell slot"),
        }
    }
}

impl std::error::Error for TransportError {}

impl Game {
    /// `true` if `unit_index` is already a passenger aboard any transport.
    /// Derived state: no separate aboard flag is stored.
    fn is_passenger_aboard(&self, unit_index: usize) -> bool {
        self.units
            .iter()
            .filter(|u| u.is_transport())
            .any(|t| t.passengers.contains(&unit_index))
    }

    /// CARGO.CPP semantics: board an infantry passenger onto a transport.
    ///
    /// LIFO boarding order (CargoClass::Attach). The passenger must be alive
    /// infantry, not already aboard, and the transport must have cargo space.
    /// On success the passenger is added to the transport's passenger list,
    /// its position is moved into the transport, and its sub-cell is cleared
    /// (it is no longer on the map).
    pub fn load_transport_at(
        &mut self,
        transport_idx: usize,
        passenger_idx: usize,
    ) -> Result<(), TransportError> {
        let transport = self
            .units
            .get(transport_idx)
            .ok_or(TransportError::InvalidTransportIndex)?;
        if !transport.is_transport() {
            return Err(TransportError::NotATransport);
        }
        if !transport.is_alive() {
            return Err(TransportError::TransportNotAlive);
        }

        let passenger = self
            .units
            .get(passenger_idx)
            .ok_or(TransportError::InvalidPassengerIndex)?;
        if !passenger.is_alive() {
            return Err(TransportError::PassengerNotAlive);
        }
        if !passenger.is_infantry() {
            return Err(TransportError::PassengerNotInfantry);
        }
        if transport.has_passenger(passenger_idx) {
            return Err(TransportError::AlreadyAboard);
        }
        if self.is_passenger_aboard(passenger_idx) {
            return Err(TransportError::AlreadyAboard);
        }
        if !transport.has_cargo_space() {
            return Err(TransportError::AtCapacity);
        }

        // Indexes were validated above with `.get()`, so direct indexing is safe.
        let transport_pos = self.units[transport_idx].position;
        self.units[transport_idx].passengers.push(passenger_idx);

        let passenger = &mut self.units[passenger_idx];
        passenger.position = transport_pos;
        passenger.set_sub_cell(None);

        Ok(())
    }

    /// CARGO.CPP semantics: unload all passengers from a transport at `cell`.
    ///
    /// Passengers exit in LIFO order (last aboard, first out). Each passenger
    /// is assigned a sub-cell slot at the target cell via
    /// [`Game::assign_infantry_sub_cell`]. If the cell is full, no passengers
    /// are unloaded and `CellFull` is returned.
    pub fn unload_transport_at(
        &mut self,
        transport_idx: usize,
        cell: Vec2i,
    ) -> Result<(), TransportError> {
        // Validate transport and clone passenger indices (immutable borrows only).
        let passengers = {
            let transport = self
                .units
                .get(transport_idx)
                .ok_or(TransportError::InvalidTransportIndex)?;
            if !transport.is_transport() {
                return Err(TransportError::NotATransport);
            }
            if !transport.is_alive() {
                return Err(TransportError::TransportNotAlive);
            }
            if transport.passengers.is_empty() {
                return Err(TransportError::NoPassengers);
            }
            let existing = self.infantry_in_cell(cell).len();
            if existing + transport.passengers.len() > 5 {
                return Err(TransportError::CellFull);
            }
            transport.passengers.clone()
        };

        // Clear the transport's passenger list.
        {
            let transport = self
                .units
                .get_mut(transport_idx)
                .ok_or(TransportError::InvalidTransportIndex)?;
            transport.passengers.clear();
        }

        // Unload in LIFO order: last aboard, first out.
        for &passenger_idx in passengers.iter().rev() {
            let passenger = self.units.get(passenger_idx);
            let Some(passenger) = passenger else {
                continue;
            };
            if !passenger.is_alive() || !passenger.is_infantry() {
                continue;
            }

            self.assign_infantry_sub_cell(passenger_idx, cell)
                .map_err(|_| TransportError::CellFull)?;
        }

        Ok(())
    }

    /// Kill all passengers aboard transports that have been destroyed.
    ///
    /// Called from [`Game::update`] each tick so that passengers die with
    /// their transport (CARGO.CPP semantics).
    pub fn kill_passengers_of_destroyed_transports(&mut self) {
        let destroyed_transports: Vec<(usize, Vec<usize>)> = self
            .units
            .iter()
            .enumerate()
            .filter_map(|(idx, unit)| {
                if unit.is_transport() && !unit.is_alive() && !unit.passengers.is_empty() {
                    let passengers = unit.passengers.clone();
                    Some((idx, passengers))
                } else {
                    None
                }
            })
            .collect();

        for (transport_idx, passengers) in destroyed_transports {
            // Clear the transport's passenger list first.
            if let Some(transport) = self.units.get_mut(transport_idx) {
                transport.passengers.clear();
            }

            // Kill each passenger that is still alive.
            for &passenger_idx in &passengers {
                if let Some(passenger) = self.units.get_mut(passenger_idx) {
                    if passenger.is_alive() {
                        passenger.health = Fixed16_16::ZERO;
                    }
                }
            }
        }
    }

    /// Sync every living passenger's position to its transport's position so
    /// cargo rides with the transport as it moves (CARGO.CPP: cargo moves with
    /// the holding unit).
    pub(crate) fn sync_passenger_positions(&mut self) {
        let transport_data: Vec<(usize, cnc_core::Vec2)> = self
            .units
            .iter()
            .enumerate()
            .filter(|(_, u)| u.is_transport() && u.is_alive())
            .map(|(i, u)| (i, u.position))
            .collect();
        for (transport_index, position) in transport_data {
            let passengers: Vec<usize> = self.units[transport_index].passengers.clone();
            for passenger_index in passengers {
                if let Some(passenger) = self.units.get_mut(passenger_index) {
                    passenger.position = position;
                }
            }
        }
    }

    /// Game-side arm of `Command::Board`: validate ownership and load a
    /// passenger onto a transport. Returns true when the order had effect.
    pub fn apply_board(&mut self, player: u8, transport_idx: usize, passenger_idx: usize) -> bool {
        let transport = match self.units.get(transport_idx) {
            Some(t) if t.owner == player && t.is_transport() && t.is_alive() => t,
            _ => return false,
        };
        if transport.has_passenger(passenger_idx) {
            return false;
        }
        self.load_transport_at(transport_idx, passenger_idx).is_ok()
    }

    /// Game-side arm of `Command::Unload`: validate ownership and unload
    /// all passengers from a transport at its current cell. Returns true when
    /// the order had effect.
    pub fn apply_unload(&mut self, player: u8, transport_idx: usize, _slot: u8) -> bool {
        let transport = match self.units.get(transport_idx) {
            Some(t) if t.owner == player && t.is_transport() && t.is_alive() => t,
            _ => return false,
        };
        if transport.passengers.is_empty() {
            return false;
        }
        let cell = Self::unit_cell(transport);
        self.unload_transport_at(transport_idx, cell).is_ok()
    }
}

/// Short-lived superweapon blast visual.
#[derive(Clone, Copy, Debug)]
pub struct Blast {
    pub position: Vec2,
    pub radius: f32,
    pub ttl: Fixed16_16,
    pub max_ttl: Fixed16_16,
    pub color: [f32; 4],
}

/// `HOUSE.CPP` `SpeakAttackDelay`: the remaster sets 35 s
/// (`TICKS_PER_MINUTE/2 + TICKS_PER_SECOND*5`), not the original 2 minutes.
const BASE_ATTACK_SPEAK_DELAY: f32 = 35.0;
/// `HOUSE.CPP` `SpeakMoneyDelay` for the "insufficient funds" line: 1 minute.
const INSUFFICIENT_FUNDS_SPEAK_DELAY: f32 = 60.0;
/// `HOUSE.CPP` threshold below which `VOX_NO_CASH` fires.
const INSUFFICIENT_FUNDS_THRESHOLD: i32 = 100;

/// `ACTION_AIRSTRIKE` damage per strafing pass (ANIM.CPP strafing run).
const AIRSTRIKE_DAMAGE: i32 = 200;
/// `ACTION_NUKE` blast radius, matching `fire_nuke`'s 4-cell reach.
const NUKE_RADIUS_PX: f32 = TILE_SIZE * 4.0;
/// `ACTION_NUKE` centre-cell damage (ANIM.CPP ANIM_ATOM_BLAST 1000 / 3).
const NUKE_DAMAGE: i32 = 333;
/// `ACTION_ION` damage, matching `fire_ion_cannon` (ANIM_ION_CANNON).
const ION_DAMAGE: i32 = 600;
/// Infantry `ACTION_DZ` lands on a 3-wide block.
const DROPZONE_UNITS: usize = 6;
/// Alpha of the fog drawn over explored-but-not-visible cells; unexplored
/// cells are drawn fully black.
pub const SHROUD_MEMORY_ALPHA: f32 = 0.55;

/// Missile launch animation from a structure to the target.
#[derive(Clone, Copy, Debug)]
pub struct Strike {
    pub from: Vec2,
    pub to: Vec2,
    pub progress: Fixed16_16,
    pub duration: Fixed16_16,
}

/// Ion Cannon beam visual.
#[derive(Clone, Copy, Debug)]
pub struct Beam {
    pub position: Vec2,
    pub ttl: Fixed16_16,
    pub max_ttl: Fixed16_16,
    pub width: f32,
}

/// Damage scheduled with a short delay for staggered per-cell blasts.
pub(crate) struct PendingBlast {
    position: Vec2,
    radius: f32,
    damage: i32,
    warhead: WarheadType,
    delay: Fixed16_16,
    color: [f32; 4],
}

/// Snapshot of HUD-relevant game state, owned by the game rather than the UI.
#[derive(Clone, Debug, Default)]
pub struct GameStatus {
    pub mission: String,
    pub units: usize,
    pub buildings: usize,
    pub selected: usize,
    pub gdi_credits: i32,
    pub nod_credits: i32,
    pub production: usize,
    pub gdi_power: i32,
    pub gdi_drain: i32,
    pub nod_power: i32,
    pub nod_drain: i32,
    pub gdi_tiberium: i32,
    pub gdi_capacity: i32,
    pub nod_tiberium: i32,
    pub nod_capacity: i32,
    pub gdi_ion: i32,
    pub gdi_nuke: i32,
    pub nod_ion: i32,
    pub nod_nuke: i32,
    pub ready_buildings: usize,
    /// Campaign objectives shown in the HUD.
    pub objectives_total: usize,
    pub objectives_done: usize,
    /// True once a `win`/`lose` action resolved the mission.
    pub mission_over: bool,
    pub mission_victory: bool,
}

/// A type the player can put into a production queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buildable {
    Unit(UnitType),
    Structure(StructureType),
}

impl Buildable {
    pub const fn ini_name(self) -> &'static str {
        match self {
            Self::Unit(unit) => unit.ini_name(),
            Self::Structure(structure) => structure.ini_name(),
        }
    }

    pub const fn class_id(self) -> ClassId {
        match self {
            Self::Unit(unit) => unit_class_id(unit),
            Self::Structure(structure) => structure.class_id(),
        }
    }
}

/// One row of the production sidebar, including why it is hidden or disabled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidebarOption {
    pub buildable: Buildable,
    /// Factory category (infantry, vehicle, aircraft or building).
    pub category: ProductionKind,
    pub cost: i32,
    /// Rules.ini TechLevel override or TDATA/BDATA fallback.
    pub tech_level: u8,
    /// False when TechLevel or prerequisites keep the row out of the sidebar.
    pub visible: bool,
    /// True when the house currently has enough credits.
    pub affordable: bool,
    /// True when the matching factory (or Construction Yard) is alive.
    pub has_producer: bool,
    /// Unmet Prerequisite= tokens, for tooltips and diagnostics.
    pub missing_prerequisites: Vec<String>,
}

/// The playable game session: simulation state plus rendering entry point.
/// HOUSE.CPP SourceType: where a house reinforcements arrive from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ReinforcementEdge {
    #[default]
    North,
    East,
    South,
    West,
    /// Dropped anywhere on the map.
    Air,
    /// Dropped where the player can see.
    Visible,
    /// Dropped at the enemy base.
    EnemyBase,
    /// Dropped at a friendly base.
    HomeBase,
}

impl ReinforcementEdge {
    /// Parse a [Houses] Edge= token (Name_From_Source spelling).
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name.trim().to_ascii_uppercase().as_str() {
            "NORTH" | "N" => Self::North,
            "EAST" | "E" => Self::East,
            "SOUTH" | "S" => Self::South,
            "WEST" | "W" => Self::West,
            "AIR" => Self::Air,
            "VISIBLE" => Self::Visible,
            "ENEMYBASE" => Self::EnemyBase,
            "HOMEBASE" => Self::HomeBase,
            "" => Self::North,
            _ => return None,
        })
    }
}
pub struct Game {
    pub map: Map,
    pub houses: Vec<House>,
    /// House controlled by the local player (mission `Player`, default GDI).
    pub player_owner: u8,
    pub units: Vec<Unit>,
    pub buildings: Vec<Building>,
    pub mission: Mission,
    pub elapsed: Fixed16_16,
    pub patrol_center: Vec2,
    pub patrol_radius: Fixed16_16,
    pub projectiles: Vec<Projectile>,
    /// Total projectiles spawned this session (useful for tests/HUD).
    pub projectiles_fired: u64,
    /// Ticks since the last tiberium growth cycle.
    pub tiberium_tick: u32,
    /// Units completed through production (for HUD/logging).
    pub units_produced: u64,
    /// Total credits delivered by harvesters (for HUD/logging).
    pub harvested_credits: i64,
    /// Completed structures waiting to be placed on the map.
    pub ready_buildings: Vec<(u8, StructureType)>,
    /// Aircraft shots fired (telemetry for tests/HUD).
    pub aircraft_shots: u64,
    /// Aircraft rearm steps completed at helipads.
    pub aircraft_rearmed: u64,
    /// Shots fired with a secondary weapon (telemetry).
    pub secondary_shots: u64,
    /// CRATE.CPP crate powerups (deterministic spawns + drive-over pickups).
    pub powerups: PowerupState,
    /// Per-house expert AI state (RA `Expert_AI` port; index == house index).
    pub ai: Vec<AI>,
    /// Active superweapon blasts (visual only).
    pub blasts: Vec<Blast>,
    /// Missile launches in flight (visual; damage is scheduled separately).
    pub strikes: Vec<Strike>,
    /// Ion Cannon beams (visual).
    pub beams: Vec<Beam>,
    /// Optional Rules.ini overrides. An empty database falls back to the
    /// hard-coded TD stats ported from UDATA/BDATA/CONST.CPP.
    pub rules: RulesDatabase,
    /// Per-house fog of war / shroud (`CELL.H` IsVisible/IsMapped).
    pub shroud: cnc_core::Shroud,
    /// `[TeamTypes]` consumer state. Empty unless a scenario defines teams.
    pub team_manager: crate::ai::TeamManager,
    /// EVA speech queue (`Speak`/`Speak_AI`). Game logic posts events here;
    /// the audio layer drains it.
    pub eva: cnc_audio::EvaQueue,
    /// Per-house low-power announcement latch, so EVA speaks on the
    /// transition into low power rather than every recalculation.
    pub(crate) low_power_announced: Vec<bool>,
    /// Cumulative kills per (house, asset class). Fed by the death
    /// announcement scan, which already samples every alive-to-dead edge.
    pub kills: std::collections::HashMap<(u8, crate::mission::Eliminated), i64>,
    /// Cells newly explored by the player since the last trigger evaluation.
    /// `EVENT_DISCOVERED` reads this; it is drained each tick so a trigger
    /// fires once per discovery rather than every frame thereafter.
    pub discovered_cells: std::collections::HashSet<Vec2i>,
    /// Units/buildings whose destruction has already been announced.
    pub(crate) dead_units_announced: Vec<usize>,
    pub(crate) dead_buildings_announced: Vec<usize>,
    /// Elapsed time when "base under attack" may next be spoken.
    pub(crate) next_base_attack_speak: Fixed16_16,
    /// Elapsed time when "insufficient funds" may next be spoken.
    pub(crate) next_funds_speak: Fixed16_16,
    pub(crate) pending_blasts: Vec<PendingBlast>,
    /// Observational event sink; `None` unless a host attached a bus.
    pub(crate) event_source: Option<cnc_events::EventSource>,
    /// Latch so the mission outcome is published exactly once.
    pub(crate) mission_outcome_emitted: bool,
}

impl Game {
    pub fn new(map_name: &str, width: u32, height: u32) -> Self {
        let mut map = Map::new(map_name.to_string(), width, height);

        // Deterministic tiberium patches so the demo has resource fields.
        for y in 0..height {
            for x in 0..width {
                if ((x / 3) + (y / 3) + x) % 13 == 0 {
                    map.grid
                        .set_tiberium(Vec2i::new(x as i32, y as i32), 160, 1);
                }
            }
        }

        Self {
            map,
            houses: Vec::new(),
            player_owner: 0,
            units: Vec::new(),
            buildings: Vec::new(),
            mission: Mission::new(map_name.to_string()),
            elapsed: Fixed16_16::ZERO,
            patrol_center: Self::cell_center(26, 26),
            patrol_radius: Fixed16_16::from_int(80),
            projectiles: Vec::new(),
            projectiles_fired: 0,
            tiberium_tick: 0,
            units_produced: 0,
            harvested_credits: 0,
            ready_buildings: Vec::new(),
            aircraft_shots: 0,
            aircraft_rearmed: 0,
            secondary_shots: 0,
            powerups: PowerupState::new(),
            ai: Vec::new(),
            blasts: Vec::new(),
            strikes: Vec::new(),
            beams: Vec::new(),
            rules: RulesDatabase::default(),
            // Skirmish starts fully revealed; missions may install a shrouded
            // grid via `enable_shroud`.
            shroud: cnc_core::Shroud::fully_revealed(width, height),
            team_manager: crate::ai::TeamManager::default(),
            eva: cnc_audio::EvaQueue::new(),
            low_power_announced: Vec::new(),
            kills: std::collections::HashMap::new(),
            discovered_cells: std::collections::HashSet::new(),
            dead_units_announced: Vec::new(),
            dead_buildings_announced: Vec::new(),
            next_base_attack_speak: Fixed16_16::ZERO,
            next_funds_speak: Fixed16_16::ZERO,
            pending_blasts: Vec::new(),
            // Observational only: no bus until a host calls set_event_bus.
            event_source: None,
            mission_outcome_emitted: false,
        }
    }

    /// Start with fog of war: nothing revealed until `refresh_shroud` runs.
    pub fn enable_shroud(&mut self) {
        self.shroud = cnc_core::Shroud::new(self.map.width, self.map.height);
    }

    /// Recompute every house's current sight from its units and buildings.
    ///
    /// Clears last tick's visibility first, then reveals a square of
    /// `sight_range` cells around each live object. Mapped history persists.
    /// Sight radius for a unit: Rules.ini `Sight=` when present, else the
    /// built-in UDATA stats table.
    pub fn unit_sight_range(&self, unit: &Unit) -> i32 {
        let name = unit.unit_type.ini_name();
        self.rules
            .type_rule(name)
            .and_then(|rule| rule.sight)
            .map(|sight| sight.clamp(0, cnc_core::shroud::MAX_SIGHT_RADIUS))
            .unwrap_or(unit.unit_type.stats().sight_range)
    }

    /// Sight radius for a building, under the same rules-first rule.
    pub fn structure_sight_range(&self, building: &Building) -> i32 {
        let name = building.structure.ini_name();
        self.rules
            .type_rule(name)
            .and_then(|rule| rule.sight)
            .map(|sight| sight.clamp(0, cnc_core::shroud::MAX_SIGHT_RADIUS))
            .unwrap_or(building.structure.stats().sight_range)
    }
    pub fn refresh_shroud(&mut self) {
        // Snapshot the mapped history so the cells that become mapped during
        // this refresh can be reported as new discoveries (EVENT_DISCOVERED).
        let before = self.shroud.clone();
        self.shroud.clear_visible();

        let sight: Vec<(u8, cnc_core::Vec2i, i32)> = self
            .units
            .iter()
            .filter(|unit| unit.is_alive())
            .map(|unit| {
                // Rules.ini Sight= wins over the built-in stats table.
                let range = self.unit_sight_range(unit);
                (unit.owner, Self::world_cell(unit.position), range)
            })
            .chain(self.buildings.iter().filter(|b| b.is_alive()).map(|b| {
                let range = self.structure_sight_range(b);
                (b.owner, Self::world_cell(b.position), range)
            }))
            .collect();

        for (house, cell, range) in sight {
            self.shroud.reveal(usize::from(house), cell, range);
        }
        self.record_discoveries(&before);
    }

    /// May `house` target `cell` right now? (Used by scoring/targeting rules.)
    pub fn can_see(&self, house: u8, cell: cnc_core::Vec2i) -> bool {
        self.shroud.can_target(usize::from(house), cell)
    }

    /// Build a game with Rules.ini overrides applied.
    pub fn with_rules(map_name: &str, width: u32, height: u32, rules: RulesDatabase) -> Self {
        let mut game = Self::new(map_name, width, height);
        game.rules = rules;
        game
    }

    /// Replace the active rules database (for hot-reload or mission setup).
    /// Existing units immediately pick up the new weapon/warhead values.
    pub fn set_rules(&mut self, rules: RulesDatabase) {
        self.rules = rules;
        self.refresh_weapon_rules();
    }

    /// Load Rules.ini from disk and apply it to this game.
    pub fn load_rules(&mut self, path: &std::path::Path) -> anyhow::Result<usize> {
        let rules = RulesDatabase::load(path)?;
        let count = rules.type_count();
        self.set_rules(rules);
        Ok(count)
    }

    /// Cost of a unit, honouring a Rules.ini override when present.
    pub fn unit_cost(&self, unit_type: UnitType) -> i32 {
        self.rules
            .type_cost(unit_type.ini_name())
            .unwrap_or_else(|| unit_type.stats().cost)
    }

    /// Cost of a structure, honouring a Rules.ini override when present.
    pub fn building_cost(&self, structure: StructureType) -> i32 {
        self.rules
            .type_cost(structure.ini_name())
            .unwrap_or_else(|| structure.stats().cost)
    }

    /// Maximum health of a unit, honouring a Rules.ini override when present.
    pub fn unit_strength(&self, unit_type: UnitType) -> i32 {
        self.rules
            .type_strength(unit_type.ini_name())
            .unwrap_or_else(|| unit_type.stats().strength)
    }

    /// Maximum health of a structure, honouring a Rules.ini override when present.
    pub fn building_strength(&self, structure: StructureType) -> i32 {
        self.rules
            .type_strength(structure.ini_name())
            .unwrap_or_else(|| structure.stats().strength)
    }

    /// Build a weapon with the current Rules.ini Damage/ROF/Range/Warhead data.
    pub fn make_weapon(&self, weapon_type: WeaponType) -> Weapon {
        Weapon::from_type_with_rules(weapon_type, &self.rules)
    }

    /// Warhead profile for explosions, honouring Rules.ini Verses/Spread.
    pub fn warhead_profile(&self, warhead: WarheadType) -> Warhead {
        match self.rules.warhead(warhead.ini_name()) {
            Some(rule) => Warhead::from_rule(warhead, rule),
            None => Warhead::from_type(warhead),
        }
    }

    /// Rebuild every unit's weapons from the current rules database.
    pub fn refresh_weapon_rules(&mut self) {
        let rules = self.rules.clone();
        for unit in &mut self.units {
            unit.apply_weapon_rules(&rules);
        }
    }

    /// Does the house own a living structure of this type?
    pub fn owns_structure(&self, owner: u8, structure: StructureType) -> bool {
        self.buildings.iter().any(|building| {
            building.owner == owner
                && building.is_alive()
                && building.structure_type() == Some(structure)
        })
    }

    /// Every distinct living structure owned by a house, for prerequisites.
    pub fn owned_structures(&self, owner: u8) -> Vec<StructureType> {
        let mut owned = Vec::new();
        for building in &self.buildings {
            if building.owner != owner || !building.is_alive() {
                continue;
            }
            if let Some(structure) = building.structure_type() {
                if !owned.contains(&structure) {
                    owned.push(structure);
                }
            }
        }
        owned
    }

    fn is_gdi(&self, owner: u8) -> bool {
        self.houses
            .get(owner as usize)
            .is_some_and(|house| house.id == ClassId::HouseGDI)
    }

    /// Rules.ini Prerequisite= enforcement (RULES.CPP / TYPE.CPP tech tree).
    ///
    /// A type with no rules entry, or an empty prerequisite list, is always
    /// allowed. Unknown tokens do not pass, matching the original engine.
    pub fn meets_prerequisites(&self, owner: u8, type_name: &str) -> bool {
        let Some(rule) = self.rules.type_rule(type_name) else {
            return true;
        };
        let is_gdi = self.is_gdi(owner);
        let owned = self.owned_structures(owner);
        rule.prerequisite
            .iter()
            .all(|token| prerequisite_token_met(token, is_gdi, &owned))
    }

    /// Human-readable list of unmet prerequisite tokens for UI/diagnostics.
    pub fn missing_prerequisites(&self, owner: u8, type_name: &str) -> Vec<String> {
        let Some(rule) = self.rules.type_rule(type_name) else {
            return Vec::new();
        };
        let is_gdi = self.is_gdi(owner);
        let owned = self.owned_structures(owner);
        rule.prerequisite
            .iter()
            .filter(|token| !prerequisite_token_met(token, is_gdi, &owned))
            .cloned()
            .collect()
    }

    /// Build level of a unit: Rules.ini TechLevel override or TDATA fallback.
    pub fn unit_tech_level(&self, unit_type: UnitType) -> u8 {
        self.rules
            .type_rule(unit_type.ini_name())
            .and_then(|rule| rule.tech_level)
            .map(|level| level.clamp(0, u8::MAX as i32) as u8)
            .unwrap_or(unit_type.stats().tech_level)
    }

    /// Rules.ini `Owner=` gate for a unit, with TD fallback when omitted.
    pub fn unit_owned_by_faction(&self, owner: u8, unit_type: UnitType) -> bool {
        let is_gdi = self.is_gdi(owner);
        match self.rules.type_rule(unit_type.ini_name()) {
            Some(rule) if !rule.owner.is_empty() => owner_token_allows(&rule.owner, is_gdi),
            _ => default_unit_owner(unit_type, is_gdi),
        }
    }

    /// Rules.ini `Owner=` gate for a structure, with TD fallback when omitted.
    pub fn structure_owned_by_faction(&self, owner: u8, structure: StructureType) -> bool {
        let is_gdi = self.is_gdi(owner);
        match self.rules.type_rule(structure.ini_name()) {
            Some(rule) if !rule.owner.is_empty() => owner_token_allows(&rule.owner, is_gdi),
            _ => default_structure_owner(structure, is_gdi),
        }
    }

    /// Build level of a structure: Rules.ini TechLevel or BDATA fallback.
    pub fn building_tech_level(&self, structure: StructureType) -> u8 {
        self.rules
            .type_rule(structure.ini_name())
            .and_then(|rule| rule.tech_level)
            .map(|level| level.clamp(0, u8::MAX as i32) as u8)
            .unwrap_or(structure.stats().tech_level)
    }

    /// HOUSE.CPP Can_Build: the house build level must cover the type level.
    pub fn meets_tech_level(&self, owner: u8, required: u8) -> bool {
        self.houses
            .get(owner as usize)
            .is_some_and(|house| required <= house.tech_level)
    }

    fn cell_center(cx: i32, cy: i32) -> Vec2 {
        Vec2::new(
            Fixed16_16::from_f32(cx as f32 * TILE_SIZE + TILE_SIZE * 0.5),
            Fixed16_16::from_f32(cy as f32 * TILE_SIZE + TILE_SIZE * 0.5),
        )
    }

    fn cell(cx: i32, cy: i32) -> Vec2 {
        Vec2::new(
            Fixed16_16::from_f32(cx as f32 * TILE_SIZE),
            Fixed16_16::from_f32(cy as f32 * TILE_SIZE),
        )
    }

    /// Spawn a small GDI/Nod skirmish so the launcher renders real game state.
    pub fn setup_demo(&mut self) {
        self.houses.clear();
        self.units.clear();
        self.buildings.clear();
        self.projectiles.clear();

        let mut gdi = House::new(ClassId::HouseGDI, "GDI".to_string());
        gdi.add_credits(5000);
        let mut nod = House::new(ClassId::HouseNod, "Nod".to_string());
        nod.add_credits(3000);

        let structures = [
            (StructureType::ConstructionYard, 12, 12, 0u8),
            (StructureType::PowerPlant, 17, 12, 0u8),
            (StructureType::Refinery, 22, 12, 0u8),
            (StructureType::Barracks, 12, 17, 0u8),
            (StructureType::WarFactory, 17, 17, 0u8),
            (StructureType::Helipad, 22, 17, 0u8),
            (StructureType::ConstructionYard, 42, 42, 1u8),
            (StructureType::PowerPlant, 47, 42, 1u8),
            (StructureType::Refinery, 42, 47, 1u8),
        ];

        for (structure, cx, cy, owner) in structures {
            let mut building = Building::from_type(structure, Self::cell(cx, cy), owner);
            building.set_powered(true);
            let id = self.buildings.len() as u32;
            if owner == 0 {
                gdi.add_building(id);
            } else {
                nod.add_building(id);
            }
            self.buildings.push(building);
        }

        for index in 0..12 {
            let angle = index as f32 * 0.55;
            let cx = 26.0 + angle.cos() * 7.0;
            let cy = 26.0 + angle.sin() * 7.0;
            let unit_type = match index {
                4 => UnitType::MammothTank,
                _ if index % 4 == 0 => UnitType::MediumTank,
                _ => UnitType::InfantryE1,
            };
            let position = Vec2::new(
                Fixed16_16::from_f32(cx * TILE_SIZE),
                Fixed16_16::from_f32(cy * TILE_SIZE),
            );
            let mut unit = Unit::new(unit_type, position, 0);
            unit.is_selected = index == 0;
            let id = self.units.len() as u32;
            gdi.add_unit(id);
            self.units.push(unit);
        }

        // A dense field next to the refinery keeps the demo economy active.
        for (x, y) in [(19i32, 22i32), (20, 22), (21, 22), (20, 23), (21, 21)] {
            self.map.grid.set_tiberium(Vec2i::new(x, y), 255, 1);
        }

        // One harvester starts the GDI economy loop.
        let harvester = Unit::new(UnitType::Harvester, Self::cell_center(20, 22), 0);
        let id = self.units.len() as u32;
        gdi.add_unit(id);
        self.units.push(harvester);

        // A patrolling Orca showcases the aircraft layer.
        let orca = Unit::new(UnitType::Orca, Self::cell_center(28, 20), 0);
        let id = self.units.len() as u32;
        gdi.add_unit(id);
        self.units.push(orca);

        // A small Nod patrol so attack orders have a target. The first one is
        // a grenadier so the demo shows arcing projectiles. Coordinates are
        // resolved relative to the map, because the demo may run on any size
        // (the original hard-coded cells sat off a 32x32 board).
        let max_x = self.map.width.saturating_sub(1) as i32;
        let max_y = self.map.height.saturating_sub(1) as i32;
        let patrol = [
            ((max_x * 33) / 40, (max_y * 24) / 40),
            ((max_x * 35) / 40, (max_y * 27) / 40),
            ((max_x * 32) / 40, (max_y * 29) / 40),
        ];
        for (index, (cx, cy)) in patrol.into_iter().enumerate() {
            let mut unit = Unit::new(UnitType::InfantryE1, Self::cell(cx, cy), 1);
            if index == 0 {
                unit.equip_weapon(Weapon::from_type_with_rules(
                    WeaponType::Grenade,
                    &self.rules,
                ));
            }
            let id = self.units.len() as u32;
            nod.add_unit(id);
            self.units.push(unit);
        }

        self.patrol_center = Self::cell_center(26, 26);
        self.houses.push(gdi);
        self.houses.push(nod);
        self.refresh_house_power();
        self.refresh_weapon_rules();
    }

    /// Recompute each house power from its structures (BDATA.CPP POWER/DRAIN).
    pub fn refresh_house_power(&mut self) {
        // Keep the announcement latch aligned with the house count.
        self.low_power_announced.resize(self.houses.len(), false);
        for (owner, house) in self.houses.iter_mut().enumerate() {
            let mut generated = 0;
            let mut drain = 0;
            let mut capacity = 0;
            for building in &self.buildings {
                if building.owner as usize != owner || !building.is_alive() {
                    continue;
                }
                if let Some(structure) = building.structure_type() {
                    let stats = structure.stats();
                    generated += stats.power;
                    drain += stats.drain;
                    capacity += stats.capacity;
                }
            }
            house.power = generated;
            house.max_power = drain;
            house.capacity = capacity;
            if house.tiberium > house.capacity {
                house.tiberium = house.capacity;
            }
            // HOUSE.CPP announces low power only on the transition into it,
            // not on every recalculation.
            let low_now = house.power < house.max_power;
            if low_now && !self.low_power_announced[owner] && owner as u8 == self.player_owner {
                self.eva.speak(cnc_audio::EvaLine::LowPower);
            }
            self.low_power_announced[owner] = low_now;
        }
    }

    fn unit_factory_kind(unit_type: UnitType) -> ProductionKind {
        let stats = unit_type.stats();
        if stats.is_aircraft {
            ProductionKind::Aircraft
        } else if stats.is_infantry {
            ProductionKind::Infantry
        } else {
            ProductionKind::Unit
        }
    }

    fn has_producer(&self, owner: u8, kind: ProductionKind) -> bool {
        self.buildings.iter().any(|building| {
            building.owner == owner
                && building.is_alive()
                && building
                    .structure_type()
                    .is_some_and(|structure| structure.stats().produces == Some(kind))
        })
    }

    /// HOUSE.CPP `MaxUnit`: has the house room for one more unit?
    ///
    /// Counts live units plus everything still queued, so a burst of orders
    /// cannot overshoot the cap before production lands.
    pub fn within_unit_limit(&self, owner: u8) -> bool {
        let Some(house) = self.houses.get(owner as usize) else {
            return false;
        };
        let alive = self
            .units
            .iter()
            .filter(|unit| unit.owner == owner && unit.is_alive())
            .count() as u32;
        let queued = house.production_queue.len() as u32;
        alive + queued < house.max_units
    }

    /// HOUSE.CPP `MaxBuilding`: room for one more structure?
    pub fn within_building_limit(&self, owner: u8) -> bool {
        let Some(house) = self.houses.get(owner as usize) else {
            return false;
        };
        let alive = self
            .buildings
            .iter()
            .filter(|building| building.owner == owner && building.is_alive())
            .count() as u32;
        let queued = house
            .production_queue
            .iter()
            .filter(|item| item.class_id.is_building())
            .count() as u32;
        alive + queued < house.max_buildings
    }
    /// The price `owner` actually pays for a raw cost.
    ///
    /// FACTORY.CPP:402 (`Balance = object.Cost_Of() * house.CostBias`) and
    /// TECHNO.CPP:4495 (`Raw_Cost() * House->CostBias`) make the purchase price
    /// the difficulty-biased cost, and SIDEBAR.CPP:2292 shows that same biased
    /// figure on the sidebar. An unknown owner pays the raw cost.
    pub fn production_cost(&self, owner: u8, raw_cost: i32) -> i32 {
        self.houses
            .get(owner as usize)
            .map(|house| house.handicap_cost(raw_cost))
            .unwrap_or(raw_cost)
    }

    pub fn can_queue_unit(&self, owner: u8, unit_type: UnitType) -> bool {
        self.within_unit_limit(owner)
            && self.meets_tech_level(owner, self.unit_tech_level(unit_type))
            && self.meets_prerequisites(owner, unit_type.ini_name())
            && self.has_producer(owner, Self::unit_factory_kind(unit_type))
            && self.houses.get(owner as usize).is_some_and(|house| {
                house.can_afford(self.production_cost(owner, self.unit_cost(unit_type)))
            })
    }

    pub fn can_queue_building(&self, owner: u8, structure: StructureType) -> bool {
        self.within_building_limit(owner)
            && self.meets_tech_level(owner, self.building_tech_level(structure))
            && self.meets_prerequisites(owner, structure.ini_name())
            && self.owns_structure(owner, StructureType::ConstructionYard)
            && self.houses.get(owner as usize).is_some_and(|house| {
                house.can_afford(self.production_cost(owner, self.building_cost(structure)))
            })
    }

    /// Full production sidebar for a house.
    ///
    /// Entries stay in this list even when hidden so callers can distinguish
    /// "locked by TechLevel/prerequisites" (visible == false) from "shown but
    /// unaffordable or missing a factory" (visible == true).
    pub fn sidebar_options(&self, owner: u8) -> Vec<SidebarOption> {
        let Some(house) = self.houses.get(owner as usize) else {
            return Vec::new();
        };

        let mut options = Vec::new();
        for unit_type in UnitType::ALL {
            let category = Self::unit_factory_kind(unit_type);
            let tech_level = self.unit_tech_level(unit_type);
            let missing = self.missing_prerequisites(owner, unit_type.ini_name());
            let cost = self.unit_cost(unit_type);
            options.push(SidebarOption {
                buildable: Buildable::Unit(unit_type),
                category,
                cost,
                tech_level,
                visible: tech_level <= house.tech_level
                    && missing.is_empty()
                    && self.unit_owned_by_faction(owner, unit_type),
                affordable: house.can_afford(cost),
                has_producer: self.has_producer(owner, category),
                missing_prerequisites: missing,
            });
        }

        let has_yard = self.owns_structure(owner, StructureType::ConstructionYard);
        for structure in StructureType::ALL {
            let tech_level = self.building_tech_level(structure);
            let missing = self.missing_prerequisites(owner, structure.ini_name());
            let cost = self.building_cost(structure);
            options.push(SidebarOption {
                buildable: Buildable::Structure(structure),
                category: ProductionKind::Building,
                cost,
                tech_level,
                visible: tech_level <= house.tech_level
                    && missing.is_empty()
                    && self.structure_owned_by_faction(owner, structure),
                affordable: house.can_afford(cost),
                has_producer: has_yard,
                missing_prerequisites: missing,
            });
        }

        options
    }

    /// Sidebar rows the player should actually see right now.
    pub fn visible_sidebar_options(&self, owner: u8) -> Vec<SidebarOption> {
        self.sidebar_options(owner)
            .into_iter()
            .filter(|option| option.visible)
            .collect()
    }

    /// Deterministic AI production pick driven by the rules registry.
    pub fn ai_production_choice(&self, owner: u8, ai: &AI, roll: u32) -> Option<UnitType> {
        let house = self.houses.get(owner as usize)?;
        let owned = self.owned_structures(owner);
        let weights = ai.production_weights(
            &self.rules,
            house.tech_level,
            house.id == ClassId::HouseGDI,
            &owned,
            house.available_money(),
        );
        AI::choose_weighted(&weights, roll)
    }

    /// Pick and queue an AI unit, returning the type when it was accepted.
    pub fn ai_queue_production(&mut self, owner: u8, ai: &AI, roll: u32) -> Option<UnitType> {
        let choice = self.ai_production_choice(owner, ai, roll)?;
        if self.queue_unit(owner, choice) {
            Some(choice)
        } else {
            None
        }
    }

    /// Queue a unit at the house that owns the matching factory.
    pub fn queue_unit(&mut self, owner: u8, unit_type: UnitType) -> bool {
        // FACTORY.CPP:402 — the buyer pays Cost_Of() * house.CostBias.
        let raw_cost = self.unit_cost(unit_type);
        let cost = self.production_cost(owner, raw_cost);
        if !self.can_queue_unit(owner, unit_type) {
            // Distinguish "no money" from "no tech": can_queue_unit folds both
            // into a single false, so re-check the purse to pick the line.
            let broke = self
                .houses
                .get(owner as usize)
                .is_none_or(|house| !house.can_afford(cost));
            if broke && owner == self.player_owner {
                self.eva.speak(cnc_audio::EvaLine::InsufficientFunds);
            } else if owner == self.player_owner {
                self.eva.speak(cnc_audio::EvaLine::UnableToBuildMore);
            }
            return false;
        }

        let Some(house) = self.houses.get_mut(owner as usize) else {
            return false;
        };
        if !house.can_afford(cost) {
            return false;
        }

        // TECHNO.CPP Time_To_Build uses the house power fraction; the
        // difficulty bias scales that time the same way it scales the price.
        let seconds = production_seconds(raw_cost, house.power, house.max_power);
        let total_time = house.handicap_time(Fixed16_16::from_f32(seconds));
        if !house.spend_credits(cost) {
            return false;
        }
        house.production_queue.push_back(ProductionItem {
            class_id: unit_class_id(unit_type),
            progress: Fixed16_16::ZERO,
            total_time,
            cost,
        });
        true
    }

    /// Cancel the front production item and refund its cost.
    pub fn cancel_production(&mut self, owner: u8) -> Option<i32> {
        let house = self.houses.get_mut(owner as usize)?;
        let item = house.production_queue.pop_front()?;
        house.add_credits(item.cost);
        Some(item.cost)
    }

    /// Repair a damaged building to full health, paying for the missing health.
    pub fn repair_building(&mut self, owner: u8, index: usize) -> Option<i32> {
        let building = self.buildings.get(index)?;
        if building.owner != owner || !building.is_alive() || building.health >= building.max_health
        {
            return None;
        }
        let structure = building.structure_type()?;
        let missing = (building.max_health - building.health).to_f32()
            / building.max_health.to_f32().max(1.0);
        let cost = ((self.building_cost(structure) as f32 * missing).ceil() as i32).max(1);
        let house = self.houses.get_mut(owner as usize)?;
        if !house.spend_credits(cost) {
            return None;
        }
        if let Some(building) = self.buildings.get_mut(index) {
            building.health = building.max_health;
        }
        Some(cost)
    }

    /// Sell a placed building: refund half its cost and mark it destroyed.
    pub fn sell_building(&mut self, owner: u8, index: usize) -> Option<i32> {
        let building = self.buildings.get(index)?;
        if building.owner != owner || !building.is_alive() {
            return None;
        }
        let structure = building.structure_type()?;
        let refund = self.building_cost(structure) / 2;
        {
            let building = &mut self.buildings[index];
            building.start_selling();
            building.health = Fixed16_16::ZERO;
        }
        if let Some(house) = self.houses.get_mut(owner as usize) {
            house.add_credits(refund);
            house.remove_building(index as u32);
        }
        Some(refund)
    }

    /// Queue a structure at the Construction Yard.
    pub fn queue_building(&mut self, owner: u8, structure: StructureType) -> bool {
        // FACTORY.CPP:402 — the buyer pays Cost_Of() * house.CostBias.
        let raw_cost = self.building_cost(structure);
        let cost = self.production_cost(owner, raw_cost);
        if !self.can_queue_building(owner, structure) {
            let broke = self
                .houses
                .get(owner as usize)
                .is_none_or(|house| !house.can_afford(cost));
            if owner == self.player_owner {
                let line = if broke {
                    cnc_audio::EvaLine::InsufficientFunds
                } else {
                    cnc_audio::EvaLine::UnableToBuildMore
                };
                self.eva.speak(line);
            }
            return false;
        }

        let Some(house) = self.houses.get_mut(owner as usize) else {
            return false;
        };
        if !house.can_afford(cost) {
            if owner == self.player_owner {
                self.eva.speak(cnc_audio::EvaLine::InsufficientFunds);
            }
            return false;
        }
        // TECHNO.CPP Time_To_Build scales with the power fraction, and the
        // difficulty bias scales that time like it scales the price.
        let seconds = production_seconds(raw_cost, house.power, house.max_power);
        let total_time = house.handicap_time(Fixed16_16::from_f32(seconds));
        if !house.spend_credits(cost) {
            return false;
        }
        house.production_queue.push_back(ProductionItem {
            class_id: structure.class_id(),
            progress: Fixed16_16::ZERO,
            total_time,
            cost,
        });
        true
    }

    pub fn ready_building_for(&self, owner: u8) -> Option<StructureType> {
        self.ready_buildings
            .iter()
            .find(|(building_owner, _)| *building_owner == owner)
            .map(|(_, structure)| *structure)
    }

    /// BDATA.CPP Legal_Placement: every foundation cell must be buildable and
    /// free of objects. Rust structures are one cell.
    pub fn can_place_building(&self, cell: Vec2i) -> bool {
        if !self.map.grid.in_bounds(cell.x, cell.y) {
            return false;
        }
        let index = (cell.y as u32 * self.map.grid.width + cell.x as u32) as usize;
        let terrain = &self.map.grid.cells[index];
        if !terrain.is_buildable || terrain.has_tiberium {
            return false;
        }
        if self
            .buildings
            .iter()
            .any(|building| building.is_alive() && Self::building_cell(building) == cell)
        {
            return false;
        }
        if self
            .units
            .iter()
            .any(|unit| unit.is_alive() && Self::unit_cell(unit) == cell)
        {
            return false;
        }
        true
    }

    pub fn place_ready_building(&mut self, owner: u8, cell: Vec2i) -> bool {
        let Some(index) = self
            .ready_buildings
            .iter()
            .position(|(building_owner, _)| *building_owner == owner)
        else {
            return false;
        };
        if !self.can_place_building(cell) {
            return false;
        }

        let (_, structure) = self.ready_buildings.remove(index);
        let mut building = Building::from_type(structure, Self::cell_center(cell.x, cell.y), owner);
        let strength = self.building_strength(structure);
        let max_health = Fixed16_16::from_int(strength);
        building.health = max_health;
        building.max_health = max_health;
        building.set_powered(owner == 0);
        let id = self.buildings.len() as u32;
        self.buildings.push(building);
        if let Some(house) = self.houses.get_mut(owner as usize) {
            house.add_building(id);
        }
        self.refresh_house_power();
        true
    }

    fn update_superweapons(&mut self, delta_time: Fixed16_16) {
        let mut ion_ready_line = false;
        let mut nuke_ready_line = false;
        for (owner, house) in self.houses.iter_mut().enumerate() {
            let ion_ready = self.buildings.iter().any(|building| {
                building.owner as usize == owner
                    && building.is_alive()
                    && building.structure_type() == Some(StructureType::IonCannon)
            });
            let temple_ready = self.buildings.iter().any(|building| {
                building.owner as usize == owner
                    && building.is_alive()
                    && building.structure_type() == Some(StructureType::TempleOfNod)
            });
            let ion_was_ready = house.ion_charge >= Fixed16_16::ONE;
            let nuke_was_ready = house.nuke_charge >= Fixed16_16::ONE;
            house.charge_superweapons(ion_ready, temple_ready, delta_time);
            // HOUSE.CPP passes VOX_ION_READY / VOX_NUKE_AVAILABLE as the
            // "became available" lines: speak on the charging→ready edge only.
            if owner as u8 == self.player_owner {
                if !ion_was_ready && house.ion_charge >= Fixed16_16::ONE {
                    ion_ready_line = true;
                }
                if !nuke_was_ready && house.nuke_charge >= Fixed16_16::ONE {
                    nuke_ready_line = true;
                }
            }
        }
        if ion_ready_line {
            self.eva.speak(cnc_audio::EvaLine::IonCannonReady);
        } else if nuke_ready_line {
            self.eva.speak(cnc_audio::EvaLine::NuclearWeaponAvailable);
        }
    }

    fn update_blasts(&mut self, delta_time: Fixed16_16) {
        let mut index = 0;
        while index < self.pending_blasts.len() {
            if self.pending_blasts[index].delay <= Fixed16_16::ZERO {
                let pending = self.pending_blasts.remove(index);
                self.apply_blast(
                    pending.position,
                    pending.radius,
                    pending.damage,
                    pending.warhead,
                );
                self.blasts.push(Blast {
                    position: pending.position,
                    radius: pending.radius,
                    ttl: Fixed16_16::from_f32(0.5),
                    max_ttl: Fixed16_16::from_f32(0.5),
                    color: pending.color,
                });
            } else {
                self.pending_blasts[index].delay =
                    (self.pending_blasts[index].delay - delta_time).max(Fixed16_16::ZERO);
                index += 1;
            }
        }

        for blast in &mut self.blasts {
            blast.ttl = (blast.ttl - delta_time).max(Fixed16_16::ZERO);
        }
        self.blasts.retain(|blast| blast.ttl > Fixed16_16::ZERO);
    }

    fn update_strikes(&mut self, delta_time: Fixed16_16) {
        for strike in &mut self.strikes {
            if strike.duration > Fixed16_16::ZERO {
                strike.progress =
                    (strike.progress + delta_time / strike.duration).min(Fixed16_16::ONE);
            } else {
                strike.progress = Fixed16_16::ONE;
            }
        }
        self.strikes
            .retain(|strike| strike.progress < Fixed16_16::ONE);

        for beam in &mut self.beams {
            beam.ttl = (beam.ttl - delta_time).max(Fixed16_16::ZERO);
        }
        self.beams.retain(|beam| beam.ttl > Fixed16_16::ZERO);
    }

    /// ANIM.CPP ANIM_ION_CANNON: 600 damage with WARHEAD_PB at the centre.
    pub fn fire_ion_cannon(&mut self, owner: u8, cell: Vec2i) -> bool {
        let has_building = self.buildings.iter().any(|building| {
            building.owner == owner
                && building.is_alive()
                && building.structure_type() == Some(StructureType::IonCannon)
        });
        if !has_building {
            return false;
        }
        let Some(house) = self.houses.get_mut(owner as usize) else {
            return false;
        };
        if house.ion_charge < Fixed16_16::ONE {
            return false;
        }

        house.ion_charge = Fixed16_16::ZERO;
        let center = Self::cell_center(cell.x, cell.y);
        self.apply_blast(
            center,
            leptons_to_px(EXPLOSION_RANGE_LEPTONS),
            600,
            WarheadType::Pb,
        );
        self.beams.push(Beam {
            position: center,
            ttl: Fixed16_16::from_f32(0.5),
            max_ttl: Fixed16_16::from_f32(0.5),
            width: 26.0,
        });
        self.blasts.push(Blast {
            position: center,
            radius: leptons_to_px(EXPLOSION_RANGE_LEPTONS),
            ttl: Fixed16_16::from_f32(0.6),
            max_ttl: Fixed16_16::from_f32(0.6),
            color: [0.60, 0.85, 1.00, 1.0],
        });
        if let Some(source) = self.event_source.as_ref() {
            crate::events::superweapon_fired(
                source,
                owner,
                crate::events::SuperweaponKind::IonCannon,
                cell,
            );
        }
        true
    }

    /// ANIM.CPP ANIM_ATOM_BLAST: 4-cell radius, 1000 raw damage, WARHEAD_FIRE.
    pub fn fire_nuke(&mut self, owner: u8, cell: Vec2i) -> bool {
        let has_temple = self.buildings.iter().any(|building| {
            building.owner == owner
                && building.is_alive()
                && building.structure_type() == Some(StructureType::TempleOfNod)
        });
        if !has_temple {
            return false;
        }
        let launch_site = self
            .buildings
            .iter()
            .find(|building| {
                building.owner == owner
                    && building.is_alive()
                    && building.structure_type() == Some(StructureType::TempleOfNod)
            })
            .map(|building| {
                building.position + Vec2::new(Fixed16_16::from_int(12), Fixed16_16::from_int(12))
            })
            .unwrap_or_else(|| Self::cell_center(cell.x, cell.y));

        let Some(house) = self.houses.get_mut(owner as usize) else {
            return false;
        };
        if house.nuke_charge < Fixed16_16::ONE {
            return false;
        }

        house.nuke_charge = Fixed16_16::ZERO;
        let center = Self::cell_center(cell.x, cell.y);
        let radius = TILE_SIZE * 4.0;

        // Missile launch: the warhead travels from the Temple to the target.
        let dx = center.x.to_f32() - launch_site.x.to_f32();
        let dy = center.y.to_f32() - launch_site.y.to_f32();
        let distance = (dx * dx + dy * dy).sqrt();
        let flight_seconds = (distance / 150.0).clamp(1.0, 4.0);
        let duration = Fixed16_16::from_f32(flight_seconds);
        self.strikes.push(Strike {
            from: launch_site,
            to: center,
            progress: Fixed16_16::ZERO,
            duration,
        });
        self.blasts.push(Blast {
            position: launch_site,
            radius: 14.0,
            ttl: Fixed16_16::from_f32(0.4),
            max_ttl: Fixed16_16::from_f32(0.4),
            color: [1.0, 0.70, 0.30, 1.0],
        });

        // ANIM.CPP ANIM_ATOM_BLAST: 9x9 cells, 1000 / ((4/2)+1) = 333 damage
        // per cell with WARHEAD_FIRE and Explosion_Damage around each cell.
        // The damage is staggered outward over ~0.15 s like the animation.
        let blast_radius = leptons_to_px(EXPLOSION_RANGE_LEPTONS);
        let per_cell_damage = 1000 / ((4 / 2) + 1);
        for dy in -4..=4 {
            for dx in -4..=4 {
                let target = Vec2i::new(cell.x + dx, cell.y + dy);
                if !self.map.grid.in_bounds(target.x, target.y) {
                    continue;
                }
                let cell_center = Self::cell_center(target.x, target.y);
                let delay = flight_seconds + ((dx.abs() + dy.abs()) as f32 * 0.015).min(0.15);
                self.pending_blasts.push(PendingBlast {
                    position: cell_center,
                    radius: blast_radius,
                    damage: per_cell_damage,
                    warhead: WarheadType::Fire,
                    delay: Fixed16_16::from_f32(delay),
                    color: [1.0, 0.55, 0.2, 1.0],
                });
            }
        }

        self.blasts.push(Blast {
            position: center,
            radius,
            ttl: Fixed16_16::from_f32(1.2),
            max_ttl: Fixed16_16::from_f32(1.2),
            color: [1.0, 0.60, 0.25, 1.0],
        });
        if let Some(source) = self.event_source.as_ref() {
            crate::events::superweapon_fired(
                source,
                owner,
                crate::events::SuperweaponKind::NuclearMissile,
                cell,
            );
        }
        true
    }

    fn apply_blast(&mut self, center: Vec2, radius: f32, damage: i32, warhead: WarheadType) {
        let warhead = self.warhead_profile(warhead);
        for unit in &mut self.units {
            if !unit.is_alive() {
                continue;
            }
            let distance = unit.position.distance(center).to_f32();
            if distance > radius {
                continue;
            }
            let modified = warhead
                .damage(
                    Fixed16_16::from_int(damage),
                    unit.armor,
                    px_to_leptons(distance),
                )
                .int();
            unit.take_damage(Fixed16_16::from_int(modified));
        }

        for building in &mut self.buildings {
            if !building.is_alive() {
                continue;
            }
            let center_position =
                building.position + Vec2::new(Fixed16_16::from_int(12), Fixed16_16::from_int(12));
            let distance = center_position.distance(center).to_f32();
            if distance > radius {
                continue;
            }
            let modified = warhead
                .damage(
                    Fixed16_16::from_int(damage),
                    building.armor,
                    px_to_leptons(distance),
                )
                .int();
            building.health =
                (building.health - Fixed16_16::from_int(modified)).max(Fixed16_16::ZERO);
        }
    }

    /// Announce newly destroyed enemy assets once per frame.
    ///
    /// Mirrors `FOOT.CPP`/`BUILDING.CPP` `Death_Announcement`: the player hears
    /// about losses on their own side (`UNIT_LOST`/`STRUCTURE_LOST`) and about
    /// kills (`ENEMY_UNIT`/`ENEMY_STRUCTURE`). Deaths reach the simulation
    /// through several paths, so this samples the alive→dead transition rather
    /// than hooking every damage site.
    fn update_death_announcements(&mut self) {
        let mut spoken: Option<cnc_audio::EvaLine> = None;
        for index in 0..self.units.len() {
            if self.units[index].is_alive() {
                continue;
            }
            if self.dead_units_announced.contains(&index) {
                continue;
            }
            self.dead_units_announced.push(index);
            let victim = self.units[index].owner;
            if let Some(source) = self.event_source.as_ref() {
                let unit_type = self.units[index].unit_type;
                crate::events::unit_lost(source, victim, unit_type, index);
            }
            let line = if victim == self.player_owner {
                cnc_audio::EvaLine::UnitLost
            } else {
                // Credit the kill to the opposing house.
                let killer = if victim == 0 { 1 } else { 0 };
                *self
                    .kills
                    .entry((killer, crate::mission::Eliminated::Units))
                    .or_insert(0) += 1;
                cnc_audio::EvaLine::EnemyUnitDestroyed
            };
            spoken.get_or_insert(line);
        }
        for index in 0..self.buildings.len() {
            if self.buildings[index].is_alive() {
                continue;
            }
            if self.dead_buildings_announced.contains(&index) {
                continue;
            }
            self.dead_buildings_announced.push(index);
            let victim = self.buildings[index].owner;
            if let Some(source) = self.event_source.as_ref() {
                let structure = self.buildings[index].structure.ini_name().to_string();
                crate::events::structure_lost(source, victim, structure, index);
            }
            let line = if victim == self.player_owner {
                cnc_audio::EvaLine::StructureLost
            } else {
                let killer = if victim == 0 { 1 } else { 0 };
                *self
                    .kills
                    .entry((killer, crate::mission::Eliminated::Buildings))
                    .or_insert(0) += 1;
                cnc_audio::EvaLine::EnemyStructureDestroyed
            };
            spoken.get_or_insert(line);
        }
        if let Some(line) = spoken {
            self.eva.speak(line);
        }
    }

    /// `HOUSE.CPP` `Attacked()`: warn when a player structure is taking damage.
    ///
    /// Gated by the same speech cooldown the remaster uses so a sustained
    /// assault does not spam the line.
    fn announce_base_attack(&mut self) {
        if self.elapsed < self.next_base_attack_speak {
            return;
        }
        let under_attack = self.buildings.iter().any(|building| {
            building.owner == self.player_owner
                && building.is_alive()
                && building.health < building.max_health
        });
        if under_attack {
            self.eva.speak(cnc_audio::EvaLine::BaseUnderAttack);
            self.next_base_attack_speak =
                self.elapsed + Fixed16_16::from_f32(BASE_ATTACK_SPEAK_DELAY);
        }
    }

    /// `HOUSE.CPP` `SpeakMoneyDelay`: warn when the player cannot afford to
    /// build. Requires at least one factory, exactly as the original does.
    fn announce_insufficient_funds(&mut self) {
        if self.elapsed < self.next_funds_speak {
            return;
        }
        let Some(house) = self.houses.get(self.player_owner as usize) else {
            return;
        };
        if house.credits >= INSUFFICIENT_FUNDS_THRESHOLD {
            return;
        }
        let has_factory = self
            .buildings
            .iter()
            .any(|building| building.owner == self.player_owner && building.is_alive());
        if !has_factory {
            return;
        }
        self.eva.speak(cnc_audio::EvaLine::InsufficientFunds);
        self.next_funds_speak = self.elapsed + Fixed16_16::from_f32(INSUFFICIENT_FUNDS_SPEAK_DELAY);
    }

    /// Advance production queues and spawn completed units at a factory.
    // --- Alliance helpers (todo 19) ---
    pub fn ally_houses(&mut self, a: u8, b: u8) -> bool {
        let ai = a as usize;
        let bi = b as usize;
        if ai >= self.houses.len() || bi >= self.houses.len() {
            return false;
        }
        if ai == bi {
            return false;
        }
        self.houses[ai].make_ally(b);
        self.houses[bi].make_ally(a);
        true
    }
    pub fn make_alliance(&mut self, a: u8, b: u8) -> bool {
        self.ally_houses(a, b)
    }
    pub fn set_alliance(&mut self, a: u8, b: u8) -> bool {
        self.ally_houses(a, b)
    }
    /// An ALLY event between two houses: makes the alliance, in both
    /// directions, exactly once.
    ///
    /// HOUSE.CPP:2092 `Make_Ally` is guarded by `!Is_Ally(house)` (:2095), so a
    /// repeat request is a **no-op** — it does not un-ally. There is no toggle
    /// in the original. Breaking an alliance is [`Game::break_alliance`],
    /// matching HOUSE.CPP:2158 `Make_Enemy`, which is bilateral (:2164-2166,
    /// "Making war is a bilateral aaction", :2156).
    ///
    /// Returns false for an out-of-range or self-directed event (a documented
    /// no-op) and true when an alliance is in effect afterwards.
    pub fn apply_ally_event(&mut self, emitter: u8, recipient: u8) -> bool {
        let ei = emitter as usize;
        let ri = recipient as usize;
        if ei >= self.houses.len() || ri >= self.houses.len() {
            return false;
        }
        if ei == ri {
            return false;
        }
        if self.houses[ei].is_ally(recipient) {
            // HOUSE.CPP:2095 — Make_Ally guarded by !Is_Ally; repeat is a no-op.
            return true;
        }
        self.houses[ei].make_ally(recipient);
        self.houses[ri].make_ally(emitter);
        true
    }
    /// Break an alliance between two houses, in both directions.
    ///
    /// HOUSE.CPP:2158-2176: `Allies &= ~(1 << house)` for this house and
    /// `enemy->Allies &= ~(1 << Class->House)` for the other one, i.e. war is
    /// always bilateral. A no-op (returns false) when the pair is not allied.
    pub fn break_alliance(&mut self, a: u8, b: u8) -> bool {
        let ai = a as usize;
        let bi = b as usize;
        if ai >= self.houses.len() || bi >= self.houses.len() || ai == bi {
            return false;
        }
        if !self.houses[ai].is_ally(b) && !self.houses[bi].is_ally(a) {
            return false;
        }
        self.houses[ai].make_enemy(b);
        self.houses[bi].make_enemy(a);
        true
    }
    pub fn ally_event(&mut self, emitter: u8, recipient: u8) -> bool {
        self.apply_ally_event(emitter, recipient)
    }
    pub fn handle_ally_event(&mut self, emitter: u8, recipient: u8) -> bool {
        self.apply_ally_event(emitter, recipient)
    }

    /// Advance credit display roll-up for all houses.
    pub fn tick_credits(&mut self) {
        for h in self.houses.iter_mut() {
            h.tick_credits();
        }
    }
    pub fn credit_tick(&mut self) {
        self.tick_credits();
    }
    pub fn update_credit_display(&mut self) {
        self.tick_credits();
    }
    pub fn tick_credit_display(&mut self) {
        self.tick_credits();
    }
    pub fn advance_credits(&mut self) {
        self.tick_credits();
    }
    pub fn tick_all_credits(&mut self) {
        self.tick_credits();
    }

    pub fn update_production(&mut self, delta_time: Fixed16_16) {
        let mut completed = Vec::new();
        for (owner, house) in self.houses.iter_mut().enumerate() {
            if let Some(item) = house.update_production(delta_time) {
                completed.push((owner as u8, item));
            }
        }

        for (owner, item) in completed {
            if let Some(structure) = crate::td_rules::StructureType::from_class_id(item.class_id) {
                // BUILDING.CPP: a finished structure announces "construction
                // complete" to the player who owns it.
                if owner == self.player_owner {
                    self.eva.speak(cnc_audio::EvaLine::ConstructionComplete);
                }
                if let Some(source) = self.event_source.as_ref() {
                    crate::events::structure_ready(source, owner, structure);
                }
                self.ready_buildings.push((owner, structure));
                continue;
            }
            let Some(unit_type) = unit_type_for_class(item.class_id) else {
                continue;
            };
            let position = self.spawn_position(owner);
            let mut unit = Unit::new(unit_type, position, owner);
            unit.apply_weapon_rules(&self.rules);
            let strength = self.unit_strength(unit_type);
            let max_health = Fixed16_16::from_int(strength);
            unit.health = max_health;
            unit.max_health = max_health;
            let id = self.units.len() as u32;
            if let Some(house) = self.houses.get_mut(owner as usize) {
                house.add_unit(id);
            }
            self.units.push(unit);
            self.units_produced += 1;
            if let Some(source) = self.event_source.as_ref() {
                crate::events::unit_produced(source, owner, unit_type, id);
            }
            // UNIT.CPP: a completed unit announces "unit ready" to its owner.
            if owner == self.player_owner {
                self.eva.speak(cnc_audio::EvaLine::UnitReady);
            }
        }
    }

    /// Map-edge entry point for a house with no factory, per its `Edge=`.
    ///
    /// `Air`, `Visible` and the base-relative modes have no distinct
    /// spawn point of their own, so they fall back to the map centre; only
    /// the four compass edges actually move the spawn.
    pub fn edge_entry_cell(&self, owner: u8) -> (i32, i32) {
        let cx = self.map.width as i32 / 2;
        let cy = self.map.height as i32 / 2;
        let edge = self
            .houses
            .get(owner as usize)
            .map(|house| house.edge)
            .unwrap_or_default();
        match edge {
            ReinforcementEdge::North => (cx, 1),
            ReinforcementEdge::South => (cx, self.map.height as i32 - 2),
            ReinforcementEdge::West => (1, cy),
            ReinforcementEdge::East => (self.map.width as i32 - 2, cy),
            // Air / base-relative drops have no fixed entry point.
            ReinforcementEdge::Air
            | ReinforcementEdge::Visible
            | ReinforcementEdge::EnemyBase
            | ReinforcementEdge::HomeBase => (cx, cy),
        }
    }
    fn spawn_position(&self, owner: u8) -> Vec2 {
        let factory = self.buildings.iter().find(|building| {
            building.owner == owner
                && building.is_alive()
                && building.structure_type().is_some_and(|structure| {
                    matches!(
                        structure.stats().produces,
                        Some(ProductionKind::Infantry)
                            | Some(ProductionKind::Unit)
                            | Some(ProductionKind::Aircraft)
                    )
                })
        });

        let (cell_x, cell_y) = match factory {
            // Clear the factory footprint by two cells.
            Some(building) => {
                let cell = Self::building_cell(building);
                (cell.x + 2, cell.y + 2)
            }
            // With no factory the reinforcement enters from the house's
            // configured [Houses] Edge= (HOUSE.CPP SourceType). No offset:
            // pushing inward would stop it being an edge entry.
            None => self.edge_entry_cell(owner),
        };

        let used: Vec<(i32, i32)> = self
            .occupied_cells(None)
            .iter()
            .map(|cell| (cell.x, cell.y))
            .collect();
        let (x, y) = self.nearby_cell(cell_x, cell_y, &used);
        Self::cell_center(x, y)
    }

    /// Advance the demo simulation.
    pub fn update(&mut self, delta_time: Fixed16_16) {
        self.elapsed += delta_time;

        for unit in &mut self.units {
            if unit.reload > Fixed16_16::ZERO {
                unit.reload = (unit.reload - delta_time).max(Fixed16_16::ZERO);
            }
            if unit.shield > Fixed16_16::ZERO {
                unit.shield = (unit.shield - delta_time).max(Fixed16_16::ZERO);
            }
        }

        self.update_production(delta_time);
        // HOUSE.CPP:1580 — HouseClass::AI drives VisibleCredits.AI(false, this, true)
        // every frame, so the credit tab rolls up gradually toward Available_Money()
        // instead of snapping when money is actually spent or harvested.
        self.tick_credits();
        self.kill_passengers_of_destroyed_transports();
        self.sync_passenger_positions();
        self.update_death_announcements();
        self.announce_base_attack();
        self.announce_insufficient_funds();
        self.update_superweapons(delta_time);
        self.update_blasts(delta_time);
        self.update_strikes(delta_time);
        self.update_harvesters(delta_time);
        self.update_aircraft(delta_time);
        self.update_special_missions(delta_time);
        self.update_defence_combat(delta_time);
        self.update_auto_acquire();
        self.update_facings(delta_time);
        self.update_attacks();

        let t = self.elapsed.to_f32();
        let center_x = self.patrol_center.x.to_f32();
        let center_y = self.patrol_center.y.to_f32();
        let radius = self.patrol_radius.to_f32();
        let player_owner = self.player_owner;

        for (index, unit) in self.units.iter_mut().enumerate() {
            // Player move orders take priority; ordered units never go back
            // to the idle patrol, so they hold their destination.
            if unit.ordered {
                unit.advance_order(delta_time);
                unit.reserved_cell = unit
                    .path
                    .get(unit.path_index)
                    .map(|point| Self::world_cell(*point));
                if unit.is_aircraft {
                    unit.rotor_phase =
                        (unit.rotor_phase + delta_time.to_f32() * 25.0) % std::f32::consts::TAU;
                } else if !unit.is_infantry() {
                    unit.track_phase = (unit.track_phase
                        + delta_time.to_f32() * unit.speed.to_f32() * 0.2)
                        % std::f32::consts::TAU;
                }
                continue;
            }
            // Harvesters run their own mission; only idle player combat units
            // drift on the patrol ring. Enemy units hold their spawn positions.
            if unit.is_harvester || unit.is_aircraft || unit.owner != player_owner {
                continue;
            }
            let phase = t * 0.6 + index as f32 * 0.55;
            unit.position = Vec2::new(
                Fixed16_16::from_f32(center_x + phase.cos() * radius),
                Fixed16_16::from_f32(center_y + phase.sin() * radius),
            );
        }

        self.update_projectiles(delta_time);
        self.update_powerups();
        self.update_house_ais(delta_time);
        self.update_mission(delta_time);

        // MAP.CPP grows/spreads tiberium over time rather than every tick.
        self.tiberium_tick = self.tiberium_tick.wrapping_add(1);
        if self.tiberium_tick.is_multiple_of(30) {
            self.map.grid.tiberium_cycle();
        }

        // Post-step observation: nothing below this line reads simulation
        // state back, so publishing cannot change the tick's outcome.
        self.emit_mission_outcome();
    }

    /// Enable the expert AI for every non-player house (skirmish and
    /// quickmatch convenience; campaign scenarios stay trigger-driven
    /// unless this is called explicitly).
    pub fn enable_enemy_ai(&mut self, difficulty: AIDifficulty) {
        for (index, house) in self.houses.iter_mut().enumerate() {
            if index as u8 != self.player_owner {
                house.ai_enabled = true;
            }
        }
        if self.ai.len() < self.houses.len() {
            for index in self.ai.len()..self.houses.len() {
                self.ai.push(AI::new(index as u8, difficulty));
            }
        }
    }

    /// CRATE.CPP: advance crate spawns and apply drive-over pickups.
    fn update_powerups(&mut self) {
        let pickups = self.powerups.tick(&self.map.grid, &self.units);
        for pickup in pickups {
            match pickup.kind {
                PowerupKind::Credits => {
                    if let Some(house) = self.houses.get_mut(pickup.owner as usize) {
                        house.add_credits(powerup::CREDITS_BONUS);
                    }
                }
                PowerupKind::Unit => {
                    let position = Self::cell_center(pickup.cell.x, pickup.cell.y);
                    let unit = Unit::new(UnitType::InfantryE1, position, pickup.owner);
                    let id = self.units.len() as u32;
                    if let Some(house) = self.houses.get_mut(pickup.owner as usize) {
                        house.add_unit(id);
                    }
                    self.units.push(unit);
                    self.units_produced += 1;
                }
                PowerupKind::Armor => {
                    if let Some(unit) = self.units.get_mut(pickup.unit_index) {
                        unit.shield = Fixed16_16::from_f32(powerup::SHIELD_SECONDS);
                    }
                }
            }
        }
    }

    /// Drive the per-house expert AI (RA `Expert_AI` port). AI state is
    /// indexed parallel to `houses`; only non-player houses with
    /// `ai_enabled` act.
    fn update_house_ais(&mut self, delta_time: Fixed16_16) {
        if self.ai.len() < self.houses.len() {
            for index in self.ai.len()..self.houses.len() {
                self.ai.push(AI::new(index as u8, AIDifficulty::Normal));
            }
        }
        let mut ais = std::mem::take(&mut self.ai);
        for (index, ai) in ais.iter_mut().enumerate() {
            let owner = index as u8;
            if owner == self.player_owner {
                continue;
            }
            let enabled = self
                .houses
                .get(owner as usize)
                .is_some_and(|house| house.ai_enabled);
            if !enabled {
                continue;
            }
            // Use shared house AI with TD config
            run_expert_ai(self, ai, delta_time);
            crate::ai::run_team_manager(self, owner, ai.tick_counter);
        }
        self.ai = ais;
    }

    /// Order AI-owned units to attack an enemy unit (thin pub wrapper over
    /// the private attack dispatcher for `run_expert_ai`).
    /// World position at the centre of a cell index (row-major).
    pub fn cell_center_position(&self, cell: i32) -> Vec2 {
        let width = self.map.width.max(1) as i32;
        let cell = cell.max(0);
        Self::cell_center(cell % width, cell / width)
    }

    /// Place a fresh unit of `unit_type` for `owner` and return its index.
    ///
    /// Used by scenario loading and tests; production goes through
    /// `queue_unit` and the placement path instead.
    pub fn spawn_unit_at(&mut self, owner: u8, unit_type: UnitType, cell: Vec2i) -> u32 {
        let position = Self::cell_center(cell.x, cell.y);
        let unit = Unit::new(unit_type, position, owner);
        let id = self.units.len() as u32;
        if let Some(house) = self.houses.get_mut(owner as usize) {
            house.add_unit(id);
        }
        self.units.push(unit);
        id
    }

    /// Position of one unit by index, if it is alive.
    pub fn unit_position(&self, unit_index: u32) -> Option<Vec2> {
        self.units
            .get(unit_index as usize)
            .filter(|unit| unit.is_alive())
            .map(|unit| unit.position)
    }

    /// `true` when the house has an unassigned, living unit of `class_id`.
    pub fn has_free_unit_of_class(&self, owner: u8, class_id: ClassId) -> bool {
        let claimed = self.team_assigned_units();
        self.units.iter().enumerate().any(|(index, unit)| {
            unit.owner == owner
                && unit.is_alive()
                && unit.class_id == class_id
                && !claimed.contains(&index)
        })
    }

    /// Claim a free unit of `class_id` for a team, returning its index.
    pub fn take_free_unit_of_class(&self, owner: u8, class_id: ClassId) -> Option<usize> {
        let claimed = self.team_assigned_units();
        self.units.iter().enumerate().find_map(|(index, unit)| {
            (unit.owner == owner
                && unit.is_alive()
                && unit.class_id == class_id
                && !claimed.contains(&index))
            .then_some(index)
        })
    }

    /// Units already claimed by an active team.
    fn team_assigned_units(&self) -> std::collections::HashSet<usize> {
        self.team_manager
            .teams
            .iter()
            .filter(|team| team.active && !team.suspended)
            .flat_map(|team| team.members.iter().map(|index| index.0 as usize))
            .collect()
    }

    /// Index of the enemy unit closest to the given own units.
    pub fn nearest_enemy_unit_index(&self, owner: u8, own: &[u32]) -> Option<usize> {
        let anchor = own
            .iter()
            .find_map(|index| self.units.get(*index as usize).map(|unit| unit.position))?;
        let mut best: Option<(usize, Fixed16_16)> = None;
        for (index, unit) in self.units.iter().enumerate() {
            if unit.owner == owner || !unit.is_alive() {
                continue;
            }
            let dx = unit.position.x - anchor.x;
            let dy = unit.position.y - anchor.y;
            let dist2 = dx * dx + dy * dy;
            if best.is_none_or(|(_, previous)| dist2 < previous) {
                best = Some((index, dist2));
            }
        }
        best.map(|(index, _)| index)
    }

    pub fn ai_order_attack(
        &mut self,
        owner: u8,
        unit_indices: &[usize],
        target_index: usize,
    ) -> usize {
        self.order_attack_indices(owner, unit_indices, target_index)
    }

    /// Attach a campaign mission and apply its scenario settings.
    pub fn start_mission(&mut self, mission: Mission) {
        self.player_owner = mission.player;
        if let Some(credits) = mission.starting_credits {
            if let Some(house) = self.houses.get_mut(mission.player as usize) {
                house.credits = credits;
            }
        }
        if let Some(tech) = mission.tech_level {
            for house in &mut self.houses {
                house.tech_level = tech;
            }
        }
        self.mission = mission;
    }

    /// Load a mission INI file and start it.
    pub fn load_mission(&mut self, path: &std::path::Path) -> anyhow::Result<()> {
        let mission = Mission::from_file(path)?;
        self.start_mission(mission);
        Ok(())
    }

    /// Evaluate campaign triggers and execute their actions.
    ///
    /// Conditions are level-triggered and checked every simulation tick; the
    /// trigger list is snapshotted before any action runs so a trigger cannot
    /// observe a half-applied action list. Actions that end the mission stop
    /// the remaining due triggers for this tick.
    pub fn update_mission(&mut self, _delta_time: Fixed16_16) {
        if self.mission.is_over() {
            return;
        }
        let due = self.mission.due(self);
        for id in due {
            let actions = self.mission.fire(id);
            for action in actions {
                self.apply_mission_action(&action);
                if self.mission.is_over() {
                    return;
                }
            }
        }
        // Discoveries are edge events: evaluated once, then cleared so a
        // trigger fires on the discovery rather than on every later tick.
        self.discovered_cells.clear();
    }

    fn apply_mission_action(&mut self, action: &Action) {
        match action {
            Action::Airstrike { cell, .. } => {
                // A strafing run: three blasts walking across the target.
                let center = Self::cell_center(cell.x, cell.y);
                let radius = leptons_to_px(EXPLOSION_RANGE_LEPTONS);
                for step in 0..3 {
                    let offset = (step - 1) as f32 * radius;
                    let point = Vec2::new(
                        center.x + Fixed16_16::from_f32(offset),
                        center.y + Fixed16_16::from_f32(offset),
                    );
                    self.apply_blast(point, radius, AIRSTRIKE_DAMAGE, WarheadType::Ap);
                    self.blasts.push(Blast {
                        position: point,
                        radius,
                        ttl: Fixed16_16::from_f32(0.5),
                        max_ttl: Fixed16_16::from_f32(0.5),
                        color: [1.0, 0.75, 0.25, 1.0],
                    });
                }
                self.aircraft_shots += 1;
            }
            Action::Nuke { cell, .. } => {
                // A scripted nuke ignores the Temple prerequisite.
                let center = Self::cell_center(cell.x, cell.y);
                self.apply_blast(center, NUKE_RADIUS_PX, NUKE_DAMAGE, WarheadType::Fire);
                self.blasts.push(Blast {
                    position: center,
                    radius: NUKE_RADIUS_PX,
                    ttl: Fixed16_16::from_f32(0.8),
                    max_ttl: Fixed16_16::from_f32(0.8),
                    color: [1.0, 0.45, 0.10, 1.0],
                });
            }
            Action::IonCannon { cell, .. } => {
                // Same shape as fire_ion_cannon minus the building/charge gate.
                let center = Self::cell_center(cell.x, cell.y);
                let radius = leptons_to_px(EXPLOSION_RANGE_LEPTONS);
                self.apply_blast(center, radius, ION_DAMAGE, WarheadType::Pb);
                self.beams.push(Beam {
                    position: center,
                    ttl: Fixed16_16::from_f32(0.5),
                    max_ttl: Fixed16_16::from_f32(0.5),
                    width: 26.0,
                });
                self.blasts.push(Blast {
                    position: center,
                    radius,
                    ttl: Fixed16_16::from_f32(0.6),
                    max_ttl: Fixed16_16::from_f32(0.6),
                    color: [0.60, 0.85, 1.00, 1.0],
                });
            }
            Action::Dropzone { owner, cell } => {
                for index in 0..DROPZONE_UNITS {
                    let x = cell.x + (index % 3) as i32;
                    let y = cell.y + (index / 3) as i32;
                    self.spawn_unit_at(*owner, UnitType::InfantryE1, Vec2i::new(x, y));
                }
            }
            Action::CreateTeam { team, .. } => {
                if let Some(team_type) = self
                    .team_manager
                    .types
                    .iter()
                    .find(|candidate| candidate.name.eq_ignore_ascii_case(team))
                    .cloned()
                {
                    let already = self
                        .team_manager
                        .teams
                        .iter()
                        .any(|existing| existing.team_type == team_type.id && existing.active);
                    if !already {
                        let id = self.team_manager.next_team_id;
                        self.team_manager.next_team_id = id.wrapping_add(1);
                        let mut instance = cnc_core::entity::Team::from_type(id, &team_type);
                        instance.active = true;
                        instance.suspended = false;
                        self.team_manager.teams.push(instance);
                    }
                }
            }
            Action::AllToHunt { owner } => {
                for index in 0..self.units.len() {
                    if self.units[index].owner == *owner
                        && self.units[index].is_alive()
                        && self.units[index].weapon.is_some()
                    {
                        self.units[index].target = None;
                        self.units[index].path.clear();
                        self.units[index].path_index = 0;
                        self.units[index].ordered = false;
                    }
                }
            }
            Action::Win => {
                self.eva.speak(cnc_audio::EvaLine::MissionAccomplished);
                self.mission.finish(MissionOutcome::Victory);
            }
            Action::Lose => {
                self.eva.speak(cnc_audio::EvaLine::MissionFailed);
                self.mission.finish(MissionOutcome::Defeat);
            }
            Action::Credits { owner, amount } => {
                if let Some(house) = self.houses.get_mut(*owner as usize) {
                    house.add_credits(*amount);
                }
            }
            Action::SpawnUnits {
                owner,
                unit_type,
                count,
                cell,
            } => {
                for index in 0..*count {
                    // Spread reinforcements over a small block so they do not
                    // all occupy exactly the same cell.
                    let x = cell.x + (index % 3) as i32;
                    let y = cell.y + (index / 3) as i32;
                    let unit = Unit::new(*unit_type, Self::cell_center(x, y), *owner);
                    let entity_id = self.units.len() as u32;
                    if let Some(house) = self.houses.get_mut(*owner as usize) {
                        house.add_unit(entity_id);
                    }
                    self.units.push(unit);
                }
            }
            Action::SpawnStructure {
                owner,
                structure,
                cell,
            } => {
                let mut building =
                    Building::from_type(*structure, Self::cell_center(cell.x, cell.y), *owner);
                building.set_powered(true);
                let entity_id = self.buildings.len() as u32;
                if let Some(house) = self.houses.get_mut(*owner as usize) {
                    house.add_building(entity_id);
                }
                self.buildings.push(building);
                self.refresh_house_power();
            }
            Action::Message(text) => self.mission.messages.push(text.clone()),
            Action::SetFlag(name) => {
                self.mission.flags.insert(name.clone());
            }
            Action::EnableTrigger(id) => self.mission.enable(*id),
            Action::DisableTrigger(id) => self.mission.disable(*id),
            Action::CompleteObjective(id) => self.mission.complete_objective(*id),
            Action::RunScript(name) => {
                if let Some(script) = self.mission.script(name).cloned() {
                    for action in &script.actions {
                        self.apply_mission_action(action);
                        if self.mission.is_over() {
                            return;
                        }
                    }
                }
            }
        }
    }

    /// UNIT.CPP Mission_Harvest: harvest tiberium, then dock at a refinery.
    fn update_harvesters(&mut self, delta_time: Fixed16_16) {
        for index in 0..self.units.len() {
            if !self.units[index].is_harvester || !self.units[index].is_alive() {
                continue;
            }

            if self.units[index].harvest_timer > Fixed16_16::ZERO {
                self.units[index].harvest_timer =
                    (self.units[index].harvest_timer - delta_time).max(Fixed16_16::ZERO);
            }

            let owner = self.units[index].owner;
            let position = self.units[index].position;
            let cell = Self::unit_cell(&self.units[index]);
            if self.units[index].tiberium >= HARVESTER_STEP_COUNT {
                self.units[index].unloading = true;
            }
            let unloading = self.units[index].unloading;
            let timer_ready = self.units[index].harvest_timer <= Fixed16_16::ZERO;

            if unloading {
                let refinery_cell = self
                    .buildings
                    .iter()
                    .find(|building| {
                        building.owner == owner
                            && building.is_alive()
                            && building.structure_type() == Some(StructureType::Refinery)
                    })
                    .map(Self::building_cell);
                let Some(refinery_cell) = refinery_cell else {
                    continue;
                };

                let refinery_center = Self::cell_center(refinery_cell.x, refinery_cell.y);
                let distance = position.distance(refinery_center).to_f32();
                if distance <= TILE_SIZE * 1.5 && timer_ready {
                    self.units[index].tiberium = self.units[index].tiberium.saturating_sub(1);
                    self.units[index].harvest_timer = Fixed16_16::from_f32(0.15);
                    self.units[index].clear_path();
                    self.units[index].target = None;
                    if let Some(house) = self.houses.get_mut(owner as usize) {
                        house.harvested(HARVESTER_STEP_CREDITS);
                    }
                    self.harvested_credits += HARVESTER_STEP_CREDITS as i64;
                    if self.units[index].tiberium == 0 {
                        self.units[index].unloading = false;
                    }
                } else if self.units[index].path.is_empty() && self.units[index].target.is_none() {
                    self.move_unit_to_cell(index, refinery_cell);
                }
                continue;
            }

            let on_tiberium = self
                .map
                .grid
                .get(cell)
                .is_some_and(|cell| cell.has_tiberium);
            if on_tiberium && timer_ready {
                let stage = self
                    .map
                    .grid
                    .get(cell)
                    .map(|cell| tiberium_stage(cell.tiberium_amount))
                    .unwrap_or(0);
                let reducer = (stage % 6) + 1;
                let capacity_left = HARVESTER_STEP_COUNT.saturating_sub(self.units[index].tiberium);
                let levels = reducer.min(capacity_left);
                let reduced = self.map.grid.reduce_tiberium(cell, levels);
                self.units[index].tiberium = self.units[index].tiberium.saturating_add(reduced);
                self.units[index].harvest_timer = Fixed16_16::from_f32(1.0);
                self.units[index].clear_path();
                self.units[index].target = None;
            } else if timer_ready
                && self.units[index].path.is_empty()
                && self.units[index].target.is_none()
            {
                if let Some(target_cell) = self.nearest_tiberium_cell(cell) {
                    self.move_unit_to_cell(index, target_cell);
                }
            }
        }
    }

    /// AIRCRAFT.CPP flight/rearm slice: aircraft with no ammunition return to
    /// an owned helipad and reload one shot every 0.25 s.
    fn update_aircraft(&mut self, delta_time: Fixed16_16) {
        for index in 0..self.units.len() {
            if !self.units[index].is_aircraft || !self.units[index].is_alive() {
                continue;
            }
            if self.units[index].rearm_timer > Fixed16_16::ZERO {
                self.units[index].rearm_timer =
                    (self.units[index].rearm_timer - delta_time).max(Fixed16_16::ZERO);
            }

            let owner = self.units[index].owner;
            let ammo = self.units[index].ammo;
            let max_ammo = self.units[index].max_ammo;
            if max_ammo <= 0 || ammo >= max_ammo {
                continue;
            }

            let helipad = self
                .buildings
                .iter()
                .find(|building| {
                    building.owner == owner
                        && building.is_alive()
                        && building.structure_type() == Some(StructureType::Helipad)
                })
                .map(Self::building_cell);
            let Some(helipad) = helipad else {
                continue;
            };

            let center = Self::cell_center(helipad.x, helipad.y);
            let distance = self.units[index].position.distance(center).to_f32();
            if distance <= TILE_SIZE * 1.5 {
                if self.units[index].rearm_timer <= Fixed16_16::ZERO {
                    self.units[index].ammo = (self.units[index].ammo + 1).min(max_ammo);
                    self.units[index].rearm_timer = Fixed16_16::from_f32(0.25);
                    self.aircraft_rearmed += 1;
                    self.units[index].clear_path();
                    self.units[index].target = None;
                }
            } else if ammo == 0
                && self.units[index].path.is_empty()
                && self.units[index].target.is_none()
            {
                self.move_unit_to_cell(index, helipad);
            }
        }
    }

    fn nearest_tiberium_cell(&self, from: Vec2i) -> Option<Vec2i> {
        let width = self.map.width as i32;
        let height = self.map.height as i32;
        let mut best: Option<(Vec2i, i32)> = None;
        for y in 0..height {
            for x in 0..width {
                let cell = self.map.grid.get(Vec2i::new(x, y))?;
                if !cell.has_tiberium {
                    continue;
                }
                let distance = (x - from.x).abs() + (y - from.y).abs();
                if best.is_none_or(|(_, current)| distance < current) {
                    best = Some((Vec2i::new(x, y), distance));
                }
            }
        }
        best.map(|(cell, _)| cell)
    }

    fn move_unit_to_cell(&mut self, index: usize, goal: Vec2i) {
        let path = self.path_to_cell(index, goal);
        let reserved = path.first().map(|point| Self::world_cell(*point));
        let destination = Self::cell_center(goal.x, goal.y);
        let unit = &mut self.units[index];
        unit.set_path(path);
        unit.reserved_cell = reserved;
        unit.target = Some(destination);
        unit.ordered = true;
    }

    /// FOOT.CPP Mission_Guard / Mission_Hunt: idle units scan for a threat.
    ///
    /// Human (GDI) units use THREAT_RANGE (twice weapon range, clamped to
    /// 0x0A00 leptons); AI units use THREAT_NORMAL (weapon range only).
    fn update_auto_acquire(&mut self) {
        for index in 0..self.units.len() {
            if !self.units[index].is_alive()
                || self.units[index].ordered
                || self.units[index].attack_target.is_some()
            {
                continue;
            }
            if self.units[index].max_ammo > 0 && self.units[index].ammo == 0 {
                continue;
            }
            let Some(weapon) = self.units[index].weapon.clone() else {
                continue;
            };
            let owner = self.units[index].owner;
            let position = self.units[index].position;
            let scan_range = if owner == 0 {
                (weapon.range.to_f32() * 2.0).min(leptons_to_px(0x0A00))
            } else {
                weapon.range.to_f32()
            };
            let scan_squared = scan_range * scan_range;

            let mut best: Option<(usize, f32)> = None;
            for (other, candidate) in self.units.iter().enumerate() {
                if other == index || candidate.owner == owner || !candidate.is_alive() {
                    continue;
                }
                let dx = candidate.position.x.to_f32() - position.x.to_f32();
                let dy = candidate.position.y.to_f32() - position.y.to_f32();
                let distance_squared = dx * dx + dy * dy;
                if distance_squared > scan_squared {
                    continue;
                }
                // Fog of war: a unit cannot acquire a target it cannot see.
                // With the default fully-revealed shroud this never rejects.
                if !self
                    .shroud
                    .can_target(usize::from(owner), Self::world_cell(candidate.position))
                {
                    continue;
                }
                if best.is_none_or(|(_, current)| distance_squared < current) {
                    best = Some((other, distance_squared));
                }
            }

            if let Some((target, _)) = best {
                self.units[index].attack_target = Some(target);
                self.units[index].target = None;
                self.units[index].ordered = true;
            }
        }
    }

    /// Rotate turrets (or bodies for non-turret vehicles) toward the target.
    fn update_facings(&mut self, delta_time: Fixed16_16) {
        for index in 0..self.units.len() {
            let Some(target_index) = self.units[index].attack_target else {
                continue;
            };
            let Some(target) = self.units.get(target_index) else {
                continue;
            };
            if !target.is_alive() {
                continue;
            }

            let dx = target.position.x.to_f32() - self.units[index].position.x.to_f32();
            let dy = target.position.y.to_f32() - self.units[index].position.y.to_f32();
            if dx == 0.0 && dy == 0.0 {
                continue;
            }
            let desired = dy.atan2(dx);
            self.units[index].rotate_toward(desired, delta_time);
        }
    }

    /// Centre of a building's footprint, matching the collision centre used by
    /// `apply_blast` and the docking checks in `update_harvesters`.
    fn building_center(building: &Building) -> Vec2 {
        building.position + Vec2::new(Fixed16_16::from_int(12), Fixed16_16::from_int(12))
    }

    /// INFANTRY.CPP:2891-2901 `InfantryClass::What_Action`: a commando aimed at
    /// a building is rerouted from ACTION_ATTACK to ACTION_SABOTAGE.
    ///
    /// The unit walks into the building cell, then
    /// [`Game::update_special_missions`] runs the Per_Cell_Process arms.
    pub fn order_sabotage(&mut self, unit_index: usize, building_index: usize) -> bool {
        let Some(building) = self.buildings.get(building_index) else {
            return false;
        };
        if !building.is_alive() {
            return false;
        }
        let Some(unit) = self.units.get(unit_index) else {
            return false;
        };
        if !unit.is_alive() || !unit.is_commando || unit.owner == building.owner {
            return false;
        }

        let destination = Self::building_center(building);
        let unit = &mut self.units[unit_index];
        unit.attack_target = None;
        unit.clear_path();
        unit.special_target = Some(building_index);
        unit.target = Some(destination);
        unit.ordered = true;
        true
    }

    /// INFANTRY.CPP:669-698 `MISSION_CAPTURE`: an engineer enters a captureable
    /// enemy building, hands it to his house and is consumed (Delete_This).
    pub fn order_capture(&mut self, unit_index: usize, building_index: usize) -> bool {
        let Some(building) = self.buildings.get(building_index) else {
            return false;
        };
        // BUILDING.CPP:3873-3885 Can_Capture: Class->IsCaptureable.
        if !building.is_alive() || !building.is_captureable() {
            return false;
        }
        let Some(unit) = self.units.get(unit_index) else {
            return false;
        };
        if !unit.is_alive() || !unit.is_engineer || unit.owner == building.owner {
            return false;
        }

        let destination = Self::building_center(building);
        let unit = &mut self.units[unit_index];
        unit.attack_target = None;
        unit.clear_path();
        unit.special_target = Some(building_index);
        unit.target = Some(destination);
        unit.ordered = true;
        true
    }

    /// Apply one lockstep command to the simulation; true when it had effect.
    ///
    /// The wire format already carries sabotage (network.rs
    /// `Command::Sabotage`), so this is only the game-side arm. Orders are
    /// validated against the issuing player, and the simulation advances on unit
    /// and building indices only, so every peer applying the same command stream
    /// reaches identical state (see `sabotage_ticks_deterministically_across_
    /// identical_games`).
    /// Apply one lockstep command to the simulation; true when it had effect.
    ///
    /// Every wire [`Command`] variant has an explicit arm here (no catch-all),
    /// so the compiler rejects a future variant that is left unhandled. The
    /// per-variant validation and mutation live in [`Self::try_apply_command`],
    /// which returns a typed error for malformed payloads; this entry point
    /// keeps the historical `bool` return for replay/UI callers.
    pub fn apply_command(&mut self, player: u8, command: &Command) -> bool {
        self.try_apply_command(player, command).unwrap_or(false)
    }

    /// Validated variant of [`Self::apply_command`]. Returns `Ok(true)` when the
    /// command had an effect, `Ok(false)` when it was valid but a no-op, and
    /// `Err` when a payload index was out of range or otherwise malformed.
    pub fn try_apply_command(
        &mut self,
        player: u8,
        command: &Command,
    ) -> Result<bool, ApplyCommandError> {
        match command {
            Command::Move { units, target } => {
                for &unit in units {
                    if unit as usize >= self.units.len() {
                        return Err(ApplyCommandError::InvalidUnit(unit));
                    }
                }
                Ok(self.order_units_move(player, units, *target) > 0)
            }
            Command::Attack { units, target } => {
                for &unit in units {
                    if unit as usize >= self.units.len() {
                        return Err(ApplyCommandError::InvalidUnit(unit));
                    }
                }
                if *target as usize >= self.units.len() {
                    return Err(ApplyCommandError::InvalidTarget(*target));
                }
                Ok(self.order_units_attack(player, units, *target as usize) > 0)
            }
            Command::Stop { units } => {
                for &unit in units {
                    if unit as usize >= self.units.len() {
                        return Err(ApplyCommandError::InvalidUnit(unit));
                    }
                }
                Ok(self.order_units_stop(player, units) > 0)
            }
            Command::Guard { units, target } => {
                for &unit in units {
                    if unit as usize >= self.units.len() {
                        return Err(ApplyCommandError::InvalidUnit(unit));
                    }
                }
                if *target as usize >= self.units.len() {
                    return Err(ApplyCommandError::InvalidTarget(*target));
                }
                let guard_pos = self.units[*target as usize].position;
                let mut ordered = 0usize;
                for &unit in units {
                    let idx = unit as usize;
                    if idx >= self.units.len() {
                        continue;
                    }
                    if self.units[idx].owner != player || !self.units[idx].is_alive() {
                        continue;
                    }
                    self.units[idx].target = Some(guard_pos);
                    self.units[idx].ordered = true;
                    ordered += 1;
                }
                Ok(ordered > 0)
            }
            Command::Build { building, location } => {
                let structure = crate::td_rules::StructureType::from_class_id(*building)
                    .ok_or(ApplyCommandError::InvalidStructure(*building))?;
                let cell = Self::world_cell(*location);
                if !self.map.grid.in_bounds(cell.x, cell.y) {
                    return Err(ApplyCommandError::InvalidCell);
                }
                let queued = self.queue_building(player, structure);
                let placed = self.place_ready_building(player, cell);
                Ok(queued || placed)
            }
            Command::Sell { building } => {
                if *building as usize >= self.buildings.len() {
                    return Err(ApplyCommandError::InvalidBuilding(*building));
                }
                Ok(self.sell_building(player, *building as usize).is_some())
            }
            Command::Repair { unit: _, target } => {
                if *target as usize >= self.buildings.len() {
                    return Err(ApplyCommandError::InvalidBuilding(*target));
                }
                Ok(self.repair_building(player, *target as usize).is_some())
            }
            Command::Power { building, on } => {
                if *building as usize >= self.buildings.len() {
                    return Err(ApplyCommandError::InvalidBuilding(*building));
                }
                if self.buildings[*building as usize].owner != player {
                    return Ok(false);
                }
                self.buildings[*building as usize].set_powered(*on);
                Ok(true)
            }
            Command::Produce { factory, unit_type } => {
                if *factory as usize >= self.buildings.len() {
                    return Err(ApplyCommandError::InvalidBuilding(*factory));
                }
                let unit_type = crate::td_rules::unit_type_for_class(*unit_type)
                    .ok_or(ApplyCommandError::InvalidUnitType(*unit_type))?;
                Ok(self.queue_unit(player, unit_type))
            }
            Command::Deploy { unit } => {
                let idx = *unit as usize;
                if *unit as usize >= self.units.len() {
                    return Err(ApplyCommandError::InvalidUnit(*unit));
                }
                if self.units[idx].owner == player && self.units[idx].is_alive() {
                    self.units[idx].deployed = true;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Command::Undeploy { unit } => {
                if *unit as usize >= self.units.len() {
                    return Err(ApplyCommandError::InvalidUnit(*unit));
                }
                let idx = *unit as usize;
                if self.units[idx].owner != player || !self.units[idx].is_alive() {
                    return Ok(false);
                }
                self.units[idx].deployed = false;
                Ok(true)
            }
            Command::Sabotage { unit, target } => {
                if *unit as usize >= self.units.len() {
                    return Err(ApplyCommandError::InvalidUnit(*unit));
                }
                if *target as usize >= self.buildings.len() {
                    return Err(ApplyCommandError::InvalidBuilding(*target));
                }
                Ok(crate::network::apply_sabotage(self, command, player))
            }
            Command::RepairStructure { building } => {
                if *building as usize >= self.buildings.len() {
                    return Err(ApplyCommandError::InvalidBuilding(*building));
                }
                Ok(self.repair_building(player, *building as usize).is_some())
            }
            Command::CancelProduction => Ok(self.cancel_production(player).is_some()),
            Command::PlaceBuilding { location } => {
                let cell = Self::world_cell(*location);
                if !self.map.grid.in_bounds(cell.x, cell.y) {
                    return Err(ApplyCommandError::InvalidCell);
                }
                Ok(self.place_ready_building(player, cell))
            }
            Command::Board {
                transport,
                passenger,
            } => {
                if *transport as usize >= self.units.len() {
                    return Err(ApplyCommandError::InvalidUnit(*transport));
                }
                if *passenger as usize >= self.units.len() {
                    return Err(ApplyCommandError::InvalidUnit(*passenger));
                }
                Ok(self.apply_board(player, *transport as usize, *passenger as usize))
            }
            Command::Unload { transport, slot } => {
                if *transport as usize >= self.units.len() {
                    return Err(ApplyCommandError::InvalidUnit(*transport));
                }
                Ok(self.apply_unload(player, *transport as usize, *slot))
            }
        }
    }

    /// Aliases for callers that expect a different name for the validated entry point.
    pub fn try_apply_command_checked(
        &mut self,
        player: u8,
        command: &Command,
    ) -> Result<bool, ApplyCommandError> {
        self.try_apply_command(player, command)
    }
    pub fn apply_command_checked(
        &mut self,
        player: u8,
        command: &Command,
    ) -> Result<bool, ApplyCommandError> {
        self.try_apply_command(player, command)
    }
    pub fn apply_command_result(
        &mut self,
        player: u8,
        command: &Command,
    ) -> Result<bool, ApplyCommandError> {
        self.try_apply_command(player, command)
    }

    /// Deterministic CRC over the full serialised game state.
    pub fn state_crc(&self) -> u64 {
        match self.to_save_bytes() {
            Ok(bytes) => {
                const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
                const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
                let mut hash = FNV_OFFSET;
                for byte in bytes {
                    hash ^= u64::from(byte);
                    hash = hash.wrapping_mul(FNV_PRIME);
                }
                hash
            }
            Err(_) => 0,
        }
    }
    pub fn world_state_crc(&self) -> u64 {
        self.state_crc()
    }
    pub fn state_checksum(&self) -> u64 {
        self.state_crc()
    }
    pub fn crc(&self) -> u64 {
        self.state_crc()
    }

    /// Kill every passenger aboard a destroyed transport.
    ///
    /// Called when a transport is destroyed so its cargo does not survive
    /// the wreck.
    fn kill_transport_passengers(&mut self, transport_idx: usize) {
        let passenger_indices = {
            let transport = &self.units[transport_idx];
            transport.passengers.clone()
        };

        for passenger_idx in passenger_indices {
            if let Some(passenger) = self.units.get_mut(passenger_idx) {
                passenger.health = Fixed16_16::ZERO;
                passenger.is_selected = false;
                passenger.attack_target = None;
                passenger.special_target = None;
                passenger.target = None;
                passenger.ordered = false;
                passenger.clear_path();
            }
        }

        if let Some(transport) = self.units.get_mut(transport_idx) {
            transport.passengers.clear();
        }
    }
    /// INFANTRY.CPP:660-740 `InfantryClass::Per_Cell_Process`: resolve the
    /// MISSION_SABOTAGE and MISSION_CAPTURE arrivals, then run the BUILDING.CPP
    /// demolition timeout.
    ///
    /// Iteration order is the stable unit/building vectors (no hash maps, no
    /// wall-clock), so lockstep peers agree on every planted charge.
    fn update_special_missions(&mut self, delta_time: Fixed16_16) {
        let mut captures: Vec<(usize, usize)> = Vec::new();
        let mut detonations: Vec<usize> = Vec::new();

        for (index, unit) in self.units.iter_mut().enumerate() {
            let Some(target) = unit.special_target else {
                continue;
            };
            let alive_target = self
                .buildings
                .get(target)
                .is_some_and(|building| building.is_alive());
            if !unit.is_alive() || !alive_target {
                unit.special_target = None;
                continue;
            }
            let entered = Self::building_center(&self.buildings[target])
                .distance(unit.position)
                .to_f32()
                <= TILE_SIZE;
            if !entered {
                continue;
            }
            if unit.is_engineer {
                unit.special_target = None;
                captures.push((index, target));
            } else if unit.is_commando {
                // INFANTRY.CPP:725-733: arm the charge, then Scatter away.
                if let Some(building) = self.buildings.get_mut(target) {
                    building.plant_c4(index as u32);
                }
                unit.special_target = None;
                unit.ordered = false;
                unit.target = None;
                unit.clear_path();
            } else {
                unit.special_target = None;
            }
        }

        // BUILDING.CPP:1229-1246: once IsGoingToBlow and CountDown expired the
        // building takes SABOTAGE_DAMAGE of WARHEAD_FIRE, credited to
        // WhomToRepay (the saboteur's house, BUILDING.H:140-142).
        let fire = self.warhead_profile(WarheadType::Fire);
        for index in 0..self.buildings.len() {
            let Some(building) = self.buildings.get_mut(index) else {
                continue;
            };
            if !building.is_going_to_blow || !building.is_alive() {
                continue;
            }
            building.count_down = (building.count_down - delta_time).max(Fixed16_16::ZERO);
            if building.demolition_expired() {
                detonations.push(index);
            }
        }
        for index in detonations {
            if let Some(building) = self.buildings.get_mut(index) {
                building.detonate(&fire);
            }
        }

        for (unit_index, building_index) in captures {
            let new_owner = self.units[unit_index].owner;
            let previous = self
                .buildings
                .get(building_index)
                .map(|building| building.owner);
            if let Some(building) = self.buildings.get_mut(building_index) {
                building.capture(new_owner);
            }
            // Ownership bookkeeping mirrors sell_building: the structure moves
            // to the capturing house's registry and power is recomputed.
            if let Some(previous) = previous.filter(|owner| *owner != new_owner) {
                if let Some(old) = self.houses.get_mut(previous as usize) {
                    old.remove_building(building_index as u32);
                }
                if let Some(new) = self.houses.get_mut(new_owner as usize) {
                    new.add_building(building_index as u32);
                }
                if let Some(source) = self.event_source.as_ref() {
                    let structure = self
                        .buildings
                        .get(building_index)
                        .map(|building| building.structure.ini_name().to_string())
                        .unwrap_or_default();
                    crate::events::structure_captured(
                        source,
                        previous,
                        new_owner,
                        structure,
                        building_index,
                    );
                }
            }
            self.refresh_house_power();
            // INFANTRY.CPP:696 Delete_This: the engineer is spent.
            self.consume_unit(unit_index);
        }
    }

    /// INFANTRY.CPP `Delete_This`: the object leaves the world but its index
    /// stays stable (the port never compacts `Game::units`), so it is simply
    /// marked dead and stripped of every order like a killed unit.
    fn consume_unit(&mut self, index: usize) {
        if let Some(unit) = self.units.get_mut(index) {
            unit.health = Fixed16_16::ZERO;
            unit.is_selected = false;
            unit.attack_target = None;
            unit.special_target = None;
            unit.target = None;
            unit.ordered = false;
            unit.clear_path();
            if let Some(house) = self.houses.get_mut(unit.owner as usize) {
                house.remove_unit(index as u32);
            }
            // Passengers die with a destroyed transport
            if unit.is_transport() {
                self.kill_transport_passengers(index);
            }
        }
    }

    fn update_attacks(&mut self) {
        for index in 0..self.units.len() {
            let Some(target_index) = self.units[index].attack_target else {
                continue;
            };
            let target_info = self
                .units
                .get(target_index)
                .map(|target| (target.position, target.owner, target.is_alive()));
            let Some((target_position, target_owner, target_alive)) = target_info else {
                self.units[index].attack_target = None;
                continue;
            };

            let (position, owner, primary, secondary, reload) = {
                let unit = &self.units[index];
                (
                    unit.position,
                    unit.owner,
                    unit.weapon.clone(),
                    unit.secondary_weapon.clone(),
                    unit.reload,
                )
            };

            if !target_alive || target_owner == owner {
                self.units[index].attack_target = None;
                continue;
            }

            let Some(primary) = primary else {
                continue;
            };

            let distance = position.distance(target_position);
            let secondary_range = secondary
                .as_ref()
                .map(|weapon| weapon.range)
                .unwrap_or(Fixed16_16::ZERO);
            let max_range = primary.range.max(secondary_range);

            let chosen = if distance <= primary.range {
                Some((primary.clone(), true))
            } else if let Some(secondary) = secondary.clone() {
                if distance <= secondary.range {
                    Some((secondary, true))
                } else {
                    Some((primary.clone(), false))
                }
            } else {
                Some((primary.clone(), false))
            };
            let Some((weapon, in_range)) = chosen else {
                continue;
            };

            if in_range {
                self.units[index].target = None;
                let desired = (target_position.y.to_f32() - position.y.to_f32())
                    .atan2(target_position.x.to_f32() - position.x.to_f32());

                let has_ammo = self.units[index].max_ammo == 0 || self.units[index].ammo > 0;
                if reload <= Fixed16_16::ZERO && has_ammo && self.units[index].aiming_at(desired) {
                    self.projectiles.push(Projectile::new(
                        weapon.clone(),
                        position,
                        target_position,
                        index as u32,
                        Some(target_index as u32),
                    ));
                    self.projectiles_fired += 1;
                    if weapon.weapon_type != primary.weapon_type {
                        self.secondary_shots += 1;
                    }
                    // TECHNO.CPP Rearm_Delay: single shooters always use the
                    // normal ROF + 3; two-shooters alternate 9 ticks and ROF + 3.
                    let delay = self.units[index].next_rearm_delay(&weapon);
                    self.units[index].reload = if delay > Fixed16_16::ZERO {
                        delay
                    } else {
                        Fixed16_16::from_f32(0.5)
                    };
                    self.units[index].note_shot_fired();
                    if self.units[index].is_aircraft {
                        self.aircraft_shots += 1;
                    }
                    if self.units[index].max_ammo > 0 {
                        self.units[index].ammo = self.units[index].ammo.saturating_sub(1);
                        if self.units[index].ammo == 0 {
                            self.units[index].attack_target = None;
                        }
                    }
                }
            } else {
                // FOOT.CPP Approach_Target: stop short of weapon range so the
                // unit can fire without walking into the target.
                let approach = (max_range - Fixed16_16::from_f32(leptons_to_px(0x00B7)))
                    .max(Fixed16_16::from_f32(8.0));
                let offset = target_position - position;
                let distance = offset.length();
                let destination = if distance > approach {
                    target_position - offset.normalize() * approach
                } else {
                    position
                };

                if self.units[index].path.is_empty() && self.units[index].target.is_none() {
                    let goal_cell = Vec2i::new(
                        (destination.x.to_f32() / TILE_SIZE).floor() as i32,
                        (destination.y.to_f32() / TILE_SIZE).floor() as i32,
                    );
                    let used: Vec<(i32, i32)> = self
                        .occupied_cells(Some(index))
                        .iter()
                        .map(|cell| (cell.x, cell.y))
                        .collect();
                    let terrain_ok = self.map.grid.in_bounds(goal_cell.x, goal_cell.y)
                        && self.map.grid.cells[(goal_cell.y as u32 * self.map.grid.width
                            + goal_cell.x as u32)
                            as usize]
                            .is_passable;
                    let goal = if terrain_ok && !used.contains(&(goal_cell.x, goal_cell.y)) {
                        goal_cell
                    } else {
                        let (x, y) = self.nearby_cell(goal_cell.x, goal_cell.y, &used);
                        Vec2i::new(x, y)
                    };
                    let path = self.path_to_cell(index, goal);
                    self.units[index].set_path(path);
                }

                self.units[index].target = Some(destination);
                self.units[index].ordered = true;
            }
        }
    }

    /// TD defences (guard towers, turrets, SAM sites, obelisks) acquire and
    /// shoot at enemy units in range.
    ///
    /// Damage is applied directly (no projectile bookkeeping): a defence needs
    /// a target, power and an expired reload to fire.
    fn update_defence_combat(&mut self, delta_time: Fixed16_16) {
        // Snapshot each potential target plus, per house, whether that house
        // can see the cell (a defence cannot shoot what its owner cannot see).
        let targets: Vec<(Vec2, u8, bool, bool, Vec<bool>)> = self
            .units
            .iter()
            .map(|unit| {
                let cell = Self::world_cell(unit.position);
                let visibility: Vec<bool> = (0..cnc_core::MAX_HOUSES)
                    .map(|house| self.shroud.can_target(house, cell))
                    .collect();
                (
                    unit.position,
                    unit.owner,
                    unit.is_alive(),
                    unit.is_aircraft,
                    visibility,
                )
            })
            .collect();

        let mut hits: Vec<(usize, i32, WarheadType)> = Vec::new();

        for building in &mut self.buildings {
            if !building.is_alive() {
                continue;
            }
            building.reload = (building.reload - delta_time).max(Fixed16_16::ZERO);

            let Some(structure) = building.structure_type() else {
                continue;
            };
            if !structure.is_defensive() {
                continue;
            }
            // Low power disables defensive fire, as the original gates on it.
            if !building.is_powered {
                continue;
            }
            let Some(weapon) = &building.weapon else {
                continue;
            };
            let range = weapon.range;
            let anti_air_only = structure.is_anti_air();

            if building.reload > Fixed16_16::ZERO {
                continue;
            }

            let mut best: Option<(usize, Fixed16_16)> = None;
            for (index, (position, owner, alive, is_air, visibility)) in targets.iter().enumerate()
            {
                if !*alive || *owner == building.owner {
                    continue;
                }
                // Fog of war: no shooting at cells the owner cannot see.
                if !visibility
                    .get(usize::from(building.owner))
                    .copied()
                    .unwrap_or(false)
                {
                    continue;
                }
                // SAM sites only engage aircraft.
                if anti_air_only && !*is_air {
                    continue;
                }
                let distance = building.position.distance(*position);
                if distance > range {
                    continue;
                }
                if best.is_none_or(|(_, current)| distance < current) {
                    best = Some((index, distance));
                }
            }

            let Some((target_index, _)) = best else {
                continue;
            };

            // Rotate turret to face the target (SAML missile cadence per BUILDING.CPP).
            let target_position = targets[target_index].0;
            let offset = target_position - building.position;
            let desired = offset.y.to_f32().atan2(offset.x.to_f32());
            let diff = (desired - building.turret_facing).rem_euclid(std::f32::consts::TAU);
            let turn_rate = weapon.rate_of_fire.to_f32() * 2.0; // turrets track at 2x ROF
            let step = diff.clamp(-turn_rate, turn_rate);
            building.turret_facing =
                (building.turret_facing + step).rem_euclid(std::f32::consts::TAU);

            hits.push((
                target_index,
                weapon.damage.int(),
                weapon.warhead.warhead_type,
            ));
            building.reload = weapon.rate_of_fire;
        }

        for (index, damage, warhead) in hits {
            if let Some(unit) = self.units.get_mut(index) {
                let modified = modify_damage(
                    damage, warhead, unit.armor,
                    0, // direct fire: no distance falloff at the impact point
                );
                unit.health = (unit.health - Fixed16_16::from_int(modified)).max(Fixed16_16::ZERO);
            }
        }
    }
    fn update_projectiles(&mut self, delta_time: Fixed16_16) {
        // BULLET.CPP homing missiles re-aim at the target every other frame.
        let target_positions: Vec<Option<Vec2>> = self
            .units
            .iter()
            .map(|unit| {
                if unit.is_alive() {
                    Some(unit.position)
                } else {
                    None
                }
            })
            .collect();

        let mut hits: Vec<(Fixed16_16, Warhead, Vec2, usize)> = Vec::new();
        self.projectiles.retain_mut(|projectile| {
            if projectile.homing && projectile.age % 2 == 0 {
                if let Some(target) = projectile.target_entity {
                    if let Some(Some(position)) = target_positions.get(target as usize) {
                        projectile.target = *position;
                    }
                }
            }
            if projectile.update(delta_time) {
                hits.push((
                    projectile.weapon.damage,
                    projectile.weapon.warhead,
                    projectile.position,
                    projectile.source as usize,
                ));
                false
            } else {
                true
            }
        });

        // COMBAT.CPP Explosion_Damage: everything within 1.5 cells of the
        // impact takes warhead-modified damage, attenuated by distance.
        let blast_radius = leptons_to_px(EXPLOSION_RANGE_LEPTONS);
        for (base_damage, warhead, impact, source) in hits {
            for (index, unit) in self.units.iter_mut().enumerate() {
                if index == source || !unit.is_alive() {
                    continue;
                }
                let distance = unit.position.distance(impact).to_f32();
                if distance > blast_radius {
                    continue;
                }
                let damage = warhead.damage(base_damage, unit.armor, px_to_leptons(distance));
                unit.take_damage(damage);
            }
        }
    }

    /// Render the full scene: terrain, buildings, then units.
    pub fn render(&self, renderer: &mut dyn Renderer) {
        self.render_with_sprites(renderer, None);
    }

    /// Record any cell the player has newly explored.
    ///
    /// `refresh_shroud` keeps `mapped` as a running history, so the set of
    /// cells that *were* mapped before a refresh versus after it is exactly
    /// the set the player just discovered.
    pub(crate) fn record_discoveries(&mut self, before: &cnc_core::Shroud) {
        let player = self.player_owner as usize;
        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                let cell = Vec2i::new(x, y);
                if self.shroud.is_mapped(player, cell) && !before.is_mapped(player, cell) {
                    self.discovered_cells.insert(cell);
                }
            }
        }
    }

    /// Kills credited to `owner` for one asset class.
    pub fn kills_by(&self, owner: u8, what: crate::mission::Eliminated) -> i64 {
        self.kills.get(&(owner, what)).copied().unwrap_or(0)
    }

    /// Was the given cell discovered since the last drain?
    pub fn was_discovered(&self, cell: Vec2i) -> bool {
        self.discovered_cells.contains(&cell)
    }

    /// Any discovery pending evaluation?
    pub fn any_discovery_pending(&self) -> bool {
        !self.discovered_cells.is_empty()
    }

    /// Is this cell currently visible to the local player?
    pub fn visible_to_player(&self, cell: Vec2i) -> bool {
        self.shroud.is_visible(self.player_owner as usize, cell)
    }

    /// Is this cell explored (seen at least once) by the local player?
    pub fn mapped_by_player(&self, cell: Vec2i) -> bool {
        self.shroud.is_mapped(self.player_owner as usize, cell)
    }

    /// Draw the fog-of-war overlay: unexplored cells are black, explored but
    /// not currently visible cells are shaded.
    ///
    /// This is the "draw the shroud" half that was missing — the grid already
    /// gated targeting, but nothing rendered it.
    fn render_shroud(&self, renderer: &mut dyn Renderer) {
        // A fully revealed board needs no overlay. The guard must be "every
        // cell is visible", not "some cell is hidden" — the latter is also
        // true when the whole board is hidden, which is when fog matters most.
        let player_cells = self.map.width * self.map.height;
        if self.shroud.visible_count(self.player_owner as usize) >= player_cells {
            return;
        }
        let player = self.player_owner as usize;
        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                let cell = Vec2i::new(x, y);
                if self.shroud.is_visible(player, cell) {
                    continue;
                }
                let color = if self.shroud.is_mapped(player, cell) {
                    // Explored but out of sight: dim, not black.
                    [0.0, 0.0, 0.0, SHROUD_MEMORY_ALPHA]
                } else {
                    [0.0, 0.0, 0.0, 1.0]
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

    pub fn render_with_sprites(&self, renderer: &mut dyn Renderer, atlas: Option<&SpriteAtlas>) {
        // Terrain layer. The tileset pack owns its own texture slot
        // (`TILESET_TEXTURE_ID`) via the multi-atlas renderer API, so terrain
        // renders as RTS tileset sprites alongside the procedural
        // unit/building atlas (`CNC_TILESET_DIR` / `CNC_TILESET_PALETTE`
        // overrides, default palette `td_inspired`); flat quads only when no
        // pack loads.
        let tileset = TD_TILESET.get_or_init(|| {
            let pack = cnc_asset::tileset_pack::load_default("td_inspired");
            // One-shot loader diagnostics: exactly one line per process,
            // success or failure (map.rs logs atlas-upload failures separately).
            match &pack {
                Some(pack) => eprintln!(
                    "tileset pack loaded (td_inspired): {}x{} atlas, {} categories",
                    pack.width(),
                    pack.height(),
                    pack.categories().count()
                ),
                None => eprintln!(
                    "tileset pack (td_inspired) unavailable; terrain falls back to flat quads"
                ),
            }
            pack
        });
        self.map.render_with_tiles(renderer, tileset.as_ref());
        for building in &self.buildings {
            // CELL.CPP only draws what the player can actually see or has
            // explored: hidden units are not drawn at all.
            if !self.visible_to_player(Self::building_cell(building)) {
                continue;
            }
            building.render_with(renderer, atlas);
        }
        for unit in &self.units {
            if !self.visible_to_player(Self::unit_cell(unit)) {
                continue;
            }
            unit.render_with(renderer, atlas);
        }
        self.render_shroud(renderer);
        for projectile in &self.projectiles {
            let x = projectile.position.x.to_f32();
            let y = projectile.position.y.to_f32();
            let altitude = projectile.altitude.to_f32();
            renderer.draw_rect(x - 2.0, y - 2.0, 4.0, 4.0, [0.15, 0.15, 0.15, 0.6]);
            renderer.draw_rect(x - 2.0, y - 2.0 - altitude, 4.0, 4.0, [1.0, 0.85, 0.2, 1.0]);
        }
        for blast in &self.blasts {
            let ratio = (blast.ttl.to_f32() / blast.max_ttl.to_f32().max(1.0e-6)).clamp(0.0, 1.0);
            let radius = blast.radius * (1.15 - 0.35 * ratio);
            let x = blast.position.x.to_f32();
            let y = blast.position.y.to_f32();
            let color = blast.color;
            // `resources_fx` blast sprite keyed by remaining lifetime: fresh
            // blasts use the explosion family, mid-life the small fire, and
            // dying blasts the smoke puff. The colour rects below stay as the
            // no-pack fallback.
            let (fx_family, fx_fallback_row) = if ratio > 0.66 {
                ("explosion", 9u8)
            } else if ratio > 0.33 {
                ("fire_small", 7u8)
            } else {
                ("smoke_puff", 8u8)
            };
            let fx_sprite = tileset
                .as_ref()
                .filter(|_| renderer.texture_ready(cnc_renderer::TILESET_TEXTURE_ID))
                .and_then(|pack| {
                    // Deterministic per-blast variant column (same convention
                    // as the terrain layer, on blast world coordinates).
                    let col = ((x as i32 * 7 + y as i32 * 13).rem_euclid(16)) as u8;
                    let row = pack
                        .family_row("resources_fx", fx_family)
                        .unwrap_or(fx_fallback_row);
                    pack.tile_px_rect("resources_fx", row, col)
                });
            if let Some((uv_x, uv_y, uv_w, uv_h)) = fx_sprite {
                let mut sprite = cnc_renderer::Sprite::new(
                    cnc_renderer::TILESET_TEXTURE_ID,
                    Vec2::new(
                        Fixed16_16::from_f32(x - radius),
                        Fixed16_16::from_f32(y - radius),
                    ),
                    Vec2::new(
                        Fixed16_16::from_f32(radius * 2.0),
                        Fixed16_16::from_f32(radius * 2.0),
                    ),
                );
                sprite.uv_rect =
                    cnc_core::Rect::new(uv_x as i32, uv_y as i32, uv_w as i32, uv_h as i32);
                renderer.draw_sprite(&sprite);
            } else {
                renderer.draw_rect(
                    x - radius,
                    y - radius,
                    radius * 2.0,
                    radius * 2.0,
                    [color[0], color[1], color[2], 0.30 * ratio],
                );
                let inner = radius * 0.6;
                renderer.draw_rect(
                    x - inner,
                    y - inner,
                    inner * 2.0,
                    inner * 2.0,
                    [color[0], color[1], color[2], 0.45 * ratio],
                );
                let core = radius * 0.3;
                renderer.draw_rect(
                    x - core,
                    y - core,
                    core * 2.0,
                    core * 2.0,
                    [1.0, 1.0, 1.0, 0.35 * ratio],
                );
            }
        }

        for beam in &self.beams {
            let ratio = (beam.ttl.to_f32() / beam.max_ttl.to_f32().max(1.0e-6)).clamp(0.0, 1.0);
            let x = beam.position.x.to_f32();
            let y = beam.position.y.to_f32();
            renderer.draw_rect(
                x - beam.width * 0.5,
                y - 260.0,
                beam.width,
                260.0,
                [0.60, 0.85, 1.00, 0.35 * ratio],
            );
            renderer.draw_rect(
                x - beam.width * 0.25,
                y - 260.0,
                beam.width * 0.5,
                260.0,
                [1.0, 1.0, 1.0, 0.50 * ratio],
            );
        }

        for strike in &self.strikes {
            let progress = strike.progress.to_f32();
            let from_x = strike.from.x.to_f32();
            let from_y = strike.from.y.to_f32();
            let to_x = strike.to.x.to_f32();
            let to_y = strike.to.y.to_f32();
            for trail in 0..4 {
                let t = (progress - 0.03 * (trail as f32 + 1.0)).max(0.0);
                let x = from_x + (to_x - from_x) * t;
                let y = from_y + (to_y - from_y) * t;
                renderer.draw_rect(
                    x - 2.0,
                    y - 2.0,
                    4.0,
                    4.0,
                    [1.0, 0.65, 0.25, 0.85 - 0.15 * trail as f32],
                );
            }
        }

        // Move-order destination markers.
        for unit in &self.units {
            if let Some(target) = unit.target {
                renderer.draw_rect(
                    target.x.to_f32() - 3.0,
                    target.y.to_f32() - 3.0,
                    6.0,
                    6.0,
                    [1.0, 0.9, 0.2, 1.0],
                );
            }
        }
    }

    /// World-space centre of the map, useful for centring a camera.
    pub fn world_center(&self) -> Vec2 {
        Vec2::new(
            Fixed16_16::from_f32(self.map.width as f32 * TILE_SIZE * 0.5),
            Fixed16_16::from_f32(self.map.height as f32 * TILE_SIZE * 0.5),
        )
    }

    pub fn unit_count(&self) -> usize {
        self.units.len()
    }

    pub fn building_count(&self) -> usize {
        self.buildings.len()
    }

    /// Select the nearest owned unit within a click radius of a world position.
    ///
    /// Returns the index of the newly selected unit, if any.
    pub fn select_at(&mut self, world: Vec2) -> Option<usize> {
        self.select_at_additive(world, false)
    }

    /// Click-selection with optional additive (shift) behaviour.
    pub fn select_at_additive(&mut self, world: Vec2, additive: bool) -> Option<usize> {
        const PICK_RADIUS: f32 = 22.0;
        let mut best: Option<(usize, f32)> = None;
        for (index, unit) in self.units.iter().enumerate() {
            if unit.owner != self.player_owner || !unit.is_alive() {
                continue;
            }
            let dx = unit.position.x.to_f32() - world.x.to_f32();
            let dy = unit.position.y.to_f32() - world.y.to_f32();
            let distance_squared = dx * dx + dy * dy;
            if distance_squared <= PICK_RADIUS * PICK_RADIUS
                && best.is_none_or(|(_, current)| distance_squared < current)
            {
                best = Some((index, distance_squared));
            }
        }

        if !additive {
            for unit in &mut self.units {
                unit.is_selected = false;
            }
        }
        if let Some((index, _)) = best {
            self.units[index].is_selected = true;
            Some(index)
        } else {
            None
        }
    }

    /// Box-select every living owned unit inside the world-space rectangle.
    pub fn select_in_rect(&mut self, a: Vec2, b: Vec2, additive: bool) -> usize {
        let min_x = a.x.to_f32().min(b.x.to_f32());
        let max_x = a.x.to_f32().max(b.x.to_f32());
        let min_y = a.y.to_f32().min(b.y.to_f32());
        let max_y = a.y.to_f32().max(b.y.to_f32());

        if !additive {
            for unit in &mut self.units {
                unit.is_selected = false;
            }
        }

        let mut count = 0;
        for unit in &mut self.units {
            if unit.owner != self.player_owner || !unit.is_alive() {
                continue;
            }
            let x = unit.position.x.to_f32();
            let y = unit.position.y.to_f32();
            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                unit.is_selected = true;
                count += 1;
            }
        }
        count
    }

    /// Issue a move order to every selected unit, spread into a formation.
    ///
    /// Returns how many units were ordered.
    pub fn order_move(&mut self, world: Vec2) -> usize {
        let selected: Vec<usize> = self
            .units
            .iter()
            .enumerate()
            .filter(|(_, unit)| unit.is_selected && unit.is_alive())
            .map(|(index, _)| index)
            .collect();
        self.order_move_indices(&selected, world)
    }

    /// Move exactly the listed unit indices (network command application).
    fn order_move_indices(&mut self, selected: &[usize], world: Vec2) -> usize {
        let count = selected.len();
        if count == 0 {
            return 0;
        }

        let target_x = (world.x.to_f32() / TILE_SIZE).floor() as i32;
        let target_y = (world.y.to_f32() / TILE_SIZE).floor() as i32;

        let mut used: Vec<(i32, i32)> = self
            .occupied_cells(None)
            .iter()
            .map(|cell| (cell.x, cell.y))
            .collect();
        for &index in selected {
            let (cell_x, cell_y) = self.nearby_cell(target_x, target_y, &used);
            used.push((cell_x, cell_y));
            let goal = Vec2i::new(cell_x, cell_y);
            let path = self.path_to_cell(index, goal);
            let reserved = path.first().map(|point| Self::world_cell(*point));
            let destination = Self::cell_center(cell_x, cell_y);
            let unit = &mut self.units[index];
            unit.set_path(path);
            unit.reserved_cell = reserved;
            unit.target = Some(destination);
            unit.ordered = true;
            unit.attack_target = None;
        }
        count
    }

    /// Unit indices owned by `owner` among the given stable ids.
    fn owned_unit_indices(&self, owner: u8, unit_ids: &[u32]) -> Vec<usize> {
        unit_ids
            .iter()
            .filter_map(|id| {
                let index = *id as usize;
                self.units
                    .get(index)
                    .filter(|unit| unit.owner == owner && unit.is_alive())
                    .map(|_| index)
            })
            .collect()
    }

    /// Move a specific player's units by stable id (lockstep network use).
    pub fn order_units_move(&mut self, owner: u8, unit_ids: &[u32], world: Vec2) -> usize {
        let indices = self.owned_unit_indices(owner, unit_ids);
        self.order_move_indices(&indices, world)
    }

    /// Top-left origin of a cell in world coordinates.
    pub fn cell_origin(cx: i32, cy: i32) -> Vec2 {
        Self::cell(cx, cy)
    }

    /// Live infantry unit indices currently resting in `cell`.
    pub fn infantry_in_cell(&self, cell: Vec2i) -> Vec<usize> {
        self.units
            .iter()
            .enumerate()
            .filter(|(_, unit)| {
                unit.is_alive() && unit.is_infantry() && Self::unit_cell(unit) == cell
            })
            .map(|(index, _)| index)
            .collect()
    }

    /// Lowest free sub-cell slot in `cell`, if fewer than five infantry occupy it.
    pub fn free_sub_cell(&self, cell: Vec2i) -> Option<u8> {
        let used: BTreeSet<u8> = self
            .infantry_in_cell(cell)
            .iter()
            .filter_map(|&index| self.units.get(index).and_then(Unit::effective_sub_cell))
            .collect();
        (0..5u8).find(|slot| !used.contains(slot))
    }

    /// Assign `unit_index` to a free sub-cell slot in `cell`.
    pub fn assign_infantry_sub_cell(
        &mut self,
        unit_index: usize,
        cell: Vec2i,
    ) -> Result<u8, SubCellError> {
        let unit = self
            .units
            .get(unit_index)
            .ok_or(SubCellError::InvalidUnit)?;
        if !unit.is_infantry() {
            return Err(SubCellError::NotInfantry);
        }
        if !unit.is_alive() {
            return Err(SubCellError::InvalidUnit);
        }
        if self.units[unit_index].effective_sub_cell().is_some()
            && Self::unit_cell(&self.units[unit_index]) == cell
        {
            return Ok(self.units[unit_index].effective_sub_cell().unwrap());
        }
        let slot = self.free_sub_cell(cell).ok_or(SubCellError::CellFull)?;
        self.units[unit_index].set_sub_cell(Some(slot));
        let origin = Self::cell(cell.x, cell.y);
        if let Some(offset) = Unit::stopping_offset(slot) {
            self.units[unit_index].position = origin + offset;
        }
        Ok(slot)
    }

    /// Deterministically assign sub-cell slots to all live infantry.
    pub fn compact_infantry_sub_cells(&mut self) {
        let mut grouped: BTreeMap<(i32, i32), Vec<usize>> = BTreeMap::new();
        for (index, unit) in self.units.iter().enumerate() {
            if !unit.is_alive() || !unit.is_infantry() {
                continue;
            }
            grouped
                .entry((Self::unit_cell(unit).x, Self::unit_cell(unit).y))
                .or_default()
                .push(index);
        }
        for ((cx, cy), indices) in grouped {
            for (slot, index) in indices.into_iter().enumerate() {
                if slot >= 5 {
                    self.units[index].set_sub_cell(None);
                    continue;
                }
                let slot = slot as u8;
                self.units[index].set_sub_cell(Some(slot));
                let origin = Self::cell(cx, cy);
                if let Some(offset) = Unit::stopping_offset(slot) {
                    self.units[index].position = origin + offset;
                }
            }
        }
    }

    /// Validate every live infantry sub-cell assignment.
    pub fn validate_sub_cells(&self) -> Result<(), SubCellError> {
        let mut grouped: BTreeMap<(i32, i32), Vec<usize>> = BTreeMap::new();
        for (index, unit) in self.units.iter().enumerate() {
            if !unit.is_alive() || !unit.is_infantry() {
                continue;
            }
            grouped
                .entry((Self::unit_cell(unit).x, Self::unit_cell(unit).y))
                .or_default()
                .push(index);
        }
        for indices in grouped.values() {
            if indices.len() > 5 {
                return Err(SubCellError::CellFull);
            }
            let mut seen = BTreeSet::new();
            for &index in indices {
                if let Some(slot) = self.units[index].effective_sub_cell() {
                    if slot >= 5 {
                        return Err(SubCellError::OutOfRange);
                    }
                    if !seen.insert(slot) {
                        return Err(SubCellError::DuplicateSlot);
                    }
                }
            }
        }
        Ok(())
    }

    fn world_cell(position: Vec2) -> Vec2i {
        Vec2i::new(
            (position.x.to_f32() / TILE_SIZE).floor() as i32,
            (position.y.to_f32() / TILE_SIZE).floor() as i32,
        )
    }

    fn unit_cell(unit: &Unit) -> Vec2i {
        Self::world_cell(unit.position)
    }

    fn building_cell(building: &Building) -> Vec2i {
        Vec2i::new(
            (building.position.x.to_f32() / TILE_SIZE).floor() as i32,
            (building.position.y.to_f32() / TILE_SIZE).floor() as i32,
        )
    }

    /// Cells currently occupied by live units and buildings.
    ///
    /// Infantry cells are only considered blocked once all five sub-slots are
    /// full, so up to five infantry may share one cell deterministically.
    fn occupied_cells(&self, exclude_unit: Option<usize>) -> Vec<Vec2i> {
        let mut cells = Vec::new();
        let mut full_cells = BTreeSet::new();
        for (index, unit) in self.units.iter().enumerate() {
            if Some(index) == exclude_unit || !unit.is_alive() || !unit.is_infantry() {
                continue;
            }
            let cell = Self::unit_cell(unit);
            let count = self
                .units
                .iter()
                .enumerate()
                .filter(|(other, other_unit)| {
                    Some(*other) != exclude_unit
                        && other_unit.is_alive()
                        && other_unit.is_infantry()
                        && Self::unit_cell(other_unit) == cell
                })
                .count();
            if count >= 5 {
                full_cells.insert((cell.x, cell.y));
            }
        }
        for (x, y) in full_cells {
            cells.push(Vec2i::new(x, y));
        }
        for (index, unit) in self.units.iter().enumerate() {
            if Some(index) == exclude_unit || !unit.is_alive() {
                continue;
            }
            if !unit.is_infantry() {
                cells.push(Self::unit_cell(unit));
            }
            if let Some(reserved) = unit.reserved_cell {
                cells.push(reserved);
            }
        }
        for building in &self.buildings {
            cells.push(Self::building_cell(building));
        }
        cells
    }

    /// Build a cell-centre waypoint path for a unit, avoiding occupied cells.
    fn path_to_cell(&self, unit_index: usize, goal: Vec2i) -> Vec<Vec2> {
        let Some(unit) = self.units.get(unit_index) else {
            return Vec::new();
        };
        if unit.is_aircraft {
            return Vec::new();
        }
        let start = Self::unit_cell(unit);
        let blocked = self.occupied_cells(Some(unit_index));
        match find_path(&self.map.grid, &blocked, start, goal, 8192) {
            Some(cells) => cells
                .iter()
                .map(|cell| Self::cell_center(cell.x, cell.y))
                .collect(),
            None => Vec::new(),
        }
    }

    /// MAP.CPP MapClass::Nearby_Location: scan the expanding box edges outward
    /// and return the first free map cell.
    fn nearby_cell(&self, cell_x: i32, cell_y: i32, used: &[(i32, i32)]) -> (i32, i32) {
        let width = self.map.width as i32;
        let height = self.map.height as i32;
        let is_free = |x: i32, y: i32, used: &[(i32, i32)]| {
            x >= 0
                && y >= 0
                && x < width
                && y < height
                && self.map.grid.cells[(y * width + x) as usize].is_passable
                && !used.contains(&(x, y))
        };

        for radius in 0..=16 {
            for x in (cell_x - radius)..=(cell_x + radius) {
                if radius > 0 {
                    let top = cell_y - radius;
                    if is_free(x, top, used) {
                        return (x, top);
                    }
                }
                let bottom = cell_y + radius;
                if is_free(x, bottom, used) {
                    return (x, bottom);
                }
            }
            if radius > 0 {
                for y in (cell_y - radius + 1)..(cell_y + radius) {
                    let left = cell_x - radius;
                    if is_free(left, y, used) {
                        return (left, y);
                    }
                    let right = cell_x + radius;
                    if is_free(right, y, used) {
                        return (right, y);
                    }
                }
            }
        }
        (cell_x.clamp(0, width - 1), cell_y.clamp(0, height - 1))
    }

    /// Find the nearest enemy unit (relative to the player's house).
    pub fn enemy_at(&self, world: Vec2) -> Option<usize> {
        const PICK_RADIUS: f32 = 22.0;
        let mut best: Option<(usize, f32)> = None;
        for (index, unit) in self.units.iter().enumerate() {
            if unit.owner == self.player_owner || !unit.is_alive() {
                continue;
            }
            let dx = unit.position.x.to_f32() - world.x.to_f32();
            let dy = unit.position.y.to_f32() - world.y.to_f32();
            let distance_squared = dx * dx + dy * dy;
            if distance_squared <= PICK_RADIUS * PICK_RADIUS
                && best.is_none_or(|(_, current)| distance_squared < current)
            {
                best = Some((index, distance_squared));
            }
        }
        best.map(|(index, _)| index)
    }

    /// Order every selected unit to attack the given enemy unit index.
    pub fn order_attack(&mut self, target_index: usize) -> usize {
        let selected: Vec<usize> = self
            .units
            .iter()
            .enumerate()
            .filter(|(_, unit)| {
                unit.is_selected && unit.owner == self.player_owner && unit.is_alive()
            })
            .map(|(index, _)| index)
            .collect();
        self.order_attack_indices(self.player_owner, &selected, target_index)
    }

    /// Order the listed unit indices (owned by `owner`) onto an enemy index.
    fn order_attack_indices(
        &mut self,
        owner: u8,
        selected: &[usize],
        target_index: usize,
    ) -> usize {
        let valid_target = self
            .units
            .get(target_index)
            .is_some_and(|target| target.owner != owner && target.is_alive());
        if !valid_target {
            return 0;
        }

        let mut ordered = 0;
        for &index in selected {
            let Some(unit) = self.units.get_mut(index) else {
                continue;
            };
            if unit.owner == owner && unit.is_alive() && unit.weapon.is_some() {
                unit.attack_target = Some(target_index);
                unit.ordered = true;
                ordered += 1;
            }
        }
        ordered
    }

    /// Attack for a specific player's units by stable id (lockstep network use).
    pub fn order_units_attack(
        &mut self,
        owner: u8,
        unit_ids: &[u32],
        target_index: usize,
    ) -> usize {
        let indices = self.owned_unit_indices(owner, unit_ids);
        self.order_attack_indices(owner, &indices, target_index)
    }

    /// Clear move/attack orders for one player's listed units.
    pub fn order_units_stop(&mut self, owner: u8, unit_ids: &[u32]) -> usize {
        let indices = self.owned_unit_indices(owner, unit_ids);
        let mut stopped = 0;
        for index in indices {
            let unit = &mut self.units[index];
            if unit.target.is_some()
                || unit.attack_target.is_some()
                || !unit.path.is_empty()
                || unit.ordered
            {
                stopped += 1;
            }
            unit.target = None;
            unit.attack_target = None;
            unit.path.clear();
            unit.path_index = 0;
            unit.reserved_cell = None;
            unit.ordered = false;
        }
        stopped
    }

    /// Select every living unit owned by the given player (0 = GDI).
    pub fn select_all_owned(&mut self, owner: u8) -> usize {
        let mut count = 0;
        for unit in &mut self.units {
            unit.is_selected = unit.owner == owner && unit.is_alive();
            if unit.is_selected {
                count += 1;
            }
        }
        count
    }

    /// HUD-relevant snapshot of the current game state.
    pub fn status(&self) -> GameStatus {
        GameStatus {
            mission: self.mission.name.clone(),
            units: self.units.len(),
            buildings: self.buildings.len(),
            selected: self.units.iter().filter(|unit| unit.is_selected).count(),
            gdi_credits: self
                .houses
                .iter()
                .find(|house| house.id == ClassId::HouseGDI)
                .map(|house| house.credits)
                .unwrap_or(0),
            nod_credits: self
                .houses
                .iter()
                .find(|house| house.id == ClassId::HouseNod)
                .map(|house| house.credits)
                .unwrap_or(0),
            production: self
                .houses
                .iter()
                .map(|house| house.production_queue.len())
                .sum(),
            gdi_power: self.houses.first().map_or(0, |house| house.power),
            gdi_drain: self.houses.first().map_or(0, |house| house.max_power),
            nod_power: self.houses.get(1).map_or(0, |house| house.power),
            nod_drain: self.houses.get(1).map_or(0, |house| house.max_power),
            gdi_tiberium: self.houses.first().map_or(0, |house| house.tiberium),
            gdi_capacity: self.houses.first().map_or(0, |house| house.capacity),
            nod_tiberium: self.houses.get(1).map_or(0, |house| house.tiberium),
            nod_capacity: self.houses.get(1).map_or(0, |house| house.capacity),
            gdi_ion: self
                .houses
                .first()
                .map_or(0, |house| (house.ion_charge.to_f32() * 100.0) as i32),
            gdi_nuke: self
                .houses
                .first()
                .map_or(0, |house| (house.nuke_charge.to_f32() * 100.0) as i32),
            nod_ion: self
                .houses
                .get(1)
                .map_or(0, |house| (house.ion_charge.to_f32() * 100.0) as i32),
            nod_nuke: self
                .houses
                .get(1)
                .map_or(0, |house| (house.nuke_charge.to_f32() * 100.0) as i32),
            ready_buildings: self.ready_buildings.len(),
            objectives_total: self.mission.objectives.len(),
            objectives_done: self.mission.objectives_done(),
            mission_over: self.mission.is_over(),
            mission_victory: self.mission.outcome == Some(MissionOutcome::Victory),
        }
    }
}

/// Trigger-record execution arms for the Tiberian Dawn engine.
///
/// `cnc_core::trigger_apply` owns the vocabulary and dispatches one parsed
/// `TACTION` record per family; this impl binds each family to TD state. The
/// families TD does not model (movies, musical themes, mission timers,
/// preferred targets) keep the trait default, which logs through `note` and
/// returns `false`: a scripted action must never silently do nothing.
///
/// Superweapon actions (`TACTION_1_SPECIAL` / `TACTION_FULL_SPECIAL`) are
/// the Red Alert spelling of TD's `ACTION_AIRSTRIKE`/`ACTION_NUKE`/
/// `ACTION_ION`. The scripted strike is fired immediately (the campaign uses
/// them without the owning house having satisfied the building gate), aimed at
/// the opposing start position.
impl cnc_core::trigger_apply::TriggerHost for Game {
    fn note(&mut self, message: String) {
        eprintln!("[td trigger] {message}");
        self.mission.messages.push(message);
    }

    fn map_size(&self) -> (u32, u32) {
        (self.map.width, self.map.height)
    }

    fn default_house(&self) -> u8 {
        self.player_owner
    }

    fn house_count(&self) -> usize {
        self.houses.len()
    }

    fn win(&mut self, _house: u8) -> bool {
        self.apply_mission_action(&Action::Win);
        true
    }

    fn lose(&mut self, _house: u8) -> bool {
        self.apply_mission_action(&Action::Lose);
        true
    }

    fn allow_win(&mut self, house: u8) -> bool {
        // TD has no early-win gate: the flag lets a script prove the action
        // ran and lets later trigger conditions query it.
        self.mission.flags.insert(format!("allowwin:{house}"));
        true
    }

    fn begin_production(&mut self, house: u8, _team: Option<u16>) -> bool {
        match self.houses.get_mut(usize::from(house)) {
            Some(entry) => {
                entry.ai_enabled = true;
                true
            }
            None => self.reject("Production Begins"),
        }
    }

    fn create_team(&mut self, _house: u8, team: Option<u16>) -> bool {
        let Some(team_id) = team else {
            return self.reject("Create Team");
        };
        let Some(team_type) = self
            .team_manager
            .types
            .iter()
            .find(|candidate| candidate.id == u32::from(team_id))
            .cloned()
        else {
            return self.reject("Create Team");
        };
        let already_running = self
            .team_manager
            .teams
            .iter()
            .any(|existing| existing.team_type == team_type.id && existing.active);
        if already_running {
            return true;
        }
        let instance_id = self.team_manager.next_team_id;
        self.team_manager.next_team_id = instance_id.wrapping_add(1);
        let mut instance = cnc_core::entity::Team::from_type(instance_id, &team_type);
        instance.active = true;
        instance.suspended = false;
        self.team_manager.teams.push(instance);
        true
    }

    fn destroy_team(&mut self, _house: u8, team: Option<u16>) -> bool {
        let Some(team_id) = team else {
            return self.reject("Destroy All Teams");
        };
        let target = u32::from(team_id);
        let mut disbanded = false;
        for instance in &mut self.team_manager.teams {
            if instance.team_type == target && instance.active {
                instance.active = false;
                instance.suspended = true;
                disbanded = true;
            }
        }
        disbanded
    }

    fn autocreate(&mut self, house: u8, _team: Option<u16>) -> bool {
        // TACTION_AUTOCREATE: alert the house so its `Autocreate=yes` team
        // types start spawning (TEAMTYPE.CPP `IsAutocreate` + `IsAlerted`).
        let Some(entry) = self.houses.get_mut(usize::from(house)) else {
            return self.reject("Autocreate Begins");
        };
        entry.ai_enabled = true;
        entry.autocreate = true;
        true
    }

    fn all_hunt(&mut self, house: u8) -> bool {
        if self.houses.get(usize::from(house)).is_none() {
            return self.reject("All to Hunt");
        }
        self.apply_mission_action(&Action::AllToHunt { owner: house });
        true
    }

    fn drop_zone(&mut self, house: u8, cell: Vec2i) -> bool {
        if self.houses.get(usize::from(house)).is_none() {
            return self.reject("Drop Zone Flare (waypoint)");
        }
        self.apply_mission_action(&Action::Dropzone { owner: house, cell });
        true
    }

    fn fire_sale(&mut self, house: u8) -> bool {
        if self.houses.get(usize::from(house)).is_none() {
            return self.reject("Fire Sale");
        }
        let indices: Vec<usize> = self
            .buildings
            .iter()
            .enumerate()
            .filter(|(_, building)| building.owner == house && building.is_alive())
            .map(|(index, _)| index)
            .collect();
        // Sell from the back so an in-place removal cannot shift a pending
        // index.
        for index in indices.into_iter().rev() {
            self.sell_building(house, index);
        }
        true
    }

    fn text_trigger(&mut self, text: i64) -> bool {
        self.mission.messages.push(format!("text trigger {text}"));
        true
    }

    fn destroy_trigger(&mut self, trigger: Option<u16>) -> bool {
        let Some(id) = trigger.map(u32::from) else {
            return self.reject("Destroy Trigger");
        };
        if !self.mission.triggers.iter().any(|entry| entry.id == id) {
            return self.reject("Destroy Trigger");
        }
        self.mission.disable(id);
        true
    }

    fn force_trigger(&mut self, trigger: Option<u16>) -> bool {
        let Some(id) = trigger.map(u32::from) else {
            return self.reject("Force Trigger");
        };
        let actions = self.mission.fire(id);
        if actions.is_empty() {
            return self.reject("Force Trigger");
        }
        for action in actions {
            self.apply_mission_action(&action);
            if self.mission.is_over() {
                break;
            }
        }
        true
    }

    fn reveal_map(&mut self, house: u8) -> bool {
        if self.houses.get(usize::from(house)).is_none() {
            return self.reject("Reveal all map");
        }
        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                self.shroud.reveal(usize::from(house), Vec2i::new(x, y), 0);
            }
        }
        true
    }

    fn reveal_area(&mut self, house: u8, cell: Vec2i) -> bool {
        if self.houses.get(usize::from(house)).is_none() {
            return self.reject("Reveal around waypoint");
        }
        self.shroud.reveal(usize::from(house), cell, 4);
        true
    }

    fn reveal_zone(&mut self, house: u8, cell: Vec2i) -> bool {
        // TD stores trigger zones without linking them to waypoints, so the
        // zone action reveals the waypoint neighbourhood (same radius as
        // TACTION_REVEAL_SOME).
        self.reveal_area(house, cell)
    }

    fn set_global(&mut self, global: i64, on: bool) -> bool {
        let key = format!("global:{global}");
        if on {
            self.mission.flags.insert(key);
        } else {
            self.mission.flags.remove(&key);
        }
        true
    }

    fn base_building(&mut self, house: u8, enabled: bool) -> bool {
        match self.houses.get_mut(usize::from(house)) {
            Some(entry) => {
                entry.ai_enabled = enabled;
                true
            }
            None => self.reject("Auto Base Building"),
        }
    }

    fn special_weapon(&mut self, house: u8, weapon: i64, _one_shot: bool) -> bool {
        use cnc_core::trigger_apply::special_weapon as spc;
        let target = td_strike_cell(self, house);
        match weapon {
            spc::NUCLEAR_BOMB => {
                self.apply_mission_action(&Action::Nuke {
                    owner: house,
                    cell: target,
                });
                true
            }
            spc::PARA_BOMB => {
                self.apply_mission_action(&Action::Airstrike {
                    owner: house,
                    cell: target,
                });
                true
            }
            spc::PARA_INFANTRY => {
                self.apply_mission_action(&Action::Dropzone {
                    owner: house,
                    cell: target,
                });
                true
            }
            _ => self.reject("Add special weapon"),
        }
    }

    fn launch_nukes(&mut self, house: u8) -> bool {
        let has_temple = self.buildings.iter().any(|building| {
            building.owner == house
                && building.is_alive()
                && building.structure_type() == Some(StructureType::TempleOfNod)
        });
        if !has_temple {
            return self.reject("Launch Nukes");
        }
        let cell = td_strike_cell(self, house);
        self.apply_mission_action(&Action::Nuke { owner: house, cell });
        true
    }

    fn evaluate_event(
        &mut self,
        kind: cnc_core::trigger_records::EventKind,
        value: i64,
    ) -> Option<bool> {
        use cnc_core::trigger_apply::resolve_house;
        use cnc_core::trigger_records::EventKind;
        match kind {
            // The RAs/TD script editors store elapsed time in seconds.
            EventKind::ElapsedTime => Some(self.elapsed.to_f32() >= value as f32),
            EventKind::Credits => {
                let house = resolve_house(self, value);
                Some(
                    self.houses
                        .get(usize::from(house))
                        .is_some_and(|entry| i64::from(entry.available_money()) >= value),
                )
            }
            EventKind::UnitsDestroyed => {
                let house = resolve_house(self, value);
                Some(
                    !self
                        .units
                        .iter()
                        .any(|unit| unit.owner == house && unit.is_alive()),
                )
            }
            EventKind::BuildingsDestroyed => {
                let house = resolve_house(self, value);
                Some(
                    !self
                        .buildings
                        .iter()
                        .any(|building| building.owner == house && building.is_alive()),
                )
            }
            EventKind::AllDestroyed => {
                let house = resolve_house(self, value);
                Some(
                    !self
                        .units
                        .iter()
                        .any(|unit| unit.owner == house && unit.is_alive())
                        && !self
                            .buildings
                            .iter()
                            .any(|building| building.owner == house && building.is_alive()),
                )
            }
            EventKind::LowPower => {
                let house = resolve_house(self, value);
                Some(
                    self.houses
                        .get(usize::from(house))
                        .is_some_and(|entry| entry.power < 0),
                )
            }
            EventKind::GlobalSet => Some(self.mission.flags.contains(&format!("global:{value}"))),
            EventKind::GlobalClear => {
                Some(!self.mission.flags.contains(&format!("global:{value}")))
            }
            _ => None,
        }
    }
}

/// Aiming cell for a scripted strike: the opposing start position, falling
/// back to this map's first start and finally the map centre.
fn td_strike_cell(game: &Game, house: u8) -> Vec2i {
    let houses = game.houses.len().max(1);
    let opponent = (usize::from(house) + 1) % houses;
    game.map
        .player_starts
        .get(opponent)
        .or_else(|| game.map.player_starts.first())
        .copied()
        .unwrap_or_else(|| Vec2i::new(game.map.width as i32 / 2, game.map.height as i32 / 2))
}

#[cfg(test)]
mod trigger_apply_tests {
    use super::*;
    use cnc_core::trigger_apply::{apply_action_record, fire_trigger};
    use cnc_core::trigger_records::{ActionRecord, EventRecord};

    fn action(record: &str) -> ActionRecord {
        ActionRecord::parse(record).expect("action record parses")
    }

    fn event(record: &str) -> EventRecord {
        EventRecord::parse(record).expect("event record parses")
    }

    fn fresh() -> Game {
        let mut game = Game::new("trigger", 32, 32);
        // A bare Game has no houses; scenario loading installs them, so the
        // tests install the two campaign houses themselves.
        game.houses = vec![
            House::new(ClassId::HouseGDI, "GDI".to_string()),
            House::new(ClassId::HouseNod, "Nod".to_string()),
        ];
        game
    }

    #[test]
    fn trigger_airstrike_fires() {
        // TACTION_1_SPECIAL with SPC_PARA_BOMB: a scripted airstrike, the
        // Red Alert spelling of TD's ACTION_AIRSTRIKE.
        let mut game = fresh();
        let before = game.aircraft_shots;
        let report = apply_action_record(&mut game, action("33,-1,-1,3"));
        assert!(report.applied, "airstrike arm executes");
        assert_eq!(
            game.aircraft_shots,
            before + 1,
            "one strafing run was recorded"
        );
        assert!(!game.blasts.is_empty(), "the strike left blast visuals");
    }

    #[test]
    fn trigger_superweapon_arms_map_the_spc_payload() {
        let mut game = fresh();
        let blasts = game.blasts.len();
        assert!(apply_action_record(&mut game, action("34,-1,-1,1")).applied);
        assert!(game.blasts.len() > blasts, "nuke spawned its blast");

        let mut game = fresh();
        assert!(apply_action_record(&mut game, action("33,0,-1,1")).applied);
        assert!(
            !game.blasts.is_empty(),
            "scripted nuke ran through the nuke arm"
        );

        let mut game = fresh();
        // SPC_SONAR_PULSE has no TD arm: it is rejected, not silently applied.
        let report = apply_action_record(&mut game, action("34,-1,-1,0"));
        assert!(!report.applied);
        assert!(game
            .mission
            .messages
            .iter()
            .any(|line| line.contains("no arm")));
    }

    #[test]
    fn trigger_win_and_lose_finish_the_mission() {
        let mut game = fresh();
        assert!(apply_action_record(&mut game, action("1,-1,-1,0")).applied);
        assert_eq!(game.mission.outcome, Some(MissionOutcome::Victory));

        let mut game = fresh();
        assert!(apply_action_record(&mut game, action("2,-1,-1,0")).applied);
        assert_eq!(game.mission.outcome, Some(MissionOutcome::Defeat));
    }

    #[test]
    fn trigger_families_mutate_their_state() {
        // ALLOWWIN flags the mission for later win checks.
        let mut game = fresh();
        assert!(apply_action_record(&mut game, action("15,-1,-1,0")).applied);
        assert!(game.mission.flags.contains("allowwin:0"));

        // ALL_TO_HUNT clears unit orders for the house.
        let mut game = fresh();
        for unit in &mut game.units {
            if unit.owner == game.player_owner {
                unit.target = Some(Vec2::new(Fixed16_16::ONE, Fixed16_16::ONE));
            }
        }
        assert!(apply_action_record(&mut game, action("6,-1,-1,0")).applied);
        assert!(game
            .units
            .iter()
            .filter(|unit| unit.owner == game.player_owner)
            .all(|unit| unit.target.is_none()));

        // REVEAL_ALL lights up the house's fog.
        let mut game = fresh();
        game.enable_shroud();
        game.shroud.clear_visible();
        let before = game.shroud.mapped_count(usize::from(game.player_owner));
        assert!(apply_action_record(&mut game, action("16,-1,-1,0")).applied);
        assert!(
            game.shroud.mapped_count(usize::from(game.player_owner)) > before,
            "reveal all mapped more cells"
        );

        // GLOBAL_SET / GLOBAL_CLEAR round-trip a mission flag.
        let mut game = fresh();
        assert!(apply_action_record(&mut game, action("28,-1,-1,7")).applied);
        assert!(game.mission.flags.contains("global:7"));
        assert!(apply_action_record(&mut game, action("29,-1,-1,7")).applied);
        assert!(!game.mission.flags.contains("global:7"));
    }

    #[test]
    fn trigger_unknown_code_is_a_logged_no_op() {
        let mut game = fresh();
        let units = game.units.len();
        let outcome = game.mission.outcome;
        let report = apply_action_record(&mut game, action("99,-1,-1,0"));
        assert!(!report.applied);
        assert_eq!(game.units.len(), units, "no unit touched");
        assert_eq!(game.mission.outcome, outcome, "mission continues");
        assert!(
            game.mission.messages.iter().any(|line| line.contains("99")),
            "the ignored code is logged: {:?}",
            game.mission.messages
        );
    }

    #[test]
    fn trigger_apply_is_deterministic() {
        let records = ["33,-1,-1,3", "15,-1,-1,0", "1,-1,-1,0", "99,-1,-1,0"];
        let mut first = fresh();
        let mut second = fresh();
        for record in records {
            apply_action_record(&mut first, action(record));
            apply_action_record(&mut second, action(record));
        }
        assert_eq!(first.aircraft_shots, second.aircraft_shots);
        assert_eq!(first.blasts.len(), second.blasts.len());
        assert_eq!(first.mission.flags, second.mission.flags);
        assert_eq!(first.mission.outcome, second.mission.outcome);
        assert_eq!(first.mission.messages, second.mission.messages);
    }

    #[test]
    fn trigger_events_gate_their_actions() {
        let actions = [action("1,-1,-1,0")];

        let mut game = fresh();
        let fired = fire_trigger(&mut game, &[event("13,-1,0")], &actions);
        assert!(fired.is_some(), "elapsed 0 >= 0 holds");
        assert_eq!(game.mission.outcome, Some(MissionOutcome::Victory));

        let mut game = fresh();
        let fired = fire_trigger(&mut game, &[event("13,-1,99999")], &actions);
        assert!(fired.is_none(), "elapsed 99999 does not hold yet");
        assert_eq!(game.mission.outcome, None, "no action ran");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_asset::{AtlasStyle, SpriteAtlas};
    use cnc_renderer::SoftwareRenderer;

    struct CountingRenderer {
        sprites: usize,
        /// Every rect drawn, so tests can inspect the fog overlay.
        rects: Vec<(f32, f32, f32, f32, [f32; 4])>,
    }

    impl CountingRenderer {
        fn new() -> Self {
            Self {
                sprites: 0,
                rects: Vec::new(),
            }
        }

        /// Rects whose alpha is the fog memory value or fully opaque black.
        fn fog_rects(&self) -> Vec<&(f32, f32, f32, f32, [f32; 4])> {
            self.rects
                .iter()
                .filter(|(_, _, _, _, color)| {
                    color[0] == 0.0
                        && color[1] == 0.0
                        && color[2] == 0.0
                        && (color[3] == 1.0 || (color[3] - SHROUD_MEMORY_ALPHA).abs() < 1.0e-6)
                })
                .collect()
        }
    }

    impl cnc_renderer::Renderer for CountingRenderer {
        fn begin_frame(&mut self) {}
        fn end_frame(&mut self) -> anyhow::Result<()> {
            Ok(())
        }
        fn draw_sprite(&mut self, _sprite: &cnc_renderer::Sprite) {
            self.sprites += 1;
        }
        fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
            self.rects.push((x, y, w, h, color));
        }
        fn draw_text(&mut self, _text: &str, _x: f32, _y: f32, _size: f32, _color: [f32; 4]) {}
        fn set_camera(&mut self, _camera: &cnc_renderer::Camera) {}
        fn resize(&mut self, _width: u32, _height: u32) {}
        fn set_palette(&mut self, _palette: &cnc_core::Palette) {}
        fn uses_sprites(&self) -> bool {
            true
        }
    }

    #[test]
    fn discovering_a_cell_is_recorded() {
        let mut game = Game::new("disc", 40, 40);
        game.setup_demo();
        game.enable_shroud();
        game.refresh_shroud();
        // enable_shroud wipes history, so the first refresh discovers
        // everything the player currently sees.
        assert!(
            game.any_discovery_pending(),
            "the first refresh must report discoveries"
        );
    }

    #[test]
    fn a_repeated_refresh_reports_no_new_discovery() {
        let mut game = Game::new("disc", 40, 40);
        game.setup_demo();
        game.enable_shroud();
        game.refresh_shroud();
        game.discovered_cells.clear();
        game.refresh_shroud();
        assert!(
            !game.any_discovery_pending(),
            "standing still discovers nothing new"
        );
    }

    #[test]
    fn moving_into_the_dark_discovers_new_cells() {
        let mut game = Game::new("disc", 40, 40);
        game.setup_demo();
        game.enable_shroud();
        game.refresh_shroud();
        game.discovered_cells.clear();

        // Teleport a unit into unexplored territory and refresh.
        let Some(unit) = game.units.iter_mut().find(|u| u.owner == 0) else {
            return;
        };
        unit.position = Game::cell_center(2, 2);
        game.refresh_shroud();
        assert!(
            game.any_discovery_pending(),
            "vision opening on new ground is a discovery"
        );
    }

    #[test]
    fn a_discovered_cell_matches_the_discovered_condition() {
        let mut game = Game::new("disc", 40, 40);
        game.setup_demo();
        game.enable_shroud();
        game.refresh_shroud();
        let cell = *game.discovered_cells.iter().next().expect("a discovery");
        let condition = crate::mission::Condition::Discovered { cell: Some(cell) };
        assert!(condition.matches(&game, &Default::default()));

        let elsewhere = crate::mission::Condition::Discovered {
            cell: Some(Vec2i::new(0, 0)),
        };
        assert!(!elsewhere.matches(&game, &Default::default()) || cell == Vec2i::new(0, 0));
    }

    #[test]
    fn discoveries_are_drained_after_trigger_evaluation() {
        let mut game = Game::new("disc", 40, 40);
        game.setup_demo();
        game.enable_shroud();
        game.refresh_shroud();
        assert!(game.any_discovery_pending());
        game.update_mission(Fixed16_16::ZERO);
        assert!(
            !game.any_discovery_pending(),
            "a discovery must not re-fire every tick"
        );
    }

    #[test]
    fn a_fully_revealed_game_draws_no_fog() {
        let game = Game::new("fog", 40, 40);
        let mut renderer = CountingRenderer::new();
        game.render_shroud(&mut renderer);
        assert!(
            renderer.fog_rects().is_empty(),
            "default games are fully revealed and must not draw fog"
        );
    }

    #[test]
    fn unrevealed_cells_draw_opaque_fog() {
        let mut game = Game::new("fog", 40, 40);
        game.enable_shroud();
        // Reveal a small patch; everything else stays unexplored.
        game.shroud.reveal(0, Vec2i::new(20, 20), 3);
        let mut renderer = CountingRenderer::new();
        game.render_shroud(&mut renderer);
        let fog = renderer.fog_rects();
        assert!(!fog.is_empty(), "hidden cells must be covered");
        assert!(
            fog.iter().any(|(_, _, _, _, c)| c[3] == 1.0),
            "unexplored cells are fully black"
        );
    }

    #[test]
    fn explored_but_unseen_cells_draw_partial_fog() {
        let mut game = Game::new("fog", 40, 40);
        game.enable_shroud();
        game.shroud.reveal(0, Vec2i::new(20, 20), 2);
        // Clear visibility but keep the mapped flags.
        game.shroud.clear_visible();
        let mut renderer = CountingRenderer::new();
        game.render_shroud(&mut renderer);
        let fog = renderer.fog_rects();
        assert!(
            fog.iter()
                .any(|(_, _, _, _, c)| (c[3] - SHROUD_MEMORY_ALPHA).abs() < 1.0e-6),
            "explored-but-unseen cells use the memory alpha"
        );
    }

    #[test]
    fn units_in_unseen_cells_are_not_drawn() {
        let mut game = Game::new("fog", 40, 40);
        game.enable_shroud();
        game.units.clear();
        // Hide the whole board, then place a unit in the dark.
        game.shroud.clear_visible();

        game.spawn_unit_at(0, UnitType::InfantryE1, Vec2i::new(30, 30));

        let mut renderer = CountingRenderer::new();
        game.render_with_sprites(&mut renderer, None);
        assert_eq!(
            renderer.sprites, 0,
            "a unit the player cannot see must not be drawn"
        );
    }

    #[test]
    fn atlas_covers_demo_classes() {
        let atlas = SpriteAtlas::generate(AtlasStyle::Neon);
        for key in [
            "InfantryE1",
            "VehicleE2",
            "VehicleE4",
            "VehicleE6",
            "AircraftE1",
            "ConstructionYard",
            "PowerPlant",
            "Refinery",
            "Barracks",
            "WarFactory",
            "Helipad",
            "Silo",
        ] {
            assert!(atlas.frame(key).is_some(), "atlas missing {key}");
        }
    }

    #[test]
    fn render_with_sprites_emits_unit_and_building_sprites() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let atlas = SpriteAtlas::generate(AtlasStyle::Neon);
        let mut renderer = CountingRenderer::new();

        game.render_with_sprites(&mut renderer, Some(&atlas));

        // Fog hides objects the player cannot see, so only objects standing
        // on visible cells are drawn.
        let visible = game
            .buildings
            .iter()
            .filter(|b| game.visible_to_player(Game::building_cell(b)))
            .count()
            + game
                .units
                .iter()
                .filter(|u| game.visible_to_player(Game::unit_cell(u)))
                .count();
        assert!(
            renderer.sprites >= visible,
            "expected at least {visible} sprites, got {}",
            renderer.sprites
        );
        assert!(visible > 0, "the demo must show the player something");
    }

    #[test]
    fn demo_game_updates_and_renders() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        assert!(game.unit_count() > 0, "demo should spawn units");
        assert!(game.building_count() > 0, "demo should spawn buildings");
        assert!(game.houses.len() >= 2, "demo should spawn houses");

        let enemy_index = game
            .units
            .iter()
            .position(|unit| unit.owner != 0)
            .expect("demo should spawn Nod units");
        let enemy_before = game.units[enemy_index].position;

        let before: Vec<Vec2> = game
            .units
            .iter()
            .filter(|unit| unit.owner == 0)
            .map(|unit| unit.position)
            .collect();
        game.update(Fixed16_16::from_int(1));
        assert!(
            game.units
                .iter()
                .filter(|unit| unit.owner == 0)
                .zip(before.iter())
                .any(|(unit, before)| unit.position != *before),
            "idle GDI units should patrol or reposition"
        );
        assert_eq!(
            game.units[enemy_index].position, enemy_before,
            "enemy units should hold position until ordered"
        );

        let mut renderer = SoftwareRenderer::new(800, 600);
        game.render(&mut renderer);
        assert!(
            renderer.buffer().iter().any(|pixel| *pixel != 0),
            "rendering the game should write pixels"
        );
    }

    #[test]
    fn move_order_routes_around_impassable_terrain() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(1, 8), 0));
        game.units[0].is_selected = true;

        // Solid wall at x = 8 with a gap at the bottom edge.
        for y in 0..16 {
            game.map.grid.set_passable(Vec2i::new(8, y), false);
        }
        game.map.grid.set_passable(Vec2i::new(8, 15), true);

        assert_eq!(game.order_move(Game::cell_center(15, 8)), 1);
        let path_cells: Vec<Vec2i> = game.units[0]
            .path
            .iter()
            .map(|point| {
                Vec2i::new(
                    (point.x.to_f32() / TILE_SIZE).floor() as i32,
                    (point.y.to_f32() / TILE_SIZE).floor() as i32,
                )
            })
            .collect();

        assert!(
            !path_cells.is_empty(),
            "a path around the wall should exist"
        );
        assert!(
            path_cells.iter().all(|cell| cell.x != 8 || cell.y == 15),
            "path must only cross the wall at the gap"
        );
        assert!(path_cells.iter().any(|cell| cell.y == 15));
    }

    #[test]
    fn move_order_avoids_building_cells() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::MediumTank, Game::cell_center(1, 8), 0));
        game.units[0].is_selected = true;
        game.buildings.push(Building::new(
            ClassId::ConstructionYard,
            Game::cell_center(8, 8),
            Fixed16_16::from_int(500),
            0,
        ));

        assert_eq!(game.order_move(Game::cell_center(15, 8)), 1);
        let path_cells: Vec<Vec2i> = game.units[0]
            .path
            .iter()
            .map(|point| {
                Vec2i::new(
                    (point.x.to_f32() / TILE_SIZE).floor() as i32,
                    (point.y.to_f32() / TILE_SIZE).floor() as i32,
                )
            })
            .collect();

        assert!(!path_cells.is_empty());
        assert!(
            !path_cells.contains(&Vec2i::new(8, 8)),
            "path must not go through the building cell"
        );
    }

    #[test]
    fn explosion_damage_hits_nearby_enemies() {
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        let tank_position = Vec2::new(Fixed16_16::from_int(500), Fixed16_16::from_int(600));
        let enemy_a = Vec2::new(Fixed16_16::from_int(600), Fixed16_16::from_int(600));
        let enemy_b = Vec2::new(Fixed16_16::from_int(615), Fixed16_16::from_int(600));

        game.units
            .push(Unit::new(UnitType::MediumTank, tank_position, 0));
        game.units.push(Unit::new(UnitType::InfantryE1, enemy_a, 1));
        game.units.push(Unit::new(UnitType::InfantryE1, enemy_b, 1));
        game.units[0].is_selected = true;

        assert_eq!(game.order_attack(1), 1);
        let start = [game.units[1].health, game.units[2].health];

        for _ in 0..200 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        assert!(
            game.units[1].health < start[0],
            "direct target should take damage"
        );
        assert!(
            game.units[2].health < start[1],
            "adjacent enemy should take explosion damage"
        );
    }

    #[test]
    fn idle_units_auto_acquire_nearby_enemy() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.update(Fixed16_16::from_f32(0.05));

        assert!(
            game.units
                .iter()
                .any(|unit| unit.owner == 0 && unit.attack_target.is_some()),
            "guarding GDI units should acquire an enemy within threat range"
        );
    }

    #[test]
    fn world_center_is_map_middle() {
        let game = Game::new("test", 64, 64);
        let center = game.world_center();
        assert!((center.x.to_f32() - 64.0 * TILE_SIZE * 0.5).abs() < 0.01);
        assert!((center.y.to_f32() - 64.0 * TILE_SIZE * 0.5).abs() < 0.01);
    }

    #[test]
    fn click_selects_nearest_unit() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let target = game.units[3].position;
        assert_eq!(game.select_at(target), Some(3));
        assert!(game.units[3].is_selected);
        assert_eq!(game.status().selected, 1);
    }

    #[test]
    fn click_on_empty_ground_clears_selection() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.select_all_owned(0);
        assert!(game.status().selected > 0);

        let empty = Vec2::new(Fixed16_16::from_int(-500), Fixed16_16::from_int(-500));
        assert_eq!(game.select_at(empty), None);
        assert_eq!(game.status().selected, 0);
    }

    #[test]
    fn move_order_drives_unit_to_target() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let index = 2;
        for unit in &mut game.units {
            unit.is_selected = false;
        }
        game.units[index].is_selected = true;
        let start = game.units[index].position;
        let target = Vec2::new(
            start.x + Fixed16_16::from_int(120),
            start.y + Fixed16_16::from_int(60),
        );

        assert_eq!(game.order_move(target), 1);
        assert!(game.units[index].ordered);
        let destination = game.units[index]
            .target
            .expect("move order should set a target");
        assert_ne!(destination, start);

        for _ in 0..2000 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        let dx = game.units[index].position.x.to_f32() - destination.x.to_f32();
        let dy = game.units[index].position.y.to_f32() - destination.y.to_f32();
        assert!(
            dx.abs() < 5.0 && dy.abs() < 5.0,
            "unit should arrive, offset {dx},{dy}"
        );
        assert!(game.units[index].target.is_none());
    }

    #[test]
    fn move_order_draws_target_marker() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let marker_target = Vec2::new(Fixed16_16::from_int(100), Fixed16_16::from_int(100));
        game.units[0].target = Some(marker_target);
        game.units[0].ordered = true;

        let mut renderer = SoftwareRenderer::new(800, 600);
        game.render(&mut renderer);

        // [1.0, 0.9, 0.2, 1.0] packed by SoftwareRenderer as ARGB.
        let marker_color = (255u32 << 24) | (255u32 << 16) | (229u32 << 8) | 51u32;
        assert!(
            renderer.buffer().contains(&marker_color),
            "expected a yellow move-order marker in the rendered frame"
        );
    }

    #[test]
    fn box_selection_selects_multiple_units() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let center = game.patrol_center;
        let offset = Fixed16_16::from_int(300);
        let min = Vec2::new(center.x - offset, center.y - offset);
        let max = Vec2::new(center.x + offset, center.y + offset);
        let count = game.select_in_rect(min, max, false);

        assert!(
            count >= 10,
            "box should select the GDI cluster, got {count}"
        );
        assert_eq!(game.status().selected, count);
        assert!(
            game.units
                .iter()
                .filter(|u| u.owner == 0)
                .filter(|u| u.is_selected)
                .count()
                >= 10
        );
    }

    #[test]
    fn shift_click_adds_to_selection() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let first = game.units[0].position;
        assert_eq!(game.select_at(first), Some(0));
        let second = game.units[5].position;
        assert_eq!(game.select_at_additive(second, true), Some(5));
        assert_eq!(game.status().selected, 2);
        assert!(game.units[0].is_selected);
        assert!(game.units[5].is_selected);
    }

    #[test]
    fn group_move_uses_nearby_cells() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert_eq!(game.select_all_owned(0), 14);

        let world = Vec2::new(Fixed16_16::from_int(700), Fixed16_16::from_int(700));
        assert_eq!(game.order_move(world), 14);

        let gdi: Vec<&Unit> = game.units.iter().filter(|u| u.owner == 0).collect();
        let mut targets = Vec::new();
        for unit in &gdi {
            let target = unit.target.expect("every selected unit gets a target");
            let x = target.x.to_f32();
            let y = target.y.to_f32();
            targets.push((x, y));

            // Nearby_Location returns cell centres.
            let cell_x = ((x - TILE_SIZE * 0.5) / TILE_SIZE).round();
            let cell_y = ((y - TILE_SIZE * 0.5) / TILE_SIZE).round();
            assert!((x - (cell_x * TILE_SIZE + TILE_SIZE * 0.5)).abs() < 0.01);
            assert!((y - (cell_y * TILE_SIZE + TILE_SIZE * 0.5)).abs() < 0.01);
            assert!((x - world.x.to_f32()).abs() <= 16.0 * TILE_SIZE);
            assert!((y - world.y.to_f32()).abs() <= 16.0 * TILE_SIZE);
        }

        for i in 0..targets.len() {
            for j in (i + 1)..targets.len() {
                assert!(
                    targets[i] != targets[j],
                    "formation targets must be distinct"
                );
            }
        }
    }

    #[test]
    fn enemy_at_finds_nod_unit() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let (index, position) = game
            .units
            .iter()
            .enumerate()
            .find(|(_, unit)| unit.owner != 0)
            .map(|(index, unit)| (index, unit.position))
            .expect("demo should spawn Nod units");
        assert_eq!(game.enemy_at(position), Some(index));
    }

    #[test]
    fn attack_order_fires_projectiles_and_damages_enemy() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert_eq!(game.select_all_owned(0), 14);

        let enemy = game
            .units
            .iter()
            .position(|unit| unit.owner != 0)
            .expect("demo should spawn Nod units");
        assert!(game.order_attack(enemy) > 0);

        let start_health = game.units[enemy].health;
        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        assert!(
            game.projectiles_fired > 0,
            "attack order should spawn projectiles"
        );
        assert!(
            game.units[enemy].health < start_health,
            "the enemy should have taken damage"
        );
    }

    #[test]
    fn turret_rotates_before_firing() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::MediumTank, Game::cell_center(8, 12), 0));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(8, 4), 1));
        game.units[0].is_selected = true;
        assert_eq!(game.order_attack(1), 1);
        assert_eq!(game.units[0].turret_facing, 0.0);

        // First short step: out of range and still facing east.
        game.update(Fixed16_16::from_f32(0.05));
        assert!(
            game.units[0].turret_facing < 0.0,
            "turret should start rotating toward the northern target"
        );
        assert_eq!(game.projectiles_fired, 0);

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.projectiles_fired > 0 {
                break;
            }
        }

        assert!(
            game.projectiles_fired > 0,
            "tank should fire once aligned and in range"
        );
        assert!(
            (game.units[0].turret_facing + std::f32::consts::FRAC_PI_2).abs() < 0.2,
            "turret should end up facing north"
        );
    }

    #[test]
    fn setup_demo_computes_house_power() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();

        let status = game.status();
        assert_eq!(status.gdi_power, 140);
        assert_eq!(status.gdi_drain, 115);
        assert_eq!(status.nod_power, 140);
        assert_eq!(status.nod_drain, 55);
        assert_eq!(status.gdi_capacity, 1000);
        assert_eq!(status.nod_capacity, 1000);
    }

    #[test]
    fn queue_unit_spends_credits_and_spawns_at_factory() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.units.clear();

        let credits = game.status().gdi_credits;
        assert!(game.queue_unit(0, UnitType::InfantryE1));
        let status = game.status();
        assert_eq!(status.gdi_credits, credits - 100);
        assert_eq!(status.production, 1);

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.units.is_empty() {
                break;
            }
        }

        assert_eq!(game.units.len(), 1, "production should spawn one unit");
        assert_eq!(game.units[0].owner, 0);
        assert_eq!(game.units[0].unit_type, UnitType::InfantryE1);
        assert_eq!(game.status().production, 0);
    }

    #[test]
    fn queue_requires_matching_factory() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let nod_credits = game.status().nod_credits;

        assert!(!game.queue_unit(1, UnitType::InfantryE1));
        assert!(!game.queue_unit(1, UnitType::MediumTank));
        assert_eq!(game.status().nod_credits, nod_credits);
        assert_eq!(game.status().production, 0);
    }

    #[test]
    fn tiberium_grows_and_spreads() {
        let mut game = Game::new("test", 8, 8);
        for cell in &mut game.map.grid.cells {
            cell.has_tiberium = false;
            cell.tiberium_amount = 0;
        }
        game.map.grid.set_tiberium(Vec2i::new(2, 2), 200, 1);

        let before = game.map.grid.get(Vec2i::new(2, 2)).unwrap().tiberium_amount;
        game.map.grid.tiberium_cycle();

        let grown = game.map.grid.get(Vec2i::new(2, 2)).unwrap().tiberium_amount;
        assert!(grown > before, "existing tiberium should grow");
        assert!(
            game.map.grid.get(Vec2i::new(3, 2)).unwrap().has_tiberium,
            "heavy tiberium should spread to a clear neighbour"
        );
    }

    #[test]
    fn harvester_harvests_tiberium() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        for cell in &mut game.map.grid.cells {
            cell.has_tiberium = false;
            cell.tiberium_amount = 0;
        }
        game.map.grid.set_tiberium(Vec2i::new(5, 5), 255, 1);

        let mut harvester = Unit::new(UnitType::Harvester, Game::cell_center(5, 5), 0);
        harvester.ordered = true;
        game.units.push(harvester);

        let before = game.map.grid.get(Vec2i::new(5, 5)).unwrap().tiberium_amount;
        for _ in 0..50 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.units[0].tiberium > 0 {
                break;
            }
        }

        assert!(
            game.units[0].tiberium > 0,
            "harvester should pick up tiberium"
        );
        let after = game.map.grid.get(Vec2i::new(5, 5)).unwrap().tiberium_amount;
        assert!(after < before, "the tiberium cell should be reduced");
    }

    #[test]
    fn harvester_unloads_at_refinery() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        game.buildings.push(Building::from_type(
            StructureType::Refinery,
            Game::cell_center(4, 4),
            0,
        ));
        game.refresh_house_power();

        let mut harvester = Unit::new(UnitType::Harvester, Game::cell_center(6, 4), 0);
        harvester.tiberium = HARVESTER_STEP_COUNT;
        harvester.ordered = true;
        game.units.push(harvester);

        for _ in 0..500 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.units[0].tiberium == 0 {
                break;
            }
        }

        assert_eq!(game.units[0].tiberium, 0, "harvester should unload fully");
        assert_eq!(
            game.houses[0].tiberium,
            HARVESTER_STEP_COUNT as i32 * HARVESTER_STEP_CREDITS,
            "each step should add 25 credits of stored tiberium"
        );
        assert_eq!(
            game.houses[0].credits, 0,
            "initial cash should be untouched"
        );
    }

    #[test]
    fn queue_building_requires_construction_yard() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.buildings
            .retain(|building| building.structure_type() != Some(StructureType::ConstructionYard));

        let credits = game.status().gdi_credits;
        assert!(!game.queue_building(0, StructureType::PowerPlant));
        assert_eq!(game.status().gdi_credits, credits);
    }

    #[test]
    fn queue_building_completes_to_ready() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        let credits = game.status().gdi_credits;

        assert!(game.queue_building(0, StructureType::PowerPlant));
        assert_eq!(game.status().gdi_credits, credits - 300);
        assert!(game.ready_building_for(0).is_none());

        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.ready_building_for(0).is_some() {
                break;
            }
        }

        assert_eq!(game.ready_building_for(0), Some(StructureType::PowerPlant));
        assert_eq!(game.status().production, 0);
    }

    #[test]
    fn place_ready_building_adds_structure() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert!(game.queue_building(0, StructureType::PowerPlant));
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.ready_building_for(0).is_some() {
                break;
            }
        }

        let mut cell = None;
        'search: for y in 0..32 {
            for x in 0..32 {
                let candidate = Vec2i::new(x, y);
                if game.can_place_building(candidate) {
                    cell = Some(candidate);
                    break 'search;
                }
            }
        }
        let cell = cell.expect("there should be a free buildable cell");

        let buildings_before = game.building_count();
        let power_before = game.status().gdi_power;
        assert!(game.place_ready_building(0, cell));
        assert_eq!(game.building_count(), buildings_before + 1);
        assert_eq!(game.status().gdi_power, power_before + 100);
        assert!(game.ready_building_for(0).is_none());
    }

    #[test]
    fn place_building_rejects_occupied_cell() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert!(game.queue_building(0, StructureType::PowerPlant));
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.1));
            if game.ready_building_for(0).is_some() {
                break;
            }
        }

        let occupied = game
            .buildings
            .iter()
            .find(|building| building.owner == 0)
            .map(Game::building_cell)
            .expect("GDI has buildings");

        assert!(!game.place_ready_building(0, occupied));
        assert_eq!(game.ready_building_for(0), Some(StructureType::PowerPlant));
    }

    #[test]
    fn house_spends_tiberium_before_credits() {
        let mut house = House::new(ClassId::HouseGDI, "GDI".to_string());
        house.add_credits(100);
        house.capacity = 1000;
        house.harvested(500);

        assert_eq!(house.available_money(), 600);
        assert!(house.spend_credits(300));
        assert_eq!(house.tiberium, 200);
        assert_eq!(house.credits, 100);

        assert!(house.spend_credits(300));
        assert_eq!(house.tiberium, 0);
        assert_eq!(house.credits, 0);
        assert!(!house.spend_credits(1));
    }

    #[test]
    fn house_harvested_caps_at_capacity() {
        let mut house = House::new(ClassId::HouseGDI, "GDI".to_string());
        house.capacity = 1000;
        assert_eq!(house.harvested(1500), 1000);
        assert_eq!(house.tiberium, 1000);
        assert_eq!(house.harvested_credits, 1500);
    }

    #[test]
    fn silo_increases_storage_capacity() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        assert_eq!(game.status().gdi_capacity, 1000);

        game.buildings.push(Building::from_type(
            StructureType::Silo,
            Game::cell_center(6, 6),
            0,
        ));
        game.refresh_house_power();
        assert_eq!(game.status().gdi_capacity, 2500);
    }

    fn mammoth_duel(range: f32, expected: WeaponType) {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        let position = Game::cell_center(5, 8);
        let target = Vec2::new(
            Fixed16_16::from_f32(position.x.to_f32() + range),
            position.y,
        );
        game.units
            .push(Unit::new(UnitType::MammothTank, position, 0));
        game.units.push(Unit::new(UnitType::InfantryE1, target, 1));
        game.units[0].is_selected = true;
        assert_eq!(game.order_attack(1), 1);

        for _ in 0..20 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.projectiles.is_empty() {
                break;
            }
        }

        assert_eq!(
            game.projectiles[0].weapon.weapon_type, expected,
            "expected {:?} at range {range}",
            expected
        );
    }

    #[test]
    fn mammoth_uses_primary_inside_range_and_tusk_outside() {
        mammoth_duel(100.0, WeaponType::Cannon120);
        mammoth_duel(117.0, WeaponType::MammothTusk);
    }

    /// A game with houses set up and exactly one player-owned structure.
    ///
    /// `Game::new` leaves `houses` empty — only `setup_demo` populates them —
    /// so any test that touches credits, power or EVA must seed them first.
    fn eva_game_with_player_building() -> (Game, usize) {
        let mut game = Game::new("eva", 40, 40);
        game.setup_demo();
        game.player_owner = 0;
        game.buildings.clear();
        game.buildings.push(Building::from_type(
            StructureType::PowerPlant,
            Game::cell_center(10, 10),
            0,
        ));
        (game, 0)
    }

    #[test]
    fn a_damaged_player_structure_announces_base_under_attack() {
        let (mut game, id) = eva_game_with_player_building();
        game.announce_base_attack();
        assert_eq!(game.eva.pending(), None, "an undamaged base is quiet");

        game.buildings[id].health = game.buildings[id].max_health - Fixed16_16::ONE;
        game.announce_base_attack();
        assert_eq!(
            game.eva.pending(),
            Some(cnc_audio::EvaLine::BaseUnderAttack)
        );
    }

    #[test]
    fn base_under_attack_respects_the_speech_cooldown() {
        let (mut game, id) = eva_game_with_player_building();
        game.buildings[id].health = game.buildings[id].max_health - Fixed16_16::ONE;
        game.announce_base_attack();
        game.eva.stop();
        // Immediately again: the cooldown must suppress it.
        game.announce_base_attack();
        assert_eq!(game.eva.pending(), None, "cooldown not yet elapsed");

        game.elapsed = Fixed16_16::from_f32(BASE_ATTACK_SPEAK_DELAY + 1.0);
        game.announce_base_attack();
        assert_eq!(
            game.eva.pending(),
            Some(cnc_audio::EvaLine::BaseUnderAttack)
        );
    }

    #[test]
    fn insufficient_funds_requires_a_factory() {
        let mut game = Game::new("eva", 40, 40);
        game.player_owner = 0;
        game.buildings.retain(|b| b.owner != 0);
        if let Some(house) = game.houses.get_mut(0) {
            house.credits = 0;
        }
        game.announce_insufficient_funds();
        assert_eq!(game.eva.pending(), None, "no factory, no announcement");
    }

    #[test]
    fn insufficient_funds_announces_when_broke_with_a_factory() {
        let (mut game, _) = eva_game_with_player_building();
        if let Some(house) = game.houses.get_mut(0) {
            house.credits = INSUFFICIENT_FUNDS_THRESHOLD - 1;
        }
        game.announce_insufficient_funds();
        assert_eq!(
            game.eva.pending(),
            Some(cnc_audio::EvaLine::InsufficientFunds)
        );
    }

    #[test]
    fn a_full_purse_is_not_announced() {
        let (mut game, _) = eva_game_with_player_building();
        game.houses[0].credits = INSUFFICIENT_FUNDS_THRESHOLD * 10;
        game.announce_insufficient_funds();
        assert_eq!(game.eva.pending(), None);
    }

    #[test]
    fn superweapon_becoming_ready_announces_once() {
        let (mut game, _) = eva_game_with_player_building();
        game.buildings.clear();
        game.buildings.push(Building::from_type(
            StructureType::IonCannon,
            Game::cell_center(12, 12),
            0,
        ));
        if let Some(house) = game.houses.get_mut(0) {
            house.ion_charge = Fixed16_16::from_f32(0.99);
        }
        game.update_superweapons(Fixed16_16::from_f32(10.0));
        assert_eq!(game.eva.pending(), Some(cnc_audio::EvaLine::IonCannonReady));

        game.eva.stop();
        game.update_superweapons(Fixed16_16::from_f32(10.0));
        assert_eq!(game.eva.pending(), None, "already ready, must not repeat");
    }

    #[test]
    fn edge_entry_cell_follows_the_configured_side() {
        let mut game = Game::new("edge", 40, 40);
        game.setup_demo();
        let (cx, cy) = (40 / 2, 40 / 2);

        game.houses[0].edge = ReinforcementEdge::North;
        assert_eq!(game.edge_entry_cell(0), (cx, 1));
        game.houses[0].edge = ReinforcementEdge::South;
        assert_eq!(game.edge_entry_cell(0), (cx, 38));
        game.houses[0].edge = ReinforcementEdge::West;
        assert_eq!(game.edge_entry_cell(0), (1, cy));
        game.houses[0].edge = ReinforcementEdge::East;
        assert_eq!(game.edge_entry_cell(0), (38, cy));
    }

    #[test]
    fn non_compass_edges_fall_back_to_the_map_centre() {
        let mut game = Game::new("edge", 40, 40);
        game.setup_demo();
        for edge in [
            ReinforcementEdge::Air,
            ReinforcementEdge::Visible,
            ReinforcementEdge::EnemyBase,
            ReinforcementEdge::HomeBase,
        ] {
            game.houses[0].edge = edge;
            assert_eq!(game.edge_entry_cell(0), (20, 20), "{edge:?}");
        }
    }

    #[test]
    fn spawn_position_uses_the_edge_when_no_factory_exists() {
        let mut game = Game::new("edge", 40, 40);
        game.setup_demo();
        game.buildings.retain(|b| b.owner != 1);
        game.houses[1].edge = ReinforcementEdge::West;
        let position = game.spawn_position(1);
        let cell = Game::world_cell(position);
        assert_eq!(cell.x, 1, "enters from the west edge");
    }
    #[test]
    fn house_eliminated_fires_only_when_everything_is_gone() {
        let mut game = Game::new("cond", 40, 40);
        game.setup_demo();
        let cond = crate::mission::Condition::HouseEliminated {
            owner: 1,
            what: crate::mission::Eliminated::Units,
        };
        let flags = Default::default();
        assert!(!cond.matches(&game, &flags), "Nod still fields units");
        for unit in game.units.iter_mut().filter(|u| u.owner == 1) {
            unit.health = Fixed16_16::ZERO;
        }
        assert!(cond.matches(&game, &flags), "all Nod units dead");
    }

    #[test]
    fn kills_are_credited_to_the_opposing_house() {
        let mut game = Game::new("cond", 40, 40);
        game.setup_demo();
        game.player_owner = 0;
        let id = game.spawn_unit_at(1, UnitType::InfantryE1, Vec2i::new(20, 20));
        game.units[id as usize].health = Fixed16_16::ZERO;
        let before = game.kills_by(0, crate::mission::Eliminated::Units);
        game.update_death_announcements();
        assert_eq!(
            game.kills_by(0, crate::mission::Eliminated::Units),
            before + 1,
            "GDI is credited with the kill"
        );
    }

    #[test]
    fn no_factories_detects_a_house_without_a_producer() {
        let mut game = Game::new("cond", 40, 40);
        game.setup_demo();
        let cond = crate::mission::Condition::NoFactories { owner: 0 };
        let flags = Default::default();
        assert!(!cond.matches(&game, &flags), "GDI owns factories");
        for building in game.buildings.iter_mut().filter(|b| b.owner == 0) {
            building.health = Fixed16_16::ZERO;
        }
        assert!(cond.matches(&game, &flags), "no GDI factories remain");
    }
    #[test]
    fn reinforcement_edge_parses_the_original_tokens() {
        use ReinforcementEdge::*;
        assert_eq!(ReinforcementEdge::from_name("North"), Some(North));
        assert_eq!(ReinforcementEdge::from_name("EAST"), Some(East));
        assert_eq!(ReinforcementEdge::from_name("south"), Some(South));
        assert_eq!(ReinforcementEdge::from_name("West"), Some(West));
        assert_eq!(ReinforcementEdge::from_name("Air"), Some(Air));
        assert_eq!(ReinforcementEdge::from_name("HomeBase"), Some(HomeBase));
        assert_eq!(ReinforcementEdge::from_name(""), Some(North), "default");
        assert_eq!(ReinforcementEdge::from_name("nonsense"), None);
    }
    #[test]
    fn max_unit_limit_blocks_further_queues() {
        let mut game = Game::new("limits", 40, 40);
        game.setup_demo();
        game.houses[0].max_units = game
            .units
            .iter()
            .filter(|u| u.owner == 0 && u.is_alive())
            .count() as u32;
        assert!(!game.within_unit_limit(0), "already at the cap");
        assert!(!game.can_queue_unit(0, UnitType::MediumTank));
    }

    #[test]
    fn max_unit_limit_counts_queued_units() {
        let mut game = Game::new("limits", 40, 40);
        game.setup_demo();
        let alive = game
            .units
            .iter()
            .filter(|u| u.owner == 0 && u.is_alive())
            .count() as u32;
        game.houses[0].max_units = alive + 1;
        game.houses[0].credits = 100_000;
        assert!(game.within_unit_limit(0), "one slot free");
        game.queue_unit(0, UnitType::MediumTank);
        assert!(
            !game.within_unit_limit(0),
            "the queued order fills the last slot"
        );
    }

    #[test]
    fn max_building_limit_blocks_further_structures() {
        let mut game = Game::new("limits", 40, 40);
        game.setup_demo();
        game.houses[0].max_buildings = game
            .buildings
            .iter()
            .filter(|b| b.owner == 0 && b.is_alive())
            .count() as u32;
        assert!(!game.within_building_limit(0));
        assert!(!game.can_queue_building(0, StructureType::PowerPlant));
    }
    #[test]
    fn rules_sight_overrides_the_builtin_td_table() {
        let mut game = Game::new("sight", 40, 40);
        game.setup_demo();
        // A single-house Rules.ini carrying a Sight override for E1.
        let ini = cnc_core::IniFile::parse("[E1]\nSight=9\n").expect("ini");
        game.set_rules(cnc_core::RulesDatabase::from_ini(&ini));
        let Some(unit) = game
            .units
            .iter()
            .find(|u| u.unit_type == UnitType::InfantryE1)
        else {
            return;
        };
        assert_eq!(game.unit_sight_range(unit), 9, "Rules.ini Sight wins");
    }

    #[test]
    fn rules_sight_is_clamped_to_the_engine_maximum() {
        let mut game = Game::new("sight", 40, 40);
        game.setup_demo();
        let ini = cnc_core::IniFile::parse("[E1]\nSight=99\n").expect("ini");
        game.set_rules(cnc_core::RulesDatabase::from_ini(&ini));
        let Some(unit) = game
            .units
            .iter()
            .find(|u| u.unit_type == UnitType::InfantryE1)
        else {
            return;
        };
        assert_eq!(
            game.unit_sight_range(unit),
            cnc_core::shroud::MAX_SIGHT_RADIUS,
            "an absurd Sight cannot reveal the whole map"
        );
    }
    #[test]
    fn queueing_an_unaffordable_unit_announces_insufficient_funds() {
        let mut game = Game::new("eva", 40, 40);
        game.setup_demo();
        game.player_owner = 0;
        game.houses[0].credits = 0;
        game.houses[0].tiberium = 0;
        let queued = game.queue_unit(0, UnitType::MediumTank);
        assert!(!queued, "cannot afford it");
        assert_eq!(
            game.eva.pending(),
            Some(cnc_audio::EvaLine::InsufficientFunds)
        );
    }
    #[test]
    fn losing_an_owned_unit_announces_unit_lost() {
        let mut game = Game::new("eva", 40, 40);
        game.player_owner = 0;
        let id = game.spawn_unit_at(0, UnitType::InfantryE1, Vec2i::new(10, 10));
        game.update_death_announcements();
        assert_eq!(game.eva.pending(), None, "a live unit says nothing");

        game.units[id as usize].health = Fixed16_16::ZERO;
        game.update_death_announcements();
        assert_eq!(game.eva.pending(), Some(cnc_audio::EvaLine::UnitLost));
    }

    #[test]
    fn destroying_an_enemy_unit_announces_enemy_unit() {
        let mut game = Game::new("eva", 40, 40);
        game.player_owner = 0;
        let id = game.spawn_unit_at(1, UnitType::InfantryE1, Vec2i::new(10, 10));
        game.units[id as usize].health = Fixed16_16::ZERO;
        game.update_death_announcements();
        assert_eq!(
            game.eva.pending(),
            Some(cnc_audio::EvaLine::EnemyUnitDestroyed)
        );
    }

    #[test]
    fn a_death_is_announced_only_once() {
        let mut game = Game::new("eva", 40, 40);
        game.player_owner = 0;
        let id = game.spawn_unit_at(0, UnitType::InfantryE1, Vec2i::new(10, 10));
        game.units[id as usize].health = Fixed16_16::ZERO;
        game.update_death_announcements();
        assert!(game.eva.pending().is_some());
        game.eva.stop();
        game.update_death_announcements();
        assert_eq!(game.eva.pending(), None, "the same corpse must not repeat");
    }

    #[test]
    fn losing_an_owned_structure_announces_structure_lost() {
        let mut game = Game::new("eva", 40, 40);
        game.player_owner = 0;
        let id = game
            .buildings
            .iter()
            .position(|b| b.owner == 0)
            .unwrap_or_else(|| {
                let before = game.buildings.len();
                game.buildings.push(crate::building::Building::from_type(
                    StructureType::PowerPlant,
                    Game::cell_center(5, 5),
                    0,
                ));
                before
            });
        game.buildings[id].health = Fixed16_16::ZERO;
        game.update_death_announcements();
        assert_eq!(game.eva.pending(), Some(cnc_audio::EvaLine::StructureLost));
    }

    #[test]
    fn winning_the_mission_queues_eva_speech() {
        let mut game = Game::new("eva", 40, 40);
        assert_eq!(game.eva.pending(), None, "nothing queued at start");
        game.apply_mission_action(&Action::Win);
        assert_eq!(
            game.eva.pending(),
            Some(cnc_audio::EvaLine::MissionAccomplished)
        );
    }

    #[test]
    fn losing_the_mission_queues_failure_speech() {
        let mut game = Game::new("eva", 40, 40);
        game.apply_mission_action(&Action::Lose);
        assert_eq!(game.eva.pending(), Some(cnc_audio::EvaLine::MissionFailed));
    }

    #[test]
    fn airstrike_action_damages_units_in_the_target_cell() {
        let mut game = Game::new("strike", 40, 40);
        let victim = game.spawn_unit_at(0, UnitType::InfantryE1, Vec2i::new(20, 20));
        let before = game.units[victim as usize].health;
        game.apply_mission_action(&Action::Airstrike {
            owner: 1,
            cell: Vec2i::new(20, 20),
        });
        let after = game.units[victim as usize].health;
        assert!(after < before, "airstrike must damage the unit in the cell");
        assert_eq!(game.aircraft_shots, 1, "the run is counted");
    }

    #[test]
    fn nuke_action_does_not_require_a_temple() {
        let mut game = Game::new("nuke", 40, 40);
        let victim = game.spawn_unit_at(0, UnitType::InfantryE1, Vec2i::new(20, 20));
        let before = game.units[victim as usize].health;
        // No Temple of Nod exists for house 1.
        game.apply_mission_action(&Action::Nuke {
            owner: 1,
            cell: Vec2i::new(20, 20),
        });
        let after = game.units[victim as usize].health;
        assert!(after < before, "scripted nuke ignores the building gate");
    }

    #[test]
    fn ion_action_does_not_require_the_ion_cannon_building() {
        let mut game = Game::new("ion", 40, 40);
        let victim = game.spawn_unit_at(0, UnitType::InfantryE1, Vec2i::new(20, 20));
        let before = game.units[victim as usize].health;
        game.apply_mission_action(&Action::IonCannon {
            owner: 0,
            cell: Vec2i::new(20, 20),
        });
        let after = game.units[victim as usize].health;
        assert!(
            after < before,
            "scripted ion strike ignores the charge gate"
        );
        assert!(!game.beams.is_empty(), "the beam is drawn");
    }

    #[test]
    fn dropzone_action_spawns_infantry() {
        let mut game = Game::new("dz", 40, 40);
        let before = game.units.iter().filter(|u| u.owner == 1).count();
        game.apply_mission_action(&Action::Dropzone {
            owner: 1,
            cell: Vec2i::new(20, 20),
        });
        let after = game.units.iter().filter(|u| u.owner == 1).count();
        assert_eq!(after - before, DROPZONE_UNITS);
    }

    #[test]
    fn all_to_hunt_clears_orders_for_armed_units_only() {
        let mut game = Game::new("hunt", 40, 40);
        let armed = game.spawn_unit_at(0, UnitType::MediumTank, Vec2i::new(10, 10));
        game.units[armed as usize].ordered = true;
        game.units[armed as usize].target =
            Some(Vec2::new(Fixed16_16::from_int(5), Fixed16_16::from_int(5)));
        game.apply_mission_action(&Action::AllToHunt { owner: 0 });
        assert!(!game.units[armed as usize].ordered);
        assert!(game.units[armed as usize].target.is_none());
    }

    #[test]
    fn create_team_activates_a_matching_team_type() {
        let mut game = Game::new("ct", 40, 40);
        let ini = cnc_core::IniFile::parse(
            "[TaskForces]\n1=TF_X\n[TF_X]\n1=MTNK,1\n             [TeamTypes]\n1=STRIKE\n[STRIKE]\nName=Strike\nOwner=Nod\n             TaskForce=TF_X\nMission=attack\nActive=no\n",
        )
        .expect("ini");
        let types = cnc_core::entity::team_types_from_ini(&ini).expect("types");
        game.team_manager = crate::ai::TeamManager::new(types);
        assert!(game.team_manager.teams.is_empty());

        game.apply_mission_action(&Action::CreateTeam {
            owner: 1,
            team: "Strike".to_string(),
        });
        assert_eq!(game.team_manager.teams.len(), 1);
        assert!(game.team_manager.teams[0].active);
    }

    #[test]
    fn ion_cannon_requires_building_and_charge() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        let target = Vec2i::new(8, 8);
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(8, 8), 1));

        assert!(!game.fire_ion_cannon(0, target), "no ion cannon building");

        game.buildings.push(Building::from_type(
            StructureType::IonCannon,
            Game::cell_center(4, 4),
            0,
        ));
        assert!(!game.fire_ion_cannon(0, target), "not charged");

        game.houses[0].ion_charge = Fixed16_16::ONE;
        let health = game.units[0].health;
        assert!(game.fire_ion_cannon(0, target));
        assert!(
            game.units[0].health < health,
            "ion blast should damage units at the target"
        );
        assert_eq!(game.houses[0].ion_charge, Fixed16_16::ZERO);
        assert!(!game.blasts.is_empty());
        assert_eq!(game.beams.len(), 1, "Ion Cannon should draw a beam");

        for _ in 0..20 {
            game.update(Fixed16_16::from_f32(0.05));
        }
        assert!(game.beams.is_empty(), "beam fades out");
    }

    #[test]
    fn nuke_hits_wide_area_and_buildings() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        game.houses
            .push(House::new(ClassId::HouseNod, "Nod".to_string()));
        game.buildings.push(Building::from_type(
            StructureType::TempleOfNod,
            Game::cell_center(2, 2),
            1,
        ));
        game.houses[1].nuke_charge = Fixed16_16::ONE;

        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(8, 8), 0));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(10, 8), 0));
        game.buildings.push(Building::from_type(
            StructureType::Barracks,
            Game::cell_center(9, 8),
            0,
        ));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(14, 8), 0));
        for unit in &mut game.units {
            // Hold position instead of drifting on the idle patrol ring.
            unit.ordered = true;
        }

        let unit_health = game.units[0].health;
        let far_health = game.units[1].health;
        let outside_health = game.units[2].health;
        let building_health = game.buildings[1].health;

        assert!(game.fire_nuke(1, Vec2i::new(8, 8)));
        assert_eq!(
            game.units[0].health, unit_health,
            "per-cell damage is deferred until the blast animation advances"
        );
        assert_eq!(game.houses[1].nuke_charge, Fixed16_16::ZERO);
        assert_eq!(game.strikes.len(), 1, "missile launch should be tracked");

        // During the missile flight the target is still undamaged.
        for _ in 0..20 {
            game.update(Fixed16_16::from_f32(0.05));
        }
        assert_eq!(game.units[0].health, unit_health, "missile still in flight");

        // After the flight and the staggered per-cell damage window.
        for _ in 0..80 {
            game.update(Fixed16_16::from_f32(0.05));
        }
        assert!(game.strikes.is_empty());
        assert!(game.units[0].health < unit_health);
        assert!(game.units[1].health < far_health, "blast covers 4 cells");
        assert_eq!(
            game.units[2].health, outside_health,
            "6 cells away is outside the atom blast"
        );
        assert!(
            game.buildings[1].health < building_health,
            "buildings take nuke damage"
        );
    }

    #[test]
    fn superweapons_charge_over_time() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        game.buildings.push(Building::from_type(
            StructureType::IonCannon,
            Game::cell_center(4, 4),
            0,
        ));

        for _ in 0..599 {
            game.update(Fixed16_16::from_int(1));
        }
        assert!(game.houses[0].ion_charge.to_f32() > 0.99);
    }

    #[test]
    fn group_moves_reserve_distinct_cells_until_arrival() {
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(2, 8), 0));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(4, 8), 0));
        game.select_all_owned(0);

        assert_eq!(game.order_move(Game::cell_center(10, 8)), 2);
        let first = game.units[0].reserved_cell.expect("first reservation");
        let second = game.units[1].reserved_cell.expect("second reservation");
        assert_ne!(first, second, "each unit reserves a different next cell");

        for _ in 0..3000 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.units[0].target.is_none() && game.units[1].target.is_none() {
                break;
            }
        }
        for _ in 0..20 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        assert!(game.units[0].reserved_cell.is_none());
        assert!(game.units[1].reserved_cell.is_none());
    }

    #[test]
    fn homing_projectile_retargets_on_even_frames() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_center(10, 10),
            1,
        ));

        let initial_target = Game::cell_center(10, 10);
        let projectile = Projectile::new(
            Weapon::from_type(WeaponType::Dragon),
            Game::cell_center(1, 1),
            initial_target,
            0,
            Some(0),
        );
        assert!(projectile.homing);
        game.projectiles.push(projectile);

        // Odd frame: missile keeps its old aim.
        game.update(Fixed16_16::from_f32(0.05));
        assert_eq!(game.projectiles[0].target, initial_target);

        // Move the target, then advance until the missile checks on an even age.
        game.units[0].position = Game::cell_center(12, 12);
        game.update(Fixed16_16::from_f32(0.05));
        game.update(Fixed16_16::from_f32(0.05));
        assert_eq!(game.projectiles[0].target, Game::cell_center(12, 12));
    }

    #[test]
    fn aircraft_fire_consumes_ammo() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::Orca, Game::cell_center(8, 8), 0));
        game.units
            .push(Unit::new(UnitType::InfantryE1, Game::cell_center(8, 10), 1));
        game.units[0].is_selected = true;
        assert_eq!(game.order_attack(1), 1);

        let start = game.units[0].ammo;
        for _ in 0..200 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.units[0].ammo < start {
                break;
            }
        }

        assert!(game.projectiles_fired > 0, "aircraft should fire");
        assert!(game.units[0].ammo < start, "firing should consume ammo");
    }

    #[test]
    fn aircraft_flies_over_impassable_terrain() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::Orca, Game::cell_center(1, 8), 0));
        game.units[0].is_selected = true;
        for y in 0..16 {
            game.map.grid.set_passable(Vec2i::new(8, y), false);
        }

        assert_eq!(game.order_move(Game::cell_center(15, 8)), 1);
        assert!(
            game.units[0].path.is_empty(),
            "aircraft ignore ground paths"
        );

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.units[0].target.is_none() {
                break;
            }
        }

        let x = game.units[0].position.x.to_f32();
        assert!((x - 15.0 * TILE_SIZE - TILE_SIZE * 0.5).abs() < 5.0);
    }

    #[test]
    fn aircraft_rotor_and_vehicle_tracks_animate_while_moving() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.units
            .push(Unit::new(UnitType::Orca, Game::cell_center(2, 2), 0));
        game.units
            .push(Unit::new(UnitType::MediumTank, Game::cell_center(2, 6), 0));
        game.select_all_owned(0);
        assert_eq!(game.order_move(Game::cell_center(8, 2)), 2);

        for _ in 0..10 {
            game.update(Fixed16_16::from_f32(0.05));
        }

        assert!(game.units[0].rotor_phase > 0.0, "rotor should spin");
        assert!(game.units[1].track_phase > 0.0, "tracks should animate");
    }

    #[test]
    fn aircraft_rearms_at_helipad() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.buildings.push(Building::from_type(
            StructureType::Helipad,
            Game::cell_center(4, 4),
            0,
        ));
        let mut orca = Unit::new(UnitType::Orca, Game::cell_center(6, 4), 0);
        orca.ammo = 0;
        game.units.push(orca);

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.units[0].ammo > 0 {
                break;
            }
        }

        assert!(game.units[0].ammo > 0, "helipad should rearm the aircraft");
    }

    #[test]
    fn status_reports_mission_and_credits() {
        let mut game = Game::new("HUD test", 16, 16);
        game.setup_demo();

        let status = game.status();
        assert_eq!(status.mission, "HUD test");
        assert!(status.units >= 1);
        assert!(status.buildings >= 1);
        assert!(status.gdi_credits > 0);
        assert!(status.nod_credits > 0);
    }

    /// A single commando, an enemy construction yard and nothing else.
    fn sabotage_scene() -> Game {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        game.houses
            .push(House::new(ClassId::HouseNod, "Nod".to_string()));
        game.buildings.push(Building::from_type(
            StructureType::ConstructionYard,
            Game::cell_center(6, 6),
            1,
        ));
        let buildings = game.buildings.len() as u32;
        game.houses[1].add_building(buildings - 1);

        game.units
            .push(Unit::new(UnitType::Commando, Game::cell_center(9, 9), 0));
        let units = game.units.len() as u32;
        game.houses[0].add_unit(units - 1);
        game
    }

    #[test]
    fn commando_sabotage_arms_the_building_then_blows_it_up() {
        let mut game = sabotage_scene();
        let commando = 0;
        let yard = 0;
        assert!(game.units[commando].is_commando);

        assert!(game.order_sabotage(commando, yard));
        assert_eq!(game.units[commando].special_target, Some(yard));
        assert_eq!(game.units[commando].attack_target, None);

        // Walk into the building: INFANTRY.CPP:717-735 Per_Cell_Process plants
        // the charge when the infantry enters the building cell.
        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if game.buildings[yard].is_going_to_blow {
                break;
            }
        }
        assert!(
            game.buildings[yard].is_going_to_blow,
            "entering the building plants C4"
        );
        assert_eq!(
            game.buildings[yard].whom_to_repay,
            Some(commando as u32),
            "BUILDING.H WhomToRepay names the saboteur"
        );
        assert!(
            game.buildings[yard].count_down > Fixed16_16::ZERO,
            "INFANTRY.CPP sets a 20 second CountDown"
        );
        assert_eq!(
            game.units[commando].special_target, None,
            "the commando scatters away from the charge"
        );

        // BUILDING.CPP:1229-1246: once the countdown expires the building takes
        // 5000 points of WARHEAD_FIRE and dies.
        let before = game.buildings[yard].health;
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.buildings[yard].is_going_to_blow {
                break;
            }
        }
        assert!(!game.buildings[yard].is_going_to_blow);
        assert_eq!(game.buildings[yard].health, Fixed16_16::ZERO);
        assert!(before > Fixed16_16::ZERO);
    }

    #[test]
    fn sabotage_orders_are_restricted_to_commandos_and_enemy_buildings() {
        let mut game = sabotage_scene();
        // A rifleman cannot sabotage (INFANTRY.CPP:2891-2901 only rewrites the
        // rambo's action).
        game.units[0] = Unit::new(UnitType::InfantryE1, Game::cell_center(9, 9), 0);
        assert!(!game.order_sabotage(0, 0));
        assert_eq!(game.units[0].special_target, None);

        // Even a commando cannot touch a friendly building.
        game.units[0] = Unit::new(UnitType::Commando, Game::cell_center(9, 9), 0);
        game.buildings[0].owner = 0;
        assert!(!game.order_sabotage(0, 0));

        // Out-of-range indices are rejected.
        assert!(!game.order_sabotage(9, 0));
        assert!(!game.order_sabotage(0, 9));
    }

    #[test]
    fn engineer_captures_an_enemy_building_and_is_consumed() {
        let mut game = sabotage_scene();
        // Swap the commando for an engineer (IDATA.CPP IsCapture = true).
        game.units[0] = Unit::new(UnitType::Engineer, Game::cell_center(9, 9), 0);
        let engineer = 0;
        let yard = 0;
        assert!(game.units[engineer].is_engineer);

        assert!(game.order_capture(engineer, yard));
        assert_eq!(game.units[engineer].special_target, Some(yard));

        for _ in 0..400 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.units[engineer].is_alive() {
                break;
            }
        }
        assert_eq!(
            game.buildings[yard].owner, 0,
            "INFANTRY.CPP:695 tech->Captured(House)"
        );
        assert!(game.buildings[yard].is_captured);
        assert!(
            !game.units[engineer].is_alive(),
            "INFANTRY.CPP:696 Delete_This consumes the engineer"
        );
        assert_eq!(game.units[engineer].special_target, None);
    }

    #[test]
    fn capture_respects_the_bdata_captureable_flag() {
        let mut game = sabotage_scene();
        game.units[0] = Unit::new(UnitType::Engineer, Game::cell_center(9, 9), 0);
        // The Eye (radar) is not captureable in BDATA.CPP.
        game.buildings[0] =
            Building::from_type(StructureType::IonCannon, Game::cell_center(6, 6), 1);
        assert!(!game.buildings[0].is_captureable());
        assert!(!game.order_capture(0, 0));
    }

    #[test]
    fn sabotage_command_applies_through_the_network_handler() {
        // network.rs already serialises Command::Sabotage { unit, target }; this
        // pins the game-side apply arm and its lockstep determinism.
        let mut game = sabotage_scene();
        let packet = crate::network::NetPacket::new(
            0,
            0,
            vec![crate::network::NetCommand::new(
                0,
                0,
                crate::network::Command::Sabotage { unit: 0, target: 0 },
            )],
        );
        let wire = packet.encode().unwrap();
        let decoded = crate::network::NetPacket::decode(&wire).unwrap();
        assert_eq!(decoded, packet);

        assert!(game.apply_command(0, &decoded.commands[0].command));
        assert_eq!(game.units[0].special_target, Some(0));

        // The same command applied to a peer reproduces identical state, and a
        // mismatched owner is not allowed to command someone else's commando.
        let mut peer = sabotage_scene();
        assert!(peer.apply_command(0, &decoded.commands[0].command));
        assert_eq!(
            peer.units[0].special_target, game.units[0].special_target,
            "identical command stream, identical state"
        );
        let mut outsider = sabotage_scene();
        assert!(!outsider.apply_command(1, &decoded.commands[0].command));
    }

    #[test]
    fn sabotage_ticks_deterministically_across_identical_games() {
        let mut first = sabotage_scene();
        let mut second = sabotage_scene();
        assert!(first.order_sabotage(0, 0));
        assert!(second.order_sabotage(0, 0));

        for step in 0..800 {
            let delta = Fixed16_16::from_f32(0.05);
            first.update(delta);
            second.update(delta);
            assert_eq!(
                first.buildings[0].is_going_to_blow, second.buildings[0].is_going_to_blow,
                "sabotage must not drift at step {step}"
            );
            assert_eq!(
                first.buildings[0].count_down.to_bits(),
                second.buildings[0].count_down.to_bits(),
                "the countdown must tick identically at step {step}"
            );
        }
        assert!(!first.buildings[0].is_alive());
        assert_eq!(first.buildings[0].health, second.buildings[0].health);
    }
}

#[cfg(test)]
mod rules_integration_tests {
    use super::*;
    use cnc_core::rules::RulesDatabase;

    fn rules(source: &str) -> RulesDatabase {
        RulesDatabase::from_str(source).expect("rules should parse")
    }

    #[test]
    fn ini_names_match_original_data_files() {
        assert_eq!(UnitType::InfantryE1.ini_name(), "E1");
        assert_eq!(UnitType::MediumTank.ini_name(), "MTNK");
        assert_eq!(UnitType::MammothTank.ini_name(), "HTNK");
        assert_eq!(UnitType::Harvester.ini_name(), "HARV");
        assert_eq!(UnitType::Orca.ini_name(), "ORCA");
        assert_eq!(UnitType::Apache.ini_name(), "HELI");

        assert_eq!(StructureType::ConstructionYard.ini_name(), "FACT");
        assert_eq!(StructureType::PowerPlant.ini_name(), "NUKE");
        assert_eq!(StructureType::Refinery.ini_name(), "PROC");
        assert_eq!(StructureType::Barracks.ini_name(), "PYLE");
        assert_eq!(StructureType::WarFactory.ini_name(), "WEAP");
        assert_eq!(StructureType::Helipad.ini_name(), "HPAD");
        assert_eq!(StructureType::IonCannon.ini_name(), "EYE");
        assert_eq!(StructureType::TempleOfNod.ini_name(), "TMPL");
    }

    #[test]
    fn empty_rules_keep_original_costs() {
        let game = Game::new("test", 32, 32);
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 100);
        assert_eq!(game.unit_cost(UnitType::MammothTank), 1500);
        assert_eq!(game.building_cost(StructureType::PowerPlant), 300);
        assert_eq!(game.unit_strength(UnitType::InfantryE1), 50);
        assert_eq!(game.building_strength(StructureType::PowerPlant), 200);
    }

    #[test]
    fn rules_override_unit_cost_and_charge() {
        let rules = rules("[E1]\nCost=40\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();
        game.units.clear();

        assert_eq!(game.unit_cost(UnitType::InfantryE1), 40);
        let credits = game.status().gdi_credits;
        assert!(game.queue_unit(0, UnitType::InfantryE1));
        assert_eq!(game.status().gdi_credits, credits - 40);
    }

    #[test]
    fn rules_override_building_cost_and_charge() {
        let rules = rules("[NUKE]\nCost=100\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();

        assert_eq!(game.building_cost(StructureType::PowerPlant), 100);
        let credits = game.status().gdi_credits;
        assert!(game.queue_building(0, StructureType::PowerPlant));
        assert_eq!(game.status().gdi_credits, credits - 100);
    }

    #[test]
    fn rules_override_strength_when_unit_spawns() {
        let rules = rules("[E1]\nCost=100\nStrength=200\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();
        game.units.clear();

        assert!(game.queue_unit(0, UnitType::InfantryE1));
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.units.is_empty() {
                break;
            }
        }
        assert_eq!(game.units.len(), 1);
        assert_eq!(game.units[0].max_health, Fixed16_16::from_int(200));
        assert_eq!(game.units[0].health, Fixed16_16::from_int(200));
    }

    #[test]
    fn rules_override_strength_when_building_is_placed() {
        let rules = rules("[NUKE]\nStrength=350\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();
        game.ready_buildings.push((0, StructureType::PowerPlant));

        let cell = Vec2i::new(1, 1);
        assert!(game.can_place_building(cell));
        assert!(game.place_ready_building(0, cell));
        let building = game
            .buildings
            .iter()
            .rev()
            .find(|building| building.structure_type() == Some(StructureType::PowerPlant))
            .expect("power plant should exist");
        assert_eq!(building.max_health, Fixed16_16::from_int(350));
        assert_eq!(building.health, Fixed16_16::from_int(350));
    }

    #[test]
    fn unknown_type_falls_back_to_static_stats() {
        let rules = rules("[E1]\nCost=40\n");
        let game = Game::with_rules("test", 32, 32, rules);
        // MTNK is absent from the rules file, so the UDATA.CPP value applies.
        assert_eq!(game.unit_cost(UnitType::MediumTank), 800);
    }

    #[test]
    fn load_rules_from_disk_reports_type_count() {
        let dir = std::env::temp_dir().join(format!(
            "cnc_td_rules_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("rules.ini");
        std::fs::write(&path, "[E1]\nCost=75\n[FACT]\nCost=4000\n").unwrap();

        let mut game = Game::new("test", 32, 32);
        let count = game.load_rules(&path).unwrap();
        assert_eq!(count, 2);
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 75);
        assert_eq!(game.building_cost(StructureType::ConstructionYard), 4000);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn set_rules_replaces_database() {
        let mut game = Game::new("test", 32, 32);
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 100);
        game.set_rules(rules("[E1]\nCost=25\n"));
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 25);
        game.set_rules(RulesDatabase::default());
        assert_eq!(game.unit_cost(UnitType::InfantryE1), 100);
    }
}

#[cfg(test)]
mod weapon_rules_tests {
    use super::*;
    use crate::td_rules::{ticks_to_seconds, ArmorType};
    use cnc_core::rules::RulesDatabase;

    fn rules(source: &str) -> RulesDatabase {
        RulesDatabase::from_str(source).expect("rules should parse")
    }

    #[test]
    fn weapon_ini_names_match_rules_tables() {
        assert_eq!(WeaponType::M16.ini_name(), "M16");
        assert_eq!(WeaponType::Cannon105.ini_name(), "105mm");
        assert_eq!(WeaponType::MammothTusk.ini_name(), "MammothTusk");
        assert_eq!(WeaponType::ObeliskLaser.ini_name(), "ObeliskLaser");
        assert_eq!(WarheadType::Sa.ini_name(), "SA");
        assert_eq!(WarheadType::HollowPoint.ini_name(), "HollowPoint");
        assert_eq!(WarheadType::from_ini_name("ap"), Some(WarheadType::Ap));
        assert_eq!(WarheadType::from_ini_name("NOPE"), None);
    }

    #[test]
    fn weapon_damage_rof_and_range_come_from_rules() {
        let rules = rules("[M16]\nDamage=25\nROF=10\nRange=1280\n");
        let game = Game::with_rules("test", 32, 32, rules);
        let weapon = game.make_weapon(WeaponType::M16);

        assert_eq!(weapon.damage, Fixed16_16::from_int(25));
        assert_eq!(weapon.range, Fixed16_16::from_f32(leptons_to_px(1280)));
        assert_eq!(
            weapon.rate_of_fire,
            Fixed16_16::from_f32(ticks_to_seconds(13))
        );
        // The static M16 row stays untouched for games without a rules file.
        let plain = Weapon::from_type(WeaponType::M16);
        assert_eq!(plain.damage, Fixed16_16::from_int(15));
        assert_ne!(plain.range, weapon.range);
    }

    #[test]
    fn weapon_warhead_override_selects_rules_warhead() {
        let rules = rules("[Grenade]\nWarhead=AP\n[AP]\nSpread=0\nVerses=10,20,30,40,50\n");
        let game = Game::with_rules("test", 32, 32, rules);
        let weapon = game.make_weapon(WeaponType::Grenade);

        assert_eq!(weapon.warhead.warhead_type, WarheadType::Ap);
        let base = Fixed16_16::from_int(100);
        assert_eq!(
            weapon.warhead.damage(base, ArmorType::Steel, 0),
            Fixed16_16::from_int(40)
        );
        assert_eq!(
            weapon.warhead.damage(base, ArmorType::None, 0),
            Fixed16_16::from_int(10)
        );
    }

    #[test]
    fn warhead_verses_override_without_weapon_section() {
        let rules = rules("[AP]\nSpread=0\nVerses=5,10,20,40,80\n");
        let game = Game::with_rules("test", 32, 32, rules);
        // M16's static warhead is SA, but an explicit warhead only applies to AP.
        let m16 = game.make_weapon(WeaponType::M16);
        assert!(m16.warhead.verses.is_none());

        let profile = game.warhead_profile(WarheadType::Ap);
        assert_eq!(
            profile.damage(Fixed16_16::from_int(100), ArmorType::Concrete, 0),
            Fixed16_16::from_int(80)
        );
    }

    #[test]
    fn rules_spread_attenuates_damage_with_distance() {
        let rules = rules("[AP]\nSpread=2\nVerses=100,100,100,100,100\n");
        let game = Game::with_rules("test", 32, 32, rules);
        let profile = game.warhead_profile(WarheadType::Ap);
        let near = profile.damage(Fixed16_16::from_int(100), ArmorType::Steel, 0);
        let far = profile.damage(Fixed16_16::from_int(100), ArmorType::Steel, 8);
        assert_eq!(near, Fixed16_16::from_int(100));
        assert_eq!(far, Fixed16_16::from_int(25));
    }

    #[test]
    fn set_rules_refreshes_existing_unit_weapons() {
        let mut game = Game::new("test", 32, 32);
        game.setup_demo();
        game.set_rules(rules("[M16]\nDamage=99\nRange=2048\n"));

        let infantry = game
            .units
            .iter()
            .find(|unit| unit.unit_type == UnitType::InfantryE1)
            .expect("demo GDI infantry");
        let weapon = infantry.weapon.as_ref().expect("rifle");
        assert_eq!(weapon.damage, Fixed16_16::from_int(99));
        assert_eq!(weapon.range, Fixed16_16::from_f32(leptons_to_px(2048)));
    }

    #[test]
    fn production_spawns_units_with_rules_weapons() {
        let rules = rules("[E1]\nCost=100\n[M16]\nDamage=42\n");
        let mut game = Game::with_rules("test", 32, 32, rules);
        game.setup_demo();
        game.units.clear();
        assert!(game.queue_unit(0, UnitType::InfantryE1));
        for _ in 0..600 {
            game.update(Fixed16_16::from_f32(0.05));
            if !game.units.is_empty() {
                break;
            }
        }
        let unit = game.units.first().expect("produced infantry");
        assert_eq!(
            unit.weapon.as_ref().unwrap().damage,
            Fixed16_16::from_int(42)
        );
    }
}

#[cfg(test)]
mod prerequisite_tests {
    use super::*;
    use crate::ai::AIDifficulty;
    use cnc_core::rules::RulesDatabase;

    fn rules(source: &str) -> RulesDatabase {
        RulesDatabase::from_str(source).expect("rules should parse")
    }

    #[test]
    fn types_without_rules_have_no_prerequisites() {
        let game = Game::new("test", 32, 32);
        assert!(game.meets_prerequisites(0, "E1"));
        assert!(game.meets_prerequisites(0, "MISSING"));
        assert!(game.missing_prerequisites(0, "E1").is_empty());
    }

    #[test]
    fn unit_prerequisite_requires_owned_structure() {
        let database = rules("[E1]\nCost=100\nPrerequisite=WEAP\n");
        let mut game = Game::with_rules("test", 32, 32, database);
        game.setup_demo();
        assert!(game.meets_prerequisites(0, "E1"));

        game.set_rules(rules("[E1]\nCost=100\nPrerequisite=EYE\n"));
        assert!(!game.meets_prerequisites(0, "E1"));
        assert_eq!(game.missing_prerequisites(0, "E1"), vec!["EYE"]);

        game.units.clear();
        let credits = game.status().gdi_credits;
        assert!(!game.queue_unit(0, UnitType::InfantryE1));
        assert_eq!(game.status().gdi_credits, credits);
        assert_eq!(game.status().production, 0);
    }

    #[test]
    fn building_prerequisite_is_enforced() {
        let database = rules("[NUKE]\nCost=300\nPrerequisite=TMPL\n");
        let mut game = Game::with_rules("test", 32, 32, database);
        game.setup_demo();
        assert!(!game.can_queue_building(0, StructureType::PowerPlant));
        assert!(!game.queue_building(0, StructureType::PowerPlant));

        game.set_rules(rules("[NUKE]\nCost=300\nPrerequisite=FACT\n"));
        assert!(game.can_queue_building(0, StructureType::PowerPlant));
        assert!(game.queue_building(0, StructureType::PowerPlant));
    }

    #[test]
    fn none_and_faction_tokens_are_supported() {
        let game = Game::with_rules("test", 32, 32, rules("[E1]\nCost=100\nPrerequisite=NONE\n"));
        assert!(game.meets_prerequisites(0, "E1"));

        let mut game =
            Game::with_rules("test", 32, 32, rules("[E1]\nCost=100\nPrerequisite=GDI\n"));
        game.setup_demo();
        assert!(game.meets_prerequisites(0, "E1"));
        assert!(!game.meets_prerequisites(1, "E1"));
        assert_eq!(game.missing_prerequisites(1, "E1"), vec!["GDI"]);

        let mut game =
            Game::with_rules("test", 32, 32, rules("[E1]\nCost=100\nPrerequisite=NOD\n"));
        game.setup_demo();
        assert!(game.meets_prerequisites(1, "E1"));
        assert!(!game.meets_prerequisites(0, "E1"));
    }

    #[test]
    fn gdi_and_nod_barracks_aliases_are_equivalent() {
        let mut game =
            Game::with_rules("test", 32, 32, rules("[E1]\nCost=100\nPrerequisite=PYLE\n"));
        game.setup_demo();
        assert!(game.meets_prerequisites(0, "E1"));

        game.set_rules(rules("[E1]\nCost=100\nPrerequisite=HAND\n"));
        // The demo GDI barracks satisfies the Nod spelling through the alias.
        assert!(game.meets_prerequisites(0, "E1"));
    }

    #[test]
    fn prerequisite_lists_require_every_token() {
        let mut game = Game::with_rules(
            "test",
            32,
            32,
            rules("[E1]\nCost=100\nPrerequisite=WEAP,FACT\n"),
        );
        game.setup_demo();
        assert!(game.meets_prerequisites(0, "E1"));

        game.set_rules(rules("[E1]\nCost=100\nPrerequisite=WEAP,EYE\n"));
        assert!(!game.meets_prerequisites(0, "E1"));
        assert_eq!(game.missing_prerequisites(0, "E1"), vec!["EYE"]);
    }

    #[test]
    fn unknown_prerequisite_fails_closed() {
        let mut game = Game::with_rules(
            "test",
            32,
            32,
            rules("[E1]\nCost=100\nPrerequisite=NOT_A_BUILDING\n"),
        );
        game.setup_demo();
        assert!(!game.meets_prerequisites(0, "E1"));
        assert_eq!(game.missing_prerequisites(0, "E1"), vec!["NOT_A_BUILDING"]);
    }

    #[test]
    fn prerequisite_only_sections_are_registered_as_types() {
        let rules = rules("[E1]\nPrerequisite=WEAP\n");
        assert_eq!(rules.type_rule("E1").unwrap().prerequisite, vec!["WEAP"]);
    }

    #[test]
    fn tech_level_gates_unit_and_structure_production() {
        let mut game = Game::new("tech", 32, 32);
        game.setup_demo();

        assert_eq!(game.unit_tech_level(UnitType::MammothTank), 5);
        assert_eq!(game.unit_tech_level(UnitType::InfantryE1), 1);
        assert_eq!(game.building_tech_level(StructureType::Helipad), 6);
        assert_eq!(game.building_tech_level(StructureType::PowerPlant), 0);

        game.houses[0].tech_level = 1;
        assert!(game.can_queue_unit(0, UnitType::InfantryE1));
        assert!(!game.can_queue_unit(0, UnitType::MammothTank));
        assert!(!game.can_queue_building(0, StructureType::Helipad));

        game.houses[0].tech_level = 6;
        assert!(game.can_queue_unit(0, UnitType::MammothTank));
        assert!(game.can_queue_building(0, StructureType::Helipad));
    }

    #[test]
    fn rules_tech_level_override_replaces_the_fallback() {
        let mut game = Game::with_rules(
            "tech-override",
            32,
            32,
            rules("[MTNK]\nTechLevel=7\n[HPAD]\nTechLevel=9\n"),
        );
        game.setup_demo();
        assert_eq!(game.unit_tech_level(UnitType::MediumTank), 7);
        assert_eq!(game.building_tech_level(StructureType::Helipad), 9);

        game.houses[0].tech_level = 6;
        assert!(!game.can_queue_unit(0, UnitType::MediumTank));
        game.houses[0].tech_level = 7;
        assert!(game.can_queue_unit(0, UnitType::MediumTank));
    }

    #[test]
    fn sidebar_hides_tech_and_prerequisite_locked_rows() {
        let mut game = Game::with_rules("sidebar", 32, 32, rules("[MTNK]\nPrerequisite=TMPL\n"));
        game.setup_demo();

        let visible = game.visible_sidebar_options(0);
        assert!(visible
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::InfantryE1)));
        assert!(!visible
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MediumTank)));

        let all = game.sidebar_options(0);
        let mtnk = all
            .iter()
            .find(|option| option.buildable == Buildable::Unit(UnitType::MediumTank))
            .expect("medium tank row exists");
        assert!(!mtnk.visible);
        assert_eq!(mtnk.missing_prerequisites, vec!["TMPL"]);

        // TechLevel 1 still shows cheap units but hides the high-tier rows.
        game.houses[0].tech_level = 1;
        let visible = game.visible_sidebar_options(0);
        assert!(visible
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::InfantryE1)));
        assert!(!visible
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MammothTank)));
        assert!(!visible
            .iter()
            .any(|option| { option.buildable == Buildable::Structure(StructureType::Helipad) }));
    }

    #[test]
    fn sidebar_keeps_rows_visible_when_unaffordable() {
        let mut game = Game::new("sidebar-cost", 32, 32);
        game.setup_demo();
        game.houses[0].credits = 0;
        game.houses[0].tiberium = 0;

        let e1 = game
            .sidebar_options(0)
            .into_iter()
            .find(|option| option.buildable == Buildable::Unit(UnitType::InfantryE1))
            .expect("infantry row exists");
        assert!(e1.visible);
        assert!(!e1.affordable);
        assert!(e1.has_producer);
    }

    #[test]
    fn sidebar_filters_owner_locked_rows() {
        let mut game = Game::with_rules(
            "sidebar-owner",
            32,
            32,
            rules(
                "[MTNK]\nOwner=GDI\n[HTNK]\nOwner=Nod\n[BGGY]\nOwner=Nod\n\
                 [TMPL]\nOwner=Nod\nTechLevel=1\n[EYE]\nOwner=GDI\nTechLevel=1\n",
            ),
        );
        game.setup_demo();

        let gdi = game.visible_sidebar_options(0);
        assert!(gdi
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MediumTank)));
        assert!(!gdi
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MammothTank)));
        assert!(!gdi
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::Buggy)));
        assert!(!gdi
            .iter()
            .any(|option| option.buildable == Buildable::Structure(StructureType::TempleOfNod)));
        assert!(gdi
            .iter()
            .any(|option| option.buildable == Buildable::Structure(StructureType::IonCannon)));
        // The full table still lists every row, just with visible = false.
        assert_eq!(
            game.sidebar_options(0).len(),
            UnitType::ALL.len() + StructureType::ALL.len()
        );

        let nod = game.visible_sidebar_options(1);
        assert!(!nod
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::MediumTank)));
        assert!(nod
            .iter()
            .any(|option| option.buildable == Buildable::Unit(UnitType::Buggy)));
        assert!(nod
            .iter()
            .any(|option| option.buildable == Buildable::Structure(StructureType::TempleOfNod)));
        assert!(!nod
            .iter()
            .any(|option| option.buildable == Buildable::Structure(StructureType::IonCannon)));
    }

    #[test]
    fn faction_ownership_uses_owner_tokens_and_td_fallback() {
        let mut game = Game::with_rules("owner-tokens", 32, 32, rules("[MTNK]\nOwner=Nod,ANY\n"));
        game.setup_demo();

        // Owner=ANY admits both sides, Owner=Nod locks the medium tank to Nod.
        assert!(game.unit_owned_by_faction(0, UnitType::MediumTank));
        assert!(game.unit_owned_by_faction(1, UnitType::MediumTank));

        // No rules entry: TD defaults (Mammoth is GDI-only, Buggy Nod-only).
        assert!(game.unit_owned_by_faction(0, UnitType::MammothTank));
        assert!(!game.unit_owned_by_faction(1, UnitType::MammothTank));
        assert!(!game.unit_owned_by_faction(0, UnitType::Buggy));
        assert!(game.unit_owned_by_faction(1, UnitType::Buggy));
        assert!(game.structure_owned_by_faction(0, StructureType::IonCannon));
        assert!(!game.structure_owned_by_faction(1, StructureType::IonCannon));
        assert!(game.structure_owned_by_faction(1, StructureType::TempleOfNod));
    }

    #[test]
    fn ai_production_uses_rules_cost_and_tech_level() {
        let ai = AI::new(0, AIDifficulty::Normal);
        let mut game = Game::with_rules(
            "ai-production",
            32,
            32,
            rules("[MTNK]\nCost=800\nTechLevel=3\n[HTNK]\nCost=10\nTechLevel=7\n"),
        );
        game.setup_demo();

        game.houses[0].tech_level = 6;
        assert_ne!(
            game.ai_production_choice(0, &ai, 3000),
            Some(UnitType::MammothTank),
            "rules TechLevel 7 must stay locked at build level 6"
        );

        // At full tech the 10-credit Mammoth dominates the weighted table.
        game.houses[0].tech_level = 7;
        assert_eq!(
            game.ai_production_choice(0, &ai, 3000),
            Some(UnitType::MammothTank)
        );
        assert!(game.ai_queue_production(0, &ai, 0).is_some());
    }

    #[test]
    fn cancelling_td_production_refunds_the_front_item() {
        let mut game = Game::new("cancel", 64, 64);
        game.setup_demo();
        let before = game.houses[0].credits;
        assert!(game.queue_unit(0, UnitType::InfantryE1));
        assert!(game.houses[0].credits < before);
        let refund = game.cancel_production(0).expect("front item");
        assert!(refund > 0);
        assert_eq!(game.houses[0].credits, before);
        assert!(game.houses[0].production_queue.is_empty());
        assert_eq!(game.cancel_production(0), None);
    }

    #[test]
    fn repairing_a_building_costs_and_heals() {
        let mut game = Game::new("repair", 64, 64);
        game.setup_demo();
        let index = game
            .buildings
            .iter()
            .position(|building| building.owner == 0 && building.is_alive())
            .unwrap();
        game.buildings[index].health = game.buildings[index].max_health / Fixed16_16::from_int(2);
        let credits = game.houses[0].credits;
        let cost = game
            .repair_building(0, index)
            .expect("repair should succeed");
        assert!(cost > 0);
        assert_eq!(game.houses[0].credits, credits - cost);
        assert_eq!(
            game.buildings[index].health,
            game.buildings[index].max_health
        );
        assert_eq!(game.repair_building(0, index), None, "already full");
        game.buildings[index].health = game.buildings[index].max_health / Fixed16_16::from_int(4);
        assert_eq!(game.repair_building(1, index), None, "wrong owner");
    }

    #[test]
    fn selling_a_building_refunds_half() {
        let mut game = Game::new("sell", 64, 64);
        game.setup_demo();
        let index = game
            .buildings
            .iter()
            .position(|building| building.owner == 0 && building.is_alive())
            .unwrap();
        let structure = game.buildings[index].structure_type().unwrap();
        let cost = game.building_cost(structure);
        let credits = game.houses[0].credits;
        let refund = game.sell_building(0, index).unwrap();
        assert_eq!(refund, cost / 2);
        assert_eq!(game.houses[0].credits, credits + refund);
        assert!(!game.buildings[index].is_alive());
        assert!(game.buildings[index].is_selling);
        assert_eq!(game.sell_building(0, index), None, "already sold");
        let enemy = game
            .buildings
            .iter()
            .position(|building| building.owner == 1 && building.is_alive())
            .unwrap();
        assert_eq!(game.sell_building(0, enemy), None, "wrong owner");
    }

    #[test]
    fn nod_mission_controls_the_nod_house() {
        let maps = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
        let mut game = Game::new("nod-test", 64, 64);
        game.setup_demo();
        game.load_mission(&maps.join("02.ini")).unwrap();
        assert_eq!(game.player_owner, 1);

        let position = game
            .units
            .iter()
            .find(|unit| unit.owner == 1 && unit.is_alive())
            .map(|unit| unit.position)
            .expect("Nod units missing");
        assert!(
            game.select_at_additive(position, false).is_some(),
            "clicking a Nod unit selects it"
        );
        assert!(game.select_all_owned(1) > 0);

        let gdi_position = game
            .units
            .iter()
            .find(|unit| unit.owner == 0 && unit.is_alive())
            .map(|unit| unit.position)
            .expect("GDI units missing");
        assert!(game.enemy_at(gdi_position).is_some());

        let path = std::env::temp_dir().join(format!("nod-player-{}.sav", std::process::id()));
        game.save_to_file(&path).unwrap();
        let loaded = Game::load_from_file(&path).unwrap();
        assert_eq!(loaded.player_owner, 1, "the save keeps the mission side");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn owner_targeted_orders_only_touch_listed_units() {
        let mut game = Game::new("net", 32, 32);
        game.setup_demo();
        let (gdi_id, gdi_start) = game
            .units
            .iter()
            .enumerate()
            .find(|(_, unit)| unit.owner == 0 && unit.is_alive())
            .map(|(index, unit)| (index as u32, unit.position))
            .unwrap();
        let (nod_id, nod_start) = game
            .units
            .iter()
            .enumerate()
            .find(|(_, unit)| unit.owner == 1 && unit.is_alive())
            .map(|(index, unit)| (index as u32, unit.position))
            .unwrap();
        let target = Vec2::new(
            Fixed16_16::from_f32(gdi_start.x.to_f32() + 6.0 * TILE_SIZE),
            Fixed16_16::from_f32(gdi_start.y.to_f32() + 2.0 * TILE_SIZE),
        );
        assert_eq!(game.order_units_move(0, &[gdi_id], target), 1);
        assert!(game.units[gdi_id as usize].target.is_some());
        assert!(
            game.units[nod_id as usize].target.is_none(),
            "other owner untouched"
        );

        // A mismatched owner id is ignored.
        assert_eq!(game.order_units_move(1, &[gdi_id], target), 0);

        // Stop clears the order but not the unit.
        assert_eq!(game.order_units_stop(0, &[gdi_id]), 1);
        assert!(game.units[gdi_id as usize].target.is_none());
        assert!(game.units[gdi_id as usize].path.is_empty());

        // Attack only accepts an enemy of the commanding owner.
        assert_eq!(game.order_units_attack(0, &[gdi_id], nod_id as usize), 1);
        assert_eq!(
            game.units[gdi_id as usize].attack_target,
            Some(nod_id as usize)
        );
        assert_eq!(
            game.order_units_attack(1, &[nod_id], nod_id as usize),
            0,
            "cannot attack yourself"
        );

        // A GDI id cannot be commanded by the Nod player.
        assert_eq!(game.order_units_attack(1, &[gdi_id], nod_id as usize), 0);

        // Dead units in the list are skipped.
        let _ = nod_start;
        game.units[gdi_id as usize].health = Fixed16_16::ZERO;
        assert_eq!(game.order_units_stop(0, &[gdi_id]), 0);
    }

    /// Spawn `structure` for `owner` at `cell` and return its index.
    fn add_defence(game: &mut Game, owner: u8, structure: StructureType, cell: Vec2i) -> usize {
        let position = Game::cell_center(cell.x, cell.y);
        let mut building = Building::from_type(structure, position, owner);
        building.set_powered(true);
        building.reload = Fixed16_16::ZERO;
        game.buildings.push(building);
        game.buildings.len() - 1
    }

    #[test]
    fn guard_tower_shoots_an_enemy_unit_in_range() {
        let mut game = Game::new("defence", 32, 32);
        let tower = add_defence(&mut game, 0, StructureType::GuardTower, Vec2i::new(10, 10));

        // An enemy vehicle two cells away is well inside the turret's reach.
        let mut enemy = Unit::new(UnitType::Jeep, Game::cell_center(12, 10), 1);
        enemy.health = enemy.max_health;
        let before = enemy.health;
        let enemy_index = game.units.len();
        game.units.push(enemy);

        game.update_defence_combat(Fixed16_16::from_int(1));

        let after = game.units[enemy_index].health;
        assert!(
            after < before,
            "the guard tower must damage an in-range enemy"
        );
        assert!(
            game.buildings[tower].reload > Fixed16_16::ZERO,
            "firing must start the reload timer"
        );
    }

    #[test]
    fn defences_hold_fire_out_of_range() {
        let mut game = Game::new("defence", 32, 32);
        add_defence(&mut game, 0, StructureType::GuardTower, Vec2i::new(2, 2));

        let mut enemy = Unit::new(UnitType::Jeep, Game::cell_center(28, 28), 1);
        enemy.health = enemy.max_health;
        let before = enemy.health;
        let index = game.units.len();
        game.units.push(enemy);

        game.update_defence_combat(Fixed16_16::from_int(1));
        assert_eq!(
            game.units[index].health, before,
            "out of range must not hit"
        );
    }

    #[test]
    fn sam_site_only_engages_aircraft() {
        let mut game = Game::new("defence", 32, 32);
        add_defence(&mut game, 0, StructureType::SamSite, Vec2i::new(10, 10));

        // A ground vehicle right next to the SAM must be ignored.
        let mut ground = Unit::new(UnitType::Jeep, Game::cell_center(11, 10), 1);
        ground.health = ground.max_health;
        let ground_before = ground.health;
        let ground_index = game.units.len();
        game.units.push(ground);

        game.update_defence_combat(Fixed16_16::from_int(1));
        assert_eq!(
            game.units[ground_index].health, ground_before,
            "a SAM site must not shoot ground units"
        );

        // An aircraft in range is engaged.
        let mut aircraft = Unit::new(UnitType::Orca, Game::cell_center(11, 11), 1);
        aircraft.health = aircraft.max_health;
        let air_before = aircraft.health;
        let air_index = game.units.len();
        game.units.push(aircraft);

        game.update_defence_combat(Fixed16_16::from_int(1));
        assert!(
            game.units[air_index].health < air_before,
            "a SAM site must engage aircraft"
        );
    }

    #[test]
    fn unpowered_defences_do_not_fire() {
        let mut game = Game::new("defence", 32, 32);
        let tower = add_defence(&mut game, 0, StructureType::GuardTower, Vec2i::new(10, 10));
        game.buildings[tower].set_powered(false);

        let mut enemy = Unit::new(UnitType::Jeep, Game::cell_center(12, 10), 1);
        enemy.health = enemy.max_health;
        let before = enemy.health;
        let index = game.units.len();
        game.units.push(enemy);

        game.update_defence_combat(Fixed16_16::from_int(1));
        assert_eq!(
            game.units[index].health, before,
            "an unpowered defence must stay silent"
        );
    }

    #[test]
    fn economy_buildings_never_shoot() {
        let mut game = Game::new("defence", 32, 32);
        add_defence(&mut game, 0, StructureType::PowerPlant, Vec2i::new(10, 10));

        let mut enemy = Unit::new(UnitType::Jeep, Game::cell_center(11, 10), 1);
        enemy.health = enemy.max_health;
        let before = enemy.health;
        let index = game.units.len();
        game.units.push(enemy);

        game.update_defence_combat(Fixed16_16::from_int(1));
        assert_eq!(
            game.units[index].health, before,
            "non-defences have no weapon"
        );
    }

    #[test]
    fn tower_fires_without_player_units() {
        // A guard tower must fire at an enemy in range even when the player
        // has no units and issues no orders — base defence fires autonomously.
        let mut game = Game::new("defence", 32, 32);
        let tower = add_defence(&mut game, 0, StructureType::GuardTower, Vec2i::new(10, 10));

        assert!(
            game.buildings[tower].weapon.is_some(),
            "guard tower carries a weapon"
        );

        let mut enemy = Unit::new(UnitType::Jeep, Game::cell_center(12, 10), 1);
        enemy.health = enemy.max_health;
        let before = enemy.health;
        let enemy_index = game.units.len();
        game.units.push(enemy);

        // No player commands are issued — the tower still fires.
        game.update_defence_combat(Fixed16_16::from_int(1));
        assert!(
            game.units[enemy_index].health < before,
            "the tower fires at the enemy without player input"
        );
        assert!(
            game.buildings[tower].reload > Fixed16_16::ZERO,
            "firing must start the reload timer"
        );
    }

    #[test]
    fn obelisk_charge_up_cadence_pinned() {
        // Obelisk of Nod fires its laser at a fixed cadence pinned to its
        // BUILDING.CPP stats: ROF 90 ticks → ~1.55s reload at 60 ticks/sec.
        let mut game = Game::new("defence", 32, 32);
        add_defence(&mut game, 0, StructureType::Obelisk, Vec2i::new(10, 10));

        let mut enemy = Unit::new(UnitType::Jeep, Game::cell_center(12, 10), 1);
        enemy.health = enemy.max_health;
        let before = enemy.health;
        game.units.push(enemy);

        // First shot fires immediately (reload starts at zero in add_defence).
        game.update_defence_combat(Fixed16_16::from_int(1));
        assert!(
            game.units[0].health < before,
            "obelisk fires on the first tick when in range"
        );
        assert!(
            game.buildings[0].reload > Fixed16_16::ZERO,
            "obelisk starts its charge-up reload after firing"
        );

        // While reloading, the obelisk holds fire.
        let after_first = game.units[0].health;
        game.update_defence_combat(Fixed16_16::from_int(1));
        assert_eq!(
            game.units[0].health, after_first,
            "obelisk does not fire while reloading"
        );
    }

    #[test]
    fn skirmish_starts_fully_revealed() {
        let game = Game::new("shroud", 32, 32);
        assert!(game.can_see(0, Vec2i::new(0, 0)));
        assert!(game.can_see(0, Vec2i::new(31, 31)));
    }

    #[test]
    fn enabling_shroud_hides_everything_until_refresh() {
        let mut game = Game::new("shroud", 32, 32);
        game.enable_shroud();
        assert!(
            !game.can_see(0, Vec2i::new(5, 5)),
            "nothing seen before a refresh"
        );

        // `Game::new` populates no objects, so a refresh with an empty board
        // reveals nothing. `setup_demo` adds units; sight follows them.
        game.refresh_shroud();
        assert_eq!(game.shroud.visible_count(0), 0);

        game.setup_demo();
        game.refresh_shroud();
        assert!(
            game.shroud.visible_count(0) > 0,
            "refreshing must reveal around friendly objects"
        );
    }

    #[test]
    fn movement_updates_visibility() {
        let mut game = Game::new("shroud", 32, 32);
        game.enable_shroud();
        game.units.clear();
        game.buildings.clear();

        let mut scout = Unit::new(UnitType::Jeep, Game::cell_center(1, 1), 0);
        scout.health = scout.max_health;
        game.units.push(scout);
        game.refresh_shroud();

        assert!(
            game.can_see(0, Vec2i::new(1, 1)),
            "the scout sees its own cell"
        );
        assert!(!game.can_see(0, Vec2i::new(20, 20)), "far cells stay dark");
    }

    #[test]
    fn sight_is_per_house() {
        let mut game = Game::new("shroud", 32, 32);
        game.enable_shroud();
        game.units.clear();
        game.buildings.clear();

        let mut friend = Unit::new(UnitType::Jeep, Game::cell_center(2, 2), 0);
        friend.health = friend.max_health;
        game.units.push(friend);
        game.refresh_shroud();

        assert!(game.can_see(0, Vec2i::new(2, 2)));
        assert!(
            !game.can_see(1, Vec2i::new(2, 2)),
            "house 1 must not inherit house 0's sight"
        );
    }

    #[test]
    fn mapped_history_survives_losing_sight() {
        let mut game = Game::new("shroud", 32, 32);
        game.enable_shroud();
        game.units.clear();
        game.buildings.clear();

        let mut scout = Unit::new(UnitType::Jeep, Game::cell_center(4, 4), 0);
        scout.health = scout.max_health;
        game.units.push(scout);
        game.refresh_shroud();
        let seen = Vec2i::new(4, 4);
        assert!(game.shroud.is_mapped(0, seen));

        game.units[0].health = Fixed16_16::ZERO;
        game.refresh_shroud();
        assert!(!game.can_see(0, seen), "no unit, no current sight");
        assert!(game.shroud.is_mapped(0, seen), "the cell stays on the map");
    }

    #[test]
    fn units_do_not_acquire_unseen_targets() {
        let mut game = Game::new("shroud", 32, 32);
        game.enable_shroud();
        game.units.clear();
        game.buildings.clear();

        let mut tank = Unit::new(UnitType::MediumTank, Game::cell_center(10, 10), 0);
        tank.health = tank.max_health;
        let mut buggy = Unit::new(UnitType::Buggy, Game::cell_center(12, 10), 1);
        buggy.health = buggy.max_health;
        game.units.push(tank);
        game.units.push(buggy);

        game.update_auto_acquire();
        assert!(
            game.units[0].attack_target.is_none(),
            "an unseen enemy must not be acquired"
        );
    }

    #[test]
    fn units_acquire_once_the_shroud_reveals_the_target() {
        let mut game = Game::new("shroud", 32, 32);
        game.enable_shroud();
        game.units.clear();
        game.buildings.clear();

        let mut tank = Unit::new(UnitType::MediumTank, Game::cell_center(10, 10), 0);
        tank.health = tank.max_health;
        let mut buggy = Unit::new(UnitType::Buggy, Game::cell_center(12, 10), 1);
        buggy.health = buggy.max_health;
        game.units.push(tank);
        game.units.push(buggy);

        game.shroud.reveal(0, Vec2i::new(12, 10), 0);
        game.update_auto_acquire();
        assert_eq!(
            game.units[0].attack_target,
            Some(1),
            "a visible enemy is acquired"
        );
    }

    #[test]
    fn default_game_acquires_targets_without_shroud() {
        // Regression guard: the default fully-revealed shroud blocks nothing.
        let mut game = Game::new("noshroud", 32, 32);
        game.units.clear();
        game.buildings.clear();

        let mut tank = Unit::new(UnitType::MediumTank, Game::cell_center(10, 10), 0);
        tank.health = tank.max_health;
        let mut buggy = Unit::new(UnitType::Buggy, Game::cell_center(12, 10), 1);
        buggy.health = buggy.max_health;
        game.units.push(tank);
        game.units.push(buggy);

        game.update_auto_acquire();
        assert_eq!(game.units[0].attack_target, Some(1));
    }

    #[test]
    fn defences_hold_fire_on_unseen_targets() {
        let mut game = Game::new("shroud", 32, 32);
        game.units.clear();
        game.buildings.clear();
        game.enable_shroud();

        add_defence(&mut game, 0, StructureType::GuardTower, Vec2i::new(10, 10));

        let mut enemy = Unit::new(UnitType::Jeep, Game::cell_center(11, 10), 1);
        enemy.health = enemy.max_health;
        let before = enemy.health;
        game.units.push(enemy);

        game.update_defence_combat(Fixed16_16::from_int(1));
        assert_eq!(
            game.units[0].health, before,
            "a tower cannot shell a cell its owner cannot see"
        );

        game.shroud.reveal(0, Vec2i::new(11, 10), 0);
        game.update_defence_combat(Fixed16_16::from_int(1));
        assert!(
            game.units[0].health < before,
            "the tower fires once the target is visible"
        );
    }

    #[test]
    fn five_infantry_share_cell_deterministically() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();

        let cell = Vec2i::new(4, 4);
        let mut indices = Vec::new();
        for _ in 0..5 {
            let unit = Unit::new(UnitType::InfantryE1, Game::cell_origin(4, 4), 0);
            let idx = game.units.len();
            game.units.push(unit);
            indices.push(idx);
            game.assign_infantry_sub_cell(idx, cell)
                .expect("slot available");
        }

        for (slot, &idx) in indices.iter().enumerate() {
            assert_eq!(
                game.units[idx].effective_sub_cell(),
                Some(slot as u8),
                "deterministic slot assignment"
            );
            let expected = Game::cell_origin(4, 4) + Unit::stopping_offset(slot as u8).unwrap();
            assert_eq!(game.units[idx].position, expected, "pinned sub-cell offset");
        }

        // Determinism: compacting again yields the same slots.
        let before: Vec<Option<u8>> = indices
            .iter()
            .map(|&idx| game.units[idx].effective_sub_cell())
            .collect();
        game.compact_infantry_sub_cells();
        let after: Vec<Option<u8>> = indices
            .iter()
            .map(|&idx| game.units[idx].effective_sub_cell())
            .collect();
        assert_eq!(before, after);

        // Sixth entry is rejected deterministically.
        let sixth = Unit::new(UnitType::InfantryE1, Game::cell_origin(4, 4), 0);
        let sixth_idx = game.units.len();
        game.units.push(sixth);
        assert_eq!(
            game.assign_infantry_sub_cell(sixth_idx, cell),
            Err(SubCellError::CellFull)
        );
    }

    #[test]
    fn infantry_stance_resolves_to_pinned_values() {
        let mut unit = Unit::new(UnitType::InfantryE1, Vec2::zero(), 0);
        assert_eq!(unit.effective_stance(), crate::Stance::Standing);
        assert_eq!(unit.stance_speed_multiplier(), 1.0);

        unit.set_stance(crate::Stance::Prone);
        assert!(unit.is_prone);
        assert_eq!(unit.effective_stance(), crate::Stance::Prone);
        assert_eq!(unit.stance_speed_multiplier(), 0.0);

        unit.set_stance(crate::Stance::Crawling);
        assert_eq!(unit.stance_speed_multiplier(), 0.5);
    }

    #[test]
    fn transport_door_blocks_then_allows_passage() {
        let mut unit = Unit::new(UnitType::APC, Vec2::zero(), 0);
        assert!(!unit.is_door_open());
        assert_eq!(unit.effective_door_state(), crate::DoorState::Closed);
        assert!(unit.open_door());
        assert!(unit.is_door_open());
        assert!(unit.close_door());
        assert!(!unit.is_door_open());
    }

    #[test]
    fn sub_cell_positions_survive_snapshot_round_trip() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let cell = Vec2i::new(3, 3);
        for _ in 0..5 {
            let idx = game.units.len();
            game.units
                .push(Unit::new(UnitType::InfantryE1, Game::cell_origin(3, 3), 0));
            game.assign_infantry_sub_cell(idx, cell).unwrap();
        }

        let bytes = game.to_save_bytes().unwrap();
        let reloaded = Game::from_save_bytes(&bytes).unwrap();
        assert_eq!(reloaded.units.len(), 5);
        for (slot, unit) in reloaded.units.iter().enumerate() {
            assert_eq!(unit.effective_sub_cell(), Some(slot as u8));
            let expected = Game::cell_origin(3, 3) + Unit::stopping_offset(slot as u8).unwrap();
            assert_eq!(unit.position, expected);
        }
        assert_eq!(
            game.to_save_bytes().unwrap(),
            reloaded.to_save_bytes().unwrap()
        );
    }

    #[test]
    fn malformed_sub_cell_snapshots_are_rejected() {
        // Six infantry in one cell.
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        for _ in 0..6 {
            game.units
                .push(Unit::new(UnitType::InfantryE1, Game::cell_origin(2, 2), 0));
        }
        // Force all six into the same cell without slot validation at construction.
        game.compact_infantry_sub_cells();
        // compact only assigns five; manually mark sixth as slot 0 (duplicate) to exercise validation.
        if let Some(unit) = game.units.get_mut(5) {
            unit.set_sub_cell(Some(0));
        }
        let bytes = game.to_save_bytes().unwrap();
        assert!(Game::from_save_bytes(&bytes).is_err());

        // Out-of-range slot.
        let mut game2 = Game::new("test", 16, 16);
        game2.units.clear();
        game2.houses.clear();
        game2.buildings.clear();
        game2.projectiles.clear();
        game2
            .houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        let mut u = Unit::new(UnitType::InfantryE1, Game::cell_origin(1, 1), 0);
        u.set_sub_cell(Some(9));
        game2.units.push(u);
        let bytes2 = game2.to_save_bytes().unwrap();
        assert!(Game::from_save_bytes(&bytes2).is_err());
    }

    #[test]
    fn malformed_duplicate_sub_slot_rejected() {
        let mut game = Game::new("test", 16, 16);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));
        let mut a = Unit::new(UnitType::InfantryE1, Game::cell_origin(2, 2), 0);
        a.set_sub_cell(Some(0));
        let mut b = Unit::new(UnitType::InfantryE1, Game::cell_origin(2, 2), 0);
        b.set_sub_cell(Some(0));
        game.units.push(a);
        game.units.push(b);
        let bytes = game.to_save_bytes().unwrap();
        assert!(Game::from_save_bytes(&bytes).is_err());
    }

    #[test]
    fn td_roster_per_unit_checks() {
        // Every roster unit must spawn with sane, non-default stats.
        let cases: &[(UnitType, &str)] = &[
            (UnitType::InfantryE1, "rifleman"),
            (UnitType::Engineer, "engineer"),
            (UnitType::Commando, "commando"),
            (UnitType::MediumTank, "medium tank"),
            (UnitType::MammothTank, "mammoth tank"),
            (UnitType::Harvester, "harvester"),
            (UnitType::APC, "APC"),
            (UnitType::Artillery, "artillery"),
            (UnitType::MLRS, "MLRS"),
            (UnitType::StealthTank, "stealth tank"),
            (UnitType::FlameTank, "flame tank"),
            (UnitType::MobileHQ, "mobile HQ"),
            (UnitType::Grenadier, "grenadier"),
            (UnitType::RocketSoldier, "rocket soldier"),
            (UnitType::Flamethrower, "flamethrower"),
            (UnitType::ChemWarrior, "chem warrior"),
            (UnitType::Chinook, "chinook"),
            (UnitType::Orca, "orca"),
            (UnitType::Apache, "apache"),
            (UnitType::Triceratops, "triceratops"),
            (UnitType::Trex, "T-rex"),
            (UnitType::Raptor, "raptor"),
            (UnitType::Stegosaurus, "stegosaurus"),
        ];

        for (unit_type, name) in cases {
            let unit = Unit::new(*unit_type, Vec2::zero(), 0);
            let stats = unit_type.stats();

            // Strength must be positive (non-zero).
            assert!(stats.strength > 0, "{name} strength must be > 0");
            // Cost must be non-negative.
            assert!(stats.cost >= 0, "{name} cost must be >= 0");
            // Sight range must be >= 0.
            assert!(stats.sight_range >= 0, "{name} sight range must be >= 0");
            // Tech level must be sane.
            assert!(stats.tech_level <= 99, "{name} tech level must be <= 99");
            // Health must match strength.
            assert_eq!(unit.max_health.to_f32(), stats.strength as f32);
            assert_eq!(unit.health, unit.max_health);
            // Unit must start alive.
            assert!(unit.health > Fixed16_16::ZERO);
            // Not deployed by default.
            assert!(!unit.deployed);
            // Infantry must be flagged infantry.
            if stats.is_infantry {
                assert!(stats.cost <= 1000, "{name} infantry cost sane");
            }
        }
    }

    #[test]
    fn td_deploy_command_sets_deployed_flag() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let mcv_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0)); // use APC as deployable stand-in
        game.units[mcv_idx].owner = 0;

        // Deploy must flip the flag.
        assert!(!game.units[mcv_idx].deployed);
        let result = game.apply_command(
            0,
            &Command::Deploy {
                unit: mcv_idx as u32,
            },
        );
        assert!(result, "deploy command must succeed");
        assert!(game.units[mcv_idx].deployed, "unit must be marked deployed");

        // Deploying another player's unit must fail.
        game.units[mcv_idx].deployed = false;
        game.units[mcv_idx].owner = 1;
        let result = game.apply_command(
            0,
            &Command::Deploy {
                unit: mcv_idx as u32,
            },
        );
        assert!(!result, "deploying another player's unit must fail");
        assert!(!game.units[mcv_idx].deployed);

        // Deploying an out-of-range index must fail.
        let result = game.apply_command(0, &Command::Deploy { unit: 9999 });
        assert!(!result, "deploying out-of-range unit must fail");
    }

    #[test]
    fn transport_loads_unloads_deterministically() {
        // Happy path: Chinook boards 5 infantry, unloads at waypoint.
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let chinook_idx = game.units.len();
        game.units.push(Unit::new(UnitType::Chinook, cell, 0));
        game.units[chinook_idx].owner = 0;

        let mut passenger_indices = Vec::new();
        for _ in 0..5 {
            let idx = game.units.len();
            game.units.push(Unit::new(
                UnitType::InfantryE1,
                Game::cell_origin(10, 10),
                0,
            ));
            game.units[idx].owner = 0;
            game.load_transport_at(chinook_idx, idx).unwrap();
            passenger_indices.push(idx);
        }

        // At capacity: 5th boarding rejected.
        let overflow_idx = game.units.len();
        game.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_origin(11, 11),
            0,
        ));
        game.units[overflow_idx].owner = 0;
        assert_eq!(
            game.load_transport_at(chinook_idx, overflow_idx),
            Err(TransportError::AtCapacity)
        );

        // All 5 aboard.
        assert_eq!(game.units[chinook_idx].passenger_count(), 5);

        // Unload at waypoint cell.
        let unload_cell = Vec2i::new(7, 7);
        game.unload_transport_at(chinook_idx, unload_cell).unwrap();
        assert_eq!(game.units[chinook_idx].passenger_count(), 0);

        // All passengers are back on the map at the unload cell.
        for idx in &passenger_indices {
            let passenger = &game.units[*idx];
            assert!(passenger.is_alive(), "passenger {idx} must be alive");
            assert_eq!(
                Game::unit_cell(passenger),
                Vec2i::new(7, 7),
                "passenger {idx} must be at unload cell"
            );
        }
    }

    #[test]
    fn transport_overload_boarding_rejected() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let apc_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0));
        game.units[apc_idx].owner = 0;

        // Fill capacity with 5 infantry.
        for _ in 0..5 {
            let idx = game.units.len();
            game.units.push(Unit::new(
                UnitType::InfantryE1,
                Game::cell_origin(10, 10),
                0,
            ));
            game.units[idx].owner = 0;
            game.load_transport_at(apc_idx, idx).unwrap();
        }

        // 6th boarding must be rejected deterministically.
        let overflow_idx = game.units.len();
        game.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_origin(11, 11),
            0,
        ));
        game.units[overflow_idx].owner = 0;
        assert_eq!(
            game.load_transport_at(apc_idx, overflow_idx),
            Err(TransportError::AtCapacity)
        );
    }

    #[test]
    fn transport_destroyed_kills_passengers() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let chinook_idx = game.units.len();
        game.units.push(Unit::new(UnitType::Chinook, cell, 0));
        game.units[chinook_idx].owner = 0;

        let mut passenger_indices = Vec::new();
        for _ in 0..3 {
            let idx = game.units.len();
            game.units.push(Unit::new(
                UnitType::InfantryE1,
                Game::cell_origin(10, 10),
                0,
            ));
            game.units[idx].owner = 0;
            game.load_transport_at(chinook_idx, idx).unwrap();
            passenger_indices.push(idx);
        }

        // Destroy the transport.
        game.units[chinook_idx].health = Fixed16_16::ZERO;

        // Kill passengers of destroyed transports.
        game.kill_passengers_of_destroyed_transports();

        // Transport passenger list cleared.
        assert_eq!(game.units[chinook_idx].passenger_count(), 0);

        // All passengers dead.
        for idx in &passenger_indices {
            assert!(!game.units[*idx].is_alive(), "passenger {idx} must be dead");
        }
    }

    #[test]
    fn transport_board_non_infantry_rejected() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let apc_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0));
        game.units[apc_idx].owner = 0;

        let tank_idx = game.units.len();
        game.units.push(Unit::new(
            UnitType::MediumTank,
            Game::cell_origin(10, 10),
            0,
        ));
        game.units[tank_idx].owner = 0;

        assert_eq!(
            game.load_transport_at(apc_idx, tank_idx),
            Err(TransportError::PassengerNotInfantry)
        );
    }

    #[test]
    fn transport_board_dead_passenger_rejected() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let apc_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0));
        game.units[apc_idx].owner = 0;

        let dead_idx = game.units.len();
        game.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_origin(10, 10),
            0,
        ));
        game.units[dead_idx].health = Fixed16_16::ZERO;
        game.units[dead_idx].owner = 0;

        assert_eq!(
            game.load_transport_at(apc_idx, dead_idx),
            Err(TransportError::PassengerNotAlive)
        );
    }

    #[test]
    fn transport_unload_cell_full_rejected() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let apc_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0));
        game.units[apc_idx].owner = 0;

        // Board 1 infantry.
        let passenger_idx = game.units.len();
        game.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_origin(10, 10),
            0,
        ));
        game.units[passenger_idx].owner = 0;
        game.load_transport_at(apc_idx, passenger_idx).unwrap();

        // Fill the unload cell with 5 infantry already.
        for _ in 0..5 {
            let idx = game.units.len();
            game.units
                .push(Unit::new(UnitType::InfantryE1, Game::cell_origin(7, 7), 0));
            game.units[idx].owner = 0;
            game.assign_infantry_sub_cell(idx, Vec2i::new(7, 7))
                .unwrap();
        }

        // Unload into full cell must fail.
        assert_eq!(
            game.unload_transport_at(apc_idx, Vec2i::new(7, 7)),
            Err(TransportError::CellFull)
        );
        // Passenger still aboard.
        assert_eq!(game.units[apc_idx].passenger_count(), 1);
    }

    /// Acceptance: passengers survive save/reload (snapshot round-trip).
    #[test]
    fn transport_passengers_survive_snapshot_round_trip() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let apc_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0));
        game.units[apc_idx].owner = 0;

        let mut passenger_indices = Vec::new();
        for _ in 0..3 {
            let idx = game.units.len();
            game.units.push(Unit::new(
                UnitType::InfantryE1,
                Game::cell_origin(10, 10),
                0,
            ));
            game.units[idx].owner = 0;
            game.load_transport_at(apc_idx, idx).unwrap();
            passenger_indices.push(idx);
        }
        assert_eq!(game.units[apc_idx].passenger_count(), 3);

        let bytes = game.to_save_bytes().unwrap();
        let reloaded = Game::from_save_bytes(&bytes).unwrap();

        // All passengers must still be aboard and alive.
        assert_eq!(reloaded.units[apc_idx].passenger_count(), 3);
        for &p in &passenger_indices {
            assert!(
                reloaded.units[p].is_alive(),
                "passenger {p} must survive reload"
            );
            assert!(reloaded.units[apc_idx].has_passenger(p));
        }

        // Byte-identical re-save (stale_state: load twice yields identical).
        assert_eq!(
            game.to_save_bytes().unwrap(),
            reloaded.to_save_bytes().unwrap(),
            "snapshot must round-trip byte-identically with passengers"
        );
    }

    /// Adversarial: a snapshot whose passenger list names a unit not aboard
    /// the transport must be rejected (deterministic typed error, never panic).
    #[test]
    fn malformed_passenger_not_aboard_is_rejected() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let apc_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0));
        game.units[apc_idx].owner = 0;

        let passenger_idx = game.units.len();
        game.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_origin(10, 10),
            0,
        ));
        game.units[passenger_idx].owner = 0;
        game.load_transport_at(apc_idx, passenger_idx).unwrap();

        // Corrupt: add a phantom passenger index that is out of bounds.
        let mut snapshot = game.snapshot();
        snapshot
            .units
            .get_mut(apc_idx)
            .unwrap()
            .passengers
            .push(9999);
        let result = Game::from_snapshot(snapshot);
        assert!(result.is_err(), "phantom passenger must be rejected");
        if let Err(e) = result {
            let err = e.to_string();
            assert!(
                err.contains("out of bounds") || err.contains("passenger"),
                "unexpected error: {err}"
            );
        }
    }

    /// Adversarial: a snapshot whose passenger count exceeds capacity must
    /// be rejected (deterministic typed error, never silent overflow).
    #[test]
    fn malformed_passenger_count_exceeds_capacity_is_rejected() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let apc_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0));
        game.units[apc_idx].owner = 0;

        // Board 5 (capacity).
        for _ in 0..5 {
            let idx = game.units.len();
            game.units.push(Unit::new(
                UnitType::InfantryE1,
                Game::cell_origin(10, 10),
                0,
            ));
            game.units[idx].owner = 0;
            game.load_transport_at(apc_idx, idx).unwrap();
        }

        // Corrupt: add a 6th passenger beyond capacity (in a different cell
        // so sub-cell validation passes first, then capacity check fires).
        let mut snapshot = game.snapshot();
        let extra_idx = snapshot.units.len();
        snapshot.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_origin(20, 20),
            0,
        ));
        snapshot
            .units
            .get_mut(apc_idx)
            .unwrap()
            .passengers
            .push(extra_idx);
        let result = Game::from_snapshot(snapshot);
        assert!(
            result.is_err(),
            "over-capacity passenger list must be rejected"
        );
        if let Err(e) = result {
            let err = e.to_string();
            assert!(
                err.contains("capacity") || err.contains("passenger"),
                "unexpected error: {err}"
            );
        }
    }

    /// Adversarial: a snapshot whose passenger references a dead unit must
    /// be rejected (deterministic typed error, never panic).
    #[test]
    fn malformed_passenger_references_dead_unit_is_rejected() {
        let cell = Game::cell_origin(5, 5);
        let mut game = Game::new("test", 32, 32);
        game.units.clear();
        game.houses.clear();
        game.buildings.clear();
        game.projectiles.clear();
        game.houses
            .push(House::new(ClassId::HouseGDI, "GDI".to_string()));

        let apc_idx = game.units.len();
        game.units.push(Unit::new(UnitType::APC, cell, 0));
        game.units[apc_idx].owner = 0;

        let passenger_idx = game.units.len();
        game.units.push(Unit::new(
            UnitType::InfantryE1,
            Game::cell_origin(10, 10),
            0,
        ));
        game.units[passenger_idx].owner = 0;
        game.load_transport_at(apc_idx, passenger_idx).unwrap();

        // Kill the passenger.
        game.units[passenger_idx].health = Fixed16_16::ZERO;
        assert!(!game.units[passenger_idx].is_alive());

        let bytes = game.to_save_bytes().unwrap();
        let result = Game::from_save_bytes(&bytes);
        assert!(result.is_err(), "dead-unit passenger must be rejected");
        if let Err(e) = result {
            let err = e.to_string();
            assert!(
                err.contains("not alive") || err.contains("passenger"),
                "unexpected error: {err}"
            );
        }
    }
}
