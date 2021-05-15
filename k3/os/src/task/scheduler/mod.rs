
pub mod rr;
pub mod stride;
pub trait Scheduler {
    fn find_next_task(&self) -> Option<usize> ;
    fn new() -> Self;
}