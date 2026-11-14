# PATH/FILE
original/TIBERIANDAWN/GLOBALS.CPP

## Purpose
Global state definitions for Tiberian Dawn. Declares all module-level singletons, object pools, game configuration, multiplayer state, debug flags, and core system instances.

## Where it fits in the game
Central global namespace - single translation unit defining all cross-module state. Included via `function.h` extern declarations. Contains object heaps, game systems, network state, UI resources, and scenario data.

## Classes, structs, enums and important data

### Debug flags (CHEAT_KEYS)
| Flag | Purpose |
|------|---------|
| `Debug_Quiet` | Suppress debug output |
| `Debug_Cheat` | Enable cheats |
| `Debug_Remap` | Debug remap tables |
| `Debug_Icon` | Debug icons |
| `Debug_Flag` | General debug mode |
| `Debug_Lose` / `Debug_Win` | Force lose/win |
| `Debug_Map` | Map editor mode |
| `Debug_Passable` | Show passable terrain |
| `Debug_Unshroud` | Hide shroud |
| `Debug_Threat` | Show threat maps |
| `Debug_Find_Path` | Pathfinding debug |
| `Debug_Check_Map` | Validate map per frame |
| `Debug_Playtest` | Playtest mode |
| `Debug_Instant_Build` | Instant construction |
| `Debug_Force_Crash` | Trigger crash |

### Object pools (TFixedIHeapClass<T>)
Fixed-size pools with `ID()` validation, `Allocate()`/`Free()`:

| Pool | Type | Max |
|------|------|-----|
| `Units` | UnitClass | |
| `Factories` | FactoryClass | FACTORY_MAX |
| `Terrains` | TerrainClass | |
| `Templates` | TemplateClass | |
| `Smudges` | SmudgeClass | |
| `Overlays` | OverlayClass | |
| `Infantry` | InfantryClass | |
| `Bullets` | BulletClass | |
| `Buildings` | BuildingClass | |
| `Anims` | AnimClass | ANIM_MAX |
| `Aircraft` | AircraftClass | |
| `Triggers` | TriggerClass | |
| `TeamTypes` | TeamTypeClass | TEAMTYPE_MAX |
| `Teams` | TeamClass | TEAM_MAX |
| `Houses` | HouseClass | HOUSE_COUNT |

### Core systems (singletons)
| Instance | Type | Purpose |
|----------|------|---------|
| `Logic` | LogicClass | Frame logic scheduling |
| `Theme` | ThemeClass | Music playback |
| `Map` | MouseClass/MapEditClass | Map control |
| `Score` | ScoreClass | Game scoring |
| `CreditDisplay` | CreditClass | Credit ticker |
| `Special` | SpecialClass | Game rule overrides |
| `Rule` | RulesClass | Game rules (RULES.INI) |
| `Options` | GameOptionsClass | User settings |
| `Base` | BaseClass | AI base layout |
| `OutList` | QueueClass<EventClass> | Outgoing network events |
| `DoList` | QueueClass<EventClass> | Incoming scheduled events |
| `Messages` | MessageListClass | Chat messages |
| `Ipx` | IPXManagerClass | IPX network manager |
| `Kbd` | WWKeyboardClass | Keyboard input |
| `WWMouse` | WWMouseClass* | Mouse input |
| `WindowsTimer` | WinTimerClass* | Windows timer |
| `FrameTimer` | CountDownTimerClass | Frame pacing (15 FPS) |
| `ProcessTimer` | TimerClass | Logic time measurement |

### View system
- `Views[4]` (CELL) - Saved tactical view positions (hotkeys 0-3)
- `View[WAYPT_COUNT]` (CELL) - Waypoint positions (A-Z)

### Scenario state
| Variable | Purpose |
|----------|---------|
| `Whom` | Initial house (GDI/NOD) |
| `Scenario` | Scenario number |
| `ScenPlayer` | SINGLE_GDI/SINGLE_NOD/MULTI_2PLAYER/MULTI_MULTIPLAYER |
| `ScenDir` | EAST/WEST |
| `ScenVar` | A/B/C variant |
| `ScenarioName` | Filename |
| `CarryOverMoney/Percent/Cap` | Campaign carryover |
| `ScenarioInit` | Loading guard counter |
| `ScenarioCRC` | INI+map CRC for MP sync |
| `BuildLevel` | Tech tree tier (1-3) |
| `GameToPlay` | GAME_NORMAL/MODEM/IPX/INTERNET |
| `RequiredCD` | -1=any, -2=HD, 0=GDI, 1=Nod, 2=Covert |
| `RunningAsDLL` | GlyphX client mode |

