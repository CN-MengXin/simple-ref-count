use std::sync::atomic::{AtomicUsize, Ordering};
pub struct RefCount {
    ptr: *mut usize,
}
impl RefCount {
    pub fn new() -> Self {
        Self {
            ptr: Box::into_raw(Box::new(1)),
        }
    }
    pub fn is(&self, rc: &Self) -> bool {
        self.ptr == rc.ptr
    }
    pub fn count(&self) -> usize {
        unsafe { *self.ptr }
    }
}
impl Drop for RefCount {
    fn drop(&mut self) {
        unsafe {
            if *self.ptr == 1 {
                drop(Box::from_raw(self.ptr))
            } else {
                *self.ptr -= 1
            }
        }
    }
}
impl Clone for RefCount {
    fn clone(&self) -> Self {
        unsafe { *self.ptr += 1 };
        Self { ptr: self.ptr }
    }
}
pub struct AtomicRefCount {
    ptr: *mut AtomicUsize,
}
impl AtomicRefCount {
    pub fn new() -> Self {
        Self {
            ptr: Box::into_raw(Box::new(1.into())),
        }
    }
    pub fn is(&self, rc: &Self) -> bool {
        self.ptr == rc.ptr
    }
    pub fn count(&self) -> usize {
        unsafe { (*self.ptr).load(Ordering::Relaxed) }
    }
}
impl Drop for AtomicRefCount {
    fn drop(&mut self) {
        unsafe {
            if (*self.ptr).fetch_sub(1, Ordering::Relaxed) == 1 {
                drop(Box::from_raw(self.ptr))
            }
        }
    }
}
impl Clone for AtomicRefCount {
    fn clone(&self) -> Self {
        unsafe { (*self.ptr).fetch_add(1, Ordering::Relaxed) };
        Self { ptr: self.ptr }
    }
}
unsafe impl Sync for AtomicRefCount {}
unsafe impl Send for AtomicRefCount {}
