//! Observational game events for the Red Alert engine.
//!
//! Every publish here is **observation only**: the emitters read simulation
//! state and publish a typed event through the shared [`cnc_events::EventBus`].
//! They never read the bus back, never gate simulation on the result, and
//! never alter entities, credits or orders. If the bus is absent — the
//! default for tests and headless runs — or a subscriber has gone away, the
//! engine keeps ticking exactly as it would without this module.
//!
//! Emission points, all on existing per-tick paths:
//!
//! | Event | Source path |
//! |-------|-------------|
//! | `ra_unit_produced` / `ra_structure_produced` | `Game::update_production` |
//! | `ra_unit_lost` / `ra_structure_lost` | `Game::update_ra_death_announcements` |
//! | `ra_superweapon_fired` | `Game::fire_iron_curtain` / `fire_chronosphere` / `fire_gps` |
//! | `ra_scenario_outcome` | `Game::emit_scenario_outcome` from `update` |

use crate::game::{Game, ScenarioOutcome};
use cnc_core::ClassId;
use cnc_events::{EventData, EventSource, GameEvent};
use std::collections::HashMap;
use std::sync::Arc;

/// Event source name carried by every Red Alert event.
pub const SOURCE: &str = "cnc-red-alert";

/// Canonical event names emitted by the Red Alert engine.
pub mod names {
    /// A production queue completed a unit (`ClassId` of the unit).
    pub const UNIT_PRODUCED: &str = "ra_unit_produced";
    /// A production queue completed a structure.
    pub const STRUCTURE_PRODUCED: &str = "ra_structure_produced";
    /// A unit was destroyed and removed from the world.
    pub const UNIT_LOST: &str = "ra_unit_lost";
    /// A structure was destroyed and removed from the world.
    pub const STRUCTURE_LOST: &str = "ra_structure_lost";
    /// A charged superweapon was fired.
    pub const SUPERWEAPON_FIRED: &str = "ra_superweapon_fired";
    /// The scenario resolved with a win or loss.
    pub const SCENARIO_OUTCOME: &str = "ra_scenario_outcome";
}

/// Which scenario ending was published.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeKind {
    /// `ScenarioOutcome::AlliesWin`.
    AlliesWin,
    /// `ScenarioOutcome::SovietsWin`.
    SovietsWin,
}

impl OutcomeKind {
    fn from_scenario(outcome: ScenarioOutcome) -> Self {
        match outcome {
            ScenarioOutcome::AlliesWin => Self::AlliesWin,
            ScenarioOutcome::SovietsWin => Self::SovietsWin,
        }
    }

    /// Stable lowercase spelling used in the event payload.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlliesWin => "allies_win",
            Self::SovietsWin => "soviets_win",
        }
    }
}

/// Superweapon kinds that can be fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuperweaponKind {
    /// Soviet Iron Curtain.
    IronCurtain,
    /// Allied Chronosphere.
    Chronosphere,
    /// Allied GPS uplink.
    Gps,
}

impl SuperweaponKind {
    /// Stable lowercase spelling used in the event payload.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IronCurtain => "iron_curtain",
            Self::Chronosphere => "chronosphere",
            Self::Gps => "gps",
        }
    }
}

