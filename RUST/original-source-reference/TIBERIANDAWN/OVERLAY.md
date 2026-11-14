# PATH/FILE
original/TIBERIANDAWN/OVERLAY.CPP

## Purpose
Overlay system for Tiberian Dawn. Implements static map overlays: roads, walls, crates, tiberium, concrete, smudges, and decorative overlays. Handles placement logic, multiplayer restrictions, and INI serialization.

## Where it fits in the game
Map overlay layer - static objects placed on cells that modify terrain appearance and behavior. Overlays are placed during map loading or gameplay (construction, crate drops) and modify cell state (overlay type, data, ownership).

## Classes, structs, enums and important data

### OverlayClass
**Core overlay object** - temporary object used during placement, deleted after marking on map.

**Key data members:**
- `Class` (OverlayTypeClass*) - Static type data pointer
- `ToOwn` (HousesType) - House ownership for walls/flags (static, set during placement)
- `VTable` - Virtual function table pointer

**OverlayTypeClass (static)**
- `Type` (OverlayType) - Overlay type enum
- `IsWall` (bool) - Is a wall type
- `IsCrate` (bool) - Is a crate type
- `Land` (LandType) - Land type for tiberium/concrete
- `IsCrate` (bool) - Crate flag
- `IsWall` (bool) - Wall flag
- `Land` (LandType) - Land type (LAND_TIBERIUM, etc.)
- `Create_And_Place(cell)` - Creates and places adjacent overlay (concrete)
- `IniName` (char*) - INI name string
- `Type` (OverlayType) - Overlay type enum

### OverlayType enum (key values)
| Type | Description |
|------|-------------|
| OVERLAY_NONE | No overlay |
| OVERLAY_ROAD | Road foundation/completed |
| OVERLAY_WALL | Wall (various types) |
| OVERLAY_CRATE | Crate (MP restricted) |
| OVERLAY_TIBERIUM | Tiberium field |
| OVERLAY_CONCRETE | Concrete |
| OVERLAY_SQUISH | Squish/body overlay |
| OVERLAY_FLAG_SPOT | CTF flag spot |
| OVERLAY_ROAD | Road (foundation=0, complete=1) |
| OVERLAY_SQUISH | Body overlay |
| OVERLAY_FLAG_SPOT | CTF flag spot |

### OverlayClass static data
- `ToOwn` (HousesType) - Static ownership for wall placement (HOUSE_NONE default)
- `VTable` - Virtual function table pointer

### Road overlay data
- `OverlayData = 0` - Foundation placed (buildable terrain)
- `OverlayData = 1` - Road completed

### Tiberium overlay data
- `OverlayData = 1` - Tiberium present (triggers Tiberium_Adjust)

### Concrete adjacency
- Even X row: adds concrete to East neighbor
- Odd X row: adds concrete to West neighbor
- Triggers Concrete_Calc() on all 4 adjacent cells

## Functions and methods

### OverlayClass

**OverlayClass::OverlayClass()** - Default constructor
- Class=0, ToOwn=HOUSE_NONE

**OverlayClass::OverlayClass(OverlayType type, CELL pos, HousesType house)** - Placement constructor
- Sets Class to OverlayTypeClass::As_Reference(type)
- If pos != -1: sets ToOwn=house, calls Unlimbo(Cell_Coord(pos)), resets ToOwn=HOUSE_NONE

**OverlayClass::Mark(MarkType mark)** → bool - **Core placement logic**
- Only MARK_DOWN supported
- Gets cell = Coord_Cell(Coord), cellptr = &Map[cell]
- **Road (OVERLAY_ROAD)**:
  - Foundation: cellptr->Overlay == OVERLAY_ROAD && OverlayData == 0 on buildable terrain
  - Complete: Overlay == OVERLAY_NONE && Is_Generally_Clear()
  - Foundation: OverlayData=0, Road: OverlayData=1
  - Sets cellptr->Overlay = Class->Type, Redraw_Objects()
- **Wall (Class->IsWall)**:
  - Requires Is_Generally_Clear() && Overlay != OVERLAY_FLAG_SPOT
  - Sets Overlay = Class->Type, OverlayData = 0, Redraw_Objects(), Wall_Update()
  - If ToOwn != HOUSE_NONE: cellptr->Owner = ToOwn
  - Else: Delete_This(), returns false
- **Other overlays (crates, tiberium, concrete, etc.)**:
  - Requires Overlay == NONE or SQUISH, no terrain, Ground[Land_Type].Build == true
  - Crate: CrateCount++
  - Non-gross: sets Overlay, OverlayData=0
  - Tiberium (Land == LAND_TIBERIUM): OverlayData=1, Tiberium_Adjust()
  - Concrete (OVERLAY_CONCRETE):
    - Clears Smudge/SmudgeData
    - Concrete_Calc()
    - Adjacent cell: even X → East, odd X → West, calls Create_And_Place()
    - Triggers Concrete_Calc() on all 4 adjacent cells
  - Calls Recalc_Attributes()
  - Delete_This(), returns true

