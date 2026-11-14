# PATH/FILE
original/TIBERIANDAWN/TARGET.CPP

## Purpose
Target conversion and resolution system for Tiberian Dawn. Provides functions to convert target numbers (TARGET type) into typed pointers (TechnoClass, Building, Unit, Aircraft, Bullet, Animation, Trigger, Team, etc.) with active-check validation. Core to the targeting system used by weapons, AI, and player commands.

## Where it fits in the game
Targeting infrastructure - every weapon, AI decision, and player command needs to resolve a target number into a usable pointer. Called from `WEAPON.CPP` (fire at target), `LOGIC.CPP` (AI targeting), `SIDEBAR.CPP` (selection), and `OPTIONS.CPP` (dialog). The `Target_Legal()` function validates targets before use. Global arrays (Infantry, Units, Buildings, etc.) are indexed by target values.

## Classes, structs, enums and important data

### TARGET type
**Target number** - Encodes both kind and value information.

**Target kind constants** (from `Target_Kind()`):
| Kind | Value range | Represents |
|------|-------------|------------|
| `KIND_INFANTRY` | infantry indices | Infantry units |
| `KIND_UNIT` | unit indices | Vehicles, buildings, other units |
| `KIND_BUILDING` | building indices | Command center, factories, etc. |
| `KIND_AIRCRAFT` | aircraft indices | Flying objects |
| `KIND_CELL` | cell indices | Map cell coordinates |
| `KIND_TRIGGER` | trigger indices | Triggers and events |
| `KIND_TEAM` | team indices | Player teams |
| `KIND_TEAMTYPE` | team type indices | AI team types |

### Target encoding
- Target value packs kind + index
- `Target_Kind(target)` → kind enum
- `Target_Value(target)` → raw index into corresponding array
- `Target_Legal(target)` → validates kind + index bounds

### Conversion functions (all follow same pattern)

**As_Object(target, check_active)** → `ObjectClass*`
- Decodes target kind
- Fetches pointer from `Infantry.Raw_Ptr()`, `Units.Raw_Ptr()`, or `Buildings.Raw_Ptr()`
- If `check_active`: validates pointer is not NULL and `IsActive`
- Returns NULL if invalid

**As_Techno(target, check_active)** → `TechnoClass*`
- Calls `As_Object(target, check_active)`
- If object is TechnoClass subclass: returns casted pointer
- Otherwise: returns NULL

**As_Aircraft(target, check_active)** → `AircraftClass*`
- Fetches from `Aircraft.Raw_Ptr(index)`
- Active check if requested

**As_Unit(target, check_active)** → `TechnoClass*` (infantry/vehicle)
- Fetches from `Infantry.Raw_Ptr()` or `Units.Raw_Ptr()`
- Active check if requested

**As_Building(target, check_active)** → `BuildingClass*`
- Fetches from `Buildings.Raw_Ptr(index)`
- Active check if requested

**As_Bullet(target, check_active)** → `BulletClass*`
- Fetches from `Bullets.Raw_Ptr(index)`
- Active check if requested

**As_Animation(target, check_active)** → `AnimClass*`
- Fetches from `Anims.Raw_Ptr(index)`
- Active check if requested

**As_Trigger(target, check_active)** → `TriggerClass*`
- Fetches from `Triggers.Raw_Ptr(index)`
- Active check: if not active → NULL

**As_Team(target, check_active)** → `TeamClass*`
- Fetches from `Teams.Raw_Ptr(index)`
- Active check: if not active → NULL

**As_TeamType(target)** → `TeamTypeClass*`
- No active check (team types are static)
- Fetches from `TeamTypes.Raw_Ptr(index)`

**As_Coord(target)** → `COORDINATE`
- If cell target: returns `Cell_Coord(cell_index)`
- Otherwise: fetches object via `As_Object(target)` and returns `obj->Target_Coord()`
- Returns 0x00000000L if invalid

**As_Movement_Coord(target)** → `COORDINATE`
- If cell target: returns `Cell_Coord(cell_index)`
- Otherwise: fetches object via `As_Object(target)` and returns `obj->Docking_Coord()`
- Returns 0x00000000L if invalid (for docking buildings)

**Target_Legal(target)** → `bool`
- Validates target kind is recognized
- Validates index within array bounds
- Used before all target conversions as safety check

## Functions and methods

### Target conversion functions (pattern)

All conversion functions follow this pattern:
1. Check target kind via `Target_Kind(target)`
2. Fetch raw pointer from appropriate global array
3. If `check_active` parameter:
   - Verify pointer is not NULL
   - Verify `IsActive` flag is set
   - Return NULL if inactive
4. Return typed pointer

### Key individual functions

**As_Object(target, check_active)** - Root converter
- Switch on `Target_Kind()`:
  - `KIND_INFANTRY` → `Infantry.Raw_Ptr(val)`
  - `KIND_UNIT` → `Units.Raw_Ptr(val)`
  - `KIND_BUILDING` → `Buildings.Raw_Ptr(val)`
- Active validation if requested

