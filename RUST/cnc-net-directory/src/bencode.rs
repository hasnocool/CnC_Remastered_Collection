//! Minimal, strict bencode codec plus `.torrent` metainfo parsing.
//!
//! The directory only needs three things from BitTorrent: read a node list
//! published as a torrent payload, build a `.torrent` for the current node
//! list so it can be seeded by any client, and compute the info-hash for the
//! magnet link. That is small enough to own rather than pull in a full
//! BitTorrent stack.

use std::collections::BTreeMap;

use sha1::{Digest, Sha1};

use crate::error::TorrentError;

/// Maximum nesting accepted while decoding.
pub const MAX_DEPTH: usize = 64;

/// A bencoded value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// Integer.
    Int(i64),
    /// Byte string.
    Bytes(Vec<u8>),
    /// Ordered list.
    List(Vec<Value>),
    /// Dictionary with lexicographically sorted keys.
    Dict(BTreeMap<String, Value>),
}

impl Value {
    /// Build a byte-string value from UTF-8 text.
    #[must_use]
    pub fn string(value: &str) -> Self {
        Self::Bytes(value.as_bytes().to_vec())
    }

    /// Build a dictionary from pairs.
    #[must_use]
    pub fn dict(pairs: impl IntoIterator<Item = (impl Into<String>, Value)>) -> Self {
        Self::Dict(pairs.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }

    /// Borrow the bytes of a string value.
    #[must_use]
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(bytes) => Some(bytes),
            _ => None,
        }
    }

    /// Borrow the bytes as UTF-8 text.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        self.as_bytes().and_then(|b| std::str::from_utf8(b).ok())
    }

    /// Read an integer.
    #[must_use]
    pub const fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// Read a list.
    #[must_use]
    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Self::List(items) => Some(items),
            _ => None,
        }
    }

    /// Read a dictionary.
    #[must_use]
    pub fn as_dict(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Self::Dict(map) => Some(map),
            _ => None,
        }
    }

    /// Look a key up in a dictionary.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_dict().and_then(|m| m.get(key))
    }

    /// Encode the value.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        self.encode_into(&mut out);
        out
    }

    fn encode_into(&self, out: &mut Vec<u8>) {
        match self {
            Self::Int(v) => {
                out.push(b'i');
                out.extend_from_slice(v.to_string().as_bytes());
                out.push(b'e');
            }
            Self::Bytes(bytes) => {
                out.extend_from_slice(bytes.len().to_string().as_bytes());
                out.push(b':');
                out.extend_from_slice(bytes);
            }
            Self::List(items) => {
                out.push(b'l');
                for item in items {
                    item.encode_into(out);
                }
                out.push(b'e');
            }
            Self::Dict(map) => {
                out.push(b'd');
                for (key, value) in map {
                    out.extend_from_slice(key.len().to_string().as_bytes());
                    out.push(b':');
                    out.extend_from_slice(key.as_bytes());
                    value.encode_into(out);
                }
                out.push(b'e');
            }
        }
    }

    /// Decode exactly one value, rejecting trailing bytes.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::Bencode] for malformed input.
    pub fn decode(bytes: &[u8]) -> Result<Self, TorrentError> {
        let mut decoder = Decoder {
            data: bytes,
            pos: 0,
            depth: 0,
        };
        let value = decoder.value()?;
        if decoder.pos != bytes.len() {
            return Err(TorrentError::Bencode(format!(
                "trailing {} bytes",
                bytes.len() - decoder.pos
            )));
        }
        Ok(value)
    }
}

struct Decoder<'a> {
    data: &'a [u8],
    pos: usize,
    depth: usize,
}

