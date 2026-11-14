# PATH/FILE
original/TIBERIANDAWN/CHECKBOX.CPP

## Purpose
Checkbox control for Tiberian Dawn UI. Provires a toggleable box with X mark when checked. Used in options dialogs and settings panels. Inherits from `ToggleClass` → `ControlClass` → `GadgetClass`.

## Where it fits in the game
UI control - Used in options dialogs (Special_Dialog, OPTIONS.CPP) for boolean settings like "Separate Helipad", "Visible Target", "MCV Deploy", etc. Simple on/off toggle with visual feedback.

## Classes, structs, enums and important data

### CheckBoxClass : ToggleClass
**Boolean toggle with box + X mark.**

**Inherited from ToggleClass (via ControlClass):**
- `IsOn` (bool) - Current state
- `IsSticky` (bool) - Sticky behavior
- `IsToggleType` (bool) - Toggle mode
- `ReflectButtonState` (bool) - Visual reflects state
- `ID` (unsigned) - Control ID (returns `ID | KN_BUTTON` on action)

**Key data members:**
- `X, Y, Width, Height` - Pixel bounds
- `IsOn` - Checked state

## Functions and methods

### Draw_Me(forced) → int
**Renders checkbox** - Green down box with grey fill, X when checked.

1. Calls `ToggleClass::Draw_Me(forced)` gate
2. Hides mouse
3. Draws `BOXSTYLE_GREEN_DOWN` box outline
4. Fills interior with `DKGREY`
5. If `IsOn`: Draws black X (two diagonals):
   - `(X+1, Y+1)` to `(X+Width-2, Y+Height-2)`
   - `(X+Width-2, Y+1)` to `(X+1, Y+Height-2)`
6. Shows mouse
7. Returns true if rendered

## ToggleClass behavior (inherited)
- `Action()` toggles `IsOn` on click
- `IsToggleType = true` enables toggle behavior
- `ReflectButtonState = true` visual matches state
- `IsSticky = true` captures mouse until release

## Inputs, outputs and side effects

**State mutations:**
- `IsOn` toggled on click (via ToggleClass::Action)

**External systems affected:**
- Options dialog (special options)
- Network settings
- Gameplay modifiers

## Calls into / called by

**Called by:**
- Options dialog (`Special_Dialog`, `OPTIONS.CPP`)
- Settings panels
- Main loop → `Input()` → `Action()` → toggle

**Calls into:**
- `ToggleClass::Draw_Me()` (base gate)
- `Draw_Box()` (outline)
- `LogicPage->Fill_Rect/Draw_Line` (interior/X)
- `Conditional_Hide/Show_Mouse()`

## Related original files
- **CHECKBOX.H** - Class declaration
- **TOGGLE.H/.CPP** - ToggleClass base
- **CONTROL.H/.CPP** - ControlClass (ID, peer)
- **BUTTON.H/.CPP** - ShapeButtonClass (similar but no toggle)
- **SPECIAL.CPP** - Special_Dialog uses CheckBoxClass
- **OPTIONS.CPP** - Options dialog

## Rust port mapping
**Crates:** `cnc-ui`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-ui/src/gadget/checkbox.rs` - CheckBoxClass
- `cnc-ui/src/gadget/toggle.rs` - ToggleClass base
- `cnc-ui/src/gadget/control.rs` - ControlClass

**Not yet ported:**
- Toggle state machine (IsOn, IsSticky, IsToggleType)
- X mark rendering (two diagonal lines)
- Green down box + dark grey fill style
- Keyboard focus handling (inherited)
- ID → KN_BUTTON return on action

## Edge cases and compatibility notes

1. **Minimal override**: Only `Draw_Me` overridden; all logic in `ToggleClass`
2. **Visual style**: Green down box + dark grey fill + black X
3. **Inheritance chain**: CheckBoxClass → ToggleClass → ControlClass → GadgetClass
3. **Toggle behavior**: Click toggles `IsOn`; `ReflectButtonState` keeps visual in sync
4. **Sticky capture**: Mouse held processes only this gadget
5. **ID return**: Action returns `ID | KN_BUTTON` for dialog handling
6. **Used in Special_Dialog**: 12 checkboxes for game options
7. **No text rendering**: Label drawn separately by dialog (Fancy_Text_Print)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/CHECKBOX.CPP at f1f0d42
