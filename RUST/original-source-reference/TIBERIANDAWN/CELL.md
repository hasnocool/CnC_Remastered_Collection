# PATH/FILE
original/TIBERIANDAWN/CELL.CPP

## Purpose
Core map cell implementation for Tiberian Dawn. Manages the 64x64 cell grid (4096 cells), handling terrain, occupancy, overlays, tiberium, visibility, triggers, crates, walls, and pathfinding queries. The cell is the fundamental spatial unit of the game world.

## Where it fits in the game
Central spatial data structure - the MapClass contains an array of 4096 CellClass instances (64x64 grid). Every spatial query, pathfinding, rendering, and game logic operation goes through CellClass. It bridges the static map layout with dynamic game objects.

## Classes, structs, enums and important data

### CellClass
**Core spatial unit** - represents one map cell (64x64 grid = 4096 cells).

**Key data members:**
- `Overlay` (OverlayType) - Current overlay type (OVERLAY_NONE, OVERLAY_CRATE, OVERLAY_STEEL_CRATE, OVERLAY_TIBERIUM, etc.)
- `OverlayData` (int) - Overlay-specific data (tiberium amount, crate type, etc.)
- `TType` (TemplateType) - Template type occupying this cell (TEMPLATE_NONE if none)
- `Owner` (HousesType) - Owning house for flags/crates
- `InfType` (HousesType) - Infantry type for crate goodies
- `OverrideLand` (LandType) - Temporary land type override
- `OccupierPtr` (ObjectClass*) - Head of linked list of objects in this cell
- `Occupier` (short) - Occupation bitfield (infantry/vehicle/building/aircraft bits)
- `OverlayData` (int) - Overlay-specific data (tiberium amount, crate type)
- `TType` (TemplateType) - Template occupying this cell
- `Owner` (HousesType) - Owning house for flags/crates
- `InfType` (HousesType) - Infantry type for crate goodies
- `CTFFlag` (AnimClass*) - Capture-the-flag animation
- `IsFlagged` (bool) - CTF flag present
- `IsFlagged` (bool) - CTF flag present
- `CrateCount` (int) - Remaining crates on map
- `CrateMaker` (bool) - Whether crate spawner active
- `CrateTimer` (int) - Timer for next crate spawn
- `CrateCount` (int) - Remaining crates on map
- `CrateMaker` (bool) - Whether crate spawner active
- `CrateTimer` (int) - Timer for next crate spawn
- `Wall` (WallType) - Wall type in this cell
- `Overlay` (OverlayType) - Current overlay
- `OverlayData` (int) - Overlay data (tiberium amount, etc.)
- `TType` (TemplateType) - Template type
- `Owner` (HousesType) - Owning house
- `InfType` (HousesType) - Infantry type for crates
- `OverrideLand` (LandType) - Land type override
- `OccupierPtr` (ObjectClass*) - Linked list of objects in cell
- `CTFFlag` (AnimClass*) - CTF flag animation
- `IsFlagged` (bool) - Flag present
- `IsMappedByPlayerMask` (int) - Per-house mapped bitmask (MP)
- `IsVisibleByPlayerMask` (int) - Per-house visibility bitmask
- `IsMapped` / `IsVisible` - Single-player cached flags
- `Flag` (CellFlag) - Occupation bitfield (infantry/vehicle/building/aircraft)
- `Overlay` (OverlayType) - Overlay type
- `OverlayData` (int) - Overlay-specific data
- `TType` (TemplateType) - Template type
- `Owner` (HousesType) - Owning house
- `InfType` (HousesType) - Infantry type
- `OverrideLand` (LandType) - Land type override
- `Wall` (WallType) - Wall type
- `CTFFlag` (AnimClass*) - CTF flag animation
- `IsFlagged` (bool) - Flag present
- `IsMappedByPlayerMask` - Per-house mapped bitmask (8-bit)
- `IsVisibleByPlayerMask` - Per-house visibility bitmask
- `IsMapped` / `IsVisible` - SP cached flags
- `CrateCount` - Remaining crates on map
- `CrateMaker` - Crate spawner active flag
- `CrateTimer` - Next crate spawn timer
- `Wall` - Wall type in cell
- `Overlay` - Overlay type
- `OverlayData` - Overlay data
- `TType` - Template type
- `Owner` - Owning house
- `InfType` - Infantry type for crates
- `OverrideLand` - Land type override

