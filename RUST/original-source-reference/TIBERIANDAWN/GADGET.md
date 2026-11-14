# PATH/FILE
original/TIBERIANDAWN/GADGET.CPP

## Purpose
UI gadget base class and input processing system for Tiberian Dawn. Provides the foundation for all interactive UI elements (buttons, sliders, checkboxes, lists) with mouse/keyboard event handling, focus management, sticky capture, and doubly-linked list organization.

## Where it fits in the game
UI framework - Root of the gadget hierarchy. All interactive controls (`ControlClass`, `ShapeButtonClass`, `TextButtonClass`, `CheckBoxClass`, `SliderClass`, `ListClass`, etc.) inherit from `GadgetClass`. The `Input()` function is the main UI event pump called per frame.

## Classes, structs, enums and important data

### GadgetClass : LinkClass (doubly-linked list)
**Base UI element** - Rectangular region with event flags.

**Key data members:**
- `X, Y, Width, Height` (int) - Pixel bounds
- `Flags` (unsigned) - Recognized event flags
- `IsSticky` (int) - Mouse capture mode
- `IsDisabled` (bool) - Input ignored
- `IsToRepaint` (bool) - Dirty flag for rendering

**Static/global state:**
- `StuckOn` (GadgetClass*) - Currently captured gadget (sticky)
- `LastList` (GadgetClass*) - Last processed list (for forced redraw)
- `Focused` (GadgetClass*) - Keyboard focus target

### Event flags (Flags)
| Flag | Meaning |
|------|---------|
| `LEFTPRESS` | Left mouse down |
| `LEFTRELEASE` | Left mouse up |
| `RIGHTPRESS` | Right mouse down |
| `RIGHTRELEASE` | Right mouse up |
| `LEFTHELD` | Left mouse held |
| `LEFTUP` | Left mouse up (polled) |
| `RIGHTHELD` | Right mouse held |
| `RIGHTUP` | Right mouse up (polled) |
| `KEYBOARD` | Keyboard event |

**Sticky behavior:** If `IsSticky`, automatically adds `LEFTPRESS|LEFTRELEASE` to `Flags`.

## Functions and methods

### GadgetClass(x, y, w, h, flags, sticky)
**Constructor** - Initializes bounds, flags, sticky. Adds `LEFTPRESS|LEFTRELEASE` if sticky.

### ~GadgetClass()
**Destructor** - Clears focus if held.

### Clicked_On(key, flags, mousex, mousey) → int
**Hit-test + action dispatch.**

1. `flags &= Flags` (filter to recognized events)
2. Triggers if:
   - `this == StuckOn` (sticky capture)
   - `flags & KEYBOARD` (keyboard event)
   - `flags && mouse in bounds` (mouse event)
3. Calls `Action(flags, key)`, returns result

### Enable() / Disable()
**Toggles input processing** - Sets `IsDisabled`, `IsToRepaint=true`, clears focus.

### Remove() → GadgetClass*
**Unlinks from list** - Clears focus, calls `LinkClass::Remove()`.

### Get_Next() / Get_Prev() → GadgetClass*
**List navigation** - Casts `LinkClass` pointers.

### Delete_List()
**Destroys entire list** - Walks to head, deletes all nodes (clears focus each).

### Action(flags, key) → int
**Base action handler** - If `flags != 0`: `IsToRepaint=true`, `Sticky_Process(flags)`, returns true (abort further processing).

### Draw_Me(forced) → int
**Render gate** - Returns true if `forced || IsToRepaint`, clears `IsToRepaint`.

### Draw_All(forced)
**Forces full list redraw** - Iterates list calling `Draw_Me(forced)`.

### Input() → KeyNumType
**Main UI event pump** - Called per frame per gadget list.

**Flow:**
1. **List change detection**: If `LastList != this` → `forced=true`, clears `StuckOn`/`Focused`
2. **Keyboard**: `Keyboard::Check()` → `Keyboard::Get()`
3. **Mouse coords**: Uses `MouseQX/MouseQY` for clicks, `Get_Mouse_X/Y()` otherwise
4. **Flag decoding**: `KN_LMOUSE`→`LEFTPRESS`, `KN_RMOUSE`→`RIGHTPRESS`, `KN_RLSE_BIT`→`LEFTRELEASE`/`RIGHTRELEASE`, else `KEYBOARD`
5. **Mouse held state**: Polls `Keyboard::Down(KN_LMOUSE/RMOUSE)` → `LEFTHELD/RIGHTHELD` or `LEFTUP/RIGHTUP`
6. **Sticky processing**: If `StuckOn` → only that gadget gets events
7. **Focus routing**: If `Focused && KEYBOARD` → only focused gadget gets keyboard
8. **List traversal**: For each enabled gadget:
   - `Draw_Me(forced)`
   - `Clicked_On()` → if true: `Draw_Me(false)`, break
9. Returns key

