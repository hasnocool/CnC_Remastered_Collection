//! Math types: vectors, matrices, rectangles

use crate::{Fixed16_16, FloatBias};
use bytemuck::{Pod, Zeroable};
use core::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

// ============================================================================
// Integer Vector Types
// ============================================================================

/// 2D integer vector
#[repr(C)]
#[derive(
    Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable, Debug,
)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    #[inline]
    pub const fn one() -> Self {
        Self { x: 1, y: 1 }
    }

    #[inline]
    pub fn length_squared(self) -> i32 {
        self.x
            .saturating_mul(self.x)
            .saturating_add(self.y.saturating_mul(self.y))
    }

    #[inline]
    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.x - other.x)
            .abs()
            .saturating_add((self.y - other.y).abs())
    }

    #[inline]
    pub fn chebyshev_distance(self, other: Self) -> i32 {
        (self.x - other.x).abs().max((self.y - other.y).abs())
    }
}

impl Add for Vec2i {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }
}
impl Sub for Vec2i {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}
impl Mul<i32> for Vec2i {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x.saturating_mul(rhs),
            y: self.y.saturating_mul(rhs),
        }
    }
}
impl Div<i32> for Vec2i {
    type Output = Self;
    #[inline]
    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Neg for Vec2i {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// 3D integer vector
#[repr(C)]
#[derive(
    Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable, Debug,
)]
pub struct Vec3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3i {
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

// ============================================================================
// Fixed16_16 Vector Types
// ============================================================================

/// 2D fixed-point vector
#[repr(C)]
#[derive(
    Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable, Debug,
)]
pub struct Vec2 {
    pub x: Fixed16_16,
    pub y: Fixed16_16,
}

/// Integer square root for 128-bit values (used for long vector lengths).
fn isqrt_u128(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = n;
    let mut y = x.div_ceil(2);
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

impl Vec2 {
    #[inline]
    pub const fn new(x: Fixed16_16, y: Fixed16_16) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self {
            x: Fixed16_16::ZERO,
            y: Fixed16_16::ZERO,
        }
    }

    #[inline]
    pub const fn one() -> Self {
        Self {
            x: Fixed16_16::ONE,
            y: Fixed16_16::ONE,
        }
    }

    #[inline]
    pub fn length_squared(self) -> Fixed16_16 {
        // Use a 64-bit intermediate: the plain fixed-point multiply overflows
        // for vectors longer than ~181 world units.
        let x = self.x.to_bits() as i64;
        let y = self.y.to_bits() as i64;
        let squared = ((x * x) >> 16) + ((y * y) >> 16);
        Fixed16_16::from_bits(squared.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
    }

    #[inline]
    pub fn length(self) -> Fixed16_16 {
        // Exact 128-bit integer sqrt of (x^2 + y^2), avoiding the i32 overflow
        // that length_squared suffers for distances beyond ~181 units.
        let x = self.x.to_bits() as i128;
        let y = self.y.to_bits() as i128;
        let sum = (x * x + y * y) as u128;
        Fixed16_16::from_bits(isqrt_u128(sum).min(i32::MAX as u128) as i32)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == Fixed16_16::ZERO {
            Self::zero()
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }

    #[inline]
    pub fn dot(self, other: Self) -> Fixed16_16 {
        let value = (self.x.to_bits() as i64 * other.x.to_bits() as i64
            + self.y.to_bits() as i64 * other.y.to_bits() as i64)
            >> 16;
        Fixed16_16::from_bits(value.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
    }

    #[inline]
    pub fn distance(self, other: Self) -> Fixed16_16 {
        (self - other).length()
    }
}

impl Add for Vec2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Vec2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Mul<Fixed16_16> for Vec2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Fixed16_16) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div<Fixed16_16> for Vec2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Fixed16_16) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl Neg for Vec2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// 3D fixed-point vector
#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable)]
pub struct Vec3 {
    pub x: Fixed16_16,
    pub y: Fixed16_16,
    pub z: Fixed16_16,
}

impl Vec3 {
    #[inline]
    pub const fn new(x: Fixed16_16, y: Fixed16_16, z: Fixed16_16) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self {
            x: Fixed16_16::ZERO,
            y: Fixed16_16::ZERO,
            z: Fixed16_16::ZERO,
        }
    }

    #[inline]
    pub fn length_squared(self) -> Fixed16_16 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(self) -> Fixed16_16 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn dot(self, other: Self) -> Fixed16_16 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul<Fixed16_16> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Fixed16_16) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div<Fixed16_16> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Fixed16_16) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// ============================================================================
// FloatBias Vector Types (for TD)
// ============================================================================

