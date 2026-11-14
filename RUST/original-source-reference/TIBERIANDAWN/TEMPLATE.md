# PATH/FILE
original/TIBERIANDAWN/TEMPLATE.CPP

## Purpose
Scenario template system for Tiberian Dawn. Manages pre-placed terrain/structure templates (base buildings, walls, etc.) from scenario INI files, with map stamping (Mark Up/Down), INI persistence, and editor integration.

## Where it fits in the game
Scenario/editor subsystem - Templates define static map elements placed at scenario start (AI base buildings, neutral structures, walls). Loaded from `[Template]` section of scenario INI. Editor uses `Mark()` to stamp/remove templates on map.

## Classes, structs, enums and important data

### TemplateClass
**Template instance** - Transient object used to stamp template onto map cells.

**Key data members:**
- `Class` (TemplateTypeClass const*) - Template type definition
- `Coord` (COORDINATE) - World position (from cell)
- Pooled allocation via `Templates` (TFixedIHeapClass)

### TemplateTypeClass (referenced via `Class`)
**Template type definition** from TEMPERAT.INI/etc.

**Key fields:**
- `Type` (TemplateType) - Template category
- `Width` / `Height` (int) - Grid dimensions in cells
- `IniName` (char*) - INI identifier
- `Get_Image_Data()` - Icon set map for stamping

### TemplateType enum (partial)
| Type | Purpose |
|------|---------|
| `TEMPLATE_NONE` | Empty |
| `TEMPLATE_CLEAR1` | Clear/erase template |
| Base buildings | Power, Barracks, War Factory, etc. |
| Walls | Various wall segments |
| Neutral structures | Tiberium silos, etc. |

### Global
- `Templates` (TFixedIHeapClass<TemplateClass>) - Pool, max `TEMPLATE_MAX`

## Functions and methods

### TemplateClass(type, pos) - Main constructor
**Creates and stamps template at cell.**

1. `Class = &TemplateTypeClass::As_Reference(type)`
2. If `pos != -1`: `Unlimbo(Cell_Coord(pos))` (places on map)

### TemplateClass() - Default constructor
**Pool placeholder** - `Class = 0`. Used only for `VTable` capture in `Init()`.

### Init() - Static
**Resets template system** (scenario load).

1. `Templates.Free_All()`
2. Creates/destroys temporary instance to capture `VTable` for `Validate()`

### Read_INI(buffer)
**Loads templates from scenario INI** `[Template]` section.

**INI format:**
```ini
[Template]
000=TEMPLATE_POWER
001=TEMPLATE_BARRACKS
...
```
- Keys are cell indices (`000`, `001` = cell numbers)
- Values are template type names
- `WWGetPrivateProfileString(NULL)` gets all keys
- For each: `cell = atoi(key)`, `type = TemplateTypeClass::From_Name(value)`
- Creates `new TemplateClass(type, cell)` (auto-stamps via constructor)

### Write_INI(buffer)
**Saves templates to scenario INI** (editor).

1. Reads existing keys via `WWGetPrivateProfileString(NULL)`
2. Clears all existing entries
3. Scans entire map (`MAP_CELL_TOTAL` cells)
4. For cells with `TType != TEMPLATE_NONE && TIcon == 0` (template anchor):
   - Writes `###=IniName` where `###` = cell index

### Mark(mark) → bool
**Stamps or lifts template on map** - Core editor operation.

**Parameters:** `MARK_UP` (remove) or `MARK_DOWN` (place)

**Process:**
1. Validates via `ObjectClass::Mark(mark)`
2. Gets icon set map from `Class->Get_Image_Data()`
3. Iterates template grid (`Class->Width` × `Class->Height`):
   - `cell = Coord_Cell(Coord) + y*MAP_CELL_W + x`
   - Checks `Map.In_Radar(cell)`
   - Reads icon map: `mapptr[number] != -1` = real icon
   - **MARK_UP**: If `cellptr->TType == Class->Type && TIcon == number` → clears to `TEMPLATE_NONE`
   - **MARK_DOWN**: 
     - If `TEMPLATE_CLEAR1` → clears cell
     - Else → sets `TType = Class->Type`, `TIcon = number`
   - Calls `cellptr->Redraw_Objects()`, `Recalc_Attributes()`
