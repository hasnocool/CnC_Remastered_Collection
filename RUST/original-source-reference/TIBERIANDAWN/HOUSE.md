# PATH/FILE
original/TIBERIANDAWN/HOUSE.CPP

## Purpose
House/player management system for Tiberian Dawn. Implements player/faction management including economy, production, diplomacy, superweapons, AI behavior, and multiplayer support.

## Where it fits in the game
Core player/faction management - every player (human or AI) is represented by a HouseClass instance. Manages economy, production queues, unit/building ownership, diplomacy, superweapons, and AI decision-making.

## Classes, structs, enums and important data

### HouseClass
**Core player/faction class** - represents a player/faction in the game.

**Key data members:**
- `Class` (HouseTypeClass*) - Static type data pointer
- `Control` (HouseControl) - Runtime control settings
- `Difficulty` (DiffType) - AI difficulty level
- `IsHuman` (bool) - Human player flag
- `IsPlayerControl` (bool) - Local player control
- `IsDefeated` (bool) - Defeated flag
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid mode
- `IsHuman` (bool) - Human player
- `IsPlayerControl` (bool) - Local player
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `IsHuman` (bool) - Human
- `IsPlayerControl` (bool) - Local player
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `Allies` (unsigned) - Ally bitmask
- `BScan` (unsigned long) - Building scan bitmask
- `UScan` (unsigned long) - Unit scan bitmask
- `IQ` (int) - AI IQ (0-MaxIQ)
- `Difficulty` (DiffType) - AI difficulty
- `Edge` (SourceType) - Map edge
- `IsPlayerControl` (bool) - Local player
- `IsHuman` (bool) - Human player
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `IsHuman` (bool) - Human
- `IsPlayerControl` (bool) - Local player
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `IsHuman` (bool) - Human
- `IsPlayerControl` (bool) - Local player
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `Allies` (unsigned) - Ally bitmask
- `BScan` (unsigned long) - Building types owned
- `UScan` (unsigned long) - Unit scan bitmask
- `IQ` (int) - AI IQ
- `Difficulty` (DiffType) - AI difficulty
- `Edge` (SourceType) - Map edge
- `IsPlayerControl` (bool) - Local player
- `IsHuman` (bool) - Human
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `Credits` (long) - Current credits (x100)
- `Control.InitialCredits` (long) - Starting credits
- `CostBias` (int) - Cost multiplier
- `FirepowerBias` (int) - Damage multiplier
- `GroundspeedBias` (int) - Ground speed multiplier
- `AirspeedBias` (int) - Air speed multiplier
- `ArmorBias` (int) - Armor multiplier
- `ROFBias` (int) - Rate of fire multiplier
- `CostBias` (int) - Cost multiplier
- `RepairDelay` (int) - Repair delay
- `BuildDelay` (int) - Build delay
- `BuildSpeedBias` (int) - Build speed multiplier
- `Power` (int) - Current power output
- `Drain` (int) - Current power drain
- `Power` (int) - Power output
- `Drain` (int) - Power drain
- `Blockage` (int) - AllowWin blockage counter
- `BorrowedTime` (int) - AllowWin grace time
- `IsAlerted` (bool) - AI alerted
- `BScan` (unsigned long) - Building scan bitmask
- `UScan` (unsigned long) - Unit scan bitmask
- `IQ` (int) - AI IQ
- `Difficulty` (DiffType) - AI difficulty
- `Edge` (SourceType) - Map edge
- `IsPlayerControl` (bool) - Local player
- `IsHuman` (bool) - Human
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `Credits` (long) - Current credits (x100)
- `Control.InitialCredits` (long) - Starting credits
- `CostBias` (int) - Cost multiplier
- `FirepowerBias` (int) - Damage multiplier
- `GroundspeedBias` (int) - Ground speed multiplier
- `AirspeedBias` (int) - Air speed multiplier
- `ArmorBias` (int) - Armor multiplier
- `ROFBias` (int) - Rate of fire multiplier
- `CostBias` (int) - Cost multiplier
- `RepairDelay` (int) - Repair delay
- `BuildDelay` (int) - Build delay
- `BuildSpeedBias` (int) - Build speed multiplier
- `Power` (int) - Power output
- `Drain` (int) - Power drain
- `Blockage` (int) - AllowWin blockage
- `BorrowedTime` (int) - AllowWin grace time
- `IsAlerted` (bool) - AI alerted
- `BScan` (unsigned long) - Building scan
- `UScan` (unsigned long) - Unit scan
- `IQ` (int) - AI IQ
- `Difficulty` (DiffType) - AI difficulty
- `Edge` (SourceType) - Map edge
- `IsPlayerControl` (bool) - Local player
- `IsHuman` (bool) - Human
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `Allies` (unsigned) - Ally bitmask
- `BScan` (unsigned long) - Building scan
- `UScan` (unsigned long) - Unit scan
- `IQ` (int) - AI IQ
- `Difficulty` (DiffType) - AI difficulty
- `Edge` (SourceType) - Map edge
- `IsPlayerControl` (bool) - Local player
- `IsHuman` (bool) - Human
- `IsDefeated` (bool) - Defeated
- `IsParanoid` (bool) - AI paranoid
- `Credits` (long) - Current credits (x100)
- `Control.InitialCredits` (long) - Starting credits
- `Control.MaxBuilding` (int) - Max buildings
- `Control.MaxUnit` (int) - Max units
- `Control.MaxInfantry` (int) - Max infantry
- `Control.MaxVessel` (int) - Max vessels
- `Control.TechLevel` (int) - Tech level
- `Control.IQ` (int) - Control IQ
- `Control.Edge` (SourceType) - Map edge
- `Control.IsPlayerControl` (bool) - Local player
- `Control.Allies` (unsigned) - Ally bitmask
- `Control.InitialCredits` (long) - Starting credits
- `Control.MaxBuilding` (int) - Max buildings
- `Control.MaxUnit` (int) - Max units
- `Control.MaxInfantry` (int) - Max infantry
- `Control.MaxVessel` (int) - Max vessels
- `Control.TechLevel` (int) - Tech level
- `Control.IQ` (int) - Control IQ
- `Control.Edge` (SourceType) - Map edge
- `Control.IsPlayerControl` (bool) - Local player
- `Control.Allies` (unsigned) - Ally bitmask

