# PATH/FILE
original/TIBERIANDAWN/EVENT.CPP

## Purpose
The event system in TiberianDawn handles all external game events that can be performed by the player, enabling multiplayer synchronization by transmitting events between linked computers. The EventClass encapsulates event type, timing, and all required data parameters into a fixed-size structure that can be reliably sent across the network. Every event affects all connected computers on the same game frame, ensuring deterministic multiplayer synchronization. The event system covers player actions (alliances, building placement, unit commands), game state changes (timing, process time), and special effects (animations, exit sequences).

## Where it fits in the game
Core system file: `EVENT.CPP` implements the `EventClass` which is used throughout the game for player-initiated and system events. Events are created from player input, game logic, and multiplayer packet transmission. The EventClass struct is defined in `EVENT.H` and its `Execute()` method is called to process queued events. Event types span player diplomacy (ALLY), unit control (MEGAMISSION, IDLE, SCATTER), building operations (PLACE, PRODUCE, SUSPEND, ABANDON), sales (SELL), special actions (SPECIAL, SPECIAL_PLACE, EXIT), animations (ANIMATION), repair (REPAIR), and system timing (GAMESPEED, RESPONSE_TIME, TIMING, PROCESS_TIME). The EventLength table maps each event type to its data size, ensuring correct packet serialization.

## Classes, structs, enums and important data

### EventClass
**Purpose:** Holds all external game events that can be transported between linked computers for multiplayer synchronization.

**Key data members (from EVENT.H and EVENT.CPP):**
- `Type` (EventType) - Type of event being dispatched
- `Frame` (unsigned : 27) - Frame number for execution (27 bits gives >25 days at 30 FPS)
- `ID` (unsigned : 4) - House index of the player originating the event
- `IsExecuted` (unsigned : 1) - Flag indicating event already executed
- `MPlayerID` (unsigned char) - Multiplayer ID: high nybble = color index, low nybble = HousesType (GDI/NOD)
- `Data` (union) - Contains type-specific data, one of many sub-structures:

**EventLength table** maps event enum values to data sizes:
- EMPTY: 0 bytes
- ALLY: size_of(EventClass, Data.General) 
- MEGAMISSION: size_of(EventClass, Data.MegaMission)
- IDLE: size_of(EventClass, Data.Target)
- SCATTER: size_of(EventClass, Data.Target)
- DESTRUCT: 0 bytes
- DEPLOY: 0 bytes
- PLACE: size_of(EventClass, Data.Place)
- OPTIONS: 0 bytes
- GAMESPEED: size_of(EventClass, Data.General)
- PRODUCE: size_of(EventClass, Data.Specific)
- SUSPEND: size_of(EventClass, Data.Specific.Type)
- ABANDON: size_of(EventClass, Data.Specific.Type)
- PRIMARY: size_of(EventClass, Data.Target)
- SPECIAL_PLACE: size_of(EventClass, Data.Special)
- EXIT: 0 bytes
- ANIMATION: size_of(EventClass, Data.Anim)
- REPAIR: size_of(EventClass, Data.Target)
- SELL: size_of(EventClass, Data.Target)
- SPECIAL: size_of(EventClass, Data.Options)
- FRAMESYNC: 0 bytes
- MESSAGE: 0 bytes
- RESPONSE_TIME: size_of(EventClass, Data.FrameInfo.Delay)
- FRAMEINFO: size_of(EventClass, Data.FrameInfo)
- ARCHIVE: size_of(EventClass, Data.NavCom)
- TIMING: size_of(EventClass, Data.Timing)
- PROCESS_TIME: size_of(EventClass, Data.ProcessTime)

**EventNames array** lists all 32 event type names:
EMPTY, ALLY, MEGAMISSION, IDLE, SCATTER, DESTRUCT, DEPLOY, PLACE, OPTIONS, GAMESPEED, PRODUCE, SUSPEND, ABANDON, PRIMARY, SPECIAL_PLACE, EXIT, ANIMATION, REPAIR, SELL, SPECIAL, FRAMESYNC, MESSAGE, RESPONSE_TIME, FRAMEINFO, ARCHIVE, TIMING, PROCESS_TIME, LAST_EVENT

**EventType enum** (32 values, underlying unsigned char):
- EMPTY, ALLY, MEGAMISSION, IDLE, SCATTER, DESTRUCT, DEPLOY, PLACE, OPTIONS, GAMESPEED, PRODUCE, SUSPEND, ABANDON, PRIMARY, SPECIAL_PLACE, EXIT, ANIMATION, REPAIR, SELL, SPECIAL, FRAMESYNC, MESSAGE, RESPONSE_TIME, FRAMEINFO, ARCHIVE, TIMING, PROCESS_TIME, LAST_EVENT

