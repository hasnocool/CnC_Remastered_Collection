//! Ranked "looking for game" matchmaking on top of the lobby directory.
//!
//! StarCraft-II-style ranked play needs four things: a persistent player
//! profile, an MMR rating split per title, a queue that pairs players of
//! comparable skill, and a settled match result that moves both ratings. This
//! module provides all four inside the existing directory node, so the same
//! HTTP listener that announces lobbies also runs the ladder.
//!
//! When two queued players are paired their match is published as an ordinary
//! GameInfo lobby through Directory, tagged ranked/lfg. The match announcement
//! therefore rides the existing gossip mesh, appears in GET /api/v1/games, and
//! disappears when the result is reported or the match expires.
//!
//! State is node-local: each directory node runs an independent ladder. The
//! ladder can be persisted to JSON so a restart does not erase ratings. A
//! deployment that wants one shared ladder should route ranked clients to one
//! designated node; the lobby announcements themselves still replicate
//! everywhere.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use uuid::Uuid;

use crate::model::{now_ms, GameId, GameKind, GameState, PublishGame};
use crate::registry::Directory;

/// Longest accepted player display name.
pub const PLAYER_NAME_MAX: usize = 32;
/// Rating assigned to a new player.
pub const DEFAULT_RATING: i32 = 2000;
/// Lowest reachable rating.
pub const RATING_MIN: i32 = 100;
/// Highest reachable rating.
pub const RATING_MAX: i32 = 8000;
/// Matches below this count use the provisional K-factor.
pub const PROVISIONAL_GAMES: u32 = 10;
/// K-factor for established players.
pub const K_FACTOR: f64 = 32.0;
/// K-factor while a player is still calibrating.
pub const PROVISIONAL_K_FACTOR: f64 = 64.0;
/// How long a queue entry waits before it is dropped.
pub const QUEUE_TTL_MS: i64 = 5 * 60 * 1000;
/// How long a pending match waits for a result.
pub const MATCH_TTL_MS: i64 = 30 * 60 * 1000;
/// Base rating gap accepted when a pair has just joined the queue.
pub const BASE_TOLERANCE: i32 = 150;
/// Extra rating gap accepted per second of waiting.
pub const TOLERANCE_PER_SEC: i32 = 25;
/// Hard ceiling on the widening match tolerance.
pub const MAX_TOLERANCE: i32 = 1500;
/// Largest leaderboard page.
pub const LEADERBOARD_LIMIT_MAX: usize = 100;
/// Largest match history page.
pub const MATCH_LIMIT_MAX: usize = 100;
/// Hard cap on queued entries.
pub const QUEUE_MAX: usize = 512;
/// Hard cap on retained match records.
pub const MATCH_HISTORY_MAX: usize = 2048;
/// Default map used when neither player picked one (Tiberian Dawn).
const TD_DEFAULT_MAP: &str = "SCG01EA";
/// Default map used when neither player picked one (Red Alert).
const RA_DEFAULT_MAP: &str = "SCM01EA";

/// Identifier of a registered ladder player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlayerId(Uuid);

impl PlayerId {
    /// Generate a fresh identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for PlayerId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s.trim()).map(Self)
    }
}

/// Identifier of one ranked match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MatchId(Uuid);

impl MatchId {
    /// Generate a fresh identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for MatchId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MatchId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for MatchId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s.trim()).map(Self)
    }
}

/// StarCraft-II-style league derived from MMR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum League {
    /// Learning the game.
    Bronze,
    /// Below average.
    Silver,
    /// Average ladder player.
    Gold,
    /// Above average.
    Platinum,
    /// High ladder.
    Diamond,
    /// Top of the ladder.
    Master,
    /// The very top; normally a capped, invitational tier.
    Grandmaster,
}

impl League {
    /// League containing a rating.
    #[must_use]
    pub const fn from_rating(rating: i32) -> Self {
        match rating {
            ..=1499 => Self::Bronze,
            1500..=1999 => Self::Silver,
            2000..=2499 => Self::Gold,
            2500..=2999 => Self::Platinum,
            3000..=3499 => Self::Diamond,
            3500..=3999 => Self::Master,
            _ => Self::Grandmaster,
        }
    }

    /// Stable wire/API name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bronze => "bronze",
            Self::Silver => "silver",
            Self::Gold => "gold",
            Self::Platinum => "platinum",
            Self::Diamond => "diamond",
            Self::Master => "master",
            Self::Grandmaster => "grandmaster",
        }
    }
}

impl fmt::Display for League {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for League {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "bronze" => Ok(Self::Bronze),
            "silver" => Ok(Self::Silver),
            "gold" => Ok(Self::Gold),
            "platinum" | "plat" => Ok(Self::Platinum),
            "diamond" | "dia" => Ok(Self::Diamond),
            "master" | "masters" => Ok(Self::Master),
            "grandmaster" | "grand-master" | "gm" => Ok(Self::Grandmaster),
            other => Err(format!("unknown league {other:?}")),
        }
    }
}

/// Who controls a ladder competitor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Controller {
    /// A person playing the game.
    #[default]
    Human,
    /// An authored AI agent played by the deterministic runtime.
    Ai,
}

impl Controller {
    /// Stable wire/API name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Ai => "ai",
        }
    }

    /// Whether this competitor is an AI agent.
    #[must_use]
    pub const fn is_ai(self) -> bool {
        matches!(self, Self::Ai)
    }
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Controller {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "" | "human" | "player" => Ok(Self::Human),
            "ai" | "bot" | "agent" => Ok(Self::Ai),
            other => Err(format!(
                "unknown controller {other:?} (expected human or ai)"
            )),
        }
    }
}

/// Provenance for an AI competitor, mirroring the scenario-side agent record.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCard {
    /// Map-local agent id from the AIAgents section.
    #[serde(default)]
    pub agent_id: Option<u32>,
    /// House class such as HouseGDI or HouseNod; empty means unassigned.
    #[serde(default)]
    pub owner: String,
    /// Doctrine chosen in the editor.
    #[serde(default)]
    pub doctrine: String,
    /// Hex fingerprint of the agent script, so the ladder pins exact code.
    #[serde(default)]
    pub script_hash: String,
    /// Human author or creator of the agent.
    #[serde(default)]
    pub creator: Option<String>,
    /// Scenario or map the agent was authored in.
    #[serde(default)]
    pub scenario: Option<String>,
}