### HouseTypeClass (static)
- `House` (HousesType) - House type enum
- `IniName` (char*) - INI name
- `FirepowerBias` (int) - Base firepower bias
- `GroundspeedBias` (int) - Base ground speed bias
- `AirspeedBias` (int) - Base air speed bias
- `ArmorBias` (int) - Base armor bias
- `ROFBias` (int) - Base ROF bias
- `CostBias` (int) - Base cost bias
- `BuildSpeedBias` (int) - Base build speed bias
- `RepairDelay` (int) - Repair delay
- `BuildDelay` (int) - Build delay

### Key constants
- `HOUSE_MAX` - Max houses (8)
- `HOUSE_FIRST` - First house (0)
- `HOUSE_MULTI1` - First multiplayer house
- `HOUSE_COUNT` - Total houses
- `HOUSE_NEUTRAL` - Neutral house
- `HOUSE_BAD` - Bad guys (NOD)
- `HOUSE_GOOD` - Good guys (GDI)
- `HOUSE_COUNT` - Total houses (8)
- `HOUSE_MAX` - Max houses (8)
- `HOUSE_FIRST` - First house (0)
- `HOUSE_MULTI1` - First multiplayer
- `HOUSE_COUNT` - Total (8)
- `HOUSE_NEUTRAL` - Neutral
- `HOUSE_BAD` - NOD
- `HOUSE_GOOD` - GDI

### HouseControl struct
- `InitialCredits` (long) - Starting credits (x100)
- `MaxBuilding` (int) - Max buildings
- `MaxUnit` (int) - Max units
- `MaxInfantry` (int) - Max infantry
- `MaxVessel` (int) - Max vessels
- `TechLevel` (int) - Tech level
- `IQ` (int) - AI IQ
- `Edge` (SourceType) - Map edge
- `IsPlayerControl` (bool) - Local player
- `Allies` (unsigned) - Ally bitmask
- `FirepowerBias` (int) - Firepower bias
- `GroundspeedBias` (int) - Ground speed bias
- `AirspeedBias` (int) - Air speed bias
- `ArmorBias` (int) - Armor bias
- `ROFBias` (int) - ROF bias
- `CostBias` (int) - Cost bias
- `RepairDelay` (int) - Repair delay
- `BuildDelay` (int) - Build delay
- `BuildSpeedBias` (int) - Build speed bias

### VTable
- Captured in Init() via dummy allocation

### DiffType enum
- `DIFF_EASY` - Easy
- `DIFF_NORMAL` - Normal
- `DIFF_HARD` - Hard

## Functions and methods

### Core Management

**HouseClass::HouseClass()** - Default constructor
- Initializes all members to defaults
- Credits = 0, Power = 0, Drain = 0, etc.

**HouseClass::HouseClass(index)** - Normal constructor
- Initializes house with index
- Sets up default control values

**HouseClass::~HouseClass()** - Destructor
- Cleans up house resources

**HouseClass::One_Time()** - One-time initialization
- Creates all house instances
- Initializes RA AI BuildChoice heap

**HouseClass::Init()** - Scenario initialization
- Resets house state for new scenario
- Called before scenario load

**HouseClass::operator HousesType()** - Conversion operator
- Returns `Class->House`

