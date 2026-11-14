# PATH/FILE
original/TIBERIANDAWN/VECTOR.CPP / VECTOR.H

## Purpose
General-purpose vector math utilities for Tiberian Dawn. Provides four template container classes: a base `VectorClass<T>` (dynamic array with optional external buffer), `DynamicVectorClass<T>` (adds active count, growth, and add/delete operations), `DynamicVectorArrayClass<T,COUNT,FIRST,DEFAULT>` (fixed-size array of dynamic vectors with context switching), and `BooleanVectorClass` (bit-packed boolean array with 8:1 storage savings). Used throughout the engine for entity lists, particle systems, pathfinding, and UI state.

## Where it fits in the game
Foundation container layer - Used by virtually every subsystem:
- **AI/Unit logic**: `DynamicVectorClass` for team members, order queues, target lists
- **Pathfinding**: `DynamicVectorClass` for open/closed lists, waypoint chains
- **Particle/Visual effects**: `VectorClass` for particle arrays, animation frames
- **UI/Editor**: `DynamicVectorArrayClass` for multi-context lists (e.g., per-player unit lists), `BooleanVectorClass` for selection/highlight state
- **Map/Scenario**: `BooleanVectorClass` for cell flags, trigger conditions

## Classes, structs, enums and important data

### VectorClass<T>
**Base dynamic array template** - Owns or references contiguous array of `T`.

**Key data members:**
- `Vector` (T*) - Pointer to element array (null if empty)
- `VectorMax` (unsigned) - Allocated capacity (total elements)
- `IsAllocated` (unsigned:1) - Whether this instance owns the buffer (must `delete[]`)

**Constructors:**
- `VectorClass(size=0, array=null)` - Allocates `new T[size]` or placement-new into external `array`
- `VectorClass(VectorClass const&)` - Copy constructor (deep copy)
- `~VectorClass()` - Calls `Clear()`

**Core operations:**
- `operator[](index)` - Unchecked access (const and non-const)
- `Length()` - Returns `VectorMax` (capacity, not active count)
- `Clear()` - Frees owned buffer, resets to empty
- `Resize(newsize, array=null)` - Reallocates, preserves min(old,new) elements; `array` = placement buffer
- `operator=(VectorClass const&)` - Deep copy assignment
- `operator==(VectorClass const&)` - Element-wise equality (requires `T::operator==`)
- `ID(ptr)` - Pointer-to-index: `((ptr - &(*this)[0]) / sizeof(T))` (only valid for actual internal pointers)
- `ID(object)` - Linear search by value, returns index or -1

**Memory model:** Supports external buffer via placement new (`new((void*)array) T[size]`). `IsAllocated` tracks ownership. `Clear()` only deletes if `IsAllocated==true`.

---

### DynamicVectorClass<T> : VectorClass<T>
**Dynamic array with active count and auto-growth** - Adds "logical size" separate from capacity.

**Additional data members:**
- `ActiveCount` (int) - Number of valid elements (packed at front)
- `GrowthStep` (int) - Growth increment when `Add` exceeds capacity (default 10; 0 = no growth)

**Constructors:**
- `DynamicVectorClass(size=0, array=null)` - Base init + `GrowthStep=10, ActiveCount=0`

**Core operations (additions to base):**
- `Count()` - Returns `ActiveCount` (logical size)
- `Add(object)` - Appends; auto-grows by `GrowthStep` if `IsAllocated` or empty and `GrowthStep>0`
- `Add_Head(object)` - Prepends; shifts existing elements right via `memmove` + assignment; auto-grows
- `Delete(object)` - Finds by value (calls `ID`), then `Delete(index)`
- `Delete(index)` - Removes at index, shifts subsequent elements down via assignment
- `Delete_All()` - `ActiveCount = 0` (instant clear, retains capacity)
- `Set_Growth_Step(step)` / `Growth_Step()` - Configure growth increment
- `Resize(newsize, array)` - Calls base `Resize`, clamps `ActiveCount` to new capacity
- `Clear()` - `ActiveCount=0` + base `Clear()`
- `ID(object)` - Overrides base: searches only `[0, ActiveCount)` not full capacity

