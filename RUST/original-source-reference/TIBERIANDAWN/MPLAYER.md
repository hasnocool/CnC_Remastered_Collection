# PATH/FILE
original/TIBERIANDAWN/MPLAYER.CPP

## Purpose
Multiplayer menu and settings system for Tiberian Dawn. Handles game type selection (modem/serial, IPX, Internet), INI persistence for player preferences, scenario discovery, computer chat messages, and surrender dialog.

## Where it fits in the game
Multiplayer frontend - called from `Select_Game()` (INIT.CPP) when player chooses multiplayer. Manages connection setup, settings persistence, and in-game MP UI (surrender, chat).

## Classes, structs, enums and important data

### Multiplayer settings (persisted in CONQUER.INI)
| Variable | INI Key | Purpose |
|----------|---------|---------|
| `MPlayerName` | `Handle` | Player display name |
| `MPlayerPrefColor` | `Color` | Preferred color index (0-5) |
| `MPlayerHouse` | `Side` | GDI/NOD preference |
| `CurPhoneIdx` | `PhoneIndex` | Selected modem entry |
| `SerialDefaults` | `SerialDefaults/*` | Modem port, IRQ, baud, init string |
| `InitStrings[]` | `InitStrings/###` | Modem init strings |
| `PhoneBook[]` | `PhoneBook/###` | Name\|Number\|Port\|IRQ\|Baud\|... |
| `TrapFrame/Type/Coord/This/Cell` | `SyncBug/*` | Debug sync recording |

### GameType enum (returned by Select_MPlayer_Game)
- `GAME_NORMAL` - Cancel/single player
- `GAME_MODEM` / `GAME_NULL_MODEM` - Serial modem
- `GAME_IPX` - IPX LAN
- `GAME_INTERNET` - TCP/IP (Winsock)

### Scenario discovery
- `MPlayerFilenum` (DynamicVector<int>) - Scenario numbers with .INI files
- `MPlayerScenarios` (DynamicVector<char*>) - Descriptions from `[Basic] Name=`
- `MPlayerDescriptions[100][40]` - Description buffer

### MP game options (set in lobby)
| Variable | Purpose |
|----------|---------|
| `MPlayerBases` | Production enabled |
| `MPlayerCredits` | Starting credits |
| `MPlayerTiberium` | Tiberium enabled |
| `MPlayerGoodies` | Crates enabled |
| `MPlayerGhosts` | AI houses without players |
| `MPlayerSolo` | Allow 1-player net game |
| `MPlayerUnitCount` | Unit limit (non-base) |
| `MPlayerCountMin/Max[2]` | Min/max players (base off/on) |

### Computer chat
- `Messages` (MessageListClass) - Chat message queue
- `LastMessage[MAX_MESSAGE_LENGTH]` - Last human message (for garbling)
- `MPlayerTColors[6]` - Remap colors for AI messages

## Functions and methods

### Select_MPlayer_Game() → GameType
**Main multiplayer menu** - Dialog with buttons.

**Buttons:**
1. `BUTTON_MODEMSERIAL` (TXT_MODEM_SERIAL) → `Select_Serial_Dialog()`
2. `BUTTON_IPX` (TXT_NETWORK) → Returns `GAME_IPX`
3. `BUTTON_CANCEL` (TXT_CANCEL) → Returns `GAME_NORMAL`

**Disabled (compiled out):**
- Internet/Winsock (`BUTTON_INTERNET`, `FORCE_WINSOCK` block)
- Internet host/join flow with `Winsock.Init/Start_Server/Start_Client`

**UI flow:**
- Loads `HTITLE.PCX` background
- `Dialog_Box` + `Draw_Caption` + `TextButtonClass` list
- Keyboard navigation (UP/DOWN/RETURN/ESC)
- Mouse click handling via `commands->Input()`

### Read_MultiPlayer_Settings()
**Loads all MP settings from CONQUER.INI** using `_ShapeBuffer` as staging.

