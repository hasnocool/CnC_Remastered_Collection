#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Torrent fallback: plain web-seed JSON, and the qBittorrent download path.

use std::net::SocketAddr;
use std::time::Duration;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use cnc_net_directory::discovery::DiscoveryConfig;
use cnc_net_directory::model::{GameKind, GameState, PublishGame};
use cnc_net_directory::node::{NodeHandle, NodeSettings};
use cnc_net_directory::torrent::{
    build_bundle, write_bundle, NodeList, QBittorrentConfig, TorrentConfig, TorrentResolver,
    TorrentSource,
};

fn settings(name: &str) -> NodeSettings {
    NodeSettings {
        name: name.to_owned(),
        gossip_bind: "127.0.0.1:0".parse().unwrap(),
        advertise: None,
        http_bind: None,
        discovery: DiscoveryConfig::default(),
        torrent: None,
        gossip_interval: Duration::from_millis(40),
        sync_interval: Duration::from_millis(80),
        maintenance_interval: Duration::from_millis(80),
        peer_timeout: Duration::from_secs(3),
        peer_retry_interval: Duration::from_millis(80),
        torrent_interval: Duration::from_millis(100),
        min_alive_peers: 1,
        lfg_state: None,
    }
}

fn lobby() -> PublishGame {
    PublishGame {
        id: None,
        name: "Fallback lobby".to_owned(),
        kind: GameKind::RedAlert,
        map: "SCM02EA".to_owned(),
        host_name: "Tanya".to_owned(),
        address: "203.0.113.40:47771".parse().unwrap(),
        players: 1,
        max_players: 4,
        state: GameState::Lobby,
        passworded: false,
        version: "1.0".to_owned(),
        tags: vec![],
        ttl_secs: Some(300),
    }
}

async fn serve_json_node_list(list: NodeList) -> SocketAddr {
    let app = Router::new()
        .route(
            "/nodes.json",
            get(|State(list): State<NodeList>| async move { Json(list) }),
        )
        .with_state(list);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    addr
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn isolated_node_finds_the_mesh_through_a_web_seed() {
    let anchor = NodeHandle::bind(settings("anchor")).await.expect("anchor");
    let game = anchor.directory().announce(lobby()).expect("announce");

    let webseed =
        serve_json_node_list(NodeList::new(vec![anchor.gossip_addr()], vec![], None)).await;

    let mut loner_settings = settings("loner");
    loner_settings.discovery.static_peers.clear();
    loner_settings.torrent = Some(TorrentConfig {
        source: TorrentSource::JsonUrl(format!("http://{webseed}/nodes.json")),
        nodes_filename: "nodes.json".to_owned(),
        qbittorrent: None,
        timeout: Duration::from_secs(5),
        poll_interval: Duration::from_millis(50),
    });
    let loner = NodeHandle::bind(loner_settings).await.expect("loner");

    let deadline = std::time::Instant::now() + Duration::from_secs(8);
    while std::time::Instant::now() < deadline {
        if loner.directory().game(game.id).is_some() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(
        loner.directory().game(game.id).is_some(),
        "torrent fallback did not recover the lobby"
    );
    assert_eq!(loner.directory().alive_peers(), 1);

    loner.shutdown().await;
    anchor.shutdown().await;
}

#[derive(Clone)]
struct QbState {
    save_path: String,
    content_path: String,
}

async fn serve_mock_qbittorrent(state: QbState) -> SocketAddr {
    let app = Router::new()
        .route("/api/v2/auth/login", post(|| async { "Ok." }))
        .route("/api/v2/torrents/add", post(|| async { "Ok." }))
        .route(
            "/api/v2/torrents/info",
            get(|State(state): State<QbState>| async move {
                Json(vec![serde_json::json!({
                    "hash": "00",
                    "name": "nodes.json",
                    "save_path": state.save_path,
                    "content_path": state.content_path,
                    "progress": 1.0,
                })])
            }),
        )
        .route(
            "/api/v2/torrents/files",
            get(|| async {
                Json(vec![serde_json::json!({
                    "name": "nodes.json",
                    "progress": 1.0,
                })])
            }),
        )
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    addr
}

#[tokio::test]
async fn qbittorrent_path_recovers_the_node_list() {
    let dir = std::env::temp_dir().join(format!("cnc-directory-qb-{}", uuid::Uuid::new_v4()));
    let list = NodeList::new(vec!["127.0.0.1:47770".parse().unwrap()], vec![], None);
    let bundle = build_bundle(&list, "nodes.json", &[], &[], 16 * 1024).expect("bundle");
    write_bundle(&dir, &bundle).expect("write bundle");
    let content_path = dir.join("nodes.json");

    let qb = serve_mock_qbittorrent(QbState {
        save_path: dir.to_string_lossy().into_owned(),
        content_path: content_path.to_string_lossy().into_owned(),
    })
    .await;

    let resolver = TorrentResolver::new(TorrentConfig {
        source: TorrentSource::Magnet(bundle.magnet.clone()),
        nodes_filename: "nodes.json".to_owned(),
        qbittorrent: Some(QBittorrentConfig {
            base_url: format!("http://{qb}"),
            username: "admin".to_owned(),
            password: "admin".to_owned(),
        }),
        timeout: Duration::from_secs(5),
        poll_interval: Duration::from_millis(50),
    })
    .expect("resolver");

    let resolved = resolver.resolve().await.expect("resolve via qbittorrent");
    assert_eq!(resolved.nodes, list.nodes);

    let _ = std::fs::remove_dir_all(&dir);
}