**Growth logic:** `Add`/`Add_Head` grow only if `(IsAllocated || !VectorMax) && GrowthStep > 0`. External buffers (`!IsAllocated && VectorMax>0`) cannot grow.

---

### DynamicVectorArrayClass<T, COUNT, FIRST=0, DEFAULT=FIRST>
**Fixed-size array of dynamic vectors with active context** - `COUNT` independent `DynamicVectorClass<T>` instances, one "active" at a time.

**Template parameters:**
- `COUNT` - Number of vectors in array (compile-time constant)
- `FIRST` - Minimum valid context index (default 0)
- `DEFAULT` - Initial active context (default `FIRST`)

**Data members:**
- `Collection[COUNT]` (DynamicVectorClass<T>) - The vector instances
- `Active` (int) - Currently selected context index

**Operations (all delegate to `Collection[Active]` or explicit context):**
- `Set_Active_Context(active)` - Switch active vector
- `Clear_All()` - Clear all contexts
- `Clear(context)` / `Clear()` - Clear specific or active
- `Count(context)` / `Count()` - Active count of specific or active
- `Add(object, context)` / `Add(object)` - Add to specific or active
- `Add_Head(object, context)` / `Add_Head(object)` - Prepend to specific or active
- `Delete(object, context)` / `Delete(object)` - Delete by value from specific or active
- `Delete_All(object)` - Delete value from all contexts, returns total count
- `Delete_All_Except(object, except)` - Delete from all except one context
- `Delete(index, context)` / `Delete(index)` - Delete by index
- `operator[](index)` - Access active vector's element
- `Raw()` / `Raw(context)` - Direct access to underlying `DynamicVectorClass<T>&`

**Use case:** Per-player/faction lists (e.g., `DynamicVectorArrayClass<UnitClass*, 8>` for 8 players), multi-context UI state.

---

### BooleanVectorClass
**Bit-packed boolean array** - 1 bit per element, 8x storage savings vs `VectorClass<bool>`.

**Key data members:**
- `BitArray` (VectorClass<unsigned char>) - Underlying byte storage (1 byte = 8 bools)
- `BitCount` (int) - Logical number of boolean elements (not necessarily multiple of 8)
- `Copy` (bool) - Cached reference proxy for `operator[]` (hides bit manipulation)
- `LastIndex` (int) - Index currently cached in `Copy` (-1 = none)

**Constructors:**
- `BooleanVectorClass(size=0, array=null)` - Allocates `(size+7)/8` bytes in `BitArray`
- `BooleanVectorClass(BooleanVectorClass const&)` - Copy via assignment

**Core operations:**
- `operator[](index)` (const and non-const) - Returns `bool&`/`bool const&` via `Copy` proxy. Calls `Fixup(index)` to sync cache.
- `Length()` - Returns `BitCount`
- `Is_True(index)` - Fast check without cache update (uses `Get_Bit` directly)
- `First_False()` - Returns first index with value 0, or -1. Calls `Fixup(-1)` first.
- `First_True()` - Returns first index with value 1, or -1. Calls `Fixup(-1)` first.
- `Set()` - Sets all bits to 1 via `memset(&BitArray[0], 0xFF, BitArray.Length())`
- `Reset()` - Sets all bits to 0 via `memset(&BitArray[0], 0x00, BitArray.Length())`
- `Clear()` - `Fixup()`, `BitCount=0`, `BitArray.Clear()` (frees storage)
- `Resize(size)` - Resizes `BitArray` to `(size+7)/8` bytes. New bits zeroed. `Clear()` if size=0.

**Fixup mechanism (critical):**
- `Fixup(index=-1)` - Syncs `Copy` to/from bit array when accessed index changes
- On index change: if `LastIndex != -1`, writes `Copy` back via `Set_Bit(&BitArray[0], LastIndex, Copy)`
- Then if `index != -1`, loads new bit via `Copy = Get_Bit(&BitArray[0], index)`
- Updates `LastIndex = index`
- **Constraint:** `operator[]` returns reference to `Copy`, NOT direct bit reference. Pointer to element is invalid after next `operator[]` call.

