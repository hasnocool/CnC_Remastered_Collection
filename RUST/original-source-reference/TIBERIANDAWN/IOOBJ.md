# PATH/FILE
original/TIBERIANDAWN/IOOBJ.CPP

## Purpose
Save/load (serialization) system for Tiberian Dawn game objects. Implements `Load()`, `Save()`, `Code_Pointers()`, and `Decode_Pointers()` for every game object type. Pointer fixup via target encoding/decoding allows saved games to load without relying on exact memory locations. 2707 lines - one of the largest core files.

## Where it fits in the game
Save/load infrastructure - every game object (teams, triggers, aircraft, animations, buildings, bullets, infantry, overlays, reinforcements, smudges, templates, missions, objects) has Load/Save/Code_Pointers/Decode_Pointers methods in this file. Called from `SAVELOAD.CPP` during save/load operations. Critical for deterministic savegame compatibility across runs.

## Classes, structs, enums and important data

### Code_Pointers / Decode_Pointers pattern
**Pointer fixup system** - Converts pointers to target numbers before save, and back to pointers after load:

**Code_Pointers()** (pre-save encoding):
1. `Next = (ObjectClass *)Next->As_Target()` - convert pointer to target number
2. `Trigger = (TriggerClass *)Trigger->As_Target()` - convert pointer to target number

**Decode_Pointers()** (post-load decoding):
1. `Next = As_Object((TARGET)Next, false)` - convert target back to pointer
2. `Check_Ptr((void *)Next, __FILE__, __LINE__)` - validate pointer
3. `Trigger = As_Trigger((TARGET)Trigger, false)` - convert target back to pointer
4. `Check_Ptr((void *)Trigger, __FILE__, __LINE__)` - validate pointer

**Why this works:** Target numbers are stable identifiers that don't depend on memory layout, allowing saved games to load even if objects are at different addresses.

## Functions and methods

### Class Load/Save methods (all follow same pattern)

Each class implements:
| Method | Purpose |
|--------|---------|
| `Load(file)` | Read object state from save game file |
| `Save(file)` | Write object state to save game file |
| `Code_Pointers()` | Encode pointers as target numbers before save |
| `Decode_Pointers()` | Decode target numbers back to pointers after load |

### Classes with Load/Save/Code_Pointers/Decode_Pointers:
| Class | Description |
|-------|-------------|
| `TeamTypeClass` | AI team type definitions |
| `TeamClass` | Active team instances |
| `TriggerClass` | Triggers and events |
| `AircraftClass` | Flying objects |
| `AnimClass` | Animations/effects |
| `BuildingClass` | Buildings/structures |
| `BulletClass` | Projectiles |
| `InfantryClass` | Infantry units |
| `OverlayClass` | Map overlays |
| `ReinforcementClass` | Reinforcement schedules |
| `SmudgeClass` | Smudge marks (tire tracks, etc.) |
| `TemplateClass` | Template stamp objects |
| `MissionClass` | Mission objectives |
| `ObjectClass` | Base object class |

### Save/Load format
- Binary write/read of class fields
- Pointer fields encoded/decoded via `Code_Pointers()`/`Decode_Pointers()`
- String fields use length-prefixed format
- Array fields written element-by-element

### MissionClass::Code_Pointers()
Delegates to `ObjectClass::Code_Pointers()` for pointer encoding.

### MissionClass::Decode_Pointers()
Delegates to `ObjectClass::Decode_Pointers()` for pointer decoding.

### ObjectClass::Code_Pointers()
Base class pointer encoding:
- `Next` pointer → target number via `As_Target()`
- `Trigger` pointer → target number via `As_Target()`

### ObjectClass::Decode_Pointers()
Base class pointer decoding:
- Target number → `Next` pointer via `As_Object(target, false)`
- `Check_Ptr()` validation on result
- Target number → `Trigger` pointer via `As_Trigger(target, false)`
- `Check_Ptr()` validation on result

## Inputs, outputs and side effects

**State mutations:**
- All object fields serialized/deserialized
- Pointer fields temporarily replaced with target numbers during Code_Pointers()
- Pointer fields restored from target numbers during Decode_Pointers()
- `Check_Ptr()` validates decoded pointers (may abort on invalid)

**External systems affected:**
- `SAVELOAD.CPP` → calls Load/Save for each object
- `TARGET.CPP` → `As_Target()`, `As_Object()`, `As_Trigger()` conversions
- Savegame file format compatibility
- Multiplayer savegame synchronization

## Calls into / called by

**Called by:**
- `SAVELOAD.CPP` → Load/Save for each object type during save/load
- Save system → Code_Pointers() before writing pointers
- Load system → Decode_Pointers() after reading pointers

**Calls into:**
- `As_Target()` → convert pointer to target number
- `As_Object(target, false)` → convert target to object pointer
- `As_Trigger(target, false)` → convert target to trigger pointer
- `Check_Ptr()` → validate decoded pointer
- File I/O functions for binary read/write

## Related original files
- **IOOBJ.H** - Header declarations for all IO methods
- **SAVELOAD.CPP** → orchestrates Load/Save calls
- **TARGET.CPP** → pointer/target conversion functions
- **ObjectClass.H** - Base class with Next/Trigger pointers

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/saveload/object.rs` - ObjectClass Load/Save/Code/Decode
- `cnc-tiberian-dawn/src/saveload/building.rs` - BuildingClass serialization
- `cnc-tiberian-dawn/src/saveload/unit.rs` - Unit/Infantry/Aircraft serialization
- `cnc-tiberian-dawn/src/saveload/bullet.rs` - BulletClass serialization
- `cnc-tiberian-dawn/src/saveload/trigger.rs` - TriggerClass serialization

**Not yet ported:**
- Binary savegame format compatibility
- Pointer fixup via target number encoding
- Code_Pointers/Decode_Pointers pattern for all 13 classes
- Check_Ptr validation
- Savegame version compatibility
- Multiplayer savegame sync

## Edge cases and compatibility notes

1. **Pointer stability**: Target numbers are stable across save/load; raw pointers are not
2. **Check_Ptr validation**: Decoded pointers validated against file/line for debugging
3. **MissionClass delegation**: Mission uses ObjectClass's Code/Decode_Pointers directly
4. **Binary format**: All classes use binary field-by-field serialization
5. **False parameter**: `As_Object(target, false)` and `As_Trigger(target, false)` skip active check (pointers may be to inactive objects during load)
6. **2707 lines**: One of the largest core files - covers all game object types
7. **Overlay module**: Comment notes all object loading/saving should be in this module for overlay capability

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/IOOBJ.CPP at f1f0d42
