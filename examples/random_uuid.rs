//! Generating a random UUID.
//!
//! If you enable the `v4` feature you can generate random UUIDs.

#[cfg(feature = "v4")]
fn main() {
    use uuid::Uuid;

    let uuid = Uuid::new_v4();

    assert_eq!(Some(uuid::Version::Random), uuid.get_version());

    println!("{}", uuid);
}

#[cfg(not(feature = "v4"))]
fn main() {}
