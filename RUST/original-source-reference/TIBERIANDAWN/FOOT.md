# PATH/FILE
original/TIBERIANDAWN/FOOT.CPP

## Purpose
Base class for all ground-traveling objects in Tiberian Dawn. FootClass extends TechnoClass with ground-specific movement physics, pathfinding (NavCom), cell occupation, and mission AI for ground units (vehicles and infantry).

## Where it fits in the game
Intermediate class in the entity hierarchy: `ObjectClass` → `TechnoClass` → `FootClass` → `UnitClass`/`InfantryClass`. All ground-traveling entities (tanks, harvesters, infantry on foot) inherit from FootClass. Aircraft also inherit from FootClass but override ground-specific behavior.

## Classes, structs, enums and important data

### FootClass
**Core instance class** - ground-traveling object with NavCom pathfinding, cell occupation, and ground mission AI.

**Key data members:**
- `Speed` (unsigned char, 0-255) - Current throttle (0=stop, 255=full)
- `IsDriving` (bool) - Currently moving under physics
- `IsInitiated` (bool) - Pathfinding initiated
- `IsPlanningToLook` (bool) - Planning to rotate to face target
- `IsDeploying` (bool) - Deploying into structure (MCV, etc.)
- `IsNewNavCom` (bool) - New navigation command issued
- `IsFiring` (bool) - Currently firing weapon
- `IsRotating` (bool) - Rotating to face target
- `IsUnloading` (bool) - Unloading passengers/cargo
- `IsDeploying` (bool) - Deploying into structure
- `IsNewNavCom` (bool) - New navigation command
- `IsPlanningToLook` (bool) - Planning to face target
- `IsFiring` (bool) - Currently firing
- `IsRotating` (bool) - Rotating to face
- `IsUnloading` (bool) - Unloading passengers
- `NavCom` (TARGET) - Navigation computer target (destination)
- `SuspendedNavCom` (TARGET) - Suspended nav target (for interruptions)
- `SuspendedMission` (MissionType) - Suspended mission during interruption
- `Path[13]` (FacingType array) - Path steps (13 steps, FACING_NONE = empty)
- `HeadToCoord` (COORDINATE) - Direct coordinate to head toward
- `Member` (int) - Team member index
- `Team` (TeamClass*) - Team membership pointer
- `PathDelay` (int) - Delay between path steps
- `TryTryAgain` (int) - Path retry counter (PATH_RETRY default)
- `Group` (int) - Group assignment (-1 = none)
- `ArchiveTarget` (TARGET) - Archived target for Mission_Enter
- `SuspendedNavCom` (TARGET) - Suspended nav target
- `SuspendedMission` (MissionType) - Suspended mission
- `Path[13]` - Path step array (13 facings)
- `HeadToCoord` (COORDINATE) - Direct coordinate target
- `Member` (int) - Team member index
- `Team` (TeamClass*) - Team pointer
- `PathDelay` (int) - Delay between path steps
- `TryTryAgain` (int) - Retry counter (PATH_RETRY = 3 default)
- `Group` (int) - Group assignment (-1 = none)
- `ArchiveTarget` (TARGET) - Archived target for Mission_Enter

**Constructor state:**
- `IsDriving = false`
- `IsInitiated = false`
- `IsPlanningToLook = false`
- `IsDeploying = false`
- `IsNewNavCom = false`
- `IsFiring = false`
- `IsRotating = false`
- `IsUnloading = false`
- `NavCom = TARGET_NONE`
- `SuspendedNavCom = TARGET_NONE`
- `Path[0] = FACING_NONE`
- `HeadToCoord = NULL`
- `Member = 0`
- `Team = NULL`
- `PathDelay = 0`
- `TryTryAgain = PATH_RETRY (3)`
- `Group = -1`

**House unit count tracking:**
- Constructor: `House->CurUnits++`
- Destructor: `House->CurUnits--`

