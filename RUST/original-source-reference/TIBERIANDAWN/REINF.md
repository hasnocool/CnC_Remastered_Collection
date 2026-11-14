# PATH/FILE
original/TIBERIANDAWN/REINF.CPP

## Purpose
Reinforcement system for Tiberian Dawn. Handles creation and delivery of reinforcement teams (ground, air, naval) with transport logic, multiplayer announcements, and special reinforcement creation for ad-hoc scenarios.

## Where it fits in the game
Reinforcement system - handles dynamic unit delivery during gameplay via triggers, AI production, air strikes, and special reinforcements (harvester replacement, airfield orders). Coordinates transport, unloading, and mission assignment.

## Classes, structs, enums and important data

### Reinforcement source types
| Source | Description |
|--------|-------------|
| `SOURCE_NONE` | Invalid |
| `SOURCE_NORTH` | North edge |
| `SOURCE_SOUTH` | South edge |
| `SOURCE_EAST` | East edge |
| `SOURCE_WEST` | West edge |
| `SOURCE_AIR` | Air delivery |
| `SOURCE_SHIPPING` | Naval delivery |
| `SOURCE_BEACH` | Beach landing |
| `SOURCE_OCEAN` | Ocean delivery |

### Transport types
| Type | Description |
|------|-------------|
| `airtransport` | Aircraft delivery |
| `watertransport` | Naval/hover transport |
| `onlytransport` | Transport-only team |
| `hastransport` | Team includes transport |
| `onlytransport` | Transport-only team |

### Reinforcement sources
| Source | Description |
|--------|-------------|
| `SOURCE_AIR` | Air drop |
| `SOURCE_SHIPPING` | Naval delivery |
| `SOURCE_BEACH` | Beach landing |
| `SOURCE_OCEAN` | Ocean delivery |
| `SOURCE_NORTH/SOUTH/EAST/WEST` | Map edge entry |

### TeamTypeClass reinforcement flags
- `IsReinforcable` - Can receive reinforcements
- `IsTransient` - Delete when empty
- `IsSuicide` - Disband after mission
- `RecruitPriority` - AI build priority

## Functions and methods

### Do_Reinforcements(teamtype) → bool
**Core reinforcement creation** - Creates and places a reinforcement team.

**Process:**
1. **Validation**: Checks teamtype and MissionCount
2. **Team creation**: Creates TeamClass if MissionCount > 0
3. **Transport detection**:
   - Scans team composition for transporters/aircraft
   - Determines `airtransport`, `watertransport`, `onlytransport`
   - Sets `hastransport`, `airtransport`, `watertransport`
4. **Source determination**:
   - Air → `SOURCE_AIR`
   - Water → `SOURCE_BEACH`
   - Gunboat → `SOURCE_SHIPPING`
   - Default → House edge
5. **Object creation**:
   - Iterates `teamtype->Class[]` and `DesiredNum[]`
   - Calls `Create_One_Of()` for each unit
   - Attaches to team or transport
   - Loaner flags for transporters/A-10s
6. **Placement by source**:
   - `SOURCE_SHIPPING`: Gunboat entry from west
   - `SOURCE_NORTH/SOUTH/EAST/WEST`: Edge entry with facing
   - `SOURCE_AIR`: Air drop with docking bay logic
   - `SOURCE_OCEAN/BEACH`: Beach landing from south
7. **Announcement**: `Speak(VOX_REINFORCEMENTS)` for player's house

### Create_Special_Reinforcement(house, type, another, mission, argument) → bool
**Ad-hoc reinforcement** for dynamic scenarios (harvester replacement, airfield orders).

**Logic:**
1. Creates temporary `TeamTypeClass` with:
   - Single unit type (`ClassCount=1`, `DesiredNum[0]=1`)
   - `IsReinforcable=false`, `IsTransient=true`
2. **Transport logic**:
   - Ground units on water edge → adds hover lander
   - South edge only for hover lander
   - No transport → defaults to `TMISSION_MOVECELL` with air entry
4. **Mission assignment**:
   - Default: `TMISSION_MOVECELL` with calculated cell
   - With transport: `TMISSION_UNLOAD` + waypoint
5. Calls `Do_Reinforcements()`

### Create_Air_Reinforcement(house, air, number, mission, tarcom, navcom) → int
**Air strike reinforcement** - creates aircraft squad for immediate attack.

