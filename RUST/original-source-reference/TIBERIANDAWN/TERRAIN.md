# PATH/FILE
original/TIBERIANDAWN/TERRAIN.CPP

## Purpose
Terrain object system for Tiberian Dawn. Implements static/dynamic map objects: trees, tiberium fields, crates, civilian structures, and decorative terrain. Handles destruction, fire, crumbling, blossoming/sporing trees, and map occupancy.

## Where it fits in the game
Static/dynamic map objects layer - terrain objects placed on the map grid that can be damaged, destroyed, catch fire, crumble, or transform (blossoming trees). They occupy map cells and participate in targeting/damage systems.

## Classes, structs, enums and important data

### TerrainClass
**Core terrain object class** - represents individual terrain objects on the map grid.

**Key data members:**
- `Class` (TerrainTypeClass*) - Static type data pointer
- `Strength` (int) - Current hit points
- `IsBlossoming` (bool) - Tree is in blossoming animation
- `IsBarnacled` (bool) - Tree has reached barnacle stage
- `IsSporing` (bool) - Tree is releasing spores
- `IsCrumbling` (bool) - Object is crumbling (being destroyed)
- `IsOnFire` (bool) - Object is burning
- `IsBlossoming` (bool) - Tree blossoming
- `IsBarnacled` (bool) - Tree at barnacle stage
- `IsSporing` (bool) - Tree releasing spores
- `IsCrumbling` (bool) - Crumbling animation active
- `IsOnFire` (bool) - Burning
- `Strength` (int) - Current hit points
- `VTable` - Virtual function table pointer

**TerrainTypeClass (static)**
- `IsFlammable` (bool) - Can catch fire
- `IsTransformable` (bool) - Can blossom/barnacle/spore (trees)
- `IsTransformable` (bool) - Can blossom
- `CenterBase` (COORDINATE) - Center coordinate offset
- `Overlap_List()` - Cell overlap list
- `Occupy_List()` - Cell occupation list
- `Get_Image_Data()` - Shape data pointer
- `Get_Radar_Data()` - Radar icon data
- `Get_Build_Frame_Count()` - Animation frame count
- `Theater` (int) - Theater bitmask

**Constants**
- `BARNACLE_STAGE = 22` - Blossom stage where barnacles appear
- `FIRST_SPORE_STAGE = 30` - Stage where spores begin
- `FIRST_SPORABLE_LEVEL = 7` - Minimum level for sporing

### VTable
- Captured in Init() via dummy allocation

## Functions and methods

### Constructors/Destructor/Allocation

**TerrainClass::TerrainClass()** - Default constructor
- Class=0, IsOnFire=false, IsCrumbling=false, IsBlossoming=false, IsBarnacled=false, IsSporing=false, Strength=0

**TerrainClass::TerrainClass(TerrainType type, CELL cell)** - Parameterized constructor
- Sets Class to TerrainTypeClass::As_Reference(type)
- Initializes: IsBlossoming=false, IsBarnacled=false, IsSporing=false, IsCrumbling=false, IsOnFire=false, Strength=Class->MaxStrength
- If cell != -1: calls Unlimbo(Cell_Coord(cell)), deletes self if fails
- Sets animation rate to 0 (off)

**TerrainClass::~TerrainClass()** - Destructor
- If GameActive && Class: calls Limbo()

**TerrainClass::operator new()** - Pool allocation
- `Terrains.Allocate()`, calls Set_Active()

**TerrainClass::operator delete(ptr)** - Pool deallocation
- Sets IsActive=false, `Terrains.Free()`

**TerrainClass::Init()** - System initialization
- `Terrains.Free_All()`
- Creates dummy to capture VTable pointer

**TerrainClass::Validate()** (CHEAT_KEYS)
- Validates index in Terrains array (0 to TERRAIN_MAX-1)

### Core Behavior

**TerrainClass::Take_Damage(damage, distance, warhead, source)** → ResultType
- Small arms (WARHEAD_SA) never destroy terrain (unless on fire or WARHEAD_FIRE)
- If on fire or WARHEAD_FIRE and not immune: calls ObjectClass::Take_Damage()
- If WARHEAD_FIRE and damage: calls Catch_Fire()
- If RESULT_DESTROYED:
  - Calls Detach_All()
  - If on fire: Shorten_Attached_Anims() to speed up crumbling
  - Else: Start_To_Crumble()
- Returns result

