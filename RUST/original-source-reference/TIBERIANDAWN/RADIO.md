# PATH/FILE
original/TIBERIANDAWN/RADIO.CPP

## Purpose
Radio communication system for Tiberian Dawn. Manages unit-to-unit radio messages for coordination (transport loading/unload, construction, repair, attack orders, tethering, passenger management) with connection state tracking.

## Where it fits in the game
Unit coordination subsystem - `RadioClass` is a base class (inherited by `MissionClass`). Used by units, aircraft, buildings for inter-object communication without direct references. Each radio-equipped object has a `RadioClass` member.

## Classes, structs, enums and important data

### RadioClass
**Radio communication endpoint** - Tracks connection state and message handling.

**Key data members:**
- `Radio` (RadioClass*) - Current connection target (NULL = idle)
- `Strength` (int) - Signal strength (0 = no connection)
- `LastMessage` (RadioMessageType) - Last received message (for debug)

### RadioMessageType enum (RADIO_COUNT = 31)
| Message | Value | Purpose |
|---------|-------|---------|
| `RADIO_HISS` | 0 | Static/noise |
| `RADIO_ROGER` | 1 | Acknowledgment |
| `RADIO_COME_IN` | 2 | Hail |
| `RADIO_OVER_OUT` | 3 | End conversation |
| `RADIO_REQUEST_TRANSPORT` | 4 | Need transport |
| `RADIO_ATTACH_TRANSPORT` | 5 | Attaching to transport |
| `RADIO_DELIVERY` | 6 | Cargo delivery |
| `RADIO_LOAD_UNLOAD` | 7 | Load/unload maneuver |
| `RADIO_CLEAR` | 8 | Clear to proceed |
| `RADIO_UNLOAD_CLEAR` | 9 | Unload complete, leaving |
| `RADIO_UNABLE_COMPLY` | 10 | Negative |
| `RADIO_CONSTRUCTION_START` | 11 | Building starting |
| `RADIO_CONSTRUCTION_DONE` | 12 | Building complete |
| `RADIO_BUMPED` | 13 | Collision redraw |
| `RADIO_LOADING` | 14 | Attempting to load |
| `RADIO_MAY_I_RIDE` | 15 | Request passenger |
| `RADIO_READY_RECEIVE` | 16 | Ready for cargo |
| `RADIO_WANT_PASSENGER` | 17 | Accept passenger |
| `RADIO_MOVE_TO` | 18 | Move order |
| `RADIO_NEED_MOVE` | 19 | Query move |
| `RADIO_ALL_RIGHT` | 20 | Acknowledged |
| `RADIO_PASSENGER_NOW` | 21 | Now passenger |
| `RADIO_BACKUP_REFINERY` | 22 | Harvester dock |
| `RADIO_RUN_AWAY` | 23 | Retreat |
| `RADIO_TETHER_ESTABLISHED` | 24 | Tether connected |
| `RADIO_TETHER_BROKEN` | 25 | Tether lost |
| `RADIO_REPAIR_STEP` | 26 | Repair progress |
| `RADIO_PREPARED_FIGHT` | 27 | Combat ready |
| `RADIO_ATTACK_TARGET` | 28 | Attack order |
| `RADIO_RELOAD_STEP` | 29 | Reload progress |
| `RADIO_KICK` | 30 | Melee kick |
| `RADIO_PUNCH` | 31 | Melee punch |
| `RADIO_FISTICUFFS` | 32 | Melee challenge |

### Messages array (debug display)
```cpp
char const * Messages[RADIO_COUNT] = {
    "hisssss", "Roger.", "Come in.", "Over and out.",
    "Requesting transport.", "Attach to transport.",
    "I've got a delivery for you.", "I'm performing load/unload maneuver. Be careful.",
    "I'm clear.", "You are clear to unload. Driving away now.",
    "Am unable to comply.", "I'm starting construction now... act busy.",
    "I've finished construction. You are free.", "We bumped, redraw yourself please.",
    "I'm trying to load up now.", "May I become a passenger?",
    "Are you ready to receive shipment?", "Are you trying to become a passenger?",
    "Move to location X.", "Do you need to move?",
    "All right already. Now what?", "I'm a passenger now.",
    "Backup into refinery now.", "Run away!",
    "Tether established.", "Tether broken.",
    "Repair one step.", "Are you prepared to fight?",
    "Attack this target please.", "Reload one step.",
    "Take this kick! You... You...", "Take this punch! You... You...",
    "Fancy a little fisticuffs, eh?"
};
```

## Functions and methods

### Receive_Message(from, message, param) → RadioMessageType
**Handles incoming message** - Base implementation for radio protocol.

1. `LastMessage = message` (debug tracking)
2. **Over/Out from connected peer**: If `from == Radio && message == RADIO_OVER_OUT`:
   - Calls `MissionClass::Receive_Message`
   - `Radio_Off()` - Clears connection
   - Returns `RADIO_ROGER`
