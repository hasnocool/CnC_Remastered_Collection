//! Executable arms for parsed `TEVENT` / `TACTION` records.
//!
//! `crate::trigger_records` owns the vocabulary: it parses the original
//! record strings and types their payloads. This module owns the *execution*
//! half: a `TriggerHost` trait that names every action family the original
//! engines could perform, and an exhaustive dispatcher
//! (`apply_action_record`) that turns one parsed record into exactly one
//! trait call.
//!
//! The split is deliberate:
//!
//! * the dispatcher in `cnc-core` is a single `match` over
//!   `ActionKind`. Adding a variant to the enumeration breaks compilation
//!   here until an arm exists, so the vocabulary can never grow silently
//!   unused;
//! * each engine implements `TriggerHost` against its own game state.
//!   Trait methods default to a logged rejection (`TriggerHost::reject`),
//!   so an engine can leave a family unimplemented without lying about it and
//!   without a silent no-op;
//! * `ActionKind::Unknown` never reaches an engine: the dispatcher logs and
//!   ignores it, and the mission continues (forward compatibility with a
//!   newer scenario revision).
//!
//! Positional actions (`TACTION_DZ`, `TACTION_REVEAL_SOME`,
//! `TACTION_REVEAL_ZONE`) carry a *waypoint number* in the record's `value`
//! field. The port has no runtime waypoint table, so
//! `waypoint_cell` reads the number as a row-major cell index on the host's
//! map. That keeps scripted scenarios executable and deterministic without
//! inventing a waypoint store.
//!
//! References: `REDALERT/TACTION.H:39` (`TActionType`, 37 codes),
//! `REDALERT/TACTION.CPP:839` (`Action_Needs`), `TIBERIANDAWN/TRIGGER.H:74`
//! (TD's `ActionType`), `REDALERT/TEVENT.H:43`.

use crate::math::Vec2i;
use crate::trigger_records::{ActionKind, ActionRecord, EventKind, EventRecord, ParamKind};

/// Red Alert special-weapon indices (`REDALERT/DEFINES.H:806`, `SPC_*`).
///
/// `TACTION_1_SPECIAL` / `TACTION_FULL_SPECIAL` carry one of these in their
/// payload; the campaigns use them to hand the computer airstrikes, nukes and
/// ion strikes without the owning house having built the prerequisite.
pub mod special_weapon {
    /// `SPC_SONAR_PULSE` (0): briefly reveals submarines.
    pub const SONAR_PULSE: i64 = 0;
    /// `SPC_NUCLEAR_BOMB` (1): tactical nuclear weapon.
    pub const NUCLEAR_BOMB: i64 = 1;
    /// `SPC_CHRONOSPHERE` (2): teleportation.
    pub const CHRONOSPHERE: i64 = 2;
    /// `SPC_PARA_BOMB` (3): the airstrike delivery.
    pub const PARA_BOMB: i64 = 3;
    /// `SPC_PARA_INFANTRY` (4): parachute reinforcements.
    pub const PARA_INFANTRY: i64 = 4;
    /// `SPC_SPY_MISSION` (5): spy plane photo reconnaissance.
    pub const SPY_MISSION: i64 = 5;
    /// `SPC_IRON_CURTAIN` (6): invulnerability.
    pub const IRON_CURTAIN: i64 = 6;
    /// `SPC_GPS` (7): unjammable radar.
    pub const GPS: i64 = 7;
}

/// The outcome of applying one parsed action record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApplyReport {
    /// The action code that was executed.
    pub action: ActionKind,
    /// `true` when the host performed the action.
    ///
    /// `false` means the record was a logged no-op: either the code was
    /// `ActionKind::Unknown`, `ActionKind::None`, or this engine rejected
    /// the family through `TriggerHost::reject`.
    pub applied: bool,
}

impl ApplyReport {
    /// A rejected or unknown record; the host has already logged why.
    pub const fn ignored(action: ActionKind) -> Self {
        Self {
            action,
            applied: false,
        }
    }

    /// A record this host executed.
    pub const fn applied(action: ActionKind) -> Self {
        Self {
            action,
            applied: true,
        }
    }
}

