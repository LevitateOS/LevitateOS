use linked_list_allocator::LockedHeap;
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_mmu() {
    // stub
}

pub fn get_dtb_phys() -> Option<usize> {
    // stub
    None
}

pub fn print_boot_regs() {
    // stub
}

pub fn init_heap() {
    // stub
}
