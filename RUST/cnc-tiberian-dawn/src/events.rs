//! Observational game events for the Tiberian Dawn engine.
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
//! | `td_unit_produced` | `Game::update_production` (unit branch) |
//! | `td_structure_ready` | `Game::update_production` (structure branch) |
//! | `td_unit_lost` / `td_structure_lost` | `Game::update_death_announcements` |
//! | `td_structure_captured` | `Game::update_engineers` capture loop |
//! | `td_superweapon_fired` | `Game::fire_ion_cannon` / `fire_nuke` |
//! | `td_mission_outcome` | `Game::emit_mission_outcome` from `update` |

use crate::game::Game;
use crate::mission::MissionOutcome;
use crate::td_rules::{StructureType, UnitType};
use cnc_core::Vec2i;
use cnc_events::{EventData, EventSource, GameEvent};
use std::collections::HashMap;
use std::sync::Arc;

/// Event source name carried by every TD event.
pub const SOURCE: &str = "cnc-tiberian-dawn";

/// Canonical event names emitted by the Tiberian Dawn engine.
pub mod names {
    /// A production queue completed a unit and it entered the world.
    pub const UNIT_PRODUCED: &str = "td_unit_produced";
    /// A production queue completed a structure awaiting placement.
    pub const STRUCTURE_READY: &str = "td_structure_ready";
    /// A unit was destroyed.
    pub const UNIT_LOST: &str = "td_unit_lost";
    /// A structure was destroyed.
    pub const STRUCTURE_LOST: &str = "td_structure_lost";
    /// An engineer took over an enemy structure.
    pub const STRUCTURE_CAPTURED: &str = "td_structure_captured";
    /// An Ion Cannon / Nuclear Missile strike was fired.
    pub const SUPERWEAPON_FIRED: &str = "td_superweapon_fired";
    /// The campaign mission resolved as a victory or defeat.
    pub const MISSION_OUTCOME: &str = "td_mission_outcome";
}

/// Which mission ending was published.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutcomeKind {
    /// `MissionOutcome::Victory`.
    Victory,
    /// `MissionOutcome::Defeat`.
    Defeat,
}

impl OutcomeKind {
    fn from_mission(outcome: MissionOutcome) -> Self {
        match outcome {
            MissionOutcome::Victory => Self::Victory,
            MissionOutcome::Defeat => Self::Defeat,
        }
    }

    /// Stable lowercase spelling used in the event payload.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Victory => "victory",
            Self::Defeat => "defeat",
        }
    }
}

/// Superweapon kinds that can be fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuperweaponKind {
    /// GDI Ion Cannon.
    IonCannon,
    /// Nod Nuclear Missile.
    NuclearMissile,
}

