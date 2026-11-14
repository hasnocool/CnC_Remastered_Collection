# PATH/FILE
original/TIBERIANDAWN/STARTUP.CPP

## Purpose
Application entry point and initialization for Tiberian Dawn. Handles command-line parsing, Windows/DLL setup, video/audio initialization, INI configuration, and main game loop invocation. Supports both standalone EXE and GlyphX DLL modes.

## Where it fits in the game
Program bootstrap - First code executed. Sets up all library systems (video, audio, input, timers), reads configuration, creates main window, then calls `Main_Game()`. Also implements `DllMain` for GlyphX client hosting.

## Classes, structs, enums and important data

### Global state
| Variable | Purpose |
|----------|---------|
| `RunningAsDLL` (bool) | True when hosted by GlyphX |
| `ProgramInstance` (HINSTANCE) | Module handle |
| `WindowsTimer` (WinTimerClass*) | 60Hz system timer |
| `Palette` (unsigned char[768]) | Game palette buffer |
| `ScreenWidth`/`ScreenHeight` | Virtual resolution (320/1536) |
| `VisiblePage`/`HiddenPage` | Double-buffer surfaces |
| `SeenBuff`/`HidPage` | 1536×1536 viewports |
| `MouseInstalled` (bool) | Mouse driver detected |
| `MMXAvailable` (bool) | CPU MMX support |
| `ProgEndCalled` (bool) | Cleanup guard |
| `SpawnedFromWChat` (bool) | Launched from WChat lobby |
| `Special.IsFromInstall` (bool) | First-run intro flag |

### Command-line parsing
- `argc`/`argv[20]` - Parsed from `command_line_in`
- Supports quoted arguments
- Passed to `Parse_Command_Line()` (INIT.CPP)

### Video modes
| Mode | Resolution | Bits |
|------|------------|------|
| Primary | 640×400 | 8-bit |
| Fallback | 640×480 | 8-bit |
| Virtual | 1536×1536 | Viewport |

### INI settings read (CONQUER.INI)
| Section/Key | Default | Purpose |
|-------------|---------|---------|
| `Intro/PlayIntro` | "Yes" | First-run intro |
| `Options/SlowPalette` | 1 | Palette fade speed |
| `Options/VideoBackBuffer` | 1 | Back buffer in VRAM |
| `Options/HardwareFills` | 1 | Hardware blit fills |
| `Options/Resolution` | 0 | 1536 (virtual) |
| `Options/Compatibility` | 0 | v1.07 compat |
| `Options/Socket` | 0 | IPX socket offset |
| `Options/DestNet` | none | Bridge network |
| `CD/Path` (DEMO) | "." | CD search path |

## Functions and methods

### DllMain(instance, fdwReason, lpvReserved) → BOOL
**DLL entry point** for GlyphX hosting.

| Reason | Action |
|--------|--------|
| `DLL_PROCESS_ATTACH` | Stores `ProgramInstance` |
| `DLL_PROCESS_DETACH` | Deletes `WindowsTimer`, calls `DLL_Shutdown()`, `MixFileClass::Free_All()`, `Uninit_Game()` |
| `DLL_THREAD_ATTACH/DETACH` | No-op |

### DLL_Startup(command_line_in) → int
**DLL entry for GlyphX** - Equivalent to `WinMain`.

1. `RunningAsDLL = true`
2. Parses command line → `argc`/`argv`
3. Single-instance check: `CC95AlreadyRunning` (via DDE) → activates existing window, exits
4. RAM check: `Ram_Free(MEM_NORMAL) < 5MB` → fail
5. `Parse_Command_Line(argc, argv)` - Handles cheats, network, etc.
6. Creates `WindowsTimer(60Hz)`
7. Reads `CONQUER.INI` → `NewConfig`, `Read_Setup_Options()`
8. Creates main window (`Create_Main_Window`)
9. `Audio_Init()` → `SoundOn`
10. Allocates `Palette[768]`
11. **Video mode setup**:
    - Tries `ScreenHeight` (400/480) at 8-bit
    - Initializes `VisiblePage`/`HiddenPage` (VRAM or system mem)
    - Attaches `SeenBuff`/`HidPage` viewports (1536×1536)
12. `Options.Adjust_Variables_For_Resolution()`
13. `MMXAvailable` → `Init_MMX()`
14. Creates `WWMouseClass`
13. Re-reads INI for intro/SlowPalette
14. DDE check for WChat spawn (`C&CSPAWN.INI`)
15. `Main_Game(argc, argv)`
16. Cleanup on exit

### Prog_End(why, fatal)
**Graceful shutdown** - Called on exit.

1. `GlyphX_Debug_Print("Prog_End")`
2. If `fatal`: `*((int*)0)=0` (crash)
3. Modem/NullModem: `NullModem.Change_IRQ_Priority(0)`
4. `Sound_End()`
5. Deletes `WWMouse`, `WindowsTimer`, `Palette`
6. `ProgEndCalled = true`

