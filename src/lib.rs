pub(crate) mod build_test;
pub(crate) mod core;
mod expose;

pub use build_test::ModelBuilder;
pub use expose::*;

pub fn check_pairwise<I, F, D, C>(init_fn: I, preempt: F, check: C)
where
    I: Fn() -> D,
    F: Fn(&D) + Sync,
    C: Fn(&D),
    D: Sync,
{
    ModelBuilder::new().check(init_fn, preempt, check);
}