**HouseClass::As_Pointer(house)** → HouseClass*
- Finds house object by HousesType enum
- Linear search through Houses array

**HouseClass::As_Pointer(house)** → HouseClass*
- Linear search through Houses array

**HouseClass::operator HousesType()** - Conversion
- Returns `Class->House`

**HouseClass::One_Time()** - One-time init
- Creates house instances
- RA AI: BuildChoice.Set_Heap(STRUCT_COUNT)

**HouseClass::Assign_Handicap(handicap)** → DiffType
- Sets Difficulty, returns old
- **MP**: Uses ActLike house type for biases
- **SP**: Uses Rule.Diff[handicap] biases
- Updates: FirepowerBias, GroundspeedBias, AirspeedBias, ArmorBias, ROFBias, CostBias, RepairDelay, BuildDelay, BuildSpeedBias

**HouseClass::Assign_Handicap(handicap)** → DiffType
- Sets Difficulty, returns old
- MP: Uses ActLike house biases * Rule.Diff
- SP: Uses Rule.Diff[handicap] directly
- Updates all bias fields

### Economy & Resources

**HouseClass::Available_Money()** → long
- Returns `Credits / 100` (credits stored as x100)

**HouseClass::Spend_Money(amount)** → bool
- Deducts amount from Credits
- Returns false if insufficient funds

**HouseClass::Refund_Money(amount)** - Adds money
- `Credits += amount * 100`

**HouseClass::Harvested(amount)** - Adds tiberium credits
- `Credits += amount`

**HouseClass::Available_Money()** → long
- Returns `Credits / 100`

**HouseClass::Spend_Money(amount)** → bool
- Deducts from Credits, returns false if insufficient

**HouseClass::Refund_Money(amount)** - Adds money
- `Credits += amount * 100`

**HouseClass::Harvested(amount)** - Adds tiberium
- `Credits += amount`

**HouseClass::Adjust_Capacity(amount)** - Adjusts tiberium storage
- Adjusts silo capacity

**HouseClass::Power_Fraction()** → int
- Returns power fraction (0-256)
- 0 = no power, 256 = full power
- Used for build time scaling

**HouseClass::Power_Fraction()** → int
- Returns 0-256 power fraction
- 0 = no power, 256 = full

**HouseClass::Adjust_Power(adjust)** - Adjusts power output
- `Power += adjust`

**HouseClass::Adjust_Drain(adjust)** - Adjusts power drain
- `Drain += adjust`

### Production & Building

**HouseClass::Begin_Production()** - Starts AI production
- Sets IsAlerted = true
- Enables production queues

**HouseClass::Suspend_Production()** - Pauses production
- Clears IsAlerted

**HouseClass::Begin_Production()** - Starts AI production
- Sets IsAlerted = true

**HouseClass::Suspend_Production()** - Pauses production
- Clears IsAlerted

**HouseClass::Begin_Production()** - Starts AI production
- Sets IsAlerted = true

**HouseClass::Can_Build(building_type)** → bool
- Checks prerequisites, tech level, build limit
- Returns true if buildable

**HouseClass::Can_Build(unit_type)** → bool
- Checks prerequisites, tech level, build limit

**HouseClass::Can_Build(infantry_type)** → bool
- Checks prerequisites, tech level, build limit

**HouseClass::Can_Build(unit_type)** → bool
- Checks prerequisites, tech level, build limit

**HouseClass::Can_Build(rtti, value)** → bool
- Generic build legality checker
- Dispatches to specific Can_Build

**HouseClass::Can_Build(building_type)** → bool
- Checks prerequisites, tech level, build limit

**HouseClass::Can_Build(unit_type)** → bool
- Checks prerequisites, tech level, build limit

**HouseClass::Can_Build(infantry_type)** → bool
- Checks prerequisites, tech level, build limit

**HouseClass::Can_Build(unit_type)** → bool
- Checks prerequisites, tech level, build limit

**HouseClass::Can_Build(rtti, value)** → bool
- Generic dispatcher

**HouseClass::Place_Object(building_type, cell)** → bool
- Places building at cell if legal
- Checks Legal_Placement, power, prerequisites

**HouseClass::Place_Object(building_type, cell)** → bool
- Checks Legal_Placement, power, prerequisites

**HouseClass::Suggest_New_Object()** → StructType
- Determines next buildable object for AI

**HouseClass::Suggested_New_Team()** → TeamTypeClass*
- Determines next team for AI

**HouseClass::Place_Object(building_type, cell)** → bool
- Checks Legal_Placement, power, prerequisites

**HouseClass::Suggest_New_Object()** → StructType
- Determines next buildable for AI

**HouseClass::Suggested_New_Team()** → TeamTypeClass*
- Determines next team for AI

