# AADATA.CPP Documentation

## File Overview

- **Path**: `original/TIBERIANDAWN/AADATA.CPP`
- **Header**: `original/TIBERIANDAWN/AADATA.H`
- **Programmer**: Joe L. Bostic
- **Start Date**: July 22, 1994
- **Last Update**: August 7, 1995 [JLB]
- **Description**: Implements aircraft type data for Command & Conquer Tiberian Dawn. Defines the `AircraftTypeClass` that stores configuration for all aircraft types (fixed-wing and rotary-wing), including their stats, weapons, armor, abilities, and INI names. Contains global rotor data pointers and five pre-initialized `AircraftTypeClass const` instances representing the five aircraft types in the game: TransportHeli, AttackPlane, AttackHeli, CargoPlane, and OrcaHeli. Also provides the `AircraftTypeClass::Pointers` lookup table and initialization routine.

---

## Purpose

This file is the central data definitions file for all aircraft types in Tiberian Dawn. It declares five static `AircraftTypeClass const` objects that embody the five flyable aircraft types in the game, each fully configured with their build level, prerequisites, weapons, armor, speed, abilities, and ownership flags. It also provides:

- Global rotor data pointers (`LRotorData`, `RRotorData`) used for rendering rotor animations
- An array `AircraftTypeClass::Pointers` mapping `AircraftType` enum values to their type class instances
- An `Init(theater)` function that loads theater-specific cameo icons for each aircraft type
- The `AircraftTypeClass` constructor that initializes type-specific fields not covered by the base `TechnoTypeClass`

It serves as the authoritative source for aircraft type configuration that the rest of the game engine reads when creating, placing, or rendering aircraft objects.

## Where it fits in the game

**Inheritance chain**: `AircraftTypeClass` → `TechnoTypeClass` → `ObjectTypeClass` → `AbstractTypeClass`

The `AircraftTypeClass` extends `TechnoTypeClass` with aircraft-specific attributes: `IsRotorEquipped`, `IsRotorCustom`, `IsLandable`, `IsFixedWing`, and `ROT` (rotation index). These types are referenced by `AircraftClass` objects (which inherit from `FootClass` and `FlyClass`) to determine the aircraft's stats, weapons, appearance, and behavior.

Five global constants of type `AircraftTypeClass const` are constructed at namespace scope, each corresponding to one of the five aircraft types:

| Aircraft Type | INI Name | Code Enum |
|---|---|---|
| Transport helicopter | TRAN | `AIRCRAFT_TRANSPORT` |
| A-10 attack plane | A10 | `AIRCRAFT_A10` |
| Apache attack helicopter | HELI | `AIRCRAFT_HELICOPTER` |
| Orca attack helicopter | ORCA | `AIRCRAFT_ORCA` |
| C-17 cargo plane | C17 | `AIRCRAFT_CARGO` |

The `AircraftTypeClass::Pointers` array (indexed by `AircraftType` enum) provides `O(1)` lookup from type enum to type class instance. The `From_Name` method uses this array to convert ASCII names to type IDs.

**Key callers**:
- `AircraftClass::Create_One_Of()` — creates an `AircraftClass` instance using `AircraftTypeClass::Create_One_Of()`
- `AircraftTypeClass::Who_Can_Build_Me()` — scans buildings to find one that can build a given aircraft type
- `AircraftTypeClass::One_Time()` — loads cameo and image SHPs for all aircraft types; also loads `LROTOR.SHP` and `RROTOR.SHP`
- `AircraftTypeClass::Init(theater)` — theater-specific cameo loading
- `AircraftTypeClass::From_Name()` — ASCII-to-type conversion for INI parsing
- `AircraftTypeClass::Display()` — scenario editor rendering
- `AircraftTypeClass::Repair_Cost()` / `Repair_Step()` — combat support calculations

## Classes, structs, enums and important data

### enum AircraftType (defined in defines.h / referenced here)

