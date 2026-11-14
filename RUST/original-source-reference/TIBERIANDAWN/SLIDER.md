# PATH/FILE
original/TIBERIANDAWN/SLIDER.CPP

## Purpose
Slider control for Tiberian Dawn UI. Provides a scrollable thumb with optional increment/decrement buttons, peer-linked to list boxes for synchronized scrolling. Supports both horizontal and vertical orientation with proportional thumb sizing.

## Where it fits in the game
UI control - Used by `ListClass` for scrollbars. Inherits from `GaugeClass` → `ControlClass` → `GadgetClass`. Creates `PlusGadget`/`MinusGadget` (`ShapeButtonClass`) for increment/decrement unless embedded in a list (`BelongToList`).

## Classes, structs, enums and important data

### SliderClass : GaugeClass
**Slider with thumb and optional buttons.**

**Key data members:**
- `PlusGadget` / `MinusGadget` (ShapeButtonClass*) - Increment/decrement buttons
- `Thumb` (int) - Logical thumb size (1..MaxValue)
- `ThumbSize` (int) - Pixel size of thumb
- `ThumbStart` (int) - Pixel offset of thumb start
- `BelongToList` (bool) - Embedded in list (no separate buttons)
- `IsHorizontal` (bool, inherited) - Orientation (width > height)
- `IsColorized` (bool) - Single-color thumb (0 for slider)

**Inherited from GaugeClass:**
- `CurValue` (int) - Current thumb position (0..MaxValue-Thumb)
- `MaxValue` (int) - Maximum value
- `X, Y, Width, Height` - Bounds
- `IsHorizontal` - Width > Height

## Functions and methods

### SliderClass(id, x, y, w, h, belong_to_list)
**Constructor** - Initializes base `GaugeClass`, sets orientation.

1. `BelongToList = belong_to_list`
2. If not in list:
   - Creates `PlusGadget` (`BTN-PLUS.SHP`) at `(X+Width+2, Y)`
   - Creates `MinusGadget` (`BTN-MINS.SHP`) at `(X-6, Y)`
   - `Make_Peer(*this)` + `Add(*this)` for both
   - `Flag_To_Redraw()`
3. `Set_Thumb_Size(1)`, `Recalc_Thumb()`
4. `IsColorized = 0` (single color)

### ~SliderClass()
**Destructor** - Deletes `PlusGadget`/`MinusGadget`.

### Set_Maximum(value) → bool
**Sets max value** - Calls base, then `Recalc_Thumb()`.

### Set_Thumb_Size(value)
**Sets logical thumb size** - Clamped to `[1, MaxValue]`, triggers `Recalc_Thumb()`.

### Set_Value(value) → bool
**Sets thumb position** - Clamped to `[0, MaxValue-Thumb]`, calls base, `Recalc_Thumb()`.

### Recalc_Thumb()
**Computes pixel geometry from logical values.**

```
length = IsHorizontal ? Width : Height
size   = Fixed_To_Cardinal(length, Cardinal_To_Fixed(MaxValue, Thumb))
ThumbSize = MAX(size, 4)
start  = Fixed_To_Cardinal(length, Cardinal_To_Fixed(MaxValue, CurValue))
ThumbStart = MIN(start, length - ThumbSize)
```

- Uses `Cardinal_To_Fixed`/`Fixed_To_Cardinal` for proportional mapping
- Minimum thumb 4 pixels
- Clamps start so thumb stays in bounds

### Action(flags, key) → int
**Handles mouse interaction.**

**Left press logic:**
1. Gets mouse position (`Get_Mouse_X/Y`)
2. If click **before thumb** (`mouse < edge+ThumbStart`):
   - `Bump(true)` (decrease)
   - Calls `GaugeClass::Action(0, key)`, clears key, returns true
3. If click **after thumb** (`mouse > edge+ThumbStart+ThumbSize`):
   - `Bump(false)` (increase)
   - Same as above
4. If click **on thumb**: Normal `GaugeClass::Action(flags, key)`

**Other events:** Delegates to `GaugeClass::Action()` (note: comment says remove LEFTRELEASE from parent).

### Bump(up) → bool
**Jumps by thumb size** - `Set_Value(CurValue ± Thumb)`.

### Step(up) → bool
**Jumps by 1** - `Set_Value(CurValue ± 1)`.

### Draw_Thumb()
**Renders thumb** - Green raised box at `ThumbStart`/`ThumbSize`.

