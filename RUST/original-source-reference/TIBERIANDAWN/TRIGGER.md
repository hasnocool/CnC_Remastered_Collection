# PATH/FILE
original/TIBERIANDAWN/TRIGGER.CPP

## Purpose
Mission trigger system for Tiberian Dawn. Implements the event-action trigger system that drives scenario scripting: events (player enters, discovered, destroyed, etc.) trigger actions (win, lose, create team, reinforcements, airstrike, etc.).

## Where it fits in the game
Scenario scripting system - the backbone of single-player campaigns and skirmish missions. Triggers link game events to scripted responses, enabling campaign progression, cinematic events, AI behavior changes, and victory/defeat conditions.

## Classes, structs, enums and important data

### TriggerClass
**Core trigger object** - links an event to an action with optional data, house, team, and persistence.

**Key data members:**
- `Event` (EventType) - Triggering event type
- `Action` (ActionType) - Action to execute when triggered
- `Data` (long) - Event-specific data value
- `DataCopy` (long) - Original data value (for resetting)
- `House` (HousesType) - Owning house (HOUSE_NONE = global)
- `Team` (TeamTypeClass*) - Associated team type
- `Name` (char[]) - Trigger name (for INI lookup)
- `IsPersistant` (PersistantType) - VOLATILE (remove after fire) or PERSISTENT
- `AttachCount` (int) - Attachment reference count
- `Team` (TeamTypeClass*) - Associated team
- `Event` (EventType) - Trigger event type
- `Action` (ActionType) - Action to execute
- `Data` (long) - Event data value
- `DataCopy` (long) - Original data value
- `House` (HousesType) - Owning house
- `Team` (TeamTypeClass*) - Team pointer
- `Name` (char[]) - Trigger name
- `IsPersistant` (PersistantType) - VOLATILE/PERSISTENT
- `AttachCount` (int) - Attachment count

### EventType enum (17 types)
| Event | Description | Needs Object | Needs House | Needs Data |
|-------|-------------|--------------|-------------|------------|
| EVENT_NONE | None | No | No | No |
| EVENT_PLAYER_ENTERED | Player enters cell | Yes | Yes* | No |
| EVENT_DISCOVERED | Object discovered | Yes | No | No |
| EVENT_ATTACKED | Object attacked | Yes | No | No |
| EVENT_DESTROYED | Object destroyed | Yes | No | No |
| EVENT_ANY | Any event on object | Yes | No | No |
| EVENT_HOUSE_DISCOVERED | House discovered | No | Yes | No |
| EVENT_UNITS_DESTROYED | N units destroyed | No | Yes | No |
| EVENT_BUILDINGS_DESTROYED | N buildings destroyed | No | Yes | No |
| EVENT_ALL_DESTROYED | All destroyed | No | Yes | No |
| EVENT_CREDITS | Credits threshold | No | Yes | Yes |
| EVENT_TIME | Time elapsed | No | Yes | Yes |
| EVENT_NBUILDINGS_DESTROYED | N buildings destroyed | No | Yes | Yes |
| EVENT_NUNITS_DESTROYED | N units destroyed | No | Yes | Yes |
| EVENT_NOFACTORIES | No factories | No | Yes | No |
| EVENT_EVAC_CIVILIAN | Civilian evacuated | No | Yes | No |
| EVENT_BUILD | Building constructed | No | Yes | Yes |

*Player entered uses player's house if none specified

