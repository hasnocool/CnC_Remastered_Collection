# PATH/FILE
original/TIBERIANDAWN/COLRLIST.CPP

## Purpose
Color list/management system extending ListClass to associate a color with each list item. Provides per-item color rendering and multiple selection highlight styles (highlight, box, bar, invert) for UI lists requiring colored entries such as player lists, chat, or multiplayer lobby.

## Where it fits in the game
UI subsystem - Specialized list box control used wherever list items need individual colors (e.g., player name lists in multiplayer lobby, chat messages with colored usernames, sidebar unit lists). Inherits from ListClass; adds color array parallel to text items.

## Classes, structs, enums and important data

### ColorListClass
**Color-aware list box** - Extends `ListClass` with per-item color storage and configurable selection rendering.

**Key data members:**
- `Colors` (DynamicArrayClass<char>) - Parallel array storing color index per list item (0-255 palette index)
- `Style` (SelectStyleType) - Selection rendering mode (default `SELECT_HIGHLIGHT`)
- `SelectColor` (int) - Override color for selected items; -1 = use item's own color

**Inherited from ListClass:**
- `List` (DynamicArrayClass<char*>) - Text strings
- `TextFlags` (TextPrintType) - Text rendering flags (e.g., `TPF_6PT_GRAD`)
- `Tabs` - Tab stop positions
- `LineHeight` - Row height in pixels
- `LogicPage` - Drawing surface

