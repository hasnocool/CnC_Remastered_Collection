//! Deterministic simulation RNG for Red Alert.
//!
//! Red Alert's original engine drew every "random" mission pick from one
//! shared, seeded generator so all lockstep peers made the same choice on the
//! same frame. This port previously used ad-hoc integer hashes instead: they
//! were reproducible but had no seed, no state and therefore no way to prove
//! that two matches (or a save and its replay) follow the same stream.
//!
//! `SimRng` replaces them with a single SplitMix64 stream (the same pinned
//! algorithm as `cnc-procgen::prng` and the TD crate's powerup stream).
//! The seed and the cursor live in `Game` and are written to the save file,
//! so a reload resumes exactly where the original tick left off.
//!
//! # Determinism contract
//!
//! * Never remove or reorder a draw on a simulation path without bumping the
//!   save version - the cursor position is part of the game state.
//! * Drawing randomness for presentation (rendering, HUD animation, cosmetic
//!   map variation) from this stream would desynchronise replays; those paths
//!   must use a local, throw-away source instead.
//! * Changing `GAMMA` or the SplitMix64 mix function breaks every stored
//!   save and recorded command stream.

use serde::{Deserialize, Serialize};

/// SplitMix64 increment (2^64 / golden ratio); shared with `cnc-procgen::prng`.
const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// Fixed seed used when a match or lobby does not supply one. Every caller
/// that wants a specific stream must pass its own seed through
/// `Game::new_with_seed`.
pub const DEFAULT_SEED: u64 = 0x5EED_0000_0000_0001;

/// Key-mix constant for [`SimRng::stream`] sub-stream seeds.
const STREAM_MIX: u64 = 0x2545_F491_4F6C_DD1D;

/// A seeded, serialisable SplitMix64 stream.
///
/// `seed` is the value the match started from (needed to derive stable
/// sub-streams later, e.g. per-unit scatters); `state` is the live cursor.
/// Both are saved verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimRng {
    seed: u64,
    state: u64,
}

impl SimRng {
    /// Start a match stream from an explicit seed.
    pub const fn new(seed: u64) -> Self {
        Self { seed, state: seed }
    }

    /// Start from [`DEFAULT_SEED`].
    pub const fn from_default_seed() -> Self {
        Self::new(DEFAULT_SEED)
    }

    /// The seed this stream started from.
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// The live cursor; serialised with the game state.
    pub const fn state(&self) -> u64 {
        self.state
    }

    /// Restore a cursor read from a save/replay.
    pub fn set_state(&mut self, state: u64) {
        self.state = state;
    }

    /// Restart the stream from a new match seed.
    pub fn reseed(&mut self, seed: u64) {
        self.seed = seed;
        self.state = seed;
    }

    /// Advance the shared cursor and return the next 64-bit draw.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GAMMA);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Next 32-bit draw (high half of the 64-bit output).
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Uniform-ish draw in `0..upper` (modulo; pinned so behaviour never
    /// drifts between builds). `upper == 0` returns 0.
    pub fn below(&mut self, upper: u32) -> u32 {
        if upper == 0 {
            return 0;
        }
        self.next_u32() % upper
    }

    /// Inclusive draw in `lo..=hi`; swapped bounds are normalised.
    pub fn range_inclusive(&mut self, lo: i32, hi: i32) -> i32 {
        let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        let span = (i64::from(hi) - i64::from(lo) + 1) as u64;
        lo + (self.next_u64() % span) as i32
    }

    /// `true` with `percent`% probability (0 = never, 100 = always).
    pub fn percent(&mut self, percent: u32) -> bool {
        self.below(100) < percent.min(100)
    }

    /// A private sub-stream derived from the match seed and a stable key
    /// (unit id, team id, ...).
    ///
    /// This is for `&self` command helpers that must answer deterministically
    /// without advancing the tick cursor: the result still depends on the
    /// match seed, so two matches with different seeds scatter differently.
    pub fn stream(seed: u64, key: u64) -> Self {
        let mut rng = Self::new(seed ^ key.wrapping_mul(STREAM_MIX));
        // Discard the first draw so adjacent keys do not produce adjacent
        // outputs (SplitMix64 is sequential in its state).
        let _ = rng.next_u64();
        rng
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix_matches_pinned_vectors() {
        // Pinned SplitMix64(seed = 0) outputs - the same contract as
        // cnc-procgen::prng and TD's powerup stream. Changing the algorithm
        // breaks every stored save/replay.
        let mut rng = SimRng::new(0);
        assert_eq!(rng.next_u64(), 16_294_208_416_658_607_535);
        assert_eq!(rng.next_u64(), 7_960_286_522_194_355_700);
        assert_eq!(rng.next_u64(), 487_617_019_471_545_679);
    }

    #[test]
    fn seeded_streams_repeat_and_diverge() {
        let draw = |seed: u64| {
            let mut rng = SimRng::new(seed);
            (0..8).map(|_| rng.next_u64()).collect::<Vec<_>>()
        };
        assert_eq!(draw(7), draw(7), "same seed repeats");
        assert_ne!(draw(7), draw(8), "different seed diverges");
    }

    #[test]
    fn sub_streams_are_stable_and_keyed() {
        let a: Vec<u64> = {
            let mut rng = SimRng::stream(42, 1);
            (0..4).map(|_| rng.next_u64()).collect()
        };
        let b: Vec<u64> = {
            let mut rng = SimRng::stream(42, 1);
            (0..4).map(|_| rng.next_u64()).collect()
        };
        let mut other = SimRng::stream(42, 2);
        assert_eq!(a, b, "same seed and key repeat");
        assert_ne!(a[0], other.next_u64(), "different keys diverge");
    }

    #[test]
    fn bounded_draws_stay_inside_their_range() {
        let mut rng = SimRng::new(DEFAULT_SEED);
        for _ in 0..256 {
            let value = rng.range_inclusive(-3, 3);
            assert!((-3..=3).contains(&value), "range draw {value}");
            assert!(rng.below(5) < 5);
            assert!(rng.below(0) == 0);
        }
        assert!(!rng.percent(0));
        assert!(rng.percent(100));
    }
}
