//! # cnc-net-directory
//!
//! A standalone, distributed game directory for Command & Conquer
//! Remastered-style lobby browsing.
//!
//! Each process is a node in a self-healing peer mesh. Game hosts publish a
//! lobby announcement to any node over HTTP; that node gossips the record to
//! every peer it knows, and peers gossip onward. Every node therefore holds a
//! full, eventually-consistent view of every game it has heard about, so no
//! single server is a single point of failure:
//!
//! * Lobby announcements are last-writer-wins records with tombstones and TTL
//!   expiry (see [registry]).
//! * Peers are discovered from static bootstrap addresses, DNS seeds, LAN
//!   multicast, and a BitTorrent fallback (see [discovery], [torrent]).
//! * Health tracking marks silent peers suspect, then dead, and the mesh routes
//!   around them (see [peer]).
//! * Browsers and hosts talk to any node over a small JSON HTTP API (see [api]).
//! * Each node also runs ranked looking-for-game ladders with per-title MMR,
//!   leagues, and a widening queue; human and AI-only pools are separate and
//!   pairings are announced as normal lobbies (see [matchmaking]).
//!
//! The wire protocol is a versioned, bincode-encoded UDP datagram protocol (see
//! [protocol]); it is deliberately lossy and idempotent so the mesh keeps
//! converging across restarts and partitions.
//!
//! ## Quick start
//!
//! ```no_run
//! use cnc_net_directory::node::{NodeHandle, NodeSettings};
//! # async fn demo() -> anyhow::Result<()> {
//! let node = NodeHandle::bind(NodeSettings::default()).await?;
//! println!("gossip on {}", node.gossip_addr());
//! node.shutdown().await;
//! # Ok(())
//! # }
//! ```

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod api;
pub mod bencode;
pub mod client;
pub mod discovery;
pub mod error;
pub mod matchmaking;
pub mod model;
pub mod node;
pub mod peer;
pub mod protocol;
pub mod registry;
pub mod torrent;

pub use error::{DirectoryError, Result};
pub use matchmaking::{
    JoinQueue, LeaderboardRow, League, LfgError, LfgService, LfgStatus, MatchId, MatchState,
    PlayerId, PlayerProfile, RankedMatch, RatingChange,
};
pub use model::{
    now_ms, GameFilter, GameId, GameInfo, GameKind, GameState, NodeId, NodeInfo, PublishGame,
};
pub use node::{NodeHandle, NodeSettings};
pub use registry::{Directory, MergeStats};
