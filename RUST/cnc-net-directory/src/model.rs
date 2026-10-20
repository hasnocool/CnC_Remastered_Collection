//! Core directory records shared by the wire protocol, the registry, and the
//! HTTP API.

use std::fmt;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ValidationError;

/// Longest accepted game or map name.
pub const NAME_MAX: usize = 64;
/// Longest accepted build/version string.
pub const VERSION_MAX: usize = 32;
/// Maximum tags attached to one game.
pub const TAGS_MAX: usize = 8;
/// Maximum length of a single tag.
pub const TAG_LEN: usize = 24;
/// Admission TTL floor, in seconds.
pub const TTL_MIN: u32 = 10;
/// Admission TTL ceiling, in seconds.
pub const TTL_MAX: u32 = 3600;
/// TTL applied when an announcement omits one.
pub const TTL_DEFAULT: u32 = 120;
/// Smallest supported lobby capacity.
pub const PLAYERS_MIN: u8 = 1;
/// Largest supported lobby capacity (the remastered engines cap at eight).
pub const PLAYERS_MAX: u8 = 8;
/// How often a publishing host should refresh its record.
pub const HEARTBEAT_SECS: u64 = 30;

/// Milliseconds since the Unix epoch.
#[must_use]
pub fn now_ms() -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_millis()).unwrap_or(i64::MAX),
        Err(_) => 0,
    }
}

/// Identifier of a published lobby.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GameId(Uuid);

impl GameId {
    /// Generate a fresh identifier.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for GameId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GameId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for GameId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s.trim()).map(Self)
    }
}

/// Identifier of a directory node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(Uuid);

impl NodeId {
    /// Generate a fresh node identity.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Raw bytes, used for deterministic merge tie-breaks.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for NodeId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s.trim()).map(Self)
    }
}

/// Which remastered title a lobby belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameKind {
    /// Command & Conquer: Tiberian Dawn.
    #[serde(rename = "td")]
    TiberianDawn,
    /// Command & Conquer: Red Alert.
    #[serde(rename = "ra")]
    RedAlert,
}

impl GameKind {
    /// Stable wire/API name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TiberianDawn => "td",
            Self::RedAlert => "ra",
        }
    }
}

impl fmt::Display for GameKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for GameKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "td" | "tiberian-dawn" | "tiberiandawn" | "c&c" | "cnc" => Ok(Self::TiberianDawn),
            "ra" | "red-alert" | "redalert" => Ok(Self::RedAlert),
            other => Err(format!("unknown game kind {other:?} (expected td or ra)")),
        }
    }
}

/// Lifecycle of a lobby, as seen by browsers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameState {
    /// Open and waiting for players.
    #[default]
    Lobby,
    /// Queued or full; the host is about to start.
    Ready,
    /// Match already running.
    InProgress,
}

impl GameState {
    /// Stable wire/API name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lobby => "lobby",
            Self::Ready => "ready",
            Self::InProgress => "in_progress",
        }
    }

    /// Whether the lobby can still accept a human player.
    #[must_use]
    pub const fn accepts_players(self, players: u8, max_players: u8) -> bool {
        matches!(self, Self::Lobby) && players < max_players
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for GameState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "lobby" | "open" | "waiting" => Ok(Self::Lobby),
            "ready" | "queued" | "queue" | "starting" => Ok(Self::Ready),
            "in_progress" | "in-progress" | "playing" | "running" => Ok(Self::InProgress),
            other => Err(format!("unknown game state {other:?}")),
        }
    }
}

/// A node participating in the mesh.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Stable identity for the process lifetime.
    pub id: NodeId,
    /// Human-readable node name.
    pub name: String,
    /// UDP gossip address.
    pub gossip_addr: SocketAddr,
    /// Optional HTTP browse address.
    #[serde(default)]
    pub http_addr: Option<SocketAddr>,
    /// Crate version of the node.
    pub version: String,
    /// Wall-clock start time, in Unix milliseconds.
    pub started_at_ms: i64,
    /// Number of non-tombstone games the node currently holds.
    #[serde(default)]
    pub games: u32,
}

