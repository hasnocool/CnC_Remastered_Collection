//! `cnc-directory` command-line entry point.
//!
//! Subcommands:
//!
//! * `run` — host a directory node (gossip + HTTP API + discovery).
//! * `browse` — list live lobbies from one or more nodes, with failover.
//! * `publish` — announce a lobby and keep its TTL fresh.
//! * `torrent` — publish or resolve the BitTorrent fallback node list.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};
use tracing::{debug, warn};

use cnc_net_directory::client::{to_pretty, DirectoryClient};
use cnc_net_directory::discovery::{DiscoveryConfig, LanConfig, DEFAULT_MULTICAST_GROUP};
use cnc_net_directory::matchmaking::{AgentCard, Controller, JoinQueue, MatchId, PlayerId};
use cnc_net_directory::model::{GameFilter, GameId, GameInfo, GameKind, GameState, PublishGame};
use cnc_net_directory::node::{run_forever, NodeSettings};
use cnc_net_directory::torrent::{
    self, build_bundle, seed_with_qbittorrent, write_bundle, NodeList, QBittorrentConfig,
    TorrentConfig, TorrentResolver, TorrentSource,
};

#[derive(Parser)]
#[command(
    name = "cnc-directory",
    version,
    about = "Distributed lobby directory for Command & Conquer Remastered",
    long_about = "A self-healing peer mesh that announces and browses lobby games. \n\
Every node serves the full directory over HTTP and gossips new games to peers, \n\
so any surviving node can replace one that goes down."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run a directory node.
    Run(RunArgs),
    /// Browse live lobbies.
    Browse(BrowseArgs),
    /// Publish a lobby and keep it fresh.
    Publish(PublishArgs),
    /// Publish or resolve the torrent fallback.
    Torrent {
        #[command(subcommand)]
        command: TorrentCommand,
    },
    /// Ranked looking-for-game ladder.
    Lfg {
        #[command(subcommand)]
        command: LfgCommand,
    },
}

#[derive(Args)]
struct RunArgs {
    /// TOML configuration file.
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,
    /// Node display name.
    #[arg(long)]
    name: Option<String>,
    /// UDP gossip bind address.
    #[arg(long, value_name = "ADDR")]
    gossip_bind: Option<String>,
    /// Address advertised to peers instead of the bind address.
    #[arg(long, value_name = "ADDR")]
    advertise: Option<String>,
    /// HTTP API bind address.
    #[arg(long, value_name = "ADDR")]
    http_bind: Option<String>,
    /// Disable the HTTP API.
    #[arg(long)]
    no_http: bool,
    /// Bootstrap peers (repeat or comma-separate).
    #[arg(long, value_delimiter = ',', value_name = "ADDR")]
    bootstrap: Vec<String>,
    /// DNS seed hostnames.
    #[arg(long, value_delimiter = ',', value_name = "HOST[:PORT]")]
    seed: Vec<String>,
    /// Disable LAN multicast discovery.
    #[arg(long)]
    no_lan: bool,
    /// LAN multicast group.
    #[arg(long, value_name = "ADDR")]
    multicast_group: Option<String>,
    /// Torrent fallback source: magnet, URL, or .torrent path.
    #[arg(long, value_name = "SPEC")]
    torrent: Option<String>,
    /// qBittorrent Web API base URL used by the fallback.
    #[arg(long, value_name = "URL")]
    qbittorrent: Option<String>,
    /// qBittorrent username.
    #[arg(long, value_name = "USER")]
    qbittorrent_user: Option<String>,
    /// qBittorrent password.
    #[arg(long, value_name = "PASS")]
    qbittorrent_pass: Option<String>,
    /// Node-list filename inside the torrent.
    #[arg(long, value_name = "NAME")]
    nodes_filename: Option<String>,
    /// Peer count below which the torrent fallback is consulted.
    #[arg(long)]
    min_peers: Option<usize>,
    /// Gossip push interval, seconds.
    #[arg(long)]
    gossip_secs: Option<u64>,
    /// Anti-entropy pull interval, seconds.
    #[arg(long)]
    sync_secs: Option<u64>,
    /// Maintenance sweep interval, seconds.
    #[arg(long)]
    maintenance_secs: Option<u64>,
    /// Peer silence timeout, seconds.
    #[arg(long)]
    peer_timeout_secs: Option<u64>,
    /// Torrent fallback retry interval, seconds.
    #[arg(long)]
    torrent_secs: Option<u64>,
    /// JSON file used to persist ranked ladder state.
    #[arg(long, value_name = "PATH")]
    lfg_state: Option<PathBuf>,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
struct FileConfig {
    name: Option<String>,
    gossip_bind: Option<String>,
    advertise: Option<String>,
    http_bind: Option<String>,
    lan: Option<bool>,
    multicast_group: Option<String>,
    bootstrap: Vec<String>,
    seeds: Vec<String>,
    torrent: Option<String>,
    nodes_filename: Option<String>,
    min_alive_peers: Option<usize>,
    gossip_secs: Option<u64>,
    sync_secs: Option<u64>,
    maintenance_secs: Option<u64>,
    peer_timeout_secs: Option<u64>,
    torrent_secs: Option<u64>,
    lfg_state: Option<PathBuf>,
    qbittorrent: Option<FileQbittorrent>,
}

#[derive(Debug, serde::Deserialize)]
struct FileQbittorrent {
    url: String,
    #[serde(default = "default_admin")]
    username: String,
    #[serde(default = "default_admin")]
    password: String,
}

fn default_admin() -> String {
    "admin".to_owned()
}

fn parse_addr(text: &str) -> Result<SocketAddr> {
    text.trim()
        .parse()
        .with_context(|| format!("invalid socket address {text:?}"))
}

fn parse_kind(text: &str) -> Result<GameKind> {
    text.parse::<GameKind>().map_err(|e| anyhow!(e))
}

fn parse_state(text: &str) -> Result<GameState> {
    text.parse::<GameState>().map_err(|e| anyhow!(e))
}

fn default_name() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .ok()
        .filter(|name| !name.trim().is_empty())
        .map_or_else(
            || "cnc-directory".to_owned(),
            |name| format!("cnc-directory@{name}"),
        )
}

