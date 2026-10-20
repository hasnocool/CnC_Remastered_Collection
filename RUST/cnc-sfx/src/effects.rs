//! The effect set: percussive hits, buzzes, and seamless engine loops.
//!
//! All effects are deterministic functions of a [`Seed`]. Durations are in
//! samples so the math is independent of the write rate; write-time helpers
//! emit 16-bit mono PCM.

use crate::waves::{adsr, decay_env, noise, sine, square};
use cnc_procgen::prng::{Rng, Seed};

/// Era-appropriate low sample rate for generated effects.
pub const DEFAULT_SAMPLE_RATE: u32 = 22_050;

fn seconds(samples: usize) -> f32 {
    samples as f32 / DEFAULT_SAMPLE_RATE as f32
}

/// Clamp helper keeping rendered samples inside `[-1, 1]`.
fn squash(v: f32) -> f32 {
    v.clamp(-1.0, 1.0)
}

/// Explosion: filtered noise burst over a low rumble, ~1.4 s.
pub fn explosion(seed: Seed, len: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut filtered = 0.0_f32;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        let burst = noise(&mut rng);
        filtered += 0.18 * (burst - filtered); // low-pass the noise
        let rumble = 0.6 * sine(46.0 + 20.0 * decay_env(t, 0.25), t);
        let env = adsr(t, 0.005, 0.32);
        out.push(squash(0.75 * (filtered * 1.2 + rumble) * env));
    }
    out
}

/// Gunfire: short click + noise crack, ~0.35 s.
pub fn gunfire(seed: Seed, len: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        let crack = 0.8 * noise(&mut rng);
        let click = 0.4 * square(900.0, t) * decay_env(t - 0.01, 0.05).max(0.0);
        out.push(squash((crack + click) * adsr(t, 0.002, 0.06)));
    }
    out
}

/// Tesla zap: descending square buzz with a noisy tail, ~0.5 s.
pub fn tesla_zap(seed: Seed, len: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        let sweep = 1800.0 - 1200.0 * (t / seconds(len)).clamp(0.0, 1.0);
        let arc = 0.5 * square(sweep, t) + 0.25 * square(sweep * 1.5, t);
        let tail = 0.2 * noise(&mut rng) * decay_env(t, 0.1);
        out.push(squash((arc + tail) * adsr(t, 0.001, 0.09)));
    }
    out
}

/// Engine rumble, seamless: every frequency is an integer multiple of the
/// loop fundamental `1/duration`, so the waveform is exactly periodic and
/// the seam is continuous. The seed only picks the harmonic mix — it cannot
/// break periodicity.
pub fn engine_loop(seed: Seed, len: usize) -> Vec<f32> {
    let base = 1.0 / seconds(len); // fundamental fits exactly one cycle
    let k_low = 3 + (seed.0 % 3) as usize; // 3..=5
    let k_mid = 6 + (seed.0 / 3 % 4) as usize; // 6..=9
    let k_high = 10 + (seed.0 / 11 % 5) as usize; // 10..=14
    let tremor_hz = 2.0 * base; // slow periodic amplitude wobble
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        // Sines only: the saw wave is inherently discontinuous at each
        // period boundary, which would put a click at the loop seam.
        let tone = 0.5 * sine(k_low as f32 * base, t)
            + 0.3 * sine(k_mid as f32 * base, t)
            + 0.2 * sine(k_high as f32 * base, t);
        let tremor = 0.5 * (1.0 + 0.4 * sine(tremor_hz, t));
        out.push(squash(0.55 * tone * tremor));
    }
    out
}

/// Selection beep: two-tone chirp, ~0.22 s.
pub fn select_beep(seed: Seed, len: usize) -> Vec<f32> {
    let _ = seed; // beep is fully tonal; seed kept for API uniformity
    let mut out = Vec::with_capacity(len);
    let split = len / 2;
    for i in 0..len {
        let t = seconds(i);
        let freq = if i < split { 660.0 } else { 880.0 };
        out.push(squash(0.5 * sine(freq, t) * adsr(t, 0.01, 0.1)));
    }
    out
}

