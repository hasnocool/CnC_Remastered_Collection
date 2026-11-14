# BUILDING.CPP — Building System Documentation

> **Source**: `original/TIBERIANDAWN/BUILDING.CPP` (5525 lines)  
> **Author**: Joe L. Bostic (Westwood Studios)  
> **Original Date**: September 10, 1993 — Last Update: August 20, 1995  
> **Remaster Notes**: Various patches by Petroglyph/EA (2019-2020) marked with `ST`, `MBL`, `LLL`, `JAS`

---

## Overview

`BUILDING.CPP` implements the `BuildingClass` — the core class for all stationary structures in Tiberian Dawn. It inherits from `TechnoClass` and manages the complete lifecycle of buildings: construction, operation, damage, repair, capture, sale/demolition, and destruction. The file also contains mission state machines for every building type (refineries, power plants, factories, defenses, superweapons, etc.) and the radio message protocol for building-unit coordination.

---

## Class Hierarchy

```
AbstractClass
  └── TechnoClass
        └── BuildingClass
```

---

## Data Members (BuildingClass)

| Member | Type | Description |
|--------|------|-------------|
| `Class` | `BuildingTypeClass*` | Pointer to the building type definition (INI data) |
| `BState` | `BStateType` | Current animation state (`BSTATE_NONE`, `BSTATE_IDLE`, `BSTATE_ACTIVE`, `BSTATE_CONSTRUCTION`, `BSTATE_FULL`, `BSTATE_AUX1`, `BSTATE_AUX2`) |
| `QueueBState` | `BStateType` | Pending animation state change (applied next AI tick) |
| `Status` | `int` | Mission-specific state (varies by mission; SAM sites use `SAMState` enum) |
| `Factory` | `FactoryClass*` | Active production factory (AI-controlled buildings only) |
| `PlacementDelay` | `TimerClass` | Delay before retrying building placement |
| `CountDown` | `TimerClass` | Destruction countdown (8 ticks after death) |
| `IsRepairing` | `bool` | Player-initiated repair in progress |
| `IsCharging` / `IsCharged` | `bool` | Obelisk of Light charge state |
| `IsInRecoilState` | `bool` | Turret recoil animation flag |
| `IsReadyToCommence` | `bool` | Flag to start a new mission on next AI tick |
| `IsWrenchVisible` | `bool` | Repair wrench icon visibility toggle |
| `IsGoingToBlow` | `bool` | C4 sabotage detonation armed |
| `IsSurvivorless` | `bool` | Suppress infantry survivors on destruction |
| `IsCaptured` | `bool` | Building was captured (changes ActLike) |
| `IsSecondShot` | `bool` | Alternates between two-shot capable weapons |
| `Strength` | `int` | Current hit points |
| `LastStrength` | `int` | Previous strength (for power output delta) |
| `WhomToRepay` | `TARGET` | Who damaged this building last (for retaliation) |
| `WhoLastHurtMe` | `HousesType` | House that last dealt damage (for kill credit) |
| `Trigger` | `TriggerClass*` | Attached mission trigger |
| `ArchiveTarget` | `TARGET` | Stored target (used by MCV deconstruction) |
| `Ammo` | `int` | Current ammunition |
| `PurchasePrice` | `int` | Price paid (for free unit logic) |
| `Special` | `BuildingSpecialFlags` | Special behavior flags (`IsMCVDeploy`, `IsSmartDefense`, etc.) |
| `SAM` | `SAMState` | SAM site animation state (enum, see below) |
| `PrimaryFacing` | `FacingClass` | Turret/body facing controller |
| `TarCom` | `TARGET` | Current attack target |
| `IsLeader` | `bool` | Primary factory flag (for multi-factory toggle) |

### SAMState Enum

```cpp
enum SAMState {
    SAM_NONE = -1,          // Non-SAM buildings
    SAM_UNDERGROUND,        // Launcher hidden, awaiting target
    SAM_RISING,             // Doors opening, launcher rising
    SAM_READY,              // Tracking targets, can rotate
    SAM_FIRING,             // Firing first missile
    SAM_READY2,             // Tracking for second missile
    SAM_FIRING2,            // Firing second missile
    SAM_LOCKING,            // Rotating to north for lowering
    SAM_LOWERING            // Lowering into ground to reload
};
```

### BStateType Enum (Building Animation States)

