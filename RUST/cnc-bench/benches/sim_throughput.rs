//! Simulation-throughput benchmarks over the real TD and RA engines.
//!
//! Two variants, both driven at the plan's target cadence (60 FPS fixed dt):
//!
//! - `td/fixed_dt_frames`: a generated Tiberian Dawn game advanced through
//!   `Game::update` for a fixed number of frames (engine anchor:
//!   `cnc-tiberian-dawn/src/game.rs` — `Game::new` / `Game::update`).
//! - `ra/scenario_frames_with_commands`: a Red Alert game booted from the
//!   shared scenario fixture (`Game::from_scenario`), advanced through
//!   `Game::update` while lockstep `GameCommand::Move` commands are applied at
//!   a fixed interval (engine anchors: `cnc-red-alert/src/game.rs` —
//!   `Game::from_scenario` / `Game::update` / `Game::apply_command`).
//!
//! Setup (game construction, scenario parse) happens in `iter_batched`'s setup
//! closure so the measured routine is exactly the frame-stepping loop.

use std::hint::black_box;

use cnc_bench::{
    bench_scenario, frame_delta, BENCH_MAP_SIZE, FRAMES_PER_ITER, RA_COMMAND_INTERVAL,
};
use cnc_core::{Fixed16_16, Vec2};
use cnc_red_alert::game::{Game as RaGame, GameCommand};
use cnc_tiberian_dawn::game::Game as TdGame;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};

/// Move-command target: ~middle of the 64x64 map in world units
/// (RA `TILE_SIZE` = 24 world units per cell; 32 * 24 = 768).
fn move_target() -> Vec2 {
    Vec2::new(Fixed16_16::from_int(32 * 24), Fixed16_16::from_int(32 * 24))
}

fn td_sim_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("sim_throughput/td");
    group.throughput(Throughput::Elements(FRAMES_PER_ITER as u64));
    group.bench_function("fixed_dt_frames", |b| {
        b.iter_batched(
            || TdGame::new("BENCH", BENCH_MAP_SIZE, BENCH_MAP_SIZE),
            |mut game| {
                let delta = frame_delta();
                for _ in 0..FRAMES_PER_ITER {
                    game.update(delta);
                }
                black_box(&game);
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn ra_sim_throughput(c: &mut Criterion) {
    let scenario = bench_scenario();
    let target = move_target();
    let mut group = c.benchmark_group("sim_throughput/ra");
    group.throughput(Throughput::Elements(FRAMES_PER_ITER as u64));
    group.bench_function("scenario_frames_with_commands", |b| {
        b.iter_batched(
            || RaGame::from_scenario(&scenario),
            |mut game| {
                let delta = frame_delta();
                for frame in 0..FRAMES_PER_ITER {
                    // Exercise the lockstep command path at a fixed cadence so
                    // movement/A* work stays alive across the whole loop.
                    if frame % RA_COMMAND_INTERVAL == 0 {
                        let units: Vec<u32> = game.units.iter().map(|unit| unit.id).collect();
                        if !units.is_empty() {
                            game.apply_command(&GameCommand::Move {
                                owner: game.player_owner,
                                units,
                                target,
                            });
                        }
                    }
                    game.update(delta);
                }
                black_box(&game);
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

criterion_group!(
    name = sim_throughput;
    config = Criterion::default();
    targets = td_sim_throughput, ra_sim_throughput
);
criterion_main!(sim_throughput);