/// Construction loop: rhythmic clanks every ~0.5 s. One period is generated
/// once and tiled, so periods repeat exactly and the loop is seamless.
pub fn construction_loop(seed: Seed, len: usize) -> Vec<f32> {
    let rate = DEFAULT_SAMPLE_RATE as usize;
    let period = (rate as f32 * 0.5) as usize; // 0.5 s per clank cycle
    let mut prng = Rng::new(Seed::new(seed.0));
    let mut one_period = Vec::with_capacity(period);
    for i in 0..period {
        let local = seconds(i);
        let jitter = 1.5 * (prng.next_f64() as f32 - 0.5);
        let clank = 0.6 * sine(210.0 + jitter * 30.0, local) + 0.4 * noise(&mut prng);
        one_period.push(squash(clank * adsr(local, 0.005, 0.16)));
    }
    // Tile: exact period repetition is what makes the loop seamless.
    let mut out = Vec::with_capacity(len);
    while out.len() < len {
        let take = period.min(len - out.len());
        out.extend_from_slice(&one_period[..take]);
    }
    out
}

/// Bit-mix constant used to derive a distinct noise stream per shot offset.
/// SplitMix64's gamma, kept local so this crate does not depend on
/// `cnc_procgen` exposing it.
const SHOT_MIX: u64 = 0x9E37_79B9_7F4A_7C15;

/// Weapon shot helper: noise crack + low body sweep + short decay, rendered at
/// sample index `i`. The index is mixed into the seed so repeated shots have
/// distinct noise with no cross-frame state, keeping output a pure function of
/// the seed and the buffer length.
fn shot(seed: Seed, i: usize) -> f32 {
    let mut rng = Rng::new(Seed::new(seed.0 ^ (i as u64).wrapping_mul(SHOT_MIX)));
    let t = seconds(i);
    let body = sine(90.0 + 30.0 * (1.0 - t / 0.05).clamp(0.0, 1.0), t);
    0.5 * (0.7 * noise(&mut rng) + 0.3 * body) * adsr(t, 0.001, 0.08)
}

/// Cannon double-shot: two `CANNON1`/`CANNON2`-style reports, second softer.
/// AUDIO.CPP: `CANNON1`, `CANNON2` (lines 140-141).
pub fn cannon_double_shot(seed: Seed, len: usize) -> Vec<f32> {
    let gap = (DEFAULT_SAMPLE_RATE as f32 * 0.15) as usize;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let second = shot(seed, i.saturating_sub(gap));
        out.push(squash(shot(seed, i) + 0.6 * second));
    }
    out
}

/// Autocannon 3-round stutter: `AACANON3`-style burst, evenly spaced.
/// AUDIO.CPP: `AACANON3` (line 166), `GUN11`/`GUN13` (154-155), `PILLBOX1` (173).
pub fn autocannon_stutter(seed: Seed, len: usize) -> Vec<f32> {
    let gap = (DEFAULT_SAMPLE_RATE as f32 * 0.1) as usize;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let burst = shot(seed, i)
            + 0.8 * shot(seed, i.saturating_sub(gap))
            + 0.55 * shot(seed, i.saturating_sub(2 * gap));
        out.push(squash(burst));
    }
    out
}

/// Rocket whoosh: rising filtered-noise sweep with a decay tail, plus the
/// launch thump. AUDIO.CPP: `MISSILE1` (VOC_MISSILE_1, high tech effect),
/// `MISSILE6` (VOC_MISSILE_2, long launch), `MISSILE7` (VOC_MISSILE_3, short
/// launch) — lines 169-171; `LAUNCH1`/`SATLNCH1` share the family.
pub fn rocket_whoosh(seed: Seed, len: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut lp = 0.0_f32;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        // The low-pass coefficient climbs with the sweep frequency, which
        // reads as the whoosh tracking the motor's rising pitch.
        let f = 300.0 + 1500.0 * (t / 0.35).clamp(0.0, 1.0);
        let coeff = 2.0 * f / DEFAULT_SAMPLE_RATE as f32;
        lp += coeff * (noise(&mut rng) - lp);
        let thump = 0.4 * sine(70.0, t) * adsr(t, 0.002, 0.1);
        let tail = 0.25 * sine(f, t) * decay_env(t, 0.45);
        out.push(squash((lp * 1.4 + thump + tail) * adsr(t, 0.01, 0.25)));
    }
    out
}

