use fmt;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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
    extern crate serde_test;

    use self::serde_test::{assert_tokens, Configure, Token};

    use Uuid;

    #[test]
    fn test_serialize_readable() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let u = Uuid::parse_str(uuid_str).unwrap();
        assert_tokens(&u.readable(), &[Token::Str(uuid_str)]);
    }

    #[test]
    fn test_serialize_compact() {
        let uuid_bytes = b"F9168C5E-CEB2-4F";
        let u = Uuid::from_bytes(uuid_bytes).unwrap();
        assert_tokens(&u.compact(), &[Token::Bytes(uuid_bytes)]);
    }
}
