#![no_std]

mod private {
    use core::{cell::UnsafeCell, marker::PhantomPinned};
    #[repr(transparent)]
    pub struct UnsafeAliasCell<T: ?Sized> {
        // need to list _pin before inner, so T can be !Sized.
        _pin: PhantomPinned,
        inner: UnsafeCell<T>,
    }

    impl<T> UnsafeAliasCell<T> {
        pub const fn new(value: T) -> Self {
            Self {
                inner: UnsafeCell::new(value),
                _pin: PhantomPinned,
            }
        }
    }

    impl<T: ?Sized> UnsafeAliasCell<T> {
        pub const fn get(&self) -> *mut T {
            self.inner.get()
        }

        pub const fn raw_get(this: *const Self) -> *mut T {
            // SAFETY: UAC is repr(transparent)
            UnsafeCell::raw_get(this as *const UnsafeCell<T>)
        }
    }
}