/// Artillery distant boom: heavily filtered noise rumble + falling sub, long
/// tail. AUDIO.CPP: `KABOOM30` (VOC_KABOOM30, short HE explosion, line 188),
/// `TANDETH1` (line 167), `KABOOM1` (161), `KABOOM12`/`KABOOM15` (162-163).
pub fn artillery_boom(seed: Seed, len: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut lp = 0.0_f32;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        // Distance = very low low-pass cutoff, so most crack is gone.
        lp += 0.02 * (noise(&mut rng) - lp);
        let sub = 0.7 * sine(48.0 + 12.0 * decay_env(t, 0.2), t);
        let crack = 0.3 * noise(&mut rng) * decay_env(t, 0.03);
        out.push(squash((lp * 1.2 + sub + crack) * adsr(t, 0.004, 0.9)));
    }
    out
}

/// Ore/refinery boom: bigger than `explosion`, heavier debris tail.
/// AUDIO.CPP: `KABOOM22` (VOC_KABOOM22, long sharp explosion, line 167),
/// `KABOOM25` (short growling explosion, line 189), `FIRETRT1` (152).
pub fn ore_refinery_boom(seed: Seed, len: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut lp = 0.0_f32;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        lp += 0.06 * (noise(&mut rng) - lp);
        // Debris tail: a second, later-filtered noise layer that outlives the
        // blast body by design (tau 0.6 vs 0.35).
        let debris = 0.5 * noise(&mut rng) * decay_env(t - 0.12, 0.6).max(0.0);
        let rumble = 0.7 * sine(40.0 + 14.0 * decay_env(t, 0.3), t);
        out.push(squash((lp * 1.3 + rumble + debris) * adsr(t, 0.004, 0.35)));
    }
    out
}

/// Building collapse: long low groan plus sequential slab impacts.
/// AUDIO.CPP: `CRMBLE2` (VOC_CRUMBLE, building crumble, line 193),
/// `PLACBLDG` (VOC_PLACE_BUILDING_DOWN, line 187), `BUILD5` (196),
/// `KABOOM12`/`KABOOM15` (162-163).
pub fn building_collapse(seed: Seed, len: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut lp = 0.0_f32;
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        lp += 0.05 * (noise(&mut rng) - lp);
        let groan = 0.55 * sine(55.0 - 18.0 * (t / seconds(len)).clamp(0.0, 1.0), t);
        // Impacts every ~0.22 s, each lower than the last as the frame fails.
        let phase = (t / 0.22).fract();
        let idx = (t / 0.22).floor();
        let f = 220.0 / (1.0 + 0.35 * idx);
        let impact = 0.5 * sine(f, t) * adsr(phase * 0.22, 0.001, 0.07);
        out.push(squash((lp * 1.1 + groan + impact) * adsr(t, 0.006, 0.5)));
    }
    out
}

/// UI bleep: high, short sine pip. AUDIO.CPP: `RABEEP1` (VOC_BEEP, generic
/// beep, line 174), `BLEEP17` (VOC_OPTIONS_CHANGED, mid range soft warble,
/// 200), `BLEEP6` (VOC_INCOMING_MESSAGE, soft happy warble, 198).
pub fn ui_bleep_high(_seed: Seed, len: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        out.push(squash(0.5 * sine(1320.0, t) * adsr(t, 0.004, 0.055)));
    }
    out
}

/// UI bleep: low, dry click-pip (soft denial / edit settle).
/// AUDIO.CPP: `BLEEP5` (VOC_SYS_ERROR, sharp soft warble, line 199).
pub fn ui_bleep_low(_seed: Seed, len: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        out.push(squash(0.5 * square(520.0, t) * adsr(t, 0.003, 0.045)));
    }
    out
}

/// UI blip: two alternating pips, the "cursor moved" family.
/// AUDIO.CPP: `BLEEP11` (VOC_PLAYER_JOINED, chirp sequence, line 203).
pub fn ui_blip(_seed: Seed, len: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(len);
    let split = len / 2;
    for i in 0..len {
        let t = seconds(i);
        let freq = if i < split { 880.0 } else { 1100.0 };
        out.push(squash(0.45 * sine(freq, t) * adsr(t, 0.003, 0.04)));
    }
    out
}

