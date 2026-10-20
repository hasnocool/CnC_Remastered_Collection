//! Small deterministic math helpers shared by the generators.
//!
//! Kept local to this crate (rather than added to `cnc-core`) so the
//! simulation-facing core API does not grow surface just for asset
//! generation.

/// Clamp to `[0.0, 1.0]`.
pub fn clamp01(v: f64) -> f64 {
    v.clamp(0.0, 1.0)
}

/// Linear interpolation: `a + (b - a) * t`.
pub fn mix(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Hermite smoothstep on `[0, 1]` input (2t^3 - 3t^2 + 1 with corrected
/// polynomial); input is clamped, so extrapolation never happens.
pub fn smoothstep(t: f64) -> f64 {
    let x = clamp01(t);
    x * x * (3.0 - 2.0 * x)
}

/// Map a value from one range to another (linear).
pub fn remap(v: f64, in_lo: f64, in_hi: f64, out_lo: f64, out_hi: f64) -> f64 {
    debug_assert!(in_hi > in_lo, "degenerate input range");
    mix(out_lo, out_hi, clamp01((v - in_lo) / (in_hi - in_lo)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_bounds() {
        assert_eq!(clamp01(-1.0), 0.0);
        assert_eq!(clamp01(0.5), 0.5);
        assert_eq!(clamp01(2.0), 1.0);
    }

    #[test]
    fn mix_endpoints() {
        assert_eq!(mix(1.0, 3.0, 0.0), 1.0);
        assert_eq!(mix(1.0, 3.0, 1.0), 3.0);
        assert_eq!(mix(1.0, 3.0, 0.5), 2.0);
    }

    #[test]
    fn smoothstep_endpoints_and_zero_derivative() {
        assert_eq!(smoothstep(0.0), 0.0);
        assert_eq!(smoothstep(1.0), 1.0);
        // Symmetric around the midpoint.
        assert!((smoothstep(0.5) - 0.5).abs() < 1e-12);
        assert!((smoothstep(0.25) + smoothstep(0.75) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn remap_clamps_input_range() {
        assert_eq!(remap(5.0, 0.0, 10.0, 100.0, 200.0), 150.0);
        assert_eq!(remap(-5.0, 0.0, 10.0, 100.0, 200.0), 100.0);
        assert_eq!(remap(11.0, 0.0, 10.0, 100.0, 200.0), 200.0);
    }
}
