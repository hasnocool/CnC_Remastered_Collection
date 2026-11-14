# PATH/FILE
original/TIBERIANDAWN/LIST.CPP

## Purpose
List box control for Tiberian Dawn UI. Provides a scrollable text list with selection, keyboard/mouse navigation, dynamic scroll bar (with up/down arrows and slider), tab stops, and peer-linked scroll gadgets.

## Where it fits in the game
UI control - Used for multiplayer lobby (player list), options menus, mission briefings, and any scrollable text selection. Inherits `ControlClass` → `GadgetClass`. Owns `UpGadget`/`DownGadget` (arrow buttons) and `ScrollGadget` (slider) as peers.

## Classes, structs, enums and important data

### ListClass : ControlClass
**Scrollable text list with selection.**

**Key data members:**
- `List` (DynamicVectorClass<char const*>) - Text items
- `UpGadget` / `DownGadget` (ShapeButtonClass) - Scroll arrows
- `ScrollGadget` (SliderClass) - Vertical scrollbar (embedded, `BelongToList=true`)
- `IsScrollActive` (bool) - Scrollbar visible
- `LineCount` (int) - Visible lines (`(h-1)/LineHeight`)
- `LineHeight` (int) - `FontHeight+FontYSpacing-1`
- `SelectedIndex` (int) - Selected item (0-based)
- `CurrentTopIndex` (int) - First visible line
- `TextFlags` (TextPrintType) - Rendering flags
- `Tabs` (int const*) - Tab stop pixel offsets
- `Width` adjusted when scrollbar added (`Width -= ScrollGadget.Width`)

**Scroll gadgets (created in constructor, activated dynamically):**
- `UpGadget` (ShapeButtonClass) - Up arrow, peer to list
- `DownGadget` (ShapeButtonClass) - Down arrow, peer to list
- `ScrollGadget` (SliderClass) - Vertical slider, `BelongToList=true`

## Functions and methods

### ListClass(id, x, y, w, h, flags, up_shape, down_shape)
**Constructor** - Initializes base `ControlClass`, creates scroll gadgets.

1. `ControlClass(id, x, y, w, h, LEFTPRESS|LEFTRELEASE|KEYBOARD, false)`
2. Creates `UpGadget`/`DownGadget` (ShapeButtonClass) with provided shapes at right edge
3. Creates `ScrollGadget` (SliderClass) vertical, `BelongToList=true`
4. Positions gadgets at right edge:
   - `UpGadget` at `(x+w, y)`
   - `DownGadget` at `(x+w, y+h)` (adjusted for height)
   - `ScrollGadget` between arrows, width = max arrow width
4. `TextFlags = flags`, `IsScrollActive=false`, `SelectedIndex=0`, `CurrentTopIndex=0`
5. `LineHeight = FontHeight+FontYSpacing-1`, `LineCount = (h-1)/LineHeight`

### ~ListClass()
**Destructor** - `Remove_Scroll_Bar()`.

### Add_Item(text) → int (index)
**Appends string** to `List`, triggers redraw.

1. `List.Add(text)`, `Flag_To_Redraw()`
2. If `List.Count() > LineCount`: `Add_Scroll_Bar()`
3. If scroll active: `ScrollGadget.Set_Maximum(List.Count())`
3. Returns new index (`Count-1`)

### Add_Item(text_id) → int
**Adds by text ID** - Calls `Add_Item(Text_String(text_id))`.

### Remove_Item(text)
**Removes string** from `List`.

1. `List.Delete(text)`
2. If `List.Count() <= LineCount`: `Remove_Scroll_Bar()`
3. If scroll active: `ScrollGadget.Set_Maximum(List.Count())`
4. Adjusts `SelectedIndex`/`CurrentTopIndex` if removed item affects them
5. If scroll active and top removed: `ScrollGadget.Step(1)`

### Action(flags, key) → int
**Input handler.**

| Input | Action |
|-------|--------|
| `LEFTRELEASE` | Clears `LEFTRELEASE`, calls `ControlClass::Action()`, returns true |
| `KN_UP` | `Step_Selected_Index(-1)`, consumes key |
| `KN_DOWN` | `Step_Selected_Index(1)`, consumes key |
| Mouse click | Computes `index = (mouse_y - Y - 1) / LineHeight`, `SelectedIndex = CurrentTopIndex + index` (clamped) |

