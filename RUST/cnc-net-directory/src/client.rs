//! Small HTTP client for browsers, hosts, and operators.
//!
//! The CLI can point at several nodes at once; [DirectoryClient::first_available]
//! picks the first healthy one, which is how a client survives one directory
//! node going down.

use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{DirectoryError, Result};
use crate::matchmaking::{
    AgentCard, Controller, JoinQueue, LeaderboardRow, LfgEntry, LfgStatus, MatchId, PlayerId,
    PlayerProfile, RankedMatch,
};
use crate::model::{GameFilter, GameId, GameInfo, PublishGame};

/// Client for one directory node.
#[derive(Debug, Clone)]
pub struct DirectoryClient {
    base: String,
    http: reqwest::Client,
}

#[derive(Debug, serde::Deserialize)]
struct GamesBody {
    games: Vec<GameInfo>,
}

#[derive(Debug, serde::Deserialize)]
struct LeaderboardBody {
    rows: Vec<LeaderboardRow>,
}

#[derive(Debug, serde::Deserialize)]
struct MatchesBody {
    matches: Vec<RankedMatch>,
}

impl DirectoryClient {
    /// Create a client for `base`, accepting a bare `host:port`.
    ///
    /// # Errors
    ///
    /// Fails when the HTTP client cannot be constructed.
    pub fn new(base: impl Into<String>) -> Result<Self> {
        let raw = base.into();
        let trimmed = raw.trim().trim_end_matches('/').to_owned();
        let base = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            trimmed
        } else {
            format!("http://{trimmed}")
        };
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent(concat!("cnc-directory-cli/", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self { base, http })
    }

    /// Base URL without a trailing slash.
    #[must_use]
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Pick the first node in `bases` that answers a health check.
    ///
    /// # Errors
    ///
    /// Returns the last error when every node is unreachable.
    pub async fn first_available(bases: &[String]) -> Result<Self> {
        let mut last_error = None;
        for base in bases {
            match Self::new(base.clone()) {
                Ok(client) => match client.health().await {
                    Ok(_) => return Ok(client),
                    Err(err) => last_error = Some(err),
                },
                Err(err) => last_error = Some(err),
            }
        }
        Err(last_error.unwrap_or_else(|| {
            DirectoryError::Config("no directory node addresses supplied".to_owned())
        }))
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
    ) -> Result<T> {
        let response = self
            .http
            .get(format!("{}{path}", self.base))
            .query(query)
            .send()
            .await?;
        decode(response).await
    }