/// One browsable lobby.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameInfo {
    /// Stable lobby identifier.
    pub id: GameId,
    /// Display name, e.g. "Nod vs GDI - 2v2".
    pub name: String,
    /// Which title this lobby belongs to.
    pub kind: GameKind,
    /// Map filename or display name.
    #[serde(default)]
    pub map: String,
    /// Nickname of the host.
    #[serde(default)]
    pub host_name: String,
    /// Address players dial to join the game session.
    pub address: SocketAddr,
    /// Players currently in the lobby.
    pub players: u8,
    /// Lobby capacity.
    pub max_players: u8,
    /// Lobby lifecycle state.
    pub state: GameState,
    /// Whether the lobby requires a password.
    #[serde(default)]
    pub passworded: bool,
    /// Game build/version string.
    #[serde(default)]
    pub version: String,
    /// Free-form tags (mods, rulesets, region).
    #[serde(default)]
    pub tags: Vec<String>,
    /// Seconds the record stays live without a refresh.
    pub ttl_secs: u32,
    /// Node that first accepted the record (merge tie-break).
    pub origin: NodeId,
    /// Monotonic-per-origin revision, also used for ordering.
    pub revision: u64,
    /// Wall-clock time of the write, in Unix milliseconds.
    pub updated_at_ms: i64,
    /// Tombstone marker: the lobby was withdrawn.
    #[serde(default)]
    pub deleted: bool,
}

impl GameInfo {
    /// Deterministic merge order: newer wall clock wins, ties break by origin
    /// then revision. Both replicas agree without a coordinator.
    #[must_use]
    pub fn order_key(&self) -> (i64, [u8; 16], u64) {
        (self.updated_at_ms, *self.origin.as_bytes(), self.revision)
    }

    /// Whether this record is a withdrawal tombstone.
    #[must_use]
    pub const fn is_tombstone(&self) -> bool {
        self.deleted
    }

    /// Human-readable "players/max" pair.
    #[must_use]
    pub fn occupancy(&self) -> String {
        format!("{}/{}", self.players, self.max_players)
    }
}

/// An announcement sent by a game host to any directory node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishGame {
    /// Keep the same id across heartbeats; omitted means "assign one".
    #[serde(default)]
    pub id: Option<GameId>,
    /// Display name.
    pub name: String,
    /// Which title.
    pub kind: GameKind,
    /// Map filename or display name.
    #[serde(default)]
    pub map: String,
    /// Host nickname.
    #[serde(default)]
    pub host_name: String,
    /// Address players dial to join.
    pub address: SocketAddr,
    /// Players currently in the lobby.
    #[serde(default = "default_players")]
    pub players: u8,
    /// Lobby capacity.
    pub max_players: u8,
    /// Lobby lifecycle state.
    #[serde(default)]
    pub state: GameState,
    /// Whether the lobby requires a password.
    #[serde(default)]
    pub passworded: bool,
    /// Game build/version string.
    #[serde(default)]
    pub version: String,
    /// Free-form tags.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Optional TTL override.
    #[serde(default)]
    pub ttl_secs: Option<u32>,
}

fn default_players() -> u8 {
    1
}

/// A validated, normalised announcement ready for the registry.
#[derive(Debug, Clone)]
pub struct ValidatedGame {
    /// Trimmed display name.
    pub name: String,
    /// Map name.
    pub map: String,
    /// Host nickname.
    pub host_name: String,
    /// Join address.
    pub address: SocketAddr,
    /// Current players.
    pub players: u8,
    /// Capacity.
    pub max_players: u8,
    /// Lifecycle state.
    pub state: GameState,
    /// Whether a password is required.
    pub passworded: bool,
    /// Build string.
    pub version: String,
    /// Normalised tags.
    pub tags: Vec<String>,
    /// Effective TTL.
    pub ttl_secs: u32,
}

