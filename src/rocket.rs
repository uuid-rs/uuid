use core::str::FromStr;

use rocket::request::FromParam;

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

//impl<'a> FromRequest<'a> for Uuid {
//    typeError = ;
//    
//    fn from_request<'life0,'async_trait>(request: &'r rocket::Request<'life0>) ->  core::pin::Pin<alloc::boxed::Box<dyn core::future::Future<Output = rocket::request::Outcome<Self,Self::Error> > + core::marker::Send+'async_trait> >where 'r:'async_trait,'life0:'async_trait,Self:'async_trait {
//        
//    }
//}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use core::str::FromStr;

    use alloc::string::{String, ToString};
    use rocket::http::uri::Uri;
    use rocket::{local::blocking::Client, launch, get, routes};
    use rocket::http::Status;
    
    use crate::Uuid;

    #[get("/<uuid>")]
    fn uuid(uuid: Uuid) -> String {
        uuid.to_string()
    }

    #[launch]
    fn rocket() -> _ {
        rocket::build().mount("/", routes![uuid])
    }

    #[test]
    fn test_from_param() {
        let uuid = Uuid::from_str("a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8").expect("Invalid uuid");
        let mut uri = String::from("/");
        uri.push_str(&uuid.to_string());
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(Uri::parse_any(&uri).expect("Invalid uri")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8");
    }
}