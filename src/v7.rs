//! The implementation for Version 7 UUIDs.
//!
//! Note that you need to enable the `v7` Cargo feature
//! in order to use this module.

use crate::{rng, timestamp::Timestamp, Builder, Uuid};

impl Uuid {
    /// Create a new version 7 UUID using the current time value.
    ///
    /// This method is a convenient alternative to [`Uuid::new_v7`] that uses the current system time
    /// as the source timestamp. All UUIDs generated through this method by the same process are
    /// guaranteed to be ordered by their creation.
    #[cfg(feature = "std")]
    pub fn now_v7() -> Self {
        Self::new_v7(Timestamp::now(
            crate::timestamp::context::shared_context_v7(),
        ))
    }

    /// Create a new version 7 UUID using a time value and random bytes.
    ///
    /// When the `std` feature is enabled, you can also use [`Uuid::now_v7`].
    ///
    /// Note that usage of this method requires the `v7` feature of this crate
    /// to be enabled.
    ///
    /// Also see [`Uuid::now_v7`] for a convenient way to generate version 7
    /// UUIDs using the current system time.
    ///
    /// # Examples
    ///
    /// A v7 UUID can be created from a unix [`Timestamp`] plus a 128 bit
    /// random number. When supplied as such, the data will be combined
    /// to ensure uniqueness and sortability at millisecond granularity.
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
    /// A v7 UUID can also be created with a counter to ensure batches of
    /// UUIDs created together remain sortable:
    ///
    /// ```rust
    /// # use uuid::{Uuid, Timestamp, ContextV7};
    /// let context = ContextV7::new();
    /// let uuid1 = Uuid::new_v7(Timestamp::from_unix(&context, 1497624119, 1234));
    /// let uuid2 = Uuid::new_v7(Timestamp::from_unix(&context, 1497624119, 1234));
    ///
    /// assert!(uuid1 < uuid2);
    /// ```
    ///
    /// # References
    ///
    /// * [UUID Version 7 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.7)
    pub fn new_v7(ts: Timestamp) -> Self {
        let (secs, nanos) = ts.to_unix();
        let millis = secs
            .saturating_mul(1000)
            .saturating_add(nanos as u64 / 1_000_000);

        let (mut counter, counter_bits) = ts.counter();

        debug_assert!(counter_bits <= 128);

        let mut counter_bits = counter_bits as u32;

        // If the counter intersects the variant field then shift around it.
        // This ensures that any bits set in the counter that would intersect
        // the variant are still preserved
        if counter_bits > 12 {
            let mask = u128::MAX << (counter_bits - 12);

            counter = (counter & !mask) | ((counter & mask) << 2);

            counter_bits += 2;
        }

        let counter_and_random = match counter_bits {
            0 => {
                // The counter doesn't contribute any bits
                rng::u128()
            }
            ..128 => {
                // The counter assigns some number of bits
                let mut counter_and_random = rng::u128();

                counter_and_random &= u128::MAX >> counter_bits;
                counter_and_random |= counter << (128 - counter_bits);

                counter_and_random
            }
            128.. => {
                // The counter overrides all bits
                counter
            }
        };

        Builder::from_unix_timestamp_millis(
            millis,
            &counter_and_random.to_be_bytes()[..10].try_into().unwrap(),
        )
        .into_uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{std::string::ToString, ClockSequence, NoContext, Variant, Version};

    #[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    fn test_new() {
        let ts: u64 = 1645557742000;

        let seconds = ts / 1000;
        let nanos = ((ts % 1000) * 1_000_000) as u32;

        let uuid = Uuid::new_v7(Timestamp::from_unix(NoContext, seconds, nanos));
        let uustr = uuid.hyphenated().to_string();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
        assert_eq!(uuid.get_variant(), Variant::RFC4122);
        assert!(uuid.hyphenated().to_string().starts_with("017f22e2-79b0-7"));

        // Ensure parsing the same UUID produces the same timestamp
        let parsed = Uuid::parse_str(uustr.as_str()).unwrap();

        assert_eq!(uuid, parsed);
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    #[cfg(feature = "std")]
    fn test_now() {
        let uuid = Uuid::now_v7();

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
        assert_eq!(uuid.get_variant(), Variant::RFC4122);
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    fn test_sorting() {
        let time1: u64 = 1_496_854_535;
        let time_fraction1: u32 = 812_000_000;

        let time2 = time1 + 4000;
        let time_fraction2 = time_fraction1;

        let uuid1 = Uuid::new_v7(Timestamp::from_unix(NoContext, time1, time_fraction1));
        let uuid2 = Uuid::new_v7(Timestamp::from_unix(NoContext, time2, time_fraction2));

        assert!(uuid1.as_bytes() < uuid2.as_bytes());
        assert!(uuid1.to_string() < uuid2.to_string());
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    fn test_new_timestamp_roundtrip() {
        let time: u64 = 1_496_854_535;
        let time_fraction: u32 = 812_000_000;

        let ts = Timestamp::from_unix(NoContext, time, time_fraction);

        let uuid = Uuid::new_v7(ts);

        let decoded_ts = uuid.get_timestamp().unwrap();

        assert_eq!(ts.to_unix(), decoded_ts.to_unix());
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    fn test_new_max_context() {
        struct MaxContext;

        impl ClockSequence for MaxContext {
            type Output = u128;

            fn generate_sequence(&self, _seconds: u64, _nanos: u32) -> Self::Output {
                u128::MAX
            }

            fn usable_bits(&self) -> usize {
                128
            }
        }

        let time: u64 = 1_496_854_535;
        let time_fraction: u32 = 812_000_000;

        // Ensure we don't overflow here
        let ts = Timestamp::from_unix(MaxContext, time, time_fraction);

        let uuid = Uuid::new_v7(ts);

        assert_eq!(uuid.get_version(), Some(Version::SortRand));
        assert_eq!(uuid.get_variant(), Variant::RFC4122);

        let decoded_ts = uuid.get_timestamp().unwrap();

        assert_eq!(ts.to_unix(), decoded_ts.to_unix());
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    fn test_new_min_counter() {
        let zero_width = Timestamp::from_unix_time(1_700_000_000, 0, u128::MAX, 0);

        let zero_a = Uuid::new_v7(zero_width);
        let zero_b = Uuid::new_v7(zero_width);

        assert_ne!(zero_a.as_fields().3, zero_b.as_fields().3);
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    fn test_new_max_counter() {
        let full_width = Timestamp::from_unix_time(1_700_000_000, 0, 0, 128);

        let full_a = Uuid::new_v7(full_width);
        let full_b = Uuid::new_v7(full_width);

        assert_eq!(full_a.as_fields().3, full_b.as_fields().3);
    }

    #[test]
    #[cfg_attr(
        all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")),
        wasm_bindgen_test
    )]
    fn test_new_max() {
        let ts = Timestamp::from_unix_time(u64::MAX, 0, 0, 0);
        let uuid = Uuid::new_v7(ts);

        let decoded_ts = uuid.get_timestamp().unwrap();

        assert_eq!((281474976710, 655000000), decoded_ts.to_unix());
    }
}