**HouseClass::Suggest_New_Object()** → StructType
- AI build suggestion

**HouseClass::Suggested_New_Team()** → TeamTypeClass*
- AI team suggestion

### Economy

**HouseClass::Available_Money()** → long
- Returns `Credits / 100`

**HouseClass::Spend_Money(amount)** → bool
- Deducts amount, returns false if insufficient

**HouseClass::Refund_Money(amount)** - Adds money
- `Credits += amount * 100`

**HouseClass::Harvested(amount)** - Adds tiberium
- `Credits += amount`

**HouseClass::Refund_Money(amount)** - Adds money
- `Credits += amount * 100`

**HouseClass::Harvested(amount)** - Adds tiberium
- `Credits += amount`

**HouseClass::Adjust_Capacity(amount)** - Tiberium storage
- Adjusts silo capacity

**HouseClass::Power_Fraction()** → int
- Returns 0-256 power fraction
- 0 = no power, 256 = full

**HouseClass::Adjust_Power(adjust)** - `Power += adjust`

**HouseClass::Adjust_Drain(adjust)** - `Drain += adjust`

### Superweapons

**HouseClass::Init_Air_Strike()** - Adds air strike sidebar
**HouseClass::Make_Air_Strike_Available()** - Enables air strike
**HouseClass::Remove_Air_Strike()** - Removes air strike
**HouseClass::Init_Nuke_Bomb()** - Adds nuke to sidebar
**HouseClass::Init_Ion_Cannon()** - Initializes ion cannon countdown
**HouseClass::Remove_Ion_Cannon()** - Disables ion cannon
**HouseClass::Remove_Nuke_Bomb()** - Removes nuke from sidebar
**HouseClass::Has_Nuke_Device()** → bool - Checks if nuke available
**HouseClass::Special_Weapon_AI()** - Fires special weapon AI
**HouseClass::Special_Weapon_AI()** - Fires special weapon AI

### Diplomacy & Allies

**HouseClass::Is_Ally(house)** → bool
- Checks Allies bitmask

**HouseClass::Make_Ally(house)** - Makes ally
- Sets Allies bit, reciprocal

**HouseClass::Make_Enemy(house)** - Makes enemy
- Clears Allies bit, reciprocal

**HouseClass::Is_Ally(house)** → bool
- Checks Allies bitmask

**HouseClass::Make_Ally(house)** - Makes ally
- Sets Allies bit, reciprocal

**HouseClass::Make_Enemy(house)** - Makes enemy
- Clears Allies bit, reciprocal

**HouseClass::Is_Allowed_To_Ally(house)** → bool
- Checks: not self, not already allied, not defeated, MP only, not computer (FIXIT), enemies remain

**HouseClass::Is_Allowed_To_Ally(house)** → bool
- Not self, not already allied, not defeated, MP only, not computer (FIXIT), enemies remain

**HouseClass::Make_Ally(house)** - Sets Allies bit, reciprocal
**HouseClass::Make_Enemy(house)** - Clears Allies bit, reciprocal

**HouseClass::Computer_Paranoid()** - AI paranoid mode
- Non-human houses: ally with AI, enemy with humans

### Production & Economy

**HouseClass::Begin_Production()** - Starts AI production
**HouseClass::Suspend_Production()** - Pauses production
**HouseClass::Abandon_Production(type)** - Abandons production
**HouseClass::Spend_Money(amount)** → bool - Deducts credits
**HouseClass::Refund_Money(amount)** - Adds credits
**HouseClass::Refund_Money(amount)** - Adds credits
**HouseClass::Refund_Amount()** → int - Refund on sell
**HouseClass::Spend_Money(amount)** → bool - Deducts credits

**HouseClass::Refund_Amount()** → int
- `Raw_Cost() * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2` / 2 for humans

**HouseClass::Adjust_Capacity(amount)** - Tiberium storage capacity

### Superweapons

**HouseClass::Init_Air_Strike()** - Adds air strike sidebar
**HouseClass::Make_Air_Strike_Available()** - Enables air strike
**HouseClass::Remove_Air_Strike()** - Removes air strike
**HouseClass::Init_Nuke_Bomb()** - Adds nuke to sidebar
**HouseClass::Init_Ion_Cannon()** - Initializes ion cannon
**HouseClass::Remove_Ion_Cannon()** - Disables ion cannon
**HouseClass::Remove_Nuke_Bomb()** - Removes nuke
**HouseClass::Has_Nuke_Device()** → bool - Checks nuke
**HouseClass::Special_Weapon_AI()** - Fires special weapon

**HouseClass::Special_Weapon_AI()** - Fires special weapon

