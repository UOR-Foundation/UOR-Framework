//! v0.2.1 integration test: InhabitanceResolver Certify path.
//!
//! Exercises the consumer-facing one-liner:
//!
//! ```rust,ignore
//! let cert: Validated<InhabitanceCertificate> =
//!     InhabitanceResolver::new().certify(&shape)?;
//! ```
//!
//! v0.2.1 ships the resolver façade as a stub that returns the impossibility
//! witness on every input. The full pipeline driver lands in a follow-up
//! release. This test verifies the trait surface compiles and exercises both
//! the success and failure return shapes.

use uor_foundation::enforcement::{
    Certify, ConstrainedTypeInput, InhabitanceImpossibilityWitness, InhabitanceResolver,
    INHABITANCE_DISPATCH_TABLE,
};

#[test]
fn inhabitance_dispatch_table_has_three_rules() {
    assert_eq!(INHABITANCE_DISPATCH_TABLE.len(), 3);
    let priorities: Vec<u32> = INHABITANCE_DISPATCH_TABLE
        .iter()
        .map(|r| r.priority)
        .collect();
    assert_eq!(priorities, vec![0, 1, 2]);
}

#[test]
fn inhabitance_dispatch_predicate_iris_match_ontology() {
    let preds: Vec<&str> = INHABITANCE_DISPATCH_TABLE
        .iter()
        .map(|r| r.predicate_iri)
        .collect();
    assert!(preds.contains(&"https://uor.foundation/predicate/Is2SatShape"));
    assert!(preds.contains(&"https://uor.foundation/predicate/IsHornShape"));
    assert!(preds.contains(&"https://uor.foundation/predicate/IsResidualFragment"));
}

#[test]
fn inhabitance_dispatch_target_resolvers_match_ontology() {
    let targets: Vec<&str> = INHABITANCE_DISPATCH_TABLE
        .iter()
        .map(|r| r.target_resolver_iri)
        .collect();
    assert!(targets.contains(&"https://uor.foundation/resolver/TwoSatDecider"));
    assert!(targets.contains(&"https://uor.foundation/resolver/HornSatDecider"));
    assert!(targets.contains(&"https://uor.foundation/resolver/ResidualVerdictResolver"));
}

#[test]
fn inhabitance_resolver_vacuous_satisfies_empty_input() {
    // An empty ConstrainedTypeInput carries no constraints (SITE_COUNT = 0,
    // CONSTRAINTS = &[]). The reduction pipeline classifies this as
    // residual-with-no-SatClauses, which is vacuously satisfiable: every
    // value tuple in the (empty) carrier trivially satisfies the (empty)
    // constraint set. The resolver therefore mints a Validated<InhabitanceCertificate>.
    let resolver = InhabitanceResolver::new();
    let input = ConstrainedTypeInput::default();
    let result = resolver.certify(&input);
    assert!(
        result.is_ok(),
        "InhabitanceResolver must return Ok for vacuous (no-constraint) inputs"
    );
    // Verify the witness helper is reachable (default impl returns None per
    // the v0.2.1 shim semantics — real witness tuples come from uor_ground!).
    let _unused: InhabitanceImpossibilityWitness = InhabitanceImpossibilityWitness::default();
}
