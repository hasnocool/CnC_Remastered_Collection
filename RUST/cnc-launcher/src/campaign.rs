//! Original-campaign progression and carry-over (plan todo 13, TD half).
//!
//! Pure orchestration, no window and no renderer, so both the launcher shell
//! and the tests can drive it:
//!
//! 1. **Listing** - [`discover_campaign`] presents the scenarios in a
//!    `CNC_ORIGINAL_DATA_DIR` install (the same variable the todo 8/9 loader
//!    reads) as an ordered campaign. Non-mission INIs are skipped with a
//!    reason, never fatal, and the order is the campaign order (`01`, `02`,
//!    ...), so the list is byte-stable.
//! 2. **Progression** - [`CampaignFlow`] maps a finished mission onto exactly
//!    one step: advance to the next mission (with the score report the shell
//!    shows), retry the same mission after a loss, or open the completion
//!    screen when nothing is left - including when the next mission's file has
//!    gone missing.
//! 3. **Carry-over** - a victory writes the surviving units and treasury
//!    through `cnc-tiberian-dawn`'s [`Carryover`] (`CNCCARRY`, FNV-1a
//!    checksummed) at mission end and loads them back at the next mission's
//!    start. Exactly one carry-over is ever applied per mission: a replayed
//!    mission starts clean from the scenario, and a corrupt file is refused
//!    with a message instead of panicking or writing garbage.

use std::path::{Path, PathBuf};

use cnc_tiberian_dawn::Carryover;

/// The env var the todo 8/9 original-campaign loader reads.
pub use crate::original_campaign::DATA_DIR_ENV as ORIGINAL_DATA_ENV;
use crate::score::{compute_score, ScoreReport, DIFFICULTY_NEUTRAL_BP};
use cnc_tiberian_dawn::scenario_loader;

/// Carry-over file name written next to the campaign progress file.
pub const CARRYOVER_FILE_NAME: &str = "td-carryover.bin";

/// A mission in a campaign, in play order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CampaignMission {
    /// The scenario INI to hand to `Game::load_mission`.
    pub path: PathBuf,
    /// Position in the campaign (1-based, from the file name).
    pub order: u32,
    /// `[Basic] Name` when readable, else the file name.
    pub name: String,
    /// True for original-format scenarios (the `[Basic]` + `[Wunits]` dialect),
    /// which the shell fields through the todo 8/9 loader.
    pub original: bool,
}

/// Outcome of one discovery pass. Never fatal: an unusable directory yields an
/// empty list plus a human-readable `reason`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CampaignDiscovery {
    /// Directory that was inspected, when one was configured.
    pub root: Option<PathBuf>,
    /// Missions in campaign order.
    pub missions: Vec<CampaignMission>,
    /// Files that were considered and rejected, with the reason.
    pub skipped: Vec<(PathBuf, String)>,
    /// Why the list came back empty, when it did.
    pub reason: Option<String>,
}

impl CampaignDiscovery {
    /// True when the pass found at least one playable mission.
    pub fn is_empty(&self) -> bool {
        self.missions.is_empty()
    }
}

/// What the shell must do when a mission ends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlowStep {
    /// The next mission is selected and the score screen shows this report.
    Advance { next: usize, score: ScoreReport },
    /// A loss keeps the same mission selected (retry) and shows the score.
    Retry { index: usize, score: ScoreReport },
    /// Nothing is left to play - open the completion screen with this score.
    Complete { score: ScoreReport },
}

/// Campaign position, order and carry-over bookkeeping.
pub struct CampaignFlow {
    missions: Vec<CampaignMission>,
    index: usize,
    done: Vec<bool>,
    /// Carry-over file location; `None` keeps the flow in-memory only.
    carryover_path: Option<PathBuf>,
    /// Carry-over waiting to be applied to the next mission.
    pending: Option<Carryover>,
    /// True once `pending`/file carry-over has been handed out, so a replay
    /// does not apply it twice.
    applied: bool,
    /// Last degrade reason (unreadable carry-over, unwritable file, ...).
    last_error: Option<String>,
    /// True once the campaign reached the completion screen.
    complete: bool,
}

impl CampaignFlow {
    /// A flow over explicit missions (authored directories, saved campaigns).
    pub fn from_missions(missions: Vec<CampaignMission>, carryover_path: Option<PathBuf>) -> Self {
        let len = missions.len();
        Self {
            missions,
            index: 0,
            done: vec![false; len],
            carryover_path,
            pending: None,
            applied: false,
            last_error: None,
            complete: false,
        }
    }

