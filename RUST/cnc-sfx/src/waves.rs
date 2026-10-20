//! Waveforms, envelopes and noise primitives.
//!
//! All functions are pure: sample values depend only on time, frequency and
//! (for noise) the seeded [`Rng`](cnc_procgen::prng::Rng). No wall clock, no
//! shared mutable state.

use cnc_procgen::prng::Rng;

/// Two pi.
pub const TAU: f32 = std::f32::consts::TAU;

/// Sine wave in `[-1, 1]` at frequency `freq` Hz, evaluated at `t` seconds.
pub fn sine(freq: f32, t: f32) -> f32 {
    (TAU * freq * t).sin()
}

/// Square wave in `[-1, 1]` with a 50% duty cycle.
pub fn square(freq: f32, t: f32) -> f32 {
    if (t * freq).fract() < 0.5 {
        1.0
    } else {
        -1.0
    }
}

/// Sawtooth wave in `[-1, 1]`.
pub fn saw(freq: f32, t: f32) -> f32 {
    2.0 * ((t * freq).fract()) - 1.0
}

/// One white-noise sample in `[-1, 1]` from the seeded generator.
pub fn noise(rng: &mut Rng) -> f32 {
    rng.next_f64() as f32 * 2.0 - 1.0
}

/// Exponential decay: `e^(-t/tau)`, clamped at 0 after 10 half-lives.
pub fn decay_env(t: f32, tau: f32) -> f32 {
    if tau <= 1e-9 {
        return 0.0;
    }
    (-t / tau).exp().max(0.0)
}

/// Linear ramp 0→1 over `dur` seconds, then hold at 1.
pub fn ramp(t: f32, dur: f32) -> f32 {
    if dur <= 1e-9 || t >= dur {
        1.0
    } else {
        (t / dur).clamp(0.0, 1.0)
    }
}

/// Attack-decay envelope: 0→1 over `a` seconds, then exponential decay with
/// time constant `tau`. Used for percussive hits and zaps.
pub fn adsr(t: f32, a: f32, tau: f32) -> f32 {
    if t < a {
        ramp(t, a)
    } else {
        decay_env(t - a, tau)
    }
}

/// One-pole low-pass smoothing helper: returns the new state given `input`,
/// previous `state` and a coefficient `alpha` in `[0, 1]`.
pub fn one_pole(state: f32, input: f32, alpha: f32) -> f32 {
    state + alpha * (input - state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waves_stay_in_range() {
        for i in 0..1000 {
            let t = i as f32 / 1000.0;
            for f in [40.0, 220.0, 1200.0] {
                assert!(sine(f, t).abs() <= 1.0 + 1e-6);
                assert!(square(f, t).abs() <= 1.0);
                assert!(saw(f, t).abs() <= 1.0 + 1e-6);
            }
        }
    }

    #[test]
    fn square_wave_values_are_binary() {
        let mut saw_pos = 0;
        let mut saw_neg = 0;
        for i in 0..2000 {
            let v = square(100.0, i as f32 / 2000.0 * 0.02);
            if v > 0.0 {
                saw_pos += 1;
            } else {
                saw_neg += 1;
            }
        }
        assert!(
            saw_pos > 500 && saw_neg > 500,
            "duty cycle off: {saw_pos}/{saw_neg}"
        );
    }

    #[test]
    fn decay_env_monotone_and_bounded() {
        let mut prev = 1.0;
        for i in 1..500 {
            let v = decay_env(i as f32 / 100.0, 0.2);
            assert!((0.0..=1.0).contains(&v));
            assert!(v <= prev + 1e-6, "decay must not grow");
            prev = v;
        }
    }

    #[test]
    fn adsr_attacks_then_decays() {
        let a = adsr(0.05, 0.1, 0.3);
        let b = adsr(0.1, 0.1, 0.3);
        let c = adsr(1.0, 0.1, 0.3);
        assert!(b > a && c < b, "bad envelope shape: {a} {b} {c}");
    }

    #[test]
    fn one_pole_moves_toward_input() {
        let mut state = 0.0;
        for _ in 0..100 {
            state = one_pole(state, 1.0, 0.2);
        }
        assert!(state > 0.9, "low-pass did not converge: {state}");
        for _ in 0..100 {
            state = one_pole(state, -1.0, 0.2);
        }
        assert!(state < -0.9, "low-pass did not track down: {state}");
    }
}