### Path array
- `Path[13]` - 13 FacingType steps (FACING_NONE = empty)
- `Path[0] = FACING_NONE` means no path

### Debug display (CHEAT_KEYS)
- Team name + ID, ArchiveTarget, NavCom, Speed
- Path steps as characters: "-", "0"-"7"
- HeadToCoord, Deploying/Rotating/Driving/Firing/PlanningToLook flags

## Functions and methods

### Constructors/Destructor

**FootClass::FootClass()** - Default constructor
- Initializes all state to defaults
- `House->CurUnits++` if House exists

**FootClass::FootClass(house)** - Normal constructor
- Calls `TechnoClass(house)`
- Initializes all FootClass-specific members
- `House->CurUnits++`

**FootClass::~FootClass()** - Destructor
- `House->CurUnits--` if GameActive and House exists

### Movement & Physics

**FootClass::Set_Speed(speed)** - Set throttle (0-255)
- Clamps to 0-255 via `speed & 0xFF`
- Direct velocity control for momentum physics

**FootClass::Basic_Path(from_cell, to_cell, speed)** → bool
- **Core ground pathfinding** for vehicles and infantry
- Finds path from from_cell to to_cell at given speed
- Returns true if path found, populates Path[] array

**FootClass::Start_Driver()** - Initiates movement physics
- Called after path found to start physics processing

**FootClass::Stop_Driver()** - Clears driving state
- Clears IsDriving, Path[0] = FACING_NONE

**FootClass::Set_Speed(speed)** - Throttle control (0-255)
- Clamps to 0-255

### Navigation & Pathfinding

**FootClass::Assign_Destination(target)** - Set NavCom target
- Simple assignment: `NavCom = target`

**FootClass::Assign_Mission(mission)** - Assign ground mission
- Sets Mission, clears NavCom if needed

**FootClass::Assign_Destination(target)** - Sets NavCom = target

**FootClass::Approach_Target(target)** → bool
- Sets NavCom to approach target object
- Returns true if successful

**FootClass::Active_Click_With(target)** - Left-click action
- Initiates attack or move based on target type

**FootClass::Active_Click_With(target)** - Left-click action
- Attack or move based on target

**FootClass::Approach_Target(target)** → bool
- Sets navigation to approach target object

### Cell Entry & Movement Validation

**FootClass::Can_Enter_Cell(cell, facing)** → MoveType
- Base implementation: always returns MOVE_OK
- Overridden by UnitClass/InfantryClass for terrain/occupancy checks
- **Critical for pathfinding** - determines cell passability

**FootClass::Can_Enter_Cell(cell, facing)** → MoveType
- Base: always MOVE_OK
- Overridden by derived classes

**FootClass::Can_Demolish()** → bool
- Can sell back only on repair bay (STRUCT_REPAIR)
- Must be in radio contact, distance < 0x80 (128 leptons = 0.5 cells)
- Units and aircraft only

**FootClass::Sell_Back(control)** - Sell object for refund
- control=1: refund money, play cash sound, Stun(), Limbo(), Delete_This()

**FootClass::Can_Demolish()** → bool
- Only on repair bay (STRUCT_REPAIR), distance < 128 leptons
- Units and aircraft only

**FootClass::Sell_Back(control)** - Sell for refund
- control=1: plays VOX_CASHTURN, refunds Refund_Amount(), Stun(), Limbo(), Delete_This()

### Mission AI (Ground-specific)

**FootClass::Mission_Move()** → int
- **Core movement AI** for ground units
- Processes NavCom path, handles obstacles, calls Basic_Path
- Returns ticks until next call

**FootClass::Mission_Attack()** → int
- Attack AI: approach target, fire when in range
- Handles rotation, firing, approach

**FootClass::Mission_Guard_Area()** → int
- Guards area ~2x weapon range
- Calls Greatest_Threat() for targets

**FootClass::Mission_Hunt()** → int
- Default hunt order: seek and destroy

