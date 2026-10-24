//! C&C Launcher library

pub mod audio_director;
pub mod campaign;
pub mod events_subscriber;
pub mod lobby;
pub mod lobby_screen;
pub mod menu;
pub mod ready;
/// End-of-mission scoring, shared by the launcher shell and the campaign flow.
pub mod score;
pub mod types;

pub use types::{
    LauncherApp, RaLauncherApp, RendererKind, DEFAULT_GAME_ZOOM, MAP_SIZE, RA_TILE_SIZE,
};

/// Structured logging bootstrap shared by the launcher binary and its tests.
///
/// The launcher emits through the `tracing` macros; `cnc-logs` installs a
/// subscriber whose layer also copies every enabled event into a bounded,
/// thread-safe `LogBuffer` for in-process monitoring.
pub mod logging {
    use cnc_logs::config::LogLevelConfig;

    /// Maximum number of structured entries retained by the in-memory buffer.
    ///
    /// Fixed at boot so a runaway log flood cannot grow memory without bound:
    /// past this point the oldest entry is dropped for every new one.
    pub const LOG_BUFFER_CAPACITY: usize = 4096;

    /// Build the launcher's structured logging configuration.
    ///
    /// Global level is `Info`; launcher modules get `Debug` while the noisy
    /// GPU/windowing crates are capped at `Warn`.
    pub fn launcher_log_config() -> cnc_logs::LogConfig {
        let mut config = cnc_logs::LogConfig {
            level: LogLevelConfig::Info,
            ..cnc_logs::LogConfig::default()
        };

        for target in ["cnc_launcher", "cnc_launcher::logging"] {
            config
                .target_levels
                .insert(target.to_string(), LogLevelConfig::Debug);
        }
        for target in ["wgpu", "wgpu_core", "naga", "winit", "calloop"] {
            config
                .target_levels
                .insert(target.to_string(), LogLevelConfig::Warn);
        }

        config
    }
}

/// Original (retail) Tiberian Dawn campaign discovery.
///
/// The launcher ships hand-authored campaign missions under
/// `maps/td_campaign` (new dialect, loaded by `Mission::from_file`). A retail
/// C&C install instead has original-format `SC-*.INI` + `.BIN` files, which
/// only the `scenario_loader` path understands. When the operator points
/// `CNC_ORIGINAL_DATA_DIR` at such an install the launcher discovers those
/// files and validates them through the production loader at startup.
///
/// Unset or unusable data is never fatal: the launcher logs a warning and
/// carries on with the bundled missions.
pub mod original_campaign {
    use std::path::{Path, PathBuf};

    /// Environment variable naming a retail (or extracted) C&C data directory.
    pub const DATA_DIR_ENV: &str = "CNC_ORIGINAL_DATA_DIR";
    /// Directory recursion limit while looking for scenarios.
    const MAX_DEPTH: usize = 3;
    /// Upper bound on candidate INI files, so a huge install cannot stall boot.
    const MAX_FILES: usize = 512;

    /// Summary of one discovery pass.
    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct DiscoveryReport {
        pub root: PathBuf,
        /// `.ini` files inspected.
        pub candidates: usize,
        /// Original-format scenarios that loaded through the production loader.
        pub loaded: usize,
        pub units: usize,
        pub structures: usize,
        pub triggers: usize,
        /// Recoverable loader warnings across every loaded scenario.
        pub warnings: usize,
        /// Candidates that could not be read or parsed, with the reason.
        pub skipped: Vec<(PathBuf, String)>,
    }

    /// The configured original data directory, when the operator set one.
    pub fn original_data_dir() -> Option<PathBuf> {
        let raw = std::env::var_os(DATA_DIR_ENV)?;
        if raw.is_empty() {
            return None;
        }
        Some(PathBuf::from(raw))
    }

    /// Recursively collect `.ini` files under `root` (depth-bounded).
    pub fn discover_td_scenarios(root: &Path) -> Vec<PathBuf> {
        let mut found = Vec::new();
        collect_ini_files(root, 0, &mut found);
        found.sort();
        found
    }

