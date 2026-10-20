//! The node runtime: UDP gossip, peer maintenance, discovery loops, and the
//! optional HTTP browse API.
//!
//! A node is cheap to run and needs no configuration to be useful: with LAN
//! discovery enabled two nodes on the same network find each other, and with
//! bootstrap addresses configured they form a wider mesh. Every node is equal;
//! there is no leader, and no node is required for the others to keep serving.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use tokio::net::{TcpListener, UdpSocket};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{debug, info, warn};

use crate::api;
use crate::discovery::{lan, resolve_bootstrap, DiscoveryConfig, LanConfig};
use crate::error::{DirectoryError, Result};
use crate::matchmaking::LfgService;
use crate::model::{now_ms, GameInfo, NodeId, NodeInfo};
use crate::peer::PeerOrigin;
use crate::protocol::{self, Hello, HelloAck, Message};
use crate::registry::Directory;
use crate::torrent::{TorrentConfig, TorrentResolver};

/// Datagram budget for gossip batches, comfortably below a typical MTU.
pub const GOSSIP_BYTES: usize = 1200;
/// How far back a periodic gossip push looks for changed records.
pub const PUSH_WINDOW_MS: i64 = 120_000;
/// Maximum records returned for one delta pull.
pub const MAX_SYNC_RECORDS: usize = 256;
/// Maximum peer addresses shared in one handshake or peer message.
pub const MAX_SHARED_PEERS: usize = 32;

/// Runtime configuration for one node.
#[derive(Debug, Clone)]
pub struct NodeSettings {
    /// Display name shown in peer lists.
    pub name: String,
    /// UDP bind address for gossip.
    pub gossip_bind: SocketAddr,
    /// Address advertised to peers; defaults to the bound address.
    pub advertise: Option<SocketAddr>,
    /// HTTP bind address; `None` disables the browse API.
    pub http_bind: Option<SocketAddr>,
    /// Bootstrap and LAN discovery settings.
    pub discovery: DiscoveryConfig,
    /// Optional BitTorrent fallback.
    pub torrent: Option<TorrentConfig>,
    /// How often to push recent records.
    pub gossip_interval: Duration,
    /// How often to pull a full state from one peer.
    pub sync_interval: Duration,
    /// How often to expire records and probe peers.
    pub maintenance_interval: Duration,
    /// Silence after which an alive peer is marked suspect.
    pub peer_timeout: Duration,
    /// Backoff applied after a missed probe.
    pub peer_retry_interval: Duration,
    /// How often to consult the torrent fallback while short on peers.
    pub torrent_interval: Duration,
    /// Peer count below which the torrent fallback is consulted.
    pub min_alive_peers: usize,
    /// Optional JSON file used to persist ranked ladder state across restarts.
    pub lfg_state: Option<PathBuf>,
}

impl Default for NodeSettings {
    fn default() -> Self {
        Self {
            name: "cnc-directory".to_owned(),
            gossip_bind: SocketAddr::from(([0, 0, 0, 0], 47_770)),
            advertise: None,
            http_bind: Some(SocketAddr::from(([127, 0, 0, 1], 8088))),
            discovery: DiscoveryConfig {
                lan: Some(LanConfig::default()),
                ..DiscoveryConfig::default()
            },
            torrent: None,
            gossip_interval: Duration::from_secs(3),
            sync_interval: Duration::from_secs(15),
            maintenance_interval: Duration::from_secs(5),
            peer_timeout: Duration::from_secs(30),
            peer_retry_interval: Duration::from_secs(5),
            torrent_interval: Duration::from_secs(120),
            min_alive_peers: 2,
            lfg_state: None,
        }
    }
}

#[derive(Clone)]
struct Shared {
    dir: Arc<Directory>,
    lfg: Arc<LfgService>,
    socket: Arc<UdpSocket>,
    settings: Arc<NodeSettings>,
    probes: Arc<Mutex<HashMap<SocketAddr, Instant>>>,
}

