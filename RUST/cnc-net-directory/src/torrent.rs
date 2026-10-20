//! BitTorrent fallback discovery.
//!
//! If the gossip mesh cannot find enough peers (cold start, a partition, or
//! every hard-coded seed going down), the node can recover a peer list from a
//! torrent:
//!
//! 1. A plain HTTP JSON node list (`--torrent https://.../nodes.json`).
//! 2. A magnet or `.torrent` fetched through a local qBittorrent Web API, then
//!    read from the client's download directory.
//! 3. A BEP-19 web seed carried by the magnet or torrent, fetched over plain
//!    HTTP — no BitTorrent client required.
//!
//! The reverse direction is also provided: [build_bundle] writes a node list,
//! a `.torrent` and a magnet link, so operators can keep publishing the mesh
//! entry points to a torrent swarm while the nodes themselves stay independent.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::bencode::{build_single_file_torrent, TorrentMeta, Value};
use crate::error::TorrentError;
use crate::model::now_ms;

/// Default name of the published node list inside the torrent.
pub const NODES_FILENAME: &str = "nodes.json";

/// JSON document shared through the torrent fallback.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeList {
    /// Unix milliseconds when the list was generated.
    #[serde(default)]
    pub generated_at_ms: i64,
    /// Gossip endpoints.
    #[serde(default)]
    pub nodes: Vec<SocketAddr>,
    /// Optional HTTP browse endpoints.
    #[serde(default)]
    pub http: Vec<String>,
    /// Operator note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

use std::net::SocketAddr;

impl NodeList {
    /// Build a list stamped with the current time.
    #[must_use]
    pub fn new(nodes: Vec<SocketAddr>, http: Vec<String>, note: Option<String>) -> Self {
        Self {
            generated_at_ms: now_ms(),
            nodes,
            http,
            note,
        }
    }

    /// Parse the JSON payload.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::NodeList] when the document is not a node list.
    pub fn parse_json(bytes: &[u8]) -> Result<Self, TorrentError> {
        serde_json::from_slice(bytes).map_err(|e| TorrentError::NodeList(e.to_string()))
    }

    /// Serialise the payload.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::NodeList] if serialisation fails.
    pub fn to_json(&self) -> Result<Vec<u8>, TorrentError> {
        serde_json::to_vec_pretty(self).map_err(|e| TorrentError::NodeList(e.to_string()))
    }
}

/// Where the fallback list comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TorrentSource {
    /// Plain HTTP(S) JSON document.
    JsonUrl(String),
    /// Magnet URI (`magnet:?xt=urn:btih:...`).
    Magnet(String),
    /// Local `.torrent` file.
    TorrentFile(PathBuf),
}

impl TorrentSource {
    /// Short label for logs.
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::JsonUrl(url) => format!("json:{url}"),
            Self::Magnet(uri) => {
                let short: String = uri.chars().take(48).collect();
                format!("magnet:{short}")
            }
            Self::TorrentFile(path) => format!("torrent:{}", path.display()),
        }
    }
}

impl FromStr for TorrentSource {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let text = text.trim();
        if text.starts_with("magnet:") {
            return Ok(Self::Magnet(text.to_owned()));
        }
        if text.starts_with("http://") || text.starts_with("https://") {
            return Ok(Self::JsonUrl(text.to_owned()));
        }
        if text.is_empty() {
            return Err("empty torrent source".to_owned());
        }
        Ok(Self::TorrentFile(PathBuf::from(text)))
    }
}

/// qBittorrent Web API credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QBittorrentConfig {
    /// Base URL, e.g. `http://127.0.0.1:8080`.
    pub base_url: String,
    /// Web UI username.
    pub username: String,
    /// Web UI password.
    pub password: String,
}

