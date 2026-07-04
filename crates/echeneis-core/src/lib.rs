use std::panic::Location;

pub mod sync;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum YieldData {
    AtomicTransition(Option<&'static Location<'static>>),
    #[allow(unused)]
    Complete,
    #[allow(unused)]
    Terminated,
}

pub trait TaskHandle {
    fn yield_now(&mut self, payload: YieldData);
}

pub struct Task {
    yielder: Box<dyn TaskHandle>,
}

pub struct TaskID {}
