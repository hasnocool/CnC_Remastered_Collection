# RUST/cnc-red-alert/tests/

## Responsibility
Integration tests for Red Alert campaign scenario loading and deterministic RNG validation.

## Design
- campaign_scenarios.rs (2,023 lines): Pins that authored Red Alert campaign scenarios remain loadable end-to-end. Test authored_ra_campaign_scenarios_load() enumerates authored RA campaign scenario INIs and asserts each parses into cnc_red_alert::scenario::Scenario without error.
- rng_determinism.rs (17,347 lines): Validates deterministic RNG behavior across simulation runs. Tests that identical seeds produce bit-identical game states, RNG stream isolation per system (combat, movement, AI), and replay consistency.

## Flow
- Campaign: Load scenario file(s) -> Scenario parse -> assert no validation errors; guards against editor-side INI dialect drift breaking the game's scenario loader.
- RNG: Construct Game with fixed seed -> run N ticks -> snapshot state -> repeat with same seed -> assert bit-identical snapshots; tests RNG stream separation (combat vs movement vs AI).

## Integration
- Campaign covers cnc-red-alert/src/scenario.rs parse path and its INI dialect contract with cnc-map-editor
- RNG covers cnc-red-alert/src/rng.rs DeterministicRng and its integration with Game, House, Unit, Weapon systems
- RA unit coverage (63+ tests incl. superweapon/MCV/tesla/naval/air/spy cases) lives inline in src/ and lib.rs integration_tests, per BUILD_STATUS.

## Key Files
- campaign_scenarios.rs - Campaign scenario loading tests
- rng_determinism.rs - Deterministic RNG validation tests
