#![expect(unsafe_code, reason = "This is expected for Unique<>")]

use core::{marker::PhantomData, ptr::NonNull};

/// This is basically `ptr::Unique` except it's not perma-unstable.
/// 
/// The primary motivation for this type is to be `Send` and `Sync` if `T` is `Send` and `Sync`, based on the promise that the `Unique` pointer owns its memory.
/// 
/// [Reference](https://github.com/rust-lang/rust/blob/master/library/core/src/ptr/unique.rs)
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unique<T: ?Sized> {
    pointer: NonNull<T>,
    _marker: PhantomData<T>,
}

// SAFETY: OK because the caller of new() promises that nobody else is using the data pointed to by the pointer
unsafe impl<T: Send + ?Sized> Send for Unique<T> {}

// SAFETY: OK because it has no interior mutability
unsafe impl<T: Sync + ?Sized> Sync for Unique<T> {}

impl<T: ?Sized> Unique<T> {
    /// # Safety
    /// 
    /// Unlike `std`'s `Unique`, I'm making this `unsafe` because you must promise that `NonNull` is pointing to memory that is not aliased by anyone else
    #[must_use]
    pub const unsafe fn new(pointer: NonNull<T>) -> Self {
        Self {
            pointer,
            _marker: PhantomData,
        }
    }
    #[must_use]
    pub const fn get(&self) -> NonNull<T> {
        self.pointer
    }
}