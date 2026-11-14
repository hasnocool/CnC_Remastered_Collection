# PATH/FILE
original/TIBERIANDAWN/CONQUER.CPP

## Purpose
Core game orchestration for Tiberian Dawn. Contains main game loop, startup/shutdown, input handling, CD management, recording/playback, debug utilities, and global utility functions.

## Where it fits in the game
Central game controller - entry point (`Main_Game`), main loop (`Main_Loop`), input processing (`Keyboard_Process`), and cross-cutting utilities. Coordinates all subsystems: map, UI, network, audio, scenario, recording.

## Classes, structs, enums and important data

### Global state
| Variable | Purpose |
|----------|---------|
| `InMainLoop` (bool) | Currently in main game loop |
| `GameToPlay` (GameType) | GAME_NORMAL/MODEM/IPX/INTERNET/NULL_MODEM |
| `PlaybackGame` (bool) | Playing recorded game |
| `RecordGame` / `SuperRecord` (bool) | Recording modes |
| `RunningAsDLL` (bool) | Running as GlyphX DLL |
| `SpecialDialog` (SDLG_) | Active special dialog (NONE/SPECIAL/OPTIONS/SURRENDER) |
| `Special.IsFromWChat` (bool) | Launched from WChat |
| `SpawnedFromWChat` (bool) | |
| `ConnectionLost` (bool) | Network disconnect |
| `Views[10]` (CELL) | Saved tactical view positions |

### GameType enum
- `GAME_NORMAL` - Single player
- `GAME_MODEM` / `GAME_NULL_MODEM` - Serial modem
- `GAME_IPX` - IPX LAN
- `GAME_INTERNET` - TCP/IP (Winsock)

### SpecialDialog enum
- `SDLG_NONE` - No dialog
- `SDLG_SPECIAL` - Special dialog (cheats/etc)
- `SDLG_OPTIONS` - Options menu
- `SDLG_SURRENDER` - Surrender confirmation

## Functions and methods

### Main_Game(argc, argv)
**Entry point** - Full game lifecycle.

1. `Init_Game(argc, argv)` - One-time init
2. Loop: `Select_Game(fade)` → returns false to exit
   - `fade=false` first iteration, then `true`
   - If `RunningAsDLL` → return immediately
3. Per-game setup:
   - `ScenarioInit = 0`
   - `Fade_Palette_To(GamePalette, MEDIUM)`
   - `Keyboard::Clear()`
   - Mouse show/hide based on `PlaybackGame`
   - `SpecialDialog = SDLG_NONE`
   - Network: `Register_Game_Start_Time()`, `DDEServer.Disable()`
   - `InMainLoop = true`
4. **Main loop** (editor or normal):
   - `TotalLocks = 0`
   - `Main_Loop()` → returns true to break
   - If `SpecialDialog != SDLG_NONE`:
     - `SDLG_SPECIAL` → `Special_Dialog()`
     - `SDLG_OPTIONS` → `Options.Process()`
     - `SDLG_SURRENDER` → `Surrender_Dialog()` → may add `DESTRUCT` event
5. Cleanup:
   - `InMainLoop = false`
   - Statistics packet if needed
   - `Fade_Palette_To(BlackPalette, SLOW)`
   - Network shutdown per `GameToPlay`
   - Recording file close
   - Demo: shows demo screen
   - `Free_Scenario_Descriptions()`
   - `Uninit_Game()`

### Keyboard_Process(input)
**Tactical map input handler** - Called per frame with `Input_Num()` result.

**Hotkeys:**
| Key | Action |
|-----|--------|
| `SPACE` / `ESC` | `Queue_Options()` |
| `HOME` | Center on selection (or fallback to H) |
| `H` | Find MCV/Construction Yard, select & center |
| `N` | Next/Prev object (SHIFT=prev, CTRL/ALT=team action) |
| `R` | Surrender dialog (MP) |
| `A` | Alliance dialog (MP/cheat) |
| `S` | Sell mode |
| `D` | Deploy (MCV) |
| `G` | Guard mode |
| `F` | Free scroll toggle (cheat) |
| `W`+ALT | Win cheat (`Blockage=false`, `Flag_To_Win()`) |
| `M`+modifiers | +10000 credits (cheat) |
| `/` (cheat) | Special dialog |

**Team actions** (via `Handle_Team`):
- No modifier: Select team
- SHIFT: Add to selection
- CTRL: Create team
- ALT: Create team (alt)

### Main_Loop() → bool
**Per-frame game logic** - Returns true to exit scenario.

(Implementation elided - handles game logic, AI, rendering, input)

### Color_Cycle() → bool
**Palette cycling** - Animates palette indices for water, tiberium, etc.

### Handle_Team(action)
**Team selection/creation** - `action`: 0=select, 1=add, 2=create, 3=create alt.