**Bit helpers (external, from function.h):** `Get_Bit(ptr, index)`, `Set_Bit(ptr, index, value)`, `First_False_Bit(ptr)`, `First_True_Bit(ptr)`

---

## Functions and methods

### VectorClass<T>

#### VectorClass(size, array)
**Creates vector with capacity `size`.** If `array` provided, placement-new into it (caller owns memory, `IsAllocated=false`). Else `new T[size]` (`IsAllocated=true`). Default constructs all `T` elements.

#### ~VectorClass()
**Destructor.** Calls `Clear()` - frees owned buffer.

#### VectorClass(VectorClass const&)
**Copy constructor.** Delegates to `operator=`.

#### operator=(VectorClass const&)
**Deep copy assignment.** `Clear()`, allocate new `T[vector.Length()]`, copy elements via `T::operator=`. Self-assignment safe (clear first).

#### operator==(VectorClass const&)
**Element-wise equality.** Returns true iff lengths equal and all `T[i] == T[i]`.

#### Clear()
**Free owned buffer.** If `Vector && IsAllocated`: `delete[] Vector`. Resets `Vector=0, IsAllocated=false, VectorMax=0`.

#### Resize(newsize, array)
**Change capacity.** Allocates new buffer (`new T[newsize]` or placement into `array`). Copies `min(old, new)` elements via assignment. Frees old buffer if `IsAllocated`. Updates `Vector, VectorMax, IsAllocated=(Vector && !array)`. `newsize=0` → `Clear()`. Returns success (bool).

#### ID(ptr)
**Pointer to index.** `((ptr - &(*this)[0]) / sizeof(T))`. **Only valid for pointers into this vector's buffer.**

#### ID(object)
**Linear search by value.** Returns first index where `(*this)[i] == object`, or -1.

---

### DynamicVectorClass<T>

#### DynamicVectorClass(size, array)
**Base init + `GrowthStep=10, ActiveCount=0`.**

#### Add(object)
**Append to end.** If `ActiveCount >= Length()`: attempt grow by `GrowthStep` (if `IsAllocated || !VectorMax` and `GrowthStep>0`). On growth failure or disabled growth: return false. Else `(*this)[ActiveCount++] = object`, return true.

#### Add_Head(object)
**Prepend.** Same growth logic as `Add`. If `ActiveCount>0`: `memmove(&(*this)[1], &(*this)[0], ActiveCount*sizeof(T))`. Then `(*this)[0]=object; ActiveCount++`.

#### Delete(object)
**Delete by value.** `index = ID(object)`. If found: `Delete(index)`. Returns bool.

#### Delete(index)
**Delete by index.** If `index < ActiveCount`: `ActiveCount--`, shift `[index+1..ActiveCount]` down via assignment. Returns bool.

#### Delete_All()
**Instant clear.** `ActiveCount = 0`. Capacity unchanged.

#### Resize(newsize, array)
**Base resize + clamp ActiveCount.** Calls `VectorClass::Resize`. If success and `Length() < ActiveCount`: `ActiveCount = Length()`.

#### Clear()
**Full reset.** `ActiveCount=0` + `VectorClass::Clear()`.

#### ID(object)
**Search active range only.** Loops `0..Count()-1` (not full capacity).

---

### DynamicVectorArrayClass<T, COUNT, FIRST, DEFAULT>

All operations delegate to `Collection[context]` (default `Active`). See class description for full list.

Key pattern: `Add(object)` → `Collection[Active].Add(object)`, `Raw()` → `Collection[Active]` for direct access.

---

### BooleanVectorClass

#### BooleanVectorClass(size, array)
**Bit-packed init.** `BitArray.Resize((size+7)/8, array)`. `BitCount=size, LastIndex=-1`.

#### BooleanVectorClass(BooleanVectorClass const&)
**Copy.** `LastIndex=-1; *this = vector`.

