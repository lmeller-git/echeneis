pub use crate::core::sync::atomic::Ordering;

macro_rules! atomic_int {
    ($name: ident, $int_type: ident) => {
        #[doc = concat!(
            " Mock implementation of `portable_atomic::` or `std::sync::atomic::` `", stringify!($name), "`"
        )]
        #[derive(Debug)]
        pub struct $name($crate::core::sync::atomic::$name);

        impl $name {
            #[track_caller]
            fn access<R>(&self, f: impl FnOnce(&$crate::core::sync::atomic::$name) -> R) -> R {
                $crate::core::rt::env::yield_current(
                    $crate::core::rt::YieldData::AtomicTransition(
                        Some(std::panic::Location::caller())
                    )
                );
                f(&self.0)
            }

            #[doc = concat!(" Creates a new instance of `", stringify!($name), "`.")]
            #[track_caller]
            pub fn new(v: $int_type) -> Self {
                Self($crate::core::sync::atomic::$name::new(v))
            }

            /// Get access to a mutable reference to the inner value.
            #[track_caller]
            pub fn get_mut(&mut self) -> &mut $int_type {
                // Exclusive mutable borrows do not need an execution checkpoint
                // because no other thread can concurrently observe this state.
                self.0.get_mut()
            }

            /// Consumes the atomic and returns the contained value.
            #[track_caller]
            pub fn into_inner(self) -> $int_type {
                self.0.into_inner()
            }

            /// Loads a value from the atomic integer.
            #[track_caller]
            pub fn load(&self, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.load(order))
            }

            /// Stores a value into the atomic integer.
            #[track_caller]
            pub fn store(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) {
                self.access(|inner| inner.store(val, order))
            }

            /// Stores a value into the atomic integer, returning the previous value.
            #[track_caller]
            pub fn swap(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.swap(val, order))
            }

            #[cfg(not(feature = "portable-atomics"))]
            /// Stores a value into the atomic integer if the current value is the same as the `current` value.
            #[track_caller]
            pub fn compare_and_swap(
                &self,
                current: $int_type,
                new: $int_type,
                order: $crate::core::sync::atomic::Ordering,
            ) -> $int_type {
                #[allow(deprecated)]
                self.access(|inner| inner.compare_and_swap(current, new, order))
            }

            /// Stores a value into the atomic if the current value is the same as the `current` value.
            #[track_caller]
            pub fn compare_exchange(
                &self,
                current: $int_type,
                new: $int_type,
                success: $crate::core::sync::atomic::Ordering,
                failure: $crate::core::sync::atomic::Ordering,
            ) -> Result<$int_type, $int_type> {
                self.access(|inner| inner.compare_exchange(current, new, success, failure))
            }

            /// Stores a value into the atomic if the current value is the same as the current value.
            #[track_caller]
            pub fn compare_exchange_weak(
                &self,
                current: $int_type,
                new: $int_type,
                success: $crate::core::sync::atomic::Ordering,
                failure: $crate::core::sync::atomic::Ordering,
            ) -> Result<$int_type, $int_type> {
                self.compare_exchange(current, new, success, failure)
            }

            /// Adds to the current value, returning the previous value.
            #[track_caller]
            pub fn fetch_add(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.fetch_add(val, order))
            }

            /// Subtracts from the current value, returning the previous value.
            #[track_caller]
            pub fn fetch_sub(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.fetch_sub(val, order))
            }

            /// Bitwise "and" with the current value.
            #[track_caller]
            pub fn fetch_and(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.fetch_and(val, order))
            }

            /// Bitwise "nand" with the current value.
            #[track_caller]
            pub fn fetch_nand(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.fetch_nand(val, order))
            }

            /// Bitwise "or" with the current value.
            #[track_caller]
            pub fn fetch_or(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.fetch_or(val, order))
            }

            /// Bitwise "xor" with the current value.
            #[track_caller]
            pub fn fetch_xor(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.fetch_xor(val, order))
            }

            /// Stores the maximum of the current and provided value, returning the previous value
            #[track_caller]
            pub fn fetch_max(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.fetch_max(val, order))
            }

            /// Stores the minimum of the current and provided value, returning the previous value
            #[track_caller]
            pub fn fetch_min(&self, val: $int_type, order: $crate::core::sync::atomic::Ordering) -> $int_type {
                self.access(|inner| inner.fetch_min(val, order))
            }

            /// Fetches the value, and applies a function to it that returns an optional new value.
            #[track_caller]
            pub fn fetch_update<F>(
                &self,
                set_order: $crate::core::sync::atomic::Ordering,
                fetch_order: $crate::core::sync::atomic::Ordering,
                f: F,
            ) -> Result<$int_type, $int_type>
            where
                F: FnMut($int_type) -> Option<$int_type>,
            {
                self.access(|inner| inner.fetch_update(set_order, fetch_order, f))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new(Default::default())
            }
        }

        impl From<$int_type> for $name {
            fn from(v: $int_type) -> Self {
                Self::new(v)
            }
        }

    };
}

