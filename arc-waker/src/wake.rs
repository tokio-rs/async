use std::sync::Arc;

pub trait Wake: Send + Sync + Sized {
    /// Wake up the task associated with this `Wake` value.
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self);
    }

    /// Wake up the task associated with this `Wake` value without consuming `self`.
    fn wake_by_ref(me: &Arc<Self>);
}
