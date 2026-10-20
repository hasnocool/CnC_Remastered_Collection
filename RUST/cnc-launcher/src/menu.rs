//! Main menu front-end for the launcher.
//!
//! The menu is a pure state machine (no window or renderer ownership) so it can
//! be exercised by unit tests and driven by the window shell in `main.rs`.
//! It offers the three start-game paths requested by the launcher milestone:
//!
//! - **Campaign** - pick Tiberian Dawn or Red Alert; the game then opens its
//!   scenario selector loaded from the matching campaign directory.
//! - **Local Game** - start a single-player skirmish in either engine.
//! - **Online Game** - open the Red Alert host/join network lobby.
//!
//! The main screen also carries **Options** (the video/audio/controls modal,
//! owned by this module) and Quit. Every screen also has a Back entry, and the
//! main screen has Quit.

use std::path::{Path, PathBuf};

use cnc_renderer::Renderer;
use winit::keyboard::{Key, NamedKey};

/// Environment override for the Tiberian Dawn campaign directory.
pub(crate) const TD_CAMPAIGN_ENV: &str = "CNC_TD_CAMPAIGN";
/// Environment override for the Red Alert campaign directory.
pub(crate) const RA_CAMPAIGN_ENV: &str = "CNC_RA_CAMPAIGN";

const ITEM_X: f32 = 60.0;
const ITEM_TOP: f32 = 224.0;
const ITEM_HEIGHT: f32 = 48.0;
const ITEM_GAP: f32 = 10.0;
const MAX_ITEM_WIDTH: f32 = 560.0;

/// Which engine a start-game choice refers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameChoice {
    TiberianDawn,
    RedAlert,
}

impl GameChoice {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::TiberianDawn => "Tiberian Dawn",
            Self::RedAlert => "Red Alert",
        }
    }

    pub(crate) fn campaign_env(self) -> &'static str {
        match self {
            Self::TiberianDawn => TD_CAMPAIGN_ENV,
            Self::RedAlert => RA_CAMPAIGN_ENV,
        }
    }

    fn campaign_folder(self) -> &'static str {
        match self {
            Self::TiberianDawn => "td_campaign",
            Self::RedAlert => "ra_campaign",
        }
    }
}

/// A discovered campaign directory and its scenario count. Scenario names,
/// briefings and progress live in each game's in-game scenario selector; the
/// briefing video overlay (draw_briefing_video) renders mission description, credits,
/// and key points via renderer.draw_text() in main.rs.
/// menu only reports whether a library was found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CampaignInfo {
    pub(crate) dir: PathBuf,
    pub(crate) scenarios: usize,
}

impl CampaignInfo {
    /// Fallback used when no campaign directory could be discovered.
    pub(crate) fn empty(game: GameChoice) -> Self {
        Self {
            dir: default_campaign_dir(game),
            scenarios: 0,
        }
    }

    pub(crate) fn scenario_count(&self) -> usize {
        self.scenarios
    }

    fn summary(&self, game: GameChoice) -> String {
        if self.scenarios == 0 {
            format!("No scenario INIs found - set {}", game.campaign_env())
        } else {
            format!(
                "{} scenario{} - opens the in-game selector",
                self.scenarios,
                if self.scenarios == 1 { "" } else { "s" }
            )
        }
    }
}

/// Where the shell should take the player after the menu is dismissed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchTarget {
    /// Open the in-game scenario selector for the given engine.
    Campaign(GameChoice),
    /// Start a local skirmish in the given engine.
    Local(GameChoice),
    /// Open the network lobby for the given engine (Red Alert only).
    Online(GameChoice),
}

/// Result of a key press or click that the shell must act on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuCommand {
    Launch(LaunchTarget),
    Quit,
    /// Open the Options modal over the menu; the shell seeds it from the
    /// persisted `LauncherOptions` and owns the save/mixer wiring.
    Options,
}

/// The menu page currently displayed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Main,
    Campaign,
    Local,
    Online,
}

