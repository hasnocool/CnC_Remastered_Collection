# PATH/FILE
original/TIBERIANDAWN/AUDIO.CPP

## Purpose
Audio system for Tiberian Dawn. Implements sound effect playback, EVA voice system, and sound effect management with distance attenuation, panning, and contextual variations.

## Where it fits in the game
Audio subsystem - handles all sound effects, unit voice responses, and EVA (Electronic Voice Advisor) speech. Integrates with game events (combat, construction, UI) and the sidebar UI.

## Classes, structs, enums and important data

### SoundEffectNameStruct
**Sound effect definition** - maps VOC enum to filename, priority, and context.

**Fields:**
- `Name` (char const*) - Base filename (without extension)
- `Priority` (int) - Playback priority
- `Where` (ContextType) - Context: `IN_NOVAR`, `IN_JUV`, `IN_VAR`

### ContextType enum
| Value | Description |
|-------|-------------|
| `IN_NOVAR` | No variation - single file |
| `IN_JUV` | Juvenile variant (`.JUV` extension) |
| `IN_VAR` | Infantry/vehicle variation (`.V00`-`.V03`) |

### SoundEffectName array
**Complete VOC enum mapping** - 100+ entries mapping `VOC_*` constants to filenames:
- **Special voices** (commando): `BOMBIT1`, `CMON1`, `GOTIT1`, `KEEPEM1`, `LAUGH1`, `LEFTY1`, `NOPRBLM1`, `ONIT1`, `RAMYELL1`, `ROKROLL1`, `TUFFGUY1`, `YEAH1`, `YES1`, `YO1`
- **Civilian**: `GIRLOKAY`, `GIRLYEAH`, `GUYOKAY1`, `GUYYEAH1`
- **Infantry/vehicle responses**: `2DANGR1`, `ACKNO`, `AFFIRM1`, `AWAIT1`, `MOVOUT1`, `NEGATV1`, `NOPROB`, `READY`, `REPORT1`, `RITAWAY`, `ROGER`, `UGOTIT`, `UNIT1`, `VEHIC1`, `YESSIR1`
- **Juvenile variants** (`.JUV`): `BAZOOK1`, `BLEEP2`, `BOMB1`, `BUTTON`, `COMCNTR1`, `CONSTRU2`, `CRUMBLE`, `FLAMER2`, `GUN18/19/20/5/8`, `GUNCLIP1`, `HVYDOOR1`, `HVYGUN10`, `ION1`, `MGUN11`, `MGUN2`, `NUKEMISL`, `NUKEXPLO`, `OBELRAY1`, `OBELPOWR`, `POWRDN1`, `RAMGUN2`, `ROCKET1/2`, `SAMMOTR2`, `SCOLD2`, `SIDBAR1C/2C`, `SQUISH2`, `TNKFIRE2/3/4/6`, `TONE15/16/2/5`, `TOSS`, `TRANS1`, `TREEBRN1`, `TURRFIR5`, `XPLOBIG4/6/7`, `XPLODE`, `XPLOS`, `XPLOSML2`
- **No variation**: `NUYELL1-12`, `YELL1`, `MYES1`, `MCOMND1`, `MHELLO1`, `MHMMM1`, `MCOURSE1`, `MYESYES1`, `MTHANKS1`, `MTIBER1`, `CASHTURN`, `BLEEP2`, `DINOMOUT`, `DINOYES`, `DINOATK1`, `DINODIE1`, `BEACON`, etc.

