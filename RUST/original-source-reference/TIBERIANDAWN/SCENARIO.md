# SCENARIO.CPP

## Purpose
This module handles scenario reading, writing, and lifecycle management for Command & Conquer: Tiberian Dawn. It orchestrates loading scenario INI data, clearing previous scenario state, managing win/lose/restart conditions, and coordinating movie playback, difficulty settings, and mission progression.

## Where it fits in the game
SCENARIO.CPP sits at the top level of game flow control, acting as the bridge between:
- **Main game loop** (`MAIN.CPP`) — calls `Do_Restart`, `Do_Win`, `Do_Lose`
- **INI parsing** (`LOAD.CPP` / `Read_Scenario_Ini`) — loads scenario data from disk
- **Game systems** (Map, House, Unit, Building, Team, Trigger, etc.) — initializes/clears all subsystems via their `Init()` methods
- **Movie/Briefing system** (`MOVIE.CPP`) — plays intro, briefing, action, win, lose movies
- **Menu/UI system** (`NEWMENU` / `CCMessageBox`) — handles mission restatement and dialog prompts
- **Score/Presentation** (`SCORE.CPP`) — shows score screens between missions

It is the central coordinator for single-player campaign progression and multiplayer match handling.

## Classes, structs, enums and important data

### Global Variables Referenced
| Variable | Type | Purpose |
|----------|------|---------|
| `Scenario` | `int` | Current scenario number (1-15 campaign, 20+ covert ops, 60+ Jurassic Park, 72 secret) |
| `ScenarioName` | `char[]` | Filename root for current scenario (e.g., "SCG01EA") |
| `ScenarioInit` | `int` | Reentrancy guard counter (incremented during init/clear) |
| `GameToPlay` | `int` | Game mode: `GAME_NORMAL` (campaign), `GAME_NETWORK`, `GAME_LAN`, etc. |
| `Whom` | `int` | Player house: `HOUSE_GOOD` (GDI), `HOUSE_BAD` (Nod) |
| `PlayerPtr` | `HouseClass*` | Pointer to human player's House object |
| `RequiredCD` | `int` | CD check requirement for next mission (-2=none, -1=any, 0=GDI, 1=Nod, 2=Covert Ops) |
| `CarryOverMoney` | `int` | Credits carried to next mission |
| `SabotagedType` | `int` | Building type sabotaged in previous mission (for GDI mission 7 airfield) |
| `BriefMovie`, `ActionMovie`, `IntroMovie`, `WinMovie`, `LoseMovie` | `char[]` | Movie filenames from scenario INI |
| `TransitTheme` | `int` | Theme song to play during transition (`THEME_NONE`, `THEME_AOI`, etc.) |
| `BriefingText` | `char[]` | Preloaded mission briefing text from MISSION.INI |
| `TutorFlags[2]` | `long[]` | Tutorial message flags (reset for scenarios 1-4) |
| `EndCountDown` | `int` | Countdown timer for end-of-mission (30 seconds default) |
| `CrateCount`, `CrateTimer`, `CrateMaker` | various | Crate spawning system state |
| `InMainLoop` | `bool` | Flag indicating main game loop is active |
| `PlaybackGame` | `bool` | Whether playing a recorded game |
| `RunningAsDLL` | `bool` | Whether running as DLL (Remastered) vs standalone |
| `Special.IsJurassic`, `Special.ModernBalance`, `AreThingiesEnabled` | various | Special game mode flags |

### Subsystem Objects Initialized/Cleared
- `Map` — `MapClass` (terrain, cells, radar)
- `Score` — Score tracking
- `Logic` — Game logic/scripting
- `HouseClass` — Player houses/factions
- `ObjectClass` — Base game object
- `TeamTypeClass`, `TeamClass` — AI team definitions and instances
- `TriggerClass` — Mission triggers
- `AircraftClass`, `AnimClass`, `BuildingClass`, `BulletClass`, `InfantryClass`, `OverlayClass`, `SmudgeClass`, `TemplateClass`, `TerrainClass`, `UnitClass` — Core game entity types
- `FactoryClass` — Unit production queues
- `Base` — AI base planning (`BaseClass` / `BaseNodeClass`)
- `CurrentObject` — Current selection/cursor object

