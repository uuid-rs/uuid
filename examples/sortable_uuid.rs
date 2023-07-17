//! Generating a sortable UUID.
//!
//! If you enable the `v7` feature you can generate sortable UUIDs.

#[cfg(all(uuid_unstable, feature = "v7"))]
fn main() {
    use uuid::Uuid;

    let uuid = Uuid::now_v7();

    assert_eq!(Some(uuid::Version::SortRand), uuid.get_version());

    println!("{}", uuid);
}

#[cfg(not(all(uuid_unstable, feature = "v7")))]
fn main() {}