### ActionType enum (22 types)
| Action | Description | Needs Team | Description |
|--------|-------------|------------|-------------|
| ACTION_NONE | None | No | - |
| ACTION_WIN | Player wins | No | Flag_To_Win() |
| ACTION_LOSE | Player loses | No | Flag_To_Lose() |
| ACTION_BEGIN_PRODUCTION | Start AI production | No | Begin_Production() |
| ACTION_CREATE_TEAM | Create team | Yes | Team->Create_One_Of() |
| ACTION_DESTROY_TEAM | Destroy team | Yes | Team->Destroy_All_Of() |
| ACTION_REINFORCEMENTS | Reinforcements | Yes | Do_Reinforcements() |
| ACTION_ALL_HUNT | All units hunt | No | Do_All_To_Hunt() |
| ACTION_REINFORCEMENTS | Reinforcements | Yes | Do_Reinforcements() |
| ACTION_AIRSTRIKE | Grant airstrike | No | AirStrike.Enable() |
| ACTION_NUCLEAR_MISSILE | Grant nuke | No | NukeStrike.Enable() |
| ACTION_ION_CANNON | Grant ion cannon | No | IonCannon.Enable() |
| ACTION_DESTROY_XXXX | Destroy trigger XXXX | No | Remove trigger XXXX |
| ACTION_DESTROY_YYYY | Destroy trigger YYYY | No | Remove trigger YYYY |
| ACTION_DESTROY_ZZZZ | Destroy trigger ZZZZ | No | Remove trigger ZZZZ |
| ACTION_AUTOCREATE | Auto-create teams | No | House->IsAlerted = true |
| ACTION_DZ | Drop zone smoke | No | ANIM_LZ_SMOKE at waypoint 25 |
| ACTION_WIN | Win mission | No | Flag_To_Win() |
| ACTION_LOSE | Lose mission | No | Flag_To_Lose() |
| ACTION_BEGIN_PRODUCTION | Start AI production | No | Begin_Production() |
| ACTION_CREATE_TEAM | Create team | Yes | Team->Create_One_Of() |
| ACTION_DESTROY_TEAM | Destroy team | Yes | Team->Destroy_All_Of() |
| ACTION_REINFORCEMENTS | Reinforcements | Yes | Do_Reinforcements() |
| ACTION_ALLOWWIN | Allow win | No | Blockage--, BorrowedTime=4s |

### PersistantType enum
- `VOLATILE` - Trigger removes itself after firing
- `PERSISTENT` - Trigger remains after firing

### Global arrays
- `EventText[EVENT_COUNT+1]` - Event name strings (17 + 1)
- `ActionText[ACTION_COUNT+1]` - Action name strings (22 + 1)

## Functions and methods

### TriggerClass

**TriggerClass::TriggerClass()** - Constructor
- `IsPersistant = VOLATILE`, `AttachCount = 0`, `Event = EVENT_NONE`, `Action = ACTION_NONE`, `House = HOUSE_NONE`, `Data = DataCopy = 0`, `Name = ""`, `Team = NULL`

**TriggerClass::~TriggerClass()** - Destructor
- If `Action == ACTION_ALLOWWIN` and `House != HOUSE_NONE`: decrements House->Blockage, sets BorrowedTime = 4 seconds

**TriggerClass::Init()** - System initialization
- `Triggers.Free_All()` - Clears all triggers

**TriggerClass::Spring(event, object)** → int - **Cell trigger**
- Checks if trigger fires for event on cell
- Returns 1 if sprung, 0 otherwise
- Matches: event, object type, house, data value
- Calls `Fire(object)` if matches

**TriggerClass::Spring(event, house, data)** → int - **House trigger**
- Checks if trigger fires for house event with data
- Matches: event, house, data value (if Event_Need_Data)
- Calls `Fire(HouseClass::As_Pointer(house))`

**TriggerClass::Spring(event, house, object)** → int - **Object trigger**
- Checks if trigger fires for event on object
- Matches: event, object type, house, data value
- Calls `Fire(object)`

