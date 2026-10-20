# RUST/cnc-red-alert/tests/

## Responsibility

Single integration test file, `campaign_scenarios.rs` (61 lines): pins that the authored Red Alert campaign scenarios remain loadable end-to-end.

## Design

Single test `authored_ra_campaign_scenarios_load()` (module doc: "The authored RA campaign scenarios must stay loadable") — enumerates the authored RA campaign scenario INIs (shared with the map editor's output dialect) and asserts each parses into `cnc_red_alert::scenario::Scenario` without error.

## Flow

Load scenario file(s) → `Scenario` parse → assert no validation errors; guards against editor-side INI dialect drift breaking the game's scenario loader.

## Integration

Covers the `cnc-red-alert/src/scenario.rs` parse path and its INI dialect contract with `cnc-map-editor` (the editor writes what the game must read); RA unit coverage (63 tests incl. superweapon/MCV/tesla/naval/air/spy cases) lives inline in `src/` and `lib.rs` `integration_tests`, per BUILD_STATUS.