### Difficulty Constants
- `DIFF_EASY` (0), `DIFF_NORMAL` (1), `DIFF_HARD` (2) — passed to `HouseClass::Assign_Handicap()`

### House Constants
- `HOUSE_GOOD` — GDI
- `HOUSE_BAD` — Nod

### Structure Types (referenced)
- `STRUCT_AIRSTRIP` — GDI airfield (mission 7 sabotage)
- `STRUCT_WEAP` — Weapons Factory (Modern Balance: +30% health)
- `STRUCT_REPAIR` — Repair Pad (Modern Balance: APC prerequisite)
- `STRUCT_NONE` — No structure

### Unit/Aircraft Types (referenced)
- `UNIT_APC` — Armored Personnel Carrier
- `AIRCRAFT_ORCA` — Orca aircraft (PATSUX mission: laser weapon)

### Infantry Types (referenced)
- `INFANTRY_FIRST` to `INFANTRY_COUNT` — Iteration range
- `IsFraidyCat`, `IsAvoidingTiberium` — Civilian behavior flags (SCG08EB mission)

### Movie/Theme Constants
- `THEME_AOI` — "Act on Instinct" theme song
- `THEME_NONE` — No transition theme

## Functions and methods

### `Start_Scenario(char *root, bool briefing)` → `bool`
**Entry point for starting a scenario.** Loads scenario data, plays movies, queues theme music, and applies options.

**Parameters:**
- `root` — Scenario filename root (e.g., "SCG01EA")
- `briefing` — Whether to play briefing movie (false on restart)

**Returns:** `true` on success, `false` if `Read_Scenario` fails

**Flow:**
1. Calls `Read_Scenario(root)` — loads INI, initializes all subsystems
2. **DEMO build:** Plays BriefMovie + ActionMovie, queues THEME_AOI
3. **Full game (non-DEMO):**
   - If `Scenario < 20` and not Jurassic Park special mode:
     - Plays IntroMovie (except Scenario 1 Nod)
     - Plays BriefMovie (with `PreserveVQAScreen = true` for Scenario 1)
     - Plays ActionMovie with `TransitTheme`
     - Queues THEME_AOI if no transit theme
   - Else (covert ops/Jurassic):
     - Plays BriefMovie + ActionMovie
     - **NEWMENU:** If briefing VQA missing, shows text briefing via `Restate_Mission` (commented)
4. Calls `Options.Set()` to apply video/audio settings
5. Returns success

**Side effects:** Initializes entire game state, plays movies, sets global `RequiredCD` for next mission

---

### `Set_Scenario_Difficulty(int difficulty)` → `void`
**Sets player handicap for single-player campaign.** Must be called after `Start_Scenario` initializes `PlayerPtr`.

**Parameters:**
- `difficulty` — 0=Easy, 1=Normal, 2=Hard

**Behavior:**
- Only operates if `GameToPlay == GAME_NORMAL`
- Calls `PlayerPtr->Assign_Handicap()` with corresponding `DIFF_*` constant
- No-op for invalid values

---

### `Read_Scenario(char *root)` → `bool`
**Loads scenario from disk.** Core loading routine called by `Start_Scenario` and `Do_Restart`.

**Parameters:**
- `root` — Scenario filename root

**Returns:** `true` on success, `false` on failure

**Flow:**
1. Debug log entry
2. Calls `Clear_Scenario()` — wipes all previous scenario state
3. Increments `ScenarioInit` (reentrancy guard)
4. Calls `Read_Scenario_Ini(root)` (in `LOAD.CPP`) — parses INI file
5. On success:
   - Calls `Fill_In_Data()` — rebuilds derived game state
   - Sets map view dimensions to full map size
   - Clears `TutorFlags[0]` and `TutorFlags[1]` for scenarios 1-4 (tutorial reset)
6. On failure:
   - Logs error via `GlyphX_Debug_Print`
   - Returns `false`
