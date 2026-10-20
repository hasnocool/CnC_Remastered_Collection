//! Shared setup helpers for the `cnc-bench` criterion suites.
//!
//! This crate deliberately exposes **no metric functions** of its own. Every
//! measurement lives in the criterion bench targets (`benches/sim_throughput.rs`
//! and `benches/cold_start.rs`) and drives the real TD/RA simulation code. The
//! library only holds the deterministic fixtures those benches share, so the
//! benches cannot drift apart in their setup.

use cnc_core::Fixed16_16;
use cnc_red_alert::scenario::Scenario;

/// Map side used by every bench scene (64x64 cells, both engines).
pub const BENCH_MAP_SIZE: u32 = 64;

/// Fixed simulation step: exactly one 60 FPS frame (the plan's target cadence).
pub const FRAME_SECS: f64 = 1.0 / 60.0;

/// Frames advanced per measured iteration in the throughput benches
/// (600 frames = 10 seconds of simulated game time).
pub const FRAMES_PER_ITER: usize = 600;

/// RA `apply_command` interval in the throughput bench (every 100th frame).
pub const RA_COMMAND_INTERVAL: usize = 100;

/// Build the shared RA scenario fixture as editor-dialect INI text.
///
/// The [Overlay] rows are generated instead of written out to keep the source
/// readable (one `3` = ore token per cell); everything else is a literal so the
/// fixture is fully deterministic and version-controlled.
pub fn bench_scenario_ini() -> String {
    let mut ini = String::with_capacity(2048);
    ini.push_str(
        "[Map]\n\
         Width=64\n\
         Height=64\n\
         Name=Bench Skirmish\n\
         Theater=Temperate\n\
         Player=Allies\n\
         \n\
         [PlayerStarts]\n\
         1=6,6\n\
         2=58,58\n\
         \n\
         [Overlay]\n",
    );
    // Ore rows across the map so harvesters shuttle real cells during the run.
    for (index, y) in [20u32, 41u32].into_iter().enumerate() {
        let row = vec!["3"; BENCH_MAP_SIZE as usize].join(",");
        ini.push_str(&format!("{index}={y}:({row})\n"));
    }
    ini.push_str(
        "\n\
         [Objects]\n\
         1=PowerPlant,8,8,HouseAllies\n\
         2=Refinery,10,8,HouseAllies\n\
         3=Barracks,8,12,HouseAllies\n\
         4=WarFactory,10,12,HouseAllies\n\
         5=PowerPlant,56,56,HouseSoviet\n\
         6=Refinery,54,56,HouseSoviet\n\
         7=Barracks,56,52,HouseSoviet\n\
         8=VehicleE2,20,16,HouseAllies\n\
         9=VehicleE2,22,16,HouseAllies\n\
         10=VehicleE2,24,16,HouseAllies\n\
         11=InfantryE1,20,14,HouseAllies\n\
         12=VehicleE2,44,48,HouseSoviet\n\
         13=VehicleE2,42,48,HouseSoviet\n\
         14=InfantryE1,44,50,HouseSoviet\n\
         15=InfantryE1,42,50,HouseSoviet\n",
    );
    ini
}

/// Parse the shared RA scenario fixture (`Scenario::from_ini`).
pub fn bench_scenario() -> Scenario {
    Scenario::from_ini(&bench_scenario_ini()).expect("bench scenario fixture must parse")
}

/// One 60 FPS frame as the engines' fixed-point delta type.
///
/// `Fixed16_16::from_ratio` is const and uses exact integer arithmetic
/// (65536 / 60 = 1092 ticks), so every bench iteration steps with a
/// bit-identical delta. `from_f32` is documented as non-deterministic in
/// `cnc-core` and is deliberately avoided here.
pub fn frame_delta() -> Fixed16_16 {
    Fixed16_16::from_ratio(1, 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_scenario_fixture_parses() {
        let scenario = bench_scenario();
        assert_eq!(scenario.width, BENCH_MAP_SIZE);
        assert_eq!(scenario.height, BENCH_MAP_SIZE);
        assert!(
            !scenario.objects.is_empty(),
            "scenario fixture must spawn real objects"
        );
        assert!(
            !scenario.ore.is_empty() && scenario.ore.iter().any(|&amount| amount > 0),
            "scenario fixture must contain ore cells"
        );
    }

    #[test]
    fn frame_delta_is_sixtieth_of_a_second() {
        // 1/60 s in 16.16 fixed point: 65536 / 60 truncated = 1092 ticks.
        assert_eq!(frame_delta().to_bits(), 1092);
    }
}
