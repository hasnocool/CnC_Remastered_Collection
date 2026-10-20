//! JSON HTTP API and the built-in browser.
//!
//! Any node in the mesh serves the same API, so a client may hard-code several
//! and fail over between them. The API is read-mostly: two write endpoints
//! publish a lobby and withdraw it, plus a heartbeat refresh.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::watch;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

use crate::error::ValidationError;
use crate::matchmaking::{
    AgentCard, Controller, JoinQueue, LfgError, LfgService, MatchId, PlayerId, PlayerProfile,
    RankedMatch,
};
use crate::model::{GameFilter, GameId, GameInfo, GameKind, GameState};
use crate::registry::{Directory, Stats, NODE_VERSION};

const INDEX_HTML: &str = include_str!("../static/index.html");

/// Shared HTTP state: the replicated directory and this node's ranked ladder.
#[derive(Clone)]
struct AppState {
    dir: Arc<Directory>,
    lfg: Arc<LfgService>,
}

/// Build the HTTP router for a directory.
pub fn router(dir: Arc<Directory>, lfg: Arc<LfgService>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/api/v1/games", get(list_games).post(publish_game))
        .route("/api/v1/games/{id}", get(get_game).delete(withdraw_game))
        .route("/api/v1/games/{id}/heartbeat", post(heartbeat_game))
        .route("/api/v1/nodes", get(list_nodes))
        .route("/api/v1/stats", get(stats))
        .route(
            "/api/v1/lfg/players",
            get(list_players).post(register_player),
        )
        .route("/api/v1/lfg/players/{id}", get(get_player))
        .route("/api/v1/lfg/queue", get(list_queue).post(join_queue))
        .route("/api/v1/lfg/queue/{id}", delete(leave_queue))
        .route("/api/v1/lfg/leaderboard", get(leaderboard))
        .route("/api/v1/lfg/matches", get(list_matches))
        .route("/api/v1/lfg/matches/{id}", get(get_match))
        .route("/api/v1/lfg/matches/{id}/report", post(report_match))
        .route("/api/v1/lfg/matches/{id}/cancel", post(cancel_match))
        .route("/api/v1/lfg/status", get(lfg_status))
        .fallback(not_found)
        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { dir, lfg })
}