| State | Description |
|-------|-------------|
| `BSTATE_NONE` | No animation |
| `BSTATE_IDLE` | Default idle loop |
| `BSTATE_ACTIVE` | Active operation (refinery unloading, repair bay working) |
| `BSTATE_CONSTRUCTION` | Build-up animation (placement or sell) |
| `BSTATE_FULL` | Refinery "harvester docked" flashing lights |
| `BSTATE_AUX1` | First auxiliary (refinery offload bails) |
| `BSTATE_AUX2` | Second auxiliary (refinery harvester undocking) |

---

## Static Data

### CenterOffset Table

```cpp
COORDINATE const BuildingClass::CenterOffset[BSIZE_COUNT] = {
    0x00800080L,  // BSIZE_1x1
    0x008000FFL,  // BSIZE_1x2
    0x00FF0080L,  // BSIZE_2x1
    0x00FF00FFL,  // BSIZE_2x2
    0x018000FFL,  // BSIZE_3x2
    0x00FF0180L,  // BSIZE_2x3
    0x01800180L,  // BSIZE_3x3
    0x00FF0200L,  // BSIZE_2x4 (Airstrip)
    0x02800280L,  // BSIZE_4x4 (Construction Yard)
};
```

Offset from building origin (`Coord`) to visual/logical center.

### VTable Pointer

```cpp
void* BuildingClass::VTable;  // Cached virtual table pointer for validation
```

---

## Core Methods

### Construction / Destruction

#### `BuildingClass::BuildingClass(StructType type, HousesType house)`
**Constructor.** Initializes all member variables, sets `ActLike` based on house/building type (captured GDI/Nod factories act like their original owner), registers with `House` factory counters, allocates from `Buildings` pool.

#### `BuildingClass::~BuildingClass()`
**Destructor.** Calls `Limbo()` to remove from map, decrements factory counters, removes from AI tracking (if `USE_RA_AI`).

#### `void* BuildingClass::operator new(size_t)`
**Pool allocator.** Allocates from global `Buildings` array, marks active.

#### `void BuildingClass::operator delete(void* ptr)`
**Pool deallocator.** Marks inactive, returns to `Buildings` pool.

#### `void BuildingClass::Init()`
**System initialization.** Frees all buildings, allocates one dummy to capture `VTable` pointer.

---

### Animation & Rendering

#### `void BuildingClass::Draw_It(int x, int y, WindowNumberType window)`
**Main render function.** Determines shape file and frame based on:
- **Construction**: Uses buildup data; reverses frames if deconstructing
- **Turrets** (`IsTurretEquipped`): Body shape from `PrimaryFacing`; adds +16 for SAM rotating states, +32 for recoil, +64 if damaged (<50% health)
- **Obelisk**: Frame 0 (idle), 3 (charged), or animating charge frames
- **Silo** (`STRUCT_STORAGE`): Fill level 0-4 based on `House->Tiberium / Capacity`; +5 if damaged
- **Weapon Factory**: Custom overlay (`WEAP2`/`WarFactoryOverlay`) for door animation; +10 if damaged
- **Damaged generic**: Uses largest anim end frame or simple damage frame

Calls `Techno_Draw_Object` / `Techno_Draw_Object_Virtual`, then `TechnoClass::Draw_It` for selection/health bar.

#### `bool BuildingClass::Mark(MarkType mark)`
**Map cell marking.** Handles `MARK_UP` (remove), `MARK_DOWN` (place), `MARK_CHANGE` (refresh).
- **Walls**: Convert to `OverlayClass` on `MARK_DOWN` and self-delete
- **Bibs** (foundation smudges): Created/removed via `Class->Bib_And_Offset()`
- Calls `Map.Pick_Up` / `Map.Place_Down` / `Map.Refresh_Cells`

#### `void BuildingClass::Begin_Mode(BStateType bstate)`
**Queue animation state change.** Immediate if `BSTATE_NONE`, `BSTATE_CONSTRUCTION`, or `ScenarioInit`. Sets rate (normalized for regulated buildings) and start stage.

---

### AI & Mission System