```cpp
typedef enum AircraftType : char {
    AIRCRAFT_TRANSPORT,    // Transport helicopter (Chinook)
    AIRCRAFT_A10,          // Ground attack plane (A-10)
    AIRCRAFT_HELICOPTER,   // Apache gunship (GDI)
    AIRCRAFT_CARGO,        // Cargo plane (C-17)
    AIRCRAFT_ORCA,         // Nod attack helicopter

    AIRCRAFT_COUNT,
    AIRCRAFT_NONE = -1,
    AIRCRAFT_FIRST = 0
} AircraftType;
```

**Bit flags for scanning** (used in `Who_Can_Build_Me`):
- `AIRCRAFTF_TRANSPORT` = (1L << AIRCRAFT_TRANSPORT)
- `AIRCRAFTF_A10` = (1L << AIRCRAFT_A10)
- `AIRCRAFTF_HELICOPTER` = (1L << AIRCRAFT_HELICOPTER)
- `AIRCRAFTF_CARGO` = (1L << AIRCRAFT_CARGO)
- `AIRCRAFTF_ORCA` = (1L << AIRCRAFT_ORCA)

### struct AircraftTypeClass

Core data class storing all configuration for one aircraft type. Key members (as observed in the constructor initializer list and aggregate initialization):

| Member | Type | Description |
|---|---|---|
| `Type` | `AircraftType` | The aircraft type enum value |
| `IsRotorEquipped` | `bool` | Whether the aircraft has a rotor |
| `IsRotorCustom` | `bool` | Whether custom rotor sets are used per facing |
| `IsLandable` | `bool` | Whether the aircraft can land on clear terrain |
| `IsFixedWing` | `bool` | Whether the aircraft is fixed-wing vs rotary-wing |
| `ROT` | `int` | Rotation index / facing index |
| `Mission` | `MissionType` | Default mission assignment |

The class also inherits from `TechnoTypeClass` which provides the following aircraft-relevant fields (passed through the constructor):

- `IniName` — identifier used in INI files and `From_Name` lookup
- `BuildLevel` — construction prerequisite level
- `StruktF` — building prerequisite flag (e.g. `STRUCTF_HELIPAD`, `STRUCTF_NONE`)
- `IsLeader` — whether this type is a leader unit
- `IsTwoShooter` — fires a pair of shots in quick succession
- `IsTransporter` — typical transport vehicle
- `Ammo` — default ammo count
- `Strength` — unit strength hit points
- `SightRange` — terrain revelation range
- `Cost` — credit cost to construct
- `Scenario` — scenario availability flag
- `Risk`, `Reward` — AI calculation parameters
- `Ownable` — ownership bitmask (multi-house flags)
- `PrimaryWeapon`, `SecondaryWeapon` — weapon type enums
- `Armor` — armor type enum
- `MaxSpeed` — `MPHType` maximum speed
- `IsStealthy`, `IsSelectable`, `IsLegalTarget`, `IsInsignificant`, `IsImmune`, `IsTheater`, `IsRepairable`, `IsBuildable`, `IsCrew` — boolean flags

### Global variables

| Variable | Type | Description |
|---|---|---|
| `AircraftTypeClass::LRotorData` | `void const *` | Pointer to left rotor SHM data (loaded from `LROTOR.SHP`) |
| `AircraftTypeClass::RRotorData` | `void const *` | Pointer to right rotor SHM data (loaded from `RROTOR.SHP`) |
| `AircraftTypeClass::Pointers[AIRCRAFT_COUNT]` | `AircraftTypeClass const * const[5]` | Lookup table: `[TransportHeli, AttackPlane, AttackHeli, CargoPlane, OrcaHeli]` |

### Pre-initialized aircraft type instances

All five aircraft types are defined as `static AircraftTypeClass const` at file scope, meaning they are constructed before `main()` and live for the entire program duration. Their order in the `Pointers` array is: `[TransportHeli, AttackPlane, AttackHeli, CargoPlane, OrcaHeli]`.

