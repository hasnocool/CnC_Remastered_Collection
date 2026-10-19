//! 16.16 Fixed-point arithmetic (matching Westwood's `fixed` type)

use bytemuck::{Pod, Zeroable};
use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Shr, Sub, SubAssign};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

/// 16.16 fixed-point number (matches original C&C `fixed` type exactly)
/// Range: -32768.0 to 32767.99998474121
/// Precision: 1/65536 ≈ 0.0000152587890625
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable)]
pub struct Fixed16_16(pub i32);

impl Fixed16_16 {
    /// Number of fractional bits
    pub const FRAC_BITS: i32 = 16;
    /// Scaling factor (1 << 16 = 65536)
    pub const SCALE: i32 = 1 << 16;
    /// Maximum value (32767.99998474121)
    pub const MAX: Self = Self(i32::MAX);
    /// Minimum value (-32768.0)
    pub const MIN: Self = Self(i32::MIN);
    /// One (1.0)
    pub const ONE: Self = Self(Self::SCALE);
    /// Zero (0.0)
    pub const ZERO: Self = Self(0);
    /// Half (0.5)
    pub const HALF: Self = Self(Self::SCALE / 2);
    /// Pi approximation
    pub const PI: Self = Self(205887); // 3.14159 * 65536
    /// 2 * Pi
    pub const TAU: Self = Self(411775); // 2 * PI
    /// Pi / 2
    pub const FRAC_PI_2: Self = Self(102944); // PI / 2
    /// Pi / 4
    pub const FRAC_PI_4: Self = Self(51472); // PI / 4
    /// Epsilon (smallest positive value)
    pub const EPSILON: Self = Self(1);
    /// Degrees to radians factor
    pub const DEG_TO_RAD: Self = Self(1141); // PI / 180 * 65536
    /// Radians to degrees factor
    pub const RAD_TO_DEG: Self = Self(3759466); // 180 / PI * 65536

    /// Create from raw bits
    #[inline]
    pub const fn from_bits(bits: i32) -> Self {
        Self(bits)
    }

    /// Get raw bits
    #[inline]
    pub const fn to_bits(self) -> i32 {
        self.0
    }

    /// Create from integer
    #[inline]
    pub const fn from_int(value: i32) -> Self {
        Self(value.saturating_mul(Self::SCALE))
    }

    /// Create from float (for initialization only - not deterministic)
    pub fn from_f32(value: f32) -> Self {
        Self((value * Self::SCALE as f32).round() as i32)
    }

    /// Convert to float (for display/debug only)
    pub fn to_f32(self) -> f32 {
        self.0 as f32 / Self::SCALE as f32
    }

    /// Create from numerator/denominator (exact rational)
    pub const fn from_ratio(num: i32, den: i32) -> Self {
        // Use saturating arithmetic to match original behavior
        let scaled_num = num.saturating_mul(Self::SCALE);
        Self(scaled_num / den)
    }

    /// Integer part (truncated toward zero)
    #[inline]
    pub const fn int(self) -> i32 {
        self.0 / Self::SCALE
    }

    /// Fractional part
    #[inline]
    pub const fn frac(self) -> Self {
        Self(self.0 % Self::SCALE)
    }

    /// Absolute value
    #[inline]
    pub const fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Saturating addition
    #[inline]
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    /// Saturating subtraction
    #[inline]
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    /// Saturating multiplication
    pub fn saturating_mul(self, rhs: Self) -> Self {
        // (a * b) >> 16 with saturation
        let result = (self.0 as i64).saturating_mul(rhs.0 as i64) >> 16;
        Self(result.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
    }

    /// Saturating division
    pub fn saturating_div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            return if self.0 >= 0 { Self::MAX } else { Self::MIN };
        }
        let result = ((self.0 as i64) << 16) / (rhs.0 as i64);
        Self(result.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
    }

