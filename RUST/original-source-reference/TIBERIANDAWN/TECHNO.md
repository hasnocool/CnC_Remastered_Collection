# PATH/FILE
original/TIBERIANDAWN/TECHNO.CPP

## Purpose
Core TechnoClass hierarchy for Tiberian Dawn. Implements the base class for all "technological" objects (units, infantry, aircraft, buildings) with movement, combat, cloaking, transport, docking, AI, and player interaction logic. Also defines TechnoTypeClass for type metadata.

## Where it fits in the game
Core entity hierarchy - `TechnoClass` extends `ObjectClass` (which extends `AbstractClass`). All mobile/active entities inherit: `UnitClass`, `InfantryClass`, `AircraftClass`, `BuildingClass`. Global pools: `Units`, `Infantry`, `Aircraft`, `Buildings`.

## Classes, structs, enums and important data

### TechnoClass : ObjectClass
**Mobile/active entity base** - Adds movement, combat, cloaking, transport, docking, AI.

**Key data members:**
| Field | Purpose |
|-------|---------|
| `House` (HouseClass*) | Owner |
| `Class` (TechnoTypeClass*) | Type metadata |
| `Mission` / `MissionQueue` / `SuspendedMission` | Order system |
| `Timer` / `Status` | Mission state machine |
| `PrimaryFacing` / `SecondaryFacing` | Body/turret facing |
| `Cloak` (CloakType) | Cloak state |
| `IsDiscoveredByPlayerMask` | Per-house discovery bits |
| `IsDiscoveredByPlayer` | Legacy single-player flag |
| `Attached` / `Next` | Transport cargo chain |
| `IsLeader` / `IsSelected` | UI state |
| `Factory` (BuildingClass*) | Factory link (buildings) |
| `IsReadyToCommence` | Docking ready |
| `WhomToRepay` (TARGET) | Repair payment |
| `PurchasePrice` | Build cost paid |

### TechnoTypeClass : ObjectTypeClass
**Type metadata** - Static data for all techno objects.

**Key data members:**
| Field | Purpose |
|-------|---------|
| `Primary` / `Secondary` (WeaponType) | Weapons |
| `Cost` (int) | Credit cost |
| `MaxSpeed` (MPHType) | Movement speed |
| `SightRange` (int) | Vision range |
| `Armor` (ArmorType) | Armor type |
| `MaxAmmo` (int) | Ammo capacity |
| `CostBias` (via House) | Difficulty cost modifier |
| `Primary` / `Secondary` (WeaponType) | Weapon slots |
| `IsTurretEquipped` | Has independent turret |
| `IsTransporter` | Can carry cargo |
| `IsBuildable` | Player can build |
| `IsRepairable` | Can be repaired |
| `IsStealthy` | Can cloak |
| `IsCrushable` | Can crush infantry |
| `IsLeader` | Can be leader unit |
| `IsScanner` | Reveals shroud |
| `IsTransporter` | Can transport |
| `IsTwoShooter` | Dual weapons |
| `IsTurretEquipped` | Has turret |
| `IsRepairable` | Can be repaired |
| `IsBuildable` | Player buildable |
| `Ownable` (bitmask) | Which houses can build |

### CloakType enum
| Value | State |
|-------|-------|
| `CLOAKED` | Fully cloaked |
| `UNCLOAKING` | Revealing |
| `UNCLOAKED` | Visible |

### BodyShape table (32 entries)
Maps 256-direction facing to 32 body shape frames (0-31).

## Functions and methods

### TechnoTypeClass constructor
**25 parameters** - Comprehensive type initialization.

**Key computed:**
- `Risk = (Attack * (Range>>4)) / ROF` (primary weapon only)
- `Cost` = base cost
- `Ownable` = house bitmask

### TechnoTypeClass::Time_To_Build(house) → int
**Build time calculation** - Complex difficulty/power/IQ scaling.

**Algorithm:**
1. Base = `Raw_Cost()`
2. AI easy: +4000/2; AI normal: +2000/2 (buildings only, human opponent)
3. Nod air units: -25% (airfield delivery)
4. AI build slowdown: `time * (MaxIQ*2) / (IQ + MaxIQ)`
4. Power scaling: `time * 256 / inv_scale`
   - 0 power: inv_scale=64 (4x slower)
   - <50%: inv_scale=103 (~2.5x)
   - <100%: inv_scale=171 (~1.5x)
   - Full: inv_scale=256 (normal)

### TechnoTypeClass::Cost_Of() → int
Returns `Cost` (credit cost).

### TechnoTypeClass::Raw_Cost() → int
Returns `Cost` (base cost, no free items).

### TechnoTypeClass::Get_Ownable() → unsigned short
Returns `Ownable` bitmask.

### TechnoClass::Time_To_Build(house) → int
Delegates to `Class->Time_To_Build(house)`.

