//! Launcher options data layer (plan line 206, Phase 9.1 "Options
//! (video, audio, controls)").
//!
//! This module owns the persisted launcher settings only: which renderer
//! to prefer, the audio volume buses and the read-only display list of
//! the hard-coded hotkeys. Rendering the Options modal and reacting to it
//! are owned by the UI lane; nothing here touches window, renderer or
//! game state.

use std::path::{Path, PathBuf};

use cnc_audio::AudioMixer;

/// Environment variable that points directly at a launcher-options file.
pub const OPTIONS_FILE_ENV: &str = "CNC_OPTIONS_FILE";

/// Environment variable naming the save directory (mirrors
/// `main.rs::default_campaign_progress_path`).
const SAVE_DIR_ENV: &str = "CNC_SAVE_DIR";

/// Default save directory when `CNC_SAVE_DIR` is unset (same fallback as
/// the campaign-progress paths in `main.rs`).
const DEFAULT_SAVE_DIR: &str = "saves";

/// Options filename inside the save directory.
const OPTIONS_FILE_NAME: &str = "launcher-options.ini";

/// Persisted launcher settings: video renderer choice, audio volume buses
/// and the read-only controls display list (plan line 206).
#[derive(Clone, Debug, PartialEq)]
pub struct LauncherOptions {
    /// Preferred renderer backend ("wgpu" or "software"); validated through
    /// `RendererKind::parse` at resolution time (main.rs:194).
    pub renderer: String,
    /// Music bus volume, 0..=1 (cnc-audio `AudioMixer::set_music_volume`).
    pub music_volume: f32,
    /// Effects (SFX + voice) bus volume, 0..=1
    /// (cnc-audio `AudioMixer::set_effects_volume`).
    pub effects_volume: f32,
    /// Master volume, 0..=1 (cnc-audio `AudioMixer::set_master_volume`).
    pub master_volume: f32,
    /// Game speed multiplier, 0.25..=3.0 (applied to running game simulation).
    pub game_speed: f32,
    /// Maximum buildable tech level (0 = no cap). Matches `BuildLevel`.
    pub build_threshold: u8,
    /// Shroud / fog disabled (`IsShadowGrow` / `ShroudOff`).
    pub shroud_off: bool,
    /// One-tick strikes enabled (instant attack execution).
    pub one_tick_strikes: bool,
    /// Read-only display list of the hard-coded hotkeys as
    /// `(action, key)` pairs; never persisted because rebinding is out of
    /// scope for this lane.
    pub controls: Vec<(String, String)>,
}

impl Default for LauncherOptions {
    fn default() -> Self {
        Self {
            renderer: "wgpu".to_string(),
            music_volume: 1.0,
            effects_volume: 1.0,
            master_volume: 1.0,
            game_speed: 1.0,
            build_threshold: 0,
            shroud_off: false,
            one_tick_strikes: false,
            controls: Self::default_controls(),
        }
    }
}

impl LauncherOptions {
    /// The hotkey display list, mirroring the hard-coded bindings in
    /// `main.rs` (TD key handler ~4360-4543, RA key handler ~7502-7635)
    /// and `menu.rs` `MainMenu::on_key` (~329-354). Ordered and
    /// deterministic (a `Vec`, no map iteration order) and display-only:
    /// rebinding belongs to a later lane.
    pub fn default_controls() -> Vec<(String, String)> {
        let controls = [
            // Shared game hotkeys (TD main.rs ~4394-4449, RA main.rs ~7580-7595).
            ("Quick save", "F5"),
            ("Quick load", "F9"),
            ("Game speed down", "F6"),
            ("Game speed up", "F7"),
            ("Zoom in", "+ / ="),
            ("Zoom out", "- / _"),
            ("Select all units", "A"),
            ("Set / jump bookmark", "Ctrl+1-4"),
            ("Store / recall control group", "Alt+1-4"),
            // Menu navigation (menu.rs on_key ~331-347).
            ("Move cursor up", "ArrowUp / W"),
            ("Move cursor down", "ArrowDown / S"),
            ("Confirm", "Enter"),
            ("Back / Quit", "Escape"),
            // Tiberian Dawn specifics (TD main.rs ~4356-4542).
            ("TD: pause", "Escape"),
            ("TD: pause menu", "ArrowUp / ArrowDown / Enter"),
            ("TD: sell building", "X"),
            ("TD: repair building", "R"),
            ("TD: queue buildable", "1-4, 5-9, 0"),
            ("TD: cancel production", "Backspace"),
            ("TD: ion cannon", "I"),
            ("TD: nuclear strike", "N"),
            // Red Alert specifics (RA main.rs ~7502-7634).
            ("RA: pause", "Escape"),
            ("RA: pause menu", "ArrowUp / ArrowDown / Enter"),
            ("RA: produce unit", "Space / 1 / 2 / 3"),
            ("RA: cancel production", "Backspace"),
            ("RA: scatter selection", "G"),
            ("RA: toggle power", "O"),
            ("RA: repair structure", "R"),
            ("RA: stop selection", "S"),
            ("RA: sell structure", "X"),
            ("RA: deploy MCV", "D"),
            ("RA: place structure", "P / B / W / T"),
            ("RA: chronoshift", "C"),
            ("RA: iron curtain", "I"),
            ("RA: open chat (network)", "Y"),
            ("RA: send chat (network)", "Enter"),
        ];
        controls
            .into_iter()
            .map(|(action, key)| (action.to_string(), key.to_string()))
            .collect()
    }

