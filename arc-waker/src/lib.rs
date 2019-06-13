//! Implement `Waker` values using `Arc` for ref counting.

mod wake;
mod waker;
mod waker_ref;

pub use crate::wake::Wake;
pub use crate::waker::waker;
pub use crate::waker_ref::{waker_ref, WakerRef};