impl SuperweaponKind {
    /// Stable lowercase spelling used in the event payload.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::IonCannon => "ion_cannon",
            Self::NuclearMissile => "nuclear_missile",
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

/// Publish a unit-production event. Observation only.
pub(crate) fn unit_produced(source: &EventSource, owner: u8, unit: UnitType, unit_id: u32) {
    let event = GameEvent::with_source(
        names::UNIT_PRODUCED,
        structured([
            ("owner", serde_json::json!(owner)),
            ("unit", serde_json::json!(unit.ini_name())),
            ("unit_id", serde_json::json!(unit_id)),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a structure-ready event. Observation only.
pub(crate) fn structure_ready(source: &EventSource, owner: u8, structure: StructureType) {
    let event = GameEvent::with_source(
        names::STRUCTURE_READY,
        structured([
            ("owner", serde_json::json!(owner)),
            ("structure", serde_json::json!(structure.ini_name())),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a unit-loss event. Observation only.
pub(crate) fn unit_lost(source: &EventSource, owner: u8, unit: UnitType, index: usize) {
    let event = GameEvent::with_source(
        names::UNIT_LOST,
        structured([
            ("owner", serde_json::json!(owner)),
            ("unit", serde_json::json!(unit.ini_name())),
            ("unit_index", serde_json::json!(index)),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a structure-loss event. Observation only.
pub(crate) fn structure_lost(source: &EventSource, owner: u8, structure: String, index: usize) {
    let event = GameEvent::with_source(
        names::STRUCTURE_LOST,
        structured([
            ("owner", serde_json::json!(owner)),
            ("structure", serde_json::json!(structure)),
            ("structure_index", serde_json::json!(index)),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a structure-captured event. Observation only.
pub(crate) fn structure_captured(
    source: &EventSource,
    previous_owner: u8,
    new_owner: u8,
    structure: String,
    index: usize,
) {
    let event = GameEvent::with_source(
        names::STRUCTURE_CAPTURED,
        structured([
            ("previous_owner", serde_json::json!(previous_owner)),
            ("owner", serde_json::json!(new_owner)),
            ("structure", serde_json::json!(structure)),
            ("structure_index", serde_json::json!(index)),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a superweapon-fired event. Observation only.
pub(crate) fn superweapon_fired(
    source: &EventSource,
    owner: u8,
    kind: SuperweaponKind,
    cell: Vec2i,
) {
    let event = GameEvent::with_source(
        names::SUPERWEAPON_FIRED,
        structured([
            ("owner", serde_json::json!(owner)),
            ("superweapon", serde_json::json!(kind.as_str())),
            ("cell_x", serde_json::json!(cell.x)),
            ("cell_y", serde_json::json!(cell.y)),
        ]),
        source.source_name(),
    );
    let _ = source.publish(event);
}

/// Publish a mission-outcome event. Observation only.
pub(crate) fn mission_outcome(source: &EventSource, owner: u8, kind: OutcomeKind, mission: &str) {
    let event = GameEvent::with_source(
        names::MISSION_OUTCOME,
        structured([
            ("outcome", serde_json::json!(kind.as_str())),
            ("player_owner", serde_json::json!(owner)),
            ("mission", serde_json::json!(mission)),
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

    /// Publish the mission outcome the first time the mission resolves.
    ///
    /// Called from `update` after every subsystem has run, so it observes the
    /// post-step state. Idempotent: the latch means one publish per mission.
    pub(crate) fn emit_mission_outcome(&mut self) {
        let Some(outcome) = self.mission.outcome else {
            return;
        };
        if self.mission_outcome_emitted {
            return;
        }
        self.mission_outcome_emitted = true;
        let Some(source) = self.event_source.as_ref() else {
            return;
        };
        mission_outcome(
            source,
            self.player_owner,
            OutcomeKind::from_mission(outcome),
            &self.mission.name,
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::mission::Mission;
    use cnc_core::{ClassId, Fixed16_16};
    use cnc_events::EventBus;

    /// Attach a bus and return it together with an already-registered
    /// subscription, so no event can be published before we listen.
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
        let mut game = Game::new("no-bus", 16, 16);
        game.setup_demo();
        assert!(game.event_source().is_none());

        // A full tick must not panic and must advance exactly as before.
        game.update(Fixed16_16::from_int(1));
        assert!(game.elapsed > Fixed16_16::ZERO);
    }

    #[test]
    fn attached_bus_does_not_change_the_simulation() {
        let mut with_bus = Game::new("with-bus", 16, 16);
        with_bus.setup_demo();
        let mut without_bus = Game::new("with-bus", 16, 16);
        without_bus.setup_demo();

        let (_bus, sub) = attach(&mut with_bus);
        for _ in 0..60 {
            with_bus.update(Fixed16_16::from_int(1));
            without_bus.update(Fixed16_16::from_int(1));
        }

        // Publishing is observation only: identical state on both sides.
        assert_eq!(with_bus.elapsed, without_bus.elapsed);
        assert_eq!(with_bus.units.len(), without_bus.units.len());
        assert_eq!(with_bus.buildings.len(), without_bus.buildings.len());
        assert_eq!(with_bus.units_produced, without_bus.units_produced);
        assert_eq!(with_bus.houses[0].credits, without_bus.houses[0].credits);

        // ...and the observer still saw nothing during a quiet tick.
        let _ = drain(&sub);
    }

    #[test]
    fn dropped_subscriber_leaves_the_engine_running() {
        let mut game = Game::new("drop", 16, 16);
        game.setup_demo();
        let (_bus, sub) = attach(&mut game);
        drop(sub);

        // Publishing into a bus with no live receivers must be inert.
        for _ in 0..10 {
            game.update(Fixed16_16::from_int(1));
        }
        assert!(game.elapsed > Fixed16_16::ZERO);
    }

    #[test]
    fn mission_outcome_is_published_once() {
        let mut game = Game::new("outcome", 16, 16);
        game.setup_demo();
        let (_bus, sub) = attach(&mut game);

        let mut mission = Mission::new("test-mission".to_string());
        mission.finish(MissionOutcome::Victory);
        game.mission = mission;

        game.update(Fixed16_16::from_int(1));
        game.update(Fixed16_16::from_int(1));
        game.update(Fixed16_16::from_int(1));

        let names = drain(&sub);
        let outcomes = names
            .iter()
            .filter(|name| name.as_str() == names::MISSION_OUTCOME)
            .count();
        assert_eq!(outcomes, 1, "outcome must publish exactly once");
    }

    #[test]
    fn unit_production_publishes_a_typed_event() {
        let mut game = Game::new("produce", 16, 16);
        game.setup_demo();
        let (_bus, sub) = attach(&mut game);

        // Force a completed unit straight into the player's queue.
        let owner = game.player_owner as usize;
        let before = game.units.len();
        game.houses[owner].queue_production(crate::house::ProductionItem {
            class_id: ClassId::InfantryE1,
            progress: Fixed16_16::ZERO,
            total_time: Fixed16_16::from_int(1),
            cost: 0,
        });
        game.update_production(Fixed16_16::from_int(5));

        let names = drain(&sub);
        if game.units.len() > before {
            assert!(
                names.iter().any(|name| name == names::UNIT_PRODUCED),
                "expected a unit-production event, saw {names:?}"
            );
        }
    }

    #[test]
    fn superweapon_fire_publishes_a_typed_event() {
        let mut game = Game::new("sw", 16, 16);
        game.setup_demo();
        let (_bus, sub) = attach(&mut game);

        let owner = game.player_owner;
        if let Some(house) = game.houses.get_mut(owner as usize) {
            house.ion_charge = Fixed16_16::ONE;
        }
        if game.fire_ion_cannon(owner, cnc_core::Vec2i::new(8, 8)) {
            let names = drain(&sub);
            assert!(
                names.iter().any(|name| name == names::SUPERWEAPON_FIRED),
                "expected a superweapon event, saw {names:?}"
            );
        }
    }
}
