# PATH/FILE
original/TIBERIANDAWN/INI.CPP

## Purpose
Scenario INI file parsing and multiplayer initialization for Tiberian Dawn. Handles scenario INI loading, multiplayer house assignment, unit/infantry creation, map object placement, and scenario-specific special cases.

## Where it fits in the game
Scenario initialization system - the entry point for loading single-player campaigns and multiplayer skirmishes. Coordinates loading of all game systems (map, houses, units, infantry, buildings, triggers, overlays, etc.) from scenario INI files.

## Classes, structs, enums and important data

### Key structures

#### `utable` - Unit type table
```cpp
struct {
    int MinLevel;           // Minimum build level
    int GDICount;           // Number of GDI unit types
    int NODCount;           // Number of NOD unit types
    UnitType GDIType[];     // GDI unit types
    UnitType NODType[];     // NOD unit types
} utable[NUM_UNIT_CATEGORIES];
```

#### `itable` - Infantry type table
```cpp
struct {
    int GDICount;           // Number of GDI infantry types
    int NODCount;           // Number of NOD infantry types
    InfantryType GDIType[]; // GDI infantry types
    InfantryType NODType[]; // NOD infantry types
} itable[NUM_INFANTRY_CATEGORIES];
```

### Key constants
- `NUM_UNIT_CATEGORIES` - Number of unit categories
- `NUM_INFANTRY_CATEGORIES` - Number of infantry categories
- `UNIT_MAX`, `INFANTRY_MAX` - Max objects
- `MAP_CELL_TOTAL` = 4096 (64x64 grid)
- `MAP_CELL_W` = 64, `MAP_CELL_H` = 64
- `WAYPT_COUNT` = 26 (max waypoints)
- `BARNACLE_STAGE` = 22, `FIRST_SPORE_STAGE` = 30, `FIRST_SPORABLE_LEVEL` = 7

### Global variables
- `BuildLevel` - Current tech level (1-98)
- `MPlayerMax` - Max multiplayer players
- `MPlayerCount` - Current multiplayer players
- `MPlayerBases` - Whether bases are enabled
- `MPlayerUnitCount` - Units per player
- `MPlayerBases` - Bases enabled flag
- `Special.IsCaptureTheFlag` - CTF mode
- `Special.IsJurassic` - Jurassic Park mode
- `Special.IsJurassic` - Jurassic Park mode
- `Special.IsEasy` / `Special.IsDifficult` - Difficulty flags
- `RequiredCD` - CD check requirement
- `ScenarioInit` - Initialization counter
- `ScenarioCRC` - Scenario CRC checksum
- `CrateCount`, `CrateMaker`, `CrateTimer` - Crate system
- `MPlayerBases` - Multiplayer bases enabled
- `MPlayerUnitCount` - Units per player

### Scenario file naming
```
SC<player><scenario><dir><var>.INI
```
- `player`: G(GDI), B(NOD), J(Japan), M(Multi)
- `scenario`: 2-digit number
- `dir`: E (East) or W (West)
- `var`: A, B, C, D, or X (error)

## Functions and methods

### Read_Scenario_Ini(root, fresh) → bool
**Main scenario loader** - loads complete scenario from INI file.

**Process:**
1. Increments `ScenarioInit`
2. Allocates staging buffer (`_ShapeBuffer`)
3. If `fresh`: `Clear_Scenario()`
4. **CD check**: Verifies required CD for scenario
5. **Opens INI file**: `SCENARIO_NAME.INI`
6. **Reads file** into `_ShapeBuffer`
7. **CRC calculation** on buffer
8. **Reads movie names** from `[Basic]` section:
   - `Intro`, `Brief`, `Win`, `Win2`, `Win3`, `Win4`, `Lose`, `Action`
9. **BuildLevel** determination:
   - SP: scenario number or INI `BuildLevel`
   - Jurassic: level 98
   - N64 special cases: level 15
10. **TransitTheme** from INI
11. **TeamTypeClass::Read_INI()** - team type definitions
12. **HouseClass::Read_INI()** - house definitions
13. **TriggerClass::Read_INI()** - triggers
14. **Map.Read_INI()** - map dimensions, theater
15. **Player assignment:**
    - SP: reads `Player` from INI, sets `PlayerPtr`
    - MP: `GlyphX_Assign_Houses()`
