//! Smart pointer equivalents (CCPtr, etc.)

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::UnsafeCell;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Shared control block backing both `CCPtr` (strong) and `CCWeak` handles.
///
/// Mirrors the classic shared-ownership layout so that weak handles keep the
/// block (and the counts) valid even after the managed value is destroyed:
/// - `strong` counts live `CCPtr` handles. When the last `CCPtr` drops, the
///   value is destroyed (custom deleter or `Box` drop), never the block.
/// - `weak` counts live `CCWeak` handles **plus one implicit reference owned
///   by the strong group**. The implicit reference is released by the last
///   `CCPtr` drop; the block itself is freed when `weak` reaches 0, which can
///   only happen after `strong` is 0.
struct ControlBlock<T> {
    strong: AtomicUsize,
    weak: AtomicUsize,
    deleter: Option<Box<dyn FnOnce(*mut T) + Send + Sync>>,
}

impl<T> ControlBlock<T> {
    fn new(deleter: Option<Box<dyn FnOnce(*mut T) + Send + Sync>>) -> *mut Self {
        Box::into_raw(Box::new(ControlBlock {
            strong: AtomicUsize::new(1),
            weak: AtomicUsize::new(1),
            deleter,
        }))
    }
}

/// Reference counted pointer with custom deleter (like Westwood's CCPtr)
/// Uses atomic reference counting for thread safety
pub struct CCPtr<T> {
    ptr: *mut T,
    control: *mut ControlBlock<T>,
}

impl<T> CCPtr<T> {
    /// Create new CCPtr with default deleter (Box::drop)
    pub fn new(value: T) -> Self {
        let ptr = Box::into_raw(Box::new(value));
        Self {
            ptr,
            control: ControlBlock::new(None),
        }
    }

    /// Create with custom deleter
    pub fn with_deleter<F>(value: T, deleter: F) -> Self
    where
        F: FnOnce(*mut T) + Send + Sync + 'static,
    {
        let ptr = Box::into_raw(Box::new(value));
        let control = ControlBlock::new(Some(Box::new(deleter)));
        Self { ptr, control }
    }

    /// Create from raw pointer (assumes ownership)
    ///
    /// # Safety
    ///
    /// The caller must guarantee `ptr` was created via `Box::into_raw` (or
    /// equivalent) and is not owned by any other smart pointer: this `CCPtr`
    /// takes sole ownership and frees it through the default deleter when the
    /// final reference is dropped.
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self {
            ptr,
            control: ControlBlock::new(None),
        }
    }

    /// Get raw pointer
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// Get mutable raw pointer
    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr
    }

    /// Get reference count
    pub fn strong_count(&self) -> usize {
        unsafe { (*self.control).strong.load(Ordering::Acquire) }
    }

    /// Convert to raw pointer (transfers ownership)
    ///
    /// Note: the control block and value are intentionally leaked, matching
    /// the pre-existing semantics of this method; ownership passes to the
    /// caller.
    pub fn into_raw(self) -> *mut T {
        let ptr = self.ptr;
        std::mem::forget(self);
        ptr
    }

    /// Destroy the managed value exactly once (last-strong drop helper)
    ///
    /// # Safety
    ///
    /// Must only be called when the atomically observed previous strong count
    /// was 1, i.e. this is the last live `CCPtr`: it destroys `ptr` through
    /// the stored deleter (or `Box` drop) and releases the implicit weak
    /// reference, freeing the control block when it reaches zero.
    unsafe fn destroy(ptr: *mut T, control: *mut ControlBlock<T>) {
        // SAFETY: the caller guarantees exclusivity (previous strong count
        // was 1); no other handle can observe `ptr` or the block while this
        // runs, because they must acquire the strong count first.
        let block = &mut *control;
        match block.deleter.take() {
            Some(deleter) => deleter(ptr),
            None => drop(Box::from_raw(ptr)),
        }
        // Release the implicit weak reference; the block is freed when the
        // last weak handle (or this final strong-owner reference) goes away.
        if block.weak.fetch_sub(1, Ordering::AcqRel) == 1 {
            drop(Box::from_raw(control));
        }
    }
}

