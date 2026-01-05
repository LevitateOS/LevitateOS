//! VirtIO MMIO Transport and HAL Implementation
//! TEAM_092: Extracted from kernel/src/virtio.rs
//! TEAM_099: Added levitate_virtio::VirtioHal implementation

extern crate alloc;
pub use core::ptr::NonNull;
pub use virtio_drivers::{Hal, PhysAddr};
pub type StaticMmioTransport = virtio_drivers::transport::mmio::MmioTransport<'static>;

/// HAL implementation for virtio-drivers crate (legacy, to be removed)
pub struct VirtioHal;

/// TEAM_099: HAL implementation for levitate-virtio crate (new)
pub struct LevitateVirtioHal;

// TEAM_099: Implementation of dma_alloc using the frame allocator
unsafe impl Hal for VirtioHal {
    fn dma_alloc(
        pages: usize,
        _direction: virtio_drivers::BufferDirection,
    ) -> (PhysAddr, NonNull<u8>) {
        let layout = core::alloc::Layout::from_size_align(pages * 4096, 4096).unwrap();
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            panic!("VirtIO DMA allocation failed");
        }
        let vaddr = ptr as usize;
        let paddr = crate::mmu::virt_to_phys(vaddr);
        (paddr as u64, NonNull::new(ptr).unwrap())
    }

    unsafe fn dma_dealloc(paddr: PhysAddr, _vaddr: NonNull<u8>, pages: usize) -> i32 {
        let layout = core::alloc::Layout::from_size_align(pages * 4096, 4096).unwrap();
        let vaddr = crate::mmu::phys_to_virt(paddr as usize);
        unsafe { alloc::alloc::dealloc(vaddr as *mut u8, layout) };
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: PhysAddr, _size: usize) -> NonNull<u8> {
        let vaddr = crate::mmu::phys_to_virt(paddr as usize);
        NonNull::new(vaddr as *mut u8).unwrap()
    }

    unsafe fn share(
        buffer: NonNull<[u8]>,
        _direction: virtio_drivers::BufferDirection,
    ) -> PhysAddr {
        let vaddr = buffer.as_ptr() as *mut u8 as usize;
        crate::mmu::virt_to_phys(vaddr) as u64
    }

    unsafe fn unshare(
        _paddr: PhysAddr,
        _buffer: NonNull<[u8]>,
        _direction: virtio_drivers::BufferDirection,
    ) {
    }
}

// TEAM_099: Implement levitate_virtio::VirtioHal for LevitateVirtioHal
pub use levitate_virtio::{BufferDirection, VirtioHal as LevitateVirtioHalTrait};

unsafe impl LevitateVirtioHalTrait for LevitateVirtioHal {
    fn dma_alloc(
        pages: usize,
        _direction: BufferDirection,
    ) -> (u64, NonNull<u8>) {
        let layout = core::alloc::Layout::from_size_align(pages * 4096, 4096).unwrap();
        let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
        if ptr.is_null() {
            panic!("VirtIO DMA allocation failed");
        }
        let vaddr = ptr as usize;
        let paddr = crate::mmu::virt_to_phys(vaddr);
        (paddr as u64, NonNull::new(ptr).unwrap())
    }

    unsafe fn dma_dealloc(paddr: u64, _vaddr: NonNull<u8>, pages: usize) {
        let layout = core::alloc::Layout::from_size_align(pages * 4096, 4096).unwrap();
        let vaddr = crate::mmu::phys_to_virt(paddr as usize);
        unsafe { alloc::alloc::dealloc(vaddr as *mut u8, layout) };
    }

    unsafe fn mmio_phys_to_virt(paddr: u64, _size: usize) -> NonNull<u8> {
        let vaddr = crate::mmu::phys_to_virt(paddr as usize);
        NonNull::new(vaddr as *mut u8).unwrap()
    }

    unsafe fn share(
        buffer: NonNull<[u8]>,
        _direction: BufferDirection,
    ) -> u64 {
        let vaddr = buffer.as_ptr() as *mut u8 as usize;
        crate::mmu::virt_to_phys(vaddr) as u64
    }

    unsafe fn unshare(
        _paddr: u64,
        _buffer: NonNull<[u8]>,
        _direction: BufferDirection,
    ) {
        // No-op for identity-mapped memory
    }

    fn virt_to_phys(vaddr: usize) -> usize {
        crate::mmu::virt_to_phys(vaddr)
    }

    fn phys_to_virt(paddr: usize) -> usize {
        crate::mmu::phys_to_virt(paddr)
    }
}
