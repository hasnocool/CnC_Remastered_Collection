# PATH/FILE
original/TIBERIANDAWN/VISUDLG.CPP

## Purpose
Visual controls dialog box system for adjusting display settings (brightness, color/saturation, contrast, and tint/hue). The dialog presents four slider controls with accompanying labels and provides OK/Reset button interaction for the player to adjust visual display parameters. Part of the options/settings UI subsystem in Command & Conquer Tiberian Dawn.

## Where it fits in the game
Options/display settings subsystem. The VisualControlsClass::Process() method displays and processes a modal dialog box that allows the player to adjust game visual parameters. This dialog is typically accessed from the main options menu. The dialog processes slider input for four video parameters and button input for reset or accepting changes.

## Classes, structs, enums and important data

### VisualControlsClass
**Main class** - Handles creation, display, and processing of the visual controls dialog.

**Private enum VisualControlEnums:**
| Enum Value | Purpose |
|---|---|
| `BUTTON_BRIGHTNESS` = 1 | Brightness slider/button |
| `BUTTON_COLOR` | Color/saturation slider/button |
| `BUTTON_CONTRAST` | Contrast slider/button |
| `BUTTON_TINT` | Tint/hue slider/button |
| `BUTTON_RESET` | Reset button - restores default values |
| `BUTTON_OPTIONS` | Options button - exits dialog |

**Key data members (from class definition):**
- `Option_Width` / `Option_Height` - Dialog box dimensions (216×122 pixels, scaled by factor)
- `Option_X` / `Option_Y` - Dialog box screen position (centered)
- `Text_X` / `Text_Y` - Title/text label positions
- `Slider_X` / `Slider_Y` / `Slider_Y_Spacing` - Slider control layout (vertical stack)
- `Slider_Width` / `Slider_Height` - Slider dimensions (70×5 pixels, scaled)
- `Button_X` / `Button_Y` - Button positions (Options and Reset buttons)

**Public data members:**
- `Option_Width`, `Option_Height`, `Option_X`, `Option_Y`
- `Text_X`, `Text_Y`
- `Slider_X`, `Slider_Y`
- `Slider_Width`, `Slider_Height`
- `Slider_Y_Spacing`
- `Button_X`, `Button_Y`

### Global/Static state
- `SeenBuff` - Screen buffer used to determine scaling factor (320×200 → factor=1, otherwise factor=2)
- `Options` - Global options object providing getter/setter methods for brightness, color, contrast, tint values
- `_titles[4]` - Static array of title IDs: `TXT_BRIGHTNESS`, `TXT_COLOR`, `TXT_CONTRAST`, `TXT_TINT`

## Functions and methods

### VisualControlsClass::Init(void) → int
**Static initialization** - Called once to set up dialog geometry based on screen resolution.

**Process:**
1. Compute `factor` = 1 if screen width is 320, else 2 (for high-res scaling)
2. Set `Option_Width = 216 * factor`, `Option_Height = 122 * factor`
3. Center dialog: `Option_X = ((SeenBuff.Get_Width() - Option_Width) / 2)`, `Option_Y = ((SeenBuff.Get_Height() - Option_Height) / 2)`
4. Set text/slider positions relative to Option_X/Y with scaling
5. Set button positions
6. Store and return `factor`

**Computed coordinates (with factor):**
- `Text_X = Option_X + (28 * factor)`, `Text_Y = Option_Y + (30 * factor)`
- `Slider_X = Option_X + (105 * factor)`, `Slider_Y = Option_Y + (30 * factor)`
- `Slider_Width = 70 * factor`, `Slider_Height = 5 * factor`
- `Slider_Y_Spacing = 11 * factor`
- `Button_X = Option_X + (63 * factor)`, `Button_Y = Option_Y + (102 * factor)`

### VisualControlsClass::Process(void)
**Main processing loop** - Displays the visual controls dialog and processes all user input until the player clicks Options or presses Escape.

**High-level flow:**
1. Call `Init()` to compute geometry
2. Call `Set_Logic_Page(SeenBuff)` to set the logic page
3. Create user interface objects:
   - `optionsbtn` - "Game Controls" text button
   - `resetbtn` - "Reset Menu" text button
   - Four `SliderClass` instances: `brightness`, `color`, `contrast`, `tint`
   - `dialog` - Capturing GadgetClass (left-press only)
   - `background` - Full-screen control capturing left/right press
