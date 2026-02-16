//! Compile-time tests for `wdpe-macros` derive macros using `trybuild`.
//!
//! Note: `WdElement` happy-path tests are not included here because the derive
//! macro generates code with `crate::element::*` paths that only resolve inside
//! the `wdpe` crate. The error-case tests work because the macro rejects input
//! before generating those paths.

#[test]
fn derive_macro_tests() {
    let t = trybuild::TestCases::new();
    // Happy-path cases (WdLsData works externally since it only uses serde/std paths)
    t.pass("tests/ui/pass_lsdata.rs");
    // Error cases (rejected before crate-internal path generation)
    t.compile_fail("tests/ui/fail_missing_element_ref.rs");
    t.compile_fail("tests/ui/fail_duplicate_element_ref.rs");
    t.compile_fail("tests/ui/fail_lsevents_without_interactable.rs");
}
