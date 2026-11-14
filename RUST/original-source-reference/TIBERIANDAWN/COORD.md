# PATH/FILE
original/TIBERIANDAWN/COORD.CPP

## Purpose
Coordinate math utilities for Tiberian Dawn. Provides high-performance fixed-point trigonometry, cell spillage/overlap calculation, coordinate movement, and random scatter - all optimized for frequent per-frame calls.

## Where it fits in the game
Core math library - used by unit movement, weapon targeting, building placement, map rendering, and AI pathfinding. All coordinate operations use 16.16 fixed-point (leptons) with 256-entry sin/cos tables.

## Classes, structs, enums and important data

### Coordinate system
- **COORDINATE**: 32-bit packed (X:16 bits, Y:16 bits) in leptons (1/256 pixel)
- **Lepton**: 1/256 pixel precision
- **Cell**: 32×32 pixels (ICON_PIXEL_W/H = 32)
- **MAP_CELL_W**: Cell stride in cell array

### Key constants
| Constant | Value | Purpose |
|----------|-------|---------|
| `ICON_PIXEL_W/H` | 32 | Cell pixel dimensions |
| `MAP_CELL_W` | Map width in cells | Cell array stride |
| `REFRESH_EOL` | -1 | End-of-list sentinel |
| `FACING_COUNT` | 8/256 | Direction count (8 cardinal, 256 fine) |