| Instance | AirType | IniName | BuildLevel | Prerequisite | Weapons | Armor | Speed | Special Flags |
|---|---|---|---|---|---|---|---|---|
| **TransportHeli** | `AIRCRAFT_TRANSPORT` | `"TRAN"` | 6 | `STRUCTF_HELIPAD` | `NONE/NONE` | `ALUMINUM` | `MPH_MEDIUM_FAST` | Transport, landable, crew, rotor-equipped |
| **AttackPlane** | `AIRCRAFT_A10` | `"A10"` | 99 | `STRUCTF_NONE` | `NAPALM/NONE` | `ALUMINUM` | `MPH_FAST` | Invisible on radar, selectable, targetable, crew |
| **AttackHeli** | `AIRCRAFT_HELICOPTER` | `"HELI"` | 6 | `STRUCTF_HELIPAD` | `CHAIN_GUN/NONE` | `STEEL` | `MPH_FAST` | Leader, two-shot, rotor-equipped |
| **CargoPlane** | `AIRCRAFT_CARGO` | `"C17"` | 99 | `STRUCTF_NONE` | `NONE/NONE` | `ALUMINUM` | `MPH_FAST` | Crew, transport, invisible on radar (selected) |
| **OrcaHeli** | `AIRCRAFT_ORCA` | `"ORCA"` | 6 | `STRUCTF_HELIPAD` | `DRAGON/NONE` | `STEEL` | `MPH_FAST` | Rotor-equipped, good/bad house flags |

### Constructor: `AircraftTypeClass::AircraftTypeClass(...)`

Initializes an `AircraftTypeClass` instance. First delegates to `TechnoTypeClass` constructor with all base parameters (name, INI name, level, prerequisite, leader, two-shooter, transporter, crushable, stealthy, selectable, legal target, insignificant, immune, theater, two-shooter false, repairable, buildable, crew, ammo, strength, maxspeed, sightrange, cost, scenario, risk, reward, ownable, primary, secondary, armor). Then initializes aircraft-specific fields:

- `IsRotorEquipped = is_rotorequipped;`
- `IsRotorCustom = is_rotorcustom;`
- `IsLandable = is_landable;`
- `IsFixedWing = is_fixedwing;`
- `Type = airtype;`
- `ROT = rot;`
- `Mission = deforder;`

## Functions and methods

### `AircraftTypeClass::AircraftTypeClass(...)` (constructor)

See the "Classes, structs, enums and important data" section for full parameter list and initialization behavior.

### `AircraftTypeClass::From_Name(char const *name) → AircraftType`

Converts an ASCII name to the matching `AircraftType` enum value. Iterates through `Pointers[classid]->IniName` using `stricmp` for case-insensitive comparison. Returns `AIRCRAFT_NONE` if no match is found.

- **Input**: `name` — Pointer to ASCII name to translate
- **Output**: `AircraftType` enum value (`AIRCRAFT_FIRST` through `AIRCRAFT_COUNT-1`) on match, or `AIRCRAFT_NONE`
- **Behavior**: Case-insensitive match against the `IniName` of each pre-initialized aircraft type

### `AircraftTypeClass::One_Time(void)`

Performs one-time initialization loading graphical assets for all aircraft types. Called once early in game startup.

- **Behavior**: For each aircraft type (index `AIRCRAFT_FIRST` to `AIRCRAFT_COUNT-1`):
  - Constructs a file path: if resolution factor is set, loads `{IniName}ICNH.SHP`; otherwise loads `{IniName}ICON.SHP` → assigns to `uclass.CameoData`
  - Always loads `{IniName}.SHP` → assigns to `uclass.ImageData`
- After the loop:
  - Loads `LROTOR.SHP` → assigns to `LRotorData`
  - Loads `RROTOR.SHP` → assigns to `RRotorData`

### `AircraftTypeClass::Create_One_Of(HouseClass * house) → ObjectClass *`

Creates an `AircraftClass` instance of the appropriate type owned by the given house.

- **Input**: `house` — The house owner of the aircraft
- **Output**: Pointer to newly created `AircraftClass`, or `NULL` if allocation fails
- **Behavior**: Delegates to `new AircraftClass(Type, house->Class->House)`

### `AircraftTypeClass::Prep_For_Add(void)` (scenario editor)

Builds a list of pointers to aircraft object types that can be added in the scenario editor.

- **Behavior**: Iterates through all aircraft types; if `Get_Image_Data()` is non-null, adds the type to the map's selection list.

### `AircraftTypeClass::Display(int x, int y, WindowNumberType window, HousesType house) const` (scenario editor)