impl PublishGame {
    /// Validate and normalise an announcement.
    ///
    /// # Errors
    ///
    /// Returns [ValidationError] when a field is empty, oversized, or
    /// semantically inconsistent.
    pub fn validate(&self) -> Result<ValidatedGame, ValidationError> {
        let name = self.name.trim();
        if name.is_empty() || name.chars().count() > NAME_MAX {
            return Err(ValidationError::Name { max: NAME_MAX });
        }
        let map = self.map.trim();
        if map.is_empty() || map.chars().count() > NAME_MAX {
            return Err(ValidationError::Map { max: NAME_MAX });
        }
        let ip = self.address.ip();
        if self.address.port() == 0 || ip.is_unspecified() {
            return Err(ValidationError::Address(self.address));
        }
        if !(PLAYERS_MIN..=PLAYERS_MAX).contains(&self.max_players) {
            return Err(ValidationError::Capacity {
                value: self.max_players,
                min: PLAYERS_MIN,
                max: PLAYERS_MAX,
            });
        }
        if self.players == 0 || self.players > self.max_players {
            return Err(ValidationError::Players {
                players: self.players,
                max_players: self.max_players,
            });
        }
        let ttl_secs = self.ttl_secs.unwrap_or(TTL_DEFAULT);
        if !(TTL_MIN..=TTL_MAX).contains(&ttl_secs) {
            return Err(ValidationError::Ttl {
                value: ttl_secs,
                min: TTL_MIN,
                max: TTL_MAX,
            });
        }
        let version = self.version.trim();
        if version.chars().count() > VERSION_MAX {
            return Err(ValidationError::Version(VERSION_MAX));
        }
        if self.tags.len() > TAGS_MAX {
            return Err(ValidationError::Tags {
                max_tags: TAGS_MAX,
                max_len: TAG_LEN,
            });
        }
        let mut tags = Vec::with_capacity(self.tags.len());
        for tag in &self.tags {
            let tag = tag.trim();
            if tag.chars().count() > TAG_LEN {
                return Err(ValidationError::Tags {
                    max_tags: TAGS_MAX,
                    max_len: TAG_LEN,
                });
            }
            if tag.is_empty() || tags.iter().any(|t: &String| t == tag) {
                continue;
            }
            tags.push(tag.to_owned());
        }
        Ok(ValidatedGame {
            name: name.to_owned(),
            map: map.to_owned(),
            host_name: self.host_name.trim().to_owned(),
            address: self.address,
            players: self.players,
            max_players: self.max_players,
            state: self.state,
            passworded: self.passworded,
            version: version.to_owned(),
            tags,
            ttl_secs,
        })
    }
}

/// Query accepted by the HTTP browse API and the registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameFilter {
    /// Restrict to one lifecycle state.
    #[serde(default)]
    pub state: Option<GameState>,
    /// Restrict to one title.
    #[serde(default)]
    pub kind: Option<GameKind>,
    /// Case-insensitive substring match over name, map, host, and tags.
    #[serde(default)]
    pub query: Option<String>,
    /// Minimum joined players.
    #[serde(default)]
    pub min_players: Option<u8>,
    /// Maximum joined players.
    #[serde(default)]
    pub max_players: Option<u8>,
    /// Skip password-protected lobbies.
    #[serde(default)]
    pub open_only: bool,
    /// Only lobbies that still have a free slot.
    #[serde(default)]
    pub with_slots: bool,
    /// Result cap.
    #[serde(default)]
    pub limit: Option<usize>,
}

impl GameFilter {
    /// Default result cap when the caller omits a limit.
    pub const DEFAULT_LIMIT: usize = 200;

    /// Effective limit, clamped to a safe ceiling.
    #[must_use]
    pub fn effective_limit(&self) -> usize {
        self.limit.unwrap_or(Self::DEFAULT_LIMIT).clamp(1, 1000)
    }

