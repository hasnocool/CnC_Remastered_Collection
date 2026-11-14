# PATH/FILE
original/TIBERIANDAWN/POWER.CPP

## Purpose
Power bar UI for Tiberian Dawn. Renders the sidebar power/drain indicator with animated bounce transitions, color-coded power levels (green/yellow/red), and drain threshold marker. Integrates with radar panel and sidebar.

## Where it fits in the game
Sidebar UI - Right side of screen below radar. Shows real-time power output vs drain with animated bars. Global `PowerClass Power` instance (GLOBALS.CPP). Called per-frame via `AI()` and `Draw_It()`.

## Classes, structs, enums and important data

### PowerClass : RadarClass
**Power bar controller** - Inherits radar functionality, adds power/drain visualization.

**Key data members:**
| Variable | Purpose |
|----------|---------|
| `RecordedPower` / `RecordedDrain` | Last synced values |
| `DesiredPowerHeight` / `DesiredDrainHeight` | Target bar heights |
| `PowerHeight` / `DrainHeight` | Current animated heights |
| `PowerBounce` / `DrainBounce` | Bounce animation counters (12→0) |
| `PowerDir` / `DrainDir` | Animation direction (-1/0/1) |
| `IsToRedraw` | Dirty flag |
| `PowX/Y/Width/Height` | Screen position/size |
| `PowLineSpace/Width` | Internal layout |

### PowerButtonClass : GadgetClass
**Invisible button over power bar** - Shows help text on hover.

### Constants
| Constant | Purpose |
|----------|---------|
| `POWER_STEP_LEVEL` | Power per "hundred" (100) |
| `POWER_STEP_FACTOR` | Diminishing returns factor |
| `PowHeight` | Total bar height in pixels |

### Shape assets
- `PowerShape` - `POWER.SHP` / `HPOWER.SHP` (drain marker)
- `PowerBarShape` - `PWRBAR.SHP` (bar segments)

### Color coding (power_color)
| Condition | Color index | Visual |
|-----------|-------------|--------|
| `Drain ≤ Power` | 0 | Green |
| `Power < Drain ≤ 2×Power` | 2 | Yellow |
| `Drain > 2×Power` | 4 | Red |

### Shape frames (PWRBAR.SHP)
| Frame | Purpose |
|-------|---------|
| 0 | Unfilled top |
| 1 | Unfilled bottom |
| 2/3/4 | Filled green/yellow/red (top) |
| 5/6/7 | Filled green/yellow/red (bottom) |

### Global references
- `PlayerPtr->Power` / `PlayerPtr->Drain` - Current values
- `PlayerPtr->Power_Fraction()` - Power ratio
- `Map.IsSidebarActive` - Visibility gate

## Functions and methods

### PowerClass()
**Constructor** - Initializes all heights/bounce to 0/-1.

### Init_Clear()
**Scenario reset** - Calls `RadarClass::Init_Clear()`, resets all heights/bounce to 0.

### One_Time()
**One-time init** (from `Init_Game`):
1. `RadarClass::One_Time()`
2. Calculates position: `PowX = SeenBuff.Width - Map.RadWidth`, below radar
3. `PowWidth = 8×factor`, `PowHeight = SeenBuff.Height - PowY`
4. Sets `PowerButton` bounds
5. Loads shapes: `HPOWER.SHP`/`POWER.SHP`, `PWRBAR.SHP` (hi-res aware)

### Draw_It(complete)
**Renders power bar** with clipping regions.

**Process:**
1. Locks `LogicPage`
2. If `Map.IsSidebarActive`:
   - Computes animated heights with bounce:
     - `power_height = PowerHeight + _modtable[PowerBounce] * PowerDir`
     - `drain_height = DrainHeight + _modtable[DrainBounce] * DrainDir`
   - Clamps to `[0, PowHeight-2]`
3. **Unfilled section** (clip above power):
   - Frames 0, 1 at `PowX, PowY` and `PowY+100`
4. **Filled section** (clip at power height):
   - Determines `power_color` from `PlayerPtr->Drain/Power` ratio
   - Frames `2+color` / `3+color` at adjusted Y
5. **Drain marker**: `PowerShape` frame 0 at `bottom - drain_height + 1`
6. Calls `RadarClass::Draw_It()`

### _modtable (bounce animation)
```cpp
static int _modtable[] = {
    0, -1, 0, 1, 0, -1, -2, -1, 0, 1, 2, 1, 0
};
```
13-frame bounce pattern: 0, -1, 0, 1, 0, -1, -2, -1, 0, 1, 2, 1, 0

### AI(input, x, y)
**Per-frame animation logic.**

1. Gates on `Map.IsSidebarActive`
2. **Power sync**: If `PlayerPtr->Power != RecordedPower`:
   - `DesiredPowerHeight = Power_Height(Power)`
   - `RecordedPower = Power`
   - `PowerBounce = 12`, sets `PowerDir` toward target
