//! The implementation for Version 1 UUIDs.
//!
//! Note that you need to enable the `v1` Cargo feature
//! in order to use this module.

use crate::timestamp::{ClockSequence, Timestamp};
use crate::Uuid;

/// The Context implementation is specific to Uuids v1 and v6
pub use crate::timestamp::context::Context;

impl Uuid {
    /// Create a new UUID (version 6) using a time value + sequence +
    /// *NodeId*.
    /// This is similar to UUIDv1, except that it is lexographically sortable by timestamp.
    ///
    /// When generating [`Timestamp`]s using a [`ClockSequence`], this function
    /// is only guaranteed to produce unique values if the following conditions
    /// hold:
    ///
    /// 1. The *NodeId* is unique for this process,
    /// 2. The *Context* is shared across all threads which are generating v1
    ///    UUIDs,
    /// 3. The [`ClockSequence`] implementation reliably returns unique
    ///    clock sequences (this crate provides [`Context`] for this
    ///    purpose. However you can create your own [`ClockSequence`]
    ///    implementation, if [`Context`] does not meet your needs).
    ///
    /// The NodeID must be exactly 6 bytes long.
    ///
    /// Note that usage of this method requires the `v6` feature of this crate
    /// to be enabled.
    ///
    /// # Examples
    ///
    /// A UUID can be created from a unix [`Timestamp`] with a
    /// [`ClockSequence`]. RFC4122 requires the clock sequence
    /// is seeded with a random value:
    ///
    /// ```rust
    /// # use uuid::{Uuid, Timestamp, Context};
    /// # fn random_seed() -> u16 { 42 }
    /// let context = Context::new(random_seed());
    /// let ts = Timestamp::from_unix(context, 1497624119, 1234);
    ///
    /// let uuid = Uuid::new_v6(ts, &[1, 2, 3, 4, 5, 6]);
    ///
    /// assert_eq!(
    ///     uuid.hyphenated().to_string(),
    ///     "1e752a1f-3b49-658c-802a-010203040506"
    /// );
    /// ```
    ///
    /// The timestamp can also be created manually as per RFC4122:
    ///
    /// ```
    /// # use uuid::{Uuid, Timestamp, Context, ClockSequence};
    /// # fn random_seed() -> u16 { 42 }
    /// let context = Context::new(random_seed());
    /// let ts = Timestamp::from_rfc4122(14976241191231231313, context.generate_sequence(0, 0) );
    ///
    /// let uuid = Uuid::new_v6(ts, &[1, 2, 3, 4, 5, 6]);
    ///
    /// assert_eq!(
    ///     uuid.hyphenated().to_string(),
    ///     "fd64c041-1e91-6551-802a-010203040506"
    /// );
    /// ```
    /// The timestamp can also be created automatically from the current SystemTime
    ///
    /// # use uuid::{Uuid, Timestamp, Context};
    /// let context = Context::new(42);
    /// let ts = Timestamp::from_rfc4122(14976241191231231313);
    ///
    /// let uuid = Uuid::new_v6(ts, &context, &[1, 2, 3, 4, 5, 6]);
    ///
    /// [`Timestamp`]: v1/struct.Timestamp.html
    /// [`ClockSequence`]: v1/trait.ClockSequence.html
    /// [`Context`]: v1/struct.Context.html
    pub fn new_v6(ts: Timestamp, node_id: &[u8; 6]) -> Self {
        let (ticks, counter) = ts.to_rfc4122();
        let time_high = ((ticks >> 28) & 0xFFFF_FFFF) as u32;
        let time_mid = ((ticks >> 12) & 0xFFFF) as u16;
        let time_low_and_version = ((ticks & 0x0FFF) as u16) | (0x6 << 12);

        let mut d4 = [0; 8];

        d4[0] = (((counter & 0x3F00) >> 8) as u8) | 0x80;
        d4[1] = (counter & 0xFF) as u8;
        d4[2] = node_id[0];
        d4[3] = node_id[1];
        d4[4] = node_id[2];
        d4[5] = node_id[3];
        d4[6] = node_id[4];
        d4[7] = node_id[5];

        Uuid::from_fields(time_high, time_mid, time_low_and_version, &d4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Variant, Version};
    use std::string::ToString;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_new_v6() {
        let time: u64 = 1_496_854_535;
        let time_fraction: u32 = 812_946_000;
        let node = [1, 2, 3, 4, 5, 6];
        let context = Context::new(0);

        let uuid = Uuid::new_v6(Timestamp::from_unix(context, time, time_fraction), &node);

        assert_eq!(uuid.get_version(), Some(Version::SortMac));
        assert_eq!(uuid.get_variant(), Variant::RFC4122);
        assert_eq!(
            uuid.hyphenated().to_string(),
            "1e74ba22-0616-6934-8000-010203040506"
        );

        let ts = uuid.get_timestamp().unwrap().to_rfc4122();

        assert_eq!(ts.0 - 0x01B2_1DD2_1381_4000, 14_968_545_358_129_460);

        // Ensure parsing the same UUID produces the same timestamp
        let parsed = Uuid::parse_str("1e74ba22-0616-6934-8000-010203040506").unwrap();

        assert_eq!(
            uuid.get_timestamp().unwrap(),
            parsed.get_timestamp().unwrap()
        );
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_new_v6_context() {
        let time: u64 = 1_496_854_535;
        let time_fraction: u32 = 812_946_000;
        let node = [1, 2, 3, 4, 5, 6];

        // This context will wrap
        let context = Context::new((u16::MAX >> 2) - 1);

        let uuid1 = Uuid::new_v6(Timestamp::from_unix(&context, time, time_fraction), &node);

        let time: u64 = 1_496_854_536;

        let uuid2 = Uuid::new_v6(Timestamp::from_unix(&context, time, time_fraction), &node);

        assert_eq!(uuid1.get_timestamp().unwrap().to_rfc4122().1, 16382);
        assert_eq!(uuid2.get_timestamp().unwrap().to_rfc4122().1, 0);

        let time = 1_496_854_535;

        let uuid3 = Uuid::new_v6(Timestamp::from_unix(&context, time, time_fraction), &node);
        let uuid4 = Uuid::new_v6(Timestamp::from_unix(&context, time, time_fraction), &node);

        assert_eq!(uuid3.get_timestamp().unwrap().counter, 1);
        assert_eq!(uuid4.get_timestamp().unwrap().counter, 2);
    }
}
