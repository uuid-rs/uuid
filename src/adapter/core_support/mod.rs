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
use core::str;
use crate::prelude::*;

impl fmt::Display for super::Hyphenated {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl<'a> fmt::Display for super::HyphenatedRef<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Display for super::Simple {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl<'a> fmt::Display for super::SimpleRef<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Display for super::Urn {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl<'a> fmt::Display for super::UrnRef<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

fn format(
    f: &mut fmt::Formatter,
    uuid: &Uuid,
    hyphens: bool,
    upper: bool,
) -> fmt::Result {
    const UPPER: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B',
        b'C', b'D', b'E', b'F',
    ];
    const LOWER: [u8; 16] = [
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b',
        b'c', b'd', b'e', b'f',
    ];

    let mut buffer = [b'-'; 36];
    let mut idx = 0;

    let characters = if upper { &UPPER } else { &LOWER };

    for b in uuid.as_bytes() {
        buffer[idx] = characters[(b >> 4) as usize];
        buffer[idx + 1] = characters[(b & 0b1111) as usize];

        // Now skip forward to the place to write the next two
        // characters.  We need to skip forward an extra one to leave
        // a hyphen when we've just written the two bytes before it
        // (but only if hyphens are turned on):
        //
        // uuid: 00000000-0000-0000-0000-000000000000
        //             ^    ^    ^    ^
        // idx:            111111111122
        //       0123456789012345678901
        match idx {
            6 | 11 | 16 | 21 if hyphens => idx += 3,
            _ => idx += 2,
        }
    }
    let len = if hyphens { 36 } else { 32 };
    f.write_str(str::from_utf8(&buffer[..len]).unwrap())
}

impl fmt::LowerHex for super::Hyphenated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, &self.0, true, false)
    }
}

impl<'a> fmt::LowerHex for super::HyphenatedRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, self.0, true, false)
    }
}

impl fmt::LowerHex for super::Simple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, &self.0, false, false)
    }
}

impl<'a> fmt::LowerHex for super::SimpleRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, self.0, false, false)
    }
}

impl fmt::LowerHex for super::Urn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("urn:uuid:")?;
        format(f, &self.0, true, false)
    }
}

impl<'a> fmt::LowerHex for super::UrnRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("urn:uuid:")?;
        format(f, self.0, true, false)
    }
}

impl fmt::UpperHex for super::Hyphenated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, &self.0, true, true)
    }
}

impl<'a> fmt::UpperHex for super::HyphenatedRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, self.0, true, true)
    }
}

impl fmt::UpperHex for super::Simple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, &self.0, false, true)
    }
}

impl<'a> fmt::UpperHex for super::SimpleRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format(f, self.0, false, true)
    }
}

impl fmt::UpperHex for super::Urn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("urn:uuid:")?;
        format(f, &self.0, true, true)
    }
}

impl<'a> fmt::UpperHex for super::UrnRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("urn:uuid:")?;
        format(f, self.0, true, true)
    }
}

impl From<Uuid> for super::Hyphenated {
    #[inline]
    fn from(f: Uuid) -> Self {
        super::Hyphenated::from_uuid(f)
    }
}

impl<'a> From<&'a Uuid> for super::HyphenatedRef<'a> {
    #[inline]
    fn from(f: &'a Uuid) -> Self {
        super::HyphenatedRef::from_uuid_ref(f)
    }
}

impl From<Uuid> for super::Simple {
    #[inline]
    fn from(f: Uuid) -> Self {
        super::Simple::from_uuid(f)
    }
}

impl<'a> From<&'a Uuid> for super::SimpleRef<'a> {
    #[inline]
    fn from(f: &'a Uuid) -> Self {
        super::SimpleRef::from_uuid_ref(f)
    }
}

impl From<Uuid> for super::Urn {
    #[inline]
    fn from(f: Uuid) -> Self {
        super::Urn::from_uuid(f)
    }
}

impl<'a> From<&'a Uuid> for super::UrnRef<'a> {
    #[inline]
    fn from(f: &'a Uuid) -> Self {
        super::UrnRef::from_uuid_ref(f)
    }
}
