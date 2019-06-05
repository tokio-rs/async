use std::ops::Deref;
use std::pin::Pin;

/// Convert to a `Pin<&T>`.
///
/// The conversion is cheap.
pub trait AsPinRef<T: ?Sized> {
    /// Perform the conversion
    fn as_pin_ref(&self) -> Pin<&T>;
}

impl<T> AsPinRef<T::Target> for Pin<T>
where
    T: Deref,
{
    fn as_pin_ref(&self) -> Pin<&T::Target> {
        self.as_ref()
    }
}

impl<'a, T: Unpin> AsPinRef<T> for &'a T {
    fn as_pin_ref(&self) -> Pin<&T> {
        Pin::new(self)
    }
}
