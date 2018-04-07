use prelude::*;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::fmt;
        use std::str;
    } else {
        use core::fmt;
        use core::str;
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(self, f)
    }
}

impl fmt::Display for UuidVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UuidVariant::NCS => write!(f, "NCS"),
            UuidVariant::RFC4122 => write!(f, "RFC4122"),
            UuidVariant::Microsoft => write!(f, "Microsoft"),
            UuidVariant::Future => write!(f, "Future"),
        }
    }
}

impl fmt::LowerHex for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <super::Hyphenated as fmt::LowerHex>::fmt(&self.hyphenated(), f)
    }
}


impl fmt::UpperHex for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <super::Hyphenated as fmt::LowerHex>::fmt(&self.hyphenated(), f)
    }
}

impl str::FromStr for Uuid {
    type Err = super::ParseError;

    fn from_str(uuid_str: &str) -> Result<Uuid, super::ParseError> {
        Uuid::parse_str(uuid_str)
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Uuid::nil()
    }
}
