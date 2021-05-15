
pub mod rr;
pub mod stride;
use crate::config::{MAX_APP_NUM};

use super::task::TaskControlBlock;
pub trait Scheduler {
    fn push(&mut self, task: TaskControlBlock) -> u8;
    fn find_next_task(&self) -> Option<usize> ;
    fn new(tasks: [TaskControlBlock; MAX_APP_NUM]) -> Self;
}