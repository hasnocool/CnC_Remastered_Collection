# PATH/FILE
original/TIBERIANDAWN/MIXFILE.CPP

## Purpose
MIX archive system for Tiberian Dawn. Manages multiple `.MIX` files (Westwood's custom archive format) with on-demand caching, CRC-based binary search lookup, and transparent file access via `CCFileClass`.

## Where it fits in the game
Asset pipeline - all game data (art, audio, maps, INI, movies) stored in MIX files. Registered at startup (`INIT.CPP`), cached selectively, accessed via `MixFileClass::Retrieve()` or `CCFileClass` which calls `MixFileClass::Offset()`.

## Classes, structs, enums and important data

### MixFileClass : LinkClass (doubly-linked list)
**Single MIX archive** - Maintains index in memory, data block cached on demand.

**Key data members:**
- `Filename` (char*) - Basename only (e.g., "CONQUER.MIX")
- `Buffer` (SubBlock*) - Index array (Count entries)
- `Data` (char*) - Cached raw data block (DataSize bytes), NULL if uncached
- `Count` (long) - Number of files in archive
- `DataSize` (long) - Total size of all file data
- `First` (static MixFileClass*) - Head of global registration list

### FileHeader (on disk)
```c
struct FileHeader {
    long count;   // Number of files
    long size;    // Total data size
};
```

### SubBlock (index entry, in Buffer[])
```c
struct SubBlock {
    long CRC;     // CRC32 of uppercase filename
    long Offset;  // Offset from start of data block
    long Size;    // File size in bytes
};
```
- Sorted by CRC for binary search (`bsearch`)

## Functions and methods

### MixFileClass(filename) - Constructor
1. `file.Set_Name(filename)` → `Filename = strdup(file.File_Name())`
2. `Force_CD_Available(RequiredCD)` - Fails if CD not present
3. If file available:
   - Opens, reads `FileHeader`
   - Allocates `Buffer[Count]`, reads index
   - Closes file
4. `Data = NULL` (uncached)
5. Links into global list: `Add_Tail(*First)` or `First = this`

### ~MixFileClass() - Destructor
1. Frees `Filename`, `Data[]`, `Buffer[]`
2. Unlinks from global list

### Cache() → bool - Instance method
**Loads data block into RAM.**
1. Returns true if already cached
2. `Data = new char[DataSize]`
3. Opens file, seeks past header + index
4. Reads `DataSize` bytes
5. Validates `actual == DataSize` else `Fatal("Corrupt .MIX file")`
6. Returns true on success

### Cache(filename) → bool - Static
**Finds MIX by name, calls instance `Cache()`.**

### Free() - Instance
**Uncaches data block only** - `delete[] Data; Data = 0;` (keeps index)

### Free(filename) → bool - Static
**Finds MIX, calls instance `Free()`.**

### Free_All() - Static
**Deletes all registered MIX files** - `while (First) delete First;`

### Retrieve(filename) → void const*
**Direct RAM access** - Calls `Offset()`, returns `realptr` (NULL if not cached).

### Finder(filename) → MixFileClass*
**Linear search** of global list by basename (case-insensitive suffix match).

### Offset(filename, realptr, mixfile, offset, size) → bool
**Core lookup** - Searches all registered MIX files.

**Algorithm:**
1. Uppercase filename, compute `CRC = Calculate_CRC(name, len)`
2. Create `SubBlock key{CRC}`
3. For each MIX in list:
   - `bsearch(&key, Buffer, Count, sizeof(SubBlock), compfunc)`
   - If found:
     - `*mixfile = ptr`
     - `*size = block->Size`
     - `*offset = block->Offset`
     - `*realptr = ptr->Data ? ptr->Data + block->Offset : NULL`
     - If uncached: `*offset += index_size + header_size` (disk offset)
     - Return true
4. Return false

### compfunc - Binary search comparator
Compares `SubBlock.CRC` (long)

## MIX file format (on disk)
```
[FileHeader: 8 bytes]
[count][size]

[SubBlock index: Count * 12 bytes]
[CRC][Offset][Size]...
(sorted by CRC)

[File data: DataSize bytes]
<file1><file2>...
```

## Inputs, outputs and side effects

**State mutations:**
- Global list registration (constructor/destructor)
- Data block allocation/free (Cache/Free)
- File I/O (open/read/seek/close)

**External systems affected:**
- `CCFileClass` (uses `Offset()` for transparent access)
- Asset loading (`Retrieve()` for direct pointers)
- Startup caching (`INIT.CPP` calls `Cache()` on key MIXes)
- CD verification (`Force_CD_Available` in constructor)

## Calls into / called by

**Called by:**
- `INIT.CPP`: Constructor for each MIX, `Cache()` on critical files
- `CCFileClass`: `Offset()` for file open/read/seek
- `MixFileClass::Retrieve()`: Direct asset pointers (shapes, fonts, palettes)
- `CONQUER.CPP`: `Force_CD_Available` reloads MIXes on CD swap
- `SAVELOAD.CPP`: MIX access during save/load

**Calls into:**
- `CCFileClass` (file I/O)
- `Calculate_CRC` (filename hashing)
- `bsearch` (binary search)
- `Force_CD_Available` (CD check)
- `Fatal()` (corrupt/error)
- `strdup`/`free`, `new[]`/`delete[]`

## Related original files
- **MIXFILE.H** - Class declaration, SubBlock, FileHeader
- **CCFILE.H/.CPP** - CCFileClass (consumer)
- **FUNCTION.H** - Global `First`, `RequiredCD`
- **INIT.CPP** - Startup registration order
- **CONQUER.CPP** - CD swap reload
- **CRC.H** - Calculate_CRC

## Rust port mapping
**Crates:** `cnc-core`, `cnc-tiberian-dawn`

**Mapped modules:**
- `cnc-core/src/assets/mix.rs` - MixFileClass, MIX format parser
- `cnc-core/src/assets/archive.rs` - Archive trait, registry
- `cnc-tiberian-dawn/src/assets/td_mix.rs` - TD-specific MIX list

**Not yet ported:**
- Global linked list registration (`First`, `LinkClass`)
- On-demand caching with `Data`/`Buffer` separation
- CRC32 filename index with binary search
- Disk offset calculation for uncached files
- `Force_CD_Available` integration in constructor
- Case-insensitive basename matching in `Finder`
- `Fatal()` on corrupt read
- Static `Cache(filename)` / `Free(filename)` helpers
- `Free_All()` bulk cleanup
- `Retrieve()` direct pointer access

## Edge cases and compatibility notes

1. **Constructor CD check**: Fails hard if `Force_CD_Available` fails (exit or DLL return)
2. **Index always resident**: `Buffer` loaded at construction, never freed until destruct
3. **Data block optional**: `Cache()`/`Free()` manage separately for memory control
3. **Cached = fast path**: `Retrieve()`/`Offset()` return direct pointer if cached
4. **Uncached = disk offset**: `Offset()` returns file-relative offset for `CCFileClass`
5. **CRC collision**: Not handled (assumes unique per archive)
6. **Case handling**: Filenames uppercased before CRC; `Finder` uses `stricmp` on basename
7. **Registration order matters**: First match wins (INIT.CPP orders by priority)
8. **Addon MIX scan**: `SC*.MIX`/`SS*.MIX` wildcard at startup
9. **Never cached**: `GENERAL.MIX`, `MOVIES.MIX`, `SCORES.MIX`, `SPEECH.MIX`, `SOUNDS.MIX`
10. **Cached at startup**: `CCLOCAL.MIX`, `UPDATE*.MIX`, `CONQUER.MIX`, `TRANSIT.MIX`
11. **DEMO build**: Different MIX set (`DEMO.MIX`, `DEMOL.MIX`, `DEMOM.MIX`)
12. **Japanese**: Extra `LANGUAGE.MIX`
13. **Corrupt detection**: Validates full `DataSize` read
14. **Memory alloc**: `MEM_CLEAR|MEM_REAL` not used here (plain `new[]`)
15. **LinkClass**: Doubly-linked list via `Add_Tail`/`Remove`/`Get_Next`

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/MIXFILE.CPP at f1f0d42