    /// Resolve the persisted renderer choice through `RendererKind::parse`
    /// (main.rs:194). An empty or invalid value falls back to wgpu with a
    /// status note so a bad file never blocks boot (plan line 206).
    pub fn resolved_renderer_kind(&self) -> crate::RendererKind {
        let requested = self.renderer.trim();
        if requested.is_empty() {
            return crate::RendererKind::Wgpu;
        }
        match crate::RendererKind::parse(requested) {
            Ok(kind) => kind,
            Err(_) => {
                tracing::warn!(
                    requested,
                    "persisted renderer is invalid; falling back to wgpu"
                );
                crate::RendererKind::Wgpu
            }
        }
    }

    /// Push the persisted volumes into `mixer` (plan line 206). The call
    /// sites are `AudioMixer::set_master_volume`,
    /// `AudioMixer::set_music_volume` and
    /// `AudioMixer::set_effects_volume` in cnc-audio.
    pub fn apply_to_mixer(&self, mixer: &AudioMixer) {
        mixer.set_master_volume(self.master_volume);
        mixer.set_music_volume(self.music_volume);
        mixer.set_effects_volume(self.effects_volume);
    }

    /// Filesystem path of the options file for a CLI `--options` value,
    /// mirroring `main.rs::default_campaign_progress_path`: an explicit
    /// `CNC_OPTIONS_FILE` wins over `CNC_SAVE_DIR`/launcher-options.ini,
    /// which falls back to ./saves (plan line 206).
    pub fn options_path(cli: Option<&Path>) -> PathBuf {
        let env_file = std::env::var(OPTIONS_FILE_ENV).ok();
        let save_dir = std::env::var(SAVE_DIR_ENV).ok();
        Self::resolve_options_path(cli, env_file.as_deref(), save_dir.as_deref())
    }

    /// Pure path resolution behind [`LauncherOptions::options_path`], split
    /// out so tests stay independent of the process environment.
    pub fn resolve_options_path(
        cli: Option<&Path>,
        env_file: Option<&str>,
        save_dir: Option<&str>,
    ) -> PathBuf {
        if let Some(cli) = cli {
            return cli.to_path_buf();
        }
        if let Some(env_file) = env_file.filter(|value| !value.is_empty()) {
            return PathBuf::from(env_file);
        }
        let dir = save_dir
            .filter(|value| !value.is_empty())
            .unwrap_or(DEFAULT_SAVE_DIR);
        PathBuf::from(dir).join(OPTIONS_FILE_NAME)
    }

    /// Load the options file for this run (default location or the CLI
    /// `--options` override). Missing or corrupt files fall back to
    /// [`LauncherOptions::default`] (plan line 206).
    pub fn load(cli: Option<&Path>) -> Self {
        Self::load_from(&Self::options_path(cli))
    }

