# PATH/FILE
original/TIBERIANDAWN/TARCOM.CPP

## Purpose
Turret command system for Tiberian Dawn. Handles AI logic for turret objects including targeting, weapon selection, and firing control. Turrets are defensive structures (guard towers, defensive positions) that autonomously engage enemies within range. Integrated with `LOGIC.CPP` game AI loop and `WEAPON.CPP` firing system.

## Where it fits in the game
Defensive AI - turret objects use `TarComClass` for targeting decisions. Part of the `TechnoClass` hierarchy for structure-based AI. Called per game tick from `LOGIC.CPP`. Linked to `NavComClass` for navigation coordination. Used by defensive structures like guard towers and defensive posts.

## Classes, structs, enums and important data

### TarComClass
**Turret command AI class** - Handles targeting and firing logic for turret objects.

**Inherits from:** `NavComClass` (navigation + AI base for structure objects)

**Key data members (via Class pointer):**
| Field | Purpose |
|-------|---------|
| `Class->Primary` (WeaponTypeIndex) | Primary weapon index |
| `Class->Secondary` (WeaponTypeIndex) | Secondary weapon index |
| `Class->IsLockTurret` (bool) | Whether turret locks on target |
| `Class->IsDriving` (bool) | Whether unit is driving/transported |
| `Weapons[2]` array | Primary and secondary weapon definitions |

### TurretClass (referred via `TarComClass::Debug_Dump`, `TarComClass::AI`)
**Base turret class** - Core turret AI logic shared across all turret types.

**AI behavior:**
- Weapon selection: primary first, then secondary
- Fire error checking via `Can_Fire()`
- Target legal validation
- Facing/aiming logic
- Sound effects on fire

## Functions and methods

### TarComClass::~TarComClass()
**Destructor:**
- Empty body (no dynamic cleanup needed)
- Turret objects use static weapon tables

### TarComClass::Debug_Dump(mono) → void
**Display turret status to monochrome screen:**
1. Delegates to `TurretClass::Debug_Dump(mono)`
2. Used for developer debugging only

### TarComClass::AI() → void
**Handle logical AI for targeting computer:**

**Weapon selection logic:**
1. Check primary weapon `Can_Fire(TarCom, 0)`
   - If `FIRE_OK`: use primary weapon
   - If not `FIRE_OK`: check secondary `Can_Fire(TarCom, 1)`
     - If `FIRE_OK`: use secondary weapon (set `primary = 1`)
     - Otherwise: failure from primary weapon

2. **Fire execution based on result:**
   - `FIRE_OK`:
     - Check `What_Am_I() != RTTI_UNIT` → `IsFiring = false`
     - For units: check `Class->IsFireAnim` → `IsFiring = false` if no fire anim
     - Call `TurretClass::Fire_At(TarCom, primary)`
     - On success: `Sound_Effect(weapon->Sound, Coord)`
   
   - `FIRE_FACING`:
     - If `Class->IsLockTurret`:
       - If no legal target and not driving: set desired facing to `Direction(TarCom)`
       - Secondary facing syncs with primary
     - If not locked turret:
       - Special handling for `UNIT_FTANK` (tank-specific logic)
     - Other facing cases: additional targeting checks

3. **Default case**: Fallback behavior when fire conditions not met

### Fire coordination
- `Can_Fire()` checks weapon readiness, target visibility, range, and facing
- `Fire_At()` executes the actual fire animation and damage
- `Sound_Effect()` plays weapon sound at coordinate position

## Inputs, outputs and side effects

**State mutations:**
- `IsFiring` flag per turret
- `PrimaryFacing`/`SecondaryFacing` desired direction
- `Control` timers for firing cooldown
- Weapon discharge state
- Sound playback

**External systems affected:**
- `LOGIC.CPP` → called per game tick for all turret objects
- `WEAPON.CPP` → weapon definitions and damage tables
- `NavComClass` → navigation coordination
- `Targeting computer` → enemy detection and tracking
- `Sound system` → weapon fire audio

## Calls into / called by

**Called by:**
- `LOGIC.CPP` → `TarComClass::AI()` per game tick for each turret
- `STARTUP.CPP` → turret initialization

**Calls into:**
- `TurretClass::AI()` → base turret AI
- `Can_Fire()` → weapon fire eligibility check
- `Fire_At()` → execute fire
- `Sound_Effect()` → audio playback
- `Direction()` → facing calculation
- `Target_Legal()` → target validation

## Related original files
- **TarCom.H** - Class declarations
- **Turret.H** - Turret base class
- **NavCom.H** - Navigation command base
- **LOGIC.CPP** → per-frame AI processing
- **WEAPON.CPP** → weapon definitions and `Can_Fire()`
- **TURRET.CPP** → turret firing implementation

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/ai/turret_ai.rs` - TarComClass and AI logic
- `cnc-tiberian-dawn/src/ai/weapon_selection.rs` - Can_Fire and weapon choice

**Not yet ported:**
- Windows turret UI rendering
- `Can_Fire()` eligibility logic (range, facing, target visibility)
- `Fire_At()` execution and damage application
- Sound effect synchronization
- `IsLockTurret` AI behavior toggles
- `IsDriving` transported state handling
- Two-weapon primary/secondary selection
- Facing/aiming animation logic
- Multiplayer turret synchronization

## Edge cases and compatibility notes

1. **Primary/secondary weapon fallback**: If primary cannot fire, secondary is checked
2. **Lock turret mode**: `IsLockTurret` forces direction setting even without target
3. **Driving state**: `IsDriving` skips some targeting checks (transported units)
4. **Unit fire animation**: `IsFireAnim` check prevents firing without animation
5. **Tank-specific logic**: `UNIT_FTANK` has special facing behavior
6. **Sound on fire**: Weapon sound always plays on successful fire
7. **Debug only**: `Debug_Dump` is conditional on `CHECK_KEYS`
8. **Facing states**: Multiple `FIRE_FACING` sub-cases for different aiming conditions
9. **Target legality**: `Target_Legal()` validates target before firing
10. **One fire per tick**: Turret typically fires once per game logic cycle

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TARCOM.CPP at f1f0d42
