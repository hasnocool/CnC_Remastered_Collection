//! Camera with View/Projection matrices
use cnc_core::{Fixed16_16, Mat3x3, Rect, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec2,
    pub zoom: Fixed16_16,
    pub rotation: Fixed16_16,
    pub viewport_width: u32,
    pub viewport_height: u32,
    /// Current screen-shake intensity in screen pixels (0 = none).
    pub shake_intensity: f32,
    /// Fraction of shake intensity kept for each elapsed tick (decay),
    /// clamped to 0.0..=1.0 (values outside decay instantly).
    pub shake_decay: f32,
    /// Deterministic LCG state for the shake offset (no rand dependency).
    shake_seed: u32,
    /// Current shake offset applied to the effective camera position.
    shake_offset: Vec2,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::zero(),
            zoom: Fixed16_16::ONE,
            rotation: Fixed16_16::ZERO,
            viewport_width: 800,
            viewport_height: 600,
            shake_intensity: 0.0,
            shake_decay: 0.85,
            shake_seed: 0x9E37_79B9,
            shake_offset: Vec2::zero(),
        }
    }
}

impl Camera {
    pub fn view_matrix(&self) -> Mat3x3 {
        // Translation to center camera on position
        Mat3x3::translation(-self.position.x, -self.position.y)
    }

    pub fn projection_matrix(&self) -> Mat3x3 {
        // Orthographic projection for 2D
        let zoom = self.zoom.to_f32();

        // World units per screen pixel
        let world_width = (self.viewport_width as f32 / zoom) / 2.0;
        let world_height = (self.viewport_height as f32 / zoom) / 2.0;

        Mat3x3::scale(
            Fixed16_16::from_f32(1.0 / world_width),
            Fixed16_16::from_f32(1.0 / world_height),
        )
    }

    pub fn view_projection_matrix(&self) -> [[f32; 4]; 4] {
        let view = self.view_matrix();
        let proj = self.projection_matrix();

        // Convert to 4x4 matrix for GPU
        let mut m = [[0.0f32; 4]; 4];
        m[0][0] = proj.m11.to_f32();
        m[0][1] = proj.m12.to_f32();
        m[0][2] = 0.0;
        m[0][3] = proj.m13.to_f32();
        m[1][0] = proj.m21.to_f32();
        m[1][1] = proj.m22.to_f32();
        m[1][2] = 0.0;
        m[1][3] = proj.m23.to_f32();
        m[2][0] = 0.0;
        m[2][1] = 0.0;
        m[2][2] = 1.0;
        m[2][3] = 0.0;
        m[3][0] = view.m13.to_f32();
        m[3][1] = view.m23.to_f32();
        m[3][2] = 0.0;
        m[3][3] = 1.0;

        // Multiply proj * view
        // For orthographic 2D, we can simplify
        m
    }

    pub fn world_to_screen(&self, world: Vec2) -> Vec2 {
        let relative = world - self.position;
        let zoom_f = self.zoom.to_f32();
        Vec2::new(
            Fixed16_16::from_f32(relative.x.to_f32() * zoom_f + self.viewport_width as f32 / 2.0),
            Fixed16_16::from_f32(relative.y.to_f32() * zoom_f + self.viewport_height as f32 / 2.0),
        )
    }

    pub fn screen_to_world(&self, screen: Vec2) -> Vec2 {
        let zoom_f = self.zoom.to_f32();
        Vec2::new(
            Fixed16_16::from_f32(
                (screen.x.to_f32() - self.viewport_width as f32 / 2.0) / zoom_f
                    + self.position.x.to_f32(),
            ),
            Fixed16_16::from_f32(
                (screen.y.to_f32() - self.viewport_height as f32 / 2.0) / zoom_f
                    + self.position.y.to_f32(),
            ),
        )
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport_width = width;
        self.viewport_height = height;
    }

    pub fn zoom_in(&mut self, factor: f32) {
        self.zoom *= Fixed16_16::from_f32(factor);
    }

    pub fn zoom_out(&mut self, factor: f32) {
        self.zoom /= Fixed16_16::from_f32(factor);
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.position = self.position + delta;
    }

    // ========================================================================
    // Screen shake
    // ========================================================================

    /// Start (or re-amplify) a screen shake in screen pixels. The jitter
    /// offset stays inside ±intensity/2 until it decays back to zero.
    pub fn add_shake(&mut self, intensity: f32) {
        if intensity < 0.0 {
            return;
        }
        self.shake_intensity = self.shake_intensity.max(intensity);
    }

