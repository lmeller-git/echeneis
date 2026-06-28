use std::cell::Cell;

use crate::core::rt::TaskHandle;

crate::core::sync::thread_local! {
    pub(crate) static CURRENT_TASK: Cell<Option<Box<dyn TaskHandle>>> = const { Cell::new(None) };
}

pub(crate) fn yield_current(payload: super::YieldData) {
    let mut current = CURRENT_TASK.take();

    if let Some(task) = &mut current {
        task.yield_now(payload);
    }

    CURRENT_TASK.set(current);
}
