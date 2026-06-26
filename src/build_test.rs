use std::marker::PhantomData;

pub struct ModelBuilder {}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check<I, F, D>(&mut self, init: I, preempted: F, checked: F)
    where
        I: Fn() -> D,
        F: Fn(&D),
        D: Sync,
    {
        todo!()
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
