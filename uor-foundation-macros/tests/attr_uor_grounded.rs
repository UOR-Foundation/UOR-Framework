//! v0.2.1 test: `#[uor_grounded(level = "W8")]` attribute.
//!
//! Asserts that the attribute:
//! 1. Leaves the annotated function intact.
//! 2. Emits a compile-time const that references the named WittLevel.
//! 3. Would fail type-check if the level name were undefined (documented in
//!    the macro docstring; not directly exercised because rustc rejects the
//!    test at compile time rather than producing a runtime assertion).

use uor_foundation_macros::uor_grounded;

#[uor_grounded(level = "W8")]
#[allow(dead_code)]
fn lowered_at_w8() -> u8 {
    42
}

#[uor_grounded(level = "W16")]
#[allow(dead_code)]
fn lowered_at_w16() -> u16 {
    42
}

#[test]
fn uor_grounded_accepts_named_levels() {
    assert_eq!(lowered_at_w8(), 42);
    assert_eq!(lowered_at_w16(), 42);
}
