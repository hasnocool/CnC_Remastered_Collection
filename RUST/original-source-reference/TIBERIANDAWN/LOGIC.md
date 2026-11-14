# PATH/FILE
original/TIBERIANDAWN/LOGIC.CPP

## Purpose
Core game logic loop for Tiberian Dawn. Manages the per-frame AI processing for all game objects (houses, teams, units, infantry, aircraft, buildings, factories) and map logic. The central "tick" function called once per game frame.

## Where it fits in the game
Central game loop - LogicClass::AI() is called once per game frame. It coordinates AI for all game systems: houses (players), teams, sentient objects (units/infantry/aircraft/buildings), factories, and the map (tiberium growth).

## Classes, structs, enums and important data

### LogicClass
**Core game loop controller** - singleton managing per-frame AI updates.

**Key data members:**
- `FramesPerSecond` (unsigned) - Current frame rate
- `TotalFrames` (unsigned, CHEAT_KEYS) - Total frames since start
- `FPSDivider` (unsigned, CHEAT_KEYS) - Frame counter for averaging
- `AverageFramesPerSecond` (unsigned, CHEAT_KEYS) - Running average FPS
- `SpareTicks` - CPU time remaining in frame

### LogicClass static data (CHEAT_KEYS)
- `FramesPerSecond` - Current frame rate
- `TotalFrames` - Total frames
- `FPSDivider` - Frame counter for averaging
- `AverageFramesPerSecond` - Running average

### Global constants
- `RECORDCOUNT = 40` - Performance graph width
- `RECORDHEIGHT = 21` - Performance graph height
- `TICKS_PER_MINUTE` - Ticks per minute (for crate timer)
- `TICKS_PER_SECOND` - Ticks per second

### Crate system globals
- `CrateMaker` (bool) - Crate spawner active
- `CrateTimer` - Timer for next crate spawn

## Functions and methods

### LogicClass::AI() - **Main game loop** (called once per frame)

**Execution order:**
1. **Frame counter**: `FramesPerSecond++`
2. **Crate regeneration** (MP only):
   - If `CrateMaker && CrateTimer.Expired()`: `Map.Place_Random_Crate()`, resets timer to 7-15 minutes
3. **Team AI**: Iterates `Teams[]`, calls `TeamClass::AI()` for each
4. **Sentient object AI** (first pass):
   - Iterates `LogicClass::Count()` objects via `(*this)[index]->AI()`
   - Handles object deletion during AI: adjusts index if count decreases
4. **Scan bit update** (second pass):
   - Updates per-house scan bitmasks for discovered/active objects
   - Units: `NewUScan` / `NewActiveUScan` (locked, discovered/owned)
   - Infantry: `NewIScan` / `NewActiveIScan`
   - Aircraft: `NewAScan` / `NewActiveAScan`
   - Buildings: `NewBScan` / `NewActiveBScan`
   - Only for locked objects, MP/human/discovered conditions
5. **Map logic**: `Map.Logic()` - tiberium growth/spread
5. **Factory AI**: Iterates `Factories[]`, calls `FactoryClass::AI()`
6. **House AI** (conditional):
   - **MP** (`GameToPlay != GAME_NORMAL`): Only active houses `HOUSE_MULTI1` to `HOUSE_COUNT`, plus neutral (`HOUSE_NEUTRAL`) and JP (`HOUSE_JP`)
   - **SP** (`GAME_NORMAL`): All houses `HOUSE_FIRST` to `HOUSE_COUNT`
   - Calls `HouseClass::AI()` for each active house

**LogicClass::AI()** - **Main per-frame logic**
1. `FramesPerSecond++`
2. **Crate regen** (MP): if `CrateMaker && CrateTimer.Expired()` → `Map.Place_Random_Crate()`, reset timer 7-15 min
5. **Team AI**: `Teams[].AI()`
6. **Object AI pass 1**: All objects `AI()`, handles deletion during iteration
5. **Scan bit update** (pass 2):
   - Units: `NewUScan`/`NewActiveUScan` (locked + MP/human/discovered)
   - Infantry: `NewIScan`/`NewActiveIScan`
   - Aircraft: `NewAScan`/`NewActiveAScan`
   - Buildings: `NewBScan` / `NewActiveBScan`
   - Conditions: locked + (MP || !human || discovered)
6. **Map logic**: `Map.Logic()` (tiberium growth/spread)
5. **Factory AI**: `Factories[].AI()`
6. **House AI** (conditional):
   - MP: active houses `HOUSE_MULTI1`..`HOUSE_COUNT` + NEUTRAL + JP
   - SP: all houses `HOUSE_FIRST`..`HOUSE_COUNT`
   - Calls `HouseClass::AI()`

**LogicClass::Clear_Recently_Created_Bits()** - Clears `IsRecentlyCreated` flag on all objects

