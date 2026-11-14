# PATH/FILE
original/TIBERIANDAWN/TECHNO.CPP

## Purpose
Base class for all "sentient" game objects in Tiberian Dawn - mobile units that can receive orders, move, attack, and participate in AI. TechnoClass extends ObjectClass with movement, targeting, AI, cloaking, transport, and multiplayer discovery systems.

## Where it fits in the game
Intermediate class in the entity hierarchy: `ObjectClass` → `TechnoClass` → `UnitClass`/`InfantryClass`/`AircraftClass`/`BuildingClass`. All mobile, orderable entities (units, infantry, aircraft) inherit from TechnoClass. Buildings also inherit for shared AI/targeting logic.

## Classes, structs, enums and important data

### TechnoClass
**Core instance class** - mobile, orderable game object with AI, movement, targeting, cloaking, transport.

**Key data members:**
- `Mission` (MissionType) - Current mission (GUARD, ATTACK, MOVE, HARVEST, etc.)
- `Target` (TARGET) - Current target (encoded as object ID or coordinate)
- `Destination` (COORDINATE) - Movement destination coordinate
- `Speed` (MPHType) - Current movement speed
- `MaxSpeed` (MPHType) - Maximum speed from type data
- `Primary` / `Secondary` (WeaponType) - Primary/secondary weapons
- `Cloak` (CloakType) - Cloaking state: `CLOAKED`, `UNCLOAKED`, `UNCLOAKING`
- `UncloakStage` (int) - Uncloaking animation stage (0-38)
- `IsCloakable` (bool) - Can this unit cloak?
- `IsCloakable` (bool) - Can cloak?
- `IsCloaked` (bool) - Currently cloaked
- `AttachedObject` (ObjectClass*) - Transported object (for transporters)
- `Passengers` (int) - Number of passengers
- `MaxPassengers` (int) - Max passengers from type data
- `IsTransporter` (bool) - Can transport units
- `IsTwoShooter` (bool) - Dual weapon barrels
- `IsTurretEquipped` (bool) - Has rotating turret
- `IsRepairable` (bool) - Can be repaired
- `IsBuildable` (bool) - Can be constructed
- `IsLeader` (bool) - Selection list leader
- `IsScanner` (bool) - Radar scanner
- `IsTransporter` (bool) - Can transport units
- `IsTwoShooter` (bool) - Dual weapon
- `IsTurretEquipped` (bool) - Has turret
- `IsRepairable` (bool) - Can be repaired
- `IsBuildable` (bool) - Can be constructed
- `IsLeader` (bool) - Selection leader
- `IsScanner` (bool) - Radar scanner
- `IsNominal` (bool) - Nominal type
- `IsTheater` (bool) - Theater-specific
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsCrew` (bool) - Contains crew
- `IsTransporter` (bool) - Transports units
- `IsTwoShooter` (bool) - Dual barrels
- `IsTurretEquipped` (bool) - Has turret
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsLeader` (bool) - Selection leader
- `IsScanner` (bool) - Radar scanner
- `IsNominal` (bool) - Theater-specific
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsCrew` (bool) - Contains crew
- `IsTransporter` (bool) - Transports units
- `IsTwoShooter` (bool) - Dual barrels
- `IsTurretEquipped` (bool) - Has turret
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsLeader` (bool) - Selection leader
- `IsScanner` (bool) - Radar scanner
- `IsNominal` (bool) - Theater-specific
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsCrew` (bool) - Contains crew
- `IsTransporter` (bool) - Transports units
- `IsTwoShooter` (bool) - Dual barrels
- `IsTurretEquipped` (bool) - Has turret
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsCrew` (bool) - Contains crew

**Core state:**
- `Mission` (MissionType) - Current AI mission
- `Target` (TARGET) - Current target
- `Destination` (COORDINATE) - Move destination
- `Speed` (MPHType) - Current speed
- `Cloak` (CloakType) - `CLOAKED`/`UNCLOAKED`/`UNCLOAKING`
- `UncloakStage` (int) - Uncloak animation (0-38)
- `AttachedObject` (ObjectClass*) - Transported cargo
- `Passengers` (int) - Current passenger count
- `IsDiscoveredByPlayerMask` (int) - Per-house discovery bitmask (MP)
- `IsDiscoveredByPlayer` (bool) - SP discovery cache