impl<T> Clone for CCPtr<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.control).strong.fetch_add(1, Ordering::AcqRel);
        }
        Self {
            ptr: self.ptr,
            control: self.control,
        }
    }
}

impl<T> Drop for CCPtr<T> {
    fn drop(&mut self) {
        let count = unsafe { (*self.control).strong.fetch_sub(1, Ordering::AcqRel) };
        if count == 1 {
            // Last owner: destroy the value, keep the control block until
            // the last weak handle has been dropped.
            unsafe { CCPtr::destroy(self.ptr, self.control) };
        }
    }
}

impl<T> Deref for CCPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for CCPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> fmt::Debug for CCPtr<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CCPtr")
            .field("ptr", &self.ptr)
            .field("count", &self.strong_count())
            .finish()
    }
}

// For serialization, we serialize as Option<T> (or just the ID if using fixup)
impl<T: Serialize> Serialize for CCPtr<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the underlying value
        (**self).serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for CCPtr<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(CCPtr::new(value))
    }
}

/// Weak pointer (non-owning reference)
///
/// Holds a weak count on the shared [`ControlBlock`] so the block survives
/// after every `CCPtr` has been dropped; `upgrade` then safely reports `None`
/// (the value itself is already destroyed by the last strong drop).
pub struct CCWeak<T> {
    ptr: *mut T,
    control: *mut ControlBlock<T>,
}

impl<T> CCWeak<T> {
    pub fn new(strong: &CCPtr<T>) -> Self {
        // Register on the control block so it outlives the strong group.
        unsafe { (*strong.control).weak.fetch_add(1, Ordering::AcqRel) };
        Self {
            ptr: strong.ptr,
            control: strong.control,
        }
    }

