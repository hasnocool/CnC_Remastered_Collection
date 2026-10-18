//! Float bias arithmetic (matching Tiberian Dawn's float bias system)

use core::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use core::cmp::Ordering;
use core::fmt;
use num_traits::{Zero, One};
use serde::{Serialize, Deserialize};
use bytemuck::{Pod, Zeroable};

/// Tiberian Dawn's float bias system
/// Original uses float with a bias value for deterministic comparison
/// We replicate this exactly for demo/MP compatibility
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Serialize, Deserialize, Pod, Zeroable)]
pub struct FloatBias(pub f32);

impl FloatBias {
    /// Bias value used by Tiberian Dawn for float comparisons
    /// This ensures deterministic floating-point behavior across platforms
    pub const BIAS: f32 = 1.0 / 65536.0; // 1/65536 ≈ 0.0000152587890625
    /// Maximum value
    pub const MAX: Self = Self(f32::MAX);
    /// Minimum value
    pub const MIN: Self = Self(f32::MIN);
    /// One (1.0)
    pub const ONE: Self = Self(1.0);
    /// Zero (0.0)
    pub const ZERO: Self = Self(0.0);
    /// Half (0.5)
    pub const HALF: Self = Self(0.5);
    /// Pi
    pub const PI: Self = Self(std::f32::consts::PI);
    /// 2 * Pi
    pub const TAU: Self = Self(std::f32::consts::TAU);
    /// Epsilon for comparisons
    pub const EPSILON: Self = Self(Self::BIAS);

    /// Create from raw f32
    #[inline]
    pub const fn from_f32(value: f32) -> Self {
        Self(value)
    }

    /// Get raw f32
    #[inline]
    pub const fn to_f32(self) -> f32 {
        self.0
    }

    /// Create from integer
    #[inline]
    pub const fn from_int(value: i32) -> Self {
        Self(value as f32)
    }

    /// Deterministic equality with bias
    /// Two floats are equal if their difference is within BIAS
    #[inline]
    pub fn approx_eq(self, other: Self) -> bool {
        (self.0 - other.0).abs() <= Self::BIAS
    }

    /// Deterministic less-than with bias
    #[inline]
    pub fn approx_lt(self, other: Self) -> bool {
        self.0 < other.0 - Self::BIAS
    }

    /// Deterministic greater-than with bias
    #[inline]
    pub fn approx_gt(self, other: Self) -> bool {
        self.0 > other.0 + Self::BIAS
    }

    /// Deterministic less-than-or-equal with bias
    #[inline]
    pub fn approx_le(self, other: Self) -> bool {
        self.0 <= other.0 + Self::BIAS
    }

    /// Deterministic greater-than-or-equal with bias
    #[inline]
    pub fn approx_ge(self, other: Self) -> bool {
        self.0 >= other.0 - Self::BIAS
    }

    /// Square root
    #[inline]
    pub fn sqrt(self) -> Self {
        Self(self.0.sqrt())
    }

    /// Sine
    #[inline]
    pub fn sin(self) -> Self {
        Self(self.0.sin())
    }

    /// Cosine
    #[inline]
    pub fn cos(self) -> Self {
        Self(self.0.cos())
    }

    /// Arctangent2
    #[inline]
    pub fn atan2(y: Self, x: Self) -> Self {
        Self(y.0.atan2(x.0))
    }

    /// Absolute value
    #[inline]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Floor
    #[inline]
    pub fn floor(self) -> Self {
        Self(self.0.floor())
    }

    /// Ceil
    #[inline]
    pub fn ceil(self) -> Self {
        Self(self.0.ceil())
    }

    /// Round
    #[inline]
    pub fn round(self) -> Self {
        Self(self.0.round())
    }

    /// Trunc
    #[inline]
    pub fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    /// Fract
    #[inline]
    pub fn fract(self) -> Self {
        Self(self.0.fract())
    }

    /// Min with bias
    #[inline]
    pub fn min(self, other: Self) -> Self {
        if self.approx_lt(other) { self } else { other }
    }

    /// Max with bias
    #[inline]
    pub fn max(self, other: Self) -> Self {
        if self.approx_gt(other) { self } else { other }
    }

    /// Clamp with bias
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        if self.approx_lt(min) {
            min
        } else if self.approx_gt(max) {
            max
        } else {
            self
        }
    }

    /// Convert to integer (truncated)
    #[inline]
    pub fn to_int(self) -> i32 {
        self.0.trunc() as i32
    }
}

// Standard arithmetic (using raw f32 operations for compatibility)
impl Add for FloatBias {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for FloatBias {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl Mul for FloatBias {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}
impl Div for FloatBias {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
impl Neg for FloatBias {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl AddAssign for FloatBias {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl SubAssign for FloatBias {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl MulAssign for FloatBias {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl DivAssign for FloatBias {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

// Comparison with bias
impl PartialOrd for FloatBias {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.approx_eq(*other) {
            Some(Ordering::Equal)
        } else if self.approx_lt(*other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

// Numeric traits
impl Zero for FloatBias {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }
    #[inline]
    fn is_zero(&self) -> bool {
        self.approx_eq(Self::ZERO)
    }
}
impl One for FloatBias {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }
}

impl fmt::Debug for FloatBias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FloatBias({:.5})", self.0)
    }
}
impl fmt::Display for FloatBias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.5}", self.0)
    }
}

// Conversions
impl From<f32> for FloatBias {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value)
    }
}
impl From<f64> for FloatBias {
    #[inline]
    fn from(value: f64) -> Self {
        Self(value as f32)
    }
}
impl From<i32> for FloatBias {
    #[inline]
    fn from(value: i32) -> Self {
        Self(value as f32)
    }
}
impl From<i16> for FloatBias {
    #[inline]
    fn from(value: i16) -> Self {
        Self(value as f32)
    }
}
impl From<i8> for FloatBias {
    #[inline]
    fn from(value: i8) -> Self {
        Self(value as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bias_comparison() {
        let a = FloatBias::from_f32(1.0);
        let b = FloatBias::from_f32(1.0 + FloatBias::BIAS / 2.0);
        assert!(a.approx_eq(b));

        let c = FloatBias::from_f32(1.0 + FloatBias::BIAS * 2.0);
        assert!(!a.approx_eq(c));
    }

    #[test]
    fn test_arithmetic() {
        let a = FloatBias::from_f32(5.0);
        let b = FloatBias::from_f32(3.0);
        assert!((a + b).approx_eq(FloatBias::from_f32(8.0)));
        assert!((a - b).approx_eq(FloatBias::from_f32(2.0)));
        assert!((a * b).approx_eq(FloatBias::from_f32(15.0)));
        assert!((a / b).approx_eq(FloatBias::from_f32(5.0 / 3.0)));
    }

    #[test]
    fn test_sqrt() {
        let four = FloatBias::from_f32(4.0);
        let sqrt = four.sqrt();
        assert!(sqrt.approx_eq(FloatBias::from_f32(2.0)));
    }

    #[test]
    fn test_trig() {
        let zero = FloatBias::ZERO;
        assert!(zero.sin().approx_eq(FloatBias::ZERO));
        assert!(zero.cos().approx_eq(FloatBias::ONE));
    }
}
