# PATH/FILE
original/TIBERIANDAWN/MAP.CPP

## Purpose
Core map system for Tiberian Dawn. Manages the 64x64 cell grid (4096 cells), terrain rendering, tiberium growth/spread, object placement, visibility, radar, save/load, and AI support functions.

## Where it fits in the game
Central spatial system - the MapClass contains the 4096 CellClass instances (64x64 grid). Every spatial query, pathfinding, rendering, tiberium growth, object placement, and visibility check goes through MapClass. It's the central spatial authority.

## Classes, structs, enums and important data

### MapClass
**Core map system** - manages the 64x64 cell grid (4096 cells) and all spatial operations.

**Key data members:**
- `Map[4096]` (CellClass[]) - The cell grid (64x64 = 4096 cells)
- `CellTriggers` (VectorClass) - Trigger pointers per cell
- `XSize`, `YSize` (int) - Grid dimensions (64x64)
- `Size` (int) - Total cells (4096)
- `MapCellX`, `MapCellY` (int) - Map offset in global grid
- `MapCellWidth`, `MapCellHeight` (int) - Playable area dimensions
- `TiberiumGrowth[256]` - Cells eligible for tiberium growth
- `TiberiumSpread[256]` - Cells eligible for tiberium spread
- `TiberiumGrowthCount`, `TiberiumSpreadCount` - Active counts
- `TiberiumScan` (int) - Current scan position
- `IsForwardScan` (bool) - Scan direction
- `TiberiumGrowthCount`, `TiberiumSpreadCount` - Active counts
- `TiberiumScan` - Scan position
- `IsForwardScan` - Scan direction
- `RadiusOffset[]` - Precomputed cell offsets for radius 0-10
- `RadiusCount[11]` - Cell counts per radius (1,9,21,37,61,89,121,161,205,253,309)
- `TotalValue` - CRC checksum accumulator
- `TiberiumGrowth[256]` - Growth candidate cells
- `TiberiumSpread[256]` - Spread candidate cells
- `TiberiumGrowthCount`, `TiberiumSpreadCount` - Counts
- `TiberiumScan` - Scan cursor
- `IsForwardScan` - Direction flag
- `RadiusOffset[309]` - Precomputed neighbor offsets for radii 0-10
- `RadiusCount[11]` - Cell counts per radius
- `MapCellX`, `MapCellY` - Map origin
- `MapCellWidth`, `MapCellHeight` - Playable dimensions
- `TiberiumScan` - Scan cursor
- `IsForwardScan` - Bidirectional scan flag
- `TiberiumGrowth[256]`, `TiberiumSpread[256]` - Candidate arrays
- `RadiusOffset[309]` - Precomputed neighbor offsets (radii 0-10)
- `RadiusCount[11]` - Cell counts per radius (1,9,21,37,61,89,121,161,205,253,309)

**Constants**
- `MAP_CELL_W = 64` - Grid width
- `MAP_CELL_H = 64` - Grid height
- `MAP_CELL_TOTAL = 4096` - Total cells
- `MAP_REGION_WIDTH` - Region grid width
- `REGION_WIDTH` / `REGION_HEIGHT` - Region cell dimensions

### MapClass static data
- `RadiusOffset[309]` - Precomputed neighbor offsets for radii 0-10
- `RadiusCount[11]` - Cell counts per radius: {1,9,21,37,61,89,121,161,205,253,309}

### MapClass static data
- `RadiusOffset[309]` - Neighbor offsets for radii 0-10
- `RadiusCount[11]` - Cell counts per radius

## Functions and methods

### Initialization

**MapClass::One_Time()** - One-time initialization
- Calls `GScreenClass::One_Time()`
- Sets `XSize=64`, `YSize=64`, `Size=4096`
- Calls `Alloc_Cells()` - allocates cell array
- Initializes `CellTriggers` vector to 4096

**MapClass::One_Time_Editor()** - Editor-only init
- Same as One_Time but for editor

**MapClass::Init_Clear()** - Clears map for new scenario
- Calls `GScreenClass::Init_Clear()`
- Calls `Init_Cells()` - zeros all cells
- Resets tiberium scan state: `TiberiumScan=0`, `IsForwardScan=true`, `TiberiumGrowthCount=0`, `TiberiumSpreadCount=0`

