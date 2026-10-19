//! Two kinds of typed id are two types: passing one where the other is expected does not compile.
//! The expected compiler output is in tests/ui; it is checked on stable Rust only (CI), since the
//! compiler's wording changes between versions.

#[test]
fn ui() {
    trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
}