impl AgentCard {
    /// Trim and bound every free-text field.
    fn sanitize(mut self) -> Result<Self, LfgError> {
        self.owner = self.owner.trim().chars().take(64).collect();
        self.doctrine = self.doctrine.trim().chars().take(64).collect();
        self.script_hash = self.script_hash.trim().to_ascii_lowercase();
        if !self.script_hash.is_empty()
            && (self.script_hash.len() > 64
                || !self.script_hash.chars().all(|ch| ch.is_ascii_hexdigit()))
        {
            return Err(LfgError::InvalidAgent);
        }
        self.creator = clean_optional(self.creator);
        self.scenario = clean_optional(self.scenario);
        Ok(self)
    }
}

/// One player's rating and record for one title.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KindRating {
    /// Which remastered title this rating belongs to.
    pub kind: GameKind,
    /// Matchmaking rating.
    pub rating: i32,
    /// Wins on this title.
    pub wins: u32,
    /// Losses on this title.
    pub losses: u32,
    /// Positive for a win streak, negative for a losing streak.
    pub streak: i32,
    /// Total settled matches on this title.
    pub games: u32,
    /// Milliseconds since the epoch of the last settled match.
    pub last_played_ms: i64,
}

impl KindRating {
    fn new(kind: GameKind) -> Self {
        Self {
            kind,
            rating: DEFAULT_RATING,
            wins: 0,
            losses: 0,
            streak: 0,
            games: 0,
            last_played_ms: 0,
        }
    }

    /// League this rating currently sits in.
    #[must_use]
    pub const fn league(&self) -> League {
        League::from_rating(self.rating)
    }

    /// Whether the player is still calibrating.
    #[must_use]
    pub const fn provisional(&self) -> bool {
        self.games < PROVISIONAL_GAMES
    }
}

/// A registered ladder player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    /// Stable player identifier.
    pub id: PlayerId,
    /// Display name.
    pub name: String,
    /// Whether a person or an AI agent controls this competitor.
    #[serde(default)]
    pub controller: Controller,
    /// Provenance for an AI competitor; always absent for humans.
    #[serde(default)]
    pub agent: Option<AgentCard>,
    /// Per-title ratings, always one entry for each game kind.
    pub ratings: Vec<KindRating>,
    /// Milliseconds since the epoch when the profile was created.
    pub created_ms: i64,
    /// Milliseconds since the epoch of the last profile change.
    pub updated_ms: i64,
}

impl PlayerProfile {
    fn new(name: String, controller: Controller, agent: Option<AgentCard>) -> Self {
        let now = now_ms();
        Self {
            id: PlayerId::new(),
            name,
            controller,
            agent,
            ratings: vec![
                KindRating::new(GameKind::TiberianDawn),
                KindRating::new(GameKind::RedAlert),
            ],
            created_ms: now,
            updated_ms: now,
        }
    }

    /// Whether this competitor is an AI agent.
    #[must_use]
    pub fn is_ai(&self) -> bool {
        self.controller.is_ai()
    }

    /// Rating record for one title.
    #[must_use]
    pub fn rating(&self, kind: GameKind) -> Option<&KindRating> {
        self.ratings.iter().find(|rating| rating.kind == kind)
    }

    fn rating_mut(&mut self, kind: GameKind) -> Option<&mut KindRating> {
        self.ratings.iter_mut().find(|rating| rating.kind == kind)
    }
}

/// One player waiting in the ranked queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LfgEntry {
    /// Queued player.
    pub player_id: PlayerId,
    /// Display name captured at queue time.
    pub name: String,
    /// Whether this competitor is human or AI.
    #[serde(default)]
    pub controller: Controller,
    /// AI provenance, copied from the profile at queue time.
    #[serde(default)]
    pub agent: Option<AgentCard>,
    /// Title being queued for.
    pub kind: GameKind,
    /// Address other players should dial for this player's hosted game.
    pub address: SocketAddr,
    /// Preferred map, when the player expressed one.
    pub map: Option<String>,
    /// Free-text note (for example "no rush 10"). Never interpreted.
    pub note: Option<String>,
    /// Milliseconds since the epoch when the player entered the queue.
    pub joined_ms: i64,
}

/// Request to enter the ranked queue.
#[derive(Debug, Clone)]
pub struct JoinQueue {
    /// Registered player.
    pub player_id: PlayerId,
    /// Title to queue for.
    pub kind: GameKind,
    /// Address other players should dial.
    pub address: SocketAddr,
    /// Preferred map.
    pub map: Option<String>,
    /// Free-text note.
    pub note: Option<String>,
}

/// Lifecycle of a ranked match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchState {
    /// Players have been paired and the lobby is live.
    Pending,
    /// A participant reported a winner and ratings moved.
    Settled,
    /// A participant cancelled before reporting.
    Cancelled,
    /// Nobody reported in time.
    Expired,
}

impl MatchState {
    /// Stable wire/API name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Settled => "settled",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
        }
    }
}

/// One side of a ranked match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchPlayer {
    /// Player identifier.
    pub player_id: PlayerId,
    /// Display name at pairing time.
    pub name: String,
    /// Whether this side was human or AI.
    #[serde(default)]
    pub controller: Controller,
    /// AI provenance for an AI side.
    #[serde(default)]
    pub agent: Option<AgentCard>,
    /// Rating before the match.
    pub rating: i32,
    /// League before the match.
    pub league: League,
    /// 1 or 2.
    pub team: u8,
    /// Address this player hosts on.
    pub address: SocketAddr,
}

/// Rating movement produced by a settled match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingChange {
    /// Player whose rating moved.
    pub player_id: PlayerId,
    /// Rating before the result.
    pub before: i32,
    /// Rating after the result.
    pub after: i32,
    /// Signed movement (after - before).
    pub delta: i32,
}

