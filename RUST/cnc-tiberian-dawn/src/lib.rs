//! Tiberian Dawn Game Logic

pub mod ai;
pub mod building;
pub mod game;
pub mod house;
pub mod map;
pub mod mission;
pub mod network;
pub mod savegame;
pub mod pathfind;
pub mod td_rules;
pub mod unit;
pub mod weapon;

pub use ai::{AIDifficulty, ProductionWeight, AI};
pub use building::{Building, Factory, PowerPlant, Refinery};
pub use game::{Buildable, Game, GameStatus, SidebarOption};
pub use house::House;
pub use map::{CellGrid, Map};
pub use mission::{
    Action, Comparison, Condition, Mission, MissionOutcome, Objective, Script, Trigger,
};
pub use network::{
    CommandQueue, LiveEvent, LiveSession, NetworkConfig, PeerHandshake, Replay,
};
pub use savegame::{GameSnapshot, SAVE_MAGIC, SAVE_VERSION};
pub use td_rules::{ArmorType, StructureType, UnitType, WarheadType, WeaponType};
pub use unit::{Aircraft, Infantry, Unit, Vehicle};
pub use weapon::{Projectile, Warhead, Weapon};
