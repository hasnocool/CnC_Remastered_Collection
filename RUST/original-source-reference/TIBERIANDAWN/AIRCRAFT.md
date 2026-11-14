# AIRCRAFT.CPP Documentation

## File Overview
- **Path**: `original/TIBERIANDAWN/AIRCRAFT.CPP`
- **Header**: `original/TIBERIANDAWN/AIRCRAFT.H`
- **Programmer**: Joe L. Bostic
- **Start Date**: July 22, 1994
- **Last Update**: August 13, 1995 [JLB]
- **Description**: Implements aircraft behavior for Command & Conquer Tiberian Dawn. Handles all aircraft types including transport helicopters, attack helicopters (Apache, Orca), fixed-wing aircraft (A-10, Cargo plane), and their AI, movement, combat, landing/takeoff, and transport logic.

---

## Class Hierarchy

```
AbstractClass
    └── FootClass (radio.h)
        └── FlyClass (fly.h)
            └── AircraftClass
```

**Inheritance**: `AircraftClass : public FootClass, public FlyClass`

---

## Aircraft Types (from DEFINES.H)

```cpp
typedef enum AircraftType : char {
    AIRCRAFT_TRANSPORT,    // Transport helicopter (Chinook)
    AIRCRAFT_A10,          // Ground attack plane (A-10)
    AIRCRAFT_HELICOPTER,   // Apache gunship (GDI)
    AIRCRAFT_CARGO,        // Cargo plane (C-17)
    AIRCRAFT_ORCA,         // Nod attack helicopter

    AIRCRAFT_COUNT,
    AIRCRAFT_NONE = -1,
    AIRCRAFT_FIRST = 0
} AircraftType;
```

**Bit flags for scanning**:
- `AIRCRAFTF_TRANSPORT` = (1L << AIRCRAFT_TRANSPORT)
- `AIRCRAFTF_A10` = (1L << AIRCRAFT_A10)
- `AIRCRAFTF_HELICOPTER` = (1L << AIRCRAFT_HELICOPTER)
- `AIRCRAFTF_CARGO` = (1L << AIRCRAFT_CARGO)
- `AIRCRAFTF_ORCA` = (1L << AIRCRAFT_ORCA)

**Constants**:
- `AIRCRAFT_MAX = 100` — Maximum simultaneous aircraft objects (pool size)

---

## Class: AircraftClass

### Data Members

| Member | Type | Description |
|--------|------|-------------|
| `Class` | `AircraftTypeClass const * const` | Pointer to aircraft type definition (stats, weapons, art) |
| `SecondaryFacing` | `FacingClass` | Body facing for helicopters (rotors); separate from movement facing |
| `IsLanding` | `unsigned:1` | Flag: aircraft is descending to land |
| `IsTakingOff` | `unsigned:1` | Flag: aircraft is ascending from ground |
| `IsHoming` | `unsigned:1` | Flag: constantly adjusting facing toward target (TarCom) |
| `IsHovering` | `unsigned:1` | Flag: helicopter hovering/shifting sideways over LZ |
| `Jitter` | `unsigned char` | Animation counter for helicopter bobbing effect |
| `Altitude` | `int` | Altitude in pixels (0 = landed, FLIGHT_LEVEL = 24 = flying) |
| `SightTimer` | `TCountDownTimerClass` | Controls periodic sight revelation for human players |
| `AttacksRemaining` | `char` | Number of attack runs remaining (fixed-wing) |
| `ReinforcementStart` | `long` | Frame when cargo plane reinforcement becomes active |
| `SaveLoadPadding` | `unsigned char[28]` | Padding for save/load compatibility |
| `VTable` | `static void *` | Virtual function table pointer (for pool allocation) |

**Inherited from FootClass**: Mission, Status, TarCom, NavCom, Team, Health, Speed, PrimaryFacing, Coord, House, IsActive, IsInLimbo, etc.

**Inherited from FlyClass**: Physics, movement, facing logic.

### Constants

```cpp
enum { FLIGHT_LEVEL = 24 };  // Altitude when fully airborne (pixels)
```

---

## Constructor & Destructor

### AircraftClass(AircraftType classid, HousesType house)
Creates an aircraft of the specified type owned by the given house.

