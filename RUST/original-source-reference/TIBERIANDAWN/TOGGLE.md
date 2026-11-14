# PATH/FILE
original/TIBERIANDAWN/TOGGLE.CPP

## Purpose
Toggle button base class for Tiberian Dawn UI. Provides the core toggle/press state machine used by `TextButtonClass`, `ShapeButtonClass`, `CheckBoxClass`, and `SliderClass` buttons. Handles mouse press/release, sticky capture, hover press tracking, and toggle state management.

## Where it fits in the game
UI control foundation - Base class for all toggle-style buttons. Inherits `ControlClass` → `GadgetClass`. Used by `TextButtonClass`, `ShapeButtonClass`, `CheckBoxClass` (via inheritance), and `SliderClass` (peer buttons).

## Classes, structs, enums and important data

### ToggleClass : ControlClass
**Toggle/press state machine** - Core button behavior.

**Key data members:**
- `IsPressed` (bool) - Mouse currently held down on button
- `IsOn` (bool) - Toggle state (on/off)
- `IsToggleType` (bool) - Enables toggle behavior (click toggles `IsOn`)
- `ReflectButtonState` (bool) - Visual reflects `IsOn` (used by `ShapeButtonClass`)

**Inherited from ControlClass:**
- `ID` (unsigned) - Control ID
- `Peer` (GadgetClass*) - Peer notification target
- `IsSticky` (bool) - Mouse capture mode

**Constructor defaults:**
- `IsSticky = true` (via `ControlClass` flags)
- `Flags = LEFTPRESS|LEFTRELEASE`
- `IsPressed = false`
- `IsOn = false`
- `IsToggleType = false` (must be enabled by derived class)
- `ReflectButtonState = false`

## Functions and methods

### ToggleClass(id, x, y, w, h)
**Constructor** - Initializes base `ControlClass` with sticky flags.
- `ControlClass(id, x, y, w, h, LEFTPRESS|LEFTRELEASE, true)`
- `IsPressed = false`
- `IsOn = false`
- `IsToggleType = false`

### Turn_On() / Turn_Off()
**Programmatic state control** - Sets `IsOn` and `Flag_To_Redraw()`.

### Action(flags, key) → int
**Core button state machine** - Handles press/release/hover/toggle.

**Flow:**

1. **Forced call (flags=0)**: Hover press tracking
   - If mouse over button and not pressed → `IsPressed=true`, redraw
   - If mouse not over and pressed → `IsPressed=false`, redraw

2. **Sticky processing**: `Sticky_Process(flags)` (capture/release)

3. **Left press**: `flags & LEFTPRESS`
   - `IsPressed = true`, redraw
   - Clears `LEFTPRESS` from flags
   - Calls `ControlClass::Action(flags, key)`
   - Returns `true` (consumes event, stops propagation)

4. **Left release**: `flags & LEFTRELEASE`
   - If was pressed:
     - If `IsToggleType`: `IsOn = !IsOn` (toggle)
     - `IsPressed = false`
   - Else: clears `LEFTRELEASE`

5. **Delegates to `ControlClass::Action()`** for ID return (`ID | KN_BUTTON`)

### Sticky_Process(flags) (inherited from GadgetClass)
- `IsSticky && LEFTPRESS` → `StuckOn = this` (capture)
- `StuckOn == this && LEFTRELEASE` → `StuckOn = 0` (release)

## State machine summary

| Event | IsPressed | IsOn (if ToggleType) | Visual |
|-------|-----------|---------------------|--------|
| Hover enter (forced) | false → true | - | Pressed |
| Hover leave (forced) | true → false | - | Raised |
| LEFTPRESS | false → true | - | Pressed |
| LEFTRELEASE (pressed) | true → false | false → true (if toggle) | Toggled/Raised |
| LEFTRELEASE (not pressed) | false | - | - |

## Toggle behavior (when `IsToggleType=true`)
- Click (press+release on same button) toggles `IsOn`
- `ReflectButtonState` (in `ShapeButtonClass`) maps `IsOn` to frame
- `CheckBoxClass` uses this for checked/unchecked X mark
- `TextButtonClass` uses for on/off text buttons

## Pressed tracking (for visual feedback)
- `IsPressed` tracks mouse hold
- Hover tracking via forced calls (no flags) updates `IsPressed` without click
- `IsSticky=true` captures mouse until release (exclusive processing)

## Inputs, outputs and side effects

**State mutations:**
- `IsPressed`, `IsOn` (toggle)
- `StuckOn` (static, via `Sticky_Process`)
- Redraw flags

**External systems affected:**
- Derived button rendering (pressed/raised/toggle visuals)
- Peer notification (`ControlClass::Action` → `Peer_To_Peer`)
- Event consumption (returns `true` to stop propagation)

## Calls into / called by

**Called by:**
- Derived classes: `TextButtonClass`, `ShapeButtonClass`, `CheckBoxClass`
- `GadgetClass::Input()` → `Clicked_On()` → `Action()`
- Peer gadgets → `Peer_To_Peer()` → `Action()`

**Calls into:**
- `ControlClass::Action()` (returns `ID | KN_BUTTON`)
- `Sticky_Process()` (sticky capture)
- `Flag_To_Redraw()` (visual update)
- `Get_Mouse_X/Y()` (hover tracking)

## Related original files
- **TOGGLE.H** - Class declaration
- **CONTROL.H/.CPP** - ControlClass base (ID, peer)
- **GADGET.H/.CPP** - GadgetClass base (sticky, flags)
- **TEXTBTN.H/.CPP** - TextButtonClass (text toggle)
- **SHAPEBTN.H/.CPP** - ShapeButtonClass (shape toggle)
- **CHECKBOX.H/.CPP** - CheckBoxClass (checkbox)
- **SLIDER.H/.CPP** - SliderClass (peer buttons)

## Rust port mapping
**Crates:** `cnc-ui`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-ui/src/gadget/toggle.rs` - ToggleClass
- `cnc-ui/src/gadget/control.rs` - ControlClass
- `cnc-ui/src/gadget/gadget.rs` - GadgetClass (sticky, StuckOn)

**Not yet ported:**
- Hover press tracking via forced `Action(0)` calls
- Sticky mouse capture (`StuckOn` static)
- Toggle state machine with `IsToggleType`
- `ReflectButtonState` for visual sync
- Pressed/hover/released visual states
- `LEFTPRESS` consumption (stops event propagation)
- `KN_BUTTON` ID return via `ControlClass::Action`
- Static `StuckOn` / `Focused` global state

## Edge cases and compatibility notes

1. **Forced calls**: `GadgetClass::Input()` calls `Action(0)` for hover press tracking
2. **LEFTPRESS consumption**: Returns `true` immediately, stops other buttons
3. **Toggle only on release**: `IsOn` flips only on `LEFTRELEASE` when pressed
4. **Sticky capture**: `IsSticky=true` → exclusive processing until `LEFTRELEASE`
5. **Hover press**: Tracks mouse over button without click (visual feedback)
6. **Peer notification**: `ControlClass::Action` calls `Peer->Peer_To_Peer()`
7. **ID return**: `ControlClass::Action` returns `ID | KN_BUTTON` for dialog handling
8. **IsToggleType default false**: Must be explicitly enabled by derived class
9. **ReflectButtonState**: Used by `ShapeButtonClass` for frame selection
10. **Forced call mouse bounds**: Uses unsigned comparison for bounds check

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TOGGLE.CPP at f1f0d42