/// Fallback configuration.
#[derive(Debug, Clone)]
pub struct TorrentConfig {
    /// Where the node list lives.
    pub source: TorrentSource,
    /// Expected file name inside the torrent.
    pub nodes_filename: String,
    /// Optional local qBittorrent client.
    pub qbittorrent: Option<QBittorrentConfig>,
    /// Overall wait budget.
    pub timeout: Duration,
    /// Poll cadence while waiting for a download.
    pub poll_interval: Duration,
}

impl Default for TorrentConfig {
    fn default() -> Self {
        Self {
            source: TorrentSource::JsonUrl(String::new()),
            nodes_filename: NODES_FILENAME.to_owned(),
            qbittorrent: None,
            timeout: Duration::from_secs(30),
            poll_interval: Duration::from_millis(500),
        }
    }
}

/// Resolves node lists from the configured fallback source.
#[derive(Debug, Clone)]
pub struct TorrentResolver {
    client: Client,
    config: TorrentConfig,
}

impl TorrentResolver {
    /// Build a resolver.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::Fetch] if the HTTP client cannot be created.
    pub fn new(config: TorrentConfig) -> Result<Self, TorrentError> {
        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent(concat!("cnc-directory/", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| TorrentError::Fetch(e.to_string()))?;
        Ok(Self { client, config })
    }

    /// The configured source.
    #[must_use]
    pub fn source(&self) -> &TorrentSource {
        &self.config.source
    }

    /// Resolve the node list, trying every available transport.
    ///
    /// # Errors
    ///
    /// Returns the most specific failure when no transport produces a list.
    pub async fn resolve(&self) -> Result<NodeList, TorrentError> {
        info!(source = %self.config.source.describe(), "torrent fallback: resolving node list");
        match &self.config.source {
            TorrentSource::JsonUrl(url) => self.fetch_json(url).await,
            TorrentSource::Magnet(uri) => self.resolve_magnet(uri).await,
            TorrentSource::TorrentFile(path) => self.resolve_file(path).await,
        }
    }

    async fn fetch_json(&self, url: &str) -> Result<NodeList, TorrentError> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| TorrentError::Fetch(e.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            return Err(TorrentError::Http {
                status: status.as_u16(),
                url: url.to_owned(),
            });
        }
        let bytes = response
            .bytes()
            .await
            .map_err(|e| TorrentError::Fetch(e.to_string()))?;
        NodeList::parse_json(&bytes)
    }

    async fn resolve_magnet(&self, uri: &str) -> Result<NodeList, TorrentError> {
        let (info_hash, web_seeds, _trackers) = TorrentMeta::parse_magnet(uri)?;
        let mut last_error: Option<TorrentError> = None;

        if let Some(qb) = &self.config.qbittorrent {
            let client = QBittorrentClient::new(qb.clone(), self.client.clone());
            match self
                .read_via_qbittorrent(&client, QbAdd::Magnet(uri), info_hash)
                .await
            {
                Ok(list) => return Ok(list),
                Err(err) => {
                    warn!(error = %err, "qBittorrent path failed; trying web seeds");
                    last_error = Some(err);
                }
            }
        }

        for seed in web_seeds {
            for candidate in web_seed_candidates(&seed, &self.config.nodes_filename) {
                match self.fetch_json(&candidate).await {
                    Ok(list) => return Ok(list),
                    Err(err) => {
                        debug!(url = %candidate, error = %err, "web seed candidate failed");
                        last_error = Some(err);
                    }
                }
            }
        }

        Err(last_error.unwrap_or(TorrentError::NoTransport))
    }

