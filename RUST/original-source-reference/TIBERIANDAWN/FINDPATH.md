# PATH/FILE
original/TIBERIANDAWN/FINDPATH.CPP

## Purpose
Pathfinding system for Tiberian Dawn. Implements A*-like pathfinding with edge-following around obstacles, diagonal movement support, loop detection/prevention, and threat-aware routing for all ground/air units.

## Where it fits in the game
Core pathfinding system - used by all mobile units (infantry, vehicles, aircraft) for navigation. Called from FootClass::Find_Path() and used by all mobile unit AI for movement.

## Classes, structs, enums and important data

### PathType
**Path data structure** - stores computed path for a unit.

**Key fields:**
- `Start` (CELL) - Starting cell
- `Cost` (int) - Total path cost
- `Length` (int) - Number of commands
- `Command[MAX_MLIST_SIZE]` (FacingType[]) - Movement commands
- `Overlap[MAP_CELL_TOTAL/32]` - Bitmask of visited cells
- `LastOverlap` (CELL) - Last overlap cell for loop detection
- `LastFixup` (CELL) - Last fixup position for loop unraveling

### Constants
- `CLOCK` = 1 (clockwise)
- `COUNTERCLOCK` = -1 (counterclockwise)
- `DIAGONAL` - Diagonal movement enabled
- `END` = FACING_NONE (path terminator)
- `SAVEMEM` - Memory vs speed tradeoff
- `CELL_FACING(a,b)` - Direction from cell a to b
- `MODULO` = MAP_CELL_W (64)
- `MAX_MLIST_SIZE` = 300 (max path commands)
- `THREAT_THRESHOLD` = 5
- `MAX_PATH_EDGE_FOLLOW` = 400
- `MAX_MLIST_SIZE` = 300

### Static data
- `MainOverlap[MAP_CELL_TOTAL/32]` - Main path overlap bitmask
- `LeftOverlap[MAP_CELL_TOTAL/32]` - Left edge path overlap
- `RightOverlap[MAP_CELL_TOTAL/32]` - Right edge path overlap
- `DestLocation` - Destination cell
- `StartLocation` - Start cell

## Functions and methods

### Find_Path(source, dest, threat, threshhold, flags) → PathType*
**Main pathfinding entry point** - finds path from source to destination.

**Algorithm:**
1. **Initialize path**: Clear overlap bits, set start cell, max length
2. **Straight-line scan**: Move toward destination via `CELL_FACING`
3. **Obstacle handling**: When blocked:
   - Check if destination reached
   - Try edge following both directions (clockwise/counter-clockwise)
   - Take shorter valid path
3. **Threat handling**: Increases threat tolerance in stages (0→half→full→ignore)
4. **Doughnut handling**: Scans forward through impassable "doughnuts"
5. **Optimization**: `Optimize_Moves()` removes redundant commands

**Threat stages:**
1. Stage 0: `threat = unit_threat >> 1` (half)
2. Stage 1: `threat += unit_threat` (full)
3. Stage 2: `threat = -1` (ignore threats)

**Doughnut handling:** Scans forward through impassable ring to find exit

### Follow_Edge(start, target, path, search, olddir, threat, threat_stage, max_cells, threshhold) → bool
**Edge-following algorithm** - navigates around obstacles.

**Algorithm:**
1. Initializes search direction based on `search` (clockwise/counterclockwise)
2. **Edge tracing loop:**
   - Rotates search direction
   - Checks diagonal corners for destination
   - Validates passable cells with `Passable_Cell()`
   - Handles diagonal cross-line detection
   - Tracks online/offline state relative to start-dest line
   - Prevents infinite loops with `MAX_PATH_EDGE_FOLLOW`
4. Returns true if target reached

**Key features:**
- Clockwise/counter-clockwise edge following
- Diagonal movement with corner-cutting checks
- Line-crossing detection via `Point_Relative_To_Line`
- Loop prevention with `MAX_PATH_EDGE_FOLLOW`

### Register_Cell(path, cell, dir, cost, threshhold) → bool
**Registers cell in path** with overlap detection.

**Returns false if:**
- Cell already in path (loop detected) → triggers `Unravel_Loop()`
- Immediate backtrack → removes last command

**Overlap tracking:** Bitmask per cell in `path->Overlap[]`

### Unravel_Loop(path, cell, dir, sx, sy, dx, dy, threshhold) → bool
**Loop detection and resolution** - fixes path loops caused by diagonal movement.

**Algorithm:**
1. Walks backward through path
2. Finds point on straight line to destination
3. Fixes diagonal corner-cutting
4. Truncates path at fixup point

### Register_Cell (overlap version)
**Overlap tracking** - sets bit in `path->Overlap[]` bitmask

### Follow_Edge
**Edge following** with clockwise/counter-clockwise search:
- Tracks online/offline state relative to start-dest line
- Handles diagonal corner cases
- Prevents immediate backtracking
- Limits to `MAX_PATH_EDGE_FOLLOW` steps

### Optimize_Moves(path, threshhold)
**Path optimization** - removes redundant commands.

**Optimizations:**
- **Backtrack elimination**: Removes opposing pairs (3 = backtrack)
- **Diagonal smoothing**: 45° optimizations if intermediate cell passable
- **90° smoothing**: Right-angle turns → diagonal if intermediate cell passable
- **135° optimizations**: Always allowed
- **Packing**: Removes EMPTY commands

