//! End-of-mission scoring — Rust port of the original `SCORE.CPP` /
//! `STATS.CPP` scoreboard surface.
//!
//! Pure, deterministic integer math over the telemetry the TD [`Game`]
//! already tracks (mirrored into `GameSnapshot`: `harvested_credits`,
//! `units_produced`, `projectiles_fired`, `elapsed`). Difficulty multipliers
//! are expressed in basis points so no floating point ever enters scoring.

use std::time::Duration;

use cnc_renderer::Renderer;

/// Neutral difficulty multiplier (100%).
pub const DIFFICULTY_NEUTRAL_BP: u32 = 10_000;

/// Credits awarded per second saved below the par time.
pub const TIME_BONUS_CREDITS_PER_SECOND: i64 = 10;
/// Par time for the speed bonus (30 minutes, matching the original's
/// long-mission pacing).
pub const TIME_BONUS_PAR_SECONDS: i64 = 1_800;
/// Flat bonus for completing the mission.
pub const OUTCOME_BONUS_VICTORY: i64 = 5_000;
/// Consolation bonus so a failed run still shows movement.
pub const OUTCOME_BONUS_DEFEAT: i64 = 500;

/// Milliseconds each score row takes to count up.
const REVEAL_MS: i64 = 700;
/// Stagger between consecutive rows.
const REVEAL_STAGGER_MS: i64 = 250;

/// Deterministic end-of-mission score derived from match telemetry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreReport {
    pub won: bool,
    pub harvest_credits: i64,
    pub units_produced: u64,
    pub shots_fired: u64,
    pub time_seconds: u64,
    pub time_bonus: i64,
    pub outcome_bonus: i64,
    pub difficulty_multiplier_bp: u32,
    pub subtotal: i64,
    pub total: i64,
}

/// Compute the score from raw telemetry. Same inputs always produce the
/// same report; overflow saturates instead of panicking.
pub fn compute_score(
    harvested_credits: i64,
    units_produced: u64,
    shots_fired: u64,
    elapsed_seconds: u64,
    won: bool,
    difficulty_multiplier_bp: u32,
) -> ScoreReport {
    let time_bonus = if won {
        TIME_BONUS_PAR_SECONDS.saturating_sub(elapsed_seconds as i64)
            * TIME_BONUS_CREDITS_PER_SECOND
    } else {
        0
    };
    let outcome_bonus = if won {
        OUTCOME_BONUS_VICTORY
    } else {
        OUTCOME_BONUS_DEFEAT
    };
    let harvest_credits = harvested_credits.max(0);
    let subtotal = harvest_credits
        .saturating_add(time_bonus)
        .saturating_add(outcome_bonus);
    let total = subtotal.saturating_mul(i64::from(difficulty_multiplier_bp)) / 10_000;
    ScoreReport {
        won,
        harvest_credits,
        units_produced,
        shots_fired,
        time_seconds: elapsed_seconds,
        time_bonus,
        outcome_bonus,
        difficulty_multiplier_bp,
        subtotal,
        total,
    }
}