    async fn resolve_file(&self, path: &Path) -> Result<NodeList, TorrentError> {
        let bytes = tokio::fs::read(path)
            .await
            .map_err(|e| TorrentError::Fetch(format!("{}: {e}", path.display())))?;
        let meta = TorrentMeta::parse(&bytes)?;
        let mut last_error: Option<TorrentError> = None;

        if let Some(qb) = &self.config.qbittorrent {
            let client = QBittorrentClient::new(qb.clone(), self.client.clone());
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("nodes.torrent")
                .to_owned();
            match self
                .read_via_qbittorrent(
                    &client,
                    QbAdd::Torrent {
                        name,
                        bytes: bytes.clone(),
                    },
                    meta.info_hash,
                )
                .await
            {
                Ok(list) => return Ok(list),
                Err(err) => {
                    warn!(error = %err, "qBittorrent path failed; trying web seeds");
                    last_error = Some(err);
                }
            }
        }

        for seed in &meta.url_list {
            for candidate in web_seed_candidates(seed, &self.config.nodes_filename) {
                match self.fetch_json(&candidate).await {
                    Ok(list) => return Ok(list),
                    Err(err) => {
                        debug!(url = %candidate, error = %err, "web seed candidate failed");
                        last_error = Some(err);
                    }
                }
            }
        }

        Err(last_error.unwrap_or(TorrentError::NoTransport))
    }

    async fn read_via_qbittorrent(
        &self,
        client: &QBittorrentClient,
        add: QbAdd<'_>,
        info_hash: [u8; 20],
    ) -> Result<NodeList, TorrentError> {
        client.login().await?;
        match &add {
            QbAdd::Magnet(uri) => client.add_magnet(uri).await?,
            QbAdd::Torrent { name, bytes } => client.add_torrent(name, bytes).await?,
        }
        let path = client
            .wait_for_file(
                info_hash,
                &self.config.nodes_filename,
                self.config.timeout,
                self.config.poll_interval,
            )
            .await?;
        let bytes = tokio::fs::read(&path)
            .await
            .map_err(|e| TorrentError::Fetch(format!("{}: {e}", path.display())))?;
        let list = NodeList::parse_json(&bytes)?;
        info!(path = %path.display(), nodes = list.nodes.len(), "torrent fallback: node list recovered");
        Ok(list)
    }
}

fn web_seed_candidates(seed: &str, filename: &str) -> Vec<String> {
    let seed = seed.trim_end_matches('/');
    let mut out = Vec::new();
    if seed.ends_with(".json") {
        out.push(seed.to_owned());
    }
    out.push(format!("{seed}/{filename}"));
    if !seed.ends_with(".json") && !seed.ends_with(".torrent") {
        out.push(format!("{seed}/{}", filename.trim_start_matches("nodes/")));
    }
    out.sort();
    out.dedup();
    out
}

enum QbAdd<'a> {
    Magnet(&'a str),
    Torrent { name: String, bytes: Vec<u8> },
}

/// Minimal qBittorrent Web API v2 client.
#[derive(Debug, Clone)]
pub struct QBittorrentClient {
    config: QBittorrentConfig,
    http: Client,
    sid: std::sync::Arc<Mutex<Option<String>>>,
}

/// One torrent row from `torrents/info`.
#[derive(Debug, Clone, Deserialize)]
struct QbTorrent {
    hash: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    save_path: String,
    #[serde(default)]
    content_path: String,
    #[serde(default)]
    progress: f64,
}

/// One file row from `torrents/files`.
#[derive(Debug, Clone, Deserialize)]
struct QbFile {
    name: String,
    #[serde(default)]
    progress: f64,
}

