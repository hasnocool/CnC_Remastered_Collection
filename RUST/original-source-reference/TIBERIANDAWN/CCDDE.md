# PATH/FILE
original/TIBERIANDAWN/CCDDE.CPP

## Purpose
DDE (Dynamic Data Exchange) server interface for Tiberian Dawn. Handles communication with WChat (Westwood Chat) for multiplayer game launching, heartbeats, and game info exchange. Windows-only implementation using DDE protocol.

## Where it fits in the game
Multiplayer infrastructure - Windows-specific DDE client for WChat integration. Used by `STARTUP.CPP` and `INIT.CPP` for WChat spawning and game info exchange. Global `DDEServer` instance.

## Classes, structs, enums and important data

### DDEServerClass
**DDE server wrapper** - Manages DDE connection to WChat.

**Key data members:**
| Field | Purpose |
|-------|---------|
| `DDE_Class` (Instance_Class*) | DDE instance wrapper |
| `MPlayerGameInfo` (char*) | Multiplayer game setup from WChat |
| `MPlayerGameInfoLength` (int) | Length of game info |
| `LastHeartbeat` (unsigned long) | Last heartbeat timestamp |
| `IsEnabled` (bool) | DDE callback enabled |
| `CC95AlreadyRunning` (static bool) | Another instance detected |

### Instance_Class (external)
**DDE instance wrapper** - Encapsulates Windows DDE API.

**Key methods:**
- `Enable_Callback(bool)` - Enable/disable DDE callback
- `Test_Server_Running(name)` - Check if DDE server running
- `Register_Server(callback)` - Register callback
- `Open_Poke_Connection(name)` - Open poke connection
- `Poke_Server(data, length)` - Send poke
- `Close_Poke_Connection()` - Close connection

### Packet types
| Constant | Value | Purpose |
|----------|-------|---------|
| `DDE_ADVISE_CONNECT` | <0 | Client connect advisory |
| `DDE_ADVISE_DISCONNECT` | <0 | Client disconnect advisory |
| `DDE_PACKET_START_MPLAYER_GAME` | >0 | C&CSPAWN.INI from WChat |
| `DDE_TICKLE` | >0 | Window focus request |
| `DDE_PACKET_HEART_BEAT` | >0 | Heartbeat from WChat |

### Packet format (network byte order)
```cpp
struct DDEPacket {
    int length;      // Total length including header
    int type;        // Packet type
    char data[];     // Payload
};
```

## Functions and methods

### DDE_Callback(data, length) → BOOL
**Windows DDE callback entry point** - Wrapper for `DDEServerClass::Callback()`.

### DDEServerClass::DDEServerClass()
**Constructor:**
1. Creates `Instance_Class("CONQUER", "WCHAT")`
2. Enables callback
3. Checks if WChat already running → `CC95AlreadyRunning`
4. (Disabled) `Register_Server(DDE_Callback)`

### ~DDEServerClass()
**Destructor:**
1. `Delete_MPlayer_Game_Info()`
2. `delete DDE_Class`

### Enable() / Disable()
**Callback toggle:**
- `Enable()`: `DDE_Class->Enable_Callback(TRUE)`
- `Disable()`: `DDE_Class->Enable_Callback(FALSE)`

### Callback(data, length) → BOOL
**Main DDE callback handler:**

**Advisory packets (length < 0):**
| Code | Action |
|------|--------|
| `DDE_ADVISE_CONNECT` | Log "client connect" |
| `DDE_ADVISE_DISCONNECT` | Log "client disconnect" |

**Data packets (length ≥ 8 bytes):**
1. Parses length (ntohl) and type (ntohl)
2. Strips 8-byte header
3. **Packet handling:**
   - `DDE_PACKET_START_MPLAYER_GAME`: Stores game info in `MPlayerGameInfo`, resets heartbeat
   - `DDE_TICKLE`: (Disabled) Window focus
   - `DDE_PACKET_HEART_BEAT`: Updates `LastHeartbeat` from `GameTimer.Time()`
   - Default: Log "unrecognised packet"

