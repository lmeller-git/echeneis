use crate::core::{
    rt::{TaskHandle, YieldData},
    sync::{Arc, Condvar, Mutex},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Turn {
    Scheduler(YieldData),
    Task,
    Poisoned,
}

pub(crate) struct ThreadTaskHandle {
    cvar_pair: Arc<(Mutex<Turn>, Condvar)>,
}

impl ThreadTaskHandle {
    pub(crate) fn new(cvar_pair: Arc<(Mutex<Turn>, Condvar)>) -> Self {
        Self { cvar_pair }
    }
}

impl TaskHandle for ThreadTaskHandle {
    fn yield_now(&mut self, payload: YieldData) {
        // at this point state should be Turn::Task.
        // We want to giove a turn to the scheduler and wait until its our turn again
        let mut state = self.cvar_pair.0.lock();

        if *state == Turn::Poisoned {
            drop(state);
            panic!("Echeneis Internal Abort")
        }

        debug_assert_eq!(*state, Turn::Task);

        *state = Turn::Scheduler(payload);
        drop(state);

        self.cvar_pair.1.notify_all();

        let mut state = self.cvar_pair.0.lock();
        state = self
            .cvar_pair
            .1
            .wait_while(state, |state| !matches!(state, Turn::Task | Turn::Poisoned))
            .unwrap();

        if *state == Turn::Poisoned {
            drop(state);
            panic!("Echeneis Internal Abort");
        }
    }
}