impl Decoder<'_> {
    fn peek(&self) -> Option<u8> {
        self.data.get(self.pos).copied()
    }

    fn expect(&mut self, byte: u8) -> Result<(), TorrentError> {
        match self.peek() {
            Some(found) if found == byte => {
                self.pos += 1;
                Ok(())
            }
            Some(found) => Err(TorrentError::Bencode(format!(
                "expected {:?} at {}, found {:?}",
                byte as char, self.pos, found as char
            ))),
            None => Err(TorrentError::Bencode("unexpected end of input".to_owned())),
        }
    }

    fn value(&mut self) -> Result<Value, TorrentError> {
        if self.depth >= MAX_DEPTH {
            return Err(TorrentError::Bencode("nesting too deep".to_owned()));
        }
        self.depth += 1;
        let result = match self.peek() {
            Some(b'i') => self.integer(),
            Some(b'l') => self.list(),
            Some(b'd') => self.dict(),
            Some(b'0'..=b'9') => self.byte_string(),
            Some(other) => Err(TorrentError::Bencode(format!(
                "unexpected byte {:?} at {}",
                other as char, self.pos
            ))),
            None => Err(TorrentError::Bencode("unexpected end of input".to_owned())),
        };
        self.depth -= 1;
        result
    }

    fn integer(&mut self) -> Result<Value, TorrentError> {
        self.expect(b'i')?;
        let start = self.pos;
        while matches!(self.peek(), Some(b'0'..=b'9') | Some(b'-')) {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.data[start..self.pos])
            .map_err(|e| TorrentError::Bencode(e.to_string()))?;
        let value: i64 = text
            .parse()
            .map_err(|_| TorrentError::Bencode(format!("bad integer {text:?}")))?;
        self.expect(b'e')?;
        Ok(Value::Int(value))
    }

    fn byte_string(&mut self) -> Result<Value, TorrentError> {
        let start = self.pos;
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.data[start..self.pos])
            .map_err(|e| TorrentError::Bencode(e.to_string()))?;
        let len: usize = text
            .parse()
            .map_err(|_| TorrentError::Bencode(format!("bad string length {text:?}")))?;
        self.expect(b':')?;
        let end = self
            .pos
            .checked_add(len)
            .filter(|end| *end <= self.data.len())
            .ok_or_else(|| TorrentError::Bencode("string runs past input".to_owned()))?;
        let bytes = self.data[self.pos..end].to_vec();
        self.pos = end;
        Ok(Value::Bytes(bytes))
    }

    fn list(&mut self) -> Result<Value, TorrentError> {
        self.expect(b'l')?;
        let mut items = Vec::new();
        while self.peek() != Some(b'e') {
            items.push(self.value()?);
        }
        self.expect(b'e')?;
        Ok(Value::List(items))
    }

    fn dict(&mut self) -> Result<Value, TorrentError> {
        self.expect(b'd')?;
        let mut map = BTreeMap::new();
        while self.peek() != Some(b'e') {
            let key = self.byte_string()?;
            let key = key
                .as_str()
                .ok_or_else(|| TorrentError::Bencode("non-UTF-8 dictionary key".to_owned()))?
                .to_owned();
            let value = self.value()?;
            map.insert(key, value);
        }
        self.expect(b'e')?;
        Ok(Value::Dict(map))
    }
}

/// Parsed magnet link: info-hash, HTTP web seeds, trackers.
pub type MagnetParts = ([u8; 20], Vec<String>, Vec<String>);

/// Parsed `.torrent` metainfo, reduced to the fields the node list needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TorrentMeta {
    /// Raw `info` dictionary.
    pub info: Value,
    /// SHA-1 of the bencoded `info` dictionary.
    pub info_hash: [u8; 20],
    /// Primary tracker URL, when present.
    pub announce: Option<String>,
    /// Tiered tracker list flattened.
    pub announce_list: Vec<String>,
    /// BEP-19 HTTP seeds.
    pub url_list: Vec<String>,
    /// Torrent name, usually the node-list filename.
    pub name: Option<String>,
    /// Single-file length, when present.
    pub length: Option<i64>,
}