impl QBittorrentClient {
    /// Build a client.
    #[must_use]
    pub fn new(config: QBittorrentConfig, http: Client) -> Self {
        Self {
            config: QBittorrentConfig {
                base_url: config.base_url.trim_end_matches('/').to_owned(),
                ..config
            },
            http,
            sid: std::sync::Arc::new(Mutex::new(None)),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url, path)
    }

    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let mut builder = self
            .http
            .request(method, self.url(path))
            .header(reqwest::header::REFERER, &self.config.base_url)
            .header(reqwest::header::ORIGIN, &self.config.base_url);
        if let Some(sid) = self.sid.lock().clone() {
            builder = builder.header(reqwest::header::COOKIE, format!("SID={sid}"));
        }
        builder
    }

    /// Authenticate against the Web UI.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::QBittorrent] when credentials are rejected.
    pub async fn login(&self) -> Result<(), TorrentError> {
        let response = self
            .http
            .post(self.url("/api/v2/auth/login"))
            .header(reqwest::header::REFERER, &self.config.base_url)
            .header(reqwest::header::ORIGIN, &self.config.base_url)
            .form(&[
                ("username", self.config.username.as_str()),
                ("password", self.config.password.as_str()),
            ])
            .send()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("login: {e}")))?;
        if !response.status().is_success() {
            return Err(TorrentError::QBittorrent(format!(
                "login returned {}",
                response.status().as_u16()
            )));
        }
        let sid = response
            .cookies()
            .find(|c| c.name() == "SID")
            .map(|c| c.value().to_owned());
        let body = response
            .text()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("login body: {e}")))?;
        if body.trim() == "Fails." {
            return Err(TorrentError::QBittorrent("bad credentials".to_owned()));
        }
        *self.sid.lock() = sid;
        Ok(())
    }

    /// Add a magnet and return immediately.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::QBittorrent] when the client rejects the magnet.
    pub async fn add_magnet(&self, uri: &str) -> Result<(), TorrentError> {
        let response = self
            .request(reqwest::Method::POST, "/api/v2/torrents/add")
            .form(&[("urls", uri), ("paused", "false")])
            .send()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("add: {e}")))?;
        Self::expect_ok(response, "add magnet").await
    }

    /// Add a `.torrent` payload and return immediately.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::QBittorrent] when the client rejects the file.
    pub async fn add_torrent(&self, name: &str, bytes: &[u8]) -> Result<(), TorrentError> {
        let part = reqwest::multipart::Part::bytes(bytes.to_vec())
            .file_name(name.to_owned())
            .mime_str("application/x-bittorrent")
            .map_err(|e| TorrentError::QBittorrent(e.to_string()))?;
        let form = reqwest::multipart::Form::new().part("torrents", part);
        let response = self
            .request(reqwest::Method::POST, "/api/v2/torrents/add")
            .multipart(form)
            .send()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("add torrent: {e}")))?;
        Self::expect_ok(response, "add torrent").await
    }

    async fn expect_ok(response: reqwest::Response, action: &str) -> Result<(), TorrentError> {
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("{action}: {e}")))?;
        if status.is_success() && !body.trim().starts_with("Fails") {
            Ok(())
        } else {
            Err(TorrentError::QBittorrent(format!(
                "{action}: status {status}, body {body:?}"
            )))
        }
    }

    /// Fetch torrent rows for one hash.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::QBittorrent] on transport or decode failure.
    pub async fn torrents(
        &self,
        info_hash: [u8; 20],
    ) -> Result<Vec<(String, String, String, f64, String)>, TorrentError> {
        let hash = hex_lower(&info_hash);
        let response = self
            .request(
                reqwest::Method::GET,
                &format!("/api/v2/torrents/info?hashes={hash}"),
            )
            .send()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("info: {e}")))?;
        if !response.status().is_success() {
            return Err(TorrentError::QBittorrent(format!(
                "info returned {}",
                response.status().as_u16()
            )));
        }
        let rows: Vec<QbTorrent> = response
            .json()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("info decode: {e}")))?;
        Ok(rows
            .into_iter()
            .map(|t| (t.hash, t.name, t.save_path, t.progress, t.content_path))
            .collect())
    }

    /// Fetch file rows for one hash.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::QBittorrent] on transport or decode failure.
    pub async fn files(&self, info_hash: [u8; 20]) -> Result<Vec<(String, f64)>, TorrentError> {
        let hash = hex_lower(&info_hash);
        let response = self
            .request(
                reqwest::Method::GET,
                &format!("/api/v2/torrents/files?hash={hash}"),
            )
            .send()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("files: {e}")))?;
        if !response.status().is_success() {
            return Err(TorrentError::QBittorrent(format!(
                "files returned {}",
                response.status().as_u16()
            )));
        }
        let rows: Vec<QbFile> = response
            .json()
            .await
            .map_err(|e| TorrentError::QBittorrent(format!("files decode: {e}")))?;
        Ok(rows.into_iter().map(|f| (f.name, f.progress)).collect())
    }

    /// Poll until the node-list file is fully downloaded, then return its path.
    ///
    /// # Errors
    ///
    /// Returns [TorrentError::Timeout] when the file never completes.
    pub async fn wait_for_file(
        &self,
        info_hash: [u8; 20],
        filename: &str,
        timeout: Duration,
        poll: Duration,
    ) -> Result<PathBuf, TorrentError> {
        let started = Instant::now();
        loop {
            if started.elapsed() > timeout {
                return Err(TorrentError::Timeout(filename.to_owned()));
            }
            if let Some(path) = self.probe_file(info_hash, filename).await? {
                return Ok(path);
            }
            tokio::time::sleep(poll).await;
        }
    }

    async fn probe_file(
        &self,
        info_hash: [u8; 20],
        filename: &str,
    ) -> Result<Option<PathBuf>, TorrentError> {
        let torrents = self.torrents(info_hash).await?;
        let Some((_hash, _name, save_path, progress, content_path)) = torrents.first() else {
            return Ok(None);
        };
        if *progress < 1.0 && content_path.is_empty() {
            return Ok(None);
        }
        for (name, file_progress) in self.files(info_hash).await? {
            if !name.ends_with(filename) || file_progress < 1.0 {
                continue;
            }
            let direct = PathBuf::from(content_path);
            if direct.is_file() {
                return Ok(Some(direct));
            }
            let joined = Path::new(save_path).join(&name);
            if joined.is_file() {
                return Ok(Some(joined));
            }
        }
        Ok(None)
    }
}