/// The live game state a trigger record is applied to.
///
/// Every method returns whether the family was actually performed. The
/// defaults log through `TriggerHost::reject` and return `false`, so an
/// engine only overrides what it can really do.
pub trait TriggerHost {
    /// Record one diagnostic (engine log, test sink, ...). Unknown action
    /// codes and rejected families both go through here.
    fn note(&mut self, message: String);

    /// Map size in cells. Used by `waypoint_cell` to resolve positional
    /// payloads deterministically.
    fn map_size(&self) -> (u32, u32);

    /// The house that acts when an action carries no house payload.
    fn default_house(&self) -> u8;

    /// Number of simulated houses; payload house indices are clamped to it.
    fn house_count(&self) -> usize;

    /// Log and reject one action family. Overriding engines call this from
    /// their own "not supported" arms.
    fn reject(&mut self, action: &str) -> bool {
        self.note(format!(
            "trigger action {action:?} has no arm in this engine; ignored"
        ));
        false
    }

    // ---- mission outcome -------------------------------------------------

    /// `TACTION_WIN`: the player wins.
    fn win(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("Winner is")
    }

    /// `TACTION_LOSE`: the player loses.
    fn lose(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("Loser is")
    }

    /// `TACTION_WINLOSE`: win if the attached object is captured, lose if it
    /// is destroyed.
    fn win_lose(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("win if captured / lose if destroyed")
    }

    /// `TACTION_ALLOWWIN`: permit the mission to end on the next win check.
    fn allow_win(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("Allow Win")
    }

    // ---- teams -----------------------------------------------------------

    /// `TACTION_BEGIN_PRODUCTION`: the house starts factory production.
    fn begin_production(&mut self, house: u8, team: Option<u16>) -> bool {
        let _ = (house, team);
        self.reject("Production Begins")
    }

    /// `TACTION_CREATE_TEAM`: create one instance of a `[TeamTypes]` entry.
    fn create_team(&mut self, house: u8, team: Option<u16>) -> bool {
        let _ = (house, team);
        self.reject("Create Team")
    }

    /// `TACTION_DESTROY_TEAM`: disband every instance of a team type.
    fn destroy_team(&mut self, house: u8, team: Option<u16>) -> bool {
        let _ = (house, team);
        self.reject("Destroy All Teams")
    }

    /// `TACTION_AUTOCREATE`: the house begins autocreating teams.
    fn autocreate(&mut self, house: u8, team: Option<u16>) -> bool {
        let _ = (house, team);
        self.reject("Autocreate Begins")
    }

    /// `TACTION_REINFORCEMENTS`: send a reinforcement team.
    fn reinforcements(&mut self, house: u8, team: Option<u16>) -> bool {
        let _ = (house, team);
        self.reject("Reinforcement (team)")
    }

    /// `TACTION_ALL_HUNT`: every unit of the house goes into hunt mode.
    fn all_hunt(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("All to Hunt")
    }

    // ---- map / spatial ---------------------------------------------------

    /// `TACTION_DZ`: deploy a drop-zone flare and the reinforcement squad.
    fn drop_zone(&mut self, house: u8, cell: Vec2i) -> bool {
        let _ = (house, cell);
        self.reject("Drop Zone Flare (waypoint)")
    }

    /// `TACTION_CREEP_SHADOW`: shroud creeps back one step.
    fn creep_shadow(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("Grow shroud one step")
    }

    /// `TACTION_DESTROY_OBJECT`: destroy the object this trigger is attached
    /// to.
    fn destroy_object(&mut self) -> bool {
        self.reject("Destroy attached building")
    }

    /// `TACTION_REVEAL_ALL`: reveal the whole map for the house.
    fn reveal_map(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("Reveal all map")
    }

    /// `TACTION_REVEAL_SOME`: reveal the area around a waypoint.
    fn reveal_area(&mut self, house: u8, cell: Vec2i) -> bool {
        let _ = (house, cell);
        self.reject("Reveal around waypoint")
    }

