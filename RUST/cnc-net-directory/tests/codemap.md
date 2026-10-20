# RUST/cnc-net-directory/tests/

## Responsibility

Three integration test files — network-topology and matchmaking behaviour against real nodes/HTTP servers (BUILD_STATUS: 5 integration + 1 doc test; unit tests live in `src/`).

## Design

- `lfg.rs` (219 L) — "End-to-end ranked ladder over the HTTP API: register, queue, pair, report." `node() -> NodeHandle`, `two_players_are_paired_and_the_result_moves_the_ladder`, `ai_agents_use_a_separate_ladder_and_announce_ai_lobbies`, `a_player_cannot_join_twice_while_a_match_is_pending`.
- `mesh.rs` (191 L) — "convergence, withdrawal, peer-of-peer discovery, and survival of a node failure." Helpers `base_settings(name)`/`spawn(name, bootstrap)` spawn real nodes with chosen bootstrap seeds; `lobby(name, address)` builds `PublishGame`; `wait_for(label, condition)` polls; tests: `a_chain_of_nodes_converges_on_one_lobby`, `surviving_nodes_keep_serving_after_one_dies` (+ withdrawal/discovery cases).
- `torrent_fallback.rs` (178 L) — "plain web-seed JSON, and the qBittorrent download path." Mock servers: `serve_json_node_list(list) -> SocketAddr`, `serve_mock_qbittorrent(state: QbState)`; tests `isolated_node_finds_the_mesh_through_a_web_seed`, `qbittorrent_path_recovers_the_node_list`.

## Flow

Each test spawns real `NodeHandle`s (and mock HTTP servers where needed) on ephemeral ports, drives operations through the public API/HTTP client, and polls until gossip converges or a result appears — no real internet, no long sleeps beyond bounded `wait_for` deadlines.

## Integration

Exercises the `node.rs` runtime (bind/bind/gossip/shutdown), `registry.rs` convergence guarantees, `api.rs` + `client.rs` HTTP surface, `matchmaking.rs` queue/pair/report state machine, and `torrent.rs`/`discovery/` fallback backends, including failure-path coverage (node death, cold start).
