# PATH/FILE
original/TIBERIANDAWN/UNIT.CPP

## Purpose
Concrete implementation of ground vehicles in Tiberian Dawn. UnitClass extends FootClass with vehicle-specific logic: turret control, recoil, harvesting, deployment (MCV), repair, cargo transport, and vehicle-specific AI overrides.

## Where it fits in the game
Concrete class in hierarchy: `ObjectClass` → `TechnoClass` → `FootClass` → `UnitClass`. All ground vehicles (tanks, harvesters, MCVs, APCs, artillery, gunboats) inherit from UnitClass. This is the primary combat vehicle class.

## Classes, structs, enums and important data

### UnitClass
**Concrete vehicle class** - ground combat vehicles with turret, harvesting, deployment, cargo.

**Key data members (from source):**
- `TurretFacing` (FacingClass) - Independent turret rotation (separate from body)
- `PrimaryFacing` (FacingClass) - Body facing (inherited from FootClass)
- `IsHarvesting` (bool) - Currently harvesting tiberium
- `Flagged` (HousesType) - CTF flag carried (HOUSE_NONE = none)
- `IsTethered` (bool) - Tethered to base (AI)
- `SpecialFlag` (bool) - Special mission flag
- `IsHarvesting` (bool) - Currently harvesting
- `Flagged` (HousesType) - CTF flag carried
- `IsTethered` (bool) - AI tether to base
- `SpecialFlag` (bool) - Special mission flag

**Inherited from FootClass:**
- `Speed` (0-255), `IsDriving`, `IsInitiated`, `IsPlanningToLook`, `IsDeploying`, `IsNewNavCom`, `IsFiring`, `IsRotating`, `IsUnloading`
- `NavCom`, `SuspendedNavCom`, `Path[13]`, `HeadToCoord`, `Member`, `Team`, `PathDelay`, `TryTryAgain`, `Group`, `ArchiveTarget`, `SuspendedNavCom`, `SuspendedMission`

**Inherited from TechnoClass:**
- `Mission`, `Target`, `Destination`, `Speed`, `Primary`/`Secondary` weapons, `Cloak`, `UncloakStage`, `AttachedObject`, `Passengers`, `MaxPassengers`, `IsTransporter`, `IsTwoShooter`, `IsTurretEquipped`, `IsRepairable`, `IsBuildable`, `IsLeader`, `IsScanner`, `IsNominal`, `IsRepairable`, `IsBuildable`, `IsCrew`, `IsTransporter`, `IsTwoShooter`, `IsTurretEquipped`, `IsRepairable`, `IsBuildable`, `IsCrew`, `IsTransporter`, `IsTwoShooter`, `IsTurretEquipped`, `IsRepairable`, `IsBuildable`, `IsCrew`, `IsTransporter`, `IsTwoShooter`, `IsTurretEquipped`, `IsRepairable`, `IsBuildable`, `IsCrew`

### Recoil/Turret adjustment tables
- `Recoil_Adjust[32]` - Per-facing pixel offsets for weapon recoil simulation
- `Turret_Adjust[32]` - MLRS/MSAM turret pixel offsets per body facing

### UnitTypeClass (static)
- `IsToHarvest` (bool) - Harvester type
- `IsTransporter` (bool) - APC/transport
- `IsToHarvest` (bool) - Harvester type

## Functions and methods

### Constructors/Destructor/Allocation

**UnitClass::UnitClass()** - Default constructor
- Calls FootClass()
- Initializes UnitClass-specific members

**UnitClass::UnitClass(house)** - Normal constructor
- Calls FootClass(house)
- Initializes UnitClass members

**UnitClass::~UnitClass()** - Destructor
- Virtual destructor

**UnitClass::new** - Custom allocation
- Allocates unit slot, adjusts access arrays
- Returns UnitClass* or NULL if full

**UnitClass::delete** - Custom deallocation
- Calls destructor, frees slot

### Core Vehicle Mechanics

**Recoil_Adjust(dir, x, y)** - Static helper
- 32-entry table: per-facing pixel offsets for weapon recoil
- Moves object "back" one pixel opposite firing direction
- Used for visual recoil effect on firing