### TechnoClass::Refund_Amount() → int
**Sell value:**
1. `cost = Raw_Cost() * House->CostBias`
2. Harvester load: `+ Fixed_To_Cardinal(FULL_LOAD_CREDITS, Tiberium_Load())/2`
3. Human: `/= 2`

### TechnoClass::Find_Docking_Bay(type, friendly) → BuildingClass*
**Finds nearest docking building:**
1. Checks `House->BScan` bitmask for type
2. Iterates buildings: ally check, not limbo, correct type, `RADIO_CAN_LOAD` = ROGER
3. Picks closest (or `IsLeader` preferred)
4. Returns best or NULL

### TechnoClass::Find_Exit_Cell(techno) → CELL
Returns `Coord_Cell(Docking_Coord())` (building exit cell).

### TechnoClass::Is_Discovered_By_Player(house) → bool
Checks `IsDiscoveredByPlayerMask` bit for house.

### TechnoClass::Set_Discovered_By_Player(house)
Sets bit in `IsDiscoveredByPlayerMask`; updates legacy `IsDiscoveredByPlayer` for PlayerPtr.

### TechnoClass::Clear_Discovered_By_Players()
Clears mask and legacy flag.

### TechnoClass::Is_Owned_By_Player(house) → bool
Returns `House == player` (or PlayerPtr if null).

### TechnoClass::Is_Cloaked(house/object) → bool
Returns `!Is_Ally(house) && Cloak == CLOAKED`.

### TechnoTypeClass::Legal_Placement(cell) → int (USE_RA_AI)
**Placement validation:**
- Buildings: delegates to `BuildingTypeClass::Legal_Placement`
- Mobile: checks `Map[cell].Is_Clear_To_Move(false, false)` for all occupy offsets

## RA AI extensions (USE_RA_AI)

### TechnoClass::Anti_Air() → int
**AA strength:** `(Attack * warhead->Modifier[ALUMINUM] * Range) / ROF / 50` (if homing AA weapon)

### TechnoClass::Anti_Armor() → int
**Armor strength:** `(Attack * warhead->Modifier[STEEL] * min(Range,0x400) * SpreadFactor) / ROF / 50` (if ground weapon)

### TechnoClass::Anti_Infantry() → int
**Infantry strength:** `(Attack * warhead->Modifier[NONE] * min(Range,0x400) * SpreadFactor) / ROF / 50` (if ground weapon)

### TechnoTypeClass::Legal_Placement(cell) → int (USE_RA_AI)
Same as TechnoClass version.

## Core virtual methods (overridden by derivatives)

### TechnoClass::AI()
Per-frame AI - calls `ObjectClass::AI()`, processes mission via `MissionClass::AI()`.

### TechnoClass::AI() (TechnoTypeClass)
Static type methods.

### TechnoClass::Can_Fire(target, which) → FireErrorType
**Firing legality:**
1. Base check
2. Weapon selection (primary/secondary)
3. Range check
4. Facing check
5. Cloak check

### TechnoClass::Fire_At(target, which) → BulletClass*
Fires weapon, creates bullet, applies reload.

### TechnoClass::Can_Repair() → bool
Repairable + damaged + not building.

### TechnoClass::Take_Damage(amount, source, warhead) → Result
Damage application with armor/warhead modifiers.

### TechnoClass::Receive_Message(from, message, param) → RadioMessageType
Radio protocol handler.

### TechnoClass::Receive_Message(from, message, param) → RadioMessageType
**Radio protocol:**
- `RADIO_HELLO`: Attach request
- `RADIO_CAN_LOAD`: Dock permission
- `RADIO_IM_IN`: Docked
- `RADIO_DOCKING`: Maneuvering
- `RADIO_NEED_REPAIR`: Repair request
- `RADIO_OVER_OUT`: Contact lost

### TechnoClass::Assign_Destination(dest) / Assign_Target(target)
Sets `NavCom` / `TarCom`.

### TechnoClass::Assign_Mission(order)
Queues mission via `MissionClass::Assign_Mission()`.

### TechnoClass::Enter_Idle_Mode(initial)
Virtual - reverts to `MISSION_GUARD`.

### TechnoClass::Find_Docking_Bay(type, friendly) → BuildingClass*
Finds nearest compatible docking building.

### TechnoClass::Draw_Pips(x, y, window)
Renders transport pips (cargo/harvester load).

### TechnoClass::Random_Animate()
Idle animation.

### TechnoClass::Scatter(coord, forced)
Flees from coordinate.

### TechnoClass::Target_Something_Nearby()
Auto-acquires nearby target.

### TechnoClass::Greatest_Threat(threat_type) → TARGET
Finds highest-threat target in range.

### TechnoClass::In_Range(target) → bool
Weapon range check.

### TechnoClass::Risk() → int
Threat value for AI.

### TechnoClass::Response_Attack() / Response_Move() / Response_Select()
Voice responses.

### TechnoClass::Record_The_Kill(victim)
Kill scoring.

### TechnoClass::Captured(captor)
Ownership transfer.

