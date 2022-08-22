// Copyright 2022 The Uuid Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{Uuid, fmt::Hyphenated};

impl defmt::Format for Uuid {
    fn format(&self, f: defmt::Formatter<'_>) {
        let mut buf = [0u8; Hyphenated::LENGTH];
        let s = self.as_hyphenated().encode_lower(&mut buf);
        defmt::write!(f, "{=str}", s);
    }
}
