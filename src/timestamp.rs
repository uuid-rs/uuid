/// Contains the `Timestamp` struct and `ClockSequence` traits
/// as well as an implementation of ClockSequence for Context (only for features v1 and v6)

/// The number of 100 ns ticks between the UUID epoch
/// `1582-10-15 00:00:00` and the Unix epoch `1970-01-01 00:00:00`.
pub const UUID_TICKS_BETWEEN_EPOCHS: u64 = 0x01B2_1DD2_1381_4000;
/// Stores the number of seconds since epoch,
/// as well as the fractional nanoseconds of that second
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Timestamp {
    pub(crate) seconds: u64,
    pub(crate) nanos: u32,
}

impl Timestamp {
    /// Construct a `Timestamp` from its raw component values: an RFC4122
    /// timestamp and counter.
    ///
    /// RFC4122, which defines the V1 UUID, specifies a 60-byte timestamp format
    /// as the number of 100-nanosecond intervals elapsed since 00:00:00.00,
    /// 15 Oct 1582, "the date of the Gregorian reform of the Christian
    /// calendar."
    pub const fn from_rfc4122(ticks: u64) -> Self {
        let (seconds, nanos) = Self::rfc4122_to_unix(ticks);
        Timestamp { seconds, nanos }
    }

    /// Construct a `Timestamp` from a unix timestamp
    ///
    /// A unix timestamp represents the elapsed time since Jan 1 1970. Libc's
    /// `clock_gettime` and other popular implementations traditionally
    /// represent this duration as a `timespec`: a struct with `u64` and
    /// `u32` fields representing the seconds, and "subsecond" or fractional
    /// nanoseconds elapsed since the timestamp's second began,
    /// respectively.
    pub fn from_unix(seconds: u64, nanos: u32) -> Self {
        Timestamp { seconds, nanos }
    }

    /// Construct a `Timestamp` from the current time of day
    /// according to Rust's SystemTime
    pub fn now() -> Self {
        let dur = std::time::SystemTime::UNIX_EPOCH
            .elapsed()
            .expect("Getting elapsed time since UNIX_EPOCH.  If this fails, we've somehow violated causality");
        Timestamp {
            seconds: dur.as_secs(),
            nanos: dur.subsec_nanos(),
        }
    }

    /// Returns the raw RFC4122 timestamp "tick" values stored by the
    /// `Timestamp`.
    ///
    /// The ticks represent the  number of 100-nanosecond intervals
    /// since 00:00:00.00, 15 Oct 1582.
    pub const fn to_rfc4122(&self) -> u64 {
        Self::unix_to_rfc4122_ticks(self.seconds, self.nanos)
    }

    /// Returns the timestamp converted to the seconds and fractional
    /// nanoseconds since Jan 1 1970.
    pub const fn to_unix(&self) -> (u64, u32) {
        (self.seconds, self.nanos)
    }

    /// Returns the timestamp converted into nanoseconds elapsed since Jan 1
    /// 1970.
    pub const fn to_unix_nanos(&self) -> u32 {
        self.nanos
    }

    /// internal utility functions for converting between Unix and Uuid-epoch
    /// convert unix-timestamp into rfc4122 ticks
    const fn unix_to_rfc4122_ticks(seconds: u64, nanos: u32) -> u64 {
        let ticks = UUID_TICKS_BETWEEN_EPOCHS
            + seconds * 10_000_000
            + nanos as u64 / 100;

        ticks
    }

    /// convert rfc4122 ticks into unix-timestamp
    const fn rfc4122_to_unix(ticks: u64) -> (u64, u32) {
        (
            (ticks - UUID_TICKS_BETWEEN_EPOCHS) / 10_000_000,
            ((ticks - UUID_TICKS_BETWEEN_EPOCHS) % 10_000_000) as u32 * 100,
        )
    }
}

/// A trait that abstracts over generation of UUID v1 "Clock Sequence" values.
///
/// # References
///
/// * [Clock Sequence in RFC4122](https://datatracker.ietf.org/doc/html/rfc4122#section-4.1.5)
pub trait ClockSequence {
    /// The primitive type you wish out output
    type Output;
    /// Return an arbitrary width number that will be used as the "clock sequence" in
    /// the UUID. The number must be different if the time has changed since
    /// the last time a clock sequence was requested.
    fn next(&self, ts: &Timestamp) -> Self::Output;
}

impl<'a, T: ClockSequence + ?Sized> ClockSequence for &'a T {
    type Output = T::Output;
    fn next(&self, ts: &Timestamp) -> Self::Output {
        (**self).next(ts)
    }
}

/// For features v1 and v1, constructs a `Context` struct which implements the `ClockSequence` trait
#[cfg(any(feature = "v1", feature = "v6"))]
pub mod context {
    use std::sync::atomic::{AtomicU16, Ordering};
    /// A thread-safe, stateful context for the v1 generator to help ensure
    /// process-wide uniqueness.
    #[derive(Debug)]
    pub struct Context {
        count: AtomicU16,
    }

    impl Context {
        /// Creates a thread-safe, internally mutable context to help ensure
        /// uniqueness.
        ///
        /// This is a context which can be shared across threads. It maintains an
        /// internal counter that is incremented at every request, the value ends
        /// up in the clock_seq portion of the UUID (the fourth group). This
        /// will improve the probability that the UUID is unique across the
        /// process.
        pub const fn new(count: u16) -> Self {
            Self {
                count: AtomicU16::new(count),
            }
        }

        /// Creates a thread-safe, internally mutable context that's seeded with a
        /// random value.
        ///
        /// This method requires either the `rng` or `fast-rng` feature to also be
        /// enabled.
        ///
        /// This is a context which can be shared across threads. It maintains an
        /// internal counter that is incremented at every request, the value ends
        /// up in the clock_seq portion of the UUID (the fourth group). This
        /// will improve the probability that the UUID is unique across the
        /// process.
        #[cfg(feature = "rng")]
        pub fn new_random() -> Self {
            Self {
                count: AtomicU16::new(crate::rng::u16()),
            }
        }
    }

    impl super::ClockSequence for Context {
        type Output = u16;
        fn next(&self, _: &super::Timestamp) -> Self::Output {
            // RFC4122 reserves 2 bits of the clock sequence so the actual
            // maximum value is smaller than `u16::MAX`. Since we unconditionally
            // increment the clock sequence we want to wrap once it becomes larger
            // than what we can represent in a "u14". Otherwise there'd be patches
            // where the clock sequence doesn't change regardless of the timestamp
            self.count.fetch_add(1, Ordering::AcqRel) % (u16::MAX >> 2)
        }
    }
}