impl Screen {
    fn heading(self) -> &'static str {
        match self {
            Self::Main => "MAIN MENU",
            Self::Campaign => "CAMPAIGN",
            Self::Local => "LOCAL GAME",
            Self::Online => "ONLINE GAME",
        }
    }

    fn breadcrumb(self) -> &'static str {
        match self {
            Self::Main => "COMMAND & CONQUER",
            Self::Campaign => "MAIN MENU > CAMPAIGN",
            Self::Local => "MAIN MENU > LOCAL GAME",
            Self::Online => "MAIN MENU > ONLINE GAME",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemAction {
    Open(Screen),
    Back,
    Quit,
    Launch(LaunchTarget),
    /// Open the Options modal (owned by the shell while it is up).
    OpenOptions,
    /// Reserved state for entries that are visible but not launchable yet.
    #[allow(dead_code)]
    Unavailable,
}

struct MenuItem {
    label: String,
    detail: String,
    enabled: bool,
    action: ItemAction,
}

impl MenuItem {
    fn new(
        label: impl Into<String>,
        detail: impl Into<String>,
        enabled: bool,
        action: ItemAction,
    ) -> Self {
        Self {
            label: label.into(),
            detail: detail.into(),
            enabled,
            action,
        }
    }
}

/// The interactive main-menu state machine.
pub struct MainMenu {
    screen: Screen,
    cursor: usize,
    hover: Option<usize>,
    td: CampaignInfo,
    ra: CampaignInfo,
    notice: Option<String>,
}

// Input/notice accessors (`on_pointer`, `on_click`, `set_notice`, ...) are
// wired to the window shell in a follow-up change; keep them until that
// integration lands.
#[allow(dead_code)]
impl MainMenu {
    /// Discover the campaign libraries on disk and build the menu.
    pub fn discover() -> Self {
        Self::from_campaigns(
            discover_campaign(GameChoice::TiberianDawn),
            discover_campaign(GameChoice::RedAlert),
        )
    }

    /// Build the menu from explicit campaign sources (unit-test friendly).
    pub(crate) fn from_campaigns(td: CampaignInfo, ra: CampaignInfo) -> Self {
        Self {
            screen: Screen::Main,
            cursor: 0,
            hover: None,
            td,
            ra,
            notice: None,
        }
    }

    /// Get the current screen.
    pub fn screen(&self) -> Screen {
        self.screen
    }

    #[cfg(test)]
    pub(crate) fn cursor(&self) -> usize {
        self.cursor
    }

    #[cfg(test)]
    pub(crate) fn notice(&self) -> Option<&str> {
        self.notice.as_deref()
    }

    /// Show a one-line status under the menu (used for launch failures).
    pub(crate) fn set_notice(&mut self, notice: impl Into<String>) {
        self.notice = Some(notice.into());
    }

    pub(crate) fn campaign(&self, game: GameChoice) -> &CampaignInfo {
        match game {
            GameChoice::TiberianDawn => &self.td,
            GameChoice::RedAlert => &self.ra,
        }
    }

    /// Return to the main page (used when a game hands control back).
    pub(crate) fn reset(&mut self) {
        self.screen = Screen::Main;
        self.cursor = 0;
        self.hover = None;
        self.notice = None;
    }

    fn back(&mut self) {
        self.screen = Screen::Main;
        self.cursor = 0;
        self.hover = None;
        self.notice = None;
    }

    fn items(&self) -> Vec<MenuItem> {
        match self.screen {
            Screen::Main => vec![
                MenuItem::new(
                    "Campaign",
                    "Story missions with scenario and difficulty selection",
                    true,
                    ItemAction::Open(Screen::Campaign),
                ),
                MenuItem::new(
                    "Local Game",
                    "Skirmish against the computer",
                    true,
                    ItemAction::Open(Screen::Local),
                ),
                MenuItem::new(
                    "Online Game",
                    "Host or join a network match",
                    true,
                    ItemAction::Open(Screen::Online),
                ),
                MenuItem::new(
                    "Options",
                    "Video, audio and controls",
                    true,
                    ItemAction::OpenOptions,
                ),
                MenuItem::new("Quit", "Leave the launcher", true, ItemAction::Quit),
            ],
            Screen::Campaign => vec![
                MenuItem::new(
                    "Tiberian Dawn Campaign",
                    self.td.summary(GameChoice::TiberianDawn),
                    self.td.scenario_count() > 0,
                    ItemAction::Launch(LaunchTarget::Campaign(GameChoice::TiberianDawn)),
                ),
                MenuItem::new(
                    "Red Alert Campaign",
                    self.ra.summary(GameChoice::RedAlert),
                    self.ra.scenario_count() > 0,
                    ItemAction::Launch(LaunchTarget::Campaign(GameChoice::RedAlert)),
                ),
                MenuItem::new("Back", "Return to the main menu", true, ItemAction::Back),
            ],
            Screen::Local => vec![
                MenuItem::new(
                    "Tiberian Dawn - Local Skirmish",
                    "GDI vs Nod on the demo battlefield",
                    true,
                    ItemAction::Launch(LaunchTarget::Local(GameChoice::TiberianDawn)),
                ),
                MenuItem::new(
                    "Red Alert - Local Skirmish",
                    "Allies vs Soviet; map size, country and AI difficulty",
                    true,
                    ItemAction::Launch(LaunchTarget::Local(GameChoice::RedAlert)),
                ),
                MenuItem::new("Back", "Return to the main menu", true, ItemAction::Back),
            ],
            Screen::Online => vec![
                MenuItem::new(
                    "Red Alert - Network Match",
                    "Host or join over UDP / LAN with the built-in lobby",
                    true,
                    ItemAction::Launch(LaunchTarget::Online(GameChoice::RedAlert)),
                ),
                MenuItem::new("Back", "Return to the main menu", true, ItemAction::Back),
            ],
        }
    }

    fn move_cursor(&mut self, delta: i32) {
        let len = self.items().len();
        if len == 0 {
            return;
        }
        let len = len as i32;
        self.cursor = (((self.cursor as i32 + delta) % len + len) % len) as usize;
        self.notice = None;
    }

    /// Handle a pressed key.
    pub fn on_key(&mut self, key: &Key) -> Option<MenuCommand> {
        match key {
            Key::Named(NamedKey::ArrowUp) => {
                self.move_cursor(-1);
                None
            }
            Key::Named(NamedKey::ArrowDown) => {
                self.move_cursor(1);
                None
            }
            Key::Named(NamedKey::Enter) => self.activate(),
            Key::Named(NamedKey::Escape) => {
                if self.screen == Screen::Main {
                    Some(MenuCommand::Quit)
                } else {
                    self.back();
                    None
                }
            }
            Key::Character(text) => {
                match text.to_ascii_lowercase().as_str() {
                    "w" => {
                        self.move_cursor(-1);
                    }
                    "s" => {
                        self.move_cursor(1);
                    }
                    _ => {}
                }
                None
            }
            _ => None,
        }
    }

    /// Activate the highlighted entry.
    pub fn activate(&mut self) -> Option<MenuCommand> {
        let items = self.items();
        let item = items.get(self.cursor)?;
        if !item.enabled {
            self.notice = Some(self.disabled_notice(item.action));
            return None;
        }
        match item.action {
            ItemAction::Open(screen) => {
                self.screen = screen;
                self.cursor = 0;
                self.hover = None;
                self.notice = None;
                None
            }
            ItemAction::Back => {
                self.back();
                None
            }
            ItemAction::Quit => Some(MenuCommand::Quit),
            ItemAction::Launch(target) => Some(MenuCommand::Launch(target)),
            ItemAction::OpenOptions => Some(MenuCommand::Options),
            ItemAction::Unavailable => None,
        }
    }

    fn disabled_notice(&self, action: ItemAction) -> String {
        match action {
            ItemAction::Launch(LaunchTarget::Campaign(game)) => format!(
                "No {} scenarios found. Set {} to a directory of scenario INIs.",
                game.label(),
                game.campaign_env()
            ),
            _ => "This option is not available in this build.".to_string(),
        }
    }

    /// Update the hovered row from a pointer position.
    pub(crate) fn on_pointer(&mut self, x: f32, y: f32, width: u32) -> bool {
        let hovered = self
            .items()
            .iter()
            .enumerate()
            .find(|(index, _)| {
                let (ix, iy, iw, ih) = item_rect(*index, width);
                x >= ix && x <= ix + iw && y >= iy && y <= iy + ih
            })
            .map(|(index, _)| index);
        if hovered != self.hover {
            self.hover = hovered;
            true
        } else {
            false
        }
    }

    /// Handle a left click; clicks outside the rows are ignored.
    pub(crate) fn on_click(&mut self, x: f32, y: f32, width: u32) -> Option<MenuCommand> {
        let items = self.items();
        let clicked = items.iter().enumerate().find(|(index, _)| {
            let (ix, iy, iw, ih) = item_rect(*index, width);
            x >= ix && x <= ix + iw && y >= iy && y <= iy + ih
        })?;
        self.cursor = clicked.0;
        self.activate()
    }

    /// Draw the current page.
    pub fn draw(&self, renderer: &mut dyn Renderer, width: u32, height: u32) {
        let w = width as f32;
        let h = height as f32;
        renderer.draw_rect(0.0, 0.0, w, h, [0.02, 0.03, 0.06, 1.0]);
        renderer.draw_rect(0.0, 0.0, w, 4.0, [0.82, 0.42, 0.16, 1.0]);
        renderer.draw_rect(0.0, h - 4.0, w, 4.0, [0.10, 0.18, 0.32, 1.0]);

        renderer.draw_text(
            "COMMAND & CONQUER",
            56.0,
            58.0,
            40.0,
            [0.96, 0.83, 0.46, 1.0],
        );
        renderer.draw_text(
            "REMASTERED COLLECTION",
            60.0,
            110.0,
            12.0,
            [0.62, 0.74, 0.90, 1.0],
        );
        renderer.draw_text("RUST EDITION", 60.0, 130.0, 8.0, [0.40, 0.50, 0.64, 1.0]);

        let breadcrumb = self.screen.breadcrumb();
        let crumb_size = 9.0;
        let crumb_width = text_width(breadcrumb, crumb_size);
        renderer.draw_text(
            breadcrumb,
            (w - 60.0 - crumb_width).max(60.0),
            74.0,
            crumb_size,
            [0.42, 0.52, 0.68, 1.0],
        );

        renderer.draw_text(
            self.screen.heading(),
            60.0,
            176.0,
            17.0,
            [0.90, 0.93, 0.98, 1.0],
        );
        let rule_width = content_width(width);
        renderer.draw_rect(60.0, 200.0, rule_width, 1.0, [0.20, 0.30, 0.45, 1.0]);

        for (index, item) in self.items().iter().enumerate() {
            let (x, y, iw, ih) = item_rect(index, width);
            let selected = index == self.cursor;
            let hovered = selected || self.hover == Some(index);
            if hovered {
                let fill = if selected {
                    [0.13, 0.19, 0.29, 1.0]
                } else {
                    [0.07, 0.10, 0.16, 1.0]
                };
                renderer.draw_rect(x, y, iw, ih, fill);
            }
            let accent = if selected {
                [0.90, 0.55, 0.20, 1.0]
            } else if item.enabled {
                [0.25, 0.34, 0.48, 1.0]
            } else {
                [0.16, 0.19, 0.24, 1.0]
            };
            renderer.draw_rect(x, y, 3.0, ih, accent);
            let label_color = if !item.enabled {
                [0.42, 0.46, 0.54, 1.0]
            } else if selected {
                [1.0, 0.88, 0.55, 1.0]
            } else {
                [0.82, 0.88, 0.96, 1.0]
            };
            renderer.draw_text(&item.label, x + 16.0, y + 7.0, 15.0, label_color);
            let detail_color = if item.enabled {
                [0.55, 0.64, 0.77, 1.0]
            } else {
                [0.36, 0.40, 0.47, 1.0]
            };
            renderer.draw_text(&item.detail, x + 16.0, y + 29.0, 8.0, detail_color);
        }

        self.draw_library_panel(renderer, width, h);

        renderer.draw_text(
            "Up/Down: select   Enter: confirm   Esc: back / quit   Mouse: click",
            60.0,
            h - 52.0,
            8.0,
            [0.45, 0.55, 0.70, 1.0],
        );
        if let Some(notice) = &self.notice {
            renderer.draw_text(notice, 60.0, h - 32.0, 8.0, [1.0, 0.72, 0.40, 1.0]);
        } else {
            renderer.draw_text(
                "Campaign missions, briefings and progress live in each game's scenario selector.",
                60.0,
                h - 32.0,
                8.0,
                [0.34, 0.42, 0.55, 1.0],
            );
        }
    }

    fn draw_library_panel(&self, renderer: &mut dyn Renderer, width: u32, h: f32) {
        let w = width as f32;
        if w < 900.0 {
            return;
        }
        let panel_w = 280.0;
        let panel_x = w - 60.0 - panel_w;
        let panel_y = 224.0;
        let panel_h = (h - panel_y - 90.0).max(180.0);
        renderer.draw_rect(panel_x, panel_y, panel_w, panel_h, [0.04, 0.06, 0.10, 1.0]);
        renderer.draw_rect(panel_x, panel_y, panel_w, 2.0, [0.35, 0.50, 0.70, 1.0]);
        renderer.draw_text(
            "MISSION LIBRARY",
            panel_x + 18.0,
            panel_y + 16.0,
            10.0,
            [0.70, 0.82, 0.96, 1.0],
        );
        let rows = [
            ("Tiberian Dawn", self.td.scenarios, self.td.scenarios > 0),
            ("Red Alert", self.ra.scenarios, self.ra.scenarios > 0),
        ];
        let mut y = panel_y + 48.0;
        for (name, scenarios, found) in rows {
            renderer.draw_text(name, panel_x + 18.0, y, 11.0, [0.86, 0.90, 0.97, 1.0]);
            let value = if found {
                format!(
                    "{scenarios} scenario{}",
                    if scenarios == 1 { "" } else { "s" }
                )
            } else {
                "not found".to_string()
            };
            renderer.draw_text(
                &value,
                panel_x + 18.0,
                y + 18.0,
                8.0,
                if found {
                    [0.55, 0.75, 0.60, 1.0]
                } else {
                    [0.70, 0.52, 0.42, 1.0]
                },
            );
            y += 58.0;
        }
        renderer.draw_text(
            "Online",
            panel_x + 18.0,
            y + 6.0,
            11.0,
            [0.86, 0.90, 0.97, 1.0],
        );
        renderer.draw_text(
            "Red Alert LAN - host / join",
            panel_x + 18.0,
            y + 24.0,
            8.0,
            [0.55, 0.70, 0.86, 1.0],
        );
        renderer.draw_text(
            "Tiberian Dawn - unavailable",
            panel_x + 18.0,
            y + 40.0,
            8.0,
            [0.60, 0.46, 0.42, 1.0],
        );
        renderer.draw_text(
            "Campaign selections resume saved progress.",
            panel_x + 18.0,
            panel_y + panel_h - 30.0,
            7.0,
            [0.38, 0.46, 0.58, 1.0],
        );
    }
}

/// Renderer choice offered by the Options modal; labels mirror the
/// `--renderer` CLI values and the shell maps them to `RendererKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RendererChoice {
    Wgpu,
    Software,
}