**Nested data sub-structures in the Data union:**
- `Options` { SpecialClass Data; } - Special option flags
- `Target` { TARGET Whom; } - Target object for event
- `Anim` { AnimType What; HousesType Owner; COORDINATE Where; int Visible; } - Animation event data
- `General` { int Value; } - General-purpose integer data
- `MegaMission` { TARGET Whom; MissionType Mission; TARGET Target; TARGET Destination; } - Full mission assignment
- `Mission` { TARGET Whom; MissionType Mission; } - Mission assignment without target/destination
- `NavCom` { TARGET Whom; TARGET Where; } - Navigation computer assignment
- `TarCom` { TARGET Whom; TARGET Target; } - Target computer assignment
- `Specific` { RTTIType Type; int ID; } - Type and ID identification
- `Place` { RTTIType Type; CELL Cell; } - Place building at cell location
- `Special` { int ID; CELL Cell; } - Special ID and cell
- `FrameInfo` { unsigned long CRC; unsigned short CommandCount; unsigned char Delay; } - Frame synchronization
- `Timing` { unsigned short DesiredFrameRate; unsigned short MaxAhead; } - Timing values
- `ProcessTime` { unsigned short AverageTicks; } - Processing time per frame

### EventType enum values and their data requirements:
- EMPTY: No data, no operation
- ALLY: Make ally/enemy of specified house (Data.General.Value = house index)
- MEGAMISSION: Full mission assignment with target and destination (Data.MegaMission)
- IDLE: Enter idle mode (Data.Target.Whom = unit/techno)
- SCATTER: Scatter from current location (Data.Target.Whom = unit/techno)
- DESTRUCT: Self-destruct/surrender (no data)
- DEPLOY: MCV deploy at current location (no data)
- PLACE: Place building at cell (Data.Place.Type = object type, Data.Place.Cell = cell)
- OPTIONS: Bring up options screen (no data)
- GAMESPEED: Set game speed (Data.General.Value = speed value)
- PRODUCE: Start/resume production (Data.Specific.Type = object type, Data.Specific.ID = factory index)
- SUSPEND: Suspend production (Data.Specific.Type = object type)
- ABANDON: Abandon production (Data.Specific.Type = object type)
- PRIMARY: Toggle primary factory state (Data.Target.Whom = building)
- SPECIAL_PLACE: Place special weapon blast (Data.Special.ID = special weapon type, Data.Special.Cell = cell location)
- EXIT: Exit game (no data)
- ANIMATION: Create animation (Data.Anim.What = anim type, Data.Anim.Owner = owner, Data.Anim.Where = coord, Data.Anim.Visible = visibility)
- REPAIR: Start repair (Data.Target.Whom = techno to repair)
- SELL: Sell building/unit (Data.Target.Whom = techno to sell)
- SPECIAL: Special options control (Data.Options.Data = special flags)
- FRAMESYNC: Game connection packet (no data payload)
- MESSAGE: Message to another player (40 bytes message data follows event class)
- RESPONSE_TIME: Set propagation delay (Data.FrameInfo.Delay = frame delay)
- FRAMEINFO: Game heartbeat (CRC, CommandCount, Delay)
- ARCHIVE: Update archive target (Data.NavCom.Whom = techno, Data.NavCom.Where = archive target)
- TIMING: New timing values for all systems (Data.Timing.DesiredFrameRate, Data.Timing.MaxAhead)
- PROCESS_TIME: System processing time (Data.ProcessTime.AverageTicks = ticks per frame)

## Functions and methods

### Constructors

**EventClass::EventClass()** - Default constructor
- Sets Type = EMPTY
- Initializes Frame = ::Frame, ID = Houses.ID(PlayerPtr)

**EventClass::EventClass(SpecialClass data)** - Construct event to transfer special flags
- ID = Houses.ID(PlayerPtr), Type = SPECIAL
- Frame = ::Frame, Data.Options.Data = data

**EventClass::EventClass(EventType type, TARGET target)** - Construct simple target type event
- ID = Houses.ID(PlayerPtr), Type = type
- Frame = ::Frame, Data.Target.Whom = target

**EventClass::EventClass(EventType type)** - Default constructor for event objects
- ID = Houses.ID(PlayerPtr), Type = type
- Frame = ::Frame

**EventClass::EventClass(EventType type, int val)** - Constructor for general-purpose-data events
- ID = Houses.ID(PlayerPtr), Type = type
- Data.General.Value = val
- Frame = ::Frame

