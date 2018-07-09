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

use core::fmt;
use util;

impl<T> fmt::Display for util::UuidLength<T>
where
    T: AsRef<[usize]> + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            util::UuidLength::Exact(v) => write!(f, "{}", v),
            util::UuidLength::OneOf(v) => write!(f, "any one of {:?}", v),
            util::UuidLength::Range { max, min } => {
                write!(f, "{}..{}", min, max)
            }
        }
    }
}
