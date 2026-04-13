//! v0.2.1 example: "At what Witt level is this softmax representable?"
//!
//! Second consumer one-liner from the v0.2.1 ergonomics spec. The softmax
//! shape is declared via `#[derive(ConstrainedType)]` and certified through
//! the `TowerCompletenessResolver`; the resulting `Validated<LiftChainCertificate>`
//! exposes `target_level()` via auto-deref.
//!
//! This example is the load-bearing Phase 7b.1 smoke test for per-call
//! Witt-level selection: it calls `certify_at(&shape, WittLevel::W16)`
//! with an explicit level and asserts the minted certificate carries that
//! exact level back through `target_level()`. The default `certify()` case
//! is also shown for comparison.
//!
//! Run with: `cargo run --example softmax_repr_level -p uor-foundation`

use uor_foundation::enforcement::{
    Certify, LiftChainCertificate, TowerCompletenessResolver, Validated,
};
use uor_foundation::WittLevel;
use uor_foundation_macros::ConstrainedType;

#[derive(ConstrainedType, Default)]
#[uor(residue = 65535, hamming = 16)]
struct SoftmaxShape;

fn main() {
    let shape = SoftmaxShape;
    let resolver = TowerCompletenessResolver::new();

    // Default-level one-liner: routes through `Certify::DEFAULT_LEVEL = W32`.
    let default_cert: Validated<LiftChainCertificate> = resolver
        .certify(&shape)
        .expect("softmax certifies at the default W32 level");
    let default_level: WittLevel = default_cert.target_level();
    println!(
        "softmax (default)  is representable at W{} ({}-bit ring)",
        default_level.witt_length(),
        default_level.witt_length()
    );
    assert_eq!(
        default_level.witt_length(),
        32,
        "default certify should route through Certify::DEFAULT_LEVEL = W32"
    );

    // Explicit W16 selection: `certify_at(&shape, WittLevel::W16)` threads
    // the level through the pipeline's run_tower_completeness entry point
    // and the minted LiftChainCertificate reports it back.
    let w16_cert: Validated<LiftChainCertificate> = resolver
        .certify_at(&shape, WittLevel::W16)
        .expect("softmax certifies at W16");
    let w16_level: WittLevel = w16_cert.target_level();
    println!(
        "softmax (at W16)  is representable at W{} ({}-bit ring)",
        w16_level.witt_length(),
        w16_level.witt_length()
    );
    assert_eq!(
        w16_level.witt_length(),
        16,
        "certify_at should propagate the explicit W16 level to the certificate"
    );

    // Explicit W24 selection — same propagation path, different level.
    let w24_cert: Validated<LiftChainCertificate> = resolver
        .certify_at(&shape, WittLevel::new(24))
        .expect("softmax certifies at W24");
    let w24_level: WittLevel = w24_cert.target_level();
    println!(
        "softmax (at W24)  is representable at W{} ({}-bit ring)",
        w24_level.witt_length(),
        w24_level.witt_length()
    );
    assert_eq!(
        w24_level.witt_length(),
        24,
        "certify_at should propagate the explicit W24 level to the certificate"
    );
}
