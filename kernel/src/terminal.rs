//! Kernel Terminal Integration
//! TEAM_092: Unified integration for dual-console mirroring.
//! This file replaces the previous terminal.rs and console_gpu.rs.

use levitate_hal::IrqSafeLock;
use levitate_terminal::Terminal;

pub static TERMINAL: IrqSafeLock<Option<Terminal>> = IrqSafeLock::new(None);

/// Initialize the global GPU terminal.
/// Mirroring is enabled after this by calling levitate_hal::console::set_secondary_output.
pub fn init() {
    if let Some(mut gpu_guard) = crate::gpu::GPU.try_lock() {
        if let Some(gpu_state) = gpu_guard.as_mut() {
            let (width, height) = gpu_state.resolution();
            let term = Terminal::new(width, height);
            *TERMINAL.lock() = Some(term);
        }
    }
}

/// Mirror console output to the GPU terminal.
/// Called via the secondary output callback in levitate-hal.
pub fn write_str(s: &str) {
    if let Some(mut term_guard) = TERMINAL.try_lock() {
        if let Some(term) = term_guard.as_mut() {
            if let Some(mut gpu_guard) = crate::gpu::GPU.try_lock() {
                if let Some(gpu_state) = gpu_guard.as_mut() {
                    term.write_str(gpu_state, s);
                }
            }
        }
    }
}
