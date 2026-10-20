//! C&C Core - Shared types, math, and utilities

pub mod class_id;
pub mod color;
pub mod entity;
pub mod fixed;
pub mod float_bias;
pub mod ini;
pub mod math;
pub mod object_pool;
pub mod palette;
pub mod pointers;
pub mod rules;
pub mod serialization;
pub mod shroud;
pub mod smart_ptr;
pub mod trigger_records;

// Re-exports
pub use class_id::ClassId;
pub use color::{PaletteIndex, PlayerRemap, ARGB8888, RGB555, RGB565, RGB888};
pub use entity::{
    class_id_from_token, house_class_from_token, taskforces_from_ini, team_types_from_ini,
    triggers_from_ini, HasObject, Object, ObjectFlags, TaskForce, Team, TeamMember, TeamObjective,
    TeamType, Trigger, TriggerAction, TriggerEvent,
};
pub use fixed::Fixed16_16;
pub use float_bias::FloatBias;
pub use ini::{IniFile, IniSection, IniValue};
pub use math::{Bounds, Mat2x2, Mat3x3, Rect, Vec2, Vec2i, Vec3, Vec3i};
pub use object_pool::{ObjectPool, PoolError, PoolHandle, PoolStats, ThreadLocalPool};
pub use palette::Palette;
pub use pointers::{CodePointers, DecodePointers};
pub use rules::{
    DifficultyLevel, DifficultyRule, GeneralRules, HouseRule, RulesDatabase, TypeRule, WarheadRule,
    WeaponRule,
};
pub use serialization::{BinaryReader, BinaryWriter, Serializable};
pub use shroud::{Shroud, MAX_HOUSES};
pub use smart_ptr::CCPtr;