### LogicClass::Debug_Dump() (CHEAT_KEYS)
- Displays: Frame count, object counts (Units, Infantry, Aircraft, Buildings, Terrains, Bullets, Anims, Teams, Triggers, Factories)
- FPS counter and average
- Performance graph (40 columns x 21 rows) showing spare CPU time

## Inputs, outputs and side effects

**State mutations:**
- House scan bitmasks (`NewUScan`, `NewIScan`, `NewAScan`, `NewBScan`)
- Crate timer (`CrateTimer`, `CrateMaker`)
- Object AI state mutations (via `obj->AI()`)
- House AI state (via `HouseClass::AI()`)

**External systems affected:**
- All game objects via `AI()` calls
- Map tiberium system via `Map.Logic()`
- House AI systems
- Team AI systems
- Factory production

## Calls into / called by

**Called by:**
- Main game loop: once per frame

**Calls into:**
- `Teams[]->AI()` - Team AI
- All objects: `obj->AI()`
- `Map.Logic()` - Tiberium growth/spread
- `Factories[]->AI()` - Factory production
- `HouseClass::AI()` - House AI (conditional)
- `HouseClass::Recalc_Attributes()` (RA AI)
- `Map.Logic()` - Tiberium growth/spread
- `Factories[]->AI()` - Factory production
- `HouseClass::AI()` - House AI (conditional)
- `HouseClass::Recalc_Attributes()` (RA AI)

## Related original files
- **LOGIC.H** - Class declaration
- **HOUSE.CPP/.H** - HouseClass AI
- **TEAM.CPP/.H** - TeamClass AI
- **OBJECT.CPP/.H** - ObjectClass AI
- **UNIT.CPP/.H** - UnitClass AI
- **INFANTRY.CPP/.H** - InfantryClass AI
- **AIRCRAFT.CPP/.H** - AircraftClass AI
- **BUILDING.CPP/.H** - BuildingClass AI
- **TEAM.CPP/.H** - TeamClass AI
- **FACTORY.CPP/.H** - FactoryClass AI
- **MAP.CPP/.H** - MapClass::Logic()
- **FUNCTION.H** - Core includes

## Rust port mapping
**Crates:** `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-core/src/game/logic.rs` - LogicClass main loop
- `cnc-core/src/game/tick.rs` - Per-frame tick
- `cnc-tiberian-dawn/src/game/td_logic.rs` - TD-specific logic
- `cnc-core/src/game/scan.rs` - Scan bitmask updates
- `cnc-core/src/game/crate.rs` - Crate regeneration

**Not yet ported:**
- Full per-frame AI dispatch
- Two-pass object AI (execution + scan bits)
- Crate regeneration (MP only)
- Per-house AI conditional (SP vs MP)
- Scan bitmask updates (UScan, IScan, AScan, BScan)
- Factory AI dispatch
- Performance monitoring (CHEAT_KEYS)

## Edge cases and compatibility notes

1. **Two-pass object AI**: First pass executes AI (may delete objects), second pass updates scan bits. Index adjustment handles deletions during first pass.

2. **MP vs SP house AI**: MP only processes `HOUSE_MULTI1`+`HOUSE_NEUTRAL`+`HOUSE_JP`; SP processes all houses.

2. **Two-pass**: First pass runs AI (may delete objects), second pass updates scan bits. Index adjustment handles deletions.

3. **Crate regeneration**: MP only. `CrateTimer` expires → `Map.Place_Random_Crate()`, timer reset to 7-15 minutes.

4. **House AI conditional**: MP only processes multiplayer houses + neutral + JP. SP processes all.

4. **Scan bitmasks**: Per-house bitmasks for units/infantry/aircraft/buildings. Updated for locked, discovered/owned objects.

4. **Scan bits**: Per-house bitmasks (UScan, IScan, AScan, BScan) for locked, discovered/owned objects.

5. **Frame counter**: `FramesPerSecond` incremented each call. `SpareTicks` tracks CPU budget.

5. **Frame counter**: `FramesPerSecond++` per call. `SpareTicks` tracks CPU budget.

6. **Factory AI**: Called after map logic, before house AI.

6. **Factory AI**: Called between map and house AI.

7. **RA AI integration**: `HouseClass::Recalc_Attributes()` called if `USE_RA_AI`.

7. **RA AI**: `HouseClass::Recalc_Attributes()` if `USE_RA_AI`.

8. **Debug dump**: CHEAT_KEYS only. Shows object counts, FPS, performance graph.

8. **Debug dump**: CHEAT_KEYS only. Object counts, FPS, performance graph.

9. **Two-pass design**: Critical for handling object deletion during AI. First pass executes AI (may delete objects), second pass updates scan bits with adjusted index.

9. **Two-pass**: First pass runs AI (may delete), second pass updates scan bits with index adjustment.

10. **House AI conditional**: MP vs SP determines which houses get AI processing.

10. **House AI conditional**: MP processes multiplayer houses + neutral + JP; SP processes all.

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/LOGIC.CPP at f1f0d42
