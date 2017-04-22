extern crate rocket;

use Uuid;
use ParseError;
use self::rocket::request::FromParam;

impl<'a> FromParam<'a> for Uuid {
    type Error = ParseError;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Uuid::parse_str(param)
    }
}

#[cfg(test)]
mod tests {
    use super::rocket::request::FromParam;
    use Uuid;

    #[test]
    fn test_serialize_round_trip() {
        assert_eq!(
            Uuid::from_param("C9344778-FC32-4151-82B1-DA78601584FB").unwrap(),
            Uuid::parse_str("C9344778-FC32-4151-82B1-DA78601584FB").unwrap()
        )
    }
}
