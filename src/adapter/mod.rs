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

use crate::prelude::*;

mod core_support;

/// An adaptor for formatting an [`Uuid`] as a hyphenated string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hyphenated(Uuid);

/// An adaptor for formatting an [`Uuid`] as a hyphenated string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HyphenatedRef<'a>(&'a Uuid);

/// An adaptor for formatting an [`Uuid`] as a simple string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Simple(Uuid);

/// An adaptor for formatting an [`Uuid`] as a simple string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SimpleRef<'a>(&'a Uuid);

/// An adaptor for formatting an [`Uuid`] as a URN string.
///
/// Takes an owned instance of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Urn(Uuid);

/// An adaptor for formatting an [`Uuid`] as a URN string.
///
/// Takes a reference of the [`Uuid`].
///
/// [`Uuid`]: ../struct.Uuid.html
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UrnRef<'a>(&'a Uuid);

impl Uuid {
    /// Creates a [`UuidHyphenated`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenated`]: struct.UuidHyphenated.html
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_hyphenated(self) -> Hyphenated {
        Hyphenated::from_uuid(self)
    }

    /// Creates a [`UuidHyphenated`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenated`]: struct.UuidHyphenated.html
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_hyphenated(self) -> Hyphenated {
        Hyphenated::from_uuid(self)
    }

    /// Creates a [`UuidHyphenatedRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenatedRef`]: struct.UuidHyphenatedRef.html
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_hyphenated_ref(&self) -> HyphenatedRef {
        HyphenatedRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidHyphenatedRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenatedRef`]: struct.UuidHyphenatedRef.html
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_hyphenated_ref(&self) -> HyphenatedRef {
        HyphenatedRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidSimple`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimple`]: struct.UuidSimple.html
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_simple(self) -> Simple {
        Simple::from_uuid(self)
    }

    /// Creates a [`UuidSimple`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimple`]: struct.UuidSimple.html
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_simple(self) -> Simple {
        Simple::from_uuid(self)
    }

    /// Creates a [`UuidSimpleRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.UuidSimpleRef.html
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_simple_ref(&self) -> SimpleRef {
        SimpleRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidSimpleRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.UuidSimpleRef.html
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_simple_ref(&self) -> SimpleRef {
        SimpleRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidUrn`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrn`]: struct.UuidUrn.html
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_urn(self) -> Urn {
        Urn::from_uuid(self)
    }

    /// Creates a [`UuidUrn`] instance from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrn`]: struct.UuidUrn.html
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_urn(self) -> Urn {
        Urn::from_uuid(self)
    }

    /// Creates a [`UuidUrnRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UuidUrnRef.html
    #[cfg(not(feature = "const_fn"))]
    #[inline]
    pub fn to_urn_ref(&self) -> UrnRef {
        UrnRef::from_uuid_ref(self)
    }

    /// Creates a [`UuidUrnRef`] instance from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UuidUrnRef.html
    #[cfg(feature = "const_fn")]
    #[inline]
    pub fn to_urn_ref(&self) -> UrnRef {
        UrnRef::from_uuid_ref(self)
    }
}

impl Hyphenated {
    /// The length of a hyphenated [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 36;

    /// Creates a [`UuidHyphenated`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenated`]: struct.UuidHyphenated.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Hyphenated(uuid)
    }

    /// Creates a [`UuidHyphenated`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenated`]: struct.UuidHyphenated.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Hyphenated(uuid)
    }
}

impl<'a> HyphenatedRef<'a> {
    /// The length of a hyphenated [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 36;

    /// Creates a [`UuidHyphenatedRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenatedRef`]: struct.UuidHyphenatedRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        HyphenatedRef(uuid)
    }

    /// Creates a [`UuidHyphenatedRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidHyphenatedRef`]: struct.UuidHyphenatedRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        HyphenatedRef(uuid)
    }
}

impl Simple {
    /// The length of a simple [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 32;

    /// Creates a [`UuidSimple`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimple`]: struct.UuidSimple.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Simple(uuid)
    }

    /// Creates a [`UuidSimple`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimple`]: struct.UuidSimple.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Simple(uuid)
    }
}

impl<'a> SimpleRef<'a> {
    /// The length of a simple [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 32;

    /// Creates a [`UuidSimpleRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.UuidSimpleRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        SimpleRef(uuid)
    }

    /// Creates a [`UuidSimpleRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidSimpleRef`]: struct.UuidSimpleRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        SimpleRef(uuid)
    }
}

impl Urn {
    /// The length of a URN [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 45;

    /// Creates a [`UuidUrn`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrn`]: struct.UuidUrn.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Urn(uuid)
    }

    /// Creates a [`UuidUrn`] from a [`Uuid`].
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrn`]: struct.UuidUrn.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid(uuid: Uuid) -> Self {
        Urn(uuid)
    }
}

impl<'a> UrnRef<'a> {
    /// The length of a URN [`Uuid`] string.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    pub const LENGTH: usize = 45;

    /// Creates a [`UuidUrnRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UuidUrnRef.html
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UrnRef(uuid)
    }

    /// Creates a [`UuidUrnRef`] from a [`Uuid`] reference.
    ///
    /// [`Uuid`]: ../struct.Uuid.html
    /// [`UuidUrnRef`]: struct.UuidUrnRef.html
    #[cfg(feature = "const_fn")]
    pub fn from_uuid_ref(uuid: &'a Uuid) -> Self {
        UrnRef(&uuid)
    }
}