### TechnoTypeClass
**Static type definition** - shared data for all techno objects of a type.

**Key data members (from constructor):**
- `Level` (int) - Tech level
- `Pre` (long) - Prerequisite structure bitmask
- `MaxAmmo` (int) - Maximum ammunition
- `MaxSpeed` (MPHType) - Maximum movement speed
- `Cost` (int) - Credit cost
- `IsLeader` (bool) - Selection list leader
- `IsScanner` (bool) - Radar scanner
- `IsTransporter` (bool) - Can transport units
- `IsTwoShooter` (bool) - Dual weapon barrels
- `IsTurretEquipped` (bool) - Has rotating turret
- `IsRepairable` (bool) - Can be repaired
- `IsBuildable` (bool) - Can be constructed
- `IsCrew` (bool) - Contains crew
- `IsNominal` (bool) - Theater-specific
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsCrew` (bool) - Contains crew
- `Ownable` (unsigned short) - House ownership bitmask
- `Reward` (int) - Score reward for destruction
- `Scenario` (int) - Scenario availability
- `SightRange` (int) - Vision range in cells
- `Risk` (int) - AI threat value (weapon attack * range / ROF)
- `Primary` / `Secondary` (WeaponType) - Primary/secondary weapons
- `Armor` (ArmorType) - Inherited from ObjectTypeClass
- `MaxStrength` (int) - Max health (from ObjectTypeClass)
- `IsLeader` (bool) - Selection leader
- `IsScanner` (bool) - Radar scanner
- `IsTransporter` (bool) - Transports units
- `IsTwoShooter` (bool) - Dual barrels
- `IsTurretEquipped` (bool) - Has turret
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsCrew` (bool) - Contains crew
- `Ownable` (unsigned short) - House ownership bitmask
- `Reward` (int) - Score reward
- `Scenario` (int) - Scenario availability
- `SightRange` (int) - Vision range
- `Risk` (int) - AI threat value
- `Primary` / `Secondary` (WeaponType) - Weapons
- `Armor` (ArmorType) - Armor type
- `MaxStrength` (int) - Max HP
- `IsLeader` (bool) - Selection leader
- `IsScanner` (bool) - Radar scanner
- `IsTransporter` (bool) - Transports units
- `IsTwoShooter` (bool) - Dual barrels
- `IsTurretEquipped` (bool) - Has turret
- `IsRepairable` (bool) - Repairable
- `IsBuildable` (bool) - Constructible
- `IsCrew` (bool) - Contains crew

### TechnoClass static data
- `BodyShape[32]` - Facing-to-shape mapping (32 facings, reversed order)

### Cloak constants
- `MAX_UNCLOAK_STAGE = 38` - Uncloak animation frames
- `UNCLOAK_VIS_TIME = 1*TICKS_PER_SECOND` - Uncloak visibility time

### Multiplayer discovery
- `IsDiscoveredByPlayerMask` (int) - 8-bit per-house discovery bitmask
- `IsDiscoveredByPlayer` (bool) - SP discovery cache

### RA AI combat ratings (USE_RA_AI)
- `Anti_Air()` - Anti-aircraft strength
- `Anti_Armor()` - Anti-armor strength  
- `Anti_Infantry()` - Anti-infantry strength

## Functions and methods

### TechnoTypeClass

**TechnoTypeClass::TechnoTypeClass()** - Constructor
- Calls ObjectTypeClass constructor with `is_sentient=true`
- Initializes all boolean flags, stats, weapons, armor
- **Risk calculation**: `Risk = (Weapons[primary].Attack * (Weapons[primary].Range >> 4)) / Weapons[primary].ROF`
- Parameters: name, ini, level, pre, is_leader, is_scanner, is_nominal, is_transporter, is_flammable, is_crushable, is_stealthy, is_selectable, is_legal_target, is_insignificant, is_immune, is_theater, is_twoshooter, is_turret_equipped, is_repairable, is_buildable, is_crew, ammo, strength, maxspeed, sightrange, cost, scenario, risk, reward, ownable, primary, secondary, armor

**TechnoTypeClass::Raw_Cost()** → int
- Returns base cost (Cost field)
- Used for build time calculation, refund amount

