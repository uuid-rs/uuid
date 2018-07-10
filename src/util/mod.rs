// Copyright 2013-2014 The Rust Project Developers.
// Copyright 2018 The Uuid Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Utility constructs for ['Uuid`] use.
//!
//! [`Uuid`]: ../struct.Uuid.html

use core::fmt;

mod core_support;

/// Possible [`Uuid`] lengths.
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UuidLength<T>
where
    T: AsRef<[usize]> + fmt::Debug,
{
    /// The [`Uuid`] length is exactly as given.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    Exact(usize),
    /// The [`Uuid`] length is one of the given values.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    OneOf(T),
    /// The [`Uuid`] length is between the given range inclusive.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    Range {
        /// The maximum [`Uuid`] length.
        ///
        /// [`Uuid`]: ../struct.Uuid.html
        max: usize,
        /// The minimum [`Uuid`] length.
        ///
        /// [`Uuid`]: ../struct.Uuid.html
        min: usize,
    },
}