Other keys: clears `KEYBOARD`, delegates to `ControlClass::Action()`.

### Draw_Me(forced) → int
**Renders list box.**

1. `GadgetClass::Draw_Me(forced)` gate
2. Hides mouse over bounds
3. Draws `BOXSTYLE_GREEN_BOX` background
4. For each visible line (`0..LineCount-1`):
   - `line = CurrentTopIndex + index`
   - If `line < List.Count()`: `Draw_Entry(line, X+1, Y+LineHeight*index+1, Width-2, line==SelectedIndex)`
5. Shows mouse

### Bump(up) / Step(up)
**Page/line scroll via slider.**

- If `IsScrollActive` and `ScrollGadget.Step(up)`/`Bump(up)` returns true:
  - `CurrentTopIndex = ScrollGadget.Get_Value()`
  - `Flag_To_Redraw()`

### Get_Item(index) → char const*
Returns `List[index]` (clamped), or NULL if empty.

### Current_Item() → char const*
Returns `List[SelectedIndex]`.

### Current_Index() → int
Returns `SelectedIndex`.

### Peer_To_Peer(flags, key, whom)
**Handles scroll gadget peer events.**

| Peer | `LEFTRELEASE` Action |
|------|---------------------|
| `UpGadget` | `Step(true)` (up) |
| `DownGadget` | `Step(false)` (down) |
| `ScrollGadget` | `Set_View_Index(ScrollGadget.Get_Value())` |

### Set_View_Index(index) → bool
**Sets top visible line.**

1. `index = Bound(0, List.Count()-LineCount)`
2. If changed: `CurrentTopIndex = index`, `Flag_To_Redraw()`, updates slider `Set_Value()`

### Add_Scroll_Bar() → bool
**Activates scrollbar** (when `List.Count() > LineCount`).

1. `IsScrollActive=true`, `Width -= ScrollGadget.Width`
2. `Make_Peer(*this)` for all three gadgets
3. `Add(*this)` to gadget list (order: Up, Down, Scroll)
4. `Flag_To_Redraw()` all
5. `ScrollGadget.Set_Maximum(List.Count())`, `Set_Thumb_Size(LineCount)`, `Set_Value(CurrentTopIndex)`

### Remove_Scroll_Bar() → bool
**Deactivates scrollbar** (when `List.Count() <= LineCount`).

1. `IsScrollActive=false`, `Width += ScrollGadget.Width`
2. `Remove()` all three gadgets from list
3. `Flag_To_Redraw()`

### Set_Tabs(tabs)
**Sets tab stops** - Stores pointer to int array of pixel offsets.

### Draw_Entry(index, x, y, width, selected)
**Renders single line.**

| Style | Selected rendering | Unselected |
|-------|-------------------|------------|
| `TPF_6PT_GRAD` | `TPF_BRIGHT_COLOR`, `Fill_Rect(CC_GREEN_SHADOW)` | `TPF_MEDIUM_COLOR` if not `TPF_USE_GRAD_PAL` |
| Other | `BLUE` text | `WHITE` text |

Calls `Conquer_Clip_Text_Print(List[index], x, y, color, TBLACK, flags, width, Tabs)`.

### Add(list) / Add_Head(list) / Add_Tail(list) → LinkClass&
**List insertion** - Adds self + scroll gadgets (if active) to gadget chain in order:
- `Add/Add_Tail`: Listbox → Up → Down → Scroll
- `Add_Head`: Scroll → Down → Up → Listbox

### Remove() → GadgetClass*
**Removes self + scroll gadgets** from chain.

### Set_Selected_Index(index)
**Sets selection with auto-scroll.**

1. Clamps to `List.Count()-1`
2. `SelectedIndex = index`, `Flag_To_Redraw()`
3. If `index < CurrentTopIndex`: `Set_View_Index(index)`
4. If `index >= CurrentTopIndex+LineCount`: `Set_View_Index(index-(LineCount-1))`

