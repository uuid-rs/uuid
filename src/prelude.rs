//! The [`uuid`] prelude.
//!
//! This module contains the most important items of the [`uuid`] crate.
//!
//! To use the prelude, include the following in your crate root:
//!
//! ```rust
//! extern crate uuid;
//! ```
//!
//! and the following in every module:
//!
//! ```rust
//! use uuid::prelude::*;
//! ```
//!
//! # Prelude Contents
//!
//! Currently the prelude reexports the following:
//!
//! [`uuid`]`::{`[`Uuid`], [`UuidVariant`]`}`: The fundamental types used in
//! [`uuid`] crate.
//!
//! [`uuid`]: ../index.html
//! [`Uuid`]: ../struct.Uuid.html
//! [`UuidVariant`]: enum.UuidVariant.html
//!
#![cfg_attr(feature = "v1",
doc = "
[`uuid::v1`]`::{`[`UuidClockSequence`],[`UuidContext`]`}`: The types useful for
handling uuid version 1. Requires feature `v1`.

[`uuid::v1`]: ../v1/index.html
[`UuidContext`]: ../v1/struct.UuidContext.html
[`UuidClockSequence`]: ../v1/trait.UuidClockSequence.html")]

#[doc(inline)]
pub use super::{Uuid, UuidVariant};

cfg_if! {
    if #[cfg(feature = "v1")] {
        #[doc(inline)]
        pub use super::v1::{UuidClockSequence, UuidContext};
    }
}
