# PATH/FILE
original/TIBERIANDAWN/TURRET.CPP

## Purpose
Turret AI and firing logic for Tiberian Dawn. Handles turret rotation, weapon firing conditions, projectile spawn coordinates, and facing calculations for all turret-equipped units (tanks, artillery, gunboats, flame tanks, SAM sites).

## Where it fits in the game
Unit combat subsystem - `TurretClass` extends `DriveClass` (mobile units). Used by all turret-equipped vehicles and structures. Called per-frame via `AI()` for rotation logic, and `Can_Fire()`/`Fire_At()` for combat.

## Classes, structs, enums and important data

### TurretClass : DriveClass
**Turret-equipped unit base** - Adds turret rotation and dual-weapon firing.

**Key data members:**
- `SecondaryFacing` (FacingTracker) - Turret facing (independent of body)
- `PrimaryFacing` (FacingTracker, inherited) - Body facing
- `Reload` (int) - MSAM reload timer
- `IsTurretLockedDown` (bool) - Turret locked to body facing
- `IsRotating` (bool) - Currently rotating
- `IsFiring` (bool) - Currently firing
- `IsSecondShot` (bool) - Flame tank alternating barrels

**Inherited from DriveClass:**
- `TarCom` (TARGET) - Attack target
- `NavCom` (TARGET) - Navigation target
- `Class` (UnitTypeClass const*) - Unit type definition

**Type-specific fields (from Class):**
- `Primary` / `Secondary` (WeaponType) - Weapon slots
- `IsTurretEquipped` (bool) - Has independent turret
- `IsLockTurret` (bool) - Turret locked to body
- `IsRadarEquipped` (bool) - Constant rotating radar
- `ROT` (int) - Rotation speed
- `TurretOffset` (int) - Gunboat barrel offset
- `Fires` (BulletType) - Projectile type per weapon

### FireErrorType enum
| Error | Meaning |
|-------|---------|
| `FIRE_OK` | Can fire |
| `FIRE_MOVING` | Unit moving, cannot fire |
| `FIRE_ROTATING` | Turret rotating, non-homing projectile |
| `FIRE_FACING` | Wrong facing |
| `FIRE_CLOAKED` | Target cloaked |

### FireDataType struct
```cpp
struct FireDataType {
    COORDINATE coord;  // Spawn coordinate
    int dist;          // Forward distance
    int lateral;       // Lateral offset (for dual barrels)
};
```

## Functions and methods

### TurretClass(classid, house)
**Constructor** - Calls `DriveClass`, sets `Reload=0`.

### ~TurretClass() / Default constructor
Empty.

### AI()
**Per-frame turret logic** - Called from unit AI loop.

1. `DriveClass::AI()`
2. **Radar rotation** (`IsRadarEquipped`):
   - `SecondaryFacing += 8` (45° per tick)
   - `Mark(MARK_CHANGE)`
3. **Turret rotation** (`IsTurretEquipped`):
   - If `IsTurretLockedDown`: `SecondaryFacing.Desired = PrimaryFacing.Current()`
   - If rotating: `Rotation_Adjust(ROT+1)`, `Mark(MARK_CHANGE)`, `IsRotating = Is_Rotating()`
   - Else if not locked down and no targets:
     - No nav target → `SecondaryFacing.Desired = PrimaryFacing.Current()`
     - Has nav target → `SecondaryFacing.Desired = Direction(NavCom)`

### Ok_To_Move(dir) → bool
**Movement gate for locked turrets.**

- If `IsLockTurret`:
  - If rotating → false
  - If facing difference → sets desired, returns false
- Else → true

### Can_Fire(target, which) → FireErrorType
**Firing legality check** (which: 0=primary, 1=secondary).

1. Base check: `DriveClass::Can_Fire()` → if not `FIRE_OK`, return
2. **Moving check**: If `!IsTurretEquipped || IsLockTurret` and `Target_Legal(NavCom)` → `FIRE_MOVING`
3. **Rotation check**: If `!IsFiring && IsRotating && !IsHoming` → `FIRE_ROTATING`
4. **Facing check**:
   - `dir = Direction(target)`
   - `diff = (IsTurretEquipped ? SecondaryFacing : PrimaryFacing).Difference(dir)`
   - `diff = ABS(diff)`
   - **Flame tank special**: If `Fires == BULLET_FLAME` and `Dir_Facing(dir) == Dir_Facing(PrimaryFacing)` → `diff = 0`
   - **Homing reduction**: If homing → `diff >>= 2` (divide by 4)
   - If `diff < 8` → `FIRE_OK`, else `FIRE_FACING`

### Fire_At(target, which) → BulletClass*
**Executes firing** if legal.

1. `Can_Fire(target, which)` → if `FIRE_OK`:
   - `bullet = DriveClass::Fire_At(target, which)`
   - **MSAM special**: If `UNIT_MSAM && Reload==0` → `Reload = 30 seconds`
2. Returns bullet or NULL

### Fire_Coord(which) → COORDINATE (legacy)
**Simple projectile spawn** - Returns coordinate only.

| Unit | Logic |
|------|-------|
| `GUNBOAT` | `Coord_Move(Center, PrimaryFacing, TurretOffset)`, dist=0x60 |
| `ARTY` | `Coord_Move(Center, DIR_N, 0x40)`, dist=0x60 |
| `FTANK` | dist=0x30 |
| `HTANK` | `Coord_Move(Center, DIR_N, 0x40)`, primary dist=0xC0, secondary dist=0x08 |
| `LTANK/MTANK` | `Coord_Move(Center, DIR_N, 0x20/0x30)`, dist=0xC0 |
| `APC/JEEP/BUGGY` | `Coord_Move(Center, DIR_N, 0x30)`, dist=0x30 |