/// Serve the API until the node shuts down.
pub async fn serve(
    listener: TcpListener,
    dir: Arc<Directory>,
    lfg: Arc<LfgService>,
    mut shutdown: watch::Receiver<bool>,
) {
    let addr = listener.local_addr().ok();
    let app = router(dir, lfg);
    let shutdown_signal = async move {
        let _ = shutdown.changed().await;
    };
    info!(?addr, "HTTP browse API listening");
    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
    {
        warn!(error = %err, "HTTP server stopped");
    }
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

#[derive(Debug, Serialize)]
struct Health {
    status: &'static str,
    node_id: crate::model::NodeId,
    version: &'static str,
    uptime_secs: u64,
    games: usize,
    peers_alive: usize,
}

async fn health(State(state): State<AppState>) -> Json<Health> {
    let dir = state.dir;
    Json(Health {
        status: "ok",
        node_id: dir.node_id(),
        version: NODE_VERSION,
        uptime_secs: dir.uptime_secs(),
        games: dir.games_count(),
        peers_alive: dir.alive_peers(),
    })
}

/// Query parameters accepted by the browse endpoint.
#[derive(Debug, Deserialize, Default)]
pub struct GameQuery {
    /// `lobby`, `ready`, or `in_progress`.
    pub state: Option<String>,
    /// `td` or `ra`.
    pub kind: Option<String>,
    /// Free-text search.
    pub q: Option<String>,
    /// Minimum joined players.
    pub min_players: Option<u8>,
    /// Maximum joined players.
    pub max_players: Option<u8>,
    /// Drop password-protected lobbies.
    pub open_only: Option<bool>,
    /// Only lobbies with a free slot.
    pub with_slots: Option<bool>,
    /// Result cap.
    pub limit: Option<usize>,
}

impl GameQuery {
    fn into_filter(self) -> Result<GameFilter, ApiError> {
        let state = match self.state.as_deref() {
            None | Some("") => None,
            Some(raw) => Some(raw.parse::<GameState>().map_err(ApiError::bad_request)?),
        };
        let kind = match self.kind.as_deref() {
            None | Some("") => None,
            Some(raw) => Some(raw.parse::<GameKind>().map_err(ApiError::bad_request)?),
        };
        Ok(GameFilter {
            state,
            kind,
            query: self.q.filter(|q| !q.trim().is_empty()),
            min_players: self.min_players,
            max_players: self.max_players,
            open_only: self.open_only.unwrap_or(false),
            with_slots: self.with_slots.unwrap_or(false),
            limit: self.limit,
        })
    }
}

#[derive(Debug, Serialize)]
struct GamesResponse {
    games: Vec<GameInfo>,
    count: usize,
    node: crate::model::NodeId,
    digest: String,
}

async fn list_games(
    State(state): State<AppState>,
    Query(query): Query<GameQuery>,
) -> Result<Json<GamesResponse>, ApiError> {
    let dir = state.dir;
    let filter = query.into_filter()?;
    let games = dir.games(&filter);
    let count = games.len();
    Ok(Json(GamesResponse {
        games,
        count,
        node: dir.node_id(),
        digest: format!("{:016x}", dir.digest()),
    }))
}

async fn publish_game(
    State(state): State<AppState>,
    Json(publish): Json<crate::model::PublishGame>,
) -> Result<(StatusCode, Json<GameInfo>), ApiError> {
    let dir = state.dir;
    let game = dir.announce(publish).map_err(ApiError::validation)?;
    Ok((StatusCode::CREATED, Json(game)))
}

async fn get_game(
    State(state): State<AppState>,
    Path(id): Path<GameId>,
) -> Result<Json<GameInfo>, ApiError> {
    let dir = state.dir;
    dir.game(id)
        .map(Json)
        .ok_or_else(|| ApiError::not_found("game not found"))
}

async fn withdraw_game(
    State(state): State<AppState>,
    Path(id): Path<GameId>,
) -> Result<StatusCode, ApiError> {
    let dir = state.dir;
    if dir.withdraw(id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::not_found("game not found"))
    }
}

async fn heartbeat_game(
    State(state): State<AppState>,
    Path(id): Path<GameId>,
) -> Result<Json<GameInfo>, ApiError> {
    let dir = state.dir;
    dir.heartbeat(id)
        .map(Json)
        .ok_or_else(|| ApiError::not_found("game not found"))
}

#[derive(Debug, Serialize)]
struct NodesResponse {
    node: crate::model::NodeInfo,
    peers: Vec<crate::peer::PeerView>,
    count: usize,
    alive: usize,
}

async fn list_nodes(State(state): State<AppState>) -> Json<NodesResponse> {
    let dir = state.dir;
    let peers = dir.peers();
    Json(NodesResponse {
        node: dir.self_info(),
        count: peers.len(),
        alive: peers
            .iter()
            .filter(|p| p.state == crate::peer::PeerState::Alive)
            .count(),
        peers,
    })
}

async fn stats(State(state): State<AppState>) -> Json<Stats> {
    let dir = state.dir;
    Json(dir.stats())
}

#[derive(Debug, Deserialize)]
struct RegisterPlayerRequest {
    name: String,
    /// Defaults to a human competitor.
    controller: Option<Controller>,
    /// Required provenance for AI competitors.
    agent: Option<AgentCard>,
}

#[derive(Debug, Serialize)]
struct PlayersResponse {
    players: Vec<PlayerProfile>,
    count: usize,
}

#[derive(Debug, Deserialize, Default)]
struct PlayersQuery {
    controller: Option<String>,
}

