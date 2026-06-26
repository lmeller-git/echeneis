use std::cell::RefCell;

use crate::core::rt::TaskHandle;

crate::core::sync::thread_local! {
    pub(crate) static CURRENT_TASK: RefCell<Option<Box<dyn TaskHandle>>> = const { RefCell::new(None) };
}

pub(crate) fn yield_current(payload: super::YieldData) {
    CURRENT_TASK.with_borrow_mut(|task| {
        if let Some(task) = task {
            task.yield_now(payload);
        }
    });
}