**TriggerClass::Fire(object)** → int - **Core trigger execution**
- Validates object exists
- Executes action via switch on `Action`:
  - `ACTION_NONE`: success=false
  - `ACTION_DESTROY_XXXX/YYYY/ZZZZ`: Finds trigger by name, calls Remove()
  - `ACTION_AIRSTRIKE`: Enables AirStrike, forces charge for player
  - `ACTION_NUCLEAR_MISSILE`: Enables NukeStrike, adds to sidebar
  - `ACTION_ION_CANNON`: Enables IonCannon, adds to sidebar
  - `ACTION_DZ`: Spawns ANIM_LZ_SMOKE at Waypoint[25]
  - `ACTION_WIN`: PlayerPtr->Flag_To_Win()
  - `ACTION_LOSE`: PlayerPtr->Flag_To_Lose()
  - `ACTION_BEGIN_PRODUCTION`: House->Begin_Production()
  - `ACTION_CREATE_TEAM`: Team->Create_One_Of() (if Team exists)
  - `ACTION_DESTROY_TEAM`: Team->Destroy_All_Of()
  - `ACTION_REINFORCEMENTS`: Do_Reinforcements(Team)
  - `ACTION_ALL_HUNT`: Do_All_To_Hunt()
  - `ACTION_AUTOCREATE`: House->IsAlerted = true
  - `ACTION_DESTROY_XXXX/YYYY/ZZZZ`: Finds trigger by name, Remove()
  - `ACTION_AIRSTRIKE`: Enables AirStrike, forces charge for player, adds sidebar
  - `ACTION_NUCLEAR_MISSILE`: Enables NukeStrike, adds sidebar
  - `ACTION_ION_CANNON`: Enables IonCannon, adds sidebar
  - `ACTION_DZ`: ANIM_LZ_SMOKE at Waypoint[25]
  - `ACTION_WIN`: Flag_To_Win()
  - `ACTION_LOSE`: Flag_To_Lose()
  - `ACTION_BEGIN_PRODUCTION`: Begin_Production()
  - `ACTION_CREATE_TEAM`: Team->Create_One_Of()
  - `ACTION_DESTROY_TEAM`: Team->Destroy_All_Of()
  - `ACTION_REINFORCEMENTS`: Do_Reinforcements()
  - `ACTION_ALL_HUNT`: Do_All_To_Hunt()
  - `ACTION_AUTOCREATE`: House->IsAlerted = true
  - `ACTION_DESTROY_XXXX/YYYY/ZZZZ`: Remove named triggers
  - `ACTION_AIRSTRIKE`: AirStrike.Enable(), Forced_Charge, sidebar
  - `ACTION_NUCLEAR_MISSILE`: NukeStrike.Enable(), sidebar
  - `ACTION_ION_CANNON`: IonCannon.Enable(), sidebar
  - `ACTION_DZ`: ANIM_LZ_SMOKE at Waypoint[25]
  - `ACTION_WIN`: Flag_To_Win()
  - `ACTION_LOSE`: Flag_To_Lose()
  - `ACTION_BEGIN_PRODUCTION`: Begin_Production()
  - `ACTION_CREATE_TEAM`: Team->Create_One_Of()
  - `ACTION_DESTROY_TEAM`: Team->Destroy_All_Of()
  - `ACTION_REINFORCEMENTS`: Do_Reinforcements()
  - `ACTION_ALL_HUNT`: Do_All_To_Hunt()
  - `ACTION_AUTOCREATE`: IsAlerted = true
  - `ACTION_DESTROY_XXXX/YYYY/ZZZZ`: Remove by name
  - `ACTION_AIRSTRIKE`: AirStrike.Enable(false,true), Forced_Charge(true), Map.Add(SPC_AIR_STRIKE)
  - `ACTION_NUCLEAR_MISSILE`: NukeStrike.Enable(true), sidebar
  - `ACTION_ION_CANNON`: IonCannon.Enable(true), sidebar
  - `ACTION_DZ`: ANIM_LZ_SMOKE at Waypoint[25]
  - `ACTION_WIN`: Flag_To_Win()
  - `ACTION_LOSE`: Flag_To_Lose()
  - `ACTION_BEGIN_PRODUCTION`: Begin_Production()
  - `ACTION_CREATE_TEAM`: Team->Create_One_Of()
  - `ACTION_DESTROY_TEAM`: Team->Destroy_All_Of()
  - `ACTION_REINFORCEMENTS`: Do_Reinforcements()
  - `ACTION_ALL_HUNT`: Do_All_To_Hunt()
  - `ACTION_AUTOCREATE`: House->IsAlerted = true
  - `ACTION_ALLOWWIN`: Blockage--, BorrowedTime=4s
  - `ACTION_DZ`: ANIM_LZ_SMOKE at Waypoint[25]

- Sets `success=true` if action executed
- If `!success && Event == EVENT_TIME`: `Data = 1` (reschedule)
- If `success && IsPersistant == VOLATILE`: `Remove()`

