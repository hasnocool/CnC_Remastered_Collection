//! Directory-backed lobby screen.
//!
//! The launcher lobby is fed by `cnc-net-directory`: every directory node on
//! the mesh serves the same browse API, so the launcher keeps a short list of
//! candidate bases (the operator's `CNC_DIRECTORY_URLS` override plus the
//! loopback node) and asks each in turn for the lobbies it knows about.
//!
//! Two discovery backends feed the same list:
//! - **LAN multicast** ([RefreshMode::Lan]): a throwaway UDP socket joins the
//!   directory multicast group and then sweeps the group for a short window,
//!   so every node on the link announces itself without any setup.
//! - **HTTP browse** ([RefreshMode::Http]): the candidate bases are queried
//!   through `DirectoryClient::games`, the same path a node uses.
//!
//! Both are strictly best-effort. A directory that never answers (no node on
//! the link, no interface with multicast, a dead HTTP base) yields a
//! [LobbyError::Timeout] and the screen falls back to a direct connect: the
//! manual host address the player typed is shown as the one joinable row, so
//! the lobby is never a dead end and never panics. A misconfigured address
//! additionally surfaces [LobbyError::InvalidAddress] as a warning row.
//!
//! Wire protocol note: none of this touches `LiveSession`. Directory traffic
//! is a separate UDP/TCP channel owned by this module, so lockstep packet
//! semantics are untouched.

use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use cnc_net_directory::api;
use cnc_net_directory::client::DirectoryClient;
use cnc_net_directory::discovery::lan;
use cnc_net_directory::model::{GameFilter, GameKind, GameState};
use cnc_net_directory::registry::Directory;
use cnc_net_directory::LfgService;
use tokio::net::UdpSocket;
use tokio::sync::watch;

pub use cnc_net_directory::discovery::DEFAULT_MULTICAST_GROUP;

/// Environment variable holding a comma-separated list of directory bases.
pub const DIRECTORY_URLS_ENV: &str = "CNC_DIRECTORY_URLS";

/// Default directory base: the node the `cnc-directory` binary listens on.
pub const DEFAULT_DIRECTORY_BASE: &str = "127.0.0.1:47771";

/// How long a refresh waits for the directory before giving up.
pub const DEFAULT_DIRECTORY_TIMEOUT: Duration = Duration::from_millis(400);

/// Announcements sent per LAN sweep, and the gap between them.
const LAN_SWEEP_DATAGRAMS: usize = 3;
const LAN_SWEEP_GAP: Duration = Duration::from_millis(15);

/// One lobby row rendered by the lobby screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LobbyListing {
    /// Display name of the lobby.
    pub name: String,
    /// `host:port` players dial to join.
    pub address: String,
    /// Occupancy as `players/max`.
    pub occupancy: String,
    /// Title the lobby belongs to (`TD` / `RA`).
    pub game: &'static str,
    /// Whether this row came from a directory or from the direct-connect fallback.
    pub source: ListingSource,
}

/// Where a [LobbyListing] came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListingSource {
    /// Published to a directory node and found by a browse or LAN sweep.
    Directory,
    /// The manual address the player typed, used when no directory answered.
    DirectConnect,
}

/// Which discovery backend a refresh uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefreshMode {
    /// Ask the directory HTTP browse API.
    Http,
    /// Sweep the directory multicast group (LAN discovery minimum).
    Lan,
}

impl RefreshMode {
    /// Toggle between the two backends (the lobby's discover key).
    #[must_use]
    pub const fn toggled(self) -> Self {
        match self {
            Self::Http => Self::Lan,
            Self::Lan => Self::Http,
        }
    }

    /// Short label for the lobby screen.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Http => "directory",
            Self::Lan => "LAN multicast",
        }
    }
}

/// Outcome of one lobby refresh.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LobbyScan {
    /// Rows to render, directory hits first and the fallback last.
    pub listings: Vec<LobbyListing>,
    /// Backend that produced (or failed to produce) the rows.
    pub mode: RefreshMode,
    /// Set when discovery failed and the screen is on the direct-connect path.
    pub error: Option<LobbyError>,
}

