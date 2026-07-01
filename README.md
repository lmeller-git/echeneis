[![Codecov](https://codecov.io/github/lmeller-git/echeneis/coverage.svg?branch=main)](https://codecov.io/gh/lmeller-git/echeneis)
![CI Test](https://github.com/lmeller-git/echeneis/actions/workflows/test.yml/badge.svg?branch=main)
![Safety Test](https://github.com/lmeller-git/echeneis/actions/workflows/safety.yml/badge.svg?branch=main)
[![Crates.io](https://img.shields.io/crates/v/echeneis)](https://crates.io/crates/echeneis)
[![Docs.rs](https://docs.rs/echeneis/badge.svg)](https://docs.rs/echeneis)

# echeneis

> Stopping the world to find world-stoppers

<!-- cargo-rdme start -->

Echeneis is a controlled model-checking tool designed specifically to detect blocking code in concurrent Rust programs. It systematically explores execution paths to verify whether one thread could block another.

### Why echeneis?

Many concurrent algorithms expect a specific sequence of operations to complete without interruption.
If a thread is preempted at a critical operation, such as right after updating a state flag, it can inadvertently leave the rest of the system completely stalled.

In particular, echeneis tests for `obstruction-freedom` of the checked function at each preemption point of the preempted function.
This crate makes no assumption about the memory model, delegating this layer to native atomic implementations.

Focusing on pairwise blocking interactions allows fast and simple checking with informative errors of concurrent models.

### Quick Start

Using `echeneis::check_pairwise`, you can orchestrate an initial state, a preemptible worker thread, and a checking thread to ensure that the checker can always make progress regardless of when the worker is interrupted.

First add echeneis to your dependencies:

```toml
[target.'cfg(echeneis)'.dependencies]
echeneis = "0.1"
```

Next, create a test file and add a test:

```rust
  use echeneis::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
  #[test]
  #[should_panic]
  fn blocking_concurrent() {
    echeneis::check_pairwise(
        || (AtomicBool::new(true), AtomicUsize::new(0)),
        |(b, v)| {
            for _ in 0..3 {
                if v.fetch_add(1, Ordering::AcqRel) == 2 {
                    b.store(false, Ordering::Release);
                    // if we get preempted here, it should block
                    b.store(true, Ordering::Release);
                }
            }
        },
        |(b, _)| {
            while ! b.load(Ordering::Acquire) {}
        },
    );
  }
```

Then, run the test with

```console
RUSTFLAGS="--cfg echeneis" cargo test --test blocking_concurrent
```

### Writing tests

Echeneis intrusively redirects all calls to synchronization primitives in order to check preemption permutations systematically.
This means that all tested functionality should use the synchronization primitives exported in this crate.

One way to do so easily is to use an `echeneis` cfg flag and conditionally use `echeneis` synchronization primitives for test runs by conditionally exporting them in some central module:

```rust
 #[cfg(echeneis)]
 pub(crate) use echeneis::sync::atomic::AtomicUsize;

 #[cfg(not(echeneis))]
 pub(crate) use std::sync::atomic::AtomicUsize;
```
Then, elsewhere:

```rust
 use crate::sync::AtomicUsize;
```

### Limitations

Echeneis currently only supports `check_pairwise`. In other words, blocks depending on n-way scheduling decisions can currently not be detected.

### Feature Flags

- `portable-atomic`: Uses the `portable-atomic` crate as backend for atomics

- `atomic-float`: Enables `portable-atomic` `float` feature and exposes shims for atomic floats.

- `atomic-fallback`: Uses `portable-atomic` `fallback` feature for atomics if necessary.

- `default`:

<!-- cargo-rdme end -->
