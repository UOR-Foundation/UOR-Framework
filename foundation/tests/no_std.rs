//! v0.2.1: `#![no_std]` compile-time verification for `uor-foundation`.
//!
//! Run with: `cargo test -p uor-foundation --no-default-features --test no_std`.
//!
//! This test intentionally uses only core types and the foundation's
//! `#![no_std]`-compatible surface. If a change to the foundation crate
//! pulls in `std`, this test fails to compile.

#![no_std]

extern crate uor_foundation;

use uor_foundation::enforcement::{
    Certify, ConstrainedTypeInput, InhabitanceResolver, TowerCompletenessResolver,
};
use uor_foundation::pipeline::{
    decide_horn_sat, decide_two_sat, fragment_classify, ConstraintRef, FragmentKind,
};

#[test]
fn no_std_constrained_type_input_flows_through_certify() {
    // The foundation crate builds with `#![no_std]`; this test exercises
    // the Certify path against the canonical vacuous input.
    let input = ConstrainedTypeInput::default();
    assert!(TowerCompletenessResolver::new().certify(&input).is_ok());
    assert!(InhabitanceResolver::new().certify(&input).is_ok());
}

#[test]
fn no_std_pipeline_deciders_are_core_only() {
    // Both deciders accept core slices and return core booleans.
    assert!(decide_two_sat(&[], 0));
    assert!(decide_horn_sat(&[], 0));
    let residue = &[ConstraintRef::Residue {
        modulus: 256,
        residue: 255,
    }];
    assert_eq!(fragment_classify(residue), FragmentKind::Residual);
}
