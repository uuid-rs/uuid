use prelude::*;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::str;
    } else {
        use core::str;
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