### Win/lose state
- `PlayerWins` / `PlayerLoses` / `PlayerRestarts` / `PlayerAborts`
- `TempleIoned` - Nod temple destroyed by ion cannon
- `SabotagedType` - Building pre-destroyed for next mission

### Multiplayer state
| Variable | Purpose |
|----------|---------|
| `MPlayerID[MAX_PLAYERS]` | Packed: house(4bits) + color(4bits) |
| `MPlayerHouses[MAX_PLAYERS]` | Actual HousesType |
| `MPlayerNames[MAX_PLAYERS][MPLAYER_NAME_MAX]` | Player names |
| `MPlayerGColors/TColors[6]` | Remap indices for graphics/text |
| `MPlayerCount` | Human players |
| `MPlayerBases` | Production enabled |
| `MPlayerCredits/Tiberium/Goodies/Ghosts` | Game options |
| `MPlayerUnitCount` | Unit limit (non-base) |
| `MPlayerBlitz` | AI all-at-once attack |
| `MPlayerObiWan` | Defeated spectator mode |
| `MPlayerScore[MAX_MULTI_NAMES]` | Scoreboard |
| `MPlayerGamesPlayed` | Session counter |
| `FrameSendRate` | Frames between packets |
| `MPlayerMaxAhead` | Frame lookahead for sync |
| `NewMaxAheadFrame1/2` | Sync vulnerability windows |

### Network (IPX)
- `Ipx` - Manager with global channel (10 entries) + private queues (8 each)
- `NullModem` - Serial modem manager (disabled in remaster)
- `Games` / `Players` - DynamicVector of NodeNameType* for lobby
- `GPacket` / `GAddress` / `GProductID` - Global channel packet
- `MetaPacket` - Batched events (546 bytes max)
- `BridgeNet` / `IsBridge` - Bridge crossing
- `NetStealth` / `NetProtect` / `NetOpen` - Visibility flags
- `MPlayerGameName` - Session name

### Rendering resources
| Resource | Purpose |
|----------|---------|
| `VisiblePage` / `HiddenPage` | GraphicBufferClass (double buffer) |
| `SeenBuff` / `HidPage` | Viewports (1536×1536 virtual) |
| `ModeXBuff` / `SysMemPage` | System memory buffers |
| `GamePalette` / `BlackPalette` / `WhitePalette` / `OriginalPalette` / `Palette` | 256-color palettes |
| `Font*Ptr` | Various font pointers (6pt, 8pt, LED, VCR, Score, etc) |

### Input/recording
- `RecordFile` (CCFileClass) - "RECORD.BIN"
- `RecordGame` / `SuperRecord` / `PlaybackGame` / `AllowAttract`
- `SpeakQueue` (VoxType) - Pending voice sample
- `AllowVoice` - Suppresses duplicate unit responses
- `CrateCount` / `CrateTimer` / `CrateMaker` - Crate spawning

### Debug/trap
- `TrapFrame` / `TrapObjType` / `TrapObject` / `TrapCoord` / `TrapThis` / `TrapCell` / `TrapCheckHeap`
- `MonoArray[MonoClass::MAX_MONO_PAGES]` / `MonoPage` - Monochrome debug screens

### Random/sync
- `Seed` - Synchronized RNG seed (MP)
- `RandSeedPtr` - Pointer to active seed
- `CustomSeed` - Override for bug reproduction

### Window/Menu layouts
- `WindowList[][8]` - Monochrome debug window rects
- `MenuList[][8]` - Menu dimensions

### Misc
- `Frame` - Monotonic frame counter (saved in games)
- `SpareTicks` - Accumulated idle ticks (performance histogram)
- `EndCountDown` - Delays win/lose check
- `ScoresPresent` - Music files available
- `BreakoutAllowed` / `Brokeout` / `SlowPalette` - Movie control
- `IntroMovie` / `BriefMovie` / `WinMovie*` / `LoseMovie` / `ActionMovie` - VQ filenames
- `TransitTheme` - Transition music
- `SystemStrings` - Localized text pointer
- `GameActive` - Main loop guard
- `LParam` - Scratch long reference
- `Argc`/`Argv[20]` - Command line
- `ScreenWidth/Height` - Virtual resolution (1536)
- `hInstance` - Windows module handle
- `AllDone` - Exit flag
- `InMovie` / `MMXAvailable` - Runtime caps
- `GameStatisticsPacketSent` / `ConnectionLost` - MP stats
- `LastTheater` - Previous theater
- `CurrentObject` - Per-house selection (SelectedObjectsType)
- `CurrentCell` - Editor selected cell

