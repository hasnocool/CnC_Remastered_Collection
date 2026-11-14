# PATH/FILE
original/TIBERIANDAWN/TEAMTYPE.CPP

## Purpose
AI team type definitions and behaviors for Tiberian Dawn. Defines AI personalities for computer-controlled players, including mission priorities, recruitment rules, team characteristics, and INI format parsing. Each team type has a name, associated house, mission list, and behavioral flags that determine AI gameplay style.

## Where it fits in the game
AI system - computer players use `TeamTypeClass` instances to determine their strategy, unit production priorities, and mission objectives. Parsed from scenario INI files via `Read_Scenario_Ini()`. Integrated with `INIT.CPP` scenario loading, `LOGIC.CPP` per-frame AI processing, and `INI.CPP` scenario loader. Global `TeamTypes` array manages all team type definitions.

## Classes, structs, enums and important data

### TeamTypeClass
**AI team type definition** - Complete personality profile for a computer player team.

**Key data members:**
| Field | Type | Purpose |
|-------|------|---------|
| `IsPrebuilt` (bool) | Whether team is pre-built (scenario-defined) |
| `IsReinforcable` (bool) | Whether team receives reinforcements |
| `IsRoundAbout` (bool) | AI behavior flag |
| `IsLearning` (bool) | AI learning enabled |
| `IsSuicide` (bool) | Aggressive AI behavior |
| `IsAutocreate` (bool) | Auto-create units |
| `IsTransient` (bool) | Temporary team (disappears) |
| `IsMercenary` (bool) | Mercenary team (sells to highest bidder) |
| `RecruitPriority` (int) | Unit recruitment priority (0-10) |
| `MaxAllowed` (int) | Maximum units allowed |
| `Fear` (int) | Fear factor (0-255, affects aggression) |
| `InitNum` (int) | Initial unit count |
| `House` (HousesType) | Associated player house |
| `MissionCount` (int) | Number of mission goals |
| `IniName[50]` (char) | INI registration name |
| `Class[MAX_TEAM_CLASSCOUNT]` (Class pointers) | Associated class types |
| `DesiredNum[MAX_TEAM_CLASSCOUNT]` (int) | Desired unit counts per class |

### TeamTypeClass::TMissions[TMISSION_COUNT]
**Static mission string array** (10 total):
| Index | Mission Name |
|-------|-------------|
| 0 | "Attack Base" |
| 1 | "Attack Units" |
| 2 | "Attack Civil." (Civilian) |
| 3 | "Rampage" |
| 4 | "Defend Base" |
| 5 | "Harvest" |
| 6 | "Move" |
| 7 | "Move to Cell" |
| 8 | "Retreat" |
| 9 | "Guard" |

### TeamTypeClass::Validate() → int (CHEAT_KEYS only)
**Validate team type pointer:**
1. Gets team index via `TeamTypes.ID(this)`
2. Checks `0 <= num < TEAMTYPE_MAX`
3. If invalid: `Validate_Error("TEAMTYPE")` → returns 0
4. If valid: returns 1
5. Non-CHEAT_KEYS: macro expands to empty (always returns 1)

### TeamTypeClass::TeamTypeClass()
**Constructor:**
1. Default initialization:
   - `IsPrebuilt = true`
   - `IsReinforcable = true`
   - `IsRoundAbout = false`
   - `IsLearning = false`
   - `IsSuicide = false`
   - `IsAutocreate = false`
   - `IsTransient = false`
   - `IsMercenary = false`
   - `RecruitPriority = 7`
   - `MaxAllowed = 0`
   - `Fear = 0`
   - `InitNum = 0`
   - `House = HOUSE_NONE`
   - `MissionCount = 0`
   - `IniName[0] = '\0'`
   - All `Class[i] = NULL`
   - All `DesiredNum[i] = 0`

### TeamTypeClass::Init() → void
**Pre-scenario initialization:**
1. `TeamTypes.Free_All()` - clear all existing team types
2. Create default team type instance
3. Extract VTable pointer from constructed instance
4. Delete the temporary instance (VTable extraction side effect)

### TeamTypeClass::Read_INI(buffer) → void
**Parse INI data for team type:**
**INI format:**
```
TeamName = Housename,Roundabout,Learning,Suicide,Spy,
       RecruitPriority,MaxAllowed,InitNum,Fear,
       ClassCount,Class:Num,Class:Num,...,      
       MissionCount,Mission:Arg,Mission:Arg,Mission:Arg,...
```

**Parsing logic:**
1. `tbuffer` accumulates team names from buffer
2. `len` = data length in buffer
3. `buf[500]` = INI entry buffer for parsing
4. Parses comma-separated values:
   - House name
   - Roundabout, Learning, Suicide, Spy flags
   - RecruitPriority, MaxAllowed, InitNum, Fear
   - ClassCount followed by Class:Num pairs
   - MissionCount followed by Mission:Arg pairs

### TeamTypeClass::Write_INI() → void
**Write team type to INI format** (reverse of Read_INI).

### TeamTypeClass::As_Pointer(name) → TeamTypeClass*
**Get pointer by name:**
- Searches `TeamTypes` array for matching `IniName`
- Returns pointer or NULL if not found

