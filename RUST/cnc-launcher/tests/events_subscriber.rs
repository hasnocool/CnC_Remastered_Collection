//! Integration tests for the launcher's engine-event subscriber.
//!
//! These exercise the real Tiberian Dawn engine: a Game is stepped with an
//! EventBus attached and the launcher-side EventSubscriber must observe the
//! typed events the engine publishes, in publish order.

use std::sync::Arc;

use cnc_core::{ClassId, Fixed16_16};
use cnc_events::{EventBus, EventSubscription};
use cnc_launcher::events_subscriber::EventSubscriber;
use cnc_tiberian_dawn::house::ProductionItem;
use cnc_tiberian_dawn::{event_names, Game, Mission, MissionOutcome};

/// Drain a raw subscription into the list of event names it saw, preserving
/// publish order.
fn raw_names(subscription: &EventSubscription) -> Vec<String> {
    let mut names = Vec::new();
    while let Ok(event) = subscription.try_recv() {
        names.push(event.name().to_string());
    }
    names
}

/// A finished mission is published from Game::update on the next tick, which
/// gives the test at least one deterministic engine event.
fn finish_mission(game: &mut Game, name: &str) {
    let mut mission = Mission::new(name.to_string());
    mission.finish(MissionOutcome::Victory);
    game.mission = mission;
}

#[test]
fn engine_events_reach_launcher_subscriber() {
    let bus = Arc::new(EventBus::new(4096));

    // Both observers subscribe before the engine can publish: the bus is a
    // live channel and retains no history.
    let mut subscriber = EventSubscriber::new(bus.clone());
    let reference = bus.subscribe();

    let mut game = Game::new("launcher-events", 16, 16);
    game.setup_demo();
    game.set_event_bus(bus.clone());
    finish_mission(&mut game, "launcher-events");

    // Queue a unit that completes within this tick, so the tick can publish
    // more than the mission outcome.
    let owner = game.player_owner as usize;
    game.houses[owner].queue_production(ProductionItem {
        class_id: ClassId::InfantryE1,
        progress: Fixed16_16::ZERO,
        total_time: Fixed16_16::from_int(1),
        cost: 0,
    });

    let units_before = game.units.len();
    for _ in 0..3 {
        game.update(Fixed16_16::from_int(5));
    }

    let received = subscriber.drain();
    let expected_order = raw_names(&reference);

    // The launcher subscriber sees exactly what a raw bus subscriber sees, in
    // the same order.
    let seen: Vec<String> = received.iter().map(|event| event.name.clone()).collect();
    assert_eq!(
        seen, expected_order,
        "launcher subscriber must observe events in publish order"
    );
    assert!(
        !received.is_empty(),
        "expected the engine to publish at least one event"
    );

    // The mission outcome is the deterministic engine event this test drives.
    assert_eq!(
        subscriber.count_of(event_names::MISSION_OUTCOME),
        1,
        "a finished mission publishes exactly one outcome event, saw {seen:?}"
    );
    let outcome = received
        .iter()
        .find(|event| event.name == event_names::MISSION_OUTCOME)
        .expect("mission outcome event must reach the launcher subscriber");
    assert_eq!(outcome.source, "cnc-tiberian-dawn");

    // Unit production, when it completes, is observed before the outcome that
    // the same tick publishes at its end.
    if game.units.len() > units_before {
        assert_eq!(
            seen.first().map(String::as_str),
            Some(event_names::UNIT_PRODUCED),
            "the produced unit must be the first observed event, saw {seen:?}"
        );
        assert!(
            seen.iter()
                .position(|name| name == event_names::UNIT_PRODUCED)
                < seen
                    .iter()
                    .position(|name| name == event_names::MISSION_OUTCOME),
            "unit production is published before the mission outcome"
        );
    }

    // Counters and the bounded recent buffer agree with the drained batch.
    assert_eq!(subscriber.total(), received.len() as u64);
    assert_eq!(
        subscriber.counts().values().sum::<u64>(),
        subscriber.total()
    );
    assert_eq!(subscriber.recent().count(), received.len());
    assert_eq!(
        subscriber.last().map(|event| event.name.as_str()),
        received.last().map(|event| event.name.as_str())
    );
    assert!(!subscriber.is_empty());
}

#[test]
fn subscriber_drains_in_publish_order() {
    let bus = Arc::new(EventBus::new(256));
    let mut subscriber = EventSubscriber::new(bus.clone());

    for name in ["td_unit_produced", "td_unit_lost", "td_mission_outcome"] {
        bus.publish(cnc_events::GameEvent::new(
            name,
            cnc_events::EventData::message("launcher test"),
        ))
        .expect("bus accepts publishes");
    }

    let received = subscriber.drain();
    let seen: Vec<&str> = received.iter().map(|event| event.name.as_str()).collect();
    assert_eq!(
        seen,
        ["td_unit_produced", "td_unit_lost", "td_mission_outcome"],
        "drain must preserve publish order"
    );
    assert_eq!(subscriber.count_of("td_unit_lost"), 1);
    assert!(
        subscriber.drain().is_empty(),
        "second drain has nothing left"
    );
}

#[test]
fn subscriber_ignores_a_quiet_engine_tick() {
    let bus = Arc::new(EventBus::new(256));
    let mut subscriber = EventSubscriber::new(bus.clone());

    let mut game = Game::new("quiet", 16, 16);
    game.setup_demo();
    game.set_event_bus(bus.clone());

    // No mission outcome, no production: the engine must not publish.
    game.update(Fixed16_16::from_int(1));
    assert!(subscriber.drain().is_empty());
    assert!(subscriber.is_empty());
}
