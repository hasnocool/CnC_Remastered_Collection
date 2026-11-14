# original/TIBERIANDAWN/UDATA.CPP

## Purpose
UDATA.CPP defines the unit type data structures and associated functions for Tiberian Dawn. It contains the `UnitTypeClass` definition and all unit type instantiations, representing every unit in the game (vehicles, special units, and infantry-like entities). The file provides the complete unit database used for unit creation, placement, behavior, and editor integration. A pointer array `UnitTypeClass::Pointers[UNIT_COUNT]` maps unit type enums to their static data, enabling runtime lookups by name or class ID.

## Where it fits in the game
Unit data subsystem - UDATA.CPP is the central repository for all unit type metadata. Every unit in the game is represented by a `UnitTypeClass` static constant defined in this file. The `Pointers` array enables O(1) lookup from unit type enums to their data. Unit data drives:
- Unit creation and placement (via `Create_And_Place`, `Create_One_Of`)
- Building prerequisites and buildability checks (via `Who_Can_Build_Me`)
- Rendering and selection dimensions (via `Dimensions`, `Display`)
- Repair cost and step calculations (via `Repair_Cost`, `Repair_Step`)
- Occupation list for map placement (via `Occupy_List`)
- Scenario editor unit addition (via `Prep_For_Add`)
- One-time initialization loading shapes and brain files (via `One_Time`)

## Classes, structs, enums and important data

### UnitTypeClass
**Unit type definition** - Transient object representing a unique unit type in the game. Instantiated as static constants in UDATA.CPP and referenced through the `Pointers` array.

**Key data members (set via constructor, matching `UnitTypeClass::UnitTypeClass` parameters):**
- `Explosion` (AnimType) - Explosion animation type when destroyed
- `IsCrateGoodie` (bool) - Can this be a goodie surprise from a crate?
- `IsPieceOfEight` (bool) - Only has eight facings?
- `IsCloakable` (bool) - Unit capable of cloaking
- `IsChunkyShape` (bool) - Unit art as "chunky" cardinal facing only
- `IsCrusher` (bool) - Can this unit squash infantry?
- `IsFireAnim` (bool) - Associated firing animation exists
- `IsGigundo` (bool) - Gigundo-rotund-enormous unit
- `IsLockTurret` (bool) - Turret must be locked down while moving
- `IsRadarEquipped` (bool) - Has rotating radar dish
- `IsToHarvest` (bool) - Harvests Tiberium
- `IsTracked` (bool) - Tracked locomotion
- `IsAnimating` (bool) - Capable of constant animation
- `Mission` (MissionType) - Default order for new unit
- `ROT` (unsigned) - Rate of turn (degrees per tick)
- `Speed` (SpeedType) - Locomotion type
- `TurretOffset` (int) - Turret center offset along body centerline
- `Type` (UnitType) - Unit type enum identifier

**Per-unit data fields (passed to constructor):**
- `Type` - UnitType enum value (UNIT_FIRST to UNIT_COUNT-1)
- `name` - Text identifier (used for INI lookup)
- `ininame` - INI name string (e.g. "VICE", "FTNK", "HARV")
- `exp` - Explosion animation type
- `level` - Build level
- `pre` - Building prerequisite (STRUCTF_*)
- `is_goodie` - Crate goodie flag
- `is_leader` - Leader type flag
- `is_eight` - Eight facings flag
- `is_nominal` - Nominal flag
- `is_transporter` - Typical transport vehicle flag
- `is_crushable` - Can be crushed by heavy vehicle
- `is_crusher` - Can crush infantry
- `is_harvest` - Harvests Tiberium
- `is_stealthy` - Stealth capable
- `is_selectable` - Selectable by player
- `is_legal_target` - Can be targeted in attack/move
- `is_insignificant` - Won't be announced
- `is_immune` - Immune to normal combat damage
- `is_turret_equipped` - Has combat turret
- `is_twoshooter` - Fires multiple shots in quick succession
- `is_repairable` - Can be repaired in repair facility
- `is_buildable` - Player can construct or order this unit
- `is_crew` - Has crew inside
- `is_radar_equipped` - Rotating radar dish
- `is_fire_anim` - Associated firing animation
- `is_lock_turret` - Turret locked while moving
- `is_tracked` - Tracked movement
- `is_gigundo` - Gigundo-rotund-enormous unit
- `is_chunky` - Chunky cardinal facing only art
- `is_animating` - Constant animation
- `ammo` - Number of shots (default -1 = unlimited)
- `strength` - Strength/damage points
- `sightrange` - Sighting range
- `cost` - Build cost in credits
- `scenario` - Starting availability scenario
- `risk` / `reward` - Risk/reward rating values
- `ownable` - Ownable by house (bit field of HOUSEF_* flags)
- `primary`, `secondary` - WeaponType primary/secondary
- `armor` - ArmorType armor kind
- `speed` - SpeedType locomotion type
- `maxSpeed` - MPHType maximum speed (Miles per hour)
- `rot` - Rate of turn (degrees per tick)
- `toffset` - Turret offset along body centerline
- `order` - MissionType default order