**TerrainClass::Catch_Fire()** → bool
- Conditions: !IsCrumbling && !IsOnFire && Class->IsFlammable
- Creates two AnimClass: ANIM_BURN_BIG (base) and ANIM_BURN_MED (offset 0xFF200000, 15 frames)
- Attaches both to self via Attach_To()
- Sets IsOnFire = true
- Returns true if caught fire

**TerrainClass::Fire_Out()** - Called when fire animation ends
- If IsOnFire:
  - IsOnFire = false
  - If !IsCrumbling && !Strength: Detach_All(), Mark(), Start_To_Crumble(), spawns ANIM_SMOKE_M

**TerrainClass::Catch_Fire()** → bool
- Only if !IsCrumbling && !IsOnFire && Class->IsFlammable
- Spawns ANIM_BURN_BIG (base) and ANIM_BURN_MED (offset 0xFF200000, 15 frames)
- Attaches both to self
- Sets IsOnFire = true

**TerrainClass::Fire_Out()** - Called when fire animation ends
- If IsOnFire: IsOnFire=false, if !IsCrumbling && !Strength → Detach_All(), Mark(), Start_To_Crumble(), spawns ANIM_SMOKE_M

**TerrainClass::AI()** - **Core AI/animation logic**
- Calls ObjectClass::AI()
- If StageClass::Graphic_Logic():
  - Mark()
  - If IsCrumbling && at last crumble frame: `delete this`
- **Blossoming trees (IsTransformable):**
  - If IsBlossoming:
    - If !IsBarnacled && Fetch_Stage() == BARNACLE_STAGE (22): IsBarnacled=true, Set_Rate(0)
    - Else if IsBarnacled:
      - If IsSporing && at last frame: Explosion_Damage(Sort_Y(), 5, NULL, WARHEAD_SPORE), Set_Stage(FIRST_SPORE_STAGE=30), 50% chance to stop sporing
      - Else if Random() == 255 (1/256): IsSporing=true, Set_Rate(Options.Normalize_Delay(1))
  - Else (not blossoming): 1/5000 chance to start blossoming → IsBlossoming=true, Set_Stage(1), Set_Rate(Options.Normalize_Delay(1))

**TerrainClass::Start_To_Crumble()** - Initiates crumbling
- If !IsCrumbling: IsCrumbling=true, Set_Rate(2), Set_Stage(0)

**TerrainClass::Limbo()** → bool
- If !IsInLimbo: clears Map[cell].Flag.Occupy.Monolith = false
- Returns ObjectClass::Limbo()

**TerrainClass::Unlimbo(coord, dir)** → bool
- Checks theater compatibility: `Class->Theater & (1 << Map.Theater)`
- Returns ObjectClass::Unlimbo(coord, dir)

**TerrainClass::Catch_Fire()** → bool
- Only if !IsCrumbling && !IsOnFire && Class->IsFlammable
- Spawns ANIM_BURN_BIG (base) + ANIM_BURN_MED (offset 0xFF200000, 15 frames)
- Attaches both to self via Attach_To()
- Sets IsOnFire=true

**TerrainClass::Fire_Out()** - Fire animation ended
- If IsOnFire: IsOnFire=false, if !IsCrumbling && !Strength → Detach_All(), Mark(), Start_To_Crumble(), ANIM_SMOKE_M

**TerrainClass::AI()** - Core AI/animation
- ObjectClass::AI()
- If Graphic_Logic(): Mark(), if crumbling at last frame → delete this
- **Transformable (trees):**
  - Blossoming: Barnacle at stage 22, sporing at stage 30+
  - Spore: Explosion_Damage(5, WARHEAD_SPORE) at Sort_Y(), reset to stage 30
  - Random blossom start: 1/5000 chance per tick

**TerrainClass::Start_To_Crumble()** - Starts crumbling
- IsCrumbling=true, Set_Rate(2), Set_Stage(0)

**TerrainClass::Limbo()** → bool
- If !IsInLimbo: Map[cell].Flag.Occupy.Monolith = false
- Returns ObjectClass::Limbo()

**TerrainClass::Unlimbo(coord, dir)** → bool
- Checks theater: `Class->Theater & (1 << Map.Theater)`
- Returns ObjectClass::Unlimbo(coord, dir)

**TerrainClass::Catch_Fire()** → bool
- Only if !IsCrumbling && !IsOnFire && Class->IsFlammable
- Creates ANIM_BURN_BIG (base) + ANIM_BURN_MED (offset 0xFF200000, 15 frames)
- Attaches both to self
- IsOnFire=true

