# PATH/FILE
original/TIBERIANDAWN/FIELD.CPP

## Purpose
Field class implementation for network packet serialization in Tiberian Dawn. Provides generic typed data fields with automatic host-to-network byte order conversion. Used by `CCFILE.CPP`/`CDFILE.CPP` connection files for multiplayer network packet field encoding/decoding. Windows-only implementation with `htons`/`htonl`/`ntohs`/`ntohl` byte swapping.

## Where it fits in the game
Network infrastructure - `FieldClass` instances are used in `CCFILE.CPP` for sending/receiving typed data over network connections. Each field has an ID string, typed data (char, short, long, string, chunk), and automatic byte-order conversion. Part of the Westwood Auto Registration App project (started 04/22/96).

## Classes, structs, enums and important data

### DataType enum
| Constant | Purpose |
|----------|---------|
| `TYPE_CHAR` | Signed 8-bit char |
| `TYPE_UNSIGNED_CHAR` | Unsigned 8-bit |
| `TYPE_SHORT` | Signed 16-bit |
| `TYPE_UNSIGNED_SHORT` | Unsigned 16-bit |
| `TYPE_LONG` | Signed 32-bit |
| `TYPE_UNSIGNED_LONG` | Unsigned 32-bit |
| `TYPE_STRING` | Null-terminated string |
| `TYPE_CHUNK` | Raw binary chunk |

### FieldClass
**Typed network data field** - One field in a network packet.

**Key data members:**
| Field | Type | Purpose |
|-------|------|---------|
| `ID` (char[]) | Field name identifier |
| `DataType` (DataType) | Type of data stored |
| `Size` (unsigned short) | Size in bytes |
| `Data` (char*) | Heap-allocated data buffer |
| `Next` (FieldClass*) | Linked list next pointer |

### FieldClass constructors (8 overloads)
| Constructor | Parameters | DataType |
|-------------|------------|----------|
| `FieldClass(char*, char)` | id, char data | TYPE_CHAR |
| `FieldClass(char*, unsigned char)` | id, uchar data | TYPE_UNSIGNED_CHAR |
| `FieldClass(char*, short)` | id, short data | TYPE_SHORT |
| `FieldClass(char*, unsigned short)` | id, ushort data | TYPE_UNSIGNED_SHORT |
| `FieldClass(char*, long)` | id, long data | TYPE_LONG |
| `FieldClass(char*, unsigned long)` | id, ulong data | TYPE_UNSIGNED_LONG |
| `FieldClass(char*, char*)` | id, string data | TYPE_STRING |
| `FieldClass(char*, void*, int)` | id, chunk ptr, length | TYPE_CHUNK |

All constructors:
1. `strncpy(ID, id, sizeof(ID))` - copy field name
2. Set `DataType` appropriately
3. `Size = sizeof(data)` (or strlen+1 for strings, length for chunks)
4. `Data = new char[Size]` - heap allocate
5. `memcpy(Data, &data/data, Size)` - copy data
6. `Next = NULL`

### FieldClass::Host_To_Net() → void
**Convert field to network byte order:**
1. For `TYPE_SHORT/UNSIGNED_SHORT`: `htons()` the data
2. For `TYPE_LONG/UNSIGNED_LONG`: `htonl()` the data
3. For `TYPE_CHAR/UNSIGNED_CHAR/TYPE_STRING/TYPE_CHUNK`: no conversion needed
4. `DataType = htons(DataType)` - convert type enum
5. `Size = htons(Size)` - convert size field

### FieldClass::Net_To_Host() → void
**Convert field from network byte order:**
1. `Size = ntohs(Size)` - convert size first
2. `DataType = ntohs(DataType)` - convert type enum
3. For `TYPE_SHORT/UNSIGNED_SHORT`: `ntohs()` the data
4. For `TYPE_LONG/UNSIGNED_LONG`: `ntohl()` the data
5. For `TYPE_CHAR/UNSIGNED_CHAR/TYPE_STRING/TYPE_CHUNK`: no conversion needed

**Important:** Size must be converted before DataType because the switch statement uses DataType to determine how to convert the data.

## Functions and methods

### FieldClass constructors (8 overloads)
Standard constructors for each supported data type. All allocate `Data` on heap and copy the provided value.

### FieldClass::Host_To_Net()
Convert host-ordered field to network byte order for transmission.

### FieldClass::Net_To_Host()
Convert network-ordered field back to host byte order after reception.

## Inputs, outputs and side effects

**State mutations:**
- `DataType` field modified by Host_To_Net()/Net_To_Host()
- `Size` field modified by Host_To_Net()/Net_To_Host()
- `Data` buffer contents modified (byte-swapped) by Host_To_Net()/Net_To_Host()
- `Data` allocated on heap in constructor, freed elsewhere

**External systems affected:**
- `CCFILE.CPP` - connection file uses FieldClass for packet fields
- `CDFILE.CPP` - connection data file uses FieldClass
- Network multiplayer synchronization

## Calls into / called by

**Called by:**
- `CCFILE.CPP` - FieldClass objects created for network packets
- `CDFILE.CPP` - FieldClass objects for data exchange

**Calls into:**
- `strncpy()` - ID string copy
- `new[]` / `memcpy()` - data allocation and copy
- `htons()`/`htonl()` - host-to-network conversion
- `ntohs()`/`ntohl()` - network-to-host conversion

## Related original files
- **field.h** - Class declarations
- **CCFILE.CPP** → uses FieldClass for network packet fields
- **CDFILE.CPP** → uses FieldClass for connection data
- **TCPIP.H** - TCP/IP network headers

## Rust port mapping
**Crates:** `cnc-tiberian-dawn` (Windows-only)

**Mapped modules:**
- `cnc-tiberian-dawn/src/network/field.rs` - FieldClass equivalent
- `cnc-tiberian-dawn/src/network/packet.rs` - Packet serialization

**Not yet ported:**
- Windows-specific `#pragma warning(disable:4996)`
- `htons`/`htonl`/`ntohs`/`ntohl` byte swapping
- Heap-allocated `Data` buffer management
- Linked list `Next` pointer chain
- `strncpy` for ID field
- Network packet field serialization

## Edge cases and compatibility notes

1. **Host_To_Net order**: Size must NOT be converted before DataType conversion in Net_To_Host (Size converted first because switch depends on DataType)
2. **DataType conversion**: DataType itself is also byte-swapped (htons/ntohs)
3. **Heap allocation**: All constructors `new char[Size]` - caller must `delete[]` elsewhere
4. **TYPE_CHUNK**: Raw binary data, length specified explicitly
5. **TYPE_STRING**: Includes null terminator in Size (strlen+1)
6. **Char types not swapped**: TYPE_CHAR/UNSIGNED_CHAR are single byte, no conversion needed
7. **STRING/CHUNK not swapped**: Raw data copied as-is
8. **`#pragma warning(disable:4996)`**: Disables strncpy deprecation warning

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/FIELD.CPP at f1f0d42
