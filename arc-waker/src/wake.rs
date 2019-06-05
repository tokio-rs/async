use std::sync::Arc;

/// Wake a pending task
pub trait Wake {
    fn wake(self: Arc<Self>);

    fn wake_by_ref(&self);
}