**Initialization**:
- Sets `Class` pointer to type data
- `IsSecondShot = false` if two-shooter, else `true`
- `Ammo = Class->MaxAmmo`
- `AttacksRemaining = 3` (fixed-wing attack runs)
- `Altitude = FLIGHT_LEVEL`
- `IsLanding = IsTakingOff = IsHovering = IsHoming = false`
- `Strength = Class->MaxStrength`
- `NavCom = TARGET_NONE`
- `SecondaryFacing = PrimaryFacing`
- `Jitter = 0`
- `ReinforcementStart = -1`
- Increments house aircraft totals (except CARGO in multiplayer)
- Registers with RA AI tracking system

### ~AircraftClass()
**Cleanup**:
- Removes from RA AI tracking
- Deletes all attached cargo/passengers
- Calls `Limbo()` to remove from map
- Removes from team if attached

---

## Memory Management (Pool Allocation)

### operator new(size_t) → void*
Allocates from global `Aircraft` pool (size `AIRCRAFT_MAX`). Calls `Set_Active()` on success.

### operator delete(void* ptr)
Returns object to pool, clears `IsActive` flag.

### static void Init()
Initializes pool: frees all, allocates one temporary to capture VTable pointer, then deletes it.

---

## Core Query Functions

### operator AircraftType() const → AircraftType
Returns `Class->Type`.

### RTTIType What_Am_I() const → RTTIType
Returns `RTTI_AIRCRAFT`.

### TARGET As_Target() const
Returns `Build_Target(KIND_AIRCRAFT, Aircraft.ID(this))`.

### int Validate() const
Debug-only: verifies object index is within pool bounds.

### COORDINATE Sort_Y() const
Returns `Coord_Add(Coord, 0x00800000L)` for ground-layer sorting.

### LayerType In_Which_Layer() const
- Fixed-wing: always `LAYER_TOP`
- Helicopters: `LAYER_GROUND` if `Altitude < FLIGHT_LEVEL - FLIGHT_LEVEL/3`, else `LAYER_TOP`

### ObjectTypeClass const& Class_Of() const
Returns `*Class`.

---

## Mission State Machine

Each mission is a virtual function returning ticks until next call. Uses `Status` for sub-states.

### Mission_Hunt() → int
**Fixed-wing (A-10, Cargo)**:
- States: `LOOK_FOR_TARGET` → `FLY_TO_TARGET` → `DROP_BOMBS`
- Acquires target via `Greatest_Threat(THREAT_NORMAL)`
- Homes in (`IsHoming = true`), drops bombs at distance `< 0x0380`
- Depletes `Ammo`, decrements `AttacksRemaining` (3 runs), then `MISSION_RETREAT`

**Rotary-wing**:
- If no ammo: `Enter_Idle_Mode()`
- Else: `Assign_Mission(MISSION_ATTACK)`

### Mission_Attack() → int
**Rotary-wing only** (fixed-wing redirects to Hunt).
States: `VALIDATE_AZ` → `PICK_ATTACK_LOCATION` → `TAKE_OFF` → `FLY_TO_POSITION` → `FIRE_AT_TARGET` → `FIRE_AT_TARGET2` → `RETURN_TO_BASE`
- Picks firing position via `Good_Fire_Location()` within weapon range
- Takes off if landed, flies to position, faces target, fires
- Two-shot burst (`IsSecondShot` flag), then re-acquires or returns to base

### Mission_Unload() → int
**Fixed-wing (Cargo/Transport plane)**:
- States: `PICK_AIRSTRIP` → `FLY_TO_AIRSTRIP` → `BUG_OUT`
- Finds friendly Airstrip (`STRUCT_AIRSTRIP`), establishes radio contact
- Flies to strip, descends, unloads passengers at `< 0x0080` distance
- Passengers scatter from exit cell; announces reinforcements via VOX if player-owned
- Then `MISSION_RETREAT`

**Rotary-wing (Transport helicopter)**:
- States: `SEARCH_FOR_LZ` → `FLY_TO_LZ` → `LAND_ON_LZ` → `UNLOAD_PASSENGERS` → `TAKE_OFF`
- Finds/validates LZ via `Is_LZ_Clear()` and `New_LZ()`
- Lands (`Process_Landing()`), unloads one passenger per tick via `Exit_Object()`
- Establishes radio contact (`RADIO_HELLO` → `RADIO_UNLOAD`) with each passenger
- After all unloaded: `Enter_Idle_Mode()` or take off for next LZ

### Mission_Move() → int
**Fixed-wing**: Redirects A-10 to `MISSION_RETREAT`; Cargo plane flies to Airstrip to unload.
**Rotary-wing**:
- States: `VALIDATE_LZ` → `TAKE_OFF` → `FLY_TO_LZ` → `LAND`
- Validates destination, takes off, flies, lands, then idle or queued mission

