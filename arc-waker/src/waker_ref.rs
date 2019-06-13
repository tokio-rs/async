use crate::Wake;
use crate::waker::{clone, wake_by_ref};

use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;
use std::task::{Waker, RawWaker, RawWakerVTable};

#[derive(Debug)]
pub struct WakerRef<'a> {
    waker: Waker,
    _marker: PhantomData<&'a ()>,
}

impl Deref for WakerRef<'_> {
    type Target = Waker;

    fn deref(&self) -> &Waker {
        &self.waker
    }
}

#[inline]
unsafe fn noop(_data: *const ()) {}

unsafe fn wake_unreachable(_data: *const ()) {
    // With only a reference, calling `wake_arc_raw()` would be unsound,
    // since the `WakerRef` didn't increment the refcount of the `ArcWake`,
    // and `wake_arc_raw` would *decrement* it.
    //
    // This should never be reachable, since `WakerRef` only provides a `Deref`
    // to the inner `Waker`.
    //
    // Still, safer to panic here than to call `wake_arc_raw`.
    unreachable!("WakerRef::wake");
}

/// Creates a reference to a [`Waker`](::std::task::Waker)
/// from a local [`ArcWake`].
///
/// The resulting [`Waker`](::std::task::Waker) will call
/// [`ArcWake.wake()`](ArcWake::wake) if awoken.
#[inline]
pub fn waker_ref<W>(wake: &Arc<W>) -> WakerRef<'_>
where
    W: Wake
{
    // This uses the same mechanism as Arc::into_raw, without needing a reference.
    // This is potentially not stable
    let ptr = &*wake as &W as *const W as *const ();

    // Similar to `waker_vtable`, but with a no-op `drop` function.
    // Clones of the resulting `RawWaker` will still be dropped normally.
    let vtable = &RawWakerVTable::new(
        clone::<W>,
        wake_unreachable,
        wake_by_ref::<W>,
        noop,
    );

    let waker = unsafe {
        Waker::from_raw(RawWaker::new(ptr, vtable))
    };

    WakerRef {
        waker,
        _marker: PhantomData,
    }
}
