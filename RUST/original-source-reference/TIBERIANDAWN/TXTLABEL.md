# PATH/FILE
original/TIBERIANDAWN/TXTLABEL.CPP

## Purpose
Text label rendering system for Tiberian Dawn UI. Provides `TextLabelClass` - a lightweight gadget for displaying static or formatted text with configurable font, color, alignment, and shadow styles. Used throughout dialogs, message lists, buttons, and in-game UI for rendering labels, status text, and dynamic messages.

## Where it fits in the game
UI/gadget subsystem - `TextLabelClass` inherits from `GadgetClass` and serves as the fundamental text rendering primitive. Used by:
- Message lists (`MSGLIST.CPP`) for chat/in-game messages
- Text buttons (`TEXTBTN.CPP`) for button labels
- List boxes (`LIST.CPP`, `CHEKLIST.CPP`, `COLRLIST.CPP`) for item text
- Edit controls (`EDIT.CPP`) for text display
- Game dialogs (`GAMEDLG.CPP`) for speed/scroll rate labels
- Radar (`RADAR.CPP`) for radar text overlays
- Sound dialogs (`SOUNDDLG.CPP`) for list items

## Classes, structs, enums and important data

### TextLabelClass
**Text label gadget** - Lightweight UI element for rendering text strings.

**Inheritance:** `GadgetClass` → `TextLabelClass`

**Key data members:**
| Member | Type | Purpose |
|--------|------|---------|
| `Text` | `char*` | Pointer to text buffer to display |
| `Color` | `int` | Foreground color index |
| `Style` | `TextPrintType` | Font/shadow/alignment flags (bitfield) |
| `UserData` | `unsigned long` | General-purpose user data field |
| `PixWidth` | `int` | Clipping width in pixels (`-1` = no clip) |
| `Segments` | `char` | Reserved/segment count (unused in base) |
| `CRC` | `unsigned short` | Checksum for text validation (message lists) |

**Constructor:** `TextLabelClass(char* txt, int x, int y, int color, TextPrintType style)`
- Initializes `GadgetClass(x, y, 1, 1, 0, 0)` - minimal 1×1 pixel gadget
- Stores text pointer, color, style
- `PixWidth = -1` (unbounded), `Segments = 0`, `UserData = 0`

**Methods:**
- `Draw_Me(int forced)` - Renders text via `Fancy_Text_Print` or `Conquer_Clip_Text_Print`
- `Set_Text(char* txt)` - Updates displayed text pointer

### TextPrintType (enum, DEFINES.H:1894-1915)
**Text rendering control flags** - Bitfield controlling font, shadow, alignment, and color treatment.

**Font selection (bits 0-3):**
| Flag | Value | Purpose |
|------|-------|---------|
| `TPF_LASTPOINT` | 0x0000 | Use previous font point |
| `TPF_6POINT` | 0x0001 | 6pt font |
| `TPF_8POINT` | 0x0002 | 8pt font (default) |
| `TPF_3POINT` | 0x0003 | 3pt font |
| `TPF_LED` | 0x0004 | LED font |
| `TPF_VCR` | 0x0005 | VCR font |
| `TPF_6PT_GRAD` | 0x0006 | 6pt gradient font |
| `TPF_MAP` | 0x0007 | Map font (6pt gradient) |
| `TPF_GREEN12` | 0x0008 | Green tab font |
| `TPF_GREEN12_GRAD` | 0x0009 | Graduated green tab font |

**Shadow style (bits 4-7):**
| Flag | Value | Purpose |
|------|-------|---------|
| `TPF_LASTSHADOW` | 0x0000 | Previous shadow |
| `TPF_NOSHADOW` | 0x0010 | No shadow |
| `TPF_DROPSHADOW` | 0x0020 | Simple drop shadow (default) |
| `TPF_FULLSHADOW` | 0x0040 | Full outline shadow |
| `TPF_LIGHTSHADOW` | 0x0080 | Engraved drop shadow |

**Alignment (bits 8-9):**
| Flag | Value | Purpose |
|------|-------|---------|
| `TPF_CENTER` | 0x0100 | Center about X axis |
| `TPF_RIGHT` | 0x0200 | Right justify |

**Color treatment (bits 12-14):**
| Flag | Value | Purpose |
|------|-------|---------|
| `TPF_MEDIUM_COLOR` | 0x1000 | Medium gradient color |
| `TPF_BRIGHT_COLOR` | 0x2000 | Bright gradient color |
| `TPF_USE_GRAD_PAL` | 0x4000 | Gradient palette from fore color |

**Operators:** `|`, `&`, `~` overloaded for bitfield composition.

## Functions and methods