**Sections parsed:**
1. `[MultiPlayer]` - Handle, Color, Side, PhoneIndex, CheckHeap
2. `[SerialDefaults]` - ModemName, Port(hex), IRQ, Baud, Init, DialMethod, Compression, ErrorCorrection, HardwareFlowControl, CallWaitString, CallWaitStringIndex, InitStringIndex
3. `[InitStrings]` - All entries (NULL-terminated list via `WWGetPrivateProfileString(NULL)`)
4. `[PhoneBook]` - Entries: `Name|Number|Port|IRQ|Baud|Compression|ErrorCorrection|HWFlow|DialMethod|InitIdx|CallWaitIdx|CallWaitStr`
5. `[SyncBug]` - Frame, Type(RTTI), Coord, this, Cell (for playback debug)

**PhoneBook parsing:** Tokenizes by `|`, maps dial method string to enum

### Write_MultiPlayer_Settings()
**Saves all MP settings to CONQUER.INI** - Clears sections first, writes in reverse order for correct INI ordering.

### Read_Scenario_Descriptions()
**Scans for multiplayer scenarios** (SCEN_PLAYER_MPLAYER, SCEN_DIR_EAST, SCEN_VAR_A).

1. Loops 0-99: `Set_Scenario_Name`, checks `fname.INI` availability
2. Adds available numbers to `MPlayerFilenum`
3. For each: reads `[Basic] Name=` into `MPlayerDescriptions`, adds to `MPlayerScenarios`

### Free_Scenario_Descriptions()
**Clears** `MPlayerScenarios` and `MPlayerFilenum` (DynamicVector::Clear)

### Computer_Message()
**AI chat message** - 25% chance per AI house per call.

1. Finds first non-human, non-defeated house ≥ `HOUSE_MULTI1`
2. 25% chance: echoes `LastMessage` (33% garbled via `Garble_Message`)
3. 75% chance: random `TXT_COMP_MSG1..TXT_COMP_MSG12`
4. Adds to `Messages` with house's remap color (`MPlayerTColors[RemapColor]`)

### Garble_Message(buf)
**Shuffles words** in message using `Sim_IRandom` (sync-safe RNG).

1. Strips trailing punctuation
2. Tokenizes by spaces into `words[40]`
3. Fisher-Yates shuffle using `Sim_IRandom(0, numwords)`
4. Rebuilds buffer, restores punctuation

### Surrender_Dialog() → int (0=cancel, 1=surrender)
**Confirmation dialog** with OK/Cancel buttons.

- Centers on screen (factor for hi-res)
- `Dialog_Box` + caption + `Fancy_Text_Print(TXT_SURRENDER)`
- `Main_Loop()` runs in background (allows game to continue)
- Returns 1 on OK/ENTER, 0 on ESC/Cancel
- On exit: `HiddenPage.Clear()`, `Map.Flag_To_Redraw(true)`, `Map.Render()`

## Inputs, outputs and side effects

**State mutations:**
- MP settings globals (`MPlayer*`, `SerialDefaults`, `InitStrings`, `PhoneBook`)
- `CONQUER.INI` file
- Scenario lists (`MPlayerScenarios`, `MPlayerFilenum`)
- Chat messages (`Messages`, `LastMessage`)
- Game type selection (`GameToPlay`, `ModemGameToPlay`)

**External systems affected:**
- Network setup (IPX, Winsock, NullModem)
- Scenario loading
- In-game chat UI
- Surrender logic (adds DESTRUCT event)

## Calls into / called by

**Called by:**
- `Select_Game()` → `Select_MPlayer_Game()`
- `Main_Game()` cleanup → `Free_Scenario_Descriptions()`
- Recording → `Save/Load_Recording_Values()` (calls `Read_MultiPlayer_Settings`)
- In-game: `Computer_Message()` periodically, `Surrender_Dialog()` on 'R' key
- MP lobby: `Read/Write_MultiPlayer_Settings()`