### Mission_Retreat() → int
**Fixed-wing**: Gains altitude to `FLIGHT_LEVEL`, heads West (Cargo) or friendly map edge.
**Rotary-wing**:
- States: `TAKE_OFF` → `FACE_MAP_EDGE` → `KEEP_FLYING`
- Takes off, faces friendly map edge (derived from `House->Edge`), flies off map
- Auto-deleted when leaving radar

### Mission_Enter() → int
Fly to helipad/repair bay for rearm/repair.
States: `INITIAL` → `TAKEOFF` → `ALTITUDE` → `TRAVEL` → `LANDING`
- Takes off if needed, establishes radio contact (`RADIO_CAN_LOAD` → `RADIO_HELLO` → `RADIO_DOCKING`)
- Ground controller guides via `RADIO_MOVE_HERE`
- On landing: `RADIO_IM_IN` → either `MISSION_GUARD` or `Limbo()` + attach to building

### Mission_Guard() → int
If flying: either assign move mission or `Enter_Idle_Mode()`.
If landed (AI only):
- Low health (≤ 0x0080) & money ≥ 100: seek Repair Bay (`STRUCT_REPAIR`)
- No ammo & has weapon: seek Helipad (`STRUCT_HELIPAD`) for rearm
- Has target: `MISSION_ATTACK`
- Transport (no weapon): idle
- Scatter if landed with no radio contact
- Else: delegate to `FootClass::Mission_Guard()`

### Mission_Guard_Area() → int
Similar to Guard but for area guard. If flying: `Enter_Idle_Mode()`. If landed: scatter if no contact, attack if target, else delegate to base.

---

## Enter_Idle_Mode(bool initial = false)
Determines appropriate mission when idle.

**If landed (LAYER_GROUND)**:
- Loaner with cargo: `MISSION_UNLOAD`
- Loaner empty: `MISSION_RETREAT` (remove from team)
- Owned: `MISSION_GUARD` (clear targets)

**If flying**:
- Has cargo:
  - Loaner: `MISSION_GUARD` (if team) or `MISSION_UNLOAD` (find LZ via `Good_LZ()`)
  - Owned: `MISSION_MOVE` to `Good_LZ()`
- No cargo, has weapon:
  - Loaner out of ammo: `MISSION_RETREAT`
  - Owned: seek Helipad via `Find_Docking_Bay(STRUCT_HELIPAD)` → `MISSION_ENTER`, else `MISSION_MOVE` to `Good_LZ()`
- No cargo, no weapon: `MISSION_MOVE` to `Good_LZ()`

---

## Movement & Physics

### Process_Fly_To(bool slowdown) → int
Flies toward `NavCom` (building docking coord or target coord).
- Sets `PrimaryFacing` toward destination
- If `slowdown`: reduces speed as distance decreases (`MIN(distance, 0x0300) / 3`, bounded 0x20–0xFF)
- Returns remaining distance

### Process_Take_Off() → bool
State machine for takeoff transition (landed → flying):
| Altitude | Action |
|----------|--------|
| 0 | Close door, align `PrimaryFacing` = `SecondaryFacing` |
| FLIGHT_LEVEL/2 | Set `PrimaryFacing` toward `NavCom` |
| FLIGHT_LEVEL - FLIGHT_LEVEL/3 | Set `SecondaryFacing` toward `PrimaryFacing.Desired()`, speed = 0x20 |
| FLIGHT_LEVEL - FLIGHT_LEVEL/5 | Speed = 0x40 |
| FLIGHT_LEVEL | Speed = 0xFF, `IsTakingOff = false`, return true |

### Process_Landing() → bool
State machine for landing transition (flying → landed):
| Altitude | Action |
|----------|--------|
| 0 | `IsLanding = false`, return true |
| FLIGHT_LEVEL/2 | Speed = 0 |
| FLIGHT_LEVEL | No action |

### Set_Speed(int speed)
Sets fly speed as fraction of max: `Fly_Speed(speed, min(Class->MaxSpeed * House->AirspeedBias, MPH_LIGHT_SPEED))`

### Fly_Speed, Physics
Inherited from `FlyClass`. Handles actual coordinate updates.

---

## Landing Zone (LZ) Logic