### TextLabelClass::TextLabelClass(txt, x, y, color, style)
**Main constructor** - Creates text label at position with styling.

```cpp
TextLabelClass::TextLabelClass(char *txt, int x, int y, int color,
    TextPrintType style) : GadgetClass(x,y,1,1,0,0)
{
    Text = txt;
    Color = color;
    Style = style;
    UserData = 0;
    PixWidth = -1;
    Segments = 0;
}
```
1. Calls `GadgetClass` base constructor with 1×1 pixel bounds
2. Stores text pointer (not copied - caller owns buffer)
3. Stores color and style flags
4. Initializes `PixWidth = -1` (no clipping), `UserData = 0`

### TextLabelClass::Draw_Me(forced) → int
**Graphical update routine** - Renders text if gadget needs redraw.

```cpp
int TextLabelClass::Draw_Me(int forced)
{
    if (GadgetClass::Draw_Me(forced)) {
        if (PixWidth == -1) {
            Fancy_Text_Print("%s", X, Y, Color, TBLACK, Style, Text);
        } else {
            Conquer_Clip_Text_Print(Text, X, Y, Color, TBLACK, Style, PixWidth);
        }
        return true;
    }
    return false;
}
```
1. Calls base `GadgetClass::Draw_Me(forced)` - returns true if redraw needed
2. **Unbounded path** (`PixWidth == -1`): Uses `Fancy_Text_Print` with format string
   - Supports `printf`-style formatting via varargs
   - Renders with drop shadow per `Style` flags
3. **Clipped path** (`PixWidth >= 0`): Uses `Conquer_Clip_Text_Print`
   - Clips text at `PixWidth` pixels
   - Supports tab stops (unused here - `tabs = nullptr`)
4. Returns `true` if drawn, `false` if skipped

### TextLabelClass::Set_Text(txt)
**Updates displayed text** - Simple pointer replacement.

```cpp
virtual void Set_Text(char *txt) {Text = txt;};
```
- No redraw triggered automatically; caller must invalidate
- Text buffer owned by caller

## Text rendering pipeline

### Fancy_Text_Print (DIALOG.CPP:595-680)
**Primary text renderer** - Handles formatted strings with shadows.

**Two overloads:**
1. `Fancy_Text_Print(int text_id, x, y, fore, back, flag, ...)` - Text ID from string table
2. `Fancy_Text_Print(char const* text, x, y, fore, back, flag, ...)` - Direct string pointer

**Process:**
1. Formats text into 512-byte stack buffer via `vsprintf`
2. Calls `Simple_Text_Print` for actual rendering
3. `text_id = TXT_NONE` or `text = nullptr` → only updates font/shadow state

### Conquer_Clip_Text_Print (DIALOG.CPP:712-787)
**Clipped text renderer** - Enforces pixel width limit with tab support.

**Parameters:**
- `width` - Maximum pixel width (clipping boundary)
- `tabs` - Optional tab stop pixel positions (null in TextLabelClass)

**Process:**
1. Formats text into 512-byte buffer
2. Iterates characters, tracking pixel position
3. Stops when next char would exceed `width`
4. Handles `\t` by advancing to next tab stop
5. Renders via `Simple_Text_Print` per segment

### Simple_Text_Print (DIALOG.CPP:315-590)
**Low-level glyph renderer** - Core font rendering with shadow/alignment.

**Handles:**
- Font selection (6pt, 8pt, LED, VCR, gradient variants)
- Shadow rendering (drop, full, light, none)
- Alignment (left, center, right via `TPF_CENTER`/`TPF_RIGHT`)
- Color gradients (`TPF_MEDIUM_COLOR`, `TPF_BRIGHT_COLOR`, `TPF_USE_GRAD_PAL`)
- X/Y spacing adjustments per font

## Inputs, outputs and side effects

**State mutations:**
- None directly (stateless rendering)
- `GadgetClass::Draw_Me` manages `Redraw` flag
- `PixWidth` can be modified externally to enable clipping

**External systems affected:**
- Video buffer (via `Simple_Text_Print` → blits to `SEENPAGE`)
- Font system (loads/selects font pointers)
- Palette (gradient palettes for `TPF_USE_GRAD_PAL`)

**Performance notes:**
- `Fancy_Text_Print` allocates 512-byte stack buffer per call
- `Conquer_Clip_Text_Print` allocates 512-byte buffer + per-char loop
- Drop shadow renders text twice (shadow + foreground)
- Not suitable for high-frequency per-frame updates without caching

## Calls into / called by