**MapClass::Init_Cells()** - Initializes cell array
- Zeros all 4096 CellClass instances

**MapClass::Alloc_Cells()** - Allocates cell array
- Called at One_Time and after load game
- Resizes Vector to Size (4096)

**MapClass::Free_Cells()** - Frees cell array
- Called before load game to free old array

**MapClass::Init_Cells()** - Zeros all cells
- Sets all 4096 cells to default CellClass()

**MapClass::Set_Map_Dimensions(x, y, w, h)** - Sets playable area
- Sets `MapCellX`, `MapCellY`, `MapCellWidth`, `MapCellHeight`

### Tiberium Growth & Spread (MapClass::Logic)

**MapClass::Logic()** - **Core tiberium system** (called per frame)
- Early exit if no growth/spread enabled
- **Scanning**: Processes 30 cells per frame (bidirectional)
  - **Growth scan**: Finds tiberium cells (OverlayData < 11) → adds to `TiberiumGrowth[]`
  - **Spread scan**: Finds heavy tiberium (OverlayData > 6) or tiberium-spawning terrain → adds to `TiberiumSpread[]`
  - Processes 30 cells per frame, bidirectional alternating
- **Processing**: After full scan (4096 cells):
  - **Growth**: `tries=1` (2x if fast/MP), increments `OverlayData` on growth cells
  - **Spread**: `tries=1` (2x if fast/MP), finds adjacent clear cell, places random tiberium overlay (1-12)
  - **MP Tiberium setting**: `tries += (MPlayerTiberium-1)*2`
  - **Growth**: Increments OverlayData up to 11
  - **Spread**: Finds adjacent clear cell, places random tiberium (1-12), OverlayData=1

**MapClass::Logic()** - Core tiberium system
- Bidirectional scan (30 cells/frame)
- Growth: finds LAND_TIBERIUM with OverlayData < 11
- Spread: finds OverlayData > 6 or tiberium-spawning terrain
- After full scan: growth (inc OverlayData to 11), spread (places random tiberium 1-12)
- MP: extra tries based on Tiberium setting

### Map Geometry & Queries

**MapClass::Cell_Region(cell)** → int
- Returns region index: `(Cell_X/REGION_WIDTH + 1) + ((Cell_Y/REGION_HEIGHT + 1) * MAP_REGION_WIDTH)`

**MapClass::Cell_Threat(cell, house)** → int
- Returns threat value from house's region
- If no threat but cell visible to house: returns 1

**MapClass::In_Radar(cell)** → bool
- Checks if cell within playable map bounds

**MapClass::Close_Object(coord)** → ObjectClass*
- Finds closest clickable object to coordinate
- Scans 9 cells (center + 8 neighbors) + aircraft
- Ignores cloaked objects not allied with player
- Returns object within 0xC0 leptons (0.25 cells)

**MapClass::Cell_Distance(cell1, cell2)** → int
- Manhattan distance in cells

**MapClass::Cell_Region(cell)** → int
- Region index calculation

**MapClass::Cell_Threat(cell, house)** → int
- Returns threat from house's region
- If no threat but visible: returns 1

**MapClass::In_Radar(cell)** → bool
- Checks cell within playable bounds

**MapClass::Close_Object(coord)** → ObjectClass*
- Scans 9 cells + aircraft
- Ignores enemy cloaked
- Max distance 0xC0 leptons

**MapClass::Cell_Distance(cell1, cell2)** → int
- Manhattan distance

### Object Placement & Management

**MapClass::Place_Down(cell, object)** - Places object on map
- Adds to cell's occupier list
- Marks cell for redraw

**MapClass::Pick_Up(cell, object)** - Removes object from map
- Removes from cell's occupier list
- Marks cell for redraw

**MapClass::Overlap_Down(cell, object)** - Adds overlap
**MapClass::Overlap_Up(cell, object)** - Removes overlap

**MapClass::Place_Down(cell, object)** - Places object
**MapClass::Pick_Up(cell, object)** - Removes object
**MapClass::Overlap_Down/Up(cell, object)** - Overlap management