/// A paired ranked match and its result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedMatch {
    /// Stable match identifier.
    pub id: MatchId,
    /// Title played.
    pub kind: GameKind,
    /// Whether this was a human or an AI-only match.
    #[serde(default)]
    pub controller: Controller,
    /// Map for the match.
    pub map: String,
    /// Current lifecycle state.
    pub state: MatchState,
    /// The two paired players, in team order.
    pub players: Vec<MatchPlayer>,
    /// Milliseconds since the epoch when the match was created.
    pub created_ms: i64,
    /// Milliseconds since the epoch of the last match change.
    pub updated_ms: i64,
    /// Milliseconds since the epoch after which a pending match expires.
    pub expires_ms: i64,
    /// Lobby announcement created in the directory, when publication succeeded.
    pub game_id: Option<GameId>,
    /// Reported winner, once settled.
    pub winner: Option<PlayerId>,
    /// Participant who reported the result.
    pub reported_by: Option<PlayerId>,
    /// Rating movement, once settled.
    pub rating_changes: Vec<RatingChange>,
}

/// One row of the ranked leaderboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardRow {
    /// 1-based position in the returned page.
    pub rank: usize,
    /// Player identifier.
    pub player_id: PlayerId,
    /// Display name.
    pub name: String,
    /// Title this row describes.
    pub kind: GameKind,
    /// Whether this row belongs to the human or AI ladder.
    #[serde(default)]
    pub controller: Controller,
    /// AI provenance, for AI-ladder rows.
    #[serde(default)]
    pub agent: Option<AgentCard>,
    /// Current rating.
    pub rating: i32,
    /// League for the rating.
    pub league: League,
    /// Wins on the title.
    pub wins: u32,
    /// Losses on the title.
    pub losses: u32,
    /// Settled matches on the title.
    pub games: u32,
    /// Positive for a win streak, negative for a losing streak.
    pub streak: i32,
    /// Whether the player is still calibrating.
    pub provisional: bool,
}

/// Aggregate ranked counters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LfgStatus {
    /// Registered players.
    pub players: usize,
    /// Players currently queued.
    pub queued: usize,
    /// Matches waiting for a result.
    pub pending_matches: usize,
    /// Matches with a settled result retained in memory.
    pub settled_matches: usize,
    /// Highest current rating, when any player is registered.
    pub top_rating: Option<i32>,
    /// Registered AI competitors.
    #[serde(default)]
    pub ai_players: usize,
    /// AI competitors currently queued.
    #[serde(default)]
    pub ai_queued: usize,
    /// AI-only matches waiting for a result.
    #[serde(default)]
    pub ai_pending_matches: usize,
    /// Settled AI-only matches retained in memory.
    #[serde(default)]
    pub ai_settled_matches: usize,
    /// Highest current AI rating, when any AI is registered.
    #[serde(default)]
    pub ai_top_rating: Option<i32>,
}

/// Errors returned by the ranked ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum LfgError {
    /// No profile with that identifier.
    #[error("player not found")]
    UnknownPlayer,
    /// No match with that identifier.
    #[error("match not found")]
    UnknownMatch,
    /// The player name was empty or too long.
    #[error("player name must be 1-{PLAYER_NAME_MAX} characters")]
    InvalidName,
    /// The supplied queue address is not dialable.
    #[error("queue address must have a routable IP and a non-zero port")]
    InvalidAddress,
    /// The player already has a pending match.
    #[error("player already has a pending match")]
    AlreadyMatched,
    /// The match can no longer accept a result.
    #[error("match is not pending")]
    MatchClosed,
    /// The reporter is not one of the two players.
    #[error("player is not a participant in this match")]
    NotParticipant,
    /// The reported winner is not one of the two players.
    #[error("winner is not a participant in this match")]
    InvalidWinner,
    /// The match passed its result deadline.
    #[error("match has expired")]
    MatchExpired,
    /// Agent metadata was malformed.
    #[error("agent card is malformed")]
    InvalidAgent,
}

/// Persisted ladder state.
#[derive(Debug, Default, Serialize, Deserialize)]
struct LfgSnapshot {
    #[serde(default)]
    players: Vec<PlayerProfile>,
    #[serde(default)]
    queue: Vec<LfgEntry>,
    #[serde(default)]
    matches: Vec<RankedMatch>,
}

#[derive(Debug, Default)]
struct LfgState {
    players: HashMap<PlayerId, PlayerProfile>,
    queue: Vec<LfgEntry>,
    matches: HashMap<MatchId, RankedMatch>,
}

impl LfgState {
    fn from_snapshot(snapshot: LfgSnapshot) -> Self {
        let players = snapshot
            .players
            .into_iter()
            .map(|profile| (profile.id, profile))
            .collect();
        let matches = snapshot
            .matches
            .into_iter()
            .map(|record| (record.id, record))
            .collect();
        Self {
            players,
            queue: snapshot.queue,
            matches,
        }
    }

    fn snapshot(&self) -> LfgSnapshot {
        LfgSnapshot {
            players: self.players.values().cloned().collect(),
            queue: self.queue.clone(),
            matches: self.matches.values().cloned().collect(),
        }
    }
}

/// Ranked matchmaking service for one directory node.
pub struct LfgService {
    dir: Arc<Directory>,
    state_path: Option<PathBuf>,
    state: Mutex<LfgState>,
}

impl fmt::Debug for LfgService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.state.lock();
        f.debug_struct("LfgService")
            .field("players", &state.players.len())
            .field("queue", &state.queue.len())
            .field("matches", &state.matches.len())
            .field("state_path", &self.state_path)
            .finish()
    }
}

impl LfgService {
    /// Create the service, loading persisted state when a path is configured.
    #[must_use]
    pub fn new(dir: Arc<Directory>, state_path: Option<PathBuf>) -> Self {
        let state = state_path
            .as_deref()
            .map(load_snapshot)
            .map(LfgState::from_snapshot)
            .unwrap_or_default();
        Self {
            dir,
            state_path,
            state: Mutex::new(state),
        }
    }

    /// Register a new player and return the generated profile.
    ///
    /// # Errors
    ///
    /// Returns [LfgError::InvalidName] when the trimmed name is empty or longer
    /// than [PLAYER_NAME_MAX].
    pub fn register(&self, name: &str) -> Result<PlayerProfile, LfgError> {
        self.register_competitor(name, Controller::Human, None)
    }

    /// Register an AI competitor with an agent card for the AI-only ladder.
    ///
    /// # Errors
    ///
    /// Returns [LfgError::InvalidName] for a bad display name and
    /// [LfgError::InvalidAgent] when the card fingerprint is malformed.
    pub fn register_ai(&self, name: &str, agent: AgentCard) -> Result<PlayerProfile, LfgError> {
        self.register_competitor(name, Controller::Ai, Some(agent))
    }