fn hex_lower(hash: &[u8; 20]) -> String {
    hash.iter().map(|b| format!("{b:02x}")).collect()
}

/// A published node-list torrent.
#[derive(Debug, Clone)]
pub struct TorrentBundle {
    /// Serialised JSON node list.
    pub nodes_json: Vec<u8>,
    /// Bencoded `.torrent` payload.
    pub torrent: Vec<u8>,
    /// Shareable magnet URI, including web seeds.
    pub magnet: String,
    /// Info-hash in lowercase hex.
    pub info_hash: String,
    /// File name used for the JSON payload.
    pub filename: String,
}

/// Build a node-list torrent bundle with optional trackers and web seeds.
///
/// # Errors
///
/// Returns [TorrentError] if the list cannot be serialised or parsed back.
pub fn build_bundle(
    list: &NodeList,
    filename: &str,
    trackers: &[String],
    web_seeds: &[String],
    piece_length: usize,
) -> Result<TorrentBundle, TorrentError> {
    let nodes_json = list.to_json()?;
    let torrent =
        build_single_file_torrent(filename, &nodes_json, piece_length, trackers, web_seeds);
    let meta = TorrentMeta::parse(&torrent)?;
    let magnet = meta.magnet(web_seeds, trackers);
    Ok(TorrentBundle {
        nodes_json,
        torrent,
        magnet,
        info_hash: meta.info_hash_hex(),
        filename: filename.to_owned(),
    })
}

/// Write the bundle to a directory: the JSON payload and the `.torrent` file.
///
/// # Errors
///
/// Returns the underlying IO error.
pub fn write_bundle(dir: &Path, bundle: &TorrentBundle) -> std::io::Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    std::fs::write(dir.join(&bundle.filename), &bundle.nodes_json)?;
    let torrent_path = dir.join(format!(
        "{}.torrent",
        bundle.filename.trim_end_matches(".json")
    ));
    std::fs::write(&torrent_path, &bundle.torrent)?;
    Ok(torrent_path)
}

