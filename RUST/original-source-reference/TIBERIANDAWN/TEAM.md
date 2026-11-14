# PATH/FILE
original/TIBERIANDAWN/TEAM.CPP

## Purpose
Team management system for Tiberian Dawn. Implements coordinated group behavior for AI units: recruitment, movement coordination, attack coordination, regrouping, loading/unloading, and mission execution.

## Where it fits in the game
Team AI system - coordinates groups of units (vehicles, infantry, aircraft) to execute complex missions. Each team has a type definition (TeamTypeClass) defining its composition and mission list. Teams are the primary AI coordination mechanism for computer-controlled forces.

## Classes, structs, enums and important data

### TeamClass
**Core team instance** - manages a group of FootClass members (units, infantry, aircraft) executing a mission list.

**Key data members:**
- `Class` (TeamTypeClass*) - Team type definition
- `House` (HouseClass*) - Owning house/player
- `Member` (FootClass*) - Head of member linked list
- `Quantity[TEAMTYPE_MAX]` (unsigned char[]) - Current count per unit type
- `Class->DesiredNum[]` - Desired count per unit type
- `Total` (int) - Total members
- `MissionTarget` (TARGET) - Mission objective target
- `Target` (TARGET) - Current target
- `MissionTarget` (TARGET) - Mission objective
- `Target` (TARGET) - Current target
- `Center` (CELL) - Team center cell
- `ObjectiveCenter` (CELL) - Objective center
- `Member` (FootClass*) - Head of member list
- `Total` (int) - Total members
- `Risk` (int) - Team risk value
- `CurrentMission` (int) - Current mission index
- `IsNextMission` (bool) - Advance to next mission
- `TimeOut` (int) - Mission timeout
- `SuspendTimer` - Suspension timer
- `Suspended` (bool) - Team suspended
- `Number[TEAMTYPE_MAX]` - Global team count per type
- `Success[TEAMTYPE_MAX]` - Success rating per type

**State flags:**
- `IsAltered` (bool) - Composition changed
- `IsForcedActive` (bool) - Forced activation
- `IsFullStrength` (bool) - At desired strength
- `IsUnderStrength` (bool) - Below desired strength
- `IsReforming` (bool) - Regrouping
- `IsLagging` (bool) - Members lagging
- `IsMoving` (bool) - Executing move mission
- `IsHasBeen` (bool) - Ever reached full strength
- `IsMoving` (bool) - Moving
- `IsUnderStrength` (bool) - Below strength
- `IsReforming` (bool) - Regrouping
- `IsLagging` (bool) - Lagging units
- `IsForcedActive` (bool) - Forced activation
- `IsFullStrength` (bool) - At full strength
- `IsUnderStrength` (bool) - Below strength
- `IsReforming` (bool) - Regrouping
- `IsLagging` (bool) - Lagging
- `IsForcedActive` (bool) - Forced active
- `IsFullStrength` (bool) - At full strength
- `IsUnderStrength` (bool) - Under strength
- `IsReforming` (bool) - Reforming
- `IsLagging` (bool) - Lagging
- `IsForcedActive` (bool) - Forced active

**TeamTypeClass (static)**
- `ClassCount` (int) - Number of unit types in team
- `Class[]` - Unit type array
- `DesiredNum[]` - Desired count per type
- `MissionCount` (int) - Number of missions
- `MissionList[]` - Mission list
- `IsReinforcable` (bool) - Can be reinforced
- `IsTransient` (bool) - Delete when empty
- `IsSuicide` (bool) - Suicide team
- `RecruitPriority` (int) - Recruitment priority

### TeamMissionStruct
- `Mission` (TeamMissionType) - Mission type
- `Argument` (int) - Mission argument (cell, waypoint, etc.)

### TeamMissionType enum
| Mission | Description |
|---------|-------------|
| TMISSION_MOVECELL | Move to cell |
| TMISSION_MOVE | Move to target |
| TMISSION_UNLOAD | Unload passengers |
| TMISSION_ATTACKTARCOM | Attack target |
| TMISSION_ATTACKBASE | Attack base |
| TMISSION_ATTACKUNITS | Attack units |
| TMISSION_MISSILE | Missile strike |

### Global arrays
- `Number[TEAMTYPE_MAX]` - Team count per type
- `Success[TEAMTYPE_MAX]` - Success rating per type

### Constants
- `TEAM_MAX` - Max teams
- `TEAMTYPE_MAX` - Max team types
- `TEAMTYPE_MAX` - Max team types
- `STRAY_DISTANCE` - Max distance from center
- `TICKS_PER_MINUTE` - Ticks per minute

## Functions and methods

### TeamClass

**TeamClass::TeamClass(TeamTypeClass*, HouseClass*)** - Constructor
- Initializes all state flags
- Increments `Number[TeamTypes.ID(Class)]`

**TeamClass::~TeamClass()** - Destructor
- Decrements `Number[]`
- Removes all members
- Deletes TeamTypeClass if transient and last team

