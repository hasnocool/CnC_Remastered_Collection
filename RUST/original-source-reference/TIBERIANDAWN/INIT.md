# PATH/FILE
original/TIBERIANDAWN/INIT.CPP

## Purpose
Game initialization and main menu system for Tiberian Dawn. Handles one-time startup (heaps, MIX files, fonts, palettes, CD detection), command-line parsing with obfuscated cheat codes, recording/playback serialization, and the main menu loop (`Select_Game`).

## Where it fits in the game
Entry point subsystem - called by `Main_Game()` once at startup. Sets up all global state, loads assets, validates CD, plays intro movies, then enters `Select_Game()` loop for menu navigation.

## Classes, structs, enums and important data

### Heap initialization (TFixedIHeapClass)
Called once at startup with compile-time max constants:
| Pool | Max Constant |
|------|-------------|
| Units | UNIT_MAX |
| Factories | FACTORY_MAX |
| Terrains | TERRAIN_MAX |
| Templates | TEMPLATE_MAX |
| Smudges | SMUDGE_MAX |
| Overlays | OVERLAY_MAX |
| Infantry | INFANTRY_MAX |
| Bullets | BULLET_MAX |
| Buildings | BUILDING_MAX |
| Anims | ANIM_MAX |
| Aircraft | AIRCRAFT_MAX |
| Triggers | TRIGGER_MAX |
| TeamTypes | TEAMTYPE_MAX |
| Teams | TEAM_MAX |
| Houses | HOUSE_MAX |

### MIX file registration order
1. `CCLOCAL.MIX` / `UPDATE.MIX` / `UPDATA.MIX` / `UPDATEC.MIX` (cached, local data)
2. `LANGUAGE.MIX` (Japanese)
3. Addon MIX: `SC*.MIX` / `SS*.MIX` (scanned from search paths)
4. `CONQUER.MIX` / `TRANSIT.MIX` (cached)
5. `GENERAL.MIX` (never cached)
6. `MOVIES.MIX` (never cached)
7. `SCORES.MIX` (music, triggers ThemeClass::Scan)
8. `SPEECH.MIX` / `SOUNDS.MIX` / `ZOUNDS.MIX` (juvenile)
9. `DEMO.MIX` / `DEMOL.MIX` / `DEMOM.MIX` (demo build)

### Command-line parameters (obfuscated via `Obfuscate()`)
| Parameter | Effect |
|-----------|--------|
| `-EASY` / `-HARD` | Difficulty flags |
| `-PLAYTEST` / `-CHEATERIK` / `-CHEATADAM` / `-CHEATMIKE` / `-CHEATDAVID` / `-CHEATPHIL` / `-CHEATBILL` / `-CHEAT_STEVET` | `Debug_Playtest=true`, `Debug_Flag=true` |
| `-EDITORBILL` / `-EDITORERIK` | `Debug_Map=true`, `Debug_Unshroud=true`, `Debug_Flag=true` |
| `-SPECIAL` | `Special.IsJurassic=true`, `AreThingiesEnabled=true` |
| `-INSTALL` | `Special.IsFromInstall=true` |
| `-O` / `-0` | `IsV107=true` (v1.07 compat) |
| `-CD<path>` | `CCFileClass::Set_Search_Drives()` |
| `-ENGLISH` (Japanese) | `ForceEnglish=true` |
| `-DESTNET<xx.xx.xx.xx>` | Bridge network address |
| `-SOCKET<id>` | IPX socket offset from 0x4000 |
| `-STEALTH` | `NetStealth=true` |
| `-MESSAGES` | `NetProtect=false` |
| `-ATTRACT` | `AllowAttract=true` |
| `-480` | `ScreenHeight=480` |
| `-WCHAT` | `SpawnedFromWChat=true` |
| `-MMX` | `MMXAvailable=true` |
| `-HANSOLO` | `MPlayerSolo=true` |
| `-SEED<num>` | `CustomSeed` for RNG |
| `-X<flags>` | Special flags: `M`=mono, `I`=inert, `H`=speed build, `S`=super-record, `X`=record, `Y`=playback, `Z`=jurassic, `Q`=quiet, `V`=visible target |

### Recording serialization
`Save_Recording_Values()` / `Load_Recording_Values()` serialize entire MP game state:
- Game type, build level, player name/color/house
- Player count, bases, credits, tiberium, goodies, ghosts, unit limit
- `MPlayerID`, `MPlayerHouses`, `Seed`, `Scenario`, `ScenPlayer`, `ScenDir`, `Whom`
- `SpecialClass`, `GameOptionsClass`, `FrameSendRate`, `CommProtocol`
- `SuperRecord` reopens file per write for crash safety

