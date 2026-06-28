use std::{cell::Cell, ops::ControlFlow, panic::Location, rc::Rc};

use crate::{
    build_test::Model,
    core::{
        rt::{TaskHandle, YieldData, env::CURRENT_TASK},
        schedule::linear_pairwise::run_checked_once,
    },
};

pub(crate) mod linear_pairwise;

pub(crate) trait TestSchedule<I, F, D, C> {
    fn check_model(&mut self, model: Model<I, F, D, C>);
}

pub(crate) trait Strategy<I, F, D, C> {
    fn next_check(
        &mut self,
        model: &Model<I, F, D, C>,
        state: &D,
        loc: Option<&'static Location<'static>>,
    ) -> ControlFlow<()>;

    fn new_with(model: &Model<I, F, D, C>) -> Self;
}

struct ClearTaskGuard;

impl Drop for ClearTaskGuard {
    fn drop(&mut self) {
        CURRENT_TASK.set(None);
    }
}

#[derive(Default)]
pub(crate) struct CloneCheck;

impl<I, F, D, C> Strategy<I, F, D, C> for CloneCheck
where
    D: Clone,
    I: Fn() -> D,
    F: Fn(&D),
    C: Fn(&D) -> ControlFlow<()>,
{
    fn next_check(
        &mut self,
        model: &Model<I, F, D, C>,
        state: &D,
        loc: Option<&'static Location<'static>>,
    ) -> ControlFlow<()> {
        let cloned_state = state.clone();
        run_checked_once(model, &cloned_state, loc)
    }

    fn new_with(_model: &Model<I, F, D, C>) -> Self {
        Self
    }
}

pub(crate) struct Retry {
    steps: Rc<Cell<usize>>,
    max_steps: usize,
}

struct RetryHandle {
    steps: Rc<Cell<usize>>,
    max_steps: usize,
    loc: Option<&'static Location<'static>>,
}

impl TaskHandle for RetryHandle {
    fn yield_now(&mut self, payload: YieldData) {
        match payload {
            YieldData::AtomicTransition(loc) => {
                let s = self.steps.get() + 1;
                if s > self.max_steps {
                    panic!(
                        "Maximum steps exceeded {}.\n\
                 Exceeded allowed limit of {} atomic transitions {}.
                 Did you call a spin-loop or similar somewhere?",
                        if let Some(loc) = self.loc {
                            format!(
                                "while another thread was preempted before completing the operation at {}",
                                loc
                            )
                        } else {
                            "".into()
                        },
                        self.max_steps,
                        if let Some(loc) = loc {
                            format!("while trying to evaluate the operation at {}", loc)
                        } else {
                            "".into()
                        }
                    );
                }
                self.steps.set(s);
            }
            YieldData::Complete => {}
            YieldData::Terminated => panic!("terminated"),
        }
    }
}

impl<I, F, D, C> Strategy<I, F, D, C> for Retry
where
    I: Fn() -> D,
    F: Fn(&D),
    C: Fn(&D) -> ControlFlow<()>,
{
    fn next_check(
        &mut self,
        model: &Model<I, F, D, C>,
        state: &D,
        loc: Option<&'static Location<'static>>,
    ) -> ControlFlow<()> {
        let _guard = ClearTaskGuard;

        let handle = RetryHandle {
            steps: self.steps.clone(),
            max_steps: self.max_steps,
            loc,
        };

        let current = CURRENT_TASK.take();
        CURRENT_TASK.set(Some(Box::new(handle)));

        let r = model.checked_fn()(state);

        drop(_guard);
        CURRENT_TASK.set(current);
        r
    }

    fn new_with(model: &Model<I, F, D, C>) -> Self {
        Self {
            steps: Rc::new(Cell::new(0)),
            max_steps: model.steps,
        }
    }
}

pub(crate) struct Shared;

impl<I, F, D, C> Strategy<I, F, D, C> for Shared
where
    I: Fn() -> D,
    F: Fn(&D),
    C: Fn(&D) -> ControlFlow<()>,
{
    fn next_check(
        &mut self,
        model: &Model<I, F, D, C>,
        state: &D,
        loc: Option<&'static Location<'static>>,
    ) -> ControlFlow<()> {
        run_checked_once(model, state, loc)
    }

    fn new_with(_model: &Model<I, F, D, C>) -> Self {
        Self
    }
}