impl TorrentMeta {
    /// Parse a `.torrent` file payload.
    ///
    /// # Errors
    ///
    /// Rejects payloads that are not a dictionary with an `info` dictionary.
    pub fn parse(bytes: &[u8]) -> Result<Self, TorrentError> {
        let root = Value::decode(bytes)?;
        let info = root
            .get("info")
            .cloned()
            .ok_or_else(|| TorrentError::Bencode("missing info dictionary".to_owned()))?;
        let info_hash: [u8; 20] = Sha1::digest(info.encode()).into();
        let announce = root
            .get("announce")
            .and_then(Value::as_str)
            .map(str::to_owned);
        let mut announce_list = Vec::new();
        if let Some(tiers) = root.get("announce-list").and_then(Value::as_list) {
            for tier in tiers {
                if let Some(urls) = tier.as_list() {
                    announce_list.extend(urls.iter().filter_map(Value::as_str).map(str::to_owned));
                }
            }
        }
        let url_list = match root.get("url-list") {
            Some(Value::Bytes(_)) => root
                .get("url-list")
                .and_then(Value::as_str)
                .map(|s| vec![s.to_owned()])
                .unwrap_or_default(),
            Some(Value::List(items)) => items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect(),
            _ => Vec::new(),
        };
        Ok(Self {
            name: info.get("name").and_then(Value::as_str).map(str::to_owned),
            length: info.get("length").and_then(Value::as_int),
            info_hash,
            announce,
            announce_list,
            url_list,
            info,
        })
    }

    /// Lowercase hex info-hash, as used in magnet links.
    #[must_use]
    pub fn info_hash_hex(&self) -> String {
        self.info_hash.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// Build a magnet URI for this torrent, optionally with HTTP web seeds.
    #[must_use]
    pub fn magnet(&self, web_seeds: &[String], trackers: &[String]) -> String {
        let mut uri = format!("magnet:?xt=urn:btih:{}", self.info_hash_hex());
        if let Some(name) = &self.name {
            uri.push_str("&dn=");
            uri.push_str(&percent_encode(name));
        }
        for seed in web_seeds.iter().chain(self.url_list.iter()) {
            uri.push_str("&ws=");
            uri.push_str(&percent_encode(seed));
        }
        for tracker in trackers {
            uri.push_str("&tr=");
            uri.push_str(&percent_encode(tracker));
        }
        uri
    }

    /// Parse a magnet URI into (info-hash hex, web seeds, trackers).
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::Bencode] when `xt=urn:btih:` is missing.
    pub fn parse_magnet(uri: &str) -> Result<MagnetParts, TorrentError> {
        let query = uri
            .split_once('?')
            .map(|(_, q)| q)
            .ok_or_else(|| TorrentError::Bencode("magnet has no query".to_owned()))?;
        let mut info_hash = None;
        let mut seeds = Vec::new();
        let mut trackers = Vec::new();
        for pair in query.split('&') {
            let Some((key, value)) = pair.split_once('=') else {
                continue;
            };
            let value = percent_decode(value);
            match key {
                "xt" if value.starts_with("urn:btih:") => {
                    info_hash = decode_btih(&value["urn:btih:".len()..]);
                }
                "ws" => seeds.push(value),
                "tr" => trackers.push(value),
                _ => {}
            }
        }
        let info_hash =
            info_hash.ok_or_else(|| TorrentError::Bencode("magnet has no btih".to_owned()))?;
        Ok((info_hash, seeds, trackers))
    }
}

fn decode_btih(text: &str) -> Option<[u8; 20]> {
    if text.len() == 40 {
        let mut out = [0u8; 20];
        for (i, chunk) in text.as_bytes().chunks(2).enumerate() {
            out[i] = u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()?;
        }
        return Some(out);
    }
    None
}