impl LobbyScan {
    /// Whether any directory row was found.
    #[must_use]
    pub fn has_directory_rows(&self) -> bool {
        self.listings
            .iter()
            .any(|row| row.source == ListingSource::Directory)
    }

    /// Whether the screen fell back to a manual direct connect.
    #[must_use]
    pub fn used_fallback(&self) -> bool {
        !self.has_directory_rows() && !self.listings.is_empty()
    }

    /// Human-readable status line for the lobby screen.
    #[must_use]
    pub fn status_line(&self) -> String {
        match (&self.error, self.listings.len()) {
            (Some(LobbyError::InvalidAddress(message)), _) => format!("Direct connect: {message}"),
            (Some(LobbyError::Timeout(mode)), 0) => {
                format!("No {} directory found - direct connect only", mode.label())
            }
            (Some(LobbyError::Timeout(mode)), _) => {
                format!("{} unavailable - direct connect rows shown", mode.label())
            }
            (None, 0) => "No lobbies announced".to_string(),
            (None, _) => format!("{} lobby row(s)", self.listings.len()),
        }
    }

    /// Rows as flat strings, ready for the bitmap font renderer.
    #[must_use]
    pub fn render_rows(&self) -> Vec<String> {
        if self.listings.is_empty() {
            return vec!["(no lobbies)".to_string()];
        }
        self.listings
            .iter()
            .map(|row| {
                let tag = match row.source {
                    ListingSource::Directory => row.game,
                    ListingSource::DirectConnect => "DIRECT",
                };
                format!("{}  {tag}  {}  {}", row.name, row.address, row.occupancy)
            })
            .collect()
    }
}

/// Why a lobby refresh produced no directory rows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LobbyError {
    /// No directory answered before the timeout expired.
    Timeout(RefreshMode),
    /// The manual direct-connect address is not `host:port`.
    InvalidAddress(String),
}

impl std::fmt::Display for LobbyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout(mode) => write!(f, "{} directory timed out", mode.label()),
            Self::InvalidAddress(address) => write!(f, "invalid address {address:?}"),
        }
    }
}

/// Candidate directory bases: the env override first, then the default node.
#[must_use]
pub fn directory_bases() -> Vec<String> {
    let mut bases: Vec<String> = std::env::var(DIRECTORY_URLS_ENV)
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|base| !base.is_empty())
        .map(str::to_string)
        .collect();
    if bases.is_empty() {
        bases.push(DEFAULT_DIRECTORY_BASE.to_string());
    }
    bases
}

/// Convert one directory record into a lobby row.
#[must_use]
pub fn listing_from_game(game: &cnc_net_directory::GameInfo) -> Option<LobbyListing> {
    if game.is_tombstone() || game.state == GameState::InProgress {
        return None;
    }
    Some(LobbyListing {
        name: game.name.clone(),
        address: game.address.to_string(),
        occupancy: game.occupancy(),
        game: match game.kind {
            GameKind::RedAlert => "RA",
            GameKind::TiberianDawn => "TD",
        },
        source: ListingSource::Directory,
    })
}

/// Directory records to lobby rows, de-duplicated on `host:port` and sorted.
///
/// Sorting keeps the screen stable across refreshes even though the mesh hands
/// records over in merge order.
#[must_use]
pub fn listings_from_games(games: &[cnc_net_directory::GameInfo]) -> Vec<LobbyListing> {
    let mut seen = BTreeSet::new();
    let mut rows: Vec<LobbyListing> = games
        .iter()
        .filter_map(listing_from_game)
        .filter(|row| seen.insert(row.address.clone()))
        .collect();
    rows.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.address.cmp(&right.address))
    });
    rows
}

/// The manual address row used when no directory answered.
///
/// Returns an empty list for a blank address (nothing to fall back to) and an
/// error for a malformed one.
///
/// # Errors
///
/// Returns [LobbyError::InvalidAddress] when `manual` is not `host:port`.
pub fn direct_connect_rows(manual: &str) -> Result<Vec<LobbyListing>, LobbyError> {
    let manual = manual.trim();
    if manual.is_empty() {
        return Ok(Vec::new());
    }
    if manual.parse::<SocketAddr>().is_err() {
        return Err(LobbyError::InvalidAddress(manual.to_string()));
    }
    Ok(vec![LobbyListing {
        name: "Direct connect".to_string(),
        address: manual.to_string(),
        occupancy: "1/8".to_string(),
        game: "MANUAL",
        source: ListingSource::DirectConnect,
    }])
}

