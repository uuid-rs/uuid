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

/// An adaptor for formatting an [`Uuid`] as a hyphenated string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidHyphenated(Uuid);

/// An adaptor for formatting an [`Uuid`] as a hyphenated string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidHyphenatedRef<'a>(&'a Uuid);

/// An adaptor for formatting an [`Uuid`] as a simple string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidSimple(Uuid);

/// An adaptor for formatting an [`Uuid`] as a simple string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidSimpleRef<'a>(&'a Uuid);

/// An adaptor for formatting an [`Uuid`] as a URN string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidUrn(Uuid);

/// An adaptor for formatting an [`Uuid`] as a URN string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UuidUrnRef<'a>(&'a Uuid);

impl Uuid {
    /// Creates a [`UuidHyphenated`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenated`]: struct.UuidHyphenated.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_hyphenated(self) -> UuidHyphenated {
        UuidHyphenated::from_uuid(self)
    }

    /// Creates a [`UuidHyphenated`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenated`]: struct.UuidHyphenated.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_hyphenated(self) -> UuidHyphenated {
        UuidHyphenated::from_uuid(self)
    }

    /// Creates a [`UuidHyphenatedRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenatedRef`]: struct.UuidHyphenatedRef.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_hyphenated_ref(&self) -> UuidHyphenatedRef {
        UuidHyphenatedRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidHyphenatedRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenatedRef`]: struct.UuidHyphenatedRef.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_hyphenated_ref(&self) -> UuidHyphenatedRef {
        UuidHyphenatedRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidSimple`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimple`]: struct.UuidSimple.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_simple(self) -> UuidSimple {
        UuidSimple::from_uuid(self)
    }

    /// Creates a [`UuidSimple`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimple`]: struct.UuidSimple.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_simple(self) -> UuidSimple {
        UuidSimple::from_uuid(self)
    }

    /// Creates a [`UuidSimpleRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.UuidSimpleRef.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_simple_ref(&self) -> UuidSimpleRef {
        UuidSimpleRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidSimpleRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.UuidSimpleRef.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_simple_ref(&self) -> UuidSimpleRef {
        UuidSimpleRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidUrn`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrn`]: struct.UuidUrn.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_urn(self) -> UuidUrn {
        UuidUrn::from_uuid(self)
    }

    /// Creates a [`UuidUrn`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrn`]: struct.UuidUrn.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_urn(self) -> UuidUrn {
        UuidUrn::from_uuid(self)
    }

    /// Creates a [`UuidUrnRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UuidUrnRef.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_urn_ref(&self) -> UuidUrnRef {
        UuidUrnRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidUrnRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UuidUrnRef.html
    // TODO(kinggoesgaming): discuss to_ vs as_ vs into_
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_urn_ref(&self) -> UuidUrnRef {
        UuidUrnRef::from_uuid_ref(self)
    }
}

impl UuidHyphenated {
    /// Creates a [`UuidHyphenated`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenated`]: struct.UuidHyphenated.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidHyphenated(uuid)
    }

    /// Creates a [`UuidHyphenated`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenated`]: struct.UuidHyphenated.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidHyphenated(uuid)
    }
}

impl<'a> UuidHyphenatedRef<'a> {
    /// Creates a [`UuidHyphenatedRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenatedRef`]: struct.UuidHyphenatedRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidHyphenatedRef(uuid)
    }

    /// Creates a [`UuidHyphenatedRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenatedRef`]: struct.UuidHyphenatedRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidHyphenatedRef(uuid)
    }
}

impl UuidSimple {
    /// Creates a [`UuidSimple`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimple`]: struct.UuidSimple.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidSimple(uuid)
    }

    /// Creates a [`UuidSimple`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimple`]: struct.UuidSimple.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidSimple(uuid)
    }
}

impl<'a> UuidSimpleRef<'a> {
    /// Creates a [`UuidSimpleRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.UuidSimpleRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidSimpleRef(uuid)
    }

    /// Creates a [`UuidSimpleRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.UuidSimpleRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidSimpleRef(uuid)
    }
}

impl UuidUrn {
    /// Creates a [`UuidUrn`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrn`]: struct.UuidUrn.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidUrn(uuid)
    }

    /// Creates a [`UuidUrn`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrn`]: struct.UuidUrn.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        UuidUrn(uuid)
    }
}

impl<'a> UuidUrnRef<'a> {
    /// Creates a [`UuidUrnRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UuidUrnRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidUrnRef(uuid)
    }

    /// Creates a [`UuidUrnRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UuidUrnRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UuidUrnRef(&uuid)
    }
}
