//! Peer table with liveness tracking for the gossip mesh.
//!
//! UDP gives no connection state, so liveness is inferred: every inbound
//! datagram revives a peer, every unanswered probe increments a failure
//! counter. Two misses mark a peer suspect, four mark it dead. Dead peers are
//! retried on a backoff schedule rather than forgotten, because a node that
//! rebooted or lost a route can come back and still be useful.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::model::{now_ms, NodeId, NodeInfo};

/// Consecutive misses before a peer becomes suspect.
pub const SUSPECT_AFTER: u32 = 2;
/// Consecutive misses before a peer becomes dead.
pub const DEAD_AFTER: u32 = 4;

/// Observed health of a peer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeerState {
    /// Recently answered.
    Alive,
    /// Missed a probe but not yet declared dead.
    Suspect,
    /// Silent; retried with backoff.
    Dead,
}

/// How a peer address entered the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeerOrigin {
    /// Hard-coded `--bootstrap` address.
    Static,
    /// Resolved from a DNS seed.
    Dns,
    /// Announced over LAN multicast.
    Multicast,
    /// Fetched from the BitTorrent fallback.
    Torrent,
    /// Learned from another node.
    Gossip,
    /// A node that dialled us.
    Inbound,
}

/// One row of the peer table.
#[derive(Debug, Clone)]
pub struct PeerEntry {
    /// Gossip endpoint.
    pub addr: SocketAddr,
    /// Identity, once known.
    pub node: Option<NodeInfo>,
    /// Current health.
    pub state: PeerState,
    /// Monotonic time of the last inbound datagram.
    pub last_seen: Instant,
    /// Wall clock of the last inbound datagram.
    pub last_seen_ms: i64,
    /// Consecutive unanswered probes.
    pub failures: u32,
    /// Last measured round-trip time.
    pub latency_ms: Option<u32>,
    /// Earliest time to probe again.
    pub next_attempt: Instant,
    /// How the address was learned.
    pub origin: PeerOrigin,
    /// Bootstrap addresses are never garbage-collected.
    pub pinned: bool,
}

/// Serialisable peer row for the HTTP API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerView {
    /// Gossip endpoint.
    pub addr: SocketAddr,
    /// Node identity when known.
    pub node_id: Option<NodeId>,
    /// Node name when known.
    pub name: Option<String>,
    /// Node HTTP address when known.
    pub http_addr: Option<SocketAddr>,
    /// Current health.
    pub state: PeerState,
    /// Wall clock of the last inbound datagram, in Unix milliseconds.
    pub last_seen_ms: i64,
    /// Consecutive misses.
    pub failures: u32,
    /// Last round-trip time.
    pub latency_ms: Option<u32>,
    /// How the address was learned.
    pub origin: PeerOrigin,
}

impl PeerEntry {
    fn view(&self) -> PeerView {
        PeerView {
            addr: self.addr,
            node_id: self.node.as_ref().map(|n| n.id),
            name: self.node.as_ref().map(|n| n.name.clone()),
            http_addr: self.node.as_ref().and_then(|n| n.http_addr),
            state: self.state,
            last_seen_ms: self.last_seen_ms,
            failures: self.failures,
            latency_ms: self.latency_ms,
            origin: self.origin,
        }
    }
}

/// Bounded collection of known peers.
#[derive(Debug)]
pub struct PeerTable {
    peers: HashMap<SocketAddr, PeerEntry>,
    max_peers: usize,
    retain_dead: Duration,
}

impl PeerTable {
    /// Create a table capped at `max_peers` entries.
    #[must_use]
    pub fn new(max_peers: usize) -> Self {
        Self {
            peers: HashMap::new(),
            max_peers: max_peers.max(1),
            retain_dead: Duration::from_secs(600),
        }
    }