    /// Register a human or AI competitor.
    ///
    /// # Errors
    ///
    /// Returns [LfgError::InvalidName] for a bad display name,
    /// [LfgError::InvalidAgent] for malformed AI metadata or when agent
    /// metadata is supplied for a human competitor.
    pub fn register_competitor(
        &self,
        name: &str,
        controller: Controller,
        agent: Option<AgentCard>,
    ) -> Result<PlayerProfile, LfgError> {
        let name = name.trim();
        if name.is_empty() || name.chars().count() > PLAYER_NAME_MAX {
            return Err(LfgError::InvalidName);
        }
        let agent = match controller {
            Controller::Human => {
                if agent.is_some() {
                    return Err(LfgError::InvalidAgent);
                }
                None
            }
            Controller::Ai => Some(agent.unwrap_or_default().sanitize()?),
        };
        let profile = PlayerProfile::new(name.to_owned(), controller, agent);
        self.state
            .lock()
            .players
            .insert(profile.id, profile.clone());
        Ok(profile)
    }

    /// Fetch one profile.
    #[must_use]
    pub fn profile(&self, id: PlayerId) -> Option<PlayerProfile> {
        self.state.lock().players.get(&id).cloned()
    }

    /// All registered profiles, sorted by name.
    #[must_use]
    pub fn players(&self) -> Vec<PlayerProfile> {
        let mut players: Vec<PlayerProfile> = self.state.lock().players.values().cloned().collect();
        players.sort_by(|a, b| a.name.cmp(&b.name));
        players
    }

    /// Enter (or refresh) the ranked queue for one title.
    ///
    /// # Errors
    ///
    /// Returns [LfgError::UnknownPlayer] for an unregistered player,
    /// [LfgError::InvalidAddress] for an unusable host address, and
    /// [LfgError::AlreadyMatched] while the player has an unreported match.
    pub fn join(&self, request: JoinQueue) -> Result<LfgEntry, LfgError> {
        validate_address(request.address)?;
        let mut state = self.state.lock();
        let profile = state
            .players
            .get(&request.player_id)
            .ok_or(LfgError::UnknownPlayer)?;
        let name = profile.name.clone();
        let controller = profile.controller;
        let agent = profile.agent.clone();
        if state.matches.values().any(|record| {
            record.state == MatchState::Pending
                && record
                    .players
                    .iter()
                    .any(|player| player.player_id == request.player_id)
        }) {
            return Err(LfgError::AlreadyMatched);
        }
        let entry = LfgEntry {
            player_id: request.player_id,
            name,
            controller,
            agent,
            kind: request.kind,
            address: request.address,
            map: clean_optional(request.map),
            note: clean_optional(request.note),
            joined_ms: now_ms(),
        };
        state
            .queue
            .retain(|queued| queued.player_id != request.player_id);
        state.queue.push(entry.clone());
        if state.queue.len() > QUEUE_MAX {
            state.queue.remove(0);
        }
        Ok(entry)
    }

    /// Remove a player from the queue.
    #[must_use]
    pub fn leave(&self, player_id: PlayerId) -> bool {
        let mut state = self.state.lock();
        let before = state.queue.len();
        state.queue.retain(|entry| entry.player_id != player_id);
        state.queue.len() != before
    }

    /// Snapshot of the human queue, oldest first, optionally filtered by title.
    #[must_use]
    pub fn queue(&self, kind: Option<GameKind>) -> Vec<LfgEntry> {
        self.queue_for(Controller::Human, kind)
    }

    /// Snapshot of one controller's queue, oldest first.
    #[must_use]
    pub fn queue_for(&self, controller: Controller, kind: Option<GameKind>) -> Vec<LfgEntry> {
        let state = self.state.lock();
        let mut queue: Vec<LfgEntry> = state
            .queue
            .iter()
            .filter(|entry| {
                entry.controller == controller && kind.is_none_or(|wanted| entry.kind == wanted)
            })
            .cloned()
            .collect();
        queue.sort_by_key(|entry| entry.joined_ms);
        queue
    }

    /// Human ranked leaderboard, highest rating first.
    #[must_use]
    pub fn leaderboard(&self, kind: Option<GameKind>, limit: usize) -> Vec<LeaderboardRow> {
        self.leaderboard_for(Controller::Human, kind, limit)
    }

    /// Ranked leaderboard for one controller, highest rating first.
    #[must_use]
    pub fn leaderboard_for(
        &self,
        controller: Controller,
        kind: Option<GameKind>,
        limit: usize,
    ) -> Vec<LeaderboardRow> {
        let state = self.state.lock();
        let mut rows = Vec::new();
        for profile in state.players.values() {
            if profile.controller != controller {
                continue;
            }
            for rating in &profile.ratings {
                if kind.is_some_and(|wanted| rating.kind != wanted) {
                    continue;
                }
                rows.push(LeaderboardRow {
                    rank: 0,
                    player_id: profile.id,
                    name: profile.name.clone(),
                    kind: rating.kind,
                    controller: profile.controller,
                    agent: profile.agent.clone(),
                    rating: rating.rating,
                    league: rating.league(),
                    wins: rating.wins,
                    losses: rating.losses,
                    games: rating.games,
                    streak: rating.streak,
                    provisional: rating.provisional(),
                });
            }
        }
        rows.sort_by(|a, b| {
            b.rating
                .cmp(&a.rating)
                .then_with(|| b.games.cmp(&a.games))
                .then_with(|| a.name.cmp(&b.name))
        });
        rows.truncate(limit.clamp(1, LEADERBOARD_LIMIT_MAX));
        for (index, row) in rows.iter_mut().enumerate() {
            row.rank = index + 1;
        }
        rows
    }

    /// Fetch one match.
    #[must_use]
    pub fn match_record(&self, id: MatchId) -> Option<RankedMatch> {
        self.state.lock().matches.get(&id).cloned()
    }

    /// Match history, newest first.
    #[must_use]
    pub fn matches(
        &self,
        player_id: Option<PlayerId>,
        kind: Option<GameKind>,
        limit: usize,
    ) -> Vec<RankedMatch> {
        let state = self.state.lock();
        let mut records: Vec<RankedMatch> = state
            .matches
            .values()
            .filter(|record| {
                player_id.is_none_or(|wanted| {
                    record
                        .players
                        .iter()
                        .any(|player| player.player_id == wanted)
                }) && kind.is_none_or(|wanted| record.kind == wanted)
            })
            .cloned()
            .collect();
        records.sort_by(|a, b| b.created_ms.cmp(&a.created_ms).then(b.id.cmp(&a.id)));
        records.truncate(limit.clamp(1, MATCH_LIMIT_MAX));
        records
    }