fn load_file_config(path: Option<&PathBuf>) -> Result<FileConfig> {
    let Some(path) = path else {
        return Ok(FileConfig::default());
    };
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading config {}", path.display()))?;
    toml::from_str(&text).with_context(|| format!("parsing config {}", path.display()))
}

fn build_settings(args: RunArgs) -> Result<NodeSettings> {
    let file = load_file_config(args.config.as_ref())?;

    let name = args.name.or(file.name).unwrap_or_else(default_name);
    let gossip_bind = parse_addr(
        args.gossip_bind
            .or(file.gossip_bind)
            .as_deref()
            .unwrap_or("0.0.0.0:47770"),
    )?;
    let advertise = args
        .advertise
        .or(file.advertise)
        .map(|text| parse_addr(&text))
        .transpose()?;
    let http_bind = if args.no_http {
        None
    } else {
        Some(parse_addr(
            args.http_bind
                .or(file.http_bind)
                .as_deref()
                .unwrap_or("127.0.0.1:8088"),
        )?)
    };

    let lan_enabled = !args.no_lan && file.lan.unwrap_or(true);
    let group = parse_addr(
        args.multicast_group
            .or(file.multicast_group)
            .as_deref()
            .unwrap_or(DEFAULT_MULTICAST_GROUP),
    )?;
    let bootstrap = if args.bootstrap.is_empty() {
        file.bootstrap.clone()
    } else {
        args.bootstrap.clone()
    };
    let seeds = if args.seed.is_empty() {
        file.seeds.clone()
    } else {
        args.seed.clone()
    };

    let file_qb = file.qbittorrent.as_ref();
    let qb_url = args
        .qbittorrent
        .clone()
        .or_else(|| file_qb.map(|qb| qb.url.clone()));
    let qb_user = args
        .qbittorrent_user
        .clone()
        .or_else(|| file_qb.map(|qb| qb.username.clone()))
        .unwrap_or_else(default_admin);
    let qb_pass = args
        .qbittorrent_pass
        .clone()
        .or_else(|| file_qb.map(|qb| qb.password.clone()))
        .unwrap_or_else(default_admin);

    let torrent_spec = args.torrent.clone().or_else(|| file.torrent.clone());
    let torrent = match torrent_spec {
        Some(spec) => {
            let source: TorrentSource = spec.parse().map_err(|e: String| anyhow!(e))?;
            let filename = args
                .nodes_filename
                .clone()
                .or_else(|| file.nodes_filename.clone())
                .unwrap_or_else(|| torrent::NODES_FILENAME.to_owned());
            let qbittorrent = qb_url.map(|base_url| QBittorrentConfig {
                base_url,
                username: qb_user,
                password: qb_pass,
            });
            Some(TorrentConfig {
                source,
                nodes_filename: filename,
                qbittorrent,
                timeout: Duration::from_secs(30),
                poll_interval: Duration::from_millis(500),
            })
        }
        None => None,
    };

    Ok(NodeSettings {
        name,
        gossip_bind,
        advertise,
        http_bind,
        discovery: DiscoveryConfig {
            static_peers: bootstrap,
            dns_seeds: seeds,
            lan: lan_enabled.then(|| LanConfig {
                group,
                ..LanConfig::default()
            }),
        },
        torrent,
        gossip_interval: Duration::from_secs(args.gossip_secs.or(file.gossip_secs).unwrap_or(3)),
        sync_interval: Duration::from_secs(args.sync_secs.or(file.sync_secs).unwrap_or(15)),
        maintenance_interval: Duration::from_secs(
            args.maintenance_secs.or(file.maintenance_secs).unwrap_or(5),
        ),
        peer_timeout: Duration::from_secs(
            args.peer_timeout_secs
                .or(file.peer_timeout_secs)
                .unwrap_or(30),
        ),
        peer_retry_interval: Duration::from_secs(5),
        torrent_interval: Duration::from_secs(
            args.torrent_secs.or(file.torrent_secs).unwrap_or(120),
        ),
        min_alive_peers: args.min_peers.or(file.min_alive_peers).unwrap_or(2),
        lfg_state: args.lfg_state.or(file.lfg_state),
    })
}