/// UI affirm: rising two-tone confirm chirp.
/// AUDIO.CPP: `BLEEP12` (VOC_PLAYER_LEFT, chirp sequence, line 202).
pub fn ui_affirm(_seed: Seed, len: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        let f = 740.0 + 220.0 * (t / seconds(len)).clamp(0.0, 1.0);
        out.push(squash(0.5 * sine(f, t) * adsr(t, 0.003, 0.07)));
    }
    out
}

/// UI deny soft: short falling warble, muted error/refusal.
/// AUDIO.CPP: `BLEEP13` (VOC_GAME_FORMING, long warble, line 201),
/// `BLEEP9` (VOC_GAME_CLOSED, long bleep, 197).
pub fn ui_deny_soft(_seed: Seed, len: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        let f = 560.0 - 180.0 * (t / seconds(len)).clamp(0.0, 1.0);
        let warble = sine(f, t) + 0.25 * sine(f * 1.5, t);
        out.push(squash(0.35 * warble * adsr(t, 0.004, 0.05)));
    }
    out
}

/// Chronosphere: charge-up pitch sweep ending in a shift sparkle burst.
/// AUDIO.CPP: `CHRONO2` (VOC_CHRONO, chronosphere sound, line 139);
/// `CHROCHR1` (VOX_CHRONO_CHARGING, 587) and `JCHRGE1` (VOC_STCHRGE1,
/// "Fully charged!", 268) are the voice-bus announcements — this is the
/// procedural charge/shift cue that plays with them.
pub fn chronosphere_shift(seed: Seed, len: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut lp = 0.0_f32;
    let total = seconds(len);
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        let p = (t / total).clamp(0.0, 1.0);
        // Slow ascending charge through 60% of the buffer, then the shift:
        // frequency keeps rising while the tone thins out.
        let charge = 180.0 + 620.0 * p;
        lp += 0.3 * (noise(&mut rng) - lp);
        // Sparkle only starts once the shift lands (p > 0.6).
        let sparkle = if p > 0.6 { 0.5 * lp } else { 0.0 };
        let shift = sine(charge, t) * if p > 0.6 { 0.35 } else { 1.0 };
        // Long attack (0.45 s) so the charge reads as a build, not a hit.
        out.push(squash((shift + sparkle) * adsr(t, 0.45, 0.5)));
    }
    out
}

/// Iron curtain: sustained low hum, seamless loop.
/// AUDIO.CPP: `IRONCUR9` (VOC_IRON1, line 142), `INVUL2`
/// (VOC_INVULNERABLE, invulnerability effect, 160). `IRONCHG1`
/// (VOX_IRON_CHARGING, 612) is the voice-bus announcement.
pub fn iron_curtain_hum(seed: Seed, len: usize) -> Vec<f32> {
    let base = 1.0 / seconds(len); // fundamental fits exactly one cycle
    let k = 2 + (seed.0 % 3) as usize; // 2..=4
    let k2 = 5 + (seed.0 / 3 % 3) as usize; // 5..=7
    let k3 = 9 + (seed.0 / 7 % 4) as usize; // 9..=12
    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        let t = seconds(i);
        // Sines only, integer multiples of the loop fundamental (same rule
        // as `engine_loop`): exactly periodic, so the seam is continuous.
        let tone = 0.55 * sine(k as f32 * base, t)
            + 0.3 * sine(k2 as f32 * base, t)
            + 0.15 * sine(k3 as f32 * base, t);
        let swell = 0.6 + 0.4 * sine(2.0 * base, t);
        out.push(squash(0.6 * tone * swell));
    }
    out
}

/// Render mono f32 samples to 16-bit PCM WAV bytes.
pub fn to_wav_bytes(samples: &[f32], rate: u32) -> Vec<u8> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut buffer = std::io::Cursor::new(Vec::new());
    {
        // Cursor<Vec<u8>> is itself Write + Seek, which is what hound wants.
        let mut writer =
            hound::WavWriter::new(&mut buffer, spec).expect("wav writer over memory cannot fail");
        for &s in samples {
            let clamped = s.clamp(-1.0, 1.0);
            let int = (clamped * i16::MAX as f32) as i16;
            writer.write_sample(int).expect("memory write");
        }
        writer.finalize().expect("memory finalize");
    }
    buffer.into_inner()
}

