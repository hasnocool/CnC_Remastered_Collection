# PATH/FILE
original/TIBERIANDAWN/GAUGE.CPP

## Purpose
Gauge/slider base class for Tiberian Dawn UI. Provides a draggable thumb on a horizontal or vertical track with value-to-pixel conversion, colorized fill, and optional tri-color (red/yellow/green) zones. Used as base for `SliderClass` and power bar.

## Where it fits in the game
UI control foundation - Base for `SliderClass` (scrollbars, list boxes) and `PowerClass` (sidebar power bar). Inherits `ControlClass` → `GadgetClass`. Provides value↔pixel conversion, thumb dragging, and colorized fill.

## Classes, structs, enums and important data

### GaugeClass : ControlClass
**Draggable value gauge** - Horizontal or vertical.

**Key data members:**
- `CurValue` (int) - Current value (0..MaxValue)
- `MaxValue` (int) - Maximum value (default 255)
- `IsHorizontal` (bool) - Orientation (width > height)
- `IsColorized` (bool) - Fill bar with color (default true)
- `HasThumb` (bool) - Shows draggable thumb (default true)
- `ClickDiff` (int) - Drag offset compensation
- `IsColorized` (bool) - Green fill (default true)

**Inherited from ControlClass:**
- `X, Y, Width, Height` - Bounds
- `ID` - Control ID
- Flags: `LEFTHELD|LEFTPRESS|LEFTRELEASE`, sticky

### TriColorGaugeClass : GaugeClass
**Three-zone gauge** (power bar) - Red/yellow/green segments.

**Additional fields:**
- `RedLimit` (int) - Red zone max (0..YellowLimit)
- `YellowLimit` (int) - Yellow zone max (RedLimit..MaxValue)

## Functions and methods

### GaugeClass(id, x, y, w, h)
**Constructor** - Initializes base, sets defaults.
1. `ControlClass(id, x, y, w, h, LEFTHELD|LEFTPRESS|LEFTRELEASE, true)`
2. `Set_Maximum(255)`, `Set_Value(0)`
3. `HasThumb = true`
4. `IsHorizontal = (w > h)`
5. `IsColorized = true`
6. `ClickDiff = 0`

### Set_Maximum(value) → bool
**Sets max value** - Updates `MaxValue`, triggers redraw.

### Set_Value(value) → bool
**Sets current value** - Clamped to `[0, MaxValue]`, triggers redraw.

### Pixel_To_Value(pixel) → int
**Mouse position → gauge value.**

```
pixel -= IsHorizontal ? (X+1) : (Y+1)
maximum = (IsHorizontal ? Width : Height) - 2
pixel = Bound(0, maximum, pixel)
return Fixed_To_Cardinal(MaxValue, Cardinal_To_Fixed(maximum, pixel))
```

### Value_To_Pixel(value) → int
**Gauge value → pixel offset.**

```
maximum = IsHorizontal ? Width : Height
start = IsHorizontal ? X : Y
maximum -= 2
return start + Fixed_To_Cardinal(maximum, Cardinal_To_Fixed(MaxValue, value))
```

### Draw_Me(forced) → int
**Renders gauge track + fill + thumb.**

1. `ControlClass::Draw_Me(forced)` gate
2. Hides mouse over bounds
3. Draws `BOXSTYLE_GREEN_DOWN` box background
4. **Colorized fill** (if `IsColorized`):
   - `middle = Value_To_Pixel(CurValue)`
   - Horizontal: `Fill_Rect(X+1, Y+1, middle, Y+Height-2, CC_BRIGHT_GREEN)`
   - Vertical: `Fill_Rect(X+1, Y+1, X+Width-2, middle, CC_BRIGHT_GREEN)`
5. If `HasThumb`: `Draw_Thumb()`
6. Shows mouse

### Action(flags, key) → int
**Handles thumb dragging.**

**If `HasThumb`:**
1. `Sticky_Process(flags)`
2. **Left press/held on thumb**:
   - `LEFTPRESS`: Computes `ClickDiff` (mouse offset from thumb edge)
   - `LEFTHELD` (if `StuckOn==this`): Updates value from mouse position
   - `ClickDiff` compensates for `Pixel_To_Value`/`Value_To_Pixel` rounding
   - `Set_Value(Pixel_To_Value(mouse - ClickDiff))`
   - If no change: Clears flags, delegates to `ControlClass::Action(0)`
3. **Else**: Clears `LEFTHELD` (allows derived classes to handle)
4. Delegates to `ControlClass::Action()`

### Draw_Thumb()
**Renders 4px thumb at current value.**

