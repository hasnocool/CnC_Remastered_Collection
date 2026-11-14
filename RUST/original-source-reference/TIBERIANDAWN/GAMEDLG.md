# PATH/FILE
original/TIBERIANDAWN/GAMEDLG.CPP

## Purpose
Game controls options dialog for Tiberian Dawn. Provides the main options screen UI with game speed slider, scroll rate slider, visual controls button, sound controls button, and OK button. Handles player input for adjusting game settings during gameplay or from the main menu. Written by Maria del Mar McCready Legg and Joe L. Bostic.

## Where it fits in the game
UI/options system - the game controls dialog is accessed from the options menu. Integrated with `OPTIONS.CPP` for settings persistence, `VISUDLG.CPP` for visual controls sub-dialog, and `SOUNDDLG.CPP` for sound controls sub-dialog. Uses `EventClass` to post GAMESPEED events for multiplayer sync safety. Global `Options` object for settings storage.

## Classes, structs, enums and important data

### GameControlsClass::Process() → void
**Main options dialog processing loop:**

**Dialog geometry** (scaled by factor for 640x480):
| Element | Width | Height | X | Y |
|---------|-------|--------|---|---|
| Dialog | 232*factor | 141*factor | centered | centered |
| Game speed slider | d_dialog_w - 20*factor | 6*factor | d_speed_x | d_speed_y |
| Scroll rate slider | d_dialog_w - 20*factor | 6*factor | d_scroll_x | d_scroll_y |
| Visual button | d_dialog_w - 40*factor | 9*factor | d_visual_x | d_visual_y |
| Sound button | d_dialog_w - 40*factor | 9*factor | d_sound_x | d_sound_y |
| OK button | 20*factor | 9*factor | centered | bottom |

**factor**: 1 for 320x200, 2 for 640x480

### Button enum
| Value | Name | Purpose |
|-------|------|---------|
| 100 | BUTTON_SPEED | Game speed slider |
| 101 | BUTTON_SCROLLRATE | Scroll rate slider |
| 102 | BUTTON_VISUAL | Visual controls sub-dialog |
| 103 | BUTTON_SOUND | Sound controls sub-dialog |
| 104 | BUTTON_OK | Confirm and close |

**BUTTON_FIRST = 100**, **BUTTON_COUNT = 105**

### Slider controls
| Slider | Max setting | Range | Variable |
|--------|-------------|-------|----------|
| `gspeed_btn` | `OptionsClass::MAX_SPEED_SETTING` (0-7) | `MAX_SPEED_SETTING-1 - gamespeed` | `Options.GameSpeed` |
| `scrate_btn` | `OptionsClass::MAX_SCROLL_SETTING` (0-7) | `MAX_SCROLL_SETTING-1 - scrollrate` | `Options.ScrollRate` |

**Slider value conversion**: `val = (MAX - slider.Get_Value()) - 1`

### Input handling
| Key | Action |
|-----|--------|
| `BUTTON_SPEED | KN_BUTTON` | Select speed slider |
| `BUTTON_SCROLLRATE | KN_BUTTON` | Select scroll slider |
| `BUTTON_VISUAL | KN_BUTTON` | Open visual controls dialog |
| `BUTTON_SOUND | KN_BUTTON` | Open sound controls dialog |
| `BUTTON_OK | KN_BUTTON` | Save settings and close |
| `KN_ESC` | Close dialog |
| `KN_LEFT` / `KN_RIGHT` | Adjust selected slider |
| `KN_UP` / `KN_DOWN` | Cycle through buttons |
| `KN_RETURN` | Activate selected button |

## Functions and methods

### GameControlsClass::Process() → void
**Full dialog procedure:**

1. **Compute geometry** based on screen resolution (factor = 1 or 2)
2. **Create controls**: sliders (speed, scroll rate), buttons (visual, sound, OK)
3. **Initialize slider values** from `Options.GameSpeed` and `Options.ScrollRate`
4. **Build button list** via linked gadget chain
5. **Dialog loop** (`while (process)`):
   - `Call_Back()` if `GAME_NORMAL`, else `Main_Loop()` for pause
   - Redraw dialog on `SurfacesRestored` or `display`/`refresh` flags
   - Draw caption `TXT_GAME_CONTROLS`, slider labels, all buttons
   - Get input via `commands->Input()`
   - Process input (slider adjust, button cycle, activation)
   - On `pressed`: save settings via `Options.Save_Settings()`, launch sub-dialogs

