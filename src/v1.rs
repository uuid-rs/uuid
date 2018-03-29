use prelude::*;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::sync::atomic;
    } else if #[cfg(not(feature = "v1"))] {
        use core::sync::atomic;
    }
}

/// A trait that abstracts over generation of Uuid v1 "Clock Sequence" values.
pub trait UuidClockSequence {
    /// Return a 16-bit number that will be used as the "clock sequence" in
    /// the Uuid. The number must be different if the time has changed since
    /// the last time a clock sequence was requested.
    fn generate_sequence(&self, seconds: u64, nano_seconds: u32) -> u16;
}

/// A thread-safe, stateful context for the v1 generator to help ensure
/// process-wide uniqueness. Uses `AtomicUsize` as the counter.
pub struct AtomicUsizeUuidV1Context {
    count: atomic::AtomicUsize,
}

/// The number of 100 ns ticks between the UUID epoch `1582-10-15 00:00:00` and
/// the Unix epoch `1970-01-01 00:00:00`.
const UUID_TICKS_BETWEEN_EPOCHS: u64 = 0x01B2_1DD2_1381_4000;

impl AtomicUsizeUuidV1Context {
    /// Creates a thread-safe, internally mutable context to help ensure
    /// uniqueness.
    ///
    /// This is a context which can be shared across threads. It maintains an
    /// internal counter that is incremented at every request, the value ends
    /// up in the clock_seq portion of the Uuid (the fourth group). This will
    /// improve the probability that the Uuid is unique across the process.
    pub fn new(count: u16) -> Self {
        AtomicUsizeUuidV1Context {
            count: atomic::AtomicUsize::new(count as usize),
        }
    }
}

impl Uuid {
    /// Creates a new `Uuid` (version 1 style) using a time value + seq + NodeID.
    ///
    /// This expects two values representing a monotonically increasing value
    /// as well as a unique 6 byte NodeId, and an implementation of `UuidV1ClockSequence`.
    /// This function is only guaranteed to produce unique values if the following conditions hold:
    ///
    /// 1. The NodeID is unique for this process,
    /// 2. The Context is shared across all threads which are generating V1 UUIDs,
    /// 3. The `UuidV1ClockSequence` implementation reliably returns unique clock sequences
    ///    (this crate provides `UuidV1Context` for this purpose).
    ///
    /// The NodeID must be exactly 6 bytes long. If the NodeID is not a valid length
    /// this will return a `ParseError::InvalidLength`.
    ///
    /// The function is not guaranteed to produce monotonically increasing values
    /// however.  There is a slight possibility that two successive equal time values
    /// could be supplied and the sequence counter wraps back over to 0.
    ///
    /// If uniqueness and monotonicity is required, the user is responsibile for ensuring
    /// that the time value always increases between calls
    /// (including between restarts of the process and device).
    ///
    /// Note that usage of this method requires the `v1` feature of this crate
    /// to be enabled.
    ///
    /// # Examples
    /// Basic usage:
    ///
    /// ```
    /// use uuid::{Uuid, UuidV1Context};
    ///
    /// let ctx = UuidV1Context::new(42);
    /// let v1uuid = Uuid::new_v1(&ctx, 1497624119, 1234, &[1,2,3,4,5,6]).unwrap();
    ///
    /// assert_eq!(v1uuid.hyphenated().to_string(), "f3b4958c-52a1-11e7-802a-010203040506");
    /// ```
    pub fn new_v1<C: UuidClockSequence>(
        context: &C,
        seconds: u64,
        nano_seconds: u32,
        node: &[u8],
    ) -> Result<Self, ParseError> {
        if node.len() != 6 {
            return Err(ParseError::InvalidLength(node.len()))
        }

        let count = context.generate_sequence(seconds, nano_seconds);
        let timestamp = seconds * 10_000_000 + u64::from(nano_seconds / 100);
        let uuid_time = timestamp + UUID_TICKS_BETWEEN_EPOCHS;
        let time_low: u32 = (uuid_time & 0xFFFF_FFFF) as u32;
        let time_mid: u16 = ((uuid_time >> 32) & 0xFFFF) as u16;
        let time_high_and_ver: u16 = (((uuid_time >> 48) & 0x0FFF) as u16) | (1 << 12);
        let mut d4 =[0_u8; 8];
        d4[0] = (((count & 0x3F00) >> 8) as u8) | 0x80;
        d4[1] = (count & 0xFF) as u8;
        d4[2..].copy_from_slice(node);
        Uuid::from_fields(time_low, time_mid, time_high_and_ver, &d4)
    }
}

impl UuidClockSequence for AtomicUsizeUuidV1Context {
    fn generate_sequence(&self, _: u64, _: u32) -> u16 {
        (self.count.fetch_add(1, atomic::Ordering::SeqCst) & 0xffff) as u16
    }
}
