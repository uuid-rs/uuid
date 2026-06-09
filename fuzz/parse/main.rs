fn main() {
    uuid_fuzz::main(de);
}

pub fn de(input: &[u8]) {
    use std::str::FromStr;

    let Ok(input) = std::str::from_utf8(input) else {
        return;
    };

    let cases = [
        ("all", uuid::Uuid::from_str(input).ok()),
        ("hyphenated", uuid::fmt::Hyphenated::from_str(input).ok().map(|uuid| uuid.into_uuid())),
        ("simple", uuid::fmt::Simple::from_str(input).ok().map(|uuid| uuid.into_uuid())),
        ("urn", uuid::fmt::Urn::from_str(input).ok().map(|uuid| uuid.into_uuid())),
        ("braced", uuid::fmt::Braced::from_str(input).ok().map(|uuid| uuid.into_uuid())),
    ];

    for (ac, a) in cases {
        for (bc, b) in cases {
            if let (Some(a), Some(b)) = (a, b) {
                assert_eq!(a, b, "{ac} == {bc}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial() {
        uuid_fuzz::initial_cases("parse", de);
    }

    #[test]
    fn repro() {
        uuid_fuzz::repro_cases("parse", de);
    }
}
