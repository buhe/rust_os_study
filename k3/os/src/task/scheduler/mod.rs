
pub mod rr;
pub mod stride;
use alloc::sync::Arc;
use spin::Mutex;
use crate::config::{MAX_APP_NUM};
use alloc::vec::Vec;
use super::task::TaskControlBlock;
pub trait Scheduler {
    fn find_next_task(&self) -> Option<usize> ;
    fn new(tasks: [Mutex<TaskControlBlock>; MAX_APP_NUM]) -> Self;
}