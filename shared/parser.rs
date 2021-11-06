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

use crate::{error::*, std::str};

/// Check if the length matches any of the given criteria lengths.
fn len_matches_any(len: usize, crits: &[usize]) -> bool {
    for crit in crits {
        if len == *crit {
            return true;
        }
    }

    false
}

// Accumulated length of each hyphenated group in hex digits.
const ACC_GROUP_LENS: [usize; 5] = [8, 12, 16, 20, 32];

// Length of each hyphenated group in hex digits.
pub const GROUP_LENS: [usize; 5] = [8, 4, 4, 4, 12];

const URN_PREFIX: &str = "urn:uuid:";

pub fn parse_str(mut input: &str) -> Result<[u8; 16], Error> {
    // Ensure length is valid for any of the supported formats
    let len = input.len();

    let mut start = 0;
    // Check for a URN prefixed UUID
    if len == 45 {
        if let Some(stripped) = input.strip_prefix(URN_PREFIX) {
            input = stripped;
            start = URN_PREFIX.len();
        }
    }
    // Check for a Microsoft GUID wrapped in {}
    else if len == 38 {
        if let Some(stripped) =
            input.strip_prefix('{').and_then(|s| s.strip_suffix('}'))
        {
            input = stripped;
            start = 1;
        }
    }
    // In other cases, check for a simple or hyphenated UUID
    else if !len_matches_any(len, &[36, 32]) {
        return Err(Error::length(ExpectedLength::Any(&[36, 32]), len));
    }

    // `digit` counts only hexadecimal digits, `i_char` counts all chars.
    let mut digit = 0;
    let mut group = 0;
    let mut acc = 0;
    let mut buffer = [0u8; 16];

    for (i_char, character) in input.char_indices() {
        let chr = character as u8;
        if digit as usize >= 32 && group != 4 {
            if group == 0 {
                return Err(Error::length(ExpectedLength::Any(&[36, 32]), len));
            }

            return Err(Error::group_count(
                ExpectedLength::Any(&[1, 5]),
                group + 1,
            ));
        }

        if digit % 2 == 0 {
            // First digit of the byte.
            match chr {
                // Calculate upper half.
                b'0'..=b'9' => acc = chr - b'0',
                b'a'..=b'f' => acc = chr - b'a' + 10,
                b'A'..=b'F' => acc = chr - b'A' + 10,
                // Found a group delimiter
                b'-' => {
                    if ACC_GROUP_LENS[group] as u8 != digit {
                        // Calculate how many digits this group consists of
                        // in the input.
                        let found = if group > 0 {
                            digit as usize - ACC_GROUP_LENS[group - 1]
                        } else {
                            digit as usize
                        };

                        return Err(Error::group_length(
                            ExpectedLength::Exact(GROUP_LENS[group]),
                            found,
                            group,
                            start,
                        ));
                    }
                    // Next group, decrement digit, it is incremented again
                    // at the bottom.
                    group += 1;
                    digit -= 1;
                }
                _ => {
                    return Err(Error::character(character, i_char, start));
                }
            }
        } else {
            // Second digit of the byte, shift the upper half.
            acc *= 16;
            match chr {
                b'0'..=b'9' => acc += chr - b'0',
                b'a'..=b'f' => acc += chr - b'a' + 10,
                b'A'..=b'F' => acc += chr - b'A' + 10,
                b'-' => {
                    // The byte isn't complete yet.
                    let found = if group > 0 {
                        digit - ACC_GROUP_LENS[group - 1] as u8
                    } else {
                        digit
                    };

                    return Err(Error::group_length(
                        ExpectedLength::Exact(GROUP_LENS[group]),
                        found as usize,
                        group,
                        start,
                    ));
                }
                _ => {
                    // let found = input[i_char..].chars().next().unwrap();
                    // let found = char::from(chr);
                    return Err(Error::character(character, i_char, start));
                }
            }
            buffer[(digit / 2) as usize] = acc;
        }
        digit += 1;
    }

    // Now check the last group.
    if ACC_GROUP_LENS[4] as u8 != digit {
        return Err(Error::group_length(
            ExpectedLength::Exact(GROUP_LENS[4]),
            digit as usize - ACC_GROUP_LENS[3],
            group,
            start,
        ));
    }

    Ok(buffer)
}