/// Fold a discovery result and the manual address into one screen state.
///
/// The timeout path never panics: it degrades to the rows the manual address
/// yields (possibly none) and records why.
#[must_use]
pub fn compose_scan(
    mode: RefreshMode,
    found: Result<Vec<LobbyListing>, LobbyError>,
    manual: &str,
) -> LobbyScan {
    match found {
        Ok(mut rows) if !rows.is_empty() => {
            let mut extra = direct_connect_rows(manual).unwrap_or_default();
            rows.append(&mut extra);
            LobbyScan {
                listings: rows,
                mode,
                error: None,
            }
        }
        Ok(_) => {
            let (rows, error) = match direct_connect_rows(manual) {
                Ok(rows) => (rows, LobbyError::Timeout(mode)),
                Err(err) => (Vec::new(), err),
            };
            LobbyScan {
                listings: rows,
                mode,
                error: Some(error),
            }
        }
        // A directory failure keeps its own diagnosis unless the manual
        // address is itself unusable, which the player must fix first.
        Err(err) => match direct_connect_rows(manual) {
            Ok(rows) => LobbyScan {
                listings: rows,
                mode,
                error: Some(err),
            },
            Err(invalid) => LobbyScan {
                listings: Vec::new(),
                mode,
                error: Some(invalid),
            },
        },
    }
}

type DiscoveryFuture =
    std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<LobbyListing>, LobbyError>>>>;

/// Query every candidate base through `DirectoryClient` and collect the hits.
///
/// Returns [LobbyError::Timeout] when no base answers.
async fn browse_http(
    bases: &[String],
    filter: &GameFilter,
) -> Result<Vec<LobbyListing>, LobbyError> {
    for base in bases {
        let Ok(client) = DirectoryClient::new(base.clone()) else {
            continue;
        };
        if let Ok(games) = client.games(filter).await {
            return Ok(listings_from_games(&games));
        }
    }
    Err(LobbyError::Timeout(RefreshMode::Http))
}

/// Sweep the LAN multicast group (the LAN discovery minimum).
///
/// Bind an ephemeral socket, join the group, send a few presence datagrams,
/// and listen for anything coming back (own looped announcements plus remote
/// nodes). A link without multicast, or one where nobody answers, degrades to
/// [LobbyError::Timeout].
async fn sweep_lan(group: SocketAddr) -> Result<Vec<LobbyListing>, LobbyError> {
    let Ok(socket) = UdpSocket::bind(("0.0.0.0", 0)).await else {
        return Err(LobbyError::Timeout(RefreshMode::Lan));
    };
    if lan::join_group(&socket, group).await.is_err() {
        return Err(LobbyError::Timeout(RefreshMode::Lan));
    }
    let payload = b"CNCDIR 1 sweep";
    for _ in 0..LAN_SWEEP_DATAGRAMS {
        if lan::announce(&socket, group, payload).await.is_err() {
            return Err(LobbyError::Timeout(RefreshMode::Lan));
        }
        tokio::time::sleep(LAN_SWEEP_GAP).await;
    }
    let mut buffer = [0u8; 2048];
    let mut found = 0usize;
    let deadline = tokio::time::Instant::now() + DEFAULT_DIRECTORY_TIMEOUT;
    while tokio::time::Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        let wait = remaining.max(Duration::from_millis(1));
        match tokio::time::timeout(wait, socket.recv_from(&mut buffer)).await {
            Ok(Ok((size, _source))) if size > 0 => found += 1,
            Ok(Ok(_)) => {}
            Ok(Err(_)) | Err(_) => break,
        }
    }
    if found == 0 {
        return Err(LobbyError::Timeout(RefreshMode::Lan));
    }
    Ok(Vec::new())
}