### TeamTypeClass::Remove() → bool
**Remove team type:**
- Checks if removal is allowed (editor constraint: can't remove if no classes)
- Frees team type resources

### TeamTypeClass::Mission_From_Name(name) → mission enum
**Get mission enum from string name.**

### TeamTypeClass::Name_From_Mission(mission) → name string
**Get mission string from enum.**

## Functions and methods

### TeamTypeClass::TeamTypeClass() → constructor
Standard constructor with default values as listed above.

### TeamTypeClass::Init() → void
Pre-scenario setup: clears team types, creates default instance, extracts VTable.

### TeamTypeClass::Read_INI(char *buffer) → void
Parse INI formatted team definition. Key parsing points:
- Header: `TeamName =`
- Values separated by commas
- 13+ parameters: house, flags, priority, limits, init count, fear
- ClassCount + Class:Num pairs (variable number)
- MissionCount + Mission:Arg pairs (variable number)
- Uses `buf[500]` as working buffer
- `tbuffer` for team name accumulation

### TeamTypeClass::Write_INI() → void
Write back in reverse INI format.

### TeamTypeClass::As_Pointer(const char *name) → TeamTypeClass*
Search `TeamTypes` global array for team with matching `IniName`.

### TeamTypeClass::Remove() → bool
Remove team type from global array. Constraints:
- Can't remove if editor needs empty teams (check `ClassCount == 0`)
- One-time cleanup

### TeamTypeClass::Mission_From_Name(const char *name) → MissionTypeEnum
Convert mission string ("Attack Base", "Defend Base", etc.) to enum.

### TeamTypeClass::Name_From_Mission(MissionTypeEnum mission) → char*
Convert enum back to string.

## Inputs, outputs and side effects

**State mutations:**
- `TeamTypes` global array (add/remove/modify entries)
- `IniName[50]` per team type
- `Class[]` and `DesiredNum[]` per team
- `MissionCount` and mission enum state
- `Fear`, `RecruitPriority`, `MaxAllowed` numeric values

**External systems affected:**
- `INIT.CPP` → `TeamTypes.Init()` called at scenario load
- `INI.CPP` → `Read_Scenario_Ini()` parses team definitions
- `LOGIC.CPP` → per-frame AI uses `TeamTypeClass` mission priorities
- `REINF.CPP` → reinforcement scheduling based on `IsReinforcable`, `RecruitPriority`
- Multiplayer: team types must synchronize across clients

## Calls into / called by

**Called by:**
- `INIT.CPP` → `TeamTypes.Init()` at scenario start
- `INI.CPP` → `Read_Scenario_Ini()` team type parsing
- `LOGIC.CPP` → AI mission selection via `Mission_From_Name()`
- `REINF.CPP` → reinforcement rates from `RecruitPriority`, `MaxAllowed`
- `OPTIONS.CPP` → team type settings (linked to `Special.CPP`)

**Calls into:**
- `TeamTypes.Free_All()` → clear all team types
- `Validate_Error()` → error reporting (CHEAT_KEYS only)
- String comparison for `As_Pointer` name matching
- INI parsing utilities (strtok, atoi, etc.)

## Related original files
- **TeamType.H** - Class declarations
- **INIT.CPP** → `TeamTypes.Init()` call
- **INI.CPP** → `Read_Scenario_Ini()` team parsing
- **REINF.CPP** → reinforcement using `RecruitPriority`, `MaxAllowed`
- **AI** system → mission selection and behavior
- **Missions** string table (TMissions)

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/ai/team_type.rs` - TeamTypeClass and array
- `cnc-tiberian-dawn/src/ai/missions.rs` - Mission enum and strings
- `cnc-tiberian-dawn/src/config/ini_team.rs` - INI parsing for teams

**Not yet ported:**
- Windows AI framework integration
- INI format parsing (13+ comma-separated parameters)
- Class:Num pair handling (variable count)
- Mission:Arg pair handling
- `IsMercenary` buy/sell logic
- `Fear` aggression calculations
- `IsRoundAbout` AI pathing behavior
- `IsLearning` AI learning state
- `IsTransient` team lifetime management
- Multiplayer team type synchronization
- VTable extraction pattern

## Edge cases and compatibility notes

1. **Mercenary teams**: `IsMercenary=true` enables selling to highest bidder (multiplayer gold sharing)
2. **Prebuilt vs dynamic**: `IsPrebuilt=true` means team defined in scenario INI; false = created during game
3. **Editor constraint**: Can't remove team type if `ClassCount == 0` (editor needs empty teams)
4. **Fear factor**: `Fear 0-255` affects AI aggression; higher = more aggressive
5. **RecruitPriority**: `7` is default; lower = prefer other types first
6. **MaxAllowed**: `0` = unlimited; set limit on unit counts
7. **Class pairs**: `ClassCount` followed by `Class:Num` pairs define preferred unit production
8. **Mission goals**: `MissionCount` + `Mission:Arg` defines AI objectives (attack base, defend, etc.)
9. **VTable extraction**: `Init()` extracts VTable via constructor/destructor pattern
10. **Mission string table**: `TMissions[10]` defines the 10 mission strings used across all teams

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TEAMTYPE.CPP at f1f0d42