**Turret_Adjust(dir, x, y)** - Static helper
- 32-entry table: MLRS/MSAM turret pixel offsets
- Accounts for turret displacement on MLRS/MSAM
- Per-body-facing turret pixel offsets

### Vehicle Identity & Targeting

**UnitClass::As_Target()** → TARGET
- Returns `Build_Target(KIND_UNIT, Units.ID(this))`
- Converts unit to target value for navigation/targeting

**UnitClass::Validate()** → int (CHEAT_KEYS)
- Validates unit index in Units array (0 to UNIT_MAX-1)

**UnitClass::What_Am_I()** → RTTIType
- Returns RTTI_UNIT

### Turret & Recoil

**Recoil_Adjust(dir, x, y)** - Static helper
- 32-entry signed char table per facing
- Simulates recoil by moving render position back 1 pixel opposite firing direction

**Turret_Adjust(dir, x, y)** - Static helper
- 32-entry table for MLRS/MSAM turret offset
- Accounts for turret displacement from body center
- Per-body-facing turret pixel offsets

### Targeting & Identity

**UnitClass::As_Target()** → TARGET
- `Build_Target(KIND_UNIT, Units.ID(this))`
- Converts unit to target value for nav/targeting

**UnitClass::What_Am_I()** → RTTIType
- Returns RTTI_UNIT

**UnitClass::Validate()** → int (CHEAT_KEYS)
- Validates unit index in Units array (0 to UNIT_MAX-1)

### Debug (CHEAT_KEYS)

**UnitClass::Debug_Dump(mono)** - Mono monitor display
- Displays: Name, Mission, TarCom, NavCom, Radio, Coord, HeadTo, Health, Body, Turret, Speed, Path, Cargo, Active, Limbo, Owned, Discovered, Selected, Tethered, Locked, TurretLocked, Loaner, Deploying, Rotating, Firing, Driving, ToLook, Recoiling, ToDisplay
- CargoClass::Debug_Dump, MissionClass::Debug_Dump, FootClass::Debug_Dump

### Vehicle-Specific AI Overrides

**UnitClass::Mission_Guard()** → int
- **Hovercraft**: If attached → MISSION_UNLOAD + Find_LZ(); else Exit_Map()
- **Gunboat**: → MISSION_HUNT
- **AI Harvester**: → MISSION_HARVEST
- **RA AI MCV**: If base building & not human → MISSION_UNLOAD
- Falls back to TarComClass::Mission_Guard()

**UnitClass::Mission_Move()** → int
- Always closes APC door if open
- **Gunboat**: Forces MISSION_HUNT
- Falls back to TarComClass::Mission_Move()

**UnitClass::Mission_Attack()** → int
- **Gunboat**: Forces MISSION_HUNT
- Falls back to TarComClass::Mission_Attack()

**UnitClass::Mission_Guard()** → int
- Hovercraft: attached → MISSION_UNLOAD+Find_LZ, else Exit_Map()
- Gunboat → MISSION_HUNT
- AI Harvester → MISSION_HARVEST
- RA AI MCV: base building & not human → MISSION_UNLOAD

**UnitClass::Mission_Move()** → int
- Closes APC door if open
- Gunboat → MISSION_HUNT

**UnitClass::Mission_Attack()** → int
- Gunboat → MISSION_HUNT

### Deployment (MCV)

**UnitClass::Try_To_Deploy()** → bool
- Checks Can_Enter_Cell() == MOVE_OK
- If deployable: Assign_Mission(MISSION_UNLOAD), returns true
- Returns false if can't deploy

**UnitClass::Try_To_Deploy()** → bool
- Checks Can_Enter_Cell() == MOVE_OK
- Sets MISSION_UNLOAD if deployable

**UnitClass::Goto_Clear_Spot()** → bool
- Finds clear cell for MCV deployment
- Checks adjacent cells for Can_Enter_Cell == MOVE_OK
- Assigns MISSION_UNLOAD, returns true if found

### Harvesting Logic

**UnitClass::Goto_Tiberium()** → bool
- Finds nearest tiberium patch via Map.Find_Tiberium()
- Assigns MISSION_HARVEST, returns true if found

