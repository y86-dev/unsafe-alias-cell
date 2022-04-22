#![doc = include_str!("../README.md")]
#![no_std]
use core::{cell::UnsafeCell, marker::PhantomPinned};

#[doc = include_str!("../README.md")]
#[repr(transparent)]
pub struct UnsafeAliasCell<T: ?Sized> {
    // need to list _pin before inner, so T can be !Sized.
    _pin: PhantomPinned,
    inner: UnsafeCell<T>,
}

impl<T> UnsafeAliasCell<T> {
    /// Construct a new instance of `UnsafeAliasCell` which will wrap the specified value.
    ///
    /// All access to the inner value through methods is `unsafe`.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self {
            inner: UnsafeCell::new(value),
            _pin: PhantomPinned,
        }
    }
}

impl<T: ?Sized> UnsafeAliasCell<T> {
    /// Gets a mutable pointer to the wrapped value.
    ///
    /// The result can be cast to a pointer of any kind. It is the responsibility of the caller to ensure
    /// that the invariants explained on [`UnsafeAliasCell<T>`] are upheld and no UB occurrs.
    #[inline]
    pub const fn get(&self) -> *mut T {
        self.inner.get()
    }

    /// Gets a mutable pointer to the wrapped value without creating temporary references.
    ///
    /// The result can be cast to a pointer of any kind. It is the responsibility of the caller to ensure
    /// that the invariants explained on [`UnsafeAliasCell<T>`] are upheld and no UB occurrs.
    #[inline]
    pub const fn raw_get(this: *const Self) -> *mut T {
        // SAFETY: UAC is repr(transparent)
        UnsafeCell::raw_get(this as *const UnsafeCell<T>)
    }
}
