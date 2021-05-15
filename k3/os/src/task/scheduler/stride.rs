use super::Scheduler;
use super::super::TASK_MANAGER;
use super::super::TaskStatus;
use crate::{config::{MAX_APP_NUM}, task::task::TaskControlBlock};

use heapless::binary_heap::{BinaryHeap, Max};

// 关于pass的值，等于BIG_STRIDE/优先级
pub struct Stride{
    heap: BinaryHeap<i32, Max, 8>,
}

impl Scheduler for Stride {
    fn new(tasks: [TaskControlBlock; MAX_APP_NUM]) -> Self{Self{
        heap: BinaryHeap::new()
    }}
    fn find_next_task(&self) -> Option<usize> {
        let inner = TASK_MANAGER.inner.borrow();
        let current = inner.current_task;
        (current + 1..current + TASK_MANAGER.num_app + 1)
            .map(|id| id % TASK_MANAGER.num_app)
            .find(|id| {
                inner.tasks[*id].task_status == TaskStatus::Ready
            })
    }
}