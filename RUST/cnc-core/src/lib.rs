//! C&C Core - Shared types, math, and utilities

pub mod class_id;
pub mod color;
pub mod entity;
pub mod fixed;
pub mod float_bias;
pub mod house_ai;
pub mod ini;
pub mod math;
pub mod object_pool;
pub mod palette;
pub mod pointers;
pub mod rules;
pub mod serialization;
pub mod shroud;
pub mod smart_ptr;
pub mod trigger_apply;
pub mod trigger_attach;
pub mod trigger_records;
pub mod visibility;
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
pub use house_ai::{
    choose_weighted, run_expert_ai, AIDifficulty, BaseBuildingRole, BuildTask, HouseAIConfig,
    HouseAIState, ProductionWeight,
};
pub use ini::{IniFile, IniSection, IniValue};
pub use math::{Bounds, Mat2x2, Mat3x3, Rect, Vec2, Vec2i, Vec3, Vec3i};
pub use object_pool::{ObjectPool, PoolError, PoolHandle, PoolStats, ThreadLocalPool};
pub use palette::Palette;
pub use pointers::{CodePointers, DecodePointers};
pub use rules::{
    BuildChoice, DifficultyLevel, DifficultyRule, GeneralRules, HouseRule, RulesDatabase,
    SpecialFlags, TypeRule, WarheadRule, WeaponRule,
};
pub use serialization::{BinaryReader, BinaryWriter, Serializable};
pub use shroud::{Shroud, MAX_HOUSES};
pub use smart_ptr::CCPtr;
pub use trigger_apply::{
    apply_action_record, apply_actions, events_hold, fire_trigger, special_weapon, ApplyReport,
    TriggerHost,
};
pub use trigger_attach::{
    AttachTarget, AttachmentFiring, AttachmentId, AttachmentSpec, TriggerAttachment,
    TriggerAttachments, TriggerMode, TriggerSubject,
};
pub use visibility::{CellState, FogPolicy, SightSource, Visibility, VISIBILITY_SNAPSHOT_VERSION};