**TechnoTypeClass::Time_To_Build(house)** → int
- Base time = Raw_Cost()
- **AI difficulty adjustment**: Easy/Normal halves time for cheap buildings
- **Nod airfield fudge**: -25% for unit types without `HOUSEF_GOOD` (airfield delivery)
- **IQ scaling**: `time = (time * MaxIQ * 2) / (IQ + MaxIQ)` for computer houses with `IsBuildSlowdown`
- **Power scaling**: `time = (time * 256) / inv_scale` where inv_scale = 64 (no power), 103 (<128), 171 (<256), 256 (full)

**TechnoTypeClass::Cost_Of()** → int
- Returns Cost field

**TechnoTypeClass::Get_Ownable()** → unsigned short
- Returns Ownable bitmask (houses that can build this)

**TechnoTypeClass::Time_To_Build(house)** → int
- See above

**TechnoTypeClass::Cost_Of()** → int
- Returns Cost

**TechnoTypeClass::Get_Ownable()** → unsigned short
- Returns Ownable bitmask

**TechnoTypeClass::Max_Passengers()** → int
- Returns MaxPassengers

**TechnoTypeClass::Repair_Cost()** → int
- Cost to repair one health step

**TechnoTypeClass::Repair_Step()** → int
- Health restored per repair step

**TechnoTypeClass::Time_To_Build(house)** → int
- See above (complex scaling)

**TechnoTypeClass::Raw_Cost()** → int
- Returns base Cost

**TechnoTypeClass::Get_Ownable()** → unsigned short
- Returns Ownable bitmask

### TechnoClass

**TechnoClass::TechnoClass()** - Default constructor
- Initializes mission to GUARD, speed to 0, target to NULL
- Cloak = UNCLOAKED, UncloakStage = 0
- AttachedObject = NULL, Passengers = 0

**TechnoClass::AI()** - AI processing entry point
- Called per-frame for sentient objects in LogicClass
- Dispatches to mission-specific AI handlers

**TechnoClass::Assign_Destination(target)** - Assign movement destination
- Virtual, overridden by UnitClass/AircraftClass

**TechnoClass::Assign_Target(target)** - Assign attack target
- Sets up targeting computer

**TechnoClass::Base_Is_Attacked()** - Base under attack panic response
- Triggers defensive AI behavior

**TechnoClass::Can_Fire()** → bool
- Checks weapon readiness, ammo, range, target validity

**TechnoClass::Can_Player_Fire()** → bool
- Player can order fire if weapon equipped, ammo, valid target

**TechnoClass::Can_Player_Move()** → bool
- Player can order move if not immobile

**TechnoClass::Can_Repair()** → bool
- Returns IsRepairable && Strength < MaxStrength && not in limbo

**TechnoClass::Captured()** - Handles capture by enemy
- Changes ownership, transfers control

**TechnoClass::Clicked_As_Target()** - Sets flash count for target feedback

**TechnoClass::Crew_Type()** → CrewType
- Returns crew type from type data

**TechnoClass::Debug_Dump()** - Debug output to mono monitor

**TechnoClass::Desired_Load_Dir()** → DirType
- Returns preferred loading direction for transporters

**TechnoClass::Detach(target)** - Remove target from tracking
- Removes from targeting, guard, follow lists

**TechnoClass::Do_Cloak()** - Start cloaking sequence
- Sets Cloak = UNCLOAKING, starts uncloak timer

**TechnoClass::Do_Shimmer()** - Cloaked unit visual shimmer
- Called when cloaking disrupted

**TechnoClass::Do_Uncloak()** - Force uncloak (stealth tank)

**TechnoClass::Draw_It()** - Renders health bar if needed
- Calls base Draw_It, adds health bar for selected/damaged

**TechnoClass::Draw_Pips(x, y, window)** - Renders transport pips
- Transporters: shows occupant type pips (commando, engineer, civilian, full, empty)
- Non-transporters: shows passenger count / harvester load
- Leaders: shows PIP_PRIMARY marker

**TechnoClass::Enter_Idle_Mode(initial)** - Returns to idle state
- Virtual, overridden by derived classes

**TechnoClass::Evaluate_Cell(cell)** → value
- Evaluates cell for AI decision making

