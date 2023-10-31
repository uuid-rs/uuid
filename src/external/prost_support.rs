#[cfg(all(feature = "prost-string", feature = "prost-bytes"))]
compile_error!("prost-bytes and prost-string features are mutually exclusive");

use crate::Uuid;

use prost::{
    bytes::{Buf, BufMut},
    encoding::*,
    DecodeError, Message,
};

impl Message for Uuid {
    #[cfg(feature = "prost-string")]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        use crate::fmt::Hyphenated;

        let mut uuid_buf = [0u8; Hyphenated::LENGTH];
        self.as_hyphenated().encode_lower(&mut uuid_buf);

        encode_key(1, WireType::LengthDelimited, buf);
        buf.put_u8(Hyphenated::LENGTH as u8);
        buf.put_slice(&uuid_buf[..]);
    }

    #[cfg(feature = "prost-bytes")]
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        encode_key(1, WireType::LengthDelimited, buf);
        buf.put_u8(16);
        buf.put_slice(self.as_bytes())
    }

    #[cfg(feature = "prost-string")]
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
        if tag != 1 {
            return skip_field(wire_type, tag, buf, ctx);
        }

        check_wire_type(WireType::LengthDelimited, wire_type)?;

        if !buf.has_remaining() {
            return Err(DecodeError::new("buffer underflow"));
        }

        let len = match buf.get_u8() {
            len @ (32 | 36 | 38 | 45) => len,
            _ => return Err(DecodeError::new("invalid uuid length")),
        };

        if buf.remaining() < len as usize {
            return Err(DecodeError::new("buffer underflow"));
        }

        let mut uuid_buf = [0u8; 45];
        let mut uuid_buf = &mut uuid_buf[..len as usize];

        buf.copy_to_slice(&mut uuid_buf);

        let uuid_str =
            core::str::from_utf8(uuid_buf).map_err(|_| DecodeError::new("invaild utf8 string"))?;

        *self = match Uuid::parse_str(uuid_str) {
            Ok(uuid) => uuid,
            Err(_) => return Err(DecodeError::new("invalid uuid string")),
        };

        Ok(())
    }

    #[cfg(feature = "prost-bytes")]
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
            check_wire_type(WireType::LengthDelimited, wire_type)?;

            if buf.remaining() < 17 {
                return Err(DecodeError::new("buffer underflow"));
            }

            if buf.get_u8() != 16 {
                return Err(DecodeError::new("invalid uuid length"));
            }

            buf.copy_to_slice(&mut self.0);

            Ok(())
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }

    #[inline]
    fn encoded_len(&self) -> usize {
        #[cfg(feature = "prost-string")]
        const LEN: usize = crate::fmt::Hyphenated::LENGTH;
        #[cfg(feature = "prost-bytes")]
        const LEN: usize = 16;

        key_len(1) + encoded_len_varint(LEN as u64) + LEN
    }

    #[inline]
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
    #[cfg(feature = "prost-string")]
    fn test_serialize_deserialize_string() {
        use std::string::ToString;

        const UUID_STR: &str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";

        let epxected_uuid = Uuid::parse_str(UUID_STR).unwrap();
        let encoded_uuid = epxected_uuid.encode_to_vec();
        let decoded_uuid = Uuid::decode(&encoded_uuid[..]).unwrap();
        let actual_uuid = decoded_uuid.to_string();

        assert_eq!(UUID_STR, actual_uuid);
    }

    #[test]
    #[cfg(feature = "prost-bytes")]
    fn test_serialize_deserialize_bytes() {
        const UUID_BUF: [u8; 16] = [
            0x0f, 0x10, 0x8c, 0x6e,
            0xb2, 0xce, 0xaa, 0x4f,
            0xb6, 0xbf, 0x32, 0x9b,
            0xf3, 0x9f, 0xa1, 0xe4,
        ];

        let expected_uuid = Uuid::from_bytes(UUID_BUF);
        let encoded_uuid = expected_uuid.encode_to_vec();
        let decoded_uuid = Uuid::decode(&encoded_uuid[..]).unwrap();
        let actual_uuid = decoded_uuid.as_bytes();

        assert_eq!(&UUID_BUF, actual_uuid);
    }
}