**EventClass::EventClass(EventType type, TARGET src, TARGET dest)** - Constructor for navigation computer events
- ID = Houses.ID(PlayerPtr), Type = type
- Frame = ::Frame, Data.NavCom.Whom = src, Data.NavCom.Where = dest

**EventClass::EventClass(AnimType anim, HousesType owner, COORDINATE coord, int visible)** - Constructor for animation events
- ID = Houses.ID(PlayerPtr), Type = ANIMATION
- Frame = ::Frame, Data.Anim.What = anim, Data.Anim.Owner = owner, Data.Anim.Where = coord, Data.Anim.Visible = visible

**EventClass::EventClass(TARGET src, MissionType mission, TARGET target=TARGET_NONE, TARGET destination=TARGET_NONE)** - Constructor for megamission events
- ID = Houses.ID(PlayerPtr), Type = MEGAMISSION
- Frame = ::Frame, Data.MegaMission.Whom = src, Data.MegaMission.Mission = mission, Data.MegaMission.Target = target, Data.MegaMission.Destination = destination

**EventClass::EventClass(EventType type, RTTIType object, int id)** - Constructor for object-type events
- ID = Houses.ID(PlayerPtr), Type = type
- Frame = ::Frame, Data.Specific.Type = object, Data.Specific.ID = id

**EventClass::EventClass(EventType type, RTTIType object, CELL cell)** - Constructor for place-building events
- ID = Houses.ID(PlayerPtr), Type = type
- Frame = ::Frame, Data.Place.Type = object, Data.Place.Cell = cell

**EventClass::EventClass(EventType type, int id, CELL cell)** - Constructor for special ID+cell events
- ID = Houses.ID(PlayerPtr), Type = type
- Frame = ::Frame, Data.Special.ID = id, Data.Special.Cell = cell

### Execute(void)
**Purpose:** Execute a queued event. The event must already be confirmed by any remote machine before calling this routine.

**Switch statement processes each event type:**

**ARCHIVE:** Updates archive target for a building.
- Gets techno from Data.NavCom.Whom
- If techno is active, sets techno->ArchiveTarget = Data.NavCom.Where

**ALLY:** Make or break alliance.
- Gets house from Data.General.Value
- If current house is ally of target, makes target enemy; otherwise makes target ally

**DESTRUCT:** Self-destruct/resignation.
- Calls Houses.Raw_Ptr(ID)->Flag_To_Die()
- Sets Houses.Raw_Ptr(ID)->Resigned = true

**SPECIAL:** Update special control flags.
- Sets global Special = Data.Options.Data
- Displays warning message with house name
- Flags map redraw

**REPAIR:** Start or stop repair on specified object.
- Gets techno from Data.Target.Whom
- If techno is active, calls techno->Repair(-1)

**SELL:** Tell building/unit to sell.
- Gets techno from Data.Target.Whom
- If techno is active and belongs to current house, calls techno->Sell_Back(-1)
- Otherwise, if target cell, calls Houses.Raw_Ptr(ID)->Sell_Wall(As_Cell(Data.Target.Whom))

**ANIMATION:** Trigger player-intervention animation.
- Creates new AnimClass with Data.Anim.What at Data.Anim.Where
- Sets owner and visibility flags
- If beacon, sets kill time 30 seconds from now

**PLACE:** Place object at location.
- Calls Houses.Raw_Ptr(ID)->Place_Object(Data.Place.Type, Data.Place.Cell)

**PRODUCE:** Start production of specified object.
- Calls Houses.Raw_Ptr(ID)->Begin_Production(Data.Specific.Type, Data.Specific.ID)

**SUSPEND:** Suspend production.
- Calls Houses.Raw_Ptr(ID)->Suspend_Production(Data.Specific.Type)

**ABANDON:** Abandon production.
- Calls Houses.Raw_Ptr(ID)->Abandon_Production(Data.Specific.Type)

**PRIMARY:** Toggle primary factory state of specified building.
- Gets building from Data.Target.Whom
- If building is active, calls building->Toggle_Primary()

**MEGAMISSION:** General purpose mission control event. Most player action routes through this event.
- Gets techno from Data.MegaMission.Whom
- If techno is active:
  - Resolves target object (or destination if target illegal)
  - Records who clicked on the object (house ID, ally check)
  - Calls techno->Assign_Mission(Data.MegaMission.Mission)
  - Handles guard area mode: sets archive target, clears target, sets destination
  - Handles enter mode for refineries: transmits hello message, clears destination
  - Otherwise assigns target and destination from mission data
  - Handles team contact removal for non-tethered techno

