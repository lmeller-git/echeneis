use std::thread;

use corosensei::Yielder;

use crate::core::rt::{TaskHandle, YieldData};

pub(crate) struct CoroTaskHandle<'a> {
    yielder: &'a Yielder<usize, YieldData>,
    local_yield_count: usize,
    target_yield_count: usize,
}

impl<'a> CoroTaskHandle<'a> {
    pub(crate) fn new(yielder: &'a Yielder<usize, YieldData>, target_yield_count: usize) -> Self {
        Self {
            yielder,
            local_yield_count: 0,
            target_yield_count,
        }
    }
}

impl<'a> TaskHandle for CoroTaskHandle<'a> {
    fn yield_now(&mut self, payload: YieldData) {
        if self.local_yield_count >= self.target_yield_count && !thread::panicking() {
            self.target_yield_count = self.yielder.suspend(payload);
        }

        self.local_yield_count += 1;
    }
}

pub(crate) unsafe fn transmute_lt<'a>(
    state: Box<CoroTaskHandle<'a>>,
) -> Box<CoroTaskHandle<'static>> {
    // Safety:
    // need to manually ensure that its used correctly
    unsafe { ::std::mem::transmute(state) }
}
