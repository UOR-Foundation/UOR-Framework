//! v0.2.1 integration test: `uor_ground!` macro end-to-end.
//!
//! Verifies that the macro parses a conformance declaration, recovers the
//! `T` from the trailing `as Grounded<T>` clause, and emits tokens that
//! produce a real `Grounded<T>` value via the foundation's back-door
//! minting API.

use uor_foundation::enforcement::Grounded;
use uor_foundation_macros::{uor_ground, ConstrainedType};

#[derive(ConstrainedType, Default)]
#[uor(residue = 255, hamming = 8)]
#[allow(dead_code)]
struct Pixel(u8);

#[derive(ConstrainedType, Default)]
#[uor(residue = 65535, hamming = 16)]
#[allow(dead_code)]
struct PixelWide(u16);

#[test]
fn uor_ground_produces_grounded_for_pixel() {
    let unit: Grounded<Pixel> = uor_ground! {
        compile_unit hello_pixel {
            root_term: { 0 };
            witt_level_ceiling: W8;
            thermodynamic_budget: 64.0;
            target_domains: { ComposedAlgebraic };
        } as Grounded<Pixel>
    };
    // Witt level defaults to 8 bits because the macro passes W8 to the
    // pipeline driver (§4.2 v0.2.1 scope — literal is recognised but not
    // yet threaded through).
    assert_eq!(unit.witt_level_bits(), 8);
    // Content-addressed unit id is non-zero for any non-empty constraint
    // list (Pixel has residue + hamming, hashed by `hash_constraints`).
    assert_ne!(unit.unit_address(), 0);
}

#[test]
fn uor_ground_produces_distinct_grounded_for_different_shapes() {
    let a: Grounded<Pixel> = uor_ground! {
        compile_unit pixel_a {
            root_term: { 0 };
            witt_level_ceiling: W8;
            thermodynamic_budget: 64.0;
            target_domains: { ComposedAlgebraic };
        } as Grounded<Pixel>
    };
    let b: Grounded<PixelWide> = uor_ground! {
        compile_unit pixel_b {
            root_term: { 0 };
            witt_level_ceiling: W16;
            thermodynamic_budget: 128.0;
            target_domains: { ComposedAlgebraic };
        } as Grounded<PixelWide>
    };
    // The two grounded values carry distinct phantom types, so they are
    // not interchangeable at the type level. Content addresses differ
    // because the IRIs and constraint lists differ.
    assert_ne!(a.unit_address(), b.unit_address());
}

#[test]
fn uor_ground_witt_level_ceiling_w16_propagates() {
    // v0.2.1 Phase 8b.11: proves the macro's body parser actually threads
    // the `witt_level_ceiling: W16` keyword through to the pipeline driver,
    // distinct from the W8 default in the earlier test. If the macro
    // ignored the body value and used a stale default, this assertion
    // would fail with `witt_level_bits == 8`.
    let unit: Grounded<PixelWide> = uor_ground! {
        compile_unit pixel_w16 {
            root_term: { 0 };
            witt_level_ceiling: W16;
            thermodynamic_budget: 128.0;
            target_domains: { ComposedAlgebraic };
        } as Grounded<PixelWide>
    };
    assert_eq!(
        unit.witt_level_bits(),
        16,
        "W16 body value must propagate through run_pipeline"
    );
}

#[test]
fn uor_ground_witt_level_ceiling_w32_propagates() {
    // Same load-bearing test at W32, exercising the PixelWide shape with
    // an explicit W32 body value.
    let unit: Grounded<PixelWide> = uor_ground! {
        compile_unit pixel_w32 {
            root_term: { 0 };
            witt_level_ceiling: W32;
            thermodynamic_budget: 256.0;
            target_domains: { ComposedAlgebraic };
        } as Grounded<PixelWide>
    };
    assert_eq!(
        unit.witt_level_bits(),
        32,
        "W32 body value must propagate through run_pipeline"
    );
}