    /// Aggregate ladder counters, split by controller.
    #[must_use]
    pub fn status(&self) -> LfgStatus {
        let state = self.state.lock();
        let human_top_rating = top_rating(&state, Controller::Human);
        let ai_top_rating = top_rating(&state, Controller::Ai);
        LfgStatus {
            players: state
                .players
                .values()
                .filter(|profile| profile.controller == Controller::Human)
                .count(),
            queued: state
                .queue
                .iter()
                .filter(|entry| entry.controller == Controller::Human)
                .count(),
            pending_matches: state
                .matches
                .values()
                .filter(|record| {
                    record.controller == Controller::Human && record.state == MatchState::Pending
                })
                .count(),
            settled_matches: state
                .matches
                .values()
                .filter(|record| {
                    record.controller == Controller::Human && record.state == MatchState::Settled
                })
                .count(),
            top_rating: human_top_rating,
            ai_players: state
                .players
                .values()
                .filter(|profile| profile.controller == Controller::Ai)
                .count(),
            ai_queued: state
                .queue
                .iter()
                .filter(|entry| entry.controller == Controller::Ai)
                .count(),
            ai_pending_matches: state
                .matches
                .values()
                .filter(|record| {
                    record.controller == Controller::Ai && record.state == MatchState::Pending
                })
                .count(),
            ai_settled_matches: state
                .matches
                .values()
                .filter(|record| {
                    record.controller == Controller::Ai && record.state == MatchState::Settled
                })
                .count(),
            ai_top_rating,
        }
    }

    /// Report a result, settle the match, and move both ratings.
    ///
    /// Either participant may report; the first report wins. Ratings use Elo
    /// with a provisional K-factor for a player's first ten settled matches.
    ///
    /// # Errors
    ///
    /// Returns [LfgError::UnknownMatch] for an unknown match,
    /// [LfgError::MatchClosed] when it is no longer pending,
    /// [LfgError::MatchExpired] after the report deadline,
    /// [LfgError::NotParticipant] when the reporter is not playing, and
    /// [LfgError::InvalidWinner] when the winner is not playing.
    pub fn report(
        &self,
        id: MatchId,
        reporter: PlayerId,
        winner: PlayerId,
    ) -> Result<RankedMatch, LfgError> {
        let now = now_ms();
        let mut state = self.state.lock();
        let mut record = state
            .matches
            .get(&id)
            .cloned()
            .ok_or(LfgError::UnknownMatch)?;
        if record.state != MatchState::Pending {
            return Err(LfgError::MatchClosed);
        }
        if now >= record.expires_ms {
            return Err(LfgError::MatchExpired);
        }
        if !record
            .players
            .iter()
            .any(|player| player.player_id == reporter)
        {
            return Err(LfgError::NotParticipant);
        }
        if !record
            .players
            .iter()
            .any(|player| player.player_id == winner)
        {
            return Err(LfgError::InvalidWinner);
        }
        let first = record
            .players
            .first()
            .cloned()
            .ok_or(LfgError::UnknownMatch)?;
        let second = record
            .players
            .get(1)
            .cloned()
            .ok_or(LfgError::UnknownMatch)?;
        let (winner_side, loser_side) = if winner == first.player_id {
            (first, second)
        } else {
            (second, first)
        };
        let winner_rating = state
            .players
            .get(&winner_side.player_id)
            .and_then(|profile| profile.rating(record.kind))
            .cloned()
            .ok_or(LfgError::UnknownPlayer)?;
        let loser_rating = state
            .players
            .get(&loser_side.player_id)
            .and_then(|profile| profile.rating(record.kind))
            .cloned()
            .ok_or(LfgError::UnknownPlayer)?;
        let expected_winner = expected_score(winner_rating.rating, loser_rating.rating);
        let winner_delta = k_factor(winner_rating.games) * (1.0 - expected_winner);
        let loser_delta = k_factor(loser_rating.games) * (0.0 - (1.0 - expected_winner));
        let winner_after = clamp_rating(winner_rating.rating + round_rating(winner_delta));
        let loser_after = clamp_rating(loser_rating.rating + round_rating(loser_delta));
        let changes = vec![
            RatingChange {
                player_id: winner_side.player_id,
                before: winner_rating.rating,
                after: winner_after,
                delta: winner_after - winner_rating.rating,
            },
            RatingChange {
                player_id: loser_side.player_id,
                before: loser_rating.rating,
                after: loser_after,
                delta: loser_after - loser_rating.rating,
            },
        ];
        for change in &changes {
            if let Some(profile) = state.players.get_mut(&change.player_id) {
                let won = change.player_id == winner;
                if let Some(rating) = profile.rating_mut(record.kind) {
                    rating.rating = change.after;
                    rating.games = rating.games.saturating_add(1);
                    rating.last_played_ms = now;
                    if won {
                        rating.wins = rating.wins.saturating_add(1);
                        rating.streak = if rating.streak >= 0 {
                            rating.streak.saturating_add(1)
                        } else {
                            1
                        };
                    } else {
                        rating.losses = rating.losses.saturating_add(1);
                        rating.streak = if rating.streak <= 0 {
                            rating.streak.saturating_sub(1)
                        } else {
                            -1
                        };
                    }
                }
                profile.updated_ms = now;
            }
        }
        record.state = MatchState::Settled;
        record.winner = Some(winner);
        record.reported_by = Some(reporter);
        record.rating_changes = changes;
        record.updated_ms = now;
        let game_id = record.game_id;
        state.matches.insert(record.id, record.clone());
        drop(state);
        if let Some(game_id) = game_id {
            self.dir.withdraw(game_id);
        }
        info!(match_id = %record.id, %winner, "ranked result settled");
        Ok(record)
    }