### VOX enum (EVA voices)
```cpp
enum VoxType {
    VOX_NONE,
    VOX_ACCOM1,    // mission accomplished
    VOX_FAIL1,     // mission failed
    VOX_BLDG1,     // building in progress
    VOX_CONSTRU1,  // construction complete
    VOX_UNITREDY,  // unit ready
    VOX_NEWOPT1,   // new construction options
    VOX_DEPLOY1,   // cannot deploy
    VOX_GDIDEAD1,  // GDI unit destroyed
    VOX_NODDEAD1,  // Nod unit destroyed
    VOX_CIVDEAD1,  // civilian killed
    VOX_NOCASH1,   // insufficient funds
    VOX_BATLCON1,  // battle control terminated
    VOX_REINFOR1,  // reinforcements arrived
    VOX_CANCEL1,   // canceled
    VOX_BLDGING1,  // building
    VOX_LOPOWER1,  // low power
    VOX_NOPOWER1,  // insufficient power
    VOX_MOCASH1,   // need more funds
    VOX_BASEATK1,  // base under attack
    VOX_INCOME1,   // incoming missile
    VOX_ENEMYA,    // enemy planes approaching
    VOX_NUKE1,     // nuclear warhead incoming
    VOX_NOBUILD1,  // unable to build
    VOX_PRIBLDG1,  // primary building selected
    VOX_NODCAPT1,  // Nod building captured
    VOX_GDICAPT1,  // GDI building captured
    VOX_SOLD1,     // structure sold
    VOX_IONCHRG1,  // ion cannon charging
    VOX_IONREDY1,  // ion cannon ready
    VOX_NUKAVAIL,  // nuclear weapon available
    VOX_NUKLNCH1,  // nuclear launched
    VOX_UNITLOST,  // unit lost
    VOX_STRCLOST,  // structure lost
    VOX_NEEDHARV,  // need harvester
    VOX_SELECT1,   // select target
    VOX_AIRREDY1,  // airstrike ready
    VOX_NOREDY1,   // not ready
    VOX_TRANSSEE,  // Nod transport sighted
    VOX_TRANLOAD,  // Nod transport loaded
    VOX_ENMYAPP1,  // enemy approaching
    VOX_SILOS1,    // silos needed
    VOX_ONHOLD1,   // on hold
    VOX_REPAIR1,   // repairing
    VOX_ESTRUCX,   // enemy structure destroyed
    VOX_GSTRUC1,   // GDI structure destroyed
    VOX_NSTRUC1,   // Nod structure destroyed
    VOX_ENMYUNIT,  // enemy unit destroyed
    // ... and many more for multiplayer colors
}
```

## Functions and methods

### Sound_Effect(voc, coord, variation)
**Contextual sound effect** - plays at world coordinate with distance attenuation and panning.

**Logic:**
1. Calls `On_Sound_Effect()` hook (external)
2. **Legacy path (disabled):**
   - Calculates distance from tactical view center
   - Attenuates volume by distance (off-screen = 50% + distance falloff)
   - Calculates panning from X position relative to view center
   - Calls `Play_Sample()` with adjusted volume and pan

**Legacy path (disabled):**
- Distance attenuation: off-screen = 50% + distance falloff
- Panning: X offset from view center, scaled to 16-bit range

### Sound_Effect(voc, volume, variation, pan_value) → int
**General sound effect** - plays UI/game sound without spatial positioning.

**Logic:**
1. Early exit: volume=0, VOC_NONE, sound off, no sample support
2. **Extension selection:**
   - `.JUV` if `Special.IsJuvenile` and `IN_JUV`
   - `.V00/.V02` (odd/even) for infantry `IN_VAR` with negative variation
   - `.V01/.V03` (odd/even) for positive variation
   - `.AUD` default
2. **File lookup:** `_makepath(name, NULL, NULL, Name, ext)` → `MixFileClass::Retrieve()`
3. **Playback:** `Play_Sample(ptr, priority, volume, pan)`

### Speak(voice, house, coord)
**EVA speech queue** - queues voice line for playback.

**Logic:**
1. Calls `On_Speech(voice, house)` hook
2. If `coord`: calls `On_Ping(house, coord)` for map ping
3. **Legacy (disabled):** Queues voice in `SpeakQueue`, plays when `SpeakBuffer` free

### Speak_AI()
**EVA voice processor** - processes speech queue.

**Logic (disabled):**
- If `SpeakBuffer` free and `SpeakQueue` pending:
  - Loads `.AUD` file into `SpeechBuffer`
  - Plays at priority 254
  - Tracks `_last` played to avoid repeats

### Stop_Speaking()
**Stops EVA voice** - clears queue, stops current playback.

### Is_Speaking() → bool
**Checks if EVA speaking** - runs `Speak_AI()`, checks queue or `SpeechBuffer` playing.

### Sound_Effect(voc, volume, variation, pan) → int
**Core playback** - loads and plays sample.

**Logic:**
1. Early exit: no volume, VOC_NONE, sound off, no sample support
2. **Extension selection:**
   - `.JUV` if juvenile mode + `IN_JUV`
   - `IN_VAR`: `.V00/.V02` (odd/even negative variation), `.V01/.V03` (odd/even positive)
   - Default: `.AUD`
3. **File lookup:** `_makepath(name, NULL, NULL, Name, ext)` → `MixFileClass::Retrieve()`
4. **Playback:** `Play_Sample(ptr, priority, volume, pan)`

### Speak(voice, house, coord)
**EVA speech** - queues voice for player's house.

### Speak_AI()
**EVA queue processor** - plays queued voices sequentially.

### Stop_Speaking()
**Stops EVA** - clears queue, stops current playback.

### Is_Speaking() → bool
**Checks if EVA speaking** - runs `Speak_AI()`, checks queue/buffer.

## Inputs, outputs and side effects