**HouseClass::Make_Air_Strike_Available()** - Enables airstrike
**HouseClass::Remove_Air_Strike()** - Removes airstrike
**HouseClass::Init_Nuke_Bomb()** - Adds nuke to sidebar
**HouseClass::Init_Ion_Cannon()** - Initializes ion cannon
**HouseClass::Remove_Ion_Cannon()** - Disables ion cannon
**HouseClass::Remove_Nuke_Bomb()** - Removes nuke
**HouseClass::Has_Nuke_Device()** → bool - Checks nuke

### AI & Behavior

**HouseClass::AI()** - House AI processing
- Calls Special_Weapon_AI()
- Processes production queues

**HouseClass::AI()** - House AI
- Calls Special_Weapon_AI()

**HouseClass::Special_Weapon_AI()** - Fires superweapons
- Checks conditions, fires nuke/ion/air strike

**HouseClass::Special_Weapon_AI()** - Fires superweapons

**HouseClass::Do_All_To_Hunt()** - Forces all units to hunt
- Iterates Units/Infantry/Aircraft, non-human, assigns MISSION_HUNT

**HouseClass::Do_All_To_Hunt()** - Forces AI units to hunt

**HouseClass::Suggest_New_Object()** → StructType - AI build suggestion
**HouseClass::Suggested_New_Team()** → TeamTypeClass* - AI team suggestion
**HouseClass::Suggest_New_Object()** → StructType
**HouseClass::Suggested_New_Team()** → TeamTypeClass*

**HouseClass::Attacked()** → bool - Base under attack alert

**HouseClass::Does_Enemy_Building_Exist(type)** → bool - Checks enemy building

**HouseClass::Attacked()** → bool - Base under attack

**HouseClass::Does_Enemy_Building_Exist(type)** → bool - Enemy building check

**HouseClass::Find_Cell_In_Zone(techno, zone)** → CELL - AI placement
**HouseClass::Random_Cell_In_Zone(zone)** → CELL - Random cell in zone

**HouseClass::Find_Cell_In_Zone(techno, zone)** → CELL
**HouseClass::Random_Cell_In_Zone(zone)** → CELL

### Diplomacy

**HouseClass::Is_Ally(house)** → bool - Checks Allies bitmask
**HouseClass::Make_Ally(house)** - Makes ally
**HouseClass::Make_Enemy(house)** - Makes enemy
**HouseClass::Is_Ally(house)** → bool - Ally check
**HouseClass::Make_Ally(house)** - Sets ally bit
**HouseClass::Make_Enemy(house)** - Clears ally bit

**HouseClass::Is_Allowed_To_Ally(house)** → bool
- Not self, not allied, not defeated, MP only, not computer (FIXIT), enemies remain

**HouseClass::Is_Allowed_To_Ally(house)** → bool
- Not self, not allied, not defeated, MP only, not computer, enemies remain

**HouseClass::Make_Ally(house)** - Sets ally bit, reciprocal
**HouseClass::Make_Enemy(house)** - Clears ally bit, reciprocal

**HouseClass::Computer_Paranoid()** - AI bands together vs humans

### Combat & Threat

**HouseClass::Attacked()** → bool - Base under attack
**HouseClass::Does_Enemy_Building_Exist(type)** → bool - Enemy building check
**HouseClass::Adjust_Threat(region, adjust)** - Threat map adjustment
**HouseClass::Adjust_Threat(region, adjust)** - Threat map

### Superweapons

**HouseClass::Init_Air_Strike()** - Adds air strike sidebar
**HouseClass::Make_Air_Strike_Available()** - Enables air strike
**HouseClass::Remove_Air_Strike()** - Removes air strike
**HouseClass::Init_Nuke_Bomb()** - Adds nuke
**HouseClass::Init_Ion_Cannon()** - Initializes ion cannon
**HouseClass::Remove_Ion_Cannon()** - Disables ion cannon
**HouseClass::Remove_Nuke_Bomb()** - Removes nuke
**HouseClass::Has_Nuke_Device()** → bool - Checks nuke
**HouseClass::Special_Weapon_AI()** - Fires superweapon

**HouseClass::Make_Air_Strike_Available()** - Enables air strike
**HouseClass::Remove_Air_Strike()** - Removes air strike
**HouseClass::Init_Nuke_Bomb()** - Adds nuke
**HouseClass::Init_Ion_Cannon()** - Initializes ion cannon
**HouseClass::Remove_Ion_Cannon()** - Disables ion cannon
**HouseClass::Remove_Nuke_Bomb()** - Removes nuke
**HouseClass::Has_Nuke_Device()** → bool - Checks nuke

### Diplomacy & Allies

**HouseClass::Is_Ally(house)** → bool - Checks Allies bitmask
**HouseClass::Make_Ally(house)** - Makes ally
**HouseClass::Make_Enemy(house)** - Makes enemy

**HouseClass::Is_Ally(house)** → bool - Ally check
**HouseClass::Make_Ally(house)** - Sets ally bit
**HouseClass::Make_Enemy(house)** - Clears ally bit