### TechnoClass::Stun()
Pre-removal prep.

### TechnoClass::Click_As_Target()
Flash feedback.

### TechnoClass::Do_Cloak() / Do_Uncloak() / Do_Shimmer()
Cloak state machine.

### TechnoClass::Kill_Cargo()
Destroys attached cargo.

### TechnoClass::Value() → long
Target priority value.

### TechnoClass::Pip_Count() → int
Transport pips / harvester load.

### TechnoClass::Visual_Character() → int
Rendering variant.

### TechnoClass::Weapon_Range() → int
Max weapon range.

### TechnoClass::Threat_Range() → int
AI scan range.

### TechnoClass::Desired_Load_Dir() → FacingType
Transport load facing.

### TechnoClass::Per_Cell_Process()
Per-cell periodic (tiberium harvest).

### TechnoClass::Click_As_Target()
Selection flash.

### TechnoClass::Exit_Object(obj)
Ejects cargo.

### TechnoClass::What_Action(object/cell) → ActionType
Context-sensitive action (move/attack/enter/capture/etc.).

### TechnoClass::Player_Assign_Mission(order)
Player order with `Commence()`.

### TechnoClass::Set_Mission(mission) (editor)
Forced mission set.

### TechnoClass::Stun()
Pre-delete cleanup.

## Inputs, outputs and side effects

**State mutations:**
- Position, facing, velocity
- Mission/queue/status/timer
- Cloak state
- Health/strength
- Cargo attachments
- Discovery bits
- Selection state

**External systems affected:**
- Combat (damage, projectiles)
- Movement (pathfinding)
- Transport (cargo)
- AI (orders, threats)
- UI (selection, pips, health bar)
- Audio (voice responses)
- Network (sync)

## Calls into / called by

**Called by:**
- `LogicClass::AI()` → `TechnoClass::AI()`
- Player input → `Assign_Mission()`, `Assign_Destination/Target()`
- Factory → `Create_One_Of()`
- Radio → `Receive_Message()`
- AI planning → `Assign_Mission()`, `Greatest_Threat()`

**Calls into:**
- `MissionClass` (orders)
- `RadioClass` (messages)
- `BulletClass` (firing)
- `Pathfinding` (movement)
- `HouseClass` (ownership, factory)
- `MapClass` (placement, docking)
- `Voice` (`Speak()`)
- `Sound` (`Sound_Effect()`)

## Related original files
- **TECHNO.H** - Class declarations
- **OBJECT.H/.CPP** - ObjectClass base
- **UNIT.H/.CPP** - UnitClass
- **INFANTRY.H/.CPP** - InfantryClass
- **AIRCRAFT.H/.CPP** - AircraftClass
- **BUILDING.H/.CPP** - BuildingClass
- **MISSION.H/.CPP** - MissionClass
- **RADIO.H/.CPP** - RadioClass
- **WEAPON.H/.CPP** - Weapons
- **TARGET.H** - TARGET type
- **VOX.H** - Voice constants
- **TECHNOTYPE.H** - TechnoTypeClass

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/techno.rs` - TechnoClass
- `cnc-tiberian-dawn/src/techno/td_techno.rs` - TD types
- `cnc-core/src/techno/type.rs` - TechnoTypeClass
- `cnc-core/src/ai/mission.rs` - MissionClass
- `cnc-core/src/ai/targeting.rs` - Greatest_Threat, In_Range
- `cnc-core/src/ai/radio.rs` - Receive_Message
- `cnc-core/src/transport.rs` - Transport/cargo

**Not yet ported:**
- Complex constructor (25 params)
- Build time with power/IQ/difficulty scaling
- Refund with harvester load
- Docking bay search with radio protocol
- Cloak state machine
- Transport pip rendering
- Anti-air/armor/infantry calculations
- Legal placement (RA AI)
- Voice response system
- 21-mission type system
- Per-cell processing (tiberium)
- Leader/primary unit tracking

## Edge cases and compatibility notes

1. **Build time scaling**: Complex formula with 5 factors (difficulty, faction, AI IQ, power, base cost)
2. **Refund**: Human gets 50%; harvester load adds 50% of full load value
3. **Docking**: Requires `RADIO_CAN_LOAD` = ROGER; prefers `IsLeader` building
4. **Cloak**: Only vs non-allies; `CLOAKED` state
5. **Discovery**: Per-house bitmask (`IsDiscoveredByPlayerMask`)
6. **AA/Armor/Infantry**: Only for RA AI (`USE_RA_AI`); uses weapon/warhead stats
7. **Placement**: Buildings delegate to `BuildingTypeClass`; mobile uses `Is_Clear_To_Move`
8. **Cost_Of vs Raw_Cost**: `Raw_Cost` = base; `Cost_Of` = same (no free items in TD)
9. **Refund**: Includes harvester tiberium load (50% value)
10. **Owner bits**: `Ownable` bitmask per house (GDI/NOD/etc.)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/TECHNO.CPP at f1f0d42
