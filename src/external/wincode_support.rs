#[cfg(test)]
mod wincode_tests {
    use crate::Uuid;
    use std::string::ToString;

    #[test]
    fn test_serialize() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        let uuid_bytes = uuid.as_bytes();
        let wincode_bytes = wincode::serialize(&uuid).unwrap();
        assert_eq!(uuid_bytes, wincode_bytes.as_slice());
    }

    #[test]
    fn test_deserialize() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let uuid = Uuid::parse_str(uuid_str).unwrap();
        let uuid_bytes = uuid.as_bytes();
        let deserialized = wincode::deserialize::<Uuid>(uuid_bytes)
            .unwrap()
            .to_string();
        assert_eq!(uuid_str, deserialized);
    }
}