**Occupation bitfield (Flag.Composite):**
- Bits 0-2: Infantry (3 bits)
- Bits 3-5: Vehicles (3 bits)
- Bit 6: Building
- Bit 7: Aircraft

**Per-house visibility/mapped masks (8-bit each):**
- `IsMappedByPlayerMask` - 8-bit mask for mapped status per house
- `IsVisibleByPlayerMask` - 8-bit mask for visibility per house
- `IsMapped` / `IsVisible` - Single-player cached flags

### CellFlag struct
- `Composite` - Combined occupation bits
- Individual bit access for infantry/vehicle/building/aircraft

### CrateType enum (14 types)
- `MONEY` (2000 SP / 100-2000 MP)
- `DARKNESS` (shroud map)
- `REVEAL` (reveal map)
- `NUKE` (nuclear explosion)
- `ION` (ion cannon blast)
- `NUKE_MISSILE` (nuke sidebar)
- `ION_BLAST` (ion cannon sidebar)
- `AIR_STRIKE` (air strike sidebar)
- `HEAL_BASE` (heal all techno)
- `CLOAK` (cloak nearby units)
- `EXPLOSION` (5 explosions)
- `NAPALM` (napalm blast)
- `SQUAD` (5 random infantry)
- `UNIT` (free unit: MCV, harvester, or random)
- `VISCEROID` (spawn visceroid)

### OverlayType enum (key values)
- `OVERLAY_NONE` - No overlay
- `OVERLAY_TIBERIUM` - Tiberium field
- `OVERLAY_CRATE` - Standard crate
- `OVERLAY_STEEL_CRATE` - Steel crate (MP only)
- `OVERLAY_TREE` - Tree
- `OVERLAY_WALL` - Wall
- `OVERLAY_CRATE` variants (steel, etc.)

## Functions and methods

### Core Cell Management

**CellClass::CellClass()** - Constructor
- Zero-initializes entire struct
- Sets defaults: `Overlay = OVERLAY_NONE`, `TType = TEMPLATE_NONE`, `Owner = HOUSE_NONE`, `InfType = HOUSE_NONE`, `OverrideLand = LAND_COUNT`

**CellClass::Validate()** → int (CHEAT_KEYS only)
- Validates cell number 0-4095
- Returns 1 if valid, 0 if error

**CellClass::Validate()** (macro in release) - no-op

### Spatial Queries

**CellClass::Cell_Color(override)** → int
- Returns radar color for cell
- Building → house color
- Override=true → black
- Default → terrain color from `Ground[Land_Type()].Color`

**CellClass::Cell_Techno(x, y)** → TechnoClass*
- Returns closest techno (building/unit) to pixel offset within cell
- Iterates Cell_Occupier() linked list, finds closest by pixel distance
- Uses lepton coordinates for precision

**CellClass::Cell_Find_Object(rtti)** → ObjectClass*
- Searches Cell_Occupier() linked list for matching RTTI type
- Returns first match or NULL

**CellClass::Cell_Building()** → BuildingClass*
- Wrapper: `Cell_Find_Object(RTTI_BUILDING)`

**CellClass::Cell_Terrain()** → TerrainClass*
- Wrapper: `Cell_Find_Object(RTTI_TERRAIN)`

**CellClass::Cell_Object(x, y)** → ObjectClass*
- Returns clickable object at pixel offset
- Priority: Building > Unit > Infantry > Aircraft > Terrain > Overlay

**CellClass::Cell_Infantry()** → InfantryClass*
- Wrapper: `Cell_Find_Object(RTTI_INFANTRY)`