#[derive(Args)]
struct BrowseArgs {
    /// Directory nodes to try in order.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "http://127.0.0.1:8088",
        value_name = "URL"
    )]
    node: Vec<String>,
    /// Filter by title: td or ra.
    #[arg(long)]
    kind: Option<String>,
    /// Filter by state: lobby, ready, in_progress.
    #[arg(long)]
    state: Option<String>,
    /// Free-text search.
    #[arg(long)]
    q: Option<String>,
    /// Maximum results.
    #[arg(long)]
    limit: Option<usize>,
    /// Hide password-protected lobbies.
    #[arg(long)]
    open_only: bool,
    /// Only lobbies with a free player slot.
    #[arg(long)]
    with_slots: bool,
    /// Print JSON instead of a table.
    #[arg(long)]
    json: bool,
}

fn clip(text: &str, width: usize) -> String {
    let count = text.chars().count();
    if count > width {
        let mut out: String = text.chars().take(width.saturating_sub(1)).collect();
        out.push('…');
        out
    } else {
        let mut out = text.to_owned();
        out.push_str(&" ".repeat(width - count));
        out
    }
}

fn print_games(games: &[GameInfo]) {
    if games.is_empty() {
        println!("no lobbies match");
        return;
    }
    println!(
        "{} {} {} {} {} {} ADDRESS",
        clip("NAME", 32),
        clip("KIND", 4),
        clip("MAP", 16),
        clip("HOST", 14),
        clip("PLR", 5),
        clip("STATE", 12)
    );
    for game in games {
        println!(
            "{} {} {} {} {} {} {}",
            clip(&game.name, 32),
            clip(game.kind.as_str(), 4),
            clip(&game.map, 16),
            clip(&game.host_name, 14),
            clip(&game.occupancy(), 5),
            clip(game.state.as_str(), 12),
            game.address
        );
    }
}