### Step_Selected_Index(step) → int (old index)
**Moves selection with auto-scroll.**

1. `Set_Selected_Index(old + step)`
2. Returns old `SelectedIndex`

## Dynamic scrollbar lifecycle

| Condition | Action |
|-----------|--------|
| `Add_Item` → `Count > LineCount` | `Add_Scroll_Bar()` |
| `Remove_Item` → `Count <= LineCount` | `Remove_Scroll_Bar()` |
| `Add_Item/Remove_Item` with scroll active | `ScrollGadget.Set_Maximum(Count)` |

## Inputs, outputs and side effects

**State mutations:**
- `List` (DynamicVector)
- `SelectedIndex`, `CurrentTopIndex`
- `IsScrollActive`, `Width` (adjusted for scrollbar)
- Scroll gadget state (max, thumb, value)

**External systems affected:**
- Gadget list (peer gadgets added/removed)
- Rendering (mouse hidden during draw)
- Slider peer synchronization

## Calls into / called by

**Called by:**
- UI dialogs (lobby, options, briefing)
- Main loop → `Input()` per frame
- Slider peer → `Peer_To_Peer()`

**Calls into:**
- `DynamicVectorClass` (List)
- `ShapeButtonClass` (arrows)
- `SliderClass` (scrollbar)
- `Conquer_Clip_Text_Print` (rendering)
- `Draw_Box` (background)
- `Conditional_Hide/Show_Mouse`

## Related original files
- **LIST.H** - Class declaration
- **CONTROL.H/.CPP** - ControlClass base
- **BUTTON.H/.CPP** - ShapeButtonClass (arrows)
- **SLIDER.H/.CPP** - SliderClass (scrollbar)
- **GADGET.H/.CPP** - GadgetClass base
- **TEXTBLIT.H** - Conquer_Clip_Text_Print
- **BOX.H/.CPP** - Draw_Box

## Rust port mapping
**Crates:** `cnc-ui`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-ui/src/gadget/list.rs` - ListClass
- `cnc-ui/src/gadget/slider.rs` - SliderClass (scrollbar)
- `cnc-ui/src/gadget/button.rs` - ShapeButtonClass (arrows)

**Not yet ported:**
- Dynamic scrollbar lifecycle (add/remove based on item count)
- Peer gadget pattern (arrows + slider all peers)
- Auto-width adjustment (`Width -= ScrollGadget.Width`)
- Tab stop rendering (`Tabs` pointer)
- Gradient vs flat text rendering (`TPF_6PT_GRAD`)
- Selection highlight (`CC_GREEN_SHADOW` fill)
- Tab stop support (`Tabs` pointer)
- Mouse click line calculation
- Keyboard navigation (UP/DOWN)
- Page bump via slider (`Bump`)
- Gadget chain insertion order (listbox → up → down → scroll)

## Edge cases and compatibility notes

1. **Width mutation**: `Width` shrinks/grows with scrollbar; affects hit-test
2. **Scrollbar ownership**: `UpGadget`/`DownGadget`/`ScrollGadget` owned by ListClass (deleted in dtor)
3. **Peer pattern**: All three scroll gadgets `Make_Peer(*this)` to list
4. **Slider embedded**: `BelongToList=true` → no separate ± buttons
5. **Thumb size**: `LineCount` (visible lines) = proportional thumb
5. **Selection bounds**: Clamped to `0..Count-1`
6. **View bounds**: `CurrentTopIndex` clamped to `0..Count-LineCount`
7. **SelectedIndex auto-scroll**: `Set_Selected_Index` calls `Set_View_Index` to keep visible
8. **Mouse click math**: `(mouse_y - Y - 1) / LineHeight` + `CurrentTopIndex`
9. **Gadget chain order**: `Add_Tail` puts scroll gadgets after listbox
10. **Dynamic activation**: Scrollbar appears/disappears based on `Count > LineCount`
11. **SelectedIndex adjustment**: On remove, decrements if past end; on top remove, steps view up
12. **Tab stops**: Pointer stored, not copied (caller must keep alive)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/LIST.CPP at f1f0d42