2. **Drain sync**: Same for `Drain`
3. **Bounce animation**:
   - If `Bounce > 0 && Height == Desired`: `IsToRedraw=true`, `Bounce--`
   - Else if `Height != Desired`: `Height += Dir`
4. **Change detection**: If heights changed → `IsToRedraw=true`

### Refresh_Cells(cell, list)
**Sidebar redraw hook** - If `REFRESH_SIDEBAR` in list, flags redraw.

### Power_Height(value) → int
**Converts power value to bar height** with diminishing returns.

```cpp
num = value / POWER_STEP_LEVEL
retval = 0
for lp = 0 to num-1:
    retval += ((PowHeight-2) - retval) / POWER_STEP_FACTOR
if value % POWER_STEP_LEVEL > 0:
    retval += ((((PowHeight-2)-retval) / POWER_STEP_FACTOR) * remainder) / POWER_STEP_LEVEL
return Bound(retval, 0, PowHeight-2)
```

- Each "hundred" power adds fraction of remaining space
- `POWER_STEP_FACTOR` controls diminishing returns

### PowerButtonClass::Action(flags, key) → int
**Hover help text** over power bar.

1. Returns false if sidebar inactive
2. `Map.Override_Mouse_Shape(MOUSE_NORMAL)`
3. Shows help text:
   - `Power_Fraction() < 256 && Power > 0` → `TXT_POWER_OUTPUT_LOW`
   - Else → `TXT_POWER_OUTPUT`

## Power color logic
```cpp
if (PlayerPtr->Drain > PlayerPtr->Power) power_color = 2;      // Yellow
if (PlayerPtr->Drain > PlayerPtr->Power * 2) power_color = 4;  // Red
else power_color = 0;                                           // Green
```

## Inputs, outputs and side effects

**State mutations:**
- Animated heights (`PowerHeight`, `DrainHeight`)
- Bounce counters (`PowerBounce`, `DrainBounce`)
- Synced values (`RecordedPower`, `RecordedDrain`)
- Redraw flags

**External systems affected:**
- Sidebar rendering (power bar visibility)
- Radar panel (inherited)
- Help text system (hover)

## Calls into / called by

**Called by:**
- `INIT.CPP` → `One_Time()`
- `Main_Game` cleanup → `Init_Clear()`
- Main loop → `AI()`, `Draw_It()`
- Map redraw → `Refresh_Cells()`
- Mouse hover → `PowerButtonClass::Action()`

**Calls into:**
- `RadarClass` (base methods)
- `PlayerPtr->Power/Drain/Power_Fraction()`
- `CC_Draw_Shape` (rendering)
- `LogicPage->Lock/Unlock` (clipping)
- `WindowList[WINDOW_CUSTOM]` (clip regions)
- `Bound` (clamping)
- `Map.Help_Text/Override_Mouse_Shape`

## Related original files
- **POWER.H** - Class declarations
- **RADAR.H/.CPP** - RadarClass base
- **SIDEBAR.H/.CPP** - Sidebar integration (`IsSidebarActive`)
- **MAP.H/.CPP** - MapClass (sidebar, radar)
- **PLAYER.H/.CPP** - PlayerPtr (Power, Drain)
- **MIXFILE.H/.CPP** - Shape loading
- **SIDEBAR.CPP** - `Init_Clear`/`One_Time` calls

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-ui`

**Mapped modules:**
- `cnc-ui/src/sidebar/power_bar.rs` - PowerClass
- `cnc-ui/src/sidebar/radar.rs` - RadarClass base
- `cnc-tiberian-dawn/src/ui/td_sidebar.rs` - TD layout

**Not yet ported:**
- Bounce animation with 13-frame `_modtable`
- Diminishing returns `Power_Height` calculation
- Three-color bar rendering (green/yellow/red)
- Clipping region split (filled/unfilled)
- Drain threshold marker (`PowerShape`)
- Hi-res shape variants (`HPOWER.SHP`)
- Help text on hover (`PowerButtonClass`)
- Sidebar active gating (`Map.IsSidebarActive`)

## Edge cases and compatibility notes

1. **Bounce frames**: 13-frame `_modtable` creates overshoot/settle effect
2. **Diminishing returns**: Each power "hundred" adds less height
3. **Color thresholds**: Green → Yellow at `Drain > Power`, Red at `Drain > 2×Power`
4. **Hi-res shapes**: `HPOWER.SHP` for 640+ width
5. **Clipping regions**: Two `WINDOW_CUSTOM` clips (unfilled/filled)
6. **Drain marker**: Separate shape (`PowerShape` frame 0) overlaid
7. **Bounce sync**: Power/drain animate independently
8. **Sidebar gate**: All rendering gated on `Map.IsSidebarActive`
8. **Recorded values**: Only updates animation when `PlayerPtr` values change
9. **Power_Fraction**: Used for low-power help text (< 256/65536)
10. **Inheritance**: RadarClass provides radar rendering, PowerClass adds bar below

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/POWER.CPP at f1f0d42
