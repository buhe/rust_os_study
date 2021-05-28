
pub mod rr;
pub mod stride;
use core::cell::RefCell;

use alloc::{rc::Rc, sync::Arc};
use spin::Mutex;
use crate::config::{MAX_APP_NUM};
use alloc::vec::Vec;
use super::task::TaskControlBlock;
pub trait Scheduler {
    fn find_next_task(&self) -> Option<usize> ;
    fn new(tasks: Vec<Rc<RefCell<TaskControlBlock>>>) -> Self;
}