### Handle_View(view, action)
**Tactical view bookmarks** - 10 slots (0-9).
- `action=0`: Restore view → `Map.Set_Tactical_Position(Views[view])`
- `action=1`: Save view → `Views[view] = Coord_Cell(Map.TacticalCoord)`

### Trap_Object()
**Debug: Click-to-inspect** - Finds object under cursor, prints debug info.

### Force_CD_Available(cd) → bool
**CD verification & swap** - Ensures correct CD in drive.

**Modern path (enabled):**
- Tracks `_last` CD index
- If changed: Stops theme, deletes `MoviesMix`/`GeneralMix`/`ScoreMix`
- Recreates all three from `MOVIES.MIX`, `GENERAL.MIX`, `SCORES.MIX`
- `ThemeClass::Scan()`, restores theme

**Legacy path (disabled):**
- Searches all CD drives with timeouts (2-10 min for changers)
- Prompts user via `CCMessageBox` if not found
- Verifies via `GetVolumeInformation` + `movies.mix` existence
- Volume labels: `GDI95`, `NOD95`, `COVERT`
- On insert: Refreshes search paths, reloads MIX files

### Get_CD_Index(drive, timeout) → int
**Reads CD volume label** - Returns 0=GDI, 1=Nod, 2=Covert, -1=other.
- Retries on `ROR_NOT_READY` (changer swap)
- Verifies by opening `movies.mix`

### Disk_Space_Available() → unsigned long
**Returns free disk space** - Hardcoded to `0x7FFFFFFF` (2GB).

### Validate_Error(name)
**Validation failure handler** - Debug only.
- `GlyphX_Debug_Print(name)`
- Cheat builds: prints, `Prog_End()`, `exit(0)`

### Do_Record_Playback()
**Recording/playback sync** - Called per frame.

**Recording (`RecordGame`):**
- `SuperRecord`: Opens/closes file each frame
- Writes: `Map.DesiredTacticalCoord`, `CurrentObject.Count()`, CRC (sum of target IDs), all target IDs

**Playback (`PlaybackGame`):**
- Reads coord, updates map if different
- Reads count, CRC
- Compares CRC with current selection → `Unselect_All()` if mismatch
- Reads targets, selects objects (silently if CRC matched)
- `Map.Render()` (map not drawn in playback)

### Hires_Retrieve(name) → void const*
**Resolution-aware asset lookup** - Prefixes `H` for hi-res (>320 width).

### Get_Resolution_Factor() → int
Returns 0 (320) or 1 (hi-res).

### Blit_Hid_Page_To_Seen_Buff()
**Visible page update** - `HidPage.Blit(SeenBuff)`.

### Shake_The_Screen(shakes, house)
**Screen shake dispatch** - Adds `shakes*2` to `ScreenShakeTime` for human houses.

### Sync_Delay()
**Frame rate limiter** - Forces ~15 FPS.

### Message_Input(input)
**MP chat input** - Processes keys for inter-player messages.

### KN_To_Facing(kn) → FacingType
Converts keypad num to facing (1-9 → 8 directions).

### Name_From_Source(source) → char*
**SourceType → string** for UI.

### Source_From_Name(name) → SourceType
**String → SourceType** parsing.

### Theater_From_Name(name) → int
**String → theater index**.

### Language_Name() → char*
**Current language filename**.

### Fading_Table_Name() → char*
**Theater-specific fade table**.

### Get_Radar_Icon(shapefile) → void*
**Builds radar icon** from SHP.

### MixFileHandler()
**VQ movie file access**.

### Play_Movie()
**Plays VQ movie**.

### VQ_Call_Back()
**Movie maintenance callback**.

### Fetch_Techno_Type(type, id) → TechnoTypeClass*
**Type+ID → TechnoTypeClass***.

### Call_Back()
**Maintenance callback** for GlyphX.

### CC_Draw_Shape()
**Custom shape draw handler** - Hook for rendering.

## Inputs, outputs and side effects

**State mutations:**
- Game loop control (`InMainLoop`, `GameToPlay`)
- Palette fades (`Fade_Palette_To`)
- Network state (connect/disconnect)
- CD state (MIX file reloads)
- Recording file I/O
- Selection state (playback)
- Screen shake timers

**External systems affected:**
- All subsystems (init/uninit order)
- Network (DDE, IPX, Winsock)
- Audio (Theme, CD audio)
- Video (palette, resolution)
- Input (keyboard, mouse)
- Recording/playback

## Calls into / called by

**Called by:**
- OS/Launcher: `Main_Game()` entry point
- Game loop: `Main_Loop()`, `Keyboard_Process()`
- UI: `Handle_View`, `Handle_Team`
- Superweapons: `Shake_The_Screen`
- Recording: `Do_Record_Playback` per frame