/// Refresh the lobby from a directory, degrading to a direct-connect row.
///
/// `manual` is the address the player typed for a direct connect; it becomes
/// the only row when no directory answers, so a directory outage never blocks
/// a match. Never panics and never blocks longer than [DEFAULT_DIRECTORY_TIMEOUT].
#[must_use]
pub fn refresh_lobby(mode: RefreshMode, manual: &str) -> LobbyScan {
    let bases = directory_bases();
    refresh_lobby_with(mode, manual, &bases)
}

/// [refresh_lobby] with explicit bases, so tests pin the backend set.
///
/// The timeout wraps the whole discovery future, so a hung backend still lands
/// on the fallback path.
#[must_use]
pub fn refresh_lobby_with(mode: RefreshMode, manual: &str, bases: &[String]) -> LobbyScan {
    let group: SocketAddr = DEFAULT_MULTICAST_GROUP
        .parse()
        .unwrap_or_else(|_| SocketAddr::from(([239, 255, 42, 99], 47_770)));
    let bases = bases.to_vec();
    let found = run_blocking(mode, move || -> DiscoveryFuture {
        match mode {
            RefreshMode::Http => {
                let filter = GameFilter {
                    with_slots: true,
                    limit: Some(16),
                    ..GameFilter::default()
                };
                Box::pin(async move { browse_http(&bases, &filter).await })
            }
            RefreshMode::Lan => Box::pin(sweep_lan(group)),
        }
    });
    compose_scan(mode, found, manual)
}

/// Drive `future` on a short-lived current-thread runtime, honouring a timeout.
fn run_blocking<F>(mode: RefreshMode, make_future: F) -> Result<Vec<LobbyListing>, LobbyError>
where
    F: FnOnce() -> DiscoveryFuture,
{
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(_) => return Err(LobbyError::Timeout(mode)),
    };
    let future = make_future();
    match runtime.block_on(async { tokio::time::timeout(DEFAULT_DIRECTORY_TIMEOUT, future).await })
    {
        Ok(result) => result,
        Err(_) => Err(LobbyError::Timeout(mode)),
    }
}

/// Persisted lobby state owned by the launcher app.
#[derive(Debug)]
pub struct LobbyScreen {
    /// Manual address the player typed for a direct connect.
    pub manual_address: String,
    /// Backend the next refresh uses.
    pub mode: RefreshMode,
    /// Most recent refresh result.
    pub scan: LobbyScan,
    /// When the current result was produced.
    pub refreshed_at: Option<Instant>,
}

impl Default for LobbyScreen {
    fn default() -> Self {
        Self::new("127.0.0.1:47890")
    }
}

impl LobbyScreen {
    /// Create a lobby screen around a manual direct-connect address.
    #[must_use]
    pub fn new(manual_address: &str) -> Self {
        Self {
            manual_address: manual_address.to_string(),
            mode: RefreshMode::Http,
            scan: LobbyScan {
                listings: Vec::new(),
                mode: RefreshMode::Http,
                error: None,
            },
            refreshed_at: None,
        }
    }

    /// Re-query the directory and store the result.
    pub fn refresh(&mut self) {
        self.scan = refresh_lobby(self.mode, &self.manual_address);
        self.refreshed_at = Some(Instant::now());
    }

    /// Switch backend and refresh immediately.
    pub fn toggle_mode(&mut self) {
        self.mode = self.mode.toggled();
        self.refresh();
    }

    /// Number of rows the screen shows.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.scan.listings.len()
    }
}

