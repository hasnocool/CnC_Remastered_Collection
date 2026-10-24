//! Acceptance test for todo 42: in-game SFX + theme queue.
//!
//! A scripted TD combat scenario runs through the real engine with an
//! EventBus attached; the launcher-side [`AudioDirector`] routes the drained
//! events into a headless [`AudioMixer`]. Acceptance (plan line 420):
//! scripted combat emits >= 4 distinct SFX through the mixer
//! (`combat_emits_sfx`), and a base-attack event switches the theme layer.

use std::sync::Arc;

use cnc_audio::{AudioMixer, MusicLayer};
use cnc_core::Fixed16_16;
use cnc_events::EventBus;
use cnc_launcher::audio_director::{AudioDirector, ThemeQueue};
use cnc_launcher::events_subscriber::EventSubscriber;
use cnc_tiberian_dawn::Game;

/// Build a scripted combat scenario: the demo roster attacks an enemy, so
/// projectiles fly and units die (fire/explosion family events + stats).
fn setup_combat_scenario(game: &mut Game) {
    game.setup_demo();
    let enemy = game
        .units
        .iter()
        .position(|unit| unit.owner != game.player_owner)
        .expect("demo spawns Nod units");
    game.order_attack(enemy);
}

#[test]
fn combat_emits_sfx() {
    // Headless mixer records every accepted play request.
    let mixer = AudioMixer::headless();

    // Subscribe before the first tick: the bus retains no history.
    let bus = Arc::new(EventBus::new(4096));
    let mut subscriber = EventSubscriber::new(bus.clone());
    let mut director = AudioDirector::new();

    let mut game = Game::new("combat-test", 32, 32);
    game.setup_demo();
    game.set_event_bus(bus);
    setup_combat_scenario(&mut game);

    // Order a move so the move-acknowledged sound family is exercised too.
    game.order_move(cnc_core::Vec2::new(
        Fixed16_16::from_int(20),
        Fixed16_16::from_int(20),
    ));
    // Click-select a unit so the select beep is exercised.
    let pick = game
        .units
        .iter()
        .position(|unit| unit.owner == game.player_owner)
        .expect("demo spawns player units");
    let position = game.units[pick].position;
    game.select_at(position);

    // Run the combat and pump events through the director every tick.
    for _ in 0..600 {
        game.update(Fixed16_16::from_f32(0.05));
        let events = subscriber.drain();
        director.route_batch(&mixer, &events);
    }

    let plays = mixer.played();
    assert!(
        !plays.is_empty(),
        "combat must reach the mixer: routed {} subscriber events, \
         director accepted {} plays",
        subscriber.total(),
        director.plays()
    );

    // Acceptance: >= 4 distinct SFX (different sounds) through the mixer.
    assert!(
        director.distinct_sounds() >= 4,
        "expected >= 4 distinct SFX, got {:?}",
        director.heard_snapshot()
    );
}

#[test]
fn base_attack_switches_theme_layer() {
    let mixer = AudioMixer::headless();
    let mut themes = ThemeQueue::new();

    // Fresh queue sits on the explore layer at neutral intensity.
    assert_eq!(themes.player().layer(), MusicLayer::Explore);
    assert!(themes.play_song(&mixer).is_some(), "scan queued themes");

    // A base-attack event escalates to the combat layer...
    assert!(themes.on_base_attack(), "first attack switches the layer");
    assert_eq!(themes.player().layer(), MusicLayer::Combat);

    // ...and the next track comes from the combat roster.
    let combat_first = themes.next(&mixer).expect("combat track queued");
    assert!(
        themes.theme_names().1.contains(&combat_first),
        "{combat_first} must be a combat-layer theme"
    );

    // Combat resolved de-escalates back to explore.
    assert!(themes.on_combat_resolved());
    assert_eq!(themes.player().layer(), MusicLayer::Explore);
}
