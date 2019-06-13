use arc_waker::Wake;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

#[derive(Debug, Default)]
struct MyWaker {
    woke: AtomicBool,
}

impl Wake for MyWaker {
    fn wake_by_ref(&self) {
        self.woke.store(true, Relaxed);
    }
}

fn waker() -> Arc<MyWaker> {
    Arc::new(MyWaker::default())
}

#[test]
fn ref_inc_dec() {
    let my_waker = waker();
    let waker = arc_waker::waker_ref(&my_waker);

    assert_eq!(1, Arc::strong_count(&my_waker));

    let waker2 = waker.clone();

    assert_eq!(2, Arc::strong_count(&my_waker));

    drop(waker);

    assert_eq!(2, Arc::strong_count(&my_waker));

    drop(waker2);

    assert_eq!(1, Arc::strong_count(&my_waker));
}

#[test]
fn wake_by_ref() {
    let my_waker = waker();
    let waker = arc_waker::waker_ref(&my_waker);

    waker.wake_by_ref();
    assert!(my_waker.woke.load(Relaxed));
}
