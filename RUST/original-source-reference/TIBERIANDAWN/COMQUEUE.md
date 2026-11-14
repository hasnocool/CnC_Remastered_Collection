# PATH/FILE
original/TIBERIANDAWN/COMQUEUE.CPP

## Purpose
Network communication queue system for Tiberian Dawn (disabled in release build). Circular buffer implementation for send/receive packet queues with ACK tracking, response time statistics, and debug visualization. Similar to COMBUF.CPP but uses ring buffer indices instead of active-index indirection.

## Where it fits in the game
Network layer (legacy/disabled) - Alternative implementation to COMBUF.CPP. Compiled out with `#if (0)` and `#ifndef DEMO`. Provides packet queuing for multiplayer with latency monitoring.

## Classes, structs, enums and important data

### CommQueueClass
**Dual ring-buffer communication queue** - Separate send/receive circular buffers.

**Key data members:**
- `MaxSend` / `MaxReceive` (int) - Buffer capacity
- `MaxPacketSize` (int) - Max bytes per packet
- `SendQueue` / `ReceiveQueue` (array) - Ring buffer entries
- `SendNext` / `SendEmpty` (int) - Ring buffer head/tail indices
- `SendCount` (int) - Active send entries
- `ReceiveNext` / `ReceiveEmpty` / `ReceiveCount` - Receive ring indices
- `SendTotal` / `ReceiveTotal` (long) - Lifetime counters
- `DelaySum` / `NumDelay` / `MeanDelay` / `MaxDelay` - RTT stats
- `DebugOffset` / `DebugSize` / `DebugNames` / `DebugMaxNames` - Packet decode config

### SendQueueType
**Send ring entry:**
- `Buffer` (char[MaxPacketSize])
- `IsActive` (char)
- `IsACK` (char)
- `FirstTime` / `LastTime` (unsigned long) - Timestamps
- `SendCount` (unsigned long) - Retransmissions
- `BufLen` (int)

### ReceiveQueueType
**Receive ring entry:**
- `Buffer` (char[MaxPacketSize])
- `IsActive` (char)
- `IsRead` (char)
- `IsACK` (char)
- `BufLen` (int)

## Functions and methods

### CommQueueClass(numsend, numreceive, maxlen)
**Constructor** - Allocates ring buffers and per-entry packet buffers.

### ~CommQueueClass()
**Destructor** - Frees all allocations.

### Init()
**Full reset** - Clears ring indices, counts, stats, entry flags.

### Queue_Send(buf, buflen) → int (1=OK, 0=full)
**Enqueue at `SendEmpty` (tail).**
1. Returns 0 if `SendCount==MaxSend` or `SendQueue[SendEmpty].IsActive`
2. Sets flags at `SendEmpty`, copies data
3. `SendEmpty = (SendEmpty + 1) % MaxSend`
4. `SendCount++`, `SendTotal++`

### UnQueue_Send(buf, buflen) → int (1=OK, 0=empty)
**Dequeue from `SendNext` (head).**
1. Returns 0 if `SendCount==0` or `SendQueue[SendNext].IsActive==0`
2. Copies data, clears entry
3. `SendNext = (SendNext + 1) % MaxSend`
4. `SendCount--`

### Next_Send() → SendQueueType*
**Peek at head** (`SendNext`).

### Get_Send(index) → SendQueueType*
**Peek at `SendNext + index` (modulo MaxSend).**

### Queue_Receive(buf, buflen) → int (1=OK, 0=full)
**Enqueue at `ReceiveEmpty` (tail).** Same pattern as send.

### UnQueue_Receive(buf, buflen) → int (1=OK, 0=empty)
**Dequeue from `ReceiveNext` (head).**

### Next_Receive() / Get_Receive(index)
**Peek receive ring.**

### Add_Delay(delay) - Same as COMBUF
**Running average with 256-sample window.**

### Avg_Response_Time() / Max_Response_Time() / Reset_Response_Time()
**Stats accessors.**

### Configure_Debug(offset, size, names, maxnames)
**Packet decode setup** - Same as COMBUF.

### Mono_Debug_Print(refresh) / Mono_Debug_Print2(refresh)
**Monochrome debug UI** - Same as COMBUF (compact vs verbose).

## Key differences from COMBUF.CPP

| Aspect | COMQUEUE | COMBUF |
|--------|----------|--------|
| Buffer type | Ring buffer (circular) | Active-index array |
| Dequeue | Head index (`SendNext`) | Active-index array shift |
| Indirection | None (direct ring) | `SendIndex[]` mapping |
| Compilation | Disabled (`#if 0`) | Active |
| Packet overflow check | `buflen > MaxPacketLen` warning | Silent truncation |

## Inputs, outputs and side effects

**State mutations:**
- Ring buffer indices
- Packet buffer copies
- RTT statistics

**External systems affected:**
- Network layer (if enabled)
- Debug monitor

## Calls into / called by

**Called by:** (disabled) Network manager
**Calls into:**
- `memcpy`
- `ConnectionClass::Command_Name`
- `Mono_Clear_Screen/Set_Cursor/Printf`
- `WWLIB32_H` guard

## Related original files
- **COMQUEUE.H** - Class declarations
- **COMBUF.H/.CPP** - Active implementation
- **CONNECT.H/.CPP** - ConnectionClass
- **WWLIB32.H** - Mono debug functions

## Rust port mapping
**Crates:** `cnc-net` (reference only - disabled in release)

**Not ported:** This implementation is compiled out. COMBUF.CPP is the active version.

## Edge cases and compatibility notes

1. **Disabled in release**: `#if (0)` and `#ifndef DEMO` guards
2. **Ring buffer**: Simpler than COMBUF's active-index; no O(n) shift on dequeue
3. **MaxPacketLen check**: Warns on oversized receive packets
4. **Same debug UI**: Identical `Mono_Debug_Print`/`Mono_Debug_Print2` to COMBUF
5. **Same RTT algorithm**: 256-sample window with roundoff
6. **Same packet header assumption**: `CommHdr` at buffer start

## Evidence and review status
Status: Reviewed (disabled code)
Source: original/TIBERIANDAWN/COMQUEUE.CPP at f1f0d42
