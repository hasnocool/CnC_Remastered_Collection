# PATH/FILE
original/TIBERIANDAWN/ALLOC.CPP

## Purpose
Custom memory allocator for the Westwood Library (Tiberian Dawn). Provides a wrapper around DOS/DPMI memory management with support for protected mode and real mode allocations, DPMI memory locking, debug validation headers, and memory statistics tracking. Originally designed for Watcom C/C++ 32-bit DOS extender environment.

## Where it fits in the game
Core infrastructure - **Westwood Library (WWLIB)** memory subsystem. All game allocations route through `Alloc()`/`Free()` instead of raw `malloc`/`free`. Used by every subsystem: graphics, audio, map, units, UI, INI parsing, etc. The allocator was the foundation for memory management in the original DOS game.

## Classes, structs, enums and important data

### MemoryFlagType (from wwmem.h, referenced here)
**Allocation control flags** - Bitwise OR'd into `Alloc()` call.

| Flag | Value | Purpose |
|------|-------|---------|
| `MEM_NORMAL` | 0x00 | Standard allocation |
| `MEM_CLEAR` | 0x01 | Zero-initialize allocated block |
| `MEM_NEW` | 0x02 | Called by C++ `new` operator |
| `MEM_LOCK` | 0x04 | DPMI-lock pages (prevent swap/page-out) |
| `MEM_REAL` | 0x08 | Force real-mode allocation (DPMI 0x100) |

### Global state variables
| Variable | Type | Purpose |
|----------|------|---------|
| `MinRam` | `unsigned long` | Lowest free RAM observed (worst-case) |
| `MaxRam` | `unsigned long` | Highest total allocated at any point |
| `TotalRam` | `unsigned long` | Total heap size (cached at init) |
| `Memory_Calls` | `unsigned long` | Active allocation count (debug) |
| `RequestedSystemRam` | `unsigned long` | Target heap size (default 8 MB) |
| `LargestRamBlock` | `unsigned long` | Largest contiguous block found at init |
| `Memory_Error` | `void (*)(void)` | Callback on allocation failure |
| `Memory_Error_Exit` | `void (*)(char*)` | Callback on header corruption (fatal) |

### Header layout (prepended to every allocation)
**LONG_ALIGNMENT mode (default, 4-byte aligned):**
```
[VALIDATION DWORD] [FLAGS BYTE] [FLAGS^0xFF BYTE] [PAD] [PAD] [USER DATA...]
       4 bytes           1           1            1      1
```
- Validation DWORD = `(actual_block_size ^ 0xFFFFFFFF) & 0xFFFF0000` (Watcom-specific)
- Flags byte + XOR complement for corruption detection
- 2 padding bytes for MEM_LOCK/MEM_REAL allocations

**Non-LONG_ALIGNMENT mode (legacy):**
```
[ENCODED FLAGS BYTE] [USER DATA...]
        1 byte
```
- Encoded: `flags | (((flags ^ 0x07) & 0x07) << 5)` - uses upper 3 bits as checksum

### DPMI Lock/Unlock
- `DPMI_Lock(ptr, size)` - Int 0x31, AX=0x0600 (lock physical pages)
- `DPMI_Unlock(ptr, size)` - Int 0x31, AX=0x0601 (unlock pages)
- Used for `MEM_LOCK` allocations (critical data: audio buffers, video memory)

## Functions and methods

### Alloc(bytes_to_alloc, flags) → void*
**Primary allocation entry point.** Handles initialization, protected/real mode selection, header insertion, DPMI locking, zeroing, and statistics.

**Process:**
1. **First-call initialization** (`_allocinit == 0`):
   - Calls `Largest_Mem_Block()` (external asm) to find largest free block
   - Subtracts 1 KB for heap header, rounds down to 64 KB boundary
   - Clamps to `RequestedSystemRam` (8 MB default)
   - Does provisional `malloc`/`free` to reserve heap space
   - Sets `TotalRam = Total_Ram_Free(MEM_NORMAL)`