**FootClass::Mission_Guard_Area()** → int
- Guards area ~2x weapon range

**FootClass::Mission_Hunt()** → int
- Seek and destroy behavior

**FootClass::Mission_Attack()** → int
- Approach target, fire when in range

**FootClass::Mission_Harvest()** → int
- Harvester AI: find tiberium, mine, return to refinery

**FootClass::Mission_Enter()** → int
- **Cooperative enter mission** (APC loading, repair bay entry)
- Coordinates with transporter via RADIO_DOCKING handshake
- Returns TICKS_PER_SECOND/2 (30 ticks)

**FootClass::Mission_Enter()** → int
- Cooperative enter (APC loading, repair bay)
- RADIO_DOCKING handshake with transporter
- Returns 30 ticks (TICKS_PER_SECOND/2)

**FootClass::Mission_Capture()** → int
- Engineer/Commando building capture logic

**FootClass::Mission_Harvest()** → int
- Harvester: find tiberium, mine, return to refinery

**FootClass::Mission_Capture()** → int
- Engineer/Commando building capture

**FootClass::Mission_Guard_Area()** → int
- Guards area ~2x weapon range

**FootClass::Mission_Hunt()** → int
- Seek and destroy

### Radio Messages (Ground-specific)

**FootClass::Receive_Message(from, message, param)** → RadioMessageType
- **RADIO_ON_DEPOT**: Returns ROGER if on repair depot (STRUCT_REPAIR)
- **RADIO_REPAIR**: NEGATIVE if moving (NavCom legal), else passes to TechnoClass
- **RADIO_RUN_AWAY**: Aborts current mission, scatters if not rotating
- **RADIO_NEED_TO_MOVE**: Returns ROGER if not moving, else NEGATIVE
- **RADIO_MOVE_HERE**: Assigns destination, handles guard→move transition
- **RADIO_TRYING_TO_LOAD**: Handles cooperative loading

**FootClass::Receive_Message(from, message, param)** → RadioMessageType
- RADIO_ON_DEPOT: ROGER if on STRUCT_REPAIR
- RADIO_REPAIR: NEGATIVE if NavCom legal
- RADIO_RUN_AWAY: Aborts mission, scatters
- RADIO_NEED_TO_MOVE: ROGER if not moving
- RADIO_MOVE_HERE: Assigns destination, guard→move
- RADIO_TRYING_TO_LOAD: Cooperative loading

### Marking & Rendering

**FootClass::Mark(mark)** → bool
- Calls TechnoClass::Mark(), then handles map updates:
- MARK_UP: Map.Pick_Up() (except fixed-wing aircraft)
- MARK_DOWN: Map.Place_Down() (except fixed-wing aircraft)
- MARK_CHANGE: Map.Refresh_Cells() for overlap/occupy lists

**FootClass::Mark(mark)** → bool
- Calls TechnoClass::Mark(), then map updates:
- MARK_UP: Map.Pick_Up() (non-fixed-wing)
- MARK_DOWN: Map.Place_Down() (non-fixed-wing)
- MARK_CHANGE: Map.Refresh_Cells()

### Damage & Destruction

**FootClass::Take_Damage(damage, distance, warhead, source)** → ResultType
- Overrides ObjectClass::Take_Damage
- Calls base, then handles death announcement

**FootClass::Death_Announcement(source)** - VOX announcements
- Multiplayer: human houses, discovered, or owned
- Civilian infantry: VOX_DEAD_CIV
- Enemy units: VOX_ENEMY_UNIT
- Own units: VOX_UNIT_LOST / VOX_DEAD_GDI / VOX_DEAD_NOD
- Rambo exception: no announcement

**FootClass::Death_Announcement(source)** - VOX
- MP: human houses, discovered, owned
- Civilian: VOX_DEAD_CIV
- Enemy: VOX_ENEMY_UNIT
- Own: VOX_UNIT_LOST / VOX_DEAD_GDI / VOX_DEAD_NOD
- Rambo: no announcement

