# PATH/FILE
original/TIBERIANDAWN/TEXTBTN.CPP

## Purpose
Text button control for Tiberian Dawn UI. Provides a button with dynamic text label, auto-sizing, gradient/flat styles, pressed/disabled states, and optional black border. Inherits toggle behavior from `ToggleClass`.

## Where it fits in the game
UI control - Used extensively in menus, dialogs, and options screens for clickable text labels (OK, Cancel, Yes, No, menu items). Inherits `ToggleClass` → `ControlClass` → `GadgetClass`. Used by `Special_Dialog`, `MPLAYER.CPP`, `SIDEBAR.CPP`, etc.

## Classes, structs, enums and important data

### TextButtonClass : ToggleClass
**Text-labeled button with auto-sizing and style variants.**

**Key data members:**
- `String` (char const*) - Display text (pointer, not copied)
- `PrintFlags` (TextPrintType) - `Fancy_Text_Print` style flags
- `IsBlackBorder` (bool) - Black rectangle border
- `IsPressed` (bool, inherited) - Currently pressed
- `IsOn` (bool, inherited) - Toggle state
- `IsDisabled` (bool, inherited) - Disabled state
- `IsPressed` / `IsOn` - Visual state flags

**Inherited from ToggleClass:**
- `IsSticky` = true (captures mouse)
- `IsToggleType` = true (toggles `IsOn`)
- `ReflectButtonState` = true
- `ID` → returns `ID | KN_BUTTON` on action

## Functions and methods

### Constructors

#### TextButtonClass(id, text_ptr, style, x, y, w, h, blackborder)
**String pointer constructor.**
1. `ToggleClass(id, x, y, w, h)`
2. `String = text_ptr`
3. `PrintFlags = style`, `IsBlackBorder = blackborder`
4. Auto-size if `w==-1 || h==-1`:
   - `Width = String_Pixel_Width(String) + 8`
   - `Height = FontHeight + FontYSpacing + 2`
   - (Hi-res scaling commented out)

#### TextButtonClass()
**Default constructor** - Zeroed fields, `PrintFlags = TPF_8POINT`.

#### TextButtonClass(id, text_id, style, x, y, w, h, blackborder)
**Text ID constructor** - Calls `Set_Text(text_id)`, then same auto-size logic.

### Set_Text(text_ptr, resize) / Set_Text(text_id, resize)
**Updates button text** - Stores pointer (not copied!), `Flag_To_Redraw()`.
- If `resize && String`: Recalculates `Width`/`Height` from text metrics.

### Draw_Me(forced) → int
**Renders button** - Delegates to virtual `Draw_Background()` + `Draw_Text()`.

1. `ControlClass::Draw_Me(forced)` gate
2. Hides mouse over bounds
3. `Draw_Background()` (virtual)
4. `Draw_Text(String)` (virtual)
5. Shows mouse

### Draw_Background()
**Renders button box with state-based style.**

| Condition | Style (TPF_6PT_GRAD) | Style (flat) |
|-----------|---------------------|--------------|
| Disabled | `BOXSTYLE_GREEN_DIS_RAISED` | `BOXSTYLE_DIS_RAISED` |
| Pressed | `BOXSTYLE_GREEN_DOWN` | `BOXSTYLE_DOWN` |
| Default | `BOXSTYLE_GREEN_RAISED` | `BOXSTYLE_RAISED` |

Plus optional black border: `Draw_Rect(X-1, Y-1, X+Width+2, Y+Height+2, BLACK)`.

### Draw_Text(text)
**Renders centered text with state-based color.**

**Gradient style (`TPF_6PT_GRAD`):**
| State | Color | Flags |
|-------|-------|-------|
| Disabled | `CC_GREEN` | (none) |
| Pressed/On | `CC_GREEN` | `TPF_USE_GRAD_PAL|TPF_BRIGHT_COLOR` |
| Normal | `CC_GREEN` | `TPF_USE_GRAD_PAL|TPF_MEDIUM_COLOR` |

**Flat style:**
| State | Color |
|-------|-------|
| Disabled | `LTGREY` |
| Pressed (no shadow) | `DKGREY` |
| Pressed (shadow) | `LTGREY` |
| On | `RED` |
| Normal | `WHITE` |

Always centered: `X+(Width>>1)-1, Y+1`, `TPF_CENTER`.

