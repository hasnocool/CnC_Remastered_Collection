# PATH/FILE
original/TIBERIANDAWN/DEBUG.CPP

## Purpose
Debug subsystem for Tiberian Dawn. Provides keyboard debug mode processing (CHEAT_KEYS only) and logic timer self-regulation for smooth animation. Used for developer debugging - screenshots, cheat keys, monochrome debug display, and performance monitoring.

## Where it fits in the game
Debug/diagnostics - called from `LOGIC.CPP` game loop via `Self_Regulate()` every frame. `Debug_Key()` handles unrecognized keystrokes when `Debug_Flag` is true. Windows-only debug features (screenshots via PCX). Integrated with `MonoClass` monochrome system for real-time status display.

## Classes, structs, enums and important data

### Debug_Key(input) → void
**Debug mode keyboard processing:**
- Called for every keystroke game doesn't recognize
- Only active when `Debug_Flag` is true
- `#ifdef CHEAT_KEYS` - only compiled with cheat keys enabled

**Key handlers:**
| Key | Action |
|-----|--------|
| `KN_L` | Toggle `NetMonoMode` / `NewMonoMode` |
| `KN_K + CTRL` | Start `ScreenRecording = true` |
| `KN_K` | (Disabled) PCX screenshot via `Write_PCX_File()` |
| `KN_P` | Pause loop: `Keyboard::Clear()` while `!Keyboard::Check()` calling `Self_Regulate()`/`Sound_Callback()` |
| `KN_O` | Spawn `AIRCRAFT_ORCA` at mouse position |

**NEVER-blocked keys** (`#ifdef NEVER`):
- `KN_F5`: Toggle `Special.IsShowPath`
- `KN_F6`: Radar cell check debug print
- `KN_F9 + CTRL`: `HouseClass::As_Pointer(HOUSE_GOOD)->Blowup_All()`
- `KN_F10 + CTRL`: `HouseClass::As_Pointer(HOUSE_BAD)->Blowup_All()`

### Self_Regulate() → void
**Logic timer regulation for smooth animation:**
- Called every display loop
- Timer interval: `UPDATE_INTERVAL = TIMER_SECOND`
- Displays status on monochrome monitor when `MonoClass::Is_Enabled()`
- Shows: Heap size, Largest block, Total free, Fragmentation
- Also calls `_lastobject->Debug_Dump(mono)` and `Logic.Debug_Dump(mono)`

### External dependencies
| Symbol | Purpose |
|--------|---------|
| `Debug_Flag` | Global debug mode flag |
| `ScreenRecording` | External bool for screen capture mode |
| `NetMonoMode`, `NewMonoMode` | Mono display modes |
| `MonoPage` | Current monochrome page |
| `MonoArray[]` | Monochrome display array |
| `SeenBuff` | Graphics buffer |
| `CurrentPalette` | Current palette pointer |

## Functions and methods

### Debug_Key(unsigned input)
**Debug keyboard handler:**

1. Early return if `!input || input & KN_BUTTON` (ignore button events)
2. Only processes if `Debug_Flag` is true
3. Switch on key code:
   - `KN_L`: Toggle mono mode flags
   - `KN_K + CTRL`: Enable `ScreenRecording`
   - `KN_K`: PCX screenshot (disabled via `#if (0)`)
   - `KN_P`: Pause until key released, calling `Self_Regulate()`+`Sound_Callback()` each loop
   - `KN_O`: Spawn Orca aircraft at mouse position
4. `#ifdef NEVER` block: F5/F6/F9/F10 cheat keys (compiled out)

### Self_Regulate(void)
**Per-frame logic timer regulation:**