**FootClass::Death_Announcement(source)** - VOX
- MP: human houses, discovered, owned
- Civilian: VOX_DEAD_CIV
- Enemy: VOX_ENEMY_UNIT
- Own: VOX_UNIT_LOST / VOX_DEAD_GDI / VOX_DEAD_NOD
- Rambo: no announcement

**FootClass::Take_Damage(damage, distance, warhead, source)** → ResultType
- Calls base Take_Damage
- Triggers Death_Announcement on death

**FootClass::Stun()** - Prepares for removal
- Sets up for deletion

**FootClass::Limbo()** → bool
- Intercepts limbo: sets Path[0]=FACING_NONE, clears NavCom
- Calls TechnoClass::Limbo()

**FootClass::Unlimbo(coord, dir)** → bool
- Calls TechnoClass::Unlimbo(), then fixes up state

**FootClass::Limbo()** → bool
- Path[0]=FACING_NONE, NavCom=TARGET_NONE, then TechnoClass::Limbo()

**FootClass::Unlimbo(coord, dir)** → bool
- Calls TechnoClass::Unlimbo(), then fixes up

**FootClass::Detach(target, all)** - Detach target from tracking
- Calls TechnoClass::Detach()
- Clears ArchiveTarget, SuspendedNavCom if matching
- Clears NavCom/Path if targeting
- Human: clears Path[0] if targeting

**FootClass::Detach(target, all)** - Detach target
- Clears ArchiveTarget, SuspendedNavCom
- Clears NavCom/Path if targeting
- Human: clears Path[0]

**FootClass::Detach_All(all)** - Remove from all systems
- Team->Remove(this), Team=NULL
- Calls TechnoClass::Detach_All()

**FootClass::Detach_All(all)** - Remove from all
- Team->Remove(), Team=NULL
- TechnoClass::Detach_All()

### Utility & Helper

**FootClass::Offload_Tiberium_Bail()** → int
- Returns tiberium credits to offload per step
- Base returns 0 (overridden by harvester)

**FootClass::Likely_Coord()** → COORDINATE
- Returns HeadToCoord if set, else Target_Coord()
- Used for approach prediction

**FootClass::Sort_Y()** → int
- Returns sort coordinate for rendering order

**FootClass::Rescue_Mission(target)** → int
- Returns threat value for rescue AI
- Returns 0 if target illegal, already attacking, or busy
- Returns -Risk() if already targeting
- Returns 0 if target has weapon (will destroy us)
- Returns 0 if in team, harvesting, or no risk
- Calculates distance penalty: threat = Risk*256 / max(dist/speed, 1)

**FootClass::Greatest_Threat(method)** → TARGET
- Cloaked human units on GUARD don't auto-target
- Adds THREAT_AIR if primary/secondary is anti-air
- Calls TechnoClass::Greatest_Threat(method|THREAT_GROUND)

**FootClass::Basic_Path(from_cell, to_cell, speed)** → bool
- Core ground pathfinding for vehicles/infantry
- Finds path between cells at speed

**FootClass::Can_Enter_Cell(cell, facing)** → MoveType
- Base: MOVE_OK (overridden by UnitClass/InfantryClass)

**FootClass::Can_Demolish()** → bool
- Only on repair bay (STRUCT_REPAIR), distance < 128 leptons
- Units and aircraft only

**FootClass::Sell_Back(control)** - Sell for refund
- control=1: VOX_CASHTURN, Refund_Amount(), Stun(), Limbo(), Delete_This()

**FootClass::Offload_Tiberium_Bail()** → int
- Base returns 0 (overridden by harvester)

**FootClass::Can_Enter_Cell(cell, facing)** → MoveType
- Base: MOVE_OK (overridden)

**FootClass::Can_Demolish()** → bool
- Repair bay only, distance < 128 leptons

