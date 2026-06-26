pub(crate) mod build_test;
pub(crate) mod core;
mod expose;

pub use expose::*;

pub use build_test::ModelBuilder;

pub fn check_pairwise<I, F, D>(init_fn: I, preempt: F, check: F)
where
    I: Fn() -> D,
    F: Fn(&D) + Sync,
    D: Sync,
{
    ModelBuilder::new().check(init_fn, preempt, check);
}
