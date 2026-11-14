# PATH/FILE
original/TIBERIANDAWN/MISSION.CPP

## Purpose
Mission/order system for Tiberian Dawn units. Implements a script-based order processor with timer-driven state machine, mission queue for deferred orders, and 21 mission types covering movement, combat, harvesting, guarding, and special behaviors.

## Where it fits in the game
Unit AI subsystem - Every `ObjectClass` (and derivatives: `UnitClass`, `InfantryClass`, `AircraftClass`, `BuildingClass`) contains a `MissionClass` member. Called per-frame via `LogicClass::AI()` → `ObjectClass::AI()` → `MissionClass::AI()`.

## Classes, structs, enums and important data

### MissionClass
**Order processor** - Embedded in every `ObjectClass`.

**Key data members:**
| Field | Purpose |
|-------|---------|
| `Mission` (MissionType) | Current active order |
| `MissionQueue` (MissionType) | Deferred order (set by `Assign_Mission`) |
| `SuspendedMission` (MissionType) | Temporarily overridden mission |
| `Timer` (CountDownTimerClass) | State machine delay timer |
| `Status` (int) | Script state index |
| `Strength` | Unit health/strength (inherited) |

### MissionType enum (21 types, MISSION_COUNT=21)
| Mission | Description | Timer (ticks) |
|---------|-------------|---------------|
| `MISSION_NONE` | No order | - |
| `MISSION_SLEEP` | Idle wait | 30 sec |
| `MISSION_ATTACK` | Attack target | 30 sec |
| `MISSION_MOVE` | Move to location | 30 sec |
| `MISSION_RETREAT` | Flee from threat | 30 sec |
| `MISSION_GUARD` | Guard target | 30 sec |
| `MISSION_STICKY` | Persistent guard | 30 sec |
| `MISSION_ENTER` | Enter structure | 30 sec |
| `MISSION_CAPTURE` | Capture building | 30 sec |
| `MISSION_HARVEST` | Harvest tiberium | 30 sec |
| `MISSION_GUARD_AREA` | Area patrol | 30 sec |
| `MISSION_RETURN` | Return to base | 30 sec |
| `MISSION_STOP` | Halt | 30 sec |
| `MISSION_AMBUSH` | Wait in ambush | 30 sec |
| `MISSION_HUNT` | Seek & destroy | 30 sec |
| `MISSION_RESCUE` | Rescue unit | 30 sec |
| `MISSION_TIMED_HUNT` | Timed hunt | 30 sec |
| `MISSION_UNLOAD` | Unload passengers | 30 sec |
| `MISSION_SABOTAGE` | Sabotage building | 30 sec |
| `MISSION_CONSTRUCTION` | Build structure | 30 sec |
| `MISSION_SELLING` | Sell building | 30 sec |
| `MISSION_REPAIR` | Repair structure | 30 sec |
| `MISSION_MISSILE` | Missile strike | 30 sec |

**Special constants:**
- `MISSION_NONE` = -1 (invalid)
- `MISSION_FIRST` = 0
- `MISSION_COUNT` = 21
- `TICKS_PER_SECOND` = 15 (15 FPS)
- Default timer = `TICKS_PER_SECOND * 30` = 450 ticks (30 seconds)

### MissionClass::Missions[] (string names)
Maps `MissionType` → display string (e.g., "Attack", "Move", "Area Guard").

## Functions and methods

### MissionClass()
**Constructor** - Initializes to idle state:
- `Mission = MISSION_NONE`
- `MissionQueue = MISSION_NONE`
- `SuspendedMission = MISSION_NONE`
- `Status = 0`, `Timer = 0`

### Set_Mission(mission)
**Immediate order change** - Bypasses queue:
1. `Mission = mission`
2. `MissionQueue = MISSION_NONE`

### Get_Mission() → MissionType
**Current active mission** - Returns `Mission` if active, else `MissionQueue`.

### Assign_Mission(order)
**Queues new order** - Non-immediate:
1. Returns if `order == MISSION_NONE` or `Mission == order`
2. `MissionQueue = order`
3. (Commented) `Status = 0`

### Commence() → bool
**Processes queued order** - Call when unit ready to change order:
1. If `MissionQueue != MISSION_NONE`:
   - `Mission = MissionQueue`, `MissionQueue = MISSION_NONE`
   - `Timer = 0`, `Status = 0` (reset state machine)
   - Returns true
2. Returns false

### Override_Mission(mission, tarcom, navcom)
**Temporary mission override**:
1. Saves current (`MissionQueue` or `Mission`) to `SuspendedMission`
2. `Assign_Mission(mission)` (queues new)

### Restore_Mission() → bool
**Restores suspended mission**:
1. If `SuspendedMission != MISSION_NONE`:
   - `Assign_Mission(SuspendedMission)`
   - `SuspendedMission = MISSION_NONE`
   - Returns true
2. Returns false

### Mission_From_Name(name) → MissionType
**String → enum** - Linear search through `Missions[]` array (case-insensitive).

### Mission_Name(mission) → char const*
**Enum → string** - Returns `Missions[mission]` or "None".

### AI()
**Per-frame order processor** - Called from `ObjectClass::AI()`.

