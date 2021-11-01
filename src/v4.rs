use crate::{Uuid, Variant, Version};

impl Uuid {
    /// Creates a random UUID.
    ///
    /// This uses the [`getrandom`] crate to utilise the operating system's RNG
    /// as the source of random numbers. If you'd like to use a custom
    /// generator, don't use this method: generate random bytes using your
    /// custom generator and pass them to the
    /// [`uuid::Builder::from_random_bytes`][from_random_bytes] function instead.
    ///
    /// Note that usage of this method requires the `v4` feature of this crate
    /// to be enabled.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::{Uuid, Version};
    /// let uuid = Uuid::new_v4();
    ///
    /// assert_eq!(Some(Version::Random), uuid.get_version());
    /// ```
    ///
    /// [`getrandom`]: https://crates.io/crates/getrandom
    /// [from_random_bytes]: struct.Builder.html#method.from_random_bytes
    pub fn new_v4() -> Uuid {
        let mut bytes = [0u8; 16];
        getrandom::getrandom(&mut bytes).unwrap_or_else(|err| {
            // NB: getrandom::Error has no source; this is adequate display
            panic!("could not retrieve random bytes for uuid: {}", err)
        });

        crate::Builder::from_bytes(bytes)
            .set_variant(Variant::RFC4122)
            .set_version(Version::Random)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_new() {
        let uuid = Uuid::new_v4();

        assert_eq!(uuid.get_version(), Some(Version::Random));
        assert_eq!(uuid.get_variant(), Variant::RFC4122);
    }

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_get_version() {
        let uuid = Uuid::new_v4();

        assert_eq!(uuid.get_version(), Some(Version::Random));
        assert_eq!(uuid.get_version_num(), 4)
    }
}