    /// Decay the current shake over `ticks` game ticks and draw a fresh
    /// deterministic jitter offset. The seed lives on the camera so the
    /// simulation stays deterministic (same tick count = same offset).
    pub fn update_shake(&mut self, ticks: u32) {
        if ticks == 0 {
            return;
        }
        let decay = self.shake_decay.clamp(0.0, 1.0).powi(ticks as i32);
        self.shake_intensity *= decay;
        if self.shake_intensity < 0.05 {
            self.shake_intensity = 0.0;
            self.shake_offset = Vec2::zero();
            return;
        }
        // xorshift32: tiny deterministic PRNG (no `rand` dependency needed).
        let mut state = self.shake_seed;
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        self.shake_seed = state;
        let half = self.shake_intensity * 0.5;
        let fx = (state as f32 / u32::MAX as f32) * self.shake_intensity;
        let fy = ((state >> 16) as f32 / u32::MAX as f32) * self.shake_intensity;
        self.shake_offset = Vec2::new(
            Fixed16_16::from_f32(fx - half),
            Fixed16_16::from_f32(fy - half),
        );
    }

    /// Camera position including the current shake offset. Rendering paths
    /// (view/projection) consume this; picking (`screen_to_world`) does not,
    /// so clicks stay anchored to the unshaken simulation position.
    pub fn effective_position(&self) -> Vec2 {
        self.position + self.shake_offset
    }

    /// Current shake jitter in screen pixels (0 when the shake ended).
    pub fn shake_offset_pixels(&self) -> (f32, f32) {
        (self.shake_offset.x.to_f32(), self.shake_offset.y.to_f32())
    }

    /// True when a shake is active (intensity above the cutoff).
    pub fn is_shaking(&self) -> bool {
        self.shake_intensity > 0.0
    }

    // ========================================================================
    // Viewport clipping
    // ========================================================================

    /// World-space rectangle the viewport currently covers (shake ignored).
    pub fn visible_rect(&self) -> Rect {
        let half_w = self.viewport_width as f32 / (2.0 * self.zoom.to_f32().max(1.0e-6));
        let half_h = self.viewport_height as f32 / (2.0 * self.zoom.to_f32().max(1.0e-6));
        let cx = self.position.x.to_f32();
        let cy = self.position.y.to_f32();
        Rect::new(
            (cx - half_w) as i32,
            (cy - half_h) as i32,
            (half_w * 2.0) as i32,
            (half_h * 2.0) as i32,
        )
    }

    /// Whether a world point is inside the visible area (with margin pixels).
    pub fn is_on_screen(&self, world: Vec2, margin: f32) -> bool {
        let rect = self.visible_rect();
        let x = world.x.to_f32();
        let y = world.y.to_f32();
        x >= (rect.x as f32 - margin)
            && x <= (rect.x as f32 + rect.width as f32 + margin)
            && y >= (rect.y as f32 - margin)
            && y <= (rect.y as f32 + rect.height as f32 + margin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shake_jitter_is_deterministic_and_bounded() {
        let mut camera = Camera::default();
        camera.add_shake(8.0);
        camera.update_shake(1);
        assert!(camera.is_shaking());
        let (px, py) = camera.shake_offset_pixels();
        assert!(px.abs() <= 4.0, "offset within half intensity: {px}");
        assert!(py.abs() <= 4.0, "offset within half intensity: {py}");

        let mut other = Camera::default();
        other.add_shake(8.0);
        other.update_shake(1);
        assert_eq!(camera.shake_offset_pixels(), other.shake_offset_pixels());
    }

    #[test]
    fn shake_decays_to_rest() {
        let mut camera = Camera::default();
        camera.add_shake(12.0);
        for _ in 0..600 {
            camera.update_shake(1);
            if !camera.is_shaking() {
                break;
            }
        }
        assert!(!camera.is_shaking(), "shake must decay fully with decay<1");
        assert_eq!(camera.shake_offset_pixels(), (0.0, 0.0));
    }

    #[test]
    fn add_shake_takes_max_and_rejects_negative() {
        let mut camera = Camera::default();
        camera.add_shake(5.0);
        camera.add_shake(2.0);
        assert_eq!(camera.shake_intensity, 5.0, "re-amplify takes the max");
        camera.add_shake(-3.0);
        assert_eq!(camera.shake_intensity, 5.0);
    }

    #[test]
    fn screen_to_world_ignores_shake() {
        let mut camera = Camera::default();
        camera.add_shake(6.0);
        camera.update_shake(1);
        let world = Vec2::new(Fixed16_16::from_f32(35.0), Fixed16_16::from_f32(35.0));
        let screen = camera.world_to_screen(world);
        assert_eq!(camera.screen_to_world(screen), world);
    }

    #[test]
    fn viewport_clipping_covers_the_visible_world_rect() {
        let camera = Camera::default();
        let rect = camera.visible_rect();
        assert_eq!((rect.x, rect.y), (-400, -300));
        assert_eq!((rect.width, rect.height), (800, 600));
        let inside = Vec2::new(Fixed16_16::ZERO, Fixed16_16::ZERO);
        let outside = Vec2::new(Fixed16_16::from_f32(5000.0), Fixed16_16::from_f32(5000.0));
        assert!(camera.is_on_screen(inside, 0.0));
        assert!(!camera.is_on_screen(outside, 0.0));
        let edge = Vec2::new(Fixed16_16::from_f32(400.0), Fixed16_16::ZERO);
        assert!(camera.is_on_screen(edge, 1.0));
        assert!(!camera.is_on_screen(edge, -1.0));
    }
}
