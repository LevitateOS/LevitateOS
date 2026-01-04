// TEAM_070: Scheduler implementation.
use crate::task::{TaskControlBlock, TaskId};
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use levitate_hal::IrqSafeLock;

/// TEAM_070: Global scheduler state.
pub struct Scheduler {
    /// Queue of tasks ready to run.
    /// TEAM_070: Uses IrqSafeLock to prevent deadlocks during IRQ preemption (Rule 7).
    pub ready_list: IrqSafeLock<VecDeque<Arc<TaskControlBlock>>>,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            ready_list: IrqSafeLock::new(VecDeque::new()),
        }
    }

    /// Add a task to the ready list.
    pub fn add_task(&self, task: Arc<TaskControlBlock>) {
        self.ready_list.lock().push_back(task);
    }

    /// Pick the next task to run.
    pub fn pick_next(&self) -> Option<Arc<TaskControlBlock>> {
        self.ready_list.lock().pop_front()
    }

    /// Perform a context switch to the next ready task.
    /// TEAM_070: This is the core of cooperative multitasking.
    pub fn schedule(&self) {
        if let Some(next) = self.pick_next() {
            crate::task::switch_to(next);
        }
    }
}

/// TEAM_070: Global scheduler instance.
pub static SCHEDULER: Scheduler = Scheduler::new();
