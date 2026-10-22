//! Todo 19: TD economy fidelity tests (handicap, credit tick, alliances).

use cnc_core::FloatBias;
use cnc_core::{ClassId, Fixed16_16};
use cnc_tiberian_dawn::game::Game;
use cnc_tiberian_dawn::house::{House, ProductionItem};
use cnc_tiberian_dawn::UnitType;

#[test]
fn handicap_alters_economy() {
    let mut h0 = House::new(ClassId::HouseGDI, "GDI".to_string());
    h0.add_credits(10000);
    let item0 = ProductionItem {
        class_id: ClassId::InfantryE1,
        progress: Fixed16_16::ZERO,
        total_time: Fixed16_16::from_int(100),
        cost: 1000,
    };
    assert!(h0.queue_production(item0));
    let spent0 = h0.credits_spent;
    let time0 = h0.production_queue.front().unwrap().total_time;

    let mut h1 = House::new(ClassId::HouseGDI, "GDI".to_string());
    h1.add_credits(10000);
    h1.set_handicap(FloatBias::HALF);
    let item1 = ProductionItem {
        class_id: ClassId::InfantryE1,
        progress: Fixed16_16::ZERO,
        total_time: Fixed16_16::from_int(100),
        cost: 1000,
    };
    assert!(h1.queue_production(item1));
    let spent1 = h1.credits_spent;
    let time1 = h1.production_queue.front().unwrap().total_time;

    assert!(spent1 < spent0);
    assert!(time1.to_f32() < time0.to_f32());
    assert_eq!(spent1, 500);
    assert_eq!(time1.to_f32(), 50.0);

    // Bias zero identical to unhandicapped.
    let mut hz = House::new(ClassId::HouseGDI, "GDI".to_string());
    hz.add_credits(10000);
    assert!(!hz.has_handicap());
    let itemz = ProductionItem {
        class_id: ClassId::InfantryE1,
        progress: Fixed16_16::ZERO,
        total_time: Fixed16_16::from_int(100),
        cost: 1000,
    };
    assert!(hz.queue_production(itemz));
    assert_eq!(hz.credits_spent, 1000);
    assert_eq!(
        hz.production_queue.front().unwrap().total_time,
        Fixed16_16::from_int(100)
    );
}

#[test]
fn credit_display_rolls_up_gradually() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.capacity = 1000;
    h.harvested(1000);
    assert_eq!(h.available_money(), 1000);
    assert_eq!(h.credit_display_value(), 0);

    h.tick_credits();
    assert!(h.credit_display_value() > 0);
    assert!(h.credit_display_value() < 1000);

    h.credit_ticks(1000);
    assert_eq!(h.credit_display_value(), 1000);
}

#[test]
fn ally_event_sets_alliance_in_both_houses() {
    let mut game = Game::new("ally", 16, 16);
    game.setup_demo();
    assert!(!game.houses[0].is_ally(1));
    assert!(!game.houses[1].is_ally(0));

    assert!(game.apply_ally_event(0, 1));
    assert!(game.houses[0].is_ally(1));
    assert!(game.houses[1].is_ally(0));
    assert!(game.houses[0].allies_list().contains(&1));
    assert!(game.houses[1].allies_list().contains(&0));
    assert!(game.houses[0].alliance_members_list().contains(&1));
}

#[test]
fn non_ally_event_does_not_create_alliance() {
    let mut game = Game::new("noally", 16, 16);
    game.setup_demo();
    game.tick_credits();
    assert!(!game.houses[0].is_ally(1));
    assert!(!game.houses[1].is_ally(0));
    assert!(!game.apply_ally_event(0, 99));
    assert!(!game.houses[0].is_ally(99));
}

#[test]
fn credit_tick_malformed_inputs_are_noops() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.capacity = 100;
    h.harvested(100);
    let before = h.credit_display_value();
    h.credit_ticks(-5);
    assert_eq!(h.credit_display_value(), before);
    h.credit_ticks(0);
    assert_eq!(h.credit_display_value(), before);
    h.credit_ticks(i32::MAX);
    assert!(h.credit_display_value() >= 0);
    assert!(h.credit_display_value() <= h.available_money().max(0));
}

#[test]
fn handicap_out_of_range_bias_is_noop() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.add_credits(1000);
    h.set_handicap(FloatBias::from_f32(f32::NAN));
    let item = ProductionItem {
        class_id: ClassId::InfantryE1,
        progress: Fixed16_16::ZERO,
        total_time: Fixed16_16::from_int(10),
        cost: 100,
    };
    assert!(h.queue_production(item));
    assert_eq!(h.credits_spent, 100);

    let mut h2 = House::new(ClassId::HouseGDI, "GDI".to_string());
    h2.add_credits(1000);
    h2.set_handicap(FloatBias::from_f32(-0.5));
    let item2 = ProductionItem {
        class_id: ClassId::InfantryE1,
        progress: Fixed16_16::ZERO,
        total_time: Fixed16_16::from_int(10),
        cost: 100,
    };
    assert!(h2.queue_production(item2));
    assert_eq!(h2.credits_spent, 100);
}