#### `void BuildingClass::AI()`
**Main AI tick.** Called every frame. Sequence:
1. `Graphic_Logic()` — advance animation, detect stage changes
2. SAM site: `Mark(MARK_CHANGE)` on stage change
3. Non-turret buildings: Handle animation loop → set `IsReadyToCommence` on last frame; loop animation
4. Door animation redraw check
5. Process pending `QueueBState` change
6. Power output delta → `House->Adjust_Power()`
7. Destruction countdown → `Limbo()`, `Drop_Debris()`, `Delete_This()`
8. Obelisk charging logic (requires power ≥ 100%)
9. Repair process (every 15 frames: spend money, heal `Repair_Step()`)
10. AI factory production (non-human only): start `FactoryClass` if money > 10
11. C4 detonation check (`IsGoingToBlow` + `CountDown`)
12. Turret rotation (SAM: 15°/tick, others: 12°/tick)

#### `int BuildingClass::Commence()`
**Mission starter.** Called when `IsReadyToCommence` is true. Dispatches to mission-specific handlers. Returns `true` if mission changed.

---

### Mission State Machines

Each mission is a state machine with `Status` enum and returns delay in ticks.

#### `int BuildingClass::Mission_Guard()`
**Default idle mission.**
- **Armed buildings**: Set `IsReadyToCommence`, search `Greatest_Threat()`, switch to `MISSION_ATTACK` on target
- **Unarmed**: Simple `INITIAL` → `IDLE` loop (5 sec delay)
- **Repair bay special**: If customer waiting at dock, switch to `MISSION_REPAIR`

#### `int BuildingClass::Mission_Construction()`
**Build-up animation.** `INITIAL` → `DURING`. On `IsReadyToCommence`: `RADIO_COMPLETE`, `RADIO_OVER_OUT`, `Begin_Mode(BSTATE_IDLE)`, `Grand_Opening()`, `MISSION_GUARD`.

#### `int BuildingClass::Mission_Deconstruction()`
**Sell/demolish animation.**
- `INITIAL`: If repair bay has unit, sell unit instead. Else `RADIO_RUN_AWAY`, spawn crew (1-5 infantry based on cost, captured = double divisor), play cash sound, destroy attached objects, `RADIO_OVER_OUT`, `Begin_Mode(BSTATE_CONSTRUCTION)`, `IsSurvivorless=true`
- `HOLDING`: Wait for `!IsTethered`
- `DURING`: On `IsReadyToCommence`:
  - **MCV deploy** (human, `Special.IsMCVDeploy`): Convert to `UNIT_MCV` at adjacent cell, preserve health, restore move order
  - **Normal**: `WhoLastHurtMe = HOUSE_NONE`, record kill, refund `Refund_Amount()`, `Limbo()`, `Delete_This()`

#### `int BuildingClass::Mission_Attack()`
**Combat building attack.**
- **SAM Site**: Full 9-state machine (`SAM_UNDERGROUND` → `RISING` → `READY` → `FIRING` → `READY2` → `FIRING2` → `LOCKING` → `LOWERING` → `UNDERGROUND`). Tracks aircraft only. Two-shot burst. 3-sec reload lower.
- **Others**: Check `Can_Fire()`:
  - `FIRE_ILLEGAL/CANT/RANGE/AMMO` → clear target, `MISSION_GUARD`
  - `FIRE_FACING` → rotate turret, delay 2
  - `FIRE_REARM/BUSY` → delay 1
  - `FIRE_CLOAKED` → uncloak
  - `FIRE_OK` → `Fire_At()`, delay 1

#### `int BuildingClass::Mission_Harvest()`
**Refinery unload.** 5 states:
- `INITIAL` → `Begin_Mode(BSTATE_ACTIVE)`
- `WAIT_FOR_DOCK` → on `IsReadyToCommence`: `Begin_Mode(BSTATE_AUX1)`
- `MIDDLE` → on `IsReadyToCommence`: offload tiberium bails (`Offload_Tiberium_Bail()`), scatter bib occupants, if harvester still has load → repeat, else `Begin_Mode(BSTATE_AUX2)`
- `WAIT_FOR_UNDOCK` → on `IsReadyToCommence`: `Exit_Object(Detach_Object())`, `MISSION_GUARD`

#### `int BuildingClass::Mission_Repair()`
**Repair facility / Construction Yard / Helipad.**
- **Construction Yard** (`STRUCT_CONST`): `Begin_Mode(BSTATE_ACTIVE)`, wait for radio contact loss → `MISSION_GUARD`
- **Repair Bay** (`STRUCT_REPAIR`): `INITIAL` → `IDLE` (wait for docked unit at <16 leptons) → `DURING` (active repair: `RADIO_REPAIR` each tick, rate scaled by power). Exits on `RADIO_NEGATIVE` or no contact.
- **Helipad** (`STRUCT_HELIPAD`): `INITIAL` (wait for aircraft, `RADIO_PREPARED`=negative) → `DURING` (reload ammo: `RADIO_RELOAD`, delay scaled by power).