### UnitType enum (partial, as reflected in Pointers array)
| Enum Value | Unit Type |
|---|---|
| UNIT_HTANK | Mastodon tank |
| UNIT_MTANK | Medium tank |
| UNIT_LTANK | Light tank |
| UNIT_STANK | Stealth tank |
| UNIT_FTANK | Flame tank |
| UNIT_VICE | Visceroid |
| UNIT_APC | Armored personnel carrier |
| UNIT_MLRS | MLRS rocket launcher |
| UNIT_JEEP | Jeep (hummer) |
| UNIT_BUGGY | Buggy |
| UNIT_HARVESTER | Harvester |
| UNIT_ARTY | Artillery |
| UNIT_MSAM | Mobile SAM launcher |
| UNIT_HOVER | Landing craft |
| UNIT_MHQ | Mobile HQ |
| UNIT_GUNBOAT | Gunboat |
| UNIT_TRIC | Triceratops |
| UNIT_TREX | T-Rex |
| UNIT_RAPT | Velociraptor |
| UNIT_STEG | Stegosaurus |
| UNIT_BIKE | Attack cycle |
| #ifdef PETROGLYPH_EXAMPLE_MOD | UNIT_NUKE_TANK | Nuclear tank |

### Global pointer array
```cpp
UnitTypeClass const * const UnitTypeClass::Pointers[UNIT_COUNT] = {
    &UnitHTank, &UnitMTank, &UnitLTank, &UnitSTank,
    &UnitFTank, &UnitVice, &UnitAPC, &UnitMLRS,
    &UnitJeep, &UnitBuggy, &UnitHarvester, &UnitArty,
    &UnitSAM, &UnitHover, &UnitMHQ, &UnitGunBoat,
    &UnitMCV, &UnitBike, &UnitTric, &UnitTrex,
    &UnitRapt, &UnitSteg
    #ifdef PETROGLYPH_EXAMPLE_MOD
    , &UnitNukeTank
    #endif
};
```

## Functions and methods

### UnitTypeClass::UnitTypeClass -- Constructor for unit types
Initializes the unit type class structure. Called for every unique unit type as it exists in the `Pointers` array. Initializes fields inherited from `TechnoTypeClass` plus unit-specific flags and values.

### UnitTypeClass::Occupy_List -- Returns with unit occupation list
Returns a pointer to the unit occupation list used for placing units on the map.
- For gunboats: returns `{0, -1, 1, REFRESH_EOL}` (occupies 3 cells)
- For all other units: returns `{0, REFRESH_EOL}` (occupies 1 cell)

### UnitTypeClass::From_Name -- Fetch class pointer from specified name
Converts an ASCII representation of a unit class into a real unit class number. Iterates through `Pointers[classid]->IniName` and compares with `stricmp`. Returns `UNIT_NONE` if not found.

### UnitTypeClass::Display -- Displays a generic unit shape
Displays a generic representation of a unit of this type. Typically used when adding objects to the game map. Draws either cameo data or image data with appropriate shape rendering.

### UnitTypeClass::Prep_For_Add -- Prepares scenario editor to add unit
Scans all unit types and adds those with image data to the map editing interface list.

### UnitTypeClass::One_Time -- Performs one time processing for unit type class objects
Loads unit shapes and brain files. Should only be called once. For buildable units, retrieves `.ICNH` or `.ICON` shader files from Mix database. Also loads build frame dimensions and wake shape (`WAKE.SHP`).