**TeamClass::Init()** - System initialization
- `Teams.Free_All()`, clears `Number[]`, `Success[]`

**TeamClass::operator new/delete()** - Pool allocation

**TeamClass::~TeamClass()** - Destructor
- Decrements type count
- Removes all members
- Deletes TeamTypeClass if transient and last

**TeamClass::Validate()** (CHEAT_KEYS)
- Validates index in Teams array

**TeamClass::Assign_Mission_Target(new_target)** - Sets mission target
- Clears member TarCom/NavCom if pointing to old target
- Updates MissionTarget/Target

**TeamClass::AI()** - **Main team AI** (called per frame)
- Handles suspension timer
- **Recalculates strength** if `IsAltered`:
  - Sums `Class->DesiredNum[]`
  - Human teams always full strength
  - Reinforcable: under strength if `Total <= desired/3`
  - Non-reinforcable: under strength if never `IsHasBeen`
  - Sets `IsReforming` if transitioned from under-strength
- **Under-strength handling**: Sets regroup target (prefers repair bays)
- **Full strength activation**: Sets `IsMoving=true`, `IsHasBeen=true`, initiates members
- **Center calculation**: `Calc_Center()` for average position
- **Recruitment**: Scans for needed unit types if under-strength
- **Auto-delete**: If no members and `IsHasBeen`, deletes team
- **Mission advancement**: Advances `CurrentMission` when `IsNextMission`
- **Mission execution**: Dispatches to mission handlers

**TeamClass::Calc_Center(center, obj_center)** - Average member position
- Averages coordinates of initiated members
- Sets `obj_center` to closest to target

**TeamClass::AI()** - **Main AI loop**
1. Checks suspension timer
2. Recalculates strength if `IsAltered`
3. Under-strength: regroup at repair bay
6. Full strength: activates, initiates members
8. Recruits needed types
9. Auto-delete if empty and `IsHasBeen`
10. Advances mission if `IsNextMission`
11. Executes current mission

**TeamClass::Assign_Mission_Target(new_target)**
- Clears member TarCom/NavCom pointing to old target
- Sets `MissionTarget` and optionally `Target`

**TeamClass::AI()** - Main AI loop
1. Checks suspension timer
2. Recalculates strength if altered
3. Under-strength: regroup at repair bay
6. Full strength: activates, initiates members
8. Recruits needed types
10. Advances mission if `IsNextMission`
11. Executes current mission

**TeamClass::Recruit(typeindex)** → int
- Recruits units/infantry of `Class->Class[typeindex]`
- Scans Infantry/Units arrays for matching house/type
- Calls `Add()` to add to team
- Returns count added

**TeamClass::Add(unit, typeindex)** → bool
- Adds FootClass to member list
- Increments `Quantity[typeindex]`, `Total`
- Sets `unit->Team = this`, `unit->IsInitiated = false`
- Returns true if added

**TeamClass::Remove(obj)** → bool
- Unlinks from member list
- Updates `Total`, `Risk`
- Chooses new captain if no initiated members remain
- Sets `IsAltered = true`

**TeamClass::Remove(obj)** → bool
- Unlinks from member list
- Updates `Total`, `Risk`
- Chooses new captain if no initiated members
- Sets `IsAltered = true`

**TeamClass::Detach(target, all)** - Clears targets
- Clears `Target`/`MissionTarget` if matching

**TeamClass::As_Target()** → TARGET
- Returns `Build_Target(KIND_TEAM, Teams.ID(this))`

**TeamClass::Calc_Center(center, obj_center)** - Average member position
- Averages coordinates of initiated members
- Sets `obj_center` to closest to target

**TeamClass::Took_Damage(obj, result, source)** - Damage reaction
- If not moving: should hide
- If moving: may retarget to source if source has weapon

**TeamClass::Coordinate_Attack()** - Attack coordination
- Assigns `MISSION_ATTACK` to members
- Sets `Target` for all initiated members

**TeamClass::Coordinate_Regroup()** → bool
- Moves stray members (> `STRAY_DISTANCE`) to center
- Returns false if any member not at center

**TeamClass::Coordinate_Move()** - Movement coordination
- Assigns `MISSION_MOVE` to lagging units
- Sets destination to target
- Advances mission if all arrived

**TeamClass::Coordinate_Unload()** - Unload coordination
- Orders transports to unload
- Loaner transports break off team
- Advances mission when done

**TeamClass::Coordinate_Conscript(unit)** - New member orders
- Moves to center if far
- Marks `IsInitiated` when close

**TeamClass::Is_A_Member(who)** → bool
- Checks if object in member list

**TeamClass::Suspend_Teams(priority)** - Suspends low-priority teams
- Destroys teams with `RecruitPriority < priority`
- Sets `SuspendTimer = 2 minutes`, `Suspended = true`

**TeamClass::Validate()** (CHEAT_KEYS)
- Validates index in Teams array