    /// Cancel a pending match. Either participant may cancel it.
    ///
    /// # Errors
    ///
    /// Returns [LfgError::UnknownMatch] for an unknown match,
    /// [LfgError::MatchClosed] when it is no longer pending, and
    /// [LfgError::NotParticipant] when the caller is not playing.
    pub fn cancel(&self, id: MatchId, player: PlayerId) -> Result<RankedMatch, LfgError> {
        let mut state = self.state.lock();
        let mut record = state
            .matches
            .get(&id)
            .cloned()
            .ok_or(LfgError::UnknownMatch)?;
        if record.state != MatchState::Pending {
            return Err(LfgError::MatchClosed);
        }
        if !record
            .players
            .iter()
            .any(|participant| participant.player_id == player)
        {
            return Err(LfgError::NotParticipant);
        }
        record.state = MatchState::Cancelled;
        record.updated_ms = now_ms();
        let game_id = record.game_id;
        state.matches.insert(record.id, record.clone());
        drop(state);
        if let Some(game_id) = game_id {
            self.dir.withdraw(game_id);
        }
        Ok(record)
    }

    /// Advance the ladder clock: expire stale queue entries and pending
    /// matches, then pair compatible players and publish each pairing as a
    /// ranked lobby in the directory.
    ///
    /// Returns the matches created by this tick.
    pub fn tick(&self) -> Vec<RankedMatch> {
        let now = now_ms();
        let mut state = self.state.lock();
        state
            .queue
            .retain(|entry| now.saturating_sub(entry.joined_ms) < QUEUE_TTL_MS);
        let expired: Vec<GameId> = state
            .matches
            .values()
            .filter(|record| record.state == MatchState::Pending && now >= record.expires_ms)
            .filter_map(|record| record.game_id)
            .collect();
        for record in state.matches.values_mut() {
            if record.state == MatchState::Pending && now >= record.expires_ms {
                record.state = MatchState::Expired;
                record.updated_ms = now;
            }
        }
        for game_id in expired {
            self.dir.withdraw(game_id);
        }

        let mut candidates: Vec<(LfgEntry, i32)> = state
            .queue
            .iter()
            .filter_map(|entry| {
                state
                    .players
                    .get(&entry.player_id)
                    .and_then(|profile| profile.rating(entry.kind))
                    .map(|rating| (entry.clone(), rating.rating))
            })
            .collect();
        candidates.sort_by_key(|(entry, _)| entry.joined_ms);

        let mut paired = Vec::new();
        let mut used: HashSet<usize> = HashSet::new();
        for first in 0..candidates.len() {
            if used.contains(&first) {
                continue;
            }
            let mut best: Option<(usize, i32)> = None;
            let first_tolerance = tolerance_for(now.saturating_sub(candidates[first].0.joined_ms));
            for second in (first + 1)..candidates.len() {
                if used.contains(&second)
                    || candidates[first].0.kind != candidates[second].0.kind
                    || candidates[first].0.controller != candidates[second].0.controller
                {
                    continue;
                }
                let gap = (candidates[first].1 - candidates[second].1).abs();
                let second_tolerance =
                    tolerance_for(now.saturating_sub(candidates[second].0.joined_ms));
                if gap > first_tolerance.max(second_tolerance) {
                    continue;
                }
                if best.is_none_or(|(_, best_gap)| gap < best_gap) {
                    best = Some((second, gap));
                }
            }
            let Some((second, _)) = best else { continue };
            used.insert(first);
            used.insert(second);
            paired.push((first, second));
        }

        let mut created = Vec::new();
        for (first, second) in paired {
            let (entry_a, rating_a) = &candidates[first];
            let (entry_b, rating_b) = &candidates[second];
            let map = entry_a
                .map
                .clone()
                .or_else(|| entry_b.map.clone())
                .unwrap_or_else(|| default_map(entry_a.kind).to_owned());
            let match_id = MatchId::new();
            let ladder = if entry_a.controller.is_ai() {
                "AI"
            } else {
                "Human"
            };
            let mut tags = vec![
                "ranked".to_owned(),
                "lfg".to_owned(),
                entry_a.kind.to_string(),
            ];
            if entry_a.controller.is_ai() {
                tags.push("ai".to_owned());
            }
            let publish = PublishGame {
                id: None,
                name: format!(
                    "Ranked {ladder} {}: {} vs {}",
                    entry_a.kind.as_str().to_ascii_uppercase(),
                    entry_a.name,
                    entry_b.name
                ),
                kind: entry_a.kind,
                map: map.clone(),
                host_name: entry_a.name.clone(),
                address: entry_a.address,
                players: 2,
                max_players: 2,
                state: GameState::Ready,
                passworded: true,
                version: env!("CARGO_PKG_VERSION").to_owned(),
                tags,
                ttl_secs: Some(900),
            };
            let game_id = match self.dir.announce(publish) {
                Ok(game) => Some(game.id),
                Err(err) => {
                    warn!(%match_id, error = %err, "failed to announce ranked lobby");
                    None
                }
            };
            let record = RankedMatch {
                id: match_id,
                kind: entry_a.kind,
                controller: entry_a.controller,
                map,
                state: MatchState::Pending,
                players: vec![
                    MatchPlayer {
                        player_id: entry_a.player_id,
                        name: entry_a.name.clone(),
                        controller: entry_a.controller,
                        agent: entry_a.agent.clone(),
                        rating: *rating_a,
                        league: League::from_rating(*rating_a),
                        team: 1,
                        address: entry_a.address,
                    },
                    MatchPlayer {
                        player_id: entry_b.player_id,
                        name: entry_b.name.clone(),
                        controller: entry_b.controller,
                        agent: entry_b.agent.clone(),
                        rating: *rating_b,
                        league: League::from_rating(*rating_b),
                        team: 2,
                        address: entry_b.address,
                    },
                ],
                created_ms: now,
                updated_ms: now,
                expires_ms: now + MATCH_TTL_MS,
                game_id,
                winner: None,
                reported_by: None,
                rating_changes: Vec::new(),
            };
            state.matches.insert(record.id, record.clone());
            created.push(record);
        }
        if !created.is_empty() {
            let paired_players: HashSet<PlayerId> = created
                .iter()
                .flat_map(|record| record.players.iter().map(|player| player.player_id))
                .collect();
            state
                .queue
                .retain(|entry| !paired_players.contains(&entry.player_id));
            for record in &created {
                info!(
                    match_id = %record.id,
                    kind = %record.kind,
                    controller = %record.controller,
                    "ranked match found"
                );
            }
        }
        if state.matches.len() > MATCH_HISTORY_MAX {
            trim_history(&mut state.matches);
        }
        created
    }

