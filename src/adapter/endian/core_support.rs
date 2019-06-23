use byteorder;
use crate::prelude::*;
use crate::adapter::endian;
use rand::AsByteSliceMut;

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