**FootClass::Sell_Back(control)** - Refund, Stun, Limbo, Delete

**FootClass::Likely_Coord()** → COORDINATE
- HeadToCoord if set, else Target_Coord()

**FootClass::Sort_Y()** → int
- Render sort coordinate

**FootClass::Rescue_Mission(target)** → int
- Threat calculation for rescue AI
- Distance penalty: threat = Risk*256 / max(dist/speed, 1)

**FootClass::Greatest_Threat(method)** → TARGET
- Cloaked human on GUARD: no auto-target
- Adds THREAT_AIR for anti-air weapons
- Calls base with THREAT_GROUND

**FootClass::Rescue_Mission(target)** → int
- Threat calculation for rescue
- Distance penalty: threat = Risk*256 / max(dist/speed, 1)

**FootClass::Greatest_Threat(method)** → TARGET
- Cloaked human on GUARD: no auto-target
- Adds THREAT_AIR for anti-air weapons

### Radio Messages (Ground-specific)

**FootClass::Receive_Message(from, message, param)** → RadioMessageType
- **RADIO_ON_DEPOT**: ROGER if on STRUCT_REPAIR building
- **RADIO_REPAIR**: NEGATIVE if NavCom legal (moving)
- **RADIO_RUN_AWAY**: Abort mission, scatter if not rotating
- **RADIO_NEED_TO_MOVE**: ROGER if not moving, else NEGATIVE
- **RADIO_MOVE_HERE**: Assign destination, guard→move transition
- **RADIO_TRYING_TO_LOAD**: Handles cooperative loading

### Debug (CHEAT_KEYS)

**FootClass::Debug_Dump(mono)** - Mono monitor display
- Team name/ID, ArchiveTarget, NavCom, Speed
- Path steps as "-"/"0"-"7"
- HeadToCoord, Deploying/Rotating/Driving/Firing/PlanningToLook flags
- Aircraft special case handling

## Inputs, outputs and side effects

**State mutations:**
- NavCom/Path/HeadToCoord assignment
- IsDriving/IsInitiated/IsDeploying/IsRotating/IsFiring state
- Team membership (Team, Member)
- House->CurUnits increment/decrement
- Path array manipulation
- ArchiveTarget/SuspendedNavCom/SuspendedMission for interruptions

**External systems affected:**
- Map: Mark(), Pick_Up/Place_Down/Refresh_Cells()
- Team: Team->Remove(), Team assignment
- House: CurUnits++, Refund_Money(), CurUnits--
- TeamClass: Remove(), team coordination
- Radio: Receive_Message() handling

## Calls into / called by

**Called by:**
- TechnoClass AI: Mission_Move, Mission_Attack, Mission_Enter, etc.
- Player input: Active_Click_With, Assign_Destination, Assign_Mission
- Radio system: Receive_Message()
- Construction: Unlimbo, Limbo
- Team system: Team->Remove(), Team assignment
- Selection: Active_Click_With, Sell_Back

**Calls into:**
- TechnoClass: Mark(), Detach(), Receive_Message(), Take_Damage(), Limbo(), Unlimbo()
- MapClass: Pick_Up(), Place_Down(), Refresh_Cells(), Place_Down(), Pick_Up()
- TeamClass: Remove(), coordination
- HouseClass: CurUnits++, CurUnits--, Refund_Money()
- MapClass: Pick_Up(), Place_Down(), Refresh_Cells()
- TeamClass: Remove()
- HouseClass: CurUnits++, CurUnits--, Refund_Money()
- Sound: Speak() for death announcements

