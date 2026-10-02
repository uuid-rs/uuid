#![cfg(feature = "serde")]

use serde_derive::{Deserialize, Serialize};
use serde_test::{Configure, Readable, Token};
use uuid::Uuid;

const ID: &str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
const SIMPLE: &str = "f9168c5eceb24faab6bf329bf39fa1e4";
const BRACED: &str = "{f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4}";
const URN: &str = "urn:uuid:f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
const BYTES: [u8; 16] = [
    0xf9, 0x16, 0x8c, 0x5e, 0xce, 0xb2, 0x4f, 0xaa, 0xb6, 0xbf, 0x32, 0x9b, 0xf3, 0x9f, 0xa1, 0xe4,
];

macro_rules! text_option_case {
    ($test:ident, $adapter:literal, $encoded:expr, $wrong:expr) => {
        #[test]
        fn $test() {
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            struct Field {
                #[serde(with = $adapter)]
                id: Option<Uuid>,
            }

            #[derive(Debug, PartialEq, Deserialize)]
            struct DefaultField {
                #[serde(default, with = $adapter)]
                id: Option<Uuid>,
            }

            let present = Field {
                id: Some(ID.parse().unwrap()),
            };
            let absent = Field { id: None };
            let present_tokens = [
                Token::Struct {
                    name: "Field",
                    len: 1,
                },
                Token::Str("id"),
                Token::Some,
                Token::Str($encoded),
                Token::StructEnd,
            ];
            let absent_tokens = [
                Token::Struct {
                    name: "Field",
                    len: 1,
                },
                Token::Str("id"),
                Token::None,
                Token::StructEnd,
            ];

            serde_test::assert_tokens(&present.clone().readable(), &present_tokens);
            serde_test::assert_tokens(&present.clone().compact(), &present_tokens);
            serde_test::assert_tokens(&absent.clone().readable(), &absent_tokens);
            serde_test::assert_tokens(&absent.clone().compact(), &absent_tokens);

            assert_eq!(
                serde_json::to_string(&present).unwrap(),
                format!(r#"{{"id":"{}"}}"#, $encoded)
            );
            assert_eq!(serde_json::to_string(&absent).unwrap(), r#"{"id":null}"#);
            assert_eq!(
                serde_json::from_str::<Field>(&format!(r#"{{"id":"{}"}}"#, $encoded)).unwrap(),
                present
            );
            assert_eq!(
                serde_json::from_str::<Field>(r#"{"id":null}"#).unwrap(),
                absent
            );
            assert_eq!(
                serde_json::from_str::<DefaultField>("{}").unwrap(),
                DefaultField { id: None }
            );
            assert!(serde_json::from_str::<Field>("{}").is_err());
            assert!(serde_json::from_str::<Field>(r#"{"id":"invalid"}"#).is_err());
            assert!(serde_json::from_str::<Field>(&format!(r#"{{"id":"{}"}}"#, $wrong)).is_err());
        }
    };
}

text_option_case!(simple_option, "uuid::serde::simple::option", SIMPLE, ID);
text_option_case!(
    hyphenated_option,
    "uuid::serde::hyphenated::option",
    ID,
    BRACED
);
text_option_case!(braced_option, "uuid::serde::braced::option", BRACED, ID);
text_option_case!(urn_option, "uuid::serde::urn::option", URN, ID);

#[test]
fn compact_option() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Field {
        #[serde(with = "uuid::serde::compact::option")]
        id: Option<Uuid>,
    }
    #[derive(Debug, PartialEq, Deserialize)]
    struct DefaultField {
        #[serde(default, with = "uuid::serde::compact::option")]
        id: Option<Uuid>,
    }

    let present = Field {
        id: Some(Uuid::from_bytes(BYTES)),
    };
    let absent = Field { id: None };
    let mut tokens = vec![
        Token::Struct {
            name: "Field",
            len: 1,
        },
        Token::Str("id"),
        Token::Some,
        Token::Tuple { len: 16 },
    ];
    tokens.extend(BYTES.iter().copied().map(Token::U8));
    tokens.extend([Token::TupleEnd, Token::StructEnd]);
    serde_test::assert_tokens(&present.clone().readable(), &tokens);
    serde_test::assert_tokens(&present.clone().compact(), &tokens);
    let none_tokens = [
        Token::Struct {
            name: "Field",
            len: 1,
        },
        Token::Str("id"),
        Token::None,
        Token::StructEnd,
    ];
    serde_test::assert_tokens(&absent.clone().readable(), &none_tokens);
    serde_test::assert_tokens(&absent.clone().compact(), &none_tokens);
    serde_test::assert_de_tokens_error::<Readable<Field>>(
        &[
            Token::Struct {
                name: "Field",
                len: 1,
            },
            Token::Str("id"),
            Token::Some,
            Token::Bytes(&BYTES),
            Token::StructEnd,
        ],
        "invalid type: byte array, expected an array of length 16",
    );
    assert_eq!(
        serde_json::to_value(&present).unwrap()["id"],
        serde_json::json!(BYTES)
    );
    assert_eq!(serde_json::to_string(&absent).unwrap(), r#"{"id":null}"#);
    assert_eq!(
        serde_json::from_str::<Field>(r#"{"id":null}"#).unwrap(),
        absent
    );
    assert_eq!(
        serde_json::from_str::<DefaultField>("{}").unwrap(),
        DefaultField { id: None }
    );
    assert!(serde_json::from_str::<Field>("{}").is_err());
    assert!(serde_json::from_str::<Field>(r#"{"id":[1,2]}"#).is_err());
}

#[test]
fn bytes_option() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Field {
        #[serde(with = "uuid::serde::bytes::option")]
        id: Option<Uuid>,
    }
    #[derive(Debug, PartialEq, Deserialize)]
    struct DefaultField {
        #[serde(default, with = "uuid::serde::bytes::option")]
        id: Option<Uuid>,
    }

    let present = Field {
        id: Some(Uuid::from_bytes(BYTES)),
    };
    let absent = Field { id: None };
    let present_tokens = [
        Token::Struct {
            name: "Field",
            len: 1,
        },
        Token::Str("id"),
        Token::Some,
        Token::Bytes(&BYTES),
        Token::StructEnd,
    ];
    let none_tokens = [
        Token::Struct {
            name: "Field",
            len: 1,
        },
        Token::Str("id"),
        Token::None,
        Token::StructEnd,
    ];
    serde_test::assert_tokens(&present.clone().readable(), &present_tokens);
    serde_test::assert_tokens(&present.clone().compact(), &present_tokens);
    serde_test::assert_tokens(&absent.clone().readable(), &none_tokens);
    serde_test::assert_tokens(&absent.clone().compact(), &none_tokens);
    serde_test::assert_de_tokens_error::<Readable<Field>>(
        &[
            Token::Struct {
                name: "Field",
                len: 1,
            },
            Token::Str("id"),
            Token::Some,
            Token::Bytes(&BYTES[..15]),
            Token::StructEnd,
        ],
        "UUID parsing failed: invalid length: expected 16 bytes, found 15",
    );
    serde_test::assert_de_tokens_error::<Readable<Field>>(
        &[
            Token::Struct {
                name: "Field",
                len: 1,
            },
            Token::Str("id"),
            Token::Some,
            Token::Tuple { len: 16 },
        ],
        "invalid type: sequence, expected a 16 byte array",
    );
    assert_eq!(
        serde_json::to_value(&present).unwrap()["id"],
        serde_json::json!(BYTES)
    );
    assert_eq!(serde_json::to_string(&absent).unwrap(), r#"{"id":null}"#);
    assert_eq!(
        serde_json::from_str::<Field>(r#"{"id":null}"#).unwrap(),
        absent
    );
    assert_eq!(
        serde_json::from_str::<DefaultField>("{}").unwrap(),
        DefaultField { id: None }
    );
    assert!(serde_json::from_str::<Field>("{}").is_err());
}