    /// `TACTION_REVEAL_ZONE`: reveal the trigger zone at a waypoint.
    fn reveal_zone(&mut self, house: u8, cell: Vec2i) -> bool {
        let _ = (house, cell);
        self.reject("Reveal zone of waypoint")
    }

    // ---- economy ---------------------------------------------------------

    /// `TACTION_FIRE_SALE`: sell every structure and go on a rampage.
    fn fire_sale(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("Fire Sale")
    }

    // ---- presentation ----------------------------------------------------

    /// `TACTION_PLAY_MOVIE`: play a full-screen movie.
    fn play_movie(&mut self, movie: i64) -> bool {
        let _ = movie;
        self.reject("Play Movie")
    }

    /// `TACTION_TEXT_TRIGGER`: display a scripted text message.
    fn text_trigger(&mut self, text: i64) -> bool {
        let _ = text;
        self.reject("Text Trigger (ID num)")
    }

    /// `TACTION_PLAY_SOUND`: play a sound effect.
    fn play_sound(&mut self, sound: i64) -> bool {
        let _ = sound;
        self.reject("Play sound effect")
    }

    /// `TACTION_PLAY_MUSIC`: play a musical theme.
    fn play_music(&mut self, theme: i64) -> bool {
        let _ = theme;
        self.reject("Play music theme")
    }

    /// `TACTION_PLAY_SPEECH`: play an EVA speech line.
    fn play_speech(&mut self, speech: i64) -> bool {
        let _ = speech;
        self.reject("Play speech")
    }

    // ---- triggers / globals / timer --------------------------------------

    /// `TACTION_DESTROY_TRIGGER`: permanently disable another trigger.
    fn destroy_trigger(&mut self, trigger: Option<u16>) -> bool {
        let _ = trigger;
        self.reject("Destroy Trigger")
    }

    /// `TACTION_FORCE_TRIGGER`: fire another trigger immediately.
    fn force_trigger(&mut self, trigger: Option<u16>) -> bool {
        let _ = trigger;
        self.reject("Force Trigger")
    }

    /// `TACTION_SET_GLOBAL` / `TACTION_CLEAR_GLOBAL`.
    fn set_global(&mut self, global: i64, on: bool) -> bool {
        let _ = (global, on);
        self.reject("Global Set/Clear")
    }

    /// `TACTION_START_TIMER`.
    fn start_timer(&mut self) -> bool {
        self.reject("Timer Start")
    }

    /// `TACTION_STOP_TIMER`.
    fn stop_timer(&mut self) -> bool {
        self.reject("Timer Stop")
    }

    /// `TACTION_ADD_TIMER`: extend by tenths of a minute.
    fn extend_timer(&mut self, tenths_of_minute: i64) -> bool {
        let _ = tenths_of_minute;
        self.reject("Timer Extend")
    }

    /// `TACTION_SUB_TIMER`: shorten by tenths of a minute.
    fn shorten_timer(&mut self, tenths_of_minute: i64) -> bool {
        let _ = tenths_of_minute;
        self.reject("Timer Shorten")
    }

    /// `TACTION_SET_TIMER`: set and start the mission timer.
    fn set_timer(&mut self, tenths_of_minute: i64) -> bool {
        let _ = tenths_of_minute;
        self.reject("Timer Set")
    }

    /// `TACTION_BASE_BUILDING`: toggle automated base building.
    fn base_building(&mut self, house: u8, enabled: bool) -> bool {
        let _ = (house, enabled);
        self.reject("Auto Base Building")
    }

    // ---- superweapons ----------------------------------------------------

    /// `TACTION_1_SPECIAL` / `TACTION_FULL_SPECIAL`: grant or fire a special
    /// weapon; the payload is one of the `special_weapon` constants.
    ///
    /// `one_shot` is `true` for `TACTION_1_SPECIAL` (single use) and `false`
    /// for `TACTION_FULL_SPECIAL` (rechargeable).
    fn special_weapon(&mut self, house: u8, weapon: i64, one_shot: bool) -> bool {
        let _ = (house, weapon, one_shot);
        self.reject("Add special weapon")
    }