    fn collect_ini_files(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
        if depth > MAX_DEPTH || out.len() >= MAX_FILES {
            return;
        }
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_ini_files(&path, depth + 1, out);
            } else if path
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
            {
                out.push(path);
            }
        }
    }

    /// Load every original-format TD scenario under `root` through the
    /// production loader. Non-original INIs (rules, authored missions) are
    /// ignored; unreadable ones are recorded in `skipped` and never abort the
    /// pass.
    pub fn load_campaign(root: &Path) -> DiscoveryReport {
        let mut report = DiscoveryReport {
            root: root.to_path_buf(),
            ..DiscoveryReport::default()
        };
        for path in discover_td_scenarios(root) {
            report.candidates += 1;
            let text = match std::fs::read_to_string(&path) {
                Ok(text) => text,
                Err(error) => {
                    report.skipped.push((path, error.to_string()));
                    continue;
                }
            };
            if !cnc_tiberian_dawn::scenario_loader::looks_like_original_scenario(&text) {
                continue;
            }
            match cnc_tiberian_dawn::scenario_loader::load_scenario(&path) {
                Ok((scenario, map)) => {
                    let (game, conversion) = cnc_tiberian_dawn::scenario_loader::scenario_to_game(
                        &scenario,
                        map.as_ref(),
                    );
                    report.loaded += 1;
                    report.units += game.units.len();
                    report.structures += game.buildings.len();
                    report.triggers += game.mission.triggers.len();
                    report.warnings += conversion.warnings.len();
                }
                Err(error) => report.skipped.push((path, format!("{error:#}"))),
            }
        }
        report
    }

    /// Startup hook: discover and load original TD campaign data when
    /// `CNC_ORIGINAL_DATA_DIR` is set. Returns `None` (and logs a warning)
    /// when it is unset, not a directory, or yields no usable scenarios.
    pub fn load_from_env() -> Option<DiscoveryReport> {
        let Some(root) = original_data_dir() else {
            tracing::warn!(
                env = DATA_DIR_ENV,
                "original TD data directory not configured; skipping original campaign discovery"
            );
            return None;
        };
        if !root.is_dir() {
            tracing::warn!(
                dir = %root.display(),
                env = DATA_DIR_ENV,
                "original TD data directory does not exist; skipping original campaign discovery"
            );
            return None;
        }
        let report = load_campaign(&root);
        if report.loaded == 0 {
            tracing::warn!(
                dir = %root.display(),
                candidates = report.candidates,
                skipped = report.skipped.len(),
                "no loadable original TD scenarios found; continuing with bundled missions"
            );
            return None;
        }
        tracing::info!(
            dir = %root.display(),
            loaded = report.loaded,
            units = report.units,
            structures = report.structures,
            triggers = report.triggers,
            warnings = report.warnings,
            skipped = report.skipped.len(),
            "original TD campaign scenarios loaded"
        );
        Some(report)
    }
}

#[cfg(test)]
mod original_campaign_tests {
    use std::path::{Path, PathBuf};

    use super::original_campaign::{discover_td_scenarios, load_campaign};

    fn fixtures_td() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/td")
    }

    #[test]
    fn fixtures_dir_discovers_and_loads_the_original_scenario() {
        let root = fixtures_td();
        let candidates = discover_td_scenarios(&root);
        assert!(
            candidates
                .iter()
                .any(|path| path.file_name().is_some_and(|name| name == "FIXTURE.INI")),
            "{candidates:?}"
        );

        let report = load_campaign(&root);
        assert_eq!(report.loaded, 1, "{report:?}");
        assert_eq!(report.units, 3, "{report:?}");
        assert_eq!(report.structures, 2, "{report:?}");
        assert_eq!(report.triggers, 2, "{report:?}");
        assert!(report.skipped.is_empty(), "{report:?}");
    }

    #[test]
    fn missing_directory_yields_an_empty_pass() {
        let report = load_campaign(Path::new("/definitely/not/a/real/cnc/install"));
        assert_eq!(report.loaded, 0);
        assert_eq!(report.candidates, 0);
        assert!(report.skipped.is_empty());
    }
}