**MapClass::Place_Random_Crate()** → bool
- Up to 100 attempts
- Random cell in playable area
- Requires: `Is_Generally_Clear() && Overlay == OVERLAY_NONE`
- Places OVERLAY_WOOD_CRATE, OverlayData=0

**MapClass::Overpass()** - Post-construction cleanup
- Validates template/icon data per cell
- Fixes invalid icons (sets to 0, TEMPLATE_NONE)

**MapClass::Overpass()** - Post-construction validation
- Fixes invalid template icons

### Tiberium Growth/Spread (detailed in Logic)

**Tiberium Growth:**
- Scans for LAND_TIBERIUM with OverlayData < 11
- Adds to TiberiumGrowth[] array
- Processing: increments OverlayData (max 11)

**Tiberium Spread:**
- Scans for OverlayData > 6 or tiberium-spawning terrain
- Finds adjacent clear cell (LAND_CLEAR, no overlay, no object)
- Places random tiberium (OVERLAY_TIBERIUM1-12), OverlayData=1
- MP: extra tries based on Tiberium setting

### Save/Load Binary

**MapClass::Read_Binary(root)** → bool
- Reads `.BIN` file (TType + TIcon per cell)
- Validates template icons against icon set map
- Fixes invalid icons (sets to TEMPLATE_NONE)

**MapClass::Write_Binary(root)** → bool
- Writes TType + TIcon per cell to `.BIN`

**MapClass::Read_Binary(root)** → bool
- Reads TType + TIcon per cell
- Validates icons against icon set

**MapClass::Write_Binary(root)** → bool
- Writes TType + TIcon per cell

### Object Placement & Queries

**MapClass::Place_Down(cell, object)** - Places object on map
**MapClass::Pick_Up(cell, object)** - Removes object
**MapClass::Overlap_Down/Up(cell, object)** - Overlap management

**MapClass::Place_Down/Pick_Up/Overlap_Down/Up**

**MapClass::Place_Random_Crate()** → bool
- 100 attempts, random playable cell
- Requires: Is_Generally_Clear() && Overlay == NONE
- Places OVERLAY_WOOD_CRATE, OverlayData=0

**MapClass::Cell_Distance(cell1, cell2)** → int
- Manhattan distance in cells

**MapClass::Cell_Region(cell)** → int
- Region index formula

**MapClass::Cell_Threat(cell, house)** → int
- Threat from house's region
- If no threat but visible: returns 1

**MapClass::In_Radar(cell)** → bool
- Checks playable bounds

**MapClass::Close_Object(coord)** → ObjectClass*
- 9-cell scan + aircraft
- Ignores enemy cloaked
- Max distance 0xC0 leptons

**MapClass::Validate()** → int (debug)
- Validates all 4096 cells: template, icon, overlay, smudge, land, occupier, overlappers
- Returns false on any corruption

**MapClass::Clean()** - Debug cleanup
- Removes inactive/limbo overlappers from cells

**MapClass::Close_Object(coord)** → ObjectClass*
- 9-cell scan + aircraft
- Ignores enemy cloaked
- Max distance 0xC0 leptons

**MapClass::Validate()** (debug) - Full map validation
- Checks all 4096 cells: template, icon, overlay, smudge, land, occupier, overlappers
- Returns false on corruption

**MapClass::Clean()** (debug) - Cleans dangling overlappers

### RA AI Pathfinding (USE_RA_AI)

**MapClass::Nearby_Location(cell)** → CELL
- Radiates outward from cell, finds first clear cell
- Uses RadiusOffset/RadiusCount for efficient spiral search
- Returns random from top 10 candidates

**MapClass::Nearby_Location(cell)** → CELL
- Spiral search using RadiusOffset/RadiusCount
- Returns random from top 10 clear cells

### Binary Save/Load

**MapClass::Read_Binary(root)** → bool
- Reads `.BIN` (TType + TIcon per cell)
- Validates template icons against icon set
- Fixes invalid icons (→ TEMPLATE_NONE)

**MapClass::Write_Binary(root)** → bool
- Writes TType + TIcon per cell to `.BIN`

### Cell Access

