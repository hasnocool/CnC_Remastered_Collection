#![allow(clippy::unwrap_used, clippy::expect_used)]

//! End-to-end ranked ladder over the HTTP API: register, queue, pair, report.

use std::time::{Duration, Instant};

use cnc_net_directory::client::DirectoryClient;
use cnc_net_directory::discovery::DiscoveryConfig;
use cnc_net_directory::matchmaking::{AgentCard, Controller, JoinQueue, MatchState};
use cnc_net_directory::model::GameKind;
use cnc_net_directory::node::{NodeHandle, NodeSettings};

async fn node() -> NodeHandle {
    NodeHandle::bind(NodeSettings {
        name: "lfg-integration".to_owned(),
        gossip_bind: "127.0.0.1:0".parse().expect("addr"),
        advertise: None,
        http_bind: Some("127.0.0.1:0".parse().expect("addr")),
        discovery: DiscoveryConfig::default(),
        torrent: None,
        gossip_interval: Duration::from_millis(40),
        sync_interval: Duration::from_millis(80),
        maintenance_interval: Duration::from_millis(80),
        peer_timeout: Duration::from_secs(5),
        peer_retry_interval: Duration::from_millis(80),
        torrent_interval: Duration::from_secs(60),
        min_alive_peers: 0,
        lfg_state: None,
    })
    .await
    .expect("bind node")
}

#[tokio::test]
async fn two_players_are_paired_and_the_result_moves_the_ladder() {
    let node = node().await;
    let base = format!("http://{}", node.http_addr().expect("http"));
    let client = DirectoryClient::new(base).expect("client");

    let alpha = client
        .register_player("Alpha")
        .await
        .expect("register alpha");
    let bravo = client
        .register_player("Bravo")
        .await
        .expect("register bravo");
    assert_eq!(alpha.name, "Alpha");
    assert_eq!(alpha.ratings.len(), 2);

    for (player, port) in [(&alpha, 1_u16), (&bravo, 2_u16)] {
        client
            .join_queue(&JoinQueue {
                player_id: player.id,
                kind: GameKind::RedAlert,
                address: format!("203.0.113.{port}:47771").parse().expect("addr"),
                map: None,
                note: None,
            })
            .await
            .expect("queue");
    }

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut record = None;
    while Instant::now() < deadline {
        let matches = client
            .matches(Some(alpha.id), None, 10)
            .await
            .expect("matches");
        if let Some(found) = matches.into_iter().next() {
            record = Some(found);
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let record = record.expect("matchmaker paired the two queued players");
    assert_eq!(record.state, MatchState::Pending);
    assert_eq!(record.players.len(), 2);
    assert!(record.game_id.is_some(), "ranked lobby was announced");

    let settled = client
        .report_match(record.id, alpha.id, alpha.id)
        .await
        .expect("report");
    assert_eq!(settled.state, MatchState::Settled);
    assert_eq!(settled.winner, Some(alpha.id));
    assert_eq!(settled.rating_changes.len(), 2);

    let rows = client
        .leaderboard(Some(GameKind::RedAlert), 10)
        .await
        .expect("leaderboard");
    assert_eq!(rows.len(), 2);
    let winner = rows
        .iter()
        .find(|row| row.player_id == alpha.id)
        .expect("winner row");
    let loser = rows
        .iter()
        .find(|row| row.player_id == bravo.id)
        .expect("loser row");
    assert_eq!(winner.wins, 1);
    assert_eq!(loser.losses, 1);
    assert!(winner.rating > loser.rating);
    assert_eq!(winner.rank, 1);

    let status = client.lfg_status().await.expect("status");
    assert_eq!(status.players, 2);
    assert_eq!(status.queued, 0);
    assert_eq!(status.settled_matches, 1);

    node.shutdown().await;
}

#[tokio::test]
async fn ai_agents_use_a_separate_ladder_and_announce_ai_lobbies() {
    let node = node().await;
    let base = format!("http://{}", node.http_addr().expect("http"));
    let client = DirectoryClient::new(base).expect("client");

    let card = AgentCard {
        agent_id: Some(7),
        owner: "HouseNod".to_owned(),
        doctrine: "Aggressive".to_owned(),
        script_hash: "0123456789abcdef0123456789abcdef01234567".to_owned(),
        creator: Some("Kane".to_owned()),
        scenario: Some("SCG01EA".to_owned()),
    };
    let alpha = client
        .register_ai("Agent Alpha", &card)
        .await
        .expect("register ai alpha");
    let bravo = client
        .register_ai("Agent Bravo", &card)
        .await
        .expect("register ai bravo");
    assert_eq!(alpha.controller, Controller::Ai);
    assert_eq!(alpha.agent.as_ref().expect("card").doctrine, "Aggressive");

    for (player, port) in [(&alpha, 21_u16), (&bravo, 22_u16)] {
        client
            .join_queue(&JoinQueue {
                player_id: player.id,
                kind: GameKind::RedAlert,
                address: format!("203.0.113.{port}:47771").parse().expect("addr"),
                map: None,
                note: None,
            })
            .await
            .expect("queue ai");
    }

    let deadline = Instant::now() + Duration::from_secs(5);
    let mut record = None;
    while Instant::now() < deadline {
        match client.matches(Some(alpha.id), None, 10).await {
            Ok(matches) => {
                if let Some(found) = matches.into_iter().next() {
                    record = Some(found);
                    break;
                }
            }
            Err(err) => panic!("match lookup failed: {err}"),
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let record = record.expect("AI matchmaker paired the agents");
    assert_eq!(record.controller, Controller::Ai);
    let game = client
        .game(record.game_id.expect("announced lobby"))
        .await
        .expect("ranked lobby");
    assert!(game.tags.contains(&"ai".to_owned()));

    client
        .report_match(record.id, alpha.id, alpha.id)
        .await
        .expect("report ai result");

    let ai_rows = client
        .leaderboard_for(Controller::Ai, Some(GameKind::RedAlert), 10)
        .await
        .expect("ai leaderboard");
    let human_rows = client
        .leaderboard(Some(GameKind::RedAlert), 10)
        .await
        .expect("human leaderboard");
    assert_eq!(ai_rows.len(), 2);
    assert!(human_rows.is_empty());
    let status = client.lfg_status().await.expect("status");
    assert_eq!(status.ai_players, 2);
    assert_eq!(status.ai_settled_matches, 1);
    assert_eq!(status.settled_matches, 0);

    node.shutdown().await;
}

#[tokio::test]
async fn a_player_cannot_join_twice_while_a_match_is_pending() {
    let node = node().await;
    let base = format!("http://{}", node.http_addr().expect("http"));
    let client = DirectoryClient::new(base).expect("client");
    let player = client.register_player("Solo").await.expect("register");
    let queue = JoinQueue {
        player_id: player.id,
        kind: GameKind::TiberianDawn,
        address: "203.0.113.5:47771".parse().expect("addr"),
        map: None,
        note: None,
    };
    client.join_queue(&queue).await.expect("first queue");
    client.join_queue(&queue).await.expect("queue refresh");
    assert_eq!(client.lfg_status().await.expect("status").queued, 1);
    client.leave_queue(player.id).await.expect("leave");
    assert_eq!(client.lfg_status().await.expect("status").queued, 0);

    node.shutdown().await;
}