    /// A flow over explicit mission paths, numbered in the given order (the
    /// authored-campaign path: no `[Basic] Name` lookup, file-name display).
    pub fn from_paths<I, P>(paths: I, carryover_path: Option<PathBuf>) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let missions = paths
            .into_iter()
            .enumerate()
            .map(|(index, path)| {
                let path = path.as_ref().to_path_buf();
                let name = mission_name(&path).unwrap_or_else(|| file_stem(&path));
                CampaignMission {
                    path,
                    order: index as u32 + 1,
                    name,
                    original: false,
                }
            })
            .collect();
        Self::from_missions(missions, carryover_path)
    }

    /// Number of missions in the campaign.
    pub fn len(&self) -> usize {
        self.missions.len()
    }

    /// True when the campaign has no missions at all.
    pub fn is_empty(&self) -> bool {
        self.missions.is_empty()
    }

    /// Currently selected mission.
    pub fn current(&self) -> Option<&CampaignMission> {
        self.missions.get(self.index)
    }

    /// All missions, in campaign order.
    pub fn missions(&self) -> &[CampaignMission] {
        &self.missions
    }

    /// Index of the currently selected mission.
    pub fn index(&self) -> usize {
        self.index
    }

    /// True once the campaign reached the completion screen.
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Where this flow reads and writes carry-over data.
    pub fn carryover_path(&self) -> Option<&Path> {
        self.carryover_path.as_deref()
    }

    /// Last degrade reason, kept for observability (never fatal).
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    /// Mission end: record the victory, persist the carry-over, and decide the
    /// next step. `owner` is the house whose forces and treasury carry over.
    pub fn finish_mission(
        &mut self,
        game: &cnc_tiberian_dawn::Game,
        owner: u8,
        won: bool,
    ) -> FlowStep {
        let score = mission_score(game, won);
        if !won {
            // A loss replays the same mission; nothing carries over.
            self.pending = None;
            return FlowStep::Retry {
                index: self.index,
                score,
            };
        }
        if let Some(slot) = self.done.get_mut(self.index) {
            *slot = true;
        }
        let carried = Carryover::from_game(game, owner);
        if let Some(path) = self.carryover_path.clone() {
            match carried.save_to_file(&path) {
                Ok(()) => {
                    self.last_error = None;
                    tracing::info!(path = %path.display(), "carry-over saved");
                }
                Err(error) => {
                    let message = format!("{error:#}");
                    self.last_error = Some(message.clone());
                    tracing::warn!(path = %path.display(), %message, "failed to save carry-over");
                }
            }
        }
        self.pending = Some(carried);
        self.applied = false;

        let paths: Vec<PathBuf> = self.missions.iter().map(|m| m.path.clone()).collect();
        match next_after(&self.done, self.index, &paths) {
            Some(next) => {
                self.index = next;
                FlowStep::Advance { next, score }
            }
            None => {
                self.complete = true;
                FlowStep::Complete { score }
            }
        }
    }

    /// Start the next mission: hand out the carry-over to apply, exactly once.
    ///
    /// `None` means "nothing carried over" - either there is nothing to carry
    /// (first mission, a loss), the carry-over was already applied (replay), or
    /// the file is unreadable. A corrupt file is never fatal and never
    /// rewritten as garbage.
    pub fn begin_next(&mut self) -> Option<Carryover> {
        if self.applied {
            return None;
        }
        self.applied = true;
        if let Some(pending) = self.pending.take() {
            return Some(pending);
        }
        let path = self.carryover_path.clone()?;
        match Carryover::load_from_file(&path) {
            Ok(carryover) => Some(carryover),
            Err(error) => {
                let message = format!("{error:#}");
                self.last_error = Some(message.clone());
                tracing::warn!(path = %path.display(), %message, "carry-over unreadable; starting clean");
                None
            }
        }
    }

    /// Read a carry-over file without applying it (used when a mission selector
    /// is (re)opened), degrading to `None` on an unreadable file.
    pub fn load_carryover(path: &Path) -> Option<Carryover> {
        match Carryover::load_from_file(path) {
            Ok(carryover) => Some(carryover),
            Err(error) => {
                tracing::warn!(path = %path.display(), %error, "carry-over unreadable; starting clean");
                None
            }
        }
    }
}

/// First mission after `current` that is not done yet, wrapping around, whose
/// file still exists. `None` when the campaign is over.
///
/// A missing file ends the campaign rather than stranding the player on the
/// selector: a deleted install, an unmounted drive or a partial copy must open
/// the completion screen, not an error.
pub fn next_after(done: &[bool], current: usize, paths: &[PathBuf]) -> Option<usize> {
    let len = done.len();
    if len == 0 {
        return None;
    }
    let start = current.min(len.saturating_sub(1)) + 1;
    (0..len)
        .map(|offset| (start + offset) % len)
        .find(|index| !done.get(*index).copied().unwrap_or(false))
        .filter(|index| paths.get(*index).is_some_and(|path| path.is_file()))
}