Displays a generic version of the aircraft type in the scenario editor object selection dialog.

- **Input**: `x, y` — coordinates to draw at (centered); `window` — window to base coordinates upon; `house` — owner of this generic aircraft
- **Behavior**: Retrieves cameo or image data; draws using `CC_Draw_Shape` with centering and fading.

### `AircraftTypeClass::Occupy_List(bool placement) → short const *`

Returns the occupation list for landed aircraft. Always returns `{0, REFRESH_EOL}` — a single-cell offset list.

### `AircraftTypeClass::Overlap_List(void) → short const *`

Returns the overlap list for a landed aircraft. Returns 8 cell offsets forming a 3×3 grid plus diagonals: `{-(MAP_CELL_W-1), -MAP_CELL_W, -(MAP_CELL_W+1), -1, 1, (MAP_CELL_W-1), MAP_CELL_W, (MAP_CELL_W+1), REFRESH_EOL}`.

### `AircraftTypeClass::Who_Can_Build_Me(bool intheory, bool legal, HousesType house) → BuildingClass *`

Determines which building can build this aircraft type. Scans all buildings owned by the given house, checking:

- Building not in limbo
- Building not marked for deconstruction
- Ownership bitmask check: `(1L << building->ActLike) & Ownable`
- Legal build check: `!legal || building->House->Can_Build(Type, building->ActLike)`
- Building RTTI matches `RTTI_AIRCRAFTTYPE`

If `building->IsLeader` is true, returns immediately. Otherwise returns the first matching building found, or `NULL` if none.

- **Input**: `intheory` — skip busyness check; `legal` — perform prerequisite legality checks; `house` — desired owner
- **Output**: Pointer to building that can build this aircraft, or `NULL`

### `AircraftTypeClass::Repair_Cost() → int`

Returns the credit expense for every repair step. Computes `Fixed_To_Cardinal(Cost / (MaxStrength / REPAIR_STEP), REPAIR_PERCENT)`.

### `AircraftTypeClass::Repair_Step() → int`

Returns the number of health points recovered per repair step. Always returns `REPAIR_STEP` (constant).

### `AircraftTypeClass::Max_Pips() → int`

Returns the maximum number of passengers (pips) allowed for this aircraft.

- If `IsTransporter`: returns `Max_Passengers()` (inherited from `TechnoTypeClass`)
- Else if `Primary != WEAPON_NONE`: returns `5`
- Else: returns `0`

### `AircraftTypeClass::Create_And_Place(CELL, HousesType) const → bool`

Always returns `false`. Aircraft creation through the normal game system is prohibited; this is a no-op.

### `AircraftTypeClass::Init(TheaterType theater)` (theater-specific cameo loading)

Loads theater-specific cameo icons for each aircraft type. Only runs if `theater != LastTheater`.

- For each aircraft type:
  - Constructs path: `"%.4sICNH"` + `Theaters[theater].Suffix` (e.g., `"A100"` + theater suffix)
  - Retrieves from `MixFileClass`; if found, sets `uclass.CameoData` to the result

### `AircraftTypeClass::Dimensions(int &width, int &height) const`

Returns fixed graphic dimensions for all aircraft types: `width = 21`, `height = 20`.

## Inputs, outputs and side effects

**One_Time()**:
- **Side effect**: Loads SHM/SHP files from `MixFile` into global `CameoData` and `ImageData` pointers for all 5 aircraft types, plus `LRotorData` and `RRotorData`. This is a one-time initialization that goes to disk.

**Init(theater)**:
- **Side effect**: May overwrite `CameoData` for each aircraft type based on the current theater. Only runs when theater changes.

**From_Name()**:
- **Input/output**: Reads ASCII name, returns type enum — no state change.

**Who_Can_Build_Me()**:
- **Side effect**: Reads building list; does not modify building state. Returns a pointer; caller owns it.

**Create_One_Of()**:
- **Side effect**: Heap-allocates a new `AircraftClass` instance. Caller is responsible for lifecycle.

**Display()**:
- **Side effect**: Calls drawing functions; may render to screen in scenario editor context.

**Repair_Cost() / Repair_Step() / Max_Pips()**:
- **Output only**: Pure functions, no side effects.

