use std::{marker::PhantomData, panic::Location};

use crate::{
    build_test::Model,
    core::{
        rt::{
            CheckedTaskHandle,
            env::CURRENT_TASK,
            thread::{ThreadTaskHandle, Turn},
        },
        schedule::TestSchedule,
        sync::{Arc, Condvar, Mutex, thread},
    },
};

struct WorkerSentinel {
    cvar_pair: Arc<(Mutex<Turn>, Condvar)>,
}

impl WorkerSentinel {
    fn new(cvar_pair: Arc<(Mutex<Turn>, Condvar)>) -> Self {
        Self { cvar_pair }
    }
}

impl Drop for WorkerSentinel {
    fn drop(&mut self) {
        let completion_reason = if thread::panicking() {
            crate::core::rt::YieldData::Terminated
        } else {
            crate::core::rt::YieldData::Complete
        };

        *self.cvar_pair.0.lock() = Turn::Scheduler(completion_reason);
        self.cvar_pair.1.notify_all();
    }
}

struct SchedulerGuard {
    cvar_pair: Arc<(Mutex<Turn>, Condvar)>,
}

impl SchedulerGuard {
    fn new(cvar_pair: Arc<(Mutex<Turn>, Condvar)>) -> Self {
        Self { cvar_pair }
    }
}

impl Drop for SchedulerGuard {
    fn drop(&mut self) {
        if thread::panicking() {
            let mut state = self.cvar_pair.0.lock();
            *state = Turn::Poisoned;
            self.cvar_pair.1.notify_all();
        }
    }
}

pub(crate) struct ExhaustivePairwise<I, F, D, C> {
    phantom: PhantomData<(I, F, D, C)>,
}

impl<I, F, D, C> ExhaustivePairwise<I, F, D, C> {
    pub(crate) fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<I, F, D, C> ExhaustivePairwise<I, F, D, C>
where
    I: Fn() -> D,
    F: Fn(&D) + Sync,
    C: Fn(&D),
    D: Sync,
{
    fn run_checked_once(
        &self,
        model: &Model<I, F, D, C>,
        state: &D,
        loc: Option<&'static Location<'static>>,
    ) {
        let mut state_checker = CheckedTaskHandle::new(1_000_000);
        if let Some(loc) = loc {
            state_checker = state_checker.with_preempted_loc(loc);
        }
        CURRENT_TASK.with_borrow_mut(|cell| cell.replace(Box::new(state_checker)));

        model.checked_fn()(state);

        CURRENT_TASK.with_borrow_mut(|cell| cell.take());
    }
}

impl<I, F, D, C> TestSchedule<I, F, D, C> for ExhaustivePairwise<I, F, D, C>
where
    I: Fn() -> D,
    F: Fn(&D) + Sync,
    C: Fn(&D),
    D: Sync,
{
    fn check_model(&mut self, model: Model<I, F, D, C>) {
        let init_state = model.init_fn()();
        let preempted = model.preemptible_fn();
        let cvar_pair = Arc::new((Mutex::new(Turn::Task), Condvar::new()));

        thread::scope(|s| {
            let _preempted = s.spawn(|| {
                let _drop_guard = WorkerSentinel::new(cvar_pair.clone());
                let preempted_task_thread = Box::new(ThreadTaskHandle::new(cvar_pair.clone()));
                CURRENT_TASK.with_borrow_mut(|cell| cell.replace(preempted_task_thread));

                preempted(&init_state);
            });

            let _drop_guard = SchedulerGuard::new(cvar_pair.clone());

            loop {
                let mut state = cvar_pair.0.lock();
                state = cvar_pair
                    .1
                    .wait_while(state, |state| matches!(state, Turn::Task))
                    .unwrap();

                let Turn::Scheduler(reason) = *state else {
                    panic!(
                        "The mutex associated with the condvar was overwritten before the scheduler could read it"
                    )
                };

                drop(state);

                match reason {
                    crate::core::rt::YieldData::AtomicTransition(loc) => {
                        // run checked once and wait for the next state
                        self.run_checked_once(&model, &init_state, loc);
                    }
                    crate::core::rt::YieldData::Terminated => {
                        panic!("Failed")
                    }
                    crate::core::rt::YieldData::Complete => {
                        // run checked one more time, then end
                        self.run_checked_once(&model, &init_state, None);
                        break;
                    }
                }

                *cvar_pair.0.lock() = Turn::Task;
                cvar_pair.1.notify_all();
            }
        });
    }
}
