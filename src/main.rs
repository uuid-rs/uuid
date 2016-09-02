extern crate uuid;

use uuid::{Uuid, UuidVersion};

fn main() {
    let i = Uuid::new(UuidVersion::Random);
    match i {
        Some(name) => println!("{}", name),
        None => panic!("unable to generate name")
    }
}
