# PATH/FILE
original/TIBERIANDAWN/SUPER.CPP

## Purpose
Super weapon system for Tiberian Dawn. Manages charging, discharging, and animation of super weapons (nuke, ion cannon, etc.). Handles voice announcements, recharge timing, and one-time super weapons from crates. Core to the special weapons gameplay loop.

## Where it fits in the game
Special weapons infrastructure - super weapons are player-activated destructive abilities with cooldown/recharge periods. Integrated with `STARTUP.CPP` initialization, `LOGIC.CPP` game loop, and `SPECIAL.CPP` options dialog. Global `Super` instance array manages multiple simultaneous super weapons.

## Classes, structs, enums and important data

### SuperClass
**Individual super weapon instance** - One super weapon (nuke, ion cannon, etc.).

**Constructor parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `recharge` | int | Recharge delay in game frames |
| `ready` | VoxType | Voice announcement when fully charged |
| `charging` | VoxType | Voice announcement while charging |
| `impatient` | VoxType | Voice announcement during charging |
| `suspend` | VoxType | Voice announcement when suspended |

**Key data members:**
| Field | Type | Purpose |
|-------|------|---------|
| `IsPresent` (bool) | Whether super weapon is active |
| `IsOneTime` (bool) | Whether this is a one-time crate super weapon |
| `IsReady` (bool) | Whether weapon is fully charged and ready |
| `IsSuspended` (bool) | Whether charging is temporarily suspended |
| `OldStage` (int) | Previous animation stage |
| `Control` (int) | Control/timer counter |
| `RechargeTime` (int) | Recharge delay in frames |
| `SuspendTime` (int) | Frame when suspension started |
| `VoxRecharge` (VoxType) | Voice ID for "ready" announcement |
| `VoxCharging` (VoxType) | Voice ID for charging announcement |
| `VoxImpatient` (VoxType) | Voice ID for impatient warning |
| `VoxSuspend` (VoxType) | Voice ID for suspend announcement |

### VoxType enum
Voice announcement types for super weapon state changes.

### SuperClass::AI()
**Process super weapon AI:**
- AI-controlled super weapon behavior
- Target selection and firing logic

### SuperClass::Anim_Stage() → int
**Fetch animation stage:**
- Returns current animation frame/stage
- Used for charging animation progression

### SuperClass::Discharged()
**Handle discharged action:**
- What happens when super weapon is used/fired
- Animation, effects, cooldown start

### SuperClass::Forced_Charge()
**Force super weapon to full charge:**
- Bypasses normal recharge time
- Used for cheats or special circumstances

### SuperClass::Impatient_Click()
**Called when player clicks unfinished super weapon:**
- Player interaction when weapon not yet charged
- May show impatient voice or enable firing at reduced power

### SuperClass::Recharge() → bool
**Start super weapon recharging:**
1. Checks `IsPresent && !IsReady`
2. Disables suspension: `IsSuspended = false`
3. Resets animation: `OldStage = -1`
4. Cheat override: if `Special.IsSpeedBuild` → accelerate recharge
5. Player voice announcement if `player` parameter true
6. Returns bool: success/failure

### SuperClass::Remove(forced) → bool
**Remove super weapon availability:**
1. Check `IsPresent && (!IsOneTime || forced)`
2. `IsReady = false; IsPresent = false;`
3. Returns success/failure
4. One-time weapons require `forced=true` for removal

### SuperClass::Enable(onetime, player, quiet) → bool
**Enable super weapon:**
1. Check `!IsPresent`
2. Set `IsPresent = true`
3. `IsOneTime = onetime`
4. Call `Recharge(player && !quiet)`
5. If `quiet`: `Suspend(true)`
6. Return retval (success/failure)

## Functions and methods

### SuperClass::SuperClass(recharge, ready, charging, impatient, suspend)
**Constructor:**
1. Initialize all flags to false:
   - `IsPresent = false`
   - `IsOneTime = false`
   - `IsReady = false`
   - `IsSuspended = false`
   - `OldStage = -1`
   - `Control = 0`
