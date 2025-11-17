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

use miniserde::{de, make_place, ser, Deserialize, Error, Result, Serialize};

use crate::Uuid;

impl Deserialize for Uuid {
    fn begin(out: &mut Option<Self>) -> &mut dyn de::Visitor {
        make_place!(Place);

        impl de::Visitor for Place<Uuid> {
            fn string(&mut self, s: &str) -> Result<()> {
                let uuid = Uuid::parse_str(s).map_err(|_| Error)?;
                self.out = Some(uuid);
                Ok(())
            }
        }

        Place::new(out)
    }
}

#[cfg(feature = "std")]
impl Serialize for Uuid {
    fn begin(&self) -> ser::Fragment<'_> {
        use std::borrow::Cow;
        use std::string::ToString;

        ser::Fragment::Str(Cow::Owned(self.to_string()))
    }
}

#[cfg(test)]
mod miniserde_tests {
    use std::string::ToString;

    use super::*;

    use miniserde::json::{from_str, to_string};

    #[derive(Deserialize, Serialize)]
    struct Demo {
        id: Uuid,
    }

    const UUID_SIMPLE_STR: &str = "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8";
    const UUID_HYPHENATED_STR: &str = "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8";
    const UUID_URN_STR: &str = "urn:uuid:a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8";
    const UUID_BRACED_STR: &str = "{a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8}";

    #[test]
    fn test_deserialize_simple() {
        let demo_str = format!(r#"{{"id":"{}"}}"#, UUID_SIMPLE_STR);
        let demo: Demo = from_str(&demo_str).unwrap();
        let uuid_str = demo.id.simple().to_string();
        assert_eq!(UUID_SIMPLE_STR, uuid_str);
    }

    #[test]
    fn test_deserialize_hyphenated() {
        let demo_str = format!(r#"{{"id":"{}"}}"#, UUID_HYPHENATED_STR);
        let demo: Demo = from_str(&demo_str).unwrap();
        let uuid_str = demo.id.hyphenated().to_string();
        assert_eq!(UUID_HYPHENATED_STR, uuid_str);
    }

    #[test]
    fn test_deserialize_urn() {
        let demo_str = format!(r#"{{"id":"{}"}}"#, UUID_URN_STR);
        let demo: Demo = from_str(&demo_str).unwrap();
        let uuid_str = demo.id.urn().to_string();
        assert_eq!(UUID_URN_STR, uuid_str);
    }

    #[test]
    fn test_deserialize_braced() {
        let demo_str = format!(r#"{{"id":"{}"}}"#, UUID_BRACED_STR);
        let demo: Demo = from_str(&demo_str).unwrap();
        let uuid_str = demo.id.braced().to_string();
        assert_eq!(UUID_BRACED_STR, uuid_str);
    }

    #[test]
    fn test_deserialize_failure() {
        let demo_str = format!(r#"{{"id":"{}"}}"#, "invalid-uuid");
        let result: Result<Demo> = from_str(&demo_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_simple() {
        let uuid = Uuid::nil();
        let demo = Demo { id: uuid };
        let demo_str = to_string(&demo);

        assert_eq!(demo_str, format!(r#"{{"id":"{}"}}"#, uuid.to_string()));
    }
}