**State mutations:**
- `SpeakQueue` - voice queue
- `CurrentVoice` - currently playing
- `SpeakBuffer` - audio buffer
- `SoundOn` - global sound toggle

**External systems affected:**
- Sound playback system
- EVA voice queue
- Map ping (`On_Ping`)

## Calls into / called by

**Called by:**
- Game events: `Sound_Effect()`, `Speak()`
- AI: `Speak_AI()` per frame
- UI: `Stop_Speaking()`, `Is_Speaking()`

**Calls into:**
- `Play_Sample()` - low-level playback
- `MixFileClass::Retrieve()` - asset loading
- `On_Sound_Effect()`, `On_Speech()`, `On_Ping()` - external hooks
- `Play_Sample()` - low-level playback

## Related original files
- **AUDIO.H** - Function declarations, VOC enum, SoundEffectName array
- **MIXFILE.CPP** - Audio file retrieval
- **SIDEBAR.CPP** - Sidebar sound effects
- **FUNCTION.H** - Core includes
- **VOX.H** - VoxType enum

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-audio`, `cnc-sfx`

**Mapped modules:**
- `cnc-audio/src/eva.rs` - EVA voice system
- `cnc-audio/src/effects.rs` - Sound effects
- `cnc-sfx/src/player.rs` - Sample playback
- `cnc-tiberian-dawn/src/audio/td_audio.rs` - TD-specific audio

**Not yet ported:**
- VOC enum with 100+ entries
- Context-sensitive variations (IN_VAR, IN_JUV, IN_NOVAR)
- EVA voice queue with priority
- Distance attenuation + panning
- Juvenile mode (`.JUV` variants)
- Infantry/vehicle variation system (`.V00`-.`V03`)
- External hooks (`On_Sound_Effect`, `On_Speech`, `On_Ping`)
- Speech queue with repeat prevention

## Edge cases and compatibility notes

1. **Variation system:** Infantry uses `.V00/.V02` (odd/even negative), `.V01/.V03` (odd/even positive). Vehicles always use vehicle table.

2. **Juvenile mode:** `Special.IsJuvenile` enables `.JUV` variants for comical sounds.

2. **Juvenile mode:** Enables `.JUV` variants.

3. **Distance attenuation:** Off-screen sounds attenuated by distance, panned by X offset.

3. **Distance attenuation:** Off-screen = 50% + distance falloff; panning from X offset.

4. **EVA queue:** Single queue, processes sequentially. `SpeakQueue` holds pending.

4. **EVA queue:** Single slot `SpeakQueue`, processes sequentially.

5. **Legacy paths:** Most implementation disabled (`#if (0)`), replaced by external hooks (`On_Sound_Effect`, `On_Speech`, `On_Ping`).

5. **Legacy disabled:** Most implementation `#if (0)`, replaced by hooks.

6. **Priority system:** `SoundEffectName[voc].Priority` used for `Play_Sample()`.

6. **Priority:** `SoundEffectName[voc].Priority` used for playback priority.

7. **Juvenile mode:** `Special.IsJuvenile` enables `.JUV` variants for comical sounds.

7. **Juvenile mode:** `.JUV` variants for comical sounds.

8. **Extension logic:** `.AUD` default, `.JUV` for juvenile, `.V00-.V03` for infantry variations.

8. **Extensions:** `.AUD` default, `.JUV` juvenile, `.V00-.V03` infantry variations.

9. **External hooks:** `On_Sound_Effect`, `On_Speech`, `On_Ping` replace internal implementation.

9. **Hooks:** `On_Sound_Effect`, `On_Speech`, `On_Ping` replace internal logic.

10. **VOC enum:** 100+ entries covering all game sounds and EVA voices.

10. **VOC enum:** 100+ entries for all sounds and EVA.

11. **File format:** `.AUD`/`.JUV`/`.V00-.V03` retrieved from MIX archives via `MixFileClass::Retrieve()`.

11. **Formats:** `.AUD`/`.JUV`/`.V00-.V03` from MIX archives.

12. **Distance attenuation:** Off-screen sounds reduced by distance, panned by X position.

12. **Distance attenuation:** Off-screen = 50% + distance falloff; panning from X offset.

13. **Priority system:** Sound effects have priority (1-20), EVA uses 254.

13. **Priority:** Sound effects 1-20, EVA uses 254 (highest).

14. **Callback system:** `On_Sound_Effect`, `On_Speech`, `On_Ping` for external integration.

14. **Hooks:** `On_Sound_Effect`, `On_Speech`, `On_Ping` for external integration.

15. **Legacy code disabled:** Most implementation in `#if (0)` blocks, replaced by hooks.

15. **Legacy disabled:** `#if (0)` blocks, replaced by external hooks.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/AUDIO.CPP at f1f0d42