#[test]
fn snapshot_roundtrips_credit_display_and_handicap() {
    let mut game = Game::new("snap", 8, 8);
    game.setup_demo();
    game.houses[0].set_handicap(FloatBias::HALF);
    game.houses[0].capacity = 1000;
    game.houses[0].harvested(1000);
    game.houses[0].tick_credits();
    game.houses[0].credit_ticks(3);
    let display = game.houses[0].credit_display_value();
    let handicap = game.houses[0].handicap_value();

    let dir = std::env::temp_dir();
    let path = dir.join("todo19_snap_test.bin");
    game.save_to_file(&path).unwrap();
    let loaded = Game::load_from_file(&path).unwrap();
    assert_eq!(loaded.houses[0].credit_display_value(), display);
    assert_eq!(loaded.houses[0].handicap_value(), handicap);
    let loaded2 = Game::load_from_file(&path).unwrap();
    assert_eq!(
        loaded2.houses[0].credit_display_value(),
        loaded.houses[0].credit_display_value()
    );
    let _ = std::fs::remove_file(&path);
}

#[test]
fn credit_tick_applied_twice_does_not_double_count() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.capacity = 100;
    h.harvested(100);
    h.force_credit_display();
    assert_eq!(h.credit_display_value(), 100);
    h.tick_credits();
    h.tick_credits();
    assert_eq!(h.credit_display_value(), 100);
}

#[test]
fn stale_state_snapshot_mid_accumulation_restores() {
    let mut game = Game::new("stale", 8, 8);
    game.setup_demo();
    game.houses[0].credits = 0;
    game.houses[0].capacity = 500;
    game.houses[0].harvested(500);
    game.houses[0].credit_ticks(5);
    let display = game.houses[0].credit_display_value();
    assert!(display > 0 && display < 500);

    let dir = std::env::temp_dir();
    let path = dir.join("todo19_stale_test.bin");
    game.save_to_file(&path).unwrap();
    let loaded = Game::load_from_file(&path).unwrap();
    assert_eq!(loaded.houses[0].credit_display_value(), display);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn handicap_zero_identical_to_unhandicapped() {
    let mut a = House::new(ClassId::HouseGDI, "GDI".to_string());
    a.add_credits(5000);
    let mut b = House::new(ClassId::HouseGDI, "GDI".to_string());
    b.add_credits(5000);
    b.set_handicap(FloatBias::ZERO);
    let ia = ProductionItem {
        class_id: ClassId::InfantryE1,
        progress: Fixed16_16::ZERO,
        total_time: Fixed16_16::from_int(50),
        cost: 250,
    };
    let ib = ProductionItem {
        class_id: ClassId::InfantryE1,
        progress: Fixed16_16::ZERO,
        total_time: Fixed16_16::from_int(50),
        cost: 250,
    };
    assert!(a.queue_production(ia));
    assert!(b.queue_production(ib));
    assert_eq!(a.credits_spent, b.credits_spent);
    assert_eq!(
        a.production_queue.front().unwrap().total_time,
        b.production_queue.front().unwrap().total_time
    );
    assert_eq!(a.available_money(), b.available_money());
}

#[test]
fn ally_event_is_idempotent_and_does_not_toggle() {
    let mut game = Game::new("toggle", 8, 8);
    game.setup_demo();
    game.apply_ally_event(0, 1);
    assert!(game.houses[0].is_ally(1));
    assert!(game.houses[1].is_ally(0));

    // HOUSE.CPP:2095 — Make_Ally is guarded by `!Is_Ally(house)`, so a repeat
    // ALLY event is a no-op. There is no toggle in the original.
    assert!(game.apply_ally_event(0, 1));
    assert!(game.houses[0].is_ally(1));
    assert!(game.houses[1].is_ally(0));
    assert_eq!(game.houses[0].allies_list().len(), 1);

    // Breaking is a separate, bilateral action (HOUSE.CPP:2158-2176).
    assert!(game.break_alliance(0, 1));
    assert!(!game.houses[0].is_ally(1));
    assert!(!game.houses[1].is_ally(0));
    // Breaking again is a no-op once already at war.
    assert!(!game.break_alliance(0, 1));
}

#[test]
fn handicap_applies_at_game_production_entry_points() {
    // FACTORY.CPP:402: the buyer pays Cost_Of() * house.CostBias, so the
    // Game-level queue entry points must scale the price and the build time,
    // not just House::queue_production.
    let mut plain = Game::new("plain", 8, 8);
    plain.setup_demo();
    plain.houses[0].add_credits(10_000);
    let raw_cost = plain.unit_cost(UnitType::InfantryE1);
    assert!(plain.queue_unit(0, UnitType::InfantryE1));
    let spent_plain = plain.houses[0].credits_spent;
    let time_plain = plain.houses[0].production_queue.front().unwrap().total_time;
    assert_eq!(spent_plain, raw_cost as i64);

    let mut easy = Game::new("easy", 8, 8);
    easy.setup_demo();
    easy.houses[0].add_credits(10_000);
    easy.houses[0].set_handicap(FloatBias::HALF);
    assert!(easy.queue_unit(0, UnitType::InfantryE1));
    let spent_easy = easy.houses[0].credits_spent;
    let time_easy = easy.houses[0].production_queue.front().unwrap().total_time;

    // The pinned FloatBias::HALF ratio, applied to both price and time.
    let expected = ((raw_cost as f64) * FloatBias::HALF.to_f32() as f64).round() as i64;
    assert_eq!(spent_easy, expected);
    assert_eq!(easy.production_cost(0, raw_cost), expected as i32);
    assert_eq!(spent_easy * 2, spent_plain);
    assert!((time_easy.to_f32() * 2.0 - time_plain.to_f32()).abs() < 1e-3);
}

#[test]
fn handicap_is_consistent_across_predicate_and_bias() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.set_handicap(FloatBias::from_f32(f32::NAN));
    assert!(!h.has_handicap());
    assert_eq!(h.handicap_cost(100), 100);
    assert_eq!(h.cost_bias(), FloatBias::ONE);

    h.set_handicap(FloatBias::from_f32(-2.0));
    assert!(!h.has_handicap());
    assert_eq!(h.handicap_cost(100), 100);
    assert_eq!(h.cost_bias(), FloatBias::ONE);

    h.set_handicap(FloatBias::HALF);
    assert!(h.has_handicap());
    assert!(h.bias_is_effective());
    assert_eq!(h.handicap_cost(100), 50);
    assert_eq!(h.cost_bias(), FloatBias::HALF);
}

