use std::sync::Arc;

pub trait Wake: Send + Sync + Sized {
    /// Wake up the task associated with this `Wake` value.
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    /// Wake up the task associated with this `Wake` value without consuming `self`.
    fn wake_by_ref(&self);
}
