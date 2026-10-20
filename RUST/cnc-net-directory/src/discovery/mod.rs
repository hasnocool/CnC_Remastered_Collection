//! Peer discovery backends.
//!
//! The mesh never depends on one entry point. At startup the node resolves
//! every configured seed (literal socket addresses, `host:port` names, and DNS
//! seed hostnames), then keeps listening on LAN multicast so two nodes on the
//! same network find each other without any configuration. A BitTorrent-based
//! fallback lives in [crate::torrent] and is consulted when too few peers are
//! alive.

use std::collections::BTreeSet;
use std::net::SocketAddr;
use std::time::Duration;

use tracing::{debug, info, warn};

use crate::error::{DirectoryError, Result};

pub mod lan;

/// Default LAN multicast group.
pub const DEFAULT_MULTICAST_GROUP: &str = "239.255.42.99:47770";

/// Configuration for LAN peer discovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanConfig {
    /// Multicast group to join and announce on.
    pub group: SocketAddr,
    /// How often to re-announce to the group.
    pub announce_interval: Duration,
}

impl Default for LanConfig {
    fn default() -> Self {
        Self {
            group: DEFAULT_MULTICAST_GROUP
                .parse()
                .unwrap_or_else(|_| SocketAddr::from(([239, 255, 42, 99], 47_770))),
            announce_interval: Duration::from_secs(10),
        }
    }
}

/// Where bootstrap addresses come from.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryConfig {
    /// Literal addresses or `host:port` names, resolved at startup.
    pub static_peers: Vec<String>,
    /// DNS seed hostnames such as `seed.cnc.example` (default port 47770).
    pub dns_seeds: Vec<String>,
    /// LAN multicast settings; `None` disables LAN discovery.
    pub lan: Option<LanConfig>,
}

/// Outcome of a bootstrap resolution pass.
#[derive(Debug, Clone, Default)]
pub struct BootstrapReport {
    /// Unique addresses discovered, in discovery order.
    pub addrs: Vec<SocketAddr>,
    /// Human-readable source labels that produced at least one address.
    pub sources: Vec<String>,
    /// Non-fatal failures, kept for the status endpoint and logs.
    pub errors: Vec<String>,
}

/// Resolve every configured bootstrap source once.
///
/// Resolution never fails as a whole: a broken seed is reported and the rest
/// still contribute. The mesh is expected to start with whatever it can get.
pub async fn resolve_bootstrap(config: &DiscoveryConfig) -> BootstrapReport {
    let mut report = BootstrapReport::default();
    let mut seen: BTreeSet<SocketAddr> = BTreeSet::new();

    for entry in &config.static_peers {
        match resolve_one(entry, 47_770).await {
            Ok(addrs) => {
                let mut added = 0;
                for addr in addrs {
                    if seen.insert(addr) {
                        report.addrs.push(addr);
                        added += 1;
                    }
                }
                if added > 0 {
                    report.sources.push(format!("static:{entry}"));
                }
            }
            Err(err) => {
                warn!(seed = %entry, error = %err, "static bootstrap failed");
                report.errors.push(format!("static {entry}: {err}"));
            }
        }
    }

    for seed in &config.dns_seeds {
        match resolve_dns_seed(seed).await {
            Ok(addrs) => {
                let mut added = 0;
                for addr in addrs {
                    if seen.insert(addr) {
                        report.addrs.push(addr);
                        added += 1;
                    }
                }
                if added > 0 {
                    report.sources.push(format!("dns:{seed}"));
                } else {
                    report.errors.push(format!("dns {seed}: no addresses"));
                }
            }
            Err(err) => {
                warn!(seed = %seed, error = %err, "DNS seed failed");
                report.errors.push(format!("dns {seed}: {err}"));
            }
        }
    }

    if !report.addrs.is_empty() {
        info!(
            peers = report.addrs.len(),
            sources = ?report.sources,
            "bootstrap resolved"
        );
    }
    report
}

/// Resolve a literal address or a `host[:port]` name.
pub async fn resolve_one(entry: &str, default_port: u16) -> Result<Vec<SocketAddr>> {
    let entry = entry.trim();
    if entry.is_empty() {
        return Err(DirectoryError::Config("empty seed".to_owned()));
    }
    if let Ok(addr) = entry.parse::<SocketAddr>() {
        return Ok(vec![addr]);
    }
    // Bracketed IPv6 literals with a port parse above; anything else with a
    // colon is already host:port, otherwise apply the default port.
    let target = if entry.contains(':') {
        entry.to_owned()
    } else {
        format!("{entry}:{default_port}")
    };
    debug!(target = %target, "resolving seed");
    let addrs = tokio::net::lookup_host(target.as_str())
        .await
        .map_err(|e| DirectoryError::discovery("dns", e))?
        .collect::<Vec<_>>();
    if addrs.is_empty() {
        return Err(DirectoryError::Config(format!(
            "seed {entry} resolved to no addresses"
        )));
    }
    Ok(addrs)
}

/// Resolve a DNS seed hostname to gossip addresses.
pub async fn resolve_dns_seed(seed: &str) -> Result<Vec<SocketAddr>> {
    resolve_one(seed, 47_770).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn literal_addresses_resolve_unchanged() {
        let addrs = resolve_one("127.0.0.1:4000", 47_770)
            .await
            .expect("resolve");
        assert_eq!(addrs, vec!["127.0.0.1:4000".parse().expect("addr")]);
    }

    #[tokio::test]
    async fn localhost_without_port_gets_default() {
        let addrs = resolve_one("localhost", 47_770).await.expect("resolve");
        assert!(addrs.iter().all(|a| a.port() == 47_770));
        assert!(!addrs.is_empty());
    }

    #[tokio::test]
    async fn empty_seed_is_rejected() {
        assert!(resolve_one("   ", 47_770).await.is_err());
    }

    #[tokio::test]
    async fn bootstrap_dedupes_sources() {
        let config = DiscoveryConfig {
            static_peers: vec!["127.0.0.1:4000".to_owned(), "127.0.0.1:4000".to_owned()],
            ..DiscoveryConfig::default()
        };
        let report = resolve_bootstrap(&config).await;
        assert_eq!(report.addrs.len(), 1);
    }
}