    async fn post_json<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .http
            .post(format!("{}{path}", self.base))
            .json(body)
            .send()
            .await?;
        decode(response).await
    }

    /// Health probe.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn health(&self) -> Result<serde_json::Value> {
        self.get_json("/health", &[]).await
    }

    /// Browse lobbies matching a filter.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn games(&self, filter: &GameFilter) -> Result<Vec<GameInfo>> {
        let body: GamesBody = self
            .get_json("/api/v1/games", &filter_query(filter))
            .await?;
        Ok(body.games)
    }

    /// Fetch one lobby.
    ///
    /// # Errors
    ///
    /// Returns [DirectoryError::Remote] with status 404 when unknown.
    pub async fn game(&self, id: GameId) -> Result<GameInfo> {
        self.get_json(&format!("/api/v1/games/{id}"), &[]).await
    }

    /// Publish a lobby announcement.
    ///
    /// # Errors
    ///
    /// Returns [DirectoryError::Remote] with status 422 when invalid.
    pub async fn publish(&self, game: &PublishGame) -> Result<GameInfo> {
        let response = self
            .http
            .post(format!("{}/api/v1/games", self.base))
            .json(game)
            .send()
            .await?;
        decode(response).await
    }

    /// Refresh a published lobby.
    ///
    /// # Errors
    ///
    /// Returns [DirectoryError::Remote] with status 404 when unknown.
    pub async fn heartbeat(&self, id: GameId) -> Result<GameInfo> {
        let response = self
            .http
            .post(format!("{}/api/v1/games/{id}/heartbeat", self.base))
            .send()
            .await?;
        decode(response).await
    }

    /// Withdraw a published lobby.
    ///
    /// # Errors
    ///
    /// Returns [DirectoryError::Remote] with status 404 when unknown.
    pub async fn withdraw(&self, id: GameId) -> Result<()> {
        let response = self
            .http
            .delete(format!("{}/api/v1/games/{id}", self.base))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(DirectoryError::Remote {
                status: status.as_u16(),
                body,
            })
        }
    }

    /// Node and peer status.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn nodes(&self) -> Result<serde_json::Value> {
        self.get_json("/api/v1/nodes", &[]).await
    }

    /// Aggregate counters.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn stats(&self) -> Result<serde_json::Value> {
        self.get_json("/api/v1/stats", &[]).await
    }

    /// Register a ranked-ladder player.
    ///
    /// # Errors
    ///
    /// Returns a transport, validation, or status error.
    pub async fn register_player(&self, name: &str) -> Result<PlayerProfile> {
        self.post_json(
            "/api/v1/lfg/players",
            &serde_json::json!({ "name": name, "controller": "human" }),
        )
        .await
    }

    /// Register an AI competitor on the separate AI-only ladder.
    ///
    /// # Errors
    ///
    /// Returns a transport, validation, or status error.
    pub async fn register_ai(&self, name: &str, agent: &AgentCard) -> Result<PlayerProfile> {
        self.post_json(
            "/api/v1/lfg/players",
            &serde_json::json!({
                "name": name,
                "controller": "ai",
                "agent": agent,
            }),
        )
        .await
    }

    /// Fetch one ranked-ladder player.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn lfg_player(&self, id: PlayerId) -> Result<PlayerProfile> {
        self.get_json(&format!("/api/v1/lfg/players/{id}"), &[])
            .await
    }

    /// Join (or refresh) the ranked queue.
    ///
    /// # Errors
    ///
    /// Returns a transport, validation, or status error.
    pub async fn join_queue(&self, request: &JoinQueue) -> Result<LfgEntry> {
        let body = serde_json::json!({
            "player_id": request.player_id.to_string(),
            "kind": request.kind.as_str(),
            "address": request.address.to_string(),
            "map": request.map.clone(),
            "note": request.note.clone(),
        });
        self.post_json("/api/v1/lfg/queue", &body).await
    }

    /// Leave the ranked queue.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn leave_queue(&self, id: PlayerId) -> Result<()> {
        let response = self
            .http
            .delete(format!("{}/api/v1/lfg/queue/{id}", self.base))
            .send()
            .await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let body = response.text().await.unwrap_or_default();
            Err(DirectoryError::Remote {
                status: status.as_u16(),
                body,
            })
        }
    }

    /// Ranked leaderboard for one title (or every title when omitted).
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn leaderboard(
        &self,
        kind: Option<crate::model::GameKind>,
        limit: usize,
    ) -> Result<Vec<LeaderboardRow>> {
        self.leaderboard_for(Controller::Human, kind, limit).await
    }

    /// Ranked leaderboard for one controller.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn leaderboard_for(
        &self,
        controller: Controller,
        kind: Option<crate::model::GameKind>,
        limit: usize,
    ) -> Result<Vec<LeaderboardRow>> {
        let query = vec![
            ("controller".to_owned(), controller.as_str().to_owned()),
            (
                "kind".to_owned(),
                kind.map(|kind| kind.as_str().to_owned())
                    .unwrap_or_default(),
            ),
            ("limit".to_owned(), limit.to_string()),
        ];
        let body: LeaderboardBody = self.get_json("/api/v1/lfg/leaderboard", &query).await?;
        Ok(body.rows)
    }

    /// Ranked ladder counters.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn lfg_status(&self) -> Result<LfgStatus> {
        self.get_json("/api/v1/lfg/status", &[]).await
    }

    /// Report a ranked match result.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn report_match(
        &self,
        id: MatchId,
        reporter: PlayerId,
        winner: PlayerId,
    ) -> Result<RankedMatch> {
        self.post_json(
            &format!("/api/v1/lfg/matches/{id}/report"),
            &serde_json::json!({
                "reporter": reporter.to_string(),
                "winner": winner.to_string(),
            }),
        )
        .await
    }

    /// Ranked match history for a player and/or title, newest first.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn matches(
        &self,
        player_id: Option<PlayerId>,
        kind: Option<crate::model::GameKind>,
        limit: usize,
    ) -> Result<Vec<RankedMatch>> {
        let query = vec![
            (
                "player_id".to_owned(),
                player_id.map(|id| id.to_string()).unwrap_or_default(),
            ),
            (
                "kind".to_owned(),
                kind.map(|kind| kind.as_str().to_owned())
                    .unwrap_or_default(),
            ),
            ("limit".to_owned(), limit.to_string()),
        ];
        let body: MatchesBody = self.get_json("/api/v1/lfg/matches", &query).await?;
        Ok(body.matches)
    }

    /// Fetch one ranked match.
    ///
    /// # Errors
    ///
    /// Returns a transport or status error.
    pub async fn match_record(&self, id: MatchId) -> Result<RankedMatch> {
        self.get_json(&format!("/api/v1/lfg/matches/{id}"), &[])
            .await
    }
}