## Related original files
- **FOOT.H** - Class declarations, inline helpers
- **TECHNO.CPP/.H** - Parent TechnoClass
- **UNIT.CPP/.H** - UnitClass (ground vehicles)
- **INFANTRY.CPP/.H** - InfantryClass (foot soldiers)
- **AIRCRAFT.CPP/.H** - AircraftClass (inherits but overrides)
- **TEAM.CPP/.H** - TeamClass (team coordination)
- **MAP.CPP/.H** - MapClass (pathfinding, cells)
- **CELL.CPP/.H** - CellClass (Can_Enter_Cell, occupation)
- **TEAM.CPP/.H** - TeamClass
- **MAP.CPP/.H** - MapClass
- **CELL.CPP/.H** - CellClass
- **FUNCTION.H** - Core includes, Distance()
- **VOX.H** - Voice announcements
- **RADIO.CPP** - Radio messaging

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/entity/foot.rs` - FootClass base
- `cnc-tiberian-dawn/src/entity/td_foot.rs` - TD-specific foot
- `cnc-core/src/entity/unit.rs` - UnitClass (ground vehicles)
- `cnc-core/src/entity/infantry.rs` - InfantryClass
- `cnc-core/src/ai/mission.rs` - Mission_Move, Mission_Attack, Mission_Enter
- `cnc-core/src/ai/pathfinding.rs` - Basic_Path, NavCom, Path[]
- `cnc-core/src/entity/ground_movement.rs` - Set_Speed, IsDriving, physics
- `cnc-core/src/entity/cell_entry.rs` - Can_Enter_Cell, cell occupation
- `cnc-core/src/ai/threat.rs` - Rescue_Mission, Greatest_Threat
- `cnc-core/src/combat/damage.rs` - Take_Damage override
- `cnc-core/src/entity/sell.rs` - Sell_Back, Can_Demolish
- `cnc-core/src/entity/transport.rs` - Mission_Enter, RADIO_DOCKING
- `cnc-core/src/ai/rescue.rs` - Rescue_Mission threat calculation

**Not yet ported:**
- Full ground mission AI (Mission_Move, Mission_Attack, Mission_Enter, etc.)
- NavCom pathfinding (Path[13], Basic_Path, HeadToCoord)
- Ground movement physics (Set_Speed, IsDriving, momentum)
- Cell entry validation (Can_Enter_Cell overrides)
- Radio message handling (RADIO_ON_DEPOT, RADIO_REPAIR, etc.)
- Team coordination (Team, Member, coordination)
- Death announcements (VOX_* per house)
- Harvester-specific logic (Mission_Harvest)
- Deployment (IsDeploying, MCV, etc.)
- Pathfinding integration (Path[13], HeadToCoord, Basic_Path)

## Edge cases and compatibility notes

1. **House unit count**: CurUnits incremented in constructors, decremented in destructor. Critical for AI build limits.

2. **Aircraft inheritance**: AircraftClass inherits FootClass but overrides Mark() for fixed-wing (never Pick_Up/Place_Down). Debug_Dump skips ground-specific fields for aircraft.

3. **Path array**: 13 steps max. Path[0] = FACING_NONE means no path. HeadToCoord for direct coordinate targeting.

4. **TryTryAgain = PATH_RETRY (3)**: Path retry counter for pathfinding failures.

5. **Group = -1**: No group assignment by default.

6. **ArchiveTarget**: Used by Mission_Enter for cooperative enter when radio contact lost.

6. **SuspendedNavCom/SuspendedMission**: For interrupting missions (RADIO_RUN_AWAY, etc.)

7. **Rescue_Mission threat calculation**: 
   - Distance penalty: threat = Risk*256 / max(dist/speed, 1)
   - Returns -Risk if already targeting
   - Returns 0 if target has weapon (will destroy us)
   - Returns 0 if in team, harvesting, or no risk

7. **Greatest_Threat**: Cloaked human on GUARD returns NONE (cloaking useless if auto-targets). Adds THREAT_AIR for anti-air weapons.

8. **Can_Enter_Cell**: Base returns MOVE_OK. UnitClass/InfantryClass override for terrain/occupancy.

9. **Can_Demolish**: Only on repair bay (STRUCT_REPAIR), radio contact, distance < 128 leptons.

10. **Sell_Back**: control=1 triggers VOX_CASHTURN, Refund_Amount(), Stun(), Limbo(), Delete_This().

11. **Death_Announcement**: Multiplayer: human houses, discovered, or owned. Civilian: VOX_DEAD_CIV. Enemy: VOX_ENEMY_UNIT. Own: VOX_UNIT_LOST / VOX_DEAD_GDI / VOX_DEAD_NOD. Rambo: silent.

11. **Death_Announcement**: MP: human/discovered/owned. Civilian: VOX_DEAD_CIV. Enemy: VOX_ENEMY_UNIT. Own: VOX_UNIT_LOST/VOX_DEAD_GDI/VOX_DEAD_NOD. Rambo: silent.

12. **FootClass::Mark()**: Aircraft fixed-wing never Pick_Up/Place_Down (never land).

12. **Mark()**: Aircraft fixed-wing never Pick_Up/Place_Down.

13. **Can_Enter_Cell**: Base returns MOVE_OK. UnitClass/InfantryClass override for terrain/occupancy/blocking.

14. **Radio messages**: 
    - RADIO_ON_DEPOT: ROGER if on STRUCT_REPAIR
    - RADIO_REPAIR: NEGATIVE if NavCom legal (moving)
    - RADIO_RUN_AWAY: Abort, scatter if not rotating
    - RADIO_NEED_TO_MOVE: ROGER if not moving
    - RADIO_MOVE_HERE: Assign dest, guard→move
    - RADIO_TRYING_TO_LOAD: Cooperative loading

14. **Radio**: ON_DEPOT (repair bay), REPAIR (moving?), RUN_AWAY (abort), NEED_TO_MOVE (moving?), MOVE_HERE (dest), TRYING_TO_LOAD (coop)

15. **Mission_Enter**: Cooperative loading (APC, repair bay). RADIO_DOCKING handshake. Returns 30 ticks.

15. **Mission_Enter**: Cooperative enter (APC, repair). RADIO_DOCKING handshake. 30 ticks.

16. **Death_Announcement**: MP: human/discovered/owned. Civilian: VOX_DEAD_CIV. Enemy: VOX_ENEMY_UNIT. Own: VOX_UNIT_LOST/VOX_DEAD_GDI/VOX_DEAD_NOD. Rambo: silent.

17. **Constructor/destructor**: House->CurUnits++/-- for AI build limits.

18. **Group = -1**: No group assignment default.

19. **TryTryAgain = PATH_RETRY (3)**: Path retry counter.

20. **ArchiveTarget**: For Mission_Enter when radio contact lost.

21. **SuspendedNavCom/SuspendedMission**: For interruptions.

22. **Rescue_Mission**: Distance penalty = Risk*256 / max(dist/speed, 1). Returns -Risk if already targeting. Returns 0 if target armed, in team, harvesting, or no risk.

21. **Greatest_Threat**: Cloaked human on GUARD = no auto-target (cloaking useless otherwise). Adds THREAT_AIR for anti-air weapons.

22. **Can_Demolish**: Repair bay (STRUCT_REPAIR), radio contact, distance < 128 leptons (0.5 cells).

23. **Sell_Back**: control=1 → VOX_CASHTURN, Refund_Amount(), Stun(), Limbo(), Delete_This().

23. **Sell_Back**: control=1 → VOX_CASHTURN, Refund, Stun, Limbo, Delete.

24. **Multiplayer death announcements**: Human houses, discovered, or owned players get announcements.

24. **MP death**: Human houses, discovered, or owned.

25. **Rambo exception**: No death announcement for Rambo infantry.

25. **Rambo**: Silent death.

26. **House->CurUnits**: Tracked for AI build limits. ++ in constructors, -- in destructor.

26. **CurUnits**: AI build limit tracking.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/FOOT.CPP at f1f0d42