**Logic:**
1. `ObjectClass::AI()` (base movement/behavior)
2. If `Timer.Expired() && Strength > 0`:
   - Switches on `Mission`, sets `Timer` to mission-specific duration
   - All missions use `TICKS_PER_SECOND * 30` (30 seconds)
   - `MISSION_STICKY`/`SLEEP` → `Mission_Sleep()`
   - `MISSION_GUARD` → `Mission_Guard()`
   - `MISSION_ENTER` → `Mission_Enter()`
   - `MISSION_CONSTRUCTION` → `Mission_Construction()`
   - `MISSION_DECONSTRUCTION` → `Mission_Deconstruction()`
   - `MISSION_CAPTURE`/`SABOTAGE` → `Mission_Capture()`
   - `MISSION_MOVE` → `Mission_Move()`
   - `MISSION_ATTACK` → `Mission_Attack()`
   - `MISSION_RETREAT` → `Mission_Retreat()`
   - `MISSION_HARVEST` → `Mission_Harvest()`
   - `MISSION_GUARD_AREA` → `Mission_Guard_Area()`
   - `MISSION_RETURN` → `Mission_Return()`
   - `MISSION_STOP` → `Mission_Stop()`
   - `MISSION_AMBUSH` → `Mission_Ambush()`
   - `MISSION_HUNT`/`RESCUE` → `Mission_Hunt()`
   - `MISSION_TIMED_HUNT` → `Mission_Timed_Hunt()`
   - `MISSION_UNLOAD` → `Mission_Unload()`
   - `MISSION_REPAIR` → `Mission_Repair()`
   - `MISSION_MISSILE` → `Mission_Missile()`

### Debug_Dump(mono) (CHEAT_KEYS)
**Monochrome debug output**:
- Mission/Queue names (5-char truncated)
- Timer value
- Status
- Calls `ObjectClass::Debug_Dump()`

## Mission timer constants
All missions use `TICKS_PER_SECOND * 30` = 450 ticks (30 seconds at 15 FPS).

```cpp
int MissionClass::Mission_Sleep(void) {return TICKS_PER_SECOND*30;};
// ... all 21 missions return same value
```

## Mission queue vs immediate
| Method | Behavior |
|--------|----------|
| `Set_Mission()` | Immediate, clears queue |
| `Assign_Mission()` | Queues for next `Commence()` |
| `Commence()` | Processes queue if ready |
| `Override_Mission()` | Saves current, queues new |
| `Restore_Mission()` | Restores suspended |

## Inputs, outputs and side effects

**State mutations:**
- `Mission`, `MissionQueue`, `SuspendedMission`
- `Timer` (countdown)
- `Status` (script state)
- `Strength` check (dead units skip)

**External systems affected:**
- Unit movement (`MISSION_MOVE`, `MISSION_RETREAT`, etc.)
- Combat (`MISSION_ATTACK`, `MISSION_AMBUSH`, `MISSION_HUNT`)
- Economy (`MISSION_HARVEST`)
- Construction (`MISSION_CONSTRUCTION`, `MISSION_REPAIR`, `MISSION_SELLING`)
- Base defense (`MISSION_GUARD`, `MISSION_GUARD_AREA`)

## Calls into / called by

**Called by:**
- `LogicClass::AI()` → `ObjectClass::AI()` → `MissionClass::AI()`
- Player input → `Assign_Mission()` / `Set_Mission()`
- AI planning → `Assign_Mission()` / `Override_Mission()`
- Order completion → `Commence()`

**Calls into:**
- `CountDownTimerClass::Expired()`
- `ObjectClass::AI()` (base)
- `Timer = Mission_*()` (timer reset)

## Related original files
- **MISSION.H** - Class declaration, MissionType enum
- **OBJECT.H/.CPP** - ObjectClass (contains MissionClass)
- **UNIT.H/.CPP** - UnitClass (specific mission implementations)
- **INFANTRY.H/.CPP** - InfantryClass
- **AIRCRAFT.H/.CPP** - AircraftClass
- **BUILDING.H/.CPP** - BuildingClass
- **TIMER.H** - CountDownTimerClass
- **TARGET.H** - TARGET type for tarcom/navcom

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/mission.rs` - MissionClass, MissionType
- `cnc-tiberian-dawn/src/ai/td_mission.rs` - TD-specific missions
- `cnc-core/src/ai/state_machine.rs` - Timer/Status state machine

**Not yet ported:**
- 21 mission type enum with string mapping
- Timer-driven state machine (Status + Timer)
- Mission queue with Commence() processing
- Override/Restore suspend stack (single level)
- 30-second uniform timer for all missions
- Mission_From_Name/Mission_Name string mapping
- Cheat debug dump
- Strength check (dead units skip AI)

## Edge cases and compatibility notes

1. **Uniform timer**: All 21 missions use identical 30-second timer (placeholder)
2. **Queue vs immediate**: `Assign_Mission` queues; `Set_Mission` immediate
3. **SuspendedMission**: Single-level stack (override → restore)
4. **Timer reset**: `Commence()` sets `Timer=0`, `Status=0` for immediate processing
5. **Strength gate**: Dead/weak units (`Strength <= 0`) skip mission AI
6. **Default case**: Falls to `Mission_Sleep()` (idle)
7. **Status/Timer**: State machine framework (Status=state index, Timer=delay)
8. **Mission_From_Name**: Linear search, case-insensitive
9. **Mission_Name**: Returns "None" for `MISSION_NONE`
10. **Uniform durations**: All missions currently use same 30s timer (likely placeholder)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/MISSION.CPP at f1f0d42