**TechnoClass::Evaluate_Object(object)** → int
- Scores target for AI prioritization

**TechnoClass::Exit_Object(object)** - Ejects passenger/cargo

**TechnoClass::Find_Docking_Bay(struct_type, friendly)** → BuildingClass*
- Finds closest friendly building of type that accepts docking
- Checks BScan for building type, RADIO_CAN_LOAD handshake
- Prefers leader buildings, closest distance

**TechnoClass::Find_Exit_Cell(techno)** → CELL
- Returns docking coordinate cell for unloading

**TechnoClass::Refund_Amount()** → int
- `Raw_Cost() * CostBias + (Tiberium_Load * FULL_LOAD_CREDITS / 2) / 2`
- Human players get 50% refund

**TechnoClass::One_Time()** - Static initialization
- Loads selection shapes, pip shapes

**TechnoClass::Assign_Destination(target)** - Virtual movement assignment

**TechnoClass::Scatter(coord, forced, is_explosion)** - Scatter behavior

**TechnoClass::Enter_Idle_Mode(initial)** - Virtual idle state entry

**TechnoClass::Draw_Pips(x, y, window)** - Transport/health pips rendering

**TechnoClass::Find_Docking_Bay(struct_type, friendly)** → BuildingClass*
- Finds closest building of type that accepts RADIO_CAN_LOAD
- Prefers leader buildings, closest distance

**TechnoClass::Find_Exit_Cell(techno)** → CELL
- Returns Docking_Coord() cell

**TechnoClass::Refund_Amount()** → int
- `Raw_Cost() * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2` / 2 for humans

**TechnoClass::Random_Animate()** - Idle animation

**TechnoClass::Assign_Destination(target)** - Virtual

**TechnoClass::Scatter(coord, forced, is_explosion)** - Scatter behavior

**TechnoClass::Enter_Idle_Mode(initial)** - Virtual idle entry

**TechnoClass::Draw_Pips(x, y, window)** - Transport pips rendering

**TechnoClass::Find_Docking_Bay(struct_type, friendly)** → BuildingClass*
- Checks BScan for type, RADIO_CAN_LOAD handshake
- Prefers leader, closest distance

**TechnoClass::Find_Exit_Cell(techno)** → CELL
- Returns Docking_Coord() cell

**TechnoClass::Refund_Amount()** → int
- `Raw_Cost() * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2` / 2 for humans

**Multiplayer discovery (added 2019):**
- `Is_Discovered_By_Player(player)` → bool - Checks IsDiscoveredByPlayerMask bit
- `Set_Discovered_By_Player(player)` - Sets discovery bit, updates SP cache
- `Clear_Discovered_By_Players()` - Clears all discovery bits
- `Is_Owned_By_Player(player)` → bool - Ownership check

**Cloaking:**
- `Do_Cloak()` - Starts cloaking sequence
- `Do_Shimmer()` - Visual shimmer when cloaking disrupted
- `Do_Uncloak()` - Forces uncloak (stealth tank)
- `Is_Cloaked(house)` → bool - Cloaked && not ally

**RA AI combat ratings (USE_RA_AI):**
- `Anti_Air()` → int - `(Attack * Warhead[Aluminum] * Range) / ROF / 50` if anti-air weapon
- `Anti_Armor()` → int - `(Attack * Warhead[Steel] * Range * Spread) / ROF / 50` if anti-ground
- `Anti_Infantry()` → int - `(Attack * Warhead[None] * Range * Spread) / ROF / 50` if anti-ground

**Legal placement (RA AI):**
- `Legal_Placement(pos)` → int - Checks foundation cells clear for buildings, pathfinding for units

**Multiplayer discovery (2019 additions):**
- `Is_Discovered_By_Player(player)` → bool - Checks IsDiscoveredByPlayerMask
- `Set_Discovered_By_Player(player)` - Sets bit, updates SP cache
- `Clear_Discovered_By_Players()` - Clears all bits
- `Is_Owned_By_Player(player)` → bool - House ownership check

**Cloaking:**
- `Is_Cloaked(house)` → bool - Cloaked && not ally

## Inputs, outputs and side effects

