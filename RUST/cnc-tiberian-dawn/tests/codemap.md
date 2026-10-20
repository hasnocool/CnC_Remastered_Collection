# RUST/cnc-tiberian-dawn/tests/

## Responsibility

Two integration test files exercising the TD netcode harness and authored campaign files (BUILD_STATUS: 154 unit + 4 integration; unit tests live inside `src/`).

## Design

- `live_session.rs` (103 L) — module doc: "Live relay + host-migration coverage for the network session harness." Helpers `addr(port)`, `point(x, y)`; tests: `live_session_relays_a_command_once_and_builds_packets` (dedup guarantee), `host_migration_updates_the_elected_host_address` (election result applied to session), `network_config_validates_nat_friendly_settings`.
- `campaign_missions.rs` (46 L) — module doc: "The authored TD campaign missions must stay loadable." Test `authored_td_campaign_missions_parse()`.

## Flow

Live-session tests construct a `LiveSession`/`MeshRelay` over injected socket addresses, emit commands, and assert relay/migration state transitions without real network I/O beyond loopback; campaign test enumerates authored mission INIs (`assets/missions/gdi_01.ini`, `../maps/`) and asserts `Mission` parse success.

## Integration

Covers `cnc-tiberian-dawn/src/network.rs` (relay, `HostMigration`, `NetworkConfig`) and `mission.rs` loaders, enforcing the launcher-facing contracts: sessions must survive migration, commands must dedup, mission files must remain editable-loadable.