#### `int BuildingClass::Mission_Missile()`
**Temple of Nod nuclear missile.** 5 states:
- `INITIAL` → `Begin_Mode(BSTATE_ACTIVE)`
- `DOOR_OPENING` → on `IsReadyToCommence`: `Begin_Mode(BSTATE_IDLE)`, spawn `ANIM_ATOM_DOOR`
- `LAUNCH_UP` → spawn `BULLET_NUKE_UP` at offset, `VOX_NUKE_LAUNCHED`, `VOC_NUKE_FIRE`
- `LAUNCH_DOWN` (8 sec later) → spawn `BULLET_NUKE_DOWN` at map top over `House->NukeDest`, `VOX_INCOMING_NUKE`, `Payback = this` (for kill credit)
- `DONE_LAUNCH` → `MISSION_GUARD` (60 sec cooldown)

#### `int BuildingClass::Mission_Unload()`
**Weapon Factory vehicle exit.** 5 states:
- `INITIAL` → `Open_Door(2,11)`, `CLEAR_BIB`
- `CLEAR_BIB` → scatter units blocking exit cell and adjacents
- `OPEN` → on `Is_Door_Open()`: order unit `MISSION_MOVE` with `Force_Track(OUT_OF_WEAPON_FACTORY)`, `LEAVE`
- `LEAVE` → on `!IsTethered`: `Close_Door(2,11)`, `CLOSE`
- `CLOSE` → on `Is_Door_Closed()`: `Enter_Idle_Mode()`

---

### Radio Message Protocol

`RadioMessageType BuildingClass::Receive_Message(RadioClass* from, RadioMessageType message, long& param)`

Handles building↔unit coordination. Key messages:

| Message | Context | Response |
|---------|---------|----------|
| `RADIO_HELLO` | Unit requests attach | `RADIO_NEGATIVE` if `MISSION_HARVEST` busy |
| `RADIO_CAN_LOAD` | Unit asks if can dock | Checks type: Airstrip→Cargo plane, Helipad→rotary, Repair→damaged unit/aircraft, Refinery→harvester (if not busy) |
| `RADIO_IM_IN` | Unit docked | Repair/Helipad→`MISSION_REPAIR`; Refinery→`Begin_Mode(BSTATE_ACTIVE)`, `MISSION_HARVEST` |
| `RADIO_DOCKING` | Unit approaching | Refinery: flash lights (`BSTATE_FULL`); Repair: if other unit waiting, `RADIO_ON_DEPOT`/`RADIO_NEED_REPAIR`/`RADIO_RUN_AWAY`; Establish contact, send `RADIO_MOVE_HERE` with target cell |
| `RADIO_ARE_REFINERY` | Harvester seeks refinery | `RADIO_NEGATIVE` if busy, wrong house, or not refinery |
| `RADIO_BUILDING` | ConYard starts build | `MISSION_REPAIR` (animate ConYard) |
| `RADIO_COMPLETE` | Build finished | `MISSION_GUARD` (stop ConYard anim) |
| `RADIO_OVER_OUT` | Contact lost | `Begin_Mode(BSTATE_IDLE)` |
| `RADIO_UNLOADED` | Unit left | Refinery: `Begin_Mode(BSTATE_IDLE)`; Weapon/Repair/Airstrip: `RADIO_RUN_AWAY` |

---

### Damage & Destruction

#### `ResultType BuildingClass::Take_Damage(int& damage, int distance, WarheadType warhead, TechnoClass* source)`
**Damage handler.** Special cases:
- **SAM underground**: 50% damage (min 1)
- **Temple vs Ion Cannon** (`WARHEAD_PB`): +50% damage
- Calls `TechnoClass::Take_Damage()`
- On `RESULT_DESTROYED`: Destroy attached objects, spawn crater/fire/smoke anims per occupied cell, screen shake (`Cost/400`), `VOC_CRUMBLE`, `CountDown=8` (or 0 if deconstructing), set `TempleIoned` if Temple+Ion
- On `RESULT_HALF/MAJOR`: Spawn fire/smoke anims on occupied cells
- On any damage: `House->Attacked(this)`, `WhoLastHurtMe = source->Owner()`, auto-target attacker if no target and not SAM

