//! Gossip wire protocol.
//!
//! Every datagram is `MAGIC (4) || VERSION (1) || TAG (1) || bincode(payload)`.
//! The transport is deliberately stateless and idempotent: a lost packet only
//! delays convergence, a duplicated packet is merged twice and ignored the
//! second time, and a reordered packet loses the last-writer-wins comparison.

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::error::ProtocolError;
use crate::model::{GameInfo, NodeId, NodeInfo};

/// Four-byte datagram prefix.
pub const MAGIC: [u8; 4] = *b"CNCD";
/// Current wire version. Bump on breaking layout changes.
pub const VERSION: u8 = 1;
/// Largest datagram accepted from the network.
pub const MAX_DATAGRAM: usize = 64 * 1024;
/// Bytes available for the payload after the fixed header.
pub const HEADER_LEN: usize = 6;

/// Initial handshake sent by the dialling node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hello {
    /// Sender identity and endpoints.
    pub node: NodeInfo,
    /// Other endpoints the sender knows about.
    #[serde(default)]
    pub peers: Vec<SocketAddr>,
    /// Registry fingerprint, used to decide whether a sync is needed.
    pub digest: u64,
}

/// Handshake reply, also carrying the responder's peer list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelloAck {
    /// Responder identity and endpoints.
    pub node: NodeInfo,
    /// Other endpoints the responder knows about.
    #[serde(default)]
    pub peers: Vec<SocketAddr>,
    /// Registry fingerprint of the responder.
    pub digest: u64,
}

/// One gossip datagram.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Message {
    /// Introduce ourselves.
    Hello(Hello),
    /// Acknowledge an introduction.
    HelloAck(HelloAck),
    /// Liveness probe.
    Ping {
        /// Echoed back in the pong.
        nonce: u64,
    },
    /// Liveness reply.
    Pong {
        /// Nonce from the ping.
        nonce: u64,
    },
    /// Peer addresses to spread through the mesh.
    Peers(Vec<SocketAddr>),
    /// A batch of records, merged one by one.
    Games(Vec<GameInfo>),
    /// Ask the peer for records newer than a wall-clock instant.
    DeltaRequest {
        /// Unix milliseconds; `0` means "send everything you have".
        since_ms: i64,
    },
    /// Graceful departure; peers should probe us soon.
    Bye {
        /// Node that is leaving.
        node: NodeId,
    },
}

impl Message {
    /// Short tag used in logs.
    #[must_use]
    pub const fn tag(&self) -> &'static str {
        match self {
            Self::Hello(_) => "hello",
            Self::HelloAck(_) => "hello_ack",
            Self::Ping { .. } => "ping",
            Self::Pong { .. } => "pong",
            Self::Peers(_) => "peers",
            Self::Games(_) => "games",
            Self::DeltaRequest { .. } => "delta_request",
            Self::Bye { .. } => "bye",
        }
    }
}

const TAG_HELLO: u8 = 1;
const TAG_HELLO_ACK: u8 = 2;
const TAG_PING: u8 = 3;
const TAG_PONG: u8 = 4;
const TAG_PEERS: u8 = 5;
const TAG_GAMES: u8 = 6;
const TAG_DELTA_REQUEST: u8 = 7;
const TAG_BYE: u8 = 8;

fn tag_of(message: &Message) -> u8 {
    match message {
        Message::Hello(_) => TAG_HELLO,
        Message::HelloAck(_) => TAG_HELLO_ACK,
        Message::Ping { .. } => TAG_PING,
        Message::Pong { .. } => TAG_PONG,
        Message::Peers(_) => TAG_PEERS,
        Message::Games(_) => TAG_GAMES,
        Message::DeltaRequest { .. } => TAG_DELTA_REQUEST,
        Message::Bye { .. } => TAG_BYE,
    }
}

/// Encode one message into a datagram.
///
/// # Errors
///
/// Returns [ProtocolError::Payload] if bincode cannot represent the message.
pub fn encode(message: &Message) -> Result<Vec<u8>, ProtocolError> {
    let payload = bincode::serialize(message).map_err(|e| ProtocolError::Payload(e.to_string()))?;
    let mut out = Vec::with_capacity(HEADER_LEN + payload.len());
    out.extend_from_slice(&MAGIC);
    out.push(VERSION);
    out.push(tag_of(message));
    out.extend_from_slice(&payload);
    Ok(out)
}

/// Decode one datagram.
///
/// # Errors
///
/// Rejects foreign, truncated, oversized, or version-mismatched datagrams.
pub fn decode(datagram: &[u8]) -> Result<Message, ProtocolError> {
    if datagram.len() > MAX_DATAGRAM {
        return Err(ProtocolError::TooLarge {
            limit: MAX_DATAGRAM,
            actual: datagram.len(),
        });
    }
    if datagram.len() < HEADER_LEN {
        return Err(ProtocolError::TooShort(datagram.len()));
    }
    if datagram[..4] != MAGIC {
        return Err(ProtocolError::BadMagic);
    }
    if datagram[4] != VERSION {
        return Err(ProtocolError::Version(datagram[4]));
    }
    let tag = datagram[5];
    let payload = &datagram[HEADER_LEN..];
    // The payload is the bincode encoding of the whole enum, so decoding is
    // uniform; the leading tag lets peers discard unsupported messages before
    // attempting a decode.
    let message: Message =
        bincode::deserialize(payload).map_err(|e| ProtocolError::Payload(e.to_string()))?;
    if tag_of(&message) != tag {
        return Err(ProtocolError::UnknownTag(tag));
    }
    Ok(message)
}

