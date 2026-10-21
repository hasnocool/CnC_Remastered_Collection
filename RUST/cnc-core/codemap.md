# RUST/cnc-core/

## Responsibility

Foundation crate shared by every game/parser crate: Westwood-compatible numeric and data primitives (16.16 fixed-point, float bias, INI, Rules.ini, class IDs, serialization, pools, colors/palettes, math, shroud, triggers, visibility). No rendering, no game logic.

## Design

- Faithful ports of Westwood idioms as Rust types:
  - `Fixed16_16` (`fixed.rs`, the RA `fixed` 16.16 type)
  - `FloatBias` (`float_bias.rs`, TD's float-bias system)
  - `ClassId` enum (`class_id.rs`, the RTTI replacement)
  - `CCPtr<T>` (`smart_ptr.rs`)
  - `ObjectPool`/`ThreadLocalPool` (`object_pool.rs`)
  - `CodePointers`/`DecodePointers` (`pointers.rs`, save/load pointer fixup)
- Data-driven registries:
  - `RulesDatabase` (`rules.rs`) parses Rules.ini into `GeneralRules`, `DifficultyRule`, `WeaponRule`, `WarheadRule`, `TypeRule`, `HouseRule`
  - `IniFile`/`IniSection`/`IniValue` (`ini.rs`) extends the Westwood format with inheritance
- Binary IO: little-endian, versioned `BinaryReader`/`BinaryWriter`/`Serializable` trait (`serialization.rs`)
- Visuals: `Palette` with 256-colour remap (`palette.rs`), `color.rs` with `RGB555/RGB565/RGB888/ARGB8888`, `PaletteIndex`, `PlayerRemap`
- Math: `Vec2/Vec2i/Vec3/Vec3i/Rect/Bounds/Mat2x2/Mat3x3` over fixed-point (`math.rs`)
- Entity System: `entity.rs` - `Object`, `TaskForce`, `Team`, `TeamType`, `Trigger`, `TriggerAction`, `TriggerEvent`, `HasObject`
- Shroud/Fog of War: `shroud.rs` - `Shroud`, `MAX_HOUSES`
- Visibility: `visibility.rs` - `CellState`, `FogPolicy`, `SightSource`, `Visibility`, `VISIBILITY_SNAPSHOT_VERSION`
- Triggers: 
  - `trigger_records.rs` - trigger data structures
  - `trigger_apply.rs` - `apply_action_record`, `apply_actions`, `fire_trigger`, `special_weapon`, `ApplyReport`, `TriggerHost`
  - `trigger_attach.rs` - `AttachTarget`, `AttachmentFiring`, `AttachmentId`, `AttachmentSpec`, `TriggerAttachment`, `TriggerAttachments`, `TriggerMode`, `TriggerSubject`
- lib.rs re-exports the primary types; errors via `thiserror`, IO contexts via `anyhow`
- Deps: serde, bytemuck, bitflags, ahash, bincode, flate2/lzw, nom, chrono, uuid, parking_lot
- Dev deps: `proptest`, `criterion` (determinism + bench)

## Flow

Input: byte streams (INI text, binary saves, palette blobs) or Rules.ini paths from a game crate. `ini::IniFile::parse` → `RulesDatabase::load`; `serialization::BinaryWriter/Reader` underpin savegames in `cnc-tiberian-dawn::savegame` and `BinarySerializable` data elsewhere. Output: validated structured values; no globals, no I/O after construction.

## Integration

- Consumed by: every other member (`cnc-renderer`, `cnc-audio`, `cnc-asset`, both engines, `cnc-map-editor`, `cnc-launcher`)
- Depends on: external crates only (no intra-workspace deps)

## Key Files

- src/lib.rs - Module declarations and comprehensive re-exports
- src/fixed.rs - Fixed16_16 16.16 fixed-point arithmetic (610 lines)
- src/float_bias.rs - TD's FloatBias system
- src/class_id.rs - ClassId enum (RTTI replacement)
- src/smart_ptr.rs - CCPtr<T> smart pointer
- src/object_pool.rs - ObjectPool, ThreadLocalPool
- src/pointers.rs - CodePointers, DecodePointers (save/load fixup)
- src/ini.rs - Extended INI parser with inheritance
- src/rules.rs - RulesDatabase, Rules.ini parsing (902 lines)
- src/serialization.rs - BinaryReader, BinaryWriter, Serializable trait
- src/palette.rs - Palette with 256-color remap
- src/color.rs - RGB555, RGB565, RGB888, ARGB8888, PaletteIndex, PlayerRemap
- src/math.rs - Fixed-point vector/matrix math (868 lines)
- src/entity.rs - Object, TaskForce, Team, Trigger types
- src/shroud.rs - Shroud, MAX_HOUSES
- src/visibility.rs - CellState, FogPolicy, SightSource, Visibility
- src/trigger_records.rs - Trigger data structures
- src/trigger_apply.rs - Trigger action application
- src/trigger_attach.rs - Trigger attachment system