/// 2D float-bias vector
#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Serialize, Deserialize, Pod, Zeroable)]
pub struct Vec2f {
    pub x: FloatBias,
    pub y: FloatBias,
}

impl Vec2f {
    #[inline]
    pub const fn new(x: FloatBias, y: FloatBias) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self {
            x: FloatBias::ZERO,
            y: FloatBias::ZERO,
        }
    }

    #[inline]
    pub fn length_squared(self) -> FloatBias {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn length(self) -> FloatBias {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len.is_zero() {
            Self::zero()
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }

    #[inline]
    pub fn dot(self, other: Self) -> FloatBias {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Vec2f {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Vec2f {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Mul<FloatBias> for Vec2f {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: FloatBias) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div<FloatBias> for Vec2f {
    type Output = Self;
    #[inline]
    fn div(self, rhs: FloatBias) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// ============================================================================
// Matrix Types
// ============================================================================

/// 2x2 fixed-point matrix
#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Pod, Zeroable)]
pub struct Mat2x2 {
    pub m11: Fixed16_16,
    pub m12: Fixed16_16,
    pub m21: Fixed16_16,
    pub m22: Fixed16_16,
}

impl Mat2x2 {
    #[inline]
    pub const fn identity() -> Self {
        Self {
            m11: Fixed16_16::ONE,
            m12: Fixed16_16::ZERO,
            m21: Fixed16_16::ZERO,
            m22: Fixed16_16::ONE,
        }
    }

    #[inline]
    pub fn mul_vec(self, v: Vec2) -> Vec2 {
        Vec2 {
            x: self.m11 * v.x + self.m12 * v.y,
            y: self.m21 * v.x + self.m22 * v.y,
        }
    }

    /// Rotation matrix
    pub fn rotation(angle: Fixed16_16) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m11: c,
            m12: -s,
            m21: s,
            m22: c,
        }
    }

    /// Scale matrix
    pub fn scale(sx: Fixed16_16, sy: Fixed16_16) -> Self {
        Self {
            m11: sx,
            m12: Fixed16_16::ZERO,
            m21: Fixed16_16::ZERO,
            m22: sy,
        }
    }
}

impl Mul for Mat2x2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
        }
    }
}

/// 3x3 fixed-point matrix (for 2D affine transforms)
#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Pod, Zeroable)]
pub struct Mat3x3 {
    pub m11: Fixed16_16,
    pub m12: Fixed16_16,
    pub m13: Fixed16_16,
    pub m21: Fixed16_16,
    pub m22: Fixed16_16,
    pub m23: Fixed16_16,
    pub m31: Fixed16_16,
    pub m32: Fixed16_16,
    pub m33: Fixed16_16,
}

impl Mat3x3 {
    #[inline]
    pub const fn identity() -> Self {
        Self {
            m11: Fixed16_16::ONE,
            m12: Fixed16_16::ZERO,
            m13: Fixed16_16::ZERO,
            m21: Fixed16_16::ZERO,
            m22: Fixed16_16::ONE,
            m23: Fixed16_16::ZERO,
            m31: Fixed16_16::ZERO,
            m32: Fixed16_16::ZERO,
            m33: Fixed16_16::ONE,
        }
    }

    /// Translation matrix
    pub fn translation(tx: Fixed16_16, ty: Fixed16_16) -> Self {
        Self {
            m11: Fixed16_16::ONE,
            m12: Fixed16_16::ZERO,
            m13: tx,
            m21: Fixed16_16::ZERO,
            m22: Fixed16_16::ONE,
            m23: ty,
            m31: Fixed16_16::ZERO,
            m32: Fixed16_16::ZERO,
            m33: Fixed16_16::ONE,
        }
    }

    /// Scale matrix
    pub fn scale(sx: Fixed16_16, sy: Fixed16_16) -> Self {
        Self {
            m11: sx,
            m12: Fixed16_16::ZERO,
            m13: Fixed16_16::ZERO,
            m21: Fixed16_16::ZERO,
            m22: sy,
            m23: Fixed16_16::ZERO,
            m31: Fixed16_16::ZERO,
            m32: Fixed16_16::ZERO,
            m33: Fixed16_16::ONE,
        }
    }

    /// Rotation matrix
    pub fn rotation(angle: Fixed16_16) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self {
            m11: c,
            m12: -s,
            m13: Fixed16_16::ZERO,
            m21: s,
            m22: c,
            m23: Fixed16_16::ZERO,
            m31: Fixed16_16::ZERO,
            m32: Fixed16_16::ZERO,
            m33: Fixed16_16::ONE,
        }
    }

    /// Multiply by Vec2 (with translation)
    pub fn mul_vec(self, v: Vec2) -> Vec2 {
        Vec2 {
            x: self.m11 * v.x + self.m12 * v.y + self.m13,
            y: self.m21 * v.x + self.m22 * v.y + self.m23,
        }
    }
}

