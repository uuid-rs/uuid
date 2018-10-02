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

use core::fmt;
use prelude::*;
use serde::de::{self, SeqAccess, Error};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(all(feature = "serde", not(feature = "dense_serde")))]
impl Serialize for Uuid {
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer
                .serialize_str(&self.to_hyphenated().encode_lower(&mut [0; 36]))
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

#[cfg(feature = "dense_serde")]
impl Serialize for Uuid {
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer
                .serialize_str(&self.to_hyphenated().encode_lower(&mut [0; 36]))
        } else {
            let mut seq = serializer.serialize_tuple(16)?;

            for byte in self.as_bytes() {
                seq.serialize_element(byte)?;
            }

            seq.end()
        }
    }
}

struct UuidStringVisitor;

impl<'vi> de::Visitor<'vi> for UuidStringVisitor {
    type Value = Uuid;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a UUID string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Uuid, E> {
        value.parse::<Uuid>().map_err(E::custom)
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Uuid, E> {
        Uuid::from_slice(value).map_err(E::custom)
    }
}

#[cfg(feature = "dense_serde")]
impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(UuidStringVisitor)
        } else {
            struct DenseUuidBytesVisitor;

            impl<'vi> de::Visitor<'vi> for DenseUuidBytesVisitor {
                type Value = Uuid;

                fn expecting(
                    &self,
                    formatter: &mut fmt::Formatter,
                ) -> fmt::Result {
                    write!(formatter, "tuple")
                }

                fn visit_seq<A: SeqAccess<'vi>>(
                    self,
                    mut seq: A,
                ) -> Result<Self::Value, A::Error> {
                    if seq.size_hint() == Some(16) {
                        let mut buf = [0; 16];

                        for i in 0..16 {
                            buf[i] = seq.next_element().unwrap().unwrap()
                        }

                        Ok(Uuid::from_bytes(buf))
                    } else {
                        Err(Error::custom("Uuid must be 16 bytes long"))
                    }
                }
            }

            deserializer.deserialize_tuple(16, DenseUuidBytesVisitor)
        }
    }
}

#[cfg(all(feature = "serde", not(feature = "dense_serde")))]
impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(UuidStringVisitor)
        } else {
            struct UuidBytesVisitor;

            impl<'vi> de::Visitor<'vi> for UuidBytesVisitor {
                type Value = Uuid;

                fn expecting(
                    &self,
                    formatter: &mut fmt::Formatter,
                ) -> fmt::Result {
                    write!(formatter, "bytes")
                }

                fn visit_bytes<E: de::Error>(
                    self,
                    value: &[u8],
                ) -> Result<Uuid, E> {
                    Uuid::from_slice(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_bytes(UuidBytesVisitor)
        }
    }
}

#[cfg(all(test, feature = "serde", not(feature = "dense_serde")))]
mod serde_tests {
    use serde_test;

    use prelude::*;

    #[test]
    fn test_serialize_readable() {
        use serde_test::Configure;

        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let u = Uuid::parse_str(uuid_str).unwrap();
        serde_test::assert_tokens(
            &u.readable(),
            &[serde_test::Token::Str(uuid_str)],
        );
    }

    #[test]
    fn test_serialize_compact() {
        use serde_test::Configure;

        let uuid_bytes = b"F9168C5E-CEB2-4F";
        let u = Uuid::from_slice(uuid_bytes).unwrap();
        serde_test::assert_tokens(
            &u.compact(),
            &[serde_test::Token::Bytes(uuid_bytes)],
        );
    }
}

#[cfg(all(test, feature = "dense_serde"))]
mod dense_serde_tests {
    use serde_test;

    use prelude::*;

    #[test]
    fn test_serialize_readable() {
        use serde_test::Configure;

        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let u = Uuid::parse_str(uuid_str).unwrap();
        serde_test::assert_tokens(
            &u.readable(),
            &[serde_test::Token::Str(uuid_str)],
        );
    }

    #[test]
    fn test_serialize_compact() {
        use serde_test::Configure;

        let uuid_bytes = b"F9168C5E-CEB2-4F";
        let u = Uuid::from_slice(uuid_bytes).unwrap();
        serde_test::assert_tokens(
            &u.compact(),
            &[
                serde_test::Token::Tuple { len: 16 },
                serde_test::Token::U8(uuid_bytes[0]),
                serde_test::Token::U8(uuid_bytes[1]),
                serde_test::Token::U8(uuid_bytes[2]),
                serde_test::Token::U8(uuid_bytes[3]),
                serde_test::Token::U8(uuid_bytes[4]),
                serde_test::Token::U8(uuid_bytes[5]),
                serde_test::Token::U8(uuid_bytes[6]),
                serde_test::Token::U8(uuid_bytes[7]),
                serde_test::Token::U8(uuid_bytes[8]),
                serde_test::Token::U8(uuid_bytes[9]),
                serde_test::Token::U8(uuid_bytes[10]),
                serde_test::Token::U8(uuid_bytes[11]),
                serde_test::Token::U8(uuid_bytes[12]),
                serde_test::Token::U8(uuid_bytes[13]),
                serde_test::Token::U8(uuid_bytes[14]),
                serde_test::Token::U8(uuid_bytes[15]),
                serde_test::Token::TupleEnd,
            ],
        )
    }
}