/// A running directory node.
pub struct NodeHandle {
    dir: Arc<Directory>,
    lfg: Arc<LfgService>,
    socket: Arc<UdpSocket>,
    shutdown: watch::Sender<bool>,
    tasks: Vec<JoinHandle<()>>,
    gossip_addr: SocketAddr,
    local_addr: SocketAddr,
    http_addr: Option<SocketAddr>,
}

impl NodeHandle {
    /// Bind sockets, perform one bootstrap pass, and start every background loop.
    ///
    /// # Errors
    ///
    /// Fails when the gossip socket, the HTTP listener, or the torrent resolver
    /// cannot be created. Broken individual seeds are logged and do not fail the
    /// node.
    pub async fn bind(settings: NodeSettings) -> Result<Self> {
        let socket = UdpSocket::bind(settings.gossip_bind)
            .await
            .map_err(DirectoryError::Io)?;
        let local_addr = socket.local_addr().map_err(DirectoryError::Io)?;
        let advertise = settings.advertise.unwrap_or(local_addr);

        let http_listener = match settings.http_bind {
            Some(bind) => {
                let listener = TcpListener::bind(bind).await.map_err(DirectoryError::Io)?;
                let addr = listener.local_addr().map_err(DirectoryError::Io)?;
                Some((listener, addr))
            }
            None => None,
        };
        let http_addr = http_listener.as_ref().map(|(_, addr)| *addr);

        if let Some(lan) = settings.discovery.lan.as_ref() {
            if let Err(err) = lan::join_group(&socket, lan.group).await {
                warn!(group = %lan.group, error = %err, "LAN multicast unavailable; continuing with unicast peers");
            }
        }

        let dir = Arc::new(Directory::new(
            NodeId::new(),
            settings.name.clone(),
            advertise,
            http_addr,
        ));
        let lfg = Arc::new(LfgService::new(dir.clone(), settings.lfg_state.clone()));
        let socket = Arc::new(socket);
        let settings = Arc::new(settings);
        let shared = Shared {
            dir: dir.clone(),
            lfg: lfg.clone(),
            socket: socket.clone(),
            settings: settings.clone(),
            probes: Arc::new(Mutex::new(HashMap::new())),
        };

        let (shutdown, shutdown_rx) = watch::channel(false);
        let mut tasks = Vec::new();

        tasks.push(tokio::spawn(recv_loop(shared.clone(), shutdown_rx.clone())));
        bootstrap(&shared).await;
        tasks.push(tokio::spawn(gossip_loop(
            shared.clone(),
            shutdown_rx.clone(),
        )));
        tasks.push(tokio::spawn(sync_loop(shared.clone(), shutdown_rx.clone())));
        tasks.push(tokio::spawn(maintenance_loop(
            shared.clone(),
            shutdown_rx.clone(),
        )));
        tasks.push(tokio::spawn(lfg_loop(shared.clone(), shutdown_rx.clone())));
        if settings.discovery.lan.is_some() {
            tasks.push(tokio::spawn(lan_loop(shared.clone(), shutdown_rx.clone())));
        }
        if let Some(torrent) = settings.torrent.clone() {
            tasks.push(tokio::spawn(torrent_loop(
                shared.clone(),
                shutdown_rx.clone(),
                torrent,
            )));
        }
        if let Some((listener, _)) = http_listener {
            tasks.push(tokio::spawn(api::serve(
                listener,
                dir.clone(),
                lfg.clone(),
                shutdown_rx.clone(),
            )));
        }

        Ok(Self {
            dir,
            lfg,
            socket,
            shutdown,
            tasks,
            gossip_addr: advertise,
            local_addr,
            http_addr,
        })
    }

    /// The shared directory state.
    #[must_use]
    pub fn directory(&self) -> Arc<Directory> {
        self.dir.clone()
    }

    /// The node-local ranked ladder.
    #[must_use]
    pub fn lfg(&self) -> Arc<LfgService> {
        self.lfg.clone()
    }