impl RendererChoice {
    pub fn label(self) -> &'static str {
        match self {
            Self::Wgpu => "Wgpu",
            Self::Software => "Software",
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Wgpu => Self::Software,
            Self::Software => Self::Wgpu,
        }
    }
}

/// Audio bus of a volume row; mirrors the cnc-audio mixer buses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VolumeBus {
    Master,
    Music,
    Effects,
}

impl VolumeBus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Master => "Master",
            Self::Music => "Music",
            Self::Effects => "Effects",
        }
    }

    fn index(self) -> usize {
        match self {
            Self::Master => 0,
            Self::Music => 1,
            Self::Effects => 2,
        }
    }

    const ALL: [Self; 3] = [Self::Master, Self::Music, Self::Effects];
}

/// One gesture from the Options modal that the shell must apply: a live
/// volume push, a renderer choice, a save or a cancel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptionsAction {
    Renderer(RendererChoice),
    Volume(VolumeBus, f32),
    Save,
    Cancel,
}

/// Rows the modal cursor can focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OptionsRow {
    Renderer,
    Volume(VolumeBus),
    Save,
    Cancel,
}

const OPTIONS_ROWS: [OptionsRow; 6] = [
    OptionsRow::Renderer,
    OptionsRow::Volume(VolumeBus::Master),
    OptionsRow::Volume(VolumeBus::Music),
    OptionsRow::Volume(VolumeBus::Effects),
    OptionsRow::Save,
    OptionsRow::Cancel,
];

