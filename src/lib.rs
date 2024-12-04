#![no_std]

use core::{
    fmt::{Debug, Pointer},
    ptr::{self, NonNull},
    sync::atomic::{AtomicPtr, Ordering},
};

/// A wrapper around `core::ptr::mut_ptr` that represents a pointer
/// that is checked for null before accessed.
///
/// MaybeNull is marked as `repr(transparent)`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MaybeNull<T: ?Sized> {
    ptr: *mut T,
}

impl<T> Debug for MaybeNull<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.ptr, f)
    }
}

impl<T> Pointer for MaybeNull<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Pointer::fmt(&self.ptr, f)
    }
}

impl<T> MaybeNull<T> {
    #[inline]
    pub fn new(ptr: *mut T) -> Self {
        Self { ptr }
    }

    /// Look at [`core::ptr::with_exposed_provenance`] for more information.
    #[inline]
    pub fn with_exposed_provenance(addr: usize) -> Self {
        Self::new(ptr::with_exposed_provenance_mut(addr))
    }

    /// Look at [`core::ptr::without_provenance`] for more information.
    #[inline]
    pub fn without_provenance(addr: usize) -> Self {
        Self::new(ptr::without_provenance_mut(addr))
    }

    /// Look at [`core::ptr::null`] for more information.
    #[inline]
    pub fn null() -> Self {
        Self::new(ptr::null_mut())
    }

    /// Look at [`core::ptr::dangling`] for more information.
    #[inline]
    pub fn dangling() -> Self {
        Self::new(ptr::dangling_mut())
    }

    /// Returns `true` if the pointer was null, otherwise returns `false`.
    #[inline]
    pub fn is_null(self) -> bool {
        self.get().is_none()
    }

    /// Sets the pointer to a non-null value.
    #[inline]
    pub fn set(&mut self, value: NonNull<T>) {
        self.ptr = value.as_ptr()
    }

    /// Sets the pointer to null.
    #[inline]
    pub fn nullify(&mut self) {
        *self = Self::null();
    }

    /// Returns `Some` if the pointer is non-null, otherwise return `None`.
    #[inline]
    pub fn get(self) -> Option<NonNull<T>> {
        NonNull::new(self.ptr)
    }

    #[inline]
    pub fn get_unchecked(self) -> *mut T {
        self.ptr
    }

    /// Look at [`ptr::cast`] for more information.
    #[inline]
    pub fn cast<U>(self) -> MaybeNull<U> {
        MaybeNull::new(self.ptr.cast::<U>())
    }

    /// Look at [`ptr::addr`] for more information.
    #[inline]
    pub fn addr(self) -> usize {
        self.ptr.addr()
    }

    /// Look at [`ptr::as_ref`] for more information.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn as_ref<'a>(self) -> Option<&'a T> {
        unsafe { self.ptr.as_ref() }
    }

    /// Look at [`ptr::as_mut`] for more information.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn as_mut<'a>(self) -> Option<&'a mut T> {
        unsafe { self.ptr.as_mut() }
    }

    /// Look at [`ptr::offset`] for more information.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn offset(self, count: isize) -> Self {
        unsafe { Self::new(self.ptr.offset(count)) }
    }

    /// Look at [`ptr::byte_offset`] for more information.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn byte_offset(self, count: isize) -> Self {
        unsafe { Self::new(self.ptr.byte_offset(count)) }
    }

    /// Look at [`ptr::add`] for more information.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn add(self, count: usize) -> Self {
        unsafe { Self::new(self.ptr.add(count)) }
    }

    /// Look at [`ptr::byte_add`] for more information.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn byte_add(self, count: usize) -> Self {
        unsafe { Self::new(self.ptr.byte_add(count)) }
    }

    /// Look at [`ptr::sub`] for more information.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn sub(self, count: usize) -> Self {
        unsafe { Self::new(self.ptr.sub(count)) }
    }

    /// Look at [`ptr::byte_sub`] for more information.
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn byte_sub(self, count: usize) -> Self {
        unsafe { Self::new(self.ptr.byte_sub(count)) }
    }

    /// Look at [`ptr::wrapping_add`] for more information.
    #[inline]
    pub fn wrapping_add(self, count: usize) -> Self {
        Self::new(self.ptr.wrapping_add(count))
    }

    /// Look at [`ptr::wrapping_byte_add`] for more information.
    #[inline]
    pub fn wrapping_byte_add(self, count: usize) -> Self {
        Self::new(self.ptr.wrapping_byte_add(count))
    }

    /// Look at [`ptr::wrapping_sub`] for more information.
    #[inline]
    pub fn wrapping_sub(self, count: usize) -> Self {
        Self::new(self.ptr.wrapping_sub(count))
    }

    /// Look at [`ptr::wrapping_byte_sub`] for more information.
    #[inline]
    pub fn wrapping_byte_sub(self, count: usize) -> Self {
        Self::new(self.ptr.wrapping_byte_sub(count))
    }
}

