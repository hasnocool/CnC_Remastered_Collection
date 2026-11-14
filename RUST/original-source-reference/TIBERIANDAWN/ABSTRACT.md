# PATH/FILE
original/TIBERIANDAWN/ABSTRACT.CPP

## Purpose
Base abstract classes for Tiberian Dawn's type system. Provides `AbstractClass` (root of all game objects) and `AbstractTypeClass` (type metadata) with RTTI replacement, distance calculation, and vtable hack for MSVC destructor compatibility.

## Where it fits in the game
Type system foundation - Root of all game object hierarchies. `ObjectClass` → `AbstractClass`, and all type classes (`UnitTypeClass`, `BuildingTypeClass`, etc.) → `AbstractTypeClass`. Provides RTTI replacement via `What_Am_I()` and vtable management.

## Classes, structs, enums and important data

### AbstractClass
**Root game object class** - Common interface for all game entities.

**Key methods:**
- `Distance(TARGET)` → int (leptons)
- `What_Am_I()` → `RTTI_ABSTRACT` (virtual)
- `Coord_Fixup(COORDINATE)` → COORDINATE (virtual)
- `Full_Name()` → int (text ID, virtual)
- `Get_Ownable()` → unsigned short (virtual)
- `Delete_This()` - Vtable hack for MSVC
- `As_Building(TARGET)` → BuildingClass* (helper)
- `As_Coord(TARGET)` → COORDINATE (helper)

### AbstractTypeClass
**Type metadata class** - Describes object types from INI.

**Key data members:**
- `Name` (int) - Text string ID for display name
- `IniName` (char[sizeof]) - INI identifier string

**Constructor:** `AbstractTypeClass(name_id, ini_string)`

**Methods:**
- `What_Am_I()` → `RTTI_ABSTRACTTYPE`
- `Coord_Fixup(coord)` → coord (identity)
- `Full_Name()` → `Name` (text ID)
- `Get_Ownable()` → 0xffff (all ownable by default)

## Functions and methods

### AbstractClass::Distance(target) → int (leptons)
**Calculates range to target** - Used for weapon range checks.

**Algorithm:**
1. `dist = Distance(As_Coord(target))` (coordinate distance)
2. If target is `BuildingClass` and active:
   - `dist -= ((Width() + Height()) * (0x100 / 4))` (building radius fudge)
   - `dist = max(0, dist)`
3. Returns distance in leptons (1/256 pixel)

### AbstractTypeClass::AbstractTypeClass(name, ini)
**Constructor** - Initializes type metadata.
1. `Name = name` (text string ID)
2. `strncpy(IniName, ini, sizeof(IniName))`
3. Null-terminates `IniName`

### Virtual method defaults
| Method | Default return |
|--------|----------------|
| `What_Am_I()` | `RTTI_ABSTRACTTYPE` |
| `Coord_Fixup(coord)` | `coord` (identity) |
| `Full_Name()` | `Name` (text ID) |
| `Get_Ownable()` | `0xffff` (all bits set) |

### Delete_This() - MSVC vtable hack
**Problem:** MSVC overwrites vtable pointer during destructor chain, breaking virtual calls on "deleted" objects (C&C pattern: `Delete_This()` → virtual call).

**Solution:**
```cpp
unsigned long *this_ptr = (unsigned long*)this;
unsigned long vtable_ptr = *this_ptr;
delete this;  // Calls operator delete, doesn't deallocate
*this_ptr = vtable_ptr;  // Restore original vtable
```

**Note:** Watcom preserved vtable; MSVC doesn't. This restores it for post-delete virtual calls.

