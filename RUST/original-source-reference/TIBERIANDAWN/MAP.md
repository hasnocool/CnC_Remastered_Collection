# PATH/FILE
original/TIBERIANDAWN/MAP.CPP

## Purpose
Map management system for Tiberian Dawn. Handles the cell grid, terrain/overlay/template data, object placement, tiberium growth/spread, shroud/visibility, binary save/load, and tactical map coordination.

## Where it fits in the game
Core world representation - Central `MapClass Map` global (GLOBALS.CPP). Manages the 2D cell grid, all spatial queries, object placement/removal, tiberium simulation, and tactical/radar map coordination.

## Classes, structs, enums and important data

### MapClass : GScreenClass
**World map controller** - Global `Map` instance.

**Key data members:**
| Variable | Purpose |
|----------|---------|
| `MapCellX/Y` | Map origin in global cell grid |
| `MapCellWidth/Height` | Active map dimensions |
| `Size` | Total cells (Width×Height) |
| `TiberiumScan` | Scan position for tiberium processing |
| `IsForwardScan` | Scan direction toggle |
| `TiberiumGrowth[]/Count` | Cells queued for growth |
| `TiberiumSpread[]/Count` | Cells queued for spread |
| `TiberiumGrowthCount/SpreadCount` | Queue sizes |
| `TotalValue` | Map value sum (CRC) |
| `CellTriggers` | Trigger list per cell |
| `BlubCell` | Debug cell pointer |

### CellClass (per cell)
```cpp
struct CellClass {
    TemplateType TType;     // Template type
    unsigned char TIcon;    // Template icon index
    OverlayType Overlay;    // Overlay type
    unsigned char OverlayData; // Overlay state (tiberium amount)
    SmudgeType Smudge;      // Scorch/crater decal
    ObjectClass* Cell_Occupier() // Ground object
    ObjectClass* Overlapper[3] // Air/overlap objects
    Land_Type() -> LandType
    Adjacent_Cell(facing) -> CELL
    Is_Clear_To_Move() -> bool
    Redraw_Objects()
    Recalc_Attributes()
    Is_Visible(house) -> bool
};
```

### Constants
| Constant | Value |
|----------|-------|
| `MAP_CELL_W` | Map width in cells |
| `MAP_CELL_H` | Map height in cells |
| `MAP_CELL_TOTAL` | Total cells |
| `REGION_WIDTH/HEIGHT` | Threat region size |
| `MAP_REGION_WIDTH` | Regions across |
| `RADIUS_COUNT[11]` | Cell counts per radius 0-10 |
| `RadiusOffset[]` | Precomputed cell offsets for radii 0-10 |
| `POWER_STEP_LEVEL` | Power per "hundred" |
| `POWER_STEP_FACTOR` | Diminishing returns factor |

### RadiusOffset table
Precomputed cell offset arrays for radii 0-10:
- Radius 0: 1 cell (0)
- Radius 1: 9 cells (center + 8 neighbors)
- Radius 2: 21 cells
- Radius 10: 309 cells

## Functions and methods

### One_Time() / One_Time_Editor()
**One-time initialization:**
1. `GScreenClass::One_Time()`
2. `XSize=MAP_CELL_W`, `YSize=MAP_CELL_H`, `Size=XSize*YSize`
3. `Alloc_Cells()` → `Resize(Size)`
4. `CellTriggers.Resize(MAP_CELL_TOTAL)`

### Alloc_Cells() / Free_Cells() / Init_Cells()
**Cell array management:**
- `Alloc_Cells()`: `Resize(Size)` (VectorClass)
- `Free_Cells()`: `Clear()`
- `Init_Cells()`: Default-constructs all `CellClass` elements

### Init_Clear()
**Scenario reset:**
1. `GScreenClass::Init_Clear()`
2. `Init_Cells()`
3. `TiberiumScan=0`, `IsForwardScan=true`
4. `TiberiumGrowthCount=0`, `TiberiumSpreadCount=0`

### Set_Map_Dimensions(x, y, w, h)
**Sets active map rectangle** - Updates `MapCellX/Y/Width/Height`

### Sight_From(house, cell, radius, reveal)
**Fog of war reveal** - Marks cells visible for house within radius

### Read_Binary(root) / Write_Binary(root)
**Binary map I/O** (`.BIN` files):
- Per-cell: `TemplateType` (2 bytes) + `TIcon` (1 byte)
- CRC accumulation (non-DEMO)
- Returns bool success

### Logic()
**Per-frame map logic** - Tiberium growth/spread simulation.