atomic_int!(AtomicU8, u8);
atomic_int!(AtomicU16, u16);
atomic_int!(AtomicU32, u32);
atomic_int!(AtomicUsize, usize);

atomic_int!(AtomicI8, i8);
atomic_int!(AtomicI16, i16);
atomic_int!(AtomicI32, i32);
atomic_int!(AtomicIsize, isize);

#[cfg(any(target_has_atomic = "64", feature = "atomic-fallback"))]
atomic_int!(AtomicU64, u64);

#[cfg(any(target_has_atomic = "64", feature = "atomic-fallback"))]
atomic_int!(AtomicI64, i64);

#[cfg(any(target_has_atomic = "128", feature = "atomic-fallback"))]
atomic_int!(AtomicU128, u128);

#[cfg(any(target_has_atomic = "128", feature = "atomic-fallback"))]
atomic_int!(AtomicI128, i128);

#[cfg(feature = "atomic-float")]
macro_rules! atomic_float {
    ($name: ident, $float_type: ident) => {
        #[doc = concat!(
                                    " Mock implementation of `portable_atomic::", stringify!($name), "`"
                                )]
        #[derive(Debug)]
        pub struct $name($crate::core::sync::atomic::$name);

        impl $name {
            #[track_caller]
            fn access<R>(&self, f: impl FnOnce(&$crate::core::sync::atomic::$name) -> R) -> R {
                $crate::core::rt::env::yield_current(
                    $crate::core::rt::YieldData::AtomicTransition(Some(
                        std::panic::Location::caller(),
                    )),
                );
                f(&self.0)
            }

            #[doc = concat!(" Creates a new instance of `", stringify!($name), "`.")]
            #[track_caller]
            pub fn new(v: $float_type) -> Self {
                Self($crate::core::sync::atomic::$name::new(v))
            }

            /// Consumes the atomic and returns the contained value.
            #[track_caller]
            pub fn into_inner(self) -> $float_type {
                self.0.into_inner()
            }

            /// Loads a value from the atomic integer.
            #[track_caller]
            pub fn load(&self, order: $crate::core::sync::atomic::Ordering) -> $float_type {
                self.access(|inner| inner.load(order))
            }

            /// Stores a value into the atomic integer.
            #[track_caller]
            pub fn store(&self, val: $float_type, order: $crate::core::sync::atomic::Ordering) {
                self.access(|inner| inner.store(val, order))
            }

            /// Get access to a mutable reference to the inner value.
            #[track_caller]
            pub fn get_mut(&mut self) -> &mut $float_type {
                // Exclusive mutable borrows do not need an execution checkpoint
                // because no other thread can concurrently observe this state.
                self.0.get_mut()
            }

            /// Stores a value into the atomic integer, returning the previous value.
            #[track_caller]
            pub fn swap(
                &self,
                val: $float_type,
                order: $crate::core::sync::atomic::Ordering,
            ) -> $float_type {
                self.access(|inner| inner.swap(val, order))
            }

            /// Stores a value into the atomic if the current value is the same as the `current` value.
            #[track_caller]
            pub fn compare_exchange(
                &self,
                current: $float_type,
                new: $float_type,
                success: $crate::core::sync::atomic::Ordering,
                failure: $crate::core::sync::atomic::Ordering,
            ) -> Result<$float_type, $float_type> {
                self.access(|inner| inner.compare_exchange(current, new, success, failure))
            }

            /// Stores a value into the atomic if the current value is the same as the current value.
            #[track_caller]
            pub fn compare_exchange_weak(
                &self,
                current: $float_type,
                new: $float_type,
                success: $crate::core::sync::atomic::Ordering,
                failure: $crate::core::sync::atomic::Ordering,
            ) -> Result<$float_type, $float_type> {
                self.compare_exchange(current, new, success, failure)
            }

            /// Adds to the current value, returning the previous value.
            #[track_caller]
            pub fn fetch_add(
                &self,
                val: $float_type,
                order: $crate::core::sync::atomic::Ordering,
            ) -> $float_type {
                self.access(|inner| inner.fetch_add(val, order))
            }

            /// Subtracts from the current value, returning the previous value.
            #[track_caller]
            pub fn fetch_sub(
                &self,
                val: $float_type,
                order: $crate::core::sync::atomic::Ordering,
            ) -> $float_type {
                self.access(|inner| inner.fetch_sub(val, order))
            }

            /// Stores the maximum of the current and provided value, returning the previous value
            #[track_caller]
            pub fn fetch_max(
                &self,
                val: $float_type,
                order: $crate::core::sync::atomic::Ordering,
            ) -> $float_type {
                self.access(|inner| inner.fetch_max(val, order))
            }

            /// Stores the minimum of the current and provided value, returning the previous value
            #[track_caller]
            pub fn fetch_min(
                &self,
                val: $float_type,
                order: $crate::core::sync::atomic::Ordering,
            ) -> $float_type {
                self.access(|inner| inner.fetch_min(val, order))
            }

            /// Fetches the value, and applies a function to it that returns an optional new value.
            #[track_caller]
            pub fn fetch_update<F>(
                &self,
                set_order: $crate::core::sync::atomic::Ordering,
                fetch_order: $crate::core::sync::atomic::Ordering,
                f: F,
            ) -> Result<$float_type, $float_type>
            where
                F: FnMut($float_type) -> Option<$float_type>,
            {
                self.access(|inner| inner.fetch_update(set_order, fetch_order, f))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new(Default::default())
            }
        }

        impl From<$float_type> for $name {
            fn from(v: $float_type) -> Self {
                Self::new(v)
            }
        }
    };
}