impl Mul for Mat3x3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // Standard 3x3 matrix multiplication
        Self {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
        }
    }
}

// ============================================================================
// Rectangle and Bounds
// ============================================================================

/// Rectangle with integer coordinates
#[repr(C)]
#[derive(
    Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize, Pod, Zeroable, Debug,
)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    #[inline]
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    #[inline]
    pub fn left(self) -> i32 {
        self.x
    }

    #[inline]
    pub fn right(self) -> i32 {
        self.x + self.width
    }

    #[inline]
    pub fn top(self) -> i32 {
        self.y
    }

    #[inline]
    pub fn bottom(self) -> i32 {
        self.y + self.height
    }

    #[inline]
    pub fn center(self) -> Vec2i {
        Vec2i::new(self.x + self.width / 2, self.y + self.height / 2)
    }

    #[inline]
    pub fn contains(self, point: Vec2i) -> bool {
        point.x >= self.left()
            && point.x < self.right()
            && point.y >= self.top()
            && point.y < self.bottom()
    }

    #[inline]
    pub fn intersects(self, other: Self) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    #[inline]
    pub fn intersection(self, other: Self) -> Option<Self> {
        let left = self.left().max(other.left());
        let right = self.right().min(other.right());
        let top = self.top().max(other.top());
        let bottom = self.bottom().min(other.bottom());

        if left < right && top < bottom {
            Some(Self::new(left, top, right - left, bottom - top))
        } else {
            None
        }
    }
}

/// Bounding box with fixed-point coordinates
#[repr(C)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Pod, Zeroable)]
pub struct Bounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl Bounds {
    #[inline]
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn empty() -> Self {
        Self {
            min: Vec2::new(Fixed16_16::MAX, Fixed16_16::MAX),
            max: Vec2::new(Fixed16_16::MIN, Fixed16_16::MIN),
        }
    }

    #[inline]
    pub fn from_point(point: Vec2) -> Self {
        Self {
            min: point,
            max: point,
        }
    }

    #[inline]
    pub fn expand(&mut self, point: Vec2) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
    }

    #[inline]
    pub fn width(self) -> Fixed16_16 {
        self.max.x - self.min.x
    }

    #[inline]
    pub fn height(self) -> Fixed16_16 {
        self.max.y - self.min.y
    }

    #[inline]
    pub fn center(self) -> Vec2 {
        Vec2 {
            x: (self.min.x + self.max.x) / Fixed16_16::from_int(2),
            y: (self.min.y + self.max.y) / Fixed16_16::from_int(2),
        }
    }

    #[inline]
    pub fn contains(self, point: Vec2) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }

    #[inline]
    pub fn intersects(self, other: Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2i() {
        let a = Vec2i::new(3, 4);
        let b = Vec2i::new(1, 2);
        assert_eq!(a + b, Vec2i::new(4, 6));
        assert_eq!(a - b, Vec2i::new(2, 2));
        assert_eq!(a.length_squared(), 25);
        assert_eq!(a.manhattan_distance(b), 4);
    }

    #[test]
    fn test_vec2_fixed() {
        let a = Vec2::new(Fixed16_16::from_int(3), Fixed16_16::from_int(4));
        assert_eq!(a.length(), Fixed16_16::from_int(5));
    }

    #[test]
    fn test_vec2_long_length() {
        let v = Vec2::new(Fixed16_16::from_int(336), Fixed16_16::ZERO);
        assert!((v.length().to_f32() - 336.0).abs() < 0.01);

        let n = v.normalize();
        assert!((n.x.to_f32() - 1.0).abs() < 0.001);
        assert!(n.y.to_f32().abs() < 0.001);
    }

    #[test]
    fn test_fixed_sqrt_large_value() {
        let v = Fixed16_16::from_int(10000);
        assert!((v.sqrt().to_f32() - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_rect() {
        let r = Rect::new(0, 0, 10, 10);
        assert!(r.contains(Vec2i::new(5, 5)));
        assert!(!r.contains(Vec2i::new(15, 5)));
        assert!(r.intersects(Rect::new(5, 5, 10, 10)));
    }

    #[test]
    fn test_mat2x2_rotation() {
        let rot = Mat2x2::rotation(Fixed16_16::FRAC_PI_2);
        let v = Vec2::new(Fixed16_16::ONE, Fixed16_16::ZERO);
        let result = rot.mul_vec(v);
        assert!(result.x.abs() < Fixed16_16::from_f32(0.01));
        assert!((result.y - Fixed16_16::ONE).abs() < Fixed16_16::from_f32(0.01));
    }
}