#### operator=(BooleanVectorClass const&)
**Copy assignment.** `Fixup()`, then copies `Copy, LastIndex, BitArray, BitCount`.

#### operator==(BooleanVectorClass const&)
**Equality.** `Fixup(LastIndex)`. Returns `BitCount == other.BitCount && BitArray == other.BitArray`.

#### operator[](index) const / non-const
**Proxy access.** If `LastIndex != index`: `Fixup(index)`. Returns `Copy` (reference). Non-const allows modification via `Copy`.

#### Fixup(index)
**Cache sync.** If `index >= BitCount`: `index = -1`. If `index != LastIndex`: if `LastIndex != -1`: `Set_Bit(&BitArray[0], LastIndex, Copy)`. If `index != -1`: `Copy = Get_Bit(&BitArray[0], index)`. `LastIndex = index`.

#### Is_True(index)
**Fast read.** If `index == LastIndex`: return `Copy`. Else direct `Get_Bit(&BitArray[0], index)`.

#### First_False() / First_True()
**Scan for value.** `Fixup(-1)` to flush cache. Call `First_False_Bit/First_True_Bit(&BitArray[0])`. If result `< BitCount`: return it. Else return -1.

#### Set() / Reset()
**Bulk set/clear.** `LastIndex=-1`. `memset(&BitArray[0], 0xFF/0x00, BitArray.Length())`.

#### Clear()
**Free storage.** `Fixup()`, `BitCount=0`, `BitArray.Clear()`.

#### Resize(size)
**Change bit count.** `Fixup()`. `oldsize=BitCount`. `success = BitArray.Resize((size+7)/8)`. `BitCount=size`. If success and grown: zero new bits `oldsize..size-1`. `size=0` → `Clear()`. Returns success.

---

## Inputs, outputs and side effects

**State mutations:**
- `VectorClass`/`DynamicVectorClass`: Buffer allocation/deallocation, element construction/destruction via `T` ctors/dtors/assignment
- `DynamicVectorClass`: `ActiveCount` changes on add/delete; capacity growth on demand
- `DynamicVectorArrayClass`: Per-context active counts, active context index
- `BooleanVectorClass`: Bit array allocation, `Copy`/`LastIndex` cache state on every `operator[]`

**External systems affected:**
- All container users (AI, pathfinding, UI, particles, map) - allocation patterns affect performance
- `BooleanVectorClass` cache invalidation: any direct `BitArray` manipulation requires `Fixup(-1)` first

---

## Calls into / called by

**Called by (representative):**
- **AI/Team logic**: `DynamicVectorClass<TeamTypeClass*>` for team members
- **Pathfinding**: `DynamicVectorClass<COORDINATE>` for waypoint lists
- **Particle system**: `VectorClass<ParticleClass>` for active particles
- **Editor/UI**: `DynamicVectorArrayClass<UnitClass*, 8>` per-player unit lists; `BooleanVectorClass` for selection state
- **Map/Triggers**: `BooleanVectorClass` for cell flags, trigger conditions

**Calls into:**
- `operator new` / `operator delete[]` (memory management)
- `memmove` (DynamicVectorClass::Add_Head, Delete)
- `memset` (BooleanVectorClass::Set/Reset)
- `Get_Bit`, `Set_Bit`, `First_False_Bit`, `First_True_Bit` (from `function.h` - bit manipulation intrinsics)
- `T` default constructor, copy assignment, destructor, equality operator (template requirements)

---

## Related original files

- **VECTOR.H** - Class declarations, template definitions (all inline in header)
- **FUNCTION.H** - Bit manipulation helpers (`Get_Bit`, `Set_Bit`, `First_False_Bit`, `First_True_Bit`)
- **HEAP.H** - `TFixedIHeapClass` (used by other systems for pooled allocation; not directly used here)
- **COORD.H** - `COORDINATE` (common `T` for pathfinding vectors)
- **TEAMTYPE.H** - `TeamTypeClass` (common `T` for AI vectors)

