# PATH/FILE
original/TIBERIANDAWN/IPX.CPP

## Purpose
IPX/SPX network transport layer for Tiberian Dawn (DOS/Win95 era). Provides low-level IPX socket management, packet send/receive via ECBs (Event Control Blocks), address resolution, and Novell NetWare integration. Compiled out for Windows 95 (`#ifdef NOT_FOR_WIN95`).

## Where it fits in the game
Network transport - Used by `IPXManagerClass` and `IPXGlobalConnClass` for LAN multiplayer. Implements raw IPX calls via DPMI real-mode interrupts (Int 0x2F for IPX, Int 0x21 for NetWare).

## Classes, structs, enums and important data

### IPX detection return codes
| Return | Meaning |
|--------|---------|
| 0 | Not installed |
| 1 | IPX only |
| 2 | IPX + SPX |

### Socket numbers
- Custom range: `0x4000` - `0x8000`
- Game uses `VIRGIN_SOCKET` (from IPX.H)

### ECB (Event Control Block) structure
```cpp
struct ECB {
    unsigned char Link[4];
    unsigned char ESR[4];        // Event Service Routine
    unsigned char InUse;
    unsigned char CompletionCode;
    unsigned short SocketNumber;
    IPXHeaderType Header;
    unsigned char Data[546];
    unsigned char ImmediateAddress[6];
    // Packet fragments...
};
```

### IPXHeaderType
```cpp
struct IPXHeaderType {
    unsigned short Checksum;
    unsigned short Length;
    unsigned char TransportControl;
    unsigned char PacketType;        // 4 = IPX
    unsigned char DestNetwork[4];
    unsigned char DestNode[6];
    unsigned short DestSocket;
    unsigned char SrcNetwork[4];
    unsigned char SrcNode[6];
    unsigned short SrcSocket;
};
```

### Request/Reply buffers (Int 0x21, AH=0xE3)
Various structs for connection lookup, address translation, user ID.

## Functions and methods

### IPX_SPX_Installed() → int
**Detects IPX/SPX presence.**

**Win95 path:** `IPX_Initialise()` (from IPX95)
**DOS path:** DPMI real-mode interrupt:
1. Int 0x2F, AH=0x7A, AL=0 → AL=0xFF if IPX present
2. If IPX: Call IPX function BX=0x10 → AL=0xFF if SPX present

### IPX_Open_Socket(socket) → int
**Opens IPX socket** (long-lived, AL=0xFF).

| Return | Meaning |
|--------|---------|
| 0 | OK |
| -1 | IPX not installed |
| 0xFE | Socket table full |
| 0xFF | Socket already open |

### IPX_Close_Socket(socket) → int
**Closes socket** - Returns 0 on success.

### IPX_Get_Connection_Number() → int
**Gets local Novell connection number** (0 if not logged in).
- Int 0x21, AH=0xDC → AL = connection number

### IPX_Get_1st_Connection_Num(username) → int
**Resolves username to first connection number.**
- Allocates DOS memory for request/reply buffers
- Int 0x21, AH=0xE3, buffer type 0x15
- Returns first connection_num from reply

### IPX_Get_Internet_Address(conn_num, network, node) → int
**Translates connection number to network/node address.**
- Int 0x21, AH=0xE3, buffer type 0x13
- Fills 4-byte network, 6-byte node

### IPX_Get_User_ID(conn_num, user_id) → int
**Gets username from connection number.**
- Int 0x21, AH=0xE3, buffer type 0x16
- Copies 48-char name to caller buffer

### IPX_Listen_For_Packet(ecb) → int
**Posts receive ECB** - Caller must fill ECB:
- `SocketNumber` = opened socket
- `ESR` = optional callback
- `PacketCount` = 2 (header + data)
- `Packet[0]` = IPXHeaderType
- `Packet[1]` = data buffer
- ECB must be in real-mode memory