#[derive(Args)]
struct PublishArgs {
    /// Directory nodes to try in order.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "http://127.0.0.1:8088",
        value_name = "URL"
    )]
    node: Vec<String>,
    /// Lobby display name.
    #[arg(long)]
    name: String,
    /// Title: td or ra.
    #[arg(long)]
    kind: String,
    /// Map name.
    #[arg(long)]
    map: String,
    /// Host nickname.
    #[arg(long, default_value = "")]
    host_name: String,
    /// Join address advertised to players.
    #[arg(long, value_name = "ADDR")]
    address: String,
    /// Players currently in the lobby.
    #[arg(long, default_value_t = 1)]
    players: u8,
    /// Lobby capacity (1-8).
    #[arg(long, default_value_t = 4)]
    max_players: u8,
    /// State: lobby, ready, in_progress.
    #[arg(long, default_value = "lobby")]
    state: String,
    /// Mark the lobby as password protected.
    #[arg(long)]
    passworded: bool,
    /// Game build string.
    #[arg(long, default_value = "")]
    version: String,
    /// Tags (repeat or comma-separate).
    #[arg(long, value_delimiter = ',')]
    tag: Vec<String>,
    /// TTL in seconds (10-3600).
    #[arg(long)]
    ttl: Option<u32>,
    /// Reuse an existing lobby id.
    #[arg(long)]
    id: Option<String>,
    /// Heartbeat interval in seconds; 0 publishes once and exits.
    #[arg(long, default_value_t = 30)]
    heartbeat_secs: u64,
    /// Print the created record as JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Subcommand)]
enum TorrentCommand {
    /// Write nodes.json + .torrent and print a magnet link.
    Publish(TorrentPublishArgs),
    /// Resolve a node list from the fallback.
    Fetch(TorrentFetchArgs),
}

#[derive(Args)]
struct TorrentPublishArgs {
    /// Gossip addresses to include (repeat or comma-separate).
    #[arg(long, value_delimiter = ',', value_name = "ADDR")]
    nodes: Vec<String>,
    /// Also read peers from this running node.
    #[arg(long, value_name = "URL")]
    from: Option<String>,
    /// Output directory.
    #[arg(long, default_value = ".")]
    out: PathBuf,
    /// Payload filename inside the torrent.
    #[arg(long, default_value = torrent::NODES_FILENAME)]
    filename: String,
    /// Tracker URLs.
    #[arg(long, value_delimiter = ',')]
    tracker: Vec<String>,
    /// HTTP web seeds.
    #[arg(long, value_delimiter = ',')]
    webseed: Vec<String>,
    /// Piece length in bytes.
    #[arg(long, default_value_t = 262_144)]
    piece_length: usize,
    /// Seed the result through qBittorrent at this URL.
    #[arg(long, value_name = "URL")]
    qbittorrent: Option<String>,
    /// qBittorrent username.
    #[arg(long)]
    qbittorrent_user: Option<String>,
    /// qBittorrent password.
    #[arg(long)]
    qbittorrent_pass: Option<String>,
}

#[derive(Args)]
struct TorrentFetchArgs {
    /// Magnet URI, HTTP JSON URL, or local .torrent path.
    #[arg(long, value_name = "SPEC")]
    source: String,
    /// Use this qBittorrent Web API to download the torrent.
    #[arg(long, value_name = "URL")]
    qbittorrent: Option<String>,
    /// qBittorrent username.
    #[arg(long)]
    qbittorrent_user: Option<String>,
    /// qBittorrent password.
    #[arg(long)]
    qbittorrent_pass: Option<String>,
    /// Payload filename inside the torrent.
    #[arg(long, default_value = torrent::NODES_FILENAME)]
    filename: String,
    /// Overall wait budget, seconds.
    #[arg(long, default_value_t = 30)]
    timeout_secs: u64,
    /// Print only addresses, one per line.
    #[arg(long)]
    plain: bool,
}

#[derive(Subcommand)]
enum LfgCommand {
    /// Create a ranked player profile.
    Register(LfgRegisterArgs),
    /// Join the ranked queue.
    Queue(LfgQueueArgs),
    /// Leave the ranked queue.
    Leave(LfgLeaveArgs),
    /// Show ladder counters, optionally with one profile.
    Status(LfgStatusArgs),
    /// Show the leaderboard.
    Leaderboard(LfgLeaderboardArgs),
    /// Report a match result.
    Report(LfgReportArgs),
}

