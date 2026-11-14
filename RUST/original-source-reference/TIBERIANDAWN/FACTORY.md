# PATH/FILE
original/TIBERIANDAWN/FACTORY.CPP

## Purpose
Factory production system for Tiberian Dawn. Manages construction queues for buildings, units, infantry, aircraft, and special weapons (ion cannon) with cost installment payments, multi-factory acceleration for human players, and suspension/resumption logic.

## Where it fits in the game
Production system - each building with a factory capability (Barracks, War Factory, Airstrip, Construction Yard) owns a `FactoryClass` instance. Called once per tick via `FactoryClass::AI()` to advance production, deduct costs, and signal completion.

## Classes, structs, enums and important data

### FactoryClass
**Production controller** - Pooled allocation via `Factories` pool.

**Key data members:**
- `Object` (TechnoClass*) - Unit/building/infantry/aircraft being built (in limbo)
- `SpecialItem` (int) - Special weapon type (SPC_ION_CANNON etc), `SPC_NONE` = normal
- `House` (HouseClass*) - Owner house
- `Balance` (int) - Remaining credits to spend
- `IsSuspended` (bool) - Paused (insufficient funds, player hold, or complete)
- `IsDifferent` (bool) - Production advanced this tick (for UI update)
- `IsBlocked` (bool) - Placement blocked (unused?)
- `Rate` / `Stage` - Inherited tick counter (from `StageClass`)

### SpecialItem enum (SPC_)
- `SPC_NONE` - Normal production
- `SPC_ION_CANNON` - Ion cannon superweapon
- Others for nuke, GPS, etc.

### Constants
- `STEP_COUNT` - Total production steps (typically 100)
- `FACTORY_MAX` - Pool size limit
- `TICKS_PER_MINUTE` - Time base

### House factory counts (human acceleration)
- `House->AircraftFactories` - Airstrips
- `House->InfantryFactories` - Barracks/Hand of Nod
- `House->UnitFactories` - War Factories
- `House->BuildingFactories` - Construction Yards

## Functions and methods

### FactoryClass::AI()
**Per-tick production advance** - Called once per game tick per factory.

**Logic:**
1. Skip if `IsSuspended` or empty (`!Object && !SpecialItem`)
2. **Acceleration factor** (human only):
   - Count factories of same type for this house
   - `stages = MAX(count, 1)`
   - Computer: always 1 (per-building basis)
3. Loop `stages` times:
   - If not complete and `Graphic_Logic()` (tick gate):
     - `IsDifferent = true`
     - `cost = Cost_Per_Tick()` (remaining balance / remaining steps)
     - `cost = MIN(cost, Balance)`
     - **Funds check**: If `cost > House->Available_Money()`:
       - `Set_Stage(Fetch_Stage()-1)` (rollback)
     - Else:
       - `House->Spend_Money(cost)`, `Balance -= cost`
     - **Instant build cheat**: `Debug_Instant_Build` → `Set_Stage(STEP_COUNT)`
     - **Completion**: If `Fetch_Stage() == STEP_COUNT`:
       - `IsSuspended = true`, `Set_Rate(0)`
       - `House->Spend_Money(Balance)`, `Balance = 0`

### FactoryClass::Set(type, house) → bool
**Start normal production** - Prepares factory for object type.

1. `Abandon()` any current production
2. `IsDifferent=true`, `IsSuspended=true`, `Rate=0`, `Stage=0`
3. `Object = type.Create_One_Of(&house)` (places in limbo)
4. If success:
   - `House = Object->House`
   - `Balance = type.Cost_Of() * house.CostBias`
   - `Object->PurchasePrice = Balance`
4. Returns `Object != NULL`

### FactoryClass::Set(specialType, house) → bool
**Start special weapon production** (ion cannon).

1. `Abandon()` current
2. `IsDifferent=true`, `IsSuspended=true`, `Rate=0`, `Stage=0`
3. `SpecialItem = type`, `House = &house`, `Balance = 0`
4. Returns `SpecialItem != SPC_NONE`

### FactoryClass::Set(object) - Place completed object
**Accepts pre-built object** (e.g., from placement cancellation).

1. `Abandon()` current
2. `Object = &object`, `House = Object->House`, `Balance = 0`
3. `Rate=0`, `Stage=STEP_COUNT`, `IsDifferent=true`, `IsSuspended=true`

### FactoryClass::Start() → bool
**Resume production** after suspension or new setup.

1. Requires: `(Object || SpecialItem) && IsSuspended && !Has_Completed()`
2. **Funds check**: Human = always OK; AI = needs `Available_Money() >= Cost_Per_Tick()`
3. **Rate calculation**:
   - Normal: `time = Object->Class_Of().Time_To_Buld(house)` ticks
   - Special: `time = TICKS_PER_MINUTE * 5` (5 minutes)
   - `time /= STEP_COUNT`, `Bound(time, 1, 255)`
4. `Set_Rate(time)`, `IsSuspended = false`
5. Returns true if started

### FactoryClass::Suspend() → bool
**Pause production** - Returns true if was running.

### FactoryClass::Abandon() → bool
**Cancel with refund** - Clears factory.

1. If `Object`:
   - `money = Object->Class_Of().Cost_Of() * Object->House->CostBias`
   - `House->Refund_Money(money - Balance)` (spent so far)
   - `ScenarioInit++`, `delete Object`, `ScenarioInit--`, `Object = NULL`
2. If `SpecialItem`: `SpecialItem = SPC_NONE`
3. `Rate=0`, `Stage=0`, `IsSuspended=true`, `IsDifferent=true`
4. Returns true if had object