4. **Main while loop** (`process` = true):
   a. **Game callback**: If `GameToPlay == GAME_NORMAL`, call `Call_Back()`. Else, call `Main_Loop()` and set `process = false` on completion.
   b. **Surface restoration**: If `AllSurfaces.SurfacesRestored`, clear the flag and set `display = TRUE`.
   c. **Full redraw** (if `display`):
      - Hide mouse
      - Draw dialog box at `Option_X, Option_Y`
      - Draw caption "Visual Controls"
      - Show mouse
      - Set `display = false`, `partial = true`
   d. **Partial redraw** (if `partial`):
      - Hide mouse
      - Draw four title labels (Brightness, Color, Contrast, Tint) with highlighting for current `curbutton`
      - Draw all buttons
      - Show mouse
      - Set `partial = false`
   e. **Input processing**:
      - Get `input = optionsbtn.Input()`
      - Switch on input type:
      
      **Button presses** (KN_BUTTON variants):
      - `BUTTON_BRIGHTNESS` → Set `Options.Set_Brightness(brightness.Get_Value())`
      - `BUTTON_COLOR` → Set `Options.Set_Color(color.Get_Value())`
      - `BUTTON_CONTRAST` → Set `Options.Set_Contrast(contrast.Get_Value())`
      - `BUTTON_TINT` → Set `Options.Set_Tint(tint.Get_Value())`
      - `BUTTON_RESET` → Reset all sliders to 128 and update Options
      - `BUTTON_OPTIONS` or `KN_ESC` → Set `selection = BUTTON_OPTIONS`, `pressed = true`
      - `KN_LEFT` → Bump slider left; cycle button selection left; wrap-around handling
      - `KN_RIGHT` → Bump slider right; cycle button selection right; wrap-around handling
      - `KN_UP` → Decrement `curbutton` with wrap; set `partial = true` for title redraw
      - `KN_DOWN` → Increment `curbutton` with wrap; set `partial = true` for title redraw
      - `KN_RETURN` → Set `selection = curbutton + BUTTON_BRIGHTNESS`, `pressed = true`

   f. **Process pending selection** (if `pressed`):
      - `BUTTON_RESET` → Set all slider values to 128 and update Options
      - `BUTTON_OPTIONS` → Set `process = false` (exit loop)

   g. Loop continues until `process = false`

## Inputs, outputs and side effects

### State mutations
- **Slider values**: Brightness, color, contrast, tint values updated via slider interaction
- **Options global**: `Options.Set_Brightness()`, `Set_Color()`, `Set_Contrast()`, `Set_Tint()` called with new values
- **Reset**: All four parameters reset to 128 (midpoint/neutral values)
- **Selection**: `curbutton` index tracks which control has focus (0=brightness, 1=color, 2=contrast, 3=tint)
- `pressed` flag: Signals that a button action should be processed at end of loop iteration
- `display` / `partial`: Redraw flags controlling rendering

### External systems affected
- **Options object**: Global display settings modified
- **Mouse cursor**: Temporarily hidden during redraw, shown during interaction
- **Dialog rendering**: Dialog box, caption, sliders, buttons, titles
- **Game callback**: `Call_Back()` or `Main_Loop()` invoked during dialog processing

### Side effects
- Mouse cursor visibility toggled during dialog display
- Display refresh with full or partial redraw based on state
- Slider thumb positions updated to reflect current values
- Button highlighting reflects current `curbutton` selection

## Calls into / called by

### Called by:
- **Options menu system** → Invokes `VisualControlsClass::Process()` to display the dialog
- **Game initialization** → `VisualControlsClass::Init()` called to set up geometry

