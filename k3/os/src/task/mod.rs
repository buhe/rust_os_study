mod context;
mod switch;
mod task;
mod scheduler;
use crate::config::{MAX_APP_NUM, INIT_PRIORITY, BIG_STRIDE};
use crate::loader::{get_num_app, init_app_cx};
use core::borrow::Borrow;
use core::{cell::RefCell};
use alloc::rc::Rc;
use spin::Mutex;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};
use scheduler::stride::Stride;
use scheduler::Scheduler;
use alloc::vec::Vec;

pub use context::TaskContext;

pub struct TaskManager {
    num_app: usize,
    inner: RefCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: Vec<Rc<RefCell<TaskControlBlock>>>,
    current_task: usize,
     s: Stride,
}

unsafe impl Sync for TaskManager {}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = Vec::new();
        tasks.push(Rc::new(RefCell::new(TaskControlBlock {task_mo:0, task_cx_ptr: 0, task_status: TaskStatus::UnInit, task_sride: 0, task_priority: INIT_PRIORITY })));
        for i in 0..num_app {
            tasks[i].borrow_mut().task_priority = i as u8+ 2;
            tasks[i].borrow_mut().task_cx_ptr = init_app_cx(i) as * const _ as usize;
            tasks[i].borrow_mut().task_status = TaskStatus::Ready;
            tasks[i].borrow_mut().task_mo = i as u8;
        }
        TaskManager {
            num_app,
            inner: RefCell::new(TaskManagerInner {
                s: Stride::new(tasks),
                tasks,
                current_task: 0,
            }),
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) {
        self.inner.borrow_mut().tasks[0].borrow_mut().task_status = TaskStatus::Running;
        let next_task_cx_ptr2 = self.inner.borrow().tasks[0].borrow_mut().get_task_cx_ptr2();
        let _unused: usize = 0;
        unsafe {
            __switch(
                &_unused as *const _,
                next_task_cx_ptr2,
            );
        }
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].borrow_mut().task_status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.borrow_mut();
        let current = inner.current_task;
        inner.tasks[current].borrow_mut().task_status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.borrow();
        inner.s.find_next_task()
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.borrow_mut();
            let current = inner.current_task;
            let mut next_task = inner.tasks[next];
            // inner.s.push(next_task);
            next_task.borrow_mut().task_status = TaskStatus::Running;
            let pass = BIG_STRIDE / next_task.borrow_mut().task_priority;
            next_task.borrow_mut().task_sride += pass;
            // debug!("task stride is {},{},{}", next_task.borrow().task_sride,current,next);
            inner.current_task = next;
            let current_task_cx_ptr2 = inner.tasks[current].borrow_mut().get_task_cx_ptr2();
            let next_task_cx_ptr2 = inner.tasks[next].borrow_mut().get_task_cx_ptr2();
            core::mem::drop(inner);
            unsafe {
                __switch(
                    current_task_cx_ptr2,
                    next_task_cx_ptr2,
                );
            }
        } else {
            panic!("All applications completed!");
        }
    }
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}