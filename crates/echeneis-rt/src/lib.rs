use std::{cell::Cell, panic::Location};

use corosensei::Yielder;
use echeneis_core::{Task, TaskHandle, TaskID, sync::thread};

echeneis_core::sync::thread_local! {
    pub(crate) static EXEC_ENV: Cell<Option<ExecutionState>> = const { Cell::new(None) };
}

pub struct ExecutionState {
    tasks: Vec<Task>,
    current: TaskID,
    next: TaskID,
    scheduler: Box<dyn Schedule>,
}

impl ExecutionState {
    pub fn with(f: impl FnOnce(&mut Self) -> R) -> R {
        EXEC_ENV.with(f)
    }

    pub fn curent() -> &Task {
        todo!()
    }
}

pub struct Master {}

pub(crate) fn yield_current(payload: YieldData) {
    let mut current = CURRENT_TASK.take();

    if let Some(task) = &mut current {
        task.yield_now(payload);
    }

    CURRENT_TASK.set(current);
}

pub(crate) struct CoroTaskHandle<'a> {
    yielder: &'a Yielder<(), YieldData>,
}

impl<'a> CoroTaskHandle<'a> {
    pub(crate) fn new(yielder: &'a Yielder<(), YieldData>) -> Self {
        Self { yielder }
    }
}

impl<'a> TaskHandle for CoroTaskHandle<'a> {
    fn yield_now(&mut self, payload: YieldData) {
        if !thread::panicking() {
            self.yielder.suspend(payload);
        }
    }
}

pub(crate) unsafe fn transmute_lt<'a>(
    state: Box<CoroTaskHandle<'a>>,
) -> Box<CoroTaskHandle<'static>> {
    // Safety:
    // need to manually ensure that its used correctly
    unsafe { ::std::mem::transmute(state) }
}