#### `void BuildingClass::Drop_Debris(TARGET source)`
**Destruction effects.**
- **Special case**: Mission 10, Nod, `STRUCT_MISSION` → spawn `INFANTRY_CHAN` with win trigger
- **Survivors**: Per occupied cell, `1/odds` chance (odds=2, -1 if sabotaged, +6 if captured) → spawn `Crew_Type()` infantry, scatter, attack source or guard/hunt
- **Smoke**: 60% chance `ANIM_SMOKE_M` per cell
- **Ground scars**: 33% scorch, 66% crater smudges

#### `void BuildingClass::Death_Announcement(TechnoClass const*)`
**Voice announcements.** If discovered/owned by player:
- Enemy building: `VOX_ENEMY_STRUCTURE`
- Own building: `VOX_STRUCTURE_LOST` or faction-specific `VOX_GDI_STRUCTURE`/`VOX_NOD_STRUCTURE`

---

### Building-Specific Logic

#### `COORDINATE BuildingClass::Center_Coord() const`
Returns `Coord + CenterOffset[Class->Size]`.

#### `COORDINATE BuildingClass::Fire_Coord(int) const`
**Projectile spawn offset by type:**
- Guard/Advanced Guard Tower: `+0x30 N`, `+0x40 toward target`
- Obelisk: `+0xA8 N`, `+0x18 W`
- SAM/Turret: `+0x30 N`, `+0x80 toward PrimaryFacing`

#### `FireErrorType BuildingClass::Can_Fire(TARGET target, int which) const`
**Fire validation.** Adds turret facing check (±8 256ths), Advanced Guard Tower power check (≥100%), Obelisk charge check (`IsCharged`).

#### `TARGET BuildingClass::Greatest_Threat(ThreatType threat) const`
**Target acquisition.** Non-SAM: adds `THREAT_INFANTRY|BOATS|VEHICLES|RANGE` (+`BUILDINGS` for AI). SAM: adds `THREAT_AREA`. Anti-air weapon → adds `THREAT_AIR`.

#### `bool BuildingClass::Can_Demolish() const`
**Sell availability.** False if `IsUnsellable`, no buildup data, constructing, deconstructing, or refinery with attached harvester.

#### `bool BuildingClass::Can_Capture() const`
**Capture availability.** `IsCaptureable` and not deconstructing. Override: AI-owned with win-trigger → capturable.

#### `int BuildingClass::Refund_Amount() const`
**Sell value.** Base refund + stored tiberium (if `IsV107` and `Capacity>0`): `Capacity * (Tiberium/Capacity)`.

#### `InfantryType BuildingClass::Crew_Type() const`
**Survivor infantry type.**
- Silo: 50% `INFANTRY_C1` (technician), 50% `INFANTRY_C7`
- Construction Yard (human, not captured): 25% `INFANTRY_E7` (engineer)
- Default: `TechnoClass::Crew_Type()` (minigunner)

#### `bool BuildingClass::Passes_Proximity_Check(CELL homecell)`
**Placement rule.** Human only. Scans foundation + adjacent cells for friendly building or `Map[cell].Owner == house`. Editor/debug/AI always pass.

#### `bool BuildingClass::Can_Player_Move() const`
**MCV undeploy.** Only `STRUCT_CONST` in `MISSION_GUARD` with `Special.IsMCVDeploy`.

---

### Power & Resources

#### `int BuildingClass::Power_Output() const`
**Current power.** `Class->Power * (LastStrength / MaxStrength)` (fixed-point math).

#### `void BuildingClass::Grand_Opening(bool captured)`
**Post-construction init.** Adjusts house: `Drain`, `Capacity`, `IsRecalcNeeded`.
- **Refinery**: Free harvester (if not captured, not debug, human or free/cheat) at SW cell
- **Helipad**: Free Orca (GDI) or Helicopter (Nod) at docking coord

#### `void BuildingClass::Repair(int control)`
**Player repair toggle.** `-1`=toggle, `1`=on, `0`=off. Plays `VOC_BUTTON`/`VOC_SCOLD`, shows wrench icon. AI repair handled in `AI()` tick.

