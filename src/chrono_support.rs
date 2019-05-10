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

use crate::prelude::*;
use chrono::{DateTime, TimeZone, Utc};

/// The number of 100 ns ticks between the UUID epoch
/// `1582-10-15 00:00:00` and the Unix epoch `1970-01-01 00:00:00`.
const UUID_TICKS_BETWEEN_EPOCHS: i64 = 0x01B2_1DD2_1381_4000;

impl Uuid {
    /// Returns an Optional `DateTime<Utc>` from the timestamp embedded
    /// in a v1 UUID. The timestamp (available in raw form via `to_timestamp`)
    /// is stored in a V1 UUID as the number of 100 nanosecond intervals since
    /// midnight 15 October 1582. If the supplied UUID is not V1, this will
    /// return None.
    pub fn to_utc(&self) -> Option<DateTime<Utc>> {
        self.to_timestamp().map(|(ts, _)| {
            // convert from 1582 epoch to 1970 epoch
            let ticks = ts as i64 - UUID_TICKS_BETWEEN_EPOCHS;
            // div/mod by 10_000_000 (instead of 1_000_000_000) because the
            // ticks are 100ns intervals.
            let seconds = ticks / 10_000_000;
            let subsec_nanos = (ticks % 10_000_000).abs() as u32 * 100;
            Utc.timestamp(seconds, subsec_nanos)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_to_chrono_utc_recent() {
        let ctx = Context::new(1);
        let node_id = &[1, 2, 3, 4, 5, 6];

        // at epoch
        assert_eq!(
            Uuid::new_v1(&ctx, 0, 0, node_id).unwrap().to_utc(),
            Some(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0))
        );

        // < 100ns from epoch
        assert_eq!(
            Uuid::new_v1(&ctx, 0, 99, node_id).unwrap().to_utc(),
            Some(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0))
        );

        // exactly 100ns from epoch (sanity check to_timestamp)
        assert_eq!(
            Uuid::new_v1(&ctx, 0, 100, node_id)
                .unwrap()
                .to_timestamp()
                .unwrap()
                .0 as i64,
            UUID_TICKS_BETWEEN_EPOCHS + 1
        );

        // exactly 100ns from epoch
        assert_eq!(
            Uuid::new_v1(&ctx, 0, 100, node_id).unwrap().to_utc(),
            Some(Utc.ymd(1970, 1, 1).and_hms_nano(0, 0, 0, 100))
        );

        // 299ns from epoch
        assert_eq!(
            Uuid::new_v1(&ctx, 0, 299, node_id).unwrap().to_utc(),
            Some(Utc.ymd(1970, 1, 1).and_hms_nano(0, 0, 0, 200))
        );

        // 7sec, 453ns from epoch
        assert_eq!(
            Uuid::new_v1(&ctx, 7, 453, node_id).unwrap().to_utc(),
            Some(Utc.ymd(1970, 1, 1).and_hms_nano(0, 0, 7, 400))
        );

        // current time
        let utc = Utc::now();
        let v1 = Uuid::new_v1(
            &ctx,
            utc.timestamp() as u64,
            utc.timestamp_subsec_nanos(),
            node_id,
        )
        .unwrap();
        let rt = v1.to_utc().unwrap();
        assert!(
            utc.signed_duration_since(rt)
                .num_nanoseconds()
                .unwrap()
                .abs()
                < 100
        );
    }

    #[test]
    fn test_v1_to_chrono_utc_pre_1970() {
        // have to manually construct as dates prior to 1970 impossible
        // with existing `new_v1` implementation.
        let long_ago_v1 = |uuid_time: i64| -> Uuid {
            let time_low = (uuid_time & 0xFFFF_FFFF) as u32;
            let time_mid = ((uuid_time >> 32) & 0xFFFF) as u16;
            let time_high_and_version =
                (((uuid_time >> 48) & 0x0FFF) as u16) | (1 << 12);
            Uuid::from_fields(
                time_low,
                time_mid,
                time_high_and_version,
                &[0u8; 8],
            )
            .unwrap()
        };

        // 1sec prior to 1970
        assert_eq!(
            long_ago_v1(-10_000_000 + UUID_TICKS_BETWEEN_EPOCHS).to_utc(),
            Some(Utc.ymd(1969, 12, 31).and_hms(23, 59, 59))
        );

        // 1sec after 1582 epoch
        assert_eq!(
            long_ago_v1(10_000_000).to_utc(),
            Some(Utc.ymd(1582, 10, 15).and_hms(0, 0, 1))
        );

        // independence day
        assert_eq!(
            long_ago_v1(61_132_320_010_000_000).to_utc(),
            Some(Utc.ymd(1776, 7, 4).and_hms(0, 0, 1))
        );
    }

    #[test]
    fn test_non_v1_variants_to_chrono_utc() {
        assert!(Uuid::nil().to_utc().is_none());

        #[cfg(feature = "v3")]
        assert!(Uuid::new_v3(&Uuid::nil(), "v3 name".as_bytes())
            .to_utc()
            .is_none());

        #[cfg(feature = "v4")]
        assert!(Uuid::new_v4().to_utc().is_none());

        #[cfg(feature = "v5")]
        assert!(Uuid::new_v5(&Uuid::nil(), "v5 name".as_bytes())
            .to_utc()
            .is_none());
    }
}