```
x = Value_To_Pixel(CurValue)
if (x + 4 > Value_To_Pixel(MaxValue)) x = Value_To_Pixel(MaxValue) - 2
Horizontal: Draw_Box(x, Y, 4, Height, BOXSTYLE_GREEN_RAISED)
Vertical: Draw_Box(X, x, Width, 4, BOXSTYLE_GREEN_RAISED)
```
(Thumb width reduced from 8→4px in code)

### Thumb_Pixels() (implied)
Returns thumb width (4px horizontal, 4px vertical).

## TriColorGaugeClass : GaugeClass
**Power bar with red/yellow/green zones.**

### TriColorGaugeClass(id, x, y, w, h)
**Constructor** - Calls base, `RedLimit=0`, `YellowLimit=0`.

### Set_Red_Limit(value) / Set_Yellow_Limit(value) → bool
**Sets zone boundaries** - Clamped to `[0, MaxValue)`, triggers redraw.

### Draw_Me(forced) → int
**Three-zone rendering.**

**Background:** `BOXSTYLE_GREEN_DOWN` (or `GREEN_RAISED` if disabled)

**Zones (horizontal):**
| Value range | Color | Rect |
|-------------|-------|------|
| `0..RedLimit` | `PINK` (red) | `X+1..middle` |
| `RedLimit..YellowLimit` | `YELLOW` | `red..middle` |
| `YellowLimit..MaxValue` | `GREEN` | `yellow..middle` |

**Vertical:** Same logic, Y-axis.

Where:
- `red = Value_To_Pixel(RedLimit)`
- `yellow = Value_To_Pixel(YellowLimit)`
- `middle = Value_To_Pixel(CurValue)`

**Thumb:** Calls `Draw_Thumb()` if `HasThumb`.

## Conversion math
Uses `Cardinal_To_Fixed`/`Fixed_To_Cardinal` (16.16 fixed-point):
- `Cardinal_To_Fixed(base, val)` = `(val << 8) / base`
- `Fixed_To_Cardinal(base, fixed)` = `(base * fixed + 128) >> 8`

## Inputs, outputs and side effects

**State mutations:**
- `CurValue`, `MaxValue`, `ClickDiff`
- Red/yellow limits (tri-color)
- Redraw flags

**External systems affected:**
- Power bar (sidebar)
- Scrollbars (via SliderClass)
- Mouse cursor (hidden during draw)

## Calls into / called by

**Called by:**
- `SliderClass` (base)
- `PowerClass` (tri-color)
- UI input loop → `Action()`
- Rendering → `Draw_Me()`

**Calls into:**
- `ControlClass::Action/Draw_Me`
- `Fixed_To_Cardinal`/`Cardinal_To_Fixed` (COORD.CPP)
- `Draw_Box`, `Fill_Rect` (rendering)
- `Conditional_Hide/Show_Mouse`
- `Bound` (clamping)

## Related original files
- **GAUGE.H** - Class declarations
- **CONTROL.H/.CPP** - ControlClass base
- **SLIDER.H/.CPP** - SliderClass (consumer)
- **POWER.H/.CPP** - PowerClass (TriColorGaugeClass consumer)
- **COORD.H/.CPP** - Fixed-point math
- **BOX.H/.CPP** - Draw_Box, Fill_Rect

## Rust port mapping
**Crates:** `cnc-ui`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-ui/src/gadget/gauge.rs` - GaugeClass
- `cnc-ui/src/gadget/slider.rs` - SliderClass (consumer)
- `cnc-ui/src/gadget/power_bar.rs` - TriColorGaugeClass
- `cnc-core/src/math/fixed.rs` - Fixed-point math

**Not yet ported:**
- Horizontal/vertical orientation auto-detect
- Fixed-point value↔pixel conversion
- ClickDiff drag compensation
- Green raised thumb (4px)
- Bright green fill (`CC_BRIGHT_GREEN`)
- Tri-color zones (pink/yellow/green)
- Disabled style (`BOXSTYLE_GREEN_RAISED`)
- Thumb clamp at max value
- Sticky drag (`StuckOn`)

## Edge cases and compatibility notes

1. **Thumb width**: 4px (reduced from 8px in commented code)
2. **Thumb clamp**: `x + 4 > max` → `x = max - 2`
3. **ClickDiff**: Compensates for fixed-point rounding in drag
4. **Orientation**: Auto-detected from `w > h` in constructor
5. **Colorized default**: Green fill (`CC_BRIGHT_GREEN`)
6. **Tri-color zones**: Pink (red) → Yellow → Green
7. **Disabled style**: `BOXSTYLE_GREEN_RAISED` for tri-color
8. **Thumb size**: 4px (was 8px)
9. **Sticky drag**: `StuckOn == this` enables `LEFTHELD` processing
10. **Fixed-point**: 16.16 via `Cardinal_To_Fixed`/`Fixed_To_Cardinal`

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/GAUGE.CPP at f1f0d42
