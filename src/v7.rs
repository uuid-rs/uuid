//! The implementation for Version 7 UUIDs.
//!
//! Note that you need to enable the `v7` Cargo feature
//! in order to use this module.

use crate::{rng, timestamp::Timestamp, Builder, NoContext, Uuid};
use core::convert::TryInto;

impl Uuid {
    /// Create a new UUID (version 7) using the current time value and random bytes.
    ///
    /// This method is a convenient alternative to [`Uuid::new_v7`] that uses the current system time
    /// as the source timestamp.
    #[cfg(feature = "std")]
    pub fn now_v7() -> Self {
        Self::new_v7(Timestamp::now(NoContext))
    }

    /// Create a new UUID (version 7) using a time value and random bytes.
    ///
    /// When the `std` feature is enabled, you can also use [`Uuid::now_v7`].
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
    /// # References
    ///
    /// * [Version 7 UUIDs in Draft RFC: New UUID Formats, Version 4](https://datatracker.ietf.org/doc/html/draft-peabody-dispatch-new-uuid-format-04#section-5.2)
    pub fn new_v7(ts: Timestamp) -> Self {
        let (secs, nanos) = ts.to_unix();
        let millis = secs.saturating_add(nanos as u64 / 1_000_000);

        Builder::from_unix_timestamp_millis(millis, &rng::bytes()[..10].try_into().unwrap())
            .into_uuid()
    }
}

pub(crate) const fn encode_unix_timestamp_millis(millis: u64, random_bytes: &[u8; 10]) -> Uuid {
    let millis_high = ((millis >> 16) & 0xFFFF_FFFF) as u32;
    let millis_low = (millis & 0xFFFF) as u16;

    let random_and_version =
        (random_bytes[0] as u16 | ((random_bytes[1] as u16) << 8) & 0x0FFF) | (0x7 << 12);

    let mut d4 = [0; 8];

    d4[0] = (random_bytes[2] & 0x3F) | 0x80;
    d4[1] = random_bytes[3];
    d4[2] = random_bytes[4];
    d4[3] = random_bytes[5];
    d4[4] = random_bytes[6];
    d4[5] = random_bytes[7];
    d4[6] = random_bytes[8];
    d4[7] = random_bytes[9];

    Uuid::from_fields(millis_high, millis_low, random_and_version, &d4)
}

pub(crate) const fn decode_unix_timestamp_millis(uuid: &Uuid) -> u64 {
    let bytes = uuid.as_bytes();

    let millis: u64 = (bytes[0] as u64) << 40
        | (bytes[1] as u64) << 32
        | (bytes[2] as u64) << 24
        | (bytes[3] as u64) << 16
        | (bytes[4] as u64) << 8
        | (bytes[5] as u64);

    millis
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{NoContext, Variant, Version};
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

        assert_eq!(uuid, parsed);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_new_v7_timestamp_roundtrip() {
        let time: u64 = 1_496_854_535;
        let time_fraction: u32 = 812_946_000;

        let ts = Timestamp::from_unix(NoContext, time, time_fraction);

        let uuid = Uuid::new_v7(ts);

        let decoded_ts = uuid.get_timestamp().unwrap();

        assert_eq!(ts.to_unix(), decoded_ts.to_unix());
    }
}
