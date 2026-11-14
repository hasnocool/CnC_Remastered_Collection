# PATH/FILE
original/TIBERIANDAWN/THEME.CPP

## Purpose
Music playback system for Tiberian Dawn. Manages streaming audio tracks (AUD/VAR files) with playlist logic (sequential/shuffle/repeat), scenario-gated availability, theme variations, and crossfade transitions.

## Where it fits in the game
Audio subsystem - Global `Theme` instance (GLOBALS.CPP). Called per-frame via `ThemeClass::AI()` to auto-advance tracks. Integrates with options (volume, repeat, shuffle), special flags (variations, jurassic), and scenario progression.

## Classes, structs, enums and important data

### ThemeClass
**Music manager** - Single global instance.

**State:**
- `Current` (int) - Active stream handle (-1 = none)
- `Score` (ThemeType) - Current theme
- `Pending` (ThemeType) - Next theme (or `THEME_PICK_ANOTHER`)

### ThemeType enum (partial, 34 themes)
| Theme | Filename | Text ID | Duration | Repeat | Variation | Scenario |
|-------|----------|---------|----------|--------|-----------|----------|
| `THEME_AIRSTRIKE` | AIRSTRIK.AUD | TXT_THEME_AIRSTRIKE | 200s | false | false | - |
| `THEME_80MX` | 80MX226M.AUD | TXT_THEME_80MX | 248s | false | false | - |
| `THEME_CHRG` | CHRG226M.AUD | TXT_THEME_CHRG | 256s | true | false | - |
| `THEME_CREP` | CREP226M.AUD | TXT_THEME_CREP | 222s | true | false | - |
| `THEME_DRIL` | DRIL226M.AUD | TXT_THEME_DRIL | 272s | true | false | - |
| `THEME_DRON` | DRON226M.AUD | TXT_THEME_DRON | 275s | true | false | - |
| `THEME_FIST` | FIST226M.AUD | TXT_THEME_FIST | 212s | true | false | - |
| `THEME_RECON` | RECN226M.AUD | TXT_THEME_RECON | 261s | true | false | - |
| `THEME_VOICE` | VOIC226M.AUD | TXT_THEME_VOICE | 306s | true | false | - |
| `THEME_HEAVYG` | HEAVYG.AUD | TXT_THEME_HEAVYG | 180s | true | false | - |
| `THEME_J1` | J1.AUD | TXT_THEME_J1 | 187s | true | false | Jurassic |
| `THEME_JDI_V2` | JDI_V2.AUD | TXT_THEME_JDI_V2 | 183s | true | false | - |
| `THEME_RADIO` | RADIO.AUD | TXT_THEME_RADIO | 183s | true | false | - |
| `THEME_RAIN` | RAIN.AUD | TXT_THEME_RAIN | 156s | true | false | - |
| `THEME_AOI` | AOI.AUD | TXT_THEME_AOI | 168s | true | true | - |
| `THEME_CCTHANG` | CCTHANG.AUD | TXT_THEME_CCTHANG | 193s | true | false | - |
| `THEME_DIE` | DIE.AUD | TXT_THEME_DIE | 162s | false | false | - |
| `THEME_FWP` | FWP.AUD | TXT_THEME_FWP | 53s | true | false | - |
| `THEME_IND` | IND.AUD | TXT_THEME_IND | 175s | true | false | - |
| `THEME_IND2` | IND2.AUD | TXT_THEME_IND2 | 38s | true | false | - |
| `THEME_JUSTDOIT` | JUSTDOIT.AUD | TXT_THEME_JUSTDOIT | 142s | true | false | - |
| `THEME_LINEFIRE` | LINEFIRE.AUD | TXT_THEME_LINEFIRE | 125s | true | false | - |
| `THEME_MARCH` | MARCH.AUD | TXT_THEME_MARCH | 157s | true | false | - |
| `THEME_TARGET` | TARGET.AUD | TXT_THEME_TARGET | 173s | true | false | - |
| `THEME_NOMERCY` | NOMERCY.AUD | TXT_THEME_NOMERCY | 204s | true | false | - |
| `THEME_OTP` | OTP.AUD | TXT_THEME_OTP | 182s | true | false | - |
| `THEME_PRP` | PRP.AUD | TXT_THEME_PRP | 211s | true | false | - |
| `THEME_ROUT` | ROUT.AUD | TXT_THEME_ROUT | 121s | false | true | - |
| `THEME_HEART` | HEART.AUD | TXT_THEME_HEART | 206s | false | true | - |
| `THEME_STOPTHEM` | STOPTHEM.AUD | TXT_THEME_STOPTHEM | 190s | true | false | - |
| `THEME_TROUBLE` | TROUBLE.AUD | TXT_THEME_TROUBLE | 191s | true | true | - |
| `THEME_WARFARE` | WARFARE.AUD | TXT_THEME_WARFARE | 182s | true | false | - |
| `THEME_BEFEARED` | BEFEARED.AUD | TXT_THEME_BEFEARED | 164s | false | true | - |
| `THEME_I_AM` | I_AM.AUD | TXT_THEME_IAM | 161s | false | false | - |
| `THEME_WIN1` | WIN1.AUD | TXT_THEME_WIN1 | 41s | false | true | win |
| `THEME_MAP1` | MAP1.AUD | TXT_THEME_WIN1 | 61s | false | true | map |
| `THEME_VALKYRIE` | VALKYRIE.AUD | TXT_THEME_VALK | 306s | false | true | - |

