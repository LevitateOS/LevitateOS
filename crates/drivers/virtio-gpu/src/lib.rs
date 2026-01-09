//! GPU Driver for LevitateOS
//!
//! TEAM_114: Wrapper around virtio-drivers VirtIOGpu with embedded-graphics support.
//! TEAM_334: Migrated to virtio-gpu crate.
//! TEAM_336: Added unified GPU backend with Limine framebuffer fallback support.
//!
//! This crate provides:
//! - VirtIO GPU driver with embedded-graphics DrawTarget
//! - Limine framebuffer fallback for x86_64
//! - Unified `GpuBackend` enum for transparent backend switching
//!
//! ## Kernel SOP Alignment
//!
//! - **Rule 1 (Modular Scope):** This crate handles all GPU display backends
//! - **Rule 2 (Type-Driven Composition):** `GpuBackend` enum provides unified interface
//! - **Rule 13 (Representation):** Uses enum to encode backend type; match for dispatch

#![no_std]
#![allow(clippy::unwrap_used)]

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use los_hal::serial_println;
use virtio_drivers::device::gpu::VirtIOGpu;
use virtio_drivers::Hal;
use virtio_drivers::transport::Transport;

// ============================================================================
// GPU Error Types
// ============================================================================

/// GPU error type
#[derive(Debug)]
pub enum GpuError {
    /// PCI device not found
    NotFound,
    /// VirtIO driver error
    VirtioError,
    /// Framebuffer not available
    NoFramebuffer,
}

// ============================================================================
// Framebuffer Types (for Limine boot fallback)
// ============================================================================

/// TEAM_336: Pixel format for framebuffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// RGB (red at lowest address)
    Rgb,
    /// BGR (blue at lowest address)
    Bgr,
    /// Unknown format
    Unknown,
}

/// TEAM_336: Framebuffer configuration from bootloader.
/// Used to initialize `FramebufferGpu` when VirtIO GPU is unavailable.
#[derive(Debug, Clone, Copy)]
pub struct FramebufferConfig {
    /// Physical address of framebuffer memory
    pub address: usize,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Bytes per scanline
    pub pitch: u32,
    /// Bits per pixel
    pub bpp: u8,
    /// Pixel format
    pub format: PixelFormat,
}

// ============================================================================
// VirtIO GPU Backend
// ============================================================================

/// GPU driver wrapper around virtio-drivers VirtIOGpu
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
    /// Create a new GPU driver from a transport
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

/// Display adapter for VirtIO GPU (embedded-graphics DrawTarget)
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

// ============================================================================
// Limine Framebuffer Backend
// ============================================================================

/// TEAM_336: Simple framebuffer-based GPU for Limine boot (x86_64 fallback).
pub struct FramebufferGpu {
    address: usize,
    width: u32,
    height: u32,
    pitch: u32,
    format: PixelFormat,
}

// SAFETY: Framebuffer access protected by kernel lock
unsafe impl Send for FramebufferGpu {}
unsafe impl Sync for FramebufferGpu {}

impl FramebufferGpu {
    /// Create a new framebuffer GPU from bootloader config.
    pub fn new(config: &FramebufferConfig) -> Self {
        Self {
            address: config.address,
            width: config.width,
            height: config.height,
            pitch: config.pitch,
            format: config.format,
        }
    }

    /// Get display resolution
    pub fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get mutable reference to framebuffer
    pub fn framebuffer(&mut self) -> &mut [u8] {
        let size = (self.pitch as usize) * (self.height as usize);
        // SAFETY: Limine framebuffer address is valid and mapped by bootloader
        unsafe { core::slice::from_raw_parts_mut(self.address as *mut u8, size) }
    }

    /// Flush framebuffer (no-op for direct-mapped framebuffer)
    pub fn flush(&mut self) -> Result<(), GpuError> {
        // Limine framebuffer is directly mapped - no flush needed
        Ok(())
    }

    /// Get pixel format
    pub fn format(&self) -> PixelFormat {
        self.format
    }

    /// Get pitch (bytes per scanline)
    pub fn pitch(&self) -> u32 {
        self.pitch
    }
}

/// TEAM_336: DrawTarget wrapper for framebuffer GPU.
pub struct FramebufferDisplay<'a> {
    gpu: &'a mut FramebufferGpu,
}

impl<'a> FramebufferDisplay<'a> {
    /// Create a new framebuffer display adapter
    pub fn new(gpu: &'a mut FramebufferGpu) -> Self {
        Self { gpu }
    }
}

impl DrawTarget for FramebufferDisplay<'_> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let width = self.gpu.width;
        let height = self.gpu.height;
        let pitch = self.gpu.pitch as usize;
        let is_bgr = matches!(self.gpu.format, PixelFormat::Bgr);
        let fb = self.gpu.framebuffer();

        for Pixel(point, color) in pixels {
            if point.x >= 0 && point.x < width as i32 && point.y >= 0 && point.y < height as i32 {
                let offset = (point.y as usize) * pitch + (point.x as usize) * 4;
                if offset + 3 < fb.len() {
                    if is_bgr {
                        fb[offset] = color.b();
                        fb[offset + 1] = color.g();
                        fb[offset + 2] = color.r();
                    } else {
                        fb[offset] = color.r();
                        fb[offset + 1] = color.g();
                        fb[offset + 2] = color.b();
                    }
                    fb[offset + 3] = 255; // Alpha
                }
            }
        }
        Ok(())
    }
}

impl OriginDimensions for FramebufferDisplay<'_> {
    fn size(&self) -> Size {
        Size::new(self.gpu.width, self.gpu.height)
    }
}
