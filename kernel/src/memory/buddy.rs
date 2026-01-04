use crate::memory::page::Page;
use core::ptr::NonNull;

// TEAM_047: Buddy Allocator implementation
// Handles physical frame allocation and freeing with coalescing.

pub const MAX_ORDER: usize = 21; // Up to 8GB (2^21 * 4KB)
pub const PAGE_SIZE: usize = 4096;

pub struct BuddyAllocator {
    /// Free lists for each order.
    /// free_lists[i] stores head of a doubly-linked list of free blocks of order i.
    free_lists: [Option<NonNull<Page>>; MAX_ORDER],

    /// Pointer to the global memory map (array of Page structs).
    mem_map: Option<&'static mut [Page]>,

    /// Physical address corresponding to the first entry in mem_map.
    phys_base: usize,
}

// SAFETY: BuddyAllocator is managed via Spinlock in the FrameAllocator global
unsafe impl Send for BuddyAllocator {}
unsafe impl Sync for BuddyAllocator {}

impl BuddyAllocator {
    /// Create a new, uninitialized Buddy Allocator.
    pub const fn new() -> Self {
        Self {
            free_lists: [None; MAX_ORDER],
            mem_map: None,
            phys_base: 0,
        }
    }

    /// Initialize the allocator with a memory map and physical base address.
    pub unsafe fn init(&mut self, mem_map: &'static mut [Page], phys_base: usize) {
        self.mem_map = Some(mem_map);
        self.phys_base = phys_base;
    }

    /// Add a range of physical memory to the allocator.
    ///
    /// The range must be page-aligned.
    pub unsafe fn add_range(&mut self, start_pa: usize, end_pa: usize) {
        let mut curr_pa = start_pa;
        while curr_pa < end_pa {
            // Find the largest order that fits and is aligned
            let mut order = MAX_ORDER - 1;
            while order > 0 {
                let size = (1 << order) * PAGE_SIZE;
                if curr_pa + size <= end_pa && (curr_pa % size) == 0 {
                    break;
                }
                order -= 1;
            }

            self.free_block(curr_pa, order);
            curr_pa += (1 << order) * PAGE_SIZE;
        }
    }

    /// Allocate a block of memory of the given order.
    pub fn alloc(&mut self, order: usize) -> Option<usize> {
        if order >= MAX_ORDER {
            return None;
        }

        // 1. Find the smallest free block of order >= requested
        for i in order..MAX_ORDER {
            if let Some(mut page_ptr) = self.free_lists[i] {
                // Found a block! Remove it from the list.
                let page = unsafe { page_ptr.as_mut() };
                self.remove_from_list(i, page);

                // 2. Split the block if it's larger than needed
                for j in (order..i).rev() {
                    let buddy_pa = self.page_to_pa(page) + (1 << j) * PAGE_SIZE;
                    let buddy_page = self
                        .pa_to_page_mut(buddy_pa)
                        .expect("Buddy page must exist");

                    buddy_page.reset();
                    buddy_page.order = j as u8;
                    buddy_page.mark_free();
                    self.add_to_list(j, buddy_page);
                }

                page.mark_allocated();
                page.order = order as u8;
                return Some(self.page_to_pa(page));
            }
        }

        None
    }

    /// Free a block of memory.
    pub fn free(&mut self, pa: usize, order: usize) {
        self.free_block(pa, order);
    }

    fn free_block(&mut self, pa: usize, order: usize) {
        let mut curr_pa = pa;
        let mut curr_order = order;

        // Coalesce with buddy if possible
        while curr_order < MAX_ORDER - 1 {
            let buddy_pa = curr_pa ^ ((1 << curr_order) * PAGE_SIZE);

            if let Some(buddy_page) = self.pa_to_page_mut(buddy_pa) {
                // Buddy must be free and have the same order
                if buddy_page.is_free() && buddy_page.order as usize == curr_order {
                    // Pull buddy out of its list
                    self.remove_from_list(curr_order, buddy_page);

                    // Coalesce
                    if buddy_pa < curr_pa {
                        curr_pa = buddy_pa;
                    }
                    curr_order += 1;
                    continue;
                }
            }
            break;
        }

        // Add the (possibly coalesced) block to the free list
        let page = self.pa_to_page_mut(curr_pa).expect("Page must exist");
        page.reset();
        page.mark_free();
        page.order = curr_order as u8;
        self.add_to_list(curr_order, page);
    }

    // Helper: Convert Physical Address to Page descriptor
    pub(crate) fn pa_to_page_mut(&mut self, pa: usize) -> Option<&'static mut Page> {
        let mem_map = self.mem_map.as_mut()?;
        let index = (pa - self.phys_base) / PAGE_SIZE;
        if index < mem_map.len() {
            unsafe {
                let ptr = mem_map.as_mut_ptr();
                Some(&mut *ptr.add(index))
            }
        } else {
            None
        }
    }

    // Helper: Convert Page descriptor to Physical Address
    fn page_to_pa(&self, page: &Page) -> usize {
        let mem_map = self.mem_map.as_ref().expect("mem_map must be set");
        let offset = page as *const Page as usize - mem_map.as_ptr() as usize;
        let index = offset / core::mem::size_of::<Page>();
        self.phys_base + index * PAGE_SIZE
    }

    fn add_to_list(&mut self, order: usize, page: &'static mut Page) {
        page.next = self.free_lists[order];
        page.prev = None;
        if let Some(mut next_ptr) = self.free_lists[order] {
            unsafe { next_ptr.as_mut().prev = Some(NonNull::from(&mut *page)) };
        }
        self.free_lists[order] = Some(NonNull::from(&mut *page));
    }

    fn remove_from_list(&mut self, order: usize, page: &mut Page) {
        if let Some(mut prev_ptr) = page.prev {
            unsafe { prev_ptr.as_mut().next = page.next };
        } else {
            self.free_lists[order] = page.next;
        }

        if let Some(mut next_ptr) = page.next {
            unsafe { next_ptr.as_mut().prev = page.prev };
        }

        page.next = None;
        page.prev = None;
    }
}