7. Decrements `ScenarioInit`
8. Debug log exit

**Side effects:** Complete game state reset + load; modifies `ScenarioInit`, `TutorFlags`, map view

---

### `Fill_In_Data()` → `void`
**Reconstructs derived game state after INI load.** Called by `Read_Scenario` after successful INI parse.

**Behavior:**
1. Increments `ScenarioInit`
2. Iterates all buildings, calls `Update_Buildables()` on each (refreshes build queues/prerequisites)
3. Flags map for full redraw via `Map.Flag_To_Redraw(true)`
4. If multiplayer (`GameToPlay != GAME_NORMAL`), shows player names on radar via `Map.Player_Names(1)`
5. Decrements `ScenarioInit`

**Side effects:** Updates building buildables, marks map dirty, shows MP player names

---

### `Clear_Scenario()` → `void`
**Wipes all scenario-specific game state.** Called at start of `Read_Scenario` before loading new scenario.

**Resets:**
- `EndCountDown = 30 seconds` (TICKS_PER_SECOND * 30)
- `CrateCount = 0`, `CrateTimer = 0`, `CrateMaker = false`

**Calls Init() on all subsystems:**
- `Map.Init_Clear()` — clears cell array only (not full Display init)
- `Score.Init()`, `Logic.Init()`
- `HouseClass::Init()`, `ObjectClass::Init()`, `TeamTypeClass::Init()`, `TeamClass::Init()`, `TriggerClass::Init()`
- `AircraftClass::Init()`, `AnimClass::Init()`, `BuildingClass::Init()`, `BulletClass::Init()`, `InfantryClass::Init()`, `OverlayClass::Init()`, `SmudgeClass::Init()`, `TemplateClass::Init()`, `TerrainClass::Init()`, `UnitClass::Init()`
- `FactoryClass::Init()`
- `Base.Init()` — AI base planning
- `CurrentObject.Clear_All()` — selection state
- `Invalidate_Cached_Icons()` — UI icon cache

**Note:** Does NOT call `Map.Init()` or `Display::Init()` (would reload MixFiles and require theater). `Display::Read_INI` handles its own init.

---

### `Do_Win()` → `void`
**Handles mission victory sequence.** Called when player completes mission objectives.

**Flow:**
1. Sets mouse to normal, hides mouse
2. If multiplayer: clears `MPlayerGameName`, shows score presentation, increments game counter, sets `GameActive = 0`, returns
3. Single-player:
   - Displays "MISSION SCENARIO WON" text centered on radar map
   - 3-second countdown with `VOX_ACCOMPLISHED` voice
   - Sets `RequiredCD` for next mission:
     - Scenarios 20-59 (Covert Ops): CD 2
     - Scenarios 60+ (Jurassic): CD -1 (any)
     - GDI campaign: CD 0 (GDI disc)
     - Nod campaign: CD 1 (Nod disc)
   - Plays `WinMovie` (skipped in DEMO)
   - **If not playback game:**
     - **DEMO:** Cycles through demo missions (1→10→6→GDI Ending)
     - **Full game:**
       - Covert ops (Scenario ≥ 20): Score presentation → end game
       - Nod mission 13: `Nod_Ending()` → end game
       - GDI mission 15: `GDI_Ending()` → end game
       - Jurassic mission 5: `Prog_End()` → exit
       - Standard campaign: Score presentation → `Map_Selection()` (mission select)
       - GDI mission 6 → skips mission 7 if airfield sabotaged (`SabotagedType == STRUCT_AIRSTRIP`)
     - Increments `Scenario`
   - Carries over `PlayerPtr->Credits` to `CarryOverMoney`
   - Preserves `NukePieces` across mission transition
   - Generates next scenario filename via `Set_Scenario_Name()`
   - Calls `Start_Scenario(ScenarioName)` for next mission
   - **Mission 7 sabotage cleanup (GDI):**
     - Finds and destroys sabotaged building (Airstrip) on map
     - Removes from AI `Base` prebuild list
   - Resets `SabotagedType = STRUCT_NONE`
   - Renders map, fades in palette, shows mouse

