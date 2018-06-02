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

//! Generate and parse UUIDs.
//!
//! Provides support for Universally Unique Identifiers (UUIDs). A UUID is a
//! unique 128-bit number, stored as 16 octets.  UUIDs are used to  assign
//! unique identifiers to entities without requiring a central allocating
//! authority.
//!
//! They are particularly useful in distributed systems, though can be used in
//! disparate areas, such as databases and network protocols.  Typically a UUID
//! is displayed in a readable string form as a sequence of hexadecimal digits,
//! separated into groups by hyphens.
//!
//! The uniqueness property is not strictly guaranteed, however for all
//! practical purposes, it can be assumed that an unintentional collision would
//! be extremely unlikely.
//!
//! # Dependencies
//!
//! By default, this crate depends on nothing but `std` and cannot generate
//! [`Uuid`]s. You need to enable the following Cargo features to enable
//! various pieces of functionality:
//!
//! * `v1` - adds the `Uuid::new_v1` function and the ability to create a V1
//!   using an implementation of `UuidV1ClockSequence` (usually `UuidV1Context`)
//!   and a timestamp from `time::timespec`.
//! * `v3` - adds the `Uuid::new_v3` function and the ability to create a V3
//!   UUID based on the MD5 hash of some data.
//! * `v4` - adds the `Uuid::new_v4` function and the ability to randomly
//!   generate a `Uuid`.
//! * `v5` - adds the `Uuid::new_v5` function and the ability to create a V5
//!   UUID based on the SHA1 hash of some data.
//! * `serde` - adds the ability to serialize and deserialize a `Uuid` using the
//!   `serde` crate.
//!
//! By default, `uuid` can be depended on with:
//!
//! ```toml
//! [dependencies]
//! uuid = "0.6"
//! ```
//!
//! To activate various features, use syntax like:
//!
//! ```toml
//! [dependencies]
//! uuid = { version = "0.6", features = ["serde", "v4"] }
//! ```
//!
//! You can disable default features with:
//!
//! ```toml
//! [dependencies]
//! uuid = { version = "0.6", default-features = false }
//! ```
//!
//! # Examples
//!
//! To parse a UUID given in the simple format and print it as a urn:
//!
//! ```rust
//! use uuid::Uuid;
//!
//! fn main() {
//!     let my_uuid =
//!         Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
//!     println!("{}", my_uuid.to_urn());
//! }
//! ```
//!
//! To create a new random (V4) UUID and print it out in hexadecimal form:
//!
//! ```ignore,rust
//! // Note that this requires the `v4` feature enabled in the uuid crate.
//!
//! use uuid::Uuid;
//!
//! fn main() {
//!     let my_uuid = Uuid::new_v4();
//!     println!("{}", my_uuid);
//! }
//! ```
//!
//! # Strings
//!
//! Examples of string representations:
//!
//! * simple: `936DA01F9ABD4d9d80C702AF85C822A8`
//! * hyphenated: `550e8400-e29b-41d4-a716-446655440000`
//! * urn: `urn:uuid:F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4`
//!
//! # References
//!
//! * [Wikipedia: Universally Unique Identifier](
//!     http://en.wikipedia.org/wiki/Universally_unique_identifier)
//! * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](
//!     http://tools.ietf.org/html/rfc4122)

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://docs.rs/uuid"
)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "const_fn", feature(const_fn))]

#[cfg(feature = "byteorder")]
extern crate byteorder;
#[cfg(feature = "std")]
extern crate core;
#[cfg(feature = "md5")]
extern crate md5;
#[cfg(feature = "rand")]
extern crate rand;
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(all(feature = "serde", test))]
extern crate serde_test;
#[cfg(feature = "sha1")]
extern crate sha1;
#[cfg(feature = "slog")]
#[cfg_attr(test, macro_use)]
extern crate slog;

use core::{fmt, str};

pub mod adapter;
pub mod ns;
pub mod prelude;
#[cfg(feature = "v1")]
pub mod v1;

mod core_support;
#[cfg(feature = "serde")]
mod serde_support;
#[cfg(feature = "slog")]
mod slog_support;
#[cfg(feature = "std")]
mod std_support;
#[cfg(test)]
mod test_util;
#[cfg(feature = "u128")]
mod u128_support;
#[cfg(feature = "v3")]
mod v3;
#[cfg(feature = "v4")]
mod v4;
#[cfg(feature = "v5")]
mod v5;