/// Write 16-bit PCM WAV to `path` (atomic: temp file + rename in same dir).
pub fn write_wav_16(path: &std::path::Path, samples: &[f32], rate: u32) -> std::io::Result<()> {
    let bytes = to_wav_bytes(samples, rate);
    let tmp = path.with_extension("wav.tmp");
    std::fs::write(&tmp, &bytes)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt()
    }

    fn seed(n: u64) -> Seed {
        Seed::new(n)
    }

    #[test]
    fn every_effect_is_bounded_and_nonzero() {
        let len = DEFAULT_SAMPLE_RATE as usize / 2;
        assert!(rms(&explosion(seed(1), len)) > 0.02);
        assert!(rms(&gunfire(seed(1), len)) > 0.02);
        assert!(rms(&tesla_zap(seed(1), len)) > 0.02);
        assert!(rms(&engine_loop(seed(1), len)) > 0.02);
        assert!(rms(&select_beep(seed(1), len)) > 0.02);
        assert!(rms(&construction_loop(seed(1), len)) > 0.02);
        for effect in [
            explosion(seed(2), len),
            gunfire(seed(2), len),
            tesla_zap(seed(2), len),
            engine_loop(seed(2), len),
            select_beep(seed(2), len),
            construction_loop(seed(2), len),
        ] {
            assert!(effect.iter().all(|s| s.is_finite() && s.abs() <= 1.0));
        }
    }

    #[test]
    fn effects_are_deterministic_per_seed_and_seed_sensitive() {
        let len = 4096;
        assert_eq!(explosion(seed(9), len), explosion(seed(9), len));
        assert_ne!(explosion(seed(9), len), explosion(seed(10), len));
        assert_eq!(gunfire(seed(9), len), gunfire(seed(9), len));
        assert_ne!(gunfire(seed(9), len), gunfire(seed(11), len));
    }

    #[test]
    fn percussive_effects_decay() {
        let len = DEFAULT_SAMPLE_RATE as usize;
        for effect in [explosion(seed(3), len), gunfire(seed(3), len)] {
            let head = rms(&effect[..len / 4]);
            let tail = rms(&effect[len * 3 / 4..]);
            assert!(
                tail < head * 0.5,
                "effect did not decay: head {head} tail {tail}"
            );
        }
    }

    #[test]
    fn engine_loop_is_seamless() {
        let len = DEFAULT_SAMPLE_RATE as usize / 2;
        let samples = engine_loop(seed(4), len);
        // The seam is the wrap from the last sample back to the first. For a
        // loop whose period equals the buffer length, that jump must be no
        // larger than a typical per-sample step.
        let seam = (samples[0] - samples[len - 1]).abs();
        let typical: f32 =
            samples.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f32>() / (len - 1) as f32;
        assert!(
            seam <= typical * 4.0 + 1e-4,
            "engine loop seam jump {seam} vs typical step {typical}"
        );
    }

    #[test]
    fn construction_loop_periods_are_identical() {
        // Deterministic per-period shape means period 0 repeats exactly.
        let len = DEFAULT_SAMPLE_RATE as usize; // 2 periods of 0.5 s
        let samples = construction_loop(seed(5), len);
        let period = len / 2;
        assert_eq!(
            &samples[..period / 8],
            &samples[period..period + period / 8]
        );
    }

    #[test]
    fn weapon_variants_are_distinct_and_bounded() {
        let len = DEFAULT_SAMPLE_RATE as usize;
        let variants = [
            cannon_double_shot(seed(1), len),
            autocannon_stutter(seed(1), len),
            rocket_whoosh(seed(1), len),
            artillery_boom(seed(1), len),
        ];
        for v in &variants {
            assert!(rms(v) > 0.02, "silent weapon variant");
            assert!(v.iter().all(|s| s.is_finite() && s.abs() <= 1.0));
        }
        assert_ne!(variants[0], variants[1], "cannon vs autocannon identical");
        assert_ne!(variants[2], variants[3], "rocket vs artillery identical");
    }

    #[test]
    fn weapon_variants_are_deterministic_per_seed() {
        let len = 4096;
        assert_eq!(
            cannon_double_shot(seed(9), len),
            cannon_double_shot(seed(9), len)
        );
        assert_ne!(
            cannon_double_shot(seed(9), len),
            cannon_double_shot(seed(10), len)
        );
        assert_eq!(
            autocannon_stutter(seed(9), len),
            autocannon_stutter(seed(9), len)
        );
        assert_ne!(
            autocannon_stutter(seed(9), len),
            autocannon_stutter(seed(10), len)
        );
        assert_eq!(rocket_whoosh(seed(9), len), rocket_whoosh(seed(9), len));
        assert_ne!(rocket_whoosh(seed(9), len), rocket_whoosh(seed(10), len));
        assert_eq!(artillery_boom(seed(9), len), artillery_boom(seed(9), len));
        assert_eq!(
            iron_curtain_hum(seed(9), len),
            iron_curtain_hum(seed(9), len)
        );
    }

    #[test]
    fn explosion_variants_decay_with_debris_tails() {
        let len = DEFAULT_SAMPLE_RATE as usize;
        for effect in [
            ore_refinery_boom(seed(3), len),
            building_collapse(seed(3), len),
        ] {
            let head = rms(&effect[..len / 4]);
            let tail = rms(&effect[len * 3 / 4..]);
            assert!(tail < head * 0.5, "no decay: head {head} tail {tail}");
            assert!(tail > 0.005, "no debris tail left: {tail}");
        }
    }

    #[test]
    fn ui_bleeps_are_short_distinct_and_bounded() {
        let len = DEFAULT_SAMPLE_RATE as usize / 4; // 0.25 s
        let bleeps = [
            ui_bleep_high(seed(1), len),
            ui_bleep_low(seed(1), len),
            ui_blip(seed(1), len),
            ui_affirm(seed(1), len),
            ui_deny_soft(seed(1), len),
        ];
        for b in &bleeps {
            assert!(rms(b) > 0.02, "silent bleep");
            assert!(b.iter().all(|s| s.is_finite() && s.abs() <= 1.0));
            let head = rms(&b[..len / 2]);
            let tail = rms(&b[len / 2..]);
            assert!(tail < head, "bleep did not decay: {tail} vs {head}");
        }
        // Every member of the family must have its own shape.
        for (i, a) in bleeps.iter().enumerate() {
            for b in bleeps.iter().skip(i + 1) {
                assert_ne!(a, b, "duplicate bleep shape at pair {i}");
            }
        }
    }

    #[test]
    fn superweapon_cues_charge_then_shift() {
        let len = DEFAULT_SAMPLE_RATE as usize;
        let chrono = chronosphere_shift(seed(1), len);
        assert!(rms(&chrono) > 0.02, "silent chronosphere cue");
        assert!(chrono.iter().all(|s| s.is_finite() && s.abs() <= 1.0));
        // Charge: opens quietly, peaks around the shift instant.
        let tenth = len / 10;
        let head = rms(&chrono[..tenth]);
        let mid = rms(&chrono[len * 4 / 10..len * 6 / 10]);
        assert!(head < mid, "no charge-up: head {head} mid {mid}");
        // Iron curtain is a sustained hum, not a one-shot.
        assert!(rms(&iron_curtain_hum(seed(1), len)) > 0.02);
        assert_ne!(chrono, iron_curtain_hum(seed(1), len));
    }

    #[test]
    fn iron_curtain_hum_is_seamless() {
        // Sines at integer multiples of 1/duration wrap without a click.
        let len = DEFAULT_SAMPLE_RATE as usize / 2;
        let samples = iron_curtain_hum(seed(4), len);
        let seam = (samples[0] - samples[len - 1]).abs();
        let typical: f32 =
            samples.windows(2).map(|w| (w[1] - w[0]).abs()).sum::<f32>() / (len - 1) as f32;
        assert!(
            seam <= typical * 4.0 + 1e-4,
            "iron curtain seam jump {seam} vs typical step {typical}"
        );
    }

    #[test]
    fn wav_bytes_have_riff_header_and_match_len() {
        let samples = explosion(seed(6), 512);
        let bytes = to_wav_bytes(&samples, DEFAULT_SAMPLE_RATE);
        assert_eq!(&bytes[..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WAVE");
        // 44-byte header + 2 bytes/sample mono.
        assert_eq!(bytes.len(), 44 + samples.len() * 2);
    }
}