### Is_LZ_Clear(TARGET target) const → bool
Checks if target cell is clear for landing:
- Must be in radar
- Cell object must be null, or this aircraft, or current radio contact
- Cell must be `Is_Generally_Clear()`

### New_LZ(TARGET oldlz, bool stable = false) const → TARGET
If `oldlz` not clear, scans concentric rings (radius 0–15 cells) for clear cell.
- `stable = true`: starts scan from North; `false`: random facing offset
- Returns first valid cell or original target

### Good_LZ() const → TARGET
Finds friendly building to land near:
- Scans all buildings owned by same house
- Helipads (`STRUCT_HELIPAD`) get 4x priority (distance / 4)
- Returns best building center cell, or current position if none

### Cell_Seems_Ok(CELL cell, bool strict = false) const → bool
Checks airspace conflicts: no other aircraft (except self if not strict) targeting same cell or having same `NavCom`.

### Can_Enter_Cell(CELL cell, FacingType) const → MoveType
Validates landing cell:
- In radar
- `Is_Generally_Clear(true)`
- If player-owned in normal game: cell must be visible to player

---

## Combat

### Fire_At(TARGET target, int which) → BulletClass*
Calls `FootClass::Fire_At()`, then:
- Reveals terrain for human player (sight range 1)
- Plays weapon sound effect
- If bullet `IsDropping`: sets speed proportional to aircraft (40–MaxSpeed)

### Fire_Coord(int) const → COORDINATE
Returns `Coord_Move(Coord_Add(XYP_Coord(0, -Altitude), Coord), SecondaryFacing, 0x040)` — slightly ahead of aircraft body.

### Target_Coord() const → COORDINATE
Returns `Coord_Add(XYP_Coord(0, -Altitude), Coord)` — aircraft center at current altitude.

### Fire_Direction() const → DirType
Returns `SecondaryFacing.Current()`.

### Take_Damage(int& damage, int distance, WarheadType warhead, TechnoClass* source) → ResultType
- Flying aircraft (`Altitude > 0`): damage halved
- Calls `FootClass::Take_Damage()`
- On `RESULT_DESTROYED`: kills cargo (`Kill_Cargo`), announces death, creates explosion anim at `Target_Coord()`, deletes self

### Rearm_Delay(bool second) const → int
Returns `FootClass::Rearm_Delay(second) / 2` — aircraft rearm twice as fast.

### Threat_Range(int control) const → int
- `control == -1`: return -1
- `control == 0`: 20 * ICON_LEPTON_W (short)
- `control == 1`: 40 * ICON_LEPTON_W (long)
- Ignores weapon range (aircraft fly fast)

### Good_Fire_Location(TARGET target) const → TARGET
Finds firing position within weapon range:
- Scans rings from `range - 0x0180` down to `0x0180` in steps of `0x0100`
- 16 facing increments per ring
- Prefers cells visible to player (normal game), passable (`Cell_Seems_Ok`)
- Picks closest to self; randomly chooses between best and 2nd best

---

## Transport / Cargo Logic

### Pip_Count() const → int
Number of "pips" to render:
- Transporter: `How_Many()` (passenger count)
- Attack: proportional to remaining ammo (`Ammo / MaxAmmo * Max_Pips()`, minimum 1)

### Exit_Object(TechnoClass* unit) → int
Unloads passenger:
- Finds adjacent clear cell (prefers South, then sweeping)
- Unlimbos passenger at transport coord facing that cell
- Assigns `MISSION_MOVE` to that cell
- Establishes radio: `RADIO_HELLO` → `RADIO_UNLOAD`
- Returns 1 on success

### Desired_Load_Dir(ObjectClass* object, CELL& moveto) const → DirType
For loading passengers: scans adjacent cells (South ± sweep) for clear cell. Returns `DIR_N` (transport always faces North when loading).

### Is_Something_Attached(), How_Many(), Attach(), Detach_Object()
Inherited from `FootClass` (container logic).

---

## Radio Communication (Receive_Message)

| Message | Handling |
|---------|----------|
| `RADIO_PREPARED` | Ready if (landed & full ammo) or (flying & has ammo) |
| `RADIO_RUN_AWAY` | Abort landing → take off, scatter |
| `RADIO_MOVE_HERE` | If building: check `RADIO_CAN_LOAD`, then `MISSION_ENTER`; else `MISSION_MOVE` |
| `RADIO_NEED_TO_MOVE` | Ready if no `NavCom`, not taking off/landing |
| `RADIO_IM_IN` | Passenger boarded; close door if full; if civilian → `MISSION_RETREAT` |
| `RADIO_DOCKING` | If transporter with space & landed & not tethered: open door, guide passenger to staging cell via `RADIO_MOVE_HERE` → `RADIO_TETHER` → `RADIO_MOVE_HERE` |
| `RADIO_CAN_LOAD` | Accept if no radio contact, is transporter, has space, same house, landed |