**As_Aircraft(target, check_active)** - Aircraft converter
- `Aircraft.Raw_Ptr(Target_Value(target))`
- Active check: `aircraft->IsActive`

**As_Trigger(target, check_active)** - Trigger converter
- `Triggers.Raw_Ptr(Target_Value(target))`
- Active check: `trigger->IsActive` → if false, return NULL

**As_Team(target, check_active)** - Team converter
- `Teams.Raw_Ptr(Target_Value(target))`
- Active check: `team->IsActive` → if false, return NULL

**As_TeamType(target)** - Team type (no active check)
- `TeamTypes.Raw_Ptr(Target_Value(target))`
- Always returns pointer (team types are static definitions)

**As_Coord(target)** - Coordinate resolver
- Special case: `Is_Target_Cell(target)` → `Cell_Coord((CELL)Target_Value(target))`
- Normal case: `As_Object(target)` → `obj->Target_Coord()`
- Error: invalid ptr or dead object → return 0

**As_Movement_Coord(target)** - Movement coordinate
- Special case: cell → `Cell_Coord(cell_index)`
- Normal case: object → `obj->Docking_Coord()`
- Used for moving to targets (including docking buildings)

**Target_Legal(target)** - Validation
- Checks kind is recognized
- Checks index within bounds
- Prerequisite before all target conversions

## Inputs, outputs and side effects

**State mutations:**
- None directly (pure conversion functions)
- May read `IsActive` flags on target objects
- May read global array state (Infantry, Units, Buildings, etc.)

**External systems affected:**
- `WEAPON.CPP` → all weapon targeting resolutions
- `LOGIC.CPP` → AI target resolution and AI decision making
- `SIDEBAR.CPP` → object selection from player clicks
- `OPTIONS.CPP` → target selection in dialogs
- `CRATE.CPP` → crate content target resolution
- Multiplayer: target synchronization across network

## Calls into / called by

**Called by:**
- `WEAPON.CPP` → `As_Aircraft()`, `As_Unit()`, `As_Techno()` for fire-at-target
- `LOGIC.CPP` → `Target_Legal()` before AI target decisions
- `SIDEBAR.CPP` → `As_Object()`, `As_Techno()` for selection display
- `OPTIONS.CPP` → target conversion for dialog interactions
- `CRATE.CPP` → crate item target resolution
- `SPECIAL.CPP` → super weapon target resolution

**Calls into:**
- `Target_Kind()` → decode target kind enum
- `Target_Value()` → extract raw index
- `Is_Target_*()` → kind-specific validation macros
- `Infantry.Raw_Ptr()`, `Units.Raw_Ptr()`, `Buildings.Raw_Ptr()`, `Aircraft.Raw_Ptr()`, `Bullets.Raw_Ptr()`, `Anims.Raw_Ptr()`, `Triggers.Raw_Ptr()`, `Teams.Raw_Ptr()`, `TeamTypes.Raw_Ptr()` → array lookups
- `ObjectClass::Target_Coord()` → coordinate extraction
- `ObjectClass::Docking_Coord()` → docking coordinate
- `IsBadReadPtr()` → memory safety check (in `As_Coord`)

## Related original files
- **target.H** - Header with function declarations and TARGET type definition
- **function.H** - Included utility headers
- **WEAPON.CPP** → all weapon targeting calls
- **LOGIC.CPP** → AI target resolution
- **SIDEBAR.CPP** → player object selection
- **OPTIONS.CPP** → target selection dialog
- **CRATE.CPP** → crate item targeting
- **Infantry.CPP**, **Unit.CPP**, **Building.CPP** → underlying arrays

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/target/resolution.rs` - Target conversion functions
- `cnc-tiberian-dawn/src/target/legal.rs` - Target_Legal validation
- `cnc-tiberian-dawn/src/target/as_object.rs` - As_Object and derivatives

**Not yet ported:**
- TARGET type encoding/decoding logic
- Global array lookups (Infantry, Units, Buildings, etc.)
- Active flag validation
- Cell coordinate conversion
- Docking coordinate extraction
- Team type access (no active check)
- Multiplayer target synchronization
- Memory safety checks (IsBadReadPtr equivalent)

## Edge cases and compatibility notes

1. **Kind encoding**: Target number packs both kind and index; must decode correctly
2. **Active validation**: `check_active=true` filters out deleted/inactive objects
3. **Cell targets**: Special handling for map cell targets (used for area-effect targeting)
4. **Docking coordinates**: `As_Movement_Coord` uses `Docking_Coord` for buildings with docking bays
5. **Team types**: `As_TeamType` has no active check - team types are static definitions
6. **Bullet targets**: Rarely used but available for weapon-target interactions
7. **Animation targets**: For effect playback, not logical targeting
8. **Invalid targets**: All functions return NULL/0 for invalid/out-of-range targets
9. **Memory safety**: `As_Coord` uses `IsBadReadPtr` as defense against corrupted target values
10. **Multiplayer sync**: Target values must remain consistent across networked game instances

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TARGET.CPP at f1f0d42