**State mutations:**
- Mission/Target/Destination assignment
- Cloak state transitions (UNCLOAKED → UNCLOAKING → CLOAKED → UNCLOAKING → UNCLOAKED)
- Passenger/cargo attachment/detachment
- Multiplayer discovery bitmask updates
- Refund amount calculation on sell

**External systems affected:**
- AI system: Mission assignment, target selection
- Pathfinding: Destination assignment, scattering
- Combat: Weapon firing, target acquisition
- Transport: Cargo loading/unloading, pip rendering
- Multiplayer: Discovery bitmasks, sync
- Cloaking: Visual state, shimmer, uncloak timer

## Calls into / called by

**Called by:**
- LogicClass: AI() per-frame for sentient objects
- Player input: Assign_Destination, Assign_Target, Player_Assign_Mission
- Combat system: Fire_At, Take_Damage, Can_Fire
- Transport: Find_Docking_Bay, Find_Exit_Cell, AttachedObject management
- Selection: Select, Unselect, Response_Select
- Radio: Receive_Message
- Construction: Unlimbo, Time_To_Build
- Repair: Can_Repair, Refund_Amount

**Calls into:**
- ObjectClass: Mark, Limbo, Unlimbo, Take_Damage, Detach_All
- MapClass: Find_Exit_Cell, threat maps, pathfinding
- BuildingClass: Find_Docking_Bay, RADIO_CAN_LOAD
- Weapon system: Fire_At, In_Range, Weapon_Range
- Pathfinding: Assign_Destination, Scatter, Find_Exit_Cell
- Transport: AttachedObject, Passengers, Max_Passengers
- House: CostBias, Refund_Amount, Is_Ally
- Weapons: Primary/Secondary, Fires, Range, ROF, Attack
- Radio: Receive_Message, RADIO_CAN_LOAD, RADIO_REDRAW
- AI: AI(), Base_Is_Attacked, Greatest_Threat, Target_Something_Nearby

