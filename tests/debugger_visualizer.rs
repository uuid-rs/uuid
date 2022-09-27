use debugger_test::*;
use uuid::*;

#[inline(never)]
fn __break() {}

#[debugger_test(
    debugger = "cdb",
    commands = r#"
.nvlist

dx uuid

dx uuid_empty
"#,
    expected_statements = r#"
uuid             : "67e55044-10b1-426f-9247-bb680e5fe0c8" [Type: uuid::Uuid]

uuid_empty       : "00000000-0000-0000-0000-000000000000" [Type: uuid::Uuid]
"#
)]
fn test_debugger_visualizer() {
    let uuid_str = "67e5504410b1426f9247bb680e5fe0c8";
    let uuid_hyphenated_str = "67e55044-10b1-426f-9247-bb680e5fe0c8";

    let uuid = Uuid::parse_str(uuid_str).unwrap();
    assert_eq!(uuid_hyphenated_str, uuid.as_hyphenated().to_string());

    let result = uuid.to_string();
    assert_eq!(uuid_hyphenated_str, result);

    let uuid_empty = Uuid::nil();
    assert_eq!(
        String::from("00000000-0000-0000-0000-000000000000"),
        uuid_empty.to_string()
    );

    __break();
}