    /// `TACTION_PREFERRED_TARGET`: designate a preferred target for the house.
    fn preferred_target(&mut self, house: u8, quarry: i64) -> bool {
        let _ = (house, quarry);
        self.reject("Preferred target")
    }

    /// `TACTION_LAUNCH_NUKES`: launch every nuclear silo's missile.
    fn launch_nukes(&mut self, house: u8) -> bool {
        let _ = house;
        self.reject("Launch Nukes")
    }

    // ---- events ----------------------------------------------------------

    /// Evaluate one event code against live state.
    ///
    /// `None` means "this engine cannot answer". A trigger that asks for such
    /// an event does not fire (conservative: an unknown condition must not
    /// trigger a scripted action), and `events_hold` logs it.
    fn evaluate_event(&mut self, kind: EventKind, value: i64) -> Option<bool> {
        let _ = (kind, value);
        None
    }
}

/// Resolve a waypoint/number payload into a map cell (row-major).
///
/// Negative payloads and payloads past the end of the map wrap, matching the
/// original engines' habit of treating waypoint numbers as small non-negative
/// indices while keeping the port panic-free.
pub fn waypoint_cell(width: u32, height: u32, waypoint: i64) -> Vec2i {
    let width = width.max(1) as i64;
    let height = height.max(1) as i64;
    let index = waypoint.rem_euclid(width * height);
    Vec2i::new((index % width) as i32, (index / width) as i32)
}

/// Resolve a house payload against the host's house table.
///
/// `value` is the record's `Data.Value`; negative payloads mean "the player's
/// house", out-of-range indices clamp to the last house so a stale scenario
/// cannot index out of bounds.
pub fn resolve_house<H: TriggerHost + ?Sized>(host: &H, value: i64) -> u8 {
    let count = host.house_count().max(1);
    if value < 0 {
        return host.default_house().min(count.saturating_sub(1) as u8);
    }
    (value as u64).min(count.saturating_sub(1) as u64) as u8
}

/// Interpret a reference payload (team/trigger) with the record's dedicated
/// reference field as fallback.
fn reference(value: i64, fallback: Option<u16>) -> Option<u16> {
    if value >= 0 {
        u16::try_from(value).ok()
    } else {
        fallback
    }
}

/// Apply one parsed action record to the host.
///
/// The `match` is exhaustive over `ActionKind`, so a new code cannot be added
/// to the vocabulary without an arm here. Unknown codes are logged and
/// ignored; the mission keeps running.
pub fn apply_action_record<H: TriggerHost + ?Sized>(
    host: &mut H,
    record: ActionRecord,
) -> ApplyReport {
    let kind = record.kind;
    let (width, height) = host.map_size();
    let cell = waypoint_cell(width, height, record.value);
    let house = match kind.param_kind() {
        ParamKind::House => resolve_house(host, record.value),
        _ => resolve_house(host, -1),
    };
    let team = reference(record.value, record.team);
    let trigger = reference(record.value, record.trigger);

    let applied = match kind {
        ActionKind::None => false,
        ActionKind::Unknown(code) => {
            host.note(format!(
                "trigger action code {code} is outside the enumerated range; ignored"
            ));
            false
        }
        ActionKind::Win => host.win(house),
        ActionKind::Lose => host.lose(house),
        ActionKind::WinLose => host.win_lose(house),
        ActionKind::AllowWin => host.allow_win(house),
        ActionKind::BeginProduction => host.begin_production(house, team),
        ActionKind::CreateTeam => host.create_team(house, team),
        ActionKind::DestroyTeam => host.destroy_team(house, team),
        ActionKind::Autocreate => host.autocreate(house, team),
        ActionKind::Reinforcements => host.reinforcements(house, team),
        ActionKind::AllHunt => host.all_hunt(house),
        ActionKind::DropZone => host.drop_zone(house, cell),
        ActionKind::FireSale => host.fire_sale(house),
        ActionKind::PlayMovie => host.play_movie(record.value),
        ActionKind::TextTrigger => host.text_trigger(record.value),
        ActionKind::DestroyTrigger => host.destroy_trigger(trigger),
        ActionKind::ForceTrigger => host.force_trigger(trigger),
        ActionKind::RevealAll => host.reveal_map(house),
        ActionKind::RevealSome => host.reveal_area(house, cell),
        ActionKind::RevealZone => host.reveal_zone(house, cell),
        ActionKind::PlaySound => host.play_sound(record.value),
        ActionKind::PlayMusic => host.play_music(record.value),
        ActionKind::PlaySpeech => host.play_speech(record.value),
        ActionKind::StartTimer => host.start_timer(),
        ActionKind::StopTimer => host.stop_timer(),
        ActionKind::ExtendTimer => host.extend_timer(record.value),
        ActionKind::ShortenTimer => host.shorten_timer(record.value),
        ActionKind::SetTimer => host.set_timer(record.value),
        ActionKind::SetGlobal => host.set_global(record.value, true),
        ActionKind::ClearGlobal => host.set_global(record.value, false),
        ActionKind::BaseBuilding => host.base_building(house, record.value != 0),
        ActionKind::CreepShadow => host.creep_shadow(house),
        ActionKind::DestroyObject => host.destroy_object(),
        ActionKind::OneTimeSpecial => host.special_weapon(house, record.value, true),
        ActionKind::FullSpecial => host.special_weapon(house, record.value, false),
        ActionKind::PreferredTarget => host.preferred_target(house, record.value),
        ActionKind::LaunchNukes => host.launch_nukes(house),
    };
    if applied {
        ApplyReport::applied(kind)
    } else {
        ApplyReport::ignored(kind)
    }
}