**TriggerClass::Remove()** → bool
- Clears cell triggers: loops all 4096 cells, clears CellTriggers/IsTrigger
- Clears object pointers: Infantry, Buildings, Units, Terrains
- Removes from all HouseTriggers lists
- `delete this`

**TriggerClass::Read_INI(buffer)** - Loads triggers from scenario INI
- Format: `Triggername = Eventname, Actionname, Data, Housename, TeamName, IsPersistant`
- Must be called before other Read_INI (teams must exist)
- Creates trigger, parses entry via Fill_In()
- Adds to HouseTriggers if House != NONE
- For ACTION_ALLOWWIN: increments House->Blockage

**TriggerClass::Fill_In(name, entry)** - Parses INI entry
- Tokens: Event, Action, Data, House, Team, IsPersistant (optional)
- Uses strtok on comma-separated values
- Resolves Event/Action from name via Event_From_Name/Action_From_Name
- Resolves House from name, Team from TeamTypeClass::As_Pointer
- IsPersistant defaults to VOLATILE if omitted

**TriggerClass::Write_INI(buffer, refresh)** - Writes triggers to INI
- Format: `Triggername = Eventname, Actionname, Data, Housename, TeamName, IsPersistant`
- If refresh: clears existing INI section
- Writes all active triggers

**TriggerClass::As_Pointer(name)** → TriggerClass*
- Linear search by name (case-insensitive)

**TriggerClass::Event_From_Name(name)** → EventType
- Linear search in EventText array

**TriggerClass::Name_From_Event(event)** → const char*
- Returns EventText[event+1]

**TriggerClass::Action_From_Name(name)** → ActionType
- Linear search in ActionText array

**TriggerClass::Name_From_Action(action)** → const char*
- Returns ActionText[action+1]

**TriggerClass::As_Target()** → TARGET
- `Build_Target(KIND_TRIGGER, Triggers.ID(this))`

**TriggerClass::operator new()** - Pool allocation
- `Triggers.Allocate()`, sets IsActive=true

**TriggerClass::operator delete(ptr)** - Pool deallocation
- Sets IsActive=false, `Triggers.Free()`

**TriggerClass::Event_From_Name(name)** → EventType
- Linear search in EventText (1-based)

**TriggerClass::Name_From_Event(event)** → const char*
- Returns EventText[event+1]

**TriggerClass::Action_From_Name(name)** → ActionType
- Linear search in ActionText

**TriggerClass::Name_From_Action(action)** → const char*
- Returns ActionText[action+1]

**TriggerClass::As_Target()** → TARGET
- `Build_Target(KIND_TRIGGER, Triggers.ID(this))`

**TriggerClass::operator new()** - Pool allocation
- `Triggers.Allocate()`, sets IsActive=true

**TriggerClass::operator delete(ptr)** - Pool deallocation
- Sets IsActive=false, `Triggers.Free()`

**TriggerClass::operator new()** - Pool alloc
- `Triggers.Allocate()`, IsActive=true

**TriggerClass::operator delete(ptr)** - Pool dealloc
- IsActive=false, `Triggers.Free()`

### Global helper

**Do_All_To_Hunt()** - Forces all AI units to hunt
- Iterates Units + Infantry arrays
- Non-human, IsDown, not InLimbo
- Removes from team, assigns MISSION_HUNT

## Inputs, outputs and side effects

**State mutations:**
- Trigger firing: executes action, may remove self (VOLATILE)
- ACTION_ALLOWWIN: decrements House->Blockage, sets BorrowedTime=4s
- ACTION_CREATE_TEAM: creates team unit
- ACTION_DESTROY_TEAM: destroys all team units
- ACTION_REINFORCEMENTS: spawns reinforcements
- ACTION_ALL_HUNT: forces all AI units to hunt
- ACTION_AIRSTRIKE/NUKE/ION: enables superweapon, adds sidebar
- ACTION_DESTROY_XXXX/YYYY/ZZZZ: removes named triggers
- ACTION_WIN/LOSE: ends mission
- ACTION_ALLOWWIN: decrements Blockage, sets BorrowedTime