/// One rendered line of the read-only controls list: a game group header or
/// an (action, key) entry with the group prefix stripped for display.
#[derive(Clone)]
enum ControlsRow {
    Group(&'static str),
    Entry { action: String, key: String },
}

/// Volume nudge per key press or arrow click (0..=1 scale).
const VOLUME_STEP: f32 = 0.05;
/// Row-end zones that step a value instead of just focusing the row.
const OPTIONS_ARROW_ZONE: f32 = 24.0;
/// Controls-list line height and its scroll amounts.
const CONTROLS_ROW_H: f32 = 15.0;
const CONTROLS_WHEEL_ROWS: i32 = 3;
const CONTROLS_PAGE_ROWS: i32 = 6;
/// Right-edge clearance for the controls list's key column, measured from the
/// list rect's right edge. The 9 px font advances 11.25 px per glyph
/// (`text_width`), so 24 px leaves the binding column more than two glyphs
/// clear of the list edge: a visible step inside the panel's 20 px inner
/// margin instead of the 8 px that read as running into the modal's right
/// border at smaller scalings.
const CONTROLS_KEY_GUTTER: f32 = 24.0;

// Modal panel geometry; offsets are relative to the panel's top-left so the
// whole dialog rescales with the window like the menu rows do.
const OPTIONS_PANEL_X: f32 = 60.0;
const OPTIONS_PANEL_Y: f32 = 120.0;
const OPTIONS_VIDEO_Y: f32 = 70.0;
const OPTIONS_RENDERER_Y: f32 = 86.0;
const OPTIONS_RENDERER_H: f32 = 32.0;
const OPTIONS_NOTE_Y: f32 = 122.0;
const OPTIONS_INVALID_NOTE_Y: f32 = 134.0;
const OPTIONS_AUDIO_Y: f32 = 152.0;
const OPTIONS_VOLUME_Y: f32 = 168.0;
const OPTIONS_VOLUME_H: f32 = 46.0;
const OPTIONS_CONTROLS_Y: f32 = 232.0;
const OPTIONS_CONTROLS_SUB_Y: f32 = 248.0;
const OPTIONS_LIST_TOP: f32 = 266.0;
const OPTIONS_BUTTON_H: f32 = 30.0;
const OPTIONS_BUTTON_W: f32 = 104.0;

/// Modal panel rect (x, y, w, h); mirrors the menu's left margin and rules.
fn options_panel_rect(width: u32, height: u32) -> (f32, f32, f32, f32) {
    let w = (width as f32 - 120.0).clamp(560.0, 900.0);
    let h = (height as f32 - 160.0).max(380.0);
    (OPTIONS_PANEL_X, OPTIONS_PANEL_Y, w, h)
}

fn options_renderer_rect(width: u32, height: u32) -> (f32, f32, f32, f32) {
    let (px, py, pw, _) = options_panel_rect(width, height);
    (
        px + 20.0,
        py + OPTIONS_RENDERER_Y,
        pw - 40.0,
        OPTIONS_RENDERER_H,
    )
}

fn options_volume_rect(bus: VolumeBus, width: u32, height: u32) -> (f32, f32, f32, f32) {
    let (px, py, pw, _) = options_panel_rect(width, height);
    let column = (pw - 40.0 - 24.0) / 3.0;
    let x = px + 20.0 + bus.index() as f32 * (column + 12.0);
    (x, py + OPTIONS_VOLUME_Y, column, OPTIONS_VOLUME_H)
}

/// Save (index 0) and Cancel (index 1), right-aligned to the panel's inner
/// edge with Save left of Cancel.
pub(crate) fn options_button_rect(index: usize, width: u32, height: u32) -> (f32, f32, f32, f32) {
    let (px, py, pw, ph) = options_panel_rect(width, height);
    let x = px + pw - 20.0 - (2 - index) as f32 * (OPTIONS_BUTTON_W + 12.0) - OPTIONS_BUTTON_W;
    (x, py + ph - 78.0, OPTIONS_BUTTON_W, OPTIONS_BUTTON_H)
}

/// Read-only controls list rect.
fn options_controls_rect(width: u32, height: u32) -> (f32, f32, f32, f32) {
    let (px, py, pw, ph) = options_panel_rect(width, height);
    let y = py + OPTIONS_LIST_TOP;
    let h = (py + ph - 90.0 - y).max(CONTROLS_ROW_H);
    (px + 20.0, y, pw - 40.0, h)
}

fn options_row_rect(row: usize, width: u32, height: u32) -> (f32, f32, f32, f32) {
    match OPTIONS_ROWS[row] {
        OptionsRow::Renderer => options_renderer_rect(width, height),
        OptionsRow::Volume(bus) => options_volume_rect(bus, width, height),
        OptionsRow::Save => options_button_rect(0, width, height),
        OptionsRow::Cancel => options_button_rect(1, width, height),
    }
}

/// The Options modal (video, audio, controls) drawn over the main menu. A
/// pure state machine like `MainMenu`: the shell applies the emitted
/// actions (live mixer push, persistence, notices) and owns the file paths,
/// so this never touches window, mixer or disk state.
pub struct OptionsModal {
    renderer: RendererChoice,
    /// Volumes by `VolumeBus::index`: master, music, effects.
    volumes: [f32; 3],
    /// Read-only controls list as (action, key) pairs from the data layer.
    controls: Vec<(String, String)>,
    focus: usize,
    hover: Option<usize>,
    /// Controls-list lines scrolled past; clamped against the visible area.
    controls_scroll: i32,
    /// Fallback note surfaced when the persisted renderer value is invalid.
    note: Option<String>,
    /// Transient shell feedback (e.g. a failed save).
    status: Option<String>,
}

impl OptionsModal {
    /// Build the modal from the persisted settings: `note` carries the
    /// invalid-renderer fallback text when the saved value did not parse.
    pub fn new(
        renderer: RendererChoice,
        master: f32,
        music: f32,
        effects: f32,
        controls: Vec<(String, String)>,
        note: Option<String>,
    ) -> Self {
        Self {
            renderer,
            volumes: [master, music, effects].map(|volume| volume.clamp(0.0, 1.0)),
            controls,
            focus: 0,
            hover: None,
            controls_scroll: 0,
            note,
            status: None,
        }
    }

    #[cfg(test)]
    pub(crate) fn renderer(&self) -> RendererChoice {
        self.renderer
    }

    pub(crate) fn volume(&self, bus: VolumeBus) -> f32 {
        self.volumes[bus.index()]
    }

    #[cfg(test)]
    pub(crate) fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    #[cfg(test)]
    fn focus_index(&self) -> usize {
        self.focus
    }

    /// Number of rendered controls-list lines (test helper).
    #[cfg(test)]
    fn controls_rows_total(&self) -> usize {
        self.controls_rows().len()
    }

    /// Rendered controls-list line `index` (test helper).
    #[cfg(test)]
    fn controls_row_label(&self, index: usize) -> ControlsRow {
        self.controls_rows()[index].clone()
    }

    /// First visible controls-list line, clamped to what the list can show
    /// in a window `height` pixels tall.
    pub fn controls_offset(&self, height: u32) -> usize {
        (self.controls_scroll.max(0) as usize).min(self.max_scroll(height))
    }

    pub fn set_status(&mut self, status: impl Into<String>) {
        self.status = Some(status.into());
    }

    /// Scroll the controls list by `rows` lines (positive scrolls down).
    pub fn scroll_controls(&mut self, rows: i32) {
        self.controls_scroll = (self.controls_scroll + rows).max(0);
    }

    /// Apply a mouse-wheel gesture to the controls list (each notch moves
    /// `CONTROLS_WHEEL_ROWS` lines).
    pub fn scroll_controls_wheel(&mut self, notches: f32) {
        if notches.abs() > f32::EPSILON {
            self.scroll_controls((notches * CONTROLS_WHEEL_ROWS as f32).round() as i32);
        }
    }

    /// Clamp the scroll offset against the visible area for `height`.
    pub fn clamp_controls_scroll(&mut self, height: u32) {
        self.controls_scroll = self
            .controls_scroll
            .clamp(0, self.max_scroll(height) as i32);
    }

    /// Handle a pressed key; arrow keys move the cursor, left/right adjust
    /// the focused row, Enter activates and Escape cancels.
    pub fn on_key(&mut self, key: &Key) -> Option<OptionsAction> {
        match key {
            Key::Named(NamedKey::ArrowUp) => {
                self.move_focus(-1);
                None
            }
            Key::Named(NamedKey::ArrowDown) => {
                self.move_focus(1);
                None
            }
            Key::Named(NamedKey::ArrowLeft) => self.adjust(-1),
            Key::Named(NamedKey::ArrowRight) => self.adjust(1),
            Key::Named(NamedKey::PageUp) => {
                self.scroll_controls(-CONTROLS_PAGE_ROWS);
                None
            }
            Key::Named(NamedKey::PageDown) => {
                self.scroll_controls(CONTROLS_PAGE_ROWS);
                None
            }
            Key::Named(NamedKey::Enter) => self.activate(),
            Key::Named(NamedKey::Escape) => Some(OptionsAction::Cancel),
            Key::Character(text) => match text.to_ascii_lowercase().as_str() {
                "w" => {
                    self.move_focus(-1);
                    None
                }
                "s" => {
                    self.move_focus(1);
                    None
                }
                _ => None,
            },
            _ => None,
        }
    }

    /// Update the hovered row from a pointer position; reports whether the
    /// highlight changed so the shell only redraws when it must.
    pub fn on_pointer(&mut self, x: f32, y: f32, width: u32, height: u32) -> bool {
        let hovered = self.row_at(x, y, width, height);
        if hovered != self.hover {
            self.hover = hovered;
            true
        } else {
            false
        }
    }

