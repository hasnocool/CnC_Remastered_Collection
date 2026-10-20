# RUST/cnc-core/src/

## Responsibility

Twelve flat modules implementing ported Westwood primitives; `lib.rs` declares the modules and re-exports their primary types.

## Design

- `fixed.rs` — `Fixed16_16`: transmutation-safe 16.16 with `from_f32/into_f32`, arithmetic matching Westwood's `fixed`.
- `float_bias.rs` — `FloatBias`: TD's float-bias grading type; kept separate from RA fixpoint (do not unify, per root AGENTS.md).
- `class_id.rs` — `ClassId` enum: RTTI replacement shared by units/buildings/houses across engines.
- `entity.rs` — Phase 5.1 foundation: `Object` (ObjectId + owner `ClassId` + hp/strength + `ObjectFlags`, `HasObject` composition trait), `TaskForce`/`TeamType`/`Team` (mission teams: members, `TeamObjective`, 0–100 progress) and the core-level `Trigger`/`TriggerEvent`/`TriggerAction` records; INI helpers parse the `[TaskForces]`/`[TeamTypes]`/`[Triggers]` dialect (`taskforces_from_ini`, `team_types_from_ini`, `triggers_from_ini`), token maps (`house_class_from_token`, `class_id_from_token`, `team_objective_from_token`, `trigger_event_from_token`, `trigger_action_from_token`). vs. game crates: additive shared foundation — TD's `mission.rs` Trigger and per-crate `house.rs` stay authoritative in their own layer.
- `math.rs` — `Vec2`, `Vec2i`, `Vec3`, `Vec3i`, `Rect`, `Bounds`, `Mat2x2`, `Mat3x3` with fixed-point-friendly ops.
- `color.rs` / `palette.rs` — `RGB555/RGB565/RGB888/ARGB8888`, `PaletteIndex`, `PlayerRemap`; `Palette` (256 colours, gamma/grayscale helpers, remap).
- `ini.rs` — `IniFile`, `IniSection`, `IniValue`: Westwood INI parser + inheritance support.
- `rules.rs` — `RulesDatabase` + `GeneralRules`, `DifficultyLevel`/`DifficultyRule`, `WeaponRule`, `WarheadRule`, `TypeRule`, `HouseRule`: shared Rules.ini balance data for both engines.
- `serialization.rs` — `BinaryReader`/`BinaryWriter` and the `Serializable` trait (versioned little-endian).
- `smart_ptr.rs` — `CCPtr<T>` reference-counted smart pointer analogue.
- `object_pool.rs` — `ObjectPool`, `PoolHandle`, `PoolError`, `PoolStats`, `ThreadLocalPool` for hot allocations.
- `pointers.rs` — `CodePointers`/`DecodePointers`: pointer<->index fixup for save/load, replacing the original's raw `Code_Pointers`.
- `lib.rs` — module declarations + re-exports (e.g. `pub use rules::{DifficultyLevel, RulesDatabase, WeaponRule, ...}`).

## Flow

No call flow between modules beyond type reuse: `ini` feeds `rules`; `serialization` types consume `fixed`/`color`; `math` types are consumed by `pointers`-fixed layouts. Everything exits as plain data for caller crates.

## Integration

- Consumed by all sibling crates; `RulesDatabase` API surface (`DifficultyLevel`, `WeaponRule`, `WarheadRule`) is what `cnc-tiberian-dawn::ai` and `td_rules` read at production time.
- Uses deps declared in `cnc-core/Cargo.toml` (bitflags for flag sets, bincode for structured saves, flate2/lzw for compressed blobs, nom for low-level parsing, serde for `Serialize/Deserialize` derivation).