---

## AI Processing

### AI()
Called each frame for non-graphic AI.
- Calls `Commence()` (mission queue processing) if not landing/taking off
- Calls `FootClass::AI()`
- Calls `Commence()` again
- Processes `PrimaryFacing` and `SecondaryFacing` rotation (rate `Class->ROT`), calls `Mark()` if changed
- Handles reinforcement delay (Modern Balance: cargo planes wait off-map until `ReinforcementStart`)
- Calls `Physics()` for movement, `Mark()` if moved
- Periodic sight revelation for human players (`SightTimer` expired, `Class->SightRange > 0`)
- Handles landing/takeoff altitude transitions:
  - Landing: `Altitude--` per frame; at 0: `IsLanding = false`, speed = 0, radio `RADIO_IM_IN` to contact
  - Takeoff: `Altitude++` per frame; at `FLIGHT_LEVEL`: `IsTakingOff = false`
  - Layer transitions: `Map.Remove`/`Submit`, `Place_Down`/`Pick_Up`, radio `RADIO_TETHER`/`RADIO_UNTETHER`
- Flags map redraw if in view
- Off-map cleanup: if `MISSION_RETREAT` and off radar → unload cargo (refund cargo plane cost, flag civilian evacuation), stun & delete

---

## Rendering

### Draw_It(int x, int y, WindowNumberType window)
Renders aircraft at screen coordinates.

**Early exit**: Modern Balance — cargo planes off-map before `ReinforcementStart` not drawn.

**Body**:
- Shape from `Class->Get_Image_Data()`, index from `UnitClass::BodyShape[Facing_To_32(SecondaryFacing)]`
- Orca at high speed (`>= MPH_MEDIUM_FAST`): shape + 32
- Door open (Chinook): shape = 32 + `Door_Stage()`
- Helicopter bobbing: `Jitter` cycles through `{0,0,0,0,1,1,1,0,0,0,0,0,-1,-1,-1,0}` added to Y

**Draw order** (virtual window draws body first for shadow layering):
- Shadow: `CC_Draw_Shape` with `SHAPE_PREDATOR|SHAPE_CENTER|SHAPE_WIN_REL|SHAPE_FADING` at (x+1, y+2)
- Body: `Techno_Draw_Object` at (x, y - Altitude + jitter)

**Rotors** (if `Class->IsRotorEquipped`):
- Landed (Altitude=0): slow idle (stage % 8 + 4), `SHAPE_GHOST`
- Flying: fast (stage % 4), `SHAPE_FADING|SHAPE_PREDATOR`
- Transport (dual rotor): two rotors offset along flight axis (`_stretch[]` per facing)
- Others: single centered rotor

Calls `FootClass::Draw_It(x, y - Altitude, window)` for attached objects.

### Mark(MarkType mark = MARK_CHANGE) → bool
Calls base `Mark()`, then refreshes `Occupy_List()` and `Overlap_List()` cells.

### Overlap_List() const → short const*
Flying: 3x3 + 2 rings (24 cells + center) — static array
Landed: `Class->Overlap_List()` (type-specific)

---

## Player Input

### Active_Click_With(ActionType action, ObjectClass* object)
- `ACTION_ENTER` → `Player_Assign_Mission(MISSION_ENTER, TARGET_NONE, object->As_Target())`
- `ACTION_SELF` → `Player_Assign_Mission(MISSION_UNLOAD)`
- Else: base class

### Active_Click_With(ActionType action, CELL cell)
- Delegates to base (attack/move handled there)

### Player_Assign_Mission(MissionType mission, TARGET target, TARGET destination)
- Plays voice response: `Response_Attack()` or `Response_Move()`
- Queues mission via `Queue_Mission()`

### What_Action(ObjectClass* target) const → ActionType
Base action, then:
- `ACTION_SELF` disabled if no cargo, flying, or tethered
- `ACTION_ATTACK` disabled if no primary weapon
- If player-owned, allied building, `RADIO_CAN_LOAD` succeeds → `ACTION_ENTER`

