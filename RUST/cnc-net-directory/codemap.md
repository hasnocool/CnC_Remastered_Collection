# RUST/cnc-net-directory/

## Responsibility

Standalone decentralized game-directory node (crate description: "Decentralised game-directory node: announces lobby games and gossips them across a self-healing peer mesh"). Runs an HTTP+UDP node binary (`cnc-directory`) that hosts publish lobby announcements to, browsers query, and ranked LFG matchmaking — no single point of failure (lib.rs docs).

## Design

- Gossip-based replication (`registry.rs`, 1004 L): `Directory` records are last-writer-wins keyed by `(updated_at_ms, origin, revision)`; withdrawals are tombstones; `MergeStats` reports convergence deltas.
- Wire protocol (`protocol.rs`, 304 L): UDP datagrams `MAGIC(4) || VERSION(1) || TAG(1) || bincode(payload)`, stateless and idempotent.
- Discovery backends (`discovery/`): static seeds/DNS, LAN multicast, BitTorrent fallback (`torrent.rs` with `bencode.rs` codec + `.torrent` metainfo generation/info-hash); health-tracked `peer.rs` peer table (2 misses suspect, 4 dead).
- HTTP plane (`api.rs`, 759 L): JSON API + built-in browser UI via `axum`+`tower-http` (CORS, trace, timeout, gzip); read-mostly (publish/withdraw/heartbeat writes).
- Matchmaking (`matchmaking.rs`, 1736 L): `LfgService`, `PlayerProfile`/`PlayerId`, per-title MMR, `League`/`LeaderboardRow`, queues `JoinQueue`, `RankedMatch`/`MatchState`/`MatchId`, `RatingChange`; human vs AI-only pools (see README lfg tests).
- Node runtime (`node.rs`, 741 L): `NodeSettings`/`NodeHandle`, `run_forever(settings)`, `describe_node(info)`; `client.rs` HTTP client `DirectoryClient::first_available` for failover across several nodes; `error.rs` typed `DirectoryError` + `Result`.
- Enforced quality gates in Cargo.toml `[lints.rust] unsafe_code = forbid`, `[lints.clippy] unwrap_used/expect_used/todo/unimplemented/dbg_macro = deny` — the most lint-strict crate in the workspace. Notably independent: no cnc-* dependencies.

## Flow

Publish: host → HTTP `api.rs` → `Directory` write → `node.rs` gossip loop sends `protocol` datagrams to peers → peers converge. Discovery: `discovery` seeds + LAN multicast + `torrent` fallback seed the peer list when the mesh is cold. Query: browser/CLI (`main.rs` subcommands `Torrent {`/`Lfg {}`, `run`, register, queue) → `client.rs` → any node API. Matchmaking: queue → pair → announce pair as a lobby.

## Integration

- Consumed by: end users via `cnc-directory` bin and `DirectoryClient`; integrated tests `tests/{mesh,lfg,torrent_fallback}.rs`; `cnc-ai::ladder` produces AI-player cards (conceptual integration, no direct dependency). Game side (`cnc-tiberian-dawn::network::LiveSession`) is not wired as a dependency — hosts call the HTTP API themselves.
- Deps: `axum 0.8`, `reqwest 0.13` (rustls), `tokio`, `tower-http`, `toml` config, `rand`, `sha1`, `bincode`, `uuid`, `tracing`.