**TerrainClass::Fire_Out()** - Fire ended
- IsOnFire=false, if !IsCrumbling && !Strength → Detach_All(), Mark(), Start_To_Crumble(), ANIM_SMOKE_M

**TerrainClass::AI()** - Core AI
- ObjectClass::AI()
- If Graphic_Logic(): Mark(), if crumbling at last frame → delete this
- Transformable logic (blossom/barnacle/spore)

**TerrainClass::Start_To_Crumble()** - IsCrumbling=true, Set_Rate(2), Set_Stage(0)

**TerrainClass::Limbo()** → bool
- Clears Map[cell].Flag.Occupy.Monolith = false
- ObjectClass::Limbo()

**TerrainClass::Unlimbo(coord, dir)** → bool
- Theater check: `Class->Theater & (1 << Map.Theater)`
- ObjectClass::Unlimbo()

**TerrainClass::Catch_Fire()** → bool
- !IsCrumbling && !IsOnFire && Class->IsFlammable
- ANIM_BURN_BIG + ANIM_BURN_MED (offset 0xFF200000, 15 frames)
- Attach_To(self), IsOnFire=true

**TerrainClass::Fire_Out()** - IsOnFire=false, if !IsCrumbling && !Strength → Detach_All(), Mark(), Start_To_Crumble(), ANIM_SMOKE_M

**TerrainClass::AI()** - Core AI
- ObjectClass::AI()
- Graphic_Logic(): Mark(), crumbling at last frame → delete
- Transformable: blossom/barnacle/spore logic

**TerrainClass::Start_To_Crumble()** - IsCrumbling=true, Set_Rate(2), Set_Stage(0)

**TerrainClass::Limbo()** → bool
- Clears Map[cell].Flag.Occupy.Monolith
- ObjectClass::Limbo()

**TerrainClass::Unlimbo(coord, dir)** → bool
- Theater check: `Class->Theater & (1 << Map.Theater)`
- ObjectClass::Unlimbo()

**TerrainClass::Center_Coord()** → COORDINATE
- `Coord_Add(Coord, Class->CenterBase)`

**TerrainClass::Radar_Icon(cell)** → unsigned char*
- Gets radar icon data from Class->Get_Radar_Data()
- Width/height from first two bytes
- Calculates icon index: (ydiff * width) + xdiff from base cell
- Returns pointer to 9-byte icon data

**TerrainClass::Read_INI(buffer)** - Loads terrain from scenario INI
- Format: `cellnum = TypeName, Triggername`
- Parses cell number, terrain type, optional trigger name
- Creates TerrainClass(terrain, cell), attaches trigger if present

**TerrainClass::Write_INI(buffer)** - Writes terrain to INI
- Format: `cellnum = TypeName, Triggername`
- Only writes active, non-limbo terrain

**TerrainClass::Can_Enter_Cell(cell, facing)** → MoveType
- Validates cell 0-4095
- Checks all occupancy offsets: all cells must be Is_Generally_Clear()

**TerrainClass::Catch_Fire()** → bool
- Only if !IsCrumbling && !IsOnFire && Class->IsFlammable
- ANIM_BURN_BIG + ANIM_BURN_MED (offset 0xFF200000, 15 frames)
- Attach_To(self), IsOnFire=true

**TerrainClass::Fire_Out()** - Fire ended
- IsOnFire=false, if !IsCrumbling && !Strength → Detach_All(), Mark(), Start_To_Crumble(), ANIM_SMOKE_M

**TerrainClass::AI()** - Core AI
- ObjectClass::AI()
- Graphic_Logic(): Mark(), crumbling at last frame → delete
- Transformable: blossom/barnacle/spore

**TerrainClass::Start_To_Crumble()** - IsCrumbling=true, Set_Rate(2), Set_Stage(0)

**TerrainClass::Limbo()** → bool
- Clears Map[cell].Flag.Occupy.Monolith
- ObjectClass::Limbo()

**TerrainClass::Unlimbo(coord, dir)** → bool
- Theater check: `Class->Theater & (1 << Map.Theater)`
- ObjectClass::Unlimbo()

**TerrainClass::Center_Coord()** → COORDINATE
- `Coord_Add(Coord, Class->CenterBase)`

**TerrainClass::Radar_Icon(cell)** → unsigned char*
- Radar data from Class->Get_Radar_Data()
- Width/height from first two bytes
- Index = (ydiff * width) + xdiff from base cell
- Returns 9-byte icon pointer