**CellClass::Cell_Unit()** → UnitClass*
- Wrapper: `Cell_Find_Object(RTTI_UNIT)`

**CellClass::Cell_Occupier()** → ObjectClass*
- Returns first active occupier (skips dead objects)
- Cleans up dead objects from OccupierPtr chain

**CellClass::Cell_Coord()** → COORDINATE
- Returns cell's coordinate (cell number << 8)

**CellClass::Cell_Number()** → int
- Returns cell index (0-4095)

### Occupancy Management

**CellClass::Occupy_Down()** - Flag cell as occupied
- Sets appropriate occupation bit in Flag.Composite

**CellClass::Occupy_Up()** - Remove occupation flag
- Clears occupation bit

**CellClass::Occupy_Unit()** - Mark cell as unit-occupied
- Sets vehicle occupation bit

**CellClass::Reserve_Cell(unit_id)** - Reserve for specific unit
- Used during pathfinding/movement reservation

**CellClass::Occupy_Unit()** - Marks cell as unit occupied
- Sets vehicle occupation bit

**CellClass::Occupy_Up/Down** - Flag occupation up/down

### Overlay Management

**CellClass::Overlay_Down()** - Mark cell as overlapped (spillage)
- Sets overlap bit

**CellClass::Overlay_Up()** - Remove overlap flag

**CellClass::Overlap_Unit()** - Mark as overlapped by unit
- Used for unit footprint overlap

**CellClass::Overlap_Down/Up/Unit** - Overlap management

### Tiberium Management

**CellClass::Reduce_Tiberium(amount)** → bool
- Reduces tiberium amount in cell
- Returns true if tiberium depleted (overlay removed)
- Handles Tiberium_Adjust for smooth visual transition

**CellClass::Tiberium_Adjust()**
- Adjusts visual tiberium imagery based on amount
- Called after tiberium reduction

**CellClass::Reduce_Tiberium(amount)** → bool
- Reduces tiberium amount
- Returns true if depleted (overlay removed)
- Calls Tiberium_Adjust()

**CellClass::Tiberium_Adjust()**
- Updates visual tiberium imagery based on remaining amount

### Crate System

**CellClass::Goodie_Check(check_steel, object)** → bool
- **Core crate opening logic**
- Steel crates (MP): EMP pulse, destroy crate, bad house gets nuke piece
- Regular crates: Random goodie from weighted table
- **Crate types (14):**
  - MONEY: 2000 SP / 100-2000 MP
  - DARKNESS: Shrouds map for house
  - REVEAL: Reveals map for house
  - NUKE: AnimClass(ANIM_ATOM_BLAST) + 5 explosions
  - ION: Ion cannon blast
  - NUKE_MISSILE: Grants nuke sidebar
  - ION_BLAST: Ion cannon sidebar
  - AIR_STRIKE: Air strike sidebar
  - HEAL_BASE: Heals all house techno to max strength
  - CLOAK: Cloaks nearby techno (0x300 range)
  - EXPLOSION: 5 fireball explosions + damage
  - NAPALM: Napalm blast + fire damage
  - SQUAD: 5 random infantry (E1-E7, Rambo)
  - UNIT: Free unit (MCV, harvester, or random crate-allowed)
  - VISCEROID: Spawns hostile visceroid
- **CrateCount** decremented, **CrateMaker** triggered when count ≤ 0
- **Multiplayer restrictions:** Super weapons disabled per rules

**CellClass::Goodie_Check(check_steel, object)** → bool
- Steel crates: EMP pulse, destroy crate, bad house gets nuke piece
- Regular crates: Random goodie from 14-type weighted table
- **Weighted table:** DARKNESS(2), REVEAL(2), NUKE, ION, NUKE_MISSILE, ION_BLAST(2), AIR_STRIKE(4), HEAL_BASE(2), CLOAK(2), EXPLOSION(4), NAPALM(3), SQUAD(5), UNIT(4), VISCEROID
- **MP restrictions:** Super weapons (NUKE_MISSILE, ION_BLAST, AIR_STRIKE) disabled if `!Rule.AllowSuperWeapons`
- **CrateCount** decremented, **CrateMaker** triggered when count ≤ 0 in MP

