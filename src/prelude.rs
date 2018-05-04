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
//! ## The core types
//!
//! [`uuid`]`::{`[`Uuid`], [`UuidVariant`]`}`: The fundamental
//! types used in [`uuid`] crate.
//!
//! [`uuid`]: ../index.html
//! [`Uuid`]: ../struct.Uuid.html
//! [`UuidVariant`]: enum.UuidVariant.html

#[doc(inline)]
pub use super::{Uuid, UuidVariant};