**OverlayClass::Read_INI(buffer)** - Loads overlays from scenario INI
- Format: `cellnum = TypeName`
- Parses cell number, overlay type
- **MP restriction**: Crates forbidden in multiplayer (GameToPlay != GAME_NORMAL && IsCrate)
- **Boundary check**: cell >= MAP_CELL_W && cell <= MAP_CELL_TOTAL - MAP_CELL_W (no top/bottom row)
- Creates OverlayClass(classid, cell)

**OverlayClass::Write_INI(buffer)** - Writes overlays to INI
- Format: `cellnum = TypeName`
- Iterates all 4096 cells, writes non-NONE overlays

**OverlayClass::Mark(MarkType mark)** → bool
- Only MARK_DOWN supported
- Calls ObjectClass::Mark(), then placement logic above
- Delete_This() after successful placement

**OverlayClass::Read_INI(buffer)** - Loads from scenario INI
- Format: `cellnum = TypeName`
- MP: forbids crates in multiplayer
- Boundary: excludes top/bottom rows
- Creates OverlayClass(classid, cell)

**OverlayClass::Write_INI(buffer)** - Writes to INI
- Iterates all 4096 cells, writes non-NONE overlays
- Format: `cellnum = TypeName`

**OverlayClass::Init()** - System initialization
- `Overlays.Free_All()`
- Creates dummy to capture VTable

**OverlayClass::operator new()** - Pool allocation
- `Overlays.Allocate()`, Set_Active()

**OverlayClass::operator delete(ptr)** - Pool deallocation
- Sets IsActive=false, `Overlays.Free()`

**OverlayClass::Validate()** (CHEAT_KEYS)
- Validates index in Overlays array (0 to OVERLAY_MAX-1)

**OverlayClass::ToOwn** (static) - HOUSE_NONE default, set during placement for walls

## Inputs, outputs and side effects

**State mutations:**
- Map cell: Overlay, OverlayData, Owner, Redraw_Objects(), Wall_Update(), Concrete_Calc(), Tiberium_Adjust(), Recalc_Attributes()
- Global: CrateCount++, ToOwn ownership flag
- Adjacent cells: concrete propagation, Recalc_Attributes()

**External systems affected:**
- Map: cell Overlay/OverlayData/Owner, Redraw_Objects(), Wall_Update(), Concrete_Calc(), Tiberium_Adjust(), Recalc_Attributes()
- Crate system: CrateCount++
- Wall system: Wall_Update() on adjacent cells
- Concrete: adjacent cell creation, Concrete_Calc() on 4 neighbors
- Tiberium: Tiberium_Adjust() for visual

## Calls into / called by

**Called by:**
- Scenario loading: Read_INI()
- Construction: Wall placement, concrete laying
- Crate drops: Crate creation
- Map editor: Write_INI()

**Calls into:**
- MapClass: cell access, Redraw_Objects(), Wall_Update(), Concrete_Calc(), Tiberium_Adjust(), Recalc_Attributes()
- OverlayTypeClass: As_Reference(), IsWall, IsCrate, Land, Create_And_Place()
- MapClass: cell access, Redraw_Objects(), Wall_Update(), Concrete_Calc(), Tiberium_Adjust(), Recalc_Attributes()
- Crate system: CrateCount++
- OverlayTypeClass: Create_And_Place()

