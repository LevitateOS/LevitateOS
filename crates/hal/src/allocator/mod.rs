pub mod buddy;
// TEAM_135: Shared intrusive list for buddy and slab allocators
pub mod intrusive_list;
pub mod page;
pub mod slab; // TEAM_051: Slab allocator module

pub use buddy::BuddyAllocator;
pub use intrusive_list::{IntrusiveList, ListNode};
pub use page::Page;
pub use slab::SLAB_ALLOCATOR; // TEAM_051: Export global slab allocator
