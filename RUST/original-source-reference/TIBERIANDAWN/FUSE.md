# PATH/FILE
original/TIBERIANDAWN/FUSE.CPP

## Purpose
Fuse/proximity detonation system for Tiberian Dawn. Implements missile/bullet fuses with proximity detection, minimum arming time, and maximum lifetime. Used by projectiles that detonate when near a target or when time expires. Core to weapon explosion timing.

## Where it fits in the game
Weapon systems - `FuseClass` is embedded in bullet/missile objects for proximity detonation. Called from `BULLET.CPP` per-tick to check if a fuse should trigger. Integrated with `WEAPON.CPP` warhead system and `COMBAT.CPP` damage pipeline. Save/load support via `Fuse_Write()`/`Fuse_Read()`.

## Classes, structs, enums and important data

### FuseClass
**Proximity detonation fuse controller** - Manages fuse timing and proximity detection.

**Key data members:**
| Field | Type | Purpose |
|-------|------|---------|
| `Timer` (int) | Maximum fuse lifetime counter (clamped 0xFF) |
| `Arming` (int) | Minimum arming delay before detonation allowed (clamped 0xFF) |
| `HeadTo` (COORDINATE) | Target/impact point coordinates |
| `Proximity` (int) | Last known distance to target |

**Constants:**
| Constant | Value | Purpose |
|----------|-------|---------|
| `0xFF` | 255 | Max value for Timer/Arming (1 byte) |
| `0x0010` | 16 | Immediate proximity trigger threshold |
| `ICON_LEPTON_W` | - | Icon width in leptons, proximity re-approach threshold |

## Functions and methods

### FuseClass::FuseClass()
**Constructor:**
1. `Timer = 0`
2. `Arming = 0`
3. `HeadTo = 0`
4. `Proximity = 0`

### FuseClass::Arm_Fuse(location, target, timeto, arming) → void
**Set up fuse for detonation:**

1. `timeto = MAX(timeto, arming)` - ensure max time >= arming time
2. `Timer = MIN(timeto, 0xFF)` - clamp timer to 1 byte
3. `Arming = MIN(arming, 0xFF)` - clamp arming to 1 byte
4. `HeadTo = target` - store target coordinates
5. `Proximity = Distance(location, target)` - initial distance to target

### FuseClass::Fuse_Checkup(newlocation) → bool
**Per-tick fuse processing - returns true if detonation should occur:**

1. **Always decrement timer**: `if (Timer) Timer--`
2. **Arming check**: If `Arming > 0`:
   - `Arming--` - still arming, no detonation
   - Return `false`
3. **Timer expired**: If `!Timer`:
   - Return `true` (force explosion)
4. **Proximity check**: `proximity = Distance(newlocation, HeadTo)`:
   - If `proximity < 0x0010` (16 leptons): Return `true` (very close)
   - If `proximity < ICON_LEPTON_W && proximity > Proximity`:
     - Getting closer than before and within icon width → Return `true`
   - `Proximity = proximity` - update last known distance
5. Return `false` (no detonation yet)

**Called once per game tick per active fuse.**

### FuseClass::Fuse_Write(file) → void
**Serialize fuse to save game:**
1. `file.Write(&Timer, sizeof(Timer))`
2. `file.Write(&Arming, sizeof(Arming))`
3. `file.Write(&HeadTo, sizeof(HeadTo))`
4. `file.Write(&Proximity, sizeof(Proximity))`

### FuseClass::Fuse_Read(file) → void
**Deserialize fuse from save game:**
1. `file.Read(&Timer, sizeof(Timer))`
2. `file.Read(&Arming, sizeof(Arming))`
3. `file.Read(&HeadTo, sizeof(HeadTo))`
4. `file.Read(&Proximity, sizeof(Proximity))`

## Inputs, outputs and side effects

**State mutations:**
- `Timer` decremented each tick
- `Arming` decremented each tick (while > 0)
- `Proximity` updated each tick (current distance to target)
- Returns `true` when detonation should occur

**External systems affected:**
- `BULLET.CPP` → calls `Fuse_Checkup()` per tick for bullets with fuses
- `WEAPON.CPP` → fuse timing integrated with warhead detonation
- `COMBAT.CPP` → explosion damage when fuse triggers
- `SAVELOAD.CPP` → `Fuse_Write()`/`Fuse_Read()` for savegame

## Calls into / called by

**Called by:**
- `BULLET.CPP` → `Fuse_Checkup()` per game tick per bullet with fuse
- `SAVELOAD.CPP` → `Fuse_Write()`/`Fuse_Read()` for save/load

**Calls into:**
- `Distance(location, target)` → lepton distance calculation
- `FileClass::Write()`/`Read()` → savegame binary I/O

## Related original files
- **FUSE.H** - Class declarations
- **BULLET.CPP** → fuse checkup per bullet tick
- **WEAPON.CPP** → warhead detonation integration
- **COMBAT.CPP** → explosion damage pipeline
- **SAVELOAD.CPP** → fuse serialization

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/weapon/fuse.rs` - FuseClass and detonation logic
- `cnc-tiberian-dawn/src/weapon/proximity.rs` - Proximity detection

**Not yet ported:**
- Timer/Arming 1-byte clamping (0xFF)
- Proximity threshold `0x0010` (16 leptons)
- `ICON_LEPTON_W` re-approach trigger
- `Arm_Fuse()` initialization with location/target/time/arming
- Save/load binary format (`Fuse_Write`/`Fuse_Read`)
- Per-tick fuse processing in game loop

## Edge cases and compatibility notes

1. **Arming before timer**: `Arming` must elapse before proximity/timer detonation allowed
2. **Timer forced explosion**: When `Timer` reaches 0, fuse detonates regardless of proximity
3. **Proximity re-approach**: `proximity < ICON_LEPTON_W && proximity > Proximity` detects getting closer than before within icon width
4. **1-byte clamping**: Both `Timer` and `Arming` clamped to 0xFF (255 max)
5. **Always decrement**: Timer decrements even during arming phase
6. **Distance in leptons**: All proximity checks use lepton coordinate units
7. **Save/load**: All 4 fields serialized as raw binary (no pointer fixup needed)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/FUSE.CPP at f1f0d42