async fn decode<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    let bytes = response.bytes().await?;
    if !status.is_success() {
        return Err(DirectoryError::Remote {
            status: status.as_u16(),
            body: String::from_utf8_lossy(&bytes).into_owned(),
        });
    }
    Ok(serde_json::from_slice(&bytes)?)
}

fn filter_query(filter: &GameFilter) -> Vec<(String, String)> {
    let mut query = Vec::new();
    if let Some(state) = filter.state {
        query.push(("state".to_owned(), state.as_str().to_owned()));
    }
    if let Some(kind) = filter.kind {
        query.push(("kind".to_owned(), kind.as_str().to_owned()));
    }
    if let Some(text) = &filter.query {
        query.push(("q".to_owned(), text.clone()));
    }
    if let Some(min) = filter.min_players {
        query.push(("min_players".to_owned(), min.to_string()));
    }
    if let Some(max) = filter.max_players {
        query.push(("max_players".to_owned(), max.to_string()));
    }
    if filter.open_only {
        query.push(("open_only".to_owned(), "true".to_owned()));
    }
    if filter.with_slots {
        query.push(("with_slots".to_owned(), "true".to_owned()));
    }
    if let Some(limit) = filter.limit {
        query.push(("limit".to_owned(), limit.to_string()));
    }
    query
}

/// Serialise a value as pretty JSON for CLI output.
///
/// # Errors
///
/// Returns [DirectoryError::Json] when serialisation fails.
pub fn to_pretty<T: Serialize>(value: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(value)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::DiscoveryConfig;
    use crate::model::{GameKind, GameState};
    use crate::node::{NodeHandle, NodeSettings};

    async fn node() -> (NodeHandle, String) {
        let node = NodeHandle::bind(NodeSettings {
            name: "client-test".to_owned(),
            gossip_bind: "127.0.0.1:0".parse().expect("addr"),
            advertise: None,
            http_bind: Some("127.0.0.1:0".parse().expect("addr")),
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
        })
        .await
        .expect("bind");
        let base = format!("http://{}", node.http_addr().expect("http"));
        (node, base)
    }

    #[tokio::test]
    async fn publish_and_browse_through_the_client() {
        let (node, base) = node().await;
        let client = DirectoryClient::first_available(&[base])
            .await
            .expect("client");
        let published = PublishGame {
            id: None,
            name: "Client lobby".to_owned(),
            kind: GameKind::RedAlert,
            map: "SCM01EA".to_owned(),
            host_name: "Tanya".to_owned(),
            address: "203.0.113.21:47771".parse().expect("addr"),
            players: 2,
            max_players: 4,
            state: GameState::Lobby,
            passworded: false,
            version: "1.0".to_owned(),
            tags: vec![],
            ttl_secs: None,
        };
        let created = client.publish(&published).await.expect("publish");
        let filter = GameFilter {
            kind: Some(GameKind::RedAlert),
            with_slots: true,
            ..GameFilter::default()
        };
        let games = client.games(&filter).await.expect("games");
        assert_eq!(games.len(), 1);
        assert_eq!(games[0].id, created.id);
        assert_eq!(
            client.game(created.id).await.expect("one").name,
            "Client lobby"
        );
        client.heartbeat(created.id).await.expect("heartbeat");
        client.withdraw(created.id).await.expect("withdraw");
        assert!(client.games(&filter).await.expect("games").is_empty());
        node.shutdown().await;
    }

    #[tokio::test]
    async fn failover_skips_unreachable_nodes() {
        let (node, base) = node().await;
        let dead = "http://127.0.0.1:1".to_owned();
        let client = DirectoryClient::first_available(&[dead, base])
            .await
            .expect("failover");
        assert!(client.base().starts_with("http://127.0.0.1"));
        node.shutdown().await;
    }
}
