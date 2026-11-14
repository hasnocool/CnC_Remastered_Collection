# PATH/FILE
original/TIBERIANDAWN/MISSION.CPP

## Purpose
Mission/order system for Tiberian Dawn. Implements the finite state machine for unit orders (missions), including mission queue, timer management, and per-mission AI handlers.

## Where it fits in the game
Mission system - every orderable object (units, infantry, aircraft, buildings) has a MissionClass instance that manages its current order, queued orders, and timer-driven state machine. The core of unit AI behavior.

## Classes, structs, enums and important data

### MissionClass
**Core mission/order state** - attached to every orderable object.

**Key data members:**
- `Mission` (MissionType) - Current active mission
- `MissionQueue` (MissionType) - Queued mission (pending)
- `SuspendedMission` (MissionType) - Temporarily overridden mission
- `Timer` - Mission timer (ticks until next state)
- `Status` (int) - Mission state machine status
- `MissionQueue` (MissionType) - Queued mission
- `SuspendedMission` (MissionType) - Overridden mission
- `Timer` - Mission timer
- `Status` (int) - State machine status

### MissionType enum (24 types)
| Mission | Description | Timer |
|---------|-------------|-------|
| MISSION_NONE | No mission | - |
| MISSION_SLEEP | Idle | 30s |
| MISSION_ATTACK | Attack target | 30s |
| MISSION_MOVE | Move to location | 30s |
| MISSION_RETREAT | Retreat | 30s |
| MISSION_GUARD | Guard area | 30s |
| MISSION_STICKY | Sticky guard | 30s |
| MISSION_ENTER | Enter transport/building | 30s |
| MISSION_CAPTURE | Capture building | 30s |
| MISSION_HARVEST | Harvest tiberium | 30s |
| MISSION_GUARD_AREA | Guard area | 30s |
| MISSION_RETURN | Return to base | 30s |
| MISSION_STOP | Stop | 30s |
| MISSION_AMBUSH | Ambush | 30s |
| MISSION_HUNT | Hunt enemies | 30s |
| MISSION_TIMED_HUNT | Timed hunt | 30s |
| MISSION_UNLOAD | Unload cargo | 30s |
| MISSION_SABOTAGE | Sabotage | 30s |
| MISSION_CONSTRUCTION | Construction | 30s |
| MISSION_SELLING | Selling | 30s |
| MISSION_REPAIR | Repair | 30s |
| MISSION_RESCUE | Rescue | 30s |
| MISSION_MISSILE | Missile | 30s |

**Constants:**
- `MISSION_FIRST` - First mission enum
- `MISSION_COUNT` - Total missions (24)
- `MISSION_NONE` - No mission
- `TICKS_PER_SECOND` - Ticks per second (60)
- `TICKS_PER_MINUTE` - Ticks per minute (3600)

### Mission timer constants
All missions default to `TICKS_PER_SECOND * 30` (30 seconds) unless overridden.

## Functions and methods

### Core Mission Management

**MissionClass::MissionClass()** - Default constructor
- `Status=0, Timer=0, Mission=MISSION_NONE, SuspendedMission=MISSION_NONE, MissionQueue=MISSION_NONE`

**MissionClass::Set_Mission(mission)** - Immediate mission change
- Sets `Mission = mission`, clears `MissionQueue`
- Bypasses queue, immediate change

**MissionClass::Get_Mission()** → MissionType
- Returns current mission, or `MissionQueue` if none active

**MissionClass::Assign_Mission(order)** - Queue mission
- If `order == MISSION_NONE` or same as current: returns
- Sets `MissionQueue = order` (processed by AI)

**MissionClass::Commence()** → bool
- Processes queued mission: `Mission = MissionQueue`, `MissionQueue = NONE`
- Resets `Timer=0, Status=0`
- Returns true if mission changed

**MissionClass::Override_Mission(mission, tarcom, navcom)** - Temporary override
- Saves current mission to `SuspendedMission`
- Calls `Assign_Mission(mission)`

**MissionClass::Restore_Mission()** → bool
- Restores `SuspendedMission` via `Assign_Mission()`
- Clears `SuspendedMission`
- Returns true if restored

### Mission AI Processing

**MissionClass::AI()** - **Core mission state machine** (called per frame)
- Calls `ObjectClass::AI()`
- If `Timer.Expired() && Strength > 0`:
  - Switches on `Mission`, calls mission-specific handler
  - Sets `Timer = Mission_XXX()` for each mission type
- All missions default to `TICKS_PER_SECOND * 30` (30 seconds)

**Mission handlers (all return ticks until next call):**
- `MISSION_STICKY`, `MISSION_SLEEP`: `Mission_Sleep()` (30s)
- `MISSION_GUARD`: `Mission_Guard()` (30s)
- `MISSION_ENTER`: `Mission_Enter()` (30s)
- `MISSION_CONSTRUCTION`: `Mission_Construction()` (30s)
- `MISSION_DECONSTRUCTION`: `Mission_Deconstruction()` (30s)
- `MISSION_CAPTURE`, `MISSION_SABOTAGE`: `Mission_Capture()` (30s)
- `MISSION_MOVE`: `Mission_Move()` (30s)
- `MISSION_ATTACK`: `Mission_Attack()` (30s)
- `MISSION_RETREAT`: `Mission_Retreat()` (30s)
- `MISSION_HARVEST`: `Mission_Harvest()` (30s)
- `MISSION_GUARD_AREA`: `Mission_Guard_Area()` (30s)
- `MISSION_RETURN`: `Mission_Return()` (30s)
- `MISSION_STOP`: `Mission_Stop()` (30s)
- `MISSION_AMBUSH`: `Mission_Ambush()` (30s)
- `MISSION_HUNT`, `MISSION_RESCUE`: `Mission_Hunt()` (30s)
- `MISSION_TIMED_HUNT`: `Mission_Timed_Hunt()` (30s)
- `MISSION_UNLOAD`: `Mission_Unload()` (30s)
- `MISSION_SABOTAGE`: `Mission_Capture()` (30s)
- `MISSION_CONSTRUCTION`: `Mission_Construction()` (30s)
- `MISSION_SELLING`: `Mission_Selling()` (30s)
- `MISSION_REPAIR`: `Mission_Repair()` (30s)
- `MISSION_MISSILE`: `Mission_Missile()` (30s)