### FactoryClass::Completed() → bool
**Clear after placement** - Called when player places completed unit.

1. If `Object` at `STEP_COUNT`: `Object=NULL`, `IsSuspended=true`, `IsDifferent=true`, `Stage=0`, `Rate=0` → true
2. If `SpecialItem` at `STEP_COUNT`: `SpecialItem=SPC_NONE`, same flags → true
3. Else false

### FactoryClass::Has_Completed() → bool
**Check if ready for placement** - `Fetch_Stage() == STEP_COUNT` for Object or SpecialItem.

### FactoryClass::Has_Changed() → bool
**UI update flag** - Returns and clears `IsDifferent`.

### FactoryClass::Cost_Per_Tick() → int
**Installment amount** - `Balance / (STEP_COUNT - Fetch_Stage())`, or `Balance` if final step.

### FactoryClass::Completion() → int
**Current step** - Returns `Fetch_Stage()` (0 to STEP_COUNT).

### FactoryClass::Get_Object() → TechnoClass*
**Access building object** - For placement mode.

### FactoryClass::Get_Special_Item() → int
**Access special weapon type**.

### FactoryClass::Force_Complete()
**Debug: Instant finish** - Sets `Stage=STEP_COUNT`, `IsSuspended=true`, `Balance=0`.

### Pool management
- `operator new` → `Factories.Allocate()`, sets `IsActive=true`
- `operator delete` → `IsActive=false`, `Factories.Free()`
- `Init()` → `Factories.Free_All()` (scenario start)
- Destructor → `Abandon()` if `GameActive`

## Inputs, outputs and side effects

**State mutations:**
- House money: `Spend_Money`, `Refund_Money`, `Available_Money`
- Factory pool allocation
- Object creation/deletion (limbo objects)
- Production stage/rate
- UI flag `IsDifferent`

**External systems affected:**
- House economy (credits)
- Object placement (completed units)
- Superweapon readiness (ion cannon)
- UI sidebar (progress bars)

## Calls into / called by

**Called by:**
- Building AI: `BuildingClass::AI()` calls `FactoryClass::AI()`
- UI: `SidebarClass` checks `Has_Changed()`, `Completion()`, `Has_Completed()`
- Player input: `Start()`, `Suspend()`, `Abandon()`, `Completed()`
- Special weapons: `Set(SPC_ION_CANNON)` for superweapon timer
- Scenario load: `Init()`, `Set()` for pre-placed buildings

**Calls into:**
- `TechnoTypeClass::Create_One_Of()` - Creates limbo object
- `TechnoTypeClass::Cost_Of()`, `Time_To_Build()`
- `HouseClass::Spend_Money/Refund_Money/Available_Money/CostBias`
- `HouseClass` factory counts (AircraftFactories, etc)
- `StageClass::Graphic_Logic()` - Tick gate
- `ScenarioInit` counter (save/load guard)
- `Debug_Instant_Build` cheat

## Related original files
- **FACTORY.H** - Class declaration, SPC_* constants
- **BUILDING.H/.CPP** - BuildingClass owns FactoryClass
- **HOUSE.H/.CPP** - HouseClass economy, factory counts
- **TECHNOTYPE.H/.CPP** - TechnoTypeClass cost/time
- **SIDEBAR.H/.CPP** - UI progress display
- **SPECIAL.H/.CPP** - Special weapon types
- **SCENARIO.H/.CPP** - ScenarioInit

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/production/factory.rs` - FactoryClass
- `cnc-tiberian-dawn/src/production/td_factory.rs` - TD-specific types
- `cnc-core/src/production/queue.rs` - Production queue per building
- `cnc-core/src/economy.rs` - Cost installment logic

**Not yet ported:**
- Pooled allocation (`Factories` pool with `IsActive`)
- Multi-factory acceleration (human only, per-type counts)
- Cost installment with rollback on insufficient funds
- Special weapon production (ion cannon timer)
- Placement suspension (Set with completed object)
- `ScenarioInit` guard for save/load
- `Debug_Instant_Build` cheat
- `Force_Complete` debug
- `Has_Changed` UI dirty flag
- `Cost_Per_Tick` dynamic installment calculation

## Edge cases and compatibility notes

1. **Human vs AI acceleration**: Human players get speedup from multiple same-type factories; AI builds per-building (classic C&C balance)
2. **Installment rollback**: If funds insufficient, stage decrements (visual "stutter" in progress bar)
3. **Final step spends remainder**: `House->Spend_Money(Balance)` ensures exact cost
4. **Special items have no cost**: `Balance=0`, fixed 5-minute timer
5. **Placement cancellation**: `Set(TechnoClass&)` puts object back at `STEP_COUNT`, suspended
6. **ScenarioInit guard**: Prevents save/load issues during create/delete
7. **CostBias**: Difficulty modifier (house.CostBias) applied to total cost
8. **PurchasePrice**: Stored on object for sell value calculation
9. **Pool limit**: `FACTORY_MAX` factories total; allocation returns NULL if full
10. **Instant build cheat**: Sets stage to `STEP_COUNT` immediately
11. **Suspended states**: Created suspended; `Start()` required; auto-suspends on completion
12. **Abandon refund**: `money - Balance` = spent so far (total - remaining)
13. **Completed() vs Has_Completed()**: `Completed()` clears factory; `Has_Completed()` only checks
14. **Object in limbo**: Created by `Create_One_Of`, deleted on `Abandon()` or moved on placement
15. **Rate bounds**: `Bound(time, 1, 255)` ticks per step

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/FACTORY.CPP at f1f0d42
