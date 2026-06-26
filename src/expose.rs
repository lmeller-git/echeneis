mod atomic;

pub mod sync {
    //! Mock implementations of `std::sync`.
    use super::atomic as raw_atomic;
    pub mod atomic {
        //! Mock implementations for `std::sync::atomic` or `portable_atomic`.
        pub use super::raw_atomic::*;
    }
}
