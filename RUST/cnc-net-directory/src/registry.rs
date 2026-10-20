//! The replicated lobby registry and the shared node state.
//!
//! Records converge without a coordinator: every write carries an
//! `(updated_at_ms, origin, revision)` order key, and replicas keep the
//! greatest key they have seen. Withdrawals are tombstones that beat older
//! live records and are retained for an hour so a delayed gossip packet cannot
//! resurrect a dead lobby.

use std::collections::{BTreeMap, HashMap};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::ValidationError;
use crate::model::{
    now_ms, GameFilter, GameId, GameInfo, GameKind, GameState, NodeId, NodeInfo, PublishGame,
    PLAYERS_MAX, PLAYERS_MIN, TTL_MAX, TTL_MIN,
};
use crate::peer::{PeerOrigin, PeerTable, PeerView};

/// Version reported by the HTTP API and the wire Hello message.
pub const NODE_VERSION: &str = env!("CARGO_PKG_VERSION");
/// How long a withdrawal tombstone is remembered.
pub const TOMBSTONE_TTL: Duration = Duration::from_secs(3600);
/// Hard cap on replicated records, tombstone included.
pub const MAX_ENTRIES: usize = 4096;

/// Result of merging one remote record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeOutcome {
    /// The record was new.
    Inserted,
    /// The record was newer than the local copy.
    Updated,
    /// The local copy was equal or newer.
    Ignored,
    /// The record failed sanity checks and was dropped.
    Rejected,
}

/// Aggregate counters for one merge batch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MergeStats {
    /// New records.
    pub inserted: usize,
    /// Records that replaced an older local copy.
    pub updated: usize,
    /// Records that lost the merge.
    pub ignored: usize,
    /// Malformed records dropped.
    pub rejected: usize,
}

impl MergeStats {
    /// Fold another batch into this one.
    pub fn absorb(&mut self, other: Self) {
        self.inserted += other.inserted;
        self.updated += other.updated;
        self.ignored += other.ignored;
        self.rejected += other.rejected;
    }

    /// Number of records that changed the local view.
    #[must_use]
    pub const fn changed(self) -> usize {
        self.inserted + self.updated
    }
}

#[derive(Debug, Clone)]
struct GameEntry {
    info: GameInfo,
    received_at: Instant,
    expires_at: Instant,
}

/// Replicated map of game id to lobby record.
#[derive(Debug)]
pub struct GameRegistry {
    entries: HashMap<GameId, GameEntry>,
    tombstone_ttl: Duration,
    max_entries: usize,
}

