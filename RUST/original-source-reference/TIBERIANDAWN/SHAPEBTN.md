# PATH/FILE
original/TIBERIANDAWN/SHAPEBTN.CPP

## Purpose
Shape-based button control for Tiberian Dawn UI. Provides a button whose imagery comes from a shape file (SHP) with multiple frames for different states (up, down, disabled, on/off). Used for icon-based buttons like sidebar mode buttons (repair, sell, upgrade), radar toggle, and scroll arrows.

## Where it fits in the game
UI control - Image-based button used for sidebar mode buttons (Repair, Sell, Upgrade, Zoom), list box scroll arrows, and other icon-only buttons. Inherits `ToggleClass` → `ControlClass` → `GadgetClass`. Simpler than `TextButtonClass` - no text rendering, pure shape frames.

## Classes, structs, enums and important data

### ShapeButtonClass : ToggleClass
**Shape-frame button** - Visual states mapped to SHP frames.

**Key data members:**
- `ShapeData` (void const*) - Pointer to SHP file data
- `ReflectButtonState` (bool) - Frame selection mode
  - `false`: `UP_SHAPE`/`DOWN_SHAPE` based on `IsPressed`
  - `true`: Frame = `IsOn` (0/1 for on/off)

**Inherited from ToggleClass:**
- `IsOn` (bool) - Toggle state
- `IsPressed` (bool) - Mouse held down
- `IsDisabled` (bool) - Disabled state
- `IsSticky` = true (captures mouse)
- `IsToggleType` = true (click toggles)
- `ReflectButtonState` (bool) - Frame selection mode

**Shape frame constants:**
| Constant | Frame index | State |
|----------|-------------|-------|
| `UP_SHAPE` | 0 | Normal (not pressed) |
| `DOWN_SHAPE` | 1 | Pressed (mouse held) |
| `DISABLED_SHAPE` | 2 | Disabled |

**SHP requirements:** All frames same size; first frame defines `Width`/`Height`.

## Functions and methods

### ShapeButtonClass()
**Default constructor** - `ToggleClass(0,0,0,0,0)`, `ReflectButtonState=false`.

### ShapeButtonClass(id, shape, x, y)
**Main constructor** - Shape-based button.
1. `ToggleClass(id, x, y, 0, 0)`
2. `ReflectButtonState = false`
3. `Set_Shape(shape)` - Sets `ShapeData`, extracts `Width`/`Height` from first frame

### Set_Shape(data)
**Updates shape data** - Extracts dimensions from first frame.
- `ShapeData = data`
- `Width = Get_Build_Frame_Width(ShapeData)`
- `Height = Get_Build_Frame_Height(ShapeData)`

### Draw_Me(forced) → int
**Renders current frame** based on state.

**Frame selection logic:**
| Condition | Frame |
|-----------|-------|
| `IsDisabled` | `DISABLED_SHAPE` (2) |
| `!ReflectButtonState && IsPressed` | `DOWN_SHAPE` (1) |
| `!ReflectButtonState && !IsPressed` | `UP_SHAPE` (0) |
| `ReflectButtonState` | `IsOn` (0 or 1) |

**Rendering:**
1. `ControlClass::Draw_Me(forced)` gate
2. Hides mouse over bounds
3. `CC_Draw_Shape(ShapeData, shapenum, X, Y, WINDOW_MAIN, SHAPE_NORMAL)`
4. Shows mouse

### Toggle behavior (inherited)
- `IsSticky = true` (mouse capture until release)
- `IsToggleType = true` (click toggles `IsOn`)
- `ReflectButtonState` controls frame mapping
- Click returns `ID | KN_BUTTON`

## Shape frame mapping modes

### Mode 1: `ReflectButtonState = false` (default)
| State | Frame |
|-------|-------|
| Normal | `UP_SHAPE` (0) |
| Pressed | `DOWN_SHAPE` (1) |
| Disabled | `DISABLED_SHAPE` (2) |
| `IsOn` ignored for frame selection |

### Mode 2: `ReflectButtonState = true`
| State | Frame |
|-------|-------|
| Off (`IsOn=false`) | Frame 0 |
| On (`IsOn=true`) | Frame 1 |
| Disabled | Frame 2 |
| Pressed uses toggle state |

## Inputs, outputs and side effects

**State mutations:**
- `IsOn` toggled on click (via ToggleClass)
- `IsPressed` during mouse hold
- `ShapeData` pointer (set once, not owned)

**External systems affected:**
- Sidebar mode toggles (repair/sell/upgrade)
- List box scroll arrows
- Radar toggle

## Calls into / called by

**Called by:**
- `SIDEBAR.CPP` (Repair/Sell/Upgrade/Zoom buttons)
- `LIST.CPP` (UpGadget/DownGadget scroll arrows)
- `MPLAYER.CPP` (dialog buttons)
- Main loop → `Input()` → `Action()` → toggle

**Calls into:**
- `ToggleClass::Action()` (toggle logic)
- `ControlClass::Draw_Me()` (render gate)
- `CC_Draw_Shape` (SHP rendering)
- `Get_Build_Frame_Width/Height` (SHP metrics)
- `Conditional_Hide/Show_Mouse()`

## Related original files
- **SHAPEBTN.H** - Class declaration
- **TOGGLE.H/.CPP** - ToggleClass base
- **CONTROL.H/.CPP** - ControlClass (ID, peer)
- **BUTTON.H/.CPP** - Button base classes
- **TEXTBTN.H/.CPP** - TextButtonClass (text alternative)
- **CHECKBOX.H/.CPP** - CheckBoxClass (similar toggle)
- **SHAPE.H/.CPP** - CC_Draw_Shape, Get_Build_Frame_Width/Height
- **SIDEBAR.H/.CPP** - Sidebar buttons
- **LIST.H/.CPP** - List scroll arrows

## Rust port mapping
**Crates:** `cnc-ui`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-ui/src/gadget/shape_button.rs` - ShapeButtonClass
- `cnc-ui/src/gadget/toggle.rs` - ToggleClass base
- `cnc-ui/src/gadget/control.rs` - ControlClass
- `cnc-render/src/shape.rs` - CC_Draw_Shape, SHP frame extraction

**Not yet ported:**
- SHP frame extraction (`Get_Build_Frame_Width/Height`)
- Frame index constants (`UP_SHAPE`, `DOWN_SHAPE`, `DISABLED_SHAPE`)
- Two frame selection modes (`ReflectButtonState`)
- Toggle state visual reflection (`ReflectButtonState`)
- Mouse capture via `IsSticky`
- SHP data pointer ownership (external)
- `CC_Draw_Shape` with `SHAPE_NORMAL` flag

## Edge cases and compatibility notes

1. **SHP frame uniformity**: All frames must be same size (first frame defines button bounds)
2. **Frame count**: Minimum 3 frames (up, down, disabled); 4th for toggle-on if `ReflectButtonState`
3. **Default mode**: `ReflectButtonState=false` uses press/release frames
4. **Toggle mode**: `ReflectButtonState=true` maps `IsOn` to frame 0/1
5. **Disabled override**: Always shows `DISABLED_SHAPE` regardless of other state
6. **Dimensions from SHP**: `Width`/`Height` auto-set from first frame
7. **Mouse bounds**: Uses `X, Y, Width, Height` for hit-test and mouse hide
6. **No text rendering**: Pure shape frames - no text support
7. **Peer pattern**: Used as peers for `SliderClass` (scroll arrows) and `ListClass` (scroll arrows)
8. **SHP data ownership**: `ShapeData` is external pointer (from `MixFileClass::Retrieve`)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SHAPEBTN.CPP at f1f0d42
