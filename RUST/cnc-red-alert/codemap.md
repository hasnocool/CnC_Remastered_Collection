# RUST/cnc-red-alert/

## Responsibility
Red Alert game logic implementation in Rust. Deterministic lockstep simulation compatible with original Westwood RA. Includes all RA-specific units (naval, aircraft, Tesla, Gap Generator, MCV, Spy, Superweapons), house/faction logic, scenario system, pathfinding, and network stack.

## Design Patterns
- **Deterministic Simulation**: Fixed timestep (`Fixed16_16`), lockstep-safe, no RNG in sim (deterministic RNG via `rng.rs`)
- **Entity Component Style**: `Unit` (Infantry/Vehicle/Aircraft/Naval), `Building`, `Projectile` with `ClassId` RTTI
- **House/Faction System**: `house.rs` - `House`, `HouseKind` (Allies/Soviet), production, AI, sidebar
- **Scenario System**: `scenario.rs` - INI-driven missions/skirmish, `scenario_mpr.rs` - MPR map format
- **Pathfinding**: `pathfind.rs` - A* on `CellGrid`, clearance-based, naval/air/ground separate
- **Naval Units**: `naval.rs` - ships, submarines, naval yards, shore logic
- **Aircraft**: `aircraft.rs` - helicopters, jets, airfield, ammo/reload
- **Gap Generator**: `gapgen.rs` - shroud regeneration radius
- **MCV**: `mcv.rs` - Mobile Construction Vehicle, deploy/undeploy
- **Spy**: `spy.rs` - infiltration, sabotage, disguise
- **Tesla**: `tesla.rs` - Tesla coil, Tesla tank, arc logic
- **Superweapons**: `superweapon.rs` - Iron Curtain, Chronosphere, Nuke, GPS Satellite
- **Network Stack**: `network.rs` (in TD, but RA uses similar) - `CommandQueue`, `LockstepSession`, `UdpTransport`, `HandshakeMessage`, `HostMigration`, `MeshRelay`, `LiveSession`, `Replay`
- **Events**: `events.rs` - `cnc-events` integration for gameplay events
- **Rules Integration**: Reads `cnc_core::rules::RulesDatabase` for costs, tech tree, prerequisites
- **Original Scenario**: `original_scenario.rs` - legacy mission format support
- **Replay**: `replay.rs` - command stream recording/playback

## Data & Control Flow
1. **Scenario Load**: `Scenario::from_ini()` or `from_mpr()` → `Scenario` with `Map`, `PlayerStarts`, `Objects`, `Triggers`
2. **Game Init**: `Game::new(scenario)` → spawns `House` per player, initializes `CellGrid`, `CommandQueue`
3. **Tick Loop**: `Game::tick(delta)` → 
   - `CommandQueue` processes local + network commands
   - `House::think()` - AI production, targeting, harassment
   - `Unit/Building::update()` - movement, combat, construction
   - `pathfind` routes units on `CellGrid`
   - `Projectile` simulation, `Warhead` application
   - `Trigger` evaluation → `MissionOutcome`
4. **Network**: `LockstepSession` relays `Command`s, handles `HostMigration`, `MeshRelay`
5. **Save/Replay**: `GameSnapshot` serialization, `Replay` command stream

## Public API
- **game**: `Game`, `GameStatus`, `Buildable`, `SidebarOption`
- **house**: `House`, `HouseKind`, `ProductionQueue`
- **map**: `Map`, `CellGrid`
- **unit**: `Unit`, `Infantry`, `Vehicle`, `Aircraft`, `Naval`, `Stance`
- **building**: `Building`, `Factory`, `PowerPlant`, `Refinery`, `Barracks`, `WarFactory`, `NavalYard`, `Airfield`, `TeslaCoil`, `GapGenerator`, `Superweapon`
- **weapon**: `Weapon`, `Projectile`, `Warhead`
- **aircraft**: `Aircraft`, `Airfield`, `AmmoType`
- **naval**: `NavalUnit`, `Ship`, `Submarine`, `NavalYard`
- **gapgen**: `GapGenerator`
- **mcv**: `MCV`, `DeployState`
- **spy**: `Spy`, `InfiltrationTarget`
- **tesla**: `TeslaCoil`, `TeslaTank`, `ArcTarget`
- **superweapon**: `Superweapon`, `SuperweaponType` (IronCurtain, Chronosphere, Nuke, GPSSatellite)
- **scenario**: `Scenario`, `ScenarioLoader`, `from_ini`, `from_mpr`
- **scenario_mpr**: `MprScenario`, `load_mpr`
- **pathfind**: `find_path`, `PathRequest`, `Clearance`
- **rng**: `DeterministicRng`, `RngStream`
- **events**: `publish_combat_event`, `publish_construction_event`, etc.
- **replay**: `Replay`, `ReplayFrame`, `record_replay`, `playback_replay`
- **original_scenario**: `OriginalScenario`, `load_original`

## Integration Points
- **Dependencies**: cnc-core, cnc-renderer, cnc-audio, cnc-asset, cnc-events, cnc-logs, anyhow, thiserror, serde, parking_lot, once_cell, crossbeam-channel, dashmap, smallvec
- **Consumed by**: cnc-launcher (RA game mode), cnc-map-editor (RedAlertPlugin), cnc-bench (throughput bench), cnc-classic (classic mode)
- **Network**: Compatible with `cnc-net-directory` matchmaking
- **Rules**: `cnc_core::rules::RulesDatabase` for all unit/weapon/building stats

## Key Files
- src/lib.rs - Module declarations, comprehensive re-exports
- src/game.rs - Game, GameStatus, tick loop, command processing
- src/house.rs - House, HouseKind, production, AI
- src/map.rs - Map, CellGrid
- src/unit.rs - Unit hierarchy (Infantry, Vehicle, Aircraft, Naval)
- src/building.rs - Building types, Factory, PowerPlant, Refinery, Barracks, WarFactory, NavalYard, Airfield, TeslaCoil, GapGenerator, Superweapon
- src/weapon.rs - Weapon, Projectile, Warhead
- src/aircraft.rs - Aircraft, Airfield, ammo
- src/naval.rs - Naval units, NavalYard, shore logic
- src/gapgen.rs - Gap Generator shroud logic
- src/mcv.rs - MCV deploy/undeploy
- src/spy.rs - Spy infiltration, sabotage
- src/tesla.rs - Tesla coil, Tesla tank, arc
- src/superweapon.rs - Iron Curtain, Chronosphere, Nuke, GPS Satellite
- src/scenario.rs - Scenario, INI loading
- src/scenario_mpr.rs - MPR map format support
- src/pathfind.rs - A* pathfinding, clearance
- src/rng.rs - Deterministic RNG
- src/events.rs - cnc-events integration
- src/replay.rs - Replay recording/playback
- src/original_scenario.rs - Legacy mission format
- src/render.rs - Render data preparation for cnc-renderer