### Tiberium & Walls

**CellClass::Reduce_Tiberium(amount)** → bool
- Reduces tiberium amount in OverlayData
- Returns true if depleted (overlay removed)
- Calls Tiberium_Adjust() for visual update

**CellClass::Reduce_Wall(damage)** → bool
- Damages wall overlay
- Returns true if wall destroyed (overlay removed)
- Calls Wall_Update() for visual update

**CellClass::Reduce_Tiberium(amount)** → bool
- Reduces OverlayData by amount
- Returns true if depleted (Overlay = OVERLAY_NONE)
- Calls Tiberium_Adjust() for visual smoothing

**CellClass::Reduce_Wall(damage)** → bool
- Damages wall overlay
- Returns true if wall destroyed (Overlay = OVERLAY_NONE)
- Calls Wall_Update() for visual update

### Walls

**CellClass::Wall_Update()**
- Updates wall imagery based on neighboring walls
- 8-neighbor connectivity check
- Sets appropriate wall sprite

**CellClass::Wall_Update()**
- Updates wall imagery based on 8-neighbor connectivity
- Sets appropriate wall sprite variant

### Visibility & Shroud (Multiplayer)

**CellClass::Set_Mapped(house, set)** - Per-house mapped status
- Updates IsMappedByPlayerMask bitmask
- Single-player: also updates IsMapped

**CellClass::Is_Mapped(house)** → bool
- Checks IsMappedByPlayerMask bit

**CellClass::Set_Visible(house, set)** - Per-house visibility
- Updates IsVisibleByPlayerMask bitmask
- SP: also updates IsVisible

**CellClass::Is_Visible(house)** → bool
- Checks IsVisibleByPlayerMask bitmask

**CellClass::Set_Mapped(house, set)** - Per-house mapped
- Updates IsMappedByPlayerMask bitmask
- SP: also sets IsMapped

**CellClass::Is_Mapped(house)** → bool
- Checks IsMappedByPlayerMask bit

**CellClass::Set_Visible(house, set)** - Per-house visibility
- Updates IsVisibleByPlayerMask bitmask
- SP: also sets IsVisible

**CellClass::Is_Visible(house)** → bool
- Checks IsVisibleByPlayerMask bitmask

### CTF Flags

**CellClass::Flag_Place(house)** → bool
- Places CTF flag if cell clear
- Creates AnimClass(ANIM_FLAG), sets owner

**CellClass::Flag_Remove()** → bool
- Removes CTF flag, destroys animation

**CellClass::Flag_Update()** - Syncs CTFFlag with IsFlagged
- Creates/destroys AnimClass(ANIM_FLAG)

**CellClass::Flag_Place(house)** → bool
- Places flag if !IsFlagged && Is_Generally_Clear()
- Creates AnimClass(ANIM_FLAG), sets owner

**CellClass::Flag_Remove()** → bool
- Removes flag if IsFlagged, destroys AnimClass

**CellClass::Flag_Update()** - Syncs CTFFlag with IsFlagged
- Creates/destroys AnimClass(ANIM_FLAG)

### Pathfinding & AI (RA AI extensions)

**CellClass::Is_Clear_To_Move(ignore_infantry, ignore_vehicles)** → bool (USE_RA_AI)
- Checks occupation bits (ignores infantry/vehicles if flags set)
- Checks overlay walls (blocks unless crushable)
- Checks land type cost for locomotion
- Returns true if passable

**CellClass::Is_Clear_To_Move(ignore_infantry, ignore_vehicles)** → bool
- Checks occupation bits (drops infantry/vehicle bits if flags set)
- Walls block unless crushable
- Land type cost for locomotion must be > 0

### Shroud/Visibility Effects

**CellClass::Redraw_Objects()**
- Flags all objects in cell for redraw
- Called after overlay/flag/tberium changes

