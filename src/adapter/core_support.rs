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

use crate::prelude::*;
use core::fmt;

macro_rules! impl_adapter_traits {
    ($($T:ident<$($a:lifetime),*>),+) => {$(
        impl<$($a),*> fmt::Display for super::$T<$($a),*> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::LowerHex::fmt(self, f)
            }
        }

        impl<$($a),*> fmt::LowerHex for super::$T<$($a),*> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // TODO: Self doesn't work https://github.com/rust-lang/rust/issues/52808
                f.write_str(self.encode_lower(&mut [0; super::$T::LENGTH]))
            }
        }

        impl<$($a),*> fmt::UpperHex for super::$T<$($a),*> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // TODO: Self doesn't work https://github.com/rust-lang/rust/issues/52808
                f.write_str(self.encode_upper(&mut [0; super::$T::LENGTH]))
            }
        }

        impl_adapter_from!($T<$($a),*>);
    )+}
}

macro_rules! impl_adapter_from {
    ($T:ident<>) => {
        impl From<Uuid> for super::$T {
            #[inline]
            fn from(f: Uuid) -> Self {
                super::$T::from_uuid(f)
            }
        }
    };
    ($T:ident<$a:lifetime>) => {
        impl<$a> From<&$a Uuid> for super::$T<$a> {
            #[inline]
            fn from(f: &$a Uuid) -> Self {
                super::$T::from_uuid_ref(f)
            }
        }
    };
}

impl_adapter_traits! {
    Hyphenated<>,
    HyphenatedRef<'a>,
    Simple<>,
    SimpleRef<'a>,
    Urn<>,
    UrnRef<'a>
}