    pub fn upgrade(&self) -> Option<CCPtr<T>> {
        if self.ptr.is_null() {
            return None;
        }
        // CAS loop on the strong count; a null/zero strong count means the
        // value has been destroyed and must never be resurrected.
        let mut current = unsafe { (*self.control).strong.load(Ordering::Acquire) };
        loop {
            if current == 0 {
                return None;
            }
            match unsafe {
                (*self.control).strong.compare_exchange_weak(
                    current,
                    current + 1,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                )
            } {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
        Some(CCPtr {
            ptr: self.ptr,
            control: self.control,
        })
    }
}

impl<T> Clone for CCWeak<T> {
    fn clone(&self) -> Self {
        if !self.ptr.is_null() {
            unsafe { (*self.control).weak.fetch_add(1, Ordering::AcqRel) };
        }
        Self {
            ptr: self.ptr,
            control: self.control,
        }
    }
}

impl<T> Drop for CCWeak<T> {
    fn drop(&mut self) {
        if self.ptr.is_null() {
            // Default-constructed weak: nothing to release.
            return;
        }
        let remaining = unsafe { (*self.control).weak.fetch_sub(1, Ordering::AcqRel) };
        if remaining == 1 {
            // Last weak handle: the strong group must already be gone (the
            // implicit weak reference was released with the last `CCPtr`),
            // so reclaim the control block.
            unsafe { drop(Box::from_raw(self.control)) };
        }
    }
}

impl<T> Default for CCWeak<T> {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            control: std::ptr::null_mut(),
        }
    }
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
        match self
            .initialized
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => {
                unsafe {
                    (*self.value.get()).write(value);
                }
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

    /// Assume the value is initialized.
    ///
    /// # Safety
    ///
    /// Must only be called after `init` has returned `Ok`, i.e. once the
    /// internal initialized flag has transitioned to 1; otherwise the returned
    /// reference exposes uninitialized memory.
    pub unsafe fn assume_init(&self) -> &T {
        &*(self.value.get() as *const T)
    }

    /// Assume the value is initialized and get a mutable reference.
    ///
    /// # Safety
    ///
    /// Must only be called after `init` has returned `Ok`; the caller must
    /// additionally hold the only mutable access to this `NoInit` (exclusivity
    /// is normally guaranteed by `&mut self`).
    pub unsafe fn assume_init_mut(&mut self) -> &mut T {
        &mut *(self.value.get() as *mut T)
    }
}

impl<T> Drop for NoInit<T> {
    fn drop(&mut self) {
        if self.is_initialized() {
            unsafe {
                std::ptr::drop_in_place(self.value.get() as *mut T);
            }
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
        Self {
            ptr: Box::into_raw(boxed),
            deleter: None,
        }
    }

    pub fn with_deleter<F>(value: T, deleter: F) -> Self
    where
        F: FnOnce(*mut T) + Send + Sync + 'static,
    {
        let boxed = Box::new(value);
        Self {
            ptr: Box::into_raw(boxed),
            deleter: Some(Box::new(deleter)),
        }
    }

    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }
    pub fn as_mut_ptr(&self) -> *mut T {
        self.ptr
    }
    pub fn into_raw(self) -> *mut T {
        let p = self.ptr;
        std::mem::forget(self);
        p
    }
}

impl<T> Deref for CUPtr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}
impl<T> DerefMut for CUPtr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for CUPtr<T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(deleter) = self.deleter.take() {
                deleter(self.ptr);
            } else {
                drop(Box::from_raw(self.ptr));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ccptr_basic() {
        let ptr = CCPtr::new(42u32);
        assert_eq!(*ptr, 42);
        assert_eq!(ptr.strong_count(), 1);

        let ptr2 = ptr.clone();
        assert_eq!(ptr.strong_count(), 2);
        assert_eq!(*ptr2, 42);
    }

    #[test]
    fn test_ccptr_drop() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        let dropped = Arc::new(AtomicBool::new(false));
        let dropped_clone = dropped.clone();
        {
            let _ptr = CCPtr::with_deleter(42u32, move |_| {
                dropped_clone.store(true, Ordering::SeqCst);
            });
        }
        assert!(dropped.load(Ordering::SeqCst));
    }

    #[test]
    fn test_noinit() {
        let noinit = NoInit::<u32>::uninit();
        assert!(!noinit.is_initialized());
        assert!(noinit.get().is_none());
        noinit.init(42).unwrap();
        assert!(noinit.is_initialized());
        assert_eq!(noinit.get(), Some(&42));
        assert!(noinit.init(100).is_err()); // Already initialized
    }

    #[test]
    fn test_ccptr_clone_counts_and_shared_value() {
        let base = CCPtr::new(vec![1u8, 2, 3]);
        assert_eq!(base.strong_count(), 1);
        let c1 = base.clone();
        assert_eq!(base.strong_count(), 2, "clone must increment the count");
        assert_eq!(c1.strong_count(), 2);
        let c2 = base.clone();
        assert_eq!(base.strong_count(), 3);
        assert_eq!(*c2, vec![1, 2, 3], "clones must view the same value");
        drop(c1);
        assert_eq!(base.strong_count(), 2, "dropping a clone must decrement");
        drop(c2);
        assert_eq!(base.strong_count(), 1);
        assert_eq!(base.len(), 3);
    }

    #[test]
    fn test_ccweak_upgrade_while_strong_alive() {
        let strong = CCPtr::new(7u32);
        let weak = CCWeak::new(&strong);
        let weak_clone = weak.clone();
        assert_eq!(
            strong.strong_count(),
            1,
            "weak refs must not affect the strong count"
        );
        let upgraded = weak_clone.upgrade().unwrap();
        assert_eq!(*upgraded, 7);
        assert_eq!(strong.strong_count(), 2, "upgrade must increment");
        drop(upgraded);
        assert_eq!(strong.strong_count(), 1);
    }

    #[test]
    fn test_noinit_double_init_err_carries_value() {
        let slot = NoInit::<u32>::uninit();
        slot.init(42).unwrap();
        let err = slot.init(100).unwrap_err();
        assert_eq!(err, 100, "second init must return the rejected value");
        assert_eq!(slot.get(), Some(&42), "first value must be preserved");
    }

    #[test]
    fn test_noinit_get_mut() {
        let mut slot = NoInit::<u32>::uninit();
        assert!(slot.get_mut().is_none());
        slot.init(5).unwrap();
        if let Some(v) = slot.get_mut() {
            *v += 1;
        }
        assert_eq!(slot.get(), Some(&6));
    }

    #[test]
    fn test_ccweak_survives_strong_drop_then_frees_block() {
        // The soundness regression: a weak handle created before every strong
        // handle drops must report `None` (not touch freed refcount memory)
        // and, together with the control block, release cleanly.
        let strong = CCPtr::new(11u32);
        let weak = CCWeak::new(&strong);
        let weak_clone = weak.clone();
        drop(strong);
        assert!(weak.upgrade().is_none(), "dead weak must not resurrect");
        assert!(weak_clone.upgrade().is_none());
        drop(weak);
        drop(weak_clone);
        // Dropping the last weak reclaims the control block; running without
        // crash/leak is asserted by the test harness (valgrind when present).
    }

    #[test]
    fn test_ccweak_lifecycle_stress() {
        // Reuse allocator slots aggressively: under the old design the lone
        // refcount box was freed on last strong drop while CCWeak still held
        // its raw pointer, so upgrade() scribbled on freed memory (SIGABRT).
        for _ in 0..1_000 {
            let strong = CCPtr::new(0u64);
            let weak = CCWeak::new(&strong);
            drop(strong);
            assert!(weak.upgrade().is_none());
            drop(weak);

            // Opposite order: weak dropped before the strong owner, block
            // reclaimed by the last strong drop.
            let strong = CCPtr::new(1u64);
            let weak = CCWeak::new(&strong);
            let weak2 = weak.clone();
            drop(weak);
            drop(weak2);
            drop(strong);
        }
    }

    #[test]
    fn test_ccweak_upgrade_after_partial_strong_drop() {
        let strong = CCPtr::new(String::from("alive"));
        let weak = CCWeak::new(&strong);
        let first = weak.upgrade().expect("upgrade while strong alive");
        drop(strong);
        assert_eq!(*first, "alive", "upgraded strong keeps the value alive");
        drop(first);
        assert!(weak.upgrade().is_none());
        drop(weak);
    }

    #[test]
    fn test_ccptr_deleter_runs_independent_of_drop_order() {
        // The deleter now lives on the shared control block, so it must run
        // when the *last* strong handle drops, whichever handle that is.
        use std::sync::atomic::AtomicBool;
        use std::sync::Arc;
        let flag = Arc::new(AtomicBool::new(false));
        let deleter_flag = Arc::clone(&flag);
        let original = CCPtr::with_deleter(1u32, move |_| {
            deleter_flag.store(true, Ordering::SeqCst);
        });
        let clone = original.clone();
        drop(original);
        assert!(!flag.load(Ordering::SeqCst), "clones keep the value alive");
        drop(clone);
        assert!(
            flag.load(Ordering::SeqCst),
            "deleter must run at last strong"
        );
    }

    #[test]
    fn test_cuptr_with_deleter_runs_deleter_on_drop() {
        use std::sync::atomic::AtomicBool;
        use std::sync::Arc;
        let flag = Arc::new(AtomicBool::new(false));
        let deleter_flag = Arc::clone(&flag);
        {
            let ptr = CUPtr::with_deleter(9u32, move |p| {
                deleter_flag.store(true, Ordering::SeqCst);
                drop(unsafe { Box::from_raw(p) });
            });
            assert_eq!(*ptr, 9);
            assert!(!flag.load(Ordering::SeqCst));
        }
        assert!(
            flag.load(Ordering::SeqCst),
            "CUPtr::with_deleter must invoke the deleter on drop"
        );
    }

    #[test]
    fn test_cuptr_new_drops_value_on_drop() {
        use std::sync::atomic::AtomicBool;
        use std::sync::Arc;
        struct FlagDrop {
            flag: Arc<AtomicBool>,
        }
        impl Drop for FlagDrop {
            fn drop(&mut self) {
                self.flag.store(true, Ordering::SeqCst);
            }
        }
        let flag = Arc::new(AtomicBool::new(false));
        {
            let _ptr = CUPtr::new(FlagDrop {
                flag: Arc::clone(&flag),
            });
        }
        assert!(
            flag.load(Ordering::SeqCst),
            "CUPtr::new must drop the value"
        );
    }
}
