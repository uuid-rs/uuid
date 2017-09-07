extern crate serde;
extern crate std;

use self::std::fmt;
use self::std::prelude::v1::*;
use self::serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use Uuid;

impl Serialize for Uuid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.collect_str(&self.hyphenated())
        } else {
            serializer.serialize_bytes(self.as_bytes())
        }
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
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
                    Uuid::from_bytes(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_str(UuidStringVisitor)
        } else {
            struct UuidBytesVisitor;

            impl<'vi> de::Visitor<'vi> for UuidBytesVisitor {
                type Value = Uuid;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "bytes")
                }

                fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Uuid, E> {
                    Uuid::from_bytes(value).map_err(E::custom)
                }
            }

            deserializer.deserialize_bytes(UuidBytesVisitor)
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate serde_json;
    extern crate serde_test;

    use self::serde_test::Token;

    use Uuid;

    #[test]
    fn test_str() {
        let str_uuid = "67e55044-10b1-426f-9247-bb680e5fe0c8";
        let uuid = Uuid::parse_str(str_uuid).unwrap();

        serde_test::assert_tokens(&uuid, &[Token::BorrowedStr(str_uuid)]);
    }

    #[test]
    fn test_serialize_round_trip() {
        let u = Uuid::parse_str("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4").unwrap();
        let s = serde_json::to_string(&u).unwrap();
        let u2 = serde_json::from_str(&s).unwrap();
        assert_eq!(u, u2);
    }
}
