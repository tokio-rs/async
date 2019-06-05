use std::ops::{Deref, DerefMut};
use std::pin::Pin;

/// Convert to a `Pin<&mut T>`.
///
/// The conversion is cheap.
pub trait AsPinMut<T: ?Sized> {
    /// Perform the conversion
    fn as_pin_mut(&mut self) -> Pin<&mut T>;
}

impl<T> AsPinMut<<T as Deref>::Target> for Pin<T>
where
    T: DerefMut,
{
    fn as_pin_mut(&mut self) -> Pin<&mut <T as Deref>::Target> {
        self.as_mut()
    }
}

impl<'a, T: Unpin> AsPinMut<T> for &'a mut T {
    fn as_pin_mut(&mut self) -> Pin<&mut T> {
        Pin::new(self)
    }
}
