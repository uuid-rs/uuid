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

//! [`Uuid`] parsing constructs and utilities.
//!
//! [`Uuid`]: ../struct.Uuid.html

use core::fmt;

mod core_support;
#[cfg(feature = "std")]
mod std_support;

/// The expected value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Expected<T>
where
    T: AsRef<[usize]> + fmt::Debug,
{
    /// Expected any one of the given values.
    Any(T),
    /// Expected the given value.
    Exact(usize),
    /// Expected any values in the given range.
    Range {
        /// The minimum expected value.
        min: usize,
        /// The maximum expected value.
        max: usize,
    },
}

/// An error that can occur while parsing a [`Uuid`] string.
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UuidParseError<'chars, T>
where
    T: AsRef<[usize]> + fmt::Debug,
{
    /// Invalid character in the [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    InvalidCharacter {
        /// The expected characters.
        expected: &'chars str,
        /// The invalid character found.
        found: char,
        /// The invalid character position.
        index: usize,
    },
    /// Invalid number of segments in the [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    InvalidGroupCount {
        /// The expected number of segments.
        // TODO: explain multiple segment count.
        // BODY: Parsers can expect a range of Uuid segment count.
        //       This needs to be expanded on.
        expected: T,
        /// The number of segments found.
        found: usize,
    },
    /// Invalid length of a segment in a [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    InvalidGroupLength {
        /// The expected length of the segment.
        expected: T,
        /// The length of segment found.
        found: usize,
        /// The segment with invalid length.
        group: usize,
    },
    /// Invalid length of the [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    InvalidLength {
        /// The expected length(s).
        // TODO: explain multiple lengths.
        // BODY: Parsers can expect a range of Uuid lenghts.
        //       This needs to be expanded on.
        expected: T,
        /// The invalid length found.
        found: usize,
    },
}

/// Check if the length matches the given criterion length.
// TODO: Find another way handling this.
// BODY: This is not idiomatic.
#[inline]
pub fn len_matches(len: usize, crit: usize) -> bool {
    len == crit
}

/// Check if the length matches any of the given criteria lengths.
pub fn len_matches_any(len: usize, crits: &[usize]) -> bool {
    for crit in crits {
        if let true = len_matches(len, *crit) {
            return true
        }
    }

    false
}
