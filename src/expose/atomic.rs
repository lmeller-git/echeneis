use crate::core::rt::env::yield_current;

pub struct AtomicU64(crate::core::sync::atomic::AtomicU64);

impl AtomicU64 {
    pub fn fetch_add(&self, val: u64, order: crate::core::sync::atomic::Ordering) -> u64 {
        yield_current(crate::core::rt::YieldData::AtomicTransition);

        self.0.fetch_add(val, order)
    }
}