impl Default for GameRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl GameRegistry {
    /// Empty registry with production limits.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            tombstone_ttl: TOMBSTONE_TTL,
            max_entries: MAX_ENTRIES,
        }
    }

    /// Number of tracked records, including tombstones.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry holds no records at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Merge a remote record.
    pub fn merge(&mut self, incoming: GameInfo, now: Instant) -> MergeOutcome {
        let Some(incoming) = sanitize(incoming) else {
            return MergeOutcome::Rejected;
        };
        let outcome = match self.entries.get(&incoming.id) {
            None => MergeOutcome::Inserted,
            Some(existing) => {
                if incoming.order_key() > existing.info.order_key() {
                    MergeOutcome::Updated
                } else {
                    MergeOutcome::Ignored
                }
            }
        };
        if matches!(outcome, MergeOutcome::Ignored | MergeOutcome::Rejected) {
            return outcome;
        }
        if outcome == MergeOutcome::Inserted && self.entries.len() >= self.max_entries {
            self.evict_one();
        }
        let ttl = if incoming.deleted {
            self.tombstone_ttl
        } else {
            Duration::from_secs(u64::from(incoming.ttl_secs))
        };
        self.entries.insert(
            incoming.id,
            GameEntry {
                info: incoming,
                received_at: now,
                expires_at: now + ttl,
            },
        );
        outcome
    }

    /// Insert or replace a locally accepted record unconditionally.
    pub fn upsert_local(&mut self, info: GameInfo, now: Instant) {
        let ttl = if info.deleted {
            self.tombstone_ttl
        } else {
            Duration::from_secs(u64::from(info.ttl_secs))
        };
        if !self.entries.contains_key(&info.id) && self.entries.len() >= self.max_entries {
            self.evict_one();
        }
        self.entries.insert(
            info.id,
            GameEntry {
                info,
                received_at: now,
                expires_at: now + ttl,
            },
        );
    }

    /// Look one record up, treating expired entries as absent.
    #[must_use]
    pub fn get(&self, id: GameId, now: Instant) -> Option<GameInfo> {
        self.entries
            .get(&id)
            .filter(|e| e.expires_at > now)
            .map(|e| e.info.clone())
    }

    /// Public, non-tombstone records matching a filter, newest first.
    #[must_use]
    pub fn public_games(&self, filter: &GameFilter, now: Instant) -> Vec<GameInfo> {
        let mut out: Vec<GameInfo> = self
            .entries
            .values()
            .filter(|e| e.expires_at > now && !e.info.deleted)
            .map(|e| e.info.clone())
            .filter(|g| filter.matches(g))
            .collect();
        out.sort_by(|a, b| {
            b.order_key()
                .cmp(&a.order_key())
                .then_with(|| a.name.cmp(&b.name))
        });
        out.truncate(filter.effective_limit());
        out
    }

    /// Every live record, for anti-entropy replication.
    #[must_use]
    pub fn all_live(&self, now: Instant) -> Vec<GameInfo> {
        let mut out: Vec<GameInfo> = self
            .entries
            .values()
            .filter(|e| e.expires_at > now)
            .map(|e| e.info.clone())
            .collect();
        out.sort_by_key(GameInfo::order_key);
        out
    }

    /// Records written after `since_ms`, oldest first, capped at `limit`.
    #[must_use]
    pub fn since(&self, since_ms: i64, limit: usize, now: Instant) -> Vec<GameInfo> {
        let mut out: Vec<GameInfo> = self
            .entries
            .values()
            .filter(|e| e.expires_at > now && e.info.updated_at_ms > since_ms)
            .map(|e| e.info.clone())
            .collect();
        out.sort_by_key(GameInfo::order_key);
        out.truncate(limit.max(1));
        out
    }

    /// Deterministic fingerprint of the current view, used to detect divergence.
    #[must_use]
    pub fn digest(&self) -> u64 {
        let mut live: Vec<(GameId, i64, u64, bool)> = self
            .entries
            .values()
            .map(|e| {
                (
                    e.info.id,
                    e.info.updated_at_ms,
                    e.info.revision,
                    e.info.deleted,
                )
            })
            .collect();
        live.sort_unstable();
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for (id, updated, revision, deleted) in live {
            for byte in id.to_string().as_bytes() {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            }
            hash ^= updated as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            hash ^= revision;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
            hash ^= u64::from(deleted);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// Remove expired live records and stale tombstones.
    pub fn expire(&mut self, now: Instant) -> usize {
        let before = self.entries.len();
        self.entries.retain(|_, e| e.expires_at > now);
        before - self.entries.len()
    }

    /// Number of live (non-tombstone, unexpired) records.
    #[must_use]
    pub fn live_count(&self, now: Instant) -> usize {
        self.entries
            .values()
            .filter(|e| e.expires_at > now && !e.info.deleted)
            .count()
    }

    /// Count of retained tombstones.
    #[must_use]
    pub fn tombstone_count(&self) -> usize {
        self.entries.values().filter(|e| e.info.deleted).count()
    }

    /// Per-state and per-kind histograms for the status endpoint.
    #[must_use]
    pub fn histograms(&self, now: Instant) -> (BTreeMap<String, usize>, BTreeMap<String, usize>) {
        let mut by_state: BTreeMap<String, usize> = BTreeMap::new();
        let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for entry in self.entries.values() {
            if entry.expires_at <= now || entry.info.deleted {
                continue;
            }
            *by_state
                .entry(entry.info.state.as_str().to_owned())
                .or_insert(0) += 1;
            *by_kind
                .entry(entry.info.kind.as_str().to_owned())
                .or_insert(0) += 1;
        }
        (by_state, by_kind)
    }

    fn evict_one(&mut self) {
        let oldest = self
            .entries
            .iter()
            .min_by_key(|(_, e)| (e.info.deleted, e.received_at))
            .map(|(id, _)| *id);
        if let Some(id) = oldest {
            self.entries.remove(&id);
        }
    }
}

/// Normalise a record received from the network; drop records that cannot be
/// repaired (empty name, impossible player counts).
fn sanitize(mut info: GameInfo) -> Option<GameInfo> {
    info.name = info.name.trim().to_owned();
    info.map = info.map.trim().to_owned();
    info.host_name = info.host_name.trim().to_owned();
    info.version = info.version.trim().chars().take(32).collect();
    if info.name.is_empty() || info.map.is_empty() {
        return None;
    }
    info.name.truncate(128);
    info.map.truncate(128);
    info.host_name.truncate(64);
    info.max_players = info.max_players.clamp(PLAYERS_MIN, PLAYERS_MAX);
    info.players = info.players.clamp(1, info.max_players);
    info.ttl_secs = info.ttl_secs.clamp(TTL_MIN, TTL_MAX);
    info.tags.truncate(8);
    for tag in &mut info.tags {
        tag.truncate(24);
    }
    info.tags.retain(|t| !t.is_empty());
    if info.address.port() == 0 || info.address.ip().is_unspecified() {
        return None;
    }
    Some(info)
}

/// Live counters exported by the status endpoint.
#[derive(Debug, Default)]
pub struct Metrics {
    datagrams_in: AtomicU64,
    datagrams_out: AtomicU64,
    bytes_in: AtomicU64,
    bytes_out: AtomicU64,
    games_announced: AtomicU64,
    games_merged: AtomicU64,
    games_rejected: AtomicU64,
    peer_errors: AtomicU64,
}

impl Metrics {
    /// Count one accepted inbound datagram.
    pub fn note_in(&self, bytes: usize) {
        self.datagrams_in.fetch_add(1, Ordering::Relaxed);
        self.bytes_in
            .fetch_add(u64::try_from(bytes).unwrap_or(u64::MAX), Ordering::Relaxed);
    }

    /// Count one sent datagram.
    pub fn note_out(&self, bytes: usize) {
        self.datagrams_out.fetch_add(1, Ordering::Relaxed);
        self.bytes_out
            .fetch_add(u64::try_from(bytes).unwrap_or(u64::MAX), Ordering::Relaxed);
    }

    /// Count one failed send.
    pub fn note_peer_error(&self) {
        self.peer_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Snapshot the counters.
    #[must_use]
    pub fn snapshot(&self) -> MetricsView {
        MetricsView {
            datagrams_in: self.datagrams_in.load(Ordering::Relaxed),
            datagrams_out: self.datagrams_out.load(Ordering::Relaxed),
            bytes_in: self.bytes_in.load(Ordering::Relaxed),
            bytes_out: self.bytes_out.load(Ordering::Relaxed),
            games_announced: self.games_announced.load(Ordering::Relaxed),
            games_merged: self.games_merged.load(Ordering::Relaxed),
            games_rejected: self.games_rejected.load(Ordering::Relaxed),
            peer_errors: self.peer_errors.load(Ordering::Relaxed),
        }
    }
}

/// Point-in-time counter values.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MetricsView {
    /// Inbound datagrams.
    pub datagrams_in: u64,
    /// Outbound datagrams.
    pub datagrams_out: u64,
    /// Inbound bytes.
    pub bytes_in: u64,
    /// Outbound bytes.
    pub bytes_out: u64,
    /// Announcements accepted from hosts.
    pub games_announced: u64,
    /// Remote records accepted or updated.
    pub games_merged: u64,
    /// Remote records rejected as malformed.
    pub games_rejected: u64,
    /// Failed send attempts.
    pub peer_errors: u64,
}

/// Shared state of one directory node: registry, peer table, and counters.
#[derive(Debug)]
pub struct Directory {
    node_id: NodeId,
    node_name: String,
    gossip_addr: SocketAddr,
    http_addr: Option<SocketAddr>,
    started: Instant,
    started_at_ms: i64,
    registry: RwLock<GameRegistry>,
    peers: RwLock<PeerTable>,
    metrics: Metrics,
}

impl Directory {
    /// Build an empty directory for a node.
    #[must_use]
    pub fn new(
        node_id: NodeId,
        node_name: impl Into<String>,
        gossip_addr: SocketAddr,
        http_addr: Option<SocketAddr>,
    ) -> Self {
        Self {
            node_id,
            node_name: node_name.into(),
            gossip_addr,
            http_addr,
            started: Instant::now(),
            started_at_ms: now_ms(),
            registry: RwLock::new(GameRegistry::new()),
            peers: RwLock::new(PeerTable::new(10_000)),
            metrics: Metrics::default(),
        }
    }

    /// This node's identity.
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// This node's display name.
    #[must_use]
    pub fn node_name(&self) -> &str {
        &self.node_name
    }

    /// Advertised gossip address.
    #[must_use]
    pub fn gossip_addr(&self) -> SocketAddr {
        self.gossip_addr
    }

    /// Advertised HTTP address, when enabled.
    #[must_use]
    pub fn http_addr(&self) -> Option<SocketAddr> {
        self.http_addr
    }

    /// Configured HTTP address after the listener binds.
    pub fn set_http_addr(&mut self, addr: SocketAddr) {
        self.http_addr = Some(addr);
    }

    /// Seconds since the node started.
    #[must_use]
    pub fn uptime_secs(&self) -> u64 {
        self.started.elapsed().as_secs()
    }

    /// The node's Hello payload.
    #[must_use]
    pub fn self_info(&self) -> NodeInfo {
        NodeInfo {
            id: self.node_id,
            name: self.node_name.clone(),
            gossip_addr: self.gossip_addr,
            http_addr: self.http_addr,
            version: NODE_VERSION.to_owned(),
            started_at_ms: self.started_at_ms,
            games: u32::try_from(self.registry.read().live_count(Instant::now()))
                .unwrap_or(u32::MAX),
        }
    }

    /// Publish a lobby announcement locally.
    ///
    /// # Errors
    ///
    /// Returns [ValidationError] when the announcement is unusable.
    pub fn announce(&self, publish: PublishGame) -> Result<GameInfo, ValidationError> {
        let validated = publish.validate()?;
        let now = Instant::now();
        let id = publish.id.unwrap_or_default();
        let mut registry = self.registry.write();
        let revision = registry.entries.get(&id).map_or_else(
            || now_ms() as u64,
            |e| (now_ms() as u64).max(e.info.revision.saturating_add(1)),
        );
        let info = GameInfo {
            id,
            name: validated.name,
            kind: publish.kind,
            map: validated.map,
            host_name: validated.host_name,
            address: validated.address,
            players: validated.players,
            max_players: validated.max_players,
            state: validated.state,
            passworded: validated.passworded,
            version: validated.version,
            tags: validated.tags,
            ttl_secs: validated.ttl_secs,
            origin: self.node_id,
            revision,
            updated_at_ms: now_ms(),
            deleted: false,
        };
        registry.upsert_local(info.clone(), now);
        drop(registry);
        self.metrics.games_announced.fetch_add(1, Ordering::Relaxed);
        Ok(info)
    }

    /// Refresh an existing lobby's TTL and revision.
    pub fn heartbeat(&self, id: GameId) -> Option<GameInfo> {
        let now = Instant::now();
        let mut registry = self.registry.write();
        let entry = registry.entries.get_mut(&id)?;
        if entry.info.deleted {
            return None;
        }
        entry.info.revision = (now_ms() as u64).max(entry.info.revision.saturating_add(1));
        entry.info.updated_at_ms = now_ms();
        entry.received_at = now;
        entry.expires_at = now + Duration::from_secs(u64::from(entry.info.ttl_secs));
        Some(entry.info.clone())
    }

    /// Withdraw a lobby, leaving a tombstone that gossips outward.
    pub fn withdraw(&self, id: GameId) -> bool {
        let now = Instant::now();
        let tombstone_ttl = self.registry.read().tombstone_ttl;
        let mut registry = self.registry.write();
        let Some(entry) = registry.entries.get_mut(&id) else {
            return false;
        };
        if entry.info.deleted {
            return false;
        }
        entry.info.deleted = true;
        entry.info.revision = (now_ms() as u64).max(entry.info.revision.saturating_add(1));
        entry.info.updated_at_ms = now_ms();
        entry.received_at = now;
        entry.expires_at = now + tombstone_ttl;
        true
    }

    /// Merge a batch of gossip records.
    pub fn merge_games(&self, games: Vec<GameInfo>) -> MergeStats {
        let now = Instant::now();
        let mut registry = self.registry.write();
        let mut stats = MergeStats::default();
        for game in games {
            match registry.merge(game, now) {
                MergeOutcome::Inserted => stats.inserted += 1,
                MergeOutcome::Updated => stats.updated += 1,
                MergeOutcome::Ignored => stats.ignored += 1,
                MergeOutcome::Rejected => stats.rejected += 1,
            }
        }
        drop(registry);
        self.metrics
            .games_merged
            .fetch_add(stats.changed() as u64, Ordering::Relaxed);
        self.metrics
            .games_rejected
            .fetch_add(stats.rejected as u64, Ordering::Relaxed);
        stats
    }

    /// Browse records matching a filter.
    #[must_use]
    pub fn games(&self, filter: &GameFilter) -> Vec<GameInfo> {
        self.registry.read().public_games(filter, Instant::now())
    }

    /// Fetch one public record by id.
    #[must_use]
    pub fn game(&self, id: GameId) -> Option<GameInfo> {
        self.registry
            .read()
            .get(id, Instant::now())
            .filter(|g| !g.deleted)
    }

    /// Current registry fingerprint.
    #[must_use]
    pub fn digest(&self) -> u64 {
        self.registry.read().digest()
    }

    /// Records written after a wall-clock instant, for delta sync.
    #[must_use]
    pub fn sync_batch(&self, since_ms: i64, limit: usize) -> Vec<GameInfo> {
        self.registry.read().since(since_ms, limit, Instant::now())
    }

    /// Every live record, for a full anti-entropy push.
    #[must_use]
    pub fn all_games(&self) -> Vec<GameInfo> {
        self.registry.read().all_live(Instant::now())
    }

    /// Drop expired records; returns how many were removed.
    pub fn expire(&self) -> usize {
        self.registry.write().expire(Instant::now())
    }

    /// Whether `addr` refers to this node's own gossip socket.
    ///
    /// When bound to an unspecified address the advertised address carries
    /// `0.0.0.0`, so compare the port and treat any local address with that
    /// port as ourselves.
    #[must_use]
    pub fn is_self(&self, addr: SocketAddr) -> bool {
        addr.port() == self.gossip_addr.port()
            && (self.gossip_addr.ip().is_unspecified() || addr.ip() == self.gossip_addr.ip())
    }

    /// Add a peer address discovered by a bootstrap backend.
    pub fn learn_peer(&self, addr: SocketAddr, origin: PeerOrigin, pinned: bool) -> bool {
        if self.is_self(addr) {
            return false;
        }
        self.peers.write().add(addr, origin, pinned)
    }

    /// Learn several addresses at once.
    pub fn learn_peers(&self, addrs: &[SocketAddr], origin: PeerOrigin) {
        let mut peers = self.peers.write();
        for addr in addrs {
            if self.is_self(*addr) {
                continue;
            }
            peers.add(*addr, origin, false);
        }
    }

    /// Record an inbound datagram, optionally learning the sender identity.
    pub fn observe_peer(&self, addr: SocketAddr, node: Option<NodeInfo>) {
        if self.is_self(addr) {
            return;
        }
        self.peers.write().observe(addr, node);
    }

    /// Record a failed probe.
    pub fn fail_peer(&self, addr: SocketAddr, backoff: Duration) -> Option<crate::peer::PeerState> {
        self.peers.write().fail(addr, backoff)
    }

    /// Live-or-suspect peers silent for at least `timeout`.
    #[must_use]
    pub fn silent_peers(&self, timeout: Duration) -> Vec<SocketAddr> {
        self.peers.read().silent_peers(Instant::now(), timeout)
    }

    /// Force a peer to dead after a graceful Bye.
    pub fn mark_peer_dead(&self, addr: SocketAddr) {
        self.peers.write().mark_dead(addr);
    }

    /// Mark a probe as sent.
    pub fn mark_peer_attempt(&self, addr: SocketAddr, backoff: Duration) {
        self.peers.write().mark_attempt(addr, backoff);
    }

    /// Peers whose backoff has elapsed.
    #[must_use]
    pub fn retry_candidates(&self, limit: usize) -> Vec<SocketAddr> {
        self.peers.read().retry_candidates(Instant::now(), limit)
    }

    /// Random live gossip targets.
    #[must_use]
    pub fn gossip_targets(&self, k: usize, rng: &mut impl rand::Rng) -> Vec<SocketAddr> {
        self.peers.read().gossip_targets(k, rng)
    }

    /// Snapshot of every known peer.
    #[must_use]
    pub fn peers(&self) -> Vec<PeerView> {
        self.peers.read().snapshot()
    }

    /// Number of peers currently considered alive.
    #[must_use]
    pub fn alive_peers(&self) -> usize {
        self.peers.read().alive_count()
    }

    /// Whether any peer is already tracked.
    #[must_use]
    pub fn knows_peer(&self, addr: SocketAddr) -> bool {
        self.peers.read().contains(addr)
    }

    /// Identity learned for a peer, if any.
    #[must_use]
    pub fn peer_identity(&self, addr: SocketAddr) -> Option<NodeInfo> {
        self.peers.read().node_info(addr)
    }

    /// Record a round-trip measurement.
    pub fn note_latency(&self, addr: SocketAddr, latency: Duration) {
        self.peers.write().set_latency(addr, latency);
    }

    /// Drop long-dead unpinned peers.
    pub fn purge_peers(&self) -> usize {
        self.peers.write().purge(Instant::now())
    }

    /// Counters for inbound/outbound traffic.
    pub fn metrics(&self) -> &Metrics {
        &self.metrics
    }

    /// Aggregated status for the HTTP API.
    #[must_use]
    pub fn stats(&self) -> Stats {
        let now = Instant::now();
        let registry = self.registry.read();
        let (by_state, by_kind) = registry.histograms(now);
        let games = registry.live_count(now);
        let tombstones = registry.tombstone_count();
        let digest = registry.digest();
        drop(registry);
        let peers = self.peers.read();
        Stats {
            node_id: self.node_id,
            node_name: self.node_name.clone(),
            version: NODE_VERSION.to_owned(),
            gossip_addr: self.gossip_addr,
            http_addr: self.http_addr,
            uptime_secs: self.uptime_secs(),
            games,
            tombstones,
            by_state,
            by_kind,
            peers_total: peers.len(),
            peers_alive: peers.alive_count(),
            digest,
            metrics: self.metrics.snapshot(),
        }
    }

    /// Number of live games held by this node.
    #[must_use]
    pub fn games_count(&self) -> usize {
        self.registry.read().live_count(Instant::now())
    }

    /// Game kinds currently present, for CLI defaults.
    #[must_use]
    pub fn kinds(&self) -> Vec<GameKind> {
        let registry = self.registry.read();
        let now = Instant::now();
        let mut kinds: Vec<GameKind> = registry
            .entries
            .values()
            .filter(|e| e.expires_at > now && !e.info.deleted)
            .map(|e| e.info.kind)
            .collect();
        kinds.sort_by_key(|k| k.as_str());
        kinds.dedup();
        kinds
    }

    /// States currently present, for CLI validation.
    #[must_use]
    pub fn states(&self) -> Vec<GameState> {
        let registry = self.registry.read();
        let now = Instant::now();
        let mut states: Vec<GameState> = registry
            .entries
            .values()
            .filter(|e| e.expires_at > now && !e.info.deleted)
            .map(|e| e.info.state)
            .collect();
        states.sort_by_key(|s| s.as_str());
        states.dedup();
        states
    }
}

/// Node status payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    /// Node identity.
    pub node_id: NodeId,
    /// Node name.
    pub node_name: String,
    /// Crate version.
    pub version: String,
    /// Advertised gossip address.
    pub gossip_addr: SocketAddr,
    /// Advertised HTTP address.
    pub http_addr: Option<SocketAddr>,
    /// Seconds since start.
    pub uptime_secs: u64,
    /// Live games.
    pub games: usize,
    /// Retained tombstones.
    pub tombstones: usize,
    /// Games per lifecycle state.
    pub by_state: BTreeMap<String, usize>,
    /// Games per title.
    pub by_kind: BTreeMap<String, usize>,
    /// Known peers.
    pub peers_total: usize,
    /// Alive peers.
    pub peers_alive: usize,
    /// Registry fingerprint.
    pub digest: u64,
    /// Traffic counters.
    pub metrics: MetricsView,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{GameKind, GameState, NodeId, PublishGame};

    fn node() -> NodeId {
        NodeId::new()
    }

    fn game(id: GameId, origin: NodeId, updated_at_ms: i64, name: &str) -> GameInfo {
        GameInfo {
            id,
            name: name.to_owned(),
            kind: GameKind::TiberianDawn,
            map: "SCG01EA".to_owned(),
            host_name: "Kane".to_owned(),
            address: "203.0.113.7:47771".parse().expect("addr"),
            players: 2,
            max_players: 4,
            state: GameState::Lobby,
            passworded: false,
            version: "1.0".to_owned(),
            tags: vec![],
            ttl_secs: 60,
            origin,
            revision: updated_at_ms as u64,
            updated_at_ms,
            deleted: false,
        }
    }

    #[test]
    fn merge_converges_regardless_of_order() {
        let id = GameId::new();
        let a = node();
        let b = node();
        let old = game(id, a, 100, "old");
        let new = game(id, b, 200, "new");
        let mut left = GameRegistry::new();
        let mut right = GameRegistry::new();
        let now = Instant::now();
        assert_eq!(left.merge(old.clone(), now), MergeOutcome::Inserted);
        assert_eq!(left.merge(new.clone(), now), MergeOutcome::Updated);
        assert_eq!(right.merge(new, now), MergeOutcome::Inserted);
        assert_eq!(right.merge(old, now), MergeOutcome::Ignored);
        assert_eq!(left.digest(), right.digest());
        assert_eq!(
            left.public_games(&GameFilter::default(), now)[0].name,
            "new"
        );
    }

    #[test]
    fn tombstone_beats_older_live_record() {
        let id = GameId::new();
        let origin = node();
        let mut registry = GameRegistry::new();
        let now = Instant::now();
        registry.merge(game(id, origin, 100, "live"), now);
        let mut dead = game(id, origin, 200, "live");
        dead.deleted = true;
        assert_eq!(registry.merge(dead, now), MergeOutcome::Updated);
        assert!(registry.get(id, now).is_some_and(|g| g.deleted));
        assert!(registry
            .public_games(&GameFilter::default(), now)
            .is_empty());
    }

    #[test]
    fn expired_records_disappear() {
        let id = GameId::new();
        let mut registry = GameRegistry::new();
        let now = Instant::now();
        registry.merge(game(id, node(), 100, "live"), now);
        assert_eq!(registry.live_count(now), 1);
        let later = now + Duration::from_secs(61);
        assert_eq!(registry.live_count(later), 0);
        assert_eq!(
            registry.public_games(&GameFilter::default(), later).len(),
            0
        );
        registry.expire(later);
        assert!(registry.is_empty());
    }

    #[test]
    fn malformed_remote_record_is_rejected() {
        let mut registry = GameRegistry::new();
        let mut bad = game(GameId::new(), node(), 100, "ok");
        bad.name = "   ".to_owned();
        assert_eq!(registry.merge(bad, Instant::now()), MergeOutcome::Rejected);
        assert!(registry.is_empty());
    }

    #[test]
    fn delta_sync_returns_newer_records_only() {
        let a = GameId::new();
        let b = GameId::new();
        let origin = node();
        let mut registry = GameRegistry::new();
        let now = Instant::now();
        registry.merge(game(a, origin, 100, "a"), now);
        registry.merge(game(b, origin, 200, "b"), now);
        let delta = registry.since(150, 10, now);
        assert_eq!(delta.len(), 1);
        assert_eq!(delta[0].name, "b");
    }

    #[test]
    fn own_address_is_never_a_peer() {
        let dir = Directory::new(
            node(),
            "self-test",
            "0.0.0.0:47770".parse().expect("addr"),
            None,
        );
        assert!(dir.is_self("127.0.0.1:47770".parse().expect("addr")));
        assert!(!dir.learn_peer(
            "192.0.2.5:47770".parse().expect("addr"),
            PeerOrigin::Gossip,
            false
        ));
        dir.observe_peer("10.0.0.7:47770".parse().expect("addr"), None);
        assert_eq!(dir.alive_peers(), 0);
        assert!(dir.learn_peer(
            "192.0.2.9:47771".parse().expect("addr"),
            PeerOrigin::Gossip,
            false
        ));
    }

    #[test]
    fn directory_announce_heartbeat_withdraw() {
        let dir = Directory::new(
            node(),
            "test",
            "127.0.0.1:4000".parse().expect("addr"),
            None,
        );
        let published = PublishGame {
            id: None,
            name: "Nod lobby".to_owned(),
            kind: GameKind::RedAlert,
            map: "SCM01EA".to_owned(),
            host_name: "Kane".to_owned(),
            address: "203.0.113.7:47771".parse().expect("addr"),
            players: 1,
            max_players: 4,
            state: GameState::Lobby,
            passworded: false,
            version: "1.0".to_owned(),
            tags: vec![],
            ttl_secs: Some(60),
        };
        let info = dir.announce(published).expect("valid");
        assert_eq!(dir.games(&GameFilter::default()).len(), 1);
        let bumped = dir.heartbeat(info.id).expect("heartbeat");
        assert!(bumped.revision > info.revision);
        assert!(dir.withdraw(info.id));
        assert!(dir.games(&GameFilter::default()).is_empty());
        assert!(dir.game(info.id).is_none());
    }
}