**Multiplayer sync protection:** `Options.GameSpeed` is saved/restored around `Save_Settings()` to prevent desync. Game speed changes posted via `EventClass(EventClass::GAMESPEED, gamespeed)` instead of direct assignment.

### Sub-dialog launching
| Selection | Action |
|-----------|--------|
| `BUTTON_VISUAL` | `VisualControlsClass().Process()` → restart main loop |
| `BUTTON_SOUND` | If `!SoundType`: message box "no sound card"; else `SoundControlsClass().Process()` |
| `BUTTON_OK` | Close dialog |

## Inputs, outputs and side effects

**State mutations:**
- `Options.GameSpeed` (saved/restored around Save_Settings for multiplayer safety)
- `Options.ScrollRate` (directly set from slider)
- `gamespeed` local variable updated from slider
- `scrollrate` local variable updated from slider
- `OutList` gets `EventClass(EventClass::GAMESPEED, gamespeed)` on change

**External systems affected:**
- `OPTIONS.CPP` → `Options.Save_Settings()` persistence
- `VISUDLG.CPP` → `VisualControlsClass().Process()` sub-dialog
- `SOUNDDLG.CPP` → `SoundControlsClass().Process()` sub-dialog
- `EventClass` → GAMESPEED event for multiplayer sync
- `AllSurfaces.SurfacesRestored` → redraw on focus regain

## Calls into / called by

**Called by:**
- `OPTIONS.CPP` → `GameControlsClass::Process()` entry point
- `STARTUP.CPP` → initial options setup

**Calls into:**
- `Call_Back()` / `Main_Loop()` → game message pump
- `Dialog_Box()`, `Draw_Caption()`, `Hide_Mouse()`, `Show_Mouse()` → UI rendering
- `Fancy_Text_Print()` → slider labels
- `commands->Input()`, `commands->Draw_All()` → gadget input/rendering
- `gspeed_btn.Bump()`, `scrate_btn.Bump()` → slider adjustment
- `OutList.Add(EventClass::GAMESPEED)` → multiplayer event posting
- `Options.Save_Settings()` → settings persistence
- `VisualControlsClass().Process()` → visual sub-dialog
- `SoundControlsClass().Process()` → sound sub-dialog
- `CCMessageBox().Process()` → no sound card error

## Related original files
- **GAMEDLG.H** - Class declarations
- **gamedlg.h** - Dialog constants
- **OPTIONS.CPP** → `Options` global and `Save_Settings()`
- **VISUDLG.CPP** → visual controls sub-dialog
- **SOUNDDLG.CPP** → sound controls sub-dialog
- **SOUNDDLG.H** - Sound controls header
- **VISUDLG.H** - Visual controls header

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/ui/game_controls.rs` - GameControlsClass::Process()
- `cnc-tiberian-dawn/src/ui/slider.rs` - SliderClass
- `cnc-tiberian-dawn/src/ui/options.rs` - Options class

**Not yet ported:**
- Windows dialog rendering (`Dialog_Box`, `Draw_Caption`)
- Slider thumb-based value conversion
- `factor` resolution scaling (320x200 vs 640x480)
- `EventClass::GAMESPEED` multiplayer event posting
- `Options.GameSpeed` save/restore for multiplayer safety
- `VisualControlsClass` / `SoundControlsClass` sub-dialogs
- Gadget linked-list button chain
- `Fancy_Text_Print()` text rendering

## Edge cases and compatibility notes

1. **Multiplayer sync**: `Options.GameSpeed` must NOT be set directly - use `EventClass::GAMESPEED` event instead
2. **Save/restore pattern**: `Options.GameSpeed` saved to `old`, set to `gamespeed`, `Save_Settings()`, then restored to `old`
3. **Resolution scaling**: All dimensions multiplied by `factor` (1 for 320x200, 2 for 640x480)
4. **Slider inversion**: Slider thumb value is inverted: `val = (MAX - slider.Get_Value()) - 1`
5. **Sub-dialog restart**: After visual/sound sub-dialog closes, main loop restarts with `process = true; display = true; refresh = true`
6. **No sound card**: `SoundType` check prevents opening sound dialog without hardware
7. **Button cycling**: UP/DOWN wrap around button array
8. **Slider adjustment**: LEFT/RIGHT bump only affects currently selected slider

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/GAMEDLG.CPP at f1f0d42