**Note:** Specific implementations in derived classes (UnitClass, InfantryClass, etc.)

### Mission Management

**MissionClass::Set_Mission(mission)** - Immediate change
- `Mission = mission`, `MissionQueue = MISSION_NONE`

**MissionClass::Get_Mission()** → MissionType
- Returns `Mission` or `MissionQueue` if none

**MissionClass::Assign_Mission(order)** - Queues mission
- If `order == NONE` or same as current: returns
- Sets `MissionQueue = order`

**MissionClass::Commence()** → bool
- Processes queue: `Mission = MissionQueue`, `MissionQueue = NONE`
- Resets `Timer=0, Status=0`
- Returns true if changed

**MissionClass::Override_Mission(mission, tarcom, navcom)** - Temporary override
- Saves current to `SuspendedMission`
- Calls `Assign_Mission(mission)`

**MissionClass::Restore_Mission()** → bool
- Restores `SuspendedMission` via `Assign_Mission()`
- Returns true if restored

**MissionClass::Assign_Mission(order)** - Queue mission
- No-op if NONE or same as current
- Sets `MissionQueue = order`

### Utility

**MissionClass::Mission_From_Name(name)** → MissionType
- Converts string name to enum (case-insensitive)
- Searches `Missions[MISSION_COUNT]` array

**MissionClass::Mission_Name(mission)** → const char*
- Returns string name from `Missions[mission]` array
- "None" for MISSION_NONE

**MissionClass::Mission_Name(mission)** → const char*
- Returns string from `Missions[]` array

### Constants

**Mission timer defaults:**
All return `TICKS_PER_SECOND * 30` (30 seconds = 1800 ticks)

**Mission name array:**
`Missions[MISSION_COUNT]` - String names for INI/debug

## Inputs, outputs and side effects

**State mutations:**
- Mission, MissionQueue, SuspendedMission
- Timer, Status
- Mission state machine progression

**External systems affected:**
- Object AI processing
- Pathfinding (NavCom)
- Targeting (TarCom)
- Movement (FootClass/UnitClass)

## Calls into / called by

**Called by:**
- Object AI: `MissionClass::AI()` per frame
- Player input: `Assign_Mission()`, `Set_Mission()`
- Commence: `Commence()` after queue
- Override: `Override_Mission()`, `Restore_Mission()`

**Calls into:**
- ObjectClass: `AI()`
- Derived mission handlers: `Mission_Move()`, `Mission_Attack()`, etc.
- Timer system: `Timer.Expired()`

## Related original files
- **MISSION.H** - Class declarations, MissionType enum
- **OBJECT.CPP/.H** - Base ObjectClass
- **FOOT.CPP/.H** - FootClass (movement)
- **UNIT.CPP/.H** - UnitClass (vehicle missions)
- **INFANTRY.CPP/.H** - InfantryClass (infantry missions)
- **AIRCRAFT.CPP/.H** - AircraftClass (air missions)
- **BUILDING.CPP/.H** - BuildingClass (building missions)
- **TECHNO.CPP/.H** - TechnoClass (base techno missions)
- **FOOT.CPP/.H** - FootClass (movement missions)
- **TARCOM.CPP** - Target/navigation computer

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/mission.rs` - MissionClass core
- `cnc-tiberian-dawn/src/ai/td_mission.rs` - TD-specific missions
- `cnc-core/src/ai/state_machine.rs` - Mission state machine
- `cnc-core/src/ai/timer.rs` - Mission timer
- `cnc-core/src/ai/mission_types.rs` - MissionType enum

**Not yet ported:**
- Full mission state machine (24 states)
- Per-mission AI handlers (Move, Attack, Harvest, etc.)
- Mission queue/override/restore system
- 30-second timer default
- Mission name/string conversion
- Commence/Override/Restore mechanics

## Edge cases and compatibility notes

1. **30-second timer**: All missions default to 30-second timer (`TICKS_PER_SECOND * 30`). Specific implementations override.

2. **Mission queue**: `Assign_Mission()` queues; `Commence()` processes. Allows player input to queue while AI processes.

2. **Queue vs immediate**: `Set_Mission` = immediate; `Assign_Mission` = queued; `Commence` processes queue.

3. **Override/Restore**: `Override_Mission` saves current to `SuspendedMission`; `Restore_Mission` restores it.

3. **Override/Restore**: `Override_Mission` saves to `SuspendedMission`; `Restore_Mission` restores.

4. **Commence**: Processes queue immediately, resets timer/status.

4. **Commence**: Processes queue, resets timer/status=0.

5. **Timer**: 30-second default for all missions. Specific implementations may return different values.

5. **Timer**: 30-second default for all missions.

6. **Mission names**: `Missions[]` array provides string names for INI/debug.

6. **Mission names**: `Missions[]` array for INI/debug.

7. **Get_Mission**: Returns current mission, or queued if none active.

7. **Get_Mission**: Returns current or queued.

8. **No implementation in base**: All `Mission_XXX()` return 30s. Derived classes override.

7. **No base implementation**: All `Mission_XXX()` return 30s. Derived classes override.

8. **Mission names**: `Missions[]` for INI/debug.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/MISSION.CPP at f1f0d42
