# PATH/FILE
original/TIBERIANDAWN/BUILDING.CPP

## Purpose
Building logic for Tiberian Dawn. Implements all structure behaviors: construction, production, power, repair, capture, superweapons (SAM, missile), refinery harvesting, weapon factory unloading, and base management.

## Where it fits in the game
Core structure system - Every structure on the map is a `BuildingClass` instance. Global `Buildings` pool (GLOBALS.CPP). Integrates with factory system, power grid, radar, sidebar, and AI.

## Classes, structs, enums and important data

### BuildingClass : TechnoClass
**Core structure class** - All map structures.

**Key data members:**
| Field | Purpose |
|-------|---------|
| `Class` (BuildingTypeClass*) | Type definition |
| `BState` (BStateType) | Animation state |
| `Factory` (FactoryClass*) | Production queue |
| `Power` | Power output |
| `IsReadyToCommence` | Docking ready flag |
| `WhomToRepay` (TARGET) | Repair payment target |
| `CenterOffset[BSIZE_COUNT]` | Size-based center offsets |

### BStateType enum
| State | Description |
|-------|-------------|
| `BSTATE_IDLE` | Normal operation |
| `BSTATE_CONSTRUCTION` | Building animating |
| `BSTATE_ACTIVE` | Active (refinery full) |
| `BSTATE_FULL` | Refinery ready |
| `BSTATE_IDLE` | Idle/guard |

### SAMState enum (SAM site)
| State | Description |
|-------|-------------|
| `SAM_UNDERGROUND` | Hidden |
| `SAM_RISING` | Opening |
| `SAM_READY` | Tracking |
| `SAM_FIRING` | Launching |
| `SAM_READY2` | Tracking (2nd) |
| `SAM_FIRING2` | Firing 2nd |
| `SAM_LOCKING` | Locking to lower |
| `SAM_LOWERING` | Retracting |

### CenterOffset table
| Size | Offset |
|------|--------|
| 1×1 | 0x00800080 |
| 2×2 | 0x00FF00FF |
| 3×3 | 0x01800180 |
| ... | ... |

## Functions and methods

### BuildingClass::AI()
**Per-frame building logic** - Called from `LogicClass::AI()`.

**Dispatches to mission handlers:**
| Mission | Handler |
|---------|---------|
| `MISSION_GUARD` | `Mission_Guard()` |
| `MISSION_ATTACK` | `Mission_Attack()` |
| `MISSION_CONSTRUCTION` | `Mission_Construction()` |
| `MISSION_DECONSTRUCTION` | `Mission_Deconstruction()` |
| `MISSION_HARVEST` | `Mission_Harvest()` |
| `MISSION_REPAIR` | `Mission_Repair()` |
| `MISSION_MISSILE` | `Mission_Missile()` |
| `MISSION_UNLOAD` | `Mission_Unload()` |
| `MISSION_CAPTURE`/`SABOTAGE` | `Mission_Capture()` |
| `MISSION_UNLOAD` | `Mission_Unload()` |
| `MISSION_REPAIR` | `Mission_Repair()` |

### BuildingClass::Receive_Message() → RadioMessageType
**Radio message handler** - Coordinates with units.

**Message handling:**
| Message | Behavior |
|---------|----------|
| `RADIO_HELLO` | Attach request; rejects if harvesting |
| `RADIO_CAN_LOAD` | Dock permission; checks type/state |
| `RADIO_IM_IN` | Docked; starts mission (repair/harvest) |
| `RADIO_DOCKING` | Maintenance; starts animations |
| `RADIO_ARE_REFINERY` | Harvester query |
| `RADIO_BUILDING` | Construction yard notify |
| `RADIO_COMPLETE` | Construction done |
| `RADIO_OVER_OUT` | Contact lost; idle mode |
| `RADIO_UNLOADED` | Unit left; cleanup |
| `RADIO_BUILDING` | Construction yard |
| `RADIO_COMPLETE` | Build done |
| `RADIO_OVER_OUT` | Contact lost |

### BuildingClass::Mission_Harvest() → int
**Refinery harvester logic** - State machine.

| Status | Action |
|--------|--------|
| `BSTATE_IDLE` | Wait for harvester |
| `BSTATE_FULL` | Harvester docking |
| `BSTATE_ACTIVE` | Unloading |
| `BSTATE_EMPTY` | Harvester leaving |

### BuildingClass::Mission_Missile() → int
**Nuclear missile silo** - 6-state machine.

