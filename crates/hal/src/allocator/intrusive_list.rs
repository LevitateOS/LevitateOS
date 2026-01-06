// TEAM_135: Shared intrusive linked list implementation
// Generalized from slab/list.rs for use by both buddy and slab allocators.
// This encapsulates unsafe pointer operations in a safe abstraction.
// TEAM_158: Added behavior ID traceability [SL1]-[SL8]

use core::ptr::NonNull;

/// Intrusive list node trait.
/// Types stored in IntrusiveList must implement this to provide list pointers.
pub trait ListNode: Sized {
    fn next(&self) -> Option<NonNull<Self>>;
    fn prev(&self) -> Option<NonNull<Self>>;
    fn set_next(&mut self, next: Option<NonNull<Self>>);
    fn set_prev(&mut self, prev: Option<NonNull<Self>>);
}

/// Intrusive doubly-linked list for kernel data structures.
/// Behaviors: [SL1] empty on new, [SL7] is_empty, [SL8] integrity
///
/// # Invariants
/// - All nodes maintain valid prev/next pointers
/// - head.prev is None
/// - [SL1] Empty list has head = None and count = 0
///
/// # Safety
/// This structure encapsulates all unsafe pointer operations internally,
/// providing a safe interface to consumers.
pub struct IntrusiveList<T: ListNode> {
    head: Option<NonNull<T>>,
    count: usize,
}

impl<T: ListNode> IntrusiveList<T> {
    /// [SL1] Create an empty list. Const-compatible for static initialization.
    pub const fn new() -> Self {
        Self {
            head: None, // [SL1] new list is empty
            count: 0,
        }
    }

    /// [SL2] Insert node at the front of the list. O(1).
    /// [SL4] Updates head pointer.
    ///
    /// # Safety Contract
    /// - `node` must be a valid mutable reference
    /// - `node` must not already be in this or another list
    pub fn push_front(&mut self, node: &mut T) {
        let new_node = NonNull::from(&mut *node);

        // Update new node's pointers
        node.set_next(self.head);
        node.set_prev(None);

        // Update old head's prev pointer
        if let Some(mut old_head) = self.head {
            // SAFETY: old_head is from self.head which only contains valid pointers
            // to nodes that were previously inserted via push_front.
            unsafe {
                old_head.as_mut().set_prev(Some(new_node));
            }
        }

        // [SL4] Update list head
        self.head = Some(new_node);
        self.count += 1; // [SL2] node added
    }

    /// [SL5] Remove a specific node from the list. O(1).
    /// [SL6] Updates prev/next pointers.
    ///
    /// # Safety Contract
    /// - `node` must be in this list
    pub fn remove(&mut self, node: &mut T) {
        let prev = node.prev();
        let next = node.next();

        // [SL6] Update previous node's next pointer (or head)
        match prev {
            Some(mut prev_node) => {
                // SAFETY: prev_node is valid and within this list (came from node.prev())
                unsafe {
                    prev_node.as_mut().set_next(next); // [SL6]
                }
            }
            None => {
                // Removing head
                self.head = next;
            }
        }

        // [SL6] Update next node's prev pointer
        if let Some(mut next_node) = next {
            // SAFETY: next_node is valid and within this list (came from node.next())
            unsafe {
                next_node.as_mut().set_prev(prev); // [SL6]
            }
        }

        // Clear removed node's pointers
        node.set_next(None);
        node.set_prev(None);

        self.count -= 1;
    }

    /// [SL3] Remove and return the node at the front of the list. O(1).
    pub fn pop_front(&mut self) -> Option<NonNull<T>> {
        let head = self.head?;

        // SAFETY: head is a valid NonNull from self.head.
        // The pointer is non-null by NonNull invariant.
        unsafe {
            let head_ref = head.as_ptr().as_mut().expect("TEAM_135: NonNull was null - impossible");
            self.remove(head_ref);
        }

        Some(head)
    }