/// A wrapper around `core::sync::atomic::AtomicPtr` that represents a pointer
/// that is checked for null before accessed.
///
/// AtomicMaybeNull is marked as `repr(transparent)`.
#[repr(transparent)]
pub struct AtomicMaybeNull<T> {
    ptr: AtomicPtr<T>,
}

impl<T> Debug for AtomicMaybeNull<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.ptr, f)
    }
}

impl<T> Pointer for AtomicMaybeNull<T> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Pointer::fmt(&self.ptr, f)
    }
}

impl<T> AtomicMaybeNull<T> {
    #[inline]
    pub fn new(ptr: *mut T) -> Self {
        Self {
            ptr: AtomicPtr::new(ptr),
        }
    }

    /// Look at [`core::ptr::with_exposed_provenance`] for more information.
    #[inline]
    pub fn with_exposed_provenance(addr: usize) -> Self {
        Self::new(ptr::with_exposed_provenance_mut(addr))
    }

    /// Look at [`core::ptr::without_provenance`] for more information.
    #[inline]
    pub fn without_provenance(addr: usize) -> Self {
        Self::new(ptr::without_provenance_mut(addr))
    }

    /// Look at [`core::ptr::null`] for more information.
    #[inline]
    pub fn null() -> Self {
        Self::new(ptr::null_mut())
    }

    /// Look at [`core::ptr::dangling`] for more information.
    #[inline]
    pub fn dangling() -> Self {
        Self::new(ptr::dangling_mut())
    }

    /// Returns `true` if the pointer was null, otherwise returns `false`.
    #[inline]
    pub fn is_null(&self, order: Ordering) -> bool {
        self.get(order).is_none()
    }

    /// Sets the pointer to a non-null value with an atomic ordering of `order`.
    ///
    /// `set` takes an Ordering argument which describes the memory ordering
    /// of this operation. Possible values are SeqCst, Release and Relaxed.
    #[inline]
    pub fn set(&self, value: NonNull<T>, order: Ordering) {
        self.ptr.store(value.as_ptr(), order);
    }

    /// Sets the pointer to null with an atomic ordering of `order`.
    ///
    /// `nullify` takes an Ordering argument which describes the memory ordering
    /// of this operation. Possible values are SeqCst, Release and Relaxed.
    #[inline]
    pub fn nullify(&self, order: Ordering) {
        self.ptr.store(ptr::null_mut(), order);
    }

    /// Returns `Some` if the pointer is non-null, otherwise return `None`.
    ///
    /// `get` takes an Ordering argument which describes the memory ordering
    /// of this operation. Possible values are SeqCst, Release and Relaxed.
    #[inline]
    pub fn get(&self, order: Ordering) -> Option<NonNull<T>> {
        NonNull::new(self.get_unchecked(order))
    }

    /// `get_unchecked` takes an Ordering argument which describes the memory ordering
    /// of this operation. Possible values are SeqCst, Release and Relaxed.
    #[inline]
    pub fn get_unchecked(&self, order: Ordering) -> *mut T {
        self.ptr.load(order)
    }

    /// Look at [`core::sync::atomic::AtomicPtr::swap`] for more information.
    #[inline]
    pub fn swap(&self, other: *mut T, order: Ordering) -> Self {
        Self::new(self.ptr.swap(other, order))
    }

    /// Look at [`core::sync::atomic::AtomicPtr::compare_exchange`] for more information.
    #[inline]
    pub fn compare_exchange(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self, Self> {
        self.ptr
            .compare_exchange(current, new, success, failure)
            .map(Self::new)
            .map_err(Self::new)
    }

    /// Look at [`core::sync::atomic::AtomicPtr::compare_exchange_weak`] for more information.
    #[inline]
    pub fn compare_exchange_weak(
        &self,
        current: *mut T,
        new: *mut T,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self, Self> {
        self.ptr
            .compare_exchange_weak(current, new, success, failure)
            .map(Self::new)
            .map_err(Self::new)
    }

    /// Look at [`core::sync::atomic::AtomicPtr::fetch_update`] for more information.
    #[inline]
    pub fn fetch_update(
        &self,
        set_order: Ordering,
        fetch_order: Ordering,
        f: impl FnMut(*mut T) -> Option<*mut T>,
    ) -> Result<Self, Self> {
        self.ptr
            .fetch_update(set_order, fetch_order, f)
            .map(Self::new)
            .map_err(Self::new)
    }
}
