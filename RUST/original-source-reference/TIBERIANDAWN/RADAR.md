# PATH/FILE
original/TIBERIANDAWN/RADAR.CPP

## Purpose
Radar/minimap system for Tiberian Dawn. Renders the lower-right tactical map with terrain, units, structures, player names mode, zoom/pan, animation, and click-to-center interaction. Integrates with sidebar and power bar.

## Where it fits in the game
Sidebar UI - Lower-right corner below radar map. Global `RadarClass Radar` instance (inherited by `PowerClass`). Toggled on/off, shows terrain/objects, supports player names overlay, zoom, and click-to-center tactical map.

## Classes, structs, enums and important data

### RadarClass : DisplayClass
**Radar/minimap controller** - Renders tactical overview.

**Key data members:**
| Variable | Purpose |
|----------|---------|
| `IsRadarActive` | Radar visible |
| `IsRadarActivating/Deactivating` | Animation state |
| `DoesRadarExist` | Fully activated |
| `IsZoomed` | Zoom mode |
| `IsPlayerNames` | Player names overlay |
| `RadarAnim` | Radar background animation (WSA) |
| `RadarAnimFrame` | Current animation frame |
| `RadarCursorRedraw` | Cursor dirty flag |
| `RadX/Y/Width/Height` | Radar plate position |
| `RadOffX/Y/IWidth/IHeight` | Inner map area |
| `RadPWidth/PHeight` | Plate size |
| `RadarCell` | Upper-left cell of radar view |
| `RadarCellWidth/Height` | View size in cells |
| `ZoomFactor` | Zoom level |
| `IsPlayerNames` | Names overlay mode |
| `SpecialRadarFrame` | Cursor animation frames |
| `PixelPtr/PixelStack` | Dirty pixel stack for incremental redraw |

### RadarButton (TacticalClass)
**Clickable radar area** - Handles clicks/drag/zoom.

### Animation frames
| Frame | Purpose |
|-------|---------|
| `RADAR_ACTIVATED_FRAME` | Fully open |
| `MAX_RADAR_FRAMES` | Fully closed |
| `RadarAnimFrame` | Current frame (0..MAX) |

### Constants
| Constant | Value | Purpose |
|----------|-------|---------|
| `RadWidth` | 80×factor | Plate width |
| `RadHeight` | 70×factor | Plate height |
| `RadPWidth/PHeight` | 64×factor | Inner map |
| `RadOffX/Y` | 16/7 (hi-res) | Inner offset |
| `RadIWidth/IHeight` | 128/128 (hi-res) | Inner size |

### Player names mode
| Element | Position/Style |
|---------|----------------|
| Title | `TXT_NAME_COLON` / `TXT_KILLS_COLON` |
| Names | Truncated to 9 chars + `..` |
| Kills | Right-aligned, 2-digit |
| Colors | `MPlayerTColors[RemapColor]` (defeated=GREY) |
| Labels | `TXT_COMPUTER` for AI |

## Functions and methods

### RadarClass()
**Constructor** - Defaults: `IsZoomed=true`, `IsRadarActive=false`, `IsPlayerNames=false`.

### One_Time()
**One-time init** (from `Init_Game`):
1. `DisplayClass::One_Time()`
2. Computes layout with `factor = Get_Resolution_Factor()`:
   - `RadWidth = 80×factor`, `RadHeight = 70×factor`
   - `RadX = SeenBuff.Width - RadWidth`
   - `RadY = Map.Get_Tab_Height() - factor`
   - Inner map: `RadPWidth=64×factor`, `RadPHeight=64×factor`
   - Offsets: `factor=1` → 16/7/128/128; `factor=0` → 4/1/72/69
3. `RadarButton` bounds over inner map area

### Init_Clear(theater)
**Scenario reset**:
1. `DisplayClass::Init_Clear()`
2. Resets all radar state (active=false, redraw=true, names=false)
3. If map exists: `IsZoomed=false`, `Zoom_Mode(Coord_Cell(Map.TacticalCoord))`

### Radar_Activate(control) → bool
**Controls radar state machine**.

| Control | Action |
|---------|--------|
| -1 | Toggle |
| 0 | Off (with deactivate animation) |
| 1 | On (with activate animation) |
| 2 | Remove gadgets (sidebar inactive) |
| 3 | Add gadgets (sidebar active) |
| 4 | Full remove (reset all) |

**Animation states:**
- `IsRadarActivating`: Frames 0→RADAR_ACTIVATED_FRAME
- `IsRadarDeactivating`: Frames RADAR_ACTIVATED_FRAME→MAX_RADAR_FRAMES
- `DoesRadarExist`: Fully open

### Draw_It(forced)
**Renders radar map** (legacy path `#if 0` disabled).