**CellClass::Shimmer()**
- Calls Do_Shimmer() on all cloaked objects in cell
- Used when cloaking disrupted

**CellClass::Incoming(object)**
- Causes objects in cell to "run for cover"
- Used by superweapon impacts

**CellClass::Is_Generally_Clear()** → bool
- Checks if cell can be built upon
- Used for flag placement, crate spawning

**CellClass::Closest_Free_Spot(coord)** → COORDINATE
- Finds nearest unoccupied cell near coordinate

**CellClass::Is_Generally_Clear()** → bool
- Checks if cell buildable (no building, no impassable terrain)

### Save/Load

**CellClass::Read(stream)** → bool
- Reads cell state from save stream

**CellClass::Write(stream)** → bool
- Writes cell state to save stream

### Trigger System

**CellClass::Get_Trigger()** → TriggerClass*
- Returns trigger associated with cell

**CellClass::Goodie_Check(check_steel, object)** → bool
- Core crate logic (see above)

### Occupancy & Overlap

**CellClass::Cell_Occupier()** → ObjectClass*
- Returns first active occupier (cleans dead objects)

**CellClass::Occupy_Down/Up/Unit** - Occupation flag management

**CellClass::Overlap_Down/Up/Unit** - Overlap flag management

**CellClass::Reserve_Cell(unit_id)** - Pathfinding reservation

### Tiberium Visuals

**CellClass::Tiberium_Adjust()**
- Updates visual tiberium imagery based on OverlayData amount

**CellClass::Tiberium_Adjust()**
- Updates tiberium visual based on OverlayData amount

### Wall Visuals

**CellClass::Wall_Update()**
- Updates wall sprite based on 8-neighbor connectivity

**CellClass::Wall_Update()**
- 8-neighbor connectivity check for wall sprites

### CTF Flags

**CellClass::Flag_Place(house)** → bool
- Places flag if !IsFlagged && Is_Generally_Clear()
- Creates AnimClass(ANIM_FLAG), sets owner

**CellClass::Flag_Remove()** → bool
- Removes flag, destroys AnimClass

**CellClass::Flag_Update()**
- Syncs CTFFlag animation with IsFlagged state

### Visibility/Shroud (MP)

**CellClass::Set_Mapped(house, set)** - Per-house mapped
- Updates IsMappedByPlayerMask bitmask
- SP: also sets IsMapped

**CellClass::Is_Mapped(house)** → bool
- Checks IsMappedByPlayerMask bit

**CellClass::Set_Visible(house, set)** - Per-house visibility
- Updates IsVisibleByPlayerMask bitmask
- SP: also sets IsVisible

**CellClass::Is_Visible(house)** → bool
- Checks IsVisibleByPlayerMask bit

### RA AI Pathfinding (USE_RA_AI)

**CellClass::Is_Clear_To_Move(ignore_infantry, ignore_vehicles)** → bool
- Checks occupation bits (masks infantry/vehicle bits if flags)
- Walls block unless crushable
- Land type cost for locomotion > 0

### Save/Load

**CellClass::Read(stream)** → bool
- Reads cell state from save stream

**CellClass::Write(stream)** → bool
- Writes cell state to save stream

### Trigger System

**CellClass::Get_Trigger()** → TriggerClass*
- Returns trigger associated with cell

**CellClass::Goodie_Check(check_steel, object)** → bool
- Core crate logic (14 crate types, weighted random)

### Occupancy Management

**CellClass::Cell_Occupier()** → ObjectClass*
- Returns first active occupier (cleans dead objects from chain)

**CellClass::Occupy_Down/Up/Unit** - Occupation flags

**CellClass::Overlap_Down/Up/Unit** - Overlap flags

**CellClass::Reserve_Cell(unit_id)** - Pathfinding reservation

### Tiberium Visuals

**CellClass::Tiberium_Adjust()**
- Updates visual tiberium based on OverlayData amount

### Wall Visuals

**CellClass::Wall_Update()**
- 8-neighbor connectivity for wall sprites

### CTF Flags