#### `void BuildingClass::Sell_Back(int control)`
**Player sell toggle.** Same control scheme. Only if has buildup data. `Assign_Mission(MISSION_DECONSTRUCTION)`.

---

### Capture Logic

#### `bool BuildingClass::Captured(HouseClass* newowner)`
**Ownership transfer.** If capturable and different owner:
1. Voice: `VOX_GDI_CAPTURED`/`VOX_NOD_CAPTURED`
2. Achievement events (captured opposing/same faction)
3. Internet stats: `CapturedBuildings++`
4. Power/drain/capacity adjustment (old owner loses, new gains)
5. Attached/contact units captured too
6. Factory counters decremented (old) / incremented (new)
7. `IsCaptured=true`, `TechnoClass::Captured(newowner)`
8. Bib smudge transfer
9. `Grand_Opening(true)` (free units for new owner)
10. `Look()` for new owner
11. Old owner `Check_Pertinent_Structures()`

---

### Serialization

#### `void BuildingClass::Read_INI(char* buffer)`
**Scenario load.** Format per entry: `House,Type,Strength,Cell,Facing,TriggerName`. Creates building, `Unlimbo()` at cell, sets strength (fixed-point), trigger attachment.

#### `void BuildingClass::Write_INI(char* buffer)`
**Scenario save.** Iterates `Buildings` pool, writes non-limbo buildings. Format: `House,Type,HealthRatio,Cell,Facing,TriggerName`.

---

### Utility & Queries

| Method | Description |
|--------|-------------|
| `Validate()` | Debug: checks `Buildings.ID(this)` in range |
| `Debug_Dump(MonoClass*)` | Debug: prints state to monochrome screen |
| `As_Target()` | Returns `Build_Target(KIND_BUILDING, ID)` |
| `Target_Coord()` | Center offset adjusted to occupied cell |
| `Docking_Coord()` | Helipad: `(24,18)`, Airstrip: `(18,30)`, else base |
| `Sort_Y()` | Render sort Y: Repair=origin, Helipad/Barracks/Refinery=center, else center+height*2/3 |
| `Find_Exit_Cell(TechnoClass*)` | Searches `Class->ExitList` for valid cell |
| `Crew_Type()` | See above |
| `Detach_All(bool)` | Abandons `Factory`, checks house factory for last-building abandonment |
| `Flush_For_Placement(TechnoClass*, CELL)` | Scatters allied units, attacks enemy units in target footprint |
| `Hidden()` | Calls base; commented drain adjustment |
| `Update_Buildables()` | Human player: adds buildable types to sidebar/Map per `Class->ToBuild` |
| `Update_Specials()` | Stub (removed computer specials logic) |
| `Pip_Count()` | Tiberium pips for silo: `Max_Pips * (Tiberium/Capacity)` |
| `Remap_Table()` | Returns house remap (non-bluishing) |
| `Fire_Direction()` | Turret: `PrimaryFacing`, else `Direction(TarCom)` |

---

## Special Building Behaviors

| Building | Type | Special Logic |
|----------|------|---------------|
| **Construction Yard** | `STRUCT_CONST` | MCV deploy/undeploy (`Special.IsMCVDeploy`); animates during AI build (`MISSION_REPAIR`); sells to MCV |
| **Refinery** | `STRUCT_REFINERY` | Free harvester on build; `MISSION_HARVEST` state machine; `BSTATE_FULL` flashing lights; harvester exit SW |
| **Power Plant/Adv Power** | `STRUCT_POWER`/`STRUCT_APWR` | Power output scales with health |
| **Barracks/Hand of Nod** | `STRUCT_BARRACKS`/`STRUCT_HAND` | Infantry exit via `ExitList`; captured → acts like original owner |
| **Weapon Factory/Airstrip** | `STRUCT_WEAP`/`STRUCT_AIRSTRIP` | Vehicle exit with door anim (`MISSION_UNLOAD`); custom overlay `WEAP2`; Cargo plane for Airstrip |
| **Repair Bay** | `STRUCT_REPAIR` | Repairs docked units; `MISSION_REPAIR`; sells docked unit first |
| **Helipad** | `STRUCT_HELIPAD` | Free aircraft on build; rearms/repairs rotary; `MISSION_REPAIR` |
| **Silo** | `STRUCT_STORAGE` | Visual fill 0-4; technician survivors; tiberium capacity |
| **Guard Tower/Adv Guard Tower** | `STRUCT_GTOWER`/`STRUCT_ATOWER` | Turret; AGT requires power |
| **Obelisk of Light** | `STRUCT_OBELISK` | Charge anim (4 frames); requires power; laser weapon |
| **SAM Site** | `STRUCT_SAM` | 9-state underground/ready/fire/lower; 2-shot burst; 3s reload; half damage underground |
| **Temple of Nod** | `STRUCT_TEMPLE` | Nuke launch (`MISSION_MISSILE`); ion cannon death flag; win trigger protection |
| **Walls** | `STRUCT_*_WALL` | Convert to overlay on placement; never unlimbo as building |
| **Communication Center** | `STRUCT_TECH` | Radar reveal; special mission logic elsewhere |