    /// Read options from `path`; a missing or unreadable file (or any
    /// unrecognized content) yields [`LauncherOptions::default`], the same
    /// tolerant behavior as the campaign-progress reader in `main.rs`
    /// (`load_campaign_progress`, ~5312).
    pub fn load_from(path: &Path) -> Self {
        let mut options = Self::default();
        let Ok(text) = std::fs::read_to_string(path) else {
            return options;
        };
        for line in text.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            match key.trim().to_ascii_lowercase().as_str() {
                "renderer" => options.renderer = value.trim().to_string(),
                "musicvolume" => options.music_volume = volume_or(value, options.music_volume),
                "effectsvolume" => {
                    options.effects_volume = volume_or(value, options.effects_volume)
                }
                "mastervolume" => options.master_volume = volume_or(value, options.master_volume),
                "gamespeed" => options.game_speed = speed_or(value, options.game_speed),
                "buildthreshold" => {
                    if let Ok(v) = value.trim().parse::<u8>() {
                        options.build_threshold = v;
                    }
                }
                "shroudoff" => {
                    options.shroud_off =
                        value.trim().eq_ignore_ascii_case("true") || value.trim() == "1";
                }
                "onetickstrikes" => {
                    options.one_tick_strikes =
                        value.trim().eq_ignore_ascii_case("true") || value.trim() == "1";
                }
                _ => {}
            }
        }
        options
    }

    /// Persist the options to the default (or CLI-overridden) path.
    /// Called by the options-modal UI lane; unused by the boot path so a
    /// read-only launch never writes to disk.
    #[allow(dead_code)] // Phase 9.1 UI lane applies user changes through this.
    pub fn save(&self, cli: Option<&Path>) -> std::io::Result<()> {
        self.save_to(&Self::options_path(cli))
    }

    /// Write the options INI to `path`, creating parent directories like
    /// the campaign-progress saver (`main.rs::save_campaign_progress`,
    /// ~5350).
    #[allow(dead_code)] // Reached through `save` / the Phase 9.1 UI lane.
    pub fn save_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(
            path,
            format!(
                "[Options]\nRenderer={}\nMusicVolume={}\nEffectsVolume={}\nMasterVolume={}\nGameSpeed={}\nBuildThreshold={}\nShroudOff={}\nOneTickStrikes={}\n",
                self.renderer,
                self.music_volume,
                self.effects_volume,
                self.master_volume,
                self.game_speed,
                self.build_threshold,
                self.shroud_off,
                self.one_tick_strikes
            ),
        )
    }
}

/// Parse a persisted volume: finite values clamp into 0..=1, everything
/// else (unparseable text, NaN, infinities) keeps `fallback`.
fn volume_or(value: &str, fallback: f32) -> f32 {
    match value.trim().parse::<f32>() {
        Ok(volume) if volume.is_finite() => volume.clamp(0.0, 1.0),
        _ => fallback,
    }
}