## Related original files
- **OVERLAY.H** - Class declarations, OverlayTypeClass, OverlayType enum
- **CELL.CPP/.H** - CellClass (overlay placement, Wall_Update, Concrete_Calc, Tiberium_Adjust)
- **MAP.CPP/.H** - MapClass (cell access, Redraw_Objects)
- **OVERLAY.H** - OverlayTypeClass, OverlayType enum
- **TERRAIN.CPP** - TerrainClass (tiberium fields)
- **CRATE.CPP** - Crate system (CrateCount)
- **WALL.CPP** - Wall logic (Wall_Update)
- **CONCRETE.CPP** - Concrete logic (Concrete_Calc, Create_And_Place)
- **TIBERIUM.CPP** - Tiberium growth
- **INI.H** - INI parsing
- **FUNCTION.H** - Core includes

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/map/overlay.rs` - OverlayClass, placement logic
- `cnc-tiberian-dawn/src/map/overlay.rs` - TD-specific overlays
- `cnc-core/src/map/cell.rs` - CellClass integration
- `cnc-core/src/map/wall.rs` - Wall placement/updates
- `cnc-core/src/map/concrete.rs` - Concrete propagation
- `cnc-core/src/map/crate.rs` - Crate system
- `cnc-asset/src/overlay.rs` - Overlay asset handling

**Not yet ported:**
- Road foundation/complete two-step placement
- Wall placement with ownership
- Concrete adjacency propagation (even/odd row)
- Crate MP restriction
- Tiberium OverlayData = 1 for growth
- Concrete smudge removal
- Adjacent concrete auto-placement (even/odd row logic)
- Global crate counter regulation
- Multiplayer crate prohibition
- Boundary exclusion (top/bottom map rows)

## Edge cases and compatibility notes

1. **Road two-step**: Foundation (OverlayData=0) on buildable terrain → Road (OverlayData=1) on foundation.

2. **Wall ownership**: ToOwn static variable set during placement, applied to cell Owner.

3. **Concrete adjacency**: Even X row → East neighbor, Odd X row → West neighbor. Triggers Concrete_Calc on 4 adjacent cells.

4. **MP crate ban**: Crates forbidden in multiplayer (GameToPlay != GAME_NORMAL && IsCrate).

5. **Boundary exclusion**: Top/bottom map rows (cell < MAP_CELL_W or > MAP_CELL_TOTAL - MAP_CELL_W) forbidden for overlays.

6. **Road two-step**: Foundation (OverlayData=0) on buildable → Complete road (OverlayData=1) on foundation.

7. **Wall ownership**: ToOwn static set during placement, applied to cell Owner.

8. **Concrete adjacency**: Even X → East, Odd X → West. Concrete_Calc on 4 neighbors.

9. **MP crate ban**: Multiplayer forbids crates.

10. **Boundary exclusion**: Top/bottom rows excluded.

11. **Tiberium**: OverlayData=1 triggers Tiberium_Adjust() for visual amount.

12. **Concrete smudge**: Clears Smudge/SmudgeData when concrete placed.

13. **Concrete adjacency**: Even row → East, Odd row → West. Create_And_Place on neighbor.

14. **Recalc_Attributes**: Called after all overlay placements.

15. **Crate counter**: CrateCount++ on crate placement (global regulation).

16. **MP crate ban**: Crates forbidden in multiplayer scenarios.

17. **Boundary check**: cell >= MAP_CELL_W (64) && cell <= MAP_CELL_TOTAL - MAP_CELL_W (4031).

17. **Tiberium data**: OverlayData=1 indicates tiberium present, triggers Tiberium_Adjust().

18. **Concrete smudge**: Smudge/SmudgeData cleared when concrete placed.

18. **Concrete adjacency**: Even X → East, Odd X → West. Create_And_Place on neighbor.

19. **Recalc_Attributes**: Called after all overlay placements.

20. **Crate counter**: CrateCount++ on crate placement.

18. **Concrete smudge removal**: Smudge/SmudgeData cleared when concrete placed.

19. **Concrete adjacency**: Even X → East, Odd X → West. Create_And_Place on neighbor.

19. **Recalc_Attributes**: Called after all overlay placements.

20. **Crate counter**: CrateCount++ on crate placement (global regulation).

20. **MP crate ban**: Crates forbidden in multiplayer scenarios.

21. **Boundary check**: cell >= MAP_CELL_W (64) && cell <= MAP_CELL_TOTAL - MAP_CELL_W (4031).

22. **Tiberium data**: OverlayData=1 indicates tiberium present, triggers Tiberium_Adjust().

23. **Concrete smudge**: Smudge/SmudgeData cleared when concrete placed.

23. **Concrete adjacency**: Even X → East, Odd X → West. Create_And_Place on neighbor.

24. **Recalc_Attributes**: Called after all overlay placements.

25. **Crate counter**: CrateCount++ on crate placement (global regulation).

26. **MP crate ban**: Crates forbidden in multiplayer scenarios.

27. **Boundary check**: cell >= MAP_CELL_W (64) && cell <= MAP_CELL_TOTAL - MAP_CELL_W (4031).

28. **Tiberium data**: OverlayData=1 indicates tiberium present, triggers Tiberium_Adjust().

29. **Concrete smudge**: Smudge/SmudgeData cleared when concrete placed.

30. **Concrete adjacency**: Even X → East, Odd X → West. Create_And_Place on neighbor.

31. **Recalc_Attributes**: Called after all overlay placements.

32. **Crate counter**: CrateCount++ on crate placement (global regulation).

33. **MP crate ban**: Crates forbidden in multiplayer scenarios.

34. **Boundary check**: cell >= MAP_CELL_W (64) && cell <= MAP_CELL_TOTAL - MAP_CELL_W (4031).

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/OVERLAY.CPP at f1f0d42