### Calls into:
- `Set_Logic_Page(SeenBuff)` - Sets the game logic page for rendering
- `Dialog_Box(Option_X, Option_Y, Option_Width, Option_Height)` - Draws the dialog box border/background
- `Draw_Caption(TXT_VISUAL_CONTROLS, Option_X, Option_Y, Option_Width)` - Draws the window caption
- `Hide_Mouse()` / `Show_Mouse()` - Mouse cursor management
- `Fancy_Text_Print()` - Draws title labels with gradient palette
- `TextButtonClass::Draw_All()` - Draws the options/reset buttons
- `SliderClass::Bump(direction)` - Adjusts slider value left/right
- `SliderClass::Get_Value()` / `Set_Value()` - Slider value access
- `Options.Get_Brightness()/Get_Color/Get_Contrast/Get_Tint()` - Read current options
- `Options.Set_Brightness/Set_Color/Set_Contrast/Set_Tint()` - Write options values
- `Call_Back()` - Game callback for pause/unpause
- `Main_Loop()` - Main game loop when not in normal game mode

## Related original files
- **VISUDLG.H** - Class declarations, enums, and data member definitions
- **FUNCTION.H** - Global functions and `SeenBuff` screen buffer reference
- **GADGET.H** - Base class for `GadgetClass`, `TextButtonClass`, `SliderClass`, `ControlClass`
- **OPTIONS.H** / **OPTIONS.CPP** - Global `Options` object providing getter/setter methods
- **KEYBOARD.H** - Key code definitions (`KN_BUTTON`, `KN_ESC`, `KN_LEFT`, `KN_RIGHT`, `KN_UP`, `KN_RETURN`)

## Rust port mapping

**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-map-editor`

**Mapped modules:**
- `cnc-core/src/visual-controls.rs` - VisualControlsClass port: Init(), Process(), slider/button management
- `cnc-core/src/options.rs` - Options struct/data with brightness/color/contrast/tint fields and getter/setter methods
- `cnc-core/src/screen-buffer.rs` - SeenBuff/screen resolution detection for scaling factor

**Not yet ported:**
- Direct `GadgetClass`/`TextButtonClass`/`SliderClass` wrapper types (Rust GUI framework differences)
- Platform-dependent `Dialog_Box()`, `Draw_Caption()`, `Hide_Mouse()`, `Show_Mouse()` functions
- `Fancy_Text_Print()` with gradient palette rendering
- `Call_Back()` and `Main_Loop()` game mode switching logic
- Full input event pipeline (`optionsbtn.Input()` switching on KN_* codes)
- `ControlClass` background capture for right-click/outside-dialog dismissal
- Integer scaling factor computation based on `SeenBuff.Get_Width()`
- `TPF_6PT_GRAD | TPF_NOSHADOW` text printing flags
- Wrap-around button selection cycling logic (left/right/up/down boundary conditions)

## Edge cases and compatibility notes

1. **Resolution scaling**: Dialog geometry scales with `factor` = 1 (320×200) or 2 (other resolutions). All coordinates, sizes, and spacing multiplied by factor.

2. **Slider bump direction**: `KN_LEFT` decreases slider value; `KN_RIGHT` increases. Bump affects the currently focused slider (`buttonsliders[curbutton]`).

3. **Button cycling**: `curbutton` cycles through 0-3 (brightness→color→contrast→tint) and can wrap past the ends to reset/options buttons. Left/right wrap follows specific boundary conditions.

4. **Partial vs full redraw**: `partial = true` only redraws titles and buttons; `display = true` does a complete dialog box redraw. This minimizes flicker during interaction.

5. **Reset behavior**: All four sliders reset to value 128 (midpoint), representing neutral/unset display settings. The `Options` global is updated accordingly.

6. **Escape/Options exit**: Both `KN_ESC` and `BUTTON_OPTIONS|KN_BUTTON` or clicking the options button exit the dialog without saving changes (selection = BUTTON_OPTIONS).

7. **Return key**: `KN_RETURN` triggers action based on `curbutton` selection: `selection = curbutton + BUTTON_BRIGHTNESS`, which maps to BUTTON_BRIGHTNESS..BUTTON_TINT range, effectively accepting the currently focused slider's value.

8. **Background capture**: The `dialog` GadgetClass captures left-press within dialog bounds; `background` ControlClass captures left/right press anywhere, with right-click or clicking outside equivalent to pressing Options.

9. **Title highlighting**: When `curbutton == i`, titles are drawn in bright green (`TPF_BRIGHT_COLOR`); otherwise with gradient palette. This provides visual feedback on which control has focus.

10. **INI independence**: Unlike many CnC subsystems, this dialog does not read/write from INI files; all values are in-memory and session-only.