    /// Write the ladder snapshot when persistence is configured.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the parent directory cannot be created, the
    /// snapshot cannot be encoded, or the file cannot be written.
    pub fn save(&self) -> std::io::Result<()> {
        let Some(path) = self.state_path.as_deref() else {
            return Ok(());
        };
        let snapshot = self.state.lock().snapshot();
        let bytes = serde_json::to_vec_pretty(&snapshot).map_err(std::io::Error::other)?;
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, bytes)
    }
}

fn load_snapshot(path: &Path) -> LfgSnapshot {
    match std::fs::read(path) {
        Ok(bytes) => match serde_json::from_slice(&bytes) {
            Ok(snapshot) => snapshot,
            Err(err) => {
                warn!(path = %path.display(), error = %err, "ignoring malformed ladder snapshot");
                LfgSnapshot::default()
            }
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => LfgSnapshot::default(),
        Err(err) => {
            warn!(path = %path.display(), error = %err, "cannot read ladder snapshot");
            LfgSnapshot::default()
        }
    }
}

fn validate_address(address: SocketAddr) -> Result<(), LfgError> {
    if address.port() == 0 || address.ip().is_unspecified() {
        return Err(LfgError::InvalidAddress);
    }
    Ok(())
}

fn clean_optional(value: Option<String>) -> Option<String> {
    value
        .map(|text| text.trim().to_owned())
        .filter(|text| !text.is_empty())
        .map(|text| text.chars().take(64).collect())
}

fn default_map(kind: GameKind) -> &'static str {
    match kind {
        GameKind::TiberianDawn => TD_DEFAULT_MAP,
        GameKind::RedAlert => RA_DEFAULT_MAP,
    }
}

fn tolerance_for(wait_ms: i64) -> i32 {
    let waited_secs = i32::try_from(wait_ms / 1000).unwrap_or(i32::MAX);
    BASE_TOLERANCE
        .saturating_add(waited_secs.saturating_mul(TOLERANCE_PER_SEC))
        .min(MAX_TOLERANCE)
}

/// Probability that the first rating beats the second under the Elo model.
#[must_use]
pub fn expected_score(rating_a: i32, rating_b: i32) -> f64 {
    1.0 / (1.0 + 10f64.powf(f64::from(rating_b - rating_a) / 400.0))
}

fn k_factor(games: u32) -> f64 {
    if games < PROVISIONAL_GAMES {
        PROVISIONAL_K_FACTOR
    } else {
        K_FACTOR
    }
}

fn round_rating(delta: f64) -> i32 {
    let bounded = delta.clamp(-(RATING_MAX as f64), RATING_MAX as f64);
    bounded.round() as i32
}

fn clamp_rating(rating: i32) -> i32 {
    rating.clamp(RATING_MIN, RATING_MAX)
}

fn trim_history(matches: &mut HashMap<MatchId, RankedMatch>) {
    let mut settled: Vec<(MatchId, i64)> = matches
        .iter()
        .filter(|(_, record)| record.state != MatchState::Pending)
        .map(|(id, record)| (*id, record.updated_ms))
        .collect();
    settled.sort_by_key(|(_, updated)| *updated);
    let excess = matches.len().saturating_sub(MATCH_HISTORY_MAX);
    for (id, _) in settled.into_iter().take(excess) {
        matches.remove(&id);
    }
}