    /// This node's identity.
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        self.dir.node_id()
    }

    /// Address advertised to peers.
    #[must_use]
    pub const fn gossip_addr(&self) -> SocketAddr {
        self.gossip_addr
    }

    /// Address the gossip socket is actually bound to.
    #[must_use]
    pub const fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// HTTP browse address, when enabled.
    #[must_use]
    pub const fn http_addr(&self) -> Option<SocketAddr> {
        self.http_addr
    }

    /// Broadcast a graceful Bye and stop every task.
    pub async fn shutdown(mut self) {
        if let Err(err) = self.lfg.save() {
            warn!(error = %err, "failed to persist ranked ladder on shutdown");
        }
        let peers: Vec<SocketAddr> = self.dir.peers().into_iter().map(|p| p.addr).collect();
        let bye = Message::Bye {
            node: self.dir.node_id(),
        };
        for peer in peers {
            send_msg_raw(&self.socket, &self.dir, peer, &bye).await;
        }
        let _ = self.shutdown.send(true);
        for task in self.tasks.drain(..) {
            let _ = tokio::time::timeout(Duration::from_secs(2), task).await;
        }
    }
}

/// Run a node until Ctrl-C.
///
/// # Errors
///
/// Propagates bind failures.
pub async fn run_forever(settings: NodeSettings) -> Result<()> {
    let node = NodeHandle::bind(settings).await?;
    info!(
        node = %node.node_id(),
        gossip = %node.gossip_addr(),
        local = %node.local_addr(),
        http = ?node.http_addr(),
        "cnc-directory node running"
    );
    if let Err(err) = tokio::signal::ctrl_c().await {
        warn!(error = %err, "could not install Ctrl-C handler");
    }
    node.shutdown().await;
    Ok(())
}

async fn bootstrap(shared: &Shared) {
    let report = resolve_bootstrap(&shared.settings.discovery).await;
    for addr in &report.addrs {
        shared.dir.learn_peer(*addr, PeerOrigin::Static, true);
    }
    for addr in &report.addrs {
        send_hello(shared, *addr, false).await;
    }
    if !report.errors.is_empty() {
        info!(errors = ?report.errors, "some bootstrap sources failed");
    }
    if report.addrs.is_empty() {
        info!("no bootstrap peers configured; waiting for LAN or inbound peers");
    }
}

async fn recv_loop(shared: Shared, mut shutdown: watch::Receiver<bool>) {
    let mut buf = vec![0u8; protocol::MAX_DATAGRAM];
    loop {
        tokio::select! {
            biased;
            _ = shutdown.changed() => break,
            received = shared.socket.recv_from(&mut buf) => {
                match received {
                    Ok((len, from)) => {
                        shared.dir.metrics().note_in(len);
                        match protocol::decode(&buf[..len]) {
                            Ok(message) => {
                                let identity = match &message {
                                    Message::Hello(hello) => Some(hello.node.clone()),
                                    Message::HelloAck(ack) => Some(ack.node.clone()),
                                    _ => None,
                                };
                                shared.dir.observe_peer(from, identity);
                                handle_message(&shared, from, message).await;
                            }
                            Err(err) => debug!(peer = %from, error = %err, "dropping invalid datagram"),
                        }
                    }
                    Err(err) => {
                        warn!(error = %err, "UDP receive failed");
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }
                }
            }
        }
    }
}

