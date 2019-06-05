#![doc(html_root_url = "https://docs.rs/pin-convert/0.1.0")]
#![deny(missing_debug_implementations, missing_docs, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

//! Traits for converting to `Pin` variants.
//!
//! Similar to traits found in `std::convert`, but yield `Pin` values.

mod as_pin_mut;
mod as_pin_ref;

pub use crate::as_pin_mut::AsPinMut;
pub use crate::as_pin_ref::AsPinRef;
