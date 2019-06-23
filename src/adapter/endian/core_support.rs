use byteorder;
use crate::prelude::*;
use crate::adapter::endian;

impl From<u128> for endian::Big {
    fn from(f: u128) -> Self {
        let mut bytes: Bytes = [0; 16];

        <byteorder::BE as byteorder::ByteOrder>::write_u128(&mut bytes, f);

        endian::Big(bytes)
    }
}

impl From<u128> for endian::Little {
    fn from(f: u128) -> Self {
        let mut bytes: Bytes = [0; 16];

        <byteorder::LE as byteorder::ByteOrder>::write_u128(&mut bytes, f);

        endian::Little(bytes)
    }
}

impl From<Uuid> for endian::Big {
    fn from(f: Uuid) -> Self {
        let bytes = f.0;

        endian::Big(bytes)
    }
}

impl From<Uuid> for endian::Little {
    fn from(f: Uuid) -> Self {
        let mut bytes = f.0;

        bytes.reverse();

        endian::Little(bytes)
    }
}