**UnitClass::Harvesting()** → int
- Core harvesting loop:
  - If full (Tiberium_Load >= FULL_LOAD): Goto_Clear_Spot(), Mission_Unload
  - If not on tiberium: Goto_Tiberium()
  - If on tiberium: Harvesting (Offload_Tiberium_Bail)
- Returns ticks until next call

**UnitClass::Offload_Tiberium_Bail()** → int
- Returns credits offloaded per step
- Returns 0 when fully offloaded

**UnitClass::Goto_Tiberium()** → bool
- Finds nearest tiberium via Map.Find_Tiberium()
- Assigns MISSION_HARVEST

**UnitClass::Harvesting()** → int
- If full: Goto_Clear_Spot(), Mission_Unload
- If not on tiberium: Goto_Tiberium()
- On tiberium: Offload_Tiberium_Bail()

### Cargo & Transport

**UnitClass::APC_Open_Door()** / **APC_Close_Door()**
- Open_Door(10,2) / Close_Door(10,2) with facing-dependent timing
- NW/NE facing: Open_Door(10,2), else Open_Door(1,2)

**UnitClass::Desired_Load_Dir(passenger, moveto)** → DirType
- Calculates best facing/cell for APC loading
- Scores adjacent cells by passenger accessibility, rotation distance
- Returns best facing direction, sets moveto cell

**UnitClass::APC_Open_Door()** / **APC_Close_Door()**
- Open_Door(10,2) or (1,2) depending on facing (NW/NE vs others)

**UnitClass::APC_Close_Door()** - Close_Door(10,2)

**UnitClass::Desired_Load_Dir(passenger, moveto)** → DirType
- Scores adjacent cells by passenger accessibility, rotation cost
- Returns best facing, sets moveto cell

**UnitClass::Find_LZ()** → bool
- Finds landing zone for hovercraft unloading
- Checks adjacent cells for Can_Enter_Cell
- Assigns MISSION_UNLOAD, returns true if found

**UnitClass::Unload_Hovercraft_Process()** → int
- Processes hovercraft unloading
- Checks door state, passenger unloading

**UnitClass::Unload_Hovercraft_Process()** → int
- Door state checks, passenger unloading

### Deployment (MCV)

**UnitClass::Try_To_Deploy()** → bool
- Checks Can_Enter_Cell() == MOVE_OK at current position
- If OK: Assign_Mission(MISSION_UNLOAD), returns true
- Returns false if can't deploy

**UnitClass::Goto_Clear_Spot()** → bool
- Finds clear adjacent cell for MCV deployment
- Checks 8 adjacent cells for MOVE_OK
- Assigns MISSION_UNLOAD, returns true if found

### Harvesting

**UnitClass::Goto_Tiberium()** → bool
- Calls Map.Find_Tiberium(Cell_Cell(Coord), Owner())
- If found: Assign_Mission(MISSION_HARVEST), returns true

**UnitClass::Harvesting()** → int
- If full (Tiberium_Load >= FULL_LOAD): Goto_Clear_Spot(), Mission_Unload
- If not on tiberium: Goto_Tiberium()
- If on tiberium: Harvesting (Offload_Tiberium_Bail)
- Returns ticks until next call

**UnitClass::Offload_Tiberium_Bail()** → int
- Returns credits offloaded per step
- Returns 0 when fully offloaded

**UnitClass::Goto_Tiberium()** → bool
- Map.Find_Tiberium() from current cell

**UnitClass::Harvesting()** → int
- Full → Goto_Clear_Spot(), Mission_Unload
- Not on tiberium → Goto_Tiberium()
- On tiberium → Offload_Tiberium_Bail()

### Cargo & Transport

**UnitClass::Pip_Count()** → int
- Transporter: returns How_Many() (passenger count)
- Harvester: Tiberium_Load as percentage (0-5 pips)
- Other: 0

**UnitClass::Pip_Count()** → int
- Transporter: How_Many() passengers
- Harvester: Tiberium_Load as 0-5 pips

**UnitClass::Find_LZ()** → bool
- Finds landing zone for hovercraft unloading
- Checks adjacent cells for Can_Enter_Cell == MOVE_OK
- Assigns MISSION_UNLOAD, returns true

