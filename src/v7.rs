//! The implementation for Version 7 UUIDs.
//!
//! Note that you need to enable the `v7` Cargo feature
//! in order to use this module.

use crate::rng::{bytes, u16};
use crate::timestamp::Timestamp;
use crate::Uuid;
use core::convert::TryInto;

impl Uuid {
    /// Create a new UUID (version 7) using a time value + random number
    ///
    /// Note that usage of this method requires the `v7` feature of this crate
    /// to be enabled.
    ///
    /// # Examples
    ///
    /// A v7 UUID can be created from a unix [`Timestamp`] plus a 128 bit
    /// random number. When supplied as such, the data will be
    ///
    /// ```rust
    /// # use uuid::{Uuid, Timestamp, NoContext};
    /// let ts = Timestamp::from_unix(NoContext, 1497624119, 1234);
    ///
    /// let uuid = Uuid::new_v7(ts);
    ///
    /// assert!(
    ///     uuid.hyphenated().to_string().starts_with("015cb15a-86d8-7")
    /// );
    /// ```
    ///
    /// The timestamp can also be created automatically from the current SystemTime
    ///
    /// let ts = Timestamp::now();
    ///
    /// let uuid = Uuid::new_v7(ts);
    ///
    /// [`Timestamp`]: v1/struct.Timestamp.html
    pub fn new_v7(ts: Timestamp) -> Self {
        let millis = ts.seconds * 1000 + (ts.nanos as u64) / 1_000_000;
        let ms_high = ((millis >> 16) & 0xFFFF_FFFF) as u32;
        let ms_low = (millis & 0xFFFF) as u16;
        let ver_rand = u16() & 0xFFF | (0x7 << 12);
        let mut rnd = bytes();
        rnd[0] = (rnd[0] & 0x3F) | 0x80;
        let buf: [u8; 8] = (&rnd[0..8]).try_into().unwrap();
        Uuid::from_fields(ms_high, ms_low, ver_rand, &buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Variant, Version, NoContext};
    use std::string::ToString;
    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_new_v7() {
        let time: u64 = 1_496_854_535;
        let time_fraction: u32 = 812_946_000;

        let uuid = Uuid::new_v7(Timestamp::from_unix(NoContext, time, time_fraction));
        let uustr = uuid.hyphenated().to_string();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
        assert_eq!(uuid.get_variant(), Variant::RFC4122);
        assert!(uustr.starts_with("015c837b-9e84-7"));

        // Ensure parsing the same UUID produces the same timestamp
        let parsed = Uuid::parse_str(uustr.as_str()).unwrap();

        assert_eq!(uuid, parsed,);
    }
}