| Status | Action |
|--------|--------|
| `INITIAL` | Doors opening |
| `ARMED` | Ready to fire |
| `FIRING` | Launch animation |
| `LAUNCHED` | Missile away |
| `RELOAD` | Doors closing |
| `COMPLETE` | Returns to idle |

### BuildingClass::Mission_Unload() → int
**Weapon factory unload** - 5-state door/unit sequence.

| Status | Action |
|--------|--------|
| `INITIAL` | Unit guard |
| `CLEAR_BIB` | Scatter blockers |
| `OPEN` | Door open, unit exits |
| `LEAVE` | Unit drives out |
| `CLOSE` | Door closes, idle |

### BuildingClass::Mission_Construction() / Mission_Deconstruction()
**Construction yard** - Animates build/sell.

### BuildingClass::Mission_Repair()
**Repair bay** - Repairs docked units.

### BuildingClass::Mission_Missile()
**SAM site** - 8-state machine (underground→rising→ready→firing→locking→lowering).

### BuildingClass::Power_Output() → int
**Power generation** - `Class->Power * (LastStrength/MaxStrength)` via fixed-point.

### BuildingClass::Refund_Amount() → int
**Sell value** - Base cost + stored tiberium (v1.07+).

### BuildingClass::Crew_Type() → InfantryType
**Survivor type** - Storage=silo→C1/C7; Const yard→E7 (rare).

### BuildingClass::Detach_All(all)
**Cleanup on destroy/capture** - Abandons factory, checks house factory dependency.

### BuildingClass::Flush_For_Placement(techno, cell) → bool
**Placement clearing** - Scatters allies, attacks enemies in footprint.

### BuildingClass::Can_Player_Move() → bool
**MCV deploy** - Only construction yard, guard mode, MCVDeploy enabled.

### BuildingClass::Passes_Proximity_Check(cell) → bool
**Placement validation** - Requires adjacent friendly building/wall (human only).

### BuildingClass::Receive_Message() → RadioMessageType
**Radio protocol** - Handles 12 message types for unit coordination.

### BuildingClass::Mission_Attack() / Mission_Guard()
**Combat buildings** - Target acquisition, firing logic.

### BuildingClass::Fire_Coord() / Fire_Direction()
**Projectile spawn** - Uses `CenterOffset` table, turret facing.

### BuildingClass::Death_Announcement(source)
**Voice notification** - VOX_STRUCTURE_LOST/GDI_STRUCTURE/NOD_STRUCTURE.

### BuildingClass::Can_Enter_Cell() → MoveErrorType
**Placement check** - Proximity, clear area, buildable terrain.

### BuildingClass::Begin_Mode(state) / Enter_Idle_Mode()
**Animation state** - Controls `BState` transitions.

### BuildingClass::Detach_All(all)
**Cleanup** - Abandons factory, updates house factory counts.

### BuildingClass::Flush_For_Placement()
**Placement clearing** - Scatters friendlies, attacks hostiles.

### BuildingClass::Power_Output() → int
**Power generation** - Scales with damage: `Power * (Strength/MaxStrength)`.

### BuildingClass::Refund_Amount() → int
**Sell value** - Cost + stored tiberium (v1.07).

### BuildingClass::Crew_Type() → InfantryType
**Survivor** - Silo→C1/C7; Const yard→E7 (rare).

### BuildingClass::Pip_Count() → int
**Sidebar pips** - `Max_Pips * Tiberium_Fraction`.

### BuildingClass::Detach_All()
**Cleanup** - Abandons factory, checks house factory dependency.

### BuildingClass::Death_Announcement()
**Voice** - VOX_STRUCTURE_LOST (own), VOX_ENEMY_STRUCTURE (enemy), house-specific.

### BuildingClass::Fire_Coord() / Fire_Direction()
**Projectile spawn** - Uses `CenterOffset` table, turret facing.

### BuildingClass::Receive_Message()
**Radio protocol** - 12 message types for unit coordination.

### BuildingClass::Validate() (CHEAT_KEYS)
**Pool bounds check** - `Buildings.ID(this)`.

## Building states and missions

### BStateType
| State | Usage |
|-------|-------|
| `BSTATE_IDLE` | Default |
| `BSTATE_CONSTRUCTION` | Build anim |
| `BSTATE_ACTIVE` | Refinery ready |
| `BSTATE_FULL` | Refinery full |
| `BSTATE_IDLE` | Powered down |