### Extract_Gadget(id) → ControlClass*
**Finds gadget by ID** - Linear search, returns first `ControlClass` with matching `Get_ID()`.

### Flag_To_Redraw()
**Marks dirty** - `IsToRepaint = true`.

### Sticky_Process(flags)
**Sticky capture logic:**
- `IsSticky && LEFTPRESS` → `StuckOn = this`
- `StuckOn == this && LEFTRELEASE` → `StuckOn = 0`

### Set_Focus() / Clear_Focus() / Has_Focus()
**Keyboard focus management:**
- `Set_Focus()`: Clears previous focus, sets `Flags|=KEYBOARD`, `Focused=this`
- `Clear_Focus()`: If `Focused==this`, clears `KEYBOARD` flag, `Focused=0`
- `Has_Focus()`: `this == Focused`

### Is_List_To_Redraw() → int
**Checks entire list for dirty flags** - Returns true if any `IsToRepaint`.

## Editor extension
Under `SCENARIO_EDITOR`: `KN_K` → screenshot (PCX) via `Write_PCX_File()`.

## Inputs, outputs and side effects

**State mutations:**
- `StuckOn`, `Focused`, `LastList` (static)
- Per-gadget: `IsDisabled`, `IsToRepaint`, `Flags`, focus flag
- List links (via `LinkClass`)

**External systems affected:**
- Keyboard (`Keyboard::Check/Get/Down`)
- Mouse (`Get_Mouse_X/Y`, `MouseQX/Y`)
- Rendering (`Draw_Me` → derived render)
- Editor screenshot (`Write_PCX_File`)

## Calls into / called by

**Called by:**
- Main loop → `GadgetList.Input()` per frame
- Dialogs → `Set_Focus()`, `Enable/Disable()`, `Draw_All()`
- Derived controls → override `Action()`, `Draw_Me()`

**Calls into:**
- `Keyboard` class
- Mouse globals (`Get_Mouse_X/Y`, `MouseQX/Y`)
- `LinkClass` (list management)
- Derived `Action()`, `Draw_Me()`
- `Write_PCX_File` (editor)

## Related original files
- **GADGET.H** - Class declaration, event flags
- **CONTROL.H/.CPP** - ControlClass (ID + peer)
- **BUTTON.H/.CPP** - ShapeButtonClass, TextButtonClass
- **CHECKBOX.H/.CPP** - CheckBoxClass
- **SLIDER.H/.CPP** - SliderClass
- **LIST.H/.CPP** - ListClass
- **RADIO.H/.CPP** - RadioClass
- **CONTROLDLG.H/.CPP** - ControlDialogClass (dialog gadget list)
- **KEYBOARD.H/.CPP** - Keyboard input
- **MOUSE.H/.CPP** - Mouse globals
- **LINK.H** - LinkClass (doubly-linked list base)

## Rust port mapping
**Crates:** `cnc-ui`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-ui/src/gadget/gadget.rs` - GadgetClass, event system
- `cnc-ui/src/gadget/list.rs` - Doubly-linked list (LinkClass)
- `cnc-ui/src/gadget/focus.rs` - Focus management
- `cnc-ui/src/gadget/sticky.rs` - Sticky capture
- `cnc-tiberian-dawn/src/ui/td_gadgets.rs` - TD-specific controls

**Not yet ported:**
- Doubly-linked list with `LinkClass` base
- Static `StuckOn`/`Focused`/`LastList` global state
- Event flag bitmask system
- Sticky mouse capture (exclusive processing)
- Keyboard focus routing
- Forced redraw on list change (`LastList`)
- Mouse coordinate sources (queued vs current)
- Keyboard focus flag injection (`Flags|=KEYBOARD`)
- Editor screenshot hotkey (`KN_K`)

## Edge cases and compatibility notes

1. **Sticky capture**: Only one gadget at a time (`StuckOn` static); mouse release releases
2. **Focus routing**: Keyboard events bypass hit-test if `Focused` set
3. **List change detection**: `LastList` forces full redraw on first `Input()` call after list switch
4. **Mouse coordinates**: Click events use `MouseQX/Y` (position at click time); polling uses current position
5. **Flag priority**: `KEYBOARD` flag injected by `Set_Focus()`; allows non-mouse gadgets to receive keys
6. **Sticky auto-flags**: Constructor adds `LEFTPRESS|LEFTRELEASE` if `IsSticky`
7. **Disabled gadgets**: Skipped in `Input()` traversal but remain in list
8. **Delete_List walks to head**: Ensures all nodes deleted even if mid-list
9. **Extract_Gadget**: Only `ControlClass` derivatives have IDs; returns first match
10. **Focus clear on delete**: `Remove()` and `Delete_List()` call `Clear_Focus()`
11. **Draw_All**: Used by dialogs to force full refresh
12. **Editor hotkey**: `KN_K` → PCX screenshot (only in `SCENARIO_EDITOR` build)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/GADGET.CPP at f1f0d42
