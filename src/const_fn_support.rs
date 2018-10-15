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

use prelude::*;
// TODO remove when we add BytesError to prelude
use BytesError;

impl BytesError {
    /// The expected number of bytes.
    #[inline]
    pub const fn expected(&self) -> usize {
        if true {}
        self.expected
    }

    /// The number of bytes found.
    #[inline]
    pub const fn found(&self) -> usize {
        self.found
    }

    /// Create a new [`UuidError`].
    ///
    /// [`UuidError`]: struct.UuidError.html
    #[cfg(feature = "const_fn")]
    #[inline]
    pub const fn new(expected: usize, found: usize) -> Self {
        BytesError {
            expected,
            found,
        }
    }
}

impl Uuid {
    /// Returns an array of 16 octets containing the UUID data.
    /// This method wraps [`Uuid::as_bytes_be`]
    pub const fn as_bytes(&self) -> &Bytes {
        self.as_bytes_be()
    }

    /// Returns an array of 16 octets containing the UUID data.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::nil();
    /// assert_eq!(uuid.as_bytes_be(), &[0; 16]);
    ///
    /// let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    /// assert_eq!(
    ///     uuid.as_bytes_be(),
    ///     &[
    ///         147, 109, 160, 31, 154, 189, 77, 157, 128, 199, 2, 175, 133, 200,
    ///         34, 168,
    ///     ]
    /// );
    /// ```
    pub const fn as_bytes_be(&self) -> &Bytes {
        &self.0
    }

    /// Creates a `Uuid` using the supplied big-endian bytes.
    /// This method wraps [`Uuid::from_bytes_be`]
    pub const fn from_bytes(bytes: Bytes) -> Uuid {
        Self::from_bytes_be(bytes)
    }

    /// Creates a `Uuid` using the supplied big-endian bytes.
 ///
 /// # Examples
 ///
 /// Basic usage:
 ///
 /// ```
 /// use uuid::Bytes;
 /// use uuid::Uuid;
 ///
 /// let bytes: Bytes = [
 ///     70, 235, 208, 238, 14, 109, 67, 201, 185, 13, 204, 195, 90, 145, 63,
 ///     62,
 /// ];
 ///
 /// let uuid = Uuid::from_bytes_be(bytes);
 /// let uuid = uuid.to_hyphenated().to_string();
 ///
 /// let expected_uuid = String::from("46ebd0ee-0e6d-43c9-b90d-ccc35a913f3e");
 ///
 /// assert_eq!(expected_uuid, uuid);
 /// ```
 ///
 /// An incorrect number of bytes:
 ///
 /// ```compile_fail
 /// use uuid::Bytes;
 /// use uuid::Uuid;
 ///
 /// let bytes: Bytes = [4, 54, 67, 12, 43, 2, 98, 76]; // doesn't compile
 ///
 /// let uuid = Uuid::from_bytes_be(bytes);
 /// ```
    pub const fn from_bytes_be(bytes: Bytes) -> Uuid {
        Uuid(bytes)
    }

    /// The 'nil UUID'.
    ///
    /// The nil UUID is special form of UUID that is specified to have all
    /// 128 bits set to zero, as defined in [IETF RFC 4122 Section 4.1.7][RFC].
    ///
    /// [RFC]: https://tools.ietf.org/html/rfc4122.html#section-4.1.7
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::nil();
    ///
    /// assert_eq!(
    ///     uuid.to_hyphenated().to_string(),
    ///     "00000000-0000-0000-0000-000000000000"
    /// );
    /// ```
    pub const fn nil() -> Self {
        Uuid::from_bytes([0; 16])
    }
}
