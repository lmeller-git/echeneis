use std::marker::PhantomData;

use crate::core::schedule::{TestSchedule, linear_pairwise::ExhaustivePairwise};

pub struct ModelBuilder {
    max_steps: usize,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            max_steps: 1_000_000,
        }
    }

    pub fn with_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        self
    }

    pub fn check<I, F, D>(&mut self, init: I, preempted: F, checked: F)
    where
        I: Fn() -> D,
        F: Fn(&D) + Sync,
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

pub(crate) struct Model<I, F, D> {
    pub init_fn: I,
    pub preempted_fn: F,
    pub checked_fn: F,
    _phantom: PhantomData<D>,
}

impl<I, F, D> Model<I, F, D>
where
    I: Fn() -> D,
    F: Fn(&D),
{
    pub(crate) fn init_fn(&self) -> &I {
        &self.init_fn
    }

    pub(crate) fn preemptible_fn(&self) -> &F {
        &self.preempted_fn
    }

    pub(crate) fn checked_fn(&self) -> &F {
        &self.checked_fn
    }
}
