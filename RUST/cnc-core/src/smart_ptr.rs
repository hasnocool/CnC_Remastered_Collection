//! Smart pointer equivalents (CCPtr, etc.)

use std::ops::{Deref, DerefMut};
use std::fmt;
use std::marker::PhantomData;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

/// Reference counted pointer with custom deleter (like Westwood's CCPtr)
/// Uses atomic reference counting for thread safety
pub struct CCPtr<T> {
    ptr: *mut T,
    ref_count: *mut AtomicUsize,
    deleter: Option<Box<dyn FnOnce(*mut T) + Send + Sync>>,
}

impl<T> CCPtr<T> {
    /// Create new CCPtr with default deleter (Box::drop)
    pub fn new(value: T) -> Self {
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        let ref_count = Box::into_raw(Box::new(AtomicUsize::new(1)));
        Self {
            ptr,
            ref_count,
            deleter: None,
        }
    }

    /// Create with custom deleter
    pub fn with_deleter<F>(value: T, deleter: F) -> Self
    where
        F: FnOnce(*mut T) + Send + Sync + 'static,
    {
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        let ref_count = Box::into_raw(Box::new(AtomicUsize::new(1)));
        Self {
            ptr,
            ref_count,
            deleter: Some(Box::new(deleter)),
        }
    }

    /// Create from raw pointer (assumes ownership)
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            ptr,
            ref_count: Box::into_raw(Box::new(AtomicUsize::new(1))),
            deleter: None,
        }
    }

    /// Get raw pointer
    pub fn as_ptr(&self) -> *const T { self.ptr }

    /// Get mutable raw pointer
    pub fn as_mut_ptr(&self) -> *mut T { self.ptr }

    /// Get reference count
    pub fn strong_count(&self) -> usize {
        unsafe { (*self.ref_count).load(Ordering::Acquire) }
    }

    /// Convert to raw pointer (transfers ownership)
    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }
}

impl<T> Clone for CCPtr<T> {
    fn clone(&self) -> Self {
        unsafe { (*self.ref_count).fetch_add(1, Ordering::AcqRel); }
        Self {
            ptr: self.ptr,
            ref_count: self.ref_count,
            deleter: None, // Only original owns deleter
        }
    }
}

impl<T> Drop for CCPtr<T> {
    fn drop(&mut self) {
        let count = unsafe { (*self.ref_count).fetch_sub(1, Ordering::AcqRel) };
        if count == 1 {
            // Last owner - drop the object
            unsafe {
                if let Some(deleter) = self.deleter.take() {
                    deleter(self.ptr);
                } else {
                    drop(Box::from_raw(self.ptr));
                }
                drop(Box::from_raw(self.ref_count));
            }
        }
    }
}

impl<T> Deref for CCPtr<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.ptr } }
}

impl<T> DerefMut for CCPtr<T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.ptr } }
}

impl<T> fmt::Debug for CCPtr<T> where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CCPtr").field("ptr", &self.ptr).field("count", &self.strong_count()).finish()
    }
}

// For serialization, we serialize as Option<T> (or just the ID if using fixup)
impl<T: Serialize> Serialize for CCPtr<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // Serialize the underlying value
        (**self).serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for CCPtr<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = T::deserialize(deserializer)?;
        Ok(CCPtr::new(value))
    }
}

/// Weak pointer (non-owning reference)
pub struct CCWeak<T> {
    ptr: *mut T,
    ref_count: *mut AtomicUsize,
}

impl<T> CCWeak<T> {
    pub fn new(strong: &CCPtr<T>) -> Self {
        Self { ptr: strong.ptr, ref_count: strong.ref_count }
    }