/// The scoreboard for a finished mission (same telemetry `score.rs` renders).
fn mission_score(game: &cnc_tiberian_dawn::Game, won: bool) -> ScoreReport {
    compute_score(
        game.harvested_credits,
        game.units_produced,
        game.projectiles_fired,
        game.elapsed.int().max(0) as u64,
        won,
        DIFFICULTY_NEUTRAL_BP,
    )
}

/// `[Basic] Name` of a scenario INI, when the file carries one.
fn mission_name(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let mut in_basic = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_basic = trimmed.eq_ignore_ascii_case("[Basic]");
            continue;
        }
        if !in_basic {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once('=') {
            if key.trim().eq_ignore_ascii_case("Name") {
                let value = value.trim();
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}

/// File name without extension, used for display and for the campaign order.
fn file_stem(path: &Path) -> String {
    path.file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Campaign order from a file stem: the leading digits of `01 - Hold`
/// (original C&C names scenarios `SCU-01.INI`, `01 - ...`, ...). Files without
/// digits sort last, by path, so the order is still total and byte-stable.
fn campaign_order(stem: &str) -> u32 {
    let digits: String = stem.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().unwrap_or(u32::MAX)
}

/// Present every original-format scenario in `root` as an ordered campaign.
///
/// Non-mission INIs (rules, authored new-dialect missions, notes) are skipped
/// with a reason; unreadable files likewise. The pass never aborts.
pub fn discover_campaign(root: &Path) -> CampaignDiscovery {
    let mut discovery = CampaignDiscovery {
        root: Some(root.to_path_buf()),
        ..CampaignDiscovery::default()
    };
    for path in crate::original_campaign::discover_td_scenarios(root) {
        let Some(text) = read_lossy(&path, &mut discovery) else {
            continue;
        };
        if !scenario_loader::looks_like_original_scenario(&text) {
            discovery.skipped.push((
                path,
                "not an original-format scenario (no [Basic] + [Wunits])".to_string(),
            ));
            continue;
        }
        // `load_scenario` is the production validating path; a scenario that
        // will not load is reported and skipped, never fatal.
        if let Err(error) = scenario_loader::load_scenario(&path) {
            discovery
                .skipped
                .push((path, format!("scenario loader rejected it: {error:#}")));
            continue;
        }
        let name = mission_name(&path).unwrap_or_else(|| file_stem(&path));
        discovery.missions.push(CampaignMission {
            order: campaign_order(&file_stem(&path)),
            path,
            name,
            original: true,
        });
    }
    // Byte-stable order: campaign number first, then path.
    discovery
        .missions
        .sort_by(|a, b| a.order.cmp(&b.order).then_with(|| a.path.cmp(&b.path)));
    if discovery.missions.is_empty() {
        discovery.reason = Some(format!(
            "no original Tiberian Dawn scenarios found in {}",
            root.display()
        ));
    }
    discovery
}

/// The env-gated original campaign, discovered from `CNC_ORIGINAL_DATA_DIR`
/// (the todo 8/9 loader's variable). Unset or unusable configuration degrades to
/// an empty list plus a reason - never an error, never a panic.
pub fn discover_from_env() -> CampaignDiscovery {
    let Some(root) = std::env::var_os(ORIGINAL_DATA_ENV).filter(|raw| !raw.is_empty()) else {
        return CampaignDiscovery {
            reason: Some(format!("{ORIGINAL_DATA_ENV} is not set")),
            ..CampaignDiscovery::default()
        };
    };
    let root = PathBuf::from(root);
    if !root.is_dir() {
        return CampaignDiscovery {
            reason: Some(format!("{} does not exist", root.display())),
            ..CampaignDiscovery::default()
        };
    }
    let mut discovery = discover_campaign(&root);
    if discovery.missions.is_empty() {
        discovery.reason = Some(format!(
            "{ORIGINAL_DATA_ENV} ({}) has no original Tiberian Dawn scenarios",
            root.display()
        ));
    }
    discovery
}

/// Read a scenario INI, recording unreadable files as skipped.
fn read_lossy(path: &Path, discovery: &mut CampaignDiscovery) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(text) => Some(text),
        Err(error) => {
            discovery
                .skipped
                .push((path.to_path_buf(), error.to_string()));
            None
        }
    }
}