/// Split records into datagrams that stay under `max_bytes`.
///
/// A single record is never dropped: if it alone exceeds the budget it is sent
/// alone and the receiver will accept it up to [MAX_DATAGRAM].
#[must_use]
pub fn chunk_games(games: &[GameInfo], max_bytes: usize) -> Vec<Vec<GameInfo>> {
    let budget = max_bytes.clamp(256, MAX_DATAGRAM);
    let mut chunks: Vec<Vec<GameInfo>> = Vec::new();
    let mut current: Vec<GameInfo> = Vec::new();
    let mut current_len = HEADER_LEN;
    for game in games {
        let record_len = bincode::serialize(game).map(|b| b.len()).unwrap_or(budget);
        if !current.is_empty() && current_len + record_len > budget {
            chunks.push(std::mem::take(&mut current));
            current_len = HEADER_LEN;
        }
        current_len += record_len;
        current.push(game.clone());
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{GameId, GameKind, GameState, NodeId};

    fn node_info() -> NodeInfo {
        NodeInfo {
            id: NodeId::new(),
            name: "node".to_owned(),
            gossip_addr: "127.0.0.1:4000".parse().expect("addr"),
            http_addr: None,
            version: "0.1.0".to_owned(),
            started_at_ms: 1,
            games: 0,
        }
    }

    fn game(i: u8) -> GameInfo {
        GameInfo {
            id: GameId::new(),
            name: format!("lobby {i}"),
            kind: GameKind::TiberianDawn,
            map: "SCG01EA".to_owned(),
            host_name: "Kane".to_owned(),
            address: "203.0.113.7:47771".parse().expect("addr"),
            players: 1,
            max_players: 4,
            state: GameState::Lobby,
            passworded: false,
            version: "1.0".to_owned(),
            tags: vec![],
            ttl_secs: 60,
            origin: NodeId::new(),
            revision: u64::from(i),
            updated_at_ms: 100,
            deleted: false,
        }
    }

    fn roundtrip(message: &Message) {
        let bytes = encode(message).expect("encode");
        let decoded = decode(&bytes).expect("decode");
        assert_eq!(&decoded, message);
    }

    #[test]
    fn every_message_roundtrips() {
        let info = node_info();
        roundtrip(&Message::Hello(Hello {
            node: info.clone(),
            peers: vec!["127.0.0.1:1".parse().expect("addr")],
            digest: 42,
        }));
        roundtrip(&Message::HelloAck(HelloAck {
            node: info,
            peers: vec![],
            digest: 7,
        }));
        roundtrip(&Message::Ping { nonce: 9 });
        roundtrip(&Message::Pong { nonce: 9 });
        roundtrip(&Message::Peers(vec!["127.0.0.1:2".parse().expect("addr")]));
        roundtrip(&Message::Games(vec![game(1), game(2)]));
        roundtrip(&Message::DeltaRequest { since_ms: 123 });
        roundtrip(&Message::Bye {
            node: NodeId::new(),
        });
    }

    #[test]
    fn foreign_datagram_is_rejected() {
        assert_eq!(decode(b"GET / HTTP/1.1\r\n"), Err(ProtocolError::BadMagic));
    }

    #[test]
    fn truncated_datagram_is_rejected() {
        assert_eq!(decode(b"CNC"), Err(ProtocolError::TooShort(3)));
    }

    #[test]
    fn wrong_version_is_rejected() {
        let mut bytes = encode(&Message::Ping { nonce: 1 }).expect("encode");
        bytes[4] = 99;
        assert_eq!(decode(&bytes), Err(ProtocolError::Version(99)));
    }

    #[test]
    fn unknown_tag_is_rejected() {
        let mut bytes = encode(&Message::Ping { nonce: 1 }).expect("encode");
        bytes[5] = 200;
        assert_eq!(decode(&bytes), Err(ProtocolError::UnknownTag(200)));
    }

    #[test]
    fn chunking_respects_budget_and_keeps_every_record() {
        let games: Vec<GameInfo> = (0..40).map(game).collect();
        let chunks = chunk_games(&games, 512);
        assert!(chunks.len() > 1);
        let total: usize = chunks.iter().map(Vec::len).sum();
        assert_eq!(total, games.len());
        for chunk in &chunks {
            let bytes = encode(&Message::Games(chunk.clone())).expect("encode");
            assert!(bytes.len() <= 1024, "chunk was {} bytes", bytes.len());
        }
    }

    #[test]
    fn empty_game_batch_roundtrips() {
        roundtrip(&Message::Games(Vec::new()));
    }
}
