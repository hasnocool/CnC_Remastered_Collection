//! Seeded, deterministic PRNG and hashing helpers.
//!
//! Determinism is load-bearing for this project (the game runs a
//! deterministic lockstep simulation; generated assets must be bit-identical
//! for a given seed on the same toolchain), so this crate deliberately uses
//! its own PRNG instead of a shared `rand` dependency: the algorithm is
//! pinned here, never version-drifted. All outputs are pure functions of
//! the seed.

/// SplitMix64 bit-mix constant.
const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// Deterministic seed wrapper (explicit type keeps call sites honest).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seed(pub u64);

impl Seed {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// SplitMix64 generator state.
#[derive(Clone, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: Seed) -> Self {
        Self { state: seed.0 }
    }

    /// Next SplitMix64 output; advances the state.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GAMMA);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform u32 from the top bits of `next_u64`.
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Uniform f64 in `[0, 1)`, derived from a 53-bit slice of the output.
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Uniform value in `[lo, hi)` (integer range; `hi > lo` required).
    pub fn next_range(&mut self, lo: u32, hi: u32) -> u32 {
        debug_assert!(hi > lo, "empty range");
        lo + self.next_u32() % (hi - lo)
    }
}

/// Stateless lattice hash for value noise: `hash_coord(x, y, seed)`.
///
/// Same function the noise field uses, exposed for tests and callers that
/// need per-point randomness without a generator instance.
pub fn hash_coord(x: i32, y: i32, seed: Seed) -> u64 {
    let mut z = (x as i64 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ (y as i64 as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
        ^ seed.0.wrapping_mul(GAMMA);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Hash to f64 in `[0, 1)`.
pub fn hash_f64(x: i32, y: i32, seed: Seed) -> f64 {
    (hash_coord(x, y, seed) >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix64_known_vector() {
        // Pinned outputs of SplitMix64(seed=0): any change to the algorithm
        // breaks determinism of previously generated assets.
        let mut rng = Rng::new(Seed::new(0));
        let values: Vec<u64> = (0..3).map(|_| rng.next_u64()).collect();
        assert_eq!(
            values,
            vec![
                16_294_208_416_658_607_535,
                7_960_286_522_194_355_700,
                487_617_019_471_545_679,
            ]
        );
    }

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::new(Seed::new(42));
        let mut b = Rng::new(Seed::new(42));
        for _ in 0..64 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seed_diverges() {
        let mut a = Rng::new(Seed::new(1));
        let mut b = Rng::new(Seed::new(2));
        let differs = (0..8).any(|_| a.next_u64() != b.next_u64());
        assert!(differs, "distinct seeds must diverge quickly");
    }

    #[test]
    fn next_f64_is_unit_interval() {
        let mut rng = Rng::new(Seed::new(7));
        for _ in 0..1000 {
            let v = rng.next_f64();
            assert!((0.0..1.0).contains(&v), "f64 out of range: {v}");
        }
    }

    #[test]
    fn hash_coord_is_stable_and_seed_sensitive() {
        let a = hash_f64(3, 4, Seed::new(9));
        let b = hash_f64(3, 4, Seed::new(9));
        let c = hash_f64(3, 4, Seed::new(10));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn next_range_stays_in_bounds() {
        let mut rng = Rng::new(Seed::new(123));
        for _ in 0..1000 {
            let v = rng.next_range(10, 15);
            assert!((10..15).contains(&v), "range violation: {v}");
        }
    }
}
