/// Sealed traits and implementations for `spsc`
pub mod spsc {

    use core::sync::atomic::{AtomicUsize, Ordering};
    #[cfg(feature = "single_core")]
    use core::sync::atomic;


    pub unsafe trait Uxx: Into<usize> + Send {
        #[doc(hidden)]
        unsafe fn load_acquire(x: *const Self) -> Self;

        #[doc(hidden)]
        fn load_relaxed(x: *const Self) -> Self;

        #[doc(hidden)]
        unsafe fn store_release(x: *const Self, val: Self);
    }

    unsafe impl Uxx for usize {
        #[cfg(feature = "single_core")]
        unsafe fn load_acquire(x: *const Self) -> Self {
            let y = (*(x as *const AtomicUsize)).load(Ordering::Relaxed); // read
            atomic::compiler_fence(Ordering::Acquire); // ▼
            y
        }
        #[cfg(not(feature = "single_core"))]
        unsafe fn load_acquire(x: *const Self) -> Self {
            (*(x as *const AtomicUsize)).load(Ordering::Acquire)
        }

        fn load_relaxed(x: *const Self) -> Self {
            unsafe { (*(x as *const AtomicUsize)).load(Ordering::Relaxed) }
        }

        #[cfg(feature = "single_core")]
        unsafe fn store_release(x: *const Self, val: Self) {
            atomic::compiler_fence(Ordering::Release); // ▲
            (*(x as *const AtomicUsize)).store(val, Ordering::Relaxed); // write
        }

        #[cfg(not(feature = "single_core"))]
        unsafe fn store_release(x: *const Self, val: Self) {
            (*(x as *const AtomicUsize)).store(val, Ordering::Release)
        }
    }
}
