# RUST/cnc-core/

## Responsibility

Foundation crate shared by every game/parser crate: Westwood-compatible numeric and data primitives (16.16 fixed-point, float bias, INI, Rules.ini, class IDs, serialization, pools, colors/palettes, math). No rendering, no game logic.

## Design

- Faithful ports of Westwood idioms as Rust types: `Fixed16_16` (`fixed.rs`, 610 lines, the RA `fixed` 16.16 type), `FloatBias` (`float_bias.rs`, TD's float-bias system), `ClassId` enum (`class_id.rs`, the RTTI replacement), `CCPtr<T>` (`smart_ptr.rs`), `ObjectPool`/`ThreadLocalPool` (`object_pool.rs`), `CodePointers`/`DecodePointers` (`pointers.rs`, save/load pointer fixup).
- Data-driven registries: `RulesDatabase` (`rules.rs`, 902 lines) parses Rules.ini into `GeneralRules`, `DifficultyRule`, `WeaponRule`, `WarheadRule`, `TypeRule`, `HouseRule`; `IniFile`/`IniSection`/`IniValue` (`ini.rs`) extends the Westwood format with inheritance.
- Binary IO: little-endian, versioned `BinaryReader`/`BinaryWriter`/`Serializable` trait (`serialization.rs`); `Palette` with 256-colour remap (`palette.rs`, `color.rs` with `RGB555/RGB565/RGB888/ARGB8888`, `PaletteIndex`, `PlayerRemap`); `Vec2/Vec2i/Vec3/Vec3i/Rect/Bounds/Mat2x2/Mat3x3` over fixed-point (`math.rs`, 868 lines).
- lib.rs re-exports the primary types; errors via `thiserror`, IO contexts via `anyhow`; deps: serde, bytemuck, bitflags, ahash, bincode, flate2/lzw, nom, chrono, uuid, parking_lot. Dev deps: `proptest`, `criterion` (determinism + bench).

## Flow

Input: byte streams (INI text, binary saves, palette blobs) or Rules.ini paths from a game crate. `ini::IniFile::parse` → `RulesDatabase::load`; `serialization::BinaryWriter/Reader` underpin savegames in `cnc-tiberian-dawn::savegame` and `BinarySerializable` data elsewhere. Output: validated structured values; no globals, no I/O after construction.

## Integration

- Consumed by: every other member (`cnc-renderer`, `cnc-audio`, `cnc-asset`, both engines, `cnc-map-editor`, `cnc-launcher`).
- Depends on: external crates only — it is the dependency root of the workspace.
- 57 unit tests incl. INI and Rules.ini round-trips (see BUILD_STATUS.md).
