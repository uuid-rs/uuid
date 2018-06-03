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

//! Adapters for various formats for [`Uuid`]s
//!
//! [`Uuid`]: ../struct.Uuid.html

use prelude::*;

mod core_support;

/// The length of a Hyphenated [`Uuid`] string.
///
/// [`Uuid`]: ../struct.Uuid.html
pub const UUID_HYPHENATED_LENGTH: usize = 36;

/// The length of a Simple [`Uuid`] string.
///
/// [`Uuid`]: ../struct.Uuid.html
pub const UUID_SIMPLE_LENGTH: usize = 32;

/// The length of a Urn [`Uuid`] string.
///
/// [`Uuid`]: ../struct.Uuid.html
pub const UUID_URN_LENGTH: usize = 45;

// TODO: documentation
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidHyphenated(Uuid);
// TODO: documentation
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidHyphenatedRef<'a>(&'a Uuid);
// TODO(kinggoesgaming): documentation
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidSimple(Uuid);
// TODO(kinggoesgaming): documentation
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidSimpleRef<'a>(&'a Uuid);
// TODO(kinggoesgaming): documentation
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidUrn(Uuid);
// TODO(kinggoesgaming): documentation
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidUrnRef<'a>(&'a Uuid);

impl Uuid {
    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_hyphenated(self) -> UuidHyphenated {
        UuidHyphenated::from_uuid(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_hyphenated(self) -> UuidHyphenated {
        UuidHyphenated::from_uuid(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_hyphenated_ref(&self) -> UuidHyphenatedRef {
        UuidHyphenatedRef::from_uuid_ref(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_hyphenated_ref(&self) -> UuidHyphenatedRef {
        UuidHyphenatedRef::from_uuid_ref(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_simple(self) -> UuidSimple {
        UuidSimple::from_uuid(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_simple(self) -> UuidSimple {
        UuidSimple::from_uuid(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_simple_ref(&self) -> UuidSimpleRef {
        UuidSimpleRef::from_uuid_ref(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_simple_ref(&self) -> UuidSimpleRef {
        UuidSimpleRef::from_uuid_ref(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_urn(self) -> UuidUrn {
        UuidUrn::from_uuid(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_urn(self) -> UuidUrn {
        UuidUrn::from_uuid(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_urn_ref(&self) -> UuidUrnRef {
        UuidUrnRef::from_uuid_ref(self)
    }

    // TODO(kinggoesgaming): documentation
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_urn_ref(&self) -> UuidUrnRef {
        UuidUrnRef::from_uuid_ref(self)
    }
}

impl UuidHyphenated {
    // TODO(kinggoesgaming): documentation
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidHyphenated(uuid)
    }

    // TODO(kinggoesgaming): documentation
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidHyphenated(uuid)
    }
}

impl<'a> UuidHyphenatedRef<'a> {
    // TODO(kinggoesgaming): documentation
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidHyphenatedRef(uuid)
    }

    // TODO(kinggoesgaming): documentation
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidHyphenatedRef(uuid)
    }
}

impl UuidSimple {
    // TODO(kinggoesgaming): documentation
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidSimple(uuid)
    }

    // TODO(kinggoesgaming): documentation
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidSimple(uuid)
    }
}

impl<'a> UuidSimpleRef<'a> {
    // TODO(kinggoesgaming): documentation
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidSimpleRef(uuid)
    }

    // TODO(kinggoesgaming): documentation
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidSimpleRef(uuid)
    }
}

impl UuidUrn {
    // TODO(kinggoesgaming): documentation
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidUrn(uuid)
    }

    // TODO(kinggoesgaming): documentation
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidUrn(uuid)
    }
}

impl<'a> UuidUrnRef<'a> {
    // TODO(kinggoesgaming): documentation
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidUrnRef(uuid)
    }

    // TODO(kinggoesgaming): documentation
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidUrnRef(uuid)
    }
}