15. **Carryover money**: Calculates carryover credits
16. **Difficulty handicap**: Easy/Hard assignment
16. **Map binary load**: `Map.Read_Binary()` or `TemplateClass::Read_INI()`
17. **TerrainClass::Read_INI()** - 3D terrain objects
18. **UnitClass::Read_INI()** - units
19. **AircraftClass::Read_INI()** - aircraft
20. **InfantryClass::Read_INI()** - infantry
21. **BuildingClass::Read_INI()** - buildings
22. **Base.Read_INI()** - AI base
22. **OverlayClass::Read_INI()** - overlays
22. **SmudgeClass::Read_INI()** - smudges
22. **Briefing text** - builds `BriefingText` from `[Briefing]` section
23. **Map.Overpass()** - terrain smoothing
23. **Scenario-specific fixes** (hardcoded):
   - NOD7A: Cell 2795 → LAND_ROCK
   - NOD09A: Radar destroyed → delete airstrike trigger
   - NOD10B: Cell 2015 → LAND_ROCK
   - NOD13B: AI production trigger, fix repeating airstrike
   - NOD13C: Delete airstrike trigger when radar destroyed

### Multiplayer initialization (`Assign_Houses`, `Create_Units`)

**`Assign_Houses()` / `GlyphX_Assign_Houses()`**
- Assigns houses to multiplayer slots
- Uses GlyphX start locations (override) or random waypoints
- **Bases ON**: Human gets MCV + flag; AI gets MHQ (CTF) or MCV
- **Bases OFF**: All get Mobile HQ (CTF) or MCV
- Sets `MaxUnit = MPlayerUnitCount * scaleval`

**`Create_Units()`** - Creates starting units/infantry
- Computes `tot_units = (MPlayerUnitCount * 2) / 3`
- Distributes across `NUM_UNIT_CATEGORIES` categories
- Infantry: `tot_infantry = MPlayerUnitCount - tot_units`
- Scales by `scaleval` (1 for human/bases-off, 3 for AI bases-on)
- **Unit creation:** Loops through `utable` categories
- **Infantry creation:** Uses `itable` for GDI/NOD types
- **Placement:** `Scan_Place_Object()` with `Clip_Scatter()` for dispersion

### Unit/Infantry creation helpers

**`Reserve_Unit()` / `Reserve_Infantry()`** - Increments allocation counters

**`Scan_Place_Object(obj, centerpt)` → bool**
- Tries to place object at `centerpt`
- Spirals outward with `Clip_Move()` and `Clip_Scatter()`
- Handles infantry stacking in same cell

**`Clip_Move(cell, facing, dist)`** - Moves cell in direction, clips to map bounds
**`Clip_Scatter(cell, maxdist)`** - Random scatter within distance
**`Furthest_Cell()` / `Sort_Cells()`** - Waypoint sorting

### Scenario INI I/O

**`Set_Scenario_Name()`** - Generates filename
```
SC<player><scenario><dir><var>.INI
```
- player: G(GDI), B(NOD), J(Japan), M(Multi)
- scenario: 2-digit
- dir: E/East, W/West
- var: A/B/C/D...

**`Read_Scenario_Ini(root, fresh)` → bool**
1. Increments `ScenarioInit`
2. Allocates `_ShapeBuffer`
3. If `fresh`: `Clear_Scenario()`
3. CD check for scenario
4. Opens `SCENARIO.INI`
5. Reads file into `_ShapeBuffer`
3. CRC calculation
4. Reads movie names: Intro, Brief, Win/Win2/3/4, Lose, Action
4. BuildLevel from INI or scenario
5. TransitTheme from INI
6. Calls in order:
   - `TeamTypeClass::Read_INI()`
   - `HouseClass::Read_INI()`
   - `TriggerClass::Read_INI()`
   - `Map.Read_INI()`
   - Player assignment (SP: INI Player; MP: `GlyphX_Assign_Houses()`)
   - Carryover money calculation
   - Difficulty handicap (Easy/Hard)
   - Map binary load (`Map.Read_Binary()` or `TemplateClass::Read_INI()`)
   - TerrainClass::Read_INI()
   - UnitClass/InfantryClass/AircraftClass/BuildingClass/OverlayClass/SmudgeClass Read_INI
   - Base.Read_INI()
   - Briefing text from `[Briefing]` section
   - `Map.Overpass()` - terrain smoothing
   - **Scenario-specific hardcoded fixes** (e.g., NOD09A radar→trigger)

**`Write_Scenario_Ini()`** - Writes scenario to INI (editor)

### Multiplayer house assignment

**`Assign_Houses()` / `GlyphX_Assign_Houses()`**
- Assigns houses to `HOUSE_MULTI1` through `HOUSE_MULTI1 + MPlayerMax - 1`
- **Bases ON**: Human→MCV+flag, AI→MCV/MHQ
- **Bases OFF**: All get MHQ (CTF) or MCV
- `MaxUnit = MPlayerUnitCount * scaleval`
- `GlyphX_Assign_Houses()` uses GlyphX start locations