**Optimization table `_trans[]`:**
- Maps facing difference → optimization code
- 0 = no optimization, 3 = backtrack elimination
- Others = direction adjustment

### FootClass::Find_Path
**High-level pathfinder** for FootClass units.

**Process:**
1. Initializes `PathType` with start/dest
2. Calls `Find_Path()` algorithm
3. Returns optimized `PathType*`

### FootClass::Follow_Edge
**Edge following** for unit movement around obstacles.

### FootClass::Unravel_Loop
**Loop unraveling** for diagonal movement loops.

### FootClass::Register_Cell
**Cell registration** with overlap detection and loop prevention.

### FootClass::Register_Cell (overlap version)
**Overlap bitmask registration** with loop detection.

### FootClass::Unravel_Loop
**Loop resolution** for diagonal movement loops.

### FootClass::Register_Cell (overlap)
**Overlap bitmask** management.

### FootClass::Follow_Edge (detailed)
**Edge follower** with clockwise/counter-clockwise search.

### FootClass::Optimize_Moves
**Path optimization** with diagonal/orthogonal smoothing.

### FootClass::Safety_Point
**Finds safe cell** near destination for deployment.

### FootClass::Passable_Cell
**Cell passability check** with threat thresholds.

### Debug functions
- `Debug_Draw_Map()` - Visualizes pathfinding
- `Debug_Draw_Path()` - Draws computed path
- `Debug_Draw_Map()` - Map visualization
- `Debug_Draw_Path()` - Path visualization

## Inputs, outputs and side effects

**Inputs:**
- Source/destination cells
- Threat level and threshold
- Unit type (movement capabilities)
- Debug flags

**Outputs:**
- `PathType*` with command sequence
- Optimized command list

**Side effects:**
- Map overlap bitmaps updated
- Debug visualization (if enabled)
- Path overlap bitmaps modified

## Calls into / called by

**Called by:**
- FootClass::AI() - unit movement
- UnitClass::AI() - vehicle movement
- InfantryClass::AI() - infantry movement
- Any mobile unit movement

**Calls into:**
- MapClass: Cell operations, passability checks
- CellClass: Occupancy, terrain
- FootClass: Passable_Cell, Register_Cell, Unravel_Loop
- Optimize_Moves
- Follow_Edge
- Debug_Draw_Map/Path

## Related original files
- **FINDPATH.H** - Class declarations, PathType
- **FOOT.CPP/.H** - FootClass (base movement)
- **UNIT.CPP/.H** - UnitClass (vehicles)
- **INFANTRY.CPP/.H** - InfantryClass
- **AIRCRAFT.CPP/.H** - AircraftClass (different pathfinding)
- **MAP.CPP/.H** - MapClass (cell operations)
- **CELL.CPP/.H** - CellClass (cell operations)
- **FUNCTION.H** - Core includes, Distance, Random_Pick

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/pathfinding.rs` - Core pathfinding
- `cnc-tiberian-dawn/src/ai/td_pathfinding.rs` - TD-specific
- `cnc-core/src/ai/edge_following.rs` - Edge following
- `cnc-core/src/ai/optimization.rs` - Path optimization
- `cnc-core/src/ai/loop_detection.rs` - Loop detection

**Not yet ported:**
- Diagonal movement with corner cutting
- Edge-following with clockwise/counter-clockwise
- Loop detection and unraveling
- Path optimization (backtrack, diagonal smoothing)
- Threat-aware pathfinding stages
- Doughnut hole handling
- Debug visualization
- Memory-efficient path storage (bitmask overlap)

## Edge cases and compatibility notes

1. **Diagonal movement**: Corner-cutting allowed if intermediate cells passable
2. **Loop detection**: Overlap bitmask prevents infinite loops
3. **Loop unraveling**: Fixes diagonal corner-cutting loops
3. **Loop unraveling**: Fixes diagonal corner-cutting loops
4. **Threat stages**: 3-stage escalation (half→full→ignore)
5. **Doughnut handling**: Scans through impassable rings
6. **Edge following**: Bidirectional with clockwise/counter-clockwise
7. **Optimization**: Backtrack removal, diagonal/90°/135° smoothing
8. **Memory vs speed**: `SAVEMEM` controls intermediate optimizations
8. **SAVEMEM**: Trades memory for speed
9. **Path limits**: `MAX_MLIST_SIZE=300`, `MAX_PATH_EDGE_FOLLOW=400`
9. **Limits**: 300 commands, 400 edge steps
10. **Overlap bitmask**: 32-bit per cell, 4096 cells = 128 longs
10. **Overlap tracking**: 4096 cells → 128 32-bit integers
11. **Diagonal corner cutting**: Checks intermediate cells for 90° turns
11. **Diagonal corner cutting**: Checks adjacent cells
12. **Threat escalation**: 3-stage (half → full → ignore)
12. **Threat stages**: 3-level escalation
13. **Doughnut handling**: Scans through impassable rings
13. **Doughnut handling**: Scans through impassable rings
14. **Max path length**: 300 commands, edge follow 400 steps
14. **Limits**: 300 commands, 400 edge steps
15. **Debug visualization**: `Draw_Cell_Point`, `Debug_Draw_Map`, `Debug_Draw_Path`
15. **Debug viz**: Cell coloring, path drawing

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/FINDPATH.CPP at f1f0d42