### Sin/Cos tables (256 entries each)
- `CosTable[256]` / `SinTable[256]` - Unsigned bytes, treated as signed (-128..127)
- Index = `DirType` (0-255 for fine, 0-7 for cardinal × 32)
- Values: 0x7F = 1.0, 0x00 = 0.0, 0x80 = -1.0 (two's complement)

### Spillage tables
- `_MoveSpillage[9][5]` - Precomputed cell offsets per 8 directions + stationary
- `_SpillTable[16]` - 4-bit index → spillage table selector
- `_manual[10]` - Dynamic list for large objects
- `_gigundo[26]` - 5×5 cell region for huge objects (>64px)

## Functions and methods

### Coord_Spillage_List(coord, maxsize) → short const*
**Returns cell offset list** for object overlap rendering. First entry always 0 (center cell).

**Algorithm by size:**
1. **Gigantic (>64px)**: Returns `_gigundo` (5×5 = 25 cells)
2. **Large (>32px)**: Builds `_manual` dynamically:
   - Calculates lepton position in cell
   - Checks 4 edges + 4 corners against `maxsize/2` radius
   - Adds offsets: 0, ±1, ±MAP_CELL_W, ±(MAP_CELL_W±1)
3. **Normal (≤32px)**: Uses `_MoveSpillage` via `_SpillTable`:
   - Computes 4-bit index from lepton position vs `posval = Pixel2Lepton[(32-maxsize)/2]`
   - Bits: 8=South, 4=North, 2=East, 1=West
   - `_SpillTable[index]` selects precomputed directional list

### Coord_Move(start, dir, distance) → COORDINATE
**Moves coordinate by distance in direction** using fixed-point trig.

```c
x = Coord_X(start); y = Coord_Y(start);
Move_Point(x, y, dir, distance);
return XY_Coord(x, y);
```

### Move_Point(x, y, dir, distance)
**Core trigonometry** - Updates x,y by reference.

```
cos = CosTable[dir] (signed char)
sin = SinTable[dir] (signed char)
x += (cos * distance * 2) >> 16  // Fixed-point multiply
y -= (sin * distance * 2) >> 16  // Negated (inverted Y)
```

**Implementation**: Uses `calcx`/`calcy` inline asm (MSVC) or `imul`+`shl`+`rcl` sequence:
1. `imul` distance (16×16→32)
2. `shl ax,1` / `rcl dx,1` (×2, 32-bit)
3. `mov al,ah` / `mov ah,dl` (extract high word = /65536)
4. Result in AX (signed 16-bit)

### Coord_Scatter(coord, distance, lock) → COORDINATE
**Random offset** from anchor point.

1. `Coord_Move(coord, Random_Pick(DIR_N, DIR_MAX), distance)`
2. Bounds check: if `newcoord & 0xC000C000` (sign bits) → return original
3. If `lock` → `Coord_Snap(newcoord)` (cell center)

### Cardinal_To_Fixed(base, cardinal) → unsigned (OBSOLETE)
**Fixed-point fraction** = `cardinal / base` × 256.
- Implemented in `COORDA.ASM` (assembly)
- Uses 32-bit dividend (cardinal << 8) / base

### Fixed_To_Cardinal(base, fixed) → unsigned (OBSOLETE)
**Inverse**: `base * fixed / 256` with rounding.
- `(base * fixed + 0x80) >> 8`
- Assembly in `COORDA.ASM`

### Fixed_To_Cardinal (inline, used by Coord_Spillage_List)
```c
unsigned Fixed_To_Cardinal(unsigned base, unsigned fixed) {
    unsigned long temp = (unsigned long)base * fixed + 0x80;
    if (temp & 0xFF000000) return 0xFFFF;  // Overflow
    return (unsigned)(temp >> 8);
}
```

## Inputs, outputs and side effects

**Pure functions** - No global state mutation (except static buffers for return pointers).

**Static buffers** (not thread-safe):
- `_manual[10]` - Large object spillage
- `_gigundo[26]` - Huge object spillage
- Return pointers valid until next call

## Calls into / called by

**Called by:**
- Unit movement (`UnitClass::Move`)
- Weapon targeting (`Coord_Scatter` for missile spread)
- Building placement (overlap checks)
- Map rendering (cell refresh lists)
- AI pathfinding (spillage for obstruction)
- Animation positioning

**Calls into:**
- `Random_Pick` (random direction)
- `Coord_Snap` (cell center alignment)
- `Pixel2Lepton` table (lepton thresholds)
- `CosTable`/`SinTable` (trig)

## Related original files
- **COORD.H** - Declarations, COORDINATE macros
- **COORDA.ASM** - Assembly fixed-point math
- **FUNCTION.H** - Common types, `DirType`, `FACING_COUNT`
- **MAP.H** - `MAP_CELL_W`, `CellClass`
- **UNIT.H/.CPP** - Unit movement
- **WEAPON.H/.CPP** - Projectile scatter
- **BUILDING.H/.CPP** - Placement overlap

## Rust port mapping
**Crates:** `cnc-core`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-core/src/math/coord.rs` - COORDINATE, lepton math
- `cnc-core/src/math/trig.rs` - Sin/Cos tables, Move_Point
- `cnc-core/src/math/spillage.rs` - Coord_Spillage_List
- `cnc-tiberian-dawn/src/math/td_coord.rs` - TD-specific constants

**Not yet ported:**
- 256-entry sin/cos tables with signed byte interpretation
- Fixed-point multiply sequence (imul+shl+rcl → high word)
- Spillage list tiering (gigantic/large/normal)
- `_SpillTable` 4-bit index mapping
- `Pixel2Lepton` threshold table
- Static buffer returns (need owned `Vec` or `SmallVec`)
- `Coord_Snap` (cell center alignment)
- Assembly `COORDA.ASM` routines (Cardinal_To_Fixed, Fixed_To_Cardinal)

## Edge cases and compatibility notes

1. **Lepton coordinate**: 16.16 fixed-point; X=low word, Y=high word of 32-bit
2. **Sin table inverted**: Y uses `neg` because screen Y increases downward
3. **Distance units**: Leptons (1/256 pixel) for `Coord_Move`; pixels for `Coord_Scatter`
4. **Direction encoding**: 0-255 fine (256 = 360°); 8 cardinal = 0,32,64,96,128,160,192,224
5. **Spillage bounds**: `maxsize > 48` → incomplete list (warned in header)
6. **Gigantic threshold**: `> ICON_PIXEL_W * 2` (64px) → 5×5 fixed pattern
7. **Large threshold**: `> ICON_PIXEL_W` (32px) → manual edge/corner check
8. **Normal path**: 4-bit spill index from lepton position vs `posval`
9. **posval**: `Pixel2Lepton[(32-maxsize)/2]` - half the slack space in leptons
10. **Spill table index**: North=4, South=8, East=2, West=1 (bitmask)
11. **Static buffers**: Not reentrant; single-threaded game loop OK
12. **REFRESH_EOL**: -1 terminates offset lists
13. **Coord_Scatter bounds**: `0xC000C000` checks sign bits of both X and Y
14. **Move_Point asm**: MSVC inline asm (`calcx`/`calcy`) or Watcom-style
15. **OBSOLETE functions**: Moved to `COORDA.ASM`; C versions kept for reference

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/COORD.CPP at f1f0d42
