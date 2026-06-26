#![deny(missing_docs)]
#![deny(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
#![warn(unsafe_op_in_unsafe_fn)]

//! Echeneis is a controlled model-checking tool designed specifically to detect blocking code in concurrent Rust programs. It systematically explores execution paths to verify whether one thread could block another.
//!
//! ## Why echeneis?
//!
//! Many concurrent algorithms expect a specific sequence of operations to complete without interruption.
//! If a thread is preempted at a critical operation, such as right after updating a state flag, it can inadvertently leave the rest of the system completely stalled.
//!
//! Exhaustive model checkers like `loom` may face runtime problems on complex systems in practice, limiting there use for exhaustivce checking of these systems somewhat.
//! `Echeneis` tries to solve this by focusing only on a small subset of bugs (pairwise blocks) and searchign for them by checking if some thread blocks given another is preempted at some point.
//! This crate also makes no assumption about the memory model, delegating this layer to native atomic implementations.
//!
//! Focusing on pairwise blocking interactions allows fast and simple checking with informative errors of concurrent models.
//!
//! ## Quick Start
//!
//! Using `echeneis::check_pairwise`, you can orchestrate an initial state, a preemptible worker thread, and a checking thread to ensure that the checker can always make progress regardless of when the worker is interrupted.
//!
//! First add echeneis to your dependencies:
//!
//! ```toml
//! [target.'cfg(echeneis)'.dependencies]
//! echeneis = "0.1"
//! ```
//!
//! Next, create a test file and add a test:
//!
//! ```rust,no_run
//!   use echeneis::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
//!   #[test]
//!   #[should_panic]
//!   fn blocking_concurrent() {
//!     echeneis::check_pairwise(
//!         || (AtomicBool::new(true), AtomicUsize::new(0)),
//!         |(b, v)| {
//!             for _ in 0..3 {
//!                 if v.fetch_add(1, Ordering::AcqRel) == 2 {
//!                     b.store(false, Ordering::Release);
//!                     // if we get preempted here, it should block
//!                     b.store(true, Ordering::Release);
//!                 }
//!             }
//!         },
//!         |(b, _)| {
//!             while ! b.load(Ordering::Acquire) {}
//!         },
//!     );
//!   }
//! ```
//!
//! Then, run the test with
//!
//! ```console
//! RUSTFLAGS="--cfg echeneis" cargo test --test blocking_concurrent
//! ```
//!
//! ## Writing tests
//!
//! Echeneis intrusively redirects all calls to synchronization primitives in order to check preemption permutations systematically.
//! This means that all tested functionality should use the synchronization primitives exported in this crate.
//!
//! One way to do so easily is to use an `echeneis` cfg flag and conditionally use `echeneis` synchronization primitives for test runs by conditionally exporting them in some central module:
//!
//! ```rust, no_run
//!  #[cfg(echeneis)]
//!  pub(crate) use echeneis::sync::atomic::AtomicUsize;
//!
//!  #[cfg(not(echeneis))]
//!  pub(crate) use std::sync::atomic::AtomicUsize;
//! ```
//! Then, elsewhere:
//!
//! ```rust, no_run
//!  use crate::sync::AtomicUsize;
//! ```
//!
//! ## Limitations
//!
//! Echeneis currently only supports `check_pairwise`. In other words, blocks depending on n-way scheduling decisions can currently not be detected.
//! Further, `check_pairwise` currently does not reistantiate the init state created by `init_fn`. If the code in `preempt` or `checked` mutate the state in a way that changes the execution, blocking code-paths may be missed or falsely detected.
//!
//! ## Feature Flags
//!
//! - `portable-atomic`: Uses the `portable-atomic` crate as backend for atomics
//!
//! - `atomic-float`: Enables `portable-atomic` `float` feature and exposes shims for atomic floats.
//!
//! - `atomic-fallback`: Uses `portable-atomic` `fallback` feature for atomics if necessary.
//!
//! - `default`:
//!

pub(crate) mod build_test;
pub(crate) mod core;
mod expose;

pub use build_test::ModelBuilder;
pub use expose::*;

/// Exhaustively checks wether closure passed as `check` blocks on any (atomic) operation in `preempt`.
///
/// The state created by `init_fn` may be reused multiple times. This is akin to retrying `check` on the state created by `init_fn` while `preempt` runs once.
///
/// Uses a default ModelBuilder.
pub fn check_pairwise<I, F, D, C>(init_fn: I, preempt: F, check: C)
where
    I: Fn() -> D,
    F: Fn(&D) + Sync,
    C: Fn(&D),
    D: Sync,
{
    ModelBuilder::new().check(init_fn, preempt, check);
}
