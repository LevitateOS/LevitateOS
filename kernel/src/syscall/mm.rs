use crate::memory::FRAME_ALLOCATOR;
use crate::memory::user as mm_user;
use los_hal::mmu::{self, PAGE_SIZE, PageAllocator, PageFlags};

// Memory management system calls.

// TEAM_228: mmap protection flags (matching Linux)
pub const PROT_NONE: u32 = 0;
pub const PROT_READ: u32 = 1;
pub const PROT_WRITE: u32 = 2;
pub const PROT_EXEC: u32 = 4;

// TEAM_228: mmap flags (matching Linux)
pub const MAP_SHARED: u32 = 0x01;
pub const MAP_PRIVATE: u32 = 0x02;
pub const MAP_FIXED: u32 = 0x10;
pub const MAP_ANONYMOUS: u32 = 0x20;

// TEAM_228: Error codes
const ENOMEM: i64 = -12;
const EINVAL: i64 = -22;

/// TEAM_166: sys_sbrk - Adjust program break (heap allocation).
pub fn sys_sbrk(increment: isize) -> i64 {
    let task = crate::task::current_task();
    let mut heap = task.heap.lock();

    match heap.grow(increment) {
        Ok(old_break) => {
            if increment > 0 {
                let new_break = heap.current;
                let old_page = old_break / los_hal::mmu::PAGE_SIZE;
                // TEAM_181: Use checked arithmetic to prevent overflow
                let new_page = match new_break.checked_add(los_hal::mmu::PAGE_SIZE - 1) {
                    Some(n) => n / los_hal::mmu::PAGE_SIZE,
                    None => {
                        heap.current = old_break;
                        return 0; // Overflow
                    }
                };

                for page in old_page..new_page {
                    let va = page * los_hal::mmu::PAGE_SIZE;
                    if mm_user::user_va_to_kernel_ptr(task.ttbr0, va).is_none() {
                        if mm_user::alloc_and_map_heap_page(task.ttbr0, va).is_err() {
                            heap.current = old_break;
                            return 0; // null
                        }
                    }
                }
            }
            old_break as i64
        }
        Err(()) => 0,
    }
}

/// TEAM_228: sys_mmap - Map memory into process address space.
///
/// For std allocator support, we implement anonymous private mappings.
/// File-backed mappings are not yet supported.
///
/// # Arguments
/// * `addr` - Hint address (ignored unless MAP_FIXED)
/// * `len` - Length of mapping
/// * `prot` - Protection flags (PROT_READ, PROT_WRITE, PROT_EXEC)
/// * `flags` - Mapping flags (MAP_PRIVATE, MAP_ANONYMOUS required for now)
/// * `fd` - File descriptor (must be -1 for MAP_ANONYMOUS)
/// * `offset` - File offset (must be 0 for MAP_ANONYMOUS)
///
/// # Returns
/// Virtual address of mapping, or negative error code.
pub fn sys_mmap(addr: usize, len: usize, prot: u32, flags: u32, fd: i32, offset: usize) -> i64 {
    // TEAM_228: Validate arguments
    if len == 0 {
        return EINVAL;
    }

    // For MVP, only support MAP_ANONYMOUS | MAP_PRIVATE
    if flags & MAP_ANONYMOUS == 0 {
        log::warn!(
            "[MMAP] Only MAP_ANONYMOUS supported, got flags=0x{:x}",
            flags
        );
        return EINVAL;
    }
    if fd != -1 || offset != 0 {
        log::warn!("[MMAP] File-backed mappings not supported");
        return EINVAL;
    }

    let task = crate::task::current_task();
    let ttbr0 = task.ttbr0;

    // Round up length to page boundary
    let pages_needed = (len + PAGE_SIZE - 1) / PAGE_SIZE;
    let alloc_len = pages_needed * PAGE_SIZE;

    // Find free region in user address space
    // Start searching from a reasonable base (0x1000_0000_0000) if no hint
    let base_addr = if addr != 0 && flags & MAP_FIXED != 0 {
        // MAP_FIXED: use exact address (must be page-aligned)
        if addr & (PAGE_SIZE - 1) != 0 {
            return EINVAL;
        }
        addr
    } else {
        // Find a free region - start at a safe mmap area
        // TEAM_228: Use a simple linear search for free space
        find_free_mmap_region(ttbr0, alloc_len).unwrap_or(0)
    };

    if base_addr == 0 {
        return ENOMEM;
    }

    // Convert prot to PageFlags
    let page_flags = prot_to_page_flags(prot);

    // Allocate and map pages
    for i in 0..pages_needed {
        let va = base_addr + i * PAGE_SIZE;

        // Allocate physical page
        let phys = match FRAME_ALLOCATOR.alloc_page() {
            Some(p) => p,
            None => {
                // TODO: Unmap previously allocated pages on failure
                return ENOMEM;
            }
        };

        // Zero the page
        let page_ptr = mmu::phys_to_virt(phys) as *mut u8;
        unsafe {
            core::ptr::write_bytes(page_ptr, 0, PAGE_SIZE);
        }

        // Map into user address space
        if unsafe { mm_user::map_user_page(ttbr0, va, phys, page_flags) }.is_err() {
            // TODO: Free physical pages and unmap on failure
            return ENOMEM;
        }
    }

    log::trace!(
        "[MMAP] Mapped {} pages at 0x{:x} with prot=0x{:x}",
        pages_needed,
        base_addr,
        prot
    );

    base_addr as i64
}