async fn list_players(
    State(state): State<AppState>,
    Query(query): Query<PlayersQuery>,
) -> Result<Json<PlayersResponse>, ApiError> {
    let controller = query
        .controller
        .as_deref()
        .filter(|raw| !raw.trim().is_empty())
        .map(|raw| raw.parse::<Controller>().map_err(ApiError::bad_request))
        .transpose()?;
    let mut players = state.lfg.players();
    if let Some(controller) = controller {
        players.retain(|profile| profile.controller == controller);
    }
    Ok(Json(PlayersResponse {
        count: players.len(),
        players,
    }))
}

async fn register_player(
    State(state): State<AppState>,
    Json(body): Json<RegisterPlayerRequest>,
) -> Result<(StatusCode, Json<PlayerProfile>), ApiError> {
    let profile = state
        .lfg
        .register_competitor(&body.name, body.controller.unwrap_or_default(), body.agent)
        .map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(profile)))
}

async fn get_player(
    State(state): State<AppState>,
    Path(id): Path<PlayerId>,
) -> Result<Json<PlayerProfile>, ApiError> {
    state
        .lfg
        .profile(id)
        .map(Json)
        .ok_or_else(|| ApiError::not_found("player not found"))
}

#[derive(Debug, Deserialize)]
struct JoinQueueRequest {
    player_id: PlayerId,
    kind: GameKind,
    address: SocketAddr,
    map: Option<String>,
    note: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct QueueQuery {
    kind: Option<String>,
    player_id: Option<PlayerId>,
    /// human (default) or ai.
    controller: Option<String>,
}

#[derive(Debug, Serialize)]
struct QueueResponse {
    queue: Vec<crate::matchmaking::LfgEntry>,
    count: usize,
}

async fn list_queue(
    State(state): State<AppState>,
    Query(query): Query<QueueQuery>,
) -> Result<Json<QueueResponse>, ApiError> {
    let kind = parse_optional_kind(&query.kind)?;
    let controller = parse_controller(query.controller.as_deref())?;
    let mut queue = state.lfg.queue_for(controller, kind);
    if let Some(player_id) = query.player_id {
        queue.retain(|entry| entry.player_id == player_id);
    }
    Ok(Json(QueueResponse {
        count: queue.len(),
        queue,
    }))
}

async fn join_queue(
    State(state): State<AppState>,
    Json(body): Json<JoinQueueRequest>,
) -> Result<(StatusCode, Json<crate::matchmaking::LfgEntry>), ApiError> {
    let entry = state
        .lfg
        .join(JoinQueue {
            player_id: body.player_id,
            kind: body.kind,
            address: body.address,
            map: body.map,
            note: body.note,
        })
        .map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(entry)))
}

async fn leave_queue(
    State(state): State<AppState>,
    Path(id): Path<PlayerId>,
) -> Result<StatusCode, ApiError> {
    if state.lfg.leave(id) {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(ApiError::not_found("player is not queued"))
    }
}

#[derive(Debug, Deserialize, Default)]
struct LeaderboardQuery {
    kind: Option<String>,
    limit: Option<usize>,
    /// human (default) or ai.
    controller: Option<String>,
}

#[derive(Debug, Serialize)]
struct LeaderboardResponse {
    rows: Vec<crate::matchmaking::LeaderboardRow>,
    count: usize,
    kind: Option<GameKind>,
}

async fn leaderboard(
    State(state): State<AppState>,
    Query(query): Query<LeaderboardQuery>,
) -> Result<Json<LeaderboardResponse>, ApiError> {
    let kind = parse_optional_kind(&query.kind)?;
    let controller = parse_controller(query.controller.as_deref())?;
    let rows = state
        .lfg
        .leaderboard_for(controller, kind, query.limit.unwrap_or(25));
    Ok(Json(LeaderboardResponse {
        count: rows.len(),
        rows,
        kind,
    }))
}

#[derive(Debug, Deserialize, Default)]
struct MatchesQuery {
    player_id: Option<PlayerId>,
    kind: Option<String>,
    state: Option<String>,
    limit: Option<usize>,
    /// human or ai; omitted returns both ladders.
    controller: Option<String>,
}

