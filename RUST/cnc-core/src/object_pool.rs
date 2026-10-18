//! Object pool for hot allocations (like Westwood's object pools)

use std::alloc::{Layout, alloc, dealloc};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::marker::PhantomData;
use parking_lot::Mutex;
use thiserror::Error;

/// Error type for object pool allocation failures
#[derive(Debug, Error)]
pub enum PoolError {
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Invalid layout")]
    InvalidLayout,
}

impl From<std::alloc::LayoutError> for PoolError {
    fn from(_: std::alloc::LayoutError) -> Self {
        PoolError::InvalidLayout
    }
}

/// Object pool for fast allocation/deallocation of same-sized objects
pub struct ObjectPool<T> {
    /// Free list (linked list of available slots)
    free_head: AtomicUsize,
    /// All allocated blocks
    blocks: Mutex<Vec<Block>>,
    /// Object size (including padding)
    object_size: usize,
    /// Objects per block
    objects_per_block: usize,
    /// Total allocated
    total_allocated: AtomicUsize,
    /// Total freed
    total_freed: AtomicUsize,
    _phantom: PhantomData<T>,
}

struct Block {
    ptr: NonNull<u8>,
    layout: Layout,
    /// Bitmap of free slots (1 = free, 0 = used)
    free_bitmap: Vec<u64>,
}

// The block only owns a raw allocation; ownership moves with the pool.
unsafe impl Send for Block {}

impl<T> ObjectPool<T> {
    /// Create new object pool
    /// objects_per_block: number of objects per allocation block (default 256)
    pub fn new(objects_per_block: usize) -> Self {
        // Each slot must hold either T or the free-list usize next pointer, so
        // its size and alignment are the max of both. This keeps the free list
        // correctly aligned for any T (notably small types such as u32).
        let align = std::mem::align_of::<T>().max(std::mem::align_of::<usize>());
        let raw_size = std::mem::size_of::<T>().max(std::mem::size_of::<usize>());
        let object_size = (raw_size + align - 1) & !(align - 1);

        Self {
            free_head: AtomicUsize::new(usize::MAX),
            blocks: Mutex::new(Vec::new()),
            object_size,
            objects_per_block,
            total_allocated: AtomicUsize::new(0),
            total_freed: AtomicUsize::new(0),
            _phantom: PhantomData,
        }
    }

    /// Allocate a new object (uninitialized)
    pub fn allocate(&self) -> Result<*mut T, PoolError> {
        // Try to get from free list
        let mut head = self.free_head.load(Ordering::Acquire);
        loop {
            if head != usize::MAX {
                // Try to pop from free list
                let next = unsafe { *(head as *mut usize) };
                if self.free_head.compare_exchange_weak(head, next, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    self.total_allocated.fetch_add(1, Ordering::Relaxed);
                    return Ok(head as *mut T);
                }
            } else {
                break;
            }
        }

        // Free list empty, allocate new block
        self.allocate_block()?;
        
        // Retry
        let mut head = self.free_head.load(Ordering::Acquire);
        let next = unsafe { *(head as *mut usize) };
        if self.free_head.compare_exchange(head, next, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.total_allocated.fetch_add(1, Ordering::Relaxed);
            return Ok(head as *mut T);
        }

        // Should not happen
        Err(PoolError::OutOfMemory)
    }

    /// Allocate and initialize with value
    pub fn allocate_init(&self, value: T) -> Result<*mut T, PoolError> {
        let ptr = self.allocate()?;
        unsafe { ptr.write(value); }
        Ok(ptr)
    }

    /// Deallocate an object
    pub fn deallocate(&self, ptr: *mut T) {
        if ptr.is_null() { return; }

        // Add to free list
        let mut head = self.free_head.load(Ordering::Acquire);
        loop {
            unsafe { *(ptr as *mut usize) = head; }
            if self.free_head.compare_exchange_weak(head, ptr as usize, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                self.total_freed.fetch_add(1, Ordering::Relaxed);
                return;
            }
        }
    }

    /// Deallocate and run destructor
    pub fn deallocate_drop(&self, ptr: *mut T) {
        if ptr.is_null() { return; }
        unsafe { std::ptr::drop_in_place(ptr); }
        self.deallocate(ptr);
    }

