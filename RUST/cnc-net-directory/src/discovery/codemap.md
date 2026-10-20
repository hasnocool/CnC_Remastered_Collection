# RUST/cnc-net-directory/src/discovery/

## Responsibility

Discovery backends for bootstrap peer resolution: static seeds (literal `host:port`, DNS seed hostnames), and the multicast LAN path — the mesh entry points that keep the gossip network from ever depending on one source (per lib.rs/`discovery` docs).

## Design

- `mod.rs` (194 L) — backend abstraction/sequencing: at startup resolve every configured seed, keep listening on LAN multicast afterwards, plumb results into the peer table; falls back through methods so cold-start recovers.
- `lan.rs` (68 L) — LAN multicast discovery: nodes join a multicast group and periodically send their ordinary `Hello` datagram to it; receivers learn the sender from the datagram's source address, exactly like a unicast hello, reusing `protocol.rs` messages without a separate handshake format.

## Flow

`NodeSettings` seeds → `mod.rs` resolve → peer addresses → `peer.rs` table; ongoing: multicast Hello → source-addr registration → gossip proceeds over the unicast path.

## Integration

Called by `cnc-net-directory/src/node.rs` (discovery loops, `run_forever`); sends/receives through `protocol.rs` datagram framing; tested indirectly via `tests/torrent_fallback.rs` (the third backend lives in `../torrent.rs` — the sibling plumbing used after seeds/multicast return too few peers).