### Version numbering
`Version_Number()` returns compile-date-based version (WIN32: localized .01/.02/.07; PATCH: .34/.14; Demo: 1.0a). Reads `VERSION.TXT` for `VersionText` display.

## Functions and methods

### Init_Game(argc, argv) → bool
**One-time startup sequence:**

1. `LoadLibrary("reslib.dll")`
2. **Heap init**: All 17 pools `Set_Heap(max)`
3. `memset(Waypoint, 0xFF)` - Invalid waypoints
4. `Keyboard::Clear()`, `Kbd.Clear()`
5. `Set_Shape_Buffer(new[SHAPE_BUFFER_SIZE])`
6. **MIX registration** (see order above)
7. **Font loading**: `Load_Alloc_Data` (FNT files) + `MixFileClass::Retrieve`
8. **Palette alloc**: `BlackPalette`/`GamePalette`/`OriginalPalette`/`WhitePalette` (768 bytes each, `MEM_CLEAR|MEM_REAL`)
9. `Set_Palette(BlackPalette)`, `VisiblePage.Clear()`, `Set_Palette(GamePalette)`
10. **Mouse cursor**: `MixFileClass::Retrieve("MOUSE.SHP")`, `Set_Mouse_Cursor`, `Show_Mouse()`
11. **Focus wait**: `Keyboard::Check()` loop until `GameInFocus`
12. **Language text**: `RawFileClass(Language_Name("CONQUER"))` → `Load_Alloc_Data` or `MixFileClass::Retrieve`
13. **Default palette**: `CCFileClass("TEMPERAT.PAL").Read(GamePalette, 768)`
14. **Mouse check**: Fail if `!MouseInstalled`
15. **Addon MIX scan**: `FindFirstFile("SC*.MIX"/"SS*.MIX")` on search paths
16. `Force_CD_Available(RequiredCD)` (loads GENERAL/MOVIES/SCORES.MIX)
17. `Anim_Init()`
18. `Play_Intro(true)` (unless install/WChat)
19. `Load_Title_Screen("HTITLE.PCX")`, palette swap via `Wait_Vert_Blank()`
20. **Cache main MIX**: `MixFileClass::Cache("CONQUER.MIX")`, `SOUNDS.MIX`, `ZOUNDS.MIX`
21. `Mono_Clear_Screen()`, `MonoClass::Enable()` if mono debug
22. Returns true

### Parse_Command_Line(argc, argv) → bool
**Processes argv[1..argc-1]:** Each arg → `Obfuscate(string)` → switch on result. Handles `-X<flags>` multi-char. Unknown → usage + false.

### Obfuscate(string) → long
**One-way CRC-based obfuscation** for cheat codes:
1. Copy to buffer, uppercase, sanitize non-graph chars
2. Pad to 16+ chars, multiple of 4
3. `code = CRC(buffer)`; `copy = code`
4. `strrev(buffer)`, `code ^= CRC(buffer)`
5. `code ^= copy`
6. Per-char: `code ^= buffer[i]`, rotate, `buffer[i] ^= temp`
7. Loss/add bit masks per position
8. 4-char block cipher (Feistel-like with self-key)
9. Final `CRC(buffer)` returned

### Select_Game(fade) → bool
**Main menu loop** - Returns false to exit game.
- `fade`: Palette fade on entry
- Handles: Single player (GDI/NOD), Multiplayer, Options, Credits, Exit
- Calls `Play_Intro(false)` for return visits

### Play_Intro(for_real)
**Plays VQ movies:** `IntroMovie` → `BriefMovie` → loops attract mode (`ATTRACT_MODE_TIMEOUT`)

### Anim_Init()
**Initializes VQ animation controller** from `PLAYER.INI`

### Load/Save_Recording_Values()
**Full MP state serialization** for record/playback (see data list above)

### Version_Number() → int
**Compile-date version** with localized suffix and `VERSION.TXT` overlay

## Inputs, outputs and side effects

**State mutations:**
- All 17 heaps allocated
- Global palettes/fonts allocated
- MIX files registered/cached
- CD drive detected/verified
- Mouse cursor set
- Language strings loaded
- `GameInFocus` waited on
- Intro movies played
- Main MIX cached (slow)
- `VersionText` set

**External systems affected:**
- File system (search paths, CD)
- Renderer (palettes, fonts, pages)
- Audio (ThemeClass::Scan)
- Network (IPX socket, bridge)
- Input (keyboard, mouse)
- Recording (RECORD.BIN)

## Calls into / called by

