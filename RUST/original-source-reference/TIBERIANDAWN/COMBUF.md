# PATH/FILE
original/TIBERIANDAWN/COMBUF.CPP

## Purpose
Network communication buffer system for Tiberian Dawn. Manages send/receive packet queues with ACK tracking, response time statistics, and debug visualization for multiplayer synchronization.

## Where it fits in the game
Network layer - provides reliable packet queuing for multiplayer games. Used by ConnectionClass/CommManager for sending/receiving game commands, unit orders, and state updates with acknowledgment and latency monitoring.

## Classes, structs, enums and important data

### CommBufferClass
**Dual-queue communication buffer** - separate send and receive queues with configurable capacity.

**Key data members:**
- `MaxSend` / `MaxReceive` (int) - Queue capacity
- `MaxPacketSize` (int) - Max bytes per packet
- `SendQueue` / `ReceiveQueue` (array) - Queue entries
- `SendIndex` / `ReceiveIndex` (int array) - Active entry indices
- `SendCount` / `ReceiveCount` (int) - Active entry count
- `SendTotal` / `ReceiveTotal` (long) - Lifetime counters
- `DelaySum` / `NumDelay` / `MeanDelay` / `MaxDelay` (unsigned long) - RTT stats
- `DebugOffset` / `DebugSize` / `DebugNames` / `DebugMaxNames` - Packet decode config

### SendQueueType
**Send queue entry:**
- `Buffer` (char[MaxPacketSize]) - Packet data
- `IsActive` (char) - Slot in use
- `IsACK` (char) - Acknowledged
- `FirstTime` / `LastTime` (unsigned long) - Send timestamps
- `SendCount` (unsigned long) - Retransmission count
- `BufLen` (int) - Actual data length

### ReceiveQueueType
**Receive queue entry:**
- `Buffer` (char[MaxPacketSize]) - Packet data
- `IsActive` (char) - Slot in use
- `IsRead` (char) - Processed by game
- `IsACK` (char) - ACK sent
- `BufLen` (int) - Actual data length

### CommHdr (internal debug struct)
- `MagicNumber` (unsigned short)
- `Code` (unsigned char) - Command type
- `PacketID` (unsigned long) - Sequence ID

## Functions and methods

### CommBufferClass(numsend, numreceive, maxlen)
**Constructor** - Allocates queues and per-entry buffers.

1. Stores capacities
2. Allocates `SendQueue[MaxSend]`, `ReceiveQueue[MaxReceive]`
3. Allocates `SendIndex[MaxSend]`, `ReceiveIndex[MaxReceive]`
4. Allocates `Buffer[MaxPacketSize]` for each entry
5. Calls `Init()`

### ~CommBufferClass()
**Destructor** - Frees all allocations in reverse order.

### Init()
**Full reset** - Clears all queues and statistics.
- `SendTotal=ReceiveTotal=0`
- `DelaySum=NumDelay=MeanDelay=MaxDelay=0`
- `SendCount=ReceiveCount=0`
- All entries: `IsActive=0`, `IsACK=0`, timestamps=0, `SendCount=0`, `BufLen=0`
- Debug config cleared

### Init_Send_Queue()
**Send-only reset** - Clears send queue, preserves receive queue and stats.

### Queue_Send(buf, buflen) → int (1=OK, 0=full)
**Enqueue packet for sending.**
1. Returns 0 if `SendCount == MaxSend`
2. Finds first inactive slot
3. Sets `IsActive=1`, `IsACK=0`, timestamps=0, `SendCount=0`, `BufLen=buflen`
4. `memcpy` packet data
5. Records index in `SendIndex[SendCount]`
6. Increments `SendCount`, `SendTotal`

### UnQueue_Send(buf, buflen, index) → int (1=OK, 0=empty)
**Dequeue by active index** (0 = oldest active).
1. Returns 0 if empty or entry inactive
2. Copies data to `buf` (if not NULL), sets `*buflen`
3. Clears entry flags
4. Shifts `SendIndex[index..SendCount-2]` left
5. Decrements `SendCount`

### Get_Send(index) → SendQueueType*
**Peek at active entry by index** (0 = next to send).

### Queue_Receive(buf, buflen) → int (1=OK, 0=full)
**Enqueue received packet.**
1. Returns 0 if `ReceiveCount == MaxReceive`
2. Finds first inactive slot
3. Sets `IsActive=1`, `IsRead=0`, `IsACK=0`, `BufLen=buflen`
4. `memcpy` packet data
5. Records index in `ReceiveIndex[ReceiveCount]`
6. Increments `ReceiveCount`, `ReceiveTotal`

### UnQueue_Receive(buf, buflen, index) → int (1=OK, 0=empty)
**Dequeue received packet by active index.**
1. Returns 0 if empty or inactive
2. Copies data, sets `*buflen`
3. Clears entry
4. Shifts `ReceiveIndex` left
5. Decrements `ReceiveCount`

