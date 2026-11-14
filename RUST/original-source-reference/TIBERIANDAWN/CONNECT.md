# PATH/FILE
original/TIBERIANDAWN/CONNECT.CPP

## Purpose
Base connection class for Tiberian Dawn networking. Provides packet framing (magic number, header), retry/timeout logic, and command name lookup. Used by IPX, modem, and null-modem connections as a common base.

## Where it fits in the game
Network transport layer - Abstract base for `IPXConnectionClass`, `NullModemClass`, etc. Handles packet serialization, ACK/retry logic, and command identification.

## Classes, structs, enums and important data

### ConnectionClass
**Connection base class** - Manages packet staging buffer and retry parameters.

**Key data members:**
- `MaxPacketLen` (int) - App packet max + `sizeof(CommHeaderType)`
- `MagicNum` (unsigned short) - Packet magic number for validation
- `RetryDelta` (unsigned long) - Min time between retries
- `MaxRetries` (unsigned long) - Max retry count (-1 = infinite)
- `Timeout` (unsigned long) - Max time before giving up (-1 = infinite)
- `PacketBuf` (char[MaxPacketLen]) - Staging buffer for packet assembly

### CommHeaderType (from CONNECT.H)
```cpp
struct CommHeaderType {
    unsigned short MagicNumber;  // Must match MagicNum
    unsigned char Code;          // Packet command (PACKET_ADATA, etc.)
    unsigned long PacketID;      // Sequence number
};
```

### Packet commands (PACKET_COUNT = 3)
| Command | Value | Name | Purpose |
|---------|-------|------|---------|
| `PACKET_ADATA` | 0 | "ADATA" | Application data |
| `PACKET_NDATA` | 1 | "NDATA" | Network data |
| `PACKET_ACK` | 2 | "ACK" | Acknowledgment |

### ConnectionClass::Commands
Static array mapping command enum to string names for debug.

## Functions and methods

### ConnectionClass(maxlen, magicnum, retry_delta, max_retries, timeout)
**Constructor** - Initializes parameters, allocates `PacketBuf`.

1. `MaxPacketLen = maxlen + sizeof(CommHeaderType)`
2. `MagicNum = magicnum`
3. `RetryDelta = retry_delta`
4. `MaxRetries = max_retries` (-1 = infinite)
5. `Timeout = timeout` (-1 = infinite)
6. `PacketBuf = new char[MaxPacketLen]`

### ~ConnectionClass()
**Destructor** - Frees `PacketBuf`.

### Service() → int (1=OK, 0=broken)
**Main polling routine** - Calls send/receive queue service.

```cpp
return Service_Send_Queue() && Service_Receive_Queue();
```
(Actual queue service methods not shown in this file - in derived classes)

### Time() → unsigned long
**Current time in game ticks** (15 FPS base).

- **Windows** (`WWLIB32_H`): `TickCount.Time()`
- **DOS**: `ftime()` → milliseconds → `(msec / 100) * 6` (converts to ~15Hz ticks)

### Command_Name(command) → char*
**Command enum to string** for debug display.

1. If `0 <= command < PACKET_COUNT` → `Commands[command]`
2. Else → NULL

## Inputs, outputs and side effects

**State mutations:**
- Packet buffer content during assembly
- Derived class queue states (via Service)

**External systems affected:**
- Network transport (IPX, modem, serial)
- Debug visualization (COMBUF/COMQUEUE)

## Calls into / called by

**Called by:**
- Derived connection classes (IPX, NullModem)
- Network manager (Service polling)
- Debug systems (Command_Name)

**Calls into:**
- `TickCount.Time()` or `ftime()`
- Derived `Service_Send_Queue`/`Service_Receive_Queue`

## Related original files
- **CONNECT.H** - Class declaration, CommHeaderType, PACKET_* enums
- **COMBUF.H/.CPP** - Uses Command_Name for debug
- **COMQUEUE.H/.CPP** - Uses Command_Name for debug
- **IPX.H/.CPP** - IPXConnectionClass derives from this
- **MODEM.H/.CPP** - NullModemClass derives from this
- **TIMER.H** - TickCount

## Rust port mapping
**Crates:** `cnc-net`, `cnc-core`

**Mapped modules:**
- `cnc-net/src/connection.rs` - ConnectionClass base
- `cnc-net/src/packet.rs` - CommHeaderType, packet commands
- `cnc-net/src/ipx/connection.rs` - IPXConnectionClass
- `cnc-net/src/modem/connection.rs` - NullModemClass

**Not yet ported:**
- Retry/timeout logic with infinite (-1) handling
- Packet staging buffer management
- Time base conversion (15Hz ticks)
- Magic number validation
- Virtual Service() with derived queue servicing

## Edge cases and compatibility notes

1. **Abstract base**: Pure virtual `Service_Send_Queue`/`Service_Receive_Queue` in derived classes
2. **Magic number**: Unique per connection type (e.g., `0x1234` for NullModem)
3. **Retry logic**: `RetryDelta` = min interval; `MaxRetries` = count; `Timeout` = absolute deadline
4. **Infinite retry**: -1 for `MaxRetries` or `Timeout` disables that limit
5. **Time base**: DOS `ftime()` converts to 15Hz game ticks: `(ms/100)*6`
6. **PacketBuf**: Single staging buffer, not per-packet (reused)
7. **Command names**: Only 3 commands in base; derived may extend
8. **Demo build**: Some network code compiled out (`#ifndef DEMO`)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/CONNECT.CPP at f1f0d42