**IDLE:** Request unit enter idle mode.
- Gets techno from Data.Target.Whom
- If techno active, not in limbo, not tethered:
  - Assigns TARGET_NONE to destination and target
  - Calls techno->Enter_Idle_Mode()

**SCATTER:** Request unit scatter from current location.
- Gets techno from Data.Target.Whom
- If techno active, not in limbo, not tethered:
  - Calls techno->Scatter(0, true)

**SPECIAL_PLACE:** Place special weapon blast.
- Calls Houses.Raw_Ptr(ID)->Place_Special_Blast((SpecialWeaponType)Data.Special.ID, Data.Special.Cell)

**EXIT:** Exit the game.
- Queues theme song to THEME_NONE
- Stops speaking
- Speaks VOX_CONTROL_EXIT
- Waits while speaking
- Sets GameActive = false

**OPTIONS:** Process options menu.
- Sets SpecialDialog = SDLG_OPTIONS

**GAMESPEED:** Set game speed.
- Sets Options.GameSpeed = Data.General.Value

**RESPONSE_TIME:** Adjust connection timing for multiplayer games.
- Sets MPlayerMaxAhead = Data.FrameInfo.Delay
- Logs the change

**TIMING:** New timing values for all systems.
- Records vulnerability window frames if MaxAhead increasing
- Sets DesiredFrameRate = Data.Timing.DesiredFrameRate
- Sets MPlayerMaxAhead = Data.Timing.MaxAhead
- Handles WChat heartbeat poking

**PROCESS_TIME:** Process timing requirements for multiplayer games.
- Iterates over all multiplayer participants
- If current ID matches participant, stores TheirProcessTime[i] = Data.ProcessTime.AverageTicks
- Breaks on match

**default:** Do nothing.

## Inputs, outputs and side effects

**State mutations:**
- Alliance state (ALLY event changes house relationships)
- House resignation (DESTRUCT sets Resigned flag)
- Special options flags (SPECIAL event updates global Special)
- Repair state (REPAIR event modifies techno repair status)
- Sales status (SELL event triggers sell sequence)
- Animation creation (ANIMATION event creates AnimClass instances)
- Building placement (PLACE event positions new structures)
- Production control (PRODUCE/SUSPEND/ABANDON modify factory state)
- Primary factory toggle (PRIMARY event switches building mode)
- Mission assignment (MEGAMISSION assigns unit objectives)
- Idle mode (IDLE event puts units in idle)
- Scatter behavior (SCATTER event disperses units)
- Special weapon placement (SPECIAL_PLACE places blast effects)
- Game exit (EXIT event terminates game)
- Options menu (OPTIONS sets dialog state)
- Game speed (GAMESPEED modifies global speed)
- Multiplayer timing (RESPONSE_TIME/TIMING/PROCESS_TIME synchronize network state)
- Archive target (ARCHIVE updates techno archive target)

**External systems affected:**
- HouseClass: Alliance changes, resignation, special flags, sell operations
- TechnoClass: Repair, sell, mission assignment, idle/scatter behavior
- BuildingClass: Primary factory toggle, place object, sell wall
- AnimClass: Animation creation and lifecycle
- MapClass: Cell operations for placement
- Options system: Game speed, timing parameters
- Multiplayer subsystem: Timing synchronization, process time sharing
- Theme/speech system: Exit sequence audio

**Calls into:**
- HouseClass: Make_Enemy, Make_Ally, Flag_To_Die, Resigned, Name, RemapColor, Flag_To_Redraw, Place_Object, Begin_Production, Suspend_Production, Abandon_Production, Toggle_Primary, Place_Special_Blast, Sell_Back, Sell_Wall
- TechnoClass: IsActive, Repair, Sell_Back, Assign_Mission, Enter_Idle_Mode, Scatter
- BuildingClass: Toggle_Primary
- AnimClass: Constructor with owner, location, visibility, kill time
- Messages system: Add_Message with formatting flags
- Map redraw: Flag_To_Redraw
- MPlayer systems: MaxAhead, DesiredFrameRate, ProcessTime tracking