### Fire_Data(which) → FireDataType (enhanced)
**Detailed spawn data** with lateral offset for dual barrels.

| Unit | Logic |
|------|-------|
| `GUNBOAT/ARTY` | Same as Fire_Coord |
| `FTANK` | dist=0x30, lateral=±0x20 (alternates `IsSecondShot`) |
| `HTANK` | Forward 0x40, primary dist=0xC0/lat=0x28, sec dist=0x08/lat=0x40, alternates |
| `LTANK/MTANK` | Forward 0x20/0x30, dist=0xC0 |
| `APC/JEEP/BUGGY` | Forward 0x30, dist=0x30 |

### Fire_Direction() → DirType
**Returns projectile facing.**

- **MSAM special**: Calculates elevation adjustment toward nearest horizon (E/W):
  - `diff = min(|Secondary-DIR_E|, |Secondary-DIR_W|)`
  - `adj = Fixed_To_Cardinal(|Secondary-DIR_N|, 64-diff)`
  - Returns `Secondary ± adj` based on N/S
- **Turret equipped**: `SecondaryFacing.Current()`
- **No turret**: `PrimaryFacing.Current()`

### Unlimbo(coord, dir) → bool
**Spawn initialization** - Sets `SecondaryFacing = dir` after base unlimbo.

### Debug_Dump(mono) (CHEAT_KEYS)
Shows `SecondaryFacing.Current/Desired`, `Arm` (reload), calls base.

## Unit-specific behaviors

| Unit | Type | Turret | Lock | Special |
|------|------|--------|------|---------|
| `FTANK` | Flame Tank | Yes | No | Alternating barrels, 45° flame logic |
| `HTANK` | Heavy Tank | Yes | No | Dual weapons (cannon + missile) |
| `MTANK/LTANK` | Medium/Light Tank | Yes | No | Single cannon |
| `ARTY` | Artillery | Yes | No | Fixed north offset |
| `GUNBOAT` | Gunboat | Yes | No | Primary facing offset |
| `MSAM` | SAM Site | Yes | No | Elevation calculation |
| `APC/JEEP/BUGGY` | Light vehicles | No | N/A | Body-facing only |

## Inputs, outputs and side effects

**State mutations:**
- `SecondaryFacing` (rotation)
- `Reload` (MSAM)
- `IsRotating`, `IsFiring`, `IsSecondShot`
- `Mark(MARK_CHANGE)` for render updates

**External systems affected:**
- Projectile spawning (`DriveClass::Fire_At`)
- Sound (`Sound_Effect` via base)
- Rotation rendering (`Mark`)

## Calls into / called by

**Called by:**
- Unit AI loop → `AI()`
- Movement → `Ok_To_Move()`
- Targeting computer → `Can_Fire()`, `Fire_At()`
- Spawn → `Unlimbo()`
- TarCom → `Fire_Direction()`

**Calls into:**
- `DriveClass` (base AI, fire, movement)
- `FacingTracker` (rotation, difference)
- `Direction()`, `Target_Legal()`
- `BulletTypeClass::As_Reference()` (homing check)
- `Coord_Move()`, `Dir_Facing()`, `ABS()`
- `Fixed_To_Cardinal()` (MSAM elevation)
- `Mark()` (render dirty)

## Related original files
- **TURRET.H** - Class declaration
- **DRIVE.H/.CPP** - DriveClass base
- **UNIT.H/.CPP** - Unit types
- **WEAPON.H/.CPP** - WeaponTypeClass
- **BULLET.H/.CPP** - BulletClass, BulletTypeClass
- **TARGET.H/.CPP** - Target conversion
- **COORD.H/.CPP** - Coord_Move, Direction
- **TARCOM.CPP** - Targeting computer (consumer)

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/turret.rs` - TurretClass
- `cnc-tiberian-dawn/src/unit/td_turret.rs` - TD unit specifics
- `cnc-core/src/combat/firing.rs` - Can_Fire, Fire_At

**Not yet ported:**
- Dual-facing system (Primary/Secondary FacingTracker)
- Rotation speed (`ROT`) and smooth adjustment
- Lock turret vs free turret logic
- MSAM elevation calculation (fixed-point)
- Flame tank alternating barrels (`IsSecondShot`)
- HTANK dual weapon with lateral offsets
- Gunboat primary-facing offset
- Homing projectile facing tolerance (`diff >>= 2`)
- Fire coordinate variants (simple vs enhanced)
- Radar constant rotation (`IsRadarEquipped`)
- Locked turret movement gate (`Ok_To_Move`)

## Edge cases and compatibility notes

1. **Facing tolerance**: `diff < 8` (of 256) = ~11.25° tolerance
2. **Homing bonus**: `diff >>= 2` = 4x tolerance for homing
3. **Flame tank**: `Dir_Facing` match → `diff=0` (45° sectors)
4. **MSAM elevation**: Complex fixed-point calculation for vertical aim
5. **FTank alternating**: `IsSecondShot` toggles lateral offset ±0x20
6. **HTank dual**: Primary=main gun (0xC0, 0x28 lat), Secondary=missile (0x08, 0x40 lat)
7. **Locked turret**: `Ok_To_Move` blocks movement until aligned
8. **Radar**: Constant 45°/tick rotation (`+8` of 256)
9. **Lock down**: `IsTurretLockedDown` forces turret to body facing
10. **Homing fires while rotating**: Non-homing blocked by `FIRE_ROTATING`
11. **Unlimbo sync**: Turret facing = body facing on spawn
12. **Fire_Data vs Fire_Coord**: Enhanced version returns lateral offset for dual barrels

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TURRET.CPP at f1f0d42