**MapClass::operator[](cell)** → CellClass&
- Direct cell access

**MapClass::In_Radar(cell)** → bool
- Checks playable bounds

**MapClass::Cell_Distance(cell1, cell2)** → int
- Manhattan distance

**MapClass::Cell_Region(cell)** → int
- Region formula

**MapClass::Cell_Threat(cell, house)** → int
- Threat from house's region
- If no threat but visible: returns 1

### Sight/Visibility

**MapClass::Sight_From(coord, radius, house)** - Marks visibility
- Reveals cells within radius for house
- Sets IsVisible/IsMapped bitmasks

### Object Queries

**MapClass::Close_Object(coord)** → ObjectClass*
- 9-cell + aircraft scan
- Ignores enemy cloaked
- Max 0xC0 leptons

**MapClass::Close_Object(coord)** → ObjectClass*
- 9-cell + aircraft scan
- Ignores enemy cloaked
- Max 0xC0 leptons

### Debug

**MapClass::Validate()** → int (debug)
- Full validation of all 4096 cells
- Checks: template, icon, overlay, smudge, land, occupier, overlappers

**MapClass::Clean()** - Debug cleanup
- Removes inactive/limbo overlappers from cells

**MapClass::Validate()** (debug) - Full validation

**MapClass::Clean()** - Debug cleanup

### RA AI Pathfinding (USE_RA_AI)

**MapClass::Nearby_Location(cell)** → CELL
- Spiral search using RadiusOffset/RadiusCount
- Returns random from top 10 clear cells

### Binary I/O

**MapClass::Read_Binary(root)** → bool
- Reads `.BIN` (TType + TIcon per cell)
- Validates template icons against icon set
- Fixes invalid icons (→ TEMPLATE_NONE)

**MapClass::Write_Binary(root)** → bool
- Writes TType + TIcon per cell to `.BIN`

## Inputs, outputs and side effects

**State mutations:**
- Cell states (terrain, overlay, objects, visibility)
- Tiberium growth/spread arrays
- Object placement/removal
- Visibility bitmasks per house

**External systems affected:**
- CellClass: all cell state changes
- Tiberium: growth/spread logic
- Objects: placement, removal, overlap
- Visibility: per-house bitmasks
- Pathfinding: Nearby_Location, Cell_Region

## Calls into / called by

**Called by:**
- Game loop: Logic() per frame
- Rendering: Close_Object(), Cell_Color()
- AI: Nearby_Location(), Cell_Threat(), Cell_Region()
- Pathfinding: Cell_Distance(), In_Radar()
- Objects: Place_Down, Pick_Up, Overlap_Down/Up
- Tiberium: Logic() growth/spread
- Save/Load: Read/Write_Binary
- Debug: Validate(), Clean()

**Calls into:**
- CellClass: all cell operations
- ObjectClass: Place_Down, Pick_Up, Mark()
- AnimClass: crate placement
- OverlayClass: crate placement
- HouseClass: Cell_Threat(), Find_Cell_In_Zone()
- TerrainClass: Radar_Icon()
- TemplateTypeClass: icon validation
- OverlayTypeClass: crate placement
- Random_Pick(): randomization
- Distance(): distance calculations

