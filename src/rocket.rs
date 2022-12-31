extern crate alloc;

use core::str::FromStr;

use alloc::string::String;
use rocket::{request::{FromParam, FromRequest, Outcome}, http::Status};
use std::boxed::Box;

use crate::Uuid;

impl<'a> FromParam<'a> for Uuid {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match Uuid::from_str(param) {
            Ok(uuid) => Ok(uuid),
            Err(_err) => Err("Unable to parse uuid from str."),
        }
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for Uuid {
    type Error = UuidFromRquestError;

    async fn from_request(request: &'a rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let uuid_str = match request.headers().get_one("uuid") {
            Some(header) => header,
            None => return Outcome::Failure((Status::Forbidden, UuidFromRquestError::Missing)),
        };
        let uuid = match Uuid::from_str(uuid_str) {
            Ok(uuid) => uuid,
            Err(_err) => return Outcome::Failure((Status::Forbidden, UuidFromRquestError::Invalid(String::from("Unable to parse uuid from header.")))),
        };
        Outcome::Success(uuid)
    }
}

#[derive(Debug)]
pub enum UuidFromRquestError {
    Missing,
    Invalid(String),
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use core::str::FromStr;

    use alloc::string::{String, ToString};
    use rocket::http::uri::Uri;
    use rocket::{local::blocking::Client, launch, get, routes};
    use rocket::http::{Status, Header};
    
    use crate::Uuid;

    #[get("/<uuid>")]
    fn uuid(uuid: Uuid) -> String {
        uuid.to_string()
    }

    #[get("/uuid")]
    fn uuid_request(uuid: Uuid) -> String {
        uuid.to_string()
    }

    #[launch]
    fn rocket() -> _ {
        rocket::build().mount("/", routes![uuid, uuid_request])
    }

    #[test]
    fn test_from_param() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // Test if response is correct
        let uuid = Uuid::from_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").expect("Invalid uuid");
        let mut uri = String::from("/");
        uri.push_str(&uuid.to_string());
        let response = client.get(Uri::parse_any(&uri).expect("Invalid uri")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");

        // Test if failure message is correct
        let mut uri = String::from("/");
        uri.push_str("this-is-an-invalid-uuid");
        let response = client.get(Uri::parse_any(&uri).expect("Invalid uri")).dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_from_request() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // Test if response is correct
        let uuid = Uuid::from_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").expect("Invalid uuid");
        let uri = String::from("/uuid");
        let response = client.get(Uri::parse_any(&uri).expect("Invalid uri"))
            .header(Header::new("uuid", uuid.to_string()))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");

        // Test if failure message is correct
        let uri = String::from("/uuid");
        let response = client.get(Uri::parse_any(&uri).expect("Invalid uri")).dispatch();
        assert_eq!(response.status(), Status::Forbidden);
    }
}