### IPX_Send_Packet(ecb)
**Sends packet** - Caller must fill ECB:
- `SocketNumber` = opened socket
- `PacketCount` = 2
- `Packet[0]` = IPXHeaderType (filled)
- `Packet[1]` = data buffer
- `ImmediateAddress` = from `IPX_Get_Local_Target`
- Header: PacketType=4, DestNetwork/Node/Socket filled
- ECB + header + data in real-mode memory
- **No return** - Check `ECB.CompletionCode` later

### IPX_Get_Local_Target(dest_network, dest_node, dest_socket, bridge) → int
**Resolves route to destination** - Fills `bridge` with immediate address (bridge or direct).
- Allocates DOS buffers
- Int 0x21, AH=0xE3, IPX_GET_LOCAL_TARGET
- Requires valid connection number (non-zero)

### IPX_Cancel_Event(ecb) → int
**Cancels pending ECB operation.**

### Let_IPX_Breath()
**Yields to IPX** - Call while polling `ECB.InUse`.
- IPX_RELINQUISH_CONTROL (Int 0x7A, BX=0x1A)

## Win95 vs DOS implementation

| Function | Win95 (`#ifndef NOT_FOR_WIN95`) | DOS (`#ifdef NOT_FOR_WIN95`) |
|----------|--------------------------------|------------------------------|
| `IPX_SPX_Installed` | `IPX_Initialise()` | DPMI Int 0x2F/Int 0x21 |
| Socket ops | `IPX95.cpp` | DPMI Int 0x7A |
| Address lookup | `IPX95.cpp` | Int 0x21, AH=0xE3 |
| Send/Listen | `IPX95.cpp` | DPMI Int 0x7A, ECB in real mem |

## Inputs, outputs and side effects

**State mutations:**
- IPX socket table (open/close)
- ECB completion codes (async)
- Real-mode memory (DOS buffers, ECBs)

**External systems affected:**
- Multiplayer game state
- Network sync (deterministic RNG seed)
- Lobby (Games/Players lists)

## Calls into / called by

**Called by:**
- `IPXManagerClass` / `IPXGlobalConnClass` / `IPXPrivateConnClass`
- `MPLAYER.CPP` lobby functions
- `INIT.CPP` startup detection

**Calls into:**
- DPMI (`int386x`, `DPMI_CALL_REAL_INT`, `DPMI_ALLOC_DOS_MEM`)
- Real-mode interrupts (0x2F, 0x21, 0x7A)
- `FP_SEG`/`FP_OFF` for far pointers

## Related original files
- **IPX.H** - ECB, IPXHeaderType, function declarations
- **IPX95.H/.CPP** - Win95 implementation
- **IPXMGR.H/.CPP** - IPXManagerClass
- **IPXCONN.H/.CPP** - Connection classes
- **MPLAYER.H/.CPP** - Lobby, Games/Players lists
- **CONNECT.H/.CPP** - ConnectionClass base
- **COMBUF.H/.CPP** - Packet queues

## Rust port mapping
**Crates:** `cnc-net` (legacy reference only - IPX not used in modern remaster)

**Not ported:** IPX is legacy LAN protocol. Modern remaster uses TCP/IP (Winsock) or relays.

## Edge cases and compatibility notes

1. **Real-mode memory requirement**: All ECBs, headers, buffers must be in DOS-accessible memory (`DPMI_ALLOC_DOS_MEM`, segment:offset)
2. **Register saving**: IPX trashes registers - all functions save/restore via DPMI
3. **Socket range**: Custom sockets `0x4000-0x8000`
4. **Long-lived sockets**: `AL=0xFF` on open
5. **Connection number 0**: Not logged into NetWare - address lookup fails
6. **Bridge routing**: `IPX_Get_Local_Target` handles bridge vs direct
7. **Async completion**: `IPX_Send_Packet` returns void; poll `ECB.CompletionCode`
8. **Win95 path**: Uses `IPX95.cpp` (Winsock/IPX wrapper) instead of raw interrupts
9. **SPX detection**: Separate IPX function (BX=0x10)
10. **User ID buffer**: 48 chars max (Novell limit)

## Evidence and review status
Status: Reviewed (legacy/DOS implementation)
Source: original/TIBERIANDAWN/IPX.CPP at f1f0d42