## RTTI system (replaces C++ RTTI)
**RTTIType enum** (partial):
- `RTTI_ABSTRACT` - AbstractClass
- `RTTI_ABSTRACTTYPE` - AbstractTypeClass
- `RTTI_OBJECT` - ObjectClass
- `RTTI_TECHNO` - TechnoClass
- `RTTI_UNIT` - UnitClass
- `RTTI_INFANTRY` - InfantryClass
- `RTTI_AIRCRAFT` - AircraftClass
- `RTTI_BUILDING` - BuildingClass
- `RTTI_BULLET` - BulletClass
- `RTTI_ANIM` - AnimClass
- `RTTI_TEAM` - TeamClass
- `RTTI_TRIGGER` - TriggerClass
- `RTTI_TEAMTYPE` - TeamTypeClass
- `RTTI_TECHNOTYPE` - TechnoTypeClass
- `RTTI_UNITTYPE` - UnitTypeClass
- `RTTI_INFANTRYTYPE` - InfantryTypeClass
- `RTTI_AIRCRAFTTYPE` - AircraftTypeClass
- `RTTI_BUILDINGTYPE` - BuildingTypeClass
- `RTTI_WEAPONTYPE` - WeaponTypeClass
- `RTTI_TERRAIN` - TerrainClass
- `RTTI_OVERLAY` - OverlayClass
- `RTTI_SMUDGE` - SmudgeClass
- `RTTI_TEMPLATE` - TemplateClass
- `RTTI_FACTORY` - FactoryClass
- `RTTI_HOUSE` - HouseClass

Used instead of C++ `dynamic_cast`/`typeid` for performance and binary compatibility.

## Helper functions (referenced)
- `As_Building(TARGET)` → `BuildingClass*`
- `As_Coord(TARGET)` → `COORDINATE`
- `Distance(COORDINATE)` → int (leptons)
- `Width()/Height()` → int (pixels)
- `IsActive` (bool)

## Inputs, outputs and side effects

**State mutations:** None (mostly queries)
**Vtable hack:** Temporarily corrupts vtable during `delete`, restores after

## Calls into / called by

**Called by:**
- Weapon range checks → `Distance()`
- Type queries → `What_Am_I()`
- Object deletion → `Delete_This()`
- Type metadata → `AbstractTypeClass` constructor

**Calls into:**
- `As_Building()`, `As_Coord()` (TARGET helpers)
- `Distance(COORDINATE)` (coordinate math)
- `Width()/Height()` (building dimensions)
- `delete` / `operator delete` (memory)

## Related original files
- **ABSTRACT.H** - Class declarations, RTTIType enum
- **OBJECT.H/.CPP** - ObjectClass (derives from AbstractClass)
- **TECHNOTYPE.H/.CPP** - TechnoTypeClass (derives AbstractTypeClass)
- **UNITTYPE.H/.CPP** - UnitTypeClass
- **BUILDINGTYPE.H/.CPP** - BuildingTypeClass
- **FUNCTION.H** - RTTIType enum, helpers
- **TARGET.H** - TARGET type, As_Building/As_Coord

## Rust port mapping
**Crates:** `cnc-core`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-core/src/type_system.rs` - AbstractClass, AbstractTypeClass
- `cnc-core/src/rtti.rs` - RTTIType enum, What_Am_I pattern
- `cnc-tiberian-dawn/src/type_system/td_types.rs` - TD type registry

**Not yet ported:**
- MSVC vtable hack (Delete_This) - not needed in Rust
- RTTI replacement (What_Am_I virtual)
- Distance with building radius fudge
- Coord_Fixup virtual (identity default)
- Get_Ownable virtual (ownership bitmask)
- IniName string storage (fixed buffer)
- Text ID indirection (Name → Text_String)

## Edge cases and compatibility notes

1. **MSVC vs Watcom**: Vtable hack only needed for MSVC; Watcom preserved vtable
2. **Delete_This pattern**: `delete this` calls `operator delete` (no dealloc), then restores vtable
3. **Building distance fudge**: Subtracts `(Width+Height)*64` leptons (≈1/4 pixel per dimension)
4. **RTTI enum**: Manual enum replaces C++ RTTI; `What_Am_I()` virtual dispatch
5. **AbstractTypeClass**: Not a full RTTI; just metadata (name, INI name)
6. **Get_Ownable**: Returns 0xffff (all ownable) by default; overridden for neutral/tech
7. **Delete_This safety**: Only works if memory not actually freed (C&C pools)
8. **Vtable pointer**: Stored as `unsigned long*` (platform-dependent size)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/ABSTRACT.CPP at f1f0d42