async fn handle_message(shared: &Shared, from: SocketAddr, message: Message) {
    match message {
        Message::Hello(hello) => {
            let discovered: Vec<SocketAddr> = hello
                .peers
                .iter()
                .filter(|addr| **addr != from && !shared.dir.knows_peer(**addr))
                .take(4)
                .copied()
                .collect();
            shared.dir.learn_peers(&hello.peers, PeerOrigin::Gossip);
            send_hello(shared, from, true).await;
            if hello.digest != shared.dir.digest() {
                let games = shared.dir.all_games();
                send_games(shared, from, &games).await;
            }
            for addr in discovered {
                send_hello(shared, addr, false).await;
            }
        }
        Message::HelloAck(ack) => {
            let latency = {
                let mut probes = shared.probes.lock();
                probes.remove(&from)
            };
            if let Some(started) = latency {
                shared.dir.note_latency(from, started.elapsed());
            }
            let discovered: Vec<SocketAddr> = ack
                .peers
                .iter()
                .filter(|addr| **addr != from && !shared.dir.knows_peer(**addr))
                .take(4)
                .copied()
                .collect();
            shared.dir.learn_peers(&ack.peers, PeerOrigin::Gossip);
            if ack.digest != shared.dir.digest() {
                let games = shared.dir.all_games();
                send_games(shared, from, &games).await;
            }
            for addr in discovered {
                send_hello(shared, addr, false).await;
            }
        }
        Message::Ping { nonce } => {
            send_msg(shared, from, &Message::Pong { nonce }).await;
        }
        Message::Pong { .. } => {}
        Message::Peers(addrs) => {
            let discovered: Vec<SocketAddr> = addrs
                .iter()
                .filter(|addr| **addr != from && !shared.dir.knows_peer(**addr))
                .take(4)
                .copied()
                .collect();
            shared.dir.learn_peers(&addrs, PeerOrigin::Gossip);
            for addr in discovered {
                send_hello(shared, addr, false).await;
            }
        }
        Message::Games(games) => {
            let stats = shared.dir.merge_games(games.clone());
            if stats.inserted > 0 {
                let peers = {
                    let mut rng = rand::thread_rng();
                    shared.dir.gossip_targets(3, &mut rng)
                };
                for peer in peers.into_iter().filter(|p| *p != from).take(2) {
                    send_games(shared, peer, &games).await;
                }
            }
        }
        Message::DeltaRequest { since_ms } => {
            let games = shared.dir.sync_batch(since_ms, MAX_SYNC_RECORDS);
            send_games(shared, from, &games).await;
        }
        Message::Bye { .. } => {
            shared.dir.mark_peer_dead(from);
            debug!(peer = %from, "peer said goodbye");
        }
    }
}

async fn gossip_loop(shared: Shared, mut shutdown: watch::Receiver<bool>) {
    let mut ticker = interval(shared.settings.gossip_interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_ok() || *shutdown.borrow() { break; }
            }
            _ = ticker.tick() => {}
        }
        let targets = {
            let mut rng = rand::thread_rng();
            shared.dir.gossip_targets(3, &mut rng)
        };
        if targets.is_empty() {
            continue;
        }
        let delta = shared.dir.sync_batch(now_ms() - PUSH_WINDOW_MS, 128);
        if !delta.is_empty() {
            for target in &targets {
                send_games(&shared, *target, &delta).await;
            }
        }
        let peers = known_peer_addrs(&shared, MAX_SHARED_PEERS);
        if !peers.is_empty() {
            if let Some(target) = targets.first() {
                send_msg(&shared, *target, &Message::Peers(peers)).await;
            }
        }
    }
}

async fn sync_loop(shared: Shared, mut shutdown: watch::Receiver<bool>) {
    let mut ticker = interval(shared.settings.sync_interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_ok() || *shutdown.borrow() { break; }
            }
            _ = ticker.tick() => {}
        }
        let target = {
            let mut rng = rand::thread_rng();
            shared.dir.gossip_targets(1, &mut rng).into_iter().next()
        };
        if let Some(target) = target {
            send_msg(&shared, target, &Message::DeltaRequest { since_ms: 0 }).await;
        }
    }
}

