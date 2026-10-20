//! CPU presentation for the map editor's software renderer.
//!
//! `cnc-renderer`'s `SoftwareRenderer` draws into an offscreen `u32` buffer.
//! This module blits that buffer to a winit window through `softbuffer` so the
//! editor remains interactive on machines without a usable wgpu adapter (or
//! when `--renderer software` is requested explicitly).

use std::num::NonZeroU32;
use std::sync::Arc;

use winit::window::Window;

/// Convert an ARGB pixel from `SoftwareRenderer` to softbuffer's 0x00RRGGBB.
pub fn mask_pixel(argb: u32) -> u32 {
    argb & 0x00FF_FFFF
}

/// A winit window-backed software frame surface.
pub struct SoftPresenter {
    context: softbuffer::Context<Arc<Window>>,
    surface: softbuffer::Surface<Arc<Window>, Arc<Window>>,
    width: u32,
    height: u32,
}

impl SoftPresenter {
    /// Attach to an already-created window.
    pub fn new(window: Arc<Window>) -> Result<Self, softbuffer::SoftBufferError> {
        let context = softbuffer::Context::new(window.clone())?;
        let surface = softbuffer::Surface::new(&context, window)?;
        Ok(Self {
            context,
            surface,
            width: 0,
            height: 0,
        })
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Resize the software surface; the next `present` uploads a full frame.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), softbuffer::SoftBufferError> {
        let width = width.max(1);
        let height = height.max(1);
        if (width, height) == (self.width, self.height) {
            return Ok(());
        }
        self.surface.resize(
            NonZeroU32::new(width).expect("width is non-zero"),
            NonZeroU32::new(height).expect("height is non-zero"),
        )?;
        self.width = width;
        self.height = height;
        Ok(())
    }

    /// Copy an ARGB `u32` frame (as produced by `SoftwareRenderer::buffer`)
    /// into the window surface and present it.
    pub fn present(
        &mut self,
        buffer: &[u32],
        width: u32,
        height: u32,
    ) -> Result<(), softbuffer::SoftBufferError> {
        self.resize(width, height)?;
        let expected = (self.width as usize) * (self.height as usize);
        if buffer.len() < expected {
            return Err(softbuffer::SoftBufferError::PlatformError(
                Some("software frame buffer is smaller than the surface".to_string()),
                Some(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "software frame buffer is smaller than the surface",
                ))),
            ));
        }
        let mut frame = self.surface.buffer_mut()?;
        // softbuffer expects 0x00RRGGBB; the software renderer stores alpha in
        // the high byte, so mask it off.
        for (destination, source) in frame.iter_mut().zip(buffer.iter()) {
            *destination = mask_pixel(*source);
        }
        frame.present()?;
        let _ = &self.context;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn softbuffer_pixels_drop_the_alpha_byte() {
        assert_eq!(mask_pixel(0xFF11_2233), 0x0011_2233);
        assert_eq!(mask_pixel(0x0011_2233), 0x0011_2233);
        // Converted to the byte order softbuffer expects on little endian.
        let pixel = mask_pixel(0xFF96_4C33);
        let bytes = pixel.to_le_bytes();
        assert_eq!(bytes[0], 0x33);
        assert_eq!(bytes[1], 0x4C);
        assert_eq!(bytes[2], 0x96);
        assert_eq!(bytes[3], 0x00);
    }
}
