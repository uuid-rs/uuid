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

macro_rules! hyphenated_write {
    ($f:ident, $format:expr, $bytes:expr) => {{
        let data1 = u32::from($bytes[0]) << 24 | u32::from($bytes[1]) << 16
            | u32::from($bytes[2]) << 8
            | u32::from($bytes[3]);

        let data2 = u16::from($bytes[4]) << 8 | u16::from($bytes[5]);

        let data3 = u16::from($bytes[6]) << 8 | u16::from($bytes[7]);

        write!(
            $f,
            $format,
            data1,
            data2,
            data3,
            $bytes[8],
            $bytes[9],
            $bytes[10],
            $bytes[11],
            $bytes[12],
            $bytes[13],
            $bytes[14],
            $bytes[15]
        )
    }};
}
