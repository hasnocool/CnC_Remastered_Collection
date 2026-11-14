# PATH/FILE
original/TIBERIANDAWN/FLY.CPP

## Purpose
Flying object physics for Tiberian Dawn. Implements vector-based movement for aircraft and projectiles using fixed-point speed accumulation with pixel-lepton conversion. Core to airborne unit and projectile motion in the game engine.

## Where it fits in the game
Movement physics - `FlyClass::Physics()` handles per-tick movement for all flying objects (aircraft, projectiles, missiles). Called from `LOGIC.CPP` game loop. Integrated with `Coord_Move()` for coordinate transformation and `Map` edge collision detection. Simplified in 6/5/95 update to just movement (removed complex physics).

## Classes, structs, enums and important data

### FlyClass
**Flying object movement controller** - Vector physics for airborne objects.

**Key data members:**
| Field | Type | Purpose |
|-------|------|---------|
| `SpeedAdd` (MPHType) | Base speed addition per tick |
| `SpeedAccum` (int) | Fixed-point remainder accumulator |

### ImpactType enum
| Constant | Purpose |
|----------|---------|
| `IMPACT_NONE` | No movement occurred |
| `IMPACT_NORMAL` | Normal successful movement |
| `IMPACT_EDGE` | Movement hit world edge |

### MPHType
Speed type used for `SpeedAdd` field.

## Functions and methods

### FlyClass::Physics(coord, facing) → ImpactType
**Per-tick vector physics movement:**

1. Check `SpeedAdd != MPH_IMMOBILE` - skip if immobile
2. Compute `actual = (int)SpeedAdd + SpeedAccum`
3. Fixed-point division: `div(actual, PIXEL_LEPTON_W)`:
   - `SpeedAccum = result.rem` (save remainder for next tick)
   - `actual -= result.rem` (remove fractional part)
4. Save old coordinate: `old = coord`
5. If `result.quot > 0` (at least 1 pixel movement):
   - `newcoord = Coord_Move(coord, facing, actual)` - move in facing direction
   - If `newcoord == coord`: return `IMPACT_NONE` (no actual movement)
   - `coord = newcoord` - update position
   - Edge check: `newcoord & 0xC000C000L` - if off-world edge:
     - `coord = old` - revert
     - Return `IMPACT_EDGE`
   - Otherwise: Return `IMPACT_NORMAL`
6. Return `IMPACT_NONE` (immobile or no movement)

**Movement speed:** 0 = stop, 255 = max speed. Speed is accumulated in fixed-point with remainder carried between ticks.

### FlyClass::Fly_Speed(speed, maximum) → void
**Set flying object speed (throttle):**

1. `SpeedAdd = (MPHType)Fixed_To_Cardinal((int)maximum, speed)`
   - Converts `speed` (0-255) as percentage of `maximum` to fixed-point speed
   - 0 = immobile, 255 = maximum speed

## Inputs, outputs and side effects

**State mutations:**
- `SpeedAccum` - accumulates fractional movement between ticks
- `coord` parameter - updated in-place if movement occurs
- `SpeedAdd` - set by `Fly_Speed()`

**External systems affected:**
- `LOGIC.CPP` → calls `Physics()` per game tick per flying object
- `Map` → edge collision detection via `IMPACT_EDGE`
- `Coord_Move()` → coordinate transformation based on facing

## Calls into / called by

**Called by:**
- `LOGIC.CPP` → `FlyClass::Physics()` per frame for each flying object
- Unit/projectile movement systems → `Fly_Speed()` to set speed

**Calls into:**
- `Coord_Move(coord, facing, actual)` → coordinate transformation
- `div()` → fixed-point division
- `Fixed_To_Cardinal()` → speed conversion

## Related original files
- **FLY.H** - Class declarations
- **LOGIC.CPP** → per-frame physics calls
- **VECTOR.CPP** → `Coord_Move()` coordinate math
- **MAP.CPP** → edge collision detection

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/movement/fly.rs` - FlyClass Physics and Fly_Speed
- `cnc-tiberian-dawn/src/movement/vector.rs` - Coord_Move vector math

**Not yet ported:**
- Fixed-point speed accumulation (`SpeedAccum` remainder)
- `PIXEL_LEPTON_W` pixel-to-lepton conversion
- Edge-of-world collision detection
- MPHType speed system
- `Fixed_To_Cardinal()` speed conversion

## Edge cases and compatibility notes

1. **Fixed-point remainder**: `SpeedAccum` carries fractional movement between ticks for smooth motion
2. **Immobile check**: `SpeedAdd == MPH_IMMOBILE` skips all movement
3. **Edge collision**: `0xC000C000L` mask detects off-world coordinates
4. **Coordinate revert**: On edge hit, coordinate reverts to old position
5. **No movement case**: If `Coord_Move` returns same coord, `IMPACT_NONE` returned
6. **Simplified 6/5/95**: Original complex physics removed, now just movement
7. **Speed range**: 0-255 throttle mapped to fixed-point via `Fixed_To_Cardinal()`

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/FLY.CPP at f1f0d42