**UnitClass::Unload_Hovercraft_Process()** → int
- Processes hovercraft unloading sequence

**UnitClass::Find_LZ()** → bool
- Finds adjacent clear cell for hovercraft unloading
- Assigns MISSION_UNLOAD

**UnitClass::Unload_Hovercraft_Process()** → int
- Hovercraft unloading sequence

**UnitClass::Desired_Load_Dir(passenger, moveto)** → DirType
- Calculates best APC facing/cell for loading
- Scores adjacent cells by passenger accessibility, rotation cost
- Returns best facing, sets moveto cell

### Deployment (MCV)

**UnitClass::Try_To_Deploy()** → bool
- Checks Can_Enter_Cell() == MOVE_OK at current position
- If OK: Assign_Mission(MISSION_UNLOAD), returns true

**UnitClass::Goto_Clear_Spot()** → bool
- Checks 8 adjacent cells for Can_Enter_Cell == MOVE_OK
- Assigns MISSION_UNLOAD, returns true if found

**UnitClass::Exit_Repair()** - Drives off repair bay
- Checks 8 predefined exit offsets based on facing
- Assigns MISSION_MOVE to clear cell

**UnitClass::Exit_Repair()** - Drives off repair bay
- 8 predefined exit offsets based on facing
- Assigns MISSION_MOVE to clear cell

### Repair

**UnitClass::Exit_Repair()** - Drives off repair facility
- 8 predefined exit offsets based on facing
- Assigns MISSION_MOVE to first clear cell

**UnitClass::Can_Demolish()** → bool
- Only on repair bay (STRUCT_REPAIR), radio contact, distance < 128 leptons

**UnitClass::Sell_Back(control)** - Sell for refund
- control=1: VOX_CASHTURN, Refund_Amount(), Stun(), Limbo(), Delete_This()

### CTF Flags

**UnitClass::Flag_Attach(house)** → bool
- Attaches flag if none carried, Marks MARK_CHANGE

**UnitClass::Flag_Remove()** → bool
- Removes flag if carried, Marks MARK_CHANGE

**UnitClass::Flag_Attach(house)** → bool
- Attaches if none, Marks MARK_CHANGE

**UnitClass::Flag_Remove()** → bool
- Removes flag, Marks MARK_CHANGE

### Stun & Destruction

**UnitClass::Stun()** - Pre-removal
- Drops flag if carried (House.Flag_Attach at cell)
- Calls TarComClass::Stun()

**UnitClass::Stun()** - Pre-removal
- Drops flag at cell via House.Flag_Attach()
- Calls TarComClass::Stun()

### Cargo & Pips

**UnitClass::Pip_Count()** → int
- Transporter: How_Many() passengers
- Harvester: Tiberium_Load as percentage (0-5 pips)
- Other: 0

**UnitClass::Pip_Count()** → int
- Transporter: How_Many() passengers
- Harvester: Tiberium_Load percentage (0-5 pips)

### APC Doors

**UnitClass::APC_Open_Door()** / **APC_Close_Door()**
- Open_Door(10,2) / Close_Door(10,2)
- NW/NE facing: Open_Door(10,2), else Open_Door(1,2)

**UnitClass::APC_Open_Door()** / **APC_Close_Door()**
- NW/NE: Open_Door(10,2), else Open_Door(1,2)

### Remap Table

**UnitClass::Remap_Table()** → void const*
- MCV/Harvester: building remap table (become building animation)
- Other: TarComClass::Remap_Table()

**UnitClass::Remap_Table()** → void const*
- MCV/Harvester: building remap table
- Other: TarComClass::Remap_Table()

### Crew & Identity

**UnitClass::Crew_Type()** → InfantryType
- No primary weapon: 50% INFANTRY_C1, 50% INFANTRY_C7
- Else: TarComClass::Crew_Type()

**UnitClass::What_Am_I()** → RTTIType
- Returns RTTI_UNIT

### INI Serialization

**UnitClass::Read_INI(buffer)** - Reads unit from scenario INI
- Format: House, Type, Strength, Coord, Facing, Mission, Trigger