### What_Action(CELL cell) const → ActionType
Base action, then:
- `ACTION_MOVE`/`ACTION_ATTACK` → `ACTION_NOMOVE` if cell not visible to player
- `ACTION_ATTACK` disabled if no primary weapon

---

## Voice Responses

### Response_Attack()
Random from: `VOC_AFFIRM`, `VOC_ACKNOWL`, `VOC_YESSIR` (×3)

### Response_Move()
Random from: `VOC_MOVEOUT` (×3), `VOC_ACKNOWL`, `VOC_AFFIRM` (×2)

### Response_Select()
Random from: `VOC_VEHIC`, `VOC_UNIT`, `VOC_YESSIR` (×2), `VOC_AWAIT`

All play at `Sound_Effect(response, 0, -(Aircraft.ID(this)+1))` if `AllowVoice`.

---

## Save/Load & Scenario

### Read_INI(char* buffer)
Parses `[AIRCRAFT]` section entries:
- Format: `NNN = House,Type,Strength,Cell,Dir,Mission`
- Creates aircraft, sets strength, unlimbos at coord/dir, assigns mission
- Deletes if off-map

### Write_INI(char* buffer)
Writes all non-limbo aircraft to `[AIRCRAFT]` section:
- Format: `NNN = HouseIniName,TypeIniName,HealthRatio,Cell,PrimaryFacing,MissionName`

### Load(FileClass&) / Save(FileClass&) / Code_Pointers() / Decode_Pointers()
Standard persistence (not fully shown in source).

---

## Special Aircraft Behaviors

| Type | Class Flags | Behavior |
|------|-------------|----------|
| **TRANSPORT** (Chinook) | `IsTransporter`, `IsRotorEquipped`, dual rotor, door | Carries passengers, lands to load/unload, faces North when landed |
| **A10** | `IsFixedWing`, `IsTwoShooter` | Hunt mission: 3 attack runs, drops bombs in stream, then retreats West |
| **HELICOPTER** (Apache) | `IsRotorEquipped`, weapon | Guard/Attack missions, seeks helipad for rearm, repair bay when damaged |
| **CARGO** (C-17) | `IsFixedWing`, `IsTransporter` | Reinforcement delivery: flies from off-map to Airstrip, unloads, announces VOX, refunds cost if aborted |
| **ORCA** | `IsRotorEquipped`, weapon, special high-speed shape | Nod attack heli; shape changes at `MPH_MEDIUM_FAST` |

---

## Modern Balance Changes (Remaster)

Conditional on `Special.ModernBalance`:
- Cargo planes (`AIRCRAFT_CARGO`) delayed off-map until `ReinforcementStart` frame
- Not drawn, no physics, no AI processing while delayed
- In `AI()`: if off-map and `MISSION_RETREAT` → cleanup (refund, evacuate civilians)

---

## Debug

### Debug_Dump(MonoClass* mono) const
Displays status panel on mono monitor: name, mission, TarCom, NavCom, radio, coord, altitude, health, facings, speed, cargo, flags, timers, team, etc.

---

## Related Files

| File | Relationship |
|------|--------------|
| `AIRCRAFT.H` | Class declaration |
| `AADATA.CPP` | Aircraft type definitions (AttackPlane, TransportHeli, AttackHeli, OrcaHeli, CargoPlane) |
| `FLY.H/CPP` | FlyClass base (physics, movement) |
| `FOOT.H/CPP` | FootClass base (container, radio, missions) |
| `TECHNO.H/CPP` | TechnoClass hierarchy root |
| `BUILDING.CPP` | Helipad/Airstrip/Repair docking logic |
| `HOUSE.CPP` | Aircraft factory, totals, AI production |
| `DEFINES.H` | AircraftType enum, constants |
| `INIT.CPP` | `Aircraft.Set_Heap(AIRCRAFT_MAX)` |

---

## Key Algorithms Summary

1. **Landing/Takeoff**: Frame-by-frame altitude interpolation with layer transitions and map occupancy updates
2. **LZ Selection**: Concentric ring scan from preferred location, avoiding airspace conflicts
3. **Fixed-wing Attack**: 3-run bomb stream with homing, fixed retreat direction
4. **Rotary-wing Attack**: Position → face → two-shot burst → reposition
5. **Transport Unload**: Radio handshake per passenger, staged exit cells
6. **Idle Decision**: Context-aware (loaner/owned, cargo/weapon, ammo/health, building availability)
7. **Rendering**: Altitude offset, rotor animation, bobbing, dual-rotor geometry, door state