**HouseClass::Is_Allowed_To_Ally(house)** → bool
- Not self, not allied, not defeated, MP only, not computer (FIXIT), enemies remain

**HouseClass::Make_Ally(house)** - Sets ally bit, reciprocal
**HouseClass::Make_Enemy(house)** - Clears ally bit, reciprocal

**HouseClass::Computer_Paranoid()** - AI bands together vs humans

### Construction & Placement

**HouseClass::Place_Object(building_type, cell)** → bool
- Checks Legal_Placement, power, prerequisites

**HouseClass::Sell_Wall(cell)** → bool - Sells wall at cell
**HouseClass::Place_Object(building_type, cell)** → bool
**HouseClass::Sell_Wall(cell)** → bool

### Economy

**HouseClass::Available_Money()** → long
- Returns `Credits / 100`

**HouseClass::Spend_Money(amount)** → bool - Deducts credits
**HouseClass::Refund_Money(amount)** - Adds credits
**HouseClass::Refund_Amount()** → int - Refund on sell
**HouseClass::Harvested(amount)** - Adds tiberium credits

**HouseClass::Available_Money()** → long - Returns `Credits / 100`
**HouseClass::Spend_Money(amount)** → bool - Deducts
**HouseClass::Refund_Money(amount)** - Adds credits
**HouseClass::Refund_Amount()** → int - Refund amount
**HouseClass::Harvested(amount)** - Adds tiberium

**HouseClass::Refund_Amount()** → int
- `Raw_Cost() * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2` / 2 for humans

**HouseClass::Refund_Amount()** → int
- `Raw_Cost() * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2` / 2 for humans

**HouseClass::Spend_Money(amount)** → bool - Deducts
**HouseClass::Refund_Money(amount)** - Adds credits

### Power

**HouseClass::Power_Fraction()** → int
- Returns 0-256 power fraction

**HouseClass::Adjust_Power(adjust)** - `Power += adjust`
**HouseClass::Adjust_Drain(adjust)** - `Drain += adjust`
**HouseClass::Power_Fraction()** → int - Returns 0-256

**HouseClass::Adjust_Power(adjust)** - `Power += adjust`
**HouseClass::Adjust_Drain(adjust)** - `Drain += adjust`

### AI & Behavior

**HouseClass::AI()** - House AI processing
- Calls Special_Weapon_AI()
- Processes production queues

**HouseClass::AI()** - House AI
- Calls Special_Weapon_AI()

**HouseClass::Special_Weapon_AI()** - Fires superweapons
- Checks conditions, fires nuke/ion/air strike

**HouseClass::Special_Weapon_AI()** - Fires superweapons

**HouseClass::Do_All_To_Hunt()** - Forces all units to hunt

**HouseClass::Do_All_To_Hunt()** - Forces AI units to hunt

**HouseClass::Suggest_New_Object()** → StructType - AI build suggestion
**HouseClass::Suggested_New_Team()** → TeamTypeClass* - AI team suggestion

**HouseClass::Attacked()** → bool - Base under attack
**HouseClass::Does_Enemy_Building_Exist(type)** → bool - Enemy building check

**HouseClass::Attacked()** → bool - Base under attack
**HouseClass::Does_Enemy_Building_Exist(type)** → bool - Enemy building

**HouseClass::Find_Cell_In_Zone(techno, zone)** → CELL - AI placement
**HouseClass::Random_Cell_In_Zone(zone)** → CELL - Random cell in zone

**HouseClass::Find_Cell_In_Zone(techno, zone)** → CELL
**HouseClass::Random_Cell_In_Zone(zone)** → CELL

### Multiplayer

**HouseClass::Init_Data()** - Initializes MP color data
**HouseClass::MPlayer_Defeated()** - MP house defeated
**HouseClass::MPlayer_Defeated()** - MP defeated
**HouseClass::Computer_Paranoid()** - AI bands together vs humans
**HouseClass::Is_Allowed_To_Ally(house)** → bool - Ally rules
**HouseClass::Get_Ally_Flags()** → unsigned - Ally bitmask
**HouseClass::Get_Ally_Flags()** → unsigned - Ally bitmask

### Superweapons

**HouseClass::Init_Air_Strike()** - Adds air strike sidebar
**HouseClass::Make_Air_Strike_Available()** - Enables air strike
**HouseClass::Remove_Air_Strike()** - Removes air strike
**HouseClass::Init_Nuke_Bomb()** - Adds nuke to sidebar
**HouseClass::Init_Ion_Cannon()** - Initializes ion cannon
**HouseClass::Remove_Ion_Cannon()** - Disables ion cannon
**HouseClass::Remove_Nuke_Bomb()** - Removes nuke
**HouseClass::Has_Nuke_Device()** → bool - Checks nuke
**HouseClass::Special_Weapon_AI()** - Fires superweapon