1. Uses `CountDownTimerClass DebugTimer(BT_SYSTEM)` with `UPDATE_INTERVAL = TIMER_SECOND`
2. When timer fires:
   - Reset timer: `DebugTimer.Set(UPDATE_INTERVAL)`
   - If `MonoClass::Is_Enabled()`:
     - Get current mono object: `MonoClass::Get_Current()`
     - Set default attribute: `mono->Set_Default_Attribute(2)`
     - Page 0 logic:
       - Get first selected object via `CurrentObject[0]`
       - Skip inactive objects
       - Call `_lastobject->Debug_Dump(mono)`
       - Call `Logic.Debug_Dump(mono)`
       - Print heap stats: Heap size, Largest, Ttl Free, Frag
     - Copy back: `*MonoClass::Get_Current() = *mono`
     - Save: `MonoArray[MonoPage] = *mono`

## Inputs, outputs and side effects

**State mutations:**
- `ScreenRecording` (set true on KN_K+CTRL)
- `NetMonoMode` / `NewMonoMode` (toggled on KN_L)
- Mono display page content
- Heap statistics (read-only via `Heap_Size()`, `Ram_Free()`)

**External systems affected:**
- `LOGIC.CPP` → `Self_Regulate()` called every game frame
- `MonoClass` → monochrome debug display
- `Keyboard` → pause loop
- `Sound_Callback()` → audio during pause
- `AircraftClass` → spawn cheat on KN_O
- `HouseClass` → Blowup_All cheat keys (NEVER block)

## Calls into / called by

**Called by:**
- `LOGIC.CPP` → `Self_Regulate()` per display loop
- Input system → `Debug_Key()` for unrecognized keystrokes

**Calls into:**
- `CountDownTimerClass::Time()`, `Set()` → timer management
- `MonoClass::Is_Enabled()`, `Get_Current()`, `Set_Default_Attribute()`, `Clear()`, `Set_Cursor()`, `Printf()` → mono display
- `ObjectClass::Debug_Dump()` → object status
- `Logic.Debug_Dump()` → logic status
- `Heap_Size()`, `Ram_Free()`, `Total_Ram_Free()` → memory stats
- `Keyboard::Clear()`, `Keyboard::Check()` → pause loop
- `Sound_Callback()` → audio during pause
- `Write_PCX_File()` → screenshot (disabled)
- `AircraftClass::AircraftClass()`, `Unlimbo()` → spawn cheat
- `HouseClass::As_Pointer()`, `Blowup_All()` → cheat keys (NEVER block)

## Related original files
- **DEBUG.H** - Header declarations
- **FUNCTION.H** - Core includes
- **FILEPCX.H** - PCX file format
- **LOGIC.CPP** → calls `Self_Regulate()` per frame
- **MONO.H** → monochrome display system
- **AIRCRAFT.CPP** → `KN_O` spawn cheat
- **HOUSE.CPP** → `Blowup_All()` cheat keys

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/debug/debug_key.rs` - Debug_Key() keyboard processing
- `cnc-tiberian-dawn/src/debug/self_regulate.rs` - Self_Regulate() timer logic

**Not yet ported:**
- Windows PCX screenshot (`Write_PCX_File`)
- Monochrome debug display (`MonoClass`)
- Cheat key spawn aircraft (`KN_O`)
- Cheat key blowup all houses (`KN_F9/F10 + CTRL`)
- Pause loop with `Self_Regulate()`/`Sound_Callback()`
- Heap statistics display
- `Debug_Flag` global state
- `ScreenRecording` external variable

## Edge cases and compatibility notes

1. **CHEAT_KEYS only**: Entire file guarded by `#ifdef CHEAT_KEYS` - not in release builds
2. **NEVER block**: F5/F6/F9/F10 cheat keys compiled out with `#ifdef NEVER`
3. **Screenshot disabled**: `KN_K` PCX code guarded by `#if (0)`
4. **Pause loop**: `KN_P` blocks until key released, calls `Self_Regulate()`+`Sound_Callback()` each iteration
5. **Mono page 0 only**: Debug display only on page 0
6. **Object selection**: Uses `CurrentObject[0]` as debug target
7. **Heap fragmentation**: Calculated as `Total_Ram_Free - Ram_Free`
8. **Timer-based**: Only runs once per `TIMER_SECOND` interval

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/DEBUG.CPP at f1f0d42