async fn maintenance_loop(shared: Shared, mut shutdown: watch::Receiver<bool>) {
    let mut ticker = interval(shared.settings.maintenance_interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_ok() || *shutdown.borrow() { break; }
            }
            _ = ticker.tick() => {}
        }
        let expired = shared.dir.expire();
        let purged = shared.dir.purge_peers();
        if expired + purged > 0 {
            debug!(expired, purged, "maintenance swept stale records");
        }
        for addr in shared.dir.silent_peers(shared.settings.peer_timeout) {
            if let Some(state) = shared
                .dir
                .fail_peer(addr, shared.settings.peer_retry_interval)
            {
                debug!(peer = %addr, ?state, "peer missed liveness window");
            }
        }
        for addr in shared.dir.retry_candidates(8) {
            shared
                .dir
                .mark_peer_attempt(addr, shared.settings.peer_retry_interval);
            send_hello(&shared, addr, false).await;
        }
    }
}

async fn lfg_loop(shared: Shared, mut shutdown: watch::Receiver<bool>) {
    let mut ticker = interval(Duration::from_secs(1));
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    let mut ticks_since_save = 0_u32;
    loop {
        tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_ok() || *shutdown.borrow() { break; }
            }
            _ = ticker.tick() => {}
        }
        let found = shared.lfg.tick();
        if !found.is_empty() {
            debug!(matches = found.len(), "ranked matchmaking tick");
        }
        ticks_since_save = ticks_since_save.saturating_add(1);
        if ticks_since_save >= 30 {
            ticks_since_save = 0;
            if let Err(err) = shared.lfg.save() {
                warn!(error = %err, "failed to persist ranked ladder");
            }
        }
    }
    if let Err(err) = shared.lfg.save() {
        warn!(error = %err, "failed to persist ranked ladder on shutdown");
    }
}

async fn lan_loop(shared: Shared, mut shutdown: watch::Receiver<bool>) {
    let Some(lan) = shared.settings.discovery.lan.clone() else {
        return;
    };
    let mut ticker = interval(lan.announce_interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_ok() || *shutdown.borrow() { break; }
            }
            _ = ticker.tick() => {}
        }
        match protocol::encode(&hello_message(&shared)) {
            Ok(bytes) => match shared.socket.send_to(&bytes, lan.group).await {
                Ok(sent) => shared.dir.metrics().note_out(sent),
                Err(err) => debug!(group = %lan.group, error = %err, "LAN announce failed"),
            },
            Err(err) => debug!(error = %err, "LAN announce encode failed"),
        }
    }
}

async fn torrent_loop(shared: Shared, mut shutdown: watch::Receiver<bool>, config: TorrentConfig) {
    let resolver = match TorrentResolver::new(config) {
        Ok(resolver) => resolver,
        Err(err) => {
            warn!(error = %err, "torrent fallback disabled");
            return;
        }
    };
    let mut ticker = interval(shared.settings.torrent_interval);
    ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
    loop {
        tokio::select! {
            changed = shutdown.changed() => {
                if changed.is_ok() || *shutdown.borrow() { break; }
            }
            _ = ticker.tick() => {}
        }
        if shared.dir.alive_peers() >= shared.settings.min_alive_peers {
            continue;
        }
        match resolver.resolve().await {
            Ok(list) => {
                let mut contacted = 0;
                for addr in &list.nodes {
                    if *addr == shared.dir.gossip_addr() {
                        continue;
                    }
                    if shared.dir.learn_peer(*addr, PeerOrigin::Torrent, false) {
                        send_hello(&shared, *addr, false).await;
                        contacted += 1;
                    }
                }
                info!(
                    nodes = list.nodes.len(),
                    contacted, "torrent fallback discovered peers"
                );
            }
            Err(err) => warn!(error = %err, "torrent fallback could not resolve a node list"),
        }
    }
}

fn hello_message(shared: &Shared) -> Message {
    Message::Hello(Hello {
        node: shared.dir.self_info(),
        peers: known_peer_addrs(shared, MAX_SHARED_PEERS),
        digest: shared.dir.digest(),
    })
}