    /// Handle a left click: Save/Cancel activate, the row ends step a value
    /// and the row middle only focuses. Clicks outside the rows are ignored.
    pub fn on_click(&mut self, x: f32, y: f32, width: u32, height: u32) -> Option<OptionsAction> {
        let row = self.row_at(x, y, width, height)?;
        self.focus = row;
        self.hover = None;
        match OPTIONS_ROWS[row] {
            OptionsRow::Save => Some(OptionsAction::Save),
            OptionsRow::Cancel => Some(OptionsAction::Cancel),
            OptionsRow::Renderer => {
                let (rx, _, rw, _) = options_renderer_rect(width, height);
                if x <= rx + OPTIONS_ARROW_ZONE || x >= rx + rw - OPTIONS_ARROW_ZONE {
                    self.adjust(1)
                } else {
                    None
                }
            }
            OptionsRow::Volume(_) => {
                let (rx, _, rw, _) = options_row_rect(row, width, height);
                if x <= rx + OPTIONS_ARROW_ZONE {
                    self.adjust(-1)
                } else if x >= rx + rw - OPTIONS_ARROW_ZONE {
                    self.adjust(1)
                } else {
                    None
                }
            }
        }
    }

    /// Draw the modal over the menu.
    pub fn draw(&self, renderer: &mut dyn Renderer, width: u32, height: u32) {
        let (px, py, pw, ph) = options_panel_rect(width, height);
        renderer.draw_rect(px, py, pw, ph, [0.05, 0.07, 0.12, 1.0]);
        renderer.draw_rect(px, py, pw, 2.0, [0.82, 0.42, 0.16, 1.0]);
        renderer.draw_rect(px, py, 1.0, ph, [0.20, 0.30, 0.45, 1.0]);
        renderer.draw_rect(px + pw - 1.0, py, 1.0, ph, [0.20, 0.30, 0.45, 1.0]);
        renderer.draw_rect(px, py + ph - 1.0, pw, 1.0, [0.20, 0.30, 0.45, 1.0]);

        renderer.draw_text(
            "OPTIONS",
            px + 20.0,
            py + 16.0,
            22.0,
            [0.96, 0.83, 0.46, 1.0],
        );
        let breadcrumb = "MAIN MENU > OPTIONS";
        renderer.draw_text(
            breadcrumb,
            (px + pw - 20.0 - text_width(breadcrumb, 9.0)).max(px + 20.0),
            py + 26.0,
            9.0,
            [0.42, 0.52, 0.68, 1.0],
        );
        renderer.draw_rect(
            px + 20.0,
            py + 56.0,
            pw - 40.0,
            1.0,
            [0.20, 0.30, 0.45, 1.0],
        );

        renderer.draw_text(
            "VIDEO",
            px + 20.0,
            py + OPTIONS_VIDEO_Y,
            10.0,
            [0.70, 0.82, 0.96, 1.0],
        );
        self.draw_renderer_row(renderer, width, height);
        renderer.draw_text(
            "Renderer changes apply at the next launch.",
            px + 20.0,
            py + OPTIONS_NOTE_Y,
            8.0,
            [0.55, 0.64, 0.77, 1.0],
        );
        if let Some(note) = &self.note {
            renderer.draw_text(
                note,
                px + 20.0,
                py + OPTIONS_INVALID_NOTE_Y,
                8.0,
                [1.0, 0.72, 0.40, 1.0],
            );
        }

        renderer.draw_text(
            "AUDIO",
            px + 20.0,
            py + OPTIONS_AUDIO_Y,
            10.0,
            [0.70, 0.82, 0.96, 1.0],
        );
        for bus in VolumeBus::ALL {
            self.draw_volume_row(renderer, bus, width, height);
        }

        renderer.draw_text(
            "CONTROLS",
            px + 20.0,
            py + OPTIONS_CONTROLS_Y,
            10.0,
            [0.70, 0.82, 0.96, 1.0],
        );
        renderer.draw_text(
            "Hotkeys are fixed in this build.",
            px + 20.0,
            py + OPTIONS_CONTROLS_SUB_Y,
            8.0,
            [0.55, 0.64, 0.77, 1.0],
        );
        self.draw_controls_list(renderer, width, height);

        for (index, label) in [(0usize, "Save"), (1, "Cancel")] {
            let (bx, by, bw, bh) = options_button_rect(index, width, height);
            let row = if index == 0 {
                OPTIONS_ROWS.len() - 2
            } else {
                OPTIONS_ROWS.len() - 1
            };
            let selected = self.focus == row;
            let highlighted = selected || self.hover == Some(row);
            if highlighted {
                renderer.draw_rect(bx, by, bw, bh, [0.13, 0.19, 0.29, 1.0]);
            }
            renderer.draw_rect(
                bx,
                by,
                3.0,
                bh,
                if selected {
                    [0.90, 0.55, 0.20, 1.0]
                } else {
                    [0.25, 0.34, 0.48, 1.0]
                },
            );
            renderer.draw_text(
                label,
                bx + (bw - text_width(label, 13.0)) * 0.5,
                by + (bh - 13.0) * 0.5,
                13.0,
                if selected {
                    [1.0, 0.88, 0.55, 1.0]
                } else {
                    [0.82, 0.88, 0.96, 1.0]
                },
            );
        }

        renderer.draw_text(
            "Up/Down: select   Left/Right: adjust   Enter: activate   Esc: cancel   PgUp/PgDn: scroll",
            px + 20.0,
            py + ph - 36.0,
            8.0,
            [0.45, 0.55, 0.70, 1.0],
        );
        if let Some(status) = &self.status {
            renderer.draw_text(
                status,
                px + 20.0,
                py + ph - 18.0,
                8.0,
                [1.0, 0.72, 0.40, 1.0],
            );
        }
    }

    fn draw_renderer_row(&self, renderer: &mut dyn Renderer, width: u32, height: u32) {
        let (rx, ry, rw, rh) = options_renderer_rect(width, height);
        let row = 0;
        let selected = self.focus == row;
        if selected || self.hover == Some(row) {
            renderer.draw_rect(rx, ry, rw, rh, [0.13, 0.19, 0.29, 1.0]);
        }
        renderer.draw_rect(
            rx,
            ry,
            3.0,
            rh,
            if selected {
                [0.90, 0.55, 0.20, 1.0]
            } else {
                [0.25, 0.34, 0.48, 1.0]
            },
        );
        renderer.draw_text(
            "Renderer",
            rx + 16.0,
            ry + (rh - 13.0) * 0.5,
            13.0,
            if selected {
                [1.0, 0.88, 0.55, 1.0]
            } else {
                [0.82, 0.88, 0.96, 1.0]
            },
        );
        let value = self.renderer.label();
        let box_w = 104.0;
        let box_h = 22.0;
        let bx = rx + rw - box_w - 16.0;
        let by = ry + (rh - box_h) * 0.5;
        renderer.draw_rect(bx, by, box_w, box_h, [0.08, 0.11, 0.18, 1.0]);
        renderer.draw_rect(bx, by, box_w, 1.0, [0.25, 0.34, 0.48, 1.0]);
        renderer.draw_rect(bx, by + box_h - 1.0, box_w, 1.0, [0.25, 0.34, 0.48, 1.0]);
        renderer.draw_text("<", bx - 22.0, by + 4.0, 12.0, [0.62, 0.74, 0.90, 1.0]);
        renderer.draw_text(
            ">",
            bx + box_w + 10.0,
            by + 4.0,
            12.0,
            [0.62, 0.74, 0.90, 1.0],
        );
        renderer.draw_text(
            value,
            bx + (box_w - text_width(value, 12.0)) * 0.5,
            by + 5.0,
            12.0,
            [0.90, 0.93, 0.98, 1.0],
        );
    }