**External systems affected:**
- Team system: Create_One_Of, Destroy_All_Of, reinforcements
- Superweapons: AirStrike, NukeStrike, IonCannon enable
- Map: Sidebar updates, ANIM_LZ_SMOKE
- House: Blockage, BorrowedTime, IsAlerted, Begin_Production
- Team: Create_One_Of, Destroy_All_Of, reinforcements
- Cell triggers: IsTrigger, CellTriggers array
- Object triggers: Infantry/Building/Unit/Terrain Trigger pointers

## Calls into / called by

**Called by:**
- Cell events: Map cell Player_Entered → Spring(event, object)
- Object events: Infantry/Building/Unit/Terrain destruction/attack → Spring(event, object)
- House events: Credits, time, building/unit counts → Spring(event, house, data)
- Time: EVENT_TIME checked periodically
- INI loading: Read_INI() at scenario load

**Calls into:**
- TeamClass: Create_One_Of(), Destroy_All_Of()
- HouseClass: Begin_Production(), Flag_To_Win/Lose, Blockage, BorrowedTime, IsAlerted, AirStrike/NukeStrike/IonCannon.Enable()
- TeamTypeClass: Create_One_Of(), Destroy_All_Of(), Do_Reinforcements()
- MapClass: Add(RTTI_SPECIAL, SPC_*), Column[1].Flag_To_Redraw()
- AnimClass: ANIM_LZ_SMOKE
- TriggerClass: As_Pointer(), Remove()
- HouseClass: Blockage, BorrowedTime, Begin_Production(), AirStrike/NukeStrike/IonCannon.Enable()
- TeamTypeClass: Create_One_Of(), Destroy_All_Of(), Do_Reinforcements()
- MapClass: Add(), Column[].Flag_To_Redraw()
- AnimClass: ANIM_LZ_SMOKE
- PlayerPtr: Flag_To_Win/Lose()
- Do_All_To_Hunt(): Forces AI units to hunt

