# PATH/FILE
original/TIBERIANDAWN/OPTIONS.CPP

## Purpose
Game options system for Tiberian Dawn. Manages player-configurable settings (video, audio, gameplay, UI) with INI persistence, palette adjustment (HSV-based), game speed normalization, and cheat/unlock flags via obfuscated INI keys.

## Where it fits in the game
Settings subsystem - Global `Options` instance (GLOBALS.CPP). Loaded at startup (`STARTUP.CPP` → `Load_Settings`), applied on scenario load (`Set()`), saved on exit (`Save_Settings`). Controls palette rendering, scroll speed, game speed, volumes, and gameplay modifiers.

## Classes, structs, enums and important data

### OptionsClass
**Global settings controller** - Single instance `Options`.

**Settings (with defaults):**
| Setting | Default | Range | Purpose |
|---------|---------|-------|---------|
| `GameSpeed` | `TIMER_SECOND/TICKS_PER_SECOND` (4) | 1-8 | Game tick rate |
| `ScrollRate` | `TIMER_SECOND/TICKS_PER_SECOND` (4) | 1-8 | Map scroll speed |
| `Volume` | 0xE0 (224) | 0-255 | SFX volume |
| `ScoreVolume` | 0x90 (144) | 0-255 | Music volume |
| `Contrast` | 0x80 (128) | 0-255 | Palette contrast |
| `Color` | 0x80 (128) | 0-255 | Palette saturation |
| `Tint` | 0x80 (128) | 0-255 | Palette hue shift |
| `Brightness` | 0x80 (128) | 0-255 | Palette brightness |
| `AutoScroll` | true | bool | Edge scroll without click |
| `IsDeathAnnounce` | false (DE/FR: true) | bool | Unit death voice |
| `IsScoreRepeat` | false | bool | Music repeat |
| `IsScoreShuffle` | false | bool | Music shuffle |
| `IsFreeScroll` | false | bool | 360° scroll (cheat) |
| `SlowPalette` | 1 | int | Fade speed |

### Cheat/unlock flags (set via obfuscated INI keys)
| Flag | INI Key | Obfuscated Value | Effect |
|------|---------|------------------|--------|
| `Special.IsNamed` | `TrueNames` | `PARM_TRUENAME` | Show unit names |
| `MPlayerMax=6` | `Players` | `PARM_6PLAYER` | 6-player MP |
| `Special.IsThreePoint` | `Rotation` | `PARM_3POINT` | 3-point turns |
| `Special.IsSeparate` | `Helipad` | `PARM_HELIPAD` | Separate helipad |
| `Special.IsMCVDeploy` | `MCV` | `PARM_MCV` | MCV undeploy |
| `Special.IsRoad` | `Bibs` | `PARM_BIB` | No building bibs |
| `Special.IsTreeTarget` | `TreeTarget` | `PARM_TREETARGET` | Target trees w/o Shift |
| `Special.IsDefenderAdvantage=false` | `Combat` | `PARM_COMBAT` | Attacker advantage |
| `Special.IsVariation` | `Scores` | `PARM_SCORE` | Custom scores |
| `Special.IsSmartDefense/IsScatter` | `CombatIQ` | `PARM_IQ` | Smart AI |
| `Special.IsGross` | `Overrun` | `PARM_SQUISH` | Squish marks |
| `Special.IsJuvenile` | `Sounds` | `PARM_HUMAN` | Juvenile mode |
| `Special.IsScrollMod` | `Scrolling` | `PARM_SCROLLING` | No scroll over tabs |

### _adjust table (Normalize_Delay)
```cpp
static int _adjust[4][8] = {
    {2,2,1,1,1,1,1,1},   // delay=1
    {3,3,3,2,2,2,1,1},   // delay=2
    {5,4,4,3,3,2,2,1},   // delay=3
    {7,6,5,4,4,4,3,2}    // delay=4
};
// Index: [delay-1][GameSpeed] for delay 1-4
```

## Functions and methods

### OptionsClass()
**Constructor** - Sets all defaults (see table above). `IsDeathAnnounce` defaults to true for DE/FR builds.

### One_Time()
**One-time init** - Calls `Set_Score_Vol(ScoreVolume)`.

### Process()
**Options UI** - Stub (empty in this file; actual UI in GOPTIONS.CPP).

