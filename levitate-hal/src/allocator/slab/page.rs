// TEAM_051: Slab Allocator - Page Structure
// Layout: [data: 4032 bytes][metadata: 64 bytes]
// See docs/planning/slab-allocator/phase-2.md for design

use super::list::ListNode;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicU64, Ordering};

pub const PAGE_SIZE: usize = 4096;
pub const META_SIZE: usize = 64;
pub const DATA_SIZE: usize = PAGE_SIZE - META_SIZE;

/// Metadata stored at end of each slab page (64 bytes).
///
/// Layout ensures cache-line alignment and efficient bitfield operations.
#[repr(C)]
pub struct SlabPageMeta {
    /// Bitfield tracking allocation status (up to 64 objects).
    /// Bit set (1) = allocated, bit clear (0) = free.
    pub bitfield: AtomicU64, // 8 bytes

    /// Size class index (0-5).
    pub size_class: u8, // 1 byte

    /// Number of allocated objects in this page.
    pub allocated_count: u8, // 1 byte

    /// Padding for alignment.
    _pad: [u8; 6], // 6 bytes

    /// Intrusive list pointers (for SlabList).
    pub next: Option<NonNull<SlabPage>>, // 8 bytes
    pub prev: Option<NonNull<SlabPage>>, // 8 bytes

    /// Physical address of this page (for freeing back to Buddy).
    pub phys_addr: usize, // 8 bytes

    /// Reserved for future use.
    _reserved: [u8; 24], // 24 bytes
}
// Total: 64 bytes

/// A 4KB slab page subdivided into fixed-size objects.
///
/// Memory Layout:
/// ```text
/// ┌────────────────────────────────────────┐ 0x000
/// │           Object Storage               │
/// │         (4032 bytes usable)            │
/// ├────────────────────────────────────────┤ 0xFC0
/// │              Metadata                  │
/// │            (64 bytes)                  │
/// └────────────────────────────────────────┘ 0x1000
/// ```
///
/// Metadata is at the END to preserve object alignment at page start.
#[repr(C)]
pub struct SlabPage {
    /// Object storage area (page_start to page_start + DATA_SIZE).
    data: [u8; DATA_SIZE],

    /// Metadata section (64 bytes, cache-line aligned).
    pub meta: SlabPageMeta,
}

impl SlabPage {
    /// Initialize metadata for a new slab page.
    ///
    /// # Safety
    /// `page_ptr` must point to a valid 4KB-aligned page.
    pub unsafe fn init(page_ptr: *mut u8, size_class: u8, phys_addr: usize) {
        // SAFETY: Caller guarantees page_ptr points to a valid 4KB-aligned page
        unsafe {
            let page = &mut *(page_ptr as *mut SlabPage);

            page.meta = SlabPageMeta {
                bitfield: AtomicU64::new(0),
                size_class,
                allocated_count: 0,
                _pad: [0; 6],
                next: None,
                prev: None,
                phys_addr,
                _reserved: [0; 24],
            };
        }
    }

    /// Allocate one object from this page.
    ///
    /// Returns the byte offset within the page to the allocated object,
    /// or None if the page is full.
    ///
    /// # Behavior [S1]
    /// - Find first free slot (first zero bit in bitfield)
    /// - Set bit to mark as allocated
    /// - Increment allocated_count
    /// - Return offset = bit_index * object_size
    pub fn alloc_object(&mut self, object_size: usize, objects_per_page: usize) -> Option<usize> {
        // Check if we've already allocated the maximum number of objects
        if self.meta.allocated_count as usize >= objects_per_page {
            return None;
        }
        
        let slot = self.find_free_slot()?;

        self.set_allocated(slot);
        self.meta.allocated_count += 1;

        Some(slot * object_size)
    }

    /// Free an object at the given offset.
    ///
    /// # Behavior [S4][S5]
    /// - Compute slot index from offset
    /// - Clear bit in bitfield
    /// - Decrement allocated_count
    pub fn free_object(&mut self, offset: usize, object_size: usize) {
        let slot = offset / object_size;

        self.set_free(slot);
        self.meta.allocated_count = self.meta.allocated_count.saturating_sub(1);
    }

    /// Check if the page is full.
    pub fn is_full(&self, objects_per_page: usize) -> bool {
        self.meta.allocated_count as usize >= objects_per_page
    }

    /// Check if the page is empty (no objects allocated).
    pub fn is_empty(&self) -> bool {
        self.meta.allocated_count == 0
    }

    /// Get the base virtual address of this page.
    pub fn base_addr(&self) -> usize {
        self as *const _ as usize
    }

    // Private helper methods