    /// Wrapping addition (matches original overflow behavior)
    #[inline]
    pub const fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }

    /// Wrapping subtraction
    #[inline]
    pub const fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }

    /// Wrapping multiplication
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        let result = ((self.0 as i64) * (rhs.0 as i64)) >> 16;
        Self(result as i32)
    }

    /// Wrapping division
    pub fn wrapping_div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            return Self::ZERO;
        }
        let result = ((self.0 as i64) << 16) / (rhs.0 as i64);
        Self(result as i32)
    }

    /// Square root using an integer (bit-by-bit) algorithm.
    ///
    /// Computes floor(sqrt(bits << 16)) in 64-bit space, which is exact for the
    /// whole 16.16 range and avoids the overflow that Newton's method had for
    /// vectors beyond roughly 181 world units.
    pub fn sqrt(self) -> Self {
        if self.0 <= 0 {
            return Self::ZERO;
        }

        let mut target = (self.0 as u64) << 16;
        let mut root = 0u64;
        let mut bit = 1u64 << 62;
        while bit > target {
            bit >>= 2;
        }

        while bit != 0 {
            if target >= root + bit {
                target -= root + bit;
                root = (root >> 1) + bit;
            } else {
                root >>= 1;
            }
            bit >>= 2;
        }

        Self(root as i32)
    }

    /// Sine (using Taylor series, deterministic)
    pub fn sin(self) -> Self {
        // Reduce to [-PI, PI]
        let mut x = self % Self::TAU;
        if x > Self::PI {
            x -= Self::TAU;
        } else if x < -Self::PI {
            x += Self::TAU;
        }

        // Taylor series: x - x^3/6 + x^5/120 - x^7/5040
        let x2 = x.wrapping_mul(x);
        let x3 = x2.wrapping_mul(x);
        let x5 = x3.wrapping_mul(x2);
        let x7 = x5.wrapping_mul(x2);

        x.wrapping_sub(x3 / Self::from_int(6))
            .wrapping_add(x5 / Self::from_int(120))
            .wrapping_sub(x7 / Self::from_int(5040))
    }

    /// Cosine (Taylor series, deterministic)
    pub fn cos(self) -> Self {
        let mut x = self % Self::TAU;
        if x > Self::PI {
            x -= Self::TAU;
        } else if x < -Self::PI {
            x += Self::TAU;
        }

        // Taylor series: 1 - x^2/2! + x^4/4! - x^6/6! + x^8/8!
        let x2 = x.wrapping_mul(x);
        let x4 = x2.wrapping_mul(x2);
        let x6 = x4.wrapping_mul(x2);
        let x8 = x6.wrapping_mul(x2);

        Self::ONE
            .wrapping_sub(x2 / Self::from_int(2))
            .wrapping_add(x4 / Self::from_int(24))
            .wrapping_sub(x6 / Self::from_int(720))
            .wrapping_add(x8 / Self::from_int(40320))
    }

    /// Arctangent2 (y, x)
    pub fn atan2(y: Self, x: Self) -> Self {
        if x == Self::ZERO {
            if y == Self::ZERO {
                return Self::ZERO;
            }
            return if y > Self::ZERO {
                Self::FRAC_PI_2
            } else {
                -Self::FRAC_PI_2
            };
        }
        let mut angle = (y / x).atan();
        if x < Self::ZERO {
            if y >= Self::ZERO {
                angle += Self::PI;
            } else {
                angle -= Self::PI;
            }
        }
        angle
    }

    /// Arctangent (using polynomial approximation)
    fn atan(self) -> Self {
        // Minimax polynomial approximation for atan in [-1, 1]
        // For |x| > 1, use atan(x) = PI/2 - atan(1/x)
        let abs_x = self.abs();
        let reciprocal = abs_x > Self::ONE;
        let x = if reciprocal { Self::ONE / abs_x } else { abs_x };

        // Polynomial: x - x^3/3 + x^5/5 - x^7/7 + x^9/9
        let x2 = x.wrapping_mul(x);
        let x3 = x2.wrapping_mul(x);
        let x5 = x3.wrapping_mul(x2);
        let x7 = x5.wrapping_mul(x2);
        let x9 = x7.wrapping_mul(x2);

        let mut result = x
            .wrapping_sub(x3 / Self::from_int(3))
            .wrapping_add(x5 / Self::from_int(5))
            .wrapping_sub(x7 / Self::from_int(7))
            .wrapping_add(x9 / Self::from_int(9));

        if reciprocal {
            result = Self::FRAC_PI_2 - result;
        }

        if self < Self::ZERO {
            -result
        } else {
            result
        }
    }

    /// Right shift (division by 2^n)
    #[inline]
    pub const fn shr(self, n: u32) -> Self {
        Self(self.0 >> n)
    }

    /// Left shift (multiplication by 2^n)
    #[inline]
    pub const fn shl(self, n: u32) -> Self {
        Self(self.0 << n)
    }
}