**Called by:**
- `Main_Game()` - once at startup

**Calls into:**
- `LoadLibrary` (reslib.dll)
- `TFixedIHeapClass::Set_Heap`
- `Keyboard::Clear`, `Kbd.Clear`
- `Set_Shape_Buffer`
- `MixFileClass` constructor, `Cache`, `Retrieve`
- `Load_Alloc_Data` (CCFileClass)
- `Set_Palette`, `VisiblePage.Clear`, `Wait_Vert_Blank`
- `Extract_Shape`, `Set_Mouse_Cursor`, `Show_Mouse`
- `Keyboard::Check`
- `RawFileClass`, `Language_Name`
- `CCFileClass::Read`, `Set_Search_Drives`, `Get_Search_Path`
- `FindFirstFile`/`FindNextFile` (addon MIX scan)
- `Force_CD_Available`
- `Anim_Init`
- `Play_Intro`, `Load_Title_Screen`, `Blit_Hid_Page_To_Seen_Buff`
- `ThemeClass::Scan`
- `Mono_Clear_Screen`, `MonoClass::Enable`
- `Parse_Command_Line`
- `Obfuscate`
- `Select_Game`
- `RecordFile` I/O
- `Read_MultiPlayer_Settings`
- `Calculate_CRC`

## Related original files
- **INIT.H** - Declarations
- **FUNCTION.H** - Global externs
- **HEAP.H** - TFixedIHeapClass
- **MIXFILE.H** - MixFileClass
- **CCFILE.H** - CCFileClass
- **PALETTE.H** - Set_Palette, palettes
- **MOUSE.H** - Mouse cursor
- **KEYBOARD.H** - Keyboard
- **ANIM.H** - Anim_Init
- **THEME.H** - ThemeClass
- **TITLE.H** - Load_Title_Screen
- **MONO.H** - MonoClass
- **VERSION.H** - Version_Number
- **RECORD.H** - RecordFile
- **MULTI.H** - MP state
- **SPECIAL.H** - SpecialClass
- **OPTIONS.H** - GameOptionsClass
- **CRC.H** - Calculate_CRC

## Rust port mapping
**Crates:** `cnc-launcher`, `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-launcher/src/init.rs` - Init_Game startup sequence
- `cnc-launcher/src/cli.rs` - Parse_Command_Line, Obfuscate
- `cnc-core/src/assets/mix.rs` - MIX file registration/caching
- `cnc-core/src/assets/font.rs` - Font loading
- `cnc-launcher/src/menu.rs` - Select_Game main menu
- `cnc-tiberian-dawn/src/recording.rs` - Save/Load_Recording_Values
- `cnc-core/src/version.rs` - Version_Number

**Not yet ported:**
- One-time global heap initialization
- MIX file scanning (SC*/SS* wildcards)
- CD detection with `Force_CD_Available`
- Palette management (MEM_CLEAR|MEM_REAL alloc)
- VQ movie playback (`Play_Intro`, `Anim_Init`)
- Vertical blank synchronization
- Main MIX caching (async/progress)
- Monochrome debug console
- Obfuscate cheat code system
- Full MP state serialization for recording
- Localized version numbering
- Focus wait loop
- Mouse driver detection

## Edge cases and compatibility notes

1. **Init order critical**: Heaps → Shape buffer → MIX → Fonts → Palettes → Mouse → Language → CD → Anim → Intro → Title → Cache
2. **RequiredCD**: -1=any CD, -2=hard drive, 0=GDI, 1=Nod, 2=Covert
3. **DEMO build**: Different MIX set (DEMOL.MIX, DEMO.MIX, DEMOM.MIX)
4. **Japanese build**: Extra LANGUAGE.MIX, ForceEnglish flag
5. **PATCH build**: Different version numbering
6. **SuperRecord**: Reopens RECORD.BIN per frame for crash recovery
7. **Obfuscate**: Designed to resist reverse-engineering of cheat codes
8. **Addon MIX scan**: Only SC*.MIX and SS*.MIX patterns
9. **Focus wait**: Blocks until window active (prevents background launch issues)
10. **Palette swap**: `Wait_Vert_Blank()` avoids tearing during title screen
11. **Language fallback**: Raw file first, then MIX
12. **Mouse failure**: Fatal error with localized message
13. **WChat spawn**: Sets `Special.IsFromWChat`, skips intro
14. **Install mode**: `Special.IsFromInstall`, skips intro/title
15. **VersionText**: Displayed on options dialog, from VERSION.TXT
16. **Recording includes SpecialClass + GameOptionsClass**: Full state capture

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/INIT.CPP at f1f0d42
