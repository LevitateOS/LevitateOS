pub mod buddy;
pub mod page;
pub mod slab; // TEAM_051: Slab allocator module

pub use buddy::BuddyAllocator;
pub use page::Page;
pub use slab::SLAB_ALLOCATOR; // TEAM_051: Export global slab allocator