    /// Number of tracked peers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.peers.len()
    }

    /// Whether the table is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    /// Number of peers currently considered alive.
    #[must_use]
    pub fn alive_count(&self) -> usize {
        self.peers
            .values()
            .filter(|p| p.state == PeerState::Alive)
            .count()
    }

    /// Whether an address is already tracked.
    #[must_use]
    pub fn contains(&self, addr: SocketAddr) -> bool {
        self.peers.contains_key(&addr)
    }

    /// Insert a peer if there is room. Returns whether it was inserted.
    pub fn add(&mut self, addr: SocketAddr, origin: PeerOrigin, pinned: bool) -> bool {
        if self.peers.contains_key(&addr) {
            if pinned {
                if let Some(entry) = self.peers.get_mut(&addr) {
                    entry.pinned = true;
                }
            }
            return false;
        }
        if self.peers.len() >= self.max_peers {
            return false;
        }
        let now = Instant::now();
        self.peers.insert(
            addr,
            PeerEntry {
                addr,
                node: None,
                state: PeerState::Suspect,
                last_seen: now,
                last_seen_ms: now_ms(),
                failures: 0,
                latency_ms: None,
                next_attempt: now,
                origin,
                pinned,
            },
        );
        true
    }

    /// Record an inbound datagram from `addr`, reviving the peer.
    pub fn observe(&mut self, addr: SocketAddr, node: Option<NodeInfo>) -> PeerState {
        let now = Instant::now();
        let entry = self.peers.entry(addr).or_insert_with(|| PeerEntry {
            addr,
            node: None,
            state: PeerState::Suspect,
            last_seen: now,
            last_seen_ms: now_ms(),
            failures: 0,
            latency_ms: None,
            next_attempt: now,
            origin: PeerOrigin::Inbound,
            pinned: false,
        });
        if let Some(node) = node {
            entry.node = Some(node);
        }
        entry.state = PeerState::Alive;
        entry.failures = 0;
        entry.last_seen = now;
        entry.last_seen_ms = now_ms();
        entry.next_attempt = now;
        entry.state
    }

    /// Record a measured round-trip time.
    pub fn set_latency(&mut self, addr: SocketAddr, latency: Duration) {
        if let Some(entry) = self.peers.get_mut(&addr) {
            entry.latency_ms = Some(u32::try_from(latency.as_millis()).unwrap_or(u32::MAX));
        }
    }

    /// Mark a probe as in flight, delaying the next retry.
    pub fn mark_attempt(&mut self, addr: SocketAddr, backoff: Duration) {
        if let Some(entry) = self.peers.get_mut(&addr) {
            entry.next_attempt = Instant::now() + backoff;
        }
    }

    /// Record a missed probe and return the new state.
    pub fn fail(&mut self, addr: SocketAddr, backoff: Duration) -> Option<PeerState> {
        let entry = self.peers.get_mut(&addr)?;
        entry.failures = entry.failures.saturating_add(1);
        entry.next_attempt = Instant::now() + backoff;
        entry.state = if entry.failures >= DEAD_AFTER {
            PeerState::Dead
        } else if entry.failures >= SUSPECT_AFTER {
            PeerState::Suspect
        } else {
            entry.state
        };
        Some(entry.state)
    }

    /// Addresses that should be probed now (suspect or dead, backoff expired).
    #[must_use]
    pub fn retry_candidates(&self, now: Instant, limit: usize) -> Vec<SocketAddr> {
        let mut out: Vec<SocketAddr> = self
            .peers
            .values()
            .filter(|p| p.state != PeerState::Alive && p.next_attempt <= now)
            .map(|p| p.addr)
            .collect();
        out.sort();
        out.truncate(limit);
        out
    }

    /// Pick up to `k` live gossip targets, preferring healthy peers.
    #[must_use]
    pub fn gossip_targets(&self, k: usize, rng: &mut impl rand::Rng) -> Vec<SocketAddr> {
        let mut alive: Vec<SocketAddr> = self
            .peers
            .values()
            .filter(|p| p.state == PeerState::Alive)
            .map(|p| p.addr)
            .collect();
        alive.shuffle(rng);
        alive.truncate(k);
        alive
    }

    /// Snapshot every peer, ordered by address for stable output.
    #[must_use]
    pub fn snapshot(&self) -> Vec<PeerView> {
        let mut out: Vec<PeerView> = self.peers.values().map(PeerEntry::view).collect();
        out.sort_by_key(|p| p.addr);
        out
    }

    /// Identity currently recorded for a peer.
    #[must_use]
    pub fn node_info(&self, addr: SocketAddr) -> Option<NodeInfo> {
        self.peers.get(&addr).and_then(|p| p.node.clone())
    }

    /// Merge addresses advertised by a peer, ignoring our own address.
    pub fn merge_advertised(&mut self, addrs: &[SocketAddr], self_addr: SocketAddr) {
        for addr in addrs {
            if *addr == self_addr {
                continue;
            }
            self.add(*addr, PeerOrigin::Gossip, false);
        }
    }

    /// Live-or-suspect peers that have sent nothing for at least `timeout`.
    ///
    /// Including suspects is what lets a peer escalate from suspect to dead:
    /// each maintenance pass records another miss until [DEAD_AFTER] is hit.
    #[must_use]
    pub fn silent_peers(&self, now: Instant, timeout: Duration) -> Vec<SocketAddr> {
        let mut out: Vec<SocketAddr> = self
            .peers
            .values()
            .filter(|p| p.state != PeerState::Dead && now.duration_since(p.last_seen) >= timeout)
            .map(|p| p.addr)
            .collect();
        out.sort();
        out
    }

    /// Force a peer to dead, e.g. after a graceful Bye.
    pub fn mark_dead(&mut self, addr: SocketAddr) -> bool {
        if let Some(entry) = self.peers.get_mut(&addr) {
            entry.state = PeerState::Dead;
            entry.failures = DEAD_AFTER;
            entry.next_attempt = Instant::now() + Duration::from_secs(30);
            true
        } else {
            false
        }
    }

    /// Drop dead, unpinned peers that have been silent for a long time.
    pub fn purge(&mut self, now: Instant) -> usize {
        let before = self.peers.len();
        let retain = self.retain_dead;
        self.peers.retain(|_, p| {
            p.pinned || p.state != PeerState::Dead || now.duration_since(p.last_seen) < retain
        });
        before - self.peers.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn addr(port: u16) -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], port))
    }

    #[test]
    fn failures_escalate_and_observe_revives() {
        let mut table = PeerTable::new(8);
        assert!(table.add(addr(1), PeerOrigin::Static, true));
        table.observe(addr(1), None);
        assert_eq!(table.alive_count(), 1);
        let backoff = Duration::from_millis(1);
        table.fail(addr(1), backoff);
        table.fail(addr(1), backoff);
        assert_eq!(
            table.snapshot().first().map(|p| p.state),
            Some(PeerState::Suspect)
        );
        table.fail(addr(1), backoff);
        table.fail(addr(1), backoff);
        assert_eq!(
            table.snapshot().first().map(|p| p.state),
            Some(PeerState::Dead)
        );
        table.observe(addr(1), None);
        assert_eq!(table.alive_count(), 1);
    }

    #[test]
    fn gossip_targets_exclude_dead_peers() {
        let mut table = PeerTable::new(8);
        for port in 1..=4 {
            table.add(addr(port), PeerOrigin::Gossip, false);
            table.observe(addr(port), None);
        }
        let backoff = Duration::from_secs(60);
        for _ in 0..DEAD_AFTER {
            table.fail(addr(2), backoff);
        }
        let mut rng = rand::thread_rng();
        let mut targets = table.gossip_targets(10, &mut rng);
        targets.sort();
        assert_eq!(targets, vec![addr(1), addr(3), addr(4)]);
    }

    #[test]
    fn table_is_bounded() {
        let mut table = PeerTable::new(2);
        assert!(table.add(addr(1), PeerOrigin::Gossip, false));
        assert!(table.add(addr(2), PeerOrigin::Gossip, false));
        assert!(!table.add(addr(3), PeerOrigin::Gossip, false));
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn advertised_addresses_skip_self() {
        let mut table = PeerTable::new(8);
        table.merge_advertised(&[addr(1), addr(9)], addr(9));
        assert!(table.contains(addr(1)));
        assert!(!table.contains(addr(9)));
    }

    #[test]
    fn pinned_peers_survive_purge() {
        let mut table = PeerTable::new(8);
        table.add(addr(1), PeerOrigin::Static, true);
        table.add(addr(2), PeerOrigin::Gossip, false);
        for _ in 0..DEAD_AFTER {
            table.fail(addr(1), Duration::ZERO);
            table.fail(addr(2), Duration::ZERO);
        }
        table.purge(Instant::now() + Duration::from_secs(700));
        assert!(table.contains(addr(1)));
        assert!(!table.contains(addr(2)));
    }
}