#[derive(Debug, Serialize)]
struct MatchesResponse {
    matches: Vec<RankedMatch>,
    count: usize,
}

async fn list_matches(
    State(state): State<AppState>,
    Query(query): Query<MatchesQuery>,
) -> Result<Json<MatchesResponse>, ApiError> {
    let kind = parse_optional_kind(&query.kind)?;
    let wanted_state = match query.state.as_deref() {
        None | Some("") => None,
        Some(raw) => Some(parse_match_state(raw)?),
    };
    let wanted_controller = match query.controller.as_deref() {
        None | Some("") => None,
        Some(raw) => Some(raw.parse::<Controller>().map_err(ApiError::bad_request)?),
    };
    let mut matches = state
        .lfg
        .matches(query.player_id, kind, query.limit.unwrap_or(25));
    if let Some(wanted) = wanted_state {
        matches.retain(|record| record.state == wanted);
    }
    if let Some(wanted) = wanted_controller {
        matches.retain(|record| record.controller == wanted);
    }
    Ok(Json(MatchesResponse {
        count: matches.len(),
        matches,
    }))
}

async fn get_match(
    State(state): State<AppState>,
    Path(id): Path<MatchId>,
) -> Result<Json<RankedMatch>, ApiError> {
    state
        .lfg
        .match_record(id)
        .map(Json)
        .ok_or_else(|| ApiError::not_found("match not found"))
}

#[derive(Debug, Deserialize)]
struct ReportMatchRequest {
    reporter: PlayerId,
    winner: PlayerId,
}

async fn report_match(
    State(state): State<AppState>,
    Path(id): Path<MatchId>,
    Json(body): Json<ReportMatchRequest>,
) -> Result<Json<RankedMatch>, ApiError> {
    state
        .lfg
        .report(id, body.reporter, body.winner)
        .map(Json)
        .map_err(ApiError::from)
}

#[derive(Debug, Deserialize)]
struct CancelMatchRequest {
    player_id: PlayerId,
}

async fn cancel_match(
    State(state): State<AppState>,
    Path(id): Path<MatchId>,
    Json(body): Json<CancelMatchRequest>,
) -> Result<Json<RankedMatch>, ApiError> {
    state
        .lfg
        .cancel(id, body.player_id)
        .map(Json)
        .map_err(ApiError::from)
}

async fn lfg_status(State(state): State<AppState>) -> Json<crate::matchmaking::LfgStatus> {
    Json(state.lfg.status())
}

fn parse_controller(raw: Option<&str>) -> Result<Controller, ApiError> {
    match raw {
        None | Some("") => Ok(Controller::Human),
        Some(text) => text.parse::<Controller>().map_err(ApiError::bad_request),
    }
}

fn parse_optional_kind(raw: &Option<String>) -> Result<Option<GameKind>, ApiError> {
    match raw.as_deref() {
        None | Some("") => Ok(None),
        Some(text) => text
            .parse::<GameKind>()
            .map(Some)
            .map_err(ApiError::bad_request),
    }
}

fn parse_match_state(raw: &str) -> Result<crate::matchmaking::MatchState, ApiError> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "pending" => Ok(crate::matchmaking::MatchState::Pending),
        "settled" => Ok(crate::matchmaking::MatchState::Settled),
        "cancelled" | "canceled" => Ok(crate::matchmaking::MatchState::Cancelled),
        "expired" => Ok(crate::matchmaking::MatchState::Expired),
        other => Err(ApiError::bad_request(format!(
            "unknown match state {other:?}"
        ))),
    }
}

async fn not_found() -> ApiError {
    ApiError::not_found("no such endpoint")
}

/// API error rendered as `{"error": "..."}` with a status code.
#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    fn validation(err: ValidationError) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            message: err.to_string(),
        }
    }

    fn conflict(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            message: message.into(),
        }
    }

    fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            message: message.into(),
        }
    }
}

