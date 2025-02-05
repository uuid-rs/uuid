#![cfg(test)]

use uuid::{Uuid, Variant, Version};

use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn test_new() {
    let uuid = Uuid::new_v4();

    assert_eq!(uuid.get_version(), Some(Version::Random));
    assert_eq!(uuid.get_variant(), Variant::RFC4122);
}
