//! The implementation for Version 7 UUIDs.
//!
//! Note that you need to enable the `v7` Cargo feature
//! in order to use this module.

use crate::{Builder, NoContext, rng, timestamp::Timestamp, Uuid};
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
    /// This method will use millisecond precision for the timestamp and fill the
    /// rest with random data.
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
    /// ```
    /// # use uuid::{Uuid, Timestamp, NoContext};
    /// let ts = Timestamp::now(NoContext);
    ///
    /// let uuid = Uuid::new_v7(ts);
    /// ```
    pub fn new_v7(ts: Timestamp) -> Self {
        let buf: &[u8] = &rng::bytes()[0..11];

        Builder::from_timestamp_millis(ts, buf.try_into().unwrap()).into_uuid()
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
