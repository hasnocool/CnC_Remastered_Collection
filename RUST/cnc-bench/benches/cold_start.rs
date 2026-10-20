//! Cold-start benchmarks: scenario parse, engine boot, one A* path query.
//!
//! Everything measured here is real engine work, not a placeholder loop:
//!
//! - `cold_start/ra_scenario_from_ini`: `Scenario::from_ini` parses the shared
//!   editor-dialect fixture (engine anchor: `cnc-red-alert/src/scenario.rs`).
//! - `cold_start/ra_game_from_scenario`: `Game::from_scenario` boots an RA
//!   skirmish from the pre-parsed fixture (engine anchor:
//!   `cnc-red-alert/src/game.rs`).
//! - `cold_start/td_game_new`: TD `Game::new` builds a generated map and the
//!   full game session (engine anchor: `cnc-tiberian-dawn/src/game.rs`).
//! - `cold_start/td_find_path`: A* over a generated 64x64 `CellGrid` with a
//!   deterministic obstacle wall (engine anchor:
//!   `cnc-tiberian-dawn/src/pathfind.rs` — `find_path`).

use std::hint::black_box;

use cnc_bench::{bench_scenario, bench_scenario_ini, BENCH_MAP_SIZE};
use cnc_core::Vec2i;
use cnc_red_alert::game::Game as RaGame;
use cnc_red_alert::scenario::Scenario;
use cnc_tiberian_dawn::game::Game as TdGame;
use cnc_tiberian_dawn::map::CellGrid;
use cnc_tiberian_dawn::pathfind::find_path;

use criterion::{criterion_group, criterion_main, Criterion};

fn ra_scenario_from_ini(c: &mut Criterion) {
    let mut group = c.benchmark_group("cold_start");
    group.bench_function("ra_scenario_from_ini", |b| {
        b.iter(|| {
            let parsed: Scenario = Scenario::from_ini(&bench_scenario_ini())
                .expect("bench scenario fixture must parse");
            black_box(parsed)
        })
    });

    let scenario = bench_scenario();
    group.bench_function("ra_game_from_scenario", |b| {
        b.iter(|| black_box(RaGame::from_scenario(&scenario)))
    });
    group.finish();
}

fn td_game_new(c: &mut Criterion) {
    let mut group = c.benchmark_group("cold_start");
    group.bench_function("td_game_new", |b| {
        b.iter(|| black_box(TdGame::new("BENCH", BENCH_MAP_SIZE, BENCH_MAP_SIZE)))
    });
    group.finish();
}

fn td_find_path(c: &mut Criterion) {
    let mut grid = CellGrid::new(BENCH_MAP_SIZE, BENCH_MAP_SIZE);
    // Deterministic wall: a vertical gap at y=32 through x in [24..40), which
    // forces A* to route around it rather than walk straight to the goal.
    for x in 24..40u32 {
        if let Some(cell) = grid.get_mut(Vec2i::new(x as i32, 32)) {
            cell.is_passable = false;
        }
    }
    let start = Vec2i::new(2, 2);
    let goal = Vec2i::new(62, 62);
    let blocked: [Vec2i; 0] = [];
    let mut group = c.benchmark_group("cold_start");
    group.bench_function("td_find_path", |b| {
        b.iter(|| black_box(find_path(&grid, &blocked, start, goal, grid.cells.len())))
    });
    group.finish();
}

criterion_group!(
    name = cold_start;
    config = Criterion::default();
    targets = ra_scenario_from_ini, td_game_new, td_find_path
);
criterion_main!(cold_start);