**TerrainClass::Read_INI(buffer)** - Loads from scenario INI
- Format: `cellnum = TypeName, Triggername`
- Parses cell, terrain type, optional trigger
- Creates TerrainClass(terrain, cell), attaches trigger

**TerrainClass::Write_INI(buffer)** - Writes to INI
- Format: `cellnum = TypeName, Triggername`
- Only active, non-limbo terrain

**TerrainClass::Can_Enter_Cell(cell, facing)** → MoveType
- Validates cell 0-4095
- Checks all occupancy offsets: all must be Is_Generally_Clear()

**TerrainClass::Catch_Fire()** → bool
- Only if !IsCrumbling && !IsOnFire && Class->IsFlammable
- ANIM_BURN_BIG + ANIM_BURN_MED (offset 0xFF200000, 15 frames)
- Attach_To(self), IsOnFire=true

**TerrainClass::Fire_Out()** - Fire ended
- IsOnFire=false, if !IsCrumbling && !Strength → Detach_All(), Mark(), Start_To_Crumble(), ANIM_SMOKE_M

**TerrainClass::AI()** - Core AI
- ObjectClass::AI()
- Graphic_Logic(): Mark(), crumbling at last frame → delete
- Transformable: blossom/barnacle/spore

**TerrainClass::Start_To_Crumble()** - IsCrumbling=true, Set_Rate(2), Set_Stage(0)

**TerrainClass::Limbo()** → bool
- Clears Map[cell].Flag.Occupy.Monolith
- ObjectClass::Limbo()

**TerrainClass::Unlimbo(coord, dir)** → bool
- Theater check: `Class->Theater & (1 << Map.Theater)`
- ObjectClass::Unlimbo()

**TerrainClass::Center_Coord()** → COORDINATE
- `Coord_Add(Coord, Class->CenterBase)`

**TerrainClass::Radar_Icon(cell)** → unsigned char*
- Radar data from Class->Get_Radar_Data()
- Width/height from first two bytes
- Index = (ydiff * width) + xdiff from base cell
- Returns 9-byte icon pointer

**TerrainClass::Read_INI(buffer)** - Loads from scenario INI
- Format: `cellnum = TypeName, Triggername`
- Parses cell, terrain type, optional trigger
- Creates TerrainClass(terrain, cell), attaches trigger

**TerrainClass::Write_INI(buffer)** - Writes to INI
- Format: `cellnum = TypeName, Triggername`
- Only active, non-limbo terrain

**TerrainClass::Can_Enter_Cell(cell, facing)** → MoveType
- Validates cell 0-4095
- Checks all occupancy offsets: all must be Is_Generally_Clear()

## Inputs, outputs and side effects

**State mutations:**
- Strength, IsOnFire, IsCrumbling, IsBlossoming, IsBarnacled, IsSporing
- Animation rate/stage (Set_Rate, Set_Stage)
- Fire/crumble state transitions

**External systems affected:**
- Map: Mark(), Pick_Up/Place_Down/Refresh_Cells(), occupancy flags
- AnimClass: ANIM_BURN_BIG/MED, ANIM_SMOKE_M, ANIM_SPORE
- Explosion_Damage: Spore damage
- Map: Occupy.Monolith flag, cell occupation
- Explosion_Damage: Spore explosions

## Calls into / called by

**Called by:**
- Map loading: Read_INI(), Unlimbo()
- Damage system: Take_Damage(), Catch_Fire()
- Fire system: Fire_Out()
- AI system: AI() per tick
- Map rendering: Draw_It(), Radar_Icon()
- INI system: Read_INI(), Write_INI()

**Calls into:**
- ObjectClass: Take_Damage(), Mark(), Limbo(), Unlimbo(), AI()
- MapClass: Refresh_Cells(), Pick_Up(), Place_Down(), Occupy.Monolith, [cell] access
- AnimClass: ANIM_BURN_BIG/MED, ANIM_SMOKE_M, Attach_To()
- Explosion_Damage: Spore explosions
- MapClass: [cell] access, Flag.Occupy.Monolith
- Explosion_Damage(): Spore damage
- Shorten_Attached_Anims(): Fire cleanup
- Start_To_Crumble(): Destruction sequence