---

## Constants & Magic Values

| Constant | Value | Use |
|----------|-------|-----|
| `OBELISK_ANIMATION_RATE` | (undefined in file) | Obelisk charge animation speed |
| `REPAIR_THRESHHOLD` | (undefined) | Min money for AI auto-repair |
| `TICKS_PER_SECOND` | 15 | Frame rate base |
| `0x0100` | 256 | Fixed-point 1.0 (full health/power) |
| `0x0080` | 128 | Fixed-point 0.5 (half health) |
| `0x0040` | 64 | Fixed-point 0.25 |
| `0x00C0` | 192 | Fixed-point 0.75 |

---

## Multiplayer / Remaster Changes (2019-2020)

| Change | Author | Date | Description |
|--------|--------|------|-------------|
| GlyphX Sidebar | ST | 3/26/2019 | `SidebarGlyphx.h`, `Sidebar_Glyphx_Add()` |
| Multiplayer Reveal | ST | 4/3/2019 | `Look()` checks per-player visibility |
| Power Redraw | ST | 3/13/2019 | `Is_Owned_By_Player()` replaces `IsOwnedByPlayer` |
| Refinery Anim Fix | LLL | 4/22/2020 | `BSTATE_FULL` only when not `MISSION_HARVEST` |
| Nuke Kill Credit | MBL | 5/20/2020 | `bullet->Payback = this` for `BULLET_NUKE_DOWN` |
| Repair VO | MBL | 7/6/2020 | `VOX_REPAIRING` on repair bay start |
| Nuke VO Multiplayer | MBL | 3/27/2020 | All players hear `VOX_NUKE_LAUNCHED`/`INCOMING_NUKE` |
| Sell SFX | MBL | 7/10/2020 | All players hear `VOC_CASHTURN` positionally |
| Achievement Events | ST | 11/14/2019 | `BUILDING_DESTROYED_C4`, `ION_DESTROYS_TEMPLE`, `OPPOSING_BUILDING_CAPTURED`, `BUILDING_CAPTURED` |
| AI Tracking | ST | 7/26/2019 | `USE_RA_AI`: `House->Tracking_Add/Remove/Recalc_Center()` |
| Primary Factory VO | MBL | 7/22/2020 | Per-player `VOX_PRIMARY_SELECTED` |
| Detach Cleanup | ST | 4/24/2020 | Destroy attached objects on sell |

---

## File Dependencies

```cpp
#include "function.h"
#include "SidebarGlyphx.h"  // Multiplayer sidebar
```

Relies on global systems: `Buildings` (pool), `Map`, `HouseClass`, `TechnoClass`, `FactoryClass`, `TriggerClass`, `AnimClass`, `BulletClass`, `SmudgeClass`, `OverlayClass`, `Sidebar`/`Map` buildable UI, `Options`, `ScenarioInit`, `GameToPlay`, `PlayerPtr`, `Debug_Map`.

---

## Summary

`BUILDING.CPP` is the authoritative implementation of all structure gameplay in Tiberian Dawn. It combines:
- **Entity management** (pool allocation, map marking, serialization)
- **Visual representation** (animation state machine, turret rendering, damage overlays)
- **AI behavior** (per-mission state machines, target acquisition, production)
- **Multiplayer coordination** (radio protocol, per-player reveal/voice)
- **Economy integration** (power, tiberium capacity, repair costs, sell refunds)
- **Special logic** for every unique building type (MCV, nuke, ion, walls, etc.)

The codebase shows its age (manual memory management, fixed-point math, global state) but is functionally complete and well-commented with historical context.