#[cfg(feature = "atomic-float")]
atomic_float!(AtomicF32, f32);
#[cfg(feature = "atomic-float")]
atomic_float!(AtomicF64, f64);

/// Mock implementation of `std::sync::atomic::AtomicBool` or `portable_atomic::AtomicBool`.
#[derive(Debug)]
pub struct AtomicBool(crate::core::sync::atomic::AtomicBool);

impl AtomicBool {
    #[track_caller]
    fn access<R>(&self, f: impl FnOnce(&crate::core::sync::atomic::AtomicBool) -> R) -> R {
        crate::core::rt::env::yield_current(crate::core::rt::YieldData::AtomicTransition(Some(
            std::panic::Location::caller(),
        )));
        f(&self.0)
    }

    /// Creates a new instance of `AtomicBool`.
    pub fn new(v: bool) -> Self {
        Self(crate::core::sync::atomic::AtomicBool::new(v))
    }

    /// Get access to a mutable reference to the inner value.
    pub fn get_mut(&mut self) -> &mut bool {
        self.0.get_mut()
    }

    /// Consumes the atomic and returns the contained value.
    #[track_caller]
    pub fn into_inner(self) -> bool {
        self.0.into_inner()
    }

    /// Loads a value from the atomic bool.
    #[track_caller]
    pub fn load(&self, order: Ordering) -> bool {
        self.access(|inner| inner.load(order))
    }

    /// Stores a value into the atomic bool.
    #[track_caller]
    pub fn store(&self, val: bool, order: Ordering) {
        self.access(|inner| inner.store(val, order));
    }

    /// Stores a value into the atomic bool, returning the previous value.
    #[track_caller]
    pub fn swap(&self, val: bool, order: Ordering) -> bool {
        self.access(|inner| inner.swap(val, order))
    }

    #[cfg(not(feature = "portable-atomics"))]
    /// Stores a value into the atomic bool if the current value is the same as the `current` value.
    #[track_caller]
    pub fn compare_and_swap(&self, current: bool, new: bool, order: Ordering) -> bool {
        #[allow(deprecated)]
        self.access(|inner| inner.compare_and_swap(current, new, order))
    }