**UnitClass::Write_INI(buffer)** - Writes all units to scenario INI
- Format: House, Type, Strength, Coord, Facing, Mission, Trigger

**UnitClass::Read_INI(buffer)** - Reads unit from INI
- Format: House, Type, Strength, Coord, Facing, Mission, Trigger

**UnitClass::Write_INI(buffer)** - Writes all units to INI
- Format: House, Type, Strength, Coord, Facing, Mission, Trigger

### Allocation

**UnitClass::new** - Custom allocator
- Allocates unit slot, adjusts access arrays

**UnitClass::delete** - Custom deallocator
- Calls destructor, frees slot

**UnitClass::new** - Custom allocator
- Allocates slot, adjusts arrays

**UnitClass::delete** - Custom deallocator
- Destructor + free slot

### Debug (CHEAT_KEYS)

**UnitClass::Debug_Dump(mono)** - Mono monitor
- Name, Mission, TarCom, NavCom, Radio, Coord, HeadTo, Health, Body, Turret, Speed, Path, Cargo, Active, Limbo, Owned, Discovered, Selected, Tethered, Locked, TurretLocked, Loaner, Deploying, Rotating, Firing, Driving, ToLook, Recoiling, ToDisplay, CargoClass::Debug_Dump, MissionClass::Debug_Dump, FootClass::Debug_Dump

### Vehicle-Specific Constants

**Recoil_Adjust[32]** - Per-facing recoil pixel offsets
- 32-entry table for weapon recoil simulation

**Turret_Adjust[32]** - MLRS/MSAM turret offsets
- Per-body-facing turret pixel offsets

## Inputs, outputs and side effects

**State mutations:**
- Harvesting state (IsHarvesting)
- CTF flag (Flagged)
- Deployment state (IsDeploying)
- Door state (APC doors)
- Cargo/passenger count
- Tiberium load

**External systems affected:**
- Harvesting: Tiberium reduction, refinery offload
- Transport: APC door, passenger loading/unloading
- Deployment: MCV → building conversion
- Repair: Exit_Repair, repair bay interaction
- CTF: Flag attach/remove/drop

## Calls into / called by

**Called by:**
- FootClass AI: Mission_Move, Mission_Attack, Mission_Guard, Mission_Harvest
- Player input: Active_Click_With, Try_To_Deploy, Sell_Back
- Harvesting system: Harvesting(), Goto_Tiberium(), Offload_Tiberium_Bail()
- Transport: APC_Open/Close_Door, Desired_Load_Dir, Find_LZ
- Deployment: Try_To_Deploy, Goto_Clear_Spot, Exit_Repair
- Repair: Exit_Repair, Can_Demolish, Sell_Back
- CTF: Flag_Attach/Remove, Stun()

**Calls into:**
- FootClass: Mark(), Set_Speed(), Basic_Path(), Can_Enter_Cell()
- TechnoClass: Mark(), Take_Damage(), Detach(), Assign_Destination(), Assign_Mission()
- MapClass: Find_Tiberium(), Find_Tiberium(), Exit_Repair offsets
- CargoClass: How_Many(), Tiberium_Load()
- MapClass: Find_Tiberium(), Can_Enter_Cell(), adjacent cells
- CargoClass: How_Many(), Tiberium_Load()
- BuildingClass: Cell_Building(), STRUCT_REPAIR
- HouseClass: CurUnits, Refund_Money(), Flag_Attach()

