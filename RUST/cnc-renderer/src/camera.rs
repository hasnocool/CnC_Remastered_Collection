//! Camera with View/Projection matrices
use cnc_core::{Vec2, Fixed16_16, Mat3x3};
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec2,
    pub zoom: Fixed16_16,
    pub rotation: Fixed16_16,
    pub viewport_width: u32,
    pub viewport_height: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::zero(),
            zoom: Fixed16_16::ONE,
            rotation: Fixed16_16::ZERO,
            viewport_width: 800,
            viewport_height: 600,
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
        m[0][0] = proj.m11.to_f32(); m[0][1] = proj.m12.to_f32(); m[0][2] = 0.0; m[0][3] = proj.m13.to_f32();
        m[1][0] = proj.m21.to_f32(); m[1][1] = proj.m22.to_f32(); m[1][2] = 0.0; m[1][3] = proj.m23.to_f32();
        m[2][0] = 0.0; m[2][1] = 0.0; m[2][2] = 1.0; m[2][3] = 0.0;
        m[3][0] = view.m13.to_f32(); m[3][1] = view.m23.to_f32(); m[3][2] = 0.0; m[3][3] = 1.0;
        
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
            Fixed16_16::from_f32((screen.x.to_f32() - self.viewport_width as f32 / 2.0) / zoom_f + self.position.x.to_f32()),
            Fixed16_16::from_f32((screen.y.to_f32() - self.viewport_height as f32 / 2.0) / zoom_f + self.position.y.to_f32()),
        )
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.viewport_width = width;
        self.viewport_height = height;
    }

    pub fn zoom_in(&mut self, factor: f32) {
        self.zoom = self.zoom * Fixed16_16::from_f32(factor);
    }

    pub fn zoom_out(&mut self, factor: f32) {
        self.zoom = self.zoom / Fixed16_16::from_f32(factor);
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.position = self.position + delta;
    }
}