## Functions and methods
None - pure data module. All access via extern declarations in `function.h`.

## Inputs, outputs and side effects
**Global mutable state** - Every system reads/writes these variables.

## Calls into / called by
**Referenced by virtually all .CPP files** via `function.h`:
- Object allocation: `Units.Allocate()`, `Factories.Free()`, etc
- Game loop: `Logic`, `Theme`, `Map`, `FrameTimer`
- Network: `Ipx`, `OutList`, `DoList`, `MetaPacket`
- UI: `Options`, `Score`, `CreditDisplay`, fonts, palettes
- Scenario: `Scenario*`, `BuildLevel`, `GameToPlay`
- MP: `MPlayer*`, `Games`, `Players`, `Messages`
- Debug: `MonoArray`, `Trap*`, debug flags

## Related original files
- **FUNCTION.H** - Extern declarations for all globals
- **HEAP.H** - TFixedIHeapClass template
- **LOGIC.H** - LogicClass
- **THEME.H** - ThemeClass
- **MAP.H** - MouseClass/MapEditClass
- **SCORE.H** - ScoreClass
- **CREDIT.H** - CreditClass
- **SPECIAL.H** - SpecialClass
- **RULES.H** - RulesClass
- **OPTIONS.H** - GameOptionsClass
- **BASE.H** - BaseClass
- **QUEUE.H** - QueueClass
- **IPX.H** - IPXManagerClass
- **KEYBOARD.H** - WWKeyboardClass
- **MOUSE.H** - WWMouseClass
- **TIMER.H** - TimerClass/CountDownTimerClass
- **MIXFILE.H** - MixFileClass
- **CCFILE.H** - CCFileClass

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/globals.rs` - Global state struct (replaces globals.cpp)
- `cnc-core/src/pools.rs` - TFixedIHeapClass → typed arenas
- `cnc-tiberian-dawn/src/game_state.rs` - Scenario/MP state
- `cnc-core/src/debug/trap.rs` - Debug trap system

**Not yet ported:**
- Single global namespace → modular state with explicit dependencies
- Fixed heaps → generational arenas or slab allocators
- Raw pointers → owned references with lifetimes
- Monochrome debug → logging/tracing framework
- IPX manager → abstraction over transport
- VQ movie system → video playback crate
- Palette management → renderer-owned
- Font pointers → asset manager
- Recording/playback → deterministic replay system
- Command line → clap argument parsing

## Edge cases and compatibility notes

1. **Single definition**: `GLOBALS.CPP` is the only definition; all others `extern` via `function.h`
2. **Pool validation**: `ID(this)` checks pool bounds (CHEAT_KEYS only)
3. **ScenarioInit guard**: Incremented during create/delete to prevent save/load corruption
4. **MP sync**: `ScenarioCRC`, `Seed`, `MPlayerMaxAhead` must match across peers
5. **Double buffering**: `VisiblePage`/`HiddenPage` swapped per frame
6. **Virtual resolution**: 1536×1536 viewport, scaled to actual window
7. **Font loading**: Lazy-loaded on first use (NULL initially)
8. **CD handling**: `RequiredCD` drives `Force_CD_Available()`
9. **DLL mode**: `RunningAsDLL` changes `Main_Game` flow
10. **Editor vs game**: `Map` type changes via `SCENARIO_EDITOR` define
11. **NullModem**: Disabled in remaster (`#if 0`)
12. **MetaPacket**: Batches events to fit IPX 546-byte limit
13. **Sync vulnerability**: `NewMaxAheadFrame1/2` track frame windows where MaxAhead increases
14. **Crate system**: Timer-based spawn when `CrateCount` reaches zero
15. **Voice suppression**: `AllowVoice` toggled during multi-select orders
16. **Carryover**: Campaign persistence via `CarryOverMoney/Percent/Cap`
17. **BuildLevel**: Gates sidebar tech tree (1=simplest, 3=full)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/GLOBALS.CPP at f1f0d42
