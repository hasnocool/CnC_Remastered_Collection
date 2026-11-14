# PATH/FILE
original/TIBERIANDAWN/LOGIC.CPP

## Purpose
Main game logic scheduler for Tiberian Dawn. Orchestrates per-frame AI processing for all game systems: teams, units/infantry/aircraft/buildings, houses, factories, map, and crates. Implements the central `LogicClass::AI()` loop called once per frame.

## Where it fits in the game
Central game loop coordinator - Global `Logic` instance (GLOBALS.CPP). Called once per frame from main loop. Sequences all AI subsystems in deterministic order for lockstep multiplayer synchronization.

## Classes, structs, enums and important data

### LogicClass
**Frame logic scheduler** - No persistent state beyond frame counters.

**Global instance:** `Logic` (GLOBALS.CPP)

**Static frame counters (CHEAT_KEYS):**
- `FramesPerSecond` - Current frame count
- `TotalFrames` - Lifetime frame total
- `FPSDivider` - Running average divisor
- `AverageFramesPerSecond` - Running average FPS

### LogicClass::AI() execution order
**Per-frame sequence (deterministic for lockstep):**

1. **Crate regeneration** (MP only):
   - If `CrateMaker && CrateTimer.Expired()`: `Map.Place_Random_Crate()`, resets timer (7-15 min)

2. **Team AI**: `Teams.Ptr(i)->AI()` for all teams

3. **Object AI pass 1**: All objects in `Logic` array
   - Calls `obj->AI()`
   - **Deletion-safe iteration**: Tracks `Count()` before/after, adjusts index if objects deleted

4. **Scan bit update pass 2** (fog of war / shroud):
   - Units → `House->NewUScan/ActiveUScan` bitmask by type
   - Infantry → `NewIScan/ActiveIScan`
   - Aircraft → `NewAScan/ActiveAScan`
   - Buildings → `NewBScan/ActiveBScan`
   - Condition: `IsLocked && (MP || !Human || DiscoveredByPlayer)`

5. **Map logic**: `Map.Logic()`

6. **Factory AI**: `Factories.Ptr(i)->AI()` for all factories

7. **House AI** (conditional):
   - **Single player**: All houses `HOUSE_FIRST..HOUSE_COUNT`
   - **Multiplayer**: Only `HOUSE_MULTI1..HOUSE_COUNT` + `HOUSE_NEUTRAL` + `HOUSE_JP`
   - Calls `hptr->AI()` for active houses

8. **Clear recently created bits**: `obj->IsRecentlyCreated = false` for all objects

### Debug_Dump(mono) (CHEAT_KEYS)
**Monochrome performance monitor** - Called once/second.

**Displays:**
| Metric | Source |
|--------|--------|
| Frame | `Frame` |
| Units/Infantry/Aircraft/Buildings/Terrains/Bullets/Anims/Teams/Triggers/Factories | Pool `.Count()` |
| Frame Rate | `FramesPerSecond` (reset each call) |
| Avg FPS | `AverageFramesPerSecond` |
| Spare CPU | Bar graph (13-frame history, 40 columns) |

**Spare CPU calculation:** `Fixed_To_Cardinal(RECORDHEIGHT, Cardinal_To_Fixed(TIMER_SECOND, SpareTicks))`

## Constants
| Constant | Value | Purpose |
|----------|-------|---------|
| `RECORDCOUNT` | 40 | Performance history width |
| `RECORDHEIGHT` | 21 | Bar graph height |
| `TIMER_SECOND` | (ticks) | Ticks per second |
| `TICKS_PER_MINUTE` | 15×60 | Crate timer base |

## Functions and methods

### LogicClass::AI()
**Main per-frame entry point** - Called from game loop.

**Key implementation details:**
- **Deletion-safe iteration**: Captures `Count()` before `AI()`, adjusts index if `Count()` decreased
- **Two-pass object AI**: First pass runs behavior; second updates house scan bits
- **Conditional house AI**: MP vs SP different house ranges
- **Crate timer**: `TICKS_PER_MINUTE * Random_Pick(7,15)` (7-15 minutes)

### LogicClass::Debug_Dump(mono) (CHEAT_KEYS)
**Monochrome monitor output** - Called once/second from main loop.

**Performance tracking:**
- `TotalFrames += FramesPerSecond`
- `AverageFramesPerSecond = TotalFrames / FPSDivider++`
- `SpareTicks` → bar graph via `Fixed_To_Cardinal`
- `_record[40]` circular buffer of spare CPU history