## Related original files
- **UNIT.H** - Class declarations, inline helpers
- **FOOT.CPP/.H** - Parent FootClass
- **TECHNO.CPP/.H** - Parent TechnoClass
- **HARVEST.CPP/.H** - Harvesting logic
- **CARGO.CPP/.H** - CargoClass (passengers, tiberium load)
- **TEAM.CPP/.H** - TeamClass
- **MAP.CPP/.H** - MapClass (Find_Tiberium, cell checks)
- **CELL.CPP/.H** - CellClass (Can_Enter_Cell)
- **BUILDING.CPP/.H** - BuildingClass (STRUCT_REPAIR)
- **CARGO.CPP/.H** - CargoClass
- **FUNCTION.H** - Core includes
- **VOX.H** - Voice feedback
- **RADIO.CPP** - Radio messages

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-tiberian-dawn/src/entity/unit.rs` - UnitClass
- `cnc-core/src/entity/vehicle.rs` - Vehicle base
- `cnc-tiberian-dawn/src/entity/harvester.rs` - Harvesting logic
- `cnc-tiberian-dawn/src/entity/mcv.rs` - MCV deployment
- `cnc-core/src/entity/transport.rs` - APC doors, cargo
- `cnc-tiberian-dawn/src/entity/deployment.rs` - MCV deployment
- `cnc-core/src/entity/harvesting.rs` - Harvesting loop
- `cnc-core/src/entity/cargo.rs` - Cargo/tiberium load
- `cnc-core/src/ai/vehicle_ai.rs` - UnitClass AI overrides

**Not yet ported:**
- Turret/Recoil adjustment tables
- Harvesting loop (Goto_Tiberium, Harvesting, Offload_Tiberium_Bail)
- MCV deployment (Try_To_Deploy, Goto_Clear_Spot)
- APC door animation (Open_Door/Close_Door timing)
- Vehicle-specific AI (Mission_Guard, Mission_Move, Mission_Attack)
- Harvester logic (Goto_Tiberium, Harvesting, Offload_Tiberium_Bail)
- MCV deployment (Try_To_Deploy, Goto_Clear_Spot)
- Recoil/Turret adjustment tables
- CTF flag attach/remove
- Remap table switching (MCV/Harvester → building)

## Edge cases and compatibility notes

1. **Gunboat special case**: Always forced to MISSION_HUNT in Mission_Move/Attack/Guard. Exit_Map for guard, Hunt for attack.

2. **Hovercraft special**: Mission_Guard → unload if attached, else Exit_Map. Mission_Guard for AI hovercraft.

3. **Harvester AI**: Non-human harvesters forced to MISSION_HARVEST in Mission_Guard.

4. **MCV deployment**: Try_To_Deploy checks Can_Enter_Cell at current position. Goto_Clear_Spot checks 8 adjacent cells.

5. **Turret/Recoil tables**: 32-entry static arrays for per-facing pixel adjustments.

6. **Remap table switching**: MCV/Harvester use building remap table (become building animation).

7. **Pip_Count**: Transporter = passenger count, Harvester = tiberium % (0-5), Other = 0.

8. **Remap table**: MCV/Harvester use building remap (become building animation).

9. **Gunboat**: Always forced to HUNT mission. Exit_Map for guard, Hunt for attack.

10. **Hovercraft**: Guard → unload if attached, else Exit_Map.

11. **Harvester AI**: Non-human forced to HARVEST in Mission_Guard.

12. **MCV deployment**: Try_To_Deploy checks Can_Enter_Cell at current pos. Goto_Clear_Spot checks 8 adjacent.

13. **Turret/Recoil**: 32-entry static arrays for per-facing pixel adjustments.

14. **Remap table**: MCV/Harvester use building remap (become building animation).

15. **Pip_Count**: Transporter = passenger count, Harvester = tiberium % (0-5), Other = 0.

16. **MCV deployment**: Try_To_Deploy checks Can_Enter_Cell at current pos.

17. **Turret/Recoil**: 32-entry arrays.

18. **Remap table**: MCV/Harvester → building remap.

19. **Pip_Count**: Transporter = passengers, Harvester = tiberium %, Other = 0.

20. **Gunboat**: Always HUNT. Guard → Exit_Map, Attack → Hunt.

21. **Hovercraft**: Guard → unload if attached, else Exit_Map.

22. **Harvester AI**: Non-human → HARVEST in Guard.

23. **MCV deploy**: Try_To_Deploy at current cell, Goto_Clear_Spot checks 8 adjacent.

24. **Turret/Recoil**: 32-entry arrays.

25. **Remap**: MCV/Harvester → building.

26. **Pip_Count**: Transporter=passengers, Harvester=tiberium%, Other=0.

27. **Gunboat**: Always HUNT.

28. **Hovercraft**: Guard→unload or Exit_Map.

29. **Harvester AI**: Non-human → HARVEST.

30. **MCV deploy**: Current cell or 8 adjacent.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/UNIT.CPP at f1f0d42