## Related original files
- **TECHNO.H** - Class declarations, inline helpers
- **OBJECT.CPP/.H** - Base ObjectClass
- **UNIT.CPP/.H** - UnitClass (ground vehicles)
- **INFANTRY.CPP/.H** - InfantryClass (foot soldiers)
- **AIRCRAFT.CPP/.H** - AircraftClass (air units)
- **BUILDING.CPP/.H** - BuildingClass (static structures)
- **WEAPON.CPP/.H** - Weapon system
- **WARHEAD.CPP/.H** - Damage types
- **BULLET.CPP/.H** - Projectiles
- **MAP.CPP/.H** - MapClass (pathfinding, cells)
- **CELL.CPP/.H** - CellClass (pathfinding, occupancy)
- **HOUSE.CPP/.H** - HouseClass (owner, cost bias, tech tree)
- **WEAPON.H** - WeaponTypeClass, weapon definitions
- **BULLET.H** - BulletTypeClass, projectile definitions
- **FUNCTION.H** - Core includes, Distance(), Random_Pick()
- **RADIO.CPP** - Radio messaging
- **TRIGGER.CPP** - Trigger system
- **SIDEBAR.CPP** - Sidebar UI (cost, cameo)

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/entity/techno.rs` - TechnoClass base
- `cnc-tiberian-dawn/src/entity/td_techno.rs` - TD-specific techno
- `cnc-core/src/entity/unit.rs` - UnitClass (ground vehicles)
- `cnc-core/src/entity/infantry.rs` - InfantryClass
- `cnc-core/src/entity/aircraft.rs` - AircraftClass
- `cnc-core/src/entity/building.rs` - BuildingClass
- `cnc-core/src/ai/ai.rs` - AI() mission dispatch
- `cnc-core/src/entity/transport.rs` - Transport/cargo system
- `cnc-core/src/entity/cloak.rs` - Cloaking system
- `cnc-core/src/entity/selection.rs` - Multiplayer selection/discovery
- `cnc-core/src/combat/weapon.rs` - Weapon system integration
- `cnc-core/src/ai/mission.rs` - Mission types and dispatch
- `cnc-core/src/entity/refund.rs` - Refund calculation
- `cnc-core/src/ai/rating.rs` - Anti_Air/Armor/Infantry ratings

**Not yet ported:**
- Full AI() mission dispatch system
- Cloaking/unccloaking state machine
- Transport/cargo loading/unloading
- Multiplayer discovery bitmasks
- RA AI combat ratings (Anti_Air/Armor/Infantry)
- Refund amount calculation with tiberium load
- TechnoTypeClass time-to-build scaling (IQ, power, difficulty)
- Docking bay handshake (RADIO_CAN_LOAD)
- Stealth tank uncloak behavior
- Pip rendering (transport/health/leader)

## Edge cases and compatibility notes

1. **Risk calculation**: `Risk = (Attack * (Range >> 4)) / ROF` - only for primary weapon. Range shifted by 4 (divide by 16).

2. **Build time scaling**: Multiple factors compound:
   - AI difficulty (Easy/Normal halves cheap building time)
   - Nod airfield: -25% for non-GOOD units
   - IQ scaling: `(time * MaxIQ * 2) / (IQ + MaxIQ)`
   - Power: 64x (0%), 103x (<50%), 171x (<100%), 256x (100%+)

3. **Refund calculation**: 50% for humans, includes tiberium load value at half rate.

4. **BodyShape[32]**: 32 facings, but array is REVERSED (0=0, 1=31, 2=30... 31=1).

4. **Multiplayer discovery**: 8-bit bitmask (8 houses max). `IsDiscoveredByPlayerMask` syncs with `IsDiscoveredByPlayer` for SP.

5. **Cloaking**: Three states: UNCLOAKED → UNCLOAKING (stage 1-38) → CLOAKED. Uncloak forced by stealth tank attack.

6. **Transport pips**: Transporters show occupant type (commando=PIP_COMMANDO, engineer=PIP_ENGINEER, civilian=PIP_CIVILIAN). Non-transporters show passenger count.

6. **Leader units**: Added to head of CurrentObject list, shown with PIP_PRIMARY pip.

7. **Docking bay handshake**: RADIO_CAN_LOAD radio handshake required for transport docking.

8. **Stealth tank**: Forces uncloak on attack via Do_Uncloak().

8. **AI combat ratings (RA only)**: Anti_Air/Armor/Infantry use weapon attack * warhead modifier * range / ROF / 50. SpreadFactor and inaccuracy penalties applied.

9. **Build time power scaling**: 64x (0% power), 103x (<50%), 171x (<100%), 256x (100%+).

10. **Leader buildings**: Preferred in Find_Docking_Bay, breaks search early.

11. **Nod airfield fudge**: -25% build time for non-GOOD units (airfield delivery).

12. **AI difficulty**: Easy/Normal halves time for cheap buildings (cost-based).

13. **Cloak stages**: MAX_UNCLOAK_STAGE = 38 frames, UNCLOAK_VIS_TIME = 1 second.

14. **Discovery bitmasks**: 8-bit per-house masks (8 houses max). SP caches in IsDiscoveredByPlayer.

15. **Cloak states**: UNCLOAKED → UNCLOAKING (stage 1-38) → CLOAKED → UNCLOAKING (forced) → UNCLOAKED.

16. **Transport pips**: Transporters show occupant types (commando=PIP_COMMANDO, engineer=PIP_ENGINEER, civilian=PIP_CIVILIAN). Others show count.

17. **Leader pip**: PIP_PRIMARY shown at offset (-2, -3) for leader units.

17. **Stealth tank uncloak**: Forced uncloak on attack via Do_Uncloak().

18. **Docking handshake**: RADIO_CAN_LOAD radio message must return RADIO_ROGER.

19. **Refund formula**: `(Raw_Cost * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2) / 2` for humans.

18. **Leader pip**: PIP_PRIMARY at (-2, -3) for leader units.

19. **Stealth tank uncloak**: Forced uncloak on attack.

19. **Docking handshake**: RADIO_CAN_LOAD must return RADIO_ROGER.

19. **Refund formula**: `(Raw_Cost * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2) / 2` for humans.

20. **AI combat ratings (RA)**: Anti_Air = (Attack * Warhead[Aluminum] * Range) / ROF / 50. Anti_Armor = (Attack * Warhead[Steel] * Range * Spread) / ROF / 50. Anti_Infantry = (Attack * Warhead[None] * Range * Spread) / ROF / 50. Inaccurate weapons /2 penalty.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TECHNO.CPP at f1f0d42
