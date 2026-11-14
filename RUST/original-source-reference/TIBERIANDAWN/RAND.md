# PATH/FILE
original/TIBERIANDAWN/RAND.CPP

## Purpose
Synchronized random number generator for Tiberian Dawn. Uses a fixed 256-byte permutation table for deterministic, reproducible randomness across multiplayer sessions. Provides both byte-range (`Sim_Random`) and inclusive integer-range (`Sim_IRandom`) interfaces.

## Where it fits in the game
Core deterministic RNG - Used for all gameplay randomness (damage variance, scatter, AI decisions, crate contents, map generation). Synchronized via `Seed` (GLOBALS.CPP) for multiplayer lockstep. `Sim_IRandom` used in `Garble_Message` for MP-safe message shuffling.

## Classes, structs, enums and important data

### State
- `SimRandIndex` (int, 0-255) - Current position in permutation table
- `_randvals[256]` (static unsigned char[]) - Fixed permutation of 0-255

### Permutation table
256 unique bytes covering all values 0-255 exactly once. Pre-computed for uniform distribution without modulo bias.

```cpp
static unsigned char _randvals[] = {
    0x47, 0xCE, 0xC6, 0x6E, 0xD7, 0x9F, 0x98, 0x29, 0x92, 0x0C, 0x74, 0xA2,
    0x65, 0x20, 0x4B, 0x4F, 0x1E, 0xED, 0x3A, 0xDF, 0xA5, 0x7D, 0xB5, 0xC8,
    0x86, 0x01, 0x81, 0xCA, 0xF1, 0x17, 0xD6, 0x23, 0xE1, 0xBD, 0x0E, 0xE4,
    ... (256 total entries)
};
```

### Global sync
- `Seed` (int, GLOBALS.CPP) - Initial `SimRandIndex` value, synchronized at game start
- `CustomSeed` (int) - Override for bug reproduction
- `RandSeedPtr` (long*) - Pointer to active seed

## Functions and methods

### Sim_Random() → int (0-255)
**Next permutation byte.**

```cpp
((unsigned char&)SimRandIndex)++;  // Wraps at 255→0 via unsigned char overflow
return _randvals[SimRandIndex];
```

- Increments index as unsigned char (auto-wraps 255→0)
- Returns `_randvals[SimRandIndex]`
- Period: 256 calls before repeat

### Sim_IRandom(minval, maxval) → int (inclusive)
**Random integer in range [minval, maxval].**

```cpp
return Fixed_To_Cardinal(maxval - minval, Sim_Random()) + minval;
```

- Uses `Fixed_To_Cardinal(range, byte)` = `(range * byte + 128) >> 8`
- Uniform distribution without modulo bias
- Handles `minval > maxval` via unsigned wrap (caller responsibility)

### Table generator (disabled, `#if 0`)
**Creates new permutation table** using system `rand()`.

1. Initializes `is_used[256]` = 0
2. For 256 iterations:
   - `r = (rand() * 256) / RAND_MAX`
   - If `is_used[r]==0`: `rand_val[r] = i`, `is_used[r]=1`
   - Else: retry (i--)
3. Outputs `unsigned char _RandVals[] = {0xXX, ...}` C array

## Inputs, outputs and side effects

**State mutations:**
- `SimRandIndex` increments each `Sim_Random()` call

**External systems affected:**
- All gameplay randomness (deterministic)
- Multiplayer sync (same seed → same sequence)
- Save/load (index persisted)

## Calls into / called by

**Called by:**
- `Garble_Message` (MP-safe message shuffle)
- `Coord_Scatter` (missile/spread variance)
- `AnimClass::Middle` (napalm/fire spawn randomness)
- `BulletClass` (damage variance)
- AI decisions (build/random target)
- Crate contents
- Map generation

**Calls into:**
- `Fixed_To_Cardinal` (COORD.CPP) for range scaling

## Related original files
- **RAND.H** - Declarations
- **GLOBALS.CPP** - `Seed`, `CustomSeed`, `RandSeedPtr`
- **COORD.H/.CPP** - `Fixed_To_Cardinal`
- **SYNC.H** - Sync-safe RNG usage
- **MPLAYER.CPP** - `Sim_IRandom` in `Garble_Message`
- **ANIM.CPP** - `Random_Pick` (wraps `Sim_IRandom`)

## Rust port mapping
**Crates:** `cnc-core`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-core/src/rng.rs` - Sim_Random/Sim_IRandom, permutation table
- `cnc-tiberian-dawn/src/rng/td_rng.rs` - TD-specific table
- `cnc-core/src/sync.rs` - Seed synchronization

**Not yet ported:**
- 256-byte permutation table (compile-time const)
- `SimRandIndex` as `u8` with wrapping increment
- `Fixed_To_Cardinal` integration for unbiased range
- Seed serialization for save/load
- `CustomSeed` override for debugging
- Table generator tool (for new permutations)

## Edge cases and compatibility notes

1. **Perfect shuffle**: Table contains each 0-255 exactly once → uniform, no repeats until full cycle
2. **No modulo bias**: `Fixed_To_Cardinal` uses fixed-point multiply `(range * byte + 128) >> 8`
3. **MP sync critical**: `Seed` must be identical across all peers at game start
4. **Save/load**: `SimRandIndex` must persist (included in save game)
5. **Period**: 256 calls before sequence repeats (sufficient for frame-rate use)
6. **CustomSeed**: Overrides `Seed` for reproducible bug testing
7. **Table generation**: Disabled code uses `rand()` - not cryptographically secure but sufficient for gameplay
8. **Sim_IRandom inclusive**: Both `minval` and `maxval` possible outputs
9. **Index wrap**: `unsigned char` auto-wraps 255→0
10. **Deterministic**: No system entropy used after seed initialization

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/RAND.CPP at f1f0d42