### SelectStyleType enum
| Value | Behavior |
|-------|----------|
| `SELECT_NONE` | No special selection rendering; draws in item's color |
| `SELECT_HIGHLIGHT` | Draws selected item in `SelectColor` (or item's color if -1); applies `TPF_BRIGHT_COLOR` if `TPF_6PT_GRAD` set |
| `SELECT_BOX` | Draws rectangle around selected item in `SelectColor`; text in item's color |
| `SELECT_BAR` | Draws filled bar under selected item in `SelectColor`; text in item's color (bright if gradient) |
| `SELECT_INVERT` | Inverts colors: fills rect with item's color, draws text in BLACK/LTGREY |

## Functions and methods

### ColorListClass(id, x, y, w, h, flags, up, down) - Constructor
**Initializes color list with default selection style.**

Parameters:
- `id` - Button/control ID
- `x, y` - Upper-left corner in pixels
- `w, h` - Width, height in pixels
- `flags` - TextPrintType flags (passed to ListClass)
- `up, down` - Shape pointers for scroll buttons (passed to ListClass)

1. Calls `ListClass` constructor
2. Sets `Style = SELECT_HIGHLIGHT`
3. Sets `SelectColor = -1` (use item's color)

### ~ColorListClass() - Destructor
**Clears color array.**

1. `Colors.Clear()`

### Add_Item(text, color) → int (overload 1: char const*)
**Adds string item with associated color.**

Parameters:
- `text` - Null-terminated string to add
- `color` - Palette index (0-255) for this item

1. `Colors.Add(color)` - Appends to color array
2. Returns `ListClass::Add_Item(text)` - Index of new item

### Add_Item(text, color) → int (overload 2: int)
**Adds string ID item with associated color.**

Parameters:
- `text` - String table ID (from STRINGTABLE.H/.TXT)
- `color` - Palette index for this item

1. `Colors.Add(color)`
2. Returns `ListClass::Add_Item(text)`

### Remove_Item(text) - void
**Removes item by string match; maintains color array sync.**

Parameters:
- `text` - String to find and remove

1. `index = List.ID(text)` - Find index via base class
2. If `index != -1`:
   - `Colors.Delete(index)` - Remove color at same index
   - `ListClass::Remove_Item(text)` - Remove text

### Set_Selected_Style(style, color) - void
**Configures selection rendering mode and optional override color.**

Parameters:
- `style` - SelectStyleType value
- `color` - Override color for selected items; -1 = use each item's own color

1. `Style = style`
2. `SelectColor = color`

### Draw_Entry(index, x, y, width, selected) - void
**Renders single list entry with per-item color and selection style.**

Parameters:
- `index` - Index into List/Colors arrays
- `x, y` - Draw coordinates
- `width` - Maximum text width (clipping)
- `selected` - Boolean: true if this row is selected

**Non-selected (selected == false):**
1. `Conquer_Clip_Text_Print(List[index], x, y, Colors[index], TBLACK, TextFlags, width, Tabs)`
2. Returns

**Selected (selected == true):**
1. Determines `color`: `SelectColor == -1 ? Colors[index] : SelectColor`
2. Switch on `Style`:
   - **SELECT_NONE**: Same as non-selected (item's color)
   - **SELECT_HIGHLIGHT**: 
     - If `TextFlags & TPF_6PT_GRAD`: adds `TPF_BRIGHT_COLOR` flag
     - Prints with determined `color`
   - **SELECT_BOX**:
     - `LogicPage->Draw_Rect(x, y, x + width - 2, y + LineHeight - 2, color)`
     - Prints text in item's own color (`Colors[index]`)
   - **SELECT_BAR**:
     - If gradient: `Fill_Rect(x, y, x + width - 1, y + LineHeight - 1, SelectColor)` + bright text
     - Else: `Fill_Rect(x, y, x + width - 2, y + LineHeight - 2, SelectColor)` + normal text
     - Text always in item's color (`Colors[index]`)
   - **SELECT_INVERT**:
     - If gradient: fills rect with `Colors[index]`, prints in BLACK
     - Else: fills rect with `Colors[index]`, prints in LTGREY

## Inputs, outputs and side effects

**State mutations:**
- `Colors` array (parallel to `List`): Add/Delete on item add/remove
- `Style`, `SelectColor`: Configuration via `Set_Selected_Style`

**External systems affected:**
- UI rendering via `LogicPage` (Draw_Rect, Fill_Rect)
- Text rendering via `Conquer_Clip_Text_Print`
- ListClass base functionality (scroll, selection, mouse handling)

## Calls into / called by

**Called by:**
- UI code creating colored lists (multiplayer lobby, chat, sidebar)
- ListClass mouse/keyboard handling (selection changes trigger redraw)

**Calls into:**
- `ListClass` constructor/destructor/Add_Item/Remove_Item/ID
- `DynamicArrayClass<char>::Add/Delete/Clear`
- `Conquer_Clip_Text_Print` (text rendering with clipping)
- `LogicPage->Draw_Rect` / `Fill_Rect` (selection graphics)
- Palette constants: `TBLACK`, `LTGREY`, `BLACK`

## Related original files
- **COLRLIST.H** - Class declaration, SelectStyleType enum
- **LIST.H/.CPP** - ListClass base (text array, scroll, selection, Draw_Entry virtual)
- **FUNCTION.H** - Global types, `Conquer_Clip_Text_Print`, palette constants
- **DYNARRAY.H** - DynamicArrayClass template
- **TEXTPRNT.H** - TextPrintType flags (TPF_6PT_GRAD, TPF_BRIGHT_COLOR)
- **SHAPE.H** - ShapeClass for scroll buttons

## Rust port mapping

**Crates:** `cnc-core`, `cnc-tiberian-dawn`, `cnc-ui`

**Mapped modules:**
- `cnc-ui/src/color_list.rs` - ColorListClass, SelectStyleType
- `cnc-core/src/ui/list.rs` - ListClass base
- `cnc-tiberian-dawn/src/ui/lobby.rs` - Multiplayer lobby usage

**Not yet ported:**
- Per-item color array parallel to text
- Selection style rendering variants (box, bar, invert)
- `TPF_6PT_GRAD` / `TPF_BRIGHT_COLOR` gradient text handling
- `LogicPage` drawing surface abstraction
- `Conquer_Clip_Text_Print` with clipping and tabs

## Edge cases and compatibility notes

1. **Parallel arrays invariant**: `Colors` and `List` must always have same length. `Add_Item` appends to both; `Remove_Item` deletes at same index. No bounds checking in `Draw_Entry` - caller must ensure valid index.

2. **Color type**: Uses `char` (signed 8-bit) for palette index. Values 0-255 map to game palette; -1 used as sentinel in `SelectColor`.

3. **Selection color override**: `SelectColor = -1` means "use item's own color". Any other value (0-255) forces that color for selected items regardless of item's color.

4. **Gradient text interaction**: `TPF_6PT_GRAD` flag (6-point gradient font) modifies SELECT_HIGHLIGHT (adds bright), SELECT_BAR (full-width fill + bright), SELECT_INVERT (full-width fill + BLACK text vs LTGREY).

5. **Box vs Bar coordinates**: 
   - BOX: `x + width - 2`, `y + LineHeight - 2` (1px inset border)
   - BAR (non-gradient): `x + width - 2`, `y + LineHeight - 2`
   - BAR (gradient): `x + width - 1`, `y + LineHeight - 1` (full cell)

6. **Invert colors**: 
   - Gradient: fill with item's color, text = BLACK
   - Non-gradient: fill with item's color, text = LTGREY

7. **No owner-draw callback**: Rendering fully internal; customization only via `Style`/`SelectColor`/item colors.

8. **String ID overload**: `Add_Item(int text, char color)` accepts string table ID; base class handles lookup.

9. **Destructor only clears Colors**: Base `ListClass` destructor handles `List` array cleanup.

10. **VTable**: Virtual `Draw_Entry` allows polymorphic list rendering; `ListClass` declares it virtual.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/COLRLIST.CPP at f1f0d42