/// Register the bundle with a local qBittorrent so it starts seeding.
///
/// # Errors
///
/// Returns [TorrentError] when login or add fails.
pub async fn seed_with_qbittorrent(
    config: &QBittorrentConfig,
    bundle: &TorrentBundle,
    http: Client,
) -> Result<(), TorrentError> {
    let client = QBittorrentClient::new(config.clone(), http);
    client.login().await?;
    client
        .add_torrent(
            &format!("{}.torrent", bundle.filename.trim_end_matches(".json")),
            &bundle.torrent,
        )
        .await
}

/// Small helper for converting a JSON map into a bencoded dictionary (used by
/// tests and future metadata extensions).
#[must_use]
pub fn json_to_bencode(value: &serde_json::Value) -> Option<Value> {
    match value {
        serde_json::Value::Null => None,
        serde_json::Value::Bool(b) => Some(Value::Int(i64::from(*b))),
        serde_json::Value::Number(n) => n.as_i64().map(Value::Int),
        serde_json::Value::String(s) => Some(Value::string(s)),
        serde_json::Value::Array(items) => Some(Value::List(
            items.iter().filter_map(json_to_bencode).collect(),
        )),
        serde_json::Value::Object(map) => {
            let mut out = BTreeMap::new();
            for (key, value) in map {
                if let Some(value) = json_to_bencode(value) {
                    out.insert(key.clone(), value);
                }
            }
            Some(Value::Dict(out))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_list_roundtrips() {
        let list = NodeList::new(
            vec!["127.0.0.1:4000".parse().expect("addr")],
            vec![],
            Some("test".to_owned()),
        );
        let json = list.to_json().expect("json");
        assert_eq!(NodeList::parse_json(&json).expect("parse"), list);
    }

    #[test]
    fn source_parsing_prefers_magnet_and_urls() {
        assert!(matches!(
            "magnet:?xt=urn:btih:00"
                .parse::<TorrentSource>()
                .expect("magnet"),
            TorrentSource::Magnet(_)
        ));
        assert!(matches!(
            "https://example.com/nodes.json"
                .parse::<TorrentSource>()
                .expect("url"),
            TorrentSource::JsonUrl(_)
        ));
        assert!(matches!(
            "/tmp/nodes.torrent".parse::<TorrentSource>().expect("file"),
            TorrentSource::TorrentFile(_)
        ));
    }

    #[test]
    fn bundle_is_self_consistent() {
        let list = NodeList::new(
            vec!["127.0.0.1:4000".parse().expect("addr")],
            vec!["http://127.0.0.1:8088".to_owned()],
            None,
        );
        let bundle = build_bundle(
            &list,
            "nodes.json",
            &["udp://tracker.example:6969".to_owned()],
            &["https://seed.example/".to_owned()],
            16 * 1024,
        )
        .expect("bundle");
        assert_eq!(bundle.info_hash.len(), 40);
        assert!(bundle.magnet.contains("urn:btih:"));
        assert!(bundle.magnet.contains("ws="));
        let meta = TorrentMeta::parse(&bundle.torrent).expect("parse");
        assert_eq!(meta.info_hash_hex(), bundle.info_hash);
    }

    #[test]
    fn web_seed_candidates_include_the_filename() {
        let out = web_seed_candidates("https://seed.example/", "nodes.json");
        assert!(out.contains(&"https://seed.example/nodes.json".to_owned()));
        let out = web_seed_candidates("https://seed.example/nodes.json", "nodes.json");
        assert!(out.contains(&"https://seed.example/nodes.json".to_owned()));
    }

    #[test]
    fn bundle_writes_both_files() {
        let dir = std::env::temp_dir().join(format!("cnc-dir-{}", uuid::Uuid::new_v4()));
        let list = NodeList::new(vec![], vec![], None);
        let bundle = build_bundle(&list, "nodes.json", &[], &[], 16 * 1024).expect("bundle");
        let torrent = write_bundle(&dir, &bundle).expect("write");
        assert!(dir.join("nodes.json").is_file());
        assert!(torrent.is_file());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