impl Shr<u32> for Fixed16_16 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: u32) -> Self::Output {
        self.shr(rhs)
    }
}

impl Shr<i32> for Fixed16_16 {
    type Output = Self;
    #[inline]
    fn shr(self, rhs: i32) -> Self::Output {
        self.shr(rhs as u32)
    }
}

impl Rem for Fixed16_16 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

// Arithmetic operators (wrapping - matches original C behavior)
impl Add for Fixed16_16 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.wrapping_add(rhs)
    }
}
impl Sub for Fixed16_16 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}
impl Mul for Fixed16_16 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.wrapping_mul(rhs)
    }
}
impl Div for Fixed16_16 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.wrapping_div(rhs)
    }
}
impl Neg for Fixed16_16 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl AddAssign for Fixed16_16 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for Fixed16_16 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl MulAssign for Fixed16_16 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl DivAssign for Fixed16_16 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

// Comparison
impl PartialOrd for Fixed16_16 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Fixed16_16 {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

// Numeric traits
impl Zero for Fixed16_16 {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
impl One for Fixed16_16 {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }
}

impl fmt::Debug for Fixed16_16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fixed16_16({:.5})", self.to_f32())
    }
}
impl fmt::Display for Fixed16_16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.5}", self.to_f32())
    }
}

// Conversion from primitives
impl From<i32> for Fixed16_16 {
    #[inline]
    fn from(value: i32) -> Self {
        Self::from_int(value)
    }
}
impl From<i16> for Fixed16_16 {
    #[inline]
    fn from(value: i16) -> Self {
        Self::from_int(value as i32)
    }
}
impl From<i8> for Fixed16_16 {
    #[inline]
    fn from(value: i8) -> Self {
        Self::from_int(value as i32)
    }
}

