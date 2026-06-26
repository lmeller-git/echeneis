use std::marker::PhantomData;

use crate::core::schedule::{TestSchedule, linear_pairwise::ExhaustivePairwise};

pub struct ModelBuilder {
    max_steps: usize,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self { max_steps: 100_000 }
    }

    pub fn with_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        self
    }

    pub fn check<I, F, D, C>(&mut self, init: I, preempted: F, checked: C)
    where
        I: Fn() -> D,
        F: Fn(&D) + Sync,
        C: Fn(&D),
        D: Sync,
    {
        let model = Model {
            init_fn: init,
            preempted_fn: preempted,
            checked_fn: checked,
            _phantom: PhantomData::<D>,
        };

        let mut scheduler = ExhaustivePairwise::new();
        scheduler.check_model(model);
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
    _phantom: PhantomData<D>,
}

impl<I, F, D, C> Model<I, F, D, C>
where
    I: Fn() -> D,
    F: Fn(&D),
    C: Fn(&D),
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