## Related original files
- **EVENT.H** - Class declarations, EventType enum, Data union, EventLength table, EventNames array
- **FUNCTION.H** - Core includes and type definitions
- **CCDDE.H** - DDE communication header
- **TECHNO.CPP/.H** - TechnoClass (referenced for IsActive, Repair, etc.)
- **HOUSE.CPP/.H** - HouseClass (alliance, resignation, sell, name)
- **BUILDING.CPP/.H** - BuildingClass (Toggle_Primary, Sell_Wall, Place_Special_Blast)
- **ANIM.CPP/.H** - AnimClass (animation creation)
- **MESSAGE.H** - Message display functions
- **MAP.CPP/.H** - Map cell operations
- **MULTIPLIER.CPP** - Multiplayer synchronization

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-events`

**Mapped modules:**
- `cnc-tiberian-dawn/src/event.rs` - EventClass (full event system port)
- `cnc-core/src/event_records.rs` - EventRecord data structures for INI parsing
- `cnc-tiberian-dawn/src/scenario_loader.rs` - Scenario INI event mapping
- `cnc-core/src/trigger_records.rs` - Trigger system event records (references TEVENT.CPP)
- `cnc-tiberian-dawn/src/multiplayer/synchronization.rs` - Multiplayer event transmission
- `cnc-tiberian-dawn/src/entity/unit.rs` - UnitClass AI overrides affected by events
- `cnc-core/src/entity/techno.rs` - TechnoClass repair/sell operations
- `cnc-tiberian-dawn/src/building/building.rs` - BuildingClass primary toggle, place object
- `cnc-core/src/ai/team_ai.rs` - Alliance/relationship modifications

**Not yet ported:**
- EventLength table maintenance synchronization with EventType enum
- EventNames string array for diagnostics
- MEGAMISSION Execute() full logic (complex mission assignment with all branches)
- ALLY event alliance toggle logic
- DESTRUCT resignation/multiplayer synchronization
- SPECIAL event special flags and message display
- REPAIR event techno repair integration
- SELL event sell sequence and refund calculation
- ANIMATION event creation and beacon kill timing
- PLACE event building placement
- PRODUCE/SUSPEND/ABANDON production control
- PRIMARY event primary factory toggle
- MEGAMISSION full mission assignment with all edge cases (guard area, enter refinery, team contact removal)
- IDLE event idle mode entry
- SCATTER event scatter behavior
- SPECIAL_PLACE special blast placement
- EXIT event game exit sequence
- OPTIONS event dialog system integration
- GAMESPEED game speed modification
- RESPONSE_TIME/TIMING/PROCESS_TIME multiplayer timing synchronization
- FRAMEINFO/FRAMESYNC packet structure handling
- MESSAGE event 40-byte message payload

## Edge cases and compatibility notes

1. **MEGAMISSION is the most complex event**, with multiple branching paths: guard area mode, refinery enter mode, normal target/destination assignment, and team contact removal for non-tethered techno. The guard area path sets archive target and clears target/destination, while refinery enter transmits a hello message and clears only destination.

2. **ALLY event toggles relationship** - checks current alliance status and switches between Make_Ally and Make_Enemy. Must remain synchronized across all multiplayer clients.

3. **DESTRUCT event sets resignation flag** which persists for the house. In multiplayer, this must be confirmed by all machines before execution.

4. **ANIMATION event beacon kill time** is set to exactly 30 seconds from creation time using GetSystemTimeAsFileTime. This is critical for multiplayer beacon synchronization.

5. **SPECIAL event displays a warning message** with the house name and flags map redraw. The message uses Text_String(TXT_SPECIAL_WARNING) formatting.

6. **PROCESS_TIME event** iterates over MPlayerCount participants to find matching MPlayerID, storing average ticks per frame. Handles the case where current ID doesn't match any participant (silently breaks).

7. **TIMING event** has special WChat handling: if from WChat, adds time-since-heartbeat/(70*60) to MPlayerMaxAhead, potentially drifting the value if heartbeats are missed.

8. **PLACE event** uses Data.Place.Type (RTTIType object type) and Data.Place.Cell (CELL location) to position buildings. The house determines the exact factory and real object pointer.

9. **SUSPEND/ABANDON events** use Data.Specific.Type (RTTIType object type) to identify which production to suspend/abandon, and Data.Specific.ID for the factory index.

10. **FRAMEINFO event** contains CRC (game CRC for sync error detection), CommandCount (# commands sent so far for missed packet detection), and Delay (propagation delay: Frame - Delay = sender's current frame #).

11. **EXIT event** sequence: queues theme song, stops speaking, speaks VOX_CONTROL_EXIT, waits for speech completion, then sets GameActive = false.

12. **All event constructors** set ID = Houses.ID(PlayerPtr) and Frame = ::Frame, ensuring proper player attribution and frame tracking.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/EVENT.CPP and original/TIBERIANDAWN/EVENT.H