**Calls into:**
- `Init_Game/Uninit_Game/Select_Game`
- `Main_Loop/Map_Edit_Loop`
- `Fade_Palette_To`
- `Keyboard::Clear`, `Show/Hide_Mouse`
- `Special_Dialog/Options.Process/Surrender_Dialog`
- Network: `Register_Game_Start_Time`, `Shutdown_Network`, `Winsock.Close`
- `DDEServer.Enable/Disable/Delete_MPlayer_Game_Info`
- `Spawn_WChat`
- `Theme.Stop/Queue_Song/Scan/What_Is_Playing`
- `MixFileClass` (MOVIES/GENERAL/SCORES.MIX)
- `Map.Set_Tactical_Position/Center_Map/Flag_To_Redraw/Render`
- `CurrentObject` selection
- `PlayerPtr->Credits/Blockage/Flag_To_Win`
- `Options.IsFreeScroll`
- `CCMessageBox.Process`
- `GetVolumeInformation/CreateFile` (CD)
- `RecordFile` I/O
- `CC_Draw_Shape`, `CCFileClass`, `FontPtr`

## Related original files
- **CONQUER.H** - Declarations, globals
- **GAME.H/.CPP** - Init_Game, Select_Game
- **MAINLOOP.H/.CPP** - Main_Loop
- **KEYBOARD.H/.CPP** - Keyboard_Process, Input_Num
- **MAP.H/.CPP** - MapClass
- **OPTIONS.H/.CPP** - OptionsClass
- **THEME.H/.CPP** - ThemeClass
- **MIXFILE.H/.CPP** - MixFileClass
- **NETWORK/IPX*.CPP** - Network
- **RECORD.H/.CPP** - RecordFile
- **SPECIAL.H/.CPP** - SpecialDialog, cheats
- **CCDDE.H/.CPP** - DDEServer
- **MOUSE.H/.CPP** - Mouse
- **PALETTE.H/.CPP** - Fade_Palette_To

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-launcher`

**Mapped modules:**
- `cnc-launcher/src/main.rs` - Main_Game entry point
- `cnc-tiberian-dawn/src/game_loop.rs` - Main_Loop
- `cnc-core/src/input/keyboard.rs` - Keyboard_Process
- `cnc-core/src/cd/cd_manager.rs` - Force_CD_Available
- `cnc-tiberian-dawn/src/recording.rs` - Do_Record_Playback
- `cnc-core/src/ui/view_bookmarks.rs` - Handle_View

**Not yet ported:**
- Full game state machine (Select_Game → Main_Loop → cleanup)
- DLL vs standalone branching (`RunningAsDLL`)
- Palette fade integration with renderer
- CD verification with volume labels + MIX reload
- Multiplayer network shutdown per GameType
- WChat spawn/return logic
- Demo mode flow
- Recording/playback with CRC sync
- Hi-res asset prefix (`H*`)
- Screen shake per-house dispatch
- Cheat key handling (Debug_Flag, Debug_Playtest)
- Team hotkeys (create/select/add)
- Tactical view bookmarks (10 slots)
- Color cycling palette animation
- Debug heap validation (CHEAT_KEYS)
- VQ movie playback pipeline
- DDE server for WChat integration

## Edge cases and compatibility notes

1. **DLL mode**: `RunningAsDLL` causes early return from `Main_Game` after `Select_Game`
2. **CD handling**: Modern path ignores CD index, always loads same 3 MIX files; legacy had per-CD MIX variants
3. **Recording CRC**: Sum of target IDs detects desync; mismatch → selects all playback objects
4. **SuperRecord**: Opens/closes file every frame for crash safety
5. **Playback rendering**: `Map.Render()` called explicitly (normal loop renders via main loop)
6. **View bookmarks**: Stored as cell coordinates, restored with `Map.Set_Tactical_Position`
7. **Team actions**: Modifiers (SHIFT/CTRL/ALT) map to add/create/select
8. **Surrender**: Only in MP or if not defeated
9. **Alliances**: Only MP or cheat mode
10. **Free scroll**: Cheat-only toggle
11. **Win cheat**: `W`+ALT sets `Blockage=false`, calls `Flag_To_Win()`
12. **Credit cheat**: `M`+modifiers adds 10000
13. **Special dialog**: `/` key (cheat) opens debug dialog
14. **Network stats**: `Register_Game_Start_Time/End_Time`, `Send_Statistics_Packet`
15. **Heap debug**: `Heap_Dump_Check` walks CRT heap (disabled in release)
16. **Resolution factor**: 0=320, 1=hi-res; affects asset loading via `Hires_Retrieve`
17. **Screen shake**: Multiplied by 2, per-human-house
18. **Color cycling**: Returns bool (handled in main loop)
19. **Mouse handling**: Hidden during playback, shown after
20. **Fade speeds**: `FADE_PALETTE_MEDIUM/SLOW`

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/CONQUER.CPP at f1f0d42
