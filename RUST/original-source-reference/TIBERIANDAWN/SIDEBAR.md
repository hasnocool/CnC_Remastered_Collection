# PATH/FILE
original/TIBERIANDAWN/SIDEBAR.CPP

## Purpose
Sidebar UI system for Tiberian Dawn. Manages buildable unit/structure cameos, production queue visualization (clock animation), special weapons (ion cannon, nuke, airstrike), repair/sell/upgrade modes, and radar/minimap toggle.

## Where it fits in the game
Main gameplay UI - Right-side panel showing production options. Two columns (Structures/Units vs Infantry/Aircraft). Integrates with FactoryClass for production status, HouseClass for tech tree, and MapClass for placement mode.

## Classes, structs, enums and important data

### SidebarClass
**Main sidebar controller** - Two `StripClass` columns.

**Key data members:**
- `Column[2]` (StripClass) - Column 0: Structures/Units, Column 1: Infantry/Aircraft
- `IsSidebarActive` / `IsRepairActive` / `IsUpgradeActive` / `IsDemolishActive` (bool) - Mode flags
- `IsToRedraw` (bool) - Dirty flag
- `SideX/Y/Width/Height` - Screen coordinates
- `PowWidth` - Power bar width
- `MaxVisible` (4) - Visible cameos per column
- `Background` (SBGadgetClass) - Sidebar background gadget
- `Repair`/`Upgrade`/`Zoom` (ShapeButtonClass) - Mode buttons

### StripClass
**Single sidebar column** - Vertical list of buildable cameos with scroll.

**Key data members:**
- `Buildables[MAX_STRIP]` (BuildableEntry[]) - Buildable objects
- `BuildableCount` (int) - Active entries
- `TopIndex` (int) - Scroll offset
- `X, Y` - Column position
- `IsBuilding` (bool) - Any production active
- `IsToRedraw` (bool) - Dirty flag
- `UpButton[2]` / `DownButton[2]` (ShapeButtonClass) - Scroll buttons
- `SelectButton[2][MAX_VISIBLE]` (SelectClass) - Cameo buttons

### BuildableEntry
```cpp
struct BuildableEntry {
    RTTIType BuildableType;   // RTTI_INFANTRYTYPE, UNITTYPE, etc.
    int BuildableID;          // StructType/UnitType/InfantryType/AircraftType
    int Factory;              // FactoryClass index (-1 = none)
};
```

### SelectClass : ControlClass
**Cameo button** - Handles click to start/stop production.

**Fields:** `Strip*`, `Index` (0..MAX_VISIBLE-1)

### ButtonNumberType enum
- `BUTTON_RADAR` (100) - Minimap toggle
- `BUTTON_REPAIR` - Repair mode
- `BUTTON_DEMOLISH` - Sell mode
- `BUTTON_UPGRADE` - Upgrade mode (sell building)
- `BUTTON_SELECT` - Selection
- `BUTTON_ZOOM` - Radar zoom

### Static shape pointers
- `SidebarShape1`/`SidebarShape2` - `SIDE1.SHP`/`SIDE2.SHP` (hi-res aware)
- `StripClass::LogoShapes` - Blank/logo shapes
- `StripClass::ClockShapes` - Construction clock frames
- `StripClass::SpecialShapes[3]` - Ion cannon, nuke, airstrike cameos
- `StripClass::ClockTranslucentTable[512]` - Clock animation translucency

### Constants
- `STRIP_WIDTH` / `OBJECT_WIDTH` / `OBJECT_HEIGHT` - Layout dimensions
- `COLUMNS` = 2
- `MAX_VISIBLE` = 4

## Functions and methods

### SidebarClass::One_Time()
**One-time init** (from `Init_Game`):
1. `PowerClass::One_Time()`
2. Calculates layout with `factor` (hi-res = 2)
3. Sets `WindowList[WINDOW_SIDEBAR]` clipping rect
4. Positions `Column[0]` and `Column[1]` with spacing
5. Loads `SIDE1.SHP`/`SIDE2.SHP` via `Hires_Retrieve`

### SidebarClass::Init_Clear()
**Scenario reset**: Clears columns, deactivates modes, `Activate(false)`

### SidebarClass::Init_IO()
**Button setup** (non-editor only):
1. Measures button text widths (`TXT_REPAIR_BUTTON`, `TXT_BUTTON_SELL`, `TXT_MAP`)
2. Creates `Repair`/`Upgrade`/`Zoom` `ShapeButtonClass`:
   - Localized shapes: `REPAIR.F/SHP`, `REPAIRG.SHP`, `REPAIRF.SHP`
   - `IsSticky=true`, `IsToggleType=true`, `ReflectButtonState=true`
3. `Zoom` enabled if `IsRadarActive || GameToPlay!=GAME_NORMAL`
4. Calls `Column[0/1].Init_IO()`
5. If `IsSidebarActive` from save: `Activate(true)`

### SidebarClass::Init_Theater(theater)
**Theater switch**: Delegates to `PowerClass` and both columns