#[derive(Args)]
struct LfgRegisterArgs {
    /// Directory nodes to try in order.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "http://127.0.0.1:8088",
        value_name = "URL"
    )]
    node: Vec<String>,
    /// Display name.
    #[arg(long)]
    name: String,
    /// human (default) or ai.
    #[arg(long, default_value = "human")]
    controller: String,
    /// AI owner house, for example HouseNod.
    #[arg(long)]
    owner: Option<String>,
    /// AI doctrine from the editor.
    #[arg(long)]
    doctrine: Option<String>,
    /// Hex SHA-1 of the agent script.
    #[arg(long)]
    script_hash: Option<String>,
    /// Human author of the AI.
    #[arg(long)]
    creator: Option<String>,
    /// Scenario the AI was authored in.
    #[arg(long)]
    scenario: Option<String>,
    /// Map-local agent id.
    #[arg(long)]
    agent_id: Option<u32>,
    /// Print JSON instead of a summary.
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct LfgQueueArgs {
    /// Directory nodes to try in order.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "http://127.0.0.1:8088",
        value_name = "URL"
    )]
    node: Vec<String>,
    /// Player identifier returned by register.
    #[arg(long)]
    player: String,
    /// Title to queue for: td or ra.
    #[arg(long)]
    kind: String,
    /// Your game host address, host:port.
    #[arg(long)]
    address: String,
    /// Preferred map.
    #[arg(long)]
    map: Option<String>,
    /// Free-text note.
    #[arg(long)]
    note: Option<String>,
    /// Print JSON instead of a summary.
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct LfgLeaveArgs {
    /// Directory nodes to try in order.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "http://127.0.0.1:8088",
        value_name = "URL"
    )]
    node: Vec<String>,
    /// Player identifier.
    #[arg(long)]
    player: String,
}

#[derive(Args)]
struct LfgStatusArgs {
    /// Directory nodes to try in order.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "http://127.0.0.1:8088",
        value_name = "URL"
    )]
    node: Vec<String>,
    /// Player identifier to include.
    #[arg(long)]
    player: Option<String>,
    /// Print JSON instead of a summary.
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct LfgLeaderboardArgs {
    /// Directory nodes to try in order.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "http://127.0.0.1:8088",
        value_name = "URL"
    )]
    node: Vec<String>,
    /// Filter to one title: td or ra.
    #[arg(long)]
    kind: Option<String>,
    /// human (default) or ai.
    #[arg(long, default_value = "human")]
    controller: String,
    /// Maximum rows.
    #[arg(long, default_value_t = 25)]
    limit: usize,
    /// Print JSON instead of a table.
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct LfgReportArgs {
    /// Directory nodes to try in order.
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "http://127.0.0.1:8088",
        value_name = "URL"
    )]
    node: Vec<String>,
    /// Match identifier.
    #[arg(long = "match")]
    match_id: String,
    /// Reporting player identifier.
    #[arg(long)]
    player: String,
    /// Winner player identifier.
    #[arg(long)]
    winner: String,
    /// Print JSON instead of a summary.
    #[arg(long)]
    json: bool,
}

fn parse_player(text: &str) -> Result<PlayerId> {
    text.trim()
        .parse()
        .map_err(|err| anyhow!("invalid player id {text:?}: {err}"))
}

fn parse_match(text: &str) -> Result<MatchId> {
    text.trim()
        .parse()
        .map_err(|err| anyhow!("invalid match id {text:?}: {err}"))
}

fn qbittorrent_config(
    url: Option<String>,
    user: Option<String>,
    pass: Option<String>,
) -> Option<QBittorrentConfig> {
    url.map(|base_url| QBittorrentConfig {
        base_url,
        username: user.unwrap_or_else(default_admin),
        password: pass.unwrap_or_else(default_admin),
    })
}