**Active path** (inline in `Draw_It`):
1. If `IsPlayerNames`: `Draw_Names()`
2. If animating: `Radar_Anim()`
3. If active:
   - Loads `RadarAnim` (house-specific: `RADAR.suffix` or `RADAR.JP`)
   - **Incremental redraw**: Processes `PixelStack` dirty pixels
     - `Plot_Radar_Pixel(cell)` for each
     - Refills stack from `IsPlot` flags
   - `Radar_Cursor()` for cursor animation
4. Delegates to `DisplayClass::Draw_It()`

### Radar_Anim()
**Activation/deactivation animation**:
1. Draws `RADAR_ACTIVATED_FRAME` frame
2. Fills inner area with `DKGREY` if base exists

### AI(input, x, y)
**Per-frame logic**:
1. Cursor animation: `SpecialRadarFrame--` → `RadarCursorRedraw`
2. Activation animation: Increments/decrements `RadarAnimFrame`
3. Calls `DisplayClass::AI()`

### TacticalClass::Action(flags, key) → int
**Radar click/drag handler** - Main interaction.

**Flow:**
1. Gates on `Map.IsSidebarActive` and `IsRadarActive`
2. Disables if `IsPlayerNames`
3. Mouse coords: `MouseQX/Y` for clicks, current for moves
4. `Click_In_Radar()` → hit test
5. **Inside radar**:
   - `Click_Cell_Calc()` → cell under cursor
   - Determines `shadow` (shroud), `object` under cursor
   - If selection exists: `action = What_Action(object/cell)`
   - Filters valid actions: MOVE, ATTACK, ENTER, CAPTURE, SABOTAGE
   - `RIGHTPRESS` → `Mouse_Right_Press()` (zoom toggle)
   - `LEFTUP` → `Mouse_Left_Up()`
   - `LEFTPRESS` → `Mouse_Left_Release()`
4. **Outside radar** (but sidebar active):
   - `RIGHTPRESS` → `Zoom_Mode()`
   - `LEFTPRESS` → Centers tactical map on clicked cell
     - Computes centered cell, adjusts for zoom
     - `Set_Tactical_Position()`, `SpecialRadarFrame=4`
5. `RIGHTPRESS` always → `Zoom_Mode()`
6. `result=-1` (outside sidebar) → `MOUSE_NORMAL`

### Click_In_Radar(x, y, check_bounds) → int
**Hit test** - Returns 1 (inside), 0 (outside plate), -1 (outside sidebar).

### Click_Cell_Calc(x, y) → CELL
**Pixel → cell** conversion for radar inner area.

### Refresh_Cells(cell, list)
**Sidebar redraw hook** - If `REFRESH_SIDEBAR` in list, flags radar redraw.

### Set_Radar_Position(cell)
**Centers radar on cell** with blit optimization.

**Process:**
1. Computes `oldx/oldy` from cell, applies zoom
2. `Confine_Rect()` clamps to map bounds
3. If position changed:
   - Computes overlapping region (`radw`, `radh`)
   - Blits overlapping section (with `OverlappedVideoBlits` check)
   - Falls back to temp surface if no overlap blit support
   - Flags dirty pixels for changed edges (X/Y deltas)
4. Updates `RadarCell`, `RadarX/Y`, `RadarCursorRedraw`, `IsToRedraw`

### Set_Map_Dimensions(x, y, w, h)
**Map resize** - Calls `Set_Radar_Position()` + base.

### Set_Tactical_Position(coord)
**Tactical map move** - Calls base + `Set_Radar_Position()`.

### Cell_On_Radar(cell) → bool
**Visibility test** - Checks if cell within `RadarCell ± RadarCellWidth/Height`.

### Player_Names(on)
**Toggles names overlay** - `IsPlayerNames`, flags redraw.

### Draw_Names()
**Renders player list** in radar area:
1. Draws `RADAR_ACTIVATED_FRAME` background
2. Fills inner with black
3. Headers: `TXT_NAME_COLON`, `TXT_KILLS_COLON`
3. For each house `HOUSE_MULTI1..MPlayerMax`:
   - Color: `MPlayerTColors[RemapColor]` (defeated=GREY)
   - Name: `MPlayerNames` or `TXT_COMPUTER` (truncated 9 chars)
   - Kills: Sum of `UnitsKilled+BuildingsKilled` (right-aligned)
   - Style: `TPF_6PT_GRAD|TPF_USE_GRAD_PAL|TPF_NOSHADOW`

### Radar_Cursor(redraw)
**Cursor animation** - Blinking crosshair (elided in source).

### Plot_Radar_Pixel(cell)
**Single pixel update** - Renders terrain/object at cell (elided).

### Radar_Pixel(cell)
**Marks cell dirty** - Adds to `PixelStack` for incremental redraw.

### Coord_To_Radar_Pixel(coord) → (x, y)
**World → radar pixel** conversion (elided).