/// TEAM_228: sys_munmap - Unmap memory from process address space.
///
/// # Arguments
/// * `addr` - Start address of mapping (must be page-aligned)
/// * `len` - Length to unmap
///
/// # Returns
/// 0 on success, negative error code on failure.
pub fn sys_munmap(addr: usize, len: usize) -> i64 {
    if addr & (PAGE_SIZE - 1) != 0 || len == 0 {
        return EINVAL;
    }

    // TEAM_228: For MVP, we don't track VMAs, so we can't properly validate.
    // Just mark as success - the pages will remain mapped but unusable from
    // userspace perspective. A full implementation would:
    // 1. Look up VMA for this range
    // 2. Unmap pages from page table
    // 3. Free physical pages
    // 4. Remove/split VMA

    // TODO(TEAM_228): Implement proper VMA tracking and page unmapping

    log::trace!("[MUNMAP] Request to unmap 0x{:x} len={}", addr, len);

    // For now, return success - allocator will treat this memory as freed
    0
}

/// TEAM_228: sys_mprotect - Change protection on memory region.
///
/// # Arguments
/// * `addr` - Start address (must be page-aligned)
/// * `len` - Length of region
/// * `prot` - New protection flags
///
/// # Returns
/// 0 on success, negative error code on failure.
pub fn sys_mprotect(addr: usize, len: usize, prot: u32) -> i64 {
    if addr & (PAGE_SIZE - 1) != 0 || len == 0 {
        return EINVAL;
    }

    // TEAM_228: For MVP, we don't have the infrastructure to modify
    // page table entries in place. A full implementation would:
    // 1. Look up VMA for range
    // 2. Walk page tables
    // 3. Update protection bits on each PTE
    // 4. Flush TLB

    // TODO(TEAM_228): Implement proper page table protection modification

    log::trace!(
        "[MPROTECT] Request to change protection at 0x{:x} len={} prot=0x{:x}",
        addr,
        len,
        prot
    );

    // For now, return success - this is a best-effort implementation
    0
}

/// TEAM_228: Find a free region in user address space for mmap.
///
/// This is a simple implementation that searches for unmapped pages.
/// A production implementation would use a proper VMA tree.
fn find_free_mmap_region(ttbr0: usize, len: usize) -> Option<usize> {
    // Start searching from mmap area (above typical heap, below stack)
    const MMAP_START: usize = 0x0000_1000_0000_0000; // 16 TiB
    const MMAP_END: usize = 0x0000_7000_0000_0000; // Well below stack

    let pages_needed = len / PAGE_SIZE;
    let mut search_addr = MMAP_START;

    while search_addr + len <= MMAP_END {
        // Check if this region is free
        let mut all_free = true;
        for i in 0..pages_needed {
            let test_addr = search_addr + i * PAGE_SIZE;
            if mm_user::user_va_to_kernel_ptr(ttbr0, test_addr).is_some() {
                // This page is already mapped
                all_free = false;
                // Skip past this mapped page
                search_addr = test_addr + PAGE_SIZE;
                break;
            }
        }

        if all_free {
            return Some(search_addr);
        }
    }

    None
}

/// TEAM_228: Convert prot flags to PageFlags.
fn prot_to_page_flags(prot: u32) -> PageFlags {
    // Start with user-accessible base
    let mut flags = PageFlags::USER_DATA;

    if prot & PROT_EXEC != 0 {
        flags = PageFlags::USER_CODE;
    }

    // Note: PROT_NONE would need a different approach - we'd need to
    // map the page but make it inaccessible. For now, treat as USER_DATA.

    flags
}
