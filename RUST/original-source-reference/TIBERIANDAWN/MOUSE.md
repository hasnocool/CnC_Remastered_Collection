# PATH/FILE
original/TIBERIANDAWN/MOUSE.CPP

## Purpose
Mouse cursor system for Tiberian Dawn. Handles animated cursor shapes, state-based overrides (scroll, deploy, attack, sell, repair, superweapons), and integration with map scrolling.

## Where it fits in the game
Input/rendering bridge - `MouseClass` is the main map handler (non-editor build). Called per-frame via `MouseClass::AI()` from main loop. Manages cursor animation timer, shape overrides, and delegates scrolling to `ScrollClass`.

## Classes, structs, enums and important data

### MouseClass
**Main mouse handler** (also map handler in non-editor build).

**Key data members:**
- `MouseShapes` (void const*) - Loaded MOUSE.SHP data
- `Timer` (CountDownTimerClass) - Animation frame timer
- `CurrentMouseShape` (MouseType) - Active override shape
- `NormalMouseShape` (MouseType) - Base shape (default MOUSE_NORMAL)
- `IsSmall` (bool) - Small cursor variant flag
- `Frame` (int) - Animation frame counter
- `VTable` - Virtual table pointer

### MouseStruct (MouseControl[MOUSE_COUNT])
**Cursor animation definition** per shape:

| Field | Purpose |
|-------|---------|
| `StartFrame` | Base frame index in SHP |
| `FrameCount` | Number of animation frames |
| `FrameRate` | Timer ticks per frame (0 = static) |
| `SmallFrame` | Small variant base frame (-1 = none) |
| `X` / `Y` | Hotspot offset |

### MouseType enum (partial)
| Cursor | Use case |
|--------|----------|
| `MOUSE_NORMAL` | Default arrow |
| `MOUSE_N`...`MOUSE_NW` | Directional scroll (8-way) |
| `MOUSE_NO_N`...`MOUSE_NO_NW` | No-scroll directional |
| `MOUSE_NO_MOVE` / `MOUSE_CAN_MOVE` | Move validity |
| `MOUSE_ENTER` | Cell entry |
| `MOUSE_DEPLOY` | MCV deploy |
| `MOUSE_CAN_SELECT` | Valid selection target |
| `MOUSE_CAN_ATTACK` | Valid attack target |
| `MOUSE_SELL_BACK` / `MOUSE_SELL_UNIT` | Sell mode |
| `MOUSE_REPAIR` / `MOUSE_NO_REPAIR` | Repair mode |
| `MOUSE_RADAR_CURSOR` | Minimap |
| `MOUSE_ION_CANNON` / `MOUSE_NUCLEAR_BOMB` / `MOUSE_AIR_STRIKE` | Superweapons |
| `MOUSE_DEMOLITIONS` | Demo charge |
| `MOUSE_AREA_GUARD` | Area guard |

## Functions and methods

### MouseClass::AI(input, x, y)
**Per-frame update** - Called from main loop.

1. **Animation timer**: If `control->FrameRate` and `Timer.Time() == 0`:
   - `Frame++`, `Frame %= FrameCount`
   - `Timer.Set(FrameRate)`
   - If not small or has small variant:
     - `baseframe = IsSmall ? SmallFrame : StartFrame`
     - `Extract_Shape(MouseShapes, baseframe + Frame)`
     - `Set_Mouse_Cursor(control->X, control->Y, shape)`
2. **Scroll delegation**: `ScrollClass::AI(input, x, y)`

### Override_Mouse_Shape(mouse, wwsmall) → bool
**Changes cursor shape** with animation reset.

1. If `control->SmallFrame == -1` → `wwsmall = false`
2. If first call or shape/size changed:
   - `startup = true`
   - `Timer.Set(control->FrameRate)`, `Frame = 0`
   - `baseshp = wwsmall ? SmallFrame : StartFrame`
   - `Set_Mouse_Cursor(control->X, control->Y, Extract_Shape(...))`
   - `CurrentMouseShape = mouse`, `IsSmall = wwsmall`
   - Returns true
3. Returns false (no change)

### Revert_Mouse_Shape()
**Restores normal shape** - `Override_Mouse_Shape(NormalMouseShape, false)`

### Set_Default_Mouse(mouse, size)
**Sets new base shape** - Updates `NormalMouseShape`, calls `Override_Mouse_Shape`

### Mouse_Small(wwsmall)
**Toggles small variant** - If changed, updates cursor immediately

### One_Time()
**One-time init** (called from `Init_Game`):
1. `ScrollClass::One_Time()`
2. Loads `MOUSE.SHP`: `RawFileClass` first, then `MixFileClass::Retrieve`
3. Captures `VTable` pointer

