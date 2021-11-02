#[cfg(feature = "macros")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/compile_pass/*.rs");
    t.compile_fail("tests/ui/compile_fail/*.rs");
}