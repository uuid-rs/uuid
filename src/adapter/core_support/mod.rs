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
use prelude::*;

#[macro_use]
mod macros;

impl fmt::Display for super::UuidHyphenated {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl<'a> fmt::Display for super::UuidHyphenatedRef<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Display for super::UuidSimple {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl<'a> fmt::Display for super::UuidSimpleRef<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Display for super::UuidUrn {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl<'a> fmt::Display for super::UuidUrnRef<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::LowerHex for super::UuidHyphenated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hyphenated_write!(
            f,
            "{:08x}-\
             {:04x}-\
             {:04x}-\
             {:02x}{:02x}-\
             {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0.as_bytes()
        )
    }
}

impl<'a> fmt::LowerHex for super::UuidHyphenatedRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hyphenated_write!(
            f,
            "{:08x}-\
             {:04x}-\
             {:04x}-\
             {:02x}{:02x}-\
             {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0.as_bytes()
        )
    }
}

impl fmt::LowerHex for super::UuidSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.as_bytes() {
            write!(f, "{:02x}", byte)?
        }

        Ok(())
    }
}

impl<'a> fmt::LowerHex for super::UuidSimpleRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.as_bytes() {
            write!(f, "{:02x}", byte)?
        }

        Ok(())
    }
}

impl fmt::LowerHex for super::UuidUrn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hyphenated_write!(
            f,
            "urn:uuid:\
             {:08x}-\
             {:04x}-\
             {:04x}-\
             {:02x}{:02x}-\
             {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0.as_bytes()
        )
    }
}

impl<'a> fmt::LowerHex for super::UuidUrnRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hyphenated_write!(
            f,
            "urn:uuid:\
             {:08x}-\
             {:04x}-\
             {:04x}-\
             {:02x}{:02x}-\
             {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.0.as_bytes()
        )
    }
}

impl fmt::UpperHex for super::UuidHyphenated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hyphenated_write!(
            f,
            "{:08X}-\
             {:04X}-\
             {:04X}-\
             {:02X}{:02X}-\
             {:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0.as_bytes()
        )
    }
}

impl<'a> fmt::UpperHex for super::UuidHyphenatedRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hyphenated_write!(
            f,
            "{:08X}-\
             {:04X}-\
             {:04X}-\
             {:02X}{:02X}-\
             {:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0.as_bytes()
        )
    }
}

impl fmt::UpperHex for super::UuidSimple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.as_bytes() {
            write!(f, "{:02X}", byte)?
        }

        Ok(())
    }
}

impl<'a> fmt::UpperHex for super::UuidSimpleRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0.as_bytes() {
            write!(f, "{:02X}", byte)?
        }

        Ok(())
    }
}

impl fmt::UpperHex for super::UuidUrn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hyphenated_write!(
            f,
            "urn:uuid:\
             {:08X}-\
             {:04X}-\
             {:04X}-\
             {:02X}{:02X}-\
             {:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0.as_bytes()
        )
    }
}

impl<'a> fmt::UpperHex for super::UuidUrnRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        hyphenated_write!(
            f,
            "urn:uuid:\
             {:08X}-\
             {:04X}-\
             {:04X}-\
             {:02X}{:02X}-\
             {:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0.as_bytes()
        )
    }
}

impl From<Uuid> for super::UuidHyphenated {
    #[inline]
    fn from(f: Uuid) -> Self {
        super::UuidHyphenated::from_uuid(f)
    }
}

impl<'a> From<&'a Uuid> for super::UuidHyphenatedRef<'a> {
    #[inline]
    fn from(f: &'a Uuid) -> Self {
        super::UuidHyphenatedRef::from_uuid_ref(f)
    }
}

impl From<Uuid> for super::UuidSimple {
    #[inline]
    fn from(f: Uuid) -> Self {
        super::UuidSimple::from_uuid(f)
    }
}

impl<'a> From<&'a Uuid> for super::UuidSimpleRef<'a> {
    #[inline]
    fn from(f: &'a Uuid) -> Self {
        super::UuidSimpleRef::from_uuid_ref(f)
    }
}

impl From<Uuid> for super::UuidUrn {
    #[inline]
    fn from(f: Uuid) -> Self {
        super::UuidUrn::from_uuid(f)
    }
}

impl<'a> From<&'a Uuid> for super::UuidUrnRef<'a> {
    #[inline]
    fn from(f: &'a Uuid) -> Self {
        super::UuidUrnRef::from_uuid_ref(f)
    }
}