**Create_And_Place()**:
- **Output only**: Always returns `false`; no allocation or placement occurs.

## Calls into / called by

### Called by:
- `AircraftClass::Create_One_Of()` — creates aircraft objects
- `AircraftTypeClass::Who_Can_Build_Me()` — finds buildable factories
- `AircraftTypeClass::One_Time()` — invoked during game init
- `AircraftTypeClass::Init()` — invoked when theater changes
- `AircraftTypeClass::From_Name()` — used by INI parser for name→type conversion
- `AircraftTypeClass::Display()` — scenario editor rendering
- `AircraftTypeClass::Repair_Cost()`, `Repair_Step()`, `Max_Pips()` — combat calculations
- `AircraftTypeClass::Create_And_Place()` — called (and always fails) during map loading

### Calls into:
- `TechnoTypeClass` constructor — base class initialization (delegation via initializer list)
- `MixFileClass::Retrieve()` — loads SHM/SHP graphical assets from archive
- `_makepath()` — constructs file paths for graphical assets
- `stricmp()` — case-insensitive string comparison
- `Greatest_Threat()`, `Good_Fire_Location()`, `Find_Docking_Bay()` — AI and movement helpers (indirectly, via `AircraftClass` methods)
- `RA AI tracking system` — registers/deregisters aircraft types

## Related original files

- `original/TIBERIANDAWN/AADATA.H` — header declaring `AircraftTypeClass`, enum, and function prototypes
- `original/TIBERIANDAWN/AIRCRAFT.CPP` — `AircraftClass` implementation, flight behavior, mission state machines
- `original/TIBERIANDAWN/AIRCRAFT.H` — `AircraftClass` declaration, mission functions, physics
- `original/TIBERIANDAWN/TECHNO.CPP` — `TechnoTypeClass` base, shared type infrastructure
- `original/TIBERIANDAWN/DEFINES.H` — `AircraftType` enum definition, `AIRCRAFTF_*` flags, `MAP_CELL_W`
- `original/TIBERIANDAWN/MIXFILE.CPP` — `MixFileClass::Retrieve` implementation for asset loading
- `original/TIBERIANDAWN/BUILDING.CPP` — `Who_Can_Build_Me` scans building objects from this file
- `original/TIBERIANDAWN/RTTI.H` — `RTTIType` enum includes `RTTI_AIRCRAFTTYPE`

## Rust port mapping

The `AircraftTypeClass` and its five instances are planned for translation into the Rust port. Mapping status:

- **AircraftType enum** → `aadata.rs` — Rust enum mirroring the 5 aircraft types + `Count`, `None`, `First`
- **AircraftTypeClass struct** → `aadata.rs` — Rust struct with same fields: `type`, `ini_name`, `build_level`, `prerequisite`, `is_leader`, `is_two_shooter`, `is_transporter`, `is_fixed_wing`, `is_rotor_equipped`, `is_rotor_custom`, `is_landable`, `is_crushable`, `is_stealthy`, `is_selectable`, `is_legal_target`, `is_insignificant`, `is_immune`, `is_theater`, `is_repairable`, `is_buildable`, `is_crew`, `ammo`, `strength`, `sightrange`, `cost`, `scenario`, `risk`, `reward`, `ownable`, `primary_weapon`, `secondary_weapon`, `armor`, `max_speed`, `rot`, `mission`
- **Five static instances** (`TransportHeli`, `AttackPlane`, `AttackHeli`, `CargoPlane`, `OrcaHeli`) → `aadata.rs` — constant instances initialized from the original C++ aggregate initializers
- **`AircraftTypeClass::Pointers`** → `aadata.rs` — static array `[AircraftTypeClass; 5]` for O(1) lookup
- **`AircraftTypeClass::From_Name()`** → `aadata.rs` — linear search against `Pointers` array with case-insensitive comparison
- **`AircraftTypeClass::One_Time()`** → `aadata.rs` — one-time asset loading; in Rust this would load from bundled resources instead of `MixFile`
- **`AircraftTypeClass::Init(theater)`** → `aadata.rs` — theater-specific cameo loading; maps `TheaterType` to suffix strings
- **`AircraftTypeClass::Who_Can_Build_Me()`** → `aadata.rs` — building scan logic; needs `Buildings` collection access
- **`AircraftTypeClass::Repair_Cost()` / `Repair_Step()` / `Max_Pips()`** → `aadata.rs` — math functions using existing Rust `Cost`, `MaxStrength`, `REPAIR_STEP`, `REPAIR_PERCENT`
- **Edge cases**: The `IsTransporter` / `IsFixedWing` branching in `Max_Pips()` and the `IsRotorCustom`/`IsRotorEquipped` flags need explicit mapping. The `theater != LastTheater` guard in `Init()` should use a Rust `last_theater` mutable state or lazy-init pattern.