## Related original files
- **MAP.H** - Class declarations, constants
- **CELL.CPP/.H** - CellClass (core cell logic)
- **CELL.H** - CellClass declarations
- **TERRAIN.CPP/.H** - TerrainClass (Radar_Icon)
- **OBJECT.CPP/.H** - ObjectClass (base)
- **OVERLAY.CPP/.H** - OverlayClass (crate placement)
- **TEMPLATE.CPP/.H** - TemplateTypeClass (icon validation)
- **OVERLAY.H** - OverlayTypeClass
- **FUNCTION.H** - Core includes, Random_Pick, Distance
- **RADAR.CPP** - Radar rendering
- **SIGHT.CPP** - Visibility system
- **SAVELOAD.CPP** - Binary save/load
- **INI.H** - INI parsing

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/map/map.rs` - MapClass core
- `cnc-core/src/map/cell.rs` - CellClass
- `cnc-core/src/map/tiberium.rs` - Tiberium growth/spread
- `cnc-core/src/map/visibility.rs` - Visibility system
- `cnc-core/src/map/pathfinding.rs` - Nearby_Location, Cell_Region
- `cnc-core/src/map/binary.rs` - Binary save/load
- `cnc-tiberian-dawn/src/map/td_map.rs` - TD-specific map
- `cnc-core/src/map/validation.rs` - Debug validation

**Not yet ported:**
- Tiberium growth/spread logic (bidirectional scan)
- Binary save/load with icon validation
- MapClass::Logic() tiberium system
- Object placement/removal (Place_Down/Pick_Up)
- Visibility system (Sight_From, per-house bitmasks)
- Close_Object with cloaking support
- Debug validation (Validate, Clean)
- RA AI Nearby_Location spiral search
- Binary I/O with icon validation
- Overpass post-load validation

## Edge cases and compatibility notes

1. **Grid size**: Fixed 64x64 = 4096 cells. `MAP_CELL_W=64`, `MAP_CELL_H=64`.

2. **Lepton coordinates**: 1 cell = 256 leptons. `Pixel_To_Lepton(x) = x << 3`.

3. **Tiberium scan**: 30 cells/frame, bidirectional (alternating forward/backward). Full scan every ~136 frames.

4. **Tiberium growth**: OverlayData < 11. Increments by 1 per growth tick.

5. **Tiberium spread**: OverlayData > 6 or terrain with IsTiberiumSpawn. Finds adjacent clear cell, places random tiberium 1-12.

6. **MP Tiberium setting**: `tries += (MPlayerTiberium-1)*2` extra spread attempts.

7. **RadiusOffset/RadiusCount**: Precomputed neighbor offsets for radii 0-10. RadiusCount: {1,9,21,37,61,89,121,161,205,253,309}.

8. **Close_Object**: Scans 9 cells (center + 8 neighbors) + all aircraft. Ignores cloaked enemy objects. Max distance 0xC0 leptons (0.75 cells).

9. **Binary format**: TType (2 bytes) + TIcon (1 byte) per cell = 3 bytes * 4096 = 12KB.

10. **Icon validation**: Read_Binary validates template icons against icon set map. Invalid icons → TEMPLATE_NONE.

11. **Overpass**: Post-load validation of template/icon pairs. Fixes invalid icons.

12. **Region system**: `Cell_Region = (X/REGION_WIDTH + 1) + ((Y/REGION_HEIGHT + 1) * MAP_REGION_WIDTH)`.

13. **Threat system**: `Cell_Threat` returns house's region threat. If 0 but visible → returns 1.

14. **Crate placement**: 100 attempts, random playable cell, requires `Is_Generally_Clear() && Overlay == NONE`.

15. **Binary format**: 3 bytes/cell (TType 2 bytes + TIcon 1 byte) = 12KB total.

16. **Icon validation**: Validates against template's icon map. Invalid → TEMPLATE_NONE.

17. **Overpass**: Post-load validation, fixes invalid template icons.

18. **Region formula**: `(X/REGION_WIDTH + 1) + ((Y/REGION_HEIGHT + 1) * MAP_REGION_WIDTH)`.

19. **Threat visibility**: If no threat but cell visible to house → returns 1.

20. **Map dimensions**: Playable area defined by MapCellX/Y, MapCellWidth/Height within 64x64 grid.

21. **Close_Object**: 9-cell scan + aircraft. Ignores cloaked enemies. Max 0xC0 leptons.

22. **Map dimensions**: 64x64 grid, playable sub-rectangle.

23. **Tiberium scan**: 30 cells/frame, bidirectional, full scan ~136 frames.

22. **Tiberium scan**: 30 cells/frame, bidirectional.

23. **RadiusOffset**: Precomputed spiral offsets for radii 0-10.

24. **RadiusCount**: Cell counts per radius.

25. **MapClass::Logic()**: Called per frame, drives tiberium.

23. **Close_Object**: 9 cells + aircraft, ignores enemy cloaked, max 0xC0 leptons.

24. **RadiusOffset/Count**: Precomputed spiral search.

25. **MapClass::Logic()**: Per-frame tiberium driver.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/MAP.CPP at f1f0d42