    /// Find first free slot (first zero bit in bitfield).
    fn find_free_slot(&self) -> Option<usize> {
        let bits = self.meta.bitfield.load(Ordering::Relaxed);
        if bits == u64::MAX {
            return None; // All allocated
        }
        Some(bits.trailing_ones() as usize)
    }

    /// Mark slot as allocated.
    fn set_allocated(&self, index: usize) {
        debug_assert!(index < 64, "Slot index out of bounds");
        self.meta.bitfield.fetch_or(1 << index, Ordering::Relaxed);
    }

    /// Mark slot as free.
    fn set_free(&self, index: usize) {
        debug_assert!(index < 64, "Slot index out of bounds");
        self.meta
            .bitfield
            .fetch_and(!(1 << index), Ordering::Relaxed);
    }
}

impl ListNode for SlabPage {
    fn next(&self) -> Option<NonNull<Self>> {
        self.meta.next
    }

    fn prev(&self) -> Option<NonNull<Self>> {
        self.meta.prev
    }

    fn set_next(&mut self, next: Option<NonNull<Self>>) {
        self.meta.next = next;
    }

    fn set_prev(&mut self, prev: Option<NonNull<Self>>) {
        self.meta.prev = prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_size_constants() {
        assert_eq!(PAGE_SIZE, 4096);
        assert_eq!(META_SIZE, 64);
        assert_eq!(DATA_SIZE, 4032);
        assert_eq!(core::mem::size_of::<SlabPage>(), PAGE_SIZE);
        assert_eq!(core::mem::size_of::<SlabPageMeta>(), META_SIZE);
    }

    #[test]
    fn test_init_sets_metadata() {
        let mut buffer = [0u8; PAGE_SIZE];
        let page_ptr = buffer.as_mut_ptr();

        unsafe {
            SlabPage::init(page_ptr, 2, 0x1000);
            let page = &*(page_ptr as *const SlabPage);

            assert_eq!(page.meta.size_class, 2);
            assert_eq!(page.meta.allocated_count, 0);
            assert_eq!(page.meta.phys_addr, 0x1000);
            assert_eq!(page.meta.bitfield.load(Ordering::Relaxed), 0);
        }
    }

    #[test]
    fn test_alloc_object_returns_sequential_offsets() {
        let mut buffer = [0u8; PAGE_SIZE];
        let page_ptr = buffer.as_mut_ptr();

        unsafe {
            SlabPage::init(page_ptr, 0, 0x1000);
            let page = &mut *(page_ptr as *mut SlabPage);

            let offset1 = page.alloc_object(64, 63).unwrap();
            assert_eq!(offset1, 0);
            assert_eq!(page.meta.allocated_count, 1);

            let offset2 = page.alloc_object(64, 63).unwrap();
            assert_eq!(offset2, 64);
            assert_eq!(page.meta.allocated_count, 2);

            let offset3 = page.alloc_object(64, 63).unwrap();
            assert_eq!(offset3, 128);
            assert_eq!(page.meta.allocated_count, 3);
        }
    }

    #[test]
    fn test_free_object_clears_bit() {
        let mut buffer = [0u8; PAGE_SIZE];
        let page_ptr = buffer.as_mut_ptr();

        unsafe {
            SlabPage::init(page_ptr, 0, 0x1000);
            let page = &mut *(page_ptr as *mut SlabPage);

            let offset = page.alloc_object(64, 63).unwrap();
            assert_eq!(page.meta.allocated_count, 1);

            page.free_object(offset, 64);
            assert_eq!(page.meta.allocated_count, 0);

            // Should be able to allocate same slot again
            let offset2 = page.alloc_object(64, 63).unwrap();
            assert_eq!(offset, offset2);
        }
    }

    #[test]
    fn test_is_full_after_max_allocations() {
        let mut buffer = [0u8; PAGE_SIZE];
        let page_ptr = buffer.as_mut_ptr();

        unsafe {
            SlabPage::init(page_ptr, 0, 0x1000);
            let page = &mut *(page_ptr as *mut SlabPage);

            // Allocate 63 objects (max for 64B class)
            for _ in 0..63 {
                page.alloc_object(64, 63).unwrap();
            }

            assert!(page.is_full(63));
            assert!(page.alloc_object(64, 63).is_none());
        }
    }

    #[test]
    fn test_is_empty_after_freeing_all() {
        let mut buffer = [0u8; PAGE_SIZE];
        let page_ptr = buffer.as_mut_ptr();

        unsafe {
            SlabPage::init(page_ptr, 0, 0x1000);
            let page = &mut *(page_ptr as *mut SlabPage);

            let offset1 = page.alloc_object(64, 63).unwrap();
            let offset2 = page.alloc_object(64, 63).unwrap();

            assert!(!page.is_empty());

            page.free_object(offset1, 64);
            page.free_object(offset2, 64);

            assert!(page.is_empty());
        }
    }
}
