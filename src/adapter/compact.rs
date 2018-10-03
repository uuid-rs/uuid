//! Module for use with `#[serde(with = "...")]` to serialize a [`Uuid`]
//! as a `[u8; 16]
//!
//! [`Uuid`]: ../../struct.Uuid.html

use serde::de::{self, Error, SeqAccess};
use serde::ser::SerializeTuple;
use serde::{Deserializer, Serializer};

use prelude::*;

use std::fmt;

/// Serializer for a [`Uuid`] into a `[u8; 16]`
///
/// [`Uuid`]: ../../struct.Uuid.html
pub fn serialize<S: Serializer>(
    u: &Uuid,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_tuple(16)?;

    for byte in u.as_bytes() {
        seq.serialize_element(byte)?;
    }

    seq.end()
}

/// Deserializer from a `[u8; 16]` into a [`Uuid`]
///
/// [`Uuid`]: ../../struct.Uuid.html
pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Uuid, D::Error> {
    struct DenseUuidBytesVisitor;

    impl<'vi> de::Visitor<'vi> for DenseUuidBytesVisitor {
        type Value = Uuid;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
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

#[cfg(test)]
mod tests {
    use serde_test;

    use prelude::*;

    #[derive(Serialize, Debug, Deserialize, PartialEq)]
    struct UuidContainer {
        #[serde(with = "super")]
        u: Uuid,
    }

    #[test]
    fn test_serialize_compact() {
        use serde_test::Configure;

        let uuid_bytes = b"F9168C5E-CEB2-4F";
        let container = UuidContainer {
            u: Uuid::from_slice(uuid_bytes).unwrap(),
        };

        // more complex because of the struct wrapping the actual UUID
        // serialization
        serde_test::assert_tokens(
            &container.compact(),
            &[
                serde_test::Token::Struct {
                    name: "UuidContainer",
                    len: 1,
                },
                serde_test::Token::Str("u"),
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
                serde_test::Token::StructEnd,
            ],
        )
    }
}