#[test]
fn ally_house_index_out_of_bounds_is_noop() {
    let mut game = Game::new("oob", 8, 8);
    game.setup_demo();
    assert!(!game.ally_houses(0, 99));
    assert!(!game.ally_houses(99, 0));
    assert!(!game.make_alliance(5, 5));
}

#[test]
fn handicap_value_accessors() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.set_handicap(FloatBias::HALF);
    assert!(h.has_handicap());
    assert_eq!(h.handicap_value(), FloatBias::HALF);
    assert_eq!(h.handicap_bias(), FloatBias::HALF);
    assert_eq!(h.cost_bias(), FloatBias::HALF);
    assert_eq!(h.build_speed_bias(), FloatBias::HALF);
    h.set_handicap(FloatBias::ZERO);
    assert!(!h.has_handicap());
    assert_eq!(h.cost_bias(), FloatBias::ONE);
}

#[test]
fn credit_tick_n_aliases_work() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.capacity = 10;
    h.harvested(10);
    h.tick_credits_n(2);
    h.tick_n_credits(1);
    h.credits_tick(1);
    h.credits_ticks(1);
    h.apply_credit_ticks(1);
    h.tick_credit(1);
    h.advance_credit_display();
    h.update_credit_display();
    assert!(h.credit_display_value() >= 0);
}

#[test]
fn alliance_aliases_work() {
    let mut game = Game::new("alias", 8, 8);
    game.setup_demo();
    game.make_alliance(0, 1);
    assert!(game.houses[0].is_allied(1));
    game.set_alliance(0, 1);
    game.ally_event(0, 1);
    game.houses[0].make_allied(1);
    game.houses[1].add_ally(0);
    game.houses[0].add_alliance(1);
    game.houses[0].set_ally(1);
    game.houses[0].ally(1);
    assert!(game.houses[0].is_ally(1));
}

#[test]
fn credit_display_force_and_reset() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.capacity = 7;
    h.harvested(7);
    h.force_credits();
    assert_eq!(h.credit_display_value(), 7);
    h.reset_credit_display();
    assert_eq!(h.credit_display_value(), 7);
    h.force_credit_display();
    assert_eq!(h.displayed_credits(), 7);
    assert_eq!(h.credits_display(), 7);
    assert_eq!(h.current_credits(), 7);
}

#[test]
fn handicap_cost_rounds_and_clamps() {
    let mut h = House::new(ClassId::HouseGDI, "GDI".to_string());
    h.set_handicap(FloatBias::from_f32(0.3));
    assert_eq!(h.handicap_cost(10), (10.0_f32 * 0.3).round() as i32);
}

#[test]
fn float_bias_easy_ratio_pinned() {
    // Easy handicap uses the pinned FloatBias::HALF constant.
    assert_eq!(FloatBias::HALF.to_f32(), 0.5);
    assert_eq!(FloatBias::ZERO.to_f32(), 0.0);
    assert_eq!(FloatBias::ONE.to_f32(), 1.0);
}
