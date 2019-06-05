//! Implement `Waker` values using `Arc` for ref counting.

use std::sync::Arc;

use std::mem;
use std::task::{RawWaker, RawWakerVTable, Waker};

/// Wake a pending task
pub trait Wake: Send + Sync + Sized {
    /// Wake up the task associated with this `Wake` value.
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    /// Wake up the task associated with this `Wake` value without consuming `self`.
    fn wake_by_ref(&self);

    /// Convert into a `Waker`.
    fn into_waker(self: Arc<Self>) -> Waker {
        let data = Arc::into_raw(self) as *const ();

        let raw = RawWaker::new(data, vtable::<Self>());
        unsafe { Waker::from_raw(raw) }
    }
}

fn vtable<T: Wake>() -> &'static RawWakerVTable {
    &RawWakerVTable::new(clone::<T>, wake::<T>, wake_by_ref::<T>, drop::<T>)
}

unsafe fn clone<T: Wake>(raw: *const ()) -> RawWaker {
    let wake = Arc::from_raw(raw as *const T);

    mem::forget(wake.clone());
    mem::forget(wake);

    RawWaker::new(raw, vtable::<T>())
}

unsafe fn wake<T: Wake>(raw: *const ()) {
    let wake = Arc::from_raw(raw as *const T);
    wake.wake();
}

unsafe fn wake_by_ref<T: Wake>(raw: *const ()) {
    let wake = Arc::from_raw(raw as *const T);
    wake.wake_by_ref();

    mem::forget(wake);
}

unsafe fn drop<T: Wake>(raw: *const ()) {
    let _ = Arc::from_raw(raw as *const T);
}
