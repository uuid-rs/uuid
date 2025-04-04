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

use crate::{
    convert::TryFrom,
    fmt::{Braced, Hyphenated, Simple, Urn},
    non_nil::NonNilUuid,
    Uuid,
};
use bincode::{
    config::{Config, IntEncoding},
    de::{BorrowDecoder, Decoder},
    enc::Encoder,
    error::{DecodeError, EncodeError},
    BorrowDecode, Decode, Encode,
};
use std::string::ToString;

impl Encode for Uuid {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        if encoder.config().int_encoding() == IntEncoding::Fixed {
            usize::encode(&16, encoder)?;
        } else {
            u8::encode(&16, encoder)?;
        }

        <&[u8; 16]>::encode(&self.as_bytes(), encoder)?;

        Ok(())
    }
}

impl Encode for NonNilUuid {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.get().encode(encoder)
    }
}

impl Encode for Hyphenated {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_uuid().encode(encoder)
    }
}

impl Encode for Simple {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_uuid().encode(encoder)
    }
}

impl Encode for Urn {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_uuid().encode(encoder)
    }
}

impl Encode for Braced {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.as_uuid().encode(encoder)
    }
}

impl<Context> Decode<Context> for Uuid {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        if decoder.config().int_encoding() == IntEncoding::Fixed {
            usize::decode(decoder)?;
        } else {
            u8::decode(decoder)?;
        };

        Ok(Uuid::from_bytes(Decode::decode(decoder)?))
    }
}
impl<'de, Context> BorrowDecode<'de, Context> for Uuid {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        if decoder.config().int_encoding() == IntEncoding::Fixed {
            usize::borrow_decode(decoder)?;
        } else {
            u8::borrow_decode(decoder)?;
        };

        Ok(Uuid::from_bytes(BorrowDecode::borrow_decode(decoder)?))
    }
}

impl<Context> Decode<Context> for NonNilUuid {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let uuid = Uuid::decode(decoder)?;
        NonNilUuid::try_from(uuid).map_err(|e| DecodeError::OtherString(e.to_string()))
    }
}
impl<'de, Context> BorrowDecode<'de, Context> for NonNilUuid {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let uuid = Uuid::borrow_decode(decoder)?;
        NonNilUuid::try_from(uuid).map_err(|e| DecodeError::OtherString(e.to_string()))
    }
}

#[cfg(test)]
mod bincode_tests {
    use super::*;
    use bincode::config;

    #[test]
    fn test_legacy() {
        let uuid = Uuid::parse_str("f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4").unwrap();

        let legacy_bytes = bincodev1::serialize(&uuid).expect(&format!(
            "Should have been able to encode {} with bincode v1.",
            uuid.to_string()
        ));
        let (decoded, _) = bincode::decode_from_slice::<Uuid, _>(&legacy_bytes, config::legacy())
            .expect("Bincode v2 should have been able to decode bytes encoded by bincode v1.");
        assert_eq!(uuid, decoded);

        let new_bytes = bincode::encode_to_vec(&decoded, config::legacy())
            .expect("Bincode v2 should have been able to encode uuid using legacy config.");
        let decoded = bincodev1::deserialize::<Uuid>(&new_bytes)
            .expect("Bincode v1 should have been able to decode bytes encoded by bincode v1.");
        assert_eq!(uuid, decoded);
    }

    #[test]
    fn test_encode_readable_string() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let uuid = Uuid::parse_str(uuid_str).unwrap();

        let bytes = bincode::encode_to_vec(&uuid, config::standard())
            .expect(&format!("Failed to encode {uuid_str}."));
        let (decoded_uuid, _) = bincode::decode_from_slice::<Uuid, _>(&bytes, config::standard())
            .expect(&format!("Failed to decode {bytes:?}."));