3. **Hello (connection request)**: If `message == RADIO_HELLO && Strength`:
   - If `Radio == from || !Radio` (idle or same peer):
     - `Radio = from` (establish connection)
     - Returns `RADIO_ROGER`
   - Else: Returns `RADIO_NEGATIVE` (busy)
4. **Other messages**: Delegates to `MissionClass::Receive_Message`

### Transmit_Message(message, param, to) → RadioMessageType
**Sends message to target** - Handles connection establishment/teardown.

1. If `!to` → `to = Contact_With_Whom()` (current connection)
2. If `!to` → Returns `RADIO_STATIC` (no target)
3. **Over/Out to connected peer**: If `to == Radio && message == RADIO_OVER_OUT`:
   - `Radio = 0` (clear locally)
4. **Hello (initiate connection)**: If `message == RADIO_HELLO`:
   - Sends `RADIO_OVER_OUT` first (cleanup any stale)
   - Calls `to->Receive_Message(this, RADIO_HELLO, param)`
   - If response == `RADIO_ROGER`: `Radio = to`, returns `RADIO_ROGER`
   - Else: Returns `RADIO_NEGATIVE`
5. **Other messages**: Forwards to `to->Receive_Message(this, message, param)`

### Limbo() → bool
**Breaks radio on limbo** - Called when object enters limbo.

1. If `!IsInLimbo`: `Transmit_Message(RADIO_OVER_OUT)` (clean disconnect)
2. Calls `MissionClass::Limbo()`

### Transmit_Message(message, to) → RadioMessageType
**Convenience overload** - Uses global `LParam` as param.

### Debug_Dump(mono) (CHEAT_KEYS)
**Monochrome debug display** - Shows `Messages[LastMessage]`, connected target ID, calls `MissionClass::Debug_Dump`.

## Radio protocol flows

### Connection establishment
```
A: Transmit(RADIO_HELLO) → B.Receive(RADIO_HELLO)
B: If idle → Radio=A, return ROGER
A: Receives ROGER → Radio=B, connected
```

### Conversation
```
A: Transmit(RADIO_ATTACK_TARGET) → B.Receive(RADIO_ATTACK_TARGET)
B: MissionClass::Receive_Message → returns ROGER
A: Receives ROGER
```

### Teardown
```
A: Transmit(RADIO_OVER_OUT) → B.Receive(RADIO_OVER_OUT)
B: Radio_Off(), returns ROGER
A: Radio=0
```

### Limbo cleanup
```
Object.Limbo() → Transmit(RADIO_OVER_OUT) to peer
Peer receives → Radio_Off()
```

## Inputs, outputs and side effects

**State mutations:**
- `Radio` pointer (connection state)
- `Strength` (signal)
- `LastMessage` (debug)

**External systems affected:**
- Unit coordination (transport, construction, repair, attack)
- Mission/behavior system (`MissionClass`)
- Debug monitor

## Calls into / called by

**Called by:**
- Unit AI: `UnitClass` methods for transport, repair, attack
- Building AI: Construction start/complete
- Harvester: Refinery docking
- Aircraft: Passenger loading
- Limbo system: `RadioClass::Limbo()`

**Calls into:**
- `MissionClass::Receive_Message` (derived behavior)
- `Radio_Off()` (connection clear)
- `Contact_With_Whom()` (current target)
- `MonoClass` (debug)

## Related original files
- **RADIO.H** - Class declaration, RadioMessageType enum
- **MISSION.H/.CPP** - MissionClass (derived radio behavior)
- **UNIT.H/.CPP** - UnitClass uses radio
- **AIRCRAFT.H/.CPP** - AircraftClass uses radio
- **BUILDING.H/.CPP** - BuildingClass uses radio
- **TRANSPORT.H/.CPP** - Transport logic
- **MONO.H** - Debug display

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/ai/radio.rs` - RadioClass, protocol
- `cnc-tiberian-dawn/src/ai/td_radio.rs` - TD message types
- `cnc-core/src/ai/mission.rs` - MissionClass (derived radio)

**Not yet ported:**
- Radio message enum with 31 variants
- Connection state machine (idle/connected/busy)
- Hello/Over-Out handshake
- Strength signal tracking
- Limbo auto-disconnect
- Param passing for extended data
- Debug message strings
- Derived `MissionClass` message handling

## Edge cases and compatibility notes

1. **Base class only**: `RadioClass` provides carrier; `MissionClass` implements behavior
2. **Single connection**: One `Radio` pointer at a time (no conference calls)
3. **Hello returns NEGATIVE if busy**: Prevents connection stealing
4. **Over/Out symmetry**: Both sides clear on `RADIO_OVER_OUT`
5. **Limbo sends Over/Out**: Ensures peer knows connection dropped
6. **Strength gate**: `RADIO_HELLO` only works if `Strength > 0`
7. **Static return**: `RADIO_STATIC` when no target available
8. **Param reference**: `long& param` for bidirectional data (e.g., target coordinates)
9. **Debug strings**: Human-readable for mono monitor
10. **Inheritance**: `MissionClass` extends for type-specific responses

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/RADIO.CPP at f1f0d42