fn top_rating(state: &LfgState, controller: Controller) -> Option<i32> {
    state
        .players
        .values()
        .filter(|profile| profile.controller == controller)
        .flat_map(|profile| profile.ratings.iter().map(|rating| rating.rating))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::NodeId;

    fn service() -> LfgService {
        let dir = Arc::new(Directory::new(
            NodeId::new(),
            "lfg-test",
            "127.0.0.1:47770".parse().expect("addr"),
            None,
        ));
        LfgService::new(dir, None)
    }

    fn address(port: u16) -> SocketAddr {
        format!("203.0.113.{port}:47771")
            .parse()
            .expect("test address")
    }

    fn queued(service: &LfgService, name: &str, port: u16) -> (PlayerProfile, LfgEntry) {
        let player = service.register(name).expect("register");
        let entry = service
            .join(JoinQueue {
                player_id: player.id,
                kind: GameKind::RedAlert,
                address: address(port),
                map: None,
                note: None,
            })
            .expect("join");
        (player, entry)
    }

    fn ai_card(name: &str) -> AgentCard {
        AgentCard {
            agent_id: Some(1),
            owner: "HouseNod".to_owned(),
            doctrine: "Aggressive".to_owned(),
            script_hash: format!("{:040x}", name.len()),
            creator: Some("Kane".to_owned()),
            scenario: Some("SCG01EA".to_owned()),
        }
    }

    fn ai_queued(service: &LfgService, name: &str, port: u16) -> (PlayerProfile, LfgEntry) {
        let player = service
            .register_ai(name, ai_card(name))
            .expect("register ai");
        let entry = service
            .join(JoinQueue {
                player_id: player.id,
                kind: GameKind::RedAlert,
                address: address(port),
                map: None,
                note: None,
            })
            .expect("join ai");
        (player, entry)
    }

    #[test]
    fn rating_starts_in_gold_and_provisional() {
        let rating = KindRating::new(GameKind::RedAlert);
        assert_eq!(rating.rating, DEFAULT_RATING);
        assert_eq!(rating.league(), League::Gold);
        assert!(rating.provisional());
        assert_eq!(League::from_rating(4200), League::Grandmaster);
        assert_eq!(League::from_rating(0), League::Bronze);
    }

    #[test]
    fn expected_score_is_symmetric() {
        let a = expected_score(2000, 2000);
        assert!((a - 0.5).abs() < f64::EPSILON);
        assert!((expected_score(2400, 2000) + expected_score(2000, 2400) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn tick_pairs_similar_players_and_publishes_a_lobby() {
        let service = service();
        let (_, first) = queued(&service, "Kane", 1);
        let (_, second) = queued(&service, "GDI", 2);
        let created = service.tick();
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].players.len(), 2);
        assert_eq!(created[0].state, MatchState::Pending);
        assert!(created[0].game_id.is_some());
        assert!(service.queue(None).is_empty());
        assert_eq!(first.kind, second.kind);
        let games = service.dir.games(&crate::model::GameFilter::default());
        assert_eq!(games.len(), 1);
        assert!(games[0].tags.contains(&"ranked".to_owned()));
    }

    #[test]
    fn distant_ratings_wait_for_a_wider_window() {
        let service = service();
        let (strong, _) = queued(&service, "Veteran", 1);
        {
            let mut state = service.state.lock();
            let rating = state
                .players
                .get_mut(&strong.id)
                .and_then(|profile| profile.rating_mut(GameKind::RedAlert))
                .expect("rating");
            rating.rating = 3600;
        }
        let (weak, _) = queued(&service, "Rookie", 2);
        {
            let mut state = service.state.lock();
            let rating = state
                .players
                .get_mut(&weak.id)
                .and_then(|profile| profile.rating_mut(GameKind::RedAlert))
                .expect("rating");
            rating.rating = 1600;
        }
        assert!(service.tick().is_empty());
    }

    #[test]
    fn report_moves_ratings_and_updates_records() {
        let service = service();
        let (winner, _) = queued(&service, "Winner", 1);
        let (loser, _) = queued(&service, "Loser", 2);
        let created = service.tick();
        let record = &created[0];
        let settled = service
            .report(record.id, winner.id, winner.id)
            .expect("report");
        assert_eq!(settled.state, MatchState::Settled);
        assert_eq!(settled.winner, Some(winner.id));
        assert_eq!(settled.rating_changes.len(), 2);
        let winner_rating = service
            .profile(winner.id)
            .and_then(|profile| profile.rating(GameKind::RedAlert).cloned())
            .expect("winner rating");
        let loser_rating = service
            .profile(loser.id)
            .and_then(|profile| profile.rating(GameKind::RedAlert).cloned())
            .expect("loser rating");
        assert_eq!(winner_rating.wins, 1);
        assert_eq!(loser_rating.losses, 1);
        assert!(winner_rating.rating > DEFAULT_RATING);
        assert!(loser_rating.rating < DEFAULT_RATING);
        assert_eq!(service.matches(Some(winner.id), None, 10).len(), 1);
    }

    #[test]
    fn only_a_participant_can_report() {
        let service = service();
        let _ = queued(&service, "One", 1);
        let _ = queued(&service, "Two", 2);
        let outsider = service.register("Outsider").expect("register");
        let record = &service.tick()[0];
        assert_eq!(
            service
                .report(record.id, outsider.id, outsider.id)
                .expect_err("outsider report must fail"),
            LfgError::NotParticipant
        );
    }

    #[test]
    fn queue_and_status_reflect_state() {
        let service = service();
        let _ = queued(&service, "Alpha", 1);
        let status = service.status();
        assert_eq!(status.players, 1);
        assert_eq!(status.queued, 1);
        assert_eq!(service.leaderboard(Some(GameKind::RedAlert), 10).len(), 1);
        assert_eq!(service.queue(Some(GameKind::TiberianDawn)).len(), 0);
    }

    #[test]
    fn ai_and_human_queues_never_pair() {
        let service = service();
        let _ = queued(&service, "Human", 1);
        let _ = ai_queued(&service, "Agent", 2);
        assert!(service.tick().is_empty());
        assert_eq!(service.queue(None).len(), 1);
        assert_eq!(service.queue_for(Controller::Ai, None).len(), 1);
        assert_eq!(service.status().queued, 1);
        assert_eq!(service.status().ai_queued, 1);
    }

    #[test]
    fn ai_only_matches_are_rated_on_the_ai_ladder() {
        let service = service();
        let (agent_a, _) = ai_queued(&service, "Agent A", 1);
        let (_agent_b, _) = ai_queued(&service, "Agent B", 2);
        let created = service.tick();
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].controller, Controller::Ai);
        assert_eq!(
            created[0].players[0]
                .agent
                .as_ref()
                .expect("agent card")
                .doctrine,
            "Aggressive"
        );
        let games = service.dir.games(&crate::model::GameFilter::default());
        assert_eq!(games.len(), 1);
        assert!(games[0].tags.contains(&"ai".to_owned()));
        let settled = service
            .report(created[0].id, agent_a.id, agent_a.id)
            .expect("report ai");
        assert_eq!(settled.controller, Controller::Ai);
        assert!(service.leaderboard(Some(GameKind::RedAlert), 10).is_empty());
        let rows = service.leaderboard_for(Controller::Ai, Some(GameKind::RedAlert), 10);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].controller, Controller::Ai);
        assert_eq!(service.status().settled_matches, 0);
        assert_eq!(service.status().ai_settled_matches, 1);
    }

    #[test]
    fn human_registration_rejects_agent_metadata() {
        let service = service();
        assert_eq!(
            service
                .register_competitor("Human", Controller::Human, Some(ai_card("x")))
                .expect_err("human cannot carry an agent card"),
            LfgError::InvalidAgent
        );
    }

    #[test]
    fn persistence_roundtrips_ratings() {
        let dir = Arc::new(Directory::new(
            NodeId::new(),
            "lfg-test",
            "127.0.0.1:47770".parse().expect("addr"),
            None,
        ));
        let path = std::env::temp_dir().join(format!("cnc-lfg-{}.json", Uuid::new_v4()));
        let service = LfgService::new(dir.clone(), Some(path.clone()));
        let (winner, _) = queued(&service, "Winner", 1);
        let (_, _) = queued(&service, "Loser", 2);
        let (agent, _) = ai_queued(&service, "Persist AI", 3);
        let record = &service.tick()[0];
        service
            .report(record.id, winner.id, winner.id)
            .expect("report");
        service.save().expect("save");
        let reopened = LfgService::new(dir, Some(path.clone()));
        let rating = reopened
            .profile(winner.id)
            .and_then(|profile| profile.rating(GameKind::RedAlert).cloned())
            .expect("persisted profile");
        assert_eq!(rating.wins, 1);
        assert_eq!(
            reopened
                .profile(agent.id)
                .and_then(|profile| profile.agent)
                .map(|card| card.doctrine),
            Some("Aggressive".to_owned())
        );
        assert_eq!(
            reopened.queue_for(Controller::Ai, None).len(),
            1,
            "AI queue survives a restart"
        );
        let _ = std::fs::remove_file(path);
    }
}