### Set_Shuffle(on) / Set_Repeat(on)
**Music playback flags** - Direct boolean setters.

### Set_Score_Volume(volume)
**Music volume** - `Bound(0,255)`, stores, calls `Set_Score_Vol()`.

### Set_Sound_Volume(volume, feedback)
**SFX volume** - `Bound(0,255)`, stores. If `feedback`: `Sound_Effect(VOC_BLEEPY3)`.

### Set_Brightness(brightness)
**Brightness** - `Brightness = 0x40 + Fixed_To_Cardinal(0x80, brightness)` → `Adjust_Palette()` → `Set_Palette()` if in main loop.

### Get_Brightness() → int
Returns `Cardinal_To_Fixed(0x80, Brightness-0x40)` (0-255).

### Set_Color(color) / Get_Color()
**Saturation** - Direct store + `Adjust_Palette()` + live palette update.

### Set_Contrast(contrast) / Get_Contrast()
**Contrast** - `Contrast = 0x40 + Fixed_To_Cardinal(0x80, contrast)` → `Adjust_Palette()` + live update.

### Set_Tint(tint) / Get_Tint()
**Hue shift** - Direct store + `Adjust_Palette()` + live update.

### Adjust_Palette(oldpal, newpal, brightness, color, tint, contrast) const
**HSV palette transformation** - Currently disabled (`#if 0`).

**Algorithm (disabled):**
1. For each palette index (skip 255):
   - RGB → HSV via `Convert_RGB_To_HSV`
   - `v = (v * brightness) / 0x80` (brightness)
   - `v = (((v-0x80) * contrast) / 0x80) + 0x80` (contrast around midpoint)
   - `s = (s * color) / 0x80` (saturation)
   - `h = (h * tint) / 0x80` (hue)
   - HSV → RGB via `Convert_HSV_To_RGB`
   - Write to `newpal`

### Load_Settings()
**Loads from CONQUER.INI `[Options]` section.**

**Standard settings:** `GameSpeed`, `ScrollRate`, `Brightness`, `Volume`, `ScoreVolume`, `Contrast`, `Color`, `Tint`, `AutoScroll`, `IsScoreRepeat`, `IsScoreShuffle`, `IsDeathAnnounce`, `IsFreeScroll`, `SlowPalette` (all via `WWGetPrivateProfileInt` + setters).

**Obfuscated cheat keys:** Reads string keys, passes through `Obfuscate()`, compares to `PARM_*` constants:
- `TrueNames` → `PARM_TRUENAME` → `Special.IsNamed`
- `Players` → `PARM_6PLAYER` → `MPlayerMax=6`
- `Rotation` → `PARM_3POINT` → `Special.IsThreePoint`
- `Helipad` → `PARM_HELIPAD` → `Special.IsSeparate`
- `MCV` → `PARM_MCV` → `Special.IsMCVDeploy`
- `Bibs` → `PARM_BIB` → `Special.IsRoad`
- `TreeTarget` → `PARM_TREETARGET` → `Special.IsTreeTarget`
- `Combat` → `PARM_COMBAT` → `Special.IsDefenderAdvantage=false`
- `Scores` → `PARM_SCORE` → `Special.IsVariation`
- `CombatIQ` → `PARM_IQ` → `Special.IsSmartDefense/IsScatter`
- `Overrun` → `PARM_SQUISH` → `Special.IsGross`
- `Sounds` → `PARM_HUMAN` → `Special.IsJuvenile`
- `Scrolling` → `PARM_SCROLLING` → `Special.IsScrollMod`

### Save_Settings()
**Saves to CONQUER.INI `[Options]`** - Writes standard settings only (not cheat flags).

### Set()
**Applies current settings** - Calls all setters for palette/audio. Called on scenario load.

### Normalize_Delay(delay) → int
**Game-speed-adjusted delay** for synchronized animations.

```
if delay == 0: return 0
if delay < 5: return _adjust[delay-1][GameSpeed]
else: return (delay * 8) / (GameSpeed + 1)
```

- `GameSpeed` 1-8 (index 0-7 in `_adjust`)
- Delays 1-4 use lookup table; 5+ use linear scaling

### Fixup_Palette()
**Re-applies palette** - Calls `Adjust_Palette()` with current settings.

