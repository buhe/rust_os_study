use alloc::sync::Arc;

use super::Scheduler;
use super::super::TASK_MANAGER;
use super::super::TaskStatus;

pub struct RR;
use crate::{config::{MAX_APP_NUM}, task::task::TaskControlBlock};
use alloc::vec::Vec;
impl Scheduler for RR {
    fn new(_: Vec<Arc<TaskControlBlock>>) -> Self{Self{}}
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