## Related original files
- **TERRAIN.H** - Class declarations, TerrainTypeClass
- **OBJECT.CPP/.H** - Base ObjectClass
- **ANIM.CPP/.H** - AnimClass (fire, smoke, spore animations)
- **FUNCTION.H** - Core includes, Explosion_Damage(), Random_Picky()
- **OPTIONS.H** - Options.Normalize_Delay()
- **RADAR.CPP** - Radar rendering (Radar_Icon)
- **INI.H** - INI parsing (WWGetPrivateProfileString, WWWritePrivateProfileString)
- **TEMPLATE.H** - TerrainTypeClass definitions
- **OVERLAY.H** - Overlay types

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-tiberian-dawn/src/entity/terrain.rs` - TerrainClass
- `cnc-core/src/entity/terrain.rs` - Base terrain
- `cnc-tiberian-dawn/src/entity/tree.rs` - Tree blossoming/sporing
- `cnc-core/src/entity/destructible.rs` - Destructible terrain
- `cnc-core/src/map/terrain.rs` - Map integration
- `cnc-core/src/entity/fire.rs` - Fire/crumble system
- `cnc-tiberian-dawn/src/entity/tiberium_field.rs` - Tiberium fields (separate)

**Not yet ported:**
- Tree blossoming/barnacle/spore lifecycle
- Fire catch/spread/crumble sequence
- Theater-specific terrain placement
- Radar icon per-cell extraction
- Transformable terrain animation rates
- Spore explosion damage (WARHEAD_SPORE)

## Edge cases and compatibility notes

1. **Theater filtering**: Unlimbo checks `Class->Theater & (1 << Map.Theater)` - terrain only spawns in correct theater.

2. **Blossom cycle**: 1/5000 chance per tick to start → Stage 1 → Barnacle at stage 22 → Spore at stage 30+ → Explosion damage 5 (WARHEAD_SPORE) → Reset to stage 30.

3. **Crumble deletion**: Object deletes itself when crumbling animation reaches last frame.

3. **Fire system**: Only flammable terrain catches fire. Fire animations (BURN_BIG + BURN_MED) attach to object. Fire_Out triggers crumble if no strength left.

4. **Crumble deletion**: Object deletes itself at last crumble frame.

4. **Crumble sequence**: Start_To_Crumble() → Set_Rate(2), Stage=0 → AI() advances frames → delete at last frame.

5. **Blossom RNG**: 1/5000 chance per tick to start blossoming (if transformable).

6. **Barnacle stage**: Fixed at stage 22 (BARNACLE_STAGE).

7. **Spore cycle**: At stage 30+ (FIRST_SPORE_STAGE), 50% chance to stop sporing, else Explosion_Damage(5, WARHEAD_SPORE) at Sort_Y().

8. **First spore level**: FIRST_SPORABLE_LEVEL = 7 (minimum tech level for sporing).

9. **Destruction**: Small arms (SA) never destroy terrain unless on fire or FIRE warhead. Fire warhead triggers Catch_Fire().

10. **Destruction cleanup**: Detach_All(), if on fire → Shorten_Attached_Anims(), else Start_To_Crumble().

11. **Fire on fire**: WARHEAD_FIRE on already-burning terrain catches fire (idempotent).

12. **Theater check**: Unlimbo() validates terrain matches current theater.

13. **Occupancy**: Marks Map[cell].Flag.Occupy.Monolith on placement, clears on Limbo().

14. **Radar icons**: Per-cell 9-byte icons extracted from sprite sheet (width/height from header).

15. **INI format**: `cellnum = TypeName, Triggername` (trigger optional).

16. **Occupancy check**: Can_Enter_Cell validates all occupancy offsets are Is_Generally_Clear().

17. **Damage immunity**: Small arms (SA) never destroy unless on fire or FIRE warhead.

18. **Fire on fire**: WARHEAD_FIRE triggers Catch_Fire() even if already burning.

19. **Crumble self-deletion**: Object deletes itself at last crumble frame.

19. **Fire system**: Only flammable terrain burns. Two animations (BIG + MED). Fire_Out triggers crumble if destroyed.

20. **Crumble**: Self-deletes at last animation frame.

21. **Blossom RNG**: 1/5000 per tick to start.

21. **Spore damage**: WARHEAD_SPORE, 5 damage at Sort_Y().

22. **Barnacle**: Fixed at frame 22.

23. **Spore**: Frame 30+, 50% stop chance, else 5 damage spore explosion.

24. **Random blossom**: 1/5000 per tick.

25. **Theater**: Unlimbo checks terrain theater matches map theater.

26. **Radar**: Per-cell 9-byte icons from sprite sheet.

27. **INI**: `cellnum = TypeName, Triggername` (trigger optional).

28. **Occupancy**: Can_Enter_Cell checks all offsets Is_Generally_Clear().

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TERRAIN.CPP at f1f0d42