### Get_Receive(index) → ReceiveQueueType*
**Peek at received entry by index.**

### Add_Delay(delay)
**Update RTT statistics** - Running average with 256-sample window.

```
if NumDelay == 256:
    DelaySum -= MeanDelay  // Remove oldest contribution
    DelaySum += delay
    roundoff = (DelaySum & 0xFF) > 127 ? 1 : 0
    MeanDelay = (DelaySum >> 8) + roundoff  // Divide by 256
else:
    NumDelay++
    DelaySum += delay
    MeanDelay = DelaySum / NumDelay

MaxDelay = max(MaxDelay, delay)
```

### Avg_Response_Time() → unsigned long
Returns `MeanDelay`.

### Max_Response_Time() → unsigned long
Returns `MaxDelay`.

### Reset_Response_Time()
Clears `DelaySum`, `NumDelay`, `MeanDelay`, `MaxDelay`.

### Configure_Debug(offset, size, names, maxnames)
**Sets up packet decode for debug display.**
- `offset`: Byte offset of ID in packet
- `size`: 1/2/4 bytes (uint8/16/32)
- `names`: String array for ID → name mapping
- `maxnames`: Array bound

### Mono_Debug_Print(refresh)
**Full-screen debug visualization** (MaxSend > 16).
- Draws boxed UI with send/receive queue columns
- 3 columns × 16 rows each side
- Shows PacketID, SendCount/IsRead, ACK flag
- `refresh=1`: Clears screen, draws borders/headers

### Mono_Debug_Print2(refresh)
**Verbose debug format** (MaxSend ≤ 16).
- Single column per queue, 1 entry per row
- Shows: PacketID, SendCount/IsRead, Command Name, Data ID, Decoded Name, ACK
- Uses `Configure_Debug` config to decode app-specific ID
- `ConnectionClass::Command_Name(code)` for command names

## Inputs, outputs and side effects

**State mutations:**
- Queue entry allocation/free
- Packet buffer copies
- RTT statistics accumulation
- Debug screen output (monochrome text mode)

**External systems affected:**
- ConnectionClass packet processing
- Multiplayer synchronization
- Latency monitoring for game speed adjustment

## Calls into / called by

**Called by:**
- ConnectionClass / CommManager
- Network layer for packet queuing
- Game loop for send/receive processing

**Calls into:**
- `memcpy` for packet copies
- `ConnectionClass::Command_Name()` for debug
- `Mono_Clear_Screen/Set_Cursor/Printf` for debug UI
- `WWLIB32_H` guard for Windows monochrome console

## Related original files
- **COMBUF.H** - Class declarations, queue structs
- **CONNECT.H/.CPP** - ConnectionClass, Command_Name
- **MANGER.H/.CPP** - CommManager (likely consumer)
- **IPX*.CPP** - Transport layer
- **SESSION.H/.CPP** - Session management
- **WWLIB32.H** - Mono_* debug functions

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-net`

**Mapped modules:**
- `cnc-net/src/comm_buffer.rs` - CommBufferClass
- `cnc-net/src/packet_queue.rs` - Send/Receive queues
- `cnc-net/src/rtt_tracker.rs` - Response time statistics

**Not yet ported:**
- Dual-queue with active-index indirection (SendIndex/ReceiveIndex)
- Per-entry retransmission tracking (SendCount, timestamps)
- RTT running average with 256-sample circular behavior
- Debug packet decode (offset/size/names)
- Monochrome console debug UI (Mono_*)
- Packet ACK tracking (IsACK, IsRead)
- MaxSend/MaxReceive configurable capacities
- MaxPacketSize per-entry buffers

## Edge cases and compatibility notes

1. **Active index indirection**: `SendIndex`/`ReceiveIndex` map logical queue position (0 = oldest active) to physical slot; allows O(1) dequeue from middle
2. **Retransmission tracking**: `SendCount` incremented by manager on resend; `FirstTime`/`LastTime` for RTT calculation
3. **RTT window**: After 256 samples, becomes exponential moving average (1/256 weight per new sample)
4. **Roundoff handling**: `(DelaySum & 0xFF) > 127` adds 1 for proper rounding when dividing by 256
5. **Debug decode**: Supports 1/2/4 byte IDs at configurable offset; maps to name array
6. **Two debug formats**: Verbose (≤16 entries) shows command names + decoded IDs; compact (>16) shows only PacketID/Count/ACK
7. **Windows-only debug**: Guarded by `WWLIB32_H` (Mono_* functions)
8. **Packet header**: Assumes `CommHeaderType` at buffer start (MagicNumber, Code, PacketID)
9. **ACK flow**: Send entries cleared on ACK receive; Receive entries marked IsRead when processed, IsACK when ACK sent
10. **No dynamic resize**: Capacities fixed at construction

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/COMBUF.CPP at f1f0d42