## Related original files
- **TRIGGER.H** - Class declarations, EventType/ActionType enums
- **TEAM.CPP/.H** - TeamClass, TeamTypeClass
- **HOUSE.CPP/.H** - HouseClass (Blockage, BorrowedTime, superweapons)
- **MAP.CPP/.H** - MapClass (Waypoint[25], sidebar, columns)
- **INFANTRY.CPP/.H** - InfantryClass (object triggers)
- **BUILDING.CPP/.H** - BuildingClass (object triggers)
- **UNIT.CPP/.H** - UnitClass (object triggers)
- **TERRAIN.CPP/.H** - TerrainClass (object triggers)
- **MAP.CPP/.H** - MapClass (CellTriggers, Waypoint[25])
- **SIDEBAR.CPP** - Sidebar updates
- **SIDEBARGLYPHX.H** - MP sidebar
- **INI.H** - INI parsing (WWGetPrivateProfileString, WWWritePrivateProfileString)
- **FUNCTION.H** - Core includes
- **TEAM.H** - TeamClass, TeamTypeClass

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/scenario/trigger.rs` - TriggerClass core
- `cnc-tiberian-dawn/src/scenario/td_trigger.rs` - TD-specific triggers
- `cnc-core/src/scenario/event.rs` - EventType, ActionType
- `cnc-core/src/scenario/action.rs` - Action execution
- `cnc-tiberian-dawn/src/scenario/campaign.rs` - Campaign scripting
- `cnc-core/src/scenario/ini.rs` - INI parsing/serialization
- `cnc-core/src/scenario/condition.rs` - Event conditions
- `cnc-core/src/scenario/action_dispatch.rs` - Action dispatch

**Not yet ported:**
- Full trigger system with cell/object/house events
- 17 EventType + 22 ActionType dispatch
- INI serialization (trigger format)
- VOLATILE/PERSISTENT lifecycle
- Cell trigger attachment (CellTriggers array)
- Object trigger attachment (Infantry/Building/Unit/Terrain)
- House trigger lists (HouseTriggers)
- Special actions: WIN/LOSE, superweapons, team management
- ACTION_DESTROY_XXXX/YYYY/ZZZZ named trigger removal
- ACTION_ALLOWWIN blockage/borrowed time
- ACTION_DZ drop zone smoke
- INI format parsing with team/house resolution

## Edge cases and compatibility notes

1. **Trigger limits**: TRIGGER_MAX limit (pool allocation)

2. **Persistence**: VOLATILE triggers auto-remove after firing; PERSISTENT remain

3. **Attachment count**: AttachCount tracks references (cell + object + house)

4. **Named trigger destruction**: ACTION_DESTROY_XXXX/YYYY/ZZZZ finds by name "XXXX"/"YYYY"/"ZZZZ"

5. **ACTION_ALLOWWIN**: Decrements House->Blockage, sets BorrowedTime=4 seconds

6. **Cell triggers**: Stored in CellTriggers[4096] array, IsTrigger flag per cell

7. **Object triggers**: Infantry/Building/Unit/Terrain have Trigger* pointers

7. **House triggers**: HouseTriggers[8] lists per house

8. **INI format**: `Name = Event,Action,Data,House,Team,Persistant` (Persistant optional, defaults VOLATILE)

9. **Event requirements**:
   - Needs Object: PLAYER_ENTERED, DISCOVERED, ATTACKED, DESTROYED, ANY
   - Needs House: HOUSE_DISCOVERED, UNITS_DESTROYED, BUILDINGS_DESTROYED, ALL_DESTROYED, CREDITS, TIME, NBUILDINGS_DESTROYED, NUNITS_DESTROYED, NOFACTORIES, EVAC_CIVILIAN, BUILD
   - Needs Data: CREDITS, TIME, NBUILDINGS_DESTROYED, NUNITS_DESTROYED, BUILD

9. **Action requirements**:
   - Needs Team: CREATE_TEAM, DESTROY_TEAM, REINFORCEMENTS

10. **ACTION_ALLOWWIN**: Decrements House->Blockage, sets BorrowedTime = 4 seconds

11. **Trigger persistence**: VOLATILE removes after fire; PERSISTENT stays

11. **ACTION_DESTROY_XXXX/YYYY/ZZZZ**: Finds trigger by exact name "XXXX"/"YYYY"/"ZZZZ"

12. **House trigger attachment**: If House != NONE, adds to HouseTriggers[house], increments AttachCount

12. **Read_INI order**: Must be called before other classes' Read_INI (teams must exist)

13. **ACTION_DESTROY_XXXX/YYYY/ZZZZ**: Exact name match "XXXX"/"YYYY"/"ZZZZ"

14. **Cell triggers**: Stored in CellTriggers[4096], IsTrigger flag

15. **Object triggers**: Infantry/Building/Unit/Terrain have Trigger* pointers

16. **House triggers**: HouseTriggers[8] lists per house

17. **INI format**: `Name = Event,Action,Data,House,Team,Persistant` (Persistant optional, default VOLATILE)

18. **Event requirements**: See Event_Need_Object/House/Data functions

19. **Action requirements**: See Action_Need_Team function

20. **ACTION_ALLOWWIN**: House->Blockage--, BorrowedTime = 4 seconds

21. **Trigger persistence**: VOLATILE = remove after fire; PERSISTENT = stay

22. **Cell triggers**: 4096-element CellTriggers array + IsTrigger flags

23. **Object triggers**: Infantry/Building/Unit/Terrain have Trigger* pointers

24. **House triggers**: HouseTriggers[8] per-house lists

25. **INI format**: `Name = Event,Action,Data,House,Team,Persistant` (Persistant optional, default VOLATILE)

26. **Event requirements**: See Event_Need_Object/House/Data

27. **Action requirements**: See Action_Need_Team

28. **ACTION_ALLOWWIN**: Blockage--, BorrowedTime = 4s

29. **Persistence**: VOLATILE removes after fire; PERSISTENT stays

30. **Cell triggers**: 4096-element array + IsTrigger flags

31. **Object triggers**: Infantry/Building/Unit/Terrain Trigger* pointers

32. **House triggers**: 8 HouseTriggers lists

33. **INI format**: `Name = Event,Action,Data,House,Team,Persistant` (Persistant optional, default VOLATILE)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TRIGGER.CPP at f1f0d42