### UnitTypeClass::Init -- Fetches the sidebar icons for the unittypeclass objects
Resets cameo data for all unit types. If resolution factor is set, loads theater-specific cameo icons (`<IniName>ICNH`<TheaterSuffix>` from Mix files.

### UnitTypeClass::Create_And_Place -- Creates and places a unit object onto the map
Creates a new `UnitClass` and places it unlimboed at the specified cell coordinate with a random direction.

### UnitTypeClass::Create_One_Of -- Creates a unit in limbo
Creates a `UnitClass` object and keeps it in limbo. Returns pointer to the created unit. The caller is responsible for later unlimboing at the correct time and place.

### UnitTypeClass::Who_Can_Build_Me -- Determines which factory can build this unit type
Examines all buildings on the map to determine which factory can produce the unit type. Checks building activity, mission status, ownership, prerequisites, and radio contact. Returns the leader building if found, otherwise the first suitable building.

### UnitTypeClass::As_Reference -- Fetches a reference to the unit type class specified
Returns a reference to the `UnitTypeClass` object as indicated by the unit type number. Simple index into the `Pointers` array.

### UnitTypeClass::Dimensions -- Determines the unit's pixel dimensions
Fills in width and height for this unit type, used for selection rectangle and health bar positioning.
- For gunboats: width=46, height=18 (special dimensions)
- For all other units: width = height = MaxSize - MaxSize/4

### UnitTypeClass::Repair_Cost -- Determines cost per "step" of repair
Returns the credit cost to repair this unit one step. Computes `Fixed_To_Cardinal(Cost / (MaxStrength / REPAIR_STEP), REPAIR_PERCENT)`.

### UnitTypeClass::Repair_Step -- Determines the repair step rate
Returns the number of strength points healed for each repair step. Constant `REPAIR_STEP`.

### UnitTypeClass::Max_Pips -- Fetches the maximum pips allowed for this unit
Returns maximum pips allowed for this unit type.
- For harvesters: `FULL_LOAD_CREDITS / 100` (credits held divided by 100)
- For transporters: `Max_Passengers()`
- For all other units: 0

## Inputs, outputs and side effects

### State mutations
- Unit type data is read-only at runtime; static constants are initialized at program start
- `One_Time` loads shape data from Mix files into `UnitTypeClass::CameoData` and `ImageData` pointers
- `Init` resets cameo pointers for all unit types
- `From_Name` is pure lookup, no side effects
- `Who_Can_Build_Me` reads building state but does not modify it

### External systems affected
- **Map rendering**: Unit dimensions drive selection rectangles and health bar positioning
- **Unit creation**: `Create_And_Place` and `Create_One_Of` produce `UnitClass` objects
- **Building AI**: `Who_Can_Build_Me` drives factory availability logic
- **Scenario editor**: `Prep_For_Add` and `Display` enable unit placement in editor
- **Repair system**: `Repair_Cost` and `Repair_Step` compute repair costs/steps
- **Occupation system**: `Occupy_List` determines map cell occupancy during placement
- **Sidebar**: `Init` loads sidebar icon bitmaps

### Calls into / called by

**Called by:**
- Scenario load/initialization → `One_Time`, `Init`
- Scenario editor → `Prep_For_Add`, `Display`, `Create_And_Place`
- Unit production logic → `Who_Can_Build_Me`, `Create_One_Of`
- Map placement → `Occupy_List`, `Create_And_Place`
- Repair subsystem → `Repair_Cost`, `Repair_Step`
- Name lookup → `From_Name`

**Calls into:**
- `TemplateTypeClass::As_Reference / From_Name` (in `Init`)
- `MixFileClass::Retrieve` (in `One_Time`, `Init`) - loads `.ICNH`, `.ICON`, and `.SHP` files
- `Map` cell access (`TType`, `TIcon`, `Redraw_Objects`, `Recalc_Attributes`)
- `ObjectClass::Mark` (indirectly via editor operations)
- `Templates` pool (for template system, not unit data directly)
- `Get_Build_Frame_Width`, `Get_Build_Frame_Height` (in `One_Time`)
- `Fixed_To_Cardinal`, `REPAIR_STEP`, `REPAIR_PERCENT` (in repair functions)
- `FULL_LOAD_CREDITS`, `Max_Passengers()` (in `Max_Pips`)

## Related original files
- **UDATA.H** - Class declarations, UnitTypeClass header, UnitType enum definitions
- **UNIT.H/.CPP** - UnitClass implementation, unit creation and management
- **TECHNO.H/.CPP** - TechnoTypeClass base class (parent of UnitTypeClass)
- **MIXFILE.H/.CPP** - File retrieval for shape/cameo assets
- **BUILDING.H/.CPP** - BuildingClass, prerequisite and buildability checks
- **MAP.H/.CPP** - MapClass, CellClass, occupation logic
- **INI.H/.CPP** - INI name lookups and scenario availability
- **FUNCTION.H** - Global constants and function declarations
- **HEAP.H** - Memory pool infrastructure (indirectly via unit allocation)
- **RULES.H** - Repair constants (`REPAIR_STEP`, `REPAIR_PERCENT`)
- **HOUSE.H** - HouseClass, ownership and buildability checks

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-map-editor`

**Mapped modules:**
- `cnc-core/src/unit_type.rs` - UnitTypeClass, UnitType enum, Pointers array
- `cnc-tiberian-dawn/src/unit.rs` - Unit type definitions and data
- `cnc-core/src/repair.rs` - Repair cost and step calculations
- `cnc-core/src/occupy.rs` - Unit occupation lists for map placement
- `cnc-map-editor/src/unit_tool.rs` - Scenario editor unit addition logic

**Not yet ported:**
- Icon set map stamping (per-cell icon indices from `.ICNH`/`.ICON` files)
- Theater-specific cameo loading in `Init()`
- Shape and brain file loading via `MixFileClass::Retrieve` in `One_Time`
- Wake shape (`WAKE.SHR`) retrieval
- Build frame dimension calculation
- `Who_Can_Build_Me` building traversal logic
- `From_Name` ASCII-to-enum name resolution
- `Display` generic shape rendering
- `Create_And_Place` / `Create_One_Of` unit object construction
- Repair cost/step computation with `Fixed_To_Cardinal`
- `Max_Pips` harvester/transporter logic
- Occupation list differentiation (gunboat vs simple)
- PETROGLYPH_EXAMPLE_MOD nuclear tank conditional compilation
- Scenario editor unit list population

## Edge cases and compatibility notes

1. **Gunboat special dimensions**: `UnitTypeClass::Dimensions` returns width=46, height=18 for gunboats, unlike the standard `MaxSize - MaxSize/4` formula
2. **PETROGLYPH_EXAMPLE_MOD**: Conditional compilation for `UnitNukeTank`; absent in default build
3. **Infinite ammo default**: `ammo = -1` signifies unlimited shots across most unit types
4. **Harvester pips**: `Max_Pips` returns `FULL_LOAD_CREDITS/100` for harvesters, representing credits carried divided by 100
5. **Transporter pips**: `Max_Pips` returns `Max_Passengers()` for transporter-type units
6. **Two-shooter units**: Flag `is_twoshooter` enables firing multiple shots in quick succession; affects weapons like `WEAPON_FLAME_TONGUE`
7. **Immune to normal combat**: `is_immune` flag prevents damage from standard small-arms; requires specialized weapons
8. **Eight facings**: `is_eight` controls whether unit has 8 directional facings or fewer
9. **Chunky art**: `is_chunky` indicates unit art is "chunky" cardinal-facing only (fewer animation frames)
10. **Constant animation**: `is_animating` indicates unit has a constant/looping animation vs. one-shot
11. **Eight facings vs. continuous rotation**: `is_eight` and `ROT` together define turning behavior; `MPH_MEDIUM` vs `MPH_MEDIUM_FAST` etc. modulate speed
12. **Build prerequisites**: `STRUCTF_*` flags determine which buildings must exist before unit can be produced; `STRUCTF_NONE` means no prerequisite
13. **Owner bit field**: `ownable` uses `HOUSEF_*` flags (MULTI1-6, JP, GOOD, BAD, NEUTRAL) to control which houses can own the unit
14. **Scenario availability**: `scenario` field controls starting availability in scenario editor; 99 = unavailable, lower numbers = earlier availability
15. **Risk/reward values**: `risk` and `reward` rating pairs (e.g., 80,20) used in AI and point-value calculations
16. **Turret offset**: `toffset` shifts the turret position along the body centerline; non-zero only for gunboat (value 14)
17. **Default mission orders**: `MISSION_HUNT` vs `MISSION_GUARD` vs `MISSION_GUARD` determines initial AI behavior
18. **Stealth flag**: `is_stealthy` combined with radar status determines visibility; stealth units detectable by radar
19. **Crate goodie**: `is_goodie` unit can randomly appear from crates during gameplay
20. **Leader type**: `is_leader` marks unit as a leader; leader buildings are preferred returns from `Who_Can_Build_Me`

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/UDATA.CPP at f1f0d42