use crate::Wake;

use std::mem;
use std::sync::Arc;
use std::task::{RawWaker, RawWakerVTable, Waker};

pub fn waker<W: Wake>(waker: Arc<W>) -> Waker {
    let data = Arc::into_raw(waker) as *const ();

    let raw = RawWaker::new(data, vtable::<W>());
    unsafe { Waker::from_raw(raw) }
}

fn vtable<T: Wake>() -> &'static RawWakerVTable {
    &RawWakerVTable::new(clone::<T>, wake::<T>, wake_by_ref::<T>, drop::<T>)
}

pub(crate) unsafe fn clone<T: Wake>(raw: *const ()) -> RawWaker {
    let wake = Arc::from_raw(raw as *const T);

    mem::forget(wake.clone());
    mem::forget(wake);

    RawWaker::new(raw, vtable::<T>())
}

unsafe fn wake<T: Wake>(raw: *const ()) {
    let wake = Arc::from_raw(raw as *const T);
    wake.wake();
}

pub(crate) unsafe fn wake_by_ref<T: Wake>(raw: *const ()) {
    let wake = Arc::from_raw(raw as *const T);
    wake.wake_by_ref();

    mem::forget(wake);
}

unsafe fn drop<T: Wake>(raw: *const ()) {
    let _ = Arc::from_raw(raw as *const T);
}
