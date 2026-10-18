//! C&C Core - Shared types, math, and utilities

pub mod fixed;
pub mod float_bias;
pub mod math;
pub mod color;
pub mod palette;
pub mod serialization;
pub mod ini;
pub mod rules;
pub mod pointers;
pub mod smart_ptr;
pub mod object_pool;
pub mod class_id;

// Re-exports
pub use fixed::Fixed16_16;
pub use float_bias::FloatBias;
pub use math::{Vec2, Vec3, Vec2i, Vec3i, Mat2x2, Mat3x3, Rect, Bounds};
pub use color::{RGB555, RGB565, RGB888, ARGB8888, PaletteIndex, PlayerRemap};
pub use palette::Palette;
pub use serialization::{BinaryReader, BinaryWriter, Serializable};
pub use ini::{IniFile, IniSection, IniValue};
pub use rules::{DifficultyLevel, DifficultyRule, GeneralRules, HouseRule, RulesDatabase, TypeRule, WarheadRule, WeaponRule};
pub use pointers::{CodePointers, DecodePointers};
pub use smart_ptr::CCPtr;
pub use object_pool::{ObjectPool, PoolError, PoolStats, PoolHandle, ThreadLocalPool};
pub use class_id::ClassId;
