# PATH/FILE
original/TIBERIANDAWN/SCROLL.CPP

## Purpose
Map scrolling system for Tiberian Dawn. Handles edge-scrolling, mouse-driven scrolling with inertia, autoscroll toggle, and integration with mouse cursor shapes for scroll direction feedback.

## Where it fits in the game
Input/rendering bridge - `ScrollClass` is used by `MouseClass::AI()` per frame. Manages tactical map viewport movement via `Scroll_Map()` with configurable rate, inertia, and free-scroll option.

## Classes, structs, enums and important data

### ScrollClass
**Scroll controller** - Integrated into `MouseClass` (non-editor build).

**Key data members:**
- `IsAutoScroll` (bool) - Autoscroll enabled (mouse over edge scrolls without click)
- `Counter` (CountDownTimerClass) - Scroll delay timer (static, `SCROLL_DELAY=1`)
- `Inertia` (int) - Current inertia level (0-8, higher = faster)

### Static rate table
```cpp
static int _rate[9] = {
    0x01C0, 0x0180, 0x0140, 0x0100,
    0x00C0, 0x0080, 0x0040, 0x0020, 0x0010
};
// Index = rate (0-8), value = leptons per scroll step
```

### Scroll regions
- **Edges**: `x==0`, `x==width-1`, `y==0`, `y==height-1`
- **No-scroll zones** (when `Special.IsScrollMod`): Top edge, `x in (3, 80)` and `x in (width-80, width-3)` (EVA/sidebar areas)

## Functions and methods

### ScrollClass::ScrollClass()
**Constructor** - Initializes defaults.
- `IsAutoScroll = true`
- `Counter.Set(SCROLL_DELAY)`, `Counter.Start()`
- `Inertia = 0`

### ScrollClass::AI(input, x, y)
**Per-frame scroll processing** - Called from `MouseClass::AI()`.

**Current implementation (mostly disabled `#if 0`):**
1. Delegates to `HelpClass::AI(input, x, y)` (only active code)

**Disabled logic (original behavior):**
1. **Rubber band check**: Skip if `IsRubberBand`
2. **No-scroll zones**: Top edge over EVA (80px) and sidebar areas
3. **Edge detection**: Mouse at screen edges → `player_scrolled=true`
4. **Coordinate adjustment**: Emphasizes cardinal over diagonal:
   - `altx/alty` compressed near center, expanded near edges
5. **Direction calculation**: `Desired_Facing256(center_x, center_y, altx, alty)` → 256-dir
6. **Rate control**: `rate = 8 - Inertia`, minimum `Options.ScrollRate+1`
7. **Right-click boost**: `rate = Bound(rate+1, 4, 8)`
8. **Free scroll option**: `Options.IsFreeScroll` forces 8-facing
9. **Scroll attempt**: `Scroll_Map(direction, _rate[rate]/2, false)` (test)
   - Fail → `MOUSE_NO_N+control` cursor
   - Success → `MOUSE_N+control` cursor
10. **Actual scroll**: If `LMOUSE` down or `IsAutoScroll`:
    - `Scroll_Map(direction, _rate[rate], true)`
    - On `Counter` expiry + `player_scrolled`: `Inertia++`
11. **Inertia decay**: If no player scroll and `Counter` expired: `Inertia--` (min 0)

### ScrollClass::Set_Autoscroll(control) → bool
**Toggles autoscroll mode.**

| Control | Action |
|---------|--------|
| `-1` | Toggle |
| `0` | Off |
| `1` | On |

Returns previous `IsAutoScroll` value.

## Inputs, outputs and side effects

**State mutations:**
- `Inertia` (0-8, accelerates on sustained scroll, decays on release)
- `Counter` (delay timer)
- `IsAutoScroll` (user preference)

**External systems affected:**
- Map viewport (`Scroll_Map`)
- Mouse cursor (`Override_Mouse_Shape` with `MOUSE_N/NO_N` variants)
- Options (`Options.ScrollRate`, `Options.IsFreeScroll`)

## Calls into / called by

**Called by:**
- `MouseClass::AI()` per frame

**Calls into:**
- `Scroll_Map(direction, distance, execute)` - Map viewport movement
- `Desired_Facing256` / `Dir_Facing` / `Facing_Dir` - Direction math
- `Override_Mouse_Shape` - Cursor feedback
- `Options.ScrollRate` / `IsFreeScroll` - User settings
- `Keyboard::Down(KN_LMOUSE/RMOUSE)` - Button state
- `Counter.Time()/Set()` - Delay timer
- `HelpClass::AI` - Help text system

## Related original files
- **SCROLL.H** - Class declaration
- **MOUSE.H/.CPP** - MouseClass (calls ScrollClass::AI)
- **MAP.H/.CPP** - Scroll_Map implementation
- **OPTIONS.H/.CPP** - ScrollRate, IsFreeScroll
- **SPECIAL.H** - IsScrollMod
- **TIMER.H** - CountDownTimerClass
- **HELP.H** - HelpClass

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-ui`, `cnc-core`

**Mapped modules:**
- `cnc-ui/src/scroll.rs` - ScrollClass
- `cnc-tiberian-dawn/src/input/td_scroll.rs` - TD-specific rates/zones
- `cnc-core/src/input/mouse.rs` - Mouse integration

**Not yet ported:**
- Inertia-based acceleration (0→8 over sustained edge hover)
- Cardinal-biased coordinate adjustment
- No-scroll zones (EVA/sidebar top edge)
- Free-scroll vs 8-direction option
- Autoscroll toggle (clickless edge scroll)
- Rate table (`_rate[9]`) with right-click boost
- Mouse cursor direction feedback (`MOUSE_N/NO_N` + 8 facings)
- Counter-based inertia decay

## Edge cases and compatibility notes

1. **Mostly disabled**: `#if 0` wraps original logic; only `HelpClass::AI` active
2. **Inertia range**: 0-8, clamps at boundaries
3. **Rate indexing**: `_rate[rate]` where `rate` = 0-8
4. **Right-click**: Increases rate by 1 (max 8)
4. **Options.ScrollRate**: Minimum rate floor (0-7)
5. **Free scroll**: `Options.IsFreeScroll` disables 8-direction snap
6. **Autoscroll**: Scrolls on hover without mouse button
7. **No-scroll zones**: Protects EVA (left) and sidebar (right) top edge
8. **Cursor feedback**: `MOUSE_N+control` (can scroll) vs `MOUSE_NO_N+control` (blocked)
9. **Counter delay**: `SCROLL_DELAY=1` frame between inertia changes
10. **Player_scrolled flag**: Only accelerates inertia on actual player input

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SCROLL.CPP at f1f0d42