**`Assign_Houses()` / `GlyphX_Assign_Houses()`**
- Bases ON: Human→MCV+flag, AI→MCV/MHQ
- Bases OFF: All get MHQ (CTF)
- `MaxUnit = MPlayerUnitCount * scaleval`

### Utility functions

**`Clip_Move(cell, facing, dist)`** → CELL
- Moves cell in direction, clips to map bounds

**`Clip_Scatter(cell, maxdist)`** → CELL
- Random scatter within distance, clips to map

**`Clip_Scatter(cell, maxdist)`** - Random scatter within distance, clips to map
**`Clip_Move(cell, facing, dist)`** - Moves cell in direction, clips to map

**`Furthest_Cell(cells, numcells, tcells, numtcells)`** → int
- Finds cell furthest from group

**`Sort_Cells(cells, numcells, outcells)`** - Sorts by distance from first

**`Clip_Scatter(cell, maxdist)`** - Random scatter within distance
**`Clip_Move(cell, facing, dist)`** - Move in direction, clip to bounds

**`Furthest_Cell(cells, numcells, tcells, numtcells)`** - Finds furthest cell
**`Sort_Cells(cells, numcells, outcells)`** - Distance-based sort

### Multiplayer house assignment (`Assign_Houses`, `Create_Units`)

**`GlyphX_Assign_Houses()`** - Uses GlyphX start locations
**`Assign_Houses()`** - Random waypoint selection
**`Create_Units()`** - Creates starting forces:
- `tot_units = (MPlayerUnitCount * 2) / 3`
- `tot_infantry = MPlayerUnitCount - tot_units`
- Distributes across `NUM_UNIT_CATEGORIES` / `NUM_INFANTRY_CATEGORIES`
- Scale: 1 (human/bases-off) or 3 (AI bases-on)
- GDI/NOD unit types from `utable` / `itable`
- `Scan_Place_Object` with `Clip_Scatter` for dispersion

### Scenario INI format

**Root section `[Basic]`:**
- `Intro`, `Brief`, `Win`/`Win2`/`Win3`/`Win4`, `Lose`, `Action` - movie files
- `BuildLevel` - tech level
- `Theme` - transit theme
- `Player` - GDI/NOD (SP)
- `Credits`, `CarryOverMoney`, `CarryOverCap`
- `IQ`, `Edge`, `PlayerControl`
- `Allies` - bitmask

**TeamType section:**
- Team type definitions (read by `TeamTypeClass::Read_INI`)

**House section:**
- Per-house control settings (TechLevel, MaxBuilding, MaxUnit, etc.)

**Trigger section:**
- `Triggername = Eventname, Actionname, Data, Housename, TeamName, IsPersistant`

**Briefing section:**
- `1=text`, `2=text`... sequential paragraphs

**Map section:**
- Dimensions, theater, cell data

### Scenario-specific hardcoded fixes (in `Read_Scenario_Ini`)

| Scenario | Fix |
|----------|-----|
| `scb07ea` (NOD7A) | Cell 2795 → `LAND_ROCK` |
| `scb09ea` (NOD09A) | Radar destroyed → delete `dely` trigger |
| `scb10eb` (NOD10B) | Cell 2015 → `LAND_ROCK` |
| `scb13eb` (NOD13B) | Add `EVENT_PLAYER_ENTERED` → `ACTION_BEGIN_PRODUCTION` triggers at cells 276,340,404,468; fix `xxxx` trigger |
| `scb13ec` (NOD13C) | Radar destroyed → delete `dely` trigger |

## Inputs, outputs and side effects

**State mutations:**
- Global scenario state (`Scenario`, `BuildLevel`, `PlayerPtr`, etc.)
- All game object arrays populated
- Map cells initialized
- House states initialized
- Trigger system initialized

**External systems affected:**
- All game systems initialized from INI
- Map, objects, houses, triggers, teams, base, overlays, smudges
- CD verification for scenario

## Calls into / called by

**Called by:**
- Game initialization: `Read_Scenario_Ini()`
- Scenario editor: `Write_Scenario_Ini()`