### ThemeControl struct
```cpp
struct ThemeControl {
    char const *Name;       // Base filename (e.g., "AIRSTRIK")
    int Fullname;           // Text string ID (TXT_THEME_*)
    int Scenario;           // Scenario gate (0 = always, >0 = scenario#)
    int Duration;           // Length in seconds
    bool Repeat;            // Auto-repeat in sequence
    bool Variation;         // Has .VAR variant
    bool Normal;            // In normal playlist
    bool Available;         // File exists (set by Scan)
};
```

### Special flags
- `THEME_NONE` = -1
- `THEME_FIRST` = 0
- `THEME_LAST` = THEME_COUNT-1
- `THEME_PICK_ANOTHER` = special sentinel for auto-pick

### Global availability
- `ScoresPresent` (bool) - SCORES.MIX loaded
- `SampleType` - Audio hardware type
- `Debug_Quiet` - Mute all audio

## Functions and methods

### ThemeClass()
**Constructor** - `Current=-1`, `Score=THEME_NONE`, `Pending=THEME_NONE`.

### Base_Name(theme) → char const*
Returns `_themes[theme].Name` or "No theme".

### Full_Name(theme) → char const*
Returns `Text_String(_themes[theme].Fullname)`.

### AI()
**Per-frame maintenance** - Called from main loop.

1. Checks `SampleType && !Debug_Quiet`
2. Checks `ScoresPresent && Options.ScoreVolume`
3. If `!Still_Playing() && Pending != THEME_NONE`:
   - If `Pending == THEME_PICK_ANOTHER`: `Pending = Next_Song(Score)`
   - `Play_Song(Pending)`
   - `Pending = THEME_PICK_ANOTHER`
4. `Sound_Callback()` (streaming callback)

### Next_Song(theme) → ThemeType
**Playlist logic** - Returns next track based on options.

| Condition | Behavior |
|-----------|----------|
| `theme == THEME_NONE` | Recurse with `THEME_PICK_ANOTHER` |
| `theme == THEME_PICK_ANOTHER` or `!Repeat && !IsScoreRepeat` | Pick new |
| `IsScoreShuffle` | Random `Sim_Random_Pick(FIRST, LAST)` ≠ current, `Is_Allowed()` |
| Sequential | `theme++` (wrap at LAST), `Is_Allowed()` |

### Queue_Song(theme)
**Crossfade transition** - Friendly song change.

1. Checks `ScoresPresent && SampleType && !Debug_Quiet`
2. If `Pending` empty: `Pending = theme`, `Fade_Sample(Current, THEME_DELAY)`

### Play_Song(theme) → int
**Immediate playback** - Cuts current track.

1. `Stop()` current
2. `Score = theme`
3. If `theme >= THEME_FIRST`:
   - DEMO: Checks `Scenario != 99` and file exists
   - `Current = File_Stream_Sample_Vol(Theme_File_Name(theme), 0xFF, true)`
4. Returns stream handle

### Theme_File_Name(theme) → char const*
**Constructs filename** - Static buffer.

1. If `Variation && Special.IsVariation`: tries `.VAR` first
2. Falls back to `.AUD`

### Track_Length(theme) → int
Returns `_themes[theme].Duration` (seconds).

### Stop()
**Halts playback** - `Stop_Sample(Current)`, clears `Current/Score/Pending`.

### Still_Playing() → int
Returns `Sample_Status(Current)` if valid.

### Is_Allowed(index) → bool
**Availability gate** - Checks multiple conditions.

