use trybuild::TestCases;

#[test]
fn parameter_macro_tests() {
    let t = TestCases::new();

    // Test case for successful expansion
    t.pass("tests/parameter_macro/pass.rs");

    // Test case for expected error
    // t.compile_fail("tests/parameter_macro/fail.rs");
}