---

## Rust port mapping

**Crates:** `cnc-core` (foundation containers), `cnc-tiberian-dawn` (game-specific instantiations)

**Mapped modules:**
- `cnc-core/src/containers/vector.rs` - `VectorClass<T>` → `Vector<T>` (owned) + `VectorView<T>` (external buffer)
- `cnc-core/src/containers/dynamic_vector.rs` - `DynamicVectorClass<T>` → `DynamicVector<T>` with `active_len`, `growth_step`
- `cnc-core/src/containers/dynamic_vector_array.rs` - `DynamicVectorArrayClass` → `DynamicVectorArray<T, COUNT>`
- `cnc-core/src/containers/boolean_vector.rs` - `BooleanVectorClass` → `BitVector` (bit-packed, proxy reference)

**Not yet ported / design decisions:**
- Placement-new external buffer support: Rust uses `Vec::from_raw_parts` / `Box::leak` or `Vec::with_capacity` + `set_len`; external buffer = `Vec<T>` with custom allocator or `&mut [T]` view
- `BooleanVectorClass` proxy reference (`bool&` via `Copy`): Rust cannot return `&mut bool` to bit; use `BitRef` proxy type (like `bitvec` crate) or `get`/`set` methods
- `DynamicVectorClass::Add_Head` O(n) shift: document complexity; consider `VecDeque` alternative
- Growth step = 10 (hardcoded default): make configurable via `new(growth_step)`
- `DynamicVectorArrayClass` compile-time `COUNT`: Rust const generics `DynamicVectorArray<T, const COUNT: usize>`
- Template instantiation duplication: Rust monomorphization handles this automatically

---

## Edge cases and compatibility notes

1. **External buffer ownership**: `VectorClass(array)` does NOT own buffer; caller must ensure buffer outlives vector. `Clear()` will NOT free it. Rust port: use `VectorView<T>` (non-owning) vs `Vector<T>` (owning).

2. **`IsAllocated` flag**: Only true for `new T[]` allocations. Placement-new and zero-size leave it false. `Resize` with external array sets `IsAllocated = false`.

3. **`VectorClass::ID(ptr)` pointer arithmetic**: Uses `unsigned long` cast. Only valid for pointers into THIS vector's buffer. Undefined for external pointers.

4. **`DynamicVectorClass` growth restricted**: Cannot grow if using external buffer (`!IsAllocated && VectorMax>0`). `Add`/`Add_Head` return false.

5. **`DynamicVectorClass::Add_Head` uses `memmove` + assignment**: For non-trivial `T`, this calls `T::operator=` for each shifted element. Not a pure memmove.

6. **`BooleanVectorClass` proxy reference**: `operator[]` returns reference to internal `Copy` member. **Pointer to element becomes invalid on next `operator[]` call.** Cannot store `bool*` or `bool&` across calls.

7. **`Fixup` const-correctness**: `Fixup` is `const` but mutates `Copy` and `LastIndex` (marked `mutable` in practice via C++ cast). Rust: `Fixup` needs `&mut self` or interior mutability (`RefCell`/`UnsafeCell`).

8. **`First_False`/`First_True` scan entire bit array**: Calls `Fixup(-1)` first to flush pending `Copy`. Returns -1 if not found.

9. **`Resize` preserves elements**: Copies `min(old, new)` via assignment. For `BooleanVectorClass`, new bits zeroed.

10. **`DynamicVectorArrayClass::Add_Head` delegates to `Add`**: Bug in original - `Add_Head(context, object)` calls `Collection[context].Add(object)` not `Add_Head`. Preserved for parity.

11. **No iterators**: Original uses index loops. Rust port should implement `IntoIterator`, `Index`, `IndexMut`.

12. **`BooleanVectorClass::BitArray` is `VectorClass<unsigned char>`**: Uses vector's `Resize`/`Clear`. Bit count vs byte count: `BitCount` = logical bits, `BitArray.Length()` = bytes.

---

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/VECTOR.CPP and VECTOR.H at f1f0d42