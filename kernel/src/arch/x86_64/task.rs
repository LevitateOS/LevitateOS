//! TEAM_162: x86_64 Context Stub

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Context {
    // x86_64 context stub
    pub sp: u64,
}

impl Context {
    pub fn new(stack_top: usize, _entry_wrapper: usize) -> Self {
        Self {
            sp: stack_top as u64,
        }
    }
}

pub unsafe fn enter_user_mode(_entry_point: usize, _user_sp: usize) -> ! {
    unimplemented!("x86_64 enter_user_mode")
}

// Stubs for asm globals
unsafe extern "C" {
    pub fn cpu_switch_to(old: *mut Context, new: *const Context);
    pub fn task_entry_trampoline();
}

// Global asm stub if needed, but for now we rely on panic
#[unsafe(no_mangle)]
pub unsafe extern "C" fn x86_cpu_switch_to_stub() {
    unimplemented!("cpu_switch_to");
}