### Construction

**HouseClass::Place_Object(building_type, cell)** → bool
- Checks Legal_Placement, power, prerequisites

**HouseClass::Sell_Wall(cell)** → bool - Sells wall at cell
**HouseClass::Place_Object(building_type, cell)** → bool
**HouseClass::Sell_Wall(cell)** → bool

### Construction & Placement

**HouseClass::Place_Object(building_type, cell)** → bool
- Checks Legal_Placement, power, prerequisites

**HouseClass::Sell_Wall(cell)** → bool - Sells wall at cell

**HouseClass::Place_Object(building_type, cell)** → bool
**HouseClass::Sell_Wall(cell)** → bool

### Debug (CHEAT_KEYS)

**HouseClass::Debug_Dump(mono)** - Mono monitor dump

### Allocation

**HouseClass::new** - Allocator
**HouseClass::delete** - Deallocator
**HouseClass::operator HousesType** - Conversion
**HouseClass::new** - Allocator
**HouseClass::delete** - Deallocator
**HouseClass::operator HousesType** - Conversion to HousesType

### Validation

**HouseClass::Validate()** (CHEAT_KEYS)
- Validates index in Houses array

## Inputs, outputs and side effects

**State mutations:**
- Credits, Power, Drain, Allies, BScan, UScan
- Production queues, build limits
- Superweapon states, superweapon availability
- Diplomacy (allies/enemies)
- AI state (paranoid, alerted, IQ)
- Power output/drain
- Build limits, tech level

**External systems affected:**
- All units/buildings owned by house
- Production queues
- Superweapon sidebar
- Diplomacy system
- AI behavior
- Power grid
- Economy

## Calls into / called by

**Called by:**
- Game initialization: One_Time(), Init()
- Player input: production, diplomacy, superweapons
- AI system: AI(), Special_Weapon_AI(), Do_All_To_Hunt()
- Economy: Spend_Money, Refund_Money, Harvested
- Construction: Place_Object, Can_Build
- Diplomacy: Make_Ally, Make_Enemy, Is_Ally
- Superweapons: Special_Weapon_AI, Init_*
- Diplomacy: Make_Ally, Make_Enemy, Is_Ally, Is_Allowed_To_Ally
- Superweapons: Special_Weapon_AI, Init_*
- Construction: Place_Object, Can_Build
- Economy: Spend_Money, Refund_Money, Refund_Amount, Harvested
- Power: Power_Fraction, Adjust_Power, Adjust_Drain

**Calls into:**
- Unit/Building/Infantry/Aircraft arrays: iteration
- TeamClass: Create_One_Of, Destroy_All_Of, reinforcements
- MapClass: Find_Cell_In_Zone, Random_Cell_In_Zone
- Superweapons: AirStrike, NukeStrike, IonCannon
- Sidebar: Add, Flag_To_Redraw
- Units/Buildings/Infantry/Aircraft: iteration
- TeamClass: Create_One_Of, Destroy_All_Of, Do_Reinforcements
- MapClass: Find_Cell_In_Zone, Random_Cell_In_Zone
- Superweapons: AirStrike.Enable, NukeStrike.Enable, IonCannon.Enable
- Sidebar: Add, Flag_To_Redraw
- Units/Buildings/Infantry/Aircraft: iteration
- TeamClass: Create_One_Of, Destroy_All_Of, Do_Reinforcements
- MapClass: Find_Cell_In_Zone, Random_Cell_In_Zone
- Superweapons: AirStrike.Enable, NukeStrike.Enable, IonCannon.Enable
- Sidebar: Add, Flag_To_Redraw