**Side effects:** Complete mission transition; loads next scenario; modifies `Scenario`, `RequiredCD`, `CarryOverMoney`, `SabotagedType`, map state

---

### `Do_Lose()` → `void`
**Handles mission defeat sequence.** Called when player loses (construction yard destroyed, etc.).

**Flow:**
1. Sets mouse to normal, hides mouse
2. If multiplayer: clears `MPlayerGameName`, shows score presentation, increments game counter, sets `GameActive = 0`, shows mouse, returns
3. Single-player:
   - Displays "MISSION SCENARIO LOST" text centered on radar map
   - 3-second countdown with `VOX_FAIL` voice
   - Plays `LoseMovie`
   - Restores palette, shows mouse
   - Prompts "TO REPLAY?" via `CCMessageBox` (YES/NO)
   - If YES: hides mouse, clears keyboard, calls `Start_Scenario(ScenarioName, false)` to restart mission
   - If NO: sets `GameActive = 0` (return to menu)
   - Fades palette, shows mouse

**Side effects:** Either restarts current mission or ends game session

---

### `Do_Restart()` → `void`
**Handles manual mission restart (from menu).** Called from main game loop when player chooses Restart Mission.

**Flow:**
1. Saves mouse visibility state
2. Shows mouse if hidden
3. Shows "RESTARTING" message box
4. Sets mouse to normal, clears keyboard
5. Calls `Start_Scenario(ScenarioName, false)` — reloads same mission without briefing
6. Restores mouse visibility
7. Clears keyboard, renders map

**Side effects:** Full scenario reload without briefing movie

---

### `Restate_Mission(char const *name, int button1, int button2)` → `bool`
**Displays mission objective text and optional briefing movie prompt.** Called from in-game menu (F1 or Objective button).

**Parameters:**
- `name` — Scenario identifier for briefing text lookup (e.g., "SCG01EA")
- `button1`, `button2` — Text IDs for dialog buttons (e.g., `TXT_OK`, `TXT_NONE`, `TXT_YES`, `TXT_NO`)

**Returns:** `true` if player requested briefing movie, `false` otherwise

**Flow:**
1. **JAPANESE:** Looks for `.CPS` file, shows it in message box if found
2. **Other languages:**
   - **NEWMENU:** Checks if BriefMovie/ActionMovie VQA files exist; if neither, forces text-only (button1=TXT_OK, button2=TXT_NONE, brief=false)
   - If `BriefingText` not empty:
     - Copies text to local buffer
     - Shows `CCMessageBox` with "OBJECTIVE" title and briefing text
     - If player clicks button1 (usually "Play Briefing"): returns `true`
     - If player clicks button2 (usually "Return"): returns `false` (or `true` if no movies available)

**Side effects:** Shows modal dialog; mouse state preserved

---

### `Fixup_Scenario()` → `void`
**Applies scenario-specific gameplay tweaks/balance changes.** Called after scenario load to handle special cases.

**Behavior:**
1. **"Fraidycat" civilians (SCG08EB — GDI Mission 8, Variant B):**
   - If `GameToPlay == GAME_NORMAL`, `PlayerPtr->ActLike == HOUSE_GOOD`, `Scenario == 8`, `ScenVar == SCEN_VAR_B`
   - Sets `IsAvoidingTiberium = true` for all infantry types with `IsFraidyCat` flag
2. **PATSUX secret mission (Scenario 72):**
   - If `Scenario == 72`: Sets Orca aircraft primary weapon to `WEAPON_OBELISK_LASER`
   - Else: Sets Orca primary weapon to `WEAPON_DRAGON` (default)
3. **Modern Balance mode (`Special.ModernBalance`):**
   - **Enabled:** Weapons Factory (`STRUCT_WEAP`) health = 520 (+30%); Repair Pad (`STRUCT_REPAIR`) becomes prerequisite for APC (`UNIT_APC`)
   - **Disabled:** Weapons Factory health = 400 (original); APC prerequisite unchanged

**Side effects:** Modifies global type definitions (InfantryTypeClass, AircraftTypeClass, BuildingTypeClass, UnitTypeClass) — affects all instances