    fn draw_volume_row(
        &self,
        renderer: &mut dyn Renderer,
        bus: VolumeBus,
        width: u32,
        height: u32,
    ) {
        let (rx, ry, rw, rh) = options_volume_rect(bus, width, height);
        let row = OPTIONS_ROWS
            .iter()
            .position(|entry| *entry == OptionsRow::Volume(bus))
            .unwrap_or(0);
        let selected = self.focus == row;
        if selected || self.hover == Some(row) {
            renderer.draw_rect(rx, ry, rw, rh, [0.13, 0.19, 0.29, 1.0]);
        }
        renderer.draw_rect(
            rx,
            ry,
            3.0,
            rh,
            if selected {
                [0.90, 0.55, 0.20, 1.0]
            } else {
                [0.25, 0.34, 0.48, 1.0]
            },
        );
        renderer.draw_text(
            bus.label(),
            rx + 16.0,
            ry + 4.0,
            12.0,
            if selected {
                [1.0, 0.88, 0.55, 1.0]
            } else {
                [0.82, 0.88, 0.96, 1.0]
            },
        );
        let percent = format!("{}%", (self.volume(bus) * 100.0).round() as i32);
        let percent_w = text_width(&percent, 12.0);
        renderer.draw_text(
            &percent,
            rx + rw - 44.0 - percent_w,
            ry + 4.0,
            12.0,
            [0.90, 0.93, 0.98, 1.0],
        );
        renderer.draw_text("<", rx + rw - 34.0, ry + 5.0, 11.0, [0.62, 0.74, 0.90, 1.0]);
        renderer.draw_text(">", rx + rw - 18.0, ry + 5.0, 11.0, [0.62, 0.74, 0.90, 1.0]);

        let bar_x = rx + 16.0;
        let bar_w = rw - 32.0;
        let bar_y = ry + 26.0;
        let bar_h = 9.0;
        renderer.draw_rect(bar_x, bar_y, bar_w, bar_h, [0.10, 0.14, 0.22, 1.0]);
        let fill = bar_w * self.volume(bus);
        if fill > 0.5 {
            renderer.draw_rect(
                bar_x,
                bar_y,
                fill,
                bar_h,
                if selected {
                    [0.90, 0.55, 0.20, 1.0]
                } else {
                    [0.30, 0.42, 0.62, 1.0]
                },
            );
        }
        renderer.draw_rect(bar_x, bar_y, bar_w, 1.0, [0.22, 0.32, 0.48, 1.0]);
        renderer.draw_rect(
            bar_x,
            bar_y + bar_h - 1.0,
            bar_w,
            1.0,
            [0.22, 0.32, 0.48, 1.0],
        );
    }

    fn draw_controls_list(&self, renderer: &mut dyn Renderer, width: u32, height: u32) {
        let (lx, ly, lw, lh) = options_controls_rect(width, height);
        let visible = (lh / CONTROLS_ROW_H).floor().max(1.0) as usize;
        let offset = self.controls_offset(height);
        let max_scroll = self.max_scroll(height);
        for (index, row) in self
            .controls_rows()
            .into_iter()
            .enumerate()
            .skip(offset)
            .take(visible)
        {
            let y = ly + (index - offset) as f32 * CONTROLS_ROW_H;
            match row {
                ControlsRow::Group(group) => {
                    renderer.draw_text(group, lx + 2.0, y, 9.0, [0.82, 0.42, 0.16, 1.0]);
                }
                ControlsRow::Entry { action, key } => {
                    let key_w = text_width(&key, 9.0);
                    let key_x = lx + lw - CONTROLS_KEY_GUTTER - key_w;
                    renderer.draw_text(&key, key_x, y, 9.0, [0.62, 0.74, 0.90, 1.0]);
                    // The fitted label keeps its 18 px lead-in before the key
                    // column, so only the gutter moved.
                    let avail = key_x - lx - 2.0 - 18.0;
                    renderer.draw_text(
                        &fit_text(&action, avail, 9.0),
                        lx + 2.0,
                        y,
                        9.0,
                        [0.82, 0.88, 0.96, 1.0],
                    );
                }
            }
        }
        if offset > 0 {
            renderer.draw_text("^", lx + lw - 12.0, ly - 12.0, 8.0, [0.45, 0.55, 0.70, 1.0]);
        }
        if offset < max_scroll {
            renderer.draw_text("v", lx + lw - 12.0, ly + lh, 8.0, [0.45, 0.55, 0.70, 1.0]);
        }
    }

    /// Group the controls list for display: game groups first, then the menu
    /// and shared keys. Matches the data layer's deterministic ordering.
    fn controls_rows(&self) -> Vec<ControlsRow> {
        let mut rows: Vec<ControlsRow> = Vec::new();
        let mut group: Option<&'static str> = None;
        for (action, key) in &self.controls {
            let next = controls_group(action);
            if Some(next) != group {
                rows.push(ControlsRow::Group(next));
                group = Some(next);
            }
            let label = action
                .strip_prefix("TD: ")
                .or_else(|| action.strip_prefix("RA: "))
                .unwrap_or(action);
            rows.push(ControlsRow::Entry {
                action: label.to_string(),
                key: key.clone(),
            });
        }
        rows
    }

    fn max_scroll(&self, height: u32) -> usize {
        let (_, _, _, list_h) = options_controls_rect(0, height);
        let visible = (list_h / CONTROLS_ROW_H).floor().max(1.0) as usize;
        self.controls_rows().len().saturating_sub(visible)
    }

    fn row_at(&self, x: f32, y: f32, width: u32, height: u32) -> Option<usize> {
        (0..OPTIONS_ROWS.len()).find(|row| {
            let (rx, ry, rw, rh) = options_row_rect(*row, width, height);
            x >= rx && x <= rx + rw && y >= ry && y <= ry + rh
        })
    }

    fn move_focus(&mut self, delta: i32) {
        let len = OPTIONS_ROWS.len() as i32;
        self.focus = (((self.focus as i32 + delta) % len + len) % len) as usize;
    }

    /// Left/Right on the focused row: the renderer toggles, a volume steps.
    fn adjust(&mut self, delta: i32) -> Option<OptionsAction> {
        match OPTIONS_ROWS[self.focus] {
            OptionsRow::Renderer => {
                self.renderer = self.renderer.other();
                Some(OptionsAction::Renderer(self.renderer))
            }
            OptionsRow::Volume(bus) => {
                let stepped = (self.volume(bus) + delta as f32 * VOLUME_STEP).clamp(0.0, 1.0);
                self.volumes[bus.index()] = stepped;
                Some(OptionsAction::Volume(bus, stepped))
            }
            OptionsRow::Save | OptionsRow::Cancel => None,
        }
    }

    /// Enter on the focused row: Save/Cancel commit, the renderer toggles
    /// and the volume rows have no discrete action.
    fn activate(&mut self) -> Option<OptionsAction> {
        match OPTIONS_ROWS[self.focus] {
            OptionsRow::Save => Some(OptionsAction::Save),
            OptionsRow::Cancel => Some(OptionsAction::Cancel),
            OptionsRow::Renderer => self.adjust(1),
            OptionsRow::Volume(_) => None,
        }
    }
}

/// Group header for a controls-list entry (TD / RA / everything else).
fn controls_group(action: &str) -> &'static str {
    if action.starts_with("TD:") {
        "TIBERIAN DAWN"
    } else if action.starts_with("RA:") {
        "RED ALERT"
    } else {
        "MENU / SHARED"
    }
}

/// Shorten text with an ellipsis when it would collide with the key column.
fn fit_text(text: &str, available: f32, size: f32) -> String {
    if text_width(text, size) <= available {
        return text.to_string();
    }
    let per_char = size * 1.25;
    let keep = ((available / per_char).floor() as usize).saturating_sub(1);
    let mut shortened: String = text.chars().take(keep).collect();
    shortened.push('…');
    shortened
}

fn content_width(width: u32) -> f32 {
    (width as f32 - 120.0).clamp(320.0, MAX_ITEM_WIDTH)
}