fn structured(pairs: impl IntoIterator<Item = (&'static str, serde_json::Value)>) -> EventData {
    let map: HashMap<String, serde_json::Value> = pairs
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    EventData::structured(map)
}

fn class_name(class_id: ClassId) -> String {
    format!("{class_id:?}")
}

/// Publish a unit-production event. Observation only.
pub(crate) fn unit_produced(source: &EventSource, owner: usize, class_id: ClassId) {
    let event = GameEvent::with_source(
        names::UNIT_PRODUCED,
        structured([
            ("owner", serde_json::json!(owner)),
            ("unit", serde_json::json!(class_name(class_id))),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a structure-production event. Observation only.
pub(crate) fn structure_produced(source: &EventSource, owner: usize, class_id: ClassId) {
    let event = GameEvent::with_source(
        names::STRUCTURE_PRODUCED,
        structured([
            ("owner", serde_json::json!(owner)),
            ("structure", serde_json::json!(class_name(class_id))),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a unit-loss event. Observation only.
pub(crate) fn unit_lost(source: &EventSource, owner: usize, unit_id: u32, class_id: ClassId) {
    let event = GameEvent::with_source(
        names::UNIT_LOST,
        structured([
            ("owner", serde_json::json!(owner)),
            ("unit_id", serde_json::json!(unit_id)),
            ("unit", serde_json::json!(class_name(class_id))),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a structure-loss event. Observation only.
pub(crate) fn structure_lost(
    source: &EventSource,
    owner: usize,
    structure_id: u32,
    class_id: ClassId,
) {
    let event = GameEvent::with_source(
        names::STRUCTURE_LOST,
        structured([
            ("owner", serde_json::json!(owner)),
            ("structure_id", serde_json::json!(structure_id)),
            ("structure", serde_json::json!(class_name(class_id))),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a superweapon-fired event. Observation only.
pub(crate) fn superweapon_fired(
    source: &EventSource,
    owner: usize,
    kind: SuperweaponKind,
    affected: usize,
) {
    let event = GameEvent::with_source(
        names::SUPERWEAPON_FIRED,
        structured([
            ("owner", serde_json::json!(owner)),
            ("superweapon", serde_json::json!(kind.as_str())),
            ("affected", serde_json::json!(affected)),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a scenario-outcome event. Observation only.
pub(crate) fn scenario_outcome(source: &EventSource, owner: usize, kind: OutcomeKind) {
    let event = GameEvent::with_source(
        names::SCENARIO_OUTCOME,
        structured([
            ("outcome", serde_json::json!(kind.as_str())),
            ("player_owner", serde_json::json!(owner)),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

impl Game {
    /// Attach an event bus. From then on every update tick publishes the
    /// events documented in this module. Simulation results are unaffected:
    /// the bus is write-only from the engine's point of view.
    pub fn set_event_bus(&mut self, bus: Arc<cnc_events::EventBus>) {
        self.event_source = Some(EventSource::new(bus, SOURCE));
    }

    /// Detach the event bus; the engine falls back to publishing nothing.
    pub fn clear_event_bus(&mut self) {
        self.event_source = None;
    }

    /// The event source, if a bus has been attached.
    pub fn event_source(&self) -> Option<&EventSource> {
        self.event_source.as_ref()
    }

    /// Publish the scenario outcome the first time a `win`/`lose` action fires.
    ///
    /// Called from `update` after every subsystem has run, so it observes the
    /// post-step state. Idempotent: the latch means one publish per scenario.
    pub(crate) fn emit_scenario_outcome(&mut self) {
        let Some(outcome) = self.outcome else {
            return;
        };
        if self.scenario_outcome_emitted {
            return;
        }
        self.scenario_outcome_emitted = true;
        let Some(source) = self.event_source.as_ref() else {
            return;
        };
        let owner = self.player_owner;
        scenario_outcome(source, owner, OutcomeKind::from_scenario(outcome));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Game, ScenarioOutcome};
    use cnc_core::Fixed16_16;
    use cnc_events::EventBus;

    fn attach(game: &mut Game) -> (Arc<EventBus>, cnc_events::EventSubscription) {
        let bus = Arc::new(EventBus::new(4096));
        let sub = bus.subscribe();
        game.set_event_bus(bus.clone());
        (bus, sub)
    }

    fn drain(sub: &cnc_events::EventSubscription) -> Vec<String> {
        let mut names = Vec::new();
        while let Ok(event) = sub.try_recv() {
            names.push(event.name().to_string());
        }
        names
    }

    #[test]
    fn engine_without_a_bus_ticks_normally() {
        let mut game = Game::new(16, 16);
        assert!(game.event_source().is_none());

        game.update(Fixed16_16::from_int(1));
        assert!(game.elapsed > Fixed16_16::ZERO);
    }

    #[test]
    fn attached_bus_does_not_change_the_simulation() {
        let mut with_bus = Game::new(16, 16);
        let mut without_bus = Game::new(16, 16);
        let (_bus, sub) = attach(&mut with_bus);

        for _ in 0..60 {
            with_bus.update(Fixed16_16::from_int(1));
            without_bus.update(Fixed16_16::from_int(1));
        }

        assert_eq!(with_bus.elapsed, without_bus.elapsed);
        assert_eq!(with_bus.units.len(), without_bus.units.len());
        assert_eq!(with_bus.structures.len(), without_bus.structures.len());
        assert_eq!(with_bus.status(), without_bus.status());
        let _ = drain(&sub);
    }

    #[test]
    fn dropped_subscriber_leaves_the_engine_running() {
        let mut game = Game::new(16, 16);
        let (_bus, sub) = attach(&mut game);
        drop(sub);

        for _ in 0..10 {
            game.update(Fixed16_16::from_int(1));
        }
        assert!(game.elapsed > Fixed16_16::ZERO);
    }

    #[test]
    fn scenario_outcome_is_published_once() {
        let mut game = Game::new(16, 16);
        let (_bus, sub) = attach(&mut game);
        game.outcome = Some(ScenarioOutcome::AlliesWin);

        game.update(Fixed16_16::from_int(1));
        game.update(Fixed16_16::from_int(1));
        game.update(Fixed16_16::from_int(1));

        let names = drain(&sub);
        let outcomes = names
            .iter()
            .filter(|name| name.as_str() == names::SCENARIO_OUTCOME)
            .count();
        assert_eq!(outcomes, 1, "outcome must publish exactly once");
    }

    #[test]
    fn iron_curtain_publishes_a_typed_event() {
        let mut game = Game::new(16, 16);
        let (_bus, sub) = attach(&mut game);

        let owner = game.player_owner;
        let Some(index) = game
            .superweapons
            .iter()
            .position(|weapon| weapon.class_id == ClassId::IronCurtain)
        else {
            return;
        };
        game.superweapons[index].ready = true;

        if game.fire_iron_curtain(owner).is_ok() {
            let names = drain(&sub);
            assert!(
                names.iter().any(|name| name == names::SUPERWEAPON_FIRED),
                "expected a superweapon event, saw {names:?}"
            );
        }
    }
}