## Inputs, outputs and side effects

### Inputs
- **Scenario INI files** (`SCG##??.INI`) — parsed by `Read_Scenario_Ini` (LOAD.CPP)
- **MISSION.INI** — provides `BriefingText` for each scenario
- **Movie files** (`.VQA`) — `IntroMovie`, `BriefMovie`, `ActionMovie`, `WinMovie`, `LoseMovie`
- **Global game state** — `Scenario`, `Whom`, `GameToPlay`, `Special.*`, `PlayerPtr`, etc.
- **User input** — via `CCMessageBox` (Restate_Mission, Do_Lose retry prompt)

### Outputs
- **Game state initialization** — all subsystems reset and loaded
- **Next scenario filename** — `ScenarioName` updated via `Set_Scenario_Name`
- **Carry-over resources** — `CarryOverMoney`, `NukePieces` preserved across missions
- **CD requirement** — `RequiredCD` set for next mission disc check
- **UI/Visual** — Movies played, text displayed, palette faded, mouse shown/hidden

### Side Effects (Global State Mutations)
| Variable | Modified By |
|----------|-------------|
| `Scenario` | `Do_Win` (increment), `Do_Restart` (same), `Read_Scenario` (assumed pre-set) |
| `ScenarioInit` | `Read_Scenario`, `Fill_In_Data`, `Clear_Scenario` (increment/decrement) |
| `RequiredCD` | `Do_Win` (set for next mission) |
| `CarryOverMoney` | `Do_Win` (from PlayerPtr->Credits) |
| `SabotagedType` | `Do_Win` (reset after mission 7 cleanup) |
| `TutorFlags[0..1]` | `Read_Scenario` (cleared for scenarios 1-4) |
| `EndCountDown`, `CrateCount`, `CrateTimer`, `CrateMaker` | `Clear_Scenario` (reset) |
| `MPlayerGameName`, `MPlayerGamesPlayed`, `MPlayerCurGame` | `Do_Win`, `Do_Lose` (multiplayer) |
| `GameActive` | `Do_Win`, `Do_Lose`, `Do_Restart` (0 = stop) |
| `InfantryTypeClass::IsAvoidingTiberium` | `Fixup_Scenario` (SCG08EB) |
| `AircraftTypeClass::Primary` (ORCA) | `Fixup_Scenario` (Scenario 72) |
| `BuildingTypeClass::MaxStrength` (WEAP) | `Fixup_Scenario` (Modern Balance) |
| `UnitTypeClass::Pre` (APC) | `Fixup_Scenario` (Modern Balance) |
| All subsystem `Init()` state | `Clear_Scenario` |

## Calls into / called by

### Called By (External Entry Points)
| Caller | Function | Context |
|--------|----------|---------|
| `MAIN.CPP` (main loop) | `Do_Restart()` | Player selects "Restart Mission" from menu |
| `MAIN.CPP` (win condition) | `Do_Win()` | Mission objectives completed |
| `MAIN.CPP` (lose condition) | `Do_Lose()` | Construction yard destroyed / player defeated |
| `MAIN.CPP` / Menu | `Restate_Mission()` | Player presses F1 or clicks Objective button |
| Campaign menu / `STARTUP.CPP` | `Start_Scenario()` | New game / continue campaign |
| `LOAD.CPP` / `Read_Scenario_Ini` | `Fixup_Scenario()` | After INI parsing, before gameplay starts |