### Get_Multi_Color() → int
**Multiplayer color offset** (elided).

### Zoom_Mode(cell)
**Zoom toggle** - Toggles `IsZoomed`, recenters (elided).

### Set_Tactical_Position(coord) (overload)
**Tactical position setter** - Calls base + `Set_Radar_Position()`.

### Radar_Cursor(redraw) (elided)
**Cursor blink** - Toggles crosshair visibility.

## Radar states
| State | Flags | Animation |
|-------|-------|-----------|
| Off | `!IsRadarActive` | None |
| Activating | `IsRadarActivating` | 0→RADAR_ACTIVATED_FRAME |
| Active | `IsRadarActive` | Cursor blink |
| Deactivating | `IsRadarDeactivating` | RADAR_ACTIVATED_FRAME→MAX |
| Names mode | `IsPlayerNames` | Static list |

## Inputs, outputs and side effects

**State mutations:**
- Radar view position (`RadarCell`, `RadarX/Y`)
- Animation frames (`RadarAnimFrame`, `SpecialRadarFrame`)
- Dirty pixel stack (`PixelStack`, `PixelPtr`)
- Player names mode (`IsPlayerNames`)
- Zoom state (`IsZoomed`)

**External systems affected:**
- Tactical map position (click centers map)
- Sidebar rendering (power bar, radar plate)
- Tactical map redraw (via `Flag_To_Redraw`)

## Calls into / called by

**Called by:**
- `INIT.CPP` → `One_Time()`
- Main loop → `AI()`, `Draw_It()`
- Sidebar → `Radar_Activate()`, `Init_Clear()`
- Map move → `Set_Tactical_Position()`, `Set_Map_Dimensions()`
- Map redraw → `Refresh_Cells()`
- Mouse input → `TacticalClass::Action()`

**Calls into:**
- `DisplayClass` (base drawing)
- `CC_Draw_Shape` (radar background, animation)
- `Plot_Radar_Pixel` (terrain/objects)
- `Map` (tactical position, cell visibility)
- `PlayerPtr->ActLike` (house-specific radar art)
- `MPlayerNames/Colors` (names mode)
- `Fancy_Text_Print` (names rendering)

## Related original files
- **RADAR.H** - Class declarations, TacticalClass
- **DISPLAY.H/.CPP** - DisplayClass base
- **POWER.H/.CPP** - PowerClass (inherits RadarClass)
- **MAP.H/.CPP** - MapClass (tactical position, sidebar)
- **SIDEBAR.H/.CPP** - Sidebar integration
- **PLAYER.H/.CPP** - PlayerPtr (ActLike for radar art)
- **MULTI.H** - MP names/colors
- **WSA** - Radar animation (RADAR.suffix)
- **RADAR.SHP** - Radar cursor shapes

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-ui`

**Mapped modules:**
- `cnc-ui/src/sidebar/radar.rs` - RadarClass
- `cnc-ui/src/sidebar/minimap.rs` - Rendering, interaction
- `cnc-tiberian-dawn/src/ui/td_radar.rs` - TD-specific (house art, names)

**Not yet ported:**
- Incremental dirty-pixel redraw (`PixelStack`)
- Activation/deactivation animation frames
- House-specific radar art (`RADAR.suffix`)
- Player names overlay with kills
- Zoom/pan with blit optimization
- Click-to-center tactical map
- Right-click zoom toggle
- Rubber-band drag (tactical map)
- Shroud-aware object detection
- Radar cursor animation blink
- Multiplayer ID decoding for names

## Edge cases and compatibility notes

1. **Legacy Draw_It disabled**: `#if 0` wraps old rendering; new path inline
2. **Hi-res factor**: All dimensions ×2 at 640+ width
3. **Radar art**: `RADAR.suffix` per house (temperate/winter/desert)
4. **Japanese**: `RADAR.JP` variant
5. **Activation sound**: `VOC_RADAR_ON/OFF` (commented out)
6. **Incremental redraw**: `PixelStack` processes in chunks (`PIXELSTACK` limit)
7. **Blit optimization**: Overlapping blit for radar scroll; temp surface fallback
8. **Zoom factor**: Affects blit coordinates and clipping
9. **Names mode**: Disables normal radar rendering
9. **MP only**: `HOUSE_MULTI1` base for player houses
10. **Kills calculation**: Sums `UnitsKilled+BuildingsKilled` per house
11. **Name truncation**: 9 chars + `..`
12. **Defeated styling**: GREY color, no grad palette
13. **Computer label**: `TXT_COMPUTER` for AI houses
14. **Right-click**: Always toggles zoom (`Mouse_Right_Press`/`Zoom_Mode`)
15. **Click outside radar**: Centers tactical map (not zoom)
16. **Rubber band**: `LEFTPRESS` initiates drag mode (tactical map)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/RADAR.CPP at f1f0d42