**Calls into:**
- `Select_Serial_Dialog()` (modem setup)
- `CCFileClass` (INI file I/O)
- `WWGetPrivateProfileString/Int`, `WWWritePrivateProfileString/Int`
- `Set_Scenario_Name` (scenario naming convention)
- `DynamicVectorClass::Add/Clear`
- `MessageListClass::Add_Message`
- `TextButtonClass`, `ControlClass`, `Dialog_Box`, `Draw_Caption`
- `Fancy_Text_Print`, `Text_String`
- `Main_Loop()` (background in surrender dialog)
- `Sim_IRandom` (sync-safe RNG)
- `IRandom` (game RNG)
- `Map.Render`, `Map.Flag_To_Redraw`

## Related original files
- **MPLAYER.H** - Declarations
- **INIT.CPP** - `Select_Game` calls `Select_MPlayer_Game`
- **CONQUER.CPP** - `Main_Game` cleanup, recording serialization
- **TCP/IP** - Winsock (disabled)
- **MODEM.H/.CPP** - `Select_Serial_Dialog`, `SerialDefaults`
- **MESSAGES.H/.CPP** - `MessageListClass`
- **MULTI.H** - MP globals (`MPlayer*`, `MPlayerTColors`)
- **SYNC.H** - `Sim_IRandom`
- **KEYBOARD.H** - `Keyboard::Clear`, input codes

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-net`, `cnc-launcher`

**Mapped modules:**
- `cnc-launcher/src/mp_menu.rs` - Select_MPlayer_Game dialog
- `cnc-net/src/mp_settings.rs` - Read/Write_MultiPlayer_Settings
- `cnc-tiberian-dawn/src/mp/scenario_browser.rs` - Read_Scenario_Descriptions
- `cnc-net/src/chat.rs` - Computer_Message, Garble_Message
- `cnc-ui/src/dialogs/surrender.rs` - Surrender_Dialog

**Not yet ported:**
- INI-based settings persistence (CONQUER.INI)
- Modem/serial configuration (PhoneBook, InitStrings, SerialDefaults)
- IPX availability detection (`Ipx.Is_IPX()`)
- Scenario scanning with naming convention
- Sync-safe RNG (`Sim_IRandom`) for garbling
- Background `Main_Loop()` in modal dialog
- Winsock/Internet code (compiled out in remaster)
- Hi-res dialog scaling (factor = width/320)

## Edge cases and compatibility notes

1. **Internet/Winsock disabled**: Large `#if 0` blocks; `FORCE_WINSOCK` not defined in remaster
2. **Modem/serial**: Only `GAME_MODEM`/`GAME_NULL_MODEM` via `Select_Serial_Dialog`
3. **Scenario naming**: `Set_Scenario_Name(name, num, SCEN_PLAYER_MPLAYER, SCEN_DIR_EAST, SCEN_VAR_A)` → `MP##EA.INI`
4. **PhoneBook format**: `Name|Number|Port|IRQ|Baud|Comp|ErrCorr|HWFlow|DialMethod|InitIdx|CallWaitIdx|CallWaitStr`
5. **DialMethod mapping**: String → enum via `DialMethodCheck[]` (`T`/`P`)
6. **CallWaitString**: Predefined (`*70,`, `70#,`, `1170,`) or custom
7. **Computer message rate**: Called periodically; 1 AI house per call
8. **Garbling uses Sim_IRandom**: Sync-safe for MP playback
9. **Surrender dialog runs Main_Loop**: Game continues in background
10. **HIRES scaling**: `factor = (SeenBuff.Get_Width() == 320) ? 1 : 2`
11. **CheckHeap debug**: `TrapCheckHeap` from INI
12. **WChat spawn**: `Special.IsFromWChat` skips handle/color load
13. **SyncBug section**: Only read during `PlaybackGame` for debug trapping

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/MPLAYER.CPP at f1f0d42