fn item_rect(index: usize, width: u32) -> (f32, f32, f32, f32) {
    (
        ITEM_X,
        ITEM_TOP + index as f32 * (ITEM_HEIGHT + ITEM_GAP),
        content_width(width),
        ITEM_HEIGHT,
    )
}

/// Approximate rendered width of the embedded 8x8 font (10 px advance at size 8).
fn text_width(text: &str, size: f32) -> f32 {
    text.chars().count() as f32 * size * 1.25
}

/// Directory used when nothing could be discovered; kept for notices/tests.
pub(crate) fn default_campaign_dir(game: GameChoice) -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("maps")
        .join(game.campaign_folder())
}

/// Candidate campaign directories, most specific first.
pub(crate) fn campaign_candidates(game: GameChoice) -> Vec<PathBuf> {
    let folder = game.campaign_folder();
    let mut candidates = Vec::new();
    if let Some(value) = std::env::var_os(game.campaign_env()) {
        if !value.is_empty() {
            candidates.push(PathBuf::from(value));
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join("maps").join(folder));
        candidates.push(cwd.join("RUST").join("maps").join(folder));
        if let Some(parent) = cwd.parent() {
            candidates.push(parent.join("maps").join(folder));
        }
    }
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    if let Some(root) = manifest.parent() {
        candidates.push(root.join("maps").join(folder));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("maps").join(folder));
        }
    }
    candidates
}

/// Count scenario INIs in a directory (0 when it cannot be read).
pub(crate) fn scenario_count(dir: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    entries
        .flatten()
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ini"))
        })
        .count()
}

