# RUST/cnc-bench/

## Responsibility
Criterion benchmark suite for C&C simulation performance. Measures cold-start latency and sustained simulation throughput for both Tiberian Dawn and Red Alert engines using deterministic fixtures. No metric functions exposed - all measurements live in criterion bench targets.

## Design Patterns
- Fixture-Driven: Shared bench_scenario_ini() builds identical 64x64 RA skirmish map for all benches
- Deterministic Time: frame_delta() returns exact 1/60s as Fixed16_16::from_ratio(1, 60) - no float drift
- Separation of Concerns: Library (src/lib.rs) holds only fixtures; benches (benches/*.rs) drive real engine code
- Throughput Protocol: 600 frames (10s sim time) per iteration, RA command injection every 100 frames

## Data & Control Flow
1. Fixture Build: bench_scenario_ini() to INI string to Scenario::from_ini() to Scenario struct
2. Cold Start Bench: benches/cold_start.rs times Scenario::new() + first frame
3. Throughput Bench: benches/sim_throughput.rs runs 600 frames, injects commands at interval, measures frame time
4. Determinism Assertion: Tests verify frame_delta().to_bits() == 1092 (16.16 fixed 1/60)

## Public API (Library - minimal, fixture-only)
- BENCH_MAP_SIZE: u32 = 64
- FRAME_SECS: f64 = 1.0/60.0
- FRAMES_PER_ITER: usize = 600
- RA_COMMAND_INTERVAL: usize = 100
- bench_scenario_ini() -> String - deterministic RA skirmish INI
- bench_scenario() -> Scenario - parsed scenario fixture
- frame_delta() -> Fixed16_16 - exact 1/60s fixed-point delta

## Integration Points
- Dependencies: cnc-core, cnc-tiberian-dawn, cnc-red-alert, criterion (dev)
- Consumed by: CI performance regression detection, cargo bench runs
- No runtime dependency - dev-only crate, not linked into game binaries

## Key Files
- src/lib.rs - Shared fixtures, constants, deterministic frame delta
- benches/cold_start.rs - Cold start latency measurement
- benches/sim_throughput.rs - Sustained simulation throughput measurement
- Cargo.toml - harness = false for criterion bench targets