### Calls Into (Dependencies)
| Target | Functions Called | Purpose |
|--------|------------------|---------|
| `LOAD.CPP` | `Read_Scenario_Ini(char*)` | Parse scenario INI file |
| `MOVIE.CPP` | `Play_Movie(char*, int)`, `Play_Movie(char*)` | Play VQA movies |
| `THEME.CPP` / `Theme` | `Queue_Song(int)` | Queue transition/theme music |
| `OPTIONS.CPP` / `Options` | `Set()` | Apply video/audio settings |
| `MapClass` / `Map` | `Set_View_Dimensions`, `Get_Tab_Height`, `Flag_To_Redraw`, `Player_Names`, `Render`, `Set_Default_Mouse`, `TacPixelX`, `TacPixelY`, `TacLeptonWidth`, `TacLeptonHeight` | Map view, radar, rendering |
| `HouseClass` | `Init()`, `Assign_Handicap(int)`, `PlayerPtr->Credits`, `PlayerPtr->NukePieces`, `PlayerPtr->Class->House`, `PlayerPtr->ActLike` | House/player management |
| `BuildingClass` / `Buildings` | `Count()`, `Ptr(index)`, `Update_Buildables()`, `IsInLimbo`, `House`, `Class->Type`, `Limbo()`, `delete` | Building management |
| `BaseClass` / `Base` | `Init()`, `Nodes.Count()`, `Get_Node(index)`, `Nodes.Delete(index)` | AI base planning |
| `Score` | `Init()`, `Presentation()` | Score tracking/display |
| `Logic` | `Init()` | Scripting/logic system |
| `*Class::Init()` (14 types) | Static `Init()` methods | Entity pool reset |
| `FactoryClass` | `Init()` | Production queues |
| `CurrentObject` | `Clear_All()` | Selection state |
| `ICON.CPP` | `Invalidate_Cached_Icons()` | UI icon cache |
| `CCMessageBox` | `Process(text, btn1, btn2)`, `Process(int txtId)` | Modal dialogs |
| `SOUND.CPP` | `Speak(int vox)`, `Stop_Speaking()`, `Is_Speaking()` | Voice lines |
| `TIMER.CPP` / `CountDownTimer` | `Set(int)`, `Time()` | Countdown timers |
| `KEYBOARD.CPP` / `Keyboard` | `Clear()` | Input buffer flush |
| `PALETTE.CPP` | `Fade_Palette_To`, `Set_Palette`, `GamePalette` | Palette transitions |
| `MOUSE.CPP` | `Show_Mouse()`, `Hide_Mouse()`, `Get_Mouse_State()`, `MOUSE_NORMAL` | Mouse visibility |
| `NEWMENU` / `Map_Selection()` | `Map_Selection()` | Mission select screen |
| `ENDING.CPP` | `GDI_Ending()`, `Nod_Ending()`, `Prog_End()` | Campaign endings |
| `MULTI.CPP` | `Multi_Score_Presentation()` | Multiplayer score screen |
| `SCENARIO.CPP` (self) | `Set_Scenario_Name()`, `Clear_Scenario()`, `Fill_In_Data()` | Internal helpers |
| `FILE.CPP` / `CCFileClass` | `Is_Available()` | Check movie file existence |

## Related original files

| File | Relationship |
|------|--------------|
| `LOAD.CPP` / `LOAD.H` | `Read_Scenario_Ini()` implementation; defines scenario INI format |
| `FUNCTION.H` | Global declarations for all functions/variables used here |
| `MOVIE.CPP` | Movie playback (`Play_Movie`) |
| `THEME.CPP` / `THEME.H` | Theme music system (`Theme.Queue_Song`) |
| `OPTIONS.CPP` / `OPTIONS.H` | Options application (`Options.Set()`) |
| `MAP.CPP` / `MAP.H` | Map rendering, radar, view management |
| `HOUSE.CPP` / `HOUSE.H` | `HouseClass`, player management, handicaps |
| `BUILDING.CPP` / `BUILDING.H` | `BuildingClass`, `Buildings` list, buildables |
| `BASE.CPP` / `BASE.H` | AI base planning (`BaseClass`, `BaseNodeClass`) |
| `SCORE.CPP` / `SCORE.H` | Score presentation |
| `LOGIC.CPP` / `LOGIC.H` | Game logic/scripting |
| `TEAM.CPP` / `TEAM.H`, `TEAMTYPE.CPP` / `TEAMTYPE.H` | AI teams |
| `TRIGGER.CPP` / `TRIGGER.H` | Mission triggers |
| `UNIT.CPP` / `UNIT.H`, `INFANTRY.CPP` / `INFANTRY.H`, `AIRCRAFT.CPP` / `AIRCRAFT.H` | Entity classes |
| `OVERLAY.CPP`, `SMUDGE.CPP`, `TERRAIN.CPP`, `TEMPLATE.CPP`, `BULLET.CPP`, `ANIM.CPP` | Other entity types |
| `FACTORY.CPP` / `FACTORY.H` | Production queues |
| `ICON.CPP` / `ICON.H` | Icon cache invalidation |
| `MESSAGE.CPP` / `MESSAGE.H` | `CCMessageBox` dialogs |
| `SOUND.CPP` / `SOUND.H` | Voice lines (`Speak`, `VOX_*`) |
| `TIMER.CPP` / `TIMER.H` | `CountDownTimer` |
| `KEYBOARD.CPP` / `KEYBOARD.H` | Input clearing |
| `PALETTE.CPP` / `PALETTE.H` | Palette fading/setting |
| `MOUSE.CPP` / `MOUSE.H` | Mouse control |
| `ENDING.CPP` | `GDI_Ending`, `Nod_Ending`, `Prog_End` |
| `MULTI.CPP` | Multiplayer score presentation |
| `SPECIAL.H` | `Special` struct (ModernBalance, IsJurassic) |
| `RULES.CPP` / `RULES.H` | Game rules/constants (`DIFF_*`, `HOUSE_*`, `STRUCT_*`, `UNIT_*`, `AIRCRAFT_*`, `WEAPON_*`) |

