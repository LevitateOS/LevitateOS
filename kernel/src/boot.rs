//! TEAM_146: Boot and Early Initialization
//!
//! This module contains architecture-specific boot code that rarely changes:
//! - Assembly entry point and page table setup
//! - MMU initialization
//! - Heap initialization
//! - DTB discovery
//!
//! Separated from init.rs (device discovery) for upgradability.

use core::arch::global_asm;

use levitate_hal::mmu;
use levitate_hal::println;
use linked_list_allocator::LockedHeap;

// =============================================================================
// Assembly Boot Code
// =============================================================================

global_asm!(
    r#"
.section ".text.head", "ax"
.global _head
.global _start

_head:
    b       _start
    .long   0
    .quad   0x80000          /* text_offset: kernel expects RAM_BASE + 0x80000 */
    .quad   _kernel_size     /* image_size: calculated by linker script */
    .quad   0x0A             /* flags: LE, 4K pages */
    .quad   0
    .quad   0
    .quad   0
    .ascii  "ARM\x64"
    .long   0

_start:
    msr     daifset, #0xf
    mrs     x1, mpidr_el1
    and     x1, x1, #0xFF
    cbz     x1, primary_cpu

secondary_halt:
    wfe
    b       secondary_halt

primary_cpu:
    /* Save x0-x3 to callee-saved registers x19-x22 */
    mov     x19, x0
    mov     x20, x1
    mov     x21, x2
    mov     x22, x3

    /* Enable FP/SIMD */
    mov     x0, #0x300000
    msr     cpacr_el1, x0
    isb

    /* Zero BSS (using physical addresses during boot) */
    /* Note: In higher half, symbols like __bss_start are high VAs. */
    /* We need to convert them to physical for early boot. */
    ldr     x0, =__bss_start
    ldr     x1, =_kernel_virt_base
    sub     x0, x0, x1          /* x0 = __bss_start_phys */
    ldr     x2, =__bss_end
    sub     x2, x2, x1          /* x2 = __bss_end_phys */
    mov     x3, #0
bss_loop:
    cmp     x0, x2
    b.ge    bss_done
    str     x3, [x0], #8
    b       bss_loop
bss_done:

    /* Save preserved registers to global variable BOOT_REGS */
    ldr     x0, =BOOT_REGS
    ldr     x1, =_kernel_virt_base
    sub     x0, x0, x1          /* x0 = physical address of BOOT_REGS */
    str     x19, [x0]           /* x0 */
    str     x20, [x0, #8]       /* x1 */
    str     x21, [x0, #16]      /* x2 */
    str     x22, [x0, #24]      /* x3 */

    /* Save DTB to BOOT_DTB_ADDR for compatibility */
    ldr     x0, =BOOT_DTB_ADDR
    ldr     x1, =_kernel_virt_base
    sub     x0, x0, x1
    str     x19, [x0]

    /* Setup Early Page Tables */
    /* L0_low[0] -> L1_low (ID map for first 1GB) */
    ldr     x4, =_kernel_virt_base
    
    ldr     x0, =boot_pt_l0_low
    sub     x0, x0, x4          /* x0 = boot_pt_l0_low_phys */
    ldr     x1, =boot_pt_l1_low
    sub     x1, x1, x4          /* x1 = boot_pt_l1_low_phys */
    orr     x1, x1, #0x3        /* Table + Valid */
    str     x1, [x0]

    /* L1_low[0] -> 0x00000000 (1GB Device Block) */
    ldr     x0, =boot_pt_l1_low
    sub     x0, x0, x4
    mov     x1, #0x00000000
    add     x1, x1, #0x405      /* Block + AF + Attr1 (Device) */
    str     x1, [x0]
    
    /* L1_low[1] -> 0x40000000 (1GB Normal Block for RAM) */
    mov     x1, #0x40000000
    add     x1, x1, #0x401      /* Block + AF + Attr0 (Normal) */
    str     x1, [x0, #8]        /* Index 1 = 1GB */

    /* L0_high[256] -> L1_high (Higher-half base 0xFFFF8000...) */
    ldr     x0, =boot_pt_l0_high
    sub     x0, x0, x4
    ldr     x1, =boot_pt_l1_high
    sub     x1, x1, x4
    orr     x1, x1, #0x3
    str     x1, [x0, #256*8]

    /* TEAM_078: L1_high[0] -> 0x00000000 (1GB Device Block for UART, GIC, VirtIO) */
    /* This enables device access via TTBR1 high VA before kmain() */
    ldr     x0, =boot_pt_l1_high
    sub     x0, x0, x4
    mov     x1, #0x00000000
    add     x1, x1, #0x405      /* Block + AF + Attr1 (Device) */
    str     x1, [x0]            /* Index 0 for 0x00000000 */

    /* L1_high[1] -> 0x40000000 (1GB Block matching 0x40000000 physical) */
    mov     x1, #0x40000000
    add     x1, x1, #0x401      /* Block + AF + Attr0 (Normal) */
    str     x1, [x0, #8]        /* Index 1 for 0x40000000 */

    /* Configure MMU Registers */
    /* MAIR_EL1: Attr0=0xFF (Normal), Attr1=0x04 (Device) */
    ldr     x0, =0x00000000000004FF
    msr     mair_el1, x0

    /* TCR_EL1: T0SZ=16, T1SZ=16, TG0=4K, TG1=4K, IPS=48bit, SH0/SH1=Inner, Cacheable */
    ldr     x0, =0x00000005b5103510
    msr     tcr_el1, x0
    isb

    /* Load TTBR0 and TTBR1 */
    ldr     x0, =boot_pt_l0_low
    sub     x0, x0, x4
    msr     ttbr0_el1, x0
    ldr     x0, =boot_pt_l0_high
    sub     x0, x0, x4
    msr     ttbr1_el1, x0
    isb

    /* Enable MMU */
    mrs     x0, sctlr_el1
    orr     x0, x0, #0x1        /* M (MMU) */
    orr     x0, x0, #0x4        /* C (D-Cache) */
    orr     x0, x0, #0x1000     /* I (I-Cache) */
    msr     sctlr_el1, x0
    isb

    /* Jump to High VA kmain */
    ldr     x0, =stack_top
    mov     sp, x0
    ldr     x0, =kmain
    br      x0

.section ".text", "ax"

.section ".data.boot_pt", "aw"
.align 12
boot_pt_l0_low:  .space 4096
boot_pt_l1_low:  .space 4096
boot_pt_l0_high: .space 4096
boot_pt_l1_high: .space 4096

.section ".data"
.global _end
_end:
"#
);

// =============================================================================
// Boot Data
// =============================================================================

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Physical address of the Device Tree Blob (DTB) passed by the bootloader.
/// Saved from x0 in `_start`.
#[unsafe(no_mangle)]
pub static mut BOOT_DTB_ADDR: u64 = 0;

#[unsafe(no_mangle)]
pub static mut BOOT_REGS: [u64; 4] = [0; 4];

// =============================================================================
// Early Initialization
// =============================================================================

/// Initialize the kernel heap from linker-defined symbols.
pub fn init_heap() {
    unsafe extern "C" {
        static __heap_start: u8;
        static __heap_end: u8;
    }

    unsafe {
        let heap_start = &__heap_start as *const u8 as usize;
        let heap_end = &__heap_end as *const u8 as usize;
        let heap_size = heap_end - heap_start;
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size);
    }

    crate::verbose!("Heap initialized.");
}

/// Initialize MMU with fine-grained page tables.
///
/// The assembly boot code sets up coarse 1GB blocks. This function
/// creates more granular mappings for kernel, devices, and RAM.
pub fn init_mmu() {
    // MMU is already enabled by assembly boot, but we re-initialize
    // with more granular mappings if needed.
    mmu::init();

    let root = unsafe {
        static mut ROOT_PT: mmu::PageTable = mmu::PageTable::new();
        &mut *core::ptr::addr_of_mut!(ROOT_PT)
    };

    // Map Kernel - RWX (no PXN) for now to keep it simple.
    // We map it to BOTH identity and higher-half for a smooth transition.
    let kernel_flags = mmu::PageFlags::KERNEL_DATA.difference(mmu::PageFlags::PXN);

    // Critical boot mappings - system cannot continue if these fail
    #[allow(clippy::expect_used, clippy::unwrap_used)]
    {
        // Identity map for early access (until we fully switch to high VA for everything)
        mmu::identity_map_range_optimized(
            root,
            mmu::KERNEL_PHYS_START,
            mmu::KERNEL_PHYS_END,
            kernel_flags,
        )
        .expect("Failed to identity map kernel");

        // Higher-half map kernel
        mmu::map_range(
            root,
            mmu::KERNEL_VIRT_START + mmu::KERNEL_PHYS_START,
            mmu::KERNEL_PHYS_START,
            mmu::KERNEL_PHYS_END - mmu::KERNEL_PHYS_START,
            kernel_flags,
        )
        .expect("Failed to higher-half map kernel");

        // Higher-half map all boot RAM (0x4000_0000 to 0x8000_0000)
        // to support access to DTB, initrd, and mem_map. Use 1GB to cover QEMU default + Pixel 6 base.
        mmu::map_range(
            root,
            mmu::KERNEL_VIRT_START + 0x4000_0000,
            0x4000_0000,
            0x4000_0000, // 1GB (was 256MB)
            kernel_flags,
        )
        .expect("Failed to map boot RAM to higher half");

        // TEAM_078: Map Devices to HIGH VA (via TTBR1) instead of identity mapping
        // This ensures devices remain accessible when TTBR0 is switched for userspace

        // UART (PA: 0x0900_0000 -> VA: KERNEL_VIRT_START + 0x0900_0000)
        mmu::map_range(
            root,
            mmu::UART_VA,
            0x0900_0000,
            0x1000,
            mmu::PageFlags::DEVICE,
        )
        .unwrap();

        // TEAM_042: GIC mapping extended for GICv3 support
        // QEMU virt GIC layout:
        //   GICD: 0x0800_0000 - 0x0801_0000 (64KB)
        //   GICC: 0x0801_0000 - 0x0802_0000 (64KB) - GICv2 only
        //   GICR: 0x080A_0000 - 0x080C_0000 (128KB per CPU, 8 CPUs = 1MB)
        // Map 0x0800_0000 - 0x0820_0000 to cover all GIC components
        mmu::map_range(
            root,
            mmu::GIC_DIST_VA,
            0x0800_0000,
            0x20_0000, // 2MB covers GICD + GICC + GICR
            mmu::PageFlags::DEVICE,
        )
        .unwrap();

        // VirtIO MMIO (PA: 0x0A00_0000 -> VA: KERNEL_VIRT_START + 0x0A00_0000)
        mmu::map_range(
            root,
            mmu::VIRTIO_MMIO_VA,
            0x0a00_0000,
            0x10_0000, // 1MB for VirtIO devices
            mmu::PageFlags::DEVICE,
        )
        .unwrap();

        // TEAM_114: PCI ECAM for VirtIO PCI transport
        // ECAM is at 0x4010_0000_0000 (256MB for 256 buses)
        // Map just enough for bus 0 scanning (1MB)
        mmu::map_range(
            root,
            mmu::ECAM_VA,
            mmu::ECAM_PA,
            0x10_0000, // 1MB for bus 0
            mmu::PageFlags::DEVICE,
        )
        .unwrap();

        // TEAM_114: PCI 32-bit memory region for BAR allocation
        mmu::map_range(
            root,
            mmu::PCI_MEM32_VA,
            mmu::PCI_MEM32_PA,
            mmu::PCI_MEM32_SIZE,
            mmu::PageFlags::DEVICE,
        )
        .unwrap();

        // Map Boot RAM including DTB/initrd region (QEMU places DTB after kernel)
        // DTB is at ~0x4820_0000 for a ~100KB kernel + initrd
        mmu::identity_map_range_optimized(root, 0x4000_0000, 0x5000_0000, kernel_flags)
            .expect("Failed to map boot RAM");
    }

    // Enable MMU with both TTBR0 and TTBR1
    mmu::tlb_flush_all();
    let root_phys = mmu::virt_to_phys(root as *const _ as usize);
    unsafe {
        // During transition, we can use the same root if it has
        // both bottom-half and top-half entries.
        mmu::enable_mmu(root_phys, root_phys);
    }
    crate::verbose!("MMU re-initialized (Higher-Half + Identity).");
}

/// Returns the physical address of the DTB if one was provided.
///
/// # TEAM_038: DTB Detection Strategy
/// 1. Check x0 (passed by bootloader via BOOT_DTB_ADDR)
/// 2. Scan common QEMU DTB locations (0x4000_0000 region)
/// 3. Search for DTB magic (0xD00DFEED) in early RAM
///
/// On real hardware (Pixel 6), step 1 should work. Steps 2-3 are for QEMU ELF boot.
pub fn get_dtb_phys() -> Option<usize> {
    // Step 1: Check if bootloader passed DTB address in x0
    let addr = unsafe { BOOT_DTB_ADDR };
    if addr != 0 {
        crate::verbose!("DTB address from x0: 0x{:x}", addr);
        return Some(addr as usize);
    }

    // Step 2: Scan likely DTB locations in early RAM (fallback for ELF boot)
    // QEMU may place DTB at start of RAM or after kernel
    let scan_start = 0x4000_0000usize;
    let scan_end = 0x4900_0000usize; // Scan first ~144MB of RAM

    // Scan page-aligned addresses (DTB must be 8-byte aligned per spec)
    for addr in (scan_start..scan_end).step_by(0x1000) {
        let magic = unsafe { core::ptr::read_volatile(addr as *const u32) };
        if u32::from_be(magic) == 0xd00d_feed {
            crate::verbose!("Found DTB at 0x{:x}", addr);
            return Some(addr);
        }
    }

    None
}

/// Print boot registers (for debugging).
pub fn print_boot_regs() {
    unsafe {
        println!(
            "BOOT_REGS: x0={:x} x1={:x} x2={:x} x3={:x}",
            BOOT_REGS[0], BOOT_REGS[1], BOOT_REGS[2], BOOT_REGS[3]
        );
    }
}
