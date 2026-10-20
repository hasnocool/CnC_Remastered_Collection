//! Typed errors for the directory node.

use std::net::AddrParseError;

use thiserror::Error;

/// Errors produced while running or querying a directory node.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DirectoryError {
    /// A lobby announcement failed validation.
    #[error("invalid announcement: {0}")]
    Validation(#[from] ValidationError),

    /// A wire datagram could not be decoded.
    #[error("protocol error: {0}")]
    Protocol(#[from] ProtocolError),

    /// A bootstrap source could not be reached.
    #[error("discovery backend {backend} failed: {source}")]
    Discovery {
        /// Which backend failed (static, dns, multicast, torrent, ...).
        backend: &'static str,
        /// Underlying cause.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Torrent metainfo handling failed.
    #[error("torrent error: {0}")]
    Torrent(#[from] TorrentError),

    /// Filesystem or configuration IO failed.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// A network address could not be parsed.
    #[error("invalid socket address: {0}")]
    Addr(#[from] AddrParseError),

    /// An HTTP request to another node failed.
    #[error("http request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Another node answered with an error status.
    #[error("node returned {status}: {body}")]
    Remote {
        /// HTTP status code.
        status: u16,
        /// Response body.
        body: String,
    },

    /// A JSON document could not be decoded.
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Configuration was structurally invalid.
    #[error("invalid configuration: {0}")]
    Config(String),

    /// The node has already been shut down.
    #[error("node is shut down")]
    Shutdown,
}

/// Details about why a [crate::model::PublishGame] was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum ValidationError {
    /// The game name was empty or too long.
    #[error("game name must be 1..={max} characters")]
    Name {
        /// Maximum accepted length.
        max: usize,
    },
    /// The map name was empty or too long.
    #[error("map name must be 1..={max} characters")]
    Map {
        /// Maximum accepted length.
        max: usize,
    },
    /// The advertised join address is unusable.
    #[error("join address {0} cannot be advertised")]
    Address(std::net::SocketAddr),
    /// Player counts are inconsistent.
    #[error("players={players} must be within 1..={max_players}")]
    Players {
        /// Joined players.
        players: u8,
        /// Configured capacity.
        max_players: u8,
    },
    /// Player capacity is outside the supported range.
    #[error("max_players={value} must be within {min}..={max}")]
    Capacity {
        /// Supplied capacity.
        value: u8,
        /// Minimum supported.
        min: u8,
        /// Maximum supported.
        max: u8,
    },
    /// The announcement TTL is outside the supported range.
    #[error("ttl_secs={value} must be within {min}..={max}")]
    Ttl {
        /// Supplied TTL.
        value: u32,
        /// Minimum supported.
        min: u32,
        /// Maximum supported.
        max: u32,
    },
    /// A version string is too long.
    #[error("version string must be at most {0} characters")]
    Version(usize),
    /// Too many tags, or a tag is too long.
    #[error("at most {max_tags} tags of {max_len} characters are allowed")]
    Tags {
        /// Maximum tags.
        max_tags: usize,
        /// Maximum tag length.
        max_len: usize,
    },
}

/// Errors produced while encoding or decoding gossip datagrams.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum ProtocolError {
    /// The datagram is smaller than the fixed header.
    #[error("datagram too short: {0} bytes")]
    TooShort(usize),
    /// The magic prefix is not ours.
    #[error("bad magic prefix")]
    BadMagic,
    /// The protocol version is unknown.
    #[error("unsupported protocol version {0}")]
    Version(u8),
    /// The message tag is unknown.
    #[error("unknown message tag {0}")]
    UnknownTag(u8),
    /// The payload is too large to accept.
    #[error("datagram exceeds {limit} bytes: {actual}")]
    TooLarge {
        /// Configured limit.
        limit: usize,
        /// Observed size.
        actual: usize,
    },
    /// The bincode payload was malformed.
    #[error("payload decode failed: {0}")]
    Payload(String),
}

/// Torrent-specific failures.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TorrentError {
    /// Bencode data was malformed.
    #[error("bencode error: {0}")]
    Bencode(String),
    /// The torrent payload had no usable web seed or local client.
    #[error("torrent has no usable web seed or local client")]
    NoTransport,
    /// The node list could not be fetched.
    #[error("node list fetch failed: {0}")]
    Fetch(String),
    /// The qBittorrent Web API rejected or failed a request.
    #[error("qBittorrent API error: {0}")]
    QBittorrent(String),
    /// The expected node-list file never appeared.
    #[error("timed out waiting for {0}")]
    Timeout(String),
    /// JSON decoding failed.
    #[error("node list decode failed: {0}")]
    NodeList(String),
    /// An HTTP response was not successful.
    #[error("http {status} from {url}")]
    Http {
        /// Response status.
        status: u16,
        /// Requested URL.
        url: String,
    },
}

impl DirectoryError {
    /// Wrap an underlying error from a named discovery backend.
    pub fn discovery(
        backend: &'static str,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Discovery {
            backend,
            source: Box::new(source),
        }
    }
}

/// Convenience alias used across the crate.
pub type Result<T, E = DirectoryError> = std::result::Result<T, E>;