2. **Size adjustment**: Adds header space (4 bytes normal, 8 bytes if MEM_LOCK)
3. **Protected mode attempt**: `malloc(bytes_to_alloc)` unless `MEM_REAL` flag
4. **Real mode fallback** (if protected fails or `MEM_REAL`):
   - DPMI Int 0x31, AX=0x100 (allocate real mode memory)
   - Size in paragraphs: `(bytes + 19) >> 4`
   - Returns selector:DX, constructs far pointer
   - Stores selector in header for later free
5. **Failure handling**: Calls `Memory_Error()` callback, returns NULL
6. **MEM_LOCK handling**: Stores `original_size` in header before flags, calls `DPMI_Lock()`
7. **Header insertion**:
   - Normal: Validation DWORD + flags + flags^0xFF + 2 padding bytes
   - MEM_LOCK/MEM_REAL: flags + flags^0xFF + 2 zero padding bytes
8. **MEM_CLEAR**: `memset(retval, 0, original_size)`
9. **Statistics update**: Tracks `MinRam`, `MaxRam`, increments `Memory_Calls`
10. Returns pointer **past header** (user data start)

### Free(pointer) → void
**Frees allocation with header validation and DPMI unlock.**

**Process:**
1. Null check
2. **Header validation** (back up 4 or 1 byte):
   - LONG_ALIGNMENT: Checks flags ^ flags^0xFF match; validates validation DWORD against block size (Watcom heap header)
   - Non-LONG_ALIGNMENT: Checks encoded flag checksum
   - On corruption: calls `Memory_Error_Exit()` with pointer address
3. **MEM_LOCK handling**: Reads original_size from header, calls `DPMI_Unlock()`, adjusts pointer to header start
4. **MEM_REAL handling**: DPMI Int 0x31, AX=0x101 (free real mode memory) using stored selector
5. **Protected mode**: Standard `free(pointer_to_header)`
6. Decrements `Memory_Calls`

### Resize_Alloc(original_ptr, new_size_in_bytes) → void*
**Reallocates block via `realloc()`.** Note: Does NOT preserve header validation logic - treats pointer as raw malloc block. Limited utility in this allocator design.

**Process:**
1. Casts pointer to `unsigned long*`
2. Calls `realloc(temp, new_size_in_bytes)`
3. On failure: calls `Memory_Error()`, returns NULL
4. Returns new pointer

### Ram_Free(flags) → long
**Returns largest free contiguous block.** Currently returns `_memmax()` (Watcom RTL). Commented fallback to `Largest_Mem_Block()`.

### Heap_Size(flags) → long
**Returns total heap size.** Returns cached `TotalRam` (initialized on first Alloc or first call).

### Total_Ram_Free(flags) → long
**Returns total available free RAM.** Returns `_memavl()` (Watcom RTL). Commented fallback to `Largest_Mem_Block()`.

### Largest_Mem_Block() → unsigned long (external)
**Assembly routine** (in separate .ASM file) that walks DPMI/DPMI/VCPI memory maps to find largest contiguous free block. Declared `extern "C"`.

### DPMI_Lock(ptr, size) → void
**Locks physical pages** via DPMI Int 0x31 AX=0x0600. Uses linear address (EBX:CX) and size (SI:DI). No error handling (carry flag ignored).

### DPMI_Unlock(ptr, size) → void
**Unlocks physical pages** via DPMI Int 0x31 AX=0x0601. Same register convention.

## Inputs, outputs and side effects

### State mutations
- Global statistics: `MinRam`, `MaxRam`, `TotalRam`, `Memory_Calls`, `LargestRamBlock`
- Heap memory (via malloc/free/DPMI)
- Physical page lock state (MEM_LOCK)

### External systems affected
- **All game subsystems** - every allocation goes through this
- **DPMI host** - real mode memory, page locking
- **Watcom RTL heap** - protected mode allocations
- **Debug callbacks** - `Memory_Error`/`Memory_Error_Exit` for OOM/corruption

## Calls into / called by

### Called by
- **Every game subsystem** - graphics (shapes, palettes), audio (samples), map (cells, overlays), units, UI, INI, etc.
- **C++ `new`/`delete` overrides** (if MEM_NEW used)
- **Scenario load/save** - large buffer allocations