**Process:**
1. Early exit if `!IsTGrowth && !IsTSpread`
2. **Scan phase** (30 cells/frame, alternating direction):
   - Scans `TiberiumScan` forward/backward
   - **Growth candidates**: `Land_Type()==TIBERIUM && OverlayData<11` → `TiberiumGrowth[]`
   - **Spread candidates**: `OverlayData>6` or `terrain->IsTiberiumSpawn` → `TiberiumSpread[]` (×3 tries if terrain)
2. **Processing phase** (when scan completes):
   - `tries = 1` (×2 if `IsTFast` or MP; +2×(tiberium-1) in GlyphX MP)
   - **Growth**: Random pick from `TiberiumGrowth[]`, `OverlayData++` (max 11)
   - **Spread**: Random pick from `TiberiumSpread[]`, finds random adjacent clear cell, places `OVERLAY_TIBERIUM1..12` with `OverlayData=1`
   - Avoids bridges, requires `LAND_CLEAR` and `OVERLAY_NONE`

### Cell_Region(cell) → int
**Threat region index** - `(X/REGION_WIDTH+1) + (Y/REGION_HEIGHT+1)*MAP_REGION_WIDTH`

### Cell_Threat(cell, house) → int
**Threat value** - From `House->Regions[region].Threat_Value()`, minimum 1 if visible

### Place_Random_Crate() → bool
**Random crate placement** - 100 attempts, finds clear cell, places `OVERLAY_WOOD_CRATE`

### Validate() → bool
**Debug map validation** (CHEAT_KEYS):
- Template/Icon bounds
- Overlay/Smudge/Land bounds
- Occupier/Overlapper validity (active, not limbo, coord bounds)
- Returns false on first error

### Clean()
**Dangling pointer cleanup** (debug):
- Clears inactive/limbo overlappers
- Logs cleaned objects via `GlyphX_Debug_Print`

### Close_Object(coord) → ObjectClass*
**Nearest clickable object** - Scans 9-cell neighborhood + aircraft
- Ground: 9-cell pattern, distance check (buildings ≤0xC0, units exact)
- Ignores enemy cloaked
- Aircraft: Projects to ground (altitude adjusted)
- Max distance 0xC0 (1/4 cell)

### Place_Random_Crate() → bool
**Crate spawning** - 100 random attempts, clear cell → `OVERLAY_WOOD_CRATE`

### Place_Down(obj) / Pick_Up(obj)
**Object placement/removal** - Updates cell Occupier/Overlapper arrays

### Overlap_Up(obj) / Overlap_Down(obj)
**Overlap tracking** - Adds/removes from cell's `Overlapper[3]` array

### Overpass()
**Post-construction cleanup** - Validates template icons, recalculates attributes

### Sight_From(house, cell, radius, reveal)
**Fog reveal** - Iterates `RadiusOffset[radius]`, marks cells visible

### Cell_Distance(cell1, cell2) → int
**Cell distance** - Chebyshev max(|dx|,|dy|)

### In_Radar(cell) → bool
**Radar visibility** - Checks `RadarX/Y/Width/Height`

### Place_Random_Crate() → bool
**Crate placement** - 100 random attempts on clear cells

### Read_Binary(root) / Write_Binary(root)
**Binary map I/O** - Per-cell: `TemplateType` (2B) + `TIcon` (1B), CRC (non-DEMO)

### Validate() → bool
**Full map validation** (CHEAT_KEYS):
- Template/Icon bounds
- Overlay/Smudge/Land bounds
- Occupier/Overlapper: active, not limbo, coord ≤ 4095
- Debug `BlubCell` inspection

### Clean()
**Dangling pointer cleanup** - Nulls inactive/limbo overlappers, logs via `GlyphX_Debug_Print`

### Nearby_Location(cell) → CELL (USE_RA_AI)
**AI clear location finder** - Radiating box search, returns random from top 10 clear cells

## Tiberium simulation details

### Growth
- **Condition**: `LAND_TIBERIUM` + `OverlayData < 11`
- **Rate**: 1 cell per frame (×tries), `OverlayData++`
- **Max**: `OverlayData = 11` (12 = full)

### Spread
- **Condition**: `OverlayData > 6` OR `terrain->IsTiberiumSpawn`
- **Rate**: 1-3 attempts per frame
- **Placement**: Random adjacent clear cell (`LAND_CLEAR`, `OVERLAY_NONE`, not bridge)
- **Result**: `OVERLAY_TIBERIUM1..12`, `OverlayData=1`

### Scan optimization
- 30 cells/frame
- Alternates forward/backward each full pass
- Maintains candidate queues (`TiberiumGrowth[]`, `TiberiumSpread[]`)
- Reservoir sampling when queues full (`Random_Pick` replacement)