fn percent_encode(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b'/' | b':' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(value) = u8::from_str_radix(&text[i + 1..i + 3], 16) {
                out.push(value);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Build a single-file `.torrent` for one file payload.
#[must_use]
pub fn build_single_file_torrent(
    name: &str,
    payload: &[u8],
    piece_length: usize,
    trackers: &[String],
    url_list: &[String],
) -> Vec<u8> {
    let pieces: Vec<u8> = if payload.is_empty() {
        Vec::new()
    } else {
        payload
            .chunks(piece_length.max(16 * 1024))
            .flat_map(|chunk| Sha1::digest(chunk).to_vec())
            .collect()
    };
    let info = Value::dict([
        ("name", Value::string(name)),
        ("length", Value::Int(payload.len() as i64)),
        (
            "piece length",
            Value::Int(piece_length.max(16 * 1024) as i64),
        ),
        ("pieces", Value::Bytes(pieces)),
    ]);
    let mut root = BTreeMap::new();
    if let Some(first) = trackers.first() {
        root.insert("announce".to_owned(), Value::string(first));
    }
    if trackers.len() > 1 {
        root.insert(
            "announce-list".to_owned(),
            Value::List(
                trackers
                    .iter()
                    .map(|t| Value::List(vec![Value::string(t)]))
                    .collect(),
            ),
        );
    }
    if !url_list.is_empty() {
        root.insert(
            "url-list".to_owned(),
            Value::List(url_list.iter().map(|u| Value::string(u)).collect()),
        );
    }
    root.insert("info".to_owned(), info);
    Value::Dict(root).encode()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips_nested_values() {
        let value = Value::dict([
            ("a", Value::Int(42)),
            ("b", Value::string("spam")),
            (
                "c",
                Value::List(vec![Value::Int(-1), Value::string("eggs")]),
            ),
        ]);
        let bytes = value.encode();
        assert_eq!(Value::decode(&bytes).expect("decode"), value);
    }

    #[test]
    fn decodes_canonical_examples() {
        assert_eq!(Value::decode(b"i42e").expect("int"), Value::Int(42));
        assert_eq!(
            Value::decode(b"4:spam").expect("string"),
            Value::string("spam")
        );
        assert_eq!(
            Value::decode(b"l4:spami3ee").expect("list"),
            Value::List(vec![Value::string("spam"), Value::Int(3)])
        );
    }

    #[test]
    fn rejects_malformed_input() {
        assert!(Value::decode(b"i4").is_err());
        assert!(Value::decode(b"4:spamX").is_err());
        assert!(Value::decode(b"d3:abci1e").is_err());
        assert!(Value::decode(b"l").is_err());
    }

    #[test]
    fn torrent_parses_and_hashes_info_dict() {
        let torrent = build_single_file_torrent(
            "nodes.json",
            b"{\"nodes\":[]}",
            16 * 1024,
            &["udp://tracker.example:6969".to_owned()],
            &["https://seed.example/".to_owned()],
        );
        let meta = TorrentMeta::parse(&torrent).expect("parse");
        assert_eq!(meta.name.as_deref(), Some("nodes.json"));
        assert_eq!(meta.url_list, vec!["https://seed.example/".to_owned()]);
        assert_eq!(meta.announce.as_deref(), Some("udp://tracker.example:6969"));
        assert_eq!(meta.info_hash_hex().len(), 40);
        assert_eq!(
            meta.info_hash,
            <[u8; 20]>::try_from(Sha1::digest(meta.info.encode()).as_slice()).expect("20 bytes")
        );
    }

    #[test]
    fn magnet_roundtrips_hash_and_web_seed() {
        let torrent = build_single_file_torrent(
            "nodes.json",
            b"x",
            16 * 1024,
            &[],
            &["https://seed.example/nodes.json".to_owned()],
        );
        let meta = TorrentMeta::parse(&torrent).expect("parse");
        let magnet = meta.magnet(&[], &[]);
        let (hash, seeds, _) = TorrentMeta::parse_magnet(&magnet).expect("magnet");
        assert_eq!(hash, meta.info_hash);
        assert_eq!(seeds, vec!["https://seed.example/nodes.json".to_owned()]);
    }

    #[test]
    fn deep_input_is_rejected() {
        let mut deep = vec![b'l'; MAX_DEPTH + 1];
        deep.push(b'e');
        assert!(Value::decode(&deep).is_err());
    }
}
