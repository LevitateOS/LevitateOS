//! Kernel-side GPU Interface
//! TEAM_099: Transitioned to levitate-virtio-gpu driver

pub use levitate_virtio_gpu::VirtioGpu;
use levitate_hal::IrqSafeLock;
use levitate_hal::LevitateVirtioHal;

/// New GPU state using the integrated driver
pub type GpuState = VirtioGpu<LevitateVirtioHal>;

pub static GPU: IrqSafeLock<Option<GpuState>> = IrqSafeLock::new(None);

pub fn init(mmio_base: usize) {
    match unsafe { VirtioGpu::<LevitateVirtioHal>::new(mmio_base) } {
        Ok(mut gpu) => {
            if let Err(e) = gpu.init() {
                levitate_hal::serial_println!("[GPU] Init logic failed: {:?}", e);
                return;
            }
            *GPU.lock() = Some(gpu);
        }
        Err(e) => {
            levitate_hal::serial_println!("[GPU] Transport init failed: {:?}", e);
        }
    }
}

pub fn get_resolution() -> Option<(u32, u32)> {
    GPU.lock().as_ref().map(|s| s.resolution())
}
