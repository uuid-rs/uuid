use crate::{Builder, Uuid, Variant, Version};

impl Uuid {
    /// Creates a custom UUID comprised almost entirely of user-supplied bytes
    ///
    /// This will inject the UUID Version at 4 bits starting at the 48th bit
    /// and the Variant into 2 bits 64th bit.
    /// So if there are bits are supplied in the input buffer, they will not be
    /// visible in the result
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::{Uuid, Version};
    /// let buf: [u8; 16] = *b"abcdefghijklmnop";
    /// let uuid = Uuid::new_v8(buf);
    ///
    /// assert_eq!(Some(Version::Custom), uuid.get_version());
    /// ```
    ///
    /// [`getrandom`]: https://crates.io/crates/getrandom
    /// [from_random_bytes]: struct.Builder.html#method.from_random_bytes
    pub fn new_v8(buf: [u8; 16]) -> Uuid {
        Builder(Uuid::from_bytes(buf))
            .with_variant(Variant::RFC4122)
            .with_version(Version::Custom)
            .into_uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::bytes;
    use std::string::ToString;

    #[cfg(target_arch = "wasm32")]
    use wasm_bindgen_test::*;

    #[test]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
    fn test_new() {
        let buf = bytes();
        let uuid = Uuid::new_v8(buf);
        assert_eq!(uuid.get_version(), Some(Version::Custom));
        assert_eq!(uuid.get_variant(), Variant::RFC4122);
        assert_eq!(uuid.get_version_num(), 8)
    }
}