/// A 128-bit (16 byte) buffer containing the ID.
pub type UuidBytes = [u8; 16];

/// The version of the UUID, denoting the generating algorithm.
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub enum UuidVersion {
    /// Special case for `nil` [`Uuid`].
    ///
    /// [`Uuid`]: struct.Uuid.html
    Nil = 0,
    /// Version 1: MAC address
    Mac,
    /// Version 2: DCE Security
    Dce,
    /// Version 3: MD5 hash
    Md5,
    /// Version 4: Random
    Random,
    /// Version 5: SHA-1 hash
    Sha1,
}

/// The reserved variants of UUIDs.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub enum UuidVariant {
    /// Reserved by the NCS for backward compatibility
    NCS = 0,
    /// As described in the RFC4122 Specification (default)
    RFC4122,
    /// Reserved by Microsoft for backward compatibility
    Microsoft,
    /// Reserved for future expansion
    Future,
}

/// A Universally Unique Identifier (UUID).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Uuid {
    /// The 128-bit number stored in 16 bytes
    bytes: UuidBytes,
}

/// Error details for string parsing failures.
#[allow(missing_docs)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ParseError {
    InvalidLength(usize),
    InvalidCharacter(char, usize),
    InvalidGroups(usize),
    InvalidGroupLength(usize, usize, u8),
}

/// Converts a `ParseError` to a string.
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidLength(found) => write!(
                f,
                "Invalid length; expecting {} or {} chars, found {}",
                adapter::UUID_SIMPLE_LENGTH,
                adapter::UUID_HYPHENATED_LENGTH,
                found
            ),
            ParseError::InvalidCharacter(found, pos) => write!(
                f,
                "Invalid character; found `{}` (0x{:02x}) at offset {}",
                found, found as usize, pos
            ),
            ParseError::InvalidGroups(found) => write!(
                f,
                "Malformed; wrong number of groups: expected 1 or 5, found {}",
                found
            ),
            ParseError::InvalidGroupLength(group, found, expecting) => write!(
                f,
                "Malformed; length of group {} was {}, expecting {}",
                group, found, expecting
            ),
        }
    }
}

// Length of each hyphenated group in hex digits.
const GROUP_LENS: [u8; 5] = [8, 4, 4, 4, 12];
// Accumulated length of each hyphenated group in hex digits.
const ACC_GROUP_LENS: [u8; 5] = [8, 12, 16, 20, 32];

