use core::cmp::Ordering;
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TaskControlBlock {
    pub task_cx_ptr: usize,
    pub task_status: TaskStatus,
    pub task_sride: u8,
    pub task_priority: u8,
}

impl TaskControlBlock {
    pub fn get_task_cx_ptr2(&self) -> *const usize {
        &self.task_cx_ptr as *const usize
    }
}

impl Ord for TaskControlBlock {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        // other.cost.cmp(&self.cost)
        //     .then_with(|| self.position.cmp(&other.position))
        other.task_sride.cmp(&self.task_sride)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for TaskControlBlock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone,Eq, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}