/// Parse a persisted game speed: finite values clamp into 0.25..=3.0 (the
/// in-game speed range in `main.rs::adjust_td_speed`), everything else keeps
/// `fallback`.
fn speed_or(value: &str, fallback: f32) -> f32 {
    match value.trim().parse::<f32>() {
        Ok(speed) if speed.is_finite() => speed.clamp(0.25, 3.0),
        _ => fallback,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LauncherApp;
    use crate::RendererKind;
    use cnc_core::RulesDatabase;

    // --- LauncherOptions: defaults, validation, persistence ---

    #[test]
    fn defaults_pick_wgpu_full_volume_and_a_controls_list() {
        let options = LauncherOptions::default();
        assert_eq!(options.renderer, "wgpu");
        assert_eq!(options.music_volume, 1.0);
        assert_eq!(options.effects_volume, 1.0);
        assert_eq!(options.master_volume, 1.0);
        assert!(!options.controls.is_empty());
        assert_eq!(options.controls, LauncherOptions::default_controls());
    }

    #[test]
    fn renderer_resolution_matches_rendererkind_parse() {
        for value in ["wgpu", "gpu"] {
            let options = LauncherOptions {
                renderer: value.to_string(),
                ..LauncherOptions::default()
            };
            assert_eq!(
                options.resolved_renderer_kind(),
                crate::RendererKind::Wgpu,
                "{value}"
            );
        }
        for value in ["software", "cpu"] {
            let options = LauncherOptions {
                renderer: value.to_string(),
                ..LauncherOptions::default()
            };
            assert_eq!(
                options.resolved_renderer_kind(),
                crate::RendererKind::Software,
                "{value}"
            );
        }
        // Every value RendererKind::parse accepts resolves identically.
        for value in ["wgpu", "gpu", "software", "cpu", "WGPU", "Software"] {
            let options = LauncherOptions {
                renderer: value.to_string(),
                ..LauncherOptions::default()
            };
            assert_eq!(
                options.resolved_renderer_kind(),
                crate::RendererKind::parse(value).expect("parse accepts"),
                "{value}"
            );
        }
        // Invalid or empty persisted values fall back to wgpu, never an error.
        for value in ["bogus", "", "   "] {
            let options = LauncherOptions {
                renderer: value.to_string(),
                ..LauncherOptions::default()
            };
            assert_eq!(
                options.resolved_renderer_kind(),
                crate::RendererKind::Wgpu,
                "{value}"
            );
        }
    }

    #[test]
    fn apply_to_mixer_pushes_every_volume_bus() {
        let mixer = cnc_audio::AudioMixer::headless();
        let options = LauncherOptions {
            music_volume: 0.25,
            effects_volume: 0.5,
            master_volume: 0.75,
            ..LauncherOptions::default()
        };
        options.apply_to_mixer(&mixer);
        assert_eq!(mixer.music_volume(), 0.25);
        assert_eq!(mixer.effects_volume(), 0.5);
        assert_eq!(mixer.master_volume(), 0.75);
    }

    // --- persistence: the campaign-progress INI idiom ---

    #[test]
    fn loaded_volumes_clamp_into_range() {
        let dir = scratch("clamp");
        let _guard = ScratchGuard(dir.clone());
        let path = dir.join("launcher-options.ini");
        std::fs::write(
            &path,
            "[Options]\nMusicVolume=5.0\nEffectsVolume=-2.0\nMasterVolume=not-a-number\n",
        )
        .expect("write options");
        let options = LauncherOptions::load_from(&path);
        assert_eq!(options.music_volume, 1.0);
        assert_eq!(options.effects_volume, 0.0);
        assert_eq!(options.master_volume, 1.0);
    }

    #[test]
    fn save_and_load_round_trip_preserves_settings() {
        let dir = scratch("round-trip");
        let _guard = ScratchGuard(dir.clone());
        let path = dir.join("nested").join("launcher-options.ini");
        let options = LauncherOptions {
            renderer: "software".to_string(),
            music_volume: 0.25,
            effects_volume: 0.5,
            master_volume: 0.75,
            game_speed: 1.0,
            build_threshold: 0,
            shroud_off: false,
            one_tick_strikes: false,
            controls: LauncherOptions::default_controls(),
        };
        options.save_to(&path).expect("save creates parent dirs");
        assert_eq!(LauncherOptions::load_from(&path), options);
    }

    #[test]
    fn corrupt_file_falls_back_to_defaults() {
        let dir = scratch("corrupt");
        let _guard = ScratchGuard(dir.clone());
        let path = dir.join("launcher-options.ini");
        std::fs::write(&path, "\u{feff}not an ini at all\x00\x01").expect("write options");
        assert_eq!(
            LauncherOptions::load_from(&path),
            LauncherOptions::default()
        );
    }

    #[test]
    fn unknown_keys_are_ignored() {
        let dir = scratch("unknown-keys");
        let _guard = ScratchGuard(dir.clone());
        let path = dir.join("launcher-options.ini");
        std::fs::write(
            &path,
            "[Options]\nRenderer = gpu\nMusicVolume=0.5\nUnknown=42\nFullscreen=true\n",
        )
        .expect("write options");
        let options = LauncherOptions::load_from(&path);
        assert_eq!(options.renderer, "gpu");
        assert_eq!(options.music_volume, 0.5);
        // Unknown keys never break the load.
        assert_eq!(options.effects_volume, 1.0);
    }

    #[test]
    fn missing_file_falls_back_to_defaults() {
        let dir = scratch("missing");
        let _guard = ScratchGuard(dir.clone());
        let options = LauncherOptions::load_from(&dir.join("does-not-exist.ini"));
        assert_eq!(options, LauncherOptions::default());
    }

    // --- path resolution ---

    #[test]
    fn path_precedence_cli_over_env_over_save_dir() {
        // An explicit --options path wins over every environment variable.
        assert_eq!(
            LauncherOptions::resolve_options_path(
                Some(Path::new("/nonexistent/cli.ini")),
                Some("/nonexistent/env.ini"),
                Some("/nonexistent/save-dir")
            ),
            PathBuf::from("/nonexistent/cli.ini")
        );
        // CNC_OPTIONS_FILE wins over the save directory.
        assert_eq!(
            LauncherOptions::resolve_options_path(
                None,
                Some("/nonexistent/env.ini"),
                Some("/nonexistent/save-dir")
            ),
            PathBuf::from("/nonexistent/env.ini")
        );
        // CNC_SAVE_DIR/launcher-options.ini mirrors the campaign-progress
        // path (main.rs default_campaign_progress_path).
        assert_eq!(
            LauncherOptions::resolve_options_path(None, None, Some("/nonexistent/save-dir")),
            PathBuf::from("/nonexistent/save-dir").join("launcher-options.ini")
        );
        // ./saves default when nothing is set.
        assert_eq!(
            LauncherOptions::resolve_options_path(None, None, None),
            PathBuf::from("saves").join("launcher-options.ini")
        );
    }

    #[test]
    fn env_override_selects_the_options_file() {
        // Process-wide environment edits are serialized inside this test
        // binary so parallel tests never observe a half-applied override.
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = ENV_LOCK.lock().unwrap_or_else(|poison| poison.into_inner());
        let dir = scratch("env-override");
        let _guard = ScratchGuard(dir.clone());
        let file = dir.join("custom.ini");
        std::fs::write(&file, "[Options]\nRenderer=software\n").expect("write options");

        let previous_file = std::env::var_os(OPTIONS_FILE_ENV);
        let previous_dir = std::env::var_os("CNC_SAVE_DIR");
        std::env::set_var(OPTIONS_FILE_ENV, &file);
        std::env::remove_var("CNC_SAVE_DIR");
        let resolved = LauncherOptions::options_path(None);
        // Restore the process environment for the other tests.
        match previous_file {
            Some(value) => std::env::set_var(OPTIONS_FILE_ENV, value),
            None => std::env::remove_var(OPTIONS_FILE_ENV),
        }
        match previous_dir {
            Some(value) => std::env::set_var("CNC_SAVE_DIR", value),
            None => std::env::remove_var("CNC_SAVE_DIR"),
        }
        assert_eq!(resolved, file);
        assert_eq!(
            LauncherOptions::load_from(&resolved).renderer,
            "software".to_string()
        );
    }

    // --- controls: read-only hotkey display list ---

    #[test]
    fn controls_list_covers_the_hardcoded_hotkeys() {
        let controls = LauncherOptions::default_controls();
        let find = |action: &str, key: &str| {
            controls
                .iter()
                .any(|(entry_action, entry_key)| entry_action == action && entry_key == key)
        };
        // TD / RA quick save and load (TD main.rs ~4405-4424, RA main.rs ~7586).
        assert!(find("Quick save", "F5"));
        assert!(find("Quick load", "F9"));
        // Zoom and game speed (TD main.rs ~4425-4449, RA main.rs ~7594-7607).
        assert!(find("Zoom in", "+ / ="));
        assert!(find("Game speed up", "F7"));
        // Menu navigation (menu.rs on_key ~331-347).
        assert!(find("Back / Quit", "Escape"));
        // Deterministic: repeated calls return the same ordered list.
        assert_eq!(LauncherOptions::default_controls(), controls);
    }

    // --- test scaffolding ---

    /// Scratch directory for this test, rooted inside the workspace
    /// (`RUST/target/scratch-options/`) so no test ever writes outside the
    /// project tree.
    fn scratch(tag: &str) -> PathBuf {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../target/scratch-options")
            .join(format!("{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create scratch dir");
        dir
    }

    struct ScratchGuard(PathBuf);

    impl Drop for ScratchGuard {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn options_apply_to_running_game() {
        let dir = scratch("apply-running");
        let _guard = ScratchGuard(dir.clone());
        let path = dir.join("launcher-options.ini");

        // Create options with game speed and special toggles
        let options = LauncherOptions {
            renderer: "software".to_string(),
            music_volume: 0.5,
            effects_volume: 0.75,
            master_volume: 0.8,
            game_speed: 1.5,
            build_threshold: 5,
            shroud_off: true,
            one_tick_strikes: false,
            controls: LauncherOptions::default_controls(),
        };
        options.save_to(&path).expect("save options");

        // Load options in new session
        let loaded = LauncherOptions::load_from(&path);

        // Verify persistence
        assert_eq!(loaded.game_speed, 1.5);
        assert_eq!(loaded.build_threshold, 5);
        assert!(loaded.shroud_off);
        assert!(!loaded.one_tick_strikes);

        // Apply to running game
        let app = LauncherApp::new(
            RendererKind::Software,
            320,
            240,
            RulesDatabase::default(),
            loaded,
        )
        .expect("create app");

        // Game speed should be applied from options
        assert_eq!(app.game_speed, 1.5);

        // Volume should be applied to mixer
        let mixer = cnc_audio::AudioMixer::headless();
        options.apply_to_mixer(&mixer);
        assert_eq!(mixer.music_volume(), 0.5);
        assert_eq!(mixer.effects_volume(), 0.75);
        assert_eq!(mixer.master_volume(), 0.8);
    }
}
