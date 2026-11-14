# PATH/FILE
original/TIBERIANDAWN/GOPTIONS.CPP

## Purpose
Main options menu dialog for Tiberian Dawn. Provides the pause/options screen with Load, Save, Delete, Game Controls, Quit, Resume, and Restate Mission buttons. Handles player input during gameplay pause. Also contains `Draw_Caption()` utility for drawing dialog captions with decorative filigree. Core to the in-game pause menu system.

## Where it fits in the game
UI/pause menu - the options dialog is invoked when player pauses during gameplay or from main menu. Integrated with `GAMEDLG.CPP` (game controls), `VISUDLG.CPP` (visual controls), `SOUNDDLG.CPP` (sound controls), `LOADDLG.CPP` (load/save), `CONFDLG.CPP` (confirmation), `DESCDLG.CPP` (description). Called from `LOGIC.CPP` when game is paused. Uses `EventClass` for multiplayer-safe game speed changes.

## Classes, structs, enums and important data

### Button constants (_constants[] array)
| ID | Text | Multiplay? | Purpose |
|----|------|-----------|---------|
| BUTTON_LOAD | TXT_LOAD_MISSION | false | Load saved game |
| BUTTON_SAVE | TXT_SAVE_MISSION | false | Save game |
| BUTTON_DELETE | TXT_DELETE_MISSION | true | Delete save / Resign |
| BUTTON_GAME | TXT_GAME_CONTROLS | true | Game controls dialog |
| BUTTON_QUIT | TXT_QUIT_MISSION | true | Quit to main menu |
| BUTTON_RESUME | TXT_RESUME_MISSION | true | Resume gameplay |
| BUTTON_RESTATE | TXT_RESTATE_MISSION | false | Restate mission objectives |

**BUTTON_COUNT** = total number of buttons
**BUTTON_LOAD** through **BUTTON_RESUME** range

### GameOptionsClass::Process() → void
**Main options dialog processing loop:**

**Dialog geometry** (scaled by `resfactor`):
| Field | Value |
|-------|-------|
| `OptionWidth` | (216+8) * factor |
| `OptionHeight` | 100 * factor |
| `OptionX` | (screen_w - OptionWidth) / 2 |
| `OptionY` | (screen_h - OptionHeight) / 2 |
| `ButtonWidth` | 130 * factor |
| `OButtonHeight` | 9 * factor |
| `CaptionYPos` | 5 * factor |
| `ButtonY` | 21 * factor |
| `Border1Len` | 72 * factor |
| `Border2Len` | 16 * factor |
| `ButtonResumeY` | OptionHeight - (15 * factor) |

**factor**: 1 for 320x200, 2 for 640x480

### Button layout logic
- Buttons 0-4 (index < 5): positioned at `y = (screen_h - OptionHeight)/2 + ButtonY + (OButtonHeight+2) * index`
- Buttons 5+: positioned at `y = OptionY + ButtonResumeY`
- All buttons forced to `MAX(maxwidth, 90 * resfactor)` width
- French version: Resume button width = 104 * resfactor
- Other versions: Resume button width = 90 * resfactor
- Restate button: right-aligned in normal game mode

### Input handling
| Key | Action |
|-----|--------|
| `BUTTON_LOAD | KN_BUTTON` | Load game |
| `BUTTON_SAVE | KN_BUTTON` | Save game |
| `BUTTON_DELETE | KN_BUTTON` | Delete / Resign |
| `BUTTON_QUIT | KN_BUTTON` | Quit confirmation |
| `BUTTON_GAME | KN_BUTTON` | Game controls dialog |
| `KN_ESC` / `BUTTON_RESUME | KN_BUTTON` | Resume game |
| `KN_UP` / `KN_DOWN` | Cycle buttons |
| `KN_RETURN` | Activate selected button |

### Sub-dialog actions
| Selection | Action |
|-----------|--------|
| `BUTTON_RESTATE` | `Restate_Mission()` → may play movie, exit dialog |
| `BUTTON_LOAD` | `LoadOptionsClass::LOAD().Process()` → may exit |
| `BUTTON_SAVE` | `LoadOptionsClass::SAVE().Process()` |
| `BUTTON_DELETE` | If multiplayer: `Surrender_Dialog()` → `EventClass::DESTRUCT`; else delete save |
| `BUTTON_QUIT` | Confirmation dialog → exit/restart |
| `BUTTON_GAME` | `GameControlsClass().Process()` → restart main loop |
| `BUTTON_RESUME` | Close dialog, return to game |

### Draw_Caption(text, x, y, w) → void
**Draw dialog caption with filigree:**

1. Map text to `OptionControlType` filigree style:
   - `TXT_GAME_CONTROLS`, `TXT_OPTIONS` → `OPTION_CONTROLS`
   - `TXT_LOAD_MISSION`, `TXT_SAVE_MISSION`, `TXT_DELETE_MISSION` → `OPTION_DELETE`
   - `TXT_NONE`, `TXT_MODEM_SERIAL`, etc. → `OPTION_DIALOG`
   - `TXT_HOST_SERIAL_GAME`, `TXT_JOIN_SERIAL_GAME` → `OPTION_SERIAL`
   - `TXT_SETTINGS`, `TXT_PHONE_LIST`, etc. → `OPTION_PHONE`
   - `TXT_JOIN_NETWORK_GAME` → `OPTION_JOIN_NETWORK`
   - `TXT_NETGAME_SETUP` → `OPTION_NETWORK`
   - `TXT_VISUAL_CONTROLS` → `OPTION_VISUAL`
   - `TXT_SOUND_CONTROLS` → `OPTION_SOUND`
   - Default → `OPTION_DIALOG`