### Normalize_Sound(volume) → int
**Volume scaling** - `Fixed_To_Cardinal(volume, Volume)`.

## HSV Palette Adjustment (disabled)
The `Adjust_Palette` implementation converts RGB→HSV, applies multipliers, converts back. Currently `#if 0`'d out - likely replaced by shader or disabled for performance.

## Inputs, outputs and side effects

**State mutations:**
- All settings fields
- Live palette (`GamePalette`) via `Set_Palette()`
- Audio volumes via `Set_Score_Vol()`/`Set_Sound_Volume()`
- INI file (`CONQUER.INI`)

**External systems affected:**
- Renderer (palette)
- Audio (volumes)
- Game speed (tick rate)
- Scroll behavior
- Multiplayer limits
- Special gameplay flags

## Calls into / called by

**Called by:**
- `STARTUP.CPP` → `Load_Settings()`, `Read_Setup_Options()`
- Scenario load → `Set()`
- Main menu → `Process()`, `Save_Settings()`
- Game loop → `Normalize_Delay()` (animations), `Normalize_Sound()` (SFX)
- Palette changes → `Fixup_Palette()`

**Calls into:**
- `WWGetPrivateProfileInt/String`, `WWWritePrivateProfileInt`
- `Obfuscate()` (INIT.CPP) for cheat keys
- `Fixed_To_Cardinal`, `Cardinal_To_Fixed` (COORD.CPP)
- `Bound()` (clamping)
- `Adjust_Palette` (internal)
- `Set_Palette`, `Set_Score_Vol`, `Sound_Effect`
- `Special` globals (cheat flags)
- `MPlayerMax` (MP limit)

## Related original files
- **OPTIONS.H** - Class declaration
- **GLOBALS.CPP** - `OptionsClass Options;` global
- **STARTUP.CPP** - `Load_Settings()`, `Read_Setup_Options()`
- **CONQUER.CPP** - `Main_Game` calls `Set()`
- **SPECIAL.H/.CPP** - Cheat flags (`Special.Is*`)
- **INIT.CPP** - `Obfuscate()`, `PARM_*` constants
- **COORD.H/.CPP** - `Fixed_To_Cardinal`, `Cardinal_To_Fixed`
- **SOUND.H/.CPP** - `Set_Score_Vol`, `Sound_Effect`
- **PALETTE.H/.CPP** - `Set_Palette`
- **MULTI.H** - `MPlayerMax`

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`, `cnc-launcher`

**Mapped modules:**
- `cnc-core/src/config/options.rs` - OptionsClass
- `cnc-launcher/src/settings.rs` - INI load/save, UI
- `cnc-core/src/video/palette.rs` - Adjust_Palette (HSV)
- `cnc-tiberian-dawn/src/config/td_options.rs` - TD-specific defaults

**Not yet ported:**
- HSV palette adjustment (disabled but spec'd)
- `Normalize_Delay` with `_adjust` table
- Obfuscated cheat key system (`Obfuscate` + `PARM_*`)
- Live palette updates during gameplay
- GameSpeed-indexed delay scaling
- Volume normalization via fixed-point
- Cheat flags propagated to `Special` globals

## Edge cases and compatibility notes

1. **Brightness/Contrast offset**: Stored as `0x40 + Fixed_To_Cardinal(0x80, value)` to map 0-255 → 0x40-0xC0 range
2. **GameSpeed range**: 1-8 (indexes `_adjust[0-3][0-7]`)
3. **Delay normalization**: Crude due to 15Hz timer resolution; assumes synchronized `GameSpeed`
4. **Cheat keys**: Only loaded, never saved (security by obscurity via `Obfuscate`)
5. **Live palette**: `InMainLoop` guard prevents `Set_Palette` during init
6. **Default volumes**: `Volume=0xE0` (SFX), `ScoreVolume=0x90` (music)
7. **Regional defaults**: `IsDeathAnnounce=true` for German/French builds
8. **SlowPalette**: Integer (1=slow), not boolean
9. **FreeScroll**: Cheat flag (`IsFreeScroll`), not in standard UI
10. **Adjust_Palette disabled**: `#if 0` - likely shader-based in remaster

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/OPTIONS.CPP at f1f0d42
