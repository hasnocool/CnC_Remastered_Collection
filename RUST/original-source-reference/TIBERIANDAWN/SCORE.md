# PATH/FILE
original/TIBERIANDAWN/SCORE.CPP

## Purpose
Mission score screen system for Tiberian Dawn. Handles single-player and multiplayer score presentation with animated text, bar graphs (casualties), infantry animations, hall of fame entry, and LBM screenshot export.

## Where it fits in the game
Post-mission UI - Called after mission completion (win/lose). Single-player shows detailed stats with animated counters; multiplayer shows ranked leaderboard. Integrates with theme music, palette effects, and animated infantry.

## Classes, structs, enums and important data

### ScoreClass
**Main score screen controller** - Global instance `Score` (GLOBALS.CPP).

### ScoreAnimClass (base)
**Animated score element** - Timer-driven updates, auto-removal when done.

**Fields:**
- `XPos`, `YPos` (int) - Screen position
- `Timer` (CountDownTimerClass) - Animation timing
- `DataPtr` (void const*) - Text/shape data
- `Stage` (int) - Animation stage
- `Background` / `PrimaryPalette` - Rendering params

### Derived animation classes
| Class | Purpose |
|-------|---------|
| `ScoreTimeClass` | Counting hours/minutes/seconds |
| `ScoreCredsClass` | Counting credits with sound (CLOCK1/CASHTURN) |
| `ScorePrintClass` | Typewriter text reveal |
| `MultiStagePrintClass` | Fast 10-char-at-once text |
| `ScoreScaleClass` | Scale-up letter animation |

### ScoreAnimClass array
- `ScoreObjs[MAXSCOREOBJS]` - Active animations
- `Alloc_Object()` - Finds free slot, adds to array

### InfantryAnim struct
```cpp
struct InfantryAnim {
    int xpos, ypos;
    void const *shapefile;
    void const *remap;
    int anim;      // -1 = dead
    int stage;
    char delay;
    InfantryTypeClass const *Class;
} InfantryMan[NUMINFANTRYMEN];  // NUMINFANTRYMEN = 15
```

### Remap tables (256 bytes each)
- `RemapCiv` - Civilian colors
- `ScoreRemapGrey` - Grey casualty bar
- `ScoreRemapYellow` - Yellow bar
- `ScoreRemapBldg` - Building damage colors
- `ScoreRemapFBall` - Fireball colors

### Constants
| Constant | Value | Purpose |
|----------|-------|---------|
| `NUMINFANTRYMEN` | 15 | Infantry animation slots |
| `NUMFAMENAMES` | 7 | Hall of fame entries |
| `MAX_FAMENAME_LENGTH` | 12 | Name input limit |
| `MAXSCOREOBJS` | ? | Max concurrent animations |
| `SIZEGBAR` | 119 | Bar graph max width |

### Multiplayer score
```cpp
struct Fame {
    char name[MAX_FAMENAME_LENGTH];
    int score;
    int level;
};
```
- `MPlayerScore[MAX_MULTI_NAMES]` - Leaderboard entries
- `MPlayerCurGame` - Current game index
- `_colors[6]` - Player color palettes (yellow, red, blue, orange, green, gray)

## Functions and methods

### ScoreClass::Presentation()
**Single-player score screen** - Full sequence:

1. **Setup**: Creates `PseudoSeenBuff` (320×200), `TextPrintBuffer`, clears `BlitList`
2. **Loads WSA**: `SCRSCN1.WSA` background, fades in
3. **Mission stats**: Calculates kills, buildings, credits, time
4. **Animated counters**:
   - `ScoreTimeClass` for hours/minutes/seconds
   - `ScoreCredsClass` for credits (with CLOCK1/CASHTURN sounds)
   - `ScorePrintClass` for text labels
5. **Casualty bar graphs**: Three bars (GDI=cyan, Nod=red, Civilian=red)
   - Animates width `i` from 0 to max
   - Spawns infantry death animations at intervals