### MissionType (building-specific)
| Mission | Building types |
|---------|----------------|
| `MISSION_HARVEST` | Refinery |
| `MISSION_REPAIR` | Repair bay, Helipad |
| `MISSION_MISSILE` | SAM site |
| `MISSION_UNLOAD` | Weapon factory |
| `MISSION_CONSTRUCTION` | Const yard |
| `MISSION_DECONSTRUCTION` | Selling |
| `MISSION_GUARD` | Turrets, defenses |
| `MISSION_REPAIR` | Repair bay |

## Inputs, outputs and side effects

**State mutations:**
- `BState` (animation)
- `Factory` (production)
- `Mission`/`MissionQueue` (orders)
- `Power` output
- `IsReadyToCommence` (docking)
- `WhomToRepay` (repair payment)

**External systems affected:**
- Power grid (`Power_Output`)
- Sidebar (pips, buildables)
- Radio network (unit coordination)
- Factory system (production)
- Power grid (drain/output)
- Voice announcements

## Calls into / called by

**Called by:**
- `LogicClass::AI()` → `BuildingClass::AI()`
- Unit docking → `Receive_Message()`
- Player input → `Sell_Back()`, `Repair()`, `Toggle_Primary()`
- Factory → `Factory_Link()`
- Map → `Place_Down()`, `Pick_Up()`, `Overlap_Up/Down()`

**Calls into:**
- `FactoryClass` (production)
- `RadioClass` (messages)
- `MissionClass` (orders)
- `MapClass` (placement, sight)
- `HouseClass` (power, factory counts)
- `Voice` (`Speak()`)
- `Particle` (`Drop_Debris`)

## Related original files
- **BUILDING.H** - Class declaration, BStateType, SAMState
- **BUILDINGTYPE.H/.CPP** - BuildingTypeClass
- **FACTORY.H/.CPP** - FactoryClass
- **RADIO.H/.CPP** - RadioClass, messages
- **MISSION.H/.CPP** - MissionClass
- **POWER.H/.CPP** - PowerClass
- **SIDEBAR.H/.CPP** - Sidebar integration
- **VOX.H** - Voice constants
- **TECHNO.H/.CPP** - TechnoClass base
- **RADAR.H/.CPP** - RadarClass (power bar inherits)

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/building.rs` - BuildingClass
- `cnc-tiberian-dawn/src/building/td_building.rs` - TD building types
- `cnc-core/src/building/missions.rs` - Mission state machines
- `cnc-core/src/building/refinery.rs` - Harvest logic
- `cnc-core/src/building/sam_site.rs` - SAM state machine
- `cnc-core/src/building/weapon_factory.rs` - Unload logic
- `cnc-core/src/building/construction_yard.rs` - Construction

**Not yet ported:**
- 8-state SAM site machine
- 6-state weapon factory unload
- Refinery harvester docking protocol
- Construction yard build/sell anim
- Power scaling with damage
- Tiberium storage refund (v1.07)
- Crew survivor types
- Proximity check (adjacent friendly)
- Radio message protocol (12 types)
- Bib clearing for placement
- Power scaling with damage
- MCV deploy logic
- Sidebar pip count
- Death announcements (house-specific)

## Edge cases and compatibility notes

1. **Refinery harvester protocol**: Complex radio handshake (HELLO→CAN_LOAD→IM_IN→DOCKING→UNLOADED)
2. **SAM site**: 8-state animation (underground→rising→ready→firing→locking→lowering)
3. **Weapon factory**: 5-state unload (clear→open→leave→close)
4. **Construction yard**: Build/sell animation sync with factory
4. **Power scaling**: Linear with damage (`Power * Strength/MaxStrength`)
5. **Tiberium refund**: Only v1.07+ (`IsV107`), capacity × tiberium fraction
6. **Crew types**: Silo random C1/C7; Const yard 25% E7 for human
7. **Proximity check**: Human only, requires adjacent friendly building/wall
8. **MCV deploy**: Only const yard, guard mode, `Special.IsMCVDeploy`
9. **Sidebar pips**: `Max_Pips * Tiberium_Fraction` (fixed-point)
10. **Death voice**: Own=STRUCTURE_LOST, Enemy=ENEMY_STRUCTURE, House-specific GDI/NOD
11. **Fire coord**: Uses `CenterOffset` table by building size
12. **Radio protocol**: 12 message types, building-type-specific responses
13. **Bib clearing**: Scatters friendlies, attacks enemies in footprint
13. **MCV deploy**: Undeploys to MCV unit
14. **Primary factory toggle**: `Toggle_Primary()` switches house factory
15. **Repair payment**: `WhomToRepay` tracks who pays for repairs

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/BUILDING.CPP at f1f0d42