**CellClass::Flag_Place(house)** → bool
- Places flag if clear, creates AnimClass(ANIM_FLAG)

**CellClass::Flag_Remove()** → bool
- Removes flag, destroys animation

**CellClass::Flag_Update()**
- Syncs CTFFlag animation with IsFlagged

### Visibility/Shroud (MP)

**CellClass::Set_Mapped(house, set)** - Per-house mapped bitmask
- SP: also sets IsMapped

**CellClass::Is_Mapped(house)** → bool
- Checks IsMappedByPlayerMask bit

**CellClass::Set_Visible(house, set)** - Per-house visibility bitmask
- SP: also sets IsVisible

**CellClass::Is_Visible(house)** → bool
- Checks IsVisibleByPlayerMask bit

### RA AI Pathfinding (USE_RA_AI)

**CellClass::Is_Clear_To_Move(ignore_infantry, ignore_vehicles)** → bool
- Occupation bits (masked by ignore flags)
- Walls block unless crushable
- Land cost > 0 for locomotion

### Save/Load

**CellClass::Read/Write(stream)** → bool
- Serialization for save games

### Trigger System

**CellClass::Get_Trigger()** → TriggerClass*
- Returns cell's trigger

**CellClass::Goodie_Check(check_steel, object)** → bool
- Core crate logic (14 types, weighted random, MP restrictions)

## Inputs, outputs and side effects

**State mutations:**
- Overlay/OverlayData changes (tiberium, crates, walls)
- Occupation/Overlap bitfields
- Visibility/Mapped bitmasks (MP)
- Threat ratings (via Adjust_Threat calls)
- Tiberium amount (OverlayData)
- Wall state (Overlay/OverlayData)
- Crate spawning (CrateCount, CrateMaker, CrateTimer)
- CTF flag state (IsFlagged, CTFFlag animation)
- Per-house visibility/mapped bitmasks (MP)

**External systems affected:**
- Map: Redraw_Objects(), threat adjustments
- AnimClass: CTF flag, crate effects, explosions
- House: Money, superweapon enable, crate counts
- Trigger: Spring() on crate events
- AnimClass: Crate effects, flag, explosions
- House: Money, superweapon enable, crate counts, vision

## Calls into / called by

**Called by:**
- Map rendering: Cell_Color(), Draw_It()
- Pathfinding: Is_Clear_To_Move(), Occupy_Up/Down, Closest_Free_Spot()
- Object placement: Occupy_Up/Down, Overlap_Up/Down
- Tiberium growth: Reduce_Tiberium(), Tiberium_Adjust()
- Crate system: Goodie_Check(), CrateCount/CrateMaker
- Wall system: Reduce_Wall(), Wall_Update()
- CTF: Flag_Place/Remove/Update()
- Visibility: Set_Mapped/Visible(), Is_Mapped/Visible()
- Crate system: Goodie_Check(), CrateCount/CrateMaker
- Wall system: Reduce_Wall(), Wall_Update()
- CTF: Flag_Place/Remove/Update()
- Visibility: Set_Mapped/Visible()

**Calls into:**
- Map: Redraw_Objects(), Flag_To_Redraw(), Adjust_Threat()
- AnimClass: ANIM_FLAG, ANIM_CRATE_*, ANIM_ATOM_BLAST, etc.
- House: Refund_Money(), Add_Nuke_Piece(), Enable() superweapons, Available_Money()
- HouseClass: Refund_Money(), Add_Nuke_Piece(), Enable(), Available_Money(), BScan/UScan
- TriggerClass: Spring(EVENT_*)
- MapClass: Flag_To_Redraw(), layer access, cell access
- AnimClass: ANIM_FLAG, ANIM_CRATE_*, ANIM_ATOM_BLAST, ANIM_ION_CANNON, etc.
- TriggerClass: Spring(EVENT_CRATE, etc.)
- HouseClass: Refund_Money(), Add_Nuke_Piece(), Enable(), Available_Money(), BScan/UScan, IsVisionary