    fn allocate_block(&self) -> Result<(), PoolError> {
        let align = std::mem::align_of::<T>().max(std::mem::align_of::<usize>());
        let layout = Layout::from_size_align(
            self.object_size * self.objects_per_block,
            align,
        )?;

        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() { return Err(PoolError::OutOfMemory); }

        let block = Block {
            ptr: NonNull::new(ptr).unwrap(),
            layout,
            free_bitmap: vec![u64::MAX; (self.objects_per_block + 63) / 64],
        };

        // Initialize free list for this block
        let base = ptr as usize;
        for i in (0..self.objects_per_block).rev() {
            let obj_ptr = base + i * self.object_size;
            unsafe { *(obj_ptr as *mut usize) = self.free_head.load(Ordering::Relaxed); }
            self.free_head.store(obj_ptr, Ordering::Release);
        }

        // Keep ownership of the block so it is released in Drop.
        self.blocks.lock().push(block);
        Ok(())
    }

    /// Get statistics
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            total_allocated: self.total_allocated.load(Ordering::Relaxed),
            total_freed: self.total_freed.load(Ordering::Relaxed),
            current_count: self.total_allocated.load(Ordering::Relaxed) - self.total_freed.load(Ordering::Relaxed),
            blocks: self.blocks.lock().len(),
            object_size: self.object_size,
        }
    }
}

impl<T> Default for ObjectPool<T> {
    fn default() -> Self { Self::new(256) }
}

impl<T> Drop for ObjectPool<T> {
    fn drop(&mut self) {
        for block in self.blocks.get_mut() {
            unsafe { dealloc(block.ptr.as_ptr(), block.layout); }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub current_count: usize,
    pub blocks: usize,
    pub object_size: usize,
}

/// Pool handle for RAII deallocation
pub struct PoolHandle<T> {
    ptr: *mut T,
    pool: *const ObjectPool<T>,
}

impl<T> PoolHandle<T> {
    pub fn new(ptr: *mut T, pool: &ObjectPool<T>) -> Self {
        Self { ptr, pool }
    }

    pub fn ptr(&self) -> *mut T { self.ptr }
    pub fn as_ref(&self) -> Option<&T> { if self.ptr.is_null() { None } else { Some(unsafe { &*self.ptr }) } }
    pub fn as_mut(&mut self) -> Option<&mut T> { if self.ptr.is_null() { None } else { Some(unsafe { &mut *self.ptr }) } }
}

impl<T> Deref for PoolHandle<T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.ptr } }
}

impl<T> DerefMut for PoolHandle<T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.ptr } }
}

impl<T> Drop for PoolHandle<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { std::ptr::drop_in_place(self.ptr); }
            unsafe { (*self.pool).deallocate(self.ptr); }
        }
    }
}

/// Thread-local object pool (for per-thread allocation)
pub struct ThreadLocalPool<T> {
    pool: ObjectPool<T>,
}

impl<T> ThreadLocalPool<T> {
    pub fn new(objects_per_block: usize) -> Self {
        Self { pool: ObjectPool::new(objects_per_block) }
    }

    pub fn allocate(&self) -> Result<*mut T, PoolError> {
        self.pool.allocate()
    }

    pub fn allocate_init(&self, value: T) -> Result<*mut T, PoolError> {
        self.pool.allocate_init(value)
    }
    pub fn deallocate(&self, ptr: *mut T) { self.pool.deallocate(ptr); }
    pub fn deallocate_drop(&self, ptr: *mut T) { self.pool.deallocate_drop(ptr); }
    pub fn stats(&self) -> PoolStats { self.pool.stats() }
}

use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn test_pool_basic() {
        let pool = ObjectPool::<u32>::new(16);
        let ptr1 = pool.allocate_init(42).unwrap();
        let ptr2 = pool.allocate_init(100).unwrap();

        assert_eq!(unsafe { *ptr1 }, 42);
        assert_eq!(unsafe { *ptr2 }, 100);

        pool.deallocate_drop(ptr1);
        pool.deallocate_drop(ptr2);

        let stats = pool.stats();
        assert_eq!(stats.total_allocated, 2);
        assert_eq!(stats.total_freed, 2);
        assert_eq!(stats.current_count, 0);
    }

    #[test] fn test_pool_reuse() {
        let pool = ObjectPool::<u32>::new(16);
        let ptr1 = pool.allocate_init(1).unwrap();
        pool.deallocate_drop(ptr1);

        let ptr2 = pool.allocate_init(2).unwrap();
        // Should reuse the same memory
        assert_eq!(ptr1, ptr2);
        assert_eq!(unsafe { *ptr2 }, 2);

        pool.deallocate_drop(ptr2);
    }

    #[test] fn test_pool_handle() {
        let pool = ObjectPool::<u32>::new(16);
        let handle = PoolHandle::new(pool.allocate_init(42).unwrap(), &pool);
        assert_eq!(*handle, 42);
        // Automatically deallocated on drop
    }
}