async fn run_torrent_publish(args: TorrentPublishArgs) -> Result<()> {
    let mut nodes: Vec<SocketAddr> = Vec::new();
    for entry in &args.nodes {
        nodes.push(parse_addr(entry)?);
    }
    if let Some(from) = &args.from {
        let client = DirectoryClient::new(from.clone())?;
        let value = client.nodes().await?;
        if let Some(addr) = value
            .get("node")
            .and_then(|n| n.get("gossip_addr"))
            .and_then(|a| a.as_str())
        {
            if let Ok(addr) = addr.parse() {
                nodes.push(addr);
            }
        }
        if let Some(peers) = value.get("peers").and_then(|p| p.as_array()) {
            for peer in peers {
                if let Some(addr) = peer.get("addr").and_then(|a| a.as_str()) {
                    if let Ok(addr) = addr.parse() {
                        nodes.push(addr);
                    }
                }
            }
        }
    }
    nodes.sort();
    nodes.dedup();
    if nodes.is_empty() {
        return Err(anyhow!(
            "no nodes to publish; pass --nodes or --from http://node:8088"
        ));
    }

    let list = NodeList::new(nodes, Vec::new(), None);
    let bundle = build_bundle(
        &list,
        &args.filename,
        &args.tracker,
        &args.webseed,
        args.piece_length,
    )?;
    let torrent_path = write_bundle(&args.out, &bundle)?;
    println!("wrote {}", torrent_path.display());
    println!("info-hash {}", bundle.info_hash);
    println!("magnet {}", bundle.magnet);

    if let Some(qb) = qbittorrent_config(
        args.qbittorrent,
        args.qbittorrent_user,
        args.qbittorrent_pass,
    ) {
        let http = reqwest::Client::new();
        seed_with_qbittorrent(&qb, &bundle, http).await?;
        println!("added to qBittorrent for seeding");
    }
    Ok(())
}

async fn run_torrent_fetch(args: TorrentFetchArgs) -> Result<()> {
    let source: TorrentSource = args.source.parse().map_err(|e: String| anyhow!(e))?;
    let qbittorrent = qbittorrent_config(
        args.qbittorrent,
        args.qbittorrent_user,
        args.qbittorrent_pass,
    );
    let resolver = TorrentResolver::new(TorrentConfig {
        source,
        nodes_filename: args.filename,
        qbittorrent,
        timeout: Duration::from_secs(args.timeout_secs),
        poll_interval: Duration::from_millis(500),
    })?;
    let list = resolver.resolve().await?;
    if args.plain {
        for addr in &list.nodes {
            println!("{addr}");
        }
    } else {
        println!("{}", to_pretty(&list)?);
    }
    Ok(())
}

async fn run_node(args: RunArgs) -> Result<()> {
    let settings = build_settings(args)?;
    println!(
        "starting {} on {} (http {})",
        settings.name,
        settings.gossip_bind,
        settings
            .http_bind
            .map_or_else(|| "disabled".to_owned(), |addr| addr.to_string())
    );
    run_forever(settings).await?;
    Ok(())
}

async fn run_browse(args: BrowseArgs) -> Result<()> {
    let client = DirectoryClient::first_available(&args.node).await?;
    let filter = GameFilter {
        state: args.state.as_deref().map(parse_state).transpose()?,
        kind: args.kind.as_deref().map(parse_kind).transpose()?,
        query: args.q,
        min_players: None,
        max_players: None,
        open_only: args.open_only,
        with_slots: args.with_slots,
        limit: args.limit,
    };
    let games = client.games(&filter).await?;
    if args.json {
        println!("{}", to_pretty(&games)?);
    } else {
        eprintln!("node {} · {} lobbies", client.base(), games.len());
        print_games(&games);
    }
    Ok(())
}