    /// Whether a record satisfies every active predicate.
    #[must_use]
    pub fn matches(&self, game: &GameInfo) -> bool {
        if self.state.is_some_and(|s| s != game.state) {
            return false;
        }
        if self.kind.is_some_and(|k| k != game.kind) {
            return false;
        }
        if self.open_only && game.passworded {
            return false;
        }
        if self.with_slots && !game.state.accepts_players(game.players, game.max_players) {
            return false;
        }
        if self.min_players.is_some_and(|m| game.players < m) {
            return false;
        }
        if self.max_players.is_some_and(|m| game.players > m) {
            return false;
        }
        match &self.query {
            None => true,
            Some(q) => {
                let q = q.trim().to_lowercase();
                if q.is_empty() {
                    return true;
                }
                game.name.to_lowercase().contains(&q)
                    || game.map.to_lowercase().contains(&q)
                    || game.host_name.to_lowercase().contains(&q)
                    || game.tags.iter().any(|t| t.to_lowercase().contains(&q))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> PublishGame {
        PublishGame {
            id: None,
            name: "Nod vs GDI".to_owned(),
            kind: GameKind::TiberianDawn,
            map: "SCG01EA".to_owned(),
            host_name: "Kane".to_owned(),
            address: "203.0.113.7:47771".parse().expect("valid addr"),
            players: 2,
            max_players: 4,
            state: GameState::Lobby,
            passworded: false,
            version: "1.0".to_owned(),
            tags: vec!["2v2".to_owned(), " 2v2 ".to_owned(), String::new()],
            ttl_secs: None,
        }
    }

    #[test]
    fn validation_normalises_tags_and_defaults_ttl() {
        let v = sample().validate().expect("valid");
        assert_eq!(v.ttl_secs, TTL_DEFAULT);
        assert_eq!(v.tags, vec!["2v2".to_owned()]);
    }

    #[test]
    fn validation_rejects_bad_capacity() {
        let mut g = sample();
        g.max_players = 12;
        assert!(matches!(
            g.validate(),
            Err(ValidationError::Capacity { max: 8, .. })
        ));
    }

    #[test]
    fn validation_rejects_unspecified_address() {
        let mut g = sample();
        g.address = "0.0.0.0:47771".parse().expect("valid addr");
        assert!(matches!(g.validate(), Err(ValidationError::Address(_))));
    }

    #[test]
    fn order_key_breaks_ties_deterministically() {
        let mut a = sample().validate().expect("valid");
        let mut b = a.clone();
        let origin_a = NodeId::new();
        let origin_b = NodeId::new();
        a.name = "a".to_owned();
        b.name = "b".to_owned();
        let ga = GameInfo {
            id: GameId::new(),
            name: a.name,
            kind: GameKind::TiberianDawn,
            map: a.map,
            host_name: a.host_name,
            address: a.address,
            players: a.players,
            max_players: a.max_players,
            state: a.state,
            passworded: a.passworded,
            version: a.version,
            tags: a.tags,
            ttl_secs: a.ttl_secs,
            origin: origin_a,
            revision: 1,
            updated_at_ms: 100,
            deleted: false,
        };
        let mut gb = ga.clone();
        gb.origin = origin_b;
        gb.updated_at_ms = 101;
        assert!(gb.order_key() > ga.order_key());
        let mut gc = ga.clone();
        gc.origin = origin_b;
        assert_eq!(gc.order_key() > ga.order_key(), origin_b > origin_a);
    }

    #[test]
    fn filter_matches_case_insensitively() {
        let mut g = sample().validate().expect("valid");
        let id = GameId::new();
        g.name = "Nod vs GDI".to_owned();
        let info = GameInfo {
            id,
            name: g.name,
            kind: GameKind::TiberianDawn,
            map: g.map,
            host_name: g.host_name,
            address: g.address,
            players: g.players,
            max_players: g.max_players,
            state: g.state,
            passworded: g.passworded,
            version: g.version,
            tags: g.tags,
            ttl_secs: g.ttl_secs,
            origin: NodeId::new(),
            revision: 1,
            updated_at_ms: 0,
            deleted: false,
        };
        let f = GameFilter {
            query: Some("nod".to_owned()),
            ..GameFilter::default()
        };
        assert!(f.matches(&info));
        let f = GameFilter {
            query: Some("gdi".to_owned()),
            ..GameFilter::default()
        };
        assert!(f.matches(&info));
        let f = GameFilter {
            query: Some("scrinn".to_owned()),
            ..GameFilter::default()
        };
        assert!(!f.matches(&info));
    }

    #[test]
    fn game_kind_and_state_roundtrip() {
        for kind in [GameKind::TiberianDawn, GameKind::RedAlert] {
            assert_eq!(kind.to_string().parse::<GameKind>().expect("parse"), kind);
        }
        for state in [GameState::Lobby, GameState::Ready, GameState::InProgress] {
            assert_eq!(
                state.to_string().parse::<GameState>().expect("parse"),
                state
            );
        }
    }
}
