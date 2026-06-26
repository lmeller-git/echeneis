use crate::core::{
    rt::{TaskHandle, YieldData},
    sync::{
        Arc,
        Condvar,
        Mutex,
        thread::{self, Thread},
    },
};

pub(crate) struct ThreadTaskHandle {
    scheduler: Thread,
    data: Arc<Mutex<Option<YieldData>>>,
}

impl ThreadTaskHandle {
    pub(crate) fn new(scheduler_thread: Thread, data: Arc<Mutex<Option<YieldData>>>) -> Self {
        Self {
            scheduler: scheduler_thread,
            data,
        }
    }
}

impl TaskHandle for ThreadTaskHandle {
    fn yield_now(&mut self, payload: YieldData) {
        self.data.lock().replace(payload);
        self.scheduler.unpark();

        thread::park();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Turn {
    Scheduler(YieldData),
    Task,
}

pub(crate) struct ThreadTaskHandle_ {
    cvar_pair: Arc<(Mutex<Turn>, Condvar)>,
}

impl ThreadTaskHandle_ {
    pub(crate) fn new(cvar_pair: Arc<(Mutex<Turn>, Condvar)>) -> Self {
        Self { cvar_pair }
    }
}

impl TaskHandle for ThreadTaskHandle_ {
    fn yield_now(&mut self, payload: YieldData) {
        // at this point state shouldbe Turn::Task.
        // We want to giove a turn to the scheduler and wait until its our turn again
        let mut state = self.cvar_pair.0.lock();

        debug_assert_eq!(*state, Turn::Task);
        *state = Turn::Scheduler(payload);
        drop(state);

        self.cvar_pair.1.notify_all();

        let state = self.cvar_pair.0.lock();
        drop(
            self.cvar_pair
                .1
                .wait_while(state, |state| !matches!(state, Turn::Task))
                .unwrap(),
        );
    }
}