4. **MARK_DOWN special**: Sets `noup=true`, `delete this`, `noup=false` (template consumed)

### Unlimbo(coord)
**Places template at coordinate** - Inherited, sets position for stamping.

### As_Target() → TARGET
**Target conversion** - `Build_Target(KIND_TEMPLATE, Templates.ID(this))`

### Pool management
- `operator new` → `Templates.Allocate()`, sets active
- `operator delete` → clears active, `Templates.Free()`
- `Init()` → `Templates.Free_All()`

## Inputs, outputs and side effects

**State mutations:**
- Map cell `TType`/`TIcon` (template stamp)
- Template pool allocation
- Cell redraw/attribute recalc

**External systems affected:**
- Map rendering (template icons)
- Scenario editor (Mark Up/Down)
- AI base building (pre-placed structures)
- Save/load (via INI)

## Calls into / called by

**Called by:**
- Scenario load → `Read_INI`
- Scenario editor save → `Write_INI`
- Editor tool → `Mark(MARK_DOWN/UP)`
- `TemplateClass::Init()` at scenario start

**Calls into:**
- `TemplateTypeClass::As_Reference/From_Name`
- `WWGetPrivateProfileString/WritePrivateProfileString`
- `Map` cell access (`TType`, `TIcon`, `Redraw_Objects`, `Recalc_Attributes`)
- `Coord_Cell`, `Cell_Coord`, `Map.In_Radar`
- `ObjectClass::Mark`
- `Templates` pool

## Related original files
- **TEMPLATE.H** - Class declarations, TemplateType enum
- **TEMPLATETYPE.H/.CPP** - TemplateTypeClass definitions
- **MAP.H/.CPP** - MapClass, CellClass (TType, TIcon)
- **INI.H/.CPP** - INI functions
- **FUNCTION.H** - Global `Templates` pool
- **HEAP.H** - TFixedIHeapClass

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-map-editor`

**Mapped modules:**
- `cnc-core/src/template.rs` - TemplateClass, TemplateTypeClass
- `cnc-map-editor/src/template_tool.rs` - Mark Up/Down logic
- `cnc-tiberian-dawn/src/scenario/td_template.rs` - TD template types

**Not yet ported:**
- Icon set map stamping (per-cell icon indices)
- `MARK_DOWN` consumes template (delete this)
- `noup` guard prevents double-lift
- `TIcon == 0` anchor detection (only stamp at origin cell)
- `TEMPLATE_CLEAR1` special erase type
- Map cell attribute recalc after stamp
- INI key = cell index (not sequential)
- VTable capture for Validate()

## Edge cases and compatibility notes

1. **Anchor cell only**: `TIcon == 0` identifies template origin; other cells have `TIcon > 0`
2. **Template consumed on place**: `MARK_DOWN` deletes the TemplateClass instance
3. **noup flag**: Prevents `Mark(MARK_UP)` during `MARK_DOWN` deletion
4. **Clear template**: `TEMPLATE_CLEAR1` erases cells instead of placing
5. **Icon map**: `Get_Image_Data()` returns byte map; `-1` = no icon at that grid position
6. **Editor vs game**: Templates only placed at scenario load or editor; not during gameplay
7. **INI keys are cell indices**: Not sequential 000, 001... but actual cell numbers
8. **Write_INI scans whole map**: Finds all `TType != NONE && TIcon == 0` cells
9. **Pool allocation**: Templates transient; created for stamping, deleted after
10. **VTable hack**: Default constructor used only for vtable pointer capture

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TEMPLATE.CPP at f1f0d42