impl ScoreReport {
    /// `(label, value)` rows for the scoreboard, in draw order.
    pub fn rows(&self) -> [(&'static str, i64); 4] {
        [
            ("Harvested credits", self.harvest_credits),
            ("Time bonus", self.time_bonus),
            ("Mission bonus", self.outcome_bonus),
            ("Total", self.total),
        ]
    }

    /// Animated reveal: 0 until `delay_ms` after the screen opens, then a
    /// linear count-up over [`REVEAL_MS`], clamped to `value`.
    pub fn reveal(value: i64, elapsed: Duration, delay_ms: i64) -> i64 {
        let elapsed_ms = elapsed.as_millis() as i64;
        if elapsed_ms <= delay_ms {
            return 0;
        }
        let progress = (elapsed_ms - delay_ms).min(REVEAL_MS);
        value.saturating_mul(progress) / REVEAL_MS
    }
}

/// Draw the score overlay into the HUD panel anchored at `(origin_x,
/// origin_y)` (the same panel origin the mission-status line uses).
pub fn draw_overlay(
    renderer: &mut dyn Renderer,
    report: &ScoreReport,
    since: Duration,
    origin_x: f32,
    origin_y: f32,
) {
    const W: f32 = 300.0;
    const H: f32 = 170.0;
    let (banner, banner_color) = if report.won {
        ("MISSION ACCOMPLISHED", [0.45, 1.00, 0.55, 1.0])
    } else {
        ("MISSION FAILED", [1.00, 0.45, 0.45, 1.0])
    };
    renderer.draw_rect(origin_x, origin_y, W, H, [0.03, 0.05, 0.09, 0.90]);
    renderer.draw_rect(origin_x, origin_y, W, 3.0, banner_color);
    renderer.draw_text(banner, origin_x + 16.0, origin_y + 14.0, 11.0, banner_color);

    let _elapsed_ms = since.as_millis() as i64;
    for (index, (label, value)) in report.rows().iter().enumerate() {
        let delay = 300 + i64::try_from(index).unwrap_or(0) * REVEAL_STAGGER_MS;
        let shown = ScoreReport::reveal(*value, since, delay);
        let row_y = origin_y + 42.0 + 20.0 * index as f32;
        renderer.draw_text(label, origin_x + 16.0, row_y, 9.0, [0.75, 0.82, 0.92, 1.0]);
        renderer.draw_text(
            &shown.to_string(),
            origin_x + W - 96.0,
            row_y,
            9.0,
            [0.95, 0.97, 1.0, 1.0],
        );
    }

    let footer = format!(
        "Units produced {}   Shots fired {}   Time {}s",
        report.units_produced, report.shots_fired, report.time_seconds
    );
    renderer.draw_text(
        &footer,
        origin_x + 16.0,
        origin_y + H - 22.0,
        8.0,
        [0.62, 0.70, 0.82, 1.0],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn victory_bonuses_compose_deterministically() {
        let report = compute_score(10_000, 42, 128, 600, true, DIFFICULTY_NEUTRAL_BP);
        // (1800 par - 600 elapsed) * 10 credits/s = 12_000.
        assert_eq!(report.time_bonus, 12_000);
        assert_eq!(report.outcome_bonus, OUTCOME_BONUS_VICTORY);
        assert_eq!(report.subtotal, 27_000);
        assert_eq!(report.total, 27_000);
        // Same inputs, same report.
        assert_eq!(
            report,
            compute_score(10_000, 42, 128, 600, true, DIFFICULTY_NEUTRAL_BP)
        );
    }

    #[test]
    fn defeat_gets_consolation_and_no_time_bonus() {
        let report = compute_score(2_500, 7, 40, 60, false, DIFFICULTY_NEUTRAL_BP);
        assert_eq!(report.time_bonus, 0);
        assert_eq!(report.outcome_bonus, OUTCOME_BONUS_DEFEAT);
        assert_eq!(report.total, 3_000);
    }

    #[test]
    fn negative_harvest_clamps_to_zero() {
        let report = compute_score(-50, 0, 0, 0, false, DIFFICULTY_NEUTRAL_BP);
        assert_eq!(report.harvest_credits, 0);
        assert_eq!(report.total, OUTCOME_BONUS_DEFEAT);
    }

    #[test]
    fn multiplier_scales_the_total() {
        let base = compute_score(10_000, 0, 0, 1_800, true, 12_000);
        assert_eq!(base.total, (10_000 + OUTCOME_BONUS_VICTORY) * 12 / 10);
    }

    #[test]
    fn extreme_values_saturate_without_panicking() {
        let report = compute_score(i64::MAX / 2, u64::MAX, u64::MAX, 0, true, 20_000);
        assert!(report.total > 0);
    }

    #[test]
    fn reveal_animates_in_stages() {
        let value = 1_000;
        assert_eq!(
            ScoreReport::reveal(value, Duration::from_millis(100), 300),
            0
        );
        let half = ScoreReport::reveal(value, Duration::from_millis(300 + 350), 300);
        assert!(half > 0 && half < value);
        assert_eq!(
            ScoreReport::reveal(value, Duration::from_millis(3_000), 300),
            value
        );
    }

    #[test]
    fn rows_are_in_draw_order() {
        let report = compute_score(1, 2, 3, 4, true, DIFFICULTY_NEUTRAL_BP);
        let labels: Vec<_> = report.rows().iter().map(|(label, _)| *label).collect();
        assert_eq!(
            labels,
            ["Harvested credits", "Time bonus", "Mission bonus", "Total"]
        );
    }
}
