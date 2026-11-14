# PATH/FILE
original/TIBERIANDAWN/UTRACKER.CPP

## Purpose
Unit tracking statistics system for Tiberian Dawn internet/multiplayer games. Maintains per-unit-type counts (built, lost, etc.) with network byte-order conversion for cross-platform multiplayer synchronization.

## Where it fits in the game
Multiplayer/networking subsystem - The `UnitTrackerClass` tracks unit statistics required for internet games (win/loss reporting, scoreboards, match history). Used to serialize unit counts in network byte order for transmission between players on different architectures (big-endian vs little-endian).

## Classes, structs, enums and important data

### UnitTrackerClass
**Statistics tracker** - Tracks per-unit-type totals for network game reporting.

**Key data members:**
- `UnitTotals` (`long*`) - Dynamically allocated array of unit counts, indexed by unit type ID
- `UnitCount` (`int`) - Number of unit types (size of `UnitTotals` array)
- `InNetworkFormat` (`int` flag) - `0` = host (PC) byte order, `1` = network byte order (big-endian)

**Memory management:**
- Array allocated in constructor via `new long[unit_count]`
- Freed in destructor via `delete UnitTotals` (note: should be `delete[]` but original uses `delete`)
- `Clear_Unit_Total()` zeroes all entries via `memset`

## Functions and methods

### UnitTrackerClass(unit_count) - Constructor
**Initializes tracker for given number of unit types.**

1. Allocates `UnitTotals` array: `new long[unit_count]`
2. Stores `UnitCount = unit_count`
3. Sets `InNetworkFormat = 0` (host format)
4. Calls `Clear_Unit_Total()` to zero all entries

### ~UnitTrackerClass() - Destructor
**Frees the unit totals array.**
- `delete UnitTotals` (note: uses scalar `delete` on array)

### Increment_Unit_Total(unit_type)
**Increments the count for a specific unit type.**
- `UnitTotals[unit_type]++`
- No bounds checking (assumes valid `unit_type`)

### Decrement_Unit_Total(unit_type)
**Decrements the count for a specific unit type.**
- `UnitTotals[unit_type]--`
- No bounds checking; can go negative

### Get_All_Totals() → long*
**Returns raw pointer to the unit totals array.**
- Caller can read/modify all counts directly
- No const qualifier; allows external mutation

### Clear_Unit_Total()
**Zeros all unit totals.**
- `memset(UnitTotals, 0, UnitCount * sizeof(long))`

### To_Network_Format()
**Converts all totals from host to network byte order (big-endian).**

1. If `InNetworkFormat == 0`:
   - For each entry: `UnitTotals[i] = htonl(UnitTotals[i])`
2. Sets `InNetworkFormat = 1`
- Idempotent: safe to call multiple times

### To_PC_Format()
**Converts all totals from network to host byte order.**

1. If `InNetworkFormat == 1`:
   - For each entry: `UnitTotals[i] = ntohl(UnitTotals[i])`
2. Sets `InNetworkFormat = 0`
- Idempotent: safe to call multiple times

### Get_Unit_Total(unit_type) → int
**Returns count for a specific unit type (added post-release).**

1. Returns `0` if `UnitTotals == NULL`
2. Returns `0` if `unit_type` out of bounds (`< 0` or `>= UnitCount`)
3. Otherwise returns `UnitTotals[unit_type]`
- Added later per comment: "MBL 01.17.2020: Defined in .h, but not implemented"

## Inputs, outputs and side effects

**State mutations:**
- `UnitTotals` array contents (increment/decrement/clear)
- `InNetworkFormat` flag (byte-order conversion)
- Memory allocation/deallocation

**External systems affected:**
- Network serialization (unit stats sent over wire)
- Multiplayer scoreboard/reporting
- Match history/statistics

**Byte-order protocol:**
- Host format: little-endian (x86 PC)
- Network format: big-endian (standard Internet byte order)
- `htonl`/`ntohl` from Winsock/Berkeley sockets API

## Calls into / called by

**Called by:**
- Multiplayer game initialization → `UnitTrackerClass(unit_count)`
- Unit create/destroy events → `Increment_Unit_Total` / `Decrement_Unit_Total`
- Pre-network-send → `To_Network_Format()`
- Post-network-receive → `To_PC_Format()`
- Game reset/new match → `Clear_Unit_Total()`
- Scoreboard/stats UI → `Get_All_Totals()` or `Get_Unit_Total()`

**Calls into:**
- `new` / `delete` (memory management)
- `memset` (clear)
- `htonl` / `ntohl` (Winsock byte-order conversion)

## Related original files

- **UTRACKER.H** - Class declaration, `UnitTrackerClass` definition
- **FUNCTION.H** - Global includes, possibly unit type count constant
- Network/multiplayer modules (not in provided source tree) - Consumers of tracker data

## Rust port mapping

**Crates:** `cnc-tiberian-dawn`, `cnc-net`

**Mapped modules:**
- `cnc-net/src/unit_tracker.rs` - `UnitTracker` struct with `Vec<u32>` or `Vec<i64>`
- `cnc-tiberian-dawn/src/net/stats.rs` - Multiplayer stats integration

**Type mappings:**
- `long` (32-bit signed) → `i32` or `i64` depending on platform assumption
- `htonl`/`ntohl` → `u32::to_be_bytes`/`u32::from_be_bytes` or `byteorder` crate
- Raw pointer `Get_All_Totals()` → slice/iterator API

**Not yet ported:**
- Network byte-order conversion (platform-specific endianness handling)
- Integration with multiplayer session lifecycle
- Statistics persistence (match history)

## Edge cases and compatibility notes

1. **`delete` vs `delete[]` bug**: Original uses `delete UnitTotals` on array allocated with `new long[]` - undefined behavior in C++. Rust port must use proper `Vec` drop.

2. **No bounds checking**: `Increment_Unit_Total`/`Decrement_Unit_Total` assume valid `unit_type`. Rust should bounds-check (panic or `Result`).

3. **Signed `long` for counts**: Unit totals use signed 32-bit. Negative values possible via `Decrement_Unit_Total` underflow. Consider `u32` with saturating arithmetic.

4. **`InNetworkFormat` flag**: Prevents double-conversion. Rust port should track this in type system (e.g., `UnitTracker<HostOrder>` vs `UnitTracker<NetworkOrder>`).

5. **`Get_All_Totals` returns mutable pointer**: Allows external unsynchronized mutation. Rust should expose immutable slice or controlled mutation methods.

6. **`Get_Unit_Total` added post-release**: Original header declared it but implementation missing until 2020. Rust port should include from start.

7. **Network byte order assumes 32-bit**: `htonl`/`ntohl` operate on 32-bit values. If `long` is 64-bit on target, conversion is incorrect. Original assumes 32-bit `long`.

8. **Single tracker per game**: Designed for one global tracker instance per match (unit_count = total unit types in game).

9. **No thread safety**: Not designed for concurrent access. Multiplayer networking likely single-threaded in original.

10. **Stat semantics undefined**: Whether totals track "built", "destroyed", "current alive", or "kills" is determined by callers, not this class.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/UTRACKER.CPP at f1f0d42