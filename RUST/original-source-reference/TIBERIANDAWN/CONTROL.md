# PATH/FILE
original/TIBERIANDAWN/CONTROL.CPP

## Purpose
UI control gadget base class for Tiberian Dawn. Extends `GadgetClass` with ID-based action dispatch, peer notification, and cooperative drawing for composite widgets (e.g., slider + list box).

## Where it fits in the game
UI framework - base class for interactive controls (buttons, sliders, checkboxes). Used by `ControlDialogClass`, `ListClass`, `SliderClass`, etc. Integrates with `GadgetClass` event system and `Input()` processing loop.

## Classes, structs, enums and important data

### ControlClass : GadgetClass
**Interactive control base** - Adds ID and peer linking to rectangular gadget.

**Key data members:**
- `ID` (unsigned) - Control identifier; 0 = no pseudo-key returned
- `Peer` (GadgetClass*) - Linked gadget for notification (e.g., slider → list box)

**Inherited from GadgetClass:**
- `X, Y, W, H` - Pixel bounds
- `Flags` - Recognized input events
- `Sticky` - Mouse capture behavior

### KN_BUTTON flag
**Pseudo-key modifier** - ORed with `ID` to create `KeyNumType` return value.

## Functions and methods

### ControlClass(id, x, y, w, h, flags, sticky)
**Constructor** - Initializes base `GadgetClass`, stores `ID`, clears `Peer`.

### Action(flags, key) → int (abort flag)
**Event handler** - Called when gadget receives matching input event.

1. **Peer notification**: If `Peer` exists → `Peer->Peer_To_Peer(flags, key, *this)`
2. **ID mapping**: If `flags != 0` (recognized event):
   - `ID != 0` → `key = ID | KN_BUTTON`
   - `ID == 0` → `key = KN_NONE`
3. Returns `GadgetClass::Action(flags, key)` (typically 0 = continue, 1 = abort)

### Make_Peer(gadget)
**Links peer gadget** - Sets `Peer = &gadget` for action/draw notifications.

### Get_ID() → unsigned
**Returns control ID** - Used by `Extract_Gadget()` for dialog traversal.

### Draw_Me(forced) → int (redrawn flag)
**Cooperative drawing** - Notifies peer before base draw.
1. If `Peer` → `Peer->Draw_Me()`
2. Returns `GadgetClass::Draw_Me(forced)`

## Inputs, outputs and side effects

**State mutations:**
- Peer gadget receives `Peer_To_Peer()` and `Draw_Me()` calls
- Return key code modified to `ID | KN_BUTTON`

**External systems affected:**
- Input processing loop (`Input()` → `GadgetClass::Action()`)
- Dialog rendering (`ControlDialogClass::Draw_Me()`)
- Composite widget sync (slider ↔ list box)

## Calls into / called by

**Called by:**
- `GadgetClass` event dispatch (mouse/keyboard)
- `ControlDialogClass` gadget list iteration
- Parent composite controls (slider calls list box)

**Calls into:**
- `GadgetClass::Action/Draw_Me`
- `Peer->Peer_To_Peer/Draw_Me` (if linked)

## Related original files
- **CONTROL.H** - Class declaration
- **GADGET.H/.CPP** - GadgetClass base, event flags
- **SLIDER.H/.CPP** - SliderClass (uses peer for list box)
- **LIST.H/.CPP** - ListClass (peer target)
- **CHECKBOX.H/.CPP** - CheckBoxClass
- **RADIO.H/.CPP** - RadioClass
- **CONTROLDLG.H/.CPP** - ControlDialogClass
- **INPUT.H/.CPP** - Input() main loop

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-ui`

**Mapped modules:**
- `cnc-ui/src/gadget/control.rs` - ControlClass
- `cnc-ui/src/gadget/gadget.rs` - GadgetClass base
- `cnc-ui/src/gadget/slider.rs` - SliderClass (peer pattern)
- `cnc-ui/src/gadget/list.rs` - ListClass

**Not yet ported:**
- ID → `KN_BUTTON` pseudo-key mapping
- Peer notification (`Peer_To_Peer`)
- Cooperative `Draw_Me` propagation
- Sticky mouse capture (from GadgetClass)
- Event flag filtering (from GadgetClass)

## Edge cases and compatibility notes

1. **ID = 0**: Special case - returns `KN_NONE` instead of pseudo-key; used for non-action controls
2. **Peer pattern**: Enables composite widgets (slider+list) without inheritance coupling
3. **Sticky behavior**: Inherited from `GadgetClass`; captures mouse until release
4. **Action return**: Propagates `GadgetClass::Action` result (abort flag for modal dialogs)
5. **Draw_Me order**: Peer drawn first (background), then self (foreground)
6. **No virtual destructor**: Relies on `GadgetClass` cleanup
7. **KN_BUTTON**: High-bit flag distinguishes control IDs from real keystrokes

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/CONTROL.CPP at f1f0d42
