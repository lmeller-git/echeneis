use std::thread::yield_now;

use echeneis::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[test]
fn runs() {
    echeneis::check_pairwise(|| {}, |_| {}, |_| std::ops::ControlFlow::Continue(()));
}

#[cfg(not(miri))]
#[test]
fn runs_atomic() {
    echeneis::check_pairwise(
        || AtomicUsize::new(0),
        |v| {
            for i in 0..10 {
                v.fetch_add(i, Ordering::Release);
            }
        },
        |v| {
            let val = v.load(Ordering::Acquire);

            for _ in 0..10 {
                let v2 = v.load(Ordering::Acquire);
                assert!(val == v2);
                yield_now();
            }

            std::ops::ControlFlow::Continue(())
        },
    );
}

#[test]
#[should_panic]
fn panics() {
    echeneis::check_pairwise(|| {}, |_| panic!(), |_| std::ops::ControlFlow::Continue(()));
}

#[test]
#[should_panic]
fn panics2() {
    echeneis::check_pairwise(|| {}, |_| {}, |_| panic!());
}

#[cfg(not(miri))]
#[test]
#[should_panic]
fn kills() {
    #[allow(clippy::infinite_loop)]
    echeneis::check_pairwise(
        || AtomicUsize::new(0),
        |_| {},
        |v| loop {
            v.fetch_add(1, Ordering::Release);
        },
    );
}

#[cfg(not(miri))]
#[test]
#[should_panic]
fn kills2() {
    echeneis::check_pairwise(
        || AtomicBool::new(false),
        |_| {},
        |v| {
            while !v.load(Ordering::Acquire) {}
            std::ops::ControlFlow::Continue(())
        },
    );
}

#[cfg(not(miri))]
#[test]
#[should_panic]
fn kills_block() {
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
            while !b.load(Ordering::Acquire) {}
            std::ops::ControlFlow::Continue(())
        },
    );
}