| Check | Condition |
|-------|-----------|
| DEMO | File exists, `Scenario != 99` |
| Available | `_themes[index].Available` (file found by Scan) |
| Normal playlist | `_themes[index].Normal` |
| Variation | `Special.IsVariation && Variation && index != THEME_WIN1` |
| Scenario gate | `GameToPlay != GAME_NORMAL || Scenario <= current` |
| Jurassic | `index != THEME_J1 || Special.IsJurassic` |

### From_Name(name) → ThemeType
**Name lookup** - Tries exact filename match (case-insensitive), then substring in full name (case-sensitive).

### Scan()
**Availability scan** - Called when SCORES.MIX registered.
- For each theme: `_themes[theme].Available = CCFileClass(Theme_File_Name(theme)).Is_Available()`
- (Jurassic check commented out)

## File formats
- `.AUD` - Standard stream
- `.VAR` - Variation (enabled by `Special.IsVariation`)

## Inputs, outputs and side effects

**State mutations:**
- `Current`, `Score`, `Pending` stream handles
- `Available` flags (via `Scan`)
- Audio streaming (via `File_Stream_Sample_Vol`)

**External systems affected:**
- Audio output (streaming)
- Options (volume, repeat, shuffle)
- Special flags (variations, jurassic)
- Scenario progression

## Calls into / called by

**Called by:**
- Main loop → `AI()` per frame
- `CONQUER.CPP` / `SCORE.CPP` → `Queue_Song()`, `Play_Song()`, `Stop()`
- `INIT.CPP` / `STARTUP.CPP` → `Scan()` on MIX registration
- `SPECIAL.CPP` → `Special.IsVariation` affects playlist

**Calls into:**
- `File_Stream_Sample_Vol`, `Stop_Sample`, `Fade_Sample`, `Sample_Status`, `Sound_Callback`
- `Sim_Random_Pick` (shuffle)
- `CCFileClass::Is_Available`
- `Options` (ScoreVolume, IsScoreRepeat, IsScoreShuffle)
- `Special` (IsVariation, IsJurassic)
- `GameToPlay`, `Scenario`

## Related original files
- **THEME.H** - Class declaration, ThemeControl, ThemeType enum
- **SOUND.H/.CPP** - Streaming functions
- **OPTIONS.H/.CPP** - ScoreVolume, IsScoreRepeat, IsScoreShuffle
- **SPECIAL.H/.CPP** - IsVariation, IsJurassic
- **INIT.CPP** - `Sim_Random_Pick`
- **MIXFILE.H/.CPP** - SCORES.MIX registration triggers `Scan()`
- **GLOBALS.CPP** - `ThemeClass Theme;` global

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-audio`

**Mapped modules:**
- `cnc-audio/src/theme.rs` - ThemeClass, playlist logic
- `cnc-tiberian-dawn/src/audio/td_theme.rs` - TD theme list
- `cnc-core/src/audio/stream.rs` - File_Stream_Sample_Vol abstraction

**Not yet ported:**
- Streaming audio with `File_Stream_Sample_Vol`
- Crossfade via `Fade_Sample` + `Pending` queue
- Scenario-gated availability (`Scenario` field)
- Variation tracks (`.VAR` files)
- Shuffle with `Sim_Random_Pick` + `Is_Allowed` filter
- Win/Map specific themes (WIN1, MAP1, VALKYRIE)
- DEMO build Scenario=99 gating
- `Sound_Callback` for stream maintenance
- Text string IDs for display names

## Edge cases and compatibility notes

1. **THEME_PICK_ANOTHER sentinel**: Used for "pick next when current ends"
2. **Variation priority**: `.VAR` tried first if `Special.IsVariation`
3. **Shuffle never repeats**: `do...while(newtheme == theme)`
4. **Sequential wraps**: `theme++` with `THEME_LAST` → `THEME_FIRST`
5. **Is_Allowed filters**: Multiple gates (Normal, Variation, Scenario, Jurassic)
6. **Win/Map themes**: `THEME_WIN1`, `THEME_MAP1`, `THEME_VALKYRIE` have `Normal=false`, `Variation=true`
7. **J1 gated**: Requires `Special.IsJurassic`
8. **DEMO builds**: Extra file existence check, `Scenario=99` disables
9. **Scan() called on MIX registration**: Dynamic availability
10. **From_Name substring**: Case-sensitive full-name search as fallback
11. **Track_Length**: Pre-stored duration (seconds), no header parsing at runtime
12. **Pending queue**: Single-slot (crossfade only, no multi-queue)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/THEME.CPP at f1f0d42
