use crate::prelude::*;

impl Uuid {
    /// Creates a random UUID.
    ///
    /// This uses the [`getrandom`] crate to utilise the operating system's RNG
    /// as the source of random numbers. If you'd like to use a custom generator,
    /// don't use this method: generate random bytes using your custom generator
    /// and pass them to the [`uuid::Builder::from_bytes`][from_bytes] function
    /// instead.
    ///
    /// Note that usage of this method requires the `v4` feature of this crate
    /// to be enabled.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// let uuid = Uuid::new_v4();
    /// ```
    ///
    /// [`getrandom`]: https://crates.io/crates/getrandom
    /// [from_bytes]: struct.Builder.html#method.from_bytes
    // TODO: change signature to support uuid's Error.
    pub fn new_v4() -> Result<Uuid, getrandom::Error> {
        let mut bytes = [0u8; 16];
        getrandom::getrandom(&mut bytes)?;

        let uuid = crate::builder::Builder::from_bytes(bytes)
            .set_variant(Variant::RFC4122)
            .set_version(Version::Random)
            .build();
        Ok(uuid)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_new() {
        let uuid = Uuid::new_v4().unwrap();

        assert_eq!(uuid.get_version(), Some(Version::Random));
        assert_eq!(uuid.get_variant(), Some(Variant::RFC4122));
    }

    #[test]
    fn test_get_version() {
        let uuid = Uuid::new_v4().unwrap();

        assert_eq!(uuid.get_version(), Some(Version::Random));
        assert_eq!(uuid.get_version_num(), 4)
    }
}