### Calls into
- **Watcom RTL**: `malloc`, `free`, `realloc`, `_memmax`, `_memavl`, `_heapchk`
- **DPMI services** (Int 0x31): 0x100 (alloc real), 0x101 (free real), 0x0600 (lock), 0x0601 (unlock)
- **External asm**: `Largest_Mem_Block()`
- **DOS/BIOS**: `segread`, `int386x` (via DOS extender)
- **Callbacks**: `Memory_Error()`, `Memory_Error_Exit()`

## Related original files
- **WWMEM.H** - MemoryFlagType enum, function prototypes, MEM_* constants
- **MONO.H** - Debug logging (used in DPMI_Lock/Unlock)
- **External .ASM** - `Largest_Mem_Block` implementation
- **All .CPP files** - Every allocation site

## Rust port mapping

**Crates:** `cnc-core` (memory subsystem), `cnc-tiberian-dawn` (game-specific config)

**Mapped modules:**
- `cnc-core/src/memory/allocator.rs` - Alloc/Free/Resize_Alloc with header validation
- `cnc-core/src/memory/stats.rs` - MinRam/MaxRam/TotalRam/Memory_Calls tracking
- `cnc-core/src/memory/dpmi.rs` - DPMI lock/unlock abstractions (stubbed for non-DOS)
- `cnc-core/src/memory/error.rs` - Memory_Error/Error_Exit callback system

**Not yet ported:**
- Real mode DPMI allocation (Int 0x31 AX=0x100/0x101) - DOS-specific
- DPMI page locking (Int 0x31 AX=0x0600/0x0601) - requires kernel support
- Watcom-specific validation DWORD (relies on RTL heap header layout)
- `Largest_Mem_Block()` asm routine - platform-specific memory probing
- `_memmax()` / `_memavl()` / `_heapchk()` - Watcom RTL functions
- 8 MB default heap reservation logic
- Logging to `mem.txt` (LOGGING compile flag)

## Edge cases and compatibility notes

1. **Header validation is Watcom-specific**: The validation DWORD reads the Watcom malloc header (located at `ptr - sizeof(size_t)`) and XORs it. This will NOT work with other allocators (glibc, jemalloc, etc.).

2. **Real mode allocation is DOS/DPMI only**: Uses DPMI Int 0x31 services. No equivalent on modern OSes.

3. **MEM_LOCK requires kernel privilege**: DPMI page locking prevents paging to disk. On Linux/Windows, would need `mlock()`/`VirtualLock()` with appropriate permissions.

4. **Initialization side effect**: First `Alloc()` call does a provisional `malloc`/`free` of ~8 MB to "claim" heap space. This fragments the heap on modern allocators.

5. **Resize_Alloc is unsafe**: It bypasses header validation and assumes the pointer is a raw malloc block. Using it on a block allocated with MEM_LOCK or MEM_REAL will corrupt state.

6. **Memory_Error_Exit is fatal**: Called on header corruption (buffer underflow/overflow detected). Original game would crash with diagnostic message.

7. **LONG_ALIGNMENT default**: 4-byte alignment with 4-byte header (8 with MEM_LOCK). Non-LONG_ALIGNMENT is 1-byte header with bit-packed flags.

8. **RequestedSystemRam = 8 MB**: Hardcoded default. Game would fail to start if < 8 MB available after DPMI/DPMI/VCPI.

9. **MinRam/MaxRam track "worst case"**: MinRam = lowest free bytes ever seen; MaxRam = highest allocated bytes ever seen. Useful for profiling memory pressure.

10. **MEM_NEW flag**: Indicates allocation from C++ `new`. Not used for any special handling in this file.

11. **DPMI_Lock/Unlock ignore carry flag**: No error checking on lock failure. Commented-out code shows a page-touch loop alternative.

12. **Selector storage for real mode**: Real mode allocations store the DPMI selector (16-bit) in header at `pointer - 4` (longptr[-1]) for Free() to recover.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/ALLOC.CPP at f1f0d42