// For debug/testing only - not for deterministic gameplay
impl From<f32> for Fixed16_16 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}
impl From<f64> for Fixed16_16 {
    fn from(value: f64) -> Self {
        Self::from_f32(value as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let a = Fixed16_16::from_int(5);
        let b = Fixed16_16::from_int(3);
        assert_eq!(a + b, Fixed16_16::from_int(8));
        assert_eq!(a - b, Fixed16_16::from_int(2));
        assert_eq!(a * b, Fixed16_16::from_int(15));
        assert_eq!(a / b, Fixed16_16::from_ratio(5, 3));
    }

    #[test]
    fn test_from_int() {
        assert_eq!(Fixed16_16::from_int(42).to_bits(), 42 * 65536);
    }

    #[test]
    fn test_sqrt() {
        let four = Fixed16_16::from_int(4);
        let sqrt = four.sqrt();
        assert!((sqrt - Fixed16_16::from_int(2)).abs() < Fixed16_16::from_f32(0.001));
    }

    #[test]
    fn test_sin_cos() {
        let zero = Fixed16_16::ZERO;
        assert_eq!(zero.sin(), Fixed16_16::ZERO);
        assert_eq!(zero.cos(), Fixed16_16::ONE);

        let pi_2 = Fixed16_16::FRAC_PI_2;
        assert!((pi_2.sin() - Fixed16_16::ONE).abs() < Fixed16_16::from_f32(0.01));
        assert!(pi_2.cos().abs() < Fixed16_16::from_f32(0.01));
    }

    #[test]
    fn test_atan2() {
        let y = Fixed16_16::ONE;
        let x = Fixed16_16::ZERO;
        let angle = Fixed16_16::atan2(y, x);
        assert!((angle - Fixed16_16::FRAC_PI_2).abs() < Fixed16_16::from_f32(0.01));
    }

    #[test]
    fn test_overflow_behavior() {
        let max = Fixed16_16::MAX;
        let one = Fixed16_16::ONE;
        // Wrapping addition should overflow without panicking
        let result = max + one;
        assert_eq!(result.0, i32::MAX.wrapping_add(one.0));
    }

    /// Property-based tests (proptest).
    ///
    /// Note: `neg`/`abs` use `-self.0`/`self.0.abs()`, which overflow-panic on
    /// the `i32::MIN` bit pattern in debug builds — those cases are excluded.
    #[cfg(test)]
    mod property_tests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(256))]

            #[test]
            fn wrapping_add_sub_act_on_i32_bits(a in any::<i32>(), b in any::<i32>()) {
                let fa = Fixed16_16::from_bits(a);
                let fb = Fixed16_16::from_bits(b);
                prop_assert_eq!(fa.wrapping_add(fb).to_bits(), a.wrapping_add(b));
                prop_assert_eq!(fa.wrapping_sub(fb).to_bits(), a.wrapping_sub(b));
                // The `+`/`-` operators delegate to the wrapping variants.
                prop_assert_eq!((fa + fb).to_bits(), a.wrapping_add(b));
                prop_assert_eq!((fa - fb).to_bits(), a.wrapping_sub(b));
            }

            #[test]
            fn from_int_round_trips_for_small_integers(v in -32767i32..=32767) {
                let f = Fixed16_16::from_int(v);
                prop_assert_eq!(f.to_bits(), v * Fixed16_16::SCALE);
                prop_assert_eq!(f.int(), v);
            }

            #[test]
            fn neg_is_involutive(a in any::<i32>()) {
                prop_assume!(a != i32::MIN);
                let f = Fixed16_16::from_bits(a);
                prop_assert_eq!((-(-f)).to_bits(), a);
            }

            #[test]
            fn abs_is_nonnegative_and_idempotent(a in any::<i32>()) {
                prop_assume!(a != i32::MIN);
                let f = Fixed16_16::from_bits(a);
                let abs = f.abs();
                prop_assert!(abs.to_bits() >= 0);
                prop_assert_eq!(abs.abs().to_bits(), abs.to_bits());
            }

            #[test]
            fn ordering_matches_i32_ordering(a in any::<i32>(), b in any::<i32>()) {
                let fa = Fixed16_16::from_bits(a);
                let fb = Fixed16_16::from_bits(b);
                prop_assert_eq!(fa < fb, a < b);
                prop_assert_eq!(fa.cmp(&fb), a.cmp(&b));
            }

            #[test]
            fn mul_matches_i64_shift_for_nonoverflowing_inputs(
                a in -0x40_0000i32..0x40_0000,
                b in -0x40_0000i32..0x40_0000,
            ) {
                // |a|,|b| < 2^22 so the product < 2^44 and `>> 16` always fits
                // in i32: the inputs are non-overflowing by construction.
                let expected = (a as i64 * b as i64) >> 16;
                let fa = Fixed16_16::from_bits(a);
                let fb = Fixed16_16::from_bits(b);
                prop_assert_eq!(fa.wrapping_mul(fb).to_bits(), expected as i32);
                prop_assert_eq!((fa * fb).to_bits(), expected as i32);
            }

            #[test]
            fn wrapping_div_by_zero_is_zero(a in any::<i32>()) {
                let fa = Fixed16_16::from_bits(a);
                prop_assert_eq!(fa.wrapping_div(Fixed16_16::ZERO), Fixed16_16::ZERO);
                prop_assert_eq!((fa / Fixed16_16::ZERO).to_bits(), 0);
            }

            #[test]
            fn saturating_div_by_zero_saturates(a in any::<i32>()) {
                let fa = Fixed16_16::from_bits(a);
                let expected = if a >= 0 { Fixed16_16::MAX } else { Fixed16_16::MIN };
                prop_assert_eq!(fa.saturating_div(Fixed16_16::ZERO), expected);
            }

            #[test]
            fn div_matches_i64_formula_for_nonoverflowing_inputs(a in any::<i32>(), b in any::<i32>()) {
                prop_assume!(b != 0);
                let expected = ((a as i64) << 16) / (b as i64);
                prop_assume!(expected >= i64::from(i32::MIN) && expected <= i64::from(i32::MAX));
                let fa = Fixed16_16::from_bits(a);
                let fb = Fixed16_16::from_bits(b);
                prop_assert_eq!(fa.wrapping_div(fb).to_bits(), expected as i32);
            }

            #[test]
            fn sqrt_squares_back_within_tolerance(v in 0.0f32..=1000.0) {
                let x = Fixed16_16::from_f32(v);
                let root = x.sqrt();
                prop_assert!(root >= Fixed16_16::ZERO);
                // `sqrt` is a floor sqrt, so `root^2 <= x` up to rounding of
                // `from_f32`; the worst-case gap for |x| <= 1000 is ~0.001.
                prop_assert!(
                    (x - root.wrapping_mul(root)).abs() <= Fixed16_16::from_f32(0.01),
                    "sqrt({v:?})^2 = {:?}, expected near {:?}",
                    root.wrapping_mul(root),
                    x
                );
            }
        }
    }
}