impl Uuid {
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
    #[cfg(feature = "const_fn")]
    pub const fn nil() -> Self {
        Uuid { bytes: [0; 16] }
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
    #[cfg(not(feature = "const_fn"))]
    pub fn nil() -> Uuid {
        Uuid { bytes: [0; 16] }
    }

    /// Creates a `Uuid` from four field values.
    ///
    /// # Errors
    ///
    /// This function will return an error if `d4`'s length is not 8 bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// let d4 = [12, 3, 9, 56, 54, 43, 8, 9];
    ///
    /// let uuid = Uuid::from_fields(42, 12, 5, &d4);
    /// let uuid = uuid.map(|uuid| uuid.to_hyphenated().to_string());
    ///
    /// let expected_uuid =
    ///     Ok(String::from("0000002a-000c-0005-0c03-0938362b0809"));
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    ///
    /// An invalid length:
    ///
    /// ```
    /// use uuid::ParseError;
    /// use uuid::Uuid;
    ///
    /// let d4 = [12];
    ///
    /// let uuid = Uuid::from_fields(42, 12, 5, &d4);
    ///
    /// let expected_uuid = Err(ParseError::InvalidLength(1));
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    pub fn from_fields(
        d1: u32,
        d2: u16,
        d3: u16,
        d4: &[u8],
    ) -> Result<Uuid, ParseError> {
        if d4.len() != 8 {
            return Err(ParseError::InvalidLength(d4.len()));
        }

        Ok(Uuid {
            bytes: [
                (d1 >> 24) as u8,
                (d1 >> 16) as u8,
                (d1 >> 8) as u8,
                d1 as u8,
                (d2 >> 8) as u8,
                d2 as u8,
                (d3 >> 8) as u8,
                d3 as u8,
                d4[0],
                d4[1],
                d4[2],
                d4[3],
                d4[4],
                d4[5],
                d4[6],
                d4[7],
            ],
        })
    }

    /// Creates a `Uuid` using the supplied bytes.
    ///
    /// # Errors
    ///
    /// This function will return an error if `b` has any length other than 16.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// let bytes = [4, 54, 67, 12, 43, 2, 98, 76, 32, 50, 87, 5, 1, 33, 43, 87];
    ///
    /// let uuid = Uuid::from_bytes(&bytes);
    /// let uuid = uuid.map(|uuid| uuid.to_hyphenated().to_string());
    ///
    /// let expected_uuid =
    ///     Ok(String::from("0436430c-2b02-624c-2032-570501212b57"));
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    ///
    /// An incorrect number of bytes:
    ///
    /// ```
    /// use uuid::ParseError;
    /// use uuid::Uuid;
    ///
    /// let bytes = [4, 54, 67, 12, 43, 2, 98, 76];
    ///
    /// let uuid = Uuid::from_bytes(&bytes);
    ///
    /// let expected_uuid = Err(ParseError::InvalidLength(8));
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    pub fn from_bytes(b: &[u8]) -> Result<Uuid, ParseError> {
        let len = b.len();
        if len != 16 {
            return Err(ParseError::InvalidLength(len));
        }

        let mut uuid = Uuid { bytes: [0; 16] };
        uuid.bytes.copy_from_slice(b);
        Ok(uuid)
    }

    /// Creates a `Uuid` using the supplied bytes.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Uuid;
    /// use uuid::UuidBytes;
    ///
    /// let bytes: UuidBytes = [
    ///     70, 235, 208, 238, 14, 109, 67, 201, 185, 13, 204, 195, 90, 145, 63,
    ///     62,
    /// ];
    ///
    /// let uuid = Uuid::from_uuid_bytes(bytes);
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
    /// use uuid::Uuid;
    /// use uuid::UuidBytes;
    ///
    /// let bytes: UuidBytes = [4, 54, 67, 12, 43, 2, 98, 76]; // doesn't
    /// compile
    ///
    /// let uuid = Uuid::from_uuid_bytes(bytes);
    /// ```
    #[cfg(not(feature = "const_fn"))]
    pub fn from_uuid_bytes(bytes: UuidBytes) -> Uuid {
        Uuid { bytes }
    }

    #[cfg(feature = "const_fn")]
    pub const fn from_uuid_bytes(bytes: UuidBytes) -> Uuid {
        Uuid { bytes }
    }

    /// Creates a v4 Uuid from random bytes (e.g. bytes supplied from `Rand`
    /// crate)
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Uuid;
    /// use uuid::UuidBytes;
    ///
    /// let bytes: UuidBytes = [
    ///     70, 235, 208, 238, 14, 109, 67, 201, 185, 13, 204, 195, 90, 145, 63,
    ///     62,
    /// ];
    /// let uuid = Uuid::from_random_bytes(bytes);
    /// let uuid = uuid.to_hyphenated().to_string();
    ///
    /// let expected_uuid = String::from("46ebd0ee-0e6d-43c9-b90d-ccc35a913f3e");
    ///
    /// assert_eq!(expected_uuid, uuid);
    /// ```
    ///
    pub fn from_random_bytes(b: [u8; 16]) -> Uuid {
        let mut uuid = Uuid { bytes: b };
        uuid.set_variant(UuidVariant::RFC4122);
        uuid.set_version(UuidVersion::Random);
        uuid
    }

    /// Specifies the variant of the UUID structure
    #[allow(dead_code)]
    fn set_variant(&mut self, v: UuidVariant) {
        // Octet 8 contains the variant in the most significant 3 bits
        self.bytes[8] = match v {
            UuidVariant::NCS => self.bytes[8] & 0x7f, // b0xx...
            UuidVariant::RFC4122 => (self.bytes[8] & 0x3f) | 0x80, // b10x...
            UuidVariant::Microsoft => (self.bytes[8] & 0x1f) | 0xc0, // b110...
            UuidVariant::Future => (self.bytes[8] & 0x1f) | 0xe0, // b111...
        }
    }

    /// Returns the variant of the `Uuid` structure.
    ///
    /// This determines the interpretation of the structure of the UUID.
    /// Currently only the RFC4122 variant is generated by this module.
    ///
    /// * [Variant Reference](http://tools.ietf.org/html/rfc4122#section-4.1.1)
    pub fn get_variant(&self) -> Option<UuidVariant> {
        match self.bytes[8] {
            x if x & 0x80 == 0x00 => Some(UuidVariant::NCS),
            x if x & 0xc0 == 0x80 => Some(UuidVariant::RFC4122),
            x if x & 0xe0 == 0xc0 => Some(UuidVariant::Microsoft),
            x if x & 0xe0 == 0xe0 => Some(UuidVariant::Future),
            _ => None,
        }
    }

    /// Specifies the version number of the `Uuid`.
    #[allow(dead_code)]
    fn set_version(&mut self, v: UuidVersion) {
        self.bytes[6] = (self.bytes[6] & 0xF) | ((v as u8) << 4);
    }

    /// Returns the version number of the `Uuid`.
    ///
    /// This represents the algorithm used to generate the contents.
    ///
    /// Currently only the Random (V4) algorithm is supported by this
    /// module.  There are security and privacy implications for using
    /// older versions - see [Wikipedia: Universally Unique Identifier](
    /// http://en.wikipedia.org/wiki/Universally_unique_identifier) for
    /// details.
    ///
    /// * [Version Reference](http://tools.ietf.org/html/rfc4122#section-4.1.3)
    pub fn get_version_num(&self) -> usize {
        (self.bytes[6] >> 4) as usize
    }

    /// Returns the version of the `Uuid`.
    ///
    /// This represents the algorithm used to generate the contents
    pub fn get_version(&self) -> Option<UuidVersion> {
        let v = self.bytes[6] >> 4;
        match v {
            0 if self.is_nil() => Some(UuidVersion::Nil),
            1 => Some(UuidVersion::Mac),
            2 => Some(UuidVersion::Dce),
            3 => Some(UuidVersion::Md5),
            4 => Some(UuidVersion::Random),
            5 => Some(UuidVersion::Sha1),
            _ => None,
        }
    }

    /// Returns the four field values of the UUID.
    ///
    /// These values can be passed to the `from_fields()` method to get the
    /// original `Uuid` back.
    ///
    /// * The first field value represents the first group of (eight) hex
    ///   digits, taken as a big-endian `u32` value.  For V1 UUIDs, this field
    ///   represents the low 32 bits of the timestamp.
    /// * The second field value represents the second group of (four) hex
    ///   digits, taken as a big-endian `u16` value.  For V1 UUIDs, this field
    ///   represents the middle 16 bits of the timestamp.
    /// * The third field value represents the third group of (four) hex
    ///   digits, taken as a big-endian `u16` value.  The 4 most significant
    ///   bits give the UUID version, and for V1 UUIDs, the last 12 bits
    ///   represent the high 12 bits of the timestamp.
    /// * The last field value represents the last two groups of four and
    ///   twelve hex digits, taken in order.  The first 1-3 bits of this
    ///   indicate the UUID variant, and for V1 UUIDs, the next 13-15 bits
    ///   indicate the clock sequence and the last 48 bits indicate the node
    ///   ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::nil();
    /// assert_eq!(uuid.as_fields(), (0, 0, 0, &[0u8; 8]));
    ///
    /// let uuid = Uuid::parse_str("936DA01F-9ABD-4D9D-80C7-02AF85C822A8").unwrap();
    /// assert_eq!(
    ///     uuid.as_fields(),
    ///     (
    ///         0x936DA01F,
    ///         0x9ABD,
    ///         0x4D9D,
    ///         b"\x80\xC7\x02\xAF\x85\xC8\x22\xA8"
    ///     )
    /// );
    /// ```
    pub fn as_fields(&self) -> (u32, u16, u16, &[u8; 8]) {
        let d1 = u32::from(self.bytes[0]) << 24
            | u32::from(self.bytes[1]) << 16
            | u32::from(self.bytes[2]) << 8
            | u32::from(self.bytes[3]);

        let d2 = u16::from(self.bytes[4]) << 8 | u16::from(self.bytes[5]);

        let d3 = u16::from(self.bytes[6]) << 8 | u16::from(self.bytes[7]);

        let d4: &[u8; 8] =
            unsafe { &*(self.bytes[8..16].as_ptr() as *const [u8; 8]) };
        (d1, d2, d3, d4)
    }

    /// Returns an array of 16 octets containing the UUID data.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::nil();
    /// assert_eq!(uuid.as_bytes(), &[0; 16]);
    ///
    /// let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    /// assert_eq!(
    ///     uuid.as_bytes(),
    ///     &[
    ///         147, 109, 160, 31, 154, 189, 77, 157, 128, 199, 2, 175, 133, 200,
    ///         34, 168,
    ///     ]
    /// );
    /// ```
    #[cfg(feature = "const_fn")]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    /// Returns an array of 16 octets containing the UUID data.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::nil();
    /// assert_eq!(uuid.as_bytes(), &[0; 16]);
    ///
    /// let uuid = Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    /// assert_eq!(
    ///     uuid.as_bytes(),
    ///     &[
    ///         147, 109, 160, 31, 154, 189, 77, 157, 128, 199, 2, 175, 133, 200,
    ///         34, 168
    ///     ]
    /// );
    /// ```
    #[cfg(not(feature = "const_fn"))]
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    /// Returns an Optional Tuple of (u64, u16) representing the timestamp and
    /// counter portion of a V1 UUID.  If the supplied UUID is not V1, this
    /// will return None
    pub fn to_timestamp(&self) -> Option<(u64, u16)> {
        if self
            .get_version()
            .map(|v| v != UuidVersion::Mac)
            .unwrap_or(true)
        {
            return None;
        }

        let ts: u64 = u64::from(self.bytes[6] & 0x0F) << 56
            | u64::from(self.bytes[7]) << 48
            | u64::from(self.bytes[4]) << 40
            | u64::from(self.bytes[5]) << 32
            | u64::from(self.bytes[0]) << 24
            | u64::from(self.bytes[1]) << 16
            | u64::from(self.bytes[2]) << 8
            | u64::from(self.bytes[3]);

        let count: u16 =
            u16::from(self.bytes[8] & 0x3F) << 8 | u16::from(self.bytes[9]);

        Some((ts, count))
    }

    /// Parses a `Uuid` from a string of hexadecimal digits with optional
    /// hyphens.
    ///
    /// Any of the formats generated by this module (simple, hyphenated, urn)
    /// are supported by this parsing function.
    pub fn parse_str(mut input: &str) -> Result<Uuid, ParseError> {
        // Ensure length is valid for any of the supported formats
        let len = input.len();
        if len == (adapter::UUID_HYPHENATED_LENGTH + 9)
            && input.starts_with("urn:uuid:")
        {
            input = &input[9..];
        } else if len != adapter::UUID_SIMPLE_LENGTH
            && len != adapter::UUID_HYPHENATED_LENGTH
        {
            return Err(ParseError::InvalidLength(len));
        }

        // `digit` counts only hexadecimal digits, `i_char` counts all chars.
        let mut digit = 0;
        let mut group = 0;
        let mut acc = 0;
        let mut buffer = [0u8; 16];

        for (i_char, chr) in input.bytes().enumerate() {
            if digit as usize >= adapter::UUID_SIMPLE_LENGTH && group != 4 {
                if group == 0 {
                    return Err(ParseError::InvalidLength(len));
                }
                return Err(ParseError::InvalidGroups(group + 1));
            }

            if digit % 2 == 0 {
                // First digit of the byte.
                match chr {
                    // Calulate upper half.
                    b'0'...b'9' => acc = chr - b'0',
                    b'a'...b'f' => acc = chr - b'a' + 10,
                    b'A'...b'F' => acc = chr - b'A' + 10,
                    // Found a group delimiter
                    b'-' => {
                        if ACC_GROUP_LENS[group] != digit {
                            // Calculate how many digits this group consists of
                            // in the input.
                            let found = if group > 0 {
                                digit - ACC_GROUP_LENS[group - 1]
                            } else {
                                digit
                            };
                            return Err(ParseError::InvalidGroupLength(
                                group,
                                found as usize,
                                GROUP_LENS[group],
                            ));
                        }
                        // Next group, decrement digit, it is incremented again
                        // at the bottom.
                        group += 1;
                        digit -= 1;
                    }
                    _ => {
                        return Err(ParseError::InvalidCharacter(
                            input[i_char..].chars().next().unwrap(),
                            i_char,
                        ))
                    }
                }
            } else {
                // Second digit of the byte, shift the upper half.
                acc *= 16;
                match chr {
                    b'0'...b'9' => acc += chr - b'0',
                    b'a'...b'f' => acc += chr - b'a' + 10,
                    b'A'...b'F' => acc += chr - b'A' + 10,
                    b'-' => {
                        // The byte isn't complete yet.
                        let found = if group > 0 {
                            digit - ACC_GROUP_LENS[group - 1]
                        } else {
                            digit
                        };
                        return Err(ParseError::InvalidGroupLength(
                            group,
                            found as usize,
                            GROUP_LENS[group],
                        ));
                    }
                    _ => {
                        return Err(ParseError::InvalidCharacter(
                            input[i_char..].chars().next().unwrap(),
                            i_char,
                        ))
                    }
                }
                buffer[(digit / 2) as usize] = acc;
            }
            digit += 1;
        }

        // Now check the last group.
        if ACC_GROUP_LENS[4] != digit {
            return Err(ParseError::InvalidGroupLength(
                group,
                (digit - ACC_GROUP_LENS[3]) as usize,
                GROUP_LENS[4],
            ));
        }

        Ok(Uuid::from_bytes(&buffer).unwrap())
    }

    /// Tests if the UUID is nil
    pub fn is_nil(&self) -> bool {
        self.bytes.iter().all(|&b| b == 0)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use self::std::prelude::v1::*;
    use super::test_util;

    use super::ns::{
        NAMESPACE_X500, NAMESPACE_DNS, NAMESPACE_OID, NAMESPACE_URL,
    };

    use prelude::*;

    #[test]
    fn test_nil() {
        let nil = Uuid::nil();
        let not_nil = test_util::new();
        let from_bytes = Uuid::from_uuid_bytes([
            4, 54, 67, 12, 43, 2, 2, 76, 32, 50, 87, 5, 1, 33, 43, 87,
        ]);

        assert_eq!(from_bytes.get_version(), None);

        assert!(nil.is_nil());
        assert!(!not_nil.is_nil());

        assert_eq!(nil.get_version(), Some(UuidVersion::Nil));
        assert_eq!(not_nil.get_version(), Some(UuidVersion::Random))
    }

    #[test]
    fn test_predefined_namespaces() {
        assert_eq!(
            NAMESPACE_DNS.to_hyphenated().to_string(),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        );
        assert_eq!(
            NAMESPACE_URL.to_hyphenated().to_string(),
            "6ba7b811-9dad-11d1-80b4-00c04fd430c8"
        );
        assert_eq!(
            NAMESPACE_OID.to_hyphenated().to_string(),
            "6ba7b812-9dad-11d1-80b4-00c04fd430c8"
        );
        assert_eq!(
            NAMESPACE_X500.to_hyphenated().to_string(),
            "6ba7b814-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[cfg(feature = "v3")]
    #[test]
    fn test_get_version_v3() {
        let uuid = Uuid::new_v3(&NAMESPACE_DNS, "rust-lang.org");

        assert_eq!(uuid.get_version().unwrap(), UuidVersion::Md5);
        assert_eq!(uuid.get_version_num(), 3);
    }

    #[test]
    fn test_get_variant() {
        let uuid1 = test_util::new();
        let uuid2 =
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let uuid3 =
            Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").unwrap();
        let uuid4 =
            Uuid::parse_str("936DA01F9ABD4d9dC0C702AF85C822A8").unwrap();
        let uuid5 =
            Uuid::parse_str("F9168C5E-CEB2-4faa-D6BF-329BF39FA1E4").unwrap();
        let uuid6 =
            Uuid::parse_str("f81d4fae-7dec-11d0-7765-00a0c91e6bf6").unwrap();

        assert_eq!(uuid1.get_variant().unwrap(), UuidVariant::RFC4122);
        assert_eq!(uuid2.get_variant().unwrap(), UuidVariant::RFC4122);
        assert_eq!(uuid3.get_variant().unwrap(), UuidVariant::RFC4122);
        assert_eq!(uuid4.get_variant().unwrap(), UuidVariant::Microsoft);
        assert_eq!(uuid5.get_variant().unwrap(), UuidVariant::Microsoft);
        assert_eq!(uuid6.get_variant().unwrap(), UuidVariant::NCS);
    }

    #[test]
    fn test_parse_uuid_v4() {
        use super::ParseError::*;

        // Invalid
        assert_eq!(Uuid::parse_str(""), Err(InvalidLength(0)));
        assert_eq!(Uuid::parse_str("!"), Err(InvalidLength(1)));
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E45"),
            Err(InvalidLength(37))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-BBF-329BF39FA1E4"),
            Err(InvalidLength(35))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-BGBF-329BF39FA1E4"),
            Err(InvalidCharacter('G', 20))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2F4faaFB6BFF329BF39FA1E4"),
            Err(InvalidGroups(2))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faaFB6BFF329BF39FA1E4"),
            Err(InvalidGroups(3))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-B6BFF329BF39FA1E4"),
            Err(InvalidGroups(4))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa"),
            Err(InvalidLength(18))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faaXB6BFF329BF39FA1E4"),
            Err(InvalidCharacter('X', 18))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB-24fa-eB6BFF32-BF39FA1E4"),
            Err(InvalidGroupLength(1, 3, 4))
        );
        assert_eq!(
            Uuid::parse_str("01020304-1112-2122-3132-41424344"),
            Err(InvalidGroupLength(4, 8, 12))
        );
        assert_eq!(
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c"),
            Err(InvalidLength(31))
        );
        assert_eq!(
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c88"),
            Err(InvalidLength(33))
        );
        assert_eq!(
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0cg8"),
            Err(InvalidLength(33))
        );
        assert_eq!(
            Uuid::parse_str("67e5504410b1426%9247bb680e5fe0c8"),
            Err(InvalidCharacter('%', 15))
        );
        assert_eq!(
            Uuid::parse_str("231231212212423424324323477343246663"),
            Err(InvalidLength(36))
        );

        // Valid
        assert!(Uuid::parse_str("00000000000000000000000000000000").is_ok());
        assert!(
            Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8").is_ok()
        );
        assert!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4").is_ok()
        );
        assert!(Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c8").is_ok());
        assert!(
            Uuid::parse_str("01020304-1112-2122-3132-414243444546").is_ok()
        );
        assert!(
            Uuid::parse_str("urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8")
                .is_ok()
        );

        // Nil
        let nil = Uuid::nil();
        assert_eq!(
            Uuid::parse_str("00000000000000000000000000000000").unwrap(),
            nil
        );
        assert_eq!(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
            nil
        );

        // Round-trip
        let uuid_orig = test_util::new();
        let orig_str = uuid_orig.to_string();
        let uuid_out = Uuid::parse_str(&orig_str).unwrap();
        assert_eq!(uuid_orig, uuid_out);

        // Test error reporting
        assert_eq!(
            Uuid::parse_str("67e5504410b1426f9247bb680e5fe0c"),
            Err(InvalidLength(31))
        );
        assert_eq!(
            Uuid::parse_str("67e550X410b1426f9247bb680e5fe0cd"),
            Err(InvalidCharacter('X', 6))
        );
        assert_eq!(
            Uuid::parse_str("67e550-4105b1426f9247bb680e5fe0c"),
            Err(InvalidGroupLength(0, 6, 8))
        );
        assert_eq!(
            Uuid::parse_str("F9168C5E-CEB2-4faa-B6BF1-02BF39FA1E4"),
            Err(InvalidGroupLength(3, 5, 4))
        );
    }

    #[test]
    fn test_to_simple_string() {
        let uuid1 = test_util::new();
        let s = uuid1.to_simple().to_string();

        assert_eq!(s.len(), 32);
        assert!(s.chars().all(|c| c.is_digit(16)));
    }

    #[test]
    fn test_to_hyphenated_string() {
        let uuid1 = test_util::new();
        let s = uuid1.to_hyphenated().to_string();

        assert!(s.len() == 36);
        assert!(s.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_upper_lower_hex() {
        use super::fmt::Write;

        let mut buf = String::new();
        let u = test_util::new();

        macro_rules! check {
            ($buf:ident, $format:expr, $target:expr, $len:expr, $cond:expr) => {
                $buf.clear();
                write!($buf, $format, $target).unwrap();
                assert!(buf.len() == $len);
                assert!($buf.chars().all($cond), "{}", $buf);
            };
        }

        check!(buf, "{:X}", u, 36, |c| c.is_uppercase()
            || c.is_digit(10)
            || c == '-');
        check!(buf, "{:X}", u.to_hyphenated(), 36, |c| c.is_uppercase()
            || c.is_digit(10)
            || c == '-');
        check!(buf, "{:X}", u.to_simple(), 32, |c| c.is_uppercase()
            || c.is_digit(10));

        check!(buf, "{:x}", u.to_hyphenated(), 36, |c| c.is_lowercase()
            || c.is_digit(10)
            || c == '-');
        check!(buf, "{:x}", u.to_simple(), 32, |c| c.is_lowercase()
            || c.is_digit(10));
    }

    #[test]
    fn test_to_urn_string() {
        let uuid1 = test_util::new();
        let ss = uuid1.to_urn().to_string();
        let s = &ss[9..];

        assert!(ss.starts_with("urn:uuid:"));
        assert_eq!(s.len(), 36);
        assert!(s.chars().all(|c| c.is_digit(16) || c == '-'));
    }

    #[test]
    fn test_to_simple_string_matching() {
        let uuid1 = test_util::new();

        let hs = uuid1.to_hyphenated().to_string();
        let ss = uuid1.to_simple().to_string();

        let hsn = hs.chars().filter(|&c| c != '-').collect::<String>();

        assert_eq!(hsn, ss);
    }

    #[test]
    fn test_string_roundtrip() {
        let uuid = test_util::new();

        let hs = uuid.to_hyphenated().to_string();
        let uuid_hs = Uuid::parse_str(&hs).unwrap();
        assert_eq!(uuid_hs, uuid);

        let ss = uuid.to_string();
        let uuid_ss = Uuid::parse_str(&ss).unwrap();
        assert_eq!(uuid_ss, uuid);
    }

    #[test]
    fn test_from_fields() {
        let d1: u32 = 0xa1a2a3a4;
        let d2: u16 = 0xb1b2;
        let d3: u16 = 0xc1c2;
        let d4 = [0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

        let u = Uuid::from_fields(d1, d2, d3, &d4).unwrap();

        let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
        let result = u.to_simple().to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_as_fields() {
        let u = test_util::new();
        let (d1, d2, d3, d4) = u.as_fields();

        assert_ne!(d1, 0);
        assert_ne!(d2, 0);
        assert_ne!(d3, 0);
        assert_eq!(d4.len(), 8);
        assert!(!d4.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_fields_roundtrip() {
        let d1_in: u32 = 0xa1a2a3a4;
        let d2_in: u16 = 0xb1b2;
        let d3_in: u16 = 0xc1c2;
        let d4_in = &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8];

        let u = Uuid::from_fields(d1_in, d2_in, d3_in, d4_in).unwrap();
        let (d1_out, d2_out, d3_out, d4_out) = u.as_fields();

        assert_eq!(d1_in, d1_out);
        assert_eq!(d2_in, d2_out);
        assert_eq!(d3_in, d3_out);
        assert_eq!(d4_in, d4_out);
    }

    #[test]
    fn test_from_bytes() {
        let b = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
            0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
        ];

        let u = Uuid::from_bytes(&b).unwrap();
        let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";

        assert_eq!(u.to_simple().to_string(), expected);
    }

    #[test]
    fn test_from_uuid_bytes() {
        let b = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
            0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
        ];

        let u = Uuid::from_uuid_bytes(b);
        let expected = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";

        assert_eq!(u.to_simple().to_string(), expected);
    }

    #[test]
    fn test_as_bytes() {
        let u = test_util::new();
        let ub = u.as_bytes();

        assert_eq!(ub.len(), 16);
        assert!(!ub.iter().all(|&b| b == 0));
    }

    #[test]
    fn test_bytes_roundtrip() {
        let b_in: [u8; 16] = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
            0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
        ];

        let u = Uuid::from_bytes(&b_in).unwrap();

        let b_out = u.as_bytes();

        assert_eq!(&b_in, b_out);
    }

    #[test]
    fn test_from_random_bytes() {
        let b = [
            0xa1, 0xa2, 0xa3, 0xa4, 0xb1, 0xb2, 0xc1, 0xc2, 0xd1, 0xd2, 0xd3,
            0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
        ];

        let u = Uuid::from_random_bytes(b);
        let expected = "a1a2a3a4b1b241c291d2d3d4d5d6d7d8";

        assert_eq!(u.to_simple().to_string(), expected);
    }

    #[test]
    fn test_iterbytes_impl_for_uuid() {
        let mut set = std::collections::HashSet::new();
        let id1 = test_util::new();
        let id2 = test_util::new2();
        set.insert(id1.clone());

        assert!(set.contains(&id1));
        assert!(!set.contains(&id2));
    }
}
