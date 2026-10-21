# RUST/cnc-red-alert/src/

## Responsibility

Fourteen modules declared in `lib.rs` covering the RA simulation; the bulk of logic lives in `game.rs`; each feature module owns one Westwood subsystem port.

## Design

- `game.rs` — the simulation loop and command surface (`Game`, `GameCommand`, `GameUnit`, `GameStructure`, `TargetRef`, `Shot`, `AiCommander`, `RunningTrigger`, `ObjectiveState`, `ScenarioOutcome`, `BuildError`); ties houses/map/combat/production together; carries a big unit-test suite inline (BUILD_STATUS 63 + 1 + authoring). `Game.rng` is the seeded simulation stream; the compact text save (`RA_SAVE` v2) persists its seed and cursor, so a reload resumes the same stream.
- `rng.rs` (182 L) — `SimRng`: seeded, serialisable SplitMix64 simulation RNG (pinned algorithm shared with `cnc-procgen::prng`). Owns every random draw on a simulation path (team retreat scatter, per-unit scatter sub-streams); presentation code must not consume it.
- `house.rs` (260 L) — `House`, `Faction` (Allied/Soviet), `Country` (5 names e.g. `Country::England`), `ProductionItem`; treasury/credits/power (`low_power()`), `can_build_unique_structure(ClassId::GapGenerator)` unique-structure gate.
- `scenario.rs` (400 L) — INI dialect loader into `Scenario`/`ScenarioObject`/`ScenarioTrigger` (matching the Mobius-editor contract).
- `superweapon.rs` (292 L) — `Superweapon` charging trait/enum: `Chronosphere`, `IronCurtain` (+ `ArmoredUnit::harden` invulnerability), `GPS`; `SuperweaponError`.
- `tesla.rs` (173 L) — `TeslaCoil` (charge, power gate, range check, `update`/`is_ready`) + fired `TeslaBolt`.
- `gapgen.rs` (120 L) — `GapGenerator` shroud toggle + power drain.
- `aircraft.rs` (237 L) — `Aircraft` (finite ammo, rearm docking, no fire while parked), `Helipad` (1), `Airfield` (up to 4).
- `naval.rs` (190 L) — `Ship`/`Submarine`: surfaced movement, submerge, targeting rules.
- `mcv.rs` (160 L) — `MCV::begin_deploy/update` unpack flow returning the yard position; `can_deploy_at(closure-blocked map)`, `yard_footprint` — block map passed as a closure for reuse in campaigns.
- `spy.rs` (201 L) — `Spy`/`Thief`, target `InfiltrationTarget` + one-shot `InfiltrationEffect` (credits/sabotage/radar/reveal).
- `pathfind.rs` (252 L) — 8-direction A* with `MoveClass` ground/water/air passability.
- `map.rs` (201 L) — `Map`, `Terrain`, `TILE_SIZE` cell grid.
- `render.rs` (25 L) — `render_entity`/`entity_asset` adapters to renderer+asset system.
- `lib.rs` — re-exports plus inline `#[cfg(test)] mod integration_tests` (RA-specific: `allied_country_can_charge_its_superweapon_and_defend_a_base`, `soviet_mcv_deploys_into_a_yard_and_iron_curtain_protects_it`).

## Flow

Scenario → entities spawned → per-tick: command dispatch → unit/naval/air/mcv/tesla/gap/superweapon updates → pathfinder consults map → combat writes `Shot`s/effects → triggers evaluated → status/outcome. Random picks along that path advance `Game.rng`; rendering reads entities through `render.rs` adapters and never touches the stream.

## Integration

- Depends on core/renderer/audio/asset crates; used by `cnc-launcher` (gameplay loop) and, at author-time, the editor's `RedAlertPlugin`.
- No other workspace crate depends on RA — engines are leaves consumed by the launcher.