### Delete_Swap_Files()
**Cleans *.SWP files** - Disabled (`#if 0`).

### Print_Error_End_Exit / Print_Error_Exit
**Error handlers** - Print message, `Get_Key()`, `Prog_End()`, `exit(1)`.

### Read_Setup_Options(config_file)
**Reads early INI settings** before full init.

- `VideoBackBufferAllowed`, `AllowHardwareBlitFills`
- `ScreenHeight` (Resolution)
- `IsV107` (Compatibility)
- `Socket` → `Ipx.Set_Socket(socket + 0x4000)`
- `DestNet` → parses `xx.xx.xx.xx` → `BridgeNet`, `IsBridge=1`

## Inputs, outputs and side effects

**State mutations:**
- All library systems initialized
- Video mode set (DirectDraw surfaces)
- Audio device opened
- Palette allocated
- Mouse/keyboard/timer created
- INI parsed
- Single-instance enforced

**External systems affected:**
- Windows (window creation, DirectDraw)
- Audio (DirectSound)
- Network (IPX/Winsock init)
- DDE (WChat communication)
- File system (INI read/write)

## Calls into / called by

**Called by:**
- OS loader (`WinMain`/`DllMain`)
- GlyphX client (DLL mode)

**Calls into:**
- `Parse_Command_Line` (INIT.CPP)
- `Create_Main_Window` (platform)
- `Audio_Init` / `Sound_End` (SOUND.CPP)
- `Set_Video_Mode` / `VisiblePage.Init` (VIDEO.CPP)
- `MixFileClass::Free_All` (MIXFILE.CPP)
- `Init_MMX` (MMX detection)
- `WWMouseClass` (MOUSE.CPP)
- `DDEServer.Get_MPlayer_Game_Info` (CCDDE.CPP)
- `Main_Game` (CONQUER.CPP)
- `Uninit_Game` (GAME.CPP)
- `Ram_Free`, `Disk_Space_Available` (CONQUER.CPP)

## Related original files
- **STARTUP.H** - Declarations
- **INIT.CPP** - Parse_Command_Line, Main_Game
- **GAME.CPP** - Init_Game, Uninit_Game
- **SOUND.H/.CPP** - Audio_Init, Sound_End
- **VIDEO.H/.CPP** - Set_Video_Mode, GraphicBufferClass
- **MOUSE.H/.CPP** - WWMouseClass
- **TIMER.H** - WinTimerClass
- **CCDDE.H/.CPP** - DDEServer
- **MIXFILE.H/.CPP** - MixFileClass
- **OPTIONS.H/.CPP** - OptionsClass
- **MMX.H** - Detect_MMX_Availability, Init_MMX
- **CONQUER.CPP** - Main_Game

## Rust port mapping
**Crates:** `cnc-launcher`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-launcher/src/main.rs` - DLL_Startup/WinMain entry
- `cnc-launcher/src/video.rs` - Video mode setup
- `cnc-launcher/src/audio.rs` - Audio_Init
- `cnc-launcher/src/config/ini.rs` - Read_Setup_Options
- `cnc-core/src/platform/windows.rs` - DllMain, WinTimerClass
- `cnc-core/src/single_instance.rs` - CC95AlreadyRunning check

**Not yet ported:**
- DirectDraw surface management (VRAM vs system memory)
- 1536×1536 virtual viewport with 2X scaling
- MMX detection/initialization
- DDE-based single-instance + WChat spawn
- CONQUER.INI early parsing (before full init)
- Palette allocation (MEM_CLEAR)
- GlyphX DLL hosting (DllMain PROCESS_DETACH cleanup)
- Command-line parsing with quote support
- Fallback video mode (400→480)
- Hardware blit capability detection

## Edge cases and compatibility notes

1. **DLL vs EXE**: `RunningAsDLL` changes flow (early return, no `Prog_End` cleanup)
2. **Single instance**: DDE server detects existing instance, activates window
3. **WChat spawn**: `SpawnedFromWChat` skips intro, uses `C&CSPAWN.INI` packet
4. **VRAM allocation**: Tries VRAM for `HiddenPage`; falls back to system RAM if no blitter/low VRAM
5. **ScreenHeight logic**: INI "Resolution" sets 1536 (virtual); actual window 400/480
6. **MMX**: Detected at startup, initialized before game
7. **INI write-back**: `Intro/PlayIntro` set to "No" after first run
8. **BreakoutAllowed**: False during install intro (can't ESC)
9. **Error handlers**: `Memory_Error` points to handler; `Prog_End` on fatal
10. **Swap files**: `*.SWP` cleanup disabled
11. **DDE shutdown**: `DLL_Shutdown()` called on DLL detach
12. **IPX socket**: `Options/Socket` adds 0x4000 base
13. **Bridge networking**: `Options/DestNet` parses hex IPX address

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/STARTUP.CPP at f1f0d42
