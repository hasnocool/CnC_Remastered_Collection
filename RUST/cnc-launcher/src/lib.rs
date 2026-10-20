//! C&C Launcher library

pub mod events_subscriber;
pub mod lobby;
pub mod lobby_screen;
pub mod menu;
pub mod ready;
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
        let mut config = cnc_logs::LogConfig::default();
        config.level = LogLevelConfig::Info;

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