**Missing / unimplemented in Rust port** (as of current inventory):
- Theater-specific cameo suffix mapping (depends on `Theaters[]` array and `Get_Resolution_Factor()` logic)
- `MixFileClass` asset loading replacement — Rust uses asset bundle paths or `include!`d resources
- `Who_Can_Build_Me` building RTTI check (`RTTI_AIRCRAFTTYPE`) — Rust equivalent of building type identification
- The `IsTwoShooter` / `IsLeader` flags propagation from `TechnoTypeClass` base

## Edge cases and compatibility notes

1. **`AIRCRAFT_COUNT` = 5** but the enum has `AIRCRAFT_FIRST = 0` through `AIRCRAFT_COUNT`, meaning valid indices are 0–4. The `Pointers` array has exactly 5 elements matching this range.

2. **`From_Name` is case-insensitive** via `stricmp`, but INI files typically use exact capitalization (e.g., `A10`, `HELI`, `ORCA`, `TRAN`, `C17`). The function handles any case variation.

3. **`One_Time` loads from disk** and must only be called once. In the Rust port, this must be guarded against re-initialization (e.g., via `std::sync::Once` or a `OnceCell`).

4. **`Pointers` array order** is significant: `[TransportHeli, AttackPlane, AttackHeli, CargoPlane, OrcaHeli]`. This order must be preserved in the Rust port, as `From_Name` and any iteration depends on it.

5. **Theater cameo loading** in `Init(theater)` uses `%.4sICNH` format + theater suffix. The suffix depends on the `TheaterType` and `Get_Resolution_Factor()` state. In Rust, this mapping needs to be reproduced from the theater data.

6. **`IsRotorCustom`** is `false` for all five aircraft types in the original code. If future variants add custom rotors, the Rust struct should have this field but default to `false`.

7. **`StruktF` values**: Two types (`AIRCRAFT_A10`, `AIRCRAFT_CARGO`) use `STRUCTF_NONE` (no prerequisite), while three types (`TransportHeli`, `AttackHeli`, `OrcaHeli`) use `STRUCTF_HELIPAD`. This affects buildability logic in the Rust port.

8. **`Ownable` bitmask**: All five types set multiple `HOUSEF_MULTI*` flags plus `HOUSEF_JP` and either `HOUSEF_GOOD` or `HOUSEF_BAD`. The Rust port must replicate this exact bitmask for ownership logic to work correctly.

## Evidence and review status

- **Review status**: **Reviewed** — full source file inspected; all 5 aircraft type instances, constructor, 11 public methods, and 2 global data items documented.
- **Evidence**: All behavior traced to original C++ source at `/home/hyperion/Projects/Games/CnC_Remastered_Collection/original/TIBERIANDAWN/AADATA.CPP` (f1f0d42 commit).
- **Key functions verified**: Constructor, `From_Name`, `One_Time`, `Create_One_Of`, `Prep_For_Add`, `Display`, `Occupy_List`, `Overlap_List`, `Who_Can_Build_Me`, `Repair_Cost`, `Repair_Step`, `Max_Pips`, `Create_And_Place`, `Init`, `Dimensions`.
- **Cross-referenced**: Symbols and behaviors validated against `AIRCRAFT.CPP`, `AIRCRAFT.H`, `TECHNO.CPP`, `DEFINES.H`, and `BUILDING.CPP` in the same repository.
- **Rust port readiness**: High — all symbols, data layouts, and control flow are documented and can be directly transcribed to Rust. Theater cameo logic and MixFile asset loading are the main translation gaps.