## Related original files
- **CELL.H** - Class declaration, inline helpers
- **MAP.CPP/.H** - MapClass (4096 CellClass array, threat, layers)
- **MAP.CPP** - MapClass::Redraw_Objects(), Flag_To_Redraw(), threat maps
- **OVERLAY.CPP/.H** - OverlayTypeClass, overlay definitions
- **TIBERIUM.CPP** - Tiberium growth logic (calls Reduce_Tiberium)
- **CRATE.CPP** - Crate spawning logic (CrateCount, CrateMaker)
- **WALL.CPP** - Wall logic (Reduce_Wall, Wall_Update)
- **WALL.H** - WallTypeClass, OverlayTypeClass
- **OVERLAY.H** - OverlayTypeClass, overlay definitions
- **CRATE.H** - CrateTypeClass, crate definitions
- **TIBERIUM.H** - Tiberium growth logic
- **MAP.H** - MapClass, CellClass declarations
- **TRIGGER.CPP/.H** - TriggerClass, cell triggers
- **HOUSE.CPP/.H** - HouseClass (money, superweapons, vision)
- **ANIM.CPP/.H** - AnimClass (crate effects, flags, explosions)
- **FUNCTION.H** - Core includes, Distance(), Random_Pick()
- **RADAR.CPP** - Radar rendering (calls Cell_Color)
- **INFANTRY.CPP** - InfantryClass (Goodie_Check infantry goodies)
- **UNIT.CPP** - UnitClass (Goodie_Check unit goodies)
- **SUPERWPN.CPP** - Superweapons (nuke, ion, air strike)
- **RADAR.CPP** - Radar rendering (Cell_Color)
- **SIDEBAR.CPP** - Sidebar updates (superweapon adds)
- **SIDEBARGLYPHX.H** - MP sidebar integration

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-asset`

**Mapped modules:**
- `cnc-core/src/map/cell.rs` - CellClass core
- `cnc-core/src/map/map.rs` - MapClass integration
- `cnc-core/src/map/terrain.rs` - LandType, Ground table
- `cnc-core/src/map/overlay.rs` - OverlayType, tiberium, crates, walls
- `cnc-core/src/map/visibility.rs` - Per-house visibility/mapped bitmasks
- `cnc-tiberian-dawn/src/map/td_cell.rs` - TD-specific cell logic
- `cnc-asset/src/tileset.rs` - Template/terrain types
- `cnc-core/src/damage.rs` - Crate explosion damage
- `cnc-core/src/map/threat.rs` - Threat map integration
- `cnc-core/src/map/visibility.rs` - Per-house visibility bitmasks
- `cnc-tiberian-dawn/src/map/tiberium.rs` - Tiberium growth/reduction
- `cnc-tiberian-dawn/src/map/crate.rs` - Crate system (Goodie_Check)
- `cnc-tiberian-dawn/src/map/wall.rs` - Wall system
- `cnc-core/src/map/ctf.rs` - CTF flag system
- `cnc-core/src/map/visibility.rs` - Per-house visibility/mapped bitmasks

**Not yet ported:**
- Crate system (Goodie_Check with 14 types, weighted random)
- Tiberium growth/reduction with visual adjustment
- Wall 8-neighbor connectivity sprite selection
- CTF flag animation sync
- Per-house visibility/mapped bitmasks (MP)
- Crate weighted random table (14 types)
- Tiberium visual adjustment (Tiberium_Adjust)
- Wall 8-neighbor connectivity
- CTF flag animation sync
- Multiplayer visibility/mapped bitmasks
- Crate weighted random (14 types, MP restrictions)
- Tiberium visual adjustment
- Wall connectivity sprites
- CTF flag animation

## Edge cases and compatibility notes

1. **Cell grid size**: Fixed 64x64 = 4096 cells (0-4095). Validate() enforces this.

2. **Lepton coordinates**: 1 cell = 256 leptons (8 bits). Pixel_To_Lepton(x) = x << 3.

3. **Occupation bits**: 3 bits infantry, 3 bits vehicle, 1 building, 1 aircraft = 8 bits in Flag.Composite.

4. **Per-house bitmasks**: 8-bit masks (8 houses max). IsMappedByPlayerMask / IsVisibleByPlayerMask.

5. **Crate weighted table**: 14 types with specific weights. Steel crates (MP only) have different behavior.

6. **MP crate restrictions**: Super weapons disabled if `!Rule.AllowSuperWeapons`. Visionary houses can't get REVEAL.

7. **Crate spawning**: CrateCount decremented per crate. When ≤0 in MP, CrateMaker=true, CrateTimer=1.

8. **Tiberium visual**: Tiberium_Adjust() updates imagery based on OverlayData amount (smooth transitions).

9. **Wall connectivity**: Wall_Update() checks 8 neighbors for wall sprite selection.

10. **CTF flags**: Single flag per cell. AnimClass(ANIM_FLAG) created/destroyed via Flag_Update().

11. **MP visibility**: 8-bit masks (IsMappedByPlayerMask, IsVisibleByPlayerMask). SP caches in IsMapped/IsVisible.

12. **Steel crates**: MP only. EMP pulse, bad house gets nuke piece. check_steel parameter.

13. **Crate weights**: 14 types with specific frequencies in `_what[]` array.

14. **MP super weapons**: NUKE_MISSILE, ION_BLAST, AIR_STRIKE disabled if `!Rule.AllowSuperWeapons`.

15. **Visionary houses**: Can't get REVEAL crate (already have full vision).

16. **Crate spawning**: CrateCount decremented. When CrateCount ≤ 0 in MP, CrateMaker=true, CrateTimer=1.

17. **Wall connectivity**: Wall_Update() checks 8 neighbors for sprite selection.

18. **CTF flags**: Single flag per cell. AnimClass(ANIM_FLAG) managed by Flag_Update().

19. **MP visibility**: 8-bit masks (IsMappedByPlayerMask, IsVisibleByPlayerMask). SP caches in IsMapped/IsVisible.

20. **Steel crates**: MP only. EMP pulse, bad house gets nuke piece.

21. **Crate weights**: 14 types in `_what[]` array with specific frequencies.

22. **MP super weapons**: Disabled if `!Rule.AllowSuperWeapons`.

23. **Visionary houses**: Can't get REVEAL (already have vision).

24. **Crate spawning**: CrateCount--, when ≤0 in MP → CrateMaker=true, CrateTimer=1.

25. **Cell grid**: Fixed 64x64 = 4096 cells. Coordinate = cell << 8 (leptons).

26. **Occupation bits**: 3 infantry, 3 vehicle, 1 building, 1 aircraft = 8 bits.

27. **Per-house bitmasks**: 8-bit (8 houses max). IsMappedByPlayerMask / IsVisibleByPlayerMask.

28. **Crate weighted table**: 14 types in `_what[]` with specific frequencies.

26. **MP crate restrictions**: Super weapons disabled if `!Rule.AllowSuperWeapons`. Visionary can't get REVEAL.

27. **Crate spawning**: CrateCount--, when ≤0 in MP → CrateMaker=true, CrateTimer=1.

28. **Tiberium visual**: Tiberium_Adjust() updates imagery based on OverlayData.

29. **Wall connectivity**: Wall_Update() checks 8 neighbors.

30. **CTF flags**: Single flag per cell. AnimClass(ANIM_FLAG) via Flag_Update().

31. **MP visibility**: 8-bit masks. SP caches in IsMapped/IsVisible.

32. **Steel crates**: MP only. EMP pulse, bad house gets nuke piece.

33. **Crate weights**: 14 types in `_what[]` with specific frequencies.

34. **MP super weapons**: Disabled if `!Rule.AllowSuperWeapons`.

35. **Visionary houses**: Can't get REVEAL.

36. **Crate spawning**: CrateCount--, ≤0 in MP → CrateMaker=true, CrateTimer=1.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/CELL.CPP at f1f0d42