### SidebarClass::Which_Column(type) → int
**Routes type to column**:
- `RTTI_BUILDINGTYPE` / `RTTI_UNITTYPE` → 0 (Structures/Units)
- `RTTI_INFANTRYTYPE` / `RTTI_AIRCRAFTTYPE` → 1 (Infantry/Aircraft)
- Else → 0

### SidebarClass::Activate(activate)
**Show/hide sidebar**: Updates `IsSidebarActive`, adds/removes `Background` gadget, activates/deactivates columns

### SidebarClass::Activate_Repair/Upgrade/Demolish(activate)
**Mode toggles**: Updates flag, button pressed state, calls `Map.Activate_Repair/Sell/Upgrade`

### SidebarClass::Set_Current(obj)
**Selection-driven update**: Finds object's column, calls `StripClass::Set_Current`

### SidebarClass::Add(obj) → bool
**Adds buildable to sidebar**: Routes to correct column via `Which_Column`

### SidebarClass::Abandon_Production(type, factory) → bool
**Delegates to column**

### SidebarClass::Recalc() → bool
**Delegates to both columns** (OR result)

### SidebarClass::AI(input)
**Input handling**: Delegates to active column's `AI`

### SidebarClass::Draw_It()
**Renders sidebar**:
1. Draws `SidebarShape1`/`SidebarShape2` background
2. Draws power bar (`PowerClass::Draw_It`)
3. Draws `Repair`/`Upgrade`/`Zoom` buttons
4. Calls `Column[0/1].Draw_It()`

### SidebarClass::Refresh_Cells(cell, list) → bool
**Clipping intercept**: Checks `WINDOW_SIDEBAR` overlap

### StripClass::One_Time(col)
**Column init**: Creates `SelectButton` grid, `UpButton`/`DownButton`

### StripClass::Init_Clear()
**Reset**: `BuildableCount=0`, `TopIndex=0`, `IsBuilding=false`, deactivates buttons

### StripClass::Init_IO(col)
**Button wiring**: Positions `UpButton`/`DownButton`/`SelectButton`, adds to input system

### StripClass::Init_Theater(theater)
**Loads theater shapes**: `LogoShapes`, `ClockShapes`, `SpecialShapes[3]`
Builds `ClockTranslucentTable` (fading + translucent combo)

### StripClass::Add(obj) → bool
**Adds buildable to list**:
1. Finds `BuildableType`/`BuildableID` via `Fetch_Techno_Type`
2. Special items: `SPC_ION_CANNON`, `SPC_NUCLEAR_BOMB`, `SPC_AIR_STRIKE`
3. Appends to `Buildables[BuildableCount++]`
4. `IsToRedraw = true`

### StripClass::Set_Current(obj)
**Scrolls to show selected object**: Adjusts `TopIndex` if needed

### StripClass::Scroll(direction)
**Manual scroll**: `TopIndex += direction`, clamped, `Flag_To_Redraw()`

### StripClass::Factory_Link(factory, type, id) → bool
**Links factory to cameo**: Finds matching buildable, sets `Factory=factory`, `IsBuilding=true`, `Flag_To_Redraw()`

### StripClass::Abandon_Production(factory) → bool
**Clears factory link**: Calls `factory->Abandon()`, clears `Buildables[index].Factory`, updates `IsBuilding`, `Flag_To_Redraw()`

### StripClass::Recalc() → bool
**Validates buildables**: Removes entries if `tech->Who_Can_Build_Me()` fails or special not present (ion cannon/nuke/airstrike ready). Shifts array down, `TopIndex=0`.

### StripClass::Draw_It()
**Renders column**:
1. Loops `MAX_VISIBLE` slots from `TopIndex`
2. For each: determines shape, remapper, production state
3. **Buildable**: Gets cameo via `Get_Cameo_Data()`, checks factory link
4. **Special**: `Get_Special_Cameo()` for ion/nuke/airstrike
5. **Drawing**:
   - `CC_Draw_Shape` with remapper (house colors) or `FadingLight` (flasher)
   - If `darken` (factory busy): `ClockTranslucentTable` overlay
   - If `production`: Clock shape (`stage+1`) or `PIP_READY`/`PIP_HOLDING`
6. Position adjusted for `WINDOW_SIDEBAR` clipping

### StripClass::Get_Special_Cameo(spc) → void const*
**Returns special shape**: `SpecialShapes[spc-1]`

### SelectClass::Set_Owner(strip, index)
**Positions button**: `X=strip.X`, `Y=strip.Y + index * OBJECT_HEIGHT`

### SelectClass::Action(flags, key) → int
**Click handling**:
- **LeftUp**: Shows help text (`Full_Name()`, cost with `CostBias`)
- **RightPress**: Cancel/suspend production → `EventClass::ABANDON`/`SUSPEND`
- **LeftPress**: 
  - Special: Enters targetting mode (`Map.IsTargettingMode = spc`), `Speak(VOX_SELECT_TARGET)`
  - Normal: 
    - If factory busy elsewhere → `VOX_NO_FACTORY`
    - If factory completed → `EventClass::PLACE` (unit) or `PlayerPtr->Manual_Place` (building)
    - If factory suspended → `EventClass::PRODUCE` (resume)
    - No factory → `EventClass::PRODUCE` (new)

