# RUST/cnc-net-directory/src/

## Responsibility

Thirteen modules + `discovery/` submodule directory and the `cnc-directory` main: registry, gossip protocol, peer table, discovery, torrent fallback, HTTP API+UI, LFG matchmaking, and the CLI `main.rs` (1144 L, subcommands: `run`, plus `Torrent {}` / `Lfg {}` variants per its doc "run — host a directory node (gossip + HTTP API + discovery)").

## Design

- `model.rs` (656 L) — shared records for protocol/registry/API: `GameId`, `GameInfo`, `GameKind`, `GameState`, `GameFilter`, `PublishGame`, `NodeId`, `NodeInfo`, `now_ms`.
- `registry.rs` (1004 L) — `Directory` + `MergeStats`: LWW convergence on `(updated_at_ms, origin, revision)`, tombstone withdrawals, TTL expiry.
- `protocol.rs` (304 L) — datagram framing + bincode payloads; idempotent by design.
- `peer.rs` (419 L) — peer table with liveness counters (inbound datagram revives, probes escalate: 2 suspect → 4 dead; routing around corpses).
- `node.rs` (741 L) — `NodeSettings`/`NodeHandle`, `run_forever`, gossip + peer maintenance + discovery + optional API.
- `api.rs` (759 L) — axum router: read-mostly endpoints (two writes: publish, withdraw; heartbeats), built-in browser UI, CORS/trace/timeout/gzip middleware.
- `matchmaking.rs` (1736 L) — `LfgService` swing component: profiles, per-title MMR, leagues, queues pairing comparable skill, settled results moving ratings, AI-only pool separation.
- `torrent.rs` (802 L) — recover node list from web-seed JSON or qBittorrent download; `bencode.rs` (568 L) strict codec + `.torrent` builder/info-hash.
- `client.rs` (531 L) — `DirectoryClient` with `first_available` multi-node failover.
- `error.rs` (202 L) — `DirectoryError`/`Result`; `main.rs`/`lib.rs` as above.

## Flow

Inbound datagrams/API writes enter `node.rs` → mutate `Directory` → gossip fanout through `peer.rs` table over `protocol.rs`; discovery fills the peer table at startup and during partitions; HTTP reads serve any-consumers directly from the converged registry state; LfgService periodic pairing emits standard lobby announcements.

## Integration

- Pure standalone crate: only external deps; nothing imports it (this workspace consumes it as an operational service binary).
- Modules form a pipeline: `discovery` → `peer` → `protocol` → `registry` → `api`, with `matchmaking` layered on top of `registry`.
- Test corner: 47 unit + 5 integration + 1 doc test per BUILD_STATUS.
