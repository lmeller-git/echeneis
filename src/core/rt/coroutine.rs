use std::thread;

use corosensei::Yielder;

use crate::core::rt::{TaskHandle, YieldData};

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
