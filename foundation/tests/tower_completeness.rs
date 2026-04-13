//! v0.2.1 integration test: TowerCompletenessResolver.certify().target_level().
//!
//! Exercises the consumer-facing one-liner from the v0.2.1 ergonomics spec:
//!
//! ```rust,ignore
//! let cert: Validated<LiftChainCertificate> =
//!     TowerCompletenessResolver::new().certify(&shape)?;
//! let level: WittLevel = cert.target_level();
//! ```
//!
//! v0.2.1 ships the resolver façade as a stub that returns the witness on
//! every input; this test verifies that the trait surface compiles and that
//! `Validated<LiftChainCertificate>` exposes `target_level` via auto-deref.

use uor_foundation::enforcement::{
    Certify, ConstrainedTypeInput, GenericImpossibilityWitness, IncrementalCompletenessResolver,
    LiftChainCertificate, TowerCompletenessResolver, Validated,
};
use uor_foundation::WittLevel;

#[test]
fn tower_resolver_constructible_via_new() {
    let _resolver = TowerCompletenessResolver::new();
}

#[test]
fn tower_resolver_vacuously_certifies_empty_input() {
    // An empty ConstrainedTypeInput passes all 6 preflight checks and all 7
    // reduction stages vacuously. The pipeline returns Ok(LiftChainCertificate)
    // whose target_level defaults to W8 per the shim.
    let resolver = TowerCompletenessResolver::new();
    let input = ConstrainedTypeInput::default();
    let result = resolver.certify(&input);
    assert!(result.is_ok(), "vacuous input must produce Ok");
    let _: GenericImpossibilityWitness = GenericImpossibilityWitness::default();
}

#[test]
fn incremental_resolver_certify_signature() {
    // IncrementalCompletenessResolver::certify has the expected type surface
    // and returns Ok for vacuous inputs.
    let resolver = IncrementalCompletenessResolver::new();
    let input = ConstrainedTypeInput::default();
    let result: Result<Validated<LiftChainCertificate>, GenericImpossibilityWitness> =
        resolver.certify(&input);
    assert!(result.is_ok());
}

#[test]
fn lift_chain_certificate_has_target_level_accessor() {
    // v0.2.1 Phase 7b.1: LiftChainCertificate::default() now carries the
    // canonical W32 level (Certify::DEFAULT_LEVEL) instead of the hardcoded
    // W8 used in earlier drafts.
    let cert = LiftChainCertificate::default();
    let level: WittLevel = cert.target_level();
    assert_eq!(level.witt_length(), 32);
}

#[test]
fn tower_resolver_certify_at_w16_returns_w16_target_level() {
    // Phase 7b.1 load-bearing test: `certify_at` with an explicit level
    // propagates through the pipeline and the minted LiftChainCertificate
    // carries that level back to the caller via `target_level()`.
    let resolver = TowerCompletenessResolver::new();
    let input = ConstrainedTypeInput::default();
    let cert = resolver
        .certify_at(&input, WittLevel::W16)
        .expect("empty constrained type certifies");
    assert_eq!(cert.target_level().witt_length(), 16);
}

#[test]
fn tower_resolver_certify_at_w24_returns_w24_target_level() {
    let resolver = TowerCompletenessResolver::new();
    let input = ConstrainedTypeInput::default();
    let cert = resolver
        .certify_at(&input, WittLevel::new(24))
        .expect("empty constrained type certifies");
    assert_eq!(cert.target_level().witt_length(), 24);
}

#[test]
fn tower_resolver_default_certify_uses_w32() {
    // Bare `.certify(&input)` (no explicit level) routes through
    // Certify::DEFAULT_LEVEL = W32.
    let resolver = TowerCompletenessResolver::new();
    let input = ConstrainedTypeInput::default();
    let cert = resolver.certify(&input).expect("empty certifies");
    assert_eq!(cert.target_level().witt_length(), 32);
}

// Note: `LiftChainCertificate::with_witt_bits` is pub(crate) and exercised
// indirectly through the `certify_at` tests above — the pipeline constructs
// the certificate with that helper and the integration tests observe the
// resulting `target_level()` value.