        assert_eq!(uuid, decoded_uuid);
    }

    #[test]
    fn test_encode_hyphenated() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let uuid = Uuid::parse_str(uuid_str).unwrap();

        let bytes = bincode::encode_to_vec(&uuid, config::standard())
            .expect(&format!("Failed to encode {uuid_str}."));
        let (decoded_uuid, _) = bincode::decode_from_slice::<Uuid, _>(&bytes, config::standard())
            .expect(&format!("Failed to decode {bytes:?}."));

        assert_eq!(uuid, decoded_uuid);
    }

    #[test]
    fn test_encode_simple() {
        let uuid_str = "f9168c5eceb24faab6bf329bf39fa1e4";
        let uuid = Uuid::parse_str(uuid_str).unwrap();

        let bytes = bincode::encode_to_vec(&uuid, config::standard())
            .expect(&format!("Failed to encode {uuid_str}."));
        let (decoded_uuid, _) = bincode::decode_from_slice::<Uuid, _>(&bytes, config::standard())
            .expect(&format!("Failed to decode {bytes:?}."));

        assert_eq!(uuid, decoded_uuid);
    }

    #[test]
    fn test_encode_urn() {
        let uuid_str = "urn:uuid:f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let uuid = Uuid::parse_str(uuid_str).unwrap();

        let bytes = bincode::encode_to_vec(&uuid, config::standard())
            .expect(&format!("Failed to encode {uuid_str}."));
        let (decoded_uuid, _) = bincode::decode_from_slice::<Uuid, _>(&bytes, config::standard())
            .expect(&format!("Failed to decode {bytes:?}."));

        assert_eq!(uuid, decoded_uuid);
    }

    #[test]
    fn test_encode_braced() {
        let uuid_str = "{f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4}";
        let uuid = Uuid::parse_str(uuid_str).unwrap();

        let bytes = bincode::encode_to_vec(&uuid, config::standard())
            .expect(&format!("Failed to encode {uuid_str}."));
        let (decoded_uuid, _) = bincode::decode_from_slice::<Uuid, _>(&bytes, config::standard())
            .expect(&format!("Failed to decode {bytes:?}."));

        assert_eq!(uuid, decoded_uuid);
    }

    #[test]
    fn test_encode_non_human_readable() {
        let uuid_bytes = b"F9168C5E-CEB2-4F";
        let uuid = Uuid::from_slice(uuid_bytes).unwrap();

        let bytes = bincode::encode_to_vec(&uuid, config::standard())
            .expect(&format!("{:?} failed to encode.", uuid_bytes));
        let (decoded_uuid, _) = bincode::decode_from_slice::<Uuid, _>(&bytes, config::standard())
            .expect(&format!("Failed to decode {bytes:?}."));

        assert_eq!(uuid, decoded_uuid);
    }

    #[test]
    fn test_decode_failure() {
        let bytes = "hello_world".as_bytes();
        let error = bincode::decode_from_slice::<Uuid, _>(bytes, config::standard())
            .expect_err(&format!("Should not have been able to decode {bytes:?}."));

        match error {
            DecodeError::UnexpectedEnd { additional: 5 } => {}
            _ => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_decode_non_nil_uuid() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let uuid = Uuid::parse_str(uuid_str).unwrap();

        let bytes = bincode::encode_to_vec(&uuid, config::standard())
            .expect(&format!("Failed to encode {uuid_str}."));

        let (decoded_uuid, decoded_size) =
            bincode::decode_from_slice::<NonNilUuid, _>(&bytes, config::standard()).expect(
                &format!("Should have been able to decode {bytes:?} to NonNilUuid."),
            );

        assert_eq!(uuid, decoded_uuid);
        assert_eq!(16, decoded_size);
    }

    #[test]
    fn test_decode_nil_uuid() {
        let uuid = Uuid::nil();

        let bytes = bincode::encode_to_vec(&uuid, config::standard())
            .expect(&format!("Failed to encode {}.", uuid.to_string()));

        let error = bincode::decode_from_slice::<NonNilUuid, _>(&bytes, config::standard())
            .expect_err(&format!(
                "Should not have been able to decode {bytes:?} to NonNilUuid."
            ));

        match error {
            DecodeError::OtherString(s) if s == "the UUID is nil" => {}
            _ => panic!("Unexpected error"),
        }
    }
}