    /// [SL7] Check if the list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.is_none() // [SL7] returns true for empty list
    }

    /// Get the number of nodes in the list.
    #[allow(dead_code)]
    #[inline]
    pub fn len(&self) -> usize {
        self.count
    }

    /// Get head of the list without removing it.
    #[inline]
    pub fn head(&self) -> Option<NonNull<T>> {
        self.head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test node implementation
    struct TestNode {
        value: u32,
        next: Option<NonNull<TestNode>>,
        prev: Option<NonNull<TestNode>>,
    }

    impl TestNode {
        fn new(value: u32) -> Self {
            Self {
                value,
                next: None,
                prev: None,
            }
        }
    }

    impl ListNode for TestNode {
        fn next(&self) -> Option<NonNull<Self>> {
            self.next
        }
        fn prev(&self) -> Option<NonNull<Self>> {
            self.prev
        }
        fn set_next(&mut self, next: Option<NonNull<Self>>) {
            self.next = next;
        }
        fn set_prev(&mut self, prev: Option<NonNull<Self>>) {
            self.prev = prev;
        }
    }

    /// Tests: [SL1] new list is empty, [SL7] is_empty returns true
    #[test]
    fn test_new_list_is_empty() {
        let list: IntrusiveList<TestNode> = IntrusiveList::new();
        assert!(list.is_empty()); // [SL1][SL7]
        assert_eq!(list.len(), 0);
    }

    /// Tests: [SL2] push_front adds to front, [SL4] updates head pointer
    #[test]
    fn test_push_front_adds_to_head() {
        let mut list = IntrusiveList::new();
        let mut node1 = TestNode::new(1);
        let mut node2 = TestNode::new(2);

        list.push_front(&mut node1); // [SL2]
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());

        list.push_front(&mut node2); // [SL2]
        assert_eq!(list.len(), 2);

        // [SL4] Head should be node2
        let head = list.head.unwrap();
        unsafe {
            assert_eq!(head.as_ref().value, 2); // [SL4] head updated
        }
    }

    /// Tests: [SL3] pop_front removes from front
    #[test]
    fn test_pop_front_returns_head() {
        let mut list = IntrusiveList::new();
        let mut node1 = TestNode::new(1);
        let mut node2 = TestNode::new(2);

        list.push_front(&mut node1);
        list.push_front(&mut node2);

        let popped = list.pop_front().unwrap(); // [SL3]
        unsafe {
            assert_eq!(popped.as_ref().value, 2); // [SL3] removes from front
        }
        assert_eq!(list.len(), 1);

        let popped = list.pop_front().unwrap(); // [SL3]
        unsafe {
            assert_eq!(popped.as_ref().value, 1);
        }
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    /// Tests: [SL5] remove unlinks from middle, [SL6] updates prev/next
    #[test]
    fn test_remove_from_middle() {
        let mut list = IntrusiveList::new();
        let mut node1 = TestNode::new(1);
        let mut node2 = TestNode::new(2);
        let mut node3 = TestNode::new(3);

        list.push_front(&mut node3);
        list.push_front(&mut node2);
        list.push_front(&mut node1);

        // List: 1 -> 2 -> 3
        list.remove(&mut node2); // [SL5] unlinks from middle
        assert_eq!(list.len(), 2);

        // [SL6] Verify 1 -> 3 (prev/next updated)
        let head = list.head.unwrap();
        unsafe {
            assert_eq!(head.as_ref().value, 1);
            let next = head.as_ref().next.unwrap();
            assert_eq!(next.as_ref().value, 3); // [SL6] pointers updated
        }
    }

    #[test]
    fn test_remove_head() {
        let mut list = IntrusiveList::new();
        let mut node1 = TestNode::new(1);
        let mut node2 = TestNode::new(2);

        list.push_front(&mut node2);
        list.push_front(&mut node1);

        list.remove(&mut node1);
        assert_eq!(list.len(), 1);

        let head = list.head.unwrap();
        unsafe {
            assert_eq!(head.as_ref().value, 2);
        }
    }

    #[test]
    fn test_empty_list_pop() {
        let mut list: IntrusiveList<TestNode> = IntrusiveList::new();
        assert!(list.pop_front().is_none());
    }

    /// Tests: [SL8] Multiple operations maintain list integrity
    #[test]
    fn test_multiple_operations_integrity() {
        let mut list = IntrusiveList::new();
        let mut nodes: [TestNode; 5] = core::array::from_fn(|i| TestNode::new(i as u32));

        // [SL8] Add all nodes
        for node in &mut nodes {
            list.push_front(node);
        }
        assert_eq!(list.len(), 5);

        // [SL8] Remove some, add back, verify integrity
        list.pop_front();
        list.pop_front();
        assert_eq!(list.len(), 3);

        // [SL8] List still works correctly
        let head = list.head.unwrap();
        unsafe {
            assert_eq!(head.as_ref().value, 2); // [SL8] correct ordering
        }
    }
}