### SBGadgetClass::Action → int
**Background hover**: Forces `MOUSE_NORMAL`, clears help text

### sortfunc
**qsort comparator** for buildable ordering (not shown in excerpt)

## Inputs, outputs and side effects

**State mutations:**
- Buildable list (`Buildables[]`)
- Factory links (`Factory` index)
- Production modes (repair/sell/upgrade)
- Scroll position (`TopIndex`)
- Redraw flags

**External systems affected:**
- Factory production (via `FactoryClass`)
- Map placement mode (`Map.PendingObjectPtr`, `IsTargettingMode`)
- Network events (`OutList.Add(EventClass::...)`)
- Voice (`Speak(VOX_*)`)
- Power display (`PowerClass`)

## Calls into / called by

**Called by:**
- `Init_Game` → `One_Time`
- Scenario load → `Init_Clear`, `Init_Theater`
- `Init_Game` → `Init_IO`
- Main loop → `AI`, `Draw_It`
- Factory completion → `Factory_Link`
- Unit selection → `Set_Current`
- Tech tree unlock → `Add`
- Building destruction → `Recalc`
- Player input → `SelectClass::Action`, mode buttons

**Calls into:**
- `PowerClass` (power bar)
- `FactoryClass` (`Has_Completed`, `Completion`, `Get_Object`, `Abandon`)
- `HouseClass` (`Remap_Table`, `CostBias`, factory indices)
- `TechnoTypeClass` (`Get_Cameo_Data`, `Cost_Of`, `Full_Name`)
- `MapClass` (`Override_Mouse_Shape`, `Help_Text`, `PendingObjectPtr`, `IsTargettingMode`)
- `CC_Draw_Shape` (rendering)
- `EventClass` / `OutList` (network events)
- `Speak` (voice)
- `Fetch_Techno_Type`, `Get_Special_Cameo`

## Related original files
- **SIDEBAR.H** - Class declarations, nested classes
- **POWER.H/.CPP** - PowerClass (sidebar power bar)
- **FACTORY.H/.CPP** - FactoryClass
- **HOUSE.H/.CPP** - HouseClass (tech tree, remap)
- **TECHNOTYPE.H/.CPP** - Cameo data, cost
- **MAP.H/.CPP** - MapClass (placement, targetting)
- **EVENT.H/.CPP** - EventClass (PRODUCE, PLACE, ABANDON, SUSPEND)
- **VOX.H** - Voice constants
- **SHAPE.H/.CPP** - CC_Draw_Shape
- **BUTTON.H/.CPP** - ShapeButtonClass, ControlClass

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-ui`, `cnc-core`

**Mapped modules:**
- `cnc-ui/src/sidebar.rs` - SidebarClass, StripClass
- `cnc-ui/src/sidebar/cameo.rs` - SelectClass, buildable rendering
- `cnc-tiberian-dawn/src/ui/td_sidebar.rs` - TD-specific layout
- `cnc-core/src/production/ui.rs` - Factory link, production state

**Not yet ported:**
- Two-column layout with dynamic spacing
- Hi-res scaling (factor = width/320)
- Clipping window (`WINDOW_SIDEBAR`)
- Construction clock animation (`ClockShapes`, `ClockTranslucentTable`)
- House remapper for cameo colors
- Flasher highlight (`FadingLight`)
- Special weapon cameos (ion/nuke/airstrike)
- Scroll buttons (Up/Down)
- Mode buttons (Repair/Sell/Upgrade/Zoom) with localized shapes
- Right-click suspend/abandon logic
- Network event generation (`EventClass::PRODUCE/PLACE/ABANDON/SUSPEND`)
- Voice feedback (`VOX_SELECT_TARGET`, `VOX_BUILDING`, `VOX_NO_FACTORY`, etc.)

## Edge cases and compatibility notes

1. **Column routing**: Buildings/Units → col 0, Infantry/Aircraft → col 1
2. **Hi-res factor**: All dimensions ×2 at 640+ width
3. **Clipping**: `WINDOW_SIDEBAR` prevents cameo overflow
4. **Clock animation**: `stage+1` frames, `PIP_READY` when complete, `PIP_HOLDING` when suspended
5. **Darken overlay**: `ClockTranslucentTable` when factory busy elsewhere
6. **Special items**: Only appear if superweapon structure present (`Is_Present()`)
7. **Recalc cleanup**: Shifts array down, doesn't collapse sidebar (retains position)
8. **Factory link**: One factory per cameo; `Factory_Link` called on production start
9. **Placement mode**: Buildings → `Manual_Place`, Units → `EventClass::PLACE`
10. **Network events**: Generated for MP sync (`OutList.Add`)
11. **Localized button shapes**: `REPAIR.F/G.SHP`, `SELL.F/G.SHP`, `MAP.F/G.SHP`
12. **Zoom button**: Enabled only if radar active or MP game
13. **Save/load**: `IsSidebarActive` persisted, restored via `Activate(true)`
14. **Editor mode**: `Debug_Map` skips button creation

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SIDEBAR.CPP at f1f0d42