/// Apply a list of action records in written order.
///
/// Order matters: `TACTION_CREATE_TEAM` before `TACTION_BEGIN_PRODUCTION` is
/// a different script from the reverse, so the slice order is preserved.
pub fn apply_actions<H: TriggerHost + ?Sized>(
    host: &mut H,
    actions: &[ActionRecord],
) -> Vec<ApplyReport> {
    actions
        .iter()
        .copied()
        .map(|record| apply_action_record(host, record))
        .collect()
}

/// True when every event record holds against the host's live state.
///
/// An empty event list holds (the original engine's "no condition" trigger
/// always fires). An event the engine cannot evaluate, or an unknown event
/// code, does not hold and is logged once.
pub fn events_hold<H: TriggerHost + ?Sized>(host: &mut H, events: &[EventRecord]) -> bool {
    for record in events {
        match record.kind {
            EventKind::None => return true,
            EventKind::Unknown(code) => {
                host.note(format!(
                    "trigger event code {code} is outside the enumerated range; condition not met"
                ));
                return false;
            }
            kind => match host.evaluate_event(kind, record.value) {
                Some(true) => {}
                Some(false) => return false,
                None => {
                    host.note(format!(
                        "trigger event {kind:?} is not evaluated by this engine; condition not met"
                    ));
                    return false;
                }
            },
        }
    }
    true
}