## Inputs, outputs and side effects

**State mutations:**
- Cell grid (`TType`, `TIcon`, `Overlay`, `OverlayData`, `Smudge`)
- Object placement (`Occupier`, `Overlapper[]`)
- Tiberium queues (`TiberiumGrowth/Spread[]`)
- Visibility (`Sight_From`)
- Map CRC (`TotalValue`)

**External systems affected:**
- Unit movement/placement
- AI threat assessment (`Cell_Threat`)
- Shroud/fog of war
- Tiberium economy
- Crate spawning
- Save/load
- Radar/minimap rendering

## Calls into / called by

**Called by:**
- `INIT.CPP` → `One_Time()`, `Init_Clear()`
- `LOGIC.CPP` → `Logic()` per frame
- `RADAR.CPP` → `Set_Radar_Position()`, `Set_Map_Dimensions()`
- `SAVELOAD.CPP` → `Read_Binary()`, `Write_Binary()`
- `CONQUER.CPP` → `Sight_From()`, `Close_Object()`, `Place_Random_Crate()`
- Unit AI → `Close_Object()`, `Place_Down()`, `Pick_Up()`, `Overlap_Up/Down()`

**Calls into:**
- `CellClass` methods
- `TemplateTypeClass` (icon validation)
- `OverlayClass` (spread placement)
- `Random_Pick`, `Random_Pick` (reservoir sampling)
- `Bound` (clamping)
- `GlyphX_Debug_Print` (debug)
- `Add_CRC` (CRC)

## Related original files
- **MAP.H** - Class declaration, CellClass
- **CELL.H** - CellClass definition
- **TEMPLATE.H/.CPP** - TemplateTypeClass
- **OVERLAY.H/.CPP** - OverlayClass
- **TERRAIN.H/.CPP** - TerrainClass
- **SMUDGE.H/.CPP** - SmudgeClass
- **RADAR.H/.CPP** - RadarClass (tactical map sync)
- **POWER.H/.CPP** - PowerClass (inherits RadarClass)
- **SAVELOAD.H/.CPP** - Binary I/O
- **LOGIC.CPP** → `Logic()` per frame
- **RADIUS.H** - RadiusOffset/Count tables

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/map.rs` - MapClass, CellClass
- `cnc-core/src/map/tiberium.rs` - Growth/spread logic
- `cnc-core/src/map/fog.rs` - Sight_From, visibility
- `cnc-tiberian-dawn/src/map/td_map.rs` - TD-specific constants
- `cnc-core/src/map/binary.rs` - Binary I/O

**Not yet ported:**
- RadiusOffset/Count tables (precomputed)
- Reservoir sampling for candidate queues
- Bidirectional scan with frame budget
- Tiberium diminishing returns (OverlayData 1-11)
- Bridge avoidance in spread
- Blossom tree weighting (terrain->IsTiberiumSpawn)
- MP tiberium rate multiplier
- Dangling pointer cleanup (debug)
- Binary CRC validation
- Map validation (CHEAT_KEYS)
- Debug BlubCell inspection

## Edge cases and compatibility notes

1. **Bidirectional scan**: Alternates `IsForwardScan` each full pass for fairness
2. **Reservoir sampling**: When queue full, `Random_Pick(0, Count-1)` replaces random entry
3. **Tiberium cap**: `OverlayData` max 11 (12 = full, no growth)
4. **Spread tries**: 1 normally, 3 if `terrain->IsTiberiumSpawn` (blossom trees)
4. **MP rate**: `tries ×2` if `IsTFast` or MP; GlyphX MP adds `2×(tiberium-1)`
5. **Spread placement**: Avoids bridges, requires `LAND_CLEAR` + `OVERLAY_NONE`
6. **Random adjacent**: `Random_Pick(FACING_N, FACING_NW)` + sequential scan
7. **Bridge check**: Explicit `TEMPLATE_BRIDGE1..4` exclusion
8. **Crate placement**: 100 attempts max, `OVERLAY_WOOD_CRATE`
9. **Binary format**: 3 bytes/cell (2+1), CRC for MP sync
10. **Validation**: Checks all cells including off-map (memory corruption detection)
11. **Clean()**: Removes dangling overlappers (inactive/limbo)
12. **Close_Object**: 9-cell ground + aircraft projection, max 0xC0 distance
13. **Sight_From**: Uses `RadiusOffset[radius]` precomputed table
14. **Region threat**: `Cell_Region` → `House->Regions[].Threat_Value()`

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/MAP.CPP at f1f0d42