## Related original files
- **HOUSE.H** - Class declarations, HouseControl, HouseTypeClass
- **TEAM.CPP/.H** - TeamClass, TeamTypeClass
- **UNIT.CPP/.H** - UnitClass
- **BUILDING.CPP/.H** - BuildingClass
- **INFANTRY.CPP/.H** - InfantryClass
- **AIRCRAFT.CPP/.H** - AircraftClass
- **MAP.CPP/.H** - MapClass
- **TEAM.CPP/.H** - TeamClass
- **SUPERWPN.CPP** - Superweapons
- **SIDEBAR.CPP** - Sidebar UI
- **SIDEBARGlyphx.H** - MP sidebar
- **INI.H** - INI parsing
- **FUNCTION.H** - Core includes
- **OPTIONS.H** - Difficulty settings
- **RULES.CPP** - Game rules

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/faction/house.rs` - HouseClass core
- `cnc-tiberian-dawn/src/faction/td_house.rs` - TD-specific house
- `cnc-core/src/economy.rs` - Credits, power, resources
- `cnc-core/src/faction/diplomacy.rs` - Allies, enemies
- `cnc-core/src/faction/production.rs` - Production queues
- `cnc-tiberian-dawn/src/faction/superweapon.rs` - Nuke, ion, airstrike
- `cnc-core/src/ai/house_ai.rs` - House AI
- `cnc-core/src/faction/superweapon.rs` - Superweapon logic
- `cnc-core/src/faction/power.rs` - Power system

**Not yet ported:**
- Full AI() with production management
- Superweapon AI (nuke/ion/air strike timing)
- Build time scaling (IQ, power, difficulty)
- Diplomacy system (ally/enemy/neutral)
- Power fraction build time scaling
- Refund calculation with tiberium load
- Multiplayer diplomacy rules (FIXIT_NO_COMP_ALLY)
- AI paranoid mode (Computer_Paranoid)
- Difficulty handicap system (Assign_Handicap)
- Base building AI (Suggest_New_Object, Suggested_New_Team)
- Power fraction calculation (0-256)
- House control settings (Control struct)

## Edge cases and compatibility notes

1. **Credits stored as x100**: Available_Money() returns Credits/100

2. **Difficulty handicap**: MP uses ActLike house type biases; SP uses Rule.Diff directly

3. **Power fraction**: 0-256 scale. 0=0%, 256=100%. Used for build time scaling:
   - 0%: 64x, <50%: 103x, <100%: 171x, 100%+: 256x

3. **Refund formula**: `(Raw_Cost * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2) / 2` for humans

4. **AI difficulty**: Easy/Normal halves cheap building time for AI. IQ scaling: `(time * MaxIQ * 2) / (IQ + MaxIQ)`

4. **Refund**: 50% for humans, includes tiberium load at half rate.

5. **AI build time**: `(Raw_Cost * 256) / inv_scale` where inv_scale depends on power.

5. **Build time power scaling**: 64x (0%), 103x (<50%), 171x (<100%), 256x (100%+)

6. **Superweapon AI**: Checks conditions, fires nuke/ion/air strike

6. **Refund**: 50% for humans, includes half tiberium load value.

7. **Diplomacy**: MP only, not with computers (FIXIT), not if defeated, not if no enemies left

7. **Diplomacy**: MP only, not with computers (FIXIT_NO_COMP_ALLY), not if defeated, not if no enemies left

8. **Credits stored as x100**: Available_Money() = Credits/100

8. **Credits**: Stored as x100. Available_Money() = Credits/100.

9. **Difficulty bias**: MP uses ActLike house; SP uses Rule.Diff directly

9. **Difficulty**: MP uses ActLike biases; SP uses Rule.Diff directly.

10. **Power fraction**: 0-256 scale. 0=0%, 256=100%. Build time scaling.

10. **Power fraction**: 0-256. 0=0%, 256=100%. Build time scaling.

11. **AI IQ scaling**: `(time * MaxIQ * 2) / (IQ + MaxIQ)` for computer houses

11. **AI IQ**: `(time * MaxIQ * 2) / (IQ + MaxIQ)` for computer houses.

12. **Power scaling**: 64x (0%), 103x (<50%), 171x (<100%), 256x (100%+)

12. **Power scaling**: 64x (0%), 103x (<50%), 171x (<100%), 256x (100%+).

13. **Refund**: 50% for humans, includes tiberium load at half rate.

13. **Refund**: 50% for humans, includes half tiberium load value.

14. **AI build time**: Easy/Normal halves cheap building time. IQ scaling applied.

14. **AI build time**: Easy/Normal halves cheap building time. IQ scaling.

15. **Nod airfield fudge**: -25% for non-GOOD units (airfield delivery).

15. **Nod airfield**: -25% build time for non-GOOD units.

16. **MP diplomacy**: Not with computers (FIXIT), not if defeated, enemies must remain

16. **MP diplomacy**: Not with computers (FIXIT_NO_COMP_ALLY), not if defeated, enemies must remain.

17. **AI paranoid**: Computers ally with AI, enemy with humans.

17. **AI paranoid**: Computers ally with AI, enemy with humans.

18. **Superweapon AI**: Checks conditions, fires nuke/ion/air strike.

18. **Superweapon AI**: Checks conditions, fires nuke/ion/air strike.

19. **Refund formula**: `(Raw_Cost * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2) / 2` for humans

19. **Refund**: `(Raw_Cost * CostBias + Tiberium_Load * FULL_LOAD_CREDITS / 2) / 2` for humans.

20. **AI build time scaling**: Multiple compounding factors.

20. **AI build time**: Multiple compounding factors.

21. **Refund includes tiberium**: Half value of carried tiberium.

20. **Refund includes tiberium**: Half value of carried tiberium.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/HOUSE.CPP at f1f0d42