5. **Infantry animations**: 15 `InfantryMan` slots
   - `Draw_Bar_Graphs` triggers `New_Infantry_Anim(DO_GUN_DEATH)`
   - `Draw_InfantryMan` cycles frames, auto-switches to `DO_STAND_READY`
6. **Hall of Fame**: If high score
   - `Get_High_Score_Name` with blinking cursor (`Animate_Cursor`)
   - `ScoreScaleClass` for letter pop-in
7. **Wait for click**: `Cycle_Wait_Click`
8. **Cleanup**: Deletes all `ScoreObjs`, fades out

### Multi_Score_Presentation()
**Multiplayer score screen** - Leaderboard:

1. **Background**: `MLTIPLYR.WSA` animation
2. **Player rows**: For each `MPlayerScore[i]` with name:
   - Name (color-coded)
   - Wins count
   - Kills per game (up to `MPlayerCurGame`)
3. **Click to continue**: `TXT_MAP_CLICK2` with cycling yellow
4. **Cleanup**: Deletes animations, restores palette

### Score animation classes

#### ScoreTimeClass::Update()
Increments `Stage` up to `MaxStage` every `TimerReset` ticks.

#### ScoreCredsClass::Update()
Similar but plays `Clock1` (stages 0-21) then `CashTurn` (stage 24).

#### ScorePrintClass::Update()
Typewriter effect: reveals 1 char per frame with white-on-black outline.
- Deletes self when string ends (`\0`)

#### MultiStagePrintClass::Update()
Fast version: 10 chars per frame.

#### ScoreScaleClass::Update()
Scale-up animation: starts large, shrinks to normal over 6 stages using `TextPrintBuffer->Scale()`.

### Infantry animations

#### Draw_InfantryMen()
1. Restores background on `SysMemPage`
2. Calls `Draw_InfantryMan(k)` for all 15
3. Blits to seen page after graphs

#### Draw_InfantryMan(index)
- Skips if `anim == -1` (dead)
- Draws current frame with `SHAPE_FADING|SHAPE_CENTER|SHAPE_WIN_REL`
- Decrements `delay`; when <0:
  - Advances `stage`
  - If anim complete:
    - Death anim (`>=DO_GUN_DEATH`) → `anim = -1` (remove)
    - Else → `New_Infantry_Anim(DO_STAND_READY)`

#### New_Infantry_Anim(index, anim)
Sets `anim`, `stage=0`, `delay` (1 for death, random 0-15 for idle).

### Draw_Bar_Graphs(i, gkilled, nkilled, ckilled)
Draws three horizontal bars on `LogicPage`:
- GDI: cyan (LTCYAN) at y=4-5
- Nod: red at y=16-17
- Civilian: red at y=28-29
Width = `MIN(i, killed)`; triggers infantry death at `i/11` intervals.

### Animate_Cursor(pos, ypos)
Blinking cursor for hall of fame name entry:
- Green/black line at `HALLFAME_X + pos*6`
- Toggles every 5 ticks (`_timer`)

### Get_High_Score_Name()
Hall of fame name entry:
- Renders `NUMFAMENAMES` (7) existing entries
- Input loop: BACKSPACE, RETURN, A-Z/space
- `ScoreScaleClass` for each new letter pop-in
- Returns entered name

### Call_Back_Delay(time)
Wrapper: `Call_Back()` + `CountDownTimerClass` wait.
- `ControlQ` cheat (LCTRL+Q) skips delay

### Cycle_Wait_Click()
Waits for mouse click or keypress (ESC/ENTER/SPACE/LMB).

### TextBlitClass
Deferred blitting system:
- `Add(x,y,dx,dy,w,h)` - Queues blit
- `Update()` - Executes all queued blits `TextPrintBuffer->Blit(HidPage, ...)`
- `Clear()` - Resets queue

### LBM Export (WRITE_LBM)
`CCWrite_LBM_File` - Writes 320×200 screenshot as LBM (IFF/ILBM):
- `CCWrite_BMHD` - Bitmap header
- `CCWrite_CMAP` - Palette (DPaint → LBM conversion)
- `CCWrite_BODY` - Planar RLE compression
- `CCWrite_Row` - Run-length encoding per plane