fn known_peer_addrs(shared: &Shared, limit: usize) -> Vec<SocketAddr> {
    let mut addrs: Vec<SocketAddr> = shared
        .dir
        .peers()
        .into_iter()
        .filter(|peer| peer.state != crate::peer::PeerState::Dead)
        .map(|peer| peer.addr)
        .take(limit)
        .collect();
    addrs.sort();
    addrs
}

async fn send_hello(shared: &Shared, addr: SocketAddr, ack: bool) {
    shared.probes.lock().insert(addr, Instant::now());
    let node = shared.dir.self_info();
    let peers = known_peer_addrs(shared, MAX_SHARED_PEERS);
    let digest = shared.dir.digest();
    let message = if ack {
        Message::HelloAck(HelloAck {
            node,
            peers,
            digest,
        })
    } else {
        Message::Hello(Hello {
            node,
            peers,
            digest,
        })
    };
    send_msg(shared, addr, &message).await;
}

async fn send_games(shared: &Shared, addr: SocketAddr, games: &[GameInfo]) {
    if games.is_empty() {
        return;
    }
    for chunk in protocol::chunk_games(games, GOSSIP_BYTES) {
        send_msg(shared, addr, &Message::Games(chunk)).await;
    }
}

async fn send_msg(shared: &Shared, addr: SocketAddr, message: &Message) {
    send_msg_raw(&shared.socket, &shared.dir, addr, message).await;
}

async fn send_msg_raw(socket: &UdpSocket, dir: &Directory, addr: SocketAddr, message: &Message) {
    match protocol::encode(message) {
        Ok(bytes) => match socket.send_to(&bytes, addr).await {
            Ok(sent) => dir.metrics().note_out(sent),
            Err(err) => {
                dir.metrics().note_peer_error();
                debug!(peer = %addr, tag = message.tag(), error = %err, "UDP send failed");
            }
        },
        Err(err) => {
            dir.metrics().note_peer_error();
            warn!(tag = message.tag(), error = %err, "failed to encode datagram");
        }
    }
}

/// Peer identity helper used by tests and the API.
#[must_use]
pub fn describe_node(info: &NodeInfo) -> String {
    format!("{} ({})", info.name, info.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(port: u16) -> NodeSettings {
        NodeSettings {
            name: format!("test-{port}"),
            gossip_bind: SocketAddr::from(([127, 0, 0, 1], port)),
            advertise: None,
            http_bind: None,
            discovery: DiscoveryConfig::default(),
            torrent: None,
            gossip_interval: Duration::from_millis(50),
            sync_interval: Duration::from_millis(100),
            maintenance_interval: Duration::from_millis(100),
            peer_timeout: Duration::from_secs(5),
            peer_retry_interval: Duration::from_millis(100),
            torrent_interval: Duration::from_secs(60),
            min_alive_peers: 0,
            lfg_state: None,
        }
    }

    #[tokio::test]
    async fn two_nodes_converge_on_a_game() {
        use crate::model::{GameKind, GameState, PublishGame};

        let a = NodeHandle::bind(settings(0)).await.expect("bind a");
        let mut b_settings = settings(0);
        b_settings.discovery.static_peers = vec![a.gossip_addr().to_string()];
        let b = NodeHandle::bind(b_settings).await.expect("bind b");

        let published = PublishGame {
            id: None,
            name: "Convergence lobby".to_owned(),
            kind: GameKind::RedAlert,
            map: "SCM01EA".to_owned(),
            host_name: "host".to_owned(),
            address: "203.0.113.9:47771".parse().expect("addr"),
            players: 1,
            max_players: 4,
            state: GameState::Lobby,
            passworded: false,
            version: "1.0".to_owned(),
            tags: vec![],
            ttl_secs: None,
        };
        let game = a.directory().announce(published).expect("announce");

        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            if b.directory().game(game.id).is_some() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        assert!(
            b.directory().game(game.id).is_some(),
            "game did not replicate"
        );
        assert_eq!(a.directory().alive_peers(), 1);
        assert_eq!(b.directory().alive_peers(), 1);

        a.shutdown().await;
        b.shutdown().await;
    }
}