impl From<LfgError> for ApiError {
    fn from(err: LfgError) -> Self {
        match err {
            LfgError::UnknownPlayer | LfgError::UnknownMatch => Self::not_found(err.to_string()),
            LfgError::NotParticipant => Self::forbidden(err.to_string()),
            LfgError::MatchClosed | LfgError::MatchExpired | LfgError::AlreadyMatched => {
                Self::conflict(err.to_string())
            }
            LfgError::InvalidName
            | LfgError::InvalidAddress
            | LfgError::InvalidWinner
            | LfgError::InvalidAgent => Self {
                status: StatusCode::UNPROCESSABLE_ENTITY,
                message: err.to_string(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct Body {
            error: String,
        }
        (
            self.status,
            Json(Body {
                error: self.message,
            }),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use crate::discovery::DiscoveryConfig;
    use crate::model::{GameKind, GameState, PublishGame};
    use crate::node::{NodeHandle, NodeSettings};

    async fn node() -> NodeHandle {
        NodeHandle::bind(NodeSettings {
            name: "api-test".to_owned(),
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
        .expect("bind")
    }

    fn publish() -> PublishGame {
        PublishGame {
            id: None,
            name: "API lobby".to_owned(),
            kind: GameKind::TiberianDawn,
            map: "SCG01EA".to_owned(),
            host_name: "Kane".to_owned(),
            address: "203.0.113.11:47771".parse().expect("addr"),
            players: 1,
            max_players: 4,
            state: GameState::Lobby,
            passworded: false,
            version: "1.0".to_owned(),
            tags: vec![],
            ttl_secs: None,
        }
    }

    #[tokio::test]
    async fn publish_browse_and_withdraw_over_http() {
        let node = node().await;
        let base = format!("http://{}", node.http_addr().expect("http"));
        let client = reqwest::Client::new();

        let created: GameInfo = client
            .post(format!("{base}/api/v1/games"))
            .json(&publish())
            .send()
            .await
            .expect("post")
            .json()
            .await
            .expect("created body");

        let listed: serde_json::Value = client
            .get(format!("{base}/api/v1/games?kind=td&q=api"))
            .send()
            .await
            .expect("get")
            .json()
            .await
            .expect("listed body");
        assert_eq!(listed["count"], 1);
        assert_eq!(listed["games"][0]["name"], "API lobby");

        let heartbeat = client
            .post(format!("{base}/api/v1/games/{}/heartbeat", created.id))
            .send()
            .await
            .expect("heartbeat");
        assert_eq!(heartbeat.status(), StatusCode::OK);

        let stats: serde_json::Value = client
            .get(format!("{base}/api/v1/stats"))
            .send()
            .await
            .expect("stats")
            .json()
            .await
            .expect("stats body");
        assert_eq!(stats["games"], 1);

        let deleted = client
            .delete(format!("{base}/api/v1/games/{}", created.id))
            .send()
            .await
            .expect("delete");
        assert_eq!(deleted.status(), StatusCode::NO_CONTENT);

        let missing = client
            .get(format!("{base}/api/v1/games/{}", created.id))
            .send()
            .await
            .expect("get missing");
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);

        node.shutdown().await;
    }

    #[tokio::test]
    async fn invalid_announcement_is_rejected() {
        let node = node().await;
        let base = format!("http://{}", node.http_addr().expect("http"));
        let mut bad = publish();
        bad.name = "   ".to_owned();
        let response = reqwest::Client::new()
            .post(format!("{base}/api/v1/games"))
            .json(&bad)
            .send()
            .await
            .expect("post");
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        node.shutdown().await;
    }

    #[tokio::test]
    async fn health_reports_the_node() {
        let node = node().await;
        let base = format!("http://{}", node.http_addr().expect("http"));
        let health: serde_json::Value = reqwest::Client::new()
            .get(format!("{base}/health"))
            .send()
            .await
            .expect("health")
            .json()
            .await
            .expect("body");
        assert_eq!(health["status"], "ok");
        node.shutdown().await;
    }
}