    /// Stores a value into the atomic if the current value is the same as the `current` value.
    #[track_caller]
    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        self.access(|inner| inner.compare_exchange(current, new, success, failure))
    }

    /// Stores a value into the atomic if the current value is the same as the current value.
    #[track_caller]
    pub fn compare_exchange_weak(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        self.compare_exchange(current, new, success, failure)
    }

    /// Logical "and" with the current value.
    #[track_caller]
    pub fn fetch_and(&self, val: bool, order: Ordering) -> bool {
        self.access(|inner| inner.fetch_and(val, order))
    }

    /// Logical "nand" with the current value.
    #[track_caller]
    pub fn fetch_nand(&self, val: bool, order: Ordering) -> bool {
        self.access(|inner| inner.fetch_nand(val, order))
    }

    /// Logical "or" with the current value.
    #[track_caller]
    pub fn fetch_or(&self, val: bool, order: Ordering) -> bool {
        self.access(|inner| inner.fetch_or(val, order))
    }

    /// Logical "xor" with the current value.
    #[track_caller]
    pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool {
        self.access(|inner| inner.fetch_xor(val, order))
    }

    /// Fetches the value, and applies a function to it that returns an optional new value.
    #[track_caller]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<bool, bool>
    where
        F: FnMut(bool) -> Option<bool>,
    {
        self.access(|inner| inner.fetch_update(set_order, fetch_order, f))
    }
}

impl Default for AtomicBool {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl From<bool> for AtomicBool {
    fn from(v: bool) -> Self {
        Self::new(v)
    }
}

/// Mock implementation of `std::sync::atomic::AtomicPtr` or `portable_atomic::AtomicPtr`.
pub struct AtomicPtr<T>(crate::core::sync::atomic::AtomicPtr<T>);

impl<T> std::fmt::Debug for AtomicPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> AtomicPtr<T> {
    #[track_caller]
    fn access<R>(&self, f: impl FnOnce(&crate::core::sync::atomic::AtomicPtr<T>) -> R) -> R {
        crate::core::rt::env::yield_current(crate::core::rt::YieldData::AtomicTransition(Some(
            std::panic::Location::caller(),
        )));
        f(&self.0)
    }

    /// Creates a new instance of `AtomicPtr`.
    pub fn new(v: *mut T) -> Self {
        Self(crate::core::sync::atomic::AtomicPtr::new(v))
    }

    #[cfg(not(feature = "portable-atomics"))]
    /// Get access to a mutable reference to the inner value.
    #[track_caller]
    pub fn get_mut(&mut self) -> &mut *mut T {
        self.0.get_mut()
    }

    /// Consumes the atomic and returns the contained value.
    #[track_caller]
    pub fn into_inner(self) -> *mut T {
        self.0.into_inner()
    }

    /// Loads a value from the pointer.
    #[track_caller]
    pub fn load(&self, order: Ordering) -> *mut T {
        self.access(|inner| inner.load(order))
    }

    /// Stores a value into the pointer.
    #[track_caller]
    pub fn store(&self, val: *mut T, order: Ordering) {
        self.access(|inner| inner.store(val, order));
    }

    /// Stores a value into the pointer, returning the previous value.
    #[track_caller]
    pub fn swap(&self, val: *mut T, order: Ordering) -> *mut T {
        self.access(|inner| inner.swap(val, order))
    }

    #[cfg(not(feature = "portable-atomics"))]
    /// Stores a value into the pointer if the current value is the same as the `current` value.
    #[track_caller]
    pub fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T {
        #[allow(deprecated)]
        self.access(|inner| inner.compare_and_swap(current, new, order))
    }

    /// Stores a value into the pointer if the current value is the same as the `current` value.
    #[track_caller]
    pub fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.access(|inner| inner.compare_exchange(current, new, success, failure))
    }

    /// Stores a value into the atomic if the current value is the same as the current value.
    #[track_caller]
    pub fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<*mut T, *mut T> {
        self.compare_exchange(current, new, success, failure)
    }

    /// Fetches the value, and applies a function to it that returns an optional new value.
    #[track_caller]
    pub fn fetch_update<F>(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: F,
    ) -> Result<*mut T, *mut T>
    where
        F: FnMut(*mut T) -> Option<*mut T>,
    {
        self.access(|inner| inner.fetch_update(set_order, fetch_order, f))
    }
}

impl<T> Default for AtomicPtr<T> {
    fn default() -> Self {
        Self::new(std::ptr::null_mut())
    }
}

impl<T> From<*mut T> for AtomicPtr<T> {
    fn from(v: *mut T) -> Self {
        Self::new(v)
    }
}

/// Mock implementation of `std::sync::atomic::fence`.
#[track_caller]
pub fn fence(order: Ordering) {
    crate::core::rt::env::yield_current(crate::core::rt::YieldData::AtomicTransition(Some(
        std::panic::Location::caller(),
    )));
    crate::core::sync::atomic::fence(order);
}
