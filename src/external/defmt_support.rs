use crate::{fmt::Hyphenated, Uuid};

impl defmt::Format for Uuid {
    fn format(&self, f: defmt::Formatter<'_>) {
        let mut buf = [0u8; Hyphenated::LENGTH];
        let s = self.as_hyphenated().encode_lower(&mut buf);
        defmt::write!(f, "{=str}", s);
    }
}