### Init_Clear(theater)
**Scenario reset** - Clears scroll, `IsSmall=false`, `NormalMouseShape=MOUSE_NORMAL`

### Constructor
- `CurrentMouseShape = MOUSE_NORMAL`
- `NormalMouseShape = MOUSE_NORMAL`
- `Timer.Start()`

## MouseControl table (key entries)
```cpp
{0, 1, 0, 86, 0, 0},      // MOUSE_NORMAL: frame 0, 1 frame, 0 rate, hotspot (0,0)
{1, 1, 0, -1, 15, 0},     // MOUSE_N: frame 1, static
{11, 1, 0, 27, 15, 12},   // MOUSE_NO_MOVE: frame 11
{10, 1, 0, 26, 15, 12},   // MOUSE_CAN_MOVE: frame 10
{119, 3, 4, 148, 15, 12}, // MOUSE_ENTER: 3 frames, rate 4
{53, 9, 4, -1, 15, 12},   // MOUSE_DEPLOY: 9 frames
{18, 8, 4, 140, 15, 12},  // MOUSE_CAN_ATTACK: 8 frames
{103, 16, 2, -1, 15, 12}, // MOUSE_ION_CANNON: 16 frames, rate 2
{96, 7, 4, -1, 15, 12},   // MOUSE_NUCLEAR_BOMB: 7 frames
{122, 3, 4, 127, 15, 12}, // MOUSE_DEMOLITIONS: 3 frames
```

## Inputs, outputs and side effects

**State mutations:**
- Cursor shape via `Set_Mouse_Cursor`
- Animation frame/timer
- Scroll state (via `ScrollClass`)

**External systems affected:**
- Renderer (mouse cursor display)
- Map scrolling (`ScrollClass::AI`)
- UI mode indication (deploy/attack/sell cursors)

## Calls into / called by

**Called by:**
- `Init_Game()` → `One_Time()`
- Main loop → `AI()` per frame
- UI code → `Override_Mouse_Shape()`/`Revert_Mouse_Shape()`/`Set_Default_Mouse()`
- Scenario load → `Init_Clear()`

**Calls into:**
- `ScrollClass::One_Time/AI/Init_Clear`
- `MixFileClass::Retrieve` / `RawFileClass` / `Load_Alloc_Data`
- `Extract_Shape` (SHP frame extraction)
- `Set_Mouse_Cursor` (WWLIB)
- `CountDownTimerClass::Time/Set`

## Related original files
- **MOUSE.H** - Class declaration, MouseStruct, MouseType enum
- **SCROLL.H/.CPP** - ScrollClass (map scrolling)
- **MIXFILE.H/.CPP** - MOUSE.SHP retrieval
- **SHAPE.H/.CPP** - Extract_Shape
- **TIMER.H** - CountDownTimerClass
- **FUNCTION.H** - Global `MouseClass Map` (non-editor)

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-render`, `cnc-ui`

**Mapped modules:**
- `cnc-render/src/cursor.rs` - MouseClass, animation
- `cnc-ui/src/cursor_states.rs` - MouseType, MouseControl table
- `cnc-tiberian-dawn/src/input/mouse.rs` - TD-specific cursors

**Not yet ported:**
- Animated cursor system with `CountDownTimerClass`
- Hotspot offsets (X,Y per cursor)
- Small variant support (IsSmall toggle)
- State-based override stack (NormalMouseShape + CurrentMouseShape)
- ScrollClass integration
- SHP frame extraction (`Extract_Shape`)
- VTable capture (C++ RTTI workaround)
- FrameRate = 0 for static cursors

## Edge cases and compatibility notes

1. **Non-editor build**: `MouseClass Map` is the global map handler (see GLOBALS.CPP)
2. **Editor build**: Uses `MapEditClass` instead
3. **Shape loading**: Raw file override first, then MIX
4. **Animation**: `FrameRate` in timer ticks; `Timer` uses system timer (constant rate)
5. **Frame indexing**: `baseframe + Frame` where `Frame` cycles `0..FrameCount-1`
6. **Small cursors**: Only shapes with `SmallFrame != -1` support small mode
7. **Override vs Default**: `Override_Mouse_Shape` temporary; `Set_Default_Mouse` changes base
8. **Revert**: Goes back to `NormalMouseShape` (not necessarily `MOUSE_NORMAL`)
9. **Scroll delegation**: `ScrollClass::AI` handles map edge scrolling
10. **VTable hack**: Captures vtable pointer for `Validate()` in cheat builds
11. **Timer.Start()**: Constructor starts animation timer immediately

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/MOUSE.CPP at f1f0d42