## Rust port mapping

| C++ Component | Rust Crate | Mapping Notes |
|---------------|------------|---------------|
| `Start_Scenario`, `Read_Scenario`, `Clear_Scenario`, `Fill_In_Data` | `cnc-tiberian-dawn` | Core scenario lifecycle → `ScenarioManager::load()`, `ScenarioManager::restart()`, `ScenarioManager::transition_to_next()` |
| `Do_Win`, `Do_Lose`, `Do_Restart` | `cnc-tiberian-dawn` | Game outcome handlers → `GameOutcomeHandler` trait implementations |
| `Restate_Mission` | `cnc-tiberian-dawn` / `cnc-core` | UI dialog → `MissionBriefingSystem::show_objective()` |
| `Fixup_Scenario` | `cnc-tiberian-dawn` | Scenario-specific patches → `ScenarioPatch::apply(ScenarioId)` |
| `Set_Scenario_Difficulty` | `cnc-tiberian-dawn` | Difficulty → `GameDifficulty` enum + `Player::set_handicap()` |
| Subsystem `Init()` calls | `cnc-core` | Entity pools → `World::reset()`, `EntityPool::clear_all()` |
| Movie/Theme playback | `cnc-asset` + `cnc-tiberian-dawn` | `MoviePlayer::play()`, `MusicSystem::queue()` |
| INI parsing (`Read_Scenario_Ini`) | `cnc-asset` | `IniScenarioLoader::load(root)` |
| Map operations | `cnc-core` | `Map::set_view()`, `Map::flag_redraw()`, `Map::render()` |
| House/Player management | `cnc-core` | `House::init()`, `Player::credits`, `Player::nuke_pieces` |
| Building/Base management | `cnc-core` | `Building::update_buildables()`, `AIBase::init()`, `AIBase::remove_prebuild()` |
| Score/Presentation | `cnc-tiberian-dawn` | `ScoreSystem::present()` |
| Dialogs/Message boxes | `cnc-core` (UI) | `DialogSystem::show_message_box()` |
| Voice/Sound | `cnc-asset` | `AudioSystem::speak(VOX_*)` |
| Palette/Mouse/Keyboard | `cnc-core` (platform) | `Platform::fade_palette()`, `Platform::set_mouse_visible()` |
| Campaign endings | `cnc-tiberian-dawn` | `CampaignEnding::gdi()`, `CampaignEnding::nod()` |
| Multiplayer score | `cnc-tiberian-dawn` (MP) | `MultiplayerScore::present()` |