/// Fire a trigger: when every event holds, apply every action in order.
///
/// Returns `None` when the condition did not hold (nothing was applied).
pub fn fire_trigger<H: TriggerHost + ?Sized>(
    host: &mut H,
    events: &[EventRecord],
    actions: &[ActionRecord],
) -> Option<Vec<ApplyReport>> {
    if !events_hold(host, events) {
        return None;
    }
    Some(apply_actions(host, actions))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trigger_records::ActionKind;

    /// A host that records every call and reports success for everything it
    /// implements.
    #[derive(Default)]
    struct RecordingHost {
        calls: Vec<String>,
        notes: Vec<String>,
        /// The single event kind this fake can answer.
        known_event: Option<EventKind>,
        known_event_result: bool,
    }

    impl TriggerHost for RecordingHost {
        fn note(&mut self, message: String) {
            self.notes.push(message);
        }
        fn map_size(&self) -> (u32, u32) {
            (16, 16)
        }
        fn default_house(&self) -> u8 {
            0
        }
        fn house_count(&self) -> usize {
            2
        }
        fn win(&mut self, house: u8) -> bool {
            self.calls.push(format!("win:{house}"));
            true
        }
        fn lose(&mut self, house: u8) -> bool {
            self.calls.push(format!("lose:{house}"));
            true
        }
        fn win_lose(&mut self, house: u8) -> bool {
            self.calls.push(format!("win_lose:{house}"));
            true
        }
        fn allow_win(&mut self, house: u8) -> bool {
            self.calls.push(format!("allow_win:{house}"));
            true
        }
        fn begin_production(&mut self, house: u8, team: Option<u16>) -> bool {
            self.calls
                .push(format!("begin_production:{house}:{team:?}"));
            true
        }
        fn create_team(&mut self, house: u8, team: Option<u16>) -> bool {
            self.calls.push(format!("create_team:{house}:{team:?}"));
            true
        }
        fn destroy_team(&mut self, house: u8, team: Option<u16>) -> bool {
            self.calls.push(format!("destroy_team:{house}:{team:?}"));
            true
        }
        fn autocreate(&mut self, house: u8, team: Option<u16>) -> bool {
            self.calls.push(format!("autocreate:{house}:{team:?}"));
            true
        }
        fn reinforcements(&mut self, house: u8, team: Option<u16>) -> bool {
            self.calls.push(format!("reinforcements:{house}:{team:?}"));
            true
        }
        fn all_hunt(&mut self, house: u8) -> bool {
            self.calls.push(format!("all_hunt:{house}"));
            true
        }
        fn drop_zone(&mut self, house: u8, cell: Vec2i) -> bool {
            self.calls
                .push(format!("drop_zone:{house}:{}x{}", cell.x, cell.y));
            true
        }
        fn fire_sale(&mut self, house: u8) -> bool {
            self.calls.push(format!("fire_sale:{house}"));
            true
        }
        fn play_movie(&mut self, movie: i64) -> bool {
            self.calls.push(format!("play_movie:{movie}"));
            true
        }
        fn text_trigger(&mut self, text: i64) -> bool {
            self.calls.push(format!("text_trigger:{text}"));
            true
        }
        fn play_sound(&mut self, sound: i64) -> bool {
            self.calls.push(format!("play_sound:{sound}"));
            true
        }
        fn play_music(&mut self, theme: i64) -> bool {
            self.calls.push(format!("play_music:{theme}"));
            true
        }
        fn play_speech(&mut self, speech: i64) -> bool {
            self.calls.push(format!("play_speech:{speech}"));
            true
        }
        fn destroy_trigger(&mut self, trigger: Option<u16>) -> bool {
            self.calls.push(format!("destroy_trigger:{trigger:?}"));
            true
        }
        fn force_trigger(&mut self, trigger: Option<u16>) -> bool {
            self.calls.push(format!("force_trigger:{trigger:?}"));
            true
        }
        fn set_global(&mut self, global: i64, on: bool) -> bool {
            self.calls.push(format!("set_global:{global}:{on}"));
            true
        }
        fn start_timer(&mut self) -> bool {
            self.calls.push("start_timer".to_string());
            true
        }
        fn stop_timer(&mut self) -> bool {
            self.calls.push("stop_timer".to_string());
            true
        }
        fn extend_timer(&mut self, tenths: i64) -> bool {
            self.calls.push(format!("extend_timer:{tenths}"));
            true
        }
        fn shorten_timer(&mut self, tenths: i64) -> bool {
            self.calls.push(format!("shorten_timer:{tenths}"));
            true
        }
        fn set_timer(&mut self, tenths: i64) -> bool {
            self.calls.push(format!("set_timer:{tenths}"));
            true
        }
        fn base_building(&mut self, house: u8, enabled: bool) -> bool {
            self.calls.push(format!("base_building:{house}:{enabled}"));
            true
        }
        fn creep_shadow(&mut self, house: u8) -> bool {
            self.calls.push(format!("creep_shadow:{house}"));
            true
        }
        fn destroy_object(&mut self) -> bool {
            self.calls.push("destroy_object".to_string());
            true
        }
        fn reveal_map(&mut self, house: u8) -> bool {
            self.calls.push(format!("reveal_map:{house}"));
            true
        }
        fn reveal_area(&mut self, house: u8, cell: Vec2i) -> bool {
            self.calls
                .push(format!("reveal_area:{house}:{}x{}", cell.x, cell.y));
            true
        }
        fn reveal_zone(&mut self, house: u8, cell: Vec2i) -> bool {
            self.calls
                .push(format!("reveal_zone:{house}:{}x{}", cell.x, cell.y));
            true
        }
        fn special_weapon(&mut self, house: u8, weapon: i64, one_shot: bool) -> bool {
            self.calls
                .push(format!("special_weapon:{house}:{weapon}:{one_shot}"));
            true
        }
        fn preferred_target(&mut self, house: u8, quarry: i64) -> bool {
            self.calls
                .push(format!("preferred_target:{house}:{quarry}"));
            true
        }
        fn launch_nukes(&mut self, house: u8) -> bool {
            self.calls.push(format!("launch_nukes:{house}"));
            true
        }
        fn evaluate_event(&mut self, kind: EventKind, value: i64) -> Option<bool> {
            if self.known_event == Some(kind) {
                self.calls.push(format!("event:{kind:?}:{value}"));
                Some(self.known_event_result)
            } else {
                None
            }
        }
    }

    fn action(record: &str) -> ActionRecord {
        ActionRecord::parse(record).expect("record parses")
    }

    fn event(record: &str) -> EventRecord {
        EventRecord::parse(record).expect("record parses")
    }

    #[test]
    fn every_enumerated_action_code_reaches_an_arm() {
        for code in 0..crate::trigger_records::ACTION_CODE_COUNT {
            let record = action(&format!("{code},-1,-1,3"));
            let mut host = RecordingHost::default();
            let report = apply_action_record(&mut host, record);
            if record.kind == ActionKind::None {
                assert!(!report.applied, "-No Action- is a no-op");
                assert!(host.calls.is_empty());
            } else {
                assert!(report.applied, "code {code} must be executed");
                assert_eq!(host.calls.len(), 1, "code {code} dispatches once");
            }
            assert!(
                host.notes.is_empty(),
                "supported codes log nothing: {:?}",
                host.notes
            );
        }
    }

    #[test]
    fn unknown_action_code_is_a_logged_no_op() {
        let mut host = RecordingHost::default();
        let report = apply_action_record(&mut host, action("99,-1,-1,0"));
        assert!(!report.applied);
        assert_eq!(report.action, ActionKind::Unknown(99));
        assert!(host.calls.is_empty(), "no engine state is touched");
        assert_eq!(host.notes.len(), 1);
        assert!(host.notes[0].contains("99"), "{:?}", host.notes);
    }

    #[test]
    fn house_and_reference_payloads_resolve_per_record() {
        let mut host = RecordingHost::default();
        // TACTION_WIN has NEED_HOUSE: value 1 selects the second house.
        apply_action_record(&mut host, action("1,-1,-1,1"));
        assert_eq!(host.calls, vec!["win:1"]);

        // Out-of-range houses clamp instead of panicking.
        let mut host = RecordingHost::default();
        apply_action_record(&mut host, action("1,-1,-1,9"));
        assert_eq!(host.calls, vec!["win:1"]);

        // Missing house payload falls back to the host's default house.
        let mut host = RecordingHost::default();
        apply_action_record(&mut host, action("1,-1,-1,-1"));
        assert_eq!(host.calls, vec!["win:0"]);

        // NEED_TEAM payload: value is the team logical id.
        let mut host = RecordingHost::default();
        apply_action_record(&mut host, action("4,-1,-1,7"));
        assert_eq!(host.calls, vec!["create_team:0:Some(7)"]);

        // NEED_TRIGGER payload falls back to the record's trigger field.
        let mut host = RecordingHost::default();
        apply_action_record(&mut host, action("22,-1,5,-1"));
        assert_eq!(host.calls, vec!["force_trigger:Some(5)"]);
    }

    #[test]
    fn positional_actions_resolve_their_waypoint_to_a_cell() {
        // Waypoint 18 on a 16x16 map is (2, 1).
        let mut host = RecordingHost::default();
        apply_action_record(&mut host, action("8,-1,-1,18"));
        assert_eq!(host.calls, vec!["drop_zone:0:2x1"]);

        // Wrapping keeps a stale waypoint harmless.
        assert_eq!(waypoint_cell(16, 16, 256), Vec2i::new(0, 0));
        assert_eq!(waypoint_cell(16, 16, -1), Vec2i::new(15, 15));
    }

    #[test]
    fn special_weapon_actions_carry_the_spc_index_and_mode() {
        let mut host = RecordingHost::default();
        // TACTION_1_SPECIAL (33) with SPC_PARA_BOMB: one-shot airstrike.
        apply_action_record(&mut host, action("33,-1,-1,3"));
        // TACTION_FULL_SPECIAL (34) with SPC_NUCLEAR_BOMB: rechargeable nuke.
        apply_action_record(&mut host, action("34,-1,-1,1"));
        assert_eq!(
            host.calls,
            vec!["special_weapon:0:3:true", "special_weapon:0:1:false"]
        );
    }

    #[test]
    fn applying_the_same_records_twice_is_deterministic() {
        let records = [
            "1,-1,-1,0",
            "3,-1,-1,1",
            "8,-1,-1,4",
            "16,-1,-1,0",
            "28,-1,-1,5",
            "33,-1,-1,3",
            "99,-1,-1,0",
        ];
        let mut first = RecordingHost::default();
        let mut second = RecordingHost::default();
        for record in records {
            apply_action_record(&mut first, action(record));
            apply_action_record(&mut second, action(record));
        }
        assert_eq!(first.calls, second.calls);
        assert_eq!(first.notes, second.notes);
    }

    #[test]
    fn events_hold_only_when_every_event_holds() {
        let mut host = RecordingHost::default();
        assert!(events_hold(&mut host, &[]), "no condition always fires");

        host.known_event = Some(EventKind::ElapsedTime);
        host.known_event_result = true;
        assert!(events_hold(&mut host, &[event("13,-1,600")]));

        host.known_event_result = false;
        assert!(!events_hold(&mut host, &[event("13,-1,600")]));
    }

    #[test]
    fn unevaluable_and_unknown_events_do_not_fire() {
        let mut host = RecordingHost::default();
        // No known_event configured: the fake cannot answer this event.
        assert!(!events_hold(&mut host, &[event("13,-1,1")]));
        assert_eq!(host.notes.len(), 1);

        let mut host = RecordingHost::default();
        assert!(!events_hold(&mut host, &[event("200,-1,1")]));
        assert!(host.notes[0].contains("200"));
    }

    #[test]
    fn fire_trigger_applies_actions_only_when_the_condition_holds() {
        let actions = [action("1,-1,-1,0")];
        let mut host = RecordingHost::default();
        assert!(fire_trigger(&mut host, &[event("200,-1,1")], &actions).is_none());
        assert!(host.calls.is_empty());

        let mut host = RecordingHost::default();
        let reports = fire_trigger(&mut host, &[], &actions).expect("empty condition fires");
        assert_eq!(reports.len(), 1);
        assert!(reports[0].applied);
        assert_eq!(host.calls, vec!["win:0"]);
    }

    #[test]
    fn rejected_families_are_logged_not_silent() {
        // A bare host implements nothing but the required methods; every
        // action family must reject through note().
        struct BareHost {
            notes: Vec<String>,
        }
        impl TriggerHost for BareHost {
            fn note(&mut self, message: String) {
                self.notes.push(message);
            }
            fn map_size(&self) -> (u32, u32) {
                (8, 8)
            }
            fn default_house(&self) -> u8 {
                0
            }
            fn house_count(&self) -> usize {
                1
            }
        }
        let mut host = BareHost { notes: Vec::new() };
        let report = apply_action_record(&mut host, action("1,-1,-1,0"));
        assert!(!report.applied);
        assert_eq!(host.notes.len(), 1);
        assert!(host.notes[0].contains("no arm"), "{:?}", host.notes);
    }
}
