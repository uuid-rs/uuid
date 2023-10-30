use crate::Uuid;

use prost::{
    bytes::{Buf, BufMut},
    encoding::{string, *},
    DecodeError, Message,
};
use std::string::String;
use std::string::ToString;

impl Message for Uuid {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        string::encode(1, &self.to_string(), buf)
    }

    fn merge_field<B>(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut B,
        ctx: DecodeContext,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            let mut uuid_str = String::new();

            string::merge(wire_type, &mut uuid_str, buf, ctx)?;

            *self = match Uuid::parse_str(&uuid_str) {
                Ok(uuid) => uuid,
                Err(err) => return Err(DecodeError::new(err.to_string())),
            };

            Ok(())
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }

    fn encoded_len(&self) -> usize {
        string::encoded_len(1, &self.to_string())
    }

    fn clear(&mut self) {
        for b in &mut self.0 {
            *b = 0
        }
    }
}

#[cfg(test)]
mod prost_tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        const UUID_STR: &str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";

        let epxected_uuid = Uuid::parse_str(UUID_STR).unwrap();
        let encoded_uuid = epxected_uuid.encode_to_vec();
        let decoded_uuid = Uuid::decode(&encoded_uuid[..]).unwrap();
        let actual_uuid = decoded_uuid.to_string();

        assert_eq!(UUID_STR, actual_uuid);
    }
}