    pub fn upgrade(&self) -> Option<CCPtr<T>> {
        let count = unsafe { (*self.ref_count).load(Ordering::Acquire) };
        if count == 0 { return None; }
        // Try to increment
        let mut current = count;
        loop {
            if current == 0 { return None; }
            match unsafe { (*self.ref_count).compare_exchange_weak(current, current + 1, Ordering::AcqRel, Ordering::Acquire) } {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
        Some(CCPtr { ptr: self.ptr, ref_count: self.ref_count, deleter: None })
    }
}

impl<T> Clone for CCWeak<T> {
    fn clone(&self) -> Self { Self { ptr: self.ptr, ref_count: self.ref_count } }
}

impl<T> Default for CCWeak<T> {
    fn default() -> Self { Self { ptr: std::ptr::null_mut(), ref_count: std::ptr::null_mut() } }
}

/// NoInit wrapper for placement-new patterns (like Westwood's NoInitClass)
/// Allows creating uninitialized memory that must be initialized before use
pub struct NoInit<T> {
    value: UnsafeCell<std::mem::MaybeUninit<T>>,
    initialized: AtomicUsize, // 0 = uninit, 1 = init
}

impl<T> NoInit<T> {
    pub const fn uninit() -> Self {
        Self {
            value: UnsafeCell::new(std::mem::MaybeUninit::uninit()),
            initialized: AtomicUsize::new(0),
        }
    }

    pub fn init(&self, value: T) -> Result<(), T> {
        // Try to mark as initialized
        match self.initialized.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => {
                unsafe { (*self.value.get()).write(value); }
                Ok(())
            }
            Err(_) => Err(value), // Already initialized
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire) == 1
    }

    pub fn get(&self) -> Option<&T> {
        if self.is_initialized() {
            Some(unsafe { &*(self.value.get() as *const T) })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_initialized() {
            Some(unsafe { &mut *(self.value.get() as *mut T) })
        } else {
            None
        }
    }

    pub unsafe fn assume_init(&self) -> &T {
        &*(self.value.get() as *const T)
    }

    pub unsafe fn assume_init_mut(&mut self) -> &mut T {
        &mut *(self.value.get() as *mut T)
    }
}

impl<T> Drop for NoInit<T> {
    fn drop(&mut self) {
        if self.is_initialized() {
            unsafe { std::ptr::drop_in_place(self.value.get() as *mut T); }
        }
    }
}

/// Unique pointer (like Box but with custom deleter)
pub struct CUPtr<T> {
    ptr: *mut T,
    deleter: Option<Box<dyn FnOnce(*mut T) + Send + Sync>>,
}

impl<T> CUPtr<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::new(value);
        Self { ptr: Box::into_raw(boxed), deleter: None }
    }

    pub fn with_deleter<F>(value: T, deleter: F) -> Self
    where F: FnOnce(*mut T) + Send + Sync + 'static,
    {
        let boxed = Box::new(value);
        Self { ptr: Box::into_raw(boxed), deleter: Some(Box::new(deleter)) }
    }

    pub fn as_ptr(&self) -> *const T { self.ptr }
    pub fn as_mut_ptr(&self) -> *mut T { self.ptr }
    pub fn into_raw(self) -> *mut T { let p = self.ptr; std::mem::forget(self); p }
}

impl<T> Deref for CUPtr<T> { type Target = T; fn deref(&self) -> &T { unsafe { &*self.ptr } } }
impl<T> DerefMut for CUPtr<T> { fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.ptr } } }

impl<T> Drop for CUPtr<T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(deleter) = self.deleter.take() { deleter(self.ptr); }
            else { drop(Box::from_raw(self.ptr)); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_ccptr_basic() {
        let ptr = CCPtr::new(42u32);
        assert_eq!(*ptr, 42);
        assert_eq!(ptr.strong_count(), 1);

        let ptr2 = ptr.clone();
        assert_eq!(ptr.strong_count(), 2);
        assert_eq!(*ptr2, 42);
    }

    #[test] fn test_ccptr_drop() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering};
        let dropped = Arc::new(AtomicBool::new(false));
        let dropped_clone = dropped.clone();
        {
            let _ptr = CCPtr::with_deleter(42u32, move |_| { dropped_clone.store(true, Ordering::SeqCst); });
        }
        assert!(dropped.load(Ordering::SeqCst));
    }

    #[test] fn test_noinit() {
        let noinit = NoInit::<u32>::uninit();
        assert!(!noinit.is_initialized());
        assert!(noinit.get().is_none());
        noinit.init(42).unwrap();
        assert!(noinit.is_initialized());
        assert_eq!(noinit.get(), Some(&42));
        assert!(noinit.init(100).is_err()); // Already initialized
    }
}