//! TD original-campaign progression + carry-over (plan todo 13, TD half).
//!
//! Failing-first suite: every test below was written before the production
//! `campaign` module existed and captured failing. They drive the flow through
//! real `cnc_tiberian-dawn` games and the real `CNCCARRY` file.
//!
//! Scope: orchestration only (mission order, score screen, carry-over write at
//! mission end / load at next start, completion screen). Combat, scoring and
//! the carry-over codec belong to `cnc-tiberian-dawn` / `score.rs`.

use std::path::{Path, PathBuf};
use std::process;

use cnc_core::rules::RulesDatabase;
use cnc_core::{Fixed16_16, Vec2};
use cnc_launcher::campaign::{self, CampaignFlow, FlowStep};
use cnc_launcher::score::{compute_score, DIFFICULTY_NEUTRAL_BP};
use cnc_tiberian_dawn::carryover::CARRY_MAGIC;
use cnc_tiberian_dawn::{Carryover, Game, Unit, UnitType};

/// TD tile size (cnc-tiberian-dawn/src/map.rs:17), so cell positions match the
/// engine's own cell-centre math without reaching into private helpers.
const TILE_SIZE: f32 = 24.0;

/// Fresh, uniquely named scratch directory; leftovers are removed up front so
/// a re-run cannot observe a previous test's files.
fn workspace(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("todo13-{label}-{}", process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// An original-format TD scenario INI (the `[Basic]` + `[Wunits]` dialect the
/// todo 8/9 loader recognises).
fn write_original(dir: &Path, stem: &str, name: &str, credits: i32) -> PathBuf {
    std::fs::write(
        dir.join(format!("{stem}.ini")),
        format!(
            "[Basic]\nName={name}\nPlayer=0\nCredits={credits}\nTechLevel=7\n\
             Theater=Temperate\nBriefing=Hold the ridge.\n\
             [Objectives]\n1=Survive the assault\n\
             [Houses]\n0=GDI\n1=Nod\n[Wunits]\n[Structures]\n"
        ),
    )
    .unwrap();
    dir.join(format!("{stem}.ini"))
}

/// A real TD game: deterministic board, two houses, demo torn down so only the
/// units a test adds are candidates for carry-over.
fn new_game() -> Game {
    let mut game = Game::with_rules("campaign test", 64, 64, RulesDatabase::default());
    game.setup_demo();
    game.units.clear();
    game
}

/// Spawn a live player unit at a cell centre (passable in every test board).
fn push_unit(game: &mut Game, unit_type: UnitType, cell_x: i32, cell_y: i32, owner: u8) {
    let position = Vec2::new(
        Fixed16_16::from_f32(cell_x as f32 * TILE_SIZE + TILE_SIZE * 0.5),
        Fixed16_16::from_f32(cell_y as f32 * TILE_SIZE + TILE_SIZE * 0.5),
    );
    game.units.push(Unit::new(unit_type, position, owner));
}

fn owned_units(game: &Game, owner: u8) -> usize {
    game.units.iter().filter(|unit| unit.owner == owner).count()
}

fn mission_score(game: &Game, won: bool) -> cnc_launcher::score::ScoreReport {
    compute_score(
        game.harvested_credits,
        game.units_produced,
        game.projectiles_fired,
        game.elapsed.int().max(0) as u64,
        won,
        DIFFICULTY_NEUTRAL_BP,
    )
}

/// A finished, victorious mission 1: 8 000 starting credits, 500 harvested and
/// two surviving units.
fn finished_first_mission(path: &Path) -> Game {
    let mut game = new_game();
    game.load_mission(path).unwrap();
    assert_eq!(game.houses[0].credits, 8_000, "scenario default treasury");
    game.houses[0].add_credits(500);
    push_unit(&mut game, UnitType::InfantryE1, 30, 30, 0);
    push_unit(&mut game, UnitType::MediumTank, 31, 30, 0);
    game.harvested_credits = 1_200;
    game.elapsed = Fixed16_16::from_int(90);
    game.mission.outcome = Some(cnc_tiberian_dawn::MissionOutcome::Victory);
    game
}

/// PLAN TODO 13 ACCEPTANCE TEST. A scripted two-mission TD run: mission 1 ends,
/// the score screen is shown, mission 2 starts, and the carried-over state
/// (credits + surviving units) equals what the carry-over file recorded.
#[test]
fn campaign_flow_advances_and_carries_credits() {
    let dir = workspace("advance");
    let first = write_original(&dir, "01 - Hold", "Hold the Ridge", 8_000);
    let second = write_original(&dir, "02 - Break", "Break the Defenses", 3_000);
    let carry = dir.join("saves").join(campaign::CARRYOVER_FILE_NAME);
    let mut flow =
        CampaignFlow::from_paths(vec![first.clone(), second.clone()], Some(carry.clone()));
    assert_eq!(flow.len(), 2, "both original missions are listed");
    assert_eq!(flow.current().unwrap().path, first);

    let game = finished_first_mission(flow.current().unwrap().path.as_path());
    let expected_score = mission_score(&game, true);
    assert!(expected_score.won);
    assert!(
        expected_score.total > 0,
        "the score screen has stats to show: {expected_score:?}"
    );

    match flow.finish_mission(&game, 0, true) {
        FlowStep::Advance { next, score } => {
            assert_eq!(next, 1, "a win advances to the next mission");
            assert_eq!(score, expected_score, "the score screen carries the report");
        }
        other => panic!("expected Advance, got {other:?}"),
    }
    assert_eq!(
        flow.current().unwrap().path,
        second,
        "the next mission is now selected"
    );

    // The carry-over file exists with the recorded credits and surviving units.
    assert!(carry.is_file(), "carry-over written at mission end");
    let recorded = Carryover::load_from_file(&carry).unwrap();
    assert_eq!(recorded.credits, 8_500, "credits + harvest carry over");
    assert_eq!(recorded.units.len(), 2, "both survivors carry over");

    // The next mission starts: the carry-over loads back and replaces the
    // scenario default (3 000) with the carried treasury (8 500), redeploying
    // the survivors.
    let carried = flow
        .begin_next()
        .expect("the carry-over restores at the next start");
    assert_eq!(carried, recorded, "loaded state equals the recorded state");
    let mut second_game = new_game();
    second_game
        .load_mission(flow.current().unwrap().path.as_path())
        .unwrap();
    assert_eq!(
        second_game.houses[0].credits, 3_000,
        "the scenario default before carry-over"
    );
    carried.apply_to_game(&mut second_game);
    assert_eq!(
        second_game.houses[0].credits, 8_500,
        "carried credits replace the scenario default"
    );
    assert_eq!(
        owned_units(&second_game, 0),
        2,
        "the surviving units re-deploy in the next mission"
    );
}

/// File round-trip: written at mission end, re-read at the next start, checksum
/// validated, exact credits + units restored, and never double-applied.
#[test]
fn carryover_round_trip_restores_credits_and_units() {
    let dir = workspace("round-trip");
    let first = write_original(&dir, "01 - Hold", "Hold the Ridge", 8_000);
    let second = write_original(&dir, "02 - Break", "Break the Defenses", 3_000);
    let carry = dir.join("saves").join(campaign::CARRYOVER_FILE_NAME);

    let game = finished_first_mission(&first);
    let mut flow = CampaignFlow::from_paths(vec![first, second.clone()], Some(carry.clone()));
    match flow.finish_mission(&game, 0, true) {
        FlowStep::Advance { next, .. } => {
            assert_eq!(next, 1);
        }
        other => panic!("expected Advance, got {other:?}"),
    }

    // A fresh flow (selector reload / process restart) reads the file back.
    let fresh = CampaignFlow::from_paths(vec![second.clone()], Some(carry.clone()));
    let mut fresh = fresh;
    let restored = fresh
        .begin_next()
        .expect("the written carry-over file loads back");
    assert_eq!(restored.credits, 8_500);
    assert_eq!(restored.units.len(), 2);

    // The byte image is a checksummed CNCCARRY file.
    let bytes = std::fs::read(&carry).unwrap();
    assert_eq!(&bytes[..8], &CARRY_MAGIC, "CNCCARRY magic");
    assert_eq!(Carryover::from_bytes(&bytes).unwrap(), restored);

    // Loading twice must not double-apply: the second call applies nothing.
    assert_eq!(
        fresh.begin_next(),
        None,
        "a replayed mission does not re-apply the carry-over"
    );

    // A replay of the same mission produces the same state from the same file.
    let mut replay_a = new_game();
    replay_a.load_mission(&second).unwrap();
    restored.apply_to_game(&mut replay_a);
    let mut replay_b = new_game();
    replay_b.load_mission(&second).unwrap();
    restored.apply_to_game(&mut replay_b);
    assert_eq!(replay_a.houses[0].credits, replay_b.houses[0].credits);
    assert_eq!(owned_units(&replay_a, 0), owned_units(&replay_b, 0));
}

/// FAILURE QA: a missing next-mission file leads to the completion screen, not
/// a panic and not a silent hang on the selector.
#[test]
fn missing_next_mission_file_ends_on_the_completion_screen() {
    let dir = workspace("missing-next");
    let first = write_original(&dir, "01 - Hold", "Hold the Ridge", 8_000);
    let second = write_original(&dir, "02 - Break", "Break the Defenses", 3_000);
    let carry = dir.join("saves").join(campaign::CARRYOVER_FILE_NAME);
    let game = finished_first_mission(&first);
    let expected = mission_score(&game, true);

    // The file disappears between mission 1 and mission 2 (deleted install,
    // unmounted drive, partial copy).
    std::fs::remove_file(&second).unwrap();

    let mut flow = CampaignFlow::from_paths(vec![first, second.clone()], Some(carry.clone()));
    assert!(!second.is_file(), "the next mission file is gone");
    match flow.finish_mission(&game, 0, true) {
        FlowStep::Complete { score } => assert_eq!(score, expected, "the score screen still shows"),
        other => panic!("expected the completion screen, got {other:?}"),
    }
    assert!(flow.is_complete(), "the campaign is over");
    assert!(
        flow.current().is_some(),
        "the flow keeps its last mission for the completion screen"
    );

    // A loss on the same broken campaign retries the same mission instead.
    match flow.finish_mission(&game, 0, false) {
        FlowStep::Retry { index, .. } => assert_eq!(index, 0, "a loss retries the same mission"),
        other => panic!("expected Retry, got {other:?}"),
    }
}

/// PLAN TODO 13 "Do NOT regress authored-campaign flow": the bundled authored
/// `maps/td_campaign` missions advance 01 -> 02, still show their own score
/// screen, and carry credits + survivors through the same engine.
#[test]
fn authored_campaign_flow_is_unchanged() {
    let dir = workspace("authored");
    let maps = Path::new(env!("CARGO_MANIFEST_DIR")).join("../maps/td_campaign");
    let first = maps.join("01.ini");
    let second = maps.join("02.ini");
    let carry = dir.join("saves").join(campaign::CARRYOVER_FILE_NAME);
    let mut flow =
        CampaignFlow::from_paths(vec![first.clone(), second.clone()], Some(carry.clone()));

    let mut game = new_game();
    game.load_mission(flow.current().unwrap().path.as_path())
        .unwrap();
    assert_eq!(game.mission.name, "GDI 01 - Holding the Line");
    assert_eq!(
        game.houses[0].credits, 8_000,
        "the authored mission's own credits"
    );
    game.houses[0].add_credits(500);
    push_unit(&mut game, UnitType::InfantryE1, 30, 30, 0);
    game.elapsed = Fixed16_16::from_int(45);
    game.mission.outcome = Some(cnc_tiberian_dawn::MissionOutcome::Victory);

    match flow.finish_mission(&game, 0, true) {
        FlowStep::Advance { next, score } => {
            assert_eq!(next, 1, "the authored campaign still advances");
            assert!(score.total > 0, "the authored score screen still computes");
        }
        other => panic!("expected Advance, got {other:?}"),
    }
    assert_eq!(flow.current().unwrap().path, second);
    let carried = flow
        .begin_next()
        .expect("the authored campaign carries over");
    assert_eq!(carried.credits, 8_500);

    let mut next = new_game();
    next.load_mission(flow.current().unwrap().path.as_path())
        .unwrap();
    assert_eq!(next.mission.name, "Nod 01 - Breakout");
    carried.apply_to_game(&mut next);
    assert_eq!(
        next.houses[0].credits, 8_500,
        "the GDI treasury carries into the Nod mission, replacing its 9 000 default"
    );
    assert_eq!(owned_units(&next, 0), 1, "the survivor redeploys");
}

/// The original-campaign list is env-gated on the todo 8/9 data directory and
/// degrades to an empty list with a reason when it is unusable.
#[test]
fn original_campaign_listing_is_env_gated_and_ordered() {
    let dir = workspace("env-gate");
    // A decoy INI that is not a mission must be skipped, not crash the pass.
    std::fs::write(dir.join("RULES.INI"), "[Basic]\nNot=a scenario\n").unwrap();
    write_original(&dir, "02 - Break", "Break the Defenses", 3_000);
    write_original(&dir, "01 - Hold", "Hold the Ridge", 8_000);

    // The only variable read here; no other test consults it.
    std::env::set_var(campaign::ORIGINAL_DATA_ENV, &dir);
    let found = campaign::discover_from_env();
    std::env::remove_var(campaign::ORIGINAL_DATA_ENV);

    assert_eq!(found.reason, None, "no degradation when the dir is usable");
    assert_eq!(found.missions.len(), 2, "both original scenarios listed");
    assert_eq!(found.skipped.len(), 1, "the decoy INI is skipped");
    assert_eq!(
        found.missions[0].order, 1,
        "campaign order, not directory order"
    );
    assert_eq!(found.missions[0].name, "Hold the Ridge");
    assert_eq!(found.missions[1].order, 2);
    assert_eq!(found.missions[1].name, "Break the Defenses");
    assert!(
        found.missions.iter().all(|mission| mission.path.exists()),
        "listed missions are real files"
    );

    // Unset (tests run without it set) and missing directories both degrade.
    let unset = campaign::discover_from_env();
    assert!(unset.missions.is_empty());
    assert!(
        unset.reason.is_some(),
        "an unset var explains the empty list"
    );

    std::env::set_var(campaign::ORIGINAL_DATA_ENV, dir.join("nope"));
    let missing = campaign::discover_from_env();
    std::env::remove_var(campaign::ORIGINAL_DATA_ENV);
    assert!(missing.missions.is_empty());
    assert!(missing.skipped.is_empty());
    assert!(
        missing
            .reason
            .as_deref()
            .is_some_and(|reason| reason.contains("does not exist")),
        "{missing:?}"
    );
}

/// ADVERSARIAL - malformed input: a corrupt carry-over (bad checksum, truncated
/// payload) never panics and never corrupts a save; the next mission starts
/// clean instead.
#[test]
fn corrupt_carryover_degrades_to_a_clean_mission() {
    let dir = workspace("corrupt");
    let first = write_original(&dir, "01 - Hold", "Hold the Ridge", 8_000);
    let second = write_original(&dir, "02 - Break", "Break the Defenses", 3_000);
    let carry = dir.join("saves").join(campaign::CARRYOVER_FILE_NAME);
    let game = finished_first_mission(&first);

    let mut written =
        CampaignFlow::from_paths(vec![first.clone(), second.clone()], Some(carry.clone()));
    written.finish_mission(&game, 0, true);
    let good = std::fs::read(&carry).unwrap();

    // 1. Bad checksum: flip a payload byte.
    let mut flipped = good.clone();
    let last = flipped.len() - 1;
    flipped[last] ^= 0xff;
    std::fs::write(&carry, &flipped).unwrap();
    let mut corrupted =
        CampaignFlow::from_paths(vec![first.clone(), second.clone()], Some(carry.clone()));
    let applied = corrupted.begin_next();
    assert_eq!(applied, None, "a bad checksum is refused");
    assert!(
        corrupted
            .last_error()
            .is_some_and(|error| error.contains("checksum")),
        "{:?}",
        corrupted.last_error()
    );

    // 2. Truncated payload: only the magic survives.
    std::fs::write(&carry, &good[..8]).unwrap();
    let mut truncated = CampaignFlow::from_paths(vec![first, second.clone()], Some(carry.clone()));
    assert_eq!(truncated.begin_next(), None, "a truncated file is refused");
    assert!(
        truncated.last_error().is_some(),
        "{:?}",
        truncated.last_error()
    );

    // 3. The corrupt file is never rewritten as garbage: a fresh run rewrites
    //    a valid CNCCARRY image from the still-valid game state.
    let mut again = CampaignFlow::from_paths(vec![second.clone()], Some(carry.clone()));
    again.finish_mission(&game, 0, true);
    let repaired = Carryover::load_from_file(&carry).unwrap();
    assert_eq!(repaired.credits, 8_500);
}

/// ADVERSARIAL - malformed input: a carry-over holding a unit the next map
/// cannot host. Off-map coordinates are CLAMPED into the map (never a panic,
/// never a corrupt save); an impassable landing cell is skipped. Either way the
/// treasury still carries over and the run continues deterministically.
#[test]
fn carryover_with_unusable_units_drops_them_deterministically() {
    let dir = workspace("bad-units");
    let first = write_original(&dir, "01 - Hold", "Hold the Ridge", 8_000);
    let second = write_original(&dir, "02 - Break", "Break the Defenses", 3_000);
    let carry = dir.join("saves").join(campaign::CARRYOVER_FILE_NAME);
    // One unit at far-off-map coordinates, one landing on an impassable cell,
    // one that is fine.
    let unusable = Carryover {
        owner: 0,
        credits: 4_200,
        units: vec![
            cnc_tiberian_dawn::carryover::CarriedUnit {
                unit_type: UnitType::InfantryE1,
                position: Vec2::new(
                    Fixed16_16::from_f32(-50_000.0),
                    Fixed16_16::from_f32(-50_000.0),
                ),
                health: Fixed16_16::from_int(100),
            },
            cnc_tiberian_dawn::carryover::CarriedUnit {
                unit_type: UnitType::MediumTank,
                position: Vec2::new(
                    Fixed16_16::from_f32(9_999_999.0),
                    Fixed16_16::from_f32(9_999_999.0),
                ),
                health: Fixed16_16::from_int(100),
            },
            cnc_tiberian_dawn::carryover::CarriedUnit {
                unit_type: UnitType::Jeep,
                position: Vec2::new(
                    Fixed16_16::from_f32(32.0 * TILE_SIZE + TILE_SIZE * 0.5),
                    Fixed16_16::from_f32(32.0 * TILE_SIZE + TILE_SIZE * 0.5),
                ),
                health: Fixed16_16::from_int(100),
            },
        ],
    };
    unusable.save_to_file(&carry).unwrap();

    // The bytes round-trip, so the record itself is intact...
    let reloaded = Carryover::load_from_file(&carry).unwrap();
    assert_eq!(reloaded, unusable);
    let mut flow = CampaignFlow::from_paths(vec![first, second.clone()], Some(carry.clone()));
    let applied = flow
        .begin_next()
        .expect("a well-formed carry-over loads even when its units are unusable");
    assert_eq!(applied, unusable);

    // ...and applying it degrades deterministically. Off-map coordinates are
    // clamped into the map (the engine's documented behaviour); a genuinely
    // impassable landing cell is skipped.
    let mut game = new_game();
    game.load_mission(&second).unwrap();
    game.map
        .grid
        .set_passable(cnc_core::Vec2i::new(32, 32), false);
    applied.apply_to_game(&mut game);
    let placed = owned_units(&game, 0);
    assert_eq!(
        placed, 2,
        "off-map units clamp to passable edge cells and deploy; the impassable one is skipped: {placed}"
    );
    assert!(
        game.units.iter().all(|unit| {
            let x = (unit.position.x.to_f32() / TILE_SIZE).floor() as i32;
            let y = (unit.position.y.to_f32() / TILE_SIZE).floor() as i32;
            game.map.grid.in_bounds(x, y)
        }),
        "no placed unit sits outside the map"
    );
    assert_eq!(
        game.houses[0].credits, 4_200,
        "credits always carry over, even when units do not"
    );

    // Deterministic: the same carry-over applied to an equivalent board lands
    // the same way every time.
    for _ in 0..3 {
        let mut repeat = new_game();
        repeat.load_mission(&second).unwrap();
        repeat
            .map
            .grid
            .set_passable(cnc_core::Vec2i::new(32, 32), false);
        applied.apply_to_game(&mut repeat);
        assert_eq!(owned_units(&repeat, 0), placed);
        assert_eq!(repeat.houses[0].credits, 4_200);
    }
}