**Calls into:**
- `TeamTypeClass::Read_INI()`
- `HouseClass::Read_INI()`
- `TriggerClass::Read_INI()`
- `Map.Read_INI()`
- `TerrainClass::Read_INI()`
- `UnitClass::Read_INI()`
- `AircraftClass::Read_INI()`
- `InfantryClass::Read_INI()`
- `BuildingClass::Read_INI()`
- `Base.Read_INI()`
- `OverlayClass::Read_INI()`
- `SmudgeClass::Read_INI()`
- `Base.Read_INI()`
- `TemplateClass::Read_INI()` (fallback)
- `Map.Read_Binary()` / `Map.Read_INI()`
- `Map.Overpass()`
- Scenario-specific hardcoded fixes

## Related original files
- **INI.H** - Function declarations
- **HOUSE.CPP/.H** - `HouseClass::Read_INI`
- **TEAM.CPP/.H** - `TeamTypeClass::Read_INI`
- **TRIGGER.CPP/.H** - `TriggerClass::Read_INI`
- **MAP.CPP/.H** - `Map.Read_INI`/`Read_Binary`
- **TERRAIN.CPP/.H** - `TerrainClass::Read_INI`
- **UNIT.CPP/.H** - `UnitClass::Read_INI`
- **AIRCRAFT.CPP/.H** - `AircraftClass::Read_INI`
- **INFANTRY.CPP/.H** - `InfantryClass::Read_INI`
- **BUILDING.CPP/.H** - `BuildingClass::Read_INI`
- **OVERLAY.CPP/.H** - `OverlayClass::Read_INI`
- **SMUDGE.CPP/.H** - `SmudgeClass::Read_INI`
- **BASE.CPP/.H** - `Base.Read_INI`
- **TEMPLATE.CPP/.H** - `TemplateClass::Read_INI`
- **TRIGGER.CPP/.H** - `TriggerClass::Read_INI`
- **TEAMTYPE.H** - `TeamTypeClass`
- **FUNCTION.H** - Core includes, `WWGetPrivateProfileString/Int`

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-tiberian-dawn/src/scenario/ini.rs` - INI parsing
- `cnc-tiberian-dawn/src/scenario/loader.rs` - Scenario loading
- `cnc-tiberian-dawn/src/scenario/multiplayer.rs` - MP initialization
- `cnc-core/src/scenario/ini.rs` - INI parsing core
- `cnc-core/src/scenario/loader.rs` - Scenario loading

**Not yet ported:**
- INI parsing with `WWGetPrivateProfileString`
- Scenario-specific hardcoded fixes
- Multiplayer house assignment (`GlyphX_Assign_Houses`)
- Unit/infantry creation with `Scan_Place_Object`/`Clip_Scatter`
- Briefing text assembly
- Scenario-specific hardcoded fixes
- CRC calculation for scenario validation
- CD verification logic

## Edge cases and compatibility notes

1. **Scenario naming**: `SC<player><scenario><dir><var>.INI`
   - Player: G(GDI), B(NOD), J(Japan), M(Multi)
   - Dir: E/East, W/West, or random
   - Var: A-D, or random if none available

2. **BuildLevel**: SP = scenario# or INI override; MP = 98 (Jurassic) or INI

2. **BuildLevel**: SP = scenario or INI; MP = 98 (Jurassic) or INI

3. **CD check**: Campaign missions require CD 2; bonus = CD -1; GDI=0, NOD=1

3. **CD check**: SP campaigns need CD 2; bonus = -1; GDI=0, NOD=1

4. **CRC**: Calculated on INI buffer for save validation

4. **CRC**: Calculated on INI buffer

5. **Multiplayer house assignment**: GlyphX start locations override random waypoints

5. **MP houses**: GlyphX start locations override random waypoints

6. **Unit scaling**: AI bases-on = 3x units; human/bases-off = 1x

6. **Unit scaling**: AI bases-on = 3x; human/bases-off = 1x

7. **Unit placement**: `Scan_Place_Object` + `Clip_Scatter` for dispersion

7. **Placement**: `Scan_Place_Object` + `Clip_Scatter` for dispersion

8. **Hardcoded scenario fixes**: 5 specific scenario patches

8. **Scenario patches**: 5 hardcoded fixes (NOD7A, 09A, 10B, 13B, 13C)

9. **CTF mode**: Flag attachment to MCV/MHQ

9. **CTF mode**: Flag attachment to MCV/MHQ

10. **BuildLevel 98**: Jurassic Park mode unlocks all units

10. **Jurassic**: BuildLevel 98 unlocks all

11. **Carryover money**: SP only, scaled by percentage, capped

11. **Carryover**: SP only, percentage with cap

12. **Difficulty handicap**: Easy/Normal/Hard adjusts biases

12. **Handicap**: Easy/Normal/Hard → bias multipliers

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/INI.CPP at f1f0d42
