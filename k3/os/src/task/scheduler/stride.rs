use super::super::TaskStatus;
// use super::super::TASK_MANAGER;
use super::Scheduler;
use crate::{config::MAX_APP_NUM, task::task::TaskControlBlock};

use heapless::binary_heap::{BinaryHeap, Max};

// 关于pass的值，等于BIG_STRIDE/优先级
pub struct Stride {
    heap: BinaryHeap<&TaskControlBlock, Max, 64>,
}

impl Scheduler for Stride {
    fn new(tasks: &[&TaskControlBlock; MAX_APP_NUM]) -> Self {
        let mut scheduler = Self {
            heap: BinaryHeap::new(),
        };
        for t in (*tasks).iter() {
            scheduler.heap.push(*t).unwrap();
        }
        scheduler
    }
    fn find_next_task(&self) -> Option<usize> {
        // let inner = TASK_MANAGER.inner.borrow();
        // let current = inner.current_task;
        // (current + 1..current + TASK_MANAGER.num_app + 1)
        //     .map(|id| id % TASK_MANAGER.num_app)
        //     .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
        let next = self.heap.peek().unwrap();
        debug!("next is {:?}", next);
        if next.task_status == TaskStatus::Ready {
            Some(next.task_mo.into())
            // (0..TASK_MANAGER.num_app + 1)
                // .map(|id| id % TASK_MANAGER.num_app)
                // .find(|id| inner.tasks[*id] == *next)
        } else {
            self.find_next_task()
        }
        // .and(|id|  inner.tasks[*id].task_status == TaskStatus::Ready)
    }
}
