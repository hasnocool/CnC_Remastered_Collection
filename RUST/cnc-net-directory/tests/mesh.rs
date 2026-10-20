#![allow(clippy::unwrap_used, clippy::expect_used)]

//! End-to-end mesh behaviour: convergence, withdrawal, peer-of-peer discovery,
//! and survival of a node failure.

use std::net::SocketAddr;
use std::time::{Duration, Instant};

use cnc_net_directory::discovery::DiscoveryConfig;
use cnc_net_directory::model::{GameFilter, GameKind, GameState, PublishGame};
use cnc_net_directory::node::{NodeHandle, NodeSettings};

fn base_settings(name: &str) -> NodeSettings {
    NodeSettings {
        name: name.to_owned(),
        gossip_bind: "127.0.0.1:0".parse().unwrap(),
        advertise: None,
        http_bind: Some("127.0.0.1:0".parse().unwrap()),
        discovery: DiscoveryConfig::default(),
        torrent: None,
        gossip_interval: Duration::from_millis(40),
        sync_interval: Duration::from_millis(80),
        maintenance_interval: Duration::from_millis(80),
        peer_timeout: Duration::from_secs(3),
        peer_retry_interval: Duration::from_millis(80),
        torrent_interval: Duration::from_secs(30),
        min_alive_peers: 0,
        lfg_state: None,
    }
}

async fn spawn(name: &str, bootstrap: &[SocketAddr]) -> NodeHandle {
    let mut settings = base_settings(name);
    settings.discovery.static_peers = bootstrap.iter().map(ToString::to_string).collect();
    NodeHandle::bind(settings).await.expect("bind node")
}

fn lobby(name: &str, address: &str) -> PublishGame {
    PublishGame {
        id: None,
        name: name.to_owned(),
        kind: GameKind::TiberianDawn,
        map: "SCG05EA".to_owned(),
        host_name: "Kane".to_owned(),
        address: address.parse().unwrap(),
        players: 2,
        max_players: 4,
        state: GameState::Lobby,
        passworded: false,
        version: "1.0".to_owned(),
        tags: vec!["2v2".to_owned()],
        ttl_secs: Some(300),
    }
}

async fn wait_for(label: &str, mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(8);
    while Instant::now() < deadline {
        if condition() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
    panic!("timed out waiting for {label}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn a_chain_of_nodes_converges_on_one_lobby() {
    let a = spawn("a", &[]).await;
    let b = spawn("b", &[a.gossip_addr()]).await;
    let c = spawn("c", &[b.gossip_addr()]).await;

    let game = a
        .directory()
        .announce(lobby("Chain lobby", "203.0.113.30:47771"))
        .expect("announce");

    wait_for("b to receive the lobby", || {
        b.directory().game(game.id).is_some()
    })
    .await;
    wait_for("c to receive the lobby via b", || {
        c.directory().game(game.id).is_some()
    })
    .await;

    // c bootstrapped only to b, but should still learn about a through gossip.
    wait_for("c to learn the address of a", || {
        c.directory()
            .peers()
            .iter()
            .any(|peer| peer.addr == a.gossip_addr())
    })
    .await;

    assert_eq!(c.directory().games(&GameFilter::default()).len(), 1);
    assert!(
        !a.directory()
            .peers()
            .iter()
            .any(|peer| peer.addr == a.gossip_addr()),
        "a node must not track its own address as a peer"
    );

    // Withdrawal must propagate as a tombstone.
    assert!(a.directory().withdraw(game.id));
    wait_for("b to drop the lobby", || {
        b.directory().game(game.id).is_none()
    })
    .await;
    wait_for("c to drop the lobby", || {
        c.directory().game(game.id).is_none()
    })
    .await;

    a.shutdown().await;
    b.shutdown().await;
    c.shutdown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn surviving_nodes_keep_serving_after_one_dies() {
    let a = spawn("a", &[]).await;
    let b = spawn("b", &[a.gossip_addr()]).await;
    let c = spawn("c", &[a.gossip_addr(), b.gossip_addr()]).await;

    let game = a
        .directory()
        .announce(lobby("Failover lobby", "203.0.113.31:47771"))
        .expect("announce");
    wait_for("b to receive the lobby", || {
        b.directory().game(game.id).is_some()
    })
    .await;
    wait_for("c to receive the lobby", || {
        c.directory().game(game.id).is_some()
    })
    .await;

    // The publishing node dies. The directory must survive elsewhere.
    a.shutdown().await;

    assert!(
        b.directory().game(game.id).is_some(),
        "b lost the lobby when a shut down"
    );
    assert!(
        c.directory().game(game.id).is_some(),
        "c lost the lobby when a shut down"
    );

    // A new lobby published on c must still reach b without a.
    let second = c
        .directory()
        .announce(lobby("Post-failover lobby", "203.0.113.32:47771"))
        .expect("announce");
    wait_for("b to receive the lobby published after a died", || {
        b.directory().game(second.id).is_some()
    })
    .await;

    b.shutdown().await;
    c.shutdown().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn restarted_node_catches_up_from_a_peer() {
    let a = spawn("a", &[]).await;
    let b = spawn("b", &[a.gossip_addr()]).await;

    let game = b
        .directory()
        .announce(lobby("Late joiner lobby", "203.0.113.33:47771"))
        .expect("announce");
    wait_for("a to receive from b", || {
        a.directory().game(game.id).is_some()
    })
    .await;

    b.shutdown().await;

    // A fresh node with the same bootstrap point should get the full state.
    let c = spawn("c", &[a.gossip_addr()]).await;
    wait_for("c to pull the existing registry", || {
        c.directory().game(game.id).is_some()
    })
    .await;

    a.shutdown().await;
    c.shutdown().await;
}