2. Draw corner filigree shapes from `OPTIONS.SHP` if `option != OPTION_NONE`
3. Draw centered caption text with underline

## Functions and methods

### GameOptionsClass::Adjust_Variables_For_Resolution()
Set dialog dimensions based on screen resolution (320x200 vs 640x480).

### GameOptionsClass::Process()
Full options dialog loop with button creation, input handling, and sub-dialog launching.

### Draw_Caption(text, x, y, w)
Draw caption text with decorative corner filigree from OPTIONS.SHP.

## Inputs, outputs and side effects

**State mutations:**
- `BreakoutAllowed` (set true on Restate_Mission failure)
- `PlayerRestarts` (set on Quit→Restart)
- `OutList` gets `EventClass::DESTRUCT` on surrender
- `Options.GameSpeed` changes via GameControls sub-dialog
- Map re-rendered on dialog exit

**External systems affected:**
- `LOGIC.CPP` → pause menu invocation
- `GAMEDLG.CPP` → game controls sub-dialog
- `VISUDLG.CPP` → visual controls sub-dialog
- `SOUNDDLG.CPP` → sound controls sub-dialog
- `LOADDLG.CPP` → load/save dialogs
- `CONFDLG.CPP` → quit confirmation
- `DESCDLG.CPP` → mission restate

## Calls into / called by

**Called by:**
- `LOGIC.CPP` → when game paused
- `STARTUP.CPP` → initial options

**Calls into:**
- `Call_Back()` / `Main_Loop()` → game message pump
- `Map.Render()`, `HiddenPage.Clear()` → map rendering
- `Window_Box()`, `Draw_Caption()` → dialog drawing
- `buttons->Input()`, `buttons->Draw_All()` → gadget I/O
- `TextButtonClass::Turn_On/Off()` → button state
- `LoadOptionsClass::LOAD/SAVE/WWDELETE().Process()` → save/load
- `Surrender_Dialog()` → multiplayer surrender
- `CCMessageBox().Process()` → quit confirmation
- `GameControlsClass().Process()` → game sub-dialog
- `VisualControlsClass().Process()` → visual sub-dialog
- `SoundControlsClass().Process()` → sound sub-dialog
- `Restate_Mission()` → mission objective restatement
- `Queue_Exit()` → quit game
- `Play_Movie()` → play briefing/action movie
- `CC_Draw_Shape()`, `MixFileClass::Retrieve()` → filigree shapes
- `Fancy_Text_Print()` → caption text

## Related original files
- **GOPTIONS.H** - Class declarations
- **goptions.h** - Dialog constants
- **GAMEDLG.CPP** → game controls sub-dialog
- **VISUDLG.CPP** → visual controls sub-dialog
- **SOUNDDLG.CPP** → sound controls sub-dialog
- **LOADDLG.CPP** → load/save dialogs
- **CONFDLG.CPP** → confirmation dialogs
- **DESCDLG.CPP** → mission restate dialog
- **OPTIONS.CPP** → settings persistence

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/ui/options_menu.rs` - GameOptionsClass::Process()
- `cnc-tiberian-dawn/src/ui/draw_caption.rs` - Draw_Caption()

**Not yet ported:**
- Windows dialog rendering (`Window_Box`, `Draw_Caption`)
- `factor` resolution scaling
- Button linked-list chain (`Add_Tail`)
- `GadgetClass::LEFTPRESS`/`RIGHTPRESS` input modes
- `EventClass::DESTRUCT` multiplayer event
- `ControlClass` full-screen click capture
- `Restate_Mission()` movie playback
- `Surrender_Dialog()` multiplayer surrender
- `MixFileClass::Retrieve("OPTIONS.SHP")` filigree shapes

## Edge cases and compatibility notes

1. **Multiplay mode**: Some buttons disabled when `GameToPlay != GAME_NORMAL` (Load, Save, Restate)
2. **DELETE → RESIGN**: In multiplayer, DELETE button text changes to `TXT_RESIGN`
3. **French localization**: Resume button width differs (104 vs 90)
4. **Demo version**: Shows "DEMO" suffix and different version string format
5. **Japanese localization**: Different text IDs for Restate_Mission and Quit confirmation
6. **Game speed multiplayer safety**: Game speed changes go through `EventClass::GAMESPEED` not direct assignment
7. **Full-screen click capture**: `ControlClass` catches right-click anywhere or left-click outside dialog → resume
8. **Button width normalization**: All buttons forced to match widest button width
9. **Map re-render**: Map cleared and re-rendered on dialog exit
10. **Keyboard clear**: `Keyboard::Clear()` called before and after dialog

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/GOPTIONS.CPP at f1f0d42