**Key Rust types to define:**
```rust
// cnc-tiberian-dawn/src/scenario/
pub struct ScenarioManager {
    pub current: ScenarioId,
    pub name: ScenarioName,
    pub difficulty: GameDifficulty,
    pub carry_over_money: i32,
    pub nuke_pieces: u8,
    pub sabotaged_type: Option<StructureType>,
    pub required_cd: CdRequirement,
}

pub enum GameOutcome { Win, Lose, Restart }

pub trait ScenarioPatch {
    fn apply(&self, world: &mut World, scenario: ScenarioId);
}

// cnc-core/src/world/
pub struct World {
    pub map: Map,
    pub houses: Vec<House>,
    pub buildings: EntityPool<Building>,
    pub teams: EntityPool<Team>,
    pub triggers: EntityPool<Trigger>,
    // ... all entity pools
    pub ai_base: AIBase,
    pub score: ScoreSystem,
    pub logic: LogicSystem,
}
```

## Edge cases and compatibility notes

### Campaign Progression Logic
1. **Mission 7 airfield sabotage (GDI):** If player destroys Airstrip in mission 6, mission 7 is skipped (`Scenario++` twice). The sabotaged building is removed from map and AI prebuild list in `Do_Win`.

2. **Covert Operations (Scenario ≥ 20):** No mission select screen (`Map_Selection`); goes directly to score presentation then ends game. Uses CD 2.

3. **Jurassic Park (Scenario ≥ 60, Special.IsJurassic):** Special movie flow (no intro); mission 5 ends game via `Prog_End()`. Uses CD -1 (any disc).

4. **Demo mode:** Hardcoded mission cycle (1→10→6→GDI Ending). No mission select.

5. **Secret mission (Scenario 72 - PATSUX):** Orca gets Obelisk laser via `Fixup_Scenario`.

### Difficulty System
- Only works in single-player (`GAME_NORMAL`)
- Must be called AFTER `Start_Scenario` (needs `PlayerPtr` initialized)
- Maps 0/1/2 → `DIFF_EASY`/`DIFF_NORMAL`/`DIFF_HARD`

### Modern Balance (Special.ModernBalance)
- Global toggle affecting type definitions at load time
- Weapons Factory: 400 → 520 HP (+30%)
- APC: Gains Repair Pad prerequisite
- Applied in `Fixup_Scenario` after INI load, so INI values are overridden

### Fraidycat Civilians (SCG08EB)
- Only for GDI Mission 8 Variant B (`Scenario==8`, `ScenVar==SCEN_VAR_B`, `ActLike==HOUSE_GOOD`)
- Sets `IsAvoidingTiberium` on all `IsFraidyCat` infantry types
- Mission-critical civilians won't wander into Tiberium

### Reentrancy Guard (`ScenarioInit`)
- Incremented in `Read_Scenario`, `Fill_In_Data`, `Clear_Scenario`
- Prevents nested initialization calls
- Not a full mutex; single-threaded game loop assumption

### Multiplayer vs Single-Player Divergence
- `Do_Win`/`Do_Lose` have completely different flows for `GAME_NORMAL` vs multiplayer
- Multiplayer: no movies, no campaign progression, shows MP score, increments `MPlayerGamesPlayed`
- Single-player: full campaign flow with movies, endings, mission select

### Movie File Fallback (NEWMENU)
- If BriefMovie/ActionMovie VQA files missing, `Restate_Mission` falls back to text-only briefing
- `Do_Win`/`Start_Scenario` still attempt to play movies (may fail silently)

### Palette/Mouse Management
- Extensive manual palette fading (`Fade_Palette_To`) and mouse show/hide
- `PreserveVQAScreen` flag for Scenario 1 preserves VQA screen during briefing
- Remastered version runs as DLL (`RunningAsDLL`); `Prog_End` doesn't call `exit(0)`

### Build Configuration Differences
- `DEMO`: Stripped campaign flow, hardcoded mission cycle
- `NEWMENU`: Remastered UI with file existence checks, text fallback
- `GERMAN`/`FRENCH`/`JAPANESE`: Localized text handling (Japanese uses .CPS files)
- `OBSOLETE`: Dead code (debug map editor on lose)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SCENARIO.CPP at f1f0d42