**TeamClass::operator new/delete()** - Pool allocation

**TeamClass::~TeamClass()** - Destructor
- Decrements `Number[]`
- Removes all members
- Deletes TeamTypeClass if transient and last

**TeamClass::operator new/delete()** - Pool allocation

**TeamClass::VTable** - Virtual function table pointer

### Static arrays
- `Number[TEAMTYPE_MAX]` - Team count per type
- `Success[TEAMTYPE_MAX]` - Success rating per type

## Inputs, outputs and side effects

**State mutations:**
- Member list modifications
- Strength flags (`IsUnderStrength`, `IsFullStrength`, `IsReforming`)
- Mission progression (`CurrentMission`, `IsNextMission`)
- Target assignments
- Center calculation

**External systems affected:**
- Member units: mission assignments, targets, destinations
- House: team coordination
- Map: threat calculations, cell distances

## Calls into / called by

**Called by:**
- LogicClass::AI() - per-frame team AI
- Trigger actions: CREATE_TEAM, DESTROY_TEAM, REINFORCEMENTS
- House: Suggested_New_Team()
- Unit/Infantry events: Took_Damage, removal

**Calls into:**
- FootClass/UnitClass/InfantryClass: mission assignment
- MapClass: Cell_Distance, Cell_Threat, Cell_Region
- HouseClass: RecruitPriority, IsHuman, IsHuman
- MapClass: Cell_Distance, Cell_Threat, Cell_Region
- Random_Pick(): randomization
- MapClass: Cell_Distance, Cell_Threat, Cell_Region

## Related original files
- **TEAM.H** - Class declarations, TeamTypeClass, TeamMissionStruct
- **TEAMTYPE.H** - TeamTypeClass, TeamMissionStruct
- **FOOT.CPP/.H** - FootClass (member base)
- **UNIT.CPP/.H** - UnitClass
- **INFANTRY.CPP/.H** - InfantryClass
- **AIRCRAFT.CPP/.H** - AircraftClass
- **HOUSE.CPP/.H** - HouseClass (owner)
- **MAP.CPP/.H** - MapClass (distances, threats)
- **TRIGGER.CPP** - Trigger actions (CREATE_TEAM, etc.)
- **FUNCTION.H** - Core includes

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/team.rs` - TeamClass core
- `cnc-tiberian-dawn/src/ai/td_team.rs` - TD-specific teams
- `cnc-core/src/ai/coordination.rs` - Coordination logic
- `cnc-core/src/ai/recruitment.rs` - Recruitment logic
- `cnc-tiberian-dawn/src/ai/td_missions.rs` - Mission types

**Not yet ported:**
- Full team AI state machine
- Recruitment scanning (Infantry/Units arrays)
- Mission list execution (TMISSION_*)
- Coordination routines (Attack, Move, Regroup, Unload)
- Strength calculation (reinforcable vs non-reinforcable)
- Center calculation and lagging unit handling
- Team suspension/priority system

## Edge cases and compatibility notes

1. **Human teams**: Always considered full strength, used only for reinforcements
2. **Reinforcable teams**: Revert from under-strength at 1/3 strength threshold
3. **Non-reinforcable**: Never under-strength once `IsHasBeen` (started mission)
4. **Auto-delete**: Human/reinforcement teams delete when empty and `IsHasBeen`
5. **Captain selection**: First initiated member becomes captain; if none, first member becomes captain
5. **Captain**: First initiated member; if none, first member
6. **Mission timeout**: `TimeOut = Argument * (TICKS_PER_MINUTE/10)`
6. **Mission timeout**: `Argument * (TICKS_PER_MINUTE/10)`
7. **Stray distance**: `STRAY_DISTANCE` threshold for lagging/regrouping
7. **Stray distance**: `STRAY_DISTANCE` threshold
8. **Target switching**: On damage, may retarget to attacker if current target can't fire back
8. **Target switching**: On damage, may retarget to attacker
9. **Regroup preference**: Prefers repair bays, then other friendly structures
9. **Regroup**: Prefers repair bays
10. **Suspension**: `Suspend_Teams(priority)` suspends teams below priority
10. **Suspension**: 2-minute timer, then reactivates
11. **Lagging units**: Ordered to catch up to `ObjectiveCenter`
11. **Lagging**: Ordered to catch up to `ObjectiveCenter`
12. **Loaner transports**: Break off team during unload (`IsALoaner`)
12. **Loaner**: Breaks off during unload
13. **Team deletion**: Human teams delete when empty and `IsHasBeen`
13. **Deletion**: Human teams delete when empty and `IsHasBeen`
14. **Recruitment**: Scans Infantry/Units arrays for matching house/type
14. **Recruitment**: Scans Infantry/Units for matching house/type
15. **Team types**: Defined in TeamTypeClass with mission lists
15. **Team types**: Defined in TeamTypeClass

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TEAM.CPP at f1f0d42