### LogicClass::Clear_Recently_Created_Bits()
**Post-frame cleanup** - Resets `IsRecentlyCreated` flag on all objects.

## Multiplayer vs Single Player differences

| System | Single Player | Multiplayer |
|--------|---------------|-------------|
| House AI | All houses (FIRST..COUNT) | MULTI1..COUNT + NEUTRAL + JP |
| Crate regen | Disabled | Enabled (CrateMaker) |
| House paranoia | Immediate | Delayed (added 2019) |

## Inputs, outputs and side effects

**State mutations (per frame):**
- `FramesPerSecond++`
- Object `AI()` mutations (position, state, targets)
- House scan bits (`New*Scan`, `NewActive*Scan`)
- Factory production progress
- Team AI state
- Map logic (shroud, radar, etc.)
- `IsRecentlyCreated` cleared

**External systems affected:**
- All unit/building/infantry/aircraft AI
- House AI (production, attacks, repairs)
- Team coordination
- Factory production
- Map shroud/radar
- Crate spawning

## Calls into / called by

**Called by:**
- Main game loop (once per frame)

**Calls into (in order):**
1. `Map.Place_Random_Crate()` (MP crates)
2. `TeamClass::AI()` (all teams)
3. `ObjectClass::AI()` (all objects - pass 1)
4. `ObjectClass::AI()` pass 2 → House scan bits
5. `MapClass::Logic()`
6. `FactoryClass::AI()` (all factories)
6. `HouseClass::AI()` (conditional houses)
7. `ObjectClass::IsRecentlyCreated = false` (all objects)

**Debug only:**
- `MonoClass` output
- `Fixed_To_Cardinal` / `Cardinal_To_Fixed` (spare CPU bars)
- Pool `.Count()` calls

## Related original files
- **LOGIC.H** - Class declaration
- **GLOBALS.CPP** - `LogicClass Logic;` global
- **TEAM.H/.CPP** - TeamClass
- **OBJECT.H/.CPP** - ObjectClass base
- **UNIT.H/.CPP** - UnitClass
- **INFANTRY.H/.CPP** - InfantryClass
- **AIRCRAFT.H/.CPP** - AircraftClass
- **BUILDING.H/.CPP** - BuildingClass
- **HOUSE.H/.CPP** - HouseClass (AI, scan bits)
- **FACTORY.H/.CPP** - FactoryClass
- **MAP.H/.CPP** - MapClass::Logic()
- **CRATE.H/.CPP** - Crate spawning
- **GLOBALS.CPP** - Object pools

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/game_logic.rs` - LogicClass::AI() scheduler
- `cnc-tiberian-dawn/src/game/td_logic.rs` - TD-specific order
- `cnc-core/src/ai/team.rs` - TeamClass::AI()
- `cnc-core/src/ai/house.rs` - HouseClass::AI()
- `cnc-core/src/ai/factory.rs` - FactoryClass::AI()
- `cnc-core/src/debug/monitor.rs` - Debug_Dump equivalent

**Not yet ported:**
- Deterministic execution order (critical for lockstep)
- Deletion-safe iteration pattern
- Two-pass object AI (behavior + scan bits)
- MP vs SP house AI branching
- Crate timer with `Random_Pick`
- Monochrome debug monitor (replace with logging)
- Spare CPU tracking via `SpareTicks`
- `HouseClass::Recalc_Attributes()` (RA AI integration)

## Edge cases and compatibility notes

1. **Deletion-safe iteration**: Critical for lockstep - captures `Count()` before `AI()`, adjusts index if objects destroyed
2. **Two-pass object AI**: First pass = behavior; second = scan bit updates (fog of war)
3. **MP house AI restriction**: Only human/active MP houses + neutral + JP (prevents instant AI paranoia)
4. **Crate timer**: 7-15 minutes random, MP only
5. **Frame counter**: `FramesPerSecond` incremented once per `AI()` call
6. **SpareTicks**: Accumulated elsewhere, consumed here for debug graph
7. **Debug_Dump timing**: Assumes once/second call (not per-frame)
8. **IsRecentlyCreated**: Cleared post-frame for "new unit" detection
9. **Deterministic order**: Critical for multiplayer lockstep sync
10. **House AI conditional**: `#if 1` block with MP/SP split (added 2019)

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/LOGIC.CPP at f1f0d42
