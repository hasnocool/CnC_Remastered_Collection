# PATH/FILE
original/TIBERIANDAWN/SPECIAL.CPP

## Purpose
Special options dialog and configuration for Tiberian Dawn. Provides player-adjustable game rules and settings via a modal dialog with checkboxes. Controls gameplay variants such as separate helipads, visible targets, Tiberium growth rate, and other rule modifiers. Used by `OPTIONS.CPP` for the game options menu.

## Where it fits in the game
Gameplay configuration - modal dialog accessed from the options menu. Settings persist via `SpecialClass` global and affect game rules for the current session. Integrated with `STARTUP.CPP` initialization and `LOGIC.CPP` rule processing. Global `Special` instance.

## Classes, structs, enums and important data

### SpecialClass
**Game settings and rules configuration.**

**Key data members (bool flags):**
| Flag | Description | Effect |
|------|-------------|--------|
| `IsSeparate` | Separate helipads | Helipads don't share landing space |
| `IsNamed` | Show unit names | Display unit names above objects |
| `IsDefenderAdvantage` | Defender advantage | Game balance modifier |
| `IsVisibleTarget` | Visible target | Target always visible |
| `IsTreeTarget` | Tree target | Trees can be targeted |
| `IsMCVDeploy` | MCV deploy | Mobile Construction Vehicle can deploy |
| `IsSmartDefense` | Smart defense | AI defensive behavior |
| `IsThreePoint` | Three-point attack | Three-way attack pattern |
| `IsTGrowth` | Tiberium growth | Tiberium expansion rate |
| `IsTSpread` | Tiberium spread | Tiberium coverage area |
| `IsTFast` | Tiberium fast growth | Accelerated Tiberium growth |
| `IsRoad` | Road pieces | Road graphics enabled |
| `IsScatter` | Tiberium scatter | Random Tiberium distribution |

### Special_Dialog() → displays options dialog
**Entry point for the special options dialog:**
1. Saves current `Special` state: `SpecialClass oldspecial = Special`
2. Creates checkbox options from `_options[]` array (15 total)
3. Each checkbox maps to a `SpecialClass` flag
4. Default values loaded from saved game/config
5. Dialog loop with OK/Cancel buttons
6. On OK: updates `Special` flags from checkbox states

### Individual flag accessors
Each flag has implicit boolean access through `Special.FlagName`:
- `Special.IsSeparate` → helipad separation
- `Special.IsNamed` → unit naming
- `Special.IsDefenderAdvantage` → balance
- `Special.IsVisibleTarget` → target visibility
- `Special.IsTreeTarget` → tree targeting
- `Special.IsMCVDeploy` → MCV mechanics
- `Special.IsSmartDefense` → AI defense
- `Special.IsThreePoint` → attack pattern
- `Special.IsTGrowth` → Tiberium growth rate
- `Special.IsTSpread` → Tiberium spread
- `Special.IsTFast` → Tiberium fast growth
- `Special.IsRoad` → road graphics
- `Special.IsScatter` → Tiberium distribution

## Functions and methods

### Special_Dialog() → displays modal dialog
**Full dialog procedure:**

1. **Save state**: `SpecialClass oldspecial = Special`
2. **Create option array** `_options[15]` with:
   - Description text ID
   - Setting variable
   - CheckBoxClass button
3. **Option mapping switch** sets initial checkbox state:
   - `TXT_SEPARATE_HELIPAD` → `Special.IsSeparate`
   - `TXT_SHOW_NAMES` → `Special.IsNamed`
   - `TXT_VISIBLE_TARGET` → `Special.IsVisibleTarget`
   - `TXT_TREE_TARGET` → `Special.IsTreeTarget`
   - `TXT_MCV_DEPLOY` → `Special.IsMCVDeploy`
   - `TXT_SMART_DEFENCE` → `Special.IsSmartDefense`
   - `TXT_THREE_POINT` → `Special.IsThreePoint`
   - `TXT_TIBERIUM_GROWTH` → `Special.IsTGrowth`
   - `TXT_TIBERIUM_SPREAD` → `Special.IsTSpread`
   - `TXT_TIBERIUM_FAST` → `Special.IsTFast`
   - `TXT_ROAD_PIECES` → `Special.IsRoad`
   - `TXT_SCATTER` → `Special.IsScatter`
4. **UI setup**: Two buttons (OK=200, Cancel=201)
5. **Mouse shape**: `Map.Override_Mouse_Shape(MOUSE_NORMAL)`
6. **Logic page**: `Set_Logic_Page(SeenBuff)`
7. **Dialog loop**: `while (process)` with:
   - `Call_Back()` for normal game mode
   - `Main_Loop()` for pause mode
   - `process = false` on exit
8. **On dialog close**: Updates `Special` flags from checked boxes

### Special flag read/write
- Read: `Special.{flag_name}` (e.g., `Special.IsTGrowth`)
- Write: Set via dialog checkboxes; persist through session

## Inputs, outputs and side effects

**State mutations:**
- `Special` class all 15 bool flags
- Dialog button states (OK/Cancel)
- `Map.Mouse_Shape` override
- `Logic_Page` current page

**External systems affected:**
- `STARTUP.CPP` → reads `Special` flags at game start
- `LOGIC.CPP` → applies rule modifiers based on `Special` flags
- `OPTIONS.CPP` → hosts the dialog entry point
- Multiplayer: flags synchronized via game setup

## Calls into / called by

**Called by:**
- `OPTIONS.CPP` → `Special_Dialog()` entry point
- `STARTUP.CPP` → `Special` flags initialization
- `LOGIC.CPP` → rule modifiers based on flag values

**Calls into:**
- Windows GUI primitives (checkbox creation, button handling)
- `Map.Override_Mouse_Shape()` cursor control
- `Set_Logic_Page()` page management
- `Call_Back()` / `Main_Loop()` message pump

## Related original files
- **Special.H** - Class declarations (if separate)
- **OPTIONS.CPP** → `Special_Dialog()` invocation
- **STARTUP.CPP** → `Special` flags setup
- **LOGIC.CPP** → rule processing with `Special` flags
- **SEENBUFF** - Logic page constant

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/game/special.rs` - SpecialClass and 15 flags
- `cnc-tiberian-dawn/src/ui/options_dialog.rs` - Special_Dialog() port

**Not yet ported:**
- Windows modal dialog framework
- 15 checkbox option mappings
- Tiberium growth rate calculations
- Smart defense AI behavior changes
- Three-point attack pattern logic
- Road/graphic toggles
- Multiplayer flag synchronization
- Option persistence/savegame integration

## Edge cases and compatibility notes

1. **Windows-only**: Dialog UI is Windows-specific
2. **15 configurable flags**: Wide range of gameplay modifiers
3. **Flag interactions**: Some flags may have implicit dependencies
4. **Saved game state**: `Special` flags persist through save/load cycles
5. **Multiplayer sync**: All players must agree on flag settings
6. **Default values**: Initial values from config or saved game
7. **Dialog modeless/pause**: Behavior differs based on `GameToPlay` mode
8. **Mouse cursor**: Overridden to `MOUSE_NORMAL` during dialog
9. **Logic page**: Temporarily switches to `SeenBuff` during dialog
10. **Commented options**: Some flags (TXT_DEFENDER_ADVANTAGE, TXT_TIBERIUM_GROWTH alternatives) are commented out/removed

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SPECIAL.CPP at f1f0d42
