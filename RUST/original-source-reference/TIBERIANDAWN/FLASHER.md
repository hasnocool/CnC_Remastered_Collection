# PATH/FILE
original/TIBERIANDAWN/FLASHER.CPP

## Purpose
Flashing ability logic for Tiberian Dawn. Controls the flash/blush visual effect on game objects (selected units, buildings under attack, etc.). Manages per-player flash counters and blush state toggling. Integrated with rendering system for flash visual feedback.

## Where it fits in the game
Visual effects - flashing is used for selection feedback, attack indicators, and object state changes. Called once per game tick per flashing object via `Process()`. Multiplayer-aware: tracks flash state per player house (`FlashCountPerPlayer`). Integrated with `LOGIC.CPP` per-frame processing and rendering system for redraw decisions.

## Classes, structs, enums and important data

### FlasherClass
**Flashing visual effect controller** - Manages flash state for a game object.

**Key data members:**
| Field | Type | Purpose |
|-------|------|---------|
| `FlashCount` (int) | Global flash counter (decrements each tick) |
| `FlashCountPerPlayer[HOUSE_COUNT]` (int) | Per-player flash counters |
| `IsBlushing` (bool) | Current blink state (true=visible, false=hidden) |

**Flash toggle logic:**
- Each tick: decrement all non-zero `FlashCountPerPlayer[i]`
- Decrement `FlashCount` if > 0
- `IsBlushing = (FlashCount & 0x01)` - toggles every tick (odd=visible)
- Returns `true` if flash active (needs redraw), `false` if done

## Functions and methods

### FlasherClass::Process() → bool
**Per-tick flash logic processing:**

1. **Per-player flash decay**: For each house `i` in `0..HOUSE_COUNT`:
   - If `FlashCountPerPlayer[i] > 0`: decrement
2. **Global flash decay**: If `FlashCount > 0`:
   - `FlashCount--`
   - `IsBlushing = false`
   - If `FlashCount & 0x01` (odd): `IsBlushing = true`
   - Return `true` (needs redraw)
3. Return `false` (flash complete, no redraw needed)

**Called once per game tick per flashing object.**

### FlasherClass::Get_Flashing_Flags() → unsigned int
**Get bitmask of players currently flashing:**

1. Initialize `flags = 0`
2. For each house `i` in `0..HOUSE_COUNT`:
   - If `FlashCountPerPlayer[i] > 0`: `flags |= (1 << i)`
3. Return `flags`

### FlasherClass::Debug_Dump(mono) → void (CHEAT_KEYS only)
**Display flash status on monochrome screen:**
1. Set cursor to (50, 7)
2. Print `FlashCount` as 2-digit integer

## Inputs, outputs and side effects

**State mutations:**
- `FlashCountPerPlayer[i]` for each house `i`
- `FlashCount` global counter
- `IsBlushing` toggle state

**External systems affected:**
- `LOGIC.CPP` → calls `Process()` per game tick per flashing object
- Rendering system → uses `IsBlushing` and return value for redraw decisions
- `MonoClass` → `Debug_Dump()` for developer status display

## Calls into / called by

**Called by:**
- `LOGIC.CPP` → `Process()` per frame for each flashing object
- Rendering system → checks return value for redraw requirement
- UI system → `Get_Flashing_Flags()` for per-player flash state

**Calls into:**
- `MonoClass::Set_Cursor()`, `Printf()` → debug display (CHEAT_KEYS only)

## Related original files
- **FLASHER.H** - Class declarations
- **LOGIC.CPP** → per-frame `Process()` calls
- **MONO.H** → `Debug_Dump()` mono display

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/visuals/flasher.rs` - FlasherClass and Process()
- `cnc-tiberian-dawn/src/visuals/blush.rs` - Blush/flash rendering state

**Not yet ported:**
- Per-player flash tracking (`FlashCountPerPlayer`)
- Blush toggle logic (`FlashCount & 0x01`)
- Debug_Dump mono display (CHEAT_KEYS)
- Multiplayer per-house flash synchronization

## Edge cases and compatibility notes

1. **Per-player tracking**: `FlashCountPerPlayer[HOUSE_COUNT]` supports multiplayer flash per player
2. **Toggle frequency**: Flash toggles every game tick via `FlashCount & 0x01`
3. **Redraw signal**: Returns `true` while flash active → caller redraws object
4. **Completion**: Returns `false` when `FlashCount` reaches 0 → no more redraws
5. **2019 JAS modification**: Added per-player flash counters for multiplayer support
6. **CHEAT_KEYS guard**: `Debug_Dump()` only compiled with cheat keys enabled

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/FLASHER.CPP at f1f0d42