/// Find the first candidate directory that contains at least one scenario.
pub(crate) fn discover_campaign(game: GameChoice) -> CampaignInfo {
    let candidates = campaign_candidates(game);
    for dir in &candidates {
        let scenarios = scenario_count(dir);
        if scenarios > 0 {
            return CampaignInfo {
                dir: dir.clone(),
                scenarios,
            };
        }
    }
    CampaignInfo::empty(game)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cnc_renderer::{Camera, SoftwareRenderer};

    /// Screen-space camera (world units == pixels) for headless draw tests.
    fn screen_camera(width: u32, height: u32) -> Camera {
        let mut camera = Camera::default();
        camera.set_viewport(width.max(1), height.max(1));
        camera.position = cnc_core::Vec2::new(
            cnc_core::Fixed16_16::from_f32(width as f32 * 0.5),
            cnc_core::Fixed16_16::from_f32(height as f32 * 0.5),
        );
        camera
    }

    /// One entry per controls group so the grouping headers are exercised.
    fn test_controls() -> Vec<(String, String)> {
        [
            ("Quick save", "F5"),
            ("Move cursor down", "ArrowDown / S"),
            ("TD: pause", "Escape"),
            ("RA: scatter selection", "G"),
        ]
        .into_iter()
        .map(|(action, key)| (action.to_string(), key.to_string()))
        .collect()
    }

    fn modal() -> OptionsModal {
        OptionsModal::new(RendererChoice::Wgpu, 1.0, 1.0, 1.0, test_controls(), None)
    }

    #[test]
    fn options_modal_seeds_from_the_persisted_values() {
        let options = OptionsModal::new(
            RendererChoice::Software,
            0.5,
            0.25,
            0.75,
            test_controls(),
            Some("Saved renderer 'vulkan' is invalid - using Wgpu.".to_string()),
        );
        assert_eq!(options.renderer(), RendererChoice::Software);
        assert_eq!(options.volume(VolumeBus::Master), 0.5);
        assert_eq!(options.volume(VolumeBus::Music), 0.25);
        assert_eq!(options.volume(VolumeBus::Effects), 0.75);
        assert_eq!(options.focus_index(), 0);
        assert!(options.note().is_some());
        assert!(options.status().is_none());

        // Out-of-range seeds clamp; the modal never stores a bad volume.
        let clamped = OptionsModal::new(RendererChoice::Wgpu, 5.0, -2.0, 1.5, vec![], None);
        assert_eq!(clamped.volume(VolumeBus::Master), 1.0);
        assert_eq!(clamped.volume(VolumeBus::Music), 0.0);
        assert_eq!(clamped.volume(VolumeBus::Effects), 1.0);
    }

    #[test]
    fn options_modal_volume_keys_step_and_clamp_at_both_ends() {
        let mut options = modal();
        // W/S move focus like the menu, so focus the Master row.
        options.on_key(&Key::Character("s".into()));
        assert_eq!(options.focus_index(), 1);

        let mut last = f32::NAN;
        for _ in 0..40 {
            if let Some(OptionsAction::Volume(bus, value)) =
                options.on_key(&Key::Named(NamedKey::ArrowLeft))
            {
                assert_eq!(bus, VolumeBus::Master);
                last = value;
            }
        }
        assert!((last - 0.0).abs() < 1.0e-6, "stepping down stops at zero");
        assert!((options.volume(VolumeBus::Master) - 0.0).abs() < 1.0e-6);
        options.on_key(&Key::Named(NamedKey::ArrowLeft));
        assert!(
            (options.volume(VolumeBus::Master) - 0.0).abs() < 1.0e-6,
            "zero stays clamped at zero"
        );
        for _ in 0..40 {
            options.on_key(&Key::Named(NamedKey::ArrowRight));
        }
        assert!(
            (options.volume(VolumeBus::Master) - 1.0).abs() < 1.0e-6,
            "stepping up stops at one"
        );
        assert_eq!(options.volume(VolumeBus::Music), 1.0);
        assert_eq!(options.volume(VolumeBus::Effects), 1.0);
    }

    #[test]
    fn options_modal_focus_wraps_over_every_row() {
        let mut options = modal();
        assert_eq!(options.focus_index(), 0);
        options.on_key(&Key::Character("w".into()));
        assert_eq!(
            options.focus_index(),
            OPTIONS_ROWS.len() - 1,
            "up wraps to Cancel"
        );
        options.on_key(&Key::Named(NamedKey::ArrowDown));
        assert_eq!(options.focus_index(), 0);
        for _ in 0..OPTIONS_ROWS.len() {
            options.on_key(&Key::Named(NamedKey::ArrowDown));
        }
        assert_eq!(
            options.focus_index(),
            0,
            "one full cycle returns to the first row"
        );
    }

    #[test]
    fn options_modal_enter_saves_and_escape_cancels() {
        let mut options = modal();
        for _ in 0..(OPTIONS_ROWS.len() - 2) {
            options.on_key(&Key::Named(NamedKey::ArrowDown));
        }
        assert_eq!(options.focus_index(), OPTIONS_ROWS.len() - 2, "Save row");
        assert_eq!(
            options.on_key(&Key::Named(NamedKey::Enter)),
            Some(OptionsAction::Save)
        );
        options.on_key(&Key::Named(NamedKey::ArrowDown));
        assert_eq!(
            options.on_key(&Key::Named(NamedKey::Enter)),
            Some(OptionsAction::Cancel)
        );
        // Escape cancels from any row, so the modal always has a way out.
        for _ in 0..3 {
            options.on_key(&Key::Named(NamedKey::ArrowDown));
        }
        assert_eq!(
            options.on_key(&Key::Named(NamedKey::Escape)),
            Some(OptionsAction::Cancel)
        );
    }

    #[test]
    fn options_modal_renderer_toggles_with_arrows_and_enter() {
        let mut options = modal();
        assert_eq!(options.renderer(), RendererChoice::Wgpu);
        assert_eq!(
            options.on_key(&Key::Named(NamedKey::ArrowRight)),
            Some(OptionsAction::Renderer(RendererChoice::Software))
        );
        assert_eq!(options.renderer(), RendererChoice::Software);
        assert_eq!(
            options.on_key(&Key::Named(NamedKey::ArrowLeft)),
            Some(OptionsAction::Renderer(RendererChoice::Wgpu))
        );
        assert_eq!(
            options.on_key(&Key::Named(NamedKey::Enter)),
            Some(OptionsAction::Renderer(RendererChoice::Software))
        );
    }

    #[test]
    fn options_modal_click_zones_adjust_and_activate() {
        let mut options =
            OptionsModal::new(RendererChoice::Wgpu, 0.5, 0.5, 0.5, test_controls(), None);
        let (vx, vy, vw, vh) = options_volume_rect(VolumeBus::Music, 1024, 768);
        assert_eq!(
            options.on_click(vx + 4.0, vy + vh * 0.5, 1024, 768),
            Some(OptionsAction::Volume(VolumeBus::Music, 0.45))
        );
        assert_eq!(options.focus_index(), 2, "clicking a row focuses it");
        assert_eq!(
            options.on_click(vx + vw - 4.0, vy + vh * 0.5, 1024, 768),
            Some(OptionsAction::Volume(VolumeBus::Music, 0.5))
        );
        assert_eq!(
            options.on_click(vx + vw * 0.5, vy + vh * 0.5, 1024, 768),
            None,
            "the row middle only focuses"
        );
        assert_eq!(options.focus_index(), 2);

        let (rx, ry, rw, rh) = options_renderer_rect(1024, 768);
        assert_eq!(
            options.on_click(rx + 4.0, ry + rh * 0.5, 1024, 768),
            Some(OptionsAction::Renderer(RendererChoice::Software))
        );
        assert_eq!(
            options.on_click(rx + rw - 4.0, ry + rh * 0.5, 1024, 768),
            Some(OptionsAction::Renderer(RendererChoice::Wgpu))
        );

        let (sx, sy, sw, sh) = options_button_rect(0, 1024, 768);
        assert_eq!(
            options.on_click(sx + sw * 0.5, sy + sh * 0.5, 1024, 768),
            Some(OptionsAction::Save)
        );
        let (cx, cy, cw, ch) = options_button_rect(1, 1024, 768);
        assert_eq!(
            options.on_click(cx + cw * 0.5, cy + ch * 0.5, 1024, 768),
            Some(OptionsAction::Cancel)
        );
        assert_eq!(
            options.on_click(2.0, 2.0, 1024, 768),
            None,
            "clicks outside the rows do nothing"
        );

        assert!(options.on_pointer(vx + 4.0, vy + vh * 0.5, 1024, 768));
        assert!(
            !options.on_pointer(vx + 4.0, vy + vh * 0.5, 1024, 768),
            "the same hover reports no change"
        );
    }

    #[test]
    fn options_modal_controls_list_scrolls_and_clamps() {
        let mut options = modal();
        let rows = options.controls_rows_total();
        // A tall window shows every row, so a short list never scrolls.
        options.scroll_controls(2);
        assert_eq!(options.controls_offset(768), 0);
        options.clamp_controls_scroll(768);
        assert_eq!(options.controls_offset(768), 0);

        // A short window shows one line, so the scroll clamps to the end.
        options.scroll_controls(10);
        options.clamp_controls_scroll(400);
        let (_, _, _, list_h) = options_controls_rect(1024, 400);
        let visible = (list_h / CONTROLS_ROW_H).floor().max(1.0) as usize;
        assert_eq!(
            options.controls_offset(400),
            rows.saturating_sub(visible),
            "the scroll stops at the last visible line"
        );
        options.scroll_controls(-10);
        options.clamp_controls_scroll(400);
        assert_eq!(
            options.controls_offset(400),
            0,
            "scroll never goes negative"
        );

        // Page keys scroll the same list without moving the row focus.
        let focus = options.focus_index();
        options.on_key(&Key::Named(NamedKey::PageDown));
        assert_eq!(options.controls_scroll, CONTROLS_PAGE_ROWS);
        options.on_key(&Key::Named(NamedKey::PageUp));
        options.on_key(&Key::Named(NamedKey::PageUp));
        assert_eq!(options.controls_scroll, 0);
        assert_eq!(options.focus_index(), focus);
    }

    #[test]
    fn options_modal_groups_controls_by_game() {
        let options = OptionsModal::new(RendererChoice::Wgpu, 1.0, 1.0, 1.0, test_controls(), None);
        // "Quick save" / "Move cursor down" share the MENU / SHARED header,
        // so the four test entries produce exactly three group headers and
        // the display strips the game prefixes.
        assert_eq!(options.controls_rows_total(), 4 + 3);
        assert!(matches!(
            options.controls_row_label(0),
            ControlsRow::Group("MENU / SHARED")
        ));
        assert!(matches!(
            options.controls_row_label(3),
            ControlsRow::Group("TIBERIAN DAWN")
        ));
        assert!(matches!(
            options.controls_row_label(5),
            ControlsRow::Group("RED ALERT")
        ));
    }

    #[test]
    fn options_modal_draws_the_whole_dialog_without_a_window() {
        let options = OptionsModal::new(
            RendererChoice::Software,
            0.8,
            0.55,
            0.35,
            test_controls(),
            Some("Saved renderer 'vulkan' is invalid - using Wgpu.".to_string()),
        );
        let mut renderer = SoftwareRenderer::new(1024, 768);
        renderer.begin_frame();
        renderer.set_camera(&screen_camera(1024, 768));
        options.draw(&mut renderer, 1024, 768);
        renderer.end_frame().unwrap();
        assert!(
            renderer.buffer().iter().any(|pixel| *pixel != 0),
            "the modal should draw pixels"
        );

        // A short window must still draw (the controls list just clips).
        let mut small = SoftwareRenderer::new(640, 480);
        small.begin_frame();
        small.set_camera(&screen_camera(640, 480));
        options.draw(&mut small, 640, 480);
        small.end_frame().unwrap();
        assert!(small.buffer().iter().any(|pixel| *pixel != 0));
    }

    #[test]
    fn options_modal_key_column_reserves_right_edge_clearance() {
        // Grounded in the rendered 9 px font: `text_width` charges 11.25 px
        // per glyph, so the clearance is sized in glyph advances and must
        // hold at every window size the panel clamps to (560..900 px wide).
        // The bindings mirror `options::default_controls()`; menu.rs stays
        // decoupled from the data layer, so they are inlined here.
        for (width, height) in [(1280u32, 720u32), (1024, 768), (640, 480)] {
            let (lx, _, lw, _) = options_controls_rect(width, height);
            let (px, _, pw, _) = options_panel_rect(width, height);
            let key_right = lx + lw - CONTROLS_KEY_GUTTER;
            // The list's 20 px inset plus the gutter keep the binding column
            // visibly clear of the modal's right border at every scaling.
            assert!(
                px + pw - key_right > 30.0,
                "the key column must keep visible padding from the right edge"
            );
            for key in [
                "Ctrl+1-4",
                "Alt+1-4",
                "ArrowUp / W",
                "ArrowUp / ArrowDown / Enter",
            ] {
                let key_x = key_right - text_width(key, 9.0);
                assert!(
                    key_x >= lx + 2.0,
                    "{key} must start right of the action label at {width}x{height}"
                );
            }
            // The widest binding still leaves its label room: the width the
            // fitted action text gets (key column minus the 2 px label inset
            // and the 18 px lead-in) covers "pause menu", the longest label
            // paired with that binding, so nothing newly truncates.
            let widest = text_width("ArrowUp / ArrowDown / Enter", 9.0);
            let avail = lw - CONTROLS_KEY_GUTTER - widest - 20.0;
            assert!(
                avail >= text_width("pause menu", 9.0),
                "the widest binding must not force its label to truncate"
            );
        }
        // Wider than one glyph advance, so the padding stays visible.
        assert!(CONTROLS_KEY_GUTTER > 9.0 * 1.25);
    }
}
