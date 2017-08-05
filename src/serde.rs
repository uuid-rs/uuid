extern crate serde;
extern crate std;

use self::std::fmt;
use self::std::prelude::v1::*;
use self::serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use Uuid;

impl Serialize for Uuid {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.hyphenated())
    }
}

impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct UuidVisitor;

        impl<'vi> de::Visitor<'vi> for UuidVisitor {
            type Value = Uuid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a UUID string")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Uuid, E> {
                value.parse::<Uuid>().map_err(|e| E::custom(e.to_string()))
            }
        }

        deserializer.deserialize_str(UuidVisitor)
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;
    use Uuid;

    #[test]
    fn test_serialize_round_trip() {
        let u = Uuid::parse_str("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4").unwrap();
        let s = serde_json::to_string(&u).unwrap();
        let u2 = serde_json::from_str(&s).unwrap();
        assert_eq!(u, u2);
    }
}
