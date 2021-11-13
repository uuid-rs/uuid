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

use crate::error::InvalidUuid;

pub const fn try_parse(input: &str) -> Result<[u8; 16], InvalidUuid> {
    const fn parse_blocks<'a>(
        s: &'a [u8],
        hyphenated: bool,
    ) -> Option<[u8; 16]> {
        let block_table = if hyphenated {
            match (s[8], s[13], s[18], s[23]) {
                (b'-', b'-', b'-', b'-') => [0, 4, 9, 14, 19, 24, 28, 32],
                _ => return None,
            }
        } else {
            [0, 4, 8, 12, 16, 20, 24, 28]
        };

        let mut buf = [0; 16];
        let mut j = 0;
        while j < 8 {
            let i = block_table[j];
            // Check 4 bytes at a time
            let h1 = HEX_TABLE[s[i] as usize];
            let h2 = HEX_TABLE[s[i + 1] as usize];
            let h3 = HEX_TABLE[s[i + 2] as usize];
            let h4 = HEX_TABLE[s[i + 3] as usize];
            // If any of the bytes aren't valid, they will be 0xff, making this
            // fail
            if h1 | h2 | h3 | h4 == 0xff {
                return None;
            }
            buf[j * 2] = SHL4_TABLE[h1 as usize] | h2;
            buf[j * 2 + 1] = SHL4_TABLE[h3 as usize] | h4;
            j += 1;
        }
        Some(buf)
    }

    let b = input.as_bytes();
    let maybe_parsed = match (b.len(), b) {
        (32, s) => parse_blocks(s, false),
        (36, s)
        | (38, [b'{', s @ .., b'}'])
        | (
            45,
            [b'u', b'r', b'n', b':', b'u', b'u', b'i', b'd', b':', s @ ..],
        ) => parse_blocks(s, true),
        _ => None,
    };
    match maybe_parsed {
        Some(b) => Ok(b),
        None => Err(InvalidUuid(input)),
    }
}

type Table = [u8; 256];

const fn generate_lookup_table() -> Table {
    let mut buf = [0u8; 256];
    let mut i = 0u8;
    loop {
        buf[i as usize] = match i {
            b'0'..=b'9' => i - b'0',
            b'a'..=b'f' => i - b'a' + 10,
            b'A'..=b'F' => i - b'A' + 10,
            _ => 0xff,
        };
        if i == 255 {
            return buf;
        }
        i += 1
    }
}

const HEX_TABLE: Table = generate_lookup_table();

const fn generate_shl4_table() -> Table {
    let mut buf = [0u8; 256];
    let mut i = 0u8;
    loop {
        buf[i as usize] = i.wrapping_shl(4);
        if i == 255 {
            return buf;
        }
        i += 1;
    }
}

const SHL4_TABLE: Table = generate_shl4_table();