async fn run_publish(args: PublishArgs) -> Result<()> {
    let client = DirectoryClient::first_available(&args.node).await?;
    let kind = parse_kind(&args.kind)?;
    let state = parse_state(&args.state)?;
    let address = parse_addr(&args.address)?;
    let id = args
        .id
        .as_deref()
        .map(str::parse::<GameId>)
        .transpose()
        .context("invalid game id")?;
    let publish = PublishGame {
        id,
        name: args.name,
        kind,
        map: args.map,
        host_name: args.host_name,
        address,
        players: args.players,
        max_players: args.max_players,
        state,
        passworded: args.passworded,
        version: args.version,
        tags: args.tag,
        ttl_secs: args.ttl,
    };
    let created = client.publish(&publish).await?;
    if args.json {
        println!("{}", to_pretty(&created)?);
    } else {
        println!(
            "published {} ({}) -> {}",
            created.name, created.id, created.address
        );
    }

    if args.heartbeat_secs == 0 {
        return Ok(());
    }
    println!(
        "refreshing every {}s; Ctrl-C to withdraw",
        args.heartbeat_secs
    );
    let period = Duration::from_secs(args.heartbeat_secs.max(5));
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => break,
            _ = tokio::time::sleep(period) => {
                match client.heartbeat(created.id).await {
                    Ok(_) => debug!(id = %created.id, "heartbeat ok"),
                    Err(err) => {
                        warn!(id = %created.id, error = %err, "heartbeat failed; re-publishing");
                        let mut retry = publish.clone();
                        retry.id = Some(created.id);
                        if let Err(err) = client.publish(&retry).await {
                            warn!(error = %err, "re-publish failed");
                        }
                    }
                }
            }
        }
    }
    if let Err(err) = client.withdraw(created.id).await {
        warn!(error = %err, "withdraw on shutdown failed");
    } else {
        println!("withdrew {}", created.id);
    }
    Ok(())
}

async fn run_lfg_register(args: LfgRegisterArgs) -> Result<()> {
    let client = DirectoryClient::first_available(&args.node).await?;
    let controller = args
        .controller
        .parse::<Controller>()
        .map_err(|err| anyhow!(err))?;
    let has_agent_metadata = args.owner.is_some()
        || args.doctrine.is_some()
        || args.script_hash.is_some()
        || args.creator.is_some()
        || args.scenario.is_some()
        || args.agent_id.is_some();
    if !controller.is_ai() && has_agent_metadata {
        return Err(anyhow!("AI agent flags require --controller ai"));
    }
    let profile = if controller.is_ai() {
        let agent = AgentCard {
            agent_id: args.agent_id,
            owner: args.owner.unwrap_or_default(),
            doctrine: args.doctrine.unwrap_or_default(),
            script_hash: args.script_hash.unwrap_or_default(),
            creator: args.creator,
            scenario: args.scenario,
        };
        client.register_ai(&args.name, &agent).await?
    } else {
        client.register_player(&args.name).await?
    };
    if args.json {
        println!("{}", to_pretty(&profile)?);
    } else {
        println!("{} {} ({})", profile.controller, profile.name, profile.id);
        if let Some(agent) = &profile.agent {
            println!(
                "  agent: {} {} · owner {} · script {}",
                if agent.doctrine.is_empty() {
                    "AI"
                } else {
                    agent.doctrine.as_str()
                },
                agent.scenario.as_deref().unwrap_or("no scenario"),
                agent.owner,
                agent.script_hash
            );
        }
        for rating in &profile.ratings {
            println!(
                "  {}: {} {} · {}W-{}L{}",
                rating.kind,
                rating.rating,
                rating.league(),
                rating.wins,
                rating.losses,
                if rating.provisional() {
                    " · provisional"
                } else {
                    ""
                }
            );
        }
    }
    Ok(())
}

async fn run_lfg_queue(args: LfgQueueArgs) -> Result<()> {
    let client = DirectoryClient::first_available(&args.node).await?;
    let kind = parse_kind(&args.kind)?;
    let address = parse_addr(&args.address)?;
    let entry = client
        .join_queue(&JoinQueue {
            player_id: parse_player(&args.player)?,
            kind,
            address,
            map: args.map,
            note: args.note,
        })
        .await?;
    if args.json {
        println!("{}", to_pretty(&entry)?);
    } else {
        println!(
            "queued {} for {} at {}",
            entry.name, entry.kind, entry.address
        );
    }
    Ok(())
}

async fn run_lfg_leave(args: LfgLeaveArgs) -> Result<()> {
    let client = DirectoryClient::first_available(&args.node).await?;
    client.leave_queue(parse_player(&args.player)?).await?;
    println!("left the ranked queue");
    Ok(())
}