**Process:**
1. Gets `AircraftTypeClass` for air type
2. Loops `number` times:
   - Creates aircraft via `type->Create_One_Of(house)`
   - Sets `IsALoaner = true` (always loaners)
   - Calculates entry cell from house edge
   - **Cargo planes**: Finds docking bay (airstrip), calculates spawn delay
   - Places at edge, assigns mission/target/navcom
   - `Commence()` starts AI

**Returns:** Number of aircraft successfully created

## Inputs, outputs and side effects

**State mutations:**
- Creates TeamClass and TechnoClass instances
- Modifies map object arrays
- Updates house unit counts
- Triggers VOX_REINFORCEMENTS announcement

**External systems affected:**
- Team system (TeamClass)
- Unit/Infantry/Aircraft creation
- Map placement (Unlimbo)
- Voice announcements (VOX_REINFORCEMENTS)
- AI team tracking

## Calls into / called by

**Called by:**
- Trigger actions: `ACTION_REINFORCEMENTS`
- House AI: Special_Weapon_AI (air strikes)
- Harvester logic: `Create_Special_Reinforcement()`
- Airfield orders: `Create_Air_Reinforcement()`

**Calls into:**
- `TeamClass` constructor
- `TechnoTypeClass::Create_One_Of()`
- `UnitClass::Unlimbo()`
- `FootClass::Assign_Mission/Destination/Target()`
- `Map.Calculated_Cell()`
- `BuildingClass::Docking_Coord()`
- `Speak()` for VOX_REINFORCEMENTS
- `MixFileClass::Retrieve()` for audio

## Related original files
- **REINF.H** - Function declarations
- **TEAM.CPP/.H** - TeamClass, TeamTypeClass
- **UNIT.CPP/.H** - UnitClass creation
- **AIRCRAFT.CPP/.H** - AircraftClass
- **HOUSE.CPP/.H** - HouseClass (edges, superweapons)
- **MAP.CPP/.H** - MapClass (Calculated_Cell)
- **BUILDING.CPP/.H** - BuildingClass (docking)
- **TRIGGER.CPP** - ACTION_REINFORCEMENTS
- **VOX.H** - VOX_REINFORCEMENTS
- **SPEAK.CPP** - Speak() function

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/reinforcement.rs` - Reinforcement logic
- `cnc-tiberian-dawn/src/ai/td_reinforcement.rs` - TD-specific
- `cnc-core/src/ai/transport.rs` - Transport logic
- `cnc-tiberian-dawn/src/ai/td_air_reinforcement.rs` - Air strikes

**Not yet ported:**
- Multi-source reinforcement placement (air/ground/naval)
- Transport loaner system (IsALoaner)
- Air strike cargo plane delay calculation
- Special reinforcement hover lander fallback
- Reinforcement announcement (VOX_REINFORCEMENTS)
- Modern balance cargo plane delay (ST 2/28/2020)

## Edge cases and compatibility notes

1. **Transport loaners**: Transports carrying cargo marked `IsALoaner=true`
2. **Cargo planes**: Find docking bay, calculate spawn delay based on speed/distance
3. **Hover lander fallback**: Ground units on water edge → hover transport (south edge only)
4. **Gunboat special**: `SOURCE_SHIPPING` entry from west edge
5. **Air strike cargo**: Delay calculated from distance/speed
5. **Cargo delay**: `(spawn_x - border_x) / speed` for modern balance
6. **Gunboat**: Always `SOURCE_SHIPPING` from west edge
7. **Transport loaners**: Non-transport cargo → `IsALoaner=true`
8. **A-10**: Always loaner
9. **Reinforcement announcement**: VOX_REINFORCEMENTS for player's house
10. **Hover lander**: Only from south edge, requires clear beach cells
11. **Modern balance**: Cargo plane delay calculation with speed/distance
12. **Loaner flag**: Transports with cargo = loaner; A-10 always loaner
13. **Reinforcement source**: Determined by transport type (air/water/edge)
14. **Air transport**: Cargo planes with docking bay logic
15. **Naval**: Gunboat = shipping; hover = beach
16. **Edge entry**: North/South/East/West with facing direction
17. **Failure cleanup**: Deletes team/objects on placement failure
18. **MP announcement**: Only for player's house

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/REINF.CPP at f1f0d42