| Orientation | Box params |
|-------------|------------|
| Horizontal | `(X+ThumbStart, Y, ThumbSize, Height)` |
| Vertical | `(X, Y+ThumbStart, Width, ThumbSize)` |

Style: `BOXSTYLE_GREEN_RAISED`, filled.

### Draw_Me(forced) → int
**Renders slider body + thumb.**

**If `BelongToList`:**
1. Calls `ControlClass::Draw_Me(forced)`
2. Hides mouse if over `SeenBuff`
3. Draws body: `BOXSTYLE_GREEN_DOWN` filled box
4. Calls `Draw_Thumb()`
5. Shows mouse

**Else:** Delegates to `GaugeClass::Draw_Me()`.

### Peer_To_Peer(flags, key, whom)
**Handles button peer events** - Called when `PlusGadget`/`MinusGadget` triggered.

- `LEFTRELEASE` on `PlusGadget` → `Step(false)` (down)
- `LEFTRELEASE` on `MinusGadget` → `Step(true)` (up)

## Peer pattern
- `PlusGadget`/`MinusGadget` are `ShapeButtonClass` peers
- `Make_Peer(*this)` + `Add(*this)` links them
- Button release → `Peer_To_Peer` → `Step()`

## Orientation
- `IsHorizontal = (Width > Height)` (from `GaugeClass`)
- Affects: mouse coordinate, thumb geometry, draw params

## Inputs, outputs and side effects

**State mutations:**
- `CurValue`, `ThumbStart`, `ThumbSize` (via `Recalc_Thumb`)
- Button peer list
- Redraw flags

**External systems affected:**
- Linked list box (`ListClass` sync via peer)
- Mouse cursor (hidden during draw)
- Rendering (`Draw_Box`)

## Calls into / called by

**Called by:**
- `ListClass` (scroll sync via peer)
- UI input loop → `Action()`
- Dialog rendering → `Draw_Me()`

**Calls into:**
- `GaugeClass` (base value/max logic)
- `ControlClass::Action/Draw_Me`
- `ShapeButtonClass` (buttons)
- `Fixed_To_Cardinal` / `Cardinal_To_Fixed` (proportional math)
- `Draw_Box` (rendering)
- `Get_Mouse_X/Y` (input)
- `Conditional_Hide/Show_Mouse`

## Related original files
- **SLIDER.H** - Class declaration
- **GAUGE.H/.CPP** - GaugeClass base (CurValue, MaxValue)
- **CONTROL.H/.CPP** - ControlClass (peer, ID)
- **BUTTON.H/.CPP** - ShapeButtonClass (Plus/Minus gadgets)
- **LIST.H/.CPP** - ListClass (consumer)
- **BOX.H/.CPP** - Draw_Box styles
- **COORD.H** - Fixed_To_Cardinal

## Rust port mapping
**Crates:** `cnc-ui`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-ui/src/gadget/slider.rs` - SliderClass
- `cnc-ui/src/gadget/gauge.rs` - GaugeClass base
- `cnc-ui/src/gadget/list.rs` - ListClass integration

**Not yet ported:**
- Proportional thumb sizing (Fixed_To_Cardinal math)
- Peer button pattern (Make_Peer + Peer_To_Peer)
- Horizontal/vertical geometry
- Minimum thumb size (4px)
- Mouse click regions (before/on/after thumb)
- Bump (thumb-size) vs Step (1 unit)
- Green raised/down box styles
- Mouse hide/show during draw
- BelongToList flag (embedded vs standalone)

## Edge cases and compatibility notes

1. **Thumb size minimum**: 4 pixels (enforced in `Recalc_Thumb`)
2. **Value clamping**: `CurValue` max = `MaxValue - Thumb`
3. **Thumb size clamp**: `1..MaxValue`
4. **Click regions**: Before thumb → up; On thumb → drag; After thumb → down
5. **Peer buttons**: Only created when `!BelongToList`
6. **Button positions**: Plus at right/bottom (+2), Minus at left/top (-6)
7. **Orientation auto**: Based on `Width > Height`
8. **Colorized=false**: Single-color thumb (not gradient)
9. **Mouse hide**: Only when `LogicPage == SeenBuff`
10. **Green style**: `BOXSTYLE_GREEN_RAISED` thumb, `BOXSTYLE_GREEN_DOWN` body
11. **Action comment**: Parent `GaugeClass::Action` should remove `LEFTRELEASE` check

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SLIDER.CPP at f1f0d42