2. Set timing: `RechargeTime = recharge`
3. Set vox IDs: `VoxRecharge = ready`, `VoxCharging = charging`, `VoxImpatient = impatient`, `VoxSuspend = suspend`

### SuperClass::Suspend(on) → bool
**Suspend/unsuspend charging:**
1. Check: `IsPresent && !IsReady && !IsOneTime`
2. If state change needed (`on != IsSuspended`):
   - If `on=true`: `SuspendTime = Control` (record current time)
   - If `on=false`: `Control = SuspendTime` (restore from suspension)
3. Set: `IsSuspended = on`
4. Return: whether state was changed

### SuperClass::Enable(onetime, player, quiet) → bool
**Enable super weapon (called from startup/crates):**
1. If `!IsPresent`:
   - `IsPresent = true`
   - `IsOneTime = onetime`
   - `retval = Recharge(player && !quiet)`
   - If `quiet`: `Suspend(true)`
   - Return `retval`
2. Return `false` if already present

### SuperClass::Remove(forced) → bool
**Remove super weapon:**
1. If `IsPresent && (!IsOneTime || forced)`:
   - `IsReady = false`
   - `IsPresent = false`
   - Return `true`
2. Return `false` if conditions not met

## Inputs, outputs and side effects

**State mutations:**
- All 7 `SuperClass` boolean/timer fields
- `Super` array global state
- Vox voice playback state
- Animation stage counters
- Recharge timers per super weapon

**External systems affected:**
- `STARTUP.CPP` → super weapon initialization
- `LOGIC.CPP` → super weapon AI processing per game frame
- `SPECIAL.CPP` → options dialog affects `Special.IsSpeedBuild`
- `CRATE.CPP` → one-time super weapons from crates
- Multiplayer: super weapon synchronization

## Calls into / called by

**Called by:**
- `LOGIC.CPP` → `SuperClass::AI()` per game tick
- `STARTUP.CPP` → `SuperClass::Enable()` at game start
- `CRATE.CPP` → one-time super weapon enable
- `SPECIAL.CPP` → `Special.IsSpeedBuild` checks

**Calls into:**
- `SpecialClass` → `Special.IsSpeedBuild` for cheat acceleration
- Vox voice system for announcements
- Animation system for charging visuals
- Recharge timer management

## Related original files
- **Super.H** - Class declarations
- **STARTUP.CPP** → super weapon setup
- **LOGIC.CPP** → per-frame AI processing
- **CRATE.CPP** → one-time super weapons
- **SPECIAL.CPP** → `Special.IsSpeedBuild` cheat integration
- **VOX** voice enum definitions

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/super/superweapon.rs` - SuperClass and array
- `cnc-tiberian-dawn/src/super/vox.rs` - Voice announcement types
- `cnc-tiberian-dawn/src/super/recharge.rs` - Recharge timing logic

**Not yet ported:**
- Windows super weapon UI rendering
- Vox voice synthesis integration
- Recharge timer management per frame
- One-time super weapon crate logic
- AI target selection and firing
- Multiplayer super weapon sync
- `Special.IsSpeedBuild` cheat acceleration
- Animation stage progression

## Edge cases and compatibility notes

1. **One-time vs persistent**: `IsOneTime` distinguishes crate weapons from player-purchased
2. **Quiet mode**: `Enable(..., ..., true)` starts suspended for silent operation
3. **Cheat acceleration**: `Special.IsSpeedBuild` can bypass recharge delays
4. **Forced removal**: One-time weapons require `forced=true` to remove
5. **Suspension persistence**: `SuspendTime` tracks frames suspended; `Control` restores on unsuspend
6. **Animation stages**: `OldStage` tracks progression for charging visuals
7. **Global array**: Multiple super weapons can coexist via `Super[]` array
8. **Player vs AI**: `player` parameter controls voice announcements
9. **Recharge time**: Set at construction; `RechargeTime` frames between states
10. **Initial state**: All super weapons start `!IsPresent` until enabled

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SUPER.CPP at f1f0d42
