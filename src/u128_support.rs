use byteorder;
use prelude::*;

impl From<u128> for Uuid {
    fn from(f: u128) -> Self {
        let mut uuid = Uuid::default();

        {
            use byteorder::ByteOrder;

            byteorder::NativeEndian::write_u128(&mut uuid.bytes.as_mut(), f);
        }

        uuid
    }
}


#[cfg(test)]
mod tests {
    use prelude::*;

    #[test]
    fn test_from_u128() {
        const U128: u128 = 0x3a0724b4_93a0_4d87_ac28_759c6caa13c4;

        let uuid = Uuid::from(U128);

        let uuid2: Uuid = U128.into();

        assert_eq!(uuid, uuid2)
    }

}