**Called by:**
- `MessageListClass::Add_Message` (MSGLIST.CPP:195) - Creates labels for chat messages
- `MessageListClass::Add_Edit` (MSGLIST.CPP:460) - Creates editable message labels
- `TextButtonClass::Draw_Me` (TEXTBTN.CPP) - Renders button text
- `ListClass::Draw_Item` (LIST.CPP) - Renders list items
- `CheckListClass`, `ColorListClass`, `EListClass` - Derived list item rendering
- `EditClass::Draw_Me` (EDIT.CPP:339) - Renders edit control text
- `GameOptionsClass::Display` (GAMEDLG.CPP:231,246) - Speed/scroll labels
- `RadarClass::Draw_Radar_Text` (RADAR.CPP:1859) - Radar overlays
- `SoundDialogClass` (SOUNDDLG.CPP:414) - Sound list items

**Calls into:**
- `GadgetClass::Draw_Me` - Base redraw check
- `Fancy_Text_Print` - Unbounded formatted text
- `Conquer_Clip_Text_Print` - Clipped text with tabs
- `Simple_Text_Print` - Ultimate glyph rendering (via above)

## Related original files
- **TXTLABEL.H** - Class declaration
- **DIALOG.CPP** - `Fancy_Text_Print`, `Conquer_Clip_Text_Print`, `Simple_Text_Print`
- **DEFINES.H** - `TextPrintType` enum, `TPF_*` flags
- **GADGET.H/.CPP** - Base `GadgetClass`
- **MSGLIST.H/.CPP** - `MessageListClass` (primary consumer)
- **TEXTBTN.H/.CPP** - `TextButtonClass` (button labels)
- **LIST.H/.CPP** - `ListClass` (list item text)
- **EDIT.H/.CPP** - `EditClass` (edit control text)
- **FUNCTION.H** - Forward declarations for text print functions
- **REAL.H** - External declarations for text print functions

## Rust port mapping

**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-ui`

**Mapped modules:**
- `cnc-core/src/ui/text_label.rs` - `TextLabel` widget
- `cnc-core/src/ui/text_print.rs` - `TextPrintType` flags, `fancy_text_print`, `conquer_clip_text_print`
- `cnc-core/src/ui/gadget.rs` - `Gadget` base trait
- `cnc-ui/src/message_list.rs` - `MessageList` using `TextLabel`
- `cnc-ui/src/widgets/text_button.rs` - `TextButton` using `TextLabel`

**Not yet ported:**
- Varargs `Fancy_Text_Print` formatting (use `format!`/`format_args!` instead)
- `TPF_USE_GRAD_PAL` gradient palette generation
- LED/VCR/Map font variants (font atlas needed)
- `Simple_Text_Print` low-level glyph blitting (requires font system)
- `SEENPAGE` double-buffer integration
- `PixWidth` clipping with tab stops

## Edge cases and compatibility notes

1. **Text pointer ownership**: `TextLabelClass` stores raw `char*` - caller must ensure buffer lifetime exceeds label. No copy made.

2. **No auto-redraw on `Set_Text`**: Caller must invalidate gadget or call `Draw_Me(true)` after text change.

3. **`PixWidth` sentinel**: `-1` = unbounded (uses `Fancy_Text_Print`), `>=0` = clipped (uses `Conquer_Clip_Text_Print`). No separate "clipping enabled" flag.

4. **Format string vulnerability**: `Fancy_Text_Print("%s", ...)` in `Draw_Me` treats `Text` as format string. If `Text` contains `%` specifiers, undefined behavior. Original code assumes plain text.

5. **512-byte buffer limit**: Both print functions use fixed 512-byte stack buffer. Long formatted strings truncate silently.

6. **`Segments` and `CRC` unused in base**: Reserved for `MessageListClass` message segmentation and deduplication.

7. **`UserData` generic field**: Used by `MessageListClass` for `magic_number` (message ID) and `CRC`.

8. **Font state leakage**: `Simple_Text_Print` uses `static int xspace, yspace` - global spacing state persists across calls. Not thread-safe.

9. **Shadow renders twice**: `TPF_DROPSHADOW`/`TPF_FULLSHADOW` draw shadow then foreground - 2× pixel writes.

10. **`TPF_LASTPOINT`/`TPF_LASTSHADOW`**: Use previous font/shadow - stateful across calls. Requires consistent call ordering.

11. **Background color**: Hardcoded to `TBLACK` (transparent black) in `Draw_Me`. No background fill - text rendered over existing pixels.

12. **`CRC` field**: 16-bit checksum for message deduplication in `MessageListClass`. Not used by `TextLabelClass` itself.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TXTLABEL.CPP at f1f0d42
Header: original/TIBERIANDAWN/TXTLABEL.H at f1f0d42
TextPrintType: original/TIBERIANDAWN/DEFINES.H:1894-1915
Renderers: original/TIBERIANDAWN/DIALOG.CPP:315-787