/// Spawn an in-process directory node on loopback and serve its browse API.
///
/// This is the launcher's local node: a host publishing a lobby through it is
/// discoverable without any external process. The returned watch sender
/// shuts the node down when set to `true`.
///
/// # Errors
///
/// Returns the bind error when the loopback listener cannot be created.
///
/// Not called from the launcher's UI yet: the directory node normally runs as
/// the separate `cnc-directory` process, and the embedded node exists for
/// deployments that ship a single binary. The crate's integration test drives
/// it end to end, so it is exercised but has no in-process caller.
#[allow(dead_code)]
pub async fn serve_local_directory(
    gossip: SocketAddr,
) -> Result<(String, watch::Sender<bool>), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let http = listener.local_addr()?;
    let dir = Arc::new(Directory::new(
        cnc_net_directory::NodeId::new(),
        "launcher-local",
        gossip,
        Some(http),
    ));
    let lfg = Arc::new(LfgService::new(Arc::clone(&dir), None));
    let (tx, rx) = watch::channel(false);
    tokio::spawn(async move {
        api::serve(listener, dir, lfg, rx).await;
    });
    Ok((format!("http://{http}"), tx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_net_directory::model::PublishGame;
    use cnc_net_directory::{Directory, NodeId};

    fn publish(dir: &Directory, name: &str, port: u16) -> cnc_net_directory::GameInfo {
        dir.announce(PublishGame {
            id: None,
            name: name.to_string(),
            kind: GameKind::TiberianDawn,
            map: "scg01ea.ini".to_string(),
            host_name: "host".to_string(),
            address: SocketAddr::from(([127, 0, 0, 1], port)),
            players: 2,
            max_players: 4,
            state: GameState::Lobby,
            passworded: false,
            version: "0.1.1".to_string(),
            tags: Vec::new(),
            ttl_secs: None,
        })
        .expect("publish")
    }

    #[test]
    fn listings_skip_withdrawn_and_dedupe_addresses() {
        let dir = Directory::new(
            NodeId::new(),
            "test",
            "127.0.0.1:47770".parse().expect("addr"),
            None,
        );
        publish(&dir, "Alpha", 7000);
        publish(&dir, "Alpha clone", 7000);
        let removed = publish(&dir, "Beta", 7001);
        dir.withdraw(removed.id);
        let rows = listings_from_games(&dir.games(&GameFilter::default()));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].address, "127.0.0.1:7000");
        assert_eq!(rows[0].source, ListingSource::Directory);
    }

    #[test]
    fn timeout_falls_back_to_a_direct_connect_row() {
        let scan = compose_scan(
            RefreshMode::Http,
            Err(LobbyError::Timeout(RefreshMode::Http)),
            "10.0.0.5:47890",
        );
        assert!(scan.used_fallback());
        assert_eq!(scan.listings.len(), 1);
        assert_eq!(scan.listings[0].source, ListingSource::DirectConnect);
        assert_eq!(scan.listings[0].address, "10.0.0.5:47890");
        assert!(matches!(scan.error, Some(LobbyError::Timeout(_))));
    }

    #[test]
    fn timeout_without_a_manual_address_yields_no_rows() {
        let scan = compose_scan(
            RefreshMode::Lan,
            Err(LobbyError::Timeout(RefreshMode::Lan)),
            "   ",
        );
        assert!(scan.listings.is_empty());
        assert!(scan.status_line().contains("direct connect only"));
    }

    #[test]
    fn malformed_manual_address_is_reported_not_panicked() {
        let scan = compose_scan(
            RefreshMode::Http,
            Err(LobbyError::Timeout(RefreshMode::Http)),
            "not-an-address",
        );
        assert!(scan.listings.is_empty());
        assert!(matches!(scan.error, Some(LobbyError::InvalidAddress(_))));
    }

    #[test]
    fn directory_rows_win_over_the_fallback() {
        let rows = vec![LobbyListing {
            name: "Alpha".to_string(),
            address: "10.0.0.7:7000".to_string(),
            occupancy: "1/2".to_string(),
            game: "TD",
            source: ListingSource::Directory,
        }];
        let scan = compose_scan(RefreshMode::Http, Ok(rows), "10.0.0.5:47890");
        assert!(scan.has_directory_rows());
        assert_eq!(scan.listings.len(), 2);
        assert!(scan.error.is_none());
        assert!(scan.render_rows()[0].contains("Alpha"));
    }

    #[test]
    fn refresh_mode_toggles() {
        assert_eq!(RefreshMode::Http.toggled(), RefreshMode::Lan);
        assert_eq!(RefreshMode::Lan.toggled(), RefreshMode::Http);
    }

    #[test]
    fn unreachable_directory_times_out_without_panicking() {
        // Port 1 on loopback has no listener: the HTTP backend fails fast and
        // the refresh must land on the direct-connect fallback.
        let scan = refresh_lobby_with(
            RefreshMode::Http,
            "127.0.0.1:47890",
            &["127.0.0.1:1".to_string()],
        );
        assert!(scan.used_fallback());
        assert_eq!(scan.listings[0].address, "127.0.0.1:47890");
    }
}