### Get_MPlayer_Game_Info() → char*
Returns `MPlayerGameInfo` (C&CSPAWN.INI content from WChat).

### Delete_MPlayer_Game_Info()
Frees `MPlayerGameInfo` buffer, sets to NULL.

### Time_Since_Heartbeat() → int
Returns `GameTimer.Time() - LastHeartbeat` (ticks since last heartbeat).

### Send_Data_To_DDE_Server(data, length, packet_type) → BOOL
**Sends packet to WChat via DDE poke:**

1. Opens poke connection to "WCHAT"
2. Allocates buffer: `length + 8` bytes
2. Writes header (length+8, packet_type) in network order
3. Copies payload
4. `Poke_Server()` via DDE
5. Closes connection
6. Returns TRUE/FALSE

## DDE Protocol Details

### Connection
- Service: "CONQUER"
- Topic: "WCHAT"
- Uses Windows DDE API via `Instance_Class` wrapper

### Packet encoding
- All integers: network byte order (big-endian) via `htonl`/`ntohl`
- Header: 8 bytes (length + type)
- Payload: Raw data (INI text for game info)

### Heartbeat
- WChat sends `DDE_PACKET_HEART_BEAT` periodically
- `LastHeartbeat` updated from `GameTimer.Time()`
- `Time_Since_Heartbeat()` used for timeout detection

## Inputs, outputs and side effects

**State mutations:**
- `MPlayerGameInfo` buffer (allocated/freed)
- `LastHeartbeat` timestamp
- `IsEnabled` flag
- `CC95AlreadyRunning` flag

**External systems affected:**
- WChat communication
- Multiplayer game setup (`INIT.CPP`, `MPLAYER.CPP`)
- Single-instance detection (`CC95AlreadyRunning`)

## Calls into / called by

**Called by:**
- `STARTUP.CPP` → `DDEServer.Enable/Disable()`
- `INIT.CPP` → `Get_MPlayer_Game_Info()`, `Delete_MPlayer_Game_Info()`
- Windows DDE system → `DDE_Callback()`
- `MPLAYER.CPP` → `Get_MPlayer_Game_Info()`

**Calls into:**
- Windows DDE API (`Instance_Class`)
- `GameTimer.Time()`
- `CCDebugString()` logging
- `Send_Data_To_DDE_Server()` for outgoing packets

## Related original files
- **CCDDE.H** - Class declarations
- **INSTANCE.H/.CPP** - Instance_Class wrapper
- **STARTUP.CPP** - `DDEServer.Enable/Disable()`, `CC95AlreadyRunning`
- **INIT.CPP** → `Get_MPlayer_Game_Info()`, `Delete_MPlayer_Game_Info()`
- **MPLAYER.CPP** → `Read_MultiPlayer_Settings()` uses game info
- **TIMER.H** - `GameTimer`, `CountDownTimerClass`

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/ipc/dde.rs` - DDEServerClass
- `cnc-tiberian-dawn/src/ipc/wchat.rs` - WChat protocol

**Not yet ported:**
- Windows DDE API integration (`Instance_Class`)
- DDE callback registration
- Network byte order handling
- Heartbeat monitoring
- INI buffer management
- Single-instance detection via DDE

## Edge cases and compatibility notes

1. **Windows-only**: Entire file guarded by Windows DDE API usage
2. **Packet validation**: Minimum 8 bytes (2 ints) for data packets
3. **Advisory packets**: Negative length indicates advisory
3. **Heartbeat timeout**: `Time_Since_Heartbeat()` for connection health
4. **Game info lifetime**: `MPlayerGameInfo` freed on new game/startup
5. **Single instance**: `CC95AlreadyRunning` prevents multiple C&C instances
5. **DDE vs TCP/IP**: Legacy DDE for WChat; modern would use TCP/WebSocket
6. **Endianess**: All packet integers use network byte order
7. **Callback threading**: DDE callback may fire on different thread
8. **Disabled features**: `Register_Server`, window focus (`DDE_TICKLE`) commented out

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/CCDDE.CPP at f1f0d42