async fn run_lfg_status(args: LfgStatusArgs) -> Result<()> {
    let client = DirectoryClient::first_available(&args.node).await?;
    let status = client.lfg_status().await?;
    let profile = match args.player.as_deref() {
        Some(raw) => Some(client.lfg_player(parse_player(raw)?).await?),
        None => None,
    };
    if args.json {
        let body = serde_json::json!({ "status": status, "player": profile });
        println!("{}", to_pretty(&body)?);
    } else {
        println!(
            "human: {} players · {} queued · {} pending · {} settled · top {}",
            status.players,
            status.queued,
            status.pending_matches,
            status.settled_matches,
            status
                .top_rating
                .map_or_else(|| "-".to_owned(), |rating| rating.to_string())
        );
        println!(
            "ai:    {} players · {} queued · {} pending · {} settled · top {}",
            status.ai_players,
            status.ai_queued,
            status.ai_pending_matches,
            status.ai_settled_matches,
            status
                .ai_top_rating
                .map_or_else(|| "-".to_owned(), |rating| rating.to_string())
        );
        if let Some(profile) = profile {
            println!("{} ({})", profile.name, profile.id);
            for rating in &profile.ratings {
                println!(
                    "  {}: {} {} · {}W-{}L",
                    rating.kind,
                    rating.rating,
                    rating.league(),
                    rating.wins,
                    rating.losses
                );
            }
        }
    }
    Ok(())
}

async fn run_lfg_leaderboard(args: LfgLeaderboardArgs) -> Result<()> {
    let client = DirectoryClient::first_available(&args.node).await?;
    let controller = args
        .controller
        .parse::<Controller>()
        .map_err(|err| anyhow!(err))?;
    let kind = args.kind.as_deref().map(parse_kind).transpose()?;
    let rows = client.leaderboard_for(controller, kind, args.limit).await?;
    if args.json {
        println!("{}", to_pretty(&rows)?);
    } else if rows.is_empty() {
        println!("no ranked {controller} competitors yet");
    } else {
        println!(
            "{:>4}  {:<24}  {:<6}  {:>6}  {:<12}  W-L",
            "#", "player", "game", "rating", "league"
        );
        for row in rows {
            let name = if row.controller.is_ai() {
                match &row.agent {
                    Some(agent) if !agent.doctrine.is_empty() => {
                        format!("{} [{}]", row.name, agent.doctrine)
                    }
                    _ => format!("{} [AI]", row.name),
                }
            } else {
                row.name.clone()
            };
            println!(
                "{:>4}  {:<24}  {:<6}  {:>6}  {:<12}  {}-{}{}",
                row.rank,
                clip(&name, 24),
                row.kind.as_str(),
                row.rating,
                row.league,
                row.wins,
                row.losses,
                if row.provisional { " (p)" } else { "" }
            );
        }
    }
    Ok(())
}

async fn run_lfg_report(args: LfgReportArgs) -> Result<()> {
    let client = DirectoryClient::first_available(&args.node).await?;
    let record = client
        .report_match(
            parse_match(&args.match_id)?,
            parse_player(&args.player)?,
            parse_player(&args.winner)?,
        )
        .await?;
    if args.json {
        println!("{}", to_pretty(&record)?);
    } else {
        println!(
            "match {} {} · {} on {}",
            record.id,
            record.state.as_str(),
            record.map,
            record.kind
        );
        for change in &record.rating_changes {
            println!(
                "  {}: {} -> {} ({:+})",
                change.player_id, change.before, change.after, change.delta
            );
        }
    }
    Ok(())
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    match Cli::parse().command {
        Command::Run(args) => run_node(args).await,
        Command::Browse(args) => run_browse(args).await,
        Command::Publish(args) => run_publish(args).await,
        Command::Torrent { command } => match command {
            TorrentCommand::Publish(args) => run_torrent_publish(args).await,
            TorrentCommand::Fetch(args) => run_torrent_fetch(args).await,
        },
        Command::Lfg { command } => match command {
            LfgCommand::Register(args) => run_lfg_register(args).await,
            LfgCommand::Queue(args) => run_lfg_queue(args).await,
            LfgCommand::Leave(args) => run_lfg_leave(args).await,
            LfgCommand::Status(args) => run_lfg_status(args).await,
            LfgCommand::Leaderboard(args) => run_lfg_leaderboard(args).await,
            LfgCommand::Report(args) => run_lfg_report(args).await,
        },
    }
}
