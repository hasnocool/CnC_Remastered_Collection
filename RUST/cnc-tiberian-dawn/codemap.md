# RUST/cnc-tiberian-dawn/

## Responsibility
Tiberian Dawn (C&C1) game logic implementation in Rust. Deterministic lockstep simulation compatible with original Westwood TD. Includes TD-specific units, buildings, carryover system, powerups, mission/campaign system, savegames, and network stack.

## Design Patterns
- **Deterministic Simulation**: Fixed timestep (`Fixed16_16`), lockstep-safe, float-bias values (`FloatBias`) for TD physics
- **Entity Hierarchy**: `Unit` (Infantry/Vehicle/Aircraft), `Building` with `ClassId` RTTI (`cnc-core`)
- **House/Faction**: `house.rs` - `House`, GDI/Nod, production, AI (`ai.rs`)
- **Buildings**: `building.rs` - `Factory`, `PowerPlant`, `Refinery`, `Barracks`, `WarFactory`, `Airstrip`, `Helipad`, `Silo`, `Temple`, `AdvancedCommCenter`
- **Units**: `unit.rs` - `Infantry`, `Vehicle`, `Aircraft`, `DoorState` (for transports), `Stance`
- **Weapons**: `weapon.rs` - `Weapon`, `Projectile`, `Warhead`, `WeaponType`, `WarheadType`, `ArmorType`
- **Carryover**: `carryover.rs` - `Carryover` system for campaign persistence (units, credits, tech)
- **Powerups**: `powerup.rs` - `Crate`, `PowerupKind` (credits, unit, power, armor, firepower, speed), `PowerupState`
- **Mission System**: `mission.rs` - `Mission`, `Objective`, `Trigger`, `Condition`, `Action`, `Comparison`, `Script`, `MissionOutcome`
- **Scenario Loader**: `scenario_loader.rs` - INI-driven mission/skirmish loading
- **TD Rules**: `td_rules.rs` - `Theater` (Temperate/Arctic/Desert), `UnitType`, `StructureType`, `ArmorType`, `WarheadType`, `WeaponType`
- **Savegames**: `savegame.rs` - `GameSnapshot`, `SAVE_MAGIC`, `SAVE_VERSION`, pointer fixup via `CodePointers`
- **Network**: `network.rs` - `CommandQueue`, `LiveSession`, `LiveEvent`, `NetworkConfig`, `PeerHandshake`, `Replay`
- **Pathfinding**: `pathfind.rs` - A* on `CellGrid`, clearance-based
- **Events**: `events.rs` - `cnc-events` integration
- **AI**: `ai.rs` - `AI`, `AIDifficulty`, `ProductionWeight`, deterministic via caller-supplied roll

## Data & Control Flow
1. **Scenario Load**: `scenario_loader::load()` → `Scenario` with `Map`, `PlayerStarts`, `Objects`, `Triggers`, `Carryover`
2. **Game Init**: `Game::new(scenario)` → spawns `House` per player, initializes `CellGrid`, `CommandQueue`, `Carryover`
3. **Tick Loop**: `Game::tick(delta)` →
   - `CommandQueue` processes local + network commands
   - `House::think()` - AI production planning (reads `RulesDatabase`), targeting
   - `Unit/Building::update()` - movement, combat, construction, powerup collection
   - `pathfind` routes on `CellGrid`
   - `Projectile` simulation, `Warhead` application
   - `Trigger` evaluation → `MissionOutcome` (win/loss)
   - `Carryover` updates for campaign persistence
4. **Save/Load**: `GameSnapshot` serialization with `CodePointers` fixup, `SAVE_MAGIC`/`SAVE_VERSION` header
5. **Network**: `LockstepSession` (via `cnc-red-alert` network types), `Command` relay, `Replay` recording
6. **Campaign**: `Mission` sequence with `Carryover` transfer between missions

## Public API
- **game**: `Game`, `GameStatus`, `Buildable`, `SidebarOption`, `SubCellError`
- **house**: `House`
- **map**: `Map`, `CellGrid`
- **unit**: `Unit`, `Infantry`, `Vehicle`, `Aircraft`, `DoorState`, `Stance`
- **building**: `Building`, `Factory`, `PowerPlant`, `Refinery`, `Barracks`, `WarFactory`, `Airstrip`, `Helipad`, `Silo`, `Temple`, `AdvancedCommCenter`
- **weapon**: `Weapon`, `Projectile`, `Warhead`, `WeaponType`, `WarheadType`, `ArmorType`
- **carryover**: `Carryover`
- **powerup**: `Crate`, `PowerupKind`, `PowerupState`
- **mission**: `Mission`, `Objective`, `Trigger`, `Condition`, `Action`, `Comparison`, `Script`, `MissionOutcome`
- **scenario_loader**: `load_scenario`, `load_mission`
- **td_rules**: `Theater`, `UnitType`, `StructureType`, `ArmorType`, `WarheadType`, `WeaponType`
- **savegame**: `GameSnapshot`, `SAVE_MAGIC`, `SAVE_VERSION`
- **network**: `CommandQueue`, `LiveSession`, `LiveEvent`, `NetworkConfig`, `PeerHandshake`, `Replay`
- **pathfind**: `find_path`, `PathRequest`, `Clearance`
- **events**: `event_names`, `OutcomeKind`, `SuperweaponKind`
- **ai**: `AI`, `AIDifficulty`, `ProductionWeight`

## Integration Points
- **Dependencies**: cnc-core, cnc-renderer, cnc-audio, cnc-asset, cnc-events, cnc-logs, anyhow, thiserror, serde, parking_lot, once_cell, crossbeam-channel, dashmap, smallvec
- **Consumed by**: cnc-launcher (TD game mode, `Buildable` sidebar, `draw_td_briefing`), cnc-map-editor (TiberianDawnPlugin), cnc-bench (throughput bench), cnc-classic (classic mode)
- **Network**: Compatible with `cnc-net-directory` matchmaking
- **Rules**: `cnc_core::rules::RulesDatabase` for unit/weapon/building stats (with hardcoded TD fallback tables)
- **Assets**: `assets/missions/gdi_01.ini`, `../maps/` campaigns pinned in tests

## Key Files
- src/lib.rs - Module declarations, comprehensive re-exports
- src/game.rs - Game, GameStatus, tick loop, command processing
- src/house.rs - House, GDI/Nod, production
- src/unit.rs - Unit hierarchy (Infantry, Vehicle, Aircraft)
- src/building.rs - TD building types (Factory, PowerPlant, Refinery, Barracks, WarFactory, Airstrip, Helipad, Silo, Temple, AdvancedCommCenter)
- src/weapon.rs - Weapon, Projectile, Warhead, type enums
- src/carryover.rs - Campaign carryover system
- src/powerup.rs - Crates, powerup kinds, state
- src/mission.rs - Mission, Objective, Trigger, Condition, Action, Script, MissionOutcome
- src/scenario_loader.rs - Scenario/mission INI loading
- src/td_rules.rs - Theater, UnitType, StructureType, ArmorType, WarheadType, WeaponType
- src/savegame.rs - GameSnapshot, SAVE_MAGIC, SAVE_VERSION, pointer fixup
- src/network.rs - CommandQueue, LiveSession, PeerHandshake, Replay
- src/pathfind.rs - A* pathfinding
- src/events.rs - cnc-events integration
- src/ai.rs - AI, ProductionWeight, deterministic rolls