## Style flags (PrintFlags / TextPrintType)
| Flag | Effect |
|------|--------|
| `TPF_6PT_GRAD` | Gradient 6pt font (green palette) |
| `TPF_8POINT` | Standard 8pt font |
| `TPF_CENTER` | Horizontal center |
| `TPF_NOSHADOW` | No drop shadow |
| `TPF_USE_GRAD_PAL` | Use gradient palette |
| `TPF_BRIGHT_COLOR` | Bright gradient color |
| `TPF_MEDIUM_COLOR` | Medium gradient color |

## Toggle behavior (inherited)
- `IsSticky = true` (mouse capture)
- `IsToggleType = true` (click toggles `IsOn`)
- `ReflectButtonState = true` (visual matches state)
- `IsSticky` → captures mouse until release
- Click returns `ID | KN_BUTTON`

## Auto-sizing
If `w==-1` or `h==-1` in constructor:
- `Width = String_Pixel_Width(String) + 8`
- `Height = FontHeight + FontYSpacing + 2`
- Hi-res scaling commented out (`// if (SeenBuff.Get_Width() != 320) Width *= 2`)

## Inputs, outputs and side effects

**State mutations:**
- `String` pointer (not copied - caller must keep valid)
- `IsOn` toggled on click (via ToggleClass)
- `IsPressed` during mouse hold
- Redraw flags

**External systems affected:**
- Dialog navigation (OK/Cancel/Yes/No)
- Menu navigation
- Option toggles

## Calls into / called by

**Called by:**
- Dialogs (`Special_Dialog`, `MPLAYER.CPP`, `SIDEBAR.CPP`)
- Main loop → `Input()` → `Action()` → toggle
- Menu systems

**Calls into:**
- `ToggleClass::Action()` (toggle logic)
- `ControlClass::Draw_Me()` (render gate)
- `Fancy_Text_Print` / `String_Pixel_Width` (text metrics)
- `Draw_Box` / `Draw_Rect` (box rendering)
- `Conditional_Hide/Show_Mouse()`

## Related original files
- **TEXTBTN.H** - Class declaration
- **TOGGLE.H/.CPP** - ToggleClass base
- **CONTROL.H/.CPP** - ControlClass (ID, peer)
- **BUTTON.H/.CPP** - ShapeButtonClass (image buttons)
- **CHECKBOX.H/.CPP** - CheckBoxClass (similar toggle)
- **TOGGLE.H/.CPP** - ToggleClass base
- **FANCYTEXT.H/.CPP** - Fancy_Text_Print, String_Pixel_Width
- **BOX.H/.CPP** - Draw_Box styles

## Rust port mapping
**Crates:** `cnc-ui`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-ui/src/gadget/text_button.rs` - TextButtonClass
- `cnc-ui/src/gadget/toggle.rs` - ToggleClass base
- `cnc-ui/src/gadget/control.rs` - ControlClass

**Not yet ported:**
- Auto-sizing from text metrics
- Gradient vs flat style switching (`TPF_6PT_GRAD`)
- State-based color/style selection
- Text pointer ownership (not copied)
- Optional black border
- Centered text with `TPF_CENTER`
- Hi-res auto-scale (commented out)
- Disabled/pressed/on visual states
- Toggle behavior with `IsSticky` capture

## Edge cases and compatibility notes

1. **Text not copied**: `String` is raw pointer; caller must ensure lifetime
2. **Hi-res scaling**: Commented out in source; would need `SeenBuff` check
3. **Gradient palette**: Requires `TPF_USE_GRAD_PAL` + `TPF_6PT_GRAD` for green palette
4. **Color logic**: Gradient uses green shades; flat uses white/grey/red
5. **Border**: `IsBlackBorder` adds 1px black rect outside button bounds
6. **Default constructor**: Creates invalid button (zero size, no text)
7. **Resize flag**: `Set_Text(text, true)` triggers auto-size recalculation
6. **Text ID overload**: Converts via `Text_String()` then delegates
7. **Hi-res comments**: `// if (SeenBuff.Get_Width() != 320) Width *= 2` suggests planned but disabled
8. **TextPrintType**: Bitmask of TPF_ flags
9. **Toggle visual**: `IsOn` affects gradient brightness, flat color (RED when on)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TEXTBTN.CPP at f1f0d42
