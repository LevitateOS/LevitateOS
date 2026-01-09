//! GPU Driver for LevitateOS
//!
//! TEAM_114: Wrapper around virtio-drivers VirtIOGpu with embedded-graphics support.
//! TEAM_336: Made generic over transport type to support both PCI (x86_64) and MMIO (AArch64).
//!
//! This crate provides:
//! - VirtIO GPU initialization via any transport (PCI or MMIO)
//! - Framebuffer management
//! - embedded-graphics DrawTarget implementation

#![no_std]
#![allow(clippy::unwrap_used)]

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use los_hal::serial_println;
use virtio_drivers::Hal;
use virtio_drivers::device::gpu::VirtIOGpu;
use virtio_drivers::transport::Transport;

/// GPU error type
#[derive(Debug)]
pub enum GpuError {
    /// Device not found
    NotFound,
    /// VirtIO driver error
    VirtioError,
    /// Framebuffer not available
    NoFramebuffer,
}

/// GPU driver wrapper around virtio-drivers VirtIOGpu
/// TEAM_336: Generic over transport type T to support both PCI and MMIO
pub struct Gpu<H: Hal, T: Transport> {
    inner: VirtIOGpu<H, T>,
    width: u32,
    height: u32,
    fb_ptr: Option<*mut u8>,
    fb_size: usize,
}

// SAFETY: GPU access should be protected by a lock at the kernel level
unsafe impl<H: Hal, T: Transport> Send for Gpu<H, T> {}
unsafe impl<H: Hal, T: Transport> Sync for Gpu<H, T> {}

impl<H: Hal, T: Transport> Gpu<H, T> {
    /// Create a new GPU driver from a transport (PCI or MMIO)
    pub fn new(transport: T) -> Result<Self, GpuError> {
        let mut gpu = VirtIOGpu::new(transport).map_err(|_| GpuError::VirtioError)?;

        let (width, height) = gpu.resolution().map_err(|_| GpuError::VirtioError)?;
        serial_println!("[GPU] Resolution: {}x{}", width, height);

        // Setup framebuffer
        let fb = gpu.setup_framebuffer().map_err(|_| GpuError::VirtioError)?;
        let fb_ptr = fb.as_mut_ptr();
        let fb_size = fb.len();

        // TEAM_116: Clear to black for terminal background
        for i in (0..fb_size).step_by(4) {
            fb[i] = 0x00; // B
            fb[i + 1] = 0x00; // G
            fb[i + 2] = 0x00; // R
            fb[i + 3] = 0xFF; // A
        }

        // Flush to display
        gpu.flush().map_err(|_| GpuError::VirtioError)?;

        Ok(Self {
            inner: gpu,
            width: width as u32,
            height: height as u32,
            fb_ptr: Some(fb_ptr),
            fb_size,
        })
    }

    /// Flush framebuffer to display
    pub fn flush(&mut self) -> Result<(), GpuError> {
        self.inner.flush().map_err(|_| GpuError::VirtioError)
    }

    /// Get display resolution
    pub fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get mutable reference to framebuffer
    pub fn framebuffer(&mut self) -> &mut [u8] {
        if let Some(ptr) = self.fb_ptr {
            // SAFETY: We own this framebuffer memory
            unsafe { core::slice::from_raw_parts_mut(ptr, self.fb_size) }
        } else {
            &mut []
        }
    }
}

/// Display adapter for embedded-graphics
/// TEAM_336: Generic over transport type T
pub struct Display<'a, H: Hal, T: Transport> {
    gpu: &'a mut Gpu<H, T>,
}

impl<'a, H: Hal, T: Transport> Display<'a, H, T> {
    /// Create a new display adapter
    pub fn new(gpu: &'a mut Gpu<H, T>) -> Self {
        Self { gpu }
    }
}

impl<H: Hal, T: Transport> DrawTarget for Display<'_, H, T> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let (width, height) = self.gpu.resolution();
        let fb = self.gpu.framebuffer();

        for Pixel(point, color) in pixels {
            if point.x >= 0 && point.x < width as i32 && point.y >= 0 && point.y < height as i32 {
                let idx = (point.y as usize * width as usize + point.x as usize) * 4;
                if idx + 3 < fb.len() {
                    fb[idx] = color.b();
                    fb[idx + 1] = color.g();
                    fb[idx + 2] = color.r();
                    fb[idx + 3] = 255;
                }
            }
        }
        Ok(())
    }
}

impl<H: Hal, T: Transport> OriginDimensions for Display<'_, H, T> {
    fn size(&self) -> Size {
        let (w, h) = self.gpu.resolution();
        Size::new(w, h)
    }
}
