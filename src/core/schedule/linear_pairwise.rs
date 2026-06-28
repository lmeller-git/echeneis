use std::{marker::PhantomData, ops::ControlFlow, panic::Location};

use corosensei::{ScopedCoroutine, ScopedCoroutineRef, stack::DefaultStack};

use crate::{
    build_test::Model,
    core::{
        rt::{
            CheckedTaskHandle,
            YieldData,
            coroutine::{CoroTaskHandle, transmute_lt},
            env::CURRENT_TASK,
        },
        schedule::{Strategy, TestSchedule},
    },
};

fn with_coroutine<'a, D, F, R>(
    stack: &'a mut DefaultStack,
    state: &'a D,
    preempted: &'a F,
    f: impl FnOnce(ScopedCoroutineRef<'_, (), YieldData, (), &'a mut DefaultStack>) -> R,
) -> R
where
    F: Fn(&D),
{
    let c = ScopedCoroutine::with_stack(stack, |yielder, _| {
        let _guard = ClearTaskGuard;
        let handle = Box::new(CoroTaskHandle::new(yielder));
        // Safety:
        // we will drop it before return/on panic via _guard
        CURRENT_TASK.set(Some(unsafe { transmute_lt(handle) }));
        preempted(state);
    });
    c.scope(f)
}

pub(crate) fn run_checked_once<I, F, D, C>(
    model: &Model<I, F, D, C>,
    state: &D,
    loc: Option<&'static Location<'static>>,
) -> ControlFlow<()>
where
    I: Fn() -> D,
    F: Fn(&D),
    C: Fn(&D) -> ControlFlow<()>,
{
    let mut state_checker = CheckedTaskHandle::new(model.steps);
    let _guard = ClearTaskGuard;
    if let Some(loc) = loc {
        state_checker = state_checker.with_preempted_loc(loc);
    }

    let current = CURRENT_TASK.take();
    CURRENT_TASK.set(Some(Box::new(state_checker)));

    let r = model.checked_fn()(state);

    drop(_guard);
    CURRENT_TASK.set(current);

    r
}

struct ClearTaskGuard;

impl Drop for ClearTaskGuard {
    fn drop(&mut self) {
        CURRENT_TASK.set(None);
    }
}

pub(crate) struct RestartPairwise<I, F, D, C, S> {
    strategy_provider: S,
    phantom: PhantomData<(I, F, D, C)>,
}

impl<I, F, D, C, S> RestartPairwise<I, F, D, C, S> {
    pub(crate) fn new(strategy_provider: S) -> Self {
        Self {
            strategy_provider,
            phantom: PhantomData,
        }
    }
}

impl<I, F, D, C, S> TestSchedule<I, F, D, C> for RestartPairwise<I, F, D, C, S>
where
    I: Fn() -> D,
    F: Fn(&D),
    C: Fn(&D) -> ControlFlow<()>,
    S: Strategy<I, F, D, C>,
{
    fn check_model(&mut self, model: Model<I, F, D, C>) {
        let preempted = model.preemptible_fn();
        let mut stack = DefaultStack::default();
        let mut yields = 0;

        loop {
            let init_state = model.init_fn()();

            if with_coroutine(&mut stack, &init_state, preempted, |mut coro| {
                let mut current_yields = 0;
                loop {
                    match coro.resume(()) {
                        corosensei::CoroutineResult::Yield(result) => match result {
                            YieldData::AtomicTransition(loc) => {
                                if current_yields < yields {
                                    current_yields += 1;
                                    continue;
                                }
                                let r = self.strategy_provider.next_check(&model, &init_state, loc);
                                yields += 1;
                                if r.is_continue() {
                                    coro.force_unwind();
                                }
                                break r;
                            }
                            YieldData::Complete | YieldData::Terminated => unreachable!(),
                        },
                        corosensei::CoroutineResult::Return(_) => {
                            _ = self.strategy_provider.next_check(&model, &init_state, None);
                            break ControlFlow::Break(());
                        }
                    }
                }
            })
            .is_break()
            {
                break;
            }
        }
    }
}

pub(crate) struct PersistentPairwise<I, F, D, C, S> {
    strategy_provider: S,
    phantom: PhantomData<(I, F, D, C)>,
}

impl<I, F, D, C, S> PersistentPairwise<I, F, D, C, S> {
    pub(crate) fn new(strategy_provider: S) -> Self {
        Self {
            strategy_provider,
            phantom: PhantomData,
        }
    }
}

impl<I, F, D, C, S> TestSchedule<I, F, D, C> for PersistentPairwise<I, F, D, C, S>
where
    I: Fn() -> D,
    F: Fn(&D),
    C: Fn(&D) -> ControlFlow<()>,
    S: Strategy<I, F, D, C>,
{
    fn check_model(&mut self, model: Model<I, F, D, C>) {
        let preempted = model.preemptible_fn();
        let mut stack = DefaultStack::default();
        let init_state = model.init_fn()();

        with_coroutine(&mut stack, &init_state, preempted, |mut coro| {
            loop {
                if match coro.resume(()) {
                    corosensei::CoroutineResult::Yield(result) => match result {
                        YieldData::AtomicTransition(loc) => {
                            self.strategy_provider.next_check(&model, &init_state, loc)
                        }
                        YieldData::Complete | YieldData::Terminated => unreachable!(),
                    },
                    corosensei::CoroutineResult::Return(_) => {
                        _ = self.strategy_provider.next_check(&model, &init_state, None);
                        ControlFlow::Break(())
                    }
                }
                .is_break()
                {
                    return;
                }
            }
        });
    }
}