## Inputs, outputs and side effects

**State mutations:**
- Animation objects (`ScoreObjs[]`)
- Infantry animation state (`InfantryMan[]`)
- Palette (fade in/out, luminance increase)
- Screen buffers (`PseudoSeenBuff`, `TextPrintBuffer`, `SysMemPage`)

**External systems affected:**
- Theme music (`THEME_WIN1`/`THEME_WIN2`/`THEME_LOSE1`)
- Sound effects (`CLOCK1`, `CASHTURN`, `KEYSTROK`, `VOC_SCREAM*`)
- High score table (hall of fame)
- Mouse cursor

## Calls into / called by

**Called by:**
- `CONQUER.CPP` / `MAINLOOP.CPP` after mission end
- Win/lose conditions trigger `Score.Presentation()` or `Multi_Score_Presentation()`

**Calls into:**
- `GraphicBufferClass`, `GraphicViewPortClass`
- `CC_Draw_Shape`, `TextBlitClass`, `TextPrintBuffer`
- `ThemeClass`, `Fade_Palette_To`, `Play_Sample`
- `MixFileClass::Retrieve` (WSA, AUD, CLOCK1, CASHTURN)
- `Animate_Frame`, `Interpolate_2X_Scale`, `Increase_Palette_Luminance`
- `CountDownTimerClass`, `Keyboard`, `WWMouse`
- `Random_Pick`, `Sound_Effect`

## Related original files
- **SCORE.H** - Class declarations, animation classes
- **SCOREANIM.H** - ScoreAnimClass hierarchy
- **THEME.H/.CPP** - Music queue
- **SOUND.H/.CPP** - Play_Sample
- **ANIMATION.H/.CPP** - WSA playback
- **PALETTE.H/.CPP** - Fade_Palette_To, luminance
- **MOUSE.H/.CPP** - WWMouse
- **TEXTBLIT.H** - TextBlitClass
- **LBM.H** - CCWrite_LBM_File (conditional)

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-ui`

**Mapped modules:**
- `cnc-ui/src/score_screen.rs` - ScoreClass::Presentation
- `cnc-ui/src/score/animations.rs` - ScoreAnimClass hierarchy
- `cnc-ui/src/score/casualty_bars.rs` - Draw_Bar_Graphs
- `cnc-ui/src/score/infantry.rs` - InfantryMan animations
- `cnc-tiberian-dawn/src/ui/td_score.rs` - TD-specific layouts
- `cnc-core/src/export/lbm.rs` - LBM writer

**Not yet ported:**
- Typewriter text animations (`ScorePrintClass`, `MultiStagePrintClass`)
- Scale-up letter animation (`ScoreScaleClass`)
- Credit counter with sound sync (`ScoreCredsClass`)
- Infantry casualty animations with death/stand cycles
- Blinking cursor (`Animate_Cursor`)
- Hall of fame name entry with `ScoreScaleClass`
- Deferred blitting (`TextBlitClass`)
- LBM export with planar RLE
- Palette luminance boost for WSA background
- Cheat key `ControlQ` (LCTRL+Q) to skip delays

## Edge cases and compatibility notes

1. **Double-buffered rendering**: `PseudoSeenBuff` → `HidPage` → `SeenBuff` (2X scale)
2. **StreamLowImpact**: Set true during delays to reduce background processing
3. **ControlQ cheat**: LCTRL+Q sets `time=0` in `Call_Back_Delay`
4. **Infantry only for Nod**: Comment says "GDI wouldn't execute people like that"
5. **Bar graph scaling**: `MIN(i, killed)` clamps width; `SIZEGBAR=119` max
6. **Infantry death trigger**: Every 11 pixels of bar growth (`i/11`)
7. **Three casualty categories**: Split 15 infantry into 5 per category
8. **Multiplayer score shift**: If `MPlayerCurGame == MAX_MULTI_GAMES-1`, shifts kills left
9. **Font spacing**: `FontXSpacing = 0` for tight score text
10. **LBM format**: Planar 8-bit, RLE per row per plane (DPaint compatible)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/SCORE.CPP at f1f0d42
