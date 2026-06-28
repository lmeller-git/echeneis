use std::{marker::PhantomData, ops::ControlFlow};

use crate::core::schedule::{
    CloneCheck,
    Retry,
    Shared,
    Strategy,
    TestSchedule,
    linear_pairwise::{PersistentPairwise, RestartPairwise},
};

/// Configure a model
#[derive(Debug)]
#[non_exhaustive]
pub struct ModelBuilder {
    max_steps: usize,
}

impl ModelBuilder {
    /// Conbstructs a new `ModelBuilder`.
    pub fn new() -> Self {
        Self { max_steps: 100_000 }
    }

    /// Set the maximum number of steps.
    pub fn with_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        self
    }

    /// Checks the model by checking `checked` against each preemption point in `preempted`.
    /// To ensure consistent state `preempted` is replayed up to the last checked point at each iteration.
    pub fn check_pairwise<I, F, D, C>(&self, init: I, preempted: F, checked: C)
    where
        I: Fn() -> D,
        F: Fn(&D),
        C: Fn(&D) -> ControlFlow<()>,
        D: Sync,
    {
        let model = Model::new(init, preempted, checked, self.max_steps);
        RestartPairwise::new(Shared::new_with(&model)).check_model(model);
    }

    /// Checks the model by checking `checked` against each preemption point in `preempted`.
    /// To ensure consistent state the state `D` is cloned at each iteration.
    ///
    /// Scales better than `check_pairwise`.
    pub fn check_pairwise_clone<I, F, D, C>(&self, init: I, preempted: F, checked: C)
    where
        I: Fn() -> D,
        F: Fn(&D),
        C: Fn(&D) -> ControlFlow<()>,
        D: Sync + Clone,
    {
        let model = Model::new(init, preempted, checked, self.max_steps);
        PersistentPairwise::new(CloneCheck::new_with(&model)).check_model(model);
    }

    /// Checks the model by interpreting `checked` as a retry loop on `D` during `preempted`.
    /// The retry loop should complete in bounded time.
    pub fn check_retry<I, F, D, C>(&self, init: I, preempted: F, checked: C)
    where
        I: Fn() -> D,
        F: Fn(&D),
        C: Fn(&D) -> ControlFlow<()>,
        D: Sync,
    {
        let model = Model::new(init, preempted, checked, self.max_steps);
        PersistentPairwise::new(Retry::new_with(&model)).check_model(model);
    }
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) struct Model<I, F, D, C> {
    pub init_fn: I,
    pub preempted_fn: F,
    pub checked_fn: C,
    pub steps: usize,
    _phantom: PhantomData<D>,
}

impl<I, F, D, C> Model<I, F, D, C> {
    pub(crate) fn new(init_fn: I, preempted_fn: F, checked_fn: C, steps: usize) -> Self {
        Self {
            init_fn,
            preempted_fn,
            checked_fn,
            steps,
            _phantom: PhantomData,
        }
    }
}

impl<I, F, D, C> Model<I, F, D, C>
where
    I: Fn() -> D,
    F: Fn(&D),
    C: Fn(&D) -> ControlFlow<()>,
{
    pub(crate) fn init_fn(&self) -> &I {
        &self.init_fn
    }

    pub(crate) fn preemptible_fn(&self) -> &F {
        &self.preempted_fn
    }

    pub(crate) fn checked_fn(&self) -> &C {
        &self.checked_fn
    }
}
